/*
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE).
 */
//! Owned reads of root-window properties using maintained XCB requests/replies.
#![deny(unsafe_op_in_unsafe_fn)]

use std::ffi::{c_char, CStr, CString};
use std::{ptr, slice};
use xcb::{x, Xid};
use xcb_sys::xcb_connection_t;

/// Read a property using a temporary connection to the configured display.
/// Missing/empty properties and unavailable displays return None.
pub fn read_root_property(atom: &CStr) -> Option<CString> {
    let (connection, screen) = xcb::Connection::connect(None).ok()?;
    read_on_connection(atom, &connection, screen)
}

fn request_property(
    connection: &xcb::Connection,
    root: x::Window,
    atom: x::Atom,
    words: u32,
) -> Option<x::GetPropertyReply> {
    // This deliberately remains unchecked: native callers historically receive
    // errors through their event queue rather than consuming a checked error.
    let cookie = connection.send_request_unchecked(&x::GetProperty {
        delete: false,
        window: root,
        property: atom,
        r#type: x::ATOM_ANY,
        long_offset: 0,
        long_length: words,
    });
    connection.wait_for_reply_unchecked(cookie).ok().flatten()
}

/// Read the native byte view, including padding for short CARDINAL properties.
/// The maintained reply owns the allocation; only this legacy wire quirk needs
/// raw access. Validate declared content lengths before creating any slice.
fn reply_content(reply: &x::GetPropertyReply) -> Option<CString> {
    let payload_length = usize::try_from(reply.length()).ok()?.checked_mul(4)?;
    // SAFETY: a maintained GetPropertyReply owns an initialized 32-byte header
    // and its declared payload. Read only the fixed value_len header field.
    let raw = unsafe { xcb::Reply::as_raw(reply) };
    let value_len = unsafe { raw.add(16).cast::<u32>().read_unaligned() };
    let length = property_byte_length(reply.format(), value_len)?;
    if length == 0 {
        return None;
    }
    let read_length = if reply.r#type() == x::ATOM_CARDINAL {
        length.max(4)
    } else {
        length
    };
    if read_length > payload_length {
        return None;
    }
    // SAFETY: checked length fits the owned reply's trailing allocation and the
    // borrowed bytes cannot outlive that reply.
    let bytes = unsafe { slice::from_raw_parts(raw.add(32), read_length) };
    property_content(reply.r#type().resource_id(), bytes)
}

fn property_byte_length(format: u8, values: u32) -> Option<usize> {
    // Native xcb_get_property_value_length returns an int after unsigned u32
    // multiplication. Preserve that truncation/rejection for malformed counts.
    usize::try_from(values.wrapping_mul(u32::from(format) / 8) as i32).ok()
}

fn property_content(kind: u32, bytes: &[u8]) -> Option<CString> {
    if bytes.is_empty() {
        return None;
    }
    if kind == x::ATOM_CARDINAL.resource_id() {
        let value = u32::from_ne_bytes(bytes.get(..4)?.try_into().ok()?);
        return Some(CString::new(value.to_string()).expect("decimal integers have no NUL"));
    }
    let prefix = bytes.split(|byte| *byte == 0).next().unwrap_or_default();
    Some(CString::new(prefix).expect("the first-NUL prefix has no NUL"))
}

fn read_on_connection(atom: &CStr, connection: &xcb::Connection, screen: i32) -> Option<CString> {
    // XCB's original entry point truncated the name length to u16 before
    // serializing its bytes; don't serialize a longer slice after truncation.
    let length = atom.to_bytes().len() as u16 as usize;
    let cookie = connection.send_request(&x::InternAtom {
        only_if_exists: false,
        name: &atom.to_bytes()[..length],
    });
    let root = connection
        .get_setup()
        .roots()
        .nth(usize::try_from(screen).ok()?)?
        .root();
    let atom = connection.wait_for_reply(cookie).ok()?.atom();
    const INITIAL_WORDS: u32 = 256;
    let first = request_property(connection, root, atom, INITIAL_WORDS)?;
    // Check only the scalar length here: the native accessor did not read the
    // payload until content(), after any required second request.
    let raw = unsafe { xcb::Reply::as_raw(&first) };
    let value_len = unsafe { raw.add(16).cast::<u32>().read_unaligned() };
    let has_value =
        property_byte_length(first.format(), value_len).is_some_and(|length| length > 0);
    let reply = if has_value && first.bytes_after() > 0 {
        let words = INITIAL_WORDS + first.bytes_after().div_ceil(4);
        drop(first);
        request_property(connection, root, atom, words)?
    } else {
        first
    };
    reply_content(&reply)
}

