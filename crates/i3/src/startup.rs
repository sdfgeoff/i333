use i3_common::safewrappers::sasprintf;
/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * startup.c: Startup notification code. Ensures a startup notification context
 *            is set up when launching applications. We store the current
 *            workspace to open windows in that startup notification context on
 *            the appropriate workspace.
 *
 */
// Mechanical file-by-file Rust translation of src/startup.c.
// Originally translated with C2Rust 0.22.1; subsequently maintained in Rust.
pub use libev_sys::{ev_loop, ev_timer, ev_timer_start, ev_tstamp, ev_watcher, ev_watcher_time};
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __pid_t = ::core::ffi::c_int;
pub type __rlim64_t = ::core::ffi::c_ulong;
pub type __time_t = ::core::ffi::c_long;
pub type __suseconds_t = ::core::ffi::c_long;
pub type time_t = __time_t;

pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;

pub use libc::sighandler_t as __sighandler_t;
pub type i3String = _i3String;

pub type Time = ::core::ffi::c_ulong;

pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const SCRATCHPAD_CHANGED: C2RustUnnamed = 2;
pub const SCRATCHPAD_FRESH: C2RustUnnamed = 1;
pub const SCRATCHPAD_NONE: C2RustUnnamed = 0;

pub type C2RustUnnamed_4 = ::core::ffi::c_uint;
pub const FLOATING_USER_ON: C2RustUnnamed_4 = 3;
pub const FLOATING_AUTO_ON: C2RustUnnamed_4 = 2;
pub const FLOATING_USER_OFF: C2RustUnnamed_4 = 1;
pub const FLOATING_AUTO_OFF: C2RustUnnamed_4 = 0;
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

pub type C2RustUnnamed_6 = ::core::ffi::c_uint;
pub const M_BELOW: C2RustUnnamed_6 = 2;
pub const M_ASSIGN_WS: C2RustUnnamed_6 = 1;
pub const M_HERE: C2RustUnnamed_6 = 0;
pub type C2RustUnnamed_7 = ::core::ffi::c_uint;
pub const WM_FLOATING: C2RustUnnamed_7 = 6;
pub const WM_FLOATING_USER: C2RustUnnamed_7 = 5;
pub const WM_FLOATING_AUTO: C2RustUnnamed_7 = 4;
pub const WM_TILING: C2RustUnnamed_7 = 3;
pub const WM_TILING_USER: C2RustUnnamed_7 = 2;
pub const WM_TILING_AUTO: C2RustUnnamed_7 = 1;
pub const WM_ANY: C2RustUnnamed_7 = 0;
pub type C2RustUnnamed_8 = ::core::ffi::c_int;
pub const M_DOCK_BOTTOM: C2RustUnnamed_8 = 3;
pub const M_DOCK_TOP: C2RustUnnamed_8 = 2;
pub const M_DOCK_ANY: C2RustUnnamed_8 = 1;
pub const M_NODOCK: C2RustUnnamed_8 = 0;
pub const M_DONTCHECK: C2RustUnnamed_8 = -1;
pub type C2RustUnnamed_9 = ::core::ffi::c_int;
pub const U_OLDEST: C2RustUnnamed_9 = 1;
pub const U_LATEST: C2RustUnnamed_9 = 0;
pub const U_DONTCHECK: C2RustUnnamed_9 = -1;

pub type C2RustUnnamed_10 = ::core::ffi::c_uint;
pub const W_DOCK_BOTTOM: C2RustUnnamed_10 = 2;
pub const W_DOCK_TOP: C2RustUnnamed_10 = 1;
pub const W_NODOCK: C2RustUnnamed_10 = 0;

pub type C2RustUnnamed_13 = ::core::ffi::c_uint;
pub const A_TO_OUTPUT: C2RustUnnamed_13 = 16;
pub const A_TO_WORKSPACE_NUMBER: C2RustUnnamed_13 = 8;
pub const A_NO_FOCUS: C2RustUnnamed_13 = 4;
pub const A_TO_WORKSPACE: C2RustUnnamed_13 = 2;
pub const A_COMMAND: C2RustUnnamed_13 = 1;
pub const A_ANY: C2RustUnnamed_13 = 0;

