use i3_common::safewrappers::sasprintf;
/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * log.c: Logging functions.
 *
 */
// Mechanical file-by-file Rust translation of src/log.c.
// Originally translated with C2Rust 0.22.1; subsequently maintained in Rust.
use i3_common::i333_va_list;
pub use libev_sys::{ev_io, ev_loop};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct log_client {
    pub fd: ::core::ffi::c_int,
    pub clients: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub tqe_next: *mut log_client,
    pub tqe_prev: *mut *mut log_client,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct log_client_head {
    pub tqh_first: *mut log_client,
    pub tqh_last: *mut *mut log_client,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct i3_shmlog_header {
    pub offset_next_write: u32,
    pub offset_last_wrap: u32,
    pub size: u32,
    pub wrap_count: u32,
}

pub const EINTR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const __S_IREAD: ::core::ffi::c_int = 0o400 as ::core::ffi::c_int;
pub const __S_IWRITE: ::core::ffi::c_int = 0o200 as ::core::ffi::c_int;
pub const S_IREAD: ::core::ffi::c_int = S_IRUSR;
pub const S_IWRITE: ::core::ffi::c_int = S_IWUSR;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const O_RDWR: ::core::ffi::c_int = 0o2 as ::core::ffi::c_int;
pub const O_CREAT: ::core::ffi::c_int = 0o100 as ::core::ffi::c_int;
pub const F_SETFD: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const FD_CLOEXEC: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const S_IRUSR: ::core::ffi::c_int = __S_IREAD;
pub const S_IWUSR: ::core::ffi::c_int = __S_IWRITE;
pub const PROT_READ: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const PROT_WRITE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const MAP_SHARED: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const MAP_FAILED: *mut ::core::ffi::c_void =
    -(1 as ::core::ffi::c_int) as *mut ::core::ffi::c_void;
static mut debug_logging: bool = false_0 != 0;
static mut verbose: bool = false_0 != 0;
static mut errorfile: *mut FILE = ::core::ptr::null::<FILE>() as *mut FILE;

pub static mut errorfilename: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;

pub static mut shmlogname: *mut ::core::ffi::c_char = c"".as_ptr().cast_mut();

pub static mut shmlog_size: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut logbuffer: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
static mut logwalk: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
static mut header: *mut i3_shmlog_header =
    ::core::ptr::null::<i3_shmlog_header>() as *mut i3_shmlog_header;
static mut loglastwrap: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
static mut logbuffer_size: ::core::ffi::c_int = 0;
static mut logbuffer_shm: ::core::ffi::c_int = 0;
static mut physical_mem_bytes: ::core::ffi::c_longlong = 0;

pub static mut log_clients: log_client_head = log_client_head {
    tqh_first: ::core::ptr::null::<log_client>() as *mut log_client,
    tqh_last: ::core::ptr::null::<*mut log_client>() as *mut *mut log_client,
};
unsafe fn store_log_markers() {
    (*header).offset_next_write = logwalk.offset_from(logbuffer) as ::core::ffi::c_long as u32;
    (*header).offset_last_wrap = loglastwrap.offset_from(logbuffer) as ::core::ffi::c_long as u32;
    (*header).size = logbuffer_size as u32;
}

pub unsafe fn init_logging() {
    if errorfilename.is_null() {
        errorfilename = get_process_filename(c"errorlog".as_ptr());
        if errorfilename.is_null() {
            fprintf(stderr, c"Could not initialize errorlog\n".as_ptr());
        } else {
            errorfile = fopen(errorfilename, c"w".as_ptr()) as *mut FILE;
            if errorfile.is_null() {
                fprintf(
                    stderr,
                    c"Could not initialize errorlog on %s: %s\n".as_ptr(),
                    errorfilename,
                    strerror(*__errno_location()),
                );
            } else if fcntl(fileno(errorfile), F_SETFD, FD_CLOEXEC) != 0 {
                fprintf(stderr, c"Could not set close-on-exec flag\n".as_ptr());
            }
        }
    }
    if physical_mem_bytes == 0 as ::core::ffi::c_longlong {
        physical_mem_bytes = sysconf(libc::_SC_PHYS_PAGES as ::core::ffi::c_int)
            as ::core::ffi::c_longlong
            * sysconf(libc::_SC_PAGESIZE as ::core::ffi::c_int) as ::core::ffi::c_longlong;
    }
    if shmlog_size > 0 as ::core::ffi::c_int && logbuffer.is_null() {
        open_logbuffer();
    } else if shmlog_size <= 0 as ::core::ffi::c_int && !logbuffer.is_null() {
        close_logbuffer();
    }
    atexit(purge_zerobyte_logfile);
}

pub unsafe fn open_logbuffer() {
    logbuffer_size = shmlog_size;
    if physical_mem_bytes as ::core::ffi::c_double * 0.01f64
        < shmlog_size as ::core::ffi::c_longlong as ::core::ffi::c_double
    {
        logbuffer_size =
            (physical_mem_bytes as ::core::ffi::c_double * 0.01f64) as ::core::ffi::c_int;
    }
    sasprintf(&raw mut shmlogname, c"/i3-log-%d".as_ptr(), getpid());
    logbuffer_shm = shm_open(
        shmlogname,
        O_RDWR | O_CREAT,
        (S_IREAD | S_IWRITE) as libc::mode_t,
    );
    if logbuffer_shm == -(1 as ::core::ffi::c_int) {
        fprintf(
            stderr,
            c"Could not shm_open SHM segment for the i3 log: %s\n".as_ptr(),
            strerror(*__errno_location()),
        );
        return;
    }
    let mut ret: ::core::ffi::c_int = 0;
    ret = posix_fallocate(
        logbuffer_shm,
        0 as libc::off64_t,
        logbuffer_size as libc::off64_t,
    );
    if ret != 0 as ::core::ffi::c_int {
        fprintf(
            stderr,
            c"Could not ftruncate SHM segment for the i3 log: %s\n".as_ptr(),
            strerror(ret),
        );
        close(logbuffer_shm);
        shm_unlink(shmlogname);
        return;
    }
    logbuffer = mmap(
        NULL,
        logbuffer_size as usize,
        PROT_READ | PROT_WRITE,
        MAP_SHARED,
        logbuffer_shm,
        0 as libc::off64_t,
    ) as *mut ::core::ffi::c_char;
    if logbuffer == MAP_FAILED as *mut ::core::ffi::c_char {
        close_logbuffer();
        fprintf(
            stderr,
            c"Could not mmap SHM segment for the i3 log: %s\n".as_ptr(),
            strerror(*__errno_location()),
        );
        return;
    }
    memset(
        logbuffer as *mut ::core::ffi::c_void,
        '\0' as i32,
        logbuffer_size as usize,
    );
    header = logbuffer as *mut i3_shmlog_header;
    logwalk = logbuffer.offset(::core::mem::size_of::<i3_shmlog_header>() as usize as isize);
    loglastwrap = logbuffer.offset(logbuffer_size as isize);
    store_log_markers();
}

pub unsafe fn close_logbuffer() {
    close(logbuffer_shm);
    shm_unlink(shmlogname);
    free(shmlogname as *mut ::core::ffi::c_void);
    logbuffer = ::core::ptr::null_mut::<::core::ffi::c_char>();
    shmlogname = c"".as_ptr().cast_mut();
}

pub unsafe fn set_verbosity(mut _verbose: bool) {
    verbose = _verbose;
}

pub unsafe fn get_debug_logging() -> bool {
    debug_logging
}

pub unsafe fn set_debug_logging(_debug_logging: bool) {
    debug_logging = _debug_logging;
}
unsafe fn vlog(print: bool, fmt: *const ::core::ffi::c_char, mut args: ::core::ffi::VaList) {
    static mut message: [::core::ffi::c_char; 4096] = [0; 4096];
    static mut result: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: ::core::ptr::null::<::core::ffi::c_char>(),
    };
    static mut t: libc::time_t = 0;
    static mut tmp: *mut tm = ::core::ptr::null::<tm>() as *mut tm;
    static mut len: usize = 0;
    t = time(::core::ptr::null_mut::<libc::time_t>());
    tmp = localtime_r(&raw mut t, &raw mut result);
    len = strftime(
        &raw mut message as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as usize,
        c"%x %X - ".as_ptr(),
        tmp,
    );
    if logbuffer.is_null() {
        printf(c"%s".as_ptr(), &raw mut message as *mut ::core::ffi::c_char);
        vprintf(fmt, i333_va_list!(args));
    } else {
        len = len.wrapping_add(vsnprintf(
            (&raw mut message as *mut ::core::ffi::c_char).offset(len as isize),
            (::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as usize).wrapping_sub(len),
            fmt,
            i333_va_list!(args),
        ) as usize);
        if len >= ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as usize {
            fprintf(stderr, c"BUG: single log message > 4k\n".as_ptr());
            len = ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as usize as usize;
            message[len.wrapping_sub(2 as usize)] = '\n' as i32 as ::core::ffi::c_char;
        }
        if len
            >= (logbuffer_size as ::core::ffi::c_long
                - logwalk.offset_from(logbuffer) as ::core::ffi::c_long) as usize
        {
            loglastwrap = logwalk;
            logwalk =
                logbuffer.offset(::core::mem::size_of::<i3_shmlog_header>() as usize as isize);
            store_log_markers();
            (*header).wrap_count = (*header).wrap_count.wrapping_add(1);
        }
        strncpy(logwalk, &raw mut message as *mut ::core::ffi::c_char, len);
        logwalk = logwalk.offset(len as isize);
        store_log_markers();
        if print {
            fwrite(
                &raw mut message as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                len,
                1 as usize,
                stdout,
            );
        }
        log_broadcast_to_clients(&raw mut message as *mut ::core::ffi::c_char, len);
    };
}

pub unsafe extern "C" fn verboselog(fmt: *mut ::core::ffi::c_char, args: ...) {
    let mut args_0;
    if logbuffer.is_null() && !verbose {
        return;
    }
    args_0 = args.clone();
    vlog(verbose, fmt, i333_va_list!(args_0));
}

pub unsafe extern "C" fn errorlog(fmt: *mut ::core::ffi::c_char, args: ...) {
    let mut args_0;
    args_0 = args.clone();
    vlog(true_0 != 0, fmt, i333_va_list!(args_0));
    if errorfile.is_null() {
        return;
    }
    args_0 = args.clone();
    vfprintf(errorfile, fmt, i333_va_list!(args_0));
    fflush(errorfile);
}

pub unsafe extern "C" fn debuglog(fmt: *mut ::core::ffi::c_char, args: ...) {
    let mut args_0;
    if logbuffer.is_null() && !debug_logging {
        return;
    }
    args_0 = args.clone();
    vlog(debug_logging, fmt, i333_va_list!(args_0));
}

pub extern "C" fn purge_zerobyte_logfile() {
    unsafe {
        let mut st: stat = ::core::mem::zeroed();
        let mut slash: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        if errorfilename.is_null() {
            return;
        }
        if stat(errorfilename, &raw mut st) == -(1 as ::core::ffi::c_int)
            || st.st_size > 0 as libc::off_t
        {
            return;
        }
        if unlink(errorfilename) == -(1 as ::core::ffi::c_int) {
            return;
        }
        slash = strrchr(errorfilename, '/' as i32);
        if !slash.is_null() {
            *slash = '\0' as i32 as ::core::ffi::c_char;
            rmdir(errorfilename);
        }
    }
}

pub static mut current_log_stream_socket_path: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;

pub unsafe extern "C" fn log_new_client(
    loop_0: *mut ev_loop,
    w: *mut ev_io,
    revents: ::core::ffi::c_int,
) {
    let mut peer: sockaddr_un = sockaddr_un {
        sun_family: 0,
        sun_path: [0; 108],
    };
    let mut len: libc::socklen_t = ::core::mem::size_of::<sockaddr_un>() as libc::socklen_t;
    let mut fd: ::core::ffi::c_int = 0;
    fd = accept((*w).fd, &raw mut peer as *mut sockaddr, &raw mut len);
    if fd < 0 as ::core::ffi::c_int {
        if *__errno_location() != EINTR {
            perror(c"accept()".as_ptr());
        }
        return;
    }
    fcntl(fd, F_SETFD, FD_CLOEXEC);
    set_nonblock(fd);
    let client: *mut log_client =
        scalloc(1 as usize, ::core::mem::size_of::<log_client>() as usize) as *mut log_client;
    (*client).fd = fd;
    (*client).clients.tqe_next = ::core::ptr::null_mut::<log_client>();
    (*client).clients.tqe_prev = log_clients.tqh_last;
    *log_clients.tqh_last = client as *mut log_client;
    log_clients.tqh_last = &raw mut (*client).clients.tqe_next;
    debuglog(
        c"%s:%s:%d - log: new client connected on fd %d\n"
            .as_ptr()
            .cast_mut(),
        c"../src/log.c".as_ptr(),
        c"log_new_client".as_ptr(),
        419 as ::core::ffi::c_int,
        fd,
    );
}

pub unsafe fn log_broadcast_to_clients(message: *const ::core::ffi::c_char, len: usize) {
    let mut current: *mut log_client = log_clients.tqh_first as *mut log_client;
    while !current.is_null() {
        let n: isize = writeall((*current).fd, message as *const ::core::ffi::c_void, len);
        let previous: *mut log_client = current;
        current = (*current).clients.tqe_next as *mut log_client;
        if n < 0 as isize {
            if !(*previous).clients.tqe_next.is_null() {
                (*(*previous).clients.tqe_next).clients.tqe_prev = (*previous).clients.tqe_prev;
            } else {
                log_clients.tqh_last = (*previous).clients.tqe_prev;
            }
            *(*previous).clients.tqe_prev = (*previous).clients.tqe_next;
            free(previous as *mut ::core::ffi::c_void);
        }
    }
}
unsafe extern "C" fn run_static_initializers() {
    log_clients = log_client_head {
        tqh_first: ::core::ptr::null_mut::<log_client>(),
        tqh_last: &raw mut log_clients.tqh_first,
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];

// Internal helpers use Rust linkage rather than foreign symbol lookup.
use i3_common::safewrappers::scalloc;
use i3_common::safewrappers::writeall;

pub use libc::{
    __errno_location, accept, atexit, close, fcntl, fflush, fileno, fopen, fprintf, free, fwrite,
    getpid, localtime_r, memset, mmap, perror, posix_fallocate, printf, rmdir, shm_open,
    shm_unlink, sockaddr, sockaddr_un, stat, strerror, strftime, strncpy, strrchr, sysconf, time,
    tm, unlink, FILE,
};

use i3_common::native_stdio::{stderr, stdout, vfprintf, vprintf, vsnprintf};

// Internal calls and globals use canonical Rust identities.
use i3_common::get_process_filename::get_process_filename;
use i3_common::nonblock::set_nonblock;
