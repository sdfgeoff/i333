/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * gaps.c: gaps logic: whether to display gaps at all, and how big
 *         they should be.
 *
 */
#![deny(unsafe_op_in_unsafe_fn)]

use crate::application::{
    ws_assignments, GAPS_BOTTOM, GAPS_INNER, GAPS_LEFT, GAPS_RIGHT, GAPS_TOP,
};
use crate::con::{
    con_get_fullscreen_con, con_get_workspace, con_has_parent, con_inside_floating,
    con_inside_stacked_or_tabbed, con_is_leaf, con_num_children, con_num_visible_children,
    CF_GLOBAL, CF_OUTPUT, CT_CON, CT_WORKSPACE,
};
use crate::config::config;
use crate::log::debuglog;
use crate::output::output_get_content;
use crate::resize::resize_find_tiling_participants;
use crate::tree::croot;
use crate::types::{gaps_t, Con, Direction, Layout, SmartGaps};
use crate::util::{name_is_digits, ws_name_to_number};
use std::ffi::CStr;

const NO_GAPS: gaps_t = gaps_t {
    inner: 0,
    top: 0,
    right: 0,
    bottom: 0,
    left: 0,
};

fn single_visible_child(visible: i32, children: i32, first_layout: Option<Layout>) -> bool {
    visible <= 1 || children == 1 && matches!(first_layout, Some(Layout::Tabbed | Layout::Stacked))
}

fn effective_gaps(
    workspace: gaps_t,
    configured: gaps_t,
    smart: SmartGaps,
    one_child: bool,
) -> gaps_t {
    if smart == SmartGaps::On && one_child {
        return NO_GAPS;
    }
    // Preserve signed gap offsets and the existing arithmetic policy: normal
    // addition/subtraction, checked in debug builds and wrapping in release.
    let mut gaps = gaps_t {
        inner: workspace.inner + configured.inner,
        ..NO_GAPS
    };
    if smart != SmartGaps::InverseOuter || one_child {
        gaps.top = workspace.top + configured.top;
        gaps.right = workspace.right + configured.right;
        gaps.bottom = workspace.bottom + configured.bottom;
        gaps.left = workspace.left + configured.left;
    }
    gaps
}

#[derive(Clone, Copy)]
enum InsetParent {
    Workspace,
    Split,
    Other,
}

fn should_inset(
    parent: Option<InsetParent>,
    floating: bool,
    leaf: bool,
    layout: Layout,
    stacked_ancestor: bool,
) -> bool {
    if floating || !(leaf || matches!(layout, Layout::Stacked | Layout::Tabbed)) {
        return false;
    }
    match parent {
        Some(InsetParent::Workspace) => true,
        Some(InsetParent::Split) => !stacked_ancestor,
        Some(InsetParent::Other) | None => false,
    }
}

#[derive(Clone, Copy)]
enum FullscreenNeighbor {
    NoFullscreen,
    ContainerIsFullscreen,
    WithinFullscreen { container: bool, neighbor: bool },
}

fn has_adjacent(found_neighbor: bool, fullscreen: FullscreenNeighbor) -> bool {
    found_neighbor
        && match fullscreen {
            FullscreenNeighbor::NoFullscreen => true,
            FullscreenNeighbor::ContainerIsFullscreen => false,
            FullscreenNeighbor::WithinFullscreen {
                container,
                neighbor,
            } => container && neighbor,
        }
}

#[derive(Clone, Copy)]
enum AssignmentMatch {
    Name,
    Number,
    None,
}

#[derive(Clone, Copy)]
struct GapAssignment {
    matching: AssignmentMatch,
    gaps: gaps_t,
    mask: u32,
}

fn assignment_offsets(mut gaps: gaps_t, mask: u32, configured: gaps_t) -> gaps_t {
    for (value, global, bit) in [
        (&mut gaps.inner, configured.inner, GAPS_INNER),
        (&mut gaps.top, configured.top, GAPS_TOP),
        (&mut gaps.right, configured.right, GAPS_RIGHT),
        (&mut gaps.bottom, configured.bottom, GAPS_BOTTOM),
        (&mut gaps.left, configured.left, GAPS_LEFT),
    ] {
        if mask & bit != 0 {
            *value -= global;
        }
    }
    gaps
}