pub type C2RustUnnamed_15 = ::core::ffi::c_uint;
pub const CT_DOCKAREA: C2RustUnnamed_15 = 5;
pub const CT_WORKSPACE: C2RustUnnamed_15 = 4;
pub const CT_FLOATING_CON: C2RustUnnamed_15 = 3;
pub const CT_CON: C2RustUnnamed_15 = 2;
pub const CT_OUTPUT: C2RustUnnamed_15 = 1;
pub const CT_ROOT: C2RustUnnamed_15 = 0;
pub type i3Window = Window;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Startup_Sequence {
    pub id: *mut ::core::ffi::c_char,
    pub workspace: *mut ::core::ffi::c_char,
    pub context: *mut SnLauncherContext,
    pub delete_at: time_t,
    pub sequences: C2RustUnnamed_16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_16 {
    pub tqe_next: *mut Startup_Sequence,
    pub tqe_prev: *mut *mut Startup_Sequence,
}

pub type __rlimit_resource = ::core::ffi::c_uint;
pub const __RLIM_NLIMITS: __rlimit_resource = 16;
pub const __RLIMIT_NLIMITS: __rlimit_resource = 16;
pub const __RLIMIT_RTTIME: __rlimit_resource = 15;
pub const __RLIMIT_RTPRIO: __rlimit_resource = 14;
pub const __RLIMIT_NICE: __rlimit_resource = 13;
pub const __RLIMIT_MSGQUEUE: __rlimit_resource = 12;
pub const __RLIMIT_SIGPENDING: __rlimit_resource = 11;
pub const __RLIMIT_LOCKS: __rlimit_resource = 10;
pub const __RLIMIT_MEMLOCK: __rlimit_resource = 8;
pub const __RLIMIT_NPROC: __rlimit_resource = 6;
pub const RLIMIT_AS: __rlimit_resource = 9;
pub const __RLIMIT_OFILE: __rlimit_resource = 7;
pub const RLIMIT_NOFILE: __rlimit_resource = 7;
pub const __RLIMIT_RSS: __rlimit_resource = 5;
pub const RLIMIT_CORE: __rlimit_resource = 4;
pub const RLIMIT_STACK: __rlimit_resource = 3;
pub const RLIMIT_DATA: __rlimit_resource = 2;
pub const RLIMIT_FSIZE: __rlimit_resource = 1;
pub const RLIMIT_CPU: __rlimit_resource = 0;
pub type rlim_t = __rlim64_t;

pub type __rlimit_resource_t = __rlimit_resource;
pub type xcursor_cursor_t = ::core::ffi::c_uint;
pub const XCURSOR_CURSOR_MAX: xcursor_cursor_t = 9;
pub const XCURSOR_CURSOR_MOVE: xcursor_cursor_t = 8;
pub const XCURSOR_CURSOR_WATCH: xcursor_cursor_t = 7;
pub const XCURSOR_CURSOR_BOTTOM_RIGHT_CORNER: xcursor_cursor_t = 6;
pub const XCURSOR_CURSOR_BOTTOM_LEFT_CORNER: xcursor_cursor_t = 5;
pub const XCURSOR_CURSOR_TOP_RIGHT_CORNER: xcursor_cursor_t = 4;
pub const XCURSOR_CURSOR_TOP_LEFT_CORNER: xcursor_cursor_t = 3;
pub const XCURSOR_CURSOR_RESIZE_VERTICAL: xcursor_cursor_t = 2;
pub const XCURSOR_CURSOR_RESIZE_HORIZONTAL: xcursor_cursor_t = 1;
pub const XCURSOR_CURSOR_POINTER: xcursor_cursor_t = 0;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct startup_sequence_head {
    pub tqh_first: *mut Startup_Sequence,
    pub tqh_last: *mut *mut Startup_Sequence,
}
pub const __ASSERT_FUNCTION: [::core::ffi::c_char; 56] = unsafe {
    ::core::mem::transmute::<[u8; 56], [::core::ffi::c_char; 56]>(
        *b"void startup_sequence_delete(struct Startup_Sequence *)\0",
    )
};
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const SIG_DFL: __sighandler_t = libc::SIG_DFL;
pub const SIGPIPE: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const SD_LISTEN_FDS_START: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const _PATH_BSHELL: [::core::ffi::c_char; 8] =
    unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"/bin/sh\0") };
