use crate::config::barconfigs;
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
use i3_common::safewrappers::sasprintf;
use std::ffi::{CStr, CString};
use std::io::Read;
use std::os::fd::{AsRawFd, FromRawFd};
use std::ptr::NonNull;
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

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Binding {
    pub input_type: InputType,
    pub release: BindingRelease,
    pub border: bool,
    pub whole_window: bool,
    pub exclude_titlebar: bool,
    pub keycode: u32,
    pub event_state_mask: i3_event_state_mask_t,
    pub symbol: *mut ::core::ffi::c_char,
    pub keycodes_head: keycodes_head,
    pub command: *mut ::core::ffi::c_char,
    pub bindings: C2RustUnnamed_13,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
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
    pub keycode: xcb_keycode_t,
    pub modifiers: i3_event_state_mask_t,
    pub keycodes: C2RustUnnamed_14,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub tqe_next: *mut Binding_Keycode,
    pub tqe_prev: *mut *mut Binding_Keycode,
}
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

pub type hide_edge_borders_mode_t = ::core::ffi::c_uint;
pub const HEBM_SMART_NO_GAPS: hide_edge_borders_mode_t = 64;
pub const HEBM_SMART: hide_edge_borders_mode_t = 32;
pub const HEBM_BOTH: hide_edge_borders_mode_t = 23;
pub const HEBM_HORIZONTAL: hide_edge_borders_mode_t = 20;
pub const HEBM_VERTICAL: hide_edge_borders_mode_t = 3;
pub const HEBM_NONE: hide_edge_borders_mode_t = 0;