fn workspace_gaps(
    assignments: impl IntoIterator<Item = GapAssignment>,
    configured: gaps_t,
) -> gaps_t {
    let mut gaps = NO_GAPS;
    let mut mask = 0;
    for assignment in assignments {
        match assignment.matching {
            AssignmentMatch::None => continue,
            AssignmentMatch::Name | AssignmentMatch::Number => {
                gaps = assignment.gaps;
                mask = assignment.mask;
                // Exact name wins immediately. Number-only matches overwrite
                // earlier numeric assignments, even if their mask is zero.
                if matches!(assignment.matching, AssignmentMatch::Name) {
                    break;
                }
            }
        }
    }
    assignment_offsets(gaps, mask, configured)
}

/// Calculate effective gaps using the current workspace and configuration.
///
/// # Safety
/// `con` must be a live node in the initialized container tree. Requires exclusive
/// event-thread access to that tree and configuration for the entire call.
pub unsafe fn calculate_effective_gaps(con: *mut Con) -> gaps_t {
    // SAFETY: The caller guarantees the native tree's pointers and lifetimes.
    // Only copied geometry/layout values leave this adapter.
    unsafe {
        let workspace = con_get_workspace(con);
        if workspace.is_null() {
            return NO_GAPS;
        }
        let visible = con_num_visible_children(workspace);
        let one_child = if visible <= 1 {
            true
        } else {
            let children = con_num_children(workspace);
            let first_layout = if children == 1 {
                // A consistent one-child intrusive queue contains a live node.
                Some((*(*workspace).nodes_head.tqh_first).layout)
            } else {
                None
            };
            single_visible_child(visible, children, first_layout)
        };
        effective_gaps((*workspace).gaps, config.gaps, config.smart_gaps, one_child)
    }
}

/// Decide whether this native container should receive an inset.
///
/// # Safety
/// `con` and all its parent links must be live in the initialized tree. Requires
/// exclusive event-thread access while native ancestry queries inspect the tree.
pub unsafe fn gaps_should_inset_con(con: *mut Con, _children: i32) -> bool {
    // SAFETY: Native graph access is confined to this adapter under the caller's
    // exclusive access contract; the decision helper receives no pointers.
    unsafe {
        let parent = (*con).parent;
        if parent.is_null() {
            return false;
        }
        if !con_inside_floating(con).is_null() {
            return false;
        }
        let parent_kind = if (*parent).type_0 == CT_WORKSPACE {
            InsetParent::Workspace
        } else if (*parent).type_0 == CT_CON
            && matches!((*parent).layout, Layout::SplitH | Layout::SplitV)
        {
            InsetParent::Split
        } else {
            InsetParent::Other
        };
        let leaf = con_is_leaf(con);
        let layout = (*con).layout;
        // Preserve short circuiting: no ancestry walk for non-inset candidates
        // or direct workspace children.
        let stacked_ancestor = (leaf || matches!(layout, Layout::Stacked | Layout::Tabbed))
            && !matches!(parent_kind, InsetParent::Workspace)
            && con_inside_stacked_or_tabbed(con);
        should_inset(Some(parent_kind), false, leaf, layout, stacked_ancestor)
    }
}

/// Check for an adjacent tiling container within the same fullscreen region.
///
/// # Safety
/// `con` must be a live workspace descendant with valid ancestry/tiling queues.
/// Requires exclusive event-thread access to the initialized native tree.
pub unsafe fn gaps_has_adjacent_container(con: *mut Con, direction: Direction) -> bool {
    // SAFETY: Native lookup routines borrow the caller-owned graph and write
    // only these stack participant pointers. No graph references are retained.
    unsafe {
        let workspace = con_get_workspace(con);
        let mut fullscreen = con_get_fullscreen_con(workspace, CF_GLOBAL);
        if fullscreen.is_null() {
            fullscreen = con_get_fullscreen_con(workspace, CF_OUTPUT);
        }
        if con == fullscreen {
            return has_adjacent(false, FullscreenNeighbor::ContainerIsFullscreen);
        }
        let mut first = con;
        let mut second = std::ptr::null_mut();
        let found = resize_find_tiling_participants(&mut first, &mut second, direction, false);
        let context = if !found || fullscreen.is_null() {
            FullscreenNeighbor::NoFullscreen
        } else {
            let container = con_has_parent(con, fullscreen);
            FullscreenNeighbor::WithinFullscreen {
                container,
                neighbor: container && con_has_parent(second, fullscreen),
            }
        };
        has_adjacent(found, context)
    }
}

