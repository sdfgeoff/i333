use crate::con::arena::parent::{parent_ptr, set_parent_ptr};
use crate::con::arena::{focus, identity};
use crate::types::Direction;
use crate::types::Layout;
use crate::types::Orientation;
use crate::types::OutputSelection;
use crate::types::Position;
/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * move.c: Moving containers into some direction.
 *
 */
// Local policy and temporary values use owned Rust storage. Native entrypoints
// require live event-thread graph handles and a live display; callbacks may mutate
// the graph, so node references must never span those calls.

const CF_GLOBAL: u32 = 2;
const CF_OUTPUT: u32 = 1;
const CT_WORKSPACE: u32 = 4;
type Output = xoutput;
type i3Window = Window;

fn backward(direction: Direction) -> bool {
    matches!(direction, Direction::Left | Direction::Up)
}
fn branch_position(direction: Direction, aligned: bool) -> Position {
    if !aligned || backward(direction) {
        Position::After
    } else {
        Position::Before
    }
}
fn common_ancestor<T: Copy + Eq>(first: &[T], second: &[T]) -> Option<T> {
    first.iter().copied().find(|node| second.contains(node))
}
fn containing_child<T: Copy + Eq>(path: &[T], ancestor: T) -> Option<T> {
    path.windows(2)
        .find(|pair| pair[1] == ancestor)
        .map(|pair| pair[0])
}
fn focus_precedes<T: Copy + Eq>(order: &[T], moving: T, target: T) -> bool {
    order
        .iter()
        .find(|&&node| node == moving || node == target)
        .is_some_and(|&node| node == moving)
}

// Snapshot only opaque native handles. They are never dereferenced by the safe
// ancestry/order algorithms, and no snapshot survives a graph-mutating callback.
unsafe fn ancestor_path(mut node: *mut Con) -> Vec<*mut Con> {
    let mut path = Vec::new();
    while !node.is_null() {
        path.push(node);
        node = parent_ptr(node);
    }
    path
}
unsafe fn lowest_common_ancestor(a: *mut Con, b: *mut Con) -> *mut Con {
    common_ancestor(&ancestor_path(a), &ancestor_path(b)).expect("containers must share a root")
}
unsafe fn child_containing_con_recursively(ancestor: *mut Con, con: *mut Con) -> *mut Con {
    containing_child(&ancestor_path(con), ancestor).expect("container must descend from ancestor")
}
unsafe fn is_focused_descendant(mut con: *mut Con, ancestor: *mut Con) -> bool {
    while con != ancestor {
        let parent = parent_ptr(con);
        assert!(
            !parent.is_null(),
            "container must descend from focus ancestor"
        );
        if focus::first_ptr(parent) != con {
            return false;
        }
        con = parent;
    }
    true
}

