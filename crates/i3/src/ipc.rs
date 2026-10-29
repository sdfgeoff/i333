use crate::commands_parser::command_result_free;
use crate::commands_parser::parse_command;
use crate::config::barconfigs;
use crate::config::included_files;
use crate::config::modes;
use crate::randr::outputs;
use crate::tree::all_cons;
use i3_common::safewrappers::sasprintf;
/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * ipc.c: UNIX domain socket IPC (initialization, client handling, protocol).
 *
 */
// Mechanical file-by-file Rust translation of src/ipc.c.
// Originally translated with C2Rust 0.22.1; subsequently maintained in Rust.
use i3_json::compat::{
    yajl_alloc, yajl_free, yajl_free_error, yajl_gen_alloc, yajl_gen_array_close,
    yajl_gen_array_open, yajl_gen_bool, yajl_gen_double, yajl_gen_free, yajl_gen_get_buf,
    yajl_gen_integer, yajl_gen_map_close, yajl_gen_map_open, yajl_gen_null, yajl_gen_string,
    yajl_get_error, yajl_parse,
};
pub use i3_json::compat::{
    yajl_alloc_funcs, yajl_callbacks, yajl_gen, yajl_handle, yajl_status, yajl_status_ok,
};
pub use libev_sys::{
    ev_io, ev_io_start, ev_io_stop, ev_loop, ev_timer, ev_timer_start, ev_timer_stop, ev_tstamp,
    ev_watcher, ev_watcher_time, EV_READ, EV_WRITE, EV__IOFDSET,
};
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __uid_t = ::core::ffi::c_uint;
pub type __gid_t = ::core::ffi::c_uint;
pub type __pid_t = ::core::ffi::c_int;
pub type __time_t = ::core::ffi::c_long;
pub type __suseconds_t = ::core::ffi::c_long;
pub type __socklen_t = ::core::ffi::c_uint;
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
pub type pid_t = __pid_t;
pub type ssize_t = isize;
pub type time_t = __time_t;
pub type int32_t = __int32_t;

pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uintptr_t = usize;

pub type socklen_t = __socklen_t;
pub type i3String = _i3String;

pub use crate::application::Binding;
pub type i3_event_state_mask_t = uint32_t;
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const B_UPON_KEYRELEASE_IGNORE_MODS: C2RustUnnamed_1 = 2;
pub const B_UPON_KEYRELEASE: C2RustUnnamed_1 = 1;
pub const B_UPON_KEYPRESS: C2RustUnnamed_1 = 0;
pub type input_type_t = ::core::ffi::c_uint;
pub const B_MOUSE: input_type_t = 1;
pub const B_KEYBOARD: input_type_t = 0;

pub type C2RustUnnamed_3 = ::core::ffi::c_uint;
pub const SCRATCHPAD_CHANGED: C2RustUnnamed_3 = 2;
pub const SCRATCHPAD_FRESH: C2RustUnnamed_3 = 1;
pub const SCRATCHPAD_NONE: C2RustUnnamed_3 = 0;

pub type C2RustUnnamed_8 = ::core::ffi::c_uint;
pub const FLOATING_USER_ON: C2RustUnnamed_8 = 3;
pub const FLOATING_AUTO_ON: C2RustUnnamed_8 = 2;
pub const FLOATING_USER_OFF: C2RustUnnamed_8 = 1;
pub const FLOATING_AUTO_OFF: C2RustUnnamed_8 = 0;
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

pub type C2RustUnnamed_10 = ::core::ffi::c_uint;
pub const M_BELOW: C2RustUnnamed_10 = 2;
pub const M_ASSIGN_WS: C2RustUnnamed_10 = 1;
pub const M_HERE: C2RustUnnamed_10 = 0;
pub type C2RustUnnamed_11 = ::core::ffi::c_uint;
pub const WM_FLOATING: C2RustUnnamed_11 = 6;
pub const WM_FLOATING_USER: C2RustUnnamed_11 = 5;
pub const WM_FLOATING_AUTO: C2RustUnnamed_11 = 4;
pub const WM_TILING: C2RustUnnamed_11 = 3;
pub const WM_TILING_USER: C2RustUnnamed_11 = 2;
pub const WM_TILING_AUTO: C2RustUnnamed_11 = 1;
pub const WM_ANY: C2RustUnnamed_11 = 0;
pub type C2RustUnnamed_12 = ::core::ffi::c_int;
pub const M_DOCK_BOTTOM: C2RustUnnamed_12 = 3;
pub const M_DOCK_TOP: C2RustUnnamed_12 = 2;
pub const M_DOCK_ANY: C2RustUnnamed_12 = 1;
pub const M_NODOCK: C2RustUnnamed_12 = 0;
pub const M_DONTCHECK: C2RustUnnamed_12 = -1;
pub type C2RustUnnamed_13 = ::core::ffi::c_int;
pub const U_OLDEST: C2RustUnnamed_13 = 1;
pub const U_LATEST: C2RustUnnamed_13 = 0;
pub const U_DONTCHECK: C2RustUnnamed_13 = -1;

pub type C2RustUnnamed_14 = ::core::ffi::c_uint;
pub const W_DOCK_BOTTOM: C2RustUnnamed_14 = 2;
pub const W_DOCK_TOP: C2RustUnnamed_14 = 1;
pub const W_NODOCK: C2RustUnnamed_14 = 0;

pub type C2RustUnnamed_17 = ::core::ffi::c_uint;
pub const A_TO_OUTPUT: C2RustUnnamed_17 = 16;
pub const A_TO_WORKSPACE_NUMBER: C2RustUnnamed_17 = 8;
pub const A_NO_FOCUS: C2RustUnnamed_17 = 4;
pub const A_TO_WORKSPACE: C2RustUnnamed_17 = 2;
pub const A_COMMAND: C2RustUnnamed_17 = 1;
pub const A_ANY: C2RustUnnamed_17 = 0;

pub type C2RustUnnamed_19 = ::core::ffi::c_uint;
pub const CT_DOCKAREA: C2RustUnnamed_19 = 5;
pub const CT_WORKSPACE: C2RustUnnamed_19 = 4;
pub const CT_FLOATING_CON: C2RustUnnamed_19 = 3;
pub const CT_CON: C2RustUnnamed_19 = 2;
pub const CT_OUTPUT: C2RustUnnamed_19 = 1;
pub const CT_ROOT: C2RustUnnamed_19 = 0;

pub type Output = xoutput;
pub type orientation_t = ::core::ffi::c_uint;
pub const VERT: orientation_t = 2;
pub const HORIZ: orientation_t = 1;
pub const NO_ORIENTATION: orientation_t = 0;
pub type C2RustUnnamed_21 = ::core::ffi::c_uint;
pub const I3_XKB_GROUP_MASK_4: C2RustUnnamed_21 = 8;
pub const I3_XKB_GROUP_MASK_3: C2RustUnnamed_21 = 4;
pub const I3_XKB_GROUP_MASK_2: C2RustUnnamed_21 = 2;
pub const I3_XKB_GROUP_MASK_1: C2RustUnnamed_21 = 1;
pub const I3_XKB_GROUP_MASK_ANY: C2RustUnnamed_21 = 0;

pub use crate::application::Barconfig;
pub use crate::config::IncludedFile;
pub type C2RustUnnamed_25 = ::core::ffi::c_uint;
pub const P_TOP: C2RustUnnamed_25 = 1;
pub const P_BOTTOM: C2RustUnnamed_25 = 0;
pub use crate::application::Barbinding;
pub type C2RustUnnamed_27 = ::core::ffi::c_uint;
pub const S_SHOW: C2RustUnnamed_27 = 1;
pub const S_HIDE: C2RustUnnamed_27 = 0;
pub type C2RustUnnamed_28 = ::core::ffi::c_uint;
pub const M_INVISIBLE: C2RustUnnamed_28 = 2;
pub const M_HIDE: C2RustUnnamed_28 = 1;
pub const M_DOCK: C2RustUnnamed_28 = 0;
pub use crate::application::tray_output_t;
pub use crate::bindings::Mode;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct i3_ipc_header {
    pub magic: [::core::ffi::c_char; 6],
    pub size: uint32_t,
    pub type_0: uint32_t,
}
pub type i3_ipc_header_t = i3_ipc_header;
pub use crate::application::ipc_client;
pub type handler_t =
    Option<unsafe fn(*mut ipc_client, *mut uint8_t, ::core::ffi::c_int, uint32_t, uint32_t) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ipc_client_head {
    pub tqh_first: *mut ipc_client,
    pub tqh_last: *mut *mut ipc_client,
}
pub type ylength = size_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sync_state {
    pub last_key: *mut ::core::ffi::c_char,
    pub rnd: uint32_t,
    pub window: xcb_window_t,
}
pub use crate::assignments::CommandResult;

pub type sa_family_t = ::core::ffi::c_ushort;

pub type shutdown_reason_t = ::core::ffi::c_uint;
pub const SHUTDOWN_REASON_EXIT: shutdown_reason_t = 1;
pub const SHUTDOWN_REASON_RESTART: shutdown_reason_t = 0;
pub const SHUT_RDWR: C2RustUnnamed_32 = 2;
pub type C2RustUnnamed_32 = ::core::ffi::c_uint;
pub const SHUT_WR: C2RustUnnamed_32 = 1;
pub const SHUT_RD: C2RustUnnamed_32 = 0;
pub const MAJOR_VERSION: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const MINOR_VERSION: ::core::ffi::c_int = 25 as ::core::ffi::c_int;
pub const PATCH_VERSION: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const EINTR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const EAGAIN: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const I3_IPC_REPLY_TYPE_COMMAND: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const I3_IPC_REPLY_TYPE_WORKSPACES: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const I3_IPC_REPLY_TYPE_SUBSCRIBE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const I3_IPC_REPLY_TYPE_OUTPUTS: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const I3_IPC_REPLY_TYPE_TREE: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const I3_IPC_REPLY_TYPE_MARKS: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const I3_IPC_REPLY_TYPE_BAR_CONFIG: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const I3_IPC_REPLY_TYPE_VERSION: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const I3_IPC_REPLY_TYPE_BINDING_MODES: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const I3_IPC_REPLY_TYPE_CONFIG: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const I3_IPC_REPLY_TYPE_TICK: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const I3_IPC_REPLY_TYPE_SYNC: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const I3_IPC_REPLY_TYPE_GET_BINDING_STATE: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const I3_IPC_EVENT_MASK: ::core::ffi::c_ulong =
    (1 as ::core::ffi::c_ulong) << 31 as ::core::ffi::c_int;
pub const I3_IPC_EVENT_WORKSPACE: ::core::ffi::c_ulong =
    I3_IPC_EVENT_MASK | 0 as ::core::ffi::c_ulong;
pub const I3_IPC_EVENT_WINDOW: ::core::ffi::c_ulong = I3_IPC_EVENT_MASK | 3 as ::core::ffi::c_ulong;
pub const I3_IPC_EVENT_BARCONFIG_UPDATE: ::core::ffi::c_ulong =
    I3_IPC_EVENT_MASK | 4 as ::core::ffi::c_ulong;
pub const I3_IPC_EVENT_BINDING: ::core::ffi::c_ulong =
    I3_IPC_EVENT_MASK | 5 as ::core::ffi::c_ulong;
pub const I3_IPC_EVENT_SHUTDOWN: ::core::ffi::c_ulong =
    I3_IPC_EVENT_MASK | 6 as ::core::ffi::c_ulong;
pub const I3_IPC_EVENT_TICK: ::core::ffi::c_ulong = I3_IPC_EVENT_MASK | 7 as ::core::ffi::c_ulong;
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const F_SETFD: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const FD_CLOEXEC: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const __LC_NUMERIC: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SOL_SOCKET: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SO_PEERCRED: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const LC_NUMERIC: ::core::ffi::c_int = __LC_NUMERIC;

pub static mut current_socketpath: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;

pub static mut all_clients: ipc_client_head = ipc_client_head {
    tqh_first: ::core::ptr::null::<ipc_client>() as *mut ipc_client,
    tqh_last: ::core::ptr::null::<*mut ipc_client>() as *mut *mut ipc_client,
};
static mut kill_timeout: ev_tstamp = 10.0f64;

pub unsafe fn ipc_set_kill_timeout(new: ev_tstamp) {
    kill_timeout = new;
}
unsafe fn ipc_push_pending(client: *mut ipc_client) {
    let result: ssize_t = writeall_nonblock(
        (*client).fd,
        (*client).buffer as *const ::core::ffi::c_void,
        (*client).buffer_size,
    ) as ssize_t;
    if result < 0 as ssize_t {
        return;
    }
    if result as size_t == (*client).buffer_size {
        free((*client).buffer as *mut ::core::ffi::c_void);
        (*client).buffer = ::core::ptr::null_mut::<uint8_t>();
        (*client).buffer_size = 0 as size_t;
        if !(*client).timeout.is_null() {
            ev_timer_stop(main_loop, (*client).timeout as *mut ev_timer);
            free((*client).timeout as *mut ::core::ffi::c_void);
            (*client).timeout = ::core::ptr::null_mut::<ev_timer>();
        }
        ev_io_stop(main_loop, (*client).write_callback as *mut ev_io);
        return;
    }
    ev_io_start(main_loop, (*client).write_callback as *mut ev_io);
    if (*client).timeout.is_null() {
        let timeout: *mut ev_timer =
            scalloc(1 as size_t, ::core::mem::size_of::<ev_timer>() as size_t) as *mut ev_timer;
        let ref mut fresh2 = (*(timeout as *mut ::core::ffi::c_void as *mut ev_watcher)).pending;
        *fresh2 = 0 as ::core::ffi::c_int;
        (*(timeout as *mut ::core::ffi::c_void as *mut ev_watcher)).active = *fresh2;
        (*(timeout as *mut ::core::ffi::c_void as *mut ev_watcher)).priority =
            0 as ::core::ffi::c_int;
        (*timeout).cb = Some(
            ipc_client_timeout
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
        (*(timeout as *mut ev_watcher_time)).at = kill_timeout;
        (*timeout).repeat = 0.0f64 as ev_tstamp;
        (*timeout).data = client as *mut ::core::ffi::c_void;
        (*client).timeout = timeout as *mut ev_timer;
        (*(timeout as *mut ::core::ffi::c_void as *mut ev_watcher)).priority =
            if 0x7f as ::core::ffi::c_int & 4 as ::core::ffi::c_int != 0 {
                -(2 as ::core::ffi::c_int)
            } else {
                0 as ::core::ffi::c_int
            };
        ev_timer_start(main_loop, (*client).timeout as *mut ev_timer);
    } else if result > 0 as ssize_t {
        ev_timer_stop(main_loop, (*client).timeout as *mut ev_timer);
        (*((*client).timeout as *mut ev_watcher_time)).at = kill_timeout;
        (*(*client).timeout).repeat = 0.0f64 as ev_tstamp;
        ev_timer_start(main_loop, (*client).timeout as *mut ev_timer);
    }
    if result == 0 as ssize_t {
        return;
    }
    (*client).buffer_size = (*client).buffer_size.wrapping_sub(result as size_t);
    memmove(
        (*client).buffer as *mut ::core::ffi::c_void,
        (*client).buffer.offset(result as isize) as *const ::core::ffi::c_void,
        (*client).buffer_size,
    );
    (*client).buffer = srealloc(
        (*client).buffer as *mut ::core::ffi::c_void,
        (*client).buffer_size,
    ) as *mut uint8_t;
}
unsafe fn ipc_send_client_message(
    client: *mut ipc_client,
    size: size_t,
    message_type: uint32_t,
    payload: *const uint8_t,
) {
    let header: i3_ipc_header_t = i3_ipc_header {
        magic: [
            'i' as i32 as ::core::ffi::c_char,
            '3' as i32 as ::core::ffi::c_char,
            '-' as i32 as ::core::ffi::c_char,
            'i' as i32 as ::core::ffi::c_char,
            'p' as i32 as ::core::ffi::c_char,
            'c' as i32 as ::core::ffi::c_char,
        ],
        size: size as uint32_t,
        type_0: message_type,
    };
    let header_size: size_t = ::core::mem::size_of::<i3_ipc_header_t>() as size_t;
    let message_size: size_t = header_size.wrapping_add(size);
    let push_now: bool = (*client).buffer_size == 0 as size_t;
    (*client).buffer = srealloc(
        (*client).buffer as *mut ::core::ffi::c_void,
        (*client).buffer_size.wrapping_add(message_size),
    ) as *mut uint8_t;
    memcpy(
        (*client).buffer.offset((*client).buffer_size as isize) as *mut ::core::ffi::c_void,
        &raw const header as *mut ::core::ffi::c_void,
        header_size,
    );
    memcpy(
        (*client)
            .buffer
            .offset((*client).buffer_size as isize)
            .offset(header_size as isize) as *mut ::core::ffi::c_void,
        payload as *const ::core::ffi::c_void,
        size,
    );
    (*client).buffer_size = (*client).buffer_size.wrapping_add(message_size);
    if push_now {
        ipc_push_pending(client);
    }
}
unsafe fn free_ipc_client(client: *mut ipc_client, exempt_fd: ::core::ffi::c_int) {
    if (*client).fd != exempt_fd {
        debuglog(
            c"%s:%s:%d - Disconnecting client on fd %d\n"
                .as_ptr()
                .cast_mut(),
            c"../src/ipc.c".as_ptr(),
            c"free_ipc_client".as_ptr(),
            119 as ::core::ffi::c_int,
            (*client).fd,
        );
        close((*client).fd);
    }
    ev_io_stop(main_loop, (*client).read_callback as *mut ev_io);
    free((*client).read_callback as *mut ::core::ffi::c_void);
    (*client).read_callback = ::core::ptr::null_mut::<ev_io>();
    ev_io_stop(main_loop, (*client).write_callback as *mut ev_io);
    free((*client).write_callback as *mut ::core::ffi::c_void);
    (*client).write_callback = ::core::ptr::null_mut::<ev_io>();
    if !(*client).timeout.is_null() {
        ev_timer_stop(main_loop, (*client).timeout as *mut ev_timer);
        free((*client).timeout as *mut ::core::ffi::c_void);
        (*client).timeout = ::core::ptr::null_mut::<ev_timer>();
    }
    free((*client).buffer as *mut ::core::ffi::c_void);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*client).num_events {
        free(*(*client).events.offset(i as isize) as *mut ::core::ffi::c_void);
        i += 1;
    }
    free((*client).events as *mut ::core::ffi::c_void);
    if !(*client).clients.tqe_next.is_null() {
        (*(*client).clients.tqe_next).clients.tqe_prev = (*client).clients.tqe_prev;
    } else {
        all_clients.tqh_last = (*client).clients.tqe_prev;
    }
    *(*client).clients.tqe_prev = (*client).clients.tqe_next;
    free(client as *mut ::core::ffi::c_void);
}

