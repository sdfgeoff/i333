use crate::con::arena::parent::{parent_ptr, set_parent_ptr};
use crate::con::arena::{floating, identity, resolve};
use crate::handlers::add_ignore_event;
use crate::types::Direction;
use crate::types::FocusWrapping;
use crate::types::KillWindow;
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
 * tree.c: Everything that primarily modifies the layout tree data structure.
 *
 */
// Mechanical file-by-file Rust translation of src/tree.c.
// Originally translated with C2Rust 0.22.1; subsequently maintained in Rust.
pub use libev_sys::{ev_timer, ev_timer_stop};

pub type fullscreen_mode_t = ::core::ffi::c_uint;
pub const CF_GLOBAL: fullscreen_mode_t = 2;
pub const CF_OUTPUT: fullscreen_mode_t = 1;
pub const CF_NONE: fullscreen_mode_t = 0;

pub const CT_DOCKAREA: ::core::ffi::c_uint = 5;
pub const CT_WORKSPACE: ::core::ffi::c_uint = 4;
pub const CT_FLOATING_CON: ::core::ffi::c_uint = 3;
pub const CT_CON: ::core::ffi::c_uint = 2;
pub const CT_OUTPUT: ::core::ffi::c_uint = 1;
pub const CT_ROOT: ::core::ffi::c_uint = 0;

pub type Output = xoutput;
pub type i3Window = Window;

pub use crate::application::ignore_event_sequence_t;

pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();

pub static mut croot: *mut Con = ::core::ptr::null::<Con>() as *mut Con;

pub static mut focused: *mut Con = ::core::ptr::null::<Con>() as *mut Con;

unsafe fn _create___i3() -> *mut Con {
    let mut __i3: *mut Con = con_new(croot, ::core::ptr::null_mut::<i3Window>());
    free((*__i3).name as *mut ::core::ffi::c_void);
    (*__i3).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*__i3).name = sstrdup(c"__i3".as_ptr());
    (*__i3).type_0 = CT_OUTPUT;
    (*__i3).layout = Layout::Output;
    con_fix_percent(croot);
    x_set_name(__i3, c"[i3 con] pseudo-output __i3".as_ptr());
    (*__i3).rect.width = 1280 as u32;
    (*__i3).rect.height = 1024 as u32;
    debuglog(
        c"%s:%s:%d - adding main content container\n"
            .as_ptr()
            .cast_mut(),
        c"../src/tree.c".as_ptr(),
        c"_create___i3".as_ptr(),
        39 as ::core::ffi::c_int,
    );
    let content: *mut Con = con_new(
        ::core::ptr::null_mut::<Con>(),
        ::core::ptr::null_mut::<i3Window>(),
    );
    (*content).type_0 = CT_CON;
    free((*content).name as *mut ::core::ffi::c_void);
    (*content).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*content).name = sstrdup(c"content".as_ptr());
    (*content).layout = Layout::SplitH;
    x_set_name(content, c"[i3 con] content __i3".as_ptr());
    con_attach(content, __i3, false);
    let ws: *mut Con = con_new(
        ::core::ptr::null_mut::<Con>(),
        ::core::ptr::null_mut::<i3Window>(),
    );
    (*ws).type_0 = CT_WORKSPACE;
    (*ws).num = -(1 as ::core::ffi::c_int);
    (*ws).name = sstrdup(c"__i3_scratch".as_ptr());
    (*ws).layout = Layout::SplitH;
    con_attach(ws, content, false);
    x_set_name(ws, c"[i3 con] workspace __i3_scratch".as_ptr());
    (*ws).fullscreen_mode = CF_OUTPUT;
    __i3
}

/// A uniquely owned malloc buffer returned by resolve_tilde/slurp. Tree parsing
/// may retain partial graph effects, but never borrows these file bytes afterward.
struct RestoreBuffer(*mut libc::c_char);
impl Drop for RestoreBuffer {
    fn drop(&mut self) {
        unsafe { libc::free(self.0.cast()) };
    }
}

fn geometry_rect(x: i16, y: i16, width: u16, height: u16) -> Rect {
    Rect {
        x: x as u32,
        y: y as u32,
        width: u32::from(width),
        height: u32::from(height),
    }
}
fn flatten_layouts(con: Layout, child: Layout, parent: Orientation) -> bool {
    match (con, child) {
        (Layout::SplitH, Layout::SplitV) => parent == Orientation::Vertical,
        (Layout::SplitV, Layout::SplitH) => parent == Orientation::Horizontal,
        _ => false,
    }
}

