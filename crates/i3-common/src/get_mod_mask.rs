/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 */
// Mechanical file-by-file Rust translation of libi3/get_mod_mask.c.
// Originally translated with C2Rust 0.22.1; subsequently maintained in Rust.

pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();

pub unsafe fn aio_get_mod_mask_for(keysym: u32, symbols: *mut xcb_key_symbols_t) -> u32 {
    let crate::display::DisplayContext {
        connection: conn, ..
    } = crate::display::context();

    let mut cookie: xcb_get_modifier_mapping_cookie_t =
        xcb_get_modifier_mapping_cookie_t { sequence: 0 };
    let mut modmap_r: *mut xcb_get_modifier_mapping_reply_t =
        ::core::ptr::null_mut::<xcb_get_modifier_mapping_reply_t>();
    xcb_flush(conn);
    cookie = xcb_get_modifier_mapping(conn);
    modmap_r = xcb_get_modifier_mapping_reply(
        conn,
        cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if modmap_r.is_null() {
        return 0 as u32;
    }
    let result: u32 = get_mod_mask_for(keysym, symbols, modmap_r);
    free(modmap_r as *mut ::core::ffi::c_void);
    result
}

pub unsafe fn get_mod_mask_for(
    keysym: u32,
    symbols: *mut xcb_key_symbols_t,
    modmap_reply: *mut xcb_get_modifier_mapping_reply_t,
) -> u32 {
    let mut codes: *mut xcb_keycode_t = ::core::ptr::null_mut::<xcb_keycode_t>();
    let mut modmap: *mut xcb_keycode_t = ::core::ptr::null_mut::<xcb_keycode_t>();
    let mut mod_code: xcb_keycode_t = 0;
    modmap = xcb_get_modifier_mapping_keycodes(modmap_reply);
    codes = xcb_key_symbols_get_keycode(symbols, keysym as xcb_keysym_t);
    if codes.is_null() {
        return 0 as u32;
    }
    let mut mod_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while mod_0 < 8 as ::core::ffi::c_int {
        let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while j < (*modmap_reply).keycodes_per_modifier as ::core::ffi::c_int {
            mod_code = *modmap.offset(
                (mod_0 * (*modmap_reply).keycodes_per_modifier as ::core::ffi::c_int + j) as isize,
            );
            let mut code: *mut xcb_keycode_t = codes;
            while *code != 0 {
                if *code as ::core::ffi::c_int != mod_code as ::core::ffi::c_int {
                    code = code.offset(1);
                } else {
                    free(codes as *mut ::core::ffi::c_void);
                    return ((1 as ::core::ffi::c_int) << mod_0) as u32;
                }
            }
            j += 1;
        }
        mod_0 += 1;
    }
    0 as u32
}

use xcb_sys::{
    xcb_flush, xcb_generic_error_t, xcb_get_modifier_mapping, xcb_get_modifier_mapping_cookie_t,
    xcb_get_modifier_mapping_keycodes, xcb_get_modifier_mapping_reply,
    xcb_get_modifier_mapping_reply_t, xcb_key_symbols_get_keycode, xcb_key_symbols_t,
    xcb_keycode_t, xcb_keysym_t,
};

pub use libc::free;
