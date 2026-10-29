use crate::tree::all_cons;
use crate::types::BorderStyle;
use crate::types::Direction;
use crate::types::FloatingState;
use crate::types::KillWindow;
use crate::types::Layout;
use crate::types::MarkMode;
use crate::types::Orientation;
use i3_common::format_placeholders::format_placeholders;
use i3_common::safewrappers::sasprintf;
/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * con.c: Functions which deal with containers directly (creating containers,
 *        searching containers, getting specific properties from containers,
 *        …).
 *
 */
// Mechanical file-by-file Rust translation of src/con.c.
// Originally translated with C2Rust 0.22.1; subsequently maintained in Rust.
pub use i3_json::compat::yajl_gen;
use i3_json::compat::{yajl_gen_free, yajl_gen_get_buf};

pub type i3String = _i3String;

pub type i3Font = Font;

pub use i3_common::format_placeholders::placeholder_t;

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

pub type Output = xoutput;
pub type i3Window = Window;

pub type adjacent_t = ::core::ffi::c_uint;
pub const ADJ_LOWER_SCREEN_EDGE: adjacent_t = 16;
pub const ADJ_UPPER_SCREEN_EDGE: adjacent_t = 4;
pub const ADJ_RIGHT_SCREEN_EDGE: adjacent_t = 2;
pub const ADJ_LEFT_SCREEN_EDGE: adjacent_t = 1;
pub const ADJ_NONE: adjacent_t = 0;

pub type hide_edge_borders_mode_t = ::core::ffi::c_uint;
pub const HEBM_SMART_NO_GAPS: hide_edge_borders_mode_t = 64;
pub const HEBM_SMART: hide_edge_borders_mode_t = 32;
pub const HEBM_BOTH: hide_edge_borders_mode_t = 23;
pub const HEBM_HORIZONTAL: hide_edge_borders_mode_t = 20;
pub const HEBM_VERTICAL: hide_edge_borders_mode_t = 3;
pub const HEBM_NONE: hide_edge_borders_mode_t = 0;

pub type tiling_drag_t = ::core::ffi::c_uint;
pub const TILING_DRAG_MODIFIER_OR_TITLEBAR: tiling_drag_t = 3;
pub const TILING_DRAG_TITLEBAR: tiling_drag_t = 2;
pub const TILING_DRAG_MODIFIER: tiling_drag_t = 1;
pub const TILING_DRAG_OFF: tiling_drag_t = 0;

pub type ylength = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bfs_entry {
    pub con: *mut Con,
    pub entries: C2RustUnnamed_24,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_24 {
    pub tqe_next: *mut bfs_entry,
    pub tqe_prev: *mut *mut bfs_entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bfs_head {
    pub tqh_first: *mut bfs_entry,
    pub tqh_last: *mut *mut bfs_entry,
}
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const I3_IPC_EVENT_MASK: ::core::ffi::c_ulong =
    (1 as ::core::ffi::c_ulong) << 31 as ::core::ffi::c_int;
pub const I3_IPC_EVENT_WORKSPACE: ::core::ffi::c_ulong =
    I3_IPC_EVENT_MASK | 0 as ::core::ffi::c_ulong;

pub unsafe fn con_force_split_parents_redraw(con: *mut Con) {
    let mut parent: *mut Con = con;
    while !parent.is_null()
        && (*parent).type_0 as ::core::ffi::c_uint
            != CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
        && (*parent).type_0 as ::core::ffi::c_uint
            != CT_DOCKAREA as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if !con_is_leaf(parent) {
            free((*parent).deco_render_params as *mut ::core::ffi::c_void);
            (*parent).deco_render_params = ::core::ptr::null_mut::<deco_render_params>();
        }
        parent = (*parent).parent as *mut Con;
    }
}

pub unsafe fn con_new_skeleton(parent: *mut Con, window: *mut i3Window) -> *mut Con {
    let new: *mut Con = scalloc(1 as usize, ::core::mem::size_of::<Con>() as usize) as *mut Con;
    (*new).on_remove_child =
        Some(con_on_remove_child as unsafe fn(*mut Con) -> ()) as Option<unsafe fn(*mut Con) -> ()>;
    (*new).all_cons.tqe_next = ::core::ptr::null_mut::<Con>();
    (*new).all_cons.tqe_prev = all_cons.tqh_last;
    *all_cons.tqh_last = new as *mut Con;
    all_cons.tqh_last = &raw mut (*new).all_cons.tqe_next;
    (*new).type_0 = CT_CON;
    (*new).window = window as *mut Window;
    (*new).max_user_border_style = config.default_border;
    (*new).border_style = (*new).max_user_border_style;
    (*new).current_border_width = -(1 as ::core::ffi::c_int);
    (*new).window_icon_padding = -(1 as ::core::ffi::c_int);
    if !window.is_null() {
        (*new).depth = (*window).depth;
    } else {
        (*new).depth = root_depth as u16;
    }
    debuglog(
        c"%s:%s:%d - opening window\n".as_ptr().cast_mut(),
        c"../src/con.c".as_ptr(),
        c"con_new_skeleton".as_ptr(),
        52 as ::core::ffi::c_int,
    );
    (*new).floating_head.tqh_first = ::core::ptr::null_mut::<Con>();
    (*new).floating_head.tqh_last = &raw mut (*new).floating_head.tqh_first;
    (*new).nodes_head.tqh_first = ::core::ptr::null_mut::<Con>();
    (*new).nodes_head.tqh_last = &raw mut (*new).nodes_head.tqh_first;
    (*new).focus_head.tqh_first = ::core::ptr::null_mut::<Con>();
    (*new).focus_head.tqh_last = &raw mut (*new).focus_head.tqh_first;
    (*new).swallow_head.tqh_first = ::core::ptr::null_mut::<Match>();
    (*new).swallow_head.tqh_last = &raw mut (*new).swallow_head.tqh_first;
    (*new).marks_head.tqh_first = ::core::ptr::null_mut::<mark_t>();
    (*new).marks_head.tqh_last = &raw mut (*new).marks_head.tqh_first;
    if !parent.is_null() {
        con_attach(new, parent, false_0 != 0);
    }
    new
}

pub unsafe fn con_new(parent: *mut Con, window: *mut i3Window) -> *mut Con {
    let new: *mut Con = con_new_skeleton(parent, window);
    x_con_init(new);
    new
}

pub unsafe fn con_free(con: *mut Con) {
    free((*con).name as *mut ::core::ffi::c_void);
    free((*con).deco_render_params as *mut ::core::ffi::c_void);
    (*con).deco_render_params = ::core::ptr::null_mut::<deco_render_params>();
    if !(*con).all_cons.tqe_next.is_null() {
        (*(*con).all_cons.tqe_next).all_cons.tqe_prev = (*con).all_cons.tqe_prev;
    } else {
        all_cons.tqh_last = (*con).all_cons.tqe_prev;
    }
    *(*con).all_cons.tqe_prev = (*con).all_cons.tqe_next;
    while !(*con).swallow_head.tqh_first.is_null() {
        let match_0: *mut Match = (*con).swallow_head.tqh_first as *mut Match;
        if !(*match_0).matches.tqe_next.is_null() {
            (*(*match_0).matches.tqe_next).matches.tqe_prev = (*match_0).matches.tqe_prev;
        } else {
            (*con).swallow_head.tqh_last = (*match_0).matches.tqe_prev;
        }
        *(*match_0).matches.tqe_prev = (*match_0).matches.tqe_next;
        match_free(match_0);
        free(match_0 as *mut ::core::ffi::c_void);
    }
    while !(*con).marks_head.tqh_first.is_null() {
        let mut mark: *mut mark_t = (*con).marks_head.tqh_first as *mut mark_t;
        if !(*mark).marks.tqe_next.is_null() {
            (*(*mark).marks.tqe_next).marks.tqe_prev = (*mark).marks.tqe_prev;
        } else {
            (*con).marks_head.tqh_last = (*mark).marks.tqe_prev;
        }
        *(*mark).marks.tqe_prev = (*mark).marks.tqe_next;
        free((*mark).name as *mut ::core::ffi::c_void);
        (*mark).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
        free(mark as *mut ::core::ffi::c_void);
        mark = ::core::ptr::null_mut::<mark_t>();
    }
    debuglog(
        c"%s:%s:%d - con %p freed\n".as_ptr().cast_mut(),
        c"../src/con.c".as_ptr(),
        c"con_free".as_ptr(),
        96 as ::core::ffi::c_int,
        con,
    );
    free(con as *mut ::core::ffi::c_void);
}
unsafe fn _con_attach(con: *mut Con, parent: *mut Con, previous: *mut Con, ignore_focus: bool) {
    (*con).parent = parent as *mut Con;
    let mut loop_0: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut current: *mut Con = previous;
    let mut nodes_head: *mut nodes_head = &raw mut (*parent).nodes_head;
    let mut focus_head: *mut focus_head = &raw mut (*parent).focus_head;
    if (*con).type_0 as ::core::ffi::c_uint
        == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        debuglog(
            c"%s:%s:%d - it's a workspace. num = %d\n"
                .as_ptr()
                .cast_mut(),
            c"../src/con.c".as_ptr(),
            c"_con_attach".as_ptr(),
            110 as ::core::ffi::c_int,
            (*con).num,
        );
        if (*con).num == -(1 as ::core::ffi::c_int) || (*nodes_head).tqh_first.is_null() {
            (*con).nodes.tqe_next = ::core::ptr::null_mut::<Con>();
            (*con).nodes.tqe_prev = (*nodes_head).tqh_last;
            *(*nodes_head).tqh_last = con as *mut Con;
            (*nodes_head).tqh_last = &raw mut (*con).nodes.tqe_next;
        } else {
            current = (*nodes_head).tqh_first as *mut Con;
            if (*con).num < (*current).num {
                (*con).nodes.tqe_next = (*nodes_head).tqh_first;
                if !(*con).nodes.tqe_next.is_null() {
                    (*(*nodes_head).tqh_first).nodes.tqe_prev = &raw mut (*con).nodes.tqe_next;
                } else {
                    (*nodes_head).tqh_last = &raw mut (*con).nodes.tqe_next;
                }
                (*nodes_head).tqh_first = con as *mut Con;
                (*con).nodes.tqe_prev = &raw mut (*nodes_head).tqh_first;
            } else {
                while (*current).num != -(1 as ::core::ffi::c_int) && (*con).num >= (*current).num {
                    current = (*current).nodes.tqe_next as *mut Con;
                    if !current.is_null() {
                        continue;
                    }
                    current = ::core::ptr::null_mut::<Con>();
                    break;
                }
                if !current.is_null() {
                    (*con).nodes.tqe_prev = (*current).nodes.tqe_prev;
                    (*con).nodes.tqe_next = current as *mut Con;
                    *(*current).nodes.tqe_prev = con as *mut Con;
                    (*current).nodes.tqe_prev = &raw mut (*con).nodes.tqe_next;
                } else {
                    (*con).nodes.tqe_next = ::core::ptr::null_mut::<Con>();
                    (*con).nodes.tqe_prev = (*nodes_head).tqh_last;
                    *(*nodes_head).tqh_last = con as *mut Con;
                    (*nodes_head).tqh_last = &raw mut (*con).nodes.tqe_next;
                }
            }
        }
    } else if (*parent).type_0 as ::core::ffi::c_uint
        == CT_DOCKAREA as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        current = ::core::ptr::null_mut::<Con>();
        loop_0 = (*nodes_head).tqh_first as *mut Con;
        while !loop_0.is_null() {
            let mut result: ::core::ffi::c_int = strcasecmp_nullable(
                (*(*con).window).class_class,
                (*(*loop_0).window).class_class,
            );
            if result == 0 as ::core::ffi::c_int {
                result = strcasecmp_nullable(
                    (*(*con).window).class_instance,
                    (*(*loop_0).window).class_instance,
                );
            }
            if result < 0 as ::core::ffi::c_int {
                current = loop_0;
                break;
            } else {
                loop_0 = (*loop_0).nodes.tqe_next as *mut Con;
            }
        }
        if !current.is_null() {
            (*con).nodes.tqe_prev = (*loop_0).nodes.tqe_prev;
            (*con).nodes.tqe_next = loop_0 as *mut Con;
            *(*loop_0).nodes.tqe_prev = con as *mut Con;
            (*loop_0).nodes.tqe_prev = &raw mut (*con).nodes.tqe_next;
        } else {
            (*con).nodes.tqe_next = ::core::ptr::null_mut::<Con>();
            (*con).nodes.tqe_prev = (*nodes_head).tqh_last;
            *(*nodes_head).tqh_last = con as *mut Con;
            (*nodes_head).tqh_last = &raw mut (*con).nodes.tqe_next;
        }
    } else if (*con).type_0 as ::core::ffi::c_uint
        == CT_FLOATING_CON as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        debuglog(
            c"%s:%s:%d - Inserting into floating containers\n"
                .as_ptr()
                .cast_mut(),
            c"../src/con.c".as_ptr(),
            c"_con_attach".as_ptr(),
            162 as ::core::ffi::c_int,
        );
        (*con).floating_windows.tqe_next = ::core::ptr::null_mut::<Con>();
        (*con).floating_windows.tqe_prev = (*parent).floating_head.tqh_last;
        *(*parent).floating_head.tqh_last = con as *mut Con;
        (*parent).floating_head.tqh_last = &raw mut (*con).floating_windows.tqe_next;
    } else {
        if !ignore_focus {
            loop_0 = (*parent).focus_head.tqh_first as *mut Con;
            while !loop_0.is_null() {
                if (*loop_0).type_0 as ::core::ffi::c_uint
                    == CT_FLOATING_CON as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    loop_0 = (*loop_0).focused.tqe_next as *mut Con;
                } else {
                    current = loop_0;
                    break;
                }
            }
        }
        if !(*con).window.is_null()
            && (*parent).type_0 as ::core::ffi::c_uint
                == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
            && (*parent).workspace_layout != Layout::Default
        {
            debuglog(
                c"%s:%s:%d - Parent is a workspace. Applying default layout...\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/con.c".as_ptr(),
                c"_con_attach".as_ptr(),
                186 as ::core::ffi::c_int,
            );
            let target: *mut Con = workspace_attach_to(parent);
            nodes_head = &raw mut (*target).nodes_head as *mut nodes_head;
            focus_head = &raw mut (*target).focus_head as *mut focus_head;
            (*con).parent = target as *mut Con;
            current = ::core::ptr::null_mut::<Con>();
            debuglog(
                c"%s:%s:%d - done\n".as_ptr().cast_mut(),
                c"../src/con.c".as_ptr(),
                c"_con_attach".as_ptr(),
                195 as ::core::ffi::c_int,
            );
        }
        if !current.is_null()
            && (*parent).type_0 as ::core::ffi::c_uint
                != CT_OUTPUT as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            debuglog(
                c"%s:%s:%d - Inserting con = %p after con %p\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/con.c".as_ptr(),
                c"_con_attach".as_ptr(),
                201 as ::core::ffi::c_int,
                con,
                current,
            );
            (*con).nodes.tqe_next = (*current).nodes.tqe_next;
            if !(*con).nodes.tqe_next.is_null() {
                (*(*con).nodes.tqe_next).nodes.tqe_prev = &raw mut (*con).nodes.tqe_next;
            } else {
                (*nodes_head).tqh_last = &raw mut (*con).nodes.tqe_next;
            }
            (*current).nodes.tqe_next = con as *mut Con;
            (*con).nodes.tqe_prev = &raw mut (*current).nodes.tqe_next;
        } else {
            (*con).nodes.tqe_next = ::core::ptr::null_mut::<Con>();
            (*con).nodes.tqe_prev = (*nodes_head).tqh_last;
            *(*nodes_head).tqh_last = con as *mut Con;
            (*nodes_head).tqh_last = &raw mut (*con).nodes.tqe_next;
        }
    }
    (*con).focused.tqe_next = ::core::ptr::null_mut::<Con>();
    (*con).focused.tqe_prev = (*focus_head).tqh_last;
    *(*focus_head).tqh_last = con as *mut Con;
    (*focus_head).tqh_last = &raw mut (*con).focused.tqe_next;
    con_force_split_parents_redraw(con);
}

pub unsafe fn con_attach(con: *mut Con, parent: *mut Con, ignore_focus: bool) {
    _con_attach(con, parent, ::core::ptr::null_mut::<Con>(), ignore_focus);
}

