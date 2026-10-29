// Copyright © 2009 Michael Stapelberg and contributors (see LICENSE).

use std::ffi::{CStr, CString, OsString};
use std::io;
use std::os::unix::ffi::{OsStrExt, OsStringExt};
use std::path::{Path, PathBuf};

/// Resolve tilde or glob syntax in only the first path component. Zero or
/// multiple matches leave the original path unchanged, including its spelling.
pub fn resolve(path: impl AsRef<Path>) -> io::Result<PathBuf> {
    let path = path.as_ref();
    let bytes = path.as_os_str().as_bytes();
    if bytes.contains(&0) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "path contains NUL",
        ));
    }
    let split = bytes
        .iter()
        .position(|&byte| byte == b'/')
        .unwrap_or(bytes.len());
    let (head, tail) = bytes.split_at(split);
    if !head.starts_with(b"~") && !head.iter().any(|byte| b"*?[\\".contains(byte)) {
        return Ok(path.to_owned());
    }
    let pattern = CString::new(head).expect("NUL checked above");
    // SAFETY: A zeroed glob_t is the documented initial state. Glob owns any
    // subsequently allocated paths; Drop releases them on every exit path.
    let mut matches = Glob(unsafe { std::mem::zeroed() });
    let status = unsafe { libc::glob(pattern.as_ptr(), libc::GLOB_TILDE, None, &mut matches.0) };
    if status == libc::GLOB_NOMATCH || matches.0.gl_pathc != 1 {
        return Ok(path.to_owned());
    }
    if status != 0 {
        return Err(io::Error::new(
            if status == libc::GLOB_NOSPACE {
                io::ErrorKind::OutOfMemory
            } else {
                io::ErrorKind::Other
            },
            "glob() failed",
        ));
    }
    // SAFETY: Successful glob with one match supplies a live C string until Drop.
    let matched = unsafe { CStr::from_ptr(*matches.0.gl_pathv) }.to_bytes();
    let mut result = Vec::with_capacity(matched.len() + tail.len());
    result.extend_from_slice(matched);
    result.extend_from_slice(tail);
    Ok(PathBuf::from(OsString::from_vec(result)))
}

struct Glob(libc::glob_t);
impl Drop for Glob {
    fn drop(&mut self) {
        // SAFETY: This is the same initialized glob_t passed to glob, freed once.
        unsafe { libc::globfree(&mut self.0) }
    }
}

mod legacy {
    use super::*;

    pub(crate) fn abort(error: io::Error) -> ! {
        // SAFETY: Preserve the fatal legacy glob diagnostic without unwinding.
        unsafe {
            *libc::__errno_location() = error.raw_os_error().unwrap_or(libc::ENOMEM);
            err(libc::EXIT_FAILURE, c"glob() failed".as_ptr());
        }
    }

    /// Legacy malloc/free adapter. The caller supplies a live C string.
    ///
    /// # Safety
    /// `path` must be a live NUL-terminated C string.

    pub unsafe fn resolve_tilde(path: *const libc::c_char) -> *mut libc::c_char {
        // SAFETY: path follows the caller's C-string contract.
        let input = unsafe { CStr::from_ptr(path) };
        let resolved = resolve(std::ffi::OsStr::from_bytes(input.to_bytes()))
            .unwrap_or_else(|error| abort(error));
        let bytes = resolved.as_os_str().as_bytes();
        // SAFETY: Duplicate into malloc-owned storage for unconverted callers.
        unsafe { crate::safewrappers::sstrndup(bytes.as_ptr().cast(), bytes.len()) }
    }
}
pub(crate) use legacy::abort;
pub use legacy::resolve_tilde;

use crate::native_stdio::err;