pub unsafe fn insert_con_into(mut con: *mut Con, target: *mut Con, position: Position) {
    let parent: *mut Con = parent_ptr(target);
    let old_parent: *mut Con = parent_ptr(con);
    let lca: *mut Con = lowest_common_ancestor(con, parent);
    if lca == con {
        errorlog(
            c"ERROR: Container is being inserted into one of its descendants.\n"
                .as_ptr()
                .cast_mut(),
        );
        return;
    }
    let con_ancestor: *mut Con = child_containing_con_recursively(lca, con);
    let target_ancestor: *mut Con = child_containing_con_recursively(lca, target);
    let moves_focus_from_ancestor: bool = is_focused_descendant(con, con_ancestor);
    let focus_before = if con_ancestor == target_ancestor {
        moves_focus_from_ancestor
    } else {
        let mut order = Vec::new();
        let mut current = focus::first_ptr(lca);
        while !current.is_null() {
            order.push(current);
            if current == con_ancestor || current == target_ancestor {
                break;
            }
            current = focus::next_ptr(current);
        }
        focus_precedes(&order, con_ancestor, target_ancestor)
    };
    if moves_focus_from_ancestor && focus_before {
        let owner = identity(lca).expect("live focus owner");
        let moving = identity(target_ancestor).expect("live target ancestor");
        let place = identity(focus::previous_ptr(con_ancestor));
        if let Some(place) = place {
            focus::move_after(owner, moving, place).expect("existing focus members");
        } else {
            focus::move_to_front(owner, moving).expect("existing focus member");
        }
    }
    con_detach(con);
    con_fix_percent(parent_ptr(con));
    if (*parent).type_0 as u32 == CT_WORKSPACE {
        let split: *mut Con = workspace_attach_to(parent);
        if split != parent {
            debuglog(
                c"%s:%s:%d - Got a new split con, using that one instead\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/move.c".as_ptr(),
                c"insert_con_into".as_ptr(),
                126,
            );
            set_parent_ptr(con, split as *mut Con).expect("live acyclic ancestry");
            con_attach(con, split, false);
            debuglog(
                c"%s:%s:%d - attached\n".as_ptr().cast_mut(),
                c"../src/move.c".as_ptr(),
                c"insert_con_into".as_ptr(),
                129,
            );
            (*con).percent = 0.0f64;
            con_fix_percent(split);
            con = split;
            debuglog(
                c"%s:%s:%d - ok, continuing with con %p instead\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/move.c".as_ptr(),
                c"insert_con_into".as_ptr(),
                133,
                con,
            );
            con_detach(con);
        }
    }
    set_parent_ptr(con, parent as *mut Con).expect("live acyclic ancestry");
    let owner = identity(parent).expect("live focus owner");
    let child = identity(con).expect("live moved container");
    if parent == lca {
        let target = identity(target).expect("live insertion target");
        if focus_before {
            focus::insert_before(owner, child, target).expect("detached focus insertion");
        } else {
            focus::insert_after(owner, child, target).expect("detached focus insertion");
        }
    } else if focus_before {
        focus::prepend(owner, child).expect("detached focus insertion");
    } else {
        focus::append(owner, child).expect("detached focus insertion");
    }
    if position == Position::Before {
        (*con).nodes.tqe_prev = (*target).nodes.tqe_prev;
        (*con).nodes.tqe_next = target as *mut Con;
        *(*target).nodes.tqe_prev = con as *mut Con;
        (*target).nodes.tqe_prev = &raw mut (*con).nodes.tqe_next;
    } else if position == Position::After {
        (*con).nodes.tqe_next = (*target).nodes.tqe_next;
        if !(*con).nodes.tqe_next.is_null() {
            (*(*con).nodes.tqe_next).nodes.tqe_prev = &raw mut (*con).nodes.tqe_next;
        } else {
            (*parent).nodes_head.tqh_last = &raw mut (*con).nodes.tqe_next;
        }
        (*target).nodes.tqe_next = con as *mut Con;
        (*con).nodes.tqe_prev = &raw mut (*target).nodes.tqe_next;
    }
    (*con).percent = 0.0f64;
    con_fix_percent(parent);
    (*old_parent)
        .on_remove_child
        .expect("non-null function pointer")(old_parent);
}
unsafe fn attach_to_workspace(con: *mut Con, ws: *mut Con, direction: Direction) {
    con_detach(con);
    let old_parent: *mut Con = parent_ptr(con);
    set_parent_ptr(con, ws as *mut Con).expect("live acyclic ancestry");
    if direction == Direction::Right || direction == Direction::Down {
        (*con).nodes.tqe_next = (*ws).nodes_head.tqh_first;
        if !(*con).nodes.tqe_next.is_null() {
            (*(*ws).nodes_head.tqh_first).nodes.tqe_prev = &raw mut (*con).nodes.tqe_next;
        } else {
            (*ws).nodes_head.tqh_last = &raw mut (*con).nodes.tqe_next;
        }
        (*ws).nodes_head.tqh_first = con as *mut Con;
        (*con).nodes.tqe_prev = &raw mut (*ws).nodes_head.tqh_first;
    } else {
        (*con).nodes.tqe_next = ::core::ptr::null_mut::<Con>();
        (*con).nodes.tqe_prev = (*ws).nodes_head.tqh_last;
        *(*ws).nodes_head.tqh_last = con as *mut Con;
        (*ws).nodes_head.tqh_last = &raw mut (*con).nodes.tqe_next;
    }
    focus::append(
        identity(ws).expect("live workspace focus owner"),
        identity(con).expect("live moved container"),
    )
    .expect("detached focus insertion");
    (*con).percent = 0.0f64;
    con_fix_percent(ws);
    con_fix_percent(old_parent);
    (*old_parent)
        .on_remove_child
        .expect("non-null function pointer")(old_parent);
}
unsafe fn move_to_output_directed(con: *mut Con, direction: Direction) {
    let current_output: *mut Output = get_output_for_con(con);
    let output: *mut Output = get_output_next(direction, current_output, OutputSelection::Closest);
    if output.is_null() {
        debuglog(
            c"%s:%s:%d - No output in this direction found. Not moving.\n"
                .as_ptr()
                .cast_mut(),
            c"../src/move.c".as_ptr(),
            c"move_to_output_directed".as_ptr(),
            211,
        );
        return;
    }
    let mut ws: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut child = (*output_get_content((*output).con)).nodes_head.tqh_first;
    while !child.is_null() {
        if workspace_is_visible(child) {
            ws = child;
            break;
        }
        child = (*child).nodes.tqe_next;
    }
    if ws.is_null() {
        debuglog(
            c"%s:%s:%d - No workspace on output in this direction found. Not moving.\n"
                .as_ptr()
                .cast_mut(),
            c"../src/move.c".as_ptr(),
            c"move_to_output_directed".as_ptr(),
            219,
        );
        return;
    }
    let old_ws: *mut Con = con_get_workspace(con);
    let moves_focus: bool = focused == con;
    attach_to_workspace(con, ws, direction);
    if moves_focus {
        con_focus(con);
        focused = old_ws;
        workspace_show(ws);
        con_focus(con);
    }
    free((*con).deco_render_params as *mut ::core::ffi::c_void);
    (*con).deco_render_params = ::core::ptr::null_mut::<deco_render_params>();
    ipc_send_window_event(c"move".as_ptr(), con);
    tree_flatten(croot);
    ewmh_update_wm_desktop();
}

