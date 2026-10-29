use crate::types::DragResult;
use crate::xcursor::Cursor;
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
pub use libev_sys::{
    ev_break, ev_loop, ev_prepare, ev_prepare_start, ev_prepare_stop, EVBREAK_ONE,
};

pub type DragCallback = Option<
    unsafe fn(
        *mut Con,
        *mut Rect,
        u32,
        u32,
        *const xcb_button_press_event_t,
        *const ::core::ffi::c_void,
    ) -> (),
>;

#[derive(Copy, Clone)]
#[repr(C)]
struct DragState {
    pub prepare: ev_prepare,
    pub result: DragResult,
    pub con: *mut Con,
    pub event: *const xcb_button_press_event_t,
    pub old_rect: Rect,
    pub callback: DragCallback,
    pub threshold_exceeded: bool,
    pub xcursor: xcb_cursor_t,
    pub extra: *const ::core::ffi::c_void,
}

fn threshold_exceeded(x1: u32, y1: u32, x2: u32, y2: u32, threshold: u32) -> bool {
    x1.wrapping_sub(x2)
        .wrapping_mul(x1.wrapping_sub(x2))
        .wrapping_add(y1.wrapping_sub(y2).wrapping_mul(y1.wrapping_sub(y2)))
        > threshold.wrapping_mul(threshold)
}
unsafe fn drain_drag_events(loop_0: *mut ev_loop, dragloop: *mut DragState) -> bool {
    let mut pending_motion: Option<xcb_motion_notify_event_t> = None;
    let mut event: *mut xcb_generic_event_t = ::core::ptr::null_mut::<xcb_generic_event_t>();
    loop {
        event = xcb_poll_for_event(conn);
        if event.is_null() {
            break;
        }
        let _event_owner = Native::take(event);
        if (*event).response_type as i32 == 0 {
            let error: *mut xcb_generic_error_t = event as *mut xcb_generic_error_t;
            debuglog(
                c"%s:%s:%d - X11 Error received (probably harmless)! sequence 0x%x, error_code = %d\n".as_ptr().cast_mut(),
                c"../src/drag.c".as_ptr(),
                c"drain_drag_events".as_ptr(),
                58,
                (*error).sequence as i32,
                error.as_ref().map_or(0,|error|i32::from(error.error_code)),
            );
        } else {
            let type_0: i32 = (*event).response_type as i32 & 0x7f as i32;
            match type_0 {
                XCB_BUTTON_PRESS => {
                    let press: *const xcb_button_press_event_t =
                        event as *mut xcb_button_press_event_t;
                    debuglog(
                        c"%s:%s:%d - Ignoring button %d press during drag initiated by button %d\n"
                            .as_ptr()
                            .cast_mut(),
                        c"../src/drag.c".as_ptr(),
                        c"drain_drag_events".as_ptr(),
                        70,
                        (*press).detail as i32,
                        (*(*dragloop).event).detail as i32,
                    );
                }
                XCB_BUTTON_RELEASE => {
                    let release: *const xcb_button_release_event_t =
                        event as *mut xcb_button_release_event_t;
                    if released_drag_button((*(*dragloop).event).detail, (*release).detail) {
                        debuglog(
                            c"%s:%s:%d - Button %d released, ending drag initiated by button %d\n"
                                .as_ptr()
                                .cast_mut(),
                            c"../src/drag.c".as_ptr(),
                            c"drain_drag_events".as_ptr(),
                            79,
                            (*release).detail as i32,
                            (*(*dragloop).event).detail as i32,
                        );
                        (*dragloop).result = DragResult::Success;
                    } else {
                        debuglog(
                            c"%s:%s:%d - Ignoring button %d release during drag initiated by button %d\n".as_ptr().cast_mut(),
                            c"../src/drag.c".as_ptr(),
                            c"drain_drag_events".as_ptr(),
                            83,
                            (*release).detail as i32,
                            (*(*dragloop).event).detail as i32,
                        );
                    }
                }
                XCB_KEY_PRESS => {
                    debuglog(
                        c"%s:%s:%d - A key was pressed during drag, reverting changes.\n"
                            .as_ptr()
                            .cast_mut(),
                        c"../src/drag.c".as_ptr(),
                        c"drain_drag_events".as_ptr(),
                        89,
                    );
                    (*dragloop).result = DragResult::Revert;
                    handle_event(type_0, event);
                }
                XCB_UNMAP_NOTIFY => {
                    let unmap_event: *mut xcb_unmap_notify_event_t =
                        event as *mut xcb_unmap_notify_event_t;
                    let con: *mut Con = con_by_window_id((*unmap_event).window);
                    if !con.is_null() {
                        debuglog(
                            c"%s:%s:%d - UnmapNotify for window 0x%08x (container %p)\n"
                                .as_ptr()
                                .cast_mut(),
                            c"../src/drag.c".as_ptr(),
                            c"drain_drag_events".as_ptr(),
                            99,
                            (*unmap_event).window,
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
                    handle_event(type_0, event);
                }
                XCB_MOTION_NOTIFY => {
                    pending_motion = Some(*(event as *const xcb_motion_notify_event_t));
                }
                XCB_CLIENT_MESSAGE => {
                    let client_message: *const xcb_client_message_event_t =
                        event as *mut xcb_client_message_event_t;
                    if (*client_message).type_0 == A__NET_WM_MOVERESIZE {
                        debuglog(
                            c"%s:%s:%d - Ignoring _NET_WM_MOVERESIZE request during active drag (window = 0x%08x, direction = %d)\n".as_ptr().cast_mut(),
                            c"../src/drag.c".as_ptr(),
                            c"drain_drag_events".as_ptr(),
                            122,
                            (*client_message).window,
                            (*client_message)
                                .data
                                .data32[2 as usize],
                        );
                    } else {
                        handle_event(type_0, event);
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
                    handle_event(type_0, event);
                }
            }
            if !((*dragloop).result != DragResult::Dragging) {
                continue;
            }
            ev_break(loop_0, EVBREAK_ONE as i32);
            if (*dragloop).result == DragResult::Success {
                break;
            }
            return true;
        }
    }
    let Some(last_motion_notify) = pending_motion else {
        return true;
    };
    if !(*dragloop).threshold_exceeded
        && threshold_exceeded(
            last_motion_notify.root_x as u32,
            last_motion_notify.root_y as u32,
            (*(*dragloop).event).root_x as u32,
            (*(*dragloop).event).root_y as u32,
            logical_px(15) as u32,
        ) as i32
            != 0
    {
        if (*dragloop).xcursor as ::core::ffi::c_long != XCB_NONE {
            xcb_change_active_pointer_grab(
                conn,
                (*dragloop).xcursor,
                XCB_CURRENT_TIME as xcb_timestamp_t,
                (XCB_EVENT_MASK_BUTTON_RELEASE as i32 | XCB_EVENT_MASK_POINTER_MOTION as i32)
                    as u16,
            );
        }
        (*dragloop).threshold_exceeded = true;
    }
    if (*dragloop).threshold_exceeded as i32 != 0
        && ((*dragloop).con.is_null() || con_exists((*dragloop).con) as i32 != 0)
    {
        (*dragloop).callback.expect("non-null function pointer")(
            (*dragloop).con,
            &raw mut (*dragloop).old_rect,
            last_motion_notify.root_x as u32,
            last_motion_notify.root_y as u32,
            (*dragloop).event,
            (*dragloop).extra,
        );
    }
    xcb_flush(conn);
    (*dragloop).result != DragResult::Dragging
}
unsafe extern "C" fn xcb_drag_prepare_cb(loop_0: *mut ev_loop, w: *mut ev_prepare, revents: i32) {
    let dragloop: *mut DragState = (*w).data as *mut DragState;
    while !drain_drag_events(loop_0, dragloop) {}
}

pub unsafe fn drag_pointer(
    con: *mut Con,
    event: *const xcb_button_press_event_t,
    confine_to: xcb_window_t,
    cursor: Cursor,
    use_threshold: bool,
    callback: DragCallback,
    extra: *const ::core::ffi::c_void,
) -> DragResult {
    let xcursor: xcb_cursor_t = (if cursor != Cursor::Pointer {
        xcursor_get_cursor(cursor) as ::core::ffi::c_long
    } else {
        XCB_NONE
    }) as xcb_cursor_t;
    let mut reply: *mut xcb_grab_pointer_reply_t =
        ::core::ptr::null_mut::<xcb_grab_pointer_reply_t>();
    let mut error: *mut xcb_generic_error_t = ::core::ptr::null_mut::<xcb_generic_error_t>();
    let cookie: xcb_grab_pointer_cookie_t = xcb_grab_pointer(
        conn,
        0u8,
        root,
        (XCB_EVENT_MASK_BUTTON_RELEASE as i32 | XCB_EVENT_MASK_POINTER_MOTION as i32) as u16,
        XCB_GRAB_MODE_ASYNC as i32 as u8,
        XCB_GRAB_MODE_ASYNC as i32 as u8,
        confine_to,
        (if use_threshold {
            XCB_NONE
        } else {
            xcursor as ::core::ffi::c_long
        }) as xcb_cursor_t,
        XCB_CURRENT_TIME as xcb_timestamp_t,
    );
    reply = xcb_grab_pointer_reply(conn, cookie, &raw mut error);
    let pointer_reply = Native::take(reply);
    let pointer_error = Native::take(error);
    if reply.is_null() {
        errorlog(
            c"ERROR: Could not grab pointer (error_code = %d)\n"
                .as_ptr()
                .cast_mut(),
            error
                .as_ref()
                .map_or(0, |error| i32::from(error.error_code)),
        );

        return DragResult::Abort;
    }
    if (*reply).status as i32 != XCB_GRAB_STATUS_SUCCESS as i32 {
        errorlog(
            c"ERROR: Could not grab pointer (status = %d)\n"
                .as_ptr()
                .cast_mut(),
            (*reply).status as i32,
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
    drop(pointer_error);
    let mut grabs = InputGrabs {
        connection: conn,
        keyboard: false,
    };
    let mut keyb_reply: *mut xcb_grab_keyboard_reply_t =
        ::core::ptr::null_mut::<xcb_grab_keyboard_reply_t>();
    let keyb_cookie: xcb_grab_keyboard_cookie_t = xcb_grab_keyboard(
        conn,
        0u8,
        root,
        XCB_CURRENT_TIME as xcb_timestamp_t,
        XCB_GRAB_MODE_ASYNC as i32 as u8,
        XCB_GRAB_MODE_ASYNC as i32 as u8,
    );
    error = std::ptr::null_mut();
    keyb_reply = xcb_grab_keyboard_reply(conn, keyb_cookie, &raw mut error);
    let keyboard_reply = Native::take(keyb_reply);
    let keyboard_error = Native::take(error);
    if keyb_reply.is_null() {
        errorlog(
            c"ERROR: Could not grab keyboard (error_code = %d)\n"
                .as_ptr()
                .cast_mut(),
            error
                .as_ref()
                .map_or(0, |error| i32::from(error.error_code)),
        );

        return DragResult::Abort;
    }
    if (*keyb_reply).status as i32 != XCB_GRAB_STATUS_SUCCESS as i32 {
        errorlog(
            c"ERROR: Could not grab keyboard (status = %d)\n"
                .as_ptr()
                .cast_mut(),
            (*keyb_reply).status as i32,
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
    drop(keyboard_error);
    grabs.keyboard = true;
    let mut loop_0: DragState = DragState {
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
        (*event).detail as i32,
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
        (*event).detail as i32,
        loop_0.result as u32,
    );
    ev_prepare_stop(main_loop, prepare);
    main_set_x11_cb(true);
    drop(grabs);
    xcb_flush(conn);
    loop_0.result
}

// Internal helpers use Rust linkage rather than foreign symbol lookup.
use crate::application::main_set_x11_cb;
use crate::xcursor::xcursor_get_cursor;
use i3_common::dpi::logical_px;

use xcb_sys::{
    xcb_button_press_event_t, xcb_button_release_event_t, xcb_change_active_pointer_grab,
    xcb_client_message_event_t, xcb_connection_t, xcb_cursor_t, xcb_flush, xcb_generic_error_t,
    xcb_generic_event_t, xcb_grab_keyboard, xcb_grab_keyboard_cookie_t, xcb_grab_keyboard_reply,
    xcb_grab_keyboard_reply_t, xcb_grab_pointer, xcb_grab_pointer_cookie_t, xcb_grab_pointer_reply,
    xcb_grab_pointer_reply_t, xcb_motion_notify_event_t, xcb_poll_for_event, xcb_timestamp_t,
    xcb_ungrab_keyboard, xcb_ungrab_pointer, xcb_unmap_notify_event_t, xcb_window_t,
    XCB_BUTTON_INDEX_ANY, XCB_BUTTON_PRESS, XCB_BUTTON_RELEASE, XCB_CLIENT_MESSAGE,
    XCB_CURRENT_TIME, XCB_EVENT_MASK_BUTTON_RELEASE, XCB_EVENT_MASK_POINTER_MOTION,
    XCB_GRAB_MODE_ASYNC, XCB_GRAB_STATUS_SUCCESS, XCB_KEY_PRESS, XCB_MOTION_NOTIFY, XCB_NONE,
    XCB_UNMAP_NOTIFY,
};

use libc::free;

use crate::log::{debuglog, errorlog};

// Shared model types retain field layout while using one Rust identity.
pub use crate::types::Con;
pub use crate::types::Rect;

// Internal calls and globals use canonical Rust identities.
use crate::application::conn;
use crate::application::main_loop;
use crate::application::root;
use crate::application::A__NET_WM_MOVERESIZE;
use crate::con::con_by_window_id;
use crate::con::con_exists;
use crate::con::con_get_workspace;
use crate::handlers::handle_event;
use crate::tree::focused;

struct Native<T>(std::ptr::NonNull<T>);
impl<T> Native<T> {
    /// Takes exclusive ownership of a malloc allocation or null from XCB.
    unsafe fn take(pointer: *mut T) -> Option<Self> {
        std::ptr::NonNull::new(pointer).map(Self)
    }
}
impl<T> Drop for Native<T> {
    fn drop(&mut self) {
        unsafe { free(self.0.as_ptr().cast()) }
    }
}
struct InputGrabs {
    connection: *mut xcb_connection_t,
    keyboard: bool,
}
impl Drop for InputGrabs {
    fn drop(&mut self) {
        unsafe {
            if self.keyboard {
                xcb_ungrab_keyboard(self.connection, XCB_CURRENT_TIME as u32);
            }
            xcb_ungrab_pointer(self.connection, XCB_CURRENT_TIME as u32);
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
