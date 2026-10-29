/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 */
// Mechanical file-by-file Rust translation of libi3/get_colorpixel.c.
// Originally translated with C2Rust 0.22.1; subsequently maintained in Rust.
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Colorpixel {
    pub hex: [::core::ffi::c_char; 8],
    pub pixel: uint32_t,
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

pub unsafe fn get_colorpixel(hex: *const ::core::ffi::c_char) -> uint32_t {
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
    let r: uint8_t = strtol(
        &raw mut *(&raw mut strgroups as *mut [::core::ffi::c_char; 3])
            .offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        16 as ::core::ffi::c_int,
    ) as uint8_t;
    let g: uint8_t = strtol(
        &raw mut *(&raw mut strgroups as *mut [::core::ffi::c_char; 3])
            .offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        16 as ::core::ffi::c_int,
    ) as uint8_t;
    let b: uint8_t = strtol(
        &raw mut *(&raw mut strgroups as *mut [::core::ffi::c_char; 3])
            .offset(2 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        16 as ::core::ffi::c_int,
    ) as uint8_t;
    let a: uint8_t = strtol(
        &raw mut *(&raw mut strgroups as *mut [::core::ffi::c_char; 3])
            .offset(3 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        16 as ::core::ffi::c_int,
    ) as uint8_t;
    if root_screen.is_null()
        || (*root_screen).root_depth as ::core::ffi::c_int == 24 as ::core::ffi::c_int
        || (*root_screen).root_depth as ::core::ffi::c_int == 32 as ::core::ffi::c_int
    {
        return (a as uint32_t) << 24 as ::core::ffi::c_int
            | (r as uint32_t) << 16 as ::core::ffi::c_int
            | (g as uint32_t) << 8 as ::core::ffi::c_int
            | b as uint32_t;
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
            r16 as uint16_t,
            g16 as uint16_t,
            b16 as uint16_t,
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
    let pixel: uint32_t = (*reply).pixel;
    free(reply as *mut ::core::ffi::c_void);
    let cache_pixel: *mut Colorpixel =
        scalloc(1 as size_t, ::core::mem::size_of::<Colorpixel>() as size_t) as *mut Colorpixel;
    strncpy(
        &raw mut (*cache_pixel).hex as *mut ::core::ffi::c_char,
        hex,
        7 as size_t,
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
