use crate::commands_parser::command_result_free;
use crate::commands_parser::parse_command;
use crate::config::barconfigs;
use crate::config::included_files;
use crate::config::modes;
use crate::randr::outputs;
use crate::tree::all_cons;
use crate::types::BarHiddenState;
use crate::types::BarPosition;
use crate::types::BindingRelease;
use crate::types::BorderStyle;
use crate::types::FloatingState;
use crate::types::InputType;
use crate::types::Layout;
use crate::types::Orientation;
use crate::types::ScratchpadState;
use crate::types::ShutdownReason;
use i3_common::safewrappers::sasprintf;
use std::cell::RefCell;
use std::ffi::{CStr, CString};
use std::io::Read;
use std::os::fd::{FromRawFd, IntoRawFd};
use std::rc::Rc;
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
pub use i3_json::compat::{yajl_alloc_funcs, yajl_gen};
use i3_json::compat::{
    yajl_gen_alloc, yajl_gen_array_close, yajl_gen_array_open, yajl_gen_bool, yajl_gen_double,
    yajl_gen_free, yajl_gen_get_buf, yajl_gen_integer, yajl_gen_map_close, yajl_gen_map_open,
    yajl_gen_null, yajl_gen_string,
};
pub use libev_sys::{
    ev_io, ev_io_start, ev_io_stop, ev_loop, ev_timer, ev_timer_start, ev_timer_stop, ev_tstamp,
    ev_watcher, ev_watcher_time, EV_READ, EV_WRITE, EV__IOFDSET,
};

pub type i3String = _i3String;

pub use crate::application::Binding;
pub type i3_event_state_mask_t = u32;

pub type fullscreen_mode_t = ::core::ffi::c_uint;
pub const CF_GLOBAL: fullscreen_mode_t = 2;
pub const CF_OUTPUT: fullscreen_mode_t = 1;
pub const CF_NONE: fullscreen_mode_t = 0;

pub const M_BELOW: ::core::ffi::c_uint = 2;
pub const M_ASSIGN_WS: ::core::ffi::c_uint = 1;
pub const M_HERE: ::core::ffi::c_uint = 0;

pub const M_DOCK_BOTTOM: ::core::ffi::c_int = 3;
pub const M_DOCK_TOP: ::core::ffi::c_int = 2;
pub const M_DOCK_ANY: ::core::ffi::c_int = 1;
pub const M_NODOCK: ::core::ffi::c_int = 0;
pub const M_DONTCHECK: ::core::ffi::c_int = -1;

pub const A_TO_OUTPUT: ::core::ffi::c_uint = 16;
pub const A_TO_WORKSPACE_NUMBER: ::core::ffi::c_uint = 8;
pub const A_NO_FOCUS: ::core::ffi::c_uint = 4;
pub const A_TO_WORKSPACE: ::core::ffi::c_uint = 2;
pub const A_COMMAND: ::core::ffi::c_uint = 1;
pub const A_ANY: ::core::ffi::c_uint = 0;

pub const CT_DOCKAREA: ::core::ffi::c_uint = 5;
pub const CT_WORKSPACE: ::core::ffi::c_uint = 4;
pub const CT_FLOATING_CON: ::core::ffi::c_uint = 3;
pub const CT_CON: ::core::ffi::c_uint = 2;
pub const CT_OUTPUT: ::core::ffi::c_uint = 1;
pub const CT_ROOT: ::core::ffi::c_uint = 0;

pub type Output = xoutput;

pub const I3_XKB_GROUP_MASK_4: ::core::ffi::c_uint = 8;
pub const I3_XKB_GROUP_MASK_3: ::core::ffi::c_uint = 4;
pub const I3_XKB_GROUP_MASK_2: ::core::ffi::c_uint = 2;
pub const I3_XKB_GROUP_MASK_1: ::core::ffi::c_uint = 1;
pub const I3_XKB_GROUP_MASK_ANY: ::core::ffi::c_uint = 0;

pub use crate::application::Barconfig;
pub use crate::config::IncludedFile;

pub use crate::application::Barbinding;

