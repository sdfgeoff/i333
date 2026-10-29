/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * sync.c: i3 sync protocol: https://i3wm.org/docs/testsuite.html#i3_sync
 *
 */
#![deny(unsafe_op_in_unsafe_fn)]

use crate::application::{conn, A_I3_SYNC};
use crate::log::debuglog;
use xcb_sys::{
    xcb_atom_t, xcb_client_message_data_t, xcb_client_message_event_t, xcb_flush, xcb_send_event,
    xcb_window_t, XCB_CLIENT_MESSAGE, XCB_EVENT_MASK_NO_EVENT,
};

// SendEvent copies exactly 32 bytes. These assertions also exclude padding in
// the fully initialized stack message, including its 20-byte data union.
const _: () = {
    assert!(std::mem::size_of::<xcb_client_message_event_t>() == 32);
    assert!(std::mem::offset_of!(xcb_client_message_event_t, data) == 12);
};

fn sync_reply(window: xcb_window_t, random: u32, atom: xcb_atom_t) -> xcb_client_message_event_t {
    xcb_client_message_event_t {
        response_type: XCB_CLIENT_MESSAGE as u8,
        format: 32,
        sequence: 0,
        window,
        type_0: atom,
        data: xcb_client_message_data_t {
            data32: [window, random, 0, 0, 0],
        },
    }
}

/// Reply using the initialized display connection and i3 sync atom.
///
/// # Safety
/// The caller must run on the display event thread with a live `conn` and
/// initialized `A_I3_SYNC`, without concurrent mutation of those globals.
pub unsafe fn sync_respond(window: xcb_window_t, rnd: u32) {
    // SAFETY: The caller supplies exclusive display-thread access; the native
    // logging arguments match the format and SendEvent copies the stack message
    // synchronously before returning.
    unsafe {
        debuglog(
            c"%s:%s:%d - [i3 sync protocol] Sending random value %d back to X11 window 0x%08x\n"
                .as_ptr()
                .cast_mut(),
            c"../src/sync.c".as_ptr(),
            c"sync_respond".as_ptr(),
            13i32,
            rnd,
            window,
        );
        let reply = sync_reply(window, rnd, A_I3_SYNC);
        xcb_send_event(
            conn,
            0,
            window,
            XCB_EVENT_MASK_NO_EVENT,
            (&raw const reply).cast(),
        );
        xcb_flush(conn);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reply_has_exact_native_client_message_wire_contents() {
        for (window, random, atom) in [(0, 0, 0), (u32::MAX, 0x12345678, 0x87654321)] {
            let reply = sync_reply(window, random, atom);
            let mut expected = [0u8; 32];
            expected[0] = XCB_CLIENT_MESSAGE as u8;
            expected[1] = 32;
            expected[4..8].copy_from_slice(&window.to_ne_bytes());
            expected[8..12].copy_from_slice(&atom.to_ne_bytes());
            expected[12..16].copy_from_slice(&window.to_ne_bytes());
            expected[16..20].copy_from_slice(&random.to_ne_bytes());
            // SAFETY: repr(C), the asserted offsets/size and initialized fields
            // cover every byte, including all five words of the data union.
            let bytes = unsafe { std::slice::from_raw_parts((&raw const reply).cast::<u8>(), 32) };
            assert_eq!(bytes, expected);
        }
    }
}
