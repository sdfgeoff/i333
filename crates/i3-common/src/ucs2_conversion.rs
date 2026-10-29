/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 */
// Mechanical file-by-file Rust translation of libi3/ucs2_conversion.c.
// Generated with C2Rust 0.22.1; C ABI and original ownership retained.
extern "C" {
    fn perror(__s: *const ::core::ffi::c_char);
    fn smalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn scalloc(num: size_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn srealloc(ptr: *mut ::core::ffi::c_void, size: size_t) -> *mut ::core::ffi::c_void;
    fn err(__status: ::core::ffi::c_int, __format: *const ::core::ffi::c_char, ...) -> !;
    fn iconv_open(
        __tocode: *const ::core::ffi::c_char,
        __fromcode: *const ::core::ffi::c_char,
    ) -> iconv_t;
    fn iconv(
        __cd: iconv_t,
        __inbuf: *mut *mut ::core::ffi::c_char,
        __inbytesleft: *mut size_t,
        __outbuf: *mut *mut ::core::ffi::c_char,
        __outbytesleft: *mut size_t,
    ) -> size_t;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type uint8_t = __uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_char2b_t {
    pub byte1: uint8_t,
    pub byte2: uint8_t,
}
pub type iconv_t = *mut ::core::ffi::c_void;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
static mut utf8_conversion_descriptor: iconv_t = unsafe { -(1 as ::core::ffi::c_int) as iconv_t };
static mut ucs2_conversion_descriptor: iconv_t = unsafe { -(1 as ::core::ffi::c_int) as iconv_t };
#[no_mangle]
pub unsafe extern "C" fn convert_ucs2_to_utf8(
    mut text: *mut xcb_char2b_t,
    mut num_glyphs: size_t,
) -> *mut ::core::ffi::c_char {
    let mut buffer_size: size_t = num_glyphs
        .wrapping_mul(4 as size_t)
        .wrapping_add(1 as size_t);
    let mut buffer: *mut ::core::ffi::c_char =
        scalloc(buffer_size, 1 as size_t) as *mut ::core::ffi::c_char;
    let mut output: *mut ::core::ffi::c_char = buffer;
    let mut output_size: size_t = buffer_size.wrapping_sub(1 as size_t);
    if utf8_conversion_descriptor == -(1 as ::core::ffi::c_int) as iconv_t {
        utf8_conversion_descriptor = iconv_open(
            b"UTF-8\0" as *const u8 as *const ::core::ffi::c_char,
            b"UCS-2BE\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if utf8_conversion_descriptor == -(1 as ::core::ffi::c_int) as iconv_t {
            err(
                EXIT_FAILURE,
                b"Error opening the conversion context\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    } else {
        iconv(
            utf8_conversion_descriptor,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            ::core::ptr::null_mut::<size_t>(),
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            ::core::ptr::null_mut::<size_t>(),
        );
    }
    let mut input_len: size_t =
        num_glyphs.wrapping_mul(::core::mem::size_of::<xcb_char2b_t>() as size_t);
    let mut rc: size_t = iconv(
        utf8_conversion_descriptor,
        &raw mut text as *mut *mut ::core::ffi::c_char,
        &raw mut input_len,
        &raw mut output,
        &raw mut output_size,
    );
    if rc == -(1 as ::core::ffi::c_int) as size_t {
        perror(b"Converting to UTF-8 failed\0" as *const u8 as *const ::core::ffi::c_char);
        free(buffer as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    return buffer;
}
#[no_mangle]
pub unsafe extern "C" fn convert_utf8_to_ucs2(
    mut input: *mut ::core::ffi::c_char,
    mut real_strlen: *mut size_t,
) -> *mut xcb_char2b_t {
    let mut input_size: size_t = strlen(input);
    let mut buffer_size: size_t =
        input_size.wrapping_mul(::core::mem::size_of::<xcb_char2b_t>() as size_t);
    let mut buffer: *mut xcb_char2b_t = smalloc(buffer_size) as *mut xcb_char2b_t;
    let mut output_bytes_left: size_t = buffer_size;
    let mut output: *mut xcb_char2b_t = buffer;
    if ucs2_conversion_descriptor == -(1 as ::core::ffi::c_int) as iconv_t {
        ucs2_conversion_descriptor = iconv_open(
            b"UCS-2BE//IGNORE\0" as *const u8 as *const ::core::ffi::c_char,
            b"UTF-8\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if ucs2_conversion_descriptor == -(1 as ::core::ffi::c_int) as iconv_t {
            ucs2_conversion_descriptor = iconv_open(
                b"UCS-2BE\0" as *const u8 as *const ::core::ffi::c_char,
                b"UTF-8\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        if ucs2_conversion_descriptor == -(1 as ::core::ffi::c_int) as iconv_t {
            err(
                EXIT_FAILURE,
                b"Error opening the conversion context\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    } else {
        iconv(
            ucs2_conversion_descriptor,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            ::core::ptr::null_mut::<size_t>(),
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            ::core::ptr::null_mut::<size_t>(),
        );
    }
    let mut rc: size_t = iconv(
        ucs2_conversion_descriptor,
        &raw mut input,
        &raw mut input_size,
        &raw mut output as *mut *mut ::core::ffi::c_char,
        &raw mut output_bytes_left,
    );
    if rc == -(1 as ::core::ffi::c_int) as size_t {
        perror(b"Converting to UCS-2 failed\0" as *const u8 as *const ::core::ffi::c_char);
    }
    buffer_size = buffer_size.wrapping_sub(output_bytes_left);
    buffer = srealloc(buffer as *mut ::core::ffi::c_void, buffer_size) as *mut xcb_char2b_t;
    if !real_strlen.is_null() {
        *real_strlen = buffer_size.wrapping_div(::core::mem::size_of::<xcb_char2b_t>() as size_t);
    }
    return buffer;
}
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
