/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * string.c: Define an i3String type to automagically handle UTF-8/UCS-2
 *           conversions. Some font backends need UCS-2 (X core fonts),
 *           others want UTF-8 (Pango).
 *
 */
// Mechanical file-by-file Rust translation of libi3/string.c.
// Generated with C2Rust 0.22.1; C ABI and original ownership retained.
extern "C" {
    fn g_utf8_make_valid(str: *const gchar, len: gssize) -> *mut gchar;
    fn g_markup_escape_text(text: *const gchar, length: gssize) -> *mut gchar;
    fn scalloc(num: size_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn convert_ucs2_to_utf8(
        text: *mut xcb_char2b_t,
        num_glyphs: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn convert_utf8_to_ucs2(
        input: *mut ::core::ffi::c_char,
        real_strlen: *mut size_t,
    ) -> *mut xcb_char2b_t;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type ssize_t = isize;
pub type uint8_t = __uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_char2b_t {
    pub byte1: uint8_t,
    pub byte2: uint8_t,
}
pub type gssize = ::core::ffi::c_long;
pub type gchar = ::core::ffi::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _i3String {
    pub utf8: *mut ::core::ffi::c_char,
    pub ucs2: *mut xcb_char2b_t,
    pub num_glyphs: size_t,
    pub num_bytes: size_t,
    pub pango_markup: bool,
}
pub type i3String = _i3String;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[no_mangle]
pub unsafe extern "C" fn i3string_from_utf8(
    mut from_utf8: *const ::core::ffi::c_char,
) -> *mut i3String {
    return i3string_from_utf8_with_length(from_utf8, -(1 as ::core::ffi::c_int) as ssize_t);
}
#[no_mangle]
pub unsafe extern "C" fn i3string_from_markup(
    mut from_markup: *const ::core::ffi::c_char,
) -> *mut i3String {
    let mut str: *mut i3String = i3string_from_utf8(from_markup);
    (*str).pango_markup = true_0 != 0;
    return str;
}
#[no_mangle]
pub unsafe extern "C" fn i3string_from_utf8_with_length(
    mut from_utf8: *const ::core::ffi::c_char,
    mut num_bytes: ssize_t,
) -> *mut i3String {
    let mut str: *mut i3String =
        scalloc(1 as size_t, ::core::mem::size_of::<i3String>() as size_t) as *mut i3String;
    (*str).utf8 = g_utf8_make_valid(from_utf8 as *const gchar, num_bytes as gssize)
        as *mut ::core::ffi::c_char;
    (*str).num_bytes = if num_bytes < 0 as ssize_t {
        strlen((*str).utf8)
    } else {
        num_bytes as size_t
    };
    return str;
}
#[no_mangle]
pub unsafe extern "C" fn i3string_from_markup_with_length(
    mut from_markup: *const ::core::ffi::c_char,
    mut num_bytes: size_t,
) -> *mut i3String {
    let mut str: *mut i3String = i3string_from_utf8_with_length(from_markup, num_bytes as ssize_t);
    (*str).pango_markup = true_0 != 0;
    return str;
}
#[no_mangle]
pub unsafe extern "C" fn i3string_from_ucs2(
    mut from_ucs2: *const xcb_char2b_t,
    mut num_glyphs: size_t,
) -> *mut i3String {
    let mut str: *mut i3String =
        scalloc(1 as size_t, ::core::mem::size_of::<i3String>() as size_t) as *mut i3String;
    (*str).ucs2 =
        scalloc(num_glyphs, ::core::mem::size_of::<xcb_char2b_t>() as size_t) as *mut xcb_char2b_t;
    memcpy(
        (*str).ucs2 as *mut ::core::ffi::c_void,
        from_ucs2 as *const ::core::ffi::c_void,
        num_glyphs.wrapping_mul(::core::mem::size_of::<xcb_char2b_t>() as size_t),
    );
    (*str).num_glyphs = num_glyphs;
    (*str).utf8 = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*str).num_bytes = 0 as size_t;
    return str;
}
#[no_mangle]
pub unsafe extern "C" fn i3string_copy(mut str: *mut i3String) -> *mut i3String {
    let mut copy: *mut i3String = i3string_from_utf8(i3string_as_utf8(str));
    (*copy).pango_markup = (*str).pango_markup;
    return copy;
}
#[no_mangle]
pub unsafe extern "C" fn i3string_free(mut str: *mut i3String) {
    if str.is_null() {
        return;
    }
    free((*str).utf8 as *mut ::core::ffi::c_void);
    free((*str).ucs2 as *mut ::core::ffi::c_void);
    free(str as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn i3string_ensure_utf8(mut str: *mut i3String) {
    if !(*str).utf8.is_null() {
        return;
    }
    (*str).utf8 = convert_ucs2_to_utf8((*str).ucs2, (*str).num_glyphs);
    if !(*str).utf8.is_null() {
        (*str).num_bytes = strlen((*str).utf8);
    }
}
unsafe extern "C" fn i3string_ensure_ucs2(mut str: *mut i3String) {
    if !(*str).ucs2.is_null() {
        return;
    }
    (*str).ucs2 = convert_utf8_to_ucs2((*str).utf8, &raw mut (*str).num_glyphs);
}
#[no_mangle]
pub unsafe extern "C" fn i3string_as_utf8(mut str: *mut i3String) -> *const ::core::ffi::c_char {
    i3string_ensure_utf8(str);
    return (*str).utf8;
}
#[no_mangle]
pub unsafe extern "C" fn i3string_as_ucs2(mut str: *mut i3String) -> *const xcb_char2b_t {
    i3string_ensure_ucs2(str);
    return (*str).ucs2;
}
#[no_mangle]
pub unsafe extern "C" fn i3string_get_num_bytes(mut str: *mut i3String) -> size_t {
    i3string_ensure_utf8(str);
    return (*str).num_bytes;
}
#[no_mangle]
pub unsafe extern "C" fn i3string_is_markup(mut str: *mut i3String) -> bool {
    return (*str).pango_markup;
}
#[no_mangle]
pub unsafe extern "C" fn i3string_set_markup(mut str: *mut i3String, mut pango_markup: bool) {
    (*str).pango_markup = pango_markup;
}
#[no_mangle]
pub unsafe extern "C" fn i3string_escape_markup(mut str: *mut i3String) -> *mut i3String {
    let mut text: *const ::core::ffi::c_char = i3string_as_utf8(str);
    let mut escaped: *mut ::core::ffi::c_char =
        g_markup_escape_text(text as *const gchar, -(1 as ::core::ffi::c_int) as gssize)
            as *mut ::core::ffi::c_char;
    let mut result: *mut i3String = i3string_from_utf8(escaped);
    free(escaped as *mut ::core::ffi::c_void);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn i3string_get_num_glyphs(mut str: *mut i3String) -> size_t {
    i3string_ensure_ucs2(str);
    return (*str).num_glyphs;
}