pub unsafe fn con_detach(con: *mut Con) {
    con_force_split_parents_redraw(con);
    if (*con).type_0 as ::core::ffi::c_uint
        == CT_FLOATING_CON as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if !(*con).floating_windows.tqe_next.is_null() {
            (*(*con).floating_windows.tqe_next)
                .floating_windows
                .tqe_prev = (*con).floating_windows.tqe_prev;
        } else {
            (*(*con).parent).floating_head.tqh_last = (*con).floating_windows.tqe_prev;
        }
        *(*con).floating_windows.tqe_prev = (*con).floating_windows.tqe_next;
        if !(*con).focused.tqe_next.is_null() {
            (*(*con).focused.tqe_next).focused.tqe_prev = (*con).focused.tqe_prev;
        } else {
            (*(*con).parent).focus_head.tqh_last = (*con).focused.tqe_prev;
        }
        *(*con).focused.tqe_prev = (*con).focused.tqe_next;
    } else {
        if !(*con).nodes.tqe_next.is_null() {
            (*(*con).nodes.tqe_next).nodes.tqe_prev = (*con).nodes.tqe_prev;
        } else {
            (*(*con).parent).nodes_head.tqh_last = (*con).nodes.tqe_prev;
        }
        *(*con).nodes.tqe_prev = (*con).nodes.tqe_next;
        if !(*con).focused.tqe_next.is_null() {
            (*(*con).focused.tqe_next).focused.tqe_prev = (*con).focused.tqe_prev;
        } else {
            (*(*con).parent).focus_head.tqh_last = (*con).focused.tqe_prev;
        }
        *(*con).focused.tqe_prev = (*con).focused.tqe_next;
    };
}

pub unsafe fn con_focus(con: *mut Con) {
    if con != ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut Con {
    } else {
        __assert_fail(
            c"con != NULL".as_ptr(),
            c"../src/con.c".as_ptr(),
            251 as ::core::ffi::c_uint,
            c"void con_focus(Con *)".as_ptr(),
        );
    };
    debuglog(
        c"%s:%s:%d - con_focus = %p\n".as_ptr().cast_mut(),
        c"../src/con.c".as_ptr(),
        c"con_focus".as_ptr(),
        252 as ::core::ffi::c_int,
        con,
    );
    if !(*con).focused.tqe_next.is_null() {
        (*(*con).focused.tqe_next).focused.tqe_prev = (*con).focused.tqe_prev;
    } else {
        (*(*con).parent).focus_head.tqh_last = (*con).focused.tqe_prev;
    }
    *(*con).focused.tqe_prev = (*con).focused.tqe_next;
    (*con).focused.tqe_next = (*(*con).parent).focus_head.tqh_first;
    if !(*con).focused.tqe_next.is_null() {
        (*(*(*con).parent).focus_head.tqh_first).focused.tqe_prev =
            &raw mut (*con).focused.tqe_next;
    } else {
        (*(*con).parent).focus_head.tqh_last = &raw mut (*con).focused.tqe_next;
    }
    (*(*con).parent).focus_head.tqh_first = con as *mut Con;
    (*con).focused.tqe_prev = &raw mut (*(*con).parent).focus_head.tqh_first;
    if !(*(*con).parent).parent.is_null() {
        con_focus((*con).parent as *mut Con);
    }
    focused = con;
    if (*con).urgent as ::core::ffi::c_int != 0 && con_is_leaf(con) as ::core::ffi::c_int != 0 {
        con_set_urgency(con, false_0 != 0);
        con_update_parents_urgency(con);
        workspace_update_urgent_flag(con_get_workspace(con));
        ipc_send_window_event(c"urgent".as_ptr(), con);
    }
}
unsafe fn con_raise(con: *mut Con) {
    let floating: *mut Con = con_inside_floating(con);
    if !floating.is_null() {
        floating_raise_con(floating);
    }
}

pub unsafe fn con_activate(con: *mut Con) {
    con_focus(con);
    con_raise(con);
}

pub unsafe fn con_activate_unblock(con: *mut Con) {
    let ws: *mut Con = con_get_workspace(con);
    let previous_focus: *mut Con = focused;
    let fullscreen_on_ws: *mut Con = con_get_fullscreen_covering_ws(ws);
    if !fullscreen_on_ws.is_null()
        && fullscreen_on_ws != con
        && !con_has_parent(con, fullscreen_on_ws)
    {
        con_disable_fullscreen(fullscreen_on_ws);
    }
    con_activate(con);
    if ws != con_get_workspace(previous_focus) {
        con_activate(previous_focus);
        workspace_show(ws);
        con_activate(con);
    }
}

pub unsafe fn con_close(con: *mut Con, kill_window: KillWindow) {
    if con != ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut Con {
    } else {
        __assert_fail(
            c"con != NULL".as_ptr(),
            c"../src/con.c".as_ptr(),
            337 as ::core::ffi::c_uint,
            c"void con_close(Con *, kill_window_t)".as_ptr(),
        );
    };
    debuglog(
        c"%s:%s:%d - Closing con = %p.\n".as_ptr().cast_mut(),
        c"../src/con.c".as_ptr(),
        c"con_close".as_ptr(),
        338 as ::core::ffi::c_int,
        con,
    );
    if (*con).type_0 as ::core::ffi::c_uint
        == CT_OUTPUT as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*con).type_0 as ::core::ffi::c_uint
            == CT_ROOT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        debuglog(
            c"%s:%s:%d - con = %p is of type %d, not closing anything.\n"
                .as_ptr()
                .cast_mut(),
            c"../src/con.c".as_ptr(),
            c"con_close".as_ptr(),
            342 as ::core::ffi::c_int,
            con,
            (*con).type_0 as ::core::ffi::c_uint,
        );
        return;
    }
    if (*con).type_0 as ::core::ffi::c_uint
        == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        debuglog(
            c"%s:%s:%d - con = %p is a workspace, closing all children instead.\n"
                .as_ptr()
                .cast_mut(),
            c"../src/con.c".as_ptr(),
            c"con_close".as_ptr(),
            347 as ::core::ffi::c_int,
            con,
        );
        let mut child: *mut Con = (*con).focus_head.tqh_first as *mut Con;
        while !child.is_null() {
            let next_child: *mut Con = (*child).focused.tqe_next as *mut Con;
            debuglog(
                c"%s:%s:%d - killing child = %p.\n".as_ptr().cast_mut(),
                c"../src/con.c".as_ptr(),
                c"con_close".as_ptr(),
                350 as ::core::ffi::c_int,
                child,
            );
            tree_close_internal(child, kill_window, false_0 != 0);
            child = next_child;
        }
        return;
    }
    tree_close_internal(con, kill_window, false_0 != 0);
}

pub unsafe fn con_is_leaf(con: *mut Con) -> bool {
    (*con).nodes_head.tqh_first.is_null()
}

pub unsafe fn con_has_managed_window(con: *mut Con) -> bool {
    !con.is_null()
        && !(*con).window.is_null()
        && (*(*con).window).id != XCB_WINDOW_NONE as ::core::ffi::c_int as xcb_window_t
        && !con_get_workspace(con).is_null()
}

pub unsafe fn con_has_children(con: *mut Con) -> bool {
    !con_is_leaf(con) || !(*con).floating_head.tqh_first.is_null()
}

pub unsafe fn con_is_split(con: *mut Con) -> bool {
    if con_is_leaf(con) {
        return false_0 != 0;
    }
    match (*con).layout {
        Layout::DockArea | Layout::Output => false_0 != 0,
        Layout::Default | Layout::Stacked | Layout::Tabbed | Layout::SplitV | Layout::SplitH => {
            true_0 != 0
        }
    }
}

pub unsafe fn con_is_hidden(con: *mut Con) -> bool {
    let mut current: *mut Con = con;
    while !current.is_null()
        && (*current).type_0 as ::core::ffi::c_uint
            != CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let parent: *mut Con = (*current).parent as *mut Con;
        if !parent.is_null()
            && ((*parent).layout == Layout::Tabbed || (*parent).layout == Layout::Stacked)
        {
            if (*parent).focus_head.tqh_first != current {
                return true_0 != 0;
            }
        }
        current = parent;
    }
    false_0 != 0
}

pub unsafe fn con_is_maximized(con: *mut Con, orientation: Orientation) -> bool {
    if (*con).fullscreen_mode as ::core::ffi::c_uint
        != CF_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return true_0 != 0;
    }
    let mut layout: Layout = Layout::Default;
    match orientation {
        Orientation::Horizontal => {
            layout = Layout::SplitH;
        }
        Orientation::Vertical => {
            layout = Layout::SplitV;
        }
        Orientation::None => {
            if 0 as ::core::ffi::c_int != 0 {
            } else {
                __assert_fail(
                    c"false".as_ptr(),
                    c"../src/con.c".as_ptr(),
                    458 as ::core::ffi::c_uint,
                    c"_Bool con_is_maximized(Con *, orientation_t)".as_ptr(),
                );
            };
        }
    }
    let mut current: *mut Con = con;
    loop {
        let parent: *mut Con = (*current).parent as *mut Con;
        if parent.is_null()
            || (*current).type_0 as ::core::ffi::c_uint
                == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return true_0 != 0;
        }
        if (*parent).layout == layout && con_num_children(parent) > 1 as ::core::ffi::c_int {
            return false_0 != 0;
        }
        if (*parent).type_0 as ::core::ffi::c_uint
            == CT_FLOATING_CON as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return false_0 != 0;
        }
        current = parent;
    }
}

pub unsafe fn con_is_sticky(con: *mut Con) -> bool {
    if (*con).sticky {
        return true_0 != 0;
    }
    let mut child: *mut Con = ::core::ptr::null_mut::<Con>();
    child = (*con).nodes_head.tqh_first as *mut Con;
    while !child.is_null() {
        if con_is_sticky(child) {
            return true_0 != 0;
        }
        child = (*child).nodes.tqe_next as *mut Con;
    }
    false_0 != 0
}

pub unsafe fn con_accepts_window(con: *mut Con) -> bool {
    if (*con).type_0 as ::core::ffi::c_uint
        == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return false_0 != 0;
    }
    if con_is_split(con) {
        debuglog(
            c"%s:%s:%d - container %p does not accept windows, it is a split container.\n"
                .as_ptr()
                .cast_mut(),
            c"../src/con.c".as_ptr(),
            c"con_accepts_window".as_ptr(),
            518 as ::core::ffi::c_int,
            con,
        );
        return false_0 != 0;
    }
    (*con).window.is_null()
}

pub unsafe fn con_get_output(con: *mut Con) -> *mut Con {
    let mut result: *mut Con = con;
    while !result.is_null()
        && (*result).type_0 as ::core::ffi::c_uint
            != CT_OUTPUT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        result = (*result).parent as *mut Con;
    }
    if result != ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut Con {
    } else {
        __assert_fail(
            c"result != NULL".as_ptr(),
            c"../src/con.c".as_ptr(),
            538 as ::core::ffi::c_uint,
            c"Con *con_get_output(Con *)".as_ptr(),
        );
    };
    result
}

pub unsafe fn con_get_workspace(con: *mut Con) -> *mut Con {
    let mut result: *mut Con = con;
    while !result.is_null()
        && (*result).type_0 as ::core::ffi::c_uint
            != CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        result = (*result).parent as *mut Con;
    }
    result
}

pub unsafe fn con_parent_with_orientation(con: *mut Con, orientation: Orientation) -> *mut Con {
    debuglog(
        c"%s:%s:%d - Searching for parent of Con %p with orientation %d\n"
            .as_ptr()
            .cast_mut(),
        c"../src/con.c".as_ptr(),
        c"con_parent_with_orientation".as_ptr(),
        560 as ::core::ffi::c_int,
        con,
        orientation as ::core::ffi::c_uint,
    );
    let mut parent: *mut Con = (*con).parent as *mut Con;
    if (*parent).type_0 as ::core::ffi::c_uint
        == CT_FLOATING_CON as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return ::core::ptr::null_mut::<Con>();
    }
    while con_orientation(parent) != orientation {
        debuglog(
            c"%s:%s:%d - Need to go one level further up\n"
                .as_ptr()
                .cast_mut(),
            c"../src/con.c".as_ptr(),
            c"con_parent_with_orientation".as_ptr(),
            566 as ::core::ffi::c_int,
        );
        parent = (*parent).parent as *mut Con;
        if !parent.is_null()
            && ((*parent).type_0 as ::core::ffi::c_uint
                == CT_FLOATING_CON as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*parent).type_0 as ::core::ffi::c_uint
                    == CT_OUTPUT as ::core::ffi::c_int as ::core::ffi::c_uint
                || !(*parent).parent.is_null()
                    && (*(*parent).parent).type_0 as ::core::ffi::c_uint
                        == CT_OUTPUT as ::core::ffi::c_int as ::core::ffi::c_uint)
        {
            parent = ::core::ptr::null_mut::<Con>();
        }
        if parent.is_null() {
            break;
        }
    }
    debuglog(
        c"%s:%s:%d - Result: %p\n".as_ptr().cast_mut(),
        c"../src/con.c".as_ptr(),
        c"con_parent_with_orientation".as_ptr(),
        579 as ::core::ffi::c_int,
        parent,
    );
    parent
}

pub unsafe fn con_get_fullscreen_con(
    con: *mut Con,
    fullscreen_mode: fullscreen_mode_t,
) -> *mut Con {
    let mut bfs_head_0: bfs_head = bfs_head {
        tqh_first: ::core::ptr::null_mut::<bfs_entry>(),
        tqh_last: ::core::ptr::null_mut::<*mut bfs_entry>(),
    };
    bfs_head_0 = bfs_head {
        tqh_first: ::core::ptr::null_mut::<bfs_entry>(),
        tqh_last: &raw mut bfs_head_0.tqh_first,
    };
    let mut entry: *mut bfs_entry =
        smalloc(::core::mem::size_of::<bfs_entry>() as usize) as *mut bfs_entry;
    (*entry).con = con;
    (*entry).entries.tqe_next = ::core::ptr::null_mut::<bfs_entry>();
    (*entry).entries.tqe_prev = bfs_head_0.tqh_last;
    *bfs_head_0.tqh_last = entry;
    bfs_head_0.tqh_last = &raw mut (*entry).entries.tqe_next;
    while !bfs_head_0.tqh_first.is_null() {
        entry = bfs_head_0.tqh_first;
        let current: *mut Con = (*entry).con;
        if current != con
            && (*current).fullscreen_mode as ::core::ffi::c_uint
                == fullscreen_mode as ::core::ffi::c_uint
        {
            while !bfs_head_0.tqh_first.is_null() {
                entry = bfs_head_0.tqh_first;
                if !(*entry).entries.tqe_next.is_null() {
                    (*(*entry).entries.tqe_next).entries.tqe_prev = (*entry).entries.tqe_prev;
                } else {
                    bfs_head_0.tqh_last = (*entry).entries.tqe_prev;
                }
                *(*entry).entries.tqe_prev = (*entry).entries.tqe_next;
                free(entry as *mut ::core::ffi::c_void);
            }
            return current;
        }
        if !(*entry).entries.tqe_next.is_null() {
            (*(*entry).entries.tqe_next).entries.tqe_prev = (*entry).entries.tqe_prev;
        } else {
            bfs_head_0.tqh_last = (*entry).entries.tqe_prev;
        }
        *(*entry).entries.tqe_prev = (*entry).entries.tqe_next;
        free(entry as *mut ::core::ffi::c_void);
        let mut child: *mut Con = ::core::ptr::null_mut::<Con>();
        child = (*current).nodes_head.tqh_first as *mut Con;
        while !child.is_null() {
            entry = smalloc(::core::mem::size_of::<bfs_entry>() as usize) as *mut bfs_entry;
            (*entry).con = child;
            (*entry).entries.tqe_next = ::core::ptr::null_mut::<bfs_entry>();
            (*entry).entries.tqe_prev = bfs_head_0.tqh_last;
            *bfs_head_0.tqh_last = entry;
            bfs_head_0.tqh_last = &raw mut (*entry).entries.tqe_next;
            child = (*child).nodes.tqe_next as *mut Con;
        }
        child = (*current).floating_head.tqh_first as *mut Con;
        while !child.is_null() {
            entry = smalloc(::core::mem::size_of::<bfs_entry>() as usize) as *mut bfs_entry;
            (*entry).con = child;
            (*entry).entries.tqe_next = ::core::ptr::null_mut::<bfs_entry>();
            (*entry).entries.tqe_prev = bfs_head_0.tqh_last;
            *bfs_head_0.tqh_last = entry;
            bfs_head_0.tqh_last = &raw mut (*entry).entries.tqe_next;
            child = (*child).floating_windows.tqe_next as *mut Con;
        }
    }
    ::core::ptr::null_mut::<Con>()
}

pub unsafe fn con_get_fullscreen_covering_ws(ws: *mut Con) -> *mut Con {
    if ws.is_null() {
        return ::core::ptr::null_mut::<Con>();
    }
    let fs: *mut Con = con_get_fullscreen_con(croot, CF_GLOBAL);
    if fs.is_null() {
        return con_get_fullscreen_con(ws, CF_OUTPUT);
    }
    fs
}