pub use crate::application::tray_output_t;
pub use crate::bindings::Mode;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct i3_ipc_header {
    pub magic: [::core::ffi::c_char; 6],
    pub size: u32,
    pub type_0: u32,
}
pub type i3_ipc_header_t = i3_ipc_header;
pub use crate::application::ipc_client;
pub type handler_t =
    Option<unsafe fn(*mut ipc_client, *mut u8, ::core::ffi::c_int, u32, u32) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ipc_client_head {
    pub tqh_first: *mut ipc_client,
    pub tqh_last: *mut *mut ipc_client,
}
pub type ylength = usize;
pub use crate::assignments::CommandResult;

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
struct NumericLocale(CString);
impl NumericLocale {
    /// Process-global native locale users must be externally serialized.
    unsafe fn c() -> Option<Self> {
        let ptr = setlocale(LC_NUMERIC, std::ptr::null());
        if ptr.is_null() {
            return None;
        }
        let old = CStr::from_ptr(ptr).to_owned();
        setlocale(LC_NUMERIC, c"C".as_ptr());
        Some(Self(old))
    }
}
impl Drop for NumericLocale {
    fn drop(&mut self) {
        unsafe {
            setlocale(LC_NUMERIC, self.0.as_ptr());
        }
    }
}
struct JsonOwner(yajl_gen);
impl JsonOwner {
    fn new() -> Self {
        Self(Box::into_raw(Box::new(i3_json::Generator::new())).cast())
    }
}
impl Drop for JsonOwner {
    fn drop(&mut self) {
        unsafe {
            yajl_gen_free(self.0);
        }
    }
}
static mut NEXT_CLIENT_GENERATION: u64 = 1;
#[derive(Clone, Copy)]
struct ClientToken {
    pointer: *mut ipc_client,
    generation: u64,
}
impl ClientToken {
    unsafe fn capture(client: *mut ipc_client) -> Self {
        Self {
            pointer: client,
            generation: (*client).generation,
        }
    }
    unsafe fn live(self) -> bool {
        let mut current = all_clients.tqh_first;
        while !current.is_null() {
            if current == self.pointer {
                return (*current).generation == self.generation;
            }
            current = (*current).clients.tqe_next;
        }
        false
    }
}
fn encode_frame(message_type: u32, payload: &[u8]) -> Vec<u8> {
    let length = u32::try_from(payload.len()).expect("IPC payload exceeds u32 wire size");
    let mut frame = Vec::with_capacity(
        14usize
            .checked_add(payload.len())
            .expect("IPC frame size overflow"),
    );
    frame.extend_from_slice(b"i3-ipc");
    frame.extend_from_slice(&length.to_ne_bytes());
    frame.extend_from_slice(&message_type.to_ne_bytes());
    frame.extend_from_slice(payload);
    frame
}
unsafe fn ipc_push_pending(client: *mut ipc_client) {
    // Move bytes out of the client slot before native IO; no client reference is
    // held while callbacks/dispatch elsewhere can mutate client ownership.
    let mut queue = std::mem::take(&mut (*client).buffer);
    let result = writeall_nonblock((*client).fd, queue.as_ptr().cast(), queue.len());
    if result < 0 {
        (*client).buffer = queue;
        return;
    }
    let count = result as usize;
    assert!(count <= queue.len());
    queue.drain(..count);
    (*client).buffer = queue;
    if (*client).buffer.is_empty() {
        if !(*client).timeout.is_null() {
            ev_timer_stop(main_loop, (*client).timeout);
            drop(Box::from_raw((*client).timeout));
            (*client).timeout = std::ptr::null_mut();
        }
        ev_io_stop(main_loop, (*client).write_callback);
        return;
    }
    ev_io_start(main_loop, (*client).write_callback);
    if (*client).timeout.is_null() {
        let mut timer = Box::new(ev_timer::new(kill_timeout, 0.0, ipc_client_timeout));
        timer.data = client.cast();
        timer.priority = -2;
        (*client).timeout = Box::into_raw(timer);
        ev_timer_start(main_loop, (*client).timeout);
    } else if count > 0 {
        ev_timer_stop(main_loop, (*client).timeout);
        (*(*client).timeout).at = kill_timeout;
        (*(*client).timeout).repeat = 0.0;
        ev_timer_start(main_loop, (*client).timeout);
    }
}
/// Live event-thread client and readable payload span. No client borrow survives
/// native IO or dispatch; the published client address stays stable until free.
unsafe fn ipc_send_client_message(
    client: *mut ipc_client,
    size: usize,
    message_type: u32,
    payload: *const u8,
) {
    let payload = if size == 0 {
        &[]
    } else {
        std::slice::from_raw_parts(payload, size)
    };
    let frame = encode_frame(message_type, payload);
    let push_now = (*client).buffer.is_empty();
    (*client).buffer.extend_from_slice(&frame);
    if push_now {
        ipc_push_pending(client);
    }
}
unsafe fn free_ipc_client(client: *mut ipc_client, exempt_fd: libc::c_int) {
    if (*client).fd != exempt_fd {
        debuglog(
            c"%s:%s:%d - Disconnecting client on fd %d\n"
                .as_ptr()
                .cast_mut(),
            c"../src/ipc.c".as_ptr(),
            c"free_ipc_client".as_ptr(),
            119 as libc::c_int,
            (*client).fd,
        );
    }
    // Stop and unlink before dropping owned buffers/fd; native list edges are
    // updated without a whole-client mutable reference spanning callbacks.
    ev_io_stop(main_loop, (*client).read_callback);
    ev_io_stop(main_loop, (*client).write_callback);
    if !(*client).timeout.is_null() {
        ev_timer_stop(main_loop, (*client).timeout);
        drop(Box::from_raw((*client).timeout));
    }
    drop(Box::from_raw((*client).read_callback));
    drop(Box::from_raw((*client).write_callback));
    if !(*client).clients.tqe_next.is_null() {
        (*(*client).clients.tqe_next).clients.tqe_prev = (*client).clients.tqe_prev;
    } else {
        all_clients.tqh_last = (*client).clients.tqe_prev;
    }
    *(*client).clients.tqe_prev = (*client).clients.tqe_next;
    let mut owner = Box::from_raw(client);
    if owner.fd == exempt_fd {
        if let Some(fd) = owner.fd_owner.take() {
            let _ = fd.into_raw_fd();
        }
    }
    drop(owner);
}

pub unsafe fn ipc_send_event(
    event: *const libc::c_char,
    message_type: u32,
    payload: *const libc::c_char,
) {
    let event = CStr::from_ptr(event);
    let payload = CStr::from_ptr(payload).to_owned();
    let mut current = all_clients.tqh_first;
    while !current.is_null() {
        let subscribed = (*current)
            .events
            .iter()
            .any(|subscription| strcasecmp(subscription.as_ptr(), event.as_ptr()) == 0);
        if subscribed {
            ipc_send_client_message(
                current,
                payload.to_bytes().len(),
                message_type,
                payload.as_ptr().cast(),
            );
        }
        current = (*current).clients.tqe_next;
    }
}
unsafe fn ipc_send_shutdown_event(reason: ShutdownReason) {
    let generator_owner = JsonOwner::new();
    let gen = generator_owner.0;
    yajl_gen_map_open(gen);
    yajl_gen_string(
        gen,
        c"change".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"change".as_ptr()),
    );
    if reason == ShutdownReason::Restart {
        yajl_gen_string(
            gen,
            c"restart".as_ptr() as *mut ::core::ffi::c_uchar,
            strlen(c"restart".as_ptr()),
        );
    } else if reason == ShutdownReason::Exit {
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
        I3_IPC_EVENT_SHUTDOWN as u32,
        payload as *const ::core::ffi::c_char,
    );
}

