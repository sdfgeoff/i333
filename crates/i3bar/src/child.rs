use crate::config::config;
use crate::outputs::outputs;
use crate::parse_json_header::parse_json_header;
use i3_common::font::predict_text_width;
use i3_common::safewrappers::sasprintf;
use i3_common::string::i3string_as_utf8;
use i3_common::string::i3string_free;
use i3_common::string::i3string_from_markup_with_length;
use i3_common::string::i3string_from_utf8;
use i3_common::string::i3string_set_markup;
/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3bar - an xcb-based status- and ws-bar for i3
 * © 2010 Axel Wagner and contributors (see also: LICENSE)
 *
 * child.c: Getting input for the statusline
 *
 */
// Mechanical file-by-file Rust translation of i3bar/src/child.c.
// Originally translated with C2Rust 0.22.1; subsequently maintained in Rust.
use i3_common::i333_va_list;
use i3_json::compat::{
    yajl_alloc, yajl_free_error, yajl_gen_alloc, yajl_gen_array_close, yajl_gen_array_open,
    yajl_gen_clear, yajl_gen_get_buf, yajl_gen_integer, yajl_gen_map_close, yajl_gen_map_open,
    yajl_gen_string, yajl_get_error, yajl_parse,
};
pub use i3_json::compat::{
    yajl_alloc_funcs, yajl_callbacks, yajl_gen, yajl_gen_t, yajl_handle, yajl_handle_t,
    yajl_status, yajl_status_ok,
};
pub use libev_sys::{
    ev_child, ev_child_start, ev_child_stop, ev_io, ev_io_start, ev_io_stop, ev_loop, ev_watcher,
    EV_READ, EV__IOFDSET,
};

