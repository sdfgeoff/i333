/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * move.c: Moving containers into some direction.
 *
 */
// Mechanical file-by-file Rust translation of src/move.c.
// Originally translated with C2Rust 0.22.1; subsequently maintained in Rust.
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __time_t = ::core::ffi::c_long;
pub type __suseconds_t = ::core::ffi::c_long;
pub type time_t = __time_t;

pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;

pub type i3String = _i3String;

pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const SCRATCHPAD_CHANGED: C2RustUnnamed_0 = 2;
pub const SCRATCHPAD_FRESH: C2RustUnnamed_0 = 1;
pub const SCRATCHPAD_NONE: C2RustUnnamed_0 = 0;

pub type C2RustUnnamed_5 = ::core::ffi::c_uint;
pub const FLOATING_USER_ON: C2RustUnnamed_5 = 3;
pub const FLOATING_AUTO_ON: C2RustUnnamed_5 = 2;
pub const FLOATING_USER_OFF: C2RustUnnamed_5 = 1;
pub const FLOATING_AUTO_OFF: C2RustUnnamed_5 = 0;
pub type border_style_t = ::core::ffi::c_uint;
pub const BS_NORMAL: border_style_t = 2;
pub const BS_PIXEL: border_style_t = 1;
pub const BS_NONE: border_style_t = 0;
pub type layout_t = ::core::ffi::c_uint;
pub const L_SPLITH: layout_t = 6;
pub const L_SPLITV: layout_t = 5;
pub const L_OUTPUT: layout_t = 4;
pub const L_DOCKAREA: layout_t = 3;
pub const L_TABBED: layout_t = 2;
pub const L_STACKED: layout_t = 1;
pub const L_DEFAULT: layout_t = 0;
pub type fullscreen_mode_t = ::core::ffi::c_uint;
pub const CF_GLOBAL: fullscreen_mode_t = 2;
pub const CF_OUTPUT: fullscreen_mode_t = 1;
pub const CF_NONE: fullscreen_mode_t = 0;

pub type C2RustUnnamed_7 = ::core::ffi::c_uint;
pub const M_BELOW: C2RustUnnamed_7 = 2;
pub const M_ASSIGN_WS: C2RustUnnamed_7 = 1;
pub const M_HERE: C2RustUnnamed_7 = 0;
pub type C2RustUnnamed_8 = ::core::ffi::c_uint;
pub const WM_FLOATING: C2RustUnnamed_8 = 6;
pub const WM_FLOATING_USER: C2RustUnnamed_8 = 5;
pub const WM_FLOATING_AUTO: C2RustUnnamed_8 = 4;
pub const WM_TILING: C2RustUnnamed_8 = 3;
pub const WM_TILING_USER: C2RustUnnamed_8 = 2;
pub const WM_TILING_AUTO: C2RustUnnamed_8 = 1;
pub const WM_ANY: C2RustUnnamed_8 = 0;
pub type C2RustUnnamed_9 = ::core::ffi::c_int;
pub const M_DOCK_BOTTOM: C2RustUnnamed_9 = 3;
pub const M_DOCK_TOP: C2RustUnnamed_9 = 2;
pub const M_DOCK_ANY: C2RustUnnamed_9 = 1;
pub const M_NODOCK: C2RustUnnamed_9 = 0;
pub const M_DONTCHECK: C2RustUnnamed_9 = -1;
pub type C2RustUnnamed_10 = ::core::ffi::c_int;
pub const U_OLDEST: C2RustUnnamed_10 = 1;
pub const U_LATEST: C2RustUnnamed_10 = 0;
pub const U_DONTCHECK: C2RustUnnamed_10 = -1;

pub type C2RustUnnamed_11 = ::core::ffi::c_uint;
pub const W_DOCK_BOTTOM: C2RustUnnamed_11 = 2;
pub const W_DOCK_TOP: C2RustUnnamed_11 = 1;
pub const W_NODOCK: C2RustUnnamed_11 = 0;