pub unsafe fn tree_restore(
    path: *const ::core::ffi::c_char,
    geometry: *mut xcb_get_geometry_reply_t,
) -> bool {
    let mut out: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut ws: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut result: bool = false;
    let path_owner = RestoreBuffer(resolve_tilde(path));
    let globbed = path_owner.0;
    let mut buf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if !path_exists(globbed) {
        verboselog(
            c"%s does not exist, not restoring tree\n"
                .as_ptr()
                .cast_mut(),
            globbed,
        );
    } else {
        let len = slurp(globbed, &raw mut buf);
        let _buffer_owner = RestoreBuffer(buf);
        if !(len < 0 as isize) {
            croot = con_new(
                ::core::ptr::null_mut::<Con>(),
                ::core::ptr::null_mut::<i3Window>(),
            );
            (*croot).rect = geometry_rect(
                (*geometry).x,
                (*geometry).y,
                (*geometry).width,
                (*geometry).height,
            );
            focused = croot;
            tree_append_json(
                focused,
                buf,
                len as usize,
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            );
            debuglog(
                c"%s:%s:%d - appended tree, using new root\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/tree.c".as_ptr(),
                c"tree_restore".as_ptr(),
                93 as ::core::ffi::c_int,
            );
            croot = (*croot).nodes_head.tqh_first as *mut Con;
            if !croot.is_null() {
                debuglog(
                    c"%s:%s:%d - new root = %p\n".as_ptr().cast_mut(),
                    c"../src/tree.c".as_ptr(),
                    c"tree_restore".as_ptr(),
                    99 as ::core::ffi::c_int,
                    croot,
                );
                out = (*croot).nodes_head.tqh_first as *mut Con;
                debuglog(
                    c"%s:%s:%d - out = %p\n".as_ptr().cast_mut(),
                    c"../src/tree.c".as_ptr(),
                    c"tree_restore".as_ptr(),
                    101 as ::core::ffi::c_int,
                    out,
                );
                ws = (*out).nodes_head.tqh_first as *mut Con;
                debuglog(
                    c"%s:%s:%d - ws = %p\n".as_ptr().cast_mut(),
                    c"../src/tree.c".as_ptr(),
                    c"tree_restore".as_ptr(),
                    103 as ::core::ffi::c_int,
                    ws,
                );
                if strcmp((*out).name, c"__i3".as_ptr()) != 0 as ::core::ffi::c_int {
                    debuglog(
                        c"%s:%s:%d - Adding pseudo-output __i3 during inplace restart\n"
                            .as_ptr()
                            .cast_mut(),
                        c"../src/tree.c".as_ptr(),
                        c"tree_restore".as_ptr(),
                        108 as ::core::ffi::c_int,
                    );
                    let mut __i3: *mut Con = _create___i3();
                    if !(*__i3).nodes.tqe_next.is_null() {
                        (*(*__i3).nodes.tqe_next).nodes.tqe_prev = (*__i3).nodes.tqe_prev;
                    } else {
                        (*croot).nodes_head.tqh_last = (*__i3).nodes.tqe_prev;
                    }
                    *(*__i3).nodes.tqe_prev = (*__i3).nodes.tqe_next;
                    (*__i3).nodes.tqe_next = (*croot).nodes_head.tqh_first;
                    if !(*__i3).nodes.tqe_next.is_null() {
                        (*(*croot).nodes_head.tqh_first).nodes.tqe_prev =
                            &raw mut (*__i3).nodes.tqe_next;
                    } else {
                        (*croot).nodes_head.tqh_last = &raw mut (*__i3).nodes.tqe_next;
                    }
                    (*croot).nodes_head.tqh_first = __i3 as *mut Con;
                    (*__i3).nodes.tqe_prev = &raw mut (*croot).nodes_head.tqh_first;
                }
                restore_open_placeholder_windows(croot);
                result = true;
            }
        }
    }
    result
}

pub unsafe fn tree_init(geometry: *mut xcb_get_geometry_reply_t) {
    croot = con_new(
        ::core::ptr::null_mut::<Con>(),
        ::core::ptr::null_mut::<i3Window>(),
    );
    free((*croot).name as *mut ::core::ffi::c_void);
    (*croot).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*croot).name = c"root".as_ptr().cast_mut();
    (*croot).type_0 = CT_ROOT;
    (*croot).layout = Layout::SplitH;
    (*croot).rect = geometry_rect(
        (*geometry).x,
        (*geometry).y,
        (*geometry).width,
        (*geometry).height,
    );
    _create___i3();
}

pub unsafe fn tree_open_con(mut con: *mut Con, window: *mut i3Window) -> *mut Con {
    if con.is_null() {
        con = parent_ptr(focused) as *mut Con;
        if (*parent_ptr(con)).type_0 as ::core::ffi::c_uint
            == CT_OUTPUT as ::core::ffi::c_int as ::core::ffi::c_uint
            && (*con).type_0 as ::core::ffi::c_uint
                != CT_DOCKAREA as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            con = focused;
        }
        if (*con).type_0 as ::core::ffi::c_uint
            == CT_FLOATING_CON as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            con = con_descend_tiling_focused(parent_ptr(con) as *mut Con);
            if (*con).type_0 as ::core::ffi::c_uint
                != CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                con = parent_ptr(con) as *mut Con;
            }
        }
        debuglog(
            c"%s:%s:%d - con = %p\n".as_ptr().cast_mut(),
            c"../src/tree.c".as_ptr(),
            c"tree_open_con".as_ptr(),
            168 as ::core::ffi::c_int,
            con,
        );
    }
    if con != ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut Con {
    } else {
        __assert_fail(
            c"con != NULL".as_ptr(),
            c"../src/tree.c".as_ptr(),
            171 as ::core::ffi::c_uint,
            c"Con *tree_open_con(Con *, i3Window *)".as_ptr(),
        );
    };
    let new: *mut Con = con_new(con, window);
    (*new).layout = Layout::SplitH;
    con_fix_percent(con);
    new
}