pub unsafe fn ipc_shutdown(reason: ShutdownReason, exempt_fd: ::core::ffi::c_int) {
    ipc_send_shutdown_event(reason);
    while !all_clients.tqh_first.is_null() {
        let current: *mut ipc_client = all_clients.tqh_first as *mut ipc_client;
        if (*current).fd != exempt_fd {
            shutdown((*current).fd, libc::SHUT_RDWR as ::core::ffi::c_int);
        }
        free_ipc_client(current, exempt_fd);
    }
}
struct CommandResultOwner(*mut CommandResult);
impl Drop for CommandResultOwner {
    fn drop(&mut self) {
        unsafe {
            command_result_free(self.0);
        }
    }
}
unsafe fn handle_run_command(
    client: *mut ipc_client,
    message: *mut u8,
    _: libc::c_int,
    message_size: u32,
    _: u32,
) {
    let bytes = std::slice::from_raw_parts(message, message_size as usize);
    let prefix = bytes.split(|byte| *byte == 0).next().unwrap();
    let command = CString::new(prefix).unwrap();
    let token = ClientToken::capture(client);
    verboselog(
        c"IPC: received: *%.4000s*\n".as_ptr().cast_mut(),
        command.as_ptr(),
    );
    let generator_owner = JsonOwner::new();
    let gen = generator_owner.0;
    let result = CommandResultOwner(parse_command(command.as_ptr(), gen, client));
    if (*result.0).needs_tree_render {
        tree_render();
    }
    drop(result);
    // Reload/restart/command callbacks can free this client, even if exec later
    // fails. Pointer membership alone is insufficient if the address was reused.
    if !token.live() {
        return;
    }
    let mut reply = std::ptr::null();
    let mut length = 0;
    yajl_gen_get_buf(gen, &mut reply, &mut length);
    ipc_send_client_message(client, length, I3_IPC_REPLY_TYPE_COMMAND as u32, reply);
}
fn write_fields(
    generator: &mut i3_json::Generator,
    name: &str,
    fields: &[(&str, i64)],
) -> Result<(), i3_json::GenerateError> {
    generator.string(name)?;
    generator.start_object()?;
    for (key, value) in fields {
        generator.string(key)?;
        generator.integer(*value)?;
    }
    generator.end_object()
}
fn rect_fields(rect: Rect) -> [(&'static str, i64); 4] {
    [
        ("x", rect.x as i32 as i64),
        ("y", rect.y as i32 as i64),
        ("width", rect.width as i64),
        ("height", rect.height as i64),
    ]
}
fn gaps_fields(gaps: gaps_t) -> [(&'static str, i64); 6] {
    [
        ("inner", gaps.inner as i64),
        ("outer", gaps.top as i64),
        ("top", gaps.top as i64),
        ("right", gaps.right as i64),
        ("bottom", gaps.bottom as i64),
        ("left", gaps.left as i64),
    ]
}
/// Generator handles must be uniquely borrowed native adapters to a live Rust
/// Generator. The owned-value encoder itself never touches graph pointers.
unsafe fn dump_rect(gen: yajl_gen, name: *const libc::c_char, rect: Rect) {
    let name = CStr::from_ptr(name)
        .to_str()
        .expect("internal rectangle field name");
    let _ = write_fields(
        &mut *gen.cast::<i3_json::Generator>(),
        name,
        &rect_fields(rect),
    );
}
unsafe fn dump_gaps(gen: yajl_gen, name: *const libc::c_char, gaps: gaps_t) {
    let name = CStr::from_ptr(name)
        .to_str()
        .expect("internal gaps field name");
    let _ = write_fields(
        &mut *gen.cast::<i3_json::Generator>(),
        name,
        &gaps_fields(gaps),
    );
}
fn modifier_names(mask: u32) -> impl Iterator<Item = &'static str> {
    [
        (0, "shift"),
        (1, "lock"),
        (2, "ctrl"),
        (3, "Mod1"),
        (4, "Mod2"),
        (5, "Mod3"),
        (6, "Mod4"),
        (7, "Mod5"),
        (8, "Button1"),
        (9, "Button2"),
        (10, "Button3"),
        (11, "Button4"),
        (12, "Button5"),
        (16, "Group1"),
        (17, "Group2"),
        (18, "Group3"),
        (19, "Group4"),
    ]
    .into_iter()
    .filter_map(move |(bit, name)| (mask & (1 << bit) != 0).then_some(name))
}
unsafe fn dump_event_state_mask(gen: yajl_gen, bind: *mut Binding) {
    let mask = (*bind).event_state_mask;
    let generator = &mut *gen.cast::<i3_json::Generator>();
    let _ = generator.start_array();
    for name in modifier_names(mask) {
        let _ = generator.string(name);
    }
    let _ = generator.end_array();
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
        (if (*bind).input_type == InputType::Keyboard {
            c"keyboard".as_ptr()
        } else {
            c"mouse".as_ptr()
        }) as *mut ::core::ffi::c_uchar,
        strlen(if (*bind).input_type == InputType::Keyboard {
            c"keyboard".as_ptr()
        } else {
            c"mouse".as_ptr()
        }),
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
    yajl_gen_integer(gen, con as usize as ::core::ffi::c_longlong);
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
    } else if con_orientation(con) == Orientation::Horizontal {
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
    match (*con).scratchpad_state {
        ScratchpadState::None => {
            yajl_gen_string(
                gen,
                c"none".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"none".as_ptr()),
            );
        }
        ScratchpadState::Fresh => {
            yajl_gen_string(
                gen,
                c"fresh".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"fresh".as_ptr()),
            );
        }
        ScratchpadState::Changed => {
            yajl_gen_string(
                gen,
                c"changed".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"changed".as_ptr()),
            );
        }
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
    match (*con).layout {
        Layout::Default => {
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
        Layout::SplitV => {
            yajl_gen_string(
                gen,
                c"splitv".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"splitv".as_ptr()),
            );
        }
        Layout::SplitH => {
            yajl_gen_string(
                gen,
                c"splith".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"splith".as_ptr()),
            );
        }
        Layout::Stacked => {
            yajl_gen_string(
                gen,
                c"stacked".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"stacked".as_ptr()),
            );
        }
        Layout::Tabbed => {
            yajl_gen_string(
                gen,
                c"tabbed".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"tabbed".as_ptr()),
            );
        }
        Layout::DockArea => {
            yajl_gen_string(
                gen,
                c"dockarea".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"dockarea".as_ptr()),
            );
        }
        Layout::Output => {
            yajl_gen_string(
                gen,
                c"output".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"output".as_ptr()),
            );
        }
    }
    yajl_gen_string(
        gen,
        c"workspace_layout".as_ptr() as *mut ::core::ffi::c_uchar,
        strlen(c"workspace_layout".as_ptr()),
    );
    match (*con).workspace_layout {
        Layout::Default => {
            yajl_gen_string(
                gen,
                c"default".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"default".as_ptr()),
            );
        }
        Layout::Stacked => {
            yajl_gen_string(
                gen,
                c"stacked".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"stacked".as_ptr()),
            );
        }
        Layout::Tabbed => {
            yajl_gen_string(
                gen,
                c"tabbed".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"tabbed".as_ptr()),
            );
        }
        Layout::DockArea | Layout::Output | Layout::SplitV | Layout::SplitH => {
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
    match (*con).layout {
        Layout::SplitV => {
            yajl_gen_string(
                gen,
                c"splitv".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"splitv".as_ptr()),
            );
        }
        Layout::Default
        | Layout::Stacked
        | Layout::Tabbed
        | Layout::DockArea
        | Layout::Output
        | Layout::SplitH => {
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
    match (*con).border_style {
        BorderStyle::Normal => {
            yajl_gen_string(
                gen,
                c"normal".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"normal".as_ptr()),
            );
        }
        BorderStyle::None => {
            yajl_gen_string(
                gen,
                c"none".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"none".as_ptr()),
            );
        }
        BorderStyle::Pixel => {
            yajl_gen_string(
                gen,
                c"pixel".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"pixel".as_ptr()),
            );
        }
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
        yajl_gen_integer(gen, node as usize as ::core::ffi::c_longlong);
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
    match (*con).floating {
        FloatingState::AutoOff => {
            yajl_gen_string(
                gen,
                c"auto_off".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"auto_off".as_ptr()),
            );
        }
        FloatingState::AutoOn => {
            yajl_gen_string(
                gen,
                c"auto_on".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"auto_on".as_ptr()),
            );
        }
        FloatingState::UserOff => {
            yajl_gen_string(
                gen,
                c"user_off".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"user_off".as_ptr()),
            );
        }
        FloatingState::UserOn => {
            yajl_gen_string(
                gen,
                c"user_on".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"user_on".as_ptr()),
            );
        }
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
                    (*(*match_0).class).pattern().as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen((*(*match_0).class).pattern().as_ptr()),
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
                    (*(*match_0).instance).pattern().as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen((*(*match_0).instance).pattern().as_ptr()),
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
                    (*(*match_0).window_role).pattern().as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen((*(*match_0).window_role).pattern().as_ptr()),
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
                    (*(*match_0).title).pattern().as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen((*(*match_0).title).pattern().as_ptr()),
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
                    (*(*match_0).machine).pattern().as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen((*(*match_0).machine).pattern().as_ptr()),
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
        yajl_gen_bool(gen, i32::from((*current).release));
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
    match (*config).hidden_state {
        BarHiddenState::Show => {
            yajl_gen_string(
                gen,
                c"show".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"show".as_ptr()),
            );
        }
        BarHiddenState::Hide => {
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
    if (*config).position == BarPosition::Bottom {
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
    message: *mut u8,
    size: ::core::ffi::c_int,
    message_size: u32,
    message_type: u32,
) {
    let locale = NumericLocale::c();
    let generator_owner = JsonOwner::new();
    let gen = generator_owner.0;
    dump_node(gen, croot, false_0 != 0);
    drop(locale);
    let mut payload: *const ::core::ffi::c_uchar = ::core::ptr::null::<::core::ffi::c_uchar>();
    let mut length: ylength = 0;
    yajl_gen_get_buf(gen, &raw mut payload, &raw mut length);
    ipc_send_client_message(
        client,
        length as usize,
        I3_IPC_REPLY_TYPE_TREE as u32,
        payload as *const u8,
    );
}
unsafe fn handle_get_workspaces(
    client: *mut ipc_client,
    message: *mut u8,
    size: ::core::ffi::c_int,
    message_size: u32,
    message_type: u32,
) {
    let generator_owner = JsonOwner::new();
    let gen = generator_owner.0;
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
                yajl_gen_integer(gen, ws as usize as ::core::ffi::c_longlong);
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
        length as usize,
        I3_IPC_REPLY_TYPE_WORKSPACES as u32,
        payload as *const u8,
    );
}
unsafe fn handle_get_outputs(
    client: *mut ipc_client,
    message: *mut u8,
    size: ::core::ffi::c_int,
    message_size: u32,
    message_type: u32,
) {
    let generator_owner = JsonOwner::new();
    let gen = generator_owner.0;
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
        length as usize,
        I3_IPC_REPLY_TYPE_OUTPUTS as u32,
        payload as *const u8,
    );
}
unsafe fn handle_get_marks(
    client: *mut ipc_client,
    message: *mut u8,
    size: ::core::ffi::c_int,
    message_size: u32,
    message_type: u32,
) {
    let generator_owner = JsonOwner::new();
    let gen = generator_owner.0;
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
        length as usize,
        I3_IPC_REPLY_TYPE_MARKS as u32,
        payload as *const u8,
    );
}
unsafe fn handle_get_version(
    client: *mut ipc_client,
    message: *mut u8,
    size: ::core::ffi::c_int,
    message_size: u32,
    message_type: u32,
) {
    let generator_owner = JsonOwner::new();
    let gen = generator_owner.0;
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
        length as usize,
        I3_IPC_REPLY_TYPE_VERSION as u32,
        payload as *const u8,
    );
}
unsafe fn handle_get_bar_config(
    client: *mut ipc_client,
    message: *mut u8,
    size: ::core::ffi::c_int,
    message_size: u32,
    message_type: u32,
) {
    let generator_owner = JsonOwner::new();
    let gen = generator_owner.0;
    if message_size == 0 as u32 {
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
            length as usize,
            I3_IPC_REPLY_TYPE_BAR_CONFIG as u32,
            payload as *const u8,
        );

        return;
    }
    let bar_bytes = std::slice::from_raw_parts(message, message_size as usize);
    let bar_id_owner = CString::new(bar_bytes.split(|byte| *byte == 0).next().unwrap()).unwrap();
    let bar_id = bar_id_owner.as_ptr();
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
        length_0 as usize,
        I3_IPC_REPLY_TYPE_BAR_CONFIG as u32,
        payload_0 as *const u8,
    );
}
unsafe fn handle_get_binding_modes(
    client: *mut ipc_client,
    message: *mut u8,
    size: ::core::ffi::c_int,
    message_size: u32,
    message_type: u32,
) {
    let generator_owner = JsonOwner::new();
    let gen = generator_owner.0;
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
        length as usize,
        I3_IPC_REPLY_TYPE_BINDING_MODES as u32,
        payload as *const u8,
    );
}
fn parse_subscriptions(message: &[u8]) -> (Vec<CString>, Result<(), i3_json::ParseError>) {
    let events = Rc::new(RefCell::new(Vec::new()));
    let output = events.clone();
    let mut parser = i3_json::Parser::new(move |event: i3_json::Event<'_>| {
        if let i3_json::Event::String(bytes) = event {
            let prefix = &bytes[..bytes
                .iter()
                .position(|byte| *byte == 0)
                .unwrap_or(bytes.len())];
            output.borrow_mut().push(CString::new(prefix).unwrap());
        }
        true
    });
    parser.options_mut().allow_legacy_whitespace = true;
    let result = parser.parse(message);
    drop(parser);
    (Rc::try_unwrap(events).unwrap().into_inner(), result)
}
unsafe fn handle_subscribe(
    client: *mut ipc_client,
    message: *mut u8,
    _: libc::c_int,
    message_size: u32,
    _: u32,
) {
    let bytes = std::slice::from_raw_parts(message, message_size as usize);
    let (mut events, result) = parse_subscriptions(bytes);
    // Legacy subscription callbacks committed before a later parse error.
    (*client).events.append(&mut events);
    if let Err(error) = result {
        let error = CString::new(format!("{error}\n")).unwrap();
        errorlog(
            c"ERROR: YAJL parse error: %s\n".as_ptr().cast_mut(),
            error.as_ptr(),
        );
        ipc_send_client_message(
            client,
            17,
            I3_IPC_REPLY_TYPE_SUBSCRIBE as u32,
            b"{\"success\":false}".as_ptr(),
        );
        return;
    }
    ipc_send_client_message(
        client,
        16,
        I3_IPC_REPLY_TYPE_SUBSCRIBE as u32,
        b"{\"success\":true}".as_ptr(),
    );
    if !(*client).first_tick_sent
        && (*client)
            .events
            .iter()
            .any(|event| event.as_c_str() == c"tick")
    {
        (*client).first_tick_sent = true;
        let payload = b"{\"first\":true,\"payload\":\"\"}";
        ipc_send_client_message(
            client,
            payload.len(),
            I3_IPC_EVENT_TICK as u32,
            payload.as_ptr(),
        );
    }
}
unsafe fn handle_get_config(
    client: *mut ipc_client,
    message: *mut u8,
    size: ::core::ffi::c_int,
    message_size: u32,
    message_type: u32,
) {
    let generator_owner = JsonOwner::new();
    let gen = generator_owner.0;
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
        length as usize,
        I3_IPC_REPLY_TYPE_CONFIG as u32,
        payload as *const u8,
    );
}
unsafe fn handle_send_tick(
    client: *mut ipc_client,
    message: *mut u8,
    size: ::core::ffi::c_int,
    message_size: u32,
    message_type: u32,
) {
    let generator_owner = JsonOwner::new();
    let gen = generator_owner.0;
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
        message_size as usize,
    );
    yajl_gen_map_close(gen);
    let mut payload: *const ::core::ffi::c_uchar = ::core::ptr::null::<::core::ffi::c_uchar>();
    let mut length: ylength = 0;
    yajl_gen_get_buf(gen, &raw mut payload, &raw mut length);
    ipc_send_event(
        c"tick".as_ptr(),
        I3_IPC_EVENT_TICK as u32,
        payload as *const ::core::ffi::c_char,
    );

    let reply: *const ::core::ffi::c_char = c"{\"success\":true}".as_ptr();
    ipc_send_client_message(
        client,
        strlen(reply),
        I3_IPC_REPLY_TYPE_TICK as u32,
        reply as *const u8,
    );
    debuglog(
        c"%s:%s:%d - Sent tick event\n".as_ptr().cast_mut(),
        c"../src/ipc.c".as_ptr(),
        c"handle_send_tick".as_ptr(),
        1342 as ::core::ffi::c_int,
    );
}
#[derive(Clone, Copy, Default, Debug, PartialEq)]
struct SyncRequest {
    rnd: u32,
    window: u32,
}
struct SyncEvents {
    key: CString,
    state: Rc<RefCell<SyncRequest>>,
    error: Option<String>,
}
impl i3_json::EventHandler for SyncEvents {
    fn on_event(&mut self, event: i3_json::Event<'_>) -> bool {
        match event {
            i3_json::Event::MapKey(bytes) => {
                self.key = CString::new(bytes.split(|byte| *byte == 0).next().unwrap()).unwrap()
            }
            i3_json::Event::Number(number) if !number.contains(['.', 'e', 'E']) => {
                if let Ok(number) = number.parse::<i64>() {
                    if number != i64::MIN {
                        if unsafe { strcasecmp(self.key.as_ptr(), c"rnd".as_ptr()) == 0 } {
                            self.state.borrow_mut().rnd = number as u32;
                        } else if unsafe { strcasecmp(self.key.as_ptr(), c"window".as_ptr()) == 0 }
                        {
                            self.state.borrow_mut().window = number as u32;
                        }
                        return true;
                    }
                }
                self.error = Some("parse error: integer overflow".into());
                return false;
            }
            _ => {}
        }
        true
    }
    fn error_message(&self) -> Option<&str> {
        self.error.as_deref()
    }
}
fn parse_sync(message: &[u8]) -> Result<SyncRequest, i3_json::ParseError> {
    let state = Rc::new(RefCell::new(SyncRequest::default()));
    let mut parser = i3_json::Parser::new(SyncEvents {
        key: CString::default(),
        state: state.clone(),
        error: None,
    });
    parser.options_mut().allow_legacy_whitespace = true;
    parser.parse(message)?;
    drop(parser);
    Ok(Rc::try_unwrap(state).unwrap().into_inner())
}
unsafe fn handle_sync(
    client: *mut ipc_client,
    message: *mut u8,
    _: libc::c_int,
    message_size: u32,
    _: u32,
) {
    let token = ClientToken::capture(client);
    let parsed = parse_sync(std::slice::from_raw_parts(message, message_size as usize));
    let success = if let Ok(state) = parsed {
        sync_respond(state.window, state.rnd);
        true
    } else {
        let error = CString::new(format!("{}\n", parsed.unwrap_err())).unwrap();
        errorlog(
            c"ERROR: YAJL parse error: %s\n".as_ptr().cast_mut(),
            error.as_ptr(),
        );
        false
    };
    if !token.live() {
        return;
    }
    let reply: &[u8] = if success {
        b"{\"success\":true}"
    } else {
        b"{\"success\":false}"
    };
    ipc_send_client_message(
        client,
        reply.len(),
        I3_IPC_REPLY_TYPE_SYNC as u32,
        reply.as_ptr(),
    );
}
unsafe fn handle_get_binding_state(
    client: *mut ipc_client,
    message: *mut u8,
    size: ::core::ffi::c_int,
    message_size: u32,
    message_type: u32,
) {
    let generator_owner = JsonOwner::new();
    let gen = generator_owner.0;
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
        length as usize,
        I3_IPC_REPLY_TYPE_GET_BINDING_STATE as u32,
        payload as *const u8,
    );
}

