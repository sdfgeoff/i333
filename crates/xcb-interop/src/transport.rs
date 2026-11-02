//! Narrow gaps in the maintained crate's public API, with no protocol schemas.
use xcb::{x, Cookie, Xid};

/// Query a native connection before the maintained borrowed constructor, which
/// asserts that the connection is healthy before exposing its public methods.
///
/// # Safety
/// `connection` must point to a live libxcb connection, including a failed one.
pub unsafe fn connection_has_error(connection: *mut xcb::ffi::xcb_connection_t) -> bool {
    // SAFETY: the caller retains a live native object throughout this query.
    unsafe { ffi::xcb_connection_has_error(connection) != 0 }
}

/// Discard a pending reply without blocking. xcb1.7.1's equivalent method is
/// private; retain its full 64-bit transport sequence instead of truncating it.
pub fn discard_reply<C: Cookie>(connection: &xcb::Connection, cookie: C) {
    // SAFETY: the borrowed owner keeps this native connection live for the call.
    unsafe { ffi::xcb_discard_reply64(connection.get_raw_conn(), cookie.sequence()) }
}

/// Preserve libxcb's barrier/check behavior for an unchecked request. Errors for
/// unchecked requests remain on the event queue; unlike `check_request`, this
/// deliberately does not treat the cookie as a checked request. Native sequence
/// projection matches libxcb's original u32 cookie ABI.
pub fn check_unchecked(connection: &xcb::Connection, cookie: xcb::VoidCookie) -> Option<u8> {
    // SAFETY: the connection is live, the native cookie is only its documented
    // u32 transport sequence, and libxcb owns the returned optional allocation.
    let error = unsafe {
        ffi::xcb_request_check(
            connection.get_raw_conn(),
            ffi::VoidCookie {
                sequence: cookie.sequence() as u32,
            },
        )
    };
    if error.is_null() {
        None
    } else {
        // SAFETY: a non-null result is a complete malloc-owned canonical error
        // header. Copy its scalar code before releasing that allocation once.
        unsafe {
            let code = (*error).error_code;
            libc::free(error.cast());
            Some(code)
        }
    }
}

/// Check the native status before using the maintained closed enum getter.
pub fn keyboard_grab_status(reply: &x::GrabKeyboardReply) -> Result<x::GrabStatus, u8> {
    // SAFETY: the concrete reply owns its complete initialized native header.
    let code = unsafe { xcb::Reply::as_raw(reply).add(1).read() };
    match code {
        0 => Ok(x::GrabStatus::Success),
        1 => Ok(x::GrabStatus::AlreadyGrabbed),
        2 => Ok(x::GrabStatus::InvalidTime),
        3 => Ok(x::GrabStatus::NotViewable),
        4 => Ok(x::GrabStatus::Frozen),
        code => Err(code),
    }
}

/// Grab an open numeric mouse button, including horizontal/additional buttons.
/// The maintained ButtonIndex enum only represents Any and buttons one to five.
/// Known buttons use maintained requests; only additional buttons cross the
/// native compatibility call. Native unchecked error and wire-width rules remain.
#[allow(clippy::too_many_arguments)]
pub fn grab_button(
    connection: &xcb::Connection,
    owner_events: bool,
    grab_window: x::Window,
    event_mask: x::EventMask,
    pointer_mode: x::GrabMode,
    keyboard_mode: x::GrabMode,
    confine_to: x::Window,
    cursor: x::Cursor,
    button: u8,
    modifiers: x::ModMask,
) {
    let known = match button {
        0 => Some(x::ButtonIndex::Any),
        1 => Some(x::ButtonIndex::N1),
        2 => Some(x::ButtonIndex::N2),
        3 => Some(x::ButtonIndex::N3),
        4 => Some(x::ButtonIndex::N4),
        5 => Some(x::ButtonIndex::N5),
        _ => None,
    };
    if let Some(button) = known {
        connection.send_request(&x::GrabButton {
            owner_events,
            grab_window,
            event_mask,
            pointer_mode,
            keyboard_mode,
            confine_to,
            cursor,
            button,
            modifiers,
        });
    } else {
        // SAFETY: all fields are scalar native values. The live connection
        // serializes this request through libxcb's original protocol function.
        // Native cookies are ignored exactly as in the existing passive grabs.
        unsafe {
            ffi::xcb_grab_button(
                connection.get_raw_conn(),
                u8::from(owner_events),
                grab_window.resource_id(),
                event_mask.bits() as u16,
                pointer_mode as u8,
                keyboard_mode as u8,
                confine_to.resource_id(),
                cursor.resource_id(),
                button,
                modifiers.bits() as u16,
            );
        }
    }
}

/// Preserve an unknown outgoing stacking selector without constructing an invalid
/// maintained StackMode enum. Normal selectors use typed ConfigureWindow instead.
/// The caller supplies native mask-order values; bounds are checked before FFI.
pub fn configure_window_with_open_stack_mode(
    connection: &xcb::Connection,
    window: x::Window,
    mask: u16,
    values: &[u32],
) -> u32 {
    assert_eq!(mask & !0x7f, 0, "unrecognized ConfigureWindow mask bits");
    assert_eq!(
        mask.count_ones() as usize,
        values.len(),
        "ConfigureWindow value count must match mask"
    );
    // SAFETY: the mask only names known fixed-width fields, and the validated
    // slice contains exactly one initialized value for each field in mask order.
    let cookie = unsafe {
        ffi::xcb_configure_window(
            connection.get_raw_conn(),
            window.resource_id(),
            mask,
            values.as_ptr().cast(),
        )
    };
    // Preserve the native sequence width. It cannot reconstruct the full u64
    // sequence required by maintained cookie types after libxcb's u32 wrap.
    cookie.sequence
}

mod ffi {
    use xcb::ffi::xcb_connection_t;
    // This is libxcb's native transport return ABI, not a wire message schema.
    #[repr(C)]
    pub struct VoidCookie {
        pub sequence: u32,
    }
    unsafe extern "C" {
        pub fn xcb_connection_has_error(connection: *mut xcb_connection_t) -> libc::c_int;
        pub fn xcb_request_check(
            connection: *mut xcb_connection_t,
            cookie: VoidCookie,
        ) -> *mut xcb::ffi::xcb_generic_error_t;
        pub fn xcb_configure_window(
            connection: *mut xcb_connection_t,
            window: u32,
            mask: u16,
            values: *const std::ffi::c_void,
        ) -> VoidCookie;
        pub fn xcb_discard_reply64(connection: *mut xcb_connection_t, sequence: u64);
        pub fn xcb_grab_button(
            connection: *mut xcb_connection_t,
            owner_events: u8,
            window: u32,
            event_mask: u16,
            pointer_mode: u8,
            keyboard_mode: u8,
            confine_to: u32,
            cursor: u32,
            button: u8,
            modifiers: u16,
        ) -> VoidCookie;
    }
}