pub unsafe fn tree_close_internal(
    con: *mut Con,
    kill_window: KillWindow,
    dont_kill_parent: bool,
) -> bool {
    let parent: *mut Con = parent_ptr(con) as *mut Con;
    if (*con).urgent {
        con_set_urgency(con, false);
        con_update_parents_urgency(con);
        workspace_update_urgent_flag(con_get_workspace(con));
    }
    debuglog(
        c"%s:%s:%d - closing %p, kill_window = %d\n"
            .as_ptr()
            .cast_mut(),
        c"../src/tree.c".as_ptr(),
        c"tree_close_internal".as_ptr(),
        202 as ::core::ffi::c_int,
        con,
        kill_window as ::core::ffi::c_uint,
    );
    let mut abort_kill: bool = false;
    let mut child: *mut Con = (*con).nodes_head.tqh_first as *mut Con;
    while !child.is_null() {
        let next_child: *mut Con = (*child).nodes.tqe_next as *mut Con;
        debuglog(
            c"%s:%s:%d - killing child=%p\n".as_ptr().cast_mut(),
            c"../src/tree.c".as_ptr(),
            c"tree_close_internal".as_ptr(),
            208 as ::core::ffi::c_int,
            child,
        );
        if !tree_close_internal(child, kill_window, true) {
            abort_kill = true;
        }
        child = next_child;
    }
    if abort_kill {
        debuglog(
            c"%s:%s:%d - One of the children could not be killed immediately (WM_DELETE sent), aborting.\n".as_ptr().cast_mut(),
            c"../src/tree.c".as_ptr(),
            c"tree_close_internal".as_ptr(),
            216 as ::core::ffi::c_int,
        );
        return false;
    }
    if !(*con).window.is_null() {
        if kill_window != KillWindow::DontKill {
            x_window_kill((*(*con).window).id, kill_window);
            return false;
        }
        let mut fresh0: [u32; 1] = [XCB_NONE as u32];
        xcb_change_window_attributes(
            conn,
            (*(*con).window).id,
            XCB_CW_EVENT_MASK as ::core::ffi::c_int as u32,
            &raw mut fresh0 as *mut u32 as *const ::core::ffi::c_void,
        );
        xcb_unmap_window(conn, (*(*con).window).id);
        let mut cookie: xcb_void_cookie_t = xcb_reparent_window(
            conn,
            (*(*con).window).id,
            root,
            (*con).rect.x as i16,
            (*con).rect.y as i16,
        );
        add_ignore_event(
            ignore_event_sequence_t {
                value: cookie.sequence as u32,
            },
            0 as ::core::ffi::c_int,
        );
        let mut data: [::core::ffi::c_long; 2] = [
            XCB_ICCCM_WM_STATE_WITHDRAWN as ::core::ffi::c_int as ::core::ffi::c_long,
            XCB_NONE,
        ];
        cookie = xcb_change_property(
            conn,
            XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as u8,
            (*(*con).window).id,
            A_WM_STATE,
            A_WM_STATE,
            32 as u8,
            2 as u32,
            &raw mut data as *mut ::core::ffi::c_long as *const ::core::ffi::c_void,
        );
        xcb_change_save_set(
            conn,
            XCB_SET_MODE_DELETE as ::core::ffi::c_int as u8,
            (*(*con).window).id,
        );
        if shape_supported {
            xcb_shape_select_input(conn, (*(*con).window).id, false_0 as u8);
        }
        add_ignore_event(
            ignore_event_sequence_t {
                value: cookie.sequence as u32,
            },
            0 as ::core::ffi::c_int,
        );
        ipc_send_window_event(c"close".as_ptr(), con);
        window_free((*con).window as *mut i3Window);
        (*con).window = ::core::ptr::null_mut::<Window>();
    }
    let ws: *mut Con = con_get_workspace(con);
    let next: *mut Con = if con == focused {
        con_next_focused(con)
    } else {
        ::core::ptr::null_mut::<Con>()
    };
    // Focus is deferred past teardown and its callbacks; generation checking
    // prevents a deleted/reused address from becoming an unintended target.
    let next_id = crate::con::arena::identity(next);
    debuglog(
        c"%s:%s:%d - next = %p, focused = %p\n".as_ptr().cast_mut(),
        c"../src/tree.c".as_ptr(),
        c"tree_close_internal".as_ptr(),
        267 as ::core::ffi::c_int,
        next,
        focused,
    );
    con_detach(con);
    if !(*con).urgency_timer.is_null() {
        debuglog(
            c"%s:%s:%d - Removing urgency timer of con %p\n"
                .as_ptr()
                .cast_mut(),
            c"../src/tree.c".as_ptr(),
            c"tree_close_internal".as_ptr(),
            274 as ::core::ffi::c_int,
            con,
        );
        workspace_update_urgent_flag(ws);
        ev_timer_stop(main_loop, (*con).urgency_timer as *mut ev_timer);
        free((*con).urgency_timer as *mut ::core::ffi::c_void);
        (*con).urgency_timer = ::core::ptr::null_mut::<ev_timer>();
    }
    if (*con).type_0 as ::core::ffi::c_uint
        != CT_FLOATING_CON as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        con_fix_percent(parent);
    }
    if !dont_kill_parent {
        tree_render();
    }
    x_con_kill(con);
    if ws == con {
        debuglog(
            c"%s:%s:%d - Closing workspace container %s, updating EWMH atoms\n"
                .as_ptr()
                .cast_mut(),
            c"../src/tree.c".as_ptr(),
            c"tree_close_internal".as_ptr(),
            302 as ::core::ffi::c_int,
            (*ws).name,
        );
        ewmh_update_desktop_properties();
    }
    con_free(con);
    if let Some(next) = next_id.and_then(crate::con::arena::resolve) {
        con_activate(next);
    } else {
        debuglog(
            c"%s:%s:%d - not changing focus, the container was not focused before\n"
                .as_ptr()
                .cast_mut(),
            c"../src/tree.c".as_ptr(),
            c"tree_close_internal".as_ptr(),
            311 as ::core::ffi::c_int,
        );
    }
    if !dont_kill_parent {
        (*parent)
            .on_remove_child
            .expect("non-null function pointer")(parent);
    }
    true
}