static mut startup_sequences: startup_sequence_head = startup_sequence_head {
    tqh_first: ::core::ptr::null::<Startup_Sequence>() as *mut Startup_Sequence,
    tqh_last: ::core::ptr::null::<*mut Startup_Sequence>() as *mut *mut Startup_Sequence,
};
unsafe extern "C" fn startup_timeout(
    loop_0: *mut ev_loop,
    w: *mut ev_timer,
    revents: ::core::ffi::c_int,
) {
    let id: *const ::core::ffi::c_char =
        sn_launcher_context_get_startup_id((*w).data as *mut SnLauncherContext);
    debuglog(
        c"%s:%s:%d - Timeout for startup sequence %s\n"
            .as_ptr()
            .cast_mut(),
        c"../src/startup.c".as_ptr(),
        c"startup_timeout".as_ptr(),
        36 as ::core::ffi::c_int,
        id,
    );
    let mut current: *mut Startup_Sequence = ::core::ptr::null_mut::<Startup_Sequence>();
    let mut sequence: *mut Startup_Sequence = ::core::ptr::null_mut::<Startup_Sequence>();
    current = startup_sequences.tqh_first;
    while !current.is_null() {
        if strcmp((*current).id, id) != 0 as ::core::ffi::c_int {
            current = (*current).sequences.tqe_next;
        } else {
            sequence = current;
            break;
        }
    }
    sn_launcher_context_unref((*w).data as *mut SnLauncherContext);
    if sequence.is_null() {
        debuglog(
            c"%s:%s:%d - Sequence already deleted, nevermind.\n"
                .as_ptr()
                .cast_mut(),
            c"../src/startup.c".as_ptr(),
            c"startup_timeout".as_ptr(),
            52 as ::core::ffi::c_int,
        );
        free(w as *mut ::core::ffi::c_void);
        return;
    }
    sn_launcher_context_complete((*w).data as *mut SnLauncherContext);
    free(w as *mut ::core::ffi::c_void);
}
unsafe fn _prune_startup_sequences() -> ::core::ffi::c_int {
    let current_time: time_t = time(::core::ptr::null_mut::<time_t>()) as time_t;
    let mut active_sequences: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut next: *mut Startup_Sequence = startup_sequences.tqh_first;
    while !next.is_null() {
        let current: *mut Startup_Sequence = next;
        next = (*next).sequences.tqe_next;
        if (*current).delete_at == 0 as time_t {
            active_sequences += 1;
        } else {
            if current_time <= (*current).delete_at {
                continue;
            }
            startup_sequence_delete(current);
        }
    }
    active_sequences
}

pub unsafe fn startup_sequence_delete(mut sequence: *mut Startup_Sequence) {
    if sequence != ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut Startup_Sequence {
    } else {
        __assert_fail(
            c"sequence != NULL".as_ptr(),
            c"../src/startup.c".as_ptr(),
            106 as ::core::ffi::c_uint,
            __ASSERT_FUNCTION.as_ptr(),
        );
    };
    debuglog(
        c"%s:%s:%d - Deleting startup sequence %s, delete_at = %lld, current_time = %lld\n"
            .as_ptr()
            .cast_mut(),
        c"../src/startup.c".as_ptr(),
        c"startup_sequence_delete".as_ptr(),
        108 as ::core::ffi::c_int,
        (*sequence).id,
        (*sequence).delete_at as ::core::ffi::c_longlong,
        time(::core::ptr::null_mut::<time_t>()) as ::core::ffi::c_longlong,
    );
    sn_launcher_context_unref((*sequence).context);
    if !(*sequence).sequences.tqe_next.is_null() {
        (*(*sequence).sequences.tqe_next).sequences.tqe_prev = (*sequence).sequences.tqe_prev;
    } else {
        startup_sequences.tqh_last = (*sequence).sequences.tqe_prev;
    }
    *(*sequence).sequences.tqe_prev = (*sequence).sequences.tqe_next;
    free((*sequence).id as *mut ::core::ffi::c_void);
    free((*sequence).workspace as *mut ::core::ffi::c_void);
    free(sequence as *mut ::core::ffi::c_void);
    sequence = ::core::ptr::null_mut::<Startup_Sequence>();
}