pub unsafe fn ipc_send_event(
    event: *const ::core::ffi::c_char,
    message_type: uint32_t,
    payload: *const ::core::ffi::c_char,
) {
    let mut current: *mut ipc_client = ::core::ptr::null_mut::<ipc_client>();
    current = all_clients.tqh_first as *mut ipc_client;
    while !current.is_null() {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < (*current).num_events {
            if strcasecmp(*(*current).events.offset(i as isize), event) == 0 as ::core::ffi::c_int {
                ipc_send_client_message(
                    current,
                    strlen(payload),
                    message_type,
                    payload as *mut uint8_t,
                );
                break;
            } else {
                i += 1;
            }
        }
        current = (*current).clients.tqe_next as *mut ipc_client;
    }
}
unsafe fn ipc_send_shutdown_event(reason: shutdown_reason_t) {
    let gen: yajl_gen = yajl_gen_alloc(::core::ptr::null::<yajl_alloc_funcs>());
    yajl_gen_map_open(gen);
    yajl_gen_string(
        gen,
        c"change".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"change".as_ptr()),
    );
    if reason as ::core::ffi::c_uint
        == SHUTDOWN_REASON_RESTART as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        yajl_gen_string(
            gen,
            c"restart".as_ptr() as *mut ::core::ffi::c_uchar,
            strlen(c"restart".as_ptr()),
        );
    } else if reason as ::core::ffi::c_uint
        == SHUTDOWN_REASON_EXIT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        yajl_gen_string(
            gen,
            c"exit".as_ptr() as *mut ::core::ffi::c_uchar,
            strlen(c"exit".as_ptr()),
        );
    }
    yajl_gen_map_close(gen);
    let mut payload: *const ::core::ffi::c_uchar = ::core::ptr::null::<::core::ffi::c_uchar>();
    let mut length: ylength = 0;
    yajl_gen_get_buf(gen, &raw mut payload, &raw mut length);
    ipc_send_event(
        c"shutdown".as_ptr(),
        I3_IPC_EVENT_SHUTDOWN as uint32_t,
        payload as *const ::core::ffi::c_char,
    );
    yajl_gen_free(gen);
}

pub unsafe fn ipc_shutdown(reason: shutdown_reason_t, exempt_fd: ::core::ffi::c_int) {
    ipc_send_shutdown_event(reason);
    while !all_clients.tqh_first.is_null() {
        let current: *mut ipc_client = all_clients.tqh_first as *mut ipc_client;
        if (*current).fd != exempt_fd {
            shutdown((*current).fd, SHUT_RDWR as ::core::ffi::c_int);
        }
        free_ipc_client(current, exempt_fd);
    }
}
unsafe fn handle_run_command(
    client: *mut ipc_client,
    message: *mut uint8_t,
    size: ::core::ffi::c_int,
    message_size: uint32_t,
    message_type: uint32_t,
) {
    let command: *mut ::core::ffi::c_char = sstrndup(
        message as *const ::core::ffi::c_char,
        message_size as size_t,
    );
    verboselog(c"IPC: received: *%.4000s*\n".as_ptr().cast_mut(), command);
    let gen: yajl_gen = yajl_gen_alloc(::core::ptr::null::<yajl_alloc_funcs>());
    let result: *mut CommandResult = parse_command(command, gen, client);
    free(command as *mut ::core::ffi::c_void);
    if (*result).needs_tree_render {
        tree_render();
    }
    command_result_free(result);
    let mut reply: *const ::core::ffi::c_uchar = ::core::ptr::null::<::core::ffi::c_uchar>();
    let mut length: ylength = 0;
    yajl_gen_get_buf(gen, &raw mut reply, &raw mut length);
    ipc_send_client_message(
        client,
        length as size_t,
        I3_IPC_REPLY_TYPE_COMMAND as uint32_t,
        reply as *const uint8_t,
    );
    yajl_gen_free(gen);
}
unsafe fn dump_rect(gen: yajl_gen, name: *const ::core::ffi::c_char, r: Rect) {
    yajl_gen_string(gen, name as *mut ::core::ffi::c_uchar, strlen(name));
    yajl_gen_map_open(gen);
    yajl_gen_string(
        gen,
        c"x".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"x".as_ptr()),
    );
    yajl_gen_integer(gen, r.x as int32_t as ::core::ffi::c_longlong);
    yajl_gen_string(
        gen,
        c"y".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"y".as_ptr()),
    );
    yajl_gen_integer(gen, r.y as int32_t as ::core::ffi::c_longlong);
    yajl_gen_string(
        gen,
        c"width".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"width".as_ptr()),
    );
    yajl_gen_integer(gen, r.width as ::core::ffi::c_longlong);
    yajl_gen_string(
        gen,
        c"height".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"height".as_ptr()),
    );
    yajl_gen_integer(gen, r.height as ::core::ffi::c_longlong);
    yajl_gen_map_close(gen);
}
unsafe fn dump_gaps(gen: yajl_gen, name: *const ::core::ffi::c_char, gaps: gaps_t) {
    yajl_gen_string(gen, name as *mut ::core::ffi::c_uchar, strlen(name));
    yajl_gen_map_open(gen);
    yajl_gen_string(
        gen,
        c"inner".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"inner".as_ptr()),
    );
    yajl_gen_integer(gen, gaps.inner as ::core::ffi::c_longlong);
    yajl_gen_string(
        gen,
        c"outer".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"outer".as_ptr()),
    );
    yajl_gen_integer(gen, gaps.top as ::core::ffi::c_longlong);
    yajl_gen_string(
        gen,
        c"top".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"top".as_ptr()),
    );
    yajl_gen_integer(gen, gaps.top as ::core::ffi::c_longlong);
    yajl_gen_string(
        gen,
        c"right".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"right".as_ptr()),
    );
    yajl_gen_integer(gen, gaps.right as ::core::ffi::c_longlong);
    yajl_gen_string(
        gen,
        c"bottom".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"bottom".as_ptr()),
    );
    yajl_gen_integer(gen, gaps.bottom as ::core::ffi::c_longlong);
    yajl_gen_string(
        gen,
        c"left".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"left".as_ptr()),
    );
    yajl_gen_integer(gen, gaps.left as ::core::ffi::c_longlong);
    yajl_gen_map_close(gen);
}
unsafe fn dump_event_state_mask(gen: yajl_gen, bind: *mut Binding) {
    yajl_gen_array_open(gen);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < 20 as ::core::ffi::c_int {
        if (*bind).event_state_mask & ((1 as ::core::ffi::c_int) << i) as i3_event_state_mask_t != 0
        {
            match (1 as ::core::ffi::c_int) << i {
                1 => {
                    yajl_gen_string(
                        gen,
                        c"shift".as_ptr() as *mut ::core::ffi::c_uchar,
                        strlen(c"shift".as_ptr()),
                    );
                }
                2 => {
                    yajl_gen_string(
                        gen,
                        c"lock".as_ptr() as *mut ::core::ffi::c_uchar,
                        strlen(c"lock".as_ptr()),
                    );
                }
                4 => {
                    yajl_gen_string(
                        gen,
                        c"ctrl".as_ptr() as *mut ::core::ffi::c_uchar,
                        strlen(c"ctrl".as_ptr()),
                    );
                }
                8 => {
                    yajl_gen_string(
                        gen,
                        c"Mod1".as_ptr() as *mut ::core::ffi::c_uchar,
                        strlen(c"Mod1".as_ptr()),
                    );
                }
                16 => {
                    yajl_gen_string(
                        gen,
                        c"Mod2".as_ptr() as *mut ::core::ffi::c_uchar,
                        strlen(c"Mod2".as_ptr()),
                    );
                }
                32 => {
                    yajl_gen_string(
                        gen,
                        c"Mod3".as_ptr() as *mut ::core::ffi::c_uchar,
                        strlen(c"Mod3".as_ptr()),
                    );
                }
                64 => {
                    yajl_gen_string(
                        gen,
                        c"Mod4".as_ptr() as *mut ::core::ffi::c_uchar,
                        strlen(c"Mod4".as_ptr()),
                    );
                }
                128 => {
                    yajl_gen_string(
                        gen,
                        c"Mod5".as_ptr() as *mut ::core::ffi::c_uchar,
                        strlen(c"Mod5".as_ptr()),
                    );
                }
                256 => {
                    yajl_gen_string(
                        gen,
                        c"Button1".as_ptr() as *mut ::core::ffi::c_uchar,
                        strlen(c"Button1".as_ptr()),
                    );
                }
                512 => {
                    yajl_gen_string(
                        gen,
                        c"Button2".as_ptr() as *mut ::core::ffi::c_uchar,
                        strlen(c"Button2".as_ptr()),
                    );
                }
                1024 => {
                    yajl_gen_string(
                        gen,
                        c"Button3".as_ptr() as *mut ::core::ffi::c_uchar,
                        strlen(c"Button3".as_ptr()),
                    );
                }
                2048 => {
                    yajl_gen_string(
                        gen,
                        c"Button4".as_ptr() as *mut ::core::ffi::c_uchar,
                        strlen(c"Button4".as_ptr()),
                    );
                }
                4096 => {
                    yajl_gen_string(
                        gen,
                        c"Button5".as_ptr() as *mut ::core::ffi::c_uchar,
                        strlen(c"Button5".as_ptr()),
                    );
                }
                65536 => {
                    yajl_gen_string(
                        gen,
                        c"Group1".as_ptr() as *mut ::core::ffi::c_uchar,
                        strlen(c"Group1".as_ptr()),
                    );
                }
                131072 => {
                    yajl_gen_string(
                        gen,
                        c"Group2".as_ptr() as *mut ::core::ffi::c_uchar,
                        strlen(c"Group2".as_ptr()),
                    );
                }
                262144 => {
                    yajl_gen_string(
                        gen,
                        c"Group3".as_ptr() as *mut ::core::ffi::c_uchar,
                        strlen(c"Group3".as_ptr()),
                    );
                }
                524288 => {
                    yajl_gen_string(
                        gen,
                        c"Group4".as_ptr() as *mut ::core::ffi::c_uchar,
                        strlen(c"Group4".as_ptr()),
                    );
                }
                _ => {}
            }
        }
        i += 1;
    }
    yajl_gen_array_close(gen);
}
unsafe fn dump_binding(gen: yajl_gen, bind: *mut Binding) {
    yajl_gen_map_open(gen);
    yajl_gen_string(
        gen,
        c"input_code".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"input_code".as_ptr()),
    );
    yajl_gen_integer(gen, (*bind).keycode as ::core::ffi::c_longlong);
    yajl_gen_string(
        gen,
        c"input_type".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"input_type".as_ptr()),
    );
    yajl_gen_string(
        gen,
        (if (*bind).input_type as ::core::ffi::c_uint
            == B_KEYBOARD as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            c"keyboard".as_ptr()
        } else {
            c"mouse".as_ptr()
        }) as *mut ::core::ffi::c_uchar,
        strlen(
            if (*bind).input_type as ::core::ffi::c_uint
                == B_KEYBOARD as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                c"keyboard".as_ptr()
            } else {
                c"mouse".as_ptr()
            },
        ),
    );
    yajl_gen_string(
        gen,
        c"symbol".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"symbol".as_ptr()),
    );
    if (*bind).symbol.is_null() {
        yajl_gen_null(gen);
    } else {
        yajl_gen_string(
            gen,
            (*bind).symbol as *mut ::core::ffi::c_uchar,
            strlen((*bind).symbol),
        );
    }
    yajl_gen_string(
        gen,
        c"command".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"command".as_ptr()),
    );
    yajl_gen_string(
        gen,
        (*bind).command as *mut ::core::ffi::c_uchar,
        strlen((*bind).command),
    );
    yajl_gen_string(
        gen,
        c"mods".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"mods".as_ptr()),
    );
    dump_event_state_mask(gen, bind);
    yajl_gen_string(
        gen,
        c"event_state_mask".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"event_state_mask".as_ptr()),
    );
    dump_event_state_mask(gen, bind);
    yajl_gen_map_close(gen);
}

