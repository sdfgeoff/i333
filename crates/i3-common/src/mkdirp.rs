// Copyright © 2009 Michael Stapelberg and contributors (see LICENSE).

#![deny(unsafe_op_in_unsafe_fn)]

use std::io;
use std::os::unix::fs::DirBuilderExt;
use std::path::Path;

/// Create missing parent directories using the supplied mode and process umask.
/// Existing directories retain their permissions; an empty path is an error.
pub fn create_dir_all(path: impl AsRef<Path>, mode: u32) -> io::Result<()> {
    let path = path.as_ref();
    if path.as_os_str().is_empty() {
        return Err(io::Error::from_raw_os_error(libc::ENOENT));
    }
    std::fs::DirBuilder::new()
        .recursive(true)
        .mode(mode)
        .create(path)
}

/// Report a recursive mkdir failure with the historical native diagnostic.
/// The returned error retains mkdir's errno, except when the EEXIST follow-up
/// stat itself fails. Callers at native fatal boundaries may restore that errno.
pub fn report_create_error(path: impl AsRef<Path>, error: io::Error) -> io::Error {
    use std::ffi::CStr;
    use std::os::unix::ffi::OsStrExt;

    let path = path.as_ref();
    let code = error.raw_os_error().unwrap_or(libc::EIO);
    let mut returned_code = code;
    let mut display_code = code;
    let mut operation = b"mkdir".as_slice();
    if code == libc::EEXIST {
        match std::fs::metadata(path) {
            Ok(metadata) if !metadata.is_dir() => display_code = libc::ENOTDIR,
            Err(stat_error) => {
                display_code = stat_error.raw_os_error().unwrap_or(libc::EIO);
                returned_code = display_code;
                operation = b"stat";
            }
            _ => {}
        }
    }
    let mut message = b"[libi3] ERROR: ".to_vec();
    message.extend_from_slice(operation);
    message.push(b'(');
    message.extend_from_slice(path.as_os_str().as_bytes());
    message.extend_from_slice(b") failed: ");
    // SAFETY: strerror returns native locale error text for every integer code.
    // Copy its bytes before invoking the logging callback, as the old helper did.
    let description = unsafe { CStr::from_ptr(libc::strerror(display_code)) };
    message.extend_from_slice(description.to_bytes());
    message.push(b'\n');
    crate::logging::emit(crate::logging::Level::Error, &message);
    io::Error::from_raw_os_error(returned_code)
}
