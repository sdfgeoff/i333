/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * resize.c: Interactive resizing.
 *
 */
// Mechanical file-by-file Rust translation of src/resize.c.
// Generated with C2Rust 0.22.1; C ABI and original ownership retained.
extern "C" {
    pub type xcb_connection_t;
    pub type _PangoFontDescription;
    pub type _cairo;
    pub type _cairo_surface;
    pub type _i3String;
    pub type pcre2_real_code_8;
    pub type ev_loop;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn __assert_single_arg(_: bool) -> bool;
    fn xcb_destroy_window(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t;
    fn xcb_map_window(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t;
    fn xcb_configure_window(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        value_mask: uint16_t,
        value_list: *const ::core::ffi::c_void,
    ) -> xcb_void_cookie_t;
    fn xcb_circulate_window(
        c: *mut xcb_connection_t,
        direction: uint8_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t;
    fn xcb_warp_pointer(
        c: *mut xcb_connection_t,
        src_window: xcb_window_t,
        dst_window: xcb_window_t,
        src_x: int16_t,
        src_y: int16_t,
        src_width: uint16_t,
        src_height: uint16_t,
        dst_x: int16_t,
        dst_y: int16_t,
    ) -> xcb_void_cookie_t;
    fn xcb_flush(c: *mut xcb_connection_t) -> ::core::ffi::c_int;
    static mut conn: *mut xcb_connection_t;
    fn debuglog(fmt: *mut ::core::ffi::c_char, ...);
    fn logical_px(logical: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn orientation_from_direction(direction: direction_t) -> orientation_t;
    fn tree_render();
    static mut config: Config;
    fn create_window(
        conn_0: *mut xcb_connection_t,
        r: Rect,
        depth: uint16_t,
        visual: xcb_visualid_t,
        window_class: uint16_t,
        cursor: xcursor_cursor_t,
        map: bool,
        mask: uint32_t,
        values: *mut uint32_t,
    ) -> xcb_window_t;
    fn x_mask_event_mask(mask: uint32_t);
    fn drag_pointer(
        con: *mut Con,
        event: *const xcb_button_press_event_t,
        confine_to: xcb_window_t,
        cursor: ::core::ffi::c_int,
        use_threshold: bool,
        callback: callback_t,
        extra: *const ::core::ffi::c_void,
    ) -> drag_result_t;
    fn con_get_output(con: *mut Con) -> *mut Con;
    fn con_fix_percent(con: *mut Con);
    fn con_orientation(con: *mut Con) -> orientation_t;
    fn con_descend_focused(con: *mut Con) -> *mut Con;
    fn con_rect_size_in_orientation(con: *mut Con) -> uint32_t;
}
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __time_t = ::core::ffi::c_long;
pub type __suseconds_t = ::core::ffi::c_long;
pub type time_t = __time_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
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
pub struct xcb_void_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
pub type xcb_window_t = uint32_t;
pub type xcb_font_t = uint32_t;
pub type xcb_gcontext_t = uint32_t;
pub type xcb_colormap_t = uint32_t;
pub type xcb_atom_t = uint32_t;
pub type xcb_drawable_t = uint32_t;
pub type xcb_visualid_t = uint32_t;
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
pub type xcb_window_class_t = ::core::ffi::c_uint;
pub const XCB_WINDOW_CLASS_INPUT_ONLY: xcb_window_class_t = 2;
pub const XCB_WINDOW_CLASS_INPUT_OUTPUT: xcb_window_class_t = 1;
pub const XCB_WINDOW_CLASS_COPY_FROM_PARENT: xcb_window_class_t = 0;
pub type xcb_cw_t = ::core::ffi::c_uint;
pub const XCB_CW_CURSOR: xcb_cw_t = 16384;
pub const XCB_CW_COLORMAP: xcb_cw_t = 8192;
pub const XCB_CW_DONT_PROPAGATE: xcb_cw_t = 4096;
pub const XCB_CW_EVENT_MASK: xcb_cw_t = 2048;
pub const XCB_CW_SAVE_UNDER: xcb_cw_t = 1024;
pub const XCB_CW_OVERRIDE_REDIRECT: xcb_cw_t = 512;
pub const XCB_CW_BACKING_PIXEL: xcb_cw_t = 256;
pub const XCB_CW_BACKING_PLANES: xcb_cw_t = 128;
pub const XCB_CW_BACKING_STORE: xcb_cw_t = 64;
pub const XCB_CW_WIN_GRAVITY: xcb_cw_t = 32;
pub const XCB_CW_BIT_GRAVITY: xcb_cw_t = 16;
pub const XCB_CW_BORDER_PIXEL: xcb_cw_t = 8;
pub const XCB_CW_BORDER_PIXMAP: xcb_cw_t = 4;
pub const XCB_CW_BACK_PIXEL: xcb_cw_t = 2;
pub const XCB_CW_BACK_PIXMAP: xcb_cw_t = 1;
pub type xcb_config_window_t = ::core::ffi::c_uint;
pub const XCB_CONFIG_WINDOW_STACK_MODE: xcb_config_window_t = 64;
pub const XCB_CONFIG_WINDOW_SIBLING: xcb_config_window_t = 32;
pub const XCB_CONFIG_WINDOW_BORDER_WIDTH: xcb_config_window_t = 16;
pub const XCB_CONFIG_WINDOW_HEIGHT: xcb_config_window_t = 8;
pub const XCB_CONFIG_WINDOW_WIDTH: xcb_config_window_t = 4;
pub const XCB_CONFIG_WINDOW_Y: xcb_config_window_t = 2;
pub const XCB_CONFIG_WINDOW_X: xcb_config_window_t = 1;
pub type xcb_circulate_t = ::core::ffi::c_uint;
pub const XCB_CIRCULATE_LOWER_HIGHEST: xcb_circulate_t = 1;
pub const XCB_CIRCULATE_RAISE_LOWEST: xcb_circulate_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_charinfo_t {
    pub left_side_bearing: int16_t,
    pub right_side_bearing: int16_t,
    pub character_width: int16_t,
    pub ascent: int16_t,
    pub descent: int16_t,
    pub attributes: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_query_font_reply_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub min_bounds: xcb_charinfo_t,
    pub pad1: [uint8_t; 4],
    pub max_bounds: xcb_charinfo_t,
    pub pad2: [uint8_t; 4],
    pub min_char_or_byte2: uint16_t,
    pub max_char_or_byte2: uint16_t,
    pub default_char: uint16_t,
    pub properties_len: uint16_t,
    pub draw_direction: uint8_t,
    pub min_byte1: uint8_t,
    pub max_byte1: uint8_t,
    pub all_chars_exist: uint8_t,
    pub font_ascent: int16_t,
    pub font_descent: int16_t,
    pub char_infos_len: uint32_t,
}
pub type PangoFontDescription = _PangoFontDescription;
pub type cairo_t = _cairo;
pub type cairo_surface_t = _cairo_surface;
pub type i3String = _i3String;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Font {
    pub type_0: C2RustUnnamed_1,
    pub height: ::core::ffi::c_int,
    pub pattern: *mut ::core::ffi::c_char,
    pub specific: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub xcb: C2RustUnnamed_0,
    pub pango_desc: *mut PangoFontDescription,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub id: xcb_font_t,
    pub info: *mut xcb_query_font_reply_t,
    pub table: *mut xcb_charinfo_t,
}
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const FONT_TYPE_PANGO: C2RustUnnamed_1 = 2;
pub const FONT_TYPE_XCB: C2RustUnnamed_1 = 1;
pub const FONT_TYPE_NONE: C2RustUnnamed_1 = 0;
pub type i3Font = Font;
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
    pub type_0: C2RustUnnamed_18,
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
    pub floating: C2RustUnnamed_7,
    pub nodes: C2RustUnnamed_6,
    pub focused: C2RustUnnamed_5,
    pub all_cons: C2RustUnnamed_4,
    pub floating_windows: C2RustUnnamed_3,
    pub on_remove_child: Option<unsafe extern "C" fn(*mut Con) -> ()>,
    pub scratchpad_state: C2RustUnnamed_2,
    pub old_id: ::core::ffi::c_int,
    pub depth: uint16_t,
    pub colormap: xcb_colormap_t,
}
pub type C2RustUnnamed_2 = ::core::ffi::c_uint;
pub const SCRATCHPAD_CHANGED: C2RustUnnamed_2 = 2;
pub const SCRATCHPAD_FRESH: C2RustUnnamed_2 = 1;
pub const SCRATCHPAD_NONE: C2RustUnnamed_2 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub tqe_next: *mut Con,
    pub tqe_prev: *mut *mut Con,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub tqe_next: *mut Con,
    pub tqe_prev: *mut *mut Con,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub tqe_next: *mut Con,
    pub tqe_prev: *mut *mut Con,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub tqe_next: *mut Con,
    pub tqe_prev: *mut *mut Con,
}
pub type C2RustUnnamed_7 = ::core::ffi::c_uint;
pub const FLOATING_USER_ON: C2RustUnnamed_7 = 3;
pub const FLOATING_AUTO_ON: C2RustUnnamed_7 = 2;
pub const FLOATING_USER_OFF: C2RustUnnamed_7 = 1;
pub const FLOATING_AUTO_OFF: C2RustUnnamed_7 = 0;
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
    pub urgent: C2RustUnnamed_12,
    pub dock: C2RustUnnamed_11,
    pub id: xcb_window_t,
    pub window_mode: C2RustUnnamed_10,
    pub con_id: *mut Con,
    pub match_all_windows: bool,
    pub insert_where: C2RustUnnamed_9,
    pub matches: C2RustUnnamed_8,
    pub restart_mode: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub tqe_next: *mut Match,
    pub tqe_prev: *mut *mut Match,
}
pub type C2RustUnnamed_9 = ::core::ffi::c_uint;
pub const M_BELOW: C2RustUnnamed_9 = 2;
pub const M_ASSIGN_WS: C2RustUnnamed_9 = 1;
pub const M_HERE: C2RustUnnamed_9 = 0;
pub type C2RustUnnamed_10 = ::core::ffi::c_uint;
pub const WM_FLOATING: C2RustUnnamed_10 = 6;
pub const WM_FLOATING_USER: C2RustUnnamed_10 = 5;
pub const WM_FLOATING_AUTO: C2RustUnnamed_10 = 4;
pub const WM_TILING: C2RustUnnamed_10 = 3;
pub const WM_TILING_USER: C2RustUnnamed_10 = 2;
pub const WM_TILING_AUTO: C2RustUnnamed_10 = 1;
pub const WM_ANY: C2RustUnnamed_10 = 0;
pub type C2RustUnnamed_11 = ::core::ffi::c_int;
pub const M_DOCK_BOTTOM: C2RustUnnamed_11 = 3;
pub const M_DOCK_TOP: C2RustUnnamed_11 = 2;
pub const M_DOCK_ANY: C2RustUnnamed_11 = 1;
pub const M_NODOCK: C2RustUnnamed_11 = 0;
pub const M_DONTCHECK: C2RustUnnamed_11 = -1;
pub type C2RustUnnamed_12 = ::core::ffi::c_int;
pub const U_OLDEST: C2RustUnnamed_12 = 1;
pub const U_LATEST: C2RustUnnamed_12 = 0;
pub const U_DONTCHECK: C2RustUnnamed_12 = -1;
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
    pub dock: C2RustUnnamed_13,
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
pub type C2RustUnnamed_13 = ::core::ffi::c_uint;
pub const W_DOCK_BOTTOM: C2RustUnnamed_13 = 2;
pub const W_DOCK_TOP: C2RustUnnamed_13 = 1;
pub const W_NODOCK: C2RustUnnamed_13 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Assignment {
    pub type_0: C2RustUnnamed_16,
    pub match_0: Match,
    pub dest: C2RustUnnamed_15,
    pub assignments: C2RustUnnamed_14,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub tqe_next: *mut Assignment,
    pub tqe_prev: *mut *mut Assignment,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_15 {
    pub command: *mut ::core::ffi::c_char,
    pub workspace: *mut ::core::ffi::c_char,
    pub output: *mut ::core::ffi::c_char,
}
pub type C2RustUnnamed_16 = ::core::ffi::c_uint;
pub const A_TO_OUTPUT: C2RustUnnamed_16 = 16;
pub const A_TO_WORKSPACE_NUMBER: C2RustUnnamed_16 = 8;
pub const A_NO_FOCUS: C2RustUnnamed_16 = 4;
pub const A_TO_WORKSPACE: C2RustUnnamed_16 = 2;
pub const A_COMMAND: C2RustUnnamed_16 = 1;
pub const A_ANY: C2RustUnnamed_16 = 0;
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
    pub marks: C2RustUnnamed_17,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_17 {
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
pub type C2RustUnnamed_18 = ::core::ffi::c_uint;
pub const CT_DOCKAREA: C2RustUnnamed_18 = 5;
pub const CT_WORKSPACE: C2RustUnnamed_18 = 4;
pub const CT_FLOATING_CON: C2RustUnnamed_18 = 3;
pub const CT_CON: C2RustUnnamed_18 = 2;
pub const CT_OUTPUT: C2RustUnnamed_18 = 1;
pub const CT_ROOT: C2RustUnnamed_18 = 0;
pub type direction_t = ::core::ffi::c_uint;
pub const D_DOWN: direction_t = 3;
pub const D_UP: direction_t = 2;
pub const D_RIGHT: direction_t = 1;
pub const D_LEFT: direction_t = 0;
pub type orientation_t = ::core::ffi::c_uint;
pub const VERT: orientation_t = 2;
pub const HORIZ: orientation_t = 1;
pub const NO_ORIENTATION: orientation_t = 0;
pub type smart_gaps_t = ::core::ffi::c_uint;
pub const SMART_GAPS_INVERSE_OUTER: smart_gaps_t = 2;
pub const SMART_GAPS_ON: smart_gaps_t = 1;
pub const SMART_GAPS_OFF: smart_gaps_t = 0;
pub type hide_edge_borders_mode_t = ::core::ffi::c_uint;
pub const HEBM_SMART_NO_GAPS: hide_edge_borders_mode_t = 64;
pub const HEBM_SMART: hide_edge_borders_mode_t = 32;
pub const HEBM_BOTH: hide_edge_borders_mode_t = 23;
pub const HEBM_HORIZONTAL: hide_edge_borders_mode_t = 20;
pub const HEBM_VERTICAL: hide_edge_borders_mode_t = 3;
pub const HEBM_NONE: hide_edge_borders_mode_t = 0;
pub type warping_t = ::core::ffi::c_uint;
pub const POINTER_WARPING_NONE: warping_t = 1;
pub const POINTER_WARPING_OUTPUT: warping_t = 0;
pub type focus_wrapping_t = ::core::ffi::c_uint;
pub const FOCUS_WRAPPING_WORKSPACE: focus_wrapping_t = 3;
pub const FOCUS_WRAPPING_FORCE: focus_wrapping_t = 2;
pub const FOCUS_WRAPPING_ON: focus_wrapping_t = 1;
pub const FOCUS_WRAPPING_OFF: focus_wrapping_t = 0;
pub type tiling_drag_t = ::core::ffi::c_uint;
pub const TILING_DRAG_MODIFIER_OR_TITLEBAR: tiling_drag_t = 3;
pub const TILING_DRAG_TITLEBAR: tiling_drag_t = 2;
pub const TILING_DRAG_MODIFIER: tiling_drag_t = 1;
pub const TILING_DRAG_OFF: tiling_drag_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Config {
    pub terminal: *const ::core::ffi::c_char,
    pub font: i3Font,
    pub ipc_socket_path: *mut ::core::ffi::c_char,
    pub restart_state_path: *mut ::core::ffi::c_char,
    pub default_layout: layout_t,
    pub container_stack_limit: ::core::ffi::c_int,
    pub container_stack_limit_value: ::core::ffi::c_int,
    pub default_border_width: ::core::ffi::c_int,
    pub default_floating_border_width: ::core::ffi::c_int,
    pub default_orientation: ::core::ffi::c_int,
    pub disable_focus_follows_mouse: bool,
    pub mouse_warping: warping_t,
    pub hide_edge_borders: hide_edge_borders_mode_t,
    pub disable_workspace_bar: bool,
    pub focus_wrapping: focus_wrapping_t,
    pub force_xinerama: bool,
    pub disable_randr15: bool,
    pub fake_outputs: *mut ::core::ffi::c_char,
    pub workspace_auto_back_and_forth: bool,
    pub workspace_urgency_timer: ::core::ffi::c_float,
    pub focus_on_window_activation: C2RustUnnamed_21,
    pub show_marks: bool,
    pub title_align: C2RustUnnamed_20,
    pub default_border: border_style_t,
    pub default_floating_border: border_style_t,
    pub floating_modifier: uint32_t,
    pub swap_modifier: uint32_t,
    pub floating_maximum_width: int32_t,
    pub floating_maximum_height: int32_t,
    pub floating_minimum_width: int32_t,
    pub floating_minimum_height: int32_t,
    pub client: config_client,
    pub bar: config_bar,
    pub popup_during_fullscreen: C2RustUnnamed_19,
    pub number_barconfigs: ::core::ffi::c_int,
    pub tiling_drag: tiling_drag_t,
    pub gaps: gaps_t,
    pub smart_gaps: smart_gaps_t,
}
pub type C2RustUnnamed_19 = ::core::ffi::c_uint;
pub const PDF_ALL: C2RustUnnamed_19 = 3;
pub const PDF_IGNORE: C2RustUnnamed_19 = 2;
pub const PDF_LEAVE_FULLSCREEN: C2RustUnnamed_19 = 1;
pub const PDF_SMART: C2RustUnnamed_19 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct config_bar {
    pub focused: Colortriple,
    pub unfocused: Colortriple,
    pub urgent: Colortriple,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct config_client {
    pub background: color_t,
    pub focused: Colortriple,
    pub focused_inactive: Colortriple,
    pub focused_tab_title: Colortriple,
    pub unfocused: Colortriple,
    pub urgent: Colortriple,
    pub placeholder: Colortriple,
    pub got_focused_tab_title: bool,
}
pub type C2RustUnnamed_20 = ::core::ffi::c_uint;
pub const ALIGN_RIGHT: C2RustUnnamed_20 = 2;
pub const ALIGN_CENTER: C2RustUnnamed_20 = 1;
pub const ALIGN_LEFT: C2RustUnnamed_20 = 0;
pub type C2RustUnnamed_21 = ::core::ffi::c_uint;
pub const FOWA_NONE: C2RustUnnamed_21 = 3;
pub const FOWA_FOCUS: C2RustUnnamed_21 = 2;
pub const FOWA_URGENT: C2RustUnnamed_21 = 1;
pub const FOWA_SMART: C2RustUnnamed_21 = 0;
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
pub struct callback_params {
    pub orientation: orientation_t,
    pub output: *mut Con,
    pub helpwin: xcb_window_t,
    pub new_position: *mut uint32_t,
    pub threshold_exceeded: *mut bool,
}
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const XCB_NONE: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const XCB_COPY_FROM_PARENT: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
unsafe extern "C" fn resize_callback(
    mut con: *mut Con,
    mut old_rect: *mut Rect,
    mut new_x: uint32_t,
    mut new_y: uint32_t,
    mut event: *const xcb_button_press_event_t,
    mut extra: *const ::core::ffi::c_void,
) {
    let mut params: *const callback_params = extra as *const callback_params;
    let mut output: *mut Con = (*params).output;
    debuglog(
        b"%s:%s:%d - new x = %d, y = %d\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/resize.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"resize_callback\0" as *const u8 as *const ::core::ffi::c_char,
        30 as ::core::ffi::c_int,
        new_x,
        new_y,
    );
    if !*(*params).threshold_exceeded {
        xcb_map_window(conn, (*params).helpwin);
        if (*params).orientation as ::core::ffi::c_uint
            == HORIZ as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            xcb_warp_pointer(
                conn,
                XCB_NONE as xcb_window_t,
                (*event).root,
                0 as int16_t,
                0 as int16_t,
                0 as uint16_t,
                0 as uint16_t,
                (*(*params).new_position)
                    .wrapping_add(new_x)
                    .wrapping_sub((*event).root_x as uint32_t) as int16_t,
                new_y as int16_t,
            );
        } else {
            xcb_warp_pointer(
                conn,
                XCB_NONE as xcb_window_t,
                (*event).root,
                0 as int16_t,
                0 as int16_t,
                0 as uint16_t,
                0 as uint16_t,
                new_x as int16_t,
                (*(*params).new_position)
                    .wrapping_add(new_y)
                    .wrapping_sub((*event).root_y as uint32_t) as int16_t,
            );
        }
        *(*params).threshold_exceeded = true_0 != 0;
        return;
    }
    if (*params).orientation as ::core::ffi::c_uint
        == HORIZ as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if new_x
            > (*output)
                .rect
                .x
                .wrapping_add((*output).rect.width)
                .wrapping_sub(25 as uint32_t)
            || new_x < (*output).rect.x.wrapping_add(25 as uint32_t)
        {
            return;
        }
        *(*params).new_position = new_x;
        xcb_configure_window(
            conn,
            (*params).helpwin,
            XCB_CONFIG_WINDOW_X as ::core::ffi::c_int as uint16_t,
            (*params).new_position as *const ::core::ffi::c_void,
        );
    } else {
        if new_y
            > (*output)
                .rect
                .y
                .wrapping_add((*output).rect.height)
                .wrapping_sub(25 as uint32_t)
            || new_y < (*output).rect.y.wrapping_add(25 as uint32_t)
        {
            return;
        }
        *(*params).new_position = new_y;
        xcb_configure_window(
            conn,
            (*params).helpwin,
            XCB_CONFIG_WINDOW_Y as ::core::ffi::c_int as uint16_t,
            (*params).new_position as *const ::core::ffi::c_void,
        );
    }
    xcb_flush(conn);
}
#[no_mangle]
pub unsafe extern "C" fn resize_find_tiling_participants(
    mut current: *mut *mut Con,
    mut other: *mut *mut Con,
    mut direction: direction_t,
    mut both_sides: bool,
) -> bool {
    debuglog(
        b"%s:%s:%d - Find two participants for resizing container=%p in direction=%i\n\0"
            as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/resize.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"resize_find_tiling_participants\0" as *const u8 as *const ::core::ffi::c_char,
        73 as ::core::ffi::c_int,
        other,
        direction as ::core::ffi::c_uint,
    );
    let mut first: *mut Con = *current;
    let mut second: *mut Con = ::core::ptr::null_mut::<Con>();
    if first.is_null() {
        debuglog(
            b"%s:%s:%d - Current container is NULL, aborting.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/resize.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"resize_find_tiling_participants\0" as *const u8 as *const ::core::ffi::c_char,
            77 as ::core::ffi::c_int,
        );
        return false_0 != 0;
    }
    let search_orientation: orientation_t = orientation_from_direction(direction) as orientation_t;
    let dir_backwards: bool = direction as ::core::ffi::c_uint
        == D_UP as ::core::ffi::c_int as ::core::ffi::c_uint
        || direction as ::core::ffi::c_uint == D_LEFT as ::core::ffi::c_int as ::core::ffi::c_uint;
    while (*first).type_0 as ::core::ffi::c_uint
        != CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
        && (*first).type_0 as ::core::ffi::c_uint
            != CT_FLOATING_CON as ::core::ffi::c_int as ::core::ffi::c_uint
        && second.is_null()
    {
        if con_orientation((*first).parent as *mut Con) as ::core::ffi::c_uint
            != search_orientation as ::core::ffi::c_uint
            || (*(*first).parent).layout as ::core::ffi::c_uint
                == L_STACKED as ::core::ffi::c_int as ::core::ffi::c_uint
            || (*(*first).parent).layout as ::core::ffi::c_uint
                == L_TABBED as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            first = (*first).parent as *mut Con;
        } else {
            if dir_backwards {
                second = *(*((*first).nodes.tqe_prev as *mut nodes_head)).tqh_last as *mut Con;
                if second.is_null() && both_sides as ::core::ffi::c_int == true_0 {
                    second = (*first).nodes.tqe_next as *mut Con;
                }
            } else {
                second = (*first).nodes.tqe_next as *mut Con;
                if second.is_null() && both_sides as ::core::ffi::c_int == true_0 {
                    second = *(*((*first).nodes.tqe_prev as *mut nodes_head)).tqh_last as *mut Con;
                }
            }
            if second.is_null() {
                debuglog(
                    b"%s:%s:%d - No second container in this direction found, trying to look further up in the tree...\n\0"
                        as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    b"../src/resize.c\0" as *const u8 as *const ::core::ffi::c_char,
                    b"resize_find_tiling_participants\0" as *const u8
                        as *const ::core::ffi::c_char,
                    110 as ::core::ffi::c_int,
                );
                first = (*first).parent as *mut Con;
            }
        }
    }
    debuglog(
        b"%s:%s:%d - Found participants: first=%p and second=%p.\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/resize.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"resize_find_tiling_participants\0" as *const u8 as *const ::core::ffi::c_char,
        115 as ::core::ffi::c_int,
        first,
        second,
    );
    *current = first;
    *other = second;
    if first.is_null() || second.is_null() {
        debuglog(
            b"%s:%s:%d - Could not find two participants for this resize request.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/resize.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"resize_find_tiling_participants\0" as *const u8 as *const ::core::ffi::c_char,
            119 as ::core::ffi::c_int,
        );
        return false_0 != 0;
    }
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn percent_for_1px(mut con: *mut Con) -> ::core::ffi::c_double {
    let parent_size: ::core::ffi::c_int =
        con_rect_size_in_orientation((*con).parent as *mut Con) as ::core::ffi::c_int;
    let min_size: ::core::ffi::c_int = (if con_orientation((*con).parent as *mut Con)
        as ::core::ffi::c_uint
        == HORIZ as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        1 as uint32_t
    } else {
        (1 as uint32_t).wrapping_add((*con).deco_rect.height)
    }) as ::core::ffi::c_int;
    return min_size as ::core::ffi::c_double / parent_size as ::core::ffi::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn resize_neighboring_cons(
    mut first: *mut Con,
    mut second: *mut Con,
    mut px: ::core::ffi::c_int,
    mut ppt: ::core::ffi::c_int,
) -> bool {
    if px * ppt == 0 as ::core::ffi::c_int {
    } else {
        __assert_fail(
            b"px * ppt == 0\0" as *const u8 as *const ::core::ffi::c_char,
            b"../src/resize.c\0" as *const u8 as *const ::core::ffi::c_char,
            147 as ::core::ffi::c_uint,
            b"_Bool resize_neighboring_cons(Con *, Con *, int, int)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    };
    let mut parent: *mut Con = (*first).parent as *mut Con;
    let mut new_first_percent: ::core::ffi::c_double = 0.;
    let mut new_second_percent: ::core::ffi::c_double = 0.;
    if ppt != 0 {
        new_first_percent = (*first).percent + ppt as ::core::ffi::c_double / 100.0f64;
        new_second_percent = (*second).percent - ppt as ::core::ffi::c_double / 100.0f64;
    } else {
        let pct: ::core::ffi::c_double = px as ::core::ffi::c_double
            / con_rect_size_in_orientation((*first).parent as *mut Con) as ::core::ffi::c_double;
        new_first_percent = (*first).percent + pct;
        new_second_percent = (*second).percent - pct;
    }
    if new_first_percent < percent_for_1px(first) || new_second_percent < percent_for_1px(second) {
        return false_0 != 0;
    }
    (*first).percent = new_first_percent;
    (*second).percent = new_second_percent;
    con_fix_percent(parent);
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn resize_graphical_handler(
    mut first: *mut Con,
    mut second: *mut Con,
    mut orientation: orientation_t,
    mut event: *const xcb_button_press_event_t,
    mut use_threshold: bool,
) {
    let mut output: *mut Con = con_get_output(first);
    debuglog(
        b"%s:%s:%d - x = %d, width = %d\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/resize.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"resize_graphical_handler\0" as *const u8 as *const ::core::ffi::c_char,
        177 as ::core::ffi::c_int,
        (*output).rect.x,
        (*output).rect.width,
    );
    debuglog(
        b"%s:%s:%d - first = %p / %s\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/resize.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"resize_graphical_handler\0" as *const u8 as *const ::core::ffi::c_char,
        178 as ::core::ffi::c_int,
        first,
        (*first).name,
    );
    debuglog(
        b"%s:%s:%d - second = %p / %s\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/resize.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"resize_graphical_handler\0" as *const u8 as *const ::core::ffi::c_char,
        179 as ::core::ffi::c_int,
        second,
        (*second).name,
    );
    x_mask_event_mask(!(XCB_EVENT_MASK_ENTER_WINDOW as ::core::ffi::c_int) as uint32_t);
    xcb_flush(conn);
    let mut mask: uint32_t = 0 as uint32_t;
    let mut values: [uint32_t; 2] = [0; 2];
    mask = XCB_CW_OVERRIDE_REDIRECT as ::core::ffi::c_int as uint32_t;
    values[0 as ::core::ffi::c_int as usize] = 1 as uint32_t;
    let mut grabwin: xcb_window_t = create_window(
        conn,
        (*output).rect as Rect,
        XCB_COPY_FROM_PARENT as uint16_t,
        XCB_COPY_FROM_PARENT as xcb_visualid_t,
        XCB_WINDOW_CLASS_INPUT_ONLY as ::core::ffi::c_int as uint16_t,
        XCURSOR_CURSOR_POINTER,
        true_0 != 0,
        mask,
        &raw mut values as *mut uint32_t,
    );
    let mut initial_position: uint32_t = 0;
    let mut new_position: uint32_t = 0;
    let mut helprect: Rect = Rect {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    helprect.x = (*second).rect.x;
    helprect.y = (*second).rect.y;
    let mut ffirst: *mut Con = con_descend_focused(first);
    let mut fsecond: *mut Con = con_descend_focused(second);
    if orientation as ::core::ffi::c_uint == HORIZ as ::core::ffi::c_int as ::core::ffi::c_uint {
        helprect.width = logical_px(2 as ::core::ffi::c_int) as uint32_t;
        helprect.height = (*second).rect.height;
        let ffirst_right: uint32_t = (*ffirst).rect.x.wrapping_add((*ffirst).rect.width);
        let gap: uint32_t = (*fsecond).rect.x.wrapping_sub(ffirst_right);
        let middle: uint32_t = (*fsecond)
            .rect
            .x
            .wrapping_sub(gap.wrapping_div(2 as uint32_t));
        debuglog(
            b"%s:%s:%d - ffirst->rect = {.x = %u, .width = %u}\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/resize.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"resize_graphical_handler\0" as *const u8 as *const ::core::ffi::c_char,
            215 as ::core::ffi::c_int,
            (*ffirst).rect.x,
            (*ffirst).rect.width,
        );
        debuglog(
            b"%s:%s:%d - fsecond->rect = {.x = %u, .width = %u}\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/resize.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"resize_graphical_handler\0" as *const u8 as *const ::core::ffi::c_char,
            216 as ::core::ffi::c_int,
            (*fsecond).rect.x,
            (*fsecond).rect.width,
        );
        debuglog(
            b"%s:%s:%d - gap = %u, middle = %u\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b"../src/resize.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"resize_graphical_handler\0" as *const u8 as *const ::core::ffi::c_char,
            217 as ::core::ffi::c_int,
            gap,
            middle,
        );
        initial_position = middle;
    } else {
        helprect.width = (*second).rect.width;
        helprect.height = logical_px(2 as ::core::ffi::c_int) as uint32_t;
        let ffirst_bottom: uint32_t = (*ffirst).rect.y.wrapping_add((*ffirst).rect.height);
        let gap_0: uint32_t = (*fsecond).rect.y.wrapping_sub(ffirst_bottom);
        let middle_0: uint32_t = (*fsecond)
            .rect
            .y
            .wrapping_sub(gap_0.wrapping_div(2 as uint32_t));
        debuglog(
            b"%s:%s:%d - ffirst->rect = {.y = %u, .height = %u}\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/resize.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"resize_graphical_handler\0" as *const u8 as *const ::core::ffi::c_char,
            225 as ::core::ffi::c_int,
            (*ffirst).rect.y,
            (*ffirst).rect.height,
        );
        debuglog(
            b"%s:%s:%d - fsecond->rect = {.y = %u, .height = %u}\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/resize.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"resize_graphical_handler\0" as *const u8 as *const ::core::ffi::c_char,
            226 as ::core::ffi::c_int,
            (*fsecond).rect.y,
            (*fsecond).rect.height,
        );
        debuglog(
            b"%s:%s:%d - gap = %u, middle = %u\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b"../src/resize.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"resize_graphical_handler\0" as *const u8 as *const ::core::ffi::c_char,
            227 as ::core::ffi::c_int,
            gap_0,
            middle_0,
        );
        initial_position = middle_0;
    }
    mask = XCB_CW_BACK_PIXEL as ::core::ffi::c_int as uint32_t;
    values[0 as ::core::ffi::c_int as usize] = config.client.focused.border.colorpixel;
    mask |= XCB_CW_OVERRIDE_REDIRECT as ::core::ffi::c_int as uint32_t;
    values[1 as ::core::ffi::c_int as usize] = 1 as uint32_t;
    let mut helpwin: xcb_window_t = create_window(
        conn,
        helprect,
        XCB_COPY_FROM_PARENT as uint16_t,
        XCB_COPY_FROM_PARENT as xcb_visualid_t,
        XCB_WINDOW_CLASS_INPUT_OUTPUT as ::core::ffi::c_int as uint16_t,
        (if orientation as ::core::ffi::c_uint == HORIZ as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            XCURSOR_CURSOR_RESIZE_HORIZONTAL as ::core::ffi::c_int
        } else {
            XCURSOR_CURSOR_RESIZE_VERTICAL as ::core::ffi::c_int
        }) as xcursor_cursor_t,
        false_0 != 0,
        mask,
        &raw mut values as *mut uint32_t,
    );
    if !use_threshold {
        xcb_map_window(conn, helpwin);
        if orientation as ::core::ffi::c_uint == HORIZ as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            xcb_warp_pointer(
                conn,
                XCB_NONE as xcb_window_t,
                (*event).root,
                0 as int16_t,
                0 as int16_t,
                0 as uint16_t,
                0 as uint16_t,
                initial_position as int16_t,
                (*event).root_y,
            );
        } else {
            xcb_warp_pointer(
                conn,
                XCB_NONE as xcb_window_t,
                (*event).root,
                0 as int16_t,
                0 as int16_t,
                0 as uint16_t,
                0 as uint16_t,
                (*event).root_x,
                initial_position as int16_t,
            );
        }
    }
    xcb_circulate_window(
        conn,
        XCB_CIRCULATE_RAISE_LOWEST as ::core::ffi::c_int as uint8_t,
        helpwin,
    );
    xcb_flush(conn);
    new_position = initial_position;
    let mut threshold_exceeded: bool = !use_threshold;
    let params: callback_params = callback_params {
        orientation: orientation,
        output: output,
        helpwin: helpwin,
        new_position: &raw mut new_position,
        threshold_exceeded: &raw mut threshold_exceeded,
    };
    tree_render();
    let mut drag_result: drag_result_t = drag_pointer(
        ::core::ptr::null_mut::<Con>(),
        event,
        grabwin,
        0 as ::core::ffi::c_int,
        use_threshold,
        Some(
            resize_callback
                as unsafe extern "C" fn(
                    *mut Con,
                    *mut Rect,
                    uint32_t,
                    uint32_t,
                    *const xcb_button_press_event_t,
                    *const ::core::ffi::c_void,
                ) -> (),
        ),
        &raw const params as *const ::core::ffi::c_void,
    );
    xcb_destroy_window(conn, helpwin);
    xcb_destroy_window(conn, grabwin);
    xcb_flush(conn);
    if drag_result as ::core::ffi::c_uint
        == DRAG_REVERT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return;
    }
    let mut pixels: ::core::ffi::c_int =
        new_position.wrapping_sub(initial_position) as ::core::ffi::c_int;
    debuglog(
        b"%s:%s:%d - Done, pixels = %d\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/resize.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"resize_graphical_handler\0" as *const u8 as *const ::core::ffi::c_char,
        279 as ::core::ffi::c_int,
        pixels,
    );
    if pixels == 0 as ::core::ffi::c_int {
        return;
    }
    if (*first).percent > 0.0f64 {
    } else {
        __assert_fail(
            b"first->percent > 0.0\0" as *const u8 as *const ::core::ffi::c_char,
            b"../src/resize.c\0" as *const u8 as *const ::core::ffi::c_char,
            287 as ::core::ffi::c_uint,
            b"void resize_graphical_handler(Con *, Con *, orientation_t, const xcb_button_press_event_t *, _Bool)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
    };
    if (*second).percent > 0.0f64 {
    } else {
        __assert_fail(
            b"second->percent > 0.0\0" as *const u8 as *const ::core::ffi::c_char,
            b"../src/resize.c\0" as *const u8 as *const ::core::ffi::c_char,
            288 as ::core::ffi::c_uint,
            b"void resize_graphical_handler(Con *, Con *, orientation_t, const xcb_button_press_event_t *, _Bool)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
    };
    let result: bool =
        resize_neighboring_cons(first, second, pixels, 0 as ::core::ffi::c_int) as bool;
    debuglog(
        b"%s:%s:%d - Graphical resize %s: first->percent = %f, second->percent = %f.\n\0"
            as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/resize.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"resize_graphical_handler\0" as *const u8 as *const ::core::ffi::c_char,
        291 as ::core::ffi::c_int,
        if result as ::core::ffi::c_int != 0 {
            b"successful\0" as *const u8 as *const ::core::ffi::c_char
        } else {
            b"failed\0" as *const u8 as *const ::core::ffi::c_char
        },
        (*first).percent,
        (*second).percent,
    );
}