pub unsafe fn dump_node(gen: yajl_gen, con: *mut Con, inplace_restart: bool) {
    yajl_gen_map_open(gen);
    yajl_gen_string(
        gen,
        c"id".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"id".as_ptr()),
    );
    yajl_gen_integer(gen, con as uintptr_t as ::core::ffi::c_longlong);
    yajl_gen_string(
        gen,
        c"type".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"type".as_ptr()),
    );
    match (*con).type_0 as ::core::ffi::c_uint {
        0 => {
            yajl_gen_string(
                gen,
                c"root".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"root".as_ptr()),
            );
        }
        1 => {
            yajl_gen_string(
                gen,
                c"output".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"output".as_ptr()),
            );
        }
        2 => {
            yajl_gen_string(
                gen,
                c"con".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"con".as_ptr()),
            );
        }
        3 => {
            yajl_gen_string(
                gen,
                c"floating_con".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"floating_con".as_ptr()),
            );
        }
        4 => {
            yajl_gen_string(
                gen,
                c"workspace".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"workspace".as_ptr()),
            );
        }
        5 => {
            yajl_gen_string(
                gen,
                c"dockarea".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"dockarea".as_ptr()),
            );
        }
        _ => {}
    }
    yajl_gen_string(
        gen,
        c"orientation".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"orientation".as_ptr()),
    );
    if !con_is_split(con) {
        yajl_gen_string(
            gen,
            c"none".as_ptr() as *mut ::core::ffi::c_uchar,
            strlen(c"none".as_ptr()),
        );
    } else if con_orientation(con) as ::core::ffi::c_uint
        == HORIZ as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        yajl_gen_string(
            gen,
            c"horizontal".as_ptr() as *mut ::core::ffi::c_uchar,
            strlen(c"horizontal".as_ptr()),
        );
    } else {
        yajl_gen_string(
            gen,
            c"vertical".as_ptr() as *mut ::core::ffi::c_uchar,
            strlen(c"vertical".as_ptr()),
        );
    }
    yajl_gen_string(
        gen,
        c"scratchpad_state".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"scratchpad_state".as_ptr()),
    );
    match (*con).scratchpad_state as ::core::ffi::c_uint {
        0 => {
            yajl_gen_string(
                gen,
                c"none".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"none".as_ptr()),
            );
        }
        1 => {
            yajl_gen_string(
                gen,
                c"fresh".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"fresh".as_ptr()),
            );
        }
        2 => {
            yajl_gen_string(
                gen,
                c"changed".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"changed".as_ptr()),
            );
        }
        _ => {}
    }
    yajl_gen_string(
        gen,
        c"percent".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"percent".as_ptr()),
    );
    if (*con).percent == 0.0f64 {
        yajl_gen_null(gen);
    } else {
        yajl_gen_double(gen, (*con).percent);
    }
    yajl_gen_string(
        gen,
        c"urgent".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"urgent".as_ptr()),
    );
    yajl_gen_bool(gen, (*con).urgent as ::core::ffi::c_int);
    yajl_gen_string(
        gen,
        c"marks".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"marks".as_ptr()),
    );
    yajl_gen_array_open(gen);
    let mut mark: *mut mark_t = ::core::ptr::null_mut::<mark_t>();
    mark = (*con).marks_head.tqh_first as *mut mark_t;
    while !mark.is_null() {
        yajl_gen_string(
            gen,
            (*mark).name as *mut ::core::ffi::c_uchar,
            strlen((*mark).name),
        );
        mark = (*mark).marks.tqe_next as *mut mark_t;
    }
    yajl_gen_array_close(gen);
    yajl_gen_string(
        gen,
        c"focused".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"focused".as_ptr()),
    );
    yajl_gen_bool(gen, (con == focused) as ::core::ffi::c_int);
    if (*con).type_0 as ::core::ffi::c_uint != CT_ROOT as ::core::ffi::c_int as ::core::ffi::c_uint
        && (*con).type_0 as ::core::ffi::c_uint
            != CT_OUTPUT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        yajl_gen_string(
            gen,
            c"output".as_ptr() as *mut ::core::ffi::c_uchar,
            strlen(c"output".as_ptr()),
        );
        yajl_gen_string(
            gen,
            (*con_get_output(con)).name as *mut ::core::ffi::c_uchar,
            strlen((*con_get_output(con)).name),
        );
    }
    yajl_gen_string(
        gen,
        c"layout".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"layout".as_ptr()),
    );
    match (*con).layout as ::core::ffi::c_uint {
        0 => {
            debuglog(
                c"%s:%s:%d - About to dump layout=default, this is a bug in the code.\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/ipc.c".as_ptr(),
                c"dump_node".as_ptr(),
                442 as ::core::ffi::c_int,
            );
            if 0 as ::core::ffi::c_int != 0 {
            } else {
                __assert_fail(
                    c"false".as_ptr(),
                    c"../src/ipc.c".as_ptr(),
                    443 as ::core::ffi::c_uint,
                    c"void dump_node(yajl_gen, Con *, _Bool)".as_ptr(),
                );
            };
        }
        5 => {
            yajl_gen_string(
                gen,
                c"splitv".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"splitv".as_ptr()),
            );
        }
        6 => {
            yajl_gen_string(
                gen,
                c"splith".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"splith".as_ptr()),
            );
        }
        1 => {
            yajl_gen_string(
                gen,
                c"stacked".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"stacked".as_ptr()),
            );
        }
        2 => {
            yajl_gen_string(
                gen,
                c"tabbed".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"tabbed".as_ptr()),
            );
        }
        3 => {
            yajl_gen_string(
                gen,
                c"dockarea".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"dockarea".as_ptr()),
            );
        }
        4 => {
            yajl_gen_string(
                gen,
                c"output".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"output".as_ptr()),
            );
        }
        _ => {}
    }
    yajl_gen_string(
        gen,
        c"workspace_layout".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"workspace_layout".as_ptr()),
    );
    match (*con).workspace_layout as ::core::ffi::c_uint {
        0 => {
            yajl_gen_string(
                gen,
                c"default".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"default".as_ptr()),
            );
        }
        1 => {
            yajl_gen_string(
                gen,
                c"stacked".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"stacked".as_ptr()),
            );
        }
        2 => {
            yajl_gen_string(
                gen,
                c"tabbed".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"tabbed".as_ptr()),
            );
        }
        _ => {
            debuglog(
                c"%s:%s:%d - About to dump workspace_layout=%d (none of default/stacked/tabbed), this is a bug.\n".as_ptr().cast_mut(),
                c"../src/ipc.c".as_ptr(),
                c"dump_node".as_ptr(),
                477 as ::core::ffi::c_int,
                (*con).workspace_layout as ::core::ffi::c_uint,
            );
            if 0 as ::core::ffi::c_int != 0 {
            } else {
                __assert_fail(
                    c"false".as_ptr(),
                    c"../src/ipc.c".as_ptr(),
                    478 as ::core::ffi::c_uint,
                    c"void dump_node(yajl_gen, Con *, _Bool)".as_ptr(),
                );
            };
        }
    }
    yajl_gen_string(
        gen,
        c"last_split_layout".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"last_split_layout".as_ptr()),
    );
    match (*con).layout as ::core::ffi::c_uint {
        5 => {
            yajl_gen_string(
                gen,
                c"splitv".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"splitv".as_ptr()),
            );
        }
        _ => {
            yajl_gen_string(
                gen,
                c"splith".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"splith".as_ptr()),
            );
        }
    }
    yajl_gen_string(
        gen,
        c"border".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"border".as_ptr()),
    );
    match (*con).border_style as ::core::ffi::c_uint {
        2 => {
            yajl_gen_string(
                gen,
                c"normal".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"normal".as_ptr()),
            );
        }
        0 => {
            yajl_gen_string(
                gen,
                c"none".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"none".as_ptr()),
            );
        }
        1 => {
            yajl_gen_string(
                gen,
                c"pixel".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"pixel".as_ptr()),
            );
        }
        _ => {}
    }
    yajl_gen_string(
        gen,
        c"current_border_width".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"current_border_width".as_ptr()),
    );
    yajl_gen_integer(gen, (*con).current_border_width as ::core::ffi::c_longlong);
    dump_rect(gen, c"rect".as_ptr(), (*con).rect as Rect);
    if con_draw_decoration_into_frame(con) {
        let mut simulated_deco_rect: Rect = (*con).deco_rect as Rect;
        simulated_deco_rect.x = (*con).rect.x.wrapping_sub((*(*con).parent).rect.x);
        simulated_deco_rect.y = (*con).rect.y.wrapping_sub((*(*con).parent).rect.y);
        dump_rect(gen, c"deco_rect".as_ptr(), simulated_deco_rect);
        dump_rect(gen, c"actual_deco_rect".as_ptr(), (*con).deco_rect as Rect);
    } else {
        dump_rect(gen, c"deco_rect".as_ptr(), (*con).deco_rect as Rect);
    }
    dump_rect(gen, c"window_rect".as_ptr(), (*con).window_rect as Rect);
    dump_rect(gen, c"geometry".as_ptr(), (*con).geometry as Rect);
    yajl_gen_string(
        gen,
        c"name".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"name".as_ptr()),
    );
    if !(*con).window.is_null() && !(*(*con).window).name.is_null() {
        yajl_gen_string(
            gen,
            i3string_as_utf8((*(*con).window).name) as *mut ::core::ffi::c_uchar,
            strlen(i3string_as_utf8((*(*con).window).name)),
        );
    } else if !(*con).name.is_null() {
        yajl_gen_string(
            gen,
            (*con).name as *mut ::core::ffi::c_uchar,
            strlen((*con).name),
        );
    } else {
        yajl_gen_null(gen);
    }
    if !(*con).title_format.is_null() {
        yajl_gen_string(
            gen,
            c"title_format".as_ptr() as *mut ::core::ffi::c_uchar,
            strlen(c"title_format".as_ptr()),
        );
        yajl_gen_string(
            gen,
            (*con).title_format as *mut ::core::ffi::c_uchar,
            strlen((*con).title_format),
        );
    }
    yajl_gen_string(
        gen,
        c"window_icon_padding".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"window_icon_padding".as_ptr()),
    );
    yajl_gen_integer(gen, (*con).window_icon_padding as ::core::ffi::c_longlong);
    if (*con).type_0 as ::core::ffi::c_uint
        == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        yajl_gen_string(
            gen,
            c"num".as_ptr() as *mut ::core::ffi::c_uchar,
            strlen(c"num".as_ptr()),
        );
        yajl_gen_integer(gen, (*con).num as ::core::ffi::c_longlong);
        dump_gaps(gen, c"gaps".as_ptr(), (*con).gaps);
    }
    yajl_gen_string(
        gen,
        c"window".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"window".as_ptr()),
    );
    if !(*con).window.is_null() {
        yajl_gen_integer(gen, (*(*con).window).id as ::core::ffi::c_longlong);
    } else {
        yajl_gen_null(gen);
    }
    yajl_gen_string(
        gen,
        c"window_type".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"window_type".as_ptr()),
    );
    if !(*con).window.is_null() {
        if (*(*con).window).window_type == A__NET_WM_WINDOW_TYPE_NORMAL {
            yajl_gen_string(
                gen,
                c"normal".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"normal".as_ptr()),
            );
        } else if (*(*con).window).window_type == A__NET_WM_WINDOW_TYPE_DOCK {
            yajl_gen_string(
                gen,
                c"dock".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"dock".as_ptr()),
            );
        } else if (*(*con).window).window_type == A__NET_WM_WINDOW_TYPE_DIALOG {
            yajl_gen_string(
                gen,
                c"dialog".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"dialog".as_ptr()),
            );
        } else if (*(*con).window).window_type == A__NET_WM_WINDOW_TYPE_UTILITY {
            yajl_gen_string(
                gen,
                c"utility".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"utility".as_ptr()),
            );
        } else if (*(*con).window).window_type == A__NET_WM_WINDOW_TYPE_TOOLBAR {
            yajl_gen_string(
                gen,
                c"toolbar".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"toolbar".as_ptr()),
            );
        } else if (*(*con).window).window_type == A__NET_WM_WINDOW_TYPE_SPLASH {
            yajl_gen_string(
                gen,
                c"splash".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"splash".as_ptr()),
            );
        } else if (*(*con).window).window_type == A__NET_WM_WINDOW_TYPE_MENU {
            yajl_gen_string(
                gen,
                c"menu".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"menu".as_ptr()),
            );
        } else if (*(*con).window).window_type == A__NET_WM_WINDOW_TYPE_DROPDOWN_MENU {
            yajl_gen_string(
                gen,
                c"dropdown_menu".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"dropdown_menu".as_ptr()),
            );
        } else if (*(*con).window).window_type == A__NET_WM_WINDOW_TYPE_POPUP_MENU {
            yajl_gen_string(
                gen,
                c"popup_menu".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"popup_menu".as_ptr()),
            );
        } else if (*(*con).window).window_type == A__NET_WM_WINDOW_TYPE_TOOLTIP {
            yajl_gen_string(
                gen,
                c"tooltip".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"tooltip".as_ptr()),
            );
        } else if (*(*con).window).window_type == A__NET_WM_WINDOW_TYPE_NOTIFICATION {
            yajl_gen_string(
                gen,
                c"notification".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"notification".as_ptr()),
            );
        } else {
            yajl_gen_string(
                gen,
                c"unknown".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"unknown".as_ptr()),
            );
        }
    } else {
        yajl_gen_null(gen);
    }
    if !(*con).window.is_null() && !inplace_restart {
        yajl_gen_string(
            gen,
            c"window_properties".as_ptr() as *mut ::core::ffi::c_uchar,
            strlen(c"window_properties".as_ptr()),
        );
        yajl_gen_map_open(gen);
        if !(*(*con).window).class_class.is_null() {
            yajl_gen_string(
                gen,
                c"class".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"class".as_ptr()),
            );
            yajl_gen_string(
                gen,
                (*(*con).window).class_class as *mut ::core::ffi::c_uchar,
                strlen((*(*con).window).class_class),
            );
        }
        if !(*(*con).window).class_instance.is_null() {
            yajl_gen_string(
                gen,
                c"instance".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"instance".as_ptr()),
            );
            yajl_gen_string(
                gen,
                (*(*con).window).class_instance as *mut ::core::ffi::c_uchar,
                strlen((*(*con).window).class_instance),
            );
        }
        if !(*(*con).window).role.is_null() {
            yajl_gen_string(
                gen,
                c"window_role".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"window_role".as_ptr()),
            );
            yajl_gen_string(
                gen,
                (*(*con).window).role as *mut ::core::ffi::c_uchar,
                strlen((*(*con).window).role),
            );
        }
        if !(*(*con).window).machine.is_null() {
            yajl_gen_string(
                gen,
                c"machine".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"machine".as_ptr()),
            );
            yajl_gen_string(
                gen,
                (*(*con).window).machine as *mut ::core::ffi::c_uchar,
                strlen((*(*con).window).machine),
            );
        }
        if !(*(*con).window).name.is_null() {
            yajl_gen_string(
                gen,
                c"title".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"title".as_ptr()),
            );
            yajl_gen_string(
                gen,
                i3string_as_utf8((*(*con).window).name) as *mut ::core::ffi::c_uchar,
                strlen(i3string_as_utf8((*(*con).window).name)),
            );
        }
        yajl_gen_string(
            gen,
            c"transient_for".as_ptr() as *mut ::core::ffi::c_uchar,
            strlen(c"transient_for".as_ptr()),
        );
        if (*(*con).window).transient_for as ::core::ffi::c_long == XCB_NONE {
            yajl_gen_null(gen);
        } else {
            yajl_gen_integer(
                gen,
                (*(*con).window).transient_for as ::core::ffi::c_longlong,
            );
        }
        yajl_gen_map_close(gen);
    }
    yajl_gen_string(
        gen,
        c"nodes".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"nodes".as_ptr()),
    );
    yajl_gen_array_open(gen);
    let mut node: *mut Con = ::core::ptr::null_mut::<Con>();
    if (*con).type_0 as ::core::ffi::c_uint
        != CT_DOCKAREA as ::core::ffi::c_int as ::core::ffi::c_uint
        || !inplace_restart
    {
        node = (*con).nodes_head.tqh_first as *mut Con;
        while !node.is_null() {
            dump_node(gen, node, inplace_restart);
            node = (*node).nodes.tqe_next as *mut Con;
        }
    }
    yajl_gen_array_close(gen);
    yajl_gen_string(
        gen,
        c"floating_nodes".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"floating_nodes".as_ptr()),
    );
    yajl_gen_array_open(gen);
    node = (*con).floating_head.tqh_first as *mut Con;
    while !node.is_null() {
        dump_node(gen, node, inplace_restart);
        node = (*node).floating_windows.tqe_next as *mut Con;
    }
    yajl_gen_array_close(gen);
    yajl_gen_string(
        gen,
        c"focus".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"focus".as_ptr()),
    );
    yajl_gen_array_open(gen);
    node = (*con).focus_head.tqh_first as *mut Con;
    while !node.is_null() {
        yajl_gen_integer(gen, node as uintptr_t as ::core::ffi::c_longlong);
        node = (*node).focused.tqe_next as *mut Con;
    }
    yajl_gen_array_close(gen);
    yajl_gen_string(
        gen,
        c"fullscreen_mode".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"fullscreen_mode".as_ptr()),
    );
    yajl_gen_integer(gen, (*con).fullscreen_mode as ::core::ffi::c_longlong);
    yajl_gen_string(
        gen,
        c"sticky".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"sticky".as_ptr()),
    );
    yajl_gen_bool(gen, (*con).sticky as ::core::ffi::c_int);
    yajl_gen_string(
        gen,
        c"floating".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"floating".as_ptr()),
    );
    match (*con).floating as ::core::ffi::c_uint {
        0 => {
            yajl_gen_string(
                gen,
                c"auto_off".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"auto_off".as_ptr()),
            );
        }
        2 => {
            yajl_gen_string(
                gen,
                c"auto_on".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"auto_on".as_ptr()),
            );
        }
        1 => {
            yajl_gen_string(
                gen,
                c"user_off".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"user_off".as_ptr()),
            );
        }
        3 => {
            yajl_gen_string(
                gen,
                c"user_on".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"user_on".as_ptr()),
            );
        }
        _ => {}
    }
    yajl_gen_string(
        gen,
        c"swallows".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"swallows".as_ptr()),
    );
    yajl_gen_array_open(gen);
    let mut match_0: *mut Match = ::core::ptr::null_mut::<Match>();
    match_0 = (*con).swallow_head.tqh_first as *mut Match;
    while !match_0.is_null() {
        if !(*match_0).restart_mode {
            yajl_gen_map_open(gen);
            if (*match_0).dock as ::core::ffi::c_int != M_DONTCHECK as ::core::ffi::c_int {
                yajl_gen_string(
                    gen,
                    c"dock".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"dock".as_ptr()),
                );
                yajl_gen_integer(gen, (*match_0).dock as ::core::ffi::c_longlong);
                yajl_gen_string(
                    gen,
                    c"insert_where".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"insert_where".as_ptr()),
                );
                yajl_gen_integer(gen, (*match_0).insert_where as ::core::ffi::c_longlong);
            }
            if !(*match_0).class.is_null() {
                yajl_gen_string(
                    gen,
                    c"class".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"class".as_ptr()),
                );
                yajl_gen_string(
                    gen,
                    (*(*match_0).class).pattern as *mut ::core::ffi::c_uchar,
                    strlen((*(*match_0).class).pattern),
                );
            }
            if !(*match_0).instance.is_null() {
                yajl_gen_string(
                    gen,
                    c"instance".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"instance".as_ptr()),
                );
                yajl_gen_string(
                    gen,
                    (*(*match_0).instance).pattern as *mut ::core::ffi::c_uchar,
                    strlen((*(*match_0).instance).pattern),
                );
            }
            if !(*match_0).window_role.is_null() {
                yajl_gen_string(
                    gen,
                    c"window_role".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"window_role".as_ptr()),
                );
                yajl_gen_string(
                    gen,
                    (*(*match_0).window_role).pattern as *mut ::core::ffi::c_uchar,
                    strlen((*(*match_0).window_role).pattern),
                );
            }
            if !(*match_0).title.is_null() {
                yajl_gen_string(
                    gen,
                    c"title".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"title".as_ptr()),
                );
                yajl_gen_string(
                    gen,
                    (*(*match_0).title).pattern as *mut ::core::ffi::c_uchar,
                    strlen((*(*match_0).title).pattern),
                );
            }
            if !(*match_0).machine.is_null() {
                yajl_gen_string(
                    gen,
                    c"machine".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"machine".as_ptr()),
                );
                yajl_gen_string(
                    gen,
                    (*(*match_0).machine).pattern as *mut ::core::ffi::c_uchar,
                    strlen((*(*match_0).machine).pattern),
                );
            }
            yajl_gen_map_close(gen);
        }
        match_0 = (*match_0).matches.tqe_next as *mut Match;
    }
    if inplace_restart {
        if !(*con).window.is_null() {
            yajl_gen_map_open(gen);
            yajl_gen_string(
                gen,
                c"id".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"id".as_ptr()),
            );
            yajl_gen_integer(gen, (*(*con).window).id as ::core::ffi::c_longlong);
            yajl_gen_string(
                gen,
                c"restart_mode".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"restart_mode".as_ptr()),
            );
            yajl_gen_bool(gen, true_0);
            yajl_gen_map_close(gen);
        }
    }
    yajl_gen_array_close(gen);
    if inplace_restart as ::core::ffi::c_int != 0 && !(*con).window.is_null() {
        yajl_gen_string(
            gen,
            c"depth".as_ptr() as *mut ::core::ffi::c_uchar,
            strlen(c"depth".as_ptr()),
        );
        yajl_gen_integer(gen, (*con).depth as ::core::ffi::c_longlong);
    }
    if inplace_restart as ::core::ffi::c_int != 0
        && (*con).type_0 as ::core::ffi::c_uint
            == CT_ROOT as ::core::ffi::c_int as ::core::ffi::c_uint
        && !previous_workspace_name.is_null()
    {
        yajl_gen_string(
            gen,
            c"previous_workspace_name".as_ptr() as *mut ::core::ffi::c_uchar,
            strlen(c"previous_workspace_name".as_ptr()),
        );
        yajl_gen_string(
            gen,
            previous_workspace_name as *mut ::core::ffi::c_uchar,
            strlen(previous_workspace_name),
        );
    }
    yajl_gen_map_close(gen);
}
unsafe fn dump_bar_bindings(gen: yajl_gen, config: *mut Barconfig) {
    if (*config).bar_bindings.tqh_first.is_null() {
        return;
    }
    yajl_gen_string(
        gen,
        c"bindings".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"bindings".as_ptr()),
    );
    yajl_gen_array_open(gen);
    let mut current: *mut Barbinding = ::core::ptr::null_mut::<Barbinding>();
    current = (*config).bar_bindings.tqh_first as *mut Barbinding;
    while !current.is_null() {
        yajl_gen_map_open(gen);
        yajl_gen_string(
            gen,
            c"input_code".as_ptr() as *mut ::core::ffi::c_uchar,
            strlen(c"input_code".as_ptr()),
        );
        yajl_gen_integer(gen, (*current).input_code as ::core::ffi::c_longlong);
        yajl_gen_string(
            gen,
            c"command".as_ptr() as *mut ::core::ffi::c_uchar,
            strlen(c"command".as_ptr()),
        );
        yajl_gen_string(
            gen,
            (*current).command as *mut ::core::ffi::c_uchar,
            strlen((*current).command),
        );
        yajl_gen_string(
            gen,
            c"release".as_ptr() as *mut ::core::ffi::c_uchar,
            strlen(c"release".as_ptr()),
        );
        yajl_gen_bool(
            gen,
            ((*current).release as ::core::ffi::c_int == B_UPON_KEYRELEASE as ::core::ffi::c_int)
                as ::core::ffi::c_int,
        );
        yajl_gen_map_close(gen);
        current = (*current).bindings.tqe_next;
    }
    yajl_gen_array_close(gen);
}
unsafe fn canonicalize_output_name(name: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    if strcasecmp(name, c"primary".as_ptr()) == 0 as ::core::ffi::c_int
        || strcasecmp(name, c"nonprimary".as_ptr()) == 0 as ::core::ffi::c_int
    {
        return name;
    }
    let output: *mut Output = get_output_by_name(name, false_0 != 0);
    if !output.is_null() {
        output_primary_name(output)
    } else {
        name
    }
}
unsafe fn dump_bar_config(gen: yajl_gen, config: *mut Barconfig) {
    yajl_gen_map_open(gen);
    yajl_gen_string(
        gen,
        c"id".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"id".as_ptr()),
    );
    yajl_gen_string(
        gen,
        (*config).id as *mut ::core::ffi::c_uchar,
        strlen((*config).id),
    );
    if (*config).num_outputs > 0 as ::core::ffi::c_int {
        yajl_gen_string(
            gen,
            c"outputs".as_ptr() as *mut ::core::ffi::c_uchar,
            strlen(c"outputs".as_ptr()),
        );
        yajl_gen_array_open(gen);
        let mut c: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while c < (*config).num_outputs {
            yajl_gen_string(
                gen,
                canonicalize_output_name(*(*config).outputs.offset(c as isize))
                    as *mut ::core::ffi::c_uchar,
                strlen(canonicalize_output_name(
                    *(*config).outputs.offset(c as isize),
                )),
            );
            c += 1;
        }
        yajl_gen_array_close(gen);
    }
    if !(*config).tray_outputs.tqh_first.is_null() {
        yajl_gen_string(
            gen,
            c"tray_outputs".as_ptr() as *mut ::core::ffi::c_uchar,
            strlen(c"tray_outputs".as_ptr()),
        );
        yajl_gen_array_open(gen);
        let mut tray_output: *mut tray_output_t = ::core::ptr::null_mut::<tray_output_t>();
        tray_output = (*config).tray_outputs.tqh_first as *mut tray_output_t;
        while !tray_output.is_null() {
            yajl_gen_string(
                gen,
                canonicalize_output_name((*tray_output).output) as *mut ::core::ffi::c_uchar,
                strlen(canonicalize_output_name((*tray_output).output)),
            );
            tray_output = (*tray_output).tray_outputs.tqe_next;
        }
        yajl_gen_array_close(gen);
    }
    yajl_gen_string(
        gen,
        c"tray_padding".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"tray_padding".as_ptr()),
    );
    yajl_gen_integer(gen, (*config).tray_padding as ::core::ffi::c_longlong);
    if !(*config).socket_path.is_null() {
        yajl_gen_string(
            gen,
            c"socket_path".as_ptr() as *mut ::core::ffi::c_uchar,
            strlen(c"socket_path".as_ptr()),
        );
        yajl_gen_string(
            gen,
            (*config).socket_path as *mut ::core::ffi::c_uchar,
            strlen((*config).socket_path),
        );
    }
    yajl_gen_string(
        gen,
        c"mode".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"mode".as_ptr()),
    );
    match (*config).mode as ::core::ffi::c_uint {
        1 => {
            yajl_gen_string(
                gen,
                c"hide".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"hide".as_ptr()),
            );
        }
        2 => {
            yajl_gen_string(
                gen,
                c"invisible".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"invisible".as_ptr()),
            );
        }
        0 | _ => {
            yajl_gen_string(
                gen,
                c"dock".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"dock".as_ptr()),
            );
        }
    }
    yajl_gen_string(
        gen,
        c"hidden_state".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"hidden_state".as_ptr()),
    );
    match (*config).hidden_state as ::core::ffi::c_uint {
        1 => {
            yajl_gen_string(
                gen,
                c"show".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"show".as_ptr()),
            );
        }
        0 | _ => {
            yajl_gen_string(
                gen,
                c"hide".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"hide".as_ptr()),
            );
        }
    }
    yajl_gen_string(
        gen,
        c"modifier".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"modifier".as_ptr()),
    );
    yajl_gen_integer(gen, (*config).modifier as ::core::ffi::c_longlong);
    dump_bar_bindings(gen, config);
    yajl_gen_string(
        gen,
        c"position".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"position".as_ptr()),
    );
    if (*config).position as ::core::ffi::c_uint
        == P_BOTTOM as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        yajl_gen_string(
            gen,
            c"bottom".as_ptr() as *mut ::core::ffi::c_uchar,
            strlen(c"bottom".as_ptr()),
        );
    } else {
        yajl_gen_string(
            gen,
            c"top".as_ptr() as *mut ::core::ffi::c_uchar,
            strlen(c"top".as_ptr()),
        );
    }
    if !(*config).status_command.is_null() {
        yajl_gen_string(
            gen,
            c"status_command".as_ptr() as *mut ::core::ffi::c_uchar,
            strlen(c"status_command".as_ptr()),
        );
        yajl_gen_string(
            gen,
            (*config).status_command as *mut ::core::ffi::c_uchar,
            strlen((*config).status_command),
        );
    }
    if !(*config).workspace_command.is_null() {
        yajl_gen_string(
            gen,
            c"workspace_command".as_ptr() as *mut ::core::ffi::c_uchar,
            strlen(c"workspace_command".as_ptr()),
        );
        yajl_gen_string(
            gen,
            (*config).workspace_command as *mut ::core::ffi::c_uchar,
            strlen((*config).workspace_command),
        );
    }
    if !(*config).font.is_null() {
        yajl_gen_string(
            gen,
            c"font".as_ptr() as *mut ::core::ffi::c_uchar,
            strlen(c"font".as_ptr()),
        );
        yajl_gen_string(
            gen,
            (*config).font as *mut ::core::ffi::c_uchar,
            strlen((*config).font),
        );
    }
    if (*config).bar_height != 0 {
        yajl_gen_string(
            gen,
            c"bar_height".as_ptr() as *mut ::core::ffi::c_uchar,
            strlen(c"bar_height".as_ptr()),
        );
        yajl_gen_integer(gen, (*config).bar_height as ::core::ffi::c_longlong);
    }
    dump_rect(gen, c"padding".as_ptr(), (*config).padding as Rect);
    if !(*config).separator_symbol.is_null() {
        yajl_gen_string(
            gen,
            c"separator_symbol".as_ptr() as *mut ::core::ffi::c_uchar,
            strlen(c"separator_symbol".as_ptr()),
        );
        yajl_gen_string(
            gen,
            (*config).separator_symbol as *mut ::core::ffi::c_uchar,
            strlen((*config).separator_symbol),
        );
    }
    yajl_gen_string(
        gen,
        c"workspace_buttons".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"workspace_buttons".as_ptr()),
    );
    yajl_gen_bool(gen, !(*config).hide_workspace_buttons as ::core::ffi::c_int);
    yajl_gen_string(
        gen,
        c"workspace_min_width".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"workspace_min_width".as_ptr()),
    );
    yajl_gen_integer(
        gen,
        (*config).workspace_min_width as ::core::ffi::c_longlong,
    );
    yajl_gen_string(
        gen,
        c"strip_workspace_numbers".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"strip_workspace_numbers".as_ptr()),
    );
    yajl_gen_bool(gen, (*config).strip_workspace_numbers as ::core::ffi::c_int);
    yajl_gen_string(
        gen,
        c"strip_workspace_name".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"strip_workspace_name".as_ptr()),
    );
    yajl_gen_bool(gen, (*config).strip_workspace_name as ::core::ffi::c_int);
    yajl_gen_string(
        gen,
        c"binding_mode_indicator".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"binding_mode_indicator".as_ptr()),
    );
    yajl_gen_bool(
        gen,
        !(*config).hide_binding_mode_indicator as ::core::ffi::c_int,
    );
    yajl_gen_string(
        gen,
        c"verbose".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"verbose".as_ptr()),
    );
    yajl_gen_bool(gen, (*config).verbose as ::core::ffi::c_int);
    yajl_gen_string(
        gen,
        c"colors".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"colors".as_ptr()),
    );
    yajl_gen_map_open(gen);
    if !(*config).colors.background.is_null() {
        yajl_gen_string(
            gen,
            c"background".as_ptr() as *mut ::core::ffi::c_uchar,
            strlen(c"background".as_ptr()),
        );
        yajl_gen_string(
            gen,
            (*config).colors.background as *mut ::core::ffi::c_uchar,
            strlen((*config).colors.background),
        );
    }
    if !(*config).colors.statusline.is_null() {
        yajl_gen_string(
            gen,
            c"statusline".as_ptr() as *mut ::core::ffi::c_uchar,
            strlen(c"statusline".as_ptr()),
        );
        yajl_gen_string(
            gen,
            (*config).colors.statusline as *mut ::core::ffi::c_uchar,
            strlen((*config).colors.statusline),
        );
    }
    if !(*config).colors.separator.is_null() {
        yajl_gen_string(
            gen,
            c"separator".as_ptr() as *mut ::core::ffi::c_uchar,
            strlen(c"separator".as_ptr()),
        );
        yajl_gen_string(
            gen,
            (*config).colors.separator as *mut ::core::ffi::c_uchar,
            strlen((*config).colors.separator),
        );
    }
    if !(*config).colors.focused_background.is_null() {
        yajl_gen_string(
            gen,
            c"focused_background".as_ptr() as *mut ::core::ffi::c_uchar,
            strlen(c"focused_background".as_ptr()),
        );
        yajl_gen_string(
            gen,
            (*config).colors.focused_background as *mut ::core::ffi::c_uchar,
            strlen((*config).colors.focused_background),
        );
    }
    if !(*config).colors.focused_statusline.is_null() {
        yajl_gen_string(
            gen,
            c"focused_statusline".as_ptr() as *mut ::core::ffi::c_uchar,
            strlen(c"focused_statusline".as_ptr()),
        );
        yajl_gen_string(
            gen,
            (*config).colors.focused_statusline as *mut ::core::ffi::c_uchar,
            strlen((*config).colors.focused_statusline),
        );
    }
    if !(*config).colors.focused_separator.is_null() {
        yajl_gen_string(
            gen,
            c"focused_separator".as_ptr() as *mut ::core::ffi::c_uchar,
            strlen(c"focused_separator".as_ptr()),
        );
        yajl_gen_string(
            gen,
            (*config).colors.focused_separator as *mut ::core::ffi::c_uchar,
            strlen((*config).colors.focused_separator),
        );
    }
    if !(*config).colors.focused_workspace_border.is_null() {
        yajl_gen_string(
            gen,
            c"focused_workspace_border".as_ptr() as *mut ::core::ffi::c_uchar,
            strlen(c"focused_workspace_border".as_ptr()),
        );
        yajl_gen_string(
            gen,
            (*config).colors.focused_workspace_border as *mut ::core::ffi::c_uchar,
            strlen((*config).colors.focused_workspace_border),
        );
    }
    if !(*config).colors.focused_workspace_bg.is_null() {
        yajl_gen_string(
            gen,
            c"focused_workspace_bg".as_ptr() as *mut ::core::ffi::c_uchar,
            strlen(c"focused_workspace_bg".as_ptr()),
        );
        yajl_gen_string(
            gen,
            (*config).colors.focused_workspace_bg as *mut ::core::ffi::c_uchar,
            strlen((*config).colors.focused_workspace_bg),
        );
    }
    if !(*config).colors.focused_workspace_text.is_null() {
        yajl_gen_string(
            gen,
            c"focused_workspace_text".as_ptr() as *mut ::core::ffi::c_uchar,
            strlen(c"focused_workspace_text".as_ptr()),
        );
        yajl_gen_string(
            gen,
            (*config).colors.focused_workspace_text as *mut ::core::ffi::c_uchar,
            strlen((*config).colors.focused_workspace_text),
        );
    }
    if !(*config).colors.active_workspace_border.is_null() {
        yajl_gen_string(
            gen,
            c"active_workspace_border".as_ptr() as *mut ::core::ffi::c_uchar,
            strlen(c"active_workspace_border".as_ptr()),
        );
        yajl_gen_string(
            gen,
            (*config).colors.active_workspace_border as *mut ::core::ffi::c_uchar,
            strlen((*config).colors.active_workspace_border),
        );
    }
    if !(*config).colors.active_workspace_bg.is_null() {
        yajl_gen_string(
            gen,
            c"active_workspace_bg".as_ptr() as *mut ::core::ffi::c_uchar,
            strlen(c"active_workspace_bg".as_ptr()),
        );
        yajl_gen_string(
            gen,
            (*config).colors.active_workspace_bg as *mut ::core::ffi::c_uchar,
            strlen((*config).colors.active_workspace_bg),
        );
    }
    if !(*config).colors.active_workspace_text.is_null() {
        yajl_gen_string(
            gen,
            c"active_workspace_text".as_ptr() as *mut ::core::ffi::c_uchar,
            strlen(c"active_workspace_text".as_ptr()),
        );
        yajl_gen_string(
            gen,
            (*config).colors.active_workspace_text as *mut ::core::ffi::c_uchar,
            strlen((*config).colors.active_workspace_text),
        );
    }
    if !(*config).colors.inactive_workspace_border.is_null() {
        yajl_gen_string(
            gen,
            c"inactive_workspace_border".as_ptr() as *mut ::core::ffi::c_uchar,
            strlen(c"inactive_workspace_border".as_ptr()),
        );
        yajl_gen_string(
            gen,
            (*config).colors.inactive_workspace_border as *mut ::core::ffi::c_uchar,
            strlen((*config).colors.inactive_workspace_border),
        );
    }
    if !(*config).colors.inactive_workspace_bg.is_null() {
        yajl_gen_string(
            gen,
            c"inactive_workspace_bg".as_ptr() as *mut ::core::ffi::c_uchar,
            strlen(c"inactive_workspace_bg".as_ptr()),
        );
        yajl_gen_string(
            gen,
            (*config).colors.inactive_workspace_bg as *mut ::core::ffi::c_uchar,
            strlen((*config).colors.inactive_workspace_bg),
        );
    }
    if !(*config).colors.inactive_workspace_text.is_null() {
        yajl_gen_string(
            gen,
            c"inactive_workspace_text".as_ptr() as *mut ::core::ffi::c_uchar,
            strlen(c"inactive_workspace_text".as_ptr()),
        );
        yajl_gen_string(
            gen,
            (*config).colors.inactive_workspace_text as *mut ::core::ffi::c_uchar,
            strlen((*config).colors.inactive_workspace_text),
        );
    }
    if !(*config).colors.urgent_workspace_border.is_null() {
        yajl_gen_string(
            gen,
            c"urgent_workspace_border".as_ptr() as *mut ::core::ffi::c_uchar,
            strlen(c"urgent_workspace_border".as_ptr()),
        );
        yajl_gen_string(
            gen,
            (*config).colors.urgent_workspace_border as *mut ::core::ffi::c_uchar,
            strlen((*config).colors.urgent_workspace_border),
        );
    }
    if !(*config).colors.urgent_workspace_bg.is_null() {
        yajl_gen_string(
            gen,
            c"urgent_workspace_bg".as_ptr() as *mut ::core::ffi::c_uchar,
            strlen(c"urgent_workspace_bg".as_ptr()),
        );
        yajl_gen_string(
            gen,
            (*config).colors.urgent_workspace_bg as *mut ::core::ffi::c_uchar,
            strlen((*config).colors.urgent_workspace_bg),
        );
    }
    if !(*config).colors.urgent_workspace_text.is_null() {
        yajl_gen_string(
            gen,
            c"urgent_workspace_text".as_ptr() as *mut ::core::ffi::c_uchar,
            strlen(c"urgent_workspace_text".as_ptr()),
        );
        yajl_gen_string(
            gen,
            (*config).colors.urgent_workspace_text as *mut ::core::ffi::c_uchar,
            strlen((*config).colors.urgent_workspace_text),
        );
    }
    if !(*config).colors.binding_mode_border.is_null() {
        yajl_gen_string(
            gen,
            c"binding_mode_border".as_ptr() as *mut ::core::ffi::c_uchar,
            strlen(c"binding_mode_border".as_ptr()),
        );
        yajl_gen_string(
            gen,
            (*config).colors.binding_mode_border as *mut ::core::ffi::c_uchar,
            strlen((*config).colors.binding_mode_border),
        );
    }
    if !(*config).colors.binding_mode_bg.is_null() {
        yajl_gen_string(
            gen,
            c"binding_mode_bg".as_ptr() as *mut ::core::ffi::c_uchar,
            strlen(c"binding_mode_bg".as_ptr()),
        );
        yajl_gen_string(
            gen,
            (*config).colors.binding_mode_bg as *mut ::core::ffi::c_uchar,
            strlen((*config).colors.binding_mode_bg),
        );
    }
    if !(*config).colors.binding_mode_text.is_null() {
        yajl_gen_string(
            gen,
            c"binding_mode_text".as_ptr() as *mut ::core::ffi::c_uchar,
            strlen(c"binding_mode_text".as_ptr()),
        );
        yajl_gen_string(
            gen,
            (*config).colors.binding_mode_text as *mut ::core::ffi::c_uchar,
            strlen((*config).colors.binding_mode_text),
        );
    }
    yajl_gen_map_close(gen);
    yajl_gen_map_close(gen);
}
unsafe fn handle_tree(
    client: *mut ipc_client,
    message: *mut uint8_t,
    size: ::core::ffi::c_int,
    message_size: uint32_t,
    message_type: uint32_t,
) {
    let prev_locale: *mut ::core::ffi::c_char = sstrdup(setlocale(
        LC_NUMERIC,
        ::core::ptr::null::<::core::ffi::c_char>(),
    ));
    setlocale(LC_NUMERIC, c"C".as_ptr());
    let gen: yajl_gen = yajl_gen_alloc(::core::ptr::null::<yajl_alloc_funcs>());
    dump_node(gen, croot, false_0 != 0);
    setlocale(LC_NUMERIC, prev_locale);
    free(prev_locale as *mut ::core::ffi::c_void);
    let mut payload: *const ::core::ffi::c_uchar = ::core::ptr::null::<::core::ffi::c_uchar>();
    let mut length: ylength = 0;
    yajl_gen_get_buf(gen, &raw mut payload, &raw mut length);
    ipc_send_client_message(
        client,
        length as size_t,
        I3_IPC_REPLY_TYPE_TREE as uint32_t,
        payload as *const uint8_t,
    );
    yajl_gen_free(gen);
}
unsafe fn handle_get_workspaces(
    client: *mut ipc_client,
    message: *mut uint8_t,
    size: ::core::ffi::c_int,
    message_size: uint32_t,
    message_type: uint32_t,
) {
    let gen: yajl_gen = yajl_gen_alloc(::core::ptr::null::<yajl_alloc_funcs>());
    yajl_gen_array_open(gen);
    let focused_ws: *mut Con = con_get_workspace(focused);
    let mut output: *mut Con = ::core::ptr::null_mut::<Con>();
    output = (*croot).nodes_head.tqh_first as *mut Con;
    while !output.is_null() {
        if !con_is_internal(output) {
            let mut ws: *mut Con = ::core::ptr::null_mut::<Con>();
            ws = (*output_get_content(output)).nodes_head.tqh_first as *mut Con;
            while !ws.is_null() {
                if (*ws).type_0 as ::core::ffi::c_uint
                    == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                } else {
                    __assert_fail(
                        c"ws->type == CT_WORKSPACE".as_ptr(),
                        c"../src/ipc.c".as_ptr(),
                        945 as ::core::ffi::c_uint,
                        c"void handle_get_workspaces(ipc_client *, uint8_t *, int, uint32_t, uint32_t)".as_ptr(),
                    );
                };
                yajl_gen_map_open(gen);
                yajl_gen_string(
                    gen,
                    c"id".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"id".as_ptr()),
                );
                yajl_gen_integer(gen, ws as uintptr_t as ::core::ffi::c_longlong);
                yajl_gen_string(
                    gen,
                    c"num".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"num".as_ptr()),
                );
                yajl_gen_integer(gen, (*ws).num as ::core::ffi::c_longlong);
                yajl_gen_string(
                    gen,
                    c"name".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"name".as_ptr()),
                );
                yajl_gen_string(
                    gen,
                    (*ws).name as *mut ::core::ffi::c_uchar,
                    strlen((*ws).name),
                );
                yajl_gen_string(
                    gen,
                    c"visible".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"visible".as_ptr()),
                );
                yajl_gen_bool(gen, workspace_is_visible(ws) as ::core::ffi::c_int);
                yajl_gen_string(
                    gen,
                    c"focused".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"focused".as_ptr()),
                );
                yajl_gen_bool(gen, (ws == focused_ws) as ::core::ffi::c_int);
                yajl_gen_string(
                    gen,
                    c"rect".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"rect".as_ptr()),
                );
                yajl_gen_map_open(gen);
                yajl_gen_string(
                    gen,
                    c"x".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"x".as_ptr()),
                );
                yajl_gen_integer(gen, (*ws).rect.x as ::core::ffi::c_longlong);
                yajl_gen_string(
                    gen,
                    c"y".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"y".as_ptr()),
                );
                yajl_gen_integer(gen, (*ws).rect.y as ::core::ffi::c_longlong);
                yajl_gen_string(
                    gen,
                    c"width".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"width".as_ptr()),
                );
                yajl_gen_integer(gen, (*ws).rect.width as ::core::ffi::c_longlong);
                yajl_gen_string(
                    gen,
                    c"height".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"height".as_ptr()),
                );
                yajl_gen_integer(gen, (*ws).rect.height as ::core::ffi::c_longlong);
                yajl_gen_map_close(gen);
                yajl_gen_string(
                    gen,
                    c"output".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"output".as_ptr()),
                );
                yajl_gen_string(
                    gen,
                    (*output).name as *mut ::core::ffi::c_uchar,
                    strlen((*output).name),
                );
                yajl_gen_string(
                    gen,
                    c"urgent".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"urgent".as_ptr()),
                );
                yajl_gen_bool(gen, (*ws).urgent as ::core::ffi::c_int);
                yajl_gen_map_close(gen);
                ws = (*ws).nodes.tqe_next as *mut Con;
            }
        }
        output = (*output).nodes.tqe_next as *mut Con;
    }
    yajl_gen_array_close(gen);
    let mut payload: *const ::core::ffi::c_uchar = ::core::ptr::null::<::core::ffi::c_uchar>();
    let mut length: ylength = 0;
    yajl_gen_get_buf(gen, &raw mut payload, &raw mut length);
    ipc_send_client_message(
        client,
        length as size_t,
        I3_IPC_REPLY_TYPE_WORKSPACES as uint32_t,
        payload as *const uint8_t,
    );
    yajl_gen_free(gen);
}
unsafe fn handle_get_outputs(
    client: *mut ipc_client,
    message: *mut uint8_t,
    size: ::core::ffi::c_int,
    message_size: uint32_t,
    message_type: uint32_t,
) {
    let gen: yajl_gen = yajl_gen_alloc(::core::ptr::null::<yajl_alloc_funcs>());
    yajl_gen_array_open(gen);
    let mut output: *mut Output = ::core::ptr::null_mut::<Output>();
    output = outputs.tqh_first as *mut Output;
    while !output.is_null() {
        yajl_gen_map_open(gen);
        yajl_gen_string(
            gen,
            c"name".as_ptr() as *mut ::core::ffi::c_uchar,
            strlen(c"name".as_ptr()),
        );
        yajl_gen_string(
            gen,
            output_primary_name(output) as *mut ::core::ffi::c_uchar,
            strlen(output_primary_name(output)),
        );
        yajl_gen_string(
            gen,
            c"active".as_ptr() as *mut ::core::ffi::c_uchar,
            strlen(c"active".as_ptr()),
        );
        yajl_gen_bool(gen, (*output).active as ::core::ffi::c_int);
        yajl_gen_string(
            gen,
            c"primary".as_ptr() as *mut ::core::ffi::c_uchar,
            strlen(c"primary".as_ptr()),
        );
        yajl_gen_bool(gen, (*output).primary as ::core::ffi::c_int);
        yajl_gen_string(
            gen,
            c"rect".as_ptr() as *mut ::core::ffi::c_uchar,
            strlen(c"rect".as_ptr()),
        );
        yajl_gen_map_open(gen);
        yajl_gen_string(
            gen,
            c"x".as_ptr() as *mut ::core::ffi::c_uchar,
            strlen(c"x".as_ptr()),
        );
        yajl_gen_integer(gen, (*output).rect.x as ::core::ffi::c_longlong);
        yajl_gen_string(
            gen,
            c"y".as_ptr() as *mut ::core::ffi::c_uchar,
            strlen(c"y".as_ptr()),
        );
        yajl_gen_integer(gen, (*output).rect.y as ::core::ffi::c_longlong);
        yajl_gen_string(
            gen,
            c"width".as_ptr() as *mut ::core::ffi::c_uchar,
            strlen(c"width".as_ptr()),
        );
        yajl_gen_integer(gen, (*output).rect.width as ::core::ffi::c_longlong);
        yajl_gen_string(
            gen,
            c"height".as_ptr() as *mut ::core::ffi::c_uchar,
            strlen(c"height".as_ptr()),
        );
        yajl_gen_integer(gen, (*output).rect.height as ::core::ffi::c_longlong);
        yajl_gen_map_close(gen);
        yajl_gen_string(
            gen,
            c"current_workspace".as_ptr() as *mut ::core::ffi::c_uchar,
            strlen(c"current_workspace".as_ptr()),
        );
        let mut ws: *mut Con = ::core::ptr::null_mut::<Con>();
        if !(*output).con.is_null() && {
            ws = con_get_fullscreen_con((*output).con, CF_OUTPUT);
            !ws.is_null()
        } {
            yajl_gen_string(
                gen,
                (*ws).name as *mut ::core::ffi::c_uchar,
                strlen((*ws).name),
            );
        } else {
            yajl_gen_null(gen);
        }
        yajl_gen_map_close(gen);
        output = (*output).outputs.tqe_next as *mut Output;
    }
    yajl_gen_array_close(gen);
    let mut payload: *const ::core::ffi::c_uchar = ::core::ptr::null::<::core::ffi::c_uchar>();
    let mut length: ylength = 0;
    yajl_gen_get_buf(gen, &raw mut payload, &raw mut length);
    ipc_send_client_message(
        client,
        length as size_t,
        I3_IPC_REPLY_TYPE_OUTPUTS as uint32_t,
        payload as *const uint8_t,
    );
    yajl_gen_free(gen);
}
unsafe fn handle_get_marks(
    client: *mut ipc_client,
    message: *mut uint8_t,
    size: ::core::ffi::c_int,
    message_size: uint32_t,
    message_type: uint32_t,
) {
    let gen: yajl_gen = yajl_gen_alloc(::core::ptr::null::<yajl_alloc_funcs>());
    yajl_gen_array_open(gen);
    let mut con: *mut Con = ::core::ptr::null_mut::<Con>();
    con = all_cons.tqh_first as *mut Con;
    while !con.is_null() {
        let mut mark: *mut mark_t = ::core::ptr::null_mut::<mark_t>();
        mark = (*con).marks_head.tqh_first as *mut mark_t;
        while !mark.is_null() {
            yajl_gen_string(
                gen,
                (*mark).name as *mut ::core::ffi::c_uchar,
                strlen((*mark).name),
            );
            mark = (*mark).marks.tqe_next as *mut mark_t;
        }
        con = (*con).all_cons.tqe_next as *mut Con;
    }
    yajl_gen_array_close(gen);
    let mut payload: *const ::core::ffi::c_uchar = ::core::ptr::null::<::core::ffi::c_uchar>();
    let mut length: ylength = 0;
    yajl_gen_get_buf(gen, &raw mut payload, &raw mut length);
    ipc_send_client_message(
        client,
        length as size_t,
        I3_IPC_REPLY_TYPE_MARKS as uint32_t,
        payload as *const uint8_t,
    );
    yajl_gen_free(gen);
}
unsafe fn handle_get_version(
    client: *mut ipc_client,
    message: *mut uint8_t,
    size: ::core::ffi::c_int,
    message_size: uint32_t,
    message_type: uint32_t,
) {
    let gen: yajl_gen = yajl_gen_alloc(::core::ptr::null::<yajl_alloc_funcs>());
    yajl_gen_map_open(gen);
    yajl_gen_string(
        gen,
        c"major".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"major".as_ptr()),
    );
    yajl_gen_integer(gen, MAJOR_VERSION as ::core::ffi::c_longlong);
    yajl_gen_string(
        gen,
        c"minor".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"minor".as_ptr()),
    );
    yajl_gen_integer(gen, MINOR_VERSION as ::core::ffi::c_longlong);
    yajl_gen_string(
        gen,
        c"patch".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"patch".as_ptr()),
    );
    yajl_gen_integer(gen, PATCH_VERSION as ::core::ffi::c_longlong);
    yajl_gen_string(
        gen,
        c"human_readable".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"human_readable".as_ptr()),
    );
    yajl_gen_string(
        gen,
        i3_version as *mut ::core::ffi::c_uchar,
        strlen(i3_version),
    );
    yajl_gen_string(
        gen,
        c"loaded_config_file_name".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"loaded_config_file_name".as_ptr()),
    );
    yajl_gen_string(
        gen,
        current_configpath as *mut ::core::ffi::c_uchar,
        strlen(current_configpath),
    );
    yajl_gen_string(
        gen,
        c"included_config_file_names".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"included_config_file_names".as_ptr()),
    );
    yajl_gen_array_open(gen);
    let mut file: *mut IncludedFile = ::core::ptr::null_mut::<IncludedFile>();
    file = included_files.tqh_first as *mut IncludedFile;
    while !file.is_null() {
        if !(file == included_files.tqh_first) {
            yajl_gen_string(
                gen,
                (*file).path as *mut ::core::ffi::c_uchar,
                strlen((*file).path),
            );
        }
        file = (*file).files.tqe_next as *mut IncludedFile;
    }
    yajl_gen_array_close(gen);
    yajl_gen_map_close(gen);
    let mut payload: *const ::core::ffi::c_uchar = ::core::ptr::null::<::core::ffi::c_uchar>();
    let mut length: ylength = 0;
    yajl_gen_get_buf(gen, &raw mut payload, &raw mut length);
    ipc_send_client_message(
        client,
        length as size_t,
        I3_IPC_REPLY_TYPE_VERSION as uint32_t,
        payload as *const uint8_t,
    );
    yajl_gen_free(gen);
}
unsafe fn handle_get_bar_config(
    client: *mut ipc_client,
    message: *mut uint8_t,
    size: ::core::ffi::c_int,
    message_size: uint32_t,
    message_type: uint32_t,
) {
    let gen: yajl_gen = yajl_gen_alloc(::core::ptr::null::<yajl_alloc_funcs>());
    if message_size == 0 as uint32_t {
        yajl_gen_array_open(gen);
        let mut current: *mut Barconfig = ::core::ptr::null_mut::<Barconfig>();
        current = barconfigs.tqh_first as *mut Barconfig;
        while !current.is_null() {
            yajl_gen_string(
                gen,
                (*current).id as *mut ::core::ffi::c_uchar,
                strlen((*current).id),
            );
            current = (*current).configs.tqe_next as *mut Barconfig;
        }
        yajl_gen_array_close(gen);
        let mut payload: *const ::core::ffi::c_uchar = ::core::ptr::null::<::core::ffi::c_uchar>();
        let mut length: ylength = 0;
        yajl_gen_get_buf(gen, &raw mut payload, &raw mut length);
        ipc_send_client_message(
            client,
            length as size_t,
            I3_IPC_REPLY_TYPE_BAR_CONFIG as uint32_t,
            payload as *const uint8_t,
        );
        yajl_gen_free(gen);
        return;
    }
    let mut bar_id: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    sasprintf(&raw mut bar_id, c"%.*s".as_ptr(), message_size, message);
    verboselog(
        c"IPC: looking for config for bar ID \"%s\"\n"
            .as_ptr()
            .cast_mut(),
        bar_id,
    );
    let mut current_0: *mut Barconfig = ::core::ptr::null_mut::<Barconfig>();
    let mut config: *mut Barconfig = ::core::ptr::null_mut::<Barconfig>();
    current_0 = barconfigs.tqh_first as *mut Barconfig;
    while !current_0.is_null() {
        if strcmp((*current_0).id, bar_id) != 0 as ::core::ffi::c_int {
            current_0 = (*current_0).configs.tqe_next as *mut Barconfig;
        } else {
            config = current_0;
            break;
        }
    }
    free(bar_id as *mut ::core::ffi::c_void);
    if config.is_null() {
        yajl_gen_map_open(gen);
        yajl_gen_string(
            gen,
            c"id".as_ptr() as *mut ::core::ffi::c_uchar,
            strlen(c"id".as_ptr()),
        );
        yajl_gen_null(gen);
        yajl_gen_map_close(gen);
    } else {
        dump_bar_config(gen, config);
    }
    let mut payload_0: *const ::core::ffi::c_uchar = ::core::ptr::null::<::core::ffi::c_uchar>();
    let mut length_0: ylength = 0;
    yajl_gen_get_buf(gen, &raw mut payload_0, &raw mut length_0);
    ipc_send_client_message(
        client,
        length_0 as size_t,
        I3_IPC_REPLY_TYPE_BAR_CONFIG as uint32_t,
        payload_0 as *const uint8_t,
    );
    yajl_gen_free(gen);
}
unsafe fn handle_get_binding_modes(
    client: *mut ipc_client,
    message: *mut uint8_t,
    size: ::core::ffi::c_int,
    message_size: uint32_t,
    message_type: uint32_t,
) {
    let gen: yajl_gen = yajl_gen_alloc(::core::ptr::null::<yajl_alloc_funcs>());
    yajl_gen_array_open(gen);
    let mut mode: *mut Mode = ::core::ptr::null_mut::<Mode>();
    mode = modes.slh_first as *mut Mode;
    while !mode.is_null() {
        yajl_gen_string(
            gen,
            (*mode).name as *mut ::core::ffi::c_uchar,
            strlen((*mode).name),
        );
        mode = (*mode).modes.sle_next;
    }
    yajl_gen_array_close(gen);
    let mut payload: *const ::core::ffi::c_uchar = ::core::ptr::null::<::core::ffi::c_uchar>();
    let mut length: ylength = 0;
    yajl_gen_get_buf(gen, &raw mut payload, &raw mut length);
    ipc_send_client_message(
        client,
        length as size_t,
        I3_IPC_REPLY_TYPE_BINDING_MODES as uint32_t,
        payload as *const uint8_t,
    );
    yajl_gen_free(gen);
}
unsafe fn add_subscription(
    extra: *mut ::core::ffi::c_void,
    s: *const ::core::ffi::c_uchar,
    len: ylength,
) -> ::core::ffi::c_int {
    let client: *mut ipc_client = extra as *mut ipc_client;
    debuglog(
        c"%s:%s:%d - should add subscription to extra %p, sub %.*s\n"
            .as_ptr()
            .cast_mut(),
        c"../src/ipc.c".as_ptr(),
        c"add_subscription".as_ptr(),
        1214 as ::core::ffi::c_int,
        client,
        len as ::core::ffi::c_int,
        s,
    );
    let event: ::core::ffi::c_int = (*client).num_events;
    (*client).num_events += 1;
    (*client).events = srealloc(
        (*client).events as *mut ::core::ffi::c_void,
        ((*client).num_events as size_t)
            .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t),
    ) as *mut *mut ::core::ffi::c_char;
    let ref mut fresh3 = *(*client).events.offset(event as isize);
    *fresh3 =
        scalloc((len as size_t).wrapping_add(1 as size_t), 1 as size_t) as *mut ::core::ffi::c_char;
    memcpy(
        *(*client).events.offset(event as isize) as *mut ::core::ffi::c_void,
        s as *const ::core::ffi::c_void,
        len as size_t,
    );
    debuglog(
        c"%s:%s:%d - client is now subscribed to:\n"
            .as_ptr()
            .cast_mut(),
        c"../src/ipc.c".as_ptr(),
        c"add_subscription".as_ptr(),
        1224 as ::core::ffi::c_int,
    );
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*client).num_events {
        debuglog(
            c"%s:%s:%d - event %s\n".as_ptr().cast_mut(),
            c"../src/ipc.c".as_ptr(),
            c"add_subscription".as_ptr(),
            1226 as ::core::ffi::c_int,
            *(*client).events.offset(i as isize),
        );
        i += 1;
    }
    debuglog(
        c"%s:%s:%d - (done)\n".as_ptr().cast_mut(),
        c"../src/ipc.c".as_ptr(),
        c"add_subscription".as_ptr(),
        1228 as ::core::ffi::c_int,
    );
    1 as ::core::ffi::c_int
}
unsafe fn handle_subscribe(
    client: *mut ipc_client,
    message: *mut uint8_t,
    size: ::core::ffi::c_int,
    message_size: uint32_t,
    message_type: uint32_t,
) {
    static mut callbacks: yajl_callbacks = unsafe {
        yajl_callbacks {
            yajl_null: None,
            yajl_boolean: None,
            yajl_integer: None,
            yajl_double: None,
            yajl_number: None,
            yajl_string: Some(
                add_subscription
                    as unsafe fn(
                        *mut ::core::ffi::c_void,
                        *const ::core::ffi::c_uchar,
                        ylength,
                    ) -> ::core::ffi::c_int,
            ),
            yajl_start_map: None,
            yajl_map_key: None,
            yajl_end_map: None,
            yajl_start_array: None,
            yajl_end_array: None,
        }
    };
    let p: yajl_handle = yajl_alloc(
        &raw mut callbacks,
        ::core::ptr::null_mut::<yajl_alloc_funcs>(),
        client as *mut ::core::ffi::c_void,
    ) as yajl_handle;
    let stat: yajl_status = yajl_parse(p, message, message_size as size_t) as yajl_status;
    if stat as ::core::ffi::c_uint != yajl_status_ok as ::core::ffi::c_int as ::core::ffi::c_uint {
        let err: *mut ::core::ffi::c_uchar =
            yajl_get_error(p, true_0, message, message_size as size_t);
        errorlog(c"ERROR: YAJL parse error: %s\n".as_ptr().cast_mut(), err);
        yajl_free_error(p, err);
        let reply: *const ::core::ffi::c_char = c"{\"success\":false}".as_ptr();
        ipc_send_client_message(
            client,
            strlen(reply),
            I3_IPC_REPLY_TYPE_SUBSCRIBE as uint32_t,
            reply as *const uint8_t,
        );
        yajl_free(p);
        return;
    }
    yajl_free(p);
    let reply_0: *const ::core::ffi::c_char = c"{\"success\":true}".as_ptr();
    ipc_send_client_message(
        client,
        strlen(reply_0),
        I3_IPC_REPLY_TYPE_SUBSCRIBE as uint32_t,
        reply_0 as *const uint8_t,
    );
    if (*client).first_tick_sent {
        return;
    }
    let mut is_tick: bool = false_0 != 0;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*client).num_events {
        if strcmp(*(*client).events.offset(i as isize), c"tick".as_ptr()) == 0 as ::core::ffi::c_int
        {
            is_tick = true_0 != 0;
            break;
        } else {
            i += 1;
        }
    }
    if !is_tick {
        return;
    }
    (*client).first_tick_sent = true_0 != 0;
    let payload: *const ::core::ffi::c_char = c"{\"first\":true,\"payload\":\"\"}".as_ptr();
    ipc_send_client_message(
        client,
        strlen(payload),
        I3_IPC_EVENT_TICK as uint32_t,
        payload as *const uint8_t,
    );
}
unsafe fn handle_get_config(
    client: *mut ipc_client,
    message: *mut uint8_t,
    size: ::core::ffi::c_int,
    message_size: uint32_t,
    message_type: uint32_t,
) {
    let gen: yajl_gen = yajl_gen_alloc(::core::ptr::null::<yajl_alloc_funcs>());
    yajl_gen_map_open(gen);
    yajl_gen_string(
        gen,
        c"config".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"config".as_ptr()),
    );
    let mut file: *mut IncludedFile = included_files.tqh_first as *mut IncludedFile;
    yajl_gen_string(
        gen,
        (*file).raw_contents as *mut ::core::ffi::c_uchar,
        strlen((*file).raw_contents),
    );
    yajl_gen_string(
        gen,
        c"included_configs".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"included_configs".as_ptr()),
    );
    yajl_gen_array_open(gen);
    file = included_files.tqh_first as *mut IncludedFile;
    while !file.is_null() {
        yajl_gen_map_open(gen);
        yajl_gen_string(
            gen,
            c"path".as_ptr() as *mut ::core::ffi::c_uchar,
            strlen(c"path".as_ptr()),
        );
        yajl_gen_string(
            gen,
            (*file).path as *mut ::core::ffi::c_uchar,
            strlen((*file).path),
        );
        yajl_gen_string(
            gen,
            c"raw_contents".as_ptr() as *mut ::core::ffi::c_uchar,
            strlen(c"raw_contents".as_ptr()),
        );
        yajl_gen_string(
            gen,
            (*file).raw_contents as *mut ::core::ffi::c_uchar,
            strlen((*file).raw_contents),
        );
        yajl_gen_string(
            gen,
            c"variable_replaced_contents".as_ptr() as *mut ::core::ffi::c_uchar,
            strlen(c"variable_replaced_contents".as_ptr()),
        );
        yajl_gen_string(
            gen,
            (*file).variable_replaced_contents as *mut ::core::ffi::c_uchar,
            strlen((*file).variable_replaced_contents),
        );
        yajl_gen_map_close(gen);
        file = (*file).files.tqe_next as *mut IncludedFile;
    }
    yajl_gen_array_close(gen);
    yajl_gen_map_close(gen);
    let mut payload: *const ::core::ffi::c_uchar = ::core::ptr::null::<::core::ffi::c_uchar>();
    let mut length: ylength = 0;
    yajl_gen_get_buf(gen, &raw mut payload, &raw mut length);
    ipc_send_client_message(
        client,
        length as size_t,
        I3_IPC_REPLY_TYPE_CONFIG as uint32_t,
        payload as *const uint8_t,
    );
    yajl_gen_free(gen);
}
unsafe fn handle_send_tick(
    client: *mut ipc_client,
    message: *mut uint8_t,
    size: ::core::ffi::c_int,
    message_size: uint32_t,
    message_type: uint32_t,
) {
    let gen: yajl_gen = yajl_gen_alloc(::core::ptr::null::<yajl_alloc_funcs>());
    yajl_gen_map_open(gen);
    yajl_gen_string(
        gen,
        c"first".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"first".as_ptr()),
    );
    yajl_gen_bool(gen, false_0);
    yajl_gen_string(
        gen,
        c"payload".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"payload".as_ptr()),
    );
    yajl_gen_string(
        gen,
        message as *mut ::core::ffi::c_uchar,
        message_size as size_t,
    );
    yajl_gen_map_close(gen);
    let mut payload: *const ::core::ffi::c_uchar = ::core::ptr::null::<::core::ffi::c_uchar>();
    let mut length: ylength = 0;
    yajl_gen_get_buf(gen, &raw mut payload, &raw mut length);
    ipc_send_event(
        c"tick".as_ptr(),
        I3_IPC_EVENT_TICK as uint32_t,
        payload as *const ::core::ffi::c_char,
    );
    yajl_gen_free(gen);
    let reply: *const ::core::ffi::c_char = c"{\"success\":true}".as_ptr();
    ipc_send_client_message(
        client,
        strlen(reply),
        I3_IPC_REPLY_TYPE_TICK as uint32_t,
        reply as *const uint8_t,
    );
    debuglog(
        c"%s:%s:%d - Sent tick event\n".as_ptr().cast_mut(),
        c"../src/ipc.c".as_ptr(),
        c"handle_send_tick".as_ptr(),
        1342 as ::core::ffi::c_int,
    );
}
unsafe fn _sync_json_key(
    extra: *mut ::core::ffi::c_void,
    val: *const ::core::ffi::c_uchar,
    len: size_t,
) -> ::core::ffi::c_int {
    let state: *mut sync_state = extra as *mut sync_state;
    free((*state).last_key as *mut ::core::ffi::c_void);
    (*state).last_key = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*state).last_key =
        scalloc(len.wrapping_add(1 as size_t), 1 as size_t) as *mut ::core::ffi::c_char;
    memcpy(
        (*state).last_key as *mut ::core::ffi::c_void,
        val as *const ::core::ffi::c_void,
        len,
    );
    1 as ::core::ffi::c_int
}
unsafe fn _sync_json_int(
    extra: *mut ::core::ffi::c_void,
    val: ::core::ffi::c_longlong,
) -> ::core::ffi::c_int {
    let state: *mut sync_state = extra as *mut sync_state;
    if strcasecmp((*state).last_key, c"rnd".as_ptr()) == 0 as ::core::ffi::c_int {
        (*state).rnd = val as uint32_t;
    } else if strcasecmp((*state).last_key, c"window".as_ptr()) == 0 as ::core::ffi::c_int {
        (*state).window = val as xcb_window_t;
    }
    1 as ::core::ffi::c_int
}
unsafe fn handle_sync(
    client: *mut ipc_client,
    message: *mut uint8_t,
    size: ::core::ffi::c_int,
    message_size: uint32_t,
    message_type: uint32_t,
) {
    static mut callbacks: yajl_callbacks = unsafe {
        yajl_callbacks {
            yajl_null: None,
            yajl_boolean: None,
            yajl_integer: Some(
                _sync_json_int
                    as unsafe fn(
                        *mut ::core::ffi::c_void,
                        ::core::ffi::c_longlong,
                    ) -> ::core::ffi::c_int,
            ),
            yajl_double: None,
            yajl_number: None,
            yajl_string: None,
            yajl_start_map: None,
            yajl_map_key: Some(
                _sync_json_key
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
    let mut state: sync_state = sync_state {
        last_key: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        rnd: 0,
        window: 0,
    };
    let p: yajl_handle = yajl_alloc(
        &raw mut callbacks,
        ::core::ptr::null_mut::<yajl_alloc_funcs>(),
        &raw mut state as *mut ::core::ffi::c_void,
    );
    let stat: yajl_status = yajl_parse(p, message, message_size as size_t);
    free(state.last_key as *mut ::core::ffi::c_void);
    state.last_key = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if stat as ::core::ffi::c_uint != yajl_status_ok as ::core::ffi::c_int as ::core::ffi::c_uint {
        let err: *mut ::core::ffi::c_uchar =
            yajl_get_error(p, true_0, message, message_size as size_t);
        errorlog(c"ERROR: YAJL parse error: %s\n".as_ptr().cast_mut(), err);
        yajl_free_error(p, err);
        let reply: *const ::core::ffi::c_char = c"{\"success\":false}".as_ptr();
        ipc_send_client_message(
            client,
            strlen(reply),
            I3_IPC_REPLY_TYPE_SYNC as uint32_t,
            reply as *const uint8_t,
        );
        yajl_free(p);
        return;
    }
    yajl_free(p);
    debuglog(
        c"%s:%s:%d - received IPC sync request (rnd = %d, window = 0x%08x)\n"
            .as_ptr()
            .cast_mut(),
        c"../src/ipc.c".as_ptr(),
        c"handle_sync".as_ptr(),
        1392 as ::core::ffi::c_int,
        state.rnd,
        state.window,
    );
    sync_respond(state.window, state.rnd);
    let reply_0: *const ::core::ffi::c_char = c"{\"success\":true}".as_ptr();
    ipc_send_client_message(
        client,
        strlen(reply_0),
        I3_IPC_REPLY_TYPE_SYNC as uint32_t,
        reply_0 as *const uint8_t,
    );
}
unsafe fn handle_get_binding_state(
    client: *mut ipc_client,
    message: *mut uint8_t,
    size: ::core::ffi::c_int,
    message_size: uint32_t,
    message_type: uint32_t,
) {
    let gen: yajl_gen = yajl_gen_alloc(::core::ptr::null::<yajl_alloc_funcs>());
    yajl_gen_map_open(gen);
    yajl_gen_string(
        gen,
        c"name".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"name".as_ptr()),
    );
    yajl_gen_string(
        gen,
        current_binding_mode as *mut ::core::ffi::c_uchar,
        strlen(current_binding_mode),
    );
    yajl_gen_map_close(gen);
    let mut payload: *const ::core::ffi::c_uchar = ::core::ptr::null::<::core::ffi::c_uchar>();
    let mut length: ylength = 0;
    yajl_gen_get_buf(gen, &raw mut payload, &raw mut length);
    ipc_send_client_message(
        client,
        length as size_t,
        I3_IPC_REPLY_TYPE_GET_BINDING_STATE as uint32_t,
        payload as *const uint8_t,
    );
    yajl_gen_free(gen);
}

pub static mut handlers: [handler_t; 13] = unsafe {
    [
        Some(
            handle_run_command
                as unsafe fn(
                    *mut ipc_client,
                    *mut uint8_t,
                    ::core::ffi::c_int,
                    uint32_t,
                    uint32_t,
                ) -> (),
        ),
        Some(
            handle_get_workspaces
                as unsafe fn(
                    *mut ipc_client,
                    *mut uint8_t,
                    ::core::ffi::c_int,
                    uint32_t,
                    uint32_t,
                ) -> (),
        ),
        Some(
            handle_subscribe
                as unsafe fn(
                    *mut ipc_client,
                    *mut uint8_t,
                    ::core::ffi::c_int,
                    uint32_t,
                    uint32_t,
                ) -> (),
        ),
        Some(
            handle_get_outputs
                as unsafe fn(
                    *mut ipc_client,
                    *mut uint8_t,
                    ::core::ffi::c_int,
                    uint32_t,
                    uint32_t,
                ) -> (),
        ),
        Some(
            handle_tree
                as unsafe fn(
                    *mut ipc_client,
                    *mut uint8_t,
                    ::core::ffi::c_int,
                    uint32_t,
                    uint32_t,
                ) -> (),
        ),
        Some(
            handle_get_marks
                as unsafe fn(
                    *mut ipc_client,
                    *mut uint8_t,
                    ::core::ffi::c_int,
                    uint32_t,
                    uint32_t,
                ) -> (),
        ),
        Some(
            handle_get_bar_config
                as unsafe fn(
                    *mut ipc_client,
                    *mut uint8_t,
                    ::core::ffi::c_int,
                    uint32_t,
                    uint32_t,
                ) -> (),
        ),
        Some(
            handle_get_version
                as unsafe fn(
                    *mut ipc_client,
                    *mut uint8_t,
                    ::core::ffi::c_int,
                    uint32_t,
                    uint32_t,
                ) -> (),
        ),
        Some(
            handle_get_binding_modes
                as unsafe fn(
                    *mut ipc_client,
                    *mut uint8_t,
                    ::core::ffi::c_int,
                    uint32_t,
                    uint32_t,
                ) -> (),
        ),
        Some(
            handle_get_config
                as unsafe fn(
                    *mut ipc_client,
                    *mut uint8_t,
                    ::core::ffi::c_int,
                    uint32_t,
                    uint32_t,
                ) -> (),
        ),
        Some(
            handle_send_tick
                as unsafe fn(
                    *mut ipc_client,
                    *mut uint8_t,
                    ::core::ffi::c_int,
                    uint32_t,
                    uint32_t,
                ) -> (),
        ),
        Some(
            handle_sync
                as unsafe fn(
                    *mut ipc_client,
                    *mut uint8_t,
                    ::core::ffi::c_int,
                    uint32_t,
                    uint32_t,
                ) -> (),
        ),
        Some(
            handle_get_binding_state
                as unsafe fn(
                    *mut ipc_client,
                    *mut uint8_t,
                    ::core::ffi::c_int,
                    uint32_t,
                    uint32_t,
                ) -> (),
        ),
    ]
};
unsafe extern "C" fn ipc_receive_message(
    loop_0: *mut ev_loop,
    w: *mut ev_io,
    revents: ::core::ffi::c_int,
) {
    let mut message_type: uint32_t = 0;
    let mut message_length: uint32_t = 0;
    let mut message: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let client: *mut ipc_client = (*w).data as *mut ipc_client;
    if (*client).fd == (*w).fd {
    } else {
        __assert_fail(
            c"client->fd == w->fd".as_ptr(),
            c"../src/ipc.c".as_ptr(),
            1449 as ::core::ffi::c_uint,
            c"void ipc_receive_message(struct ev_loop *, ev_io *, int)".as_ptr(),
        );
    };
    let ret: ::core::ffi::c_int = ipc_recv_message(
        (*w).fd,
        &raw mut message_type,
        &raw mut message_length,
        &raw mut message,
    );
    if ret < 0 as ::core::ffi::c_int {
        if ret == -(1 as ::core::ffi::c_int) && *__errno_location() == EAGAIN {
            free(message as *mut ::core::ffi::c_void);
            message = ::core::ptr::null_mut::<uint8_t>();
            return;
        }
        free_ipc_client(client, -(1 as ::core::ffi::c_int));
        free(message as *mut ::core::ffi::c_void);
        message = ::core::ptr::null_mut::<uint8_t>();
        return;
    }
    if message_type as usize
        >= (::core::mem::size_of::<[handler_t; 13]>() as usize)
            .wrapping_div(::core::mem::size_of::<handler_t>() as usize)
    {
        debuglog(
            c"%s:%s:%d - Unhandled message type: %d\n"
                .as_ptr()
                .cast_mut(),
            c"../src/ipc.c".as_ptr(),
            c"ipc_receive_message".as_ptr(),
            1468 as ::core::ffi::c_int,
            message_type,
        );
    } else {
        let h: handler_t = handlers[message_type as usize];
        h.expect("non-null function pointer")(
            client,
            message,
            0 as ::core::ffi::c_int,
            message_length,
            message_type,
        );
    }
    free(message as *mut ::core::ffi::c_void);
    message = ::core::ptr::null_mut::<uint8_t>();
}
unsafe extern "C" fn ipc_client_timeout(
    loop_0: *mut ev_loop,
    w: *mut ev_timer,
    revents: ::core::ffi::c_int,
) {
    let mut exepath: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut fd: ::core::ffi::c_int = 0;
    let mut buf: [::core::ffi::c_char; 512] = [0; 512];
    let mut n: ssize_t = 0;
    let client: *mut ipc_client = (*w).data as *mut ipc_client;
    let mut cmdline: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut peercred: ucred = ucred {
        pid: 0,
        uid: 0,
        gid: 0,
    };
    let mut so_len: socklen_t = ::core::mem::size_of::<ucred>() as socklen_t;
    if !(getsockopt(
        (*client).fd,
        SOL_SOCKET,
        SO_PEERCRED,
        &raw mut peercred as *mut ::core::ffi::c_void,
        &raw mut so_len,
    ) != 0 as ::core::ffi::c_int)
    {
        exepath = ::core::ptr::null_mut::<::core::ffi::c_char>();
        sasprintf(&raw mut exepath, c"/proc/%d/cmdline".as_ptr(), peercred.pid);
        fd = open(exepath, O_RDONLY);
        free(exepath as *mut ::core::ffi::c_void);
        if !(fd == -(1 as ::core::ffi::c_int)) {
            buf = [
                '\0' as i32 as ::core::ffi::c_char,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ];
            n = read(
                fd,
                &raw mut buf as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>() as size_t,
            ) as ssize_t;
            close(fd);
            if !(n < 0 as ssize_t) {
                let mut walk: *mut ::core::ffi::c_char = &raw mut buf as *mut ::core::ffi::c_char;
                while walk
                    < (&raw mut buf as *mut ::core::ffi::c_char)
                        .offset(n as isize)
                        .offset(-(1 as ::core::ffi::c_int as isize))
                {
                    if *walk as ::core::ffi::c_int == '\0' as i32 {
                        *walk = ' ' as i32 as ::core::ffi::c_char;
                    }
                    walk = walk.offset(1);
                }
                cmdline = &raw mut buf as *mut ::core::ffi::c_char;
                if !cmdline.is_null() {
                    errorlog(
                        c"ERROR: client %p with pid %d and cmdline '%s' on fd %d timed out, killing\n".as_ptr().cast_mut(),
                        client,
                        peercred.pid,
                        cmdline,
                        (*client).fd,
                    );
                }
            }
        }
    }
    if cmdline.is_null() {
        errorlog(
            c"ERROR: client %p on fd %d timed out, killing\n"
                .as_ptr()
                .cast_mut(),
            client,
            (*client).fd,
        );
    }
    free_ipc_client(client, -(1 as ::core::ffi::c_int));
}
unsafe extern "C" fn ipc_socket_writeable_cb(
    loop_0: *mut ev_loop,
    w: *mut ev_io,
    revents: ::core::ffi::c_int,
) {
    debuglog(
        c"%s:%s:%d - fd %d writeable\n".as_ptr().cast_mut(),
        c"../src/ipc.c".as_ptr(),
        c"ipc_socket_writeable_cb".as_ptr(),
        1524 as ::core::ffi::c_int,
        (*w).fd,
    );
    let client: *mut ipc_client = (*w).data as *mut ipc_client;
    if (*client).timeout != ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut ev_timer {
    } else {
        __assert_fail(
            c"client->timeout != NULL".as_ptr(),
            c"../src/ipc.c".as_ptr(),
            1529 as ::core::ffi::c_uint,
            c"void ipc_socket_writeable_cb(struct ev_loop *, ev_io *, int)".as_ptr(),
        );
    };
    ipc_push_pending(client);
}

pub unsafe extern "C" fn ipc_new_client(
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
    ipc_new_client_on_fd(loop_0, fd);
}

pub unsafe fn ipc_new_client_on_fd(
    loop_0: *mut ev_loop,
    fd: ::core::ffi::c_int,
) -> *mut ipc_client {
    set_nonblock(fd);
    let client: *mut ipc_client =
        scalloc(1 as size_t, ::core::mem::size_of::<ipc_client>() as size_t) as *mut ipc_client;
    (*client).fd = fd;
    (*client).read_callback =
        scalloc(1 as size_t, ::core::mem::size_of::<ev_io>() as size_t) as *mut ev_io;
    (*(*client).read_callback).data = client as *mut ::core::ffi::c_void;
    let ref mut fresh0 =
        (*((*client).read_callback as *mut ::core::ffi::c_void as *mut ev_watcher)).pending;
    *fresh0 = 0 as ::core::ffi::c_int;
    (*((*client).read_callback as *mut ::core::ffi::c_void as *mut ev_watcher)).active = *fresh0;
    (*((*client).read_callback as *mut ::core::ffi::c_void as *mut ev_watcher)).priority =
        0 as ::core::ffi::c_int;
    (*(*client).read_callback).cb = Some(
        ipc_receive_message
            as unsafe extern "C" fn(*mut ev_loop, *mut ev_io, ::core::ffi::c_int) -> (),
    )
        as Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_io, ::core::ffi::c_int) -> ()>;
    memmove(
        &raw mut (*((*client).read_callback as *mut ev_watcher)).cb as *mut ::core::ffi::c_void,
        &raw mut (*(*client).read_callback).cb as *const ::core::ffi::c_void,
        ::core::mem::size_of::<
            Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_io, ::core::ffi::c_int) -> ()>,
        >() as size_t,
    );
    (*(*client).read_callback).fd = fd;
    (*(*client).read_callback).events =
        EV_READ as ::core::ffi::c_int | EV__IOFDSET as ::core::ffi::c_int;
    ev_io_start(loop_0, (*client).read_callback as *mut ev_io);
    (*client).write_callback =
        scalloc(1 as size_t, ::core::mem::size_of::<ev_io>() as size_t) as *mut ev_io;
    (*(*client).write_callback).data = client as *mut ::core::ffi::c_void;
    let ref mut fresh1 =
        (*((*client).write_callback as *mut ::core::ffi::c_void as *mut ev_watcher)).pending;
    *fresh1 = 0 as ::core::ffi::c_int;
    (*((*client).write_callback as *mut ::core::ffi::c_void as *mut ev_watcher)).active = *fresh1;
    (*((*client).write_callback as *mut ::core::ffi::c_void as *mut ev_watcher)).priority =
        0 as ::core::ffi::c_int;
    (*(*client).write_callback).cb = Some(
        ipc_socket_writeable_cb
            as unsafe extern "C" fn(*mut ev_loop, *mut ev_io, ::core::ffi::c_int) -> (),
    )
        as Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_io, ::core::ffi::c_int) -> ()>;
    memmove(
        &raw mut (*((*client).write_callback as *mut ev_watcher)).cb as *mut ::core::ffi::c_void,
        &raw mut (*(*client).write_callback).cb as *const ::core::ffi::c_void,
        ::core::mem::size_of::<
            Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_io, ::core::ffi::c_int) -> ()>,
        >() as size_t,
    );
    (*(*client).write_callback).fd = fd;
    (*(*client).write_callback).events =
        EV_WRITE as ::core::ffi::c_int | EV__IOFDSET as ::core::ffi::c_int;
    debuglog(
        c"%s:%s:%d - IPC: new client connected on fd %d\n"
            .as_ptr()
            .cast_mut(),
        c"../src/ipc.c".as_ptr(),
        c"ipc_new_client_on_fd".as_ptr(),
        1580 as ::core::ffi::c_int,
        fd,
    );
    (*client).clients.tqe_next = ::core::ptr::null_mut::<ipc_client>();
    (*client).clients.tqe_prev = all_clients.tqh_last;
    *all_clients.tqh_last = client as *mut ipc_client;
    all_clients.tqh_last = &raw mut (*client).clients.tqe_next;
    client
}

