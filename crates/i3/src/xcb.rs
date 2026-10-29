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

use crate::application::ignore_event_sequence_t;
use std::ptr::NonNull;

struct Native<T>(NonNull<T>);
impl<T> Native<T> {
    /// A non-null pointer transfers a unique, complete malloc-backed XCB reply.
    unsafe fn take(pointer: *mut T) -> Option<Self> {
        NonNull::new(pointer).map(Self)
    }
    fn raw(&self) -> *mut T {
        self.0.as_ptr()
    }
}
impl<T> Drop for Native<T> {
    fn drop(&mut self) {
        unsafe {
            free(self.raw().cast());
        }
    }
}
struct ServerGrab(*mut xcb_connection_t);
impl Drop for ServerGrab {
    fn drop(&mut self) {
        unsafe {
            xcb_ungrab_server(self.0);
        }
    }
}

fn atom_words(bytes: &[u8]) -> Vec<u32> {
    bytes
        .chunks_exact(4)
        .map(|word| u32::from_ne_bytes(word.try_into().unwrap()))
        .collect()
}
/// Borrow only within the complete XCB allocation advertised by the reply.
/// The caller keeps the reply immutable/live until the returned slice expires.
unsafe fn property_bytes<'a>(reply: *const xcb_get_property_reply_t) -> Option<&'a [u8]> {
    let reply = reply.as_ref()?;
    if reply.format != 32 {
        return None;
    }
    let length = (reply.value_len as usize).checked_mul(4)?;
    let available = (reply.length as usize).checked_mul(4)?;
    if length > available || length > isize::MAX as usize {
        return None;
    }
    Some(std::slice::from_raw_parts(
        (reply as *const xcb_get_property_reply_t).add(1).cast(),
        length,
    ))
}
fn preferred_atom(atoms: impl IntoIterator<Item = u32>, supported: &[u32]) -> u32 {
    atoms
        .into_iter()
        .find(|atom| *atom != 0 && supported.contains(atom))
        .unwrap_or(0)
}
fn without_atom(atoms: impl IntoIterator<Item = u32>, removed: u32) -> Vec<u32> {
    atoms.into_iter().filter(|&atom| atom != removed).collect()
}
fn absolute_geometry(container: Rect, window: Rect) -> xcb_rectangle_t {
    xcb_rectangle_t {
        x: container.x.wrapping_add(window.x) as i16,
        y: container.y.wrapping_add(window.y) as i16,
        width: window.width as u16,
        height: window.height as u16,
    }
}
fn take_focus_message(
    window: xcb_window_t,
    timestamp: xcb_timestamp_t,
    protocols: u32,
    take_focus: u32,
) -> xcb_client_message_event_t {
    xcb_client_message_event_t {
        response_type: XCB_CLIENT_MESSAGE as u8,
        format: 32,
        sequence: 0,
        window,
        type_0: protocols,
        data: xcb_sys::xcb_client_message_data_t {
            data32: [take_focus, timestamp, 0, 0, 0],
        },
    }
}
const _: () = assert!(std::mem::size_of::<xcb_client_message_event_t>() == 32);

pub static mut xcb_numlock_mask: ::core::ffi::c_uint = 0;

