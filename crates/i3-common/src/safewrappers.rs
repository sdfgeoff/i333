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

/// Allocate malloc-compatible, uninitialized storage, or terminate on failure.
/// The returned allocation must be initialized before reads and released once
/// with libc::free (or transferred to a compatible native owner).
pub unsafe fn smalloc(size: usize) -> *mut ::core::ffi::c_void {
    let result: *mut ::core::ffi::c_void = malloc(size);
    if result.is_null() {
        err(libc::EXIT_FAILURE, c"malloc(%zd)".as_ptr(), size);
    }
    result
}

/// Allocate malloc-compatible zeroed storage, or terminate on failure.
/// Zeroed bytes do not necessarily form a valid Rust value; callers must uphold
/// the stored type's validity and release this allocation with libc::free.
pub unsafe fn scalloc(num: usize, size: usize) -> *mut ::core::ffi::c_void {
    let result: *mut ::core::ffi::c_void = calloc(num, size);
    if result.is_null() {
        err(libc::EXIT_FAILURE, c"calloc(%zd, %zd)".as_ptr(), num, size);
    }
    result
}

/// Resize a native allocation, terminating on failure for a nonzero size.
///
/// # Safety
/// ptr must be null or a live malloc-family allocation uniquely available for
/// reallocation. Existing references must not be used after this call; a zero
/// size may free the allocation and return null. The result belongs to free().
pub unsafe fn srealloc(ptr: *mut ::core::ffi::c_void, size: usize) -> *mut ::core::ffi::c_void {
    let result: *mut ::core::ffi::c_void = realloc(ptr, size);
    if result.is_null() && size > 0 as usize {
        err(libc::EXIT_FAILURE, c"realloc(%zd)".as_ptr(), size);
    }
    result
}

/// Duplicate a native string into malloc-owned storage, or terminate on failure.
///
/// # Safety
/// str must point to a readable NUL-terminated string. The caller frees the
/// independent returned allocation with libc::free.
pub unsafe fn sstrdup(str: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    let result: *mut ::core::ffi::c_char = strdup(str);
    if result.is_null() {
        err(libc::EXIT_FAILURE, c"strdup()".as_ptr());
    }
    result
}

/// Duplicate at most size bytes into a terminated malloc-owned string.
///
/// # Safety
/// str must be readable through its first NUL or size bytes, whichever comes
/// first. The caller releases the independent returned allocation with free().
pub unsafe fn sstrndup(str: *const ::core::ffi::c_char, size: usize) -> *mut ::core::ffi::c_char {
    let result: *mut ::core::ffi::c_char = strndup(str, size);
    if result.is_null() {
        err(libc::EXIT_FAILURE, c"strndup()".as_ptr());
    }
    result
}

/// Format into a new malloc-owned string, terminating on allocation failure.
///
/// # Safety
/// strp must be writable, fmt must be a live C format string, and every variadic
/// argument must match its promoted C type and pointer/buffer requirements.
/// Successful output belongs to libc::free; no Rust allocation is substituted.
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

/// Write the complete buffer, retrying EINTR and EAGAIN as the native helper did.
///
/// # Safety
/// buf must be readable for count bytes in one allocation for the whole call;
/// null is allowed only when count is zero. fd is borrowed, never closed here.
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

/// Write until complete, returning progress on EAGAIN and retrying EINTR.
///
/// # Safety
/// buf must be readable for count bytes in one allocation for the whole call;
/// null is allowed only when count is zero. fd is borrowed, never closed here.
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

/// Write a complete buffer, terminating on an unrecoverable write failure.
///
/// # Safety
/// buf must satisfy writeall's readable-buffer contract throughout this call.
/// The borrowed fd is not closed, including when a fatal diagnostic is emitted.
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

/// Compare optional native strings using the current C locale. Null sorts first.
///
/// # Safety
/// Each non-null argument must remain a readable NUL-terminated string for the
/// comparison. Identical pointers (including two null pointers) compare equal.
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
