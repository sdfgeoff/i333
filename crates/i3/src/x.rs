use crate::types::BorderStyle;
use crate::types::KillWindow;
use crate::types::Layout;
use crate::types::Orientation;
use crate::types::TitleAlign;
use crate::types::Warping;
use crate::xcursor::Cursor;
use i3_common::draw_util::draw_util_text;
use i3_common::safewrappers::sasprintf;
/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * x.c: Interface to X11, transfers our in-memory state to X11 (see also
 *      render.c). Basically a big state machine.
 *
 */
// Mechanical file-by-file Rust translation of src/x.c.
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

pub type Output = xoutput;

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

#[derive(Copy, Clone)]
#[repr(C)]
pub struct con_state {
    pub id: xcb_window_t,
    pub mapped: bool,
    pub unmap_now: bool,
    pub child_mapped: bool,
    pub is_hidden: bool,
    pub is_maximized_vert: bool,
    pub is_maximized_horz: bool,
    pub con: *mut Con,
    pub need_reparent: bool,
    pub old_frame: xcb_window_t,
    pub was_floating: bool,
    pub rect: Rect,
    pub window_rect: Rect,
    pub initial: bool,
    pub name: *mut ::core::ffi::c_char,
    pub state: C2RustUnnamed_27,
    pub old_state: C2RustUnnamed_26,
    pub initial_mapping_order: C2RustUnnamed_25,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_25 {
    pub tqe_next: *mut con_state,
    pub tqe_prev: *mut *mut con_state,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_26 {
    pub cqe_next: *mut con_state,
    pub cqe_prev: *mut con_state,
}
pub use crate::x::C2RustUnnamed_26 as C2RustUnnamed_27;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct initial_mapping_head {
    pub tqh_first: *mut con_state,
    pub tqh_last: *mut *mut con_state,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct old_state_head {
    pub cqh_first: *mut con_state,
    pub cqh_last: *mut con_state,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct state_head {
    pub cqh_first: *mut con_state,
    pub cqh_last: *mut con_state,
}
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();

pub static mut focused_id: xcb_window_t = XCB_NONE as xcb_window_t;
static mut last_focused: xcb_window_t = XCB_NONE as xcb_window_t;
static mut warp_to: *mut Rect = ::core::ptr::null::<Rect>() as *mut Rect;

pub static mut state_head: state_head = unsafe {
    state_head {
        cqh_first: &raw const state_head as *mut state_head as *mut ::core::ffi::c_void
            as *mut con_state,
        cqh_last: &raw const state_head as *mut state_head as *mut ::core::ffi::c_void
            as *mut con_state,
    }
};

pub static mut old_state_head: old_state_head = unsafe {
    old_state_head {
        cqh_first: &raw const old_state_head as *mut old_state_head as *mut ::core::ffi::c_void
            as *mut con_state,
        cqh_last: &raw const old_state_head as *mut old_state_head as *mut ::core::ffi::c_void
            as *mut con_state,
    }
};

pub static mut initial_mapping_head: initial_mapping_head = initial_mapping_head {
    tqh_first: ::core::ptr::null::<con_state>() as *mut con_state,
    tqh_last: ::core::ptr::null::<*mut con_state>() as *mut *mut con_state,
};
unsafe fn state_for_frame(window: xcb_window_t) -> *mut con_state {
    let mut state: *mut con_state = ::core::ptr::null_mut::<con_state>();
    state = state_head.cqh_first as *mut con_state;
    while state != &raw mut state_head as *mut ::core::ffi::c_void as *mut con_state {
        if (*state).id == window {
            return state;
        }
        state = (*state).state.cqe_next as *mut con_state;
    }
    errorlog(
        c"ERROR: No state found for window 0x%08x\n"
            .as_ptr()
            .cast_mut(),
        window,
    );
    if 0 as ::core::ffi::c_int != 0 {
    } else {
        __assert_fail(
            c"false".as_ptr(),
            c"../src/x.c".as_ptr(),
            99 as ::core::ffi::c_uint,
            c"con_state *state_for_frame(xcb_window_t)".as_ptr(),
        );
    };
    ::core::ptr::null_mut::<con_state>()
}
unsafe fn change_ewmh_focus(new_focus: xcb_window_t, old_focus: xcb_window_t) {
    if new_focus == old_focus {
        return;
    }
    ewmh_update_active_window(new_focus);
    if new_focus != XCB_WINDOW_NONE as ::core::ffi::c_int as xcb_window_t {
        ewmh_update_focused(new_focus, true_0 != 0);
    }
    if old_focus != XCB_WINDOW_NONE as ::core::ffi::c_int as xcb_window_t {
        ewmh_update_focused(old_focus, false_0 != 0);
    }
}

pub unsafe fn x_con_init(con: *mut Con) {
    let mut mask: u32 = 0 as u32;
    let mut values: [u32; 5] = [0; 5];
    let visual: xcb_visualid_t = get_visualid_by_depth((*con).depth);
    let mut win_colormap: xcb_colormap_t = 0;
    if (*con).depth as ::core::ffi::c_int != root_depth as ::core::ffi::c_int {
        win_colormap = xcb_generate_id(conn) as xcb_colormap_t;
        xcb_create_colormap(
            conn,
            XCB_COLORMAP_ALLOC_NONE as ::core::ffi::c_int as u8,
            win_colormap,
            root,
            visual,
        );
        (*con).colormap = win_colormap;
    } else {
        win_colormap = colormap;
        (*con).colormap = XCB_NONE as xcb_colormap_t;
    }
    mask |= XCB_CW_BACK_PIXEL as ::core::ffi::c_int as u32;
    values[0 as ::core::ffi::c_int as usize] = (*root_screen).black_pixel;
    mask |= XCB_CW_BORDER_PIXEL as ::core::ffi::c_int as u32;
    values[1 as ::core::ffi::c_int as usize] = (*root_screen).black_pixel;
    mask |= XCB_CW_OVERRIDE_REDIRECT as ::core::ffi::c_int as u32;
    values[2 as ::core::ffi::c_int as usize] = 1 as u32;
    mask |= XCB_CW_EVENT_MASK as ::core::ffi::c_int as u32;
    values[3 as ::core::ffi::c_int as usize] = ((XCB_EVENT_MASK_BUTTON_PRESS as ::core::ffi::c_int
        | XCB_EVENT_MASK_BUTTON_RELEASE as ::core::ffi::c_int
        | XCB_EVENT_MASK_POINTER_MOTION as ::core::ffi::c_int
        | XCB_EVENT_MASK_EXPOSURE as ::core::ffi::c_int
        | XCB_EVENT_MASK_STRUCTURE_NOTIFY as ::core::ffi::c_int
        | XCB_EVENT_MASK_SUBSTRUCTURE_REDIRECT as ::core::ffi::c_int
        | XCB_EVENT_MASK_SUBSTRUCTURE_NOTIFY as ::core::ffi::c_int
        | XCB_EVENT_MASK_ENTER_WINDOW as ::core::ffi::c_int)
        & !(XCB_EVENT_MASK_ENTER_WINDOW as ::core::ffi::c_int))
        as u32;
    mask |= XCB_CW_COLORMAP as ::core::ffi::c_int as u32;
    values[4 as ::core::ffi::c_int as usize] = win_colormap as u32;
    let dims: Rect = Rect {
        x: -(15 as ::core::ffi::c_int) as u32,
        y: -(15 as ::core::ffi::c_int) as u32,
        width: 10 as u32,
        height: 10 as u32,
    };
    let frame_id: xcb_window_t = create_window(
        conn,
        dims,
        (*con).depth,
        visual,
        XCB_WINDOW_CLASS_INPUT_OUTPUT as ::core::ffi::c_int as u16,
        Cursor::Pointer,
        false_0 != 0,
        mask,
        &raw mut values as *mut u32,
    );
    draw_util_surface_init(
        conn,
        &raw mut (*con).frame,
        frame_id as xcb_drawable_t,
        get_visualtype_by_id(visual),
        dims.width as ::core::ffi::c_int,
        dims.height as ::core::ffi::c_int,
    );
    xcb_change_property(
        conn,
        XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as u8,
        (*con).frame.id as xcb_window_t,
        XCB_ATOM_WM_CLASS as ::core::ffi::c_int as xcb_atom_t,
        XCB_ATOM_STRING as ::core::ffi::c_int as xcb_atom_t,
        8 as u8,
        strlen(c"i3-frame".as_ptr())
            .wrapping_add(1 as usize)
            .wrapping_mul(2 as usize) as u32,
        b"i3-frame\0i3-frame\0\0" as *const u8 as *const ::core::ffi::c_char
            as *const ::core::ffi::c_void,
    );
    let state: *mut con_state =
        scalloc(1 as usize, ::core::mem::size_of::<con_state>() as usize) as *mut con_state;
    (*state).id = (*con).frame.id as xcb_window_t;
    (*state).mapped = false_0 != 0;
    (*state).initial = true_0 != 0;
    debuglog(
        c"%s:%s:%d - Adding window 0x%08x to lists\n"
            .as_ptr()
            .cast_mut(),
        c"../src/x.c".as_ptr(),
        c"x_con_init".as_ptr(),
        186 as ::core::ffi::c_int,
        (*state).id,
    );
    (*state).state.cqe_next = state_head.cqh_first;
    (*state).state.cqe_prev = &raw mut state_head as *mut ::core::ffi::c_void as *mut con_state;
    if state_head.cqh_last == &raw mut state_head as *mut ::core::ffi::c_void as *mut con_state {
        state_head.cqh_last = state as *mut con_state;
    } else {
        (*state_head.cqh_first).state.cqe_prev = state as *mut con_state;
    }
    state_head.cqh_first = state as *mut con_state;
    (*state).old_state.cqe_next = old_state_head.cqh_first;
    (*state).old_state.cqe_prev =
        &raw mut old_state_head as *mut ::core::ffi::c_void as *mut con_state;
    if old_state_head.cqh_last
        == &raw mut old_state_head as *mut ::core::ffi::c_void as *mut con_state
    {
        old_state_head.cqh_last = state as *mut con_state;
    } else {
        (*old_state_head.cqh_first).old_state.cqe_prev = state as *mut con_state;
    }
    old_state_head.cqh_first = state as *mut con_state;
    (*state).initial_mapping_order.tqe_next = ::core::ptr::null_mut::<con_state>();
    (*state).initial_mapping_order.tqe_prev = initial_mapping_head.tqh_last;
    *initial_mapping_head.tqh_last = state as *mut con_state;
    initial_mapping_head.tqh_last = &raw mut (*state).initial_mapping_order.tqe_next;
    debuglog(
        c"%s:%s:%d - adding new state for window id 0x%08x\n"
            .as_ptr()
            .cast_mut(),
        c"../src/x.c".as_ptr(),
        c"x_con_init".as_ptr(),
        190 as ::core::ffi::c_int,
        (*state).id,
    );
}

pub unsafe fn x_reinit(con: *mut Con) {
    let mut state: *mut con_state = ::core::ptr::null_mut::<con_state>();
    state = state_for_frame((*con).frame.id as xcb_window_t);
    if state.is_null() {
        errorlog(c"ERROR: window state not found\n".as_ptr().cast_mut());
        return;
    }
    debuglog(
        c"%s:%s:%d - resetting state %p to initial\n"
            .as_ptr()
            .cast_mut(),
        c"../src/x.c".as_ptr(),
        c"x_reinit".as_ptr(),
        207 as ::core::ffi::c_int,
        state,
    );
    (*state).initial = true_0 != 0;
    (*state).child_mapped = false_0 != 0;
    (*state).con = con;
    memset(
        &raw mut (*state).window_rect as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<Rect>() as usize,
    );
}

pub unsafe fn x_reparent_child(con: *mut Con, old: *mut Con) {
    let mut state: *mut con_state = ::core::ptr::null_mut::<con_state>();
    state = state_for_frame((*con).frame.id as xcb_window_t);
    if state.is_null() {
        errorlog(
            c"ERROR: window state for con not found\n"
                .as_ptr()
                .cast_mut(),
        );
        return;
    }
    (*state).need_reparent = true_0 != 0;
    (*state).old_frame = (*old).frame.id as xcb_window_t;
}

pub unsafe fn x_move_win(src: *mut Con, dest: *mut Con) {
    let mut state_src: *mut con_state = ::core::ptr::null_mut::<con_state>();
    let mut state_dest: *mut con_state = ::core::ptr::null_mut::<con_state>();
    state_src = state_for_frame((*src).frame.id as xcb_window_t);
    if state_src.is_null() {
        errorlog(
            c"ERROR: window state for src not found\n"
                .as_ptr()
                .cast_mut(),
        );
        return;
    }
    state_dest = state_for_frame((*dest).frame.id as xcb_window_t);
    if state_dest.is_null() {
        errorlog(
            c"ERROR: window state for dest not found\n"
                .as_ptr()
                .cast_mut(),
        );
        return;
    }
    (*state_dest).con = (*state_src).con;
    (*state_src).con = ::core::ptr::null_mut::<Con>();
    if rect_equals(
        (*state_dest).window_rect,
        Rect {
            x: 0 as u32,
            y: 0 as u32,
            width: 0 as u32,
            height: 0 as u32,
        },
    ) {
        memcpy(
            &raw mut (*state_dest).window_rect as *mut ::core::ffi::c_void,
            &raw mut (*state_src).window_rect as *const ::core::ffi::c_void,
            ::core::mem::size_of::<Rect>() as usize,
        );
        debuglog(
            c"%s:%s:%d - COPYING RECT\n".as_ptr().cast_mut(),
            c"../src/x.c".as_ptr(),
            c"x_move_win".as_ptr(),
            252 as ::core::ffi::c_int,
        );
    }
}
unsafe fn _x_con_kill(con: *mut Con) {
    if (*con).colormap as ::core::ffi::c_long != XCB_NONE {
        xcb_free_colormap(conn, (*con).colormap);
    }
    draw_util_surface_free(conn, &raw mut (*con).frame);
    draw_util_surface_free(conn, &raw mut (*con).frame_buffer);
    xcb_free_pixmap(conn, (*con).frame_buffer.id as xcb_pixmap_t);
    (*con).frame_buffer.id = XCB_NONE as xcb_drawable_t;
    let state: *mut con_state = state_for_frame((*con).frame.id as xcb_window_t);
    if (*state).state.cqe_next == &raw mut state_head as *mut ::core::ffi::c_void as *mut con_state
    {
        state_head.cqh_last = (*state).state.cqe_prev;
    } else {
        (*(*state).state.cqe_next).state.cqe_prev = (*state).state.cqe_prev;
    }
    if (*state).state.cqe_prev == &raw mut state_head as *mut ::core::ffi::c_void as *mut con_state
    {
        state_head.cqh_first = (*state).state.cqe_next;
    } else {
        (*(*state).state.cqe_prev).state.cqe_next = (*state).state.cqe_next;
    }
    if (*state).old_state.cqe_next
        == &raw mut old_state_head as *mut ::core::ffi::c_void as *mut con_state
    {
        old_state_head.cqh_last = (*state).old_state.cqe_prev;
    } else {
        (*(*state).old_state.cqe_next).old_state.cqe_prev = (*state).old_state.cqe_prev;
    }
    if (*state).old_state.cqe_prev
        == &raw mut old_state_head as *mut ::core::ffi::c_void as *mut con_state
    {
        old_state_head.cqh_first = (*state).old_state.cqe_next;
    } else {
        (*(*state).old_state.cqe_prev).old_state.cqe_next = (*state).old_state.cqe_next;
    }
    if !(*state).initial_mapping_order.tqe_next.is_null() {
        (*(*state).initial_mapping_order.tqe_next)
            .initial_mapping_order
            .tqe_prev = (*state).initial_mapping_order.tqe_prev;
    } else {
        initial_mapping_head.tqh_last = (*state).initial_mapping_order.tqe_prev;
    }
    *(*state).initial_mapping_order.tqe_prev = (*state).initial_mapping_order.tqe_next;
    free((*state).name as *mut ::core::ffi::c_void);
    (*state).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free(state as *mut ::core::ffi::c_void);
    if (*con).frame.id == focused_id {
        focused_id = XCB_NONE as xcb_window_t;
    }
    if (*con).frame.id == last_focused {
        last_focused = XCB_NONE as xcb_window_t;
    }
}

pub unsafe fn x_con_kill(con: *mut Con) {
    _x_con_kill(con);
    xcb_destroy_window(conn, (*con).frame.id as xcb_window_t);
}

pub unsafe fn x_con_reframe(con: *mut Con) {
    _x_con_kill(con);
    x_con_init(con);
}

pub unsafe fn window_supports_protocol(window: xcb_window_t, atom: xcb_atom_t) -> bool {
    let mut protocols: xcb_icccm_get_wm_protocols_reply_t = xcb_icccm_get_wm_protocols_reply_t {
        atoms_len: 0,
        atoms: ::core::ptr::null_mut::<xcb_atom_t>(),
        _reply: ::core::ptr::null_mut::<xcb_get_property_reply_t>(),
    };
    let mut result: bool = false_0 != 0;
    let cookie: xcb_get_property_cookie_t =
        xcb_icccm_get_wm_protocols(conn, window, A_WM_PROTOCOLS);
    if xcb_icccm_get_wm_protocols_reply(
        conn,
        cookie,
        &raw mut protocols,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    ) as ::core::ffi::c_int
        != 1 as ::core::ffi::c_int
    {
        return false_0 != 0;
    }
    let mut i: u32 = 0 as u32;
    while i < protocols.atoms_len {
        if *protocols.atoms.offset(i as isize) == atom {
            result = true_0 != 0;
        }
        i = i.wrapping_add(1);
    }
    xcb_icccm_get_wm_protocols_reply_wipe(&raw mut protocols);
    result
}

pub unsafe fn x_window_kill(window: xcb_window_t, kill_window: KillWindow) {
    if !window_supports_protocol(window, A_WM_DELETE_WINDOW) {
        if kill_window == KillWindow::Window {
            verboselog(
                c"Killing specific window 0x%08x\n".as_ptr().cast_mut(),
                window,
            );
            xcb_destroy_window(conn, window);
        } else {
            verboselog(
                c"Killing the X11 client which owns window 0x%08x\n"
                    .as_ptr()
                    .cast_mut(),
                window,
            );
            xcb_kill_client(conn, window as u32);
        }
        return;
    }
    let event: *mut ::core::ffi::c_void = scalloc(32 as usize, 1 as usize);
    let ev: *mut xcb_client_message_event_t = event as *mut xcb_client_message_event_t;
    (*ev).response_type = XCB_CLIENT_MESSAGE as u8;
    (*ev).window = window;
    (*ev).type_0 = A_WM_PROTOCOLS;
    (*ev).format = 32 as u8;
    (*ev).data.data32[0 as ::core::ffi::c_int as usize] = A_WM_DELETE_WINDOW as u32;
    (*ev).data.data32[1 as ::core::ffi::c_int as usize] = XCB_CURRENT_TIME as u32;
    verboselog(c"Sending WM_DELETE to the client\n".as_ptr().cast_mut());
    xcb_send_event(
        conn,
        false_0 as u8,
        window,
        XCB_EVENT_MASK_NO_EVENT as ::core::ffi::c_int as u32,
        ev as *mut ::core::ffi::c_char,
    );
    xcb_flush(conn);
    free(event);
}
unsafe fn x_draw_title_border(
    con: *mut Con,
    p: *mut deco_render_params,
    dest_surface: *mut surface_t,
) {
    let dr: *mut Rect = &raw mut (*con).deco_rect;
    draw_util_rectangle(
        dest_surface,
        (*(*p).color).border,
        (*dr).x as ::core::ffi::c_double,
        (*dr).y as ::core::ffi::c_double,
        1 as ::core::ffi::c_int as ::core::ffi::c_double,
        (*dr).height as ::core::ffi::c_double,
    );
    draw_util_rectangle(
        dest_surface,
        (*(*p).color).border,
        (*dr).x.wrapping_add((*dr).width).wrapping_sub(1 as u32) as ::core::ffi::c_double,
        (*dr).y as ::core::ffi::c_double,
        1 as ::core::ffi::c_int as ::core::ffi::c_double,
        (*dr).height as ::core::ffi::c_double,
    );
    draw_util_rectangle(
        dest_surface,
        (*(*p).color).border,
        (*dr).x as ::core::ffi::c_double,
        (*dr).y as ::core::ffi::c_double,
        (*dr).width as ::core::ffi::c_double,
        1 as ::core::ffi::c_int as ::core::ffi::c_double,
    );
    draw_util_rectangle(
        dest_surface,
        (*(*p).color).border,
        (*dr).x as ::core::ffi::c_double,
        (*dr).y.wrapping_add((*dr).height).wrapping_sub(1 as u32) as ::core::ffi::c_double,
        (*dr).width as ::core::ffi::c_double,
        1 as ::core::ffi::c_int as ::core::ffi::c_double,
    );
}
unsafe fn x_draw_decoration_after_title(
    con: *mut Con,
    p: *mut deco_render_params,
    dest_surface: *mut surface_t,
) {
    if (*con).parent != ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut Con {
    } else {
        __assert_fail(
            c"con->parent != NULL".as_ptr(),
            c"../src/x.c".as_ptr(),
            381 as ::core::ffi::c_uint,
            c"void x_draw_decoration_after_title(Con *, struct deco_render_params *, surface_t *)"
                .as_ptr(),
        );
    };
    let dr: *mut Rect = &raw mut (*con).deco_rect;
    if !font_is_pango() {
        draw_util_rectangle(
            dest_surface,
            (*(*p).color).background,
            (*dr).x.wrapping_add((*dr).width).wrapping_sub(
                (2 as ::core::ffi::c_int * logical_px(1 as ::core::ffi::c_int)) as u32,
            ) as ::core::ffi::c_double,
            (*dr).y as ::core::ffi::c_double,
            (2 as ::core::ffi::c_int * logical_px(1 as ::core::ffi::c_int))
                as ::core::ffi::c_double,
            (*dr).height as ::core::ffi::c_double,
        );
    }
    x_draw_title_border(con, p, dest_surface);
}
unsafe fn x_get_border_rectangles(con: *mut Con, rectangles: *mut xcb_rectangle_t) -> usize {
    let mut count: usize = 0 as usize;
    let border_style: BorderStyle = con_border_style(con);
    if border_style != BorderStyle::None && con_is_leaf(con) as ::core::ffi::c_int != 0 {
        let borders_to_hide: adjacent_t = (con_adjacent_borders(con) as ::core::ffi::c_uint
            & config.hide_edge_borders as ::core::ffi::c_uint)
            as adjacent_t;
        let br: Rect = con_border_style_rect(con);
        if borders_to_hide as ::core::ffi::c_uint
            & ADJ_LEFT_SCREEN_EDGE as ::core::ffi::c_int as ::core::ffi::c_uint
            == 0
        {
            let fresh0 = count;
            count = count.wrapping_add(1);
            *rectangles.offset(fresh0 as isize) = xcb_rectangle_t {
                x: 0 as i16,
                y: 0 as i16,
                width: br.x as u16,
                height: (*con).rect.height as u16,
            };
        }
        if borders_to_hide as ::core::ffi::c_uint
            & ADJ_RIGHT_SCREEN_EDGE as ::core::ffi::c_int as ::core::ffi::c_uint
            == 0
        {
            let fresh1 = count;
            count = count.wrapping_add(1);
            *rectangles.offset(fresh1 as isize) = xcb_rectangle_t {
                x: (*con).rect.width.wrapping_add(br.width.wrapping_add(br.x)) as i16,
                y: 0 as i16,
                width: br.width.wrapping_add(br.x).wrapping_neg() as u16,
                height: (*con).rect.height as u16,
            };
        }
        if borders_to_hide as ::core::ffi::c_uint
            & ADJ_LOWER_SCREEN_EDGE as ::core::ffi::c_int as ::core::ffi::c_uint
            == 0
        {
            let fresh2 = count;
            count = count.wrapping_add(1);
            *rectangles.offset(fresh2 as isize) = xcb_rectangle_t {
                x: br.x as i16,
                y: (*con)
                    .rect
                    .height
                    .wrapping_add(br.height.wrapping_add(br.y)) as i16,
                width: (*con).rect.width.wrapping_add(br.width) as u16,
                height: br.height.wrapping_add(br.y).wrapping_neg() as u16,
            };
        }
        if border_style == BorderStyle::Pixel
            && borders_to_hide as ::core::ffi::c_uint
                & ADJ_UPPER_SCREEN_EDGE as ::core::ffi::c_int as ::core::ffi::c_uint
                == 0
        {
            let fresh3 = count;
            count = count.wrapping_add(1);
            *rectangles.offset(fresh3 as isize) = xcb_rectangle_t {
                x: br.x as i16,
                y: 0 as i16,
                width: (*con).rect.width.wrapping_add(br.width) as u16,
                height: br.y as u16,
            };
        }
    }
    count
}

pub unsafe fn x_draw_decoration(con: *mut Con) {
    let mut next: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut dest_surface: *mut surface_t = ::core::ptr::null_mut::<surface_t>();
    let mut text_offset_y: ::core::ffi::c_int = 0;
    let mut win: *mut Window = ::core::ptr::null_mut::<Window>();
    let mut deco_width: ::core::ffi::c_int = 0;
    let mut title_padding: ::core::ffi::c_int = 0;
    let mut mark_width: ::core::ffi::c_int = 0;
    let mut title: *mut i3String = ::core::ptr::null_mut::<i3String>();
    let mut icon_size: ::core::ffi::c_int = 0;
    let mut icon_padding: ::core::ffi::c_int = 0;
    let mut total_icon_space: ::core::ffi::c_int = 0;
    let mut has_icon: bool = false;
    let mut icon_offset_x: ::core::ffi::c_int = 0;
    let mut title_offset_x: ::core::ffi::c_int = 0;
    let parent: *mut Con = (*con).parent as *mut Con;
    let leaf: bool = con_is_leaf(con);
    if !leaf && (*parent).layout != Layout::Stacked && (*parent).layout != Layout::Tabbed
        || (*parent).type_0 as ::core::ffi::c_uint
            == CT_OUTPUT as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*parent).type_0 as ::core::ffi::c_uint
            == CT_DOCKAREA as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*con).type_0 as ::core::ffi::c_uint
            == CT_FLOATING_CON as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return;
    }
    if (*con).rect.height == 0 as u32 {
        return;
    }
    if leaf as ::core::ffi::c_int != 0 && (*con).frame_buffer.id as ::core::ffi::c_long == XCB_NONE
    {
        return;
    }
    let p: *mut deco_render_params = scalloc(
        1 as usize,
        ::core::mem::size_of::<deco_render_params>() as usize,
    ) as *mut deco_render_params;
    if (*con).urgent {
        (*p).color = &raw mut config.client.urgent as *mut Colortriple;
    } else if con == focused || con_inside_focused(con) as ::core::ffi::c_int != 0 {
        (*p).color = &raw mut config.client.focused as *mut Colortriple;
    } else if con == (*parent).focus_head.tqh_first {
        if config.client.got_focused_tab_title as ::core::ffi::c_int != 0
            && !leaf
            && con_descend_focused(con) == focused
        {
            (*p).color = &raw mut config.client.focused_tab_title as *mut Colortriple;
        } else {
            (*p).color = &raw mut config.client.focused_inactive as *mut Colortriple;
        }
    } else {
        (*p).color = &raw mut config.client.unfocused as *mut Colortriple;
    }
    (*p).border_style = con_border_style(con);
    let r: *mut Rect = &raw mut (*con).rect;
    let w: *mut Rect = &raw mut (*con).window_rect;
    (*p).con_rect = width_height {
        w: (*r).width,
        h: (*r).height,
    };
    (*p).con_window_rect = width_height {
        w: (*w).width,
        h: (*w).height,
    };
    (*p).con_deco_rect = (*con).deco_rect as Rect;
    (*p).background = config.client.background;
    (*p).con_is_leaf = con_is_leaf(con);
    (*p).parent_layout = (*(*con).parent).layout;
    if !(*con).deco_render_params.is_null()
        && ((*con).window.is_null() || !(*(*con).window).name_x_changed)
        && !(*parent).pixmap_recreated
        && !(*con).pixmap_recreated
        && !(*con).mark_changed
        && memcmp(
            p as *const ::core::ffi::c_void,
            (*con).deco_render_params as *const ::core::ffi::c_void,
            ::core::mem::size_of::<deco_render_params>() as usize,
        ) == 0 as ::core::ffi::c_int
    {
        free(p as *mut ::core::ffi::c_void);
    } else {
        next = con;
        loop {
            next = (*next).nodes.tqe_next as *mut Con;
            if next.is_null() {
                break;
            }
            free((*next).deco_render_params as *mut ::core::ffi::c_void);
            (*next).deco_render_params = ::core::ptr::null_mut::<deco_render_params>();
        }
        free((*con).deco_render_params as *mut ::core::ffi::c_void);
        (*con).deco_render_params = ::core::ptr::null_mut::<deco_render_params>();
        (*con).deco_render_params = p;
        if !(*con).window.is_null() && (*(*con).window).name_x_changed as ::core::ffi::c_int != 0 {
            (*(*con).window).name_x_changed = false_0 != 0;
        }
        (*parent).pixmap_recreated = false_0 != 0;
        (*con).pixmap_recreated = false_0 != 0;
        (*con).mark_changed = false_0 != 0;
        if !(*con).window.is_null() {
            draw_util_clear_surface(
                &raw mut (*con).frame_buffer,
                color_t {
                    red: 0.0f64,
                    green: 0.0f64,
                    blue: 0.0f64,
                    alpha: 0.,
                    colorpixel: 0,
                },
            );
            draw_util_rectangle(
                &raw mut (*con).frame_buffer,
                config.client.background,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
                (*r).width as ::core::ffi::c_double,
                (*w).y as ::core::ffi::c_double,
            );
            draw_util_rectangle(
                &raw mut (*con).frame_buffer,
                config.client.background,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
                (*w).y.wrapping_add((*w).height) as ::core::ffi::c_double,
                (*r).width as ::core::ffi::c_double,
                (*r).height.wrapping_sub((*w).y.wrapping_add((*w).height)) as ::core::ffi::c_double,
            );
            draw_util_rectangle(
                &raw mut (*con).frame_buffer,
                config.client.background,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
                (*w).x as ::core::ffi::c_double,
                (*r).height as ::core::ffi::c_double,
            );
            draw_util_rectangle(
                &raw mut (*con).frame_buffer,
                config.client.background,
                (*w).x.wrapping_add((*w).width) as ::core::ffi::c_double,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
                (*r).width.wrapping_sub((*w).x.wrapping_add((*w).width)) as ::core::ffi::c_double,
                (*r).height as ::core::ffi::c_double,
            );
        }
        if (*p).border_style != BorderStyle::None && (*p).con_is_leaf as ::core::ffi::c_int != 0 {
            let mut rectangles: [xcb_rectangle_t; 4] = [xcb_rectangle_t {
                x: 0,
                y: 0,
                width: 0,
                height: 0,
            }; 4];
            let rectangles_count: usize =
                x_get_border_rectangles(con, &raw mut rectangles as *mut xcb_rectangle_t);
            let mut i: usize = 0 as usize;
            while i < rectangles_count {
                draw_util_rectangle(
                    &raw mut (*con).frame_buffer,
                    (*(*p).color).child_border,
                    rectangles[i as usize].x as ::core::ffi::c_double,
                    rectangles[i as usize].y as ::core::ffi::c_double,
                    rectangles[i as usize].width as ::core::ffi::c_double,
                    rectangles[i as usize].height as ::core::ffi::c_double,
                );
                i = i.wrapping_add(1);
            }
            let br: Rect = con_border_style_rect(con);
            if (*con).nodes.tqe_next.is_null()
                && (*(*((*con).nodes.tqe_prev as *mut nodes_head)).tqh_last).is_null()
                && (*(*con).parent).type_0 as ::core::ffi::c_uint
                    != CT_FLOATING_CON as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                if (*p).parent_layout == Layout::SplitH {
                    draw_util_rectangle(
                        &raw mut (*con).frame_buffer,
                        (*(*p).color).indicator,
                        (*r).width.wrapping_add(br.width.wrapping_add(br.x))
                            as ::core::ffi::c_double,
                        br.y as ::core::ffi::c_double,
                        br.width.wrapping_add(br.x).wrapping_neg() as ::core::ffi::c_double,
                        (*r).height.wrapping_add(br.height) as ::core::ffi::c_double,
                    );
                } else if (*p).parent_layout == Layout::SplitV {
                    draw_util_rectangle(
                        &raw mut (*con).frame_buffer,
                        (*(*p).color).indicator,
                        br.x as ::core::ffi::c_double,
                        (*r).height.wrapping_add(br.height.wrapping_add(br.y))
                            as ::core::ffi::c_double,
                        (*r).width.wrapping_add(br.width) as ::core::ffi::c_double,
                        br.height.wrapping_add(br.y).wrapping_neg() as ::core::ffi::c_double,
                    );
                }
            }
        }
        dest_surface = &raw mut (*parent).frame_buffer;
        if con_draw_decoration_into_frame(con) {
            debuglog(
                c"%s:%s:%d - using con->frame_buffer (for con->name=%s) as dest_surface\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/x.c".as_ptr(),
                c"x_draw_decoration".as_ptr(),
                602 as ::core::ffi::c_int,
                (*con).name,
            );
            dest_surface = &raw mut (*con).frame_buffer;
        } else {
            debuglog(
                c"%s:%s:%d - sticking to parent->frame_buffer = %p\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/x.c".as_ptr(),
                c"x_draw_decoration".as_ptr(),
                605 as ::core::ffi::c_int,
                dest_surface,
            );
        }
        debuglog(
            c"%s:%s:%d - dest_surface %p is %d x %d (id=0x%08x)\n"
                .as_ptr()
                .cast_mut(),
            c"../src/x.c".as_ptr(),
            c"x_draw_decoration".as_ptr(),
            607 as ::core::ffi::c_int,
            dest_surface,
            (*dest_surface).width,
            (*dest_surface).height,
            (*dest_surface).id,
        );
        if !((*dest_surface).id as ::core::ffi::c_long == XCB_NONE) {
            if con == (*(*con).parent).nodes_head.tqh_first {
                free((*(*con).parent).deco_render_params as *mut ::core::ffi::c_void);
                (*(*con).parent).deco_render_params = ::core::ptr::null_mut::<deco_render_params>();
            }
            if !((*p).border_style != BorderStyle::Normal
                || (*con).deco_rect.width == 0 as u32
                || (*con).deco_rect.height == 0 as u32)
            {
                debuglog(
                    c"%s:%s:%d - con->deco_rect = (x=%d, y=%d, w=%d, h=%d) for con->name=%s\n"
                        .as_ptr()
                        .cast_mut(),
                    c"../src/x.c".as_ptr(),
                    c"x_draw_decoration".as_ptr(),
                    630 as ::core::ffi::c_int,
                    (*con).deco_rect.x,
                    (*con).deco_rect.y,
                    (*con).deco_rect.width,
                    (*con).deco_rect.height,
                    (*con).name,
                );
                draw_util_rectangle(
                    dest_surface,
                    (*(*p).color).background,
                    (*con).deco_rect.x as ::core::ffi::c_double,
                    (*con).deco_rect.y as ::core::ffi::c_double,
                    (*con).deco_rect.width as ::core::ffi::c_double,
                    (*con).deco_rect.height as ::core::ffi::c_double,
                );
                x_draw_title_border(con, p, dest_surface);
                text_offset_y = (*con)
                    .deco_rect
                    .height
                    .wrapping_sub(config.font.height as u32)
                    .wrapping_div(2 as u32) as ::core::ffi::c_int;
                win = (*con).window;
                deco_width = (*con).deco_rect.width as ::core::ffi::c_int;
                title_padding = logical_px(2 as ::core::ffi::c_int) as ::core::ffi::c_int;
                mark_width = 0 as ::core::ffi::c_int;
                if config.show_marks as ::core::ffi::c_int != 0
                    && !(*con).marks_head.tqh_first.is_null()
                {
                    let mut formatted_mark: *mut ::core::ffi::c_char = sstrdup(c"".as_ptr());
                    let mut had_visible_mark: bool = false_0 != 0;
                    let mut mark: *mut mark_t = ::core::ptr::null_mut::<mark_t>();
                    mark = (*con).marks_head.tqh_first as *mut mark_t;
                    while !mark.is_null() {
                        if !(*(*mark).name.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            == '_' as i32)
                        {
                            had_visible_mark = true_0 != 0;
                            let mut buf: *mut ::core::ffi::c_char =
                                ::core::ptr::null_mut::<::core::ffi::c_char>();
                            sasprintf(
                                &raw mut buf,
                                c"%s[%s]".as_ptr(),
                                formatted_mark,
                                (*mark).name,
                            );
                            free(formatted_mark as *mut ::core::ffi::c_void);
                            formatted_mark = buf;
                        }
                        mark = (*mark).marks.tqe_next as *mut mark_t;
                    }
                    if had_visible_mark {
                        let mut mark_0: *mut i3String = i3string_from_utf8(formatted_mark);
                        mark_width = predict_text_width(mark_0);
                        let mark_offset_x: ::core::ffi::c_int =
                            if config.title_align == TitleAlign::Right {
                                title_padding
                            } else {
                                deco_width - mark_width - title_padding
                            };
                        draw_util_text(
                            mark_0,
                            dest_surface,
                            (*(*p).color).text,
                            (*(*p).color).background,
                            (*con).deco_rect.x.wrapping_add(mark_offset_x as u32)
                                as ::core::ffi::c_int,
                            (*con).deco_rect.y.wrapping_add(text_offset_y as u32)
                                as ::core::ffi::c_int,
                            mark_width,
                        );
                        i3string_free(mark_0);
                        mark_0 = ::core::ptr::null_mut::<i3String>();
                        mark_width += title_padding;
                    }
                    free(formatted_mark as *mut ::core::ffi::c_void);
                    formatted_mark = ::core::ptr::null_mut::<::core::ffi::c_char>();
                }
                title = ::core::ptr::null_mut::<i3String>();
                if win.is_null() {
                    if (*con).title_format.is_null() {
                        let mut _title: *mut ::core::ffi::c_char =
                            ::core::ptr::null_mut::<::core::ffi::c_char>();
                        let tree: *mut ::core::ffi::c_char = con_get_tree_representation(con);
                        sasprintf(&raw mut _title, c"i3: %s".as_ptr(), tree);
                        free(tree as *mut ::core::ffi::c_void);
                        title = i3string_from_utf8(_title);
                        free(_title as *mut ::core::ffi::c_void);
                        _title = ::core::ptr::null_mut::<::core::ffi::c_char>();
                    } else {
                        title = con_parse_title_format(con);
                    }
                } else {
                    title = if (*con).title_format.is_null() {
                        (*win).name
                    } else {
                        con_parse_title_format(con)
                    };
                }
                if !title.is_null() {
                    icon_size = max(
                        0 as ::core::ffi::c_int,
                        (*con)
                            .deco_rect
                            .height
                            .wrapping_sub(logical_px(2 as ::core::ffi::c_int) as u32)
                            as ::core::ffi::c_int,
                    );
                    icon_padding =
                        logical_px(max(1 as ::core::ffi::c_int, (*con).window_icon_padding));
                    total_icon_space = icon_size + 2 as ::core::ffi::c_int * icon_padding;
                    has_icon = (*con).window_icon_padding > -(1 as ::core::ffi::c_int)
                        && !win.is_null()
                        && !(*win).icon.is_null()
                        && total_icon_space < deco_width;
                    if !has_icon {
                        total_icon_space = 0 as ::core::ffi::c_int;
                        icon_padding = total_icon_space;
                        icon_size = icon_padding;
                    }
                    icon_offset_x = 0;
                    title_offset_x = 0;
                    match config.title_align {
                        TitleAlign::Left => {
                            icon_offset_x = icon_padding;
                            title_offset_x = title_padding + total_icon_space;
                        }
                        TitleAlign::Center => {
                            icon_offset_x = max(
                                icon_padding,
                                (deco_width
                                    - icon_padding
                                    - icon_size
                                    - predict_text_width(title)
                                    - title_padding
                                    - mark_width)
                                    / 2 as ::core::ffi::c_int,
                            );
                            title_offset_x =
                                max(title_padding, icon_offset_x + icon_padding + icon_size);
                        }
                        TitleAlign::Right => {
                            title_offset_x = max(
                                title_padding + mark_width,
                                deco_width
                                    - title_padding
                                    - predict_text_width(title)
                                    - total_icon_space,
                            );
                            icon_offset_x = min(
                                deco_width - icon_size - icon_padding - title_padding,
                                title_offset_x + predict_text_width(title) + icon_padding,
                            );
                        }
                    }
                    draw_util_text(
                        title,
                        dest_surface,
                        (*(*p).color).text,
                        (*(*p).color).background,
                        (*con).deco_rect.x.wrapping_add(title_offset_x as u32)
                            as ::core::ffi::c_int,
                        (*con).deco_rect.y.wrapping_add(text_offset_y as u32) as ::core::ffi::c_int,
                        deco_width
                            - mark_width
                            - 2 as ::core::ffi::c_int * title_padding
                            - total_icon_space,
                    );
                    if has_icon {
                        draw_util_image(
                            (*win).icon,
                            dest_surface,
                            (*con).deco_rect.x.wrapping_add(icon_offset_x as u32)
                                as ::core::ffi::c_int,
                            (*con)
                                .deco_rect
                                .y
                                .wrapping_add(logical_px(1 as ::core::ffi::c_int) as u32)
                                as ::core::ffi::c_int,
                            icon_size,
                            icon_size,
                        );
                    }
                    if win.is_null() || !(*con).title_format.is_null() {
                        i3string_free(title);
                        title = ::core::ptr::null_mut::<i3String>();
                    }
                    x_draw_decoration_after_title(con, p, dest_surface);
                }
            }
        }
    }
    draw_util_copy_surface(
        &raw mut (*con).frame_buffer,
        &raw mut (*con).frame,
        0 as ::core::ffi::c_int as ::core::ffi::c_double,
        0 as ::core::ffi::c_int as ::core::ffi::c_double,
        0 as ::core::ffi::c_int as ::core::ffi::c_double,
        0 as ::core::ffi::c_int as ::core::ffi::c_double,
        (*con).rect.width as ::core::ffi::c_double,
        (*con).rect.height as ::core::ffi::c_double,
    );
}

