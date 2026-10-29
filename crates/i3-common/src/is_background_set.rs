/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 */
// Mechanical file-by-file Rust translation of libi3/is_background_set.c.
// Originally translated with C2Rust 0.22.1; subsequently maintained in Rust.
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type int16_t = __int16_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;

pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
unsafe fn unobscured_region(
    conn: *mut xcb_connection_t,
    window: xcb_window_t,
    window_width: uint16_t,
    window_height: uint16_t,
) -> *mut cairo_region_t {
    let mut rectangle: cairo_rectangle_int_t = cairo_rectangle_int_t {
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
    let tree: *mut xcb_query_tree_reply_t = xcb_query_tree_reply(
        conn,
        xcb_query_tree_unchecked(conn, window),
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if tree.is_null() {
        return region;
    }
    let n_children: uint16_t = (*tree).children_len;
    let children: *mut xcb_window_t = xcb_query_tree_children(tree);
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
        let geom: *mut xcb_get_geometry_reply_t = xcb_get_geometry_reply(
            conn,
            *geometries.as_mut_ptr().offset(i_0 as isize),
            ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
        );
        let attr: *mut xcb_get_window_attributes_reply_t = xcb_get_window_attributes_reply(
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
    region
}
unsafe fn find_unobscured_pixel(
    conn: *mut xcb_connection_t,
    window: xcb_window_t,
    window_width: uint16_t,
    window_height: uint16_t,
    x: *mut uint16_t,
    y: *mut uint16_t,
) {
    let region: *mut cairo_region_t = unobscured_region(conn, window, window_width, window_height);
    if cairo_region_num_rectangles(region) > 0 as ::core::ffi::c_int {
        let mut rect: cairo_rectangle_int_t = cairo_rectangle_int_t {
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
unsafe fn flicker_window_at(
    conn: *mut xcb_connection_t,
    screen: *mut xcb_screen_t,
    x: int16_t,
    y: int16_t,
    window: xcb_window_t,
    pixel: uint32_t,
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
    let img: *mut xcb_get_image_reply_t = xcb_get_image_reply(
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
        let data: *mut uint8_t = xcb_get_image_data(img);
        let depth: uint8_t = (*img).depth;
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
    result
}

pub unsafe fn is_background_set(conn: *mut xcb_connection_t, screen: *mut xcb_screen_t) -> bool {
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
    let window: xcb_window_t = xcb_generate_id(conn) as xcb_window_t;
    let pixel1: uint32_t = flicker_window_at(
        conn,
        screen,
        x as int16_t,
        y as int16_t,
        window,
        (*screen).black_pixel,
    );
    let pixel2: uint32_t = flicker_window_at(
        conn,
        screen,
        x as int16_t,
        y as int16_t,
        window,
        (*screen).white_pixel,
    );
    pixel1 == pixel2
}

// Canonical Cairo backend ABI; renderer logic remains local.
pub use cairo_sys::{
    cairo_rectangle_int_t, cairo_region_create_rectangle, cairo_region_destroy,
    cairo_region_get_rectangle, cairo_region_num_rectangles, cairo_region_subtract_rectangle,
    cairo_region_t,
};

use xcb_sys::{
    xcb_aux_sync, xcb_clear_area, xcb_connection_t, xcb_create_window, xcb_destroy_window,
    xcb_drawable_t, xcb_generate_id, xcb_generic_error_t, xcb_get_geometry_cookie_t,
    xcb_get_geometry_reply, xcb_get_geometry_reply_t, xcb_get_geometry_unchecked,
    xcb_get_image_data, xcb_get_image_reply, xcb_get_image_reply_t, xcb_get_image_unchecked,
    xcb_get_window_attributes_cookie_t, xcb_get_window_attributes_reply,
    xcb_get_window_attributes_reply_t, xcb_get_window_attributes_unchecked, xcb_map_window,
    xcb_query_tree_children, xcb_query_tree_reply, xcb_query_tree_reply_t,
    xcb_query_tree_unchecked, xcb_screen_t, xcb_visualid_t, xcb_window_t, XCB_COPY_FROM_PARENT,
    XCB_CW_BACK_PIXEL, XCB_CW_OVERRIDE_REDIRECT, XCB_IMAGE_FORMAT_Z_PIXMAP, XCB_MAP_STATE_VIEWABLE,
    XCB_WINDOW_CLASS_INPUT_OUTPUT,
};

pub use libc::free;
