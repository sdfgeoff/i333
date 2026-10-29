use crate::config::config;
use crate::outputs::get_output_by_name;
use crate::outputs::outputs;
use i3_common::font::predict_text_width;
use i3_common::safewrappers::sasprintf;
use i3_common::string::i3string_as_utf8;
use i3_common::string::i3string_free;
use i3_common::string::i3string_from_markup;
use i3_common::string::i3string_from_markup_with_length;
use i3_common::string::i3string_get_num_glyphs;
/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3bar - an xcb-based status- and ws-bar for i3
 * © 2010 Axel Wagner and contributors (see also: LICENSE)
 *
 * workspaces.c: Maintaining the workspace lists
 *
 */
// Mechanical file-by-file Rust translation of i3bar/src/workspaces.c.
// Originally translated with C2Rust 0.22.1; subsequently maintained in Rust.
use i3_json::compat::{yajl_alloc, yajl_free, yajl_free_error, yajl_get_error, yajl_parse};
pub use i3_json::compat::{yajl_alloc_funcs, yajl_callbacks, yajl_handle, yajl_status};

pub type i3String = _i3String;
pub use crate::child::rect_t;
pub type rect = rect_t;
pub use crate::child::i3_output;
pub use crate::child::i3_ws;
pub use crate::child::ws_head;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct workspaces_json_params {
    pub workspaces: *mut ws_head,
    pub workspaces_walk: *mut i3_ws,
    pub cur_key: *mut ::core::ffi::c_char,
    pub need_output: bool,
    pub parsing_rect: bool,
}

pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
unsafe fn workspaces_boolean_cb(
    params_: *mut ::core::ffi::c_void,
    val: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let params: *mut workspaces_json_params = params_ as *mut workspaces_json_params;
    if strcmp((*params).cur_key, c"visible".as_ptr()) == 0 {
        (*(*params).workspaces_walk).visible = val != 0;
        free((*params).cur_key as *mut ::core::ffi::c_void);
        (*params).cur_key = ::core::ptr::null_mut::<::core::ffi::c_char>();
        return 1 as ::core::ffi::c_int;
    }
    if strcmp((*params).cur_key, c"focused".as_ptr()) == 0 {
        (*(*params).workspaces_walk).focused = val != 0;
        free((*params).cur_key as *mut ::core::ffi::c_void);
        (*params).cur_key = ::core::ptr::null_mut::<::core::ffi::c_char>();
        return 1 as ::core::ffi::c_int;
    }
    if strcmp((*params).cur_key, c"urgent".as_ptr()) == 0 {
        (*(*params).workspaces_walk).urgent = val != 0;
        free((*params).cur_key as *mut ::core::ffi::c_void);
        (*params).cur_key = ::core::ptr::null_mut::<::core::ffi::c_char>();
        return 1 as ::core::ffi::c_int;
    }
    free((*params).cur_key as *mut ::core::ffi::c_void);
    (*params).cur_key = ::core::ptr::null_mut::<::core::ffi::c_char>();
    0 as ::core::ffi::c_int
}
unsafe fn workspaces_integer_cb(
    params_: *mut ::core::ffi::c_void,
    val: ::core::ffi::c_longlong,
) -> ::core::ffi::c_int {
    let params: *mut workspaces_json_params = params_ as *mut workspaces_json_params;
    if strcmp((*params).cur_key, c"id".as_ptr()) == 0 {
        (*(*params).workspaces_walk).id = val as usize;
        free((*params).cur_key as *mut ::core::ffi::c_void);
        (*params).cur_key = ::core::ptr::null_mut::<::core::ffi::c_char>();
        return 1 as ::core::ffi::c_int;
    }
    if strcmp((*params).cur_key, c"num".as_ptr()) == 0 {
        (*(*params).workspaces_walk).num = val as ::core::ffi::c_int;
        free((*params).cur_key as *mut ::core::ffi::c_void);
        (*params).cur_key = ::core::ptr::null_mut::<::core::ffi::c_char>();
        return 1 as ::core::ffi::c_int;
    }
    if strcmp((*params).cur_key, c"x".as_ptr()) == 0 {
        free((*params).cur_key as *mut ::core::ffi::c_void);
        (*params).cur_key = ::core::ptr::null_mut::<::core::ffi::c_char>();
        return 1 as ::core::ffi::c_int;
    }
    if strcmp((*params).cur_key, c"y".as_ptr()) == 0 {
        free((*params).cur_key as *mut ::core::ffi::c_void);
        (*params).cur_key = ::core::ptr::null_mut::<::core::ffi::c_char>();
        return 1 as ::core::ffi::c_int;
    }
    if strcmp((*params).cur_key, c"width".as_ptr()) == 0 {
        free((*params).cur_key as *mut ::core::ffi::c_void);
        (*params).cur_key = ::core::ptr::null_mut::<::core::ffi::c_char>();
        return 1 as ::core::ffi::c_int;
    }
    if strcmp((*params).cur_key, c"height".as_ptr()) == 0 {
        free((*params).cur_key as *mut ::core::ffi::c_void);
        (*params).cur_key = ::core::ptr::null_mut::<::core::ffi::c_char>();
        return 1 as ::core::ffi::c_int;
    }
    free((*params).cur_key as *mut ::core::ffi::c_void);
    (*params).cur_key = ::core::ptr::null_mut::<::core::ffi::c_char>();
    0 as ::core::ffi::c_int
}
unsafe fn workspaces_string_cb(
    params_: *mut ::core::ffi::c_void,
    val: *const ::core::ffi::c_uchar,
    len: usize,
) -> ::core::ffi::c_int {
    let params: *mut workspaces_json_params = params_ as *mut workspaces_json_params;
    if strcmp((*params).cur_key, c"name".as_ptr()) == 0 {
        let ws_name: *const ::core::ffi::c_char = val as *const ::core::ffi::c_char;
        (*(*params).workspaces_walk).canonical_name = sstrndup(ws_name, len);
        if (config.strip_ws_numbers as ::core::ffi::c_int != 0
            || config.strip_ws_name as ::core::ffi::c_int != 0)
            && (*(*params).workspaces_walk).num >= 0 as ::core::ffi::c_int
        {
            static mut ws_num: [::core::ffi::c_char; 32] = [0; 32];
            snprintf(
                &raw mut ws_num as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 32]>() as usize,
                c"%d".as_ptr(),
                (*(*params).workspaces_walk).num,
            );
            let mut offset: usize =
                strspn(ws_name, &raw mut ws_num as *mut ::core::ffi::c_char) as usize;
            if offset != 0 && *ws_name.offset(offset as isize) as ::core::ffi::c_int == ':' as i32 {
                offset = offset.wrapping_add(1 as usize);
            }
            if config.strip_ws_numbers {
                (*(*params).workspaces_walk).name = if offset < len {
                    i3string_from_markup_with_length(
                        ws_name.offset(offset as isize),
                        len.wrapping_sub(offset),
                    )
                } else {
                    i3string_from_markup(&raw mut ws_num as *mut ::core::ffi::c_char)
                };
            } else {
                (*(*params).workspaces_walk).name =
                    i3string_from_markup(&raw mut ws_num as *mut ::core::ffi::c_char);
            }
        } else {
            (*(*params).workspaces_walk).name = i3string_from_markup_with_length(ws_name, len);
        }
        (*(*params).workspaces_walk).name_width =
            predict_text_width((*(*params).workspaces_walk).name);
        if config.verbose {
            printf(
                c"[%s:%d] Got workspace canonical: %s, name: '%s', name_width: %d, glyphs: %zu\n"
                    .as_ptr(),
                c"../i3bar/src/workspaces.c".as_ptr(),
                146 as ::core::ffi::c_int,
                (*(*params).workspaces_walk).canonical_name,
                i3string_as_utf8((*(*params).workspaces_walk).name),
                (*(*params).workspaces_walk).name_width,
                i3string_get_num_glyphs((*(*params).workspaces_walk).name),
            );
        }
        free((*params).cur_key as *mut ::core::ffi::c_void);
        (*params).cur_key = ::core::ptr::null_mut::<::core::ffi::c_char>();
        return 1 as ::core::ffi::c_int;
    }
    if strcmp((*params).cur_key, c"output".as_ptr()) == 0 {
        let mut output_name: *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<::core::ffi::c_char>();
        sasprintf(
            &raw mut output_name,
            c"%.*s".as_ptr(),
            len as ::core::ffi::c_int,
            val,
        );
        let target: *mut i3_output = get_output_by_name(output_name);
        let ws: *mut i3_ws = (*params).workspaces_walk;
        if !target.is_null() {
            (*ws).output = target as *mut i3_output;
            (*ws).tailq.tqe_next = ::core::ptr::null_mut::<i3_ws>();
            (*ws).tailq.tqe_prev = (*(*(*ws).output).workspaces).tqh_last;
            *(*(*(*ws).output).workspaces).tqh_last = ws as *mut i3_ws;
            (*(*(*ws).output).workspaces).tqh_last = &raw mut (*ws).tailq.tqe_next;
        }
        (*params).need_output = false_0 != 0;
        free(output_name as *mut ::core::ffi::c_void);
        output_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
        free((*params).cur_key as *mut ::core::ffi::c_void);
        (*params).cur_key = ::core::ptr::null_mut::<::core::ffi::c_char>();
        return 1 as ::core::ffi::c_int;
    }
    0 as ::core::ffi::c_int
}
unsafe fn workspaces_start_map_cb(params_: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    let params: *mut workspaces_json_params = params_ as *mut workspaces_json_params;
    if (*params).cur_key.is_null() {
        let new_workspace: *mut i3_ws =
            scalloc(1 as usize, ::core::mem::size_of::<i3_ws>() as usize) as *mut i3_ws;
        (*new_workspace).num = -(1 as ::core::ffi::c_int);
        (*params).workspaces_walk = new_workspace;
        (*params).need_output = true_0 != 0;
        (*params).parsing_rect = false_0 != 0;
    } else {
        (*params).parsing_rect = true_0 != 0;
    }
    1 as ::core::ffi::c_int
}
unsafe fn workspaces_end_map_cb(params_: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    let params: *mut workspaces_json_params = params_ as *mut workspaces_json_params;
    if (*params).parsing_rect {
        (*params).parsing_rect = false_0 != 0;
        return 1 as ::core::ffi::c_int;
    }
    let ws: *mut i3_ws = (*params).workspaces_walk;
    if ws.is_null() || !(*ws).output.is_null() {
        return 1 as ::core::ffi::c_int;
    }
    if !(*params).need_output {
        i3string_free((*ws).name);
        (*ws).name = ::core::ptr::null_mut::<i3String>();
        free((*ws).canonical_name as *mut ::core::ffi::c_void);
        (*ws).canonical_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
        free((*params).workspaces_walk as *mut ::core::ffi::c_void);
        (*params).workspaces_walk = ::core::ptr::null_mut::<i3_ws>();
        return 1 as ::core::ffi::c_int;
    }
    if (*ws).name.is_null() || (*outputs).slh_first.is_null() {
        i3string_free((*ws).name);
        (*ws).name = ::core::ptr::null_mut::<i3String>();
        free((*ws).canonical_name as *mut ::core::ffi::c_void);
        (*ws).canonical_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
        free((*params).workspaces_walk as *mut ::core::ffi::c_void);
        (*params).workspaces_walk = ::core::ptr::null_mut::<i3_ws>();
        return 1 as ::core::ffi::c_int;
    }
    (*ws).output = get_output_by_name(c"primary".as_ptr().cast_mut()) as *mut i3_output;
    if (*ws).output.is_null() {
        (*ws).output = (*outputs).slh_first;
    }
    (*ws).tailq.tqe_next = ::core::ptr::null_mut::<i3_ws>();
    (*ws).tailq.tqe_prev = (*(*(*ws).output).workspaces).tqh_last;
    *(*(*(*ws).output).workspaces).tqh_last = ws as *mut i3_ws;
    (*(*(*ws).output).workspaces).tqh_last = &raw mut (*ws).tailq.tqe_next;
    1 as ::core::ffi::c_int
}
unsafe fn workspaces_map_key_cb(
    params_: *mut ::core::ffi::c_void,
    keyVal: *const ::core::ffi::c_uchar,
    keyLen: usize,
) -> ::core::ffi::c_int {
    let params: *mut workspaces_json_params = params_ as *mut workspaces_json_params;
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
static mut workspaces_callbacks: yajl_callbacks = unsafe {
    yajl_callbacks {
        yajl_null: None,
        yajl_boolean: Some(
            workspaces_boolean_cb
                as unsafe fn(*mut ::core::ffi::c_void, ::core::ffi::c_int) -> ::core::ffi::c_int,
        ),
        yajl_integer: Some(
            workspaces_integer_cb
                as unsafe fn(
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_longlong,
                ) -> ::core::ffi::c_int,
        ),
        yajl_double: None,
        yajl_number: None,
        yajl_string: Some(
            workspaces_string_cb
                as unsafe fn(
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_uchar,
                    usize,
                ) -> ::core::ffi::c_int,
        ),
        yajl_start_map: Some(
            workspaces_start_map_cb as unsafe fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
        ),
        yajl_map_key: Some(
            workspaces_map_key_cb
                as unsafe fn(
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_uchar,
                    usize,
                ) -> ::core::ffi::c_int,
        ),
        yajl_end_map: Some(
            workspaces_end_map_cb as unsafe fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
        ),
        yajl_start_array: None,
        yajl_end_array: None,
    }
};

pub unsafe fn parse_workspaces_json(json: *const ::core::ffi::c_uchar, size: usize) {
    free_workspaces();
    let mut params: workspaces_json_params = workspaces_json_params {
        workspaces: ::core::ptr::null_mut::<ws_head>(),
        workspaces_walk: ::core::ptr::null_mut::<i3_ws>(),
        cur_key: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        need_output: false,
        parsing_rect: false,
    };
    let handle: yajl_handle = yajl_alloc(
        &raw mut workspaces_callbacks,
        ::core::ptr::null_mut::<yajl_alloc_funcs>(),
        &raw mut params as *mut ::core::ffi::c_void,
    ) as yajl_handle;
    let state: yajl_status = yajl_parse(handle, json, size) as yajl_status;
    match state as ::core::ffi::c_uint {
        1 | 2 => {
            let err: *mut ::core::ffi::c_uchar =
                yajl_get_error(handle, 1 as ::core::ffi::c_int, json, size);
            fprintf(
                stderr,
                c"[%s:%d] ERROR: Could not parse workspaces reply, error:\n%s\njson:---%s---\n"
                    .as_ptr(),
                c"../i3bar/src/workspaces.c".as_ptr(),
                275 as ::core::ffi::c_int,
                err as *mut ::core::ffi::c_char,
                json as *mut ::core::ffi::c_char,
            );
            yajl_free_error(handle, err);
            if !config.workspace_command.is_null() {
                kill_ws_child();
                set_workspace_button_error(c"Could not parse workspace_command's JSON".as_ptr());
            } else {
                exit(libc::EXIT_FAILURE);
            }
        }
        0 | _ => {}
    }
    yajl_free(handle);
    free(params.cur_key as *mut ::core::ffi::c_void);
    params.cur_key = ::core::ptr::null_mut::<::core::ffi::c_char>();
}

pub unsafe fn free_workspaces() {
    if outputs.is_null() {
        return;
    }
    let mut outputs_walk: *mut i3_output = ::core::ptr::null_mut::<i3_output>();
    outputs_walk = (*outputs).slh_first as *mut i3_output;
    while !outputs_walk.is_null() {
        if !(*outputs_walk).workspaces.is_null()
            && !(*(*outputs_walk).workspaces).tqh_first.is_null()
        {
            let mut ws_walk: *mut i3_ws = ::core::ptr::null_mut::<i3_ws>();
            ws_walk = (*(*outputs_walk).workspaces).tqh_first as *mut i3_ws;
            while !ws_walk.is_null() {
                i3string_free((*ws_walk).name);
                (*ws_walk).name = ::core::ptr::null_mut::<i3String>();
                free((*ws_walk).canonical_name as *mut ::core::ffi::c_void);
                (*ws_walk).canonical_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
                ws_walk = (*ws_walk).tailq.tqe_next as *mut i3_ws;
            }
            let mut walk: *mut i3_ws = (*(*outputs_walk).workspaces).tqh_first as *mut i3_ws;
            while !(*(*outputs_walk).workspaces).tqh_first.is_null() {
                if !(*(*(*outputs_walk).workspaces).tqh_first)
                    .tailq
                    .tqe_next
                    .is_null()
                {
                    (*(*(*(*outputs_walk).workspaces).tqh_first).tailq.tqe_next)
                        .tailq
                        .tqe_prev = (*(*(*outputs_walk).workspaces).tqh_first).tailq.tqe_prev;
                } else {
                    (*(*outputs_walk).workspaces).tqh_last =
                        (*(*(*outputs_walk).workspaces).tqh_first).tailq.tqe_prev;
                }
                *(*(*(*outputs_walk).workspaces).tqh_first).tailq.tqe_prev =
                    (*(*(*outputs_walk).workspaces).tqh_first).tailq.tqe_next;
                free(walk as *mut ::core::ffi::c_void);
                walk = ::core::ptr::null_mut::<i3_ws>();
                walk = (*(*outputs_walk).workspaces).tqh_first as *mut i3_ws;
            }
        }
        outputs_walk = (*outputs_walk).slist.sle_next as *mut i3_output;
    }
}

// Internal helpers use Rust linkage rather than foreign symbol lookup.
use crate::child::kill_ws_child;
use crate::child::set_workspace_button_error;
use i3_common::safewrappers::scalloc;
use i3_common::safewrappers::sstrndup;

// Canonical Cairo backend ABI; renderer logic remains local.

pub use libc::{exit, fprintf, free, printf, snprintf, strcmp, strspn};

use i3_common::native_stdio::stderr;

pub use i3_common::string::I3String as _i3String;