pub unsafe fn x_deco_recurse(con: *mut Con) {
    let leaf: bool =
        (*con).nodes_head.tqh_first.is_null() && (*con).floating_head.tqh_first.is_null();
    let state: *const con_state = state_for_frame((*con).frame.id as xcb_window_t);
    if !leaf {
        let mut current: *mut Con = ::core::ptr::null_mut::<Con>();
        current = (*con).nodes_head.tqh_first as *mut Con;
        while !current.is_null() {
            x_deco_recurse(current);
            current = (*current).nodes.tqe_next as *mut Con;
        }
        current = (*con).floating_head.tqh_first as *mut Con;
        while !current.is_null() {
            x_deco_recurse(current);
            current = (*current).floating_windows.tqe_next as *mut Con;
        }
        if (*state).mapped {
            draw_util_copy_surface(
                &raw mut (*con).frame_buffer,
                &raw mut (*con).frame,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
                (*con).rect.width as ::core::ffi::c_double,
                (*con).rect.height as ::core::ffi::c_double,
            );
        }
    }
    if (*con).type_0 as ::core::ffi::c_uint != CT_ROOT as ::core::ffi::c_int as ::core::ffi::c_uint
        && (*con).type_0 as ::core::ffi::c_uint
            != CT_OUTPUT as ::core::ffi::c_int as ::core::ffi::c_uint
        && (!leaf || (*con).mapped as ::core::ffi::c_int != 0)
    {
        x_draw_decoration(con);
    }
}
unsafe fn set_hidden_state(con: *mut Con) {
    if (*con).window.is_null() {
        return;
    }
    let state: *mut con_state = state_for_frame((*con).frame.id as xcb_window_t);
    let should_be_hidden: bool = con_is_hidden(con);
    if should_be_hidden as ::core::ffi::c_int == (*state).is_hidden as ::core::ffi::c_int {
        return;
    }
    if should_be_hidden {
        debuglog(
            c"%s:%s:%d - setting _NET_WM_STATE_HIDDEN for con = %p\n"
                .as_ptr()
                .cast_mut(),
            c"../src/x.c".as_ptr(),
            c"set_hidden_state".as_ptr(),
            820 as ::core::ffi::c_int,
            con,
        );
        xcb_add_property_atom(
            conn,
            (*(*con).window).id,
            A__NET_WM_STATE,
            A__NET_WM_STATE_HIDDEN,
        );
    } else {
        debuglog(
            c"%s:%s:%d - removing _NET_WM_STATE_HIDDEN for con = %p\n"
                .as_ptr()
                .cast_mut(),
            c"../src/x.c".as_ptr(),
            c"set_hidden_state".as_ptr(),
            823 as ::core::ffi::c_int,
            con,
        );
        xcb_remove_property_atom(
            conn,
            (*(*con).window).id,
            A__NET_WM_STATE,
            A__NET_WM_STATE_HIDDEN,
        );
    }
    (*state).is_hidden = should_be_hidden;
}
unsafe fn set_maximized_state(con: *mut Con) {
    if (*con).window.is_null() {
        return;
    }
    let state: *mut con_state = state_for_frame((*con).frame.id as xcb_window_t);
    let con_maximized_horz: bool = con_is_maximized(con, Orientation::Horizontal);
    if con_maximized_horz as ::core::ffi::c_int != (*state).is_maximized_horz as ::core::ffi::c_int
    {
        debuglog(
            c"%s:%s:%d - setting _NET_WM_STATE_MAXIMIZED_HORZ for con %p(%s) to %d\n"
                .as_ptr()
                .cast_mut(),
            c"../src/x.c".as_ptr(),
            c"set_maximized_state".as_ptr(),
            843 as ::core::ffi::c_int,
            con,
            (*con).name,
            con_maximized_horz as ::core::ffi::c_int,
        );
        if con_maximized_horz {
            xcb_add_property_atom(
                conn,
                (*(*con).window).id,
                A__NET_WM_STATE,
                A__NET_WM_STATE_MAXIMIZED_HORZ,
            );
        } else {
            xcb_remove_property_atom(
                conn,
                (*(*con).window).id,
                A__NET_WM_STATE,
                A__NET_WM_STATE_MAXIMIZED_HORZ,
            );
        }
        (*state).is_maximized_horz = con_maximized_horz;
    }
    let con_maximized_vert: bool = con_is_maximized(con, Orientation::Vertical);
    if con_maximized_vert as ::core::ffi::c_int != (*state).is_maximized_vert as ::core::ffi::c_int
    {
        debuglog(
            c"%s:%s:%d - setting _NET_WM_STATE_MAXIMIZED_VERT for con %p(%s) to %d\n"
                .as_ptr()
                .cast_mut(),
            c"../src/x.c".as_ptr(),
            c"set_maximized_state".as_ptr(),
            856 as ::core::ffi::c_int,
            con,
            (*con).name,
            con_maximized_vert as ::core::ffi::c_int,
        );
        if con_maximized_vert {
            xcb_add_property_atom(
                conn,
                (*(*con).window).id,
                A__NET_WM_STATE,
                A__NET_WM_STATE_MAXIMIZED_VERT,
            );
        } else {
            xcb_remove_property_atom(
                conn,
                (*(*con).window).id,
                A__NET_WM_STATE,
                A__NET_WM_STATE_MAXIMIZED_VERT,
            );
        }
        (*state).is_maximized_vert = con_maximized_vert;
    }
}
unsafe fn x_shape_frame(con: *mut Con, shape_kind: xcb_shape_sk_t) {
    if !(*con).window.is_null() {
    } else {
        __assert_fail(
            c"con->window".as_ptr(),
            c"../src/x.c".as_ptr(),
            873 as ::core::ffi::c_uint,
            c"void x_shape_frame(Con *, xcb_shape_sk_t)".as_ptr(),
        );
    };
    xcb_shape_combine(
        conn,
        XCB_SHAPE_SO_SET as ::core::ffi::c_int as xcb_shape_op_t,
        shape_kind as xcb_shape_kind_t,
        shape_kind as xcb_shape_kind_t,
        (*con).frame.id as xcb_window_t,
        (*con)
            .window_rect
            .x
            .wrapping_add((*con).border_width as u32) as i16,
        (*con)
            .window_rect
            .y
            .wrapping_add((*con).border_width as u32) as i16,
        (*(*con).window).id,
    );
    let mut rectangles: [xcb_rectangle_t; 4] = [xcb_rectangle_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    }; 4];
    let rectangles_count: usize =
        x_get_border_rectangles(con, &raw mut rectangles as *mut xcb_rectangle_t);
    if rectangles_count != 0 {
        xcb_shape_rectangles(
            conn,
            XCB_SHAPE_SO_UNION as ::core::ffi::c_int as xcb_shape_op_t,
            shape_kind as xcb_shape_kind_t,
            XCB_CLIP_ORDERING_UNSORTED as ::core::ffi::c_int as u8,
            (*con).frame.id as xcb_window_t,
            0 as i16,
            0 as i16,
            rectangles_count as u32,
            &raw mut rectangles as *mut xcb_rectangle_t,
        );
    }
}
unsafe fn x_unshape_frame(con: *mut Con, shape_kind: xcb_shape_sk_t) {
    if !(*con).window.is_null() {
    } else {
        __assert_fail(
            c"con->window".as_ptr(),
            c"../src/x.c".as_ptr(),
            893 as ::core::ffi::c_uint,
            c"void x_unshape_frame(Con *, xcb_shape_sk_t)".as_ptr(),
        );
    };
    xcb_shape_mask(
        conn,
        XCB_SHAPE_SO_SET as ::core::ffi::c_int as xcb_shape_op_t,
        shape_kind as xcb_shape_kind_t,
        (*con).frame.id as xcb_window_t,
        0 as i16,
        0 as i16,
        XCB_PIXMAP_NONE as ::core::ffi::c_int as xcb_pixmap_t,
    );
}
unsafe fn set_shape_state(con: *mut Con, need_reshape: bool) {
    if !shape_supported || (*con).window.is_null() {
        return;
    }
    let mut state: *mut con_state = ::core::ptr::null_mut::<con_state>();
    state = state_for_frame((*con).frame.id as xcb_window_t);
    if state.is_null() {
        errorlog(
            c"ERROR: window state for con %p not found\n"
                .as_ptr()
                .cast_mut(),
            con,
        );
        return;
    }
    if need_reshape as ::core::ffi::c_int != 0 && con_is_floating(con) as ::core::ffi::c_int != 0 {
        if (*(*con).window).shaped {
            x_shape_frame(con, XCB_SHAPE_SK_BOUNDING);
        }
        if (*(*con).window).input_shaped {
            x_shape_frame(con, XCB_SHAPE_SK_INPUT);
        }
    }
    if (*state).was_floating as ::core::ffi::c_int != 0 && !con_is_floating(con) {
        if (*(*con).window).shaped {
            x_unshape_frame(con, XCB_SHAPE_SK_BOUNDING);
        }
        if (*(*con).window).input_shaped {
            x_unshape_frame(con, XCB_SHAPE_SK_INPUT);
        }
    }
}

