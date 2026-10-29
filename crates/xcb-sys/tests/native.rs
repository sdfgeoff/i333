use std::mem::{offset_of, size_of};
use xcb_sys::*;

#[test]
fn native_protocol_layouts() {
    assert_eq!(size_of::<xcb_generic_event_t>(), 36);
    assert_eq!(size_of::<xcb_generic_error_t>(), 36);
    assert_eq!(size_of::<xcb_char2b_t>(), 2);
    assert_eq!(size_of::<xcb_rectangle_t>(), 8);
    assert_eq!(size_of::<xcb_screen_t>(), 40);
    assert_eq!(size_of::<xcb_setup_t>(), 40);
    assert_eq!(size_of::<xcb_client_message_event_t>(), 32);
    assert_eq!(offset_of!(xcb_generic_event_t, full_sequence), 32);
    assert_eq!(offset_of!(xcb_generic_error_t, full_sequence), 32);
    assert_eq!(offset_of!(xcb_screen_t, root_visual), 32);
    assert_eq!(offset_of!(xcb_client_message_event_t, data), 12);
}

#[test]
fn malformed_display_returns_owned_failed_connection() {
    // A display without ':' fails parsing before any connection attempt.
    let display = c"not-a-display";
    unsafe {
        let connection = xcb_connect(display.as_ptr(), std::ptr::null_mut());
        assert!(!connection.is_null());
        assert_eq!(xcb_connection_has_error(connection), 5);
        xcb_disconnect(connection);
    }
}