pub unsafe fn start_application(
    command: *const ::core::ffi::c_char,
    no_startup_id: bool,
    window_id: xcb_window_t,
) {
    let mut context: *mut SnLauncherContext = ::core::ptr::null_mut::<SnLauncherContext>();
    if !no_startup_id {
        context = sn_launcher_context_new(sndisplay, conn_screen);
        sn_launcher_context_set_name(context, c"i3".as_ptr());
        sn_launcher_context_set_description(context, c"exec command in i3".as_ptr());
        let first_word: *mut ::core::ffi::c_char = sstrdup(command);
        let space: *mut ::core::ffi::c_char = strchr(first_word, ' ' as i32);
        if !space.is_null() {
            *space = '\0' as i32 as ::core::ffi::c_char;
        }
        sn_launcher_context_initiate(context, c"i3".as_ptr(), first_word, last_timestamp as Time);
        free(first_word as *mut ::core::ffi::c_void);
        let timeout: *mut ev_timer =
            scalloc(1 as size_t, ::core::mem::size_of::<ev_timer>() as size_t) as *mut ev_timer;
        let ref mut fresh0 = (*(timeout as *mut ::core::ffi::c_void as *mut ev_watcher)).pending;
        *fresh0 = 0 as ::core::ffi::c_int;
        (*(timeout as *mut ::core::ffi::c_void as *mut ev_watcher)).active = *fresh0;
        (*(timeout as *mut ::core::ffi::c_void as *mut ev_watcher)).priority =
            0 as ::core::ffi::c_int;
        (*timeout).cb = Some(
            startup_timeout
                as unsafe extern "C" fn(*mut ev_loop, *mut ev_timer, ::core::ffi::c_int) -> (),
        )
            as Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_timer, ::core::ffi::c_int) -> ()>;
        memmove(
            &raw mut (*(timeout as *mut ev_watcher)).cb as *mut ::core::ffi::c_void,
            &raw mut (*timeout).cb as *const ::core::ffi::c_void,
            ::core::mem::size_of::<
                Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_timer, ::core::ffi::c_int) -> ()>,
            >() as size_t,
        );
        (*(timeout as *mut ev_watcher_time)).at = 60.0f64 as ev_tstamp;
        (*timeout).repeat = 0.0f64 as ev_tstamp;
        (*timeout).data = context as *mut ::core::ffi::c_void;
        ev_timer_start(main_loop, timeout);
        verboselog(
            c"startup id = %s\n".as_ptr().cast_mut(),
            sn_launcher_context_get_startup_id(context),
        );
        let ws: *mut Con = con_get_workspace(focused);
        let sequence: *mut Startup_Sequence = scalloc(
            1 as size_t,
            ::core::mem::size_of::<Startup_Sequence>() as size_t,
        ) as *mut Startup_Sequence;
        (*sequence).id = sstrdup(sn_launcher_context_get_startup_id(context));
        (*sequence).workspace = sstrdup((*ws).name);
        (*sequence).context = context;
        (*sequence).sequences.tqe_next = ::core::ptr::null_mut::<Startup_Sequence>();
        (*sequence).sequences.tqe_prev = startup_sequences.tqh_last;
        *startup_sequences.tqh_last = sequence;
        startup_sequences.tqh_last = &raw mut (*sequence).sequences.tqe_next;
        sn_launcher_context_ref(context);
    }
    verboselog(c"executing: %s\n".as_ptr().cast_mut(), command);
    if fork() == 0 as ::core::ffi::c_int {
        setsid();
        setrlimit(RLIMIT_CORE, &raw mut original_rlimit_core);
        let mut fd: ::core::ffi::c_int = SD_LISTEN_FDS_START;
        while fd < SD_LISTEN_FDS_START + listen_fds {
            close(fd);
            fd += 1;
        }
        unsetenv(c"LISTEN_PID".as_ptr());
        unsetenv(c"LISTEN_FDS".as_ptr());
        signal(SIGPIPE, libc::SIG_DFL);
        if !no_startup_id {
            sn_launcher_context_setup_child_process(context);
        }
        setenv(
            c"I3SOCK".as_ptr(),
            current_socketpath,
            1 as ::core::ffi::c_int,
        );
        if window_id as ::core::ffi::c_long != XCB_NONE {
            let mut window_id_str: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(&raw mut window_id_str, c"%d".as_ptr(), window_id);
            setenv(
                c"I3_WINDOW_ID".as_ptr(),
                window_id_str,
                1 as ::core::ffi::c_int,
            );
            free(window_id_str as *mut ::core::ffi::c_void);
        }
        execl(
            _PATH_BSHELL.as_ptr(),
            _PATH_BSHELL.as_ptr(),
            c"-c".as_ptr(),
            command,
            NULL,
        );
        err(EXIT_FAILURE, c"execl return".as_ptr());
    }
    if !no_startup_id {
        xcursor_set_root_cursor(XCURSOR_CURSOR_WATCH as ::core::ffi::c_int);
    }
}

