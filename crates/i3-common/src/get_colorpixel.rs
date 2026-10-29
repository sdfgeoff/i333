/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 */
// Mechanical file-by-file Rust translation of libi3/get_colorpixel.c.
// Generated with C2Rust 0.22.1; C ABI and original ownership retained.
extern "C" {
    pub type xcb_connection_t;
    fn xcb_alloc_color(
        c: *mut xcb_connection_t,
        cmap: xcb_colormap_t,
        red: uint16_t,
        green: uint16_t,
        blue: uint16_t,
    ) -> xcb_alloc_color_cookie_t;
    fn xcb_alloc_color_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_alloc_color_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_alloc_color_reply_t;
    static mut conn: *mut xcb_connection_t;
    static mut root_screen: *mut xcb_screen_t;
    fn verboselog(fmt: *mut ::core::ffi::c_char, ...);
    fn scalloc(num: size_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn strncpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn exit(__status: ::core::ffi::c_int) -> !;
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
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
pub type xcb_window_t = uint32_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_alloc_color_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_alloc_color_reply_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub red: uint16_t,
    pub green: uint16_t,
    pub blue: uint16_t,
    pub pad1: [uint8_t; 2],
    pub pixel: uint32_t,
}
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
#[no_mangle]
pub static mut colorpixels: colorpixel_head = colorpixel_head {
    slh_first: ::core::ptr::null::<Colorpixel>() as *mut Colorpixel,
};
#[no_mangle]
pub unsafe extern "C" fn get_colorpixel(mut hex: *const ::core::ffi::c_char) -> uint32_t {
    let mut alpha: [::core::ffi::c_char; 2] = [0; 2];
    if strlen(hex) == strlen(b"#rrggbbaa\0" as *const u8 as *const ::core::ffi::c_char) {
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
    let mut r: uint8_t = strtol(
        &raw mut *(&raw mut strgroups as *mut [::core::ffi::c_char; 3])
            .offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        16 as ::core::ffi::c_int,
    ) as uint8_t;
    let mut g: uint8_t = strtol(
        &raw mut *(&raw mut strgroups as *mut [::core::ffi::c_char; 3])
            .offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        16 as ::core::ffi::c_int,
    ) as uint8_t;
    let mut b: uint8_t = strtol(
        &raw mut *(&raw mut strgroups as *mut [::core::ffi::c_char; 3])
            .offset(2 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        16 as ::core::ffi::c_int,
    ) as uint8_t;
    let mut a: uint8_t = strtol(
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
    let mut r16: ::core::ffi::c_int = 65535 as ::core::ffi::c_int
        * (r as ::core::ffi::c_int & 0xff as ::core::ffi::c_int)
        / 255 as ::core::ffi::c_int;
    let mut g16: ::core::ffi::c_int = 65535 as ::core::ffi::c_int
        * (g as ::core::ffi::c_int & 0xff as ::core::ffi::c_int)
        / 255 as ::core::ffi::c_int;
    let mut b16: ::core::ffi::c_int = 65535 as ::core::ffi::c_int
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
            b"[libi3] ../libi3/get_colorpixel.c Could not allocate color\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    }
    let mut pixel: uint32_t = (*reply).pixel;
    free(reply as *mut ::core::ffi::c_void);
    let mut cache_pixel: *mut Colorpixel =
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
    return pixel;
}
