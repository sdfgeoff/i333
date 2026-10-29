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
// Generated with C2Rust 0.22.1; C ABI and original ownership retained.
extern "C" {
    pub type xcb_connection_t;
    pub type xcb_cursor_context_t;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn __assert_single_arg(_: bool) -> bool;
    fn errx(__status: ::core::ffi::c_int, __format: *const ::core::ffi::c_char, ...) -> !;
    fn xcb_change_window_attributes(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        value_mask: uint32_t,
        value_list: *const ::core::ffi::c_void,
    ) -> xcb_void_cookie_t;
    static mut conn: *mut xcb_connection_t;
    static mut root_screen: *mut xcb_screen_t;
    static mut root: xcb_window_t;
    fn xcb_cursor_context_new(
        conn_0: *mut xcb_connection_t,
        screen: *mut xcb_screen_t,
        ctx_0: *mut *mut xcb_cursor_context_t,
    ) -> ::core::ffi::c_int;
    fn xcb_cursor_load_cursor(
        ctx_0: *mut xcb_cursor_context_t,
        name: *const ::core::ffi::c_char,
    ) -> xcb_cursor_t;
}
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_void_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
pub type xcb_window_t = uint32_t;
pub type xcb_cursor_t = uint32_t;
pub type xcb_colormap_t = uint32_t;
pub type xcb_visualid_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_screen_t {
    pub root: xcb_window_t,
    pub default_colormap: xcb_colormap_t,
    pub white_pixel: uint32_t,
    pub black_pixel: uint32_t,
    pub current_input_masks: uint32_t,
    pub width_in_pixels: uint16_t,
    pub height_in_pixels: uint16_t,
    pub width_in_millimeters: uint16_t,
    pub height_in_millimeters: uint16_t,
    pub min_installed_maps: uint16_t,
    pub max_installed_maps: uint16_t,
    pub root_visual: xcb_visualid_t,
    pub backing_stores: uint8_t,
    pub save_unders: uint8_t,
    pub root_depth: uint8_t,
    pub allowed_depths_len: uint8_t,
}
pub type xcb_cw_t = ::core::ffi::c_uint;
pub const XCB_CW_CURSOR: xcb_cw_t = 16384;
pub const XCB_CW_COLORMAP: xcb_cw_t = 8192;
pub const XCB_CW_DONT_PROPAGATE: xcb_cw_t = 4096;
pub const XCB_CW_EVENT_MASK: xcb_cw_t = 2048;
pub const XCB_CW_SAVE_UNDER: xcb_cw_t = 1024;
pub const XCB_CW_OVERRIDE_REDIRECT: xcb_cw_t = 512;
pub const XCB_CW_BACKING_PIXEL: xcb_cw_t = 256;
pub const XCB_CW_BACKING_PLANES: xcb_cw_t = 128;
pub const XCB_CW_BACKING_STORE: xcb_cw_t = 64;
pub const XCB_CW_WIN_GRAVITY: xcb_cw_t = 32;
pub const XCB_CW_BIT_GRAVITY: xcb_cw_t = 16;
pub const XCB_CW_BORDER_PIXEL: xcb_cw_t = 8;
pub const XCB_CW_BORDER_PIXMAP: xcb_cw_t = 4;
pub const XCB_CW_BACK_PIXEL: xcb_cw_t = 2;
pub const XCB_CW_BACK_PIXMAP: xcb_cw_t = 1;
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
#[no_mangle]
pub unsafe extern "C" fn xcursor_load_cursors() {
    if xcb_cursor_context_new(conn, root_screen, &raw mut ctx) < 0 as ::core::ffi::c_int {
        errx(
            EXIT_FAILURE,
            b"Cannot allocate xcursor context\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    cursors[XCURSOR_CURSOR_POINTER as ::core::ffi::c_int as usize] = xcb_cursor_load_cursor(
        ctx,
        b"left_ptr\0" as *const u8 as *const ::core::ffi::c_char,
    );
    cursors[XCURSOR_CURSOR_RESIZE_HORIZONTAL as ::core::ffi::c_int as usize] =
        xcb_cursor_load_cursor(
            ctx,
            b"sb_h_double_arrow\0" as *const u8 as *const ::core::ffi::c_char,
        );
    cursors[XCURSOR_CURSOR_RESIZE_VERTICAL as ::core::ffi::c_int as usize] = xcb_cursor_load_cursor(
        ctx,
        b"sb_v_double_arrow\0" as *const u8 as *const ::core::ffi::c_char,
    );
    cursors[XCURSOR_CURSOR_WATCH as ::core::ffi::c_int as usize] =
        xcb_cursor_load_cursor(ctx, b"watch\0" as *const u8 as *const ::core::ffi::c_char);
    cursors[XCURSOR_CURSOR_MOVE as ::core::ffi::c_int as usize] =
        xcb_cursor_load_cursor(ctx, b"fleur\0" as *const u8 as *const ::core::ffi::c_char);
    cursors[XCURSOR_CURSOR_TOP_LEFT_CORNER as ::core::ffi::c_int as usize] = xcb_cursor_load_cursor(
        ctx,
        b"top_left_corner\0" as *const u8 as *const ::core::ffi::c_char,
    );
    cursors[XCURSOR_CURSOR_TOP_RIGHT_CORNER as ::core::ffi::c_int as usize] =
        xcb_cursor_load_cursor(
            ctx,
            b"top_right_corner\0" as *const u8 as *const ::core::ffi::c_char,
        );
    cursors[XCURSOR_CURSOR_BOTTOM_LEFT_CORNER as ::core::ffi::c_int as usize] =
        xcb_cursor_load_cursor(
            ctx,
            b"bottom_left_corner\0" as *const u8 as *const ::core::ffi::c_char,
        );
    cursors[XCURSOR_CURSOR_BOTTOM_RIGHT_CORNER as ::core::ffi::c_int as usize] =
        xcb_cursor_load_cursor(
            ctx,
            b"bottom_right_corner\0" as *const u8 as *const ::core::ffi::c_char,
        );
}
#[no_mangle]
pub unsafe extern "C" fn xcursor_set_root_cursor(mut cursor_id: ::core::ffi::c_int) {
    let mut fresh0: [uint32_t; 1] = [(xcursor_get_cursor
        as unsafe extern "C" fn(xcursor_cursor_t) -> xcb_cursor_t)(
        cursor_id as xcursor_cursor_t
    )];
    xcb_change_window_attributes(
        conn,
        root,
        XCB_CW_CURSOR as ::core::ffi::c_int as uint32_t,
        &raw mut fresh0 as *mut uint32_t as *const ::core::ffi::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn xcursor_get_cursor(mut c: xcursor_cursor_t) -> xcb_cursor_t {
    if (c as ::core::ffi::c_uint) < XCURSOR_CURSOR_MAX as ::core::ffi::c_int as ::core::ffi::c_uint
    {
    } else {
        __assert_fail(
            b"c < XCURSOR_CURSOR_MAX\0" as *const u8 as *const ::core::ffi::c_char,
            b"../src/xcursor.c\0" as *const u8 as *const ::core::ffi::c_char,
            55 as ::core::ffi::c_uint,
            __ASSERT_FUNCTION.as_ptr(),
        );
    };
    return cursors[c as usize];
}
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