pub const handlers: [handler_t; 13] = unsafe {
    [
        Some(
            handle_run_command
                as unsafe fn(*mut ipc_client, *mut u8, ::core::ffi::c_int, u32, u32) -> (),
        ),
        Some(
            handle_get_workspaces
                as unsafe fn(*mut ipc_client, *mut u8, ::core::ffi::c_int, u32, u32) -> (),
        ),
        Some(
            handle_subscribe
                as unsafe fn(*mut ipc_client, *mut u8, ::core::ffi::c_int, u32, u32) -> (),
        ),
        Some(
            handle_get_outputs
                as unsafe fn(*mut ipc_client, *mut u8, ::core::ffi::c_int, u32, u32) -> (),
        ),
        Some(
            handle_tree as unsafe fn(*mut ipc_client, *mut u8, ::core::ffi::c_int, u32, u32) -> (),
        ),
        Some(
            handle_get_marks
                as unsafe fn(*mut ipc_client, *mut u8, ::core::ffi::c_int, u32, u32) -> (),
        ),
        Some(
            handle_get_bar_config
                as unsafe fn(*mut ipc_client, *mut u8, ::core::ffi::c_int, u32, u32) -> (),
        ),
        Some(
            handle_get_version
                as unsafe fn(*mut ipc_client, *mut u8, ::core::ffi::c_int, u32, u32) -> (),
        ),
        Some(
            handle_get_binding_modes
                as unsafe fn(*mut ipc_client, *mut u8, ::core::ffi::c_int, u32, u32) -> (),
        ),
        Some(
            handle_get_config
                as unsafe fn(*mut ipc_client, *mut u8, ::core::ffi::c_int, u32, u32) -> (),
        ),
        Some(
            handle_send_tick
                as unsafe fn(*mut ipc_client, *mut u8, ::core::ffi::c_int, u32, u32) -> (),
        ),
        Some(
            handle_sync as unsafe fn(*mut ipc_client, *mut u8, ::core::ffi::c_int, u32, u32) -> (),
        ),
        Some(
            handle_get_binding_state
                as unsafe fn(*mut ipc_client, *mut u8, ::core::ffi::c_int, u32, u32) -> (),
        ),
    ]
};
struct ReceivedBytes(*mut u8);
impl Drop for ReceivedBytes {
    fn drop(&mut self) {
        unsafe {
            free(self.0.cast());
        }
    }
}
unsafe extern "C" fn ipc_receive_message(_: *mut ev_loop, w: *mut ev_io, _: libc::c_int) {
    let client = (*w).data.cast::<ipc_client>();
    assert_eq!((*client).fd, (*w).fd);
    let mut message_type = 0;
    let mut message_length = 0;
    let mut message = std::ptr::null_mut();
    let result = ipc_recv_message(
        (*w).fd,
        &mut message_type,
        &mut message_length,
        &mut message,
    );
    let message = ReceivedBytes(message);
    if result < 0 {
        if result != -1 || *__errno_location() != EAGAIN {
            free_ipc_client(client, -1);
        }
        return;
    }
    if let Some(handler) = handlers.get(message_type as usize).copied().flatten() {
        handler(client, message.0, 0, message_length, message_type);
    } else {
        debuglog(
            c"%s:%s:%d - Unhandled message type: %d\n"
                .as_ptr()
                .cast_mut(),
            c"../src/ipc.c".as_ptr(),
            c"ipc_receive_message".as_ptr(),
            1468 as libc::c_int,
            message_type,
        );
    }
}
fn diagnostic_cmdline(bytes: &[u8]) -> CString {
    let mut bytes = bytes.to_vec();
    let last = bytes.len().saturating_sub(1);
    for byte in &mut bytes[..last] {
        if *byte == 0 {
            *byte = b' ';
        }
    }
    // Bounded native diagnostic: full 512-byte records need an explicit NUL.
    let length = bytes
        .iter()
        .position(|byte| *byte == 0)
        .unwrap_or(bytes.len());
    CString::new(&bytes[..length]).unwrap()
}
unsafe extern "C" fn ipc_client_timeout(_: *mut ev_loop, w: *mut ev_timer, _: libc::c_int) {
    let client = (*w).data.cast::<ipc_client>();
    let mut peercred: libc::ucred = std::mem::zeroed();
    let mut length = std::mem::size_of_val(&peercred) as libc::socklen_t;
    let mut identified = false;
    if getsockopt(
        (*client).fd,
        SOL_SOCKET,
        SO_PEERCRED,
        (&mut peercred as *mut libc::ucred).cast(),
        &mut length,
    ) == 0
    {
        if let Ok(mut file) = std::fs::File::open(format!("/proc/{}/cmdline", peercred.pid)) {
            let mut bytes = [0u8; 512];
            if let Ok(count) = file.read(&mut bytes) {
                let line = diagnostic_cmdline(&bytes[..count]);
                identified = true;
                errorlog(
                    c"ERROR: client %p with pid %d and cmdline '%s' on fd %d timed out, killing\n"
                        .as_ptr()
                        .cast_mut(),
                    client,
                    peercred.pid,
                    line.as_ptr(),
                    (*client).fd,
                );
            }
        }
    }
    if !identified {
        errorlog(
            c"ERROR: client %p on fd %d timed out, killing\n"
                .as_ptr()
                .cast_mut(),
            client,
            (*client).fd,
        );
    }
    free_ipc_client(client, -1);
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
    ipc_new_client_on_fd(loop_0, fd);
}

