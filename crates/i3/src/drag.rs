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
// Mechanical file-by-file Rust translation of src/drag.c.
// Originally translated with C2Rust 0.22.1; subsequently maintained in Rust.
pub use libev_sys::{
    ev_break, ev_loop, ev_prepare, ev_prepare_start, ev_prepare_stop, ev_watcher, EVBREAK_ONE,
};

pub type i3String = _i3String;

pub type fullscreen_mode_t = ::core::ffi::c_uint;
pub const CF_GLOBAL: fullscreen_mode_t = 2;
pub const CF_OUTPUT: fullscreen_mode_t = 1;
pub const CF_NONE: fullscreen_mode_t = 0;

pub const M_BELOW: ::core::ffi::c_uint = 2;
pub const M_ASSIGN_WS: ::core::ffi::c_uint = 1;
pub const M_HERE: ::core::ffi::c_uint = 0;

pub const M_DOCK_BOTTOM: ::core::ffi::c_int = 3;
pub const M_DOCK_TOP: ::core::ffi::c_int = 2;
pub const M_DOCK_ANY: ::core::ffi::c_int = 1;
pub const M_NODOCK: ::core::ffi::c_int = 0;
pub const M_DONTCHECK: ::core::ffi::c_int = -1;

pub const A_TO_OUTPUT: ::core::ffi::c_uint = 16;
pub const A_TO_WORKSPACE_NUMBER: ::core::ffi::c_uint = 8;
pub const A_NO_FOCUS: ::core::ffi::c_uint = 4;
pub const A_TO_WORKSPACE: ::core::ffi::c_uint = 2;
pub const A_COMMAND: ::core::ffi::c_uint = 1;
pub const A_ANY: ::core::ffi::c_uint = 0;

pub const CT_DOCKAREA: ::core::ffi::c_uint = 5;
pub const CT_WORKSPACE: ::core::ffi::c_uint = 4;
pub const CT_FLOATING_CON: ::core::ffi::c_uint = 3;
pub const CT_CON: ::core::ffi::c_uint = 2;
pub const CT_OUTPUT: ::core::ffi::c_uint = 1;
pub const CT_ROOT: ::core::ffi::c_uint = 0;

pub type callback_t = Option<
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
pub struct drag_x11_cb {
    pub prepare: ev_prepare,
    pub result: DragResult,
    pub con: *mut Con,
    pub event: *const xcb_button_press_event_t,
    pub old_rect: Rect,
    pub callback: callback_t,
    pub threshold_exceeded: bool,
    pub xcursor: xcb_cursor_t,
    pub extra: *const ::core::ffi::c_void,
}
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();

