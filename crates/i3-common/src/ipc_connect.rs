// Copyright © 2009 Michael Stapelberg and contributors (see LICENSE).

use std::ffi::OsStr;
use std::io;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::net::UnixStream;
use std::path::{Path, PathBuf};

/// Read a root-window property with the existing X11 backend. No connection or
/// C-owned allocation escapes this safe boundary. Missing properties and an
/// unavailable display both return None, matching the backend's current API.
///
/// This opens the configured X display; tests should prefer explicit IPC paths.
pub fn root_property(atom: impl AsRef<[u8]>) -> Option<Vec<u8>> {
    crate::root_atom_contents::read_root_property(atom)
}

/// Resolve explicit path, I3SOCK, then I3_SOCKET_PATH on the X root window.
pub fn socket_path(explicit: Option<&Path>) -> io::Result<PathBuf> {
    if let Some(path) = explicit {
        return Ok(path.to_owned());
    }
    if let Some(path) = std::env::var_os("I3SOCK") {
        return Ok(path.into());
    }
    root_property(b"I3_SOCKET_PATH")
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

/// Connect using the historical native sockaddr path limit. Older clients
/// silently retained only the first 107 bytes; modern connect_path is unchanged.
pub fn connect_compatible_path(path: &Path) -> io::Result<UnixStream> {
    let capacity =
        std::mem::size_of::<libc::sockaddr_un>() - std::mem::size_of::<libc::sa_family_t>() - 1;
    let bytes = path.as_os_str().as_bytes();
    let bytes = &bytes[..bytes
        .iter()
        .position(|&byte| byte == 0)
        .unwrap_or(bytes.len())];
    connect_path(Path::new(OsStr::from_bytes(
        &bytes[..bytes.len().min(capacity)],
    )))
}

fn restore_errno(error: &io::Error) {
    // SAFETY: the Linux errno slot belongs to the calling thread. Preserve the
    // native fatal diagnostic's OS error, including missing discovery fallback.
    unsafe { *libc::__errno_location() = error.raw_os_error().unwrap_or(libc::ENOENT) };
}

/// Resolve and connect for clients whose historical failure contract exits.
/// The full original path appears in diagnostics even when its wire path was
/// truncated. The returned stream is the sole owner of its connected socket.
pub fn connect_or_exit(explicit: Option<&Path>) -> UnixStream {
    let path = match socket_path(explicit) {
        Ok(path) => path,
        Err(error) => {
            restore_errno(&error);
            // SAFETY: the static native diagnostic exits without unwinding FFI.
            unsafe {
                crate::native_stdio::err(
                    libc::EXIT_FAILURE,
                    c"Could not determine i3 socket path".as_ptr(),
                )
            }
        }
    };
    match connect_compatible_path(&path) {
        Ok(stream) => stream,
        Err(error) => {
            let path_bytes = path.as_os_str().as_bytes();
            let prefix = &path_bytes[..path_bytes
                .iter()
                .position(|&byte| byte == 0)
                .unwrap_or(path_bytes.len())];
            let native_path = std::ffi::CString::new(prefix).expect("first-NUL path prefix");
            restore_errno(&error);
            // SAFETY: the owned original path outlives the native fatal format.
            unsafe {
                crate::native_stdio::err(
                    libc::EXIT_FAILURE,
                    c"Could not connect to i3 on socket %s".as_ptr(),
                    native_path.as_ptr(),
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::{CStr, OsString};
    use std::io::{Read, Write};
    use std::os::unix::ffi::OsStringExt;
    use std::os::unix::net::UnixListener;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct PrivateDirectory(PathBuf);
    impl PrivateDirectory {
        fn new() -> Self {
            static NEXT: AtomicUsize = AtomicUsize::new(0);
            let root = Path::new(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .unwrap()
                .parent()
                .unwrap()
                .join(".work/ic");
            std::fs::create_dir_all(&root).unwrap();
            let path = root.join(format!(
                "{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            std::fs::create_dir(&path).unwrap();
            Self(path)
        }
        fn long_socket_path(&self) -> PathBuf {
            let mut bytes = self.0.join("socket-").as_os_str().as_bytes().to_vec();
            assert!(
                bytes.len() < 106,
                "test directory must leave room for a107-byte socket"
            );
            bytes.resize(106, b'x');
            bytes.push(0xff);
            bytes.extend_from_slice(b"-original-suffix");
            PathBuf::from(OsString::from_vec(bytes))
        }
    }
    impl Drop for PrivateDirectory {
        fn drop(&mut self) {
            std::fs::remove_dir_all(&self.0).unwrap();
        }
    }

    #[test]
    fn compatible_connection_keeps_native_truncation_bytes_and_socket_ownership() {
        let directory = PrivateDirectory::new();
        let original = directory.long_socket_path();
        let truncated = Path::new(OsStr::from_bytes(&original.as_os_str().as_bytes()[..107]));
        let listener = UnixListener::bind(truncated).unwrap();
        assert!(connect_path(Path::new(OsStr::from_bytes(
            original.as_os_str().as_bytes()
        )))
        .is_err());
        let mut client = connect_compatible_path(&original).unwrap();
        let (mut server, _) = listener.accept().unwrap();
        client.write_all(b"private\0\xff").unwrap();
        let mut payload = [0; 9];
        server.read_exact(&mut payload).unwrap();
        assert_eq!(&payload, b"private\0\xff");
        drop(client);
        assert_eq!(server.read(&mut payload).unwrap(), 0);
        // The bounded Path API preserves the old C input's first-NUL view.
        let short = directory.0.join("short");
        let listener = UnixListener::bind(&short).unwrap();
        let mut bytes = short.as_os_str().as_bytes().to_vec();
        bytes.extend_from_slice(b"\0ignored-\xff");
        let with_tail = PathBuf::from(OsString::from_vec(bytes));
        let client = connect_compatible_path(&with_tail).unwrap();
        let (mut server, _) = listener.accept().unwrap();
        drop(client);
        assert_eq!(server.read(&mut payload).unwrap(), 0);
    }

    #[test]
    fn fatal_connection_keeps_full_original_path_and_native_error() {
        const CHILD_PATH: &str = "I333_CONNECT_FATAL_CHILD";
        if let Some(path) = std::env::var_os(CHILD_PATH) {
            connect_or_exit(Some(Path::new(&path)));
            panic!("missing private socket must fail");
        }
        let directory = PrivateDirectory::new();
        let original = directory.long_socket_path();
        let path = Path::new(OsStr::from_bytes(original.as_os_str().as_bytes()));
        assert_eq!(
            connect_compatible_path(&original)
                .unwrap_err()
                .raw_os_error(),
            Some(libc::ENOENT)
        );
        let child = std::process::Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "ipc_connect::tests::fatal_connection_keeps_full_original_path_and_native_error",
                "--nocapture",
            ])
            .env(CHILD_PATH, path)
            .output()
            .unwrap();
        assert_eq!(child.status.code(), Some(libc::EXIT_FAILURE));
        let mut expected = b"Could not connect to i3 on socket ".to_vec();
        expected.extend_from_slice(original.as_os_str().as_bytes());
        expected.extend_from_slice(b": ");
        // SAFETY: strerror's borrowed native diagnostic is copied immediately.
        expected
            .extend_from_slice(unsafe { CStr::from_ptr(libc::strerror(libc::ENOENT)) }.to_bytes());
        expected.push(b'\n');
        assert!(child.stderr.ends_with(&expected), "{:?}", child.stderr);
    }
}
