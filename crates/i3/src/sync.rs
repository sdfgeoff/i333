/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * sync.c: i3 sync protocol: https://i3wm.org/docs/testsuite.html#i3_sync
 *
 */
// Mechanical file-by-file Rust translation of src/sync.c.
// Originally translated with C2Rust 0.22.1; subsequently maintained in Rust.

pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub unsafe fn sync_respond(window: xcb_window_t, rnd: u32) {
    debuglog(
        c"%s:%s:%d - [i3 sync protocol] Sending random value %d back to X11 window 0x%08x\n"
            .as_ptr()
            .cast_mut(),
        c"../src/sync.c".as_ptr(),
        c"sync_respond".as_ptr(),
        13 as ::core::ffi::c_int,
        rnd,
        window,
    );
    let reply: *mut ::core::ffi::c_void = scalloc(32 as usize, 1 as usize);
    let ev: *mut xcb_client_message_event_t = reply as *mut xcb_client_message_event_t;
    (*ev).response_type = XCB_CLIENT_MESSAGE as u8;
    (*ev).window = window;
    (*ev).type_0 = A_I3_SYNC;
    (*ev).format = 32 as u8;
    (*ev).data.data32[0 as ::core::ffi::c_int as usize] = window as u32;
    (*ev).data.data32[1 as ::core::ffi::c_int as usize] = rnd;
    xcb_send_event(
        conn,
        false_0 as u8,
        window,
        XCB_EVENT_MASK_NO_EVENT as ::core::ffi::c_int as u32,
        ev as *mut ::core::ffi::c_char,
    );
    xcb_flush(conn);
    free(reply);
}

// Internal helpers use Rust linkage rather than foreign symbol lookup.
use i3_common::safewrappers::scalloc;

use xcb_sys::{
    xcb_client_message_event_t, xcb_flush, xcb_send_event, xcb_window_t, XCB_CLIENT_MESSAGE,
    XCB_EVENT_MASK_NO_EVENT,
};

pub use libc::free;

use crate::log::debuglog;

// Internal calls and globals use canonical Rust identities.
use crate::application::conn;
use crate::application::A_I3_SYNC;