pub unsafe fn con_is_internal(con: *mut Con) -> bool {
    *(*con).name.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '_' as i32
        && *(*con).name.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '_' as i32
}

pub unsafe fn con_is_floating(con: *mut Con) -> bool {
    if con != ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut Con {
    } else {
        __assert_fail(
            c"con != NULL".as_ptr(),
            c"../src/con.c".as_ptr(),
            669 as ::core::ffi::c_uint,
            c"_Bool con_is_floating(Con *)".as_ptr(),
        );
    };
    ((*con).floating).is_floating()
}

pub unsafe fn con_is_docked(con: *mut Con) -> bool {
    if (*con).parent.is_null() {
        return false_0 != 0;
    }
    if (*(*con).parent).type_0 as ::core::ffi::c_uint
        == CT_DOCKAREA as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return true_0 != 0;
    }
    con_is_docked((*con).parent as *mut Con)
}

pub unsafe fn con_inside_floating(con: *mut Con) -> *mut Con {
    if con.is_null() {
        return ::core::ptr::null_mut::<Con>();
    }
    if (*con).type_0 as ::core::ffi::c_uint
        == CT_FLOATING_CON as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return con;
    }
    if ((*con).floating).is_floating() {
        return (*con).parent as *mut Con;
    }
    if (*con).type_0 as ::core::ffi::c_uint
        == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*con).type_0 as ::core::ffi::c_uint
            == CT_OUTPUT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return ::core::ptr::null_mut::<Con>();
    }
    con_inside_floating((*con).parent as *mut Con)
}

pub unsafe fn con_inside_focused(con: *mut Con) -> bool {
    if con == focused {
        return true_0 != 0;
    }
    if (*con).parent.is_null() {
        return false_0 != 0;
    }
    con_inside_focused((*con).parent as *mut Con)
}

pub unsafe fn con_has_parent(con: *mut Con, parent: *mut Con) -> bool {
    let current: *mut Con = (*con).parent as *mut Con;
    if current.is_null() {
        return false_0 != 0;
    }
    if current == parent {
        return true_0 != 0;
    }
    con_has_parent(current, parent)
}

pub unsafe fn con_by_window_id(window: xcb_window_t) -> *mut Con {
    let mut con: *mut Con = ::core::ptr::null_mut::<Con>();
    con = all_cons.tqh_first as *mut Con;
    while !con.is_null() {
        if !(*con).window.is_null() && (*(*con).window).id == window {
            return con;
        }
        con = (*con).all_cons.tqe_next as *mut Con;
    }
    ::core::ptr::null_mut::<Con>()
}

pub unsafe fn con_by_con_id(target: ::core::ffi::c_long) -> *mut Con {
    let mut con: *mut Con = ::core::ptr::null_mut::<Con>();
    con = all_cons.tqh_first as *mut Con;
    while !con.is_null() {
        if con == target as *mut Con {
            return con;
        }
        con = (*con).all_cons.tqe_next as *mut Con;
    }
    ::core::ptr::null_mut::<Con>()
}

pub unsafe fn con_exists(con: *mut Con) -> bool {
    !con_by_con_id(con as ::core::ffi::c_long).is_null()
}

pub unsafe fn con_by_frame_id(frame: xcb_window_t) -> *mut Con {
    let mut con: *mut Con = ::core::ptr::null_mut::<Con>();
    con = all_cons.tqh_first as *mut Con;
    while !con.is_null() {
        if (*con).frame.id == frame {
            return con;
        }
        con = (*con).all_cons.tqe_next as *mut Con;
    }
    ::core::ptr::null_mut::<Con>()
}

pub unsafe fn con_by_mark(mark: *const ::core::ffi::c_char) -> *mut Con {
    let mut con: *mut Con = ::core::ptr::null_mut::<Con>();
    con = all_cons.tqh_first as *mut Con;
    while !con.is_null() {
        if con_has_mark(con, mark) {
            return con;
        }
        con = (*con).all_cons.tqe_next as *mut Con;
    }
    ::core::ptr::null_mut::<Con>()
}

pub unsafe fn con_find_transient_for_window(start: *mut Con, target: xcb_window_t) -> bool {
    let mut transient_con: *mut Con = start;
    let mut count: ::core::ffi::c_int = con_num_windows(croot);
    while !transient_con.is_null()
        && !(*transient_con).window.is_null()
        && (*(*transient_con).window).transient_for as ::core::ffi::c_long != XCB_NONE
    {
        debuglog(
            c"%s:%s:%d - transient_con = 0x%08x, transient_con->window->transient_for = 0x%08x, target = 0x%08x\n".as_ptr().cast_mut(),
            c"../src/con.c".as_ptr(),
            c"con_find_transient_for_window".as_ptr(),
            829 as ::core::ffi::c_int,
            (*(*transient_con).window).id,
            (*(*transient_con).window).transient_for,
            target,
        );
        if (*(*transient_con).window).transient_for == target {
            return true_0 != 0;
        }
        let next_transient: *mut Con = con_by_window_id((*(*transient_con).window).transient_for);
        if next_transient.is_null() {
            break;
        }
        if transient_con == next_transient {
            break;
        }
        transient_con = next_transient;
        let fresh0 = count;
        count = count - 1;
        if fresh0 <= 0 as ::core::ffi::c_int {
            break;
        }
    }
    false_0 != 0
}

pub unsafe fn con_has_mark(con: *mut Con, mark: *const ::core::ffi::c_char) -> bool {
    let mut current: *mut mark_t = ::core::ptr::null_mut::<mark_t>();
    current = (*con).marks_head.tqh_first as *mut mark_t;
    while !current.is_null() {
        if strcmp((*current).name, mark) == 0 as ::core::ffi::c_int {
            return true_0 != 0;
        }
        current = (*current).marks.tqe_next as *mut mark_t;
    }
    false_0 != 0
}

pub unsafe fn con_mark_toggle(con: *mut Con, mark: *const ::core::ffi::c_char, mode: MarkMode) {
    if con != ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut Con {
    } else {
        __assert_fail(
            c"con != NULL".as_ptr(),
            c"../src/con.c".as_ptr(),
            873 as ::core::ffi::c_uint,
            c"void con_mark_toggle(Con *, const char *, mark_mode_t)".as_ptr(),
        );
    };
    debuglog(
        c"%s:%s:%d - Toggling mark \"%s\" on con = %p.\n"
            .as_ptr()
            .cast_mut(),
        c"../src/con.c".as_ptr(),
        c"con_mark_toggle".as_ptr(),
        874 as ::core::ffi::c_int,
        mark,
        con,
    );
    if con_has_mark(con, mark) {
        con_unmark(con, mark);
    } else {
        con_mark(con, mark, mode);
    };
}

pub unsafe fn con_mark(con: *mut Con, mark: *const ::core::ffi::c_char, mode: MarkMode) {
    if con != ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut Con {
    } else {
        __assert_fail(
            c"con != NULL".as_ptr(),
            c"../src/con.c".as_ptr(),
            888 as ::core::ffi::c_uint,
            c"void con_mark(Con *, const char *, mark_mode_t)".as_ptr(),
        );
    };
    debuglog(
        c"%s:%s:%d - Setting mark \"%s\" on con = %p.\n"
            .as_ptr()
            .cast_mut(),
        c"../src/con.c".as_ptr(),
        c"con_mark".as_ptr(),
        889 as ::core::ffi::c_int,
        mark,
        con,
    );
    con_unmark(::core::ptr::null_mut::<Con>(), mark);
    if mode == MarkMode::Replace {
        debuglog(
            c"%s:%s:%d - Removing all existing marks on con = %p.\n"
                .as_ptr()
                .cast_mut(),
            c"../src/con.c".as_ptr(),
            c"con_mark".as_ptr(),
            893 as ::core::ffi::c_int,
            con,
        );
        while !(*con).marks_head.tqh_first.is_null() {
            let current: *const mark_t = (*con).marks_head.tqh_first;
            con_unmark(con, (*current).name);
        }
    }
    let new: *mut mark_t =
        scalloc(1 as usize, ::core::mem::size_of::<mark_t>() as usize) as *mut mark_t;
    (*new).name = sstrdup(mark);
    (*new).marks.tqe_next = ::core::ptr::null_mut::<mark_t>();
    (*new).marks.tqe_prev = (*con).marks_head.tqh_last;
    *(*con).marks_head.tqh_last = new as *mut mark_t;
    (*con).marks_head.tqh_last = &raw mut (*new).marks.tqe_next;
    ipc_send_window_event(c"mark".as_ptr(), con);
    (*con).mark_changed = true_0 != 0;
}

pub unsafe fn con_unmark(con: *mut Con, name: *const ::core::ffi::c_char) {
    let mut current: *mut Con = ::core::ptr::null_mut::<Con>();
    if name.is_null() {
        debuglog(
            c"%s:%s:%d - Unmarking all containers.\n"
                .as_ptr()
                .cast_mut(),
            c"../src/con.c".as_ptr(),
            c"con_unmark".as_ptr(),
            919 as ::core::ffi::c_int,
        );
        current = all_cons.tqh_first as *mut Con;
        while !current.is_null() {
            if !(!con.is_null() && current != con) {
                if !(*current).marks_head.tqh_first.is_null() {
                    while !(*current).marks_head.tqh_first.is_null() {
                        let mut mark: *mut mark_t = (*current).marks_head.tqh_first as *mut mark_t;
                        free((*mark).name as *mut ::core::ffi::c_void);
                        (*mark).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
                        if !(*mark).marks.tqe_next.is_null() {
                            (*(*mark).marks.tqe_next).marks.tqe_prev = (*mark).marks.tqe_prev;
                        } else {
                            (*current).marks_head.tqh_last = (*mark).marks.tqe_prev;
                        }
                        *(*mark).marks.tqe_prev = (*mark).marks.tqe_next;
                        free(mark as *mut ::core::ffi::c_void);
                        mark = ::core::ptr::null_mut::<mark_t>();
                        ipc_send_window_event(c"mark".as_ptr(), current);
                    }
                    (*current).mark_changed = true_0 != 0;
                }
            }
            current = (*current).all_cons.tqe_next as *mut Con;
        }
    } else {
        debuglog(
            c"%s:%s:%d - Removing mark \"%s\".\n".as_ptr().cast_mut(),
            c"../src/con.c".as_ptr(),
            c"con_unmark".as_ptr(),
            941 as ::core::ffi::c_int,
            name,
        );
        current = if con.is_null() {
            con_by_mark(name)
        } else {
            con
        };
        if current.is_null() {
            debuglog(
                c"%s:%s:%d - No container found with this mark, so there is nothing to do.\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/con.c".as_ptr(),
                c"con_unmark".as_ptr(),
                944 as ::core::ffi::c_int,
            );
            return;
        }
        debuglog(
            c"%s:%s:%d - Found mark on con = %p. Removing it now.\n"
                .as_ptr()
                .cast_mut(),
            c"../src/con.c".as_ptr(),
            c"con_unmark".as_ptr(),
            948 as ::core::ffi::c_int,
            current,
        );
        (*current).mark_changed = true_0 != 0;
        let mut mark_0: *mut mark_t = ::core::ptr::null_mut::<mark_t>();
        mark_0 = (*current).marks_head.tqh_first as *mut mark_t;
        while !mark_0.is_null() {
            if strcmp((*mark_0).name, name) != 0 as ::core::ffi::c_int {
                mark_0 = (*mark_0).marks.tqe_next as *mut mark_t;
            } else {
                free((*mark_0).name as *mut ::core::ffi::c_void);
                (*mark_0).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
                if !(*mark_0).marks.tqe_next.is_null() {
                    (*(*mark_0).marks.tqe_next).marks.tqe_prev = (*mark_0).marks.tqe_prev;
                } else {
                    (*current).marks_head.tqh_last = (*mark_0).marks.tqe_prev;
                }
                *(*mark_0).marks.tqe_prev = (*mark_0).marks.tqe_next;
                free(mark_0 as *mut ::core::ffi::c_void);
                mark_0 = ::core::ptr::null_mut::<mark_t>();
                ipc_send_window_event(c"mark".as_ptr(), current);
                break;
            }
        }
    };
}

pub unsafe fn con_for_window(
    con: *mut Con,
    window: *mut i3Window,
    store_match: *mut *mut Match,
) -> *mut Con {
    let mut child: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut match_0: *mut Match = ::core::ptr::null_mut::<Match>();
    child = (*con).nodes_head.tqh_first as *mut Con;
    while !child.is_null() {
        match_0 = (*child).swallow_head.tqh_first as *mut Match;
        while !match_0.is_null() {
            if !match_matches_window(match_0, window) {
                match_0 = (*match_0).matches.tqe_next as *mut Match;
            } else {
                if !store_match.is_null() {
                    *store_match = match_0;
                }
                return child;
            }
        }
        let result: *mut Con = con_for_window(child, window, store_match);
        if !result.is_null() {
            return result;
        }
        child = (*child).nodes.tqe_next as *mut Con;
    }
    child = (*con).floating_head.tqh_first as *mut Con;
    while !child.is_null() {
        match_0 = (*child).swallow_head.tqh_first as *mut Match;
        while !match_0.is_null() {
            if !match_matches_window(match_0, window) {
                match_0 = (*match_0).matches.tqe_next as *mut Match;
            } else {
                if !store_match.is_null() {
                    *store_match = match_0;
                }
                return child;
            }
        }
        let result_0: *mut Con = con_for_window(child, window, store_match);
        if !result_0.is_null() {
            return result_0;
        }
        child = (*child).floating_windows.tqe_next as *mut Con;
    }
    ::core::ptr::null_mut::<Con>()
}
unsafe fn num_focus_heads(con: *mut Con) -> ::core::ffi::c_int {
    let mut focus_heads: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut current: *mut Con = ::core::ptr::null_mut::<Con>();
    current = (*con).focus_head.tqh_first as *mut Con;
    while !current.is_null() {
        focus_heads += 1;
        current = (*current).focused.tqe_next as *mut Con;
    }
    focus_heads
}

pub unsafe fn get_focus_order(con: *mut Con) -> *mut *mut Con {
    let focus_heads: ::core::ffi::c_int = num_focus_heads(con) as ::core::ffi::c_int;
    let focus_order: *mut *mut Con =
        smalloc((focus_heads as usize).wrapping_mul(::core::mem::size_of::<*mut Con>() as usize))
            as *mut *mut Con;
    let mut current: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut idx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    current = (*con).focus_head.tqh_first as *mut Con;
    while !current.is_null() {
        if idx < focus_heads {
        } else {
            __assert_fail(
                c"idx < focus_heads".as_ptr(),
                c"../src/con.c".as_ptr(),
                1033 as ::core::ffi::c_uint,
                c"Con **get_focus_order(Con *)".as_ptr(),
            );
        };
        let fresh1 = idx;
        idx = idx + 1;
        let ref mut fresh2 = *focus_order.offset(fresh1 as isize);
        *fresh2 = current;
        current = (*current).focused.tqe_next as *mut Con;
    }
    focus_order
}

pub unsafe fn set_focus_order(con: *mut Con, focus_order: *mut *mut Con) {
    let mut focus_heads: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while !(*con).focus_head.tqh_first.is_null() {
        let current: *mut Con = (*con).focus_head.tqh_first as *mut Con;
        if !(*current).focused.tqe_next.is_null() {
            (*(*current).focused.tqe_next).focused.tqe_prev = (*current).focused.tqe_prev;
        } else {
            (*con).focus_head.tqh_last = (*current).focused.tqe_prev;
        }
        *(*current).focused.tqe_prev = (*current).focused.tqe_next;
        focus_heads += 1;
    }
    let mut idx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while idx < focus_heads {
        if (*con).type_0 as ::core::ffi::c_uint
            != CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
            && !con_inside_floating(*focus_order.offset(idx as isize)).is_null()
        {
            focus_heads += 1;
        } else {
            let ref mut fresh3 = (**focus_order.offset(idx as isize)).focused.tqe_next;
            *fresh3 = ::core::ptr::null_mut::<Con>();
            let ref mut fresh4 = (**focus_order.offset(idx as isize)).focused.tqe_prev;
            *fresh4 = (*con).focus_head.tqh_last;
            *(*con).focus_head.tqh_last = *focus_order.offset(idx as isize) as *mut Con;
            (*con).focus_head.tqh_last =
                &raw mut (**focus_order.offset(idx as isize)).focused.tqe_next;
        }
        idx += 1;
    }
}

