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

/// Create an application listener with the historical backlog and diagnostics.
/// Glob resolution failures retain their fatal diagnostic policy. Other failures
/// return the captured OS error; the caller retains its previous published path.
#[deny(unsafe_op_in_unsafe_fn)]
pub fn create_socket(filename: impl AsRef<Path>) -> io::Result<BoundSocket> {
    use std::os::fd::AsRawFd;
    use std::os::unix::ffi::OsStrExt;
    let path = crate::resolve_tilde::resolve(filename)
        .unwrap_or_else(|error| crate::resolve_tilde::abort(error));
    let name = path.as_os_str().as_bytes().to_vec();
    match std::str::from_utf8(&name) {
        Ok(text) => ::log::debug!(
            "libi3/create_socket.c:create_socket:30 - Creating UNIX socket at {}\n",
            text
        ),
        Err(_) => crate::logging::emit_parts(
            crate::logging::Level::Debug,
            &[
                b"libi3/create_socket.c:create_socket:30 - Creating UNIX socket at ",
                &name,
                b"\n",
            ],
        ),
    }
    match bind_resolved(path) {
        Ok(socket) => {
            // SAFETY: this listener owns its live descriptor. Preserve backlog5;
            // the modern bind helper intentionally retains its default backlog.
            if unsafe { libc::listen(socket.listener.as_raw_fd(), 5) } < 0 {
                let error = io::Error::last_os_error();
                // SAFETY: native stderr and the literal diagnostic are live.
                unsafe {
                    libc::perror(c"listen()".as_ptr());
                }
                return Err(error);
            }
            Ok(socket)
        }
        Err(error) => {
            let code = error.raw_os_error().unwrap_or(libc::EIO);
            if code == libc::EEXIST {
                match std::str::from_utf8(&name) {
                    Ok(text) => ::log::error!("[libi3] ERROR: Refusing to create UNIX socket at {}: Socket is already in use\n", text),
                    Err(_) => crate::logging::emit_parts(crate::logging::Level::Error, &[b"[libi3] ERROR: Refusing to create UNIX socket at ", &name, b": Socket is already in use\n"]),
                }
            } else {
                // SAFETY: errno is thread-local and stderr remains initialized.
                // Bind may have run Rust cleanup; restore the captured OS error
                // before emitting the same native perror message.
                unsafe {
                    *libc::__errno_location() = code;
                    libc::perror(c"bind()".as_ptr());
                }
            }
            // Retain legacy errno for genuine native callers in the transition,
            // while Rust callers use the returned io::Error directly.
            unsafe {
                *libc::__errno_location() = code;
            }
            Err(error)
        }
    }
}
