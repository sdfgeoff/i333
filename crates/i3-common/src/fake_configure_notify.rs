/*
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE).
 */
//! Synthetic ConfigureNotify events with a fully initialized wire body.
#![deny(unsafe_op_in_unsafe_fn)]

use xcb::{x, Raw, XidNew};
use xcb_sys::{xcb_connection_t, xcb_rectangle_t, xcb_window_t};

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
    let event = configure_event(rectangle, window, border_width);
    // SAFETY: the caller retains the native connection for this request. Prefer
    // its registered Rust owner whenever that executable has migrated.
    let connection = crate::display::connection()
        .filter(|connection| connection.get_raw_conn().cast::<xcb_connection_t>() == conn)
        .or_else(|| unsafe { crate::xcb_connection::borrowed(conn) }.map(std::rc::Rc::new));
    if let Some(connection) = connection {
        connection.send_request(&x::SendEvent {
            propagate: false,
            destination: x::SendEventDest::Window(unsafe { x::Window::new(window) }),
            event_mask: x::EventMask::STRUCTURE_NOTIFY,
            event: &event,
        });
        let _ = connection.flush();
    }
}

fn configure_event(
    rectangle: xcb_rectangle_t,
    window: u32,
    border_width: i32,
) -> x::ConfigureNotifyEvent {
    // XIDs remain open numeric domains; the server checks their existence.
    let window = unsafe { x::Window::new(window) };
    let event = x::ConfigureNotifyEvent::new(
        window,
        window,
        <x::Window as xcb::Xid>::none(),
        rectangle.x,
        rectangle.y,
        rectangle.width,
        rectangle.height,
        border_width as u16,
        false,
    );
    // xcb1.7.1's typed constructor allocates 32 bytes but leaves these padding
    // bytes unwritten. Initialize them before SendEvent reads the whole body.
    // All protocol fields above are serialized by the maintained constructor.
    unsafe {
        let raw = event.as_raw().cast::<u8>();
        raw.add(1).write(0);
        raw.add(27).write_bytes(0, 5);
    }
    event
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn configure_notify_wire_body_matches_native_fields_and_initializes_all_32_bytes() {
        let rectangle = xcb_rectangle_t {
            x: -123,
            y: i16::MIN,
            width: u16::MAX,
            height: 456,
        };
        let event = configure_event(rectangle, 0x12345678, -1);
        // Independent complete wire fixture retained from the native boundary:
        // response byte, sequence/padding, two XIDs, sibling, geometry, border.
        let mut expected = [0; 32];
        expected[0] = 22;
        expected[4..8].copy_from_slice(&0x12345678_u32.to_ne_bytes());
        expected[8..12].copy_from_slice(&0x12345678_u32.to_ne_bytes());
        expected[16..18].copy_from_slice(&rectangle.x.to_ne_bytes());
        expected[18..20].copy_from_slice(&rectangle.y.to_ne_bytes());
        expected[20..22].copy_from_slice(&rectangle.width.to_ne_bytes());
        expected[22..24].copy_from_slice(&rectangle.height.to_ne_bytes());
        expected[24..26].copy_from_slice(&u16::MAX.to_ne_bytes());
        let bytes = unsafe { std::slice::from_raw_parts(event.as_raw().cast::<u8>(), 32) };
        assert_eq!(bytes, expected);
        assert_eq!(
            (event.event(), event.window()),
            (unsafe { x::Window::new(0x12345678) }, unsafe {
                x::Window::new(0x12345678)
            })
        );
        assert_eq!(
            (event.x(), event.y(), event.width(), event.height()),
            (rectangle.x, rectangle.y, rectangle.width, rectangle.height)
        );
        assert_eq!(event.border_width(), u16::MAX);
        assert_eq!(event.above_sibling(), <x::Window as xcb::Xid>::none());
        assert!(!event.override_redirect());
        assert_eq!(event.sequence(), 0);
    }
}