pub type i3String = _i3String;
pub use i3_common::draw_util::surface_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rect_t {
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub w: ::core::ffi::c_int,
    pub h: ::core::ffi::c_int,
}
pub type rect = rect_t;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct status_block_render_desc {
    pub width: u32,
    pub x_offset: u32,
    pub x_append: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct status_block {
    pub full_text: *mut i3String,
    pub short_text: *mut i3String,
    pub use_short: bool,
    pub render_length: u32,
    pub color: *mut ::core::ffi::c_char,
    pub background: *mut ::core::ffi::c_char,
    pub border: *mut ::core::ffi::c_char,
    pub min_width: u32,
    pub min_width_str: *mut ::core::ffi::c_char,
    pub align: crate::child::BlockAlign,
    pub urgent: bool,
    pub no_separator: bool,
    pub border_top: u32,
    pub border_right: u32,
    pub border_bottom: u32,
    pub border_left: u32,
    pub pango_markup: bool,
    pub sep_block_width: u32,
    pub full_render: status_block_render_desc,
    pub short_render: status_block_render_desc,
    pub name: *mut ::core::ffi::c_char,
    pub instance: *mut ::core::ffi::c_char,
    pub blocks: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub tqe_next: *mut status_block,
    pub tqe_prev: *mut *mut status_block,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct statusline_head {
    pub tqh_first: *mut status_block,
    pub tqh_last: *mut *mut status_block,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct i3bar_child {
    pub pid: libc::pid_t,
    pub version: u32,
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

pub use crate::workspaces::Workspace as i3_ws;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub tqe_next: *mut i3_ws,
    pub tqe_prev: *mut *mut i3_ws,
}
pub use crate::outputs::Output as i3_output;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub sle_next: *mut i3_output,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tc_head {
    pub tqh_first: *mut trayclient,
    pub tqh_last: *mut *mut trayclient,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct trayclient {
    pub win: xcb_window_t,
    pub mapped: bool,
    pub xe_version: ::core::ffi::c_int,
    pub class_class: *mut ::core::ffi::c_char,
    pub class_instance: *mut ::core::ffi::c_char,
    pub tailq: C2RustUnnamed_6,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub tqe_next: *mut trayclient,
    pub tqe_prev: *mut *mut trayclient,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ws_head {
    pub tqh_first: *mut i3_ws,
    pub tqh_last: *mut *mut i3_ws,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct outputs_head {
    pub slh_first: *mut i3_output,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct parser_ctx {
    pub has_urgent: bool,
    pub last_map_key: *mut ::core::ffi::c_char,
    pub block: status_block,
}

pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();

pub const STDIN_CHUNK_SIZE: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;

pub const _PATH_BSHELL: [::core::ffi::c_char; 8] =
    unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"/bin/sh\0") };

pub static mut status_child: i3bar_child = i3bar_child {
    pid: 0 as libc::pid_t,
    version: 0,
    stopped: false,
    stop_signal: 0,
    cont_signal: 0,
    click_events: false,
    click_events_init: false,
    stdin_io: ::core::ptr::null::<ev_io>() as *mut ev_io,
    child_sig: ::core::ptr::null::<ev_child>() as *mut ev_child,
    stdin_fd: 0,
    pending_line: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
};

pub static mut ws_child: i3bar_child = i3bar_child {
    pid: 0 as libc::pid_t,
    version: 0,
    stopped: false,
    stop_signal: 0,
    cont_signal: 0,
    click_events: false,
    click_events_init: false,
    stdin_io: ::core::ptr::null::<ev_io>() as *mut ev_io,
    child_sig: ::core::ptr::null::<ev_child>() as *mut ev_child,
    stdin_fd: 0,
    pending_line: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
};

pub static mut parser: yajl_handle = ::core::ptr::null::<yajl_handle_t>() as *mut yajl_handle_t;

pub static mut gen: yajl_gen = ::core::ptr::null::<yajl_gen_t>() as *mut yajl_gen_t;

pub static mut parser_context: parser_ctx = parser_ctx {
    has_urgent: false,
    last_map_key: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    block: status_block {
        full_text: ::core::ptr::null::<i3String>() as *mut i3String,
        short_text: ::core::ptr::null::<i3String>() as *mut i3String,
        use_short: false,
        render_length: 0,
        color: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        background: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        border: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        min_width: 0,
        min_width_str: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        align: crate::child::BlockAlign::Left,
        urgent: false,
        no_separator: false,
        border_top: 0,
        border_right: 0,
        border_bottom: 0,
        border_left: 0,
        pango_markup: false,
        sep_block_width: 0,
        full_render: status_block_render_desc {
            width: 0,
            x_offset: 0,
            x_append: 0,
        },
        short_render: status_block_render_desc {
            width: 0,
            x_offset: 0,
            x_append: 0,
        },
        name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        instance: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        blocks: C2RustUnnamed {
            tqe_next: ::core::ptr::null::<status_block>() as *mut status_block,
            tqe_prev: ::core::ptr::null::<*mut status_block>() as *mut *mut status_block,
        },
    },
};

pub static mut statusline_head: statusline_head = statusline_head {
    tqh_first: ::core::ptr::null::<status_block>() as *mut status_block,
    tqh_last: ::core::ptr::null::<*mut status_block>() as *mut *mut status_block,
};

pub static mut statusline_buffer: statusline_head = statusline_head {
    tqh_first: ::core::ptr::null::<status_block>() as *mut status_block,
    tqh_last: ::core::ptr::null::<*mut status_block>() as *mut *mut status_block,
};

pub static mut child_stdin: ::core::ffi::c_int = 0;

pub unsafe fn clear_statusline(head: *mut statusline_head, free_resources: bool) {
    let mut first: *mut status_block = ::core::ptr::null_mut::<status_block>();
    while !(*head).tqh_first.is_null() {
        first = (*head).tqh_first;
        if free_resources {
            i3string_free((*first).full_text);
            (*first).full_text = ::core::ptr::null_mut::<i3String>();
            i3string_free((*first).short_text);
            (*first).short_text = ::core::ptr::null_mut::<i3String>();
            free((*first).color as *mut ::core::ffi::c_void);
            (*first).color = ::core::ptr::null_mut::<::core::ffi::c_char>();
            free((*first).name as *mut ::core::ffi::c_void);
            (*first).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
            free((*first).instance as *mut ::core::ffi::c_void);
            (*first).instance = ::core::ptr::null_mut::<::core::ffi::c_char>();
            free((*first).min_width_str as *mut ::core::ffi::c_void);
            (*first).min_width_str = ::core::ptr::null_mut::<::core::ffi::c_char>();
            free((*first).background as *mut ::core::ffi::c_void);
            (*first).background = ::core::ptr::null_mut::<::core::ffi::c_char>();
            free((*first).border as *mut ::core::ffi::c_void);
            (*first).border = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        if !(*first).blocks.tqe_next.is_null() {
            (*(*first).blocks.tqe_next).blocks.tqe_prev = (*first).blocks.tqe_prev;
        } else {
            (*head).tqh_last = (*first).blocks.tqe_prev;
        }
        *(*first).blocks.tqe_prev = (*first).blocks.tqe_next;
        free(first as *mut ::core::ffi::c_void);
    }
}
unsafe fn copy_statusline(from: *mut statusline_head, to: *mut statusline_head) {
    let mut current: *mut status_block = ::core::ptr::null_mut::<status_block>();
    current = (*from).tqh_first;
    while !current.is_null() {
        let new_block: *mut status_block =
            smalloc(::core::mem::size_of::<status_block>() as usize) as *mut status_block;
        memcpy(
            new_block as *mut ::core::ffi::c_void,
            current as *const ::core::ffi::c_void,
            ::core::mem::size_of::<status_block>() as usize,
        );
        (*new_block).blocks.tqe_next = ::core::ptr::null_mut::<status_block>();
        (*new_block).blocks.tqe_prev = (*to).tqh_last;
        *(*to).tqh_last = new_block;
        (*to).tqh_last = &raw mut (*new_block).blocks.tqe_next;
        current = (*current).blocks.tqe_next;
    }
}
unsafe extern "C" fn set_statusline_error(format: *const ::core::ffi::c_char, args: ...) {
    let mut err_block: *mut status_block = ::core::ptr::null_mut::<status_block>();
    let mut message_block: *mut status_block = ::core::ptr::null_mut::<status_block>();
    clear_statusline(&raw mut statusline_head, true_0 != 0);
    let mut message: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut args_0;
    args_0 = args.clone();
    if !(vasprintf(&raw mut message, format, i333_va_list!(args_0)) == -(1 as ::core::ffi::c_int)) {
        err_block = scalloc(1 as usize, ::core::mem::size_of::<status_block>() as usize)
            as *mut status_block;
        (*err_block).full_text = i3string_from_utf8(c"Error: ".as_ptr());
        (*err_block).name = sstrdup(c"error".as_ptr());
        (*err_block).color = sstrdup(c"#ff0000".as_ptr());
        (*err_block).no_separator = true_0 != 0;
        message_block = scalloc(1 as usize, ::core::mem::size_of::<status_block>() as usize)
            as *mut status_block;
        (*message_block).full_text = i3string_from_utf8(message);
        (*message_block).name = sstrdup(c"error_message".as_ptr());
        (*message_block).color = sstrdup(c"#ff0000".as_ptr());
        (*message_block).no_separator = true_0 != 0;
        (*err_block).blocks.tqe_next = statusline_head.tqh_first;
        if !(*err_block).blocks.tqe_next.is_null() {
            (*statusline_head.tqh_first).blocks.tqe_prev = &raw mut (*err_block).blocks.tqe_next;
        } else {
            statusline_head.tqh_last = &raw mut (*err_block).blocks.tqe_next;
        }
        statusline_head.tqh_first = err_block;
        (*err_block).blocks.tqe_prev = &raw mut statusline_head.tqh_first;
        (*message_block).blocks.tqe_next = ::core::ptr::null_mut::<status_block>();
        (*message_block).blocks.tqe_prev = statusline_head.tqh_last;
        *statusline_head.tqh_last = message_block;
        statusline_head.tqh_last = &raw mut (*message_block).blocks.tqe_next;
    }
    free(message as *mut ::core::ffi::c_void);
}
unsafe fn cleanup(c: *mut i3bar_child) {
    if (*c).pid == 0 as ::core::ffi::c_int {
        if config.verbose {
            printf(
                c"[%s:%d] %s: child pid = 0\n".as_ptr(),
                c"../i3bar/src/child.c".as_ptr(),
                158 as ::core::ffi::c_int,
                c"cleanup".as_ptr(),
            );
        }
    } else if (*c).pid == status_child.pid {
        if config.verbose {
            printf(
                c"[%s:%d] %s: status_command: pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n".as_ptr(),
                c"../i3bar/src/child.c".as_ptr(),
                158 as ::core::ffi::c_int,
                c"cleanup".as_ptr(),
                (*c).pid as ::core::ffi::c_long,
                (*c).stopped as ::core::ffi::c_int,
                (*c).stop_signal,
                (*c).cont_signal,
                (*c).click_events as ::core::ffi::c_int,
                (*c).click_events_init as ::core::ffi::c_int,
            );
        }
    } else if (*c).pid == ws_child.pid {
        if config.verbose {
            printf(
                c"[%s:%d] %s: workspace_command: pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n".as_ptr(),
                c"../i3bar/src/child.c".as_ptr(),
                158 as ::core::ffi::c_int,
                c"cleanup".as_ptr(),
                (*c).pid as ::core::ffi::c_long,
                (*c).stopped as ::core::ffi::c_int,
                (*c).stop_signal,
                (*c).cont_signal,
                (*c).click_events as ::core::ffi::c_int,
                (*c).click_events_init as ::core::ffi::c_int,
            );
        }
    } else {
        fprintf(
            stderr,
            c"[%s:%d] ERROR: %s: unknown child, this should never happen pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n".as_ptr(),
            c"../i3bar/src/child.c".as_ptr(),
            158 as ::core::ffi::c_int,
            c"cleanup".as_ptr(),
            (*c).pid as ::core::ffi::c_long,
            (*c).stopped as ::core::ffi::c_int,
            (*c).stop_signal,
            (*c).cont_signal,
            (*c).click_events as ::core::ffi::c_int,
            (*c).click_events_init as ::core::ffi::c_int,
        );
    }
    if !(*c).stdin_io.is_null() {
        ev_io_stop(main_loop as *mut ev_loop, (*c).stdin_io);
        free((*c).stdin_io as *mut ::core::ffi::c_void);
        (*c).stdin_io = ::core::ptr::null_mut::<ev_io>();
        if (*c).pid == status_child.pid {
            close(child_stdin);
            child_stdin = 0 as ::core::ffi::c_int;
        }
        close((*c).stdin_fd);
    }
    if !(*c).child_sig.is_null() {
        ev_child_stop(main_loop as *mut ev_loop, (*c).child_sig);
        free((*c).child_sig as *mut ::core::ffi::c_void);
        (*c).child_sig = ::core::ptr::null_mut::<ev_child>();
    }
    free((*c).pending_line as *mut ::core::ffi::c_void);
    (*c).pending_line = ::core::ptr::null_mut::<::core::ffi::c_char>();
    memset(
        c as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<i3bar_child>() as usize,
    );
}
unsafe fn stdin_start_array(context: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    clear_statusline(&raw mut statusline_buffer, false_0 != 0);
    1 as ::core::ffi::c_int
}
unsafe fn stdin_start_map(context: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    let ctx: *mut parser_ctx = context as *mut parser_ctx;
    memset(
        &raw mut (*ctx).block as *mut ::core::ffi::c_void,
        '\0' as i32,
        ::core::mem::size_of::<status_block>() as usize,
    );
    if config.separator_symbol.is_null() {
        (*ctx).block.sep_block_width = logical_px(9 as ::core::ffi::c_int) as u32;
    } else {
        (*ctx).block.sep_block_width =
            (logical_px(8 as ::core::ffi::c_int) + separator_symbol_width) as u32;
    }
    (*ctx).block.border_top = 1 as u32;
    (*ctx).block.border_right = 1 as u32;
    (*ctx).block.border_bottom = 1 as u32;
    (*ctx).block.border_left = 1 as u32;
    1 as ::core::ffi::c_int
}
unsafe fn stdin_map_key(
    context: *mut ::core::ffi::c_void,
    key: *const ::core::ffi::c_uchar,
    len: usize,
) -> ::core::ffi::c_int {
    let ctx: *mut parser_ctx = context as *mut parser_ctx;
    free((*ctx).last_map_key as *mut ::core::ffi::c_void);
    (*ctx).last_map_key = ::core::ptr::null_mut::<::core::ffi::c_char>();
    sasprintf(
        &raw mut (*ctx).last_map_key,
        c"%.*s".as_ptr(),
        len as ::core::ffi::c_int,
        key,
    );
    1 as ::core::ffi::c_int
}
unsafe fn stdin_boolean(
    context: *mut ::core::ffi::c_void,
    val: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let ctx: *mut parser_ctx = context as *mut parser_ctx;
    if (*ctx).last_map_key.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if strcasecmp((*ctx).last_map_key, c"urgent".as_ptr()) == 0 as ::core::ffi::c_int {
        (*ctx).block.urgent = val != 0;
        return 1 as ::core::ffi::c_int;
    }
    if strcasecmp((*ctx).last_map_key, c"separator".as_ptr()) == 0 as ::core::ffi::c_int {
        (*ctx).block.no_separator = val == 0;
        return 1 as ::core::ffi::c_int;
    }
    1 as ::core::ffi::c_int
}
unsafe fn stdin_string(
    context: *mut ::core::ffi::c_void,
    val: *const ::core::ffi::c_uchar,
    len: usize,
) -> ::core::ffi::c_int {
    let ctx: *mut parser_ctx = context as *mut parser_ctx;
    if (*ctx).last_map_key.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if strcasecmp((*ctx).last_map_key, c"full_text".as_ptr()) == 0 as ::core::ffi::c_int {
        (*ctx).block.full_text =
            i3string_from_markup_with_length(val as *const ::core::ffi::c_char, len);
        return 1 as ::core::ffi::c_int;
    }
    if strcasecmp((*ctx).last_map_key, c"short_text".as_ptr()) == 0 as ::core::ffi::c_int {
        (*ctx).block.short_text =
            i3string_from_markup_with_length(val as *const ::core::ffi::c_char, len);
        return 1 as ::core::ffi::c_int;
    }
    if strcasecmp((*ctx).last_map_key, c"color".as_ptr()) == 0 as ::core::ffi::c_int {
        sasprintf(
            &raw mut (*ctx).block.color,
            c"%.*s".as_ptr(),
            len as ::core::ffi::c_int,
            val,
        );
        return 1 as ::core::ffi::c_int;
    }
    if strcasecmp((*ctx).last_map_key, c"background".as_ptr()) == 0 as ::core::ffi::c_int {
        sasprintf(
            &raw mut (*ctx).block.background,
            c"%.*s".as_ptr(),
            len as ::core::ffi::c_int,
            val,
        );
        return 1 as ::core::ffi::c_int;
    }
    if strcasecmp((*ctx).last_map_key, c"border".as_ptr()) == 0 as ::core::ffi::c_int {
        sasprintf(
            &raw mut (*ctx).block.border,
            c"%.*s".as_ptr(),
            len as ::core::ffi::c_int,
            val,
        );
        return 1 as ::core::ffi::c_int;
    }
    if strcasecmp((*ctx).last_map_key, c"markup".as_ptr()) == 0 as ::core::ffi::c_int {
        (*ctx).block.pango_markup = len == strlen(c"pango".as_ptr())
            && strncasecmp(
                val as *const ::core::ffi::c_char,
                c"pango".as_ptr(),
                strlen(c"pango".as_ptr()),
            ) == 0;
        return 1 as ::core::ffi::c_int;
    }
    if strcasecmp((*ctx).last_map_key, c"align".as_ptr()) == 0 as ::core::ffi::c_int {
        if len == strlen(c"center".as_ptr())
            && strncmp(
                val as *const ::core::ffi::c_char,
                c"center".as_ptr(),
                strlen(c"center".as_ptr()),
            ) == 0
        {
            (*ctx).block.align = crate::child::BlockAlign::Center;
        } else if len == strlen(c"right".as_ptr())
            && strncmp(
                val as *const ::core::ffi::c_char,
                c"right".as_ptr(),
                strlen(c"right".as_ptr()),
            ) == 0
        {
            (*ctx).block.align = crate::child::BlockAlign::Right;
        } else {
            (*ctx).block.align = crate::child::BlockAlign::Left;
        }
        return 1 as ::core::ffi::c_int;
    }
    if strcasecmp((*ctx).last_map_key, c"min_width".as_ptr()) == 0 as ::core::ffi::c_int {
        sasprintf(
            &raw mut (*ctx).block.min_width_str,
            c"%.*s".as_ptr(),
            len as ::core::ffi::c_int,
            val,
        );
        return 1 as ::core::ffi::c_int;
    }
    if strcasecmp((*ctx).last_map_key, c"name".as_ptr()) == 0 as ::core::ffi::c_int {
        sasprintf(
            &raw mut (*ctx).block.name,
            c"%.*s".as_ptr(),
            len as ::core::ffi::c_int,
            val,
        );
        return 1 as ::core::ffi::c_int;
    }
    if strcasecmp((*ctx).last_map_key, c"instance".as_ptr()) == 0 as ::core::ffi::c_int {
        sasprintf(
            &raw mut (*ctx).block.instance,
            c"%.*s".as_ptr(),
            len as ::core::ffi::c_int,
            val,
        );
        return 1 as ::core::ffi::c_int;
    }
    1 as ::core::ffi::c_int
}
unsafe fn stdin_integer(
    context: *mut ::core::ffi::c_void,
    val: ::core::ffi::c_longlong,
) -> ::core::ffi::c_int {
    let ctx: *mut parser_ctx = context as *mut parser_ctx;
    if (*ctx).last_map_key.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if strcasecmp((*ctx).last_map_key, c"min_width".as_ptr()) == 0 as ::core::ffi::c_int {
        (*ctx).block.min_width = val as u32;
        return 1 as ::core::ffi::c_int;
    }
    if strcasecmp((*ctx).last_map_key, c"separator_block_width".as_ptr()) == 0 as ::core::ffi::c_int
    {
        (*ctx).block.sep_block_width = val as u32;
        return 1 as ::core::ffi::c_int;
    }
    if strcasecmp((*ctx).last_map_key, c"border_top".as_ptr()) == 0 as ::core::ffi::c_int {
        (*ctx).block.border_top = val as u32;
        return 1 as ::core::ffi::c_int;
    }
    if strcasecmp((*ctx).last_map_key, c"border_right".as_ptr()) == 0 as ::core::ffi::c_int {
        (*ctx).block.border_right = val as u32;
        return 1 as ::core::ffi::c_int;
    }
    if strcasecmp((*ctx).last_map_key, c"border_bottom".as_ptr()) == 0 as ::core::ffi::c_int {
        (*ctx).block.border_bottom = val as u32;
        return 1 as ::core::ffi::c_int;
    }
    if strcasecmp((*ctx).last_map_key, c"border_left".as_ptr()) == 0 as ::core::ffi::c_int {
        (*ctx).block.border_left = val as u32;
        return 1 as ::core::ffi::c_int;
    }
    1 as ::core::ffi::c_int
}
unsafe fn stdin_end_map(context: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    let ctx: *mut parser_ctx = context as *mut parser_ctx;
    let new_block: *mut status_block =
        smalloc(::core::mem::size_of::<status_block>() as usize) as *mut status_block;
    memcpy(
        new_block as *mut ::core::ffi::c_void,
        &raw mut (*ctx).block as *const ::core::ffi::c_void,
        ::core::mem::size_of::<status_block>() as usize,
    );
    if (*new_block).full_text.is_null() {
        (*new_block).full_text = i3string_from_utf8(c"SPEC VIOLATION: full_text is NULL!".as_ptr());
    }
    if (*new_block).urgent {
        (*ctx).has_urgent = true_0 != 0;
    }
    if !(*new_block).min_width_str.is_null() {
        let text: *mut i3String = i3string_from_utf8((*new_block).min_width_str);
        i3string_set_markup(text, (*new_block).pango_markup);
        (*new_block).min_width = predict_text_width(text) as u32;
        i3string_free(text);
    }
    i3string_set_markup((*new_block).full_text, (*new_block).pango_markup);
    (*new_block).use_short = false_0 != 0;
    if !(*new_block).short_text.is_null() {
        i3string_set_markup((*new_block).short_text, (*new_block).pango_markup);
    }
    (*new_block).blocks.tqe_next = ::core::ptr::null_mut::<status_block>();
    (*new_block).blocks.tqe_prev = statusline_buffer.tqh_last;
    *statusline_buffer.tqh_last = new_block;
    statusline_buffer.tqh_last = &raw mut (*new_block).blocks.tqe_next;
    1 as ::core::ffi::c_int
}
unsafe fn stdin_end_array(context: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    if config.verbose {
        printf(
            c"[%s:%d] copying statusline_buffer to statusline_head\n".as_ptr(),
            c"../i3bar/src/child.c".as_ptr(),
            374 as ::core::ffi::c_int,
        );
    }
    clear_statusline(&raw mut statusline_head, true_0 != 0);
    copy_statusline(&raw mut statusline_buffer, &raw mut statusline_head);
    if config.verbose {
        printf(
            c"[%s:%d] dumping statusline:\n".as_ptr(),
            c"../i3bar/src/child.c".as_ptr(),
            378 as ::core::ffi::c_int,
        );
    }
    let mut current: *mut status_block = ::core::ptr::null_mut::<status_block>();
    current = statusline_head.tqh_first;
    while !current.is_null() {
        if config.verbose {
            printf(
                c"[%s:%d] full_text = %s\n".as_ptr(),
                c"../i3bar/src/child.c".as_ptr(),
                381 as ::core::ffi::c_int,
                i3string_as_utf8((*current).full_text),
            );
        }
        if config.verbose {
            printf(
                c"[%s:%d] short_text = %s\n".as_ptr(),
                c"../i3bar/src/child.c".as_ptr(),
                382 as ::core::ffi::c_int,
                if (*current).short_text.is_null() {
                    ::core::ptr::null::<::core::ffi::c_char>()
                } else {
                    i3string_as_utf8((*current).short_text)
                },
            );
        }
        if config.verbose {
            printf(
                c"[%s:%d] color = %s\n".as_ptr(),
                c"../i3bar/src/child.c".as_ptr(),
                383 as ::core::ffi::c_int,
                (*current).color,
            );
        }
        current = (*current).blocks.tqe_next;
    }
    if config.verbose {
        printf(
            c"[%s:%d] end of dump\n".as_ptr(),
            c"../i3bar/src/child.c".as_ptr(),
            385 as ::core::ffi::c_int,
        );
    }
    1 as ::core::ffi::c_int
}
unsafe fn get_buffer(
    fd: ::core::ffi::c_int,
    ret_buffer_len: *mut ::core::ffi::c_int,
) -> *mut ::core::ffi::c_uchar {
    let mut rec: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut buffer_len: ::core::ffi::c_int = STDIN_CHUNK_SIZE;
    let mut buffer: *mut ::core::ffi::c_uchar =
        smalloc((buffer_len + 1 as ::core::ffi::c_int) as usize) as *mut ::core::ffi::c_uchar;
    *buffer.offset(0 as ::core::ffi::c_int as isize) = '\0' as i32 as ::core::ffi::c_uchar;
    loop {
        let n: isize = read(
            fd,
            buffer.offset(rec as isize) as *mut ::core::ffi::c_void,
            (buffer_len - rec) as usize,
        ) as isize;
        if n == -(1 as ::core::ffi::c_int) as isize {
            if *__errno_location() == libc::EAGAIN {
                break;
            }
            fprintf(
                stderr,
                c"[%s:%d] ERROR: read() failed!: %s\n".as_ptr(),
                c"../i3bar/src/child.c".as_ptr(),
                407 as ::core::ffi::c_int,
                strerror(*__errno_location()),
            );
            free(buffer as *mut ::core::ffi::c_void);
            buffer = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
            exit(libc::EXIT_FAILURE);
        } else {
            if n == 0 as isize {
                fprintf(
                    stderr,
                    c"[%s:%d] ERROR: stdin: received EOF\n".as_ptr(),
                    c"../i3bar/src/child.c".as_ptr(),
                    412 as ::core::ffi::c_int,
                );
                free(buffer as *mut ::core::ffi::c_void);
                buffer = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                *ret_buffer_len = -(1 as ::core::ffi::c_int);
                return ::core::ptr::null_mut::<::core::ffi::c_uchar>();
            }
            rec = (rec as isize + n) as ::core::ffi::c_int;
            if rec == buffer_len {
                buffer_len += STDIN_CHUNK_SIZE;
                buffer = srealloc(
                    buffer as *mut ::core::ffi::c_void,
                    (buffer_len + 1 as ::core::ffi::c_int) as usize,
                ) as *mut ::core::ffi::c_uchar;
            }
        }
    }
    *buffer.offset(rec as isize) = '\0' as i32 as ::core::ffi::c_uchar;
    if *buffer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '\0' as i32 {
        free(buffer as *mut ::core::ffi::c_void);
        buffer = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
        rec = -(1 as ::core::ffi::c_int);
    }
    *ret_buffer_len = rec;
    buffer
}
unsafe fn read_flat_input(buffer: *mut ::core::ffi::c_char, length: ::core::ffi::c_int) {
    let first: *mut status_block = statusline_head.tqh_first;
    i3string_free((*first).full_text);
    (*first).full_text = ::core::ptr::null_mut::<i3String>();
    if *buffer.offset((length - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
        == '\n' as i32
        || *buffer.offset((length - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            == '\r' as i32
    {
        *buffer.offset((length - 1 as ::core::ffi::c_int) as isize) =
            '\0' as i32 as ::core::ffi::c_char;
    } else {
        *buffer.offset(length as isize) = '\0' as i32 as ::core::ffi::c_char;
    }
    (*first).full_text = i3string_from_utf8(buffer);
}
unsafe fn read_json_input(input: *mut ::core::ffi::c_uchar, length: ::core::ffi::c_int) -> bool {
    let status: yajl_status = yajl_parse(parser, input, length as usize);
    let mut has_urgent: bool = false_0 != 0;
    if status as ::core::ffi::c_uint != yajl_status_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let message: *mut ::core::ffi::c_char =
            yajl_get_error(parser, 0 as ::core::ffi::c_int, input, length as usize)
                as *mut ::core::ffi::c_char;
        if *message.offset(strlen(message).wrapping_sub(1 as usize) as isize) as ::core::ffi::c_int
            == '\n' as i32
        {
            *message.offset(strlen(message).wrapping_sub(1 as usize) as isize) =
                '\0' as i32 as ::core::ffi::c_char;
        }
        fprintf(
            stderr,
            c"[i3bar] Could not parse JSON input (code = %d, message = %s): %.*s\n".as_ptr(),
            status as ::core::ffi::c_uint,
            message,
            length,
            input,
        );
        set_statusline_error(c"Could not parse JSON (%s)".as_ptr(), message);
        yajl_free_error(parser, message as *mut ::core::ffi::c_uchar);
        draw_bars(false_0 != 0);
    } else if parser_context.has_urgent {
        has_urgent = true_0 != 0;
    }
    has_urgent
}
unsafe fn stdin_io_cb(fd: ::core::ffi::c_int) {
    let mut rec: ::core::ffi::c_int = 0;
    let buffer: *mut ::core::ffi::c_uchar = get_buffer(fd, &raw mut rec);
    if buffer.is_null() {
        return;
    }
    let mut has_urgent: bool = false_0 != 0;
    if status_child.version > 0 as u32 {
        has_urgent = read_json_input(buffer, rec);
    } else {
        read_flat_input(buffer as *mut ::core::ffi::c_char, rec);
    }
    free(buffer as *mut ::core::ffi::c_void);
    draw_bars(has_urgent);
}
unsafe fn stdin_io_first_line_cb(fd: ::core::ffi::c_int) {
    let mut rec: ::core::ffi::c_int = 0;
    let buffer: *mut ::core::ffi::c_uchar = get_buffer(fd, &raw mut rec);
    if buffer.is_null() {
        return;
    }
    if config.verbose {
        printf(
            c"[%s:%d] Detecting input type based on buffer *%.*s*\n".as_ptr(),
            c"../i3bar/src/child.c".as_ptr(),
            503 as ::core::ffi::c_int,
            rec,
            buffer,
        );
    }
    // get_buffer owns rec initialized bytes until the matching free below.
    let header = parse_json_header(std::slice::from_raw_parts(buffer, rec as usize));
    status_child.version = header.version;
    status_child.stop_signal = header.stop_signal;
    status_child.cont_signal = header.cont_signal;
    if let Some(click_events) = header.click_events {
        status_child.click_events = click_events;
    }
    let consumed = header.consumed;
    if status_child.version > 0 as u32 {
        if config.hide_on_modifier != crate::application::BarDisplayMode::Dock {
            stop_children();
        }
        draw_bars(read_json_input(
            buffer.offset(consumed as isize),
            (rec as usize - consumed) as ::core::ffi::c_int,
        ));
    } else {
        let new_block: *mut status_block =
            scalloc(1 as usize, ::core::mem::size_of::<status_block>() as usize)
                as *mut status_block;
        (*new_block).blocks.tqe_next = ::core::ptr::null_mut::<status_block>();
        (*new_block).blocks.tqe_prev = statusline_head.tqh_last;
        *statusline_head.tqh_last = new_block;
        statusline_head.tqh_last = &raw mut (*new_block).blocks.tqe_next;
        read_flat_input(buffer as *mut ::core::ffi::c_char, rec);
    }
    free(buffer as *mut ::core::ffi::c_void);
}
unsafe fn isempty(mut s: *mut ::core::ffi::c_char) -> bool {
    while *s as ::core::ffi::c_int != '\0' as i32 {
        if *(*__ctype_b_loc()).offset(*s as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & i3_common::native_stdio::CTYPE_SPACE as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            == 0
        {
            return false_0 != 0;
        }
        s = s.offset(1);
    }
    true_0 != 0
}
unsafe fn append_string(
    previous: *const ::core::ffi::c_char,
    str: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    if !previous.is_null() {
        let mut result: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        sasprintf(&raw mut result, c"%s%s".as_ptr(), previous, str);
        return result;
    }
    sstrdup(str)
}
static mut ws_last_json: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
unsafe fn ws_stdin_io_cb(fd: ::core::ffi::c_int) {
    let mut rec: ::core::ffi::c_int = 0;
    let buffer: *mut ::core::ffi::c_uchar = get_buffer(fd, &raw mut rec);
    if buffer.is_null() {
        return;
    }
    // Empty input yields no fields; trailing empty fields otherwise remain so
    // a final incomplete line is carried across callbacks exactly as before.
    let bytes = std::ffi::CStr::from_ptr(buffer.cast()).to_bytes();
    let lines: Vec<_> = if bytes.is_empty() {
        Vec::new()
    } else {
        bytes
            .split(|&byte| byte == b'\n')
            .map(|line| std::ffi::CString::new(line).expect("C input has no interior NUL"))
            .collect()
    };
    for (index, line) in lines.iter().enumerate() {
        if ws_child.pending_line.is_null() && line.as_bytes().is_empty() {
            continue;
        }
        if index + 1 == lines.len() {
            let next = append_string(ws_child.pending_line, line.as_ptr());
            free(ws_child.pending_line.cast());
            ws_child.pending_line = next;
        } else {
            free(ws_last_json.cast());
            ws_last_json = append_string(ws_child.pending_line, line.as_ptr());
            free(ws_child.pending_line.cast());
            ws_child.pending_line = std::ptr::null_mut();
            parse_workspaces_json(ws_last_json.cast(), strlen(ws_last_json));
        }
    }
    free(buffer as *mut ::core::ffi::c_void);
    draw_bars(false_0 != 0);
}
unsafe extern "C" fn common_stdin_cb(
    loop_0: *mut ev_loop,
    watcher: *mut ev_io,
    revents: ::core::ffi::c_int,
) {
    if watcher == status_child.stdin_io {
        if status_child.version == -(1 as ::core::ffi::c_int) as u32 {
            stdin_io_first_line_cb((*watcher).fd);
        } else {
            stdin_io_cb((*watcher).fd);
        }
    } else if watcher == ws_child.stdin_io {
        ws_stdin_io_cb((*watcher).fd);
    } else {
        fprintf(
            stderr,
            c"[%s:%d] ERROR: Got callback for unknown watcher fd=%d\n".as_ptr(),
            c"../i3bar/src/child.c".as_ptr(),
            594 as ::core::ffi::c_int,
            (*watcher).fd,
        );
    };
}

pub unsafe fn repeat_last_ws_json() {
    if !ws_last_json.is_null() {
        if config.verbose {
            printf(
                c"[%s:%d] Repeating last workspace JSON\n".as_ptr(),
                c"../i3bar/src/child.c".as_ptr(),
                604 as ::core::ffi::c_int,
            );
        }
        parse_workspaces_json(
            ws_last_json as *const ::core::ffi::c_uchar,
            strlen(ws_last_json),
        );
    }
}
unsafe extern "C" fn set_workspace_button_error_f(format: *const ::core::ffi::c_char, args: ...) {
    let mut message: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut args_0;
    args_0 = args.clone();
    if !(vasprintf(&raw mut message, format, i333_va_list!(args_0)) == -(1 as ::core::ffi::c_int)) {
        set_workspace_button_error(message);
    }
    free(message as *mut ::core::ffi::c_void);
}

pub unsafe fn set_workspace_button_error(message: *const ::core::ffi::c_char) {
    free_workspaces();
    let mut text = b"Error: ".to_vec();
    text.extend_from_slice(std::ffi::CStr::from_ptr(message).to_bytes());
    let name = i3_common::string::I3String::from_bytes(&text);
    let mut output = (*outputs).slh_first;
    while !output.is_null() {
        let text = name.clone();
        let width = predict_text_width((&text as *const i3_common::string::I3String).cast_mut());
        crate::workspaces::append_workspace(output, Box::new(i3_ws::error(text, width)));
        output = (*output).slist.sle_next;
    }
}
unsafe extern "C" fn child_sig_cb(
    loop_0: *mut ev_loop,
    watcher: *mut ev_child,
    revents: ::core::ffi::c_int,
) {
    let exit_status: ::core::ffi::c_int =
        ((*watcher).rstatus & 0xff00 as ::core::ffi::c_int) >> 8 as ::core::ffi::c_int;
    fprintf(
        stderr,
        c"[%s:%d] ERROR: Child (pid: %d) unexpectedly exited with status %d\n".as_ptr(),
        c"../i3bar/src/child.c".as_ptr(),
        664 as ::core::ffi::c_int,
        (*watcher).pid,
        exit_status,
    );
    let mut error_function_pointer: Option<
        unsafe extern "C" fn(*const ::core::ffi::c_char, ...) -> (),
    > = None;
    let mut command_type: *const ::core::ffi::c_char = c"".as_ptr();
    let mut c: *mut i3bar_child = ::core::ptr::null_mut::<i3bar_child>();
    if (*watcher).pid == status_child.pid {
        command_type = c"status_command".as_ptr();
        error_function_pointer = Some(
            set_statusline_error as unsafe extern "C" fn(*const ::core::ffi::c_char, ...) -> (),
        )
            as Option<unsafe extern "C" fn(*const ::core::ffi::c_char, ...) -> ()>;
        c = &raw mut status_child;
    } else if (*watcher).pid == ws_child.pid {
        command_type = c"workspace_command".as_ptr();
        error_function_pointer = Some(
            set_workspace_button_error_f
                as unsafe extern "C" fn(*const ::core::ffi::c_char, ...) -> (),
        )
            as Option<unsafe extern "C" fn(*const ::core::ffi::c_char, ...) -> ()>;
        c = &raw mut ws_child;
    } else {
        fprintf(
            stderr,
            c"[%s:%d] ERROR: Unknown child pid, this should never happen\n".as_ptr(),
            c"../i3bar/src/child.c".as_ptr(),
            678 as ::core::ffi::c_int,
        );
        return;
    }
    if (*c).pid == 0 as ::core::ffi::c_int {
        if config.verbose {
            printf(
                c"[%s:%d] %s: child pid = 0\n".as_ptr(),
                c"../i3bar/src/child.c".as_ptr(),
                681 as ::core::ffi::c_int,
                c"child_sig_cb".as_ptr(),
            );
        }
    } else if (*c).pid == status_child.pid {
        if config.verbose {
            printf(
                c"[%s:%d] %s: status_command: pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n".as_ptr(),
                c"../i3bar/src/child.c".as_ptr(),
                681 as ::core::ffi::c_int,
                c"child_sig_cb".as_ptr(),
                (*c).pid as ::core::ffi::c_long,
                (*c).stopped as ::core::ffi::c_int,
                (*c).stop_signal,
                (*c).cont_signal,
                (*c).click_events as ::core::ffi::c_int,
                (*c).click_events_init as ::core::ffi::c_int,
            );
        }
    } else if (*c).pid == ws_child.pid {
        if config.verbose {
            printf(
                c"[%s:%d] %s: workspace_command: pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n".as_ptr(),
                c"../i3bar/src/child.c".as_ptr(),
                681 as ::core::ffi::c_int,
                c"child_sig_cb".as_ptr(),
                (*c).pid as ::core::ffi::c_long,
                (*c).stopped as ::core::ffi::c_int,
                (*c).stop_signal,
                (*c).cont_signal,
                (*c).click_events as ::core::ffi::c_int,
                (*c).click_events_init as ::core::ffi::c_int,
            );
        }
    } else {
        fprintf(
            stderr,
            c"[%s:%d] ERROR: %s: unknown child, this should never happen pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n".as_ptr(),
            c"../i3bar/src/child.c".as_ptr(),
            681 as ::core::ffi::c_int,
            c"child_sig_cb".as_ptr(),
            (*c).pid as ::core::ffi::c_long,
            (*c).stopped as ::core::ffi::c_int,
            (*c).stop_signal,
            (*c).cont_signal,
            (*c).click_events as ::core::ffi::c_int,
            (*c).click_events_init as ::core::ffi::c_int,
        );
    }
    if exit_status == 126 as ::core::ffi::c_int {
        error_function_pointer.expect("non-null function pointer")(
            c"%s is not executable (exit %d)".as_ptr(),
            command_type,
            exit_status,
        );
    } else if exit_status == 127 as ::core::ffi::c_int {
        error_function_pointer.expect("non-null function pointer")(
            c"%s not found or is missing a library dependency (exit %d)".as_ptr(),
            command_type,
            exit_status,
        );
    } else {
        error_function_pointer.expect("non-null function pointer")(
            c"%s process exited unexpectedly (exit %d)".as_ptr(),
            command_type,
            exit_status,
        );
    }
    cleanup(c);
    draw_bars(false_0 != 0);
}
unsafe fn child_write_output() {
    if status_child.click_events {
        let mut output: *const ::core::ffi::c_uchar = ::core::ptr::null::<::core::ffi::c_uchar>();
        let mut size: usize = 0;
        let mut n: isize = 0;
        yajl_gen_get_buf(gen, &raw mut output, &raw mut size);
        n = writeall(child_stdin, output as *const ::core::ffi::c_void, size);
        if n != -(1 as ::core::ffi::c_int) as isize {
            n = writeall(
                child_stdin,
                c"\n".as_ptr() as *const ::core::ffi::c_void,
                1 as usize,
            );
        }
        yajl_gen_clear(gen);
        if n == -(1 as ::core::ffi::c_int) as isize {
            status_child.click_events = false_0 != 0;
            kill_child();
            set_statusline_error(c"child_write_output failed".as_ptr());
            draw_bars(false_0 != 0);
        }
    }
}
unsafe fn sfork() -> libc::pid_t {
    let pid: libc::pid_t = fork() as libc::pid_t;
    if pid == -(1 as ::core::ffi::c_int) {
        fprintf(
            stderr,
            c"[%s:%d] ERROR: Couldn't fork(): %s\n".as_ptr(),
            c"../i3bar/src/child.c".as_ptr(),
            724 as ::core::ffi::c_int,
            strerror(*__errno_location()),
        );
        exit(libc::EXIT_FAILURE);
    }
    pid
}
unsafe fn spipe(pipedes: *mut ::core::ffi::c_int) {
    if pipe(pipedes) == -(1 as ::core::ffi::c_int) {
        err(libc::EXIT_FAILURE, c"pipe(pipe_in)".as_ptr());
    }
}
unsafe fn exec_shell(command: *mut ::core::ffi::c_char) {
    execl(
        _PATH_BSHELL.as_ptr(),
        _PATH_BSHELL.as_ptr(),
        c"-c".as_ptr(),
        command,
        NULL as *mut ::core::ffi::c_char,
    );
    err(libc::EXIT_FAILURE, c"execl return".as_ptr());
}
unsafe fn setup_child_cb(child: *mut i3bar_child) {
    fcntl((*child).stdin_fd, libc::F_SETFL, libc::O_NONBLOCK);
    (*child).stdin_io = smalloc(::core::mem::size_of::<ev_io>() as usize) as *mut ev_io;
    let ref mut fresh0 =
        (*((*child).stdin_io as *mut ::core::ffi::c_void as *mut ev_watcher)).pending;
    *fresh0 = 0 as ::core::ffi::c_int;
    (*((*child).stdin_io as *mut ::core::ffi::c_void as *mut ev_watcher)).active = *fresh0;
    (*((*child).stdin_io as *mut ::core::ffi::c_void as *mut ev_watcher)).priority =
        0 as ::core::ffi::c_int;
    (*(*child).stdin_io).cb = Some(
        common_stdin_cb as unsafe extern "C" fn(*mut ev_loop, *mut ev_io, ::core::ffi::c_int) -> (),
    )
        as Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_io, ::core::ffi::c_int) -> ()>;
    memmove(
        &raw mut (*((*child).stdin_io as *mut ev_watcher)).cb as *mut ::core::ffi::c_void,
        &raw mut (*(*child).stdin_io).cb as *const ::core::ffi::c_void,
        ::core::mem::size_of::<
            Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_io, ::core::ffi::c_int) -> ()>,
        >() as usize,
    );
    (*(*child).stdin_io).fd = (*child).stdin_fd;
    (*(*child).stdin_io).events = EV_READ as ::core::ffi::c_int | EV__IOFDSET as ::core::ffi::c_int;
    ev_io_start(main_loop as *mut ev_loop, (*child).stdin_io);
    (*child).child_sig = smalloc(::core::mem::size_of::<ev_child>() as usize) as *mut ev_child;
    let ref mut fresh1 =
        (*((*child).child_sig as *mut ::core::ffi::c_void as *mut ev_watcher)).pending;
    *fresh1 = 0 as ::core::ffi::c_int;
    (*((*child).child_sig as *mut ::core::ffi::c_void as *mut ev_watcher)).active = *fresh1;
    (*((*child).child_sig as *mut ::core::ffi::c_void as *mut ev_watcher)).priority =
        0 as ::core::ffi::c_int;
    (*(*child).child_sig).cb = Some(
        child_sig_cb as unsafe extern "C" fn(*mut ev_loop, *mut ev_child, ::core::ffi::c_int) -> (),
    )
        as Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_child, ::core::ffi::c_int) -> ()>;
    memmove(
        &raw mut (*((*child).child_sig as *mut ev_watcher)).cb as *mut ::core::ffi::c_void,
        &raw mut (*(*child).child_sig).cb as *const ::core::ffi::c_void,
        ::core::mem::size_of::<
            Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_child, ::core::ffi::c_int) -> ()>,
        >() as usize,
    );
    (*(*child).child_sig).pid = (*child).pid as ::core::ffi::c_int;
    (*(*child).child_sig).flags = (0 as ::core::ffi::c_int != 0) as ::core::ffi::c_int;
    ev_child_start(main_loop as *mut ev_loop, (*child).child_sig);
    if (*child).pid == 0 as ::core::ffi::c_int {
        if config.verbose {
            printf(
                c"[%s:%d] %s: child pid = 0\n".as_ptr(),
                c"../i3bar/src/child.c".as_ptr(),
                754 as ::core::ffi::c_int,
                c"setup_child_cb".as_ptr(),
            );
        }
    } else if (*child).pid == status_child.pid {
        if config.verbose {
            printf(
                c"[%s:%d] %s: status_command: pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n".as_ptr(),
                c"../i3bar/src/child.c".as_ptr(),
                754 as ::core::ffi::c_int,
                c"setup_child_cb".as_ptr(),
                (*child).pid as ::core::ffi::c_long,
                (*child).stopped as ::core::ffi::c_int,
                (*child).stop_signal,
                (*child).cont_signal,
                (*child).click_events as ::core::ffi::c_int,
                (*child).click_events_init as ::core::ffi::c_int,
            );
        }
    } else if (*child).pid == ws_child.pid {
        if config.verbose {
            printf(
                c"[%s:%d] %s: workspace_command: pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n".as_ptr(),
                c"../i3bar/src/child.c".as_ptr(),
                754 as ::core::ffi::c_int,
                c"setup_child_cb".as_ptr(),
                (*child).pid as ::core::ffi::c_long,
                (*child).stopped as ::core::ffi::c_int,
                (*child).stop_signal,
                (*child).cont_signal,
                (*child).click_events as ::core::ffi::c_int,
                (*child).click_events_init as ::core::ffi::c_int,
            );
        }
    } else {
        fprintf(
            stderr,
            c"[%s:%d] ERROR: %s: unknown child, this should never happen pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n".as_ptr(),
            c"../i3bar/src/child.c".as_ptr(),
            754 as ::core::ffi::c_int,
            c"setup_child_cb".as_ptr(),
            (*child).pid as ::core::ffi::c_long,
            (*child).stopped as ::core::ffi::c_int,
            (*child).stop_signal,
            (*child).cont_signal,
            (*child).click_events as ::core::ffi::c_int,
            (*child).click_events_init as ::core::ffi::c_int,
        );
    };
}

pub unsafe fn start_child(command: *mut ::core::ffi::c_char) {
    if command.is_null() {
        return;
    }
    static mut callbacks: yajl_callbacks = unsafe {
        yajl_callbacks {
            yajl_null: None,
            yajl_boolean: Some(
                stdin_boolean
                    as unsafe fn(
                        *mut ::core::ffi::c_void,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
            yajl_integer: Some(
                stdin_integer
                    as unsafe fn(
                        *mut ::core::ffi::c_void,
                        ::core::ffi::c_longlong,
                    ) -> ::core::ffi::c_int,
            ),
            yajl_double: None,
            yajl_number: None,
            yajl_string: Some(
                stdin_string
                    as unsafe fn(
                        *mut ::core::ffi::c_void,
                        *const ::core::ffi::c_uchar,
                        usize,
                    ) -> ::core::ffi::c_int,
            ),
            yajl_start_map: Some(
                stdin_start_map as unsafe fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
            ),
            yajl_map_key: Some(
                stdin_map_key
                    as unsafe fn(
                        *mut ::core::ffi::c_void,
                        *const ::core::ffi::c_uchar,
                        usize,
                    ) -> ::core::ffi::c_int,
            ),
            yajl_end_map: Some(
                stdin_end_map as unsafe fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
            ),
            yajl_start_array: Some(
                stdin_start_array as unsafe fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
            ),
            yajl_end_array: Some(
                stdin_end_array as unsafe fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
            ),
        }
    };
    parser = yajl_alloc(
        &raw mut callbacks,
        ::core::ptr::null_mut::<yajl_alloc_funcs>(),
        &raw mut parser_context as *mut ::core::ffi::c_void,
    );
    gen = yajl_gen_alloc(::core::ptr::null::<yajl_alloc_funcs>());
    let mut pipe_in: [::core::ffi::c_int; 2] = [0; 2];
    let mut pipe_out: [::core::ffi::c_int; 2] = [0; 2];
    spipe(&raw mut pipe_in as *mut ::core::ffi::c_int);
    spipe(&raw mut pipe_out as *mut ::core::ffi::c_int);
    status_child.pid = sfork();
    if status_child.pid == 0 as ::core::ffi::c_int {
        close(pipe_in[0 as ::core::ffi::c_int as usize]);
        close(pipe_out[1 as ::core::ffi::c_int as usize]);
        dup2(
            pipe_in[1 as ::core::ffi::c_int as usize],
            libc::STDOUT_FILENO,
        );
        dup2(
            pipe_out[0 as ::core::ffi::c_int as usize],
            libc::STDIN_FILENO,
        );
        setpgid(status_child.pid as libc::pid_t, 0 as libc::pid_t);
        exec_shell(command);
        return;
    }
    close(pipe_in[1 as ::core::ffi::c_int as usize]);
    close(pipe_out[0 as ::core::ffi::c_int as usize]);
    status_child.stdin_fd = pipe_in[0 as ::core::ffi::c_int as usize];
    child_stdin = pipe_out[1 as ::core::ffi::c_int as usize];
    status_child.version = -(1 as ::core::ffi::c_int) as u32;
    setup_child_cb(&raw mut status_child);
}

pub unsafe fn start_ws_child(command: *mut ::core::ffi::c_char) {
    if command.is_null() {
        return;
    }
    ws_child.stop_signal = libc::SIGSTOP;
    ws_child.cont_signal = libc::SIGCONT;
    let mut pipe_in: [::core::ffi::c_int; 2] = [0; 2];
    spipe(&raw mut pipe_in as *mut ::core::ffi::c_int);
    ws_child.pid = sfork();
    if ws_child.pid == 0 as ::core::ffi::c_int {
        close(pipe_in[0 as ::core::ffi::c_int as usize]);
        dup2(
            pipe_in[1 as ::core::ffi::c_int as usize],
            libc::STDOUT_FILENO,
        );
        setpgid(ws_child.pid as libc::pid_t, 0 as libc::pid_t);
        exec_shell(command);
    }
    close(pipe_in[1 as ::core::ffi::c_int as usize]);
    ws_child.stdin_fd = pipe_in[0 as ::core::ffi::c_int as usize];
    setup_child_cb(&raw mut ws_child);
}
unsafe fn child_click_events_initialize() {
    if status_child.pid == 0 as ::core::ffi::c_int {
        if config.verbose {
            printf(
                c"[%s:%d] %s: child pid = 0\n".as_ptr(),
                c"../i3bar/src/child.c".as_ptr(),
                847 as ::core::ffi::c_int,
                c"child_click_events_initialize".as_ptr(),
            );
        }
    } else if status_child.pid == status_child.pid {
        if config.verbose {
            printf(
                c"[%s:%d] %s: status_command: pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n".as_ptr(),
                c"../i3bar/src/child.c".as_ptr(),
                847 as ::core::ffi::c_int,
                c"child_click_events_initialize".as_ptr(),
                status_child.pid as ::core::ffi::c_long,
                status_child.stopped as ::core::ffi::c_int,
                status_child.stop_signal,
                status_child.cont_signal,
                status_child.click_events as ::core::ffi::c_int,
                status_child.click_events_init as ::core::ffi::c_int,
            );
        }
    } else if status_child.pid == ws_child.pid {
        if config.verbose {
            printf(
                c"[%s:%d] %s: workspace_command: pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n".as_ptr(),
                c"../i3bar/src/child.c".as_ptr(),
                847 as ::core::ffi::c_int,
                c"child_click_events_initialize".as_ptr(),
                status_child.pid as ::core::ffi::c_long,
                status_child.stopped as ::core::ffi::c_int,
                status_child.stop_signal,
                status_child.cont_signal,
                status_child.click_events as ::core::ffi::c_int,
                status_child.click_events_init as ::core::ffi::c_int,
            );
        }
    } else {
        fprintf(
            stderr,
            c"[%s:%d] ERROR: %s: unknown child, this should never happen pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n".as_ptr(),
            c"../i3bar/src/child.c".as_ptr(),
            847 as ::core::ffi::c_int,
            c"child_click_events_initialize".as_ptr(),
            status_child.pid as ::core::ffi::c_long,
            status_child.stopped as ::core::ffi::c_int,
            status_child.stop_signal,
            status_child.cont_signal,
            status_child.click_events as ::core::ffi::c_int,
            status_child.click_events_init as ::core::ffi::c_int,
        );
    }
    if !status_child.click_events_init {
        yajl_gen_array_open(gen);
        child_write_output();
        status_child.click_events_init = true_0 != 0;
    }
}

pub unsafe fn send_block_clicked(
    button: ::core::ffi::c_int,
    name: *const ::core::ffi::c_char,
    instance: *const ::core::ffi::c_char,
    x: ::core::ffi::c_int,
    y: ::core::ffi::c_int,
    x_rel: ::core::ffi::c_int,
    y_rel: ::core::ffi::c_int,
    out_x: ::core::ffi::c_int,
    out_y: ::core::ffi::c_int,
    width: ::core::ffi::c_int,
    height: ::core::ffi::c_int,
    mods: ::core::ffi::c_int,
) {
    if !status_child.click_events {
        return;
    }
    child_click_events_initialize();
    yajl_gen_map_open(gen);
    if !name.is_null() {
        yajl_gen_string(
            gen,
            c"name".as_ptr() as *mut ::core::ffi::c_uchar,
            strlen(c"name".as_ptr()),
        );
        yajl_gen_string(gen, name as *mut ::core::ffi::c_uchar, strlen(name));
    }
    if !instance.is_null() {
        yajl_gen_string(
            gen,
            c"instance".as_ptr() as *mut ::core::ffi::c_uchar,
            strlen(c"instance".as_ptr()),
        );
        yajl_gen_string(gen, instance as *mut ::core::ffi::c_uchar, strlen(instance));
    }
    yajl_gen_string(
        gen,
        c"button".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"button".as_ptr()),
    );
    yajl_gen_integer(gen, button as ::core::ffi::c_longlong);
    yajl_gen_string(
        gen,
        c"modifiers".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"modifiers".as_ptr()),
    );
    yajl_gen_array_open(gen);
    if mods & XCB_MOD_MASK_SHIFT as ::core::ffi::c_int != 0 {
        yajl_gen_string(
            gen,
            c"Shift".as_ptr() as *mut ::core::ffi::c_uchar,
            strlen(c"Shift".as_ptr()),
        );
    }
    if mods & XCB_MOD_MASK_CONTROL as ::core::ffi::c_int != 0 {
        yajl_gen_string(
            gen,
            c"Control".as_ptr() as *mut ::core::ffi::c_uchar,
            strlen(c"Control".as_ptr()),
        );
    }
    if mods & XCB_MOD_MASK_1 as ::core::ffi::c_int != 0 {
        yajl_gen_string(
            gen,
            c"Mod1".as_ptr() as *mut ::core::ffi::c_uchar,
            strlen(c"Mod1".as_ptr()),
        );
    }
    if mods & XCB_MOD_MASK_2 as ::core::ffi::c_int != 0 {
        yajl_gen_string(
            gen,
            c"Mod2".as_ptr() as *mut ::core::ffi::c_uchar,
            strlen(c"Mod2".as_ptr()),
        );
    }
    if mods & XCB_MOD_MASK_3 as ::core::ffi::c_int != 0 {
        yajl_gen_string(
            gen,
            c"Mod3".as_ptr() as *mut ::core::ffi::c_uchar,
            strlen(c"Mod3".as_ptr()),
        );
    }
    if mods & XCB_MOD_MASK_4 as ::core::ffi::c_int != 0 {
        yajl_gen_string(
            gen,
            c"Mod4".as_ptr() as *mut ::core::ffi::c_uchar,
            strlen(c"Mod4".as_ptr()),
        );
    }
    if mods & XCB_MOD_MASK_5 as ::core::ffi::c_int != 0 {
        yajl_gen_string(
            gen,
            c"Mod5".as_ptr() as *mut ::core::ffi::c_uchar,
            strlen(c"Mod5".as_ptr()),
        );
    }
    yajl_gen_array_close(gen);
    yajl_gen_string(
        gen,
        c"x".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"x".as_ptr()),
    );
    yajl_gen_integer(gen, x as ::core::ffi::c_longlong);
    yajl_gen_string(
        gen,
        c"y".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"y".as_ptr()),
    );
    yajl_gen_integer(gen, y as ::core::ffi::c_longlong);
    yajl_gen_string(
        gen,
        c"relative_x".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"relative_x".as_ptr()),
    );
    yajl_gen_integer(gen, x_rel as ::core::ffi::c_longlong);
    yajl_gen_string(
        gen,
        c"relative_y".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"relative_y".as_ptr()),
    );
    yajl_gen_integer(gen, y_rel as ::core::ffi::c_longlong);
    yajl_gen_string(
        gen,
        c"output_x".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"output_x".as_ptr()),
    );
    yajl_gen_integer(gen, out_x as ::core::ffi::c_longlong);
    yajl_gen_string(
        gen,
        c"output_y".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"output_y".as_ptr()),
    );
    yajl_gen_integer(gen, out_y as ::core::ffi::c_longlong);
    yajl_gen_string(
        gen,
        c"width".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"width".as_ptr()),
    );
    yajl_gen_integer(gen, width as ::core::ffi::c_longlong);
    yajl_gen_string(
        gen,
        c"height".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"height".as_ptr()),
    );
    yajl_gen_integer(gen, height as ::core::ffi::c_longlong);
    yajl_gen_map_close(gen);
    child_write_output();
}
unsafe fn is_alive(c: *mut i3bar_child) -> bool {
    (*c).pid > 0 as ::core::ffi::c_int
}

