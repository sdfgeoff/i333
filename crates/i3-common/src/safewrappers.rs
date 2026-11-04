/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 */
// Native allocation compatibility and bounded descriptor writes.
#![deny(unsafe_op_in_unsafe_fn)]
use core::ffi::{c_char, c_void};
use std::{io, os::fd::RawFd};

/// Allocate malloc-compatible, uninitialized storage, or terminate on failure.
/// The returned allocation must be initialized before reads and released once
/// with libc::free (or transferred to a compatible native owner).
pub fn smalloc(size: usize) -> *mut c_void {
    let result = unsafe { malloc(size) };
    if result.is_null() {
        unsafe { err(libc::EXIT_FAILURE, c"malloc(%zd)".as_ptr(), size) };
    }
    result
}

/// Allocate malloc-compatible zeroed storage, or terminate on failure.
/// Zeroed bytes do not necessarily form a valid Rust value; callers must uphold
/// the stored type's validity and release this allocation with libc::free.
pub fn scalloc(num: usize, size: usize) -> *mut c_void {
    let result = unsafe { calloc(num, size) };
    if result.is_null() {
        unsafe { err(libc::EXIT_FAILURE, c"calloc(%zd, %zd)".as_ptr(), num, size) };
    }
    result
}

/// Resize a native allocation, terminating on failure for a nonzero size.
///
/// # Safety
/// ptr must be null or a live malloc-family allocation uniquely available for
/// reallocation. Existing references must not be used after this call; a zero
/// size may free the allocation and return null. The result belongs to free().
pub unsafe fn srealloc(ptr: *mut c_void, size: usize) -> *mut c_void {
    let result = unsafe { realloc(ptr, size) };
    if result.is_null() && size > 0 {
        unsafe { err(libc::EXIT_FAILURE, c"realloc(%zd)".as_ptr(), size) };
    }
    result
}

/// Duplicate a native string into malloc-owned storage, or terminate on failure.
///
/// # Safety
/// str must point to a readable NUL-terminated string. The caller frees the
/// independent returned allocation with libc::free.
pub unsafe fn sstrdup(str: *const c_char) -> *mut c_char {
    let result = unsafe { strdup(str) };
    if result.is_null() {
        unsafe { err(libc::EXIT_FAILURE, c"strdup()".as_ptr()) };
    }
    result
}

/// Duplicate at most size bytes into a terminated malloc-owned string.
///
/// # Safety
/// str must be readable through its first NUL or size bytes, whichever comes
/// first. The caller releases the independent returned allocation with free().
pub unsafe fn sstrndup(str: *const c_char, size: usize) -> *mut c_char {
    let result = unsafe { strndup(str, size) };
    if result.is_null() {
        unsafe { err(libc::EXIT_FAILURE, c"strndup()".as_ptr()) };
    }
    result
}

/// Write the full slice, retrying both interrupted and would-block writes.
/// The descriptor is borrowed and need not be blocking. As with the historical
/// native helper, repeated zero-progress writes retry rather than returning a
/// WriteZero error; callers must ensure their descriptor can eventually progress.
pub fn write_all(fd: RawFd, bytes: &[u8]) -> io::Result<usize> {
    write_buffer(bytes, false, |remaining| write_once(fd, remaining))
}

/// Write until complete or until the descriptor would block, returning progress.
/// Interrupted and zero-progress writes retry. The descriptor is never closed.
pub fn write_available(fd: RawFd, bytes: &[u8]) -> io::Result<usize> {
    write_buffer(bytes, true, |remaining| write_once(fd, remaining))
}

fn write_once(fd: RawFd, bytes: &[u8]) -> io::Result<usize> {
    // SAFETY: bytes is readable for its full length and the kernel validates the
    // integer descriptor. No descriptor ownership or native pointer escapes.
    let count = unsafe { libc::write(fd, bytes.as_ptr().cast(), bytes.len()) };
    if count < 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(count as usize)
    }
}

fn write_buffer(
    mut remaining: &[u8],
    stop_on_would_block: bool,
    mut write: impl FnMut(&[u8]) -> io::Result<usize>,
) -> io::Result<usize> {
    let mut written = 0;
    while !remaining.is_empty() {
        match write(remaining) {
            Ok(count) => {
                // Slicing enforces bounds even if an injected writer misreports
                // progress. A native successful write cannot exceed its input.
                remaining = &remaining[count..];
                written += count;
            }
            Err(error) if error.kind() == io::ErrorKind::Interrupted => continue,
            Err(error) if error.kind() == io::ErrorKind::WouldBlock => {
                if stop_on_would_block {
                    return Ok(written);
                }
            }
            Err(error) => return Err(error),
        }
    }
    Ok(written)
}

/// Compare optional bounded byte strings using native locale ordering. Missing
/// sorts before present (including empty); only the first-NUL prefix is compared.
pub fn strcasecmp_nullable(a: Option<&[u8]>, b: Option<&[u8]>) -> i32 {
    match (a, b) {
        (None, None) => 0,
        (None, Some(_)) => -1,
        (Some(_), None) => 1,
        (Some(a), Some(b)) => {
            let a = std::ffi::CString::new(a.split(|&byte| byte == 0).next().unwrap()).unwrap();
            let b = std::ffi::CString::new(b.split(|&byte| byte == 0).next().unwrap()).unwrap();
            // SAFETY: independent local owners terminate both native arguments.
            unsafe { libc::strcasecmp(a.as_ptr(), b.as_ptr()) }
        }
    }
}