pub unsafe fn ipc_marshal_workspace_event(
    change: *const ::core::ffi::c_char,
    current: *mut Con,
    old: *mut Con,
) -> yajl_gen {
    let prev_locale: *mut ::core::ffi::c_char = sstrdup(setlocale(
        LC_NUMERIC,
        ::core::ptr::null::<::core::ffi::c_char>(),
    ));
    setlocale(LC_NUMERIC, c"C".as_ptr());
    let gen: yajl_gen = yajl_gen_alloc(::core::ptr::null::<yajl_alloc_funcs>());
    yajl_gen_map_open(gen);
    yajl_gen_string(
        gen,
        c"change".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"change".as_ptr()),
    );
    yajl_gen_string(gen, change as *mut ::core::ffi::c_uchar, strlen(change));
    yajl_gen_string(
        gen,
        c"current".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"current".as_ptr()),
    );
    if current.is_null() {
        yajl_gen_null(gen);
    } else {
        dump_node(gen, current, false_0 != 0);
    }
    yajl_gen_string(
        gen,
        c"old".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"old".as_ptr()),
    );
    if old.is_null() {
        yajl_gen_null(gen);
    } else {
        dump_node(gen, old, false_0 != 0);
    }
    yajl_gen_map_close(gen);
    setlocale(LC_NUMERIC, prev_locale);
    free(prev_locale as *mut ::core::ffi::c_void);
    gen
}

