/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 */
// Mechanical file-by-file Rust translation of libi3/get_visualtype.c.
// Originally translated with C2Rust 0.22.1; subsequently maintained in Rust.
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;

pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();

pub unsafe fn get_visualtype(screen: *mut xcb_screen_t) -> *mut xcb_visualtype_t {
    let mut depth_iter: xcb_depth_iterator_t = xcb_depth_iterator_t {
        data: ::core::ptr::null_mut::<xcb_depth_t>(),
        rem: 0,
        index: 0,
    };
    depth_iter = xcb_screen_allowed_depths_iterator(screen);
    while depth_iter.rem != 0 {
        let mut visual_iter: xcb_visualtype_iterator_t = xcb_visualtype_iterator_t {
            data: ::core::ptr::null_mut::<xcb_visualtype_t>(),
            rem: 0,
            index: 0,
        };
        visual_iter = xcb_depth_visuals_iterator(depth_iter.data);
        while visual_iter.rem != 0 {
            if (*screen).root_visual == (*visual_iter.data).visual_id {
                return visual_iter.data;
            }
            xcb_visualtype_next(&raw mut visual_iter);
        }
        xcb_depth_next(&raw mut depth_iter);
    }
    ::core::ptr::null_mut::<xcb_visualtype_t>()
}

use xcb_sys::{
    xcb_depth_iterator_t, xcb_depth_next, xcb_depth_t, xcb_depth_visuals_iterator,
    xcb_screen_allowed_depths_iterator, xcb_screen_t, xcb_visualtype_iterator_t,
    xcb_visualtype_next, xcb_visualtype_t,
};
