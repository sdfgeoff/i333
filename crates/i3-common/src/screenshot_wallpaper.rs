/*
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE).
 */
//! Keep a server-side copy of the current root contents as its background.
#![deny(unsafe_op_in_unsafe_fn)]

use xcb_sys::{
    XCB_CW_BACK_PIXMAP, XCB_FILL_STYLE_SOLID, XCB_GC_FILL_STYLE, XCB_GC_FUNCTION,
    XCB_GC_PLANE_MASK, XCB_GC_SUBWINDOW_MODE, XCB_GX_COPY, XCB_SUBWINDOW_MODE_INCLUDE_INFERIORS,
    xcb_change_window_attributes, xcb_connection_t, xcb_copy_area, xcb_create_gc,
    xcb_create_pixmap, xcb_flush, xcb_free_gc, xcb_free_pixmap, xcb_generate_id, xcb_screen_t,
};

/// Capture the screen, including child windows, without waiting for a helper process.
///
/// # Safety
/// conn must be a live XCB connection on which requests may be issued. screen
/// must point to its initialized screen record and remain readable during this
/// call. The caller must permit replacing that screen's root background pixmap.
pub unsafe fn set_screenshot_as_wallpaper(conn: *mut xcb_connection_t, screen: *mut xcb_screen_t) {
    // SAFETY: the caller guarantees the borrowed screen record remains live.
    let screen = unsafe { &*screen };
    let (width, height) = (screen.width_in_pixels, screen.height_in_pixels);
    // XCB value lists follow increasing mask-bit order: function, plane mask,
    // fill style, then subwindow mode. XCB copies these initialized values.
    let gc_values = [
        XCB_GX_COPY,
        u32::MAX,
        XCB_FILL_STYLE_SOLID,
        XCB_SUBWINDOW_MODE_INCLUDE_INFERIORS,
    ];
    let gc_mask = XCB_GC_FUNCTION | XCB_GC_PLANE_MASK | XCB_GC_FILL_STYLE | XCB_GC_SUBWINDOW_MODE;
    // SAFETY: connection, root and depth come from the caller's live screen.
    // IDs are allocated on this connection; fixed value lists match their masks.
    // All requests stay ordered, so the root retains the referenced pixmap after
    // our client resource IDs are freed. No blocking reply/child wait is needed.
    unsafe {
        let pixmap = xcb_generate_id(conn);
        let gc = xcb_generate_id(conn);
        xcb_create_pixmap(conn, screen.root_depth, pixmap, screen.root, width, height);
        xcb_create_gc(conn, gc, screen.root, gc_mask, gc_values.as_ptr().cast());
        xcb_copy_area(conn, screen.root, pixmap, gc, 0, 0, 0, 0, width, height);
        let background = [pixmap];
        xcb_change_window_attributes(
            conn,
            screen.root,
            XCB_CW_BACK_PIXMAP,
            background.as_ptr().cast(),
        );
        xcb_free_gc(conn, gc);
        xcb_free_pixmap(conn, pixmap);
        xcb_flush(conn);
    }
}
