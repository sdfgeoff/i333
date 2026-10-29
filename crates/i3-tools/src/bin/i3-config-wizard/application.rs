/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * i3-config-wizard: Program to convert configs using keycodes to configs using
 *                   keysyms.
 *
 */
// Mechanical file-by-file Rust translation of i3-config-wizard/main.c.
// Generated with C2Rust 0.22.1; C ABI and original ownership retained.
use ::c2rust_bitfields;
use i3_common::i333_va_list;

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
    pub type xkb_context;
    pub type xkb_keymap;
    pub type SnDisplay;
    pub type SnLauncheeContext;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fflush(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
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
    fn fputc(__c: ::core::ffi::c_int, __stream: *mut FILE) -> ::core::ffi::c_int;
    fn getline(
        __lineptr: *mut *mut ::core::ffi::c_char,
        __n: *mut size_t,
        __stream: *mut FILE,
    ) -> __ssize_t;
    fn fputs(__s: *const ::core::ffi::c_char, __stream: *mut FILE) -> ::core::ffi::c_int;
    fn fileno(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn stat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
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
    fn xcb_get_modifier_mapping(c: *mut xcb_connection_t) -> xcb_get_modifier_mapping_cookie_t;
    fn xcb_get_modifier_mapping_keycodes(
        R: *const xcb_get_modifier_mapping_reply_t,
    ) -> *mut xcb_keycode_t;
    fn xcb_get_modifier_mapping_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_get_modifier_mapping_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_get_modifier_mapping_reply_t;
    fn xcb_flush(c: *mut xcb_connection_t) -> ::core::ffi::c_int;
    fn xcb_wait_for_event(c: *mut xcb_connection_t) -> *mut xcb_generic_event_t;
    fn xcb_get_setup(c: *mut xcb_connection_t) -> *const xcb_setup_t;
    fn xcb_connection_has_error(c: *mut xcb_connection_t) -> ::core::ffi::c_int;
    fn xcb_connect(
        displayname: *const ::core::ffi::c_char,
        screenp: *mut ::core::ffi::c_int,
    ) -> *mut xcb_connection_t;
    fn xcb_generate_id(c: *mut xcb_connection_t) -> uint32_t;
    fn xcb_key_symbols_alloc(c: *mut xcb_connection_t) -> *mut xcb_key_symbols_t;
    fn xcb_key_symbols_get_keysym(
        syms: *mut xcb_key_symbols_t,
        keycode: xcb_keycode_t,
        col: ::core::ffi::c_int,
    ) -> xcb_keysym_t;
    fn xcb_key_press_lookup_keysym(
        syms: *mut xcb_key_symbols_t,
        event: *mut xcb_key_press_event_t,
        col: ::core::ffi::c_int,
    ) -> xcb_keysym_t;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn usleep(__useconds: __useconds_t) -> ::core::ffi::c_int;
    fn unlink(__name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    static mut optarg: *mut ::core::ffi::c_char;
    fn fsync(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn scalloc(num: size_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn sstrdup(str: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn sasprintf(
        strp: *mut *mut ::core::ffi::c_char,
        fmt: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn i3string_from_utf8(from_utf8: *const ::core::ffi::c_char) -> *mut i3String;
    fn i3string_free(str: *mut i3String);
    fn ipc_connect(socket_path_0: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn ipc_send_message(
        sockfd: ::core::ffi::c_int,
        message_size: uint32_t,
        message_type: uint32_t,
        payload: *const uint8_t,
    ) -> ::core::ffi::c_int;
    fn get_mod_mask_for(
        keysym: uint32_t,
        symbols_0: *mut xcb_key_symbols_t,
        modmap_reply_0: *mut xcb_get_modifier_mapping_reply_t,
    ) -> uint32_t;
    fn load_font(pattern: *const ::core::ffi::c_char, fallback: bool) -> i3Font;
    fn set_font(font_0: *mut i3Font);
    fn predict_text_width(text: *mut i3String) -> ::core::ffi::c_int;
    fn get_visualtype(screen: *mut xcb_screen_t) -> *mut xcb_visualtype_t;
    fn init_dpi();
    fn logical_px(logical: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn resolve_tilde(path: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn get_config_path(
        override_configpath: *const ::core::ffi::c_char,
        use_system_paths: bool,
    ) -> *mut ::core::ffi::c_char;
    fn mkdirp(path: *const ::core::ffi::c_char, mode: mode_t) -> ::core::ffi::c_int;
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
    fn draw_util_clear_surface(surface_0: *mut surface_t, color: color_t);
    fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
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
    fn xcb_aux_get_screen(
        c: *mut xcb_connection_t,
        screen: ::core::ffi::c_int,
    ) -> *mut xcb_screen_t;
    fn xkb_keysym_get_name(
        keysym: xkb_keysym_t,
        buffer: *mut ::core::ffi::c_char,
        size: size_t,
    ) -> ::core::ffi::c_int;
    fn xkb_context_new(flags: xkb_context_flags) -> *mut xkb_context;
    fn xkb_keymap_key_get_syms_by_level(
        keymap: *mut xkb_keymap,
        key: xkb_keycode_t,
        layout: xkb_layout_index_t,
        level: xkb_level_index_t,
        syms_out: *mut *const xkb_keysym_t,
    ) -> ::core::ffi::c_int;
    fn xkb_x11_setup_xkb_extension(
        connection: *mut xcb_connection_t,
        major_xkb_version: uint16_t,
        minor_xkb_version: uint16_t,
        flags: xkb_x11_setup_xkb_extension_flags,
        major_xkb_version_out: *mut uint16_t,
        minor_xkb_version_out: *mut uint16_t,
        base_event_out: *mut uint8_t,
        base_error_out: *mut uint8_t,
    ) -> ::core::ffi::c_int;
    fn xkb_x11_get_core_keyboard_device_id(connection: *mut xcb_connection_t) -> int32_t;
    fn xkb_x11_keymap_new_from_device(
        context: *mut xkb_context,
        connection: *mut xcb_connection_t,
        device_id: int32_t,
        flags: xkb_keymap_compile_flags,
    ) -> *mut xkb_keymap;
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
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    fn strncasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn atoi(__nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn getenv(__name: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
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
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type __dev_t = ::core::ffi::c_ulong;
pub type __uid_t = ::core::ffi::c_uint;
pub type __gid_t = ::core::ffi::c_uint;
pub type __ino_t = ::core::ffi::c_ulong;
pub type __mode_t = ::core::ffi::c_uint;
pub type __nlink_t = ::core::ffi::c_ulong;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __time_t = ::core::ffi::c_long;
pub type __useconds_t = ::core::ffi::c_uint;
pub type __blksize_t = ::core::ffi::c_long;
pub type __blkcnt_t = ::core::ffi::c_long;
pub type __ssize_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type mode_t = __mode_t;
pub type ssize_t = isize;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
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
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: ::core::ffi::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
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
pub type xcb_font_t = uint32_t;
pub type xcb_gcontext_t = uint32_t;
pub type xcb_colormap_t = uint32_t;
pub type xcb_atom_t = uint32_t;
pub type xcb_drawable_t = uint32_t;
pub type xcb_visualid_t = uint32_t;
pub type xcb_timestamp_t = uint32_t;
pub type xcb_keysym_t = uint32_t;
pub type xcb_keycode_t = uint8_t;
pub type xcb_button_t = uint8_t;
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
pub struct xcb_setup_t {
    pub status: uint8_t,
    pub pad0: uint8_t,
    pub protocol_major_version: uint16_t,
    pub protocol_minor_version: uint16_t,
    pub length: uint16_t,
    pub release_number: uint32_t,
    pub resource_id_base: uint32_t,
    pub resource_id_mask: uint32_t,
    pub motion_buffer_size: uint32_t,
    pub vendor_len: uint16_t,
    pub maximum_request_length: uint16_t,
    pub roots_len: uint8_t,
    pub pixmap_formats_len: uint8_t,
    pub image_byte_order: uint8_t,
    pub bitmap_format_bit_order: uint8_t,
    pub bitmap_format_scanline_unit: uint8_t,
    pub bitmap_format_scanline_pad: uint8_t,
    pub min_keycode: xcb_keycode_t,
    pub max_keycode: xcb_keycode_t,
    pub pad1: [uint8_t; 4],
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
pub struct xcb_get_modifier_mapping_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_get_modifier_mapping_reply_t {
    pub response_type: uint8_t,
    pub keycodes_per_modifier: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub pad0: [uint8_t; 24],
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
pub type C2RustUnnamed_2 = ::core::ffi::c_uint;
pub const _ISalnum: C2RustUnnamed_2 = 8;
pub const _ISpunct: C2RustUnnamed_2 = 4;
pub const _IScntrl: C2RustUnnamed_2 = 2;
pub const _ISblank: C2RustUnnamed_2 = 1;
pub const _ISgraph: C2RustUnnamed_2 = 32768;
pub const _ISprint: C2RustUnnamed_2 = 16384;
pub const _ISspace: C2RustUnnamed_2 = 8192;
pub const _ISxdigit: C2RustUnnamed_2 = 4096;
pub const _ISdigit: C2RustUnnamed_2 = 2048;
pub const _ISalpha: C2RustUnnamed_2 = 1024;
pub const _ISlower: C2RustUnnamed_2 = 512;
pub const _ISupper: C2RustUnnamed_2 = 256;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const ::core::ffi::c_char,
    pub has_arg: ::core::ffi::c_int,
    pub flag: *mut ::core::ffi::c_int,
    pub val: ::core::ffi::c_int,
}
pub type xkb_keycode_t = uint32_t;
pub type xkb_keysym_t = uint32_t;
pub type xkb_layout_index_t = uint32_t;
pub type xkb_level_index_t = uint32_t;
pub type xkb_context_flags = ::core::ffi::c_uint;
pub const XKB_CONTEXT_NO_SECURE_GETENV: xkb_context_flags = 4;
pub const XKB_CONTEXT_NO_ENVIRONMENT_NAMES: xkb_context_flags = 2;
pub const XKB_CONTEXT_NO_DEFAULT_INCLUDES: xkb_context_flags = 1;
pub const XKB_CONTEXT_NO_FLAGS: xkb_context_flags = 0;
pub type xkb_keymap_compile_flags = ::core::ffi::c_uint;
pub const XKB_KEYMAP_COMPILE_NO_FLAGS: xkb_keymap_compile_flags = 0;
pub type xkb_x11_setup_xkb_extension_flags = ::core::ffi::c_uint;
pub const XKB_X11_SETUP_XKB_EXTENSION_NO_FLAGS: xkb_x11_setup_xkb_extension_flags = 0;
pub type XID = ::core::ffi::c_ulong;
pub type Window = XID;
pub type KeySym = XID;
pub type SnXcbDisplayErrorTrapPush =
    Option<unsafe extern "C" fn(*mut SnDisplay, *mut xcb_connection_t) -> ()>;
pub type SnXcbDisplayErrorTrapPop =
    Option<unsafe extern "C" fn(*mut SnDisplay, *mut xcb_connection_t) -> ()>;
pub type C2RustUnnamed_3 = ::core::ffi::c_uint;
pub const STEP_GENERATE: C2RustUnnamed_3 = 1;
pub const STEP_WELCOME: C2RustUnnamed_3 = 0;
pub type C2RustUnnamed_4 = ::core::ffi::c_uint;
pub const MOD_Mod4: C2RustUnnamed_4 = 1;
pub const MOD_Mod1: C2RustUnnamed_4 = 0;
pub type cmdp_token = token;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct token {
    pub name: *mut ::core::ffi::c_char,
    pub identifier: *mut ::core::ffi::c_char,
    pub next_state: cmdp_state,
    pub extra: C2RustUnnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub call_identifier: uint16_t,
}
pub type cmdp_state = ::core::ffi::c_uint;
pub const __CALL: cmdp_state = 117;
pub const BAR: cmdp_state = 116;
pub const MODE: cmdp_state = 115;
pub const GAPS: cmdp_state = 114;
pub const FONT: cmdp_state = 113;
pub const EXEC: cmdp_state = 112;
pub const BAR_ID: cmdp_state = 111;
pub const ASSIGN: cmdp_state = 110;
pub const INITIAL: cmdp_state = 109;
pub const INCLUDE: cmdp_state = 108;
pub const BINDING: cmdp_state = 107;
pub const NO_FOCUS: cmdp_state = 106;
pub const MODENAME: cmdp_state = 105;
pub const GAPS_END: cmdp_state = 104;
pub const CRITERIA: cmdp_state = 103;
pub const BAR_MODE: cmdp_state = 102;
pub const BAR_FONT: cmdp_state = 101;
pub const BARBRACE: cmdp_state = 100;
pub const WORKSPACE: cmdp_state = 99;
pub const MODEBRACE: cmdp_state = 98;
pub const CRITERION: cmdp_state = 97;
pub const SMART_GAPS: cmdp_state = 96;
pub const SHOW_MARKS: cmdp_state = 95;
pub const IPC_SOCKET: cmdp_state = 94;
pub const FOR_WINDOW: cmdp_state = 93;
pub const COLOR_TEXT: cmdp_state = 92;
pub const BAR_OUTPUT: cmdp_state = 91;
pub const BAR_HEIGHT: cmdp_state = 90;
pub const BAR_COLORS: cmdp_state = 89;
pub const TITLE_ALIGN: cmdp_state = 88;
pub const TILING_DRAG: cmdp_state = 87;
pub const IGNORE_LINE: cmdp_state = 86;
pub const BINDCOMMAND: cmdp_state = 85;
pub const BAR_VERBOSE: cmdp_state = 84;
pub const BAR_PADDING: cmdp_state = 83;
pub const BAR_BINDSYM: cmdp_state = 82;
pub const NO_FOCUS_END: cmdp_state = 81;
pub const MODE_BINDING: cmdp_state = 80;
pub const FAKE_OUTPUTS: cmdp_state = 79;
pub const COLOR_SINGLE: cmdp_state = 78;
pub const COLOR_BORDER: cmdp_state = 77;
pub const BAR_POSITION: cmdp_state = 76;
pub const BAR_MODIFIER: cmdp_state = 75;
pub const SMART_BORDERS: cmdp_state = 74;
pub const RESTART_STATE: cmdp_state = 73;
pub const MOUSE_WARPING: cmdp_state = 72;
pub const CRITERION_STR: cmdp_state = 71;
pub const ASSIGN_OUTPUT: cmdp_state = 70;
pub const FORCE_XINERAMA: cmdp_state = 69;
pub const FOCUS_WRAPPING: cmdp_state = 68;
pub const DEFAULT_BORDER: cmdp_state = 67;
pub const CRITERION_FROM: cmdp_state = 66;
pub const GAPS_WITH_SCOPE: cmdp_state = 65;
pub const DISABLE_RANDR15: cmdp_state = 64;
pub const COLOR_INDICATOR: cmdp_state = 63;
pub const BAR_TRAY_OUTPUT: cmdp_state = 62;
pub const BAR_SOCKET_PATH: cmdp_state = 61;
pub const BAR_PADDING_TOP: cmdp_state = 60;
pub const BAR_IGNORE_LINE: cmdp_state = 59;
pub const BAR_COLORS_TEXT: cmdp_state = 58;
pub const BAR_BAR_COMMAND: cmdp_state = 57;
pub const WORKSPACE_LAYOUT: cmdp_state = 56;
pub const TILING_DRAG_MODE: cmdp_state = 55;
pub const MODE_IGNORE_LINE: cmdp_state = 54;
pub const MODE_BINDCOMMAND: cmdp_state = 53;
pub const IPC_KILL_TIMEOUT: cmdp_state = 52;
pub const COLOR_BACKGROUND: cmdp_state = 51;
pub const BAR_WHEEL_UP_CMD: cmdp_state = 50;
pub const BAR_TRAY_PADDING: cmdp_state = 49;
pub const BAR_PADDING_LEFT: cmdp_state = 48;
pub const BAR_HIDDEN_STATE: cmdp_state = 47;
pub const BAR_COLORS_BRACE: cmdp_state = 46;
pub const ASSIGN_WORKSPACE: cmdp_state = 45;
pub const WORKSPACE_COMMAND: cmdp_state = 44;
pub const HIDE_EDGE_BORDERS: cmdp_state = 43;
pub const FLOATING_MODIFIER: cmdp_state = 42;
pub const BAR_PADDING_RIGHT: cmdp_state = 41;
pub const BAR_COLORS_SINGLE: cmdp_state = 40;
pub const BAR_COLORS_BORDER: cmdp_state = 39;
pub const FOR_WINDOW_COMMAND: cmdp_state = 38;
pub const CRITERION_FROM_STR: cmdp_state = 37;
pub const COLOR_CHILD_BORDER: cmdp_state = 36;
pub const BAR_WHEEL_DOWN_CMD: cmdp_state = 35;
pub const BAR_STATUS_COMMAND: cmdp_state = 34;
pub const BAR_PADDING_BOTTOM: cmdp_state = 33;
pub const FOCUS_FOLLOWS_MOUSE: cmdp_state = 32;
pub const DEFAULT_ORIENTATION: cmdp_state = 31;
pub const BAR_TRAY_PADDING_PX: cmdp_state = 30;
pub const BAR_BINDSYM_COMMAND: cmdp_state = 29;
pub const FORCE_FOCUS_WRAPPING: cmdp_state = 28;
pub const BAR_SEPARATOR_SYMBOL: cmdp_state = 27;
pub const WORKSPACE_OUTPUT_WORD: cmdp_state = 26;
pub const DEFAULT_BORDER_PIXELS: cmdp_state = 25;
pub const BAR_WORKSPACE_COMMAND: cmdp_state = 24;
pub const BAR_WORKSPACE_BUTTONS: cmdp_state = 23;
pub const BAR_COLORS_BACKGROUND: cmdp_state = 22;
pub const CRITERION_FROM_STR_END: cmdp_state = 21;
pub const BAR_COLORS_IGNORE_LINE: cmdp_state = 20;
pub const POPUP_DURING_FULLSCREEN: cmdp_state = 19;
pub const FLOATING_MINIMUM_SIZE_X: cmdp_state = 18;
pub const FLOATING_MAXIMUM_SIZE_X: cmdp_state = 17;
pub const BAR_WORKSPACE_MIN_WIDTH: cmdp_state = 16;
pub const ASSIGN_WORKSPACE_NUMBER: cmdp_state = 15;
pub const WORKSPACE_BACK_AND_FORTH: cmdp_state = 14;
pub const DEFAULT_BORDER_PIXELS_PX: cmdp_state = 13;
pub const CRITERION_FROM_STR_START: cmdp_state = 12;
pub const BAR_STRIP_WORKSPACE_NAME: cmdp_state = 11;
pub const TILING_DRAG_SWAP_MODIFIER: cmdp_state = 10;
pub const FORCE_DISPLAY_URGENCY_HINT: cmdp_state = 9;
pub const FOCUS_ON_WINDOW_ACTIVATION: cmdp_state = 8;
pub const BAR_WORKSPACE_MIN_WIDTH_PX: cmdp_state = 7;
pub const BAR_BINDING_MODE_INDICATOR: cmdp_state = 6;
pub const FLOATING_MINIMUM_SIZE_WIDTH: cmdp_state = 5;
pub const FLOATING_MAXIMUM_SIZE_WIDTH: cmdp_state = 4;
pub const BAR_STRIP_WORKSPACE_NUMBERS: cmdp_state = 3;
pub const FLOATING_MINIMUM_SIZE_HEIGHT: cmdp_state = 2;
pub const FLOATING_MAXIMUM_SIZE_HEIGHT: cmdp_state = 1;
pub const FORCE_DISPLAY_URGENCY_HINT_MS: cmdp_state = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub str_0: *mut ::core::ffi::c_char,
    pub num: ::core::ffi::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack_entry {
    pub identifier: *const ::core::ffi::c_char,
    pub type_0: C2RustUnnamed_7,
    pub val: C2RustUnnamed_6,
}
pub type C2RustUnnamed_7 = ::core::ffi::c_uint;
pub const STACK_LONG: C2RustUnnamed_7 = 1;
pub const STACK_STR: C2RustUnnamed_7 = 0;
pub type cmdp_token_ptr = tokenptr;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tokenptr {
    pub array: *mut cmdp_token,
    pub n: ::core::ffi::c_int,
}
pub const __S_IREAD: ::core::ffi::c_int = 0o400 as ::core::ffi::c_int;
pub const __S_IWRITE: ::core::ffi::c_int = 0o200 as ::core::ffi::c_int;
pub const __S_IEXEC: ::core::ffi::c_int = 0o100 as ::core::ffi::c_int;
pub const XCB_KEY_PRESS: ::core::ffi::c_int = 2;
pub const XCB_BUTTON_PRESS: ::core::ffi::c_int = 4;
pub const XCB_EXPOSE: ::core::ffi::c_int = 12;
pub const XCB_NONE: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const XCB_COPY_FROM_PARENT: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const XCB_CURRENT_TIME: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const LONG_MAX: ::core::ffi::c_long = __LONG_MAX__;
pub const LONG_MIN: ::core::ffi::c_long = -__LONG_MAX__ - 1 as ::core::ffi::c_long;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const ERANGE: ::core::ffi::c_int = 34 as ::core::ffi::c_int;
pub const DEFAULT_DIR_MODE: ::core::ffi::c_int = S_IRWXU | S_IRGRP | S_IXGRP | S_IROTH | S_IXOTH;
pub const O_RDWR: ::core::ffi::c_int = 0o2 as ::core::ffi::c_int;
pub const O_CREAT: ::core::ffi::c_int = 0o100 as ::core::ffi::c_int;
pub const S_IRUSR: ::core::ffi::c_int = __S_IREAD;
pub const S_IXUSR: ::core::ffi::c_int = __S_IEXEC;
pub const S_IRWXU: ::core::ffi::c_int = __S_IREAD | __S_IWRITE | __S_IEXEC;
pub const S_IRGRP: ::core::ffi::c_int = S_IRUSR >> 3 as ::core::ffi::c_int;
pub const S_IXGRP: ::core::ffi::c_int = S_IXUSR >> 3 as ::core::ffi::c_int;
pub const S_IROTH: ::core::ffi::c_int = S_IRGRP >> 3 as ::core::ffi::c_int;
pub const S_IXOTH: ::core::ffi::c_int = S_IXGRP >> 3 as ::core::ffi::c_int;
pub const no_argument: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const required_argument: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const XKB_X11_MIN_MAJOR_XKB_VERSION: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const XKB_X11_MIN_MINOR_XKB_VERSION: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const XK_Return: ::core::ffi::c_int = 0xff0d as ::core::ffi::c_int;
pub const XK_Escape: ::core::ffi::c_int = 0xff1b as ::core::ffi::c_int;
pub const XK_Up: ::core::ffi::c_int = 0xff52 as ::core::ffi::c_int;
pub const XK_Down: ::core::ffi::c_int = 0xff54 as ::core::ffi::c_int;
pub const XK_KP_Enter: ::core::ffi::c_int = 0xff8d as ::core::ffi::c_int;
pub const XCB_NUM_LOCK: ::core::ffi::c_int = 0xff7f as ::core::ffi::c_int;
#[no_mangle]
pub static mut A__NET_WM_WINDOW_TYPE: xcb_atom_t = 0;
#[no_mangle]
pub static mut A_UTF8_STRING: xcb_atom_t = 0;
#[no_mangle]
pub static mut A_ATOM: xcb_atom_t = 0;
#[no_mangle]
pub static mut A_CARDINAL: xcb_atom_t = 0;
#[no_mangle]
pub static mut A__NET_WM_NAME: xcb_atom_t = 0;
#[no_mangle]
pub static mut A__NET_WM_WINDOW_TYPE_DIALOG: xcb_atom_t = 0;
#[no_mangle]
pub static mut visual_type: *mut xcb_visualtype_t =
    ::core::ptr::null::<xcb_visualtype_t>() as *mut xcb_visualtype_t;
#[no_mangle]
pub static mut current_step: C2RustUnnamed_3 = STEP_WELCOME;
#[no_mangle]
pub static mut modifier: C2RustUnnamed_4 = MOD_Mod4;
static mut config_path: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
static mut xcb_numlock_mask: uint32_t = 0;
#[no_mangle]
pub static mut conn: *mut xcb_connection_t =
    ::core::ptr::null::<xcb_connection_t>() as *mut xcb_connection_t;
static mut keysyms: *mut xcb_key_symbols_t =
    ::core::ptr::null::<xcb_key_symbols_t>() as *mut xcb_key_symbols_t;
#[no_mangle]
pub static mut root_screen: *mut xcb_screen_t =
    ::core::ptr::null::<xcb_screen_t>() as *mut xcb_screen_t;
static mut modmap_reply: *mut xcb_get_modifier_mapping_reply_t =
    ::core::ptr::null::<xcb_get_modifier_mapping_reply_t>()
        as *mut xcb_get_modifier_mapping_reply_t;
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
static mut bold_font: i3Font = Font {
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
static mut char_width: ::core::ffi::c_int = 0;
static mut socket_path: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
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
static mut symbols: *mut xcb_key_symbols_t =
    ::core::ptr::null::<xcb_key_symbols_t>() as *mut xcb_key_symbols_t;
#[no_mangle]
pub static mut root: xcb_window_t = 0;
static mut xkb_keymap: *mut xkb_keymap = ::core::ptr::null::<xkb_keymap>() as *mut xkb_keymap;
static mut xkb_base_event: uint8_t = 0;
static mut xkb_base_error: uint8_t = 0;
static mut tokens_FORCE_DISPLAY_URGENCY_HINT_MS: [cmdp_token; 2] = [
    token {
        name: b"'ms\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: FORCE_DISPLAY_URGENCY_HINT_MS,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"end\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_FLOATING_MAXIMUM_SIZE_HEIGHT: [cmdp_token; 1] = [token {
    name: b"number\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"height\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 1 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_FLOATING_MINIMUM_SIZE_HEIGHT: [cmdp_token; 1] = [token {
    name: b"number\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"height\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 2 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_STRIP_WORKSPACE_NUMBERS: [cmdp_token; 1] = [token {
    name: b"word\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"value\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 3 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_FLOATING_MAXIMUM_SIZE_WIDTH: [cmdp_token; 1] = [token {
    name: b"number\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"width\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: FLOATING_MAXIMUM_SIZE_X,
    extra: C2RustUnnamed_5 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_FLOATING_MINIMUM_SIZE_WIDTH: [cmdp_token; 1] = [token {
    name: b"number\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"width\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: FLOATING_MINIMUM_SIZE_X,
    extra: C2RustUnnamed_5 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_BINDING_MODE_INDICATOR: [cmdp_token; 1] = [token {
    name: b"word\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"value\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 4 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_WORKSPACE_MIN_WIDTH_PX: [cmdp_token; 2] = [
    token {
        name: b"'px\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_WORKSPACE_MIN_WIDTH_PX,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"end\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 5 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_FOCUS_ON_WINDOW_ACTIVATION: [cmdp_token; 1] = [token {
    name: b"word\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"mode\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 6 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_FORCE_DISPLAY_URGENCY_HINT: [cmdp_token; 1] = [token {
    name: b"number\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"duration_ms\0" as *const u8 as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    next_state: FORCE_DISPLAY_URGENCY_HINT_MS,
    extra: C2RustUnnamed_5 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_TILING_DRAG_SWAP_MODIFIER: [cmdp_token; 10] = [
    token {
        name: b"'Mod1\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: TILING_DRAG_SWAP_MODIFIER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Mod2\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: TILING_DRAG_SWAP_MODIFIER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Mod3\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: TILING_DRAG_SWAP_MODIFIER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Mod4\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: TILING_DRAG_SWAP_MODIFIER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Mod5\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: TILING_DRAG_SWAP_MODIFIER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Shift\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: TILING_DRAG_SWAP_MODIFIER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Control\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: TILING_DRAG_SWAP_MODIFIER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Ctrl\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: TILING_DRAG_SWAP_MODIFIER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'+\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: TILING_DRAG_SWAP_MODIFIER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"end\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 7 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_BAR_STRIP_WORKSPACE_NAME: [cmdp_token; 1] = [token {
    name: b"word\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"value\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 8 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_CRITERION_FROM_STR_START: [cmdp_token; 3] = [
    token {
        name: b"'\"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CRITERION_FROM_STR,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'auto\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"kind\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 9 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'user\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"kind\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 10 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_DEFAULT_BORDER_PIXELS_PX: [cmdp_token; 2] = [
    token {
        name: b"'px\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: DEFAULT_BORDER_PIXELS_PX,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"end\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 11 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_WORKSPACE_BACK_AND_FORTH: [cmdp_token; 1] = [token {
    name: b"word\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"value\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 12 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_ASSIGN_WORKSPACE_NUMBER: [cmdp_token; 1] = [token {
    name: b"string\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"number\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 13 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_WORKSPACE_MIN_WIDTH: [cmdp_token; 1] = [token {
    name: b"number\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"width\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: BAR_WORKSPACE_MIN_WIDTH_PX,
    extra: C2RustUnnamed_5 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_FLOATING_MAXIMUM_SIZE_X: [cmdp_token; 1] = [token {
    name: b"'x\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: FLOATING_MAXIMUM_SIZE_HEIGHT,
    extra: C2RustUnnamed_5 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_FLOATING_MINIMUM_SIZE_X: [cmdp_token; 1] = [token {
    name: b"'x\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: FLOATING_MINIMUM_SIZE_HEIGHT,
    extra: C2RustUnnamed_5 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_POPUP_DURING_FULLSCREEN: [cmdp_token; 4] = [
    token {
        name: b"'ignore\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"value\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 14 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'leave_fullscreen\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"value\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 15 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'all\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"value\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 16 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'smart\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"value\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 17 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_BAR_COLORS_IGNORE_LINE: [cmdp_token; 1] = [token {
    name: b"line\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: BAR_COLORS,
    extra: C2RustUnnamed_5 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_CRITERION_FROM_STR_END: [cmdp_token; 1] = [token {
    name: b"'\"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 18 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_COLORS_BACKGROUND: [cmdp_token; 1] = [token {
    name: b"word\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"background\0" as *const u8 as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    next_state: BAR_COLORS_TEXT,
    extra: C2RustUnnamed_5 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_WORKSPACE_BUTTONS: [cmdp_token; 1] = [token {
    name: b"word\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"value\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 19 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_WORKSPACE_COMMAND: [cmdp_token; 1] = [token {
    name: b"string\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"command\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 20 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_DEFAULT_BORDER_PIXELS: [cmdp_token; 2] = [
    token {
        name: b"end\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 21 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"number\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"width\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: DEFAULT_BORDER_PIXELS_PX,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_WORKSPACE_OUTPUT_WORD: [cmdp_token; 2] = [
    token {
        name: b"word\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"output\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 22 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"end\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: INITIAL,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_BAR_SEPARATOR_SYMBOL: [cmdp_token; 1] = [token {
    name: b"string\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"separator\0" as *const u8 as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 23 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_FORCE_FOCUS_WRAPPING: [cmdp_token; 1] = [token {
    name: b"word\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"value\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 24 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_BINDSYM_COMMAND: [cmdp_token; 2] = [
    token {
        name: b"'--release\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"release\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BAR_BINDSYM_COMMAND,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"string\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"command\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 25 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_BAR_TRAY_PADDING_PX: [cmdp_token; 2] = [
    token {
        name: b"'px\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_TRAY_PADDING_PX,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"end\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 26 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_DEFAULT_ORIENTATION: [cmdp_token; 3] = [
    token {
        name: b"'horizontal\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"orientation\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 27 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'vertical\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"orientation\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 28 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'auto\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"orientation\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 29 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_FOCUS_FOLLOWS_MOUSE: [cmdp_token; 1] = [token {
    name: b"word\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"value\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 30 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_PADDING_BOTTOM: [cmdp_token; 3] = [
    token {
        name: b"'px\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_PADDING_BOTTOM,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"number\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"left\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BAR_PADDING_LEFT,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"end\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 31 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_BAR_STATUS_COMMAND: [cmdp_token; 1] = [token {
    name: b"string\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"command\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 32 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_WHEEL_DOWN_CMD: [cmdp_token; 1] = [token {
    name: b"string\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"command\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 33 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_COLOR_CHILD_BORDER: [cmdp_token; 2] = [
    token {
        name: b"word\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"child_border\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 34 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"end\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 35 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_CRITERION_FROM_STR: [cmdp_token; 2] = [
    token {
        name: b"'auto\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"kind\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: CRITERION_FROM_STR_END,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'user\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"kind\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: CRITERION_FROM_STR_END,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_FOR_WINDOW_COMMAND: [cmdp_token; 1] = [token {
    name: b"string\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"command\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 36 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_COLORS_BORDER: [cmdp_token; 1] = [token {
    name: b"word\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"border\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: BAR_COLORS_BACKGROUND,
    extra: C2RustUnnamed_5 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_COLORS_SINGLE: [cmdp_token; 1] = [token {
    name: b"word\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"color\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 37 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_PADDING_RIGHT: [cmdp_token; 3] = [
    token {
        name: b"'px\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_PADDING_RIGHT,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"number\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"bottom\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BAR_PADDING_BOTTOM,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"end\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 38 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_FLOATING_MODIFIER: [cmdp_token; 10] = [
    token {
        name: b"'Mod1\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: FLOATING_MODIFIER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Mod2\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: FLOATING_MODIFIER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Mod3\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: FLOATING_MODIFIER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Mod4\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: FLOATING_MODIFIER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Mod5\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: FLOATING_MODIFIER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Shift\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: FLOATING_MODIFIER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Control\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: FLOATING_MODIFIER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Ctrl\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: FLOATING_MODIFIER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'+\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: FLOATING_MODIFIER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"end\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 39 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_HIDE_EDGE_BORDERS: [cmdp_token; 12] = [
    token {
        name: b"'none\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"hide_borders\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 40 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'vertical\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"hide_borders\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 41 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'horizontal\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"hide_borders\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 42 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'both\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"hide_borders\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 43 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'smart_no_gaps\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"hide_borders\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 44 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'smart\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"hide_borders\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 45 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'1\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"hide_borders\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 46 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'yes\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"hide_borders\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 47 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'true\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"hide_borders\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 48 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'on\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"hide_borders\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 49 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'enable\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"hide_borders\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 50 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'active\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"hide_borders\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 51 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_WORKSPACE_COMMAND: [cmdp_token; 2] = [
    token {
        name: b"'output\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: WORKSPACE_OUTPUT_WORD,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'gaps\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: GAPS,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_ASSIGN_WORKSPACE: [cmdp_token; 5] = [
    token {
        name: b"'\xE2\x86\x92\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ASSIGN_WORKSPACE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'output\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ASSIGN_OUTPUT,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'workspace\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ASSIGN_WORKSPACE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'number\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ASSIGN_WORKSPACE_NUMBER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"string\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"workspace\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 52 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_BAR_COLORS_BRACE: [cmdp_token; 2] = [
    token {
        name: b"end\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_COLORS_BRACE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'{\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_COLORS,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_BAR_HIDDEN_STATE: [cmdp_token; 2] = [
    token {
        name: b"'hide\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"hidden_state\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 53 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'show\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"hidden_state\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 54 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_BAR_PADDING_LEFT: [cmdp_token; 2] = [
    token {
        name: b"'px\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_PADDING_LEFT,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"end\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 55 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_BAR_TRAY_PADDING: [cmdp_token; 1] = [token {
    name: b"number\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"padding_px\0" as *const u8 as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    next_state: BAR_TRAY_PADDING_PX,
    extra: C2RustUnnamed_5 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_WHEEL_UP_CMD: [cmdp_token; 1] = [token {
    name: b"string\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"command\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 56 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_COLOR_BACKGROUND: [cmdp_token; 1] = [token {
    name: b"word\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"background\0" as *const u8 as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    next_state: COLOR_TEXT,
    extra: C2RustUnnamed_5 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_IPC_KILL_TIMEOUT: [cmdp_token; 1] = [token {
    name: b"number\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"timeout\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 57 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_MODE_BINDCOMMAND: [cmdp_token; 5] = [
    token {
        name: b"'--release\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"release\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: MODE_BINDCOMMAND,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'--border\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"border\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: MODE_BINDCOMMAND,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'--whole-window\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"whole_window\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: MODE_BINDCOMMAND,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'--exclude-titlebar\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"exclude_titlebar\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: MODE_BINDCOMMAND,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"string\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"command\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 58 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_MODE_IGNORE_LINE: [cmdp_token; 1] = [token {
    name: b"line\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: MODE,
    extra: C2RustUnnamed_5 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_TILING_DRAG_MODE: [cmdp_token; 3] = [
    token {
        name: b"'modifier\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"value\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: TILING_DRAG_MODE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'titlebar\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"value\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: TILING_DRAG_MODE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"end\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 59 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_WORKSPACE_LAYOUT: [cmdp_token; 4] = [
    token {
        name: b"'default\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"layout\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 60 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'stacking\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"layout\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 61 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'stacked\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"layout\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 62 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'tabbed\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"layout\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 63 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_BAR_BAR_COMMAND: [cmdp_token; 1] = [token {
    name: b"string\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"command\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 64 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_COLORS_TEXT: [cmdp_token; 2] = [
    token {
        name: b"end\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 65 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"word\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"text\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 66 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_BAR_IGNORE_LINE: [cmdp_token; 1] = [token {
    name: b"line\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: BAR,
    extra: C2RustUnnamed_5 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_PADDING_TOP: [cmdp_token; 3] = [
    token {
        name: b"'px\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_PADDING_TOP,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"number\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"right_or_right_and_left\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BAR_PADDING_RIGHT,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"end\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 67 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_BAR_SOCKET_PATH: [cmdp_token; 1] = [token {
    name: b"string\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"path\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 68 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_TRAY_OUTPUT: [cmdp_token; 1] = [token {
    name: b"word\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"output\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 69 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_COLOR_INDICATOR: [cmdp_token; 2] = [
    token {
        name: b"word\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"indicator\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: COLOR_CHILD_BORDER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"end\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 70 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_DISABLE_RANDR15: [cmdp_token; 1] = [token {
    name: b"word\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"value\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 71 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_GAPS_WITH_SCOPE: [cmdp_token; 1] = [token {
    name: b"number\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"value\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: GAPS_END,
    extra: C2RustUnnamed_5 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_CRITERION_FROM: [cmdp_token; 1] = [token {
    name: b"'=\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: CRITERION_FROM_STR_START,
    extra: C2RustUnnamed_5 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_DEFAULT_BORDER: [cmdp_token; 4] = [
    token {
        name: b"'normal\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"border\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: DEFAULT_BORDER_PIXELS,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'pixel\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"border\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: DEFAULT_BORDER_PIXELS,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'1pixel\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"border\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 72 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'none\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"border\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 73 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_FOCUS_WRAPPING: [cmdp_token; 14] = [
    token {
        name: b"'1\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"value\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 74 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'yes\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"value\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 75 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'true\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"value\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 76 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'on\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"value\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 77 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'enable\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"value\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 78 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'active\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"value\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 79 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'0\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"value\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 80 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'no\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"value\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 81 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'false\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"value\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 82 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'off\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"value\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 83 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'disable\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"value\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 84 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'inactive\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"value\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 85 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'force\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"value\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 86 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'workspace\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"value\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 87 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_FORCE_XINERAMA: [cmdp_token; 1] = [token {
    name: b"word\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"value\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 88 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_ASSIGN_OUTPUT: [cmdp_token; 1] = [token {
    name: b"string\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"output\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 89 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_CRITERION_STR: [cmdp_token; 1] = [token {
    name: b"word\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"cvalue\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 90 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_MOUSE_WARPING: [cmdp_token; 2] = [
    token {
        name: b"'none\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"value\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 91 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'output\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"value\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 92 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_RESTART_STATE: [cmdp_token; 1] = [token {
    name: b"string\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"path\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 93 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_SMART_BORDERS: [cmdp_token; 7] = [
    token {
        name: b"'1\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"enabled\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 94 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'yes\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"enabled\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 95 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'true\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"enabled\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 96 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'on\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"enabled\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 97 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'enable\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"enabled\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 98 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'active\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"enabled\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 99 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'no_gaps\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"enabled\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 100 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_BAR_MODIFIER: [cmdp_token; 12] = [
    token {
        name: b"'off\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 101 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'none\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 102 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Mod1\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BAR_MODIFIER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Mod2\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BAR_MODIFIER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Mod3\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BAR_MODIFIER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Mod4\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BAR_MODIFIER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Mod5\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BAR_MODIFIER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Shift\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BAR_MODIFIER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Control\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BAR_MODIFIER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Ctrl\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BAR_MODIFIER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'+\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_MODIFIER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"end\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 103 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_BAR_POSITION: [cmdp_token; 2] = [
    token {
        name: b"'top\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"position\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 104 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'bottom\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"position\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 105 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_COLOR_BORDER: [cmdp_token; 1] = [token {
    name: b"word\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"border\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: COLOR_BACKGROUND,
    extra: C2RustUnnamed_5 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_COLOR_SINGLE: [cmdp_token; 1] = [token {
    name: b"word\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"color\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 106 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_FAKE_OUTPUTS: [cmdp_token; 1] = [token {
    name: b"string\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"outputs\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 107 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_MODE_BINDING: [cmdp_token; 20] = [
    token {
        name: b"'--release\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"release\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'--border\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"border\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'--whole-window\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"whole_window\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'--exclude-titlebar\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"exclude_titlebar\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Mod1\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Mod2\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Mod3\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Mod4\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Mod5\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Shift\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Control\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Ctrl\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Mode_switch\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Group1\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Group2\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Group3\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Group4\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'$mod\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'+\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"word\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"key\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        next_state: MODE_BINDCOMMAND,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_NO_FOCUS_END: [cmdp_token; 1] = [token {
    name: b"end\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 108 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_BINDSYM: [cmdp_token; 2] = [
    token {
        name: b"'--release\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"release\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BAR_BINDSYM,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"word\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"button\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BAR_BINDSYM_COMMAND,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_BAR_PADDING: [cmdp_token; 1] = [token {
    name: b"number\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"top_or_all\0" as *const u8 as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    next_state: BAR_PADDING_TOP,
    extra: C2RustUnnamed_5 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_VERBOSE: [cmdp_token; 1] = [token {
    name: b"word\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"value\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 109 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BINDCOMMAND: [cmdp_token; 5] = [
    token {
        name: b"'--release\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"release\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BINDCOMMAND,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'--border\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"border\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BINDCOMMAND,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'--whole-window\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"whole_window\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BINDCOMMAND,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'--exclude-titlebar\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"exclude_titlebar\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BINDCOMMAND,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"string\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"command\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 110 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_IGNORE_LINE: [cmdp_token; 1] = [token {
    name: b"line\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: INITIAL,
    extra: C2RustUnnamed_5 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_TILING_DRAG: [cmdp_token; 9] = [
    token {
        name: b"'0\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"off\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 111 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'no\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"off\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 112 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'false\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"off\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 113 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'off\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"off\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 114 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'disable\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"off\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 115 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'inactive\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"off\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 116 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'swap_modifier\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"swap_modifier\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: TILING_DRAG_SWAP_MODIFIER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'modifier\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"value\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: TILING_DRAG_MODE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'titlebar\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"value\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: TILING_DRAG_MODE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_TITLE_ALIGN: [cmdp_token; 3] = [
    token {
        name: b"'left\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"alignment\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 117 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'center\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"alignment\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 118 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'right\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"alignment\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 119 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_BAR_COLORS: [cmdp_token; 15] = [
    token {
        name: b"end\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_COLORS,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'#\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_COLORS_IGNORE_LINE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'set\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_COLORS_IGNORE_LINE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'background\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"colorclass\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BAR_COLORS_SINGLE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'statusline\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"colorclass\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BAR_COLORS_SINGLE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'separator\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"colorclass\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BAR_COLORS_SINGLE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'focused_background\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"colorclass\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BAR_COLORS_SINGLE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'focused_statusline\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"colorclass\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BAR_COLORS_SINGLE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'focused_separator\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"colorclass\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BAR_COLORS_SINGLE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'focused_workspace\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"colorclass\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BAR_COLORS_BORDER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'active_workspace\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"colorclass\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BAR_COLORS_BORDER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'inactive_workspace\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"colorclass\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BAR_COLORS_BORDER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'urgent_workspace\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"colorclass\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BAR_COLORS_BORDER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'binding_mode\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"colorclass\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BAR_COLORS_BORDER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'}\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_BAR_HEIGHT: [cmdp_token; 1] = [token {
    name: b"number\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"value\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 120 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_OUTPUT: [cmdp_token; 1] = [token {
    name: b"word\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"output\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 121 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_COLOR_TEXT: [cmdp_token; 1] = [token {
    name: b"word\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"text\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: COLOR_INDICATOR,
    extra: C2RustUnnamed_5 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_FOR_WINDOW: [cmdp_token; 1] = [token {
    name: b"'[\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 122 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_IPC_SOCKET: [cmdp_token; 1] = [token {
    name: b"string\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"path\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 123 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_SHOW_MARKS: [cmdp_token; 1] = [token {
    name: b"word\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"value\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 124 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_SMART_GAPS: [cmdp_token; 13] = [
    token {
        name: b"'1\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"enabled\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 125 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'yes\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"enabled\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 126 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'true\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"enabled\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 127 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'on\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"enabled\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 128 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'enable\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"enabled\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 129 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'active\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"enabled\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 130 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'0\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"enabled\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 131 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'no\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"enabled\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 132 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'false\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"enabled\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 133 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'off\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"enabled\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 134 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'disable\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"enabled\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 135 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'inactive\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"enabled\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 136 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'inverse_outer\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"enabled\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 137 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_CRITERION: [cmdp_token; 1] = [token {
    name: b"'=\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: CRITERION_STR,
    extra: C2RustUnnamed_5 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_MODEBRACE: [cmdp_token; 2] = [
    token {
        name: b"end\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: MODEBRACE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'{\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: MODE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_WORKSPACE: [cmdp_token; 1] = [token {
    name: b"word\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"workspace\0" as *const u8 as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    next_state: WORKSPACE_COMMAND,
    extra: C2RustUnnamed_5 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BARBRACE: [cmdp_token; 2] = [
    token {
        name: b"end\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BARBRACE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'{\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 138 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_BAR_FONT: [cmdp_token; 1] = [token {
    name: b"string\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"font\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 139 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_MODE: [cmdp_token; 3] = [
    token {
        name: b"'dock\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"mode\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 140 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'hide\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"mode\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 141 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'invisible\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"mode\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 142 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_CRITERIA: [cmdp_token; 17] = [
    token {
        name: b"'class\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"ctype\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: CRITERION,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'instance\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"ctype\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: CRITERION,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'window_role\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"ctype\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: CRITERION,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'con_id\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"ctype\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: CRITERION,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'id\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"ctype\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: CRITERION,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'window_type\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"ctype\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: CRITERION,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'con_mark\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"ctype\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: CRITERION,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'title\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"ctype\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: CRITERION,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'urgent\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"ctype\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: CRITERION,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'workspace\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"ctype\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: CRITERION,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'machine\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"ctype\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: CRITERION,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'floating_from\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"ctype\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: CRITERION_FROM,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'tiling_from\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"ctype\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: CRITERION_FROM,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'tiling\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"ctype\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 143 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'floating\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"ctype\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 144 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'all\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"ctype\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 145 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"']\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 146 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_GAPS_END: [cmdp_token; 2] = [
    token {
        name: b"'px\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: GAPS_END,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"end\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 147 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_MODENAME: [cmdp_token; 2] = [
    token {
        name: b"'--pango_markup\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"pango_markup\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: MODENAME,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"word\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modename\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 148 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_NO_FOCUS: [cmdp_token; 1] = [token {
    name: b"'[\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 149 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BINDING: [cmdp_token; 20] = [
    token {
        name: b"'--release\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"release\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'--border\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"border\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'--whole-window\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"whole_window\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'--exclude-titlebar\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"exclude_titlebar\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Mod1\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Mod2\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Mod3\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Mod4\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Mod5\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Shift\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Control\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Ctrl\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Mode_switch\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Group1\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Group2\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Group3\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Group4\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'$mod\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'+\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"word\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"key\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        next_state: BINDCOMMAND,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_INCLUDE: [cmdp_token; 1] = [token {
    name: b"string\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"pattern\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 150 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_INITIAL: [cmdp_token; 60] = [
    token {
        name: b"end\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: INITIAL,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"error\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: INITIAL,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'#\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: IGNORE_LINE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'set \0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: IGNORE_LINE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'set\t\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: IGNORE_LINE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'set_from_resource\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: IGNORE_LINE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'include\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: INCLUDE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'bindsym\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"bindtype\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'bindcode\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"bindtype\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'bind\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"bindtype\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'bar\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BARBRACE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'font\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: FONT,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'mode\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: MODENAME,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'gaps\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: GAPS,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'smart_borders\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: SMART_BORDERS,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'smart_gaps\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: SMART_GAPS,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'floating_minimum_size\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: FLOATING_MINIMUM_SIZE_WIDTH,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'floating_maximum_size\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: FLOATING_MAXIMUM_SIZE_WIDTH,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'floating_modifier\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: FLOATING_MODIFIER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'default_orientation\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: DEFAULT_ORIENTATION,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'workspace_layout\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: WORKSPACE_LAYOUT,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'default_border\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"windowtype\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: DEFAULT_BORDER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'new_window\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"windowtype\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: DEFAULT_BORDER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'default_floating_border\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"windowtype\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: DEFAULT_BORDER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'new_float\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"windowtype\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: DEFAULT_BORDER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'hide_edge_borders\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: HIDE_EDGE_BORDERS,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'for_window\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: FOR_WINDOW,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'assign\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ASSIGN,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'no_focus\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: NO_FOCUS,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'focus_follows_mouse\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: FOCUS_FOLLOWS_MOUSE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'mouse_warping\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: MOUSE_WARPING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'focus_wrapping\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: FOCUS_WRAPPING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'force_focus_wrapping\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: FORCE_FOCUS_WRAPPING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'force_xinerama\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: FORCE_XINERAMA,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'force-xinerama\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: FORCE_XINERAMA,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'disable_randr15\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: DISABLE_RANDR15,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'disable-randr15\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: DISABLE_RANDR15,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'workspace_auto_back_and_forth\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: WORKSPACE_BACK_AND_FORTH,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'fake_outputs\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: FAKE_OUTPUTS,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'fake-outputs\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: FAKE_OUTPUTS,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'force_display_urgency_hint\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: FORCE_DISPLAY_URGENCY_HINT,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'focus_on_window_activation\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: FOCUS_ON_WINDOW_ACTIVATION,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'title_align\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: TITLE_ALIGN,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'show_marks\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: SHOW_MARKS,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'workspace\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: WORKSPACE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'ipc_socket\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: IPC_SOCKET,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'ipc-socket\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: IPC_SOCKET,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'ipc_kill_timeout\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: IPC_KILL_TIMEOUT,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'restart_state\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: RESTART_STATE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'popup_during_fullscreen\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: POPUP_DURING_FULLSCREEN,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'tiling_drag\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: TILING_DRAG,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'exec_always\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"exectype\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: EXEC,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'exec\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"exectype\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: EXEC,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'client.background\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"colorclass\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: COLOR_SINGLE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'client.focused_inactive\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"colorclass\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: COLOR_BORDER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'client.focused_tab_title\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"colorclass\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: COLOR_BORDER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'client.focused\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"colorclass\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: COLOR_BORDER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'client.unfocused\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"colorclass\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: COLOR_BORDER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'client.urgent\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"colorclass\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: COLOR_BORDER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'client.placeholder\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"colorclass\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: COLOR_BORDER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_ASSIGN: [cmdp_token; 1] = [token {
    name: b"'[\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 151 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_ID: [cmdp_token; 1] = [token {
    name: b"word\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"bar_id\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 152 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_EXEC: [cmdp_token; 2] = [
    token {
        name: b"'--no-startup-id\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"no_startup_id\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: EXEC,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"string\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"command\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 153 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_FONT: [cmdp_token; 1] = [token {
    name: b"string\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"font\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 154 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_GAPS: [cmdp_token; 8] = [
    token {
        name: b"'inner\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"scope\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: GAPS_WITH_SCOPE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'outer\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"scope\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: GAPS_WITH_SCOPE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'horizontal\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"scope\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: GAPS_WITH_SCOPE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'vertical\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"scope\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: GAPS_WITH_SCOPE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'top\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"scope\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: GAPS_WITH_SCOPE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'right\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"scope\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: GAPS_WITH_SCOPE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'bottom\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"scope\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: GAPS_WITH_SCOPE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'left\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"scope\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: GAPS_WITH_SCOPE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_MODE: [cmdp_token; 8] = [
    token {
        name: b"end\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: MODE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"error\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: MODE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'#\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: MODE_IGNORE_LINE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'set\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: MODE_IGNORE_LINE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'bindsym\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"bindtype\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'bindcode\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"bindtype\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'bind\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"bindtype\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'}\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: INITIAL,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_BAR: [cmdp_token; 31] = [
    token {
        name: b"end\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"error\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'#\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_IGNORE_LINE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'set\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_IGNORE_LINE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'i3bar_command\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_BAR_COMMAND,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'status_command\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_STATUS_COMMAND,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'workspace_command\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_WORKSPACE_COMMAND,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'socket_path\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_SOCKET_PATH,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'mode\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_MODE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'hidden_state\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_HIDDEN_STATE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'id\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_ID,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'modifier\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_MODIFIER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'wheel_up_cmd\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_WHEEL_UP_CMD,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'wheel_down_cmd\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_WHEEL_DOWN_CMD,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'bindsym\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_BINDSYM,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'position\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_POSITION,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'output\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_OUTPUT,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'tray_output\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_TRAY_OUTPUT,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'tray_padding\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_TRAY_PADDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'font\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_FONT,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'separator_symbol\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_SEPARATOR_SYMBOL,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'binding_mode_indicator\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_BINDING_MODE_INDICATOR,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'workspace_buttons\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_WORKSPACE_BUTTONS,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'workspace_min_width\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_WORKSPACE_MIN_WIDTH,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'strip_workspace_numbers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_STRIP_WORKSPACE_NUMBERS,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'strip_workspace_name\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_STRIP_WORKSPACE_NAME,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'verbose\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_VERBOSE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'height\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_HEIGHT,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'padding\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_PADDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'colors\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_COLORS_BRACE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'}\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 155 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens: [cmdp_token_ptr; 117] = unsafe {
    [
        tokenptr {
            array: &raw const tokens_FORCE_DISPLAY_URGENCY_HINT_MS as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_FLOATING_MAXIMUM_SIZE_HEIGHT as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_FLOATING_MINIMUM_SIZE_HEIGHT as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_STRIP_WORKSPACE_NUMBERS as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_FLOATING_MAXIMUM_SIZE_WIDTH as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_FLOATING_MINIMUM_SIZE_WIDTH as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_BINDING_MODE_INDICATOR as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_WORKSPACE_MIN_WIDTH_PX as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_FOCUS_ON_WINDOW_ACTIVATION as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_FORCE_DISPLAY_URGENCY_HINT as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_TILING_DRAG_SWAP_MODIFIER as *mut cmdp_token,
            n: 10 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_STRIP_WORKSPACE_NAME as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_CRITERION_FROM_STR_START as *mut cmdp_token,
            n: 3 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_DEFAULT_BORDER_PIXELS_PX as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_WORKSPACE_BACK_AND_FORTH as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_ASSIGN_WORKSPACE_NUMBER as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_WORKSPACE_MIN_WIDTH as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_FLOATING_MAXIMUM_SIZE_X as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_FLOATING_MINIMUM_SIZE_X as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_POPUP_DURING_FULLSCREEN as *mut cmdp_token,
            n: 4 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_COLORS_IGNORE_LINE as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_CRITERION_FROM_STR_END as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_COLORS_BACKGROUND as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_WORKSPACE_BUTTONS as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_WORKSPACE_COMMAND as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_DEFAULT_BORDER_PIXELS as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_WORKSPACE_OUTPUT_WORD as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_SEPARATOR_SYMBOL as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_FORCE_FOCUS_WRAPPING as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_BINDSYM_COMMAND as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_TRAY_PADDING_PX as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_DEFAULT_ORIENTATION as *mut cmdp_token,
            n: 3 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_FOCUS_FOLLOWS_MOUSE as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_PADDING_BOTTOM as *mut cmdp_token,
            n: 3 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_STATUS_COMMAND as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_WHEEL_DOWN_CMD as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_COLOR_CHILD_BORDER as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_CRITERION_FROM_STR as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_FOR_WINDOW_COMMAND as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_COLORS_BORDER as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_COLORS_SINGLE as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_PADDING_RIGHT as *mut cmdp_token,
            n: 3 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_FLOATING_MODIFIER as *mut cmdp_token,
            n: 10 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_HIDE_EDGE_BORDERS as *mut cmdp_token,
            n: 12 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_WORKSPACE_COMMAND as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_ASSIGN_WORKSPACE as *mut cmdp_token,
            n: 5 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_COLORS_BRACE as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_HIDDEN_STATE as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_PADDING_LEFT as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_TRAY_PADDING as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_WHEEL_UP_CMD as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_COLOR_BACKGROUND as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_IPC_KILL_TIMEOUT as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_MODE_BINDCOMMAND as *mut cmdp_token,
            n: 5 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_MODE_IGNORE_LINE as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_TILING_DRAG_MODE as *mut cmdp_token,
            n: 3 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_WORKSPACE_LAYOUT as *mut cmdp_token,
            n: 4 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_BAR_COMMAND as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_COLORS_TEXT as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_IGNORE_LINE as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_PADDING_TOP as *mut cmdp_token,
            n: 3 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_SOCKET_PATH as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_TRAY_OUTPUT as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_COLOR_INDICATOR as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_DISABLE_RANDR15 as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_GAPS_WITH_SCOPE as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_CRITERION_FROM as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_DEFAULT_BORDER as *mut cmdp_token,
            n: 4 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_FOCUS_WRAPPING as *mut cmdp_token,
            n: 14 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_FORCE_XINERAMA as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_ASSIGN_OUTPUT as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_CRITERION_STR as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_MOUSE_WARPING as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_RESTART_STATE as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_SMART_BORDERS as *mut cmdp_token,
            n: 7 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_MODIFIER as *mut cmdp_token,
            n: 12 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_POSITION as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_COLOR_BORDER as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_COLOR_SINGLE as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_FAKE_OUTPUTS as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_MODE_BINDING as *mut cmdp_token,
            n: 20 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_NO_FOCUS_END as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_BINDSYM as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_PADDING as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_VERBOSE as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BINDCOMMAND as *mut cmdp_token,
            n: 5 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_IGNORE_LINE as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_TILING_DRAG as *mut cmdp_token,
            n: 9 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_TITLE_ALIGN as *mut cmdp_token,
            n: 3 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_COLORS as *mut cmdp_token,
            n: 15 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_HEIGHT as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_OUTPUT as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_COLOR_TEXT as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_FOR_WINDOW as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_IPC_SOCKET as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_SHOW_MARKS as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_SMART_GAPS as *mut cmdp_token,
            n: 13 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_CRITERION as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_MODEBRACE as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_WORKSPACE as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BARBRACE as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_FONT as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_MODE as *mut cmdp_token,
            n: 3 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_CRITERIA as *mut cmdp_token,
            n: 17 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_GAPS_END as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_MODENAME as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_NO_FOCUS as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BINDING as *mut cmdp_token,
            n: 20 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_INCLUDE as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_INITIAL as *mut cmdp_token,
            n: 60 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_ASSIGN as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_ID as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_EXEC as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_FONT as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_GAPS as *mut cmdp_token,
            n: 8 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_MODE as *mut cmdp_token,
            n: 8 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR as *mut cmdp_token,
            n: 31 as ::core::ffi::c_int,
        },
    ]
};
static mut state: cmdp_state = FORCE_DISPLAY_URGENCY_HINT_MS;
static mut statelist: [cmdp_state; 10] = [
    INITIAL,
    FORCE_DISPLAY_URGENCY_HINT_MS,
    FORCE_DISPLAY_URGENCY_HINT_MS,
    FORCE_DISPLAY_URGENCY_HINT_MS,
    FORCE_DISPLAY_URGENCY_HINT_MS,
    FORCE_DISPLAY_URGENCY_HINT_MS,
    FORCE_DISPLAY_URGENCY_HINT_MS,
    FORCE_DISPLAY_URGENCY_HINT_MS,
    FORCE_DISPLAY_URGENCY_HINT_MS,
    FORCE_DISPLAY_URGENCY_HINT_MS,
];
static mut statelist_idx: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
static mut stack: [stack_entry; 10] = [stack_entry {
    identifier: ::core::ptr::null::<::core::ffi::c_char>(),
    type_0: STACK_STR,
    val: C2RustUnnamed_6 {
        str_0: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    },
}; 10];
unsafe extern "C" fn push_string(
    mut identifier: *const ::core::ffi::c_char,
    mut str: *const ::core::ffi::c_char,
) {
    let mut c: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while c < 10 as ::core::ffi::c_int {
        if !stack[c as usize].identifier.is_null()
            && strcmp(stack[c as usize].identifier, identifier) != 0 as ::core::ffi::c_int
        {
            c += 1;
        } else {
            if stack[c as usize].identifier.is_null() {
                stack[c as usize].identifier = identifier;
                stack[c as usize].val.str_0 = sstrdup(str);
                stack[c as usize].type_0 = STACK_STR;
            } else {
                let mut prev: *mut ::core::ffi::c_char = stack[c as usize].val.str_0;
                sasprintf(
                    &raw mut (*(&raw mut stack as *mut stack_entry).offset(c as isize))
                        .val
                        .str_0,
                    b"%s,%s\0" as *const u8 as *const ::core::ffi::c_char,
                    prev,
                    str,
                );
                free(prev as *mut ::core::ffi::c_void);
            }
            return;
        }
    }
    fprintf(
        stderr,
        b"BUG: commands_parser stack full. This means either a bug in the code, or a new command which contains more than 10 identified tokens.\n\0"
            as *const u8 as *const ::core::ffi::c_char,
    );
    exit(1 as ::core::ffi::c_int);
}
unsafe extern "C" fn push_long(
    mut identifier: *const ::core::ffi::c_char,
    mut num: ::core::ffi::c_long,
) {
    let mut c: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while c < 10 as ::core::ffi::c_int {
        if !stack[c as usize].identifier.is_null() {
            c += 1;
        } else {
            stack[c as usize].identifier = identifier;
            stack[c as usize].val.num = num;
            stack[c as usize].type_0 = STACK_LONG;
            return;
        }
    }
    fprintf(
        stderr,
        b"BUG: commands_parser stack full. This means either a bug in the code, or a new command which contains more than 10 identified tokens.\n\0"
            as *const u8 as *const ::core::ffi::c_char,
    );
    exit(1 as ::core::ffi::c_int);
}
unsafe extern "C" fn get_string(
    mut identifier: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    let mut c: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while c < 10 as ::core::ffi::c_int {
        if stack[c as usize].identifier.is_null() {
            break;
        }
        if strcmp(identifier, stack[c as usize].identifier) == 0 as ::core::ffi::c_int {
            return stack[c as usize].val.str_0;
        }
        c += 1;
    }
    return ::core::ptr::null::<::core::ffi::c_char>();
}
unsafe extern "C" fn clear_stack() {
    let mut c: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while c < 10 as ::core::ffi::c_int {
        if stack[c as usize].type_0 as ::core::ffi::c_uint
            == STACK_STR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            free(stack[c as usize].val.str_0 as *mut ::core::ffi::c_void);
        }
        stack[c as usize].identifier = ::core::ptr::null::<::core::ffi::c_char>();
        stack[c as usize].val.str_0 = ::core::ptr::null_mut::<::core::ffi::c_char>();
        stack[c as usize].val.num = 0 as ::core::ffi::c_long;
        c += 1;
    }
}
unsafe extern "C" fn keysym_used_on_other_key(
    mut sym: KeySym,
    mut except_keycode: xcb_keycode_t,
) -> bool {
    let mut i: xcb_keycode_t = 0;
    let mut min_keycode: xcb_keycode_t = (*xcb_get_setup(conn)).min_keycode;
    let mut max_keycode: xcb_keycode_t = (*xcb_get_setup(conn)).max_keycode;
    i = min_keycode;
    while i as ::core::ffi::c_int != 0
        && i as ::core::ffi::c_int <= max_keycode as ::core::ffi::c_int
    {
        if !(i as ::core::ffi::c_int == except_keycode as ::core::ffi::c_int) {
            let mut level: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while level < 4 as ::core::ffi::c_int {
                if xcb_key_symbols_get_keysym(keysyms, i, level) as KeySym != sym {
                    level += 1;
                } else {
                    return true_0 != 0;
                }
            }
        }
        i = i.wrapping_add(1);
    }
    return false_0 != 0;
}
unsafe extern "C" fn next_state(mut token: *const cmdp_token) -> *mut ::core::ffi::c_char {
    let mut _next_state: cmdp_state = (*token).next_state;
    if (*token).next_state as ::core::ffi::c_uint
        == __CALL as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut modifiers: *const ::core::ffi::c_char =
            get_string(b"modifiers\0" as *const u8 as *const ::core::ffi::c_char);
        let mut keycode: ::core::ffi::c_int = atoi(get_string(
            b"key\0" as *const u8 as *const ::core::ffi::c_char,
        ));
        let mut level: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if !modifiers.is_null()
            && !strstr(
                modifiers,
                b"Shift\0" as *const u8 as *const ::core::ffi::c_char,
            )
            .is_null()
        {
            level = 1 as ::core::ffi::c_int;
            let mut syms: *const xkb_keysym_t = ::core::ptr::null::<xkb_keysym_t>();
            let mut num: ::core::ffi::c_int = xkb_keymap_key_get_syms_by_level(
                xkb_keymap,
                keycode as xkb_keycode_t,
                0 as xkb_layout_index_t,
                0 as xkb_level_index_t,
                &raw mut syms,
            );
            if num == 0 as ::core::ffi::c_int {
                errx(
                    1 as ::core::ffi::c_int,
                    b"xkb_keymap_key_get_syms_by_level returned no symbols for keycode %d\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    keycode,
                );
            }
            if !keysym_used_on_other_key(
                *syms.offset(0 as ::core::ffi::c_int as isize) as KeySym,
                keycode as xcb_keycode_t,
            ) {
                level = 0 as ::core::ffi::c_int;
            }
        }
        let mut syms_0: *const xkb_keysym_t = ::core::ptr::null::<xkb_keysym_t>();
        let mut num_0: ::core::ffi::c_int = xkb_keymap_key_get_syms_by_level(
            xkb_keymap,
            keycode as xkb_keycode_t,
            0 as xkb_layout_index_t,
            level as xkb_level_index_t,
            &raw mut syms_0,
        );
        if num_0 == 0 as ::core::ffi::c_int {
            errx(
                1 as ::core::ffi::c_int,
                b"xkb_keymap_key_get_syms_by_level returned no symbols for keycode %d\0"
                    as *const u8 as *const ::core::ffi::c_char,
                keycode,
            );
        }
        if num_0 > 1 as ::core::ffi::c_int {
            printf(
                b"xkb_keymap_key_get_syms_by_level (keycode = %d) returned %d symbolsinstead of 1, using only the first one.\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
                keycode,
                num_0,
            );
        }
        let mut str: [::core::ffi::c_char; 4096] = [0; 4096];
        if xkb_keysym_get_name(
            *syms_0.offset(0 as ::core::ffi::c_int as isize),
            &raw mut str as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as size_t,
        ) == -(1 as ::core::ffi::c_int)
        {
            errx(
                EXIT_FAILURE,
                b"xkb_keysym_get_name(%u) failed\0" as *const u8 as *const ::core::ffi::c_char,
                *syms_0.offset(0 as ::core::ffi::c_int as isize),
            );
        }
        let mut release: *const ::core::ffi::c_char =
            get_string(b"release\0" as *const u8 as *const ::core::ffi::c_char);
        let mut res: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut modrep: *mut ::core::ffi::c_char = if modifiers.is_null() {
            sstrdup(b"\0" as *const u8 as *const ::core::ffi::c_char)
        } else {
            sstrdup(modifiers)
        };
        let mut comma: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        loop {
            comma = strchr(modrep, ',' as i32);
            if comma.is_null() {
                break;
            }
            *comma = '+' as i32 as ::core::ffi::c_char;
        }
        sasprintf(
            &raw mut res,
            b"bindsym %s%s%s %s%s\n\0" as *const u8 as *const ::core::ffi::c_char,
            if modifiers.is_null() {
                b"\0" as *const u8 as *const ::core::ffi::c_char
            } else {
                modrep as *const ::core::ffi::c_char
            },
            if modifiers.is_null() {
                b"\0" as *const u8 as *const ::core::ffi::c_char
            } else {
                b"+\0" as *const u8 as *const ::core::ffi::c_char
            },
            &raw mut str as *mut ::core::ffi::c_char,
            if release.is_null() {
                b"\0" as *const u8 as *const ::core::ffi::c_char
            } else {
                release
            },
            get_string(b"command\0" as *const u8 as *const ::core::ffi::c_char),
        );
        clear_stack();
        free(modrep as *mut ::core::ffi::c_void);
        return res;
    }
    state = _next_state;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < statelist_idx {
        if statelist[i as usize] as ::core::ffi::c_uint != _next_state as ::core::ffi::c_uint {
            i += 1;
        } else {
            statelist_idx = i + 1 as ::core::ffi::c_int;
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
    }
    let fresh0 = statelist_idx;
    statelist_idx = statelist_idx + 1;
    statelist[fresh0 as usize] = _next_state;
    return ::core::ptr::null_mut::<::core::ffi::c_char>();
}
unsafe extern "C" fn rewrite_binding(
    mut input: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    state = INITIAL;
    statelist_idx = 1 as ::core::ffi::c_int;
    let mut walk: *const ::core::ffi::c_char = input;
    let len: size_t = strlen(input) as size_t;
    let mut c: ::core::ffi::c_int = 0;
    let mut token: *const cmdp_token = ::core::ptr::null::<cmdp_token>();
    let mut result: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    while walk.offset_from(input) as ::core::ffi::c_long as size_t <= len {
        while (*walk as ::core::ffi::c_int == ' ' as i32
            || *walk as ::core::ffi::c_int == '\t' as i32)
            && *walk as ::core::ffi::c_int != '\0' as i32
        {
            walk = walk.offset(1);
        }
        let mut ptr: *mut cmdp_token_ptr =
            (&raw mut tokens as *mut cmdp_token_ptr).offset(state as isize) as *mut cmdp_token_ptr;
        c = 0 as ::core::ffi::c_int;
        while c < (*ptr).n {
            token = (*ptr).array.offset(c as isize) as *mut cmdp_token;
            if *(*token).name.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '\'' as i32
            {
                if strncasecmp(
                    walk,
                    (*token).name.offset(1 as ::core::ffi::c_int as isize),
                    strlen((*token).name).wrapping_sub(1 as size_t),
                ) == 0 as ::core::ffi::c_int
                {
                    if !(*token).identifier.is_null() {
                        push_string(
                            (*token).identifier,
                            (*token).name.offset(1 as ::core::ffi::c_int as isize),
                        );
                    }
                    walk = walk.offset(strlen((*token).name).wrapping_sub(1 as size_t) as isize);
                    result = next_state(token);
                    if !result.is_null() {
                        return result;
                    }
                    break;
                }
            } else if strcmp(
                (*token).name,
                b"number\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                let mut end: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                *__errno_location() = 0 as ::core::ffi::c_int;
                let mut num: ::core::ffi::c_long =
                    strtol(walk, &raw mut end, 10 as ::core::ffi::c_int);
                if !(*__errno_location() == ERANGE && (num == LONG_MIN || num == LONG_MAX)
                    || *__errno_location() != 0 as ::core::ffi::c_int
                        && num == 0 as ::core::ffi::c_long)
                {
                    if !(end == walk as *mut ::core::ffi::c_char) {
                        if !(*token).identifier.is_null() {
                            push_long((*token).identifier, num);
                        }
                        walk = end;
                        result = next_state(token);
                        if !result.is_null() {
                            return result;
                        }
                        break;
                    }
                }
            } else {
                if strcmp(
                    (*token).name,
                    b"string\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                    || strcmp(
                        (*token).name,
                        b"word\0" as *const u8 as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                {
                    let mut beginning: *const ::core::ffi::c_char = walk;
                    if *walk as ::core::ffi::c_int == '"' as i32 {
                        beginning = beginning.offset(1);
                        walk = walk.offset(1);
                        while *walk as ::core::ffi::c_int != '\0' as i32
                            && (*walk as ::core::ffi::c_int != '"' as i32
                                || *walk.offset(-(1 as ::core::ffi::c_int as isize))
                                    as ::core::ffi::c_int
                                    == '\\' as i32)
                        {
                            walk = walk.offset(1);
                        }
                    } else if *(*token).name.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        == 's' as i32
                    {
                        while *walk as ::core::ffi::c_int != '\0' as i32
                            && *walk as ::core::ffi::c_int != '\r' as i32
                            && *walk as ::core::ffi::c_int != '\n' as i32
                        {
                            walk = walk.offset(1);
                        }
                    } else {
                        while *walk as ::core::ffi::c_int != ' ' as i32
                            && *walk as ::core::ffi::c_int != '\t' as i32
                            && *walk as ::core::ffi::c_int != ']' as i32
                            && *walk as ::core::ffi::c_int != ',' as i32
                            && *walk as ::core::ffi::c_int != ';' as i32
                            && *walk as ::core::ffi::c_int != '\r' as i32
                            && *walk as ::core::ffi::c_int != '\n' as i32
                            && *walk as ::core::ffi::c_int != '\0' as i32
                        {
                            walk = walk.offset(1);
                        }
                    }
                    if walk != beginning {
                        let mut str: *mut ::core::ffi::c_char = scalloc(
                            (walk.offset_from(beginning) as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long) as size_t,
                            1 as size_t,
                        )
                            as *mut ::core::ffi::c_char;
                        let mut inpos: ::core::ffi::c_int = 0;
                        let mut outpos: ::core::ffi::c_int = 0;
                        inpos = 0 as ::core::ffi::c_int;
                        outpos = 0 as ::core::ffi::c_int;
                        while (inpos as ::core::ffi::c_long)
                            < walk.offset_from(beginning) as ::core::ffi::c_long
                        {
                            if *beginning.offset(inpos as isize) as ::core::ffi::c_int
                                == '\\' as i32
                                && *beginning.offset((inpos + 1 as ::core::ffi::c_int) as isize)
                                    as ::core::ffi::c_int
                                    == '"' as i32
                            {
                                inpos += 1;
                            }
                            *str.offset(outpos as isize) = *beginning.offset(inpos as isize);
                            inpos += 1;
                            outpos += 1;
                        }
                        if !(*token).identifier.is_null() {
                            push_string((*token).identifier, str);
                        }
                        free(str as *mut ::core::ffi::c_void);
                        if *walk as ::core::ffi::c_int == '"' as i32 {
                            walk = walk.offset(1);
                        }
                        result = next_state(token);
                        if !result.is_null() {
                            return result;
                        }
                        break;
                    }
                }
                if strcmp(
                    (*token).name,
                    b"end\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    if *walk as ::core::ffi::c_int == '\0' as i32
                        || *walk as ::core::ffi::c_int == '\n' as i32
                        || *walk as ::core::ffi::c_int == '\r' as i32
                    {
                        result = next_state(token);
                        if !result.is_null() {
                            return result;
                        }
                        walk = walk.offset(1);
                        break;
                    }
                }
            }
            c += 1;
        }
    }
    return ::core::ptr::null_mut::<::core::ffi::c_char>();
}
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
unsafe extern "C" fn txt(
    mut col: ::core::ffi::c_int,
    mut row: ::core::ffi::c_int,
    mut text: *mut ::core::ffi::c_char,
    mut fg: color_t,
    mut bg: color_t,
) {
    let mut x: ::core::ffi::c_int =
        (col - 1 as ::core::ffi::c_int) * char_width + logical_px(4 as ::core::ffi::c_int);
    let mut y: ::core::ffi::c_int =
        (row - 1 as ::core::ffi::c_int) * font.height + logical_px(4 as ::core::ffi::c_int);
    let mut string: *mut i3String = i3string_from_utf8(text);
    draw_util_text(
        string,
        &raw mut surface,
        fg,
        bg,
        x,
        y,
        logical_px(300 as ::core::ffi::c_int) - x - logical_px(4 as ::core::ffi::c_int),
    );
    i3string_free(string);
}
unsafe extern "C" fn handle_expose() -> ::core::ffi::c_int {
    let black: color_t =
        draw_util_hex_to_color(b"#000000\0" as *const u8 as *const ::core::ffi::c_char) as color_t;
    let white: color_t =
        draw_util_hex_to_color(b"#FFFFFF\0" as *const u8 as *const ::core::ffi::c_char) as color_t;
    let green: color_t =
        draw_util_hex_to_color(b"#00FF00\0" as *const u8 as *const ::core::ffi::c_char) as color_t;
    let red: color_t =
        draw_util_hex_to_color(b"#FF0000\0" as *const u8 as *const ::core::ffi::c_char) as color_t;
    draw_util_clear_surface(&raw mut surface, black);
    set_font(&raw mut font);
    if current_step as ::core::ffi::c_uint
        == STEP_WELCOME as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        txt(
            2 as ::core::ffi::c_int,
            2 as ::core::ffi::c_int,
            b"You have not configured i3 yet.\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            white,
            black,
        );
        txt(
            2 as ::core::ffi::c_int,
            3 as ::core::ffi::c_int,
            b"Do you want me to generate a config at\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            white,
            black,
        );
        let mut msg: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        sasprintf(
            &raw mut msg,
            b"%s?\0" as *const u8 as *const ::core::ffi::c_char,
            config_path,
        );
        txt(
            2 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            msg,
            white,
            black,
        );
        free(msg as *mut ::core::ffi::c_void);
        txt(
            13 as ::core::ffi::c_int,
            6 as ::core::ffi::c_int,
            b"Yes, generate the config\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            white,
            black,
        );
        txt(
            13 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int,
            b"No, I will use the defaults\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            white,
            black,
        );
        txt(
            4 as ::core::ffi::c_int,
            6 as ::core::ffi::c_int,
            b"<Enter>\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            green,
            black,
        );
        txt(
            5 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int,
            b"<ESC>\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            red,
            black,
        );
    }
    if current_step as ::core::ffi::c_uint
        == STEP_GENERATE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        txt(
            2 as ::core::ffi::c_int,
            2 as ::core::ffi::c_int,
            b"Please choose either:\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            white,
            black,
        );
        txt(
            13 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            b"Win as default modifier\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            white,
            black,
        );
        txt(
            13 as ::core::ffi::c_int,
            5 as ::core::ffi::c_int,
            b"Alt as default modifier\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            white,
            black,
        );
        txt(
            2 as ::core::ffi::c_int,
            7 as ::core::ffi::c_int,
            b"Afterwards, press\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            white,
            black,
        );
        txt(
            13 as ::core::ffi::c_int,
            9 as ::core::ffi::c_int,
            b"to write the config\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            white,
            black,
        );
        txt(
            13 as ::core::ffi::c_int,
            10 as ::core::ffi::c_int,
            b"to abort\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            white,
            black,
        );
        if modifier as ::core::ffi::c_uint == MOD_Mod4 as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            txt(
                5 as ::core::ffi::c_int,
                5 as ::core::ffi::c_int,
                b"<Alt>\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                white,
                black,
            );
        } else {
            txt(
                5 as ::core::ffi::c_int,
                4 as ::core::ffi::c_int,
                b"<Win>\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                white,
                black,
            );
        }
        set_font(&raw mut bold_font);
        if modifier as ::core::ffi::c_uint == MOD_Mod4 as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            txt(
                2 as ::core::ffi::c_int,
                4 as ::core::ffi::c_int,
                b"-> <Win>\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                white,
                black,
            );
        } else {
            txt(
                2 as ::core::ffi::c_int,
                5 as ::core::ffi::c_int,
                b"-> <Alt>\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                white,
                black,
            );
        }
        set_font(&raw mut font);
        txt(
            4 as ::core::ffi::c_int,
            9 as ::core::ffi::c_int,
            b"<Enter>\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            green,
            black,
        );
        txt(
            5 as ::core::ffi::c_int,
            10 as ::core::ffi::c_int,
            b"<ESC>\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            red,
            black,
        );
    }
    xcb_flush(conn);
    return 1 as ::core::ffi::c_int;
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
    let mut state_filtered: uint16_t = ((*event).state as uint32_t
        & !(xcb_numlock_mask | XCB_MOD_MASK_LOCK as ::core::ffi::c_int as uint32_t))
        as uint16_t;
    state_filtered =
        (state_filtered as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as uint16_t;
    let mut sym: xcb_keysym_t =
        xcb_key_press_lookup_keysym(symbols, event, state_filtered as ::core::ffi::c_int);
    printf(
        b"sym = %c (%d)\n\0" as *const u8 as *const ::core::ffi::c_char,
        sym,
        sym,
    );
    if sym == XK_Return as xcb_keysym_t || sym == XK_KP_Enter as xcb_keysym_t {
        if current_step as ::core::ffi::c_uint
            == STEP_WELCOME as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            current_step = STEP_GENERATE;
            xcb_change_property(
                conn_0,
                XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as uint8_t,
                win,
                A__NET_WM_NAME,
                A_UTF8_STRING,
                8 as uint8_t,
                strlen(b"i3: generate config\0" as *const u8 as *const ::core::ffi::c_char)
                    as uint32_t,
                b"i3: generate config\0" as *const u8 as *const ::core::ffi::c_char
                    as *const ::core::ffi::c_void,
            );
            xcb_flush(conn_0);
        } else {
            finish();
        }
    }
    if sym == XK_Up as xcb_keysym_t || sym == XK_Down as xcb_keysym_t {
        modifier = (if modifier as ::core::ffi::c_uint
            == MOD_Mod1 as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            MOD_Mod4 as ::core::ffi::c_int
        } else {
            MOD_Mod1 as ::core::ffi::c_int
        }) as C2RustUnnamed_4;
        handle_expose();
    }
    if sym == XK_Escape as xcb_keysym_t {
        exit(0 as ::core::ffi::c_int);
    }
    let mut modmap: *mut xcb_keycode_t = xcb_get_modifier_mapping_keycodes(modmap_reply);
    let mut mask: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*modmap_reply).keycodes_per_modifier as ::core::ffi::c_int {
        let mut code: xcb_keycode_t = *modmap.offset(
            (mask * (*modmap_reply).keycodes_per_modifier as ::core::ffi::c_int + i) as isize,
        );
        if !(code as ::core::ffi::c_long == XCB_NONE) {
            printf(
                b"Modifier keycode for Mod1: 0x%02x\n\0" as *const u8 as *const ::core::ffi::c_char,
                code as ::core::ffi::c_int,
            );
            if code as ::core::ffi::c_int == (*event).detail as ::core::ffi::c_int {
                modifier = MOD_Mod1;
                printf(b"This is Mod1!\n\0" as *const u8 as *const ::core::ffi::c_char);
            }
        }
        i += 1;
    }
    mask = 6 as ::core::ffi::c_int;
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < (*modmap_reply).keycodes_per_modifier as ::core::ffi::c_int {
        let mut code_0: xcb_keycode_t = *modmap.offset(
            (mask * (*modmap_reply).keycodes_per_modifier as ::core::ffi::c_int + i_0) as isize,
        );
        if !(code_0 as ::core::ffi::c_long == XCB_NONE) {
            printf(
                b"Modifier keycode for Mod4: 0x%02x\n\0" as *const u8 as *const ::core::ffi::c_char,
                code_0 as ::core::ffi::c_int,
            );
            if code_0 as ::core::ffi::c_int == (*event).detail as ::core::ffi::c_int {
                modifier = MOD_Mod4;
                printf(b"This is Mod4!\n\0" as *const u8 as *const ::core::ffi::c_char);
            }
        }
        i_0 += 1;
    }
    handle_expose();
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn handle_button_press(mut event: *mut xcb_button_press_event_t) {
    if current_step as ::core::ffi::c_uint
        != STEP_GENERATE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return;
    }
    if ((*event).event_x as ::core::ffi::c_int)
        < (5 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) * char_width
            + logical_px(4 as ::core::ffi::c_int)
        || (*event).event_x as ::core::ffi::c_int
            > (10 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) * char_width
                + logical_px(4 as ::core::ffi::c_int)
    {
        return;
    }
    if (*event).event_y as ::core::ffi::c_int
        >= (4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) * font.height
            + logical_px(4 as ::core::ffi::c_int)
        && (*event).event_y as ::core::ffi::c_int
            <= (4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) * font.height
                + logical_px(4 as ::core::ffi::c_int)
                + font.height
    {
        modifier = MOD_Mod4;
        handle_expose();
    }
    if (*event).event_y as ::core::ffi::c_int
        >= (5 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) * font.height
            + logical_px(4 as ::core::ffi::c_int)
        && (*event).event_y as ::core::ffi::c_int
            <= (5 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) * font.height
                + logical_px(4 as ::core::ffi::c_int)
                + font.height
    {
        modifier = MOD_Mod1;
        handle_expose();
    }
}
unsafe extern "C" fn finish() {
    printf(
        b"creating \"%s\"...\n\0" as *const u8 as *const ::core::ffi::c_char,
        config_path,
    );
    let mut xkb_context: *mut xkb_context = ::core::ptr::null_mut::<xkb_context>();
    xkb_context = xkb_context_new(XKB_CONTEXT_NO_FLAGS);
    if xkb_context.is_null() {
        errx(
            1 as ::core::ffi::c_int,
            b"could not create xkbcommon context\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let mut device_id: int32_t = xkb_x11_get_core_keyboard_device_id(conn);
    xkb_keymap =
        xkb_x11_keymap_new_from_device(xkb_context, conn, device_id, XKB_KEYMAP_COMPILE_NO_FLAGS);
    if xkb_keymap.is_null() {
        errx(
            1 as ::core::ffi::c_int,
            b"xkb_x11_keymap_new_from_device failed\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let mut kc_config: *mut FILE = fopen(
        b"/usr/local/etc/i3/config.keycodes\0" as *const u8 as *const ::core::ffi::c_char,
        b"r\0" as *const u8 as *const ::core::ffi::c_char,
    ) as *mut FILE;
    if kc_config.is_null() {
        err(
            1 as ::core::ffi::c_int,
            b"Could not open input file \"%s\"\0" as *const u8 as *const ::core::ffi::c_char,
            b"/usr/local/etc/i3/config.keycodes\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let mut ks_config: *mut FILE = fopen(
        config_path,
        b"w\0" as *const u8 as *const ::core::ffi::c_char,
    ) as *mut FILE;
    if ks_config.is_null() {
        err(
            1 as ::core::ffi::c_int,
            b"Could not open output config file \"%s\"\0" as *const u8
                as *const ::core::ffi::c_char,
            config_path,
        );
    }
    free(config_path as *mut ::core::ffi::c_void);
    let mut line: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut len: size_t = 0 as size_t;
    let mut read: ssize_t = 0;
    let mut head_of_file: bool = true_0 != 0;
    fputs(
        b"# This file has been auto-generated by i3-config-wizard(1).\n\0" as *const u8
            as *const ::core::ffi::c_char,
        ks_config,
    );
    fputs(
        b"# It will not be overwritten, so edit it as you like.\n\0" as *const u8
            as *const ::core::ffi::c_char,
        ks_config,
    );
    fputs(
        b"#\n\0" as *const u8 as *const ::core::ffi::c_char,
        ks_config,
    );
    fputs(
        b"# Should you change your keyboard layout some time, delete\n\0" as *const u8
            as *const ::core::ffi::c_char,
        ks_config,
    );
    fputs(
        b"# this file and re-run i3-config-wizard(1).\n\0" as *const u8
            as *const ::core::ffi::c_char,
        ks_config,
    );
    fputs(
        b"#\n\0" as *const u8 as *const ::core::ffi::c_char,
        ks_config,
    );
    let mut linecap: size_t = 0 as size_t;
    loop {
        read = getline(&raw mut line, &raw mut linecap, kc_config) as ssize_t;
        if !(read != -(1 as ::core::ffi::c_int) as ssize_t) {
            break;
        }
        len = strlen(line);
        if head_of_file as ::core::ffi::c_int != 0
            && strncmp(
                b"# WARNING\0" as *const u8 as *const ::core::ffi::c_char,
                line,
                strlen(b"# WARNING\0" as *const u8 as *const ::core::ffi::c_char),
            ) == 0 as ::core::ffi::c_int
        {
            continue;
        }
        head_of_file = false_0 != 0;
        let mut walk: *mut ::core::ffi::c_char = line;
        while *(*__ctype_b_loc())
            .offset(*walk as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
            != 0
            && walk < line.offset(len as isize)
        {
            fputc(*walk as ::core::ffi::c_int, ks_config);
            walk = walk.offset(1);
        }
        if strncmp(
            walk,
            b"set $mod \0" as *const u8 as *const ::core::ffi::c_char,
            strlen(b"set $mod \0" as *const u8 as *const ::core::ffi::c_char),
        ) == 0 as ::core::ffi::c_int
        {
            if modifier as ::core::ffi::c_uint
                == MOD_Mod1 as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                fputs(
                    b"set $mod Mod1\n\0" as *const u8 as *const ::core::ffi::c_char,
                    ks_config,
                );
            } else {
                fputs(
                    b"set $mod Mod4\n\0" as *const u8 as *const ::core::ffi::c_char,
                    ks_config,
                );
            }
        } else if strncmp(
            walk,
            b"bindcode\0" as *const u8 as *const ::core::ffi::c_char,
            strlen(b"bindcode\0" as *const u8 as *const ::core::ffi::c_char),
        ) != 0 as ::core::ffi::c_int
        {
            fputs(walk, ks_config);
        } else {
            let mut result: *mut ::core::ffi::c_char = rewrite_binding(walk);
            fputs(result, ks_config);
            free(result as *mut ::core::ffi::c_void);
        }
    }
    fflush(ks_config);
    fsync(fileno(ks_config));
    free(line as *mut ::core::ffi::c_void);
    fclose(kc_config);
    fclose(ks_config);
    let mut sockfd: ::core::ffi::c_int = ipc_connect(socket_path);
    ipc_send_message(
        sockfd,
        strlen(b"reload\0" as *const u8 as *const ::core::ffi::c_char) as uint32_t,
        0 as uint32_t,
        b"reload\0" as *const u8 as *const ::core::ffi::c_char as *mut uint8_t,
    );
    close(sockfd);
    exit(0 as ::core::ffi::c_int);
}
unsafe fn main_0(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut xdg_config_home: *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut pattern: *mut ::core::ffi::c_char = b"pango:monospace 8\0" as *const u8
        as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char;
    let mut patternbold: *mut ::core::ffi::c_char = b"pango:monospace bold 8\0" as *const u8
        as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char;
    let mut o: ::core::ffi::c_int = 0;
    let mut option_index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut headless_run: bool = false_0 != 0;
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
            name: b"modifier\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 'm' as i32,
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
        b"sm:vh\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
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
                    concat!("i3-config-wizard ", env!("I3_VERSION"), "\n\0")
                        .as_bytes()
                        .as_ptr() as *const u8 as *const ::core::ffi::c_char,
                );
                return 0 as ::core::ffi::c_int;
            }
            109 => {
                headless_run = true_0 != 0;
                if strcmp(optarg, b"alt\0" as *const u8 as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                {
                    modifier = MOD_Mod1;
                } else if strcmp(optarg, b"win\0" as *const u8 as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                {
                    modifier = MOD_Mod4;
                } else {
                    err(
                        EXIT_FAILURE,
                        b"Invalid modifier key %s\0" as *const u8 as *const ::core::ffi::c_char,
                        optarg,
                    );
                }
            }
            104 => {
                printf(
                    concat!("i3-config-wizard ", env!("I3_VERSION"), "\n\0")
                        .as_bytes()
                        .as_ptr() as *const u8 as *const ::core::ffi::c_char,
                );
                printf(
                    b"i3-config-wizard [-s <socket>] [-m win|alt] [-v] [-h]\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                return 0 as ::core::ffi::c_int;
            }
            _ => {}
        }
    }
    let mut path: *mut ::core::ffi::c_char =
        get_config_path(::core::ptr::null::<::core::ffi::c_char>(), false_0 != 0);
    if !path.is_null() {
        printf(
            b"The config file \"%s\" already exists. Exiting.\n\0" as *const u8
                as *const ::core::ffi::c_char,
            path,
        );
        free(path as *mut ::core::ffi::c_void);
        return 0 as ::core::ffi::c_int;
    }
    xdg_config_home = getenv(b"XDG_CONFIG_HOME\0" as *const u8 as *const ::core::ffi::c_char);
    if xdg_config_home.is_null() {
        xdg_config_home =
            b"~/.config\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
    }
    xdg_config_home = resolve_tilde(xdg_config_home);
    sasprintf(
        &raw mut config_path,
        b"%s/i3/config\0" as *const u8 as *const ::core::ffi::c_char,
        xdg_config_home,
    );
    let mut config_dir: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut stbuf: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    sasprintf(
        &raw mut config_dir,
        b"%s/i3\0" as *const u8 as *const ::core::ffi::c_char,
        xdg_config_home,
    );
    if stat(config_dir, &raw mut stbuf) != 0 as ::core::ffi::c_int {
        if mkdirp(config_dir, DEFAULT_DIR_MODE as mode_t) != 0 as ::core::ffi::c_int {
            err(
                EXIT_FAILURE,
                b"mkdirp(%s) failed\0" as *const u8 as *const ::core::ffi::c_char,
                config_dir,
            );
        }
    }
    free(config_dir as *mut ::core::ffi::c_void);
    free(xdg_config_home as *mut ::core::ffi::c_void);
    let mut fd: ::core::ffi::c_int = 0;
    fd = open(config_path, O_CREAT | O_RDWR, 0o644 as ::core::ffi::c_int);
    if fd == -(1 as ::core::ffi::c_int) {
        printf(
            b"Cannot open file \"%s\" for writing: %s. Exiting.\n\0" as *const u8
                as *const ::core::ffi::c_char,
            config_path,
            strerror(*__errno_location()),
        );
        return 0 as ::core::ffi::c_int;
    }
    close(fd);
    unlink(config_path);
    let mut screen: ::core::ffi::c_int = 0;
    conn = xcb_connect(::core::ptr::null::<::core::ffi::c_char>(), &raw mut screen);
    if conn.is_null() || xcb_connection_has_error(conn) != 0 {
        errx(
            1 as ::core::ffi::c_int,
            b"Cannot open display\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if xkb_x11_setup_xkb_extension(
        conn,
        XKB_X11_MIN_MAJOR_XKB_VERSION as uint16_t,
        XKB_X11_MIN_MINOR_XKB_VERSION as uint16_t,
        XKB_X11_SETUP_XKB_EXTENSION_NO_FLAGS,
        ::core::ptr::null_mut::<uint16_t>(),
        ::core::ptr::null_mut::<uint16_t>(),
        &raw mut xkb_base_event,
        &raw mut xkb_base_error,
    ) != 1 as ::core::ffi::c_int
    {
        errx(
            EXIT_FAILURE,
            b"Could not setup XKB extension.\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    keysyms = xcb_key_symbols_alloc(conn);
    let mut modmap_cookie: xcb_get_modifier_mapping_cookie_t =
        xcb_get_modifier_mapping_cookie_t { sequence: 0 };
    modmap_cookie = xcb_get_modifier_mapping(conn);
    symbols = xcb_key_symbols_alloc(conn);
    if headless_run {
        finish();
        return 0 as ::core::ffi::c_int;
    }
    let mut _NET_WM_NAME_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(b"_NET_WM_NAME\0" as *const u8 as *const ::core::ffi::c_char) as uint16_t,
        b"_NET_WM_NAME\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut _NET_WM_WINDOW_TYPE_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(b"_NET_WM_WINDOW_TYPE\0" as *const u8 as *const ::core::ffi::c_char) as uint16_t,
        b"_NET_WM_WINDOW_TYPE\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut _NET_WM_WINDOW_TYPE_DIALOG_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(b"_NET_WM_WINDOW_TYPE_DIALOG\0" as *const u8 as *const ::core::ffi::c_char)
            as uint16_t,
        b"_NET_WM_WINDOW_TYPE_DIALOG\0" as *const u8 as *const ::core::ffi::c_char,
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
    let mut UTF8_STRING_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(b"UTF8_STRING\0" as *const u8 as *const ::core::ffi::c_char) as uint16_t,
        b"UTF8_STRING\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut sndisplay: *mut SnDisplay = sn_xcb_display_new(conn, None, None);
    let mut sncontext: *mut SnLauncheeContext =
        sn_launchee_context_new_from_environment(sndisplay, screen);
    sn_display_unref(sndisplay);
    root_screen = xcb_aux_get_screen(conn, screen);
    root = (*root_screen).root;
    modmap_reply = xcb_get_modifier_mapping_reply(
        conn,
        modmap_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if modmap_reply.is_null() {
        errx(
            EXIT_FAILURE,
            b"Could not get modifier mapping\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    xcb_numlock_mask = get_mod_mask_for(XCB_NUM_LOCK as uint32_t, symbols, modmap_reply);
    init_dpi();
    font = load_font(pattern, true_0 != 0);
    bold_font = load_font(patternbold, true_0 != 0);
    set_font(&raw mut font);
    char_width = predict_text_width(i3string_from_utf8(
        b"a\0" as *const u8 as *const ::core::ffi::c_char,
    ));
    win = xcb_generate_id(conn) as xcb_window_t;
    let mut fresh1: [uint32_t; 2] = [
        0 as ::core::ffi::c_int as uint32_t,
        (XCB_EVENT_MASK_EXPOSURE as ::core::ffi::c_int
            | XCB_EVENT_MASK_BUTTON_PRESS as ::core::ffi::c_int) as uint32_t,
    ];
    xcb_create_window(
        conn,
        XCB_COPY_FROM_PARENT as uint8_t,
        win,
        root,
        logical_px(490 as ::core::ffi::c_int) as int16_t,
        logical_px(297 as ::core::ffi::c_int) as int16_t,
        logical_px(300 as ::core::ffi::c_int) as uint16_t,
        (15 as ::core::ffi::c_int * font.height + logical_px(4 as ::core::ffi::c_int)) as uint16_t,
        0 as uint16_t,
        XCB_WINDOW_CLASS_INPUT_OUTPUT as ::core::ffi::c_int as uint16_t,
        XCB_WINDOW_CLASS_COPY_FROM_PARENT as ::core::ffi::c_int as xcb_visualid_t,
        (XCB_CW_BACK_PIXEL as ::core::ffi::c_int | XCB_CW_EVENT_MASK as ::core::ffi::c_int)
            as uint32_t,
        &raw mut fresh1 as *mut uint32_t as *const ::core::ffi::c_void,
    );
    if !sncontext.is_null() {
        sn_launchee_context_setup_window(sncontext, win as Window);
    }
    xcb_map_window(conn, win);
    let mut reply: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_WM_NAME_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply.is_null() {
        errx(
            EXIT_FAILURE,
            b"Could not get atom _NET_WM_NAME\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    A__NET_WM_NAME = (*reply).atom;
    free(reply as *mut ::core::ffi::c_void);
    let mut reply_0: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_WM_WINDOW_TYPE_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_0.is_null() {
        errx(
            EXIT_FAILURE,
            b"Could not get atom _NET_WM_WINDOW_TYPE\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    A__NET_WM_WINDOW_TYPE = (*reply_0).atom;
    free(reply_0 as *mut ::core::ffi::c_void);
    let mut reply_1: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_WM_WINDOW_TYPE_DIALOG_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_1.is_null() {
        errx(
            EXIT_FAILURE,
            b"Could not get atom _NET_WM_WINDOW_TYPE_DIALOG\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    A__NET_WM_WINDOW_TYPE_DIALOG = (*reply_1).atom;
    free(reply_1 as *mut ::core::ffi::c_void);
    let mut reply_2: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        ATOM_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_2.is_null() {
        errx(
            EXIT_FAILURE,
            b"Could not get atom ATOM\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    A_ATOM = (*reply_2).atom;
    free(reply_2 as *mut ::core::ffi::c_void);
    let mut reply_3: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        CARDINAL_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_3.is_null() {
        errx(
            EXIT_FAILURE,
            b"Could not get atom CARDINAL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    A_CARDINAL = (*reply_3).atom;
    free(reply_3 as *mut ::core::ffi::c_void);
    let mut reply_4: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        UTF8_STRING_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_4.is_null() {
        errx(
            EXIT_FAILURE,
            b"Could not get atom UTF8_STRING\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    A_UTF8_STRING = (*reply_4).atom;
    free(reply_4 as *mut ::core::ffi::c_void);
    xcb_change_property(
        conn,
        XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as uint8_t,
        win,
        A__NET_WM_WINDOW_TYPE,
        A_ATOM,
        32 as uint8_t,
        1 as uint32_t,
        &raw mut A__NET_WM_WINDOW_TYPE_DIALOG as *mut ::core::ffi::c_uchar
            as *const ::core::ffi::c_void,
    );
    xcb_change_property(
        conn,
        XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as uint8_t,
        win,
        A__NET_WM_NAME,
        A_UTF8_STRING,
        8 as uint8_t,
        strlen(b"i3: first configuration\0" as *const u8 as *const ::core::ffi::c_char) as uint32_t,
        b"i3: first configuration\0" as *const u8 as *const ::core::ffi::c_char
            as *const ::core::ffi::c_void,
    );
    draw_util_surface_init(
        conn,
        &raw mut surface,
        win as xcb_drawable_t,
        get_visualtype(root_screen),
        logical_px(300 as ::core::ffi::c_int),
        15 as ::core::ffi::c_int * font.height + logical_px(4 as ::core::ffi::c_int),
    );
    xcb_flush(conn);
    let mut cookie: xcb_grab_keyboard_cookie_t = xcb_grab_keyboard_cookie_t { sequence: 0 };
    let mut reply_5: *mut xcb_grab_keyboard_reply_t =
        ::core::ptr::null_mut::<xcb_grab_keyboard_reply_t>();
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while (reply_5.is_null()
        || (*reply_5).status as ::core::ffi::c_int != XCB_GRAB_STATUS_SUCCESS as ::core::ffi::c_int)
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
        reply_5 = xcb_grab_keyboard_reply(
            conn,
            cookie,
            ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
        );
        usleep(1000 as __useconds_t);
    }
    if (*reply_5).status as ::core::ffi::c_int != XCB_GRAB_STATUS_SUCCESS as ::core::ffi::c_int {
        fprintf(
            stderr,
            b"Could not grab keyboard, status = %d\n\0" as *const u8 as *const ::core::ffi::c_char,
            (*reply_5).status as ::core::ffi::c_int,
        );
        exit(-(1 as ::core::ffi::c_int));
    }
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
                XCB_KEY_PRESS => {
                    handle_key_press(NULL, conn, event as *mut xcb_key_press_event_t);
                }
                XCB_BUTTON_PRESS => {
                    handle_button_press(event as *mut xcb_button_press_event_t);
                }
                XCB_EXPOSE => {
                    if (*(event as *mut xcb_expose_event_t)).count as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    {
                        handle_expose();
                    }
                }
                _ => {}
            }
            free(event as *mut ::core::ffi::c_void);
        }
    }
    draw_util_surface_free(conn, &raw mut surface);
    return 0 as ::core::ffi::c_int;
}
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
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
