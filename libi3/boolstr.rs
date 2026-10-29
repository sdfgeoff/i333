// i3 - an improved tiling window manager
// Copyright © 2009 Michael Stapelberg and contributors (see LICENSE).
// Direct translation of libi3/boolstr.c; C ABI and allocation ownership are preserved.

use libc::c_char;

/// # Safety
/// `string` must point to a live NUL-terminated C string.
#[no_mangle]
pub unsafe extern "C" fn boolstr(string: *const c_char) -> bool {
    // SAFETY: The caller supplies a C string and every comparison is NUL-terminated.
    unsafe {
        libc::strcasecmp(string, c"1".as_ptr()) == 0
            || libc::strcasecmp(string, c"yes".as_ptr()) == 0
            || libc::strcasecmp(string, c"true".as_ptr()) == 0
            || libc::strcasecmp(string, c"on".as_ptr()) == 0
            || libc::strcasecmp(string, c"enable".as_ptr()) == 0
            || libc::strcasecmp(string, c"active".as_ptr()) == 0
    }
}
