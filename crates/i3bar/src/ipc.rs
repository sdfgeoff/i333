use crate::child::clear_statusline;
use crate::config::config;
use crate::outputs::outputs;
use crate::xcb::kick_tray_clients;
use i3_common::safewrappers::sasprintf;
/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3bar - an xcb-based status- and ws-bar for i3
 * © 2010 Axel Wagner and contributors (see also: LICENSE)
 *
 * ipc.c: Communicating with i3
 *
 */
// Mechanical file-by-file Rust translation of i3bar/src/ipc.c.
// Originally translated with C2Rust 0.22.1; subsequently maintained in Rust.
pub use libev_sys::{ev_io, ev_io_start, ev_io_stop, ev_loop, ev_watcher, EV_READ, EV__IOFDSET};
pub type size_t = usize;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type ssize_t = isize;

pub type uint32_t = __uint32_t;
pub type uintptr_t = usize;

pub type i3String = _i3String;
pub use crate::child::rect_t;
pub type rect = rect_t;
pub type blockalign_t = ::core::ffi::c_uint;
pub const ALIGN_RIGHT: blockalign_t = 2;
pub const ALIGN_CENTER: blockalign_t = 1;
pub const ALIGN_LEFT: blockalign_t = 0;
pub use crate::child::statusline_head;

pub type handler_t = Option<unsafe extern "C" fn(*const ::core::ffi::c_uchar, size_t) -> ()>;
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const S_SHOW: C2RustUnnamed_1 = 1;
pub const S_HIDE: C2RustUnnamed_1 = 0;
pub type bar_display_mode_t = ::core::ffi::c_uint;
pub const M_INVISIBLE: bar_display_mode_t = 2;
pub const M_HIDE: bar_display_mode_t = 1;
pub const M_DOCK: bar_display_mode_t = 0;

pub type position_t = ::core::ffi::c_uint;
pub const POS_BOT: position_t = 2;
pub const POS_TOP: position_t = 1;
pub const POS_NONE: position_t = 0;
pub use crate::child::i3_output;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const I3_IPC_MAGIC: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"i3-ipc\0") };
pub const I3_IPC_MESSAGE_TYPE_GET_WORKSPACES: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const I3_IPC_MESSAGE_TYPE_SUBSCRIBE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const I3_IPC_MESSAGE_TYPE_GET_OUTPUTS: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const I3_IPC_MESSAGE_TYPE_GET_BAR_CONFIG: ::core::ffi::c_int = 6 as ::core::ffi::c_int;

pub static mut i3_connection: *mut ev_io = ::core::ptr::null::<ev_io>() as *mut ev_io;

