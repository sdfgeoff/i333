//! Maintained XCB requests over an existing native connection.
//!
//! This view does not own or disconnect the native connection. Callers must
//! release the view before the event-thread owner closes that connection.
use xcb::ffi::xcb_connection_t;

/// Borrow a healthy native connection without transferring ownership.
///
/// # Safety
/// `raw` is null or points to a live XCB connection. The native connection must
/// remain live and usable until the returned view is dropped. No disconnect or
/// reentrant callback that could close it may occur while using this view.
#[deny(unsafe_op_in_unsafe_fn)]
pub unsafe fn borrowed(raw: *mut xcb_connection_t) -> Option<xcb::Connection> {
    if raw.is_null() {
        return None;
    }
    // SAFETY: caller guarantees a live native pointer; failed connections are
    // valid XCB objects but must not be passed to the maintained constructor.
    if unsafe { xcb_interop::transport::connection_has_error(raw) } {
        return None;
    }
    // SAFETY: the pointer is healthy and its existing owner outlives this view.
    // The no-drop constructor never adds a second disconnecting owner.
    Some(unsafe { xcb::Connection::from_raw_conn_and_extensions_no_drop(raw.cast(), &[], &[]) })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[link(name = "xcb")]
    unsafe extern "C" {
        fn xcb_connect(
            display: *const libc::c_char,
            screen: *mut libc::c_int,
        ) -> *mut xcb_connection_t;
        fn xcb_disconnect(connection: *mut xcb_connection_t);
    }

    #[test]
    fn borrowed_view_rejects_null_and_failed_native_connections() {
        // SAFETY: null is explicitly accepted; the malformed display produces
        // a failed but live native object without attempting a desktop connection.
        unsafe {
            assert!(borrowed(std::ptr::null_mut()).is_none());
            let raw = xcb_connect(c"not-a-display".as_ptr(), std::ptr::null_mut());
            assert!(!raw.is_null());
            assert!(borrowed(raw.cast()).is_none());
            xcb_disconnect(raw);
        }
    }
}
