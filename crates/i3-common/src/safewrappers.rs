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

pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();

pub unsafe fn smalloc(size: usize) -> *mut ::core::ffi::c_void {
    let result: *mut ::core::ffi::c_void = malloc(size);
    if result.is_null() {
        err(libc::EXIT_FAILURE, c"malloc(%zd)".as_ptr(), size);
    }
    result
}

pub unsafe fn scalloc(num: usize, size: usize) -> *mut ::core::ffi::c_void {
    let result: *mut ::core::ffi::c_void = calloc(num, size);
    if result.is_null() {
        err(libc::EXIT_FAILURE, c"calloc(%zd, %zd)".as_ptr(), num, size);
    }
    result
}

pub unsafe fn srealloc(ptr: *mut ::core::ffi::c_void, size: usize) -> *mut ::core::ffi::c_void {
    let result: *mut ::core::ffi::c_void = realloc(ptr, size);
    if result.is_null() && size > 0 as usize {
        err(libc::EXIT_FAILURE, c"realloc(%zd)".as_ptr(), size);
    }
    result
}

pub unsafe fn sstrdup(str: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    let result: *mut ::core::ffi::c_char = strdup(str);
    if result.is_null() {
        err(libc::EXIT_FAILURE, c"strdup()".as_ptr());
    }
    result
}

pub unsafe fn sstrndup(str: *const ::core::ffi::c_char, size: usize) -> *mut ::core::ffi::c_char {
    let result: *mut ::core::ffi::c_char = strndup(str, size);
    if result.is_null() {
        err(libc::EXIT_FAILURE, c"strndup()".as_ptr());
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
        err(libc::EXIT_FAILURE, c"asprintf(%s)".as_ptr(), fmt);
    }
    result
}

pub unsafe fn writeall(
    fd: ::core::ffi::c_int,
    buf: *const ::core::ffi::c_void,
    count: usize,
) -> isize {
    let mut written: usize = 0 as usize;
    while written < count {
        let n: isize = write(
            fd,
            (buf as *mut ::core::ffi::c_char).offset(written as isize)
                as *const ::core::ffi::c_void,
            count.wrapping_sub(written),
        ) as isize;
        if n == -(1 as ::core::ffi::c_int) as isize {
            if *__errno_location() == libc::EINTR || *__errno_location() == libc::EAGAIN {
                continue;
            }
            return n;
        } else {
            written = written.wrapping_add(n as usize);
        }
    }
    written as isize
}

pub unsafe fn writeall_nonblock(
    fd: ::core::ffi::c_int,
    buf: *const ::core::ffi::c_void,
    count: usize,
) -> isize {
    let mut written: usize = 0 as usize;
    while written < count {
        let n: isize = write(
            fd,
            (buf as *mut ::core::ffi::c_char).offset(written as isize)
                as *const ::core::ffi::c_void,
            count.wrapping_sub(written),
        ) as isize;
        if n == -(1 as ::core::ffi::c_int) as isize {
            if *__errno_location() == libc::EAGAIN {
                return written as isize;
            } else {
                if *__errno_location() == libc::EINTR {
                    continue;
                }
                return n;
            }
        } else {
            written = written.wrapping_add(n as usize);
        }
    }
    written as isize
}

pub unsafe fn swrite(
    fd: ::core::ffi::c_int,
    buf: *const ::core::ffi::c_void,
    count: usize,
) -> isize {
    let mut n: isize = 0;
    n = writeall(fd, buf, count);
    if n == -(1 as ::core::ffi::c_int) as isize {
        err(libc::EXIT_FAILURE, c"Failed to write %d".as_ptr(), fd);
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

// Internal helpers use Rust linkage rather than foreign symbol lookup.
use libc::strndup;

pub use libc::{__errno_location, calloc, malloc, realloc, strcasecmp, strdup, write};

use crate::native_stdio::{err, vasprintf};