unsafe fn threshold_exceeded(x1: u32, y1: u32, x2: u32, y2: u32) -> bool {
    let threshold: u32 = logical_px(15 as ::core::ffi::c_int) as u32;
    x1.wrapping_sub(x2)
        .wrapping_mul(x1.wrapping_sub(x2))
        .wrapping_add(y1.wrapping_sub(y2).wrapping_mul(y1.wrapping_sub(y2)))
        > threshold.wrapping_mul(threshold)
}
unsafe fn drain_drag_events(loop_0: *mut ev_loop, dragloop: *mut drag_x11_cb) -> bool {
    let mut last_motion_notify: *mut xcb_motion_notify_event_t =
        ::core::ptr::null_mut::<xcb_motion_notify_event_t>();
    let mut event: *mut xcb_generic_event_t = ::core::ptr::null_mut::<xcb_generic_event_t>();
    loop {
        event = xcb_poll_for_event(conn);
        if event.is_null() {
            break;
        }
        if (*event).response_type as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            let error: *mut xcb_generic_error_t = event as *mut xcb_generic_error_t;
            debuglog(
                c"%s:%s:%d - X11 Error received (probably harmless)! sequence 0x%x, error_code = %d\n".as_ptr().cast_mut(),
                c"../src/drag.c".as_ptr(),
                c"drain_drag_events".as_ptr(),
                58 as ::core::ffi::c_int,
                (*error).sequence as ::core::ffi::c_int,
                (*error).error_code as ::core::ffi::c_int,
            );
            free(event as *mut ::core::ffi::c_void);
        } else {
            let type_0: ::core::ffi::c_int =
                (*event).response_type as ::core::ffi::c_int & 0x7f as ::core::ffi::c_int;
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
                        70 as ::core::ffi::c_int,
                        (*press).detail as ::core::ffi::c_int,
                        (*(*dragloop).event).detail as ::core::ffi::c_int,
                    );
                }
                XCB_BUTTON_RELEASE => {
                    let release: *const xcb_button_release_event_t =
                        event as *mut xcb_button_release_event_t;
                    if (*(*dragloop).event).detail as ::core::ffi::c_int
                        == XCB_BUTTON_INDEX_ANY as ::core::ffi::c_int
                        || (*release).detail as ::core::ffi::c_int
                            == (*(*dragloop).event).detail as ::core::ffi::c_int
                    {
                        debuglog(
                            c"%s:%s:%d - Button %d released, ending drag initiated by button %d\n"
                                .as_ptr()
                                .cast_mut(),
                            c"../src/drag.c".as_ptr(),
                            c"drain_drag_events".as_ptr(),
                            79 as ::core::ffi::c_int,
                            (*release).detail as ::core::ffi::c_int,
                            (*(*dragloop).event).detail as ::core::ffi::c_int,
                        );
                        (*dragloop).result = DragResult::Success;
                    } else {
                        debuglog(
                            c"%s:%s:%d - Ignoring button %d release during drag initiated by button %d\n".as_ptr().cast_mut(),
                            c"../src/drag.c".as_ptr(),
                            c"drain_drag_events".as_ptr(),
                            83 as ::core::ffi::c_int,
                            (*release).detail as ::core::ffi::c_int,
                            (*(*dragloop).event).detail as ::core::ffi::c_int,
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
                        89 as ::core::ffi::c_int,
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
                            99 as ::core::ffi::c_int,
                            (*unmap_event).window,
                            con,
                        );
                        if con_get_workspace(con) == con_get_workspace(focused) {
                            debuglog(
                                c"%s:%s:%d - UnmapNotify for a managed window on the current workspace, aborting\n".as_ptr().cast_mut(),
                                c"../src/drag.c".as_ptr(),
                                c"drain_drag_events".as_ptr(),
                                102 as ::core::ffi::c_int,
                            );
                            (*dragloop).result = DragResult::Abort;
                        }
                    }
                    handle_event(type_0, event);
                }
                XCB_MOTION_NOTIFY => {
                    free(last_motion_notify as *mut ::core::ffi::c_void);
                    last_motion_notify = ::core::ptr::null_mut::<xcb_motion_notify_event_t>();
                    last_motion_notify = event as *mut xcb_motion_notify_event_t;
                }
                XCB_CLIENT_MESSAGE => {
                    let client_message: *const xcb_client_message_event_t =
                        event as *mut xcb_client_message_event_t;
                    if (*client_message).type_0 == A__NET_WM_MOVERESIZE {
                        debuglog(
                            c"%s:%s:%d - Ignoring _NET_WM_MOVERESIZE request during active drag (window = 0x%08x, direction = %d)\n".as_ptr().cast_mut(),
                            c"../src/drag.c".as_ptr(),
                            c"drain_drag_events".as_ptr(),
                            122 as ::core::ffi::c_int,
                            (*client_message).window,
                            (*client_message)
                                .data
                                .data32[2 as ::core::ffi::c_int as usize],
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
                        131 as ::core::ffi::c_int,
                    );
                    handle_event(type_0, event);
                }
            }
            if last_motion_notify != event as *mut xcb_motion_notify_event_t {
                free(event as *mut ::core::ffi::c_void);
            }
            if !((*dragloop).result != DragResult::Dragging) {
                continue;
            }
            ev_break(loop_0, EVBREAK_ONE as ::core::ffi::c_int);
            if (*dragloop).result == DragResult::Success {
                break;
            }
            free(last_motion_notify as *mut ::core::ffi::c_void);
            return true_0 != 0;
        }
    }
    if last_motion_notify.is_null() {
        return true_0 != 0;
    }
    if !(*dragloop).threshold_exceeded
        && threshold_exceeded(
            (*last_motion_notify).root_x as u32,
            (*last_motion_notify).root_y as u32,
            (*(*dragloop).event).root_x as u32,
            (*(*dragloop).event).root_y as u32,
        ) as ::core::ffi::c_int
            != 0
    {
        if (*dragloop).xcursor as ::core::ffi::c_long != XCB_NONE {
            xcb_change_active_pointer_grab(
                conn,
                (*dragloop).xcursor,
                XCB_CURRENT_TIME as xcb_timestamp_t,
                (XCB_EVENT_MASK_BUTTON_RELEASE as ::core::ffi::c_int
                    | XCB_EVENT_MASK_POINTER_MOTION as ::core::ffi::c_int) as u16,
            );
        }
        (*dragloop).threshold_exceeded = true_0 != 0;
    }
    if (*dragloop).threshold_exceeded as ::core::ffi::c_int != 0
        && ((*dragloop).con.is_null() || con_exists((*dragloop).con) as ::core::ffi::c_int != 0)
    {
        (*dragloop).callback.expect("non-null function pointer")(
            (*dragloop).con,
            &raw mut (*dragloop).old_rect,
            (*last_motion_notify).root_x as u32,
            (*last_motion_notify).root_y as u32,
            (*dragloop).event,
            (*dragloop).extra,
        );
    }
    free(last_motion_notify as *mut ::core::ffi::c_void);
    last_motion_notify = ::core::ptr::null_mut::<xcb_motion_notify_event_t>();
    xcb_flush(conn);
    (*dragloop).result != DragResult::Dragging
}
unsafe extern "C" fn xcb_drag_prepare_cb(
    loop_0: *mut ev_loop,
    w: *mut ev_prepare,
    revents: ::core::ffi::c_int,
) {
    let dragloop: *mut drag_x11_cb = (*w).data as *mut drag_x11_cb;
    while !drain_drag_events(loop_0, dragloop) {}
}