pub unsafe fn con_num_children(con: *mut Con) -> ::core::ffi::c_int {
    let mut child: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut children: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    child = (*con).nodes_head.tqh_first as *mut Con;
    while !child.is_null() {
        children += 1;
        child = (*child).nodes.tqe_next as *mut Con;
    }
    children
}

pub unsafe fn con_num_visible_children(con: *mut Con) -> ::core::ffi::c_int {
    if con.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    let mut children: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut current: *mut Con = ::core::ptr::null_mut::<Con>();
    current = (*con).nodes_head.tqh_first as *mut Con;
    while !current.is_null() {
        if !con_is_hidden(current) && con_is_leaf(current) as ::core::ffi::c_int != 0 {
            children += 1;
        } else {
            children += con_num_visible_children(current);
        }
        current = (*current).nodes.tqe_next as *mut Con;
    }
    children
}

pub unsafe fn con_num_windows(con: *mut Con) -> ::core::ffi::c_int {
    if con.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if con_has_managed_window(con) {
        return 1 as ::core::ffi::c_int;
    }
    let mut num: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut current: *mut Con = ::core::ptr::null_mut::<Con>();
    current = (*con).nodes_head.tqh_first as *mut Con;
    while !current.is_null() {
        num += con_num_windows(current);
        current = (*current).nodes.tqe_next as *mut Con;
    }
    current = (*con).floating_head.tqh_first as *mut Con;
    while !current.is_null() {
        num += con_num_windows(current);
        current = (*current).floating_windows.tqe_next as *mut Con;
    }
    num
}

pub unsafe fn con_fix_percent(con: *mut Con) {
    let mut child: *mut Con = ::core::ptr::null_mut::<Con>();
    let children: ::core::ffi::c_int = con_num_children(con);
    let mut total: ::core::ffi::c_double = 0.0f64;
    let mut children_with_percent: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    child = (*con).nodes_head.tqh_first as *mut Con;
    while !child.is_null() {
        if (*child).percent > 0.0f64 {
            total += (*child).percent;
            children_with_percent += 1;
        }
        child = (*child).nodes.tqe_next as *mut Con;
    }
    if children_with_percent != children {
        child = (*con).nodes_head.tqh_first as *mut Con;
        while !child.is_null() {
            if (*child).percent <= 0.0f64 {
                if children_with_percent == 0 as ::core::ffi::c_int {
                    (*child).percent = 1.0f64;
                    total += (*child).percent;
                } else {
                    (*child).percent = total / children_with_percent as ::core::ffi::c_double;
                    total += (*child).percent;
                }
            }
            child = (*child).nodes.tqe_next as *mut Con;
        }
    }
    if total == 0.0f64 {
        child = (*con).nodes_head.tqh_first as *mut Con;
        while !child.is_null() {
            (*child).percent = 1.0f64 / children as ::core::ffi::c_double;
            child = (*child).nodes.tqe_next as *mut Con;
        }
    } else if total != 1.0f64 {
        child = (*con).nodes_head.tqh_first as *mut Con;
        while !child.is_null() {
            (*child).percent /= total;
            child = (*child).nodes.tqe_next as *mut Con;
        }
    }
}

pub unsafe fn con_toggle_fullscreen(con: *mut Con, fullscreen_mode: ::core::ffi::c_int) {
    if (*con).type_0 as ::core::ffi::c_uint
        == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        debuglog(
            c"%s:%s:%d - You cannot make a workspace fullscreen.\n"
                .as_ptr()
                .cast_mut(),
            c"../src/con.c".as_ptr(),
            c"con_toggle_fullscreen".as_ptr(),
            1190 as ::core::ffi::c_int,
        );
        return;
    }
    debuglog(
        c"%s:%s:%d - toggling fullscreen for %p / %s\n"
            .as_ptr()
            .cast_mut(),
        c"../src/con.c".as_ptr(),
        c"con_toggle_fullscreen".as_ptr(),
        1194 as ::core::ffi::c_int,
        con,
        (*con).name,
    );
    if (*con).fullscreen_mode as ::core::ffi::c_uint
        == CF_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        con_enable_fullscreen(con, fullscreen_mode as fullscreen_mode_t);
    } else {
        con_disable_fullscreen(con);
    };
}
unsafe fn con_set_fullscreen_mode(con: *mut Con, fullscreen_mode: fullscreen_mode_t) {
    (*con).fullscreen_mode = fullscreen_mode;
    debuglog(
        c"%s:%s:%d - mode now: %d\n".as_ptr().cast_mut(),
        c"../src/con.c".as_ptr(),
        c"con_set_fullscreen_mode".as_ptr(),
        1212 as ::core::ffi::c_int,
        (*con).fullscreen_mode as ::core::ffi::c_uint,
    );
    ipc_send_window_event(c"fullscreen_mode".as_ptr(), con);
    if (*con).window.is_null() {
        return;
    }
    if (*con).fullscreen_mode as ::core::ffi::c_uint
        != CF_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        debuglog(
            c"%s:%s:%d - Setting _NET_WM_STATE_FULLSCREEN for con = %p / window = %d.\n"
                .as_ptr()
                .cast_mut(),
            c"../src/con.c".as_ptr(),
            c"con_set_fullscreen_mode".as_ptr(),
            1225 as ::core::ffi::c_int,
            con,
            (*(*con).window).id,
        );
        xcb_add_property_atom(
            conn,
            (*(*con).window).id,
            A__NET_WM_STATE,
            A__NET_WM_STATE_FULLSCREEN,
        );
    } else {
        debuglog(
            c"%s:%s:%d - Removing _NET_WM_STATE_FULLSCREEN for con = %p / window = %d.\n"
                .as_ptr()
                .cast_mut(),
            c"../src/con.c".as_ptr(),
            c"con_set_fullscreen_mode".as_ptr(),
            1228 as ::core::ffi::c_int,
            con,
            (*(*con).window).id,
        );
        xcb_remove_property_atom(
            conn,
            (*(*con).window).id,
            A__NET_WM_STATE,
            A__NET_WM_STATE_FULLSCREEN,
        );
    };
}

pub unsafe fn con_enable_fullscreen(con: *mut Con, fullscreen_mode: fullscreen_mode_t) {
    if (*con).type_0 as ::core::ffi::c_uint
        == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        debuglog(
            c"%s:%s:%d - You cannot make a workspace fullscreen.\n"
                .as_ptr()
                .cast_mut(),
            c"../src/con.c".as_ptr(),
            c"con_enable_fullscreen".as_ptr(),
            1246 as ::core::ffi::c_int,
        );
        return;
    }
    if fullscreen_mode as ::core::ffi::c_uint
        == CF_GLOBAL as ::core::ffi::c_int as ::core::ffi::c_uint
        || fullscreen_mode as ::core::ffi::c_uint
            == CF_OUTPUT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
    } else {
        __assert_fail(
            c"fullscreen_mode == CF_GLOBAL || fullscreen_mode == CF_OUTPUT".as_ptr(),
            c"../src/con.c".as_ptr(),
            1250 as ::core::ffi::c_uint,
            c"void con_enable_fullscreen(Con *, fullscreen_mode_t)".as_ptr(),
        );
    };
    if fullscreen_mode as ::core::ffi::c_uint
        == CF_GLOBAL as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        debuglog(
            c"%s:%s:%d - enabling global fullscreen for %p / %s\n"
                .as_ptr()
                .cast_mut(),
            c"../src/con.c".as_ptr(),
            c"con_enable_fullscreen".as_ptr(),
            1253 as ::core::ffi::c_int,
            con,
            (*con).name,
        );
    } else {
        debuglog(
            c"%s:%s:%d - enabling fullscreen for %p / %s\n"
                .as_ptr()
                .cast_mut(),
            c"../src/con.c".as_ptr(),
            c"con_enable_fullscreen".as_ptr(),
            1255 as ::core::ffi::c_int,
            con,
            (*con).name,
        );
    }
    if (*con).fullscreen_mode as ::core::ffi::c_uint == fullscreen_mode as ::core::ffi::c_uint {
        debuglog(
            c"%s:%s:%d - fullscreen already enabled for %p / %s\n"
                .as_ptr()
                .cast_mut(),
            c"../src/con.c".as_ptr(),
            c"con_enable_fullscreen".as_ptr(),
            1259 as ::core::ffi::c_int,
            con,
            (*con).name,
        );
        return;
    }
    let con_ws: *mut Con = con_get_workspace(con);
    let mut fullscreen: *mut Con = con_get_fullscreen_con(croot, CF_GLOBAL);
    if fullscreen.is_null() {
        fullscreen = con_get_fullscreen_con(con_ws, CF_OUTPUT);
    }
    if !fullscreen.is_null() {
        con_disable_fullscreen(fullscreen);
    }
    let cur_ws: *mut Con = con_get_workspace(focused);
    let old_focused: *mut Con = focused;
    if fullscreen_mode as ::core::ffi::c_uint
        == CF_GLOBAL as ::core::ffi::c_int as ::core::ffi::c_uint
        && cur_ws != con_ws
    {
        workspace_show(con_ws);
    }
    con_activate(con);
    if fullscreen_mode as ::core::ffi::c_uint
        != CF_GLOBAL as ::core::ffi::c_int as ::core::ffi::c_uint
        && cur_ws != con_ws
    {
        con_activate(old_focused);
    }
    con_set_fullscreen_mode(con, fullscreen_mode);
}

pub unsafe fn con_disable_fullscreen(con: *mut Con) {
    if (*con).type_0 as ::core::ffi::c_uint
        == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        debuglog(
            c"%s:%s:%d - You cannot make a workspace fullscreen.\n"
                .as_ptr()
                .cast_mut(),
            c"../src/con.c".as_ptr(),
            c"con_disable_fullscreen".as_ptr(),
            1297 as ::core::ffi::c_int,
        );
        return;
    }
    debuglog(
        c"%s:%s:%d - disabling fullscreen for %p / %s\n"
            .as_ptr()
            .cast_mut(),
        c"../src/con.c".as_ptr(),
        c"con_disable_fullscreen".as_ptr(),
        1301 as ::core::ffi::c_int,
        con,
        (*con).name,
    );
    if (*con).fullscreen_mode as ::core::ffi::c_uint
        == CF_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        debuglog(
            c"%s:%s:%d - fullscreen already disabled for %p / %s\n"
                .as_ptr()
                .cast_mut(),
            c"../src/con.c".as_ptr(),
            c"con_disable_fullscreen".as_ptr(),
            1304 as ::core::ffi::c_int,
            con,
            (*con).name,
        );
        return;
    }
    con_set_fullscreen_mode(con, CF_NONE);
}
unsafe fn _con_move_to_con(
    mut con: *mut Con,
    mut target: *mut Con,
    behind_focused: bool,
    fix_coordinates: bool,
    dont_warp: bool,
    ignore_focus: bool,
    fix_percentage: bool,
) -> bool {
    let mut orig_target: *mut Con = target;
    let target_ws: *mut Con = con_get_workspace(target);
    if !ignore_focus && !con_fullscreen_permits_focusing(target_ws) {
        verboselog(
            c"Cannot move out of a fullscreen container.\n"
                .as_ptr()
                .cast_mut(),
        );
        return false_0 != 0;
    }
    if con_is_floating(con) {
        debuglog(
            c"%s:%s:%d - Container is floating, using parent instead.\n"
                .as_ptr()
                .cast_mut(),
            c"../src/con.c".as_ptr(),
            c"_con_move_to_con".as_ptr(),
            1322 as ::core::ffi::c_int,
        );
        con = (*con).parent as *mut Con;
    }
    let source_ws: *mut Con = con_get_workspace(con);
    if (*con).type_0 as ::core::ffi::c_uint
        == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        while !(*source_ws).floating_head.tqh_first.is_null() {
            let child: *mut Con = (*source_ws).floating_head.tqh_first as *mut Con;
            con_move_to_workspace(child, target_ws, true_0 != 0, true_0 != 0, false_0 != 0);
        }
        if con_is_leaf(con) {
            return false_0 != 0;
        }
        con = workspace_encapsulate(con);
        if con.is_null() {
            errorlog(
                c"ERROR: Workspace failed to move its contents into a container!\n"
                    .as_ptr()
                    .cast_mut(),
            );
            return false_0 != 0;
        }
    }
    let urgent: bool = (*con).urgent;
    let current_ws: *mut Con = con_get_workspace(focused);
    let source_output: *mut Con = con_get_output(con);
    let dest_output: *mut Con = con_get_output(target_ws);
    let mut focus_next: *mut Con = ::core::ptr::null_mut::<Con>();
    if !ignore_focus && source_ws == current_ws && target_ws != source_ws {
        focus_next = con_descend_focused(source_ws);
        if focus_next == con || con_has_parent(focus_next, con) as ::core::ffi::c_int != 0 {
            focus_next = con_next_focused(con);
        }
    }
    if (*target).type_0 as ::core::ffi::c_uint
        != CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        debuglog(
            c"%s:%s:%d - target originally = %p / %s / type %d\n"
                .as_ptr()
                .cast_mut(),
            c"../src/con.c".as_ptr(),
            c"_con_move_to_con".as_ptr(),
            1369 as ::core::ffi::c_int,
            target,
            (*target).name,
            (*target).type_0 as ::core::ffi::c_uint,
        );
        target = (*target).parent as *mut Con;
    }
    if (*target).type_0 as ::core::ffi::c_uint
        == CT_FLOATING_CON as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        debuglog(
            c"%s:%s:%d - floatingcon, going up even further\n"
                .as_ptr()
                .cast_mut(),
            c"../src/con.c".as_ptr(),
            c"_con_move_to_con".as_ptr(),
            1377 as ::core::ffi::c_int,
        );
        orig_target = target;
        target = (*target).parent as *mut Con;
    }
    if (*con).type_0 as ::core::ffi::c_uint
        == CT_FLOATING_CON as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let ws: *mut Con = con_get_workspace(target);
        debuglog(
            c"%s:%s:%d - This is a floating window, using workspace %p / %s\n"
                .as_ptr()
                .cast_mut(),
            c"../src/con.c".as_ptr(),
            c"_con_move_to_con".as_ptr(),
            1384 as ::core::ffi::c_int,
            ws,
            (*ws).name,
        );
        target = ws;
    }
    if source_output != dest_output {
        if fix_coordinates as ::core::ffi::c_int != 0
            && (*con).type_0 as ::core::ffi::c_uint
                == CT_FLOATING_CON as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            floating_fix_coordinates(
                con,
                &raw mut (*source_output).rect,
                &raw mut (*dest_output).rect,
            );
        } else {
            debuglog(
                c"%s:%s:%d - Not fixing coordinates, fix_coordinates flag = %d\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/con.c".as_ptr(),
                c"_con_move_to_con".as_ptr(),
                1394 as ::core::ffi::c_int,
                fix_coordinates as ::core::ffi::c_int,
            );
        }
    }
    let mut fullscreen: *mut Con = con_get_fullscreen_con(target_ws, CF_OUTPUT);
    let con_has_fullscreen: bool = (*con).fullscreen_mode as ::core::ffi::c_uint
        != CF_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
        || !con_get_fullscreen_con(con, CF_GLOBAL).is_null()
        || !con_get_fullscreen_con(con, CF_OUTPUT).is_null();
    if con_has_fullscreen as ::core::ffi::c_int != 0 && !fullscreen.is_null() {
        con_toggle_fullscreen(fullscreen, CF_OUTPUT as ::core::ffi::c_int);
        fullscreen = ::core::ptr::null_mut::<Con>();
    }
    debuglog(
        c"%s:%s:%d - Re-attaching container to %p / %s\n"
            .as_ptr()
            .cast_mut(),
        c"../src/con.c".as_ptr(),
        c"_con_move_to_con".as_ptr(),
        1414 as ::core::ffi::c_int,
        target,
        (*target).name,
    );
    let parent: *mut Con = (*con).parent as *mut Con;
    con_detach(con);
    _con_attach(
        con,
        target,
        if behind_focused as ::core::ffi::c_int != 0 {
            ::core::ptr::null_mut::<Con>()
        } else {
            orig_target
        },
        !behind_focused,
    );
    if fix_percentage {
        con_fix_percent(parent);
        (*con).percent = 0.0f64;
        con_fix_percent(target);
    }
    if !ignore_focus && !con_is_internal(target_ws) && fullscreen.is_null() {
        let old_focus_ws: *mut Con =
            (*output_get_content(dest_output)).focus_head.tqh_first as *mut Con;
        let old_focus: *mut Con = focused;
        con_activate(con_descend_focused(con));
        if old_focus_ws == current_ws
            && (*old_focus).type_0 as ::core::ffi::c_uint
                != CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            con_activate(old_focus);
        } else if con_get_workspace(focused) != old_focus_ws {
            con_focus(con_descend_focused(old_focus_ws));
        }
    }
    if !ignore_focus {
        workspace_show(current_ws);
        if dont_warp {
            debuglog(
                c"%s:%s:%d - x_set_warp_to(NULL) because dont_warp is set\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/con.c".as_ptr(),
                c"_con_move_to_con".as_ptr(),
                1455 as ::core::ffi::c_int,
            );
            x_set_warp_to(::core::ptr::null_mut::<Rect>());
        }
    }
    if !focus_next.is_null() {
        con_activate(con_descend_focused(focus_next));
    }
    if !con_is_leaf(con) {
        let mut child_0: *mut Con = ::core::ptr::null_mut::<Con>();
        child_0 = (*con).nodes_head.tqh_first as *mut Con;
        while !child_0.is_null() {
            if !(*child_0).window.is_null() {
                startup_sequence_delete_by_window((*child_0).window);
            }
            child_0 = (*child_0).nodes.tqe_next as *mut Con;
        }
    }
    if !(*con).window.is_null() {
        startup_sequence_delete_by_window((*con).window);
    }
    if urgent {
        workspace_update_urgent_flag(source_ws);
        con_set_urgency(con, true_0 != 0);
    }
    free((*con).deco_render_params as *mut ::core::ffi::c_void);
    (*con).deco_render_params = ::core::ptr::null_mut::<deco_render_params>();
    (*parent)
        .on_remove_child
        .expect("non-null function pointer")(parent);
    ipc_send_window_event(c"move".as_ptr(), con);
    ewmh_update_wm_desktop();
    true_0 != 0
}