pub unsafe extern "C" fn startup_monitor_event(
    event: *mut SnMonitorEvent,
    userdata: *mut ::core::ffi::c_void,
) {
    let sn_startup_sequence: *mut SnStartupSequence = sn_monitor_event_get_startup_sequence(event);
    let id: *const ::core::ffi::c_char = sn_startup_sequence_get_id(sn_startup_sequence);
    let mut current: *mut Startup_Sequence = ::core::ptr::null_mut::<Startup_Sequence>();
    let mut sequence: *mut Startup_Sequence = ::core::ptr::null_mut::<Startup_Sequence>();
    current = startup_sequences.tqh_first;
    while !current.is_null() {
        if strcmp((*current).id, id) != 0 as ::core::ffi::c_int {
            current = (*current).sequences.tqe_next;
        } else {
            sequence = current;
            break;
        }
    }
    if sequence.is_null() {
        debuglog(
            c"%s:%s:%d - Got event for startup sequence that we did not initiate (ID = %s). Ignoring.\n".as_ptr().cast_mut(),
            c"../src/startup.c".as_ptr(),
            c"startup_monitor_event".as_ptr(),
            237 as ::core::ffi::c_int,
            id,
        );
        return;
    }
    match sn_monitor_event_get_type(event) as ::core::ffi::c_uint {
        1 => {
            debuglog(
                c"%s:%s:%d - startup sequence %s completed\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/startup.c".as_ptr(),
                c"startup_monitor_event".as_ptr(),
                243 as ::core::ffi::c_int,
                sn_startup_sequence_get_id(sn_startup_sequence),
            );
            let current_time: time_t = time(::core::ptr::null_mut::<time_t>());
            (*sequence).delete_at = current_time + 30 as time_t;
            debuglog(
                c"%s:%s:%d - Will delete startup sequence %s at timestamp %lld\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/startup.c".as_ptr(),
                c"startup_monitor_event".as_ptr(),
                249 as ::core::ffi::c_int,
                (*sequence).id,
                (*sequence).delete_at as ::core::ffi::c_longlong,
            );
            if _prune_startup_sequences() == 0 as ::core::ffi::c_int {
                debuglog(
                    c"%s:%s:%d - No more startup sequences running, changing root window cursor to default pointer.\n".as_ptr().cast_mut(),
                    c"../src/startup.c".as_ptr(),
                    c"startup_monitor_event".as_ptr(),
                    252 as ::core::ffi::c_int,
                );
                xcursor_set_root_cursor(XCURSOR_CURSOR_POINTER as ::core::ffi::c_int);
            }
        }
        _ => {}
    };
}

pub unsafe fn startup_sequence_rename_workspace(
    old_name: *const ::core::ffi::c_char,
    new_name: *const ::core::ffi::c_char,
) {
    let mut current: *mut Startup_Sequence = ::core::ptr::null_mut::<Startup_Sequence>();
    current = startup_sequences.tqh_first;
    while !current.is_null() {
        if !(strcmp((*current).workspace, old_name) != 0 as ::core::ffi::c_int) {
            debuglog(
                c"%s:%s:%d - Renaming workspace \"%s\" to \"%s\" in startup sequence %s.\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/startup.c".as_ptr(),
                c"startup_sequence_rename_workspace".as_ptr(),
                274 as ::core::ffi::c_int,
                old_name,
                new_name,
                (*current).id,
            );
            free((*current).workspace as *mut ::core::ffi::c_void);
            (*current).workspace = sstrdup(new_name);
        }
        current = (*current).sequences.tqe_next;
    }
}