/// Native display/graph boundary: requires a live event-thread connection and
/// initialized, exclusively accessible pointers/resources for this operation.
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
    let error = Native::take(xcb_request_check(conn_0, gc_cookie));
    if let Some(error) = error {
        errorlog(
            c"ERROR: Could not create window. Error code: %d.\n"
                .as_ptr()
                .cast_mut(),
            (*error.raw()).error_code as ::core::ffi::c_int,
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

/// Native display/graph boundary: requires a live event-thread connection and
/// initialized, exclusively accessible pointers/resources for this operation.
pub unsafe fn fake_absolute_configure_notify(con: *mut Con) {
    if (*con).window.is_null() {
        return;
    }
    let absolute = absolute_geometry((*con).rect, (*con).window_rect);
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

/// Native display/graph boundary: requires a live event-thread connection and
/// initialized, exclusively accessible pointers/resources for this operation.
pub unsafe fn send_take_focus(window: xcb_window_t, timestamp: xcb_timestamp_t) {
    let event = take_focus_message(window, timestamp, A_WM_PROTOCOLS, A_WM_TAKE_FOCUS);
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
        0,
        window,
        XCB_EVENT_MASK_NO_EVENT as ::core::ffi::c_int as u32,
        (&raw const event).cast(),
    );
}

/// Native display/graph boundary: requires a live event-thread connection and
/// initialized, exclusively accessible pointers/resources for this operation.
pub unsafe fn xcb_set_window_rect(conn_0: *mut xcb_connection_t, window: xcb_window_t, r: Rect) {
    let values = [r.x, r.y, r.width, r.height];
    let cookie: xcb_void_cookie_t = xcb_configure_window(
        conn_0,
        window,
        (XCB_CONFIG_WINDOW_X as ::core::ffi::c_int
            | XCB_CONFIG_WINDOW_Y as ::core::ffi::c_int
            | XCB_CONFIG_WINDOW_WIDTH as ::core::ffi::c_int
            | XCB_CONFIG_WINDOW_HEIGHT as ::core::ffi::c_int) as u16,
        values.as_ptr().cast(),
    );
    add_ignore_event(
        ignore_event_sequence_t {
            value: cookie.sequence as u32,
        },
        -(1 as ::core::ffi::c_int),
    );
}

/// reply borrows a complete native allocation on the owning event thread.
pub unsafe fn xcb_get_preferred_window_type(reply: *mut xcb_get_property_reply_t) -> xcb_atom_t {
    let Some(bytes) = property_bytes(reply) else {
        return 0;
    };
    preferred_atom(
        atom_words(bytes),
        &[
            A__NET_WM_WINDOW_TYPE_NORMAL,
            A__NET_WM_WINDOW_TYPE_DIALOG,
            A__NET_WM_WINDOW_TYPE_UTILITY,
            A__NET_WM_WINDOW_TYPE_TOOLBAR,
            A__NET_WM_WINDOW_TYPE_SPLASH,
            A__NET_WM_WINDOW_TYPE_MENU,
            A__NET_WM_WINDOW_TYPE_DROPDOWN_MENU,
            A__NET_WM_WINDOW_TYPE_POPUP_MENU,
            A__NET_WM_WINDOW_TYPE_TOOLTIP,
            A__NET_WM_WINDOW_TYPE_NOTIFICATION,
        ],
    )
}
/// prop borrows a complete native property reply for this event-thread query.
pub unsafe fn xcb_reply_contains_atom(
    prop: *mut xcb_get_property_reply_t,
    atom: xcb_atom_t,
) -> bool {
    property_bytes(prop).is_some_and(|bytes| atom_words(bytes).contains(&atom))
}

/// Native display/graph boundary: requires a live event-thread connection and
/// initialized, exclusively accessible pointers/resources for this operation.
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

/// Native display/graph boundary: requires a live event-thread connection and
/// initialized, exclusively accessible pointers/resources for this operation.
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

/// Native display/graph boundary: requires a live event-thread connection and
/// initialized, exclusively accessible pointers/resources for this operation.
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

/// Native display/graph boundary: requires a live event-thread connection and
/// initialized, exclusively accessible pointers/resources for this operation.
pub unsafe fn xcb_remove_property_atom(
    conn_0: *mut xcb_connection_t,
    window: xcb_window_t,
    property: xcb_atom_t,
    atom: xcb_atom_t,
) {
    // The guard releases the native server lock on every Rust return. The
    // reply drops before the grab; filtered payload owns all words sent to XCB.
    xcb_grab_server(conn_0);
    let _grab = ServerGrab(conn_0);
    let reply = Native::take(xcb_get_property_reply(
        conn_0,
        xcb_get_property(
            conn_0,
            0,
            window,
            property,
            XCB_GET_PROPERTY_TYPE_ANY as u32,
            0,
            4096,
        ),
        std::ptr::null_mut(),
    ));
    let Some(reply) = reply else {
        return;
    };
    let Some(bytes) = property_bytes(reply.raw()) else {
        return;
    };
    if bytes.is_empty() {
        return;
    }
    let values = without_atom(atom_words(bytes), atom);
    xcb_change_property(
        conn_0,
        XCB_PROP_MODE_REPLACE as u8,
        window,
        property,
        XCB_ATOM_ATOM,
        32,
        values.len() as u32,
        values.as_ptr().cast(),
    );
}

/// Native display/graph boundary: requires a live event-thread connection and
/// initialized, exclusively accessible pointers/resources for this operation.
pub unsafe fn xcb_grab_buttons(
    conn_0: *mut xcb_connection_t,
    window: xcb_window_t,
    buttons: *mut ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while *buttons.offset(i as isize) > 0 as ::core::ffi::c_int {
        xcb_grab_button(
            conn_0,
            0,
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn configure_geometry_preserves_unsigned_addition_and_wire_truncation() {
        let rect = absolute_geometry(
            Rect {
                x: u32::MAX,
                y: 32767,
                width: 0,
                height: 0,
            },
            Rect {
                x: 2,
                y: 1,
                width: 65537,
                height: u32::MAX,
            },
        );
        assert_eq!(
            (rect.x, rect.y, rect.width, rect.height),
            (1, i16::MIN, 1, u16::MAX)
        );
    }
    #[test]
    fn atom_selection_and_removal_keep_property_order_and_duplicate_semantics() {
        let words = [99u32, 7, 8, 7, 0];
        let bytes: Vec<_> = words.iter().flat_map(|word| word.to_ne_bytes()).collect();
        assert_eq!(preferred_atom(atom_words(&bytes), &[8, 7]), 7);
        assert_eq!(without_atom(atom_words(&bytes), 7), [99, 8, 0]);
        assert_eq!(preferred_atom([], &[7]), 0);
        assert!(atom_words(&bytes[..3]).is_empty());
    }
    #[test]
    fn take_focus_packet_initializes_every_native_data_word() {
        let event = take_focus_message(0xffffffff, 0x12345678, 12, 34);
        assert_eq!(
            (
                event.response_type,
                event.format,
                event.sequence,
                event.window,
                event.type_0
            ),
            (XCB_CLIENT_MESSAGE as u8, 32, 0, u32::MAX, 12)
        );
        assert_eq!(unsafe { event.data.data32 }, [34, 0x12345678, 0, 0, 0]);
    }
}
