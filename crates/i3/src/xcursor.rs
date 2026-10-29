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
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;

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
pub const __ASSERT_FUNCTION: [::core::ffi::c_char; 55] = unsafe {
    ::core::mem::transmute::<[u8; 55], [::core::ffi::c_char; 55]>(
        *b"xcb_cursor_t xcursor_get_cursor(enum xcursor_cursor_t)\0",
    )
};
static mut ctx: *mut xcb_cursor_context_t =
    ::core::ptr::null::<xcb_cursor_context_t>() as *mut xcb_cursor_context_t;
static mut cursors: [xcb_cursor_t; 9] = [0; 9];

pub unsafe fn xcursor_load_cursors() {
    if xcb_cursor_context_new(conn, root_screen, &raw mut ctx) < 0 as ::core::ffi::c_int {
        errx(EXIT_FAILURE, c"Cannot allocate xcursor context".as_ptr());
    }
    cursors[XCURSOR_CURSOR_POINTER as ::core::ffi::c_int as usize] =
        xcb_cursor_load_cursor(ctx, c"left_ptr".as_ptr());
    cursors[XCURSOR_CURSOR_RESIZE_HORIZONTAL as ::core::ffi::c_int as usize] =
        xcb_cursor_load_cursor(ctx, c"sb_h_double_arrow".as_ptr());
    cursors[XCURSOR_CURSOR_RESIZE_VERTICAL as ::core::ffi::c_int as usize] =
        xcb_cursor_load_cursor(ctx, c"sb_v_double_arrow".as_ptr());
    cursors[XCURSOR_CURSOR_WATCH as ::core::ffi::c_int as usize] =
        xcb_cursor_load_cursor(ctx, c"watch".as_ptr());
    cursors[XCURSOR_CURSOR_MOVE as ::core::ffi::c_int as usize] =
        xcb_cursor_load_cursor(ctx, c"fleur".as_ptr());
    cursors[XCURSOR_CURSOR_TOP_LEFT_CORNER as ::core::ffi::c_int as usize] =
        xcb_cursor_load_cursor(ctx, c"top_left_corner".as_ptr());
    cursors[XCURSOR_CURSOR_TOP_RIGHT_CORNER as ::core::ffi::c_int as usize] =
        xcb_cursor_load_cursor(ctx, c"top_right_corner".as_ptr());
    cursors[XCURSOR_CURSOR_BOTTOM_LEFT_CORNER as ::core::ffi::c_int as usize] =
        xcb_cursor_load_cursor(ctx, c"bottom_left_corner".as_ptr());
    cursors[XCURSOR_CURSOR_BOTTOM_RIGHT_CORNER as ::core::ffi::c_int as usize] =
        xcb_cursor_load_cursor(ctx, c"bottom_right_corner".as_ptr());
}

pub unsafe fn xcursor_set_root_cursor(cursor_id: ::core::ffi::c_int) {
    let mut fresh0: [uint32_t; 1] = [(xcursor_get_cursor
        as unsafe fn(xcursor_cursor_t) -> xcb_cursor_t)(
        cursor_id as xcursor_cursor_t
    )];
    xcb_change_window_attributes(
        conn,
        root,
        XCB_CW_CURSOR as ::core::ffi::c_int as uint32_t,
        &raw mut fresh0 as *mut uint32_t as *const ::core::ffi::c_void,
    );
}

pub unsafe fn xcursor_get_cursor(c: xcursor_cursor_t) -> xcb_cursor_t {
    if (c as ::core::ffi::c_uint) < XCURSOR_CURSOR_MAX as ::core::ffi::c_int as ::core::ffi::c_uint
    {
    } else {
        __assert_fail(
            c"c < XCURSOR_CURSOR_MAX".as_ptr(),
            c"../src/xcursor.c".as_ptr(),
            55 as ::core::ffi::c_uint,
            __ASSERT_FUNCTION.as_ptr(),
        );
    };
    cursors[c as usize]
}
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

use xcb_sys::{
    xcb_change_window_attributes, xcb_cursor_context_new, xcb_cursor_context_t,
    xcb_cursor_load_cursor, xcb_cursor_t, XCB_CW_CURSOR,
};

use i3_common::native_stdio::{__assert_fail, errx};

// Internal calls and globals use canonical Rust identities.
use crate::application::conn;
use crate::application::root;
use crate::application::root_screen;
