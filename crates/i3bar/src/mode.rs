use crate::config::config;
use crate::xcb::set_current_mode;
use i3_common::font::predict_text_width;
use i3_common::safewrappers::sasprintf;
use i3_common::string::i3string_as_utf8;
use i3_common::string::i3string_free;
use i3_common::string::i3string_from_utf8;
use i3_common::string::i3string_set_markup;
/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3bar - an xcb-based status- and ws-bar for i3
 * © 2010 Axel Wagner and contributors (see also: LICENSE)
 *
 * mode.c: Handle mode event and show current binding mode in the bar
 *
 */
// Mechanical file-by-file Rust translation of i3bar/src/mode.c.
// Originally translated with C2Rust 0.22.1; subsequently maintained in Rust.
use i3_json::compat::{yajl_alloc, yajl_free, yajl_parse};
pub use i3_json::compat::{yajl_alloc_funcs, yajl_callbacks, yajl_handle, yajl_status};
pub type size_t = usize;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;

pub type uint32_t = __uint32_t;
pub type i3String = _i3String;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mode {
    pub name: *mut i3String,
    pub name_width: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mode_json_params {
    pub cur_key: *mut ::core::ffi::c_char,
    pub name: *mut ::core::ffi::c_char,
    pub pango_markup: bool,
    pub mode: *mut mode,
}

pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const S_SHOW: C2RustUnnamed = 1;
pub const S_HIDE: C2RustUnnamed = 0;
pub type bar_display_mode_t = ::core::ffi::c_uint;
pub const M_INVISIBLE: bar_display_mode_t = 2;
pub const M_HIDE: bar_display_mode_t = 1;
pub const M_DOCK: bar_display_mode_t = 0;

pub type position_t = ::core::ffi::c_uint;
pub const POS_BOT: position_t = 2;
pub const POS_TOP: position_t = 1;
pub const POS_NONE: position_t = 0;

pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
unsafe fn mode_string_cb(
    params_: *mut ::core::ffi::c_void,
    val: *const ::core::ffi::c_uchar,
    len: size_t,
) -> ::core::ffi::c_int {
    let params: *mut mode_json_params = params_ as *mut mode_json_params;
    if strcmp((*params).cur_key, c"change".as_ptr()) == 0 {
        sasprintf(
            &raw mut (*params).name,
            c"%.*s".as_ptr(),
            len as ::core::ffi::c_int,
            val,
        );
        free((*params).cur_key as *mut ::core::ffi::c_void);
        (*params).cur_key = ::core::ptr::null_mut::<::core::ffi::c_char>();
        return 1 as ::core::ffi::c_int;
    }
    free((*params).cur_key as *mut ::core::ffi::c_void);
    (*params).cur_key = ::core::ptr::null_mut::<::core::ffi::c_char>();
    0 as ::core::ffi::c_int
}
unsafe fn mode_boolean_cb(
    params_: *mut ::core::ffi::c_void,
    val: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let params: *mut mode_json_params = params_ as *mut mode_json_params;
    if strcmp((*params).cur_key, c"pango_markup".as_ptr()) == 0 as ::core::ffi::c_int {
        if config.verbose {
            printf(
                c"[%s:%d] Setting pango_markup to %d.\n".as_ptr(),
                c"../i3bar/src/mode.c".as_ptr(),
                50 as ::core::ffi::c_int,
                val,
            );
        }
        (*params).pango_markup = val != 0;
        free((*params).cur_key as *mut ::core::ffi::c_void);
        (*params).cur_key = ::core::ptr::null_mut::<::core::ffi::c_char>();
        return 1 as ::core::ffi::c_int;
    }
    free((*params).cur_key as *mut ::core::ffi::c_void);
    (*params).cur_key = ::core::ptr::null_mut::<::core::ffi::c_char>();
    0 as ::core::ffi::c_int
}
unsafe fn mode_map_key_cb(
    params_: *mut ::core::ffi::c_void,
    keyVal: *const ::core::ffi::c_uchar,
    keyLen: size_t,
) -> ::core::ffi::c_int {
    let params: *mut mode_json_params = params_ as *mut mode_json_params;
    free((*params).cur_key as *mut ::core::ffi::c_void);
    (*params).cur_key = ::core::ptr::null_mut::<::core::ffi::c_char>();
    sasprintf(
        &raw mut (*params).cur_key,
        c"%.*s".as_ptr(),
        keyLen as ::core::ffi::c_int,
        keyVal,
    );
    1 as ::core::ffi::c_int
}
unsafe fn mode_end_map_cb(params_: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    let params: *mut mode_json_params = params_ as *mut mode_json_params;
    (*(*params).mode).name = i3string_from_utf8((*params).name);
    i3string_set_markup((*(*params).mode).name, (*params).pango_markup);
    (*(*params).mode).name_width = predict_text_width((*(*params).mode).name);
    if config.verbose {
        printf(
            c"[%s:%d] Got mode change: %s\n".as_ptr(),
            c"../i3bar/src/mode.c".as_ptr(),
            83 as ::core::ffi::c_int,
            i3string_as_utf8((*(*params).mode).name),
        );
    }
    free((*params).cur_key as *mut ::core::ffi::c_void);
    (*params).cur_key = ::core::ptr::null_mut::<::core::ffi::c_char>();
    1 as ::core::ffi::c_int
}
static mut mode_callbacks: yajl_callbacks = unsafe {
    yajl_callbacks {
        yajl_null: None,
        yajl_boolean: Some(
            mode_boolean_cb
                as unsafe fn(*mut ::core::ffi::c_void, ::core::ffi::c_int) -> ::core::ffi::c_int,
        ),
        yajl_integer: None,
        yajl_double: None,
        yajl_number: None,
        yajl_string: Some(
            mode_string_cb
                as unsafe fn(
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_uchar,
                    size_t,
                ) -> ::core::ffi::c_int,
        ),
        yajl_start_map: None,
        yajl_map_key: Some(
            mode_map_key_cb
                as unsafe fn(
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_uchar,
                    size_t,
                ) -> ::core::ffi::c_int,
        ),
        yajl_end_map: Some(
            mode_end_map_cb as unsafe fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
        ),
        yajl_start_array: None,
        yajl_end_array: None,
    }
};

pub unsafe fn parse_mode_json(json: *const ::core::ffi::c_uchar, size: size_t) {
    let mut params: mode_json_params = mode_json_params {
        cur_key: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        pango_markup: false,
        mode: ::core::ptr::null_mut::<mode>(),
    };
    let mut binding: mode = mode {
        name: ::core::ptr::null_mut::<i3String>(),
        name_width: 0,
    };
    params.cur_key = ::core::ptr::null_mut::<::core::ffi::c_char>();
    params.mode = &raw mut binding;
    let handle: yajl_handle = yajl_alloc(
        &raw mut mode_callbacks,
        ::core::ptr::null_mut::<yajl_alloc_funcs>(),
        &raw mut params as *mut ::core::ffi::c_void,
    );
    let state: yajl_status = yajl_parse(handle, json, size);
    match state as ::core::ffi::c_uint {
        1 | 2 => {
            fprintf(
                stderr,
                c"[%s:%d] ERROR: Could not parse mode event!\n".as_ptr(),
                c"../i3bar/src/mode.c".as_ptr(),
                116 as ::core::ffi::c_int,
            );
            exit(EXIT_FAILURE);
        }
        0 | _ => {}
    }
    if strcmp(c"default".as_ptr(), i3string_as_utf8((*params.mode).name)) == 0 as ::core::ffi::c_int
    {
        i3string_free((*params.mode).name);
        (*params.mode).name = ::core::ptr::null_mut::<i3String>();
    }
    set_current_mode(&raw mut binding);
    yajl_free(handle);
    free(params.cur_key as *mut ::core::ffi::c_void);
    params.cur_key = ::core::ptr::null_mut::<::core::ffi::c_char>();
}
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub use libc::{exit, fprintf, free, printf, strcmp};

use i3_common::native_stdio::stderr;

pub use i3_common::string::I3String as _i3String;
