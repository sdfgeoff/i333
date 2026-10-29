/*
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE).
 */
//! Modifier lookup over checked keycode slices.
#![deny(unsafe_op_in_unsafe_fn)]

use std::ffi::CStr;
use std::ptr::{self, NonNull};
use std::slice;
use xcb_sys::{
    xcb_flush, xcb_get_modifier_mapping, xcb_get_modifier_mapping_keycodes,
    xcb_get_modifier_mapping_reply, xcb_get_modifier_mapping_reply_t, xcb_key_symbols_get_keycode,
    xcb_key_symbols_t,
};

/// A uniquely owned allocation returned by an XCB allocation API.
struct Allocation<T>(NonNull<T>);

impl<T> Allocation<T> {
    /// pointer must be null or a uniquely owned malloc-compatible allocation of T.
    unsafe fn from_raw(pointer: *mut T) -> Option<Self> {
        NonNull::new(pointer).map(Self)
    }

    fn as_ptr(&self) -> *mut T {
        self.0.as_ptr()
    }
}

impl<T> Drop for Allocation<T> {
    fn drop(&mut self) {
        // SAFETY: from_raw establishes sole ownership of a native allocation.
        unsafe { libc::free(self.as_ptr().cast()) }
    }
}

/// Fetch the current modifier map and find the first modifier containing keysym.
///
/// # Safety
/// symbols must be a live native key-symbol table, and the registered display
/// context must contain its live connection for the duration of this call.
pub unsafe fn aio_get_mod_mask_for(keysym: u32, symbols: *mut xcb_key_symbols_t) -> u32 {
    let connection = crate::display::context().connection;
    if connection.is_null() || symbols.is_null() {
        return 0;
    }
    // SAFETY: the caller guarantees the registered connection is live. The
    // native reply transfers sole ownership to the guard, including early exits.
    let reply = unsafe {
        xcb_flush(connection);
        let cookie = xcb_get_modifier_mapping(connection);
        Allocation::from_raw(xcb_get_modifier_mapping_reply(
            connection,
            cookie,
            ptr::null_mut(),
        ))
    };
    let Some(reply) = reply else {
        return 0;
    };
    // SAFETY: symbols is live by the caller's contract and reply is owned here.
    unsafe { get_mod_mask_for(keysym, symbols, reply.as_ptr()) }
}

/// Look up a key symbol in an existing native modifier-map reply.
///
/// # Safety
/// symbols must point to a live XCB key-symbol table. modmap_reply must point to
/// a complete live XCB modifier-map reply, including its declared payload.
pub unsafe fn get_mod_mask_for(
    keysym: u32,
    symbols: *mut xcb_key_symbols_t,
    modmap_reply: *mut xcb_get_modifier_mapping_reply_t,
) -> u32 {
    if symbols.is_null() || modmap_reply.is_null() {
        return 0;
    }
    // SAFETY: the caller provides a complete live reply. The length check below
    // prevents the advertised modifier count from exceeding its payload.
    let reply = unsafe { &*modmap_reply };
    let per_modifier = usize::from(reply.keycodes_per_modifier);
    let mapping_length = per_modifier * 8;
    if mapping_length == 0 || mapping_length > (reply.length as usize) * 4 {
        return 0;
    }
    // SAFETY: symbols is a live native table. This API returns a malloc-owned,
    // zero-terminated byte array, or null if there are no matching keycodes.
    let codes = unsafe { Allocation::from_raw(xcb_key_symbols_get_keycode(symbols, keysym)) };
    let Some(keycodes) = codes else {
        return 0;
    };
    // SAFETY: XCB guarantees the returned keycode list is zero-terminated. The
    // allocation guard outlives this borrowed byte slice and frees on every exit.
    let codes = unsafe { CStr::from_ptr(keycodes.as_ptr().cast()).to_bytes() };
    // SAFETY: the checked mapping length fits the complete reply's trailing
    // payload, and the caller keeps that reply alive throughout this call.
    let mapping = unsafe {
        slice::from_raw_parts(
            xcb_get_modifier_mapping_keycodes(modmap_reply),
            mapping_length,
        )
    };
    modifier_mask(codes, mapping, per_modifier)
}

fn modifier_mask(codes: &[u8], mapping: &[u8], per_modifier: usize) -> u32 {
    if per_modifier == 0 || mapping.len() != per_modifier.saturating_mul(8) {
        return 0;
    }
    mapping
        .chunks_exact(per_modifier)
        .position(|modifier| {
            modifier
                .iter()
                .any(|code| *code != 0 && codes.contains(code))
        })
        .map_or(0, |modifier| 1 << modifier)
}

#[cfg(test)]
mod tests {
    use super::modifier_mask;

    #[test]
    fn modifier_lookup_preserves_first_match_and_handles_empty_slots() {
        let mapping = [0, 0, 12, 34, 56, 78, 90, 91, 12, 0, 0, 0, 0, 0, 255, 0];
        assert_eq!(modifier_mask(&[12], &mapping, 2), 1 << 1);
        assert_eq!(modifier_mask(&[34, 56], &mapping, 2), 1 << 1);
        assert_eq!(modifier_mask(&[255], &mapping, 2), 1 << 7);
        assert_eq!(modifier_mask(&[111], &mapping, 2), 0);
        assert_eq!(modifier_mask(&[], &mapping, 2), 0);
        assert_eq!(modifier_mask(&[0], &mapping, 2), 0);
        assert_eq!(modifier_mask(&[12], &mapping[..15], 2), 0);
        assert_eq!(modifier_mask(&[12], &[], 0), 0);
    }
}
