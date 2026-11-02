//! Independent real-library utility oracles, without protocol packet schemas.
use xcb::ffi::xcb_connection_t;
use xcb_interop::keysyms::NativeKeySymbols;

#[link(name = "xcb-util")]
unsafe extern "C" {
    pub fn xcb_aux_get_screen(
        connection: *mut xcb_connection_t,
        screen: libc::c_int,
    ) -> *mut cairo_sys::xcb_screen_t;
    pub fn xcb_aux_find_visual_by_id(
        screen: *mut cairo_sys::xcb_screen_t,
        id: u32,
    ) -> *mut cairo_sys::xcb_visualtype_t;
}

#[link(name = "xcb-keysyms")]
unsafe extern "C" {
    pub fn xcb_key_symbols_alloc(connection: *mut xcb_connection_t) -> *mut NativeKeySymbols;
    pub fn xcb_key_symbols_free(symbols: *mut NativeKeySymbols);
    pub fn xcb_key_symbols_get_keysym(symbols: *mut NativeKeySymbols, code: u8, column: i32)
        -> u32;
    pub fn xcb_key_symbols_get_keycode(symbols: *mut NativeKeySymbols, keysym: u32) -> *mut u8;
}
