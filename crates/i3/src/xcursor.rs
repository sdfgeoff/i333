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

use crate::application::{conn, root, root_screen};
use i3_common::native_stdio::errx;
use std::ffi::CStr;
use std::ptr::NonNull;
use xcb_sys::{
    XCB_CW_CURSOR, xcb_change_window_attributes, xcb_cursor_context_free, xcb_cursor_context_new,
    xcb_cursor_context_t, xcb_cursor_load_cursor, xcb_cursor_t,
};

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
static mut CURSORS: [xcb_cursor_t; Cursor::COUNT] = [0; Cursor::COUNT];

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

// Constructed only from a successful context_new on the display thread. The
// context borrows the connection during loading and is destroyed before return.
struct CursorContext(NonNull<xcb_cursor_context_t>);

impl Drop for CursorContext {
    fn drop(&mut self) {
        // SAFETY: This is the unique successful context_new result. Its display
        // connection is still live. xcb-cursor explicitly leaves loaded cursor
        // resources alive when freeing the context (xcb_cursor.h).
        unsafe { xcb_cursor_context_free(self.0.as_ptr()) };
    }
}

/// Load themed cursors into the display's cache.
///
/// # Safety
/// Requires exclusive display-thread access to a live `conn`, its valid
/// `root_screen`, and the cursor cache. Cached IDs belong to this connection.
pub unsafe fn xcursor_load_cursors() {
    let mut context = std::ptr::null_mut();
    // SAFETY: The caller supplies a live connection/screen. context is a valid
    // out pointer and remains local until a successful result transfers ownership.
    let status = unsafe { xcb_cursor_context_new(conn, root_screen, &mut context) };
    if status < 0 || context.is_null() {
        unsafe {
            errx(
                libc::EXIT_FAILURE,
                c"Cannot allocate xcursor context".as_ptr(),
            );
        }
    }
    // errx exits on failure, so the checked success pointer is non-null.
    let context = CursorContext(NonNull::new(context).expect("successful xcursor context"));
    let mut cursors = [0; Cursor::COUNT];
    for (cursor, name) in CURSOR_NAMES {
        // SAFETY: The context is live and the static name is NUL terminated.
        cursors[cursor as usize] =
            unsafe { xcb_cursor_load_cursor(context.0.as_ptr(), name.as_ptr()) };
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
        let values = [xcursor_get_cursor(cursor)];
        xcb_change_window_attributes(conn, root, XCB_CW_CURSOR, values.as_ptr().cast());
    }
}

/// Read a cursor ID belonging to the current display connection.
///
/// # Safety
/// Requires event-thread access without concurrent cache writes. The caller
/// must use the returned resource ID only with the connection that loaded it.
pub unsafe fn xcursor_get_cursor(cursor: Cursor) -> xcb_cursor_t {
    // Every Cursor variant is a valid cache index; the count is not a variant.
    // SAFETY: Cache access is synchronized by the caller's event-thread contract.
    unsafe { CURSORS[cursor as usize] }
}
