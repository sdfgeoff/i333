/*
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE).
 */
//! Owned reads of root-window properties, with a legacy allocation adapter.
#![deny(unsafe_op_in_unsafe_fn)]

use std::ffi::{c_char, CStr, CString};
use std::ptr::{self, NonNull};
use std::slice;
use xcb_sys::{
    xcb_atom_t, xcb_aux_get_screen, xcb_connect, xcb_connection_has_error, xcb_connection_t,
    xcb_disconnect, xcb_get_property_reply, xcb_get_property_reply_t, xcb_get_property_unchecked,
    xcb_get_property_value, xcb_get_property_value_length, xcb_intern_atom, xcb_intern_atom_reply,
    xcb_window_t, XCB_ATOM_CARDINAL, XCB_GET_PROPERTY_TYPE_ANY,
};

/// An XCB reply allocated by the native library.
struct Reply<T>(NonNull<T>);

impl<T> Reply<T> {
    /// `pointer` must be null or a uniquely owned, malloc-allocated XCB reply.
    unsafe fn from_raw(pointer: *mut T) -> Option<Self> {
        NonNull::new(pointer).map(Self)
    }

    fn as_ptr(&self) -> *mut T {
        self.0.as_ptr()
    }

    fn header(&self) -> &T {
        // SAFETY: from_raw establishes ownership of a valid reply, kept alive by self.
        unsafe { self.0.as_ref() }
    }
}

impl<T> Drop for Reply<T> {
    fn drop(&mut self) {
        // SAFETY: XCB replies use malloc; this guard is their sole owner.
        unsafe { libc::free(self.as_ptr().cast()) }
    }
}

struct Connection(NonNull<xcb_connection_t>);

impl Connection {
    fn open(screen: &mut i32) -> Option<Self> {
        // SAFETY: null selects the configured display and screen points to an i32.
        let connection = Self(NonNull::new(unsafe { xcb_connect(ptr::null(), screen) })?);
        // SAFETY: a non-null xcb_connect result is valid even if connection failed.
        if unsafe { xcb_connection_has_error(connection.0.as_ptr()) } != 0 {
            return None;
        }
        Some(connection)
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        // SAFETY: this guard owns the connection returned by xcb_connect.
        unsafe { xcb_disconnect(self.0.as_ptr()) }
    }
}

/// Read a property using a temporary connection to the configured display.
/// Missing/empty properties and unavailable displays return None.
pub fn read_root_property(atom: &CStr) -> Option<CString> {
    let mut screen = 0;
    let connection = Connection::open(&mut screen)?;
    // SAFETY: this connection remains alive throughout the read.
    unsafe { read_on_connection(atom, connection.0.as_ptr(), screen) }
}

/// The connection must be live and usable for all native requests in this call.
unsafe fn request_property(
    connection: *mut xcb_connection_t,
    root: xcb_window_t,
    atom: xcb_atom_t,
    words: u32,
) -> Option<Reply<xcb_get_property_reply_t>> {
    // SAFETY: the caller guarantees a live connection. The reply transfers ownership.
    unsafe {
        let cookie = xcb_get_property_unchecked(
            connection,
            0,
            root,
            atom,
            XCB_GET_PROPERTY_TYPE_ANY,
            0,
            words,
        );
        Reply::from_raw(xcb_get_property_reply(connection, cookie, ptr::null_mut()))
    }
}

impl Reply<xcb_get_property_reply_t> {
    fn content(&self) -> Option<CString> {
        // SAFETY: self owns a complete native reply; these accessors only inspect it.
        let length =
            usize::try_from(unsafe { xcb_get_property_value_length(self.as_ptr()) }).ok()?;
        if length == 0 {
            return None;
        }
        let header = self.header();
        let payload_bytes = (header.length as usize).checked_mul(4)?;
        // Upstream reads the first u32 for CARDINAL, including protocol padding
        // for a nonstandard shorter CARDINAL. Keep that behavior without an
        // unaligned read or reading beyond the native reply's payload.
        let read_length = if header.type_0 == XCB_ATOM_CARDINAL {
            length.max(4)
        } else {
            length
        };
        if read_length > payload_bytes {
            return None;
        }
        // SAFETY: XCB owns at least header.length * 4 trailing bytes; the checked
        // length fits that payload and the reply stays alive while bytes is used.
        let bytes = unsafe {
            slice::from_raw_parts(
                xcb_get_property_value(self.as_ptr()).cast::<u8>(),
                read_length,
            )
        };
        property_content(header.type_0, bytes)
    }
}

