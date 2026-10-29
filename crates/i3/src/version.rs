/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * Stores the latest Git commit identifier so that it can be linked into i3
 * and used dynamically without recompiling every object file.
 *
 */
pub const I3_VERSION: &[u8] = concat!(env!("I3_VERSION"), "\0").as_bytes();

// Immutable compatibility pointer into process-lifetime build metadata.
pub const i3_version: *const ::core::ffi::c_char =
    I3_VERSION.as_ptr().cast::<::core::ffi::c_char>();
