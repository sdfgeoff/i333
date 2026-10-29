use crate::handlers::add_ignore_event;
use crate::xcursor::Cursor;
/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * xcb.c: Helper functions for easier usage of XCB
 *
 */
// Mechanical file-by-file Rust translation of src/xcb.c.
// Originally translated with C2Rust 0.22.1; subsequently maintained in Rust.

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
pub use crate::application::ignore_event_sequence_t;

pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();

pub static mut xcb_numlock_mask: ::core::ffi::c_uint = 0;

pub unsafe fn create_window(
    conn_0: *mut xcb_connection_t,
    dims: Rect,
    mut depth: u16,
    mut visual: xcb_visualid_t,
    window_class: u16,
    cursor: Cursor,
    map: bool,
    mask: u32,
    values: *mut u32,
) -> xcb_window_t {
    let result: xcb_window_t = xcb_generate_id(conn_0) as xcb_window_t;
    if window_class as ::core::ffi::c_int == XCB_WINDOW_CLASS_INPUT_ONLY as ::core::ffi::c_int {
        depth = XCB_COPY_FROM_PARENT as u16;
        visual = XCB_COPY_FROM_PARENT as xcb_visualid_t;
    }
    let gc_cookie: xcb_void_cookie_t = xcb_create_window(
        conn_0,
        depth as u8,
        result,
        root,
        dims.x as i16,
        dims.y as i16,
        dims.width as u16,
        dims.height as u16,
        0 as u16,
        window_class,
        visual,
        mask,
        values as *const ::core::ffi::c_void,
    );
    let error: *mut xcb_generic_error_t = xcb_request_check(conn_0, gc_cookie);
    if !error.is_null() {
        errorlog(
            c"ERROR: Could not create window. Error code: %d.\n"
                .as_ptr()
                .cast_mut(),
            (*error).error_code as ::core::ffi::c_int,
        );
    }
    let mut cursor_values: [u32; 1] = [xcursor_get_cursor(cursor)];
    xcb_change_window_attributes(
        conn_0,
        result,
        XCB_CW_CURSOR as ::core::ffi::c_int as u32,
        &raw mut cursor_values as *mut u32 as *const ::core::ffi::c_void,
    );
    if map {
        xcb_map_window(conn_0, result);
    }
    result
}

pub unsafe fn fake_absolute_configure_notify(con: *mut Con) {
    let mut absolute: xcb_rectangle_t = xcb_rectangle_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    if (*con).window.is_null() {
        return;
    }
    absolute.x = (*con).rect.x.wrapping_add((*con).window_rect.x) as i16;
    absolute.y = (*con).rect.y.wrapping_add((*con).window_rect.y) as i16;
    absolute.width = (*con).window_rect.width as u16;
    absolute.height = (*con).window_rect.height as u16;
    debuglog(
        c"%s:%s:%d - fake rect = (%d, %d, %d, %d)\n"
            .as_ptr()
            .cast_mut(),
        c"../src/xcb.c".as_ptr(),
        c"fake_absolute_configure_notify".as_ptr(),
        75 as ::core::ffi::c_int,
        absolute.x as ::core::ffi::c_int,
        absolute.y as ::core::ffi::c_int,
        absolute.width as ::core::ffi::c_int,
        absolute.height as ::core::ffi::c_int,
    );
    fake_configure_notify(conn, absolute, (*(*con).window).id, (*con).border_width);
}

pub unsafe fn send_take_focus(window: xcb_window_t, timestamp: xcb_timestamp_t) {
    let event: *mut ::core::ffi::c_void = scalloc(32 as usize, 1 as usize);
    let ev: *mut xcb_client_message_event_t = event as *mut xcb_client_message_event_t;
    (*ev).response_type = XCB_CLIENT_MESSAGE as u8;
    (*ev).window = window;
    (*ev).type_0 = A_WM_PROTOCOLS;
    (*ev).format = 32 as u8;
    (*ev).data.data32[0 as ::core::ffi::c_int as usize] = A_WM_TAKE_FOCUS as u32;
    (*ev).data.data32[1 as ::core::ffi::c_int as usize] = timestamp as u32;
    debuglog(
        c"%s:%s:%d - Sending WM_TAKE_FOCUS to the client\n"
            .as_ptr()
            .cast_mut(),
        c"../src/xcb.c".as_ptr(),
        c"send_take_focus".as_ptr(),
        98 as ::core::ffi::c_int,
    );
    xcb_send_event(
        conn,
        false_0 as u8,
        window,
        XCB_EVENT_MASK_NO_EVENT as ::core::ffi::c_int as u32,
        ev as *mut ::core::ffi::c_char,
    );
    free(event);
}

