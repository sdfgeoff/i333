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
    fn xcb_change_window_attributes(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        value_mask: uint32_t,
        value_list: *const ::core::ffi::c_void,
    ) -> xcb_void_cookie_t;
    fn xcb_destroy_window(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t;
    fn xcb_reparent_window(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        parent: xcb_window_t,
        x: int16_t,
        y: int16_t,
    ) -> xcb_void_cookie_t;
    fn xcb_map_window(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t;
    fn xcb_unmap_window(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t;
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
    fn xcb_delete_property(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        property: xcb_atom_t,
    ) -> xcb_void_cookie_t;
    fn xcb_send_event(
        c: *mut xcb_connection_t,
        propagate: uint8_t,
        destination: xcb_window_t,
        event_mask: uint32_t,
        event: *const ::core::ffi::c_char,
    ) -> xcb_void_cookie_t;
    fn xcb_query_pointer(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_query_pointer_cookie_t;
    fn xcb_query_pointer_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_query_pointer_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_query_pointer_reply_t;
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
    fn xcb_set_input_focus(
        c: *mut xcb_connection_t,
        revert_to: uint8_t,
        focus: xcb_window_t,
        time: xcb_timestamp_t,
    ) -> xcb_void_cookie_t;
    fn xcb_create_pixmap(
        c: *mut xcb_connection_t,
        depth: uint8_t,
        pid: xcb_pixmap_t,
        drawable: xcb_drawable_t,
        width: uint16_t,
        height: uint16_t,
    ) -> xcb_void_cookie_t;
    fn xcb_free_pixmap(c: *mut xcb_connection_t, pixmap: xcb_pixmap_t) -> xcb_void_cookie_t;
    fn xcb_change_gc(
        c: *mut xcb_connection_t,
        gc: xcb_gcontext_t,
        value_mask: uint32_t,
        value_list: *const ::core::ffi::c_void,
    ) -> xcb_void_cookie_t;
    fn xcb_create_colormap(
        c: *mut xcb_connection_t,
        alloc: uint8_t,
        mid: xcb_colormap_t,
        window: xcb_window_t,
        visual: xcb_visualid_t,
    ) -> xcb_void_cookie_t;
    fn xcb_free_colormap(c: *mut xcb_connection_t, cmap: xcb_colormap_t) -> xcb_void_cookie_t;
    fn xcb_kill_client(c: *mut xcb_connection_t, resource: uint32_t) -> xcb_void_cookie_t;
    fn xcb_flush(c: *mut xcb_connection_t) -> ::core::ffi::c_int;
    fn xcb_generate_id(c: *mut xcb_connection_t) -> uint32_t;
    fn xcb_icccm_get_wm_protocols(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        wm_protocol_atom: xcb_atom_t,
    ) -> xcb_get_property_cookie_t;
    fn xcb_icccm_get_wm_protocols_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_get_property_cookie_t,
        protocols: *mut xcb_icccm_get_wm_protocols_reply_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> uint8_t;
    fn xcb_icccm_get_wm_protocols_reply_wipe(protocols: *mut xcb_icccm_get_wm_protocols_reply_t);
    fn getpid() -> __pid_t;
    static mut conn: *mut xcb_connection_t;
    static mut root_screen: *mut xcb_screen_t;
    fn verboselog(fmt: *mut ::core::ffi::c_char, ...);
    fn errorlog(fmt: *mut ::core::ffi::c_char, ...);
    fn debuglog(fmt: *mut ::core::ffi::c_char, ...);
    fn scalloc(num: size_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn srealloc(ptr: *mut ::core::ffi::c_void, size: size_t) -> *mut ::core::ffi::c_void;
    fn sstrdup(str: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn sasprintf(
        strp: *mut *mut ::core::ffi::c_char,
        fmt: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn i3string_from_utf8(from_utf8: *const ::core::ffi::c_char) -> *mut i3String;
    fn i3string_free(str: *mut i3String);
    fn font_is_pango() -> bool;
    fn predict_text_width(text: *mut i3String) -> ::core::ffi::c_int;
    fn logical_px(logical: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn draw_util_surface_init(
        conn_0: *mut xcb_connection_t,
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
    fn draw_util_surface_free(conn_0: *mut xcb_connection_t, surface: *mut surface_t);
    fn draw_util_text(
        text: *mut i3String,
        surface: *mut surface_t,
        fg_color: color_t,
        bg_color: color_t,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
        max_width: ::core::ffi::c_int,
    );
    fn draw_util_image(
        image: *mut cairo_surface_t,
        surface: *mut surface_t,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
    );
    fn draw_util_rectangle(
        surface: *mut surface_t,
        color: color_t,
        x: ::core::ffi::c_double,
        y: ::core::ffi::c_double,
        w: ::core::ffi::c_double,
        h: ::core::ffi::c_double,
    );
    fn draw_util_clear_surface(surface: *mut surface_t, color: color_t);
    fn draw_util_copy_surface(
        src: *mut surface_t,
        dest: *mut surface_t,
        src_x: ::core::ffi::c_double,
        src_y: ::core::ffi::c_double,
        dest_x: ::core::ffi::c_double,
        dest_y: ::core::ffi::c_double,
        width: ::core::ffi::c_double,
        height: ::core::ffi::c_double,
    );
    fn min(a: ::core::ffi::c_int, b: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn max(a: ::core::ffi::c_int, b: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn rect_equals(a: Rect, b: Rect) -> bool;
    static mut focused: *mut Con;
    fn xcb_shape_rectangles(
        c: *mut xcb_connection_t,
        operation: xcb_shape_op_t,
        destination_kind: xcb_shape_kind_t,
        ordering: uint8_t,
        destination_window: xcb_window_t,
        x_offset: int16_t,
        y_offset: int16_t,
        rectangles_len: uint32_t,
        rectangles: *const xcb_rectangle_t,
    ) -> xcb_void_cookie_t;
    fn xcb_shape_mask(
        c: *mut xcb_connection_t,
        operation: xcb_shape_op_t,
        destination_kind: xcb_shape_kind_t,
        destination_window: xcb_window_t,
        x_offset: int16_t,
        y_offset: int16_t,
        source_bitmap: xcb_pixmap_t,
    ) -> xcb_void_cookie_t;
    fn xcb_shape_combine(
        c: *mut xcb_connection_t,
        operation: xcb_shape_op_t,
        destination_kind: xcb_shape_kind_t,
        source_kind: xcb_shape_kind_t,
        destination_window: xcb_window_t,
        x_offset: int16_t,
        y_offset: int16_t,
        source_window: xcb_window_t,
    ) -> xcb_void_cookie_t;
    static mut ewmh_window: xcb_window_t;
    static mut last_timestamp: xcb_timestamp_t;
    static mut root_depth: uint8_t;
    static mut colormap: xcb_colormap_t;
    static mut shape_supported: bool;
    static mut root: xcb_window_t;
    static mut current_configpath: *mut ::core::ffi::c_char;
    static mut config: Config;
    static mut current_socketpath: *mut ::core::ffi::c_char;
    fn ipc_send_window_event(property: *const ::core::ffi::c_char, con: *mut Con);
    static mut shmlogname: *mut ::core::ffi::c_char;
    static mut current_log_stream_socket_path: *mut ::core::ffi::c_char;
    static mut A__NET_WM_STATE_HIDDEN: xcb_atom_t;
    static mut A__NET_WM_STATE: xcb_atom_t;
    static mut A__NET_WM_STATE_MAXIMIZED_HORZ: xcb_atom_t;
    static mut A__NET_WM_STATE_MAXIMIZED_VERT: xcb_atom_t;
    static mut A__NET_FRAME_EXTENTS: xcb_atom_t;
    static mut A_WM_DELETE_WINDOW: xcb_atom_t;
    static mut A_WM_PROTOCOLS: xcb_atom_t;
    static mut A_WM_STATE: xcb_atom_t;
    static mut A_I3_SOCKET_PATH: xcb_atom_t;
    static mut A_I3_CONFIG_PATH: xcb_atom_t;
    static mut A_I3_SHMLOG_PATH: xcb_atom_t;
    static mut A_UTF8_STRING: xcb_atom_t;
    static mut A_I3_PID: xcb_atom_t;
    static mut A_I3_LOG_STREAM_SOCKET_PATH: xcb_atom_t;
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
    fn fake_absolute_configure_notify(con: *mut Con);
    fn send_take_focus(window: xcb_window_t, timestamp: xcb_timestamp_t);
    fn xcb_set_window_rect(conn_0: *mut xcb_connection_t, window: xcb_window_t, r: Rect);
    fn get_visualtype_by_id(visual_id: xcb_visualid_t) -> *mut xcb_visualtype_t;
    fn get_visualid_by_depth(depth: uint16_t) -> xcb_visualid_t;
    fn xcb_add_property_atom(
        conn_0: *mut xcb_connection_t,
        window: xcb_window_t,
        property: xcb_atom_t,
        atom: xcb_atom_t,
    );
    fn xcb_remove_property_atom(
        conn_0: *mut xcb_connection_t,
        window: xcb_window_t,
        property: xcb_atom_t,
        atom: xcb_atom_t,
    );
    fn get_output_containing(x: ::core::ffi::c_uint, y: ::core::ffi::c_uint) -> *mut Output;
    fn con_is_leaf(con: *mut Con) -> bool;
    fn con_has_managed_window(con: *mut Con) -> bool;
    fn con_is_hidden(con: *mut Con) -> bool;
    fn con_is_maximized(con: *mut Con, orientation: orientation_t) -> bool;
    fn con_is_floating(con: *mut Con) -> bool;
    fn con_inside_focused(con: *mut Con) -> bool;
    fn con_descend_focused(con: *mut Con) -> *mut Con;
    fn con_draw_decoration_into_frame(con: *mut Con) -> bool;
    fn con_border_style_rect(con: *mut Con) -> Rect;
    fn con_adjacent_borders(con: *mut Con) -> adjacent_t;
    fn con_border_style(con: *mut Con) -> ::core::ffi::c_int;
    fn con_get_tree_representation(con: *mut Con) -> *mut ::core::ffi::c_char;
    fn con_parse_title_format(con: *mut Con) -> *mut i3String;
    fn ewmh_update_active_window(window: xcb_window_t);
    fn ewmh_update_client_list(list: *mut xcb_window_t, num_windows: ::core::ffi::c_int);
    fn ewmh_update_client_list_stacking(stack: *mut xcb_window_t, num_windows: ::core::ffi::c_int);
    fn ewmh_update_focused(window: xcb_window_t, is_focused: bool);
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __pid_t = ::core::ffi::c_int;
pub type __time_t = ::core::ffi::c_long;
pub type __suseconds_t = ::core::ffi::c_long;
pub type pid_t = __pid_t;
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
pub type xcb_window_enum_t = ::core::ffi::c_uint;
pub const XCB_WINDOW_NONE: xcb_window_enum_t = 0;
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
pub type xcb_stack_mode_t = ::core::ffi::c_uint;
pub const XCB_STACK_MODE_OPPOSITE: xcb_stack_mode_t = 4;
pub const XCB_STACK_MODE_BOTTOM_IF: xcb_stack_mode_t = 3;
pub const XCB_STACK_MODE_TOP_IF: xcb_stack_mode_t = 2;
pub const XCB_STACK_MODE_BELOW: xcb_stack_mode_t = 1;
pub const XCB_STACK_MODE_ABOVE: xcb_stack_mode_t = 0;
pub type xcb_prop_mode_t = ::core::ffi::c_uint;
pub const XCB_PROP_MODE_APPEND: xcb_prop_mode_t = 2;
pub const XCB_PROP_MODE_PREPEND: xcb_prop_mode_t = 1;
pub const XCB_PROP_MODE_REPLACE: xcb_prop_mode_t = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_query_pointer_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_query_pointer_reply_t {
    pub response_type: uint8_t,
    pub same_screen: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub root: xcb_window_t,
    pub child: xcb_window_t,
    pub root_x: int16_t,
    pub root_y: int16_t,
    pub win_x: int16_t,
    pub win_y: int16_t,
    pub mask: uint16_t,
    pub pad0: [uint8_t; 2],
}
pub type xcb_input_focus_t = ::core::ffi::c_uint;
pub const XCB_INPUT_FOCUS_FOLLOW_KEYBOARD: xcb_input_focus_t = 3;
pub const XCB_INPUT_FOCUS_PARENT: xcb_input_focus_t = 2;
pub const XCB_INPUT_FOCUS_POINTER_ROOT: xcb_input_focus_t = 1;
pub const XCB_INPUT_FOCUS_NONE: xcb_input_focus_t = 0;
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
pub type xcb_gc_t = ::core::ffi::c_uint;
pub const XCB_GC_ARC_MODE: xcb_gc_t = 4194304;
pub const XCB_GC_DASH_LIST: xcb_gc_t = 2097152;
pub const XCB_GC_DASH_OFFSET: xcb_gc_t = 1048576;
pub const XCB_GC_CLIP_MASK: xcb_gc_t = 524288;
pub const XCB_GC_CLIP_ORIGIN_Y: xcb_gc_t = 262144;
pub const XCB_GC_CLIP_ORIGIN_X: xcb_gc_t = 131072;
pub const XCB_GC_GRAPHICS_EXPOSURES: xcb_gc_t = 65536;
pub const XCB_GC_SUBWINDOW_MODE: xcb_gc_t = 32768;
pub const XCB_GC_FONT: xcb_gc_t = 16384;
pub const XCB_GC_TILE_STIPPLE_ORIGIN_Y: xcb_gc_t = 8192;
pub const XCB_GC_TILE_STIPPLE_ORIGIN_X: xcb_gc_t = 4096;
pub const XCB_GC_STIPPLE: xcb_gc_t = 2048;
pub const XCB_GC_TILE: xcb_gc_t = 1024;
pub const XCB_GC_FILL_RULE: xcb_gc_t = 512;
pub const XCB_GC_FILL_STYLE: xcb_gc_t = 256;
pub const XCB_GC_JOIN_STYLE: xcb_gc_t = 128;
pub const XCB_GC_CAP_STYLE: xcb_gc_t = 64;
pub const XCB_GC_LINE_STYLE: xcb_gc_t = 32;
pub const XCB_GC_LINE_WIDTH: xcb_gc_t = 16;
pub const XCB_GC_BACKGROUND: xcb_gc_t = 8;
pub const XCB_GC_FOREGROUND: xcb_gc_t = 4;
pub const XCB_GC_PLANE_MASK: xcb_gc_t = 2;
pub const XCB_GC_FUNCTION: xcb_gc_t = 1;
pub type xcb_clip_ordering_t = ::core::ffi::c_uint;
pub const XCB_CLIP_ORDERING_YX_BANDED: xcb_clip_ordering_t = 3;
pub const XCB_CLIP_ORDERING_YX_SORTED: xcb_clip_ordering_t = 2;
pub const XCB_CLIP_ORDERING_Y_SORTED: xcb_clip_ordering_t = 1;
pub const XCB_CLIP_ORDERING_UNSORTED: xcb_clip_ordering_t = 0;
pub type xcb_colormap_alloc_t = ::core::ffi::c_uint;
pub const XCB_COLORMAP_ALLOC_ALL: xcb_colormap_alloc_t = 1;
pub const XCB_COLORMAP_ALLOC_NONE: xcb_colormap_alloc_t = 0;
pub type xcb_pixmap_enum_t = ::core::ffi::c_uint;
pub const XCB_PIXMAP_NONE: xcb_pixmap_enum_t = 0;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const XCB_ICCCM_WM_STATE_ICONIC: C2RustUnnamed = 3;
pub const XCB_ICCCM_WM_STATE_NORMAL: C2RustUnnamed = 1;
pub const XCB_ICCCM_WM_STATE_WITHDRAWN: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_icccm_get_wm_protocols_reply_t {
    pub atoms_len: uint32_t,
    pub atoms: *mut xcb_atom_t,
    pub _reply: *mut xcb_get_property_reply_t,
}
pub type PangoFontDescription = _PangoFontDescription;
pub type cairo_t = _cairo;
pub type cairo_surface_t = _cairo_surface;
pub type i3String = _i3String;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Font {
    pub type_0: C2RustUnnamed_2,
    pub height: ::core::ffi::c_int,
    pub pattern: *mut ::core::ffi::c_char,
    pub specific: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub xcb: C2RustUnnamed_1,
    pub pango_desc: *mut PangoFontDescription,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub id: xcb_font_t,
    pub info: *mut xcb_query_font_reply_t,
    pub table: *mut xcb_charinfo_t,
}
pub type C2RustUnnamed_2 = ::core::ffi::c_uint;
pub const FONT_TYPE_PANGO: C2RustUnnamed_2 = 2;
pub const FONT_TYPE_XCB: C2RustUnnamed_2 = 1;
pub const FONT_TYPE_NONE: C2RustUnnamed_2 = 0;
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
pub type xcb_randr_output_t = uint32_t;
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
pub struct xoutput {
    pub id: xcb_randr_output_t,
    pub active: bool,
    pub changed: bool,
    pub to_be_disabled: bool,
    pub primary: bool,
    pub names_head: names_head,
    pub con: *mut Con,
    pub rect: Rect,
    pub outputs: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub tqe_next: *mut xoutput,
    pub tqe_prev: *mut *mut xoutput,
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
    pub type_0: C2RustUnnamed_20,
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
    pub floating: C2RustUnnamed_9,
    pub nodes: C2RustUnnamed_8,
    pub focused: C2RustUnnamed_7,
    pub all_cons: C2RustUnnamed_6,
    pub floating_windows: C2RustUnnamed_5,
    pub on_remove_child: Option<unsafe extern "C" fn(*mut Con) -> ()>,
    pub scratchpad_state: C2RustUnnamed_4,
    pub old_id: ::core::ffi::c_int,
    pub depth: uint16_t,
    pub colormap: xcb_colormap_t,
}
pub type C2RustUnnamed_4 = ::core::ffi::c_uint;
pub const SCRATCHPAD_CHANGED: C2RustUnnamed_4 = 2;
pub const SCRATCHPAD_FRESH: C2RustUnnamed_4 = 1;
pub const SCRATCHPAD_NONE: C2RustUnnamed_4 = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub tqe_next: *mut Con,
    pub tqe_prev: *mut *mut Con,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub tqe_next: *mut Con,
    pub tqe_prev: *mut *mut Con,
}
pub type C2RustUnnamed_9 = ::core::ffi::c_uint;
pub const FLOATING_USER_ON: C2RustUnnamed_9 = 3;
pub const FLOATING_AUTO_ON: C2RustUnnamed_9 = 2;
pub const FLOATING_USER_OFF: C2RustUnnamed_9 = 1;
pub const FLOATING_AUTO_OFF: C2RustUnnamed_9 = 0;
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
    pub urgent: C2RustUnnamed_14,
    pub dock: C2RustUnnamed_13,
    pub id: xcb_window_t,
    pub window_mode: C2RustUnnamed_12,
    pub con_id: *mut Con,
    pub match_all_windows: bool,
    pub insert_where: C2RustUnnamed_11,
    pub matches: C2RustUnnamed_10,
    pub restart_mode: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub tqe_next: *mut Match,
    pub tqe_prev: *mut *mut Match,
}
pub type C2RustUnnamed_11 = ::core::ffi::c_uint;
pub const M_BELOW: C2RustUnnamed_11 = 2;
pub const M_ASSIGN_WS: C2RustUnnamed_11 = 1;
pub const M_HERE: C2RustUnnamed_11 = 0;
pub type C2RustUnnamed_12 = ::core::ffi::c_uint;
pub const WM_FLOATING: C2RustUnnamed_12 = 6;
pub const WM_FLOATING_USER: C2RustUnnamed_12 = 5;
pub const WM_FLOATING_AUTO: C2RustUnnamed_12 = 4;
pub const WM_TILING: C2RustUnnamed_12 = 3;
pub const WM_TILING_USER: C2RustUnnamed_12 = 2;
pub const WM_TILING_AUTO: C2RustUnnamed_12 = 1;
pub const WM_ANY: C2RustUnnamed_12 = 0;
pub type C2RustUnnamed_13 = ::core::ffi::c_int;
pub const M_DOCK_BOTTOM: C2RustUnnamed_13 = 3;
pub const M_DOCK_TOP: C2RustUnnamed_13 = 2;
pub const M_DOCK_ANY: C2RustUnnamed_13 = 1;
pub const M_NODOCK: C2RustUnnamed_13 = 0;
pub const M_DONTCHECK: C2RustUnnamed_13 = -1;
pub type C2RustUnnamed_14 = ::core::ffi::c_int;
pub const U_OLDEST: C2RustUnnamed_14 = 1;
pub const U_LATEST: C2RustUnnamed_14 = 0;
pub const U_DONTCHECK: C2RustUnnamed_14 = -1;
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
    pub dock: C2RustUnnamed_15,
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
pub type C2RustUnnamed_15 = ::core::ffi::c_uint;
pub const W_DOCK_BOTTOM: C2RustUnnamed_15 = 2;
pub const W_DOCK_TOP: C2RustUnnamed_15 = 1;
pub const W_NODOCK: C2RustUnnamed_15 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Assignment {
    pub type_0: C2RustUnnamed_18,
    pub match_0: Match,
    pub dest: C2RustUnnamed_17,
    pub assignments: C2RustUnnamed_16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_16 {
    pub tqe_next: *mut Assignment,
    pub tqe_prev: *mut *mut Assignment,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_17 {
    pub command: *mut ::core::ffi::c_char,
    pub workspace: *mut ::core::ffi::c_char,
    pub output: *mut ::core::ffi::c_char,
}
pub type C2RustUnnamed_18 = ::core::ffi::c_uint;
pub const A_TO_OUTPUT: C2RustUnnamed_18 = 16;
pub const A_TO_WORKSPACE_NUMBER: C2RustUnnamed_18 = 8;
pub const A_NO_FOCUS: C2RustUnnamed_18 = 4;
pub const A_TO_WORKSPACE: C2RustUnnamed_18 = 2;
pub const A_COMMAND: C2RustUnnamed_18 = 1;
pub const A_ANY: C2RustUnnamed_18 = 0;
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
    pub marks: C2RustUnnamed_19,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_19 {
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
pub type C2RustUnnamed_20 = ::core::ffi::c_uint;
pub const CT_DOCKAREA: C2RustUnnamed_20 = 5;
pub const CT_WORKSPACE: C2RustUnnamed_20 = 4;
pub const CT_FLOATING_CON: C2RustUnnamed_20 = 3;
pub const CT_CON: C2RustUnnamed_20 = 2;
pub const CT_OUTPUT: C2RustUnnamed_20 = 1;
pub const CT_ROOT: C2RustUnnamed_20 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct names_head {
    pub slh_first: *mut output_name,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct output_name {
    pub name: *mut ::core::ffi::c_char,
    pub names: C2RustUnnamed_21,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_21 {
    pub sle_next: *mut output_name,
}
pub type Output = xoutput;
pub type orientation_t = ::core::ffi::c_uint;
pub const VERT: orientation_t = 2;
pub const HORIZ: orientation_t = 1;
pub const NO_ORIENTATION: orientation_t = 0;
pub type kill_window_t = ::core::ffi::c_uint;
pub const KILL_CLIENT: kill_window_t = 2;
pub const KILL_WINDOW: kill_window_t = 1;
pub const DONT_KILL_WINDOW: kill_window_t = 0;
pub type adjacent_t = ::core::ffi::c_uint;
pub const ADJ_LOWER_SCREEN_EDGE: adjacent_t = 16;
pub const ADJ_UPPER_SCREEN_EDGE: adjacent_t = 4;
pub const ADJ_RIGHT_SCREEN_EDGE: adjacent_t = 2;
pub const ADJ_LEFT_SCREEN_EDGE: adjacent_t = 1;
pub const ADJ_NONE: adjacent_t = 0;
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
pub type xcb_shape_op_t = uint8_t;
pub type xcb_shape_kind_t = uint8_t;
pub type xcb_shape_so_t = ::core::ffi::c_uint;
pub const XCB_SHAPE_SO_INVERT: xcb_shape_so_t = 4;
pub const XCB_SHAPE_SO_SUBTRACT: xcb_shape_so_t = 3;
pub const XCB_SHAPE_SO_INTERSECT: xcb_shape_so_t = 2;
pub const XCB_SHAPE_SO_UNION: xcb_shape_so_t = 1;
pub const XCB_SHAPE_SO_SET: xcb_shape_so_t = 0;
pub type xcb_shape_sk_t = ::core::ffi::c_uint;
pub const XCB_SHAPE_SK_INPUT: xcb_shape_sk_t = 2;
pub const XCB_SHAPE_SK_CLIP: xcb_shape_sk_t = 1;
pub const XCB_SHAPE_SK_BOUNDING: xcb_shape_sk_t = 0;
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
    pub focus_on_window_activation: C2RustUnnamed_24,
    pub show_marks: bool,
    pub title_align: C2RustUnnamed_23,
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
    pub popup_during_fullscreen: C2RustUnnamed_22,
    pub number_barconfigs: ::core::ffi::c_int,
    pub tiling_drag: tiling_drag_t,
    pub gaps: gaps_t,
    pub smart_gaps: smart_gaps_t,
}
pub type C2RustUnnamed_22 = ::core::ffi::c_uint;
pub const PDF_ALL: C2RustUnnamed_22 = 3;
pub const PDF_IGNORE: C2RustUnnamed_22 = 2;
pub const PDF_LEAVE_FULLSCREEN: C2RustUnnamed_22 = 1;
pub const PDF_SMART: C2RustUnnamed_22 = 0;
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
pub type C2RustUnnamed_23 = ::core::ffi::c_uint;
pub const ALIGN_RIGHT: C2RustUnnamed_23 = 2;
pub const ALIGN_CENTER: C2RustUnnamed_23 = 1;
pub const ALIGN_LEFT: C2RustUnnamed_23 = 0;
pub type C2RustUnnamed_24 = ::core::ffi::c_uint;
pub const FOWA_NONE: C2RustUnnamed_24 = 3;
pub const FOWA_FOCUS: C2RustUnnamed_24 = 2;
pub const FOWA_URGENT: C2RustUnnamed_24 = 1;
pub const FOWA_SMART: C2RustUnnamed_24 = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_27 {
    pub cqe_next: *mut con_state,
    pub cqe_prev: *mut con_state,
}
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
pub const XCB_CLIENT_MESSAGE: ::core::ffi::c_int = 33 as ::core::ffi::c_int;
pub const XCB_NONE: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const XCB_CURRENT_TIME: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[no_mangle]
pub static mut focused_id: xcb_window_t = XCB_NONE as xcb_window_t;
static mut last_focused: xcb_window_t = XCB_NONE as xcb_window_t;
static mut warp_to: *mut Rect = ::core::ptr::null::<Rect>() as *mut Rect;
#[no_mangle]
pub static mut state_head: state_head = unsafe {
    state_head {
        cqh_first: &raw const state_head as *mut state_head as *mut ::core::ffi::c_void
            as *mut con_state,
        cqh_last: &raw const state_head as *mut state_head as *mut ::core::ffi::c_void
            as *mut con_state,
    }
};
#[no_mangle]
pub static mut old_state_head: old_state_head = unsafe {
    old_state_head {
        cqh_first: &raw const old_state_head as *mut old_state_head as *mut ::core::ffi::c_void
            as *mut con_state,
        cqh_last: &raw const old_state_head as *mut old_state_head as *mut ::core::ffi::c_void
            as *mut con_state,
    }
};
#[no_mangle]
pub static mut initial_mapping_head: initial_mapping_head = initial_mapping_head {
    tqh_first: ::core::ptr::null::<con_state>() as *mut con_state,
    tqh_last: ::core::ptr::null::<*mut con_state>() as *mut *mut con_state,
};
unsafe extern "C" fn state_for_frame(mut window: xcb_window_t) -> *mut con_state {
    let mut state: *mut con_state = ::core::ptr::null_mut::<con_state>();
    state = state_head.cqh_first as *mut con_state;
    while state != &raw mut state_head as *mut ::core::ffi::c_void as *mut con_state {
        if (*state).id == window {
            return state;
        }
        state = (*state).state.cqe_next as *mut con_state;
    }
    errorlog(
        b"ERROR: No state found for window 0x%08x\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        window,
    );
    if 0 as ::core::ffi::c_int != 0 {
    } else {
        __assert_fail(
            b"false\0" as *const u8 as *const ::core::ffi::c_char,
            b"../src/x.c\0" as *const u8 as *const ::core::ffi::c_char,
            99 as ::core::ffi::c_uint,
            b"con_state *state_for_frame(xcb_window_t)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    };
    return ::core::ptr::null_mut::<con_state>();
}
unsafe extern "C" fn change_ewmh_focus(mut new_focus: xcb_window_t, mut old_focus: xcb_window_t) {
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
#[no_mangle]
pub unsafe extern "C" fn x_con_init(mut con: *mut Con) {
    let mut mask: uint32_t = 0 as uint32_t;
    let mut values: [uint32_t; 5] = [0; 5];
    let mut visual: xcb_visualid_t = get_visualid_by_depth((*con).depth);
    let mut win_colormap: xcb_colormap_t = 0;
    if (*con).depth as ::core::ffi::c_int != root_depth as ::core::ffi::c_int {
        win_colormap = xcb_generate_id(conn) as xcb_colormap_t;
        xcb_create_colormap(
            conn,
            XCB_COLORMAP_ALLOC_NONE as ::core::ffi::c_int as uint8_t,
            win_colormap,
            root,
            visual,
        );
        (*con).colormap = win_colormap;
    } else {
        win_colormap = colormap;
        (*con).colormap = XCB_NONE as xcb_colormap_t;
    }
    mask |= XCB_CW_BACK_PIXEL as ::core::ffi::c_int as uint32_t;
    values[0 as ::core::ffi::c_int as usize] = (*root_screen).black_pixel;
    mask |= XCB_CW_BORDER_PIXEL as ::core::ffi::c_int as uint32_t;
    values[1 as ::core::ffi::c_int as usize] = (*root_screen).black_pixel;
    mask |= XCB_CW_OVERRIDE_REDIRECT as ::core::ffi::c_int as uint32_t;
    values[2 as ::core::ffi::c_int as usize] = 1 as uint32_t;
    mask |= XCB_CW_EVENT_MASK as ::core::ffi::c_int as uint32_t;
    values[3 as ::core::ffi::c_int as usize] = ((XCB_EVENT_MASK_BUTTON_PRESS as ::core::ffi::c_int
        | XCB_EVENT_MASK_BUTTON_RELEASE as ::core::ffi::c_int
        | XCB_EVENT_MASK_POINTER_MOTION as ::core::ffi::c_int
        | XCB_EVENT_MASK_EXPOSURE as ::core::ffi::c_int
        | XCB_EVENT_MASK_STRUCTURE_NOTIFY as ::core::ffi::c_int
        | XCB_EVENT_MASK_SUBSTRUCTURE_REDIRECT as ::core::ffi::c_int
        | XCB_EVENT_MASK_SUBSTRUCTURE_NOTIFY as ::core::ffi::c_int
        | XCB_EVENT_MASK_ENTER_WINDOW as ::core::ffi::c_int)
        & !(XCB_EVENT_MASK_ENTER_WINDOW as ::core::ffi::c_int))
        as uint32_t;
    mask |= XCB_CW_COLORMAP as ::core::ffi::c_int as uint32_t;
    values[4 as ::core::ffi::c_int as usize] = win_colormap as uint32_t;
    let mut dims: Rect = Rect {
        x: -(15 as ::core::ffi::c_int) as uint32_t,
        y: -(15 as ::core::ffi::c_int) as uint32_t,
        width: 10 as uint32_t,
        height: 10 as uint32_t,
    };
    let mut frame_id: xcb_window_t = create_window(
        conn,
        dims,
        (*con).depth,
        visual,
        XCB_WINDOW_CLASS_INPUT_OUTPUT as ::core::ffi::c_int as uint16_t,
        XCURSOR_CURSOR_POINTER,
        false_0 != 0,
        mask,
        &raw mut values as *mut uint32_t,
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
        XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as uint8_t,
        (*con).frame.id as xcb_window_t,
        XCB_ATOM_WM_CLASS as ::core::ffi::c_int as xcb_atom_t,
        XCB_ATOM_STRING as ::core::ffi::c_int as xcb_atom_t,
        8 as uint8_t,
        strlen(b"i3-frame\0" as *const u8 as *const ::core::ffi::c_char)
            .wrapping_add(1 as size_t)
            .wrapping_mul(2 as size_t) as uint32_t,
        b"i3-frame\0i3-frame\0\0" as *const u8 as *const ::core::ffi::c_char
            as *const ::core::ffi::c_void,
    );
    let mut state: *mut con_state =
        scalloc(1 as size_t, ::core::mem::size_of::<con_state>() as size_t) as *mut con_state;
    (*state).id = (*con).frame.id as xcb_window_t;
    (*state).mapped = false_0 != 0;
    (*state).initial = true_0 != 0;
    debuglog(
        b"%s:%s:%d - Adding window 0x%08x to lists\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/x.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"x_con_init\0" as *const u8 as *const ::core::ffi::c_char,
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
        b"%s:%s:%d - adding new state for window id 0x%08x\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/x.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"x_con_init\0" as *const u8 as *const ::core::ffi::c_char,
        190 as ::core::ffi::c_int,
        (*state).id,
    );
}
#[no_mangle]
pub unsafe extern "C" fn x_reinit(mut con: *mut Con) {
    let mut state: *mut con_state = ::core::ptr::null_mut::<con_state>();
    state = state_for_frame((*con).frame.id as xcb_window_t);
    if state.is_null() {
        errorlog(
            b"ERROR: window state not found\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        return;
    }
    debuglog(
        b"%s:%s:%d - resetting state %p to initial\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/x.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"x_reinit\0" as *const u8 as *const ::core::ffi::c_char,
        207 as ::core::ffi::c_int,
        state,
    );
    (*state).initial = true_0 != 0;
    (*state).child_mapped = false_0 != 0;
    (*state).con = con;
    memset(
        &raw mut (*state).window_rect as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<Rect>() as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn x_reparent_child(mut con: *mut Con, mut old: *mut Con) {
    let mut state: *mut con_state = ::core::ptr::null_mut::<con_state>();
    state = state_for_frame((*con).frame.id as xcb_window_t);
    if state.is_null() {
        errorlog(
            b"ERROR: window state for con not found\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        return;
    }
    (*state).need_reparent = true_0 != 0;
    (*state).old_frame = (*old).frame.id as xcb_window_t;
}
#[no_mangle]
pub unsafe extern "C" fn x_move_win(mut src: *mut Con, mut dest: *mut Con) {
    let mut state_src: *mut con_state = ::core::ptr::null_mut::<con_state>();
    let mut state_dest: *mut con_state = ::core::ptr::null_mut::<con_state>();
    state_src = state_for_frame((*src).frame.id as xcb_window_t);
    if state_src.is_null() {
        errorlog(
            b"ERROR: window state for src not found\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        return;
    }
    state_dest = state_for_frame((*dest).frame.id as xcb_window_t);
    if state_dest.is_null() {
        errorlog(
            b"ERROR: window state for dest not found\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        return;
    }
    (*state_dest).con = (*state_src).con;
    (*state_src).con = ::core::ptr::null_mut::<Con>();
    if rect_equals(
        (*state_dest).window_rect,
        Rect {
            x: 0 as uint32_t,
            y: 0 as uint32_t,
            width: 0 as uint32_t,
            height: 0 as uint32_t,
        },
    ) {
        memcpy(
            &raw mut (*state_dest).window_rect as *mut ::core::ffi::c_void,
            &raw mut (*state_src).window_rect as *const ::core::ffi::c_void,
            ::core::mem::size_of::<Rect>() as size_t,
        );
        debuglog(
            b"%s:%s:%d - COPYING RECT\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b"../src/x.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"x_move_win\0" as *const u8 as *const ::core::ffi::c_char,
            252 as ::core::ffi::c_int,
        );
    }
}
unsafe extern "C" fn _x_con_kill(mut con: *mut Con) {
    if (*con).colormap as ::core::ffi::c_long != XCB_NONE {
        xcb_free_colormap(conn, (*con).colormap);
    }
    draw_util_surface_free(conn, &raw mut (*con).frame);
    draw_util_surface_free(conn, &raw mut (*con).frame_buffer);
    xcb_free_pixmap(conn, (*con).frame_buffer.id as xcb_pixmap_t);
    (*con).frame_buffer.id = XCB_NONE as xcb_drawable_t;
    let mut state: *mut con_state = state_for_frame((*con).frame.id as xcb_window_t);
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
#[no_mangle]
pub unsafe extern "C" fn x_con_kill(mut con: *mut Con) {
    _x_con_kill(con);
    xcb_destroy_window(conn, (*con).frame.id as xcb_window_t);
}
#[no_mangle]
pub unsafe extern "C" fn x_con_reframe(mut con: *mut Con) {
    _x_con_kill(con);
    x_con_init(con);
}
#[no_mangle]
pub unsafe extern "C" fn window_supports_protocol(
    mut window: xcb_window_t,
    mut atom: xcb_atom_t,
) -> bool {
    let mut protocols: xcb_icccm_get_wm_protocols_reply_t = xcb_icccm_get_wm_protocols_reply_t {
        atoms_len: 0,
        atoms: ::core::ptr::null_mut::<xcb_atom_t>(),
        _reply: ::core::ptr::null_mut::<xcb_get_property_reply_t>(),
    };
    let mut result: bool = false_0 != 0;
    let mut cookie: xcb_get_property_cookie_t =
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
    let mut i: uint32_t = 0 as uint32_t;
    while i < protocols.atoms_len {
        if *protocols.atoms.offset(i as isize) == atom {
            result = true_0 != 0;
        }
        i = i.wrapping_add(1);
    }
    xcb_icccm_get_wm_protocols_reply_wipe(&raw mut protocols);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn x_window_kill(mut window: xcb_window_t, mut kill_window: kill_window_t) {
    if !window_supports_protocol(window, A_WM_DELETE_WINDOW) {
        if kill_window as ::core::ffi::c_uint
            == KILL_WINDOW as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            verboselog(
                b"Killing specific window 0x%08x\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                window,
            );
            xcb_destroy_window(conn, window);
        } else {
            verboselog(
                b"Killing the X11 client which owns window 0x%08x\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                window,
            );
            xcb_kill_client(conn, window as uint32_t);
        }
        return;
    }
    let mut event: *mut ::core::ffi::c_void = scalloc(32 as size_t, 1 as size_t);
    let mut ev: *mut xcb_client_message_event_t = event as *mut xcb_client_message_event_t;
    (*ev).response_type = XCB_CLIENT_MESSAGE as uint8_t;
    (*ev).window = window;
    (*ev).type_0 = A_WM_PROTOCOLS;
    (*ev).format = 32 as uint8_t;
    (*ev).data.data32[0 as ::core::ffi::c_int as usize] = A_WM_DELETE_WINDOW as uint32_t;
    (*ev).data.data32[1 as ::core::ffi::c_int as usize] = XCB_CURRENT_TIME as uint32_t;
    verboselog(
        b"Sending WM_DELETE to the client\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    );
    xcb_send_event(
        conn,
        false_0 as uint8_t,
        window,
        XCB_EVENT_MASK_NO_EVENT as ::core::ffi::c_int as uint32_t,
        ev as *mut ::core::ffi::c_char,
    );
    xcb_flush(conn);
    free(event);
}
unsafe extern "C" fn x_draw_title_border(
    mut con: *mut Con,
    mut p: *mut deco_render_params,
    mut dest_surface: *mut surface_t,
) {
    let mut dr: *mut Rect = &raw mut (*con).deco_rect;
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
        (*dr)
            .x
            .wrapping_add((*dr).width)
            .wrapping_sub(1 as uint32_t) as ::core::ffi::c_double,
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
        (*dr)
            .y
            .wrapping_add((*dr).height)
            .wrapping_sub(1 as uint32_t) as ::core::ffi::c_double,
        (*dr).width as ::core::ffi::c_double,
        1 as ::core::ffi::c_int as ::core::ffi::c_double,
    );
}
unsafe extern "C" fn x_draw_decoration_after_title(
    mut con: *mut Con,
    mut p: *mut deco_render_params,
    mut dest_surface: *mut surface_t,
) {
    if (*con).parent != ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut Con {
    } else {
        __assert_fail(
            b"con->parent != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            b"../src/x.c\0" as *const u8 as *const ::core::ffi::c_char,
            381 as ::core::ffi::c_uint,
            b"void x_draw_decoration_after_title(Con *, struct deco_render_params *, surface_t *)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
    };
    let mut dr: *mut Rect = &raw mut (*con).deco_rect;
    if !font_is_pango() {
        draw_util_rectangle(
            dest_surface,
            (*(*p).color).background,
            (*dr).x.wrapping_add((*dr).width).wrapping_sub(
                (2 as ::core::ffi::c_int * logical_px(1 as ::core::ffi::c_int)) as uint32_t,
            ) as ::core::ffi::c_double,
            (*dr).y as ::core::ffi::c_double,
            (2 as ::core::ffi::c_int * logical_px(1 as ::core::ffi::c_int))
                as ::core::ffi::c_double,
            (*dr).height as ::core::ffi::c_double,
        );
    }
    x_draw_title_border(con, p, dest_surface);
}
unsafe extern "C" fn x_get_border_rectangles(
    mut con: *mut Con,
    mut rectangles: *mut xcb_rectangle_t,
) -> size_t {
    let mut count: size_t = 0 as size_t;
    let mut border_style: ::core::ffi::c_int = con_border_style(con);
    if border_style != BS_NONE as ::core::ffi::c_int && con_is_leaf(con) as ::core::ffi::c_int != 0
    {
        let mut borders_to_hide: adjacent_t = (con_adjacent_borders(con) as ::core::ffi::c_uint
            & config.hide_edge_borders as ::core::ffi::c_uint)
            as adjacent_t;
        let mut br: Rect = con_border_style_rect(con);
        if borders_to_hide as ::core::ffi::c_uint
            & ADJ_LEFT_SCREEN_EDGE as ::core::ffi::c_int as ::core::ffi::c_uint
            == 0
        {
            let fresh0 = count;
            count = count.wrapping_add(1);
            *rectangles.offset(fresh0 as isize) = xcb_rectangle_t {
                x: 0 as int16_t,
                y: 0 as int16_t,
                width: br.x as uint16_t,
                height: (*con).rect.height as uint16_t,
            };
        }
        if borders_to_hide as ::core::ffi::c_uint
            & ADJ_RIGHT_SCREEN_EDGE as ::core::ffi::c_int as ::core::ffi::c_uint
            == 0
        {
            let fresh1 = count;
            count = count.wrapping_add(1);
            *rectangles.offset(fresh1 as isize) = xcb_rectangle_t {
                x: (*con).rect.width.wrapping_add(br.width.wrapping_add(br.x)) as int16_t,
                y: 0 as int16_t,
                width: br.width.wrapping_add(br.x).wrapping_neg() as uint16_t,
                height: (*con).rect.height as uint16_t,
            };
        }
        if borders_to_hide as ::core::ffi::c_uint
            & ADJ_LOWER_SCREEN_EDGE as ::core::ffi::c_int as ::core::ffi::c_uint
            == 0
        {
            let fresh2 = count;
            count = count.wrapping_add(1);
            *rectangles.offset(fresh2 as isize) = xcb_rectangle_t {
                x: br.x as int16_t,
                y: (*con)
                    .rect
                    .height
                    .wrapping_add(br.height.wrapping_add(br.y)) as int16_t,
                width: (*con).rect.width.wrapping_add(br.width) as uint16_t,
                height: br.height.wrapping_add(br.y).wrapping_neg() as uint16_t,
            };
        }
        if border_style == BS_PIXEL as ::core::ffi::c_int
            && borders_to_hide as ::core::ffi::c_uint
                & ADJ_UPPER_SCREEN_EDGE as ::core::ffi::c_int as ::core::ffi::c_uint
                == 0
        {
            let fresh3 = count;
            count = count.wrapping_add(1);
            *rectangles.offset(fresh3 as isize) = xcb_rectangle_t {
                x: br.x as int16_t,
                y: 0 as int16_t,
                width: (*con).rect.width.wrapping_add(br.width) as uint16_t,
                height: br.y as uint16_t,
            };
        }
    }
    return count;
}
#[no_mangle]
pub unsafe extern "C" fn x_draw_decoration(mut con: *mut Con) {
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
    let mut parent: *mut Con = (*con).parent as *mut Con;
    let mut leaf: bool = con_is_leaf(con);
    if !leaf
        && (*parent).layout as ::core::ffi::c_uint
            != L_STACKED as ::core::ffi::c_int as ::core::ffi::c_uint
        && (*parent).layout as ::core::ffi::c_uint
            != L_TABBED as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*parent).type_0 as ::core::ffi::c_uint
            == CT_OUTPUT as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*parent).type_0 as ::core::ffi::c_uint
            == CT_DOCKAREA as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*con).type_0 as ::core::ffi::c_uint
            == CT_FLOATING_CON as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return;
    }
    if (*con).rect.height == 0 as uint32_t {
        return;
    }
    if leaf as ::core::ffi::c_int != 0 && (*con).frame_buffer.id as ::core::ffi::c_long == XCB_NONE
    {
        return;
    }
    let mut p: *mut deco_render_params = scalloc(
        1 as size_t,
        ::core::mem::size_of::<deco_render_params>() as size_t,
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
    let mut r: *mut Rect = &raw mut (*con).rect;
    let mut w: *mut Rect = &raw mut (*con).window_rect;
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
            ::core::mem::size_of::<deco_render_params>() as size_t,
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
        if (*p).border_style != BS_NONE as ::core::ffi::c_int
            && (*p).con_is_leaf as ::core::ffi::c_int != 0
        {
            let mut rectangles: [xcb_rectangle_t; 4] = [xcb_rectangle_t {
                x: 0,
                y: 0,
                width: 0,
                height: 0,
            }; 4];
            let mut rectangles_count: size_t =
                x_get_border_rectangles(con, &raw mut rectangles as *mut xcb_rectangle_t);
            let mut i: size_t = 0 as size_t;
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
            let mut br: Rect = con_border_style_rect(con);
            if (*con).nodes.tqe_next.is_null()
                && (*(*((*con).nodes.tqe_prev as *mut nodes_head)).tqh_last).is_null()
                && (*(*con).parent).type_0 as ::core::ffi::c_uint
                    != CT_FLOATING_CON as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                if (*p).parent_layout as ::core::ffi::c_uint
                    == L_SPLITH as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    draw_util_rectangle(
                        &raw mut (*con).frame_buffer,
                        (*(*p).color).indicator,
                        (*r).width.wrapping_add(br.width.wrapping_add(br.x))
                            as ::core::ffi::c_double,
                        br.y as ::core::ffi::c_double,
                        br.width.wrapping_add(br.x).wrapping_neg() as ::core::ffi::c_double,
                        (*r).height.wrapping_add(br.height) as ::core::ffi::c_double,
                    );
                } else if (*p).parent_layout as ::core::ffi::c_uint
                    == L_SPLITV as ::core::ffi::c_int as ::core::ffi::c_uint
                {
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
                b"%s:%s:%d - using con->frame_buffer (for con->name=%s) as dest_surface\n\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"../src/x.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"x_draw_decoration\0" as *const u8 as *const ::core::ffi::c_char,
                602 as ::core::ffi::c_int,
                (*con).name,
            );
            dest_surface = &raw mut (*con).frame_buffer;
        } else {
            debuglog(
                b"%s:%s:%d - sticking to parent->frame_buffer = %p\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"../src/x.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"x_draw_decoration\0" as *const u8 as *const ::core::ffi::c_char,
                605 as ::core::ffi::c_int,
                dest_surface,
            );
        }
        debuglog(
            b"%s:%s:%d - dest_surface %p is %d x %d (id=0x%08x)\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/x.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"x_draw_decoration\0" as *const u8 as *const ::core::ffi::c_char,
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
            if !((*p).border_style != BS_NORMAL as ::core::ffi::c_int
                || (*con).deco_rect.width == 0 as uint32_t
                || (*con).deco_rect.height == 0 as uint32_t)
            {
                debuglog(
                    b"%s:%s:%d - con->deco_rect = (x=%d, y=%d, w=%d, h=%d) for con->name=%s\n\0"
                        as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    b"../src/x.c\0" as *const u8 as *const ::core::ffi::c_char,
                    b"x_draw_decoration\0" as *const u8 as *const ::core::ffi::c_char,
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
                    .wrapping_sub(config.font.height as uint32_t)
                    .wrapping_div(2 as uint32_t)
                    as ::core::ffi::c_int;
                win = (*con).window;
                deco_width = (*con).deco_rect.width as ::core::ffi::c_int;
                title_padding = logical_px(2 as ::core::ffi::c_int) as ::core::ffi::c_int;
                mark_width = 0 as ::core::ffi::c_int;
                if config.show_marks as ::core::ffi::c_int != 0
                    && !(*con).marks_head.tqh_first.is_null()
                {
                    let mut formatted_mark: *mut ::core::ffi::c_char =
                        sstrdup(b"\0" as *const u8 as *const ::core::ffi::c_char);
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
                                b"%s[%s]\0" as *const u8 as *const ::core::ffi::c_char,
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
                        let mut mark_offset_x: ::core::ffi::c_int = if config.title_align
                            as ::core::ffi::c_uint
                            == ALIGN_RIGHT as ::core::ffi::c_int as ::core::ffi::c_uint
                        {
                            title_padding
                        } else {
                            deco_width - mark_width - title_padding
                        };
                        draw_util_text(
                            mark_0,
                            dest_surface,
                            (*(*p).color).text,
                            (*(*p).color).background,
                            (*con).deco_rect.x.wrapping_add(mark_offset_x as uint32_t)
                                as ::core::ffi::c_int,
                            (*con).deco_rect.y.wrapping_add(text_offset_y as uint32_t)
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
                        let mut tree: *mut ::core::ffi::c_char = con_get_tree_representation(con);
                        sasprintf(
                            &raw mut _title,
                            b"i3: %s\0" as *const u8 as *const ::core::ffi::c_char,
                            tree,
                        );
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
                            .wrapping_sub(logical_px(2 as ::core::ffi::c_int) as uint32_t)
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
                    match config.title_align as ::core::ffi::c_uint {
                        0 => {
                            icon_offset_x = icon_padding;
                            title_offset_x = title_padding + total_icon_space;
                        }
                        1 => {
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
                        2 => {
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
                        _ => {
                            errorlog(
                                b"ERROR: BUG: invalid config.title_align value %d\n\0" as *const u8
                                    as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                                config.title_align as ::core::ffi::c_uint,
                            );
                            return;
                        }
                    }
                    draw_util_text(
                        title,
                        dest_surface,
                        (*(*p).color).text,
                        (*(*p).color).background,
                        (*con).deco_rect.x.wrapping_add(title_offset_x as uint32_t)
                            as ::core::ffi::c_int,
                        (*con).deco_rect.y.wrapping_add(text_offset_y as uint32_t)
                            as ::core::ffi::c_int,
                        deco_width
                            - mark_width
                            - 2 as ::core::ffi::c_int * title_padding
                            - total_icon_space,
                    );
                    if has_icon {
                        draw_util_image(
                            (*win).icon,
                            dest_surface,
                            (*con).deco_rect.x.wrapping_add(icon_offset_x as uint32_t)
                                as ::core::ffi::c_int,
                            (*con)
                                .deco_rect
                                .y
                                .wrapping_add(logical_px(1 as ::core::ffi::c_int) as uint32_t)
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
#[no_mangle]
pub unsafe extern "C" fn x_deco_recurse(mut con: *mut Con) {
    let mut leaf: bool =
        (*con).nodes_head.tqh_first.is_null() && (*con).floating_head.tqh_first.is_null();
    let mut state: *const con_state = state_for_frame((*con).frame.id as xcb_window_t);
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
unsafe extern "C" fn set_hidden_state(mut con: *mut Con) {
    if (*con).window.is_null() {
        return;
    }
    let mut state: *mut con_state = state_for_frame((*con).frame.id as xcb_window_t);
    let mut should_be_hidden: bool = con_is_hidden(con);
    if should_be_hidden as ::core::ffi::c_int == (*state).is_hidden as ::core::ffi::c_int {
        return;
    }
    if should_be_hidden {
        debuglog(
            b"%s:%s:%d - setting _NET_WM_STATE_HIDDEN for con = %p\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/x.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"set_hidden_state\0" as *const u8 as *const ::core::ffi::c_char,
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
            b"%s:%s:%d - removing _NET_WM_STATE_HIDDEN for con = %p\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/x.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"set_hidden_state\0" as *const u8 as *const ::core::ffi::c_char,
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
unsafe extern "C" fn set_maximized_state(mut con: *mut Con) {
    if (*con).window.is_null() {
        return;
    }
    let mut state: *mut con_state = state_for_frame((*con).frame.id as xcb_window_t);
    let con_maximized_horz: bool = con_is_maximized(con, HORIZ) as bool;
    if con_maximized_horz as ::core::ffi::c_int != (*state).is_maximized_horz as ::core::ffi::c_int
    {
        debuglog(
            b"%s:%s:%d - setting _NET_WM_STATE_MAXIMIZED_HORZ for con %p(%s) to %d\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/x.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"set_maximized_state\0" as *const u8 as *const ::core::ffi::c_char,
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
    let con_maximized_vert: bool = con_is_maximized(con, VERT) as bool;
    if con_maximized_vert as ::core::ffi::c_int != (*state).is_maximized_vert as ::core::ffi::c_int
    {
        debuglog(
            b"%s:%s:%d - setting _NET_WM_STATE_MAXIMIZED_VERT for con %p(%s) to %d\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/x.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"set_maximized_state\0" as *const u8 as *const ::core::ffi::c_char,
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
unsafe extern "C" fn x_shape_frame(mut con: *mut Con, mut shape_kind: xcb_shape_sk_t) {
    if !(*con).window.is_null() {
    } else {
        __assert_fail(
            b"con->window\0" as *const u8 as *const ::core::ffi::c_char,
            b"../src/x.c\0" as *const u8 as *const ::core::ffi::c_char,
            873 as ::core::ffi::c_uint,
            b"void x_shape_frame(Con *, xcb_shape_sk_t)\0" as *const u8
                as *const ::core::ffi::c_char,
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
            .wrapping_add((*con).border_width as uint32_t) as int16_t,
        (*con)
            .window_rect
            .y
            .wrapping_add((*con).border_width as uint32_t) as int16_t,
        (*(*con).window).id,
    );
    let mut rectangles: [xcb_rectangle_t; 4] = [xcb_rectangle_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    }; 4];
    let mut rectangles_count: size_t =
        x_get_border_rectangles(con, &raw mut rectangles as *mut xcb_rectangle_t);
    if rectangles_count != 0 {
        xcb_shape_rectangles(
            conn,
            XCB_SHAPE_SO_UNION as ::core::ffi::c_int as xcb_shape_op_t,
            shape_kind as xcb_shape_kind_t,
            XCB_CLIP_ORDERING_UNSORTED as ::core::ffi::c_int as uint8_t,
            (*con).frame.id as xcb_window_t,
            0 as int16_t,
            0 as int16_t,
            rectangles_count as uint32_t,
            &raw mut rectangles as *mut xcb_rectangle_t,
        );
    }
}
unsafe extern "C" fn x_unshape_frame(mut con: *mut Con, mut shape_kind: xcb_shape_sk_t) {
    if !(*con).window.is_null() {
    } else {
        __assert_fail(
            b"con->window\0" as *const u8 as *const ::core::ffi::c_char,
            b"../src/x.c\0" as *const u8 as *const ::core::ffi::c_char,
            893 as ::core::ffi::c_uint,
            b"void x_unshape_frame(Con *, xcb_shape_sk_t)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    };
    xcb_shape_mask(
        conn,
        XCB_SHAPE_SO_SET as ::core::ffi::c_int as xcb_shape_op_t,
        shape_kind as xcb_shape_kind_t,
        (*con).frame.id as xcb_window_t,
        0 as int16_t,
        0 as int16_t,
        XCB_PIXMAP_NONE as ::core::ffi::c_int as xcb_pixmap_t,
    );
}
unsafe extern "C" fn set_shape_state(mut con: *mut Con, mut need_reshape: bool) {
    if !shape_supported || (*con).window.is_null() {
        return;
    }
    let mut state: *mut con_state = ::core::ptr::null_mut::<con_state>();
    state = state_for_frame((*con).frame.id as xcb_window_t);
    if state.is_null() {
        errorlog(
            b"ERROR: window state for con %p not found\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
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
#[no_mangle]
pub unsafe extern "C" fn x_push_node(mut con: *mut Con) {
    let mut current: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut rect: Rect = (*con).rect as Rect;
    let mut state: *mut con_state = state_for_frame((*con).frame.id as xcb_window_t);
    if !(*state).name.is_null() {
        debuglog(
            b"%s:%s:%d - pushing name %s for con %p\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b"../src/x.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"x_push_node\0" as *const u8 as *const ::core::ffi::c_char,
            946 as ::core::ffi::c_int,
            (*state).name,
            con,
        );
        xcb_change_property(
            conn,
            XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as uint8_t,
            (*con).frame.id as xcb_window_t,
            XCB_ATOM_WM_NAME as ::core::ffi::c_int as xcb_atom_t,
            XCB_ATOM_STRING as ::core::ffi::c_int as xcb_atom_t,
            8 as uint8_t,
            strlen((*state).name) as uint32_t,
            (*state).name as *const ::core::ffi::c_void,
        );
        free((*state).name as *mut ::core::ffi::c_void);
        (*state).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if (*con).window.is_null()
        && ((*con).layout as ::core::ffi::c_uint
            == L_STACKED as ::core::ffi::c_int as ::core::ffi::c_uint
            || (*con).layout as ::core::ffi::c_uint
                == L_TABBED as ::core::ffi::c_int as ::core::ffi::c_uint)
    {
        let mut max_y: uint32_t = 0 as uint32_t;
        let mut max_height: uint32_t = 0 as uint32_t;
        current = (*con).nodes_head.tqh_first as *mut Con;
        while !current.is_null() {
            let mut dr: *mut Rect = &raw mut (*current).deco_rect;
            if (*dr).y >= max_y && (*dr).height >= max_height {
                max_y = (*dr).y;
                max_height = (*dr).height;
            }
            current = (*current).nodes.tqe_next as *mut Con;
        }
        rect.height = max_y.wrapping_add(max_height);
        if rect.height == 0 as uint32_t {
            (*con).mapped = false_0 != 0;
        }
    } else if (*con).window.is_null() {
        (*con).mapped = false_0 != 0;
    }
    let mut need_reshape: bool = false_0 != 0;
    if (*state).need_reparent as ::core::ffi::c_int != 0 && !(*con).window.is_null() {
        debuglog(
            b"%s:%s:%d - Reparenting child window\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b"../src/x.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"x_push_node\0" as *const u8 as *const ::core::ffi::c_char,
            978 as ::core::ffi::c_int,
        );
        let mut values: [uint32_t; 1] = [XCB_NONE as uint32_t];
        xcb_change_window_attributes(
            conn,
            (*state).old_frame,
            XCB_CW_EVENT_MASK as ::core::ffi::c_int as uint32_t,
            &raw mut values as *mut uint32_t as *const ::core::ffi::c_void,
        );
        xcb_change_window_attributes(
            conn,
            (*(*con).window).id,
            XCB_CW_EVENT_MASK as ::core::ffi::c_int as uint32_t,
            &raw mut values as *mut uint32_t as *const ::core::ffi::c_void,
        );
        xcb_reparent_window(
            conn,
            (*(*con).window).id,
            (*con).frame.id as xcb_window_t,
            0 as int16_t,
            0 as int16_t,
        );
        values[0 as ::core::ffi::c_int as usize] =
            (XCB_EVENT_MASK_BUTTON_PRESS as ::core::ffi::c_int
                | XCB_EVENT_MASK_BUTTON_RELEASE as ::core::ffi::c_int
                | XCB_EVENT_MASK_POINTER_MOTION as ::core::ffi::c_int
                | XCB_EVENT_MASK_EXPOSURE as ::core::ffi::c_int
                | XCB_EVENT_MASK_STRUCTURE_NOTIFY as ::core::ffi::c_int
                | XCB_EVENT_MASK_SUBSTRUCTURE_REDIRECT as ::core::ffi::c_int
                | XCB_EVENT_MASK_SUBSTRUCTURE_NOTIFY as ::core::ffi::c_int
                | XCB_EVENT_MASK_ENTER_WINDOW as ::core::ffi::c_int) as uint32_t;
        xcb_change_window_attributes(
            conn,
            (*state).old_frame,
            XCB_CW_EVENT_MASK as ::core::ffi::c_int as uint32_t,
            &raw mut values as *mut uint32_t as *const ::core::ffi::c_void,
        );
        values[0 as ::core::ffi::c_int as usize] =
            (XCB_EVENT_MASK_PROPERTY_CHANGE as ::core::ffi::c_int
                | XCB_EVENT_MASK_STRUCTURE_NOTIFY as ::core::ffi::c_int
                | XCB_EVENT_MASK_FOCUS_CHANGE as ::core::ffi::c_int) as uint32_t;
        xcb_change_window_attributes(
            conn,
            (*(*con).window).id,
            XCB_CW_EVENT_MASK as ::core::ffi::c_int as uint32_t,
            &raw mut values as *mut uint32_t as *const ::core::ffi::c_void,
        );
        (*state).old_frame = XCB_NONE as xcb_window_t;
        (*state).need_reparent = false_0 != 0;
        (*con).ignore_unmap = (*con).ignore_unmap.wrapping_add(1);
        debuglog(
            b"%s:%s:%d - ignore_unmap for reparenting of con %p (win 0x%08x) is now %d\n\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/x.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"x_push_node\0" as *const u8 as *const ::core::ffi::c_char,
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
            as ::core::ffi::c_int) != 0;
    need_reshape = (need_reshape as ::core::ffi::c_int
        | (con_is_floating(con) as ::core::ffi::c_int != 0 && !(*state).was_floating)
            as ::core::ffi::c_int) != 0;
    let mut is_pixmap_needed: bool = con_is_leaf(con) as ::core::ffi::c_int != 0
        && con_border_style(con) != BS_NONE as ::core::ffi::c_int
        || (*con).layout as ::core::ffi::c_uint
            == L_STACKED as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*con).layout as ::core::ffi::c_uint
            == L_TABBED as ::core::ffi::c_int as ::core::ffi::c_uint;
    debuglog(
        b"%s:%s:%d - Con %p (layout %d), is_pixmap_needed = %s, rect.height = %d\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/x.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"x_push_node\0" as *const u8 as *const ::core::ffi::c_char,
        1019 as ::core::ffi::c_int,
        con,
        (*con).layout as ::core::ffi::c_uint,
        if is_pixmap_needed as ::core::ffi::c_int != 0 {
            b"yes\0" as *const u8 as *const ::core::ffi::c_char
        } else {
            b"no\0" as *const u8 as *const ::core::ffi::c_char
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
        || !rect_equals((*state).rect, rect) && rect.height > 0 as uint32_t
    {
        let mut has_rect_changed: bool = (*state).rect.x != rect.x
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
            let mut win_depth: uint16_t = root_depth as uint16_t;
            if !(*con).window.is_null() {
                win_depth = (*(*con).window).depth;
            }
            let mut width: ::core::ffi::c_int = if rect.width as int32_t > 1 as int32_t {
                rect.width as ::core::ffi::c_int
            } else {
                1 as ::core::ffi::c_int
            };
            let mut height: ::core::ffi::c_int = if rect.height as int32_t > 1 as int32_t {
                rect.height as ::core::ffi::c_int
            } else {
                1 as ::core::ffi::c_int
            };
            debuglog(
                b"%s:%s:%d - creating %d x %d pixmap for con %p (con->frame_buffer.id = (pixmap_t)0x%08x) (con->frame.id (drawable_t)0x%08x)\n\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"../src/x.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"x_push_node\0" as *const u8 as *const ::core::ffi::c_char,
                1068 as ::core::ffi::c_int,
                width,
                height,
                con,
                (*con).frame_buffer.id,
                (*con).frame.id,
            );
            xcb_create_pixmap(
                conn,
                win_depth as uint8_t,
                (*con).frame_buffer.id as xcb_pixmap_t,
                (*con).frame.id,
                width as uint16_t,
                height as uint16_t,
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
            let mut fresh4: [uint32_t; 1] = [0 as ::core::ffi::c_int as uint32_t];
            xcb_change_gc(
                conn,
                (*con).frame_buffer.gc,
                XCB_GC_GRAPHICS_EXPOSURES as ::core::ffi::c_int as uint32_t,
                &raw mut fresh4 as *mut uint32_t as *const ::core::ffi::c_void,
            );
            draw_util_surface_set_size(&raw mut (*con).frame, width, height);
            (*con).pixmap_recreated = true_0 != 0;
            if (*con).parent.is_null()
                || (*(*con).parent).layout as ::core::ffi::c_uint
                    != L_STACKED as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*(*con).parent).focus_head.tqh_first == con
            {
                x_deco_recurse(con);
            }
        }
        debuglog(
            b"%s:%s:%d - setting rect (%d, %d, %d, %d)\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/x.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"x_push_node\0" as *const u8 as *const ::core::ffi::c_char,
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
            ::core::mem::size_of::<Rect>() as size_t,
        );
        fake_notify = true_0 != 0;
    }
    if !(*con).window.is_null() && !rect_equals((*state).window_rect, (*con).window_rect as Rect) {
        debuglog(
            b"%s:%s:%d - setting window rect (%d, %d, %d, %d)\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/x.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"x_push_node\0" as *const u8 as *const ::core::ffi::c_char,
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
            ::core::mem::size_of::<Rect>() as size_t,
        );
        fake_notify = true_0 != 0;
    }
    set_shape_state(con, need_reshape);
    if !con.is_null() && !(*con).window.is_null() {
        let mut bsr: Rect = con_border_style_rect(con);
        let mut r: Rect = Rect {
            x: bsr.x,
            y: (0 as uint32_t).wrapping_sub(bsr.width).wrapping_sub(bsr.x),
            width: bsr.y,
            height: (0 as uint32_t).wrapping_sub(bsr.height).wrapping_sub(bsr.y),
        };
        xcb_change_property(
            conn,
            XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as uint8_t,
            (*(*con).window).id,
            A__NET_FRAME_EXTENTS,
            XCB_ATOM_CARDINAL as ::core::ffi::c_int as xcb_atom_t,
            32 as uint8_t,
            4 as uint32_t,
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
                XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as uint8_t,
                (*(*con).window).id,
                A_WM_STATE,
                A_WM_STATE,
                32 as uint8_t,
                2 as uint32_t,
                &raw mut data as *mut ::core::ffi::c_long as *const ::core::ffi::c_void,
            );
        }
        let mut values_0: [uint32_t; 1] = [0; 1];
        if !(*state).child_mapped && !(*con).window.is_null() {
            cookie = xcb_map_window(conn, (*(*con).window).id);
            values_0[0 as ::core::ffi::c_int as usize] = (XCB_EVENT_MASK_PROPERTY_CHANGE
                as ::core::ffi::c_int
                | XCB_EVENT_MASK_STRUCTURE_NOTIFY as ::core::ffi::c_int
                | XCB_EVENT_MASK_FOCUS_CHANGE as ::core::ffi::c_int)
                as uint32_t;
            xcb_change_window_attributes(
                conn,
                (*(*con).window).id,
                XCB_CW_EVENT_MASK as ::core::ffi::c_int as uint32_t,
                &raw mut values_0 as *mut uint32_t as *const ::core::ffi::c_void,
            );
            debuglog(
                b"%s:%s:%d - mapping child window (serial %d)\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"../src/x.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"x_push_node\0" as *const u8 as *const ::core::ffi::c_char,
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
                | XCB_EVENT_MASK_ENTER_WINDOW as ::core::ffi::c_int) as uint32_t;
        xcb_change_window_attributes(
            conn,
            (*con).frame.id as xcb_window_t,
            XCB_CW_EVENT_MASK as ::core::ffi::c_int as uint32_t,
            &raw mut values_0 as *mut uint32_t as *const ::core::ffi::c_void,
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
            b"%s:%s:%d - mapping container %08x (serial %d)\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/x.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"x_push_node\0" as *const u8 as *const ::core::ffi::c_char,
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
            b"%s:%s:%d - Sending fake configure notify\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/x.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"x_push_node\0" as *const u8 as *const ::core::ffi::c_char,
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
unsafe extern "C" fn x_push_node_unmaps(mut con: *mut Con) {
    let mut current: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut state: *mut con_state = state_for_frame((*con).frame.id as xcb_window_t);
    if (*state).unmap_now {
        if !(*con).window.is_null() {
            let mut data: [::core::ffi::c_long; 2] = [
                XCB_ICCCM_WM_STATE_WITHDRAWN as ::core::ffi::c_int as ::core::ffi::c_long,
                XCB_NONE,
            ];
            xcb_change_property(
                conn,
                XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as uint8_t,
                (*(*con).window).id,
                A_WM_STATE,
                A_WM_STATE,
                32 as uint8_t,
                2 as uint32_t,
                &raw mut data as *mut ::core::ffi::c_long as *const ::core::ffi::c_void,
            );
        }
        let cookie: xcb_void_cookie_t =
            xcb_unmap_window(conn, (*con).frame.id as xcb_window_t) as xcb_void_cookie_t;
        debuglog(
            b"%s:%s:%d - unmapping container %p / %s (serial %d)\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/x.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"x_push_node_unmaps\0" as *const u8 as *const ::core::ffi::c_char,
            1229 as ::core::ffi::c_int,
            con,
            (*con).name,
            cookie.sequence,
        );
        if !(*con).window.is_null() {
            (*con).ignore_unmap = (*con).ignore_unmap.wrapping_add(1);
            debuglog(
                b"%s:%s:%d - ignore_unmap for con %p (frame 0x%08x) now %d\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"../src/x.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"x_push_node_unmaps\0" as *const u8 as *const ::core::ffi::c_char,
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
unsafe extern "C" fn is_con_attached(mut con: *mut Con) -> bool {
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
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn x_push_changes(mut con: *mut Con) {
    let mut state: *mut con_state = ::core::ptr::null_mut::<con_state>();
    let mut pointercookie: xcb_query_pointer_cookie_t = xcb_query_pointer_cookie_t { sequence: 0 };
    if !warp_to.is_null() {
        pointercookie = xcb_query_pointer(conn, root);
    }
    debuglog(
        b"%s:%s:%d - -- PUSHING WINDOW STACK --\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/x.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"x_push_changes\0" as *const u8 as *const ::core::ffi::c_char,
        1290 as ::core::ffi::c_int,
    );
    let mut values: [uint32_t; 1] =
        [XCB_EVENT_MASK_SUBSTRUCTURE_REDIRECT as ::core::ffi::c_int as uint32_t];
    state = state_head.cqh_last as *mut con_state;
    while state != &raw mut state_head as *mut ::core::ffi::c_void as *mut con_state {
        if (*state).mapped {
            xcb_change_window_attributes(
                conn,
                (*state).id,
                XCB_CW_EVENT_MASK as ::core::ffi::c_int as uint32_t,
                &raw mut values as *mut uint32_t as *const ::core::ffi::c_void,
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
            (::core::mem::size_of::<xcb_window_t>() as size_t).wrapping_mul(cnt as size_t),
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
                ::core::mem::size_of::<xcb_window_t>() as size_t,
            );
        }
        let mut prev: *mut con_state = (*state).state.cqe_prev as *mut con_state;
        let mut old_prev: *mut con_state = (*state).old_state.cqe_prev as *mut con_state;
        if prev != old_prev {
            order_changed = true_0 != 0;
        }
        if ((*state).initial as ::core::ffi::c_int != 0 || order_changed as ::core::ffi::c_int != 0)
            && prev != &raw mut state_head as *mut ::core::ffi::c_void as *mut con_state
        {
            stacking_changed = true_0 != 0;
            let mut mask: uint32_t = 0 as uint32_t;
            mask |= XCB_CONFIG_WINDOW_SIBLING as ::core::ffi::c_int as uint32_t;
            mask |= XCB_CONFIG_WINDOW_STACK_MODE as ::core::ffi::c_int as uint32_t;
            let mut values_0: [uint32_t; 2] = [
                (*state).id,
                XCB_STACK_MODE_ABOVE as ::core::ffi::c_int as uint32_t,
            ];
            xcb_configure_window(
                conn,
                (*prev).id,
                mask as uint16_t,
                &raw mut values_0 as *mut uint32_t as *const ::core::ffi::c_void,
            );
        }
        (*state).initial = false_0 != 0;
        state = (*state).state.cqe_prev as *mut con_state;
    }
    if stacking_changed {
        debuglog(
            b"%s:%s:%d - Client list changed (%i clients)\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/x.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"x_push_changes\0" as *const u8 as *const ::core::ffi::c_char,
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
        b"%s:%s:%d - PUSHING CHANGES\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/x.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"x_push_changes\0" as *const u8 as *const ::core::ffi::c_char,
        1365 as ::core::ffi::c_int,
    );
    x_push_node(con);
    if !warp_to.is_null() {
        let mut pointerreply: *mut xcb_query_pointer_reply_t = xcb_query_pointer_reply(
            conn,
            pointercookie,
            ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
        );
        if pointerreply.is_null() {
            errorlog(
                b"ERROR: Could not query pointer position, not warping pointer\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        } else {
            let mut mid_x: ::core::ffi::c_int = (*warp_to)
                .x
                .wrapping_add((*warp_to).width.wrapping_div(2 as uint32_t))
                as ::core::ffi::c_int;
            let mut mid_y: ::core::ffi::c_int = (*warp_to)
                .y
                .wrapping_add((*warp_to).height.wrapping_div(2 as uint32_t))
                as ::core::ffi::c_int;
            let mut current: *mut Output = get_output_containing(
                (*pointerreply).root_x as ::core::ffi::c_uint,
                (*pointerreply).root_y as ::core::ffi::c_uint,
            );
            let mut target: *mut Output =
                get_output_containing(mid_x as ::core::ffi::c_uint, mid_y as ::core::ffi::c_uint);
            if current != target {
                let mut fresh7: [uint32_t; 1] =
                    [XCB_EVENT_MASK_SUBSTRUCTURE_REDIRECT as ::core::ffi::c_int as uint32_t];
                xcb_change_window_attributes(
                    conn,
                    root,
                    XCB_CW_EVENT_MASK as ::core::ffi::c_int as uint32_t,
                    &raw mut fresh7 as *mut uint32_t as *const ::core::ffi::c_void,
                );
                xcb_warp_pointer(
                    conn,
                    XCB_NONE as xcb_window_t,
                    root,
                    0 as int16_t,
                    0 as int16_t,
                    0 as uint16_t,
                    0 as uint16_t,
                    mid_x as int16_t,
                    mid_y as int16_t,
                );
                let mut fresh8: [uint32_t; 1] = [(XCB_EVENT_MASK_SUBSTRUCTURE_REDIRECT
                    as ::core::ffi::c_int
                    | XCB_EVENT_MASK_BUTTON_PRESS as ::core::ffi::c_int
                    | XCB_EVENT_MASK_STRUCTURE_NOTIFY as ::core::ffi::c_int
                    | XCB_EVENT_MASK_POINTER_MOTION as ::core::ffi::c_int
                    | XCB_EVENT_MASK_PROPERTY_CHANGE as ::core::ffi::c_int
                    | XCB_EVENT_MASK_FOCUS_CHANGE as ::core::ffi::c_int
                    | XCB_EVENT_MASK_ENTER_WINDOW as ::core::ffi::c_int)
                    as uint32_t];
                xcb_change_window_attributes(
                    conn,
                    root,
                    XCB_CW_EVENT_MASK as ::core::ffi::c_int as uint32_t,
                    &raw mut fresh8 as *mut uint32_t as *const ::core::ffi::c_void,
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
        as uint32_t;
    state = state_head.cqh_last as *mut con_state;
    while state != &raw mut state_head as *mut ::core::ffi::c_void as *mut con_state {
        if (*state).mapped {
            xcb_change_window_attributes(
                conn,
                (*state).id,
                XCB_CW_EVENT_MASK as ::core::ffi::c_int as uint32_t,
                &raw mut values as *mut uint32_t as *const ::core::ffi::c_void,
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
                b"%s:%s:%d - Not updating focus (to %p / %s), focused window is not mapped.\n\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"../src/x.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"x_push_changes\0" as *const u8 as *const ::core::ffi::c_char,
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
                    b"%s:%s:%d - Updating focus by sending WM_TAKE_FOCUS to window 0x%08x (focused: %p / %s)\n\0"
                        as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    b"../src/x.c\0" as *const u8 as *const ::core::ffi::c_char,
                    b"x_push_changes\0" as *const u8 as *const ::core::ffi::c_char,
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
                    ipc_send_window_event(
                        b"focus\0" as *const u8 as *const ::core::ffi::c_char,
                        focused,
                    );
                }
            } else {
                debuglog(
                    b"%s:%s:%d - Updating focus (focused: %p / %s) to X11 window 0x%08x\n\0"
                        as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    b"../src/x.c\0" as *const u8 as *const ::core::ffi::c_char,
                    b"x_push_changes\0" as *const u8 as *const ::core::ffi::c_char,
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
                        as uint32_t;
                    xcb_change_window_attributes(
                        conn,
                        (*(*focused).window).id,
                        XCB_CW_EVENT_MASK as ::core::ffi::c_int as uint32_t,
                        &raw mut values as *mut uint32_t as *const ::core::ffi::c_void,
                    );
                }
                xcb_set_input_focus(
                    conn,
                    XCB_INPUT_FOCUS_POINTER_ROOT as ::core::ffi::c_int as uint8_t,
                    to_focus,
                    last_timestamp,
                );
                if !(*focused).window.is_null() {
                    values[0 as ::core::ffi::c_int as usize] = (XCB_EVENT_MASK_PROPERTY_CHANGE
                        as ::core::ffi::c_int
                        | XCB_EVENT_MASK_STRUCTURE_NOTIFY as ::core::ffi::c_int
                        | XCB_EVENT_MASK_FOCUS_CHANGE as ::core::ffi::c_int)
                        as uint32_t;
                    xcb_change_window_attributes(
                        conn,
                        (*(*focused).window).id,
                        XCB_CW_EVENT_MASK as ::core::ffi::c_int as uint32_t,
                        &raw mut values as *mut uint32_t as *const ::core::ffi::c_void,
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
                    ipc_send_window_event(
                        b"focus\0" as *const u8 as *const ::core::ffi::c_char,
                        focused,
                    );
                }
            }
            last_focused = to_focus;
            focused_id = last_focused;
        }
    }
    if focused_id as ::core::ffi::c_long == XCB_NONE {
        debuglog(
            b"%s:%s:%d - Still no window focused, better set focus to the EWMH support window (%d)\n\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/x.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"x_push_changes\0" as *const u8 as *const ::core::ffi::c_char,
            1451 as ::core::ffi::c_int,
            ewmh_window,
        );
        xcb_set_input_focus(
            conn,
            XCB_INPUT_FOCUS_POINTER_ROOT as ::core::ffi::c_int as uint8_t,
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
        b"%s:%s:%d - ENDING CHANGES\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/x.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"x_push_changes\0" as *const u8 as *const ::core::ffi::c_char,
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
        as uint32_t;
    state = state_head.cqh_last as *mut con_state;
    while state != &raw mut state_head as *mut ::core::ffi::c_void as *mut con_state {
        if (*state).unmap_now {
            xcb_change_window_attributes(
                conn,
                (*state).id,
                XCB_CW_EVENT_MASK as ::core::ffi::c_int as uint32_t,
                &raw mut values as *mut uint32_t as *const ::core::ffi::c_void,
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
#[no_mangle]
pub unsafe extern "C" fn x_raise_con(mut con: *const Con) {
    let mut state: *mut con_state = state_for_frame((*con).frame.id as xcb_window_t);
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
#[no_mangle]
pub unsafe extern "C" fn x_set_name(mut con: *mut Con, mut name: *const ::core::ffi::c_char) {
    let mut state: *mut con_state = ::core::ptr::null_mut::<con_state>();
    state = state_for_frame((*con).frame.id as xcb_window_t);
    if state.is_null() {
        errorlog(
            b"ERROR: window state not found\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        return;
    }
    free((*state).name as *mut ::core::ffi::c_void);
    (*state).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*state).name = sstrdup(name);
}
#[no_mangle]
pub unsafe extern "C" fn update_shmlog_atom() {
    if *shmlogname as ::core::ffi::c_int == '\0' as i32 {
        xcb_delete_property(conn, root, A_I3_SHMLOG_PATH);
    } else {
        xcb_change_property(
            conn,
            XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as uint8_t,
            root,
            A_I3_SHMLOG_PATH,
            A_UTF8_STRING,
            8 as uint8_t,
            strlen(shmlogname) as uint32_t,
            shmlogname as *const ::core::ffi::c_void,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn x_set_i3_atoms() {
    let mut pid: pid_t = getpid() as pid_t;
    xcb_change_property(
        conn,
        XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as uint8_t,
        root,
        A_I3_SOCKET_PATH,
        A_UTF8_STRING,
        8 as uint8_t,
        (if current_socketpath.is_null() {
            0 as size_t
        } else {
            strlen(current_socketpath)
        }) as uint32_t,
        current_socketpath as *const ::core::ffi::c_void,
    );
    xcb_change_property(
        conn,
        XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as uint8_t,
        root,
        A_I3_PID,
        XCB_ATOM_CARDINAL as ::core::ffi::c_int as xcb_atom_t,
        32 as uint8_t,
        1 as uint32_t,
        &raw mut pid as *const ::core::ffi::c_void,
    );
    xcb_change_property(
        conn,
        XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as uint8_t,
        root,
        A_I3_CONFIG_PATH,
        A_UTF8_STRING,
        8 as uint8_t,
        strlen(current_configpath) as uint32_t,
        current_configpath as *const ::core::ffi::c_void,
    );
    xcb_change_property(
        conn,
        XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as uint8_t,
        root,
        A_I3_LOG_STREAM_SOCKET_PATH,
        A_UTF8_STRING,
        8 as uint8_t,
        strlen(current_log_stream_socket_path) as uint32_t,
        current_log_stream_socket_path as *const ::core::ffi::c_void,
    );
    update_shmlog_atom();
}
#[no_mangle]
pub unsafe extern "C" fn x_set_warp_to(mut rect: *mut Rect) {
    if config.mouse_warping as ::core::ffi::c_uint
        != POINTER_WARPING_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        warp_to = rect;
    }
}
#[no_mangle]
pub unsafe extern "C" fn x_mask_event_mask(mut mask: uint32_t) {
    let mut values: [uint32_t; 1] = [(XCB_EVENT_MASK_BUTTON_PRESS as ::core::ffi::c_int
        | XCB_EVENT_MASK_BUTTON_RELEASE as ::core::ffi::c_int
        | XCB_EVENT_MASK_POINTER_MOTION as ::core::ffi::c_int
        | XCB_EVENT_MASK_EXPOSURE as ::core::ffi::c_int
        | XCB_EVENT_MASK_STRUCTURE_NOTIFY as ::core::ffi::c_int
        | XCB_EVENT_MASK_SUBSTRUCTURE_REDIRECT as ::core::ffi::c_int
        | XCB_EVENT_MASK_SUBSTRUCTURE_NOTIFY as ::core::ffi::c_int
        | XCB_EVENT_MASK_ENTER_WINDOW as ::core::ffi::c_int)
        as uint32_t
        & mask];
    let mut state: *mut con_state = ::core::ptr::null_mut::<con_state>();
    state = state_head.cqh_last as *mut con_state;
    while state != &raw mut state_head as *mut ::core::ffi::c_void as *mut con_state {
        if (*state).mapped {
            xcb_change_window_attributes(
                conn,
                (*state).id,
                XCB_CW_EVENT_MASK as ::core::ffi::c_int as uint32_t,
                &raw mut values as *mut uint32_t as *const ::core::ffi::c_void,
            );
        }
        state = (*state).state.cqe_prev as *mut con_state;
    }
}
#[no_mangle]
pub unsafe extern "C" fn x_set_shape(
    mut con: *mut Con,
    mut kind: xcb_shape_sk_t,
    mut enable: bool,
) {
    if state_for_frame((*con).frame.id as xcb_window_t).is_null() {
        errorlog(
            b"ERROR: window state for con %p not found\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
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
                b"ERROR: Received unknown shape event kind for con %p. This is a bug.\n\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
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
