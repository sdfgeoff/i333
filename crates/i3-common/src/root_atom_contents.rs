/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 */
// Mechanical file-by-file Rust translation of libi3/root_atom_contents.c.
// Generated with C2Rust 0.22.1; C ABI and original ownership retained.
extern "C" {
    pub type xcb_connection_t;
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
    fn xcb_connection_has_error(c: *mut xcb_connection_t) -> ::core::ffi::c_int;
    fn xcb_disconnect(c: *mut xcb_connection_t);
    fn xcb_connect(
        displayname: *const ::core::ffi::c_char,
        screenp: *mut ::core::ffi::c_int,
    ) -> *mut xcb_connection_t;
    fn sasprintf(
        strp: *mut *mut ::core::ffi::c_char,
        fmt: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn ceil(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn xcb_aux_get_screen(
        c: *mut xcb_connection_t,
        screen: ::core::ffi::c_int,
    ) -> *mut xcb_screen_t;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
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
pub type xcb_window_t = uint32_t;
pub type xcb_colormap_t = uint32_t;
pub type xcb_atom_t = uint32_t;
pub type xcb_visualid_t = uint32_t;
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
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[no_mangle]
pub unsafe extern "C" fn root_atom_contents(
    mut atomname: *const ::core::ffi::c_char,
    mut provided_conn: *mut xcb_connection_t,
    mut screen: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    let mut prop_cookie: xcb_get_property_cookie_t = xcb_get_property_cookie_t { sequence: 0 };
    let mut prop_reply: *mut xcb_get_property_reply_t =
        ::core::ptr::null_mut::<xcb_get_property_reply_t>();
    let mut current_block: u64;
    let mut atom_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom_cookie_t { sequence: 0 };
    let mut atom_reply: *mut xcb_intern_atom_reply_t =
        ::core::ptr::null_mut::<xcb_intern_atom_reply_t>();
    let mut content: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut content_max_words: size_t = 256 as size_t;
    let mut conn: *mut xcb_connection_t = provided_conn;
    if provided_conn.is_null() && {
        conn = xcb_connect(::core::ptr::null::<::core::ffi::c_char>(), &raw mut screen);
        conn.is_null() || xcb_connection_has_error(conn) != 0
    } {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    atom_cookie = xcb_intern_atom(conn, 0 as uint8_t, strlen(atomname) as uint16_t, atomname);
    let mut root_screen: *mut xcb_screen_t = xcb_aux_get_screen(conn, screen);
    let mut root: xcb_window_t = (*root_screen).root;
    atom_reply = xcb_intern_atom_reply(
        conn,
        atom_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if !atom_reply.is_null() {
        prop_cookie = xcb_get_property_cookie_t { sequence: 0 };
        prop_reply = ::core::ptr::null_mut::<xcb_get_property_reply_t>();
        prop_cookie = xcb_get_property_unchecked(
            conn,
            false_0 as uint8_t,
            root,
            (*atom_reply).atom,
            XCB_GET_PROPERTY_TYPE_ANY as ::core::ffi::c_int as xcb_atom_t,
            0 as uint32_t,
            content_max_words as uint32_t,
        );
        prop_reply = xcb_get_property_reply(
            conn,
            prop_cookie,
            ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
        );
        if !prop_reply.is_null() {
            if xcb_get_property_value_length(prop_reply) > 0 as ::core::ffi::c_int
                && (*prop_reply).bytes_after > 0 as uint32_t
            {
                content_max_words = (content_max_words as ::core::ffi::c_double
                    + ceil((*prop_reply).bytes_after as ::core::ffi::c_double / 4.0f64))
                    as size_t;
                free(prop_reply as *mut ::core::ffi::c_void);
                prop_cookie = xcb_get_property_unchecked(
                    conn,
                    false_0 as uint8_t,
                    root,
                    (*atom_reply).atom,
                    XCB_GET_PROPERTY_TYPE_ANY as ::core::ffi::c_int as xcb_atom_t,
                    0 as uint32_t,
                    content_max_words as uint32_t,
                );
                prop_reply = xcb_get_property_reply(
                    conn,
                    prop_cookie,
                    ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
                );
                if prop_reply.is_null() {
                    current_block = 10617833008915559210;
                } else {
                    current_block = 13056961889198038528;
                }
            } else {
                current_block = 13056961889198038528;
            }
            match current_block {
                10617833008915559210 => {}
                _ => {
                    if !(xcb_get_property_value_length(prop_reply) == 0 as ::core::ffi::c_int) {
                        if (*prop_reply).type_0
                            == XCB_ATOM_CARDINAL as ::core::ffi::c_int as xcb_atom_t
                        {
                            sasprintf(
                                &raw mut content,
                                b"%u\0" as *const u8 as *const ::core::ffi::c_char,
                                *(xcb_get_property_value(prop_reply) as *mut ::core::ffi::c_uint),
                            );
                        } else {
                            sasprintf(
                                &raw mut content,
                                b"%.*s\0" as *const u8 as *const ::core::ffi::c_char,
                                xcb_get_property_value_length(prop_reply),
                                xcb_get_property_value(prop_reply) as *mut ::core::ffi::c_char,
                            );
                        }
                    }
                    free(prop_reply as *mut ::core::ffi::c_void);
                }
            }
        }
        free(atom_reply as *mut ::core::ffi::c_void);
    }
    if provided_conn.is_null() {
        xcb_disconnect(conn);
    }
    return content;
}
