/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * i3-nagbar is a utility which displays a nag message, for example in the case
 * when the user has an error in their configuration file.
 *
 */
// Mechanical file-by-file Rust translation of i3-nagbar/main.c.
// Generated with C2Rust 0.22.1; C ABI and original ownership retained.
use ::c2rust_bitfields;
include!("../rust/variadic-compat.rs");

extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type xcb_connection_t;
    pub type _PangoFontDescription;
    pub type _cairo;
    pub type _cairo_surface;
    pub type _i3String;
    pub type xcb_cursor_context_t;
    pub type SnDisplay;
    pub type SnLauncheeContext;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fdopen(__fd: ::core::ffi::c_int, __modes: *const ::core::ffi::c_char) -> *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn vfprintf(
        __s: *mut FILE,
        __format: *const ::core::ffi::c_char,
        __arg: ::core::ffi::VaList,
    ) -> ::core::ffi::c_int;
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
    fn xcb_map_window(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t;
    fn xcb_get_geometry(
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
    ) -> xcb_get_geometry_cookie_t;
    fn xcb_get_geometry_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_get_geometry_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_get_geometry_reply_t;
    fn xcb_intern_atom(
        c: *mut xcb_connection_t,
        only_if_exists: uint8_t,
        name_len: uint16_t,
        name: *const ::core::ffi::c_char,
    ) -> xcb_intern_atom_cookie_t;
    fn xcb_intern_atom_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_intern_atom_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_intern_atom_reply_t;
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
    fn xcb_grab_server(c: *mut xcb_connection_t) -> xcb_void_cookie_t;
    fn xcb_ungrab_server(c: *mut xcb_connection_t) -> xcb_void_cookie_t;
    fn xcb_translate_coordinates(
        c: *mut xcb_connection_t,
        src_window: xcb_window_t,
        dst_window: xcb_window_t,
        src_x: int16_t,
        src_y: int16_t,
    ) -> xcb_translate_coordinates_cookie_t;
    fn xcb_translate_coordinates_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_translate_coordinates_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_translate_coordinates_reply_t;
    fn xcb_get_input_focus(c: *mut xcb_connection_t) -> xcb_get_input_focus_cookie_t;
    fn xcb_get_input_focus_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_get_input_focus_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_get_input_focus_reply_t;
    fn xcb_flush(c: *mut xcb_connection_t) -> ::core::ffi::c_int;
    fn xcb_wait_for_event(c: *mut xcb_connection_t) -> *mut xcb_generic_event_t;
    fn xcb_connection_has_error(c: *mut xcb_connection_t) -> ::core::ffi::c_int;
    fn xcb_connect(
        displayname: *const ::core::ffi::c_char,
        screenp: *mut ::core::ffi::c_int,
    ) -> *mut xcb_connection_t;
    fn xcb_generate_id(c: *mut xcb_connection_t) -> uint32_t;
    fn execl(
        __path: *const ::core::ffi::c_char,
        __arg: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn setsid() -> __pid_t;
    fn fork() -> __pid_t;
    fn symlink(
        __from: *const ::core::ffi::c_char,
        __to: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn unlink(__name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    static mut optarg: *mut ::core::ffi::c_char;
    static mut optind: ::core::ffi::c_int;
    fn srealloc(ptr: *mut ::core::ffi::c_void, size: size_t) -> *mut ::core::ffi::c_void;
    fn sstrdup(str: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn sasprintf(
        strp: *mut *mut ::core::ffi::c_char,
        fmt: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn i3string_from_utf8(from_utf8: *const ::core::ffi::c_char) -> *mut i3String;
    fn i3string_free(str: *mut i3String);
    fn i3string_as_utf8(str: *mut i3String) -> *const ::core::ffi::c_char;
    fn load_font(pattern: *const ::core::ffi::c_char, fallback: bool) -> i3Font;
    fn set_font(font_0: *mut i3Font);
    fn predict_text_width(text: *mut i3String) -> ::core::ffi::c_int;
    fn get_visualtype(screen: *mut xcb_screen_t) -> *mut xcb_visualtype_t;
    fn get_process_filename(prefix: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn get_exe_path(argv0_0: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn init_dpi();
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
    fn draw_util_hex_to_color(color: *const ::core::ffi::c_char) -> color_t;
    fn draw_util_text(
        text: *mut i3String,
        surface: *mut surface_t,
        fg_color: color_t,
        bg_color: color_t,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
        max_width: ::core::ffi::c_int,
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
    fn warn(__format: *const ::core::ffi::c_char, ...);
    fn err(__status: ::core::ffi::c_int, __format: *const ::core::ffi::c_char, ...) -> !;
    fn errx(__status: ::core::ffi::c_int, __format: *const ::core::ffi::c_char, ...) -> !;
    fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    fn getopt_long(
        ___argc: ::core::ffi::c_int,
        ___argv: *const *mut ::core::ffi::c_char,
        __shortopts: *const ::core::ffi::c_char,
        __longopts: *const option,
        __longind: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn wait(__stat_loc: *mut ::core::ffi::c_int) -> __pid_t;
    fn xcb_randr_get_output_info(
        c: *mut xcb_connection_t,
        output: xcb_randr_output_t,
        config_timestamp: xcb_timestamp_t,
    ) -> xcb_randr_get_output_info_cookie_t;
    fn xcb_randr_get_output_info_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_randr_get_output_info_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_randr_get_output_info_reply_t;
    fn xcb_randr_get_crtc_info(
        c: *mut xcb_connection_t,
        crtc: xcb_randr_crtc_t,
        config_timestamp: xcb_timestamp_t,
    ) -> xcb_randr_get_crtc_info_cookie_t;
    fn xcb_randr_get_crtc_info_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_randr_get_crtc_info_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_randr_get_crtc_info_reply_t;
    fn xcb_randr_get_screen_resources_current(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_randr_get_screen_resources_current_cookie_t;
    fn xcb_randr_get_screen_resources_current_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_randr_get_screen_resources_current_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_randr_get_screen_resources_current_reply_t;
    fn xcb_randr_get_output_primary(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_randr_get_output_primary_cookie_t;
    fn xcb_randr_get_output_primary_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_randr_get_output_primary_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_randr_get_output_primary_reply_t;
    fn xcb_aux_get_screen(
        c: *mut xcb_connection_t,
        screen: ::core::ffi::c_int,
    ) -> *mut xcb_screen_t;
    fn xcb_cursor_context_new(
        conn_0: *mut xcb_connection_t,
        screen: *mut xcb_screen_t,
        ctx: *mut *mut xcb_cursor_context_t,
    ) -> ::core::ffi::c_int;
    fn xcb_cursor_load_cursor(
        ctx: *mut xcb_cursor_context_t,
        name: *const ::core::ffi::c_char,
    ) -> xcb_cursor_t;
    fn xcb_cursor_context_free(ctx: *mut xcb_cursor_context_t);
    fn sn_xcb_display_new(
        xconnection: *mut xcb_connection_t,
        push_trap_func: SnXcbDisplayErrorTrapPush,
        pop_trap_func: SnXcbDisplayErrorTrapPop,
    ) -> *mut SnDisplay;
    fn sn_display_unref(display: *mut SnDisplay);
    fn sn_launchee_context_new_from_environment(
        display: *mut SnDisplay,
        screen: ::core::ffi::c_int,
    ) -> *mut SnLauncheeContext;
    fn sn_launchee_context_unref(context: *mut SnLauncheeContext);
    fn sn_launchee_context_complete(context: *mut SnLauncheeContext);
    fn sn_launchee_context_setup_window(context: *mut SnLauncheeContext, xwindow: Window);
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn strcasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn exit(__status: ::core::ffi::c_int) -> !;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: ::core::ffi::c_uint,
    pub fp_offset: ::core::ffi::c_uint,
    pub overflow_arg_area: *mut ::core::ffi::c_void,
    pub reg_save_area: *mut ::core::ffi::c_void,
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __pid_t = ::core::ffi::c_int;
pub type int16_t = __int16_t;
pub type __gnuc_va_list = __builtin_va_list;
pub type va_list = __gnuc_va_list;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: ::core::ffi::c_int,
    pub _IO_read_ptr: *mut ::core::ffi::c_char,
    pub _IO_read_end: *mut ::core::ffi::c_char,
    pub _IO_read_base: *mut ::core::ffi::c_char,
    pub _IO_write_base: *mut ::core::ffi::c_char,
    pub _IO_write_ptr: *mut ::core::ffi::c_char,
    pub _IO_write_end: *mut ::core::ffi::c_char,
    pub _IO_buf_base: *mut ::core::ffi::c_char,
    pub _IO_buf_end: *mut ::core::ffi::c_char,
    pub _IO_save_base: *mut ::core::ffi::c_char,
    pub _IO_backup_base: *mut ::core::ffi::c_char,
    pub _IO_save_end: *mut ::core::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::core::ffi::c_int,
    #[bitfield(name = "_flags2", ty = "::core::ffi::c_int", bits = "0..=23")]
    pub _flags2: [u8; 3],
    pub _short_backupbuf: [::core::ffi::c_char; 1],
    pub _old_offset: __off_t,
    pub _cur_column: ::core::ffi::c_ushort,
    pub _vtable_offset: ::core::ffi::c_schar,
    pub _shortbuf: [::core::ffi::c_char; 1],
    pub _lock: *mut ::core::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::core::ffi::c_void,
    pub _prevchain: *mut *mut _IO_FILE,
    pub _mode: ::core::ffi::c_int,
    pub _unused3: ::core::ffi::c_int,
    pub _total_written: __uint64_t,
    pub _unused2: [::core::ffi::c_char; 8],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
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
pub type xcb_font_t = uint32_t;
pub type xcb_gcontext_t = uint32_t;
pub type xcb_colormap_t = uint32_t;
pub type xcb_atom_t = uint32_t;
pub type xcb_drawable_t = uint32_t;
pub type xcb_visualid_t = uint32_t;
pub type xcb_timestamp_t = uint32_t;
pub type xcb_button_t = uint8_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_get_geometry_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
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
pub struct xcb_intern_atom_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_intern_atom_reply_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub atom: xcb_atom_t,
}
pub type xcb_prop_mode_t = ::core::ffi::c_uint;
pub const XCB_PROP_MODE_APPEND: xcb_prop_mode_t = 2;
pub const XCB_PROP_MODE_PREPEND: xcb_prop_mode_t = 1;
pub const XCB_PROP_MODE_REPLACE: xcb_prop_mode_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_translate_coordinates_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_translate_coordinates_reply_t {
    pub response_type: uint8_t,
    pub same_screen: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub child: xcb_window_t,
    pub dst_x: int16_t,
    pub dst_y: int16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_get_input_focus_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_get_input_focus_reply_t {
    pub response_type: uint8_t,
    pub revert_to: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub focus: xcb_window_t,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const ::core::ffi::c_char,
    pub has_arg: ::core::ffi::c_int,
    pub flag: *mut ::core::ffi::c_int,
    pub val: ::core::ffi::c_int,
}
pub type xcb_randr_mode_t = uint32_t;
pub type xcb_randr_crtc_t = uint32_t;
pub type xcb_randr_output_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_randr_get_output_info_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_randr_get_output_info_reply_t {
    pub response_type: uint8_t,
    pub status: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub timestamp: xcb_timestamp_t,
    pub crtc: xcb_randr_crtc_t,
    pub mm_width: uint32_t,
    pub mm_height: uint32_t,
    pub connection: uint8_t,
    pub subpixel_order: uint8_t,
    pub num_crtcs: uint16_t,
    pub num_modes: uint16_t,
    pub num_preferred: uint16_t,
    pub num_clones: uint16_t,
    pub name_len: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_randr_get_crtc_info_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_randr_get_crtc_info_reply_t {
    pub response_type: uint8_t,
    pub status: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub timestamp: xcb_timestamp_t,
    pub x: int16_t,
    pub y: int16_t,
    pub width: uint16_t,
    pub height: uint16_t,
    pub mode: xcb_randr_mode_t,
    pub rotation: uint16_t,
    pub rotations: uint16_t,
    pub num_outputs: uint16_t,
    pub num_possible_outputs: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_randr_get_screen_resources_current_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_randr_get_screen_resources_current_reply_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub timestamp: xcb_timestamp_t,
    pub config_timestamp: xcb_timestamp_t,
    pub num_crtcs: uint16_t,
    pub num_outputs: uint16_t,
    pub num_modes: uint16_t,
    pub names_len: uint16_t,
    pub pad1: [uint8_t; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_randr_get_output_primary_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_randr_get_output_primary_reply_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub output: xcb_randr_output_t,
}
pub type XID = ::core::ffi::c_ulong;
pub type Window = XID;
pub type SnXcbDisplayErrorTrapPush =
    Option<unsafe extern "C" fn(*mut SnDisplay, *mut xcb_connection_t) -> ()>;
pub type SnXcbDisplayErrorTrapPop =
    Option<unsafe extern "C" fn(*mut SnDisplay, *mut xcb_connection_t) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct button_t {
    pub label: *mut i3String,
    pub action: *mut ::core::ffi::c_char,
    pub x: int16_t,
    pub width: uint16_t,
    pub terminal: bool,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct C2RustUnnamed_2 {
    pub left: uint32_t,
    pub right: uint32_t,
    pub top: uint32_t,
    pub bottom: uint32_t,
    pub left_start_y: uint32_t,
    pub left_end_y: uint32_t,
    pub right_start_y: uint32_t,
    pub right_end_y: uint32_t,
    pub top_start_x: uint32_t,
    pub top_end_x: uint32_t,
    pub bottom_start_x: uint32_t,
    pub bottom_end_x: uint32_t,
}
pub const TYPE_ERROR: C2RustUnnamed_3 = 0;
pub type C2RustUnnamed_3 = ::core::ffi::c_uint;
pub const TYPE_WARNING: C2RustUnnamed_3 = 1;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const __S_IREAD: ::core::ffi::c_int = 0o400 as ::core::ffi::c_int;
pub const __S_IWRITE: ::core::ffi::c_int = 0o200 as ::core::ffi::c_int;
pub const XCB_BUTTON_PRESS: ::core::ffi::c_int = 4;
pub const XCB_BUTTON_RELEASE: ::core::ffi::c_int = 5;
pub const XCB_EXPOSE: ::core::ffi::c_int = 12;
pub const XCB_CONFIGURE_NOTIFY: ::core::ffi::c_int = 22;
pub const XCB_NONE: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const XCB_COPY_FROM_PARENT: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const O_WRONLY: ::core::ffi::c_int = 0o1 as ::core::ffi::c_int;
pub const O_CREAT: ::core::ffi::c_int = 0o100 as ::core::ffi::c_int;
pub const O_TRUNC: ::core::ffi::c_int = 0o1000 as ::core::ffi::c_int;
pub const S_IRUSR: ::core::ffi::c_int = __S_IREAD;
pub const S_IWUSR: ::core::ffi::c_int = __S_IWRITE;
pub const no_argument: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const required_argument: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const _PATH_BSHELL: [::core::ffi::c_char; 8] =
    unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"/bin/sh\0") };
#[no_mangle]
pub static mut visual_type: *mut xcb_visualtype_t =
    ::core::ptr::null::<xcb_visualtype_t>() as *mut xcb_visualtype_t;
#[no_mangle]
pub static mut A__NET_WM_WINDOW_TYPE_DOCK: xcb_atom_t = 0;
#[no_mangle]
pub static mut A_I3_SOCKET_PATH: xcb_atom_t = 0;
#[no_mangle]
pub static mut A__NET_WM_WINDOW_TYPE: xcb_atom_t = 0;
#[no_mangle]
pub static mut A_CARDINAL: xcb_atom_t = 0;
#[no_mangle]
pub static mut A_ATOM: xcb_atom_t = 0;
#[no_mangle]
pub static mut A__NET_WM_STRUT_PARTIAL: xcb_atom_t = 0;
static mut argv0: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
static mut win: xcb_window_t = 0;
static mut bar: surface_t = surface_t {
    id: 0,
    gc: 0,
    owns_gc: false,
    width: 0,
    height: 0,
    surface: ::core::ptr::null::<cairo_surface_t>() as *mut cairo_surface_t,
    cr: ::core::ptr::null::<cairo_t>() as *mut cairo_t,
};
static mut font: i3Font = Font {
    type_0: FONT_TYPE_NONE,
    height: 0,
    pattern: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    specific: C2RustUnnamed {
        xcb: C2RustUnnamed_0 {
            id: 0,
            info: ::core::ptr::null::<xcb_query_font_reply_t>() as *mut xcb_query_font_reply_t,
            table: ::core::ptr::null::<xcb_charinfo_t>() as *mut xcb_charinfo_t,
        },
    },
};
static mut prompt: *mut i3String = ::core::ptr::null::<i3String>() as *mut i3String;
static mut btn_close: button_t = button_t {
    label: ::core::ptr::null::<i3String>() as *mut i3String,
    action: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    x: 0,
    width: 0,
    terminal: false,
};
static mut buttons: *mut button_t = ::core::ptr::null::<button_t>() as *mut button_t;
static mut buttoncnt: ::core::ffi::c_int = 0;
static mut color_background: color_t = color_t {
    red: 0.,
    green: 0.,
    blue: 0.,
    alpha: 0.,
    colorpixel: 0,
};
static mut color_button_background: color_t = color_t {
    red: 0.,
    green: 0.,
    blue: 0.,
    alpha: 0.,
    colorpixel: 0,
};
static mut color_border: color_t = color_t {
    red: 0.,
    green: 0.,
    blue: 0.,
    alpha: 0.,
    colorpixel: 0,
};
static mut color_border_bottom: color_t = color_t {
    red: 0.,
    green: 0.,
    blue: 0.,
    alpha: 0.,
    colorpixel: 0,
};
static mut color_text: color_t = color_t {
    red: 0.,
    green: 0.,
    blue: 0.,
    alpha: 0.,
    colorpixel: 0,
};
#[no_mangle]
pub static mut root: xcb_window_t = 0;
#[no_mangle]
pub static mut conn: *mut xcb_connection_t =
    ::core::ptr::null::<xcb_connection_t>() as *mut xcb_connection_t;
#[no_mangle]
pub static mut root_screen: *mut xcb_screen_t =
    ::core::ptr::null::<xcb_screen_t>() as *mut xcb_screen_t;
#[no_mangle]
pub unsafe extern "C" fn verboselog(mut fmt: *mut ::core::ffi::c_char, mut args: ...) {
    let mut args_0;
    args_0 = args.clone();
    vfprintf(stderr, fmt, i333_va_list!(args_0));
}
#[no_mangle]
pub unsafe extern "C" fn errorlog(mut fmt: *mut ::core::ffi::c_char, mut args: ...) {
    let mut args_0;
    args_0 = args.clone();
    vfprintf(stderr, fmt, i333_va_list!(args_0));
}
#[no_mangle]
pub unsafe extern "C" fn debuglog(mut fmt: *mut ::core::ffi::c_char, mut args: ...) {}
unsafe extern "C" fn start_application(mut command: *const ::core::ffi::c_char) {
    printf(
        b"executing: %s\n\0" as *const u8 as *const ::core::ffi::c_char,
        command,
    );
    if fork() == 0 as ::core::ffi::c_int {
        setsid();
        if fork() == 0 as ::core::ffi::c_int {
            execl(
                _PATH_BSHELL.as_ptr(),
                _PATH_BSHELL.as_ptr(),
                b"-c\0" as *const u8 as *const ::core::ffi::c_char,
                command,
                NULL,
            );
            err(
                EXIT_FAILURE,
                b"execl return\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        exit(0 as ::core::ffi::c_int);
    }
    wait(::core::ptr::null_mut::<::core::ffi::c_int>());
}
unsafe extern "C" fn get_button_at(mut x: int16_t, mut y: int16_t) -> *mut button_t {
    let mut c: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while c < buttoncnt {
        if x as ::core::ffi::c_int >= (*buttons.offset(c as isize)).x as ::core::ffi::c_int
            && x as ::core::ffi::c_int
                <= (*buttons.offset(c as isize)).x as ::core::ffi::c_int
                    + (*buttons.offset(c as isize)).width as ::core::ffi::c_int
        {
            return buttons.offset(c as isize) as *mut button_t;
        }
        c += 1;
    }
    return ::core::ptr::null_mut::<button_t>();
}
unsafe extern "C" fn handle_button_press(
    mut conn_0: *mut xcb_connection_t,
    mut event: *mut xcb_button_press_event_t,
) {
    printf(
        b"button pressed on x = %d, y = %d\n\0" as *const u8 as *const ::core::ffi::c_char,
        (*event).event_x as ::core::ffi::c_int,
        (*event).event_y as ::core::ffi::c_int,
    );
}
unsafe extern "C" fn handle_button_release(
    mut conn_0: *mut xcb_connection_t,
    mut event: *mut xcb_button_release_event_t,
) {
    printf(
        b"button released on x = %d, y = %d\n\0" as *const u8 as *const ::core::ffi::c_char,
        (*event).event_x as ::core::ffi::c_int,
        (*event).event_y as ::core::ffi::c_int,
    );
    if (*event).event_x as ::core::ffi::c_int >= btn_close.x as ::core::ffi::c_int
        && ((*event).event_x as ::core::ffi::c_int)
            < btn_close.x as ::core::ffi::c_int + btn_close.width as ::core::ffi::c_int
    {
        exit(0 as ::core::ffi::c_int);
    }
    let mut button: *mut button_t = get_button_at((*event).event_x, (*event).event_y);
    if button.is_null() {
        return;
    }
    let mut script_path: *mut ::core::ffi::c_char =
        get_process_filename(b"nagbar-cmd\0" as *const u8 as *const ::core::ffi::c_char);
    let mut fd: ::core::ffi::c_int =
        open(script_path, O_WRONLY | O_CREAT | O_TRUNC, S_IRUSR | S_IWUSR);
    if fd == -(1 as ::core::ffi::c_int) {
        warn(
            b"Could not create temporary script to store the nagbar command\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut script: *mut FILE = fdopen(fd, b"w\0" as *const u8 as *const ::core::ffi::c_char);
    if script.is_null() {
        warn(
            b"Could not fdopen() temporary script to store the nagbar command\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    fprintf(
        script,
        b"#!%s\nrm %s\n%s\0" as *const u8 as *const ::core::ffi::c_char,
        _PATH_BSHELL.as_ptr(),
        script_path,
        (*button).action,
    );
    fclose(script);
    let mut link_path: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut exe_path: *mut ::core::ffi::c_char = get_exe_path(argv0);
    sasprintf(
        &raw mut link_path,
        b"%s.nagbar_cmd\0" as *const u8 as *const ::core::ffi::c_char,
        script_path,
    );
    if symlink(exe_path, link_path) == -(1 as ::core::ffi::c_int) {
        err(
            EXIT_FAILURE,
            b"Failed to symlink %s to %s\0" as *const u8 as *const ::core::ffi::c_char,
            link_path,
            exe_path,
        );
    }
    let mut terminal_cmd: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if (*button).terminal {
        sasprintf(
            &raw mut terminal_cmd,
            b"i3-sensible-terminal -e %s\0" as *const u8 as *const ::core::ffi::c_char,
            link_path,
        );
    } else {
        terminal_cmd = sstrdup(link_path);
    }
    printf(
        b"argv0 = %s\n\0" as *const u8 as *const ::core::ffi::c_char,
        argv0,
    );
    printf(
        b"terminal_cmd = %s\n\0" as *const u8 as *const ::core::ffi::c_char,
        terminal_cmd,
    );
    start_application(terminal_cmd);
    free(link_path as *mut ::core::ffi::c_void);
    free(terminal_cmd as *mut ::core::ffi::c_void);
    free(script_path as *mut ::core::ffi::c_void);
    free(exe_path as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn button_draw(
    mut button: *mut button_t,
    mut position: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut text_width: ::core::ffi::c_int = predict_text_width((*button).label);
    (*button).width = (text_width
        + 2 as ::core::ffi::c_int * logical_px(3 as ::core::ffi::c_int)
        + 2 as ::core::ffi::c_int * logical_px(3 as ::core::ffi::c_int))
        as uint16_t;
    (*button).x = (position - (*button).width as ::core::ffi::c_int) as int16_t;
    draw_util_rectangle(
        &raw mut bar,
        color_border,
        (position - (*button).width as ::core::ffi::c_int) as ::core::ffi::c_double,
        (logical_px(8 as ::core::ffi::c_int)
            - logical_px(3 as ::core::ffi::c_int)
            - logical_px(3 as ::core::ffi::c_int)) as ::core::ffi::c_double,
        (*button).width as ::core::ffi::c_double,
        (font.height
            + 2 as ::core::ffi::c_int * logical_px(3 as ::core::ffi::c_int)
            + 2 as ::core::ffi::c_int * logical_px(3 as ::core::ffi::c_int))
            as ::core::ffi::c_double,
    );
    draw_util_rectangle(
        &raw mut bar,
        color_button_background,
        (position - (*button).width as ::core::ffi::c_int + logical_px(3 as ::core::ffi::c_int))
            as ::core::ffi::c_double,
        (logical_px(8 as ::core::ffi::c_int) - logical_px(3 as ::core::ffi::c_int))
            as ::core::ffi::c_double,
        (text_width + 2 as ::core::ffi::c_int * logical_px(3 as ::core::ffi::c_int))
            as ::core::ffi::c_double,
        (font.height + 2 as ::core::ffi::c_int * logical_px(3 as ::core::ffi::c_int))
            as ::core::ffi::c_double,
    );
    draw_util_text(
        (*button).label,
        &raw mut bar,
        color_text,
        color_button_background,
        position - (*button).width as ::core::ffi::c_int
            + logical_px(3 as ::core::ffi::c_int)
            + logical_px(3 as ::core::ffi::c_int),
        logical_px(8 as ::core::ffi::c_int),
        200 as ::core::ffi::c_int,
    );
    return (*button).width as ::core::ffi::c_int;
}
unsafe extern "C" fn handle_expose(mut conn_0: *mut xcb_connection_t) -> ::core::ffi::c_int {
    draw_util_clear_surface(&raw mut bar, color_background);
    draw_util_text(
        prompt,
        &raw mut bar,
        color_text,
        color_background,
        logical_px(8 as ::core::ffi::c_int),
        logical_px(8 as ::core::ffi::c_int),
        bar.width - 2 as ::core::ffi::c_int * logical_px(8 as ::core::ffi::c_int),
    );
    let mut position: ::core::ffi::c_int = bar.width
        - (logical_px(8 as ::core::ffi::c_int)
            - logical_px(3 as ::core::ffi::c_int)
            - logical_px(3 as ::core::ffi::c_int));
    position -= button_draw(&raw mut btn_close, position);
    position -= logical_px(15 as ::core::ffi::c_int);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < buttoncnt {
        position -= logical_px(20 as ::core::ffi::c_int);
        position -= button_draw(buttons.offset(i as isize) as *mut button_t, position);
        i += 1;
    }
    draw_util_rectangle(
        &raw mut bar,
        color_border_bottom,
        0 as ::core::ffi::c_int as ::core::ffi::c_double,
        (bar.height - logical_px(2 as ::core::ffi::c_int)) as ::core::ffi::c_double,
        bar.width as ::core::ffi::c_double,
        logical_px(2 as ::core::ffi::c_int) as ::core::ffi::c_double,
    );
    xcb_flush(conn_0);
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn set_window_position_primary(mut result: *mut xcb_rectangle_t) {
    let mut output: *mut xcb_randr_get_output_info_reply_t =
        ::core::ptr::null_mut::<xcb_randr_get_output_info_reply_t>();
    let mut crtc: *mut xcb_randr_get_crtc_info_reply_t =
        ::core::ptr::null_mut::<xcb_randr_get_crtc_info_reply_t>();
    let mut rcookie: xcb_randr_get_screen_resources_current_cookie_t =
        xcb_randr_get_screen_resources_current(conn, root);
    let mut pcookie: xcb_randr_get_output_primary_cookie_t =
        xcb_randr_get_output_primary(conn, root);
    let mut primary: *mut xcb_randr_get_output_primary_reply_t =
        ::core::ptr::null_mut::<xcb_randr_get_output_primary_reply_t>();
    let mut res: *mut xcb_randr_get_screen_resources_current_reply_t =
        ::core::ptr::null_mut::<xcb_randr_get_screen_resources_current_reply_t>();
    primary = xcb_randr_get_output_primary_reply(
        conn,
        pcookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if primary.is_null() {
        verboselog(
            b"[libi3] ../i3-nagbar/main.c Could not determine the primary output.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    } else {
        res = xcb_randr_get_screen_resources_current_reply(
            conn,
            rcookie,
            ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
        );
        if res.is_null() {
            verboselog(
                b"[libi3] ../i3-nagbar/main.c Could not query screen resources.\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        } else {
            output = xcb_randr_get_output_info_reply(
                conn,
                xcb_randr_get_output_info(conn, (*primary).output, (*res).config_timestamp),
                ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
            );
            if output.is_null() || (*output).crtc as ::core::ffi::c_long == XCB_NONE {
                verboselog(
                    b"[libi3] ../i3-nagbar/main.c Could not query primary screen.\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            } else {
                crtc = xcb_randr_get_crtc_info_reply(
                    conn,
                    xcb_randr_get_crtc_info(conn, (*output).crtc, (*res).config_timestamp),
                    ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
                );
                if crtc.is_null() {
                    verboselog(
                        b"[libi3] ../i3-nagbar/main.c Could not get CRTC.\n\0" as *const u8
                            as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    );
                } else {
                    verboselog(
                        b"[libi3] ../i3-nagbar/main.c Found primary output on position x = %i / y = %i / w = %i / h = %i.\n\0"
                            as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        (*crtc).x as ::core::ffi::c_int,
                        (*crtc).y as ::core::ffi::c_int,
                        (*crtc).width as ::core::ffi::c_int,
                        (*crtc).height as ::core::ffi::c_int,
                    );
                    if (*crtc).width as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                        || (*crtc).height as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    {
                        verboselog(
                            b"[libi3] ../i3-nagbar/main.c Primary output is not active, ignoring it.\n\0"
                                as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                        );
                    } else {
                        (*result).x = (*crtc).x;
                        (*result).y = (*crtc).y;
                    }
                }
            }
        }
    }
    free(res as *mut ::core::ffi::c_void);
    free(primary as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn set_window_position_focus(mut result: *mut xcb_rectangle_t) {
    let mut success: bool = false_0 != 0;
    let mut input_focus: *mut xcb_get_input_focus_reply_t =
        ::core::ptr::null_mut::<xcb_get_input_focus_reply_t>();
    let mut geometry: *mut xcb_get_geometry_reply_t =
        ::core::ptr::null_mut::<xcb_get_geometry_reply_t>();
    let mut coordinates: *mut xcb_translate_coordinates_reply_t =
        ::core::ptr::null_mut::<xcb_translate_coordinates_reply_t>();
    xcb_grab_server(conn);
    input_focus = xcb_get_input_focus_reply(
        conn,
        xcb_get_input_focus(conn),
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if input_focus.is_null() || (*input_focus).focus as ::core::ffi::c_long == XCB_NONE {
        verboselog(
            b"[libi3] ../i3-nagbar/main.c Failed to receive the current input focus or no window has the input focus right now.\n\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    } else {
        geometry = xcb_get_geometry_reply(
            conn,
            xcb_get_geometry(conn, (*input_focus).focus as xcb_drawable_t),
            ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
        );
        if geometry.is_null() {
            verboselog(
                b"[libi3] ../i3-nagbar/main.c Failed to received window geometry.\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        } else {
            coordinates = xcb_translate_coordinates_reply(
                conn,
                xcb_translate_coordinates(
                    conn,
                    (*input_focus).focus,
                    root,
                    (*geometry).x,
                    (*geometry).y,
                ),
                ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
            );
            if coordinates.is_null() {
                verboselog(
                    b"[libi3] ../i3-nagbar/main.c Failed to translate coordinates.\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            } else {
                verboselog(
                    b"[libi3] ../i3-nagbar/main.c Found current focus at x = %i / y = %i.\n\0"
                        as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    (*coordinates).dst_x as ::core::ffi::c_int,
                    (*coordinates).dst_y as ::core::ffi::c_int,
                );
                (*result).x = (*coordinates).dst_x;
                (*result).y = (*coordinates).dst_y;
                success = true_0 != 0;
            }
        }
    }
    xcb_ungrab_server(conn);
    free(input_focus as *mut ::core::ffi::c_void);
    free(coordinates as *mut ::core::ffi::c_void);
    free(geometry as *mut ::core::ffi::c_void);
    if !success {
        verboselog(
            b"[libi3] ../i3-nagbar/main.c Could not position on focused output, trying to position on primary output.\n\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        set_window_position_primary(result);
    }
}
unsafe fn main_0(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut cmd: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let argv0_len: size_t = strlen(*argv.offset(0 as ::core::ffi::c_int as isize)) as size_t;
    if argv0_len > strlen(b".nagbar_cmd\0" as *const u8 as *const ::core::ffi::c_char)
        && strcmp(
            (*argv.offset(0 as ::core::ffi::c_int as isize))
                .offset(argv0_len as isize)
                .offset(
                    -(strlen(b".nagbar_cmd\0" as *const u8 as *const ::core::ffi::c_char) as isize),
                ),
            b".nagbar_cmd\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
    {
        unlink(*argv.offset(0 as ::core::ffi::c_int as isize));
        cmd = sstrdup(*argv.offset(0 as ::core::ffi::c_int as isize));
        *cmd.offset(argv0_len as isize).offset(
            -(strlen(b".nagbar_cmd\0" as *const u8 as *const ::core::ffi::c_char) as isize),
        ) = '\0' as i32 as ::core::ffi::c_char;
        execl(_PATH_BSHELL.as_ptr(), _PATH_BSHELL.as_ptr(), cmd, NULL);
        err(
            EXIT_FAILURE,
            b"execl(%s, %s, %s)\0" as *const u8 as *const ::core::ffi::c_char,
            _PATH_BSHELL.as_ptr(),
            _PATH_BSHELL.as_ptr(),
            cmd,
        );
    }
    argv0 = *argv.offset(0 as ::core::ffi::c_int as isize);
    let mut position_on_primary: bool = false_0 != 0;
    let mut pattern: *mut ::core::ffi::c_char =
        sstrdup(b"pango:monospace 8\0" as *const u8 as *const ::core::ffi::c_char);
    let mut o: ::core::ffi::c_int = 0;
    let mut option_index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut bar_type: C2RustUnnamed_3 = TYPE_ERROR;
    static mut long_options: [option; 9] = [
        option {
            name: b"version\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 'v' as i32,
        },
        option {
            name: b"font\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 'f' as i32,
        },
        option {
            name: b"button\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 'b' as i32,
        },
        option {
            name: b"button-no-terminal\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 'B' as i32,
        },
        option {
            name: b"help\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 'h' as i32,
        },
        option {
            name: b"message\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 'm' as i32,
        },
        option {
            name: b"type\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 't' as i32,
        },
        option {
            name: b"primary\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 'p' as i32,
        },
        option {
            name: ::core::ptr::null::<::core::ffi::c_char>(),
            has_arg: 0 as ::core::ffi::c_int,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 0 as ::core::ffi::c_int,
        },
    ];
    let mut options_string: *mut ::core::ffi::c_char =
        b"b:B:f:m:t:vhp\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
    prompt = i3string_from_utf8(
        b"Please do not run this program.\0" as *const u8 as *const ::core::ffi::c_char,
    );
    loop {
        o = getopt_long(
            argc,
            argv as *const *mut ::core::ffi::c_char,
            options_string,
            &raw mut long_options as *mut option,
            &raw mut option_index,
        );
        if !(o != -(1 as ::core::ffi::c_int)) {
            break;
        }
        match o {
            118 => {
                free(pattern as *mut ::core::ffi::c_void);
                printf(
                    concat!("i3-nagbar ", env!("I3_VERSION"), "\n\0")
                        .as_bytes()
                        .as_ptr() as *const u8 as *const ::core::ffi::c_char,
                );
                return 0 as ::core::ffi::c_int;
            }
            102 => {
                free(pattern as *mut ::core::ffi::c_void);
                pattern = sstrdup(optarg);
            }
            109 => {
                i3string_free(prompt);
                prompt = i3string_from_utf8(optarg);
            }
            116 => {
                bar_type = (if strcasecmp(
                    optarg,
                    b"warning\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    TYPE_WARNING as ::core::ffi::c_int
                } else {
                    TYPE_ERROR as ::core::ffi::c_int
                }) as C2RustUnnamed_3;
            }
            104 => {
                free(pattern as *mut ::core::ffi::c_void);
                printf(
                    concat!("i3-nagbar ", env!("I3_VERSION"), "\n\0")
                        .as_bytes()
                        .as_ptr() as *const u8 as *const ::core::ffi::c_char,
                );
                printf(
                    b"i3-nagbar [-m <message>] [-b <button> <action>] [-B <button> <action>] [-t warning|error] [-f <font>] [-v] [-p]\n\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
                return 0 as ::core::ffi::c_int;
            }
            112 => {
                position_on_primary = true_0 != 0;
            }
            98 | 66 => {
                buttons = srealloc(
                    buttons as *mut ::core::ffi::c_void,
                    (::core::mem::size_of::<button_t>() as size_t)
                        .wrapping_mul((buttoncnt + 1 as ::core::ffi::c_int) as size_t),
                ) as *mut button_t;
                let ref mut fresh0 = (*buttons.offset(buttoncnt as isize)).label;
                *fresh0 = i3string_from_utf8(optarg);
                let ref mut fresh1 = (*buttons.offset(buttoncnt as isize)).action;
                *fresh1 = *argv.offset(optind as isize);
                (*buttons.offset(buttoncnt as isize)).terminal = o == 'b' as i32;
                printf(
                    b"button with label *%s* and action *%s*\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    i3string_as_utf8((*buttons.offset(buttoncnt as isize)).label),
                    (*buttons.offset(buttoncnt as isize)).action,
                );
                buttoncnt += 1;
                printf(
                    b"now %d buttons\n\0" as *const u8 as *const ::core::ffi::c_char,
                    buttoncnt,
                );
                if optind < argc {
                    optind += 1;
                }
            }
            _ => {}
        }
    }
    btn_close.label = i3string_from_utf8(b"X\0" as *const u8 as *const ::core::ffi::c_char);
    let mut screens: ::core::ffi::c_int = 0;
    conn = xcb_connect(::core::ptr::null::<::core::ffi::c_char>(), &raw mut screens);
    if conn.is_null() || xcb_connection_has_error(conn) != 0 {
        errx(
            EXIT_FAILURE,
            b"Cannot open display\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let mut _NET_WM_WINDOW_TYPE_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(b"_NET_WM_WINDOW_TYPE\0" as *const u8 as *const ::core::ffi::c_char) as uint16_t,
        b"_NET_WM_WINDOW_TYPE\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut _NET_WM_WINDOW_TYPE_DOCK_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(b"_NET_WM_WINDOW_TYPE_DOCK\0" as *const u8 as *const ::core::ffi::c_char)
            as uint16_t,
        b"_NET_WM_WINDOW_TYPE_DOCK\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut _NET_WM_STRUT_PARTIAL_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(b"_NET_WM_STRUT_PARTIAL\0" as *const u8 as *const ::core::ffi::c_char) as uint16_t,
        b"_NET_WM_STRUT_PARTIAL\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut I3_SOCKET_PATH_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(b"I3_SOCKET_PATH\0" as *const u8 as *const ::core::ffi::c_char) as uint16_t,
        b"I3_SOCKET_PATH\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut ATOM_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(b"ATOM\0" as *const u8 as *const ::core::ffi::c_char) as uint16_t,
        b"ATOM\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut CARDINAL_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(b"CARDINAL\0" as *const u8 as *const ::core::ffi::c_char) as uint16_t,
        b"CARDINAL\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut sndisplay: *mut SnDisplay = sn_xcb_display_new(conn, None, None);
    let mut sncontext: *mut SnLauncheeContext =
        sn_launchee_context_new_from_environment(sndisplay, screens);
    sn_display_unref(sndisplay);
    root_screen = xcb_aux_get_screen(conn, screens);
    root = (*root_screen).root;
    if bar_type as ::core::ffi::c_uint == TYPE_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint {
        color_button_background =
            draw_util_hex_to_color(b"#680a0a\0" as *const u8 as *const ::core::ffi::c_char);
        color_background =
            draw_util_hex_to_color(b"#900000\0" as *const u8 as *const ::core::ffi::c_char);
        color_text =
            draw_util_hex_to_color(b"#ffffff\0" as *const u8 as *const ::core::ffi::c_char);
        color_border =
            draw_util_hex_to_color(b"#d92424\0" as *const u8 as *const ::core::ffi::c_char);
        color_border_bottom =
            draw_util_hex_to_color(b"#470909\0" as *const u8 as *const ::core::ffi::c_char);
    } else {
        color_button_background =
            draw_util_hex_to_color(b"#ffc100\0" as *const u8 as *const ::core::ffi::c_char);
        color_background =
            draw_util_hex_to_color(b"#ffa800\0" as *const u8 as *const ::core::ffi::c_char);
        color_text =
            draw_util_hex_to_color(b"#000000\0" as *const u8 as *const ::core::ffi::c_char);
        color_border =
            draw_util_hex_to_color(b"#ab7100\0" as *const u8 as *const ::core::ffi::c_char);
        color_border_bottom =
            draw_util_hex_to_color(b"#ab7100\0" as *const u8 as *const ::core::ffi::c_char);
    }
    init_dpi();
    font = load_font(pattern, true_0 != 0);
    set_font(&raw mut font);
    let mut win_pos: xcb_rectangle_t = xcb_rectangle_t {
        x: 50 as int16_t,
        y: 50 as int16_t,
        width: 500 as uint16_t,
        height: (font.height
            + 2 as ::core::ffi::c_int * logical_px(8 as ::core::ffi::c_int)
            + logical_px(2 as ::core::ffi::c_int)) as uint16_t,
    };
    if position_on_primary {
        set_window_position_primary(&raw mut win_pos);
    } else {
        set_window_position_focus(&raw mut win_pos);
    }
    let mut cursor_ctx: *mut xcb_cursor_context_t = ::core::ptr::null_mut::<xcb_cursor_context_t>();
    if xcb_cursor_context_new(conn, root_screen, &raw mut cursor_ctx) < 0 as ::core::ffi::c_int {
        errx(
            EXIT_FAILURE,
            b"Cannot allocate xcursor context\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let mut cursor: xcb_cursor_t = xcb_cursor_load_cursor(
        cursor_ctx,
        b"left_ptr\0" as *const u8 as *const ::core::ffi::c_char,
    );
    xcb_cursor_context_free(cursor_ctx);
    win = xcb_generate_id(conn) as xcb_window_t;
    let mut fresh2: [uint32_t; 3] = [
        0 as ::core::ffi::c_int as uint32_t,
        (XCB_EVENT_MASK_EXPOSURE as ::core::ffi::c_int
            | XCB_EVENT_MASK_STRUCTURE_NOTIFY as ::core::ffi::c_int
            | XCB_EVENT_MASK_BUTTON_PRESS as ::core::ffi::c_int
            | XCB_EVENT_MASK_BUTTON_RELEASE as ::core::ffi::c_int) as uint32_t,
        cursor,
    ];
    xcb_create_window(
        conn,
        XCB_COPY_FROM_PARENT as uint8_t,
        win,
        root,
        win_pos.x,
        win_pos.y,
        win_pos.width,
        win_pos.height,
        0 as uint16_t,
        XCB_WINDOW_CLASS_INPUT_OUTPUT as ::core::ffi::c_int as uint16_t,
        XCB_WINDOW_CLASS_COPY_FROM_PARENT as ::core::ffi::c_int as xcb_visualid_t,
        (XCB_CW_BACK_PIXEL as ::core::ffi::c_int
            | XCB_CW_EVENT_MASK as ::core::ffi::c_int
            | XCB_CW_CURSOR as ::core::ffi::c_int) as uint32_t,
        &raw mut fresh2 as *mut uint32_t as *const ::core::ffi::c_void,
    );
    if !sncontext.is_null() {
        sn_launchee_context_setup_window(sncontext, win as Window);
    }
    xcb_map_window(conn, win);
    let mut reply: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_WM_WINDOW_TYPE_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply.is_null() {
        errx(
            EXIT_FAILURE,
            b"Could not get atom _NET_WM_WINDOW_TYPE\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    A__NET_WM_WINDOW_TYPE = (*reply).atom;
    free(reply as *mut ::core::ffi::c_void);
    let mut reply_0: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_WM_WINDOW_TYPE_DOCK_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_0.is_null() {
        errx(
            EXIT_FAILURE,
            b"Could not get atom _NET_WM_WINDOW_TYPE_DOCK\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    A__NET_WM_WINDOW_TYPE_DOCK = (*reply_0).atom;
    free(reply_0 as *mut ::core::ffi::c_void);
    let mut reply_1: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_WM_STRUT_PARTIAL_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_1.is_null() {
        errx(
            EXIT_FAILURE,
            b"Could not get atom _NET_WM_STRUT_PARTIAL\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    A__NET_WM_STRUT_PARTIAL = (*reply_1).atom;
    free(reply_1 as *mut ::core::ffi::c_void);
    let mut reply_2: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        I3_SOCKET_PATH_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_2.is_null() {
        errx(
            EXIT_FAILURE,
            b"Could not get atom I3_SOCKET_PATH\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    A_I3_SOCKET_PATH = (*reply_2).atom;
    free(reply_2 as *mut ::core::ffi::c_void);
    let mut reply_3: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        ATOM_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_3.is_null() {
        errx(
            EXIT_FAILURE,
            b"Could not get atom ATOM\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    A_ATOM = (*reply_3).atom;
    free(reply_3 as *mut ::core::ffi::c_void);
    let mut reply_4: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        CARDINAL_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_4.is_null() {
        errx(
            EXIT_FAILURE,
            b"Could not get atom CARDINAL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    A_CARDINAL = (*reply_4).atom;
    free(reply_4 as *mut ::core::ffi::c_void);
    xcb_change_property(
        conn,
        XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as uint8_t,
        win,
        A__NET_WM_WINDOW_TYPE,
        A_ATOM,
        32 as uint8_t,
        1 as uint32_t,
        &raw mut A__NET_WM_WINDOW_TYPE_DOCK as *mut ::core::ffi::c_uchar
            as *const ::core::ffi::c_void,
    );
    let mut strut_partial: C2RustUnnamed_2 = C2RustUnnamed_2 {
        left: 0,
        right: 0,
        top: 0,
        bottom: 0,
        left_start_y: 0,
        left_end_y: 0,
        right_start_y: 0,
        right_end_y: 0,
        top_start_x: 0,
        top_end_x: 0,
        bottom_start_x: 0,
        bottom_end_x: 0,
    };
    memset(
        &raw mut strut_partial as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<C2RustUnnamed_2>() as size_t,
    );
    strut_partial.top = (font.height + logical_px(6 as ::core::ffi::c_int)) as uint32_t;
    strut_partial.top_start_x = 0 as uint32_t;
    strut_partial.top_end_x = 800 as uint32_t;
    xcb_change_property(
        conn,
        XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as uint8_t,
        win,
        A__NET_WM_STRUT_PARTIAL,
        A_CARDINAL,
        32 as uint8_t,
        12 as uint32_t,
        &raw mut strut_partial as *const ::core::ffi::c_void,
    );
    draw_util_surface_init(
        conn,
        &raw mut bar,
        win as xcb_drawable_t,
        get_visualtype(root_screen),
        win_pos.width as ::core::ffi::c_int,
        win_pos.height as ::core::ffi::c_int,
    );
    if !sncontext.is_null() {
        sn_launchee_context_complete(sncontext);
        sn_launchee_context_unref(sncontext);
    }
    xcb_flush(conn);
    let mut event: *mut xcb_generic_event_t = ::core::ptr::null_mut::<xcb_generic_event_t>();
    loop {
        event = xcb_wait_for_event(conn);
        if event.is_null() {
            break;
        }
        if (*event).response_type as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            fprintf(
                stderr,
                b"X11 Error received! sequence %x\n\0" as *const u8 as *const ::core::ffi::c_char,
                (*event).sequence as ::core::ffi::c_int,
            );
        } else {
            let mut type_0: ::core::ffi::c_int =
                (*event).response_type as ::core::ffi::c_int & 0x7f as ::core::ffi::c_int;
            match type_0 {
                XCB_EXPOSE => {
                    if (*(event as *mut xcb_expose_event_t)).count as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    {
                        handle_expose(conn);
                    }
                }
                XCB_BUTTON_PRESS => {
                    handle_button_press(conn, event as *mut xcb_button_press_event_t);
                }
                XCB_BUTTON_RELEASE => {
                    handle_button_release(conn, event as *mut xcb_button_release_event_t);
                }
                XCB_CONFIGURE_NOTIFY => {
                    let mut configure_notify: *mut xcb_configure_notify_event_t =
                        event as *mut xcb_configure_notify_event_t;
                    if (*configure_notify).width as ::core::ffi::c_int > 0 as ::core::ffi::c_int
                        && (*configure_notify).height as ::core::ffi::c_int
                            > 0 as ::core::ffi::c_int
                    {
                        draw_util_surface_set_size(
                            &raw mut bar,
                            (*configure_notify).width as ::core::ffi::c_int,
                            (*configure_notify).height as ::core::ffi::c_int,
                        );
                    }
                }
                _ => {}
            }
            free(event as *mut ::core::ffi::c_void);
        }
    }
    free(pattern as *mut ::core::ffi::c_void);
    draw_util_surface_free(conn, &raw mut bar);
    return 0 as ::core::ffi::c_int;
}
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub fn main() {
    let mut args_strings: Vec<Vec<u8>> = ::std::env::args()
        .map(|arg| {
            ::std::ffi::CString::new(arg)
                .expect("Failed to convert argument into CString.")
                .into_bytes_with_nul()
        })
        .collect();
    let mut args_ptrs: Vec<*mut ::core::ffi::c_char> = args_strings
        .iter_mut()
        .map(|arg| arg.as_mut_ptr() as *mut ::core::ffi::c_char)
        .chain(::core::iter::once(::core::ptr::null_mut()))
        .collect();
    unsafe {
        ::std::process::exit(main_0(
            (args_ptrs.len() - 1) as ::core::ffi::c_int,
            args_ptrs.as_mut_ptr() as *mut *mut ::core::ffi::c_char,
        ) as i32)
    }
}
