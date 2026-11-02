/*
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE).
 */
//! Modifier lookup over checked keycode slices.
#![deny(unsafe_op_in_unsafe_fn)]

use std::ffi::CStr;
use std::ptr::NonNull;
use xcb::x;
use xcb_sys::{xcb_key_symbols_get_keycode, xcb_key_symbols_t};

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
    let raw = crate::display::context().connection;
    if symbols.is_null() {
        return 0;
    }
    // SAFETY: the provider keeps the sole native owner live for this call.
    let Some(connection) = (unsafe { crate::xcb_connection::borrowed(raw) }) else {
        return 0;
    };
    let _ = connection.flush();
    let cookie = connection.send_request(&x::GetModifierMapping {});
    let Ok(reply) = connection.wait_for_reply(cookie) else {
        return 0;
    };
    // SAFETY: symbols remains live under the caller's contract.
    unsafe { get_mod_mask_for(keysym, symbols, &reply) }
}

/// Borrow the modifier keycodes after validating their advertised wire length.
pub fn modifier_keycodes(reply: &x::GetModifierMappingReply) -> Option<&[u8]> {
    let count = usize::from(reply.keycodes_per_modifier()) * 8;
    let available = usize::try_from(reply.length()).ok()?.checked_mul(4)?;
    (count <= available).then(|| reply.keycodes())
}

/// Look up a key symbol in an owned maintained modifier-map reply.
///
/// # Safety
/// symbols must point to a live XCB key-symbol table.
pub unsafe fn get_mod_mask_for(
    keysym: u32,
    symbols: *mut xcb_key_symbols_t,
    reply: &x::GetModifierMappingReply,
) -> u32 {
    if symbols.is_null() {
        return 0;
    }
    let per_modifier = usize::from(reply.keycodes_per_modifier());
    let Some(mapping) = modifier_keycodes(reply).filter(|_| per_modifier != 0) else {
        return 0;
    };
    // SAFETY: symbols is a live native table. This API returns a malloc-owned,
    // zero-terminated byte array, or null if there are no matching keycodes.
    let codes = unsafe { Allocation::from_raw(xcb_key_symbols_get_keycode(symbols, keysym)) };
    let Some(keycodes) = codes else {
        return 0;
    };
    // SAFETY: XCB guarantees the returned keycode list is zero-terminated. The
    // allocation guard outlives this borrowed byte slice and frees on every exit.
    let codes = unsafe { CStr::from_ptr(keycodes.as_ptr().cast()).to_bytes() };
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
    use super::*;

    #[test]
    fn maintained_mapping_checks_wire_length_before_borrowing_keycodes() {
        // A complete owned reply with a deliberately inconsistent count must
        // never reach the maintained unchecked payload accessor.
        unsafe {
            let raw = libc::calloc(1, 32).cast::<u8>();
            assert!(!raw.is_null());
            raw.write(1);
            raw.add(1).write(2);
            let reply: x::GetModifierMappingReply = xcb::Reply::from_raw(raw);
            assert!(modifier_keycodes(&reply).is_none());
        }
    }

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
