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
// Generated with C2Rust 0.22.1; C ABI and original ownership retained.
extern "C" {
    pub type ev_loop;
    pub type yajl_handle_t;
    fn yajl_alloc(
        callbacks: *const yajl_callbacks,
        afs: *mut yajl_alloc_funcs,
        ctx: *mut ::core::ffi::c_void,
    ) -> yajl_handle;
    fn yajl_config(h: yajl_handle, opt: yajl_option, ...) -> ::core::ffi::c_int;
    fn yajl_free(handle: yajl_handle);
    fn yajl_parse(
        hand: yajl_handle,
        jsonText: *const ::core::ffi::c_uchar,
        jsonTextLength: size_t,
    ) -> yajl_status;
    fn yajl_get_bytes_consumed(hand: yajl_handle) -> size_t;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn strncasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
}
pub type size_t = usize;
pub type __uint32_t = u32;
pub type __pid_t = ::core::ffi::c_int;
pub type pid_t = __pid_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ev_watcher_list {
    pub active: ::core::ffi::c_int,
    pub pending: ::core::ffi::c_int,
    pub priority: ::core::ffi::c_int,
    pub data: *mut ::core::ffi::c_void,
    pub cb:
        Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_watcher_list, ::core::ffi::c_int) -> ()>,
    pub next: *mut ev_watcher_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ev_io {
    pub active: ::core::ffi::c_int,
    pub pending: ::core::ffi::c_int,
    pub priority: ::core::ffi::c_int,
    pub data: *mut ::core::ffi::c_void,
    pub cb: Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_io, ::core::ffi::c_int) -> ()>,
    pub next: *mut ev_watcher_list,
    pub fd: ::core::ffi::c_int,
    pub events: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ev_child {
    pub active: ::core::ffi::c_int,
    pub pending: ::core::ffi::c_int,
    pub priority: ::core::ffi::c_int,
    pub data: *mut ::core::ffi::c_void,
    pub cb: Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_child, ::core::ffi::c_int) -> ()>,
    pub next: *mut ev_watcher_list,
    pub flags: ::core::ffi::c_int,
    pub pid: ::core::ffi::c_int,
    pub rpid: ::core::ffi::c_int,
    pub rstatus: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct i3bar_child {
    pub pid: pid_t,
    pub version: uint32_t,
    pub stopped: bool,
    pub stop_signal: ::core::ffi::c_int,
    pub cont_signal: ::core::ffi::c_int,
    pub click_events: bool,
    pub click_events_init: bool,
    pub stdin_io: *mut ev_io,
    pub child_sig: *mut ev_child,
    pub stdin_fd: ::core::ffi::c_int,
    pub pending_line: *mut ::core::ffi::c_char,
}
pub type yajl_handle = *mut yajl_handle_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yajl_alloc_funcs {
    pub malloc: yajl_malloc_func,
    pub realloc: yajl_realloc_func,
    pub free: yajl_free_func,
    pub ctx: *mut ::core::ffi::c_void,
}
pub type yajl_free_func =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> ()>;
pub type yajl_realloc_func = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
        size_t,
    ) -> *mut ::core::ffi::c_void,
