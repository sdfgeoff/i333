// i3 - an improved tiling window manager
// Copyright © 2009 Michael Stapelberg and contributors (see LICENSE).
// Direct port of i3-dump-log/main.c; preserves the shared-memory ABI and syscalls.
#![allow(non_camel_case_types, non_upper_case_globals, dead_code)]

use libc::{c_char, c_int, c_void, FILE};
use std::ptr::{null, null_mut};

include!("../rust/variadic-compat.rs");

unsafe extern "C" {
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn vfprintf(stream: *mut FILE, fmt: *const c_char, args: core::ffi::VaList) -> c_int;
    fn err(status: c_int, fmt: *const c_char, ...) -> !;
    fn errx(status: c_int, fmt: *const c_char, ...) -> !;
    fn swrite(fd: c_int, buf: *const c_void, count: libc::size_t) -> libc::ssize_t;
    fn root_atom_contents(atom: *const c_char, conn: *mut c_void, screen: c_int) -> *mut c_char;
    fn ipc_connect(path: *const c_char) -> c_int;
    fn ipc_send_message(fd: c_int, size: u32, kind: u32, payload: *const u8) -> c_int;
    fn ipc_recv_message(fd: c_int, kind: *mut u32, size: *mut u32, payload: *mut *mut u8) -> c_int;
    fn is_debug_build() -> bool;
}
#[link(name = "xcb")]
unsafe extern "C" {
    fn xcb_connect(display: *const c_char, screen: *mut c_int) -> *mut c_void;
    fn xcb_connection_has_error(conn: *mut c_void) -> c_int;
}

#[repr(C)]
struct i3_shmlog_header {
    offset_next_write: u32,
    offset_last_wrap: u32,
    size: u32,
    wrap_count: u32,
}
static mut wrap_count: u32 = 0;
static mut header: *mut i3_shmlog_header = null_mut();
static mut logbuffer: *mut c_char = null_mut();
static mut walk: *mut c_char = null_mut();
static mut ipcfd: c_int = -1;

