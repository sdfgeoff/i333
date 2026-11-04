use crate::config::barconfigs;
use crate::domains::{
    ContainerType, DockMatch, FullscreenMode, HideEdgeBordersMode, MatchInsertWhere,
};
use crate::flags::{GapMask, TilingDrag};
use crate::handlers::event_is_ignored;
use crate::ipc::ipc_confirm_restart;
use crate::ipc::ipc_new_client_on_fd;
use crate::randr::outputs;
use crate::types::BarHiddenState;
use crate::types::BarMode;
use crate::types::BarPosition;
use crate::types::BindingRelease;
use crate::types::ConfigLoad;
use crate::types::InputType;
use crate::types::ShutdownReason;
use crate::xcursor::Cursor;
use i3_common::display::{CairoVisual, DisplayScreen, ScreenInfo};
use std::ffi::{CStr, CString};
use std::io::Read;
use std::os::fd::{AsRawFd, FromRawFd};
use std::ptr::NonNull;
use std::rc::Rc;
use xcb::ffi::xcb_connection_t;
use xcb::{shape, x, xkb, Raw, Xid, XidNew};
/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * main.c: Initialization, main loop
 *
 */
// Mechanical file-by-file Rust translation of src/main.c.
// Originally translated with C2Rust 0.22.1; subsequently maintained in Rust.
use ::libc;
pub use libev_sys::{
    ev_default_loop, ev_feed_event, ev_io, ev_io_start, ev_loop, ev_loop_destroy, ev_prepare,
    ev_prepare_start, ev_prepare_stop, ev_signal, ev_signal_start, ev_timer, ev_unref, ev_watcher,
    ev_watcher_list, EV_READ, EV__IOFDSET,
};

pub type i3Font = Font;