pub unsafe fn tree_move(con: *mut Con, direction: Direction) {
    let mut above: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut next: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut position: Position = Position::Before;
    let mut target: *mut Con = ::core::ptr::null_mut::<Con>();
    debuglog(
        c"%s:%s:%d - Moving in direction %d\n".as_ptr().cast_mut(),
        c"../src/move.c".as_ptr(),
        c"tree_move".as_ptr(),
        263,
        direction as u32,
    );
    if (*con).type_0 as u32 == CT_WORKSPACE {
        debuglog(
            c"%s:%s:%d - Not moving workspace\n".as_ptr().cast_mut(),
            c"../src/move.c".as_ptr(),
            c"tree_move".as_ptr(),
            268,
        );
        return;
    }
    if (*con).fullscreen_mode as u32 == CF_GLOBAL {
        debuglog(
            c"%s:%s:%d - Not moving fullscreen global container\n"
                .as_ptr()
                .cast_mut(),
            c"../src/move.c".as_ptr(),
            c"tree_move".as_ptr(),
            273,
        );
        return;
    }
    if (*con).fullscreen_mode as u32 == CF_OUTPUT
        || (*parent_ptr(con)).type_0 as u32 == CT_WORKSPACE
            && con_num_children(parent_ptr(con)) == 1
    {
        move_to_output_directed(con, direction);
        return;
    }
    let o: Orientation = orientation_from_direction(direction);
    let mut same_orientation: *mut Con = con_parent_with_orientation(con, o);
    let needs_ancestor_insert = loop {
        if same_orientation.is_null() {
            if con_is_floating(con) {
                floating_disable(con);
                run_assignments((*con).window as *mut i3Window);
                return;
            }
            if !con_inside_floating(con).is_null() {
                debuglog(
                    c"%s:%s:%d - Inside floating, moving to workspace\n"
                        .as_ptr()
                        .cast_mut(),
                    c"../src/move.c".as_ptr(),
                    c"tree_move".as_ptr(),
                    301,
                );
                attach_to_workspace(con, con_get_workspace(con), direction);
                break false;
            } else {
                debuglog(
                    c"%s:%s:%d - Force-changing orientation\n"
                        .as_ptr()
                        .cast_mut(),
                    c"../src/move.c".as_ptr(),
                    c"tree_move".as_ptr(),
                    305,
                );
                ws_force_orientation(con_get_workspace(con), o);
                same_orientation = con_parent_with_orientation(con, o);
            }
        }
        if same_orientation == parent_ptr(con) {
            let swap: *mut Con = if backward(direction) {
                *(*((*con).nodes.tqe_prev as *mut nodes_head)).tqh_last
            } else {
                (*con).nodes.tqe_next as *mut Con
            };
            if !swap.is_null() {
                if !con_is_leaf(swap) {
                    debuglog(
                        c"%s:%s:%d - Moving into our bordering branch\n"
                            .as_ptr()
                            .cast_mut(),
                        c"../src/move.c".as_ptr(),
                        c"tree_move".as_ptr(),
                        317,
                    );
                    target = con_descend_direction(swap, direction);
                    position = branch_position(direction, con_orientation(parent_ptr(target)) == o);
                    insert_con_into(con, target, position);
                    break false;
                } else {
                    debuglog(
                        c"%s:%s:%d - Swapping with sibling.\n".as_ptr().cast_mut(),
                        c"../src/move.c".as_ptr(),
                        c"tree_move".as_ptr(),
                        328,
                    );
                    if backward(direction) {
                        *(*swap).nodes.tqe_prev = con as *mut Con;
                        (*con).nodes.tqe_prev = (*swap).nodes.tqe_prev;
                        (*swap).nodes.tqe_prev = &raw mut (*con).nodes.tqe_next;
                        (*swap).nodes.tqe_next = (*con).nodes.tqe_next;
                        if !(*con).nodes.tqe_next.is_null() {
                            (*(*con).nodes.tqe_next).nodes.tqe_prev =
                                &raw mut (*swap).nodes.tqe_next;
                        }
                        (*con).nodes.tqe_next = swap as *mut Con;
                        if (*parent_ptr(swap)).nodes_head.tqh_last == &raw mut (*con).nodes.tqe_next
                        {
                            (*parent_ptr(swap)).nodes_head.tqh_last =
                                &raw mut (*swap).nodes.tqe_next;
                        }
                    } else {
                        *(*con).nodes.tqe_prev = swap as *mut Con;
                        (*swap).nodes.tqe_prev = (*con).nodes.tqe_prev;
                        (*con).nodes.tqe_prev = &raw mut (*swap).nodes.tqe_next;
                        (*con).nodes.tqe_next = (*swap).nodes.tqe_next;
                        if !(*swap).nodes.tqe_next.is_null() {
                            (*(*swap).nodes.tqe_next).nodes.tqe_prev =
                                &raw mut (*con).nodes.tqe_next;
                        }
                        (*swap).nodes.tqe_next = con as *mut Con;
                        if (*parent_ptr(swap)).nodes_head.tqh_last
                            == &raw mut (*swap).nodes.tqe_next
                        {
                            (*parent_ptr(swap)).nodes_head.tqh_last =
                                &raw mut (*con).nodes.tqe_next;
                        }
                    }
                    con_force_split_parents_redraw(con);
                    ipc_send_window_event(c"move".as_ptr(), con);
                    return;
                }
            } else {
                if parent_ptr(con) == con_get_workspace(con) {
                    move_to_output_directed(con, direction);
                    return;
                }
                same_orientation = con_parent_with_orientation(parent_ptr(con), o);
            }
        }
        if !same_orientation.is_null() {
            break true;
        }
    };
    if needs_ancestor_insert {
        above = con;
        while parent_ptr(above) != same_orientation {
            above = parent_ptr(above);
        }
        if !con_fullscreen_permits_focusing(parent_ptr(above)) {
            verboselog(
                c"Cannot move out of fullscreen container\n"
                    .as_ptr()
                    .cast_mut(),
            );
            return;
        }
        debuglog(
            c"%s:%s:%d - above = %p\n".as_ptr().cast_mut(),
            c"../src/move.c".as_ptr(),
            c"tree_move".as_ptr(),
            369,
            above,
        );
        next = if backward(direction) {
            *(*((*above).nodes.tqe_prev as *mut nodes_head)).tqh_last
        } else {
            (*above).nodes.tqe_next as *mut Con
        };
        if !next.is_null() && !con_is_leaf(next) {
            debuglog(
                c"%s:%s:%d - Moving into the bordering branch of our adjacent container\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/move.c".as_ptr(),
                c"tree_move".as_ptr(),
                374,
            );
            target = con_descend_direction(next, direction);
            position = branch_position(direction, con_orientation(parent_ptr(target)) == o);
            insert_con_into(con, target, position);
        } else if next.is_null()
            && (*parent_ptr(parent_ptr(con))).type_0 as u32 == CT_WORKSPACE
            && (*parent_ptr(con)).layout != Layout::Default
            && con_num_children(parent_ptr(con)) == 1
        {
            debuglog(
                c"%s:%s:%d - Grandparent is workspace\n".as_ptr().cast_mut(),
                c"../src/move.c".as_ptr(),
                c"tree_move".as_ptr(),
                389,
            );
            move_to_output_directed(con, direction);
            return;
        } else {
            debuglog(
                c"%s:%s:%d - Moving into container above\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/move.c".as_ptr(),
                c"tree_move".as_ptr(),
                393,
            );
            position = if backward(direction) {
                Position::Before
            } else {
                Position::After
            };
            insert_con_into(con, above, position);
        }
    }
    free((*con).deco_render_params as *mut ::core::ffi::c_void);
    (*con).deco_render_params = ::core::ptr::null_mut::<deco_render_params>();
    ipc_send_window_event(c"move".as_ptr(), con);
    tree_flatten(croot);
    ewmh_update_wm_desktop();
}

