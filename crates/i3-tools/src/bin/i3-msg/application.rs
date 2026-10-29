// i3 - an improved tiling window manager
// Copyright © 2009 Michael Stapelberg and contributors (see LICENSE).
// Direct port of i3-msg/main.c. Keep the C callbacks, getopt, and IPC behavior.
#![allow(non_camel_case_types, non_upper_case_globals, dead_code)]

use libc::{c_char, c_int, c_void, size_t, FILE};
use std::ptr::{null, null_mut};

use i3_common::i333_va_list;

unsafe extern "C" {
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    static mut optarg: *mut c_char;
    static mut optind: c_int;
    fn vfprintf(stream: *mut FILE, fmt: *const c_char, args: core::ffi::VaList) -> c_int;
    fn err(status: c_int, fmt: *const c_char, ...) -> !;
    fn errx(status: c_int, fmt: *const c_char, ...) -> !;
    fn sstrdup(s: *const c_char) -> *mut c_char;
    fn sstrndup(s: *const c_char, n: size_t) -> *mut c_char;
    fn sasprintf(s: *mut *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn ipc_connect(path: *const c_char) -> c_int;
    fn ipc_send_message(fd: c_int, size: u32, kind: u32, payload: *const u8) -> c_int;
    fn ipc_recv_message(fd: c_int, kind: *mut u32, size: *mut u32, payload: *mut *mut u8) -> c_int;
}

#[repr(C)]
struct yajl_callbacks {
    yajl_null: Option<unsafe extern "C" fn(*mut c_void) -> c_int>,
    yajl_boolean: Option<unsafe extern "C" fn(*mut c_void, c_int) -> c_int>,
    yajl_integer: Option<unsafe extern "C" fn(*mut c_void, i64) -> c_int>,
    yajl_double: Option<unsafe extern "C" fn(*mut c_void, f64) -> c_int>,
    yajl_number: Option<unsafe extern "C" fn(*mut c_void, *const c_char, size_t) -> c_int>,
    yajl_string: Option<unsafe extern "C" fn(*mut c_void, *const u8, size_t) -> c_int>,
    yajl_start_map: Option<unsafe extern "C" fn(*mut c_void) -> c_int>,
    yajl_map_key: Option<unsafe extern "C" fn(*mut c_void, *const u8, size_t) -> c_int>,
    yajl_end_map: Option<unsafe extern "C" fn(*mut c_void) -> c_int>,
    yajl_start_array: Option<unsafe extern "C" fn(*mut c_void) -> c_int>,
    yajl_end_array: Option<unsafe extern "C" fn(*mut c_void) -> c_int>,
}
#[link(name = "yajl")]
unsafe extern "C" {
    fn yajl_alloc(
        callbacks: *const yajl_callbacks,
        afs: *mut c_void,
        ctx: *mut c_void,
    ) -> *mut c_void;
    fn yajl_parse(handle: *mut c_void, json: *const u8, len: size_t) -> c_int;
    fn yajl_free(handle: *mut c_void);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn verboselog(fmt: *mut c_char, mut args: ...) {
    // SAFETY: libi3 supplies printf-compatible arguments; stdout is owned by libc.
    unsafe {
        vfprintf(stdout, fmt, i333_va_list!(args));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn errorlog(fmt: *mut c_char, mut args: ...) {
    // SAFETY: libi3 supplies printf-compatible arguments; stderr is owned by libc.
    unsafe {
        vfprintf(stderr, fmt, i333_va_list!(args));
    }
}

static mut last_key: *mut c_char = null_mut();
#[repr(C)]
struct reply_t {
    success: bool,
    error: *mut c_char,
    input: *mut c_char,
    errorposition: *mut c_char,
}
static mut exit_code: c_int = 0;
static mut last_reply: reply_t = reply_t {
    success: false,
    error: null_mut(),
    input: null_mut(),
    errorposition: null_mut(),
};

unsafe extern "C" fn reply_boolean_cb(_: *mut c_void, val: c_int) -> c_int {
    // SAFETY: YAJL invokes callbacks synchronously; last_key is allocated by its key callback.
    unsafe {
        if libc::strcmp(last_key, c"success".as_ptr()) == 0 {
            last_reply.success = val != 0;
        }
    }
    1
}
unsafe extern "C" fn reply_string_cb(_: *mut c_void, val: *const u8, len: size_t) -> c_int {
    // SAFETY: YAJL lends val for len bytes; sstrndup copies into malloc-owned storage.
    unsafe {
        let s = sstrndup(val.cast(), len);
        if libc::strcmp(last_key, c"error".as_ptr()) == 0 {
            last_reply.error = s;
        } else if libc::strcmp(last_key, c"input".as_ptr()) == 0 {
            last_reply.input = s;
        } else if libc::strcmp(last_key, c"errorposition".as_ptr()) == 0 {
            last_reply.errorposition = s;
        } else {
            libc::free(s.cast());
        }
    }
    1
}
unsafe extern "C" fn reply_start_map_cb(_: *mut c_void) -> c_int {
    1
}
unsafe extern "C" fn reply_end_map_cb(_: *mut c_void) -> c_int {
    // SAFETY: These process-global callback values are only accessed on the main thread.
    unsafe {
        if !last_reply.success {
            if !last_reply.input.is_null() {
                libc::fprintf(
                    stderr,
                    c"ERROR: Your command: %s\n".as_ptr(),
                    last_reply.input,
                );
                libc::fprintf(
                    stderr,
                    c"ERROR:               %s\n".as_ptr(),
                    last_reply.errorposition,
                );
            }
            libc::fprintf(stderr, c"ERROR: %s\n".as_ptr(), last_reply.error);
            exit_code = 2;
        }
    }
    1
}
unsafe extern "C" fn reply_map_key_cb(_: *mut c_void, key: *const u8, len: size_t) -> c_int {
    // SAFETY: Previous key is malloc-owned and YAJL lends the next key for len bytes.
    unsafe {
        libc::free(last_key.cast());
        last_key = sstrndup(key.cast(), len);
    }
    1
}
static reply_callbacks: yajl_callbacks = yajl_callbacks {
    yajl_null: None,
    yajl_boolean: Some(reply_boolean_cb),
    yajl_integer: None,
    yajl_double: None,
    yajl_number: None,
    yajl_string: Some(reply_string_cb),
    yajl_start_map: Some(reply_start_map_cb),
    yajl_map_key: Some(reply_map_key_cb),
    yajl_end_map: Some(reply_end_map_cb),
    yajl_start_array: None,
    yajl_end_array: None,
};

static mut config_last_key: *mut c_char = null_mut();
unsafe extern "C" fn config_string_cb(_: *mut c_void, val: *const u8, len: size_t) -> c_int {
    // SAFETY: YAJL provides the input span; copied strings retain C allocation ownership.
    unsafe {
        let s = sstrndup(val.cast(), len);
        if libc::strcmp(config_last_key, c"config".as_ptr()) == 0 {
            libc::fprintf(stdout, c"%s".as_ptr(), s);
        }
        libc::free(s.cast());
    }
    1
}
unsafe extern "C" fn config_start_map_cb(_: *mut c_void) -> c_int {
    1
}
unsafe extern "C" fn config_end_map_cb(_: *mut c_void) -> c_int {
    1
}
unsafe extern "C" fn config_map_key_cb(_: *mut c_void, key: *const u8, len: size_t) -> c_int {
    // SAFETY: Key is a borrowed YAJL span; mirror upstream's malloc-owned key replacement.
    unsafe {
        config_last_key = sstrndup(key.cast(), len);
    }
    1
}
static config_callbacks: yajl_callbacks = yajl_callbacks {
    yajl_null: None,
    yajl_boolean: None,
    yajl_integer: None,
    yajl_double: None,
    yajl_number: None,
    yajl_string: Some(config_string_cb),
    yajl_start_map: Some(config_start_map_cb),
    yajl_map_key: Some(config_map_key_cb),
    yajl_end_map: Some(config_end_map_cb),
    yajl_start_array: None,
    yajl_end_array: None,
};

// Build integration supplies exactly the version string used by config.h.
const VERSION: &str = match option_env!("I3_VERSION") {
    Some(v) => v,
    None => "4.25",
};

pub unsafe fn main_1(argc: c_int, argv: *mut *mut c_char) -> c_int {
    // SAFETY: argv is a writable, null-terminated C argv; all C allocations and handles
    // follow main.c's lifetimes. libc getopt may permute argv. IPC owns reply allocations.
    unsafe {
        let mut socket_path: *mut c_char = null_mut();
        let mut option_index = 0;
        let mut message_type = 0u32;
        let mut payload: *mut c_char = null_mut();
        let mut quiet = false;
        let mut monitor = false;
        let mut raw_reply = false;
        let long_options = [
            libc::option {
                name: c"socket".as_ptr(),
                has_arg: 1,
                flag: null_mut(),
                val: b's' as c_int,
            },
            libc::option {
                name: c"type".as_ptr(),
                has_arg: 1,
                flag: null_mut(),
                val: b't' as c_int,
            },
            libc::option {
                name: c"version".as_ptr(),
                has_arg: 0,
                flag: null_mut(),
                val: b'v' as c_int,
            },
            libc::option {
                name: c"quiet".as_ptr(),
                has_arg: 0,
                flag: null_mut(),
                val: b'q' as c_int,
            },
            libc::option {
                name: c"monitor".as_ptr(),
                has_arg: 0,
                flag: null_mut(),
                val: b'm' as c_int,
            },
            libc::option {
                name: c"help".as_ptr(),
                has_arg: 0,
                flag: null_mut(),
                val: b'h' as c_int,
            },
            libc::option {
                name: c"raw".as_ptr(),
                has_arg: 0,
                flag: null_mut(),
                val: b'r' as c_int,
            },
            libc::option {
                name: null(),
                has_arg: 0,
                flag: null_mut(),
                val: 0,
            },
        ];
        loop {
            let o = libc::getopt_long(
                argc,
                argv,
                c"s:t:vhqmr".as_ptr(),
                long_options.as_ptr(),
                &mut option_index,
            );
            if o == -1 {
                break;
            }
            match o as u8 {
                b's' => {
                    libc::free(socket_path.cast());
                    socket_path = sstrdup(optarg);
                }
                b't' => {
                    let names = [
                        (c"command", 0),
                        (c"run_command", 0),
                        (c"get_workspaces", 1),
                        (c"get_outputs", 3),
                        (c"get_tree", 4),
                        (c"get_marks", 5),
                        (c"get_bar_config", 6),
                        (c"get_binding_modes", 8),
                        (c"get_binding_state", 12),
                        (c"get_version", 7),
                        (c"get_config", 9),
                        (c"send_tick", 10),
                        (c"subscribe", 2),
                    ];
                    let mut found = false;
                    for (name, kind) in names {
                        if libc::strcasecmp(optarg, name.as_ptr()) == 0 {
                            message_type = kind;
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        libc::printf(c"Unknown message type\n".as_ptr());
                        libc::printf(c"Known types: run_command, get_workspaces, get_outputs, get_tree, get_marks, get_bar_config, get_binding_modes, get_binding_state, get_version, get_config, send_tick, subscribe\n".as_ptr());
                        libc::exit(1);
                    }
                }
                b'q' => quiet = true,
                b'm' => monitor = true,
                b'v' => {
                    println!("i3-msg {VERSION}");
                    return 0;
                }
                b'h' => {
                    println!("i3-msg {VERSION}\ni3-msg [-s <socket>] [-t <type>] [-m] <message>");
                    return 0;
                }
                b'?' => libc::exit(1),
                b'r' => raw_reply = true,
                _ => {}
            }
        }
        if monitor && message_type != 2 {
            libc::fprintf(
                stderr,
                c"The monitor option -m is used with -t SUBSCRIBE exclusively.\n".as_ptr(),
            );
            libc::exit(1);
        }
        while optind < argc {
            if payload.is_null() {
                payload = sstrdup(*argv.add(optind as usize));
            } else {
                let mut both = null_mut();
                sasprintf(
                    &mut both,
                    c"%s %s".as_ptr(),
                    payload,
                    *argv.add(optind as usize),
                );
                libc::free(payload.cast());
                payload = both;
            }
            optind += 1;
        }
        if payload.is_null() {
            payload = sstrdup(c"".as_ptr());
        }
        let sockfd = ipc_connect(socket_path);
        if ipc_send_message(
            sockfd,
            libc::strlen(payload) as u32,
            message_type,
            payload.cast(),
        ) == -1
        {
            err(1, c"IPC: write()".as_ptr());
        }
        libc::free(payload.cast());
        let mut reply_length = 0u32;
        let mut reply_type = 0u32;
        let mut reply: *mut u8 = null_mut();
        let ret = ipc_recv_message(sockfd, &mut reply_type, &mut reply_length, &mut reply);
        if ret != 0 {
            if ret == -1 {
                err(1, c"IPC: read()".as_ptr());
            }
            libc::exit(1);
        }
        if reply_type != message_type {
            errx(
                1,
                c"IPC: Received reply of type %d but expected %d".as_ptr(),
                reply_type,
                message_type,
            );
        }
        if reply_type == 0 {
            if !raw_reply {
                let handle = yajl_alloc(&reply_callbacks, null_mut(), null_mut());
                let state = yajl_parse(handle, reply, reply_length as usize);
                yajl_free(handle);
                if state != 0 {
                    errx(1, c"IPC: Could not parse JSON reply.".as_ptr());
                }
            }
            if !quiet || raw_reply {
                libc::printf(c"%.*s\n".as_ptr(), reply_length, reply);
            }
        } else if reply_type == 9 {
            if raw_reply {
                libc::printf(c"%.*s\n".as_ptr(), reply_length, reply);
            } else {
                let handle = yajl_alloc(&config_callbacks, null_mut(), null_mut());
                let state = yajl_parse(handle, reply, reply_length as usize);
                yajl_free(handle);
                if state != 0 {
                    errx(1, c"IPC: Could not parse JSON reply.".as_ptr());
                }
            }
        } else if reply_type == 2 {
            loop {
                libc::free(reply.cast());
                let ret = ipc_recv_message(sockfd, &mut reply_type, &mut reply_length, &mut reply);
                if ret != 0 {
                    if ret == -1 {
                        err(1, c"IPC: read()".as_ptr());
                    }
                    libc::exit(1);
                }
                if reply_type & (1u32 << 31) == 0 {
                    errx(
                        1,
                        c"IPC: Received reply of type %d but expected an event".as_ptr(),
                        reply_type,
                    );
                }
                if !quiet {
                    libc::fprintf(stdout, c"%.*s\n".as_ptr(), reply_length, reply);
                    libc::fflush(stdout);
                }
                if !monitor {
                    break;
                }
            }
        } else if !quiet {
            libc::printf(c"%.*s\n".as_ptr(), reply_length, reply);
        }
        libc::free(reply.cast());
        libc::close(sockfd);
        exit_code
    }
}

pub fn main() {
    use std::os::unix::ffi::OsStrExt;
    let args: Vec<_> = std::env::args_os()
        .map(|s| std::ffi::CString::new(s.as_bytes()).expect("argument contains NUL"))
        .collect();
    let mut argv: Vec<_> = args.iter().map(|s| s.as_ptr().cast_mut()).collect();
    argv.push(null_mut());
    // SAFETY: CString allocations live until main_1 returns and argv includes its null terminator.
    std::process::exit(unsafe { main_1(args.len() as c_int, argv.as_mut_ptr()) });
}

// PORT STATUS: source i3-msg/main.c; direct C ABI port; requires libi3, libc, YAJL.