pub unsafe fn drag_pointer(
    con: *mut Con,
    event: *const xcb_button_press_event_t,
    confine_to: xcb_window_t,
    cursor: Cursor,
    use_threshold: bool,
    callback: callback_t,
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
        false_0 as u8,
        root,
        (XCB_EVENT_MASK_BUTTON_RELEASE as ::core::ffi::c_int
            | XCB_EVENT_MASK_POINTER_MOTION as ::core::ffi::c_int) as u16,
        XCB_GRAB_MODE_ASYNC as ::core::ffi::c_int as u8,
        XCB_GRAB_MODE_ASYNC as ::core::ffi::c_int as u8,
        confine_to,
        (if use_threshold as ::core::ffi::c_int != 0 {
            XCB_NONE
        } else {
            xcursor as ::core::ffi::c_long
        }) as xcb_cursor_t,
        XCB_CURRENT_TIME as xcb_timestamp_t,
    );
    reply = xcb_grab_pointer_reply(conn, cookie, &raw mut error);
    if reply.is_null() {
        errorlog(
            c"ERROR: Could not grab pointer (error_code = %d)\n"
                .as_ptr()
                .cast_mut(),
            (*error).error_code as ::core::ffi::c_int,
        );
        free(error as *mut ::core::ffi::c_void);
        return DragResult::Abort;
    }
    if (*reply).status as ::core::ffi::c_int != XCB_GRAB_STATUS_SUCCESS as ::core::ffi::c_int {
        errorlog(
            c"ERROR: Could not grab pointer (status = %d)\n"
                .as_ptr()
                .cast_mut(),
            (*reply).status as ::core::ffi::c_int,
        );
        free(reply as *mut ::core::ffi::c_void);
        return DragResult::Abort;
    }
    debuglog(
        c"%s:%s:%d - Pointer grab successful\n".as_ptr().cast_mut(),
        c"../src/drag.c".as_ptr(),
        c"drag_pointer".as_ptr(),
        238 as ::core::ffi::c_int,
    );
    free(reply as *mut ::core::ffi::c_void);
    let mut keyb_reply: *mut xcb_grab_keyboard_reply_t =
        ::core::ptr::null_mut::<xcb_grab_keyboard_reply_t>();
    let keyb_cookie: xcb_grab_keyboard_cookie_t = xcb_grab_keyboard(
        conn,
        false_0 as u8,
        root,
        XCB_CURRENT_TIME as xcb_timestamp_t,
        XCB_GRAB_MODE_ASYNC as ::core::ffi::c_int as u8,
        XCB_GRAB_MODE_ASYNC as ::core::ffi::c_int as u8,
    );
    keyb_reply = xcb_grab_keyboard_reply(conn, keyb_cookie, &raw mut error);
    if keyb_reply.is_null() {
        errorlog(
            c"ERROR: Could not grab keyboard (error_code = %d)\n"
                .as_ptr()
                .cast_mut(),
            (*error).error_code as ::core::ffi::c_int,
        );
        free(error as *mut ::core::ffi::c_void);
        xcb_ungrab_pointer(conn, XCB_CURRENT_TIME as xcb_timestamp_t);
        return DragResult::Abort;
    }
    if (*keyb_reply).status as ::core::ffi::c_int != XCB_GRAB_STATUS_SUCCESS as ::core::ffi::c_int {
        errorlog(
            c"ERROR: Could not grab keyboard (status = %d)\n"
                .as_ptr()
                .cast_mut(),
            (*keyb_reply).status as ::core::ffi::c_int,
        );
        free(keyb_reply as *mut ::core::ffi::c_void);
        xcb_ungrab_pointer(conn, XCB_CURRENT_TIME as xcb_timestamp_t);
        return DragResult::Abort;
    }
    debuglog(
        c"%s:%s:%d - Keyboard grab successful\n".as_ptr().cast_mut(),
        c"../src/drag.c".as_ptr(),
        c"drag_pointer".as_ptr(),
        266 as ::core::ffi::c_int,
    );
    free(keyb_reply as *mut ::core::ffi::c_void);
    let mut loop_0: drag_x11_cb = drag_x11_cb {
        prepare: ev_prepare {
            active: 0,
            pending: 0,
            priority: 0,
            data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            cb: None,
        },
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
        loop_0.old_rect = (*con).rect as Rect;
    }
    let ref mut fresh0 = (*(prepare as *mut ::core::ffi::c_void as *mut ev_watcher)).pending;
    *fresh0 = 0 as ::core::ffi::c_int;
    (*(prepare as *mut ::core::ffi::c_void as *mut ev_watcher)).active = *fresh0;
    (*(prepare as *mut ::core::ffi::c_void as *mut ev_watcher)).priority = 0 as ::core::ffi::c_int;
    (*prepare).cb = Some(
        xcb_drag_prepare_cb
            as unsafe extern "C" fn(*mut ev_loop, *mut ev_prepare, ::core::ffi::c_int) -> (),
    )
        as Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_prepare, ::core::ffi::c_int) -> ()>;
    memmove(
        &raw mut (*(prepare as *mut ev_watcher)).cb as *mut ::core::ffi::c_void,
        &raw mut (*prepare).cb as *const ::core::ffi::c_void,
        ::core::mem::size_of::<
            Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_prepare, ::core::ffi::c_int) -> ()>,
        >() as usize,
    );
    (*prepare).data = &raw mut loop_0 as *mut ::core::ffi::c_void;
    main_set_x11_cb(false_0 != 0);
    ev_prepare_start(main_loop, prepare);
    debuglog(
        c"%s:%s:%d - Entering drag loop (con = %p, button = %d)\n"
            .as_ptr()
            .cast_mut(),
        c"../src/drag.c".as_ptr(),
        c"drag_pointer".as_ptr(),
        288 as ::core::ffi::c_int,
        con,
        (*event).detail as ::core::ffi::c_int,
    );
    ev_loop(main_loop, 0 as ::core::ffi::c_int);
    debuglog(
        c"%s:%s:%d - Leaving drag loop (con = %p, button = %d, result = %d)\n"
            .as_ptr()
            .cast_mut(),
        c"../src/drag.c".as_ptr(),
        c"drag_pointer".as_ptr(),
        291 as ::core::ffi::c_int,
        con,
        (*event).detail as ::core::ffi::c_int,
        loop_0.result as ::core::ffi::c_uint,
    );
    ev_prepare_stop(main_loop, prepare);
    main_set_x11_cb(true_0 != 0);
    xcb_ungrab_keyboard(conn, XCB_CURRENT_TIME as xcb_timestamp_t);
    xcb_ungrab_pointer(conn, XCB_CURRENT_TIME as xcb_timestamp_t);
    xcb_flush(conn);
    loop_0.result
}

