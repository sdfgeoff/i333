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
// Generated with C2Rust 0.22.1; C ABI and original ownership retained.
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type xcb_connection_t;
    pub type yajl_handle_t;
    fn err(__status: ::core::ffi::c_int, __format: *const ::core::ffi::c_char, ...) -> !;
    fn errx(__status: ::core::ffi::c_int, __format: *const ::core::ffi::c_char, ...) -> !;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn stat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    fn time(__timer: *mut time_t) -> time_t;
    fn difftime(__time1: time_t, __time0: time_t) -> ::core::ffi::c_double;
    fn strftime(
        __s: *mut ::core::ffi::c_char,
        __maxsize: size_t,
        __format: *const ::core::ffi::c_char,
        __tp: *const tm,
    ) -> size_t;
    fn localtime(__timer: *const time_t) -> *mut tm;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn read(__fd: ::core::ffi::c_int, __buf: *mut ::core::ffi::c_void, __nbytes: size_t)
        -> ssize_t;
    fn getpid() -> __pid_t;
    fn readlink(
        __path: *const ::core::ffi::c_char,
        __buf: *mut ::core::ffi::c_char,
        __len: size_t,
    ) -> ssize_t;
    static mut conn: *mut xcb_connection_t;
    fn errorlog(fmt: *mut ::core::ffi::c_char, ...);
    fn root_atom_contents(
        atomname: *const ::core::ffi::c_char,
        provided_conn: *mut xcb_connection_t,
        screen: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn smalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn scalloc(num: size_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn srealloc(ptr: *mut ::core::ffi::c_void, size: size_t) -> *mut ::core::ffi::c_void;
    fn sasprintf(
        strp: *mut *mut ::core::ffi::c_char,
        fmt: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn ipc_connect(socket_path: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn ipc_send_message(
        sockfd: ::core::ffi::c_int,
        message_size: uint32_t,
        message_type: uint32_t,
        payload: *const uint8_t,
    ) -> ::core::ffi::c_int;
    fn ipc_recv_message(
        sockfd: ::core::ffi::c_int,
        message_type: *mut uint32_t,
        reply_length: *mut uint32_t,
        reply: *mut *mut uint8_t,
    ) -> ::core::ffi::c_int;
    fn yajl_alloc(
        callbacks: *const yajl_callbacks,
        afs: *mut yajl_alloc_funcs,
        ctx: *mut ::core::ffi::c_void,
    ) -> yajl_handle;
    fn yajl_free(handle: yajl_handle);
    fn yajl_parse(
        hand: yajl_handle,
        jsonText: *const ::core::ffi::c_uchar,
        jsonTextLength: size_t,
    ) -> yajl_status;
    static mut conn_screen: ::core::ffi::c_int;
    static mut included_files: includedfiles_head;
    fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
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
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn getenv(__name: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: ::core::ffi::c_int,
    pub tm_min: ::core::ffi::c_int,
    pub tm_hour: ::core::ffi::c_int,
    pub tm_mday: ::core::ffi::c_int,
    pub tm_mon: ::core::ffi::c_int,
    pub tm_year: ::core::ffi::c_int,
    pub tm_wday: ::core::ffi::c_int,
    pub tm_yday: ::core::ffi::c_int,
    pub tm_isdst: ::core::ffi::c_int,
    pub tm_gmtoff: ::core::ffi::c_long,
    pub tm_zone: *const ::core::ffi::c_char,
}
pub type uint8_t = __uint8_t;
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
pub type yajl_malloc_func =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> *mut ::core::ffi::c_void>;
pub type yajl_free_func =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> ()>;
pub type yajl_realloc_func = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
        size_t,
    ) -> *mut ::core::ffi::c_void,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yajl_alloc_funcs {
    pub malloc: yajl_malloc_func,
    pub realloc: yajl_realloc_func,
    pub free: yajl_free_func,
    pub ctx: *mut ::core::ffi::c_void,
}
pub type yajl_status = ::core::ffi::c_uint;
pub const yajl_status_error: yajl_status = 2;
pub const yajl_status_client_canceled: yajl_status = 1;
pub const yajl_status_ok: yajl_status = 0;
pub type yajl_handle = *mut yajl_handle_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IncludedFile {
    pub path: *mut ::core::ffi::c_char,
    pub raw_contents: *mut ::core::ffi::c_char,
    pub variable_replaced_contents: *mut ::core::ffi::c_char,
    pub files: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub tqe_next: *mut IncludedFile,
    pub tqe_prev: *mut *mut IncludedFile,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct includedfiles_head {
    pub tqh_first: *mut IncludedFile,
    pub tqh_last: *mut *mut IncludedFile,
}
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
unsafe extern "C" fn version_string(
    mut ctx: *mut ::core::ffi::c_void,
    mut val: *const ::core::ffi::c_uchar,
    mut len: size_t,
) -> ::core::ffi::c_int {
    if human_readable_key {
        sasprintf(
            &raw mut human_readable_version,
            b"%.*s\0" as *const u8 as *const ::core::ffi::c_char,
            len as ::core::ffi::c_int,
            val,
        );
    }
    if loaded_config_file_name_key {
        sasprintf(
            &raw mut loaded_config_file_name,
            b"%.*s\0" as *const u8 as *const ::core::ffi::c_char,
            len as ::core::ffi::c_int,
            val,
        );
    }
    if included_config_file_names {
        let mut file: *mut IncludedFile = scalloc(
            1 as size_t,
            ::core::mem::size_of::<IncludedFile>() as size_t,
        ) as *mut IncludedFile;
        sasprintf(
            &raw mut (*file).path,
            b"%.*s\0" as *const u8 as *const ::core::ffi::c_char,
            len as ::core::ffi::c_int,
            val,
        );
        (*file).files.tqe_next = ::core::ptr::null_mut::<IncludedFile>();
        (*file).files.tqe_prev = included_files.tqh_last;
        *included_files.tqh_last = file as *mut IncludedFile;
        included_files.tqh_last = &raw mut (*file).files.tqe_next;
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn version_map_key(
    mut ctx: *mut ::core::ffi::c_void,
    mut stringval: *const ::core::ffi::c_uchar,
    mut stringlen: size_t,
) -> ::core::ffi::c_int {
    human_readable_key = stringlen
        == strlen(b"human_readable\0" as *const u8 as *const ::core::ffi::c_char)
        && strncmp(
            stringval as *const ::core::ffi::c_char,
            b"human_readable\0" as *const u8 as *const ::core::ffi::c_char,
            strlen(b"human_readable\0" as *const u8 as *const ::core::ffi::c_char),
        ) == 0 as ::core::ffi::c_int;
    loaded_config_file_name_key = stringlen
        == strlen(b"loaded_config_file_name\0" as *const u8 as *const ::core::ffi::c_char)
        && strncmp(
            stringval as *const ::core::ffi::c_char,
            b"loaded_config_file_name\0" as *const u8 as *const ::core::ffi::c_char,
            strlen(b"loaded_config_file_name\0" as *const u8 as *const ::core::ffi::c_char),
        ) == 0 as ::core::ffi::c_int;
    included_config_file_names = stringlen
        == strlen(b"included_config_file_names\0" as *const u8 as *const ::core::ffi::c_char)
        && strncmp(
            stringval as *const ::core::ffi::c_char,
            b"included_config_file_names\0" as *const u8 as *const ::core::ffi::c_char,
            strlen(b"included_config_file_names\0" as *const u8 as *const ::core::ffi::c_char),
        ) == 0 as ::core::ffi::c_int;
    return 1 as ::core::ffi::c_int;
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
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_uchar,
                    size_t,
                ) -> ::core::ffi::c_int,
        ),
        yajl_start_map: None,
        yajl_map_key: Some(
            version_map_key
                as unsafe extern "C" fn(
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
unsafe extern "C" fn print_config_path(
    mut path: *const ::core::ffi::c_char,
    mut role: *const ::core::ffi::c_char,
) {
    let mut sb: stat = stat {
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
    let mut now: time_t = 0;
    let mut mtime: [::core::ffi::c_char; 64] = [0; 64];
    printf(
        b"  %s (%s)\0" as *const u8 as *const ::core::ffi::c_char,
        path,
        role,
    );
    if stat(path, &raw mut sb) == -(1 as ::core::ffi::c_int) {
        printf(b"\n\0" as *const u8 as *const ::core::ffi::c_char);
        errorlog(
            b"ERROR: Cannot stat config file \"%s\"\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            path,
        );
    } else {
        strftime(
            &raw mut mtime as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>() as size_t,
            b"%c\0" as *const u8 as *const ::core::ffi::c_char,
            localtime(&raw mut sb.st_mtim.tv_sec),
        );
        time(&raw mut now);
        printf(
            b" (last modified: %s, %.f seconds ago)\n\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut mtime as *mut ::core::ffi::c_char,
            difftime(now, sb.st_mtim.tv_sec as time_t),
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn display_running_version() {
    if getenv(b"DISPLAY\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
        fprintf(
            stderr,
            b"\nYour DISPLAY environment variable is not set.\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        fprintf(
            stderr,
            b"Are you running i3 via SSH or on a virtual console?\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        fprintf(
            stderr,
            b"Try DISPLAY=:0 i3 --moreversion\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
        exit(EXIT_FAILURE);
    }
    let mut pid_from_atom: *mut ::core::ffi::c_char = root_atom_contents(
        b"I3_PID\0" as *const u8 as *const ::core::ffi::c_char,
        conn,
        conn_screen,
    );
    if pid_from_atom.is_null() {
        printf(b"\nRunning version: < 4.2-200\n\0" as *const u8 as *const ::core::ffi::c_char);
        exit(EXIT_SUCCESS);
    }
    printf(
        b"(Getting version from running i3, press ctrl-c to abort\xE2\x80\xA6)\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    fflush(stdout);
    let mut sockfd: ::core::ffi::c_int = ipc_connect(::core::ptr::null::<::core::ffi::c_char>());
    if ipc_send_message(
        sockfd,
        0 as uint32_t,
        I3_IPC_MESSAGE_TYPE_GET_VERSION as uint32_t,
        b"\0" as *const u8 as *const ::core::ffi::c_char as *mut uint8_t,
    ) == -(1 as ::core::ffi::c_int)
    {
        err(
            EXIT_FAILURE,
            b"IPC: write()\0" as *const u8 as *const ::core::ffi::c_char,
        );
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
            err(
                EXIT_FAILURE,
                b"IPC: read()\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        exit(EXIT_FAILURE);
    }
    if reply_type != I3_IPC_MESSAGE_TYPE_GET_VERSION as uint32_t {
        errx(
            EXIT_FAILURE,
            b"Got reply type %d, but expected %d (GET_VERSION)\0" as *const u8
                as *const ::core::ffi::c_char,
            reply_type,
            I3_IPC_MESSAGE_TYPE_GET_VERSION,
        );
    }
    let mut handle: yajl_handle = yajl_alloc(
        &raw mut version_callbacks,
        ::core::ptr::null_mut::<yajl_alloc_funcs>(),
        NULL,
    );
    let mut state: yajl_status =
        yajl_parse(handle, reply, reply_length as ::core::ffi::c_int as size_t);
    if state as ::core::ffi::c_uint != yajl_status_ok as ::core::ffi::c_int as ::core::ffi::c_uint {
        errx(
            EXIT_FAILURE,
            b"Could not parse my own reply. That's weird. reply is %.*s\0" as *const u8
                as *const ::core::ffi::c_char,
            reply_length as ::core::ffi::c_int,
            reply,
        );
    }
    printf(b"\r\x1B[K\0" as *const u8 as *const ::core::ffi::c_char);
    printf(
        b"Running i3 version: %s (pid %s)\n\0" as *const u8 as *const ::core::ffi::c_char,
        human_readable_version,
        pid_from_atom,
    );
    if !loaded_config_file_name.is_null() {
        printf(b"Loaded i3 config:\n\0" as *const u8 as *const ::core::ffi::c_char);
        print_config_path(
            loaded_config_file_name,
            b"main\0" as *const u8 as *const ::core::ffi::c_char,
        );
        let mut file: *mut IncludedFile = ::core::ptr::null_mut::<IncludedFile>();
        file = included_files.tqh_first as *mut IncludedFile;
        while !file.is_null() {
            print_config_path(
                (*file).path,
                b"included\0" as *const u8 as *const ::core::ffi::c_char,
            );
            file = (*file).files.tqe_next as *mut IncludedFile;
        }
    }
    let mut destpath_size: size_t = 1024 as size_t;
    let mut linksize: ssize_t = 0;
    let mut exepath: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut destpath: *mut ::core::ffi::c_char = smalloc(destpath_size) as *mut ::core::ffi::c_char;
    sasprintf(
        &raw mut exepath,
        b"/proc/%d/exe\0" as *const u8 as *const ::core::ffi::c_char,
        getpid(),
    );
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
        err(
            EXIT_FAILURE,
            b"readlink(%s)\0" as *const u8 as *const ::core::ffi::c_char,
            exepath,
        );
    }
    *destpath.offset(linksize as isize) = '\0' as i32 as ::core::ffi::c_char;
    printf(b"\n\0" as *const u8 as *const ::core::ffi::c_char);
    printf(
        b"The i3 binary you just called: %s\n\0" as *const u8 as *const ::core::ffi::c_char,
        destpath,
    );
    free(exepath as *mut ::core::ffi::c_void);
    sasprintf(
        &raw mut exepath,
        b"/proc/%s/exe\0" as *const u8 as *const ::core::ffi::c_char,
        pid_from_atom,
    );
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
        err(
            EXIT_FAILURE,
            b"readlink(%s)\0" as *const u8 as *const ::core::ffi::c_char,
            exepath,
        );
    }
    *destpath.offset(linksize as isize) = '\0' as i32 as ::core::ffi::c_char;
    if !strstr(
        destpath,
        b"(deleted)\0" as *const u8 as *const ::core::ffi::c_char,
    )
    .is_null()
    {
        printf(
            b"RUNNING BINARY DIFFERENT FROM BINARY ON DISK!\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    free(exepath as *mut ::core::ffi::c_void);
    sasprintf(
        &raw mut exepath,
        b"/proc/%s/cmdline\0" as *const u8 as *const ::core::ffi::c_char,
        pid_from_atom,
    );
    let mut fd: ::core::ffi::c_int = 0;
    fd = open(exepath, O_RDONLY);
    if fd == -(1 as ::core::ffi::c_int) {
        err(
            EXIT_FAILURE,
            b"open(%s)\0" as *const u8 as *const ::core::ffi::c_char,
            exepath,
        );
    }
    if read(
        fd,
        destpath as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t,
    ) == -(1 as ::core::ffi::c_int) as ssize_t
    {
        err(
            EXIT_FAILURE,
            b"read(%s)\0" as *const u8 as *const ::core::ffi::c_char,
            exepath,
        );
    }
    close(fd);
    printf(
        b"The i3 binary you are running: %s\n\0" as *const u8 as *const ::core::ffi::c_char,
        destpath,
    );
    free(exepath as *mut ::core::ffi::c_void);
    free(destpath as *mut ::core::ffi::c_void);
    yajl_free(handle);
    free(reply as *mut ::core::ffi::c_void);
    free(pid_from_atom as *mut ::core::ffi::c_void);
}
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const EXIT_SUCCESS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
