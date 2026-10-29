// Copyright © 2009 Michael Stapelberg and contributors (see LICENSE).

use std::path::Path;

/// Follow symbolic links and report whether metadata can be read for the path.
/// Missing paths and other metadata errors both return false, as stat did.
pub fn exists(path: impl AsRef<Path>) -> bool {
    std::fs::metadata(path).is_ok()
}

/// Legacy ABI adapter for callers that have not migrated to [`exists`].
///
/// # Safety
/// `path` must point to a live NUL-terminated C string.

pub unsafe fn path_exists(path: *const libc::c_char) -> bool {
    use std::os::unix::ffi::OsStrExt;
    // SAFETY: The caller guarantees a valid borrowed C string.
    let bytes = unsafe { std::ffi::CStr::from_ptr(path) }.to_bytes();
    exists(std::ffi::OsStr::from_bytes(bytes))
}
