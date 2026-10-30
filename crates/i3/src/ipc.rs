use crate::con::arena::focus;
use crate::con::arena::parent::parent_ptr;
#[path = "ipc_messages.rs"]
mod messages;
use crate::commands_parser::command_result_free;
use crate::commands_parser::parse_command;
use crate::con::arena::{containers, floating, identity, resolve};
use crate::config::barconfigs;
use crate::config::included_files;
use crate::config::modes;
use crate::randr::outputs;
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
use messages::{WireFields, WireValue};
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
pub use libev_sys::{
    ev_io, ev_io_start, ev_io_stop, ev_loop, ev_timer, ev_timer_start, ev_timer_stop, ev_tstamp,
    ev_watcher, ev_watcher_time, EV_READ, EV_WRITE, EV__IOFDSET,
};

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
    let event = messages::Shutdown {
        change: match reason {
            ShutdownReason::Restart => "restart",
            ShutdownReason::Exit => "exit",
        },
    };
    send_typed_event(c"shutdown", I3_IPC_EVENT_SHUTDOWN as u32, &event);
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
/// Complete command response values. The parser owns their stream; command
/// handlers borrow native message bytes only while appending one response.
pub enum CommandReply<'a> {
    Success(bool),
    Error(&'a CStr),
    WithId(i64),
    ParseError {
        error: &'a CStr,
        input: &'a CStr,
        position: &'a CStr,
    },
}
#[derive(Clone, Copy)]
enum ReplyField<'a> {
    Bool(bool),
    Integer(i64),
    Text(&'a CStr),
}
impl serde::Serialize for ReplyField<'_> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Bool(value) => serializer.serialize_bool(*value),
            Self::Integer(value) => serializer.serialize_i64(*value),
            Self::Text(value) => {
                serializer.serialize_str(value.to_str().map_err(serde::ser::Error::custom)?)
            }
        }
    }
}
impl CommandReply<'_> {
    fn fields(&self) -> Vec<(&'static str, ReplyField<'_>)> {
        use ReplyField::*;
        match self {
            Self::Success(value) => vec![("success", Bool(*value))],
            Self::Error(error) => vec![("success", Bool(false)), ("error", Text(error))],
            Self::WithId(id) => vec![("success", Bool(true)), ("id", Integer(*id))],
            Self::ParseError {
                error,
                input,
                position,
            } => vec![
                ("success", Bool(false)),
                ("parse_error", Bool(true)),
                ("error", Text(error)),
                ("input", Text(input)),
                ("errorposition", Text(position)),
            ],
        }
    }
    fn write(&self, output: &mut i3_json::Generator) {
        let fields = self.fields();
        if fields
            .iter()
            .all(|(_, value)| !matches!(value, ReplyField::Text(text) if text.to_str().is_err()))
        {
            let _ = output.serialized(self);
            return;
        }
        // Legacy generators reject invalid UTF-8 without consuming a value.
        // Preserve the subsequent key/value state and partial output exactly;
        // repairing native bytes or dropping the whole response changes IPC.
        let _ = output.start_object();
        for (key, value) in fields {
            let _ = output.string(key);
            match value {
                ReplyField::Bool(value) => {
                    let _ = output.boolean(value);
                }
                ReplyField::Integer(value) => {
                    let _ = output.integer(value);
                }
                ReplyField::Text(value) => {
                    if let Ok(value) = value.to_str() {
                        let _ = output.string(value);
                    }
                }
            }
        }
        let _ = output.end_object();
    }
}
impl serde::Serialize for CommandReply<'_> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let fields = self.fields();
        let mut map = serializer.serialize_map(Some(fields.len()))?;
        for (key, value) in fields {
            map.serialize_entry(key, &value)?;
        }
        map.end()
    }
}
/// # Safety
/// `command` and its parser context are live for this synchronous dispatch.
/// No references into either object survive the short append operation.
pub unsafe fn emit_command_reply(
    command: *mut crate::commands::CommandResultIR,
    reply: CommandReply<'_>,
) {
    let context = unsafe { (*command).ctx };
    if let Some(output) = unsafe { &mut (*context).json_output } {
        reply.write(output);
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
    let mut output = i3_json::Generator::new();
    let result = CommandResultOwner(parse_command(command.as_ptr(), Some(&mut output), client));
    if (*result.0).needs_tree_render {
        tree_render();
    }
    drop(result);
    // Reload/restart/command callbacks can free this client, even if exec later
    // fails. Pointer membership alone is insufficient if the address was reused.
    if !token.live() {
        return;
    }
    let reply = output.bytes();
    ipc_send_client_message(
        client,
        reply.len(),
        I3_IPC_REPLY_TYPE_COMMAND as u32,
        reply.as_ptr(),
    );
}
unsafe fn native_text(value: *const libc::c_char) -> messages::NativeText {
    messages::NativeText::cstr(CStr::from_ptr(value))
}
fn unsigned_rect(rect: Rect) -> messages::Rectangle {
    messages::Rectangle {
        x: rect.x as i64,
        y: rect.y as i64,
        width: rect.width as i64,
        height: rect.height as i64,
    }
}
unsafe fn send_typed_reply<T: WireValue>(client: *mut ipc_client, message_type: u32, value: &T) {
    let mut output = i3_json::Generator::new();
    value.write(&mut output);
    let payload = output.bytes();
    ipc_send_client_message(client, payload.len(), message_type, payload.as_ptr());
}
unsafe fn send_typed_event<T: WireValue>(name: &CStr, message_type: u32, value: &T) {
    let mut output = i3_json::Generator::new();
    value.write(&mut output);
    let payload = CString::new(output.bytes()).expect("JSON has no literal NUL");
    ipc_send_event(name.as_ptr(), message_type, payload.as_ptr());
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
unsafe fn snapshot_binding(binding: *mut Binding) -> messages::Binding {
    let modifiers: Vec<_> = modifier_names((*binding).event_state_mask).collect();
    messages::Binding {
        input_code: (*binding).keycode as i64,
        input_type: if (*binding).input_type == InputType::Keyboard {
            "keyboard"
        } else {
            "mouse"
        },
        symbol: (!(*binding).symbol.is_null()).then(|| native_text((*binding).symbol)),
        command: native_text((*binding).command),
        mods: modifiers.clone(),
        event_state_mask: modifiers,
    }
}

fn signed_rect(rect: Rect) -> messages::Rectangle {
    messages::Rectangle {
        x: rect.x as i32 as i64,
        y: rect.y as i32 as i64,
        width: rect.width as i64,
        height: rect.height as i64,
    }
}
fn gap_values(gaps: gaps_t) -> messages::Gaps {
    messages::Gaps {
        inner: gaps.inner as i64,
        outer: gaps.top as i64,
        top: gaps.top as i64,
        right: gaps.right as i64,
        bottom: gaps.bottom as i64,
        left: gaps.left as i64,
    }
}
unsafe fn optional_text(value: *const libc::c_char) -> messages::Omitted<messages::NativeText> {
    messages::Omitted((!value.is_null()).then(|| native_text(value)))
}
unsafe fn node_layout(con: *mut Con) -> &'static str {
    match (*con).layout {
        Layout::Default => {
            debuglog(
                c"%s:%s:%d - About to dump layout=default, this is a bug in the code.\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/ipc.c".as_ptr(),
                c"dump_node".as_ptr(),
                442,
            );
            __assert_fail(
                c"false".as_ptr(),
                c"../src/ipc.c".as_ptr(),
                443,
                c"void dump_node(yajl_gen, Con *, _Bool)".as_ptr(),
            );
        }
        Layout::SplitV => "splitv",
        Layout::SplitH => "splith",
        Layout::Stacked => "stacked",
        Layout::Tabbed => "tabbed",
        Layout::DockArea => "dockarea",
        Layout::Output => "output",
    }
}
unsafe fn node_workspace_layout(con: *mut Con) -> &'static str {
    match (*con).workspace_layout {
        Layout::Default => "default",
        Layout::Stacked => "stacked",
        Layout::Tabbed => "tabbed",
        Layout::DockArea | Layout::Output | Layout::SplitV | Layout::SplitH => {
            debuglog(c"%s:%s:%d - About to dump workspace_layout=%d (none of default/stacked/tabbed), this is a bug.\n".as_ptr().cast_mut(), c"../src/ipc.c".as_ptr(), c"dump_node".as_ptr(), 477, (*con).workspace_layout as u32);
            __assert_fail(
                c"false".as_ptr(),
                c"../src/ipc.c".as_ptr(),
                478,
                c"void dump_node(yajl_gen, Con *, _Bool)".as_ptr(),
            );
        }
    }
}
unsafe fn window_kind(window: *mut crate::types::Window) -> &'static str {
    [
        (A__NET_WM_WINDOW_TYPE_NORMAL, "normal"),
        (A__NET_WM_WINDOW_TYPE_DOCK, "dock"),
        (A__NET_WM_WINDOW_TYPE_DIALOG, "dialog"),
        (A__NET_WM_WINDOW_TYPE_UTILITY, "utility"),
        (A__NET_WM_WINDOW_TYPE_TOOLBAR, "toolbar"),
        (A__NET_WM_WINDOW_TYPE_SPLASH, "splash"),
        (A__NET_WM_WINDOW_TYPE_MENU, "menu"),
        (A__NET_WM_WINDOW_TYPE_DROPDOWN_MENU, "dropdown_menu"),
        (A__NET_WM_WINDOW_TYPE_POPUP_MENU, "popup_menu"),
        (A__NET_WM_WINDOW_TYPE_TOOLTIP, "tooltip"),
        (A__NET_WM_WINDOW_TYPE_NOTIFICATION, "notification"),
    ]
    .into_iter()
    .find_map(|(atom, name)| (atom == (*window).window_type).then_some(name))
    .unwrap_or("unknown")
}
unsafe fn snapshot_node_header(con: *mut Con, restart: bool) -> messages::NodeHeader {
    use messages::{NativeValue, Omitted};
    let orientation = if !con_is_split(con) {
        "none"
    } else if con_orientation(con) == Orientation::Horizontal {
        "horizontal"
    } else {
        "vertical"
    };
    let mut marks = Vec::new();
    let mut mark = (*con).marks_head.tqh_first;
    while !mark.is_null() {
        marks.push(native_text((*mark).name));
        mark = (*mark).marks.tqe_next;
    }
    let output = Omitted(
        ((*con).type_0 != CT_ROOT as u32 && (*con).type_0 != CT_OUTPUT as u32)
            .then(|| native_text((*con_get_output(con)).name)),
    );
    let layout = node_layout(con);
    let workspace_layout = node_workspace_layout(con);
    let decoration_in_frame = con_draw_decoration_into_frame(con);
    let mut deco_rect = (*con).deco_rect;
    if decoration_in_frame {
        deco_rect.x = (*con).rect.x.wrapping_sub((*parent_ptr(con)).rect.x);
        deco_rect.y = (*con).rect.y.wrapping_sub((*parent_ptr(con)).rect.y);
    }
    let window = (*con).window;
    let window_title = (!window.is_null())
        .then(|| (*window).name.as_ref())
        .flatten();
    let name = if let Some(title) = window_title {
        match title.cstr_or_report() {
            Some(title) => NativeValue::Value(Some(messages::NativeText::cstr(title))),
            None => NativeValue::Rejected,
        }
    } else {
        NativeValue::Value((!(*con).name.is_null()).then(|| native_text((*con).name)))
    };
    let properties = Omitted((!window.is_null() && !restart).then(|| {
        messages::WindowProperties {
            class: optional_text((*window).class_class),
            instance: optional_text((*window).class_instance),
            window_role: optional_text((*window).role),
            machine: optional_text((*window).machine),
            title: Omitted(
                (*window)
                    .name
                    .as_ref()
                    .map(|title| match title.cstr_or_report() {
                        Some(title) => NativeValue::Value(messages::NativeText::cstr(title)),
                        None => NativeValue::Rejected,
                    }),
            ),
            transient_for: ((*window).transient_for as libc::c_long != XCB_NONE)
                .then_some((*window).transient_for as i64),
        }
    }));
    messages::NodeHeader {
        id: con as usize as i64,
        r#type: match (*con).type_0 {
            0 => NativeValue::Value("root"),
            1 => NativeValue::Value("output"),
            2 => NativeValue::Value("con"),
            3 => NativeValue::Value("floating_con"),
            4 => NativeValue::Value("workspace"),
            5 => NativeValue::Value("dockarea"),
            _ => NativeValue::Rejected,
        },
        orientation,
        scratchpad_state: match (*con).scratchpad_state {
            ScratchpadState::None => "none",
            ScratchpadState::Fresh => "fresh",
            ScratchpadState::Changed => "changed",
        },
        percent: ((*con).percent != 0.0).then_some((*con).percent),
        urgent: (*con).urgent,
        marks,
        focused: con == focused,
        output,
        layout,
        workspace_layout,
        // Upstream intentionally reports this from layout, not the similarly
        // named last_split_layout member.
        last_split_layout: if (*con).layout == Layout::SplitV {
            "splitv"
        } else {
            "splith"
        },
        border: match (*con).border_style {
            BorderStyle::Normal => "normal",
            BorderStyle::None => "none",
            BorderStyle::Pixel => "pixel",
        },
        current_border_width: (*con).current_border_width as i64,
        rect: signed_rect((*con).rect),
        deco_rect: signed_rect(deco_rect),
        actual_deco_rect: Omitted(decoration_in_frame.then(|| signed_rect((*con).deco_rect))),
        window_rect: signed_rect((*con).window_rect),
        geometry: signed_rect((*con).geometry),
        name,
        title_format: optional_text((*con).title_format),
        window_icon_padding: (*con).window_icon_padding as i64,
        num: Omitted(((*con).type_0 == CT_WORKSPACE as u32).then_some((*con).num as i64)),
        gaps: Omitted(((*con).type_0 == CT_WORKSPACE as u32).then_some(gap_values((*con).gaps))),
        window: (!window.is_null()).then(|| (*window).id as i64),
        window_type: (!window.is_null()).then(|| window_kind(window)),
        window_properties: properties,
    }
}
unsafe fn optional_pattern(
    pattern: *mut crate::regex::Regex,
) -> messages::Omitted<messages::NativeText> {
    messages::Omitted(
        (!pattern.is_null()).then(|| messages::NativeText::cstr((*pattern).pattern())),
    )
}
unsafe fn snapshot_node_tail(con: *mut Con, restart: bool) -> messages::NodeTail {
    use messages::Omitted;
    let mut focus = Vec::new();
    for id in focus::children(identity(con).expect("live focus parent")) {
        if let Some(child) = resolve(id) {
            focus.push(child as usize as i64);
        }
    }
    let mut swallows = Vec::new();
    let mut swallow = (*con).swallow_head.tqh_first;
    while !swallow.is_null() {
        if !(*swallow).restart_mode {
            let dock = (*swallow).dock as libc::c_int != M_DONTCHECK as libc::c_int;
            swallows.push(messages::Swallow {
                dock: Omitted(dock.then_some((*swallow).dock as i64)),
                insert_where: Omitted(dock.then_some((*swallow).insert_where as i64)),
                class: optional_pattern((*swallow).class),
                instance: optional_pattern((*swallow).instance),
                window_role: optional_pattern((*swallow).window_role),
                title: optional_pattern((*swallow).title),
                machine: optional_pattern((*swallow).machine),
                id: Omitted(None),
                restart_mode: Omitted(None),
            });
        }
        swallow = (*swallow).matches.tqe_next;
    }
    let window = (*con).window;
    if restart && !window.is_null() {
        swallows.push(messages::Swallow {
            dock: Omitted(None),
            insert_where: Omitted(None),
            class: Omitted(None),
            instance: Omitted(None),
            window_role: Omitted(None),
            title: Omitted(None),
            machine: Omitted(None),
            id: Omitted(Some((*window).id as i64)),
            restart_mode: Omitted(Some(true)),
        });
    }
    messages::NodeTail {
        focus,
        fullscreen_mode: (*con).fullscreen_mode as i64,
        sticky: (*con).sticky,
        floating: match (*con).floating {
            FloatingState::AutoOff => "auto_off",
            FloatingState::AutoOn => "auto_on",
            FloatingState::UserOff => "user_off",
            FloatingState::UserOn => "user_on",
        },
        swallows,
        depth: Omitted((restart && !window.is_null()).then_some((*con).depth as i64)),
        previous_workspace_name: Omitted(
            (restart && (*con).type_0 == CT_ROOT as u32 && !previous_workspace_name.is_null())
                .then(|| native_text(previous_workspace_name)),
        ),
    }
}
/// Native graph traversal remains streaming: Serde sees only bounded value
/// segments, never a recursively constructed copy of the complete window tree.
pub unsafe fn dump_node(output: &mut i3_json::Generator, con: *mut Con, restart: bool) {
    let mut compatibility = false;
    dump_node_stream(output, con, restart, &mut compatibility);
}
unsafe fn dump_node_stream(
    output: &mut i3_json::Generator,
    con: *mut Con,
    restart: bool,
    compatibility: &mut bool,
) {
    use messages::WireFields;
    if output.start_object().is_err() {
        *compatibility = true;
    }
    snapshot_node_header(con, restart).fields(output, compatibility);
    let _ = output.string("nodes");
    if output.start_array().is_err() {
        *compatibility = true;
    }
    if (*con).type_0 != CT_DOCKAREA as u32 || !restart {
        let mut child = (*con).nodes_head.tqh_first;
        while !child.is_null() {
            dump_node_stream(output, child, restart, compatibility);
            child = (*child).nodes.tqe_next;
        }
    }
    let _ = output.end_array();
    let _ = output.string("floating_nodes");
    if output.start_array().is_err() {
        *compatibility = true;
    }
    for id in floating::children(identity(con).expect("live floating parent")) {
        let Some(child) = resolve(id) else { continue };
        dump_node_stream(output, child, restart, compatibility);
    }
    let _ = output.end_array();
    snapshot_node_tail(con, restart).fields(output, compatibility);
    let _ = output.end_object();
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
unsafe fn snapshot_bar_config(config: *mut Barconfig) -> messages::BarConfig {
    use messages::Omitted;
    let mut configured_outputs = Vec::new();
    for index in 0..(*config).num_outputs.max(0) {
        configured_outputs.push(native_text(canonicalize_output_name(
            *(*config).outputs.add(index as usize),
        )));
    }
    let mut trays = Vec::new();
    let mut tray = (*config).tray_outputs.tqh_first;
    while !tray.is_null() {
        trays.push(native_text(canonicalize_output_name((*tray).output)));
        tray = (*tray).tray_outputs.tqe_next;
    }
    let mut bindings = Vec::new();
    let mut binding = (*config).bar_bindings.tqh_first;
    while !binding.is_null() {
        bindings.push(messages::BarBinding {
            input_code: (*binding).input_code as i64,
            command: native_text((*binding).command),
            release: (*binding).release,
        });
        binding = (*binding).bindings.tqe_next;
    }
    let colors = messages::BarColors {
        background: optional_text((*config).colors.background),
        statusline: optional_text((*config).colors.statusline),
        separator: optional_text((*config).colors.separator),
        focused_background: optional_text((*config).colors.focused_background),
        focused_statusline: optional_text((*config).colors.focused_statusline),
        focused_separator: optional_text((*config).colors.focused_separator),
        focused_workspace_border: optional_text((*config).colors.focused_workspace_border),
        focused_workspace_bg: optional_text((*config).colors.focused_workspace_bg),
        focused_workspace_text: optional_text((*config).colors.focused_workspace_text),
        active_workspace_border: optional_text((*config).colors.active_workspace_border),
        active_workspace_bg: optional_text((*config).colors.active_workspace_bg),
        active_workspace_text: optional_text((*config).colors.active_workspace_text),
        inactive_workspace_border: optional_text((*config).colors.inactive_workspace_border),
        inactive_workspace_bg: optional_text((*config).colors.inactive_workspace_bg),
        inactive_workspace_text: optional_text((*config).colors.inactive_workspace_text),
        urgent_workspace_border: optional_text((*config).colors.urgent_workspace_border),
        urgent_workspace_bg: optional_text((*config).colors.urgent_workspace_bg),
        urgent_workspace_text: optional_text((*config).colors.urgent_workspace_text),
        binding_mode_border: optional_text((*config).colors.binding_mode_border),
        binding_mode_bg: optional_text((*config).colors.binding_mode_bg),
        binding_mode_text: optional_text((*config).colors.binding_mode_text),
    };
    messages::BarConfig {
        id: native_text((*config).id),
        outputs: Omitted(((*config).num_outputs > 0).then_some(configured_outputs)),
        tray_outputs: Omitted((!trays.is_empty()).then_some(trays)),
        tray_padding: (*config).tray_padding as i64,
        socket_path: optional_text((*config).socket_path),
        mode: match (*config).mode as u32 {
            1 => "hide",
            2 => "invisible",
            _ => "dock",
        },
        hidden_state: match (*config).hidden_state {
            BarHiddenState::Show => "show",
            BarHiddenState::Hide => "hide",
        },
        modifier: (*config).modifier as i64,
        bindings: Omitted((!bindings.is_empty()).then_some(bindings)),
        position: if (*config).position == BarPosition::Bottom {
            "bottom"
        } else {
            "top"
        },
        status_command: optional_text((*config).status_command),
        workspace_command: optional_text((*config).workspace_command),
        font: optional_text((*config).font),
        bar_height: Omitted(((*config).bar_height != 0).then_some((*config).bar_height as i64)),
        padding: signed_rect((*config).padding),
        separator_symbol: optional_text((*config).separator_symbol),
        workspace_buttons: !(*config).hide_workspace_buttons,
        workspace_min_width: (*config).workspace_min_width as i64,
        strip_workspace_numbers: (*config).strip_workspace_numbers,
        strip_workspace_name: (*config).strip_workspace_name,
        binding_mode_indicator: !(*config).hide_binding_mode_indicator,
        verbose: (*config).verbose,
        colors,
    }
}
unsafe fn dump_bar_config(output: &mut i3_json::Generator, config: *mut Barconfig) {
    snapshot_bar_config(config).write(output);
}

unsafe fn handle_tree(
    client: *mut ipc_client,
    message: *mut u8,
    size: ::core::ffi::c_int,
    message_size: u32,
    message_type: u32,
) {
    let locale = NumericLocale::c();
    let mut generator = i3_json::Generator::new();
    let gen = &mut generator;
    dump_node(gen, croot, false_0 != 0);
    drop(locale);
    let payload = gen.bytes();
    let length = payload.len();
    ipc_send_client_message(
        client,
        length as usize,
        I3_IPC_REPLY_TYPE_TREE as u32,
        payload.as_ptr(),
    );
}
unsafe fn handle_get_workspaces(
    client: *mut ipc_client,
    message: *mut u8,
    size: ::core::ffi::c_int,
    message_size: u32,
    message_type: u32,
) {
    let mut workspaces = Vec::new();
    let focused_workspace = con_get_workspace(focused);
    let mut output = (*croot).nodes_head.tqh_first;
    while !output.is_null() {
        if !con_is_internal(output) {
            let mut workspace = (*output_get_content(output)).nodes_head.tqh_first;
            while !workspace.is_null() {
                assert!(
                    (*workspace).type_0 == CT_WORKSPACE as u32,
                    "workspace type in output content"
                );
                workspaces.push(messages::Workspace {
                    id: workspace as usize as i64,
                    num: (*workspace).num as i64,
                    name: native_text((*workspace).name),
                    visible: workspace_is_visible(workspace),
                    focused: workspace == focused_workspace,
                    rect: unsigned_rect((*workspace).rect),
                    output: native_text((*output).name),
                    urgent: (*workspace).urgent,
                });
                workspace = (*workspace).nodes.tqe_next;
            }
        }
        output = (*output).nodes.tqe_next;
    }
    send_typed_reply(client, I3_IPC_REPLY_TYPE_WORKSPACES as u32, &workspaces);
}
unsafe fn handle_get_outputs(
    client: *mut ipc_client,
    message: *mut u8,
    size: ::core::ffi::c_int,
    message_size: u32,
    message_type: u32,
) {
    let mut result = Vec::new();
    let mut output = outputs.tqh_first;
    while !output.is_null() {
        let workspace = if (*output).con.is_null() {
            std::ptr::null_mut()
        } else {
            con_get_fullscreen_con((*output).con, CF_OUTPUT)
        };
        result.push(messages::Output {
            name: native_text(output_primary_name(output)),
            active: (*output).active,
            primary: (*output).primary,
            rect: unsigned_rect((*output).rect),
            current_workspace: (!workspace.is_null()).then(|| native_text((*workspace).name)),
        });
        output = (*output).outputs.tqe_next;
    }
    send_typed_reply(client, I3_IPC_REPLY_TYPE_OUTPUTS as u32, &result);
}
unsafe fn handle_get_marks(
    client: *mut ipc_client,
    message: *mut u8,
    size: ::core::ffi::c_int,
    message_size: u32,
    message_type: u32,
) {
    let mut marks = Vec::new();
    for id in containers() {
        let Some(con) = resolve(id) else { continue };
        let mut mark = (*con).marks_head.tqh_first;
        while !mark.is_null() {
            marks.push(native_text((*mark).name));
            mark = (*mark).marks.tqe_next;
        }
    }
    send_typed_reply(client, I3_IPC_REPLY_TYPE_MARKS as u32, &marks);
}
unsafe fn handle_get_version(
    client: *mut ipc_client,
    message: *mut u8,
    size: ::core::ffi::c_int,
    message_size: u32,
    message_type: u32,
) {
    let mut included_config_file_names = Vec::new();
    let mut file = included_files.tqh_first;
    while !file.is_null() {
        if file != included_files.tqh_first {
            included_config_file_names.push(native_text((*file).path));
        }
        file = (*file).files.tqe_next;
    }
    let reply = messages::Version {
        major: MAJOR_VERSION as i64,
        minor: MINOR_VERSION as i64,
        patch: PATCH_VERSION as i64,
        human_readable: native_text(i3_version),
        loaded_config_file_name: native_text(current_configpath),
        included_config_file_names,
    };
    send_typed_reply(client, I3_IPC_REPLY_TYPE_VERSION as u32, &reply);
}
unsafe fn handle_get_bar_config(
    client: *mut ipc_client,
    message: *mut u8,
    size: ::core::ffi::c_int,
    message_size: u32,
    message_type: u32,
) {
    let mut generator = i3_json::Generator::new();
    let gen = &mut generator;
    if message_size == 0 {
        let mut names = Vec::new();
        let mut current = barconfigs.tqh_first;
        while !current.is_null() {
            names.push(native_text((*current).id));
            current = (*current).configs.tqe_next;
        }
        send_typed_reply(client, I3_IPC_REPLY_TYPE_BAR_CONFIG as u32, &names);
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
        messages::MissingBar { id: None }.write(gen);
    } else {
        dump_bar_config(gen, config);
    }
    let payload_0 = gen.bytes();
    let length_0 = payload_0.len();
    ipc_send_client_message(
        client,
        length_0 as usize,
        I3_IPC_REPLY_TYPE_BAR_CONFIG as u32,
        payload_0.as_ptr(),
    );
}
unsafe fn handle_get_binding_modes(
    client: *mut ipc_client,
    message: *mut u8,
    size: ::core::ffi::c_int,
    message_size: u32,
    message_type: u32,
) {
    let mut names = Vec::new();
    let mut mode = modes.slh_first;
    while !mode.is_null() {
        names.push(native_text((*mode).name));
        mode = (*mode).modes.sle_next;
    }
    send_typed_reply(client, I3_IPC_REPLY_TYPE_BINDING_MODES as u32, &names);
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
    let mut configs = Vec::new();
    let first = included_files.tqh_first;
    let config = native_text((*first).raw_contents);
    let mut file = first;
    while !file.is_null() {
        configs.push(messages::IncludedConfig {
            path: native_text((*file).path),
            raw_contents: native_text((*file).raw_contents),
            variable_replaced_contents: native_text((*file).variable_replaced_contents),
        });
        file = (*file).files.tqe_next;
    }
    let reply = messages::Config {
        config,
        included_configs: configs,
    };
    send_typed_reply(client, I3_IPC_REPLY_TYPE_CONFIG as u32, &reply);
}
unsafe fn handle_send_tick(
    client: *mut ipc_client,
    message: *mut u8,
    size: ::core::ffi::c_int,
    message_size: u32,
    message_type: u32,
) {
    let event = messages::Tick {
        first: false,
        payload: messages::NativeText::bytes(std::slice::from_raw_parts(
            message,
            message_size as usize,
        )),
    };
    send_typed_event(c"tick", I3_IPC_EVENT_TICK as u32, &event);

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
    let reply = messages::BindingState {
        name: native_text(current_binding_mode),
    };
    send_typed_reply(client, I3_IPC_REPLY_TYPE_GET_BINDING_STATE as u32, &reply);
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
) -> CString {
    let locale = NumericLocale::c();
    let mut generator = i3_json::Generator::new();
    let gen = &mut generator;
    let mut compatibility = false;
    let _ = gen.start_object();
    messages::Change {
        change: native_text(change),
    }
    .fields(gen, &mut compatibility);
    let _ = gen.string("current");
    if current.is_null() {
        let _ = gen.null();
    } else {
        dump_node_stream(gen, current, false, &mut compatibility);
    }
    let _ = gen.string("old");
    if old.is_null() {
        let _ = gen.null();
    } else {
        dump_node_stream(gen, old, false, &mut compatibility);
    }
    let _ = gen.end_object();
    let payload = CString::new(gen.bytes()).expect("JSON has no literal NUL");
    drop(locale);
    payload
}

pub unsafe fn ipc_send_workspace_event(
    change: *const ::core::ffi::c_char,
    current: *mut Con,
    old: *mut Con,
) {
    let payload = ipc_marshal_workspace_event(change, current, old);
    ipc_send_event(
        c"workspace".as_ptr(),
        I3_IPC_EVENT_WORKSPACE as u32,
        payload.as_ptr(),
    );
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
    let mut generator = i3_json::Generator::new();
    let gen = &mut generator;
    let mut compatibility = false;
    let _ = gen.start_object();
    messages::Change {
        change: native_text(property),
    }
    .fields(gen, &mut compatibility);
    let _ = gen.string("container");
    dump_node_stream(gen, con, false, &mut compatibility);
    let _ = gen.end_object();
    let payload = gen.bytes();
    let length = payload.len();
    ipc_send_event(
        c"window".as_ptr(),
        I3_IPC_EVENT_WINDOW as u32,
        CString::new(payload)
            .expect("JSON has no literal NUL")
            .as_ptr(),
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
    let mut generator = i3_json::Generator::new();
    let gen = &mut generator;
    dump_bar_config(gen, barconfig);
    let payload = gen.bytes();
    let length = payload.len();
    ipc_send_event(
        c"barconfig_update".as_ptr(),
        I3_IPC_EVENT_BARCONFIG_UPDATE as u32,
        CString::new(payload)
            .expect("JSON has no literal NUL")
            .as_ptr(),
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
    let event = messages::BindingEvent {
        change: native_text(event_type),
        mode: native_text(if modename.is_null() {
            c"default".as_ptr()
        } else {
            modename
        }),
        binding: snapshot_binding(bind),
    };
    send_typed_event(c"binding", I3_IPC_EVENT_BINDING as u32, &event);
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
        gen.string("rect").unwrap();
        signed_rect(Rect {
            x: u32::MAX,
            y: 0x8000_0000,
            width: u32::MAX,
            height: 1,
        })
        .write(&mut gen);
        gen.end_object().unwrap();
        assert_eq!(
            gen.bytes(),
            br#"{"rect":{"x":-1,"y":-2147483648,"width":4294967295,"height":1}}"#
        );
        let gaps = gap_values(gaps_t {
            inner: 1,
            top: 2,
            right: 3,
            bottom: 4,
            left: 5,
        });
        assert_eq!(gaps.outer, 2);
        assert_eq!(
            modifier_names((1 << 19) | (1 << 13) | 1).collect::<Vec<_>>(),
            ["shift", "Group4"]
        );
    }
}

#[cfg(test)]
mod typed_reply_tests {
    use super::*;
    fn stream(replies: &[CommandReply<'_>]) -> Vec<u8> {
        let mut output = i3_json::Generator::new();
        output.start_array().unwrap();
        for reply in replies {
            reply.write(&mut output);
        }
        let _ = output.end_array();
        output.bytes().to_vec()
    }
    #[test]
    fn typed_command_replies_preserve_field_order_and_escaping() {
        let reply = stream(&[
            CommandReply::Success(true),
            CommandReply::Success(false),
            CommandReply::Error(c"quoted \"value\"\n雪"),
            CommandReply::WithId(-1),
            CommandReply::ParseError {
                error: c"bad",
                input: c"input",
                position: c"  ^^^",
            },
        ]);
        assert_eq!(reply, "[{\"success\":true},{\"success\":false},{\"success\":false,\"error\":\"quoted \\\"value\\\"\\n雪\"},{\"success\":true,\"id\":-1},{\"success\":false,\"parse_error\":true,\"error\":\"bad\",\"input\":\"input\",\"errorposition\":\"  ^^^\"}]".as_bytes());
    }
    #[test]
    fn invalid_native_strings_preserve_rejected_value_and_partial_stream() {
        let invalid = CStr::from_bytes_with_nul(b"\xff\0").unwrap();
        assert_eq!(
            stream(&[CommandReply::Error(invalid), CommandReply::Success(true)]),
            br#"[{"success":false,"error""#
        );
        // Rejected input consumes no value: the next key becomes its value,
        // exactly as before; the following text becomes an unmatched key.
        assert_eq!(
            stream(&[CommandReply::ParseError {
                error: c"bad",
                input: invalid,
                position: c"^^"
            }]),
            br#"[{"success":false,"parse_error":true,"error":"bad","input":"errorposition","^^""#
        );
    }
    #[test]
    fn typed_container_ids_preserve_signed_pointer_cast_range() {
        let mut output = i3_json::Generator::new();
        output.start_array().unwrap();
        CommandReply::WithId(i64::MIN).write(&mut output);
        output.end_array().unwrap();
        assert_eq!(
            output.bytes(),
            b"[{\"success\":true,\"id\":-9223372036854775808}]"
        );
    }
}

#[cfg(test)]
#[path = "ipc_snapshot_tests.rs"]
mod snapshot_tests;
