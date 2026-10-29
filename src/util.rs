/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * util.c: Utility functions, which can be useful everywhere within i3 (see
 *         also libi3).
 *
 */
// Mechanical file-by-file Rust translation of src/util.c.
// Generated with C2Rust 0.22.1; C ABI and original ownership retained.
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type _PangoFontDescription;
    pub type _cairo;
    pub type _cairo_surface;
    pub type _i3String;
    pub type pcre2_real_code_8;
    pub type ev_loop;
    pub type yajl_gen_t;
    fn warn(__format: *const ::core::ffi::c_char, ...);
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn fread(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    fn perror(__s: *const ::core::ffi::c_char);
    fn fileno(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fstat(__fd: ::core::ffi::c_int, __buf: *mut stat) -> ::core::ffi::c_int;
    fn kill(__pid: __pid_t, __sig: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn execvp(
        __file: *const ::core::ffi::c_char,
        __argv: *const *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn _exit(__status: ::core::ffi::c_int) -> !;
    fn fork() -> __pid_t;
    fn readlink(
        __path: *const ::core::ffi::c_char,
        __buf: *mut ::core::ffi::c_char,
        __len: size_t,
    ) -> ssize_t;
    fn g_markup_escape_text(text: *const gchar, length: gssize) -> *mut gchar;
    fn verboselog(fmt: *mut ::core::ffi::c_char, ...);
    fn errorlog(fmt: *mut ::core::ffi::c_char, ...);
    fn debuglog(fmt: *mut ::core::ffi::c_char, ...);
    fn smalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn scalloc(num: size_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn sstrdup(str: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn sasprintf(
        strp: *mut *mut ::core::ffi::c_char,
        fmt: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn writeall(fd: ::core::ffi::c_int, buf: *const ::core::ffi::c_void, count: size_t) -> ssize_t;
    fn font_is_pango() -> bool;
    fn get_process_filename(prefix: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn resolve_tilde(path: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn mkdirp(path: *const ::core::ffi::c_char, mode: mode_t) -> ::core::ffi::c_int;
    fn ev_child_start(loop_0: *mut ev_loop, w: *mut ev_child);
    fn ev_child_stop(loop_0: *mut ev_loop, w: *mut ev_child);
    fn yajl_gen_alloc(allocFuncs: *const yajl_alloc_funcs) -> yajl_gen;
    fn yajl_gen_free(handle: yajl_gen);
    fn yajl_gen_get_buf(
        hand: yajl_gen,
        buf: *mut *const ::core::ffi::c_uchar,
        len: *mut size_t,
    ) -> yajl_gen_status;
    static mut croot: *mut Con;
    static mut start_argv: *mut *mut ::core::ffi::c_char;
    static mut main_loop: *mut ev_loop;
    static mut config: Config;
    fn ipc_shutdown(reason: shutdown_reason_t, exempt_fd: ::core::ffi::c_int);
    fn dump_node(gen: yajl_gen, con: *mut Con, inplace_restart: bool);
    fn get_debug_logging() -> bool;
    fn restore_geometry();
    static mut command_error_nagbar_pid: pid_t;
    static mut config_error_nagbar_pid: pid_t;
    fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    fn dirname(__path: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn setlocale(
        __category: ::core::ffi::c_int,
        __locale: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn waitpid(
        __pid: __pid_t,
        __stat_loc: *mut ::core::ffi::c_int,
        __options: ::core::ffi::c_int,
    ) -> __pid_t;
    fn memmove(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    fn strcasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    fn strtoll(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_longlong;
    fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type __uint64_t = u64;
pub type __dev_t = ::core::ffi::c_ulong;
pub type __uid_t = ::core::ffi::c_uint;
pub type __gid_t = ::core::ffi::c_uint;
pub type __ino_t = ::core::ffi::c_ulong;
pub type __mode_t = ::core::ffi::c_uint;
pub type __nlink_t = ::core::ffi::c_ulong;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __pid_t = ::core::ffi::c_int;
pub type __time_t = ::core::ffi::c_long;
pub type __suseconds_t = ::core::ffi::c_long;
pub type __blksize_t = ::core::ffi::c_long;
pub type __blkcnt_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type mode_t = __mode_t;
pub type pid_t = __pid_t;
pub type ssize_t = isize;
pub type time_t = __time_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
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
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: ::core::ffi::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
pub type xcb_window_t = uint32_t;
pub type xcb_font_t = uint32_t;
pub type xcb_gcontext_t = uint32_t;
pub type xcb_colormap_t = uint32_t;
pub type xcb_atom_t = uint32_t;
pub type xcb_drawable_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_charinfo_t {
    pub left_side_bearing: int16_t,
    pub right_side_bearing: int16_t,
    pub character_width: int16_t,
    pub ascent: int16_t,
    pub descent: int16_t,
    pub attributes: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_query_font_reply_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub min_bounds: xcb_charinfo_t,
    pub pad1: [uint8_t; 4],
    pub max_bounds: xcb_charinfo_t,
    pub pad2: [uint8_t; 4],
    pub min_char_or_byte2: uint16_t,
    pub max_char_or_byte2: uint16_t,
    pub default_char: uint16_t,
    pub properties_len: uint16_t,
    pub draw_direction: uint8_t,
    pub min_byte1: uint8_t,
    pub max_byte1: uint8_t,
    pub all_chars_exist: uint8_t,
    pub font_ascent: int16_t,
    pub font_descent: int16_t,
    pub char_infos_len: uint32_t,
}
pub type gssize = ::core::ffi::c_long;
pub type gchar = ::core::ffi::c_char;
pub type PangoFontDescription = _PangoFontDescription;
pub type cairo_t = _cairo;
pub type cairo_surface_t = _cairo_surface;
pub type i3String = _i3String;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Font {
    pub type_0: C2RustUnnamed_1,
    pub height: ::core::ffi::c_int,
    pub pattern: *mut ::core::ffi::c_char,
    pub specific: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub xcb: C2RustUnnamed_0,
    pub pango_desc: *mut PangoFontDescription,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub id: xcb_font_t,
    pub info: *mut xcb_query_font_reply_t,
    pub table: *mut xcb_charinfo_t,
}
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const FONT_TYPE_PANGO: C2RustUnnamed_1 = 2;
pub const FONT_TYPE_XCB: C2RustUnnamed_1 = 1;
pub const FONT_TYPE_NONE: C2RustUnnamed_1 = 0;
pub type i3Font = Font;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct color_t {
    pub red: ::core::ffi::c_double,
    pub green: ::core::ffi::c_double,
    pub blue: ::core::ffi::c_double,
    pub alpha: ::core::ffi::c_double,
    pub colorpixel: uint32_t,
}
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
pub type pcre2_code_8 = pcre2_real_code_8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Rect {
    pub x: uint32_t,
    pub y: uint32_t,
    pub width: uint32_t,
    pub height: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Con {
    pub mapped: bool,
    pub urgent: bool,
    pub ignore_unmap: uint8_t,
    pub frame: surface_t,
    pub frame_buffer: surface_t,
    pub pixmap_recreated: bool,
    pub type_0: C2RustUnnamed_18,
    pub num: ::core::ffi::c_int,
    pub gaps: gaps_t,
    pub parent: *mut Con,
    pub rect: Rect,
    pub window_rect: Rect,
    pub deco_rect: Rect,
    pub geometry: Rect,
    pub name: *mut ::core::ffi::c_char,
    pub title_format: *mut ::core::ffi::c_char,
    pub window_icon_padding: ::core::ffi::c_int,
    pub sticky_group: *mut ::core::ffi::c_char,
    pub marks_head: marks_head,
    pub mark_changed: bool,
    pub percent: ::core::ffi::c_double,
    pub border_width: ::core::ffi::c_int,
    pub current_border_width: ::core::ffi::c_int,
    pub window: *mut Window,
    pub urgency_timer: *mut ev_timer,
    pub deco_render_params: *mut deco_render_params,
    pub floating_head: floating_head,
    pub nodes_head: nodes_head,
    pub focus_head: focus_head,
    pub swallow_head: swallow_head,
    pub fullscreen_mode: fullscreen_mode_t,
    pub sticky: bool,
    pub layout: layout_t,
    pub last_split_layout: layout_t,
    pub workspace_layout: layout_t,
    pub border_style: border_style_t,
    pub max_user_border_style: border_style_t,
    pub floating: C2RustUnnamed_7,
    pub nodes: C2RustUnnamed_6,
    pub focused: C2RustUnnamed_5,
    pub all_cons: C2RustUnnamed_4,
    pub floating_windows: C2RustUnnamed_3,
    pub on_remove_child: Option<unsafe extern "C" fn(*mut Con) -> ()>,
    pub scratchpad_state: C2RustUnnamed_2,
    pub old_id: ::core::ffi::c_int,
    pub depth: uint16_t,
    pub colormap: xcb_colormap_t,
}
pub type C2RustUnnamed_2 = ::core::ffi::c_uint;
pub const SCRATCHPAD_CHANGED: C2RustUnnamed_2 = 2;
pub const SCRATCHPAD_FRESH: C2RustUnnamed_2 = 1;
pub const SCRATCHPAD_NONE: C2RustUnnamed_2 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub tqe_next: *mut Con,
    pub tqe_prev: *mut *mut Con,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub tqe_next: *mut Con,
    pub tqe_prev: *mut *mut Con,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub tqe_next: *mut Con,
    pub tqe_prev: *mut *mut Con,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub tqe_next: *mut Con,
    pub tqe_prev: *mut *mut Con,
}
pub type C2RustUnnamed_7 = ::core::ffi::c_uint;
pub const FLOATING_USER_ON: C2RustUnnamed_7 = 3;
pub const FLOATING_AUTO_ON: C2RustUnnamed_7 = 2;
pub const FLOATING_USER_OFF: C2RustUnnamed_7 = 1;
pub const FLOATING_AUTO_OFF: C2RustUnnamed_7 = 0;
pub type border_style_t = ::core::ffi::c_uint;
pub const BS_NORMAL: border_style_t = 2;
pub const BS_PIXEL: border_style_t = 1;
pub const BS_NONE: border_style_t = 0;
pub type layout_t = ::core::ffi::c_uint;
pub const L_SPLITH: layout_t = 6;
pub const L_SPLITV: layout_t = 5;
pub const L_OUTPUT: layout_t = 4;
pub const L_DOCKAREA: layout_t = 3;
pub const L_TABBED: layout_t = 2;
pub const L_STACKED: layout_t = 1;
pub const L_DEFAULT: layout_t = 0;
pub type fullscreen_mode_t = ::core::ffi::c_uint;
pub const CF_GLOBAL: fullscreen_mode_t = 2;
pub const CF_OUTPUT: fullscreen_mode_t = 1;
pub const CF_NONE: fullscreen_mode_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct swallow_head {
    pub tqh_first: *mut Match,
    pub tqh_last: *mut *mut Match,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Match {
    pub error: *mut ::core::ffi::c_char,
    pub title: *mut regex,
    pub application: *mut regex,
    pub class: *mut regex,
    pub instance: *mut regex,
    pub mark: *mut regex,
    pub window_role: *mut regex,
    pub workspace: *mut regex,
    pub machine: *mut regex,
    pub window_type: xcb_atom_t,
    pub urgent: C2RustUnnamed_12,
    pub dock: C2RustUnnamed_11,
    pub id: xcb_window_t,
    pub window_mode: C2RustUnnamed_10,
    pub con_id: *mut Con,
    pub match_all_windows: bool,
    pub insert_where: C2RustUnnamed_9,
    pub matches: C2RustUnnamed_8,
    pub restart_mode: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub tqe_next: *mut Match,
    pub tqe_prev: *mut *mut Match,
}
pub type C2RustUnnamed_9 = ::core::ffi::c_uint;
pub const M_BELOW: C2RustUnnamed_9 = 2;
pub const M_ASSIGN_WS: C2RustUnnamed_9 = 1;
pub const M_HERE: C2RustUnnamed_9 = 0;
pub type C2RustUnnamed_10 = ::core::ffi::c_uint;
pub const WM_FLOATING: C2RustUnnamed_10 = 6;
pub const WM_FLOATING_USER: C2RustUnnamed_10 = 5;
pub const WM_FLOATING_AUTO: C2RustUnnamed_10 = 4;
pub const WM_TILING: C2RustUnnamed_10 = 3;
pub const WM_TILING_USER: C2RustUnnamed_10 = 2;
pub const WM_TILING_AUTO: C2RustUnnamed_10 = 1;
pub const WM_ANY: C2RustUnnamed_10 = 0;
pub type C2RustUnnamed_11 = ::core::ffi::c_int;
pub const M_DOCK_BOTTOM: C2RustUnnamed_11 = 3;
pub const M_DOCK_TOP: C2RustUnnamed_11 = 2;
pub const M_DOCK_ANY: C2RustUnnamed_11 = 1;
pub const M_NODOCK: C2RustUnnamed_11 = 0;
pub const M_DONTCHECK: C2RustUnnamed_11 = -1;
pub type C2RustUnnamed_12 = ::core::ffi::c_int;
pub const U_OLDEST: C2RustUnnamed_12 = 1;
pub const U_LATEST: C2RustUnnamed_12 = 0;
pub const U_DONTCHECK: C2RustUnnamed_12 = -1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct regex {
    pub pattern: *mut ::core::ffi::c_char,
    pub regex: *mut pcre2_code_8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct focus_head {
    pub tqh_first: *mut Con,
    pub tqh_last: *mut *mut Con,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nodes_head {
    pub tqh_first: *mut Con,
    pub tqh_last: *mut *mut Con,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct floating_head {
    pub tqh_first: *mut Con,
    pub tqh_last: *mut *mut Con,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct deco_render_params {
    pub color: *mut Colortriple,
    pub border_style: ::core::ffi::c_int,
    pub con_rect: width_height,
    pub con_window_rect: width_height,
    pub con_deco_rect: Rect,
    pub background: color_t,
    pub parent_layout: layout_t,
    pub con_is_leaf: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct width_height {
    pub w: uint32_t,
    pub h: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Colortriple {
    pub border: color_t,
    pub background: color_t,
    pub text: color_t,
    pub indicator: color_t,
    pub child_border: color_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ev_timer {
    pub active: ::core::ffi::c_int,
    pub pending: ::core::ffi::c_int,
    pub priority: ::core::ffi::c_int,
    pub data: *mut ::core::ffi::c_void,
    pub cb: Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_timer, ::core::ffi::c_int) -> ()>,
    pub at: ev_tstamp,
    pub repeat: ev_tstamp,
}
pub type ev_tstamp = ::core::ffi::c_double;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Window {
    pub id: xcb_window_t,
    pub leader: xcb_window_t,
    pub transient_for: xcb_window_t,
    pub nr_assignments: uint32_t,
    pub ran_assignments: *mut *mut Assignment,
    pub class_class: *mut ::core::ffi::c_char,
    pub class_instance: *mut ::core::ffi::c_char,
    pub name: *mut i3String,
    pub role: *mut ::core::ffi::c_char,
    pub machine: *mut ::core::ffi::c_char,
    pub name_x_changed: bool,
    pub uses_net_wm_name: bool,
    pub needs_take_focus: bool,
    pub doesnt_accept_focus: bool,
    pub window_type: xcb_atom_t,
    pub wm_desktop: uint32_t,
    pub dock: C2RustUnnamed_13,
    pub urgent: timeval,
    pub reserved: reservedpx,
    pub depth: uint16_t,
    pub base_width: ::core::ffi::c_int,
    pub base_height: ::core::ffi::c_int,
    pub width_increment: ::core::ffi::c_int,
    pub height_increment: ::core::ffi::c_int,
    pub min_width: ::core::ffi::c_int,
    pub min_height: ::core::ffi::c_int,
    pub max_width: ::core::ffi::c_int,
    pub max_height: ::core::ffi::c_int,
    pub min_aspect_ratio: ::core::ffi::c_double,
    pub max_aspect_ratio: ::core::ffi::c_double,
    pub icon: *mut cairo_surface_t,
    pub shaped: bool,
    pub input_shaped: bool,
    pub managed_since: time_t,
    pub swallowed: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct reservedpx {
    pub left: uint32_t,
    pub right: uint32_t,
    pub top: uint32_t,
    pub bottom: uint32_t,
}
pub type C2RustUnnamed_13 = ::core::ffi::c_uint;
pub const W_DOCK_BOTTOM: C2RustUnnamed_13 = 2;
pub const W_DOCK_TOP: C2RustUnnamed_13 = 1;
pub const W_NODOCK: C2RustUnnamed_13 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Assignment {
    pub type_0: C2RustUnnamed_16,
    pub match_0: Match,
    pub dest: C2RustUnnamed_15,
    pub assignments: C2RustUnnamed_14,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub tqe_next: *mut Assignment,
    pub tqe_prev: *mut *mut Assignment,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_15 {
    pub command: *mut ::core::ffi::c_char,
    pub workspace: *mut ::core::ffi::c_char,
    pub output: *mut ::core::ffi::c_char,
}
pub type C2RustUnnamed_16 = ::core::ffi::c_uint;
pub const A_TO_OUTPUT: C2RustUnnamed_16 = 16;
pub const A_TO_WORKSPACE_NUMBER: C2RustUnnamed_16 = 8;
pub const A_NO_FOCUS: C2RustUnnamed_16 = 4;
pub const A_TO_WORKSPACE: C2RustUnnamed_16 = 2;
pub const A_COMMAND: C2RustUnnamed_16 = 1;
pub const A_ANY: C2RustUnnamed_16 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct marks_head {
    pub tqh_first: *mut mark_t,
    pub tqh_last: *mut *mut mark_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mark_t {
    pub name: *mut ::core::ffi::c_char,
    pub marks: C2RustUnnamed_17,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_17 {
    pub tqe_next: *mut mark_t,
    pub tqe_prev: *mut *mut mark_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gaps_t {
    pub inner: ::core::ffi::c_int,
    pub top: ::core::ffi::c_int,
    pub right: ::core::ffi::c_int,
    pub bottom: ::core::ffi::c_int,
    pub left: ::core::ffi::c_int,
}
pub type C2RustUnnamed_18 = ::core::ffi::c_uint;
pub const CT_DOCKAREA: C2RustUnnamed_18 = 5;
pub const CT_WORKSPACE: C2RustUnnamed_18 = 4;
pub const CT_FLOATING_CON: C2RustUnnamed_18 = 3;
pub const CT_CON: C2RustUnnamed_18 = 2;
pub const CT_OUTPUT: C2RustUnnamed_18 = 1;
pub const CT_ROOT: C2RustUnnamed_18 = 0;
pub type direction_t = ::core::ffi::c_uint;
pub const D_DOWN: direction_t = 3;
pub const D_UP: direction_t = 2;
pub const D_RIGHT: direction_t = 1;
pub const D_LEFT: direction_t = 0;
pub type orientation_t = ::core::ffi::c_uint;
pub const VERT: orientation_t = 2;
pub const HORIZ: orientation_t = 1;
pub const NO_ORIENTATION: orientation_t = 0;
pub type position_t = ::core::ffi::c_uint;
pub const AFTER: position_t = 1;
pub const BEFORE: position_t = 0;
pub type smart_gaps_t = ::core::ffi::c_uint;
pub const SMART_GAPS_INVERSE_OUTER: smart_gaps_t = 2;
pub const SMART_GAPS_ON: smart_gaps_t = 1;
pub const SMART_GAPS_OFF: smart_gaps_t = 0;
pub type hide_edge_borders_mode_t = ::core::ffi::c_uint;
pub const HEBM_SMART_NO_GAPS: hide_edge_borders_mode_t = 64;
pub const HEBM_SMART: hide_edge_borders_mode_t = 32;
pub const HEBM_BOTH: hide_edge_borders_mode_t = 23;
pub const HEBM_HORIZONTAL: hide_edge_borders_mode_t = 20;
pub const HEBM_VERTICAL: hide_edge_borders_mode_t = 3;
pub const HEBM_NONE: hide_edge_borders_mode_t = 0;
pub type warping_t = ::core::ffi::c_uint;
pub const POINTER_WARPING_NONE: warping_t = 1;
pub const POINTER_WARPING_OUTPUT: warping_t = 0;
pub type focus_wrapping_t = ::core::ffi::c_uint;
pub const FOCUS_WRAPPING_WORKSPACE: focus_wrapping_t = 3;
pub const FOCUS_WRAPPING_FORCE: focus_wrapping_t = 2;
pub const FOCUS_WRAPPING_ON: focus_wrapping_t = 1;
pub const FOCUS_WRAPPING_OFF: focus_wrapping_t = 0;
pub const _ISdigit: C2RustUnnamed_22 = 2048;
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
pub struct Config {
    pub terminal: *const ::core::ffi::c_char,
    pub font: i3Font,
    pub ipc_socket_path: *mut ::core::ffi::c_char,
    pub restart_state_path: *mut ::core::ffi::c_char,
    pub default_layout: layout_t,
    pub container_stack_limit: ::core::ffi::c_int,
    pub container_stack_limit_value: ::core::ffi::c_int,
    pub default_border_width: ::core::ffi::c_int,
    pub default_floating_border_width: ::core::ffi::c_int,
    pub default_orientation: ::core::ffi::c_int,
    pub disable_focus_follows_mouse: bool,
    pub mouse_warping: warping_t,
    pub hide_edge_borders: hide_edge_borders_mode_t,
    pub disable_workspace_bar: bool,
    pub focus_wrapping: focus_wrapping_t,
    pub force_xinerama: bool,
    pub disable_randr15: bool,
    pub fake_outputs: *mut ::core::ffi::c_char,
    pub workspace_auto_back_and_forth: bool,
    pub workspace_urgency_timer: ::core::ffi::c_float,
    pub focus_on_window_activation: C2RustUnnamed_21,
    pub show_marks: bool,
    pub title_align: C2RustUnnamed_20,
    pub default_border: border_style_t,
    pub default_floating_border: border_style_t,
    pub floating_modifier: uint32_t,
    pub swap_modifier: uint32_t,
    pub floating_maximum_width: int32_t,
    pub floating_maximum_height: int32_t,
    pub floating_minimum_width: int32_t,
    pub floating_minimum_height: int32_t,
    pub client: config_client,
    pub bar: config_bar,
    pub popup_during_fullscreen: C2RustUnnamed_19,
    pub number_barconfigs: ::core::ffi::c_int,
    pub tiling_drag: tiling_drag_t,
    pub gaps: gaps_t,
    pub smart_gaps: smart_gaps_t,
}
pub type tiling_drag_t = ::core::ffi::c_uint;
pub const TILING_DRAG_MODIFIER_OR_TITLEBAR: tiling_drag_t = 3;
pub const TILING_DRAG_TITLEBAR: tiling_drag_t = 2;
pub const TILING_DRAG_MODIFIER: tiling_drag_t = 1;
pub const TILING_DRAG_OFF: tiling_drag_t = 0;
pub type C2RustUnnamed_19 = ::core::ffi::c_uint;
pub const PDF_ALL: C2RustUnnamed_19 = 3;
pub const PDF_IGNORE: C2RustUnnamed_19 = 2;
pub const PDF_LEAVE_FULLSCREEN: C2RustUnnamed_19 = 1;
pub const PDF_SMART: C2RustUnnamed_19 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct config_bar {
    pub focused: Colortriple,
    pub unfocused: Colortriple,
    pub urgent: Colortriple,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct config_client {
    pub background: color_t,
    pub focused: Colortriple,
    pub focused_inactive: Colortriple,
    pub focused_tab_title: Colortriple,
    pub unfocused: Colortriple,
    pub urgent: Colortriple,
    pub placeholder: Colortriple,
    pub got_focused_tab_title: bool,
}
pub type C2RustUnnamed_20 = ::core::ffi::c_uint;
pub const ALIGN_RIGHT: C2RustUnnamed_20 = 2;
pub const ALIGN_CENTER: C2RustUnnamed_20 = 1;
pub const ALIGN_LEFT: C2RustUnnamed_20 = 0;
pub type C2RustUnnamed_21 = ::core::ffi::c_uint;
pub const FOWA_NONE: C2RustUnnamed_21 = 3;
pub const FOWA_FOCUS: C2RustUnnamed_21 = 2;
pub const FOWA_URGENT: C2RustUnnamed_21 = 1;
pub const FOWA_SMART: C2RustUnnamed_21 = 0;
pub type yajl_gen_status = ::core::ffi::c_uint;
pub const yajl_gen_invalid_string: yajl_gen_status = 7;
pub const yajl_gen_no_buf: yajl_gen_status = 6;
pub const yajl_gen_invalid_number: yajl_gen_status = 5;
pub const yajl_gen_generation_complete: yajl_gen_status = 4;
pub const yajl_gen_in_error_state: yajl_gen_status = 3;
pub const yajl_max_depth_exceeded: yajl_gen_status = 2;
pub const yajl_gen_keys_must_be_strings: yajl_gen_status = 1;
pub const yajl_gen_status_ok: yajl_gen_status = 0;
pub type shutdown_reason_t = ::core::ffi::c_uint;
pub const SHUTDOWN_REASON_EXIT: shutdown_reason_t = 1;
pub const SHUTDOWN_REASON_RESTART: shutdown_reason_t = 0;
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
pub struct ev_watcher {
    pub active: ::core::ffi::c_int,
    pub pending: ::core::ffi::c_int,
    pub priority: ::core::ffi::c_int,
    pub data: *mut ::core::ffi::c_void,
    pub cb: Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_watcher, ::core::ffi::c_int) -> ()>,
}
pub type C2RustUnnamed_22 = ::core::ffi::c_uint;
pub const _ISalnum: C2RustUnnamed_22 = 8;
pub const _ISpunct: C2RustUnnamed_22 = 4;
pub const _IScntrl: C2RustUnnamed_22 = 2;
pub const _ISblank: C2RustUnnamed_22 = 1;
pub const _ISgraph: C2RustUnnamed_22 = 32768;
pub const _ISprint: C2RustUnnamed_22 = 16384;
pub const _ISspace: C2RustUnnamed_22 = 8192;
pub const _ISxdigit: C2RustUnnamed_22 = 4096;
pub const _ISalpha: C2RustUnnamed_22 = 1024;
pub const _ISlower: C2RustUnnamed_22 = 512;
pub const _ISupper: C2RustUnnamed_22 = 256;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const __S_IREAD: ::core::ffi::c_int = 0o400 as ::core::ffi::c_int;
pub const __S_IWRITE: ::core::ffi::c_int = 0o200 as ::core::ffi::c_int;
pub const __S_IEXEC: ::core::ffi::c_int = 0o100 as ::core::ffi::c_int;
pub const INT32_MAX: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const BUFSIZ: ::core::ffi::c_int = 8192 as ::core::ffi::c_int;
pub const SIGTERM: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const __LC_NUMERIC: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const DEFAULT_DIR_MODE: ::core::ffi::c_int = S_IRWXU | S_IRGRP | S_IXGRP | S_IROTH | S_IXOTH;
pub const O_WRONLY: ::core::ffi::c_int = 0o1 as ::core::ffi::c_int;
pub const O_CREAT: ::core::ffi::c_int = 0o100 as ::core::ffi::c_int;
pub const O_TRUNC: ::core::ffi::c_int = 0o1000 as ::core::ffi::c_int;
pub const S_IRUSR: ::core::ffi::c_int = __S_IREAD;
pub const S_IWUSR: ::core::ffi::c_int = __S_IWRITE;
pub const S_IXUSR: ::core::ffi::c_int = __S_IEXEC;
pub const S_IRWXU: ::core::ffi::c_int = __S_IREAD | __S_IWRITE | __S_IEXEC;
pub const S_IRGRP: ::core::ffi::c_int = S_IRUSR >> 3 as ::core::ffi::c_int;
pub const S_IXGRP: ::core::ffi::c_int = S_IXUSR >> 3 as ::core::ffi::c_int;
pub const S_IROTH: ::core::ffi::c_int = S_IRGRP >> 3 as ::core::ffi::c_int;
pub const S_IXOTH: ::core::ffi::c_int = S_IXGRP >> 3 as ::core::ffi::c_int;
pub const LC_NUMERIC: ::core::ffi::c_int = __LC_NUMERIC;
#[no_mangle]
pub unsafe extern "C" fn min(a: ::core::ffi::c_int, b: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return if a < b { a } else { b };
}
#[no_mangle]
pub unsafe extern "C" fn max(a: ::core::ffi::c_int, b: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return if a > b { a } else { b };
}
#[no_mangle]
pub unsafe extern "C" fn rect_contains(rect: Rect, x: uint32_t, y: uint32_t) -> bool {
    return x >= rect.x
        && x <= rect.x.wrapping_add(rect.width)
        && y >= rect.y
        && y <= rect.y.wrapping_add(rect.height);
}
#[no_mangle]
pub unsafe extern "C" fn rect_add(a: Rect, b: Rect) -> Rect {
    return Rect {
        x: a.x.wrapping_add(b.x),
        y: a.y.wrapping_add(b.y),
        width: a.width.wrapping_add(b.width),
        height: a.height.wrapping_add(b.height),
    };
}
#[no_mangle]
pub unsafe extern "C" fn rect_sub(a: Rect, b: Rect) -> Rect {
    return Rect {
        x: a.x.wrapping_sub(b.x),
        y: a.y.wrapping_sub(b.y),
        width: a.width.wrapping_sub(b.width),
        height: a.height.wrapping_sub(b.height),
    };
}
#[no_mangle]
pub unsafe extern "C" fn rect_sanitize_dimensions(mut rect: Rect) -> Rect {
    rect.width = if rect.width as int32_t <= 0 as int32_t {
        1 as uint32_t
    } else {
        rect.width
    };
    rect.height = if rect.height as int32_t <= 0 as int32_t {
        1 as uint32_t
    } else {
        rect.height
    };
    return rect;
}
#[no_mangle]
pub unsafe extern "C" fn rect_equals(a: Rect, b: Rect) -> bool {
    return a.x == b.x && a.y == b.y && a.width == b.width && a.height == b.height;
}
#[no_mangle]
pub unsafe extern "C" fn name_is_digits(mut name: *const ::core::ffi::c_char) -> bool {
    let mut i: size_t = 0 as size_t;
    while i < strlen(name) {
        if *(*__ctype_b_loc())
            .offset(*name.offset(i as isize) as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & _ISdigit as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
            == 0
        {
            return false_0 != 0;
        }
        i = i.wrapping_add(1);
    }
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn layout_from_name(
    mut layout_str: *const ::core::ffi::c_char,
    mut out: *mut layout_t,
) -> bool {
    if strcmp(
        layout_str,
        b"default\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        *out = L_DEFAULT;
        return true_0 != 0;
    }
    if strcasecmp(
        layout_str,
        b"stacked\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
        || strcasecmp(
            layout_str,
            b"stacking\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
    {
        *out = L_STACKED;
        return true_0 != 0;
    }
    if strcasecmp(
        layout_str,
        b"tabbed\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        *out = L_TABBED;
        return true_0 != 0;
    }
    if strcasecmp(
        layout_str,
        b"splitv\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        *out = L_SPLITV;
        return true_0 != 0;
    }
    if strcasecmp(
        layout_str,
        b"splith\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        *out = L_SPLITH;
        return true_0 != 0;
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn ws_name_to_number(
    mut name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut endptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    *__errno_location() = 0 as ::core::ffi::c_int;
    let mut parsed_num: ::core::ffi::c_longlong =
        strtoll(name, &raw mut endptr, 10 as ::core::ffi::c_int);
    if *__errno_location() != 0 as ::core::ffi::c_int
        || parsed_num > INT32_MAX as ::core::ffi::c_longlong
        || parsed_num < 0 as ::core::ffi::c_longlong
        || endptr == name as *mut ::core::ffi::c_char
    {
        parsed_num = -(1 as ::core::ffi::c_int) as ::core::ffi::c_longlong;
    }
    return parsed_num as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn update_if_necessary(
    mut destination: *mut uint32_t,
    new_value: uint32_t,
) -> bool {
    let mut old_value: uint32_t = *destination;
    *destination = new_value;
    return *destination != old_value;
}
#[no_mangle]
pub unsafe extern "C" fn exec_i3_utility(
    mut name: *mut ::core::ffi::c_char,
    mut argv: *mut *mut ::core::ffi::c_char,
) {
    let mut binary: *mut ::core::ffi::c_char = name;
    let ref mut fresh0 = *argv.offset(0 as ::core::ffi::c_int as isize);
    *fresh0 = binary;
    execvp(binary, argv as *const *mut ::core::ffi::c_char);
    let mut pathbuf: *mut ::core::ffi::c_char =
        sstrdup(*start_argv.offset(0 as ::core::ffi::c_int as isize));
    let mut dir: *mut ::core::ffi::c_char = dirname(pathbuf);
    sasprintf(
        &raw mut binary,
        b"%s/%s\0" as *const u8 as *const ::core::ffi::c_char,
        dir,
        name,
    );
    let ref mut fresh1 = *argv.offset(0 as ::core::ffi::c_int as isize);
    *fresh1 = binary;
    execvp(binary, argv as *const *mut ::core::ffi::c_char);
    let mut buffer: [::core::ffi::c_char; 8192] = [0; 8192];
    if readlink(
        b"/proc/self/exe\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut buffer as *mut ::core::ffi::c_char,
        BUFSIZ as size_t,
    ) == -(1 as ::core::ffi::c_int) as ssize_t
    {
        warn(b"could not read /proc/self/exe\0" as *const u8 as *const ::core::ffi::c_char);
        _exit(EXIT_FAILURE);
    }
    dir = dirname(&raw mut buffer as *mut ::core::ffi::c_char);
    sasprintf(
        &raw mut binary,
        b"%s/%s\0" as *const u8 as *const ::core::ffi::c_char,
        dir,
        name,
    );
    let ref mut fresh2 = *argv.offset(0 as ::core::ffi::c_int as isize);
    *fresh2 = binary;
    execvp(binary, argv as *const *mut ::core::ffi::c_char);
    warn(
        b"Could not start %s\0" as *const u8 as *const ::core::ffi::c_char,
        name,
    );
    _exit(2 as ::core::ffi::c_int);
}
unsafe extern "C" fn add_argument(
    mut original: *mut *mut ::core::ffi::c_char,
    mut opt_char: *mut ::core::ffi::c_char,
    mut opt_arg: *mut ::core::ffi::c_char,
    mut opt_name: *mut ::core::ffi::c_char,
) -> *mut *mut ::core::ffi::c_char {
    let mut num_args: ::core::ffi::c_int = 0;
    num_args = 0 as ::core::ffi::c_int;
    while !(*original.offset(num_args as isize)).is_null() {
        num_args += 1;
    }
    let mut result: *mut *mut ::core::ffi::c_char = scalloc(
        (num_args + 3 as ::core::ffi::c_int) as size_t,
        ::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t,
    ) as *mut *mut ::core::ffi::c_char;
    let mut write_index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut skip_next: bool = false_0 != 0;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < num_args {
        if skip_next {
            skip_next = false_0 != 0;
        } else if strcmp(*original.offset(i as isize), opt_char) == 0
            || !opt_name.is_null() && strcmp(*original.offset(i as isize), opt_name) == 0
        {
            if !opt_arg.is_null() {
                skip_next = true_0 != 0;
            }
        } else {
            let fresh3 = write_index;
            write_index = write_index + 1;
            let ref mut fresh4 = *result.offset(fresh3 as isize);
            *fresh4 = *original.offset(i as isize);
        }
        i += 1;
    }
    let fresh5 = write_index;
    write_index = write_index + 1;
    let ref mut fresh6 = *result.offset(fresh5 as isize);
    *fresh6 = opt_char;
    let ref mut fresh7 = *result.offset(write_index as isize);
    *fresh7 = opt_arg;
    return result;
}
unsafe extern "C" fn store_restart_layout() -> *mut ::core::ffi::c_char {
    let mut prev_locale: *mut ::core::ffi::c_char = sstrdup(setlocale(
        LC_NUMERIC,
        ::core::ptr::null::<::core::ffi::c_char>(),
    ));
    setlocale(
        LC_NUMERIC,
        b"C\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut gen: yajl_gen = yajl_gen_alloc(::core::ptr::null::<yajl_alloc_funcs>());
    dump_node(gen, croot, true_0 != 0);
    setlocale(LC_NUMERIC, prev_locale);
    free(prev_locale as *mut ::core::ffi::c_void);
    let mut payload: *const ::core::ffi::c_uchar = ::core::ptr::null::<::core::ffi::c_uchar>();
    let mut length: size_t = 0;
    yajl_gen_get_buf(gen, &raw mut payload, &raw mut length);
    let mut filename: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if config.restart_state_path.is_null() {
        filename =
            get_process_filename(b"restart-state\0" as *const u8 as *const ::core::ffi::c_char);
        if filename.is_null() {
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
    } else {
        filename = resolve_tilde(config.restart_state_path);
    }
    let mut filenamecopy: *mut ::core::ffi::c_char = sstrdup(filename);
    let mut base: *mut ::core::ffi::c_char = dirname(filenamecopy);
    debuglog(
        b"%s:%s:%d - Creating \"%s\" for storing the restart layout\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/util.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"store_restart_layout\0" as *const u8 as *const ::core::ffi::c_char,
        253 as ::core::ffi::c_int,
        base,
    );
    if mkdirp(base, DEFAULT_DIR_MODE as mode_t) != 0 as ::core::ffi::c_int {
        errorlog(
            b"ERROR: Could not create \"%s\" for storing the restart layout, layout will be lost.\n\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            base,
        );
    }
    free(filenamecopy as *mut ::core::ffi::c_void);
    let mut fd: ::core::ffi::c_int =
        open(filename, O_WRONLY | O_CREAT | O_TRUNC, S_IRUSR | S_IWUSR);
    if fd == -(1 as ::core::ffi::c_int) {
        perror(b"open()\0" as *const u8 as *const ::core::ffi::c_char);
        free(filename as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if writeall(fd, payload as *const ::core::ffi::c_void, length)
        == -(1 as ::core::ffi::c_int) as ssize_t
    {
        errorlog(
            b"ERROR: Could not write restart layout to \"%s\", layout will be lost: %s\n\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            filename,
            strerror(*__errno_location()),
        );
        free(filename as *mut ::core::ffi::c_void);
        close(fd);
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    close(fd);
    if length > 0 as size_t {
        debuglog(
            b"%s:%s:%d - layout: %.*s\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b"../src/util.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"store_restart_layout\0" as *const u8 as *const ::core::ffi::c_char,
            276 as ::core::ffi::c_int,
            length as ::core::ffi::c_int,
            payload,
        );
    }
    yajl_gen_free(gen);
    return filename;
}
#[no_mangle]
pub unsafe extern "C" fn i3_restart(mut forget_layout: bool) {
    let mut restart_filename: *mut ::core::ffi::c_char = if forget_layout as ::core::ffi::c_int != 0
    {
        ::core::ptr::null_mut::<::core::ffi::c_char>()
    } else {
        store_restart_layout()
    };
    kill_nagbar(config_error_nagbar_pid, true_0 != 0);
    kill_nagbar(command_error_nagbar_pid, true_0 != 0);
    restore_geometry();
    ipc_shutdown(SHUTDOWN_REASON_RESTART, -(1 as ::core::ffi::c_int));
    verboselog(
        b"restarting \"%s\"...\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        *start_argv.offset(0 as ::core::ffi::c_int as isize),
    );
    start_argv = add_argument(
        start_argv,
        b"-a\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
    );
    if get_debug_logging() {
        start_argv = add_argument(
            start_argv,
            b"-d\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"all\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
        );
    }
    if !restart_filename.is_null() {
        start_argv = add_argument(
            start_argv,
            b"--restart\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            restart_filename,
            b"-r\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    execvp(
        *start_argv.offset(0 as ::core::ffi::c_int as isize),
        start_argv as *const *mut ::core::ffi::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn pango_escape_markup(
    mut input: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    if !font_is_pango() {
        return input;
    }
    let mut escaped: *mut ::core::ffi::c_char =
        g_markup_escape_text(input, -(1 as ::core::ffi::c_int) as gssize)
            as *mut ::core::ffi::c_char;
    free(input as *mut ::core::ffi::c_void);
    input = ::core::ptr::null_mut::<::core::ffi::c_char>();
    return escaped;
}
unsafe extern "C" fn nagbar_exited(
    mut loop_0: *mut ev_loop,
    mut watcher: *mut ev_child,
    mut revents: ::core::ffi::c_int,
) {
    ev_child_stop(loop_0, watcher);
    let mut exitcode: ::core::ffi::c_int =
        ((*watcher).rstatus & 0xff00 as ::core::ffi::c_int) >> 8 as ::core::ffi::c_int;
    if !((*watcher).rstatus & 0x7f as ::core::ffi::c_int == 0 as ::core::ffi::c_int) {
        errorlog(
            b"ERROR: i3-nagbar (%d) did not exit normally. This is not an error if the config was reloaded while a nagbar was active.\n\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            (*watcher).pid,
        );
    } else if exitcode != 0 as ::core::ffi::c_int {
        errorlog(
            b"ERROR: i3-nagbar (%d) process exited with status %d\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            (*watcher).pid,
            exitcode,
        );
    } else {
        debuglog(
            b"%s:%s:%d - i3-nagbar (%d) process exited with status %d\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/util.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"nagbar_exited\0" as *const u8 as *const ::core::ffi::c_char,
            348 as ::core::ffi::c_int,
            (*watcher).pid,
            exitcode,
        );
    }
    let mut nagbar_pid: *mut pid_t = (*watcher).data as *mut pid_t;
    if *nagbar_pid == (*watcher).pid {
        *nagbar_pid = -(1 as ::core::ffi::c_int) as pid_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn start_nagbar(
    mut nagbar_pid: *mut pid_t,
    mut argv: *mut *mut ::core::ffi::c_char,
) {
    if *nagbar_pid != -(1 as ::core::ffi::c_int) {
        debuglog(
            b"%s:%s:%d - i3-nagbar already running (PID %d), not starting again.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/util.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"start_nagbar\0" as *const u8 as *const ::core::ffi::c_char,
            368 as ::core::ffi::c_int,
            *nagbar_pid,
        );
        return;
    }
    *nagbar_pid = fork() as pid_t;
    if *nagbar_pid == -(1 as ::core::ffi::c_int) {
        warn(b"Could not fork()\0" as *const u8 as *const ::core::ffi::c_char);
        return;
    }
    if *nagbar_pid == 0 as ::core::ffi::c_int {
        exec_i3_utility(
            b"i3-nagbar\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            argv,
        );
    }
    debuglog(
        b"%s:%s:%d - Starting i3-nagbar with PID %d\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/util.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"start_nagbar\0" as *const u8 as *const ::core::ffi::c_char,
        383 as ::core::ffi::c_int,
        *nagbar_pid,
    );
    let mut child: *mut ev_child =
        smalloc(::core::mem::size_of::<ev_child>() as size_t) as *mut ev_child;
    let ref mut fresh8 = (*(child as *mut ::core::ffi::c_void as *mut ev_watcher)).pending;
    *fresh8 = 0 as ::core::ffi::c_int;
    (*(child as *mut ::core::ffi::c_void as *mut ev_watcher)).active = *fresh8;
    (*(child as *mut ::core::ffi::c_void as *mut ev_watcher)).priority = 0 as ::core::ffi::c_int;
    (*child).cb = Some(
        nagbar_exited
            as unsafe extern "C" fn(*mut ev_loop, *mut ev_child, ::core::ffi::c_int) -> (),
    )
        as Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_child, ::core::ffi::c_int) -> ()>;
    memmove(
        &raw mut (*(child as *mut ev_watcher)).cb as *mut ::core::ffi::c_void,
        &raw mut (*child).cb as *const ::core::ffi::c_void,
        ::core::mem::size_of::<
            Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_child, ::core::ffi::c_int) -> ()>,
        >() as size_t,
    );
    (*child).pid = *nagbar_pid as ::core::ffi::c_int;
    (*child).flags = (0 as ::core::ffi::c_int != 0) as ::core::ffi::c_int;
    (*child).data = nagbar_pid as *mut ::core::ffi::c_void;
    ev_child_start(main_loop, child);
}
#[no_mangle]
pub unsafe extern "C" fn kill_nagbar(mut nagbar_pid: pid_t, mut wait_for_it: bool) {
    if nagbar_pid == -(1 as ::core::ffi::c_int) {
        return;
    }
    if kill(nagbar_pid as __pid_t, SIGTERM) == -(1 as ::core::ffi::c_int) {
        warn(b"kill(configerror_nagbar) failed\0" as *const u8 as *const ::core::ffi::c_char);
    }
    if !wait_for_it {
        return;
    }
    waitpid(
        nagbar_pid as __pid_t,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        0 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn parse_long(
    mut str: *const ::core::ffi::c_char,
    mut out: *mut ::core::ffi::c_long,
    mut base: ::core::ffi::c_int,
) -> bool {
    let mut end: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut result: ::core::ffi::c_long = strtol(str, &raw mut end, base);
    if result == LONG_MIN
        || result == LONG_MAX
        || result < 0 as ::core::ffi::c_long
        || !end.is_null() && *end as ::core::ffi::c_int != '\0' as i32
    {
        *out = result;
        return false_0 != 0;
    }
    *out = result;
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn slurp(
    mut path: *const ::core::ffi::c_char,
    mut buf: *mut *mut ::core::ffi::c_char,
) -> ssize_t {
    let mut f: *mut FILE = ::core::ptr::null_mut::<FILE>();
    f = fopen(path, b"r\0" as *const u8 as *const ::core::ffi::c_char) as *mut FILE;
    if f.is_null() {
        errorlog(
            b"ERROR: Cannot open file \"%s\": %s\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            path,
            strerror(*__errno_location()),
        );
        return -(1 as ::core::ffi::c_int) as ssize_t;
    }
    let mut stbuf: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    if fstat(fileno(f), &raw mut stbuf) != 0 as ::core::ffi::c_int {
        errorlog(
            b"ERROR: Cannot fstat() \"%s\": %s\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            path,
            strerror(*__errno_location()),
        );
        fclose(f);
        return -(1 as ::core::ffi::c_int) as ssize_t;
    }
    *buf =
        scalloc((stbuf.st_size + 1 as __off_t) as size_t, 1 as size_t) as *mut ::core::ffi::c_char;
    let mut n: size_t = fread(
        *buf as *mut ::core::ffi::c_void,
        1 as size_t,
        stbuf.st_size as size_t,
        f,
    ) as size_t;
    fclose(f);
    if n as ssize_t != stbuf.st_size as ssize_t {
        errorlog(
            b"ERROR: File \"%s\" could not be read entirely: got %zd, want %li\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            path,
            n,
            stbuf.st_size as int64_t,
        );
        free(*buf as *mut ::core::ffi::c_void);
        *buf = ::core::ptr::null_mut::<::core::ffi::c_char>();
        return -(1 as ::core::ffi::c_int) as ssize_t;
    }
    return n as ssize_t;
}
#[no_mangle]
pub unsafe extern "C" fn orientation_from_direction(mut direction: direction_t) -> orientation_t {
    return (if direction as ::core::ffi::c_uint
        == D_LEFT as ::core::ffi::c_int as ::core::ffi::c_uint
        || direction as ::core::ffi::c_uint == D_RIGHT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        HORIZ as ::core::ffi::c_int
    } else {
        VERT as ::core::ffi::c_int
    }) as orientation_t;
}
#[no_mangle]
pub unsafe extern "C" fn position_from_direction(mut direction: direction_t) -> position_t {
    return (if direction as ::core::ffi::c_uint
        == D_LEFT as ::core::ffi::c_int as ::core::ffi::c_uint
        || direction as ::core::ffi::c_uint == D_UP as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        BEFORE as ::core::ffi::c_int
    } else {
        AFTER as ::core::ffi::c_int
    }) as position_t;
}
#[no_mangle]
pub unsafe extern "C" fn direction_from_orientation_position(
    mut orientation: orientation_t,
    mut position: position_t,
) -> direction_t {
    if orientation as ::core::ffi::c_uint == HORIZ as ::core::ffi::c_int as ::core::ffi::c_uint {
        return (if position as ::core::ffi::c_uint
            == BEFORE as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            D_LEFT as ::core::ffi::c_int
        } else {
            D_RIGHT as ::core::ffi::c_int
        }) as direction_t;
    } else {
        return (if position as ::core::ffi::c_uint
            == BEFORE as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            D_UP as ::core::ffi::c_int
        } else {
            D_DOWN as ::core::ffi::c_int
        }) as direction_t;
    };
}
#[no_mangle]
pub unsafe extern "C" fn direction_to_string(
    mut direction: direction_t,
) -> *const ::core::ffi::c_char {
    match direction as ::core::ffi::c_uint {
        0 => return b"left\0" as *const u8 as *const ::core::ffi::c_char,
        1 => return b"right\0" as *const u8 as *const ::core::ffi::c_char,
        2 => return b"up\0" as *const u8 as *const ::core::ffi::c_char,
        3 => return b"down\0" as *const u8 as *const ::core::ffi::c_char,
        _ => {}
    }
    return b"invalid\0" as *const u8 as *const ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn position_to_string(
    mut position: position_t,
) -> *const ::core::ffi::c_char {
    match position as ::core::ffi::c_uint {
        0 => return b"before\0" as *const u8 as *const ::core::ffi::c_char,
        1 => return b"after\0" as *const u8 as *const ::core::ffi::c_char,
        _ => {}
    }
    return b"invalid\0" as *const u8 as *const ::core::ffi::c_char;
}
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const LONG_MAX: ::core::ffi::c_long = __LONG_MAX__;
pub const LONG_MIN: ::core::ffi::c_long = -__LONG_MAX__ - 1 as ::core::ffi::c_long;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
