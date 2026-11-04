/*
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE).
 */
//! Modifier lookup over checked keycode slices.
#![cfg_attr(not(test), forbid(unsafe_code))]

use xcb::x;
use xcb_interop::keysyms::KeySymbols;
/// Query a modifier mask using an owned native key-symbol cache.
pub fn query_modifier_mask(keysym: u32, symbols: &KeySymbols) -> u32 {
    let connection = symbols.connection();
    let _ = connection.flush();
    let cookie = connection.send_request(&x::GetModifierMapping {});
    connection
        .wait_for_reply(cookie)
        .map_or(0, |reply| modifier_mask_for(keysym, symbols, &reply))
}

/// Find the first modifier in a validated maintained reply without raw pointers.
pub fn modifier_mask_for(
    keysym: u32,
    symbols: &KeySymbols,
    reply: &x::GetModifierMappingReply,
) -> u32 {
    let Some(mapping) = modifier_keycodes(reply) else {
        return 0;
    };
    modifier_mask(
        &symbols.keycodes(keysym),
        mapping,
        usize::from(reply.keycodes_per_modifier()),
    )
}

/// Borrow the modifier keycodes after validating their advertised wire length.
pub fn modifier_keycodes(reply: &x::GetModifierMappingReply) -> Option<&[u8]> {
    let count = usize::from(reply.keycodes_per_modifier()) * 8;
    let available = usize::try_from(reply.length()).ok()?.checked_mul(4)?;
    (count <= available).then(|| reply.keycodes())
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
