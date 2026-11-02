//! Native key-symbol cache with Rust ownership and owned keycode results.
use crate::NativeString;
use std::{ptr::NonNull, rc::Rc};
use xcb::{x, Raw};

/// Opaque libxcb-keysyms allocation, shared only by transitional native adapters.
#[repr(C)]
pub struct NativeKeySymbols {
    _private: [u8; 0],
}

pub struct KeySymbols {
    pointer: NonNull<NativeKeySymbols>,
    // Native key-symbol tables retain a borrowed connection internally.
    connection: Rc<xcb::Connection>,
}
impl KeySymbols {
    pub fn new(connection: Rc<xcb::Connection>) -> Option<Self> {
        // SAFETY: the owner remains retained until after native table destruction.
        let pointer =
            NonNull::new(unsafe { ffi::xcb_key_symbols_alloc(connection.get_raw_conn()) })?;
        Some(Self {
            pointer,
            connection,
        })
    }
    pub fn connection(&self) -> &xcb::Connection {
        &self.connection
    }
    pub fn keycodes(&self, keysym: u32) -> Vec<u8> {
        // SAFETY: this owner keeps its native cache and connection live.
        unsafe { keycodes_for_raw(self.pointer.as_ptr(), keysym) }
    }
    pub fn keysym(&self, keycode: u8, column: i32) -> u32 {
        // SAFETY: native lookup borrows this live cache synchronously.
        unsafe { ffi::xcb_key_symbols_get_keysym(self.pointer.as_ptr(), keycode, column) }
    }
    pub fn refresh(&mut self, event: &x::MappingNotifyEvent) -> bool {
        // SAFETY: the maintained event owns its complete native wire body. Its
        // pointer crosses the foreign utility ABI without recreating its layout.
        unsafe {
            ffi::xcb_refresh_keyboard_mapping(self.pointer.as_ptr(), event.as_raw().cast()) != 0
        }
    }
    /// Borrow the native handle for an existing foreign-library integration.
    pub fn as_raw(&self) -> *mut NativeKeySymbols {
        self.pointer.as_ptr()
    }
}
impl Drop for KeySymbols {
    fn drop(&mut self) {
        // SAFETY: the sole native allocation is freed before its retained Rc.
        unsafe { ffi::xcb_key_symbols_free(self.pointer.as_ptr()) }
    }
}

/// Read an existing native key-symbol cache while migrating its outer owner.
///
/// # Safety
/// symbols is null or points to a live libxcb-keysyms cache and its live display.
/// The cache must not be destroyed or refreshed concurrently with this call.
pub unsafe fn keycodes_for_raw(symbols: *mut NativeKeySymbols, keysym: u32) -> Vec<u8> {
    if symbols.is_null() {
        return Vec::new();
    }
    // SAFETY: caller retains the native cache; the result transfers an owned
    // zero-terminated keycode byte array. Keycode zero is its sentinel.
    let result =
        unsafe { NativeString::from_raw(ffi::xcb_key_symbols_get_keycode(symbols, keysym).cast()) };
    result.map_or_else(Vec::new, |codes| codes.as_c_str().to_bytes().to_vec())
}

mod ffi {
    use super::NativeKeySymbols;
    use std::ffi::{c_int, c_void};
    use xcb::ffi::xcb_connection_t;
    unsafe extern "C" {
        pub fn xcb_key_symbols_alloc(connection: *mut xcb_connection_t) -> *mut NativeKeySymbols;
        pub fn xcb_key_symbols_free(symbols: *mut NativeKeySymbols);
        pub fn xcb_key_symbols_get_keycode(symbols: *mut NativeKeySymbols, keysym: u32) -> *mut u8;
        pub fn xcb_key_symbols_get_keysym(
            symbols: *mut NativeKeySymbols,
            keycode: u8,
            column: c_int,
        ) -> u32;
        // The native MappingNotify pointer is ABI-opaque here; event ownership
        // and protocol layout come exclusively from maintained xcb.
        pub fn xcb_refresh_keyboard_mapping(
            symbols: *mut NativeKeySymbols,
            event: *const c_void,
        ) -> c_int;
    }
}
