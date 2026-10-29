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

pub type __builtin_va_list = [__va_list_tag; 1];
pub use i3_common::safewrappers::__va_list_tag;
pub type size_t = usize;
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
pub type __socklen_t = ::core::ffi::c_uint;
pub type mode_t = __mode_t;
pub type ssize_t = isize;
pub type time_t = __time_t;

pub type __gnuc_va_list = __builtin_va_list;
pub type va_list = __gnuc_va_list;

pub type uint32_t = __uint32_t;

pub type socklen_t = __socklen_t;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const _SC_SIGSTKSZ: C2RustUnnamed = 250;
pub const _SC_MINSIGSTKSZ: C2RustUnnamed = 249;
pub const _SC_THREAD_ROBUST_PRIO_PROTECT: C2RustUnnamed = 248;
pub const _SC_THREAD_ROBUST_PRIO_INHERIT: C2RustUnnamed = 247;
pub const _SC_XOPEN_STREAMS: C2RustUnnamed = 246;
pub const _SC_TRACE_USER_EVENT_MAX: C2RustUnnamed = 245;
pub const _SC_TRACE_SYS_MAX: C2RustUnnamed = 244;
pub const _SC_TRACE_NAME_MAX: C2RustUnnamed = 243;
pub const _SC_TRACE_EVENT_NAME_MAX: C2RustUnnamed = 242;
pub const _SC_SS_REPL_MAX: C2RustUnnamed = 241;
pub const _SC_V7_LPBIG_OFFBIG: C2RustUnnamed = 240;
pub const _SC_V7_LP64_OFF64: C2RustUnnamed = 239;
pub const _SC_V7_ILP32_OFFBIG: C2RustUnnamed = 238;
pub const _SC_V7_ILP32_OFF32: C2RustUnnamed = 237;
pub const _SC_RAW_SOCKETS: C2RustUnnamed = 236;
pub const _SC_IPV6: C2RustUnnamed = 235;
pub const _SC_LEVEL4_CACHE_LINESIZE: C2RustUnnamed = 199;
pub const _SC_LEVEL4_CACHE_ASSOC: C2RustUnnamed = 198;
pub const _SC_LEVEL4_CACHE_SIZE: C2RustUnnamed = 197;
pub const _SC_LEVEL3_CACHE_LINESIZE: C2RustUnnamed = 196;
pub const _SC_LEVEL3_CACHE_ASSOC: C2RustUnnamed = 195;
pub const _SC_LEVEL3_CACHE_SIZE: C2RustUnnamed = 194;
pub const _SC_LEVEL2_CACHE_LINESIZE: C2RustUnnamed = 193;
pub const _SC_LEVEL2_CACHE_ASSOC: C2RustUnnamed = 192;
pub const _SC_LEVEL2_CACHE_SIZE: C2RustUnnamed = 191;
pub const _SC_LEVEL1_DCACHE_LINESIZE: C2RustUnnamed = 190;
pub const _SC_LEVEL1_DCACHE_ASSOC: C2RustUnnamed = 189;
pub const _SC_LEVEL1_DCACHE_SIZE: C2RustUnnamed = 188;
pub const _SC_LEVEL1_ICACHE_LINESIZE: C2RustUnnamed = 187;
pub const _SC_LEVEL1_ICACHE_ASSOC: C2RustUnnamed = 186;
pub const _SC_LEVEL1_ICACHE_SIZE: C2RustUnnamed = 185;
pub const _SC_TRACE_LOG: C2RustUnnamed = 184;
pub const _SC_TRACE_INHERIT: C2RustUnnamed = 183;
pub const _SC_TRACE_EVENT_FILTER: C2RustUnnamed = 182;
pub const _SC_TRACE: C2RustUnnamed = 181;
pub const _SC_HOST_NAME_MAX: C2RustUnnamed = 180;
pub const _SC_V6_LPBIG_OFFBIG: C2RustUnnamed = 179;
pub const _SC_V6_LP64_OFF64: C2RustUnnamed = 178;
pub const _SC_V6_ILP32_OFFBIG: C2RustUnnamed = 177;
pub const _SC_V6_ILP32_OFF32: C2RustUnnamed = 176;
pub const _SC_2_PBS_CHECKPOINT: C2RustUnnamed = 175;
pub const _SC_STREAMS: C2RustUnnamed = 174;
pub const _SC_SYMLOOP_MAX: C2RustUnnamed = 173;
pub const _SC_2_PBS_TRACK: C2RustUnnamed = 172;
pub const _SC_2_PBS_MESSAGE: C2RustUnnamed = 171;
pub const _SC_2_PBS_LOCATE: C2RustUnnamed = 170;
pub const _SC_2_PBS_ACCOUNTING: C2RustUnnamed = 169;
pub const _SC_2_PBS: C2RustUnnamed = 168;
pub const _SC_USER_GROUPS_R: C2RustUnnamed = 167;
pub const _SC_USER_GROUPS: C2RustUnnamed = 166;
pub const _SC_TYPED_MEMORY_OBJECTS: C2RustUnnamed = 165;
pub const _SC_TIMEOUTS: C2RustUnnamed = 164;
pub const _SC_SYSTEM_DATABASE_R: C2RustUnnamed = 163;
pub const _SC_SYSTEM_DATABASE: C2RustUnnamed = 162;
pub const _SC_THREAD_SPORADIC_SERVER: C2RustUnnamed = 161;
pub const _SC_SPORADIC_SERVER: C2RustUnnamed = 160;
pub const _SC_SPAWN: C2RustUnnamed = 159;
pub const _SC_SIGNALS: C2RustUnnamed = 158;
pub const _SC_SHELL: C2RustUnnamed = 157;
pub const _SC_REGEX_VERSION: C2RustUnnamed = 156;
pub const _SC_REGEXP: C2RustUnnamed = 155;
pub const _SC_SPIN_LOCKS: C2RustUnnamed = 154;
pub const _SC_READER_WRITER_LOCKS: C2RustUnnamed = 153;
pub const _SC_NETWORKING: C2RustUnnamed = 152;
pub const _SC_SINGLE_PROCESS: C2RustUnnamed = 151;
pub const _SC_MULTI_PROCESS: C2RustUnnamed = 150;
pub const _SC_MONOTONIC_CLOCK: C2RustUnnamed = 149;
pub const _SC_FILE_SYSTEM: C2RustUnnamed = 148;
pub const _SC_FILE_LOCKING: C2RustUnnamed = 147;
pub const _SC_FILE_ATTRIBUTES: C2RustUnnamed = 146;
pub const _SC_PIPE: C2RustUnnamed = 145;
pub const _SC_FIFO: C2RustUnnamed = 144;
pub const _SC_FD_MGMT: C2RustUnnamed = 143;
pub const _SC_DEVICE_SPECIFIC_R: C2RustUnnamed = 142;
pub const _SC_DEVICE_SPECIFIC: C2RustUnnamed = 141;
pub const _SC_DEVICE_IO: C2RustUnnamed = 140;
pub const _SC_THREAD_CPUTIME: C2RustUnnamed = 139;
pub const _SC_CPUTIME: C2RustUnnamed = 138;
pub const _SC_CLOCK_SELECTION: C2RustUnnamed = 137;
pub const _SC_C_LANG_SUPPORT_R: C2RustUnnamed = 136;
pub const _SC_C_LANG_SUPPORT: C2RustUnnamed = 135;
pub const _SC_BASE: C2RustUnnamed = 134;
pub const _SC_BARRIERS: C2RustUnnamed = 133;
pub const _SC_ADVISORY_INFO: C2RustUnnamed = 132;
pub const _SC_XOPEN_REALTIME_THREADS: C2RustUnnamed = 131;
pub const _SC_XOPEN_REALTIME: C2RustUnnamed = 130;
pub const _SC_XOPEN_LEGACY: C2RustUnnamed = 129;
pub const _SC_XBS5_LPBIG_OFFBIG: C2RustUnnamed = 128;
pub const _SC_XBS5_LP64_OFF64: C2RustUnnamed = 127;
pub const _SC_XBS5_ILP32_OFFBIG: C2RustUnnamed = 126;
pub const _SC_XBS5_ILP32_OFF32: C2RustUnnamed = 125;
pub const _SC_NL_TEXTMAX: C2RustUnnamed = 124;
pub const _SC_NL_SETMAX: C2RustUnnamed = 123;
pub const _SC_NL_NMAX: C2RustUnnamed = 122;
pub const _SC_NL_MSGMAX: C2RustUnnamed = 121;
pub const _SC_NL_LANGMAX: C2RustUnnamed = 120;
pub const _SC_NL_ARGMAX: C2RustUnnamed = 119;
pub const _SC_USHRT_MAX: C2RustUnnamed = 118;
pub const _SC_ULONG_MAX: C2RustUnnamed = 117;
pub const _SC_UINT_MAX: C2RustUnnamed = 116;
pub const _SC_UCHAR_MAX: C2RustUnnamed = 115;
pub const _SC_SHRT_MIN: C2RustUnnamed = 114;
pub const _SC_SHRT_MAX: C2RustUnnamed = 113;
pub const _SC_SCHAR_MIN: C2RustUnnamed = 112;
pub const _SC_SCHAR_MAX: C2RustUnnamed = 111;
pub const _SC_SSIZE_MAX: C2RustUnnamed = 110;
pub const _SC_NZERO: C2RustUnnamed = 109;
pub const _SC_MB_LEN_MAX: C2RustUnnamed = 108;
pub const _SC_WORD_BIT: C2RustUnnamed = 107;
pub const _SC_LONG_BIT: C2RustUnnamed = 106;
pub const _SC_INT_MIN: C2RustUnnamed = 105;
pub const _SC_INT_MAX: C2RustUnnamed = 104;
pub const _SC_CHAR_MIN: C2RustUnnamed = 103;
pub const _SC_CHAR_MAX: C2RustUnnamed = 102;
pub const _SC_CHAR_BIT: C2RustUnnamed = 101;
pub const _SC_XOPEN_XPG4: C2RustUnnamed = 100;
pub const _SC_XOPEN_XPG3: C2RustUnnamed = 99;
pub const _SC_XOPEN_XPG2: C2RustUnnamed = 98;
pub const _SC_2_UPE: C2RustUnnamed = 97;
pub const _SC_2_C_VERSION: C2RustUnnamed = 96;
pub const _SC_2_CHAR_TERM: C2RustUnnamed = 95;
pub const _SC_XOPEN_SHM: C2RustUnnamed = 94;
pub const _SC_XOPEN_ENH_I18N: C2RustUnnamed = 93;
pub const _SC_XOPEN_CRYPT: C2RustUnnamed = 92;
pub const _SC_XOPEN_UNIX: C2RustUnnamed = 91;
pub const _SC_XOPEN_XCU_VERSION: C2RustUnnamed = 90;
pub const _SC_XOPEN_VERSION: C2RustUnnamed = 89;
pub const _SC_PASS_MAX: C2RustUnnamed = 88;
pub const _SC_ATEXIT_MAX: C2RustUnnamed = 87;
pub const _SC_AVPHYS_PAGES: C2RustUnnamed = 86;
pub const _SC_PHYS_PAGES: C2RustUnnamed = 85;
pub const _SC_NPROCESSORS_ONLN: C2RustUnnamed = 84;
pub const _SC_NPROCESSORS_CONF: C2RustUnnamed = 83;
pub const _SC_THREAD_PROCESS_SHARED: C2RustUnnamed = 82;
pub const _SC_THREAD_PRIO_PROTECT: C2RustUnnamed = 81;
pub const _SC_THREAD_PRIO_INHERIT: C2RustUnnamed = 80;
pub const _SC_THREAD_PRIORITY_SCHEDULING: C2RustUnnamed = 79;
pub const _SC_THREAD_ATTR_STACKSIZE: C2RustUnnamed = 78;
pub const _SC_THREAD_ATTR_STACKADDR: C2RustUnnamed = 77;
pub const _SC_THREAD_THREADS_MAX: C2RustUnnamed = 76;
pub const _SC_THREAD_STACK_MIN: C2RustUnnamed = 75;
pub const _SC_THREAD_KEYS_MAX: C2RustUnnamed = 74;
pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: C2RustUnnamed = 73;
pub const _SC_TTY_NAME_MAX: C2RustUnnamed = 72;
pub const _SC_LOGIN_NAME_MAX: C2RustUnnamed = 71;
pub const _SC_GETPW_R_SIZE_MAX: C2RustUnnamed = 70;
pub const _SC_GETGR_R_SIZE_MAX: C2RustUnnamed = 69;
pub const _SC_THREAD_SAFE_FUNCTIONS: C2RustUnnamed = 68;
pub const _SC_THREADS: C2RustUnnamed = 67;
pub const _SC_T_IOV_MAX: C2RustUnnamed = 66;
pub const _SC_PII_OSI_M: C2RustUnnamed = 65;
pub const _SC_PII_OSI_CLTS: C2RustUnnamed = 64;
pub const _SC_PII_OSI_COTS: C2RustUnnamed = 63;
pub const _SC_PII_INTERNET_DGRAM: C2RustUnnamed = 62;
pub const _SC_PII_INTERNET_STREAM: C2RustUnnamed = 61;
pub const _SC_IOV_MAX: C2RustUnnamed = 60;
pub const _SC_UIO_MAXIOV: C2RustUnnamed = 60;
pub const _SC_SELECT: C2RustUnnamed = 59;
pub const _SC_POLL: C2RustUnnamed = 58;
pub const _SC_PII_OSI: C2RustUnnamed = 57;
pub const _SC_PII_INTERNET: C2RustUnnamed = 56;
pub const _SC_PII_SOCKET: C2RustUnnamed = 55;
pub const _SC_PII_XTI: C2RustUnnamed = 54;
pub const _SC_PII: C2RustUnnamed = 53;
pub const _SC_2_LOCALEDEF: C2RustUnnamed = 52;
pub const _SC_2_SW_DEV: C2RustUnnamed = 51;
pub const _SC_2_FORT_RUN: C2RustUnnamed = 50;
pub const _SC_2_FORT_DEV: C2RustUnnamed = 49;
pub const _SC_2_C_DEV: C2RustUnnamed = 48;
pub const _SC_2_C_BIND: C2RustUnnamed = 47;
pub const _SC_2_VERSION: C2RustUnnamed = 46;
pub const _SC_CHARCLASS_NAME_MAX: C2RustUnnamed = 45;
pub const _SC_RE_DUP_MAX: C2RustUnnamed = 44;
pub const _SC_LINE_MAX: C2RustUnnamed = 43;
pub const _SC_EXPR_NEST_MAX: C2RustUnnamed = 42;
pub const _SC_EQUIV_CLASS_MAX: C2RustUnnamed = 41;
pub const _SC_COLL_WEIGHTS_MAX: C2RustUnnamed = 40;
pub const _SC_BC_STRING_MAX: C2RustUnnamed = 39;
pub const _SC_BC_SCALE_MAX: C2RustUnnamed = 38;
pub const _SC_BC_DIM_MAX: C2RustUnnamed = 37;
pub const _SC_BC_BASE_MAX: C2RustUnnamed = 36;
pub const _SC_TIMER_MAX: C2RustUnnamed = 35;
pub const _SC_SIGQUEUE_MAX: C2RustUnnamed = 34;
pub const _SC_SEM_VALUE_MAX: C2RustUnnamed = 33;
pub const _SC_SEM_NSEMS_MAX: C2RustUnnamed = 32;
pub const _SC_RTSIG_MAX: C2RustUnnamed = 31;
pub const _SC_PAGESIZE: C2RustUnnamed = 30;
pub const _SC_VERSION: C2RustUnnamed = 29;
pub const _SC_MQ_PRIO_MAX: C2RustUnnamed = 28;
pub const _SC_MQ_OPEN_MAX: C2RustUnnamed = 27;
pub const _SC_DELAYTIMER_MAX: C2RustUnnamed = 26;
pub const _SC_AIO_PRIO_DELTA_MAX: C2RustUnnamed = 25;
pub const _SC_AIO_MAX: C2RustUnnamed = 24;
pub const _SC_AIO_LISTIO_MAX: C2RustUnnamed = 23;
pub const _SC_SHARED_MEMORY_OBJECTS: C2RustUnnamed = 22;
pub const _SC_SEMAPHORES: C2RustUnnamed = 21;
pub const _SC_MESSAGE_PASSING: C2RustUnnamed = 20;
pub const _SC_MEMORY_PROTECTION: C2RustUnnamed = 19;
pub const _SC_MEMLOCK_RANGE: C2RustUnnamed = 18;
pub const _SC_MEMLOCK: C2RustUnnamed = 17;
pub const _SC_MAPPED_FILES: C2RustUnnamed = 16;
pub const _SC_FSYNC: C2RustUnnamed = 15;
pub const _SC_SYNCHRONIZED_IO: C2RustUnnamed = 14;
pub const _SC_PRIORITIZED_IO: C2RustUnnamed = 13;
pub const _SC_ASYNCHRONOUS_IO: C2RustUnnamed = 12;
pub const _SC_TIMERS: C2RustUnnamed = 11;
pub const _SC_PRIORITY_SCHEDULING: C2RustUnnamed = 10;
pub const _SC_REALTIME_SIGNALS: C2RustUnnamed = 9;
pub const _SC_SAVED_IDS: C2RustUnnamed = 8;
pub const _SC_JOB_CONTROL: C2RustUnnamed = 7;
pub const _SC_TZNAME_MAX: C2RustUnnamed = 6;
pub const _SC_STREAM_MAX: C2RustUnnamed = 5;
pub const _SC_OPEN_MAX: C2RustUnnamed = 4;
pub const _SC_NGROUPS_MAX: C2RustUnnamed = 3;
pub const _SC_CLK_TCK: C2RustUnnamed = 2;
pub const _SC_CHILD_MAX: C2RustUnnamed = 1;
pub const _SC_ARG_MAX: C2RustUnnamed = 0;
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
    pub offset_next_write: uint32_t,
    pub offset_last_wrap: uint32_t,
    pub size: uint32_t,
    pub wrap_count: uint32_t,
}

