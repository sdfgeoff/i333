// Copyright © 2009 Michael Stapelberg and contributors (see LICENSE).

use std::ffi::{CStr, OsStr};
use std::io;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::net::UnixStream;
use std::path::{Path, PathBuf};

/// Read a root-window property with the existing X11 backend. No connection or
/// returned allocation escapes this safe boundary. Missing properties and an
/// unavailable display both return None, matching the backend's current API.
///
/// This opens the configured X display; tests should prefer explicit IPC paths.
pub fn root_property(atom: &CStr) -> Option<Vec<u8>> {
    // SAFETY: atom is a borrowed C string; null requests a temporary connection.
    // The backend returns either null or a malloc-owned NUL-terminated string.
    let value = unsafe {
        crate::root_atom_contents::root_atom_contents(atom.as_ptr(), std::ptr::null_mut(), 0)
    };
    if value.is_null() {
        return None;
    }
    // SAFETY: Copy the C-owned property before freeing its original allocation.
    let result = unsafe {
        let result = CStr::from_ptr(value).to_bytes().to_vec();
        libc::free(value.cast());
        result
    };
    Some(result)
}

/// Resolve explicit path, I3SOCK, then I3_SOCKET_PATH on the X root window.
pub fn socket_path(explicit: Option<&Path>) -> io::Result<PathBuf> {
    if let Some(path) = explicit {
        return Ok(path.to_owned());
    }
    if let Some(path) = std::env::var_os("I3SOCK") {
        return Ok(path.into());
    }
    root_property(c"I3_SOCKET_PATH")
        .map(|bytes| PathBuf::from(OsStr::from_bytes(&bytes)))
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound,
                "Could not determine i3 socket path",
            )
        })
}

/// Connect to an explicit path. The returned stream owns its descriptor and has
/// close-on-exec set by the standard library.
pub fn connect_path(path: &Path) -> io::Result<UnixStream> {
    UnixStream::connect(path)
}

/// Discover the current i3 socket and connect with an owned stream.
pub fn connect(explicit: Option<&Path>) -> io::Result<UnixStream> {
    connect_path(&socket_path(explicit)?)
}

/// Legacy raw-descriptor and fatal-diagnostic boundary, pending caller migration.
mod legacy {
    use super::*;
    use std::ffi::CString;
    use std::os::fd::IntoRawFd;

    fn restore_errno(error: &io::Error) {
        // SAFETY: The current target uses Linux's errno ABI.
        unsafe { *libc::__errno_location() = error.raw_os_error().unwrap_or(libc::ENOENT) };
    }

    fn legacy_path(path: &CStr) -> PathBuf {
        // Original sockaddr_un construction retained at this boundary only:
        // older callers silently truncated paths to sun_path.len()-1 bytes.
        let capacity =
            std::mem::size_of::<libc::sockaddr_un>() - std::mem::size_of::<libc::sa_family_t>() - 1;
        let bytes = path.to_bytes();
        PathBuf::from(OsStr::from_bytes(&bytes[..bytes.len().min(capacity)]))
    }

    /// # Safety
    /// `path` must point to a live NUL-terminated C string.

    pub unsafe fn ipc_connect_impl(path: *const libc::c_char) -> libc::c_int {
        // SAFETY: The caller supplies a borrowed C string.
        let path = legacy_path(unsafe { CStr::from_ptr(path) });
        match connect_path(&path) {
            Ok(stream) => stream.into_raw_fd(),
            Err(error) => {
                restore_errno(&error);
                -1
            }
        }
    }

    /// # Safety
    /// `path` is null or points to a live NUL-terminated C string.

    pub unsafe fn ipc_connect(path: *const libc::c_char) -> libc::c_int {
        // SAFETY: A nonnull path follows the caller's C-string contract.
        let explicit = if path.is_null() {
            None
        } else {
            Some(Path::new(OsStr::from_bytes(
                unsafe { CStr::from_ptr(path) }.to_bytes(),
            )))
        };
        let path = match socket_path(explicit) {
            Ok(path) => path,
            Err(error) => {
                restore_errno(&error);
                // SAFETY: Static C format; err exits without unwinding across FFI.
                unsafe {
                    err(
                        libc::EXIT_FAILURE,
                        c"Could not determine i3 socket path".as_ptr(),
                    )
                }
            }
        };
        let c_path = CString::new(path.as_os_str().as_bytes()).expect("discovered path has no NUL");
        match connect_path(&legacy_path(&c_path)) {
            Ok(stream) => stream.into_raw_fd(),
            Err(error) => {
                restore_errno(&error);
                // SAFETY: c_path outlives the printf-compatible diagnostic.
                unsafe {
                    err(
                        libc::EXIT_FAILURE,
                        c"Could not connect to i3 on socket %s".as_ptr(),
                        c_path.as_ptr(),
                    )
                }
            }
        }
    }
}
pub use legacy::{ipc_connect, ipc_connect_impl};

use crate::native_stdio::err;
