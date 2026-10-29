// Copyright © 2009 Michael Stapelberg and contributors (see LICENSE).

use std::io;
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::{Path, PathBuf};

/// A listener owns its descriptor; the caller owns the filesystem socket path
/// and may unlink it during its normal shutdown sequence.
#[derive(Debug)]
pub struct BoundSocket {
    pub listener: UnixListener,
    pub path: PathBuf,
}

/// Resolve the requested path, create its parent directories, reject active
/// listeners, and replace stale socket files. The listener starts nonblocking.
pub fn bind(filename: impl AsRef<Path>) -> io::Result<BoundSocket> {
    bind_resolved(crate::resolve_tilde::resolve(filename)?)
}

fn bind_resolved(path: PathBuf) -> io::Result<BoundSocket> {
    let parent = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    if !crate::path_exists::exists(parent) {
        crate::mkdirp::create_dir_all(parent, 0o755)?;
    }
    if UnixStream::connect(&path).is_ok() {
        return Err(io::Error::from_raw_os_error(libc::EEXIST));
    }
    // Retain the original stale-path behavior: a failed probe is followed by an
    // unlink attempt, then bind reports any remaining filesystem error.
    let _ = std::fs::remove_file(&path);
    let listener = UnixListener::bind(&path)?;
    listener.set_nonblocking(true)?;
    Ok(BoundSocket { listener, path })
}

mod legacy {
    use super::*;
    use std::ffi::{CStr, CString, OsStr};
    use std::os::fd::{AsRawFd, IntoRawFd};
    use std::os::unix::ffi::OsStrExt;
    /// # Safety
    /// `filename` is a live C string. `out_path` points to a writable null or
    /// malloc-owned pointer; a successful replacement also belongs to free().

    pub unsafe fn create_socket(
        filename: *const libc::c_char,
        out_path: *mut *mut libc::c_char,
    ) -> libc::c_int {
        // SAFETY: The caller supplies a borrowed C string.
        let filename = unsafe { CStr::from_ptr(filename) };
        let path = crate::resolve_tilde::resolve(OsStr::from_bytes(filename.to_bytes()))
            .unwrap_or_else(|error| crate::resolve_tilde::abort(error));
        let name = CString::new(path.as_os_str().as_bytes()).expect("resolved path has no NUL");
        // SAFETY: Static formats and name remain live through logging calls.
        unsafe {
            debuglog(
                c"%s:%s:%d - Creating UNIX socket at %s\n".as_ptr(),
                c"libi3/create_socket.c".as_ptr(),
                c"create_socket".as_ptr(),
                30,
                name.as_ptr(),
            )
        };
        match bind_resolved(path) {
            Ok(socket) => {
                // SAFETY: Preserve the original backlog of five at the legacy
                // boundary; modern callers retain UnixListener's default backlog.
                if unsafe { libc::listen(socket.listener.as_raw_fd(), 5) } < 0 {
                    unsafe { libc::perror(c"listen()".as_ptr()) };
                    return -1;
                }
                // SAFETY: Replace the caller's malloc-owned path only after bind
                // and listen succeed; transfer the descriptor without closing it.
                unsafe {
                    libc::free((*out_path).cast());
                    *out_path = crate::safewrappers::sstrndup(name.as_ptr(), name.as_bytes().len());
                }
                socket.listener.into_raw_fd()
            }
            Err(error) => {
                // SAFETY: Preserve the OS error and active-socket diagnostic.
                unsafe {
                    let code = error.raw_os_error().unwrap_or(libc::EIO);
                    *libc::__errno_location() = code;
                    if code == libc::EEXIST {
                        errorlog(c"[libi3] ERROR: Refusing to create UNIX socket at %s: Socket is already in use\n".as_ptr(), name.as_ptr());
                    } else {
                        libc::perror(c"bind()".as_ptr());
                    }
                    *libc::__errno_location() = code;
                }
                -1
            }
        }
    }
}
pub use legacy::create_socket;

use crate::logging::{debuglog, errorlog};
