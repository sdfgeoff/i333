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
// Generated with C2Rust 0.22.1; C ABI and original ownership retained.
use ::c2rust_bitfields;
use i3_common::i333_va_list;

extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type _cairo;
    pub type _cairo_surface;
    pub type _i3String;
    pub type ev_loop;
    pub type yajl_handle_t;
    pub type yajl_gen_t;
    static mut stderr: *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn vasprintf(
        __ptr: *mut *mut ::core::ffi::c_char,
        __f: *const ::core::ffi::c_char,
        __arg: ::core::ffi::VaList,
    ) -> ::core::ffi::c_int;
    fn killpg(__pgrp: __pid_t, __sig: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn read(__fd: ::core::ffi::c_int, __buf: *mut ::core::ffi::c_void, __nbytes: size_t)
        -> ssize_t;
    fn pipe(__pipedes: *mut ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn dup2(__fd: ::core::ffi::c_int, __fd2: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn execl(
        __path: *const ::core::ffi::c_char,
        __arg: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn setpgid(__pid: __pid_t, __pgid: __pid_t) -> ::core::ffi::c_int;
    fn fork() -> __pid_t;
    fn g_strsplit(
        string: *const gchar,
        delimiter: *const gchar,
        max_tokens: gint,
    ) -> *mut *mut gchar;
    fn g_strfreev(str_array: *mut *mut gchar);
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn smalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn scalloc(num: size_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn srealloc(ptr: *mut ::core::ffi::c_void, size: size_t) -> *mut ::core::ffi::c_void;
    fn sstrdup(str: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn sasprintf(
        strp: *mut *mut ::core::ffi::c_char,
        fmt: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn writeall(fd: ::core::ffi::c_int, buf: *const ::core::ffi::c_void, count: size_t) -> ssize_t;
    fn i3string_from_utf8(from_utf8: *const ::core::ffi::c_char) -> *mut i3String;
    fn i3string_from_markup_with_length(
        from_markup: *const ::core::ffi::c_char,
        num_bytes: size_t,
    ) -> *mut i3String;
    fn i3string_free(str: *mut i3String);
    fn i3string_as_utf8(str: *mut i3String) -> *const ::core::ffi::c_char;
    fn i3string_set_markup(str: *mut i3String, pango_markup: bool);
    fn predict_text_width(text: *mut i3String) -> ::core::ffi::c_int;
    fn logical_px(logical: ::core::ffi::c_int) -> ::core::ffi::c_int;
    static mut main_loop: *mut ev_loop;
    fn ev_io_start(loop_0: *mut ev_loop, w: *mut ev_io);
    fn ev_io_stop(loop_0: *mut ev_loop, w: *mut ev_io);
    fn ev_child_start(loop_0: *mut ev_loop, w: *mut ev_child);
    fn ev_child_stop(loop_0: *mut ev_loop, w: *mut ev_child);
    static mut outputs: *mut outputs_head;
    fn parse_workspaces_json(json: *const ::core::ffi::c_uchar, size: size_t);
    fn free_workspaces();
    static mut separator_symbol_width: ::core::ffi::c_int;
    fn draw_bars(force_unhide: bool);
    static mut config: config_t;
    fn parse_json_header(
        child: *mut i3bar_child,
        buffer: *const ::core::ffi::c_uchar,
        length: ::core::ffi::c_int,
        consumed: *mut ::core::ffi::c_uint,
    );
    fn yajl_gen_alloc(allocFuncs: *const yajl_alloc_funcs) -> yajl_gen;
    fn yajl_gen_integer(hand: yajl_gen, number: ::core::ffi::c_longlong) -> yajl_gen_status;
    fn yajl_gen_string(
        hand: yajl_gen,
        str: *const ::core::ffi::c_uchar,
        len: size_t,
    ) -> yajl_gen_status;
    fn yajl_gen_map_open(hand: yajl_gen) -> yajl_gen_status;
    fn yajl_gen_map_close(hand: yajl_gen) -> yajl_gen_status;
    fn yajl_gen_array_open(hand: yajl_gen) -> yajl_gen_status;
    fn yajl_gen_array_close(hand: yajl_gen) -> yajl_gen_status;
    fn yajl_gen_get_buf(
        hand: yajl_gen,
        buf: *mut *const ::core::ffi::c_uchar,
        len: *mut size_t,
    ) -> yajl_gen_status;
    fn yajl_gen_clear(hand: yajl_gen);
    fn yajl_alloc(
        callbacks: *const yajl_callbacks,
        afs: *mut yajl_alloc_funcs,
        ctx: *mut ::core::ffi::c_void,
    ) -> yajl_handle;
    fn yajl_parse(
        hand: yajl_handle,
        jsonText: *const ::core::ffi::c_uchar,
        jsonTextLength: size_t,
    ) -> yajl_status;
    fn yajl_get_error(
        hand: yajl_handle,
        verbose: ::core::ffi::c_int,
        jsonText: *const ::core::ffi::c_uchar,
        jsonTextLength: size_t,
    ) -> *mut ::core::ffi::c_uchar;
    fn yajl_free_error(hand: yajl_handle, str: *mut ::core::ffi::c_uchar);
    fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    fn err(__status: ::core::ffi::c_int, __format: *const ::core::ffi::c_char, ...) -> !;
    fn fcntl(__fd: ::core::ffi::c_int, __cmd: ::core::ffi::c_int, ...) -> ::core::ffi::c_int;
    fn waitpid(
        __pid: __pid_t,
        __stat_loc: *mut ::core::ffi::c_int,
        __options: ::core::ffi::c_int,
    ) -> __pid_t;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memmove(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    fn strcasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strncasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn exit(__status: ::core::ffi::c_int) -> !;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: ::core::ffi::c_uint,
    pub fp_offset: ::core::ffi::c_uint,
    pub overflow_arg_area: *mut ::core::ffi::c_void,
    pub reg_save_area: *mut ::core::ffi::c_void,
}
pub type size_t = usize;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __pid_t = ::core::ffi::c_int;
pub type pid_t = __pid_t;
pub type ssize_t = isize;
pub type __gnuc_va_list = __builtin_va_list;
pub type va_list = __gnuc_va_list;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: ::core::ffi::c_int,
    pub _IO_read_ptr: *mut ::core::ffi::c_char,
    pub _IO_read_end: *mut ::core::ffi::c_char,
    pub _IO_read_base: *mut ::core::ffi::c_char,
    pub _IO_write_base: *mut ::core::ffi::c_char,
    pub _IO_write_ptr: *mut ::core::ffi::c_char,
    pub _IO_write_end: *mut ::core::ffi::c_char,
    pub _IO_buf_base: *mut ::core::ffi::c_char,
    pub _IO_buf_end: *mut ::core::ffi::c_char,
    pub _IO_save_base: *mut ::core::ffi::c_char,
    pub _IO_backup_base: *mut ::core::ffi::c_char,
    pub _IO_save_end: *mut ::core::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::core::ffi::c_int,
    #[bitfield(name = "_flags2", ty = "::core::ffi::c_int", bits = "0..=23")]
    pub _flags2: [u8; 3],
    pub _short_backupbuf: [::core::ffi::c_char; 1],
    pub _old_offset: __off_t,
    pub _cur_column: ::core::ffi::c_ushort,
    pub _vtable_offset: ::core::ffi::c_schar,
    pub _shortbuf: [::core::ffi::c_char; 1],
    pub _lock: *mut ::core::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::core::ffi::c_void,
    pub _prevchain: *mut *mut _IO_FILE,
    pub _mode: ::core::ffi::c_int,
    pub _unused3: ::core::ffi::c_int,
    pub _total_written: __uint64_t,
    pub _unused2: [::core::ffi::c_char; 8],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type uint32_t = __uint32_t;
pub type uintptr_t = usize;
pub type xcb_window_t = uint32_t;
pub type xcb_gcontext_t = uint32_t;
pub type xcb_drawable_t = uint32_t;
pub type xcb_mod_mask_t = ::core::ffi::c_uint;
pub const XCB_MOD_MASK_ANY: xcb_mod_mask_t = 32768;
pub const XCB_MOD_MASK_5: xcb_mod_mask_t = 128;
pub const XCB_MOD_MASK_4: xcb_mod_mask_t = 64;
pub const XCB_MOD_MASK_3: xcb_mod_mask_t = 32;
pub const XCB_MOD_MASK_2: xcb_mod_mask_t = 16;
pub const XCB_MOD_MASK_1: xcb_mod_mask_t = 8;
pub const XCB_MOD_MASK_CONTROL: xcb_mod_mask_t = 4;
pub const XCB_MOD_MASK_LOCK: xcb_mod_mask_t = 2;
pub const XCB_MOD_MASK_SHIFT: xcb_mod_mask_t = 1;
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type cairo_t = _cairo;
pub type cairo_surface_t = _cairo_surface;
pub type i3String = _i3String;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct surface_t {
    pub id: xcb_drawable_t,
    pub gc: xcb_gcontext_t,
    pub owns_gc: bool,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub surface: *mut cairo_surface_t,
    pub cr: *mut cairo_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rect_t {
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub w: ::core::ffi::c_int,
    pub h: ::core::ffi::c_int,
}
pub type rect = rect_t;
pub type blockalign_t = ::core::ffi::c_uint;
pub const ALIGN_RIGHT: blockalign_t = 2;
pub const ALIGN_CENTER: blockalign_t = 1;
pub const ALIGN_LEFT: blockalign_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct status_block_render_desc {
    pub width: uint32_t,
    pub x_offset: uint32_t,
    pub x_append: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct status_block {
    pub full_text: *mut i3String,
    pub short_text: *mut i3String,
    pub use_short: bool,
    pub render_length: uint32_t,
    pub color: *mut ::core::ffi::c_char,
    pub background: *mut ::core::ffi::c_char,
    pub border: *mut ::core::ffi::c_char,
    pub min_width: uint32_t,
    pub min_width_str: *mut ::core::ffi::c_char,
    pub align: blockalign_t,
    pub urgent: bool,
    pub no_separator: bool,
    pub border_top: uint32_t,
    pub border_right: uint32_t,
    pub border_bottom: uint32_t,
    pub border_left: uint32_t,
    pub pango_markup: bool,
    pub sep_block_width: uint32_t,
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
pub type C2RustUnnamed_0 = ::core::ffi::c_int;
pub const EV_ERROR: C2RustUnnamed_0 = -2147483648;
pub const EV_CUSTOM: C2RustUnnamed_0 = 16777216;
pub const EV_ASYNC: C2RustUnnamed_0 = 524288;
pub const EV_CLEANUP: C2RustUnnamed_0 = 262144;
pub const EV_FORK: C2RustUnnamed_0 = 131072;
pub const EV_EMBED: C2RustUnnamed_0 = 65536;
pub const EV_CHECK: C2RustUnnamed_0 = 32768;
pub const EV_PREPARE: C2RustUnnamed_0 = 16384;
pub const EV_IDLE: C2RustUnnamed_0 = 8192;
pub const EV_STAT: C2RustUnnamed_0 = 4096;
pub const EV_CHILD: C2RustUnnamed_0 = 2048;
pub const EV_SIGNAL: C2RustUnnamed_0 = 1024;
pub const EV_PERIODIC: C2RustUnnamed_0 = 512;
pub const EV_TIMEOUT: C2RustUnnamed_0 = 256;
pub const EV_TIMER: C2RustUnnamed_0 = 256;
pub const EV_IO: C2RustUnnamed_0 = 1;
pub const EV__IOFDSET: C2RustUnnamed_0 = 128;
pub const EV_WRITE: C2RustUnnamed_0 = 2;
pub const EV_READ: C2RustUnnamed_0 = 1;
pub const EV_NONE: C2RustUnnamed_0 = 0;
pub const EV_UNDEF: C2RustUnnamed_0 = -1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ev_watcher {
    pub active: ::core::ffi::c_int,
    pub pending: ::core::ffi::c_int,
    pub priority: ::core::ffi::c_int,
    pub data: *mut ::core::ffi::c_void,
    pub cb: Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_watcher, ::core::ffi::c_int) -> ()>,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct config_t {
    pub modifier: uint32_t,
    pub bindings: bindings_head,
    pub position: position_t,
    pub verbose: bool,
    pub bar_height: uint32_t,
    pub padding: Rect,
    pub transparency: bool,
    pub colors: xcb_color_strings_t,
    pub disable_binding_mode_indicator: bool,
    pub disable_ws: bool,
    pub ws_min_width: ::core::ffi::c_int,
    pub strip_ws_numbers: bool,
    pub strip_ws_name: bool,
    pub bar_id: *mut ::core::ffi::c_char,
    pub command: *mut ::core::ffi::c_char,
    pub workspace_command: *mut ::core::ffi::c_char,
    pub fontname: *mut ::core::ffi::c_char,
    pub separator_symbol: *mut i3String,
    pub tray_outputs: tray_outputs_head,
    pub tray_padding: ::core::ffi::c_int,
    pub num_outputs: ::core::ffi::c_int,
    pub outputs: *mut *mut ::core::ffi::c_char,
    pub hide_on_modifier: bar_display_mode_t,
    pub hidden_state: C2RustUnnamed_1,
}
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const S_SHOW: C2RustUnnamed_1 = 1;
pub const S_HIDE: C2RustUnnamed_1 = 0;
pub type bar_display_mode_t = ::core::ffi::c_uint;
pub const M_INVISIBLE: bar_display_mode_t = 2;
pub const M_HIDE: bar_display_mode_t = 1;
pub const M_DOCK: bar_display_mode_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tray_outputs_head {
    pub tqh_first: *mut tray_output_t,
    pub tqh_last: *mut *mut tray_output_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tray_output_t {
    pub output: *mut ::core::ffi::c_char,
    pub tray_outputs: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub tqe_next: *mut tray_output_t,
    pub tqe_prev: *mut *mut tray_output_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_color_strings_t {
    pub bar_fg: *mut ::core::ffi::c_char,
    pub bar_bg: *mut ::core::ffi::c_char,
    pub sep_fg: *mut ::core::ffi::c_char,
    pub focus_bar_fg: *mut ::core::ffi::c_char,
    pub focus_bar_bg: *mut ::core::ffi::c_char,
    pub focus_sep_fg: *mut ::core::ffi::c_char,
    pub active_ws_fg: *mut ::core::ffi::c_char,
    pub active_ws_bg: *mut ::core::ffi::c_char,
    pub active_ws_border: *mut ::core::ffi::c_char,
    pub inactive_ws_fg: *mut ::core::ffi::c_char,
    pub inactive_ws_bg: *mut ::core::ffi::c_char,
    pub inactive_ws_border: *mut ::core::ffi::c_char,
    pub focus_ws_bg: *mut ::core::ffi::c_char,
    pub focus_ws_fg: *mut ::core::ffi::c_char,
    pub focus_ws_border: *mut ::core::ffi::c_char,
    pub urgent_ws_bg: *mut ::core::ffi::c_char,
    pub urgent_ws_fg: *mut ::core::ffi::c_char,
    pub urgent_ws_border: *mut ::core::ffi::c_char,
    pub binding_mode_bg: *mut ::core::ffi::c_char,
    pub binding_mode_fg: *mut ::core::ffi::c_char,
    pub binding_mode_border: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Rect {
    pub x: uint32_t,
    pub y: uint32_t,
    pub width: uint32_t,
    pub height: uint32_t,
}
pub type position_t = ::core::ffi::c_uint;
pub const POS_BOT: position_t = 2;
pub const POS_TOP: position_t = 1;
pub const POS_NONE: position_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bindings_head {
    pub tqh_first: *mut binding_t,
    pub tqh_last: *mut *mut binding_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct binding_t {
    pub input_code: ::core::ffi::c_int,
    pub command: *mut ::core::ffi::c_char,
    pub release: bool,
    pub bindings: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub tqe_next: *mut binding_t,
    pub tqe_prev: *mut *mut binding_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct i3_ws {
    pub id: uintptr_t,
    pub num: ::core::ffi::c_int,
    pub canonical_name: *mut ::core::ffi::c_char,
    pub name: *mut i3String,
    pub name_width: ::core::ffi::c_int,
    pub visible: bool,
    pub focused: bool,
    pub urgent: bool,
    pub output: *mut i3_output,
    pub tailq: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub tqe_next: *mut i3_ws,
    pub tqe_prev: *mut *mut i3_ws,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct i3_output {
    pub name: *mut ::core::ffi::c_char,
    pub active: bool,
    pub primary: bool,
    pub visible: bool,
    pub ws: ::core::ffi::c_int,
    pub rect: rect,
    pub buffer: surface_t,
    pub statusline_buffer: surface_t,
    pub statusline_width: ::core::ffi::c_int,
    pub bar: surface_t,
    pub workspaces: *mut ws_head,
    pub trayclients: *mut tc_head,
    pub slist: C2RustUnnamed_5,
}
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
pub const _ISspace: C2RustUnnamed_7 = 8192;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct parser_ctx {
    pub has_urgent: bool,
    pub last_map_key: *mut ::core::ffi::c_char,
    pub block: status_block,
}
pub type yajl_handle = *mut yajl_handle_t;
pub type yajl_status = ::core::ffi::c_uint;
pub const yajl_status_error: yajl_status = 2;
pub const yajl_status_client_canceled: yajl_status = 1;
pub const yajl_status_ok: yajl_status = 0;
pub type yajl_gen = *mut yajl_gen_t;
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
pub type yajl_gen_status = ::core::ffi::c_uint;
pub const yajl_gen_invalid_string: yajl_gen_status = 7;
pub const yajl_gen_no_buf: yajl_gen_status = 6;
pub const yajl_gen_invalid_number: yajl_gen_status = 5;
pub const yajl_gen_generation_complete: yajl_gen_status = 4;
pub const yajl_gen_in_error_state: yajl_gen_status = 3;
pub const yajl_max_depth_exceeded: yajl_gen_status = 2;
pub const yajl_gen_keys_must_be_strings: yajl_gen_status = 1;
pub const yajl_gen_status_ok: yajl_gen_status = 0;
pub type C2RustUnnamed_7 = ::core::ffi::c_uint;
pub const _ISalnum: C2RustUnnamed_7 = 8;
pub const _ISpunct: C2RustUnnamed_7 = 4;
pub const _IScntrl: C2RustUnnamed_7 = 2;
pub const _ISblank: C2RustUnnamed_7 = 1;
pub const _ISgraph: C2RustUnnamed_7 = 32768;
pub const _ISprint: C2RustUnnamed_7 = 16384;
pub const _ISxdigit: C2RustUnnamed_7 = 4096;
pub const _ISdigit: C2RustUnnamed_7 = 2048;
pub const _ISalpha: C2RustUnnamed_7 = 1024;
pub const _ISlower: C2RustUnnamed_7 = 512;
pub const _ISupper: C2RustUnnamed_7 = 256;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SIGTERM: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const SIGSTOP: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const SIGCONT: ::core::ffi::c_int = 18 as ::core::ffi::c_int;
pub const STDIN_FILENO: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const STDOUT_FILENO: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EAGAIN: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const STDIN_CHUNK_SIZE: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const O_NONBLOCK: ::core::ffi::c_int = 0o4000 as ::core::ffi::c_int;
pub const F_SETFL: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const _PATH_BSHELL: [::core::ffi::c_char; 8] =
    unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"/bin/sh\0") };
#[no_mangle]
pub static mut status_child: i3bar_child = i3bar_child {
    pid: 0 as pid_t,
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
#[no_mangle]
pub static mut ws_child: i3bar_child = i3bar_child {
    pid: 0 as pid_t,
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
#[no_mangle]
pub static mut parser: yajl_handle = ::core::ptr::null::<yajl_handle_t>() as *mut yajl_handle_t;
#[no_mangle]
pub static mut gen: yajl_gen = ::core::ptr::null::<yajl_gen_t>() as *mut yajl_gen_t;
#[no_mangle]
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
        align: ALIGN_LEFT,
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
#[no_mangle]
pub static mut statusline_head: statusline_head = statusline_head {
    tqh_first: ::core::ptr::null::<status_block>() as *mut status_block,
    tqh_last: ::core::ptr::null::<*mut status_block>() as *mut *mut status_block,
};
#[no_mangle]
pub static mut statusline_buffer: statusline_head = statusline_head {
    tqh_first: ::core::ptr::null::<status_block>() as *mut status_block,
    tqh_last: ::core::ptr::null::<*mut status_block>() as *mut *mut status_block,
};
#[no_mangle]
pub static mut child_stdin: ::core::ffi::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn clear_statusline(
    mut head: *mut statusline_head,
    mut free_resources: bool,
) {
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
unsafe extern "C" fn copy_statusline(mut from: *mut statusline_head, mut to: *mut statusline_head) {
    let mut current: *mut status_block = ::core::ptr::null_mut::<status_block>();
    current = (*from).tqh_first;
    while !current.is_null() {
        let mut new_block: *mut status_block =
            smalloc(::core::mem::size_of::<status_block>() as size_t) as *mut status_block;
        memcpy(
            new_block as *mut ::core::ffi::c_void,
            current as *const ::core::ffi::c_void,
            ::core::mem::size_of::<status_block>() as size_t,
        );
        (*new_block).blocks.tqe_next = ::core::ptr::null_mut::<status_block>();
        (*new_block).blocks.tqe_prev = (*to).tqh_last;
        *(*to).tqh_last = new_block;
        (*to).tqh_last = &raw mut (*new_block).blocks.tqe_next;
        current = (*current).blocks.tqe_next;
    }
}
unsafe extern "C" fn set_statusline_error(mut format: *const ::core::ffi::c_char, mut args: ...) {
    let mut err_block: *mut status_block = ::core::ptr::null_mut::<status_block>();
    let mut message_block: *mut status_block = ::core::ptr::null_mut::<status_block>();
    clear_statusline(&raw mut statusline_head, true_0 != 0);
    let mut message: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut args_0;
    args_0 = args.clone();
    if !(vasprintf(&raw mut message, format, i333_va_list!(args_0)) == -(1 as ::core::ffi::c_int)) {
        err_block = scalloc(
            1 as size_t,
            ::core::mem::size_of::<status_block>() as size_t,
        ) as *mut status_block;
        (*err_block).full_text =
            i3string_from_utf8(b"Error: \0" as *const u8 as *const ::core::ffi::c_char);
        (*err_block).name = sstrdup(b"error\0" as *const u8 as *const ::core::ffi::c_char);
        (*err_block).color = sstrdup(b"#ff0000\0" as *const u8 as *const ::core::ffi::c_char);
        (*err_block).no_separator = true_0 != 0;
        message_block = scalloc(
            1 as size_t,
            ::core::mem::size_of::<status_block>() as size_t,
        ) as *mut status_block;
        (*message_block).full_text = i3string_from_utf8(message);
        (*message_block).name =
            sstrdup(b"error_message\0" as *const u8 as *const ::core::ffi::c_char);
        (*message_block).color = sstrdup(b"#ff0000\0" as *const u8 as *const ::core::ffi::c_char);
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
unsafe extern "C" fn cleanup(mut c: *mut i3bar_child) {
    if (*c).pid == 0 as ::core::ffi::c_int {
        if config.verbose {
            printf(
                b"[%s:%d] %s: child pid = 0\n\0" as *const u8 as *const ::core::ffi::c_char,
                b"../i3bar/src/child.c\0" as *const u8 as *const ::core::ffi::c_char,
                158 as ::core::ffi::c_int,
                b"cleanup\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    } else if (*c).pid == status_child.pid {
        if config.verbose {
            printf(
                b"[%s:%d] %s: status_command: pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
                b"../i3bar/src/child.c\0" as *const u8 as *const ::core::ffi::c_char,
                158 as ::core::ffi::c_int,
                b"cleanup\0" as *const u8 as *const ::core::ffi::c_char,
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
                b"[%s:%d] %s: workspace_command: pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
                b"../i3bar/src/child.c\0" as *const u8 as *const ::core::ffi::c_char,
                158 as ::core::ffi::c_int,
                b"cleanup\0" as *const u8 as *const ::core::ffi::c_char,
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
            b"[%s:%d] ERROR: %s: unknown child, this should never happen pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"../i3bar/src/child.c\0" as *const u8 as *const ::core::ffi::c_char,
            158 as ::core::ffi::c_int,
            b"cleanup\0" as *const u8 as *const ::core::ffi::c_char,
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
        ::core::mem::size_of::<i3bar_child>() as size_t,
    );
}
unsafe extern "C" fn stdin_start_array(
    mut context: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    clear_statusline(&raw mut statusline_buffer, false_0 != 0);
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn stdin_start_map(mut context: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    let mut ctx: *mut parser_ctx = context as *mut parser_ctx;
    memset(
        &raw mut (*ctx).block as *mut ::core::ffi::c_void,
        '\0' as i32,
        ::core::mem::size_of::<status_block>() as size_t,
    );
    if config.separator_symbol.is_null() {
        (*ctx).block.sep_block_width = logical_px(9 as ::core::ffi::c_int) as uint32_t;
    } else {
        (*ctx).block.sep_block_width =
            (logical_px(8 as ::core::ffi::c_int) + separator_symbol_width) as uint32_t;
    }
    (*ctx).block.border_top = 1 as uint32_t;
    (*ctx).block.border_right = 1 as uint32_t;
    (*ctx).block.border_bottom = 1 as uint32_t;
    (*ctx).block.border_left = 1 as uint32_t;
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn stdin_map_key(
    mut context: *mut ::core::ffi::c_void,
    mut key: *const ::core::ffi::c_uchar,
    mut len: size_t,
) -> ::core::ffi::c_int {
    let mut ctx: *mut parser_ctx = context as *mut parser_ctx;
    free((*ctx).last_map_key as *mut ::core::ffi::c_void);
    (*ctx).last_map_key = ::core::ptr::null_mut::<::core::ffi::c_char>();
    sasprintf(
        &raw mut (*ctx).last_map_key,
        b"%.*s\0" as *const u8 as *const ::core::ffi::c_char,
        len as ::core::ffi::c_int,
        key,
    );
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn stdin_boolean(
    mut context: *mut ::core::ffi::c_void,
    mut val: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ctx: *mut parser_ctx = context as *mut parser_ctx;
    if (*ctx).last_map_key.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if strcasecmp(
        (*ctx).last_map_key,
        b"urgent\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        (*ctx).block.urgent = val != 0;
        return 1 as ::core::ffi::c_int;
    }
    if strcasecmp(
        (*ctx).last_map_key,
        b"separator\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        (*ctx).block.no_separator = val == 0;
        return 1 as ::core::ffi::c_int;
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn stdin_string(
    mut context: *mut ::core::ffi::c_void,
    mut val: *const ::core::ffi::c_uchar,
    mut len: size_t,
) -> ::core::ffi::c_int {
    let mut ctx: *mut parser_ctx = context as *mut parser_ctx;
    if (*ctx).last_map_key.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if strcasecmp(
        (*ctx).last_map_key,
        b"full_text\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        (*ctx).block.full_text =
            i3string_from_markup_with_length(val as *const ::core::ffi::c_char, len);
        return 1 as ::core::ffi::c_int;
    }
    if strcasecmp(
        (*ctx).last_map_key,
        b"short_text\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        (*ctx).block.short_text =
            i3string_from_markup_with_length(val as *const ::core::ffi::c_char, len);
        return 1 as ::core::ffi::c_int;
    }
    if strcasecmp(
        (*ctx).last_map_key,
        b"color\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        sasprintf(
            &raw mut (*ctx).block.color,
            b"%.*s\0" as *const u8 as *const ::core::ffi::c_char,
            len as ::core::ffi::c_int,
            val,
        );
        return 1 as ::core::ffi::c_int;
    }
    if strcasecmp(
        (*ctx).last_map_key,
        b"background\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        sasprintf(
            &raw mut (*ctx).block.background,
            b"%.*s\0" as *const u8 as *const ::core::ffi::c_char,
            len as ::core::ffi::c_int,
            val,
        );
        return 1 as ::core::ffi::c_int;
    }
    if strcasecmp(
        (*ctx).last_map_key,
        b"border\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        sasprintf(
            &raw mut (*ctx).block.border,
            b"%.*s\0" as *const u8 as *const ::core::ffi::c_char,
            len as ::core::ffi::c_int,
            val,
        );
        return 1 as ::core::ffi::c_int;
    }
    if strcasecmp(
        (*ctx).last_map_key,
        b"markup\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        (*ctx).block.pango_markup = len
            == strlen(b"pango\0" as *const u8 as *const ::core::ffi::c_char)
            && strncasecmp(
                val as *const ::core::ffi::c_char,
                b"pango\0" as *const u8 as *const ::core::ffi::c_char,
                strlen(b"pango\0" as *const u8 as *const ::core::ffi::c_char),
            ) == 0;
        return 1 as ::core::ffi::c_int;
    }
    if strcasecmp(
        (*ctx).last_map_key,
        b"align\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        if len == strlen(b"center\0" as *const u8 as *const ::core::ffi::c_char)
            && strncmp(
                val as *const ::core::ffi::c_char,
                b"center\0" as *const u8 as *const ::core::ffi::c_char,
                strlen(b"center\0" as *const u8 as *const ::core::ffi::c_char),
            ) == 0
        {
            (*ctx).block.align = ALIGN_CENTER;
        } else if len == strlen(b"right\0" as *const u8 as *const ::core::ffi::c_char)
            && strncmp(
                val as *const ::core::ffi::c_char,
                b"right\0" as *const u8 as *const ::core::ffi::c_char,
                strlen(b"right\0" as *const u8 as *const ::core::ffi::c_char),
            ) == 0
        {
            (*ctx).block.align = ALIGN_RIGHT;
        } else {
            (*ctx).block.align = ALIGN_LEFT;
        }
        return 1 as ::core::ffi::c_int;
    }
    if strcasecmp(
        (*ctx).last_map_key,
        b"min_width\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        sasprintf(
            &raw mut (*ctx).block.min_width_str,
            b"%.*s\0" as *const u8 as *const ::core::ffi::c_char,
            len as ::core::ffi::c_int,
            val,
        );
        return 1 as ::core::ffi::c_int;
    }
    if strcasecmp(
        (*ctx).last_map_key,
        b"name\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        sasprintf(
            &raw mut (*ctx).block.name,
            b"%.*s\0" as *const u8 as *const ::core::ffi::c_char,
            len as ::core::ffi::c_int,
            val,
        );
        return 1 as ::core::ffi::c_int;
    }
    if strcasecmp(
        (*ctx).last_map_key,
        b"instance\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        sasprintf(
            &raw mut (*ctx).block.instance,
            b"%.*s\0" as *const u8 as *const ::core::ffi::c_char,
            len as ::core::ffi::c_int,
            val,
        );
        return 1 as ::core::ffi::c_int;
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn stdin_integer(
    mut context: *mut ::core::ffi::c_void,
    mut val: ::core::ffi::c_longlong,
) -> ::core::ffi::c_int {
    let mut ctx: *mut parser_ctx = context as *mut parser_ctx;
    if (*ctx).last_map_key.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if strcasecmp(
        (*ctx).last_map_key,
        b"min_width\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        (*ctx).block.min_width = val as uint32_t;
        return 1 as ::core::ffi::c_int;
    }
    if strcasecmp(
        (*ctx).last_map_key,
        b"separator_block_width\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        (*ctx).block.sep_block_width = val as uint32_t;
        return 1 as ::core::ffi::c_int;
    }
    if strcasecmp(
        (*ctx).last_map_key,
        b"border_top\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        (*ctx).block.border_top = val as uint32_t;
        return 1 as ::core::ffi::c_int;
    }
    if strcasecmp(
        (*ctx).last_map_key,
        b"border_right\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        (*ctx).block.border_right = val as uint32_t;
        return 1 as ::core::ffi::c_int;
    }
    if strcasecmp(
        (*ctx).last_map_key,
        b"border_bottom\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        (*ctx).block.border_bottom = val as uint32_t;
        return 1 as ::core::ffi::c_int;
    }
    if strcasecmp(
        (*ctx).last_map_key,
        b"border_left\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        (*ctx).block.border_left = val as uint32_t;
        return 1 as ::core::ffi::c_int;
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn stdin_end_map(mut context: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    let mut ctx: *mut parser_ctx = context as *mut parser_ctx;
    let mut new_block: *mut status_block =
        smalloc(::core::mem::size_of::<status_block>() as size_t) as *mut status_block;
    memcpy(
        new_block as *mut ::core::ffi::c_void,
        &raw mut (*ctx).block as *const ::core::ffi::c_void,
        ::core::mem::size_of::<status_block>() as size_t,
    );
    if (*new_block).full_text.is_null() {
        (*new_block).full_text = i3string_from_utf8(
            b"SPEC VIOLATION: full_text is NULL!\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if (*new_block).urgent {
        (*ctx).has_urgent = true_0 != 0;
    }
    if !(*new_block).min_width_str.is_null() {
        let mut text: *mut i3String = i3string_from_utf8((*new_block).min_width_str);
        i3string_set_markup(text, (*new_block).pango_markup);
        (*new_block).min_width = predict_text_width(text) as uint32_t;
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
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn stdin_end_array(mut context: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    if config.verbose {
        printf(
            b"[%s:%d] copying statusline_buffer to statusline_head\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"../i3bar/src/child.c\0" as *const u8 as *const ::core::ffi::c_char,
            374 as ::core::ffi::c_int,
        );
    }
    clear_statusline(&raw mut statusline_head, true_0 != 0);
    copy_statusline(&raw mut statusline_buffer, &raw mut statusline_head);
    if config.verbose {
        printf(
            b"[%s:%d] dumping statusline:\n\0" as *const u8 as *const ::core::ffi::c_char,
            b"../i3bar/src/child.c\0" as *const u8 as *const ::core::ffi::c_char,
            378 as ::core::ffi::c_int,
        );
    }
    let mut current: *mut status_block = ::core::ptr::null_mut::<status_block>();
    current = statusline_head.tqh_first;
    while !current.is_null() {
        if config.verbose {
            printf(
                b"[%s:%d] full_text = %s\n\0" as *const u8 as *const ::core::ffi::c_char,
                b"../i3bar/src/child.c\0" as *const u8 as *const ::core::ffi::c_char,
                381 as ::core::ffi::c_int,
                i3string_as_utf8((*current).full_text),
            );
        }
        if config.verbose {
            printf(
                b"[%s:%d] short_text = %s\n\0" as *const u8 as *const ::core::ffi::c_char,
                b"../i3bar/src/child.c\0" as *const u8 as *const ::core::ffi::c_char,
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
                b"[%s:%d] color = %s\n\0" as *const u8 as *const ::core::ffi::c_char,
                b"../i3bar/src/child.c\0" as *const u8 as *const ::core::ffi::c_char,
                383 as ::core::ffi::c_int,
                (*current).color,
            );
        }
        current = (*current).blocks.tqe_next;
    }
    if config.verbose {
        printf(
            b"[%s:%d] end of dump\n\0" as *const u8 as *const ::core::ffi::c_char,
            b"../i3bar/src/child.c\0" as *const u8 as *const ::core::ffi::c_char,
            385 as ::core::ffi::c_int,
        );
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn get_buffer(
    mut fd: ::core::ffi::c_int,
    mut ret_buffer_len: *mut ::core::ffi::c_int,
) -> *mut ::core::ffi::c_uchar {
    let mut rec: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut buffer_len: ::core::ffi::c_int = STDIN_CHUNK_SIZE;
    let mut buffer: *mut ::core::ffi::c_uchar =
        smalloc((buffer_len + 1 as ::core::ffi::c_int) as size_t) as *mut ::core::ffi::c_uchar;
    *buffer.offset(0 as ::core::ffi::c_int as isize) = '\0' as i32 as ::core::ffi::c_uchar;
    loop {
        let n: ssize_t = read(
            fd,
            buffer.offset(rec as isize) as *mut ::core::ffi::c_void,
            (buffer_len - rec) as size_t,
        ) as ssize_t;
        if n == -(1 as ::core::ffi::c_int) as ssize_t {
            if *__errno_location() == EAGAIN {
                break;
            }
            fprintf(
                stderr,
                b"[%s:%d] ERROR: read() failed!: %s\n\0" as *const u8 as *const ::core::ffi::c_char,
                b"../i3bar/src/child.c\0" as *const u8 as *const ::core::ffi::c_char,
                407 as ::core::ffi::c_int,
                strerror(*__errno_location()),
            );
            free(buffer as *mut ::core::ffi::c_void);
            buffer = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
            exit(EXIT_FAILURE);
        } else {
            if n == 0 as ssize_t {
                fprintf(
                    stderr,
                    b"[%s:%d] ERROR: stdin: received EOF\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    b"../i3bar/src/child.c\0" as *const u8 as *const ::core::ffi::c_char,
                    412 as ::core::ffi::c_int,
                );
                free(buffer as *mut ::core::ffi::c_void);
                buffer = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                *ret_buffer_len = -(1 as ::core::ffi::c_int);
                return ::core::ptr::null_mut::<::core::ffi::c_uchar>();
            }
            rec = (rec as ssize_t + n) as ::core::ffi::c_int;
            if rec == buffer_len {
                buffer_len += STDIN_CHUNK_SIZE;
                buffer = srealloc(
                    buffer as *mut ::core::ffi::c_void,
                    (buffer_len + 1 as ::core::ffi::c_int) as size_t,
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
    return buffer;
}
unsafe extern "C" fn read_flat_input(
    mut buffer: *mut ::core::ffi::c_char,
    mut length: ::core::ffi::c_int,
) {
    let mut first: *mut status_block = statusline_head.tqh_first;
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
unsafe extern "C" fn read_json_input(
    mut input: *mut ::core::ffi::c_uchar,
    mut length: ::core::ffi::c_int,
) -> bool {
    let mut status: yajl_status = yajl_parse(parser, input, length as size_t);
    let mut has_urgent: bool = false_0 != 0;
    if status as ::core::ffi::c_uint != yajl_status_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut message: *mut ::core::ffi::c_char =
            yajl_get_error(parser, 0 as ::core::ffi::c_int, input, length as size_t)
                as *mut ::core::ffi::c_char;
        if *message.offset(strlen(message).wrapping_sub(1 as size_t) as isize) as ::core::ffi::c_int
            == '\n' as i32
        {
            *message.offset(strlen(message).wrapping_sub(1 as size_t) as isize) =
                '\0' as i32 as ::core::ffi::c_char;
        }
        fprintf(
            stderr,
            b"[i3bar] Could not parse JSON input (code = %d, message = %s): %.*s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            status as ::core::ffi::c_uint,
            message,
            length,
            input,
        );
        set_statusline_error(
            b"Could not parse JSON (%s)\0" as *const u8 as *const ::core::ffi::c_char,
            message,
        );
        yajl_free_error(parser, message as *mut ::core::ffi::c_uchar);
        draw_bars(false_0 != 0);
    } else if parser_context.has_urgent {
        has_urgent = true_0 != 0;
    }
    return has_urgent;
}
unsafe extern "C" fn stdin_io_cb(mut fd: ::core::ffi::c_int) {
    let mut rec: ::core::ffi::c_int = 0;
    let mut buffer: *mut ::core::ffi::c_uchar = get_buffer(fd, &raw mut rec);
    if buffer.is_null() {
        return;
    }
    let mut has_urgent: bool = false_0 != 0;
    if status_child.version > 0 as uint32_t {
        has_urgent = read_json_input(buffer, rec);
    } else {
        read_flat_input(buffer as *mut ::core::ffi::c_char, rec);
    }
    free(buffer as *mut ::core::ffi::c_void);
    draw_bars(has_urgent);
}
unsafe extern "C" fn stdin_io_first_line_cb(mut fd: ::core::ffi::c_int) {
    let mut rec: ::core::ffi::c_int = 0;
    let mut buffer: *mut ::core::ffi::c_uchar = get_buffer(fd, &raw mut rec);
    if buffer.is_null() {
        return;
    }
    if config.verbose {
        printf(
            b"[%s:%d] Detecting input type based on buffer *%.*s*\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"../i3bar/src/child.c\0" as *const u8 as *const ::core::ffi::c_char,
            503 as ::core::ffi::c_int,
            rec,
            buffer,
        );
    }
    let mut consumed: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    parse_json_header(&raw mut status_child, buffer, rec, &raw mut consumed);
    if status_child.version > 0 as uint32_t {
        if config.hide_on_modifier as u64 != 0 {
            stop_children();
        }
        draw_bars(read_json_input(
            buffer.offset(consumed as isize),
            (rec as ::core::ffi::c_uint).wrapping_sub(consumed) as ::core::ffi::c_int,
        ));
    } else {
        let mut new_block: *mut status_block = scalloc(
            1 as size_t,
            ::core::mem::size_of::<status_block>() as size_t,
        ) as *mut status_block;
        (*new_block).blocks.tqe_next = ::core::ptr::null_mut::<status_block>();
        (*new_block).blocks.tqe_prev = statusline_head.tqh_last;
        *statusline_head.tqh_last = new_block;
        statusline_head.tqh_last = &raw mut (*new_block).blocks.tqe_next;
        read_flat_input(buffer as *mut ::core::ffi::c_char, rec);
    }
    free(buffer as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn isempty(mut s: *mut ::core::ffi::c_char) -> bool {
    while *s as ::core::ffi::c_int != '\0' as i32 {
        if *(*__ctype_b_loc()).offset(*s as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
            == 0
        {
            return false_0 != 0;
        }
        s = s.offset(1);
    }
    return true_0 != 0;
}
unsafe extern "C" fn append_string(
    mut previous: *const ::core::ffi::c_char,
    mut str: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    if !previous.is_null() {
        let mut result: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        sasprintf(
            &raw mut result,
            b"%s%s\0" as *const u8 as *const ::core::ffi::c_char,
            previous,
            str,
        );
        return result;
    }
    return sstrdup(str);
}
static mut ws_last_json: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
unsafe extern "C" fn ws_stdin_io_cb(mut fd: ::core::ffi::c_int) {
    let mut rec: ::core::ffi::c_int = 0;
    let mut buffer: *mut ::core::ffi::c_uchar = get_buffer(fd, &raw mut rec);
    if buffer.is_null() {
        return;
    }
    let mut strings: *mut *mut gchar = g_strsplit(
        buffer as *const gchar,
        b"\n\0" as *const u8 as *const gchar,
        0 as gint,
    );
    let mut idx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while !(*strings.offset(idx as isize)).is_null() {
        if !(ws_child.pending_line.is_null()
            && isempty(*strings.offset(idx as isize) as *mut ::core::ffi::c_char)
                as ::core::ffi::c_int
                != 0)
        {
            if (*strings.offset((idx + 1 as ::core::ffi::c_int) as isize)).is_null() {
                let mut new: *mut ::core::ffi::c_char =
                    append_string(ws_child.pending_line, *strings.offset(idx as isize));
                free(ws_child.pending_line as *mut ::core::ffi::c_void);
                ws_child.pending_line = new;
            } else {
                free(ws_last_json as *mut ::core::ffi::c_void);
                ws_last_json = append_string(ws_child.pending_line, *strings.offset(idx as isize));
                free(ws_child.pending_line as *mut ::core::ffi::c_void);
                ws_child.pending_line = ::core::ptr::null_mut::<::core::ffi::c_char>();
                parse_workspaces_json(
                    ws_last_json as *const ::core::ffi::c_uchar,
                    strlen(ws_last_json),
                );
            }
        }
        idx += 1;
    }
    g_strfreev(strings);
    free(buffer as *mut ::core::ffi::c_void);
    draw_bars(false_0 != 0);
}
unsafe extern "C" fn common_stdin_cb(
    mut loop_0: *mut ev_loop,
    mut watcher: *mut ev_io,
    mut revents: ::core::ffi::c_int,
) {
    if watcher == status_child.stdin_io {
        if status_child.version == -(1 as ::core::ffi::c_int) as uint32_t {
            stdin_io_first_line_cb((*watcher).fd);
        } else {
            stdin_io_cb((*watcher).fd);
        }
    } else if watcher == ws_child.stdin_io {
        ws_stdin_io_cb((*watcher).fd);
    } else {
        fprintf(
            stderr,
            b"[%s:%d] ERROR: Got callback for unknown watcher fd=%d\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"../i3bar/src/child.c\0" as *const u8 as *const ::core::ffi::c_char,
            594 as ::core::ffi::c_int,
            (*watcher).fd,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn repeat_last_ws_json() {
    if !ws_last_json.is_null() {
        if config.verbose {
            printf(
                b"[%s:%d] Repeating last workspace JSON\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"../i3bar/src/child.c\0" as *const u8 as *const ::core::ffi::c_char,
                604 as ::core::ffi::c_int,
            );
        }
        parse_workspaces_json(
            ws_last_json as *const ::core::ffi::c_uchar,
            strlen(ws_last_json),
        );
    }
}
unsafe extern "C" fn set_workspace_button_error_f(
    mut format: *const ::core::ffi::c_char,
    mut args: ...
) {
    let mut message: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut args_0;
    args_0 = args.clone();
    if !(vasprintf(&raw mut message, format, i333_va_list!(args_0)) == -(1 as ::core::ffi::c_int)) {
        set_workspace_button_error(message);
    }
    free(message as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn set_workspace_button_error(mut message: *const ::core::ffi::c_char) {
    free_workspaces();
    let mut name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    sasprintf(
        &raw mut name,
        b"Error: %s\0" as *const u8 as *const ::core::ffi::c_char,
        message,
    );
    let mut output: *mut i3_output = ::core::ptr::null_mut::<i3_output>();
    output = (*outputs).slh_first as *mut i3_output;
    while !output.is_null() {
        let mut fake_ws: *mut i3_ws =
            scalloc(1 as size_t, ::core::mem::size_of::<i3_ws>() as size_t) as *mut i3_ws;
        (*fake_ws).name = i3string_from_utf8(name);
        (*fake_ws).name_width = predict_text_width((*fake_ws).name);
        (*fake_ws).num = -(1 as ::core::ffi::c_int);
        (*fake_ws).visible = true_0 != 0;
        (*fake_ws).urgent = (*fake_ws).visible;
        (*fake_ws).output = output as *mut i3_output;
        (*fake_ws).tailq.tqe_next = ::core::ptr::null_mut::<i3_ws>();
        (*fake_ws).tailq.tqe_prev = (*(*output).workspaces).tqh_last;
        *(*(*output).workspaces).tqh_last = fake_ws as *mut i3_ws;
        (*(*output).workspaces).tqh_last = &raw mut (*fake_ws).tailq.tqe_next;
        output = (*output).slist.sle_next as *mut i3_output;
    }
    free(name as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn child_sig_cb(
    mut loop_0: *mut ev_loop,
    mut watcher: *mut ev_child,
    mut revents: ::core::ffi::c_int,
) {
    let exit_status: ::core::ffi::c_int =
        ((*watcher).rstatus & 0xff00 as ::core::ffi::c_int) >> 8 as ::core::ffi::c_int;
    fprintf(
        stderr,
        b"[%s:%d] ERROR: Child (pid: %d) unexpectedly exited with status %d\n\0" as *const u8
            as *const ::core::ffi::c_char,
        b"../i3bar/src/child.c\0" as *const u8 as *const ::core::ffi::c_char,
        664 as ::core::ffi::c_int,
        (*watcher).pid,
        exit_status,
    );
    let mut error_function_pointer: Option<
        unsafe extern "C" fn(*const ::core::ffi::c_char, ...) -> (),
    > = None;
    let mut command_type: *const ::core::ffi::c_char =
        b"\0" as *const u8 as *const ::core::ffi::c_char;
    let mut c: *mut i3bar_child = ::core::ptr::null_mut::<i3bar_child>();
    if (*watcher).pid == status_child.pid {
        command_type = b"status_command\0" as *const u8 as *const ::core::ffi::c_char;
        error_function_pointer = Some(
            set_statusline_error as unsafe extern "C" fn(*const ::core::ffi::c_char, ...) -> (),
        )
            as Option<unsafe extern "C" fn(*const ::core::ffi::c_char, ...) -> ()>;
        c = &raw mut status_child;
    } else if (*watcher).pid == ws_child.pid {
        command_type = b"workspace_command\0" as *const u8 as *const ::core::ffi::c_char;
        error_function_pointer = Some(
            set_workspace_button_error_f
                as unsafe extern "C" fn(*const ::core::ffi::c_char, ...) -> (),
        )
            as Option<unsafe extern "C" fn(*const ::core::ffi::c_char, ...) -> ()>;
        c = &raw mut ws_child;
    } else {
        fprintf(
            stderr,
            b"[%s:%d] ERROR: Unknown child pid, this should never happen\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"../i3bar/src/child.c\0" as *const u8 as *const ::core::ffi::c_char,
            678 as ::core::ffi::c_int,
        );
        return;
    }
    if (*c).pid == 0 as ::core::ffi::c_int {
        if config.verbose {
            printf(
                b"[%s:%d] %s: child pid = 0\n\0" as *const u8 as *const ::core::ffi::c_char,
                b"../i3bar/src/child.c\0" as *const u8 as *const ::core::ffi::c_char,
                681 as ::core::ffi::c_int,
                b"child_sig_cb\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    } else if (*c).pid == status_child.pid {
        if config.verbose {
            printf(
                b"[%s:%d] %s: status_command: pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
                b"../i3bar/src/child.c\0" as *const u8 as *const ::core::ffi::c_char,
                681 as ::core::ffi::c_int,
                b"child_sig_cb\0" as *const u8 as *const ::core::ffi::c_char,
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
                b"[%s:%d] %s: workspace_command: pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
                b"../i3bar/src/child.c\0" as *const u8 as *const ::core::ffi::c_char,
                681 as ::core::ffi::c_int,
                b"child_sig_cb\0" as *const u8 as *const ::core::ffi::c_char,
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
            b"[%s:%d] ERROR: %s: unknown child, this should never happen pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"../i3bar/src/child.c\0" as *const u8 as *const ::core::ffi::c_char,
            681 as ::core::ffi::c_int,
            b"child_sig_cb\0" as *const u8 as *const ::core::ffi::c_char,
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
            b"%s is not executable (exit %d)\0" as *const u8 as *const ::core::ffi::c_char,
            command_type,
            exit_status,
        );
    } else if exit_status == 127 as ::core::ffi::c_int {
        error_function_pointer.expect("non-null function pointer")(
            b"%s not found or is missing a library dependency (exit %d)\0" as *const u8
                as *const ::core::ffi::c_char,
            command_type,
            exit_status,
        );
    } else {
        error_function_pointer.expect("non-null function pointer")(
            b"%s process exited unexpectedly (exit %d)\0" as *const u8
                as *const ::core::ffi::c_char,
            command_type,
            exit_status,
        );
    }
    cleanup(c);
    draw_bars(false_0 != 0);
}
unsafe extern "C" fn child_write_output() {
    if status_child.click_events {
        let mut output: *const ::core::ffi::c_uchar = ::core::ptr::null::<::core::ffi::c_uchar>();
        let mut size: size_t = 0;
        let mut n: ssize_t = 0;
        yajl_gen_get_buf(gen, &raw mut output, &raw mut size);
        n = writeall(child_stdin, output as *const ::core::ffi::c_void, size);
        if n != -(1 as ::core::ffi::c_int) as ssize_t {
            n = writeall(
                child_stdin,
                b"\n\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
                1 as size_t,
            );
        }
        yajl_gen_clear(gen);
        if n == -(1 as ::core::ffi::c_int) as ssize_t {
            status_child.click_events = false_0 != 0;
            kill_child();
            set_statusline_error(
                b"child_write_output failed\0" as *const u8 as *const ::core::ffi::c_char,
            );
            draw_bars(false_0 != 0);
        }
    }
}
unsafe extern "C" fn sfork() -> pid_t {
    let pid: pid_t = fork() as pid_t;
    if pid == -(1 as ::core::ffi::c_int) {
        fprintf(
            stderr,
            b"[%s:%d] ERROR: Couldn't fork(): %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            b"../i3bar/src/child.c\0" as *const u8 as *const ::core::ffi::c_char,
            724 as ::core::ffi::c_int,
            strerror(*__errno_location()),
        );
        exit(EXIT_FAILURE);
    }
    return pid;
}
unsafe extern "C" fn spipe(mut pipedes: *mut ::core::ffi::c_int) {
    if pipe(pipedes) == -(1 as ::core::ffi::c_int) {
        err(
            EXIT_FAILURE,
            b"pipe(pipe_in)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
}
unsafe extern "C" fn exec_shell(mut command: *mut ::core::ffi::c_char) {
    execl(
        _PATH_BSHELL.as_ptr(),
        _PATH_BSHELL.as_ptr(),
        b"-c\0" as *const u8 as *const ::core::ffi::c_char,
        command,
        NULL as *mut ::core::ffi::c_char,
    );
    err(
        EXIT_FAILURE,
        b"execl return\0" as *const u8 as *const ::core::ffi::c_char,
    );
}
unsafe extern "C" fn setup_child_cb(mut child: *mut i3bar_child) {
    fcntl((*child).stdin_fd, F_SETFL, O_NONBLOCK);
    (*child).stdin_io = smalloc(::core::mem::size_of::<ev_io>() as size_t) as *mut ev_io;
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
        >() as size_t,
    );
    (*(*child).stdin_io).fd = (*child).stdin_fd;
    (*(*child).stdin_io).events = EV_READ as ::core::ffi::c_int | EV__IOFDSET as ::core::ffi::c_int;
    ev_io_start(main_loop as *mut ev_loop, (*child).stdin_io);
    (*child).child_sig = smalloc(::core::mem::size_of::<ev_child>() as size_t) as *mut ev_child;
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
        >() as size_t,
    );
    (*(*child).child_sig).pid = (*child).pid as ::core::ffi::c_int;
    (*(*child).child_sig).flags = (0 as ::core::ffi::c_int != 0) as ::core::ffi::c_int;
    ev_child_start(main_loop as *mut ev_loop, (*child).child_sig);
    if (*child).pid == 0 as ::core::ffi::c_int {
        if config.verbose {
            printf(
                b"[%s:%d] %s: child pid = 0\n\0" as *const u8 as *const ::core::ffi::c_char,
                b"../i3bar/src/child.c\0" as *const u8 as *const ::core::ffi::c_char,
                754 as ::core::ffi::c_int,
                b"setup_child_cb\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    } else if (*child).pid == status_child.pid {
        if config.verbose {
            printf(
                b"[%s:%d] %s: status_command: pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
                b"../i3bar/src/child.c\0" as *const u8 as *const ::core::ffi::c_char,
                754 as ::core::ffi::c_int,
                b"setup_child_cb\0" as *const u8 as *const ::core::ffi::c_char,
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
                b"[%s:%d] %s: workspace_command: pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
                b"../i3bar/src/child.c\0" as *const u8 as *const ::core::ffi::c_char,
                754 as ::core::ffi::c_int,
                b"setup_child_cb\0" as *const u8 as *const ::core::ffi::c_char,
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
            b"[%s:%d] ERROR: %s: unknown child, this should never happen pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"../i3bar/src/child.c\0" as *const u8 as *const ::core::ffi::c_char,
            754 as ::core::ffi::c_int,
            b"setup_child_cb\0" as *const u8 as *const ::core::ffi::c_char,
            (*child).pid as ::core::ffi::c_long,
            (*child).stopped as ::core::ffi::c_int,
            (*child).stop_signal,
            (*child).cont_signal,
            (*child).click_events as ::core::ffi::c_int,
            (*child).click_events_init as ::core::ffi::c_int,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn start_child(mut command: *mut ::core::ffi::c_char) {
    if command.is_null() {
        return;
    }
    static mut callbacks: yajl_callbacks = unsafe {
        yajl_callbacks {
            yajl_null: None,
            yajl_boolean: Some(
                stdin_boolean
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
            yajl_integer: Some(
                stdin_integer
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        ::core::ffi::c_longlong,
                    ) -> ::core::ffi::c_int,
            ),
            yajl_double: None,
            yajl_number: None,
            yajl_string: Some(
                stdin_string
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const ::core::ffi::c_uchar,
                        size_t,
                    ) -> ::core::ffi::c_int,
            ),
            yajl_start_map: Some(
                stdin_start_map
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
            ),
            yajl_map_key: Some(
                stdin_map_key
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const ::core::ffi::c_uchar,
                        size_t,
                    ) -> ::core::ffi::c_int,
            ),
            yajl_end_map: Some(
                stdin_end_map
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
            ),
            yajl_start_array: Some(
                stdin_start_array
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
            ),
            yajl_end_array: Some(
                stdin_end_array
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
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
        dup2(pipe_in[1 as ::core::ffi::c_int as usize], STDOUT_FILENO);
        dup2(pipe_out[0 as ::core::ffi::c_int as usize], STDIN_FILENO);
        setpgid(status_child.pid as __pid_t, 0 as __pid_t);
        exec_shell(command);
        return;
    }
    close(pipe_in[1 as ::core::ffi::c_int as usize]);
    close(pipe_out[0 as ::core::ffi::c_int as usize]);
    status_child.stdin_fd = pipe_in[0 as ::core::ffi::c_int as usize];
    child_stdin = pipe_out[1 as ::core::ffi::c_int as usize];
    status_child.version = -(1 as ::core::ffi::c_int) as uint32_t;
    setup_child_cb(&raw mut status_child);
}
#[no_mangle]
pub unsafe extern "C" fn start_ws_child(mut command: *mut ::core::ffi::c_char) {
    if command.is_null() {
        return;
    }
    ws_child.stop_signal = SIGSTOP;
    ws_child.cont_signal = SIGCONT;
    let mut pipe_in: [::core::ffi::c_int; 2] = [0; 2];
    spipe(&raw mut pipe_in as *mut ::core::ffi::c_int);
    ws_child.pid = sfork();
    if ws_child.pid == 0 as ::core::ffi::c_int {
        close(pipe_in[0 as ::core::ffi::c_int as usize]);
        dup2(pipe_in[1 as ::core::ffi::c_int as usize], STDOUT_FILENO);
        setpgid(ws_child.pid as __pid_t, 0 as __pid_t);
        exec_shell(command);
    }
    close(pipe_in[1 as ::core::ffi::c_int as usize]);
    ws_child.stdin_fd = pipe_in[0 as ::core::ffi::c_int as usize];
    setup_child_cb(&raw mut ws_child);
}
unsafe extern "C" fn child_click_events_initialize() {
    if status_child.pid == 0 as ::core::ffi::c_int {
        if config.verbose {
            printf(
                b"[%s:%d] %s: child pid = 0\n\0" as *const u8 as *const ::core::ffi::c_char,
                b"../i3bar/src/child.c\0" as *const u8 as *const ::core::ffi::c_char,
                847 as ::core::ffi::c_int,
                b"child_click_events_initialize\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    } else if status_child.pid == status_child.pid {
        if config.verbose {
            printf(
                b"[%s:%d] %s: status_command: pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
                b"../i3bar/src/child.c\0" as *const u8 as *const ::core::ffi::c_char,
                847 as ::core::ffi::c_int,
                b"child_click_events_initialize\0" as *const u8
                    as *const ::core::ffi::c_char,
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
                b"[%s:%d] %s: workspace_command: pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
                b"../i3bar/src/child.c\0" as *const u8 as *const ::core::ffi::c_char,
                847 as ::core::ffi::c_int,
                b"child_click_events_initialize\0" as *const u8
                    as *const ::core::ffi::c_char,
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
            b"[%s:%d] ERROR: %s: unknown child, this should never happen pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"../i3bar/src/child.c\0" as *const u8 as *const ::core::ffi::c_char,
            847 as ::core::ffi::c_int,
            b"child_click_events_initialize\0" as *const u8
                as *const ::core::ffi::c_char,
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
#[no_mangle]
pub unsafe extern "C" fn send_block_clicked(
    mut button: ::core::ffi::c_int,
    mut name: *const ::core::ffi::c_char,
    mut instance: *const ::core::ffi::c_char,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
    mut x_rel: ::core::ffi::c_int,
    mut y_rel: ::core::ffi::c_int,
    mut out_x: ::core::ffi::c_int,
    mut out_y: ::core::ffi::c_int,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut mods: ::core::ffi::c_int,
) {
    if !status_child.click_events {
        return;
    }
    child_click_events_initialize();
    yajl_gen_map_open(gen);
    if !name.is_null() {
        yajl_gen_string(
            gen,
            b"name\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
            strlen(b"name\0" as *const u8 as *const ::core::ffi::c_char),
        );
        yajl_gen_string(gen, name as *mut ::core::ffi::c_uchar, strlen(name));
    }
    if !instance.is_null() {
        yajl_gen_string(
            gen,
            b"instance\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
            strlen(b"instance\0" as *const u8 as *const ::core::ffi::c_char),
        );
        yajl_gen_string(gen, instance as *mut ::core::ffi::c_uchar, strlen(instance));
    }
    yajl_gen_string(
        gen,
        b"button\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
        strlen(b"button\0" as *const u8 as *const ::core::ffi::c_char),
    );
    yajl_gen_integer(gen, button as ::core::ffi::c_longlong);
    yajl_gen_string(
        gen,
        b"modifiers\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
        strlen(b"modifiers\0" as *const u8 as *const ::core::ffi::c_char),
    );
    yajl_gen_array_open(gen);
    if mods & XCB_MOD_MASK_SHIFT as ::core::ffi::c_int != 0 {
        yajl_gen_string(
            gen,
            b"Shift\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
            strlen(b"Shift\0" as *const u8 as *const ::core::ffi::c_char),
        );
    }
    if mods & XCB_MOD_MASK_CONTROL as ::core::ffi::c_int != 0 {
        yajl_gen_string(
            gen,
            b"Control\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
            strlen(b"Control\0" as *const u8 as *const ::core::ffi::c_char),
        );
    }
    if mods & XCB_MOD_MASK_1 as ::core::ffi::c_int != 0 {
        yajl_gen_string(
            gen,
            b"Mod1\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
            strlen(b"Mod1\0" as *const u8 as *const ::core::ffi::c_char),
        );
    }
    if mods & XCB_MOD_MASK_2 as ::core::ffi::c_int != 0 {
        yajl_gen_string(
            gen,
            b"Mod2\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
            strlen(b"Mod2\0" as *const u8 as *const ::core::ffi::c_char),
        );
    }
    if mods & XCB_MOD_MASK_3 as ::core::ffi::c_int != 0 {
        yajl_gen_string(
            gen,
            b"Mod3\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
            strlen(b"Mod3\0" as *const u8 as *const ::core::ffi::c_char),
        );
    }
    if mods & XCB_MOD_MASK_4 as ::core::ffi::c_int != 0 {
        yajl_gen_string(
            gen,
            b"Mod4\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
            strlen(b"Mod4\0" as *const u8 as *const ::core::ffi::c_char),
        );
    }
    if mods & XCB_MOD_MASK_5 as ::core::ffi::c_int != 0 {
        yajl_gen_string(
            gen,
            b"Mod5\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
            strlen(b"Mod5\0" as *const u8 as *const ::core::ffi::c_char),
        );
    }
    yajl_gen_array_close(gen);
    yajl_gen_string(
        gen,
        b"x\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
        strlen(b"x\0" as *const u8 as *const ::core::ffi::c_char),
    );
    yajl_gen_integer(gen, x as ::core::ffi::c_longlong);
    yajl_gen_string(
        gen,
        b"y\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
        strlen(b"y\0" as *const u8 as *const ::core::ffi::c_char),
    );
    yajl_gen_integer(gen, y as ::core::ffi::c_longlong);
    yajl_gen_string(
        gen,
        b"relative_x\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
        strlen(b"relative_x\0" as *const u8 as *const ::core::ffi::c_char),
    );
    yajl_gen_integer(gen, x_rel as ::core::ffi::c_longlong);
    yajl_gen_string(
        gen,
        b"relative_y\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
        strlen(b"relative_y\0" as *const u8 as *const ::core::ffi::c_char),
    );
    yajl_gen_integer(gen, y_rel as ::core::ffi::c_longlong);
    yajl_gen_string(
        gen,
        b"output_x\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
        strlen(b"output_x\0" as *const u8 as *const ::core::ffi::c_char),
    );
    yajl_gen_integer(gen, out_x as ::core::ffi::c_longlong);
    yajl_gen_string(
        gen,
        b"output_y\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
        strlen(b"output_y\0" as *const u8 as *const ::core::ffi::c_char),
    );
    yajl_gen_integer(gen, out_y as ::core::ffi::c_longlong);
    yajl_gen_string(
        gen,
        b"width\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
        strlen(b"width\0" as *const u8 as *const ::core::ffi::c_char),
    );
    yajl_gen_integer(gen, width as ::core::ffi::c_longlong);
    yajl_gen_string(
        gen,
        b"height\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
        strlen(b"height\0" as *const u8 as *const ::core::ffi::c_char),
    );
    yajl_gen_integer(gen, height as ::core::ffi::c_longlong);
    yajl_gen_map_close(gen);
    child_write_output();
}
unsafe extern "C" fn is_alive(mut c: *mut i3bar_child) -> bool {
    return (*c).pid > 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn status_child_is_alive() -> bool {
    return is_alive(&raw mut status_child);
}
#[no_mangle]
pub unsafe extern "C" fn ws_child_is_alive() -> bool {
    return is_alive(&raw mut ws_child);
}
#[no_mangle]
pub unsafe extern "C" fn kill_children_at_exit() {
    if status_child.pid == 0 as ::core::ffi::c_int {
        if config.verbose {
            printf(
                b"[%s:%d] %s: child pid = 0\n\0" as *const u8 as *const ::core::ffi::c_char,
                b"../i3bar/src/child.c\0" as *const u8 as *const ::core::ffi::c_char,
                960 as ::core::ffi::c_int,
                b"kill_children_at_exit\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    } else if status_child.pid == status_child.pid {
        if config.verbose {
            printf(
                b"[%s:%d] %s: status_command: pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
                b"../i3bar/src/child.c\0" as *const u8 as *const ::core::ffi::c_char,
                960 as ::core::ffi::c_int,
                b"kill_children_at_exit\0" as *const u8 as *const ::core::ffi::c_char,
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
                b"[%s:%d] %s: workspace_command: pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
                b"../i3bar/src/child.c\0" as *const u8 as *const ::core::ffi::c_char,
                960 as ::core::ffi::c_int,
                b"kill_children_at_exit\0" as *const u8 as *const ::core::ffi::c_char,
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
            b"[%s:%d] ERROR: %s: unknown child, this should never happen pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"../i3bar/src/child.c\0" as *const u8 as *const ::core::ffi::c_char,
            960 as ::core::ffi::c_int,
            b"kill_children_at_exit\0" as *const u8 as *const ::core::ffi::c_char,
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
                b"[%s:%d] %s: child pid = 0\n\0" as *const u8 as *const ::core::ffi::c_char,
                b"../i3bar/src/child.c\0" as *const u8 as *const ::core::ffi::c_char,
                960 as ::core::ffi::c_int,
                b"kill_children_at_exit\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    } else if ws_child.pid == status_child.pid {
        if config.verbose {
            printf(
                b"[%s:%d] %s: status_command: pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
                b"../i3bar/src/child.c\0" as *const u8 as *const ::core::ffi::c_char,
                960 as ::core::ffi::c_int,
                b"kill_children_at_exit\0" as *const u8 as *const ::core::ffi::c_char,
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
                b"[%s:%d] %s: workspace_command: pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
                b"../i3bar/src/child.c\0" as *const u8 as *const ::core::ffi::c_char,
                960 as ::core::ffi::c_int,
                b"kill_children_at_exit\0" as *const u8 as *const ::core::ffi::c_char,
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
            b"[%s:%d] ERROR: %s: unknown child, this should never happen pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"../i3bar/src/child.c\0" as *const u8 as *const ::core::ffi::c_char,
            960 as ::core::ffi::c_int,
            b"kill_children_at_exit\0" as *const u8 as *const ::core::ffi::c_char,
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
        killpg(status_child.pid as __pid_t, SIGTERM);
    }
    if is_alive(&raw mut ws_child) {
        killpg(ws_child.pid as __pid_t, SIGTERM);
    }
}
unsafe extern "C" fn cont_child(mut c: *mut i3bar_child) {
    if is_alive(c) as ::core::ffi::c_int != 0
        && (*c).cont_signal > 0 as ::core::ffi::c_int
        && (*c).stopped as ::core::ffi::c_int != 0
    {
        (*c).stopped = false_0 != 0;
        killpg((*c).pid as __pid_t, (*c).cont_signal);
    }
}
unsafe extern "C" fn kill_and_wait(mut c: *mut i3bar_child) {
    if (*c).pid == 0 as ::core::ffi::c_int {
        if config.verbose {
            printf(
                b"[%s:%d] %s: child pid = 0\n\0" as *const u8 as *const ::core::ffi::c_char,
                b"../i3bar/src/child.c\0" as *const u8 as *const ::core::ffi::c_char,
                979 as ::core::ffi::c_int,
                b"kill_and_wait\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    } else if (*c).pid == status_child.pid {
        if config.verbose {
            printf(
                b"[%s:%d] %s: status_command: pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
                b"../i3bar/src/child.c\0" as *const u8 as *const ::core::ffi::c_char,
                979 as ::core::ffi::c_int,
                b"kill_and_wait\0" as *const u8 as *const ::core::ffi::c_char,
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
                b"[%s:%d] %s: workspace_command: pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
                b"../i3bar/src/child.c\0" as *const u8 as *const ::core::ffi::c_char,
                979 as ::core::ffi::c_int,
                b"kill_and_wait\0" as *const u8 as *const ::core::ffi::c_char,
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
            b"[%s:%d] ERROR: %s: unknown child, this should never happen pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"../i3bar/src/child.c\0" as *const u8 as *const ::core::ffi::c_char,
            979 as ::core::ffi::c_int,
            b"kill_and_wait\0" as *const u8 as *const ::core::ffi::c_char,
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
    killpg((*c).pid as __pid_t, SIGTERM);
    let mut status: ::core::ffi::c_int = 0;
    waitpid(
        (*c).pid as __pid_t,
        &raw mut status,
        0 as ::core::ffi::c_int,
    );
    cleanup(c);
}
#[no_mangle]
pub unsafe extern "C" fn kill_child() {
    kill_and_wait(&raw mut status_child);
}
#[no_mangle]
pub unsafe extern "C" fn kill_ws_child() {
    kill_and_wait(&raw mut ws_child);
}
unsafe extern "C" fn stop_child(mut c: *mut i3bar_child) {
    if (*c).stop_signal > 0 as ::core::ffi::c_int && !(*c).stopped {
        (*c).stopped = true_0 != 0;
        killpg((*c).pid as __pid_t, (*c).stop_signal);
    }
}
#[no_mangle]
pub unsafe extern "C" fn stop_children() {
    if status_child.pid == 0 as ::core::ffi::c_int {
        if config.verbose {
            printf(
                b"[%s:%d] %s: child pid = 0\n\0" as *const u8 as *const ::core::ffi::c_char,
                b"../i3bar/src/child.c\0" as *const u8 as *const ::core::ffi::c_char,
                1022 as ::core::ffi::c_int,
                b"stop_children\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    } else if status_child.pid == status_child.pid {
        if config.verbose {
            printf(
                b"[%s:%d] %s: status_command: pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
                b"../i3bar/src/child.c\0" as *const u8 as *const ::core::ffi::c_char,
                1022 as ::core::ffi::c_int,
                b"stop_children\0" as *const u8 as *const ::core::ffi::c_char,
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
                b"[%s:%d] %s: workspace_command: pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
                b"../i3bar/src/child.c\0" as *const u8 as *const ::core::ffi::c_char,
                1022 as ::core::ffi::c_int,
                b"stop_children\0" as *const u8 as *const ::core::ffi::c_char,
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
            b"[%s:%d] ERROR: %s: unknown child, this should never happen pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"../i3bar/src/child.c\0" as *const u8 as *const ::core::ffi::c_char,
            1022 as ::core::ffi::c_int,
            b"stop_children\0" as *const u8 as *const ::core::ffi::c_char,
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
                b"[%s:%d] %s: child pid = 0\n\0" as *const u8 as *const ::core::ffi::c_char,
                b"../i3bar/src/child.c\0" as *const u8 as *const ::core::ffi::c_char,
                1022 as ::core::ffi::c_int,
                b"stop_children\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    } else if ws_child.pid == status_child.pid {
        if config.verbose {
            printf(
                b"[%s:%d] %s: status_command: pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
                b"../i3bar/src/child.c\0" as *const u8 as *const ::core::ffi::c_char,
                1022 as ::core::ffi::c_int,
                b"stop_children\0" as *const u8 as *const ::core::ffi::c_char,
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
                b"[%s:%d] %s: workspace_command: pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
                b"../i3bar/src/child.c\0" as *const u8 as *const ::core::ffi::c_char,
                1022 as ::core::ffi::c_int,
                b"stop_children\0" as *const u8 as *const ::core::ffi::c_char,
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
            b"[%s:%d] ERROR: %s: unknown child, this should never happen pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"../i3bar/src/child.c\0" as *const u8 as *const ::core::ffi::c_char,
            1022 as ::core::ffi::c_int,
            b"stop_children\0" as *const u8 as *const ::core::ffi::c_char,
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
#[no_mangle]
pub unsafe extern "C" fn cont_children() {
    if status_child.pid == 0 as ::core::ffi::c_int {
        if config.verbose {
            printf(
                b"[%s:%d] %s: child pid = 0\n\0" as *const u8 as *const ::core::ffi::c_char,
                b"../i3bar/src/child.c\0" as *const u8 as *const ::core::ffi::c_char,
                1032 as ::core::ffi::c_int,
                b"cont_children\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    } else if status_child.pid == status_child.pid {
        if config.verbose {
            printf(
                b"[%s:%d] %s: status_command: pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
                b"../i3bar/src/child.c\0" as *const u8 as *const ::core::ffi::c_char,
                1032 as ::core::ffi::c_int,
                b"cont_children\0" as *const u8 as *const ::core::ffi::c_char,
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
                b"[%s:%d] %s: workspace_command: pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
                b"../i3bar/src/child.c\0" as *const u8 as *const ::core::ffi::c_char,
                1032 as ::core::ffi::c_int,
                b"cont_children\0" as *const u8 as *const ::core::ffi::c_char,
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
            b"[%s:%d] ERROR: %s: unknown child, this should never happen pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"../i3bar/src/child.c\0" as *const u8 as *const ::core::ffi::c_char,
            1032 as ::core::ffi::c_int,
            b"cont_children\0" as *const u8 as *const ::core::ffi::c_char,
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
                b"[%s:%d] %s: child pid = 0\n\0" as *const u8 as *const ::core::ffi::c_char,
                b"../i3bar/src/child.c\0" as *const u8 as *const ::core::ffi::c_char,
                1032 as ::core::ffi::c_int,
                b"cont_children\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    } else if ws_child.pid == status_child.pid {
        if config.verbose {
            printf(
                b"[%s:%d] %s: status_command: pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
                b"../i3bar/src/child.c\0" as *const u8 as *const ::core::ffi::c_char,
                1032 as ::core::ffi::c_int,
                b"cont_children\0" as *const u8 as *const ::core::ffi::c_char,
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
                b"[%s:%d] %s: workspace_command: pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
                b"../i3bar/src/child.c\0" as *const u8 as *const ::core::ffi::c_char,
                1032 as ::core::ffi::c_int,
                b"cont_children\0" as *const u8 as *const ::core::ffi::c_char,
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
            b"[%s:%d] ERROR: %s: unknown child, this should never happen pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"../i3bar/src/child.c\0" as *const u8 as *const ::core::ffi::c_char,
            1032 as ::core::ffi::c_int,
            b"cont_children\0" as *const u8 as *const ::core::ffi::c_char,
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
#[no_mangle]
pub unsafe extern "C" fn child_want_click_events() -> bool {
    return status_child.click_events;
}
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
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
