use crate::types::DragResult;
use crate::xcursor::Cursor;
use xcb::{x, Xid, XidNew};
/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * drag.c: click and drag.
 *
 */
// Event/grab ownership is scoped to this synchronous nested loop. Con/event/
// extra pointers are borrowed by contract until drag_pointer returns; callbacks
// may mutate/free graph nodes, so no graph references cross their invocation.
pub use libev_sys::{ev_loop, ev_prepare, ev_prepare_start, ev_prepare_stop, ev_unloop, BreakMode};

pub type DragCallback = Option<
    unsafe fn(
        *mut Con,
        *mut Rect,
        u32,
        u32,
        &x::ButtonPressEvent,
        *const ::core::ffi::c_void,
    ) -> (),
>;

struct DragState<'event> {
    pub prepare: ev_prepare,
    pub result: DragResult,
    pub con: *mut Con,
    pub event: &'event x::ButtonPressEvent,
    pub old_rect: Rect,
    pub callback: DragCallback,
    pub threshold_exceeded: bool,
    pub xcursor: x::Cursor,
    pub extra: *const ::core::ffi::c_void,
}

fn threshold_exceeded(x1: u32, y1: u32, x2: u32, y2: u32, threshold: u32) -> bool {
    x1.wrapping_sub(x2)
        .wrapping_mul(x1.wrapping_sub(x2))
        .wrapping_add(y1.wrapping_sub(y2).wrapping_mul(y1.wrapping_sub(y2)))
        > threshold.wrapping_mul(threshold)
}
unsafe fn drain_drag_events(loop_0: *mut ev_loop, dragloop: *mut DragState<'_>) -> bool {
    let mut pending_motion: Option<(i16, i16)> = None;
    loop {
        let next = i3_common::display::connection()
            .expect("main X11 connection is registered")
            .poll_for_event();
        let owner = match next {
            Ok(Some(event)) => event,
            Err(xcb::Error::Protocol(error)) => {
                let error = &*error.as_raw();
                debuglog(
                    c"%s:%s:%d - X11 Error received (probably harmless)! sequence 0x%x, error_code = %d\n".as_ptr().cast_mut(),
                    c"../src/drag.c".as_ptr(), c"drain_drag_events".as_ptr(), 58,
                    i32::from(error.sequence), i32::from(error.error_code),
                );
                continue;
            }
            Ok(None) | Err(xcb::Error::Connection(_)) => break,
        };
        // Only this event owner spans dispatch. Handlers borrow the allocation
        // synchronously; no registry borrow or connection clone spans callbacks.
        match &owner {
            xcb::Event::X(x::Event::ButtonPress(press)) => {
                debuglog(
                    c"%s:%s:%d - Ignoring button %d press during drag initiated by button %d\n"
                        .as_ptr()
                        .cast_mut(),
                    c"../src/drag.c".as_ptr(),
                    c"drain_drag_events".as_ptr(),
                    70,
                    press.detail() as i32,
                    (*dragloop).event.detail() as i32,
                );
            }
            xcb::Event::X(x::Event::ButtonRelease(release)) => {
                if released_drag_button((*dragloop).event.detail(), release.detail()) {
                    debuglog(
                        c"%s:%s:%d - Button %d released, ending drag initiated by button %d\n"
                            .as_ptr()
                            .cast_mut(),
                        c"../src/drag.c".as_ptr(),
                        c"drain_drag_events".as_ptr(),
                        79,
                        release.detail() as i32,
                        (*dragloop).event.detail() as i32,
                    );
                    (*dragloop).result = DragResult::Success;
                } else {
                    debuglog(
                        c"%s:%s:%d - Ignoring button %d release during drag initiated by button %d\n".as_ptr().cast_mut(),
                        c"../src/drag.c".as_ptr(),
                        c"drain_drag_events".as_ptr(),
                        83,
                        release.detail() as i32,
                        (*dragloop).event.detail() as i32,
                    );
                }
            }
            xcb::Event::X(x::Event::KeyPress(_)) => {
                debuglog(
                    c"%s:%s:%d - A key was pressed during drag, reverting changes.\n"
                        .as_ptr()
                        .cast_mut(),
                    c"../src/drag.c".as_ptr(),
                    c"drain_drag_events".as_ptr(),
                    89,
                );
                (*dragloop).result = DragResult::Revert;
                handle_event(&owner);
            }
            xcb::Event::X(x::Event::UnmapNotify(unmap_event)) => {
                let con: *mut Con = con_by_window_id(unmap_event.window().resource_id());
                if !con.is_null() {
                    debuglog(
                        c"%s:%s:%d - UnmapNotify for window 0x%08x (container %p)\n"
                            .as_ptr()
                            .cast_mut(),
                        c"../src/drag.c".as_ptr(),
                        c"drain_drag_events".as_ptr(),
                        99,
                        unmap_event.window().resource_id(),
                        con,
                    );
                    if con_get_workspace(con) == con_get_workspace(focused) {
                        debuglog(
                            c"%s:%s:%d - UnmapNotify for a managed window on the current workspace, aborting\n".as_ptr().cast_mut(),
                            c"../src/drag.c".as_ptr(),
                            c"drain_drag_events".as_ptr(),
                            102,
                        );
                        (*dragloop).result = DragResult::Abort;
                    }
                }
                handle_event(&owner);
            }
            xcb::Event::X(x::Event::MotionNotify(motion)) => {
                pending_motion = Some((motion.root_x(), motion.root_y()));
            }
            xcb::Event::X(x::Event::ClientMessage(client_message)) => {
                let words = xcb_interop::events::client_message_words(client_message);
                if client_message.r#type().resource_id() == A__NET_WM_MOVERESIZE {
                    debuglog(
                        c"%s:%s:%d - Ignoring _NET_WM_MOVERESIZE request during active drag (window = 0x%08x, direction = %d)\n".as_ptr().cast_mut(),
                        c"../src/drag.c".as_ptr(),
                        c"drain_drag_events".as_ptr(),
                        122,
                        client_message.window().resource_id(),
                        words[2],
                    );
                } else {
                    handle_event(&owner);
                }
            }
            _ => {
                debuglog(
                    c"%s:%s:%d - Passing to original handler\n"
                        .as_ptr()
                        .cast_mut(),
                    c"../src/drag.c".as_ptr(),
                    c"drain_drag_events".as_ptr(),
                    131,
                );
                handle_event(&owner);
            }
        }
        if (*dragloop).result == DragResult::Dragging {
            continue;
        }
        ev_unloop(loop_0, BreakMode::One);
        if (*dragloop).result == DragResult::Success {
            break;
        }
        return true;
    }
    let Some((motion_x, motion_y)) = pending_motion else {
        return true;
    };
    if !(*dragloop).threshold_exceeded
        && threshold_exceeded(
            motion_x as u32,
            motion_y as u32,
            (*dragloop).event.root_x() as u32,
            (*dragloop).event.root_y() as u32,
            logical_px(15) as u32,
        )
    {
        if !(*dragloop).xcursor.is_none() {
            i3_common::display::connection()
                .expect("main X11 connection is registered")
                .send_request(&x::ChangeActivePointerGrab {
                    cursor: (*dragloop).xcursor,
                    time: x::CURRENT_TIME,
                    event_mask: x::EventMask::BUTTON_RELEASE | x::EventMask::POINTER_MOTION,
                });
        }
        (*dragloop).threshold_exceeded = true;
    }
    if (*dragloop).threshold_exceeded && ((*dragloop).con.is_null() || con_exists((*dragloop).con))
    {
        (*dragloop).callback.expect("non-null function pointer")(
            (*dragloop).con,
            &raw mut (*dragloop).old_rect,
            motion_x as u32,
            motion_y as u32,
            (*dragloop).event,
            (*dragloop).extra,
        );
    }
    let _ = i3_common::display::connection()
        .expect("main X11 connection is registered")
        .flush();
    (*dragloop).result != DragResult::Dragging
}
unsafe extern "C" fn xcb_drag_prepare_cb(loop_0: *mut ev_loop, w: *mut ev_prepare, revents: i32) {
    let dragloop: *mut DragState<'_> = (*w).data as *mut DragState<'_>;
    while !drain_drag_events(loop_0, dragloop) {}
}