pub unsafe fn ipc_send_workspace_event(
    change: *const ::core::ffi::c_char,
    current: *mut Con,
    old: *mut Con,
) {
    let gen: yajl_gen = ipc_marshal_workspace_event(change, current, old);
    let mut payload: *const ::core::ffi::c_uchar = ::core::ptr::null::<::core::ffi::c_uchar>();
    let mut length: ylength = 0;
    yajl_gen_get_buf(gen, &raw mut payload, &raw mut length);
    ipc_send_event(
        c"workspace".as_ptr(),
        I3_IPC_EVENT_WORKSPACE as uint32_t,
        payload as *const ::core::ffi::c_char,
    );
    yajl_gen_free(gen);
}

pub unsafe fn ipc_send_window_event(property: *const ::core::ffi::c_char, con: *mut Con) {
    debuglog(
        c"%s:%s:%d - Issue IPC window %s event (con = %p, window = 0x%08x)\n"
            .as_ptr()
            .cast_mut(),
        c"../src/ipc.c".as_ptr(),
        c"ipc_send_window_event".as_ptr(),
        1644 as ::core::ffi::c_int,
        property,
        con,
        if !(*con).window.is_null() {
            (*(*con).window).id
        } else {
            XCB_WINDOW_NONE as ::core::ffi::c_int as xcb_window_t
        },
    );
    let prev_locale: *mut ::core::ffi::c_char = sstrdup(setlocale(
        LC_NUMERIC,
        ::core::ptr::null::<::core::ffi::c_char>(),
    ));
    setlocale(LC_NUMERIC, c"C".as_ptr());
    let gen: yajl_gen = yajl_gen_alloc(::core::ptr::null::<yajl_alloc_funcs>());
    yajl_gen_map_open(gen);
    yajl_gen_string(
        gen,
        c"change".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"change".as_ptr()),
    );
    yajl_gen_string(gen, property as *mut ::core::ffi::c_uchar, strlen(property));
    yajl_gen_string(
        gen,
        c"container".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"container".as_ptr()),
    );
    dump_node(gen, con, false_0 != 0);
    yajl_gen_map_close(gen);
    let mut payload: *const ::core::ffi::c_uchar = ::core::ptr::null::<::core::ffi::c_uchar>();
    let mut length: ylength = 0;
    yajl_gen_get_buf(gen, &raw mut payload, &raw mut length);
    ipc_send_event(
        c"window".as_ptr(),
        I3_IPC_EVENT_WINDOW as uint32_t,
        payload as *const ::core::ffi::c_char,
    );
    yajl_gen_free(gen);
    setlocale(LC_NUMERIC, prev_locale);
    free(prev_locale as *mut ::core::ffi::c_void);
}

