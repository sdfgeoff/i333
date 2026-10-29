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
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __time_t = ::core::ffi::c_long;
pub type __suseconds_t = ::core::ffi::c_long;
pub type time_t = __time_t;
pub type int16_t = __int16_t;

pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;

pub type i3String = _i3String;

pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const SCRATCHPAD_CHANGED: C2RustUnnamed = 2;
pub const SCRATCHPAD_FRESH: C2RustUnnamed = 1;
pub const SCRATCHPAD_NONE: C2RustUnnamed = 0;

pub type C2RustUnnamed_4 = ::core::ffi::c_uint;
pub const FLOATING_USER_ON: C2RustUnnamed_4 = 3;
pub const FLOATING_AUTO_ON: C2RustUnnamed_4 = 2;
pub const FLOATING_USER_OFF: C2RustUnnamed_4 = 1;
pub const FLOATING_AUTO_OFF: C2RustUnnamed_4 = 0;
pub type border_style_t = ::core::ffi::c_uint;
pub const BS_NORMAL: border_style_t = 2;
pub const BS_PIXEL: border_style_t = 1;
pub const BS_NONE: border_style_t = 0;
pub type layout_t = ::core::ffi::c_uint;
pub const L_SPLITH: layout_t = 6;
pub const L_SPLITV: layout_t = 5;
pub const L_OUTPUT: layout_t = 4;
pub const L_DOCKAREA: layout_t = 3;
pub const L_TABBED: layout_t = 2;
pub const L_STACKED: layout_t = 1;
pub const L_DEFAULT: layout_t = 0;
pub type fullscreen_mode_t = ::core::ffi::c_uint;
pub const CF_GLOBAL: fullscreen_mode_t = 2;
pub const CF_OUTPUT: fullscreen_mode_t = 1;
pub const CF_NONE: fullscreen_mode_t = 0;

pub type C2RustUnnamed_6 = ::core::ffi::c_uint;
pub const M_BELOW: C2RustUnnamed_6 = 2;
pub const M_ASSIGN_WS: C2RustUnnamed_6 = 1;
pub const M_HERE: C2RustUnnamed_6 = 0;
pub type C2RustUnnamed_7 = ::core::ffi::c_uint;
pub const WM_FLOATING: C2RustUnnamed_7 = 6;
pub const WM_FLOATING_USER: C2RustUnnamed_7 = 5;
pub const WM_FLOATING_AUTO: C2RustUnnamed_7 = 4;
pub const WM_TILING: C2RustUnnamed_7 = 3;
pub const WM_TILING_USER: C2RustUnnamed_7 = 2;
pub const WM_TILING_AUTO: C2RustUnnamed_7 = 1;
pub const WM_ANY: C2RustUnnamed_7 = 0;
pub type C2RustUnnamed_8 = ::core::ffi::c_int;
pub const M_DOCK_BOTTOM: C2RustUnnamed_8 = 3;
pub const M_DOCK_TOP: C2RustUnnamed_8 = 2;
pub const M_DOCK_ANY: C2RustUnnamed_8 = 1;
pub const M_NODOCK: C2RustUnnamed_8 = 0;
pub const M_DONTCHECK: C2RustUnnamed_8 = -1;
pub type C2RustUnnamed_9 = ::core::ffi::c_int;
pub const U_OLDEST: C2RustUnnamed_9 = 1;
pub const U_LATEST: C2RustUnnamed_9 = 0;
pub const U_DONTCHECK: C2RustUnnamed_9 = -1;

pub type C2RustUnnamed_10 = ::core::ffi::c_uint;
pub const W_DOCK_BOTTOM: C2RustUnnamed_10 = 2;
pub const W_DOCK_TOP: C2RustUnnamed_10 = 1;
pub const W_NODOCK: C2RustUnnamed_10 = 0;

pub type C2RustUnnamed_13 = ::core::ffi::c_uint;
pub const A_TO_OUTPUT: C2RustUnnamed_13 = 16;
pub const A_TO_WORKSPACE_NUMBER: C2RustUnnamed_13 = 8;
pub const A_NO_FOCUS: C2RustUnnamed_13 = 4;
pub const A_TO_WORKSPACE: C2RustUnnamed_13 = 2;
pub const A_COMMAND: C2RustUnnamed_13 = 1;
pub const A_ANY: C2RustUnnamed_13 = 0;

pub type C2RustUnnamed_15 = ::core::ffi::c_uint;
pub const CT_DOCKAREA: C2RustUnnamed_15 = 5;
pub const CT_WORKSPACE: C2RustUnnamed_15 = 4;
pub const CT_FLOATING_CON: C2RustUnnamed_15 = 3;
pub const CT_CON: C2RustUnnamed_15 = 2;
pub const CT_OUTPUT: C2RustUnnamed_15 = 1;
pub const CT_ROOT: C2RustUnnamed_15 = 0;