pub type C2RustUnnamed_14 = ::core::ffi::c_uint;
pub const A_TO_OUTPUT: C2RustUnnamed_14 = 16;
pub const A_TO_WORKSPACE_NUMBER: C2RustUnnamed_14 = 8;
pub const A_NO_FOCUS: C2RustUnnamed_14 = 4;
pub const A_TO_WORKSPACE: C2RustUnnamed_14 = 2;
pub const A_COMMAND: C2RustUnnamed_14 = 1;
pub const A_ANY: C2RustUnnamed_14 = 0;

pub type C2RustUnnamed_16 = ::core::ffi::c_uint;
pub const CT_DOCKAREA: C2RustUnnamed_16 = 5;
pub const CT_WORKSPACE: C2RustUnnamed_16 = 4;
pub const CT_FLOATING_CON: C2RustUnnamed_16 = 3;
pub const CT_CON: C2RustUnnamed_16 = 2;
pub const CT_OUTPUT: C2RustUnnamed_16 = 1;
pub const CT_ROOT: C2RustUnnamed_16 = 0;

pub type Output = xoutput;
pub type i3Window = Window;
pub type direction_t = ::core::ffi::c_uint;
pub const D_DOWN: direction_t = 3;
pub const D_UP: direction_t = 2;
pub const D_RIGHT: direction_t = 1;
pub const D_LEFT: direction_t = 0;
pub type orientation_t = ::core::ffi::c_uint;
pub const VERT: orientation_t = 2;
pub const HORIZ: orientation_t = 1;
pub const NO_ORIENTATION: orientation_t = 0;
pub type position_t = ::core::ffi::c_uint;
pub const AFTER: position_t = 1;
pub const BEFORE: position_t = 0;
pub type output_close_far_t = ::core::ffi::c_uint;
pub const FARTHEST_OUTPUT: output_close_far_t = 1;
pub const CLOSEST_OUTPUT: output_close_far_t = 0;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
unsafe fn lowest_common_ancestor(a: *mut Con, b: *mut Con) -> *mut Con {
    let mut parent_a: *mut Con = a;
    while !parent_a.is_null() {
        let mut parent_b: *mut Con = b;
        while !parent_b.is_null() {
            if parent_a == parent_b {
                return parent_a;
            }
            parent_b = (*parent_b).parent as *mut Con;
        }
        parent_a = (*parent_a).parent as *mut Con;
    }
    if 0 as ::core::ffi::c_int != 0 {
    } else {
        __assert_fail(
            c"false".as_ptr(),
            c"../src/move.c".as_ptr(),
            28 as ::core::ffi::c_uint,
            c"Con *lowest_common_ancestor(Con *, Con *)".as_ptr(),
        );
    };
    panic!("Reached end of non-void function without returning");
}
unsafe fn child_containing_con_recursively(ancestor: *mut Con, con: *mut Con) -> *mut Con {
    let mut child: *mut Con = con;
    while !child.is_null() && (*child).parent != ancestor {
        child = (*child).parent as *mut Con;
        if !(*child).parent.is_null() {
        } else {
            __assert_fail(
                c"child->parent".as_ptr(),
                c"../src/move.c".as_ptr(),
                39 as ::core::ffi::c_uint,
                c"Con *child_containing_con_recursively(Con *, Con *)".as_ptr(),
            );
        };
    }
    child
}
unsafe fn is_focused_descendant(con: *mut Con, ancestor: *mut Con) -> bool {
    let mut current: *mut Con = con;
    while current != ancestor {
        if (*(*current).parent).focus_head.tqh_first != current {
            return false_0 != 0;
        }
        current = (*current).parent as *mut Con;
        if !(*current).parent.is_null() {
        } else {
            __assert_fail(
                c"current->parent".as_ptr(),
                c"../src/move.c".as_ptr(),
                55 as ::core::ffi::c_uint,
                c"_Bool is_focused_descendant(Con *, Con *)".as_ptr(),
            );
        };
    }
    true_0 != 0
}

