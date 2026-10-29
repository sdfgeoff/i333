/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * i3-input/main.c: Utility which lets the user input commands and sends them
 *                  to i3.
 *
 */
// Mechanical file-by-file Rust translation of i3-input/main.c.
// Generated with C2Rust 0.22.1; C ABI and original ownership retained.
use ::c2rust_bitfields;
include!("../rust/variadic-compat.rs");

extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type xcb_connection_t;
    pub type _XCBKeySymbols;
    pub type _PangoFontDescription;
    pub type _cairo;
    pub type _cairo_surface;
    pub type _i3String;
    static mut optarg: *mut ::core::ffi::c_char;
    fn getopt_long(
        ___argc: ::core::ffi::c_int,
        ___argv: *const *mut ::core::ffi::c_char,
        __shortopts: *const ::core::ffi::c_char,
        __longopts: *const option,
        __longind: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
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
    fn usleep(__useconds: __useconds_t) -> ::core::ffi::c_int;
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
    fn xcb_aux_get_screen(
        c: *mut xcb_connection_t,
        screen: ::core::ffi::c_int,
    ) -> *mut xcb_screen_t;
    fn xcb_key_symbols_alloc(c: *mut xcb_connection_t) -> *mut xcb_key_symbols_t;
    fn xcb_key_press_lookup_keysym(
        syms: *mut xcb_key_symbols_t,
        event: *mut xcb_key_press_event_t,
        col: ::core::ffi::c_int,
    ) -> xcb_keysym_t;
    fn xcb_is_keypad_key(keysym: xcb_keysym_t) -> ::core::ffi::c_int;
    fn xcb_is_private_keypad_key(keysym: xcb_keysym_t) -> ::core::ffi::c_int;
    fn xcb_is_cursor_key(keysym: xcb_keysym_t) -> ::core::ffi::c_int;
    fn xcb_is_pf_key(keysym: xcb_keysym_t) -> ::core::ffi::c_int;
    fn xcb_is_function_key(keysym: xcb_keysym_t) -> ::core::ffi::c_int;
    fn xcb_is_misc_function_key(keysym: xcb_keysym_t) -> ::core::ffi::c_int;
    fn xcb_is_modifier_key(keysym: xcb_keysym_t) -> ::core::ffi::c_int;
    fn errx(__status: ::core::ffi::c_int, __format: *const ::core::ffi::c_char, ...) -> !;
    fn keysym2ucs(keysym: xcb_keysym_t) -> ::core::ffi::c_long;
    fn scalloc(num: size_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn sstrdup(str: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn sasprintf(
        strp: *mut *mut ::core::ffi::c_char,
        fmt: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn i3string_from_utf8(from_utf8: *const ::core::ffi::c_char) -> *mut i3String;
    fn i3string_from_ucs2(from_ucs2: *const xcb_char2b_t, num_glyphs: size_t) -> *mut i3String;
    fn i3string_free(str: *mut i3String);
    fn ipc_connect(socket_path: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn ipc_send_message(
        sockfd_0: ::core::ffi::c_int,
        message_size: uint32_t,
        message_type: uint32_t,
        payload: *const uint8_t,
    ) -> ::core::ffi::c_int;
    fn load_font(pattern: *const ::core::ffi::c_char, fallback: bool) -> i3Font;
    fn set_font(font_0: *mut i3Font);
    fn convert_ucs2_to_utf8(
        text: *mut xcb_char2b_t,
        num_glyphs: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn predict_text_width(text: *mut i3String) -> ::core::ffi::c_int;
    fn get_visualtype(screen: *mut xcb_screen_t) -> *mut xcb_visualtype_t;
    fn init_dpi();
    fn logical_px(logical: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn draw_util_surface_init(
        conn_0: *mut xcb_connection_t,
        surface_0: *mut surface_t,
        drawable: xcb_drawable_t,
        visual: *mut xcb_visualtype_t,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
    );
    fn draw_util_surface_free(conn_0: *mut xcb_connection_t, surface_0: *mut surface_t);
    fn draw_util_hex_to_color(color: *const ::core::ffi::c_char) -> color_t;
    fn draw_util_text(
        text: *mut i3String,
        surface_0: *mut surface_t,
        fg_color: color_t,
        bg_color: color_t,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
        max_width: ::core::ffi::c_int,
    );
    fn draw_util_rectangle(
        surface_0: *mut surface_t,
        color: color_t,
        x: ::core::ffi::c_double,
        y: ::core::ffi::c_double,
        w: ::core::ffi::c_double,
        h: ::core::ffi::c_double,
    );
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strncat(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn atoi(__nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
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
pub type __useconds_t = ::core::ffi::c_uint;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const ::core::ffi::c_char,
    pub has_arg: ::core::ffi::c_int,
    pub flag: *mut ::core::ffi::c_int,
    pub val: ::core::ffi::c_int,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_char2b_t {
    pub byte1: uint8_t,
    pub byte2: uint8_t,
}
pub type xcb_window_t = uint32_t;
pub type xcb_font_t = uint32_t;
pub type xcb_gcontext_t = uint32_t;
pub type xcb_colormap_t = uint32_t;
pub type xcb_atom_t = uint32_t;
pub type xcb_drawable_t = uint32_t;
pub type xcb_visualid_t = uint32_t;
pub type xcb_timestamp_t = uint32_t;
pub type xcb_keysym_t = uint32_t;
pub type xcb_keycode_t = uint8_t;
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
pub type xcb_mod_mask_t = ::core::ffi::c_uint;
pub const XCB_MOD_MASK_ANY: xcb_mod_mask_t = 32768;
pub const XCB_MOD_MASK_5: xcb_mod_mask_t = 128;
pub const XCB_MOD_MASK_4: xcb_mod_mask_t = 64;
pub const XCB_MOD_MASK_3: xcb_mod_mask_t = 32;
pub const XCB_MOD_MASK_2: xcb_mod_mask_t = 16;
pub const XCB_MOD_MASK_1: xcb_mod_mask_t = 8;
pub const XCB_MOD_MASK_CONTROL: xcb_mod_mask_t = 4;
pub const XCB_MOD_MASK_LOCK: xcb_mod_mask_t = 2;
pub const XCB_MOD_MASK_SHIFT: xcb_mod_mask_t = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_key_press_event_t {
    pub response_type: uint8_t,
    pub detail: xcb_keycode_t,
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
pub type xcb_key_release_event_t = xcb_key_press_event_t;
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
pub type xcb_grab_status_t = ::core::ffi::c_uint;
pub const XCB_GRAB_STATUS_FROZEN: xcb_grab_status_t = 4;
pub const XCB_GRAB_STATUS_NOT_VIEWABLE: xcb_grab_status_t = 3;
pub const XCB_GRAB_STATUS_INVALID_TIME: xcb_grab_status_t = 2;
pub const XCB_GRAB_STATUS_ALREADY_GRABBED: xcb_grab_status_t = 1;
pub const XCB_GRAB_STATUS_SUCCESS: xcb_grab_status_t = 0;
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
pub type xcb_key_symbols_t = _XCBKeySymbols;
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
pub const no_argument: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const required_argument: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const XCB_KEY_PRESS: ::core::ffi::c_int = 2;
pub const XCB_KEY_RELEASE: ::core::ffi::c_int = 3;
pub const XCB_EXPOSE: ::core::ffi::c_int = 12;
pub const XCB_NONE: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const XCB_COPY_FROM_PARENT: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const XCB_CURRENT_TIME: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
#[no_mangle]
pub static mut visual_type: *mut xcb_visualtype_t =
    ::core::ptr::null::<xcb_visualtype_t>() as *mut xcb_visualtype_t;
pub const XK_BackSpace: ::core::ffi::c_int = 0xff08 as ::core::ffi::c_int;
pub const XK_Return: ::core::ffi::c_int = 0xff0d as ::core::ffi::c_int;
pub const XK_Escape: ::core::ffi::c_int = 0xff1b as ::core::ffi::c_int;
pub const XK_Mode_switch: ::core::ffi::c_int = 0xff7e as ::core::ffi::c_int;
#[no_mangle]
pub static mut EXIT_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut EXIT_CANCEL: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[no_mangle]
pub static mut EXIT_ERROR: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
static mut format: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
static mut sockfd: ::core::ffi::c_int = 0;
static mut symbols: *mut xcb_key_symbols_t =
    ::core::ptr::null::<xcb_key_symbols_t>() as *mut xcb_key_symbols_t;
static mut modeswitch_active: bool = false_0 != 0;
static mut win: xcb_window_t = 0;
static mut surface: surface_t = surface_t {
    id: 0,
    gc: 0,
    owns_gc: false,
    width: 0,
    height: 0,
    surface: ::core::ptr::null::<cairo_surface_t>() as *mut cairo_surface_t,
    cr: ::core::ptr::null::<cairo_t>() as *mut cairo_t,
};
static mut glyphs_ucs: [xcb_char2b_t; 512] = [xcb_char2b_t { byte1: 0, byte2: 0 }; 512];
static mut glyphs_utf8: [*mut ::core::ffi::c_char; 512] =
    [::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char; 512];
static mut input_position: ::core::ffi::c_int = 0;
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
static mut prompt_offset: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut limit: ::core::ffi::c_int = 0;
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
    vfprintf(stdout, fmt, i333_va_list!(args_0));
}
#[no_mangle]
pub unsafe extern "C" fn errorlog(mut fmt: *mut ::core::ffi::c_char, mut args: ...) {
    let mut args_0;
    args_0 = args.clone();
    vfprintf(stderr, fmt, i333_va_list!(args_0));
}
#[no_mangle]
pub unsafe extern "C" fn debuglog(mut fmt: *mut ::core::ffi::c_char, mut args: ...) {}
unsafe extern "C" fn concat_strings(
    mut glyphs: *mut *mut ::core::ffi::c_char,
    mut max: ::core::ffi::c_int,
) -> *mut uint8_t {
    let mut output: *mut uint8_t =
        scalloc((max + 1 as ::core::ffi::c_int) as size_t, 4 as size_t) as *mut uint8_t;
    let mut walk: *mut uint8_t = output;
    let mut c: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while c < max {
        printf(
            b"at %c\n\0" as *const u8 as *const ::core::ffi::c_char,
            *(*glyphs.offset(c as isize)).offset(0 as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int,
        );
        if *(*glyphs.offset(c as isize)).offset(0 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            == '\0' as i32
        {
            memcpy(
                walk as *mut ::core::ffi::c_void,
                *glyphs.offset(c as isize) as *const ::core::ffi::c_void,
                2 as size_t,
            );
            walk = walk.offset(2 as ::core::ffi::c_int as isize);
        } else {
            strcpy(walk as *mut ::core::ffi::c_char, *glyphs.offset(c as isize));
            walk = walk.offset(strlen(*glyphs.offset(c as isize)) as isize);
        }
        c += 1;
    }
    printf(
        b"output = %s\n\0" as *const u8 as *const ::core::ffi::c_char,
        output as *mut ::core::ffi::c_char,
    );
    return output;
}
unsafe extern "C" fn handle_expose(mut conn_0: *mut xcb_connection_t) -> ::core::ffi::c_int {
    printf(b"expose!\n\0" as *const u8 as *const ::core::ffi::c_char);
    let mut border_color: color_t =
        draw_util_hex_to_color(b"#FF0000\0" as *const u8 as *const ::core::ffi::c_char);
    let mut fg_color: color_t =
        draw_util_hex_to_color(b"#FFFFFF\0" as *const u8 as *const ::core::ffi::c_char);
    let mut bg_color: color_t =
        draw_util_hex_to_color(b"#000000\0" as *const u8 as *const ::core::ffi::c_char);
    let mut text_offset: ::core::ffi::c_int =
        logical_px(2 as ::core::ffi::c_int) + logical_px(2 as ::core::ffi::c_int);
    draw_util_rectangle(
        &raw mut surface,
        border_color,
        0 as ::core::ffi::c_int as ::core::ffi::c_double,
        0 as ::core::ffi::c_int as ::core::ffi::c_double,
        surface.width as ::core::ffi::c_double,
        surface.height as ::core::ffi::c_double,
    );
    draw_util_rectangle(
        &raw mut surface,
        bg_color,
        logical_px(2 as ::core::ffi::c_int) as ::core::ffi::c_double,
        logical_px(2 as ::core::ffi::c_int) as ::core::ffi::c_double,
        (surface.width - 2 as ::core::ffi::c_int * logical_px(2 as ::core::ffi::c_int))
            as ::core::ffi::c_double,
        (surface.height - 2 as ::core::ffi::c_int * logical_px(2 as ::core::ffi::c_int))
            as ::core::ffi::c_double,
    );
    if !prompt.is_null() {
        draw_util_text(
            prompt,
            &raw mut surface,
            fg_color,
            bg_color,
            text_offset,
            text_offset,
            logical_px(500 as ::core::ffi::c_int) - text_offset,
        );
    }
    if input_position > 0 as ::core::ffi::c_int {
        let mut input: *mut i3String = i3string_from_ucs2(
            &raw mut glyphs_ucs as *mut xcb_char2b_t,
            input_position as size_t,
        );
        draw_util_text(
            input,
            &raw mut surface,
            fg_color,
            bg_color,
            text_offset + prompt_offset,
            text_offset,
            logical_px(500 as ::core::ffi::c_int) - text_offset,
        );
        i3string_free(input);
    }
    xcb_flush(conn_0);
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn handle_key_release(
    mut ignored: *mut ::core::ffi::c_void,
    mut conn_0: *mut xcb_connection_t,
    mut event: *mut xcb_key_release_event_t,
) -> ::core::ffi::c_int {
    printf(
        b"releasing %d, state raw = %d\n\0" as *const u8 as *const ::core::ffi::c_char,
        (*event).detail as ::core::ffi::c_int,
        (*event).state as ::core::ffi::c_int,
    );
    let mut sym: xcb_keysym_t = xcb_key_press_lookup_keysym(
        symbols,
        event as *mut xcb_key_press_event_t,
        (*event).state as ::core::ffi::c_int,
    );
    if sym == XK_Mode_switch as xcb_keysym_t {
        printf(b"Mode switch disabled\n\0" as *const u8 as *const ::core::ffi::c_char);
        modeswitch_active = false_0 != 0;
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn finish_input() {
    let mut command: *mut ::core::ffi::c_char = concat_strings(
        &raw mut glyphs_utf8 as *mut *mut ::core::ffi::c_char,
        input_position,
    ) as *mut ::core::ffi::c_char;
    let len: size_t = strlen(format) as size_t;
    let mut cnt: size_t = 0 as size_t;
    let mut c: size_t = 0 as size_t;
    while c < len.wrapping_sub(1 as size_t) {
        if *format.offset(c as isize) as ::core::ffi::c_int == '%' as i32
            && *format.offset(c.wrapping_add(1 as size_t) as isize) as ::core::ffi::c_int
                == 's' as i32
        {
            cnt = cnt.wrapping_add(1);
        }
        c = c.wrapping_add(1);
    }
    printf(
        b"occurrences = %zu\n\0" as *const u8 as *const ::core::ffi::c_char,
        cnt,
    );
    let input_len: size_t = strlen(command) as size_t;
    let full_len: size_t = if input_len
        > strlen(format)
            .wrapping_sub((2 as size_t).wrapping_mul(cnt))
            .wrapping_add(input_len.wrapping_mul(cnt))
            .wrapping_add(1 as size_t)
    {
        input_len
    } else {
        (strlen(format) as size_t)
            .wrapping_sub((2 as size_t).wrapping_mul(cnt))
            .wrapping_add(input_len.wrapping_mul(cnt))
            .wrapping_add(1 as size_t)
    };
    let mut full: *mut ::core::ffi::c_char =
        scalloc(full_len, 1 as size_t) as *mut ::core::ffi::c_char;
    let mut dest: *mut ::core::ffi::c_char = full;
    let mut c_0: size_t = 0 as size_t;
    while c_0 < len {
        if *format.offset(c_0 as isize) as ::core::ffi::c_int != '%' as i32
            || c_0 == len.wrapping_sub(1 as size_t)
            || *format.offset(c_0.wrapping_add(1 as size_t) as isize) as ::core::ffi::c_int
                != 's' as i32
        {
            let fresh0 = dest;
            dest = dest.offset(1);
            *fresh0 = *format.offset(c_0 as isize);
        } else {
            strncat(dest, command, input_len);
            dest = dest.offset(input_len as isize);
            c_0 = c_0.wrapping_add(1);
        }
        c_0 = c_0.wrapping_add(1);
    }
    printf(
        b"command = %s\n\0" as *const u8 as *const ::core::ffi::c_char,
        full,
    );
    let mut ret: ::core::ffi::c_int = ipc_send_message(
        sockfd,
        strlen(full) as uint32_t,
        0 as uint32_t,
        full as *mut uint8_t,
    );
    free(full as *mut ::core::ffi::c_void);
    exit(if ret == 0 as ::core::ffi::c_int {
        EXIT_OK
    } else {
        EXIT_ERROR
    });
}
unsafe extern "C" fn handle_key_press(
    mut ignored: *mut ::core::ffi::c_void,
    mut conn_0: *mut xcb_connection_t,
    mut event: *mut xcb_key_press_event_t,
) -> ::core::ffi::c_int {
    printf(
        b"Keypress %d, state raw = %d\n\0" as *const u8 as *const ::core::ffi::c_char,
        (*event).detail as ::core::ffi::c_int,
        (*event).state as ::core::ffi::c_int,
    );
    let mut col: ::core::ffi::c_int =
        (*event).state as ::core::ffi::c_int & XCB_MOD_MASK_SHIFT as ::core::ffi::c_int;
    if modeswitch_active {
        col += 2 as ::core::ffi::c_int;
    }
    let mut sym: xcb_keysym_t = xcb_key_press_lookup_keysym(symbols, event, col);
    if sym == XK_Mode_switch as xcb_keysym_t {
        printf(b"Mode switch enabled\n\0" as *const u8 as *const ::core::ffi::c_char);
        modeswitch_active = true_0 != 0;
        return 1 as ::core::ffi::c_int;
    }
    if sym == XK_Return as xcb_keysym_t {
        finish_input();
    }
    if sym == XK_BackSpace as xcb_keysym_t {
        if input_position == 0 as ::core::ffi::c_int {
            return 1 as ::core::ffi::c_int;
        }
        input_position -= 1;
        free(glyphs_utf8[input_position as usize] as *mut ::core::ffi::c_void);
        handle_expose(conn_0);
        return 1 as ::core::ffi::c_int;
    }
    if sym == XK_Escape as xcb_keysym_t {
        exit(EXIT_CANCEL);
    }
    printf(
        b"is_keypad_key = %d\n\0" as *const u8 as *const ::core::ffi::c_char,
        xcb_is_keypad_key(sym),
    );
    printf(
        b"is_private_keypad_key = %d\n\0" as *const u8 as *const ::core::ffi::c_char,
        xcb_is_private_keypad_key(sym),
    );
    printf(
        b"xcb_is_cursor_key = %d\n\0" as *const u8 as *const ::core::ffi::c_char,
        xcb_is_cursor_key(sym),
    );
    printf(
        b"xcb_is_pf_key = %d\n\0" as *const u8 as *const ::core::ffi::c_char,
        xcb_is_pf_key(sym),
    );
    printf(
        b"xcb_is_function_key = %d\n\0" as *const u8 as *const ::core::ffi::c_char,
        xcb_is_function_key(sym),
    );
    printf(
        b"xcb_is_misc_function_key = %d\n\0" as *const u8 as *const ::core::ffi::c_char,
        xcb_is_misc_function_key(sym),
    );
    printf(
        b"xcb_is_modifier_key = %d\n\0" as *const u8 as *const ::core::ffi::c_char,
        xcb_is_modifier_key(sym),
    );
    if xcb_is_modifier_key(sym) != 0 || xcb_is_cursor_key(sym) != 0 {
        return 1 as ::core::ffi::c_int;
    }
    printf(
        b"sym = %c (%d)\n\0" as *const u8 as *const ::core::ffi::c_char,
        sym,
        sym,
    );
    let mut ucs: uint16_t = keysym2ucs(sym) as uint16_t;
    if ucs as int16_t as ::core::ffi::c_int == -(1 as ::core::ffi::c_int) {
        fprintf(
            stderr,
            b"Keysym could not be converted to UCS, skipping\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 1 as ::core::ffi::c_int;
    }
    let mut inp: xcb_char2b_t = xcb_char2b_t { byte1: 0, byte2: 0 };
    inp.byte1 = ((ucs as ::core::ffi::c_int & 0xff00 as ::core::ffi::c_int)
        >> 2 as ::core::ffi::c_int) as uint8_t;
    inp.byte2 = ((ucs as ::core::ffi::c_int & 0xff as ::core::ffi::c_int)
        >> 0 as ::core::ffi::c_int) as uint8_t;
    printf(
        b"inp.byte1 = %02x, inp.byte2 = %02x\n\0" as *const u8 as *const ::core::ffi::c_char,
        inp.byte1 as ::core::ffi::c_int,
        inp.byte2 as ::core::ffi::c_int,
    );
    let mut out: *mut ::core::ffi::c_char = convert_ucs2_to_utf8(&raw mut inp, 1 as size_t);
    printf(
        b"converted to %s\n\0" as *const u8 as *const ::core::ffi::c_char,
        out,
    );
    glyphs_ucs[input_position as usize] = inp;
    glyphs_utf8[input_position as usize] = out;
    input_position += 1;
    if input_position == limit {
        finish_input();
    }
    handle_expose(conn_0);
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn get_window_position() -> xcb_rectangle_t {
    let mut result: xcb_rectangle_t = xcb_rectangle_t {
        x: logical_px(50 as ::core::ffi::c_int) as int16_t,
        y: logical_px(50 as ::core::ffi::c_int) as int16_t,
        width: logical_px(500 as ::core::ffi::c_int) as uint16_t,
        height: (font.height
            + 2 as ::core::ffi::c_int * logical_px(2 as ::core::ffi::c_int)
            + 2 as ::core::ffi::c_int * logical_px(2 as ::core::ffi::c_int))
            as uint16_t,
    };
    let mut supporting_wm_reply: *mut xcb_get_property_reply_t =
        ::core::ptr::null_mut::<xcb_get_property_reply_t>();
    let mut input_focus: *mut xcb_get_input_focus_reply_t =
        ::core::ptr::null_mut::<xcb_get_input_focus_reply_t>();
    let mut geometry: *mut xcb_get_geometry_reply_t =
        ::core::ptr::null_mut::<xcb_get_geometry_reply_t>();
    let mut wm_class: *mut xcb_get_property_reply_t =
        ::core::ptr::null_mut::<xcb_get_property_reply_t>();
    let mut coordinates: *mut xcb_translate_coordinates_reply_t =
        ::core::ptr::null_mut::<xcb_translate_coordinates_reply_t>();
    let mut A__NET_SUPPORTING_WM_CHECK: xcb_atom_t = 0;
    let mut nswc_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(b"_NET_SUPPORTING_WM_CHECK\0" as *const u8 as *const ::core::ffi::c_char)
            as uint16_t,
        b"_NET_SUPPORTING_WM_CHECK\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut nswc_reply: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        nswc_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if nswc_reply.is_null() {
        errorlog(
            b"[libi3] ERROR: Could not intern atom _NET_SUPPORTING_WM_CHECK\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        exit(EXIT_ERROR);
    }
    A__NET_SUPPORTING_WM_CHECK = (*nswc_reply).atom;
    free(nswc_reply as *mut ::core::ffi::c_void);
    supporting_wm_reply = xcb_get_property_reply(
        conn,
        xcb_get_property(
            conn,
            false_0 as uint8_t,
            root,
            A__NET_SUPPORTING_WM_CHECK,
            XCB_ATOM_WINDOW as ::core::ffi::c_int as xcb_atom_t,
            0 as uint32_t,
            32 as uint32_t,
        ),
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    let mut supporting_wm_win: *mut xcb_window_t = ::core::ptr::null_mut::<xcb_window_t>();
    if supporting_wm_reply.is_null()
        || xcb_get_property_value_length(supporting_wm_reply) == 0 as ::core::ffi::c_int
    {
        debuglog(
            b"%s:%s:%d - Could not determine EWMH support window.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../i3-input/main.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"get_window_position\0" as *const u8 as *const ::core::ffi::c_char,
            325 as ::core::ffi::c_int,
        );
    } else {
        supporting_wm_win = xcb_get_property_value(supporting_wm_reply) as *mut xcb_window_t;
    }
    xcb_grab_server(conn);
    input_focus = xcb_get_input_focus_reply(
        conn,
        xcb_get_input_focus(conn),
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if input_focus.is_null() || (*input_focus).focus as ::core::ffi::c_long == XCB_NONE {
        debuglog(
            b"%s:%s:%d - Failed to receive the current input focus or no window has the input focus right now.\n\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../i3-input/main.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"get_window_position\0" as *const u8 as *const ::core::ffi::c_char,
            336 as ::core::ffi::c_int,
        );
    } else if !supporting_wm_win.is_null() && (*input_focus).focus == *supporting_wm_win {
        debuglog(
            b"%s:%s:%d - Input focus is on the EWMH support window, ignoring.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../i3-input/main.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"get_window_position\0" as *const u8 as *const ::core::ffi::c_char,
            342 as ::core::ffi::c_int,
        );
    } else {
        geometry = xcb_get_geometry_reply(
            conn,
            xcb_get_geometry(conn, (*input_focus).focus as xcb_drawable_t),
            ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
        );
        if geometry.is_null() {
            debuglog(
                b"%s:%s:%d - Failed to received window geometry.\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"../i3-input/main.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"get_window_position\0" as *const u8 as *const ::core::ffi::c_char,
                348 as ::core::ffi::c_int,
            );
        } else {
            wm_class = xcb_get_property_reply(
                conn,
                xcb_get_property(
                    conn,
                    false_0 as uint8_t,
                    (*input_focus).focus,
                    XCB_ATOM_WM_CLASS as ::core::ffi::c_int as xcb_atom_t,
                    XCB_GET_PROPERTY_TYPE_ANY as ::core::ffi::c_int as xcb_atom_t,
                    0 as uint32_t,
                    32 as uint32_t,
                ),
                ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
            );
            if wm_class.is_null()
                || xcb_get_property_value_length(wm_class) == 0 as ::core::ffi::c_int
                || strcmp(
                    xcb_get_property_value(wm_class) as *const ::core::ffi::c_char,
                    b"i3-frame\0" as *const u8 as *const ::core::ffi::c_char,
                ) != 0 as ::core::ffi::c_int
            {
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
                    debuglog(
                        b"%s:%s:%d - Failed to translate coordinates.\n\0" as *const u8
                            as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        b"../i3-input/main.c\0" as *const u8 as *const ::core::ffi::c_char,
                        b"get_window_position\0" as *const u8 as *const ::core::ffi::c_char,
                        360 as ::core::ffi::c_int,
                    );
                } else {
                    debuglog(
                        b"%s:%s:%d - Determined coordinates of window with input focus at x = %i / y = %i.\n\0"
                            as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        b"../i3-input/main.c\0" as *const u8
                            as *const ::core::ffi::c_char,
                        b"get_window_position\0" as *const u8
                            as *const ::core::ffi::c_char,
                        364 as ::core::ffi::c_int,
                        (*coordinates).dst_x as ::core::ffi::c_int,
                        (*coordinates).dst_y as ::core::ffi::c_int,
                    );
                    result.x = (result.x as ::core::ffi::c_int
                        + (*coordinates).dst_x as ::core::ffi::c_int)
                        as int16_t;
                    result.y = (result.y as ::core::ffi::c_int
                        + (*coordinates).dst_y as ::core::ffi::c_int)
                        as int16_t;
                }
            } else {
                debuglog(
                    b"%s:%s:%d - Determined coordinates of window with input focus at x = %i / y = %i.\n\0"
                        as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    b"../i3-input/main.c\0" as *const u8 as *const ::core::ffi::c_char,
                    b"get_window_position\0" as *const u8 as *const ::core::ffi::c_char,
                    368 as ::core::ffi::c_int,
                    (*geometry).x as ::core::ffi::c_int,
                    (*geometry).y as ::core::ffi::c_int,
                );
                result.x = (result.x as ::core::ffi::c_int + (*geometry).x as ::core::ffi::c_int)
                    as int16_t;
                result.y = (result.y as ::core::ffi::c_int + (*geometry).y as ::core::ffi::c_int)
                    as int16_t;
            }
        }
    }
    xcb_ungrab_server(conn);
    xcb_flush(conn);
    free(supporting_wm_reply as *mut ::core::ffi::c_void);
    supporting_wm_reply = ::core::ptr::null_mut::<xcb_get_property_reply_t>();
    free(input_focus as *mut ::core::ffi::c_void);
    input_focus = ::core::ptr::null_mut::<xcb_get_input_focus_reply_t>();
    free(geometry as *mut ::core::ffi::c_void);
    geometry = ::core::ptr::null_mut::<xcb_get_geometry_reply_t>();
    free(wm_class as *mut ::core::ffi::c_void);
    wm_class = ::core::ptr::null_mut::<xcb_get_property_reply_t>();
    free(coordinates as *mut ::core::ffi::c_void);
    coordinates = ::core::ptr::null_mut::<xcb_translate_coordinates_reply_t>();
    return result;
}
unsafe fn main_0(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut socket_path: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut pattern: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut o: ::core::ffi::c_int = 0;
    let mut option_index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    static mut long_options: [option; 9] = [
        option {
            name: b"socket\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 's' as i32,
        },
        option {
            name: b"version\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 'v' as i32,
        },
        option {
            name: b"limit\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 'l' as i32,
        },
        option {
            name: b"prompt\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 'P' as i32,
        },
        option {
            name: b"prefix\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 'p' as i32,
        },
        option {
            name: b"format\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 'F' as i32,
        },
        option {
            name: b"font\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 'f' as i32,
        },
        option {
            name: b"help\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 'h' as i32,
        },
        option {
            name: ::core::ptr::null::<::core::ffi::c_char>(),
            has_arg: 0 as ::core::ffi::c_int,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 0 as ::core::ffi::c_int,
        },
    ];
    let mut options_string: *mut ::core::ffi::c_char =
        b"s:p:P:f:l:F:vh\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
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
            115 => {
                free(socket_path as *mut ::core::ffi::c_void);
                socket_path = ::core::ptr::null_mut::<::core::ffi::c_char>();
                socket_path = sstrdup(optarg);
            }
            118 => {
                printf(
                    concat!("i3-input ", env!("I3_VERSION"), "\n\0")
                        .as_bytes()
                        .as_ptr() as *const u8 as *const ::core::ffi::c_char,
                );
                return EXIT_OK;
            }
            112 => {
                fprintf(
                    stderr,
                    b"i3-input: WARNING: the -p option is DEPRECATED in favor of the -F (format) option\n\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
                free(format as *mut ::core::ffi::c_void);
                format = ::core::ptr::null_mut::<::core::ffi::c_char>();
                sasprintf(
                    &raw mut format,
                    b"%s%%s\0" as *const u8 as *const ::core::ffi::c_char,
                    optarg,
                );
            }
            108 => {
                limit = atoi(optarg);
            }
            80 => {
                i3string_free(prompt);
                prompt = i3string_from_utf8(optarg);
            }
            102 => {
                free(pattern as *mut ::core::ffi::c_void);
                pattern = ::core::ptr::null_mut::<::core::ffi::c_char>();
                pattern = sstrdup(optarg);
            }
            70 => {
                free(format as *mut ::core::ffi::c_void);
                format = ::core::ptr::null_mut::<::core::ffi::c_char>();
                format = sstrdup(optarg);
            }
            104 => {
                printf(
                    concat!("i3-input ", env!("I3_VERSION"), "\n\0")
                        .as_bytes()
                        .as_ptr() as *const u8 as *const ::core::ffi::c_char,
                );
                printf(
                    b"i3-input [-s <socket>] [-F <format>] [-l <limit>] [-P <prompt>] [-f <font>] [-v]\n\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
                printf(b"\n\0" as *const u8 as *const ::core::ffi::c_char);
                printf(b"Example:\n\0" as *const u8 as *const ::core::ffi::c_char);
                printf(
                    b"    i3-input -F 'workspace \"%%s\"' -P 'Switch to workspace: '\n\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
                return EXIT_OK;
            }
            _ => {}
        }
    }
    if format.is_null() {
        format = b"%s\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
    }
    printf(
        b"using format \"%s\"\n\0" as *const u8 as *const ::core::ffi::c_char,
        format,
    );
    let mut screen: ::core::ffi::c_int = 0;
    conn = xcb_connect(::core::ptr::null::<::core::ffi::c_char>(), &raw mut screen);
    if conn.is_null() || xcb_connection_has_error(conn) != 0 {
        errx(
            EXIT_FAILURE,
            b"Cannot open display\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    sockfd = ipc_connect(socket_path);
    root_screen = xcb_aux_get_screen(conn, screen);
    root = (*root_screen).root;
    symbols = xcb_key_symbols_alloc(conn);
    init_dpi();
    font = load_font(
        if !pattern.is_null() {
            pattern as *const ::core::ffi::c_char
        } else {
            b"pango:monospace 8\0" as *const u8 as *const ::core::ffi::c_char
        },
        true_0 != 0,
    );
    set_font(&raw mut font);
    if !prompt.is_null() {
        prompt_offset = predict_text_width(prompt);
    }
    let win_pos: xcb_rectangle_t = get_window_position() as xcb_rectangle_t;
    win = xcb_generate_id(conn) as xcb_window_t;
    let mut fresh1: [uint32_t; 3] = [
        0 as ::core::ffi::c_int as uint32_t,
        1 as ::core::ffi::c_int as uint32_t,
        XCB_EVENT_MASK_EXPOSURE as ::core::ffi::c_int as uint32_t,
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
            | XCB_CW_OVERRIDE_REDIRECT as ::core::ffi::c_int
            | XCB_CW_EVENT_MASK as ::core::ffi::c_int) as uint32_t,
        &raw mut fresh1 as *mut uint32_t as *const ::core::ffi::c_void,
    );
    xcb_map_window(conn, win);
    draw_util_surface_init(
        conn,
        &raw mut surface,
        win as xcb_drawable_t,
        get_visualtype(root_screen),
        win_pos.width as ::core::ffi::c_int,
        win_pos.height as ::core::ffi::c_int,
    );
    xcb_flush(conn);
    let mut cookie: xcb_grab_keyboard_cookie_t = xcb_grab_keyboard_cookie_t { sequence: 0 };
    let mut reply: *mut xcb_grab_keyboard_reply_t =
        ::core::ptr::null_mut::<xcb_grab_keyboard_reply_t>();
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while (reply.is_null()
        || (*reply).status as ::core::ffi::c_int != XCB_GRAB_STATUS_SUCCESS as ::core::ffi::c_int)
        && {
            let fresh2 = count;
            count = count + 1;
            fresh2 < 500 as ::core::ffi::c_int
        }
    {
        cookie = xcb_grab_keyboard(
            conn,
            false_0 as uint8_t,
            win,
            XCB_CURRENT_TIME as xcb_timestamp_t,
            XCB_GRAB_MODE_ASYNC as ::core::ffi::c_int as uint8_t,
            XCB_GRAB_MODE_ASYNC as ::core::ffi::c_int as uint8_t,
        );
        reply = xcb_grab_keyboard_reply(
            conn,
            cookie,
            ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
        );
        usleep(1000 as __useconds_t);
    }
    if (*reply).status as ::core::ffi::c_int != XCB_GRAB_STATUS_SUCCESS as ::core::ffi::c_int {
        fprintf(
            stderr,
            b"Could not grab keyboard, status = %d\n\0" as *const u8 as *const ::core::ffi::c_char,
            (*reply).status as ::core::ffi::c_int,
        );
        exit(EXIT_ERROR);
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
            free(event as *mut ::core::ffi::c_void);
        } else {
            let type_0: ::core::ffi::c_int =
                (*event).response_type as ::core::ffi::c_int & 0x7f as ::core::ffi::c_int;
            match type_0 {
                XCB_KEY_PRESS => {
                    handle_key_press(NULL_0, conn, event as *mut xcb_key_press_event_t);
                }
                XCB_KEY_RELEASE => {
                    handle_key_release(NULL_0, conn, event as *mut xcb_key_release_event_t);
                }
                XCB_EXPOSE => {
                    if (*(event as *mut xcb_expose_event_t)).count as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    {
                        handle_expose(conn);
                    }
                }
                _ => {}
            }
            free(event as *mut ::core::ffi::c_void);
        }
    }
    draw_util_surface_free(conn, &raw mut surface);
    return EXIT_OK;
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