>;
pub type yajl_malloc_func =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> *mut ::core::ffi::c_void>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yajl_callbacks {
    pub yajl_null: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
    pub yajl_boolean: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub yajl_integer: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_longlong,
        ) -> ::core::ffi::c_int,
    >,
    pub yajl_double: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, ::core::ffi::c_double) -> ::core::ffi::c_int,
    >,
    pub yajl_number: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_char,
            size_t,
        ) -> ::core::ffi::c_int,
    >,
    pub yajl_string: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_uchar,
            size_t,
        ) -> ::core::ffi::c_int,
    >,
    pub yajl_start_map:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
    pub yajl_map_key: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_uchar,
            size_t,
        ) -> ::core::ffi::c_int,
    >,
    pub yajl_end_map: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
    pub yajl_start_array:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
    pub yajl_end_array:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
}
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const NO_KEY: C2RustUnnamed = 4;
pub const KEY_CLICK_EVENTS: C2RustUnnamed = 3;
pub const KEY_CONT_SIGNAL: C2RustUnnamed = 2;
pub const KEY_STOP_SIGNAL: C2RustUnnamed = 1;
pub const KEY_VERSION: C2RustUnnamed = 0;
pub const yajl_status_ok: yajl_status = 0;
pub type yajl_status = ::core::ffi::c_uint;
pub const yajl_status_error: yajl_status = 2;
pub const yajl_status_client_canceled: yajl_status = 1;
pub type yajl_option = ::core::ffi::c_uint;
pub const yajl_allow_partial_values: yajl_option = 16;
pub const yajl_allow_multiple_values: yajl_option = 8;
pub const yajl_allow_trailing_garbage: yajl_option = 4;
pub const yajl_dont_validate_strings: yajl_option = 2;
pub const yajl_allow_comments: yajl_option = 1;
pub const SIGSTOP: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const SIGCONT: ::core::ffi::c_int = 18 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
static mut current_key: C2RustUnnamed = KEY_VERSION;
unsafe extern "C" fn header_integer(
    mut ctx: *mut ::core::ffi::c_void,
    mut val: ::core::ffi::c_longlong,
) -> ::core::ffi::c_int {
    let mut child: *mut i3bar_child = ctx as *mut i3bar_child;
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
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn header_boolean(
    mut ctx: *mut ::core::ffi::c_void,
    mut val: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut child: *mut i3bar_child = ctx as *mut i3bar_child;
    match current_key as ::core::ffi::c_uint {
        3 => {
            (*child).click_events = val != 0;
        }
        _ => {}
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn header_map_key(
    mut ctx: *mut ::core::ffi::c_void,
    mut stringval: *const ::core::ffi::c_uchar,
    mut stringlen: size_t,
) -> ::core::ffi::c_int {
    if stringlen == strlen(b"version\0" as *const u8 as *const ::core::ffi::c_char)
        && (stringlen >= strlen(b"version\0" as *const u8 as *const ::core::ffi::c_char)
            && strncasecmp(
                stringval as *const ::core::ffi::c_char,
                b"version\0" as *const u8 as *const ::core::ffi::c_char,
                strlen(b"version\0" as *const u8 as *const ::core::ffi::c_char),
            ) == 0 as ::core::ffi::c_int)
    {
        current_key = KEY_VERSION;
    } else if stringlen == strlen(b"stop_signal\0" as *const u8 as *const ::core::ffi::c_char)
        && (stringlen >= strlen(b"stop_signal\0" as *const u8 as *const ::core::ffi::c_char)
            && strncasecmp(
                stringval as *const ::core::ffi::c_char,
                b"stop_signal\0" as *const u8 as *const ::core::ffi::c_char,
                strlen(b"stop_signal\0" as *const u8 as *const ::core::ffi::c_char),
            ) == 0 as ::core::ffi::c_int)
    {
        current_key = KEY_STOP_SIGNAL;
    } else if stringlen == strlen(b"cont_signal\0" as *const u8 as *const ::core::ffi::c_char)
        && (stringlen >= strlen(b"cont_signal\0" as *const u8 as *const ::core::ffi::c_char)
            && strncasecmp(
                stringval as *const ::core::ffi::c_char,
                b"cont_signal\0" as *const u8 as *const ::core::ffi::c_char,
                strlen(b"cont_signal\0" as *const u8 as *const ::core::ffi::c_char),
            ) == 0 as ::core::ffi::c_int)
    {
        current_key = KEY_CONT_SIGNAL;
    } else if stringlen == strlen(b"click_events\0" as *const u8 as *const ::core::ffi::c_char)
        && (stringlen >= strlen(b"click_events\0" as *const u8 as *const ::core::ffi::c_char)
            && strncasecmp(
                stringval as *const ::core::ffi::c_char,
                b"click_events\0" as *const u8 as *const ::core::ffi::c_char,
                strlen(b"click_events\0" as *const u8 as *const ::core::ffi::c_char),
            ) == 0 as ::core::ffi::c_int)
    {
        current_key = KEY_CLICK_EVENTS;
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn child_init(mut child: *mut i3bar_child) {
    (*child).version = 0 as uint32_t;
    (*child).stop_signal = SIGSTOP;
    (*child).cont_signal = SIGCONT;
}
#[no_mangle]
pub unsafe extern "C" fn parse_json_header(
    mut child: *mut i3bar_child,
    mut buffer: *const ::core::ffi::c_uchar,
    mut length: ::core::ffi::c_int,
    mut consumed: *mut ::core::ffi::c_uint,
) {
    static mut version_callbacks: yajl_callbacks = unsafe {
        yajl_callbacks {
            yajl_null: None,
            yajl_boolean: Some(
                header_boolean
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
            yajl_integer: Some(
                header_integer
                    as unsafe extern "C" fn(
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
                    as unsafe extern "C" fn(
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
    let mut handle: yajl_handle = yajl_alloc(
        &raw mut version_callbacks,
        ::core::ptr::null_mut::<yajl_alloc_funcs>(),
        child as *mut ::core::ffi::c_void,
    );
    yajl_config(handle, yajl_allow_trailing_garbage, 1 as ::core::ffi::c_int);
    let mut state: yajl_status = yajl_parse(handle, buffer, length as size_t);
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
