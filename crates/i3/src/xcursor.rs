/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * xcursor.c: xcursor support for themed cursors.
 *
 */
// Mechanical file-by-file Rust translation of src/xcursor.c.
// Originally translated with C2Rust 0.22.1; subsequently maintained in Rust.

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

static mut ctx: *mut xcb_cursor_context_t = ::core::ptr::null_mut();
static mut cursors: [xcb_cursor_t; Cursor::COUNT] = [0; Cursor::COUNT];

pub unsafe fn xcursor_load_cursors() {
    if xcb_cursor_context_new(conn, root_screen, &raw mut ctx) < 0 {
        errx(
            libc::EXIT_FAILURE,
            c"Cannot allocate xcursor context".as_ptr(),
        );
    }
    cursors[Cursor::Pointer as usize] = xcb_cursor_load_cursor(ctx, c"left_ptr".as_ptr());
    cursors[Cursor::ResizeHorizontal as usize] =
        xcb_cursor_load_cursor(ctx, c"sb_h_double_arrow".as_ptr());
    cursors[Cursor::ResizeVertical as usize] =
        xcb_cursor_load_cursor(ctx, c"sb_v_double_arrow".as_ptr());
    cursors[Cursor::Watch as usize] = xcb_cursor_load_cursor(ctx, c"watch".as_ptr());
    cursors[Cursor::Move as usize] = xcb_cursor_load_cursor(ctx, c"fleur".as_ptr());
    cursors[Cursor::TopLeftCorner as usize] =
        xcb_cursor_load_cursor(ctx, c"top_left_corner".as_ptr());
    cursors[Cursor::TopRightCorner as usize] =
        xcb_cursor_load_cursor(ctx, c"top_right_corner".as_ptr());
    cursors[Cursor::BottomLeftCorner as usize] =
        xcb_cursor_load_cursor(ctx, c"bottom_left_corner".as_ptr());
    cursors[Cursor::BottomRightCorner as usize] =
        xcb_cursor_load_cursor(ctx, c"bottom_right_corner".as_ptr());
}

pub unsafe fn xcursor_set_root_cursor(cursor: Cursor) {
    let values = [xcursor_get_cursor(cursor)];
    xcb_change_window_attributes(conn, root, XCB_CW_CURSOR as u32, values.as_ptr().cast());
}

pub unsafe fn xcursor_get_cursor(cursor: Cursor) -> xcb_cursor_t {
    // Every Cursor variant is a valid cache index; the count is not a variant.
    cursors[cursor as usize]
}

use crate::application::{conn, root, root_screen};
use i3_common::native_stdio::errx;
use xcb_sys::{
    xcb_change_window_attributes, xcb_cursor_context_new, xcb_cursor_context_t,
    xcb_cursor_load_cursor, xcb_cursor_t, XCB_CW_CURSOR,
};
