// i3 - an improved tiling window manager
// Copyright © 2009 Michael Stapelberg and contributors (see LICENSE).
// Direct translation of libi3/strndup.c; C ABI and allocation ownership are preserved.

use libc::{c_char, c_void, size_t};
unsafe extern "C" {
    fn smalloc(size: size_t) -> *mut c_void;
}

// Compile this fallback only on targets where HAVE_STRNDUP is false, as in C.
/// # Safety
/// `string` must be readable through its first NUL or `n` bytes.
#[no_mangle]
pub unsafe extern "C" fn strndup(string: *const c_char, n: size_t) -> *mut c_char {
    // SAFETY: The allocation covers the copied prefix and trailing NUL. Ownership
    // is transferred to the caller and remains compatible with libc::free.
    unsafe {
        let mut len = 0;
        while len < n && *string.add(len) != 0 {
            len += 1;
        }
        let copy = smalloc(len + 1).cast::<c_char>();
        core::ptr::copy_nonoverlapping(string, copy, len);
        *copy.add(len) = 0;
        copy
    }
}
