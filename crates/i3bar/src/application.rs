use crate::config::config;
/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3bar - an xcb-based status- and ws-bar for i3
 * © 2010 Axel Wagner and contributors (see also: LICENSE)
 *
 */
// Mechanical file-by-file Rust translation of i3bar/src/main.c.
// Originally translated with C2Rust 0.22.1; subsequently maintained in Rust.
use i3_common::i333_va_list;
pub use libev_sys::{
    ev_default_destroy, ev_default_loop, ev_loop, ev_signal, ev_signal_start, ev_unloop,
    ev_watcher, EVBREAK_ALL,
};

pub type i3String = _i3String;

pub use crate::config::Binding as binding_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub tqe_next: *mut binding_t,
    pub tqe_prev: *mut *mut binding_t,
}
pub use crate::config::TrayOutput as tray_output_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub tqe_next: *mut tray_output_t,
    pub tqe_prev: *mut *mut tray_output_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct config_t {
    pub modifier: u32,
    pub bindings: bindings_head,
    pub position: crate::application::BarPosition,
    pub verbose: bool,
    pub bar_height: u32,
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
    pub hide_on_modifier: crate::application::BarDisplayMode,
    pub hidden_state: crate::application::HiddenState,
}

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

pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();

pub const I3_IPC_MESSAGE_TYPE_GET_BAR_CONFIG: ::core::ffi::c_int = 6 as ::core::ffi::c_int;

pub static mut main_loop: *mut ev_loop = ::core::ptr::null::<ev_loop>() as *mut ev_loop;

pub unsafe extern "C" fn verboselog(fmt: *mut ::core::ffi::c_char, args: ...) {
    let mut args_0;
    args_0 = args.clone();
    vfprintf(stdout, fmt, i333_va_list!(args_0));
}

pub unsafe extern "C" fn errorlog(fmt: *mut ::core::ffi::c_char, args: ...) {
    let mut args_0;
    args_0 = args.clone();
    vfprintf(stderr, fmt, i333_va_list!(args_0));
}