pub unsafe fn status_child_is_alive() -> bool {
    is_alive(&raw mut status_child)
}

pub unsafe fn ws_child_is_alive() -> bool {
    is_alive(&raw mut ws_child)
}

pub extern "C" fn kill_children_at_exit() {
    unsafe {
        if status_child.pid == 0 as ::core::ffi::c_int {
            if config.verbose {
                printf(
                    c"[%s:%d] %s: child pid = 0\n".as_ptr(),
                    c"../i3bar/src/child.c".as_ptr(),
                    960 as ::core::ffi::c_int,
                    c"kill_children_at_exit".as_ptr(),
                );
            }
        } else if status_child.pid == status_child.pid {
            if config.verbose {
                printf(
                c"[%s:%d] %s: status_command: pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n".as_ptr(),
                c"../i3bar/src/child.c".as_ptr(),
                960 as ::core::ffi::c_int,
                c"kill_children_at_exit".as_ptr(),
                status_child.pid as ::core::ffi::c_long,
                status_child.stopped as ::core::ffi::c_int,
                status_child.stop_signal,
                status_child.cont_signal,
                status_child.click_events as ::core::ffi::c_int,
                status_child.click_events_init as ::core::ffi::c_int,
            );
            }
        } else if status_child.pid == ws_child.pid {
            if config.verbose {
                printf(
                c"[%s:%d] %s: workspace_command: pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n".as_ptr(),
                c"../i3bar/src/child.c".as_ptr(),
                960 as ::core::ffi::c_int,
                c"kill_children_at_exit".as_ptr(),
                status_child.pid as ::core::ffi::c_long,
                status_child.stopped as ::core::ffi::c_int,
                status_child.stop_signal,
                status_child.cont_signal,
                status_child.click_events as ::core::ffi::c_int,
                status_child.click_events_init as ::core::ffi::c_int,
            );
            }
        } else {
            fprintf(
            stderr,
            c"[%s:%d] ERROR: %s: unknown child, this should never happen pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n".as_ptr(),
            c"../i3bar/src/child.c".as_ptr(),
            960 as ::core::ffi::c_int,
            c"kill_children_at_exit".as_ptr(),
            status_child.pid as ::core::ffi::c_long,
            status_child.stopped as ::core::ffi::c_int,
            status_child.stop_signal,
            status_child.cont_signal,
            status_child.click_events as ::core::ffi::c_int,
            status_child.click_events_init as ::core::ffi::c_int,
        );
        }
        if ws_child.pid == 0 as ::core::ffi::c_int {
            if config.verbose {
                printf(
                    c"[%s:%d] %s: child pid = 0\n".as_ptr(),
                    c"../i3bar/src/child.c".as_ptr(),
                    960 as ::core::ffi::c_int,
                    c"kill_children_at_exit".as_ptr(),
                );
            }
        } else if ws_child.pid == status_child.pid {
            if config.verbose {
                printf(
                c"[%s:%d] %s: status_command: pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n".as_ptr(),
                c"../i3bar/src/child.c".as_ptr(),
                960 as ::core::ffi::c_int,
                c"kill_children_at_exit".as_ptr(),
                ws_child.pid as ::core::ffi::c_long,
                ws_child.stopped as ::core::ffi::c_int,
                ws_child.stop_signal,
                ws_child.cont_signal,
                ws_child.click_events as ::core::ffi::c_int,
                ws_child.click_events_init as ::core::ffi::c_int,
            );
            }
        } else if ws_child.pid == ws_child.pid {
            if config.verbose {
                printf(
                c"[%s:%d] %s: workspace_command: pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n".as_ptr(),
                c"../i3bar/src/child.c".as_ptr(),
                960 as ::core::ffi::c_int,
                c"kill_children_at_exit".as_ptr(),
                ws_child.pid as ::core::ffi::c_long,
                ws_child.stopped as ::core::ffi::c_int,
                ws_child.stop_signal,
                ws_child.cont_signal,
                ws_child.click_events as ::core::ffi::c_int,
                ws_child.click_events_init as ::core::ffi::c_int,
            );
            }
        } else {
            fprintf(
            stderr,
            c"[%s:%d] ERROR: %s: unknown child, this should never happen pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n".as_ptr(),
            c"../i3bar/src/child.c".as_ptr(),
            960 as ::core::ffi::c_int,
            c"kill_children_at_exit".as_ptr(),
            ws_child.pid as ::core::ffi::c_long,
            ws_child.stopped as ::core::ffi::c_int,
            ws_child.stop_signal,
            ws_child.cont_signal,
            ws_child.click_events as ::core::ffi::c_int,
            ws_child.click_events_init as ::core::ffi::c_int,
        );
        }
        cont_children();
        if is_alive(&raw mut status_child) {
            killpg(status_child.pid as libc::pid_t, libc::SIGTERM);
        }
        if is_alive(&raw mut ws_child) {
            killpg(ws_child.pid as libc::pid_t, libc::SIGTERM);
        }
    }
}
unsafe fn cont_child(c: *mut i3bar_child) {
    if is_alive(c) as ::core::ffi::c_int != 0
        && (*c).cont_signal > 0 as ::core::ffi::c_int
        && (*c).stopped as ::core::ffi::c_int != 0
    {
        (*c).stopped = false_0 != 0;
        killpg((*c).pid as libc::pid_t, (*c).cont_signal);
    }
}
unsafe fn kill_and_wait(c: *mut i3bar_child) {
    if (*c).pid == 0 as ::core::ffi::c_int {
        if config.verbose {
            printf(
                c"[%s:%d] %s: child pid = 0\n".as_ptr(),
                c"../i3bar/src/child.c".as_ptr(),
                979 as ::core::ffi::c_int,
                c"kill_and_wait".as_ptr(),
            );
        }
    } else if (*c).pid == status_child.pid {
        if config.verbose {
            printf(
                c"[%s:%d] %s: status_command: pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n".as_ptr(),
                c"../i3bar/src/child.c".as_ptr(),
                979 as ::core::ffi::c_int,
                c"kill_and_wait".as_ptr(),
                (*c).pid as ::core::ffi::c_long,
                (*c).stopped as ::core::ffi::c_int,
                (*c).stop_signal,
                (*c).cont_signal,
                (*c).click_events as ::core::ffi::c_int,
                (*c).click_events_init as ::core::ffi::c_int,
            );
        }
    } else if (*c).pid == ws_child.pid {
        if config.verbose {
            printf(
                c"[%s:%d] %s: workspace_command: pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n".as_ptr(),
                c"../i3bar/src/child.c".as_ptr(),
                979 as ::core::ffi::c_int,
                c"kill_and_wait".as_ptr(),
                (*c).pid as ::core::ffi::c_long,
                (*c).stopped as ::core::ffi::c_int,
                (*c).stop_signal,
                (*c).cont_signal,
                (*c).click_events as ::core::ffi::c_int,
                (*c).click_events_init as ::core::ffi::c_int,
            );
        }
    } else {
        fprintf(
            stderr,
            c"[%s:%d] ERROR: %s: unknown child, this should never happen pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n".as_ptr(),
            c"../i3bar/src/child.c".as_ptr(),
            979 as ::core::ffi::c_int,
            c"kill_and_wait".as_ptr(),
            (*c).pid as ::core::ffi::c_long,
            (*c).stopped as ::core::ffi::c_int,
            (*c).stop_signal,
            (*c).cont_signal,
            (*c).click_events as ::core::ffi::c_int,
            (*c).click_events_init as ::core::ffi::c_int,
        );
    }
    if !is_alive(c) {
        return;
    }
    cont_child(c);
    killpg((*c).pid as libc::pid_t, libc::SIGTERM);
    let mut status: ::core::ffi::c_int = 0;
    waitpid(
        (*c).pid as libc::pid_t,
        &raw mut status,
        0 as ::core::ffi::c_int,
    );
    cleanup(c);
}

