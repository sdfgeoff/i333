use crate::safewrappers::sasprintf;
/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 */
// Mechanical file-by-file Rust translation of libi3/root_atom_contents.c.
// Originally translated with C2Rust 0.22.1; subsequently maintained in Rust.
extern "C" {

    fn ceil(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;

}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;

pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();

pub unsafe fn root_atom_contents(
    atomname: *const ::core::ffi::c_char,
    provided_conn: *mut xcb_connection_t,
    mut screen: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    let mut prop_cookie: xcb_get_property_cookie_t = xcb_get_property_cookie_t { sequence: 0 };
    let mut prop_reply: *mut xcb_get_property_reply_t =
        ::core::ptr::null_mut::<xcb_get_property_reply_t>();
    let current_block: u64;
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
    let root_screen: *mut xcb_screen_t = xcb_aux_get_screen(conn, screen);
    let root: xcb_window_t = (*root_screen).root;
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
                                c"%u".as_ptr(),
                                *(xcb_get_property_value(prop_reply) as *mut ::core::ffi::c_uint),
                            );
                        } else {
                            sasprintf(
                                &raw mut content,
                                c"%.*s".as_ptr(),
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
    content
}

use xcb_sys::{
    xcb_atom_t, xcb_aux_get_screen, xcb_connect, xcb_connection_has_error, xcb_connection_t,
    xcb_disconnect, xcb_generic_error_t, xcb_get_property_cookie_t, xcb_get_property_reply,
    xcb_get_property_reply_t, xcb_get_property_unchecked, xcb_get_property_value,
    xcb_get_property_value_length, xcb_intern_atom, xcb_intern_atom_cookie_t,
    xcb_intern_atom_reply, xcb_intern_atom_reply_t, xcb_screen_t, xcb_window_t, XCB_ATOM_CARDINAL,
    XCB_GET_PROPERTY_TYPE_ANY,
};

pub use libc::{free, strlen};
