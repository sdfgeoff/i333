// i3 - an improved tiling window manager
// Copyright © 2009 Michael Stapelberg and contributors (see LICENSE).
// Direct translation of libi3/mkdirp.c; C ABI and allocation ownership are preserved.

use libc::{c_char, c_int, mode_t};
unsafe extern "C" {
    fn sstrdup(string: *const c_char) -> *mut c_char;
    fn errorlog(fmt: *const c_char, ...);
}

// Compile this fallback only on targets where HAVE_MKDIRP is false, as in C.
/// # Safety
/// `path` must point to a live NUL-terminated C string.
#[no_mangle]
pub unsafe extern "C" fn mkdirp(path: *const c_char, mode: mode_t) -> c_int {
    // SAFETY: C strings are valid by contract; copy is malloc-owned and writable.
    // stat's output is read only when the call succeeds.
    unsafe {
        if libc::mkdir(path, mode) == 0 {
            return 0;
        }
        let errno = *libc::__errno_location();
        if errno == libc::EEXIST {
            let mut st = core::mem::MaybeUninit::<libc::stat>::uninit();
            if libc::stat(path, st.as_mut_ptr()) != 0 {
                errorlog(
                    c"[libi3] ERROR: stat(%s) failed: %s\n".as_ptr(),
                    path,
                    libc::strerror(*libc::__errno_location()),
                );
                return -1;
            }
            if st.assume_init().st_mode & libc::S_IFMT != libc::S_IFDIR {
                errorlog(
                    c"[libi3] ERROR: mkdir(%s) failed: %s\n".as_ptr(),
                    path,
                    libc::strerror(libc::ENOTDIR),
                );
                return -1;
            }
            return 0;
        } else if errno != libc::ENOENT {
            errorlog(
                c"[libi3] ERROR: mkdir(%s) failed: %s\n".as_ptr(),
                path,
                libc::strerror(errno),
            );
            return -1;
        }
        let copy = sstrdup(path);
        // For ENOENT, upstream strips trailing slashes before finding the parent.
        // An empty path has no parent; avoid the C implementation's underflow.
        let mut len = libc::strlen(copy);
        while len > 0 && *copy.add(len - 1) == b'/' as c_char {
            len -= 1;
            *copy.add(len) = 0;
        }
        let sep = libc::strrchr(copy, b'/' as c_int);
        if sep.is_null() {
            libc::free(copy.cast());
            return -1;
        }
        *sep = 0;
        let mut result = -1;
        if mkdirp(copy, mode) == 0 {
            result = mkdirp(path, mode);
        }
        libc::free(copy.cast());
        result
    }
}
