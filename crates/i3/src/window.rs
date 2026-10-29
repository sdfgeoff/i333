/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * window.c: Updates window attributes (X11 hints/properties).
 *
 */
// Mechanical file-by-file Rust translation of src/window.c.
// Generated with C2Rust 0.22.1; C ABI and original ownership retained.
extern "C" {
    pub type xcb_connection_t;
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
    fn xcb_get_property_value(R: *const xcb_get_property_reply_t) -> *mut ::core::ffi::c_void;
    fn xcb_get_property_value_length(R: *const xcb_get_property_reply_t) -> ::core::ffi::c_int;
    fn xcb_icccm_get_wm_transient_for_from_reply(
        prop: *mut xcb_window_t,
        reply: *mut xcb_get_property_reply_t,
    ) -> uint8_t;
    fn xcb_icccm_get_wm_normal_hints_unchecked(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_get_property_cookie_t;
    fn xcb_icccm_get_wm_size_hints_from_reply(
        hints: *mut xcb_size_hints_t,
        reply: *mut xcb_get_property_reply_t,
    ) -> uint8_t;
    fn xcb_icccm_get_wm_normal_hints_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_get_property_cookie_t,
        hints: *mut xcb_size_hints_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> uint8_t;
    fn xcb_icccm_wm_hints_get_urgency(hints: *mut xcb_icccm_wm_hints_t) -> uint32_t;
    fn xcb_icccm_get_wm_hints_from_reply(
        hints: *mut xcb_icccm_wm_hints_t,
        reply: *mut xcb_get_property_reply_t,
    ) -> uint8_t;
    fn cairo_surface_destroy(surface: *mut cairo_surface_t);
    fn cairo_surface_set_user_data(
        surface: *mut cairo_surface_t,
        key: *const cairo_user_data_key_t,
        user_data: *mut ::core::ffi::c_void,
        destroy: cairo_destroy_func_t,
    ) -> cairo_status_t;
    fn cairo_image_surface_create_for_data(
        data: *mut ::core::ffi::c_uchar,
        format: cairo_format_t,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
        stride: ::core::ffi::c_int,
    ) -> *mut cairo_surface_t;
    static mut conn: *mut xcb_connection_t;
    fn verboselog(fmt: *mut ::core::ffi::c_char, ...);
    fn debuglog(fmt: *mut ::core::ffi::c_char, ...);
    fn smalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn sstrndup(str: *const ::core::ffi::c_char, size: size_t) -> *mut ::core::ffi::c_char;
    fn sasprintf(
        strp: *mut *mut ::core::ffi::c_char,
        fmt: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn i3string_from_utf8(from_utf8: *const ::core::ffi::c_char) -> *mut i3String;
    fn i3string_free(str: *mut i3String);
    fn i3string_as_utf8(str: *mut i3String) -> *const ::core::ffi::c_char;
    fn logical_px(logical: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn max(a: ::core::ffi::c_int, b: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn xcb_get_preferred_window_type(reply: *mut xcb_get_property_reply_t) -> xcb_atom_t;
    fn con_by_window_id(window: xcb_window_t) -> *mut Con;
    fn con_parse_title_format(con: *mut Con) -> *mut i3String;
    fn render_deco_height() -> ::core::ffi::c_int;
    fn ewmh_update_visible_name(window: xcb_window_t, name: *const ::core::ffi::c_char);
    fn run_assignments(window: *mut i3Window);
    fn fabs(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn strnlen(__string: *const ::core::ffi::c_char, __maxlen: size_t) -> size_t;
    fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
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
pub type uint64_t = __uint64_t;
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
pub type xcb_window_t = uint32_t;
pub type xcb_pixmap_t = uint32_t;
pub type xcb_gcontext_t = uint32_t;
pub type xcb_colormap_t = uint32_t;
pub type xcb_atom_t = uint32_t;
pub type xcb_drawable_t = uint32_t;
pub type xcb_atom_enum_t = ::core::ffi::c_uint;
pub const XCB_ATOM_WM_TRANSIENT_FOR: xcb_atom_enum_t = 68;
pub const XCB_ATOM_WM_CLASS: xcb_atom_enum_t = 67;
pub const XCB_ATOM_CAP_HEIGHT: xcb_atom_enum_t = 66;
pub const XCB_ATOM_FULL_NAME: xcb_atom_enum_t = 65;
pub const XCB_ATOM_FAMILY_NAME: xcb_atom_enum_t = 64;
pub const XCB_ATOM_FONT_NAME: xcb_atom_enum_t = 63;
pub const XCB_ATOM_NOTICE: xcb_atom_enum_t = 62;
pub const XCB_ATOM_COPYRIGHT: xcb_atom_enum_t = 61;
pub const XCB_ATOM_RESOLUTION: xcb_atom_enum_t = 60;
pub const XCB_ATOM_POINT_SIZE: xcb_atom_enum_t = 59;
pub const XCB_ATOM_WEIGHT: xcb_atom_enum_t = 58;
pub const XCB_ATOM_QUAD_WIDTH: xcb_atom_enum_t = 57;
pub const XCB_ATOM_X_HEIGHT: xcb_atom_enum_t = 56;
pub const XCB_ATOM_ITALIC_ANGLE: xcb_atom_enum_t = 55;
pub const XCB_ATOM_STRIKEOUT_DESCENT: xcb_atom_enum_t = 54;
pub const XCB_ATOM_STRIKEOUT_ASCENT: xcb_atom_enum_t = 53;
pub const XCB_ATOM_UNDERLINE_THICKNESS: xcb_atom_enum_t = 52;
pub const XCB_ATOM_UNDERLINE_POSITION: xcb_atom_enum_t = 51;
pub const XCB_ATOM_SUBSCRIPT_Y: xcb_atom_enum_t = 50;
pub const XCB_ATOM_SUBSCRIPT_X: xcb_atom_enum_t = 49;
pub const XCB_ATOM_SUPERSCRIPT_Y: xcb_atom_enum_t = 48;
pub const XCB_ATOM_SUPERSCRIPT_X: xcb_atom_enum_t = 47;
pub const XCB_ATOM_END_SPACE: xcb_atom_enum_t = 46;
pub const XCB_ATOM_MAX_SPACE: xcb_atom_enum_t = 45;
pub const XCB_ATOM_NORM_SPACE: xcb_atom_enum_t = 44;
pub const XCB_ATOM_MIN_SPACE: xcb_atom_enum_t = 43;
pub const XCB_ATOM_WM_ZOOM_HINTS: xcb_atom_enum_t = 42;
pub const XCB_ATOM_WM_SIZE_HINTS: xcb_atom_enum_t = 41;
pub const XCB_ATOM_WM_NORMAL_HINTS: xcb_atom_enum_t = 40;
pub const XCB_ATOM_WM_NAME: xcb_atom_enum_t = 39;
pub const XCB_ATOM_WM_ICON_SIZE: xcb_atom_enum_t = 38;
pub const XCB_ATOM_WM_ICON_NAME: xcb_atom_enum_t = 37;
pub const XCB_ATOM_WM_CLIENT_MACHINE: xcb_atom_enum_t = 36;
pub const XCB_ATOM_WM_HINTS: xcb_atom_enum_t = 35;
pub const XCB_ATOM_WM_COMMAND: xcb_atom_enum_t = 34;
pub const XCB_ATOM_WINDOW: xcb_atom_enum_t = 33;
pub const XCB_ATOM_VISUALID: xcb_atom_enum_t = 32;
pub const XCB_ATOM_STRING: xcb_atom_enum_t = 31;
pub const XCB_ATOM_RGB_RED_MAP: xcb_atom_enum_t = 30;
pub const XCB_ATOM_RGB_GREEN_MAP: xcb_atom_enum_t = 29;
pub const XCB_ATOM_RGB_GRAY_MAP: xcb_atom_enum_t = 28;
pub const XCB_ATOM_RGB_DEFAULT_MAP: xcb_atom_enum_t = 27;
pub const XCB_ATOM_RGB_BLUE_MAP: xcb_atom_enum_t = 26;
pub const XCB_ATOM_RGB_BEST_MAP: xcb_atom_enum_t = 25;
pub const XCB_ATOM_RGB_COLOR_MAP: xcb_atom_enum_t = 24;
pub const XCB_ATOM_RESOURCE_MANAGER: xcb_atom_enum_t = 23;
pub const XCB_ATOM_RECTANGLE: xcb_atom_enum_t = 22;
pub const XCB_ATOM_POINT: xcb_atom_enum_t = 21;
pub const XCB_ATOM_PIXMAP: xcb_atom_enum_t = 20;
pub const XCB_ATOM_INTEGER: xcb_atom_enum_t = 19;
pub const XCB_ATOM_FONT: xcb_atom_enum_t = 18;
pub const XCB_ATOM_DRAWABLE: xcb_atom_enum_t = 17;
pub const XCB_ATOM_CUT_BUFFER7: xcb_atom_enum_t = 16;
pub const XCB_ATOM_CUT_BUFFER6: xcb_atom_enum_t = 15;
pub const XCB_ATOM_CUT_BUFFER5: xcb_atom_enum_t = 14;
pub const XCB_ATOM_CUT_BUFFER4: xcb_atom_enum_t = 13;
pub const XCB_ATOM_CUT_BUFFER3: xcb_atom_enum_t = 12;
pub const XCB_ATOM_CUT_BUFFER2: xcb_atom_enum_t = 11;
pub const XCB_ATOM_CUT_BUFFER1: xcb_atom_enum_t = 10;
pub const XCB_ATOM_CUT_BUFFER0: xcb_atom_enum_t = 9;
pub const XCB_ATOM_CURSOR: xcb_atom_enum_t = 8;
pub const XCB_ATOM_COLORMAP: xcb_atom_enum_t = 7;
pub const XCB_ATOM_CARDINAL: xcb_atom_enum_t = 6;
pub const XCB_ATOM_BITMAP: xcb_atom_enum_t = 5;
pub const XCB_ATOM_ATOM: xcb_atom_enum_t = 4;
pub const XCB_ATOM_ARC: xcb_atom_enum_t = 3;
pub const XCB_ATOM_SECONDARY: xcb_atom_enum_t = 2;
pub const XCB_ATOM_PRIMARY: xcb_atom_enum_t = 1;
pub const XCB_ATOM_ANY: xcb_atom_enum_t = 0;
pub const XCB_ATOM_NONE: xcb_atom_enum_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_get_geometry_reply_t {
    pub response_type: uint8_t,
    pub depth: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub root: xcb_window_t,
    pub x: int16_t,
    pub y: int16_t,
    pub width: uint16_t,
    pub height: uint16_t,
    pub border_width: uint16_t,
    pub pad0: [uint8_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_get_property_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_get_property_reply_t {
    pub response_type: uint8_t,
    pub format: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub type_0: xcb_atom_t,
    pub bytes_after: uint32_t,
    pub value_len: uint32_t,
    pub pad0: [uint8_t; 12],
}
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const XCB_ICCCM_SIZE_HINT_P_WIN_GRAVITY: C2RustUnnamed = 512;
pub const XCB_ICCCM_SIZE_HINT_BASE_SIZE: C2RustUnnamed = 256;
pub const XCB_ICCCM_SIZE_HINT_P_ASPECT: C2RustUnnamed = 128;
pub const XCB_ICCCM_SIZE_HINT_P_RESIZE_INC: C2RustUnnamed = 64;
pub const XCB_ICCCM_SIZE_HINT_P_MAX_SIZE: C2RustUnnamed = 32;
pub const XCB_ICCCM_SIZE_HINT_P_MIN_SIZE: C2RustUnnamed = 16;
pub const XCB_ICCCM_SIZE_HINT_P_SIZE: C2RustUnnamed = 8;
pub const XCB_ICCCM_SIZE_HINT_P_POSITION: C2RustUnnamed = 4;
pub const XCB_ICCCM_SIZE_HINT_US_SIZE: C2RustUnnamed = 2;
pub const XCB_ICCCM_SIZE_HINT_US_POSITION: C2RustUnnamed = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_size_hints_t {
    pub flags: uint32_t,
    pub x: int32_t,
    pub y: int32_t,
    pub width: int32_t,
    pub height: int32_t,
    pub min_width: int32_t,
    pub min_height: int32_t,
    pub max_width: int32_t,
    pub max_height: int32_t,
    pub width_inc: int32_t,
    pub height_inc: int32_t,
    pub min_aspect_num: int32_t,
    pub min_aspect_den: int32_t,
    pub max_aspect_num: int32_t,
    pub max_aspect_den: int32_t,
    pub base_width: int32_t,
    pub base_height: int32_t,
    pub win_gravity: uint32_t,
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
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const XCB_ICCCM_WM_HINT_X_URGENCY: C2RustUnnamed_0 = 256;
pub const XCB_ICCCM_WM_HINT_WINDOW_GROUP: C2RustUnnamed_0 = 64;
pub const XCB_ICCCM_WM_HINT_ICON_MASK: C2RustUnnamed_0 = 32;
pub const XCB_ICCCM_WM_HINT_ICON_POSITION: C2RustUnnamed_0 = 16;
pub const XCB_ICCCM_WM_HINT_ICON_WINDOW: C2RustUnnamed_0 = 8;
pub const XCB_ICCCM_WM_HINT_ICON_PIXMAP: C2RustUnnamed_0 = 4;
pub const XCB_ICCCM_WM_HINT_STATE: C2RustUnnamed_0 = 2;
pub const XCB_ICCCM_WM_HINT_INPUT: C2RustUnnamed_0 = 1;
pub type cairo_t = _cairo;
pub type cairo_surface_t = _cairo_surface;
pub type cairo_destroy_func_t = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_user_data_key {
    pub unused: ::core::ffi::c_int,
}
pub type cairo_user_data_key_t = _cairo_user_data_key;
pub type _cairo_status = ::core::ffi::c_uint;
pub const CAIRO_STATUS_LAST_STATUS: _cairo_status = 45;
pub const CAIRO_STATUS_SVG_FONT_ERROR: _cairo_status = 44;
pub const CAIRO_STATUS_DWRITE_ERROR: _cairo_status = 43;
pub const CAIRO_STATUS_TAG_ERROR: _cairo_status = 42;
pub const CAIRO_STATUS_WIN32_GDI_ERROR: _cairo_status = 41;
pub const CAIRO_STATUS_FREETYPE_ERROR: _cairo_status = 40;
pub const CAIRO_STATUS_PNG_ERROR: _cairo_status = 39;
pub const CAIRO_STATUS_JBIG2_GLOBAL_MISSING: _cairo_status = 38;
pub const CAIRO_STATUS_DEVICE_FINISHED: _cairo_status = 37;
pub const CAIRO_STATUS_INVALID_MESH_CONSTRUCTION: _cairo_status = 36;
pub const CAIRO_STATUS_DEVICE_ERROR: _cairo_status = 35;
pub const CAIRO_STATUS_DEVICE_TYPE_MISMATCH: _cairo_status = 34;
pub const CAIRO_STATUS_USER_FONT_NOT_IMPLEMENTED: _cairo_status = 33;
pub const CAIRO_STATUS_INVALID_SIZE: _cairo_status = 32;
pub const CAIRO_STATUS_INVALID_WEIGHT: _cairo_status = 31;
pub const CAIRO_STATUS_INVALID_SLANT: _cairo_status = 30;
pub const CAIRO_STATUS_INVALID_CLUSTERS: _cairo_status = 29;
pub const CAIRO_STATUS_NEGATIVE_COUNT: _cairo_status = 28;
pub const CAIRO_STATUS_USER_FONT_ERROR: _cairo_status = 27;
pub const CAIRO_STATUS_USER_FONT_IMMUTABLE: _cairo_status = 26;
pub const CAIRO_STATUS_FONT_TYPE_MISMATCH: _cairo_status = 25;
pub const CAIRO_STATUS_INVALID_STRIDE: _cairo_status = 24;
pub const CAIRO_STATUS_TEMP_FILE_ERROR: _cairo_status = 23;
pub const CAIRO_STATUS_CLIP_NOT_REPRESENTABLE: _cairo_status = 22;
pub const CAIRO_STATUS_INVALID_INDEX: _cairo_status = 21;
pub const CAIRO_STATUS_INVALID_DSC_COMMENT: _cairo_status = 20;
pub const CAIRO_STATUS_INVALID_DASH: _cairo_status = 19;
pub const CAIRO_STATUS_FILE_NOT_FOUND: _cairo_status = 18;
pub const CAIRO_STATUS_INVALID_VISUAL: _cairo_status = 17;
pub const CAIRO_STATUS_INVALID_FORMAT: _cairo_status = 16;
pub const CAIRO_STATUS_INVALID_CONTENT: _cairo_status = 15;
pub const CAIRO_STATUS_PATTERN_TYPE_MISMATCH: _cairo_status = 14;
pub const CAIRO_STATUS_SURFACE_TYPE_MISMATCH: _cairo_status = 13;
pub const CAIRO_STATUS_SURFACE_FINISHED: _cairo_status = 12;
pub const CAIRO_STATUS_WRITE_ERROR: _cairo_status = 11;
pub const CAIRO_STATUS_READ_ERROR: _cairo_status = 10;
pub const CAIRO_STATUS_INVALID_PATH_DATA: _cairo_status = 9;
pub const CAIRO_STATUS_INVALID_STRING: _cairo_status = 8;
pub const CAIRO_STATUS_NULL_POINTER: _cairo_status = 7;
pub const CAIRO_STATUS_INVALID_STATUS: _cairo_status = 6;
pub const CAIRO_STATUS_INVALID_MATRIX: _cairo_status = 5;
pub const CAIRO_STATUS_NO_CURRENT_POINT: _cairo_status = 4;
pub const CAIRO_STATUS_INVALID_POP_GROUP: _cairo_status = 3;
pub const CAIRO_STATUS_INVALID_RESTORE: _cairo_status = 2;
pub const CAIRO_STATUS_NO_MEMORY: _cairo_status = 1;
pub const CAIRO_STATUS_SUCCESS: _cairo_status = 0;
pub type cairo_status_t = _cairo_status;
pub type _cairo_format = ::core::ffi::c_int;
pub const CAIRO_FORMAT_RGBA128F: _cairo_format = 7;
pub const CAIRO_FORMAT_RGB96F: _cairo_format = 6;
pub const CAIRO_FORMAT_RGB30: _cairo_format = 5;
pub const CAIRO_FORMAT_RGB16_565: _cairo_format = 4;
pub const CAIRO_FORMAT_A1: _cairo_format = 3;
pub const CAIRO_FORMAT_A8: _cairo_format = 2;
pub const CAIRO_FORMAT_RGB24: _cairo_format = 1;
pub const CAIRO_FORMAT_ARGB32: _cairo_format = 0;
pub const CAIRO_FORMAT_INVALID: _cairo_format = -1;
pub type cairo_format_t = _cairo_format;
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
    pub type_0: C2RustUnnamed_17,
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
    pub floating: C2RustUnnamed_6,
    pub nodes: C2RustUnnamed_5,
    pub focused: C2RustUnnamed_4,
    pub all_cons: C2RustUnnamed_3,
    pub floating_windows: C2RustUnnamed_2,
    pub on_remove_child: Option<unsafe extern "C" fn(*mut Con) -> ()>,
    pub scratchpad_state: C2RustUnnamed_1,
    pub old_id: ::core::ffi::c_int,
    pub depth: uint16_t,
    pub colormap: xcb_colormap_t,
}
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const SCRATCHPAD_CHANGED: C2RustUnnamed_1 = 2;
pub const SCRATCHPAD_FRESH: C2RustUnnamed_1 = 1;
pub const SCRATCHPAD_NONE: C2RustUnnamed_1 = 0;
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
pub type C2RustUnnamed_6 = ::core::ffi::c_uint;
pub const FLOATING_USER_ON: C2RustUnnamed_6 = 3;
pub const FLOATING_AUTO_ON: C2RustUnnamed_6 = 2;
pub const FLOATING_USER_OFF: C2RustUnnamed_6 = 1;
pub const FLOATING_AUTO_OFF: C2RustUnnamed_6 = 0;
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
    pub urgent: C2RustUnnamed_11,
    pub dock: C2RustUnnamed_10,
    pub id: xcb_window_t,
    pub window_mode: C2RustUnnamed_9,
    pub con_id: *mut Con,
    pub match_all_windows: bool,
    pub insert_where: C2RustUnnamed_8,
    pub matches: C2RustUnnamed_7,
    pub restart_mode: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub tqe_next: *mut Match,
    pub tqe_prev: *mut *mut Match,
}
pub type C2RustUnnamed_8 = ::core::ffi::c_uint;
pub const M_BELOW: C2RustUnnamed_8 = 2;
pub const M_ASSIGN_WS: C2RustUnnamed_8 = 1;
pub const M_HERE: C2RustUnnamed_8 = 0;
pub type C2RustUnnamed_9 = ::core::ffi::c_uint;
pub const WM_FLOATING: C2RustUnnamed_9 = 6;
pub const WM_FLOATING_USER: C2RustUnnamed_9 = 5;
pub const WM_FLOATING_AUTO: C2RustUnnamed_9 = 4;
pub const WM_TILING: C2RustUnnamed_9 = 3;
pub const WM_TILING_USER: C2RustUnnamed_9 = 2;
pub const WM_TILING_AUTO: C2RustUnnamed_9 = 1;
pub const WM_ANY: C2RustUnnamed_9 = 0;
pub type C2RustUnnamed_10 = ::core::ffi::c_int;
pub const M_DOCK_BOTTOM: C2RustUnnamed_10 = 3;
pub const M_DOCK_TOP: C2RustUnnamed_10 = 2;
pub const M_DOCK_ANY: C2RustUnnamed_10 = 1;
pub const M_NODOCK: C2RustUnnamed_10 = 0;
pub const M_DONTCHECK: C2RustUnnamed_10 = -1;
pub type C2RustUnnamed_11 = ::core::ffi::c_int;
pub const U_OLDEST: C2RustUnnamed_11 = 1;
pub const U_LATEST: C2RustUnnamed_11 = 0;
pub const U_DONTCHECK: C2RustUnnamed_11 = -1;
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
    pub dock: C2RustUnnamed_12,
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
pub type C2RustUnnamed_12 = ::core::ffi::c_uint;
pub const W_DOCK_BOTTOM: C2RustUnnamed_12 = 2;
pub const W_DOCK_TOP: C2RustUnnamed_12 = 1;
pub const W_NODOCK: C2RustUnnamed_12 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Assignment {
    pub type_0: C2RustUnnamed_15,
    pub match_0: Match,
    pub dest: C2RustUnnamed_14,
    pub assignments: C2RustUnnamed_13,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub tqe_next: *mut Assignment,
    pub tqe_prev: *mut *mut Assignment,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_14 {
    pub command: *mut ::core::ffi::c_char,
    pub workspace: *mut ::core::ffi::c_char,
    pub output: *mut ::core::ffi::c_char,
}
pub type C2RustUnnamed_15 = ::core::ffi::c_uint;
pub const A_TO_OUTPUT: C2RustUnnamed_15 = 16;
pub const A_TO_WORKSPACE_NUMBER: C2RustUnnamed_15 = 8;
pub const A_NO_FOCUS: C2RustUnnamed_15 = 4;
pub const A_TO_WORKSPACE: C2RustUnnamed_15 = 2;
pub const A_COMMAND: C2RustUnnamed_15 = 1;
pub const A_ANY: C2RustUnnamed_15 = 0;
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
    pub marks: C2RustUnnamed_16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_16 {
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
pub type C2RustUnnamed_17 = ::core::ffi::c_uint;
pub const CT_DOCKAREA: C2RustUnnamed_17 = 5;
pub const CT_WORKSPACE: C2RustUnnamed_17 = 4;
pub const CT_FLOATING_CON: C2RustUnnamed_17 = 3;
pub const CT_CON: C2RustUnnamed_17 = 2;
pub const CT_OUTPUT: C2RustUnnamed_17 = 1;
pub const CT_ROOT: C2RustUnnamed_17 = 0;
pub type i3Window = Window;
pub const __ASSERT_FUNCTION: [::core::ffi::c_char; 90] = unsafe {
    ::core::mem::transmute::<
        [u8; 90],
        [::core::ffi::c_char; 90],
    >(
        *b"_Bool window_update_motif_hints(i3Window *, xcb_get_property_reply_t *, border_style_t *)\0",
    )
};
pub const XCB_NONE: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const DBL_EPSILON: ::core::ffi::c_double = __DBL_EPSILON__;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[no_mangle]
pub unsafe extern "C" fn window_free(mut win: *mut i3Window) {
    free((*win).class_class as *mut ::core::ffi::c_void);
    (*win).class_class = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free((*win).class_instance as *mut ::core::ffi::c_void);
    (*win).class_instance = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free((*win).role as *mut ::core::ffi::c_void);
    (*win).role = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free((*win).machine as *mut ::core::ffi::c_void);
    (*win).machine = ::core::ptr::null_mut::<::core::ffi::c_char>();
    i3string_free((*win).name);
    cairo_surface_destroy((*win).icon);
    free((*win).ran_assignments as *mut ::core::ffi::c_void);
    (*win).ran_assignments = ::core::ptr::null_mut::<*mut Assignment>();
    free(win as *mut ::core::ffi::c_void);
    win = ::core::ptr::null_mut::<i3Window>();
}
#[no_mangle]
pub unsafe extern "C" fn window_update_class(
    mut win: *mut i3Window,
    mut prop: *mut xcb_get_property_reply_t,
) {
    if prop.is_null() || xcb_get_property_value_length(prop) == 0 as ::core::ffi::c_int {
        debuglog(
            b"%s:%s:%d - WM_CLASS not set.\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b"../src/window.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"window_update_class\0" as *const u8 as *const ::core::ffi::c_char,
            36 as ::core::ffi::c_int,
        );
        free(prop as *mut ::core::ffi::c_void);
        prop = ::core::ptr::null_mut::<xcb_get_property_reply_t>();
        return;
    }
    let prop_length: size_t = xcb_get_property_value_length(prop) as size_t;
    let mut new_class: *mut ::core::ffi::c_char =
        xcb_get_property_value(prop) as *mut ::core::ffi::c_char;
    let class_class_index: size_t =
        (strnlen(new_class, prop_length) as size_t).wrapping_add(1 as size_t);
    free((*win).class_instance as *mut ::core::ffi::c_void);
    (*win).class_instance = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free((*win).class_class as *mut ::core::ffi::c_void);
    (*win).class_class = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*win).class_instance = sstrndup(new_class, prop_length);
    if class_class_index < prop_length {
        (*win).class_class = sstrndup(
            new_class.offset(class_class_index as isize),
            prop_length.wrapping_sub(class_class_index),
        );
    } else {
        (*win).class_class = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    verboselog(
        b"WM_CLASS changed to %s (instance), %s (class)\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        (*win).class_instance,
        (*win).class_class,
    );
    free(prop as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn window_update_name(
    mut win: *mut i3Window,
    mut prop: *mut xcb_get_property_reply_t,
) {
    if prop.is_null() || xcb_get_property_value_length(prop) == 0 as ::core::ffi::c_int {
        debuglog(
            b"%s:%s:%d - _NET_WM_NAME not specified, not changing\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/window.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"window_update_name\0" as *const u8 as *const ::core::ffi::c_char,
            70 as ::core::ffi::c_int,
        );
        free(prop as *mut ::core::ffi::c_void);
        prop = ::core::ptr::null_mut::<xcb_get_property_reply_t>();
        return;
    }
    i3string_free((*win).name);
    let len: ::core::ffi::c_int = xcb_get_property_value_length(prop) as ::core::ffi::c_int;
    let mut name: *mut ::core::ffi::c_char = sstrndup(
        xcb_get_property_value(prop) as *const ::core::ffi::c_char,
        len as size_t,
    );
    (*win).name = i3string_from_utf8(name);
    free(name as *mut ::core::ffi::c_void);
    let mut con: *mut Con = con_by_window_id((*win).id);
    if !con.is_null() && !(*con).title_format.is_null() {
        let mut name_0: *mut i3String = con_parse_title_format(con);
        ewmh_update_visible_name((*win).id, i3string_as_utf8(name_0));
        i3string_free(name_0);
        name_0 = ::core::ptr::null_mut::<i3String>();
    }
    (*win).name_x_changed = true_0 != 0;
    verboselog(
        b"_NET_WM_NAME changed to \"%s\"\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        i3string_as_utf8((*win).name),
    );
    (*win).uses_net_wm_name = true_0 != 0;
    free(prop as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn window_update_name_legacy(
    mut win: *mut i3Window,
    mut prop: *mut xcb_get_property_reply_t,
) {
    if prop.is_null() || xcb_get_property_value_length(prop) == 0 as ::core::ffi::c_int {
        debuglog(
            b"%s:%s:%d - WM_NAME not set (_NET_WM_NAME is what you want anyways).\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/window.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"window_update_name_legacy\0" as *const u8 as *const ::core::ffi::c_char,
            106 as ::core::ffi::c_int,
        );
        free(prop as *mut ::core::ffi::c_void);
        prop = ::core::ptr::null_mut::<xcb_get_property_reply_t>();
        return;
    }
    if (*win).uses_net_wm_name {
        free(prop as *mut ::core::ffi::c_void);
        return;
    }
    i3string_free((*win).name);
    let len: ::core::ffi::c_int = xcb_get_property_value_length(prop) as ::core::ffi::c_int;
    let mut name: *mut ::core::ffi::c_char = sstrndup(
        xcb_get_property_value(prop) as *const ::core::ffi::c_char,
        len as size_t,
    );
    (*win).name = i3string_from_utf8(name);
    free(name as *mut ::core::ffi::c_void);
    let mut con: *mut Con = con_by_window_id((*win).id);
    if !con.is_null() && !(*con).title_format.is_null() {
        let mut name_0: *mut i3String = con_parse_title_format(con);
        ewmh_update_visible_name((*win).id, i3string_as_utf8(name_0));
        i3string_free(name_0);
        name_0 = ::core::ptr::null_mut::<i3String>();
    }
    verboselog(
        b"WM_NAME changed to \"%s\"\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        i3string_as_utf8((*win).name),
    );
    verboselog(
        b"Using legacy window title. Note that in order to get Unicode window titles in i3, the application has to set _NET_WM_NAME (UTF-8)\n\0"
            as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    (*win).name_x_changed = true_0 != 0;
    free(prop as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn window_update_leader(
    mut win: *mut i3Window,
    mut prop: *mut xcb_get_property_reply_t,
) {
    if prop.is_null() || xcb_get_property_value_length(prop) == 0 as ::core::ffi::c_int {
        debuglog(
            b"%s:%s:%d - CLIENT_LEADER not set on window 0x%08x.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/window.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"window_update_leader\0" as *const u8 as *const ::core::ffi::c_char,
            145 as ::core::ffi::c_int,
            (*win).id,
        );
        (*win).leader = XCB_NONE as xcb_window_t;
        free(prop as *mut ::core::ffi::c_void);
        prop = ::core::ptr::null_mut::<xcb_get_property_reply_t>();
        return;
    }
    let mut leader: *mut xcb_window_t = xcb_get_property_value(prop) as *mut xcb_window_t;
    if leader.is_null() {
        free(prop as *mut ::core::ffi::c_void);
        return;
    }
    debuglog(
        b"%s:%s:%d - Client leader changed to %08x\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/window.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"window_update_leader\0" as *const u8 as *const ::core::ffi::c_char,
        157 as ::core::ffi::c_int,
        *leader,
    );
    (*win).leader = *leader;
    free(prop as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn window_update_transient_for(
    mut win: *mut i3Window,
    mut prop: *mut xcb_get_property_reply_t,
) {
    if prop.is_null() || xcb_get_property_value_length(prop) == 0 as ::core::ffi::c_int {
        debuglog(
            b"%s:%s:%d - TRANSIENT_FOR not set on window 0x%08x.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/window.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"window_update_transient_for\0" as *const u8 as *const ::core::ffi::c_char,
            170 as ::core::ffi::c_int,
            (*win).id,
        );
        (*win).transient_for = XCB_NONE as xcb_window_t;
        free(prop as *mut ::core::ffi::c_void);
        prop = ::core::ptr::null_mut::<xcb_get_property_reply_t>();
        return;
    }
    let mut transient_for: xcb_window_t = 0;
    if xcb_icccm_get_wm_transient_for_from_reply(&raw mut transient_for, prop) == 0 {
        free(prop as *mut ::core::ffi::c_void);
        return;
    }
    debuglog(
        b"%s:%s:%d - Transient for changed to 0x%08x (window 0x%08x)\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/window.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"window_update_transient_for\0" as *const u8 as *const ::core::ffi::c_char,
        182 as ::core::ffi::c_int,
        transient_for,
        (*win).id,
    );
    (*win).transient_for = transient_for;
    free(prop as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn window_update_strut_partial(
    mut win: *mut i3Window,
    mut prop: *mut xcb_get_property_reply_t,
) {
    if prop.is_null() || xcb_get_property_value_length(prop) == 0 as ::core::ffi::c_int {
        debuglog(
            b"%s:%s:%d - _NET_WM_STRUT_PARTIAL not set.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/window.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"window_update_strut_partial\0" as *const u8 as *const ::core::ffi::c_char,
            195 as ::core::ffi::c_int,
        );
        free(prop as *mut ::core::ffi::c_void);
        prop = ::core::ptr::null_mut::<xcb_get_property_reply_t>();
        return;
    }
    let mut strut: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
    strut = xcb_get_property_value(prop) as *mut uint32_t;
    if strut.is_null() {
        free(prop as *mut ::core::ffi::c_void);
        return;
    }
    debuglog(
        b"%s:%s:%d - Reserved pixels changed to: left = %d, right = %d, top = %d, bottom = %d\n\0"
            as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/window.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"window_update_strut_partial\0" as *const u8 as *const ::core::ffi::c_char,
        207 as ::core::ffi::c_int,
        *strut.offset(0 as ::core::ffi::c_int as isize),
        *strut.offset(1 as ::core::ffi::c_int as isize),
        *strut.offset(2 as ::core::ffi::c_int as isize),
        *strut.offset(3 as ::core::ffi::c_int as isize),
    );
    (*win).reserved = reservedpx {
        left: *strut.offset(0 as ::core::ffi::c_int as isize),
        right: *strut.offset(1 as ::core::ffi::c_int as isize),
        top: *strut.offset(2 as ::core::ffi::c_int as isize),
        bottom: *strut.offset(3 as ::core::ffi::c_int as isize),
    };
    free(prop as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn window_update_role(
    mut win: *mut i3Window,
    mut prop: *mut xcb_get_property_reply_t,
) {
    if prop.is_null() || xcb_get_property_value_length(prop) == 0 as ::core::ffi::c_int {
        debuglog(
            b"%s:%s:%d - WM_WINDOW_ROLE not set.\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b"../src/window.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"window_update_role\0" as *const u8 as *const ::core::ffi::c_char,
            220 as ::core::ffi::c_int,
        );
        free(prop as *mut ::core::ffi::c_void);
        prop = ::core::ptr::null_mut::<xcb_get_property_reply_t>();
        return;
    }
    let mut new_role: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    sasprintf(
        &raw mut new_role,
        b"%.*s\0" as *const u8 as *const ::core::ffi::c_char,
        xcb_get_property_value_length(prop),
        xcb_get_property_value(prop) as *mut ::core::ffi::c_char,
    );
    free((*win).role as *mut ::core::ffi::c_void);
    (*win).role = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*win).role = new_role;
    verboselog(
        b"WM_WINDOW_ROLE changed to \"%s\"\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        (*win).role,
    );
    free(prop as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn window_update_type(
    mut window: *mut i3Window,
    mut reply: *mut xcb_get_property_reply_t,
) {
    let mut new_type: xcb_atom_t = xcb_get_preferred_window_type(reply);
    free(reply as *mut ::core::ffi::c_void);
    if new_type as ::core::ffi::c_long == XCB_NONE {
        debuglog(
            b"%s:%s:%d - cannot read _NET_WM_WINDOW_TYPE from window.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/window.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"window_update_type\0" as *const u8 as *const ::core::ffi::c_char,
            243 as ::core::ffi::c_int,
        );
        return;
    }
    (*window).window_type = new_type;
    verboselog(
        b"_NET_WM_WINDOW_TYPE changed to %i.\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        (*window).window_type,
    );
    run_assignments(window);
}
#[no_mangle]
pub unsafe extern "C" fn window_update_normal_hints(
    mut win: *mut i3Window,
    mut reply: *mut xcb_get_property_reply_t,
    mut geom: *mut xcb_get_geometry_reply_t,
) -> bool {
    let mut changed: bool = false_0 != 0;
    let mut size_hints: xcb_size_hints_t = xcb_size_hints_t {
        flags: 0,
        x: 0,
        y: 0,
        width: 0,
        height: 0,
        min_width: 0,
        min_height: 0,
        max_width: 0,
        max_height: 0,
        width_inc: 0,
        height_inc: 0,
        min_aspect_num: 0,
        min_aspect_den: 0,
        max_aspect_num: 0,
        max_aspect_den: 0,
        base_width: 0,
        base_height: 0,
        win_gravity: 0,
    };
    let mut success: bool = false;
    if !reply.is_null() {
        success = xcb_icccm_get_wm_size_hints_from_reply(&raw mut size_hints, reply) != 0;
    } else {
        success = xcb_icccm_get_wm_normal_hints_reply(
            conn,
            xcb_icccm_get_wm_normal_hints_unchecked(conn, (*win).id),
            &raw mut size_hints,
            ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
        ) != 0;
    }
    if !success {
        debuglog(
            b"%s:%s:%d - Could not get WM_NORMAL_HINTS\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/window.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"window_update_normal_hints\0" as *const u8 as *const ::core::ffi::c_char,
            269 as ::core::ffi::c_int,
        );
        return false_0 != 0;
    }
    if size_hints.flags & XCB_ICCCM_SIZE_HINT_P_MIN_SIZE as ::core::ffi::c_int as uint32_t != 0 {
        debuglog(
            b"%s:%s:%d - Minimum size: %d (width) x %d (height)\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/window.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"window_update_normal_hints\0" as *const u8 as *const ::core::ffi::c_char,
            282 as ::core::ffi::c_int,
            size_hints.min_width,
            size_hints.min_height,
        );
        if (*win).min_width as int32_t != size_hints.min_width {
            (*win).min_width = size_hints.min_width as ::core::ffi::c_int;
            changed = true_0 != 0;
        }
        if (*win).min_height as int32_t != size_hints.min_height {
            (*win).min_height = size_hints.min_height as ::core::ffi::c_int;
            changed = true_0 != 0;
        }
    }
    if size_hints.flags & XCB_ICCCM_SIZE_HINT_P_MAX_SIZE as ::core::ffi::c_int as uint32_t != 0 {
        debuglog(
            b"%s:%s:%d - Maximum size: %d (width) x %d (height)\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/window.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"window_update_normal_hints\0" as *const u8 as *const ::core::ffi::c_char,
            289 as ::core::ffi::c_int,
            size_hints.max_width,
            size_hints.max_height,
        );
        let mut max_width: ::core::ffi::c_int = max(
            0 as ::core::ffi::c_int,
            size_hints.max_width as ::core::ffi::c_int,
        );
        let mut max_height: ::core::ffi::c_int = max(
            0 as ::core::ffi::c_int,
            size_hints.max_height as ::core::ffi::c_int,
        );
        if (*win).max_width != max_width {
            (*win).max_width = max_width;
            changed = true_0 != 0;
        }
        if (*win).max_height != max_height {
            (*win).max_height = max_height;
            changed = true_0 != 0;
        }
    } else {
        debuglog(
            b"%s:%s:%d - Clearing maximum size\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b"../src/window.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"window_update_normal_hints\0" as *const u8 as *const ::core::ffi::c_char,
            297 as ::core::ffi::c_int,
        );
        if (*win).max_width != 0 as ::core::ffi::c_int {
            (*win).max_width = 0 as ::core::ffi::c_int;
            changed = true_0 != 0;
        }
        if (*win).max_height != 0 as ::core::ffi::c_int {
            (*win).max_height = 0 as ::core::ffi::c_int;
            changed = true_0 != 0;
        }
    }
    if size_hints.flags & XCB_ICCCM_SIZE_HINT_P_RESIZE_INC as ::core::ffi::c_int as uint32_t != 0 {
        debuglog(
            b"%s:%s:%d - Size increments: %d (width) x %d (height)\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/window.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"window_update_normal_hints\0" as *const u8 as *const ::core::ffi::c_char,
            304 as ::core::ffi::c_int,
            size_hints.width_inc,
            size_hints.height_inc,
        );
        if size_hints.width_inc > 0 as int32_t && size_hints.width_inc < 0xffff as int32_t {
            if (*win).width_increment as int32_t != size_hints.width_inc {
                (*win).width_increment = size_hints.width_inc as ::core::ffi::c_int;
                changed = true_0 != 0;
            }
        } else if (*win).width_increment != 0 as ::core::ffi::c_int {
            (*win).width_increment = 0 as ::core::ffi::c_int;
            changed = true_0 != 0;
        }
        if size_hints.height_inc > 0 as int32_t && size_hints.height_inc < 0xffff as int32_t {
            if (*win).height_increment as int32_t != size_hints.height_inc {
                (*win).height_increment = size_hints.height_inc as ::core::ffi::c_int;
                changed = true_0 != 0;
            }
        } else if (*win).height_increment != 0 as ::core::ffi::c_int {
            (*win).height_increment = 0 as ::core::ffi::c_int;
            changed = true_0 != 0;
        }
    } else {
        debuglog(
            b"%s:%s:%d - Clearing size increments\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b"../src/window.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"window_update_normal_hints\0" as *const u8 as *const ::core::ffi::c_char,
            318 as ::core::ffi::c_int,
        );
        if (*win).width_increment != 0 as ::core::ffi::c_int {
            (*win).width_increment = 0 as ::core::ffi::c_int;
            changed = true_0 != 0;
        }
        if (*win).height_increment != 0 as ::core::ffi::c_int {
            (*win).height_increment = 0 as ::core::ffi::c_int;
            changed = true_0 != 0;
        }
    }
    if size_hints.flags & XCB_ICCCM_SIZE_HINT_BASE_SIZE as ::core::ffi::c_int as uint32_t != 0
        && (*win).base_width >= 0 as ::core::ffi::c_int
        && (*win).base_height >= 0 as ::core::ffi::c_int
    {
        debuglog(
            b"%s:%s:%d - Base size: %d (width) x %d (height)\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/window.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"window_update_normal_hints\0" as *const u8 as *const ::core::ffi::c_char,
            327 as ::core::ffi::c_int,
            size_hints.base_width,
            size_hints.base_height,
        );
        if (*win).base_width as int32_t != size_hints.base_width {
            (*win).base_width = size_hints.base_width as ::core::ffi::c_int;
            changed = true_0 != 0;
        }
        if (*win).base_height as int32_t != size_hints.base_height {
            (*win).base_height = size_hints.base_height as ::core::ffi::c_int;
            changed = true_0 != 0;
        }
    } else {
        debuglog(
            b"%s:%s:%d - Clearing base size\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b"../src/window.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"window_update_normal_hints\0" as *const u8 as *const ::core::ffi::c_char,
            332 as ::core::ffi::c_int,
        );
        if (*win).base_width != 0 as ::core::ffi::c_int {
            (*win).base_width = 0 as ::core::ffi::c_int;
            changed = true_0 != 0;
        }
        if (*win).base_height != 0 as ::core::ffi::c_int {
            (*win).base_height = 0 as ::core::ffi::c_int;
            changed = true_0 != 0;
        }
    }
    if !geom.is_null()
        && (size_hints.flags & XCB_ICCCM_SIZE_HINT_US_POSITION as ::core::ffi::c_int as uint32_t
            != 0
            || size_hints.flags & XCB_ICCCM_SIZE_HINT_P_POSITION as ::core::ffi::c_int as uint32_t
                != 0)
        && (size_hints.flags & XCB_ICCCM_SIZE_HINT_US_SIZE as ::core::ffi::c_int as uint32_t != 0
            || size_hints.flags & XCB_ICCCM_SIZE_HINT_P_SIZE as ::core::ffi::c_int as uint32_t != 0)
    {
        debuglog(
            b"%s:%s:%d - Setting geometry x=%d y=%d w=%d h=%d\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/window.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"window_update_normal_hints\0" as *const u8 as *const ::core::ffi::c_char,
            341 as ::core::ffi::c_int,
            size_hints.x,
            size_hints.y,
            size_hints.width,
            size_hints.height,
        );
        (*geom).x = size_hints.x as int16_t;
        (*geom).y = size_hints.y as int16_t;
        (*geom).width = size_hints.width as uint16_t;
        (*geom).height = size_hints.height as uint16_t;
    }
    if size_hints.flags & XCB_ICCCM_SIZE_HINT_P_ASPECT as ::core::ffi::c_int as uint32_t != 0
        && size_hints.min_aspect_num >= 0 as int32_t
        && size_hints.min_aspect_den > 0 as int32_t
        && size_hints.max_aspect_num >= 0 as int32_t
        && size_hints.max_aspect_den > 0 as int32_t
    {
        let mut min_aspect: ::core::ffi::c_double = size_hints.min_aspect_num
            as ::core::ffi::c_double
            / size_hints.min_aspect_den as ::core::ffi::c_double;
        let mut max_aspect: ::core::ffi::c_double = size_hints.max_aspect_num
            as ::core::ffi::c_double
            / size_hints.max_aspect_den as ::core::ffi::c_double;
        debuglog(
            b"%s:%s:%d - Aspect ratio set: minimum %f, maximum %f\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/window.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"window_update_normal_hints\0" as *const u8 as *const ::core::ffi::c_char,
            355 as ::core::ffi::c_int,
            min_aspect,
            max_aspect,
        );
        if fabs((*win).min_aspect_ratio - min_aspect) > DBL_EPSILON {
            (*win).min_aspect_ratio = min_aspect;
            changed = true_0 != 0;
        }
        if fabs((*win).max_aspect_ratio - max_aspect) > DBL_EPSILON {
            (*win).max_aspect_ratio = max_aspect;
            changed = true_0 != 0;
        }
    } else {
        debuglog(
            b"%s:%s:%d - Clearing aspect ratios\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b"../src/window.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"window_update_normal_hints\0" as *const u8 as *const ::core::ffi::c_char,
            365 as ::core::ffi::c_int,
        );
        if (*win).min_aspect_ratio != 0.0f64 {
            (*win).min_aspect_ratio = 0.0f64;
            changed = true_0 != 0;
        }
        if (*win).max_aspect_ratio != 0.0f64 {
            (*win).max_aspect_ratio = 0.0f64;
            changed = true_0 != 0;
        }
    }
    return changed;
}
#[no_mangle]
pub unsafe extern "C" fn window_update_hints(
    mut win: *mut i3Window,
    mut prop: *mut xcb_get_property_reply_t,
    mut urgency_hint: *mut bool,
) {
    if !urgency_hint.is_null() {
        *urgency_hint = false_0 != 0;
    }
    if prop.is_null() || xcb_get_property_value_length(prop) == 0 as ::core::ffi::c_int {
        debuglog(
            b"%s:%s:%d - WM_HINTS not set.\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b"../src/window.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"window_update_hints\0" as *const u8 as *const ::core::ffi::c_char,
            384 as ::core::ffi::c_int,
        );
        free(prop as *mut ::core::ffi::c_void);
        prop = ::core::ptr::null_mut::<xcb_get_property_reply_t>();
        return;
    }
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
    if xcb_icccm_get_wm_hints_from_reply(&raw mut hints, prop) == 0 {
        debuglog(
            b"%s:%s:%d - Could not get WM_HINTS\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b"../src/window.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"window_update_hints\0" as *const u8 as *const ::core::ffi::c_char,
            392 as ::core::ffi::c_int,
        );
        free(prop as *mut ::core::ffi::c_void);
        return;
    }
    if hints.flags & XCB_ICCCM_WM_HINT_INPUT as ::core::ffi::c_int as int32_t != 0 {
        (*win).doesnt_accept_focus = hints.input == 0;
        verboselog(
            b"WM_HINTS.input changed to \"%d\"\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            hints.input,
        );
    }
    if !urgency_hint.is_null() {
        *urgency_hint = xcb_icccm_wm_hints_get_urgency(&raw mut hints) != 0 as uint32_t;
    }
    free(prop as *mut ::core::ffi::c_void);
}
pub const MWM_HINTS_FLAGS_FIELD: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MWM_HINTS_DECORATIONS_FIELD: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MWM_HINTS_DECORATIONS: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
pub const MWM_DECOR_ALL: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
pub const MWM_DECOR_BORDER: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
pub const MWM_DECOR_TITLE: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
unsafe extern "C" fn border_style_from_motif_value(mut value: uint32_t) -> border_style_t {
    if value & MWM_DECOR_ALL as uint32_t != 0 {
        if value & MWM_DECOR_TITLE as uint32_t != 0 {
            if value & MWM_DECOR_BORDER as uint32_t != 0 {
                return BS_NONE;
            }
            return BS_PIXEL;
        }
        return BS_NORMAL;
    }
    if value & MWM_DECOR_TITLE as uint32_t != 0 {
        return BS_NORMAL;
    }
    if value & MWM_DECOR_BORDER as uint32_t != 0 {
        return BS_PIXEL;
    }
    return BS_NONE;
}
#[no_mangle]
pub unsafe extern "C" fn window_update_motif_hints(
    mut win: *mut i3Window,
    mut prop: *mut xcb_get_property_reply_t,
    mut motif_border_style: *mut border_style_t,
) -> bool {
    if prop.is_null() {
        return false_0 != 0;
    }
    if motif_border_style != ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut border_style_t {
    } else {
        __assert_fail(
            b"motif_border_style != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            b"../src/window.c\0" as *const u8 as *const ::core::ffi::c_char,
            469 as ::core::ffi::c_uint,
            __ASSERT_FUNCTION.as_ptr(),
        );
    };
    if xcb_get_property_value_length(prop) == 0 as ::core::ffi::c_int {
        free(prop as *mut ::core::ffi::c_void);
        prop = ::core::ptr::null_mut::<xcb_get_property_reply_t>();
        return false_0 != 0;
    }
    let mut motif_hints: *mut uint32_t = xcb_get_property_value(prop) as *mut uint32_t;
    if *motif_hints.offset(MWM_HINTS_FLAGS_FIELD as isize) & MWM_HINTS_DECORATIONS as uint32_t != 0
    {
        *motif_border_style = border_style_from_motif_value(
            *motif_hints.offset(MWM_HINTS_DECORATIONS_FIELD as isize),
        );
        free(prop as *mut ::core::ffi::c_void);
        prop = ::core::ptr::null_mut::<xcb_get_property_reply_t>();
        return true_0 != 0;
    }
    free(prop as *mut ::core::ffi::c_void);
    prop = ::core::ptr::null_mut::<xcb_get_property_reply_t>();
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn window_update_machine(
    mut win: *mut i3Window,
    mut prop: *mut xcb_get_property_reply_t,
) {
    if prop.is_null() || xcb_get_property_value_length(prop) == 0 as ::core::ffi::c_int {
        debuglog(
            b"%s:%s:%d - WM_CLIENT_MACHINE not set.\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b"../src/window.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"window_update_machine\0" as *const u8 as *const ::core::ffi::c_char,
            509 as ::core::ffi::c_int,
        );
        free(prop as *mut ::core::ffi::c_void);
        prop = ::core::ptr::null_mut::<xcb_get_property_reply_t>();
        return;
    }
    free((*win).machine as *mut ::core::ffi::c_void);
    (*win).machine = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*win).machine = sstrndup(
        xcb_get_property_value(prop) as *mut ::core::ffi::c_char,
        xcb_get_property_value_length(prop) as size_t,
    );
    verboselog(
        b"WM_CLIENT_MACHINE changed to \"%s\"\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        (*win).machine,
    );
    free(prop as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn window_update_icon(
    mut win: *mut i3Window,
    mut prop: *mut xcb_get_property_reply_t,
) {
    let mut data: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
    let mut width: uint32_t = 0 as uint32_t;
    let mut height: uint32_t = 0 as uint32_t;
    let mut len: uint64_t = 0 as uint64_t;
    let pref_size: uint32_t =
        (render_deco_height() - logical_px(2 as ::core::ffi::c_int)) as uint32_t;
    if prop.is_null()
        || (*prop).type_0 != XCB_ATOM_CARDINAL as ::core::ffi::c_int as xcb_atom_t
        || (*prop).format as ::core::ffi::c_int != 32 as ::core::ffi::c_int
    {
        debuglog(
            b"%s:%s:%d - _NET_WM_ICON is not set\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b"../src/window.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"window_update_icon\0" as *const u8 as *const ::core::ffi::c_char,
            528 as ::core::ffi::c_int,
        );
        free(prop as *mut ::core::ffi::c_void);
        prop = ::core::ptr::null_mut::<xcb_get_property_reply_t>();
        return;
    }
    let mut prop_value_len: uint32_t = xcb_get_property_value_length(prop) as uint32_t;
    let mut prop_value: *mut uint32_t = xcb_get_property_value(prop) as *mut uint32_t;
    while prop_value_len as usize
        > (::core::mem::size_of::<uint32_t>() as usize).wrapping_mul(2 as usize)
        && !prop_value.is_null()
        && *prop_value.offset(0 as ::core::ffi::c_int as isize) != 0
        && *prop_value.offset(1 as ::core::ffi::c_int as isize) != 0
    {
        let cur_width: uint32_t = *prop_value.offset(0 as ::core::ffi::c_int as isize);
        let cur_height: uint32_t = *prop_value.offset(1 as ::core::ffi::c_int as isize);
        let cur_len: uint64_t = (cur_width as uint64_t).wrapping_mul(cur_height as uint64_t);
        let expected_len: uint64_t = cur_len
            .wrapping_add(2 as uint64_t)
            .wrapping_mul(4 as uint64_t);
        if expected_len > prop_value_len as uint64_t {
            break;
        }
        debuglog(
            b"%s:%s:%d - Found _NET_WM_ICON of size: (%d,%d)\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/window.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"window_update_icon\0" as *const u8 as *const ::core::ffi::c_char,
            555 as ::core::ffi::c_int,
            cur_width,
            cur_height,
        );
        let at_least_preferred_size: bool = cur_width >= pref_size && cur_height >= pref_size;
        let smaller_than_current: bool = cur_width < width || cur_height < height;
        let larger_than_current: bool = cur_width > width || cur_height > height;
        let not_yet_at_preferred: bool = width < pref_size || height < pref_size;
        if len == 0 as uint64_t
            || at_least_preferred_size as ::core::ffi::c_int != 0
                && (smaller_than_current as ::core::ffi::c_int != 0
                    || not_yet_at_preferred as ::core::ffi::c_int != 0)
            || !at_least_preferred_size
                && not_yet_at_preferred as ::core::ffi::c_int != 0
                && larger_than_current as ::core::ffi::c_int != 0
        {
            len = cur_len;
            width = cur_width;
            height = cur_height;
            data = prop_value;
        }
        if width == pref_size && height == pref_size {
            break;
        }
        prop_value_len =
            (prop_value_len as uint64_t).wrapping_sub(expected_len) as uint32_t as uint32_t;
        prop_value = (prop_value as *mut uint8_t).offset(expected_len as isize) as *mut uint32_t;
    }
    if data.is_null() {
        debuglog(
            b"%s:%s:%d - Could not get _NET_WM_ICON\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b"../src/window.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"window_update_icon\0" as *const u8 as *const ::core::ffi::c_char,
            587 as ::core::ffi::c_int,
        );
        free(prop as *mut ::core::ffi::c_void);
        prop = ::core::ptr::null_mut::<xcb_get_property_reply_t>();
        return;
    }
    debuglog(
        b"%s:%s:%d - Using icon of size (%d,%d) (preferred size: %d)\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/window.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"window_update_icon\0" as *const u8 as *const ::core::ffi::c_char,
        593 as ::core::ffi::c_int,
        width,
        height,
        pref_size,
    );
    (*win).name_x_changed = true_0 != 0;
    let mut icon: *mut uint32_t =
        smalloc((len as size_t).wrapping_mul(4 as size_t)) as *mut uint32_t;
    let mut i: uint64_t = 0 as uint64_t;
    while i < len {
        let pixel: uint32_t = *data.offset((2 as uint64_t).wrapping_add(i) as isize);
        let a: uint8_t = (pixel >> 24 as ::core::ffi::c_int & 0xff as uint32_t) as uint8_t;
        let mut r: uint8_t = (pixel >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as uint8_t;
        let mut g: uint8_t = (pixel >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as uint8_t;
        let mut b: uint8_t = (pixel >> 0 as ::core::ffi::c_int & 0xff as uint32_t) as uint8_t;
        r = (r as ::core::ffi::c_int * a as ::core::ffi::c_int / 0xff as ::core::ffi::c_int)
            as uint8_t;
        g = (g as ::core::ffi::c_int * a as ::core::ffi::c_int / 0xff as ::core::ffi::c_int)
            as uint8_t;
        b = (b as ::core::ffi::c_int * a as ::core::ffi::c_int / 0xff as ::core::ffi::c_int)
            as uint8_t;
        *icon.offset(i as isize) = (a as uint32_t) << 24 as ::core::ffi::c_int
            | ((r as ::core::ffi::c_int) << 16 as ::core::ffi::c_int) as uint32_t
            | ((g as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as uint32_t
            | b as uint32_t;
        i = i.wrapping_add(1);
    }
    if !(*win).icon.is_null() {
        cairo_surface_destroy((*win).icon);
    }
    (*win).icon = cairo_image_surface_create_for_data(
        icon as *mut ::core::ffi::c_uchar,
        CAIRO_FORMAT_ARGB32,
        width as ::core::ffi::c_int,
        height as ::core::ffi::c_int,
        width.wrapping_mul(4 as uint32_t) as ::core::ffi::c_int,
    );
    static mut free_data: cairo_user_data_key_t = _cairo_user_data_key { unused: 0 };
    cairo_surface_set_user_data(
        (*win).icon,
        &raw mut free_data,
        icon as *mut ::core::ffi::c_void,
        Some(free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
    );
    free(prop as *mut ::core::ffi::c_void);
    prop = ::core::ptr::null_mut::<xcb_get_property_reply_t>();
}
pub const __DBL_EPSILON__: ::core::ffi::c_double = 2.2204460492503131e-16f64;
