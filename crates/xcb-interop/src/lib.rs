//! Owned foreign utilities and narrow compatibility gaps beside maintained XCB.
//!
//! The `xcb` crate owns protocol requests, replies, event storage, enums and IDs.
//! Cursor themes, key-symbol tables and X resource matching use owned adapters
//! for native libraries. The events/transport modules cover documented gaps in
//! the published maintained API while retaining its canonical protocol types.
#![deny(unsafe_op_in_unsafe_fn)]

pub mod cursor;
pub mod events;
pub mod keysyms;
pub mod transport;
pub mod xrm;

use std::ffi::{c_char, CStr};
use std::ptr::NonNull;

struct NativeString(NonNull<c_char>);
impl NativeString {
    /// pointer is null or transfers a uniquely owned, terminated malloc string.
    unsafe fn from_raw(pointer: *mut c_char) -> Option<Self> {
        NonNull::new(pointer).map(Self)
    }
    fn as_c_str(&self) -> &CStr {
        // SAFETY: the foreign utility transfers this initialized terminated allocation.
        unsafe { CStr::from_ptr(self.0.as_ptr()) }
    }
}
impl Drop for NativeString {
    fn drop(&mut self) {
        // SAFETY: the native utility transfers this sole malloc-compatible owner.
        unsafe { libc::free(self.0.as_ptr().cast()) }
    }
}