pub unsafe fn ipc_send_barconfig_update_event(barconfig: *mut Barconfig) {
    debuglog(
        c"%s:%s:%d - Issue barconfig_update event for id = %s\n"
            .as_ptr()
            .cast_mut(),
        c"../src/ipc.c".as_ptr(),
        c"ipc_send_barconfig_update_event".as_ptr(),
        1674 as ::core::ffi::c_int,
        (*barconfig).id,
    );
    let prev_locale: *mut ::core::ffi::c_char = sstrdup(setlocale(
        LC_NUMERIC,
        ::core::ptr::null::<::core::ffi::c_char>(),
    ));
    setlocale(LC_NUMERIC, c"C".as_ptr());
    let gen: yajl_gen = yajl_gen_alloc(::core::ptr::null::<yajl_alloc_funcs>());
    dump_bar_config(gen, barconfig);
    let mut payload: *const ::core::ffi::c_uchar = ::core::ptr::null::<::core::ffi::c_uchar>();
    let mut length: ylength = 0;
    yajl_gen_get_buf(gen, &raw mut payload, &raw mut length);
    ipc_send_event(
        c"barconfig_update".as_ptr(),
        I3_IPC_EVENT_BARCONFIG_UPDATE as uint32_t,
        payload as *const ::core::ffi::c_char,
    );
    yajl_gen_free(gen);
    setlocale(LC_NUMERIC, prev_locale);
    free(prev_locale as *mut ::core::ffi::c_void);
}