// Internal helpers use Rust linkage rather than foreign symbol lookup.
use crate::ewmh::ewmh_update_wm_desktop;
use crate::util::orientation_from_direction;

// Canonical Cairo backend ABI; renderer logic remains local.

use libc::free;

use crate::log::{debuglog, errorlog, verboselog};

// Shared model types retain field layout while using one Rust identity.
use crate::types::deco_render_params;
use crate::types::nodes_head;
use crate::types::xoutput;
use crate::types::Con;
use crate::types::Window;

// Internal calls and globals use canonical Rust identities.
use crate::assignments::run_assignments;
use crate::con::con_attach;
use crate::con::con_descend_direction;
use crate::con::con_detach;
use crate::con::con_fix_percent;
use crate::con::con_focus;
use crate::con::con_force_split_parents_redraw;
use crate::con::con_fullscreen_permits_focusing;
use crate::con::con_get_workspace;
use crate::con::con_inside_floating;
use crate::con::con_is_floating;
use crate::con::con_is_leaf;
use crate::con::con_num_children;
use crate::con::con_orientation;
use crate::con::con_parent_with_orientation;
use crate::floating::floating_disable;
use crate::ipc::ipc_send_window_event;
use crate::output::get_output_for_con;
use crate::output::output_get_content;
use crate::randr::get_output_next;
use crate::tree::croot;
use crate::tree::focused;
use crate::tree::tree_flatten;
use crate::workspace::workspace_attach_to;
use crate::workspace::workspace_is_visible;
use crate::workspace::workspace_show;
use crate::workspace::ws_force_orientation;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ancestor_and_focus_order_preserve_nearest_and_first_matches() {
        let first = [4, 2, 1];
        let second = [5, 3, 2, 1];
        assert_eq!(common_ancestor(&first, &second), Some(2));
        assert_eq!(containing_child(&second, 2), Some(3));
        assert_eq!(containing_child(&first, 4), None);
        assert!(focus_precedes(&[9, 2, 3], 2, 3));
        assert!(!focus_precedes(&[9, 3, 2], 2, 3));
        assert!(!focus_precedes(&[9], 2, 3));
    }
    #[test]
    fn branch_insertion_preserves_direction_and_orientation() {
        assert_eq!(branch_position(Direction::Left, true), Position::After);
        assert_eq!(branch_position(Direction::Down, true), Position::Before);
        assert_eq!(branch_position(Direction::Right, false), Position::After);
        assert_eq!(branch_position(Direction::Up, false), Position::After);
    }
}
