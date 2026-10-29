use crate::application::ws_assignments;
use crate::types::Direction;
use crate::types::Layout;
use crate::types::SmartGaps;
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
// Mechanical file-by-file Rust translation of src/gaps.c.
// Originally translated with C2Rust 0.22.1; subsequently maintained in Rust.

pub type i3String = _i3String;

pub type i3Font = Font;

pub type fullscreen_mode_t = ::core::ffi::c_uint;
pub const CF_GLOBAL: fullscreen_mode_t = 2;
pub const CF_OUTPUT: fullscreen_mode_t = 1;
pub const CF_NONE: fullscreen_mode_t = 0;

pub const M_BELOW: ::core::ffi::c_uint = 2;
pub const M_ASSIGN_WS: ::core::ffi::c_uint = 1;
pub const M_HERE: ::core::ffi::c_uint = 0;

pub const M_DOCK_BOTTOM: ::core::ffi::c_int = 3;
pub const M_DOCK_TOP: ::core::ffi::c_int = 2;
pub const M_DOCK_ANY: ::core::ffi::c_int = 1;
pub const M_NODOCK: ::core::ffi::c_int = 0;
pub const M_DONTCHECK: ::core::ffi::c_int = -1;

pub const A_TO_OUTPUT: ::core::ffi::c_uint = 16;
pub const A_TO_WORKSPACE_NUMBER: ::core::ffi::c_uint = 8;
pub const A_NO_FOCUS: ::core::ffi::c_uint = 4;
pub const A_TO_WORKSPACE: ::core::ffi::c_uint = 2;
pub const A_COMMAND: ::core::ffi::c_uint = 1;
pub const A_ANY: ::core::ffi::c_uint = 0;

pub const CT_DOCKAREA: ::core::ffi::c_uint = 5;
pub const CT_WORKSPACE: ::core::ffi::c_uint = 4;
pub const CT_FLOATING_CON: ::core::ffi::c_uint = 3;
pub const CT_CON: ::core::ffi::c_uint = 2;
pub const CT_OUTPUT: ::core::ffi::c_uint = 1;
pub const CT_ROOT: ::core::ffi::c_uint = 0;

pub type hide_edge_borders_mode_t = ::core::ffi::c_uint;
pub const HEBM_SMART_NO_GAPS: hide_edge_borders_mode_t = 64;
pub const HEBM_SMART: hide_edge_borders_mode_t = 32;
pub const HEBM_BOTH: hide_edge_borders_mode_t = 23;
pub const HEBM_HORIZONTAL: hide_edge_borders_mode_t = 20;
pub const HEBM_VERTICAL: hide_edge_borders_mode_t = 3;
pub const HEBM_NONE: hide_edge_borders_mode_t = 0;

pub type gaps_mask_t = ::core::ffi::c_uint;
pub const GAPS_OUTER: gaps_mask_t = 30;
pub const GAPS_HORIZONTAL: gaps_mask_t = 20;
pub const GAPS_VERTICAL: gaps_mask_t = 10;
pub const GAPS_LEFT: gaps_mask_t = 16;
pub const GAPS_BOTTOM: gaps_mask_t = 8;
pub const GAPS_RIGHT: gaps_mask_t = 4;
pub const GAPS_TOP: gaps_mask_t = 2;
pub const GAPS_INNER: gaps_mask_t = 1;

pub use crate::application::Workspace_Assignment;
pub type tiling_drag_t = ::core::ffi::c_uint;
pub const TILING_DRAG_MODIFIER_OR_TITLEBAR: tiling_drag_t = 3;
pub const TILING_DRAG_TITLEBAR: tiling_drag_t = 2;
pub const TILING_DRAG_MODIFIER: tiling_drag_t = 1;
pub const TILING_DRAG_OFF: tiling_drag_t = 0;

pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();