pub type gaps_mask_t = ::core::ffi::c_uint;
pub const GAPS_OUTER: gaps_mask_t = 30;
pub const GAPS_HORIZONTAL: gaps_mask_t = 20;
pub const GAPS_VERTICAL: gaps_mask_t = 10;
pub const GAPS_LEFT: gaps_mask_t = 16;
pub const GAPS_BOTTOM: gaps_mask_t = 8;
pub const GAPS_RIGHT: gaps_mask_t = 4;
pub const GAPS_TOP: gaps_mask_t = 2;
pub const GAPS_INNER: gaps_mask_t = 1;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Workspace_Assignment {
    pub name: *mut ::core::ffi::c_char,
    pub output: *mut ::core::ffi::c_char,
    pub gaps: gaps_t,
    pub gaps_mask: gaps_mask_t,
    pub ws_assignments: C2RustUnnamed_35,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_35 {
    pub tqe_next: *mut Workspace_Assignment,
    pub tqe_prev: *mut *mut Workspace_Assignment,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ignore_event_sequence_t {
    pub value: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Autostart {
    pub command: *mut ::core::ffi::c_char,
    pub no_startup_id: bool,
    pub autostarts: C2RustUnnamed_37,
    pub autostarts_always: C2RustUnnamed_36,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_36 {
    pub tqe_next: *mut Autostart,
    pub tqe_prev: *mut *mut Autostart,
}
pub use crate::application::C2RustUnnamed_36 as C2RustUnnamed_37;

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
pub type tiling_drag_t = ::core::ffi::c_uint;
pub const TILING_DRAG_MODIFIER_OR_TITLEBAR: tiling_drag_t = 3;
pub const TILING_DRAG_TITLEBAR: tiling_drag_t = 2;
pub const TILING_DRAG_MODIFIER: tiling_drag_t = 1;
pub const TILING_DRAG_OFF: tiling_drag_t = 0;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Barconfig {
    pub id: *mut ::core::ffi::c_char,
    pub num_outputs: ::core::ffi::c_int,
    pub outputs: *mut *mut ::core::ffi::c_char,
    pub tray_outputs: tray_outputs_head,
    pub tray_padding: ::core::ffi::c_int,
    pub socket_path: *mut ::core::ffi::c_char,
    pub mode: BarMode,
    pub hidden_state: BarHiddenState,
    pub modifier: u32,
    pub bar_bindings: bar_bindings_head,
    pub position: BarPosition,
    pub i3bar_command: *mut ::core::ffi::c_char,
    pub status_command: *mut ::core::ffi::c_char,
    pub workspace_command: *mut ::core::ffi::c_char,
    pub font: *mut ::core::ffi::c_char,
    pub separator_symbol: *mut ::core::ffi::c_char,
    pub hide_workspace_buttons: bool,
    pub workspace_min_width: ::core::ffi::c_int,
    pub strip_workspace_numbers: bool,
    pub strip_workspace_name: bool,
    pub hide_binding_mode_indicator: bool,
    pub verbose: bool,
    pub bar_height: u32,
    pub padding: Rect,
    pub colors: bar_colors,
    pub configs: C2RustUnnamed_42,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_42 {
    pub tqe_next: *mut Barconfig,
    pub tqe_prev: *mut *mut Barconfig,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bar_colors {
    pub background: *mut ::core::ffi::c_char,
    pub statusline: *mut ::core::ffi::c_char,
    pub separator: *mut ::core::ffi::c_char,
    pub focused_background: *mut ::core::ffi::c_char,
    pub focused_statusline: *mut ::core::ffi::c_char,
    pub focused_separator: *mut ::core::ffi::c_char,
    pub focused_workspace_border: *mut ::core::ffi::c_char,
    pub focused_workspace_bg: *mut ::core::ffi::c_char,
    pub focused_workspace_text: *mut ::core::ffi::c_char,
    pub active_workspace_border: *mut ::core::ffi::c_char,
    pub active_workspace_bg: *mut ::core::ffi::c_char,
    pub active_workspace_text: *mut ::core::ffi::c_char,
    pub inactive_workspace_border: *mut ::core::ffi::c_char,
    pub inactive_workspace_bg: *mut ::core::ffi::c_char,
    pub inactive_workspace_text: *mut ::core::ffi::c_char,
    pub urgent_workspace_border: *mut ::core::ffi::c_char,
    pub urgent_workspace_bg: *mut ::core::ffi::c_char,
    pub urgent_workspace_text: *mut ::core::ffi::c_char,
    pub binding_mode_border: *mut ::core::ffi::c_char,
    pub binding_mode_bg: *mut ::core::ffi::c_char,
    pub binding_mode_text: *mut ::core::ffi::c_char,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct bar_bindings_head {
    pub tqh_first: *mut Barbinding,
    pub tqh_last: *mut *mut Barbinding,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Barbinding {
    pub input_code: ::core::ffi::c_int,
    pub command: *mut ::core::ffi::c_char,
    pub release: bool,
    pub bindings: C2RustUnnamed_44,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_44 {
    pub tqe_next: *mut Barbinding,
    pub tqe_prev: *mut *mut Barbinding,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct tray_outputs_head {
    pub tqh_first: *mut tray_output_t,
    pub tqh_last: *mut *mut tray_output_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tray_output_t {
    pub output: *mut ::core::ffi::c_char,
    pub tray_outputs: C2RustUnnamed_47,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_47 {
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
    pub fd: libc::c_int,
    pub generation: u64,
    pub fd_owner: Option<std::os::fd::OwnedFd>,
    pub events: Vec<std::ffi::CString>,
    pub first_tick_sent: bool,
    pub read_callback: *mut ev_io,
    pub write_callback: *mut ev_io,
    pub timeout: *mut ev_timer,
    pub buffer: Vec<u8>,
    pub clients: C2RustUnnamed_48,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_48 {
    pub tqe_next: *mut ipc_client,
    pub tqe_prev: *mut *mut ipc_client,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct outputs_head {
    pub tqh_first: *mut xoutput,
    pub tqh_last: *mut *mut xoutput,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_49 {
    pub message: xcb_client_message_event_t,
    pub storage: [::core::ffi::c_char; 32],
}
pub const I3_VERSION: &[u8] = concat!(env!("I3_VERSION"), "\0").as_bytes();
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const SIGILL: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SIGABRT: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const SIGFPE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const SIGSEGV: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const SIGQUIT: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const SIGPIPE: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const SIGCHLD: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const SA_SIGINFO: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SA_NODEFER: ::core::ffi::c_int = 0x40000000 as ::core::ffi::c_int;
pub const SA_RESETHAND: ::core::ffi::c_uint = 0x80000000 as ::core::ffi::c_uint;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();

pub const RLIM_INFINITY: ::core::ffi::c_ulonglong = 0xffffffffffffffff as ::core::ffi::c_ulonglong;
pub const I3_IPC_MESSAGE_TYPE_RUN_COMMAND: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const I3_IPC_REPLY_TYPE_COMMAND: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const F_GETFD: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const F_SETFD: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const FD_CLOEXEC: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const __LC_ALL: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const LC_ALL: ::core::ffi::c_int = __LC_ALL;
pub const PF_LOCAL: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const AF_LOCAL: ::core::ffi::c_int = PF_LOCAL;
pub const SD_LISTEN_FDS_START: ::core::ffi::c_int = 3 as ::core::ffi::c_int;

pub static mut original_rlimit_core: rlimit = rlimit {
    rlim_cur: 0,
    rlim_max: 0,
};

pub static mut listen_fds: ::core::ffi::c_int = 0;
static mut xcb_prepare: *mut ev_prepare = ::core::ptr::null::<ev_prepare>() as *mut ev_prepare;

pub static mut start_argv: *mut *mut ::core::ffi::c_char =
    ::core::ptr::null::<*mut ::core::ffi::c_char>() as *mut *mut ::core::ffi::c_char;

pub static mut conn: *mut xcb_connection_t =
    ::core::ptr::null::<xcb_connection_t>() as *mut xcb_connection_t;

pub static mut conn_screen: ::core::ffi::c_int = 0;

pub static mut sndisplay: *mut SnDisplay = ::core::ptr::null::<SnDisplay>() as *mut SnDisplay;

pub static mut last_timestamp: xcb_timestamp_t = XCB_CURRENT_TIME as xcb_timestamp_t;

pub static mut root_screen: *mut xcb_screen_t =
    ::core::ptr::null::<xcb_screen_t>() as *mut xcb_screen_t;

pub static mut root: xcb_window_t = 0;

pub static mut wm_sn_selection_owner: xcb_window_t = 0;

pub static mut wm_sn: xcb_atom_t = 0;

pub static mut root_depth: u8 = 0;

pub static mut visual_type: *mut xcb_visualtype_t =
    ::core::ptr::null::<xcb_visualtype_t>() as *mut xcb_visualtype_t;

pub static mut colormap: xcb_colormap_t = 0;

pub static mut main_loop: *mut ev_loop = ::core::ptr::null::<ev_loop>() as *mut ev_loop;

pub static mut keysyms: *mut xcb_key_symbols_t =
    ::core::ptr::null::<xcb_key_symbols_t>() as *mut xcb_key_symbols_t;

pub static mut default_shmlog_size: ::core::ffi::c_int =
    25 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int;

pub static mut bindings: *mut bindings_head =
    ::core::ptr::null::<bindings_head>() as *mut bindings_head;

pub static mut current_binding_mode: *const ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>();

pub static mut autostarts: autostarts_head = autostarts_head {
    tqh_first: ::core::ptr::null::<Autostart>() as *mut Autostart,
    tqh_last: ::core::ptr::null::<*mut Autostart>() as *mut *mut Autostart,
};

pub static mut autostarts_always: autostarts_always_head = autostarts_always_head {
    tqh_first: ::core::ptr::null::<Autostart>() as *mut Autostart,
    tqh_last: ::core::ptr::null::<*mut Autostart>() as *mut *mut Autostart,
};

pub static mut assignments: assignments_head = assignments_head {
    tqh_first: ::core::ptr::null::<Assignment>() as *mut Assignment,
    tqh_last: ::core::ptr::null::<*mut Assignment>() as *mut *mut Assignment,
};

pub static mut ws_assignments: ws_assignments_head = ws_assignments_head {
    tqh_first: ::core::ptr::null::<Workspace_Assignment>() as *mut Workspace_Assignment,
    tqh_last: ::core::ptr::null::<*mut Workspace_Assignment>() as *mut *mut Workspace_Assignment,
};

pub static mut xkb_supported: bool = true_0 != 0;

pub static mut shape_supported: bool = true_0 != 0;

pub static mut force_xinerama: bool = false_0 != 0;

pub static mut A__NET_WM_STATE_MAXIMIZED_HORZ: xcb_atom_t = 0;

pub static mut A__NET_WM_STATE_FOCUSED: xcb_atom_t = 0;

pub static mut A__NET_WM_STATE_MAXIMIZED_VERT: xcb_atom_t = 0;

pub static mut A__NET_WM_STATE: xcb_atom_t = 0;

pub static mut A__NET_WM_WINDOW_TYPE: xcb_atom_t = 0;

pub static mut A__NET_WM_WINDOW_TYPE_NORMAL: xcb_atom_t = 0;

pub static mut A__NET_WM_WINDOW_TYPE_DOCK: xcb_atom_t = 0;

pub static mut A__NET_WM_WINDOW_TYPE_DIALOG: xcb_atom_t = 0;

pub static mut A__NET_WM_WINDOW_TYPE_UTILITY: xcb_atom_t = 0;

pub static mut A__NET_WM_WINDOW_TYPE_TOOLBAR: xcb_atom_t = 0;

pub static mut A__NET_WM_WINDOW_TYPE_SPLASH: xcb_atom_t = 0;

pub static mut A__NET_WM_WINDOW_TYPE_MENU: xcb_atom_t = 0;

pub static mut A__NET_WM_WINDOW_TYPE_DROPDOWN_MENU: xcb_atom_t = 0;

pub static mut A__NET_WM_WINDOW_TYPE_POPUP_MENU: xcb_atom_t = 0;

pub static mut A__NET_WM_WINDOW_TYPE_TOOLTIP: xcb_atom_t = 0;

pub static mut A__NET_WM_WINDOW_TYPE_NOTIFICATION: xcb_atom_t = 0;

pub static mut A__NET_WM_STATE_HIDDEN: xcb_atom_t = 0;

pub static mut A__NET_WM_STRUT_PARTIAL: xcb_atom_t = 0;

pub static mut A__NET_CLIENT_LIST: xcb_atom_t = 0;

pub static mut A__NET_CLIENT_LIST_STACKING: xcb_atom_t = 0;

pub static mut A__NET_CURRENT_DESKTOP: xcb_atom_t = 0;

pub static mut A__NET_NUMBER_OF_DESKTOPS: xcb_atom_t = 0;

pub static mut A__NET_DESKTOP_NAMES: xcb_atom_t = 0;

pub static mut A__NET_DESKTOP_VIEWPORT: xcb_atom_t = 0;

pub static mut A__NET_ACTIVE_WINDOW: xcb_atom_t = 0;

pub static mut A__NET_CLOSE_WINDOW: xcb_atom_t = 0;

pub static mut A__NET_MOVERESIZE_WINDOW: xcb_atom_t = 0;

pub static mut A__NET_WM_STATE_STICKY: xcb_atom_t = 0;

pub static mut A__NET_SUPPORTED: xcb_atom_t = 0;

pub static mut A__NET_SUPPORTING_WM_CHECK: xcb_atom_t = 0;

pub static mut A__NET_WM_NAME: xcb_atom_t = 0;

pub static mut A__NET_WM_VISIBLE_NAME: xcb_atom_t = 0;

pub static mut A__NET_WM_STATE_FULLSCREEN: xcb_atom_t = 0;

pub static mut A__NET_WM_MOVERESIZE: xcb_atom_t = 0;

pub static mut A__NET_WM_STATE_DEMANDS_ATTENTION: xcb_atom_t = 0;

pub static mut A__NET_WM_STATE_MODAL: xcb_atom_t = 0;

pub static mut A__NET_WM_DESKTOP: xcb_atom_t = 0;

pub static mut A_WM_TAKE_FOCUS: xcb_atom_t = 0;

pub static mut A_I3_CONFIG_PATH: xcb_atom_t = 0;

pub static mut A__NET_WM_USER_TIME: xcb_atom_t = 0;

pub static mut A__NET_STARTUP_ID: xcb_atom_t = 0;

pub static mut A__NET_WORKAREA: xcb_atom_t = 0;

pub static mut A__NET_WM_ICON: xcb_atom_t = 0;

pub static mut A_WM_PROTOCOLS: xcb_atom_t = 0;

pub static mut A_WM_DELETE_WINDOW: xcb_atom_t = 0;

pub static mut A_UTF8_STRING: xcb_atom_t = 0;

pub static mut A_WM_STATE: xcb_atom_t = 0;

pub static mut A_WM_CLIENT_LEADER: xcb_atom_t = 0;

pub static mut A_WM_WINDOW_ROLE: xcb_atom_t = 0;

pub static mut A_I3_SOCKET_PATH: xcb_atom_t = 0;

pub static mut A_I3_SYNC: xcb_atom_t = 0;

pub static mut A__NET_FRAME_EXTENTS: xcb_atom_t = 0;

pub static mut A_I3_FLOATING_WINDOW: xcb_atom_t = 0;

pub static mut A_I3_LOG_STREAM_SOCKET_PATH: xcb_atom_t = 0;

pub static mut A_I3_SHMLOG_PATH: xcb_atom_t = 0;

pub static mut A_I3_PID: xcb_atom_t = 0;

pub static mut A__NET_REQUEST_FRAME_EXTENTS: xcb_atom_t = 0;

pub static mut A__MOTIF_WM_HINTS: xcb_atom_t = 0;

pub static mut A_WM_CHANGE_STATE: xcb_atom_t = 0;

pub static mut A_MANAGER: xcb_atom_t = 0;
// Native malloc ownership used only by normal startup/event callbacks, never
// added to the actual signal handler. Global display resources retain atexit ABI.
struct NativeAllocation<T>(NonNull<T>);
impl<T> NativeAllocation<T> {
    unsafe fn take(ptr: *mut T) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }
    fn raw(&self) -> *mut T {
        self.0.as_ptr()
    }
}
impl<T> Drop for NativeAllocation<T> {
    fn drop(&mut self) {
        unsafe {
            free(self.raw().cast());
        }
    }
}
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
fn bar_command(program: &CStr, verbose: bool, id: &CStr, socket: &CStr) -> CString {
    let mut bytes = program.to_bytes().to_vec();
    bytes.push(b' ');
    if verbose {
        bytes.extend_from_slice(b"-V");
    }
    bytes.extend_from_slice(b" --bar_id=");
    bytes.extend_from_slice(id.to_bytes());
    bytes.extend_from_slice(b" --socket=\"");
    bytes.extend_from_slice(socket.to_bytes());
    bytes.push(b'"');
    CString::new(bytes).unwrap()
}
fn owned_dirname(path: &CStr) -> CString {
    let mut bytes = path.to_bytes_with_nul().to_vec();
    unsafe { CStr::from_ptr(dirname(bytes.as_mut_ptr().cast())) }.to_owned()
}
unsafe extern "C" fn xcb_got_event(
    loop_0: *mut ev_loop,
    w: *mut ev_io,
    revents: ::core::ffi::c_int,
) {
}
unsafe extern "C" fn xcb_prepare_cb(
    loop_0: *mut ev_loop,
    w: *mut ev_prepare,
    revents: ::core::ffi::c_int,
) {
    let mut event: *mut xcb_generic_event_t = ::core::ptr::null_mut::<xcb_generic_event_t>();
    loop {
        let Some(owner) = NativeAllocation::take(xcb_poll_for_event(conn)) else {
            break;
        };
        event = owner.raw();
        if (*event).response_type as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            if event_is_ignored(
                ignore_event_sequence_t {
                    value: (*event).full_sequence,
                },
                0 as ::core::ffi::c_int,
            ) {
                debuglog(
                    c"%s:%s:%d - Expected X11 Error received for sequence %x\n"
                        .as_ptr()
                        .cast_mut(),
                    c"../src/main.c".as_ptr(),
                    c"xcb_prepare_cb".as_ptr(),
                    137 as ::core::ffi::c_int,
                    (*event).sequence as ::core::ffi::c_int,
                );
            } else {
                let error: *mut xcb_generic_error_t = event as *mut xcb_generic_error_t;
                debuglog(
                    c"%s:%s:%d - X11 Error received (probably harmless)! sequence 0x%x, error_code = %d\n".as_ptr().cast_mut(),
                    c"../src/main.c".as_ptr(),
                    c"xcb_prepare_cb".as_ptr(),
                    141 as ::core::ffi::c_int,
                    (*error).sequence as ::core::ffi::c_int,
                    (*error).error_code as ::core::ffi::c_int,
                );
            }
        } else {
            let type_0: ::core::ffi::c_int =
                (*event).response_type as ::core::ffi::c_int & 0x7f as ::core::ffi::c_int;
            handle_event(type_0, event);
        }
    }
    xcb_flush(conn);
}

pub unsafe fn main_set_x11_cb(enable: bool) {
    debuglog(
        c"%s:%s:%d - Setting main X11 callback to enabled=%d\n"
            .as_ptr()
            .cast_mut(),
        c"../src/main.c".as_ptr(),
        c"main_set_x11_cb".as_ptr(),
        165 as ::core::ffi::c_int,
        enable as ::core::ffi::c_int,
    );
    if enable {
        ev_prepare_start(main_loop, xcb_prepare);
        ev_feed_event(
            main_loop,
            xcb_prepare as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
        );
    } else {
        ev_prepare_stop(main_loop, xcb_prepare);
    };
}
extern "C" fn i3_exit() {
    unsafe {
        if *shmlogname as ::core::ffi::c_int != '\0' as i32 {
            fprintf(stderr, c"Closing SHM log \"%s\"\n".as_ptr(), shmlogname);
            fflush(stderr);
            shm_unlink(shmlogname);
        }
        ipc_shutdown(ShutdownReason::Exit, -(1 as ::core::ffi::c_int));
        unlink(config.ipc_socket_path);
        if !current_log_stream_socket_path.is_null() {
            unlink(current_log_stream_socket_path);
        }
        xcb_disconnect(conn);
        kill_nagbar(config_error_nagbar_pid, false_0 != 0);
        kill_nagbar(command_error_nagbar_pid, false_0 != 0);
        ev_loop_destroy(main_loop);
    }
}
unsafe extern "C" fn handle_core_signal(
    sig: ::core::ffi::c_int,
    info: *mut siginfo_t,
    data: *mut ::core::ffi::c_void,
) {
    if *shmlogname as ::core::ffi::c_int != '\0' as i32 {
        shm_unlink(shmlogname);
    }
    raise(sig);
}
unsafe extern "C" fn handle_term_signal(
    loop_0: *mut ev_loop,
    signal_0: *mut ev_signal,
    revents: ::core::ffi::c_int,
) {
    exit(128 as ::core::ffi::c_int + (*signal_0).signum);
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
unsafe fn parse_restart_fd() -> ::core::ffi::c_int {
    let restart_fd: *const ::core::ffi::c_char = getenv(c"_I3_RESTART_FD".as_ptr());
    if restart_fd.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    let mut fd: ::core::ffi::c_long = -(1 as ::core::ffi::c_int) as ::core::ffi::c_long;
    if !parse_long(restart_fd, &raw mut fd, 10 as ::core::ffi::c_int) {
        errorlog(
            c"ERROR: Malformed _I3_RESTART_FD \"%s\"\n"
                .as_ptr()
                .cast_mut(),
            restart_fd,
        );
        return -(1 as ::core::ffi::c_int);
    }
    fd as ::core::ffi::c_int
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
unsafe fn run(mut arguments: Vec<std::ffi::OsString>) -> ::core::ffi::c_int {
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
    let mut override_owner: Option<CString> = None;
    let mut layout_owner: Option<CString> = None;
    let mut fake_owner: Option<CString> = None;
    let mut override_configpath: *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut autostart: bool = true_0 != 0;
    let mut layout_path: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut delete_layout_path: bool = false_0 != 0;
    let mut disable_randr15: bool = false_0 != 0;
    let mut fake_outputs: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut disable_signalhandler: bool = false_0 != 0;
    let mut only_check_config: bool = false_0 != 0;
    let mut replace_wm: bool = false_0 != 0;
    setlocale(LC_ALL, c"".as_ptr());
    getrlimit(libc::RLIMIT_CORE, &raw mut original_rlimit_core);
    if isatty(fileno(stdout)) == 0 {
        setbuf(stdout, ::core::ptr::null_mut::<::core::ffi::c_char>());
    }
    srand(time(::core::ptr::null_mut::<libc::time_t>()) as ::core::ffi::c_uint);
    init_logging();
    shmlog_size = if is_debug_build() as ::core::ffi::c_int != 0
        || !strstr(
            _restart_arguments.strings[0].as_ptr(),
            c"i3-with-shmlog".as_ptr(),
        )
        .is_null()
    {
        default_shmlog_size
    } else {
        0 as ::core::ffi::c_int
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
            b'a' => { verboselog(c"Autostart disabled using -a\n".as_ptr().cast_mut()); autostart = false; },
            b'L' | OPT_RESTART => {
                layout_owner=value.clone();
                layout_path=layout_owner.as_ref().unwrap().as_ptr().cast_mut();
                delete_layout_path = flag == OPT_RESTART;
            },
            b'c' => { override_owner=value.clone(); override_configpath=override_owner.as_ref().unwrap().as_ptr().cast_mut(); },
            b'C' => { verboselog(c"Checking configuration file only (-C)\n".as_ptr().cast_mut()); only_check_config = true; },
            b'v' => { printf(c"i3 version %s © 2009 Michael Stapelberg and contributors\n".as_ptr(), i3_version); return EXIT_SUCCESS; },
            b'm' => { printf(c"Binary i3 version:  %s © 2009 Michael Stapelberg and contributors\n".as_ptr(), i3_version); return display_running_version(); },
            b'V' => set_verbosity(true),
            b'd' => { verboselog(c"Enabling debug logging\n".as_ptr().cast_mut()); set_debug_logging(true); },
            b'l' => {},
            b'r' => replace_wm = true,
            b'h' => { print_usage(program, shmlog_size); return EXIT_SUCCESS; },
            OPT_XINERAMA => {
                force_xinerama = true;
                errorlog(c"ERROR: Using Xinerama instead of RandR. This option should be avoided at all cost because it does not refresh the list of screens, so you cannot configure displays at runtime. Please check if your driver really does not support RandR and disable this option as soon as you can.\n".as_ptr().cast_mut());
            },
            OPT_RANDR15 => disable_randr15 = true,
            OPT_SIGNAL => disable_signalhandler = true,
            OPT_SOCKET => {
                let socket = root_atom_contents(c"I3_SOCKET_PATH".as_ptr(), std::ptr::null_mut(), 0);
                if socket.is_null() { return EXIT_FAILURE; }
                printf(c"%s\n".as_ptr(), socket);
                free(socket.cast());
                return EXIT_SUCCESS;
            },
            OPT_SHMLOG => {
                shmlog_size = i3_common::cli::decimal_prefix(value.as_ref().unwrap().as_bytes());
                init_logging();
                verboselog(c"Limiting SHM log size to %d bytes\n".as_ptr().cast_mut(), shmlog_size);
            },
            OPT_FAKE_OUTPUTS => {
                let value = value.as_ref().unwrap();
                verboselog(c"Initializing fake outputs: %s\n".as_ptr().cast_mut(), value.as_ptr());
                fake_owner=Some(value.clone());
                fake_outputs=fake_owner.as_ref().unwrap().as_ptr().cast_mut();
            },
            OPT_OLD_PARSER => errorlog(c"ERROR: You are passing --force-old-config-parser-v4.4-only, but that flag was removed by now.\n".as_ptr().cast_mut()),
            _ => unreachable!(),
        }
    }
    if only_check_config {
        exit(
            if load_configuration(override_configpath, ConfigLoad::Validate) as ::core::ffi::c_int
                != 0
            {
                EXIT_SUCCESS
            } else {
                EXIT_FAILURE
            },
        );
    }
    if !operands.is_empty() {
        set_verbosity(true_0 != 0);
        verboselog(
            c"Additional arguments passed. Sending them as a command to i3.\n"
                .as_ptr()
                .cast_mut(),
        );
        let payload_bytes = operands
            .iter()
            .map(|argument| argument.as_bytes())
            .collect::<Vec<_>>()
            .join(&b' ');
        let payload_owner =
            std::ffi::CString::new(payload_bytes).expect("OS arguments have no NUL bytes");
        let payload = payload_owner.as_ptr();
        debuglog(
            c"%s:%s:%d - Command is: %s (%zd bytes)\n"
                .as_ptr()
                .cast_mut(),
            c"../src/main.c".as_ptr(),
            c"main".as_ptr(),
            500 as ::core::ffi::c_int,
            payload,
            strlen(payload),
        );
        let mut socket_path_0: *mut ::core::ffi::c_char = root_atom_contents(
            c"I3_SOCKET_PATH".as_ptr(),
            ::core::ptr::null_mut::<xcb_connection_t>(),
            0 as ::core::ffi::c_int,
        );
        if socket_path_0.is_null() {
            errorlog(
                c"ERROR: Could not get i3 IPC socket path\n"
                    .as_ptr()
                    .cast_mut(),
            );
            return 1 as ::core::ffi::c_int;
        }
        let sockfd: ::core::ffi::c_int = socket(
            AF_LOCAL,
            libc::SOCK_STREAM as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        );
        if sockfd == -(1 as ::core::ffi::c_int) {
            err(EXIT_FAILURE, c"Could not create socket".as_ptr());
        }
        let socket_owner = std::os::fd::OwnedFd::from_raw_fd(sockfd);
        let mut addr: sockaddr_un = sockaddr_un {
            sun_family: 0 as libc::sa_family_t,
            sun_path: [0; 108],
        };
        addr.sun_family = AF_LOCAL as libc::sa_family_t;
        strncpy(
            &raw mut addr.sun_path as *mut ::core::ffi::c_char,
            socket_path_0,
            (::core::mem::size_of::<[::core::ffi::c_char; 108]>() as usize)
                .wrapping_sub(1 as usize),
        );
        free(socket_path_0 as *mut ::core::ffi::c_void);
        socket_path_0 = ::core::ptr::null_mut::<::core::ffi::c_char>();
        if connect(
            sockfd,
            &raw mut addr as *const sockaddr,
            ::core::mem::size_of::<sockaddr_un>() as libc::socklen_t,
        ) < 0 as ::core::ffi::c_int
        {
            err(EXIT_FAILURE, c"Could not connect to i3".as_ptr());
        }
        if ipc_send_message(
            sockfd,
            strlen(payload) as u32,
            I3_IPC_MESSAGE_TYPE_RUN_COMMAND as u32,
            payload as *mut u8,
        ) == -(1 as ::core::ffi::c_int)
        {
            err(EXIT_FAILURE, c"IPC: write()".as_ptr());
        }
        let mut reply_length: u32 = 0;
        let mut reply_type: u32 = 0;
        let mut reply: *mut u8 = ::core::ptr::null_mut::<u8>();
        let mut ret: ::core::ffi::c_int = 0;
        ret = ipc_recv_message(
            sockfd,
            &raw mut reply_type,
            &raw mut reply_length,
            &raw mut reply,
        );
        let reply_owner = NativeAllocation::take(reply);
        if ret != 0 as ::core::ffi::c_int {
            if ret == -(1 as ::core::ffi::c_int) {
                err(EXIT_FAILURE, c"IPC: read()".as_ptr());
            }
            return 1 as ::core::ffi::c_int;
        }
        if reply_type != I3_IPC_REPLY_TYPE_COMMAND as u32 {
            errx(
                EXIT_FAILURE,
                c"IPC: received reply of type %d but expected %d (COMMAND)".as_ptr(),
                reply_type,
                I3_IPC_REPLY_TYPE_COMMAND,
            );
        }
        printf(
            c"%.*s\n".as_ptr(),
            reply_length,
            reply as *mut ::core::ffi::c_char,
        );
        drop(reply_owner);
        reply = ::core::ptr::null_mut::<u8>();
        drop(socket_owner);
        return 0 as ::core::ffi::c_int;
    }
    init_logging();
    if is_debug_build() {
        let mut limit: rlimit = rlimit {
            rlim_cur: RLIM_INFINITY as libc::rlim_t,
            rlim_max: RLIM_INFINITY as libc::rlim_t,
        };
        setrlimit(libc::RLIMIT_CORE, &raw mut limit);
        verboselog(
            c"CORE DUMPS: You are running a development version of i3, so coredumps were automatically enabled (ulimit -c unlimited).\n".as_ptr().cast_mut(),
        );
        let cwd: *mut ::core::ffi::c_char =
            getcwd(::core::ptr::null_mut::<::core::ffi::c_char>(), 0 as usize);
        if !cwd.is_null() {
            verboselog(
                c"CORE DUMPS: Your current working directory is \"%s\".\n"
                    .as_ptr()
                    .cast_mut(),
                cwd,
            );
            free(cwd as *mut ::core::ffi::c_void);
        }
        if let Ok(mut file) = std::fs::File::open("/proc/sys/kernel/core_pattern") {
            let mut buffer = [0u8; 1024];
            if file.read(&mut buffer[..1023]).is_ok_and(|count| count > 0) {
                verboselog(
                    c"CORE DUMPS: Your core_pattern is: %s".as_ptr().cast_mut(),
                    buffer.as_ptr().cast::<libc::c_char>(),
                );
            }
        }
    }
    verboselog(c"i3 %s starting\n".as_ptr().cast_mut(), _i3_version);
    conn = xcb_connect(
        ::core::ptr::null::<::core::ffi::c_char>(),
        &raw mut conn_screen,
    );
    if xcb_connection_has_error(conn) != 0 {
        errx(EXIT_FAILURE, c"Cannot open display".as_ptr());
    }
    sndisplay = sn_xcb_display_new(conn, None, None);
    main_loop = ev_default_loop(0 as ::core::ffi::c_uint);
    if main_loop.is_null() {
        errx(
            EXIT_FAILURE,
            c"Could not initialize libev. Bad LIBEV_FLAGS?\n".as_ptr(),
        );
    }
    root_screen = xcb_aux_get_screen(conn, conn_screen);
    root = (*root_screen).root;
    xcb_prefetch_extension_data(conn, &raw mut xcb_xkb_id);
    xcb_prefetch_extension_data(conn, &raw mut xcb_shape_id);
    xcb_prefetch_extension_data(conn, &raw mut xcb_big_requests_id);
    if force_xinerama {
        xcb_prefetch_extension_data(conn, &raw mut xcb_xinerama_id);
    } else {
        xcb_prefetch_extension_data(conn, &raw mut xcb_randr_id);
    }
    let mut fresh6: [u32; 1] = [XCB_EVENT_MASK_PROPERTY_CHANGE as ::core::ffi::c_int as u32];
    xcb_change_window_attributes(
        conn,
        root,
        XCB_CW_EVENT_MASK as ::core::ffi::c_int as u32,
        &raw mut fresh6 as *mut u32 as *const ::core::ffi::c_void,
    );
    xcb_change_property(
        conn,
        XCB_PROP_MODE_APPEND as ::core::ffi::c_int as u8,
        root,
        XCB_ATOM_SUPERSCRIPT_X as ::core::ffi::c_int as xcb_atom_t,
        XCB_ATOM_CARDINAL as ::core::ffi::c_int as xcb_atom_t,
        32 as u8,
        0 as u32,
        c"".as_ptr() as *const ::core::ffi::c_void,
    );
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
        .map(|(name, _)| xcb_intern_atom(conn, 0, name.to_bytes().len() as u16, name.as_ptr()))
        .collect();
    root_depth = (*root_screen).root_depth;
    colormap = (*root_screen).default_colormap;
    visual_type =
        xcb_aux_find_visual_by_attrs(root_screen, -(1 as ::core::ffi::c_int) as i8, 32 as i8);
    if !visual_type.is_null() {
        root_depth = xcb_aux_get_depth_of_visual(root_screen, (*visual_type).visual_id);
        colormap = xcb_generate_id(conn) as xcb_colormap_t;
        let cm_cookie: xcb_void_cookie_t = xcb_create_colormap_checked(
            conn,
            XCB_COLORMAP_ALLOC_NONE as ::core::ffi::c_int as u8,
            colormap,
            root,
            (*visual_type).visual_id,
        );
        let error: *mut xcb_generic_error_t = xcb_request_check(conn, cm_cookie);
        if !error.is_null() {
            errorlog(
                c"ERROR: Could not create colormap. Error code: %d\n"
                    .as_ptr()
                    .cast_mut(),
                (*error).error_code as ::core::ffi::c_int,
            );
            exit(EXIT_FAILURE);
        }
    } else {
        visual_type = get_visualtype(root_screen);
    }
    xcb_prefetch_maximum_request_length(conn);
    init_dpi();
    debuglog(
        c"%s:%s:%d - root_depth = %d, visual_id = 0x%08x.\n"
            .as_ptr()
            .cast_mut(),
        c"../src/main.c".as_ptr(),
        c"main".as_ptr(),
        648 as ::core::ffi::c_int,
        root_depth as ::core::ffi::c_int,
        (*visual_type).visual_id,
    );
    debuglog(
        c"%s:%s:%d - root_screen->height_in_pixels = %d, root_screen->height_in_millimeters = %d\n"
            .as_ptr()
            .cast_mut(),
        c"../src/main.c".as_ptr(),
        c"main".as_ptr(),
        650 as ::core::ffi::c_int,
        (*root_screen).height_in_pixels as ::core::ffi::c_int,
        (*root_screen).height_in_millimeters as ::core::ffi::c_int,
    );
    debuglog(
        c"%s:%s:%d - One logical pixel corresponds to %d physical pixels on this display.\n"
            .as_ptr()
            .cast_mut(),
        c"../src/main.c".as_ptr(),
        c"main".as_ptr(),
        651 as ::core::ffi::c_int,
        logical_px(1 as ::core::ffi::c_int),
    );
    let gcookie: xcb_get_geometry_cookie_t = xcb_get_geometry(conn, root as xcb_drawable_t);
    let pointercookie: xcb_query_pointer_cookie_t = xcb_query_pointer(conn, root);
    xcb_flush(conn);
    let mut event: *mut xcb_generic_event_t = ::core::ptr::null_mut::<xcb_generic_event_t>();
    debuglog(
        c"%s:%s:%d - waiting for PropertyNotify event\n"
            .as_ptr()
            .cast_mut(),
        c"../src/main.c".as_ptr(),
        c"main".as_ptr(),
        660 as ::core::ffi::c_int,
    );
    loop {
        let Some(owner) = NativeAllocation::take(xcb_wait_for_event(conn)) else {
            break;
        };
        event = owner.raw();
        if (*event).response_type as ::core::ffi::c_int == XCB_PROPERTY_NOTIFY {
            last_timestamp = (*(event as *mut xcb_property_notify_event_t)).time;
            break;
        } else {
        }
    }
    debuglog(
        c"%s:%s:%d - got timestamp %d\n".as_ptr().cast_mut(),
        c"../src/main.c".as_ptr(),
        c"main".as_ptr(),
        669 as ::core::ffi::c_int,
        last_timestamp,
    );
    for index in [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
        48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59,
    ] {
        let (name, target) = atom_requests[index];
        let Some(reply) = NativeAllocation::take(xcb_intern_atom_reply(
            conn,
            atom_cookies[index],
            std::ptr::null_mut(),
        )) else {
            errorlog(
                c"ERROR: Could not get atom %s\n".as_ptr().cast_mut(),
                name.as_ptr(),
            );
            exit(-1);
        };
        target.write((*reply.raw()).atom);
    }
    load_configuration(override_configpath, ConfigLoad::Load);
    if config.ipc_socket_path.is_null() {
        config.ipc_socket_path = getenv(c"I3SOCK".as_ptr());
        if config.ipc_socket_path.is_null() {
            config.ipc_socket_path = get_process_filename(c"ipc-socket".as_ptr());
        } else {
            config.ipc_socket_path = sstrdup(config.ipc_socket_path);
        }
    }
    let ipc_socket: ::core::ffi::c_int =
        create_socket(config.ipc_socket_path, &raw mut current_socketpath);
    if ipc_socket == -(1 as ::core::ffi::c_int) {
        errx(
            EXIT_FAILURE,
            c"Could not create the IPC socket: %s".as_ptr(),
            config.ipc_socket_path,
        );
    }
    if config.force_xinerama {
        force_xinerama = true_0 != 0;
    }
    let atom_name: *mut ::core::ffi::c_char =
        xcb_atom_name_by_screen(c"WM".as_ptr(), conn_screen as u8);
    wm_sn_selection_owner = xcb_generate_id(conn) as xcb_window_t;
    if atom_name.is_null() {
        errorlog(
            c"ERROR: xcb_atom_name_by_screen(\"WM\", %d) failed, exiting\n"
                .as_ptr()
                .cast_mut(),
            conn_screen,
        );
        return 1 as ::core::ffi::c_int;
    }
    let mut atom_reply: *mut xcb_intern_atom_reply_t =
        ::core::ptr::null_mut::<xcb_intern_atom_reply_t>();
    atom_reply = xcb_intern_atom_reply(
        conn,
        xcb_intern_atom_unchecked(conn, 0 as u8, strlen(atom_name) as u16, atom_name),
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    free(atom_name as *mut ::core::ffi::c_void);
    if atom_reply.is_null() {
        errorlog(
            c"ERROR: Failed to intern the WM_Sn atom, exiting\n"
                .as_ptr()
                .cast_mut(),
        );
        return 1 as ::core::ffi::c_int;
    }
    wm_sn = (*atom_reply).atom;
    free(atom_reply as *mut ::core::ffi::c_void);
    let selection_reply: *mut xcb_get_selection_owner_reply_t = xcb_get_selection_owner_reply(
        conn,
        xcb_get_selection_owner(conn, wm_sn),
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    let selection_reply_owner = NativeAllocation::take(selection_reply);
    if !selection_reply.is_null()
        && (*selection_reply).owner as ::core::ffi::c_long != XCB_NONE
        && !replace_wm
    {
        errorlog(
            c"ERROR: Another window manager is already running (WM_Sn is owned)"
                .as_ptr()
                .cast_mut(),
        );
        return 1 as ::core::ffi::c_int;
    }
    xcb_create_window(
        conn,
        (*root_screen).root_depth,
        wm_sn_selection_owner,
        (*root_screen).root,
        -(1 as ::core::ffi::c_int) as i16,
        -(1 as ::core::ffi::c_int) as i16,
        1 as u16,
        1 as u16,
        0 as u16,
        XCB_WINDOW_CLASS_INPUT_OUTPUT as ::core::ffi::c_int as u16,
        (*root_screen).root_visual,
        0 as u32,
        ::core::ptr::null::<::core::ffi::c_void>(),
    );
    xcb_change_property(
        conn,
        XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as u8,
        wm_sn_selection_owner,
        XCB_ATOM_WM_CLASS as ::core::ffi::c_int as xcb_atom_t,
        XCB_ATOM_STRING as ::core::ffi::c_int as xcb_atom_t,
        8 as u8,
        strlen(c"i3-WM_Sn".as_ptr())
            .wrapping_add(1 as usize)
            .wrapping_mul(2 as usize) as u32,
        b"i3-WM_Sn\0i3-WM_Sn\0\0" as *const u8 as *const ::core::ffi::c_char
            as *const ::core::ffi::c_void,
    );
    xcb_set_selection_owner(conn, wm_sn_selection_owner, wm_sn, last_timestamp);
    if !selection_reply.is_null() && (*selection_reply).owner as ::core::ffi::c_long != XCB_NONE {
        let mut check_rounds: ::core::ffi::c_int = 150 as ::core::ffi::c_int;
        let mut geom_reply: *mut xcb_get_geometry_reply_t =
            ::core::ptr::null_mut::<xcb_get_geometry_reply_t>();
        debuglog(
            c"%s:%s:%d - waiting for old WM_Sn selection owner to exit"
                .as_ptr()
                .cast_mut(),
            c"../src/main.c".as_ptr(),
            c"main".as_ptr(),
            768 as ::core::ffi::c_int,
        );
        loop {
            let usleep_time: ::core::ffi::c_uint =
                100000 as ::core::ffi::c_int as ::core::ffi::c_uint;
            free(geom_reply as *mut ::core::ffi::c_void);
            usleep(usleep_time as libc::useconds_t);
            let fresh7 = check_rounds;
            check_rounds = check_rounds - 1;
            if fresh7 == 0 as ::core::ffi::c_int {
                errorlog(
                    c"ERROR: The old window manager is not exiting"
                        .as_ptr()
                        .cast_mut(),
                );
                return 1 as ::core::ffi::c_int;
            }
            geom_reply = xcb_get_geometry_reply(
                conn,
                xcb_get_geometry(conn, (*selection_reply).owner as xcb_drawable_t),
                ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
            );
            if geom_reply.is_null() {
                break;
            }
        }
    }
    drop(selection_reply_owner);
    let mut event_0: C2RustUnnamed_49 = C2RustUnnamed_49 {
        message: xcb_client_message_event_t {
            response_type: 0 as u8,
            format: 0,
            sequence: 0,
            window: 0,
            type_0: 0,
            data: xcb_client_message_data_t { data8: [0; 20] },
        },
    };
    event_0.message.response_type = XCB_CLIENT_MESSAGE as u8;
    event_0.message.window = (*root_screen).root;
    event_0.message.format = 32 as u8;
    event_0.message.type_0 = A_MANAGER;
    event_0.message.data.data32[0 as ::core::ffi::c_int as usize] = last_timestamp as u32;
    event_0.message.data.data32[1 as ::core::ffi::c_int as usize] = wm_sn as u32;
    event_0.message.data.data32[2 as ::core::ffi::c_int as usize] = wm_sn_selection_owner as u32;
    xcb_send_event(
        conn,
        0 as u8,
        (*root_screen).root,
        XCB_EVENT_MASK_STRUCTURE_NOTIFY as ::core::ffi::c_int as u32,
        &raw mut event_0.storage as *mut ::core::ffi::c_char,
    );
    let mut cookie: xcb_void_cookie_t = xcb_void_cookie_t { sequence: 0 };
    let mut fresh8: [u32; 1] = [(XCB_EVENT_MASK_SUBSTRUCTURE_REDIRECT as ::core::ffi::c_int
        | XCB_EVENT_MASK_BUTTON_PRESS as ::core::ffi::c_int
        | XCB_EVENT_MASK_STRUCTURE_NOTIFY as ::core::ffi::c_int
        | XCB_EVENT_MASK_POINTER_MOTION as ::core::ffi::c_int
        | XCB_EVENT_MASK_PROPERTY_CHANGE as ::core::ffi::c_int
        | XCB_EVENT_MASK_FOCUS_CHANGE as ::core::ffi::c_int
        | XCB_EVENT_MASK_ENTER_WINDOW as ::core::ffi::c_int)
        as u32];
    cookie = xcb_change_window_attributes_checked(
        conn,
        root,
        XCB_CW_EVENT_MASK as ::core::ffi::c_int as u32,
        &raw mut fresh8 as *mut u32 as *const ::core::ffi::c_void,
    );
    let error_0: *mut xcb_generic_error_t = xcb_request_check(conn, cookie);
    if !error_0.is_null() {
        errorlog(
            c"ERROR: Another window manager seems to be running (X error %d)\n"
                .as_ptr()
                .cast_mut(),
            (*error_0).error_code as ::core::ffi::c_int,
        );
        return 1 as ::core::ffi::c_int;
    }
    let greply: *mut xcb_get_geometry_reply_t = xcb_get_geometry_reply(
        conn,
        gcookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    let greply_owner = NativeAllocation::take(greply);
    if greply.is_null() {
        errorlog(
            c"ERROR: Could not get geometry of the root window, exiting\n"
                .as_ptr()
                .cast_mut(),
        );
        return 1 as ::core::ffi::c_int;
    }
    debuglog(
        c"%s:%s:%d - root geometry reply: (%d, %d) %d x %d\n"
            .as_ptr()
            .cast_mut(),
        c"../src/main.c".as_ptr(),
        c"main".as_ptr(),
        819 as ::core::ffi::c_int,
        (*greply).x as ::core::ffi::c_int,
        (*greply).y as ::core::ffi::c_int,
        (*greply).width as ::core::ffi::c_int,
        (*greply).height as ::core::ffi::c_int,
    );
    xcursor_load_cursors();
    xcursor_set_root_cursor(Cursor::Pointer);
    let mut extreply: *const xcb_query_extension_reply_t =
        ::core::ptr::null::<xcb_query_extension_reply_t>();
    extreply =
        xcb_get_extension_data(conn, &raw mut xcb_xkb_id) as *const xcb_query_extension_reply_t;
    xkb_supported = (*extreply).present != 0;
    if (*extreply).present == 0 {
        debuglog(
            c"%s:%s:%d - xkb is not present on this server\n"
                .as_ptr()
                .cast_mut(),
            c"../src/main.c".as_ptr(),
            c"main".as_ptr(),
            831 as ::core::ffi::c_int,
        );
    } else {
        debuglog(
            c"%s:%s:%d - initializing xcb-xkb\n".as_ptr().cast_mut(),
            c"../src/main.c".as_ptr(),
            c"main".as_ptr(),
            833 as ::core::ffi::c_int,
        );
        xcb_xkb_use_extension(
            conn,
            XCB_XKB_MAJOR_VERSION as u16,
            XCB_XKB_MINOR_VERSION as u16,
        );
        xcb_xkb_select_events(
            conn,
            XCB_XKB_ID_USE_CORE_KBD as ::core::ffi::c_int as xcb_xkb_device_spec_t,
            (XCB_XKB_EVENT_TYPE_STATE_NOTIFY as ::core::ffi::c_int
                | XCB_XKB_EVENT_TYPE_MAP_NOTIFY as ::core::ffi::c_int
                | XCB_XKB_EVENT_TYPE_NEW_KEYBOARD_NOTIFY as ::core::ffi::c_int) as u16,
            0 as u16,
            (XCB_XKB_EVENT_TYPE_STATE_NOTIFY as ::core::ffi::c_int
                | XCB_XKB_EVENT_TYPE_MAP_NOTIFY as ::core::ffi::c_int
                | XCB_XKB_EVENT_TYPE_NEW_KEYBOARD_NOTIFY as ::core::ffi::c_int) as u16,
            0xff as u16,
            0xff as u16,
            ::core::ptr::null::<::core::ffi::c_void>(),
        );
        let mask: u32 = (XCB_XKB_PER_CLIENT_FLAG_GRABS_USE_XKB_STATE as ::core::ffi::c_int
            | XCB_XKB_PER_CLIENT_FLAG_LOOKUP_STATE_WHEN_GRABBED as ::core::ffi::c_int
            | XCB_XKB_PER_CLIENT_FLAG_DETECTABLE_AUTO_REPEAT as ::core::ffi::c_int)
            as u32;
        let mut pcf_reply: *mut xcb_xkb_per_client_flags_reply_t =
            ::core::ptr::null_mut::<xcb_xkb_per_client_flags_reply_t>();
        pcf_reply = xcb_xkb_per_client_flags_reply(
            conn,
            xcb_xkb_per_client_flags(
                conn,
                XCB_XKB_ID_USE_CORE_KBD as ::core::ffi::c_int as xcb_xkb_device_spec_t,
                mask,
                mask,
                0 as u32,
                0 as u32,
                0 as u32,
            ),
            ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
        );
        if pcf_reply.is_null()
            || (*pcf_reply).value
                & XCB_XKB_PER_CLIENT_FLAG_GRABS_USE_XKB_STATE as ::core::ffi::c_int as u32
                == 0
        {
            errorlog(
                c"ERROR: Could not set XCB_XKB_PER_CLIENT_FLAG_GRABS_USE_XKB_STATE\n"
                    .as_ptr()
                    .cast_mut(),
            );
        }
        if pcf_reply.is_null()
            || (*pcf_reply).value
                & XCB_XKB_PER_CLIENT_FLAG_LOOKUP_STATE_WHEN_GRABBED as ::core::ffi::c_int as u32
                == 0
        {
            errorlog(
                c"ERROR: Could not set XCB_XKB_PER_CLIENT_FLAG_LOOKUP_STATE_WHEN_GRABBED\n"
                    .as_ptr()
                    .cast_mut(),
            );
        }
        if pcf_reply.is_null()
            || (*pcf_reply).value
                & XCB_XKB_PER_CLIENT_FLAG_DETECTABLE_AUTO_REPEAT as ::core::ffi::c_int as u32
                == 0
        {
            errorlog(
                c"ERROR: Could not set XCB_XKB_PER_CLIENT_FLAG_DETECTABLE_AUTO_REPEAT\n"
                    .as_ptr()
                    .cast_mut(),
            );
        }
        free(pcf_reply as *mut ::core::ffi::c_void);
        xkb_base = (*extreply).first_event as ::core::ffi::c_int;
    }
    extreply =
        xcb_get_extension_data(conn, &raw mut xcb_shape_id) as *const xcb_query_extension_reply_t;
    if (*extreply).present != 0 {
        shape_base = (*extreply).first_event as ::core::ffi::c_int;
        let cookie_0: xcb_shape_query_version_cookie_t = xcb_shape_query_version(conn);
        let version: *mut xcb_shape_query_version_reply_t = xcb_shape_query_version_reply(
            conn,
            cookie_0,
            ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
        );
        shape_supported = !version.is_null()
            && (*version).minor_version as ::core::ffi::c_int >= 1 as ::core::ffi::c_int;
        free(version as *mut ::core::ffi::c_void);
    } else {
        shape_supported = false_0 != 0;
    }
    if !shape_supported {
        debuglog(
            c"%s:%s:%d - shape 1.1 is not present on this server\n"
                .as_ptr()
                .cast_mut(),
            c"../src/main.c".as_ptr(),
            c"main".as_ptr(),
            903 as ::core::ffi::c_int,
        );
    }
    restore_connect();
    property_handlers_init();
    ewmh_setup_hints();
    keysyms = xcb_key_symbols_alloc(conn);
    xcb_numlock_mask = aio_get_mod_mask_for(XCB_NUM_LOCK as u32, keysyms) as ::core::ffi::c_uint;
    if !load_keymap() {
        errx(EXIT_FAILURE, c"Could not load keymap\n".as_ptr());
    }
    translate_keysyms();
    grab_all_keys(conn);
    let mut needs_tree_init: bool = true_0 != 0;
    if !layout_path.is_null() {
        verboselog(
            c"Trying to restore the layout from \"%s\".\n"
                .as_ptr()
                .cast_mut(),
            layout_path,
        );
        needs_tree_init = !tree_restore(layout_path, greply);
        if delete_layout_path {
            unlink(layout_path);
            let dir = owned_dirname(CStr::from_ptr(layout_path));
            rmdir(dir.as_ptr());
        }
    }
    if needs_tree_init {
        tree_init(greply);
    }
    drop(greply_owner);
    let configured_fake = if fake_outputs.is_null() {
        NativeAllocation::take(config.fake_outputs)
    } else {
        None
    };
    if let Some(owner) = configured_fake.as_ref() {
        fake_outputs = owner.raw();
    }
    if !fake_outputs.is_null() {
        fake_outputs_init(fake_outputs);
        if fake_owner.is_some() {
            free(config.fake_outputs.cast());
        }
        config.fake_outputs = std::ptr::null_mut();
        drop(fake_owner.take());
        drop(configured_fake);
        fake_outputs = std::ptr::null_mut();
    } else if force_xinerama {
        xinerama_init();
    } else {
        debuglog(
            c"%s:%s:%d - Checking for XRandR...\n".as_ptr().cast_mut(),
            c"../src/main.c".as_ptr(),
            c"main".as_ptr(),
            956 as ::core::ffi::c_int,
        );
        randr_init(
            &raw mut randr_base,
            disable_randr15 as ::core::ffi::c_int != 0
                || config.disable_randr15 as ::core::ffi::c_int != 0,
        );
    }
    if !layout_path.is_null() && randr_base > -(1 as ::core::ffi::c_int) {
        let mut con: *mut Con = ::core::ptr::null_mut::<Con>();
        con = (*croot).nodes_head.tqh_first as *mut Con;
        while !con.is_null() {
            let mut output: *mut Output = ::core::ptr::null_mut::<Output>();
            output = outputs.tqh_first as *mut Output;
            while !output.is_null() {
                if !((*output).active as ::core::ffi::c_int != 0
                    || strcmp((*con).name, output_primary_name(output)) != 0 as ::core::ffi::c_int)
                {
                    if (*output).con.is_null() {
                        output_init_con(output);
                        (*output).changed = false_0 != 0;
                    }
                    (*output).to_be_disabled = true_0 != 0;
                    randr_disable_output(output);
                }
                output = (*output).outputs.tqe_next as *mut Output;
            }
            con = (*con).nodes.tqe_next as *mut Con;
        }
    }
    drop(layout_owner.take());
    layout_path = ::core::ptr::null_mut::<::core::ffi::c_char>();
    scratchpad_fix_resolution();
    let mut output_0: *mut Output = ::core::ptr::null_mut::<Output>();
    let pointer_reply: *mut xcb_query_pointer_reply_t = xcb_query_pointer_reply(
        conn,
        pointercookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    let pointer_reply_owner = NativeAllocation::take(pointer_reply);
    if pointer_reply.is_null() {
        errorlog(
            c"ERROR: Could not query pointer position, using first screen\n"
                .as_ptr()
                .cast_mut(),
        );
    } else {
        debuglog(
            c"%s:%s:%d - Pointer at %d, %d\n".as_ptr().cast_mut(),
            c"../src/main.c".as_ptr(),
            c"main".as_ptr(),
            995 as ::core::ffi::c_int,
            (*pointer_reply).root_x as ::core::ffi::c_int,
            (*pointer_reply).root_y as ::core::ffi::c_int,
        );
        output_0 = get_output_containing(
            (*pointer_reply).root_x as ::core::ffi::c_uint,
            (*pointer_reply).root_y as ::core::ffi::c_uint,
        );
        if output_0.is_null() {
            errorlog(
                c"ERROR: ERROR: No screen at (%d, %d), starting on the first screen\n"
                    .as_ptr()
                    .cast_mut(),
                (*pointer_reply).root_x as ::core::ffi::c_int,
                (*pointer_reply).root_y as ::core::ffi::c_int,
            );
        }
    }
    if output_0.is_null() {
        output_0 = get_first_output();
    }
    con_activate(con_descend_focused(output_get_content((*output_0).con)));
    drop(pointer_reply_owner);
    tree_render();
    let mut watchers = MainWatchers::new();
    let ipc_io = watchers.add(ipc_socket, ipc_new_client);
    let log_stream_socket_path: *mut ::core::ffi::c_char =
        get_process_filename(c"log-stream-socket".as_ptr());
    let log_socket: ::core::ffi::c_int = create_socket(
        log_stream_socket_path,
        &raw mut current_log_stream_socket_path,
    );
    free(log_stream_socket_path as *mut ::core::ffi::c_void);
    let mut log_io: *mut ev_io = ::core::ptr::null_mut::<ev_io>();
    if log_socket == -(1 as ::core::ffi::c_int) {
        errorlog(
            c"ERROR: Could not create the log socket, i3-dump-log -f will not work\n"
                .as_ptr()
                .cast_mut(),
        );
    } else {
        log_io = watchers.add(log_socket, log_new_client);
    }
    let mut socket_ipc_io: *mut ev_io = ::core::ptr::null_mut::<ev_io>();
    listen_fds = crate::sd_daemon::listen_fds();
    if listen_fds < 0 as ::core::ffi::c_int {
        errorlog(
            c"ERROR: socket activation: Error in sd_listen_fds\n"
                .as_ptr()
                .cast_mut(),
        );
    } else if listen_fds == 0 as ::core::ffi::c_int {
        debuglog(
            c"%s:%s:%d - socket activation: no sockets passed\n"
                .as_ptr()
                .cast_mut(),
            c"../src/main.c".as_ptr(),
            c"main".as_ptr(),
            1036 as ::core::ffi::c_int,
        );
    } else {
        let mut flags: ::core::ffi::c_int = 0;
        let mut fd: ::core::ffi::c_int = SD_LISTEN_FDS_START;
        while fd < SD_LISTEN_FDS_START + listen_fds {
            debuglog(
                c"%s:%s:%d - socket activation: also listening on fd %d\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/main.c".as_ptr(),
                c"main".as_ptr(),
                1042 as ::core::ffi::c_int,
                fd,
            );
            flags = fcntl(fd, F_GETFD);
            if flags < 0 as ::core::ffi::c_int
                || fcntl(fd, F_SETFD, flags & !FD_CLOEXEC) < 0 as ::core::ffi::c_int
            {
                errorlog(
                    c"ERROR: Could not disable FD_CLOEXEC on fd %d\n"
                        .as_ptr()
                        .cast_mut(),
                    fd,
                );
            }
            socket_ipc_io = watchers.add(fd, ipc_new_client);
            fd += 1;
        }
    }
    let restart_fd: ::core::ffi::c_int = parse_restart_fd() as ::core::ffi::c_int;
    if restart_fd != -(1 as ::core::ffi::c_int) {
        debuglog(
            c"%s:%s:%d - serving restart fd %d".as_ptr().cast_mut(),
            c"../src/main.c".as_ptr(),
            c"main".as_ptr(),
            1061 as ::core::ffi::c_int,
            restart_fd,
        );
        let client: *mut ipc_client = ipc_new_client_on_fd(main_loop, restart_fd);
        ipc_confirm_restart(client);
        unsetenv(c"_I3_RESTART_FD".as_ptr());
    }
    x_set_i3_atoms();
    ewmh_update_workarea();
    ewmh_update_desktop_properties();
    let xcb_watcher = watchers.add(xcb_get_file_descriptor(conn), xcb_got_event);
    xcb_prepare = Box::into_raw(Box::new(ev_prepare::new(xcb_prepare_cb)));
    ev_prepare_start(main_loop, xcb_prepare);
    xcb_flush(conn);
    xcb_grab_server(conn);
    xcb_aux_sync(conn);
    let mut event_1: *mut xcb_generic_event_t = ::core::ptr::null_mut::<xcb_generic_event_t>();
    loop {
        let Some(owner) = NativeAllocation::take(xcb_poll_for_event(conn)) else {
            break;
        };
        event_1 = owner.raw();
        if (*event_1).response_type as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        } else {
            let type_0: ::core::ffi::c_int =
                (*event_1).response_type as ::core::ffi::c_int & 0x7f as ::core::ffi::c_int;
            if type_0 == XCB_MAP_REQUEST {
                handle_event(type_0, event_1);
            }
        }
    }
    manage_existing_windows(root);
    xcb_ungrab_server(conn);
    if autostart {
        verboselog(
            c"This is not an in-place restart, checking if a wallpaper is set.\n"
                .as_ptr()
                .cast_mut(),
        );
        let root_0: *mut xcb_screen_t = xcb_aux_get_screen(conn, conn_screen);
        if is_background_set(conn, root_0) {
            verboselog(
                c"A wallpaper is set, so no screenshot is necessary.\n"
                    .as_ptr()
                    .cast_mut(),
            );
        } else {
            verboselog(
                c"No wallpaper set, copying root window contents to a pixmap\n"
                    .as_ptr()
                    .cast_mut(),
            );
            set_screenshot_as_wallpaper(conn, root_0);
        }
    }
    if !disable_signalhandler {
        setup_signal_handler();
    } else {
        let mut action: sigaction = ::core::mem::zeroed();
        action.sa_sigaction = handle_core_signal as *const () as usize;
        action.sa_flags = (SA_NODEFER as ::core::ffi::c_uint
            | SA_RESETHAND
            | SA_SIGINFO as ::core::ffi::c_uint) as ::core::ffi::c_int;
        sigemptyset(&raw mut action.sa_mask);
        if sigaction(
            SIGQUIT,
            &raw mut action,
            ::core::ptr::null_mut::<sigaction>(),
        ) == -(1 as ::core::ffi::c_int)
            || sigaction(
                SIGILL,
                &raw mut action,
                ::core::ptr::null_mut::<sigaction>(),
            ) == -(1 as ::core::ffi::c_int)
            || sigaction(
                SIGABRT,
                &raw mut action,
                ::core::ptr::null_mut::<sigaction>(),
            ) == -(1 as ::core::ffi::c_int)
            || sigaction(
                SIGFPE,
                &raw mut action,
                ::core::ptr::null_mut::<sigaction>(),
            ) == -(1 as ::core::ffi::c_int)
            || sigaction(
                SIGSEGV,
                &raw mut action,
                ::core::ptr::null_mut::<sigaction>(),
            ) == -(1 as ::core::ffi::c_int)
        {
            errorlog(
                c"ERROR: Could not setup signal handler.\n"
                    .as_ptr()
                    .cast_mut(),
            );
        }
    }
    setup_term_handlers();
    signal(SIGPIPE, libc::SIG_IGN);
    if autostart {
        while !autostarts.tqh_first.is_null() {
            let mut exec: *mut Autostart = autostarts.tqh_first;
            verboselog(c"auto-starting %s\n".as_ptr().cast_mut(), (*exec).command);
            start_application(
                (*exec).command,
                (*exec).no_startup_id,
                XCB_NONE as xcb_window_t,
            );
            free((*exec).command as *mut ::core::ffi::c_void);
            (*exec).command = ::core::ptr::null_mut::<::core::ffi::c_char>();
            if !(*exec).autostarts.tqe_next.is_null() {
                (*(*exec).autostarts.tqe_next).autostarts.tqe_prev = (*exec).autostarts.tqe_prev;
            } else {
                autostarts.tqh_last = (*exec).autostarts.tqe_prev;
            }
            *(*exec).autostarts.tqe_prev = (*exec).autostarts.tqe_next;
            free(exec as *mut ::core::ffi::c_void);
            exec = ::core::ptr::null_mut::<Autostart>();
        }
    }
    while !autostarts_always.tqh_first.is_null() {
        let mut exec_always: *mut Autostart = autostarts_always.tqh_first;
        verboselog(
            c"auto-starting (always!) %s\n".as_ptr().cast_mut(),
            (*exec_always).command,
        );
        start_application(
            (*exec_always).command,
            (*exec_always).no_startup_id,
            XCB_NONE as xcb_window_t,
        );
        free((*exec_always).command as *mut ::core::ffi::c_void);
        (*exec_always).command = ::core::ptr::null_mut::<::core::ffi::c_char>();
        if !(*exec_always).autostarts_always.tqe_next.is_null() {
            (*(*exec_always).autostarts_always.tqe_next)
                .autostarts_always
                .tqe_prev = (*exec_always).autostarts_always.tqe_prev;
        } else {
            autostarts_always.tqh_last = (*exec_always).autostarts_always.tqe_prev;
        }
        *(*exec_always).autostarts_always.tqe_prev = (*exec_always).autostarts_always.tqe_next;
        free(exec_always as *mut ::core::ffi::c_void);
        exec_always = ::core::ptr::null_mut::<Autostart>();
    }
    let mut barconfig: *mut Barconfig = ::core::ptr::null_mut::<Barconfig>();
    barconfig = barconfigs.tqh_first as *mut Barconfig;
    while !barconfig.is_null() {
        let program = if (*barconfig).i3bar_command.is_null() {
            c"exec i3bar"
        } else {
            CStr::from_ptr((*barconfig).i3bar_command)
        };
        let command = bar_command(
            program,
            (*barconfig).verbose,
            CStr::from_ptr((*barconfig).id),
            CStr::from_ptr(current_socketpath),
        );
        verboselog(
            c"Starting bar process: %s\n".as_ptr().cast_mut(),
            command.as_ptr(),
        );
        start_application(command.as_ptr(), true, 0);
        barconfig = (*barconfig).configs.tqe_next as *mut Barconfig;
    }
    atexit(i3_exit);
    raise(SIGCHLD);
    sd_notify(1 as ::core::ffi::c_int, c"READY=1");
    ev_loop(main_loop, 0 as ::core::ffi::c_int);
    ev_prepare_stop(main_loop, xcb_prepare);
    drop(Box::from_raw(xcb_prepare));
    xcb_prepare = std::ptr::null_mut();
    drop(watchers);
    0
}
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const EXIT_SUCCESS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
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

use xcb_sys::{
    xcb_atom_name_by_screen, xcb_atom_t, xcb_aux_find_visual_by_attrs, xcb_aux_get_depth_of_visual,
    xcb_aux_get_screen, xcb_aux_sync, xcb_big_requests_id, xcb_change_property,
    xcb_change_window_attributes, xcb_change_window_attributes_checked, xcb_client_message_data_t,
    xcb_client_message_event_t, xcb_colormap_t, xcb_connect, xcb_connection_has_error,
    xcb_connection_t, xcb_create_colormap_checked, xcb_create_window, xcb_disconnect,
    xcb_drawable_t, xcb_flush, xcb_generate_id, xcb_generic_error_t, xcb_generic_event_t,
    xcb_get_extension_data, xcb_get_file_descriptor, xcb_get_geometry, xcb_get_geometry_cookie_t,
    xcb_get_geometry_reply, xcb_get_geometry_reply_t, xcb_get_selection_owner,
    xcb_get_selection_owner_reply, xcb_get_selection_owner_reply_t, xcb_grab_server,
    xcb_intern_atom, xcb_intern_atom_cookie_t, xcb_intern_atom_reply, xcb_intern_atom_reply_t,
    xcb_intern_atom_unchecked, xcb_key_symbols_alloc, xcb_key_symbols_t, xcb_keycode_t,
    xcb_poll_for_event, xcb_prefetch_extension_data, xcb_prefetch_maximum_request_length,
    xcb_property_notify_event_t, xcb_query_extension_reply_t, xcb_query_pointer,
    xcb_query_pointer_cookie_t, xcb_query_pointer_reply, xcb_query_pointer_reply_t, xcb_randr_id,
    xcb_request_check, xcb_screen_t, xcb_send_event, xcb_set_selection_owner, xcb_shape_id,
    xcb_shape_query_version, xcb_shape_query_version_cookie_t, xcb_shape_query_version_reply,
    xcb_shape_query_version_reply_t, xcb_timestamp_t, xcb_ungrab_server, xcb_visualtype_t,
    xcb_void_cookie_t, xcb_wait_for_event, xcb_window_t, xcb_xinerama_id, xcb_xkb_device_spec_t,
    xcb_xkb_id, xcb_xkb_per_client_flags, xcb_xkb_per_client_flags_reply,
    xcb_xkb_per_client_flags_reply_t, xcb_xkb_select_events, xcb_xkb_use_extension,
    XCB_ATOM_CARDINAL, XCB_ATOM_STRING, XCB_ATOM_SUPERSCRIPT_X, XCB_ATOM_WM_CLASS,
    XCB_CLIENT_MESSAGE, XCB_COLORMAP_ALLOC_NONE, XCB_CURRENT_TIME, XCB_CW_EVENT_MASK,
    XCB_EVENT_MASK_BUTTON_PRESS, XCB_EVENT_MASK_ENTER_WINDOW, XCB_EVENT_MASK_FOCUS_CHANGE,
    XCB_EVENT_MASK_POINTER_MOTION, XCB_EVENT_MASK_PROPERTY_CHANGE, XCB_EVENT_MASK_STRUCTURE_NOTIFY,
    XCB_EVENT_MASK_SUBSTRUCTURE_REDIRECT, XCB_MAP_REQUEST, XCB_NONE, XCB_NUM_LOCK,
    XCB_PROPERTY_NOTIFY, XCB_PROP_MODE_APPEND, XCB_PROP_MODE_REPLACE,
    XCB_WINDOW_CLASS_INPUT_OUTPUT, XCB_XKB_EVENT_TYPE_MAP_NOTIFY,
    XCB_XKB_EVENT_TYPE_NEW_KEYBOARD_NOTIFY, XCB_XKB_EVENT_TYPE_STATE_NOTIFY,
    XCB_XKB_ID_USE_CORE_KBD, XCB_XKB_MAJOR_VERSION, XCB_XKB_MINOR_VERSION,
    XCB_XKB_PER_CLIENT_FLAG_DETECTABLE_AUTO_REPEAT, XCB_XKB_PER_CLIENT_FLAG_GRABS_USE_XKB_STATE,
    XCB_XKB_PER_CLIENT_FLAG_LOOKUP_STATE_WHEN_GRABBED,
};

use crate::handlers::xkb_base;

use startup_notification_sys::{sn_xcb_display_new, SnDisplay};

pub use libc::{
    atexit, close, connect, dirname, exit, fcntl, fflush, fileno, fprintf, free, getcwd, getenv,
    getrlimit, isatty, memmove, open, printf, raise, read, rlimit, rmdir, setbuf, setlocale,
    setrlimit, shm_unlink, sigaction, sigemptyset, siginfo_t, signal, sockaddr, sockaddr_un,
    socket, srand, strcmp, strlen, strncpy, strstr, time, unlink, unsetenv, usleep,
};

use i3_common::native_stdio::{err, errx, stderr, stdout};

use crate::log::{debuglog, errorlog, verboselog};

// Shared model types retain field layout while using one Rust identity.
pub use crate::types::gaps_t;
pub use crate::types::xoutput;
pub use crate::types::Assignment;
pub use crate::types::Con;
pub use crate::types::Rect;
pub use i3_common::font::Font;

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
use i3_common::get_mod_mask::aio_get_mod_mask_for;
use i3_common::get_process_filename::get_process_filename;
use i3_common::get_visualtype::get_visualtype;
use i3_common::ipc_recv_message::ipc_recv_message;
use i3_common::ipc_send_message::ipc_send_message;
use i3_common::is_background_set::is_background_set;
use i3_common::root_atom_contents::root_atom_contents;
use i3_common::screenshot_wallpaper::set_screenshot_as_wallpaper;

use crate::xcb::xcb_numlock_mask;

#[cfg(test)]
mod owned_startup_tests {
    use super::*;
    #[test]
    fn bar_command_preserves_native_spaces_quotes_and_non_utf8() {
        assert_eq!(
            bar_command(c"exec i3bar", false, c"id\xff", c"/socket\xfe").to_bytes(),
            b"exec i3bar  --bar_id=id\xff --socket=\"/socket\xfe\""
        );
        assert_eq!(
            bar_command(c"custom", true, c"bar", c"/path").to_bytes(),
            b"custom -V --bar_id=bar --socket=\"/path\""
        );
        assert_eq!(owned_dirname(c"relative/file").as_c_str(), c"relative");
    }
}