pub unsafe fn x_push_node(con: *mut Con) {
    let mut current: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut rect: Rect = (*con).rect as Rect;
    let state: *mut con_state = state_for_frame((*con).frame.id as xcb_window_t);
    if !(*state).name.is_null() {
        debuglog(
            c"%s:%s:%d - pushing name %s for con %p\n"
                .as_ptr()
                .cast_mut(),
            c"../src/x.c".as_ptr(),
            c"x_push_node".as_ptr(),
            946 as ::core::ffi::c_int,
            (*state).name,
            con,
        );
        xcb_change_property(
            conn,
            XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as u8,
            (*con).frame.id as xcb_window_t,
            XCB_ATOM_WM_NAME as ::core::ffi::c_int as xcb_atom_t,
            XCB_ATOM_STRING as ::core::ffi::c_int as xcb_atom_t,
            8 as u8,
            strlen((*state).name) as u32,
            (*state).name as *const ::core::ffi::c_void,
        );
        free((*state).name as *mut ::core::ffi::c_void);
        (*state).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if (*con).window.is_null()
        && ((*con).layout == Layout::Stacked || (*con).layout == Layout::Tabbed)
    {
        let mut max_y: u32 = 0 as u32;
        let mut max_height: u32 = 0 as u32;
        current = (*con).nodes_head.tqh_first as *mut Con;
        while !current.is_null() {
            let dr: *mut Rect = &raw mut (*current).deco_rect;
            if (*dr).y >= max_y && (*dr).height >= max_height {
                max_y = (*dr).y;
                max_height = (*dr).height;
            }
            current = (*current).nodes.tqe_next as *mut Con;
        }
        rect.height = max_y.wrapping_add(max_height);
        if rect.height == 0 as u32 {
            (*con).mapped = false_0 != 0;
        }
    } else if (*con).window.is_null() {
        (*con).mapped = false_0 != 0;
    }
    let mut need_reshape: bool = false_0 != 0;
    if (*state).need_reparent as ::core::ffi::c_int != 0 && !(*con).window.is_null() {
        debuglog(
            c"%s:%s:%d - Reparenting child window\n".as_ptr().cast_mut(),
            c"../src/x.c".as_ptr(),
            c"x_push_node".as_ptr(),
            978 as ::core::ffi::c_int,
        );
        let mut values: [u32; 1] = [XCB_NONE as u32];
        xcb_change_window_attributes(
            conn,
            (*state).old_frame,
            XCB_CW_EVENT_MASK as ::core::ffi::c_int as u32,
            &raw mut values as *mut u32 as *const ::core::ffi::c_void,
        );
        xcb_change_window_attributes(
            conn,
            (*(*con).window).id,
            XCB_CW_EVENT_MASK as ::core::ffi::c_int as u32,
            &raw mut values as *mut u32 as *const ::core::ffi::c_void,
        );
        xcb_reparent_window(
            conn,
            (*(*con).window).id,
            (*con).frame.id as xcb_window_t,
            0 as i16,
            0 as i16,
        );
        values[0 as ::core::ffi::c_int as usize] =
            (XCB_EVENT_MASK_BUTTON_PRESS as ::core::ffi::c_int
                | XCB_EVENT_MASK_BUTTON_RELEASE as ::core::ffi::c_int
                | XCB_EVENT_MASK_POINTER_MOTION as ::core::ffi::c_int
                | XCB_EVENT_MASK_EXPOSURE as ::core::ffi::c_int
                | XCB_EVENT_MASK_STRUCTURE_NOTIFY as ::core::ffi::c_int
                | XCB_EVENT_MASK_SUBSTRUCTURE_REDIRECT as ::core::ffi::c_int
                | XCB_EVENT_MASK_SUBSTRUCTURE_NOTIFY as ::core::ffi::c_int
                | XCB_EVENT_MASK_ENTER_WINDOW as ::core::ffi::c_int) as u32;
        xcb_change_window_attributes(
            conn,
            (*state).old_frame,
            XCB_CW_EVENT_MASK as ::core::ffi::c_int as u32,
            &raw mut values as *mut u32 as *const ::core::ffi::c_void,
        );
        values[0 as ::core::ffi::c_int as usize] =
            (XCB_EVENT_MASK_PROPERTY_CHANGE as ::core::ffi::c_int
                | XCB_EVENT_MASK_STRUCTURE_NOTIFY as ::core::ffi::c_int
                | XCB_EVENT_MASK_FOCUS_CHANGE as ::core::ffi::c_int) as u32;
        xcb_change_window_attributes(
            conn,
            (*(*con).window).id,
            XCB_CW_EVENT_MASK as ::core::ffi::c_int as u32,
            &raw mut values as *mut u32 as *const ::core::ffi::c_void,
        );
        (*state).old_frame = XCB_NONE as xcb_window_t;
        (*state).need_reparent = false_0 != 0;
        (*con).ignore_unmap = (*con).ignore_unmap.wrapping_add(1);
        debuglog(
            c"%s:%s:%d - ignore_unmap for reparenting of con %p (win 0x%08x) is now %d\n"
                .as_ptr()
                .cast_mut(),
            c"../src/x.c".as_ptr(),
            c"x_push_node".as_ptr(),
            999 as ::core::ffi::c_int,
            con,
            (*(*con).window).id,
            (*con).ignore_unmap as ::core::ffi::c_int,
        );
        need_reshape = true_0 != 0;
    }
    need_reshape = (need_reshape as ::core::ffi::c_int
        | ((*state).rect.width != rect.width
            || (*state).rect.height != rect.height
            || (*state).window_rect.width != (*con).window_rect.width
            || (*state).window_rect.height != (*con).window_rect.height)
            as ::core::ffi::c_int)
        != 0;
    need_reshape = (need_reshape as ::core::ffi::c_int
        | (con_is_floating(con) as ::core::ffi::c_int != 0 && !(*state).was_floating)
            as ::core::ffi::c_int)
        != 0;
    let mut is_pixmap_needed: bool = con_is_leaf(con) as ::core::ffi::c_int != 0
        && con_border_style(con) != BorderStyle::None
        || (*con).layout == Layout::Stacked
        || (*con).layout == Layout::Tabbed;
    debuglog(
        c"%s:%s:%d - Con %p (layout %d), is_pixmap_needed = %s, rect.height = %d\n"
            .as_ptr()
            .cast_mut(),
        c"../src/x.c".as_ptr(),
        c"x_push_node".as_ptr(),
        1019 as ::core::ffi::c_int,
        con,
        (*con).layout as ::core::ffi::c_uint,
        if is_pixmap_needed as ::core::ffi::c_int != 0 {
            c"yes".as_ptr()
        } else {
            c"no".as_ptr()
        },
        (*con).rect.height,
    );
    if (*con).type_0 as ::core::ffi::c_uint == CT_ROOT as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*con).type_0 as ::core::ffi::c_uint
            == CT_OUTPUT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        is_pixmap_needed = false_0 != 0;
    }
    let mut fake_notify: bool = false_0 != 0;
    if is_pixmap_needed as ::core::ffi::c_int != 0
        && (*con).frame_buffer.id as ::core::ffi::c_long == XCB_NONE
        || !rect_equals((*state).rect, rect) && rect.height > 0 as u32
    {
        let has_rect_changed: bool = (*state).rect.x != rect.x
            || (*state).rect.y != rect.y
            || (*state).rect.width != rect.width
            || (*state).rect.height != rect.height;
        if !is_pixmap_needed && (*con).frame_buffer.id as ::core::ffi::c_long != XCB_NONE {
            draw_util_surface_free(conn, &raw mut (*con).frame_buffer);
            xcb_free_pixmap(conn, (*con).frame_buffer.id as xcb_pixmap_t);
            (*con).frame_buffer.id = XCB_NONE as xcb_drawable_t;
        }
        if is_pixmap_needed as ::core::ffi::c_int != 0
            && (has_rect_changed as ::core::ffi::c_int != 0
                || (*con).frame_buffer.id as ::core::ffi::c_long == XCB_NONE)
        {
            if (*con).frame_buffer.id as ::core::ffi::c_long == XCB_NONE {
                (*con).frame_buffer.id = xcb_generate_id(conn) as xcb_drawable_t;
            } else {
                draw_util_surface_free(conn, &raw mut (*con).frame_buffer);
                xcb_free_pixmap(conn, (*con).frame_buffer.id as xcb_pixmap_t);
            }
            let mut win_depth: u16 = root_depth as u16;
            if !(*con).window.is_null() {
                win_depth = (*(*con).window).depth;
            }
            let width: ::core::ffi::c_int = if rect.width as i32 > 1 as i32 {
                rect.width as ::core::ffi::c_int
            } else {
                1 as ::core::ffi::c_int
            };
            let height: ::core::ffi::c_int = if rect.height as i32 > 1 as i32 {
                rect.height as ::core::ffi::c_int
            } else {
                1 as ::core::ffi::c_int
            };
            debuglog(
                c"%s:%s:%d - creating %d x %d pixmap for con %p (con->frame_buffer.id = (pixmap_t)0x%08x) (con->frame.id (drawable_t)0x%08x)\n".as_ptr().cast_mut(),
                c"../src/x.c".as_ptr(),
                c"x_push_node".as_ptr(),
                1068 as ::core::ffi::c_int,
                width,
                height,
                con,
                (*con).frame_buffer.id,
                (*con).frame.id,
            );
            xcb_create_pixmap(
                conn,
                win_depth as u8,
                (*con).frame_buffer.id as xcb_pixmap_t,
                (*con).frame.id,
                width as u16,
                height as u16,
            );
            draw_util_surface_init(
                conn,
                &raw mut (*con).frame_buffer,
                (*con).frame_buffer.id,
                get_visualtype_by_id(get_visualid_by_depth(win_depth)),
                width,
                height,
            );
            draw_util_clear_surface(
                &raw mut (*con).frame_buffer,
                color_t {
                    red: 0.0f64,
                    green: 0.0f64,
                    blue: 0.0f64,
                    alpha: 0.,
                    colorpixel: 0,
                },
            );
            let mut fresh4: [u32; 1] = [0 as ::core::ffi::c_int as u32];
            xcb_change_gc(
                conn,
                (*con).frame_buffer.gc,
                XCB_GC_GRAPHICS_EXPOSURES as ::core::ffi::c_int as u32,
                &raw mut fresh4 as *mut u32 as *const ::core::ffi::c_void,
            );
            draw_util_surface_set_size(&raw mut (*con).frame, width, height);
            (*con).pixmap_recreated = true_0 != 0;
            if (*con).parent.is_null()
                || (*(*con).parent).layout != Layout::Stacked
                || (*(*con).parent).focus_head.tqh_first == con
            {
                x_deco_recurse(con);
            }
        }
        debuglog(
            c"%s:%s:%d - setting rect (%d, %d, %d, %d)\n"
                .as_ptr()
                .cast_mut(),
            c"../src/x.c".as_ptr(),
            c"x_push_node".as_ptr(),
            1097 as ::core::ffi::c_int,
            rect.x,
            rect.y,
            rect.width,
            rect.height,
        );
        xcb_flush(conn);
        xcb_set_window_rect(conn, (*con).frame.id as xcb_window_t, rect);
        if (*con).frame_buffer.id as ::core::ffi::c_long != XCB_NONE {
            draw_util_copy_surface(
                &raw mut (*con).frame_buffer,
                &raw mut (*con).frame,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
                (*con).rect.width as ::core::ffi::c_double,
                (*con).rect.height as ::core::ffi::c_double,
            );
        }
        xcb_flush(conn);
        memcpy(
            &raw mut (*state).rect as *mut ::core::ffi::c_void,
            &raw mut rect as *const ::core::ffi::c_void,
            ::core::mem::size_of::<Rect>() as usize,
        );
        fake_notify = true_0 != 0;
    }
    if !(*con).window.is_null() && !rect_equals((*state).window_rect, (*con).window_rect as Rect) {
        debuglog(
            c"%s:%s:%d - setting window rect (%d, %d, %d, %d)\n"
                .as_ptr()
                .cast_mut(),
            c"../src/x.c".as_ptr(),
            c"x_push_node".as_ptr(),
            1117 as ::core::ffi::c_int,
            (*con).window_rect.x,
            (*con).window_rect.y,
            (*con).window_rect.width,
            (*con).window_rect.height,
        );
        xcb_set_window_rect(conn, (*(*con).window).id, (*con).window_rect as Rect);
        memcpy(
            &raw mut (*state).window_rect as *mut ::core::ffi::c_void,
            &raw mut (*con).window_rect as *const ::core::ffi::c_void,
            ::core::mem::size_of::<Rect>() as usize,
        );
        fake_notify = true_0 != 0;
    }
    set_shape_state(con, need_reshape);
    if !con.is_null() && !(*con).window.is_null() {
        let bsr: Rect = con_border_style_rect(con);
        let mut r: Rect = Rect {
            x: bsr.x,
            y: (0 as u32).wrapping_sub(bsr.width).wrapping_sub(bsr.x),
            width: bsr.y,
            height: (0 as u32).wrapping_sub(bsr.height).wrapping_sub(bsr.y),
        };
        xcb_change_property(
            conn,
            XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as u8,
            (*(*con).window).id,
            A__NET_FRAME_EXTENTS,
            XCB_ATOM_CARDINAL as ::core::ffi::c_int as xcb_atom_t,
            32 as u8,
            4 as u32,
            &raw mut r as *const ::core::ffi::c_void,
        );
    }
    if ((*state).mapped as ::core::ffi::c_int != (*con).mapped as ::core::ffi::c_int
        || !(*con).window.is_null() && !(*state).child_mapped)
        && (*con).mapped as ::core::ffi::c_int != 0
    {
        let mut cookie: xcb_void_cookie_t = xcb_void_cookie_t { sequence: 0 };
        if !(*con).window.is_null() {
            let mut data: [::core::ffi::c_long; 2] = [
                XCB_ICCCM_WM_STATE_NORMAL as ::core::ffi::c_int as ::core::ffi::c_long,
                XCB_NONE,
            ];
            xcb_change_property(
                conn,
                XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as u8,
                (*(*con).window).id,
                A_WM_STATE,
                A_WM_STATE,
                32 as u8,
                2 as u32,
                &raw mut data as *mut ::core::ffi::c_long as *const ::core::ffi::c_void,
            );
        }
        let mut values_0: [u32; 1] = [0; 1];
        if !(*state).child_mapped && !(*con).window.is_null() {
            cookie = xcb_map_window(conn, (*(*con).window).id);
            values_0[0 as ::core::ffi::c_int as usize] =
                (XCB_EVENT_MASK_PROPERTY_CHANGE as ::core::ffi::c_int
                    | XCB_EVENT_MASK_STRUCTURE_NOTIFY as ::core::ffi::c_int
                    | XCB_EVENT_MASK_FOCUS_CHANGE as ::core::ffi::c_int) as u32;
            xcb_change_window_attributes(
                conn,
                (*(*con).window).id,
                XCB_CW_EVENT_MASK as ::core::ffi::c_int as u32,
                &raw mut values_0 as *mut u32 as *const ::core::ffi::c_void,
            );
            debuglog(
                c"%s:%s:%d - mapping child window (serial %d)\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/x.c".as_ptr(),
                c"x_push_node".as_ptr(),
                1166 as ::core::ffi::c_int,
                cookie.sequence,
            );
            (*state).child_mapped = true_0 != 0;
        }
        cookie = xcb_map_window(conn, (*con).frame.id as xcb_window_t);
        values_0[0 as ::core::ffi::c_int as usize] =
            (XCB_EVENT_MASK_BUTTON_PRESS as ::core::ffi::c_int
                | XCB_EVENT_MASK_BUTTON_RELEASE as ::core::ffi::c_int
                | XCB_EVENT_MASK_POINTER_MOTION as ::core::ffi::c_int
                | XCB_EVENT_MASK_EXPOSURE as ::core::ffi::c_int
                | XCB_EVENT_MASK_STRUCTURE_NOTIFY as ::core::ffi::c_int
                | XCB_EVENT_MASK_SUBSTRUCTURE_REDIRECT as ::core::ffi::c_int
                | XCB_EVENT_MASK_SUBSTRUCTURE_NOTIFY as ::core::ffi::c_int
                | XCB_EVENT_MASK_ENTER_WINDOW as ::core::ffi::c_int) as u32;
        xcb_change_window_attributes(
            conn,
            (*con).frame.id as xcb_window_t,
            XCB_CW_EVENT_MASK as ::core::ffi::c_int as u32,
            &raw mut values_0 as *mut u32 as *const ::core::ffi::c_void,
        );
        if (*con).frame_buffer.id as ::core::ffi::c_long != XCB_NONE {
            draw_util_copy_surface(
                &raw mut (*con).frame_buffer,
                &raw mut (*con).frame,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
                (*con).rect.width as ::core::ffi::c_double,
                (*con).rect.height as ::core::ffi::c_double,
            );
        }
        xcb_flush(conn);
        debuglog(
            c"%s:%s:%d - mapping container %08x (serial %d)\n"
                .as_ptr()
                .cast_mut(),
            c"../src/x.c".as_ptr(),
            c"x_push_node".as_ptr(),
            1181 as ::core::ffi::c_int,
            (*con).frame.id,
            cookie.sequence,
        );
        (*state).mapped = (*con).mapped;
    }
    (*state).unmap_now = (*state).mapped as ::core::ffi::c_int
        != (*con).mapped as ::core::ffi::c_int
        && !(*con).mapped;
    (*state).was_floating = con_is_floating(con);
    if fake_notify {
        debuglog(
            c"%s:%s:%d - Sending fake configure notify\n"
                .as_ptr()
                .cast_mut(),
            c"../src/x.c".as_ptr(),
            c"x_push_node".as_ptr(),
            1189 as ::core::ffi::c_int,
        );
        fake_absolute_configure_notify(con);
    }
    set_hidden_state(con);
    set_maximized_state(con);
    current = (*con).focus_head.tqh_first as *mut Con;
    while !current.is_null() {
        x_push_node(current);
        current = (*current).focused.tqe_next as *mut Con;
    }
}
unsafe fn x_push_node_unmaps(con: *mut Con) {
    let mut current: *mut Con = ::core::ptr::null_mut::<Con>();
    let state: *mut con_state = state_for_frame((*con).frame.id as xcb_window_t);
    if (*state).unmap_now {
        if !(*con).window.is_null() {
            let mut data: [::core::ffi::c_long; 2] = [
                XCB_ICCCM_WM_STATE_WITHDRAWN as ::core::ffi::c_int as ::core::ffi::c_long,
                XCB_NONE,
            ];
            xcb_change_property(
                conn,
                XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as u8,
                (*(*con).window).id,
                A_WM_STATE,
                A_WM_STATE,
                32 as u8,
                2 as u32,
                &raw mut data as *mut ::core::ffi::c_long as *const ::core::ffi::c_void,
            );
        }
        let cookie: xcb_void_cookie_t =
            xcb_unmap_window(conn, (*con).frame.id as xcb_window_t) as xcb_void_cookie_t;
        debuglog(
            c"%s:%s:%d - unmapping container %p / %s (serial %d)\n"
                .as_ptr()
                .cast_mut(),
            c"../src/x.c".as_ptr(),
            c"x_push_node_unmaps".as_ptr(),
            1229 as ::core::ffi::c_int,
            con,
            (*con).name,
            cookie.sequence,
        );
        if !(*con).window.is_null() {
            (*con).ignore_unmap = (*con).ignore_unmap.wrapping_add(1);
            debuglog(
                c"%s:%s:%d - ignore_unmap for con %p (frame 0x%08x) now %d\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/x.c".as_ptr(),
                c"x_push_node_unmaps".as_ptr(),
                1235 as ::core::ffi::c_int,
                con,
                (*con).frame.id,
                (*con).ignore_unmap as ::core::ffi::c_int,
            );
        }
        (*state).mapped = (*con).mapped;
    }
    current = (*con).nodes_head.tqh_first as *mut Con;
    while !current.is_null() {
        x_push_node_unmaps(current);
        current = (*current).nodes.tqe_next as *mut Con;
    }
    current = (*con).floating_head.tqh_first as *mut Con;
    while !current.is_null() {
        x_push_node_unmaps(current);
        current = (*current).floating_windows.tqe_next as *mut Con;
    }
}
unsafe fn is_con_attached(con: *mut Con) -> bool {
    if (*con).parent.is_null() {
        return false_0 != 0;
    }
    let mut current: *mut Con = ::core::ptr::null_mut::<Con>();
    current = (*(*con).parent).nodes_head.tqh_first as *mut Con;
    while !current.is_null() {
        if current == con {
            return true_0 != 0;
        }
        current = (*current).nodes.tqe_next as *mut Con;
    }
    false_0 != 0
}