pub unsafe fn insert_con_into(mut con: *mut Con, target: *mut Con, position: position_t) {
    let parent: *mut Con = (*target).parent as *mut Con;
    let old_parent: *mut Con = (*con).parent as *mut Con;
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
    let mut focus_before: bool = false;
    if con_ancestor == target_ancestor {
        focus_before = moves_focus_from_ancestor;
    } else {
        let mut current: *mut Con = ::core::ptr::null_mut::<Con>();
        current = (*lca).focus_head.tqh_first as *mut Con;
        while !current.is_null() {
            if current == con_ancestor || current == target_ancestor {
                break;
            }
            current = (*current).focused.tqe_next as *mut Con;
        }
        focus_before = current == con_ancestor;
    }
    if moves_focus_from_ancestor as ::core::ffi::c_int != 0
        && focus_before as ::core::ffi::c_int != 0
    {
        let place: *mut Con = *(*((*con_ancestor).focused.tqe_prev as *mut focus_head)).tqh_last;
        if !(*target_ancestor).focused.tqe_next.is_null() {
            (*(*target_ancestor).focused.tqe_next).focused.tqe_prev =
                (*target_ancestor).focused.tqe_prev;
        } else {
            (*lca).focus_head.tqh_last = (*target_ancestor).focused.tqe_prev;
        }
        *(*target_ancestor).focused.tqe_prev = (*target_ancestor).focused.tqe_next;
        if !place.is_null() {
            (*target_ancestor).focused.tqe_next = (*place).focused.tqe_next;
            if !(*target_ancestor).focused.tqe_next.is_null() {
                (*(*target_ancestor).focused.tqe_next).focused.tqe_prev =
                    &raw mut (*target_ancestor).focused.tqe_next;
            } else {
                (*lca).focus_head.tqh_last = &raw mut (*target_ancestor).focused.tqe_next;
            }
            (*place).focused.tqe_next = target_ancestor as *mut Con;
            (*target_ancestor).focused.tqe_prev = &raw mut (*place).focused.tqe_next;
        } else {
            (*target_ancestor).focused.tqe_next = (*lca).focus_head.tqh_first;
            if !(*target_ancestor).focused.tqe_next.is_null() {
                (*(*lca).focus_head.tqh_first).focused.tqe_prev =
                    &raw mut (*target_ancestor).focused.tqe_next;
            } else {
                (*lca).focus_head.tqh_last = &raw mut (*target_ancestor).focused.tqe_next;
            }
            (*lca).focus_head.tqh_first = target_ancestor as *mut Con;
            (*target_ancestor).focused.tqe_prev = &raw mut (*lca).focus_head.tqh_first;
        }
    }
    con_detach(con);
    con_fix_percent((*con).parent as *mut Con);
    if (*parent).type_0 as ::core::ffi::c_uint
        == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let split: *mut Con = workspace_attach_to(parent);
        if split != parent {
            debuglog(
                c"%s:%s:%d - Got a new split con, using that one instead\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/move.c".as_ptr(),
                c"insert_con_into".as_ptr(),
                126 as ::core::ffi::c_int,
            );
            (*con).parent = split as *mut Con;
            con_attach(con, split, false_0 != 0);
            debuglog(
                c"%s:%s:%d - attached\n".as_ptr().cast_mut(),
                c"../src/move.c".as_ptr(),
                c"insert_con_into".as_ptr(),
                129 as ::core::ffi::c_int,
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
                133 as ::core::ffi::c_int,
                con,
            );
            con_detach(con);
        }
    }
    (*con).parent = parent as *mut Con;
    if parent == lca {
        if focus_before {
            (*con).focused.tqe_prev = (*target).focused.tqe_prev;
            (*con).focused.tqe_next = target as *mut Con;
            *(*target).focused.tqe_prev = con as *mut Con;
            (*target).focused.tqe_prev = &raw mut (*con).focused.tqe_next;
        } else {
            (*con).focused.tqe_next = (*target).focused.tqe_next;
            if !(*con).focused.tqe_next.is_null() {
                (*(*con).focused.tqe_next).focused.tqe_prev = &raw mut (*con).focused.tqe_next;
            } else {
                (*parent).focus_head.tqh_last = &raw mut (*con).focused.tqe_next;
            }
            (*target).focused.tqe_next = con as *mut Con;
            (*con).focused.tqe_prev = &raw mut (*target).focused.tqe_next;
        }
    } else if focus_before {
        (*con).focused.tqe_next = (*parent).focus_head.tqh_first;
        if !(*con).focused.tqe_next.is_null() {
            (*(*parent).focus_head.tqh_first).focused.tqe_prev = &raw mut (*con).focused.tqe_next;
        } else {
            (*parent).focus_head.tqh_last = &raw mut (*con).focused.tqe_next;
        }
        (*parent).focus_head.tqh_first = con as *mut Con;
        (*con).focused.tqe_prev = &raw mut (*parent).focus_head.tqh_first;
    } else {
        (*con).focused.tqe_next = ::core::ptr::null_mut::<Con>();
        (*con).focused.tqe_prev = (*parent).focus_head.tqh_last;
        *(*parent).focus_head.tqh_last = con as *mut Con;
        (*parent).focus_head.tqh_last = &raw mut (*con).focused.tqe_next;
    }
    if position as ::core::ffi::c_uint == BEFORE as ::core::ffi::c_int as ::core::ffi::c_uint {
        (*con).nodes.tqe_prev = (*target).nodes.tqe_prev;
        (*con).nodes.tqe_next = target as *mut Con;
        *(*target).nodes.tqe_prev = con as *mut Con;
        (*target).nodes.tqe_prev = &raw mut (*con).nodes.tqe_next;
    } else if position as ::core::ffi::c_uint == AFTER as ::core::ffi::c_int as ::core::ffi::c_uint
    {
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
unsafe fn attach_to_workspace(con: *mut Con, ws: *mut Con, direction: direction_t) {
    con_detach(con);
    let old_parent: *mut Con = (*con).parent as *mut Con;
    (*con).parent = ws as *mut Con;
    if direction as ::core::ffi::c_uint == D_RIGHT as ::core::ffi::c_int as ::core::ffi::c_uint
        || direction as ::core::ffi::c_uint == D_DOWN as ::core::ffi::c_int as ::core::ffi::c_uint
    {
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
    (*con).focused.tqe_next = ::core::ptr::null_mut::<Con>();
    (*con).focused.tqe_prev = (*ws).focus_head.tqh_last;
    *(*ws).focus_head.tqh_last = con as *mut Con;
    (*ws).focus_head.tqh_last = &raw mut (*con).focused.tqe_next;
    (*con).percent = 0.0f64;
    con_fix_percent(ws);
    con_fix_percent(old_parent);
    (*old_parent)
        .on_remove_child
        .expect("non-null function pointer")(old_parent);
}
unsafe fn move_to_output_directed(con: *mut Con, direction: direction_t) {
    let current_output: *mut Output = get_output_for_con(con);
    let output: *mut Output = get_output_next(direction, current_output, CLOSEST_OUTPUT);
    if output.is_null() {
        debuglog(
            c"%s:%s:%d - No output in this direction found. Not moving.\n"
                .as_ptr()
                .cast_mut(),
            c"../src/move.c".as_ptr(),
            c"move_to_output_directed".as_ptr(),
            211 as ::core::ffi::c_int,
        );
        return;
    }
    let mut ws: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut child: *mut Con = -(1 as ::core::ffi::c_int) as *mut Con;
    while child == -(1 as ::core::ffi::c_int) as *mut Con && {
        child = ::core::ptr::null_mut::<Con>();
        true_0 != 0
    } {
        child = (*output_get_content((*output).con)).nodes_head.tqh_first as *mut Con;
        while !child.is_null() {
            if !workspace_is_visible(child) {
                child = (*child).nodes.tqe_next as *mut Con;
            } else {
                ws = child;
                break;
            }
        }
    }
    if ws.is_null() {
        debuglog(
            c"%s:%s:%d - No workspace on output in this direction found. Not moving.\n"
                .as_ptr()
                .cast_mut(),
            c"../src/move.c".as_ptr(),
            c"move_to_output_directed".as_ptr(),
            219 as ::core::ffi::c_int,
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

pub unsafe fn tree_move(con: *mut Con, direction: direction_t) {
    let mut above: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut next: *mut Con = ::core::ptr::null_mut::<Con>();
    let current_block: u64;
    let mut position: position_t = BEFORE;
    let mut target: *mut Con = ::core::ptr::null_mut::<Con>();
    debuglog(
        c"%s:%s:%d - Moving in direction %d\n".as_ptr().cast_mut(),
        c"../src/move.c".as_ptr(),
        c"tree_move".as_ptr(),
        263 as ::core::ffi::c_int,
        direction as ::core::ffi::c_uint,
    );
    if (*con).type_0 as ::core::ffi::c_uint
        == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        debuglog(
            c"%s:%s:%d - Not moving workspace\n".as_ptr().cast_mut(),
            c"../src/move.c".as_ptr(),
            c"tree_move".as_ptr(),
            268 as ::core::ffi::c_int,
        );
        return;
    }
    if (*con).fullscreen_mode as ::core::ffi::c_uint
        == CF_GLOBAL as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        debuglog(
            c"%s:%s:%d - Not moving fullscreen global container\n"
                .as_ptr()
                .cast_mut(),
            c"../src/move.c".as_ptr(),
            c"tree_move".as_ptr(),
            273 as ::core::ffi::c_int,
        );
        return;
    }
    if (*con).fullscreen_mode as ::core::ffi::c_uint
        == CF_OUTPUT as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*(*con).parent).type_0 as ::core::ffi::c_uint
            == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
            && con_num_children((*con).parent as *mut Con) == 1 as ::core::ffi::c_int
    {
        move_to_output_directed(con, direction);
        return;
    }
    let o: orientation_t = orientation_from_direction(direction);
    let mut same_orientation: *mut Con = con_parent_with_orientation(con, o);
    loop {
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
                    301 as ::core::ffi::c_int,
                );
                attach_to_workspace(con, con_get_workspace(con), direction);
                current_block = 11393196477283502759;
                break;
            } else {
                debuglog(
                    c"%s:%s:%d - Force-changing orientation\n"
                        .as_ptr()
                        .cast_mut(),
                    c"../src/move.c".as_ptr(),
                    c"tree_move".as_ptr(),
                    305 as ::core::ffi::c_int,
                );
                ws_force_orientation(con_get_workspace(con), o);
                same_orientation = con_parent_with_orientation(con, o);
            }
        }
        if same_orientation == (*con).parent {
            let swap: *mut Con = if direction as ::core::ffi::c_uint
                == D_LEFT as ::core::ffi::c_int as ::core::ffi::c_uint
                || direction as ::core::ffi::c_uint
                    == D_UP as ::core::ffi::c_int as ::core::ffi::c_uint
            {
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
                        317 as ::core::ffi::c_int,
                    );
                    target = con_descend_direction(swap, direction);
                    position = (if con_orientation((*target).parent as *mut Con)
                        as ::core::ffi::c_uint
                        != o as ::core::ffi::c_uint
                        || direction as ::core::ffi::c_uint
                            == D_UP as ::core::ffi::c_int as ::core::ffi::c_uint
                        || direction as ::core::ffi::c_uint
                            == D_LEFT as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        AFTER as ::core::ffi::c_int
                    } else {
                        BEFORE as ::core::ffi::c_int
                    }) as position_t;
                    insert_con_into(con, target, position);
                    current_block = 11393196477283502759;
                    break;
                } else {
                    debuglog(
                        c"%s:%s:%d - Swapping with sibling.\n".as_ptr().cast_mut(),
                        c"../src/move.c".as_ptr(),
                        c"tree_move".as_ptr(),
                        328 as ::core::ffi::c_int,
                    );
                    if direction as ::core::ffi::c_uint
                        == D_LEFT as ::core::ffi::c_int as ::core::ffi::c_uint
                        || direction as ::core::ffi::c_uint
                            == D_UP as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        *(*swap).nodes.tqe_prev = con as *mut Con;
                        (*con).nodes.tqe_prev = (*swap).nodes.tqe_prev;
                        (*swap).nodes.tqe_prev = &raw mut (*con).nodes.tqe_next;
                        (*swap).nodes.tqe_next = (*con).nodes.tqe_next;
                        if !(*con).nodes.tqe_next.is_null() {
                            (*(*con).nodes.tqe_next).nodes.tqe_prev =
                                &raw mut (*swap).nodes.tqe_next;
                        }
                        (*con).nodes.tqe_next = swap as *mut Con;
                        if (*(*swap).parent).nodes_head.tqh_last == &raw mut (*con).nodes.tqe_next {
                            (*(*swap).parent).nodes_head.tqh_last = &raw mut (*swap).nodes.tqe_next;
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
                        if (*(*swap).parent).nodes_head.tqh_last == &raw mut (*swap).nodes.tqe_next
                        {
                            (*(*swap).parent).nodes_head.tqh_last = &raw mut (*con).nodes.tqe_next;
                        }
                    }
                    con_force_split_parents_redraw(con);
                    ipc_send_window_event(c"move".as_ptr(), con);
                    return;
                }
            } else {
                if (*con).parent == con_get_workspace(con) {
                    move_to_output_directed(con, direction);
                    return;
                }
                same_orientation = con_parent_with_orientation((*con).parent as *mut Con, o);
            }
        }
        if !same_orientation.is_null() {
            current_block = 11777552016271000781;
            break;
        }
    }
    match current_block {
        11777552016271000781 => {
            above = con;
            while (*above).parent != same_orientation {
                above = (*above).parent as *mut Con;
            }
            if !con_fullscreen_permits_focusing((*above).parent as *mut Con) {
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
                369 as ::core::ffi::c_int,
                above,
            );
            next = if direction as ::core::ffi::c_uint
                == D_UP as ::core::ffi::c_int as ::core::ffi::c_uint
                || direction as ::core::ffi::c_uint
                    == D_LEFT as ::core::ffi::c_int as ::core::ffi::c_uint
            {
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
                    374 as ::core::ffi::c_int,
                );
                target = con_descend_direction(next, direction);
                position = (if con_orientation((*target).parent as *mut Con) as ::core::ffi::c_uint
                    != o as ::core::ffi::c_uint
                    || direction as ::core::ffi::c_uint
                        == D_UP as ::core::ffi::c_int as ::core::ffi::c_uint
                    || direction as ::core::ffi::c_uint
                        == D_LEFT as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    AFTER as ::core::ffi::c_int
                } else {
                    BEFORE as ::core::ffi::c_int
                }) as position_t;
                insert_con_into(con, target, position);
            } else if next.is_null()
                && (*(*(*con).parent).parent).type_0 as ::core::ffi::c_uint
                    == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
                && (*(*con).parent).layout as ::core::ffi::c_uint
                    != L_DEFAULT as ::core::ffi::c_int as ::core::ffi::c_uint
                && con_num_children((*con).parent as *mut Con) == 1 as ::core::ffi::c_int
            {
                debuglog(
                    c"%s:%s:%d - Grandparent is workspace\n".as_ptr().cast_mut(),
                    c"../src/move.c".as_ptr(),
                    c"tree_move".as_ptr(),
                    389 as ::core::ffi::c_int,
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
                    393 as ::core::ffi::c_int,
                );
                position = (if direction as ::core::ffi::c_uint
                    == D_UP as ::core::ffi::c_int as ::core::ffi::c_uint
                    || direction as ::core::ffi::c_uint
                        == D_LEFT as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    BEFORE as ::core::ffi::c_int
                } else {
                    AFTER as ::core::ffi::c_int
                }) as position_t;
                insert_con_into(con, above, position);
            }
        }
        _ => {}
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

pub use libc::free;

use i3_common::native_stdio::__assert_fail;

use crate::log::{debuglog, errorlog, verboselog};

// Shared model types retain field layout while using one Rust identity.
pub use crate::types::deco_render_params;
pub use crate::types::focus_head;
pub use crate::types::nodes_head;
pub use crate::types::xoutput;
pub use crate::types::Con;
pub use crate::types::Window;
pub use i3_common::string::I3String as _i3String;

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