pub unsafe fn startup_sequence_get(
    cwindow: *const i3Window,
    mut startup_id_reply: *mut xcb_get_property_reply_t,
    ignore_mapped_leader: bool,
) -> *mut Startup_Sequence {
    if startup_id_reply.is_null()
        || xcb_get_property_value_length(startup_id_reply) == 0 as ::core::ffi::c_int
    {
        free(startup_id_reply as *mut ::core::ffi::c_void);
        startup_id_reply = ::core::ptr::null_mut::<xcb_get_property_reply_t>();
        debuglog(
            c"%s:%s:%d - No _NET_STARTUP_ID set on window 0x%08x\n"
                .as_ptr()
                .cast_mut(),
            c"../src/startup.c".as_ptr(),
            c"startup_sequence_get".as_ptr(),
            290 as ::core::ffi::c_int,
            (*cwindow).id,
        );
        if (*cwindow).leader as ::core::ffi::c_long == XCB_NONE {
            return ::core::ptr::null_mut::<Startup_Sequence>();
        }
        if ignore_mapped_leader as ::core::ffi::c_int != 0
            && !con_by_window_id((*cwindow).leader).is_null()
        {
            debuglog(
                c"%s:%s:%d - Ignoring leader window 0x%08x\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/startup.c".as_ptr(),
                c"startup_sequence_get".as_ptr(),
                303 as ::core::ffi::c_int,
                (*cwindow).leader,
            );
            return ::core::ptr::null_mut::<Startup_Sequence>();
        }
        debuglog(
            c"%s:%s:%d - Checking leader window 0x%08x\n"
                .as_ptr()
                .cast_mut(),
            c"../src/startup.c".as_ptr(),
            c"startup_sequence_get".as_ptr(),
            307 as ::core::ffi::c_int,
            (*cwindow).leader,
        );
        let cookie: xcb_get_property_cookie_t = xcb_get_property(
            conn,
            false_0 as uint8_t,
            (*cwindow).leader,
            A__NET_STARTUP_ID,
            XCB_GET_PROPERTY_TYPE_ANY as ::core::ffi::c_int as xcb_atom_t,
            0 as uint32_t,
            512 as uint32_t,
        ) as xcb_get_property_cookie_t;
        startup_id_reply = xcb_get_property_reply(
            conn,
            cookie,
            ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
        );
        if startup_id_reply.is_null()
            || xcb_get_property_value_length(startup_id_reply) == 0 as ::core::ffi::c_int
        {
            free(startup_id_reply as *mut ::core::ffi::c_void);
            startup_id_reply = ::core::ptr::null_mut::<xcb_get_property_reply_t>();
            debuglog(
                c"%s:%s:%d - No _NET_STARTUP_ID set on the leader either\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/startup.c".as_ptr(),
                c"startup_sequence_get".as_ptr(),
                316 as ::core::ffi::c_int,
            );
            return ::core::ptr::null_mut::<Startup_Sequence>();
        }
    }
    let mut startup_id: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    sasprintf(
        &raw mut startup_id,
        c"%.*s".as_ptr(),
        xcb_get_property_value_length(startup_id_reply),
        xcb_get_property_value(startup_id_reply) as *mut ::core::ffi::c_char,
    );
    let mut current: *mut Startup_Sequence = ::core::ptr::null_mut::<Startup_Sequence>();
    let mut sequence: *mut Startup_Sequence = ::core::ptr::null_mut::<Startup_Sequence>();
    current = startup_sequences.tqh_first;
    while !current.is_null() {
        if strcmp((*current).id, startup_id) != 0 as ::core::ffi::c_int {
            current = (*current).sequences.tqe_next;
        } else {
            sequence = current;
            break;
        }
    }
    if sequence.is_null() {
        debuglog(
            c"%s:%s:%d - WARNING: This sequence (ID %s) was not found\n"
                .as_ptr()
                .cast_mut(),
            c"../src/startup.c".as_ptr(),
            c"startup_sequence_get".as_ptr(),
            335 as ::core::ffi::c_int,
            startup_id,
        );
        free(startup_id as *mut ::core::ffi::c_void);
        free(startup_id_reply as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<Startup_Sequence>();
    }
    free(startup_id as *mut ::core::ffi::c_void);
    free(startup_id_reply as *mut ::core::ffi::c_void);
    sequence
}

pub unsafe fn startup_workspace_for_window(
    cwindow: *const i3Window,
    startup_id_reply: *mut xcb_get_property_reply_t,
) -> *mut ::core::ffi::c_char {
    let sequence: *mut Startup_Sequence =
        startup_sequence_get(cwindow, startup_id_reply, false_0 != 0);
    if sequence.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    let current_time: time_t = time(::core::ptr::null_mut::<time_t>());
    if (*sequence).delete_at > 0 as time_t && current_time > (*sequence).delete_at {
        debuglog(
            c"%s:%s:%d - Deleting expired startup sequence %s\n"
                .as_ptr()
                .cast_mut(),
            c"../src/startup.c".as_ptr(),
            c"startup_workspace_for_window".as_ptr(),
            365 as ::core::ffi::c_int,
            (*sequence).id,
        );
        startup_sequence_delete(sequence);
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    (*sequence).workspace
}

pub unsafe fn startup_sequence_delete_by_window(win: *const i3Window) {
    let cookie: xcb_get_property_cookie_t = xcb_get_property(
        conn,
        false_0 as uint8_t,
        (*win).id,
        A__NET_STARTUP_ID,
        XCB_GET_PROPERTY_TYPE_ANY as ::core::ffi::c_int as xcb_atom_t,
        0 as uint32_t,
        512 as uint32_t,
    ) as xcb_get_property_cookie_t;
    let startup_id_reply: *mut xcb_get_property_reply_t = xcb_get_property_reply(
        conn,
        cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    let sequence: *mut Startup_Sequence = startup_sequence_get(win, startup_id_reply, true_0 != 0);
    if !sequence.is_null() {
        startup_sequence_delete(sequence);
    }
}
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
unsafe extern "C" fn run_static_initializers() {
    startup_sequences = startup_sequence_head {
        tqh_first: ::core::ptr::null_mut::<Startup_Sequence>(),
        tqh_last: &raw mut startup_sequences.tqh_first,
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];

// Internal helpers use Rust linkage rather than foreign symbol lookup.
use crate::xcursor::xcursor_set_root_cursor;
use i3_common::safewrappers::scalloc;
use i3_common::safewrappers::sstrdup;

// Canonical Cairo backend ABI; renderer logic remains local.

use xcb_sys::{
    xcb_atom_t, xcb_generic_error_t, xcb_get_property, xcb_get_property_cookie_t,
    xcb_get_property_reply, xcb_get_property_reply_t, xcb_get_property_value,
    xcb_get_property_value_length, xcb_window_t, XCB_GET_PROPERTY_TYPE_ANY, XCB_NONE,
};

use startup_notification_sys::{
    sn_launcher_context_complete, sn_launcher_context_get_startup_id, sn_launcher_context_initiate,
    sn_launcher_context_new, sn_launcher_context_ref, sn_launcher_context_set_description,
    sn_launcher_context_set_name, sn_launcher_context_setup_child_process,
    sn_launcher_context_unref, sn_monitor_event_get_startup_sequence, sn_monitor_event_get_type,
    sn_startup_sequence_get_id, SnLauncherContext, SnMonitorEvent, SnStartupSequence,
};

pub use libc::{
    close, execl, fork, free, memmove, setenv, setrlimit, setsid, signal, strchr, strcmp, time,
    unsetenv,
};

use i3_common::native_stdio::{__assert_fail, err};

use crate::log::{debuglog, verboselog};

// Shared model types retain field layout while using one Rust identity.
pub use crate::types::Con;
pub use crate::types::Window;
pub use i3_common::string::I3String as _i3String;

// Internal calls and globals use canonical Rust identities.
use crate::application::conn;
use crate::application::conn_screen;
use crate::application::last_timestamp;
use crate::application::listen_fds;
use crate::application::main_loop;
use crate::application::original_rlimit_core;
use crate::application::sndisplay;
use crate::application::A__NET_STARTUP_ID;
use crate::con::con_by_window_id;
use crate::con::con_get_workspace;
use crate::ipc::current_socketpath;
use crate::tree::focused;