pub unsafe fn con_move_to_target(con: *mut Con, mut target: *mut Con) -> bool {
    if con_get_workspace(target) == workspace_get(c"__i3_scratch".as_ptr()) {
        debuglog(
            c"%s:%s:%d - target container is in the scratchpad, moving container to scratchpad.\n"
                .as_ptr()
                .cast_mut(),
            c"../src/con.c".as_ptr(),
            c"con_move_to_target".as_ptr(),
            1501 as ::core::ffi::c_int,
        );
        scratchpad_move(con);
        return true_0 != 0;
    }
    if con_is_floating(target) {
        debuglog(
            c"%s:%s:%d - target container is floating, moving container to target's workspace.\n"
                .as_ptr()
                .cast_mut(),
            c"../src/con.c".as_ptr(),
            c"con_move_to_target".as_ptr(),
            1508 as ::core::ffi::c_int,
        );
        con_move_to_workspace(
            con,
            con_get_workspace(target),
            true_0 != 0,
            false_0 != 0,
            false_0 != 0,
        );
        return true_0 != 0;
    }
    if (*target).type_0 as ::core::ffi::c_uint
        == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
        && con_is_leaf(target) as ::core::ffi::c_int != 0
    {
        debuglog(
            c"%s:%s:%d - target container is an empty workspace, simply moving the container there.\n".as_ptr().cast_mut(),
            c"../src/con.c".as_ptr(),
            c"con_move_to_target".as_ptr(),
            1514 as ::core::ffi::c_int,
        );
        con_move_to_workspace(con, target, true_0 != 0, false_0 != 0, false_0 != 0);
        return true_0 != 0;
    }
    if con_is_split(target) {
        debuglog(
            c"%s:%s:%d - target is a split container, descending to the currently focused child.\n"
                .as_ptr()
                .cast_mut(),
            c"../src/con.c".as_ptr(),
            c"con_move_to_target".as_ptr(),
            1523 as ::core::ffi::c_int,
        );
        target = (*target).focus_head.tqh_first as *mut Con;
    }
    if con == target || con_has_parent(target, con) as ::core::ffi::c_int != 0 {
        debuglog(
            c"%s:%s:%d - cannot move the container to or inside itself, aborting.\n"
                .as_ptr()
                .cast_mut(),
            c"../src/con.c".as_ptr(),
            c"con_move_to_target".as_ptr(),
            1528 as ::core::ffi::c_int,
        );
        return false_0 != 0;
    }
    _con_move_to_con(
        con,
        target,
        false_0 != 0,
        true_0 != 0,
        false_0 != 0,
        false_0 != 0,
        true_0 != 0,
    )
}

pub unsafe fn con_move_to_mark(con: *mut Con, mark: *const ::core::ffi::c_char) -> bool {
    let target: *mut Con = con_by_mark(mark);
    if target.is_null() {
        debuglog(
            c"%s:%s:%d - found no container with mark \"%s\"\n"
                .as_ptr()
                .cast_mut(),
            c"../src/con.c".as_ptr(),
            c"con_move_to_mark".as_ptr(),
            1542 as ::core::ffi::c_int,
            mark,
        );
        return false_0 != 0;
    }
    con_move_to_target(con, target)
}

pub unsafe fn con_move_to_workspace(
    con: *mut Con,
    workspace: *mut Con,
    fix_coordinates: bool,
    dont_warp: bool,
    ignore_focus: bool,
) {
    if (*workspace).type_0 as ::core::ffi::c_uint
        == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
    } else {
        __assert_fail(
            c"workspace->type == CT_WORKSPACE".as_ptr(),
            c"../src/con.c".as_ptr(),
            1570 as ::core::ffi::c_uint,
            c"void con_move_to_workspace(Con *, Con *, _Bool, _Bool, _Bool)".as_ptr(),
        );
    };
    let source_ws: *mut Con = con_get_workspace(con);
    if workspace == source_ws {
        debuglog(
            c"%s:%s:%d - Not moving, already there\n"
                .as_ptr()
                .cast_mut(),
            c"../src/con.c".as_ptr(),
            c"con_move_to_workspace".as_ptr(),
            1574 as ::core::ffi::c_int,
        );
        return;
    }
    let target: *mut Con = con_descend_focused(workspace);
    _con_move_to_con(
        con,
        target,
        true_0 != 0,
        fix_coordinates,
        dont_warp,
        ignore_focus,
        true_0 != 0,
    );
}

pub unsafe fn con_move_to_output(con: *mut Con, output: *mut Output, fix_coordinates: bool) {
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
    if ws != ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut Con {
    } else {
        __assert_fail(
            c"ws != NULL".as_ptr(),
            c"../src/con.c".as_ptr(),
            1590 as ::core::ffi::c_uint,
            c"void con_move_to_output(Con *, Output *, _Bool)".as_ptr(),
        );
    };
    debuglog(
        c"%s:%s:%d - Moving con %p to output %s\n"
            .as_ptr()
            .cast_mut(),
        c"../src/con.c".as_ptr(),
        c"con_move_to_output".as_ptr(),
        1591 as ::core::ffi::c_int,
        con,
        output_primary_name(output),
    );
    con_move_to_workspace(con, ws, fix_coordinates, false_0 != 0, false_0 != 0);
}

pub unsafe fn con_move_to_output_name(
    con: *mut Con,
    name: *const ::core::ffi::c_char,
    fix_coordinates: bool,
) -> bool {
    let current_output: *mut Output = get_output_for_con(con);
    let output: *mut Output = get_output_from_string(current_output, name);
    if output.is_null() {
        errorlog(
            c"ERROR: Could not find output \"%s\"\n".as_ptr().cast_mut(),
            name,
        );
        return false_0 != 0;
    }
    con_move_to_output(con, output, fix_coordinates);
    true_0 != 0
}

pub unsafe fn con_orientation(con: *mut Con) -> Orientation {
    match (*con).layout {
        Layout::SplitV | Layout::Stacked => Orientation::Vertical,
        Layout::SplitH | Layout::Tabbed => Orientation::Horizontal,
        Layout::Default => {
            errorlog(
                c"ERROR: Someone called con_orientation() on a con with L_DEFAULT, this is a bug in the code.\n".as_ptr().cast_mut(),
            );
            __assert_fail(
                c"false".as_ptr(),
                c"../src/con.c".as_ptr(),
                1634,
                c"orientation_t con_orientation(Con *)".as_ptr(),
            )
        }
        Layout::DockArea | Layout::Output => {
            errorlog(
                c"ERROR: con_orientation() called on dockarea/output (%d) container %p\n"
                    .as_ptr()
                    .cast_mut(),
                (*con).layout as ::core::ffi::c_uint,
                con,
            );
            __assert_fail(
                c"false".as_ptr(),
                c"../src/con.c".as_ptr(),
                1639,
                c"orientation_t con_orientation(Con *)".as_ptr(),
            )
        }
    }
}

pub unsafe fn con_next_focused(mut con: *mut Con) -> *mut Con {
    if (*(*con).parent).type_0 as ::core::ffi::c_uint
        == CT_DOCKAREA as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        debuglog(
            c"%s:%s:%d - selecting workspace for dock client\n"
                .as_ptr()
                .cast_mut(),
            c"../src/con.c".as_ptr(),
            c"con_next_focused".as_ptr(),
            1654 as ::core::ffi::c_int,
        );
        return con_descend_focused(output_get_content((*(*con).parent).parent as *mut Con));
    }
    if con_is_floating(con) {
        con = (*con).parent as *mut Con;
    }
    let mut next: *mut Con = (*(*con).parent).focus_head.tqh_first as *mut Con;
    if next != con {
        debuglog(
            c"%s:%s:%d - Using first entry %p\n".as_ptr().cast_mut(),
            c"../src/con.c".as_ptr(),
            c"con_next_focused".as_ptr(),
            1665 as ::core::ffi::c_int,
            next,
        );
    } else {
        next = (*con).focused.tqe_next as *mut Con;
        if next.is_null() {
            next = (*con).parent as *mut Con;
        }
    }
    while !(*next).focus_head.tqh_first.is_null() && (*next).focus_head.tqh_first != con {
        next = (*next).focus_head.tqh_first as *mut Con;
    }
    if (*con).type_0 as ::core::ffi::c_uint
        == CT_FLOATING_CON as ::core::ffi::c_int as ::core::ffi::c_uint
        && next != (*con).parent
    {
        next = con_descend_focused(next);
    }
    next
}

pub unsafe fn con_descend_focused(con: *mut Con) -> *mut Con {
    let mut next: *mut Con = con;
    while next != focused && !(*next).focus_head.tqh_first.is_null() {
        next = (*next).focus_head.tqh_first as *mut Con;
    }
    next
}

pub unsafe fn con_descend_tiling_focused(con: *mut Con) -> *mut Con {
    let mut next: *mut Con = con;
    let mut before: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut child: *mut Con = ::core::ptr::null_mut::<Con>();
    if next == focused {
        return next;
    }
    loop {
        before = next;
        child = (*next).focus_head.tqh_first as *mut Con;
        while !child.is_null() {
            if (*child).type_0 as ::core::ffi::c_uint
                == CT_FLOATING_CON as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                child = (*child).focused.tqe_next as *mut Con;
            } else {
                next = child;
                break;
            }
        }
        if !(before != next && next != focused) {
            break;
        }
    }
    next
}

pub unsafe fn con_descend_direction(con: *mut Con, direction: Direction) -> *mut Con {
    let mut most: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut current: *mut Con = ::core::ptr::null_mut::<Con>();
    let orientation: Orientation = con_orientation(con);
    debuglog(
        c"%s:%s:%d - con_descend_direction(%p, orientation %d, direction %d)\n"
            .as_ptr()
            .cast_mut(),
        c"../src/con.c".as_ptr(),
        c"con_descend_direction".as_ptr(),
        1741 as ::core::ffi::c_int,
        con,
        (orientation) as u32,
        direction as ::core::ffi::c_uint,
    );
    if direction == Direction::Left || direction == Direction::Right {
        if orientation == Orientation::Horizontal {
            if direction == Direction::Right {
                most = (*con).nodes_head.tqh_first as *mut Con;
            } else {
                most = *(*((*con).nodes_head.tqh_last as *mut nodes_head)).tqh_last as *mut Con;
            }
        } else if orientation == Orientation::Vertical {
            current = (*con).focus_head.tqh_first as *mut Con;
            while !current.is_null() {
                if (*current).type_0 as ::core::ffi::c_uint
                    != CT_FLOATING_CON as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    most = current;
                    break;
                } else {
                    current = (*current).focused.tqe_next as *mut Con;
                }
            }
        } else {
            return con;
        }
    }
    if direction == Direction::Up || direction == Direction::Down {
        if orientation == Orientation::Vertical {
            if direction == Direction::Up {
                most = *(*((*con).nodes_head.tqh_last as *mut nodes_head)).tqh_last as *mut Con;
            } else {
                most = (*con).nodes_head.tqh_first as *mut Con;
            }
        } else if orientation == Orientation::Horizontal {
            current = (*con).focus_head.tqh_first as *mut Con;
            while !current.is_null() {
                if (*current).type_0 as ::core::ffi::c_uint
                    != CT_FLOATING_CON as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    most = current;
                    break;
                } else {
                    current = (*current).focused.tqe_next as *mut Con;
                }
            }
        } else {
            return con;
        }
    }
    if most.is_null() {
        return con;
    }
    con_descend_direction(most, direction)
}
unsafe fn has_outer_gaps(gaps: gaps_t) -> bool {
    gaps.top > 0 as ::core::ffi::c_int
        || gaps.right > 0 as ::core::ffi::c_int
        || gaps.bottom > 0 as ::core::ffi::c_int
        || gaps.left > 0 as ::core::ffi::c_int
}

pub unsafe fn con_draw_decoration_into_frame(con: *mut Con) -> bool {
    con_is_leaf(con) as ::core::ffi::c_int != 0
        && con_border_style(con) == BorderStyle::Normal
        && ((*con).parent.is_null()
            || (*(*con).parent).layout != Layout::Tabbed
                && (*(*con).parent).layout != Layout::Stacked)
}
unsafe fn con_border_style_rect_without_title(con: *mut Con) -> Rect {
    if config.hide_edge_borders as ::core::ffi::c_uint
        == HEBM_SMART as ::core::ffi::c_int as ::core::ffi::c_uint
        && con_num_visible_children(con_get_workspace(con)) <= 1 as ::core::ffi::c_int
        || config.hide_edge_borders as ::core::ffi::c_uint
            == HEBM_SMART_NO_GAPS as ::core::ffi::c_int as ::core::ffi::c_uint
            && con_num_visible_children(con_get_workspace(con)) <= 1 as ::core::ffi::c_int
            && !has_outer_gaps(calculate_effective_gaps(con))
    {
        if !con_is_floating(con) {
            return Rect {
                x: 0 as u32,
                y: 0 as u32,
                width: 0 as u32,
                height: 0 as u32,
            };
        }
    }
    let mut borders_to_hide: adjacent_t = ADJ_NONE;
    let mut border_width: ::core::ffi::c_int = (*con).current_border_width;
    debuglog(
        c"%s:%s:%d - The border width for con is set to: %d\n"
            .as_ptr()
            .cast_mut(),
        c"../src/con.c".as_ptr(),
        c"con_border_style_rect_without_title".as_ptr(),
        1831 as ::core::ffi::c_int,
        (*con).current_border_width,
    );
    let mut result: Rect = Rect {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    if (*con).current_border_width < 0 as ::core::ffi::c_int {
        if con_is_floating(con) {
            border_width = config.default_floating_border_width;
        } else {
            border_width = config.default_border_width;
        }
    }
    debuglog(
        c"%s:%s:%d - Effective border width is set to: %d\n"
            .as_ptr()
            .cast_mut(),
        c"../src/con.c".as_ptr(),
        c"con_border_style_rect_without_title".as_ptr(),
        1840 as ::core::ffi::c_int,
        border_width,
    );
    let border_style: BorderStyle = con_border_style(con);
    if border_style == BorderStyle::None {
        return Rect {
            x: 0 as u32,
            y: 0 as u32,
            width: 0 as u32,
            height: 0 as u32,
        };
    }
    if border_style == BorderStyle::Normal {
        result = Rect {
            x: border_width as u32,
            y: 0 as u32,
            width: -(2 as ::core::ffi::c_int * border_width) as u32,
            height: -border_width as u32,
        };
    } else {
        result = Rect {
            x: border_width as u32,
            y: border_width as u32,
            width: -(2 as ::core::ffi::c_int * border_width) as u32,
            height: -(2 as ::core::ffi::c_int * border_width) as u32,
        };
    }
    if config.hide_edge_borders as ::core::ffi::c_uint
        == HEBM_SMART_NO_GAPS as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        borders_to_hide = (con_adjacent_borders(con) as ::core::ffi::c_uint
            & HEBM_NONE as ::core::ffi::c_int as ::core::ffi::c_uint)
            as adjacent_t;
    } else {
        borders_to_hide = (con_adjacent_borders(con) as ::core::ffi::c_uint
            & config.hide_edge_borders as ::core::ffi::c_uint)
            as adjacent_t;
    }
    if borders_to_hide as ::core::ffi::c_uint
        & ADJ_LEFT_SCREEN_EDGE as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        result.x = result.x.wrapping_sub(border_width as u32);
        result.width = result.width.wrapping_add(border_width as u32);
    }
    if borders_to_hide as ::core::ffi::c_uint
        & ADJ_RIGHT_SCREEN_EDGE as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        result.width = result.width.wrapping_add(border_width as u32);
    }
    if borders_to_hide as ::core::ffi::c_uint
        & ADJ_UPPER_SCREEN_EDGE as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
        && border_style != BorderStyle::Normal
    {
        result.y = result.y.wrapping_sub(border_width as u32);
        result.height = result.height.wrapping_add(border_width as u32);
    }
    if borders_to_hide as ::core::ffi::c_uint
        & ADJ_LOWER_SCREEN_EDGE as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        result.height = result.height.wrapping_add(border_width as u32);
    }
    result
}

