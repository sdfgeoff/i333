use crate::config::included_files;
use i3_common::safewrappers::sasprintf;
/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * display_version.c: displays the running i3 version, runs as part of
 *                    i3 --moreversion.
 *
 */
// Mechanical file-by-file Rust translation of src/display_version.c.
// Originally translated with C2Rust 0.22.1; subsequently maintained in Rust.
use i3_json::compat::{yajl_alloc, yajl_free, yajl_parse};
pub use i3_json::compat::{
    yajl_alloc_funcs, yajl_callbacks, yajl_handle, yajl_status, yajl_status_ok,
};
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint32_t = u32;
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
pub type __blksize_t = ::core::ffi::c_long;
pub type __blkcnt_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type ssize_t = isize;
pub type time_t = __time_t;

pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;

pub use crate::config::IncludedFile;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const I3_IPC_MESSAGE_TYPE_GET_VERSION: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut human_readable_key: bool = false;
static mut loaded_config_file_name_key: bool = false;
static mut included_config_file_names: bool = false;
static mut human_readable_version: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
static mut loaded_config_file_name: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
unsafe fn version_string(
    ctx: *mut ::core::ffi::c_void,
    val: *const ::core::ffi::c_uchar,
    len: size_t,
) -> ::core::ffi::c_int {
    if human_readable_key {
        sasprintf(
            &raw mut human_readable_version,
            c"%.*s".as_ptr(),
            len as ::core::ffi::c_int,
            val,
        );
    }
    if loaded_config_file_name_key {
        sasprintf(
            &raw mut loaded_config_file_name,
            c"%.*s".as_ptr(),
            len as ::core::ffi::c_int,
            val,
        );
    }
    if included_config_file_names {
        let file: *mut IncludedFile = scalloc(
            1 as size_t,
            ::core::mem::size_of::<IncludedFile>() as size_t,
        ) as *mut IncludedFile;
        sasprintf(
            &raw mut (*file).path,
            c"%.*s".as_ptr(),
            len as ::core::ffi::c_int,
            val,
        );
        (*file).files.tqe_next = ::core::ptr::null_mut::<IncludedFile>();
        (*file).files.tqe_prev = included_files.tqh_last;
        *included_files.tqh_last = file as *mut IncludedFile;
        included_files.tqh_last = &raw mut (*file).files.tqe_next;
    }
    1 as ::core::ffi::c_int
}
unsafe fn version_map_key(
    ctx: *mut ::core::ffi::c_void,
    stringval: *const ::core::ffi::c_uchar,
    stringlen: size_t,
) -> ::core::ffi::c_int {
    human_readable_key = stringlen == strlen(c"human_readable".as_ptr())
        && strncmp(
            stringval as *const ::core::ffi::c_char,
            c"human_readable".as_ptr(),
            strlen(c"human_readable".as_ptr()),
        ) == 0 as ::core::ffi::c_int;
    loaded_config_file_name_key = stringlen == strlen(c"loaded_config_file_name".as_ptr())
        && strncmp(
            stringval as *const ::core::ffi::c_char,
            c"loaded_config_file_name".as_ptr(),
            strlen(c"loaded_config_file_name".as_ptr()),
        ) == 0 as ::core::ffi::c_int;
    included_config_file_names = stringlen == strlen(c"included_config_file_names".as_ptr())
        && strncmp(
            stringval as *const ::core::ffi::c_char,
            c"included_config_file_names".as_ptr(),
            strlen(c"included_config_file_names".as_ptr()),
        ) == 0 as ::core::ffi::c_int;
    1 as ::core::ffi::c_int
}
static mut version_callbacks: yajl_callbacks = unsafe {
    yajl_callbacks {
        yajl_null: None,
        yajl_boolean: None,
        yajl_integer: None,
        yajl_double: None,
        yajl_number: None,
        yajl_string: Some(
            version_string
                as unsafe fn(
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_uchar,
                    size_t,
                ) -> ::core::ffi::c_int,
        ),
        yajl_start_map: None,
        yajl_map_key: Some(
            version_map_key
                as unsafe fn(
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_uchar,
                    size_t,
                ) -> ::core::ffi::c_int,
        ),
        yajl_end_map: None,
        yajl_start_array: None,
        yajl_end_array: None,
    }
};
unsafe fn print_config_path(path: *const ::core::ffi::c_char, role: *const ::core::ffi::c_char) {
    let mut sb: stat = ::core::mem::zeroed();
    let mut now: time_t = 0;
    let mut mtime: [::core::ffi::c_char; 64] = [0; 64];
    printf(c"  %s (%s)".as_ptr(), path, role);
    if stat(path, &raw mut sb) == -(1 as ::core::ffi::c_int) {
        printf(c"\n".as_ptr());
        errorlog(
            c"ERROR: Cannot stat config file \"%s\"\n"
                .as_ptr()
                .cast_mut(),
            path,
        );
    } else {
        strftime(
            &raw mut mtime as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>() as size_t,
            c"%c".as_ptr(),
            localtime(&raw mut sb.st_mtime),
        );
        time(&raw mut now);
        printf(
            c" (last modified: %s, %.f seconds ago)\n".as_ptr(),
            &raw mut mtime as *mut ::core::ffi::c_char,
            difftime(now, sb.st_mtime as time_t),
        );
    };
}