pub unsafe fn x_push_changes(con: *mut Con) {
    let mut state: *mut con_state = ::core::ptr::null_mut::<con_state>();
    let mut pointercookie: xcb_query_pointer_cookie_t = xcb_query_pointer_cookie_t { sequence: 0 };
    if !warp_to.is_null() {
        pointercookie = xcb_query_pointer(conn, root);
    }
    debuglog(
        c"%s:%s:%d - -- PUSHING WINDOW STACK --\n"
            .as_ptr()
            .cast_mut(),
        c"../src/x.c".as_ptr(),
        c"x_push_changes".as_ptr(),
        1290 as ::core::ffi::c_int,
    );
    let mut values: [u32; 1] = [XCB_EVENT_MASK_SUBSTRUCTURE_REDIRECT as ::core::ffi::c_int as u32];
    state = state_head.cqh_last as *mut con_state;
    while state != &raw mut state_head as *mut ::core::ffi::c_void as *mut con_state {
        if (*state).mapped {
            xcb_change_window_attributes(
                conn,
                (*state).id,
                XCB_CW_EVENT_MASK as ::core::ffi::c_int as u32,
                &raw mut values as *mut u32 as *const ::core::ffi::c_void,
            );
        }
        state = (*state).state.cqe_prev as *mut con_state;
    }
    let mut order_changed: bool = false_0 != 0;
    let mut stacking_changed: bool = false_0 != 0;
    let mut cnt: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    state = state_head.cqh_last as *mut con_state;
    while state != &raw mut state_head as *mut ::core::ffi::c_void as *mut con_state {
        if con_has_managed_window((*state).con) {
            cnt += 1;
        }
        state = (*state).state.cqe_prev as *mut con_state;
    }
    static mut client_list_windows: *mut xcb_window_t =
        ::core::ptr::null::<xcb_window_t>() as *mut xcb_window_t;
    static mut client_list_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if cnt != client_list_count {
        client_list_windows = srealloc(
            client_list_windows as *mut ::core::ffi::c_void,
            (::core::mem::size_of::<xcb_window_t>() as usize).wrapping_mul(cnt as usize),
        ) as *mut xcb_window_t;
        client_list_count = cnt;
    }
    let mut walk: *mut xcb_window_t = client_list_windows;
    state = state_head.cqh_last as *mut con_state;
    while state != &raw mut state_head as *mut ::core::ffi::c_void as *mut con_state {
        if con_has_managed_window((*state).con) {
            let fresh5 = walk;
            walk = walk.offset(1);
            memcpy(
                fresh5 as *mut ::core::ffi::c_void,
                &raw mut (*(*(*state).con).window).id as *const ::core::ffi::c_void,
                ::core::mem::size_of::<xcb_window_t>() as usize,
            );
        }
        let prev: *mut con_state = (*state).state.cqe_prev as *mut con_state;
        let old_prev: *mut con_state = (*state).old_state.cqe_prev as *mut con_state;
        if prev != old_prev {
            order_changed = true_0 != 0;
        }
        if ((*state).initial as ::core::ffi::c_int != 0 || order_changed as ::core::ffi::c_int != 0)
            && prev != &raw mut state_head as *mut ::core::ffi::c_void as *mut con_state
        {
            stacking_changed = true_0 != 0;
            let mut mask: u32 = 0 as u32;
            mask |= XCB_CONFIG_WINDOW_SIBLING as ::core::ffi::c_int as u32;
            mask |= XCB_CONFIG_WINDOW_STACK_MODE as ::core::ffi::c_int as u32;
            let mut values_0: [u32; 2] = [
                (*state).id,
                XCB_STACK_MODE_ABOVE as ::core::ffi::c_int as u32,
            ];
            xcb_configure_window(
                conn,
                (*prev).id,
                mask as u16,
                &raw mut values_0 as *mut u32 as *const ::core::ffi::c_void,
            );
        }
        (*state).initial = false_0 != 0;
        state = (*state).state.cqe_prev as *mut con_state;
    }
    if stacking_changed {
        debuglog(
            c"%s:%s:%d - Client list changed (%i clients)\n"
                .as_ptr()
                .cast_mut(),
            c"../src/x.c".as_ptr(),
            c"x_push_changes".as_ptr(),
            1350 as ::core::ffi::c_int,
            cnt,
        );
        ewmh_update_client_list_stacking(client_list_windows, client_list_count);
        walk = client_list_windows;
        state = initial_mapping_head.tqh_first as *mut con_state;
        while !state.is_null() {
            if con_has_managed_window((*state).con) {
                let fresh6 = walk;
                walk = walk.offset(1);
                *fresh6 = (*(*(*state).con).window).id;
            }
            state = (*state).initial_mapping_order.tqe_next as *mut con_state;
        }
        ewmh_update_client_list(client_list_windows, client_list_count);
    }
    debuglog(
        c"%s:%s:%d - PUSHING CHANGES\n".as_ptr().cast_mut(),
        c"../src/x.c".as_ptr(),
        c"x_push_changes".as_ptr(),
        1365 as ::core::ffi::c_int,
    );
    x_push_node(con);
    if !warp_to.is_null() {
        let pointerreply: *mut xcb_query_pointer_reply_t = xcb_query_pointer_reply(
            conn,
            pointercookie,
            ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
        );
        if pointerreply.is_null() {
            errorlog(
                c"ERROR: Could not query pointer position, not warping pointer\n"
                    .as_ptr()
                    .cast_mut(),
            );
        } else {
            let mid_x: ::core::ffi::c_int = (*warp_to)
                .x
                .wrapping_add((*warp_to).width.wrapping_div(2 as u32))
                as ::core::ffi::c_int;
            let mid_y: ::core::ffi::c_int = (*warp_to)
                .y
                .wrapping_add((*warp_to).height.wrapping_div(2 as u32))
                as ::core::ffi::c_int;
            let current: *mut Output = get_output_containing(
                (*pointerreply).root_x as ::core::ffi::c_uint,
                (*pointerreply).root_y as ::core::ffi::c_uint,
            );
            let target: *mut Output =
                get_output_containing(mid_x as ::core::ffi::c_uint, mid_y as ::core::ffi::c_uint);
            if current != target {
                let mut fresh7: [u32; 1] =
                    [XCB_EVENT_MASK_SUBSTRUCTURE_REDIRECT as ::core::ffi::c_int as u32];
                xcb_change_window_attributes(
                    conn,
                    root,
                    XCB_CW_EVENT_MASK as ::core::ffi::c_int as u32,
                    &raw mut fresh7 as *mut u32 as *const ::core::ffi::c_void,
                );
                xcb_warp_pointer(
                    conn,
                    XCB_NONE as xcb_window_t,
                    root,
                    0 as i16,
                    0 as i16,
                    0 as u16,
                    0 as u16,
                    mid_x as i16,
                    mid_y as i16,
                );
                let mut fresh8: [u32; 1] = [(XCB_EVENT_MASK_SUBSTRUCTURE_REDIRECT
                    as ::core::ffi::c_int
                    | XCB_EVENT_MASK_BUTTON_PRESS as ::core::ffi::c_int
                    | XCB_EVENT_MASK_STRUCTURE_NOTIFY as ::core::ffi::c_int
                    | XCB_EVENT_MASK_POINTER_MOTION as ::core::ffi::c_int
                    | XCB_EVENT_MASK_PROPERTY_CHANGE as ::core::ffi::c_int
                    | XCB_EVENT_MASK_FOCUS_CHANGE as ::core::ffi::c_int
                    | XCB_EVENT_MASK_ENTER_WINDOW as ::core::ffi::c_int)
                    as u32];
                xcb_change_window_attributes(
                    conn,
                    root,
                    XCB_CW_EVENT_MASK as ::core::ffi::c_int as u32,
                    &raw mut fresh8 as *mut u32 as *const ::core::ffi::c_void,
                );
            }
            free(pointerreply as *mut ::core::ffi::c_void);
        }
        warp_to = ::core::ptr::null_mut::<Rect>();
    }
    values[0 as ::core::ffi::c_int as usize] = (XCB_EVENT_MASK_BUTTON_PRESS as ::core::ffi::c_int
        | XCB_EVENT_MASK_BUTTON_RELEASE as ::core::ffi::c_int
        | XCB_EVENT_MASK_POINTER_MOTION as ::core::ffi::c_int
        | XCB_EVENT_MASK_EXPOSURE as ::core::ffi::c_int
        | XCB_EVENT_MASK_STRUCTURE_NOTIFY as ::core::ffi::c_int
        | XCB_EVENT_MASK_SUBSTRUCTURE_REDIRECT as ::core::ffi::c_int
        | XCB_EVENT_MASK_SUBSTRUCTURE_NOTIFY as ::core::ffi::c_int
        | XCB_EVENT_MASK_ENTER_WINDOW as ::core::ffi::c_int)
        as u32;
    state = state_head.cqh_last as *mut con_state;
    while state != &raw mut state_head as *mut ::core::ffi::c_void as *mut con_state {
        if (*state).mapped {
            xcb_change_window_attributes(
                conn,
                (*state).id,
                XCB_CW_EVENT_MASK as ::core::ffi::c_int as u32,
                &raw mut values as *mut u32 as *const ::core::ffi::c_void,
            );
        }
        state = (*state).state.cqe_prev as *mut con_state;
    }
    x_deco_recurse(con);
    let mut to_focus: xcb_window_t = (*focused).frame.id as xcb_window_t;
    if !(*focused).window.is_null() {
        to_focus = (*(*focused).window).id;
    }
    if focused_id != to_focus {
        if !(*focused).mapped {
            debuglog(
                c"%s:%s:%d - Not updating focus (to %p / %s), focused window is not mapped.\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/x.c".as_ptr(),
                c"x_push_changes".as_ptr(),
                1406 as ::core::ffi::c_int,
                focused,
                (*focused).name,
            );
            focused_id = XCB_NONE as xcb_window_t;
        } else {
            if !(*focused).window.is_null()
                && (*(*focused).window).needs_take_focus as ::core::ffi::c_int != 0
                && (*(*focused).window).doesnt_accept_focus as ::core::ffi::c_int != 0
            {
                debuglog(
                    c"%s:%s:%d - Updating focus by sending WM_TAKE_FOCUS to window 0x%08x (focused: %p / %s)\n".as_ptr().cast_mut(),
                    c"../src/x.c".as_ptr(),
                    c"x_push_changes".as_ptr(),
                    1414 as ::core::ffi::c_int,
                    to_focus,
                    focused,
                    (*focused).name,
                );
                send_take_focus(to_focus, last_timestamp);
                change_ewmh_focus(
                    if con_has_managed_window(focused) as ::core::ffi::c_int != 0 {
                        (*(*focused).window).id
                    } else {
                        XCB_WINDOW_NONE as ::core::ffi::c_int as xcb_window_t
                    },
                    last_focused,
                );
                if to_focus != last_focused && is_con_attached(focused) as ::core::ffi::c_int != 0 {
                    ipc_send_window_event(c"focus".as_ptr(), focused);
                }
            } else {
                debuglog(
                    c"%s:%s:%d - Updating focus (focused: %p / %s) to X11 window 0x%08x\n"
                        .as_ptr()
                        .cast_mut(),
                    c"../src/x.c".as_ptr(),
                    c"x_push_changes".as_ptr(),
                    1423 as ::core::ffi::c_int,
                    focused,
                    (*focused).name,
                    to_focus,
                );
                if !(*focused).window.is_null() {
                    values[0 as ::core::ffi::c_int as usize] = ((XCB_EVENT_MASK_PROPERTY_CHANGE
                        as ::core::ffi::c_int
                        | XCB_EVENT_MASK_STRUCTURE_NOTIFY as ::core::ffi::c_int
                        | XCB_EVENT_MASK_FOCUS_CHANGE as ::core::ffi::c_int)
                        & !(XCB_EVENT_MASK_FOCUS_CHANGE as ::core::ffi::c_int))
                        as u32;
                    xcb_change_window_attributes(
                        conn,
                        (*(*focused).window).id,
                        XCB_CW_EVENT_MASK as ::core::ffi::c_int as u32,
                        &raw mut values as *mut u32 as *const ::core::ffi::c_void,
                    );
                }
                xcb_set_input_focus(
                    conn,
                    XCB_INPUT_FOCUS_POINTER_ROOT as ::core::ffi::c_int as u8,
                    to_focus,
                    last_timestamp,
                );
                if !(*focused).window.is_null() {
                    values[0 as ::core::ffi::c_int as usize] = (XCB_EVENT_MASK_PROPERTY_CHANGE
                        as ::core::ffi::c_int
                        | XCB_EVENT_MASK_STRUCTURE_NOTIFY as ::core::ffi::c_int
                        | XCB_EVENT_MASK_FOCUS_CHANGE as ::core::ffi::c_int)
                        as u32;
                    xcb_change_window_attributes(
                        conn,
                        (*(*focused).window).id,
                        XCB_CW_EVENT_MASK as ::core::ffi::c_int as u32,
                        &raw mut values as *mut u32 as *const ::core::ffi::c_void,
                    );
                }
                change_ewmh_focus(
                    if con_has_managed_window(focused) as ::core::ffi::c_int != 0 {
                        (*(*focused).window).id
                    } else {
                        XCB_WINDOW_NONE as ::core::ffi::c_int as xcb_window_t
                    },
                    last_focused,
                );
                if to_focus as ::core::ffi::c_long != XCB_NONE
                    && to_focus != last_focused
                    && !(*focused).window.is_null()
                    && is_con_attached(focused) as ::core::ffi::c_int != 0
                {
                    ipc_send_window_event(c"focus".as_ptr(), focused);
                }
            }
            last_focused = to_focus;
            focused_id = last_focused;
        }
    }
    if focused_id as ::core::ffi::c_long == XCB_NONE {
        debuglog(
            c"%s:%s:%d - Still no window focused, better set focus to the EWMH support window (%d)\n".as_ptr().cast_mut(),
            c"../src/x.c".as_ptr(),
            c"x_push_changes".as_ptr(),
            1451 as ::core::ffi::c_int,
            ewmh_window,
        );
        xcb_set_input_focus(
            conn,
            XCB_INPUT_FOCUS_POINTER_ROOT as ::core::ffi::c_int as u8,
            ewmh_window,
            last_timestamp,
        );
        change_ewmh_focus(
            XCB_WINDOW_NONE as ::core::ffi::c_int as xcb_window_t,
            last_focused,
        );
        focused_id = ewmh_window;
        last_focused = XCB_NONE as xcb_window_t;
    }
    xcb_flush(conn);
    debuglog(
        c"%s:%s:%d - ENDING CHANGES\n".as_ptr().cast_mut(),
        c"../src/x.c".as_ptr(),
        c"x_push_changes".as_ptr(),
        1460 as ::core::ffi::c_int,
    );
    values[0 as ::core::ffi::c_int as usize] = ((XCB_EVENT_MASK_BUTTON_PRESS as ::core::ffi::c_int
        | XCB_EVENT_MASK_BUTTON_RELEASE as ::core::ffi::c_int
        | XCB_EVENT_MASK_POINTER_MOTION as ::core::ffi::c_int
        | XCB_EVENT_MASK_EXPOSURE as ::core::ffi::c_int
        | XCB_EVENT_MASK_STRUCTURE_NOTIFY as ::core::ffi::c_int
        | XCB_EVENT_MASK_SUBSTRUCTURE_REDIRECT as ::core::ffi::c_int
        | XCB_EVENT_MASK_SUBSTRUCTURE_NOTIFY as ::core::ffi::c_int
        | XCB_EVENT_MASK_ENTER_WINDOW as ::core::ffi::c_int)
        & !(XCB_EVENT_MASK_ENTER_WINDOW as ::core::ffi::c_int))
        as u32;
    state = state_head.cqh_last as *mut con_state;
    while state != &raw mut state_head as *mut ::core::ffi::c_void as *mut con_state {
        if (*state).unmap_now {
            xcb_change_window_attributes(
                conn,
                (*state).id,
                XCB_CW_EVENT_MASK as ::core::ffi::c_int as u32,
                &raw mut values as *mut u32 as *const ::core::ffi::c_void,
            );
        }
        state = (*state).state.cqe_prev as *mut con_state;
    }
    x_push_node_unmaps(con);
    state = state_head.cqh_first as *mut con_state;
    while state != &raw mut state_head as *mut ::core::ffi::c_void as *mut con_state {
        if (*state).old_state.cqe_next
            == &raw mut old_state_head as *mut ::core::ffi::c_void as *mut con_state
        {
            old_state_head.cqh_last = (*state).old_state.cqe_prev;
        } else {
            (*(*state).old_state.cqe_next).old_state.cqe_prev = (*state).old_state.cqe_prev;
        }
        if (*state).old_state.cqe_prev
            == &raw mut old_state_head as *mut ::core::ffi::c_void as *mut con_state
        {
            old_state_head.cqh_first = (*state).old_state.cqe_next;
        } else {
            (*(*state).old_state.cqe_prev).old_state.cqe_next = (*state).old_state.cqe_next;
        }
        (*state).old_state.cqe_next =
            &raw mut old_state_head as *mut ::core::ffi::c_void as *mut con_state;
        (*state).old_state.cqe_prev = old_state_head.cqh_last;
        if old_state_head.cqh_first
            == &raw mut old_state_head as *mut ::core::ffi::c_void as *mut con_state
        {
            old_state_head.cqh_first = state as *mut con_state;
        } else {
            (*old_state_head.cqh_last).old_state.cqe_next = state as *mut con_state;
        }
        old_state_head.cqh_last = state as *mut con_state;
        state = (*state).state.cqe_next as *mut con_state;
    }
    xcb_flush(conn);
}

