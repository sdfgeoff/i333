// Copyright © 2009 Michael Stapelberg and contributors (see LICENSE).

use std::ffi::{CStr, OsStr, OsString};
use std::io;
use std::os::unix::ffi::{OsStrExt, OsStringExt};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

/// Owned directory for this instance's IPC, log, and other process files.
#[derive(Debug, Clone)]
pub struct RuntimePaths {
    directory: PathBuf,
}
impl RuntimePaths {
    /// Use runtime_dir/i3, or securely create /tmp/i3-<user>.XXXXXX when absent.
    pub fn new(runtime_dir: Option<&Path>) -> io::Result<Self> {
        let directory = if let Some(directory) = runtime_dir {
            let mut path = directory.as_os_str().to_owned();
            path.push("/i3");
            let path = PathBuf::from(path);
            crate::mkdirp::create_dir_all(&path, 0o700)?;
            path
        } else {
            let mut template = b"/tmp/i3-".to_vec();
            template.extend_from_slice(&username());
            template.extend_from_slice(b".XXXXXX\0");
            // SAFETY: mkdtemp receives a writable NUL-terminated template and
            // creates its directory atomically with owner-only permissions.
            let result = unsafe { libc::mkdtemp(template.as_mut_ptr().cast()) };
            if result.is_null() {
                return Err(io::Error::last_os_error());
            }
            template.pop();
            PathBuf::from(OsString::from_vec(template))
        };
        Ok(Self { directory })
    }
    pub fn from_environment() -> io::Result<Self> {
        let directory = std::env::var_os("XDG_RUNTIME_DIR").map(PathBuf::from);
        Self::new(directory.as_deref())
    }
    pub fn directory(&self) -> &Path {
        &self.directory
    }
    pub fn filename(&self, prefix: impl AsRef<OsStr>) -> PathBuf {
        let mut name = self.directory.as_os_str().to_owned();
        name.push("/");
        name.push(prefix);
        name.push(format!(".{}", std::process::id()));
        name.into()
    }
}

fn username() -> Vec<u8> {
    // SAFETY: sysconf only returns a suggested buffer length.
    let suggested = unsafe { libc::sysconf(libc::_SC_GETPW_R_SIZE_MAX) };
    let mut buffer = vec![
        0;
        if suggested > 0 {
            suggested as usize
        } else {
            1024
        }
    ];
    loop {
        let mut entry = std::mem::MaybeUninit::<libc::passwd>::uninit();
        let mut result = std::ptr::null_mut();
        // SAFETY: All output storage is live. getpwuid_r owns no static cursor;
        // any returned name is copied before the caller-owned buffer is dropped.
        let status = unsafe {
            libc::getpwuid_r(
                libc::getuid(),
                entry.as_mut_ptr(),
                buffer.as_mut_ptr().cast(),
                buffer.len(),
                &mut result,
            )
        };
        if status == libc::ERANGE {
            if let Some(size) = buffer.len().checked_mul(2) {
                buffer.resize(size, 0);
                continue;
            }
        }
        if status != 0 || result.is_null() {
            return b"unknown".to_vec();
        }
        // SAFETY: A successful getpwuid_r result supplies a live pw_name string.
        return unsafe { CStr::from_ptr((*result).pw_name) }
            .to_bytes()
            .to_vec();
    }
}

static PROCESS_PATHS: Mutex<Option<RuntimePaths>> = Mutex::new(None);
/// Lazily initialize the process directory once, without a mutable global pointer.
pub fn process_filename(prefix: impl AsRef<OsStr>) -> io::Result<PathBuf> {
    let mut paths = PROCESS_PATHS
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    if paths.is_none() {
        *paths = Some(RuntimePaths::from_environment()?);
    }
    Ok(paths.as_ref().unwrap().filename(prefix))
}

mod legacy {
    use super::*;
    use std::ffi::CString;
    /// # Safety
    /// `prefix` is a live C string. The returned allocation belongs to free().

    pub unsafe fn get_process_filename(prefix: *const libc::c_char) -> *mut libc::c_char {
        // SAFETY: The caller supplies a borrowed C string.
        let prefix = unsafe { CStr::from_ptr(prefix) };
        match process_filename(OsStr::from_bytes(prefix.to_bytes())) {
            Ok(path) => {
                let bytes = path.as_os_str().as_bytes();
                // SAFETY: Duplicate into the existing malloc allocation boundary.
                unsafe { crate::safewrappers::sstrndup(bytes.as_ptr().cast(), bytes.len()) }
            }
            Err(error) => {
                // SAFETY: Preserve fatal XDG permission errors and the original
                // nonfatal mkdtemp failure path, without unwinding over FFI.
                unsafe {
                    *libc::__errno_location() = error.raw_os_error().unwrap_or(libc::EIO);
                    if let Some(directory) = std::env::var_os("XDG_RUNTIME_DIR") {
                        let directory =
                            CString::new(directory.as_bytes()).expect("environment has no NUL");
                        let mut path = directory.as_bytes().to_vec();
                        path.extend_from_slice(b"/i3");
                        let path = CString::new(path).unwrap();
                        warn(c"Could not mkdirp(%s)".as_ptr(), path.as_ptr());
                        errx(
                            libc::EXIT_FAILURE,
                            c"Check permissions of $XDG_RUNTIME_DIR = '%s'".as_ptr(),
                            directory.as_ptr(),
                        );
                    }
                    libc::perror(c"mkdtemp()".as_ptr());
                }
                std::ptr::null_mut()
            }
        }
    }
}
pub use legacy::get_process_filename;

use crate::native_stdio::{errx, warn};