pub unsafe fn kill_child() {
    kill_and_wait(&raw mut status_child);
}

pub unsafe fn kill_ws_child() {
    kill_and_wait(&raw mut ws_child);
}
unsafe fn stop_child(c: *mut i3bar_child) {
    if (*c).stop_signal > 0 as ::core::ffi::c_int && !(*c).stopped {
        (*c).stopped = true_0 != 0;
        killpg((*c).pid as libc::pid_t, (*c).stop_signal);
    }
}

pub unsafe fn stop_children() {
    if status_child.pid == 0 as ::core::ffi::c_int {
        if config.verbose {
            printf(
                c"[%s:%d] %s: child pid = 0\n".as_ptr(),
                c"../i3bar/src/child.c".as_ptr(),
                1022 as ::core::ffi::c_int,
                c"stop_children".as_ptr(),
            );
        }
    } else if status_child.pid == status_child.pid {
        if config.verbose {
            printf(
                c"[%s:%d] %s: status_command: pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n".as_ptr(),
                c"../i3bar/src/child.c".as_ptr(),
                1022 as ::core::ffi::c_int,
                c"stop_children".as_ptr(),
                status_child.pid as ::core::ffi::c_long,
                status_child.stopped as ::core::ffi::c_int,
                status_child.stop_signal,
                status_child.cont_signal,
                status_child.click_events as ::core::ffi::c_int,
                status_child.click_events_init as ::core::ffi::c_int,
            );
        }
    } else if status_child.pid == ws_child.pid {
        if config.verbose {
            printf(
                c"[%s:%d] %s: workspace_command: pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n".as_ptr(),
                c"../i3bar/src/child.c".as_ptr(),
                1022 as ::core::ffi::c_int,
                c"stop_children".as_ptr(),
                status_child.pid as ::core::ffi::c_long,
                status_child.stopped as ::core::ffi::c_int,
                status_child.stop_signal,
                status_child.cont_signal,
                status_child.click_events as ::core::ffi::c_int,
                status_child.click_events_init as ::core::ffi::c_int,
            );
        }
    } else {
        fprintf(
            stderr,
            c"[%s:%d] ERROR: %s: unknown child, this should never happen pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n".as_ptr(),
            c"../i3bar/src/child.c".as_ptr(),
            1022 as ::core::ffi::c_int,
            c"stop_children".as_ptr(),
            status_child.pid as ::core::ffi::c_long,
            status_child.stopped as ::core::ffi::c_int,
            status_child.stop_signal,
            status_child.cont_signal,
            status_child.click_events as ::core::ffi::c_int,
            status_child.click_events_init as ::core::ffi::c_int,
        );
    }
    if ws_child.pid == 0 as ::core::ffi::c_int {
        if config.verbose {
            printf(
                c"[%s:%d] %s: child pid = 0\n".as_ptr(),
                c"../i3bar/src/child.c".as_ptr(),
                1022 as ::core::ffi::c_int,
                c"stop_children".as_ptr(),
            );
        }
    } else if ws_child.pid == status_child.pid {
        if config.verbose {
            printf(
                c"[%s:%d] %s: status_command: pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n".as_ptr(),
                c"../i3bar/src/child.c".as_ptr(),
                1022 as ::core::ffi::c_int,
                c"stop_children".as_ptr(),
                ws_child.pid as ::core::ffi::c_long,
                ws_child.stopped as ::core::ffi::c_int,
                ws_child.stop_signal,
                ws_child.cont_signal,
                ws_child.click_events as ::core::ffi::c_int,
                ws_child.click_events_init as ::core::ffi::c_int,
            );
        }
    } else if ws_child.pid == ws_child.pid {
        if config.verbose {
            printf(
                c"[%s:%d] %s: workspace_command: pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n".as_ptr(),
                c"../i3bar/src/child.c".as_ptr(),
                1022 as ::core::ffi::c_int,
                c"stop_children".as_ptr(),
                ws_child.pid as ::core::ffi::c_long,
                ws_child.stopped as ::core::ffi::c_int,
                ws_child.stop_signal,
                ws_child.cont_signal,
                ws_child.click_events as ::core::ffi::c_int,
                ws_child.click_events_init as ::core::ffi::c_int,
            );
        }
    } else {
        fprintf(
            stderr,
            c"[%s:%d] ERROR: %s: unknown child, this should never happen pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n".as_ptr(),
            c"../i3bar/src/child.c".as_ptr(),
            1022 as ::core::ffi::c_int,
            c"stop_children".as_ptr(),
            ws_child.pid as ::core::ffi::c_long,
            ws_child.stopped as ::core::ffi::c_int,
            ws_child.stop_signal,
            ws_child.cont_signal,
            ws_child.click_events as ::core::ffi::c_int,
            ws_child.click_events_init as ::core::ffi::c_int,
        );
    }
    stop_child(&raw mut status_child);
    stop_child(&raw mut ws_child);
}

