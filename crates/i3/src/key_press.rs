/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * key_press.c: key press handler
 *
 */
#![deny(unsafe_op_in_unsafe_fn)]

use crate::application::last_timestamp;
use crate::bindings::{get_binding_from_xcb_event, run_binding};
use crate::commands_parser::command_result_free;
use crate::log::debuglog;
use xcb_sys::{xcb_generic_event_t, xcb_key_press_event_t, XCB_KEY_RELEASE};

/// Dispatch a native key event against the current bindings.
///
/// # Safety
/// `event` must point to a live, aligned, initialized XCB key press/release event
/// for this call. The caller must own exclusive event-thread access to the
/// initialized display, binding and container globals. Binding execution can
/// mutate that graph, so no references into it may survive this call.
pub unsafe fn handle_key_press(event: *mut xcb_key_press_event_t) {
    // SAFETY: The caller supplies a valid native event. Copy only the fields we
    // need so the borrow ends before binding lookup or command execution.
    let (key_release, time, detail, state) = unsafe {
        let event = &*event;
        (
            event.response_type == XCB_KEY_RELEASE as u8,
            event.time,
            event.detail,
            event.state,
        )
    };
    // SAFETY: Exclusive access to the event-thread globals is part of the
    // contract. The event remains live through lookup; a returned binding is
    // used immediately and the owned command result is freed exactly once.
    unsafe {
        last_timestamp = time;
        debuglog(
            c"%s:%s:%d - %s %d, state raw = 0x%x\n".as_ptr().cast_mut(),
            c"../src/key_press.c".as_ptr(),
            c"handle_key_press".as_ptr(),
            23i32,
            if key_release {
                c"KeyRelease".as_ptr()
            } else {
                c"KeyPress".as_ptr()
            },
            i32::from(detail),
            i32::from(state),
        );
        let binding = get_binding_from_xcb_event(event.cast::<xcb_generic_event_t>());
        if binding.is_null() {
            return;
        }
        let result = run_binding(binding, std::ptr::null_mut());
        command_result_free(result);
    }
}