pub unsafe fn ipc_send_binding_event(
    event_type: *const ::core::ffi::c_char,
    bind: *mut Binding,
    modename: *const ::core::ffi::c_char,
) {
    debuglog(
        c"%s:%s:%d - Issue IPC binding %s event (sym = %s, code = %d)\n"
            .as_ptr()
            .cast_mut(),
        c"../src/ipc.c".as_ptr(),
        c"ipc_send_binding_event".as_ptr(),
        1695 as ::core::ffi::c_int,
        event_type,
        (*bind).symbol,
        (*bind).keycode,
    );
    let prev_locale: *mut ::core::ffi::c_char = sstrdup(setlocale(
        LC_NUMERIC,
        ::core::ptr::null::<::core::ffi::c_char>(),
    ));
    setlocale(LC_NUMERIC, c"C".as_ptr());
    let gen: yajl_gen = yajl_gen_alloc(::core::ptr::null::<yajl_alloc_funcs>());
    yajl_gen_map_open(gen);
    yajl_gen_string(
        gen,
        c"change".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"change".as_ptr()),
    );
    yajl_gen_string(
        gen,
        event_type as *mut ::core::ffi::c_uchar,
        strlen(event_type),
    );
    yajl_gen_string(
        gen,
        c"mode".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"mode".as_ptr()),
    );
    if modename.is_null() {
        yajl_gen_string(
            gen,
            c"default".as_ptr() as *mut ::core::ffi::c_uchar,
            strlen(c"default".as_ptr()),
        );
    } else {
        yajl_gen_string(gen, modename as *mut ::core::ffi::c_uchar, strlen(modename));
    }
    yajl_gen_string(
        gen,
        c"binding".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"binding".as_ptr()),
    );
    dump_binding(gen, bind);
    yajl_gen_map_close(gen);
    let mut payload: *const ::core::ffi::c_uchar = ::core::ptr::null::<::core::ffi::c_uchar>();
    let mut length: ylength = 0;
    yajl_gen_get_buf(gen, &raw mut payload, &raw mut length);
    ipc_send_event(
        c"binding".as_ptr(),
        I3_IPC_EVENT_BINDING as uint32_t,
        payload as *const ::core::ffi::c_char,
    );
    yajl_gen_free(gen);
    setlocale(LC_NUMERIC, prev_locale);
    free(prev_locale as *mut ::core::ffi::c_void);
}