pub unsafe fn cont_children() {
    if status_child.pid == 0 as ::core::ffi::c_int {
        if config.verbose {
            printf(
                c"[%s:%d] %s: child pid = 0\n".as_ptr(),
                c"../i3bar/src/child.c".as_ptr(),
                1032 as ::core::ffi::c_int,
                c"cont_children".as_ptr(),
            );
        }
    } else if status_child.pid == status_child.pid {
        if config.verbose {
            printf(
                c"[%s:%d] %s: status_command: pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n".as_ptr(),
                c"../i3bar/src/child.c".as_ptr(),
                1032 as ::core::ffi::c_int,
                c"cont_children".as_ptr(),
                status_child.pid as ::core::ffi::c_long,
                status_child.stopped as ::core::ffi::c_int,
                status_child.stop_signal,
                status_child.cont_signal,
                status_child.click_events as ::core::ffi::c_int,
                status_child.click_events_init as ::core::ffi::c_int,
            );
        }
    } else if status_child.pid == ws_child.pid {
        if config.verbose {
            printf(
                c"[%s:%d] %s: workspace_command: pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n".as_ptr(),
                c"../i3bar/src/child.c".as_ptr(),
                1032 as ::core::ffi::c_int,
                c"cont_children".as_ptr(),
                status_child.pid as ::core::ffi::c_long,
                status_child.stopped as ::core::ffi::c_int,
                status_child.stop_signal,
                status_child.cont_signal,
                status_child.click_events as ::core::ffi::c_int,
                status_child.click_events_init as ::core::ffi::c_int,
            );
        }
    } else {
        fprintf(
            stderr,
            c"[%s:%d] ERROR: %s: unknown child, this should never happen pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n".as_ptr(),
            c"../i3bar/src/child.c".as_ptr(),
            1032 as ::core::ffi::c_int,
            c"cont_children".as_ptr(),
            status_child.pid as ::core::ffi::c_long,
            status_child.stopped as ::core::ffi::c_int,
            status_child.stop_signal,
            status_child.cont_signal,
            status_child.click_events as ::core::ffi::c_int,
            status_child.click_events_init as ::core::ffi::c_int,
        );
    }
    if ws_child.pid == 0 as ::core::ffi::c_int {
        if config.verbose {
            printf(
                c"[%s:%d] %s: child pid = 0\n".as_ptr(),
                c"../i3bar/src/child.c".as_ptr(),
                1032 as ::core::ffi::c_int,
                c"cont_children".as_ptr(),
            );
        }
    } else if ws_child.pid == status_child.pid {
        if config.verbose {
            printf(
                c"[%s:%d] %s: status_command: pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n".as_ptr(),
                c"../i3bar/src/child.c".as_ptr(),
                1032 as ::core::ffi::c_int,
                c"cont_children".as_ptr(),
                ws_child.pid as ::core::ffi::c_long,
                ws_child.stopped as ::core::ffi::c_int,
                ws_child.stop_signal,
                ws_child.cont_signal,
                ws_child.click_events as ::core::ffi::c_int,
                ws_child.click_events_init as ::core::ffi::c_int,
            );
        }
    } else if ws_child.pid == ws_child.pid {
        if config.verbose {
            printf(
                c"[%s:%d] %s: workspace_command: pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n".as_ptr(),
                c"../i3bar/src/child.c".as_ptr(),
                1032 as ::core::ffi::c_int,
                c"cont_children".as_ptr(),
                ws_child.pid as ::core::ffi::c_long,
                ws_child.stopped as ::core::ffi::c_int,
                ws_child.stop_signal,
                ws_child.cont_signal,
                ws_child.click_events as ::core::ffi::c_int,
                ws_child.click_events_init as ::core::ffi::c_int,
            );
        }
    } else {
        fprintf(
            stderr,
            c"[%s:%d] ERROR: %s: unknown child, this should never happen pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n".as_ptr(),
            c"../i3bar/src/child.c".as_ptr(),
            1032 as ::core::ffi::c_int,
            c"cont_children".as_ptr(),
            ws_child.pid as ::core::ffi::c_long,
            ws_child.stopped as ::core::ffi::c_int,
            ws_child.stop_signal,
            ws_child.cont_signal,
            ws_child.click_events as ::core::ffi::c_int,
            ws_child.click_events_init as ::core::ffi::c_int,
        );
    }
    cont_child(&raw mut status_child);
    cont_child(&raw mut ws_child);
}