// Internal helpers use Rust linkage rather than foreign symbol lookup.
use libc::strndup;

pub use libc::{__errno_location, calloc, malloc, realloc, strcasecmp, strdup, write};

use crate::native_stdio::err;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn nullable_byte_order_keeps_native_binary_and_first_nul_semantics() {
        let values: &[Option<&[u8]>] = &[
            None,
            Some(b""),
            Some(b"Class"),
            Some(b"class"),
            Some(b"instance\xff"),
            Some(b"Class\0ignored\xff"),
        ];
        for &a in values {
            for &b in values {
                let expected = match (a, b) {
                    (None, None) => 0,
                    (None, Some(_)) => -1,
                    (Some(_), None) => 1,
                    (Some(a), Some(b)) => {
                        let a = std::ffi::CString::new(a.split(|&byte| byte == 0).next().unwrap())
                            .unwrap();
                        let b = std::ffi::CString::new(b.split(|&byte| byte == 0).next().unwrap())
                            .unwrap();
                        unsafe { libc::strcasecmp(a.as_ptr(), b.as_ptr()) }
                    }
                };
                assert_eq!(strcasecmp_nullable(a, b), expected);
            }
        }
    }

    use std::collections::VecDeque;
    use std::io::Read;
    use std::os::fd::AsRawFd;
    use std::os::unix::net::UnixStream;

    #[test]
    fn complete_writes_retry_interruptions_blocking_and_zero_progress() {
        let mut results = VecDeque::from([
            Err(io::Error::from_raw_os_error(libc::EINTR)),
            Err(io::Error::from_raw_os_error(libc::EAGAIN)),
            Ok(0),
            Ok(2),
            Ok(2),
        ]);
        let mut requests = Vec::new();
        let written = write_buffer(b"abcd", false, |bytes| {
            requests.push(bytes.to_vec());
            results.pop_front().expect("unexpected extra write")
        })
        .unwrap();
        assert_eq!(written, 4);
        assert!(results.is_empty());
        assert_eq!(
            requests,
            [b"abcd".as_slice(), b"abcd", b"abcd", b"abcd", b"cd"]
        );
    }

    #[test]
    fn available_writes_report_partial_progress_but_fatal_errors_discard_progress() {
        let mut results = VecDeque::from([
            Ok(2),
            Err(io::Error::from_raw_os_error(libc::EINTR)),
            Err(io::Error::from_raw_os_error(libc::EAGAIN)),
        ]);
        assert_eq!(
            write_buffer(b"abcd", true, |_| results.pop_front().unwrap()).unwrap(),
            2
        );
        assert!(results.is_empty());
        for partial_mode in [false, true] {
            let mut results = VecDeque::from([Ok(1), Err(io::Error::from_raw_os_error(libc::EIO))]);
            let error =
                write_buffer(b"abcd", partial_mode, |_| results.pop_front().unwrap()).unwrap_err();
            assert_eq!(error.raw_os_error(), Some(libc::EIO));
        }
    }

    #[test]
    fn slice_writes_preserve_binary_bytes_and_descriptor_ownership() {
        let (writer, mut reader) = UnixStream::pair().unwrap();
        let bytes = b"\0binary\xff";
        assert_eq!(write_all(writer.as_raw_fd(), bytes).unwrap(), bytes.len());
        assert_eq!(
            write_available(writer.as_raw_fd(), bytes).unwrap(),
            bytes.len()
        );
        let mut received = vec![0; 2 * bytes.len()];
        reader.read_exact(&mut received).unwrap();
        assert_eq!(received, bytes.repeat(2));
        assert_eq!(write_available(writer.as_raw_fd(), b"x").unwrap(), 1);
        reader.read_exact(&mut [0]).unwrap();
        // Empty slices retain zero-call behavior even for invalid descriptors.
        assert_eq!(write_all(-1, &[]).unwrap(), 0);
        assert_eq!(write_available(-1, &[]).unwrap(), 0);
        for error in [
            write_all(-1, b"x").unwrap_err(),
            write_available(-1, b"x").unwrap_err(),
        ] {
            assert_eq!(error.raw_os_error(), Some(libc::EBADF));
        }
    }

    #[test]
    fn real_nonblocking_socket_keeps_would_block_errno_and_descriptor() {
        let (writer, _reader) = UnixStream::pair().unwrap();
        writer.set_nonblocking(true).unwrap();
        let fd = writer.as_raw_fd();
        let block = [0xAB; 4096];
        loop {
            match write_once(fd, &block) {
                Ok(_) => continue,
                Err(error) if error.kind() == io::ErrorKind::WouldBlock => break,
                Err(error) => panic!("filling socket: {error}"),
            }
        }
        assert_eq!(write_available(fd, b"x").unwrap(), 0);
        assert_eq!(io::Error::last_os_error().kind(), io::ErrorKind::WouldBlock);
        // SAFETY: F_GETFD only queries flags; proves the borrowed fd stayed open.
        assert!(unsafe { libc::fcntl(fd, libc::F_GETFD) } >= 0);
    }
}