/// Resolve workspace gap assignments in their configured order.
///
/// # Safety
/// `ws` and all workspace-assignment records/names must be live and initialized.
/// Requires exclusive event-thread access to the tree, assignment queue and
/// configuration until this function and its lazy local iterator have returned.
pub unsafe fn gaps_for_workspace(ws: *mut Con) -> gaps_t {
    // SAFETY: Queue/name borrows come from the caller's initialized graph.
    let (mut current, name, number, configured) = unsafe {
        (
            ws_assignments.tqh_first,
            CStr::from_ptr((*ws).name),
            (*ws).num,
            config.gaps,
        )
    };
    // This iterator is local to the unsafe adapter: it never escapes or exposes
    // a safe API over raw graph storage. Its items are independent copied values.
    let assignments = std::iter::from_fn(|| {
        if current.is_null() {
            return None;
        }
        // SAFETY: Each record belongs to the stable native queue above. Names
        // are live NUL strings. No reference survives the next iterator call.
        unsafe {
            let assignment = &*current;
            current = assignment.ws_assignments.tqe_next;
            let matching = if CStr::from_ptr(assignment.name) == name {
                AssignmentMatch::Name
            } else if number != -1
                && name_is_digits(assignment.name)
                && ws_name_to_number(assignment.name) == number
            {
                AssignmentMatch::Number
            } else {
                AssignmentMatch::None
            };
            Some(GapAssignment {
                matching,
                gaps: assignment.gaps,
                mask: assignment.gaps_mask,
            })
        }
    });
    workspace_gaps(assignments, configured)
}

