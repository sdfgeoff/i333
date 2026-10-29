// Copyright © 2009 Michael Stapelberg and contributors (see LICENSE).

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

mod legacy {
    use super::*;
    use std::ffi::CStr;
    use std::os::unix::ffi::OsStrExt;

    /// # Safety
    /// `path` is a live C string. This adapter retains integer/errno diagnostics.

    pub unsafe fn mkdirp(path: *const libc::c_char, mode: libc::mode_t) -> libc::c_int {
        // SAFETY: The caller provides a borrowed C string.
        let raw = unsafe { CStr::from_ptr(path) };
        let target = Path::new(std::ffi::OsStr::from_bytes(raw.to_bytes()));
        match create_dir_all(target, mode) {
            Ok(()) => 0,
            Err(error) => {
                let code = error.raw_os_error().unwrap_or(libc::EIO);
                let mut display_code = code;
                let mut format = c"[libi3] ERROR: mkdir(%s) failed: %s\n";
                if code == libc::EEXIST {
                    match std::fs::metadata(target) {
                        Ok(metadata) if !metadata.is_dir() => display_code = libc::ENOTDIR,
                        Err(stat_error) => {
                            display_code = stat_error.raw_os_error().unwrap_or(libc::EIO);
                            format = c"[libi3] ERROR: stat(%s) failed: %s\n";
                        }
                        _ => {}
                    }
                }
                // SAFETY: raw and the static format are valid C strings. strerror
                // returns borrowed error text; errno retains the original failure.
                unsafe {
                    errorlog(format.as_ptr(), raw.as_ptr(), libc::strerror(display_code));
                    *libc::__errno_location() = if format == c"[libi3] ERROR: stat(%s) failed: %s\n"
                    {
                        display_code
                    } else {
                        code
                    };
                }
                -1
            }
        }
    }
}
pub use legacy::mkdirp;

use crate::logging::errorlog;