pub unsafe fn x_raise_con(con: *const Con) {
    let state: *mut con_state = state_for_frame((*con).frame.id as xcb_window_t);
    if (*state).state.cqe_next == &raw mut state_head as *mut ::core::ffi::c_void as *mut con_state
    {
        state_head.cqh_last = (*state).state.cqe_prev;
    } else {
        (*(*state).state.cqe_next).state.cqe_prev = (*state).state.cqe_prev;
    }
    if (*state).state.cqe_prev == &raw mut state_head as *mut ::core::ffi::c_void as *mut con_state
    {
        state_head.cqh_first = (*state).state.cqe_next;
    } else {
        (*(*state).state.cqe_prev).state.cqe_next = (*state).state.cqe_next;
    }
    (*state).state.cqe_next = state_head.cqh_first;
    (*state).state.cqe_prev = &raw mut state_head as *mut ::core::ffi::c_void as *mut con_state;
    if state_head.cqh_last == &raw mut state_head as *mut ::core::ffi::c_void as *mut con_state {
        state_head.cqh_last = state as *mut con_state;
    } else {
        (*state_head.cqh_first).state.cqe_prev = state as *mut con_state;
    }
    state_head.cqh_first = state as *mut con_state;
}

pub unsafe fn x_set_name(con: *mut Con, name: *const ::core::ffi::c_char) {
    let mut state: *mut con_state = ::core::ptr::null_mut::<con_state>();
    state = state_for_frame((*con).frame.id as xcb_window_t);
    if state.is_null() {
        errorlog(c"ERROR: window state not found\n".as_ptr().cast_mut());
        return;
    }
    free((*state).name as *mut ::core::ffi::c_void);
    (*state).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*state).name = sstrdup(name);
}

