// i3 - an improved tiling window manager
// Copyright © 2009 Michael Stapelberg and contributors (see LICENSE).
// Direct translation of libi3/resolve_tilde.c; C ABI and allocation ownership are preserved.

use libc::{c_char, c_int, c_void, size_t};
unsafe extern "C" {
    fn sstrdup(string: *const c_char) -> *mut c_char;
    fn sstrndup(string: *const c_char, n: size_t) -> *mut c_char;
    fn scalloc(num: size_t, size: size_t) -> *mut c_void;
    fn err(eval: c_int, fmt: *const c_char, ...) -> !;
}

/// # Safety
/// `path` must be a live NUL-terminated C string. Calls must be serialized, as
/// with the C function's static glob buffer.
#[no_mangle]
pub unsafe extern "C" fn resolve_tilde(path: *const c_char) -> *mut c_char {
    // SAFETY: libc owns glob's temporary storage until globfree; returned strings
    // are separately allocated with the same C allocation helpers as upstream.
    unsafe {
        static mut GLOBBUF: core::mem::MaybeUninit<libc::glob_t> = core::mem::MaybeUninit::zeroed();
        let globbuf = core::ptr::addr_of_mut!(GLOBBUF).cast::<libc::glob_t>();
        let tail = libc::strchr(path, b'/' as c_int);
        let mut head = sstrndup(
            path,
            if tail.is_null() {
                libc::strlen(path)
            } else {
                tail.offset_from(path) as usize
            },
        );
        let res = libc::glob(head, libc::GLOB_TILDE, None, globbuf);
        libc::free(head.cast());
        let result;
        if res == libc::GLOB_NOMATCH || (*globbuf).gl_pathc != 1 {
            result = sstrdup(path);
        } else if res != 0 {
            err(libc::EXIT_FAILURE, c"glob() failed".as_ptr());
        } else {
            head = *(*globbuf).gl_pathv;
            result = scalloc(
                libc::strlen(head)
                    + if tail.is_null() {
                        0
                    } else {
                        libc::strlen(tail)
                    }
                    + 1,
                1,
            )
            .cast::<c_char>();
            libc::strcpy(result, head);
            if !tail.is_null() {
                libc::strcat(result, tail);
            }
        }
        libc::globfree(globbuf);
        result
    }
}
