/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3bar - an xcb-based status- and ws-bar for i3
 * © 2010 Axel Wagner and contributors (see also: LICENSE)
 *
 * xcb.c: Communicating with X
 *
 */
// Mechanical file-by-file Rust translation of i3bar/src/xcb.c.
// Generated with C2Rust 0.22.1; C ABI and original ownership retained.
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type xcb_connection_t;
    pub type xcb_extension_t;
    pub type _PangoFontDescription;
    pub type _cairo;
    pub type _cairo_surface;
    pub type _i3String;
    pub type ev_loop;
    pub type xcb_cursor_context_t;
    fn xcb_create_window_checked(
        c: *mut xcb_connection_t,
        depth_0: uint8_t,
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
    fn xcb_create_window(
        c: *mut xcb_connection_t,
        depth_0: uint8_t,
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
    fn xcb_destroy_window(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t;
    fn xcb_change_save_set(
        c: *mut xcb_connection_t,
        mode: uint8_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t;
    fn xcb_reparent_window(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        parent: xcb_window_t,
        x: int16_t,
        y: int16_t,
    ) -> xcb_void_cookie_t;
    fn xcb_map_window_checked(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t;
    fn xcb_map_window(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t;
    fn xcb_unmap_window_checked(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t;
    fn xcb_unmap_window(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t;
    fn xcb_configure_window_checked(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        value_mask: uint16_t,
        value_list: *const ::core::ffi::c_void,
    ) -> xcb_void_cookie_t;
    fn xcb_configure_window(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        value_mask: uint16_t,
        value_list: *const ::core::ffi::c_void,
    ) -> xcb_void_cookie_t;
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
    fn xcb_get_property(
        c: *mut xcb_connection_t,
        _delete: uint8_t,
        window: xcb_window_t,
        property: xcb_atom_t,
        type_0: xcb_atom_t,
        long_offset: uint32_t,
        long_length: uint32_t,
    ) -> xcb_get_property_cookie_t;
    fn xcb_get_property_unchecked(
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
    fn xcb_set_selection_owner(
        c: *mut xcb_connection_t,
        owner: xcb_window_t,
        selection: xcb_atom_t,
        time: xcb_timestamp_t,
    ) -> xcb_void_cookie_t;
    fn xcb_get_selection_owner(
        c: *mut xcb_connection_t,
        selection: xcb_atom_t,
    ) -> xcb_get_selection_owner_cookie_t;
    fn xcb_get_selection_owner_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_get_selection_owner_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_get_selection_owner_reply_t;
    fn xcb_send_event(
        c: *mut xcb_connection_t,
        propagate: uint8_t,
        destination: xcb_window_t,
        event_mask: uint32_t,
        event: *const ::core::ffi::c_char,
    ) -> xcb_void_cookie_t;
    fn xcb_create_pixmap_checked(
        c: *mut xcb_connection_t,
        depth_0: uint8_t,
        pid: xcb_pixmap_t,
        drawable: xcb_drawable_t,
        width: uint16_t,
        height: uint16_t,
    ) -> xcb_void_cookie_t;
    fn xcb_free_pixmap(c: *mut xcb_connection_t, pixmap: xcb_pixmap_t) -> xcb_void_cookie_t;
    fn xcb_create_colormap_checked(
        c: *mut xcb_connection_t,
        alloc: uint8_t,
        mid: xcb_colormap_t,
        window: xcb_window_t,
        visual: xcb_visualid_t,
    ) -> xcb_void_cookie_t;
    fn xcb_free_cursor(c: *mut xcb_connection_t, cursor_0: xcb_cursor_t) -> xcb_void_cookie_t;
    fn xcb_flush(c: *mut xcb_connection_t) -> ::core::ffi::c_int;
    fn xcb_poll_for_event(c: *mut xcb_connection_t) -> *mut xcb_generic_event_t;
    fn xcb_request_check(
        c: *mut xcb_connection_t,
        cookie: xcb_void_cookie_t,
    ) -> *mut xcb_generic_error_t;
    fn xcb_get_extension_data(
        c: *mut xcb_connection_t,
        ext: *mut xcb_extension_t,
    ) -> *const xcb_query_extension_reply_t;
    fn xcb_get_file_descriptor(c: *mut xcb_connection_t) -> ::core::ffi::c_int;
    fn xcb_connection_has_error(c: *mut xcb_connection_t) -> ::core::ffi::c_int;
    fn xcb_disconnect(c: *mut xcb_connection_t);
    fn xcb_connect(
        displayname: *const ::core::ffi::c_char,
        screenp: *mut ::core::ffi::c_int,
    ) -> *mut xcb_connection_t;
    fn xcb_generate_id(c: *mut xcb_connection_t) -> uint32_t;
    static mut stderr: *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn root_atom_contents(
        atomname: *const ::core::ffi::c_char,
        provided_conn: *mut xcb_connection_t,
        screen_0: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn smalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn scalloc(num: size_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn sstrndup(str: *const ::core::ffi::c_char, size: size_t) -> *mut ::core::ffi::c_char;
    fn sasprintf(
        strp: *mut *mut ::core::ffi::c_char,
        fmt: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn strcasecmp_nullable(
        a: *const ::core::ffi::c_char,
        b: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn i3string_free(str: *mut i3String);
    fn i3string_as_utf8(str: *mut i3String) -> *const ::core::ffi::c_char;
    fn i3string_get_num_bytes(str: *mut i3String) -> size_t;
    fn fake_configure_notify(
        conn_0: *mut xcb_connection_t,
        r: xcb_rectangle_t,
        window: xcb_window_t,
        border_width: ::core::ffi::c_int,
    );
    fn load_font(pattern: *const ::core::ffi::c_char, fallback: bool) -> i3Font;
    fn set_font(font_0: *mut i3Font);
    fn free_font();
    fn predict_text_width(text: *mut i3String) -> ::core::ffi::c_int;
    fn get_visualtype(screen_0: *mut xcb_screen_t) -> *mut xcb_visualtype_t;
    fn logical_px(logical: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn draw_util_surface_init(
        conn_0: *mut xcb_connection_t,
        surface: *mut surface_t,
        drawable: xcb_drawable_t,
        visual: *mut xcb_visualtype_t,
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
    static mut main_loop: *mut ev_loop;
    static mut statusline_head: statusline_head;
    fn ev_io_start(loop_0: *mut ev_loop, w: *mut ev_io);
    fn ev_io_stop(loop_0: *mut ev_loop, w: *mut ev_io);
    fn ev_prepare_start(loop_0: *mut ev_loop, w: *mut ev_prepare);
    fn ev_prepare_stop(loop_0: *mut ev_loop, w: *mut ev_prepare);
    fn stop_children();
    fn cont_children();
    fn child_want_click_events() -> bool;
    fn send_block_clicked(
        button: ::core::ffi::c_int,
        name: *const ::core::ffi::c_char,
        instance: *const ::core::ffi::c_char,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
        x_rel: ::core::ffi::c_int,
        y_rel: ::core::ffi::c_int,
        out_x: ::core::ffi::c_int,
        out_y: ::core::ffi::c_int,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
        mods: ::core::ffi::c_int,
    );
    fn i3_send_msg(type_0: uint32_t, payload: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    static mut outputs: *mut outputs_head;
    fn free_outputs();
    fn output_has_focus(output: *mut i3_output) -> bool;
    static mut config: config_t;
    fn errx(__status: ::core::ffi::c_int, __format: *const ::core::ffi::c_char, ...) -> !;
    fn xcb_aux_get_depth_of_visual(screen_0: *mut xcb_screen_t, id: xcb_visualid_t) -> uint8_t;
    fn xcb_aux_get_screen(
        c: *mut xcb_connection_t,
        screen_0: ::core::ffi::c_int,
    ) -> *mut xcb_screen_t;
    fn xcb_aux_find_visual_by_attrs(
        screen_0: *mut xcb_screen_t,
        class_: int8_t,
        depth_0: int8_t,
    ) -> *mut xcb_visualtype_t;
    fn xcb_aux_sync(c: *mut xcb_connection_t);
    fn xcb_cursor_context_new(
        conn_0: *mut xcb_connection_t,
        screen_0: *mut xcb_screen_t,
        ctx: *mut *mut xcb_cursor_context_t,
    ) -> ::core::ffi::c_int;
    fn xcb_cursor_load_cursor(
        ctx: *mut xcb_cursor_context_t,
        name: *const ::core::ffi::c_char,
    ) -> xcb_cursor_t;
    fn xcb_cursor_context_free(ctx: *mut xcb_cursor_context_t);
    static mut xcb_xkb_id: xcb_extension_t;
    fn xcb_xkb_use_extension(
        c: *mut xcb_connection_t,
        wantedMajor: uint16_t,
        wantedMinor: uint16_t,
    ) -> xcb_xkb_use_extension_cookie_t;
    fn xcb_xkb_select_events(
        c: *mut xcb_connection_t,
        deviceSpec: xcb_xkb_device_spec_t,
        affectWhich: uint16_t,
        clear: uint16_t,
        selectAll: uint16_t,
        affectMap: uint16_t,
        map: uint16_t,
        details: *const ::core::ffi::c_void,
    ) -> xcb_void_cookie_t;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memmove(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
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
    fn strnlen(__string: *const ::core::ffi::c_char, __maxlen: size_t) -> size_t;
    fn strcasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn qsort(
        __base: *mut ::core::ffi::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
}
pub type size_t = usize;
pub type __int8_t = i8;
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type __compar_fn_t = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_void,
        *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
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
pub type uintptr_t = usize;
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
pub type xcb_cursor_t = uint32_t;
pub type xcb_font_t = uint32_t;
pub type xcb_gcontext_t = uint32_t;
pub type xcb_colormap_t = uint32_t;
pub type xcb_atom_t = uint32_t;
pub type xcb_drawable_t = uint32_t;
pub type xcb_visualid_t = uint32_t;
pub type xcb_timestamp_t = uint32_t;
pub type xcb_keycode_t = uint8_t;
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
pub type xcb_visibility_t = ::core::ffi::c_uint;
pub const XCB_VISIBILITY_FULLY_OBSCURED: xcb_visibility_t = 2;
pub const XCB_VISIBILITY_PARTIALLY_OBSCURED: xcb_visibility_t = 1;
pub const XCB_VISIBILITY_UNOBSCURED: xcb_visibility_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_visibility_notify_event_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub window: xcb_window_t,
    pub state: uint8_t,
    pub pad1: [uint8_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_destroy_notify_event_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub event: xcb_window_t,
    pub window: xcb_window_t,
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
pub struct xcb_map_notify_event_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub event: xcb_window_t,
    pub window: xcb_window_t,
    pub override_redirect: uint8_t,
    pub pad1: [uint8_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_configure_request_event_t {
    pub response_type: uint8_t,
    pub stack_mode: uint8_t,
    pub sequence: uint16_t,
    pub parent: xcb_window_t,
    pub window: xcb_window_t,
    pub sibling: xcb_window_t,
    pub x: int16_t,
    pub y: int16_t,
    pub width: uint16_t,
    pub height: uint16_t,
    pub border_width: uint16_t,
    pub value_mask: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_resize_request_event_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub window: xcb_window_t,
    pub width: uint16_t,
    pub height: uint16_t,
}
pub type xcb_property_t = ::core::ffi::c_uint;
pub const XCB_PROPERTY_DELETE: xcb_property_t = 1;
pub const XCB_PROPERTY_NEW_VALUE: xcb_property_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_property_notify_event_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub window: xcb_window_t,
    pub atom: xcb_atom_t,
    pub time: xcb_timestamp_t,
    pub state: uint8_t,
    pub pad1: [uint8_t; 3],
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
pub type xcb_set_mode_t = ::core::ffi::c_uint;
pub const XCB_SET_MODE_DELETE: xcb_set_mode_t = 1;
pub const XCB_SET_MODE_INSERT: xcb_set_mode_t = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_get_selection_owner_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_get_selection_owner_reply_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub owner: xcb_window_t,
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
pub type xcb_colormap_alloc_t = ::core::ffi::c_uint;
pub const XCB_COLORMAP_ALLOC_ALL: xcb_colormap_alloc_t = 1;
pub const XCB_COLORMAP_ALLOC_NONE: xcb_colormap_alloc_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_query_extension_reply_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub present: uint8_t,
    pub major_opcode: uint8_t,
    pub first_event: uint8_t,
    pub first_error: uint8_t,
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
pub struct rect_t {
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub w: ::core::ffi::c_int,
    pub h: ::core::ffi::c_int,
}
pub type rect = rect_t;
pub type blockalign_t = ::core::ffi::c_uint;
pub const ALIGN_RIGHT: blockalign_t = 2;
pub const ALIGN_CENTER: blockalign_t = 1;
pub const ALIGN_LEFT: blockalign_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct status_block_render_desc {
    pub width: uint32_t,
    pub x_offset: uint32_t,
    pub x_append: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct status_block {
    pub full_text: *mut i3String,
    pub short_text: *mut i3String,
    pub use_short: bool,
    pub render_length: uint32_t,
    pub color: *mut ::core::ffi::c_char,
    pub background: *mut ::core::ffi::c_char,
    pub border: *mut ::core::ffi::c_char,
    pub min_width: uint32_t,
    pub min_width_str: *mut ::core::ffi::c_char,
    pub align: blockalign_t,
    pub urgent: bool,
    pub no_separator: bool,
    pub border_top: uint32_t,
    pub border_right: uint32_t,
    pub border_bottom: uint32_t,
    pub border_left: uint32_t,
    pub pango_markup: bool,
    pub sep_block_width: uint32_t,
    pub full_render: status_block_render_desc,
    pub short_render: status_block_render_desc,
    pub name: *mut ::core::ffi::c_char,
    pub instance: *mut ::core::ffi::c_char,
    pub blocks: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub tqe_next: *mut status_block,
    pub tqe_prev: *mut *mut status_block,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct statusline_head {
    pub tqh_first: *mut status_block,
    pub tqh_last: *mut *mut status_block,
}
pub type C2RustUnnamed_3 = ::core::ffi::c_int;
pub const EV_ERROR: C2RustUnnamed_3 = -2147483648;
pub const EV_CUSTOM: C2RustUnnamed_3 = 16777216;
pub const EV_ASYNC: C2RustUnnamed_3 = 524288;
pub const EV_CLEANUP: C2RustUnnamed_3 = 262144;
pub const EV_FORK: C2RustUnnamed_3 = 131072;
pub const EV_EMBED: C2RustUnnamed_3 = 65536;
pub const EV_CHECK: C2RustUnnamed_3 = 32768;
pub const EV_PREPARE: C2RustUnnamed_3 = 16384;
pub const EV_IDLE: C2RustUnnamed_3 = 8192;
pub const EV_STAT: C2RustUnnamed_3 = 4096;
pub const EV_CHILD: C2RustUnnamed_3 = 2048;
pub const EV_SIGNAL: C2RustUnnamed_3 = 1024;
pub const EV_PERIODIC: C2RustUnnamed_3 = 512;
pub const EV_TIMEOUT: C2RustUnnamed_3 = 256;
pub const EV_TIMER: C2RustUnnamed_3 = 256;
pub const EV_IO: C2RustUnnamed_3 = 1;
pub const EV__IOFDSET: C2RustUnnamed_3 = 128;
pub const EV_WRITE: C2RustUnnamed_3 = 2;
pub const EV_READ: C2RustUnnamed_3 = 1;
pub const EV_NONE: C2RustUnnamed_3 = 0;
pub const EV_UNDEF: C2RustUnnamed_3 = -1;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct i3_output {
    pub name: *mut ::core::ffi::c_char,
    pub active: bool,
    pub primary: bool,
    pub visible: bool,
    pub ws: ::core::ffi::c_int,
    pub rect: rect,
    pub buffer: surface_t,
    pub statusline_buffer: surface_t,
    pub statusline_width: ::core::ffi::c_int,
    pub bar: surface_t,
    pub workspaces: *mut ws_head,
    pub trayclients: *mut tc_head,
    pub slist: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub sle_next: *mut i3_output,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tc_head {
    pub tqh_first: *mut trayclient,
    pub tqh_last: *mut *mut trayclient,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct trayclient {
    pub win: xcb_window_t,
    pub mapped: bool,
    pub xe_version: ::core::ffi::c_int,
    pub class_class: *mut ::core::ffi::c_char,
    pub class_instance: *mut ::core::ffi::c_char,
    pub tailq: C2RustUnnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub tqe_next: *mut trayclient,
    pub tqe_prev: *mut *mut trayclient,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ws_head {
    pub tqh_first: *mut i3_ws,
    pub tqh_last: *mut *mut i3_ws,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct i3_ws {
    pub id: uintptr_t,
    pub num: ::core::ffi::c_int,
    pub canonical_name: *mut ::core::ffi::c_char,
    pub name: *mut i3String,
    pub name_width: ::core::ffi::c_int,
    pub visible: bool,
    pub focused: bool,
    pub urgent: bool,
    pub output: *mut i3_output,
    pub tailq: C2RustUnnamed_6,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub tqe_next: *mut i3_ws,
    pub tqe_prev: *mut *mut i3_ws,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct outputs_head {
    pub slh_first: *mut i3_output,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mode {
    pub name: *mut i3String,
    pub name_width: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_color_strings_t {
    pub bar_fg: *mut ::core::ffi::c_char,
    pub bar_bg: *mut ::core::ffi::c_char,
    pub sep_fg: *mut ::core::ffi::c_char,
    pub focus_bar_fg: *mut ::core::ffi::c_char,
    pub focus_bar_bg: *mut ::core::ffi::c_char,
    pub focus_sep_fg: *mut ::core::ffi::c_char,
    pub active_ws_fg: *mut ::core::ffi::c_char,
    pub active_ws_bg: *mut ::core::ffi::c_char,
    pub active_ws_border: *mut ::core::ffi::c_char,
    pub inactive_ws_fg: *mut ::core::ffi::c_char,
    pub inactive_ws_bg: *mut ::core::ffi::c_char,
    pub inactive_ws_border: *mut ::core::ffi::c_char,
    pub focus_ws_bg: *mut ::core::ffi::c_char,
    pub focus_ws_fg: *mut ::core::ffi::c_char,
    pub focus_ws_border: *mut ::core::ffi::c_char,
    pub urgent_ws_bg: *mut ::core::ffi::c_char,
    pub urgent_ws_fg: *mut ::core::ffi::c_char,
    pub urgent_ws_border: *mut ::core::ffi::c_char,
    pub binding_mode_bg: *mut ::core::ffi::c_char,
    pub binding_mode_fg: *mut ::core::ffi::c_char,
    pub binding_mode_border: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_colors_t {
    pub bar_fg: color_t,
    pub bar_bg: color_t,
    pub sep_fg: color_t,
    pub focus_bar_fg: color_t,
    pub focus_bar_bg: color_t,
    pub focus_sep_fg: color_t,
    pub active_ws_fg: color_t,
    pub active_ws_bg: color_t,
    pub active_ws_border: color_t,
    pub inactive_ws_fg: color_t,
    pub inactive_ws_bg: color_t,
    pub inactive_ws_border: color_t,
    pub urgent_ws_bg: color_t,
    pub urgent_ws_fg: color_t,
    pub urgent_ws_border: color_t,
    pub focus_ws_bg: color_t,
    pub focus_ws_fg: color_t,
    pub focus_ws_border: color_t,
    pub binding_mode_bg: color_t,
    pub binding_mode_fg: color_t,
    pub binding_mode_border: color_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct config_t {
    pub modifier: uint32_t,
    pub bindings: bindings_head,
    pub position: position_t,
    pub verbose: bool,
    pub bar_height: uint32_t,
    pub padding: Rect,
    pub transparency: bool,
    pub colors: xcb_color_strings_t,
    pub disable_binding_mode_indicator: bool,
    pub disable_ws: bool,
    pub ws_min_width: ::core::ffi::c_int,
    pub strip_ws_numbers: bool,
    pub strip_ws_name: bool,
    pub bar_id: *mut ::core::ffi::c_char,
    pub command: *mut ::core::ffi::c_char,
    pub workspace_command: *mut ::core::ffi::c_char,
    pub fontname: *mut ::core::ffi::c_char,
    pub separator_symbol: *mut i3String,
    pub tray_outputs: tray_outputs_head,
    pub tray_padding: ::core::ffi::c_int,
    pub num_outputs: ::core::ffi::c_int,
    pub outputs: *mut *mut ::core::ffi::c_char,
    pub hide_on_modifier: bar_display_mode_t,
    pub hidden_state: C2RustUnnamed_7,
}
pub type C2RustUnnamed_7 = ::core::ffi::c_uint;
pub const S_SHOW: C2RustUnnamed_7 = 1;
pub const S_HIDE: C2RustUnnamed_7 = 0;
pub type bar_display_mode_t = ::core::ffi::c_uint;
pub const M_INVISIBLE: bar_display_mode_t = 2;
pub const M_HIDE: bar_display_mode_t = 1;
pub const M_DOCK: bar_display_mode_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tray_outputs_head {
    pub tqh_first: *mut tray_output_t,
    pub tqh_last: *mut *mut tray_output_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tray_output_t {
    pub output: *mut ::core::ffi::c_char,
    pub tray_outputs: C2RustUnnamed_8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub tqe_next: *mut tray_output_t,
    pub tqe_prev: *mut *mut tray_output_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Rect {
    pub x: uint32_t,
    pub y: uint32_t,
    pub width: uint32_t,
    pub height: uint32_t,
}
pub type position_t = ::core::ffi::c_uint;
pub const POS_BOT: position_t = 2;
pub const POS_TOP: position_t = 1;
pub const POS_NONE: position_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bindings_head {
    pub tqh_first: *mut binding_t,
    pub tqh_last: *mut *mut binding_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct binding_t {
    pub input_code: ::core::ffi::c_int,
    pub command: *mut ::core::ffi::c_char,
    pub release: bool,
    pub bindings: C2RustUnnamed_9,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub tqe_next: *mut binding_t,
    pub tqe_prev: *mut *mut binding_t,
}
pub const I3_SYNC: C2RustUnnamed_11 = 11;
pub const _XEMBED: C2RustUnnamed_11 = 10;
pub const _XEMBED_INFO: C2RustUnnamed_11 = 9;
pub const _NET_SYSTEM_TRAY_COLORS: C2RustUnnamed_11 = 8;
pub const _NET_SYSTEM_TRAY_OPCODE: C2RustUnnamed_11 = 7;
pub const _NET_SYSTEM_TRAY_VISUAL: C2RustUnnamed_11 = 6;
pub const _NET_SYSTEM_TRAY_ORIENTATION: C2RustUnnamed_11 = 5;
pub const MANAGER: C2RustUnnamed_11 = 4;
pub const I3_SOCKET_PATH: C2RustUnnamed_11 = 3;
pub const _NET_WM_STRUT_PARTIAL: C2RustUnnamed_11 = 2;
pub const _NET_WM_WINDOW_TYPE_DOCK: C2RustUnnamed_11 = 1;
pub const _NET_WM_WINDOW_TYPE: C2RustUnnamed_11 = 0;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct C2RustUnnamed_10 {
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
pub type xcb_xkb_device_spec_t = uint16_t;
pub const XCB_XKB_ID_USE_CORE_KBD: xcb_xkb_id_t = 256;
pub const XCB_XKB_EVENT_TYPE_STATE_NOTIFY: xcb_xkb_event_type_t = 4;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xkb_use_extension_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xkb_state_notify_event_t {
    pub response_type: uint8_t,
    pub xkbType: uint8_t,
    pub sequence: uint16_t,
    pub time: xcb_timestamp_t,
    pub deviceID: uint8_t,
    pub mods: uint8_t,
    pub baseMods: uint8_t,
    pub latchedMods: uint8_t,
    pub lockedMods: uint8_t,
    pub group: uint8_t,
    pub baseGroup: int16_t,
    pub latchedGroup: int16_t,
    pub lockedGroup: uint8_t,
    pub compatState: uint8_t,
    pub grabMods: uint8_t,
    pub compatGrabMods: uint8_t,
    pub lookupMods: uint8_t,
    pub compatLoockupMods: uint8_t,
    pub ptrBtnState: uint16_t,
    pub changed: uint16_t,
    pub keycode: xcb_keycode_t,
    pub eventType: uint8_t,
    pub requestMajor: uint8_t,
    pub requestMinor: uint8_t,
}
pub type xcb_xkb_event_type_t = ::core::ffi::c_uint;
pub const XCB_XKB_EVENT_TYPE_EXTENSION_DEVICE_NOTIFY: xcb_xkb_event_type_t = 2048;
pub const XCB_XKB_EVENT_TYPE_ACCESS_X_NOTIFY: xcb_xkb_event_type_t = 1024;
pub const XCB_XKB_EVENT_TYPE_ACTION_MESSAGE: xcb_xkb_event_type_t = 512;
pub const XCB_XKB_EVENT_TYPE_BELL_NOTIFY: xcb_xkb_event_type_t = 256;
pub const XCB_XKB_EVENT_TYPE_COMPAT_MAP_NOTIFY: xcb_xkb_event_type_t = 128;
pub const XCB_XKB_EVENT_TYPE_NAMES_NOTIFY: xcb_xkb_event_type_t = 64;
pub const XCB_XKB_EVENT_TYPE_INDICATOR_MAP_NOTIFY: xcb_xkb_event_type_t = 32;
pub const XCB_XKB_EVENT_TYPE_INDICATOR_STATE_NOTIFY: xcb_xkb_event_type_t = 16;
pub const XCB_XKB_EVENT_TYPE_CONTROLS_NOTIFY: xcb_xkb_event_type_t = 8;
pub const XCB_XKB_EVENT_TYPE_MAP_NOTIFY: xcb_xkb_event_type_t = 2;
pub const XCB_XKB_EVENT_TYPE_NEW_KEYBOARD_NOTIFY: xcb_xkb_event_type_t = 1;
pub type xcb_xkb_id_t = ::core::ffi::c_uint;
pub const XCB_XKB_ID_XI_NONE: xcb_xkb_id_t = 65280;
pub const XCB_XKB_ID_ALL_XI_ID: xcb_xkb_id_t = 1536;
pub const XCB_XKB_ID_ALL_XI_CLASS: xcb_xkb_id_t = 1280;
pub const XCB_XKB_ID_DFLT_XI_ID: xcb_xkb_id_t = 1024;
pub const XCB_XKB_ID_DFLT_XI_CLASS: xcb_xkb_id_t = 768;
pub const XCB_XKB_ID_USE_CORE_PTR: xcb_xkb_id_t = 512;
pub type C2RustUnnamed_11 = ::core::ffi::c_uint;
pub const NUM_ATOMS: C2RustUnnamed_11 = 12;
pub const XCB_BUTTON_PRESS: ::core::ffi::c_int = 4;
pub const XCB_BUTTON_RELEASE: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const XCB_EXPOSE: ::core::ffi::c_int = 12;
pub const XCB_VISIBILITY_NOTIFY: ::core::ffi::c_int = 15;
pub const XCB_DESTROY_NOTIFY: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const XCB_UNMAP_NOTIFY: ::core::ffi::c_int = 18;
pub const XCB_MAP_NOTIFY: ::core::ffi::c_int = 19;
pub const XCB_CONFIGURE_REQUEST: ::core::ffi::c_int = 23;
pub const XCB_RESIZE_REQUEST: ::core::ffi::c_int = 25;
pub const XCB_PROPERTY_NOTIFY: ::core::ffi::c_int = 28;
pub const XCB_CLIENT_MESSAGE: ::core::ffi::c_int = 33 as ::core::ffi::c_int;
pub const XCB_NONE: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const XCB_CURRENT_TIME: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const XCB_BUTTON_SCROLL_UP: ::core::ffi::c_int = 4;
pub const XCB_BUTTON_SCROLL_DOWN: ::core::ffi::c_int = 5;
pub const XCB_BUTTON_SCROLL_LEFT: ::core::ffi::c_int = 6;
pub const XCB_BUTTON_SCROLL_RIGHT: ::core::ffi::c_int = 7;
pub const _NET_SYSTEM_TRAY_ORIENTATION_HORZ: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SYSTEM_TRAY_REQUEST_DOCK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const XEMBED_MAPPED: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
pub const XEMBED_EMBEDDED_NOTIFY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const I3_IPC_MESSAGE_TYPE_RUN_COMMAND: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const I3_IPC_MESSAGE_TYPE_SYNC: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const XCB_XKB_MAJOR_VERSION: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const XCB_XKB_MINOR_VERSION: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const XCB_XKB_STATE_NOTIFY: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
#[no_mangle]
pub static mut atom_cookies: [xcb_intern_atom_cookie_t; 12] =
    [xcb_intern_atom_cookie_t { sequence: 0 }; 12];
#[no_mangle]
pub static mut atoms: [xcb_atom_t; 12] = [0; 12];
#[no_mangle]
pub static mut xcb_connection: *mut xcb_connection_t =
    ::core::ptr::null::<xcb_connection_t>() as *mut xcb_connection_t;
#[no_mangle]
pub static mut screen: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut root_screen: *mut xcb_screen_t =
    ::core::ptr::null::<xcb_screen_t>() as *mut xcb_screen_t;
#[no_mangle]
pub static mut xcb_root: xcb_window_t = 0;
static mut cursor: xcb_cursor_t = 0;
static mut selwin: xcb_window_t = XCB_NONE as xcb_window_t;
static mut tray_reply: *mut xcb_intern_atom_reply_t =
    ::core::ptr::null::<xcb_intern_atom_reply_t>() as *mut xcb_intern_atom_reply_t;
#[no_mangle]
pub static mut conn: *mut xcb_connection_t =
    ::core::ptr::null::<xcb_connection_t>() as *mut xcb_connection_t;
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
#[no_mangle]
pub static mut icon_size: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut visual_type: *mut xcb_visualtype_t =
    ::core::ptr::null::<xcb_visualtype_t>() as *mut xcb_visualtype_t;
#[no_mangle]
pub static mut depth: uint8_t = 0;
#[no_mangle]
pub static mut colormap: xcb_colormap_t = 0;
#[no_mangle]
pub static mut bar_height: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut xkb_base: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut mod_pressed: bool = 0 as ::core::ffi::c_int != 0;
#[no_mangle]
pub static mut xcb_prep: *mut ev_prepare = ::core::ptr::null::<ev_prepare>() as *mut ev_prepare;
#[no_mangle]
pub static mut xcb_io: *mut ev_io = ::core::ptr::null::<ev_io>() as *mut ev_io;
#[no_mangle]
pub static mut xkb_io: *mut ev_io = ::core::ptr::null::<ev_io>() as *mut ev_io;
static mut binding: mode = mode {
    name: ::core::ptr::null::<i3String>() as *mut i3String,
    name_width: 0,
};
#[no_mangle]
pub static mut activated_mode: bool = false_0 != 0;
static mut output_for_tray: *mut i3_output = ::core::ptr::null::<i3_output>() as *mut i3_output;
#[no_mangle]
pub static mut colors: xcb_colors_t = xcb_colors_t {
    bar_fg: color_t {
        red: 0.,
        green: 0.,
        blue: 0.,
        alpha: 0.,
        colorpixel: 0,
    },
    bar_bg: color_t {
        red: 0.,
        green: 0.,
        blue: 0.,
        alpha: 0.,
        colorpixel: 0,
    },
    sep_fg: color_t {
        red: 0.,
        green: 0.,
        blue: 0.,
        alpha: 0.,
        colorpixel: 0,
    },
    focus_bar_fg: color_t {
        red: 0.,
        green: 0.,
        blue: 0.,
        alpha: 0.,
        colorpixel: 0,
    },
    focus_bar_bg: color_t {
        red: 0.,
        green: 0.,
        blue: 0.,
        alpha: 0.,
        colorpixel: 0,
    },
    focus_sep_fg: color_t {
        red: 0.,
        green: 0.,
        blue: 0.,
        alpha: 0.,
        colorpixel: 0,
    },
    active_ws_fg: color_t {
        red: 0.,
        green: 0.,
        blue: 0.,
        alpha: 0.,
        colorpixel: 0,
    },
    active_ws_bg: color_t {
        red: 0.,
        green: 0.,
        blue: 0.,
        alpha: 0.,
        colorpixel: 0,
    },
    active_ws_border: color_t {
        red: 0.,
        green: 0.,
        blue: 0.,
        alpha: 0.,
        colorpixel: 0,
    },
    inactive_ws_fg: color_t {
        red: 0.,
        green: 0.,
        blue: 0.,
        alpha: 0.,
        colorpixel: 0,
    },
    inactive_ws_bg: color_t {
        red: 0.,
        green: 0.,
        blue: 0.,
        alpha: 0.,
        colorpixel: 0,
    },
    inactive_ws_border: color_t {
        red: 0.,
        green: 0.,
        blue: 0.,
        alpha: 0.,
        colorpixel: 0,
    },
    urgent_ws_bg: color_t {
        red: 0.,
        green: 0.,
        blue: 0.,
        alpha: 0.,
        colorpixel: 0,
    },
    urgent_ws_fg: color_t {
        red: 0.,
        green: 0.,
        blue: 0.,
        alpha: 0.,
        colorpixel: 0,
    },
    urgent_ws_border: color_t {
        red: 0.,
        green: 0.,
        blue: 0.,
        alpha: 0.,
        colorpixel: 0,
    },
    focus_ws_bg: color_t {
        red: 0.,
        green: 0.,
        blue: 0.,
        alpha: 0.,
        colorpixel: 0,
    },
    focus_ws_fg: color_t {
        red: 0.,
        green: 0.,
        blue: 0.,
        alpha: 0.,
        colorpixel: 0,
    },
    focus_ws_border: color_t {
        red: 0.,
        green: 0.,
        blue: 0.,
        alpha: 0.,
        colorpixel: 0,
    },
    binding_mode_bg: color_t {
        red: 0.,
        green: 0.,
        blue: 0.,
        alpha: 0.,
        colorpixel: 0,
    },
    binding_mode_fg: color_t {
        red: 0.,
        green: 0.,
        blue: 0.,
        alpha: 0.,
        colorpixel: 0,
    },
    binding_mode_border: color_t {
        red: 0.,
        green: 0.,
        blue: 0.,
        alpha: 0.,
        colorpixel: 0,
    },
};
static mut ws_hoff_px: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
static mut ws_voff_px: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
static mut ws_spacing_px: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
static mut sb_hoff_px: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
static mut tray_loff_px: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
static mut sep_voff_px: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
#[no_mangle]
pub static mut separator_symbol_width: ::core::ffi::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn _xcb_request_failed(
    mut cookie: xcb_void_cookie_t,
    mut err_msg: *mut ::core::ffi::c_char,
    mut line: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut err: *mut xcb_generic_error_t = ::core::ptr::null_mut::<xcb_generic_error_t>();
    err = xcb_request_check(xcb_connection, cookie);
    if !err.is_null() {
        fprintf(
            stderr,
            b"[%s:%d] ERROR: %s. X Error Code: %d\n\0" as *const u8 as *const ::core::ffi::c_char,
            b"../i3bar/src/xcb.c\0" as *const u8 as *const ::core::ffi::c_char,
            line,
            err_msg,
            (*err).error_code as ::core::ffi::c_int,
        );
        return (*err).error_code as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn get_sep_offset(mut block: *mut status_block) -> uint32_t {
    if !(*block).no_separator && (*block).sep_block_width > 0 as uint32_t {
        return (*block)
            .sep_block_width
            .wrapping_div(2 as uint32_t)
            .wrapping_add((*block).sep_block_width.wrapping_rem(2 as uint32_t));
    }
    return 0 as uint32_t;
}
unsafe extern "C" fn get_tray_width(mut trayclients: *mut tc_head) -> ::core::ffi::c_int {
    let mut trayclient: *mut trayclient = ::core::ptr::null_mut::<trayclient>();
    let mut tray_width: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    trayclient = *(*((*trayclients).tqh_last as *mut tc_head)).tqh_last as *mut trayclient;
    while !trayclient.is_null() {
        if (*trayclient).mapped {
            tray_width += icon_size + logical_px(config.tray_padding);
        }
        trayclient = *(*((*trayclient).tailq.tqe_prev as *mut tc_head)).tqh_last as *mut trayclient;
    }
    if tray_width > 0 as ::core::ffi::c_int {
        tray_width += logical_px(tray_loff_px);
    }
    return tray_width;
}
unsafe extern "C" fn draw_separator(
    mut output: *mut i3_output,
    mut x: uint32_t,
    mut block: *mut status_block,
    mut use_focus_colors: bool,
) {
    let mut sep_fg: color_t = if use_focus_colors as ::core::ffi::c_int != 0 {
        colors.focus_sep_fg
    } else {
        colors.sep_fg
    };
    let mut bar_bg: color_t = if use_focus_colors as ::core::ffi::c_int != 0 {
        colors.focus_bar_bg
    } else {
        colors.bar_bg
    };
    let mut sep_offset: uint32_t = get_sep_offset(block);
    if (*block).blocks.tqe_next.is_null() || sep_offset == 0 as uint32_t {
        return;
    }
    let mut center_x: uint32_t = x.wrapping_sub(sep_offset);
    if config.separator_symbol.is_null() {
        draw_util_rectangle(
            &raw mut (*output).statusline_buffer,
            sep_fg,
            center_x as ::core::ffi::c_double,
            logical_px(sep_voff_px) as ::core::ffi::c_double,
            logical_px(1 as ::core::ffi::c_int) as ::core::ffi::c_double,
            (bar_height - 2 as ::core::ffi::c_int * logical_px(sep_voff_px))
                as ::core::ffi::c_double,
        );
    } else {
        let mut separator_x: uint32_t = if x.wrapping_sub((*block).sep_block_width)
            > center_x.wrapping_sub((separator_symbol_width / 2 as ::core::ffi::c_int) as uint32_t)
        {
            x.wrapping_sub((*block).sep_block_width)
        } else {
            center_x.wrapping_sub((separator_symbol_width / 2 as ::core::ffi::c_int) as uint32_t)
        };
        draw_util_text(
            config.separator_symbol,
            &raw mut (*output).statusline_buffer,
            sep_fg,
            bar_bg,
            separator_x as ::core::ffi::c_int,
            bar_height / 2 as ::core::ffi::c_int - font.height / 2 as ::core::ffi::c_int,
            x.wrapping_sub(separator_x) as ::core::ffi::c_int,
        );
    };
}
unsafe extern "C" fn predict_block_length(mut block: *mut status_block) {
    let mut text: *mut i3String = (*block).full_text;
    let mut render: *mut status_block_render_desc = &raw mut (*block).full_render;
    if (*block).use_short as ::core::ffi::c_int != 0 && !(*block).short_text.is_null() {
        text = (*block).short_text;
        render = &raw mut (*block).short_render;
    }
    if i3string_get_num_bytes(text) == 0 as size_t {
        (*block).render_length = 0 as uint32_t;
        return;
    }
    (*render).width = predict_text_width(text) as uint32_t;
    if !(*block).border.is_null() {
        (*render).width = (*render).width.wrapping_add(logical_px(
            (*block).border_left.wrapping_add((*block).border_right) as ::core::ffi::c_int,
        ) as uint32_t);
    }
    if (*block).min_width <= (*render).width {
        (*render).x_offset = 0 as uint32_t;
        (*render).x_append = 0 as uint32_t;
    } else {
        let mut padding_width: uint32_t = (*block).min_width.wrapping_sub((*render).width);
        match (*block).align as ::core::ffi::c_uint {
            0 => {
                (*render).x_append = padding_width;
            }
            2 => {
                (*render).x_offset = padding_width;
            }
            1 => {
                (*render).x_offset = padding_width.wrapping_div(2 as uint32_t);
                (*render).x_append = padding_width
                    .wrapping_div(2 as uint32_t)
                    .wrapping_add(padding_width.wrapping_rem(2 as uint32_t));
            }
            _ => {}
        }
    }
    (*block).render_length = (*render)
        .width
        .wrapping_add((*render).x_offset)
        .wrapping_add((*render).x_append);
}
unsafe extern "C" fn predict_statusline_length() -> uint32_t {
    let mut width: uint32_t = 0 as uint32_t;
    let mut block: *mut status_block = ::core::ptr::null_mut::<status_block>();
    block = statusline_head.tqh_first;
    while !block.is_null() {
        predict_block_length(block);
        let mut block_width: uint32_t = (*block).render_length;
        if !(block_width == 0 as uint32_t) {
            width = width.wrapping_add(block_width);
            if !(*block).blocks.tqe_next.is_null() {
                width = width.wrapping_add((*block).sep_block_width);
            }
        }
        block = (*block).blocks.tqe_next;
    }
    return width;
}
unsafe extern "C" fn switch_block_to_short(mut block: *mut status_block) -> uint32_t {
    if (*block).short_text.is_null() || (*block).use_short as ::core::ffi::c_int != 0 {
        return 0 as uint32_t;
    }
    let mut full: uint32_t = (*block).render_length;
    (*block).use_short = true_0 != 0;
    predict_block_length(block);
    return full.wrapping_sub((*block).render_length);
}
unsafe extern "C" fn adjust_statusline_length(mut max_length: uint32_t) -> uint32_t {
    let mut width: uint32_t = predict_statusline_length();
    let mut block: *mut status_block = ::core::ptr::null_mut::<status_block>();
    block = statusline_head.tqh_first;
    while !block.is_null() {
        if width < max_length {
            break;
        }
        width = width.wrapping_sub(switch_block_to_short(block));
        if !(*block).name.is_null() {
            let mut other: *mut status_block = ::core::ptr::null_mut::<status_block>();
            other = statusline_head.tqh_first;
            while !other.is_null() {
                if !(*other).name.is_null() && strcmp((*other).name, (*block).name) == 0 {
                    width = width.wrapping_sub(switch_block_to_short(other));
                }
                other = (*other).blocks.tqe_next;
            }
        }
        block = (*block).blocks.tqe_next;
    }
    return width;
}
unsafe extern "C" fn draw_statusline(
    mut output: *mut i3_output,
    mut clip_left: uint32_t,
    mut use_focus_colors: bool,
) {
    let mut block: *mut status_block = ::core::ptr::null_mut::<status_block>();
    let mut bar_color: color_t = if use_focus_colors as ::core::ffi::c_int != 0 {
        colors.focus_bar_bg
    } else {
        colors.bar_bg
    };
    draw_util_clear_surface(&raw mut (*output).statusline_buffer, bar_color);
    let mut x: uint32_t = (0 as uint32_t).wrapping_sub(clip_left);
    block = statusline_head.tqh_first;
    while !block.is_null() {
        let mut text: *mut i3String = (*block).full_text;
        let mut render: *mut status_block_render_desc = &raw mut (*block).full_render;
        if (*block).use_short as ::core::ffi::c_int != 0 && !(*block).short_text.is_null() {
            text = (*block).short_text;
            render = &raw mut (*block).short_render;
        }
        if !(i3string_get_num_bytes(text) == 0 as size_t) {
            let mut fg_color: color_t = color_t {
                red: 0.,
                green: 0.,
                blue: 0.,
                alpha: 0.,
                colorpixel: 0,
            };
            if (*block).urgent {
                fg_color = colors.urgent_ws_fg;
            } else if !(*block).color.is_null() {
                fg_color = draw_util_hex_to_color((*block).color);
            } else if use_focus_colors {
                fg_color = colors.focus_bar_fg;
            } else {
                fg_color = colors.bar_fg;
            }
            let mut bg_color: color_t = bar_color;
            let mut full_render_width: ::core::ffi::c_int = (*render)
                .width
                .wrapping_add((*render).x_offset)
                .wrapping_add((*render).x_append)
                as ::core::ffi::c_int;
            let mut has_border: ::core::ffi::c_int = if !(*block).border.is_null() {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            };
            if !(*block).border.is_null()
                || !(*block).background.is_null()
                || (*block).urgent as ::core::ffi::c_int != 0
            {
                let mut border_color: color_t = bar_color;
                if (*block).urgent {
                    border_color = colors.urgent_ws_border;
                    bg_color = colors.urgent_ws_bg;
                } else {
                    if !(*block).border.is_null() {
                        border_color = draw_util_hex_to_color((*block).border);
                    }
                    if !(*block).background.is_null() {
                        bg_color = draw_util_hex_to_color((*block).background);
                    }
                }
                draw_util_rectangle(
                    &raw mut (*output).statusline_buffer,
                    border_color,
                    x as ::core::ffi::c_double,
                    logical_px(1 as ::core::ffi::c_int) as ::core::ffi::c_double,
                    full_render_width as ::core::ffi::c_double,
                    (bar_height - logical_px(2 as ::core::ffi::c_int)) as ::core::ffi::c_double,
                );
                draw_util_rectangle(
                    &raw mut (*output).statusline_buffer,
                    bg_color,
                    x.wrapping_add(
                        (has_border * logical_px((*block).border_left as ::core::ffi::c_int))
                            as uint32_t,
                    ) as ::core::ffi::c_double,
                    (logical_px(1 as ::core::ffi::c_int)
                        + has_border * logical_px((*block).border_top as ::core::ffi::c_int))
                        as ::core::ffi::c_double,
                    (full_render_width
                        - has_border
                            * logical_px((*block).border_right.wrapping_add((*block).border_left)
                                as ::core::ffi::c_int))
                        as ::core::ffi::c_double,
                    (bar_height
                        - has_border
                            * logical_px((*block).border_bottom.wrapping_add((*block).border_top)
                                as ::core::ffi::c_int)
                        - logical_px(2 as ::core::ffi::c_int))
                        as ::core::ffi::c_double,
                );
            }
            draw_util_text(
                text,
                &raw mut (*output).statusline_buffer,
                fg_color,
                bg_color,
                x.wrapping_add((*render).x_offset).wrapping_add(
                    (has_border * logical_px((*block).border_left as ::core::ffi::c_int))
                        as uint32_t,
                ) as ::core::ffi::c_int,
                bar_height / 2 as ::core::ffi::c_int - font.height / 2 as ::core::ffi::c_int,
                (*render).width.wrapping_sub(
                    (has_border
                        * logical_px((*block).border_left.wrapping_add((*block).border_right)
                            as ::core::ffi::c_int)) as uint32_t,
                ) as ::core::ffi::c_int,
            );
            x = x.wrapping_add(full_render_width as uint32_t);
            if !(*block).blocks.tqe_next.is_null() {
                x = x.wrapping_add((*block).sep_block_width);
                draw_separator(output, x, block, use_focus_colors);
            }
        }
        block = (*block).blocks.tqe_next;
    }
}
unsafe extern "C" fn hide_bars() {
    if config.hide_on_modifier as ::core::ffi::c_uint
        == M_DOCK as ::core::ffi::c_int as ::core::ffi::c_uint
        || config.hidden_state as ::core::ffi::c_uint
            == S_SHOW as ::core::ffi::c_int as ::core::ffi::c_uint
            && config.hide_on_modifier as ::core::ffi::c_uint
                == M_HIDE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return;
    }
    let mut walk: *mut i3_output = ::core::ptr::null_mut::<i3_output>();
    walk = (*outputs).slh_first as *mut i3_output;
    while !walk.is_null() {
        if (*walk).active {
            xcb_unmap_window(xcb_connection, (*walk).bar.id as xcb_window_t);
        }
        walk = (*walk).slist.sle_next as *mut i3_output;
    }
    stop_children();
}
unsafe extern "C" fn unhide_bars() {
    if config.hide_on_modifier as ::core::ffi::c_uint
        != M_HIDE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return;
    }
    let mut walk: *mut i3_output = ::core::ptr::null_mut::<i3_output>();
    let mut cookie: xcb_void_cookie_t = xcb_void_cookie_t { sequence: 0 };
    let mut mask: uint32_t = 0;
    let mut values: [uint32_t; 5] = [0; 5];
    cont_children();
    walk = (*outputs).slh_first as *mut i3_output;
    while !walk.is_null() {
        if !((*walk).bar.id as ::core::ffi::c_long == XCB_NONE) {
            mask = (XCB_CONFIG_WINDOW_X as ::core::ffi::c_int
                | XCB_CONFIG_WINDOW_Y as ::core::ffi::c_int
                | XCB_CONFIG_WINDOW_WIDTH as ::core::ffi::c_int
                | XCB_CONFIG_WINDOW_HEIGHT as ::core::ffi::c_int
                | XCB_CONFIG_WINDOW_STACK_MODE as ::core::ffi::c_int)
                as uint32_t;
            values[0 as ::core::ffi::c_int as usize] = (*walk).rect.x as uint32_t;
            if config.position as ::core::ffi::c_uint
                == POS_TOP as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                values[1 as ::core::ffi::c_int as usize] = (*walk).rect.y as uint32_t;
            } else {
                values[1 as ::core::ffi::c_int as usize] =
                    ((*walk).rect.y + (*walk).rect.h - bar_height) as uint32_t;
            }
            values[2 as ::core::ffi::c_int as usize] = (*walk).rect.w as uint32_t;
            values[3 as ::core::ffi::c_int as usize] = bar_height as uint32_t;
            values[4 as ::core::ffi::c_int as usize] =
                XCB_STACK_MODE_ABOVE as ::core::ffi::c_int as uint32_t;
            if config.verbose {
                printf(
                    b"[%s:%d] Reconfiguring window for output %s to %d,%d\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    b"../i3bar/src/xcb.c\0" as *const u8 as *const ::core::ffi::c_char,
                    435 as ::core::ffi::c_int,
                    (*walk).name,
                    values[0 as ::core::ffi::c_int as usize],
                    values[1 as ::core::ffi::c_int as usize],
                );
            }
            cookie = xcb_configure_window_checked(
                xcb_connection,
                (*walk).bar.id as xcb_window_t,
                mask as uint16_t,
                &raw mut values as *mut uint32_t as *const ::core::ffi::c_void,
            );
            if _xcb_request_failed(
                cookie,
                b"Could not reconfigure window\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                441 as ::core::ffi::c_int,
            ) != 0
            {
                exit(EXIT_FAILURE);
            }
            xcb_map_window(xcb_connection, (*walk).bar.id as xcb_window_t);
        }
        walk = (*walk).slist.sle_next as *mut i3_output;
    }
}
#[no_mangle]
pub unsafe extern "C" fn init_colors(mut new_colors: *const xcb_color_strings_t) {
    colors.bar_fg = draw_util_hex_to_color(if !(*new_colors).bar_fg.is_null() {
        (*new_colors).bar_fg as *const ::core::ffi::c_char
    } else {
        b"#FFFFFF\0" as *const u8 as *const ::core::ffi::c_char
    });
    colors.bar_bg = draw_util_hex_to_color(if !(*new_colors).bar_bg.is_null() {
        (*new_colors).bar_bg as *const ::core::ffi::c_char
    } else {
        b"#000000\0" as *const u8 as *const ::core::ffi::c_char
    });
    colors.sep_fg = draw_util_hex_to_color(if !(*new_colors).sep_fg.is_null() {
        (*new_colors).sep_fg as *const ::core::ffi::c_char
    } else {
        b"#666666\0" as *const u8 as *const ::core::ffi::c_char
    });
    colors.active_ws_fg = draw_util_hex_to_color(if !(*new_colors).active_ws_fg.is_null() {
        (*new_colors).active_ws_fg as *const ::core::ffi::c_char
    } else {
        b"#FFFFFF\0" as *const u8 as *const ::core::ffi::c_char
    });
    colors.active_ws_bg = draw_util_hex_to_color(if !(*new_colors).active_ws_bg.is_null() {
        (*new_colors).active_ws_bg as *const ::core::ffi::c_char
    } else {
        b"#333333\0" as *const u8 as *const ::core::ffi::c_char
    });
    colors.active_ws_border =
        draw_util_hex_to_color(if !(*new_colors).active_ws_border.is_null() {
            (*new_colors).active_ws_border as *const ::core::ffi::c_char
        } else {
            b"#333333\0" as *const u8 as *const ::core::ffi::c_char
        });
    colors.inactive_ws_fg = draw_util_hex_to_color(if !(*new_colors).inactive_ws_fg.is_null() {
        (*new_colors).inactive_ws_fg as *const ::core::ffi::c_char
    } else {
        b"#888888\0" as *const u8 as *const ::core::ffi::c_char
    });
    colors.inactive_ws_bg = draw_util_hex_to_color(if !(*new_colors).inactive_ws_bg.is_null() {
        (*new_colors).inactive_ws_bg as *const ::core::ffi::c_char
    } else {
        b"#222222\0" as *const u8 as *const ::core::ffi::c_char
    });
    colors.inactive_ws_border =
        draw_util_hex_to_color(if !(*new_colors).inactive_ws_border.is_null() {
            (*new_colors).inactive_ws_border as *const ::core::ffi::c_char
        } else {
            b"#333333\0" as *const u8 as *const ::core::ffi::c_char
        });
    colors.urgent_ws_fg = draw_util_hex_to_color(if !(*new_colors).urgent_ws_fg.is_null() {
        (*new_colors).urgent_ws_fg as *const ::core::ffi::c_char
    } else {
        b"#FFFFFF\0" as *const u8 as *const ::core::ffi::c_char
    });
    colors.urgent_ws_bg = draw_util_hex_to_color(if !(*new_colors).urgent_ws_bg.is_null() {
        (*new_colors).urgent_ws_bg as *const ::core::ffi::c_char
    } else {
        b"#900000\0" as *const u8 as *const ::core::ffi::c_char
    });
    colors.urgent_ws_border =
        draw_util_hex_to_color(if !(*new_colors).urgent_ws_border.is_null() {
            (*new_colors).urgent_ws_border as *const ::core::ffi::c_char
        } else {
            b"#2f343a\0" as *const u8 as *const ::core::ffi::c_char
        });
    colors.focus_ws_fg = draw_util_hex_to_color(if !(*new_colors).focus_ws_fg.is_null() {
        (*new_colors).focus_ws_fg as *const ::core::ffi::c_char
    } else {
        b"#FFFFFF\0" as *const u8 as *const ::core::ffi::c_char
    });
    colors.focus_ws_bg = draw_util_hex_to_color(if !(*new_colors).focus_ws_bg.is_null() {
        (*new_colors).focus_ws_bg as *const ::core::ffi::c_char
    } else {
        b"#285577\0" as *const u8 as *const ::core::ffi::c_char
    });
    colors.focus_ws_border = draw_util_hex_to_color(if !(*new_colors).focus_ws_border.is_null() {
        (*new_colors).focus_ws_border as *const ::core::ffi::c_char
    } else {
        b"#4c7899\0" as *const u8 as *const ::core::ffi::c_char
    });
    colors.binding_mode_fg = if !(*new_colors).binding_mode_fg.is_null() {
        draw_util_hex_to_color((*new_colors).binding_mode_fg)
    } else {
        colors.urgent_ws_fg
    };
    colors.binding_mode_bg = if !(*new_colors).binding_mode_bg.is_null() {
        draw_util_hex_to_color((*new_colors).binding_mode_bg)
    } else {
        colors.urgent_ws_bg
    };
    colors.binding_mode_border = if !(*new_colors).binding_mode_border.is_null() {
        draw_util_hex_to_color((*new_colors).binding_mode_border)
    } else {
        colors.urgent_ws_border
    };
    colors.focus_bar_fg = if !(*new_colors).focus_bar_fg.is_null() {
        draw_util_hex_to_color((*new_colors).focus_bar_fg)
    } else {
        colors.bar_fg
    };
    colors.focus_bar_bg = if !(*new_colors).focus_bar_bg.is_null() {
        draw_util_hex_to_color((*new_colors).focus_bar_bg)
    } else {
        colors.bar_bg
    };
    colors.focus_sep_fg = if !(*new_colors).focus_sep_fg.is_null() {
        draw_util_hex_to_color((*new_colors).focus_sep_fg)
    } else {
        colors.sep_fg
    };
    init_tray_colors();
    xcb_flush(xcb_connection);
}
unsafe extern "C" fn execute_custom_command(
    mut input_code: xcb_keycode_t,
    mut event_is_release: bool,
) -> bool {
    let mut binding_0: *mut binding_t = ::core::ptr::null_mut::<binding_t>();
    binding_0 = config.bindings.tqh_first as *mut binding_t;
    while !binding_0.is_null() {
        if (*binding_0).input_code != input_code as ::core::ffi::c_int
            || (*binding_0).release as ::core::ffi::c_int != event_is_release as ::core::ffi::c_int
        {
            binding_0 = (*binding_0).bindings.tqe_next as *mut binding_t;
        } else {
            i3_send_msg(
                I3_IPC_MESSAGE_TYPE_RUN_COMMAND as uint32_t,
                (*binding_0).command,
            );
            return true_0 != 0;
        }
    }
    return false_0 != 0;
}
unsafe extern "C" fn child_handle_button(
    mut event: *mut xcb_button_press_event_t,
    mut output: *mut i3_output,
    mut statusline_x: uint32_t,
    mut statusline_y: uint32_t,
) {
    if statusline_x > (*output).statusline_width as uint32_t {
        return;
    }
    let mut last_block_x: uint32_t = 0 as uint32_t;
    let mut block: *mut status_block = ::core::ptr::null_mut::<status_block>();
    block = statusline_head.tqh_first;
    while !block.is_null() {
        let mut text: *mut i3String = ::core::ptr::null_mut::<i3String>();
        let mut render: *mut status_block_render_desc =
            ::core::ptr::null_mut::<status_block_render_desc>();
        if (*block).use_short as ::core::ffi::c_int != 0 && !(*block).short_text.is_null() {
            text = (*block).short_text;
            render = &raw mut (*block).short_render;
        } else {
            text = (*block).full_text;
            render = &raw mut (*block).full_render;
        }
        if !(i3string_get_num_bytes(text) == 0 as size_t) {
            let full_render_width: uint32_t = (*render)
                .width
                .wrapping_add((*render).x_offset)
                .wrapping_add((*render).x_append);
            let relative_x: uint32_t = statusline_x.wrapping_sub(last_block_x);
            if relative_x <= full_render_width {
                let output_x: uint32_t = (*event).event_x as uint32_t;
                let output_y: uint32_t = statusline_y.wrapping_add((*event).event_y as uint32_t);
                send_block_clicked(
                    (*event).detail as ::core::ffi::c_int,
                    (*block).name,
                    (*block).instance,
                    (*event).root_x as ::core::ffi::c_int,
                    (*event).root_y as ::core::ffi::c_int,
                    relative_x as ::core::ffi::c_int,
                    (*event).event_y as ::core::ffi::c_int,
                    output_x as ::core::ffi::c_int,
                    output_y as ::core::ffi::c_int,
                    full_render_width as ::core::ffi::c_int,
                    bar_height,
                    (*event).state as ::core::ffi::c_int,
                );
                return;
            }
            last_block_x =
                last_block_x.wrapping_add(full_render_width.wrapping_add((*block).sep_block_width));
            if last_block_x > statusline_x {
                return;
            }
        }
        block = (*block).blocks.tqe_next;
    }
}
unsafe extern "C" fn predict_button_width(
    mut name_width: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return if name_width
        + 2 as ::core::ffi::c_int * logical_px(ws_hoff_px)
        + 2 as ::core::ffi::c_int * logical_px(1 as ::core::ffi::c_int)
        > logical_px(config.ws_min_width)
    {
        name_width
            + 2 as ::core::ffi::c_int * logical_px(ws_hoff_px)
            + 2 as ::core::ffi::c_int * logical_px(1 as ::core::ffi::c_int)
    } else {
        logical_px(config.ws_min_width)
    };
}
unsafe extern "C" fn quote_workspace_name(
    mut in_0: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let namelen: size_t = strlen(in_0) as size_t;
    let len: size_t = namelen
        .wrapping_add(
            strlen(b"workspace \"\"\0" as *const u8 as *const ::core::ffi::c_char) as size_t,
        )
        .wrapping_add(1 as size_t);
    let mut out: *mut ::core::ffi::c_char =
        scalloc((2 as size_t).wrapping_mul(len), 1 as size_t) as *mut ::core::ffi::c_char;
    memcpy(
        out as *mut ::core::ffi::c_void,
        b"workspace \"\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        strlen(b"workspace \"\0" as *const u8 as *const ::core::ffi::c_char),
    );
    let mut inpos: size_t = 0;
    let mut outpos: size_t = 0;
    inpos = 0 as size_t;
    outpos = strlen(b"workspace \"\0" as *const u8 as *const ::core::ffi::c_char);
    while inpos < namelen {
        if *in_0.offset(inpos as isize) as ::core::ffi::c_int == '"' as i32
            || *in_0.offset(inpos as isize) as ::core::ffi::c_int == '\\' as i32
        {
            *out.offset(outpos as isize) = '\\' as i32 as ::core::ffi::c_char;
            outpos = outpos.wrapping_add(1);
        }
        *out.offset(outpos as isize) = *in_0.offset(inpos as isize);
        inpos = inpos.wrapping_add(1);
        outpos = outpos.wrapping_add(1);
    }
    *out.offset(outpos as isize) = '"' as i32 as ::core::ffi::c_char;
    return out;
}
unsafe extern "C" fn focus_workspace(mut ws: *mut i3_ws) {
    let mut buffer: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if (*ws).id != 0 as uintptr_t {
        sasprintf(
            &raw mut buffer,
            b"[con_id=%lu] focus workspace\0" as *const u8 as *const ::core::ffi::c_char,
            (*ws).id,
        );
    } else {
        if (*ws).canonical_name.is_null() {
            return;
        }
        buffer = quote_workspace_name((*ws).canonical_name);
    }
    i3_send_msg(I3_IPC_MESSAGE_TYPE_RUN_COMMAND as uint32_t, buffer);
    free(buffer as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn handle_button(mut event: *mut xcb_button_press_event_t) {
    let mut walk: *mut i3_output = ::core::ptr::null_mut::<i3_output>();
    let mut bar: xcb_window_t = (*event).event;
    walk = (*outputs).slh_first as *mut i3_output;
    while !walk.is_null() {
        if (*walk).bar.id == bar {
            break;
        }
        walk = (*walk).slist.sle_next as *mut i3_output;
    }
    if walk.is_null() {
        if config.verbose {
            printf(
                b"[%s:%d] Unknown bar clicked!\n\0" as *const u8 as *const ::core::ffi::c_char,
                b"../i3bar/src/xcb.c\0" as *const u8 as *const ::core::ffi::c_char,
                626 as ::core::ffi::c_int,
            );
        }
        return;
    }
    if config.verbose {
        printf(
            b"[%s:%d] Got button %d\n\0" as *const u8 as *const ::core::ffi::c_char,
            b"../i3bar/src/xcb.c\0" as *const u8 as *const ::core::ffi::c_char,
            630 as ::core::ffi::c_int,
            (*event).detail as ::core::ffi::c_int,
        );
    }
    let event_is_release: bool = (*event).response_type as ::core::ffi::c_int
        & !(0x80 as ::core::ffi::c_int)
        == XCB_BUTTON_RELEASE;
    let x: ::core::ffi::c_int =
        (if (*event).event_x as ::core::ffi::c_int >= 0 as ::core::ffi::c_int {
            (*event).event_x as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) - logical_px(config.padding.x as ::core::ffi::c_int) as ::core::ffi::c_int;
    if x < 0 as ::core::ffi::c_int {
        return;
    }
    let mut workspace_width: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut cur_ws: *mut i3_ws = ::core::ptr::null_mut::<i3_ws>();
    let mut clicked_ws: *mut i3_ws = ::core::ptr::null_mut::<i3_ws>();
    let mut ws_walk: *mut i3_ws = ::core::ptr::null_mut::<i3_ws>();
    ws_walk = (*(*walk).workspaces).tqh_first as *mut i3_ws;
    while !ws_walk.is_null() {
        let mut w: ::core::ffi::c_int = predict_button_width((*ws_walk).name_width);
        if x >= workspace_width && x <= workspace_width + w {
            clicked_ws = ws_walk;
        }
        if (*ws_walk).visible {
            cur_ws = ws_walk;
        }
        workspace_width += w;
        if !(*ws_walk).tailq.tqe_next.is_null() {
            workspace_width += logical_px(ws_spacing_px);
        }
        ws_walk = (*ws_walk).tailq.tqe_next as *mut i3_ws;
    }
    if child_want_click_events() as ::core::ffi::c_int != 0 && x > workspace_width {
        let tray_width: ::core::ffi::c_int =
            get_tray_width((*walk).trayclients as *mut tc_head) as ::core::ffi::c_int;
        let offset_x: ::core::ffi::c_int = (*walk).rect.w
            - (*walk).statusline_width
            - tray_width
            - logical_px((tray_width > 0 as ::core::ffi::c_int) as ::core::ffi::c_int * sb_hoff_px)
                as ::core::ffi::c_int;
        if x >= offset_x {
            if !event_is_release {
                let statusline_x: uint32_t = (x - offset_x) as uint32_t;
                let statusline_y: uint32_t = ((*event).root_y as ::core::ffi::c_int
                    - (*walk).rect.y
                    - (*event).event_y as ::core::ffi::c_int)
                    as uint32_t;
                child_handle_button(event, walk, statusline_x, statusline_y);
            }
            return;
        }
    }
    if execute_custom_command((*event).detail as xcb_keycode_t, event_is_release)
        as ::core::ffi::c_int
        != 0
        || event_is_release as ::core::ffi::c_int != 0
    {
        return;
    }
    if cur_ws.is_null() {
        if config.verbose {
            printf(
                b"[%s:%d] No workspace active?\n\0" as *const u8 as *const ::core::ffi::c_char,
                b"../i3bar/src/xcb.c\0" as *const u8 as *const ::core::ffi::c_char,
                690 as ::core::ffi::c_int,
            );
        }
        return;
    }
    match (*event).detail as ::core::ffi::c_int {
        XCB_BUTTON_SCROLL_UP | XCB_BUTTON_SCROLL_LEFT => {
            if cur_ws == (*(*walk).workspaces).tqh_first {
                return;
            }
            cur_ws = *(*((*cur_ws).tailq.tqe_prev as *mut ws_head)).tqh_last as *mut i3_ws;
        }
        XCB_BUTTON_SCROLL_DOWN | XCB_BUTTON_SCROLL_RIGHT => {
            if cur_ws == *(*((*(*walk).workspaces).tqh_last as *mut ws_head)).tqh_last {
                return;
            }
            cur_ws = (*cur_ws).tailq.tqe_next as *mut i3_ws;
        }
        1 => {
            cur_ws = clicked_ws;
            if cur_ws.is_null() {
                cur_ws = (*(*walk).workspaces).tqh_first as *mut i3_ws;
                while !cur_ws.is_null() {
                    if (*cur_ws).visible as ::core::ffi::c_int != 0 && !(*cur_ws).focused {
                        break;
                    }
                    cur_ws = (*cur_ws).tailq.tqe_next as *mut i3_ws;
                }
            }
            if cur_ws.is_null() {
                return;
            }
        }
        _ => return,
    }
    focus_workspace(cur_ws);
}
unsafe extern "C" fn handle_visibility_notify(mut event: *mut xcb_visibility_notify_event_t) {
    let mut visible: bool =
        (*event).state as ::core::ffi::c_int != XCB_VISIBILITY_FULLY_OBSCURED as ::core::ffi::c_int;
    let mut num_visible: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut output: *mut i3_output = ::core::ptr::null_mut::<i3_output>();
    output = (*outputs).slh_first as *mut i3_output;
    while !output.is_null() {
        if (*output).active {
            if (*output).bar.id == (*event).window {
                (*output).visible = visible;
            }
            num_visible += (*output).visible as ::core::ffi::c_int;
        }
        output = (*output).slist.sle_next as *mut i3_output;
    }
    if num_visible == 0 as ::core::ffi::c_int {
        stop_children();
    } else {
        cont_children();
    };
}
unsafe extern "C" fn reorder_trayclients_cmp(
    mut _a: *const ::core::ffi::c_void,
    mut _b: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut a: *mut trayclient = *(_a as *mut *mut trayclient);
    let mut b: *mut trayclient = *(_b as *mut *mut trayclient);
    let mut result: ::core::ffi::c_int = strcasecmp_nullable((*a).class_class, (*b).class_class);
    return if result != 0 as ::core::ffi::c_int {
        result
    } else {
        strcasecmp_nullable((*a).class_instance, (*b).class_instance)
    };
}
unsafe extern "C" fn configure_trayclients() {
    let mut output: *mut i3_output = ::core::ptr::null_mut::<i3_output>();
    output = (*outputs).slh_first as *mut i3_output;
    while !output.is_null() {
        if (*output).active {
            let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut client: *mut trayclient = ::core::ptr::null_mut::<trayclient>();
            client = (*(*output).trayclients).tqh_first as *mut trayclient;
            while !client.is_null() {
                if (*client).mapped {
                    count += 1;
                }
                client = (*client).tailq.tqe_next as *mut trayclient;
            }
            let mut idx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut trayclients: *mut *mut trayclient = smalloc(
                (count as size_t).wrapping_mul(::core::mem::size_of::<*mut trayclient>() as size_t),
            ) as *mut *mut trayclient;
            client = (*(*output).trayclients).tqh_first as *mut trayclient;
            while !client.is_null() {
                if (*client).mapped {
                    let fresh2 = idx;
                    idx = idx + 1;
                    let ref mut fresh3 = *trayclients.offset(fresh2 as isize);
                    *fresh3 = client;
                }
                client = (*client).tailq.tqe_next as *mut trayclient;
            }
            qsort(
                trayclients as *mut ::core::ffi::c_void,
                count as size_t,
                ::core::mem::size_of::<*mut trayclient>() as size_t,
                Some(
                    reorder_trayclients_cmp
                        as unsafe extern "C" fn(
                            *const ::core::ffi::c_void,
                            *const ::core::ffi::c_void,
                        ) -> ::core::ffi::c_int,
                ),
            );
            let mut x: uint32_t = (*output).rect.w as uint32_t;
            idx = count;
            while idx > 0 as ::core::ffi::c_int {
                x = x.wrapping_sub((icon_size + logical_px(config.tray_padding)) as uint32_t);
                if config.verbose {
                    printf(
                        b"[%s:%d] Configuring tray window %08x to x=%d\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                        b"../i3bar/src/xcb.c\0" as *const u8 as *const ::core::ffi::c_char,
                        819 as ::core::ffi::c_int,
                        (**trayclients.offset((idx - 1 as ::core::ffi::c_int) as isize)).win,
                        x,
                    );
                }
                xcb_configure_window(
                    xcb_connection,
                    (**trayclients.offset((idx - 1 as ::core::ffi::c_int) as isize)).win,
                    XCB_CONFIG_WINDOW_X as ::core::ffi::c_int as uint16_t,
                    &raw mut x as *const ::core::ffi::c_void,
                );
                idx -= 1;
            }
            free(trayclients as *mut ::core::ffi::c_void);
        }
        output = (*output).slist.sle_next as *mut i3_output;
    }
}
unsafe extern "C" fn trayclient_and_output_from_window(
    mut win: xcb_window_t,
    mut output: *mut *mut i3_output,
) -> *mut trayclient {
    let mut o_walk: *mut i3_output = ::core::ptr::null_mut::<i3_output>();
    o_walk = (*outputs).slh_first as *mut i3_output;
    while !o_walk.is_null() {
        if (*o_walk).active {
            let mut client: *mut trayclient = ::core::ptr::null_mut::<trayclient>();
            client = (*(*o_walk).trayclients).tqh_first as *mut trayclient;
            while !client.is_null() {
                if (*client).win == win {
                    if !output.is_null() {
                        *output = o_walk;
                    }
                    return client;
                }
                client = (*client).tailq.tqe_next as *mut trayclient;
            }
        }
        o_walk = (*o_walk).slist.sle_next as *mut i3_output;
    }
    return ::core::ptr::null_mut::<trayclient>();
}
unsafe extern "C" fn trayclient_from_window(mut win: xcb_window_t) -> *mut trayclient {
    return trayclient_and_output_from_window(win, ::core::ptr::null_mut::<*mut i3_output>());
}
unsafe extern "C" fn trayclient_update_class(mut client: *mut trayclient) {
    let mut prop: *mut xcb_get_property_reply_t = xcb_get_property_reply(
        conn,
        xcb_get_property_unchecked(
            xcb_connection,
            false_0 as uint8_t,
            (*client).win,
            XCB_ATOM_WM_CLASS as ::core::ffi::c_int as xcb_atom_t,
            XCB_ATOM_STRING as ::core::ffi::c_int as xcb_atom_t,
            0 as uint32_t,
            32 as uint32_t,
        ),
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if prop.is_null() || xcb_get_property_value_length(prop) == 0 as ::core::ffi::c_int {
        if config.verbose {
            printf(
                b"[%s:%d] WM_CLASS not set.\n\0" as *const u8 as *const ::core::ffi::c_char,
                b"../i3bar/src/xcb.c\0" as *const u8 as *const ::core::ffi::c_char,
                867 as ::core::ffi::c_int,
            );
        }
        free(prop as *mut ::core::ffi::c_void);
        return;
    }
    let prop_length: size_t = xcb_get_property_value_length(prop) as size_t;
    let mut new_class: *mut ::core::ffi::c_char =
        xcb_get_property_value(prop) as *mut ::core::ffi::c_char;
    let class_class_index: size_t =
        (strnlen(new_class, prop_length) as size_t).wrapping_add(1 as size_t);
    free((*client).class_instance as *mut ::core::ffi::c_void);
    free((*client).class_class as *mut ::core::ffi::c_void);
    (*client).class_instance = sstrndup(new_class, prop_length);
    if class_class_index < prop_length {
        (*client).class_class = sstrndup(
            new_class.offset(class_class_index as isize),
            prop_length.wrapping_sub(class_class_index),
        );
    } else {
        (*client).class_class = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if config.verbose {
        printf(
            b"[%s:%d] WM_CLASS changed to %s (instance), %s (class)\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"../i3bar/src/xcb.c\0" as *const u8 as *const ::core::ffi::c_char,
            888 as ::core::ffi::c_int,
            (*client).class_instance,
            (*client).class_class,
        );
    }
    free(prop as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn handle_client_message(mut event: *mut xcb_client_message_event_t) {
    if (*event).type_0 == atoms[I3_SYNC as ::core::ffi::c_int as usize] {
        let mut window: xcb_window_t = (*event).data.data32[0 as ::core::ffi::c_int as usize];
        let mut rnd: uint32_t = (*event).data.data32[1 as ::core::ffi::c_int as usize];
        let mut payload: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        sasprintf(
            &raw mut payload,
            b"{\"rnd\":%d, \"window\":%d}\0" as *const u8 as *const ::core::ffi::c_char,
            rnd,
            window,
        );
        i3_send_msg(I3_IPC_MESSAGE_TYPE_SYNC as uint32_t, payload);
        free(payload as *mut ::core::ffi::c_void);
    } else if (*event).type_0 == atoms[_NET_SYSTEM_TRAY_OPCODE as ::core::ffi::c_int as usize]
        && (*event).format as ::core::ffi::c_int == 32 as ::core::ffi::c_int
    {
        if config.verbose {
            printf(
                b"[%s:%d] _NET_SYSTEM_TRAY_OPCODE received\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"../i3bar/src/xcb.c\0" as *const u8 as *const ::core::ffi::c_char,
                912 as ::core::ffi::c_int,
            );
        }
        let mut op: uint32_t = (*event).data.data32[1 as ::core::ffi::c_int as usize];
        let mut values: [uint32_t; 2] = [0; 2];
        if op == SYSTEM_TRAY_REQUEST_DOCK as uint32_t {
            let client: xcb_window_t = (*event).data.data32[2 as ::core::ffi::c_int as usize];
            let mut mask: uint32_t = XCB_CW_EVENT_MASK as ::core::ffi::c_int as uint32_t;
            values[0 as ::core::ffi::c_int as usize] =
                XCB_EVENT_MASK_PROPERTY_CHANGE as ::core::ffi::c_int as uint32_t;
            values[0 as ::core::ffi::c_int as usize] |=
                XCB_EVENT_MASK_STRUCTURE_NOTIFY as ::core::ffi::c_int as uint32_t;
            values[0 as ::core::ffi::c_int as usize] |=
                XCB_EVENT_MASK_RESIZE_REDIRECT as ::core::ffi::c_int as uint32_t;
            xcb_change_window_attributes(
                xcb_connection,
                client,
                mask,
                &raw mut values as *mut uint32_t as *const ::core::ffi::c_void,
            );
            let mut map_it: bool = true_0 != 0;
            let mut xe_version: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
            let mut xembedc: xcb_get_property_cookie_t = xcb_get_property_cookie_t { sequence: 0 };
            let mut error: *mut xcb_generic_error_t =
                ::core::ptr::null_mut::<xcb_generic_error_t>();
            xembedc = xcb_get_property(
                xcb_connection,
                0 as uint8_t,
                client,
                atoms[_XEMBED_INFO as ::core::ffi::c_int as usize],
                XCB_GET_PROPERTY_TYPE_ANY as ::core::ffi::c_int as xcb_atom_t,
                0 as uint32_t,
                (2 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as uint32_t,
            );
            let mut xembedr: *mut xcb_get_property_reply_t =
                xcb_get_property_reply(xcb_connection, xembedc, &raw mut error);
            if !error.is_null() {
                fprintf(
                    stderr,
                    b"[%s:%d] ERROR: Error getting _XEMBED_INFO property: error_code %d\n\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    b"../i3bar/src/xcb.c\0" as *const u8 as *const ::core::ffi::c_char,
                    950 as ::core::ffi::c_int,
                    (*error).error_code as ::core::ffi::c_int,
                );
                free(error as *mut ::core::ffi::c_void);
                return;
            }
            if !xembedr.is_null() && (*xembedr).length != 0 as uint32_t {
                if config.verbose {
                    printf(
                        b"[%s:%d] xembed format = %d, len = %d\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                        b"../i3bar/src/xcb.c\0" as *const u8 as *const ::core::ffi::c_char,
                        955 as ::core::ffi::c_int,
                        (*xembedr).format as ::core::ffi::c_int,
                        (*xembedr).length,
                    );
                }
                let mut xembed: *mut uint32_t = xcb_get_property_value(xembedr) as *mut uint32_t;
                if config.verbose {
                    printf(
                        b"[%s:%d] xembed version = %d\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                        b"../i3bar/src/xcb.c\0" as *const u8 as *const ::core::ffi::c_char,
                        957 as ::core::ffi::c_int,
                        *xembed.offset(0 as ::core::ffi::c_int as isize),
                    );
                }
                if config.verbose {
                    printf(
                        b"[%s:%d] xembed flags = %d\n\0" as *const u8 as *const ::core::ffi::c_char,
                        b"../i3bar/src/xcb.c\0" as *const u8 as *const ::core::ffi::c_char,
                        958 as ::core::ffi::c_int,
                        *xembed.offset(1 as ::core::ffi::c_int as isize),
                    );
                }
                map_it = *xembed.offset(1 as ::core::ffi::c_int as isize)
                    & XEMBED_MAPPED as uint32_t
                    == XEMBED_MAPPED as uint32_t;
                xe_version = *xembed.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
                if xe_version > 1 as ::core::ffi::c_int {
                    xe_version = 1 as ::core::ffi::c_int;
                }
                free(xembedr as *mut ::core::ffi::c_void);
            } else {
                fprintf(
                    stderr,
                    b"[%s:%d] ERROR: Window %08x violates the XEMBED protocol, _XEMBED_INFO not set\n\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    b"../i3bar/src/xcb.c\0" as *const u8 as *const ::core::ffi::c_char,
                    966 as ::core::ffi::c_int,
                    client,
                );
            }
            if config.verbose {
                printf(
                    b"[%s:%d] X window %08x requested docking\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    b"../i3bar/src/xcb.c\0" as *const u8 as *const ::core::ffi::c_char,
                    969 as ::core::ffi::c_int,
                    client,
                );
            }
            if output_for_tray.is_null() {
                fprintf(
                    stderr,
                    b"[%s:%d] ERROR: No output found for tray\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    b"../i3bar/src/xcb.c\0" as *const u8 as *const ::core::ffi::c_char,
                    972 as ::core::ffi::c_int,
                );
                return;
            }
            let mut rcookie: xcb_void_cookie_t = xcb_reparent_window(
                xcb_connection,
                client,
                (*output_for_tray).bar.id as xcb_window_t,
                ((*output_for_tray).rect.w - icon_size - logical_px(config.tray_padding))
                    as int16_t,
                logical_px(config.tray_padding) as int16_t,
            );
            if _xcb_request_failed(
                rcookie,
                b"Could not reparent window. Maybe it is using an incorrect depth/visual?\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                981 as ::core::ffi::c_int,
            ) != 0
            {
                return;
            }
            mask = (XCB_CONFIG_WINDOW_WIDTH as ::core::ffi::c_int
                | XCB_CONFIG_WINDOW_HEIGHT as ::core::ffi::c_int) as uint32_t;
            values[0 as ::core::ffi::c_int as usize] = icon_size as uint32_t;
            values[1 as ::core::ffi::c_int as usize] = icon_size as uint32_t;
            xcb_configure_window(
                xcb_connection,
                client,
                mask as uint16_t,
                &raw mut values as *mut uint32_t as *const ::core::ffi::c_void,
            );
            let mut event_0: *mut ::core::ffi::c_void = scalloc(32 as size_t, 1 as size_t);
            let mut ev: *mut xcb_client_message_event_t =
                event_0 as *mut xcb_client_message_event_t;
            (*ev).response_type = XCB_CLIENT_MESSAGE as uint8_t;
            (*ev).window = client;
            (*ev).type_0 = atoms[_XEMBED as ::core::ffi::c_int as usize];
            (*ev).format = 32 as uint8_t;
            (*ev).data.data32[0 as ::core::ffi::c_int as usize] = XCB_CURRENT_TIME as uint32_t;
            (*ev).data.data32[1 as ::core::ffi::c_int as usize] =
                XEMBED_EMBEDDED_NOTIFY as uint32_t;
            (*ev).data.data32[2 as ::core::ffi::c_int as usize] =
                (*output_for_tray).bar.id as uint32_t;
            (*ev).data.data32[3 as ::core::ffi::c_int as usize] = xe_version as uint32_t;
            xcb_send_event(
                xcb_connection,
                0 as uint8_t,
                client,
                XCB_EVENT_MASK_NO_EVENT as ::core::ffi::c_int as uint32_t,
                ev as *mut ::core::ffi::c_char,
            );
            free(event_0);
            xcb_change_save_set(
                xcb_connection,
                XCB_SET_MODE_INSERT as ::core::ffi::c_int as uint8_t,
                client,
            );
            let mut tc: *mut trayclient =
                scalloc(1 as size_t, ::core::mem::size_of::<trayclient>() as size_t)
                    as *mut trayclient;
            (*tc).win = client;
            (*tc).xe_version = xe_version;
            (*tc).mapped = false_0 != 0;
            (*tc).tailq.tqe_next = ::core::ptr::null_mut::<trayclient>();
            (*tc).tailq.tqe_prev = (*(*output_for_tray).trayclients).tqh_last;
            *(*(*output_for_tray).trayclients).tqh_last = tc as *mut trayclient;
            (*(*output_for_tray).trayclients).tqh_last = &raw mut (*tc).tailq.tqe_next;
            trayclient_update_class(tc);
            if map_it {
                if config.verbose {
                    printf(
                        b"[%s:%d] Mapping dock client\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                        b"../i3bar/src/xcb.c\0" as *const u8 as *const ::core::ffi::c_char,
                        1031 as ::core::ffi::c_int,
                    );
                }
                xcb_map_window(xcb_connection, client);
            } else if config.verbose {
                printf(
                    b"[%s:%d] Not mapping dock client yet\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    b"../i3bar/src/xcb.c\0" as *const u8 as *const ::core::ffi::c_char,
                    1034 as ::core::ffi::c_int,
                );
            }
            configure_trayclients();
            draw_bars(false_0 != 0);
        }
    }
}
unsafe extern "C" fn handle_destroy_notify(mut event: *mut xcb_destroy_notify_event_t) {
    if config.verbose {
        printf(
            b"[%s:%d] DestroyNotify for window = %08x, event = %08x\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"../i3bar/src/xcb.c\0" as *const u8 as *const ::core::ffi::c_char,
            1054 as ::core::ffi::c_int,
            (*event).window,
            (*event).event,
        );
    }
    let mut output: *mut i3_output = ::core::ptr::null_mut::<i3_output>();
    let mut client: *mut trayclient =
        trayclient_and_output_from_window((*event).window, &raw mut output);
    if client.is_null() {
        if config.verbose {
            printf(
                b"[%s:%d] WARNING: Could not find corresponding tray window.\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"../i3bar/src/xcb.c\0" as *const u8 as *const ::core::ffi::c_char,
                1059 as ::core::ffi::c_int,
            );
        }
        return;
    }
    if config.verbose {
        printf(
            b"[%s:%d] Removing tray client with window ID %08x\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"../i3bar/src/xcb.c\0" as *const u8 as *const ::core::ffi::c_char,
            1063 as ::core::ffi::c_int,
            (*event).window,
        );
    }
    if !(*client).tailq.tqe_next.is_null() {
        (*(*client).tailq.tqe_next).tailq.tqe_prev = (*client).tailq.tqe_prev;
    } else {
        (*(*output).trayclients).tqh_last = (*client).tailq.tqe_prev;
    }
    *(*client).tailq.tqe_prev = (*client).tailq.tqe_next;
    free((*client).class_class as *mut ::core::ffi::c_void);
    free((*client).class_instance as *mut ::core::ffi::c_void);
    free(client as *mut ::core::ffi::c_void);
    client = ::core::ptr::null_mut::<trayclient>();
    configure_trayclients();
    draw_bars(false_0 != 0);
}
unsafe extern "C" fn handle_map_notify(mut event: *mut xcb_map_notify_event_t) {
    if config.verbose {
        printf(
            b"[%s:%d] MapNotify for window = %08x, event = %08x\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"../i3bar/src/xcb.c\0" as *const u8 as *const ::core::ffi::c_char,
            1080 as ::core::ffi::c_int,
            (*event).window,
            (*event).event,
        );
    }
    let mut client: *mut trayclient = trayclient_from_window((*event).window);
    if client.is_null() {
        if config.verbose {
            printf(
                b"[%s:%d] WARNING: Could not find corresponding tray window.\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"../i3bar/src/xcb.c\0" as *const u8 as *const ::core::ffi::c_char,
                1084 as ::core::ffi::c_int,
            );
        }
        return;
    }
    if config.verbose {
        printf(
            b"[%s:%d] Tray client mapped (window ID %08x). Adjusting tray.\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"../i3bar/src/xcb.c\0" as *const u8 as *const ::core::ffi::c_char,
            1088 as ::core::ffi::c_int,
            (*event).window,
        );
    }
    (*client).mapped = true_0 != 0;
    configure_trayclients();
    draw_bars(false_0 != 0);
}
unsafe extern "C" fn handle_unmap_notify(mut event: *mut xcb_unmap_notify_event_t) {
    if config.verbose {
        printf(
            b"[%s:%d] UnmapNotify for window = %08x, event = %08x\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"../i3bar/src/xcb.c\0" as *const u8 as *const ::core::ffi::c_char,
            1101 as ::core::ffi::c_int,
            (*event).window,
            (*event).event,
        );
    }
    let mut client: *mut trayclient = trayclient_from_window((*event).window);
    if client.is_null() {
        if config.verbose {
            printf(
                b"[%s:%d] WARNING: Could not find corresponding tray window.\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"../i3bar/src/xcb.c\0" as *const u8 as *const ::core::ffi::c_char,
                1105 as ::core::ffi::c_int,
            );
        }
        return;
    }
    if config.verbose {
        printf(
            b"[%s:%d] Tray client unmapped (window ID %08x). Adjusting tray.\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"../i3bar/src/xcb.c\0" as *const u8 as *const ::core::ffi::c_char,
            1109 as ::core::ffi::c_int,
            (*event).window,
        );
    }
    (*client).mapped = false_0 != 0;
    configure_trayclients();
    draw_bars(false_0 != 0);
}
unsafe extern "C" fn handle_property_notify(mut event: *mut xcb_property_notify_event_t) {
    if config.verbose {
        printf(
            b"[%s:%d] PropertyNotify\n\0" as *const u8 as *const ::core::ffi::c_char,
            b"../i3bar/src/xcb.c\0" as *const u8 as *const ::core::ffi::c_char,
            1122 as ::core::ffi::c_int,
        );
    }
    if (*event).atom == atoms[_XEMBED_INFO as ::core::ffi::c_int as usize]
        && (*event).state as ::core::ffi::c_int == XCB_PROPERTY_NEW_VALUE as ::core::ffi::c_int
    {
        if config.verbose {
            printf(
                b"[%s:%d] xembed_info updated\n\0" as *const u8 as *const ::core::ffi::c_char,
                b"../i3bar/src/xcb.c\0" as *const u8 as *const ::core::ffi::c_char,
                1126 as ::core::ffi::c_int,
            );
        }
        let mut client: *mut trayclient = trayclient_from_window((*event).window);
        if client.is_null() {
            fprintf(
                stderr,
                b"[%s:%d] ERROR: PropertyNotify received for unknown window %08x\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"../i3bar/src/xcb.c\0" as *const u8 as *const ::core::ffi::c_char,
                1130 as ::core::ffi::c_int,
                (*event).window,
            );
            return;
        }
        let mut xembedc: xcb_get_property_cookie_t = xcb_get_property_cookie_t { sequence: 0 };
        xembedc = xcb_get_property_unchecked(
            xcb_connection,
            0 as uint8_t,
            (*client).win,
            atoms[_XEMBED_INFO as ::core::ffi::c_int as usize],
            XCB_GET_PROPERTY_TYPE_ANY as ::core::ffi::c_int as xcb_atom_t,
            0 as uint32_t,
            (2 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as uint32_t,
        );
        let mut xembedr: *mut xcb_get_property_reply_t = xcb_get_property_reply(
            xcb_connection,
            xembedc,
            ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
        );
        if xembedr.is_null() || (*xembedr).length == 0 as uint32_t {
            if config.verbose {
                printf(
                    b"[%s:%d] xembed_info unset\n\0" as *const u8 as *const ::core::ffi::c_char,
                    b"../i3bar/src/xcb.c\0" as *const u8 as *const ::core::ffi::c_char,
                    1147 as ::core::ffi::c_int,
                );
            }
            return;
        }
        if config.verbose {
            printf(
                b"[%s:%d] xembed format = %d, len = %d\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"../i3bar/src/xcb.c\0" as *const u8 as *const ::core::ffi::c_char,
                1151 as ::core::ffi::c_int,
                (*xembedr).format as ::core::ffi::c_int,
                (*xembedr).length,
            );
        }
        let mut xembed: *mut uint32_t = xcb_get_property_value(xembedr) as *mut uint32_t;
        if config.verbose {
            printf(
                b"[%s:%d] xembed version = %d\n\0" as *const u8 as *const ::core::ffi::c_char,
                b"../i3bar/src/xcb.c\0" as *const u8 as *const ::core::ffi::c_char,
                1153 as ::core::ffi::c_int,
                *xembed.offset(0 as ::core::ffi::c_int as isize),
            );
        }
        if config.verbose {
            printf(
                b"[%s:%d] xembed flags = %d\n\0" as *const u8 as *const ::core::ffi::c_char,
                b"../i3bar/src/xcb.c\0" as *const u8 as *const ::core::ffi::c_char,
                1154 as ::core::ffi::c_int,
                *xembed.offset(1 as ::core::ffi::c_int as isize),
            );
        }
        let mut map_it: bool = *xembed.offset(1 as ::core::ffi::c_int as isize)
            & XEMBED_MAPPED as uint32_t
            == XEMBED_MAPPED as uint32_t;
        if config.verbose {
            printf(
                b"[%s:%d] map state now %d\n\0" as *const u8 as *const ::core::ffi::c_char,
                b"../i3bar/src/xcb.c\0" as *const u8 as *const ::core::ffi::c_char,
                1156 as ::core::ffi::c_int,
                map_it as ::core::ffi::c_int,
            );
        }
        if (*client).mapped as ::core::ffi::c_int != 0 && !map_it {
            xcb_unmap_window(xcb_connection, (*client).win);
        } else if !(*client).mapped && map_it as ::core::ffi::c_int != 0 {
            xcb_map_window(xcb_connection, (*client).win);
        }
        free(xembedr as *mut ::core::ffi::c_void);
    } else if (*event).atom == XCB_ATOM_WM_CLASS as ::core::ffi::c_int as xcb_atom_t {
        let mut client_0: *mut trayclient = trayclient_from_window((*event).window);
        if !client_0.is_null() {
            trayclient_update_class(client_0);
        }
    }
}
unsafe extern "C" fn handle_configuration_change(mut window: xcb_window_t) {
    let mut trayclient: *mut trayclient = ::core::ptr::null_mut::<trayclient>();
    let mut output: *mut i3_output = ::core::ptr::null_mut::<i3_output>();
    output = (*outputs).slh_first as *mut i3_output;
    while !output.is_null() {
        if (*output).active {
            let mut clients: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            trayclient =
                *(*((*(*output).trayclients).tqh_last as *mut tc_head)).tqh_last as *mut trayclient;
            while !trayclient.is_null() {
                if (*trayclient).mapped {
                    clients += 1;
                    if !((*trayclient).win != window) {
                        let mut rect: xcb_rectangle_t = xcb_rectangle_t {
                            x: 0,
                            y: 0,
                            width: 0,
                            height: 0,
                        };
                        rect.x = ((*output).rect.w
                            - clients * (icon_size + logical_px(config.tray_padding)))
                            as int16_t;
                        rect.y = logical_px(config.tray_padding) as int16_t;
                        rect.width = icon_size as uint16_t;
                        rect.height = icon_size as uint16_t;
                        if config.verbose {
                            printf(
                                b"[%s:%d] This is a tray window. x = %d\n\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                b"../i3bar/src/xcb.c\0" as *const u8 as *const ::core::ffi::c_char,
                                1203 as ::core::ffi::c_int,
                                rect.x as ::core::ffi::c_int,
                            );
                        }
                        fake_configure_notify(
                            xcb_connection,
                            rect,
                            window,
                            0 as ::core::ffi::c_int,
                        );
                        return;
                    }
                }
                trayclient =
                    *(*((*trayclient).tailq.tqe_prev as *mut tc_head)).tqh_last as *mut trayclient;
            }
        }
        output = (*output).slist.sle_next as *mut i3_output;
    }
    if config.verbose {
        printf(
            b"[%s:%d] WARNING: Could not find corresponding tray window.\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"../i3bar/src/xcb.c\0" as *const u8 as *const ::core::ffi::c_char,
            1209 as ::core::ffi::c_int,
        );
    }
}
unsafe extern "C" fn handle_configure_request(mut event: *mut xcb_configure_request_event_t) {
    if config.verbose {
        printf(
            b"[%s:%d] ConfigureRequest for window = %08x\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"../i3bar/src/xcb.c\0" as *const u8 as *const ::core::ffi::c_char,
            1213 as ::core::ffi::c_int,
            (*event).window,
        );
    }
    handle_configuration_change((*event).window);
}
unsafe extern "C" fn handle_resize_request(mut event: *mut xcb_resize_request_event_t) {
    if config.verbose {
        printf(
            b"[%s:%d] ResizeRequest for window = %08x\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"../i3bar/src/xcb.c\0" as *const u8 as *const ::core::ffi::c_char,
            1218 as ::core::ffi::c_int,
            (*event).window,
        );
    }
    handle_configuration_change((*event).window);
}
unsafe extern "C" fn xcb_prep_cb(
    mut loop_0: *mut ev_loop,
    mut watcher: *mut ev_prepare,
    mut revents: ::core::ffi::c_int,
) {
    let mut event: *mut xcb_generic_event_t = ::core::ptr::null_mut::<xcb_generic_event_t>();
    if xcb_connection_has_error(xcb_connection) != 0 {
        fprintf(
            stderr,
            b"[%s:%d] ERROR: X11 connection was closed unexpectedly - maybe your X server terminated / crashed?\n\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"../i3bar/src/xcb.c\0" as *const u8 as *const ::core::ffi::c_char,
            1231 as ::core::ffi::c_int,
        );
        exit(1 as ::core::ffi::c_int);
    }
    loop {
        event = xcb_poll_for_event(xcb_connection);
        if event.is_null() {
            break;
        }
        if (*event).response_type as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            let mut error: *mut xcb_generic_error_t = event as *mut xcb_generic_error_t;
            if config.verbose {
                printf(
                    b"[%s:%d] Received X11 error, sequence 0x%x, error_code = %d\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    b"../i3bar/src/xcb.c\0" as *const u8 as *const ::core::ffi::c_char,
                    1241 as ::core::ffi::c_int,
                    (*error).sequence as ::core::ffi::c_int,
                    (*error).error_code as ::core::ffi::c_int,
                );
            }
            free(event as *mut ::core::ffi::c_void);
        } else {
            let mut type_0: ::core::ffi::c_int =
                (*event).response_type as ::core::ffi::c_int & !(0x80 as ::core::ffi::c_int);
            if type_0 == xkb_base && xkb_base > -(1 as ::core::ffi::c_int) {
                if config.verbose {
                    printf(
                        b"[%s:%d] received an xkb event\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                        b"../i3bar/src/xcb.c\0" as *const u8 as *const ::core::ffi::c_char,
                        1249 as ::core::ffi::c_int,
                    );
                }
                let mut state: *mut xcb_xkb_state_notify_event_t =
                    event as *mut xcb_xkb_state_notify_event_t;
                let mod_0: uint32_t = config.modifier & 0xffff as uint32_t;
                let new_mod_pressed: bool =
                    mod_0 != 0 as uint32_t && (*state).mods as uint32_t & mod_0 == mod_0;
                if new_mod_pressed as ::core::ffi::c_int != mod_pressed as ::core::ffi::c_int {
                    mod_pressed = new_mod_pressed;
                    if (*state).xkbType as ::core::ffi::c_int == XCB_XKB_STATE_NOTIFY
                        && config.modifier as ::core::ffi::c_long != XCB_NONE
                    {
                        if mod_pressed {
                            activated_mode = false_0 != 0;
                            unhide_bars();
                        } else if !activated_mode {
                            hide_bars();
                        }
                    }
                }
                free(event as *mut ::core::ffi::c_void);
            } else {
                match type_0 {
                    XCB_VISIBILITY_NOTIFY => {
                        handle_visibility_notify(event as *mut xcb_visibility_notify_event_t);
                    }
                    XCB_EXPOSE => {
                        if (*(event as *mut xcb_expose_event_t)).count as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                        {
                            redraw_bars();
                        }
                    }
                    XCB_BUTTON_RELEASE | XCB_BUTTON_PRESS => {
                        handle_button(event as *mut xcb_button_press_event_t);
                    }
                    XCB_CLIENT_MESSAGE => {
                        handle_client_message(event as *mut xcb_client_message_event_t);
                    }
                    XCB_DESTROY_NOTIFY => {
                        handle_destroy_notify(event as *mut xcb_destroy_notify_event_t);
                    }
                    XCB_UNMAP_NOTIFY => {
                        handle_unmap_notify(event as *mut xcb_unmap_notify_event_t);
                    }
                    XCB_MAP_NOTIFY => {
                        handle_map_notify(event as *mut xcb_map_notify_event_t);
                    }
                    XCB_PROPERTY_NOTIFY => {
                        handle_property_notify(event as *mut xcb_property_notify_event_t);
                    }
                    XCB_CONFIGURE_REQUEST => {
                        handle_configure_request(event as *mut xcb_configure_request_event_t);
                    }
                    XCB_RESIZE_REQUEST => {
                        handle_resize_request(event as *mut xcb_resize_request_event_t);
                    }
                    _ => {}
                }
                free(event as *mut ::core::ffi::c_void);
            }
        }
    }
    xcb_flush(xcb_connection);
}
unsafe extern "C" fn xcb_io_cb(
    mut loop_0: *mut ev_loop,
    mut watcher: *mut ev_io,
    mut revents: ::core::ffi::c_int,
) {
}
#[no_mangle]
pub unsafe extern "C" fn init_xcb_early() -> *mut ::core::ffi::c_char {
    xcb_connection = xcb_connect(::core::ptr::null::<::core::ffi::c_char>(), &raw mut screen);
    if xcb_connection_has_error(xcb_connection) != 0 {
        fprintf(
            stderr,
            b"[%s:%d] ERROR: Cannot open display\n\0" as *const u8 as *const ::core::ffi::c_char,
            b"../i3bar/src/xcb.c\0" as *const u8 as *const ::core::ffi::c_char,
            1339 as ::core::ffi::c_int,
        );
        exit(EXIT_FAILURE);
    }
    conn = xcb_connection;
    if config.verbose {
        printf(
            b"[%s:%d] Connected to xcb\n\0" as *const u8 as *const ::core::ffi::c_char,
            b"../i3bar/src/xcb.c\0" as *const u8 as *const ::core::ffi::c_char,
            1343 as ::core::ffi::c_int,
        );
    }
    atom_cookies[_NET_WM_WINDOW_TYPE as ::core::ffi::c_int as usize] = xcb_intern_atom(
        xcb_connection,
        0 as uint8_t,
        strlen(b"_NET_WM_WINDOW_TYPE\0" as *const u8 as *const ::core::ffi::c_char) as uint16_t,
        b"_NET_WM_WINDOW_TYPE\0" as *const u8 as *const ::core::ffi::c_char,
    );
    atom_cookies[_NET_WM_WINDOW_TYPE_DOCK as ::core::ffi::c_int as usize] = xcb_intern_atom(
        xcb_connection,
        0 as uint8_t,
        strlen(b"_NET_WM_WINDOW_TYPE_DOCK\0" as *const u8 as *const ::core::ffi::c_char)
            as uint16_t,
        b"_NET_WM_WINDOW_TYPE_DOCK\0" as *const u8 as *const ::core::ffi::c_char,
    );
    atom_cookies[_NET_WM_STRUT_PARTIAL as ::core::ffi::c_int as usize] = xcb_intern_atom(
        xcb_connection,
        0 as uint8_t,
        strlen(b"_NET_WM_STRUT_PARTIAL\0" as *const u8 as *const ::core::ffi::c_char) as uint16_t,
        b"_NET_WM_STRUT_PARTIAL\0" as *const u8 as *const ::core::ffi::c_char,
    );
    atom_cookies[I3_SOCKET_PATH as ::core::ffi::c_int as usize] = xcb_intern_atom(
        xcb_connection,
        0 as uint8_t,
        strlen(b"I3_SOCKET_PATH\0" as *const u8 as *const ::core::ffi::c_char) as uint16_t,
        b"I3_SOCKET_PATH\0" as *const u8 as *const ::core::ffi::c_char,
    );
    atom_cookies[MANAGER as ::core::ffi::c_int as usize] = xcb_intern_atom(
        xcb_connection,
        0 as uint8_t,
        strlen(b"MANAGER\0" as *const u8 as *const ::core::ffi::c_char) as uint16_t,
        b"MANAGER\0" as *const u8 as *const ::core::ffi::c_char,
    );
    atom_cookies[_NET_SYSTEM_TRAY_ORIENTATION as ::core::ffi::c_int as usize] = xcb_intern_atom(
        xcb_connection,
        0 as uint8_t,
        strlen(b"_NET_SYSTEM_TRAY_ORIENTATION\0" as *const u8 as *const ::core::ffi::c_char)
            as uint16_t,
        b"_NET_SYSTEM_TRAY_ORIENTATION\0" as *const u8 as *const ::core::ffi::c_char,
    );
    atom_cookies[_NET_SYSTEM_TRAY_VISUAL as ::core::ffi::c_int as usize] = xcb_intern_atom(
        xcb_connection,
        0 as uint8_t,
        strlen(b"_NET_SYSTEM_TRAY_VISUAL\0" as *const u8 as *const ::core::ffi::c_char) as uint16_t,
        b"_NET_SYSTEM_TRAY_VISUAL\0" as *const u8 as *const ::core::ffi::c_char,
    );
    atom_cookies[_NET_SYSTEM_TRAY_OPCODE as ::core::ffi::c_int as usize] = xcb_intern_atom(
        xcb_connection,
        0 as uint8_t,
        strlen(b"_NET_SYSTEM_TRAY_OPCODE\0" as *const u8 as *const ::core::ffi::c_char) as uint16_t,
        b"_NET_SYSTEM_TRAY_OPCODE\0" as *const u8 as *const ::core::ffi::c_char,
    );
    atom_cookies[_NET_SYSTEM_TRAY_COLORS as ::core::ffi::c_int as usize] = xcb_intern_atom(
        xcb_connection,
        0 as uint8_t,
        strlen(b"_NET_SYSTEM_TRAY_COLORS\0" as *const u8 as *const ::core::ffi::c_char) as uint16_t,
        b"_NET_SYSTEM_TRAY_COLORS\0" as *const u8 as *const ::core::ffi::c_char,
    );
    atom_cookies[_XEMBED_INFO as ::core::ffi::c_int as usize] = xcb_intern_atom(
        xcb_connection,
        0 as uint8_t,
        strlen(b"_XEMBED_INFO\0" as *const u8 as *const ::core::ffi::c_char) as uint16_t,
        b"_XEMBED_INFO\0" as *const u8 as *const ::core::ffi::c_char,
    );
    atom_cookies[_XEMBED as ::core::ffi::c_int as usize] = xcb_intern_atom(
        xcb_connection,
        0 as uint8_t,
        strlen(b"_XEMBED\0" as *const u8 as *const ::core::ffi::c_char) as uint16_t,
        b"_XEMBED\0" as *const u8 as *const ::core::ffi::c_char,
    );
    atom_cookies[I3_SYNC as ::core::ffi::c_int as usize] = xcb_intern_atom(
        xcb_connection,
        0 as uint8_t,
        strlen(b"I3_SYNC\0" as *const u8 as *const ::core::ffi::c_char) as uint16_t,
        b"I3_SYNC\0" as *const u8 as *const ::core::ffi::c_char,
    );
    root_screen = xcb_aux_get_screen(xcb_connection, screen);
    xcb_root = (*root_screen).root;
    depth = (*root_screen).root_depth;
    colormap = (*root_screen).default_colormap;
    visual_type = if config.transparency as ::core::ffi::c_int != 0 {
        xcb_aux_find_visual_by_attrs(
            root_screen,
            -(1 as ::core::ffi::c_int) as int8_t,
            32 as int8_t,
        )
    } else {
        ::core::ptr::null_mut::<xcb_visualtype_t>()
    };
    if !visual_type.is_null() {
        depth = xcb_aux_get_depth_of_visual(root_screen, (*visual_type).visual_id);
        colormap = xcb_generate_id(xcb_connection) as xcb_colormap_t;
        let mut cm_cookie: xcb_void_cookie_t = xcb_create_colormap_checked(
            xcb_connection,
            XCB_COLORMAP_ALLOC_NONE as ::core::ffi::c_int as uint8_t,
            colormap,
            xcb_root,
            (*visual_type).visual_id,
        );
        if _xcb_request_failed(
            cm_cookie,
            b"Could not allocate colormap\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            1363 as ::core::ffi::c_int,
        ) != 0
        {
            exit(EXIT_FAILURE);
        }
    } else {
        visual_type = get_visualtype(root_screen);
    }
    let mut cursor_ctx: *mut xcb_cursor_context_t = ::core::ptr::null_mut::<xcb_cursor_context_t>();
    if xcb_cursor_context_new(conn, root_screen, &raw mut cursor_ctx) < 0 as ::core::ffi::c_int {
        errx(
            EXIT_FAILURE,
            b"Cannot allocate xcursor context\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    cursor = xcb_cursor_load_cursor(
        cursor_ctx,
        b"left_ptr\0" as *const u8 as *const ::core::ffi::c_char,
    );
    xcb_cursor_context_free(cursor_ctx);
    xcb_io = smalloc(::core::mem::size_of::<ev_io>() as size_t) as *mut ev_io;
    xcb_prep = smalloc(::core::mem::size_of::<ev_prepare>() as size_t) as *mut ev_prepare;
    let ref mut fresh0 = (*(xcb_io as *mut ::core::ffi::c_void as *mut ev_watcher)).pending;
    *fresh0 = 0 as ::core::ffi::c_int;
    (*(xcb_io as *mut ::core::ffi::c_void as *mut ev_watcher)).active = *fresh0;
    (*(xcb_io as *mut ::core::ffi::c_void as *mut ev_watcher)).priority = 0 as ::core::ffi::c_int;
    (*xcb_io).cb =
        Some(xcb_io_cb as unsafe extern "C" fn(*mut ev_loop, *mut ev_io, ::core::ffi::c_int) -> ())
            as Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_io, ::core::ffi::c_int) -> ()>;
    memmove(
        &raw mut (*(xcb_io as *mut ev_watcher)).cb as *mut ::core::ffi::c_void,
        &raw mut (*xcb_io).cb as *const ::core::ffi::c_void,
        ::core::mem::size_of::<
            Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_io, ::core::ffi::c_int) -> ()>,
        >() as size_t,
    );
    (*xcb_io).fd = xcb_get_file_descriptor(xcb_connection);
    (*xcb_io).events = EV_READ as ::core::ffi::c_int | EV__IOFDSET as ::core::ffi::c_int;
    let ref mut fresh1 = (*(xcb_prep as *mut ::core::ffi::c_void as *mut ev_watcher)).pending;
    *fresh1 = 0 as ::core::ffi::c_int;
    (*(xcb_prep as *mut ::core::ffi::c_void as *mut ev_watcher)).active = *fresh1;
    (*(xcb_prep as *mut ::core::ffi::c_void as *mut ev_watcher)).priority = 0 as ::core::ffi::c_int;
    (*xcb_prep).cb = Some(
        xcb_prep_cb
            as unsafe extern "C" fn(*mut ev_loop, *mut ev_prepare, ::core::ffi::c_int) -> (),
    )
        as Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_prepare, ::core::ffi::c_int) -> ()>;
    memmove(
        &raw mut (*(xcb_prep as *mut ev_watcher)).cb as *mut ::core::ffi::c_void,
        &raw mut (*xcb_prep).cb as *const ::core::ffi::c_void,
        ::core::mem::size_of::<
            Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_prepare, ::core::ffi::c_int) -> ()>,
        >() as size_t,
    );
    ev_io_start(main_loop as *mut ev_loop, xcb_io);
    ev_prepare_start(main_loop as *mut ev_loop, xcb_prep);
    get_atoms();
    let mut path: *mut ::core::ffi::c_char = root_atom_contents(
        b"I3_SOCKET_PATH\0" as *const u8 as *const ::core::ffi::c_char,
        xcb_connection,
        screen,
    );
    return path;
}
unsafe extern "C" fn register_xkb_keyevents() {
    let mut extreply: *const xcb_query_extension_reply_t =
        ::core::ptr::null::<xcb_query_extension_reply_t>();
    extreply =
        xcb_get_extension_data(conn, &raw mut xcb_xkb_id) as *const xcb_query_extension_reply_t;
    if (*extreply).present == 0 {
        fprintf(
            stderr,
            b"[%s:%d] ERROR: xkb is not present on this server\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"../i3bar/src/xcb.c\0" as *const u8 as *const ::core::ffi::c_char,
            1405 as ::core::ffi::c_int,
        );
        exit(EXIT_FAILURE);
    }
    if config.verbose {
        printf(
            b"[%s:%d] initializing xcb-xkb\n\0" as *const u8 as *const ::core::ffi::c_char,
            b"../i3bar/src/xcb.c\0" as *const u8 as *const ::core::ffi::c_char,
            1408 as ::core::ffi::c_int,
        );
    }
    xcb_xkb_use_extension(
        conn,
        XCB_XKB_MAJOR_VERSION as uint16_t,
        XCB_XKB_MINOR_VERSION as uint16_t,
    );
    xcb_xkb_select_events(
        conn,
        XCB_XKB_ID_USE_CORE_KBD as ::core::ffi::c_int as xcb_xkb_device_spec_t,
        XCB_XKB_EVENT_TYPE_STATE_NOTIFY as ::core::ffi::c_int as uint16_t,
        0 as uint16_t,
        XCB_XKB_EVENT_TYPE_STATE_NOTIFY as ::core::ffi::c_int as uint16_t,
        0xff as uint16_t,
        0xff as uint16_t,
        ::core::ptr::null::<::core::ffi::c_void>(),
    );
    xkb_base = (*extreply).first_event as ::core::ffi::c_int;
}
unsafe extern "C" fn deregister_xkb_keyevents() {
    xcb_xkb_select_events(
        conn,
        XCB_XKB_ID_USE_CORE_KBD as ::core::ffi::c_int as xcb_xkb_device_spec_t,
        0 as uint16_t,
        0 as uint16_t,
        0 as uint16_t,
        0xff as uint16_t,
        0xff as uint16_t,
        ::core::ptr::null::<::core::ffi::c_void>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn init_xcb_late(mut fontname: *mut ::core::ffi::c_char) {
    if fontname.is_null() {
        fontname = b"-misc-fixed-medium-r-normal--13-120-75-75-C-70-iso10646-1\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
    }
    font = load_font(fontname, true_0 != 0);
    set_font(&raw mut font);
    if config.verbose {
        printf(
            b"[%s:%d] Calculated font-height: %d\n\0" as *const u8 as *const ::core::ffi::c_char,
            b"../i3bar/src/xcb.c\0" as *const u8 as *const ::core::ffi::c_char,
            1449 as ::core::ffi::c_int,
            font.height,
        );
    }
    let default_px: ::core::ffi::c_int =
        font.height + 2 as ::core::ffi::c_int * logical_px(ws_voff_px) as ::core::ffi::c_int;
    let mut padding_scaled: ::core::ffi::c_int = logical_px(config.padding.y as ::core::ffi::c_int)
        + logical_px(config.padding.height as ::core::ffi::c_int);
    if config.bar_height > 0 as uint32_t
        && config.padding.x == 0 as uint32_t
        && config.padding.y == 0 as uint32_t
        && config.padding.width == 0 as uint32_t
        && config.padding.height == 0 as uint32_t
    {
        padding_scaled =
            config.bar_height.wrapping_sub(default_px as uint32_t) as ::core::ffi::c_int;
        if config.verbose {
            printf(
                b"[%s:%d] setting padding_scaled=%d based on bar_height=%d\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"../i3bar/src/xcb.c\0" as *const u8 as *const ::core::ffi::c_char,
                1465 as ::core::ffi::c_int,
                padding_scaled,
                config.bar_height,
            );
        }
    } else if config.verbose {
        printf(
            b"[%s:%d] padding: x=%d, y=%d -> padding_scaled=%d\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"../i3bar/src/xcb.c\0" as *const u8 as *const ::core::ffi::c_char,
            1467 as ::core::ffi::c_int,
            config.padding.x,
            config.padding.y,
            padding_scaled,
        );
    }
    bar_height = default_px + padding_scaled;
    icon_size = bar_height - 2 as ::core::ffi::c_int * logical_px(config.tray_padding);
    if !config.separator_symbol.is_null() {
        separator_symbol_width = predict_text_width(config.separator_symbol);
    }
    xcb_flush(xcb_connection);
    if config.hide_on_modifier as ::core::ffi::c_uint
        == M_HIDE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        register_xkb_keyevents();
    }
}
unsafe extern "C" fn send_tray_clientmessage() {
    let mut buffer: [uint8_t; 32] = [
        0 as ::core::ffi::c_int as uint8_t,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut ev: *mut xcb_client_message_event_t =
        &raw mut buffer as *mut uint8_t as *mut xcb_client_message_event_t;
    (*ev).response_type = XCB_CLIENT_MESSAGE as uint8_t;
    (*ev).window = xcb_root;
    (*ev).type_0 = atoms[MANAGER as ::core::ffi::c_int as usize];
    (*ev).format = 32 as uint8_t;
    (*ev).data.data32[0 as ::core::ffi::c_int as usize] = XCB_CURRENT_TIME as uint32_t;
    (*ev).data.data32[1 as ::core::ffi::c_int as usize] = (*tray_reply).atom as uint32_t;
    (*ev).data.data32[2 as ::core::ffi::c_int as usize] = selwin as uint32_t;
    xcb_send_event(
        xcb_connection,
        0 as uint8_t,
        xcb_root,
        0xffffff as uint32_t,
        &raw mut buffer as *mut uint8_t as *mut ::core::ffi::c_char,
    );
}
unsafe extern "C" fn init_tray() {
    if config.verbose {
        printf(
            b"[%s:%d] Initializing system tray functionality\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"../i3bar/src/xcb.c\0" as *const u8 as *const ::core::ffi::c_char,
            1514 as ::core::ffi::c_int,
        );
    }
    let systray_len: size_t =
        (strlen(b"_NET_SYSTEM_TRAY_S\0" as *const u8 as *const ::core::ffi::c_char) as size_t)
            .wrapping_add(11 as size_t);
    let vla = systray_len as usize;
    let mut atomname: Vec<::core::ffi::c_char> = ::std::vec::from_elem(0, vla);
    snprintf(
        atomname.as_mut_ptr(),
        systray_len,
        b"_NET_SYSTEM_TRAY_S%d\0" as *const u8 as *const ::core::ffi::c_char,
        screen,
    );
    let mut tray_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom_cookie_t { sequence: 0 };
    if tray_reply.is_null() {
        tray_cookie = xcb_intern_atom(
            xcb_connection,
            0 as uint8_t,
            strlen(atomname.as_mut_ptr()) as uint16_t,
            atomname.as_mut_ptr(),
        );
    }
    selwin = xcb_generate_id(xcb_connection) as xcb_window_t;
    let mut selmask: uint32_t = (XCB_CW_BACK_PIXEL as ::core::ffi::c_int
        | XCB_CW_BORDER_PIXEL as ::core::ffi::c_int
        | XCB_CW_OVERRIDE_REDIRECT as ::core::ffi::c_int
        | XCB_CW_COLORMAP as ::core::ffi::c_int) as uint32_t;
    let mut selval: [uint32_t; 4] = [
        (*root_screen).black_pixel,
        (*root_screen).black_pixel,
        1 as ::core::ffi::c_int as uint32_t,
        colormap,
    ];
    xcb_create_window(
        xcb_connection,
        depth,
        selwin,
        xcb_root,
        -(1 as ::core::ffi::c_int) as int16_t,
        -(1 as ::core::ffi::c_int) as int16_t,
        1 as uint16_t,
        1 as uint16_t,
        0 as uint16_t,
        XCB_WINDOW_CLASS_INPUT_OUTPUT as ::core::ffi::c_int as uint16_t,
        (*visual_type).visual_id,
        selmask,
        &raw mut selval as *mut uint32_t as *const ::core::ffi::c_void,
    );
    let mut orientation: uint32_t = _NET_SYSTEM_TRAY_ORIENTATION_HORZ as uint32_t;
    xcb_change_property(
        xcb_connection,
        XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as uint8_t,
        selwin,
        atoms[_NET_SYSTEM_TRAY_ORIENTATION as ::core::ffi::c_int as usize],
        XCB_ATOM_CARDINAL as ::core::ffi::c_int as xcb_atom_t,
        32 as uint8_t,
        1 as uint32_t,
        &raw mut orientation as *const ::core::ffi::c_void,
    );
    xcb_change_property(
        xcb_connection,
        XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as uint8_t,
        selwin,
        atoms[_NET_SYSTEM_TRAY_VISUAL as ::core::ffi::c_int as usize],
        XCB_ATOM_VISUALID as ::core::ffi::c_int as xcb_atom_t,
        32 as uint8_t,
        1 as uint32_t,
        &raw mut (*visual_type).visual_id as *const ::core::ffi::c_void,
    );
    init_tray_colors();
    if tray_reply.is_null() {
        tray_reply = xcb_intern_atom_reply(
            xcb_connection,
            tray_cookie,
            ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
        );
        if tray_reply.is_null() {
            fprintf(
                stderr,
                b"[%s:%d] ERROR: Could not get atom %s\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"../i3bar/src/xcb.c\0" as *const u8 as *const ::core::ffi::c_char,
                1565 as ::core::ffi::c_int,
                atomname.as_mut_ptr(),
            );
            exit(EXIT_FAILURE);
        }
    }
    xcb_set_selection_owner(
        xcb_connection,
        selwin,
        (*tray_reply).atom,
        XCB_CURRENT_TIME as xcb_timestamp_t,
    );
    let mut selcookie: xcb_get_selection_owner_cookie_t =
        xcb_get_selection_owner_cookie_t { sequence: 0 };
    let mut selreply: *mut xcb_get_selection_owner_reply_t =
        ::core::ptr::null_mut::<xcb_get_selection_owner_reply_t>();
    selcookie = xcb_get_selection_owner(xcb_connection, (*tray_reply).atom);
    selreply = xcb_get_selection_owner_reply(
        xcb_connection,
        selcookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if selreply.is_null() {
        fprintf(
            stderr,
            b"[%s:%d] ERROR: Could not get selection owner for %s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"../i3bar/src/xcb.c\0" as *const u8 as *const ::core::ffi::c_char,
            1581 as ::core::ffi::c_int,
            atomname.as_mut_ptr(),
        );
        exit(EXIT_FAILURE);
    }
    if (*selreply).owner != selwin {
        fprintf(
            stderr,
            b"[%s:%d] ERROR: Could not set the %s selection. Maybe another tray is already running?\n\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"../i3bar/src/xcb.c\0" as *const u8 as *const ::core::ffi::c_char,
            1588 as ::core::ffi::c_int,
            atomname.as_mut_ptr(),
        );
        free(selreply as *mut ::core::ffi::c_void);
        return;
    }
    free(selreply as *mut ::core::ffi::c_void);
    send_tray_clientmessage();
}
#[no_mangle]
pub unsafe extern "C" fn init_tray_colors() {
    let mut bar_fg: *const ::core::ffi::c_char = if !config.colors.bar_fg.is_null() {
        config.colors.bar_fg as *const ::core::ffi::c_char
    } else {
        b"#FFFFFF\0" as *const u8 as *const ::core::ffi::c_char
    };
    if config.verbose {
        printf(
            b"[%s:%d] Setting bar_fg = %s as _NET_SYSTEM_TRAY_COLORS\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"../i3bar/src/xcb.c\0" as *const u8 as *const ::core::ffi::c_char,
            1611 as ::core::ffi::c_int,
            bar_fg,
        );
    }
    let mut strgroups: [[::core::ffi::c_char; 3]; 3] = [
        [
            *bar_fg.offset(1 as ::core::ffi::c_int as isize),
            *bar_fg.offset(2 as ::core::ffi::c_int as isize),
            '\0' as i32 as ::core::ffi::c_char,
        ],
        [
            *bar_fg.offset(3 as ::core::ffi::c_int as isize),
            *bar_fg.offset(4 as ::core::ffi::c_int as isize),
            '\0' as i32 as ::core::ffi::c_char,
        ],
        [
            *bar_fg.offset(5 as ::core::ffi::c_int as isize),
            *bar_fg.offset(6 as ::core::ffi::c_int as isize),
            '\0' as i32 as ::core::ffi::c_char,
        ],
    ];
    let r: uint8_t = strtol(
        &raw mut *(&raw mut strgroups as *mut [::core::ffi::c_char; 3])
            .offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        16 as ::core::ffi::c_int,
    ) as uint8_t;
    let g: uint8_t = strtol(
        &raw mut *(&raw mut strgroups as *mut [::core::ffi::c_char; 3])
            .offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        16 as ::core::ffi::c_int,
    ) as uint8_t;
    let b: uint8_t = strtol(
        &raw mut *(&raw mut strgroups as *mut [::core::ffi::c_char; 3])
            .offset(2 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        16 as ::core::ffi::c_int,
    ) as uint8_t;
    let r16: uint16_t = ((r as uint16_t as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
        | r as ::core::ffi::c_int) as uint16_t;
    let g16: uint16_t = ((g as uint16_t as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
        | g as ::core::ffi::c_int) as uint16_t;
    let b16: uint16_t = ((b as uint16_t as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
        | b as ::core::ffi::c_int) as uint16_t;
    let tray_colors: [uint32_t; 12] = [
        r16 as uint32_t,
        g16 as uint32_t,
        b16 as uint32_t,
        r16 as uint32_t,
        g16 as uint32_t,
        b16 as uint32_t,
        r16 as uint32_t,
        g16 as uint32_t,
        b16 as uint32_t,
        r16 as uint32_t,
        g16 as uint32_t,
        b16 as uint32_t,
    ];
    xcb_change_property(
        xcb_connection,
        XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as uint8_t,
        selwin,
        atoms[_NET_SYSTEM_TRAY_COLORS as ::core::ffi::c_int as usize],
        XCB_ATOM_CARDINAL as ::core::ffi::c_int as xcb_atom_t,
        32 as uint8_t,
        12 as uint32_t,
        &raw const tray_colors as *const uint32_t as *const ::core::ffi::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn clean_xcb() {
    free_outputs();
    free_font();
    xcb_free_cursor(xcb_connection, cursor);
    xcb_aux_sync(xcb_connection);
    xcb_disconnect(xcb_connection);
    ev_prepare_stop(main_loop as *mut ev_loop, xcb_prep);
    ev_io_stop(main_loop as *mut ev_loop, xcb_io);
    free(xcb_prep as *mut ::core::ffi::c_void);
    xcb_prep = ::core::ptr::null_mut::<ev_prepare>();
    free(xcb_io as *mut ::core::ffi::c_void);
    xcb_io = ::core::ptr::null_mut::<ev_io>();
}
#[no_mangle]
pub unsafe extern "C" fn get_atoms() {
    let mut reply: *mut xcb_intern_atom_reply_t =
        ::core::ptr::null_mut::<xcb_intern_atom_reply_t>();
    reply = xcb_intern_atom_reply(
        xcb_connection,
        atom_cookies[_NET_WM_WINDOW_TYPE as ::core::ffi::c_int as usize],
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply.is_null() {
        fprintf(
            stderr,
            b"[%s:%d] ERROR: Could not get atom %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            b"../i3bar/include/xcb_atoms.def\0" as *const u8 as *const ::core::ffi::c_char,
            1 as ::core::ffi::c_int,
            b"_NET_WM_WINDOW_TYPE\0" as *const u8 as *const ::core::ffi::c_char,
        );
        exit(EXIT_FAILURE);
    }
    atoms[_NET_WM_WINDOW_TYPE as ::core::ffi::c_int as usize] = (*reply).atom;
    free(reply as *mut ::core::ffi::c_void);
    reply = xcb_intern_atom_reply(
        xcb_connection,
        atom_cookies[_NET_WM_WINDOW_TYPE_DOCK as ::core::ffi::c_int as usize],
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply.is_null() {
        fprintf(
            stderr,
            b"[%s:%d] ERROR: Could not get atom %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            b"../i3bar/include/xcb_atoms.def\0" as *const u8 as *const ::core::ffi::c_char,
            2 as ::core::ffi::c_int,
            b"_NET_WM_WINDOW_TYPE_DOCK\0" as *const u8 as *const ::core::ffi::c_char,
        );
        exit(EXIT_FAILURE);
    }
    atoms[_NET_WM_WINDOW_TYPE_DOCK as ::core::ffi::c_int as usize] = (*reply).atom;
    free(reply as *mut ::core::ffi::c_void);
    reply = xcb_intern_atom_reply(
        xcb_connection,
        atom_cookies[_NET_WM_STRUT_PARTIAL as ::core::ffi::c_int as usize],
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply.is_null() {
        fprintf(
            stderr,
            b"[%s:%d] ERROR: Could not get atom %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            b"../i3bar/include/xcb_atoms.def\0" as *const u8 as *const ::core::ffi::c_char,
            3 as ::core::ffi::c_int,
            b"_NET_WM_STRUT_PARTIAL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        exit(EXIT_FAILURE);
    }
    atoms[_NET_WM_STRUT_PARTIAL as ::core::ffi::c_int as usize] = (*reply).atom;
    free(reply as *mut ::core::ffi::c_void);
    reply = xcb_intern_atom_reply(
        xcb_connection,
        atom_cookies[I3_SOCKET_PATH as ::core::ffi::c_int as usize],
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply.is_null() {
        fprintf(
            stderr,
            b"[%s:%d] ERROR: Could not get atom %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            b"../i3bar/include/xcb_atoms.def\0" as *const u8 as *const ::core::ffi::c_char,
            4 as ::core::ffi::c_int,
            b"I3_SOCKET_PATH\0" as *const u8 as *const ::core::ffi::c_char,
        );
        exit(EXIT_FAILURE);
    }
    atoms[I3_SOCKET_PATH as ::core::ffi::c_int as usize] = (*reply).atom;
    free(reply as *mut ::core::ffi::c_void);
    reply = xcb_intern_atom_reply(
        xcb_connection,
        atom_cookies[MANAGER as ::core::ffi::c_int as usize],
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply.is_null() {
        fprintf(
            stderr,
            b"[%s:%d] ERROR: Could not get atom %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            b"../i3bar/include/xcb_atoms.def\0" as *const u8 as *const ::core::ffi::c_char,
            5 as ::core::ffi::c_int,
            b"MANAGER\0" as *const u8 as *const ::core::ffi::c_char,
        );
        exit(EXIT_FAILURE);
    }
    atoms[MANAGER as ::core::ffi::c_int as usize] = (*reply).atom;
    free(reply as *mut ::core::ffi::c_void);
    reply = xcb_intern_atom_reply(
        xcb_connection,
        atom_cookies[_NET_SYSTEM_TRAY_ORIENTATION as ::core::ffi::c_int as usize],
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply.is_null() {
        fprintf(
            stderr,
            b"[%s:%d] ERROR: Could not get atom %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            b"../i3bar/include/xcb_atoms.def\0" as *const u8 as *const ::core::ffi::c_char,
            6 as ::core::ffi::c_int,
            b"_NET_SYSTEM_TRAY_ORIENTATION\0" as *const u8 as *const ::core::ffi::c_char,
        );
        exit(EXIT_FAILURE);
    }
    atoms[_NET_SYSTEM_TRAY_ORIENTATION as ::core::ffi::c_int as usize] = (*reply).atom;
    free(reply as *mut ::core::ffi::c_void);
    reply = xcb_intern_atom_reply(
        xcb_connection,
        atom_cookies[_NET_SYSTEM_TRAY_VISUAL as ::core::ffi::c_int as usize],
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply.is_null() {
        fprintf(
            stderr,
            b"[%s:%d] ERROR: Could not get atom %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            b"../i3bar/include/xcb_atoms.def\0" as *const u8 as *const ::core::ffi::c_char,
            7 as ::core::ffi::c_int,
            b"_NET_SYSTEM_TRAY_VISUAL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        exit(EXIT_FAILURE);
    }
    atoms[_NET_SYSTEM_TRAY_VISUAL as ::core::ffi::c_int as usize] = (*reply).atom;
    free(reply as *mut ::core::ffi::c_void);
    reply = xcb_intern_atom_reply(
        xcb_connection,
        atom_cookies[_NET_SYSTEM_TRAY_OPCODE as ::core::ffi::c_int as usize],
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply.is_null() {
        fprintf(
            stderr,
            b"[%s:%d] ERROR: Could not get atom %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            b"../i3bar/include/xcb_atoms.def\0" as *const u8 as *const ::core::ffi::c_char,
            8 as ::core::ffi::c_int,
            b"_NET_SYSTEM_TRAY_OPCODE\0" as *const u8 as *const ::core::ffi::c_char,
        );
        exit(EXIT_FAILURE);
    }
    atoms[_NET_SYSTEM_TRAY_OPCODE as ::core::ffi::c_int as usize] = (*reply).atom;
    free(reply as *mut ::core::ffi::c_void);
    reply = xcb_intern_atom_reply(
        xcb_connection,
        atom_cookies[_NET_SYSTEM_TRAY_COLORS as ::core::ffi::c_int as usize],
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply.is_null() {
        fprintf(
            stderr,
            b"[%s:%d] ERROR: Could not get atom %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            b"../i3bar/include/xcb_atoms.def\0" as *const u8 as *const ::core::ffi::c_char,
            9 as ::core::ffi::c_int,
            b"_NET_SYSTEM_TRAY_COLORS\0" as *const u8 as *const ::core::ffi::c_char,
        );
        exit(EXIT_FAILURE);
    }
    atoms[_NET_SYSTEM_TRAY_COLORS as ::core::ffi::c_int as usize] = (*reply).atom;
    free(reply as *mut ::core::ffi::c_void);
    reply = xcb_intern_atom_reply(
        xcb_connection,
        atom_cookies[_XEMBED_INFO as ::core::ffi::c_int as usize],
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply.is_null() {
        fprintf(
            stderr,
            b"[%s:%d] ERROR: Could not get atom %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            b"../i3bar/include/xcb_atoms.def\0" as *const u8 as *const ::core::ffi::c_char,
            10 as ::core::ffi::c_int,
            b"_XEMBED_INFO\0" as *const u8 as *const ::core::ffi::c_char,
        );
        exit(EXIT_FAILURE);
    }
    atoms[_XEMBED_INFO as ::core::ffi::c_int as usize] = (*reply).atom;
    free(reply as *mut ::core::ffi::c_void);
    reply = xcb_intern_atom_reply(
        xcb_connection,
        atom_cookies[_XEMBED as ::core::ffi::c_int as usize],
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply.is_null() {
        fprintf(
            stderr,
            b"[%s:%d] ERROR: Could not get atom %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            b"../i3bar/include/xcb_atoms.def\0" as *const u8 as *const ::core::ffi::c_char,
            11 as ::core::ffi::c_int,
            b"_XEMBED\0" as *const u8 as *const ::core::ffi::c_char,
        );
        exit(EXIT_FAILURE);
    }
    atoms[_XEMBED as ::core::ffi::c_int as usize] = (*reply).atom;
    free(reply as *mut ::core::ffi::c_void);
    reply = xcb_intern_atom_reply(
        xcb_connection,
        atom_cookies[I3_SYNC as ::core::ffi::c_int as usize],
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply.is_null() {
        fprintf(
            stderr,
            b"[%s:%d] ERROR: Could not get atom %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            b"../i3bar/include/xcb_atoms.def\0" as *const u8 as *const ::core::ffi::c_char,
            12 as ::core::ffi::c_int,
            b"I3_SYNC\0" as *const u8 as *const ::core::ffi::c_char,
        );
        exit(EXIT_FAILURE);
    }
    atoms[I3_SYNC as ::core::ffi::c_int as usize] = (*reply).atom;
    free(reply as *mut ::core::ffi::c_void);
    if config.verbose {
        printf(
            b"[%s:%d] Got atoms\n\0" as *const u8 as *const ::core::ffi::c_char,
            b"../i3bar/src/xcb.c\0" as *const u8 as *const ::core::ffi::c_char,
            1678 as ::core::ffi::c_int,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn kick_tray_clients(mut output: *mut i3_output) {
    if (*(*output).trayclients).tqh_first.is_null() {
        return;
    }
    let mut trayclient: *mut trayclient = ::core::ptr::null_mut::<trayclient>();
    while !(*(*output).trayclients).tqh_first.is_null() {
        trayclient = (*(*output).trayclients).tqh_first as *mut trayclient;
        xcb_unmap_window(xcb_connection, (*trayclient).win);
        xcb_reparent_window(
            xcb_connection,
            (*trayclient).win,
            xcb_root,
            0 as int16_t,
            0 as int16_t,
        );
        free((*trayclient).class_class as *mut ::core::ffi::c_void);
        free((*trayclient).class_instance as *mut ::core::ffi::c_void);
        if !(*trayclient).tailq.tqe_next.is_null() {
            (*(*trayclient).tailq.tqe_next).tailq.tqe_prev = (*trayclient).tailq.tqe_prev;
        } else {
            (*(*output).trayclients).tqh_last = (*trayclient).tailq.tqe_prev;
        }
        *(*trayclient).tailq.tqe_prev = (*trayclient).tailq.tqe_next;
        free(trayclient as *mut ::core::ffi::c_void);
        trayclient = ::core::ptr::null_mut::<trayclient>();
    }
    let mut buffer: [uint8_t; 32] = [
        0 as ::core::ffi::c_int as uint8_t,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut event: *mut xcb_destroy_notify_event_t =
        &raw mut buffer as *mut uint8_t as *mut xcb_destroy_notify_event_t;
    (*event).response_type = XCB_DESTROY_NOTIFY as uint8_t;
    (*event).event = selwin;
    (*event).window = selwin;
    xcb_send_event(
        conn,
        false_0 as uint8_t,
        selwin,
        XCB_EVENT_MASK_STRUCTURE_NOTIFY as ::core::ffi::c_int as uint32_t,
        event as *mut ::core::ffi::c_char,
    );
    send_tray_clientmessage();
}
#[no_mangle]
pub unsafe extern "C" fn destroy_window(mut output: *mut i3_output) {
    if output.is_null() || (*output).bar.id as ::core::ffi::c_long == XCB_NONE {
        return;
    }
    kick_tray_clients(output);
    draw_util_surface_free(xcb_connection, &raw mut (*output).bar);
    draw_util_surface_free(xcb_connection, &raw mut (*output).buffer);
    draw_util_surface_free(xcb_connection, &raw mut (*output).statusline_buffer);
    xcb_destroy_window(xcb_connection, (*output).bar.id as xcb_window_t);
    xcb_free_pixmap(xcb_connection, (*output).buffer.id as xcb_pixmap_t);
    xcb_free_pixmap(
        xcb_connection,
        (*output).statusline_buffer.id as xcb_pixmap_t,
    );
    (*output).bar.id = XCB_NONE as xcb_drawable_t;
}
unsafe extern "C" fn config_strut_partial(mut output: *mut i3_output) -> xcb_void_cookie_t {
    let mut strut_partial: C2RustUnnamed_10 = C2RustUnnamed_10 {
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
        ::core::mem::size_of::<C2RustUnnamed_10>() as size_t,
    );
    match config.position as ::core::ffi::c_uint {
        1 => {
            strut_partial.top = bar_height as uint32_t;
            strut_partial.top_start_x = (*output).rect.x as uint32_t;
            strut_partial.top_end_x = ((*output).rect.x + (*output).rect.w) as uint32_t;
        }
        2 => {
            strut_partial.bottom = bar_height as uint32_t;
            strut_partial.bottom_start_x = (*output).rect.x as uint32_t;
            strut_partial.bottom_end_x = ((*output).rect.x + (*output).rect.w) as uint32_t;
        }
        0 | _ => {}
    }
    return xcb_change_property(
        xcb_connection,
        XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as uint8_t,
        (*output).bar.id as xcb_window_t,
        atoms[_NET_WM_STRUT_PARTIAL as ::core::ffi::c_int as usize],
        XCB_ATOM_CARDINAL as ::core::ffi::c_int as xcb_atom_t,
        32 as uint8_t,
        12 as uint32_t,
        &raw mut strut_partial as *const ::core::ffi::c_void,
    );
}
unsafe extern "C" fn get_tray_output() -> *mut i3_output {
    let mut output: *mut i3_output = ::core::ptr::null_mut::<i3_output>();
    if config.tray_outputs.tqh_first.is_null() {
        output = (*outputs).slh_first as *mut i3_output;
        while !output.is_null() {
            if (*output).active {
                return output;
            }
            output = (*output).slist.sle_next as *mut i3_output;
        }
        return ::core::ptr::null_mut::<i3_output>();
    } else if strcasecmp(
        (*config.tray_outputs.tqh_first).output,
        b"none\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        return ::core::ptr::null_mut::<i3_output>();
    }
    let mut tray_output: *mut tray_output_t = ::core::ptr::null_mut::<tray_output_t>();
    tray_output = config.tray_outputs.tqh_first as *mut tray_output_t;
    while !tray_output.is_null() {
        output = (*outputs).slh_first as *mut i3_output;
        while !output.is_null() {
            if (*output).active as ::core::ffi::c_int != 0
                && (strcasecmp((*output).name, (*tray_output).output) == 0 as ::core::ffi::c_int
                    || strcasecmp(
                        (*tray_output).output,
                        b"primary\0" as *const u8 as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                        && (*output).primary as ::core::ffi::c_int != 0)
            {
                return output;
            }
            output = (*output).slist.sle_next as *mut i3_output;
        }
        tray_output = (*tray_output).tray_outputs.tqe_next as *mut tray_output_t;
    }
    return ::core::ptr::null_mut::<i3_output>();
}
#[no_mangle]
pub unsafe extern "C" fn reconfig_windows(mut redraw_bars_0: bool) {
    let mut mask: uint32_t = 0;
    let mut values: [uint32_t; 6] = [0; 6];
    let mut walk: *mut i3_output = ::core::ptr::null_mut::<i3_output>();
    walk = (*outputs).slh_first as *mut i3_output;
    while !walk.is_null() {
        if !(*walk).active {
            if config.verbose {
                printf(
                    b"[%s:%d] Destroying window for output %s\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    b"../i3bar/src/xcb.c\0" as *const u8 as *const ::core::ffi::c_char,
                    1857 as ::core::ffi::c_int,
                    (*walk).name,
                );
            }
            destroy_window(walk);
        } else if (*walk).bar.id as ::core::ffi::c_long == XCB_NONE {
            if config.verbose {
                printf(
                    b"[%s:%d] Creating window for output %s\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    b"../i3bar/src/xcb.c\0" as *const u8 as *const ::core::ffi::c_char,
                    1862 as ::core::ffi::c_int,
                    (*walk).name,
                );
            }
            let mut bar_id: xcb_window_t = xcb_generate_id(xcb_connection) as xcb_window_t;
            let mut buffer_id: xcb_pixmap_t = xcb_generate_id(xcb_connection) as xcb_pixmap_t;
            let mut statusline_buffer_id: xcb_pixmap_t =
                xcb_generate_id(xcb_connection) as xcb_pixmap_t;
            mask = (XCB_CW_BACK_PIXEL as ::core::ffi::c_int
                | XCB_CW_BORDER_PIXEL as ::core::ffi::c_int
                | XCB_CW_OVERRIDE_REDIRECT as ::core::ffi::c_int
                | XCB_CW_EVENT_MASK as ::core::ffi::c_int
                | XCB_CW_COLORMAP as ::core::ffi::c_int
                | XCB_CW_CURSOR as ::core::ffi::c_int) as uint32_t;
            values[0 as ::core::ffi::c_int as usize] = colors.bar_bg.colorpixel;
            values[1 as ::core::ffi::c_int as usize] = (*root_screen).black_pixel;
            values[2 as ::core::ffi::c_int as usize] = (if config.hide_on_modifier
                as ::core::ffi::c_uint
                == M_DOCK as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                0 as ::core::ffi::c_int
            } else {
                1 as ::core::ffi::c_int
            }) as uint32_t;
            values[3 as ::core::ffi::c_int as usize] = (XCB_EVENT_MASK_EXPOSURE
                as ::core::ffi::c_int
                | XCB_EVENT_MASK_SUBSTRUCTURE_REDIRECT as ::core::ffi::c_int
                | XCB_EVENT_MASK_BUTTON_PRESS as ::core::ffi::c_int
                | XCB_EVENT_MASK_BUTTON_RELEASE as ::core::ffi::c_int)
                as uint32_t;
            if config.hide_on_modifier as ::core::ffi::c_uint
                == M_DOCK as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                values[3 as ::core::ffi::c_int as usize] |=
                    XCB_EVENT_MASK_VISIBILITY_CHANGE as ::core::ffi::c_int as uint32_t;
                (*walk).visible = true_0 != 0;
            }
            values[4 as ::core::ffi::c_int as usize] = colormap as uint32_t;
            values[5 as ::core::ffi::c_int as usize] = cursor as uint32_t;
            let mut win_cookie: xcb_void_cookie_t = xcb_create_window_checked(
                xcb_connection,
                depth,
                bar_id,
                xcb_root,
                (*walk).rect.x as int16_t,
                ((*walk).rect.y + (*walk).rect.h - bar_height) as int16_t,
                (*walk).rect.w as uint16_t,
                bar_height as uint16_t,
                0 as uint16_t,
                XCB_WINDOW_CLASS_INPUT_OUTPUT as ::core::ffi::c_int as uint16_t,
                (*visual_type).visual_id,
                mask,
                &raw mut values as *mut uint32_t as *const ::core::ffi::c_void,
            );
            let mut pm_cookie: xcb_void_cookie_t = xcb_create_pixmap_checked(
                xcb_connection,
                depth,
                buffer_id,
                bar_id as xcb_drawable_t,
                (*walk).rect.w as uint16_t,
                bar_height as uint16_t,
            );
            let mut slpm_cookie: xcb_void_cookie_t = xcb_create_pixmap_checked(
                xcb_connection,
                depth,
                statusline_buffer_id,
                bar_id as xcb_drawable_t,
                (*walk).rect.w as uint16_t,
                bar_height as uint16_t,
            );
            let mut class: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            let mut len: ::core::ffi::c_int = sasprintf(
                &raw mut class,
                b"%s%ci3bar%c\0" as *const u8 as *const ::core::ffi::c_char,
                config.bar_id,
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
            );
            let mut class_cookie: xcb_void_cookie_t = xcb_void_cookie_t { sequence: 0 };
            class_cookie = xcb_change_property(
                xcb_connection,
                XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as uint8_t,
                bar_id,
                XCB_ATOM_WM_CLASS as ::core::ffi::c_int as xcb_atom_t,
                XCB_ATOM_STRING as ::core::ffi::c_int as xcb_atom_t,
                8 as uint8_t,
                len as uint32_t,
                class as *const ::core::ffi::c_void,
            );
            free(class as *mut ::core::ffi::c_void);
            let mut name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut name,
                b"i3bar for output %s\0" as *const u8 as *const ::core::ffi::c_char,
                (*walk).name,
            );
            let mut name_cookie: xcb_void_cookie_t = xcb_void_cookie_t { sequence: 0 };
            name_cookie = xcb_change_property(
                xcb_connection,
                XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as uint8_t,
                bar_id,
                XCB_ATOM_WM_NAME as ::core::ffi::c_int as xcb_atom_t,
                XCB_ATOM_STRING as ::core::ffi::c_int as xcb_atom_t,
                8 as uint8_t,
                strlen(name) as uint32_t,
                name as *const ::core::ffi::c_void,
            );
            free(name as *mut ::core::ffi::c_void);
            let mut dock_cookie: xcb_void_cookie_t = xcb_change_property(
                xcb_connection,
                XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as uint8_t,
                bar_id,
                atoms[_NET_WM_WINDOW_TYPE as ::core::ffi::c_int as usize],
                XCB_ATOM_ATOM as ::core::ffi::c_int as xcb_atom_t,
                32 as uint8_t,
                1 as uint32_t,
                (&raw mut atoms as *mut xcb_atom_t)
                    .offset(_NET_WM_WINDOW_TYPE_DOCK as ::core::ffi::c_int as isize)
                    as *mut xcb_atom_t as *mut ::core::ffi::c_uchar
                    as *const ::core::ffi::c_void,
            );
            draw_util_surface_init(
                xcb_connection,
                &raw mut (*walk).bar,
                bar_id as xcb_drawable_t,
                ::core::ptr::null_mut::<xcb_visualtype_t>(),
                (*walk).rect.w,
                bar_height,
            );
            draw_util_surface_init(
                xcb_connection,
                &raw mut (*walk).buffer,
                buffer_id as xcb_drawable_t,
                ::core::ptr::null_mut::<xcb_visualtype_t>(),
                (*walk).rect.w,
                bar_height,
            );
            draw_util_surface_init(
                xcb_connection,
                &raw mut (*walk).statusline_buffer,
                statusline_buffer_id as xcb_drawable_t,
                ::core::ptr::null_mut::<xcb_visualtype_t>(),
                (*walk).rect.w,
                bar_height,
            );
            let mut strut_cookie: xcb_void_cookie_t = config_strut_partial(walk);
            let mut map_cookie: xcb_void_cookie_t = xcb_void_cookie_t { sequence: 0 };
            if config.hide_on_modifier as ::core::ffi::c_uint
                == M_DOCK as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                map_cookie = xcb_map_window_checked(xcb_connection, bar_id);
            }
            if _xcb_request_failed(
                win_cookie,
                b"Could not create window\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                1970 as ::core::ffi::c_int,
            ) != 0
                || _xcb_request_failed(
                    pm_cookie,
                    b"Could not create pixmap\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    1971 as ::core::ffi::c_int,
                ) != 0
                || _xcb_request_failed(
                    slpm_cookie,
                    b"Could not create statusline pixmap\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    1972 as ::core::ffi::c_int,
                ) != 0
                || _xcb_request_failed(
                    dock_cookie,
                    b"Could not set dock mode\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    1973 as ::core::ffi::c_int,
                ) != 0
                || _xcb_request_failed(
                    class_cookie,
                    b"Could not set WM_CLASS\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    1974 as ::core::ffi::c_int,
                ) != 0
                || _xcb_request_failed(
                    name_cookie,
                    b"Could not set WM_NAME\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    1975 as ::core::ffi::c_int,
                ) != 0
                || _xcb_request_failed(
                    strut_cookie,
                    b"Could not set strut\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    1976 as ::core::ffi::c_int,
                ) != 0
                || config.hide_on_modifier as ::core::ffi::c_uint
                    == M_DOCK as ::core::ffi::c_int as ::core::ffi::c_uint
                    && _xcb_request_failed(
                        map_cookie,
                        b"Could not map window\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        1977 as ::core::ffi::c_int,
                    ) != 0
            {
                exit(EXIT_FAILURE);
            }
        } else {
            mask = (XCB_CONFIG_WINDOW_X as ::core::ffi::c_int
                | XCB_CONFIG_WINDOW_Y as ::core::ffi::c_int
                | XCB_CONFIG_WINDOW_WIDTH as ::core::ffi::c_int
                | XCB_CONFIG_WINDOW_HEIGHT as ::core::ffi::c_int
                | XCB_CONFIG_WINDOW_STACK_MODE as ::core::ffi::c_int)
                as uint32_t;
            values[0 as ::core::ffi::c_int as usize] = (*walk).rect.x as uint32_t;
            if config.position as ::core::ffi::c_uint
                == POS_TOP as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                values[1 as ::core::ffi::c_int as usize] = (*walk).rect.y as uint32_t;
            } else {
                values[1 as ::core::ffi::c_int as usize] =
                    ((*walk).rect.y + (*walk).rect.h - bar_height) as uint32_t;
            }
            values[2 as ::core::ffi::c_int as usize] = (*walk).rect.w as uint32_t;
            values[3 as ::core::ffi::c_int as usize] = bar_height as uint32_t;
            values[4 as ::core::ffi::c_int as usize] =
                XCB_STACK_MODE_ABOVE as ::core::ffi::c_int as uint32_t;
            if config.verbose {
                printf(
                    b"[%s:%d] Reconfiguring strut partial property for output %s\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    b"../i3bar/src/xcb.c\0" as *const u8 as *const ::core::ffi::c_char,
                    1998 as ::core::ffi::c_int,
                    (*walk).name,
                );
            }
            let mut strut_cookie_0: xcb_void_cookie_t = config_strut_partial(walk);
            if config.verbose {
                printf(
                    b"[%s:%d] Destroying buffer for output %s\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    b"../i3bar/src/xcb.c\0" as *const u8 as *const ::core::ffi::c_char,
                    2001 as ::core::ffi::c_int,
                    (*walk).name,
                );
            }
            xcb_free_pixmap(xcb_connection, (*walk).buffer.id as xcb_pixmap_t);
            if config.verbose {
                printf(
                    b"[%s:%d] Destroying statusline buffer for output %s\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    b"../i3bar/src/xcb.c\0" as *const u8 as *const ::core::ffi::c_char,
                    2004 as ::core::ffi::c_int,
                    (*walk).name,
                );
            }
            xcb_free_pixmap(xcb_connection, (*walk).statusline_buffer.id as xcb_pixmap_t);
            if config.verbose {
                printf(
                    b"[%s:%d] Reconfiguring window for output %s to %d,%d\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    b"../i3bar/src/xcb.c\0" as *const u8 as *const ::core::ffi::c_char,
                    2007 as ::core::ffi::c_int,
                    (*walk).name,
                    values[0 as ::core::ffi::c_int as usize],
                    values[1 as ::core::ffi::c_int as usize],
                );
            }
            let mut cfg_cookie: xcb_void_cookie_t = xcb_configure_window_checked(
                xcb_connection,
                (*walk).bar.id as xcb_window_t,
                mask as uint16_t,
                &raw mut values as *mut uint32_t as *const ::core::ffi::c_void,
            );
            mask = XCB_CW_OVERRIDE_REDIRECT as ::core::ffi::c_int as uint32_t;
            values[0 as ::core::ffi::c_int as usize] = (if config.hide_on_modifier
                as ::core::ffi::c_uint
                == M_DOCK as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                0 as ::core::ffi::c_int
            } else {
                1 as ::core::ffi::c_int
            }) as uint32_t;
            if config.verbose {
                printf(
                    b"[%s:%d] Changing window attribute override_redirect for output %s to %d\n\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    b"../i3bar/src/xcb.c\0" as *const u8 as *const ::core::ffi::c_char,
                    2015 as ::core::ffi::c_int,
                    (*walk).name,
                    values[0 as ::core::ffi::c_int as usize],
                );
            }
            let mut chg_cookie: xcb_void_cookie_t = xcb_change_window_attributes(
                xcb_connection,
                (*walk).bar.id as xcb_window_t,
                mask,
                &raw mut values as *mut uint32_t as *const ::core::ffi::c_void,
            );
            if config.verbose {
                printf(
                    b"[%s:%d] Recreating buffer for output %s\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    b"../i3bar/src/xcb.c\0" as *const u8 as *const ::core::ffi::c_char,
                    2021 as ::core::ffi::c_int,
                    (*walk).name,
                );
            }
            let mut pm_cookie_0: xcb_void_cookie_t = xcb_create_pixmap_checked(
                xcb_connection,
                depth,
                (*walk).buffer.id as xcb_pixmap_t,
                (*walk).bar.id,
                (*walk).rect.w as uint16_t,
                bar_height as uint16_t,
            );
            if config.verbose {
                printf(
                    b"[%s:%d] Recreating statusline buffer for output %s\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    b"../i3bar/src/xcb.c\0" as *const u8 as *const ::core::ffi::c_char,
                    2029 as ::core::ffi::c_int,
                    (*walk).name,
                );
            }
            let mut slpm_cookie_0: xcb_void_cookie_t = xcb_create_pixmap_checked(
                xcb_connection,
                depth,
                (*walk).statusline_buffer.id as xcb_pixmap_t,
                (*walk).bar.id,
                (*walk).rect.w as uint16_t,
                bar_height as uint16_t,
            );
            draw_util_surface_free(xcb_connection, &raw mut (*walk).bar);
            draw_util_surface_free(xcb_connection, &raw mut (*walk).buffer);
            draw_util_surface_free(xcb_connection, &raw mut (*walk).statusline_buffer);
            draw_util_surface_init(
                xcb_connection,
                &raw mut (*walk).bar,
                (*walk).bar.id,
                ::core::ptr::null_mut::<xcb_visualtype_t>(),
                (*walk).rect.w,
                bar_height,
            );
            draw_util_surface_init(
                xcb_connection,
                &raw mut (*walk).buffer,
                (*walk).buffer.id,
                ::core::ptr::null_mut::<xcb_visualtype_t>(),
                (*walk).rect.w,
                bar_height,
            );
            draw_util_surface_init(
                xcb_connection,
                &raw mut (*walk).statusline_buffer,
                (*walk).statusline_buffer.id,
                ::core::ptr::null_mut::<xcb_visualtype_t>(),
                (*walk).rect.w,
                bar_height,
            );
            let mut map_cookie_0: xcb_void_cookie_t = xcb_void_cookie_t { sequence: 0 };
            let mut umap_cookie: xcb_void_cookie_t = xcb_void_cookie_t { sequence: 0 };
            if redraw_bars_0 {
                umap_cookie =
                    xcb_unmap_window_checked(xcb_connection, (*walk).bar.id as xcb_window_t);
                if config.hide_on_modifier as ::core::ffi::c_uint
                    == M_DOCK as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    cont_children();
                    map_cookie_0 =
                        xcb_map_window_checked(xcb_connection, (*walk).bar.id as xcb_window_t);
                } else {
                    stop_children();
                }
                if config.hide_on_modifier as ::core::ffi::c_uint
                    == M_HIDE as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    register_xkb_keyevents();
                } else {
                    deregister_xkb_keyevents();
                }
            }
            if _xcb_request_failed(
                cfg_cookie,
                b"Could not reconfigure window\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                2064 as ::core::ffi::c_int,
            ) != 0
                || _xcb_request_failed(
                    chg_cookie,
                    b"Could not change window\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    2065 as ::core::ffi::c_int,
                ) != 0
                || _xcb_request_failed(
                    pm_cookie_0,
                    b"Could not create pixmap\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    2066 as ::core::ffi::c_int,
                ) != 0
                || _xcb_request_failed(
                    slpm_cookie_0,
                    b"Could not create statusline pixmap\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    2067 as ::core::ffi::c_int,
                ) != 0
                || _xcb_request_failed(
                    strut_cookie_0,
                    b"Could not set strut\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    2068 as ::core::ffi::c_int,
                ) != 0
                || redraw_bars_0 as ::core::ffi::c_int != 0
                    && (_xcb_request_failed(
                        umap_cookie,
                        b"Could not unmap window\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        2069 as ::core::ffi::c_int,
                    ) != 0
                        || config.hide_on_modifier as ::core::ffi::c_uint
                            == M_DOCK as ::core::ffi::c_int as ::core::ffi::c_uint
                            && _xcb_request_failed(
                                map_cookie_0,
                                b"Could not map window\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                                2070 as ::core::ffi::c_int,
                            ) != 0)
            {
                exit(EXIT_FAILURE);
            }
        }
        walk = (*walk).slist.sle_next as *mut i3_output;
    }
    output_for_tray = get_tray_output();
    if !output_for_tray.is_null() {
        if selwin as ::core::ffi::c_long == XCB_NONE {
            init_tray();
        }
    } else if selwin as ::core::ffi::c_long != XCB_NONE {
        if config.verbose {
            printf(
                b"[%s:%d] Destroying tray selection window\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"../i3bar/src/xcb.c\0" as *const u8 as *const ::core::ffi::c_char,
                2084 as ::core::ffi::c_int,
            );
        }
        xcb_destroy_window(xcb_connection, selwin);
        selwin = XCB_NONE as xcb_window_t;
    }
}
unsafe extern "C" fn draw_button(
    mut surface: *mut surface_t,
    mut fg_color: color_t,
    mut bg_color: color_t,
    mut border_color: color_t,
    mut x: ::core::ffi::c_int,
    mut width: ::core::ffi::c_int,
    mut text_width: ::core::ffi::c_int,
    mut text: *mut i3String,
) {
    let mut height: ::core::ffi::c_int =
        bar_height - 2 as ::core::ffi::c_int * logical_px(1 as ::core::ffi::c_int);
    draw_util_rectangle(
        surface,
        border_color,
        x as ::core::ffi::c_double,
        logical_px(1 as ::core::ffi::c_int) as ::core::ffi::c_double,
        width as ::core::ffi::c_double,
        height as ::core::ffi::c_double,
    );
    draw_util_rectangle(
        surface,
        bg_color,
        (x + logical_px(1 as ::core::ffi::c_int)) as ::core::ffi::c_double,
        (2 as ::core::ffi::c_int * logical_px(1 as ::core::ffi::c_int)) as ::core::ffi::c_double,
        (width - 2 as ::core::ffi::c_int * logical_px(1 as ::core::ffi::c_int))
            as ::core::ffi::c_double,
        (height - 2 as ::core::ffi::c_int * logical_px(1 as ::core::ffi::c_int))
            as ::core::ffi::c_double,
    );
    draw_util_text(
        text,
        surface,
        fg_color,
        bg_color,
        x + (width - text_width) / 2 as ::core::ffi::c_int,
        bar_height / 2 as ::core::ffi::c_int - font.height / 2 as ::core::ffi::c_int,
        text_width,
    );
}
#[no_mangle]
pub unsafe extern "C" fn draw_bars(mut unhide: bool) {
    if config.verbose {
        printf(
            b"[%s:%d] Drawing bars...\n\0" as *const u8 as *const ::core::ffi::c_char,
            b"../i3bar/src/xcb.c\0" as *const u8 as *const ::core::ffi::c_char,
            2114 as ::core::ffi::c_int,
        );
    }
    let mut outputs_walk: *mut i3_output = ::core::ptr::null_mut::<i3_output>();
    outputs_walk = (*outputs).slh_first as *mut i3_output;
    while !outputs_walk.is_null() {
        let mut workspace_width: ::core::ffi::c_int =
            logical_px(config.padding.x as ::core::ffi::c_int);
        if !(*outputs_walk).active {
            if config.verbose {
                printf(
                    b"[%s:%d] Output %s inactive, skipping...\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    b"../i3bar/src/xcb.c\0" as *const u8 as *const ::core::ffi::c_char,
                    2121 as ::core::ffi::c_int,
                    (*outputs_walk).name,
                );
            }
        } else {
            if (*outputs_walk).bar.id as ::core::ffi::c_long == XCB_NONE {
                reconfig_windows(false_0 != 0);
            }
            let mut use_focus_colors: bool = output_has_focus(outputs_walk);
            draw_util_clear_surface(
                &raw mut (*outputs_walk).buffer,
                if use_focus_colors as ::core::ffi::c_int != 0 {
                    colors.focus_bar_bg
                } else {
                    colors.bar_bg
                },
            );
            if !config.disable_ws {
                let mut ws_walk: *mut i3_ws = ::core::ptr::null_mut::<i3_ws>();
                ws_walk = (*(*outputs_walk).workspaces).tqh_first as *mut i3_ws;
                while !ws_walk.is_null() {
                    if config.verbose {
                        printf(
                            b"[%s:%d] Drawing button for WS %s at x = %d, len = %d\n\0" as *const u8
                                as *const ::core::ffi::c_char,
                            b"../i3bar/src/xcb.c\0" as *const u8 as *const ::core::ffi::c_char,
                            2138 as ::core::ffi::c_int,
                            i3string_as_utf8((*ws_walk).name),
                            workspace_width,
                            (*ws_walk).name_width,
                        );
                    }
                    let mut fg_color: color_t = colors.inactive_ws_fg;
                    let mut bg_color: color_t = colors.inactive_ws_bg;
                    let mut border_color: color_t = colors.inactive_ws_border;
                    if (*ws_walk).visible {
                        if !(*ws_walk).focused {
                            fg_color = colors.active_ws_fg;
                            bg_color = colors.active_ws_bg;
                            border_color = colors.active_ws_border;
                        } else {
                            fg_color = colors.focus_ws_fg;
                            bg_color = colors.focus_ws_bg;
                            border_color = colors.focus_ws_border;
                        }
                    }
                    if (*ws_walk).urgent {
                        if config.verbose {
                            printf(
                                b"[%s:%d] WS %s is urgent!\n\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                b"../i3bar/src/xcb.c\0" as *const u8 as *const ::core::ffi::c_char,
                                2154 as ::core::ffi::c_int,
                                i3string_as_utf8((*ws_walk).name),
                            );
                        }
                        fg_color = colors.urgent_ws_fg;
                        bg_color = colors.urgent_ws_bg;
                        border_color = colors.urgent_ws_border;
                        unhide = true_0 != 0;
                    }
                    let mut w: ::core::ffi::c_int = predict_button_width((*ws_walk).name_width);
                    draw_button(
                        &raw mut (*outputs_walk).buffer,
                        fg_color,
                        bg_color,
                        border_color,
                        workspace_width,
                        w,
                        (*ws_walk).name_width,
                        (*ws_walk).name,
                    );
                    workspace_width += w;
                    if !(*ws_walk).tailq.tqe_next.is_null() {
                        workspace_width += logical_px(ws_spacing_px);
                    }
                    ws_walk = (*ws_walk).tailq.tqe_next as *mut i3_ws;
                }
            }
            if !binding.name.is_null() && !config.disable_binding_mode_indicator {
                workspace_width += logical_px(ws_spacing_px);
                let mut w_0: ::core::ffi::c_int = predict_button_width(binding.name_width);
                draw_button(
                    &raw mut (*outputs_walk).buffer,
                    colors.binding_mode_fg,
                    colors.binding_mode_bg,
                    colors.binding_mode_border,
                    workspace_width,
                    w_0,
                    binding.name_width,
                    binding.name,
                );
                unhide = true_0 != 0;
                workspace_width += w_0;
            }
            if !statusline_head.tqh_first.is_null() {
                if config.verbose {
                    printf(
                        b"[%s:%d] Printing statusline!\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                        b"../i3bar/src/xcb.c\0" as *const u8 as *const ::core::ffi::c_char,
                        2184 as ::core::ffi::c_int,
                    );
                }
                let mut tray_width: ::core::ffi::c_int =
                    get_tray_width((*outputs_walk).trayclients as *mut tc_head);
                let mut hoff: uint32_t = logical_px(
                    ((workspace_width > 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                        + (tray_width > 0 as ::core::ffi::c_int) as ::core::ffi::c_int)
                        * sb_hoff_px,
                ) as uint32_t;
                let mut max_statusline_width: uint32_t =
                    (((*outputs_walk).rect.w - workspace_width - tray_width) as uint32_t)
                        .wrapping_sub(hoff);
                let mut clip_left: uint32_t = 0 as uint32_t;
                let mut block: *mut status_block = ::core::ptr::null_mut::<status_block>();
                block = statusline_head.tqh_first;
                while !block.is_null() {
                    (*block).use_short = false_0 != 0;
                    block = (*block).blocks.tqe_next;
                }
                let mut statusline_width: uint32_t = adjust_statusline_length(max_statusline_width);
                if statusline_width > max_statusline_width {
                    clip_left = statusline_width.wrapping_sub(max_statusline_width);
                }
                let mut visible_statusline_width: int16_t =
                    (if statusline_width < max_statusline_width {
                        statusline_width
                    } else {
                        max_statusline_width
                    }) as int16_t;
                let mut x_dest: ::core::ffi::c_int = (*outputs_walk).rect.w
                    - tray_width
                    - logical_px(
                        (tray_width > 0 as ::core::ffi::c_int) as ::core::ffi::c_int * sb_hoff_px,
                    )
                    - visible_statusline_width as ::core::ffi::c_int;
                x_dest -= logical_px(config.padding.width as ::core::ffi::c_int);
                draw_statusline(outputs_walk, clip_left, use_focus_colors);
                draw_util_copy_surface(
                    &raw mut (*outputs_walk).statusline_buffer,
                    &raw mut (*outputs_walk).buffer,
                    0 as ::core::ffi::c_int as ::core::ffi::c_double,
                    0 as ::core::ffi::c_int as ::core::ffi::c_double,
                    x_dest as ::core::ffi::c_double,
                    0 as ::core::ffi::c_int as ::core::ffi::c_double,
                    visible_statusline_width as ::core::ffi::c_double,
                    bar_height as int16_t as ::core::ffi::c_double,
                );
                (*outputs_walk).statusline_width = statusline_width as ::core::ffi::c_int;
            }
        }
        outputs_walk = (*outputs_walk).slist.sle_next as *mut i3_output;
    }
    if mod_pressed as ::core::ffi::c_int != 0
        || config.hidden_state as ::core::ffi::c_uint
            == S_SHOW as ::core::ffi::c_int as ::core::ffi::c_uint
        || unhide as ::core::ffi::c_int != 0
    {
        unhide_bars();
    } else if config.hide_on_modifier as ::core::ffi::c_uint
        == M_HIDE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        hide_bars();
    }
    redraw_bars();
}
#[no_mangle]
pub unsafe extern "C" fn redraw_bars() {
    let mut outputs_walk: *mut i3_output = ::core::ptr::null_mut::<i3_output>();
    outputs_walk = (*outputs).slh_first as *mut i3_output;
    while !outputs_walk.is_null() {
        if (*outputs_walk).active {
            draw_util_copy_surface(
                &raw mut (*outputs_walk).buffer,
                &raw mut (*outputs_walk).bar,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
                (*outputs_walk).rect.w as ::core::ffi::c_double,
                (*outputs_walk).rect.h as ::core::ffi::c_double,
            );
            xcb_flush(xcb_connection);
        }
        outputs_walk = (*outputs_walk).slist.sle_next as *mut i3_output;
    }
}
#[no_mangle]
pub unsafe extern "C" fn set_current_mode(mut current: *mut mode) {
    i3string_free(binding.name);
    binding.name = ::core::ptr::null_mut::<i3String>();
    binding = *current as mode;
    activated_mode = !binding.name.is_null();
}
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