pub unsafe fn update_shmlog_atom() {
    if *shmlogname as ::core::ffi::c_int == '\0' as i32 {
        xcb_delete_property(conn, root, A_I3_SHMLOG_PATH);
    } else {
        xcb_change_property(
            conn,
            XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as u8,
            root,
            A_I3_SHMLOG_PATH,
            A_UTF8_STRING,
            8 as u8,
            strlen(shmlogname) as u32,
            shmlogname as *const ::core::ffi::c_void,
        );
    };
}

pub unsafe fn x_set_i3_atoms() {
    let mut pid: libc::pid_t = getpid() as libc::pid_t;
    xcb_change_property(
        conn,
        XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as u8,
        root,
        A_I3_SOCKET_PATH,
        A_UTF8_STRING,
        8 as u8,
        (if current_socketpath.is_null() {
            0 as usize
        } else {
            strlen(current_socketpath)
        }) as u32,
        current_socketpath as *const ::core::ffi::c_void,
    );
    xcb_change_property(
        conn,
        XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as u8,
        root,
        A_I3_PID,
        XCB_ATOM_CARDINAL as ::core::ffi::c_int as xcb_atom_t,
        32 as u8,
        1 as u32,
        &raw mut pid as *const ::core::ffi::c_void,
    );
    xcb_change_property(
        conn,
        XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as u8,
        root,
        A_I3_CONFIG_PATH,
        A_UTF8_STRING,
        8 as u8,
        strlen(current_configpath) as u32,
        current_configpath as *const ::core::ffi::c_void,
    );
    xcb_change_property(
        conn,
        XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as u8,
        root,
        A_I3_LOG_STREAM_SOCKET_PATH,
        A_UTF8_STRING,
        8 as u8,
        strlen(current_log_stream_socket_path) as u32,
        current_log_stream_socket_path as *const ::core::ffi::c_void,
    );
    update_shmlog_atom();
}

