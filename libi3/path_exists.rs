// i3 - an improved tiling window manager
// Copyright © 2009 Michael Stapelberg and contributors (see LICENSE).
// Direct translation of libi3/path_exists.c; C ABI and allocation ownership are preserved.

use libc::c_char;

/// # Safety
/// `path` must point to a live NUL-terminated C string.
#[no_mangle]
pub unsafe extern "C" fn path_exists(path: *const c_char) -> bool {
    let mut buf = core::mem::MaybeUninit::<libc::stat>::uninit();
    // SAFETY: stat initializes its output on success; the uninitialized output is never read.
    unsafe { libc::stat(path, buf.as_mut_ptr()) == 0 }
}
