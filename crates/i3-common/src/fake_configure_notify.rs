/*
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE).
 */
//! Synthetic ConfigureNotify events with a fully initialized wire body.
#![deny(unsafe_op_in_unsafe_fn)]

use xcb_sys::{
    xcb_connection_t, xcb_flush, xcb_rectangle_t, xcb_send_event, xcb_window_t,
    XCB_CONFIGURE_NOTIFY, XCB_EVENT_MASK_STRUCTURE_NOTIFY,
};

/// Send a synthetic ConfigureNotify for an existing window.
///
/// # Safety
/// conn must be a live XCB connection for the duration of the request/flush.
pub unsafe fn fake_configure_notify(
    conn: *mut xcb_connection_t,
    rectangle: xcb_rectangle_t,
    window: xcb_window_t,
    border_width: i32,
) {
    let event = event_bytes(rectangle, window, border_width);
    // SAFETY: the caller supplies a live connection. xcb_send_event copies the
    // complete 32-byte event while its initialized stack buffer remains alive.
    unsafe {
        xcb_send_event(
            conn,
            0,
            window,
            XCB_EVENT_MASK_STRUCTURE_NOTIFY,
            event.as_ptr().cast(),
        );
        xcb_flush(conn);
    }
}

fn event_bytes(rectangle: xcb_rectangle_t, window: xcb_window_t, border_width: i32) -> [u8; 32] {
    // SendEvent always reads 32 bytes, although xcb_configure_notify_event_t is
    // only 28 bytes. Build the wire body directly so neither struct padding nor
    // bytes beyond a native event struct can be observed by the native library.
    let mut event = [0; 32];
    event[0] = XCB_CONFIGURE_NOTIFY as u8;
    event[4..8].copy_from_slice(&window.to_ne_bytes());
    event[8..12].copy_from_slice(&window.to_ne_bytes());
    // above_sibling, override_redirect, sequence and protocol padding stay zero.
    event[16..18].copy_from_slice(&rectangle.x.to_ne_bytes());
    event[18..20].copy_from_slice(&rectangle.y.to_ne_bytes());
    event[20..22].copy_from_slice(&rectangle.width.to_ne_bytes());
    event[22..24].copy_from_slice(&rectangle.height.to_ne_bytes());
    event[24..26].copy_from_slice(&(border_width as u16).to_ne_bytes());
    event
}

#[cfg(test)]
mod tests {
    use super::*;
    use xcb_sys::xcb_configure_notify_event_t;

    #[test]
    fn configure_notify_wire_body_matches_native_fields_and_initializes_all_32_bytes() {
        let rectangle = xcb_rectangle_t {
            x: -123,
            y: i16::MIN,
            width: u16::MAX,
            height: 456,
        };
        let bytes = event_bytes(rectangle, 0x12345678, -1);
        assert_eq!(std::mem::size_of::<xcb_configure_notify_event_t>(), 28);
        // SAFETY: every byte is initialized, all native fields accept arbitrary
        // integer bits, and read_unaligned needs no alignment from this array.
        let decoded = unsafe {
            bytes
                .as_ptr()
                .cast::<xcb_configure_notify_event_t>()
                .read_unaligned()
        };
        assert_eq!(decoded.response_type, XCB_CONFIGURE_NOTIFY as u8);
        assert_eq!((decoded.event, decoded.window), (0x12345678, 0x12345678));
        assert_eq!((decoded.x, decoded.y), (rectangle.x, rectangle.y));
        assert_eq!(
            (decoded.width, decoded.height),
            (rectangle.width, rectangle.height)
        );
        assert_eq!(decoded.border_width, u16::MAX);
        assert_eq!(decoded.above_sibling, 0);
        assert_eq!(decoded.override_redirect, 0);
        assert_eq!(decoded.sequence, 0);
        assert_eq!(bytes[1], 0);
        assert_eq!(&bytes[26..32], &[0; 6]);
    }
}
