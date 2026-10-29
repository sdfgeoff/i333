// Copyright © 2009 Michael Stapelberg and contributors (see LICENSE).

/// Whether a configuration value names an enabled state. Whitespace is significant.
pub fn is_enabled(value: impl AsRef<[u8]>) -> bool {
    const ENABLED: [&[u8]; 6] = [b"1", b"yes", b"true", b"on", b"enable", b"active"];
    ENABLED
        .iter()
        .any(|word| value.as_ref().eq_ignore_ascii_case(word))
}

/// Legacy ABI adapter for callers that have not migrated to [`is_enabled`].
///
/// # Safety
/// `value` must point to a live NUL-terminated C string.

pub unsafe fn boolstr(value: *const libc::c_char) -> bool {
    // SAFETY: The caller guarantees a valid C string for the duration of this call.
    is_enabled(unsafe { std::ffi::CStr::from_ptr(value) }.to_bytes())
}
