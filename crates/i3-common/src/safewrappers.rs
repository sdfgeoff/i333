/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 */
// Mechanical file-by-file Rust translation of libi3/safewrappers.c.
// Originally translated with C2Rust 0.22.1; subsequently maintained in Rust.
use crate::i333_va_list;

pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: ::core::ffi::c_uint,
    pub fp_offset: ::core::ffi::c_uint,
    pub overflow_arg_area: *mut ::core::ffi::c_void,
    pub reg_save_area: *mut ::core::ffi::c_void,
}
pub type size_t = usize;
pub type ssize_t = isize;
pub type __gnuc_va_list = __builtin_va_list;
pub type va_list = __gnuc_va_list;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EINTR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const EAGAIN: ::core::ffi::c_int = 11 as ::core::ffi::c_int;

pub unsafe fn smalloc(size: size_t) -> *mut ::core::ffi::c_void {
    let result: *mut ::core::ffi::c_void = malloc(size);
    if result.is_null() {
        err(EXIT_FAILURE, c"malloc(%zd)".as_ptr(), size);
    }
    result
}

pub unsafe fn scalloc(num: size_t, size: size_t) -> *mut ::core::ffi::c_void {
    let result: *mut ::core::ffi::c_void = calloc(num, size);
    if result.is_null() {
        err(EXIT_FAILURE, c"calloc(%zd, %zd)".as_ptr(), num, size);
    }
    result
}

pub unsafe fn srealloc(ptr: *mut ::core::ffi::c_void, size: size_t) -> *mut ::core::ffi::c_void {
    let result: *mut ::core::ffi::c_void = realloc(ptr, size);
    if result.is_null() && size > 0 as size_t {
        err(EXIT_FAILURE, c"realloc(%zd)".as_ptr(), size);
    }
    result
}

pub unsafe fn sstrdup(str: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    let result: *mut ::core::ffi::c_char = strdup(str);
    if result.is_null() {
        err(EXIT_FAILURE, c"strdup()".as_ptr());
    }
    result
}

pub unsafe fn sstrndup(str: *const ::core::ffi::c_char, size: size_t) -> *mut ::core::ffi::c_char {
    let result: *mut ::core::ffi::c_char = strndup(str, size);
    if result.is_null() {
        err(EXIT_FAILURE, c"strndup()".as_ptr());
    }
    result
}

pub unsafe extern "C" fn sasprintf(
    strp: *mut *mut ::core::ffi::c_char,
    fmt: *const ::core::ffi::c_char,
    args: ...
) -> ::core::ffi::c_int {
    let mut args_0;
    let mut result: ::core::ffi::c_int = 0;
    args_0 = args.clone();
    result = vasprintf(strp, fmt, i333_va_list!(args_0));
    if result == -(1 as ::core::ffi::c_int) {
        err(EXIT_FAILURE, c"asprintf(%s)".as_ptr(), fmt);
    }
    result
}

pub unsafe fn writeall(
    fd: ::core::ffi::c_int,
    buf: *const ::core::ffi::c_void,
    count: size_t,
) -> ssize_t {
    let mut written: size_t = 0 as size_t;
    while written < count {
        let n: ssize_t = write(
            fd,
            (buf as *mut ::core::ffi::c_char).offset(written as isize)
                as *const ::core::ffi::c_void,
            count.wrapping_sub(written),
        ) as ssize_t;
        if n == -(1 as ::core::ffi::c_int) as ssize_t {
            if *__errno_location() == EINTR || *__errno_location() == EAGAIN {
                continue;
            }
            return n;
        } else {
            written = written.wrapping_add(n as size_t);
        }
    }
    written as ssize_t
}

pub unsafe fn writeall_nonblock(
    fd: ::core::ffi::c_int,
    buf: *const ::core::ffi::c_void,
    count: size_t,
) -> ssize_t {
    let mut written: size_t = 0 as size_t;
    while written < count {
        let n: ssize_t = write(
            fd,
            (buf as *mut ::core::ffi::c_char).offset(written as isize)
                as *const ::core::ffi::c_void,
            count.wrapping_sub(written),
        ) as ssize_t;
        if n == -(1 as ::core::ffi::c_int) as ssize_t {
            if *__errno_location() == EAGAIN {
                return written as ssize_t;
            } else {
                if *__errno_location() == EINTR {
                    continue;
                }
                return n;
            }
        } else {
            written = written.wrapping_add(n as size_t);
        }
    }
    written as ssize_t
}

pub unsafe fn swrite(
    fd: ::core::ffi::c_int,
    buf: *const ::core::ffi::c_void,
    count: size_t,
) -> ssize_t {
    let mut n: ssize_t = 0;
    n = writeall(fd, buf, count);
    if n == -(1 as ::core::ffi::c_int) as ssize_t {
        err(EXIT_FAILURE, c"Failed to write %d".as_ptr(), fd);
    } else {
        n
    }
}

pub unsafe fn strcasecmp_nullable(
    a: *const ::core::ffi::c_char,
    b: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if a == b {
        return 0 as ::core::ffi::c_int;
    }
    if a.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if b.is_null() {
        return 1 as ::core::ffi::c_int;
    }
    strcasecmp(a, b)
}
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

// Internal helpers use Rust linkage rather than foreign symbol lookup.
use libc::strndup;

pub use libc::{__errno_location, calloc, malloc, realloc, strcasecmp, strdup, write};

use crate::native_stdio::{err, vasprintf};