/// fd transfers a valid owned descriptor. All clients are Box-allocated here;
/// watcher pointers are stable Box allocations stopped before matching drops.
pub unsafe fn ipc_new_client_on_fd(loop_: *mut ev_loop, fd: libc::c_int) -> *mut ipc_client {
    set_nonblock(fd);
    let owner = std::os::fd::OwnedFd::from_raw_fd(fd);
    let read = Box::into_raw(Box::new(ev_io::new(fd, EV_READ, ipc_receive_message)));
    let write = Box::into_raw(Box::new(ev_io::new(fd, EV_WRITE, ipc_socket_writeable_cb)));
    let generation = NEXT_CLIENT_GENERATION;
    NEXT_CLIENT_GENERATION = generation
        .checked_add(1)
        .expect("IPC client generation exhausted");
    let client = Box::into_raw(Box::new(ipc_client {
        fd,
        generation,
        fd_owner: Some(owner),
        events: Vec::new(),
        first_tick_sent: false,
        read_callback: read,
        write_callback: write,
        timeout: std::ptr::null_mut(),
        buffer: Vec::new(),
        clients: crate::application::C2RustUnnamed_48 {
            tqe_next: std::ptr::null_mut(),
            tqe_prev: all_clients.tqh_last,
        },
    }));
    (*read).data = client.cast();
    (*write).data = client.cast();
    *all_clients.tqh_last = client;
    all_clients.tqh_last = &raw mut (*client).clients.tqe_next;
    ev_io_start(loop_, read);
    client
}