pub unsafe fn tree_split(mut con: *mut Con, orientation: Orientation) {
    if con_is_floating(con) {
        debuglog(
            c"%s:%s:%d - Floating containers can't be split.\n"
                .as_ptr()
                .cast_mut(),
            c"../src/tree.c".as_ptr(),
            c"tree_split".as_ptr(),
            329 as ::core::ffi::c_int,
        );
        return;
    }
    if (*con).type_0 as ::core::ffi::c_uint
        == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if con_num_children(con) < 2 as ::core::ffi::c_int {
            if con_num_children(con) == 0 as ::core::ffi::c_int {
                debuglog(
                    c"%s:%s:%d - Changing workspace_layout to L_DEFAULT\n"
                        .as_ptr()
                        .cast_mut(),
                    c"../src/tree.c".as_ptr(),
                    c"tree_split".as_ptr(),
                    336 as ::core::ffi::c_int,
                );
                (*con).workspace_layout = Layout::Default;
            }
            debuglog(
                c"%s:%s:%d - Changing orientation of workspace\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/tree.c".as_ptr(),
                c"tree_split".as_ptr(),
                339 as ::core::ffi::c_int,
            );
            (*con).layout = if orientation == Orientation::Horizontal {
                Layout::SplitH
            } else {
                Layout::SplitV
            };
            return;
        } else {
            con = workspace_encapsulate(con);
        }
    }
    let parent: *mut Con = parent_ptr(con) as *mut Con;
    con_force_split_parents_redraw(con);
    if con_num_children(parent) == 1 as ::core::ffi::c_int
        && ((*parent).layout == Layout::SplitH || (*parent).layout == Layout::SplitV)
    {
        (*parent).layout = if orientation == Orientation::Horizontal {
            Layout::SplitH
        } else {
            Layout::SplitV
        };
        debuglog(
            c"%s:%s:%d - Just changing orientation of existing container\n"
                .as_ptr()
                .cast_mut(),
            c"../src/tree.c".as_ptr(),
            c"tree_split".as_ptr(),
            361 as ::core::ffi::c_int,
        );
        return;
    }
    debuglog(
        c"%s:%s:%d - Splitting in orientation %d\n"
            .as_ptr()
            .cast_mut(),
        c"../src/tree.c".as_ptr(),
        c"tree_split".as_ptr(),
        365 as ::core::ffi::c_int,
        orientation as ::core::ffi::c_uint,
    );
    let new: *mut Con = con_new(
        ::core::ptr::null_mut::<Con>(),
        ::core::ptr::null_mut::<i3Window>(),
    );
    (*new).nodes.tqe_next = (*con).nodes.tqe_next;
    if !(*new).nodes.tqe_next.is_null() {
        (*(*new).nodes.tqe_next).nodes.tqe_prev = &raw mut (*new).nodes.tqe_next;
    } else {
        (*parent).nodes_head.tqh_last = &raw mut (*new).nodes.tqe_next;
    }
    (*new).nodes.tqe_prev = (*con).nodes.tqe_prev;
    *(*new).nodes.tqe_prev = new as *mut Con;
    (*new).focused.tqe_next = (*con).focused.tqe_next;
    if !(*new).focused.tqe_next.is_null() {
        (*(*new).focused.tqe_next).focused.tqe_prev = &raw mut (*new).focused.tqe_next;
    } else {
        (*parent).focus_head.tqh_last = &raw mut (*new).focused.tqe_next;
    }
    (*new).focused.tqe_prev = (*con).focused.tqe_prev;
    *(*new).focused.tqe_prev = new as *mut Con;
    set_parent_ptr(new, parent as *mut Con).expect("live acyclic ancestry");
    (*new).layout = if orientation == Orientation::Horizontal {
        Layout::SplitH
    } else {
        Layout::SplitV
    };
    (*new).percent = (*con).percent;
    (*con).percent = 0.0f64;
    con_attach(con, new, false);
}

pub unsafe fn level_up() -> bool {
    if (*parent_ptr(focused)).type_0 as ::core::ffi::c_uint
        == CT_FLOATING_CON as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        con_activate(parent_ptr(parent_ptr(focused)) as *mut Con);
        return true;
    }
    if (*parent_ptr(focused)).type_0 as ::core::ffi::c_uint
        != CT_CON as ::core::ffi::c_int as ::core::ffi::c_uint
        && (*parent_ptr(focused)).type_0 as ::core::ffi::c_uint
            != CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*focused).type_0 as ::core::ffi::c_uint
            == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        errorlog(
            c"ERROR: 'focus parent': Focus is already on the workspace, cannot go higher than that.\n".as_ptr().cast_mut(),
        );
        return false;
    }
    con_activate(parent_ptr(focused) as *mut Con);
    true
}