pub unsafe fn con_border_style_rect(con: *mut Con) -> Rect {
    let mut result: Rect = con_border_style_rect_without_title(con);
    if con_border_style(con) == BorderStyle::Normal
        && con_draw_decoration_into_frame(con) as ::core::ffi::c_int != 0
    {
        let deco_height: ::core::ffi::c_int = render_deco_height() as ::core::ffi::c_int;
        result.y = result.y.wrapping_add(deco_height as u32);
        result.height = result.height.wrapping_sub(deco_height as u32);
    }
    result
}

pub unsafe fn con_adjacent_borders(con: *mut Con) -> adjacent_t {
    let mut result: adjacent_t = ADJ_NONE;
    if con_is_floating(con) {
        return result;
    }
    let workspace: *mut Con = con_get_workspace(con);
    if (*con).rect.x == (*workspace).rect.x {
        result = ::core::mem::transmute::<::core::ffi::c_uint, adjacent_t>(
            result as ::core::ffi::c_uint
                | ADJ_LEFT_SCREEN_EDGE as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
    }
    if (*con).rect.x.wrapping_add((*con).rect.width)
        == (*workspace).rect.x.wrapping_add((*workspace).rect.width)
    {
        result = ::core::mem::transmute::<::core::ffi::c_uint, adjacent_t>(
            result as ::core::ffi::c_uint
                | ADJ_RIGHT_SCREEN_EDGE as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
    }
    if (*con).rect.y == (*workspace).rect.y {
        result = ::core::mem::transmute::<::core::ffi::c_uint, adjacent_t>(
            result as ::core::ffi::c_uint
                | ADJ_UPPER_SCREEN_EDGE as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
    }
    if (*con).rect.y.wrapping_add((*con).rect.height)
        == (*workspace).rect.y.wrapping_add((*workspace).rect.height)
    {
        result = ::core::mem::transmute::<::core::ffi::c_uint, adjacent_t>(
            result as ::core::ffi::c_uint
                | ADJ_LOWER_SCREEN_EDGE as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
    }
    result
}

pub unsafe fn con_border_style(con: *mut Con) -> BorderStyle {
    if (*con).fullscreen_mode as ::core::ffi::c_uint
        == CF_OUTPUT as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*con).fullscreen_mode as ::core::ffi::c_uint
            == CF_GLOBAL as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        debuglog(
            c"%s:%s:%d - this one is fullscreen! overriding BS_NONE\n"
                .as_ptr()
                .cast_mut(),
            c"../src/con.c".as_ptr(),
            c"con_border_style".as_ptr(),
            1933 as ::core::ffi::c_int,
        );
        return BorderStyle::None;
    }
    if !(*con).parent.is_null() {
        if (*(*con).parent).layout == Layout::Stacked {
            return if con_num_children((*con).parent as *mut Con) == 1 as ::core::ffi::c_int {
                (*con).border_style
            } else {
                BorderStyle::Normal
            };
        }
        if (*(*con).parent).layout == Layout::Tabbed && (*con).border_style != BorderStyle::Normal {
            return if con_num_children((*con).parent as *mut Con) == 1 as ::core::ffi::c_int {
                (*con).border_style
            } else {
                BorderStyle::Normal
            };
        }
        if (*(*con).parent).type_0 as ::core::ffi::c_uint
            == CT_DOCKAREA as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return BorderStyle::None;
        }
    }
    (*con).border_style
}

pub unsafe fn con_set_border_style(
    con: *mut Con,
    mut border_style: BorderStyle,
    border_width: ::core::ffi::c_int,
) {
    if border_style as ::core::ffi::c_uint > (*con).max_user_border_style as ::core::ffi::c_uint {
        border_style = (*con).max_user_border_style;
    }
    if !con_is_floating(con) {
        (*con).border_style = border_style;
        (*con).current_border_width = border_width;
        return;
    }
    let parent: *mut Con = (*con).parent as *mut Con;
    let mut bsr: Rect = con_border_style_rect(con);
    (*con).rect = rect_add((*con).rect as Rect, bsr) as Rect;
    (*parent).rect = rect_add((*parent).rect as Rect, bsr) as Rect;
    (*con).border_style = border_style;
    (*con).current_border_width = border_width;
    bsr = con_border_style_rect(con);
    (*con).rect = rect_sub((*con).rect as Rect, bsr) as Rect;
    (*parent).rect = rect_sub((*parent).rect as Rect, bsr) as Rect;
}

pub unsafe fn con_set_layout(mut con: *mut Con, layout: Layout) {
    debuglog(
        c"%s:%s:%d - con_set_layout(%p, %d), con->type = %d\n"
            .as_ptr()
            .cast_mut(),
        c"../src/con.c".as_ptr(),
        c"con_set_layout".as_ptr(),
        1999 as ::core::ffi::c_int,
        con,
        layout as ::core::ffi::c_uint,
        (*con).type_0 as ::core::ffi::c_uint,
    );
    if (*con).type_0 as ::core::ffi::c_uint
        != CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        con = (*con).parent as *mut Con;
    }
    if (*con).layout == Layout::SplitH || (*con).layout == Layout::SplitV {
        (*con).last_split_layout = (*con).layout;
    }
    if (*con).type_0 as ::core::ffi::c_uint
        == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if con_num_children(con) == 0 as ::core::ffi::c_int {
            let ws_layout: Layout = if layout == Layout::Stacked || layout == Layout::Tabbed {
                layout
            } else {
                Layout::Default
            };
            debuglog(
                c"%s:%s:%d - Setting workspace_layout to %d\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/con.c".as_ptr(),
                c"con_set_layout".as_ptr(),
                2022 as ::core::ffi::c_int,
                ws_layout as ::core::ffi::c_uint,
            );
            (*con).workspace_layout = ws_layout;
            debuglog(
                c"%s:%s:%d - Setting layout to %d\n".as_ptr().cast_mut(),
                c"../src/con.c".as_ptr(),
                c"con_set_layout".as_ptr(),
                2024 as ::core::ffi::c_int,
                layout as ::core::ffi::c_uint,
            );
            (*con).layout = layout;
        } else if layout == Layout::Stacked
            || layout == Layout::Tabbed
            || layout == Layout::SplitV
            || layout == Layout::SplitH
        {
            debuglog(
                c"%s:%s:%d - Creating new split container\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/con.c".as_ptr(),
                c"con_set_layout".as_ptr(),
                2027 as ::core::ffi::c_int,
            );
            let new: *mut Con = con_new(
                ::core::ptr::null_mut::<Con>(),
                ::core::ptr::null_mut::<i3Window>(),
            );
            (*new).parent = con as *mut Con;
            (*new).layout = layout;
            (*new).last_split_layout = (*con).last_split_layout;
            let focus_order: *mut *mut Con = get_focus_order(con);
            debuglog(
                c"%s:%s:%d - Moving cons\n".as_ptr().cast_mut(),
                c"../src/con.c".as_ptr(),
                c"con_set_layout".as_ptr(),
                2040 as ::core::ffi::c_int,
            );
            while !(*con).nodes_head.tqh_first.is_null() {
                let child: *mut Con = (*con).nodes_head.tqh_first as *mut Con;
                con_detach(child);
                con_attach(child, new, true_0 != 0);
            }
            set_focus_order(new, focus_order);
            free(focus_order as *mut ::core::ffi::c_void);
            debuglog(
                c"%s:%s:%d - Attaching new split to ws\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/con.c".as_ptr(),
                c"con_set_layout".as_ptr(),
                2051 as ::core::ffi::c_int,
            );
            con_attach(new, con, false_0 != 0);
            tree_flatten(croot);
            con_force_split_parents_redraw(con);
            return;
        }
    }
    if (*con).type_0 as ::core::ffi::c_uint
        != CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
        && (*(*con).parent).type_0 as ::core::ffi::c_uint
            != CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
        && con_num_children(con) == 1 as ::core::ffi::c_int
        && con_num_children((*con).parent as *mut Con) == 1 as ::core::ffi::c_int
    {
        let parent: *mut Con = (*con).parent as *mut Con;
        let child_0: *mut Con = (*con).nodes_head.tqh_first as *mut Con;
        con_detach(child_0);
        con_attach(child_0, parent, true_0 != 0);
        (*parent).last_split_layout = (*con).last_split_layout;
        tree_close_internal(con, KillWindow::DontKill, true_0 != 0);
        con = parent;
    }
    if layout == Layout::Default {
        (*con).layout = (*con).last_split_layout;
        if (*con).layout == Layout::Default {
            (*con).layout = Layout::SplitH;
        }
    } else {
        (*con).layout = layout;
    }
    con_force_split_parents_redraw(con);
}

pub unsafe fn con_toggle_layout(con: *mut Con, toggle_mode: *const ::core::ffi::c_char) {
    let mut parent: *mut Con = con;
    if (*con).type_0 as ::core::ffi::c_uint
        != CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        parent = (*con).parent as *mut Con;
    }
    debuglog(
        c"%s:%s:%d - con_toggle_layout(%p, %s), parent = %p\n"
            .as_ptr()
            .cast_mut(),
        c"../src/con.c".as_ptr(),
        c"con_toggle_layout".as_ptr(),
        2117 as ::core::ffi::c_int,
        con,
        toggle_mode,
        parent,
    );
    let delim: [::core::ffi::c_char; 2] =
        ::core::mem::transmute::<[u8; 2], [::core::ffi::c_char; 2]>(*b" \0");
    if strcasecmp(toggle_mode, c"split".as_ptr()) == 0 as ::core::ffi::c_int
        || !strstr(toggle_mode, &raw const delim as *const ::core::ffi::c_char).is_null()
    {
        let mut new_layout: Layout = Layout::Default;
        let mut current_layout_found: bool = false_0 != 0;
        let tm_dup: *mut ::core::ffi::c_char = sstrdup(toggle_mode);
        let mut cur_tok: *mut ::core::ffi::c_char =
            strtok(tm_dup, &raw const delim as *const ::core::ffi::c_char);
        let mut current_block_20: u64;
        let mut layout: Layout = Layout::Default;
        while !cur_tok.is_null() {
            if strcasecmp(cur_tok, c"split".as_ptr()) == 0 as ::core::ffi::c_int {
                if (*parent).layout != Layout::SplitH && (*parent).layout != Layout::SplitV {
                    layout = (*parent).last_split_layout;
                    if layout == Layout::Default {
                        layout = Layout::SplitH;
                    }
                } else {
                    layout = if (*parent).layout == Layout::SplitH {
                        Layout::SplitV
                    } else {
                        Layout::SplitH
                    };
                }
                current_block_20 = 6057473163062296781;
            } else {
                let success: bool = layout_from_name(cur_tok, &raw mut layout);
                if !success || layout == Layout::Default {
                    errorlog(
                        c"ERROR: The token '%s' was not recognized and has been skipped.\n"
                            .as_ptr()
                            .cast_mut(),
                        cur_tok,
                    );
                    current_block_20 = 4906268039856690917;
                } else {
                    current_block_20 = 6057473163062296781;
                }
            }
            match current_block_20 {
                6057473163062296781 => {
                    if new_layout == Layout::Default {
                        new_layout = layout;
                    }
                    if current_layout_found {
                        new_layout = layout;
                        break;
                    } else if (*parent).layout == layout {
                        current_layout_found = true_0 != 0;
                    }
                }
                _ => {}
            }
            cur_tok = strtok(
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
                &raw const delim as *const ::core::ffi::c_char,
            );
        }
        free(tm_dup as *mut ::core::ffi::c_void);
        if new_layout != Layout::Default {
            con_set_layout(con, new_layout);
        }
    } else if strcasecmp(toggle_mode, c"all".as_ptr()) == 0 as ::core::ffi::c_int
        || strcasecmp(toggle_mode, c"default".as_ptr()) == 0 as ::core::ffi::c_int
    {
        if (*parent).layout == Layout::Stacked {
            con_set_layout(con, Layout::Tabbed);
        } else if (*parent).layout == Layout::Tabbed {
            if strcasecmp(toggle_mode, c"all".as_ptr()) == 0 as ::core::ffi::c_int {
                con_set_layout(con, Layout::SplitH);
            } else {
                con_set_layout(con, (*parent).last_split_layout);
            }
        } else if (*parent).layout == Layout::SplitH || (*parent).layout == Layout::SplitV {
            if strcasecmp(toggle_mode, c"all".as_ptr()) == 0 as ::core::ffi::c_int {
                if (*parent).layout == Layout::SplitH {
                    con_set_layout(con, Layout::SplitV);
                } else {
                    con_set_layout(con, Layout::Stacked);
                }
            } else {
                con_set_layout(con, Layout::Stacked);
            }
        }
    }
}
unsafe fn con_on_remove_child(con: *mut Con) {
    debuglog(
        c"%s:%s:%d - on_remove_child\n".as_ptr().cast_mut(),
        c"../src/con.c".as_ptr(),
        c"con_on_remove_child".as_ptr(),
        2206 as ::core::ffi::c_int,
    );
    if (*con).type_0 as ::core::ffi::c_uint
        == CT_OUTPUT as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*con).type_0 as ::core::ffi::c_uint
            == CT_ROOT as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*con).type_0 as ::core::ffi::c_uint
            == CT_DOCKAREA as ::core::ffi::c_int as ::core::ffi::c_uint
        || !(*con).parent.is_null()
            && (*(*con).parent).type_0 as ::core::ffi::c_uint
                == CT_OUTPUT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        debuglog(
            c"%s:%s:%d - not handling, type = %d, name = %s\n"
                .as_ptr()
                .cast_mut(),
            c"../src/con.c".as_ptr(),
            c"con_on_remove_child".as_ptr(),
            2214 as ::core::ffi::c_int,
            (*con).type_0 as ::core::ffi::c_uint,
            (*con).name,
        );
        return;
    }
    if (*con).type_0 as ::core::ffi::c_uint
        == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if (*con).focus_head.tqh_first.is_null() && !workspace_is_visible(con) {
            verboselog(
                c"Closing old workspace (%p / %s), it is empty\n"
                    .as_ptr()
                    .cast_mut(),
                con,
                (*con).name,
            );
            let gen: yajl_gen =
                ipc_marshal_workspace_event(c"empty".as_ptr(), con, ::core::ptr::null_mut::<Con>());
            tree_close_internal(con, KillWindow::DontKill, false_0 != 0);
            let mut payload: *const ::core::ffi::c_uchar =
                ::core::ptr::null::<::core::ffi::c_uchar>();
            let mut length: ylength = 0;
            yajl_gen_get_buf(gen, &raw mut payload, &raw mut length);
            ipc_send_event(
                c"workspace".as_ptr(),
                I3_IPC_EVENT_WORKSPACE as u32,
                payload as *const ::core::ffi::c_char,
            );
            yajl_gen_free(gen);
        }
        return;
    }
    con_force_split_parents_redraw(con);
    (*con).urgent = con_has_urgent_child(con);
    con_update_parents_urgency(con);
    let children: ::core::ffi::c_int = con_num_children(con);
    if children == 0 as ::core::ffi::c_int {
        debuglog(
            c"%s:%s:%d - Container empty, closing\n".as_ptr().cast_mut(),
            c"../src/con.c".as_ptr(),
            c"con_on_remove_child".as_ptr(),
            2243 as ::core::ffi::c_int,
        );
        tree_close_internal(con, KillWindow::DontKill, false_0 != 0);
    }
}