pub unsafe fn calculate_effective_gaps(con: *mut Con) -> gaps_t {
    let workspace: *mut Con = con_get_workspace(con);
    if workspace.is_null() {
        return gaps_t {
            inner: 0 as ::core::ffi::c_int,
            top: 0 as ::core::ffi::c_int,
            right: 0 as ::core::ffi::c_int,
            bottom: 0 as ::core::ffi::c_int,
            left: 0 as ::core::ffi::c_int,
        };
    }
    let one_child: bool = con_num_visible_children(workspace) <= 1 as ::core::ffi::c_int
        || con_num_children(workspace) == 1 as ::core::ffi::c_int
            && ((*(*workspace).nodes_head.tqh_first).layout == Layout::Tabbed
                || (*(*workspace).nodes_head.tqh_first).layout == Layout::Stacked);
    if config.smart_gaps == SmartGaps::On && one_child as ::core::ffi::c_int != 0 {
        return gaps_t {
            inner: 0 as ::core::ffi::c_int,
            top: 0 as ::core::ffi::c_int,
            right: 0 as ::core::ffi::c_int,
            bottom: 0 as ::core::ffi::c_int,
            left: 0 as ::core::ffi::c_int,
        };
    }
    let mut gaps: gaps_t = gaps_t {
        inner: (*workspace).gaps.inner + config.gaps.inner,
        top: 0 as ::core::ffi::c_int,
        right: 0 as ::core::ffi::c_int,
        bottom: 0 as ::core::ffi::c_int,
        left: 0 as ::core::ffi::c_int,
    };
    if config.smart_gaps != SmartGaps::InverseOuter || one_child as ::core::ffi::c_int != 0 {
        gaps.top = (*workspace).gaps.top + config.gaps.top;
        gaps.right = (*workspace).gaps.right + config.gaps.right;
        gaps.bottom = (*workspace).gaps.bottom + config.gaps.bottom;
        gaps.left = (*workspace).gaps.left + config.gaps.left;
    }
    gaps
}

pub unsafe fn gaps_should_inset_con(con: *mut Con, children: ::core::ffi::c_int) -> bool {
    if (*con).parent.is_null() {
        return false_0 != 0;
    }
    if !con_inside_floating(con).is_null() {
        return false_0 != 0;
    }
    let leaf_or_stacked_tabbed: bool = con_is_leaf(con) as ::core::ffi::c_int != 0
        || ((*con).layout == Layout::Stacked || (*con).layout == Layout::Tabbed);
    if leaf_or_stacked_tabbed as ::core::ffi::c_int != 0
        && (*(*con).parent).type_0 as ::core::ffi::c_uint
            == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return true_0 != 0;
    }
    if leaf_or_stacked_tabbed as ::core::ffi::c_int != 0
        && !con_inside_stacked_or_tabbed(con)
        && (*(*con).parent).type_0 as ::core::ffi::c_uint
            == CT_CON as ::core::ffi::c_int as ::core::ffi::c_uint
        && ((*(*con).parent).layout == Layout::SplitH || (*(*con).parent).layout == Layout::SplitV)
    {
        return true_0 != 0;
    }
    false_0 != 0
}

pub unsafe fn gaps_has_adjacent_container(con: *mut Con, direction: Direction) -> bool {
    let workspace: *mut Con = con_get_workspace(con);
    let mut fullscreen: *mut Con = con_get_fullscreen_con(workspace, CF_GLOBAL);
    if fullscreen.is_null() {
        fullscreen = con_get_fullscreen_con(workspace, CF_OUTPUT);
    }
    if con == fullscreen {
        return false_0 != 0;
    }
    let mut first: *mut Con = con;
    let mut second: *mut Con = ::core::ptr::null_mut::<Con>();
    let found_neighbor: bool =
        resize_find_tiling_participants(&raw mut first, &raw mut second, direction, false_0 != 0);
    if !found_neighbor {
        return false_0 != 0;
    }
    if fullscreen.is_null() {
        return true_0 != 0;
    }
    con_has_parent(con, fullscreen) as ::core::ffi::c_int != 0
        && con_has_parent(second, fullscreen) as ::core::ffi::c_int != 0
}