pub unsafe fn level_down() -> bool {
    let mut next: *mut Con = (*focused).focus_head.tqh_first as *mut Con;
    if next.is_null() {
        debuglog(
            c"%s:%s:%d - cannot go down\n".as_ptr().cast_mut(),
            c"../src/tree.c".as_ptr(),
            c"level_down".as_ptr(),
            413 as ::core::ffi::c_int,
        );
        return false;
    } else if (*next).type_0 as ::core::ffi::c_uint
        == CT_FLOATING_CON as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let child: *mut Con = (*next).focus_head.tqh_first as *mut Con;
        if child.is_null() {
            debuglog(
                c"%s:%s:%d - cannot go down\n".as_ptr().cast_mut(),
                c"../src/tree.c".as_ptr(),
                c"level_down".as_ptr(),
                420 as ::core::ffi::c_int,
            );
            return false;
        } else {
            next = (*next).focus_head.tqh_first as *mut Con;
        }
    }
    con_activate(next);
    true
}
unsafe fn mark_unmapped(con: *mut Con) {
    let mut current: *mut Con = ::core::ptr::null_mut::<Con>();
    (*con).mapped = false;
    current = (*con).nodes_head.tqh_first as *mut Con;
    while !current.is_null() {
        mark_unmapped(current);
        current = (*current).nodes.tqe_next as *mut Con;
    }
    if (*con).type_0 as ::core::ffi::c_uint
        == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        for id in identity(con).into_iter().flat_map(floating::children) {
            if let Some(current) = resolve(id) {
                mark_unmapped(current);
            }
        }
    }
}

pub unsafe fn tree_render() {
    if croot.is_null() {
        return;
    }
    debuglog(
        c"%s:%s:%d - -- BEGIN RENDERING --\n".as_ptr().cast_mut(),
        c"../src/tree.c".as_ptr(),
        c"tree_render".as_ptr(),
        457 as ::core::ffi::c_int,
    );
    mark_unmapped(croot);
    (*croot).mapped = true;
    render_con(croot);
    x_push_changes(croot);
    debuglog(
        c"%s:%s:%d - -- END RENDERING --\n".as_ptr().cast_mut(),
        c"../src/tree.c".as_ptr(),
        c"tree_render".as_ptr(),
        466 as ::core::ffi::c_int,
    );
}
unsafe fn get_tree_next_workspace(con: *mut Con, direction: Direction) -> *mut Con {
    if !con_get_fullscreen_con(con, CF_GLOBAL).is_null() {
        debuglog(
            c"%s:%s:%d - Cannot change workspace while in global fullscreen mode.\n"
                .as_ptr()
                .cast_mut(),
            c"../src/tree.c".as_ptr(),
            c"get_tree_next_workspace".as_ptr(),
            471 as ::core::ffi::c_int,
        );
        return ::core::ptr::null_mut::<Con>();
    }
    let x: u32 = (*con)
        .rect
        .x
        .wrapping_add((*con).rect.width.wrapping_div(2 as u32));
    let y: u32 = (*con)
        .rect
        .y
        .wrapping_add((*con).rect.height.wrapping_div(2 as u32));
    let current_output: *mut Output =
        get_output_containing(x as ::core::ffi::c_uint, y as ::core::ffi::c_uint);
    if current_output.is_null() {
        return ::core::ptr::null_mut::<Con>();
    }
    debuglog(
        c"%s:%s:%d - Current output is %s\n".as_ptr().cast_mut(),
        c"../src/tree.c".as_ptr(),
        c"get_tree_next_workspace".as_ptr(),
        483 as ::core::ffi::c_int,
        output_primary_name(current_output),
    );
    let next_output: *mut Output =
        get_output_next(direction, current_output, OutputSelection::Closest);
    if next_output.is_null() {
        return ::core::ptr::null_mut::<Con>();
    }
    debuglog(
        c"%s:%s:%d - Next output is %s\n".as_ptr().cast_mut(),
        c"../src/tree.c".as_ptr(),
        c"get_tree_next_workspace".as_ptr(),
        489 as ::core::ffi::c_int,
        output_primary_name(next_output),
    );
    let mut child = (*output_get_content((*next_output).con))
        .nodes_head
        .tqh_first;
    while !child.is_null() {
        if workspace_is_visible(child) {
            return child;
        }
        child = (*child).nodes.tqe_next;
    }
    std::ptr::null_mut()
}