// Internal helpers use Rust linkage rather than foreign symbol lookup.
use crate::application::main_set_x11_cb;
use crate::xcursor::xcursor_get_cursor;
use i3_common::dpi::logical_px;

// Canonical Cairo backend ABI; renderer logic remains local.

use xcb_sys::{
    xcb_button_press_event_t, xcb_button_release_event_t, xcb_change_active_pointer_grab,
    xcb_client_message_event_t, xcb_cursor_t, xcb_flush, xcb_generic_error_t, xcb_generic_event_t,
    xcb_grab_keyboard, xcb_grab_keyboard_cookie_t, xcb_grab_keyboard_reply,
    xcb_grab_keyboard_reply_t, xcb_grab_pointer, xcb_grab_pointer_cookie_t, xcb_grab_pointer_reply,
    xcb_grab_pointer_reply_t, xcb_motion_notify_event_t, xcb_poll_for_event, xcb_timestamp_t,
    xcb_ungrab_keyboard, xcb_ungrab_pointer, xcb_unmap_notify_event_t, xcb_window_t,
    XCB_BUTTON_INDEX_ANY, XCB_BUTTON_PRESS, XCB_BUTTON_RELEASE, XCB_CLIENT_MESSAGE,
    XCB_CURRENT_TIME, XCB_EVENT_MASK_BUTTON_RELEASE, XCB_EVENT_MASK_POINTER_MOTION,
    XCB_GRAB_MODE_ASYNC, XCB_GRAB_STATUS_SUCCESS, XCB_KEY_PRESS, XCB_MOTION_NOTIFY, XCB_NONE,
    XCB_UNMAP_NOTIFY,
};

pub use libc::{free, memmove};

use crate::log::{debuglog, errorlog};

// Shared model types retain field layout while using one Rust identity.
pub use crate::types::Con;
pub use crate::types::Rect;
pub use i3_common::string::I3String as _i3String;

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
