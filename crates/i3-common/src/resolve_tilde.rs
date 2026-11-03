// Copyright © 2009 Michael Stapelberg and contributors (see LICENSE).
#![deny(unsafe_op_in_unsafe_fn)]

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

/// Resolve a native path into Rust-owned, NUL-terminated storage. Native callers
/// retain the historical fatal glob diagnostic without malloc ownership transfer.
pub fn resolve_cstring(path: &CStr) -> CString {
    let resolved =
        resolve(std::ffi::OsStr::from_bytes(path.to_bytes())).unwrap_or_else(|error| abort(error));
    CString::new(resolved.as_os_str().as_bytes()).expect("resolved OS path contains no NUL")
}

pub(crate) fn abort(error: io::Error) -> ! {
    // SAFETY: Preserve the fatal glob diagnostic and errno without unwinding.
    unsafe {
        *libc::__errno_location() = error.raw_os_error().unwrap_or(libc::ENOMEM);
        crate::native_stdio::err(libc::EXIT_FAILURE, c"glob() failed".as_ptr());
    }
}

#[cfg(test)]
mod tests {
    use super::resolve_cstring;
    use std::ffi::CStr;

    #[test]
    fn native_paths_retain_non_utf8_and_expand_only_the_first_component() {
        let literal = CStr::from_bytes_with_nul(b"literal/\xff//tail*\0").unwrap();
        assert_eq!(resolve_cstring(literal).as_c_str(), literal);
        // This first component matches the package manifest; the non-UTF-8
        // tail and its glob syntax must remain byte-for-byte untouched.
        let globbed = CStr::from_bytes_with_nul(b"Cargo.tom?/\xff/tail*\0").unwrap();
        assert_eq!(
            resolve_cstring(globbed).to_bytes(),
            b"Cargo.toml/\xff/tail*"
        );
    }
}