pub type sa_family_t = ::core::ffi::c_ushort;

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
    (*header).offset_next_write = logwalk.offset_from(logbuffer) as ::core::ffi::c_long as uint32_t;
    (*header).offset_last_wrap =
        loglastwrap.offset_from(logbuffer) as ::core::ffi::c_long as uint32_t;
    (*header).size = logbuffer_size as uint32_t;
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
        physical_mem_bytes = sysconf(_SC_PHYS_PAGES as ::core::ffi::c_int)
            as ::core::ffi::c_longlong
            * sysconf(_SC_PAGESIZE as ::core::ffi::c_int) as ::core::ffi::c_longlong;
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
    logbuffer_shm = shm_open(shmlogname, O_RDWR | O_CREAT, (S_IREAD | S_IWRITE) as mode_t);
    if logbuffer_shm == -(1 as ::core::ffi::c_int) {
        fprintf(
            stderr,
            c"Could not shm_open SHM segment for the i3 log: %s\n".as_ptr(),
            strerror(*__errno_location()),
        );
        return;
    }
    let mut ret: ::core::ffi::c_int = 0;
    ret = posix_fallocate(logbuffer_shm, 0 as __off64_t, logbuffer_size as __off64_t);
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
        logbuffer_size as size_t,
        PROT_READ | PROT_WRITE,
        MAP_SHARED,
        logbuffer_shm,
        0 as __off64_t,
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
        logbuffer_size as size_t,
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
    static mut t: time_t = 0;
    static mut tmp: *mut tm = ::core::ptr::null::<tm>() as *mut tm;
    static mut len: size_t = 0;
    t = time(::core::ptr::null_mut::<time_t>());
    tmp = localtime_r(&raw mut t, &raw mut result);
    len = strftime(
        &raw mut message as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as size_t,
        c"%x %X - ".as_ptr(),
        tmp,
    );
    if logbuffer.is_null() {
        printf(c"%s".as_ptr(), &raw mut message as *mut ::core::ffi::c_char);
        vprintf(fmt, i333_va_list!(args));
    } else {
        len = len.wrapping_add(vsnprintf(
            (&raw mut message as *mut ::core::ffi::c_char).offset(len as isize),
            (::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as size_t).wrapping_sub(len),
            fmt,
            i333_va_list!(args),
        ) as size_t);
        if len >= ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as usize {
            fprintf(stderr, c"BUG: single log message > 4k\n".as_ptr());
            len = ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as usize as size_t;
            message[len.wrapping_sub(2 as size_t)] = '\n' as i32 as ::core::ffi::c_char;
        }
        if len
            >= (logbuffer_size as ::core::ffi::c_long
                - logwalk.offset_from(logbuffer) as ::core::ffi::c_long) as size_t
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
                1 as size_t,
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
            || st.st_size > 0 as __off_t
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
    let mut len: socklen_t = ::core::mem::size_of::<sockaddr_un>() as socklen_t;
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
        scalloc(1 as size_t, ::core::mem::size_of::<log_client>() as size_t) as *mut log_client;
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

pub unsafe fn log_broadcast_to_clients(message: *const ::core::ffi::c_char, len: size_t) {
    let mut current: *mut log_client = log_clients.tqh_first as *mut log_client;
    while !current.is_null() {
        let n: ssize_t = writeall((*current).fd, message as *const ::core::ffi::c_void, len);
        let previous: *mut log_client = current;
        current = (*current).clients.tqe_next as *mut log_client;
        if n < 0 as ssize_t {
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
