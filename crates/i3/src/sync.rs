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

use crate::application::A_I3_SYNC;
use crate::log::debuglog;
use ::xcb::{x, XidNew};
use i3_common::display::connection;

fn sync_reply(window: u32, random: u32, atom: u32) -> x::ClientMessageEvent {
    x::ClientMessageEvent::new(
        x::Window::new(window),
        x::Atom::new(atom),
        x::ClientMessageData::Data32([window, random, 0, 0, 0]),
    )
}

/// Reply using the initialized display connection and i3 sync atom.
///
/// # Safety
/// The caller must run on the display event thread with a live `conn` and
/// initialized `A_I3_SYNC`, without concurrent mutation of those globals.
pub unsafe fn sync_respond(window: u32, rnd: u32) {
    // SAFETY: The caller supplies exclusive display-thread access; the native
    // logging arguments match the format and SendEvent copies its borrowed event
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
        let Some(connection) = connection() else {
            return;
        };
        connection.send_request(&x::SendEvent {
            propagate: false,
            destination: x::SendEventDest::Window(x::Window::new(window)),
            event_mask: x::EventMask::NO_EVENT,
            event: &reply,
        });
        let _ = connection.flush();
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
            expected[0] = 33;
            expected[1] = 32;
            expected[4..8].copy_from_slice(&window.to_ne_bytes());
            expected[8..12].copy_from_slice(&atom.to_ne_bytes());
            expected[12..16].copy_from_slice(&window.to_ne_bytes());
            expected[16..20].copy_from_slice(&random.to_ne_bytes());
            // SAFETY: the maintained event owns its initialized 32-byte wire
            // representation. The slice borrows that allocation for comparison.
            let bytes =
                unsafe { std::slice::from_raw_parts(::xcb::Raw::as_raw(&reply).cast::<u8>(), 32) };
            assert_eq!(bytes, expected);
        }
    }
}