unsafe fn get_tree_next(mut con: *mut Con, direction: Direction) -> *mut Con {
    let previous: bool = position_from_direction(direction) == Position::Before;
    let orientation: Orientation = orientation_from_direction(direction);
    let mut first_wrap: *mut Con = ::core::ptr::null_mut::<Con>();
    if !((*con).type_0 as ::core::ffi::c_uint
        == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint)
    {
        while (*con).type_0 as ::core::ffi::c_uint
            != CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            if (*con).fullscreen_mode as ::core::ffi::c_uint
                == CF_OUTPUT as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                con = con_get_workspace(con);
                break;
            } else {
                if (*con).fullscreen_mode as ::core::ffi::c_uint
                    == CF_GLOBAL as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    return first_wrap;
                }
                let parent: *mut Con = parent_ptr(con) as *mut Con;
                if (*con).type_0 as ::core::ffi::c_uint
                    == CT_FLOATING_CON as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    if orientation != Orientation::Horizontal {
                        return ::core::ptr::null_mut::<Con>();
                    }
                    let parent_id = identity(parent).expect("live floating parent");
                    let child_id = identity(con).expect("live floating child");
                    let next = if previous {
                        floating::previous(parent_id, child_id)
                            .or_else(|| floating::last(parent_id))
                    } else {
                        floating::next(parent_id, child_id).or_else(|| floating::first(parent_id))
                    }
                    .and_then(resolve)
                    .unwrap_or(std::ptr::null_mut());
                    if !next.is_null() {
                    } else {
                        __assert_fail(
                            c"next".as_ptr(),
                            c"../src/tree.c".as_ptr(),
                            543 as ::core::ffi::c_uint,
                            c"Con *get_tree_next(Con *, direction_t)".as_ptr(),
                        );
                    };
                    return next;
                }
                if con_num_children(parent) > 1 as ::core::ffi::c_int
                    && con_orientation(parent) == orientation
                {
                    let next_0: *mut Con = if previous as ::core::ffi::c_int != 0 {
                        *(*((*con).nodes.tqe_prev as *mut nodes_head)).tqh_last
                    } else {
                        (*con).nodes.tqe_next as *mut Con
                    };
                    if !next_0.is_null()
                        && con_fullscreen_permits_focusing(next_0) as ::core::ffi::c_int != 0
                    {
                        return next_0;
                    }
                    let wrap: *mut Con = if previous as ::core::ffi::c_int != 0 {
                        *(*((*parent).nodes_head.tqh_last as *mut nodes_head)).tqh_last
                    } else {
                        (*parent).nodes_head.tqh_first as *mut Con
                    };
                    match config.focus_wrapping {
                        FocusWrapping::Workspace | FocusWrapping::On => {
                            if first_wrap.is_null()
                                && con_fullscreen_permits_focusing(wrap) as ::core::ffi::c_int != 0
                            {
                                first_wrap = wrap;
                            }
                        }
                        FocusWrapping::Force => {
                            if !next_0.is_null() {
                                return ::core::ptr::null_mut::<Con>();
                            }
                            return if con_fullscreen_permits_focusing(wrap) as ::core::ffi::c_int
                                != 0
                            {
                                wrap
                            } else {
                                ::core::ptr::null_mut::<Con>()
                            };
                        }
                        FocusWrapping::Off => {}
                    }
                }
                con = parent;
            }
        }
        if (*con).type_0 as ::core::ffi::c_uint
            == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
        {
        } else {
            __assert_fail(
                c"con->type == CT_WORKSPACE".as_ptr(),
                c"../src/tree.c".as_ptr(),
                579 as ::core::ffi::c_uint,
                c"Con *get_tree_next(Con *, direction_t)".as_ptr(),
            );
        };
        if config.focus_wrapping == FocusWrapping::Workspace {
            return first_wrap;
        }
    }
    let workspace: *mut Con = get_tree_next_workspace(con, direction);
    if !workspace.is_null() {
        workspace
    } else {
        first_wrap
    }
}

pub unsafe fn tree_next(con: *mut Con, direction: Direction) {
    let next: *mut Con = get_tree_next(con, direction);
    if next.is_null() {
        return;
    }
    if (*next).type_0 as ::core::ffi::c_uint
        == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut focus: *mut Con = con_descend_focused(next);
        if (*focus).fullscreen_mode as ::core::ffi::c_uint
            == CF_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            let focus_tiling: *mut Con = con_descend_tiling_focused(next);
            if focus_tiling != next {
                focus = focus_tiling;
            }
        }
        workspace_show(next);
        con_activate(focus);
        x_set_warp_to(&raw mut (*focus).rect);
        return;
    } else if (*next).type_0 as ::core::ffi::c_uint
        == CT_FLOATING_CON as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let parent: *mut Con = parent_ptr(next) as *mut Con;
        floating::rotate_after(
            identity(parent).expect("live floating parent"),
            identity(next).expect("live floating child"),
        )
        .expect("attached floating child");
    }
    workspace_show(con_get_workspace(next));
    con_activate(con_descend_focused(next));
}

pub unsafe fn get_tree_next_sibling(con: *mut Con, direction: Position) -> *mut Con {
    let to_focus: *mut Con = if direction == Position::Before {
        *(*((*con).nodes.tqe_prev as *mut nodes_head)).tqh_last
    } else {
        (*con).nodes.tqe_next as *mut Con
    };
    if !to_focus.is_null() && con_fullscreen_permits_focusing(to_focus) as ::core::ffi::c_int != 0 {
        return to_focus;
    }
    ::core::ptr::null_mut::<Con>()
}

