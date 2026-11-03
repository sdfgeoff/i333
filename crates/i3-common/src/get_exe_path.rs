// Copyright © 2009 Michael Stapelberg and contributors (see LICENSE).
#![deny(unsafe_op_in_unsafe_fn)]

use std::ffi::{CString, OsString};
use std::io;
use std::os::unix::ffi::{OsStrExt, OsStringExt};
use std::path::{Path, PathBuf};

/// Locate this executable, preserving the historical argv0/PATH fallback order.
/// If every lookup fails, retain i3's final /usr/bin/i3-nagbar fallback.
pub fn executable_path(argv0: impl AsRef<Path>) -> io::Result<PathBuf> {
    let argv0 = argv0.as_ref();
    if argv0.as_os_str().as_bytes().contains(&0) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "executable name contains NUL",
        ));
    }
    if let Ok(path) = std::env::current_exe() {
        return Ok(path);
    }
    if argv0.is_absolute() {
        return Ok(argv0.to_owned());
    }
    if argv0.as_os_str().as_bytes().contains(&b'/') {
        if let Ok(directory) = std::env::current_dir() {
            return Ok(directory.join(argv0));
        }
    }
    let search = std::env::var_os("PATH").unwrap_or_else(default_search_path);
    for component in std::env::split_paths(&search).filter(|path| !path.as_os_str().is_empty()) {
        let candidate = component.join(argv0);
        let c_path = CString::new(candidate.as_os_str().as_bytes()).map_err(|_| {
            io::Error::new(io::ErrorKind::InvalidInput, "executable path contains NUL")
        })?;
        // SAFETY: c_path is valid for access; X_OK preserves the access() executable
        // check, including ACLs, instead of guessing from mode bits.
        if unsafe { libc::access(c_path.as_ptr(), libc::X_OK) } == 0 {
            return Ok(candidate);
        }
    }
    Ok(PathBuf::from("/usr/bin/i3-nagbar"))
}

fn default_search_path() -> OsString {
    // SAFETY: A null, zero-length buffer asks confstr for the required size.
    let size = unsafe { libc::confstr(libc::_CS_PATH, std::ptr::null_mut(), 0) };
    if size == 0 {
        return OsString::from("/bin:/usr/bin");
    }
    let mut buffer = vec![0; size];
    // SAFETY: The allocated buffer is writable for its full length.
    let written =
        unsafe { libc::confstr(libc::_CS_PATH, buffer.as_mut_ptr().cast(), buffer.len()) };
    if written == 0 || written > buffer.len() {
        return OsString::from("/bin:/usr/bin");
    }
    buffer.truncate(written - 1);
    OsString::from_vec(buffer)
}
