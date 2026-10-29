/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 */
// Mechanical file-by-file Rust translation of libi3/fake_configure_notify.c.
// Originally translated with C2Rust 0.22.1; subsequently maintained in Rust.

pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub unsafe fn fake_configure_notify(
    conn: *mut xcb_connection_t,
    r: xcb_rectangle_t,
    window: xcb_window_t,
    border_width: ::core::ffi::c_int,
) {
    let event: *mut ::core::ffi::c_void = scalloc(32 as usize, 1 as usize);
    let generated_event: *mut xcb_configure_notify_event_t =
        event as *mut xcb_configure_notify_event_t;
    (*generated_event).event = window;
    (*generated_event).window = window;
    (*generated_event).response_type = XCB_CONFIGURE_NOTIFY as u8;
    (*generated_event).x = r.x;
    (*generated_event).y = r.y;
    (*generated_event).width = r.width;
    (*generated_event).height = r.height;
    (*generated_event).border_width = border_width as u16;
    (*generated_event).above_sibling = XCB_NONE as xcb_window_t;
    (*generated_event).override_redirect = false_0 as u8;
    xcb_send_event(
        conn,
        false_0 as u8,
        window,
        XCB_EVENT_MASK_STRUCTURE_NOTIFY as ::core::ffi::c_int as u32,
        generated_event as *mut ::core::ffi::c_char,
    );
    xcb_flush(conn);
    free(event);
}

// Internal helpers use Rust linkage rather than foreign symbol lookup.
use crate::safewrappers::scalloc;

use xcb_sys::{
    xcb_configure_notify_event_t, xcb_connection_t, xcb_flush, xcb_rectangle_t, xcb_send_event,
    xcb_window_t, XCB_CONFIGURE_NOTIFY, XCB_EVENT_MASK_STRUCTURE_NOTIFY, XCB_NONE,
};

pub use libc::free;