pub unsafe fn drag_pointer(
    con: *mut Con,
    event: &x::ButtonPressEvent,
    confine_to: xcb_window_t,
    cursor: Cursor,
    use_threshold: bool,
    callback: DragCallback,
    extra: *const ::core::ffi::c_void,
) -> DragResult {
    let xcursor = if cursor != Cursor::Pointer {
        x::Cursor::new(xcursor_get_cursor(cursor))
    } else {
        x::CURSOR_NONE
    };
    let pointer_reply = {
        let connection =
            i3_common::display::connection().expect("main X11 connection is registered");
        let cookie = connection.send_request(&x::GrabPointer {
            owner_events: false,
            grab_window: x::Window::new(root),
            event_mask: x::EventMask::BUTTON_RELEASE | x::EventMask::POINTER_MOTION,
            pointer_mode: x::GrabMode::Async,
            keyboard_mode: x::GrabMode::Async,
            confine_to: x::Window::new(confine_to),
            cursor: if use_threshold {
                x::CURSOR_NONE
            } else {
                xcursor
            },
            time: x::CURRENT_TIME,
        });
        match connection.wait_for_reply(cookie) {
            Ok(reply) => reply,
            Err(error) => {
                errorlog(
                    c"ERROR: Could not grab pointer (error_code = %d)\n"
                        .as_ptr()
                        .cast_mut(),
                    i32::from(protocol_error_code(&error)),
                );
                return DragResult::Abort;
            }
        }
    };
    if let Err(status) = successful_grab(&pointer_reply) {
        errorlog(
            c"ERROR: Could not grab pointer (status = %d)\n"
                .as_ptr()
                .cast_mut(),
            i32::from(status),
        );
        return DragResult::Abort;
    }
    debuglog(
        c"%s:%s:%d - Pointer grab successful\n".as_ptr().cast_mut(),
        c"../src/drag.c".as_ptr(),
        c"drag_pointer".as_ptr(),
        238,
    );
    drop(pointer_reply);
    let mut grabs = InputGrabs { keyboard: false };
    let keyboard_reply = {
        let connection =
            i3_common::display::connection().expect("main X11 connection is registered");
        let cookie = connection.send_request(&x::GrabKeyboard {
            owner_events: false,
            grab_window: x::Window::new(root),
            time: x::CURRENT_TIME,
            pointer_mode: x::GrabMode::Async,
            keyboard_mode: x::GrabMode::Async,
        });
        match connection.wait_for_reply(cookie) {
            Ok(reply) => reply,
            Err(error) => {
                errorlog(
                    c"ERROR: Could not grab keyboard (error_code = %d)\n"
                        .as_ptr()
                        .cast_mut(),
                    i32::from(protocol_error_code(&error)),
                );
                return DragResult::Abort;
            }
        }
    };
    if let Err(status) = successful_grab(&keyboard_reply) {
        errorlog(
            c"ERROR: Could not grab keyboard (status = %d)\n"
                .as_ptr()
                .cast_mut(),
            i32::from(status),
        );
        return DragResult::Abort;
    }
    debuglog(
        c"%s:%s:%d - Keyboard grab successful\n".as_ptr().cast_mut(),
        c"../src/drag.c".as_ptr(),
        c"drag_pointer".as_ptr(),
        266,
    );
    drop(keyboard_reply);
    grabs.keyboard = true;
    let mut loop_0: DragState<'_> = DragState {
        prepare: ev_prepare::new(xcb_drag_prepare_cb),
        result: DragResult::Dragging,
        con,
        event,
        old_rect: Rect {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        callback,
        threshold_exceeded: !use_threshold,
        xcursor,
        extra,
    };
    let prepare: *mut ev_prepare = &raw mut loop_0.prepare;
    if !con.is_null() {
        loop_0.old_rect = (*con).rect;
    }
    (*prepare).data = &raw mut loop_0 as *mut ::core::ffi::c_void;
    main_set_x11_cb(false);
    ev_prepare_start(main_loop, prepare);
    debuglog(
        c"%s:%s:%d - Entering drag loop (con = %p, button = %d)\n"
            .as_ptr()
            .cast_mut(),
        c"../src/drag.c".as_ptr(),
        c"drag_pointer".as_ptr(),
        288,
        con,
        event.detail() as i32,
    );
    ev_loop(main_loop, 0);
    debuglog(
        c"%s:%s:%d - Leaving drag loop (con = %p, button = %d, result = %d)\n"
            .as_ptr()
            .cast_mut(),
        c"../src/drag.c".as_ptr(),
        c"drag_pointer".as_ptr(),
        291,
        con,
        event.detail() as i32,
        loop_0.result as u32,
    );
    ev_prepare_stop(main_loop, prepare);
    main_set_x11_cb(true);
    drop(grabs);
    let _ = i3_common::display::connection()
        .expect("main X11 connection is registered")
        .flush();
    loop_0.result
}

