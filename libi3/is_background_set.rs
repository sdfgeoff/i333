/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 */
// Mechanical file-by-file Rust translation of libi3/is_background_set.c.
// Generated with C2Rust 0.22.1; C ABI and original ownership retained.
extern "C" {
    pub type xcb_connection_t;
    pub type _cairo_region;
    fn xcb_create_window(
        c: *mut xcb_connection_t,
        depth: uint8_t,
        wid: xcb_window_t,
        parent: xcb_window_t,
        x: int16_t,
        y: int16_t,
        width: uint16_t,
        height: uint16_t,
        border_width: uint16_t,
        _class: uint16_t,
        visual: xcb_visualid_t,
        value_mask: uint32_t,
        value_list: *const ::core::ffi::c_void,
    ) -> xcb_void_cookie_t;
    fn xcb_get_window_attributes_unchecked(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_get_window_attributes_cookie_t;
    fn xcb_get_window_attributes_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_get_window_attributes_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_get_window_attributes_reply_t;
    fn xcb_destroy_window(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t;
    fn xcb_map_window(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t;
    fn xcb_get_geometry_unchecked(
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
    ) -> xcb_get_geometry_cookie_t;
    fn xcb_get_geometry_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_get_geometry_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_get_geometry_reply_t;
    fn xcb_query_tree_unchecked(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_query_tree_cookie_t;
    fn xcb_query_tree_children(R: *const xcb_query_tree_reply_t) -> *mut xcb_window_t;
    fn xcb_query_tree_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_query_tree_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_query_tree_reply_t;
    fn xcb_clear_area(
        c: *mut xcb_connection_t,
        exposures: uint8_t,
        window: xcb_window_t,
        x: int16_t,
        y: int16_t,
        width: uint16_t,
        height: uint16_t,
    ) -> xcb_void_cookie_t;
    fn xcb_get_image_unchecked(
        c: *mut xcb_connection_t,
        format: uint8_t,
        drawable: xcb_drawable_t,
        x: int16_t,
        y: int16_t,
        width: uint16_t,
        height: uint16_t,
        plane_mask: uint32_t,
    ) -> xcb_get_image_cookie_t;
    fn xcb_get_image_data(R: *const xcb_get_image_reply_t) -> *mut uint8_t;
    fn xcb_get_image_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_get_image_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_get_image_reply_t;
    fn xcb_generate_id(c: *mut xcb_connection_t) -> uint32_t;
    fn cairo_region_create_rectangle(
        rectangle: *const cairo_rectangle_int_t,
    ) -> *mut cairo_region_t;
    fn cairo_region_destroy(region: *mut cairo_region_t);
    fn cairo_region_num_rectangles(region: *const cairo_region_t) -> ::core::ffi::c_int;
    fn cairo_region_get_rectangle(
        region: *const cairo_region_t,
        nth: ::core::ffi::c_int,
        rectangle: *mut cairo_rectangle_int_t,
    );
    fn cairo_region_subtract_rectangle(
        dst: *mut cairo_region_t,
        rectangle: *const cairo_rectangle_int_t,
    ) -> cairo_status_t;
    fn xcb_aux_sync(c: *mut xcb_connection_t);
    fn free(__ptr: *mut ::core::ffi::c_void);
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
pub struct xcb_generic_error_t {
    pub response_type: uint8_t,
    pub error_code: uint8_t,
    pub sequence: uint16_t,
    pub resource_id: uint32_t,
    pub minor_code: uint16_t,
    pub major_code: uint8_t,
    pub pad0: uint8_t,
    pub pad: [uint32_t; 5],
    pub full_sequence: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_void_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
pub type xcb_window_t = uint32_t;
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
pub type xcb_window_class_t = ::core::ffi::c_uint;
pub const XCB_WINDOW_CLASS_INPUT_ONLY: xcb_window_class_t = 2;
pub const XCB_WINDOW_CLASS_INPUT_OUTPUT: xcb_window_class_t = 1;
pub const XCB_WINDOW_CLASS_COPY_FROM_PARENT: xcb_window_class_t = 0;
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
pub type xcb_map_state_t = ::core::ffi::c_uint;
pub const XCB_MAP_STATE_VIEWABLE: xcb_map_state_t = 2;
pub const XCB_MAP_STATE_UNVIEWABLE: xcb_map_state_t = 1;
pub const XCB_MAP_STATE_UNMAPPED: xcb_map_state_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_get_window_attributes_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_get_window_attributes_reply_t {
    pub response_type: uint8_t,
    pub backing_store: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub visual: xcb_visualid_t,
    pub _class: uint16_t,
    pub bit_gravity: uint8_t,
    pub win_gravity: uint8_t,
    pub backing_planes: uint32_t,
    pub backing_pixel: uint32_t,
    pub save_under: uint8_t,
    pub map_is_installed: uint8_t,
    pub map_state: uint8_t,
    pub override_redirect: uint8_t,
    pub colormap: xcb_colormap_t,
    pub all_event_masks: uint32_t,
    pub your_event_mask: uint32_t,
    pub do_not_propagate_mask: uint16_t,
    pub pad0: [uint8_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_get_geometry_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_get_geometry_reply_t {
    pub response_type: uint8_t,
    pub depth: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub root: xcb_window_t,
    pub x: int16_t,
    pub y: int16_t,
    pub width: uint16_t,
    pub height: uint16_t,
    pub border_width: uint16_t,
    pub pad0: [uint8_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_query_tree_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_query_tree_reply_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub root: xcb_window_t,
    pub parent: xcb_window_t,
    pub children_len: uint16_t,
    pub pad1: [uint8_t; 14],
}
pub type xcb_image_format_t = ::core::ffi::c_uint;
pub const XCB_IMAGE_FORMAT_Z_PIXMAP: xcb_image_format_t = 2;
pub const XCB_IMAGE_FORMAT_XY_PIXMAP: xcb_image_format_t = 1;
pub const XCB_IMAGE_FORMAT_XY_BITMAP: xcb_image_format_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_get_image_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_get_image_reply_t {
    pub response_type: uint8_t,
    pub depth: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub visual: xcb_visualid_t,
    pub pad0: [uint8_t; 20],
}
pub type _cairo_status = ::core::ffi::c_uint;
pub const CAIRO_STATUS_LAST_STATUS: _cairo_status = 45;
pub const CAIRO_STATUS_SVG_FONT_ERROR: _cairo_status = 44;
pub const CAIRO_STATUS_DWRITE_ERROR: _cairo_status = 43;
pub const CAIRO_STATUS_TAG_ERROR: _cairo_status = 42;
pub const CAIRO_STATUS_WIN32_GDI_ERROR: _cairo_status = 41;
pub const CAIRO_STATUS_FREETYPE_ERROR: _cairo_status = 40;
pub const CAIRO_STATUS_PNG_ERROR: _cairo_status = 39;
pub const CAIRO_STATUS_JBIG2_GLOBAL_MISSING: _cairo_status = 38;
pub const CAIRO_STATUS_DEVICE_FINISHED: _cairo_status = 37;
pub const CAIRO_STATUS_INVALID_MESH_CONSTRUCTION: _cairo_status = 36;
pub const CAIRO_STATUS_DEVICE_ERROR: _cairo_status = 35;
pub const CAIRO_STATUS_DEVICE_TYPE_MISMATCH: _cairo_status = 34;
pub const CAIRO_STATUS_USER_FONT_NOT_IMPLEMENTED: _cairo_status = 33;
pub const CAIRO_STATUS_INVALID_SIZE: _cairo_status = 32;
pub const CAIRO_STATUS_INVALID_WEIGHT: _cairo_status = 31;
pub const CAIRO_STATUS_INVALID_SLANT: _cairo_status = 30;
pub const CAIRO_STATUS_INVALID_CLUSTERS: _cairo_status = 29;
pub const CAIRO_STATUS_NEGATIVE_COUNT: _cairo_status = 28;
pub const CAIRO_STATUS_USER_FONT_ERROR: _cairo_status = 27;
pub const CAIRO_STATUS_USER_FONT_IMMUTABLE: _cairo_status = 26;
pub const CAIRO_STATUS_FONT_TYPE_MISMATCH: _cairo_status = 25;
pub const CAIRO_STATUS_INVALID_STRIDE: _cairo_status = 24;
pub const CAIRO_STATUS_TEMP_FILE_ERROR: _cairo_status = 23;
pub const CAIRO_STATUS_CLIP_NOT_REPRESENTABLE: _cairo_status = 22;
pub const CAIRO_STATUS_INVALID_INDEX: _cairo_status = 21;
pub const CAIRO_STATUS_INVALID_DSC_COMMENT: _cairo_status = 20;
pub const CAIRO_STATUS_INVALID_DASH: _cairo_status = 19;
pub const CAIRO_STATUS_FILE_NOT_FOUND: _cairo_status = 18;
pub const CAIRO_STATUS_INVALID_VISUAL: _cairo_status = 17;
pub const CAIRO_STATUS_INVALID_FORMAT: _cairo_status = 16;
pub const CAIRO_STATUS_INVALID_CONTENT: _cairo_status = 15;
pub const CAIRO_STATUS_PATTERN_TYPE_MISMATCH: _cairo_status = 14;
pub const CAIRO_STATUS_SURFACE_TYPE_MISMATCH: _cairo_status = 13;
pub const CAIRO_STATUS_SURFACE_FINISHED: _cairo_status = 12;
pub const CAIRO_STATUS_WRITE_ERROR: _cairo_status = 11;
pub const CAIRO_STATUS_READ_ERROR: _cairo_status = 10;
pub const CAIRO_STATUS_INVALID_PATH_DATA: _cairo_status = 9;
pub const CAIRO_STATUS_INVALID_STRING: _cairo_status = 8;
pub const CAIRO_STATUS_NULL_POINTER: _cairo_status = 7;
pub const CAIRO_STATUS_INVALID_STATUS: _cairo_status = 6;
pub const CAIRO_STATUS_INVALID_MATRIX: _cairo_status = 5;
pub const CAIRO_STATUS_NO_CURRENT_POINT: _cairo_status = 4;
pub const CAIRO_STATUS_INVALID_POP_GROUP: _cairo_status = 3;
pub const CAIRO_STATUS_INVALID_RESTORE: _cairo_status = 2;
pub const CAIRO_STATUS_NO_MEMORY: _cairo_status = 1;
pub const CAIRO_STATUS_SUCCESS: _cairo_status = 0;
pub type cairo_status_t = _cairo_status;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cairo_rectangle_int {
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
}
pub type cairo_rectangle_int_t = _cairo_rectangle_int;
pub type cairo_region_t = _cairo_region;
pub const XCB_COPY_FROM_PARENT: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
unsafe extern "C" fn unobscured_region(
    mut conn: *mut xcb_connection_t,
    mut window: xcb_window_t,
    mut window_width: uint16_t,
    mut window_height: uint16_t,
) -> *mut cairo_region_t {
    let mut rectangle: cairo_rectangle_int_t = _cairo_rectangle_int {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    let mut region: *mut cairo_region_t = ::core::ptr::null_mut::<cairo_region_t>();
    rectangle.x = 0 as ::core::ffi::c_int;
    rectangle.y = 0 as ::core::ffi::c_int;
    rectangle.width = window_width as ::core::ffi::c_int;
    rectangle.height = window_height as ::core::ffi::c_int;
    region = cairo_region_create_rectangle(&raw mut rectangle);
    let mut tree: *mut xcb_query_tree_reply_t = xcb_query_tree_reply(
        conn,
        xcb_query_tree_unchecked(conn, window),
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if tree.is_null() {
        return region;
    }
    let mut n_children: uint16_t = (*tree).children_len;
    let mut children: *mut xcb_window_t = xcb_query_tree_children(tree);
    let vla = n_children as usize;
    let mut geometries: Vec<xcb_get_geometry_cookie_t> =
        ::std::vec::from_elem(xcb_get_geometry_cookie_t { sequence: 0 }, vla);
    let vla_0 = n_children as usize;
    let mut attributes: Vec<xcb_get_window_attributes_cookie_t> =
        ::std::vec::from_elem(xcb_get_window_attributes_cookie_t { sequence: 0 }, vla_0);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < n_children as ::core::ffi::c_int {
        *geometries.as_mut_ptr().offset(i as isize) =
            xcb_get_geometry_unchecked(conn, *children.offset(i as isize) as xcb_drawable_t);
        *attributes.as_mut_ptr().offset(i as isize) =
            xcb_get_window_attributes_unchecked(conn, *children.offset(i as isize));
        i += 1;
    }
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < n_children as ::core::ffi::c_int {
        let mut geom: *mut xcb_get_geometry_reply_t = xcb_get_geometry_reply(
            conn,
            *geometries.as_mut_ptr().offset(i_0 as isize),
            ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
        );
        let mut attr: *mut xcb_get_window_attributes_reply_t = xcb_get_window_attributes_reply(
            conn,
            *attributes.as_mut_ptr().offset(i_0 as isize),
            ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
        );
        if !geom.is_null()
            && !attr.is_null()
            && (*attr).map_state as ::core::ffi::c_int
                == XCB_MAP_STATE_VIEWABLE as ::core::ffi::c_int
        {
            rectangle.x = (*geom).x as ::core::ffi::c_int;
            rectangle.y = (*geom).y as ::core::ffi::c_int;
            rectangle.width = (*geom).width as ::core::ffi::c_int;
            rectangle.height = (*geom).height as ::core::ffi::c_int;
            cairo_region_subtract_rectangle(region, &raw mut rectangle);
        }
        free(geom as *mut ::core::ffi::c_void);
        free(attr as *mut ::core::ffi::c_void);
        i_0 += 1;
    }
    free(tree as *mut ::core::ffi::c_void);
    return region;
}
unsafe extern "C" fn find_unobscured_pixel(
    mut conn: *mut xcb_connection_t,
    mut window: xcb_window_t,
    mut window_width: uint16_t,
    mut window_height: uint16_t,
    mut x: *mut uint16_t,
    mut y: *mut uint16_t,
) {
    let mut region: *mut cairo_region_t =
        unobscured_region(conn, window, window_width, window_height);
    if cairo_region_num_rectangles(region) > 0 as ::core::ffi::c_int {
        let mut rect: cairo_rectangle_int_t = _cairo_rectangle_int {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        };
        cairo_region_get_rectangle(region, 0 as ::core::ffi::c_int, &raw mut rect);
        *x = rect.x as uint16_t;
        *y = rect.y as uint16_t;
    } else {
        *x = 0 as uint16_t;
        *y = 0 as uint16_t;
    }
    cairo_region_destroy(region);
}
unsafe extern "C" fn flicker_window_at(
    mut conn: *mut xcb_connection_t,
    mut screen: *mut xcb_screen_t,
    mut x: int16_t,
    mut y: int16_t,
    mut window: xcb_window_t,
    mut pixel: uint32_t,
) -> uint32_t {
    let mut fresh0: [uint32_t; 2] = [pixel, 1 as ::core::ffi::c_int as uint32_t];
    xcb_create_window(
        conn,
        XCB_COPY_FROM_PARENT as uint8_t,
        window,
        (*screen).root,
        x,
        y,
        10 as uint16_t,
        10 as uint16_t,
        0 as uint16_t,
        XCB_WINDOW_CLASS_INPUT_OUTPUT as ::core::ffi::c_int as uint16_t,
        XCB_COPY_FROM_PARENT as xcb_visualid_t,
        (XCB_CW_BACK_PIXEL as ::core::ffi::c_int | XCB_CW_OVERRIDE_REDIRECT as ::core::ffi::c_int)
            as uint32_t,
        &raw mut fresh0 as *mut uint32_t as *const ::core::ffi::c_void,
    );
    xcb_map_window(conn, window);
    xcb_clear_area(
        conn,
        0 as uint8_t,
        window,
        0 as int16_t,
        0 as int16_t,
        0 as uint16_t,
        0 as uint16_t,
    );
    xcb_aux_sync(conn);
    xcb_destroy_window(conn, window);
    let mut img: *mut xcb_get_image_reply_t = xcb_get_image_reply(
        conn,
        xcb_get_image_unchecked(
            conn,
            XCB_IMAGE_FORMAT_Z_PIXMAP as ::core::ffi::c_int as uint8_t,
            (*screen).root as xcb_drawable_t,
            x,
            y,
            1 as uint16_t,
            1 as uint16_t,
            !(0 as ::core::ffi::c_int) as uint32_t,
        ),
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    let mut result: uint32_t = 0 as uint32_t;
    if !img.is_null() {
        let mut data: *mut uint8_t = xcb_get_image_data(img);
        let mut depth: uint8_t = (*img).depth;
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i
            < (if (depth as ::core::ffi::c_int) < 4 as ::core::ffi::c_int {
                depth as ::core::ffi::c_int
            } else {
                4 as ::core::ffi::c_int
            })
        {
            result = result << 8 as ::core::ffi::c_int | *data.offset(i as isize) as uint32_t;
            i += 1;
        }
        free(img as *mut ::core::ffi::c_void);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn is_background_set(
    mut conn: *mut xcb_connection_t,
    mut screen: *mut xcb_screen_t,
) -> bool {
    let mut x: uint16_t = 0;
    let mut y: uint16_t = 0;
    find_unobscured_pixel(
        conn,
        (*screen).root,
        (*screen).width_in_pixels,
        (*screen).height_in_pixels,
        &raw mut x,
        &raw mut y,
    );
    let mut window: xcb_window_t = xcb_generate_id(conn) as xcb_window_t;
    let mut pixel1: uint32_t = flicker_window_at(
        conn,
        screen,
        x as int16_t,
        y as int16_t,
        window,
        (*screen).black_pixel,
    );
    let mut pixel2: uint32_t = flicker_window_at(
        conn,
        screen,
        x as int16_t,
        y as int16_t,
        window,
        (*screen).white_pixel,
    );
    return pixel1 == pixel2;
}