pub unsafe fn ipc_marshal_workspace_event(
    change: *const ::core::ffi::c_char,
    current: *mut Con,
    old: *mut Con,
) -> yajl_gen {
    let locale = NumericLocale::c();
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
    drop(locale);
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
        I3_IPC_EVENT_WORKSPACE as u32,
        payload as *const ::core::ffi::c_char,
    );
    drop(JsonOwner(gen));
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
    let locale = NumericLocale::c();
    let generator_owner = JsonOwner::new();
    let gen = generator_owner.0;
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
        I3_IPC_EVENT_WINDOW as u32,
        payload as *const ::core::ffi::c_char,
    );

    drop(locale);
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
    let locale = NumericLocale::c();
    let generator_owner = JsonOwner::new();
    let gen = generator_owner.0;
    dump_bar_config(gen, barconfig);
    let mut payload: *const ::core::ffi::c_uchar = ::core::ptr::null::<::core::ffi::c_uchar>();
    let mut length: ylength = 0;
    yajl_gen_get_buf(gen, &raw mut payload, &raw mut length);
    ipc_send_event(
        c"barconfig_update".as_ptr(),
        I3_IPC_EVENT_BARCONFIG_UPDATE as u32,
        payload as *const ::core::ffi::c_char,
    );

    drop(locale);
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
    let locale = NumericLocale::c();
    let generator_owner = JsonOwner::new();
    let gen = generator_owner.0;
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
        I3_IPC_EVENT_BINDING as u32,
        payload as *const ::core::ffi::c_char,
    );

    drop(locale);
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
    let reply = c"[{\"success\":true}]".as_ptr();
    ipc_send_client_message(
        client,
        strlen(reply),
        I3_IPC_REPLY_TYPE_COMMAND as u32,
        reply as *const u8,
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

#[cfg(test)]
mod owned_tests {
    use super::*;
    #[test]
    fn wire_frame_is_native_endian_and_preserves_binary_payload() {
        let frame = encode_frame(0x8000_0007, b"a\0\xff");
        assert_eq!(&frame[..6], b"i3-ipc");
        assert_eq!(&frame[6..10], &3u32.to_ne_bytes());
        assert_eq!(&frame[10..14], &0x8000_0007u32.to_ne_bytes());
        assert_eq!(&frame[14..], b"a\0\xff");
    }
    #[test]
    fn subscriptions_commit_prefixes_nuls_empty_names_and_unfinished_arrays() {
        let (events, status) = parse_subscriptions(br#"["","tick\u0000ignored","tick"] trailing"#);
        assert!(status.is_err());
        assert_eq!(
            events
                .iter()
                .map(|event| event.to_bytes())
                .collect::<Vec<_>>(),
            [b"".as_slice(), b"tick", b"tick"]
        );
        let (events, status) = parse_subscriptions(br#"["workspace""#);
        assert!(status.is_ok());
        assert_eq!(events[0].as_c_str(), c"workspace");
    }
    #[test]
    fn sync_values_keep_no_finish_key_projection_and_u32_casts() {
        assert_eq!(
            parse_sync(br#"{"rnd":-1,"WINDOW\u0000ignored":4294967297,"nested":{"rnd":2}}"#)
                .unwrap(),
            SyncRequest { rnd: 2, window: 1 }
        );
        assert_eq!(
            parse_sync(br#"{"rnd":3,"window":4,"#).unwrap(),
            SyncRequest { rnd: 3, window: 4 }
        );
        assert!(parse_sync(br#"{"rnd":9223372036854775808}"#).is_err());
        assert!(parse_sync(br#"{"rnd":-9223372036854775808}"#).is_err());
    }
    #[test]
    fn timeout_cmdline_handles_empty_embedded_and_full_buffers() {
        assert_eq!(diagnostic_cmdline(b"").as_c_str(), c"");
        assert_eq!(diagnostic_cmdline(b"a\0b\0").as_c_str(), c"a b");
        assert_eq!(diagnostic_cmdline(&[b'x'; 512]).to_bytes().len(), 512);
    }
}

#[cfg(test)]
mod client_lifetime_tests {
    use super::*;
    use std::io::{Read, Write};
    #[test]
    fn client_generation_and_fd_exemption_survive_dispatch_cleanup() {
        if std::env::var_os("I333_PRIVATE_IPC_PROBE").is_none() {
            let mut child = std::process::Command::new(std::env::current_exe().unwrap());
            child.args(["--exact","ipc::client_lifetime_tests::client_generation_and_fd_exemption_survive_dispatch_cleanup","--nocapture"])
                .env("I333_PRIVATE_IPC_PROBE","1");
            for key in [
                "DISPLAY",
                "WAYLAND_DISPLAY",
                "I3SOCK",
                "NOTIFY_SOCKET",
                "LISTEN_PID",
                "LISTEN_FDS",
                "LISTEN_FDNAMES",
                "LIBEV_FLAGS",
            ] {
                child.env_remove(key);
            }
            let output = child.output().unwrap();
            assert!(
                output.status.success(),
                "{}\n{}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
            return;
        }
        unsafe {
            let loop_ = libev_sys::ev_loop_new(0);
            assert!(!loop_.is_null());
            main_loop = loop_;
            let (stream, mut peer) = std::os::unix::net::UnixStream::pair().unwrap();
            let client = ipc_new_client_on_fd(loop_, stream.into_raw_fd());
            let token = ClientToken::capture(client);
            assert!(token.live());
            free_ipc_client(client, -1);
            assert!(!token.live());
            let mut byte = [0];
            assert_eq!(peer.read(&mut byte).unwrap(), 0);
            let (stream, mut peer) = std::os::unix::net::UnixStream::pair().unwrap();
            let fd = stream.into_raw_fd();
            let replacement = ipc_new_client_on_fd(loop_, fd);
            assert!(!ClientToken {
                pointer: replacement,
                generation: token.generation
            }
            .live());
            free_ipc_client(replacement, fd);
            let mut retained = std::os::unix::net::UnixStream::from_raw_fd(fd);
            retained.write_all(b"x").unwrap();
            peer.read_exact(&mut byte).unwrap();
            assert_eq!(&byte, b"x");
            drop(retained);
            libev_sys::ev_loop_destroy(loop_);
            main_loop = std::ptr::null_mut();
        }
    }
    #[test]
    fn value_serialization_keeps_signed_coordinates_gap_alias_and_modifier_order() {
        let mut gen = i3_json::Generator::new();
        gen.start_object().unwrap();
        write_fields(
            &mut gen,
            "rect",
            &rect_fields(Rect {
                x: u32::MAX,
                y: 0x8000_0000,
                width: u32::MAX,
                height: 1,
            }),
        )
        .unwrap();
        gen.end_object().unwrap();
        assert_eq!(
            gen.bytes(),
            br#"{"rect":{"x":-1,"y":-2147483648,"width":4294967295,"height":1}}"#
        );
        let fields = gaps_fields(gaps_t {
            inner: 1,
            top: 2,
            right: 3,
            bottom: 4,
            left: 5,
        });
        assert_eq!(fields[1], ("outer", 2));
        assert_eq!(
            modifier_names((1 << 19) | (1 << 13) | 1).collect::<Vec<_>>(),
            ["shift", "Group4"]
        );
    }
}
