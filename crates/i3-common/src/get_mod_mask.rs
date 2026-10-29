/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 */
// Mechanical file-by-file Rust translation of libi3/get_mod_mask.c.
// Generated with C2Rust 0.22.1; C ABI and original ownership retained.
extern "C" {
    pub type xcb_connection_t;
    pub type _XCBKeySymbols;
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
    fn xcb_key_symbols_get_keycode(
        syms: *mut xcb_key_symbols_t,
        keysym: xcb_keysym_t,
    ) -> *mut xcb_keycode_t;
    static mut conn: *mut xcb_connection_t;
    fn free(__ptr: *mut ::core::ffi::c_void);
}
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
pub type xcb_keysym_t = uint32_t;
pub type xcb_keycode_t = uint8_t;
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
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[no_mangle]
pub unsafe extern "C" fn aio_get_mod_mask_for(
    mut keysym: uint32_t,
    mut symbols: *mut xcb_key_symbols_t,
) -> uint32_t {
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
        return 0 as uint32_t;
    }
    let mut result: uint32_t = get_mod_mask_for(keysym, symbols, modmap_r);
    free(modmap_r as *mut ::core::ffi::c_void);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn get_mod_mask_for(
    mut keysym: uint32_t,
    mut symbols: *mut xcb_key_symbols_t,
    mut modmap_reply: *mut xcb_get_modifier_mapping_reply_t,
) -> uint32_t {
    let mut codes: *mut xcb_keycode_t = ::core::ptr::null_mut::<xcb_keycode_t>();
    let mut modmap: *mut xcb_keycode_t = ::core::ptr::null_mut::<xcb_keycode_t>();
    let mut mod_code: xcb_keycode_t = 0;
    modmap = xcb_get_modifier_mapping_keycodes(modmap_reply);
    codes = xcb_key_symbols_get_keycode(symbols, keysym as xcb_keysym_t);
    if codes.is_null() {
        return 0 as uint32_t;
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
                    return ((1 as ::core::ffi::c_int) << mod_0) as uint32_t;
                }
            }
            j += 1;
        }
        mod_0 += 1;
    }
    return 0 as uint32_t;
}
