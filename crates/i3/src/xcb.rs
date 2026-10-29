/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * xcb.c: Helper functions for easier usage of XCB
 *
 */
// Mechanical file-by-file Rust translation of src/xcb.c.
// Generated with C2Rust 0.22.1; C ABI and original ownership retained.
extern "C" {
    pub type xcb_connection_t;
    pub type _cairo;
    pub type _cairo_surface;
    pub type _i3String;
    pub type pcre2_real_code_8;
    pub type ev_loop;
    fn xcb_visualtype_next(i: *mut xcb_visualtype_iterator_t);
    fn xcb_depth_visuals_iterator(R: *const xcb_depth_t) -> xcb_visualtype_iterator_t;
    fn xcb_depth_next(i: *mut xcb_depth_iterator_t);
    fn xcb_screen_allowed_depths_iterator(R: *const xcb_screen_t) -> xcb_depth_iterator_t;
    fn xcb_create_window(
        c: *mut xcb_connection_t,
        depth: uint8_t,
        wid: xcb_window_t,
        parent: xcb_window_t,
        x: int16_t,
        y: int16_t,
        width: uint16_t,
        height: uint16_t,
        border_width: uint16_t,
        _class: uint16_t,
        visual: xcb_visualid_t,
        value_mask: uint32_t,
        value_list: *const ::core::ffi::c_void,
    ) -> xcb_void_cookie_t;
    fn xcb_change_window_attributes(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        value_mask: uint32_t,
        value_list: *const ::core::ffi::c_void,
    ) -> xcb_void_cookie_t;
    fn xcb_map_window(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t;
    fn xcb_configure_window(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        value_mask: uint16_t,
        value_list: *const ::core::ffi::c_void,
    ) -> xcb_void_cookie_t;
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
    fn xcb_get_property(
        c: *mut xcb_connection_t,
        _delete: uint8_t,
        window: xcb_window_t,
        property: xcb_atom_t,
        type_0: xcb_atom_t,
        long_offset: uint32_t,
        long_length: uint32_t,
    ) -> xcb_get_property_cookie_t;
    fn xcb_get_property_value(R: *const xcb_get_property_reply_t) -> *mut ::core::ffi::c_void;
    fn xcb_get_property_value_length(R: *const xcb_get_property_reply_t) -> ::core::ffi::c_int;
    fn xcb_get_property_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_get_property_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_get_property_reply_t;
    fn xcb_send_event(
        c: *mut xcb_connection_t,
        propagate: uint8_t,
        destination: xcb_window_t,
        event_mask: uint32_t,
        event: *const ::core::ffi::c_char,
    ) -> xcb_void_cookie_t;
    fn xcb_grab_button(
        c: *mut xcb_connection_t,
        owner_events: uint8_t,
        grab_window: xcb_window_t,
        event_mask: uint16_t,
        pointer_mode: uint8_t,
        keyboard_mode: uint8_t,
        confine_to: xcb_window_t,
        cursor: xcb_cursor_t,
        button: uint8_t,
        modifiers: uint16_t,
    ) -> xcb_void_cookie_t;
    fn xcb_grab_server(c: *mut xcb_connection_t) -> xcb_void_cookie_t;
    fn xcb_ungrab_server(c: *mut xcb_connection_t) -> xcb_void_cookie_t;
    fn xcb_request_check(
        c: *mut xcb_connection_t,
        cookie: xcb_void_cookie_t,
    ) -> *mut xcb_generic_error_t;
    fn xcb_generate_id(c: *mut xcb_connection_t) -> uint32_t;
    static mut conn: *mut xcb_connection_t;
    static mut root_screen: *mut xcb_screen_t;
    fn errorlog(fmt: *mut ::core::ffi::c_char, ...);
    fn debuglog(fmt: *mut ::core::ffi::c_char, ...);
    fn scalloc(num: size_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn fake_configure_notify(
        conn_0: *mut xcb_connection_t,
        r: xcb_rectangle_t,
        window: xcb_window_t,
        border_width: ::core::ffi::c_int,
    );
    static mut root: xcb_window_t;
    fn xcursor_get_cursor(c: xcursor_cursor_t) -> xcb_cursor_t;
    static mut A__NET_WM_WINDOW_TYPE_TOOLTIP: xcb_atom_t;
    static mut A__NET_WM_WINDOW_TYPE_DIALOG: xcb_atom_t;
    static mut A__NET_WM_WINDOW_TYPE_UTILITY: xcb_atom_t;
    static mut A__NET_WM_WINDOW_TYPE_TOOLBAR: xcb_atom_t;
    static mut A__NET_WM_WINDOW_TYPE_SPLASH: xcb_atom_t;
    static mut A__NET_WM_WINDOW_TYPE_MENU: xcb_atom_t;
    static mut A__NET_WM_WINDOW_TYPE_DROPDOWN_MENU: xcb_atom_t;
    static mut A__NET_WM_WINDOW_TYPE_POPUP_MENU: xcb_atom_t;
    static mut A__NET_WM_WINDOW_TYPE_NOTIFICATION: xcb_atom_t;
    static mut A__NET_WM_WINDOW_TYPE_NORMAL: xcb_atom_t;
    static mut A_WM_PROTOCOLS: xcb_atom_t;
    static mut A_WM_TAKE_FOCUS: xcb_atom_t;
    fn add_ignore_event(sequence: ignore_event_sequence_t, response_type: ::core::ffi::c_int);
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
pub type xcb_visualid_t = uint32_t;
pub type xcb_timestamp_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_rectangle_t {
    pub x: int16_t,
    pub y: int16_t,
    pub width: uint16_t,
    pub height: uint16_t,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_visualtype_iterator_t {
    pub data: *mut xcb_visualtype_t,
    pub rem: ::core::ffi::c_int,
    pub index: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_depth_t {
    pub depth: uint8_t,
    pub pad0: uint8_t,
    pub visuals_len: uint16_t,
    pub pad1: [uint8_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_depth_iterator_t {
    pub data: *mut xcb_depth_t,
    pub rem: ::core::ffi::c_int,
    pub index: ::core::ffi::c_int,
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
pub type xcb_button_mask_t = ::core::ffi::c_uint;
pub const XCB_BUTTON_MASK_ANY: xcb_button_mask_t = 32768;
pub const XCB_BUTTON_MASK_5: xcb_button_mask_t = 4096;
pub const XCB_BUTTON_MASK_4: xcb_button_mask_t = 2048;
pub const XCB_BUTTON_MASK_3: xcb_button_mask_t = 1024;
pub const XCB_BUTTON_MASK_2: xcb_button_mask_t = 512;
pub const XCB_BUTTON_MASK_1: xcb_button_mask_t = 256;
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
pub type xcb_prop_mode_t = ::core::ffi::c_uint;
pub const XCB_PROP_MODE_APPEND: xcb_prop_mode_t = 2;
pub const XCB_PROP_MODE_PREPEND: xcb_prop_mode_t = 1;
pub const XCB_PROP_MODE_REPLACE: xcb_prop_mode_t = 0;
pub type xcb_get_property_type_t = ::core::ffi::c_uint;
pub const XCB_GET_PROPERTY_TYPE_ANY: xcb_get_property_type_t = 0;
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
pub type xcb_grab_mode_t = ::core::ffi::c_uint;
pub const XCB_GRAB_MODE_ASYNC: xcb_grab_mode_t = 1;
pub const XCB_GRAB_MODE_SYNC: xcb_grab_mode_t = 0;
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
pub struct ignore_event_sequence_t {
    pub value: uint32_t,
}
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
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const XCB_CLIENT_MESSAGE: ::core::ffi::c_int = 33 as ::core::ffi::c_int;
pub const XCB_NONE: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const XCB_COPY_FROM_PARENT: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[no_mangle]
pub static mut xcb_numlock_mask: ::core::ffi::c_uint = 0;
#[no_mangle]
pub unsafe extern "C" fn create_window(
    mut conn_0: *mut xcb_connection_t,
    mut dims: Rect,
    mut depth: uint16_t,
    mut visual: xcb_visualid_t,
    mut window_class: uint16_t,
    mut cursor: xcursor_cursor_t,
    mut map: bool,
    mut mask: uint32_t,
    mut values: *mut uint32_t,
) -> xcb_window_t {
    let mut result: xcb_window_t = xcb_generate_id(conn_0) as xcb_window_t;
    if window_class as ::core::ffi::c_int == XCB_WINDOW_CLASS_INPUT_ONLY as ::core::ffi::c_int {
        depth = XCB_COPY_FROM_PARENT as uint16_t;
        visual = XCB_COPY_FROM_PARENT as xcb_visualid_t;
    }
    let mut gc_cookie: xcb_void_cookie_t = xcb_create_window(
        conn_0,
        depth as uint8_t,
        result,
        root,
        dims.x as int16_t,
        dims.y as int16_t,
        dims.width as uint16_t,
        dims.height as uint16_t,
        0 as uint16_t,
        window_class,
        visual,
        mask,
        values as *const ::core::ffi::c_void,
    );
    let mut error: *mut xcb_generic_error_t = xcb_request_check(conn_0, gc_cookie);
    if !error.is_null() {
        errorlog(
            b"ERROR: Could not create window. Error code: %d.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            (*error).error_code as ::core::ffi::c_int,
        );
    }
    let mut cursor_values: [uint32_t; 1] = [xcursor_get_cursor(cursor)];
    xcb_change_window_attributes(
        conn_0,
        result,
        XCB_CW_CURSOR as ::core::ffi::c_int as uint32_t,
        &raw mut cursor_values as *mut uint32_t as *const ::core::ffi::c_void,
    );
    if map {
        xcb_map_window(conn_0, result);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn fake_absolute_configure_notify(mut con: *mut Con) {
    let mut absolute: xcb_rectangle_t = xcb_rectangle_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    if (*con).window.is_null() {
        return;
    }
    absolute.x = (*con).rect.x.wrapping_add((*con).window_rect.x) as int16_t;
    absolute.y = (*con).rect.y.wrapping_add((*con).window_rect.y) as int16_t;
    absolute.width = (*con).window_rect.width as uint16_t;
    absolute.height = (*con).window_rect.height as uint16_t;
    debuglog(
        b"%s:%s:%d - fake rect = (%d, %d, %d, %d)\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/xcb.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"fake_absolute_configure_notify\0" as *const u8 as *const ::core::ffi::c_char,
        75 as ::core::ffi::c_int,
        absolute.x as ::core::ffi::c_int,
        absolute.y as ::core::ffi::c_int,
        absolute.width as ::core::ffi::c_int,
        absolute.height as ::core::ffi::c_int,
    );
    fake_configure_notify(conn, absolute, (*(*con).window).id, (*con).border_width);
}
#[no_mangle]
pub unsafe extern "C" fn send_take_focus(mut window: xcb_window_t, mut timestamp: xcb_timestamp_t) {
    let mut event: *mut ::core::ffi::c_void = scalloc(32 as size_t, 1 as size_t);
    let mut ev: *mut xcb_client_message_event_t = event as *mut xcb_client_message_event_t;
    (*ev).response_type = XCB_CLIENT_MESSAGE as uint8_t;
    (*ev).window = window;
    (*ev).type_0 = A_WM_PROTOCOLS;
    (*ev).format = 32 as uint8_t;
    (*ev).data.data32[0 as ::core::ffi::c_int as usize] = A_WM_TAKE_FOCUS as uint32_t;
    (*ev).data.data32[1 as ::core::ffi::c_int as usize] = timestamp as uint32_t;
    debuglog(
        b"%s:%s:%d - Sending WM_TAKE_FOCUS to the client\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/xcb.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"send_take_focus\0" as *const u8 as *const ::core::ffi::c_char,
        98 as ::core::ffi::c_int,
    );
    xcb_send_event(
        conn,
        false_0 as uint8_t,
        window,
        XCB_EVENT_MASK_NO_EVENT as ::core::ffi::c_int as uint32_t,
        ev as *mut ::core::ffi::c_char,
    );
    free(event);
}
#[no_mangle]
pub unsafe extern "C" fn xcb_set_window_rect(
    mut conn_0: *mut xcb_connection_t,
    window: xcb_window_t,
    mut r: Rect,
) {
    let mut cookie: xcb_void_cookie_t = xcb_configure_window(
        conn_0,
        window,
        (XCB_CONFIG_WINDOW_X as ::core::ffi::c_int
            | XCB_CONFIG_WINDOW_Y as ::core::ffi::c_int
            | XCB_CONFIG_WINDOW_WIDTH as ::core::ffi::c_int
            | XCB_CONFIG_WINDOW_HEIGHT as ::core::ffi::c_int) as uint16_t,
        &raw mut r.x as *const ::core::ffi::c_void,
    );
    add_ignore_event(
        ignore_event_sequence_t {
            value: cookie.sequence as uint32_t,
        },
        -(1 as ::core::ffi::c_int),
    );
}
#[no_mangle]
pub unsafe extern "C" fn xcb_get_preferred_window_type(
    mut reply: *mut xcb_get_property_reply_t,
) -> xcb_atom_t {
    if reply.is_null() || xcb_get_property_value_length(reply) == 0 as ::core::ffi::c_int {
        return XCB_NONE as xcb_atom_t;
    }
    let mut atoms: *mut xcb_atom_t = ::core::ptr::null_mut::<xcb_atom_t>();
    atoms = xcb_get_property_value(reply) as *mut xcb_atom_t;
    if atoms.is_null() {
        return XCB_NONE as xcb_atom_t;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < xcb_get_property_value_length(reply)
        / ((*reply).format as ::core::ffi::c_int / 8 as ::core::ffi::c_int)
    {
        if *atoms.offset(i as isize) == A__NET_WM_WINDOW_TYPE_NORMAL
            || *atoms.offset(i as isize) == A__NET_WM_WINDOW_TYPE_DIALOG
            || *atoms.offset(i as isize) == A__NET_WM_WINDOW_TYPE_UTILITY
            || *atoms.offset(i as isize) == A__NET_WM_WINDOW_TYPE_TOOLBAR
            || *atoms.offset(i as isize) == A__NET_WM_WINDOW_TYPE_SPLASH
            || *atoms.offset(i as isize) == A__NET_WM_WINDOW_TYPE_MENU
            || *atoms.offset(i as isize) == A__NET_WM_WINDOW_TYPE_DROPDOWN_MENU
            || *atoms.offset(i as isize) == A__NET_WM_WINDOW_TYPE_POPUP_MENU
            || *atoms.offset(i as isize) == A__NET_WM_WINDOW_TYPE_TOOLTIP
            || *atoms.offset(i as isize) == A__NET_WM_WINDOW_TYPE_NOTIFICATION
        {
            return *atoms.offset(i as isize);
        }
        i += 1;
    }
    return XCB_NONE as xcb_atom_t;
}
#[no_mangle]
pub unsafe extern "C" fn xcb_reply_contains_atom(
    mut prop: *mut xcb_get_property_reply_t,
    mut atom: xcb_atom_t,
) -> bool {
    if prop.is_null() || xcb_get_property_value_length(prop) == 0 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    let mut atoms: *mut xcb_atom_t = ::core::ptr::null_mut::<xcb_atom_t>();
    atoms = xcb_get_property_value(prop) as *mut xcb_atom_t;
    if atoms.is_null() {
        return false_0 != 0;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < xcb_get_property_value_length(prop)
        / ((*prop).format as ::core::ffi::c_int / 8 as ::core::ffi::c_int)
    {
        if *atoms.offset(i as isize) == atom {
            return true_0 != 0;
        }
        i += 1;
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn get_visualtype_by_id(
    mut visual_id: xcb_visualid_t,
) -> *mut xcb_visualtype_t {
    let mut depth_iter: xcb_depth_iterator_t = xcb_depth_iterator_t {
        data: ::core::ptr::null_mut::<xcb_depth_t>(),
        rem: 0,
        index: 0,
    };
    depth_iter = xcb_screen_allowed_depths_iterator(root_screen);
    while depth_iter.rem != 0 {
        let mut visual_iter: xcb_visualtype_iterator_t = xcb_visualtype_iterator_t {
            data: ::core::ptr::null_mut::<xcb_visualtype_t>(),
            rem: 0,
            index: 0,
        };
        visual_iter = xcb_depth_visuals_iterator(depth_iter.data);
        while visual_iter.rem != 0 {
            if visual_id == (*visual_iter.data).visual_id {
                return visual_iter.data;
            }
            xcb_visualtype_next(&raw mut visual_iter);
        }
        xcb_depth_next(&raw mut depth_iter);
    }
    return ::core::ptr::null_mut::<xcb_visualtype_t>();
}
#[no_mangle]
pub unsafe extern "C" fn get_visualid_by_depth(mut depth: uint16_t) -> xcb_visualid_t {
    let mut depth_iter: xcb_depth_iterator_t = xcb_depth_iterator_t {
        data: ::core::ptr::null_mut::<xcb_depth_t>(),
        rem: 0,
        index: 0,
    };
    depth_iter = xcb_screen_allowed_depths_iterator(root_screen);
    while depth_iter.rem != 0 {
        if !((*depth_iter.data).depth as ::core::ffi::c_int != depth as ::core::ffi::c_int) {
            let mut visual_iter: xcb_visualtype_iterator_t =
                xcb_depth_visuals_iterator(depth_iter.data);
            if !(visual_iter.rem == 0) {
                return (*visual_iter.data).visual_id;
            }
        }
        xcb_depth_next(&raw mut depth_iter);
    }
    return 0 as xcb_visualid_t;
}
#[no_mangle]
pub unsafe extern "C" fn xcb_add_property_atom(
    mut conn_0: *mut xcb_connection_t,
    mut window: xcb_window_t,
    mut property: xcb_atom_t,
    mut atom: xcb_atom_t,
) {
    let mut fresh0: [uint32_t; 1] = [atom];
    xcb_change_property(
        conn_0,
        XCB_PROP_MODE_APPEND as ::core::ffi::c_int as uint8_t,
        window,
        property,
        XCB_ATOM_ATOM as ::core::ffi::c_int as xcb_atom_t,
        32 as uint8_t,
        1 as uint32_t,
        &raw mut fresh0 as *mut uint32_t as *const ::core::ffi::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn xcb_remove_property_atom(
    mut conn_0: *mut xcb_connection_t,
    mut window: xcb_window_t,
    mut property: xcb_atom_t,
    mut atom: xcb_atom_t,
) {
    let mut atoms: *mut xcb_atom_t = ::core::ptr::null_mut::<xcb_atom_t>();
    xcb_grab_server(conn_0);
    let mut reply: *mut xcb_get_property_reply_t = xcb_get_property_reply(
        conn_0,
        xcb_get_property(
            conn_0,
            false_0 as uint8_t,
            window,
            property,
            XCB_GET_PROPERTY_TYPE_ANY as ::core::ffi::c_int as xcb_atom_t,
            0 as uint32_t,
            4096 as uint32_t,
        ),
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if !(reply.is_null() || xcb_get_property_value_length(reply) == 0 as ::core::ffi::c_int) {
        atoms = xcb_get_property_value(reply) as *mut xcb_atom_t;
        if !atoms.is_null() {
            let mut num: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let current_size: ::core::ffi::c_int = xcb_get_property_value_length(reply)
                as ::core::ffi::c_int
                / ((*reply).format as ::core::ffi::c_int / 8 as ::core::ffi::c_int);
            let vla = current_size as usize;
            let mut values: Vec<xcb_atom_t> = ::std::vec::from_elem(0, vla);
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i < current_size {
                if *atoms.offset(i as isize) != atom {
                    let fresh1 = num;
                    num = num + 1;
                    *values.as_mut_ptr().offset(fresh1 as isize) = *atoms.offset(i as isize);
                }
                i += 1;
            }
            xcb_change_property(
                conn_0,
                XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as uint8_t,
                window,
                property,
                XCB_ATOM_ATOM as ::core::ffi::c_int as xcb_atom_t,
                32 as uint8_t,
                num as uint32_t,
                values.as_mut_ptr() as *const ::core::ffi::c_void,
            );
        }
    }
    free(reply as *mut ::core::ffi::c_void);
    reply = ::core::ptr::null_mut::<xcb_get_property_reply_t>();
    xcb_ungrab_server(conn_0);
}
#[no_mangle]
pub unsafe extern "C" fn xcb_grab_buttons(
    mut conn_0: *mut xcb_connection_t,
    mut window: xcb_window_t,
    mut buttons: *mut ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while *buttons.offset(i as isize) > 0 as ::core::ffi::c_int {
        xcb_grab_button(
            conn_0,
            false_0 as uint8_t,
            window,
            XCB_EVENT_MASK_BUTTON_PRESS as ::core::ffi::c_int as uint16_t,
            XCB_GRAB_MODE_SYNC as ::core::ffi::c_int as uint8_t,
            XCB_GRAB_MODE_ASYNC as ::core::ffi::c_int as uint8_t,
            root,
            XCB_NONE as xcb_cursor_t,
            *buttons.offset(i as isize) as uint8_t,
            XCB_BUTTON_MASK_ANY as ::core::ffi::c_int as uint16_t,
        );
        i += 1;
    }
}
