/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * xcursor.c: xcursor support for themed cursors.
 *
 */
#![deny(unsafe_op_in_unsafe_fn)]

use crate::application::{conn_screen, root};
use ::xcb::{x, Xid, XidNew};
use i3_common::display::connection;
use i3_common::native_stdio::errx;
use std::ffi::CStr;
use xcb_interop::cursor::CursorContext;

/// An index into the cursor cache, separate from native XCB cursor IDs.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Cursor {
    Pointer = 0,
    ResizeHorizontal = 1,
    ResizeVertical = 2,
    TopLeftCorner = 3,
    TopRightCorner = 4,
    BottomLeftCorner = 5,
    BottomRightCorner = 6,
    Watch = 7,
    Move = 8,
}

impl Cursor {
    pub const COUNT: usize = 9;
}

// Native cursor IDs remain owned by the display connection. Only the display
// event thread may read or replace the cache.
static mut CURSORS: [u32; Cursor::COUNT] = [0; Cursor::COUNT];

// Preserve the original native load order (which determines X resource IDs).
const CURSOR_NAMES: [(Cursor, &CStr); Cursor::COUNT] = [
    (Cursor::Pointer, c"left_ptr"),
    (Cursor::ResizeHorizontal, c"sb_h_double_arrow"),
    (Cursor::ResizeVertical, c"sb_v_double_arrow"),
    (Cursor::Watch, c"watch"),
    (Cursor::Move, c"fleur"),
    (Cursor::TopLeftCorner, c"top_left_corner"),
    (Cursor::TopRightCorner, c"top_right_corner"),
    (Cursor::BottomLeftCorner, c"bottom_left_corner"),
    (Cursor::BottomRightCorner, c"bottom_right_corner"),
];

/// Load themed cursors into the display's cache.
///
/// # Safety
/// Requires exclusive display-thread access to the registered display, its
/// selected screen and the cursor cache. Cached IDs belong to that connection.
pub unsafe fn xcursor_load_cursors() {
    let Some(connection) = connection() else {
        return;
    };
    // The wrapper retains the display and uniquely owns the native theme
    // context until all cursor IDs have been loaded in their original order.
    let context = match CursorContext::new(connection, unsafe { conn_screen }) {
        Ok(context) => context,
        Err(_) => unsafe {
            errx(
                libc::EXIT_FAILURE,
                c"Cannot allocate xcursor context".as_ptr(),
            )
        },
    };
    let mut cursors = [0; Cursor::COUNT];
    for (cursor, name) in CURSOR_NAMES {
        cursors[cursor as usize] = context.load(name).resource_id();
    }
    // SAFETY: Exclusive display-thread access is required by this function.
    unsafe { CURSORS = cursors };
}

/// Set the initialized root window's cursor from the display's cache.
///
/// # Safety
/// Requires exclusive display-thread access to the live connection/root and a
/// cursor cache initialized for that same connection.
pub unsafe fn xcursor_set_root_cursor(cursor: Cursor) {
    // SAFETY: The caller supplies the live display and initialized cache. XCB
    // copies the single attribute value before the stack array is dropped.
    unsafe {
        let Some(connection) = connection() else {
            return;
        };
        connection.send_request(&x::ChangeWindowAttributes {
            window: x::Window::new(root),
            value_list: &[x::Cw::Cursor(x::Cursor::new(xcursor_get_cursor(cursor)))],
        });
    }
}

/// Read a cursor ID belonging to the current display connection.
///
/// # Safety
/// Requires event-thread access without concurrent cache writes. The caller
/// must use the returned resource ID only with the connection that loaded it.
pub unsafe fn xcursor_get_cursor(cursor: Cursor) -> u32 {
    // Every Cursor variant is a valid cache index; the count is not a variant.
    // SAFETY: Cache access is synchronized by the caller's event-thread contract.
    unsafe { CURSORS[cursor as usize] }
}