pub unsafe fn con_minimum_size(con: *mut Con) -> Rect {
    debuglog(
        c"%s:%s:%d - Determining minimum size for con %p\n"
            .as_ptr()
            .cast_mut(),
        c"../src/con.c".as_ptr(),
        c"con_minimum_size".as_ptr(),
        2254 as ::core::ffi::c_int,
        con,
    );
    if con_is_leaf(con) {
        debuglog(
            c"%s:%s:%d - leaf node, returning 75x50\n"
                .as_ptr()
                .cast_mut(),
            c"../src/con.c".as_ptr(),
            c"con_minimum_size".as_ptr(),
            2257 as ::core::ffi::c_int,
        );
        return Rect {
            x: 0 as u32,
            y: 0 as u32,
            width: 75 as u32,
            height: 50 as u32,
        };
    }
    if (*con).type_0 as ::core::ffi::c_uint
        == CT_FLOATING_CON as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        debuglog(
            c"%s:%s:%d - floating con\n".as_ptr().cast_mut(),
            c"../src/con.c".as_ptr(),
            c"con_minimum_size".as_ptr(),
            2262 as ::core::ffi::c_int,
        );
        let child: *mut Con = (*con).nodes_head.tqh_first as *mut Con;
        return con_minimum_size(child);
    }
    if (*con).layout == Layout::Stacked || (*con).layout == Layout::Tabbed {
        let mut max_width: u32 = 0 as u32;
        let mut max_height: u32 = 0 as u32;
        let mut deco_height: u32 = 0 as u32;
        let mut child_0: *mut Con = ::core::ptr::null_mut::<Con>();
        child_0 = (*con).nodes_head.tqh_first as *mut Con;
        while !child_0.is_null() {
            let min: Rect = con_minimum_size(child_0);
            deco_height = deco_height.wrapping_add((*child_0).deco_rect.height);
            max_width = max(
                max_width as ::core::ffi::c_int,
                min.width as ::core::ffi::c_int,
            ) as u32;
            max_height = max(
                max_height as ::core::ffi::c_int,
                min.height as ::core::ffi::c_int,
            ) as u32;
            child_0 = (*child_0).nodes.tqe_next as *mut Con;
        }
        debuglog(
            c"%s:%s:%d - stacked/tabbed now, returning %d x %d + deco_rect = %d\n"
                .as_ptr()
                .cast_mut(),
            c"../src/con.c".as_ptr(),
            c"con_minimum_size".as_ptr(),
            2277 as ::core::ffi::c_int,
            max_width,
            max_height,
            deco_height,
        );
        return Rect {
            x: 0 as u32,
            y: 0 as u32,
            width: max_width,
            height: max_height.wrapping_add(deco_height),
        };
    }
    if con_is_split(con) {
        let mut width: u32 = 0 as u32;
        let mut height: u32 = 0 as u32;
        let mut child_1: *mut Con = ::core::ptr::null_mut::<Con>();
        child_1 = (*con).nodes_head.tqh_first as *mut Con;
        while !child_1.is_null() {
            let min_0: Rect = con_minimum_size(child_1);
            if (*con).layout == Layout::SplitH {
                width = width.wrapping_add(min_0.width);
                height = max(
                    height as ::core::ffi::c_int,
                    min_0.height as ::core::ffi::c_int,
                ) as u32;
            } else {
                height = height.wrapping_add(min_0.height);
                width = max(
                    width as ::core::ffi::c_int,
                    min_0.width as ::core::ffi::c_int,
                ) as u32;
            }
            child_1 = (*child_1).nodes.tqe_next as *mut Con;
        }
        debuglog(
            c"%s:%s:%d - split container, returning width = %d x height = %d\n"
                .as_ptr()
                .cast_mut(),
            c"../src/con.c".as_ptr(),
            c"con_minimum_size".as_ptr(),
            2297 as ::core::ffi::c_int,
            width,
            height,
        );
        return Rect {
            x: 0 as u32,
            y: 0 as u32,
            width,
            height,
        };
    }
    errorlog(
        c"ERROR: Unhandled case, type = %d, layout = %d, split = %d\n"
            .as_ptr()
            .cast_mut(),
        (*con).type_0 as ::core::ffi::c_uint,
        (*con).layout as ::core::ffi::c_uint,
        con_is_split(con) as ::core::ffi::c_int,
    );
    if 0 as ::core::ffi::c_int != 0 {
    } else {
        __assert_fail(
            c"false".as_ptr(),
            c"../src/con.c".as_ptr(),
            2303 as ::core::ffi::c_uint,
            c"Rect con_minimum_size(Con *)".as_ptr(),
        );
    };
    panic!("Reached end of non-void function without returning");
}

pub unsafe fn con_fullscreen_permits_focusing(con: *mut Con) -> bool {
    if focused.is_null() {
        return true_0 != 0;
    }
    let mut fs: *mut Con = focused;
    while !fs.is_null()
        && (*fs).fullscreen_mode as ::core::ffi::c_uint
            == CF_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        fs = (*fs).parent as *mut Con;
    }
    if fs != ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut Con {
    } else {
        __assert_fail(
            c"fs != NULL".as_ptr(),
            c"../src/con.c".as_ptr(),
            2341 as ::core::ffi::c_uint,
            c"_Bool con_fullscreen_permits_focusing(Con *)".as_ptr(),
        );
    };
    if (*fs).fullscreen_mode as ::core::ffi::c_uint
        != CF_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
    } else {
        __assert_fail(
            c"fs->fullscreen_mode != CF_NONE".as_ptr(),
            c"../src/con.c".as_ptr(),
            2344 as ::core::ffi::c_uint,
            c"_Bool con_fullscreen_permits_focusing(Con *)".as_ptr(),
        );
    };
    if (*fs).type_0 as ::core::ffi::c_uint
        == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return true_0 != 0;
    }
    if con == fs {
        return true_0 != 0;
    }
    if (*fs).fullscreen_mode as ::core::ffi::c_uint
        == CF_OUTPUT as ::core::ffi::c_int as ::core::ffi::c_uint
        && con_get_workspace(con) != con_get_workspace(fs)
    {
        return true_0 != 0;
    }
    con_has_parent(con, fs)
}

pub unsafe fn con_has_urgent_child(con: *mut Con) -> bool {
    let mut child: *mut Con = ::core::ptr::null_mut::<Con>();
    if con_is_leaf(con) {
        return (*con).urgent;
    }
    child = (*con).nodes_head.tqh_first as *mut Con;
    while !child.is_null() {
        if con_has_urgent_child(child) {
            return true_0 != 0;
        }
        child = (*child).nodes.tqe_next as *mut Con;
    }
    false_0 != 0
}

pub unsafe fn con_update_parents_urgency(con: *mut Con) {
    let mut parent: *mut Con = (*con).parent as *mut Con;
    if (*con).type_0 as ::core::ffi::c_uint
        == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return;
    }
    let new_urgency_value: bool = (*con).urgent;
    while !parent.is_null()
        && (*parent).type_0 as ::core::ffi::c_uint
            != CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
        && (*parent).type_0 as ::core::ffi::c_uint
            != CT_DOCKAREA as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if new_urgency_value {
            (*parent).urgent = true_0 != 0;
        } else if !con_has_urgent_child(parent) {
            (*parent).urgent = false_0 != 0;
        }
        parent = (*parent).parent as *mut Con;
    }
}

pub unsafe fn con_set_urgency(con: *mut Con, urgent: bool) {
    if urgent as ::core::ffi::c_int != 0 && focused == con {
        debuglog(
            c"%s:%s:%d - Ignoring urgency flag for current client\n"
                .as_ptr()
                .cast_mut(),
            c"../src/con.c".as_ptr(),
            c"con_set_urgency".as_ptr(),
            2426 as ::core::ffi::c_int,
        );
        return;
    }
    let old_urgent: bool = (*con).urgent;
    if (*con).urgency_timer.is_null() {
        (*con).urgent = urgent;
    } else {
        debuglog(
            c"%s:%s:%d - Discarding urgency WM_HINT because timer is running\n"
                .as_ptr()
                .cast_mut(),
            c"../src/con.c".as_ptr(),
            c"con_set_urgency".as_ptr(),
            2435 as ::core::ffi::c_int,
        );
    }
    if !(*con).window.is_null() {
        if (*con).urgent {
            gettimeofday(&raw mut (*(*con).window).urgent, ::core::ptr::null_mut());
        } else {
            (*(*con).window).urgent.tv_sec = 0 as libc::time_t;
            (*(*con).window).urgent.tv_usec = 0 as libc::suseconds_t;
        }
    }
    con_update_parents_urgency(con);
    let mut ws: *mut Con = ::core::ptr::null_mut::<Con>();
    ws = con_get_workspace(con);
    if !ws.is_null() {
        workspace_update_urgent_flag(ws);
    }
    if (*con).urgent as ::core::ffi::c_int != old_urgent as ::core::ffi::c_int {
        verboselog(
            c"Urgency flag changed to %d\n".as_ptr().cast_mut(),
            (*con).urgent as ::core::ffi::c_int,
        );
        ipc_send_window_event(c"urgent".as_ptr(), con);
    }
}

pub unsafe fn con_get_tree_representation(con: *mut Con) -> *mut ::core::ffi::c_char {
    if con_is_leaf(con) {
        if (*con).window.is_null() {
            return sstrdup(c"nowin".as_ptr());
        }
        if (*(*con).window).class_instance.is_null() {
            return sstrdup(c"noinstance".as_ptr());
        }
        return sstrdup((*(*con).window).class_instance);
    }
    let mut buf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if (*con).layout == Layout::Default {
        buf = sstrdup(c"D[".as_ptr());
    } else if (*con).layout == Layout::SplitV {
        buf = sstrdup(c"V[".as_ptr());
    } else if (*con).layout == Layout::SplitH {
        buf = sstrdup(c"H[".as_ptr());
    } else if (*con).layout == Layout::Tabbed {
        buf = sstrdup(c"T[".as_ptr());
    } else if (*con).layout == Layout::Stacked {
        buf = sstrdup(c"S[".as_ptr());
    } else {
        errorlog(
            c"ERROR: BUG: Code not updated to account for new layout type\n"
                .as_ptr()
                .cast_mut(),
        );
        if 0 as ::core::ffi::c_int != 0 {
        } else {
            __assert_fail(
                c"false".as_ptr(),
                c"../src/con.c".as_ptr(),
                2503 as ::core::ffi::c_uint,
                c"char *con_get_tree_representation(Con *)".as_ptr(),
            );
        };
    }
    let mut child: *mut Con = ::core::ptr::null_mut::<Con>();
    child = (*con).nodes_head.tqh_first as *mut Con;
    while !child.is_null() {
        let child_txt: *mut ::core::ffi::c_char = con_get_tree_representation(child);
        let mut tmp_buf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        sasprintf(
            &raw mut tmp_buf,
            c"%s%s%s".as_ptr(),
            buf,
            if (*con).nodes_head.tqh_first == child {
                c"".as_ptr()
            } else {
                c" ".as_ptr()
            },
            child_txt,
        );
        free(buf as *mut ::core::ffi::c_void);
        buf = tmp_buf;
        free(child_txt as *mut ::core::ffi::c_void);
        child = (*child).nodes.tqe_next as *mut Con;
    }
    let mut complete_buf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    sasprintf(&raw mut complete_buf, c"%s]".as_ptr(), buf);
    free(buf as *mut ::core::ffi::c_void);
    complete_buf
}

pub unsafe fn con_parse_title_format(con: *mut Con) -> *mut i3String {
    if (*con).title_format
        != ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut ::core::ffi::c_char
    {
    } else {
        __assert_fail(
            c"con->title_format != NULL".as_ptr(),
            c"../src/con.c".as_ptr(),
            2532 as ::core::ffi::c_uint,
            c"i3String *con_parse_title_format(Con *)".as_ptr(),
        );
    };
    let win: *mut i3Window = (*con).window as *mut i3Window;
    let pango_markup: bool = font_is_pango() as bool;
    let mut title: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut class: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut instance: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut machine: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if win.is_null() {
        title = pango_escape_markup(con_get_tree_representation(con));
        class = sstrdup(c"i3-frame".as_ptr());
        instance = sstrdup(c"i3-frame".as_ptr());
        machine = sstrdup(c"".as_ptr());
    } else {
        title = pango_escape_markup(sstrdup(if (*win).name.is_null() {
            c"".as_ptr()
        } else {
            i3string_as_utf8((*win).name)
        }));
        class = pango_escape_markup(sstrdup(if (*win).class_class.is_null() {
            c"".as_ptr()
        } else {
            (*win).class_class as *const ::core::ffi::c_char
        }));
        instance = pango_escape_markup(sstrdup(if (*win).class_instance.is_null() {
            c"".as_ptr()
        } else {
            (*win).class_instance as *const ::core::ffi::c_char
        }));
        machine = pango_escape_markup(sstrdup(if (*win).machine.is_null() {
            c"".as_ptr()
        } else {
            (*win).machine as *const ::core::ffi::c_char
        }));
    }
    let mut placeholders: [placeholder_t; 4] = [
        placeholder_t {
            name: c"%title".as_ptr(),
            value: title,
        },
        placeholder_t {
            name: c"%class".as_ptr(),
            value: class,
        },
        placeholder_t {
            name: c"%instance".as_ptr(),
            value: instance,
        },
        placeholder_t {
            name: c"%machine".as_ptr(),
            value: machine,
        },
    ];
    let num: usize = (::core::mem::size_of::<[placeholder_t; 4]>() as usize)
        .wrapping_div(::core::mem::size_of::<placeholder_t>() as usize);
    let formatted_str: *mut ::core::ffi::c_char = format_placeholders(
        (*con).title_format,
        (&raw mut placeholders as *mut placeholder_t).offset(0 as ::core::ffi::c_int as isize)
            as *mut placeholder_t,
        num as ::core::ffi::c_int,
    );
    let formatted: *mut i3String = i3string_from_utf8(formatted_str);
    i3string_set_markup(formatted, pango_markup);
    free(formatted_str as *mut ::core::ffi::c_void);
    free(title as *mut ::core::ffi::c_void);
    free(class as *mut ::core::ffi::c_void);
    free(instance as *mut ::core::ffi::c_void);
    formatted
}

