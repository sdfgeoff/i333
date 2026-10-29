/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * drag.c: click and drag.
 *
 */
// Mechanical file-by-file Rust translation of src/drag.c.
// Generated with C2Rust 0.22.1; C ABI and original ownership retained.
extern "C" {
    pub type xcb_connection_t;
    pub type _cairo;
    pub type _cairo_surface;
    pub type _i3String;
    pub type pcre2_real_code_8;
    pub type ev_loop;
    fn xcb_grab_pointer(
        c: *mut xcb_connection_t,
        owner_events: uint8_t,
        grab_window: xcb_window_t,
        event_mask: uint16_t,
        pointer_mode: uint8_t,
        keyboard_mode: uint8_t,
        confine_to: xcb_window_t,
        cursor: xcb_cursor_t,
        time: xcb_timestamp_t,
    ) -> xcb_grab_pointer_cookie_t;
    fn xcb_grab_pointer_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_grab_pointer_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_grab_pointer_reply_t;
    fn xcb_ungrab_pointer(c: *mut xcb_connection_t, time: xcb_timestamp_t) -> xcb_void_cookie_t;
    fn xcb_change_active_pointer_grab(
        c: *mut xcb_connection_t,
        cursor: xcb_cursor_t,
        time: xcb_timestamp_t,
        event_mask: uint16_t,
    ) -> xcb_void_cookie_t;
    fn xcb_grab_keyboard(
        c: *mut xcb_connection_t,
        owner_events: uint8_t,
        grab_window: xcb_window_t,
        time: xcb_timestamp_t,
        pointer_mode: uint8_t,
        keyboard_mode: uint8_t,
    ) -> xcb_grab_keyboard_cookie_t;
    fn xcb_grab_keyboard_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_grab_keyboard_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_grab_keyboard_reply_t;
    fn xcb_ungrab_keyboard(c: *mut xcb_connection_t, time: xcb_timestamp_t) -> xcb_void_cookie_t;
    fn xcb_flush(c: *mut xcb_connection_t) -> ::core::ffi::c_int;
    fn xcb_poll_for_event(c: *mut xcb_connection_t) -> *mut xcb_generic_event_t;
    static mut conn: *mut xcb_connection_t;
    fn errorlog(fmt: *mut ::core::ffi::c_char, ...);
    fn debuglog(fmt: *mut ::core::ffi::c_char, ...);
    fn logical_px(logical: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn ev_run(loop_0: *mut ev_loop, flags: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn ev_break(loop_0: *mut ev_loop, how: ::core::ffi::c_int);
    fn ev_prepare_start(loop_0: *mut ev_loop, w: *mut ev_prepare);
    fn ev_prepare_stop(loop_0: *mut ev_loop, w: *mut ev_prepare);
    static mut focused: *mut Con;
    static mut root: xcb_window_t;
    static mut main_loop: *mut ev_loop;
    fn xcursor_get_cursor(c: xcursor_cursor_t) -> xcb_cursor_t;
    static mut A__NET_WM_MOVERESIZE: xcb_atom_t;
    fn handle_event(type_0: ::core::ffi::c_int, event: *mut xcb_generic_event_t);
    fn con_get_workspace(con: *mut Con) -> *mut Con;
    fn con_by_window_id(window: xcb_window_t) -> *mut Con;
    fn con_exists(con: *mut Con) -> bool;
    fn main_set_x11_cb(enable: bool);
    fn memmove(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __time_t = ::core::ffi::c_long;
pub type __suseconds_t = ::core::ffi::c_long;
pub type time_t = __time_t;
pub type int16_t = __int16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_generic_event_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub pad: [uint32_t; 7],
    pub full_sequence: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_generic_error_t {
    pub response_type: uint8_t,
    pub error_code: uint8_t,
    pub sequence: uint16_t,
    pub resource_id: uint32_t,
    pub minor_code: uint16_t,
    pub major_code: uint8_t,
    pub pad0: uint8_t,
    pub pad: [uint32_t; 5],
    pub full_sequence: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_void_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
pub type xcb_window_t = uint32_t;
pub type xcb_cursor_t = uint32_t;
pub type xcb_gcontext_t = uint32_t;
pub type xcb_colormap_t = uint32_t;
pub type xcb_atom_t = uint32_t;
pub type xcb_drawable_t = uint32_t;
pub type xcb_timestamp_t = uint32_t;
pub type xcb_button_t = uint8_t;
pub type xcb_event_mask_t = ::core::ffi::c_uint;
pub const XCB_EVENT_MASK_OWNER_GRAB_BUTTON: xcb_event_mask_t = 16777216;
pub const XCB_EVENT_MASK_COLOR_MAP_CHANGE: xcb_event_mask_t = 8388608;
pub const XCB_EVENT_MASK_PROPERTY_CHANGE: xcb_event_mask_t = 4194304;
pub const XCB_EVENT_MASK_FOCUS_CHANGE: xcb_event_mask_t = 2097152;
pub const XCB_EVENT_MASK_SUBSTRUCTURE_REDIRECT: xcb_event_mask_t = 1048576;
pub const XCB_EVENT_MASK_SUBSTRUCTURE_NOTIFY: xcb_event_mask_t = 524288;
pub const XCB_EVENT_MASK_RESIZE_REDIRECT: xcb_event_mask_t = 262144;
pub const XCB_EVENT_MASK_STRUCTURE_NOTIFY: xcb_event_mask_t = 131072;
pub const XCB_EVENT_MASK_VISIBILITY_CHANGE: xcb_event_mask_t = 65536;
pub const XCB_EVENT_MASK_EXPOSURE: xcb_event_mask_t = 32768;
pub const XCB_EVENT_MASK_KEYMAP_STATE: xcb_event_mask_t = 16384;
pub const XCB_EVENT_MASK_BUTTON_MOTION: xcb_event_mask_t = 8192;
pub const XCB_EVENT_MASK_BUTTON_5_MOTION: xcb_event_mask_t = 4096;
pub const XCB_EVENT_MASK_BUTTON_4_MOTION: xcb_event_mask_t = 2048;
pub const XCB_EVENT_MASK_BUTTON_3_MOTION: xcb_event_mask_t = 1024;
pub const XCB_EVENT_MASK_BUTTON_2_MOTION: xcb_event_mask_t = 512;
pub const XCB_EVENT_MASK_BUTTON_1_MOTION: xcb_event_mask_t = 256;
pub const XCB_EVENT_MASK_POINTER_MOTION_HINT: xcb_event_mask_t = 128;
pub const XCB_EVENT_MASK_POINTER_MOTION: xcb_event_mask_t = 64;
pub const XCB_EVENT_MASK_LEAVE_WINDOW: xcb_event_mask_t = 32;
pub const XCB_EVENT_MASK_ENTER_WINDOW: xcb_event_mask_t = 16;
pub const XCB_EVENT_MASK_BUTTON_RELEASE: xcb_event_mask_t = 8;
pub const XCB_EVENT_MASK_BUTTON_PRESS: xcb_event_mask_t = 4;
pub const XCB_EVENT_MASK_KEY_RELEASE: xcb_event_mask_t = 2;
pub const XCB_EVENT_MASK_KEY_PRESS: xcb_event_mask_t = 1;
pub const XCB_EVENT_MASK_NO_EVENT: xcb_event_mask_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_button_press_event_t {
    pub response_type: uint8_t,
    pub detail: xcb_button_t,
    pub sequence: uint16_t,
    pub time: xcb_timestamp_t,
    pub root: xcb_window_t,
    pub event: xcb_window_t,
    pub child: xcb_window_t,
    pub root_x: int16_t,
    pub root_y: int16_t,
    pub event_x: int16_t,
    pub event_y: int16_t,
    pub state: uint16_t,
    pub same_screen: uint8_t,
    pub pad0: uint8_t,
}
pub type xcb_button_release_event_t = xcb_button_press_event_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_motion_notify_event_t {
    pub response_type: uint8_t,
    pub detail: uint8_t,
    pub sequence: uint16_t,
    pub time: xcb_timestamp_t,
    pub root: xcb_window_t,
    pub event: xcb_window_t,
    pub child: xcb_window_t,
    pub root_x: int16_t,
    pub root_y: int16_t,
    pub event_x: int16_t,
    pub event_y: int16_t,
    pub state: uint16_t,
    pub same_screen: uint8_t,
    pub pad0: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_unmap_notify_event_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub event: xcb_window_t,
    pub window: xcb_window_t,
    pub from_configure: uint8_t,
    pub pad1: [uint8_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union xcb_client_message_data_t {
    pub data8: [uint8_t; 20],
    pub data16: [uint16_t; 10],
    pub data32: [uint32_t; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_client_message_event_t {
    pub response_type: uint8_t,
    pub format: uint8_t,
    pub sequence: uint16_t,
    pub window: xcb_window_t,
    pub type_0: xcb_atom_t,
    pub data: xcb_client_message_data_t,
}
pub type xcb_grab_mode_t = ::core::ffi::c_uint;
pub const XCB_GRAB_MODE_ASYNC: xcb_grab_mode_t = 1;
pub const XCB_GRAB_MODE_SYNC: xcb_grab_mode_t = 0;
pub type xcb_grab_status_t = ::core::ffi::c_uint;
pub const XCB_GRAB_STATUS_FROZEN: xcb_grab_status_t = 4;
pub const XCB_GRAB_STATUS_NOT_VIEWABLE: xcb_grab_status_t = 3;
pub const XCB_GRAB_STATUS_INVALID_TIME: xcb_grab_status_t = 2;
pub const XCB_GRAB_STATUS_ALREADY_GRABBED: xcb_grab_status_t = 1;
pub const XCB_GRAB_STATUS_SUCCESS: xcb_grab_status_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_grab_pointer_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_grab_pointer_reply_t {
    pub response_type: uint8_t,
    pub status: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
}
pub type xcb_button_index_t = ::core::ffi::c_uint;
pub const XCB_BUTTON_INDEX_5: xcb_button_index_t = 5;
pub const XCB_BUTTON_INDEX_4: xcb_button_index_t = 4;
pub const XCB_BUTTON_INDEX_3: xcb_button_index_t = 3;
pub const XCB_BUTTON_INDEX_2: xcb_button_index_t = 2;
pub const XCB_BUTTON_INDEX_1: xcb_button_index_t = 1;
pub const XCB_BUTTON_INDEX_ANY: xcb_button_index_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_grab_keyboard_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_grab_keyboard_reply_t {
    pub response_type: uint8_t,
    pub status: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
}
pub type cairo_t = _cairo;
pub type cairo_surface_t = _cairo_surface;
pub type i3String = _i3String;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct color_t {
    pub red: ::core::ffi::c_double,
    pub green: ::core::ffi::c_double,
    pub blue: ::core::ffi::c_double,
    pub alpha: ::core::ffi::c_double,
    pub colorpixel: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct surface_t {
    pub id: xcb_drawable_t,
    pub gc: xcb_gcontext_t,
    pub owns_gc: bool,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub surface: *mut cairo_surface_t,
    pub cr: *mut cairo_t,
}
pub type pcre2_code_8 = pcre2_real_code_8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Rect {
    pub x: uint32_t,
    pub y: uint32_t,
    pub width: uint32_t,
    pub height: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Con {
    pub mapped: bool,
    pub urgent: bool,
    pub ignore_unmap: uint8_t,
    pub frame: surface_t,
    pub frame_buffer: surface_t,
    pub pixmap_recreated: bool,
    pub type_0: C2RustUnnamed_15,
    pub num: ::core::ffi::c_int,
    pub gaps: gaps_t,
    pub parent: *mut Con,
    pub rect: Rect,
    pub window_rect: Rect,
    pub deco_rect: Rect,
    pub geometry: Rect,
    pub name: *mut ::core::ffi::c_char,
    pub title_format: *mut ::core::ffi::c_char,
    pub window_icon_padding: ::core::ffi::c_int,
    pub sticky_group: *mut ::core::ffi::c_char,
    pub marks_head: marks_head,
    pub mark_changed: bool,
    pub percent: ::core::ffi::c_double,
    pub border_width: ::core::ffi::c_int,
    pub current_border_width: ::core::ffi::c_int,
    pub window: *mut Window,
    pub urgency_timer: *mut ev_timer,
    pub deco_render_params: *mut deco_render_params,
    pub floating_head: floating_head,
    pub nodes_head: nodes_head,
    pub focus_head: focus_head,
    pub swallow_head: swallow_head,
    pub fullscreen_mode: fullscreen_mode_t,
    pub sticky: bool,
    pub layout: layout_t,
    pub last_split_layout: layout_t,
    pub workspace_layout: layout_t,
    pub border_style: border_style_t,
    pub max_user_border_style: border_style_t,
    pub floating: C2RustUnnamed_4,
    pub nodes: C2RustUnnamed_3,
    pub focused: C2RustUnnamed_2,
    pub all_cons: C2RustUnnamed_1,
    pub floating_windows: C2RustUnnamed_0,
    pub on_remove_child: Option<unsafe extern "C" fn(*mut Con) -> ()>,
    pub scratchpad_state: C2RustUnnamed,
    pub old_id: ::core::ffi::c_int,
    pub depth: uint16_t,
    pub colormap: xcb_colormap_t,
}
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const SCRATCHPAD_CHANGED: C2RustUnnamed = 2;
pub const SCRATCHPAD_FRESH: C2RustUnnamed = 1;
pub const SCRATCHPAD_NONE: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub tqe_next: *mut Con,
    pub tqe_prev: *mut *mut Con,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub tqe_next: *mut Con,
    pub tqe_prev: *mut *mut Con,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub tqe_next: *mut Con,
    pub tqe_prev: *mut *mut Con,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub tqe_next: *mut Con,
    pub tqe_prev: *mut *mut Con,
}
pub type C2RustUnnamed_4 = ::core::ffi::c_uint;
pub const FLOATING_USER_ON: C2RustUnnamed_4 = 3;
pub const FLOATING_AUTO_ON: C2RustUnnamed_4 = 2;
pub const FLOATING_USER_OFF: C2RustUnnamed_4 = 1;
pub const FLOATING_AUTO_OFF: C2RustUnnamed_4 = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct swallow_head {
    pub tqh_first: *mut Match,
    pub tqh_last: *mut *mut Match,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Match {
    pub error: *mut ::core::ffi::c_char,
    pub title: *mut regex,
    pub application: *mut regex,
    pub class: *mut regex,
    pub instance: *mut regex,
    pub mark: *mut regex,
    pub window_role: *mut regex,
    pub workspace: *mut regex,
    pub machine: *mut regex,
    pub window_type: xcb_atom_t,
    pub urgent: C2RustUnnamed_9,
    pub dock: C2RustUnnamed_8,
    pub id: xcb_window_t,
    pub window_mode: C2RustUnnamed_7,
    pub con_id: *mut Con,
    pub match_all_windows: bool,
    pub insert_where: C2RustUnnamed_6,
    pub matches: C2RustUnnamed_5,
    pub restart_mode: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub tqe_next: *mut Match,
    pub tqe_prev: *mut *mut Match,
}
pub type C2RustUnnamed_6 = ::core::ffi::c_uint;
pub const M_BELOW: C2RustUnnamed_6 = 2;
pub const M_ASSIGN_WS: C2RustUnnamed_6 = 1;
pub const M_HERE: C2RustUnnamed_6 = 0;
pub type C2RustUnnamed_7 = ::core::ffi::c_uint;
pub const WM_FLOATING: C2RustUnnamed_7 = 6;
pub const WM_FLOATING_USER: C2RustUnnamed_7 = 5;
pub const WM_FLOATING_AUTO: C2RustUnnamed_7 = 4;
pub const WM_TILING: C2RustUnnamed_7 = 3;
pub const WM_TILING_USER: C2RustUnnamed_7 = 2;
pub const WM_TILING_AUTO: C2RustUnnamed_7 = 1;
pub const WM_ANY: C2RustUnnamed_7 = 0;
pub type C2RustUnnamed_8 = ::core::ffi::c_int;
pub const M_DOCK_BOTTOM: C2RustUnnamed_8 = 3;
pub const M_DOCK_TOP: C2RustUnnamed_8 = 2;
pub const M_DOCK_ANY: C2RustUnnamed_8 = 1;
pub const M_NODOCK: C2RustUnnamed_8 = 0;
pub const M_DONTCHECK: C2RustUnnamed_8 = -1;
pub type C2RustUnnamed_9 = ::core::ffi::c_int;
pub const U_OLDEST: C2RustUnnamed_9 = 1;
pub const U_LATEST: C2RustUnnamed_9 = 0;
pub const U_DONTCHECK: C2RustUnnamed_9 = -1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct regex {
    pub pattern: *mut ::core::ffi::c_char,
    pub regex: *mut pcre2_code_8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct focus_head {
    pub tqh_first: *mut Con,
    pub tqh_last: *mut *mut Con,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nodes_head {
    pub tqh_first: *mut Con,
    pub tqh_last: *mut *mut Con,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct floating_head {
    pub tqh_first: *mut Con,
    pub tqh_last: *mut *mut Con,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct deco_render_params {
    pub color: *mut Colortriple,
    pub border_style: ::core::ffi::c_int,
    pub con_rect: width_height,
    pub con_window_rect: width_height,
    pub con_deco_rect: Rect,
    pub background: color_t,
    pub parent_layout: layout_t,
    pub con_is_leaf: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct width_height {
    pub w: uint32_t,
    pub h: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Colortriple {
    pub border: color_t,
    pub background: color_t,
    pub text: color_t,
    pub indicator: color_t,
    pub child_border: color_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ev_timer {
    pub active: ::core::ffi::c_int,
    pub pending: ::core::ffi::c_int,
    pub priority: ::core::ffi::c_int,
    pub data: *mut ::core::ffi::c_void,
    pub cb: Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_timer, ::core::ffi::c_int) -> ()>,
    pub at: ev_tstamp,
    pub repeat: ev_tstamp,
}
pub type ev_tstamp = ::core::ffi::c_double;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Window {
    pub id: xcb_window_t,
    pub leader: xcb_window_t,
    pub transient_for: xcb_window_t,
    pub nr_assignments: uint32_t,
    pub ran_assignments: *mut *mut Assignment,
    pub class_class: *mut ::core::ffi::c_char,
    pub class_instance: *mut ::core::ffi::c_char,
    pub name: *mut i3String,
    pub role: *mut ::core::ffi::c_char,
    pub machine: *mut ::core::ffi::c_char,
    pub name_x_changed: bool,
    pub uses_net_wm_name: bool,
    pub needs_take_focus: bool,
    pub doesnt_accept_focus: bool,
    pub window_type: xcb_atom_t,
    pub wm_desktop: uint32_t,
    pub dock: C2RustUnnamed_10,
    pub urgent: timeval,
    pub reserved: reservedpx,
    pub depth: uint16_t,
    pub base_width: ::core::ffi::c_int,
    pub base_height: ::core::ffi::c_int,
    pub width_increment: ::core::ffi::c_int,
    pub height_increment: ::core::ffi::c_int,
    pub min_width: ::core::ffi::c_int,
    pub min_height: ::core::ffi::c_int,
    pub max_width: ::core::ffi::c_int,
    pub max_height: ::core::ffi::c_int,
    pub min_aspect_ratio: ::core::ffi::c_double,
    pub max_aspect_ratio: ::core::ffi::c_double,
    pub icon: *mut cairo_surface_t,
    pub shaped: bool,
    pub input_shaped: bool,
    pub managed_since: time_t,
    pub swallowed: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct reservedpx {
    pub left: uint32_t,
    pub right: uint32_t,
    pub top: uint32_t,
    pub bottom: uint32_t,
}
pub type C2RustUnnamed_10 = ::core::ffi::c_uint;
pub const W_DOCK_BOTTOM: C2RustUnnamed_10 = 2;
pub const W_DOCK_TOP: C2RustUnnamed_10 = 1;
pub const W_NODOCK: C2RustUnnamed_10 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Assignment {
    pub type_0: C2RustUnnamed_13,
    pub match_0: Match,
    pub dest: C2RustUnnamed_12,
    pub assignments: C2RustUnnamed_11,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub tqe_next: *mut Assignment,
    pub tqe_prev: *mut *mut Assignment,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_12 {
    pub command: *mut ::core::ffi::c_char,
    pub workspace: *mut ::core::ffi::c_char,
    pub output: *mut ::core::ffi::c_char,
}
pub type C2RustUnnamed_13 = ::core::ffi::c_uint;
pub const A_TO_OUTPUT: C2RustUnnamed_13 = 16;
pub const A_TO_WORKSPACE_NUMBER: C2RustUnnamed_13 = 8;
pub const A_NO_FOCUS: C2RustUnnamed_13 = 4;
pub const A_TO_WORKSPACE: C2RustUnnamed_13 = 2;
pub const A_COMMAND: C2RustUnnamed_13 = 1;
pub const A_ANY: C2RustUnnamed_13 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct marks_head {
    pub tqh_first: *mut mark_t,
    pub tqh_last: *mut *mut mark_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mark_t {
    pub name: *mut ::core::ffi::c_char,
    pub marks: C2RustUnnamed_14,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub tqe_next: *mut mark_t,
    pub tqe_prev: *mut *mut mark_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gaps_t {
    pub inner: ::core::ffi::c_int,
    pub top: ::core::ffi::c_int,
    pub right: ::core::ffi::c_int,
    pub bottom: ::core::ffi::c_int,
    pub left: ::core::ffi::c_int,
}
pub type C2RustUnnamed_15 = ::core::ffi::c_uint;
pub const CT_DOCKAREA: C2RustUnnamed_15 = 5;
pub const CT_WORKSPACE: C2RustUnnamed_15 = 4;
pub const CT_FLOATING_CON: C2RustUnnamed_15 = 3;
pub const CT_CON: C2RustUnnamed_15 = 2;
pub const CT_OUTPUT: C2RustUnnamed_15 = 1;
pub const CT_ROOT: C2RustUnnamed_15 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ev_watcher {
    pub active: ::core::ffi::c_int,
    pub pending: ::core::ffi::c_int,
    pub priority: ::core::ffi::c_int,
    pub data: *mut ::core::ffi::c_void,
    pub cb: Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_watcher, ::core::ffi::c_int) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ev_prepare {
    pub active: ::core::ffi::c_int,
    pub pending: ::core::ffi::c_int,
    pub priority: ::core::ffi::c_int,
    pub data: *mut ::core::ffi::c_void,
    pub cb: Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_prepare, ::core::ffi::c_int) -> ()>,
}
pub type C2RustUnnamed_16 = ::core::ffi::c_uint;
pub const EVBREAK_ALL: C2RustUnnamed_16 = 2;
pub const EVBREAK_ONE: C2RustUnnamed_16 = 1;
pub const EVBREAK_CANCEL: C2RustUnnamed_16 = 0;
pub type xcursor_cursor_t = ::core::ffi::c_uint;
pub const XCURSOR_CURSOR_MAX: xcursor_cursor_t = 9;
pub const XCURSOR_CURSOR_MOVE: xcursor_cursor_t = 8;
pub const XCURSOR_CURSOR_WATCH: xcursor_cursor_t = 7;
pub const XCURSOR_CURSOR_BOTTOM_RIGHT_CORNER: xcursor_cursor_t = 6;
pub const XCURSOR_CURSOR_BOTTOM_LEFT_CORNER: xcursor_cursor_t = 5;
pub const XCURSOR_CURSOR_TOP_RIGHT_CORNER: xcursor_cursor_t = 4;
pub const XCURSOR_CURSOR_TOP_LEFT_CORNER: xcursor_cursor_t = 3;
pub const XCURSOR_CURSOR_RESIZE_VERTICAL: xcursor_cursor_t = 2;
pub const XCURSOR_CURSOR_RESIZE_HORIZONTAL: xcursor_cursor_t = 1;
pub const XCURSOR_CURSOR_POINTER: xcursor_cursor_t = 0;
pub type callback_t = Option<
    unsafe extern "C" fn(
        *mut Con,
        *mut Rect,
        uint32_t,
        uint32_t,
        *const xcb_button_press_event_t,
        *const ::core::ffi::c_void,
    ) -> (),
>;
pub type drag_result_t = ::core::ffi::c_uint;
pub const DRAG_ABORT: drag_result_t = 3;
pub const DRAG_REVERT: drag_result_t = 2;
pub const DRAG_SUCCESS: drag_result_t = 1;
pub const DRAGGING: drag_result_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct drag_x11_cb {
    pub prepare: ev_prepare,
    pub result: drag_result_t,
    pub con: *mut Con,
    pub event: *const xcb_button_press_event_t,
    pub old_rect: Rect,
    pub callback: callback_t,
    pub threshold_exceeded: bool,
    pub xcursor: xcb_cursor_t,
    pub extra: *const ::core::ffi::c_void,
}
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const XCB_KEY_PRESS: ::core::ffi::c_int = 2;
pub const XCB_BUTTON_PRESS: ::core::ffi::c_int = 4;
pub const XCB_BUTTON_RELEASE: ::core::ffi::c_int = 5;
pub const XCB_MOTION_NOTIFY: ::core::ffi::c_int = 6;
pub const XCB_UNMAP_NOTIFY: ::core::ffi::c_int = 18;
pub const XCB_CLIENT_MESSAGE: ::core::ffi::c_int = 33;
pub const XCB_NONE: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const XCB_CURRENT_TIME: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[inline]
unsafe extern "C" fn ev_loop(mut loop_0: *mut ev_loop, mut flags: ::core::ffi::c_int) {
    ev_run(loop_0, flags);
}
unsafe extern "C" fn threshold_exceeded(
    mut x1: uint32_t,
    mut y1: uint32_t,
    mut x2: uint32_t,
    mut y2: uint32_t,
) -> bool {
    let threshold: uint32_t = logical_px(15 as ::core::ffi::c_int) as uint32_t;
    return x1
        .wrapping_sub(x2)
        .wrapping_mul(x1.wrapping_sub(x2))
        .wrapping_add(y1.wrapping_sub(y2).wrapping_mul(y1.wrapping_sub(y2)))
        > threshold.wrapping_mul(threshold);
}
unsafe extern "C" fn drain_drag_events(
    mut loop_0: *mut ev_loop,
    mut dragloop: *mut drag_x11_cb,
) -> bool {
    let mut last_motion_notify: *mut xcb_motion_notify_event_t =
        ::core::ptr::null_mut::<xcb_motion_notify_event_t>();
    let mut event: *mut xcb_generic_event_t = ::core::ptr::null_mut::<xcb_generic_event_t>();
    loop {
        event = xcb_poll_for_event(conn);
        if event.is_null() {
            break;
        }
        if (*event).response_type as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            let mut error: *mut xcb_generic_error_t = event as *mut xcb_generic_error_t;
            debuglog(
                b"%s:%s:%d - X11 Error received (probably harmless)! sequence 0x%x, error_code = %d\n\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"../src/drag.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"drain_drag_events\0" as *const u8 as *const ::core::ffi::c_char,
                58 as ::core::ffi::c_int,
                (*error).sequence as ::core::ffi::c_int,
                (*error).error_code as ::core::ffi::c_int,
            );
            free(event as *mut ::core::ffi::c_void);
        } else {
            let mut type_0: ::core::ffi::c_int =
                (*event).response_type as ::core::ffi::c_int & 0x7f as ::core::ffi::c_int;
            match type_0 {
                XCB_BUTTON_PRESS => {
                    let mut press: *const xcb_button_press_event_t =
                        event as *mut xcb_button_press_event_t;
                    debuglog(
                        b"%s:%s:%d - Ignoring button %d press during drag initiated by button %d\n\0"
                            as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        b"../src/drag.c\0" as *const u8 as *const ::core::ffi::c_char,
                        b"drain_drag_events\0" as *const u8
                            as *const ::core::ffi::c_char,
                        70 as ::core::ffi::c_int,
                        (*press).detail as ::core::ffi::c_int,
                        (*(*dragloop).event).detail as ::core::ffi::c_int,
                    );
                }
                XCB_BUTTON_RELEASE => {
                    let mut release: *const xcb_button_release_event_t =
                        event as *mut xcb_button_release_event_t;
                    if (*(*dragloop).event).detail as ::core::ffi::c_int
                        == XCB_BUTTON_INDEX_ANY as ::core::ffi::c_int
                        || (*release).detail as ::core::ffi::c_int
                            == (*(*dragloop).event).detail as ::core::ffi::c_int
                    {
                        debuglog(
                            b"%s:%s:%d - Button %d released, ending drag initiated by button %d\n\0"
                                as *const u8
                                as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b"../src/drag.c\0" as *const u8 as *const ::core::ffi::c_char,
                            b"drain_drag_events\0" as *const u8 as *const ::core::ffi::c_char,
                            79 as ::core::ffi::c_int,
                            (*release).detail as ::core::ffi::c_int,
                            (*(*dragloop).event).detail as ::core::ffi::c_int,
                        );
                        (*dragloop).result = DRAG_SUCCESS;
                    } else {
                        debuglog(
                            b"%s:%s:%d - Ignoring button %d release during drag initiated by button %d\n\0"
                                as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b"../src/drag.c\0" as *const u8
                                as *const ::core::ffi::c_char,
                            b"drain_drag_events\0" as *const u8
                                as *const ::core::ffi::c_char,
                            83 as ::core::ffi::c_int,
                            (*release).detail as ::core::ffi::c_int,
                            (*(*dragloop).event).detail as ::core::ffi::c_int,
                        );
                    }
                }
                XCB_KEY_PRESS => {
                    debuglog(
                        b"%s:%s:%d - A key was pressed during drag, reverting changes.\n\0"
                            as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        b"../src/drag.c\0" as *const u8 as *const ::core::ffi::c_char,
                        b"drain_drag_events\0" as *const u8 as *const ::core::ffi::c_char,
                        89 as ::core::ffi::c_int,
                    );
                    (*dragloop).result = DRAG_REVERT;
                    handle_event(type_0, event);
                }
                XCB_UNMAP_NOTIFY => {
                    let mut unmap_event: *mut xcb_unmap_notify_event_t =
                        event as *mut xcb_unmap_notify_event_t;
                    let mut con: *mut Con = con_by_window_id((*unmap_event).window);
                    if !con.is_null() {
                        debuglog(
                            b"%s:%s:%d - UnmapNotify for window 0x%08x (container %p)\n\0"
                                as *const u8
                                as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b"../src/drag.c\0" as *const u8 as *const ::core::ffi::c_char,
                            b"drain_drag_events\0" as *const u8 as *const ::core::ffi::c_char,
                            99 as ::core::ffi::c_int,
                            (*unmap_event).window,
                            con,
                        );
                        if con_get_workspace(con) == con_get_workspace(focused) {
                            debuglog(
                                b"%s:%s:%d - UnmapNotify for a managed window on the current workspace, aborting\n\0"
                                    as *const u8 as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                                b"../src/drag.c\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                b"drain_drag_events\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                102 as ::core::ffi::c_int,
                            );
                            (*dragloop).result = DRAG_ABORT;
                        }
                    }
                    handle_event(type_0, event);
                }
                XCB_MOTION_NOTIFY => {
                    free(last_motion_notify as *mut ::core::ffi::c_void);
                    last_motion_notify = ::core::ptr::null_mut::<xcb_motion_notify_event_t>();
                    last_motion_notify = event as *mut xcb_motion_notify_event_t;
                }
                XCB_CLIENT_MESSAGE => {
                    let mut client_message: *const xcb_client_message_event_t =
                        event as *mut xcb_client_message_event_t;
                    if (*client_message).type_0 == A__NET_WM_MOVERESIZE {
                        debuglog(
                            b"%s:%s:%d - Ignoring _NET_WM_MOVERESIZE request during active drag (window = 0x%08x, direction = %d)\n\0"
                                as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b"../src/drag.c\0" as *const u8
                                as *const ::core::ffi::c_char,
                            b"drain_drag_events\0" as *const u8
                                as *const ::core::ffi::c_char,
                            122 as ::core::ffi::c_int,
                            (*client_message).window,
                            (*client_message)
                                .data
                                .data32[2 as ::core::ffi::c_int as usize],
                        );
                    } else {
                        handle_event(type_0, event);
                    }
                }
                _ => {
                    debuglog(
                        b"%s:%s:%d - Passing to original handler\n\0" as *const u8
                            as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        b"../src/drag.c\0" as *const u8 as *const ::core::ffi::c_char,
                        b"drain_drag_events\0" as *const u8 as *const ::core::ffi::c_char,
                        131 as ::core::ffi::c_int,
                    );
                    handle_event(type_0, event);
                }
            }
            if last_motion_notify != event as *mut xcb_motion_notify_event_t {
                free(event as *mut ::core::ffi::c_void);
            }
            if !((*dragloop).result as ::core::ffi::c_uint
                != DRAGGING as ::core::ffi::c_int as ::core::ffi::c_uint)
            {
                continue;
            }
            ev_break(loop_0, EVBREAK_ONE as ::core::ffi::c_int);
            if (*dragloop).result as ::core::ffi::c_uint
                == DRAG_SUCCESS as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                break;
            }
            free(last_motion_notify as *mut ::core::ffi::c_void);
            return true_0 != 0;
        }
    }
    if last_motion_notify.is_null() {
        return true_0 != 0;
    }
    if !(*dragloop).threshold_exceeded
        && threshold_exceeded(
            (*last_motion_notify).root_x as uint32_t,
            (*last_motion_notify).root_y as uint32_t,
            (*(*dragloop).event).root_x as uint32_t,
            (*(*dragloop).event).root_y as uint32_t,
        ) as ::core::ffi::c_int
            != 0
    {
        if (*dragloop).xcursor as ::core::ffi::c_long != XCB_NONE {
            xcb_change_active_pointer_grab(
                conn,
                (*dragloop).xcursor,
                XCB_CURRENT_TIME as xcb_timestamp_t,
                (XCB_EVENT_MASK_BUTTON_RELEASE as ::core::ffi::c_int
                    | XCB_EVENT_MASK_POINTER_MOTION as ::core::ffi::c_int)
                    as uint16_t,
            );
        }
        (*dragloop).threshold_exceeded = true_0 != 0;
    }
    if (*dragloop).threshold_exceeded as ::core::ffi::c_int != 0
        && ((*dragloop).con.is_null() || con_exists((*dragloop).con) as ::core::ffi::c_int != 0)
    {
        (*dragloop).callback.expect("non-null function pointer")(
            (*dragloop).con,
            &raw mut (*dragloop).old_rect,
            (*last_motion_notify).root_x as uint32_t,
            (*last_motion_notify).root_y as uint32_t,
            (*dragloop).event,
            (*dragloop).extra,
        );
    }
    free(last_motion_notify as *mut ::core::ffi::c_void);
    last_motion_notify = ::core::ptr::null_mut::<xcb_motion_notify_event_t>();
    xcb_flush(conn);
    return (*dragloop).result as ::core::ffi::c_uint
        != DRAGGING as ::core::ffi::c_int as ::core::ffi::c_uint;
}
unsafe extern "C" fn xcb_drag_prepare_cb(
    mut loop_0: *mut ev_loop,
    mut w: *mut ev_prepare,
    mut revents: ::core::ffi::c_int,
) {
    let mut dragloop: *mut drag_x11_cb = (*w).data as *mut drag_x11_cb;
    while !drain_drag_events(loop_0, dragloop) {}
}
#[no_mangle]
pub unsafe extern "C" fn drag_pointer(
    mut con: *mut Con,
    mut event: *const xcb_button_press_event_t,
    mut confine_to: xcb_window_t,
    mut cursor: ::core::ffi::c_int,
    mut use_threshold: bool,
    mut callback: callback_t,
    mut extra: *const ::core::ffi::c_void,
) -> drag_result_t {
    let xcursor: xcb_cursor_t = (if cursor != 0 {
        xcursor_get_cursor(cursor as xcursor_cursor_t) as ::core::ffi::c_long
    } else {
        XCB_NONE
    }) as xcb_cursor_t;
    let mut reply: *mut xcb_grab_pointer_reply_t =
        ::core::ptr::null_mut::<xcb_grab_pointer_reply_t>();
    let mut error: *mut xcb_generic_error_t = ::core::ptr::null_mut::<xcb_generic_error_t>();
    let mut cookie: xcb_grab_pointer_cookie_t = xcb_grab_pointer(
        conn,
        false_0 as uint8_t,
        root,
        (XCB_EVENT_MASK_BUTTON_RELEASE as ::core::ffi::c_int
            | XCB_EVENT_MASK_POINTER_MOTION as ::core::ffi::c_int) as uint16_t,
        XCB_GRAB_MODE_ASYNC as ::core::ffi::c_int as uint8_t,
        XCB_GRAB_MODE_ASYNC as ::core::ffi::c_int as uint8_t,
        confine_to,
        (if use_threshold as ::core::ffi::c_int != 0 {
            XCB_NONE
        } else {
            xcursor as ::core::ffi::c_long
        }) as xcb_cursor_t,
        XCB_CURRENT_TIME as xcb_timestamp_t,
    );
    reply = xcb_grab_pointer_reply(conn, cookie, &raw mut error);
    if reply.is_null() {
        errorlog(
            b"ERROR: Could not grab pointer (error_code = %d)\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            (*error).error_code as ::core::ffi::c_int,
        );
        free(error as *mut ::core::ffi::c_void);
        return DRAG_ABORT;
    }
    if (*reply).status as ::core::ffi::c_int != XCB_GRAB_STATUS_SUCCESS as ::core::ffi::c_int {
        errorlog(
            b"ERROR: Could not grab pointer (status = %d)\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            (*reply).status as ::core::ffi::c_int,
        );
        free(reply as *mut ::core::ffi::c_void);
        return DRAG_ABORT;
    }
    debuglog(
        b"%s:%s:%d - Pointer grab successful\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/drag.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"drag_pointer\0" as *const u8 as *const ::core::ffi::c_char,
        238 as ::core::ffi::c_int,
    );
    free(reply as *mut ::core::ffi::c_void);
    let mut keyb_reply: *mut xcb_grab_keyboard_reply_t =
        ::core::ptr::null_mut::<xcb_grab_keyboard_reply_t>();
    let mut keyb_cookie: xcb_grab_keyboard_cookie_t = xcb_grab_keyboard(
        conn,
        false_0 as uint8_t,
        root,
        XCB_CURRENT_TIME as xcb_timestamp_t,
        XCB_GRAB_MODE_ASYNC as ::core::ffi::c_int as uint8_t,
        XCB_GRAB_MODE_ASYNC as ::core::ffi::c_int as uint8_t,
    );
    keyb_reply = xcb_grab_keyboard_reply(conn, keyb_cookie, &raw mut error);
    if keyb_reply.is_null() {
        errorlog(
            b"ERROR: Could not grab keyboard (error_code = %d)\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            (*error).error_code as ::core::ffi::c_int,
        );
        free(error as *mut ::core::ffi::c_void);
        xcb_ungrab_pointer(conn, XCB_CURRENT_TIME as xcb_timestamp_t);
        return DRAG_ABORT;
    }
    if (*keyb_reply).status as ::core::ffi::c_int != XCB_GRAB_STATUS_SUCCESS as ::core::ffi::c_int {
        errorlog(
            b"ERROR: Could not grab keyboard (status = %d)\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            (*keyb_reply).status as ::core::ffi::c_int,
        );
        free(keyb_reply as *mut ::core::ffi::c_void);
        xcb_ungrab_pointer(conn, XCB_CURRENT_TIME as xcb_timestamp_t);
        return DRAG_ABORT;
    }
    debuglog(
        b"%s:%s:%d - Keyboard grab successful\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/drag.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"drag_pointer\0" as *const u8 as *const ::core::ffi::c_char,
        266 as ::core::ffi::c_int,
    );
    free(keyb_reply as *mut ::core::ffi::c_void);
    let mut loop_0: drag_x11_cb = drag_x11_cb {
        prepare: ev_prepare {
            active: 0,
            pending: 0,
            priority: 0,
            data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            cb: None,
        },
        result: DRAGGING,
        con: con,
        event: event,
        old_rect: Rect {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        callback: callback,
        threshold_exceeded: !use_threshold,
        xcursor: xcursor,
        extra: extra,
    };
    let mut prepare: *mut ev_prepare = &raw mut loop_0.prepare;
    if !con.is_null() {
        loop_0.old_rect = (*con).rect as Rect;
    }
    let ref mut fresh0 = (*(prepare as *mut ::core::ffi::c_void as *mut ev_watcher)).pending;
    *fresh0 = 0 as ::core::ffi::c_int;
    (*(prepare as *mut ::core::ffi::c_void as *mut ev_watcher)).active = *fresh0;
    (*(prepare as *mut ::core::ffi::c_void as *mut ev_watcher)).priority = 0 as ::core::ffi::c_int;
    (*prepare).cb = Some(
        xcb_drag_prepare_cb
            as unsafe extern "C" fn(*mut ev_loop, *mut ev_prepare, ::core::ffi::c_int) -> (),
    )
        as Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_prepare, ::core::ffi::c_int) -> ()>;
    memmove(
        &raw mut (*(prepare as *mut ev_watcher)).cb as *mut ::core::ffi::c_void,
        &raw mut (*prepare).cb as *const ::core::ffi::c_void,
        ::core::mem::size_of::<
            Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_prepare, ::core::ffi::c_int) -> ()>,
        >() as size_t,
    );
    (*prepare).data = &raw mut loop_0 as *mut ::core::ffi::c_void;
    main_set_x11_cb(false_0 != 0);
    ev_prepare_start(main_loop, prepare);
    debuglog(
        b"%s:%s:%d - Entering drag loop (con = %p, button = %d)\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/drag.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"drag_pointer\0" as *const u8 as *const ::core::ffi::c_char,
        288 as ::core::ffi::c_int,
        con,
        (*event).detail as ::core::ffi::c_int,
    );
    ev_loop(main_loop, 0 as ::core::ffi::c_int);
    debuglog(
        b"%s:%s:%d - Leaving drag loop (con = %p, button = %d, result = %d)\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/drag.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"drag_pointer\0" as *const u8 as *const ::core::ffi::c_char,
        291 as ::core::ffi::c_int,
        con,
        (*event).detail as ::core::ffi::c_int,
        loop_0.result as ::core::ffi::c_uint,
    );
    ev_prepare_stop(main_loop, prepare);
    main_set_x11_cb(true_0 != 0);
    xcb_ungrab_keyboard(conn, XCB_CURRENT_TIME as xcb_timestamp_t);
    xcb_ungrab_pointer(conn, XCB_CURRENT_TIME as xcb_timestamp_t);
    xcb_flush(conn);
    return loop_0.result;
}
