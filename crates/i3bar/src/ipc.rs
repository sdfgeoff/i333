/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3bar - an xcb-based status- and ws-bar for i3
 * © 2010 Axel Wagner and contributors (see also: LICENSE)
 *
 * ipc.c: Communicating with i3
 *
 */
// Mechanical file-by-file Rust translation of i3bar/src/ipc.c.
// Generated with C2Rust 0.22.1; C ABI and original ownership retained.
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type _cairo;
    pub type _cairo_surface;
    pub type _i3String;
    pub type ev_loop;
    static mut stderr: *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn read(__fd: ::core::ffi::c_int, __buf: *mut ::core::ffi::c_void, __nbytes: size_t)
        -> ssize_t;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn verboselog(fmt: *mut ::core::ffi::c_char, ...);
    fn smalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn sasprintf(
        strp: *mut *mut ::core::ffi::c_char,
        fmt: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn swrite(fd: ::core::ffi::c_int, buf: *const ::core::ffi::c_void, count: size_t) -> ssize_t;
    fn ipc_connect(socket_path: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    static mut main_loop: *mut ev_loop;
    static mut statusline_head: statusline_head;
    fn ev_io_start(loop_0: *mut ev_loop, w: *mut ev_io);
    fn ev_io_stop(loop_0: *mut ev_loop, w: *mut ev_io);
    fn clear_statusline(head: *mut statusline_head, free_resources: bool);
    fn start_child(command: *mut ::core::ffi::c_char);
    fn start_ws_child(command: *mut ::core::ffi::c_char);
    fn status_child_is_alive() -> bool;
    fn ws_child_is_alive() -> bool;
    fn kill_child();
    fn kill_ws_child();
    fn repeat_last_ws_json();
    static mut outputs: *mut outputs_head;
    fn parse_outputs_json(json: *const ::core::ffi::c_uchar, size: size_t);
    fn free_outputs();
    fn parse_workspaces_json(json: *const ::core::ffi::c_uchar, size: size_t);
    fn free_workspaces();
    fn parse_mode_json(json: *const ::core::ffi::c_uchar, size: size_t);
    fn init_xcb_late(fontname: *mut ::core::ffi::c_char);
    fn init_colors(colors: *const xcb_color_strings_t);
    fn clean_xcb();
    fn kick_tray_clients(output: *mut i3_output);
    fn reconfig_windows(redraw_bars: bool);
    fn draw_bars(force_unhide: bool);
    static mut config: config_t;
    fn parse_config_json(json: *const ::core::ffi::c_uchar, size: size_t);
    fn parse_get_first_i3bar_config(json: *const ::core::ffi::c_uchar, size: size_t);
    fn free_colors(colors: *mut xcb_color_strings_t);
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
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn exit(__status: ::core::ffi::c_int) -> !;
}
pub type size_t = usize;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type ssize_t = isize;
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
pub type handler_t = Option<unsafe extern "C" fn(*const ::core::ffi::c_uchar, size_t) -> ()>;
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
    pub slist: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
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
    pub tailq: C2RustUnnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
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
    pub tailq: C2RustUnnamed_6,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub tqe_next: *mut i3_ws,
    pub tqe_prev: *mut *mut i3_ws,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct outputs_head {
    pub slh_first: *mut i3_output,
}
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const I3_IPC_MAGIC: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"i3-ipc\0") };
pub const I3_IPC_MESSAGE_TYPE_GET_WORKSPACES: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const I3_IPC_MESSAGE_TYPE_SUBSCRIBE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const I3_IPC_MESSAGE_TYPE_GET_OUTPUTS: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const I3_IPC_MESSAGE_TYPE_GET_BAR_CONFIG: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
#[no_mangle]
pub static mut i3_connection: *mut ev_io = ::core::ptr::null::<ev_io>() as *mut ev_io;
#[no_mangle]
pub static mut sock_path: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
unsafe extern "C" fn i3_provides_workspaces() -> bool {
    return !config.disable_ws && config.workspace_command.is_null();
}
unsafe extern "C" fn got_command_reply(mut reply: *const ::core::ffi::c_uchar, mut size: size_t) {}
unsafe extern "C" fn got_workspace_reply(mut reply: *const ::core::ffi::c_uchar, mut size: size_t) {
    if config.verbose {
        printf(
            b"[%s:%d] Got workspace data!\n\0" as *const u8 as *const ::core::ffi::c_char,
            b"../i3bar/src/ipc.c\0" as *const u8 as *const ::core::ffi::c_char,
            53 as ::core::ffi::c_int,
        );
    }
    parse_workspaces_json(reply, size);
    draw_bars(false_0 != 0);
}
unsafe extern "C" fn got_subscribe_reply(mut reply: *const ::core::ffi::c_uchar, mut size: size_t) {
    if config.verbose {
        printf(
            b"[%s:%d] Got subscribe reply: %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            b"../i3bar/src/ipc.c\0" as *const u8 as *const ::core::ffi::c_char,
            64 as ::core::ffi::c_int,
            reply,
        );
    }
}
unsafe extern "C" fn got_output_reply(mut reply: *const ::core::ffi::c_uchar, mut size: size_t) {
    if config.verbose {
        printf(
            b"[%s:%d] Clearing old output configuration...\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"../i3bar/src/ipc.c\0" as *const u8 as *const ::core::ffi::c_char,
            73 as ::core::ffi::c_int,
        );
    }
    free_outputs();
    if config.verbose {
        printf(
            b"[%s:%d] Parsing outputs JSON...\n\0" as *const u8 as *const ::core::ffi::c_char,
            b"../i3bar/src/ipc.c\0" as *const u8 as *const ::core::ffi::c_char,
            76 as ::core::ffi::c_int,
        );
    }
    parse_outputs_json(reply, size);
    if config.verbose {
        printf(
            b"[%s:%d] Reconfiguring windows...\n\0" as *const u8 as *const ::core::ffi::c_char,
            b"../i3bar/src/ipc.c\0" as *const u8 as *const ::core::ffi::c_char,
            78 as ::core::ffi::c_int,
        );
    }
    reconfig_windows(false_0 != 0);
    let mut o_walk: *mut i3_output = ::core::ptr::null_mut::<i3_output>();
    o_walk = (*outputs).slh_first as *mut i3_output;
    while !o_walk.is_null() {
        kick_tray_clients(o_walk);
        o_walk = (*o_walk).slist.sle_next as *mut i3_output;
    }
    if i3_provides_workspaces() {
        i3_send_msg(
            I3_IPC_MESSAGE_TYPE_GET_WORKSPACES as uint32_t,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
    } else if !config.workspace_command.is_null() {
        repeat_last_ws_json();
    }
    draw_bars(false_0 != 0);
}
unsafe extern "C" fn got_bar_config(mut reply: *const ::core::ffi::c_uchar, mut size: size_t) {
    if config.bar_id.is_null() {
        if config.verbose {
            printf(
                b"[%s:%d] Received bar list \"%s\"\n\0" as *const u8 as *const ::core::ffi::c_char,
                b"../i3bar/src/ipc.c\0" as *const u8 as *const ::core::ffi::c_char,
                110 as ::core::ffi::c_int,
                reply,
            );
        }
        parse_get_first_i3bar_config(reply, size);
        if config.bar_id.is_null() {
            fprintf(
                stderr,
                b"[%s:%d] ERROR: No bar configuration found, please configure a bar block in your i3 config file.\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
                b"../i3bar/src/ipc.c\0" as *const u8 as *const ::core::ffi::c_char,
                114 as ::core::ffi::c_int,
            );
            exit(EXIT_FAILURE);
        }
        verboselog(
            b"[libi3] ../i3bar/src/ipc.c Using first bar config: %s. Use --bar_id to manually select a different bar configuration.\n\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            config.bar_id,
        );
        i3_send_msg(
            I3_IPC_MESSAGE_TYPE_GET_BAR_CONFIG as uint32_t,
            config.bar_id,
        );
        return;
    }
    if config.verbose {
        printf(
            b"[%s:%d] Received bar config \"%s\"\n\0" as *const u8 as *const ::core::ffi::c_char,
            b"../i3bar/src/ipc.c\0" as *const u8 as *const ::core::ffi::c_char,
            123 as ::core::ffi::c_int,
            reply,
        );
    }
    i3_send_msg(
        I3_IPC_MESSAGE_TYPE_GET_OUTPUTS as uint32_t,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    free_colors(&raw mut config.colors);
    parse_config_json(reply, size);
    subscribe_events();
    if i3_provides_workspaces() {
        i3_send_msg(
            I3_IPC_MESSAGE_TYPE_GET_WORKSPACES as uint32_t,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
    }
    init_xcb_late(config.fontname);
    init_colors(&raw mut config.colors);
    start_child(config.command);
    start_ws_child(config.workspace_command);
}
#[no_mangle]
pub static mut reply_handlers: [handler_t; 12] = [
    Some(got_command_reply as unsafe extern "C" fn(*const ::core::ffi::c_uchar, size_t) -> ()),
    Some(got_workspace_reply as unsafe extern "C" fn(*const ::core::ffi::c_uchar, size_t) -> ()),
    Some(got_subscribe_reply as unsafe extern "C" fn(*const ::core::ffi::c_uchar, size_t) -> ()),
    Some(got_output_reply as unsafe extern "C" fn(*const ::core::ffi::c_uchar, size_t) -> ()),
    None,
    None,
    Some(got_bar_config as unsafe extern "C" fn(*const ::core::ffi::c_uchar, size_t) -> ()),
    None,
    None,
    None,
    None,
    None,
];
unsafe extern "C" fn got_workspace_event(mut event: *const ::core::ffi::c_uchar, mut size: size_t) {
    if config.verbose {
        printf(
            b"[%s:%d] Got workspace event!\n\0" as *const u8 as *const ::core::ffi::c_char,
            b"../i3bar/src/ipc.c\0" as *const u8 as *const ::core::ffi::c_char,
            170 as ::core::ffi::c_int,
        );
    }
    i3_send_msg(
        I3_IPC_MESSAGE_TYPE_GET_WORKSPACES as uint32_t,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
}
unsafe extern "C" fn got_output_event(mut event: *const ::core::ffi::c_uchar, mut size: size_t) {
    if config.verbose {
        printf(
            b"[%s:%d] Got output event!\n\0" as *const u8 as *const ::core::ffi::c_char,
            b"../i3bar/src/ipc.c\0" as *const u8 as *const ::core::ffi::c_char,
            179 as ::core::ffi::c_int,
        );
    }
    i3_send_msg(
        I3_IPC_MESSAGE_TYPE_GET_OUTPUTS as uint32_t,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
}
unsafe extern "C" fn got_mode_event(mut event: *const ::core::ffi::c_uchar, mut size: size_t) {
    if config.verbose {
        printf(
            b"[%s:%d] Got mode event!\n\0" as *const u8 as *const ::core::ffi::c_char,
            b"../i3bar/src/ipc.c\0" as *const u8 as *const ::core::ffi::c_char,
            188 as ::core::ffi::c_int,
        );
    }
    parse_mode_json(event, size);
    draw_bars(false_0 != 0);
}
unsafe extern "C" fn strings_differ(
    mut a: *mut ::core::ffi::c_char,
    mut b: *mut ::core::ffi::c_char,
) -> bool {
    let a_null: bool = a.is_null();
    let b_null: bool = b.is_null();
    if a_null as ::core::ffi::c_int != b_null as ::core::ffi::c_int {
        return true_0 != 0;
    }
    if a_null as ::core::ffi::c_int != 0 && b_null as ::core::ffi::c_int != 0 {
        return false_0 != 0;
    }
    return strcmp(a, b) != 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn got_bar_config_update(
    mut event: *const ::core::ffi::c_uchar,
    mut size: size_t,
) {
    let mut expected_id: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    sasprintf(
        &raw mut expected_id,
        b"\"id\":\"%s\"\0" as *const u8 as *const ::core::ffi::c_char,
        config.bar_id,
    );
    let mut found_id: *const ::core::ffi::c_char =
        strstr(event as *const ::core::ffi::c_char, expected_id);
    free(expected_id as *mut ::core::ffi::c_void);
    expected_id = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if found_id.is_null() {
        return;
    }
    i3_send_msg(
        I3_IPC_MESSAGE_TYPE_GET_OUTPUTS as uint32_t,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    free_colors(&raw mut config.colors);
    if config.verbose {
        printf(
            b"[%s:%d] Received bar config update \"%s\"\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"../i3bar/src/ipc.c\0" as *const u8 as *const ::core::ffi::c_char,
            225 as ::core::ffi::c_int,
            event,
        );
    }
    let mut old_command: *mut ::core::ffi::c_char = config.command;
    let mut old_workspace_command: *mut ::core::ffi::c_char = config.workspace_command;
    config.command = ::core::ptr::null_mut::<::core::ffi::c_char>();
    config.workspace_command = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut old_mode: bar_display_mode_t = config.hide_on_modifier;
    parse_config_json(event, size);
    if old_mode as ::core::ffi::c_uint != config.hide_on_modifier as ::core::ffi::c_uint {
        reconfig_windows(true_0 != 0);
    }
    init_xcb_late(config.fontname);
    init_colors(&raw mut config.colors);
    if !status_child_is_alive()
        || strings_differ(old_command, config.command) as ::core::ffi::c_int != 0
    {
        kill_child();
        clear_statusline(&raw mut statusline_head, true_0 != 0);
        start_child(config.command);
    }
    free(old_command as *mut ::core::ffi::c_void);
    if !ws_child_is_alive()
        || strings_differ(old_workspace_command, config.workspace_command) as ::core::ffi::c_int
            != 0
    {
        free_workspaces();
        kill_ws_child();
        start_ws_child(config.workspace_command);
    }
    free(old_workspace_command as *mut ::core::ffi::c_void);
    draw_bars(false_0 != 0);
}
#[no_mangle]
pub static mut event_handlers: [handler_t; 5] = [
    Some(got_workspace_event as unsafe extern "C" fn(*const ::core::ffi::c_uchar, size_t) -> ()),
    Some(got_output_event as unsafe extern "C" fn(*const ::core::ffi::c_uchar, size_t) -> ()),
    Some(got_mode_event as unsafe extern "C" fn(*const ::core::ffi::c_uchar, size_t) -> ()),
    None,
    Some(got_bar_config_update as unsafe extern "C" fn(*const ::core::ffi::c_uchar, size_t) -> ()),
];
unsafe extern "C" fn got_data(
    mut loop_0: *mut ev_loop,
    mut watcher: *mut ev_io,
    mut events: ::core::ffi::c_int,
) {
    if config.verbose {
        printf(
            b"[%s:%d] Got data!\n\0" as *const u8 as *const ::core::ffi::c_char,
            b"../i3bar/src/ipc.c\0" as *const u8 as *const ::core::ffi::c_char,
            275 as ::core::ffi::c_int,
        );
    }
    let mut fd: ::core::ffi::c_int = (*watcher).fd;
    let mut header_len: uint32_t = strlen(I3_IPC_MAGIC.as_ptr())
        .wrapping_add((::core::mem::size_of::<uint32_t>() as size_t).wrapping_mul(2 as size_t))
        as uint32_t;
    let mut header: *mut ::core::ffi::c_char =
        smalloc(header_len as size_t) as *mut ::core::ffi::c_char;
    let mut rec: uint32_t = 0 as uint32_t;
    while rec < header_len {
        let n: ::core::ffi::c_int = read(
            fd,
            header.offset(rec as isize) as *mut ::core::ffi::c_void,
            header_len.wrapping_sub(rec) as size_t,
        ) as ::core::ffi::c_int;
        if n == -(1 as ::core::ffi::c_int) {
            fprintf(
                stderr,
                b"[%s:%d] ERROR: read() failed: %s\n\0" as *const u8 as *const ::core::ffi::c_char,
                b"../i3bar/src/ipc.c\0" as *const u8 as *const ::core::ffi::c_char,
                288 as ::core::ffi::c_int,
                strerror(*__errno_location()),
            );
            exit(EXIT_FAILURE);
        }
        if n == 0 as ::core::ffi::c_int {
            if config.verbose {
                printf(
                    b"[%s:%d] EOF received, exiting...\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    b"../i3bar/src/ipc.c\0" as *const u8 as *const ::core::ffi::c_char,
                    294 as ::core::ffi::c_int,
                );
            }
            clean_xcb();
            free(header as *mut ::core::ffi::c_void);
            exit(EXIT_SUCCESS);
        }
        rec = rec.wrapping_add(n as uint32_t);
    }
    if strncmp(header, I3_IPC_MAGIC.as_ptr(), strlen(I3_IPC_MAGIC.as_ptr())) != 0 {
        fprintf(
            stderr,
            b"[%s:%d] ERROR: Wrong magic code: %.*s\n Expected: %s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"../i3bar/src/ipc.c\0" as *const u8 as *const ::core::ffi::c_char,
            309 as ::core::ffi::c_int,
            strlen(I3_IPC_MAGIC.as_ptr()) as ::core::ffi::c_int,
            header,
            I3_IPC_MAGIC.as_ptr(),
        );
        exit(EXIT_FAILURE);
    }
    let mut walk: *mut ::core::ffi::c_char = header.offset(strlen(I3_IPC_MAGIC.as_ptr()) as isize);
    let mut size: uint32_t = 0;
    memcpy(
        &raw mut size as *mut ::core::ffi::c_void,
        walk as *mut uint32_t as *const ::core::ffi::c_void,
        ::core::mem::size_of::<uint32_t>() as size_t,
    );
    walk = walk.offset(::core::mem::size_of::<uint32_t>() as usize as isize);
    let mut type_0: uint32_t = 0;
    memcpy(
        &raw mut type_0 as *mut ::core::ffi::c_void,
        walk as *mut uint32_t as *const ::core::ffi::c_void,
        ::core::mem::size_of::<uint32_t>() as size_t,
    );
    let mut buffer: *mut ::core::ffi::c_uchar =
        smalloc(size.wrapping_add(1 as uint32_t) as size_t) as *mut ::core::ffi::c_uchar;
    rec = 0 as uint32_t;
    while rec < size {
        let mut n_0: ::core::ffi::c_int = read(
            fd,
            buffer.offset(rec as isize) as *mut ::core::ffi::c_void,
            size.wrapping_sub(rec) as size_t,
        ) as ::core::ffi::c_int;
        if n_0 == -(1 as ::core::ffi::c_int) {
            fprintf(
                stderr,
                b"[%s:%d] ERROR: read() failed: %s\n\0" as *const u8 as *const ::core::ffi::c_char,
                b"../i3bar/src/ipc.c\0" as *const u8 as *const ::core::ffi::c_char,
                328 as ::core::ffi::c_int,
                strerror(*__errno_location()),
            );
            exit(EXIT_FAILURE);
        }
        if n_0 == 0 as ::core::ffi::c_int {
            fprintf(
                stderr,
                b"[%s:%d] ERROR: Nothing to read!\n\0" as *const u8 as *const ::core::ffi::c_char,
                b"../i3bar/src/ipc.c\0" as *const u8 as *const ::core::ffi::c_char,
                332 as ::core::ffi::c_int,
            );
            exit(EXIT_FAILURE);
        }
        rec = rec.wrapping_add(n_0 as uint32_t);
    }
    *buffer.offset(size as isize) = '\0' as i32 as ::core::ffi::c_uchar;
    if type_0 as ::core::ffi::c_ulong & (1 as ::core::ffi::c_ulong) << 31 as ::core::ffi::c_int != 0
    {
        type_0 = (type_0 as ::core::ffi::c_ulong
            ^ (1 as ::core::ffi::c_ulong) << 31 as ::core::ffi::c_int) as uint32_t;
        event_handlers[type_0 as usize].expect("non-null function pointer")(buffer, size as size_t);
    } else if reply_handlers[type_0 as usize].is_some() {
        reply_handlers[type_0 as usize].expect("non-null function pointer")(buffer, size as size_t);
    }
    free(header as *mut ::core::ffi::c_void);
    header = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free(buffer as *mut ::core::ffi::c_void);
    buffer = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
}
#[no_mangle]
pub unsafe extern "C" fn i3_send_msg(
    mut type_0: uint32_t,
    mut payload: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut len: uint32_t = 0 as uint32_t;
    if !payload.is_null() {
        len = strlen(payload) as uint32_t;
    }
    let mut to_write: uint32_t = strlen(I3_IPC_MAGIC.as_ptr())
        .wrapping_add((::core::mem::size_of::<uint32_t>() as size_t).wrapping_mul(2 as size_t))
        .wrapping_add(len as size_t) as uint32_t;
    let mut buffer: *mut ::core::ffi::c_char =
        smalloc(to_write as size_t) as *mut ::core::ffi::c_char;
    let mut walk: *mut ::core::ffi::c_char = buffer;
    memcpy(
        buffer as *mut ::core::ffi::c_void,
        I3_IPC_MAGIC.as_ptr() as *const ::core::ffi::c_void,
        strlen(I3_IPC_MAGIC.as_ptr()),
    );
    walk = walk.offset(strlen(I3_IPC_MAGIC.as_ptr()) as isize);
    memcpy(
        walk as *mut ::core::ffi::c_void,
        &raw mut len as *const ::core::ffi::c_void,
        ::core::mem::size_of::<uint32_t>() as size_t,
    );
    walk = walk.offset(::core::mem::size_of::<uint32_t>() as usize as isize);
    memcpy(
        walk as *mut ::core::ffi::c_void,
        &raw mut type_0 as *const ::core::ffi::c_void,
        ::core::mem::size_of::<uint32_t>() as size_t,
    );
    walk = walk.offset(::core::mem::size_of::<uint32_t>() as usize as isize);
    if !payload.is_null() {
        memcpy(
            walk as *mut ::core::ffi::c_void,
            payload as *const ::core::ffi::c_void,
            len as size_t,
        );
    }
    swrite(
        (*i3_connection).fd,
        buffer as *const ::core::ffi::c_void,
        to_write as size_t,
    );
    free(buffer as *mut ::core::ffi::c_void);
    buffer = ::core::ptr::null_mut::<::core::ffi::c_char>();
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn init_connection(mut socket_path: *const ::core::ffi::c_char) {
    sock_path = socket_path;
    let mut sockfd: ::core::ffi::c_int = ipc_connect(socket_path);
    i3_connection = smalloc(::core::mem::size_of::<ev_io>() as size_t) as *mut ev_io;
    let ref mut fresh0 = (*(i3_connection as *mut ::core::ffi::c_void as *mut ev_watcher)).pending;
    *fresh0 = 0 as ::core::ffi::c_int;
    (*(i3_connection as *mut ::core::ffi::c_void as *mut ev_watcher)).active = *fresh0;
    (*(i3_connection as *mut ::core::ffi::c_void as *mut ev_watcher)).priority =
        0 as ::core::ffi::c_int;
    (*i3_connection).cb =
        Some(got_data as unsafe extern "C" fn(*mut ev_loop, *mut ev_io, ::core::ffi::c_int) -> ())
            as Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_io, ::core::ffi::c_int) -> ()>;
    memmove(
        &raw mut (*(i3_connection as *mut ev_watcher)).cb as *mut ::core::ffi::c_void,
        &raw mut (*i3_connection).cb as *const ::core::ffi::c_void,
        ::core::mem::size_of::<
            Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_io, ::core::ffi::c_int) -> ()>,
        >() as size_t,
    );
    (*i3_connection).fd = sockfd;
    (*i3_connection).events = EV_READ as ::core::ffi::c_int | EV__IOFDSET as ::core::ffi::c_int;
    ev_io_start(main_loop as *mut ev_loop, i3_connection);
}
#[no_mangle]
pub unsafe extern "C" fn destroy_connection() {
    close((*i3_connection).fd);
    ev_io_stop(main_loop as *mut ev_loop, i3_connection);
}
#[no_mangle]
pub unsafe extern "C" fn subscribe_events() {
    if i3_provides_workspaces() {
        i3_send_msg(
            I3_IPC_MESSAGE_TYPE_SUBSCRIBE as uint32_t,
            b"[ \"workspace\", \"output\", \"mode\", \"barconfig_update\" ]\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    } else {
        i3_send_msg(
            I3_IPC_MESSAGE_TYPE_SUBSCRIBE as uint32_t,
            b"[ \"output\", \"mode\", \"barconfig_update\" ]\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    };
}
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const EXIT_SUCCESS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
