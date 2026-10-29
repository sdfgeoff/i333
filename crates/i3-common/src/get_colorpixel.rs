/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 */
// Mechanical file-by-file Rust translation of libi3/get_colorpixel.c.
// Originally translated with C2Rust 0.22.1; subsequently maintained in Rust.

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Colorpixel {
    pub hex: [::core::ffi::c_char; 8],
    pub pixel: u32,
    pub colorpixels: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub sle_next: *mut Colorpixel,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct colorpixel_head {
    pub slh_first: *mut Colorpixel,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();

pub static mut colorpixels: colorpixel_head = colorpixel_head {
    slh_first: ::core::ptr::null::<Colorpixel>() as *mut Colorpixel,
};

pub unsafe fn get_colorpixel(hex: *const ::core::ffi::c_char) -> u32 {
    let crate::display::DisplayContext {
        connection: conn,
        root_screen,
        ..
    } = crate::display::context();

    let mut alpha: [::core::ffi::c_char; 2] = [0; 2];
    if strlen(hex) == strlen(c"#rrggbbaa".as_ptr()) {
        alpha[0 as ::core::ffi::c_int as usize] = *hex.offset(7 as ::core::ffi::c_int as isize);
        alpha[1 as ::core::ffi::c_int as usize] = *hex.offset(8 as ::core::ffi::c_int as isize);
    } else {
        alpha[1 as ::core::ffi::c_int as usize] = 'F' as i32 as ::core::ffi::c_char;
        alpha[0 as ::core::ffi::c_int as usize] = alpha[1 as ::core::ffi::c_int as usize];
    }
    let mut strgroups: [[::core::ffi::c_char; 3]; 4] = [
        [
            *hex.offset(1 as ::core::ffi::c_int as isize),
            *hex.offset(2 as ::core::ffi::c_int as isize),
            '\0' as i32 as ::core::ffi::c_char,
        ],
        [
            *hex.offset(3 as ::core::ffi::c_int as isize),
            *hex.offset(4 as ::core::ffi::c_int as isize),
            '\0' as i32 as ::core::ffi::c_char,
        ],
        [
            *hex.offset(5 as ::core::ffi::c_int as isize),
            *hex.offset(6 as ::core::ffi::c_int as isize),
            '\0' as i32 as ::core::ffi::c_char,
        ],
        [
            alpha[0 as ::core::ffi::c_int as usize],
            alpha[1 as ::core::ffi::c_int as usize],
            '\0' as i32 as ::core::ffi::c_char,
        ],
    ];
    let r: u8 = strtol(
        &raw mut *(&raw mut strgroups as *mut [::core::ffi::c_char; 3])
            .offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        16 as ::core::ffi::c_int,
    ) as u8;
    let g: u8 = strtol(
        &raw mut *(&raw mut strgroups as *mut [::core::ffi::c_char; 3])
            .offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        16 as ::core::ffi::c_int,
    ) as u8;
    let b: u8 = strtol(
        &raw mut *(&raw mut strgroups as *mut [::core::ffi::c_char; 3])
            .offset(2 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        16 as ::core::ffi::c_int,
    ) as u8;
    let a: u8 = strtol(
        &raw mut *(&raw mut strgroups as *mut [::core::ffi::c_char; 3])
            .offset(3 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        16 as ::core::ffi::c_int,
    ) as u8;
    if root_screen.is_null()
        || (*root_screen).root_depth as ::core::ffi::c_int == 24 as ::core::ffi::c_int
        || (*root_screen).root_depth as ::core::ffi::c_int == 32 as ::core::ffi::c_int
    {
        return (a as u32) << 24 as ::core::ffi::c_int
            | (r as u32) << 16 as ::core::ffi::c_int
            | (g as u32) << 8 as ::core::ffi::c_int
            | b as u32;
    }
    let mut colorpixel: *mut Colorpixel = ::core::ptr::null_mut::<Colorpixel>();
    colorpixel = colorpixels.slh_first;
    while !colorpixel.is_null() {
        if strcmp(&raw mut (*colorpixel).hex as *mut ::core::ffi::c_char, hex)
            == 0 as ::core::ffi::c_int
        {
            return (*colorpixel).pixel;
        }
        colorpixel = (*colorpixel).colorpixels.sle_next;
    }
    let r16: ::core::ffi::c_int = 65535 as ::core::ffi::c_int
        * (r as ::core::ffi::c_int & 0xff as ::core::ffi::c_int)
        / 255 as ::core::ffi::c_int;
    let g16: ::core::ffi::c_int = 65535 as ::core::ffi::c_int
        * (g as ::core::ffi::c_int & 0xff as ::core::ffi::c_int)
        / 255 as ::core::ffi::c_int;
    let b16: ::core::ffi::c_int = 65535 as ::core::ffi::c_int
        * (b as ::core::ffi::c_int & 0xff as ::core::ffi::c_int)
        / 255 as ::core::ffi::c_int;
    let mut reply: *mut xcb_alloc_color_reply_t =
        ::core::ptr::null_mut::<xcb_alloc_color_reply_t>();
    reply = xcb_alloc_color_reply(
        conn,
        xcb_alloc_color(
            conn,
            (*root_screen).default_colormap,
            r16 as u16,
            g16 as u16,
            b16 as u16,
        ),
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply.is_null() {
        verboselog(
            c"[libi3] ../libi3/get_colorpixel.c Could not allocate color\n"
                .as_ptr()
                .cast_mut(),
        );
        exit(1 as ::core::ffi::c_int);
    }
    let pixel: u32 = (*reply).pixel;
    free(reply as *mut ::core::ffi::c_void);
    let cache_pixel: *mut Colorpixel =
        scalloc(1 as usize, ::core::mem::size_of::<Colorpixel>() as usize) as *mut Colorpixel;
    strncpy(
        &raw mut (*cache_pixel).hex as *mut ::core::ffi::c_char,
        hex,
        7 as usize,
    );
    (*cache_pixel).hex[7 as ::core::ffi::c_int as usize] = '\0' as i32 as ::core::ffi::c_char;
    (*cache_pixel).pixel = pixel;
    (*cache_pixel).colorpixels.sle_next = colorpixels.slh_first;
    colorpixels.slh_first = cache_pixel;
    pixel
}

// Internal helpers use Rust linkage rather than foreign symbol lookup.
use crate::safewrappers::scalloc;

use xcb_sys::{
    xcb_alloc_color, xcb_alloc_color_reply, xcb_alloc_color_reply_t, xcb_generic_error_t,
};

pub use libc::{exit, free, strcmp, strlen, strncpy, strtol};

use crate::logging::verboselog;