pub unsafe fn tree_flatten(con: *mut Con) {
    let mut focus_next: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut current: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut child: *mut Con = ::core::ptr::null_mut::<Con>();
    let parent: *mut Con = parent_ptr(con) as *mut Con;
    debuglog(
        c"%s:%s:%d - Checking if I can flatten con = %p / %s\n"
            .as_ptr()
            .cast_mut(),
        c"../src/tree.c".as_ptr(),
        c"tree_flatten".as_ptr(),
        659 as ::core::ffi::c_int,
        con,
        (*con).name,
    );
    if !((*con).type_0 as ::core::ffi::c_uint
        != CT_CON as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*parent).layout == Layout::Output
        || !(*con).window.is_null())
    {
        child = (*con).nodes_head.tqh_first as *mut Con;
        if !(child.is_null() || !(*child).nodes.tqe_next.is_null()) {
            debuglog(
                c"%s:%s:%d - child = %p, con = %p, parent = %p\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/tree.c".as_ptr(),
                c"tree_flatten".as_ptr(),
                674 as ::core::ffi::c_int,
                child,
                con,
                parent,
            );
            if con_is_split(con)
                && con_is_split(child)
                && flatten_layouts((*con).layout, (*child).layout, con_orientation(parent))
            {
                debuglog(
                    c"%s:%s:%d - Alright, I have to flatten this situation now. Stay calm.\n"
                        .as_ptr()
                        .cast_mut(),
                    c"../src/tree.c".as_ptr(),
                    c"tree_flatten".as_ptr(),
                    687 as ::core::ffi::c_int,
                );
                focus_next = (*child).focus_head.tqh_first as *mut Con;
                debuglog(
                    c"%s:%s:%d - detaching...\n".as_ptr().cast_mut(),
                    c"../src/tree.c".as_ptr(),
                    c"tree_flatten".as_ptr(),
                    691 as ::core::ffi::c_int,
                );
                while !(*child).nodes_head.tqh_first.is_null() {
                    current = (*child).nodes_head.tqh_first as *mut Con;
                    debuglog(
                        c"%s:%s:%d - detaching current=%p / %s\n"
                            .as_ptr()
                            .cast_mut(),
                        c"../src/tree.c".as_ptr(),
                        c"tree_flatten".as_ptr(),
                        695 as ::core::ffi::c_int,
                        current,
                        (*current).name,
                    );
                    con_detach(current);
                    debuglog(
                        c"%s:%s:%d - re-attaching\n".as_ptr().cast_mut(),
                        c"../src/tree.c".as_ptr(),
                        c"tree_flatten".as_ptr(),
                        697 as ::core::ffi::c_int,
                    );
                    set_parent_ptr(current, parent as *mut Con).expect("live acyclic ancestry");
                    (*current).nodes.tqe_prev = (*con).nodes.tqe_prev;
                    (*current).nodes.tqe_next = con as *mut Con;
                    *(*con).nodes.tqe_prev = current as *mut Con;
                    (*con).nodes.tqe_prev = &raw mut (*current).nodes.tqe_next;
                    debuglog(
                        c"%s:%s:%d - attaching to focus list\n".as_ptr().cast_mut(),
                        c"../src/tree.c".as_ptr(),
                        c"tree_flatten".as_ptr(),
                        704 as ::core::ffi::c_int,
                    );
                    (*current).focused.tqe_next = ::core::ptr::null_mut::<Con>();
                    (*current).focused.tqe_prev = (*parent).focus_head.tqh_last;
                    *(*parent).focus_head.tqh_last = current as *mut Con;
                    (*parent).focus_head.tqh_last = &raw mut (*current).focused.tqe_next;
                    (*current).percent = (*con).percent;
                }
                debuglog(
                    c"%s:%s:%d - re-attached all\n".as_ptr().cast_mut(),
                    c"../src/tree.c".as_ptr(),
                    c"tree_flatten".as_ptr(),
                    708 as ::core::ffi::c_int,
                );
                if !focus_next.is_null() && (*parent).focus_head.tqh_first == con {
                    debuglog(
                        c"%s:%s:%d - restoring focus to focus_next=%p\n"
                            .as_ptr()
                            .cast_mut(),
                        c"../src/tree.c".as_ptr(),
                        c"tree_flatten".as_ptr(),
                        713 as ::core::ffi::c_int,
                        focus_next,
                    );
                    if !(*focus_next).focused.tqe_next.is_null() {
                        (*(*focus_next).focused.tqe_next).focused.tqe_prev =
                            (*focus_next).focused.tqe_prev;
                    } else {
                        (*parent).focus_head.tqh_last = (*focus_next).focused.tqe_prev;
                    }
                    *(*focus_next).focused.tqe_prev = (*focus_next).focused.tqe_next;
                    (*focus_next).focused.tqe_next = (*parent).focus_head.tqh_first;
                    if !(*focus_next).focused.tqe_next.is_null() {
                        (*(*parent).focus_head.tqh_first).focused.tqe_prev =
                            &raw mut (*focus_next).focused.tqe_next;
                    } else {
                        (*parent).focus_head.tqh_last = &raw mut (*focus_next).focused.tqe_next;
                    }
                    (*parent).focus_head.tqh_first = focus_next as *mut Con;
                    (*focus_next).focused.tqe_prev = &raw mut (*parent).focus_head.tqh_first;
                    debuglog(
                        c"%s:%s:%d - restored focus.\n".as_ptr().cast_mut(),
                        c"../src/tree.c".as_ptr(),
                        c"tree_flatten".as_ptr(),
                        716 as ::core::ffi::c_int,
                    );
                }
                debuglog(
                    c"%s:%s:%d - closing redundant cons\n".as_ptr().cast_mut(),
                    c"../src/tree.c".as_ptr(),
                    c"tree_flatten".as_ptr(),
                    720 as ::core::ffi::c_int,
                );
                tree_close_internal(con, KillWindow::DontKill, true);
                return;
            }
        }
    }
    current = (*con).nodes_head.tqh_first as *mut Con;
    while !current.is_null() {
        let next: *mut Con = (*current).nodes.tqe_next as *mut Con;
        tree_flatten(current);
        current = next;
    }
    if let Some(parent_id) = identity(con) {
        let mut current = floating::first(parent_id);
        while let Some(id) = current {
            // This traversal intentionally captures the successor before
            // recursion can flatten/delete/reparent the current container.
            let next = floating::next(parent_id, id);
            if let Some(con) = resolve(id) {
                tree_flatten(con);
            }
            current = next;
        }
    }
}