/// Legacy caller-owned, malloc-compatible property result.
///
/// # Safety
/// atomname must point to a NUL-terminated string. A non-null provided_conn must
/// be a live XCB connection, with screen naming an entry in its setup buffer.
/// The returned non-null allocation must be released with libc::free.
pub unsafe fn root_atom_contents(
    atomname: *const c_char,
    provided_conn: *mut xcb_connection_t,
    screen: i32,
) -> *mut c_char {
    if atomname.is_null() {
        return ptr::null_mut();
    }
    let atom = unsafe { CStr::from_ptr(atomname) };
    let content = if provided_conn.is_null() {
        read_root_property(atom)
    } else {
        // SAFETY: caller retains the native owner for this synchronous read.
        unsafe { crate::xcb_connection::borrowed(provided_conn) }
            .and_then(|connection| read_on_connection(atom, &connection, screen))
    };
    content.map_or(ptr::null_mut(), |content| {
        // SAFETY: preserve the existing malloc/free transfer for legacy callers.
        unsafe { crate::safewrappers::sstrdup(content.as_ptr()) }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    fn reply(kind: u32, format: u8, values: u32, payload: &[u8]) -> x::GetPropertyReply {
        let words = payload.len().div_ceil(4);
        let size = 32 + words * 4;
        // SAFETY: initialize a complete malloc-backed reply with padded wire
        // payload; malformed value_len is intentional and tested before use.
        unsafe {
            let raw = libc::malloc(size).cast::<u8>();
            assert!(!raw.is_null());
            raw.write_bytes(0, size);
            raw.write(1);
            raw.add(1).write(format);
            raw.add(4).cast::<u32>().write_unaligned(words as u32);
            raw.add(8).cast::<u32>().write_unaligned(kind);
            raw.add(16).cast::<u32>().write_unaligned(values);
            ptr::copy_nonoverlapping(payload.as_ptr(), raw.add(32), payload.len());
            xcb::Reply::from_raw(raw)
        }
    }
    #[test]
    fn native_payload_checks_keep_padding_and_reject_truncated_replies() {
        let cardinal = x::ATOM_CARDINAL.resource_id();
        assert_eq!(
            reply_content(&reply(cardinal, 8, 1, &[7]))
                .unwrap()
                .to_bytes(),
            u32::from_ne_bytes([7, 0, 0, 0]).to_string().as_bytes()
        );
        assert_eq!(
            reply_content(&reply(0, 8, 3, b"abc")).unwrap().to_bytes(),
            b"abc"
        );
        assert!(reply_content(&reply(0, 8, 9, b"abcd")).is_none());
        assert!(reply_content(&reply(cardinal, 32, 1, b"")).is_none());
        assert!(reply_content(&reply(0, 8, 0, b"")).is_none());
        assert!(reply_content(&reply(0, 32, u32::MAX, b"abcd")).is_none());
    }
    #[test]
    fn property_bytes_keep_native_cardinal_and_c_string_rules() {
        assert!(property_content(0, b"").is_none());
        assert_eq!(
            property_content(0, b"ab\0ignored").unwrap().to_bytes(),
            b"ab"
        );
        assert_eq!(
            property_content(0, b"\xff\xfe").unwrap().to_bytes(),
            b"\xff\xfe"
        );
        assert_eq!(property_content(0, b"\0tail").unwrap().to_bytes(), b"");
        for value in [0, 1, u32::MAX] {
            assert_eq!(
                property_content(x::ATOM_CARDINAL.resource_id(), &value.to_ne_bytes())
                    .unwrap()
                    .to_bytes(),
                value.to_string().as_bytes()
            );
        }
        assert!(property_content(x::ATOM_CARDINAL.resource_id(), &[1, 2, 3]).is_none());
    }
}