pub unsafe fn x_set_warp_to(rect: *mut Rect) {
    if config.mouse_warping != Warping::None {
        warp_to = rect;
    }
}

pub unsafe fn x_mask_event_mask(mask: u32) {
    let mut values: [u32; 1] = [(XCB_EVENT_MASK_BUTTON_PRESS as ::core::ffi::c_int
        | XCB_EVENT_MASK_BUTTON_RELEASE as ::core::ffi::c_int
        | XCB_EVENT_MASK_POINTER_MOTION as ::core::ffi::c_int
        | XCB_EVENT_MASK_EXPOSURE as ::core::ffi::c_int
        | XCB_EVENT_MASK_STRUCTURE_NOTIFY as ::core::ffi::c_int
        | XCB_EVENT_MASK_SUBSTRUCTURE_REDIRECT as ::core::ffi::c_int
        | XCB_EVENT_MASK_SUBSTRUCTURE_NOTIFY as ::core::ffi::c_int
        | XCB_EVENT_MASK_ENTER_WINDOW as ::core::ffi::c_int)
        as u32
        & mask];
    let mut state: *mut con_state = ::core::ptr::null_mut::<con_state>();
    state = state_head.cqh_last as *mut con_state;
    while state != &raw mut state_head as *mut ::core::ffi::c_void as *mut con_state {
        if (*state).mapped {
            xcb_change_window_attributes(
                conn,
                (*state).id,
                XCB_CW_EVENT_MASK as ::core::ffi::c_int as u32,
                &raw mut values as *mut u32 as *const ::core::ffi::c_void,
            );
        }
        state = (*state).state.cqe_prev as *mut con_state;
    }
}

