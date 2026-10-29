// i3 - an improved tiling window manager
// Copyright © 2009 Michael Stapelberg and contributors (see LICENSE).
// Direct translation of libi3/strndup.c; Legacy allocation ownership is preserved.

use libc::{c_char, c_void, size_t};

// Compile this fallback only on targets where HAVE_STRNDUP is false, as in C.
/// # Safety
/// `string` must be readable through its first NUL or `n` bytes.

pub unsafe  fn strndup(string: *const c_char, n: size_t) -> *mut c_char {
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

// Internal helpers use Rust linkage rather than foreign symbol lookup.
use crate::safewrappers::smalloc;