pub unsafe fn ipc_confirm_restart(client: *mut ipc_client) {
    debuglog(
        c"%s:%s:%d - ipc_confirm_restart(fd %d)\n"
            .as_ptr()
            .cast_mut(),
        c"../src/ipc.c".as_ptr(),
        c"ipc_confirm_restart".as_ptr(),
        1734 as ::core::ffi::c_int,
        (*client).fd,
    );
    static mut reply: *const ::core::ffi::c_char = c"[{\"success\":true}]".as_ptr();
    ipc_send_client_message(
        client,
        strlen(reply),
        I3_IPC_REPLY_TYPE_COMMAND as uint32_t,
        reply as *const uint8_t,
    );
    ipc_push_pending(client);
}
unsafe extern "C" fn run_static_initializers() {
    all_clients = ipc_client_head {
        tqh_first: ::core::ptr::null_mut::<ipc_client>(),
        tqh_last: &raw mut all_clients.tqh_first,
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];

// Internal helpers use Rust linkage rather than foreign symbol lookup.
use crate::sync::sync_respond;
use crate::tree::tree_render;
use i3_common::safewrappers::scalloc;
use i3_common::safewrappers::srealloc;
use i3_common::safewrappers::sstrdup;
use i3_common::safewrappers::sstrndup;
use i3_common::safewrappers::writeall_nonblock;

// Canonical Cairo backend ABI; renderer logic remains local.

use xcb_sys::{xcb_window_t, XCB_NONE, XCB_WINDOW_NONE};

pub use libc::{
    __errno_location, accept, close, fcntl, free, getsockopt, memcpy, memmove, open, perror, read,
    setlocale, shutdown, sockaddr, sockaddr_un, strcasecmp, strcmp, strlen,
};

use i3_common::native_stdio::__assert_fail;

use crate::log::{debuglog, errorlog, verboselog};

pub use libc::ucred;

// Shared model types retain field layout while using one Rust identity.
pub use crate::types::gaps_t;
pub use crate::types::mark_t;
pub use crate::types::xoutput;
pub use crate::types::Con;
pub use crate::types::Match;
pub use crate::types::Rect;
pub use i3_common::string::I3String as _i3String;

// Internal calls and globals use canonical Rust identities.
use crate::application::current_binding_mode;
use crate::application::main_loop;
use crate::application::A__NET_WM_WINDOW_TYPE_DIALOG;
use crate::application::A__NET_WM_WINDOW_TYPE_DOCK;
use crate::application::A__NET_WM_WINDOW_TYPE_DROPDOWN_MENU;
use crate::application::A__NET_WM_WINDOW_TYPE_MENU;
use crate::application::A__NET_WM_WINDOW_TYPE_NORMAL;
use crate::application::A__NET_WM_WINDOW_TYPE_NOTIFICATION;
use crate::application::A__NET_WM_WINDOW_TYPE_POPUP_MENU;
use crate::application::A__NET_WM_WINDOW_TYPE_SPLASH;
use crate::application::A__NET_WM_WINDOW_TYPE_TOOLBAR;
use crate::application::A__NET_WM_WINDOW_TYPE_TOOLTIP;
use crate::application::A__NET_WM_WINDOW_TYPE_UTILITY;
use crate::con::con_draw_decoration_into_frame;
use crate::con::con_get_fullscreen_con;
use crate::con::con_get_output;
use crate::con::con_get_workspace;
use crate::con::con_is_internal;
use crate::con::con_is_split;
use crate::con::con_orientation;
use crate::config::current_configpath;
use crate::output::output_get_content;
use crate::output::output_primary_name;
use crate::randr::get_output_by_name;
use crate::tree::croot;
use crate::tree::focused;
use crate::version::i3_version;
use crate::workspace::previous_workspace_name;
use crate::workspace::workspace_is_visible;
use i3_common::ipc_recv_message::ipc_recv_message;
use i3_common::nonblock::set_nonblock;
use i3_common::string::i3string_as_utf8;