pub unsafe fn x_set_shape(con: *mut Con, kind: xcb_shape_sk_t, enable: bool) {
    if state_for_frame((*con).frame.id as xcb_window_t).is_null() {
        errorlog(
            c"ERROR: window state for con %p not found\n"
                .as_ptr()
                .cast_mut(),
            con,
        );
        return;
    }
    match kind as ::core::ffi::c_uint {
        0 => {
            (*(*con).window).shaped = enable;
        }
        2 => {
            (*(*con).window).input_shaped = enable;
        }
        _ => {
            errorlog(
                c"ERROR: Received unknown shape event kind for con %p. This is a bug.\n"
                    .as_ptr()
                    .cast_mut(),
                con,
            );
            return;
        }
    }
    if con_is_floating(con) {
        if enable {
            x_shape_frame(con, kind);
        } else {
            x_unshape_frame(con, kind);
        }
        xcb_flush(conn);
    }
}
unsafe extern "C" fn run_static_initializers() {
    initial_mapping_head = initial_mapping_head {
        tqh_first: ::core::ptr::null_mut::<con_state>(),
        tqh_last: &raw mut initial_mapping_head.tqh_first,
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];

// Internal helpers use Rust linkage rather than foreign symbol lookup.
use crate::ewmh::ewmh_update_active_window;
use crate::ewmh::ewmh_update_client_list;
use crate::ewmh::ewmh_update_client_list_stacking;
use crate::ewmh::ewmh_update_focused;
use crate::util::max;
use crate::util::min;
use crate::xcb::get_visualid_by_depth;
use crate::xcb::send_take_focus;
use i3_common::dpi::logical_px;
use i3_common::font::font_is_pango;
use i3_common::safewrappers::scalloc;
use i3_common::safewrappers::srealloc;
use i3_common::safewrappers::sstrdup;

// Canonical Cairo backend ABI; renderer logic remains local.

// Canonical Pango backend ABI; font policy remains local.

use xcb_sys::{
    xcb_atom_t, xcb_change_gc, xcb_change_property, xcb_change_window_attributes,
    xcb_client_message_event_t, xcb_colormap_t, xcb_configure_window, xcb_create_colormap,
    xcb_create_pixmap, xcb_delete_property, xcb_destroy_window, xcb_drawable_t, xcb_flush,
    xcb_free_colormap, xcb_free_pixmap, xcb_generate_id, xcb_generic_error_t,
    xcb_get_property_cookie_t, xcb_get_property_reply_t, xcb_icccm_get_wm_protocols,
    xcb_icccm_get_wm_protocols_reply, xcb_icccm_get_wm_protocols_reply_t,
    xcb_icccm_get_wm_protocols_reply_wipe, xcb_kill_client, xcb_map_window, xcb_pixmap_t,
    xcb_query_pointer, xcb_query_pointer_cookie_t, xcb_query_pointer_reply,
    xcb_query_pointer_reply_t, xcb_rectangle_t, xcb_reparent_window, xcb_send_event,
    xcb_set_input_focus, xcb_shape_combine, xcb_shape_kind_t, xcb_shape_mask, xcb_shape_op_t,
    xcb_shape_rectangles, xcb_shape_sk_t, xcb_unmap_window, xcb_visualid_t, xcb_void_cookie_t,
    xcb_warp_pointer, xcb_window_t, XCB_ATOM_CARDINAL, XCB_ATOM_STRING, XCB_ATOM_WM_CLASS,
    XCB_ATOM_WM_NAME, XCB_CLIENT_MESSAGE, XCB_CLIP_ORDERING_UNSORTED, XCB_COLORMAP_ALLOC_NONE,
    XCB_CONFIG_WINDOW_SIBLING, XCB_CONFIG_WINDOW_STACK_MODE, XCB_CURRENT_TIME, XCB_CW_BACK_PIXEL,
    XCB_CW_BORDER_PIXEL, XCB_CW_COLORMAP, XCB_CW_EVENT_MASK, XCB_CW_OVERRIDE_REDIRECT,
    XCB_EVENT_MASK_BUTTON_PRESS, XCB_EVENT_MASK_BUTTON_RELEASE, XCB_EVENT_MASK_ENTER_WINDOW,
    XCB_EVENT_MASK_EXPOSURE, XCB_EVENT_MASK_FOCUS_CHANGE, XCB_EVENT_MASK_NO_EVENT,
    XCB_EVENT_MASK_POINTER_MOTION, XCB_EVENT_MASK_PROPERTY_CHANGE, XCB_EVENT_MASK_STRUCTURE_NOTIFY,
    XCB_EVENT_MASK_SUBSTRUCTURE_NOTIFY, XCB_EVENT_MASK_SUBSTRUCTURE_REDIRECT,
    XCB_GC_GRAPHICS_EXPOSURES, XCB_ICCCM_WM_STATE_NORMAL, XCB_ICCCM_WM_STATE_WITHDRAWN,
    XCB_INPUT_FOCUS_POINTER_ROOT, XCB_NONE, XCB_PIXMAP_NONE, XCB_PROP_MODE_REPLACE,
    XCB_SHAPE_SK_BOUNDING, XCB_SHAPE_SK_INPUT, XCB_SHAPE_SO_SET, XCB_SHAPE_SO_UNION,
    XCB_STACK_MODE_ABOVE, XCB_WINDOW_CLASS_INPUT_OUTPUT, XCB_WINDOW_NONE,
};

pub use libc::{free, getpid, memcmp, memcpy, memset, strlen};

use i3_common::native_stdio::__assert_fail;

use crate::log::{debuglog, errorlog, verboselog};

// Shared model types retain field layout while using one Rust identity.
pub use crate::types::deco_render_params;
pub use crate::types::mark_t;
pub use crate::types::nodes_head;
pub use crate::types::width_height;
pub use crate::types::xoutput;
pub use crate::types::Colortriple;
pub use crate::types::Con;
pub use crate::types::Rect;
pub use crate::types::Window;
pub use i3_common::draw_util::color_t;
pub use i3_common::draw_util::surface_t;
pub use i3_common::font::Font;
pub use i3_common::string::I3String as _i3String;

// Internal calls and globals use canonical Rust identities.
use crate::application::colormap;
use crate::application::conn;
use crate::application::last_timestamp;
use crate::application::root;
use crate::application::root_depth;
use crate::application::root_screen;
use crate::application::shape_supported;
use crate::application::A_I3_CONFIG_PATH;
use crate::application::A_I3_LOG_STREAM_SOCKET_PATH;
use crate::application::A_I3_PID;
use crate::application::A_I3_SHMLOG_PATH;
use crate::application::A_I3_SOCKET_PATH;
use crate::application::A_UTF8_STRING;
use crate::application::A_WM_DELETE_WINDOW;
use crate::application::A_WM_PROTOCOLS;
use crate::application::A_WM_STATE;
use crate::application::A__NET_FRAME_EXTENTS;
use crate::application::A__NET_WM_STATE;
use crate::application::A__NET_WM_STATE_HIDDEN;
use crate::application::A__NET_WM_STATE_MAXIMIZED_HORZ;
use crate::application::A__NET_WM_STATE_MAXIMIZED_VERT;
use crate::con::con_adjacent_borders;
use crate::con::con_border_style;
use crate::con::con_border_style_rect;
use crate::con::con_descend_focused;
use crate::con::con_draw_decoration_into_frame;
use crate::con::con_get_tree_representation;
use crate::con::con_has_managed_window;
use crate::con::con_inside_focused;
use crate::con::con_is_floating;
use crate::con::con_is_hidden;
use crate::con::con_is_leaf;
use crate::con::con_is_maximized;
use crate::con::con_parse_title_format;
use crate::config::config;
use crate::config::current_configpath;
use crate::ewmh::ewmh_window;
use crate::ipc::current_socketpath;
use crate::ipc::ipc_send_window_event;
use crate::log::current_log_stream_socket_path;
use crate::log::shmlogname;
use crate::randr::get_output_containing;
use crate::tree::focused;
use crate::util::rect_equals;
use crate::xcb::create_window;
use crate::xcb::fake_absolute_configure_notify;
use crate::xcb::get_visualtype_by_id;
use crate::xcb::xcb_add_property_atom;
use crate::xcb::xcb_remove_property_atom;
use crate::xcb::xcb_set_window_rect;
use i3_common::draw_util::draw_util_clear_surface;
use i3_common::draw_util::draw_util_copy_surface;
use i3_common::draw_util::draw_util_image;
use i3_common::draw_util::draw_util_rectangle;
use i3_common::draw_util::draw_util_surface_free;
use i3_common::draw_util::draw_util_surface_init;
use i3_common::draw_util::draw_util_surface_set_size;
use i3_common::font::predict_text_width;
use i3_common::string::i3string_free;
use i3_common::string::i3string_from_utf8;