pub unsafe fn gaps_for_workspace(ws: *mut Con) -> gaps_t {
    let mut gaps: gaps_t = gaps_t {
        inner: 0 as ::core::ffi::c_int,
        top: 0 as ::core::ffi::c_int,
        right: 0 as ::core::ffi::c_int,
        bottom: 0 as ::core::ffi::c_int,
        left: 0 as ::core::ffi::c_int,
    };
    let mut mask: gaps_mask_t = 0 as gaps_mask_t;
    let mut assignment: *mut Workspace_Assignment = ::core::ptr::null_mut::<Workspace_Assignment>();
    assignment = ws_assignments.tqh_first;
    while !assignment.is_null() {
        if strcmp((*assignment).name, (*ws).name) == 0 as ::core::ffi::c_int {
            gaps = (*assignment).gaps;
            mask = (*assignment).gaps_mask;
            break;
        } else {
            if (*ws).num != -(1 as ::core::ffi::c_int)
                && name_is_digits((*assignment).name) as ::core::ffi::c_int != 0
                && ws_name_to_number((*assignment).name) == (*ws).num
            {
                gaps = (*assignment).gaps;
                mask = (*assignment).gaps_mask;
            }
            assignment = (*assignment).ws_assignments.tqe_next;
        }
    }
    if mask as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint {
        return gaps;
    }
    if mask as ::core::ffi::c_uint & GAPS_INNER as ::core::ffi::c_int as ::core::ffi::c_uint != 0 {
        gaps.inner -= config.gaps.inner;
    }
    if mask as ::core::ffi::c_uint & GAPS_TOP as ::core::ffi::c_int as ::core::ffi::c_uint != 0 {
        gaps.top -= config.gaps.top;
    }
    if mask as ::core::ffi::c_uint & GAPS_RIGHT as ::core::ffi::c_int as ::core::ffi::c_uint != 0 {
        gaps.right -= config.gaps.right;
    }
    if mask as ::core::ffi::c_uint & GAPS_BOTTOM as ::core::ffi::c_int as ::core::ffi::c_uint != 0 {
        gaps.bottom -= config.gaps.bottom;
    }
    if mask as ::core::ffi::c_uint & GAPS_LEFT as ::core::ffi::c_int as ::core::ffi::c_uint != 0 {
        gaps.left -= config.gaps.left;
    }
    gaps
}

pub unsafe fn gaps_reapply_workspace_assignments() {
    let mut output: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut workspace: *mut Con = ::core::ptr::null_mut::<Con>();
    output = (*croot).nodes_head.tqh_first as *mut Con;
    while !output.is_null() {
        let content: *mut Con = output_get_content(output);
        workspace = (*content).nodes_head.tqh_first as *mut Con;
        while !workspace.is_null() {
            debuglog(
                c"%s:%s:%d - updating gap assignments for workspace %s\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/gaps.c".as_ptr(),
                c"gaps_reapply_workspace_assignments".as_ptr(),
                171 as ::core::ffi::c_int,
                (*workspace).name,
            );
            (*workspace).gaps = gaps_for_workspace(workspace);
            workspace = (*workspace).nodes.tqe_next as *mut Con;
        }
        output = (*output).nodes.tqe_next as *mut Con;
    }
}

// Internal helpers use Rust linkage rather than foreign symbol lookup.
use crate::util::name_is_digits;
use crate::util::ws_name_to_number;

// Canonical Cairo backend ABI; renderer logic remains local.

// Canonical Pango backend ABI; font policy remains local.

pub use libc::strcmp;

use crate::log::debuglog;

// Shared model types retain field layout while using one Rust identity.
pub use crate::types::gaps_t;
pub use crate::types::Con;
pub use i3_common::font::Font;
pub use i3_common::string::I3String as _i3String;

// Internal calls and globals use canonical Rust identities.
use crate::con::con_get_fullscreen_con;
use crate::con::con_get_workspace;
use crate::con::con_has_parent;
use crate::con::con_inside_floating;
use crate::con::con_inside_stacked_or_tabbed;
use crate::con::con_is_leaf;
use crate::con::con_num_children;
use crate::con::con_num_visible_children;
use crate::config::config;
use crate::output::output_get_content;
use crate::resize::resize_find_tiling_participants;
use crate::tree::croot;
