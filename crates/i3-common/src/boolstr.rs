#![forbid(unsafe_code)]

// Copyright © 2009 Michael Stapelberg and contributors (see LICENSE).

/// Whether a configuration value names an enabled state. Whitespace is significant.
pub fn is_enabled(value: impl AsRef<[u8]>) -> bool {
    const ENABLED: [&[u8]; 6] = [b"1", b"yes", b"true", b"on", b"enable", b"active"];
    ENABLED
        .iter()
        .any(|word| value.as_ref().eq_ignore_ascii_case(word))
}
