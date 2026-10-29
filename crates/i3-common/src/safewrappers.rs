/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 */
// Native allocation compatibility and bounded descriptor writes.
#![deny(unsafe_op_in_unsafe_fn)]
use crate::i333_va_list;
use core::ffi::{c_char, c_int, c_void};
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

/// Format into a new malloc-owned string, terminating on allocation failure.
///
/// # Safety
/// strp must be writable, fmt must be a live C format string, and every variadic
/// argument must match its promoted C type and pointer/buffer requirements.
/// Successful output belongs to libc::free; no Rust allocation is substituted.
pub unsafe extern "C" fn sasprintf(strp: *mut *mut c_char, fmt: *const c_char, args: ...) -> c_int {
    let mut cursor = args.clone();
    // SAFETY: caller guarantees the output pointer and all native printf argument
    // types. The cloned cursor preserves va_copy semantics on both compiler ABIs.
    let result = unsafe { vasprintf(strp, fmt, i333_va_list!(cursor)) };
    if result == -1 {
        // SAFETY: fmt is a live C string and the diagnostic consumes no extra args.
        unsafe { err(libc::EXIT_FAILURE, c"asprintf(%s)".as_ptr(), fmt) };
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

fn native_write_result(result: io::Result<usize>) -> isize {
    match result {
        Ok(count) => count as isize,
        Err(error) => {
            // SAFETY: errno is thread-local on the supported Linux target.
            unsafe { *libc::__errno_location() = error.raw_os_error().unwrap_or(libc::EIO) };
            -1
        }
    }
}

/// Write the complete buffer, retrying EINTR and EAGAIN as the native helper did.
///
/// # Safety
/// buf must be readable for count bytes in one allocation for the whole call,
/// with count <= isize::MAX. Null is allowed only for zero; fd is never closed.
pub unsafe fn writeall(fd: RawFd, buf: *const libc::c_void, count: usize) -> isize {
    let bytes = if count == 0 {
        &[]
    } else {
        // SAFETY: caller guarantees a live readable allocation and valid slice size.
        unsafe { std::slice::from_raw_parts(buf.cast::<u8>(), count) }
    };
    native_write_result(write_all(fd, bytes))
}

/// Write until complete, returning progress on EAGAIN and retrying EINTR.
///
/// # Safety
/// buf must be readable for count bytes in one allocation for the whole call,
/// with count <= isize::MAX. Null is allowed only for zero; fd is never closed.
pub unsafe fn writeall_nonblock(fd: RawFd, buf: *const libc::c_void, count: usize) -> isize {
    let bytes = if count == 0 {
        &[]
    } else {
        // SAFETY: caller guarantees a live readable allocation and valid slice size.
        unsafe { std::slice::from_raw_parts(buf.cast::<u8>(), count) }
    };
    native_write_result(write_available(fd, bytes))
}

/// Write a complete buffer, terminating on an unrecoverable write failure.
///
/// # Safety
/// buf must satisfy writeall's readable-buffer contract throughout this call.
/// The borrowed fd is not closed, including when a fatal diagnostic is emitted.
pub unsafe fn swrite(fd: c_int, buf: *const c_void, count: usize) -> isize {
    // SAFETY: the caller supplies writeall's readable-buffer contract.
    let count = unsafe { writeall(fd, buf, count) };
    if count == -1 {
        // SAFETY: static format consumes the native integer fd, errno was retained.
        unsafe { err(libc::EXIT_FAILURE, c"Failed to write %d".as_ptr(), fd) };
    }
    count
}

/// Compare optional native strings using the current C locale. Null sorts first.
///
/// # Safety
/// Each non-null argument must remain a readable NUL-terminated string for the
/// comparison. Identical pointers (including two null pointers) compare equal.
pub unsafe fn strcasecmp_nullable(a: *const c_char, b: *const c_char) -> c_int {
    if a == b {
        return 0;
    }
    if a.is_null() {
        return -1;
    }
    if b.is_null() {
        return 1;
    }
    // SAFETY: both non-null strings satisfy the caller's C-string contract.
    unsafe { strcasecmp(a, b) }
}

// Internal helpers use Rust linkage rather than foreign symbol lookup.
use libc::strndup;

pub use libc::{__errno_location, calloc, malloc, realloc, strcasecmp, strdup, write};

use crate::native_stdio::{err, vasprintf};

#[cfg(test)]
mod tests {
    use super::*;
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
    fn safe_and_legacy_writes_preserve_binary_bytes_and_descriptor_ownership() {
        let (writer, mut reader) = UnixStream::pair().unwrap();
        let bytes = b"\0binary\xff";
        assert_eq!(write_all(writer.as_raw_fd(), bytes).unwrap(), bytes.len());
        // SAFETY: bytes remains readable while the borrowed raw descriptor is live.
        assert_eq!(
            unsafe { writeall(writer.as_raw_fd(), bytes.as_ptr().cast(), bytes.len()) },
            bytes.len() as isize
        );
        let mut received = vec![0; 2 * bytes.len()];
        reader.read_exact(&mut received).unwrap();
        assert_eq!(received, bytes.repeat(2));
        assert_eq!(write_available(writer.as_raw_fd(), b"x").unwrap(), 1);
        reader.read_exact(&mut [0]).unwrap();
        // Invalid descriptors and null empty buffers retain the native zero-call
        // behavior: neither the kernel nor the pointer is accessed for zero bytes.
        assert_eq!(write_all(-1, &[]).unwrap(), 0);
        assert_eq!(write_available(-1, &[]).unwrap(), 0);
        // SAFETY: zero bytes explicitly permit null buffers and invalid fds.
        unsafe {
            assert_eq!(writeall(-1, std::ptr::null(), 0), 0);
            assert_eq!(writeall_nonblock(-1, std::ptr::null(), 0), 0);
            assert_eq!(swrite(-1, std::ptr::null(), 0), 0);
        }
        assert_eq!(
            write_all(-1, b"x").unwrap_err().raw_os_error(),
            Some(libc::EBADF)
        );
        // SAFETY: one-byte input is live; the kernel rejects fd -1.
        assert_eq!(
            unsafe { writeall_nonblock(-1, b"x".as_ptr().cast(), 1) },
            -1
        );
        assert_eq!(io::Error::last_os_error().raw_os_error(), Some(libc::EBADF));
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
        // SAFETY: the input is live and the descriptor is a live full socket.
        assert_eq!(unsafe { writeall_nonblock(fd, b"x".as_ptr().cast(), 1) }, 0);
        assert_eq!(io::Error::last_os_error().kind(), io::ErrorKind::WouldBlock);
        // SAFETY: F_GETFD only queries flags; proves the borrowed fd stayed open.
        assert!(unsafe { libc::fcntl(fd, libc::F_GETFD) } >= 0);
    }

    #[test]
    fn swrite_preserves_fatal_native_error() {
        const CHILD: &str = "I333_SAFEWRAPPER_FATAL_CHILD";
        if std::env::var_os(CHILD).is_some() {
            // SAFETY: the byte buffer is valid; invalid fd is deliberately used
            // to exercise the original native fatal error in this isolated child.
            unsafe { swrite(-1, b"x".as_ptr().cast(), 1) };
            panic!("swrite must terminate on failure");
        }
        let child = std::process::Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "safewrappers::tests::swrite_preserves_fatal_native_error",
                "--nocapture",
            ])
            .env(CHILD, "1")
            .output()
            .unwrap();
        assert_eq!(child.status.code(), Some(libc::EXIT_FAILURE));
        assert!(child
            .stderr
            .windows(b"Failed to write -1".len())
            .any(|part| part == b"Failed to write -1"));
    }
}
