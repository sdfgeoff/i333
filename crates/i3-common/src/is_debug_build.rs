// i3 - an improved tiling window manager
// Copyright © 2009 Michael Stapelberg and contributors (see LICENSE).
// Direct translation of libi3/is_debug_build.c; Legacy allocation ownership is preserved.

// The build must supply the same I3_VERSION as config.h. The fallback is the
// upstream version string for a checkout without generated configuration.
const I3_VERSION: &str = match option_env!("I3_VERSION") {
    Some(version) => version,
    None => "4.25-non-git",
};

pub fn is_debug_build() -> bool {
    match I3_VERSION.as_bytes().iter().position(|&ch| ch == b'(') {
        None => true,
        Some(offset) => offset > 10,
    }
}