pub unsafe extern "C" fn debuglog(fmt: *mut ::core::ffi::c_char, args: ...) {}
unsafe fn expand_path(path: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    static mut globbuf: glob_t = unsafe { ::core::mem::zeroed() };
    if glob(
        path,
        libc::GLOB_NOCHECK | libc::GLOB_TILDE,
        None,
        &raw mut globbuf,
    ) < 0 as ::core::ffi::c_int
    {
        fprintf(
            stderr,
            c"[%s:%d] ERROR: glob() failed\n".as_ptr(),
            c"../i3bar/src/main.c".as_ptr(),
            50 as ::core::ffi::c_int,
        );
        exit(libc::EXIT_FAILURE);
    }
    let result: *mut ::core::ffi::c_char = sstrdup(if globbuf.gl_pathc > 0 as usize {
        *globbuf.gl_pathv.offset(0 as ::core::ffi::c_int as isize)
    } else {
        path
    });
    globfree(&raw mut globbuf);
    result
}
fn print_usage(program: &std::ffi::OsStr) {
    use std::io::Write;
    use std::os::unix::ffi::OsStrExt;
    let mut output = std::io::stdout().lock();
    let _ = output.write_all(b"Usage: ");
    let _ = output.write_all(program.as_bytes());
    let _ = output.write_all(b" [-b bar_id] [-s sock_path] [-t] [-h] [-v] [-V]\n\n-b, --bar_id       <bar_id>\tBar ID for which to get the configuration, defaults to the first bar from the i3 config\n-s, --socket       <sock_path>\tConnect to i3 via <sock_path>\n-t, --transparency Enable transparency (RGBA colors)\n-h, --help         Display this help message and exit\n-v, --version      Display version number and exit\n-V, --verbose      Enable verbose mode\n\n PLEASE NOTE that i3bar will be automatically started by i3\n as soon as there is a 'bar' configuration block in your\n config file. You should never need to start it manually.\n\n");
}
unsafe extern "C" fn sig_cb(
    loop_0: *mut ev_loop,
    watcher: *mut ev_signal,
    revents: ::core::ffi::c_int,
) {
    match (*watcher).signum {
        libc::SIGTERM => {
            if config.verbose {
                printf(
                    c"[%s:%d] Got a SIGTERM, stopping\n".as_ptr(),
                    c"../i3bar/src/main.c".as_ptr(),
                    83 as ::core::ffi::c_int,
                );
            }
        }
        libc::SIGINT => {
            if config.verbose {
                printf(
                    c"[%s:%d] Got a SIGINT, stopping\n".as_ptr(),
                    c"../i3bar/src/main.c".as_ptr(),
                    86 as ::core::ffi::c_int,
                );
            }
        }
        libc::SIGHUP => {
            if config.verbose {
                printf(
                    c"[%s:%d] Got a SIGHUP, stopping\n".as_ptr(),
                    c"../i3bar/src/main.c".as_ptr(),
                    89 as ::core::ffi::c_int,
                );
            }
        }
        _ => {}
    }
    ev_unloop(main_loop, EVBREAK_ALL as ::core::ffi::c_int);
}
const BAR_OPTIONS: &[i3_common::cli::OptionSpec] = &[
    i3_common::cli::OptionSpec::value(b's', "socket"),
    i3_common::cli::OptionSpec::value(b'b', "bar_id"),
    i3_common::cli::OptionSpec::flag(b't', "transparency"),
    i3_common::cli::OptionSpec::flag(b'h', "help"),
    i3_common::cli::OptionSpec::flag(b'v', "version"),
    i3_common::cli::OptionSpec::flag(b'V', "verbose"),
];
unsafe fn run(arguments: Vec<std::ffi::OsString>) -> ::core::ffi::c_int {
    use i3_common::cli::{Item, Parser};
    use std::os::unix::ffi::OsStrExt;
    let mut arguments = arguments.into_iter();
    let program = arguments.next().unwrap_or_else(|| "i3bar".into());
    let mut socket_path: *mut ::core::ffi::c_char = std::ptr::null_mut();
    memset(
        &raw mut config as *mut ::core::ffi::c_void,
        0,
        ::core::mem::size_of::<config_t>(),
    );
    for item in Parser::new(
        arguments,
        BAR_OPTIONS,
        std::env::var_os("POSIXLY_CORRECT").is_some(),
    ) {
        match item {
            Item::Operand(_) => {}
            Item::Error(message) => {
                eprintln!("{}: {message}", program.to_string_lossy());
                print_usage(&program);
                return libc::EXIT_FAILURE;
            }
            Item::Option { short, value } => match short {
                b'v' => {
                    println!(
                        "i3bar version {} © 2010 Axel Wagner and contributors",
                        env!("I3_VERSION")
                    );
                    return libc::EXIT_SUCCESS;
                }
                b'h' => {
                    print_usage(&program);
                    return libc::EXIT_FAILURE;
                }
                b't' => config.transparency = true,
                b'V' => config.verbose = true,
                b's' | b'b' => {
                    let value = std::ffi::CString::new(value.unwrap().as_bytes())
                        .expect("OS arguments have no NUL bytes");
                    if short == b's' {
                        // Native initialization still owns/frees the expanded
                        // socket path. Never give libc a Rust allocation.
                        free(socket_path.cast());
                        socket_path = expand_path(value.as_ptr().cast_mut());
                    } else {
                        free(config.bar_id.cast());
                        config.bar_id = sstrdup(value.as_ptr());
                    }
                }
                _ => unreachable!(),
            },
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
    let atom_sock_path: *mut ::core::ffi::c_char = init_xcb_early();
    if socket_path.is_null() {
        socket_path = getenv(c"I3SOCK".as_ptr());
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
        let i3_default_sock_path: *mut ::core::ffi::c_char =
            c"/tmp/i3-ipc.sock".as_ptr().cast_mut();
        fprintf(
            stderr,
            c"[%s:%d] ERROR: No socket path specified, default to %s\n".as_ptr(),
            c"../i3bar/src/main.c".as_ptr(),
            157 as ::core::ffi::c_int,
            i3_default_sock_path,
        );
        socket_path = sstrdup(i3_default_sock_path);
    }
    init_dpi();
    init_outputs();
    init_connection(socket_path);
    i3_send_msg(I3_IPC_MESSAGE_TYPE_GET_BAR_CONFIG as u32, config.bar_id);
    free(socket_path as *mut ::core::ffi::c_void);
    let sig_term: *mut ev_signal =
        smalloc(::core::mem::size_of::<ev_signal>() as usize) as *mut ev_signal;
    let sig_int: *mut ev_signal =
        smalloc(::core::mem::size_of::<ev_signal>() as usize) as *mut ev_signal;
    let sig_hup: *mut ev_signal =
        smalloc(::core::mem::size_of::<ev_signal>() as usize) as *mut ev_signal;
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
        >() as usize,
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
        >() as usize,
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
        >() as usize,
    );
    (*sig_hup).signum = 1 as ::core::ffi::c_int;
    ev_signal_start(main_loop, sig_term);
    ev_signal_start(main_loop, sig_int);
    ev_signal_start(main_loop, sig_hup);
    atexit(kill_children_at_exit);
    ev_loop(main_loop, 0 as ::core::ffi::c_int);
    clean_xcb();
    ev_default_destroy();
    0 as ::core::ffi::c_int
}

pub fn main() {
    // Only native graphical initialization remains unsafe. Argument parsing
    // owns its bytes and never borrows libc's process-global getopt storage.
    std::process::exit(unsafe { run(std::env::args_os().collect()) });
}

// Internal helpers use Rust linkage rather than foreign symbol lookup.
use crate::child::kill_children_at_exit;
use crate::ipc::i3_send_msg;
use crate::ipc::init_connection;
use crate::outputs::init_outputs;
use crate::xcb::clean_xcb;
use crate::xcb::init_xcb_early;
use i3_common::dpi::init_dpi;
use i3_common::safewrappers::smalloc;
use i3_common::safewrappers::sstrdup;

use xcb_sys::xcb_color_strings_t;

pub use libc::{
    atexit, exit, fprintf, free, getenv, glob, glob_t, globfree, memmove, memset, printf,
};

use i3_common::native_stdio::{stderr, stdout, vfprintf};

#[cfg(test)]
mod cli_tests {
    use super::*;
    use i3_common::cli::{Item, Parser};
    #[test]
    fn bar_options_keep_attached_values_and_distinct_verbose_flags() {
        let args =
            ["-ts/tmp/socket", "--bar_id=bar-1", "-V", "--version"].map(std::ffi::OsString::from);
        assert_eq!(
            Parser::new(args, BAR_OPTIONS, false).collect::<Vec<_>>(),
            vec![
                Item::Option {
                    short: b't',
                    value: None
                },
                Item::Option {
                    short: b's',
                    value: Some("/tmp/socket".into())
                },
                Item::Option {
                    short: b'b',
                    value: Some("bar-1".into())
                },
                Item::Option {
                    short: b'V',
                    value: None
                },
                Item::Option {
                    short: b'v',
                    value: None
                },
            ]
        );
    }
}

pub use i3_common::string::I3String as _i3String;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum BarPosition {
    None = 0,
    Top = 1,
    Bottom = 2,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum BarDisplayMode {
    Dock = 0,
    Hide = 1,
    Invisible = 2,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum HiddenState {
    Hide = 0,
    Show = 1,
}