// Internal helpers use Rust linkage rather than foreign symbol lookup.
use crate::ewmh::ewmh_update_desktop_properties;
use crate::util::orientation_from_direction;
use crate::util::position_from_direction;
use crate::util::slurp;
use crate::x::x_window_kill;
use i3_common::path_exists::path_exists;
use i3_common::safewrappers::sstrdup;

// Canonical Cairo backend ABI; renderer logic remains local.

// Canonical Pango backend ABI; font policy remains local.

use xcb_sys::{
    xcb_change_property, xcb_change_save_set, xcb_change_window_attributes,
    xcb_get_geometry_reply_t, xcb_reparent_window, xcb_shape_select_input, xcb_unmap_window,
    xcb_void_cookie_t, XCB_CW_EVENT_MASK, XCB_ICCCM_WM_STATE_WITHDRAWN, XCB_NONE,
    XCB_PROP_MODE_REPLACE, XCB_SET_MODE_DELETE,
};

pub use libc::{free, strcmp};

use i3_common::native_stdio::__assert_fail;

use crate::log::{debuglog, errorlog, verboselog};

// Shared model types retain field layout while using one Rust identity.
pub use crate::types::nodes_head;
pub use crate::types::xoutput;
pub use crate::types::Con;
pub use crate::types::Rect;
pub use crate::types::Window;

// Internal calls and globals use canonical Rust identities.
use crate::application::conn;
use crate::application::main_loop;
use crate::application::root;
use crate::application::shape_supported;
use crate::application::A_WM_STATE;
use crate::con::con_activate;
use crate::con::con_attach;
use crate::con::con_descend_focused;
use crate::con::con_descend_tiling_focused;
use crate::con::con_detach;
use crate::con::con_fix_percent;
use crate::con::con_force_split_parents_redraw;
use crate::con::con_free;
use crate::con::con_fullscreen_permits_focusing;
use crate::con::con_get_fullscreen_con;
use crate::con::con_get_workspace;
use crate::con::con_is_floating;
use crate::con::con_is_split;
use crate::con::con_new;
use crate::con::con_next_focused;
use crate::con::con_num_children;
use crate::con::con_orientation;
use crate::con::con_set_urgency;
use crate::con::con_update_parents_urgency;
use crate::config::config;
use crate::ipc::ipc_send_window_event;
use crate::load_layout::tree_append_json;
use crate::output::output_get_content;
use crate::output::output_primary_name;
use crate::randr::get_output_containing;
use crate::randr::get_output_next;
use crate::render::render_con;
use crate::restore_layout::restore_open_placeholder_windows;
use crate::window::window_free;
use crate::workspace::workspace_encapsulate;
use crate::workspace::workspace_is_visible;
use crate::workspace::workspace_show;
use crate::workspace::workspace_update_urgent_flag;
use crate::x::x_con_kill;
use crate::x::x_push_changes;
use crate::x::x_set_name;
use crate::x::x_set_warp_to;
use i3_common::resolve_tilde::resolve_tilde;

#[cfg(test)]
mod owned_policy_tests {
    use super::*;
    #[test]
    fn restore_geometry_keeps_native_signed_coordinate_wrapping() {
        let rect = geometry_rect(-1, -32768, 65535, 0);
        assert_eq!(
            (rect.x, rect.y, rect.width, rect.height),
            (u32::MAX, (-32768i32) as u32, 65535, 0)
        );
    }
    #[test]
    fn flatten_requires_opposite_split_and_matching_parent_orientation() {
        assert!(flatten_layouts(
            Layout::SplitH,
            Layout::SplitV,
            Orientation::Vertical
        ));
        assert!(flatten_layouts(
            Layout::SplitV,
            Layout::SplitH,
            Orientation::Horizontal
        ));
        for con in [
            Layout::SplitH,
            Layout::SplitV,
            Layout::Tabbed,
            Layout::Stacked,
        ] {
            assert!(!flatten_layouts(con, con, Orientation::Horizontal));
            assert!(!flatten_layouts(con, Layout::Tabbed, Orientation::Vertical));
        }
        assert!(!flatten_layouts(
            Layout::SplitH,
            Layout::SplitV,
            Orientation::Horizontal
        ));
    }
}