pub unsafe fn xcb_set_window_rect(
    conn_0: *mut xcb_connection_t,
    window: xcb_window_t,
    mut r: Rect,
) {
    let cookie: xcb_void_cookie_t = xcb_configure_window(
        conn_0,
        window,
        (XCB_CONFIG_WINDOW_X as ::core::ffi::c_int
            | XCB_CONFIG_WINDOW_Y as ::core::ffi::c_int
            | XCB_CONFIG_WINDOW_WIDTH as ::core::ffi::c_int
            | XCB_CONFIG_WINDOW_HEIGHT as ::core::ffi::c_int) as u16,
        &raw mut r.x as *const ::core::ffi::c_void,
    );
    add_ignore_event(
        ignore_event_sequence_t {
            value: cookie.sequence as u32,
        },
        -(1 as ::core::ffi::c_int),
    );
}

pub unsafe fn xcb_get_preferred_window_type(reply: *mut xcb_get_property_reply_t) -> xcb_atom_t {
    if reply.is_null() || xcb_get_property_value_length(reply) == 0 as ::core::ffi::c_int {
        return XCB_NONE as xcb_atom_t;
    }
    let mut atoms: *mut xcb_atom_t = ::core::ptr::null_mut::<xcb_atom_t>();
    atoms = xcb_get_property_value(reply) as *mut xcb_atom_t;
    if atoms.is_null() {
        return XCB_NONE as xcb_atom_t;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < xcb_get_property_value_length(reply)
        / ((*reply).format as ::core::ffi::c_int / 8 as ::core::ffi::c_int)
    {
        if *atoms.offset(i as isize) == A__NET_WM_WINDOW_TYPE_NORMAL
            || *atoms.offset(i as isize) == A__NET_WM_WINDOW_TYPE_DIALOG
            || *atoms.offset(i as isize) == A__NET_WM_WINDOW_TYPE_UTILITY
            || *atoms.offset(i as isize) == A__NET_WM_WINDOW_TYPE_TOOLBAR
            || *atoms.offset(i as isize) == A__NET_WM_WINDOW_TYPE_SPLASH
            || *atoms.offset(i as isize) == A__NET_WM_WINDOW_TYPE_MENU
            || *atoms.offset(i as isize) == A__NET_WM_WINDOW_TYPE_DROPDOWN_MENU
            || *atoms.offset(i as isize) == A__NET_WM_WINDOW_TYPE_POPUP_MENU
            || *atoms.offset(i as isize) == A__NET_WM_WINDOW_TYPE_TOOLTIP
            || *atoms.offset(i as isize) == A__NET_WM_WINDOW_TYPE_NOTIFICATION
        {
            return *atoms.offset(i as isize);
        }
        i += 1;
    }
    XCB_NONE as xcb_atom_t
}

pub unsafe fn xcb_reply_contains_atom(
    prop: *mut xcb_get_property_reply_t,
    atom: xcb_atom_t,
) -> bool {
    if prop.is_null() || xcb_get_property_value_length(prop) == 0 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    let mut atoms: *mut xcb_atom_t = ::core::ptr::null_mut::<xcb_atom_t>();
    atoms = xcb_get_property_value(prop) as *mut xcb_atom_t;
    if atoms.is_null() {
        return false_0 != 0;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < xcb_get_property_value_length(prop)
        / ((*prop).format as ::core::ffi::c_int / 8 as ::core::ffi::c_int)
    {
        if *atoms.offset(i as isize) == atom {
            return true_0 != 0;
        }
        i += 1;
    }
    false_0 != 0
}

pub unsafe fn get_visualtype_by_id(visual_id: xcb_visualid_t) -> *mut xcb_visualtype_t {
    let mut depth_iter: xcb_depth_iterator_t = xcb_depth_iterator_t {
        data: ::core::ptr::null_mut::<xcb_depth_t>(),
        rem: 0,
        index: 0,
    };
    depth_iter = xcb_screen_allowed_depths_iterator(root_screen);
    while depth_iter.rem != 0 {
        let mut visual_iter: xcb_visualtype_iterator_t = xcb_visualtype_iterator_t {
            data: ::core::ptr::null_mut::<xcb_visualtype_t>(),
            rem: 0,
            index: 0,
        };
        visual_iter = xcb_depth_visuals_iterator(depth_iter.data);
        while visual_iter.rem != 0 {
            if visual_id == (*visual_iter.data).visual_id {
                return visual_iter.data;
            }
            xcb_visualtype_next(&raw mut visual_iter);
        }
        xcb_depth_next(&raw mut depth_iter);
    }
    ::core::ptr::null_mut::<xcb_visualtype_t>()
}

pub unsafe fn get_visualid_by_depth(depth: u16) -> xcb_visualid_t {
    let mut depth_iter: xcb_depth_iterator_t = xcb_depth_iterator_t {
        data: ::core::ptr::null_mut::<xcb_depth_t>(),
        rem: 0,
        index: 0,
    };
    depth_iter = xcb_screen_allowed_depths_iterator(root_screen);
    while depth_iter.rem != 0 {
        if !((*depth_iter.data).depth as ::core::ffi::c_int != depth as ::core::ffi::c_int) {
            let visual_iter: xcb_visualtype_iterator_t =
                xcb_depth_visuals_iterator(depth_iter.data);
            if !(visual_iter.rem == 0) {
                return (*visual_iter.data).visual_id;
            }
        }
        xcb_depth_next(&raw mut depth_iter);
    }
    0 as xcb_visualid_t
}

pub unsafe fn xcb_add_property_atom(
    conn_0: *mut xcb_connection_t,
    window: xcb_window_t,
    property: xcb_atom_t,
    atom: xcb_atom_t,
) {
    let mut fresh0: [u32; 1] = [atom];
    xcb_change_property(
        conn_0,
        XCB_PROP_MODE_APPEND as ::core::ffi::c_int as u8,
        window,
        property,
        XCB_ATOM_ATOM as ::core::ffi::c_int as xcb_atom_t,
        32 as u8,
        1 as u32,
        &raw mut fresh0 as *mut u32 as *const ::core::ffi::c_void,
    );
}

pub unsafe fn xcb_remove_property_atom(
    conn_0: *mut xcb_connection_t,
    window: xcb_window_t,
    property: xcb_atom_t,
    atom: xcb_atom_t,
) {
    let mut atoms: *mut xcb_atom_t = ::core::ptr::null_mut::<xcb_atom_t>();
    xcb_grab_server(conn_0);
    let mut reply: *mut xcb_get_property_reply_t = xcb_get_property_reply(
        conn_0,
        xcb_get_property(
            conn_0,
            false_0 as u8,
            window,
            property,
            XCB_GET_PROPERTY_TYPE_ANY as ::core::ffi::c_int as xcb_atom_t,
            0 as u32,
            4096 as u32,
        ),
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if !(reply.is_null() || xcb_get_property_value_length(reply) == 0 as ::core::ffi::c_int) {
        atoms = xcb_get_property_value(reply) as *mut xcb_atom_t;
        if !atoms.is_null() {
            let mut num: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let current_size: ::core::ffi::c_int = xcb_get_property_value_length(reply)
                as ::core::ffi::c_int
                / ((*reply).format as ::core::ffi::c_int / 8 as ::core::ffi::c_int);
            let vla = current_size as usize;
            let mut values: Vec<xcb_atom_t> = ::std::vec::from_elem(0, vla);
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i < current_size {
                if *atoms.offset(i as isize) != atom {
                    let fresh1 = num;
                    num = num + 1;
                    *values.as_mut_ptr().offset(fresh1 as isize) = *atoms.offset(i as isize);
                }
                i += 1;
            }
            xcb_change_property(
                conn_0,
                XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as u8,
                window,
                property,
                XCB_ATOM_ATOM as ::core::ffi::c_int as xcb_atom_t,
                32 as u8,
                num as u32,
                values.as_mut_ptr() as *const ::core::ffi::c_void,
            );
        }
    }
    free(reply as *mut ::core::ffi::c_void);
    reply = ::core::ptr::null_mut::<xcb_get_property_reply_t>();
    xcb_ungrab_server(conn_0);
}

pub unsafe fn xcb_grab_buttons(
    conn_0: *mut xcb_connection_t,
    window: xcb_window_t,
    buttons: *mut ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while *buttons.offset(i as isize) > 0 as ::core::ffi::c_int {
        xcb_grab_button(
            conn_0,
            false_0 as u8,
            window,
            XCB_EVENT_MASK_BUTTON_PRESS as ::core::ffi::c_int as u16,
            XCB_GRAB_MODE_SYNC as ::core::ffi::c_int as u8,
            XCB_GRAB_MODE_ASYNC as ::core::ffi::c_int as u8,
            root,
            XCB_NONE as xcb_cursor_t,
            *buttons.offset(i as isize) as u8,
            XCB_BUTTON_MASK_ANY as ::core::ffi::c_int as u16,
        );
        i += 1;
    }
}

// Internal helpers use Rust linkage rather than foreign symbol lookup.
use crate::xcursor::xcursor_get_cursor;
use i3_common::safewrappers::scalloc;

// Canonical Cairo backend ABI; renderer logic remains local.

use xcb_sys::{
    xcb_atom_t, xcb_change_property, xcb_change_window_attributes, xcb_client_message_event_t,
    xcb_configure_window, xcb_connection_t, xcb_create_window, xcb_cursor_t, xcb_depth_iterator_t,
    xcb_depth_next, xcb_depth_t, xcb_depth_visuals_iterator, xcb_generate_id, xcb_generic_error_t,
    xcb_get_property, xcb_get_property_reply, xcb_get_property_reply_t, xcb_get_property_value,
    xcb_get_property_value_length, xcb_grab_button, xcb_grab_server, xcb_map_window,
    xcb_rectangle_t, xcb_request_check, xcb_screen_allowed_depths_iterator, xcb_send_event,
    xcb_timestamp_t, xcb_ungrab_server, xcb_visualid_t, xcb_visualtype_iterator_t,
    xcb_visualtype_next, xcb_visualtype_t, xcb_void_cookie_t, xcb_window_t, XCB_ATOM_ATOM,
    XCB_BUTTON_MASK_ANY, XCB_CLIENT_MESSAGE, XCB_CONFIG_WINDOW_HEIGHT, XCB_CONFIG_WINDOW_WIDTH,
    XCB_CONFIG_WINDOW_X, XCB_CONFIG_WINDOW_Y, XCB_COPY_FROM_PARENT, XCB_CW_CURSOR,
    XCB_EVENT_MASK_BUTTON_PRESS, XCB_EVENT_MASK_NO_EVENT, XCB_GET_PROPERTY_TYPE_ANY,
    XCB_GRAB_MODE_ASYNC, XCB_GRAB_MODE_SYNC, XCB_NONE, XCB_PROP_MODE_APPEND, XCB_PROP_MODE_REPLACE,
    XCB_WINDOW_CLASS_INPUT_ONLY,
};

pub use libc::free;

use crate::log::{debuglog, errorlog};

// Shared model types retain field layout while using one Rust identity.
pub use crate::types::Con;
pub use crate::types::Rect;
pub use i3_common::string::I3String as _i3String;

// Internal calls and globals use canonical Rust identities.
use crate::application::conn;
use crate::application::root;
use crate::application::root_screen;
use crate::application::A_WM_PROTOCOLS;
use crate::application::A_WM_TAKE_FOCUS;
use crate::application::A__NET_WM_WINDOW_TYPE_DIALOG;
use crate::application::A__NET_WM_WINDOW_TYPE_DROPDOWN_MENU;
use crate::application::A__NET_WM_WINDOW_TYPE_MENU;
use crate::application::A__NET_WM_WINDOW_TYPE_NORMAL;
use crate::application::A__NET_WM_WINDOW_TYPE_NOTIFICATION;
use crate::application::A__NET_WM_WINDOW_TYPE_POPUP_MENU;
use crate::application::A__NET_WM_WINDOW_TYPE_SPLASH;
use crate::application::A__NET_WM_WINDOW_TYPE_TOOLBAR;
use crate::application::A__NET_WM_WINDOW_TYPE_TOOLTIP;
use crate::application::A__NET_WM_WINDOW_TYPE_UTILITY;
use i3_common::fake_configure_notify::fake_configure_notify;