pub type xcursor_cursor_t = ::core::ffi::c_uint;
pub const XCURSOR_CURSOR_MAX: xcursor_cursor_t = 9;
pub const XCURSOR_CURSOR_MOVE: xcursor_cursor_t = 8;
pub const XCURSOR_CURSOR_WATCH: xcursor_cursor_t = 7;
pub const XCURSOR_CURSOR_BOTTOM_RIGHT_CORNER: xcursor_cursor_t = 6;
pub const XCURSOR_CURSOR_BOTTOM_LEFT_CORNER: xcursor_cursor_t = 5;
pub const XCURSOR_CURSOR_TOP_RIGHT_CORNER: xcursor_cursor_t = 4;
pub const XCURSOR_CURSOR_TOP_LEFT_CORNER: xcursor_cursor_t = 3;
pub const XCURSOR_CURSOR_RESIZE_VERTICAL: xcursor_cursor_t = 2;
pub const XCURSOR_CURSOR_RESIZE_HORIZONTAL: xcursor_cursor_t = 1;
pub const XCURSOR_CURSOR_POINTER: xcursor_cursor_t = 0;
pub type callback_t = Option<
    unsafe fn(
        *mut Con,
        *mut Rect,
        uint32_t,
        uint32_t,
        *const xcb_button_press_event_t,
        *const ::core::ffi::c_void,
    ) -> (),
>;
pub type drag_result_t = ::core::ffi::c_uint;
pub const DRAG_ABORT: drag_result_t = 3;
pub const DRAG_REVERT: drag_result_t = 2;
pub const DRAG_SUCCESS: drag_result_t = 1;
pub const DRAGGING: drag_result_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct drag_x11_cb {
    pub prepare: ev_prepare,
    pub result: drag_result_t,
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

unsafe fn threshold_exceeded(x1: uint32_t, y1: uint32_t, x2: uint32_t, y2: uint32_t) -> bool {
    let threshold: uint32_t = logical_px(15 as ::core::ffi::c_int) as uint32_t;
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
                        (*dragloop).result = DRAG_SUCCESS;
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
                    (*dragloop).result = DRAG_REVERT;
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
                            (*dragloop).result = DRAG_ABORT;
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
            if !((*dragloop).result as ::core::ffi::c_uint
                != DRAGGING as ::core::ffi::c_int as ::core::ffi::c_uint)
            {
                continue;
            }
            ev_break(loop_0, EVBREAK_ONE as ::core::ffi::c_int);
            if (*dragloop).result as ::core::ffi::c_uint
                == DRAG_SUCCESS as ::core::ffi::c_int as ::core::ffi::c_uint
            {
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
            (*last_motion_notify).root_x as uint32_t,
            (*last_motion_notify).root_y as uint32_t,
            (*(*dragloop).event).root_x as uint32_t,
            (*(*dragloop).event).root_y as uint32_t,
        ) as ::core::ffi::c_int
            != 0
    {
        if (*dragloop).xcursor as ::core::ffi::c_long != XCB_NONE {
            xcb_change_active_pointer_grab(
                conn,
                (*dragloop).xcursor,
                XCB_CURRENT_TIME as xcb_timestamp_t,
                (XCB_EVENT_MASK_BUTTON_RELEASE as ::core::ffi::c_int
                    | XCB_EVENT_MASK_POINTER_MOTION as ::core::ffi::c_int)
                    as uint16_t,
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
            (*last_motion_notify).root_x as uint32_t,
            (*last_motion_notify).root_y as uint32_t,
            (*dragloop).event,
            (*dragloop).extra,
        );
    }
    free(last_motion_notify as *mut ::core::ffi::c_void);
    last_motion_notify = ::core::ptr::null_mut::<xcb_motion_notify_event_t>();
    xcb_flush(conn);
    (*dragloop).result as ::core::ffi::c_uint
        != DRAGGING as ::core::ffi::c_int as ::core::ffi::c_uint
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
    cursor: ::core::ffi::c_int,
    use_threshold: bool,
    callback: callback_t,
    extra: *const ::core::ffi::c_void,
) -> drag_result_t {
    let xcursor: xcb_cursor_t = (if cursor != 0 {
        xcursor_get_cursor(cursor as xcursor_cursor_t) as ::core::ffi::c_long
    } else {
        XCB_NONE
    }) as xcb_cursor_t;
    let mut reply: *mut xcb_grab_pointer_reply_t =
        ::core::ptr::null_mut::<xcb_grab_pointer_reply_t>();
    let mut error: *mut xcb_generic_error_t = ::core::ptr::null_mut::<xcb_generic_error_t>();
    let cookie: xcb_grab_pointer_cookie_t = xcb_grab_pointer(
        conn,
        false_0 as uint8_t,
        root,
        (XCB_EVENT_MASK_BUTTON_RELEASE as ::core::ffi::c_int
            | XCB_EVENT_MASK_POINTER_MOTION as ::core::ffi::c_int) as uint16_t,
        XCB_GRAB_MODE_ASYNC as ::core::ffi::c_int as uint8_t,
        XCB_GRAB_MODE_ASYNC as ::core::ffi::c_int as uint8_t,
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
        return DRAG_ABORT;
    }
    if (*reply).status as ::core::ffi::c_int != XCB_GRAB_STATUS_SUCCESS as ::core::ffi::c_int {
        errorlog(
            c"ERROR: Could not grab pointer (status = %d)\n"
                .as_ptr()
                .cast_mut(),
            (*reply).status as ::core::ffi::c_int,
        );
        free(reply as *mut ::core::ffi::c_void);
        return DRAG_ABORT;
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
        false_0 as uint8_t,
        root,
        XCB_CURRENT_TIME as xcb_timestamp_t,
        XCB_GRAB_MODE_ASYNC as ::core::ffi::c_int as uint8_t,
        XCB_GRAB_MODE_ASYNC as ::core::ffi::c_int as uint8_t,
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
        return DRAG_ABORT;
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
        return DRAG_ABORT;
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
        result: DRAGGING,
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
        >() as size_t,
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
