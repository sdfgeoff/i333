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

use xcb::x;

use crate::application::last_timestamp;
use crate::bindings::{get_binding_from_event, run_binding};
use crate::commands_parser::command_result_free;

/// Dispatch a native key event against the current bindings.
///
/// # Safety
/// The borrowed maintained event stays live through this dispatch. The caller
/// must own exclusive event-thread access to the
/// initialized display, binding and container globals. Binding execution can
/// mutate that graph, so no references into it may survive this call.
pub unsafe fn handle_key_press(event: &x::KeyPressEvent) {
    // KeyReleaseEvent is a maintained alias of KeyPressEvent, including its
    // BaseEvent number. Read the actual release response (3) and preserve the
    // legacy unmasked comparison for synthetic SendEvent input.
    let (key_release, time, detail, state) = (
        event.response_type() == 3_u8,
        event.time(),
        event.detail(),
        xcb_interop::events::key_press_state(event),
    );
    // SAFETY: Exclusive access to the event-thread globals is part of the
    // contract. The event remains live through lookup; a returned binding is
    // used immediately and the owned command result is freed exactly once.
    unsafe {
        last_timestamp = time;
        ::log::debug!(
            "../src/key_press.c:handle_key_press:23 - {} {}, state raw = 0x{:x}\n",
            if key_release {
                "KeyRelease"
            } else {
                "KeyPress"
            },
            detail as i32,
            state as u32,
        );
        let binding = get_binding_from_event(event.response_type(), state, detail);
        if binding.is_null() {
            return;
        }
        let result = run_binding(binding, std::ptr::null_mut());
        command_result_free(result);
    }
}
