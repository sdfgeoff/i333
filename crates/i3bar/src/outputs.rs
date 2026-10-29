use crate::config::config;
use crate::xcb::destroy_window;
use i3_common::safewrappers::sasprintf;
/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3bar - an xcb-based status- and ws-bar for i3
 * © 2010 Axel Wagner and contributors (see also: LICENSE)
 *
 * outputs.c: Maintaining the outputs list
 *
 */
// Mechanical file-by-file Rust translation of i3bar/src/outputs.c.
// Originally translated with C2Rust 0.22.1; subsequently maintained in Rust.
use i3_json::compat::{yajl_alloc, yajl_free, yajl_parse};
pub use i3_json::compat::{yajl_alloc_funcs, yajl_callbacks, yajl_handle, yajl_status};

pub type i3String = _i3String;
pub use crate::child::rect_t;
pub use i3_common::draw_util::surface_t;
pub type rect = rect_t;
pub use crate::child::i3_output;
pub use crate::child::i3_ws;
pub use crate::child::outputs_head;
pub use crate::child::tc_head;
pub use crate::child::trayclient;
pub use crate::child::ws_head;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct outputs_json_params {
    pub outputs_walk: *mut i3_output,
    pub cur_key: *mut ::core::ffi::c_char,
    pub in_rect: bool,
}

pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
unsafe fn outputs_null_cb(params_: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    let params: *mut outputs_json_params = params_ as *mut outputs_json_params;
    free((*params).cur_key as *mut ::core::ffi::c_void);
    (*params).cur_key = ::core::ptr::null_mut::<::core::ffi::c_char>();
    1 as ::core::ffi::c_int
}
unsafe fn outputs_boolean_cb(
    params_: *mut ::core::ffi::c_void,
    val: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let params: *mut outputs_json_params = params_ as *mut outputs_json_params;
    if strcmp((*params).cur_key, c"active".as_ptr()) == 0 {
        (*(*params).outputs_walk).active = val != 0;
        free((*params).cur_key as *mut ::core::ffi::c_void);
        (*params).cur_key = ::core::ptr::null_mut::<::core::ffi::c_char>();
        return 1 as ::core::ffi::c_int;
    }
    if strcmp((*params).cur_key, c"primary".as_ptr()) == 0 {
        (*(*params).outputs_walk).primary = val != 0;
        free((*params).cur_key as *mut ::core::ffi::c_void);
        (*params).cur_key = ::core::ptr::null_mut::<::core::ffi::c_char>();
        return 1 as ::core::ffi::c_int;
    }
    0 as ::core::ffi::c_int
}
unsafe fn outputs_integer_cb(
    params_: *mut ::core::ffi::c_void,
    val: ::core::ffi::c_longlong,
) -> ::core::ffi::c_int {
    let params: *mut outputs_json_params = params_ as *mut outputs_json_params;
    if strcmp((*params).cur_key, c"current_workspace".as_ptr()) == 0 {
        (*(*params).outputs_walk).ws = val as ::core::ffi::c_int;
        free((*params).cur_key as *mut ::core::ffi::c_void);
        (*params).cur_key = ::core::ptr::null_mut::<::core::ffi::c_char>();
        return 1 as ::core::ffi::c_int;
    }
    if strcmp((*params).cur_key, c"x".as_ptr()) == 0 {
        (*(*params).outputs_walk).rect.x = val as ::core::ffi::c_int;
        free((*params).cur_key as *mut ::core::ffi::c_void);
        (*params).cur_key = ::core::ptr::null_mut::<::core::ffi::c_char>();
        return 1 as ::core::ffi::c_int;
    }
    if strcmp((*params).cur_key, c"y".as_ptr()) == 0 {
        (*(*params).outputs_walk).rect.y = val as ::core::ffi::c_int;
        free((*params).cur_key as *mut ::core::ffi::c_void);
        (*params).cur_key = ::core::ptr::null_mut::<::core::ffi::c_char>();
        return 1 as ::core::ffi::c_int;
    }
    if strcmp((*params).cur_key, c"width".as_ptr()) == 0 {
        (*(*params).outputs_walk).rect.w = val as ::core::ffi::c_int;
        free((*params).cur_key as *mut ::core::ffi::c_void);
        (*params).cur_key = ::core::ptr::null_mut::<::core::ffi::c_char>();
        return 1 as ::core::ffi::c_int;
    }
    if strcmp((*params).cur_key, c"height".as_ptr()) == 0 {
        (*(*params).outputs_walk).rect.h = val as ::core::ffi::c_int;
        free((*params).cur_key as *mut ::core::ffi::c_void);
        (*params).cur_key = ::core::ptr::null_mut::<::core::ffi::c_char>();
        return 1 as ::core::ffi::c_int;
    }
    0 as ::core::ffi::c_int
}
unsafe fn outputs_string_cb(
    params_: *mut ::core::ffi::c_void,
    val: *const ::core::ffi::c_uchar,
    len: usize,
) -> ::core::ffi::c_int {
    let params: *mut outputs_json_params = params_ as *mut outputs_json_params;
    if strcmp((*params).cur_key, c"current_workspace".as_ptr()) == 0 {
        let mut copy: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        sasprintf(
            &raw mut copy,
            c"%.*s".as_ptr(),
            len as ::core::ffi::c_int,
            val,
        );
        let mut end: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        *__errno_location() = 0 as ::core::ffi::c_int;
        let parsed_num: ::core::ffi::c_long = strtol(copy, &raw mut end, 10 as ::core::ffi::c_int);
        if *__errno_location() == 0 as ::core::ffi::c_int
            && (!end.is_null() && *end as ::core::ffi::c_int == '\0' as i32)
        {
            (*(*params).outputs_walk).ws = parsed_num as ::core::ffi::c_int;
        }
        free(copy as *mut ::core::ffi::c_void);
        copy = ::core::ptr::null_mut::<::core::ffi::c_char>();
        free((*params).cur_key as *mut ::core::ffi::c_void);
        (*params).cur_key = ::core::ptr::null_mut::<::core::ffi::c_char>();
        return 1 as ::core::ffi::c_int;
    }
    if strcmp((*params).cur_key, c"name".as_ptr()) != 0 {
        return 0 as ::core::ffi::c_int;
    }
    sasprintf(
        &raw mut (*(*params).outputs_walk).name,
        c"%.*s".as_ptr(),
        len as ::core::ffi::c_int,
        val,
    );
    free((*params).cur_key as *mut ::core::ffi::c_void);
    (*params).cur_key = ::core::ptr::null_mut::<::core::ffi::c_char>();
    1 as ::core::ffi::c_int
}
unsafe fn outputs_start_map_cb(params_: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    let params: *mut outputs_json_params = params_ as *mut outputs_json_params;
    let mut new_output: *mut i3_output = ::core::ptr::null_mut::<i3_output>();
    if (*params).cur_key.is_null() {
        new_output = smalloc(::core::mem::size_of::<i3_output>() as usize) as *mut i3_output;
        (*new_output).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
        (*new_output).active = false_0 != 0;
        (*new_output).primary = false_0 != 0;
        (*new_output).visible = false_0 != 0;
        (*new_output).ws = 0 as ::core::ffi::c_int;
        (*new_output).statusline_width = 0 as ::core::ffi::c_int;
        memset(
            &raw mut (*new_output).rect as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<rect>() as usize,
        );
        memset(
            &raw mut (*new_output).bar as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<surface_t>() as usize,
        );
        memset(
            &raw mut (*new_output).buffer as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<surface_t>() as usize,
        );
        memset(
            &raw mut (*new_output).statusline_buffer as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<surface_t>() as usize,
        );
        (*new_output).workspaces =
            smalloc(::core::mem::size_of::<ws_head>() as usize) as *mut ws_head;
        (*(*new_output).workspaces).tqh_first = ::core::ptr::null_mut::<i3_ws>();
        (*(*new_output).workspaces).tqh_last = &raw mut (*(*new_output).workspaces).tqh_first;
        (*new_output).trayclients =
            smalloc(::core::mem::size_of::<tc_head>() as usize) as *mut tc_head;
        (*(*new_output).trayclients).tqh_first = ::core::ptr::null_mut::<trayclient>();
        (*(*new_output).trayclients).tqh_last = &raw mut (*(*new_output).trayclients).tqh_first;
        (*params).outputs_walk = new_output;
        return 1 as ::core::ffi::c_int;
    }
    if strcmp((*params).cur_key, c"rect".as_ptr()) == 0 {
        (*params).in_rect = true_0 != 0;
    }
    1 as ::core::ffi::c_int
}
unsafe fn clear_output(output: *mut i3_output) {
    free((*output).name as *mut ::core::ffi::c_void);
    (*output).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free((*output).workspaces as *mut ::core::ffi::c_void);
    (*output).workspaces = ::core::ptr::null_mut::<ws_head>();
    free((*output).trayclients as *mut ::core::ffi::c_void);
    (*output).trayclients = ::core::ptr::null_mut::<tc_head>();
}
unsafe fn outputs_end_map_cb(params_: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    let params: *mut outputs_json_params = params_ as *mut outputs_json_params;
    if (*params).in_rect {
        (*params).in_rect = false_0 != 0;
        return 1 as ::core::ffi::c_int;
    }
    if config.num_outputs > 0 as ::core::ffi::c_int {
        let is_primary: bool = (*(*params).outputs_walk).primary;
        let mut handle_output: bool = false_0 != 0;
        let mut c: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while c < config.num_outputs {
            if strcasecmp(
                (*(*params).outputs_walk).name,
                *config.outputs.offset(c as isize),
            ) == 0 as ::core::ffi::c_int
                || strcasecmp(*config.outputs.offset(c as isize), c"primary".as_ptr())
                    == 0 as ::core::ffi::c_int
                    && is_primary as ::core::ffi::c_int != 0
                || strcasecmp(*config.outputs.offset(c as isize), c"nonprimary".as_ptr())
                    == 0 as ::core::ffi::c_int
                    && !is_primary
            {
                handle_output = true_0 != 0;
                break;
            } else {
                c += 1;
            }
        }
        if !handle_output {
            if config.verbose {
                printf(
                    c"[%s:%d] Ignoring output \"%s\", not configured to handle it.\n".as_ptr(),
                    c"../i3bar/src/outputs.c".as_ptr(),
                    205 as ::core::ffi::c_int,
                    (*(*params).outputs_walk).name,
                );
            }
            clear_output((*params).outputs_walk);
            free((*params).outputs_walk as *mut ::core::ffi::c_void);
            (*params).outputs_walk = ::core::ptr::null_mut::<i3_output>();
            free((*params).cur_key as *mut ::core::ffi::c_void);
            (*params).cur_key = ::core::ptr::null_mut::<::core::ffi::c_char>();
            return 1 as ::core::ffi::c_int;
        }
    }
    let target: *mut i3_output = get_output_by_name((*(*params).outputs_walk).name);
    if target.is_null() {
        (*(*params).outputs_walk).slist.sle_next = (*outputs).slh_first;
        (*outputs).slh_first = (*params).outputs_walk as *mut i3_output;
    } else {
        (*target).active = (*(*params).outputs_walk).active;
        (*target).primary = (*(*params).outputs_walk).primary;
        (*target).ws = (*(*params).outputs_walk).ws;
        (*target).rect = (*(*params).outputs_walk).rect;
        clear_output((*params).outputs_walk);
        free((*params).outputs_walk as *mut ::core::ffi::c_void);
        (*params).outputs_walk = ::core::ptr::null_mut::<i3_output>();
    }
    1 as ::core::ffi::c_int
}
unsafe fn outputs_map_key_cb(
    params_: *mut ::core::ffi::c_void,
    keyVal: *const ::core::ffi::c_uchar,
    keyLen: usize,
) -> ::core::ffi::c_int {
    let params: *mut outputs_json_params = params_ as *mut outputs_json_params;
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
static mut outputs_callbacks: yajl_callbacks = unsafe {
    yajl_callbacks {
        yajl_null: Some(
            outputs_null_cb as unsafe fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
        ),
        yajl_boolean: Some(
            outputs_boolean_cb
                as unsafe fn(*mut ::core::ffi::c_void, ::core::ffi::c_int) -> ::core::ffi::c_int,
        ),
        yajl_integer: Some(
            outputs_integer_cb
                as unsafe fn(
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_longlong,
                ) -> ::core::ffi::c_int,
        ),
        yajl_double: None,
        yajl_number: None,
        yajl_string: Some(
            outputs_string_cb
                as unsafe fn(
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_uchar,
                    usize,
                ) -> ::core::ffi::c_int,
        ),
        yajl_start_map: Some(
            outputs_start_map_cb as unsafe fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
        ),
        yajl_map_key: Some(
            outputs_map_key_cb
                as unsafe fn(
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_uchar,
                    usize,
                ) -> ::core::ffi::c_int,
        ),
        yajl_end_map: Some(
            outputs_end_map_cb as unsafe fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
        ),
        yajl_start_array: None,
        yajl_end_array: None,
    }
};

pub static mut outputs: *mut outputs_head =
    ::core::ptr::null::<outputs_head>() as *mut outputs_head;

pub unsafe fn init_outputs() {
    outputs = smalloc(::core::mem::size_of::<outputs_head>() as usize) as *mut outputs_head;
    (*outputs).slh_first = ::core::ptr::null_mut::<i3_output>();
}

pub unsafe fn parse_outputs_json(json: *const ::core::ffi::c_uchar, size: usize) {
    let mut params: outputs_json_params = outputs_json_params {
        outputs_walk: ::core::ptr::null_mut::<i3_output>(),
        cur_key: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        in_rect: false,
    };
    params.outputs_walk = ::core::ptr::null_mut::<i3_output>();
    params.cur_key = ::core::ptr::null_mut::<::core::ffi::c_char>();
    params.in_rect = false_0 != 0;
    let handle: yajl_handle = yajl_alloc(
        &raw mut outputs_callbacks,
        ::core::ptr::null_mut::<yajl_alloc_funcs>(),
        &raw mut params as *mut ::core::ffi::c_void,
    );
    let state: yajl_status = yajl_parse(handle, json, size);
    match state as ::core::ffi::c_uint {
        1 | 2 => {
            fprintf(
                stderr,
                c"[%s:%d] ERROR: Could not parse outputs reply!\n".as_ptr(),
                c"../i3bar/src/outputs.c".as_ptr(),
                282 as ::core::ffi::c_int,
            );
            exit(libc::EXIT_FAILURE);
        }
        0 | _ => {}
    }
    yajl_free(handle);
    free(params.cur_key as *mut ::core::ffi::c_void);
}

pub unsafe fn free_outputs() {
    free_workspaces();
    let mut outputs_walk: *mut i3_output = ::core::ptr::null_mut::<i3_output>();
    if outputs.is_null() {
        return;
    }
    outputs_walk = (*outputs).slh_first as *mut i3_output;
    while !outputs_walk.is_null() {
        destroy_window(outputs_walk);
        if !(*outputs_walk).trayclients.is_null()
            && !(*(*outputs_walk).trayclients).tqh_first.is_null()
        {
            let mut walk: *mut trayclient =
                (*(*outputs_walk).trayclients).tqh_first as *mut trayclient;
            while !(*(*outputs_walk).trayclients).tqh_first.is_null() {
                if !(*(*(*outputs_walk).trayclients).tqh_first)
                    .tailq
                    .tqe_next
                    .is_null()
                {
                    (*(*(*(*outputs_walk).trayclients).tqh_first).tailq.tqe_next)
                        .tailq
                        .tqe_prev = (*(*(*outputs_walk).trayclients).tqh_first).tailq.tqe_prev;
                } else {
                    (*(*outputs_walk).trayclients).tqh_last =
                        (*(*(*outputs_walk).trayclients).tqh_first).tailq.tqe_prev;
                }
                *(*(*(*outputs_walk).trayclients).tqh_first).tailq.tqe_prev =
                    (*(*(*outputs_walk).trayclients).tqh_first).tailq.tqe_next;
                free(walk as *mut ::core::ffi::c_void);
                walk = ::core::ptr::null_mut::<trayclient>();
                walk = (*(*outputs_walk).trayclients).tqh_first as *mut trayclient;
            }
        }
        clear_output(outputs_walk);
        outputs_walk = (*outputs_walk).slist.sle_next as *mut i3_output;
    }
    let mut walk_0: *mut i3_output = (*outputs).slh_first as *mut i3_output;
    while !(*outputs).slh_first.is_null() {
        (*outputs).slh_first = (*(*outputs).slh_first).slist.sle_next;
        free(walk_0 as *mut ::core::ffi::c_void);
        walk_0 = ::core::ptr::null_mut::<i3_output>();
        walk_0 = (*outputs).slh_first as *mut i3_output;
    }
}

pub unsafe fn get_output_by_name(name: *mut ::core::ffi::c_char) -> *mut i3_output {
    if name.is_null() {
        return ::core::ptr::null_mut::<i3_output>();
    }
    let is_primary: bool = strcasecmp(name, c"primary".as_ptr()) == 0;
    let mut walk: *mut i3_output = ::core::ptr::null_mut::<i3_output>();
    walk = (*outputs).slh_first as *mut i3_output;
    while !walk.is_null() {
        if is_primary as ::core::ffi::c_int != 0 && (*walk).primary as ::core::ffi::c_int != 0
            || strcmp((*walk).name, name) == 0
        {
            break;
        }
        walk = (*walk).slist.sle_next as *mut i3_output;
    }
    walk
}

pub unsafe fn output_has_focus(output: *mut i3_output) -> bool {
    let mut ws_walk: *mut i3_ws = ::core::ptr::null_mut::<i3_ws>();
    ws_walk = (*(*output).workspaces).tqh_first as *mut i3_ws;
    while !ws_walk.is_null() {
        if (*ws_walk).focused {
            return true_0 != 0;
        }
        ws_walk = (*ws_walk).tailq.tqe_next as *mut i3_ws;
    }
    false_0 != 0
}

// Internal helpers use Rust linkage rather than foreign symbol lookup.
use crate::workspaces::free_workspaces;
use i3_common::safewrappers::smalloc;

// Canonical Cairo backend ABI; renderer logic remains local.

pub use libc::{__errno_location, exit, fprintf, free, memset, printf, strcasecmp, strcmp, strtol};

use i3_common::native_stdio::stderr;

pub use i3_common::string::I3String as _i3String;