pub unsafe fn con_swap(first: *mut Con, second: *mut Con) -> bool {
    if first != ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut Con {
    } else {
        __assert_fail(
            c"first != NULL".as_ptr(),
            c"../src/con.c".as_ptr(),
            2581 as ::core::ffi::c_uint,
            c"_Bool con_swap(Con *, Con *)".as_ptr(),
        );
    };
    if second != ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut Con {
    } else {
        __assert_fail(
            c"second != NULL".as_ptr(),
            c"../src/con.c".as_ptr(),
            2582 as ::core::ffi::c_uint,
            c"_Bool con_swap(Con *, Con *)".as_ptr(),
        );
    };
    debuglog(
        c"%s:%s:%d - Swapping containers %p / %p\n"
            .as_ptr()
            .cast_mut(),
        c"../src/con.c".as_ptr(),
        c"con_swap".as_ptr(),
        2583 as ::core::ffi::c_int,
        first,
        second,
    );
    if (*first).type_0 as ::core::ffi::c_uint != CT_CON as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        errorlog(
            c"ERROR: Only regular containers can be swapped, but found con = %p with type = %d.\n"
                .as_ptr()
                .cast_mut(),
            first,
            (*first).type_0 as ::core::ffi::c_uint,
        );
        return false_0 != 0;
    }
    if (*second).type_0 as ::core::ffi::c_uint
        != CT_CON as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        errorlog(
            c"ERROR: Only regular containers can be swapped, but found con = %p with type = %d.\n"
                .as_ptr()
                .cast_mut(),
            second,
            (*second).type_0 as ::core::ffi::c_uint,
        );
        return false_0 != 0;
    }
    if first == second {
        debuglog(
            c"%s:%s:%d - Swapping container %p with itself, nothing to do.\n"
                .as_ptr()
                .cast_mut(),
            c"../src/con.c".as_ptr(),
            c"con_swap".as_ptr(),
            2596 as ::core::ffi::c_int,
            first,
        );
        return false_0 != 0;
    }
    if con_has_parent(first, second) as ::core::ffi::c_int != 0
        || con_has_parent(second, first) as ::core::ffi::c_int != 0
    {
        errorlog(
            c"ERROR: Cannot swap containers %p and %p because they are in a parent-child relationship.\n".as_ptr().cast_mut(),
            first,
            second,
        );
        return false_0 != 0;
    }
    let ws1: *mut Con = con_get_workspace(first);
    let ws2: *mut Con = con_get_workspace(second);
    let mut restore_focus: *mut Con = ::core::ptr::null_mut::<Con>();
    if ws1 == ws2 && ws1 == con_get_workspace(focused) {
        restore_focus = focused;
    } else if first == focused || con_has_parent(focused, first) as ::core::ffi::c_int != 0 {
        restore_focus = second;
    } else if second == focused || con_has_parent(focused, second) as ::core::ffi::c_int != 0 {
        restore_focus = first;
    }
    let head1: *mut nodes_head = &raw mut (*(*first).parent).nodes_head;
    let head2: *mut nodes_head = &raw mut (*(*second).parent).nodes_head;
    let first_prev: *mut Con = *(*((*first).nodes.tqe_prev as *mut nodes_head)).tqh_last;
    let second_prev: *mut Con = *(*((*second).nodes.tqe_prev as *mut nodes_head)).tqh_last;
    if second_prev == first {
        *(*first).nodes.tqe_prev = second as *mut Con;
        (*second).nodes.tqe_prev = (*first).nodes.tqe_prev;
        (*first).nodes.tqe_prev = &raw mut (*second).nodes.tqe_next;
        (*first).nodes.tqe_next = (*second).nodes.tqe_next;
        if !(*second).nodes.tqe_next.is_null() {
            (*(*second).nodes.tqe_next).nodes.tqe_prev = &raw mut (*first).nodes.tqe_next;
        }
        (*second).nodes.tqe_next = first as *mut Con;
        if (*head1).tqh_last == &raw mut (*second).nodes.tqe_next {
            (*head1).tqh_last = &raw mut (*first).nodes.tqe_next;
        }
    } else if first_prev == second {
        *(*second).nodes.tqe_prev = first as *mut Con;
        (*first).nodes.tqe_prev = (*second).nodes.tqe_prev;
        (*second).nodes.tqe_prev = &raw mut (*first).nodes.tqe_next;
        (*second).nodes.tqe_next = (*first).nodes.tqe_next;
        if !(*first).nodes.tqe_next.is_null() {
            (*(*first).nodes.tqe_next).nodes.tqe_prev = &raw mut (*second).nodes.tqe_next;
        }
        (*first).nodes.tqe_next = second as *mut Con;
        if (*head1).tqh_last == &raw mut (*first).nodes.tqe_next {
            (*head1).tqh_last = &raw mut (*second).nodes.tqe_next;
        }
    } else {
        if !(*first).nodes.tqe_next.is_null() {
            (*(*first).nodes.tqe_next).nodes.tqe_prev = (*first).nodes.tqe_prev;
        } else {
            (*head1).tqh_last = (*first).nodes.tqe_prev;
        }
        *(*first).nodes.tqe_prev = (*first).nodes.tqe_next;
        if !(*second).nodes.tqe_next.is_null() {
            (*(*second).nodes.tqe_next).nodes.tqe_prev = (*second).nodes.tqe_prev;
        } else {
            (*head2).tqh_last = (*second).nodes.tqe_prev;
        }
        *(*second).nodes.tqe_prev = (*second).nodes.tqe_next;
        if second_prev.is_null() {
            (*first).nodes.tqe_next = (*head2).tqh_first;
            if !(*first).nodes.tqe_next.is_null() {
                (*(*head2).tqh_first).nodes.tqe_prev = &raw mut (*first).nodes.tqe_next;
            } else {
                (*head2).tqh_last = &raw mut (*first).nodes.tqe_next;
            }
            (*head2).tqh_first = first as *mut Con;
            (*first).nodes.tqe_prev = &raw mut (*head2).tqh_first;
        } else {
            (*first).nodes.tqe_next = (*second_prev).nodes.tqe_next;
            if !(*first).nodes.tqe_next.is_null() {
                (*(*first).nodes.tqe_next).nodes.tqe_prev = &raw mut (*first).nodes.tqe_next;
            } else {
                (*head2).tqh_last = &raw mut (*first).nodes.tqe_next;
            }
            (*second_prev).nodes.tqe_next = first as *mut Con;
            (*first).nodes.tqe_prev = &raw mut (*second_prev).nodes.tqe_next;
        }
        if first_prev.is_null() {
            (*second).nodes.tqe_next = (*head1).tqh_first;
            if !(*second).nodes.tqe_next.is_null() {
                (*(*head1).tqh_first).nodes.tqe_prev = &raw mut (*second).nodes.tqe_next;
            } else {
                (*head1).tqh_last = &raw mut (*second).nodes.tqe_next;
            }
            (*head1).tqh_first = second as *mut Con;
            (*second).nodes.tqe_prev = &raw mut (*head1).tqh_first;
        } else {
            (*second).nodes.tqe_next = (*first_prev).nodes.tqe_next;
            if !(*second).nodes.tqe_next.is_null() {
                (*(*second).nodes.tqe_next).nodes.tqe_prev = &raw mut (*second).nodes.tqe_next;
            } else {
                (*head1).tqh_last = &raw mut (*second).nodes.tqe_next;
            }
            (*first_prev).nodes.tqe_next = second as *mut Con;
            (*second).nodes.tqe_prev = &raw mut (*first_prev).nodes.tqe_next;
        }
    }
    let head1_0: *mut focus_head = &raw mut (*(*first).parent).focus_head;
    let head2_0: *mut focus_head = &raw mut (*(*second).parent).focus_head;
    let first_prev_0: *mut Con = *(*((*first).focused.tqe_prev as *mut focus_head)).tqh_last;
    let second_prev_0: *mut Con = *(*((*second).focused.tqe_prev as *mut focus_head)).tqh_last;
    if second_prev_0 == first {
        *(*first).focused.tqe_prev = second as *mut Con;
        (*second).focused.tqe_prev = (*first).focused.tqe_prev;
        (*first).focused.tqe_prev = &raw mut (*second).focused.tqe_next;
        (*first).focused.tqe_next = (*second).focused.tqe_next;
        if !(*second).focused.tqe_next.is_null() {
            (*(*second).focused.tqe_next).focused.tqe_prev = &raw mut (*first).focused.tqe_next;
        }
        (*second).focused.tqe_next = first as *mut Con;
        if (*head1_0).tqh_last == &raw mut (*second).focused.tqe_next {
            (*head1_0).tqh_last = &raw mut (*first).focused.tqe_next;
        }
    } else if first_prev_0 == second {
        *(*second).focused.tqe_prev = first as *mut Con;
        (*first).focused.tqe_prev = (*second).focused.tqe_prev;
        (*second).focused.tqe_prev = &raw mut (*first).focused.tqe_next;
        (*second).focused.tqe_next = (*first).focused.tqe_next;
        if !(*first).focused.tqe_next.is_null() {
            (*(*first).focused.tqe_next).focused.tqe_prev = &raw mut (*second).focused.tqe_next;
        }
        (*first).focused.tqe_next = second as *mut Con;
        if (*head1_0).tqh_last == &raw mut (*first).focused.tqe_next {
            (*head1_0).tqh_last = &raw mut (*second).focused.tqe_next;
        }
    } else {
        if !(*first).focused.tqe_next.is_null() {
            (*(*first).focused.tqe_next).focused.tqe_prev = (*first).focused.tqe_prev;
        } else {
            (*head1_0).tqh_last = (*first).focused.tqe_prev;
        }
        *(*first).focused.tqe_prev = (*first).focused.tqe_next;
        if !(*second).focused.tqe_next.is_null() {
            (*(*second).focused.tqe_next).focused.tqe_prev = (*second).focused.tqe_prev;
        } else {
            (*head2_0).tqh_last = (*second).focused.tqe_prev;
        }
        *(*second).focused.tqe_prev = (*second).focused.tqe_next;
        if second_prev_0.is_null() {
            (*first).focused.tqe_next = (*head2_0).tqh_first;
            if !(*first).focused.tqe_next.is_null() {
                (*(*head2_0).tqh_first).focused.tqe_prev = &raw mut (*first).focused.tqe_next;
            } else {
                (*head2_0).tqh_last = &raw mut (*first).focused.tqe_next;
            }
            (*head2_0).tqh_first = first as *mut Con;
            (*first).focused.tqe_prev = &raw mut (*head2_0).tqh_first;
        } else {
            (*first).focused.tqe_next = (*second_prev_0).focused.tqe_next;
            if !(*first).focused.tqe_next.is_null() {
                (*(*first).focused.tqe_next).focused.tqe_prev = &raw mut (*first).focused.tqe_next;
            } else {
                (*head2_0).tqh_last = &raw mut (*first).focused.tqe_next;
            }
            (*second_prev_0).focused.tqe_next = first as *mut Con;
            (*first).focused.tqe_prev = &raw mut (*second_prev_0).focused.tqe_next;
        }
        if first_prev_0.is_null() {
            (*second).focused.tqe_next = (*head1_0).tqh_first;
            if !(*second).focused.tqe_next.is_null() {
                (*(*head1_0).tqh_first).focused.tqe_prev = &raw mut (*second).focused.tqe_next;
            } else {
                (*head1_0).tqh_last = &raw mut (*second).focused.tqe_next;
            }
            (*head1_0).tqh_first = second as *mut Con;
            (*second).focused.tqe_prev = &raw mut (*head1_0).tqh_first;
        } else {
            (*second).focused.tqe_next = (*first_prev_0).focused.tqe_next;
            if !(*second).focused.tqe_next.is_null() {
                (*(*second).focused.tqe_next).focused.tqe_prev =
                    &raw mut (*second).focused.tqe_next;
            } else {
                (*head1_0).tqh_last = &raw mut (*second).focused.tqe_next;
            }
            (*first_prev_0).focused.tqe_next = second as *mut Con;
            (*second).focused.tqe_prev = &raw mut (*first_prev_0).focused.tqe_next;
        }
    }
    let tmp_SWAP: *mut Con = (*first).parent as *mut Con;
    (*first).parent = (*second).parent;
    (*second).parent = tmp_SWAP as *mut Con;
    let tmp_SWAP_0: FloatingState = (*first).floating;
    (*first).floating = (*second).floating;
    (*second).floating = tmp_SWAP_0;
    let tmp_SWAP_1: Rect = (*first).rect as Rect;
    (*first).rect = (*second).rect;
    (*second).rect = tmp_SWAP_1 as Rect;
    let tmp_SWAP_2: Rect = (*first).window_rect as Rect;
    (*first).window_rect = (*second).window_rect;
    (*second).window_rect = tmp_SWAP_2 as Rect;
    let tmp_SWAP_3: ::core::ffi::c_double = (*first).percent;
    (*first).percent = (*second).percent;
    (*second).percent = tmp_SWAP_3;
    if !restore_focus.is_null() {
        con_focus(restore_focus);
    }
    con_set_urgency(first, (*first).urgent);
    con_set_urgency(second, (*second).urgent);
    let second_fullscreen_mode: fullscreen_mode_t = (*second).fullscreen_mode;
    if (*first).fullscreen_mode as ::core::ffi::c_uint
        == CF_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        con_disable_fullscreen(second);
    } else {
        con_enable_fullscreen(second, (*first).fullscreen_mode);
    }
    if second_fullscreen_mode as ::core::ffi::c_uint
        == CF_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        con_disable_fullscreen(first);
    } else {
        con_enable_fullscreen(first, second_fullscreen_mode);
    }
    con_fix_percent((*first).parent as *mut Con);
    con_fix_percent((*second).parent as *mut Con);
    free((*first).deco_render_params as *mut ::core::ffi::c_void);
    (*first).deco_render_params = ::core::ptr::null_mut::<deco_render_params>();
    free((*second).deco_render_params as *mut ::core::ffi::c_void);
    (*second).deco_render_params = ::core::ptr::null_mut::<deco_render_params>();
    con_force_split_parents_redraw(first);
    con_force_split_parents_redraw(second);
    true_0 != 0
}

pub unsafe fn con_rect_size_in_orientation(con: *mut Con) -> u32 {
    if con_orientation(con) == Orientation::Horizontal {
        (*con).rect.width
    } else {
        (*con).rect.height
    }
}

pub unsafe fn con_merge_into(old: *mut Con, new: *mut Con) {
    (*new).window = (*old).window;
    (*old).window = ::core::ptr::null_mut::<Window>();
    if !(*old).title_format.is_null() {
        free((*new).title_format as *mut ::core::ffi::c_void);
        (*new).title_format = ::core::ptr::null_mut::<::core::ffi::c_char>();
        (*new).title_format = (*old).title_format;
        (*old).title_format = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if !(*old).sticky_group.is_null() {
        free((*new).sticky_group as *mut ::core::ffi::c_void);
        (*new).sticky_group = ::core::ptr::null_mut::<::core::ffi::c_char>();
        (*new).sticky_group = (*old).sticky_group;
        (*old).sticky_group = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    (*new).sticky = (*old).sticky;
    con_set_urgency(new, (*old).urgent);
    let mut mark: *mut mark_t = ::core::ptr::null_mut::<mark_t>();
    mark = (*old).marks_head.tqh_first as *mut mark_t;
    while !mark.is_null() {
        (*mark).marks.tqe_next = ::core::ptr::null_mut::<mark_t>();
        (*mark).marks.tqe_prev = (*new).marks_head.tqh_last;
        *(*new).marks_head.tqh_last = mark as *mut mark_t;
        (*new).marks_head.tqh_last = &raw mut (*mark).marks.tqe_next;
        ipc_send_window_event(c"mark".as_ptr(), new);
        mark = (*mark).marks.tqe_next as *mut mark_t;
    }
    (*new).mark_changed = !(*old).marks_head.tqh_first.is_null();
    (*old).marks_head.tqh_first = ::core::ptr::null_mut::<mark_t>();
    (*old).marks_head.tqh_last = &raw mut (*old).marks_head.tqh_first;
    tree_close_internal(old, KillWindow::DontKill, false_0 != 0);
}

pub unsafe fn con_inside_stacked_or_tabbed(con: *mut Con) -> bool {
    if (*con).parent.is_null() {
        return false_0 != 0;
    }
    if (*(*con).parent).layout == Layout::Stacked || (*(*con).parent).layout == Layout::Tabbed {
        return true_0 != 0;
    }
    con_inside_stacked_or_tabbed((*con).parent as *mut Con)
}

// Internal helpers use Rust linkage rather than foreign symbol lookup.
use crate::ewmh::ewmh_update_wm_desktop;
use crate::ipc::ipc_send_event;
use crate::render::render_deco_height;
use crate::util::layout_from_name;
use crate::util::max;
use crate::util::pango_escape_markup;
use i3_common::font::font_is_pango;
use i3_common::safewrappers::scalloc;
use i3_common::safewrappers::smalloc;
use i3_common::safewrappers::sstrdup;
use i3_common::safewrappers::strcasecmp_nullable;

// Canonical Cairo backend ABI; renderer logic remains local.

// Canonical Pango backend ABI; font policy remains local.

use xcb_sys::{xcb_window_t, XCB_NONE, XCB_WINDOW_NONE};

pub use libc::{free, gettimeofday, strcasecmp, strcmp, strstr, strtok};

use i3_common::native_stdio::__assert_fail;

use crate::log::{debuglog, errorlog, verboselog};

// Shared model types retain field layout while using one Rust identity.
pub use crate::types::deco_render_params;
pub use crate::types::focus_head;
pub use crate::types::gaps_t;
pub use crate::types::mark_t;
pub use crate::types::nodes_head;
pub use crate::types::xoutput;
pub use crate::types::Con;
pub use crate::types::Match;
pub use crate::types::Rect;
pub use crate::types::Window;
pub use i3_common::font::Font;
pub use i3_common::string::I3String as _i3String;

// Internal calls and globals use canonical Rust identities.
use crate::application::conn;
use crate::application::root_depth;
use crate::application::A__NET_WM_STATE;
use crate::application::A__NET_WM_STATE_FULLSCREEN;
use crate::config::config;
use crate::floating::floating_fix_coordinates;
use crate::floating::floating_raise_con;
use crate::gaps::calculate_effective_gaps;
use crate::ipc::ipc_marshal_workspace_event;
use crate::ipc::ipc_send_window_event;
use crate::output::get_output_for_con;
use crate::output::get_output_from_string;
use crate::output::output_get_content;
use crate::output::output_primary_name;
use crate::r#match::match_free;
use crate::r#match::match_matches_window;
use crate::scratchpad::scratchpad_move;
use crate::startup::startup_sequence_delete_by_window;
use crate::tree::croot;
use crate::tree::focused;
use crate::tree::tree_close_internal;
use crate::tree::tree_flatten;
use crate::util::rect_add;
use crate::util::rect_sub;
use crate::workspace::workspace_attach_to;
use crate::workspace::workspace_encapsulate;
use crate::workspace::workspace_get;
use crate::workspace::workspace_is_visible;
use crate::workspace::workspace_show;
use crate::workspace::workspace_update_urgent_flag;
use crate::x::x_con_init;
use crate::x::x_set_warp_to;
use crate::xcb::xcb_add_property_atom;
use crate::xcb::xcb_remove_property_atom;
use i3_common::string::i3string_as_utf8;
use i3_common::string::i3string_from_utf8;
use i3_common::string::i3string_set_markup;
