/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * restore_layout.c: Everything for restored containers that is not pure state
 *                   parsing (which can be found in load_layout.c).
 *
 *
 */
// Mechanical file-by-file Rust translation of src/restore_layout.c.
// Generated with C2Rust 0.22.1; C ABI and original ownership retained.
extern "C" {
    pub type xcb_connection_t;
    pub type _PangoFontDescription;
    pub type _cairo;
    pub type _cairo_surface;
    pub type _i3String;
    pub type pcre2_real_code_8;
    pub type ev_loop;
    fn errx(__status: ::core::ffi::c_int, __format: *const ::core::ffi::c_char, ...) -> !;
    fn xcb_destroy_window(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t;
    fn xcb_change_property(
        c: *mut xcb_connection_t,
        mode: uint8_t,
        window: xcb_window_t,
        property: xcb_atom_t,
        type_0: xcb_atom_t,
        format: uint8_t,
        data_len: uint32_t,
        data: *const ::core::ffi::c_void,
    ) -> xcb_void_cookie_t;
    fn xcb_flush(c: *mut xcb_connection_t) -> ::core::ffi::c_int;
    fn xcb_poll_for_event(c: *mut xcb_connection_t) -> *mut xcb_generic_event_t;
    fn xcb_get_file_descriptor(c: *mut xcb_connection_t) -> ::core::ffi::c_int;
    fn xcb_connection_has_error(c: *mut xcb_connection_t) -> ::core::ffi::c_int;
    fn xcb_disconnect(c: *mut xcb_connection_t);
    fn xcb_connect(
        displayname: *const ::core::ffi::c_char,
        screenp: *mut ::core::ffi::c_int,
    ) -> *mut xcb_connection_t;
    fn xcb_icccm_wm_hints_set_input(hints: *mut xcb_icccm_wm_hints_t, input: uint8_t);
    fn xcb_icccm_wm_hints_set_none(hints: *mut xcb_icccm_wm_hints_t);
    fn xcb_icccm_set_wm_hints(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        hints: *mut xcb_icccm_wm_hints_t,
    ) -> xcb_void_cookie_t;
    fn xcb_aux_sync(c: *mut xcb_connection_t);
    static mut root_screen: *mut xcb_screen_t;
    fn errorlog(fmt: *mut ::core::ffi::c_char, ...);
    fn debuglog(fmt: *mut ::core::ffi::c_char, ...);
    fn smalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn scalloc(num: size_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn sasprintf(
        strp: *mut *mut ::core::ffi::c_char,
        fmt: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn i3string_from_utf8(from_utf8: *const ::core::ffi::c_char) -> *mut i3String;
    fn i3string_free(str: *mut i3String);
    fn predict_text_width(text: *mut i3String) -> ::core::ffi::c_int;
    fn get_visualtype(screen: *mut xcb_screen_t) -> *mut xcb_visualtype_t;
    fn logical_px(logical: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn draw_util_surface_init(
        conn: *mut xcb_connection_t,
        surface: *mut surface_t,
        drawable: xcb_drawable_t,
        visual: *mut xcb_visualtype_t,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
    );
    fn draw_util_surface_set_size(
        surface: *mut surface_t,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
    );
    fn draw_util_surface_free(conn: *mut xcb_connection_t, surface: *mut surface_t);
    fn draw_util_text(
        text: *mut i3String,
        surface: *mut surface_t,
        fg_color: color_t,
        bg_color: color_t,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
        max_width: ::core::ffi::c_int,
    );
    fn draw_util_clear_surface(surface: *mut surface_t, color: color_t);
    fn ev_io_start(loop_0: *mut ev_loop, w: *mut ev_io);
    fn ev_io_stop(loop_0: *mut ev_loop, w: *mut ev_io);
    fn ev_prepare_start(loop_0: *mut ev_loop, w: *mut ev_prepare);
    fn ev_prepare_stop(loop_0: *mut ev_loop, w: *mut ev_prepare);
    static mut main_loop: *mut ev_loop;
    static mut config: Config;
    static mut A__NET_WM_NAME: xcb_atom_t;
    static mut A_UTF8_STRING: xcb_atom_t;
    fn create_window(
        conn: *mut xcb_connection_t,
        r: Rect,
        depth: uint16_t,
        visual: xcb_visualid_t,
        window_class: uint16_t,
        cursor: xcursor_cursor_t,
        map: bool,
        mask: uint32_t,
        values: *mut uint32_t,
    ) -> xcb_window_t;
    fn con_is_leaf(con: *mut Con) -> bool;
    fn match_init(match_0: *mut Match);
    fn memmove(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
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
pub type xcb_pixmap_t = uint32_t;
pub type xcb_font_t = uint32_t;
pub type xcb_gcontext_t = uint32_t;
pub type xcb_colormap_t = uint32_t;
pub type xcb_atom_t = uint32_t;
pub type xcb_drawable_t = uint32_t;
pub type xcb_visualid_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_visualtype_t {
    pub visual_id: xcb_visualid_t,
    pub _class: uint8_t,
    pub bits_per_rgb_value: uint8_t,
    pub colormap_entries: uint16_t,
    pub red_mask: uint32_t,
    pub green_mask: uint32_t,
    pub blue_mask: uint32_t,
    pub pad0: [uint8_t; 4],
}
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
pub struct xcb_screen_t {
    pub root: xcb_window_t,
    pub default_colormap: xcb_colormap_t,
    pub white_pixel: uint32_t,
    pub black_pixel: uint32_t,
    pub current_input_masks: uint32_t,
    pub width_in_pixels: uint16_t,
    pub height_in_pixels: uint16_t,
    pub width_in_millimeters: uint16_t,
    pub height_in_millimeters: uint16_t,
    pub min_installed_maps: uint16_t,
    pub max_installed_maps: uint16_t,
    pub root_visual: xcb_visualid_t,
    pub backing_stores: uint8_t,
    pub save_unders: uint8_t,
    pub root_depth: uint8_t,
    pub allowed_depths_len: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_expose_event_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub window: xcb_window_t,
    pub x: uint16_t,
    pub y: uint16_t,
    pub width: uint16_t,
    pub height: uint16_t,
    pub count: uint16_t,
    pub pad1: [uint8_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_configure_notify_event_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub event: xcb_window_t,
    pub window: xcb_window_t,
    pub above_sibling: xcb_window_t,
    pub x: int16_t,
    pub y: int16_t,
    pub width: uint16_t,
    pub height: uint16_t,
    pub border_width: uint16_t,
    pub override_redirect: uint8_t,
    pub pad1: uint8_t,
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
pub type xcb_prop_mode_t = ::core::ffi::c_uint;
pub const XCB_PROP_MODE_APPEND: xcb_prop_mode_t = 2;
pub const XCB_PROP_MODE_PREPEND: xcb_prop_mode_t = 1;
pub const XCB_PROP_MODE_REPLACE: xcb_prop_mode_t = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_icccm_wm_hints_t {
    pub flags: int32_t,
    pub input: uint32_t,
    pub initial_state: int32_t,
    pub icon_pixmap: xcb_pixmap_t,
    pub icon_window: xcb_window_t,
    pub icon_x: int32_t,
    pub icon_y: int32_t,
    pub icon_mask: xcb_pixmap_t,
    pub window_group: xcb_window_t,
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
pub type C2RustUnnamed_19 = ::core::ffi::c_int;
pub const EV_ERROR: C2RustUnnamed_19 = -2147483648;
pub const EV_CUSTOM: C2RustUnnamed_19 = 16777216;
pub const EV_ASYNC: C2RustUnnamed_19 = 524288;
pub const EV_CLEANUP: C2RustUnnamed_19 = 262144;
pub const EV_FORK: C2RustUnnamed_19 = 131072;
pub const EV_EMBED: C2RustUnnamed_19 = 65536;
pub const EV_CHECK: C2RustUnnamed_19 = 32768;
pub const EV_PREPARE: C2RustUnnamed_19 = 16384;
pub const EV_IDLE: C2RustUnnamed_19 = 8192;
pub const EV_STAT: C2RustUnnamed_19 = 4096;
pub const EV_CHILD: C2RustUnnamed_19 = 2048;
pub const EV_SIGNAL: C2RustUnnamed_19 = 1024;
pub const EV_PERIODIC: C2RustUnnamed_19 = 512;
pub const EV_TIMEOUT: C2RustUnnamed_19 = 256;
pub const EV_TIMER: C2RustUnnamed_19 = 256;
pub const EV_IO: C2RustUnnamed_19 = 1;
pub const EV__IOFDSET: C2RustUnnamed_19 = 128;
pub const EV_WRITE: C2RustUnnamed_19 = 2;
pub const EV_READ: C2RustUnnamed_19 = 1;
pub const EV_NONE: C2RustUnnamed_19 = 0;
pub const EV_UNDEF: C2RustUnnamed_19 = -1;
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
pub struct ev_watcher_list {
    pub active: ::core::ffi::c_int,
    pub pending: ::core::ffi::c_int,
    pub priority: ::core::ffi::c_int,
    pub data: *mut ::core::ffi::c_void,
    pub cb:
        Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_watcher_list, ::core::ffi::c_int) -> ()>,
    pub next: *mut ev_watcher_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ev_io {
    pub active: ::core::ffi::c_int,
    pub pending: ::core::ffi::c_int,
    pub priority: ::core::ffi::c_int,
    pub data: *mut ::core::ffi::c_void,
    pub cb: Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_io, ::core::ffi::c_int) -> ()>,
    pub next: *mut ev_watcher_list,
    pub fd: ::core::ffi::c_int,
    pub events: ::core::ffi::c_int,
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
    pub focus_on_window_activation: C2RustUnnamed_22,
    pub show_marks: bool,
    pub title_align: C2RustUnnamed_21,
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
    pub popup_during_fullscreen: C2RustUnnamed_20,
    pub number_barconfigs: ::core::ffi::c_int,
    pub tiling_drag: tiling_drag_t,
    pub gaps: gaps_t,
    pub smart_gaps: smart_gaps_t,
}
pub type C2RustUnnamed_20 = ::core::ffi::c_uint;
pub const PDF_ALL: C2RustUnnamed_20 = 3;
pub const PDF_IGNORE: C2RustUnnamed_20 = 2;
pub const PDF_LEAVE_FULLSCREEN: C2RustUnnamed_20 = 1;
pub const PDF_SMART: C2RustUnnamed_20 = 0;
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
pub type C2RustUnnamed_21 = ::core::ffi::c_uint;
pub const ALIGN_RIGHT: C2RustUnnamed_21 = 2;
pub const ALIGN_CENTER: C2RustUnnamed_21 = 1;
pub const ALIGN_LEFT: C2RustUnnamed_21 = 0;
pub type C2RustUnnamed_22 = ::core::ffi::c_uint;
pub const FOWA_NONE: C2RustUnnamed_22 = 3;
pub const FOWA_FOCUS: C2RustUnnamed_22 = 2;
pub const FOWA_URGENT: C2RustUnnamed_22 = 1;
pub const FOWA_SMART: C2RustUnnamed_22 = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct placeholder_state {
    pub window: xcb_window_t,
    pub con: *mut Con,
    pub rect: Rect,
    pub surface: surface_t,
    pub state: C2RustUnnamed_23,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_23 {
    pub tqe_next: *mut placeholder_state,
    pub tqe_prev: *mut *mut placeholder_state,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct state_head {
    pub tqh_first: *mut placeholder_state,
    pub tqh_last: *mut *mut placeholder_state,
}
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const XCB_EXPOSE: ::core::ffi::c_int = 12;
pub const XCB_CONFIGURE_NOTIFY: ::core::ffi::c_int = 22;
pub const XCB_NONE: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const XCB_COPY_FROM_PARENT: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
static mut state_head: state_head = state_head {
    tqh_first: ::core::ptr::null::<placeholder_state>() as *mut placeholder_state,
    tqh_last: ::core::ptr::null::<*mut placeholder_state>() as *mut *mut placeholder_state,
};
static mut restore_conn: *mut xcb_connection_t =
    ::core::ptr::null::<xcb_connection_t>() as *mut xcb_connection_t;
static mut xcb_watcher: *mut ev_io = ::core::ptr::null::<ev_io>() as *mut ev_io;
static mut xcb_prepare: *mut ev_prepare = ::core::ptr::null::<ev_prepare>() as *mut ev_prepare;
unsafe extern "C" fn restore_xcb_got_event(
    mut loop_0: *mut ev_loop,
    mut w: *mut ev_io,
    mut revents: ::core::ffi::c_int,
) {
}
unsafe extern "C" fn restore_xcb_prepare_cb(
    mut loop_0: *mut ev_loop,
    mut w: *mut ev_prepare,
    mut revents: ::core::ffi::c_int,
) {
    let mut event: *mut xcb_generic_event_t = ::core::ptr::null_mut::<xcb_generic_event_t>();
    if xcb_connection_has_error(restore_conn) != 0 {
        debuglog(
            b"%s:%s:%d - restore X11 connection has an error, reconnecting\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/restore_layout.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"restore_xcb_prepare_cb\0" as *const u8 as *const ::core::ffi::c_char,
            53 as ::core::ffi::c_int,
        );
        restore_connect();
        return;
    }
    loop {
        event = xcb_poll_for_event(restore_conn);
        if event.is_null() {
            break;
        }
        if (*event).response_type as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            let mut error: *mut xcb_generic_error_t = event as *mut xcb_generic_error_t;
            debuglog(
                b"%s:%s:%d - X11 Error received (probably harmless)! sequence 0x%x, error_code = %d\n\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"../src/restore_layout.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"restore_xcb_prepare_cb\0" as *const u8 as *const ::core::ffi::c_char,
                62 as ::core::ffi::c_int,
                (*error).sequence as ::core::ffi::c_int,
                (*error).error_code as ::core::ffi::c_int,
            );
            free(event as *mut ::core::ffi::c_void);
        } else {
            let mut type_0: ::core::ffi::c_int =
                (*event).response_type as ::core::ffi::c_int & 0x7f as ::core::ffi::c_int;
            restore_handle_event(type_0, event);
            free(event as *mut ::core::ffi::c_void);
        }
    }
    xcb_flush(restore_conn);
}
#[no_mangle]
pub unsafe extern "C" fn restore_connect() {
    if !restore_conn.is_null() {
        ev_io_stop(main_loop, xcb_watcher);
        ev_prepare_stop(main_loop, xcb_prepare);
        while !state_head.tqh_first.is_null() {
            let mut state: *mut placeholder_state = state_head.tqh_first as *mut placeholder_state;
            if !(*state).state.tqe_next.is_null() {
                (*(*state).state.tqe_next).state.tqe_prev = (*state).state.tqe_prev;
            } else {
                state_head.tqh_last = (*state).state.tqe_prev;
            }
            *(*state).state.tqe_prev = (*state).state.tqe_next;
            free(state as *mut ::core::ffi::c_void);
        }
        xcb_disconnect(restore_conn);
        free(xcb_watcher as *mut ::core::ffi::c_void);
        free(xcb_prepare as *mut ::core::ffi::c_void);
    }
    let mut screen: ::core::ffi::c_int = 0;
    restore_conn = xcb_connect(::core::ptr::null::<::core::ffi::c_char>(), &raw mut screen);
    if restore_conn.is_null() || xcb_connection_has_error(restore_conn) != 0 {
        if !restore_conn.is_null() {
            xcb_disconnect(restore_conn);
        }
        errx(
            EXIT_FAILURE,
            b"Cannot open display\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    xcb_watcher = scalloc(1 as size_t, ::core::mem::size_of::<ev_io>() as size_t) as *mut ev_io;
    xcb_prepare =
        scalloc(1 as size_t, ::core::mem::size_of::<ev_prepare>() as size_t) as *mut ev_prepare;
    let ref mut fresh0 = (*(xcb_watcher as *mut ::core::ffi::c_void as *mut ev_watcher)).pending;
    *fresh0 = 0 as ::core::ffi::c_int;
    (*(xcb_watcher as *mut ::core::ffi::c_void as *mut ev_watcher)).active = *fresh0;
    (*(xcb_watcher as *mut ::core::ffi::c_void as *mut ev_watcher)).priority =
        0 as ::core::ffi::c_int;
    (*xcb_watcher).cb = Some(
        restore_xcb_got_event
            as unsafe extern "C" fn(*mut ev_loop, *mut ev_io, ::core::ffi::c_int) -> (),
    )
        as Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_io, ::core::ffi::c_int) -> ()>;
    memmove(
        &raw mut (*(xcb_watcher as *mut ev_watcher)).cb as *mut ::core::ffi::c_void,
        &raw mut (*xcb_watcher).cb as *const ::core::ffi::c_void,
        ::core::mem::size_of::<
            Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_io, ::core::ffi::c_int) -> ()>,
        >() as size_t,
    );
    (*xcb_watcher).fd = xcb_get_file_descriptor(restore_conn);
    (*xcb_watcher).events = EV_READ as ::core::ffi::c_int | EV__IOFDSET as ::core::ffi::c_int;
    ev_io_start(main_loop, xcb_watcher);
    let ref mut fresh1 = (*(xcb_prepare as *mut ::core::ffi::c_void as *mut ev_watcher)).pending;
    *fresh1 = 0 as ::core::ffi::c_int;
    (*(xcb_prepare as *mut ::core::ffi::c_void as *mut ev_watcher)).active = *fresh1;
    (*(xcb_prepare as *mut ::core::ffi::c_void as *mut ev_watcher)).priority =
        0 as ::core::ffi::c_int;
    (*xcb_prepare).cb = Some(
        restore_xcb_prepare_cb
            as unsafe extern "C" fn(*mut ev_loop, *mut ev_prepare, ::core::ffi::c_int) -> (),
    )
        as Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_prepare, ::core::ffi::c_int) -> ()>;
    memmove(
        &raw mut (*(xcb_prepare as *mut ev_watcher)).cb as *mut ::core::ffi::c_void,
        &raw mut (*xcb_prepare).cb as *const ::core::ffi::c_void,
        ::core::mem::size_of::<
            Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_prepare, ::core::ffi::c_int) -> ()>,
        >() as size_t,
    );
    ev_prepare_start(main_loop, xcb_prepare);
}
unsafe extern "C" fn update_placeholder_contents(mut state: *mut placeholder_state) {
    let foreground: color_t = config.client.placeholder.text;
    let background: color_t = config.client.placeholder.background;
    draw_util_clear_surface(&raw mut (*state).surface, background);
    xcb_aux_sync(restore_conn);
    let mut swallows: *mut Match = ::core::ptr::null_mut::<Match>();
    let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    swallows = (*(*state).con).swallow_head.tqh_first as *mut Match;
    while !swallows.is_null() {
        let mut serialized: *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<::core::ffi::c_char>();
        if !(*swallows).class.is_null() {
            let mut old_serialized: *mut ::core::ffi::c_char = serialized;
            sasprintf(
                &raw mut serialized,
                b"%s%sclass=\"%s\"\0" as *const u8 as *const ::core::ffi::c_char,
                if !serialized.is_null() {
                    serialized as *const ::core::ffi::c_char
                } else {
                    b"[\0" as *const u8 as *const ::core::ffi::c_char
                },
                if !serialized.is_null() {
                    b" \0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"\0" as *const u8 as *const ::core::ffi::c_char
                },
                (*(*swallows).class).pattern,
            );
            free(old_serialized as *mut ::core::ffi::c_void);
        }
        if !(*swallows).instance.is_null() {
            let mut old_serialized_0: *mut ::core::ffi::c_char = serialized;
            sasprintf(
                &raw mut serialized,
                b"%s%sinstance=\"%s\"\0" as *const u8 as *const ::core::ffi::c_char,
                if !serialized.is_null() {
                    serialized as *const ::core::ffi::c_char
                } else {
                    b"[\0" as *const u8 as *const ::core::ffi::c_char
                },
                if !serialized.is_null() {
                    b" \0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"\0" as *const u8 as *const ::core::ffi::c_char
                },
                (*(*swallows).instance).pattern,
            );
            free(old_serialized_0 as *mut ::core::ffi::c_void);
        }
        if !(*swallows).window_role.is_null() {
            let mut old_serialized_1: *mut ::core::ffi::c_char = serialized;
            sasprintf(
                &raw mut serialized,
                b"%s%swindow_role=\"%s\"\0" as *const u8 as *const ::core::ffi::c_char,
                if !serialized.is_null() {
                    serialized as *const ::core::ffi::c_char
                } else {
                    b"[\0" as *const u8 as *const ::core::ffi::c_char
                },
                if !serialized.is_null() {
                    b" \0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"\0" as *const u8 as *const ::core::ffi::c_char
                },
                (*(*swallows).window_role).pattern,
            );
            free(old_serialized_1 as *mut ::core::ffi::c_void);
        }
        if !(*swallows).title.is_null() {
            let mut old_serialized_2: *mut ::core::ffi::c_char = serialized;
            sasprintf(
                &raw mut serialized,
                b"%s%stitle=\"%s\"\0" as *const u8 as *const ::core::ffi::c_char,
                if !serialized.is_null() {
                    serialized as *const ::core::ffi::c_char
                } else {
                    b"[\0" as *const u8 as *const ::core::ffi::c_char
                },
                if !serialized.is_null() {
                    b" \0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"\0" as *const u8 as *const ::core::ffi::c_char
                },
                (*(*swallows).title).pattern,
            );
            free(old_serialized_2 as *mut ::core::ffi::c_void);
        }
        if !(*swallows).machine.is_null() {
            let mut old_serialized_3: *mut ::core::ffi::c_char = serialized;
            sasprintf(
                &raw mut serialized,
                b"%s%smachine=\"%s\"\0" as *const u8 as *const ::core::ffi::c_char,
                if !serialized.is_null() {
                    serialized as *const ::core::ffi::c_char
                } else {
                    b"[\0" as *const u8 as *const ::core::ffi::c_char
                },
                if !serialized.is_null() {
                    b" \0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"\0" as *const u8 as *const ::core::ffi::c_char
                },
                (*(*swallows).machine).pattern,
            );
            free(old_serialized_3 as *mut ::core::ffi::c_void);
        }
        if serialized.is_null() {
            debuglog(
                b"%s:%s:%d - This swallows specification is not serializable?!\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"../src/restore_layout.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"update_placeholder_contents\0" as *const u8 as *const ::core::ffi::c_char,
                159 as ::core::ffi::c_int,
            );
        } else {
            let mut old_serialized_4: *mut ::core::ffi::c_char = serialized;
            sasprintf(
                &raw mut serialized,
                b"%s]\0" as *const u8 as *const ::core::ffi::c_char,
                serialized,
            );
            free(old_serialized_4 as *mut ::core::ffi::c_void);
            debuglog(
                b"%s:%s:%d - con %p (placeholder 0x%08x) line %d: %s\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"../src/restore_layout.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"update_placeholder_contents\0" as *const u8 as *const ::core::ffi::c_char,
                166 as ::core::ffi::c_int,
                (*state).con,
                (*state).window,
                n,
                serialized,
            );
            let mut str: *mut i3String = i3string_from_utf8(serialized);
            draw_util_text(
                str,
                &raw mut (*state).surface,
                foreground,
                background,
                logical_px(2 as ::core::ffi::c_int),
                n * (config.font.height + logical_px(2 as ::core::ffi::c_int))
                    + logical_px(2 as ::core::ffi::c_int),
                (*state).rect.width.wrapping_sub(
                    (2 as ::core::ffi::c_int * logical_px(2 as ::core::ffi::c_int)) as uint32_t,
                ) as ::core::ffi::c_int,
            );
            i3string_free(str);
            n += 1;
            free(serialized as *mut ::core::ffi::c_void);
        }
        swallows = (*swallows).matches.tqe_next as *mut Match;
    }
    let mut line: *mut i3String =
        i3string_from_utf8(b"\xE2\x8C\x9A\0" as *const u8 as *const ::core::ffi::c_char);
    let mut text_width: ::core::ffi::c_int = predict_text_width(line);
    let mut x: ::core::ffi::c_int = (*state)
        .rect
        .width
        .wrapping_div(2 as uint32_t)
        .wrapping_sub((text_width / 2 as ::core::ffi::c_int) as uint32_t)
        as ::core::ffi::c_int;
    let mut y: ::core::ffi::c_int = (*state)
        .rect
        .height
        .wrapping_div(2 as uint32_t)
        .wrapping_sub((config.font.height / 2 as ::core::ffi::c_int) as uint32_t)
        as ::core::ffi::c_int;
    draw_util_text(
        line,
        &raw mut (*state).surface,
        foreground,
        background,
        x,
        y,
        text_width,
    );
    i3string_free(line);
    xcb_aux_sync(restore_conn);
}
unsafe extern "C" fn open_placeholder_window(mut con: *mut Con) {
    if con_is_leaf(con) as ::core::ffi::c_int != 0
        && ((*con).window.is_null() || (*(*con).window).id as ::core::ffi::c_long == XCB_NONE)
        && !(*con).swallow_head.tqh_first.is_null()
        && (*con).type_0 as ::core::ffi::c_uint
            == CT_CON as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut fresh2: [uint32_t; 2] = [
            config.client.placeholder.background.colorpixel,
            (XCB_EVENT_MASK_EXPOSURE as ::core::ffi::c_int
                | XCB_EVENT_MASK_STRUCTURE_NOTIFY as ::core::ffi::c_int) as uint32_t,
        ];
        let mut placeholder: xcb_window_t = create_window(
            restore_conn,
            (*con).rect as Rect,
            XCB_COPY_FROM_PARENT as uint16_t,
            XCB_COPY_FROM_PARENT as xcb_visualid_t,
            XCB_WINDOW_CLASS_INPUT_OUTPUT as ::core::ffi::c_int as uint16_t,
            XCURSOR_CURSOR_POINTER,
            true_0 != 0,
            (XCB_CW_BACK_PIXEL as ::core::ffi::c_int | XCB_CW_EVENT_MASK as ::core::ffi::c_int)
                as uint32_t,
            &raw mut fresh2 as *mut uint32_t,
        );
        let mut hints: xcb_icccm_wm_hints_t = xcb_icccm_wm_hints_t {
            flags: 0,
            input: 0,
            initial_state: 0,
            icon_pixmap: 0,
            icon_window: 0,
            icon_x: 0,
            icon_y: 0,
            icon_mask: 0,
            window_group: 0,
        };
        xcb_icccm_wm_hints_set_none(&raw mut hints);
        xcb_icccm_wm_hints_set_input(&raw mut hints, 0 as uint8_t);
        xcb_icccm_set_wm_hints(restore_conn, placeholder, &raw mut hints);
        if !(*con).name.is_null() {
            xcb_change_property(
                restore_conn,
                XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as uint8_t,
                placeholder,
                A__NET_WM_NAME,
                A_UTF8_STRING,
                8 as uint8_t,
                strlen((*con).name) as uint32_t,
                (*con).name as *const ::core::ffi::c_void,
            );
        }
        debuglog(
            b"%s:%s:%d - Created placeholder window 0x%08x for leaf container %p / %s\n\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/restore_layout.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"open_placeholder_window\0" as *const u8 as *const ::core::ffi::c_char,
            219 as ::core::ffi::c_int,
            placeholder,
            con,
            (*con).name,
        );
        let mut state: *mut placeholder_state = scalloc(
            1 as size_t,
            ::core::mem::size_of::<placeholder_state>() as size_t,
        ) as *mut placeholder_state;
        (*state).window = placeholder;
        (*state).con = con;
        (*state).rect = (*con).rect as Rect;
        draw_util_surface_init(
            restore_conn,
            &raw mut (*state).surface,
            placeholder as xcb_drawable_t,
            get_visualtype(root_screen),
            (*state).rect.width as ::core::ffi::c_int,
            (*state).rect.height as ::core::ffi::c_int,
        );
        update_placeholder_contents(state);
        (*state).state.tqe_next = ::core::ptr::null_mut::<placeholder_state>();
        (*state).state.tqe_prev = state_head.tqh_last;
        *state_head.tqh_last = state as *mut placeholder_state;
        state_head.tqh_last = &raw mut (*state).state.tqe_next;
        let mut temp_id: *mut Match =
            smalloc(::core::mem::size_of::<Match>() as size_t) as *mut Match;
        match_init(temp_id);
        (*temp_id).dock = M_DONTCHECK;
        (*temp_id).id = placeholder;
        (*temp_id).matches.tqe_next = (*con).swallow_head.tqh_first;
        if !(*temp_id).matches.tqe_next.is_null() {
            (*(*con).swallow_head.tqh_first).matches.tqe_prev =
                &raw mut (*temp_id).matches.tqe_next;
        } else {
            (*con).swallow_head.tqh_last = &raw mut (*temp_id).matches.tqe_next;
        }
        (*con).swallow_head.tqh_first = temp_id as *mut Match;
        (*temp_id).matches.tqe_prev = &raw mut (*con).swallow_head.tqh_first;
    }
    let mut child: *mut Con = ::core::ptr::null_mut::<Con>();
    child = (*con).nodes_head.tqh_first as *mut Con;
    while !child.is_null() {
        open_placeholder_window(child);
        child = (*child).nodes.tqe_next as *mut Con;
    }
    child = (*con).floating_head.tqh_first as *mut Con;
    while !child.is_null() {
        open_placeholder_window(child);
        child = (*child).floating_windows.tqe_next as *mut Con;
    }
}
#[no_mangle]
pub unsafe extern "C" fn restore_open_placeholder_windows(mut parent: *mut Con) {
    let mut child: *mut Con = ::core::ptr::null_mut::<Con>();
    child = (*parent).nodes_head.tqh_first as *mut Con;
    while !child.is_null() {
        open_placeholder_window(child);
        child = (*child).nodes.tqe_next as *mut Con;
    }
    child = (*parent).floating_head.tqh_first as *mut Con;
    while !child.is_null() {
        open_placeholder_window(child);
        child = (*child).floating_windows.tqe_next as *mut Con;
    }
    xcb_flush(restore_conn);
}
#[no_mangle]
pub unsafe extern "C" fn restore_kill_placeholder(mut placeholder: xcb_window_t) -> bool {
    let mut state: *mut placeholder_state = ::core::ptr::null_mut::<placeholder_state>();
    state = state_head.tqh_first as *mut placeholder_state;
    while !state.is_null() {
        if (*state).window != placeholder {
            state = (*state).state.tqe_next as *mut placeholder_state;
        } else {
            xcb_destroy_window(restore_conn, (*state).window);
            draw_util_surface_free(restore_conn, &raw mut (*state).surface);
            if !(*state).state.tqe_next.is_null() {
                (*(*state).state.tqe_next).state.tqe_prev = (*state).state.tqe_prev;
            } else {
                state_head.tqh_last = (*state).state.tqe_prev;
            }
            *(*state).state.tqe_prev = (*state).state.tqe_next;
            free(state as *mut ::core::ffi::c_void);
            debuglog(
                b"%s:%s:%d - placeholder window 0x%08x destroyed.\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"../src/restore_layout.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"restore_kill_placeholder\0" as *const u8 as *const ::core::ffi::c_char,
                284 as ::core::ffi::c_int,
                placeholder,
            );
            return true_0 != 0;
        }
    }
    debuglog(
        b"%s:%s:%d - 0x%08x is not a placeholder window, ignoring.\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/restore_layout.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"restore_kill_placeholder\0" as *const u8 as *const ::core::ffi::c_char,
        288 as ::core::ffi::c_int,
        placeholder,
    );
    return false_0 != 0;
}
unsafe extern "C" fn expose_event(mut event: *mut xcb_expose_event_t) {
    let mut state: *mut placeholder_state = ::core::ptr::null_mut::<placeholder_state>();
    state = state_head.tqh_first as *mut placeholder_state;
    while !state.is_null() {
        if (*state).window != (*event).window {
            state = (*state).state.tqe_next as *mut placeholder_state;
        } else {
            debuglog(
                b"%s:%s:%d - refreshing window 0x%08x contents (con %p)\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"../src/restore_layout.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"expose_event\0" as *const u8 as *const ::core::ffi::c_char,
                299 as ::core::ffi::c_int,
                (*state).window,
                (*state).con,
            );
            update_placeholder_contents(state);
            return;
        }
    }
    errorlog(
        b"ERROR: Received ExposeEvent for unknown window 0x%08x\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        (*event).window,
    );
}
unsafe extern "C" fn configure_notify(mut event: *mut xcb_configure_notify_event_t) {
    let mut state: *mut placeholder_state = ::core::ptr::null_mut::<placeholder_state>();
    state = state_head.tqh_first as *mut placeholder_state;
    while !state.is_null() {
        if (*state).window != (*event).window {
            state = (*state).state.tqe_next as *mut placeholder_state;
        } else {
            debuglog(
                b"%s:%s:%d - ConfigureNotify: window 0x%08x has now width=%d, height=%d (con %p)\n\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"../src/restore_layout.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"configure_notify\0" as *const u8 as *const ::core::ffi::c_char,
                323 as ::core::ffi::c_int,
                (*state).window,
                (*event).width as ::core::ffi::c_int,
                (*event).height as ::core::ffi::c_int,
                (*state).con,
            );
            (*state).rect.width = (*event).width as uint32_t;
            (*state).rect.height = (*event).height as uint32_t;
            draw_util_surface_set_size(
                &raw mut (*state).surface,
                (*state).rect.width as ::core::ffi::c_int,
                (*state).rect.height as ::core::ffi::c_int,
            );
            update_placeholder_contents(state);
            return;
        }
    }
    errorlog(
        b"ERROR: Received ConfigureNotify for unknown window 0x%08x\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        (*event).window,
    );
}
unsafe extern "C" fn restore_handle_event(
    mut type_0: ::core::ffi::c_int,
    mut event: *mut xcb_generic_event_t,
) {
    match type_0 {
        XCB_EXPOSE => {
            if (*(event as *mut xcb_expose_event_t)).count as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                expose_event(event as *mut xcb_expose_event_t);
            }
        }
        XCB_CONFIGURE_NOTIFY => {
            configure_notify(event as *mut xcb_configure_notify_event_t);
        }
        _ => {
            debuglog(
                b"%s:%s:%d - Received unhandled X11 event of type %d\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"../src/restore_layout.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"restore_handle_event\0" as *const u8 as *const ::core::ffi::c_char,
                350 as ::core::ffi::c_int,
                type_0,
            );
        }
    };
}
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
unsafe extern "C" fn run_static_initializers() {
    state_head = state_head {
        tqh_first: ::core::ptr::null_mut::<placeholder_state>(),
        tqh_last: &raw mut state_head.tqh_first,
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