// Internal helpers use Rust linkage rather than foreign symbol lookup.
use crate::application::main_set_x11_cb;
use crate::xcursor::xcursor_get_cursor;
use i3_common::dpi::logical_px;

use xcb_sys::{xcb_window_t, XCB_BUTTON_INDEX_ANY};

use crate::log::{debuglog, errorlog};

// Shared model types retain field layout while using one Rust identity.
pub use crate::types::Con;
pub use crate::types::Rect;

// Internal calls and globals use canonical Rust identities.
use crate::application::main_loop;
use crate::application::root;
use crate::application::A__NET_WM_MOVERESIZE;
use crate::con::con_by_window_id;
use crate::con::con_exists;
use crate::con::con_get_workspace;
use crate::handlers::handle_event;
use crate::tree::focused;

/// The maintained reply's generated status getter transmutes the raw value.
/// Check the wire byte first so unknown native statuses stay ordinary failures.
fn checked_grab_status(raw: u8) -> Result<x::GrabStatus, u8> {
    match raw {
        0 => Ok(x::GrabStatus::Success),
        1 => Ok(x::GrabStatus::AlreadyGrabbed),
        2 => Ok(x::GrabStatus::InvalidTime),
        3 => Ok(x::GrabStatus::NotViewable),
        4 => Ok(x::GrabStatus::Frozen),
        other => Err(other),
    }
}
fn successful_grab(reply: &impl xcb::Reply) -> Result<(), u8> {
    // SAFETY: every owned grab reply contains the fixed 32-byte native header;
    // status is its second byte and does not depend on a payload length.
    let raw = unsafe { *reply.as_raw().add(1) };
    match checked_grab_status(raw) {
        Ok(x::GrabStatus::Success) => Ok(()),
        _ => Err(raw),
    }
}
fn protocol_error_code(error: &xcb::Error) -> u8 {
    match error {
        // SAFETY: the maintained owner contains a complete native error header.
        xcb::Error::Protocol(error) => unsafe { (*error.as_raw()).error_code },
        xcb::Error::Connection(_) => 0,
    }
}

