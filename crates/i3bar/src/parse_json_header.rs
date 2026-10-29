/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3bar - an xcb-based status- and ws-bar for i3
 * © 2010 Axel Wagner and contributors (see also: LICENSE)
 *
 * parse_json_header.c: Parse the JSON protocol header to determine
 *                      protocol version and features.
 *
 */
// Mechanical file-by-file Rust translation of i3bar/src/parse_json_header.c.
// Originally translated with C2Rust 0.22.1; subsequently maintained in Rust.
use i3_json::compat::{yajl_alloc, yajl_config, yajl_free, yajl_get_bytes_consumed, yajl_parse};
pub use i3_json::compat::{
    yajl_alloc_funcs, yajl_allow_trailing_garbage, yajl_callbacks, yajl_handle, yajl_status,
    yajl_status_ok,
};
pub type size_t = usize;
pub type __uint32_t = u32;
pub type __pid_t = ::core::ffi::c_int;
pub type pid_t = __pid_t;
pub type uint32_t = __uint32_t;

pub use crate::child::i3bar_child;

pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const NO_KEY: C2RustUnnamed = 4;
pub const KEY_CLICK_EVENTS: C2RustUnnamed = 3;
pub const KEY_CONT_SIGNAL: C2RustUnnamed = 2;
pub const KEY_STOP_SIGNAL: C2RustUnnamed = 1;
pub const KEY_VERSION: C2RustUnnamed = 0;

pub const SIGSTOP: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const SIGCONT: ::core::ffi::c_int = 18 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
static mut current_key: C2RustUnnamed = KEY_VERSION;
unsafe fn header_integer(
    ctx: *mut ::core::ffi::c_void,
    val: ::core::ffi::c_longlong,
) -> ::core::ffi::c_int {
    let child: *mut i3bar_child = ctx as *mut i3bar_child;
    match current_key as ::core::ffi::c_uint {
        0 => {
            (*child).version = val as uint32_t;
        }
        1 => {
            (*child).stop_signal = val as ::core::ffi::c_int;
        }
        2 => {
            (*child).cont_signal = val as ::core::ffi::c_int;
        }
        _ => {}
    }
    1 as ::core::ffi::c_int
}
unsafe fn header_boolean(
    ctx: *mut ::core::ffi::c_void,
    val: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let child: *mut i3bar_child = ctx as *mut i3bar_child;
    match current_key as ::core::ffi::c_uint {
        3 => {
            (*child).click_events = val != 0;
        }
        _ => {}
    }
    1 as ::core::ffi::c_int
}
unsafe fn header_map_key(
    ctx: *mut ::core::ffi::c_void,
    stringval: *const ::core::ffi::c_uchar,
    stringlen: size_t,
) -> ::core::ffi::c_int {
    if stringlen == strlen(c"version".as_ptr())
        && (stringlen >= strlen(c"version".as_ptr())
            && strncasecmp(
                stringval as *const ::core::ffi::c_char,
                c"version".as_ptr(),
                strlen(c"version".as_ptr()),
            ) == 0 as ::core::ffi::c_int)
    {
        current_key = KEY_VERSION;
    } else if stringlen == strlen(c"stop_signal".as_ptr())
        && (stringlen >= strlen(c"stop_signal".as_ptr())
            && strncasecmp(
                stringval as *const ::core::ffi::c_char,
                c"stop_signal".as_ptr(),
                strlen(c"stop_signal".as_ptr()),
            ) == 0 as ::core::ffi::c_int)
    {
        current_key = KEY_STOP_SIGNAL;
    } else if stringlen == strlen(c"cont_signal".as_ptr())
        && (stringlen >= strlen(c"cont_signal".as_ptr())
            && strncasecmp(
                stringval as *const ::core::ffi::c_char,
                c"cont_signal".as_ptr(),
                strlen(c"cont_signal".as_ptr()),
            ) == 0 as ::core::ffi::c_int)
    {
        current_key = KEY_CONT_SIGNAL;
    } else if stringlen == strlen(c"click_events".as_ptr())
        && (stringlen >= strlen(c"click_events".as_ptr())
            && strncasecmp(
                stringval as *const ::core::ffi::c_char,
                c"click_events".as_ptr(),
                strlen(c"click_events".as_ptr()),
            ) == 0 as ::core::ffi::c_int)
    {
        current_key = KEY_CLICK_EVENTS;
    }
    1 as ::core::ffi::c_int
}
unsafe fn child_init(child: *mut i3bar_child) {
    (*child).version = 0 as uint32_t;
    (*child).stop_signal = SIGSTOP;
    (*child).cont_signal = SIGCONT;
}

pub unsafe fn parse_json_header(
    child: *mut i3bar_child,
    buffer: *const ::core::ffi::c_uchar,
    length: ::core::ffi::c_int,
    consumed: *mut ::core::ffi::c_uint,
) {
    static mut version_callbacks: yajl_callbacks = unsafe {
        yajl_callbacks {
            yajl_null: None,
            yajl_boolean: Some(
                header_boolean
                    as unsafe fn(
                        *mut ::core::ffi::c_void,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
            yajl_integer: Some(
                header_integer
                    as unsafe fn(
                        *mut ::core::ffi::c_void,
                        ::core::ffi::c_longlong,
                    ) -> ::core::ffi::c_int,
            ),
            yajl_double: None,
            yajl_number: None,
            yajl_string: None,
            yajl_start_map: None,
            yajl_map_key: Some(
                header_map_key
                    as unsafe fn(
                        *mut ::core::ffi::c_void,
                        *const ::core::ffi::c_uchar,
                        size_t,
                    ) -> ::core::ffi::c_int,
            ),
            yajl_end_map: None,
            yajl_start_array: None,
            yajl_end_array: None,
        }
    };
    child_init(child);
    current_key = NO_KEY;
    let handle: yajl_handle = yajl_alloc(
        &raw mut version_callbacks,
        ::core::ptr::null_mut::<yajl_alloc_funcs>(),
        child as *mut ::core::ffi::c_void,
    );
    yajl_config(handle, yajl_allow_trailing_garbage, 1 as ::core::ffi::c_int);
    let state: yajl_status = yajl_parse(handle, buffer, length as size_t);
    if state as ::core::ffi::c_uint != yajl_status_ok as ::core::ffi::c_int as ::core::ffi::c_uint {
        child_init(child);
        if !consumed.is_null() {
            *consumed = 0 as ::core::ffi::c_uint;
        }
    } else if !consumed.is_null() {
        *consumed = yajl_get_bytes_consumed(handle) as ::core::ffi::c_uint;
    }
    yajl_free(handle);
}

pub use libc::{strlen, strncasecmp};