pub unsafe fn child_want_click_events() -> bool {
    status_child.click_events
}

unsafe extern "C" fn run_static_initializers() {
    statusline_head = statusline_head {
        tqh_first: ::core::ptr::null_mut::<status_block>(),
        tqh_last: &raw mut statusline_head.tqh_first,
    };
    statusline_buffer = statusline_head {
        tqh_first: ::core::ptr::null_mut::<status_block>(),
        tqh_last: &raw mut statusline_buffer.tqh_first,
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];

// Internal helpers use Rust linkage rather than foreign symbol lookup.
use crate::workspaces::free_workspaces;
use crate::workspaces::parse_workspaces_json;
use crate::xcb::draw_bars;
use i3_common::dpi::logical_px;
use i3_common::safewrappers::scalloc;
use i3_common::safewrappers::smalloc;
use i3_common::safewrappers::srealloc;
use i3_common::safewrappers::sstrdup;
use i3_common::safewrappers::writeall;

// Canonical Cairo backend ABI; renderer logic remains local.

use xcb_sys::{
    xcb_window_t, XCB_MOD_MASK_1, XCB_MOD_MASK_2, XCB_MOD_MASK_3, XCB_MOD_MASK_4, XCB_MOD_MASK_5,
    XCB_MOD_MASK_CONTROL, XCB_MOD_MASK_SHIFT,
};

pub use libc::{
    __errno_location, close, dup2, execl, exit, fcntl, fork, fprintf, free, killpg, memcpy,
    memmove, memset, pipe, printf, read, setpgid, strcasecmp, strerror, strlen, strncasecmp,
    strncmp, waitpid,
};

use i3_common::native_stdio::{__ctype_b_loc, err, stderr, vasprintf};

// Internal calls and globals use canonical Rust identities.
use crate::application::main_loop;
use crate::xcb::separator_symbol_width;

pub use i3_common::string::I3String as _i3String;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum BlockAlign {
    Left = 0,
    Center = 1,
    Right = 2,
}