pub static mut sock_path: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
unsafe fn i3_provides_workspaces() -> bool {
    !config.disable_ws && config.workspace_command.is_null()
}
unsafe extern "C" fn got_command_reply(reply: *const ::core::ffi::c_uchar, size: size_t) {}
unsafe extern "C" fn got_workspace_reply(reply: *const ::core::ffi::c_uchar, size: size_t) {
    if config.verbose {
        printf(
            c"[%s:%d] Got workspace data!\n".as_ptr(),
            c"../i3bar/src/ipc.c".as_ptr(),
            53 as ::core::ffi::c_int,
        );
    }
    parse_workspaces_json(reply, size);
    draw_bars(false_0 != 0);
}
unsafe extern "C" fn got_subscribe_reply(reply: *const ::core::ffi::c_uchar, size: size_t) {
    if config.verbose {
        printf(
            c"[%s:%d] Got subscribe reply: %s\n".as_ptr(),
            c"../i3bar/src/ipc.c".as_ptr(),
            64 as ::core::ffi::c_int,
            reply,
        );
    }
}
unsafe extern "C" fn got_output_reply(reply: *const ::core::ffi::c_uchar, size: size_t) {
    if config.verbose {
        printf(
            c"[%s:%d] Clearing old output configuration...\n".as_ptr(),
            c"../i3bar/src/ipc.c".as_ptr(),
            73 as ::core::ffi::c_int,
        );
    }
    free_outputs();
    if config.verbose {
        printf(
            c"[%s:%d] Parsing outputs JSON...\n".as_ptr(),
            c"../i3bar/src/ipc.c".as_ptr(),
            76 as ::core::ffi::c_int,
        );
    }
    parse_outputs_json(reply, size);
    if config.verbose {
        printf(
            c"[%s:%d] Reconfiguring windows...\n".as_ptr(),
            c"../i3bar/src/ipc.c".as_ptr(),
            78 as ::core::ffi::c_int,
        );
    }
    reconfig_windows(false_0 != 0);
    let mut o_walk: *mut i3_output = ::core::ptr::null_mut::<i3_output>();
    o_walk = (*outputs).slh_first as *mut i3_output;
    while !o_walk.is_null() {
        kick_tray_clients(o_walk);
        o_walk = (*o_walk).slist.sle_next as *mut i3_output;
    }
    if i3_provides_workspaces() {
        i3_send_msg(
            I3_IPC_MESSAGE_TYPE_GET_WORKSPACES as uint32_t,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
    } else if !config.workspace_command.is_null() {
        repeat_last_ws_json();
    }
    draw_bars(false_0 != 0);
}
unsafe extern "C" fn got_bar_config(reply: *const ::core::ffi::c_uchar, size: size_t) {
    if config.bar_id.is_null() {
        if config.verbose {
            printf(
                c"[%s:%d] Received bar list \"%s\"\n".as_ptr(),
                c"../i3bar/src/ipc.c".as_ptr(),
                110 as ::core::ffi::c_int,
                reply,
            );
        }
        parse_get_first_i3bar_config(reply, size);
        if config.bar_id.is_null() {
            fprintf(
                stderr,
                c"[%s:%d] ERROR: No bar configuration found, please configure a bar block in your i3 config file.\n".as_ptr(),
                c"../i3bar/src/ipc.c".as_ptr(),
                114 as ::core::ffi::c_int,
            );
            exit(EXIT_FAILURE);
        }
        verboselog(
            c"[libi3] ../i3bar/src/ipc.c Using first bar config: %s. Use --bar_id to manually select a different bar configuration.\n".as_ptr().cast_mut(),
            config.bar_id,
        );
        i3_send_msg(
            I3_IPC_MESSAGE_TYPE_GET_BAR_CONFIG as uint32_t,
            config.bar_id,
        );
        return;
    }
    if config.verbose {
        printf(
            c"[%s:%d] Received bar config \"%s\"\n".as_ptr(),
            c"../i3bar/src/ipc.c".as_ptr(),
            123 as ::core::ffi::c_int,
            reply,
        );
    }
    i3_send_msg(
        I3_IPC_MESSAGE_TYPE_GET_OUTPUTS as uint32_t,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    free_colors(&raw mut config.colors);
    parse_config_json(reply, size);
    subscribe_events();
    if i3_provides_workspaces() {
        i3_send_msg(
            I3_IPC_MESSAGE_TYPE_GET_WORKSPACES as uint32_t,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
    }
    init_xcb_late(config.fontname);
    init_colors(&raw mut config.colors);
    start_child(config.command);
    start_ws_child(config.workspace_command);
}

pub static mut reply_handlers: [handler_t; 12] = [
    Some(got_command_reply as unsafe extern "C" fn(*const ::core::ffi::c_uchar, size_t) -> ()),
    Some(got_workspace_reply as unsafe extern "C" fn(*const ::core::ffi::c_uchar, size_t) -> ()),
    Some(got_subscribe_reply as unsafe extern "C" fn(*const ::core::ffi::c_uchar, size_t) -> ()),
    Some(got_output_reply as unsafe extern "C" fn(*const ::core::ffi::c_uchar, size_t) -> ()),
    None,
    None,
    Some(got_bar_config as unsafe extern "C" fn(*const ::core::ffi::c_uchar, size_t) -> ()),
    None,
    None,
    None,
    None,
    None,
];
unsafe extern "C" fn got_workspace_event(event: *const ::core::ffi::c_uchar, size: size_t) {
    if config.verbose {
        printf(
            c"[%s:%d] Got workspace event!\n".as_ptr(),
            c"../i3bar/src/ipc.c".as_ptr(),
            170 as ::core::ffi::c_int,
        );
    }
    i3_send_msg(
        I3_IPC_MESSAGE_TYPE_GET_WORKSPACES as uint32_t,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
}
unsafe extern "C" fn got_output_event(event: *const ::core::ffi::c_uchar, size: size_t) {
    if config.verbose {
        printf(
            c"[%s:%d] Got output event!\n".as_ptr(),
            c"../i3bar/src/ipc.c".as_ptr(),
            179 as ::core::ffi::c_int,
        );
    }
    i3_send_msg(
        I3_IPC_MESSAGE_TYPE_GET_OUTPUTS as uint32_t,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
}
unsafe extern "C" fn got_mode_event(event: *const ::core::ffi::c_uchar, size: size_t) {
    if config.verbose {
        printf(
            c"[%s:%d] Got mode event!\n".as_ptr(),
            c"../i3bar/src/ipc.c".as_ptr(),
            188 as ::core::ffi::c_int,
        );
    }
    parse_mode_json(event, size);
    draw_bars(false_0 != 0);
}
unsafe fn strings_differ(a: *mut ::core::ffi::c_char, b: *mut ::core::ffi::c_char) -> bool {
    let a_null: bool = a.is_null();
    let b_null: bool = b.is_null();
    if a_null as ::core::ffi::c_int != b_null as ::core::ffi::c_int {
        return true_0 != 0;
    }
    if a_null as ::core::ffi::c_int != 0 && b_null as ::core::ffi::c_int != 0 {
        return false_0 != 0;
    }
    strcmp(a, b) != 0 as ::core::ffi::c_int
}
unsafe extern "C" fn got_bar_config_update(event: *const ::core::ffi::c_uchar, size: size_t) {
    let mut expected_id: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    sasprintf(
        &raw mut expected_id,
        c"\"id\":\"%s\"".as_ptr(),
        config.bar_id,
    );
    let found_id: *const ::core::ffi::c_char =
        strstr(event as *const ::core::ffi::c_char, expected_id);
    free(expected_id as *mut ::core::ffi::c_void);
    expected_id = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if found_id.is_null() {
        return;
    }
    i3_send_msg(
        I3_IPC_MESSAGE_TYPE_GET_OUTPUTS as uint32_t,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    free_colors(&raw mut config.colors);
    if config.verbose {
        printf(
            c"[%s:%d] Received bar config update \"%s\"\n".as_ptr(),
            c"../i3bar/src/ipc.c".as_ptr(),
            225 as ::core::ffi::c_int,
            event,
        );
    }
    let old_command: *mut ::core::ffi::c_char = config.command;
    let old_workspace_command: *mut ::core::ffi::c_char = config.workspace_command;
    config.command = ::core::ptr::null_mut::<::core::ffi::c_char>();
    config.workspace_command = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let old_mode: bar_display_mode_t = config.hide_on_modifier;
    parse_config_json(event, size);
    if old_mode as ::core::ffi::c_uint != config.hide_on_modifier as ::core::ffi::c_uint {
        reconfig_windows(true_0 != 0);
    }
    init_xcb_late(config.fontname);
    init_colors(&raw mut config.colors);
    if !status_child_is_alive()
        || strings_differ(old_command, config.command) as ::core::ffi::c_int != 0
    {
        kill_child();
        clear_statusline(&raw mut statusline_head, true_0 != 0);
        start_child(config.command);
    }
    free(old_command as *mut ::core::ffi::c_void);
    if !ws_child_is_alive()
        || strings_differ(old_workspace_command, config.workspace_command) as ::core::ffi::c_int
            != 0
    {
        free_workspaces();
        kill_ws_child();
        start_ws_child(config.workspace_command);
    }
    free(old_workspace_command as *mut ::core::ffi::c_void);
    draw_bars(false_0 != 0);
}

pub static mut event_handlers: [handler_t; 5] = [
    Some(got_workspace_event as unsafe extern "C" fn(*const ::core::ffi::c_uchar, size_t) -> ()),
    Some(got_output_event as unsafe extern "C" fn(*const ::core::ffi::c_uchar, size_t) -> ()),
    Some(got_mode_event as unsafe extern "C" fn(*const ::core::ffi::c_uchar, size_t) -> ()),
    None,
    Some(got_bar_config_update as unsafe extern "C" fn(*const ::core::ffi::c_uchar, size_t) -> ()),
];
unsafe extern "C" fn got_data(
    loop_0: *mut ev_loop,
    watcher: *mut ev_io,
    events: ::core::ffi::c_int,
) {
    if config.verbose {
        printf(
            c"[%s:%d] Got data!\n".as_ptr(),
            c"../i3bar/src/ipc.c".as_ptr(),
            275 as ::core::ffi::c_int,
        );
    }
    let fd: ::core::ffi::c_int = (*watcher).fd;
    let header_len: uint32_t = strlen(I3_IPC_MAGIC.as_ptr())
        .wrapping_add((::core::mem::size_of::<uint32_t>() as size_t).wrapping_mul(2 as size_t))
        as uint32_t;
    let mut header: *mut ::core::ffi::c_char =
        smalloc(header_len as size_t) as *mut ::core::ffi::c_char;
    let mut rec: uint32_t = 0 as uint32_t;
    while rec < header_len {
        let n: ::core::ffi::c_int = read(
            fd,
            header.offset(rec as isize) as *mut ::core::ffi::c_void,
            header_len.wrapping_sub(rec) as size_t,
        ) as ::core::ffi::c_int;
        if n == -(1 as ::core::ffi::c_int) {
            fprintf(
                stderr,
                c"[%s:%d] ERROR: read() failed: %s\n".as_ptr(),
                c"../i3bar/src/ipc.c".as_ptr(),
                288 as ::core::ffi::c_int,
                strerror(*__errno_location()),
            );
            exit(EXIT_FAILURE);
        }
        if n == 0 as ::core::ffi::c_int {
            if config.verbose {
                printf(
                    c"[%s:%d] EOF received, exiting...\n".as_ptr(),
                    c"../i3bar/src/ipc.c".as_ptr(),
                    294 as ::core::ffi::c_int,
                );
            }
            clean_xcb();
            free(header as *mut ::core::ffi::c_void);
            exit(EXIT_SUCCESS);
        }
        rec = rec.wrapping_add(n as uint32_t);
    }
    if strncmp(header, I3_IPC_MAGIC.as_ptr(), strlen(I3_IPC_MAGIC.as_ptr())) != 0 {
        fprintf(
            stderr,
            c"[%s:%d] ERROR: Wrong magic code: %.*s\n Expected: %s\n".as_ptr(),
            c"../i3bar/src/ipc.c".as_ptr(),
            309 as ::core::ffi::c_int,
            strlen(I3_IPC_MAGIC.as_ptr()) as ::core::ffi::c_int,
            header,
            I3_IPC_MAGIC.as_ptr(),
        );
        exit(EXIT_FAILURE);
    }
    let mut walk: *mut ::core::ffi::c_char = header.offset(strlen(I3_IPC_MAGIC.as_ptr()) as isize);
    let mut size: uint32_t = 0;
    memcpy(
        &raw mut size as *mut ::core::ffi::c_void,
        walk as *mut uint32_t as *const ::core::ffi::c_void,
        ::core::mem::size_of::<uint32_t>() as size_t,
    );
    walk = walk.offset(::core::mem::size_of::<uint32_t>() as usize as isize);
    let mut type_0: uint32_t = 0;
    memcpy(
        &raw mut type_0 as *mut ::core::ffi::c_void,
        walk as *mut uint32_t as *const ::core::ffi::c_void,
        ::core::mem::size_of::<uint32_t>() as size_t,
    );
    let mut buffer: *mut ::core::ffi::c_uchar =
        smalloc(size.wrapping_add(1 as uint32_t) as size_t) as *mut ::core::ffi::c_uchar;
    rec = 0 as uint32_t;
    while rec < size {
        let n_0: ::core::ffi::c_int = read(
            fd,
            buffer.offset(rec as isize) as *mut ::core::ffi::c_void,
            size.wrapping_sub(rec) as size_t,
        ) as ::core::ffi::c_int;
        if n_0 == -(1 as ::core::ffi::c_int) {
            fprintf(
                stderr,
                c"[%s:%d] ERROR: read() failed: %s\n".as_ptr(),
                c"../i3bar/src/ipc.c".as_ptr(),
                328 as ::core::ffi::c_int,
                strerror(*__errno_location()),
            );
            exit(EXIT_FAILURE);
        }
        if n_0 == 0 as ::core::ffi::c_int {
            fprintf(
                stderr,
                c"[%s:%d] ERROR: Nothing to read!\n".as_ptr(),
                c"../i3bar/src/ipc.c".as_ptr(),
                332 as ::core::ffi::c_int,
            );
            exit(EXIT_FAILURE);
        }
        rec = rec.wrapping_add(n_0 as uint32_t);
    }
    *buffer.offset(size as isize) = '\0' as i32 as ::core::ffi::c_uchar;
    if type_0 as ::core::ffi::c_ulong & (1 as ::core::ffi::c_ulong) << 31 as ::core::ffi::c_int != 0
    {
        type_0 = (type_0 as ::core::ffi::c_ulong
            ^ (1 as ::core::ffi::c_ulong) << 31 as ::core::ffi::c_int) as uint32_t;
        event_handlers[type_0 as usize].expect("non-null function pointer")(buffer, size as size_t);
    } else if reply_handlers[type_0 as usize].is_some() {
        reply_handlers[type_0 as usize].expect("non-null function pointer")(buffer, size as size_t);
    }
    free(header as *mut ::core::ffi::c_void);
    header = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free(buffer as *mut ::core::ffi::c_void);
    buffer = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
}

pub unsafe fn i3_send_msg(
    mut type_0: uint32_t,
    payload: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut len: uint32_t = 0 as uint32_t;
    if !payload.is_null() {
        len = strlen(payload) as uint32_t;
    }
    let to_write: uint32_t = strlen(I3_IPC_MAGIC.as_ptr())
        .wrapping_add((::core::mem::size_of::<uint32_t>() as size_t).wrapping_mul(2 as size_t))
        .wrapping_add(len as size_t) as uint32_t;
    let mut buffer: *mut ::core::ffi::c_char =
        smalloc(to_write as size_t) as *mut ::core::ffi::c_char;
    let mut walk: *mut ::core::ffi::c_char = buffer;
    memcpy(
        buffer as *mut ::core::ffi::c_void,
        I3_IPC_MAGIC.as_ptr() as *const ::core::ffi::c_void,
        strlen(I3_IPC_MAGIC.as_ptr()),
    );
    walk = walk.offset(strlen(I3_IPC_MAGIC.as_ptr()) as isize);
    memcpy(
        walk as *mut ::core::ffi::c_void,
        &raw mut len as *const ::core::ffi::c_void,
        ::core::mem::size_of::<uint32_t>() as size_t,
    );
    walk = walk.offset(::core::mem::size_of::<uint32_t>() as usize as isize);
    memcpy(
        walk as *mut ::core::ffi::c_void,
        &raw mut type_0 as *const ::core::ffi::c_void,
        ::core::mem::size_of::<uint32_t>() as size_t,
    );
    walk = walk.offset(::core::mem::size_of::<uint32_t>() as usize as isize);
    if !payload.is_null() {
        memcpy(
            walk as *mut ::core::ffi::c_void,
            payload as *const ::core::ffi::c_void,
            len as size_t,
        );
    }
    swrite(
        (*i3_connection).fd,
        buffer as *const ::core::ffi::c_void,
        to_write as size_t,
    );
    free(buffer as *mut ::core::ffi::c_void);
    buffer = ::core::ptr::null_mut::<::core::ffi::c_char>();
    1 as ::core::ffi::c_int
}

pub unsafe fn init_connection(socket_path: *const ::core::ffi::c_char) {
    sock_path = socket_path;
    let sockfd: ::core::ffi::c_int = ipc_connect(socket_path);
    i3_connection = smalloc(::core::mem::size_of::<ev_io>() as size_t) as *mut ev_io;
    let ref mut fresh0 = (*(i3_connection as *mut ::core::ffi::c_void as *mut ev_watcher)).pending;
    *fresh0 = 0 as ::core::ffi::c_int;
    (*(i3_connection as *mut ::core::ffi::c_void as *mut ev_watcher)).active = *fresh0;
    (*(i3_connection as *mut ::core::ffi::c_void as *mut ev_watcher)).priority =
        0 as ::core::ffi::c_int;
    (*i3_connection).cb =
        Some(got_data as unsafe extern "C" fn(*mut ev_loop, *mut ev_io, ::core::ffi::c_int) -> ())
            as Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_io, ::core::ffi::c_int) -> ()>;
    memmove(
        &raw mut (*(i3_connection as *mut ev_watcher)).cb as *mut ::core::ffi::c_void,
        &raw mut (*i3_connection).cb as *const ::core::ffi::c_void,
        ::core::mem::size_of::<
            Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_io, ::core::ffi::c_int) -> ()>,
        >() as size_t,
    );
    (*i3_connection).fd = sockfd;
    (*i3_connection).events = EV_READ as ::core::ffi::c_int | EV__IOFDSET as ::core::ffi::c_int;
    ev_io_start(main_loop as *mut ev_loop, i3_connection);
}

pub unsafe fn destroy_connection() {
    close((*i3_connection).fd);
    ev_io_stop(main_loop as *mut ev_loop, i3_connection);
}

pub unsafe fn subscribe_events() {
    if i3_provides_workspaces() {
        i3_send_msg(
            I3_IPC_MESSAGE_TYPE_SUBSCRIBE as uint32_t,
            c"[ \"workspace\", \"output\", \"mode\", \"barconfig_update\" ]".as_ptr(),
        );
    } else {
        i3_send_msg(
            I3_IPC_MESSAGE_TYPE_SUBSCRIBE as uint32_t,
            c"[ \"output\", \"mode\", \"barconfig_update\" ]".as_ptr(),
        );
    };
}
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const EXIT_SUCCESS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

// Internal helpers use Rust linkage rather than foreign symbol lookup.
use crate::child::kill_child;
use crate::child::kill_ws_child;
use crate::child::repeat_last_ws_json;
use crate::child::start_child;
use crate::child::start_ws_child;
use crate::child::status_child_is_alive;
use crate::child::ws_child_is_alive;
use crate::config::parse_config_json;
use crate::config::parse_get_first_i3bar_config;
use crate::mode::parse_mode_json;
use crate::outputs::free_outputs;
use crate::outputs::parse_outputs_json;
use crate::workspaces::free_workspaces;
use crate::workspaces::parse_workspaces_json;
use crate::xcb::clean_xcb;
use crate::xcb::draw_bars;
use crate::xcb::init_xcb_late;
use crate::xcb::reconfig_windows;
use i3_common::safewrappers::smalloc;
use i3_common::safewrappers::swrite;

// Canonical Cairo backend ABI; renderer logic remains local.

pub use libc::{
    __errno_location, close, exit, fprintf, free, memcpy, memmove, printf, read, strcmp, strerror,
    strlen, strncmp, strstr,
};

use i3_common::native_stdio::stderr;

use crate::application::verboselog;

// Internal calls and globals use canonical Rust identities.
use crate::application::main_loop;
use crate::config::free_colors;
use crate::xcb::init_colors;
use i3_common::ipc_connect::ipc_connect;

pub use i3_common::string::I3String as _i3String;