struct InputGrabs {
    keyboard: bool,
}
impl Drop for InputGrabs {
    fn drop(&mut self) {
        if let Some(connection) = i3_common::display::connection() {
            if self.keyboard {
                connection.send_request(&x::UngrabKeyboard {
                    time: x::CURRENT_TIME,
                });
            }
            connection.send_request(&x::UngrabPointer {
                time: x::CURRENT_TIME,
            });
        }
    }
}
fn released_drag_button(initiator: u8, released: u8) -> bool {
    initiator == XCB_BUTTON_INDEX_ANY as u8 || initiator == released
}
#[cfg(test)]
mod owned_tests {
    use super::*;
    #[test]
    fn native_grab_statuses_are_checked_before_enum_conversion() {
        for raw in u8::MIN..=u8::MAX {
            if raw <= 4 {
                assert_eq!(checked_grab_status(raw).unwrap() as u8, raw);
            } else {
                assert_eq!(checked_grab_status(raw), Err(raw));
            }
            // Both generated replies expose a transmuting enum getter. Exercise
            // their actual owned allocations through our checked ingress path.
            unsafe {
                let pointer = libc::calloc(1, 32).cast::<u8>();
                let keyboard = libc::calloc(1, 32).cast::<u8>();
                assert!(!pointer.is_null() && !keyboard.is_null());
                pointer.write(1);
                pointer.add(1).write(raw);
                keyboard.write(1);
                keyboard.add(1).write(raw);
                let pointer: x::GrabPointerReply = xcb::Reply::from_raw(pointer);
                let keyboard: x::GrabKeyboardReply = xcb::Reply::from_raw(keyboard);
                let expected = if raw == 0 { Ok(()) } else { Err(raw) };
                assert_eq!(successful_grab(&pointer), expected);
                assert_eq!(successful_grab(&keyboard), expected);
            }
        }
    }
    #[test]
    fn threshold_and_button_policy_keep_strict_distance_wrapping_and_any_button() {
        assert!(!threshold_exceeded(15, 0, 0, 0, 15));
        assert!(threshold_exceeded(16, 0, 0, 0, 15));
        assert!(!threshold_exceeded(u32::MAX, 0, 0, 0, 15));
        assert!(!threshold_exceeded(65536, 0, 0, 0, 15)); // u32 squared wraps exactly.
        assert!(released_drag_button(0, 5));
        assert!(released_drag_button(3, 3));
        assert!(!released_drag_button(3, 1));
    }
}