extern "C" fn disable_shmlog() {
    // SAFETY: atexit runs after the main thread's IPC connection has been initialized.
    unsafe {
        let disablecmd = c"debuglog off; shmlog off";
        if ipc_send_message(
            ipcfd,
            disablecmd.count_bytes() as u32,
            0,
            disablecmd.as_ptr().cast(),
        ) != 0
        {
            err(1, c"IPC send".as_ptr());
        }
        let mut reply_length = 0;
        let mut reply = null_mut();
        if ipc_recv_message(ipcfd, null_mut(), &mut reply_length, &mut reply) != 0 {
            err(1, c"IPC recv".as_ptr());
        }
        libc::free(reply.cast());
    }
}
unsafe fn check_for_wrap() -> c_int {
    // SAFETY: logbuffer/header/walk refer to the shared mapping initialized by main_1.
    unsafe {
        if wrap_count == (*header).wrap_count {
            return 0;
        }
        wrap_count = (*header).wrap_count;
        let len = logbuffer
            .add((*header).offset_last_wrap as usize)
            .offset_from(walk) as c_int;
        swrite(1, walk.cast(), len as libc::size_t);
        walk = logbuffer.add(std::mem::size_of::<i3_shmlog_header>());
        1
    }
}
unsafe fn print_till_end() {
    // SAFETY: Same mapped-buffer invariant as check_for_wrap; offsets originate in i3.
    unsafe {
        check_for_wrap();
        let len = logbuffer
            .add((*header).offset_next_write as usize)
            .offset_from(walk) as c_int;
        swrite(1, walk.cast(), len as libc::size_t);
        walk = walk.offset(len as isize);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn errorlog(fmt: *mut c_char, mut args: ...) {
    // SAFETY: libi3 supplies printf-compatible arguments and libc owns stderr.
    unsafe {
        vfprintf(stderr, fmt, i333_va_list!(args));
    }
}
const VERSION: &str = match option_env!("I3_VERSION") {
    Some(v) => v,
    None => "4.25",
};

pub unsafe fn main_1(argc: c_int, argv: *mut *mut c_char) -> c_int {
    // SAFETY: argv is a live, null-terminated C argv. Resource ownership and pointer
    // movement mirror main.c; shared-memory offsets are provided by the i3 logger.
    unsafe {
        let mut option_index = 0;
        let mut verbose = false;
        #[cfg(not(target_os = "openbsd"))]
        let mut follow = false;
        let long_options = [
            libc::option {
                name: c"version".as_ptr(),
                has_arg: 0,
                flag: null_mut(),
                val: b'v' as c_int,
            },
            libc::option {
                name: c"verbose".as_ptr(),
                has_arg: 0,
                flag: null_mut(),
                val: b'V' as c_int,
            },
            #[cfg(not(target_os = "openbsd"))]
            libc::option {
                name: c"follow".as_ptr(),
                has_arg: 0,
                flag: null_mut(),
                val: b'f' as c_int,
            },
            libc::option {
                name: c"help".as_ptr(),
                has_arg: 0,
                flag: null_mut(),
                val: b'h' as c_int,
            },
            libc::option {
                name: null(),
                has_arg: 0,
                flag: null_mut(),
                val: 0,
            },
        ];
        #[cfg(not(target_os = "openbsd"))]
        let options_string = c"s:vfVh";
        #[cfg(target_os = "openbsd")]
        let options_string = c"vVh";
        loop {
            let o = libc::getopt_long(
                argc,
                argv,
                options_string.as_ptr(),
                long_options.as_ptr(),
                &mut option_index,
            );
            if o == -1 {
                break;
            }
            match o as u8 {
                b'v' => {
                    println!("i3-dump-log {VERSION}");
                    return 0;
                }
                b'V' => verbose = true,
                #[cfg(not(target_os = "openbsd"))]
                b'f' => follow = true,
                b'h' => {
                    println!("i3-dump-log {VERSION}");
                    #[cfg(not(target_os = "openbsd"))]
                    println!("i3-dump-log [-fhVv]");
                    #[cfg(target_os = "openbsd")]
                    println!("i3-dump-log [-hVv]");
                    return 0;
                }
                _ => {}
            }
        }
        let mut shmname = root_atom_contents(c"I3_SHMLOG_PATH".as_ptr(), null_mut(), 0);
        if shmname.is_null() {
            let mut screen = 0;
            let conn = xcb_connect(null(), &mut screen);
            if conn.is_null() || xcb_connection_has_error(conn) != 0 {
                libc::fprintf(
                    stderr,
                    c"i3-dump-log: ERROR: Cannot connect to X11.\n\n".as_ptr(),
                );
                if libc::getenv(c"DISPLAY".as_ptr()).is_null() {
                    libc::fprintf(
                        stderr,
                        c"Your DISPLAY environment variable is not set.\n".as_ptr(),
                    );
                    libc::fprintf(
                        stderr,
                        c"Are you running i3-dump-log via SSH or on a virtual console?\n".as_ptr(),
                    );
                    libc::fprintf(stderr, c"Try DISPLAY=:0 i3-dump-log\n".as_ptr());
                    libc::exit(1);
                }
                libc::fprintf(
                    stderr,
                    c"FYI: The DISPLAY environment variable is set to \"%s\".\n".as_ptr(),
                    libc::getenv(c"DISPLAY".as_ptr()),
                );
                libc::exit(1);
            }
            if !root_atom_contents(c"I3_CONFIG_PATH".as_ptr(), conn, screen).is_null() {
                libc::fprintf(stderr, c"i3-dump-log: i3 is running, but SHM logging is not enabled. Enabling SHM log now while i3-dump-log is running\n\n".as_ptr());
                ipcfd = ipc_connect(null());
                let enablecmd = c"debuglog on; shmlog 5242880";
                if ipc_send_message(
                    ipcfd,
                    enablecmd.count_bytes() as u32,
                    0,
                    enablecmd.as_ptr().cast(),
                ) != 0
                {
                    err(1, c"IPC send".as_ptr());
                }
                let mut reply_length = 0;
                let mut reply = null_mut();
                if ipc_recv_message(ipcfd, null_mut(), &mut reply_length, &mut reply) != 0 {
                    err(1, c"IPC recv".as_ptr());
                }
                libc::free(reply.cast());
                libc::atexit(disable_shmlog);
                shmname = root_atom_contents(c"I3_SHMLOG_PATH".as_ptr(), null_mut(), 0);
                if shmname.is_null() && !is_debug_build() {
                    let version = std::ffi::CString::new(VERSION).unwrap();
                    libc::fprintf(
                        stderr,
                        c"You seem to be using a release version of i3:\n  %s\n\n".as_ptr(),
                        version.as_ptr(),
                    );
                    libc::fprintf(stderr, c"Release versions do not use SHM logging by default,\ntherefore i3-dump-log does not work.\n\n".as_ptr());
                    libc::fprintf(stderr, c"Please follow this guide instead:\nhttps://i3wm.org/docs/debugging-release-version.html\n".as_ptr());
                    libc::exit(1);
                }
            }
            if shmname.is_null() {
                errx(
                    1,
                    c"Cannot get I3_SHMLOG_PATH atom contents. Is i3 running on this display?"
                        .as_ptr(),
                );
            }
        }
        if *shmname == 0 {
            errx(
                1,
                c"Cannot dump log: SHM logging is disabled in i3.".as_ptr(),
            );
        }
        let mut statbuf: libc::stat = std::mem::zeroed();
        let logbuffer_shm = libc::shm_open(shmname, libc::O_RDWR, 0);
        if logbuffer_shm == -1 {
            err(
                1,
                c"Could not shm_open SHM segment for the i3 log (%s)".as_ptr(),
                shmname,
            );
        }
        if libc::fstat(logbuffer_shm, &mut statbuf) != 0 {
            err(1, c"stat(%s)".as_ptr(), shmname);
        }
        logbuffer = libc::mmap(
            null_mut(),
            statbuf.st_size as usize,
            libc::PROT_READ,
            libc::MAP_SHARED,
            logbuffer_shm,
            0,
        )
        .cast();
        if logbuffer.cast::<c_void>() == libc::MAP_FAILED {
            err(1, c"Could not mmap SHM segment for the i3 log".as_ptr());
        }
        header = logbuffer.cast();
        if verbose {
            libc::printf(
                c"next_write = %d, last_wrap = %d, logbuffer_size = %d, shmname = %s\n".as_ptr(),
                (*header).offset_next_write,
                (*header).offset_last_wrap,
                (*header).size,
                shmname,
            );
        }
        libc::free(shmname.cast());
        walk = logbuffer.add((*header).offset_next_write as usize);
        if *walk != 0 {
            while *walk != b'\n' as c_char {
                walk = walk.add(1);
            }
            walk = walk.add(1);
        }
        wrap_count = 0;
        check_for_wrap();
        walk = logbuffer.add(std::mem::size_of::<i3_shmlog_header>());
        print_till_end();
        #[cfg(not(target_os = "openbsd"))]
        {
            if !follow {
                return 0;
            }
            let log_stream_socket_path =
                root_atom_contents(c"I3_LOG_STREAM_SOCKET_PATH".as_ptr(), null_mut(), 0);
            if log_stream_socket_path.is_null() {
                errx(1, c"could not determine i3 log stream socket path: possible i3-dump-log and i3 version mismatch".as_ptr());
            }
            let sockfd = libc::socket(libc::AF_UNIX, libc::SOCK_STREAM, 0);
            if sockfd == -1 {
                err(1, c"Could not create socket".as_ptr());
            }
            libc::fcntl(sockfd, libc::F_SETFD, libc::FD_CLOEXEC);
            let mut addr: libc::sockaddr_un = std::mem::zeroed();
            addr.sun_family = libc::AF_UNIX as libc::sa_family_t;
            libc::strncpy(
                addr.sun_path.as_mut_ptr(),
                log_stream_socket_path,
                addr.sun_path.len() - 1,
            );
            if libc::connect(
                sockfd,
                (&raw const addr).cast(),
                std::mem::size_of::<libc::sockaddr_un>() as libc::socklen_t,
            ) < 0
            {
                err(
                    1,
                    c"Could not connect to i3 on socket %s".as_ptr(),
                    log_stream_socket_path,
                );
            }
            let mut buf = [0u8; 4096];
            loop {
                let n = libc::read(sockfd, buf.as_mut_ptr().cast(), buf.len()) as c_int;
                if n == -1 {
                    err(1, c"read(log-stream-socket):".as_ptr());
                }
                if n == 0 {
                    libc::exit(0);
                }
                // PORT NOTE: main.c writes buf[n] = '\0', but swrite takes a length.
                // That unused terminator overruns buf on full reads; omit it here.
                swrite(1, buf.as_ptr().cast(), n as libc::size_t);
            }
        }
        #[cfg(target_os = "openbsd")]
        {
            libc::exit(0);
        }
    }
}

pub fn main() {
    use std::os::unix::ffi::OsStrExt;
    let args: Vec<_> = std::env::args_os()
        .map(|s| std::ffi::CString::new(s.as_bytes()).expect("argument contains NUL"))
        .collect();
    let mut argv: Vec<_> = args.iter().map(|s| s.as_ptr().cast_mut()).collect();
    argv.push(null_mut());
    // SAFETY: argv and the CString argument allocations remain live for main_1.
    std::process::exit(unsafe { main_1(args.len() as c_int, argv.as_mut_ptr()) });
}

// PORT STATUS: source i3-dump-log/main.c; direct C ABI port; requires libi3, libc, XCB.
