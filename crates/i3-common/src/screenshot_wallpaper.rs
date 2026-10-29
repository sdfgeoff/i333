/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 */
// Mechanical file-by-file Rust translation of libi3/screenshot_wallpaper.c.
// Originally translated with C2Rust 0.22.1; subsequently maintained in Rust.
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type int16_t = __int16_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;

pub unsafe fn set_screenshot_as_wallpaper(conn: *mut xcb_connection_t, screen: *mut xcb_screen_t) {
    let width: uint16_t = (*screen).width_in_pixels;
    let height: uint16_t = (*screen).height_in_pixels;
    let pixmap: xcb_pixmap_t = xcb_generate_id(conn) as xcb_pixmap_t;
    let gc: xcb_gcontext_t = xcb_generate_id(conn) as xcb_gcontext_t;
    xcb_create_pixmap(
        conn,
        (*screen).root_depth,
        pixmap,
        (*screen).root as xcb_drawable_t,
        width,
        height,
    );
    let mut fresh0: [uint32_t; 4] = [
        XCB_GX_COPY as ::core::ffi::c_int as uint32_t,
        !(0 as ::core::ffi::c_int) as uint32_t,
        XCB_FILL_STYLE_SOLID as ::core::ffi::c_int as uint32_t,
        XCB_SUBWINDOW_MODE_INCLUDE_INFERIORS as ::core::ffi::c_int as uint32_t,
    ];
    xcb_create_gc(
        conn,
        gc,
        (*screen).root as xcb_drawable_t,
        (XCB_GC_FUNCTION as ::core::ffi::c_int
            | XCB_GC_PLANE_MASK as ::core::ffi::c_int
            | XCB_GC_FILL_STYLE as ::core::ffi::c_int
            | XCB_GC_SUBWINDOW_MODE as ::core::ffi::c_int) as uint32_t,
        &raw mut fresh0 as *mut uint32_t as *const ::core::ffi::c_void,
    );
    xcb_copy_area(
        conn,
        (*screen).root as xcb_drawable_t,
        pixmap as xcb_drawable_t,
        gc,
        0 as int16_t,
        0 as int16_t,
        0 as int16_t,
        0 as int16_t,
        width,
        height,
    );
    let mut fresh1: [uint32_t; 1] = [pixmap];
    xcb_change_window_attributes(
        conn,
        (*screen).root,
        XCB_CW_BACK_PIXMAP as ::core::ffi::c_int as uint32_t,
        &raw mut fresh1 as *mut uint32_t as *const ::core::ffi::c_void,
    );
    xcb_free_gc(conn, gc);
    xcb_free_pixmap(conn, pixmap);
    xcb_flush(conn);
}

use xcb_sys::{
    xcb_change_window_attributes, xcb_connection_t, xcb_copy_area, xcb_create_gc,
    xcb_create_pixmap, xcb_drawable_t, xcb_flush, xcb_free_gc, xcb_free_pixmap, xcb_gcontext_t,
    xcb_generate_id, xcb_pixmap_t, xcb_screen_t, XCB_CW_BACK_PIXMAP, XCB_FILL_STYLE_SOLID,
    XCB_GC_FILL_STYLE, XCB_GC_FUNCTION, XCB_GC_PLANE_MASK, XCB_GC_SUBWINDOW_MODE, XCB_GX_COPY,
    XCB_SUBWINDOW_MODE_INCLUDE_INFERIORS,
};
