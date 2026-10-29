/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 */
// Mechanical file-by-file Rust translation of libi3/screenshot_wallpaper.c.
// Generated with C2Rust 0.22.1; C ABI and original ownership retained.
extern "C" {
    pub type xcb_connection_t;
    fn xcb_change_window_attributes(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        value_mask: uint32_t,
        value_list: *const ::core::ffi::c_void,
    ) -> xcb_void_cookie_t;
    fn xcb_create_pixmap(
        c: *mut xcb_connection_t,
        depth: uint8_t,
        pid: xcb_pixmap_t,
        drawable: xcb_drawable_t,
        width: uint16_t,
        height: uint16_t,
    ) -> xcb_void_cookie_t;
    fn xcb_free_pixmap(c: *mut xcb_connection_t, pixmap: xcb_pixmap_t) -> xcb_void_cookie_t;
    fn xcb_create_gc(
        c: *mut xcb_connection_t,
        cid: xcb_gcontext_t,
        drawable: xcb_drawable_t,
        value_mask: uint32_t,
        value_list: *const ::core::ffi::c_void,
    ) -> xcb_void_cookie_t;
    fn xcb_free_gc(c: *mut xcb_connection_t, gc: xcb_gcontext_t) -> xcb_void_cookie_t;
    fn xcb_copy_area(
        c: *mut xcb_connection_t,
        src_drawable: xcb_drawable_t,
        dst_drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        src_x: int16_t,
        src_y: int16_t,
        dst_x: int16_t,
        dst_y: int16_t,
        width: uint16_t,
        height: uint16_t,
    ) -> xcb_void_cookie_t;
    fn xcb_flush(c: *mut xcb_connection_t) -> ::core::ffi::c_int;
    fn xcb_generate_id(c: *mut xcb_connection_t) -> uint32_t;
}
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type int16_t = __int16_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_void_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
pub type xcb_window_t = uint32_t;
pub type xcb_pixmap_t = uint32_t;
pub type xcb_gcontext_t = uint32_t;
pub type xcb_colormap_t = uint32_t;
pub type xcb_drawable_t = uint32_t;
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
pub type xcb_gc_t = ::core::ffi::c_uint;
pub const XCB_GC_ARC_MODE: xcb_gc_t = 4194304;
pub const XCB_GC_DASH_LIST: xcb_gc_t = 2097152;
pub const XCB_GC_DASH_OFFSET: xcb_gc_t = 1048576;
pub const XCB_GC_CLIP_MASK: xcb_gc_t = 524288;
pub const XCB_GC_CLIP_ORIGIN_Y: xcb_gc_t = 262144;
pub const XCB_GC_CLIP_ORIGIN_X: xcb_gc_t = 131072;
pub const XCB_GC_GRAPHICS_EXPOSURES: xcb_gc_t = 65536;
pub const XCB_GC_SUBWINDOW_MODE: xcb_gc_t = 32768;
pub const XCB_GC_FONT: xcb_gc_t = 16384;
pub const XCB_GC_TILE_STIPPLE_ORIGIN_Y: xcb_gc_t = 8192;
pub const XCB_GC_TILE_STIPPLE_ORIGIN_X: xcb_gc_t = 4096;
pub const XCB_GC_STIPPLE: xcb_gc_t = 2048;
pub const XCB_GC_TILE: xcb_gc_t = 1024;
pub const XCB_GC_FILL_RULE: xcb_gc_t = 512;
pub const XCB_GC_FILL_STYLE: xcb_gc_t = 256;
pub const XCB_GC_JOIN_STYLE: xcb_gc_t = 128;
pub const XCB_GC_CAP_STYLE: xcb_gc_t = 64;
pub const XCB_GC_LINE_STYLE: xcb_gc_t = 32;
pub const XCB_GC_LINE_WIDTH: xcb_gc_t = 16;
pub const XCB_GC_BACKGROUND: xcb_gc_t = 8;
pub const XCB_GC_FOREGROUND: xcb_gc_t = 4;
pub const XCB_GC_PLANE_MASK: xcb_gc_t = 2;
pub const XCB_GC_FUNCTION: xcb_gc_t = 1;
pub type xcb_gx_t = ::core::ffi::c_uint;
pub const XCB_GX_SET: xcb_gx_t = 15;
pub const XCB_GX_NAND: xcb_gx_t = 14;
pub const XCB_GX_OR_INVERTED: xcb_gx_t = 13;
pub const XCB_GX_COPY_INVERTED: xcb_gx_t = 12;
pub const XCB_GX_OR_REVERSE: xcb_gx_t = 11;
pub const XCB_GX_INVERT: xcb_gx_t = 10;
pub const XCB_GX_EQUIV: xcb_gx_t = 9;
pub const XCB_GX_NOR: xcb_gx_t = 8;
pub const XCB_GX_OR: xcb_gx_t = 7;
pub const XCB_GX_XOR: xcb_gx_t = 6;
pub const XCB_GX_NOOP: xcb_gx_t = 5;
pub const XCB_GX_AND_INVERTED: xcb_gx_t = 4;
pub const XCB_GX_COPY: xcb_gx_t = 3;
pub const XCB_GX_AND_REVERSE: xcb_gx_t = 2;
pub const XCB_GX_AND: xcb_gx_t = 1;
pub const XCB_GX_CLEAR: xcb_gx_t = 0;
pub type xcb_fill_style_t = ::core::ffi::c_uint;
pub const XCB_FILL_STYLE_OPAQUE_STIPPLED: xcb_fill_style_t = 3;
pub const XCB_FILL_STYLE_STIPPLED: xcb_fill_style_t = 2;
pub const XCB_FILL_STYLE_TILED: xcb_fill_style_t = 1;
pub const XCB_FILL_STYLE_SOLID: xcb_fill_style_t = 0;
pub type xcb_subwindow_mode_t = ::core::ffi::c_uint;
pub const XCB_SUBWINDOW_MODE_INCLUDE_INFERIORS: xcb_subwindow_mode_t = 1;
pub const XCB_SUBWINDOW_MODE_CLIP_BY_CHILDREN: xcb_subwindow_mode_t = 0;
#[no_mangle]
pub unsafe extern "C" fn set_screenshot_as_wallpaper(
    mut conn: *mut xcb_connection_t,
    mut screen: *mut xcb_screen_t,
) {
    let mut width: uint16_t = (*screen).width_in_pixels;
    let mut height: uint16_t = (*screen).height_in_pixels;
    let mut pixmap: xcb_pixmap_t = xcb_generate_id(conn) as xcb_pixmap_t;
    let mut gc: xcb_gcontext_t = xcb_generate_id(conn) as xcb_gcontext_t;
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