/// Reapply assignments to the live tree after a configuration reload.
///
/// # Safety
/// Requires exclusive event-thread access to initialized `croot`, every output's
/// content/workspace queues, the assignment records and configuration. No other
/// graph borrows may overlap the workspace gap writes performed here.
pub unsafe fn gaps_reapply_workspace_assignments() {
    // SAFETY: The caller provides the live, exclusive intrusive graph. Neither
    // gap resolution nor logging changes its queue topology.
    unsafe {
        let mut output = (*croot).nodes_head.tqh_first;
        while !output.is_null() {
            let content = output_get_content(output);
            let mut workspace = (*content).nodes_head.tqh_first;
            while !workspace.is_null() {
                debuglog(
                    c"%s:%s:%d - updating gap assignments for workspace %s\n"
                        .as_ptr()
                        .cast_mut(),
                    c"../src/gaps.c".as_ptr(),
                    c"gaps_reapply_workspace_assignments".as_ptr(),
                    171i32,
                    (*workspace).name,
                );
                (*workspace).gaps = gaps_for_workspace(workspace);
                workspace = (*workspace).nodes.tqe_next;
            }
            output = (*output).nodes.tqe_next;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn values(gaps: gaps_t) -> [i32; 5] {
        [gaps.inner, gaps.top, gaps.right, gaps.bottom, gaps.left]
    }

    fn gaps(values: [i32; 5]) -> gaps_t {
        let [inner, top, right, bottom, left] = values;
        gaps_t {
            inner,
            top,
            right,
            bottom,
            left,
        }
    }

    #[test]
    fn smart_modes_account_for_visible_children_and_tabbed_or_stacked_groups() {
        let configured = gaps([10, 20, 30, 40, 50]);
        let workspace = gaps([-3, -22, 4, -45, 7]);
        for layout in [Layout::Tabbed, Layout::Stacked] {
            let one = single_visible_child(4, 1, Some(layout));
            assert!(one);
            assert_eq!(
                values(effective_gaps(workspace, configured, SmartGaps::On, one)),
                [0; 5]
            );
            assert_eq!(
                values(effective_gaps(
                    workspace,
                    configured,
                    SmartGaps::InverseOuter,
                    one
                )),
                [7, -2, 34, -5, 57]
            );
        }
        for layout in [Layout::SplitH, Layout::SplitV, Layout::Default] {
            let one = single_visible_child(4, 1, Some(layout));
            assert!(!one);
            assert_eq!(
                values(effective_gaps(workspace, configured, SmartGaps::On, one)),
                [7, -2, 34, -5, 57]
            );
            assert_eq!(
                values(effective_gaps(
                    workspace,
                    configured,
                    SmartGaps::InverseOuter,
                    one
                )),
                [7, 0, 0, 0, 0]
            );
        }
        // A single visible window suppresses smart gaps despite additional
        // hidden children; a grouped child among multiple groups does not.
        assert!(single_visible_child(1, 4, Some(Layout::SplitH)));
        assert!(single_visible_child(0, 0, None));
        assert!(!single_visible_child(4, 2, Some(Layout::Tabbed)));
        assert_eq!(
            values(effective_gaps(workspace, configured, SmartGaps::Off, true)),
            [7, -2, 34, -5, 57]
        );
    }

    #[test]
    fn inset_avoids_nested_tabbed_double_gaps_and_floating_splits() {
        for layout in [Layout::Tabbed, Layout::Stacked, Layout::SplitH] {
            let leaf = layout == Layout::SplitH;
            assert!(should_inset(
                Some(InsetParent::Workspace),
                false,
                leaf,
                layout,
                true
            ));
            assert!(should_inset(
                Some(InsetParent::Split),
                false,
                leaf,
                layout,
                false
            ));
            assert!(!should_inset(
                Some(InsetParent::Split),
                false,
                leaf,
                layout,
                true
            ));
            assert!(!should_inset(
                Some(InsetParent::Workspace),
                true,
                leaf,
                layout,
                false
            ));
            assert!(!should_inset(None, false, leaf, layout, false));
            assert!(!should_inset(
                Some(InsetParent::Other),
                false,
                leaf,
                layout,
                false
            ));
        }
        assert!(!should_inset(
            Some(InsetParent::Workspace),
            false,
            false,
            Layout::SplitV,
            false
        ));
        assert!(!should_inset(
            Some(InsetParent::Split),
            false,
            false,
            Layout::SplitH,
            false
        ));
    }

    #[test]
    fn adjacency_stays_inside_the_fullscreen_region() {
        assert!(has_adjacent(true, FullscreenNeighbor::NoFullscreen));
        for found in [false, true] {
            assert!(!has_adjacent(
                found,
                FullscreenNeighbor::ContainerIsFullscreen
            ));
            for (container, neighbor) in
                [(false, false), (true, false), (false, true), (true, true)]
            {
                assert_eq!(
                    has_adjacent(
                        found,
                        FullscreenNeighbor::WithinFullscreen {
                            container,
                            neighbor
                        }
                    ),
                    found && container && neighbor
                );
            }
        }
        assert!(!has_adjacent(false, FullscreenNeighbor::NoFullscreen));
    }

    fn assigned(matching: AssignmentMatch, number: i32, mask: u32) -> GapAssignment {
        GapAssignment {
            matching,
            gaps: gaps([number; 5]),
            mask,
        }
    }

    #[test]
    fn first_exact_name_wins_but_last_numeric_match_applies_until_then() {
        let configured = gaps([1, 2, 3, 4, 5]);
        let numeric = [
            assigned(AssignmentMatch::Number, 10, GAPS_TOP),
            assigned(AssignmentMatch::None, 999, 31),
            assigned(AssignmentMatch::Number, 20, GAPS_LEFT),
        ];
        assert_eq!(
            values(workspace_gaps(numeric, configured)),
            [20, 20, 20, 20, 15]
        );
        let exact = numeric
            .into_iter()
            .chain([assigned(
                AssignmentMatch::Name,
                30,
                GAPS_INNER | GAPS_BOTTOM,
            )])
            .chain(std::iter::from_fn(|| {
                panic!("assignment after exact name must not be inspected")
            }));
        assert_eq!(
            values(workspace_gaps(exact, configured)),
            [29, 30, 30, 26, 30]
        );
        // A zero mask on an exact match still overrides a previous numeric match.
        assert_eq!(
            values(workspace_gaps(
                [
                    numeric[0],
                    assigned(AssignmentMatch::Name, 7, 0),
                    numeric[2]
                ],
                configured
            )),
            [7; 5]
        );
        assert_eq!(
            values(workspace_gaps(
                [assigned(AssignmentMatch::None, 100, 31)],
                configured
            )),
            [0; 5]
        );
    }

    #[test]
    fn assignment_mask_offsets_compose_with_signed_global_gaps() {
        let configured_values = [3, -5, 7, -11, 13];
        let requested_values = [-20, 30, -40, 50, -60];
        let configured = gaps(configured_values);
        for mask in 0..32 {
            let assignment = GapAssignment {
                matching: AssignmentMatch::Name,
                gaps: gaps(requested_values),
                mask,
            };
            let workspace = workspace_gaps([assignment], configured);
            let actual = values(effective_gaps(workspace, configured, SmartGaps::Off, false));
            for index in 0..5 {
                let expected = if mask & (1 << index) != 0 {
                    requested_values[index]
                } else {
                    requested_values[index] + configured_values[index]
                };
                assert_eq!(actual[index], expected, "mask {mask}, field {index}");
            }
            // Unknown mask bits cannot alter any component.
            assert_eq!(
                values(assignment_offsets(workspace, 1 << 31, configured)),
                values(workspace)
            );
        }
        // Signed extremes are retained when no overflowing operation is needed.
        let extreme = gaps([i32::MIN, i32::MAX, -1, 0, 1]);
        assert_eq!(
            values(effective_gaps(extreme, NO_GAPS, SmartGaps::Off, false)),
            values(extreme)
        );
    }
}