fn property_content(kind: xcb_atom_t, bytes: &[u8]) -> Option<CString> {
    if bytes.is_empty() {
        return None;
    }
    if kind == XCB_ATOM_CARDINAL {
        let value = u32::from_ne_bytes(bytes.get(..4)?.try_into().ok()?);
        return Some(CString::new(value.to_string()).expect("decimal integers have no NUL"));
    }
    // The old %.*s conversion preserved arbitrary bytes but stopped at the first NUL.
    let prefix = bytes.split(|byte| *byte == 0).next().unwrap_or_default();
    Some(CString::new(prefix).expect("the first-NUL prefix has no NUL"))
}

/// `connection` must be live; screen data must belong to its complete setup buffer.
unsafe fn read_on_connection(
    atom: &CStr,
    connection: *mut xcb_connection_t,
    screen: i32,
) -> Option<CString> {
    // XCB's request has a u16 name length. Preserve the original truncating cast.
    // SAFETY: the CStr and connection remain valid throughout these requests.
    let (root, atom_reply) = unsafe {
        let cookie = xcb_intern_atom(connection, 0, atom.to_bytes().len() as u16, atom.as_ptr());
        let root = xcb_aux_get_screen(connection, screen).as_ref()?.root;
        let reply = Reply::from_raw(xcb_intern_atom_reply(connection, cookie, ptr::null_mut()))?;
        (root, reply)
    };
    let atom = atom_reply.header().atom;
    const INITIAL_WORDS: u32 = 256;
    // SAFETY: the caller guarantees the live connection used for these requests.
    let first = unsafe { request_property(connection, root, atom, INITIAL_WORDS)? };
    // SAFETY: first owns a native reply, valid for this accessor.
    let has_value = unsafe { xcb_get_property_value_length(first.as_ptr()) } > 0;
    let reply = if has_value && first.header().bytes_after > 0 {
        let words = INITIAL_WORDS + first.header().bytes_after.div_ceil(4);
        drop(first);
        // SAFETY: same live connection; the second reply takes independent ownership.
        unsafe { request_property(connection, root, atom, words)? }
    } else {
        first
    };
    reply.content()
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
    // SAFETY: the caller guarantees atomname is a live C string.
    let atom = unsafe { CStr::from_ptr(atomname) };
    let content = if provided_conn.is_null() {
        read_root_property(atom)
    } else {
        // SAFETY: the caller guarantees the borrowed connection and screen remain valid.
        unsafe { read_on_connection(atom, provided_conn, screen) }
    };
    content.map_or(ptr::null_mut(), |content| {
        // SAFETY: content is a live C string. sstrdup retains the historical
        // malloc/free contract and out-of-memory behavior for translated callers.
        unsafe { crate::safewrappers::sstrdup(content.as_ptr()) }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn reply(
        kind: u32,
        format: u8,
        value_len: u32,
        payload: &[u8],
    ) -> Reply<xcb_get_property_reply_t> {
        let words = payload.len().div_ceil(4);
        let size = std::mem::size_of::<xcb_get_property_reply_t>() + words * 4;
        // SAFETY: malloc provides aligned space for the fixed reply and padded
        // payload. Initialize every byte, including wire padding, before use.
        unsafe {
            let pointer = libc::malloc(size).cast::<xcb_get_property_reply_t>();
            assert!(!pointer.is_null());
            pointer.cast::<u8>().write_bytes(0, size);
            pointer.write(xcb_get_property_reply_t {
                response_type: 1,
                format,
                sequence: 0,
                length: words as u32,
                type_0: kind,
                bytes_after: 0,
                value_len,
                pad0: [0; 12],
            });
            ptr::copy_nonoverlapping(payload.as_ptr(), pointer.add(1).cast(), payload.len());
            Reply::from_raw(pointer).unwrap()
        }
    }

    #[test]
    fn native_payload_checks_keep_padding_and_reject_truncated_replies() {
        assert_eq!(
            reply(XCB_ATOM_CARDINAL, 8, 1, &[7])
                .content()
                .unwrap()
                .to_bytes(),
            u32::from_ne_bytes([7, 0, 0, 0]).to_string().as_bytes()
        );
        assert_eq!(reply(0, 8, 3, b"abc").content().unwrap().to_bytes(), b"abc");
        assert!(reply(0, 8, 9, b"abcd").content().is_none());
        assert!(reply(XCB_ATOM_CARDINAL, 32, 1, b"").content().is_none());
        assert!(reply(0, 8, 0, b"").content().is_none());
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
                property_content(XCB_ATOM_CARDINAL, &value.to_ne_bytes())
                    .unwrap()
                    .to_bytes(),
                value.to_string().as_bytes()
            );
        }
        assert!(property_content(XCB_ATOM_CARDINAL, &[1, 2, 3]).is_none());
    }
}