pub unsafe fn display_running_version() {
    if getenv(c"DISPLAY".as_ptr()).is_null() {
        fprintf(
            stderr,
            c"\nYour DISPLAY environment variable is not set.\n".as_ptr(),
        );
        fprintf(
            stderr,
            c"Are you running i3 via SSH or on a virtual console?\n".as_ptr(),
        );
        fprintf(stderr, c"Try DISPLAY=:0 i3 --moreversion\n".as_ptr());
        exit(EXIT_FAILURE);
    }
    let pid_from_atom: *mut ::core::ffi::c_char =
        root_atom_contents(c"I3_PID".as_ptr(), conn, conn_screen);
    if pid_from_atom.is_null() {
        printf(c"\nRunning version: < 4.2-200\n".as_ptr());
        exit(EXIT_SUCCESS);
    }
    printf(c"(Getting version from running i3, press ctrl-c to abort…)".as_ptr());
    fflush(stdout);
    let sockfd: ::core::ffi::c_int = ipc_connect(::core::ptr::null::<::core::ffi::c_char>());
    if ipc_send_message(
        sockfd,
        0 as uint32_t,
        I3_IPC_MESSAGE_TYPE_GET_VERSION as uint32_t,
        c"".as_ptr() as *mut uint8_t,
    ) == -(1 as ::core::ffi::c_int)
    {
        err(EXIT_FAILURE, c"IPC: write()".as_ptr());
    }
    let mut reply_length: uint32_t = 0;
    let mut reply_type: uint32_t = 0;
    let mut reply: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut ret: ::core::ffi::c_int = 0;
    ret = ipc_recv_message(
        sockfd,
        &raw mut reply_type,
        &raw mut reply_length,
        &raw mut reply,
    );
    if ret != 0 as ::core::ffi::c_int {
        if ret == -(1 as ::core::ffi::c_int) {
            err(EXIT_FAILURE, c"IPC: read()".as_ptr());
        }
        exit(EXIT_FAILURE);
    }
    if reply_type != I3_IPC_MESSAGE_TYPE_GET_VERSION as uint32_t {
        errx(
            EXIT_FAILURE,
            c"Got reply type %d, but expected %d (GET_VERSION)".as_ptr(),
            reply_type,
            I3_IPC_MESSAGE_TYPE_GET_VERSION,
        );
    }
    let handle: yajl_handle = yajl_alloc(
        &raw mut version_callbacks,
        ::core::ptr::null_mut::<yajl_alloc_funcs>(),
        NULL,
    );
    let state: yajl_status =
        yajl_parse(handle, reply, reply_length as ::core::ffi::c_int as size_t);
    if state as ::core::ffi::c_uint != yajl_status_ok as ::core::ffi::c_int as ::core::ffi::c_uint {
        errx(
            EXIT_FAILURE,
            c"Could not parse my own reply. That's weird. reply is %.*s".as_ptr(),
            reply_length as ::core::ffi::c_int,
            reply,
        );
    }
    printf(c"\r\u{1b}[K".as_ptr());
    printf(
        c"Running i3 version: %s (pid %s)\n".as_ptr(),
        human_readable_version,
        pid_from_atom,
    );
    if !loaded_config_file_name.is_null() {
        printf(c"Loaded i3 config:\n".as_ptr());
        print_config_path(loaded_config_file_name, c"main".as_ptr());
        let mut file: *mut IncludedFile = ::core::ptr::null_mut::<IncludedFile>();
        file = included_files.tqh_first as *mut IncludedFile;
        while !file.is_null() {
            print_config_path((*file).path, c"included".as_ptr());
            file = (*file).files.tqe_next as *mut IncludedFile;
        }
    }
    let mut destpath_size: size_t = 1024 as size_t;
    let mut linksize: ssize_t = 0;
    let mut exepath: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut destpath: *mut ::core::ffi::c_char = smalloc(destpath_size) as *mut ::core::ffi::c_char;
    sasprintf(&raw mut exepath, c"/proc/%d/exe".as_ptr(), getpid());
    loop {
        linksize = readlink(exepath, destpath, destpath_size);
        if !(linksize == destpath_size as ssize_t) {
            break;
        }
        destpath_size = destpath_size.wrapping_mul(2 as size_t);
        destpath = srealloc(destpath as *mut ::core::ffi::c_void, destpath_size)
            as *mut ::core::ffi::c_char;
    }
    if linksize == -(1 as ::core::ffi::c_int) as ssize_t {
        err(EXIT_FAILURE, c"readlink(%s)".as_ptr(), exepath);
    }
    *destpath.offset(linksize as isize) = '\0' as i32 as ::core::ffi::c_char;
    printf(c"\n".as_ptr());
    printf(c"The i3 binary you just called: %s\n".as_ptr(), destpath);
    free(exepath as *mut ::core::ffi::c_void);
    sasprintf(&raw mut exepath, c"/proc/%s/exe".as_ptr(), pid_from_atom);
    loop {
        linksize = readlink(exepath, destpath, destpath_size);
        if !(linksize == destpath_size as ssize_t) {
            break;
        }
        destpath_size = destpath_size.wrapping_mul(2 as size_t);
        destpath = srealloc(destpath as *mut ::core::ffi::c_void, destpath_size)
            as *mut ::core::ffi::c_char;
    }
    if linksize == -(1 as ::core::ffi::c_int) as ssize_t {
        err(EXIT_FAILURE, c"readlink(%s)".as_ptr(), exepath);
    }
    *destpath.offset(linksize as isize) = '\0' as i32 as ::core::ffi::c_char;
    if !strstr(destpath, c"(deleted)".as_ptr()).is_null() {
        printf(c"RUNNING BINARY DIFFERENT FROM BINARY ON DISK!\n".as_ptr());
    }
    free(exepath as *mut ::core::ffi::c_void);
    sasprintf(
        &raw mut exepath,
        c"/proc/%s/cmdline".as_ptr(),
        pid_from_atom,
    );
    let mut fd: ::core::ffi::c_int = 0;
    fd = open(exepath, O_RDONLY);
    if fd == -(1 as ::core::ffi::c_int) {
        err(EXIT_FAILURE, c"open(%s)".as_ptr(), exepath);
    }
    if read(
        fd,
        destpath as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t,
    ) == -(1 as ::core::ffi::c_int) as ssize_t
    {
        err(EXIT_FAILURE, c"read(%s)".as_ptr(), exepath);
    }
    close(fd);
    printf(c"The i3 binary you are running: %s\n".as_ptr(), destpath);
    free(exepath as *mut ::core::ffi::c_void);
    free(destpath as *mut ::core::ffi::c_void);
    yajl_free(handle);
    free(reply as *mut ::core::ffi::c_void);
    free(pid_from_atom as *mut ::core::ffi::c_void);
}
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const EXIT_SUCCESS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

// Internal helpers use Rust linkage rather than foreign symbol lookup.
use i3_common::safewrappers::scalloc;
use i3_common::safewrappers::smalloc;
use i3_common::safewrappers::srealloc;

pub use libc::{
    close, difftime, exit, fflush, fprintf, free, getenv, getpid, localtime, open, printf, read,
    readlink, stat, strftime, strlen, strncmp, strstr, time,
};

use i3_common::native_stdio::{err, errx, stderr, stdout};

use crate::log::errorlog;

// Internal calls and globals use canonical Rust identities.
use crate::application::conn;
use crate::application::conn_screen;
use i3_common::ipc_connect::ipc_connect;
use i3_common::ipc_recv_message::ipc_recv_message;
use i3_common::ipc_send_message::ipc_send_message;
use i3_common::root_atom_contents::root_atom_contents;