pub struct Binding {
    pub input_type: InputType,
    pub release: BindingRelease,
    pub border: bool,
    pub whole_window: bool,
    pub exclude_titlebar: bool,
    pub keycode: u32,
    pub event_state_mask: i3_event_state_mask_t,
    pub symbol: Option<Vec<u8>>,
    pub keycodes_head: keycodes_head,
    pub command: Vec<u8>,
    pub bindings: BindingLinks,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BindingLinks {
    pub tqe_next: *mut Binding,
    pub tqe_prev: *mut *mut Binding,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct keycodes_head {
    pub tqh_first: *mut Binding_Keycode,
    pub tqh_last: *mut *mut Binding_Keycode,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Binding_Keycode {
    pub keycode: u8,
    pub modifiers: i3_event_state_mask_t,
    pub keycodes: BindingKeycodeLinks,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BindingKeycodeLinks {
    pub tqe_next: *mut Binding_Keycode,
    pub tqe_prev: *mut *mut Binding_Keycode,
}
pub type i3_event_state_mask_t = u32;

pub type Output = xoutput;

pub type gaps_mask_t = GapMask;

pub struct Workspace_Assignment {
    pub name: Vec<u8>,
    /// None is a gaps-only record; Some(empty) remains an output assignment.
    pub output: Option<Vec<u8>>,
    pub gaps: gaps_t,
    pub gaps_mask: gaps_mask_t,
    pub ws_assignments: WorkspaceAssignmentLinks,
}
impl Workspace_Assignment {
    pub fn new(name: Vec<u8>, output: Option<Vec<u8>>) -> Self {
        Self {
            name: name.split(|byte| *byte == 0).next().unwrap().to_vec(),
            output: output.map(|value| value.split(|byte| *byte == 0).next().unwrap().to_vec()),
            gaps: gaps_t {
                inner: 0,
                top: 0,
                right: 0,
                bottom: 0,
                left: 0,
            },
            gaps_mask: GapMask::empty(),
            ws_assignments: WorkspaceAssignmentLinks {
                tqe_next: std::ptr::null_mut(),
                tqe_prev: std::ptr::null_mut(),
            },
        }
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WorkspaceAssignmentLinks {
    pub tqe_next: *mut Workspace_Assignment,
    pub tqe_prev: *mut *mut Workspace_Assignment,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ignore_event_sequence_t {
    pub value: u32,
}
pub struct Autostart {
    pub command: Vec<u8>,
    pub no_startup_id: bool,
    pub autostarts: AutostartLinks,
    pub autostarts_always: AutostartAlwaysLinks,
}
impl Autostart {
    pub fn new(command: Vec<u8>, no_startup_id: bool) -> Self {
        Self {
            command: command.split(|byte| *byte == 0).next().unwrap().to_vec(),
            no_startup_id,
            autostarts: AutostartLinks {
                tqe_next: std::ptr::null_mut(),
                tqe_prev: std::ptr::null_mut(),
            },
            autostarts_always: AutostartAlwaysLinks {
                tqe_next: std::ptr::null_mut(),
                tqe_prev: std::ptr::null_mut(),
            },
        }
    }
}

/// Detach a pending command before logging or launching it, retaining its owner.
///
/// # Safety
/// The event thread exclusively owns both initialized queues. Every linked node
/// is a unique Box in exactly one queue, and no borrowed node view is in use.
#[deny(unsafe_op_in_unsafe_fn)]
pub unsafe fn take_first_autostart(always: bool) -> Option<Box<Autostart>> {
    unsafe {
        let node = if always {
            autostarts_always.tqh_first
        } else {
            autostarts.tqh_first
        };
        if node.is_null() {
            return None;
        }
        if always {
            let links = (*node).autostarts_always;
            if links.tqe_next.is_null() {
                autostarts_always.tqh_last = links.tqe_prev;
            } else {
                (*links.tqe_next).autostarts_always.tqe_prev = links.tqe_prev;
            }
            *links.tqe_prev = links.tqe_next;
            (*node).autostarts_always.tqe_next = std::ptr::null_mut();
            (*node).autostarts_always.tqe_prev = std::ptr::null_mut();
        } else {
            let links = (*node).autostarts;
            if links.tqe_next.is_null() {
                autostarts.tqh_last = links.tqe_prev;
            } else {
                (*links.tqe_next).autostarts.tqe_prev = links.tqe_prev;
            }
            *links.tqe_prev = links.tqe_next;
            (*node).autostarts.tqe_next = std::ptr::null_mut();
            (*node).autostarts.tqe_prev = std::ptr::null_mut();
        }
        Some(Box::from_raw(node))
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AutostartAlwaysLinks {
    pub tqe_next: *mut Autostart,
    pub tqe_prev: *mut *mut Autostart,
}
pub use crate::application::AutostartAlwaysLinks as AutostartLinks;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct bindings_head {
    pub tqh_first: *mut Binding,
    pub tqh_last: *mut *mut Binding,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct autostarts_head {
    pub tqh_first: *mut Autostart,
    pub tqh_last: *mut *mut Autostart,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct autostarts_always_head {
    pub tqh_first: *mut Autostart,
    pub tqh_last: *mut *mut Autostart,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ws_assignments_head {
    pub tqh_first: *mut Workspace_Assignment,
    pub tqh_last: *mut *mut Workspace_Assignment,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct assignments_head {
    pub tqh_first: *mut Assignment,
    pub tqh_last: *mut *mut Assignment,
}
pub type tiling_drag_t = TilingDrag;

pub struct Barconfig {
    pub id: Option<Vec<u8>>,
    pub outputs: Vec<Vec<u8>>,
    pub tray_outputs: tray_outputs_head,
    pub tray_padding: i32,
    pub socket_path: Option<std::path::PathBuf>,
    pub mode: BarMode,
    pub hidden_state: BarHiddenState,
    pub modifier: u32,
    pub bar_bindings: bar_bindings_head,
    pub position: BarPosition,
    pub i3bar_command: Option<Vec<u8>>,
    pub status_command: Option<Vec<u8>>,
    pub workspace_command: Option<Vec<u8>>,
    pub font: Option<Vec<u8>>,
    pub separator_symbol: Option<Vec<u8>>,
    pub hide_workspace_buttons: bool,
    pub workspace_min_width: i32,
    pub strip_workspace_numbers: bool,
    pub strip_workspace_name: bool,
    pub hide_binding_mode_indicator: bool,
    pub verbose: bool,
    pub bar_height: u32,
    pub padding: Rect,
    pub colors: bar_colors,
    pub configs: BarConfigLinks,
}
impl Barconfig {
    pub fn id(&self) -> &[u8] {
        self.id
            .as_deref()
            .expect("finished bar configuration has an ID")
    }
}
impl Default for Barconfig {
    fn default() -> Self {
        Self {
            id: None,
            outputs: Vec::new(),
            tray_outputs: tray_outputs_head {
                tqh_first: std::ptr::null_mut(),
                tqh_last: std::ptr::null_mut(),
            },
            tray_padding: 0,
            socket_path: None,
            mode: BarMode::Dock,
            hidden_state: BarHiddenState::Hide,
            modifier: 0,
            bar_bindings: bar_bindings_head {
                tqh_first: std::ptr::null_mut(),
                tqh_last: std::ptr::null_mut(),
            },
            position: BarPosition::Bottom,
            i3bar_command: None,
            status_command: None,
            workspace_command: None,
            font: None,
            separator_symbol: None,
            hide_workspace_buttons: false,
            workspace_min_width: 0,
            strip_workspace_numbers: false,
            strip_workspace_name: false,
            hide_binding_mode_indicator: false,
            verbose: false,
            bar_height: 0,
            padding: Rect {
                x: 0,
                y: 0,
                width: 0,
                height: 0,
            },
            colors: bar_colors::default(),
            configs: BarConfigLinks {
                tqe_next: std::ptr::null_mut(),
                tqe_prev: std::ptr::null_mut(),
            },
        }
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BarConfigLinks {
    pub tqe_next: *mut Barconfig,
    pub tqe_prev: *mut *mut Barconfig,
}
#[derive(Default)]
pub struct bar_colors {
    pub background: Option<Vec<u8>>,
    pub statusline: Option<Vec<u8>>,
    pub separator: Option<Vec<u8>>,
    pub focused_background: Option<Vec<u8>>,
    pub focused_statusline: Option<Vec<u8>>,
    pub focused_separator: Option<Vec<u8>>,
    pub focused_workspace_border: Option<Vec<u8>>,
    pub focused_workspace_bg: Option<Vec<u8>>,
    pub focused_workspace_text: Option<Vec<u8>>,
    pub active_workspace_border: Option<Vec<u8>>,
    pub active_workspace_bg: Option<Vec<u8>>,
    pub active_workspace_text: Option<Vec<u8>>,
    pub inactive_workspace_border: Option<Vec<u8>>,
    pub inactive_workspace_bg: Option<Vec<u8>>,
    pub inactive_workspace_text: Option<Vec<u8>>,
    pub urgent_workspace_border: Option<Vec<u8>>,
    pub urgent_workspace_bg: Option<Vec<u8>>,
    pub urgent_workspace_text: Option<Vec<u8>>,
    pub binding_mode_border: Option<Vec<u8>>,
    pub binding_mode_bg: Option<Vec<u8>>,
    pub binding_mode_text: Option<Vec<u8>>,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct bar_bindings_head {
    pub tqh_first: *mut Barbinding,
    pub tqh_last: *mut *mut Barbinding,
}
pub struct Barbinding {
    pub input_code: i32,
    pub command: Vec<u8>,
    pub release: bool,
    pub bindings: BarBindingLinks,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BarBindingLinks {
    pub tqe_next: *mut Barbinding,
    pub tqe_prev: *mut *mut Barbinding,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct tray_outputs_head {
    pub tqh_first: *mut tray_output_t,
    pub tqh_last: *mut *mut tray_output_t,
}
pub struct tray_output_t {
    pub output: Vec<u8>,
    pub tray_outputs: TrayOutputLinks,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TrayOutputLinks {
    pub tqe_next: *mut tray_output_t,
    pub tqe_prev: *mut *mut tray_output_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct barconfig_head {
    pub tqh_first: *mut Barconfig,
    pub tqh_last: *mut *mut Barconfig,
}

// Rust-owned IPC storage. Only ipc.rs constructs/destroys these Box allocations;
// other modules carry opaque pointers. Stable watcher/list addresses stay native.
pub struct ipc_client {
    pub fd: i32,
    pub generation: u64,
    pub fd_owner: Option<std::os::unix::net::UnixStream>,
    pub events: Vec<std::ffi::CString>,
    pub first_tick_sent: bool,
    pub read_callback: *mut ev_io,
    pub write_callback: *mut ev_io,
    pub timeout: *mut ev_timer,
    pub buffer: Vec<u8>,
    pub clients: IpcClientLinks,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IpcClientLinks {
    pub tqe_next: *mut ipc_client,
    pub tqe_prev: *mut *mut ipc_client,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct outputs_head {
    pub tqh_first: *mut xoutput,
    pub tqh_last: *mut *mut xoutput,
}

pub const I3_VERSION: &[u8] = concat!(env!("I3_VERSION"), "\0").as_bytes();

pub const SIGILL: i32 = 4_i32;
pub const SIGABRT: i32 = 6_i32;
pub const SIGFPE: i32 = 8_i32;
pub const SIGSEGV: i32 = 11_i32;
pub const SIGQUIT: i32 = 3_i32;
pub const SIGPIPE: i32 = 13_i32;
pub const SIGCHLD: i32 = 17_i32;
pub const SA_SIGINFO: i32 = 4_i32;
pub const SA_NODEFER: i32 = 0x40000000_i32;
pub const SA_RESETHAND: u32 = 0x80000000_u32;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();

pub const RLIM_INFINITY: u64 = 0xffffffffffffffff_u64;
pub const I3_IPC_MESSAGE_TYPE_RUN_COMMAND: i32 = 0_i32;
pub const I3_IPC_REPLY_TYPE_COMMAND: i32 = 0_i32;

pub const O_RDONLY: i32 = 0_i32;
pub const F_GETFD: i32 = 1_i32;
pub const F_SETFD: i32 = 2_i32;
pub const FD_CLOEXEC: i32 = 1_i32;
pub const __LC_ALL: i32 = 6_i32;
pub const LC_ALL: i32 = __LC_ALL;
pub const PF_LOCAL: i32 = 1_i32;
pub const AF_LOCAL: i32 = PF_LOCAL;
pub const SD_LISTEN_FDS_START: i32 = 3_i32;

pub static mut original_rlimit_core: rlimit = rlimit {
    rlim_cur: 0,
    rlim_max: 0,
};

pub static mut listen_fds: i32 = 0;
static mut xcb_prepare: *mut ev_prepare = std::ptr::null_mut::<ev_prepare>();

pub static mut start_argv: *mut *mut ::core::ffi::c_char =
    std::ptr::null_mut::<*mut ::core::ffi::c_char>();

pub static mut conn: *mut xcb_connection_t = std::ptr::null_mut::<xcb_connection_t>();
// The registration owns the one maintained connection on the event thread.
// `conn` is only a compatibility/native-library alias and never disconnects it.
static mut MAIN_DISPLAY: Option<i3_common::display::DisplayConnection> = None;
static mut MAIN_RENDERER: Option<Rc<i3_common::draw_util::RenderingContext>> = None;
/// The event thread clones explicit rendering state only on the normal path.
pub unsafe fn rendering_context() -> Rc<i3_common::draw_util::RenderingContext> {
    MAIN_RENDERER
        .as_ref()
        .expect("main rendering context is initialized")
        .clone()
}
static mut MAIN_SCREEN: Option<DisplayScreen> = None;
static mut MAIN_VISUAL: Option<CairoVisual> = None;

/// The event thread retains this setup owner until all graphics are released.
/// Safety: no reference may outlive or trigger display teardown.
pub unsafe fn root_display() -> &'static DisplayScreen {
    MAIN_SCREEN
        .as_ref()
        .expect("main setup screen is initialized")
}

/// Borrow the initialized screen, or remain detached during startup validation.
///
/// # Safety
/// Call only on the main event thread; no returned reference may outlive or
/// trigger teardown of the selected display owner.
pub unsafe fn root_display_opt() -> Option<&'static DisplayScreen> {
    MAIN_SCREEN.as_ref()
}

/// Copy immutable setup values without registry borrows or connection cloning.
pub unsafe fn root_screen_info() -> ScreenInfo {
    root_display().info()
}

/// Called only by the common event-thread context provider.
pub unsafe fn display_context() -> i3_common::display::DisplayContext {
    MAIN_SCREEN
        .as_ref()
        .map_or_else(i3_common::display::DisplayContext::default, |screen| {
            i3_common::display::DisplayContext::from_screen(screen, MAIN_VISUAL.as_ref())
        })
}

fn main_connection() -> Rc<xcb::Connection> {
    i3_common::display::connection().expect("main X11 connection is registered")
}

fn manager_notification(
    root_window: u32,
    atom: u32,
    time: u32,
    selection: u32,
    owner: u32,
) -> x::ClientMessageEvent {
    x::ClientMessageEvent::new(
        x::Window::new(root_window),
        x::Atom::new(atom),
        x::ClientMessageData::Data32([time, selection, owner, 0, 0]),
    )
}

/// Read only the response byte, valid for queue events and 32-byte factories.
fn event_response_type(event: &xcb::Event) -> u8 {
    // SAFETY: every maintained event has at least its initialized response byte.
    unsafe { event.as_raw().cast::<u8>().read() }
}

fn event_kind(event: &xcb::Event) -> i32 {
    i32::from(event_response_type(event) & 0x7f)
}

fn error_header(error: &xcb::ProtocolError) -> &xcb::ffi::xcb_generic_error_t {
    // SAFETY: every maintained protocol error owns a complete native allocation.
    unsafe { &*error.as_raw() }
}

/// Log asynchronous errors with the same full-sequence ignore-cache policy.
unsafe fn log_x11_error(error: &xcb::ProtocolError) {
    let error = error_header(error);
    if event_is_ignored(
        ignore_event_sequence_t {
            value: error.full_sequence,
        },
        0,
    ) {
        ::log::debug!(
            "../src/main.c:xcb_prepare_cb:137 - Expected X11 Error received for sequence {:x}\n",
            (i32::from(error.sequence)) as u32
        );
    } else {
        ::log::debug!("../src/main.c:xcb_prepare_cb:141 - X11 Error received (probably harmless)! sequence 0x{:x}, error_code = {}\n", (i32::from(error.sequence)) as u32, { (i32::from(error.error_code)) });
    }
}

pub static mut conn_screen: i32 = 0;

pub static mut sndisplay: *mut SnDisplay = std::ptr::null_mut::<SnDisplay>();

pub static mut last_timestamp: u32 = (::xcb::x::CURRENT_TIME);

pub static mut root: u32 = 0;

pub static mut wm_sn_selection_owner: u32 = 0;

pub static mut wm_sn: u32 = 0;

pub static mut root_depth: u8 = 0;

pub static mut colormap: u32 = 0;

pub static mut main_loop: *mut ev_loop = std::ptr::null_mut::<ev_loop>();

// Native cache ownership remains on the main event thread and outlives TLS.
// Release it explicitly before the main display registration in native atexit.
static mut KEY_SYMBOLS: Option<xcb_interop::keysyms::KeySymbols> = None;

/// Rebuild on the event thread; no key-symbol borrow may span this operation.
pub unsafe fn rebuild_key_symbols() {
    drop(KEY_SYMBOLS.take());
    KEY_SYMBOLS = xcb_interop::keysyms::KeySymbols::new(main_connection());
}
/// Refresh only during synchronous MappingNotify dispatch on the event thread.
pub unsafe fn refresh_key_symbols(event: &xcb::x::MappingNotifyEvent) {
    if let Some(symbols) = KEY_SYMBOLS.as_mut() {
        symbols.refresh(event);
    }
}
/// Query without exposing a cache borrow to graph-mutating callbacks.
pub unsafe fn key_symbol_modifiers(symbol: u32) -> u32 {
    KEY_SYMBOLS.as_ref().map_or(0, |symbols| {
        i3_common::get_mod_mask::query_modifier_mask(symbol, symbols)
    })
}
/// Copy a lookup result while the event thread retains its cache owner.
pub unsafe fn key_symbol(keycode: u8, column: i32) -> u32 {
    KEY_SYMBOLS
        .as_ref()
        .map_or(0, |symbols| symbols.keysym(keycode, column))
}

pub static mut default_shmlog_size: i32 = 25_i32 * 1024_i32 * 1024_i32;

pub static mut bindings: *mut bindings_head = std::ptr::null_mut::<bindings_head>();

pub static mut current_binding_mode: Option<Vec<u8>> = None;

pub static mut autostarts: autostarts_head = autostarts_head {
    tqh_first: std::ptr::null_mut::<Autostart>(),
    tqh_last: std::ptr::null_mut::<*mut Autostart>(),
};

pub static mut autostarts_always: autostarts_always_head = autostarts_always_head {
    tqh_first: std::ptr::null_mut::<Autostart>(),
    tqh_last: std::ptr::null_mut::<*mut Autostart>(),
};

pub static mut assignments: assignments_head = assignments_head {
    tqh_first: std::ptr::null_mut::<Assignment>(),
    tqh_last: std::ptr::null_mut::<*mut Assignment>(),
};

pub static mut ws_assignments: ws_assignments_head = ws_assignments_head {
    tqh_first: std::ptr::null_mut::<Workspace_Assignment>(),
    tqh_last: std::ptr::null_mut::<*mut Workspace_Assignment>(),
};

pub static mut xkb_supported: bool = true;

pub static mut shape_supported: bool = true;

pub static mut force_xinerama: bool = false;

pub static mut A__NET_WM_STATE_MAXIMIZED_HORZ: u32 = 0;

pub static mut A__NET_WM_STATE_FOCUSED: u32 = 0;

pub static mut A__NET_WM_STATE_MAXIMIZED_VERT: u32 = 0;

pub static mut A__NET_WM_STATE: u32 = 0;

pub static mut A__NET_WM_WINDOW_TYPE: u32 = 0;

pub static mut A__NET_WM_WINDOW_TYPE_NORMAL: u32 = 0;

pub static mut A__NET_WM_WINDOW_TYPE_DOCK: u32 = 0;

pub static mut A__NET_WM_WINDOW_TYPE_DIALOG: u32 = 0;

pub static mut A__NET_WM_WINDOW_TYPE_UTILITY: u32 = 0;

pub static mut A__NET_WM_WINDOW_TYPE_TOOLBAR: u32 = 0;

pub static mut A__NET_WM_WINDOW_TYPE_SPLASH: u32 = 0;

pub static mut A__NET_WM_WINDOW_TYPE_MENU: u32 = 0;

pub static mut A__NET_WM_WINDOW_TYPE_DROPDOWN_MENU: u32 = 0;

pub static mut A__NET_WM_WINDOW_TYPE_POPUP_MENU: u32 = 0;

pub static mut A__NET_WM_WINDOW_TYPE_TOOLTIP: u32 = 0;

pub static mut A__NET_WM_WINDOW_TYPE_NOTIFICATION: u32 = 0;

pub static mut A__NET_WM_STATE_HIDDEN: u32 = 0;

pub static mut A__NET_WM_STRUT_PARTIAL: u32 = 0;

pub static mut A__NET_CLIENT_LIST: u32 = 0;

pub static mut A__NET_CLIENT_LIST_STACKING: u32 = 0;

pub static mut A__NET_CURRENT_DESKTOP: u32 = 0;

pub static mut A__NET_NUMBER_OF_DESKTOPS: u32 = 0;

pub static mut A__NET_DESKTOP_NAMES: u32 = 0;

pub static mut A__NET_DESKTOP_VIEWPORT: u32 = 0;

pub static mut A__NET_ACTIVE_WINDOW: u32 = 0;

pub static mut A__NET_CLOSE_WINDOW: u32 = 0;

pub static mut A__NET_MOVERESIZE_WINDOW: u32 = 0;

pub static mut A__NET_WM_STATE_STICKY: u32 = 0;

pub static mut A__NET_SUPPORTED: u32 = 0;

pub static mut A__NET_SUPPORTING_WM_CHECK: u32 = 0;

pub static mut A__NET_WM_NAME: u32 = 0;

pub static mut A__NET_WM_VISIBLE_NAME: u32 = 0;

pub static mut A__NET_WM_STATE_FULLSCREEN: u32 = 0;

pub static mut A__NET_WM_MOVERESIZE: u32 = 0;

pub static mut A__NET_WM_STATE_DEMANDS_ATTENTION: u32 = 0;

pub static mut A__NET_WM_STATE_MODAL: u32 = 0;

pub static mut A__NET_WM_DESKTOP: u32 = 0;

pub static mut A_WM_TAKE_FOCUS: u32 = 0;

pub static mut A_I3_CONFIG_PATH: u32 = 0;

pub static mut A__NET_WM_USER_TIME: u32 = 0;

pub static mut A__NET_STARTUP_ID: u32 = 0;

pub static mut A__NET_WORKAREA: u32 = 0;

pub static mut A__NET_WM_ICON: u32 = 0;

pub static mut A_WM_PROTOCOLS: u32 = 0;

pub static mut A_WM_DELETE_WINDOW: u32 = 0;

pub static mut A_UTF8_STRING: u32 = 0;

pub static mut A_WM_STATE: u32 = 0;

pub static mut A_WM_CLIENT_LEADER: u32 = 0;

pub static mut A_WM_WINDOW_ROLE: u32 = 0;

pub static mut A_I3_SOCKET_PATH: u32 = 0;

pub static mut A_I3_SYNC: u32 = 0;

pub static mut A__NET_FRAME_EXTENTS: u32 = 0;

pub static mut A_I3_FLOATING_WINDOW: u32 = 0;

pub static mut A_I3_LOG_STREAM_SOCKET_PATH: u32 = 0;

pub static mut A_I3_SHMLOG_PATH: u32 = 0;

pub static mut A_I3_PID: u32 = 0;

pub static mut A__NET_REQUEST_FRAME_EXTENTS: u32 = 0;

pub static mut A__MOTIF_WM_HINTS: u32 = 0;

pub static mut A_WM_CHANGE_STATE: u32 = 0;

pub static mut A_MANAGER: u32 = 0;
// Native malloc ownership used only by normal startup/event callbacks, never
// added to the actual signal handler. Global display resources retain atexit ABI.
struct MainWatchers {
    io: Vec<NonNull<ev_io>>,
}
impl MainWatchers {
    fn new() -> Self {
        Self { io: Vec::new() }
    }
    unsafe fn add(
        &mut self,
        fd: i32,
        callback: unsafe extern "C" fn(*mut ev_loop, *mut ev_io, i32),
    ) -> *mut ev_io {
        let raw = Box::into_raw(Box::new(ev_io::new(fd, EV_READ, callback)));
        self.io.push(NonNull::new(raw).unwrap());
        ev_io_start(main_loop, raw);
        raw
    }
}
impl Drop for MainWatchers {
    fn drop(&mut self) {
        unsafe {
            for watcher in self.io.drain(..) {
                libev_sys::ev_io_stop(main_loop, watcher.as_ptr());
                drop(Box::from_raw(watcher.as_ptr()));
            }
        }
    }
}
fn bar_command(program: &[u8], verbose: bool, id: &[u8], socket: &[u8]) -> Vec<u8> {
    let mut bytes = program.to_vec();
    bytes.push(b' ');
    if verbose {
        bytes.extend_from_slice(b"-V");
    }
    bytes.extend_from_slice(b" --bar_id=");
    bytes.extend_from_slice(id);
    bytes.extend_from_slice(b" --socket=\"");
    bytes.extend_from_slice(socket);
    bytes.push(b'"');
    bytes
}
fn owned_dirname(path: &CStr) -> CString {
    let mut bytes = path.to_bytes_with_nul().to_vec();
    unsafe { CStr::from_ptr(dirname(bytes.as_mut_ptr().cast())) }.to_owned()
}
unsafe extern "C" fn xcb_got_event(loop_0: *mut ev_loop, w: *mut ev_io, revents: i32) {}
unsafe extern "C" fn xcb_prepare_cb(loop_0: *mut ev_loop, w: *mut ev_prepare, revents: i32) {
    loop {
        // Release the registry clone before dispatch: callbacks can reenter
        // display helpers without retaining a RefCell borrow or a second owner.
        let next = main_connection().poll_for_event();
        match next {
            Ok(Some(event)) => {
                // Dispatch borrows the maintained owner; neither handlers nor
                // native startup-notification may free or retain its allocation.
                handle_event(&event);
            }
            Err(xcb::Error::Protocol(error)) => log_x11_error(&error),
            Ok(None) | Err(xcb::Error::Connection(_)) => break,
        }
    }
    let _ = main_connection().flush();
}

pub unsafe fn main_set_x11_cb(enable: bool) {
    ::log::debug!(
        "../src/main.c:main_set_x11_cb:165 - Setting main X11 callback to enabled={}\n",
        { (enable as i32) }
    );
    if enable {
        ev_prepare_start(main_loop, xcb_prepare);
        ev_feed_event(main_loop, xcb_prepare as *mut ::core::ffi::c_void, 0_i32);
    } else {
        ev_prepare_stop(main_loop, xcb_prepare);
    };
}
extern "C" fn i3_exit() {
    unsafe {
        if *shmlogname as i32 != '\0' as i32 {
            fprintf(stderr, c"Closing SHM log \"%s\"\n".as_ptr(), shmlogname);
            fflush(stderr);
            shm_unlink(shmlogname);
        }
        ipc_shutdown(ShutdownReason::Exit, -1_i32);
        if let Some(path) = config.ipc_socket_path.as_deref() {
            let _ = std::fs::remove_file(path);
        }
        if let Some(path) = current_log_stream_socket_path.as_deref() {
            let _ = std::fs::remove_file(path);
        }
        crate::restore_layout::restore_shutdown();
        drop(KEY_SYMBOLS.take());
        crate::sighandler::shutdown_crash_visual();
        drop(MAIN_RENDERER.take());
        drop(MAIN_VISUAL.take());
        drop(MAIN_SCREEN.take());
        // All native shutdown requests above complete before unregistering the
        // sole owner. The raw alias must never be disconnected independently.
        conn = std::ptr::null_mut();
        // Static owning fonts do not receive Rust Drop automatically at exit.
        // Release the backend while its display/visual guards still live.
        crate::config::config.font = i3_common::font::Font::empty();
        drop(MAIN_DISPLAY.take());
        kill_nagbar(config_error_nagbar_pid, false);
        kill_nagbar(command_error_nagbar_pid, false);
        ev_loop_destroy(main_loop);
    }
}
unsafe extern "C" fn handle_core_signal(
    sig: i32,
    info: *mut siginfo_t,
    data: *mut ::core::ffi::c_void,
) {
    if *shmlogname as i32 != '\0' as i32 {
        shm_unlink(shmlogname);
    }
    raise(sig);
}
unsafe extern "C" fn handle_term_signal(
    loop_0: *mut ev_loop,
    signal_0: *mut ev_signal,
    revents: i32,
) {
    exit(128_i32 + (*signal_0).signum);
}
unsafe fn setup_term_handlers() {
    static mut WATCHERS: [std::mem::MaybeUninit<ev_signal>; 6] =
        [std::mem::MaybeUninit::uninit(); 6];
    for (index, signum) in [1, 2, 14, 15, 10, 12].into_iter().enumerate() {
        let watcher = (&raw mut WATCHERS).cast::<ev_signal>().add(index);
        watcher.write(ev_signal::new(signum, handle_term_signal));
        ev_signal_start(main_loop, watcher);
        ev_unref(main_loop);
    }
}
unsafe fn parse_restart_fd() -> i32 {
    let restart_fd: *const ::core::ffi::c_char = getenv(c"_I3_RESTART_FD".as_ptr());
    if restart_fd.is_null() {
        return -1_i32;
    }
    let Ok(fd) = parse_long(std::ffi::CStr::from_ptr(restart_fd).to_bytes(), 10) else {
        {
            // Snapshot native bytes before the logger can dispatch a callback.
            let log_pointer_1 = restart_fd;
            let log_value_0 = i3_common::logging::string_bytes(
                (if log_pointer_1.is_null() {
                    None
                } else {
                    Some(std::ffi::CStr::from_ptr(log_pointer_1))
                })
                .map(std::ffi::CStr::to_bytes),
                None,
            )
            .to_vec();
            if let Ok(log_value_0_text) = std::str::from_utf8(&log_value_0) {
                ::log::error!("ERROR: Malformed _I3_RESTART_FD \"{}\"\n", log_value_0_text);
            } else {
                i3_common::logging::emit_parts(
                    i3_common::logging::Level::Error,
                    &[
                        "ERROR: Malformed _I3_RESTART_FD \"".as_bytes(),
                        &log_value_0,
                        "\"\n".as_bytes(),
                    ],
                );
            }
        };
        return -1_i32;
    };
    fd as i32
}
/// Only the legacy restart/exec boundary receives a C-compatible argv array.
/// Argument bytes remain owned by Rust for the complete event-loop lifetime.
struct RestartArguments {
    strings: Vec<std::ffi::CString>,
    pointers: *mut *mut libc::c_char,
    installed: bool,
}
impl RestartArguments {
    fn new(arguments: &[std::ffi::OsString]) -> std::io::Result<Self> {
        use std::os::unix::ffi::OsStrExt;
        let strings = arguments
            .iter()
            .map(|argument| {
                std::ffi::CString::new(argument.as_bytes()).expect("OS arguments have no NUL bytes")
            })
            .collect::<Vec<_>>();
        // SAFETY: calloc creates a separate C allocation, including a zeroed
        // sentinel slot. No Rust Vec allocation is ever passed to native realloc.
        let pointers =
            unsafe { libc::calloc(strings.len() + 1, std::mem::size_of::<*mut libc::c_char>()) }
                .cast::<*mut libc::c_char>();
        if pointers.is_null() {
            return Err(std::io::Error::last_os_error());
        }
        for (index, argument) in strings.iter().enumerate() {
            unsafe {
                pointers.add(index).write(argument.as_ptr().cast_mut());
            }
        }
        Ok(Self {
            strings,
            pointers,
            installed: false,
        })
    }
    unsafe fn install(&mut self) {
        start_argv = self.pointers;
        self.installed = true;
    }
}
impl Drop for RestartArguments {
    fn drop(&mut self) {
        // SAFETY: Before installation we own the C pointer allocation. Once
        // installed, restart may replace/reallocate it; never free a stale
        // address in that case. CString buffers live until this owner drops.
        unsafe {
            if !self.installed || start_argv == self.pointers {
                libc::free(self.pointers.cast());
            }
            if self.installed {
                start_argv = std::ptr::null_mut();
            }
        }
    }
}

const OPT_RESTART: u8 = 128;
const OPT_XINERAMA: u8 = 129;
const OPT_RANDR15: u8 = 130;
const OPT_SIGNAL: u8 = 131;
const OPT_SHMLOG: u8 = 132;
const OPT_SOCKET: u8 = 133;
const OPT_FAKE_OUTPUTS: u8 = 134;
const OPT_OLD_PARSER: u8 = 135;
const CORE_OPTIONS: &[i3_common::cli::OptionSpec] = &[
    i3_common::cli::OptionSpec::flag(b'a', "no-autostart"),
    i3_common::cli::OptionSpec::value(b'c', "config"),
    i3_common::cli::OptionSpec::flag(b'v', "version"),
    i3_common::cli::OptionSpec::flag(b'm', "moreversion"),
    i3_common::cli::OptionSpec::flag(b'm', "more-version"),
    i3_common::cli::OptionSpec::flag(b'm', "more_version"),
    i3_common::cli::OptionSpec::flag(b'h', "help"),
    i3_common::cli::OptionSpec::value(b'L', "layout"),
    i3_common::cli::OptionSpec::value(OPT_RESTART, "restart")
        .long_only()
        .equivalent_return(0),
    i3_common::cli::OptionSpec::flag(OPT_XINERAMA, "force-xinerama")
        .long_only()
        .equivalent_return(0),
    i3_common::cli::OptionSpec::flag(OPT_XINERAMA, "force_xinerama")
        .long_only()
        .equivalent_return(0),
    i3_common::cli::OptionSpec::flag(OPT_RANDR15, "disable-randr15")
        .long_only()
        .equivalent_return(0),
    i3_common::cli::OptionSpec::flag(OPT_RANDR15, "disable_randr15")
        .long_only()
        .equivalent_return(0),
    i3_common::cli::OptionSpec::flag(OPT_SIGNAL, "disable-signalhandler")
        .long_only()
        .equivalent_return(0),
    i3_common::cli::OptionSpec::value(OPT_SHMLOG, "shmlog-size")
        .long_only()
        .equivalent_return(0),
    i3_common::cli::OptionSpec::value(OPT_SHMLOG, "shmlog_size")
        .long_only()
        .equivalent_return(0),
    i3_common::cli::OptionSpec::flag(OPT_SOCKET, "get-socketpath")
        .long_only()
        .equivalent_return(0),
    i3_common::cli::OptionSpec::flag(OPT_SOCKET, "get_socketpath")
        .long_only()
        .equivalent_return(0),
    i3_common::cli::OptionSpec::value(OPT_FAKE_OUTPUTS, "fake_outputs")
        .long_only()
        .equivalent_return(0),
    i3_common::cli::OptionSpec::value(OPT_FAKE_OUTPUTS, "fake-outputs")
        .long_only()
        .equivalent_return(0),
    i3_common::cli::OptionSpec::flag(OPT_OLD_PARSER, "force-old-config-parser-v4.4-only")
        .long_only()
        .equivalent_return(0),
    i3_common::cli::OptionSpec::flag(b'r', "replace"),
    i3_common::cli::OptionSpec::flag(b'C', "check").short_only(),
    i3_common::cli::OptionSpec::flag(b'V', "verbose").short_only(),
    i3_common::cli::OptionSpec::flag(b'l', "legacy").short_only(),
    i3_common::cli::OptionSpec::value(b'd', "debug").short_only(),
];

fn print_usage(program: &std::ffi::OsStr, shmlog_limit: i32) {
    use std::io::Write;
    use std::os::unix::ffi::OsStrExt;
    let mut output = std::io::stderr().lock();
    let _ = output.write_all(b"Usage: ");
    let _ = output.write_all(program.as_bytes());
    let _ = write!(output, " [-c configfile] [-d all] [-a] [-v] [-V] [-C]\n\n\t-a          disable autostart ('exec' lines in config)\n\t-c <file>   use the provided configfile instead\n\t-C          validate configuration file and exit\n\t-d all      enable debug output\n\t-L <file>   path to the serialized layout during restarts\n\t-v          display version and exit\n\t-V          enable verbose mode\n\n\t--force-xinerama\n\tUse Xinerama instead of RandR.\n\tThis option should only be used if you are stuck with the\n\told nVidia closed source driver (older than 302.17), which does\n\tnot support RandR.\n\n\t--get-socketpath\n\tRetrieve the i3 IPC socket path from X11, print it, then exit.\n\n\t--shmlog-size <limit>\n\tLimits the size of the i3 SHM log to <limit> bytes. Setting this\n\tto 0 disables SHM logging entirely.\n\tThe default is {shmlog_limit} bytes.\n\n\t--replace\n\tReplace an existing window manager.\n\nIf you pass plain text arguments, i3 will interpret them as a command\nto send to a currently running i3 (like i3-msg). This allows you to\nuse nice and logical commands, such as:\n\n\ti3 border none\n\ti3 floating toggle\n\ti3 kill window\n\n");
}
unsafe fn run(mut arguments: Vec<std::ffi::OsString>) -> i32 {
    use std::os::unix::ffi::OsStrExt;
    if arguments.is_empty() {
        arguments.push("i3".into());
    }
    let program = arguments[0].as_os_str();
    let mut _restart_arguments = match RestartArguments::new(&arguments) {
        Ok(owner) => owner,
        Err(error) => {
            eprintln!("i3: Could not allocate restart arguments: {error}");
            return EXIT_FAILURE;
        }
    };
    #[used]
    static mut _i3_version: *const ::core::ffi::c_char =
        I3_VERSION.as_ptr().cast::<::core::ffi::c_char>();
    let mut layout_owner: Option<CString> = None;
    let mut fake_owner: Option<Vec<u8>> = None;
    let mut override_configpath: Option<std::path::PathBuf> = None;
    let mut autostart: bool = true;
    let mut layout_path: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut delete_layout_path: bool = false;
    let mut disable_randr15: bool = false;
    let mut disable_signalhandler: bool = false;
    let mut only_check_config: bool = false;
    let mut replace_wm: bool = false;
    setlocale(LC_ALL, c"".as_ptr());
    getrlimit(libc::RLIMIT_CORE, &raw mut original_rlimit_core);
    if isatty(fileno(stdout)) == 0 {
        setbuf(stdout, ::core::ptr::null_mut::<::core::ffi::c_char>());
    }
    srand(time(::core::ptr::null_mut::<libc::time_t>()) as u32);
    init_logging();
    shmlog_size = if is_debug_build()
        || !strstr(
            _restart_arguments.strings[0].as_ptr(),
            c"i3-with-shmlog".as_ptr(),
        )
        .is_null()
    {
        default_shmlog_size
    } else {
        0_i32
    };
    _restart_arguments.install();
    let mut operands = Vec::new();
    for item in i3_common::cli::Parser::new(
        arguments.iter().skip(1).cloned(),
        CORE_OPTIONS,
        std::env::var_os("POSIXLY_CORRECT").is_some(),
    ) {
        use i3_common::cli::Item;
        let (flag, value) = match item {
            Item::Operand(value) => {
                operands.push(value);
                continue;
            }
            Item::Error(message) => {
                eprintln!("{}: {message}", program.to_string_lossy());
                print_usage(program, shmlog_size);
                return EXIT_FAILURE;
            }
            Item::Option { short, value } => (short, value),
        };
        let value = value.map(|value| {
            std::ffi::CString::new(value.as_bytes()).expect("OS arguments have no NUL bytes")
        });
        match flag {
            b'a' => { ::log::info!("Autostart disabled using -a\n"); autostart = false; },
            b'L' | OPT_RESTART => {
                layout_owner=value.clone();
                layout_path=layout_owner.as_ref().unwrap().as_ptr().cast_mut();
                delete_layout_path = flag == OPT_RESTART;
            },
            b'c' => { override_configpath = Some(std::path::PathBuf::from(std::ffi::OsStr::from_bytes(value.as_ref().unwrap().to_bytes()))); },
            b'C' => { ::log::info!("Checking configuration file only (-C)\n"); only_check_config = true; },
            b'v' => { printf(c"i3 version %s © 2009 Michael Stapelberg and contributors\n".as_ptr(), i3_version); return EXIT_SUCCESS; },
            b'm' => { printf(c"Binary i3 version:  %s © 2009 Michael Stapelberg and contributors\n".as_ptr(), i3_version); return display_running_version(); },
            b'V' => set_verbosity(true),
            b'd' => { ::log::info!("Enabling debug logging\n"); set_debug_logging(true); },
            b'l' => {},
            b'r' => replace_wm = true,
            b'h' => { print_usage(program, shmlog_size); return EXIT_SUCCESS; },
            OPT_XINERAMA => {
                force_xinerama = true;
                ::log::error!("ERROR: Using Xinerama instead of RandR. This option should be avoided at all cost because it does not refresh the list of screens, so you cannot configure displays at runtime. Please check if your driver really does not support RandR and disable this option as soon as you can.\n");
            },
            OPT_RANDR15 => disable_randr15 = true,
            OPT_SIGNAL => disable_signalhandler = true,
            OPT_SOCKET => {
                let Some(mut socket) = read_root_property(b"I3_SOCKET_PATH") else { return EXIT_FAILURE; };
                socket.push(b'\n');
                libc::fwrite(socket.as_ptr().cast(), socket.len(), 1, stdout);
                return EXIT_SUCCESS;
            },
            OPT_SHMLOG => {
                shmlog_size = i3_common::cli::decimal_prefix(value.as_ref().unwrap().as_bytes());
                init_logging();
                ::log::info!("Limiting SHM log size to {} bytes\n", { (shmlog_size) });
            },
            OPT_FAKE_OUTPUTS => {
                let value = value.as_ref().unwrap();
                {
// Snapshot native bytes before the logger can dispatch a callback.
let log_pointer_1 = value.as_ptr();
let log_value_0 = i3_common::logging::string_bytes((if log_pointer_1.is_null() { None } else { Some(std::ffi::CStr::from_ptr(log_pointer_1)) }).map(std::ffi::CStr::to_bytes), None).to_vec();
if let Ok(log_value_0_text) = std::str::from_utf8(&log_value_0) {
::log::info!("Initializing fake outputs: {}\n", log_value_0_text);
} else {
i3_common::logging::emit_parts(i3_common::logging::Level::Verbose, &[ "Initializing fake outputs: ".as_bytes(), &log_value_0, "\n".as_bytes() ]);
}
};
                fake_owner=Some(value.as_bytes().to_vec());
            },
            OPT_OLD_PARSER => ::log::error!("ERROR: You are passing --force-old-config-parser-v4.4-only, but that flag was removed by now.\n"),
            _ => unreachable!(),
        }
    }
    if only_check_config {
        exit(
            if load_configuration(override_configpath.as_deref(), ConfigLoad::Validate) {
                EXIT_SUCCESS
            } else {
                EXIT_FAILURE
            },
        );
    }
    if !operands.is_empty() {
        set_verbosity(true);
        ::log::info!("Additional arguments passed. Sending them as a command to i3.\n");
        let payload_bytes = operands
            .iter()
            .map(|argument| argument.as_bytes())
            .collect::<Vec<_>>()
            .join(&b' ');
        let payload_owner =
            std::ffi::CString::new(payload_bytes).expect("OS arguments have no NUL bytes");
        let payload = payload_owner.as_ptr();
        {
            // Snapshot native bytes before the logger can dispatch a callback.
            let log_pointer_1 = payload;
            let log_value_0 = i3_common::logging::string_bytes(
                (if log_pointer_1.is_null() {
                    None
                } else {
                    Some(std::ffi::CStr::from_ptr(log_pointer_1))
                })
                .map(std::ffi::CStr::to_bytes),
                None,
            )
            .to_vec();
            let log_value_1 = (strlen(payload)) as isize;
            if let Ok(log_value_0_text) = std::str::from_utf8(&log_value_0) {
                ::log::debug!(
                    "../src/main.c:main:500 - Command is: {} ({} bytes)\n",
                    log_value_0_text,
                    log_value_1
                );
            } else {
                i3_common::logging::emit_parts(
                    i3_common::logging::Level::Debug,
                    &[
                        "../src/main.c:main:500 - Command is: ".as_bytes(),
                        &log_value_0,
                        format!(" ({} bytes)\n", log_value_1).as_bytes(),
                    ],
                );
            }
        };
        let Some(socket_path) = read_root_property(b"I3_SOCKET_PATH") else {
            ::log::error!("ERROR: Could not get i3 IPC socket path\n");
            return 1_i32;
        };
        let socket_path = std::path::PathBuf::from(std::ffi::OsStr::from_bytes(&socket_path));
        let mut stream = match connect_forwarding_socket(&socket_path) {
            Ok(stream) => stream,
            Err(error) => {
                let (message, error) = match error {
                    ForwardConnectError::Socket(error) => (c"Could not create socket", error),
                    ForwardConnectError::Connect(error) => (c"Could not connect to i3", error),
                };
                *libc::__errno_location() = error.raw_os_error().unwrap_or(libc::EIO);
                err(EXIT_FAILURE, message.as_ptr());
            }
        };
        if let Err(error) = send_message_retry_would_block(
            &mut stream,
            I3_IPC_MESSAGE_TYPE_RUN_COMMAND as u32,
            payload_owner.to_bytes(),
        ) {
            *libc::__errno_location() = error.raw_os_error().unwrap_or(libc::EIO);
            err(EXIT_FAILURE, c"IPC: write()".as_ptr());
        }
        let reply = match receive_message_retry_payload(&mut stream) {
            Ok(reply) => reply,
            Err(error) => {
                report_receive_error(&error);
                if let ReceiveError::Io(error) = error {
                    *libc::__errno_location() = error.raw_os_error().unwrap_or(libc::EIO);
                    err(EXIT_FAILURE, c"IPC: read()".as_ptr());
                }
                return 1;
            }
        };
        if reply.message_type != I3_IPC_REPLY_TYPE_COMMAND as u32 {
            errx(
                EXIT_FAILURE,
                c"IPC: received reply of type %d but expected %d (COMMAND)".as_ptr(),
                reply.message_type,
                I3_IPC_REPLY_TYPE_COMMAND,
            );
        }
        // Match printf's bounded %.*s bytes: stop at the first NUL and emit a
        // newline even for an empty reply. No UTF-8 interpretation is performed.
        {
            use std::io::Write;
            let mut output = std::io::stdout().lock();
            let _ = output.write_all(command_reply_prefix(&reply.payload));
            let _ = output.write_all(b"\n");
        }
        return 0_i32;
    }
    init_logging();
    if is_debug_build() {
        let mut limit: rlimit = rlimit {
            rlim_cur: RLIM_INFINITY as libc::rlim_t,
            rlim_max: RLIM_INFINITY as libc::rlim_t,
        };
        setrlimit(libc::RLIMIT_CORE, &raw mut limit);
        ::log::info!("CORE DUMPS: You are running a development version of i3, so coredumps were automatically enabled (ulimit -c unlimited).\n");
        let cwd: *mut ::core::ffi::c_char =
            getcwd(::core::ptr::null_mut::<::core::ffi::c_char>(), 0_usize);
        if !cwd.is_null() {
            {
                // Snapshot native bytes before the logger can dispatch a callback.
                let log_pointer_1 = cwd;
                let log_value_0 = i3_common::logging::string_bytes(
                    (if log_pointer_1.is_null() {
                        None
                    } else {
                        Some(std::ffi::CStr::from_ptr(log_pointer_1))
                    })
                    .map(std::ffi::CStr::to_bytes),
                    None,
                )
                .to_vec();
                if let Ok(log_value_0_text) = std::str::from_utf8(&log_value_0) {
                    ::log::info!(
                        "CORE DUMPS: Your current working directory is \"{}\".\n",
                        log_value_0_text
                    );
                } else {
                    i3_common::logging::emit_parts(
                        i3_common::logging::Level::Verbose,
                        &[
                            "CORE DUMPS: Your current working directory is \"".as_bytes(),
                            &log_value_0,
                            "\".\n".as_bytes(),
                        ],
                    );
                }
            };
            free(cwd as *mut ::core::ffi::c_void);
        }
        if let Ok(mut file) = std::fs::File::open("/proc/sys/kernel/core_pattern") {
            let mut buffer = [0u8; 1024];
            if file.read(&mut buffer[..1023]).is_ok_and(|count| count > 0) {
                {
                    // Snapshot the first-NUL prefix through this zeroed byte span;
                    // the read above is bounded to 1023 bytes before dispatch.
                    let log_value_0 = buffer.split(|byte| *byte == 0).next().unwrap().to_vec();
                    if let Ok(log_value_0_text) = std::str::from_utf8(&log_value_0) {
                        ::log::info!("CORE DUMPS: Your core_pattern is: {}", log_value_0_text);
                    } else {
                        i3_common::logging::emit_parts(
                            i3_common::logging::Level::Verbose,
                            &[
                                "CORE DUMPS: Your core_pattern is: ".as_bytes(),
                                &log_value_0,
                            ],
                        );
                    }
                };
            }
        }
    }
    {
        // Snapshot native bytes before the logger can dispatch a callback.
        let log_pointer_1 = _i3_version;
        let log_value_0 = i3_common::logging::string_bytes(
            (if log_pointer_1.is_null() {
                None
            } else {
                Some(std::ffi::CStr::from_ptr(log_pointer_1))
            })
            .map(std::ffi::CStr::to_bytes),
            None,
        )
        .to_vec();
        if let Ok(log_value_0_text) = std::str::from_utf8(&log_value_0) {
            ::log::info!("i3 {} starting\n", log_value_0_text);
        } else {
            i3_common::logging::emit_parts(
                i3_common::logging::Level::Verbose,
                &["i3 ".as_bytes(), &log_value_0, " starting\n".as_bytes()],
            );
        }
    };
    // Keep extension events as owned UnknownEvent values for now. xcb 1.7's
    // optional extension resolver unwraps unknown extension opcodes/subtypes;
    // the existing checked native dispatcher must continue ignoring them.
    // Typed extension requests do not require event-resolution registration.
    let (connection, screen) = match xcb::Connection::connect(None) {
        Ok(connection) => connection,
        Err(_) => errx(EXIT_FAILURE, c"Cannot open display".as_ptr()),
    };
    conn_screen = screen;
    conn = connection.get_raw_conn().cast();
    MAIN_DISPLAY = Some(i3_common::display::install_connection(Rc::new(connection)));
    sndisplay = sn_xcb_display_new(conn, None, None);
    main_loop = ev_default_loop(0_u32);
    if main_loop.is_null() {
        errx(
            EXIT_FAILURE,
            c"Could not initialize libev. Bad LIBEV_FLAGS?\n".as_ptr(),
        );
    }
    MAIN_SCREEN = DisplayScreen::new(main_connection(), conn_screen);
    root = root_screen_info().root;
    // Query metadata without registering extension event decoders: unknown
    // future event subtypes must remain owned UnknownEvent packets.
    let xkb_extension_cookie =
        main_connection().send_request(&x::QueryExtension { name: b"XKEYBOARD" });
    let shape_extension_cookie =
        main_connection().send_request(&x::QueryExtension { name: b"SHAPE" });
    main_connection().send_request(&x::ChangeWindowAttributes {
        window: x::Window::new(root),
        value_list: &[x::Cw::EventMask(x::EventMask::PROPERTY_CHANGE)],
    });
    main_connection().send_request(&x::ChangeProperty {
        mode: x::PropMode::Append,
        window: x::Window::new(root),
        property: x::Atom::new(::xcb::x::ATOM_SUPERSCRIPT_X.resource_id()),
        r#type: x::ATOM_CARDINAL,
        data: &[] as &[u32],
    });
    let atom_requests = [
        (c"_NET_SUPPORTED", &raw mut A__NET_SUPPORTED),
        (
            c"_NET_SUPPORTING_WM_CHECK",
            &raw mut A__NET_SUPPORTING_WM_CHECK,
        ),
        (c"_NET_WM_NAME", &raw mut A__NET_WM_NAME),
        (c"_NET_WM_VISIBLE_NAME", &raw mut A__NET_WM_VISIBLE_NAME),
        (c"_NET_WM_MOVERESIZE", &raw mut A__NET_WM_MOVERESIZE),
        (c"_NET_WM_STATE_STICKY", &raw mut A__NET_WM_STATE_STICKY),
        (
            c"_NET_WM_STATE_FULLSCREEN",
            &raw mut A__NET_WM_STATE_FULLSCREEN,
        ),
        (
            c"_NET_WM_STATE_DEMANDS_ATTENTION",
            &raw mut A__NET_WM_STATE_DEMANDS_ATTENTION,
        ),
        (c"_NET_WM_STATE_MODAL", &raw mut A__NET_WM_STATE_MODAL),
        (c"_NET_WM_STATE_HIDDEN", &raw mut A__NET_WM_STATE_HIDDEN),
        (c"_NET_WM_STATE_FOCUSED", &raw mut A__NET_WM_STATE_FOCUSED),
        (
            c"_NET_WM_STATE_MAXIMIZED_VERT",
            &raw mut A__NET_WM_STATE_MAXIMIZED_VERT,
        ),
        (
            c"_NET_WM_STATE_MAXIMIZED_HORZ",
            &raw mut A__NET_WM_STATE_MAXIMIZED_HORZ,
        ),
        (c"_NET_WM_STATE", &raw mut A__NET_WM_STATE),
        (c"_NET_WM_WINDOW_TYPE", &raw mut A__NET_WM_WINDOW_TYPE),
        (
            c"_NET_WM_WINDOW_TYPE_NORMAL",
            &raw mut A__NET_WM_WINDOW_TYPE_NORMAL,
        ),
        (
            c"_NET_WM_WINDOW_TYPE_DOCK",
            &raw mut A__NET_WM_WINDOW_TYPE_DOCK,
        ),
        (
            c"_NET_WM_WINDOW_TYPE_DIALOG",
            &raw mut A__NET_WM_WINDOW_TYPE_DIALOG,
        ),
        (
            c"_NET_WM_WINDOW_TYPE_UTILITY",
            &raw mut A__NET_WM_WINDOW_TYPE_UTILITY,
        ),
        (
            c"_NET_WM_WINDOW_TYPE_TOOLBAR",
            &raw mut A__NET_WM_WINDOW_TYPE_TOOLBAR,
        ),
        (
            c"_NET_WM_WINDOW_TYPE_SPLASH",
            &raw mut A__NET_WM_WINDOW_TYPE_SPLASH,
        ),
        (
            c"_NET_WM_WINDOW_TYPE_MENU",
            &raw mut A__NET_WM_WINDOW_TYPE_MENU,
        ),
        (
            c"_NET_WM_WINDOW_TYPE_DROPDOWN_MENU",
            &raw mut A__NET_WM_WINDOW_TYPE_DROPDOWN_MENU,
        ),
        (
            c"_NET_WM_WINDOW_TYPE_POPUP_MENU",
            &raw mut A__NET_WM_WINDOW_TYPE_POPUP_MENU,
        ),
        (
            c"_NET_WM_WINDOW_TYPE_TOOLTIP",
            &raw mut A__NET_WM_WINDOW_TYPE_TOOLTIP,
        ),
        (
            c"_NET_WM_WINDOW_TYPE_NOTIFICATION",
            &raw mut A__NET_WM_WINDOW_TYPE_NOTIFICATION,
        ),
        (c"_NET_WM_DESKTOP", &raw mut A__NET_WM_DESKTOP),
        (c"_NET_WM_STRUT_PARTIAL", &raw mut A__NET_WM_STRUT_PARTIAL),
        (c"_NET_CLIENT_LIST", &raw mut A__NET_CLIENT_LIST),
        (
            c"_NET_CLIENT_LIST_STACKING",
            &raw mut A__NET_CLIENT_LIST_STACKING,
        ),
        (c"_NET_CURRENT_DESKTOP", &raw mut A__NET_CURRENT_DESKTOP),
        (
            c"_NET_NUMBER_OF_DESKTOPS",
            &raw mut A__NET_NUMBER_OF_DESKTOPS,
        ),
        (c"_NET_DESKTOP_NAMES", &raw mut A__NET_DESKTOP_NAMES),
        (c"_NET_DESKTOP_VIEWPORT", &raw mut A__NET_DESKTOP_VIEWPORT),
        (c"_NET_ACTIVE_WINDOW", &raw mut A__NET_ACTIVE_WINDOW),
        (c"_NET_CLOSE_WINDOW", &raw mut A__NET_CLOSE_WINDOW),
        (c"_NET_MOVERESIZE_WINDOW", &raw mut A__NET_MOVERESIZE_WINDOW),
        (c"_NET_WM_USER_TIME", &raw mut A__NET_WM_USER_TIME),
        (c"_NET_STARTUP_ID", &raw mut A__NET_STARTUP_ID),
        (c"_NET_WORKAREA", &raw mut A__NET_WORKAREA),
        (c"_NET_WM_ICON", &raw mut A__NET_WM_ICON),
        (c"WM_PROTOCOLS", &raw mut A_WM_PROTOCOLS),
        (c"WM_DELETE_WINDOW", &raw mut A_WM_DELETE_WINDOW),
        (c"UTF8_STRING", &raw mut A_UTF8_STRING),
        (c"WM_STATE", &raw mut A_WM_STATE),
        (c"WM_CLIENT_LEADER", &raw mut A_WM_CLIENT_LEADER),
        (c"WM_TAKE_FOCUS", &raw mut A_WM_TAKE_FOCUS),
        (c"WM_WINDOW_ROLE", &raw mut A_WM_WINDOW_ROLE),
        (c"I3_SOCKET_PATH", &raw mut A_I3_SOCKET_PATH),
        (c"I3_CONFIG_PATH", &raw mut A_I3_CONFIG_PATH),
        (c"I3_SYNC", &raw mut A_I3_SYNC),
        (c"I3_SHMLOG_PATH", &raw mut A_I3_SHMLOG_PATH),
        (c"I3_PID", &raw mut A_I3_PID),
        (
            c"I3_LOG_STREAM_SOCKET_PATH",
            &raw mut A_I3_LOG_STREAM_SOCKET_PATH,
        ),
        (c"I3_FLOATING_WINDOW", &raw mut A_I3_FLOATING_WINDOW),
        (
            c"_NET_REQUEST_FRAME_EXTENTS",
            &raw mut A__NET_REQUEST_FRAME_EXTENTS,
        ),
        (c"_NET_FRAME_EXTENTS", &raw mut A__NET_FRAME_EXTENTS),
        (c"_MOTIF_WM_HINTS", &raw mut A__MOTIF_WM_HINTS),
        (c"WM_CHANGE_STATE", &raw mut A_WM_CHANGE_STATE),
        (c"MANAGER", &raw mut A_MANAGER),
    ];
    let atom_cookies: Vec<_> = atom_requests
        .iter()
        .map(|(name, _)| {
            main_connection().send_request(&x::InternAtom {
                only_if_exists: false,
                name: name.to_bytes(),
            })
        })
        .collect();
    let screen_info = root_screen_info();
    root_depth = screen_info.root_depth;
    colormap = screen_info.default_colormap;
    // The native helper used class=-1 and selected the first visual of the
    // first nonempty depth-32 list. No closed VisualClass getter is needed.
    let argb = root_display().screen().allowed_depths().find_map(|depth| {
        (depth.depth() == 32)
            .then(|| depth.visuals().first().map(|visual| visual.visual_id()))
            .flatten()
    });
    let visual_id = argb.unwrap_or(screen_info.root_visual);
    MAIN_VISUAL = root_display().visual(visual_id);
    MAIN_RENDERER = Some(i3_common::draw_util::RenderingContext::with_default_visual(
        root_display().clone(),
        visual_id,
    ));
    if let Some(visual_id) = argb {
        root_depth = 32;
        let id: x::Colormap = main_connection().generate_id();
        colormap = id.resource_id();
        let cookie = main_connection().send_request_checked(&x::CreateColormap {
            alloc: x::ColormapAlloc::None,
            mid: id,
            window: x::Window::new(root),
            visual: visual_id,
        });
        if let Err(error) = main_connection().check_request(cookie) {
            ::log::error!("ERROR: Could not create colormap. Error code: {}\n", {
                (i32::from((*error.as_raw()).error_code))
            });
            exit(EXIT_FAILURE);
        }
    }
    main_connection().prefetch_maximum_request_length();
    init_dpi(root_display());
    ::log::debug!(
        "../src/main.c:main:648 - root_depth = {}, visual_id = 0x{:08x}.\n",
        { (root_depth as i32) },
        { (visual_id) }
    );
    ::log::debug!("../src/main.c:main:650 - root_screen->height_in_pixels = {}, root_screen->height_in_millimeters = {}\n", { (root_screen_info().height_in_pixels as i32) }, { (root_screen_info().height_in_millimeters as i32) });
    ::log::debug!("../src/main.c:main:651 - One logical pixel corresponds to {} physical pixels on this display.\n", (logical_px(1_i32)) as i32);
    let gcookie = main_connection().send_request(&x::GetGeometry {
        drawable: x::Drawable::Window(x::Window::new(root)),
    });
    let pointercookie = main_connection().send_request(&x::QueryPointer {
        window: x::Window::new(root),
    });
    let _ = main_connection().flush();
    ::log::debug!("../src/main.c:main:660 - waiting for PropertyNotify event\n");
    loop {
        match main_connection().wait_for_event() {
            Ok(event)
                if i32::from(event_response_type(&event))
                    == (<::xcb::x::PropertyNotifyEvent as ::xcb::BaseEvent>::NUMBER as i32) =>
            {
                if let xcb::Event::X(x::Event::PropertyNotify(property)) = event {
                    last_timestamp = property.time();
                    break;
                }
            }
            Ok(_) | Err(xcb::Error::Protocol(_)) => {}
            Err(xcb::Error::Connection(_)) => break,
        }
    }
    ::log::debug!(
        "../src/main.c:main:669 - got timestamp {}\n",
        (last_timestamp) as i32
    );
    for ((name, target), cookie) in atom_requests.into_iter().zip(atom_cookies) {
        let Ok(reply) = main_connection().wait_for_reply(cookie) else {
            {
                // Snapshot native bytes before the logger can dispatch a callback.
                let log_pointer_1 = name.as_ptr();
                let log_value_0 = i3_common::logging::string_bytes(
                    (if log_pointer_1.is_null() {
                        None
                    } else {
                        Some(std::ffi::CStr::from_ptr(log_pointer_1))
                    })
                    .map(std::ffi::CStr::to_bytes),
                    None,
                )
                .to_vec();
                if let Ok(log_value_0_text) = std::str::from_utf8(&log_value_0) {
                    ::log::error!("ERROR: Could not get atom {}\n", log_value_0_text);
                } else {
                    i3_common::logging::emit_parts(
                        i3_common::logging::Level::Error,
                        &[
                            "ERROR: Could not get atom ".as_bytes(),
                            &log_value_0,
                            "\n".as_bytes(),
                        ],
                    );
                }
            };
            exit(-1);
        };
        target.write(reply.atom().resource_id());
    }
    load_configuration(override_configpath.as_deref(), ConfigLoad::Load);
    if config.ipc_socket_path.is_none() {
        config.ipc_socket_path = std::env::var_os("I3SOCK")
            .map(std::path::PathBuf::from)
            .or_else(|| get_process_filename("ipc-socket"));
    }
    // Own the requested path across diagnostic callbacks; the bound result owns
    // the resolved path and descriptor independently of later config reloads.
    let socket_path = config.ipc_socket_path.clone();
    let ipc_listener = match socket_path
        .as_deref()
        .ok_or_else(|| std::io::Error::from_raw_os_error(libc::EINVAL))
        .and_then(create_socket)
    {
        Ok(bound) => bound,
        Err(_) => {
            let native_path = socket_path.as_ref().map(|path| {
                CString::new(path.as_os_str().as_bytes()).expect("native socket path has no NUL")
            });
            errx(
                EXIT_FAILURE,
                c"Could not create the IPC socket: %s".as_ptr(),
                native_path.as_deref().unwrap_or(c"(null)").as_ptr(),
            );
        }
    };
    current_socketpath = Some(ipc_listener.path.clone());
    let ipc_socket = ipc_listener.listener.as_raw_fd();
    // Declared before watchers, including the late-initialized log listener, so
    // watcher Drop always stops borrowed descriptors before their owners close.
    let log_listener: Option<i3_common::create_socket::BoundSocket>;

    if config.force_xinerama {
        force_xinerama = true;
    }
    // The native atom-name utility formats its screen argument as an unsigned
    // byte; retain that conversion for the WM_Sn selection name.
    let atom_name = format!("WM_S{}", conn_screen as u8);
    wm_sn_selection_owner = main_connection().generate_id::<x::Window>().resource_id();
    let cookie = main_connection().send_request_unchecked(&x::InternAtom {
        only_if_exists: false,
        name: atom_name.as_bytes(),
    });
    let Some(atom_reply) = main_connection()
        .wait_for_reply_unchecked(cookie)
        .ok()
        .flatten()
    else {
        ::log::error!("ERROR: Failed to intern the WM_Sn atom, exiting\n");
        return 1;
    };
    wm_sn = atom_reply.atom().resource_id();
    drop(atom_reply);
    let old_owner = main_connection()
        .wait_for_reply(main_connection().send_request(&x::GetSelectionOwner {
            selection: x::Atom::new(wm_sn),
        }))
        .ok()
        .map(|reply| reply.owner())
        .filter(|owner| !owner.is_none());
    if old_owner.is_some() && !replace_wm {
        ::log::error!("ERROR: Another window manager is already running (WM_Sn is owned)");
        return 1;
    }
    main_connection().send_request(&x::CreateWindow {
        depth: root_screen_info().root_depth,
        wid: x::Window::new(wm_sn_selection_owner),
        parent: x::Window::new(root_screen_info().root),
        x: -1,
        y: -1,
        width: 1,
        height: 1,
        border_width: 0,
        class: x::WindowClass::InputOutput,
        visual: root_screen_info().root_visual,
        value_list: &[],
    });
    main_connection().send_request(&x::ChangeProperty {
        mode: x::PropMode::Replace,
        window: x::Window::new(wm_sn_selection_owner),
        property: x::ATOM_WM_CLASS,
        r#type: x::ATOM_STRING,
        data: b"i3-WM_Sn\0i3-WM_Sn\0",
    });
    main_connection().send_request(&x::SetSelectionOwner {
        owner: x::Window::new(wm_sn_selection_owner),
        selection: x::Atom::new(wm_sn),
        time: last_timestamp,
    });
    if let Some(old_owner) = old_owner {
        let mut check_rounds = 150;
        ::log::debug!("../src/main.c:main:768 - waiting for old WM_Sn selection owner to exit");
        loop {
            usleep(100000);
            if check_rounds == 0 {
                ::log::error!("ERROR: The old window manager is not exiting");
                return 1;
            }
            check_rounds -= 1;
            let cookie = main_connection().send_request(&x::GetGeometry {
                drawable: x::Drawable::Window(old_owner),
            });
            if main_connection().wait_for_reply(cookie).is_err() {
                break;
            }
        }
    }
    let manager = manager_notification(
        root_screen_info().root,
        A_MANAGER,
        last_timestamp,
        wm_sn,
        wm_sn_selection_owner,
    );
    main_connection().send_request(&x::SendEvent {
        propagate: false,
        destination: x::SendEventDest::Window(x::Window::new(root_screen_info().root)),
        event_mask: x::EventMask::STRUCTURE_NOTIFY,
        event: &manager,
    });
    let cookie = main_connection().send_request_checked(&x::ChangeWindowAttributes {
        window: x::Window::new(root),
        value_list: &[x::Cw::EventMask(
            x::EventMask::SUBSTRUCTURE_REDIRECT
                | x::EventMask::BUTTON_PRESS
                | x::EventMask::STRUCTURE_NOTIFY
                | x::EventMask::POINTER_MOTION
                | x::EventMask::PROPERTY_CHANGE
                | x::EventMask::FOCUS_CHANGE
                | x::EventMask::ENTER_WINDOW,
        )],
    });
    if let Err(error) = main_connection().check_request(cookie) {
        ::log::error!(
            "ERROR: Another window manager seems to be running (X error {})\n",
            { (i32::from((*error.as_raw()).error_code)) }
        );
        return 1_i32;
    }
    let Ok(greply) = main_connection().wait_for_reply(gcookie) else {
        ::log::error!("ERROR: Could not get geometry of the root window, exiting\n");
        return 1_i32;
    };
    ::log::debug!(
        "../src/main.c:main:819 - root geometry reply: ({}, {}) {} x {}\n",
        { (greply.x() as i32) },
        { (greply.y() as i32) },
        { (greply.width() as i32) },
        { (greply.height() as i32) }
    );
    xcursor_load_cursors();
    xcursor_set_root_cursor(Cursor::Pointer);
    let xkb_extension = main_connection().wait_for_reply(xkb_extension_cookie).ok();
    xkb_supported = xkb_extension.as_ref().is_some_and(|reply| reply.present());
    if !xkb_supported {
        ::log::debug!("../src/main.c:main:831 - xkb is not present on this server\n");
    } else {
        ::log::debug!("../src/main.c:main:833 - initializing xcb-xkb\n");
        let cookie = main_connection().send_request(&xkb::UseExtension {
            wanted_major: xkb::MAJOR_VERSION as u16,
            wanted_minor: xkb::MINOR_VERSION as u16,
        });
        // The original request did not wait for negotiation. Discard the owned
        // full-sequence cookie instead of retaining an unconsumed reply.
        xcb_interop::transport::discard_reply(&main_connection(), cookie);
        let events = xkb::EventType::STATE_NOTIFY
            | xkb::EventType::MAP_NOTIFY
            | xkb::EventType::NEW_KEYBOARD_NOTIFY;
        main_connection().send_request(&xkb::SelectEvents {
            device_spec: xkb::Id::UseCoreKbd as u16,
            affect_which: events,
            clear: xkb::EventType::empty(),
            select_all: events,
            affect_map: xkb::MapPart::from_bits_retain(0xff),
            map: xkb::MapPart::from_bits_retain(0xff),
            details: &[],
        });
        let mask = xkb::PerClientFlag::GRABS_USE_XKB_STATE
            | xkb::PerClientFlag::LOOKUP_STATE_WHEN_GRABBED
            | xkb::PerClientFlag::DETECTABLE_AUTO_REPEAT;
        let cookie = main_connection().send_request(&xkb::PerClientFlags {
            device_spec: xkb::Id::UseCoreKbd as u16,
            change: mask,
            value: mask,
            ctrls_to_change: xkb::BoolCtrl::empty(),
            auto_ctrls: xkb::BoolCtrl::empty(),
            auto_ctrls_values: xkb::BoolCtrl::empty(),
        });
        let reply = main_connection().wait_for_reply(cookie).ok();
        let pcf_value = reply
            .as_ref()
            .map_or(xkb::PerClientFlag::empty(), |reply| reply.value());
        if !pcf_value.contains(xkb::PerClientFlag::GRABS_USE_XKB_STATE) {
            ::log::error!("ERROR: Could not set XCB_XKB_PER_CLIENT_FLAG_GRABS_USE_XKB_STATE\n");
        }
        if !pcf_value.contains(xkb::PerClientFlag::LOOKUP_STATE_WHEN_GRABBED) {
            ::log::error!(
                "ERROR: Could not set XCB_XKB_PER_CLIENT_FLAG_LOOKUP_STATE_WHEN_GRABBED\n"
            );
        }
        if !pcf_value.contains(xkb::PerClientFlag::DETECTABLE_AUTO_REPEAT) {
            ::log::error!("ERROR: Could not set XCB_XKB_PER_CLIENT_FLAG_DETECTABLE_AUTO_REPEAT\n");
        }
        xkb_base = i32::from(
            xkb_extension
                .as_ref()
                .expect("present XKB extension metadata")
                .first_event(),
        );
    }
    let shape_extension = main_connection()
        .wait_for_reply(shape_extension_cookie)
        .ok();
    if let Some(extension) = shape_extension.as_ref().filter(|reply| reply.present()) {
        shape_base = i32::from(extension.first_event());
        let cookie = main_connection().send_request(&shape::QueryVersion {});
        shape_supported = main_connection()
            .wait_for_reply(cookie)
            .ok()
            .is_some_and(|version| version.minor_version() >= 1);
    } else {
        shape_supported = false;
    }
    if !shape_supported {
        ::log::debug!("../src/main.c:main:903 - shape 1.1 is not present on this server\n");
    }
    restore_connect();
    property_handlers_init();
    ewmh_setup_hints();
    rebuild_key_symbols();
    xcb_numlock_mask = key_symbol_modifiers(xkbcommon::xkb::keysyms::KEY_Num_Lock);
    if !load_keymap() {
        errx(EXIT_FAILURE, c"Could not load keymap\n".as_ptr());
    }
    translate_keysyms();
    grab_all_keys();
    let mut needs_tree_init: bool = true;
    if !layout_path.is_null() {
        {
            // Snapshot native bytes before the logger can dispatch a callback.
            let log_pointer_1 = layout_path;
            let log_value_0 = i3_common::logging::string_bytes(
                (if log_pointer_1.is_null() {
                    None
                } else {
                    Some(std::ffi::CStr::from_ptr(log_pointer_1))
                })
                .map(std::ffi::CStr::to_bytes),
                None,
            )
            .to_vec();
            if let Ok(log_value_0_text) = std::str::from_utf8(&log_value_0) {
                ::log::info!(
                    "Trying to restore the layout from \"{}\".\n",
                    log_value_0_text
                );
            } else {
                i3_common::logging::emit_parts(
                    i3_common::logging::Level::Verbose,
                    &[
                        "Trying to restore the layout from \"".as_bytes(),
                        &log_value_0,
                        "\".\n".as_bytes(),
                    ],
                );
            }
        };
        let restore_path = std::path::PathBuf::from(std::ffi::OsStr::from_bytes(
            CStr::from_ptr(layout_path).to_bytes(),
        ));
        needs_tree_init = !tree_restore(&restore_path, &greply);
        if delete_layout_path {
            unlink(layout_path);
            let dir = owned_dirname(CStr::from_ptr(layout_path));
            rmdir(dir.as_ptr());
        }
    }
    if needs_tree_init {
        tree_init(&greply);
    }
    drop(greply);
    let configured_fake = fake_owner.take().or_else(|| config.fake_outputs.take());
    if let Some(specification) = configured_fake {
        fake_outputs_init(&specification);
        config.fake_outputs = None;
    } else if force_xinerama {
        xinerama_init();
    } else {
        ::log::debug!("../src/main.c:main:956 - Checking for XRandR...\n");
        randr_init(
            &raw mut randr_base,
            disable_randr15 || config.disable_randr15,
        );
    }
    if !layout_path.is_null() && randr_base > -1_i32 {
        let mut con: *mut Con = nodes::first_ptr(croot);

        while !con.is_null() {
            let mut output: *mut Output = outputs.tqh_first as *mut Output;

            while !output.is_null() {
                if !((*output).active
                    || (*con).name.as_deref() != Some(output_primary_name(output).as_slice()))
                {
                    if (*output).con.is_null() {
                        output_init_con(output);
                        (*output).changed = false;
                    }
                    (*output).to_be_disabled = true;
                    randr_disable_output(output);
                }
                output = (*output).outputs.tqe_next as *mut Output;
            }
            con = nodes::next_ptr(con);
        }
    }
    drop(layout_owner.take());
    layout_path = ::core::ptr::null_mut::<::core::ffi::c_char>();
    scratchpad_fix_resolution();
    let mut output_0: *mut Output = ::core::ptr::null_mut::<Output>();
    let pointer_reply = main_connection().wait_for_reply(pointercookie).ok();
    if pointer_reply.is_none() {
        ::log::error!("ERROR: Could not query pointer position, using first screen\n");
    } else if let Some(pointer_reply) = pointer_reply {
        ::log::debug!(
            "../src/main.c:main:995 - Pointer at {}, {}\n",
            { (pointer_reply.root_x() as i32) },
            { (pointer_reply.root_y() as i32) }
        );
        output_0 =
            get_output_containing(pointer_reply.root_x() as u32, pointer_reply.root_y() as u32);
        if output_0.is_null() {
            ::log::error!(
                "ERROR: ERROR: No screen at ({}, {}), starting on the first screen\n",
                { (pointer_reply.root_x() as i32) },
                { (pointer_reply.root_y() as i32) }
            );
        }
    }
    if output_0.is_null() {
        output_0 = get_first_output();
    }
    con_activate(con_descend_focused(output_get_content((*output_0).con)));
    tree_render();
    let mut watchers = MainWatchers::new();
    let ipc_io = watchers.add(ipc_socket, ipc_new_client);
    log_listener =
        get_process_filename("log-stream-socket").and_then(|path| create_socket(&path).ok());
    let mut log_io: *mut ev_io = std::ptr::null_mut();
    if let Some(bound) = log_listener.as_ref() {
        current_log_stream_socket_path = Some(bound.path.clone());
        log_io = watchers.add(bound.listener.as_raw_fd(), log_new_client);
    } else {
        ::log::error!("ERROR: Could not create the log socket, i3-dump-log -f will not work\n");
    }
    let mut socket_ipc_io: *mut ev_io = ::core::ptr::null_mut::<ev_io>();
    listen_fds = crate::sd_daemon::listen_fds();
    if listen_fds < 0_i32 {
        ::log::error!("ERROR: socket activation: Error in sd_listen_fds\n");
    } else if listen_fds == 0_i32 {
        ::log::debug!("../src/main.c:main:1036 - socket activation: no sockets passed\n");
    } else {
        let mut flags: i32 = 0;
        let mut fd: i32 = SD_LISTEN_FDS_START;
        while fd < SD_LISTEN_FDS_START + listen_fds {
            ::log::debug!(
                "../src/main.c:main:1042 - socket activation: also listening on fd {}\n",
                { (fd) }
            );
            flags = fcntl(fd, F_GETFD);
            if flags < 0_i32 || fcntl(fd, F_SETFD, flags & !FD_CLOEXEC) < 0_i32 {
                ::log::error!("ERROR: Could not disable FD_CLOEXEC on fd {}\n", { (fd) });
            }
            socket_ipc_io = watchers.add(fd, ipc_new_client);
            fd += 1;
        }
    }
    let restart_fd: i32 = parse_restart_fd();
    if restart_fd != -1_i32 {
        ::log::debug!("../src/main.c:main:1061 - serving restart fd {}", {
            (restart_fd)
        });
        let client: *mut ipc_client = ipc_new_client_on_fd(main_loop, restart_fd);
        ipc_confirm_restart(client);
        unsetenv(c"_I3_RESTART_FD".as_ptr());
    }
    x_set_i3_atoms();
    ewmh_update_workarea();
    ewmh_update_desktop_properties();
    let xcb_watcher = watchers.add(main_connection().as_raw_fd(), xcb_got_event);
    xcb_prepare = Box::into_raw(Box::new(ev_prepare::new(xcb_prepare_cb)));
    ev_prepare_start(main_loop, xcb_prepare);
    let _ = main_connection().flush();
    main_connection().send_request(&x::GrabServer {});
    let _ = main_connection().wait_for_reply(main_connection().send_request(&x::GetInputFocus {}));
    loop {
        match main_connection().poll_for_event() {
            Ok(Some(event)) => {
                let kind = event_kind(&event);
                if kind == (<::xcb::x::MapRequestEvent as ::xcb::BaseEvent>::NUMBER as i32) {
                    handle_event(&event);
                }
            }
            Err(xcb::Error::Protocol(_)) => {}
            Ok(None) | Err(xcb::Error::Connection(_)) => break,
        }
    }
    manage_existing_windows(root);
    main_connection().send_request(&x::UngrabServer {});
    if autostart {
        ::log::info!("This is not an in-place restart, checking if a wallpaper is set.\n");
        if is_background_set(&main_connection(), root_screen_info()) {
            ::log::info!("A wallpaper is set, so no screenshot is necessary.\n");
        } else {
            ::log::info!("No wallpaper set, copying root window contents to a pixmap\n");
            set_screenshot_as_wallpaper(&main_connection(), root_screen_info());
        }
    }
    if !disable_signalhandler {
        setup_signal_handler();
    } else {
        let mut action: sigaction = ::core::mem::zeroed();
        action.sa_sigaction = handle_core_signal as *const () as usize;
        action.sa_flags = (SA_NODEFER as u32 | SA_RESETHAND | SA_SIGINFO as u32) as i32;
        sigemptyset(&raw mut action.sa_mask);
        if sigaction(
            SIGQUIT,
            &raw mut action,
            ::core::ptr::null_mut::<sigaction>(),
        ) == -1_i32
            || sigaction(
                SIGILL,
                &raw mut action,
                ::core::ptr::null_mut::<sigaction>(),
            ) == -1_i32
            || sigaction(
                SIGABRT,
                &raw mut action,
                ::core::ptr::null_mut::<sigaction>(),
            ) == -1_i32
            || sigaction(
                SIGFPE,
                &raw mut action,
                ::core::ptr::null_mut::<sigaction>(),
            ) == -1_i32
            || sigaction(
                SIGSEGV,
                &raw mut action,
                ::core::ptr::null_mut::<sigaction>(),
            ) == -1_i32
        {
            ::log::error!("ERROR: Could not setup signal handler.\n");
        }
    }
    setup_term_handlers();
    signal(SIGPIPE, libc::SIG_IGN);
    if autostart {
        while let Some(exec) = take_first_autostart(false) {
            {
                // The detached owner keeps native bytes live through logging and launch.
                let log_value_0 = exec.command.as_slice();
                if let Ok(log_value_0_text) = std::str::from_utf8(log_value_0) {
                    ::log::info!("auto-starting {}\n", log_value_0_text);
                } else {
                    i3_common::logging::emit_parts(
                        i3_common::logging::Level::Verbose,
                        &["auto-starting ".as_bytes(), log_value_0, "\n".as_bytes()],
                    );
                }
            };
            start_application(&exec.command, exec.no_startup_id, 0_u32);
        }
    }
    while let Some(exec_always) = take_first_autostart(true) {
        {
            // The detached owner keeps native bytes live through logging and launch.
            let log_value_0 = exec_always.command.as_slice();
            if let Ok(log_value_0_text) = std::str::from_utf8(log_value_0) {
                ::log::info!("auto-starting (always!) {}\n", log_value_0_text);
            } else {
                i3_common::logging::emit_parts(
                    i3_common::logging::Level::Verbose,
                    &[
                        "auto-starting (always!) ".as_bytes(),
                        log_value_0,
                        "\n".as_bytes(),
                    ],
                );
            }
        };
        start_application(&exec_always.command, exec_always.no_startup_id, 0_u32);
    }
    let mut barconfig: *mut Barconfig = barconfigs.tqh_first;

    while !barconfig.is_null() {
        let program = (*barconfig)
            .i3bar_command
            .as_deref()
            .unwrap_or(b"exec i3bar");
        let command = bar_command(
            program,
            (*barconfig).verbose,
            (*barconfig).id(),
            current_socketpath
                .as_deref()
                .map_or(&[][..], |path| path.as_os_str().as_bytes()),
        );
        {
            let log_value_0 = &command;
            if let Ok(log_value_0_text) = std::str::from_utf8(log_value_0) {
                ::log::info!("Starting bar process: {}\n", log_value_0_text);
            } else {
                i3_common::logging::emit_parts(
                    i3_common::logging::Level::Verbose,
                    &[
                        "Starting bar process: ".as_bytes(),
                        log_value_0,
                        "\n".as_bytes(),
                    ],
                );
            }
        };
        start_application(&command, true, 0);
        barconfig = (*barconfig).configs.tqe_next;
    }
    atexit(i3_exit);
    raise(SIGCHLD);
    sd_notify(1_i32, c"READY=1");
    ev_loop(main_loop, 0_i32);
    ev_prepare_stop(main_loop, xcb_prepare);
    drop(Box::from_raw(xcb_prepare));
    xcb_prepare = std::ptr::null_mut();
    drop(watchers);
    0
}
pub const EXIT_FAILURE: i32 = 1_i32;
pub const EXIT_SUCCESS: i32 = 0_i32;
pub fn main() {
    // Native startup and the event loop still use their established ownership
    // contracts. CLI parsing itself owns byte-preserving Rust arguments.
    std::process::exit(unsafe { run(std::env::args_os().collect()) });
}
unsafe extern "C" fn run_static_initializers() {
    autostarts = autostarts_head {
        tqh_first: ::core::ptr::null_mut::<Autostart>(),
        tqh_last: &raw mut autostarts.tqh_first,
    };
    autostarts_always = autostarts_always_head {
        tqh_first: ::core::ptr::null_mut::<Autostart>(),
        tqh_last: &raw mut autostarts_always.tqh_first,
    };
    ws_assignments = ws_assignments_head {
        tqh_first: ::core::ptr::null_mut::<Workspace_Assignment>(),
        tqh_last: &raw mut ws_assignments.tqh_first,
    };
    assignments = assignments_head {
        tqh_first: ::core::ptr::null_mut::<Assignment>(),
        tqh_last: &raw mut assignments.tqh_first,
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];

// Internal helpers use Rust linkage rather than foreign symbol lookup.
use crate::bindings::load_keymap;
use crate::bindings::translate_keysyms;
use crate::config::load_configuration;
use crate::display_version::display_running_version;
use crate::ewmh::ewmh_setup_hints;
use crate::ewmh::ewmh_update_desktop_properties;
use crate::ewmh::ewmh_update_workarea;
use crate::fake_outputs::fake_outputs_init;
use crate::handlers::property_handlers_init;
use crate::ipc::ipc_shutdown;
use crate::log::init_logging;
use crate::log::set_debug_logging;
use crate::log::set_verbosity;
use crate::manage::manage_existing_windows;
use crate::randr::randr_init;
use crate::restore_layout::restore_connect;
use crate::scratchpad::scratchpad_fix_resolution;
use crate::sd_daemon::sd_notify;
use crate::sighandler::setup_signal_handler;
use crate::startup::start_application;
use crate::tree::tree_render;
use crate::util::kill_nagbar;
use crate::util::parse_long;
use crate::x::x_set_i3_atoms;
use crate::xcursor::xcursor_load_cursors;
use crate::xcursor::xcursor_set_root_cursor;
use crate::xinerama::xinerama_init;
use i3_common::dpi::init_dpi;
use i3_common::dpi::logical_px;
use i3_common::is_debug_build::is_debug_build;
use i3_common::safewrappers::scalloc;
use i3_common::safewrappers::sstrdup;

// Canonical Cairo backend ABI; renderer logic remains local.

// Canonical Pango backend ABI; font policy remains local.

use crate::handlers::xkb_base;

use startup_notification_sys::{sn_xcb_display_new, SnDisplay};

pub use libc::{
    atexit, close, dirname, exit, fcntl, fflush, fileno, fprintf, free, getcwd, getenv, getrlimit,
    isatty, memmove, open, printf, raise, read, rlimit, rmdir, setbuf, setlocale, setrlimit,
    shm_unlink, sigaction, sigemptyset, siginfo_t, signal, srand, strcmp, strlen, strstr, time,
    unlink, unsetenv, usleep,
};

use i3_common::native_stdio::{err, errx, stderr, stdout};

// Shared model types retain field layout while using one Rust identity.
pub use crate::types::gaps_t;
pub use crate::types::xoutput;
pub use crate::types::Assignment;
pub use crate::types::Con;
pub use crate::types::Rect;
pub use i3_common::font::Font;

/// This command-forwarding boundary preserves separate socket/connect failures
/// and the native 107-byte pathname truncation, while owning the descriptor.
#[derive(Debug)]
enum ForwardConnectError {
    Socket(std::io::Error),
    Connect(std::io::Error),
}
fn connect_forwarding_socket(
    path: &std::path::Path,
) -> Result<std::os::unix::net::UnixStream, ForwardConnectError> {
    use std::os::unix::ffi::OsStrExt;
    // SAFETY: socket creates a unique descriptor, adopted immediately on success.
    let descriptor = unsafe { libc::socket(libc::AF_UNIX, libc::SOCK_STREAM, 0) };
    if descriptor < 0 {
        return Err(ForwardConnectError::Socket(std::io::Error::last_os_error()));
    }
    let owner = unsafe { std::os::fd::OwnedFd::from_raw_fd(descriptor) };
    let mut address = libc::sockaddr_un {
        sun_family: libc::AF_UNIX as libc::sa_family_t,
        sun_path: [0; 108],
    };
    let bytes = path.as_os_str().as_bytes();
    let bytes = &bytes[..bytes
        .iter()
        .position(|&byte| byte == 0)
        .unwrap_or(bytes.len())];
    for (slot, byte) in address.sun_path.iter_mut().take(107).zip(bytes) {
        *slot = *byte as libc::c_char;
    }
    // SAFETY: the initialized address is live for its complete native size.
    if unsafe {
        libc::connect(
            owner.as_raw_fd(),
            (&address as *const libc::sockaddr_un).cast(),
            std::mem::size_of::<libc::sockaddr_un>() as libc::socklen_t,
        )
    } < 0
    {
        return Err(ForwardConnectError::Connect(std::io::Error::last_os_error()));
    }
    Ok(std::os::unix::net::UnixStream::from(owner))
}
fn command_reply_prefix(payload: &[u8]) -> &[u8] {
    &payload[..payload
        .iter()
        .position(|byte| *byte == 0)
        .unwrap_or(payload.len())]
}

#[cfg(test)]
mod cli_tests {
    use super::*;
    use i3_common::cli::{Item, Parser};
    use std::os::unix::ffi::{OsStrExt, OsStringExt};

    #[test]
    fn restart_boundary_retains_bytes_and_a_c_allocated_pointer_table() {
        let arguments = vec![
            "i3".into(),
            std::ffi::OsString::from_vec(b"/tmp/\xff.conf".to_vec()),
        ];
        let mut owner = RestartArguments::new(&arguments).unwrap();
        // SAFETY: each pointer slot is in the calloc allocation and points to a
        // CString still owned by owner; calloc also installed the sentinel.
        unsafe {
            assert_eq!(std::ffi::CStr::from_ptr(*owner.pointers).to_bytes(), b"i3");
            assert_eq!(
                std::ffi::CStr::from_ptr(*owner.pointers.add(1)).to_bytes(),
                arguments[1].as_bytes()
            );
            assert!((*owner.pointers.add(2)).is_null());
            let resized = libc::realloc(
                owner.pointers.cast(),
                4 * std::mem::size_of::<*mut libc::c_char>(),
            )
            .cast::<*mut libc::c_char>();
            assert!(!resized.is_null());
            owner.pointers = resized;
            assert_eq!(
                std::ffi::CStr::from_ptr(*owner.pointers.add(1)).to_bytes(),
                b"/tmp/\xff.conf"
            );
        }
    }
    #[test]
    fn gnu_long_aliases_and_short_only_options_keep_their_contracts() {
        let values = ["--more", "--disable", "--shmlog-size=0", "-C", "--", "-a"]
            .map(std::ffi::OsString::from);
        let parsed: Vec<_> = Parser::new(values, CORE_OPTIONS, false).collect();
        assert_eq!(
            parsed,
            vec![
                Item::Option {
                    short: b'm',
                    value: None
                },
                Item::Option {
                    short: OPT_RANDR15,
                    value: None
                },
                Item::Option {
                    short: OPT_SHMLOG,
                    value: Some("0".into())
                },
                Item::Option {
                    short: b'C',
                    value: None
                },
                Item::Operand("-a".into()),
            ]
        );
        let parsed: Vec<_> = Parser::new(
            ["--verbose"].map(std::ffi::OsString::from),
            CORE_OPTIONS,
            false,
        )
        .collect();
        assert!(matches!(&parsed[..], [Item::Error(_)]));
    }
}

// Internal calls and globals use canonical Rust identities.
use crate::bindings::command_error_nagbar_pid;
use crate::bindings::grab_all_keys;
use crate::con::con_activate;
use crate::con::con_descend_focused;
use crate::config::config;
use crate::config_parser::config_error_nagbar_pid;
use crate::handlers::handle_event;
use crate::handlers::randr_base;
use crate::handlers::shape_base;
use crate::ipc::current_socketpath;
use crate::ipc::ipc_new_client;
use crate::log::current_log_stream_socket_path;
use crate::log::log_new_client;
use crate::log::shmlog_size;
use crate::log::shmlogname;
use crate::output::output_get_content;
use crate::output::output_primary_name;
use crate::randr::get_first_output;
use crate::randr::get_output_containing;
use crate::randr::output_init_con;
use crate::randr::randr_disable_output;
use crate::tree::croot;
use crate::tree::tree_init;
use crate::tree::tree_restore;
use crate::version::i3_version;
use i3_common::create_socket::create_socket;
use i3_common::get_process_filename::get_process_filename;
use i3_common::ipc_recv_message::{
    receive_message_retry_payload, report_receive_error, ReceiveError,
};
use i3_common::ipc_send_message::send_message_retry_would_block;
use i3_common::is_background_set::is_background_set;
use i3_common::root_atom_contents::read_root_property;
use i3_common::screenshot_wallpaper::set_screenshot_as_wallpaper;

use crate::xcb::xcb_numlock_mask;

#[cfg(test)]
mod owned_startup_tests {
    use super::*;
    #[test]
    fn bar_command_preserves_native_spaces_quotes_and_non_utf8() {
        assert_eq!(
            bar_command(b"exec i3bar", false, b"id\xff", b"/socket\xfe"),
            b"exec i3bar  --bar_id=id\xff --socket=\"/socket\xfe\""
        );
        assert_eq!(
            bar_command(b"custom", true, b"bar", b"/path"),
            b"custom -V --bar_id=bar --socket=\"/path\""
        );
        assert_eq!(owned_dirname(c"relative/file").as_c_str(), c"relative");
    }
}

use crate::con::arena::nodes;

#[cfg(test)]
mod owned_event_tests {
    use super::*;

    /// Manufacture an initialized malloc allocation, transferred directly to
    /// the maintained decoder just like a native event queue allocation.
    unsafe fn event(opcode: u8, detail: u8) -> xcb::Event {
        let raw = libc::calloc(1, std::mem::size_of::<xcb::ffi::xcb_generic_event_t>())
            .cast::<xcb::ffi::xcb_generic_event_t>();
        assert!(!raw.is_null());
        (*raw).response_type = opcode;
        (*raw).pad0 = detail;
        (*raw).sequence = 0x1234;
        (*raw).full_sequence = 0xfedc_1234;
        xcb::resolve_event(raw, &[])
    }

    #[test]
    fn manager_factory_has_exact_wire_body_and_needs_no_native_sequence_tail() {
        let message =
            manager_notification(0x10203040, 0x50607080, 0x90abcdef, 0x12345678, 0xfedcba98);
        let mut expected = [0u8; 32];
        expected[0] = 33;
        expected[1] = 32;
        for (offset, value) in [
            (4, 0x10203040u32),
            (8, 0x50607080),
            (12, 0x90abcdef),
            (16, 0x12345678),
            (20, 0xfedcba98),
        ] {
            expected[offset..offset + 4].copy_from_slice(&value.to_ne_bytes());
        }
        // The canonical factory owns exactly the 32-byte wire body.
        assert_eq!(
            unsafe { std::slice::from_raw_parts(message.as_raw().cast::<u8>(), 32) },
            expected
        );
        let event = xcb::Event::X(x::Event::ClientMessage(message));
        assert_eq!(event_response_type(&event), 33);
        assert_eq!(event_kind(&event), 33);
    }

    #[test]
    fn unknown_extension_opcode_and_subtype_keep_their_native_headers() {
        // No extension metadata: a future XKB subtype or unknown event remains
        // an owned UnknownEvent rather than reaching an unwrap/enum conversion.
        let event = unsafe { event(100 | 0x80, u8::MAX) };
        assert!(matches!(&event, xcb::Event::Unknown(_)));
        assert_eq!(event_kind(&event), 100);
        // This fixture explicitly allocated the native 36-byte queue header.
        let header = unsafe { &*event.as_raw() };
        assert_eq!(header.pad0, u8::MAX);
        assert_eq!(header.sequence, 0x1234);
        assert_eq!(header.full_sequence, 0xfedc_1234);
    }

    #[test]
    fn core_event_ownership_does_not_convert_unknown_native_selectors() {
        // PropertyNotify's state byte is an open native value here. The core
        // owned event must survive decoding without invoking its enum getter.
        let event = unsafe {
            event(
                (<::xcb::x::PropertyNotifyEvent as ::xcb::BaseEvent>::NUMBER as i32) as u8 | 0x80,
                0,
            )
        };
        unsafe { event.as_raw().cast::<u8>().add(16).write(u8::MAX) };
        assert!(matches!(
            &event,
            xcb::Event::X(xcb::x::Event::PropertyNotify(_))
        ));
        assert_eq!(
            event_kind(&event),
            (<::xcb::x::PropertyNotifyEvent as ::xcb::BaseEvent>::NUMBER as i32)
        );
        // Startup timestamp filtering historically ignores SendEvent-flagged
        // PropertyNotify, while ordinary dispatch masks that same flag.
        assert_ne!(
            i32::from(event_response_type(&event)),
            (<::xcb::x::PropertyNotifyEvent as ::xcb::BaseEvent>::NUMBER as i32)
        );
        assert_eq!(
            unsafe { event.as_raw().cast::<u8>().add(16).read() },
            u8::MAX
        );
    }

    #[test]
    fn unknown_protocol_error_preserves_full_sequence_and_numeric_diagnostic() {
        unsafe {
            let raw = libc::calloc(1, std::mem::size_of::<xcb::ffi::xcb_generic_error_t>())
                .cast::<xcb::ffi::xcb_generic_error_t>();
            assert!(!raw.is_null());
            (*raw).error_code = u8::MAX;
            (*raw).sequence = 0x5678;
            (*raw).full_sequence = 0xfedc_5678;
            (*raw).major_code = u8::MAX;
            (*raw).minor_code = u16::MAX;
            let error = xcb::resolve_error(raw, &[]);
            assert!(matches!(&error, xcb::ProtocolError::Unknown(_, _)));
            assert_eq!(error_header(&error).error_code, u8::MAX);
            assert_eq!(error_header(&error).sequence, 0x5678);
            assert_eq!(error_header(&error).full_sequence, 0xfedc_5678);
            assert_eq!(error_header(&error).major_code, u8::MAX);
            assert_eq!(error_header(&error).minor_code, u16::MAX);
        }
    }
}

#[cfg(test)]
mod autostart_ownership_tests {
    use super::*;

    #[test]
    fn pending_commands_detach_before_callbacks_and_preserve_queue_and_native_bytes() {
        if std::env::var_os("I333_AUTOSTART_OWNER_PROBE").is_none() {
            let output = std::process::Command::new(std::env::current_exe().unwrap())
                .args(["--exact", "application::autostart_ownership_tests::pending_commands_detach_before_callbacks_and_preserve_queue_and_native_bytes", "--nocapture"])
                .env("I333_AUTOSTART_OWNER_PROBE", "1")
                .env_remove("DISPLAY").env_remove("I3SOCK").env_remove("WAYLAND_DISPLAY")
                .output().unwrap();
            assert!(
                output.status.success(),
                "{}\n{}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
            return;
        }
        unsafe {
            assert!(autostarts.tqh_first.is_null());
            assert!(autostarts_always.tqh_first.is_null());
            autostarts.tqh_last = &raw mut autostarts.tqh_first;
            autostarts_always.tqh_last = &raw mut autostarts_always.tqh_first;
            let command = std::ffi::CString::new(vec![0xff, b'x']).unwrap();
            crate::config_directives::cfg_exec(
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                b"exec",
                true,
                command.as_bytes(),
            );
            crate::config_directives::cfg_exec(
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                b"exec",
                false,
                b"",
            );
            crate::config_directives::cfg_exec(
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                b"exec_always",
                false,
                b"always",
            );
            let detached = take_first_autostart(false).unwrap();
            assert_eq!(detached.command.as_slice(), &[0xff, b'x']);
            assert_eq!(
                Autostart::new(b"prefix\0ignored".to_vec(), false).command,
                b"prefix"
            );
            assert!(detached.no_startup_id);
            assert!(detached.autostarts.tqe_next.is_null());
            assert!(detached.autostarts.tqe_prev.is_null());
            assert_eq!(
                (*autostarts.tqh_first).autostarts.tqe_prev,
                &raw mut autostarts.tqh_first
            );
            // A callback can append another command while the detached owner's bytes stay live.
            crate::config_directives::cfg_exec(
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                b"exec",
                false,
                b"later",
            );
            assert_eq!(detached.command.as_slice(), command.as_bytes());
            let empty = take_first_autostart(false).unwrap();
            assert!(empty.command.as_slice().is_empty());
            assert!(!empty.no_startup_id);
            assert_eq!(take_first_autostart(false).unwrap().command, b"later");
            assert!(take_first_autostart(false).is_none());
            assert_eq!(autostarts.tqh_last, &raw mut autostarts.tqh_first);
            assert_eq!(take_first_autostart(true).unwrap().command, b"always");
            assert!(take_first_autostart(true).is_none());
            assert_eq!(
                autostarts_always.tqh_last,
                &raw mut autostarts_always.tqh_first
            );
        }
    }
}
