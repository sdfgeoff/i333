/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3bar - an xcb-based status- and ws-bar for i3
 * © 2010 Axel Wagner and contributors (see also: LICENSE)
 *
 */
// Mechanical file-by-file Rust translation of i3bar/src/main.c.
// Generated with C2Rust 0.22.1; C ABI and original ownership retained.
use ::c2rust_bitfields;
include!("../../rust/variadic-compat.rs");

extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type _i3String;
    pub type ev_loop;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn vfprintf(
        __s: *mut FILE,
        __format: *const ::core::ffi::c_char,
        __arg: ::core::ffi::VaList,
    ) -> ::core::ffi::c_int;
    static mut optarg: *mut ::core::ffi::c_char;
    fn smalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn sstrdup(str: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn init_dpi();
    fn ev_default_loop(flags: ::core::ffi::c_uint) -> *mut ev_loop;
    fn ev_loop_destroy(loop_0: *mut ev_loop);
    fn ev_run(loop_0: *mut ev_loop, flags: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn ev_break(loop_0: *mut ev_loop, how: ::core::ffi::c_int);
    fn ev_signal_start(loop_0: *mut ev_loop, w: *mut ev_signal);
    fn kill_children_at_exit();
    fn init_connection(socket_path: *const ::core::ffi::c_char);
    fn i3_send_msg(type_0: uint32_t, payload: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn init_outputs();
    fn init_xcb_early() -> *mut ::core::ffi::c_char;
    fn clean_xcb();
    static mut config: config_t;
    fn getopt_long(
        ___argc: ::core::ffi::c_int,
        ___argv: *const *mut ::core::ffi::c_char,
        __shortopts: *const ::core::ffi::c_char,
        __longopts: *const option,
        __longind: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn glob(
        __pattern: *const ::core::ffi::c_char,
        __flags: ::core::ffi::c_int,
        __errfunc: Option<
            unsafe extern "C" fn(
                *const ::core::ffi::c_char,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        __pglob: *mut glob_t,
    ) -> ::core::ffi::c_int;
    fn globfree(__pglob: *mut glob_t);
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
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn atexit(__func: Option<unsafe extern "C" fn() -> ()>) -> ::core::ffi::c_int;
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn getenv(__name: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
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
pub type __dev_t = ::core::ffi::c_ulong;
pub type __uid_t = ::core::ffi::c_uint;
pub type __gid_t = ::core::ffi::c_uint;
pub type __ino_t = ::core::ffi::c_ulong;
pub type __ino64_t = ::core::ffi::c_ulong;
pub type __mode_t = ::core::ffi::c_uint;
pub type __nlink_t = ::core::ffi::c_ulong;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __time_t = ::core::ffi::c_long;
pub type __blksize_t = ::core::ffi::c_long;
pub type __blkcnt_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
    pub d_ino: __ino64_t,
    pub d_off: __off64_t,
    pub d_reclen: ::core::ffi::c_ushort,
    pub d_type: ::core::ffi::c_uchar,
    pub d_name: [::core::ffi::c_char; 256],
}
pub type i3String = _i3String;
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
pub struct ev_signal {
    pub active: ::core::ffi::c_int,
    pub pending: ::core::ffi::c_int,
    pub priority: ::core::ffi::c_int,
    pub data: *mut ::core::ffi::c_void,
    pub cb: Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_signal, ::core::ffi::c_int) -> ()>,
    pub next: *mut ev_watcher_list,
    pub signum: ::core::ffi::c_int,
}
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const EVBREAK_ALL: C2RustUnnamed = 2;
pub const EVBREAK_ONE: C2RustUnnamed = 1;
pub const EVBREAK_CANCEL: C2RustUnnamed = 0;
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
pub type position_t = ::core::ffi::c_uint;
pub const POS_BOT: position_t = 2;
pub const POS_TOP: position_t = 1;
pub const POS_NONE: position_t = 0;
pub type bar_display_mode_t = ::core::ffi::c_uint;
pub const M_INVISIBLE: bar_display_mode_t = 2;
pub const M_HIDE: bar_display_mode_t = 1;
pub const M_DOCK: bar_display_mode_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct binding_t {
    pub input_code: ::core::ffi::c_int,
    pub command: *mut ::core::ffi::c_char,
    pub release: bool,
    pub bindings: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub tqe_next: *mut binding_t,
    pub tqe_prev: *mut *mut binding_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tray_output_t {
    pub output: *mut ::core::ffi::c_char,
    pub tray_outputs: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub tqe_next: *mut tray_output_t,
    pub tqe_prev: *mut *mut tray_output_t,
}
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
    pub hidden_state: C2RustUnnamed_2,
}
pub type C2RustUnnamed_2 = ::core::ffi::c_uint;
pub const S_SHOW: C2RustUnnamed_2 = 1;
pub const S_HIDE: C2RustUnnamed_2 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tray_outputs_head {
    pub tqh_first: *mut tray_output_t,
    pub tqh_last: *mut *mut tray_output_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bindings_head {
    pub tqh_first: *mut binding_t,
    pub tqh_last: *mut *mut binding_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const ::core::ffi::c_char,
    pub has_arg: ::core::ffi::c_int,
    pub flag: *mut ::core::ffi::c_int,
    pub val: ::core::ffi::c_int,
}
pub type __size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glob_t {
    pub gl_pathc: __size_t,
    pub gl_pathv: *mut *mut ::core::ffi::c_char,
    pub gl_offs: __size_t,
    pub gl_flags: ::core::ffi::c_int,
    pub gl_closedir: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub gl_readdir: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut dirent>,
    pub gl_opendir:
        Option<unsafe extern "C" fn(*const ::core::ffi::c_char) -> *mut ::core::ffi::c_void>,
    pub gl_lstat:
        Option<unsafe extern "C" fn(*const ::core::ffi::c_char, *mut stat) -> ::core::ffi::c_int>,
    pub gl_stat:
        Option<unsafe extern "C" fn(*const ::core::ffi::c_char, *mut stat) -> ::core::ffi::c_int>,
}
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SIGINT: ::core::ffi::c_int = 2;
pub const SIGTERM: ::core::ffi::c_int = 15;
pub const SIGHUP: ::core::ffi::c_int = 1;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[inline]
unsafe extern "C" fn ev_loop(mut loop_0: *mut ev_loop, mut flags: ::core::ffi::c_int) {
    ev_run(loop_0, flags);
}
#[inline]
unsafe extern "C" fn ev_unloop(mut loop_0: *mut ev_loop, mut how: ::core::ffi::c_int) {
    ev_break(loop_0, how);
}
#[inline]
unsafe extern "C" fn ev_default_destroy() {
    ev_loop_destroy(ev_default_loop(0 as ::core::ffi::c_uint));
}
pub const no_argument: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const required_argument: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const GLOB_NOCHECK: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int;
pub const GLOB_TILDE: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int;
pub const I3_IPC_MESSAGE_TYPE_GET_BAR_CONFIG: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
#[no_mangle]
pub static mut main_loop: *mut ev_loop = ::core::ptr::null::<ev_loop>() as *mut ev_loop;
#[no_mangle]
pub unsafe extern "C" fn verboselog(mut fmt: *mut ::core::ffi::c_char, mut args: ...) {
    let mut args_0;
    args_0 = args.clone();
    vfprintf(stdout, fmt, i333_va_list!(args_0));
}
#[no_mangle]
pub unsafe extern "C" fn errorlog(mut fmt: *mut ::core::ffi::c_char, mut args: ...) {
    let mut args_0;
    args_0 = args.clone();
    vfprintf(stderr, fmt, i333_va_list!(args_0));
}
#[no_mangle]
pub unsafe extern "C" fn debuglog(mut fmt: *mut ::core::ffi::c_char, mut args: ...) {}
unsafe extern "C" fn expand_path(mut path: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    static mut globbuf: glob_t = glob_t {
        gl_pathc: 0,
        gl_pathv: ::core::ptr::null::<*mut ::core::ffi::c_char>() as *mut *mut ::core::ffi::c_char,
        gl_offs: 0,
        gl_flags: 0,
        gl_closedir: None,
        gl_readdir: None,
        gl_opendir: None,
        gl_lstat: None,
        gl_stat: None,
    };
    if glob(path, GLOB_NOCHECK | GLOB_TILDE, None, &raw mut globbuf) < 0 as ::core::ffi::c_int {
        fprintf(
            stderr,
            b"[%s:%d] ERROR: glob() failed\n\0" as *const u8 as *const ::core::ffi::c_char,
            b"../i3bar/src/main.c\0" as *const u8 as *const ::core::ffi::c_char,
            50 as ::core::ffi::c_int,
        );
        exit(EXIT_FAILURE);
    }
    let mut result: *mut ::core::ffi::c_char = sstrdup(if globbuf.gl_pathc > 0 as __size_t {
        *globbuf.gl_pathv.offset(0 as ::core::ffi::c_int as isize)
    } else {
        path
    });
    globfree(&raw mut globbuf);
    return result;
}
unsafe extern "C" fn print_usage(mut elf_name: *mut ::core::ffi::c_char) {
    printf(
        b"Usage: %s [-b bar_id] [-s sock_path] [-t] [-h] [-v] [-V]\n\0" as *const u8
            as *const ::core::ffi::c_char,
        elf_name,
    );
    printf(b"\n\0" as *const u8 as *const ::core::ffi::c_char);
    printf(
        b"-b, --bar_id       <bar_id>\tBar ID for which to get the configuration, defaults to the first bar from the i3 config\n\0"
            as *const u8 as *const ::core::ffi::c_char,
    );
    printf(
        b"-s, --socket       <sock_path>\tConnect to i3 via <sock_path>\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    printf(
        b"-t, --transparency Enable transparency (RGBA colors)\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    printf(
        b"-h, --help         Display this help message and exit\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    printf(
        b"-v, --version      Display version number and exit\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    printf(
        b"-V, --verbose      Enable verbose mode\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    printf(b"\n\0" as *const u8 as *const ::core::ffi::c_char);
    printf(
        b" PLEASE NOTE that i3bar will be automatically started by i3\n as soon as there is a 'bar' configuration block in your\n config file. You should never need to start it manually.\n\0"
            as *const u8 as *const ::core::ffi::c_char,
    );
    printf(b"\n\0" as *const u8 as *const ::core::ffi::c_char);
}
unsafe extern "C" fn sig_cb(
    mut loop_0: *mut ev_loop,
    mut watcher: *mut ev_signal,
    mut revents: ::core::ffi::c_int,
) {
    match (*watcher).signum {
        SIGTERM => {
            if config.verbose {
                printf(
                    b"[%s:%d] Got a SIGTERM, stopping\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    b"../i3bar/src/main.c\0" as *const u8 as *const ::core::ffi::c_char,
                    83 as ::core::ffi::c_int,
                );
            }
        }
        SIGINT => {
            if config.verbose {
                printf(
                    b"[%s:%d] Got a SIGINT, stopping\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    b"../i3bar/src/main.c\0" as *const u8 as *const ::core::ffi::c_char,
                    86 as ::core::ffi::c_int,
                );
            }
        }
        SIGHUP => {
            if config.verbose {
                printf(
                    b"[%s:%d] Got a SIGHUP, stopping\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    b"../i3bar/src/main.c\0" as *const u8 as *const ::core::ffi::c_char,
                    89 as ::core::ffi::c_int,
                );
            }
        }
        _ => {}
    }
    ev_unloop(main_loop, EVBREAK_ALL as ::core::ffi::c_int);
}
unsafe fn main_0(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut socket_path: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    memset(
        &raw mut config as *mut ::core::ffi::c_void,
        '\0' as i32,
        ::core::mem::size_of::<config_t>() as size_t,
    );
    static mut long_opt: [option; 7] = [
        option {
            name: b"socket\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 's' as i32,
        },
        option {
            name: b"bar_id\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 'b' as i32,
        },
        option {
            name: b"transparency\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 't' as i32,
        },
        option {
            name: b"help\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 'h' as i32,
        },
        option {
            name: b"version\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 'v' as i32,
        },
        option {
            name: b"verbose\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 'V' as i32,
        },
        option {
            name: ::core::ptr::null::<::core::ffi::c_char>(),
            has_arg: 0 as ::core::ffi::c_int,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 0 as ::core::ffi::c_int,
        },
    ];
    let mut opt: ::core::ffi::c_int = 0;
    let mut option_index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    loop {
        opt = getopt_long(
            argc,
            argv,
            b"b:s:thvV\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut long_opt as *mut option,
            &raw mut option_index,
        );
        if !(opt != -(1 as ::core::ffi::c_int)) {
            break;
        }
        match opt {
            115 => {
                socket_path = expand_path(optarg);
            }
            118 => {
                printf(
                    concat!(
                        "i3bar version ",
                        env!("I3_VERSION"),
                        " © 2010 Axel Wagner and contributors\n\0"
                    )
                    .as_bytes()
                    .as_ptr() as *const u8 as *const ::core::ffi::c_char,
                );
                exit(EXIT_SUCCESS);
            }
            98 => {
                config.bar_id = sstrdup(optarg);
            }
            116 => {
                config.transparency = true_0 != 0;
            }
            86 => {
                config.verbose = true_0 != 0;
            }
            _ => {
                print_usage(*argv.offset(0 as ::core::ffi::c_int as isize));
                exit(EXIT_FAILURE);
            }
        }
    }
    verboselog(
        concat!(
            "[libi3] ../i3bar/src/main.c i3bar version ",
            env!("I3_VERSION"),
            "\n\0"
        )
        .as_bytes()
        .as_ptr() as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    main_loop = ev_default_loop(0 as ::core::ffi::c_uint);
    let mut atom_sock_path: *mut ::core::ffi::c_char = init_xcb_early();
    if socket_path.is_null() {
        socket_path = getenv(b"I3SOCK\0" as *const u8 as *const ::core::ffi::c_char);
        if !socket_path.is_null() {
            socket_path = sstrdup(socket_path);
        }
    }
    if socket_path.is_null() {
        socket_path = atom_sock_path;
    } else {
        free(atom_sock_path as *mut ::core::ffi::c_void);
    }
    if socket_path.is_null() {
        let mut i3_default_sock_path: *mut ::core::ffi::c_char = b"/tmp/i3-ipc.sock\0" as *const u8
            as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char;
        fprintf(
            stderr,
            b"[%s:%d] ERROR: No socket path specified, default to %s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"../i3bar/src/main.c\0" as *const u8 as *const ::core::ffi::c_char,
            157 as ::core::ffi::c_int,
            i3_default_sock_path,
        );
        socket_path = sstrdup(i3_default_sock_path);
    }
    init_dpi();
    init_outputs();
    init_connection(socket_path);
    i3_send_msg(
        I3_IPC_MESSAGE_TYPE_GET_BAR_CONFIG as uint32_t,
        config.bar_id,
    );
    free(socket_path as *mut ::core::ffi::c_void);
    let mut sig_term: *mut ev_signal =
        smalloc(::core::mem::size_of::<ev_signal>() as size_t) as *mut ev_signal;
    let mut sig_int: *mut ev_signal =
        smalloc(::core::mem::size_of::<ev_signal>() as size_t) as *mut ev_signal;
    let mut sig_hup: *mut ev_signal =
        smalloc(::core::mem::size_of::<ev_signal>() as size_t) as *mut ev_signal;
    let ref mut fresh0 = (*(sig_term as *mut ::core::ffi::c_void as *mut ev_watcher)).pending;
    *fresh0 = 0 as ::core::ffi::c_int;
    (*(sig_term as *mut ::core::ffi::c_void as *mut ev_watcher)).active = *fresh0;
    (*(sig_term as *mut ::core::ffi::c_void as *mut ev_watcher)).priority = 0 as ::core::ffi::c_int;
    (*sig_term).cb = Some(
        sig_cb as unsafe extern "C" fn(*mut ev_loop, *mut ev_signal, ::core::ffi::c_int) -> (),
    )
        as Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_signal, ::core::ffi::c_int) -> ()>;
    memmove(
        &raw mut (*(sig_term as *mut ev_watcher)).cb as *mut ::core::ffi::c_void,
        &raw mut (*sig_term).cb as *const ::core::ffi::c_void,
        ::core::mem::size_of::<
            Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_signal, ::core::ffi::c_int) -> ()>,
        >() as size_t,
    );
    (*sig_term).signum = 15 as ::core::ffi::c_int;
    let ref mut fresh1 = (*(sig_int as *mut ::core::ffi::c_void as *mut ev_watcher)).pending;
    *fresh1 = 0 as ::core::ffi::c_int;
    (*(sig_int as *mut ::core::ffi::c_void as *mut ev_watcher)).active = *fresh1;
    (*(sig_int as *mut ::core::ffi::c_void as *mut ev_watcher)).priority = 0 as ::core::ffi::c_int;
    (*sig_int).cb = Some(
        sig_cb as unsafe extern "C" fn(*mut ev_loop, *mut ev_signal, ::core::ffi::c_int) -> (),
    )
        as Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_signal, ::core::ffi::c_int) -> ()>;
    memmove(
        &raw mut (*(sig_int as *mut ev_watcher)).cb as *mut ::core::ffi::c_void,
        &raw mut (*sig_int).cb as *const ::core::ffi::c_void,
        ::core::mem::size_of::<
            Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_signal, ::core::ffi::c_int) -> ()>,
        >() as size_t,
    );
    (*sig_int).signum = 2 as ::core::ffi::c_int;
    let ref mut fresh2 = (*(sig_hup as *mut ::core::ffi::c_void as *mut ev_watcher)).pending;
    *fresh2 = 0 as ::core::ffi::c_int;
    (*(sig_hup as *mut ::core::ffi::c_void as *mut ev_watcher)).active = *fresh2;
    (*(sig_hup as *mut ::core::ffi::c_void as *mut ev_watcher)).priority = 0 as ::core::ffi::c_int;
    (*sig_hup).cb = Some(
        sig_cb as unsafe extern "C" fn(*mut ev_loop, *mut ev_signal, ::core::ffi::c_int) -> (),
    )
        as Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_signal, ::core::ffi::c_int) -> ()>;
    memmove(
        &raw mut (*(sig_hup as *mut ev_watcher)).cb as *mut ::core::ffi::c_void,
        &raw mut (*sig_hup).cb as *const ::core::ffi::c_void,
        ::core::mem::size_of::<
            Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_signal, ::core::ffi::c_int) -> ()>,
        >() as size_t,
    );
    (*sig_hup).signum = 1 as ::core::ffi::c_int;
    ev_signal_start(main_loop, sig_term);
    ev_signal_start(main_loop, sig_int);
    ev_signal_start(main_loop, sig_hup);
    atexit(Some(kill_children_at_exit as unsafe extern "C" fn() -> ()));
    ev_loop(main_loop, 0 as ::core::ffi::c_int);
    clean_xcb();
    ev_default_destroy();
    return 0 as ::core::ffi::c_int;
}
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const EXIT_SUCCESS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub fn main() {
    let mut args_strings: Vec<Vec<u8>> = ::std::env::args()
        .map(|arg| {
            ::std::ffi::CString::new(arg)
                .expect("Failed to convert argument into CString.")
                .into_bytes_with_nul()
        })
        .collect();
    let mut args_ptrs: Vec<*mut ::core::ffi::c_char> = args_strings
        .iter_mut()
        .map(|arg| arg.as_mut_ptr() as *mut ::core::ffi::c_char)
        .chain(::core::iter::once(::core::ptr::null_mut()))
        .collect();
    unsafe {
        ::std::process::exit(main_0(
            (args_ptrs.len() - 1) as ::core::ffi::c_int,
            args_ptrs.as_mut_ptr() as *mut *mut ::core::ffi::c_char,
        ) as i32)
    }
}
