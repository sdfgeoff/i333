/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 */
// Mechanical file-by-file Rust translation of libi3/safewrappers.c.
// Generated with C2Rust 0.22.1; C ABI and original ownership retained.
use crate::i333_va_list;

extern "C" {
    fn vasprintf(
        __ptr: *mut *mut ::core::ffi::c_char,
        __f: *const ::core::ffi::c_char,
        __arg: ::core::ffi::VaList,
    ) -> ::core::ffi::c_int;
    fn write(__fd: ::core::ffi::c_int, __buf: *const ::core::ffi::c_void, __n: size_t) -> ssize_t;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn err(__status: ::core::ffi::c_int, __format: *const ::core::ffi::c_char, ...) -> !;
    fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn strndup(__string: *const ::core::ffi::c_char, __n: size_t) -> *mut ::core::ffi::c_char;
    fn strcasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
}
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
#[no_mangle]
pub unsafe extern "C" fn smalloc(mut size: size_t) -> *mut ::core::ffi::c_void {
    let mut result: *mut ::core::ffi::c_void = malloc(size);
    if result.is_null() {
        err(
            EXIT_FAILURE,
            b"malloc(%zd)\0" as *const u8 as *const ::core::ffi::c_char,
            size,
        );
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn scalloc(mut num: size_t, mut size: size_t) -> *mut ::core::ffi::c_void {
    let mut result: *mut ::core::ffi::c_void = calloc(num, size);
    if result.is_null() {
        err(
            EXIT_FAILURE,
            b"calloc(%zd, %zd)\0" as *const u8 as *const ::core::ffi::c_char,
            num,
            size,
        );
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn srealloc(
    mut ptr: *mut ::core::ffi::c_void,
    mut size: size_t,
) -> *mut ::core::ffi::c_void {
    let mut result: *mut ::core::ffi::c_void = realloc(ptr, size);
    if result.is_null() && size > 0 as size_t {
        err(
            EXIT_FAILURE,
            b"realloc(%zd)\0" as *const u8 as *const ::core::ffi::c_char,
            size,
        );
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn sstrdup(mut str: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    let mut result: *mut ::core::ffi::c_char = strdup(str);
    if result.is_null() {
        err(
            EXIT_FAILURE,
            b"strdup()\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn sstrndup(
    mut str: *const ::core::ffi::c_char,
    mut size: size_t,
) -> *mut ::core::ffi::c_char {
    let mut result: *mut ::core::ffi::c_char = strndup(str, size);
    if result.is_null() {
        err(
            EXIT_FAILURE,
            b"strndup()\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn sasprintf(
    mut strp: *mut *mut ::core::ffi::c_char,
    mut fmt: *const ::core::ffi::c_char,
    mut args: ...
) -> ::core::ffi::c_int {
    let mut args_0;
    let mut result: ::core::ffi::c_int = 0;
    args_0 = args.clone();
    result = vasprintf(strp, fmt, i333_va_list!(args_0));
    if result == -(1 as ::core::ffi::c_int) {
        err(
            EXIT_FAILURE,
            b"asprintf(%s)\0" as *const u8 as *const ::core::ffi::c_char,
            fmt,
        );
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn writeall(
    mut fd: ::core::ffi::c_int,
    mut buf: *const ::core::ffi::c_void,
    mut count: size_t,
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
    return written as ssize_t;
}
#[no_mangle]
pub unsafe extern "C" fn writeall_nonblock(
    mut fd: ::core::ffi::c_int,
    mut buf: *const ::core::ffi::c_void,
    mut count: size_t,
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
    return written as ssize_t;
}
#[no_mangle]
pub unsafe extern "C" fn swrite(
    mut fd: ::core::ffi::c_int,
    mut buf: *const ::core::ffi::c_void,
    mut count: size_t,
) -> ssize_t {
    let mut n: ssize_t = 0;
    n = writeall(fd, buf, count);
    if n == -(1 as ::core::ffi::c_int) as ssize_t {
        err(
            EXIT_FAILURE,
            b"Failed to write %d\0" as *const u8 as *const ::core::ffi::c_char,
            fd,
        );
    } else {
        return n;
    };
}
#[no_mangle]
pub unsafe extern "C" fn strcasecmp_nullable(
    mut a: *const ::core::ffi::c_char,
    mut b: *const ::core::ffi::c_char,
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
    return strcasecmp(a, b);
}
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
