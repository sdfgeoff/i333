use crate::config::barconfigs;
use crate::handlers::event_is_ignored;
use crate::ipc::ipc_confirm_restart;
use crate::ipc::ipc_new_client_on_fd;
use crate::randr::outputs;
use i3_common::safewrappers::sasprintf;
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
pub type size_t = usize;
pub type __int8_t = i8;
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type __uid_t = ::core::ffi::c_uint;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __pid_t = ::core::ffi::c_int;
pub type __clock_t = ::core::ffi::c_long;
pub type __rlim64_t = ::core::ffi::c_ulong;
pub type __time_t = ::core::ffi::c_long;
pub type __useconds_t = ::core::ffi::c_uint;
pub type __suseconds_t = ::core::ffi::c_long;
pub type __socklen_t = ::core::ffi::c_uint;
pub type pid_t = __pid_t;
pub type ssize_t = isize;
pub type time_t = __time_t;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;

pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;

pub type socklen_t = __socklen_t;
pub type i3String = _i3String;

pub type C2RustUnnamed_12 = ::core::ffi::c_uint;
pub const FONT_TYPE_PANGO: C2RustUnnamed_12 = 2;
pub const FONT_TYPE_XCB: C2RustUnnamed_12 = 1;
pub const FONT_TYPE_NONE: C2RustUnnamed_12 = 0;
pub type i3Font = Font;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Binding {
    pub input_type: input_type_t,
    pub release: C2RustUnnamed_15,
    pub border: bool,
    pub whole_window: bool,
    pub exclude_titlebar: bool,
    pub keycode: uint32_t,
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
pub type i3_event_state_mask_t = uint32_t;
pub type C2RustUnnamed_15 = ::core::ffi::c_uint;
pub const B_UPON_KEYRELEASE_IGNORE_MODS: C2RustUnnamed_15 = 2;
pub const B_UPON_KEYRELEASE: C2RustUnnamed_15 = 1;
pub const B_UPON_KEYPRESS: C2RustUnnamed_15 = 0;
pub type input_type_t = ::core::ffi::c_uint;
pub const B_MOUSE: input_type_t = 1;
pub const B_KEYBOARD: input_type_t = 0;

pub type C2RustUnnamed_17 = ::core::ffi::c_uint;
pub const SCRATCHPAD_CHANGED: C2RustUnnamed_17 = 2;
pub const SCRATCHPAD_FRESH: C2RustUnnamed_17 = 1;
pub const SCRATCHPAD_NONE: C2RustUnnamed_17 = 0;

pub type C2RustUnnamed_22 = ::core::ffi::c_uint;
pub const FLOATING_USER_ON: C2RustUnnamed_22 = 3;
pub const FLOATING_AUTO_ON: C2RustUnnamed_22 = 2;
pub const FLOATING_USER_OFF: C2RustUnnamed_22 = 1;
pub const FLOATING_AUTO_OFF: C2RustUnnamed_22 = 0;
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

pub type C2RustUnnamed_24 = ::core::ffi::c_uint;
pub const M_BELOW: C2RustUnnamed_24 = 2;
pub const M_ASSIGN_WS: C2RustUnnamed_24 = 1;
pub const M_HERE: C2RustUnnamed_24 = 0;
pub type C2RustUnnamed_25 = ::core::ffi::c_uint;
pub const WM_FLOATING: C2RustUnnamed_25 = 6;
pub const WM_FLOATING_USER: C2RustUnnamed_25 = 5;
pub const WM_FLOATING_AUTO: C2RustUnnamed_25 = 4;
pub const WM_TILING: C2RustUnnamed_25 = 3;
pub const WM_TILING_USER: C2RustUnnamed_25 = 2;
pub const WM_TILING_AUTO: C2RustUnnamed_25 = 1;
pub const WM_ANY: C2RustUnnamed_25 = 0;
pub type C2RustUnnamed_26 = ::core::ffi::c_int;
pub const M_DOCK_BOTTOM: C2RustUnnamed_26 = 3;
pub const M_DOCK_TOP: C2RustUnnamed_26 = 2;
pub const M_DOCK_ANY: C2RustUnnamed_26 = 1;
pub const M_NODOCK: C2RustUnnamed_26 = 0;
pub const M_DONTCHECK: C2RustUnnamed_26 = -1;
pub type C2RustUnnamed_27 = ::core::ffi::c_int;
pub const U_OLDEST: C2RustUnnamed_27 = 1;
pub const U_LATEST: C2RustUnnamed_27 = 0;
pub const U_DONTCHECK: C2RustUnnamed_27 = -1;

pub type C2RustUnnamed_28 = ::core::ffi::c_uint;
pub const W_DOCK_BOTTOM: C2RustUnnamed_28 = 2;
pub const W_DOCK_TOP: C2RustUnnamed_28 = 1;
pub const W_NODOCK: C2RustUnnamed_28 = 0;

pub type C2RustUnnamed_31 = ::core::ffi::c_uint;
pub const A_TO_OUTPUT: C2RustUnnamed_31 = 16;
pub const A_TO_WORKSPACE_NUMBER: C2RustUnnamed_31 = 8;
pub const A_NO_FOCUS: C2RustUnnamed_31 = 4;
pub const A_TO_WORKSPACE: C2RustUnnamed_31 = 2;
pub const A_COMMAND: C2RustUnnamed_31 = 1;
pub const A_ANY: C2RustUnnamed_31 = 0;

pub type C2RustUnnamed_33 = ::core::ffi::c_uint;
pub const CT_DOCKAREA: C2RustUnnamed_33 = 5;
pub const CT_WORKSPACE: C2RustUnnamed_33 = 4;
pub const CT_FLOATING_CON: C2RustUnnamed_33 = 3;
pub const CT_CON: C2RustUnnamed_33 = 2;
pub const CT_OUTPUT: C2RustUnnamed_33 = 1;
pub const CT_ROOT: C2RustUnnamed_33 = 0;

pub type Output = xoutput;
pub type smart_gaps_t = ::core::ffi::c_uint;
pub const SMART_GAPS_INVERSE_OUTER: smart_gaps_t = 2;
pub const SMART_GAPS_ON: smart_gaps_t = 1;
pub const SMART_GAPS_OFF: smart_gaps_t = 0;
pub type hide_edge_borders_mode_t = ::core::ffi::c_uint;
pub const HEBM_SMART_NO_GAPS: hide_edge_borders_mode_t = 64;
pub const HEBM_SMART: hide_edge_borders_mode_t = 32;
pub const HEBM_BOTH: hide_edge_borders_mode_t = 23;
pub const HEBM_HORIZONTAL: hide_edge_borders_mode_t = 20;
pub const HEBM_VERTICAL: hide_edge_borders_mode_t = 3;
pub const HEBM_NONE: hide_edge_borders_mode_t = 0;
pub type warping_t = ::core::ffi::c_uint;
pub const POINTER_WARPING_NONE: warping_t = 1;
pub const POINTER_WARPING_OUTPUT: warping_t = 0;
pub type gaps_mask_t = ::core::ffi::c_uint;
pub const GAPS_OUTER: gaps_mask_t = 30;
pub const GAPS_HORIZONTAL: gaps_mask_t = 20;
pub const GAPS_VERTICAL: gaps_mask_t = 10;
pub const GAPS_LEFT: gaps_mask_t = 16;
pub const GAPS_BOTTOM: gaps_mask_t = 8;
pub const GAPS_RIGHT: gaps_mask_t = 4;
pub const GAPS_TOP: gaps_mask_t = 2;
pub const GAPS_INNER: gaps_mask_t = 1;
pub type focus_wrapping_t = ::core::ffi::c_uint;
pub const FOCUS_WRAPPING_WORKSPACE: focus_wrapping_t = 3;
pub const FOCUS_WRAPPING_FORCE: focus_wrapping_t = 2;
pub const FOCUS_WRAPPING_ON: focus_wrapping_t = 1;
pub const FOCUS_WRAPPING_OFF: focus_wrapping_t = 0;
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
    pub value: uint32_t,
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

pub type C2RustUnnamed_39 = ::core::ffi::c_uint;
pub const PDF_ALL: C2RustUnnamed_39 = 3;
pub const PDF_IGNORE: C2RustUnnamed_39 = 2;
pub const PDF_LEAVE_FULLSCREEN: C2RustUnnamed_39 = 1;
pub const PDF_SMART: C2RustUnnamed_39 = 0;

pub type C2RustUnnamed_40 = ::core::ffi::c_uint;
pub const ALIGN_RIGHT: C2RustUnnamed_40 = 2;
pub const ALIGN_CENTER: C2RustUnnamed_40 = 1;
pub const ALIGN_LEFT: C2RustUnnamed_40 = 0;
pub type C2RustUnnamed_41 = ::core::ffi::c_uint;
pub const FOWA_NONE: C2RustUnnamed_41 = 3;
pub const FOWA_FOCUS: C2RustUnnamed_41 = 2;
pub const FOWA_URGENT: C2RustUnnamed_41 = 1;
pub const FOWA_SMART: C2RustUnnamed_41 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Barconfig {
    pub id: *mut ::core::ffi::c_char,
    pub num_outputs: ::core::ffi::c_int,
    pub outputs: *mut *mut ::core::ffi::c_char,
    pub tray_outputs: tray_outputs_head,
    pub tray_padding: ::core::ffi::c_int,
    pub socket_path: *mut ::core::ffi::c_char,
    pub mode: C2RustUnnamed_46,
    pub hidden_state: C2RustUnnamed_45,
    pub modifier: uint32_t,
    pub bar_bindings: bar_bindings_head,
    pub position: C2RustUnnamed_43,
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
    pub bar_height: uint32_t,
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
pub type C2RustUnnamed_43 = ::core::ffi::c_uint;
pub const P_TOP: C2RustUnnamed_43 = 1;
pub const P_BOTTOM: C2RustUnnamed_43 = 0;
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
pub type C2RustUnnamed_45 = ::core::ffi::c_uint;
pub const S_SHOW: C2RustUnnamed_45 = 1;
pub const S_HIDE: C2RustUnnamed_45 = 0;
pub type C2RustUnnamed_46 = ::core::ffi::c_uint;
pub const M_INVISIBLE: C2RustUnnamed_46 = 2;
pub const M_HIDE: C2RustUnnamed_46 = 1;
pub const M_DOCK: C2RustUnnamed_46 = 0;
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
pub type config_load_t = ::core::ffi::c_uint;
pub const C_RELOAD: config_load_t = 2;
pub const C_LOAD: config_load_t = 1;
pub const C_VALIDATE: config_load_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ipc_client {
    pub fd: ::core::ffi::c_int,
    pub num_events: ::core::ffi::c_int,
    pub events: *mut *mut ::core::ffi::c_char,
    pub first_tick_sent: bool,
    pub read_callback: *mut ev_io,
    pub write_callback: *mut ev_io,
    pub timeout: *mut ev_timer,
    pub buffer: *mut uint8_t,
    pub buffer_size: size_t,
    pub clients: C2RustUnnamed_48,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_48 {
    pub tqe_next: *mut ipc_client,
    pub tqe_prev: *mut *mut ipc_client,
}
pub type shutdown_reason_t = ::core::ffi::c_uint;
pub const SHUTDOWN_REASON_EXIT: shutdown_reason_t = 1;
pub const SHUTDOWN_REASON_RESTART: shutdown_reason_t = 0;
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
pub struct outputs_head {
    pub tqh_first: *mut xoutput,
    pub tqh_last: *mut *mut xoutput,
}

pub type __socket_type = ::core::ffi::c_uint;
pub const SOCK_NONBLOCK: __socket_type = 2048;
pub const SOCK_CLOEXEC: __socket_type = 524288;
pub const SOCK_PACKET: __socket_type = 10;
pub const SOCK_DCCP: __socket_type = 6;
pub const SOCK_SEQPACKET: __socket_type = 5;
pub const SOCK_RDM: __socket_type = 4;
pub const SOCK_RAW: __socket_type = 3;
pub const SOCK_DGRAM: __socket_type = 2;
pub const SOCK_STREAM: __socket_type = 1;
pub type sa_family_t = ::core::ffi::c_ushort;

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

pub static mut root_depth: uint8_t = 0;

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
        event = xcb_poll_for_event(conn);
        if event.is_null() {
            break;
        }
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
            free(event as *mut ::core::ffi::c_void);
        } else {
            let type_0: ::core::ffi::c_int =
                (*event).response_type as ::core::ffi::c_int & 0x7f as ::core::ffi::c_int;
            handle_event(type_0, event);
            free(event as *mut ::core::ffi::c_void);
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
        ipc_shutdown(SHUTDOWN_REASON_EXIT, -(1 as ::core::ffi::c_int));
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
    static mut signal_watchers: [ev_signal; 6] = [ev_signal {
        active: 0,
        pending: 0,
        priority: 0,
        data: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
        cb: None,
        next: ::core::ptr::null::<ev_watcher_list>() as *mut ev_watcher_list,
        signum: 0,
    }; 6];
    let num_watchers: size_t = (::core::mem::size_of::<[ev_signal; 6]>() as size_t)
        .wrapping_div(::core::mem::size_of::<ev_signal>() as size_t);
    let ref mut fresh0 = (*((&raw mut signal_watchers as *mut ev_signal)
        .offset(0 as ::core::ffi::c_int as isize) as *mut ev_signal
        as *mut ::core::ffi::c_void as *mut ev_watcher))
        .pending;
    *fresh0 = 0 as ::core::ffi::c_int;
    (*((&raw mut signal_watchers as *mut ev_signal).offset(0 as ::core::ffi::c_int as isize)
        as *mut ev_signal as *mut ::core::ffi::c_void as *mut ev_watcher))
        .active = *fresh0;
    (*((&raw mut signal_watchers as *mut ev_signal).offset(0 as ::core::ffi::c_int as isize)
        as *mut ev_signal as *mut ::core::ffi::c_void as *mut ev_watcher))
        .priority = 0 as ::core::ffi::c_int;
    signal_watchers[0 as ::core::ffi::c_int as usize].cb = Some(
        handle_term_signal
            as unsafe extern "C" fn(*mut ev_loop, *mut ev_signal, ::core::ffi::c_int) -> (),
    )
        as Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_signal, ::core::ffi::c_int) -> ()>;
    memmove(
        &raw mut (*((&raw mut signal_watchers as *mut ev_signal)
            .offset(0 as ::core::ffi::c_int as isize) as *mut ev_signal
            as *mut ev_watcher))
            .cb as *mut ::core::ffi::c_void,
        &raw mut (*(&raw mut signal_watchers as *mut ev_signal)
            .offset(0 as ::core::ffi::c_int as isize))
        .cb as *const ::core::ffi::c_void,
        ::core::mem::size_of::<
            Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_signal, ::core::ffi::c_int) -> ()>,
        >() as size_t,
    );
    signal_watchers[0 as ::core::ffi::c_int as usize].signum = 1 as ::core::ffi::c_int;
    let ref mut fresh1 = (*((&raw mut signal_watchers as *mut ev_signal)
        .offset(1 as ::core::ffi::c_int as isize) as *mut ev_signal
        as *mut ::core::ffi::c_void as *mut ev_watcher))
        .pending;
    *fresh1 = 0 as ::core::ffi::c_int;
    (*((&raw mut signal_watchers as *mut ev_signal).offset(1 as ::core::ffi::c_int as isize)
        as *mut ev_signal as *mut ::core::ffi::c_void as *mut ev_watcher))
        .active = *fresh1;
    (*((&raw mut signal_watchers as *mut ev_signal).offset(1 as ::core::ffi::c_int as isize)
        as *mut ev_signal as *mut ::core::ffi::c_void as *mut ev_watcher))
        .priority = 0 as ::core::ffi::c_int;
    signal_watchers[1 as ::core::ffi::c_int as usize].cb = Some(
        handle_term_signal
            as unsafe extern "C" fn(*mut ev_loop, *mut ev_signal, ::core::ffi::c_int) -> (),
    )
        as Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_signal, ::core::ffi::c_int) -> ()>;
    memmove(
        &raw mut (*((&raw mut signal_watchers as *mut ev_signal)
            .offset(1 as ::core::ffi::c_int as isize) as *mut ev_signal
            as *mut ev_watcher))
            .cb as *mut ::core::ffi::c_void,
        &raw mut (*(&raw mut signal_watchers as *mut ev_signal)
            .offset(1 as ::core::ffi::c_int as isize))
        .cb as *const ::core::ffi::c_void,
        ::core::mem::size_of::<
            Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_signal, ::core::ffi::c_int) -> ()>,
        >() as size_t,
    );
    signal_watchers[1 as ::core::ffi::c_int as usize].signum = 2 as ::core::ffi::c_int;
    let ref mut fresh2 = (*((&raw mut signal_watchers as *mut ev_signal)
        .offset(2 as ::core::ffi::c_int as isize) as *mut ev_signal
        as *mut ::core::ffi::c_void as *mut ev_watcher))
        .pending;
    *fresh2 = 0 as ::core::ffi::c_int;
    (*((&raw mut signal_watchers as *mut ev_signal).offset(2 as ::core::ffi::c_int as isize)
        as *mut ev_signal as *mut ::core::ffi::c_void as *mut ev_watcher))
        .active = *fresh2;
    (*((&raw mut signal_watchers as *mut ev_signal).offset(2 as ::core::ffi::c_int as isize)
        as *mut ev_signal as *mut ::core::ffi::c_void as *mut ev_watcher))
        .priority = 0 as ::core::ffi::c_int;
    signal_watchers[2 as ::core::ffi::c_int as usize].cb = Some(
        handle_term_signal
            as unsafe extern "C" fn(*mut ev_loop, *mut ev_signal, ::core::ffi::c_int) -> (),
    )
        as Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_signal, ::core::ffi::c_int) -> ()>;
    memmove(
        &raw mut (*((&raw mut signal_watchers as *mut ev_signal)
            .offset(2 as ::core::ffi::c_int as isize) as *mut ev_signal
            as *mut ev_watcher))
            .cb as *mut ::core::ffi::c_void,
        &raw mut (*(&raw mut signal_watchers as *mut ev_signal)
            .offset(2 as ::core::ffi::c_int as isize))
        .cb as *const ::core::ffi::c_void,
        ::core::mem::size_of::<
            Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_signal, ::core::ffi::c_int) -> ()>,
        >() as size_t,
    );
    signal_watchers[2 as ::core::ffi::c_int as usize].signum = 14 as ::core::ffi::c_int;
    let ref mut fresh3 = (*((&raw mut signal_watchers as *mut ev_signal)
        .offset(3 as ::core::ffi::c_int as isize) as *mut ev_signal
        as *mut ::core::ffi::c_void as *mut ev_watcher))
        .pending;
    *fresh3 = 0 as ::core::ffi::c_int;
    (*((&raw mut signal_watchers as *mut ev_signal).offset(3 as ::core::ffi::c_int as isize)
        as *mut ev_signal as *mut ::core::ffi::c_void as *mut ev_watcher))
        .active = *fresh3;
    (*((&raw mut signal_watchers as *mut ev_signal).offset(3 as ::core::ffi::c_int as isize)
        as *mut ev_signal as *mut ::core::ffi::c_void as *mut ev_watcher))
        .priority = 0 as ::core::ffi::c_int;
    signal_watchers[3 as ::core::ffi::c_int as usize].cb = Some(
        handle_term_signal
            as unsafe extern "C" fn(*mut ev_loop, *mut ev_signal, ::core::ffi::c_int) -> (),
    )
        as Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_signal, ::core::ffi::c_int) -> ()>;
    memmove(
        &raw mut (*((&raw mut signal_watchers as *mut ev_signal)
            .offset(3 as ::core::ffi::c_int as isize) as *mut ev_signal
            as *mut ev_watcher))
            .cb as *mut ::core::ffi::c_void,
        &raw mut (*(&raw mut signal_watchers as *mut ev_signal)
            .offset(3 as ::core::ffi::c_int as isize))
        .cb as *const ::core::ffi::c_void,
        ::core::mem::size_of::<
            Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_signal, ::core::ffi::c_int) -> ()>,
        >() as size_t,
    );
    signal_watchers[3 as ::core::ffi::c_int as usize].signum = 15 as ::core::ffi::c_int;
    let ref mut fresh4 = (*((&raw mut signal_watchers as *mut ev_signal)
        .offset(4 as ::core::ffi::c_int as isize) as *mut ev_signal
        as *mut ::core::ffi::c_void as *mut ev_watcher))
        .pending;
    *fresh4 = 0 as ::core::ffi::c_int;
    (*((&raw mut signal_watchers as *mut ev_signal).offset(4 as ::core::ffi::c_int as isize)
        as *mut ev_signal as *mut ::core::ffi::c_void as *mut ev_watcher))
        .active = *fresh4;
    (*((&raw mut signal_watchers as *mut ev_signal).offset(4 as ::core::ffi::c_int as isize)
        as *mut ev_signal as *mut ::core::ffi::c_void as *mut ev_watcher))
        .priority = 0 as ::core::ffi::c_int;
    signal_watchers[4 as ::core::ffi::c_int as usize].cb = Some(
        handle_term_signal
            as unsafe extern "C" fn(*mut ev_loop, *mut ev_signal, ::core::ffi::c_int) -> (),
    )
        as Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_signal, ::core::ffi::c_int) -> ()>;
    memmove(
        &raw mut (*((&raw mut signal_watchers as *mut ev_signal)
            .offset(4 as ::core::ffi::c_int as isize) as *mut ev_signal
            as *mut ev_watcher))
            .cb as *mut ::core::ffi::c_void,
        &raw mut (*(&raw mut signal_watchers as *mut ev_signal)
            .offset(4 as ::core::ffi::c_int as isize))
        .cb as *const ::core::ffi::c_void,
        ::core::mem::size_of::<
            Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_signal, ::core::ffi::c_int) -> ()>,
        >() as size_t,
    );
    signal_watchers[4 as ::core::ffi::c_int as usize].signum = 10 as ::core::ffi::c_int;
    let ref mut fresh5 = (*((&raw mut signal_watchers as *mut ev_signal)
        .offset(5 as ::core::ffi::c_int as isize) as *mut ev_signal
        as *mut ::core::ffi::c_void as *mut ev_watcher))
        .pending;
    *fresh5 = 0 as ::core::ffi::c_int;
    (*((&raw mut signal_watchers as *mut ev_signal).offset(5 as ::core::ffi::c_int as isize)
        as *mut ev_signal as *mut ::core::ffi::c_void as *mut ev_watcher))
        .active = *fresh5;
    (*((&raw mut signal_watchers as *mut ev_signal).offset(5 as ::core::ffi::c_int as isize)
        as *mut ev_signal as *mut ::core::ffi::c_void as *mut ev_watcher))
        .priority = 0 as ::core::ffi::c_int;
    signal_watchers[5 as ::core::ffi::c_int as usize].cb = Some(
        handle_term_signal
            as unsafe extern "C" fn(*mut ev_loop, *mut ev_signal, ::core::ffi::c_int) -> (),
    )
        as Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_signal, ::core::ffi::c_int) -> ()>;
    memmove(
        &raw mut (*((&raw mut signal_watchers as *mut ev_signal)
            .offset(5 as ::core::ffi::c_int as isize) as *mut ev_signal
            as *mut ev_watcher))
            .cb as *mut ::core::ffi::c_void,
        &raw mut (*(&raw mut signal_watchers as *mut ev_signal)
            .offset(5 as ::core::ffi::c_int as isize))
        .cb as *const ::core::ffi::c_void,
        ::core::mem::size_of::<
            Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_signal, ::core::ffi::c_int) -> ()>,
        >() as size_t,
    );
    signal_watchers[5 as ::core::ffi::c_int as usize].signum = 12 as ::core::ffi::c_int;
    let mut i: size_t = 0 as size_t;
    while i < num_watchers {
        ev_signal_start(
            main_loop,
            (&raw mut signal_watchers as *mut ev_signal).offset(i as isize) as *mut ev_signal,
        );
        ev_unref(main_loop);
        i = i.wrapping_add(1);
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
    getrlimit(RLIMIT_CORE, &raw mut original_rlimit_core);
    if isatty(fileno(stdout)) == 0 {
        setbuf(stdout, ::core::ptr::null_mut::<::core::ffi::c_char>());
    }
    srand(time(::core::ptr::null_mut::<time_t>()) as ::core::ffi::c_uint);
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
                free(layout_path.cast());
                layout_path = sstrdup(value.as_ref().unwrap().as_ptr());
                delete_layout_path = flag == OPT_RESTART;
            },
            b'c' => { free(override_configpath.cast()); override_configpath = sstrdup(value.as_ref().unwrap().as_ptr()); },
            b'C' => { verboselog(c"Checking configuration file only (-C)\n".as_ptr().cast_mut()); only_check_config = true; },
            b'v' => { printf(c"i3 version %s © 2009 Michael Stapelberg and contributors\n".as_ptr(), i3_version); return EXIT_SUCCESS; },
            b'm' => { printf(c"Binary i3 version:  %s © 2009 Michael Stapelberg and contributors\n".as_ptr(), i3_version); display_running_version(); return EXIT_SUCCESS; },
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
                free(fake_outputs.cast());
                fake_outputs = sstrdup(value.as_ptr());
            },
            OPT_OLD_PARSER => errorlog(c"ERROR: You are passing --force-old-config-parser-v4.4-only, but that flag was removed by now.\n".as_ptr().cast_mut()),
            _ => unreachable!(),
        }
    }
    if only_check_config {
        exit(
            if load_configuration(override_configpath, C_VALIDATE) as ::core::ffi::c_int != 0 {
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
            SOCK_STREAM as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        );
        if sockfd == -(1 as ::core::ffi::c_int) {
            err(EXIT_FAILURE, c"Could not create socket".as_ptr());
        }
        let mut addr: sockaddr_un = sockaddr_un {
            sun_family: 0 as sa_family_t,
            sun_path: [0; 108],
        };
        addr.sun_family = AF_LOCAL as sa_family_t;
        strncpy(
            &raw mut addr.sun_path as *mut ::core::ffi::c_char,
            socket_path_0,
            (::core::mem::size_of::<[::core::ffi::c_char; 108]>() as size_t)
                .wrapping_sub(1 as size_t),
        );
        free(socket_path_0 as *mut ::core::ffi::c_void);
        socket_path_0 = ::core::ptr::null_mut::<::core::ffi::c_char>();
        if connect(
            sockfd,
            &raw mut addr as *const sockaddr,
            ::core::mem::size_of::<sockaddr_un>() as socklen_t,
        ) < 0 as ::core::ffi::c_int
        {
            err(EXIT_FAILURE, c"Could not connect to i3".as_ptr());
        }
        if ipc_send_message(
            sockfd,
            strlen(payload) as uint32_t,
            I3_IPC_MESSAGE_TYPE_RUN_COMMAND as uint32_t,
            payload as *mut uint8_t,
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
            return 1 as ::core::ffi::c_int;
        }
        if reply_type != I3_IPC_REPLY_TYPE_COMMAND as uint32_t {
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
        free(reply as *mut ::core::ffi::c_void);
        reply = ::core::ptr::null_mut::<uint8_t>();
        return 0 as ::core::ffi::c_int;
    }
    init_logging();
    if is_debug_build() {
        let mut limit: rlimit = rlimit {
            rlim_cur: RLIM_INFINITY as rlim_t,
            rlim_max: RLIM_INFINITY as rlim_t,
        };
        setrlimit(RLIMIT_CORE, &raw mut limit);
        verboselog(
            c"CORE DUMPS: You are running a development version of i3, so coredumps were automatically enabled (ulimit -c unlimited).\n".as_ptr().cast_mut(),
        );
        let cwd: *mut ::core::ffi::c_char =
            getcwd(::core::ptr::null_mut::<::core::ffi::c_char>(), 0 as size_t);
        if !cwd.is_null() {
            verboselog(
                c"CORE DUMPS: Your current working directory is \"%s\".\n"
                    .as_ptr()
                    .cast_mut(),
                cwd,
            );
            free(cwd as *mut ::core::ffi::c_void);
        }
        let buffer_size: size_t = 1024 as size_t;
        let buffer: *mut ::core::ffi::c_char = scalloc(
            buffer_size,
            ::core::mem::size_of::<::core::ffi::c_char>() as size_t,
        ) as *mut ::core::ffi::c_char;
        let mut patternfd: ::core::ffi::c_int = 0;
        patternfd = open(c"/proc/sys/kernel/core_pattern".as_ptr(), O_RDONLY);
        if patternfd >= 0 as ::core::ffi::c_int {
            if read(
                patternfd,
                buffer as *mut ::core::ffi::c_void,
                buffer_size.wrapping_sub(1 as size_t),
            ) > 0 as ssize_t
            {
                verboselog(
                    c"CORE DUMPS: Your core_pattern is: %s".as_ptr().cast_mut(),
                    buffer,
                );
            }
            close(patternfd);
        }
        free(buffer as *mut ::core::ffi::c_void);
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
    let mut fresh6: [uint32_t; 1] =
        [XCB_EVENT_MASK_PROPERTY_CHANGE as ::core::ffi::c_int as uint32_t];
    xcb_change_window_attributes(
        conn,
        root,
        XCB_CW_EVENT_MASK as ::core::ffi::c_int as uint32_t,
        &raw mut fresh6 as *mut uint32_t as *const ::core::ffi::c_void,
    );
    xcb_change_property(
        conn,
        XCB_PROP_MODE_APPEND as ::core::ffi::c_int as uint8_t,
        root,
        XCB_ATOM_SUPERSCRIPT_X as ::core::ffi::c_int as xcb_atom_t,
        XCB_ATOM_CARDINAL as ::core::ffi::c_int as xcb_atom_t,
        32 as uint8_t,
        0 as uint32_t,
        c"".as_ptr() as *const ::core::ffi::c_void,
    );
    let mut _NET_SUPPORTED_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(c"_NET_SUPPORTED".as_ptr()) as uint16_t,
        c"_NET_SUPPORTED".as_ptr(),
    );
    let mut _NET_SUPPORTING_WM_CHECK_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(c"_NET_SUPPORTING_WM_CHECK".as_ptr()) as uint16_t,
        c"_NET_SUPPORTING_WM_CHECK".as_ptr(),
    );
    let mut _NET_WM_NAME_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(c"_NET_WM_NAME".as_ptr()) as uint16_t,
        c"_NET_WM_NAME".as_ptr(),
    );
    let mut _NET_WM_VISIBLE_NAME_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(c"_NET_WM_VISIBLE_NAME".as_ptr()) as uint16_t,
        c"_NET_WM_VISIBLE_NAME".as_ptr(),
    );
    let mut _NET_WM_MOVERESIZE_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(c"_NET_WM_MOVERESIZE".as_ptr()) as uint16_t,
        c"_NET_WM_MOVERESIZE".as_ptr(),
    );
    let mut _NET_WM_STATE_STICKY_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(c"_NET_WM_STATE_STICKY".as_ptr()) as uint16_t,
        c"_NET_WM_STATE_STICKY".as_ptr(),
    );
    let mut _NET_WM_STATE_FULLSCREEN_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(c"_NET_WM_STATE_FULLSCREEN".as_ptr()) as uint16_t,
        c"_NET_WM_STATE_FULLSCREEN".as_ptr(),
    );
    let mut _NET_WM_STATE_DEMANDS_ATTENTION_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(c"_NET_WM_STATE_DEMANDS_ATTENTION".as_ptr()) as uint16_t,
        c"_NET_WM_STATE_DEMANDS_ATTENTION".as_ptr(),
    );
    let mut _NET_WM_STATE_MODAL_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(c"_NET_WM_STATE_MODAL".as_ptr()) as uint16_t,
        c"_NET_WM_STATE_MODAL".as_ptr(),
    );
    let mut _NET_WM_STATE_HIDDEN_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(c"_NET_WM_STATE_HIDDEN".as_ptr()) as uint16_t,
        c"_NET_WM_STATE_HIDDEN".as_ptr(),
    );
    let mut _NET_WM_STATE_FOCUSED_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(c"_NET_WM_STATE_FOCUSED".as_ptr()) as uint16_t,
        c"_NET_WM_STATE_FOCUSED".as_ptr(),
    );
    let mut _NET_WM_STATE_MAXIMIZED_VERT_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(c"_NET_WM_STATE_MAXIMIZED_VERT".as_ptr()) as uint16_t,
        c"_NET_WM_STATE_MAXIMIZED_VERT".as_ptr(),
    );
    let mut _NET_WM_STATE_MAXIMIZED_HORZ_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(c"_NET_WM_STATE_MAXIMIZED_HORZ".as_ptr()) as uint16_t,
        c"_NET_WM_STATE_MAXIMIZED_HORZ".as_ptr(),
    );
    let mut _NET_WM_STATE_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(c"_NET_WM_STATE".as_ptr()) as uint16_t,
        c"_NET_WM_STATE".as_ptr(),
    );
    let mut _NET_WM_WINDOW_TYPE_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(c"_NET_WM_WINDOW_TYPE".as_ptr()) as uint16_t,
        c"_NET_WM_WINDOW_TYPE".as_ptr(),
    );
    let mut _NET_WM_WINDOW_TYPE_NORMAL_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(c"_NET_WM_WINDOW_TYPE_NORMAL".as_ptr()) as uint16_t,
        c"_NET_WM_WINDOW_TYPE_NORMAL".as_ptr(),
    );
    let mut _NET_WM_WINDOW_TYPE_DOCK_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(c"_NET_WM_WINDOW_TYPE_DOCK".as_ptr()) as uint16_t,
        c"_NET_WM_WINDOW_TYPE_DOCK".as_ptr(),
    );
    let mut _NET_WM_WINDOW_TYPE_DIALOG_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(c"_NET_WM_WINDOW_TYPE_DIALOG".as_ptr()) as uint16_t,
        c"_NET_WM_WINDOW_TYPE_DIALOG".as_ptr(),
    );
    let mut _NET_WM_WINDOW_TYPE_UTILITY_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(c"_NET_WM_WINDOW_TYPE_UTILITY".as_ptr()) as uint16_t,
        c"_NET_WM_WINDOW_TYPE_UTILITY".as_ptr(),
    );
    let mut _NET_WM_WINDOW_TYPE_TOOLBAR_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(c"_NET_WM_WINDOW_TYPE_TOOLBAR".as_ptr()) as uint16_t,
        c"_NET_WM_WINDOW_TYPE_TOOLBAR".as_ptr(),
    );
    let mut _NET_WM_WINDOW_TYPE_SPLASH_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(c"_NET_WM_WINDOW_TYPE_SPLASH".as_ptr()) as uint16_t,
        c"_NET_WM_WINDOW_TYPE_SPLASH".as_ptr(),
    );
    let mut _NET_WM_WINDOW_TYPE_MENU_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(c"_NET_WM_WINDOW_TYPE_MENU".as_ptr()) as uint16_t,
        c"_NET_WM_WINDOW_TYPE_MENU".as_ptr(),
    );
    let mut _NET_WM_WINDOW_TYPE_DROPDOWN_MENU_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(c"_NET_WM_WINDOW_TYPE_DROPDOWN_MENU".as_ptr()) as uint16_t,
        c"_NET_WM_WINDOW_TYPE_DROPDOWN_MENU".as_ptr(),
    );
    let mut _NET_WM_WINDOW_TYPE_POPUP_MENU_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(c"_NET_WM_WINDOW_TYPE_POPUP_MENU".as_ptr()) as uint16_t,
        c"_NET_WM_WINDOW_TYPE_POPUP_MENU".as_ptr(),
    );
    let mut _NET_WM_WINDOW_TYPE_TOOLTIP_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(c"_NET_WM_WINDOW_TYPE_TOOLTIP".as_ptr()) as uint16_t,
        c"_NET_WM_WINDOW_TYPE_TOOLTIP".as_ptr(),
    );
    let mut _NET_WM_WINDOW_TYPE_NOTIFICATION_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(c"_NET_WM_WINDOW_TYPE_NOTIFICATION".as_ptr()) as uint16_t,
        c"_NET_WM_WINDOW_TYPE_NOTIFICATION".as_ptr(),
    );
    let mut _NET_WM_DESKTOP_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(c"_NET_WM_DESKTOP".as_ptr()) as uint16_t,
        c"_NET_WM_DESKTOP".as_ptr(),
    );
    let mut _NET_WM_STRUT_PARTIAL_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(c"_NET_WM_STRUT_PARTIAL".as_ptr()) as uint16_t,
        c"_NET_WM_STRUT_PARTIAL".as_ptr(),
    );
    let mut _NET_CLIENT_LIST_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(c"_NET_CLIENT_LIST".as_ptr()) as uint16_t,
        c"_NET_CLIENT_LIST".as_ptr(),
    );
    let mut _NET_CLIENT_LIST_STACKING_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(c"_NET_CLIENT_LIST_STACKING".as_ptr()) as uint16_t,
        c"_NET_CLIENT_LIST_STACKING".as_ptr(),
    );
    let mut _NET_CURRENT_DESKTOP_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(c"_NET_CURRENT_DESKTOP".as_ptr()) as uint16_t,
        c"_NET_CURRENT_DESKTOP".as_ptr(),
    );
    let mut _NET_NUMBER_OF_DESKTOPS_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(c"_NET_NUMBER_OF_DESKTOPS".as_ptr()) as uint16_t,
        c"_NET_NUMBER_OF_DESKTOPS".as_ptr(),
    );
    let mut _NET_DESKTOP_NAMES_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(c"_NET_DESKTOP_NAMES".as_ptr()) as uint16_t,
        c"_NET_DESKTOP_NAMES".as_ptr(),
    );
    let mut _NET_DESKTOP_VIEWPORT_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(c"_NET_DESKTOP_VIEWPORT".as_ptr()) as uint16_t,
        c"_NET_DESKTOP_VIEWPORT".as_ptr(),
    );
    let mut _NET_ACTIVE_WINDOW_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(c"_NET_ACTIVE_WINDOW".as_ptr()) as uint16_t,
        c"_NET_ACTIVE_WINDOW".as_ptr(),
    );
    let mut _NET_CLOSE_WINDOW_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(c"_NET_CLOSE_WINDOW".as_ptr()) as uint16_t,
        c"_NET_CLOSE_WINDOW".as_ptr(),
    );
    let mut _NET_MOVERESIZE_WINDOW_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(c"_NET_MOVERESIZE_WINDOW".as_ptr()) as uint16_t,
        c"_NET_MOVERESIZE_WINDOW".as_ptr(),
    );
    let mut _NET_WM_USER_TIME_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(c"_NET_WM_USER_TIME".as_ptr()) as uint16_t,
        c"_NET_WM_USER_TIME".as_ptr(),
    );
    let mut _NET_STARTUP_ID_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(c"_NET_STARTUP_ID".as_ptr()) as uint16_t,
        c"_NET_STARTUP_ID".as_ptr(),
    );
    let mut _NET_WORKAREA_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(c"_NET_WORKAREA".as_ptr()) as uint16_t,
        c"_NET_WORKAREA".as_ptr(),
    );
    let mut _NET_WM_ICON_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(c"_NET_WM_ICON".as_ptr()) as uint16_t,
        c"_NET_WM_ICON".as_ptr(),
    );
    let WM_PROTOCOLS_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(c"WM_PROTOCOLS".as_ptr()) as uint16_t,
        c"WM_PROTOCOLS".as_ptr(),
    );
    let WM_DELETE_WINDOW_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(c"WM_DELETE_WINDOW".as_ptr()) as uint16_t,
        c"WM_DELETE_WINDOW".as_ptr(),
    );
    let UTF8_STRING_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(c"UTF8_STRING".as_ptr()) as uint16_t,
        c"UTF8_STRING".as_ptr(),
    );
    let WM_STATE_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(c"WM_STATE".as_ptr()) as uint16_t,
        c"WM_STATE".as_ptr(),
    );
    let WM_CLIENT_LEADER_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(c"WM_CLIENT_LEADER".as_ptr()) as uint16_t,
        c"WM_CLIENT_LEADER".as_ptr(),
    );
    let WM_TAKE_FOCUS_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(c"WM_TAKE_FOCUS".as_ptr()) as uint16_t,
        c"WM_TAKE_FOCUS".as_ptr(),
    );
    let WM_WINDOW_ROLE_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(c"WM_WINDOW_ROLE".as_ptr()) as uint16_t,
        c"WM_WINDOW_ROLE".as_ptr(),
    );
    let I3_SOCKET_PATH_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(c"I3_SOCKET_PATH".as_ptr()) as uint16_t,
        c"I3_SOCKET_PATH".as_ptr(),
    );
    let I3_CONFIG_PATH_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(c"I3_CONFIG_PATH".as_ptr()) as uint16_t,
        c"I3_CONFIG_PATH".as_ptr(),
    );
    let I3_SYNC_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(c"I3_SYNC".as_ptr()) as uint16_t,
        c"I3_SYNC".as_ptr(),
    );
    let I3_SHMLOG_PATH_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(c"I3_SHMLOG_PATH".as_ptr()) as uint16_t,
        c"I3_SHMLOG_PATH".as_ptr(),
    );
    let I3_PID_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(c"I3_PID".as_ptr()) as uint16_t,
        c"I3_PID".as_ptr(),
    );
    let I3_LOG_STREAM_SOCKET_PATH_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(c"I3_LOG_STREAM_SOCKET_PATH".as_ptr()) as uint16_t,
        c"I3_LOG_STREAM_SOCKET_PATH".as_ptr(),
    );
    let I3_FLOATING_WINDOW_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(c"I3_FLOATING_WINDOW".as_ptr()) as uint16_t,
        c"I3_FLOATING_WINDOW".as_ptr(),
    );
    let mut _NET_REQUEST_FRAME_EXTENTS_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(c"_NET_REQUEST_FRAME_EXTENTS".as_ptr()) as uint16_t,
        c"_NET_REQUEST_FRAME_EXTENTS".as_ptr(),
    );
    let mut _NET_FRAME_EXTENTS_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(c"_NET_FRAME_EXTENTS".as_ptr()) as uint16_t,
        c"_NET_FRAME_EXTENTS".as_ptr(),
    );
    let mut _MOTIF_WM_HINTS_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(c"_MOTIF_WM_HINTS".as_ptr()) as uint16_t,
        c"_MOTIF_WM_HINTS".as_ptr(),
    );
    let WM_CHANGE_STATE_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(c"WM_CHANGE_STATE".as_ptr()) as uint16_t,
        c"WM_CHANGE_STATE".as_ptr(),
    );
    let MANAGER_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(c"MANAGER".as_ptr()) as uint16_t,
        c"MANAGER".as_ptr(),
    );
    root_depth = (*root_screen).root_depth;
    colormap = (*root_screen).default_colormap;
    visual_type = xcb_aux_find_visual_by_attrs(
        root_screen,
        -(1 as ::core::ffi::c_int) as int8_t,
        32 as int8_t,
    );
    if !visual_type.is_null() {
        root_depth = xcb_aux_get_depth_of_visual(root_screen, (*visual_type).visual_id);
        colormap = xcb_generate_id(conn) as xcb_colormap_t;
        let cm_cookie: xcb_void_cookie_t = xcb_create_colormap_checked(
            conn,
            XCB_COLORMAP_ALLOC_NONE as ::core::ffi::c_int as uint8_t,
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
        event = xcb_wait_for_event(conn);
        if event.is_null() {
            break;
        }
        if (*event).response_type as ::core::ffi::c_int == XCB_PROPERTY_NOTIFY {
            last_timestamp = (*(event as *mut xcb_property_notify_event_t)).time;
            free(event as *mut ::core::ffi::c_void);
            break;
        } else {
            free(event as *mut ::core::ffi::c_void);
        }
    }
    debuglog(
        c"%s:%s:%d - got timestamp %d\n".as_ptr().cast_mut(),
        c"../src/main.c".as_ptr(),
        c"main".as_ptr(),
        669 as ::core::ffi::c_int,
        last_timestamp,
    );
    let reply_0: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_SUPPORTED_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_0.is_null() {
        errorlog(
            c"ERROR: Could not get atom _NET_SUPPORTED\n"
                .as_ptr()
                .cast_mut(),
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_SUPPORTED = (*reply_0).atom;
    free(reply_0 as *mut ::core::ffi::c_void);
    let reply_1: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_SUPPORTING_WM_CHECK_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_1.is_null() {
        errorlog(
            c"ERROR: Could not get atom _NET_SUPPORTING_WM_CHECK\n"
                .as_ptr()
                .cast_mut(),
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_SUPPORTING_WM_CHECK = (*reply_1).atom;
    free(reply_1 as *mut ::core::ffi::c_void);
    let reply_2: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_WM_NAME_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_2.is_null() {
        errorlog(
            c"ERROR: Could not get atom _NET_WM_NAME\n"
                .as_ptr()
                .cast_mut(),
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_WM_NAME = (*reply_2).atom;
    free(reply_2 as *mut ::core::ffi::c_void);
    let reply_3: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_WM_VISIBLE_NAME_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_3.is_null() {
        errorlog(
            c"ERROR: Could not get atom _NET_WM_VISIBLE_NAME\n"
                .as_ptr()
                .cast_mut(),
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_WM_VISIBLE_NAME = (*reply_3).atom;
    free(reply_3 as *mut ::core::ffi::c_void);
    let reply_4: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_WM_MOVERESIZE_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_4.is_null() {
        errorlog(
            c"ERROR: Could not get atom _NET_WM_MOVERESIZE\n"
                .as_ptr()
                .cast_mut(),
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_WM_MOVERESIZE = (*reply_4).atom;
    free(reply_4 as *mut ::core::ffi::c_void);
    let reply_5: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_WM_STATE_STICKY_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_5.is_null() {
        errorlog(
            c"ERROR: Could not get atom _NET_WM_STATE_STICKY\n"
                .as_ptr()
                .cast_mut(),
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_WM_STATE_STICKY = (*reply_5).atom;
    free(reply_5 as *mut ::core::ffi::c_void);
    let reply_6: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_WM_STATE_FULLSCREEN_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_6.is_null() {
        errorlog(
            c"ERROR: Could not get atom _NET_WM_STATE_FULLSCREEN\n"
                .as_ptr()
                .cast_mut(),
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_WM_STATE_FULLSCREEN = (*reply_6).atom;
    free(reply_6 as *mut ::core::ffi::c_void);
    let reply_7: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_WM_STATE_DEMANDS_ATTENTION_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_7.is_null() {
        errorlog(
            c"ERROR: Could not get atom _NET_WM_STATE_DEMANDS_ATTENTION\n"
                .as_ptr()
                .cast_mut(),
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_WM_STATE_DEMANDS_ATTENTION = (*reply_7).atom;
    free(reply_7 as *mut ::core::ffi::c_void);
    let reply_8: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_WM_STATE_MODAL_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_8.is_null() {
        errorlog(
            c"ERROR: Could not get atom _NET_WM_STATE_MODAL\n"
                .as_ptr()
                .cast_mut(),
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_WM_STATE_MODAL = (*reply_8).atom;
    free(reply_8 as *mut ::core::ffi::c_void);
    let reply_9: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_WM_STATE_HIDDEN_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_9.is_null() {
        errorlog(
            c"ERROR: Could not get atom _NET_WM_STATE_HIDDEN\n"
                .as_ptr()
                .cast_mut(),
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_WM_STATE_HIDDEN = (*reply_9).atom;
    free(reply_9 as *mut ::core::ffi::c_void);
    let reply_10: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_WM_STATE_FOCUSED_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_10.is_null() {
        errorlog(
            c"ERROR: Could not get atom _NET_WM_STATE_FOCUSED\n"
                .as_ptr()
                .cast_mut(),
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_WM_STATE_FOCUSED = (*reply_10).atom;
    free(reply_10 as *mut ::core::ffi::c_void);
    let reply_11: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_WM_STATE_MAXIMIZED_VERT_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_11.is_null() {
        errorlog(
            c"ERROR: Could not get atom _NET_WM_STATE_MAXIMIZED_VERT\n"
                .as_ptr()
                .cast_mut(),
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_WM_STATE_MAXIMIZED_VERT = (*reply_11).atom;
    free(reply_11 as *mut ::core::ffi::c_void);
    let reply_12: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_WM_STATE_MAXIMIZED_HORZ_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_12.is_null() {
        errorlog(
            c"ERROR: Could not get atom _NET_WM_STATE_MAXIMIZED_HORZ\n"
                .as_ptr()
                .cast_mut(),
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_WM_STATE_MAXIMIZED_HORZ = (*reply_12).atom;
    free(reply_12 as *mut ::core::ffi::c_void);
    let reply_13: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_WM_STATE_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_13.is_null() {
        errorlog(
            c"ERROR: Could not get atom _NET_WM_STATE\n"
                .as_ptr()
                .cast_mut(),
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_WM_STATE = (*reply_13).atom;
    free(reply_13 as *mut ::core::ffi::c_void);
    let reply_14: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_WM_WINDOW_TYPE_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_14.is_null() {
        errorlog(
            c"ERROR: Could not get atom _NET_WM_WINDOW_TYPE\n"
                .as_ptr()
                .cast_mut(),
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_WM_WINDOW_TYPE = (*reply_14).atom;
    free(reply_14 as *mut ::core::ffi::c_void);
    let reply_15: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_WM_WINDOW_TYPE_NORMAL_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_15.is_null() {
        errorlog(
            c"ERROR: Could not get atom _NET_WM_WINDOW_TYPE_NORMAL\n"
                .as_ptr()
                .cast_mut(),
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_WM_WINDOW_TYPE_NORMAL = (*reply_15).atom;
    free(reply_15 as *mut ::core::ffi::c_void);
    let reply_16: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_WM_WINDOW_TYPE_DOCK_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_16.is_null() {
        errorlog(
            c"ERROR: Could not get atom _NET_WM_WINDOW_TYPE_DOCK\n"
                .as_ptr()
                .cast_mut(),
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_WM_WINDOW_TYPE_DOCK = (*reply_16).atom;
    free(reply_16 as *mut ::core::ffi::c_void);
    let reply_17: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_WM_WINDOW_TYPE_DIALOG_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_17.is_null() {
        errorlog(
            c"ERROR: Could not get atom _NET_WM_WINDOW_TYPE_DIALOG\n"
                .as_ptr()
                .cast_mut(),
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_WM_WINDOW_TYPE_DIALOG = (*reply_17).atom;
    free(reply_17 as *mut ::core::ffi::c_void);
    let reply_18: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_WM_WINDOW_TYPE_UTILITY_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_18.is_null() {
        errorlog(
            c"ERROR: Could not get atom _NET_WM_WINDOW_TYPE_UTILITY\n"
                .as_ptr()
                .cast_mut(),
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_WM_WINDOW_TYPE_UTILITY = (*reply_18).atom;
    free(reply_18 as *mut ::core::ffi::c_void);
    let reply_19: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_WM_WINDOW_TYPE_TOOLBAR_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_19.is_null() {
        errorlog(
            c"ERROR: Could not get atom _NET_WM_WINDOW_TYPE_TOOLBAR\n"
                .as_ptr()
                .cast_mut(),
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_WM_WINDOW_TYPE_TOOLBAR = (*reply_19).atom;
    free(reply_19 as *mut ::core::ffi::c_void);
    let reply_20: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_WM_WINDOW_TYPE_SPLASH_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_20.is_null() {
        errorlog(
            c"ERROR: Could not get atom _NET_WM_WINDOW_TYPE_SPLASH\n"
                .as_ptr()
                .cast_mut(),
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_WM_WINDOW_TYPE_SPLASH = (*reply_20).atom;
    free(reply_20 as *mut ::core::ffi::c_void);
    let reply_21: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_WM_WINDOW_TYPE_MENU_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_21.is_null() {
        errorlog(
            c"ERROR: Could not get atom _NET_WM_WINDOW_TYPE_MENU\n"
                .as_ptr()
                .cast_mut(),
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_WM_WINDOW_TYPE_MENU = (*reply_21).atom;
    free(reply_21 as *mut ::core::ffi::c_void);
    let reply_22: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_WM_WINDOW_TYPE_DROPDOWN_MENU_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_22.is_null() {
        errorlog(
            c"ERROR: Could not get atom _NET_WM_WINDOW_TYPE_DROPDOWN_MENU\n"
                .as_ptr()
                .cast_mut(),
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_WM_WINDOW_TYPE_DROPDOWN_MENU = (*reply_22).atom;
    free(reply_22 as *mut ::core::ffi::c_void);
    let reply_23: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_WM_WINDOW_TYPE_POPUP_MENU_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_23.is_null() {
        errorlog(
            c"ERROR: Could not get atom _NET_WM_WINDOW_TYPE_POPUP_MENU\n"
                .as_ptr()
                .cast_mut(),
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_WM_WINDOW_TYPE_POPUP_MENU = (*reply_23).atom;
    free(reply_23 as *mut ::core::ffi::c_void);
    let reply_24: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_WM_WINDOW_TYPE_TOOLTIP_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_24.is_null() {
        errorlog(
            c"ERROR: Could not get atom _NET_WM_WINDOW_TYPE_TOOLTIP\n"
                .as_ptr()
                .cast_mut(),
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_WM_WINDOW_TYPE_TOOLTIP = (*reply_24).atom;
    free(reply_24 as *mut ::core::ffi::c_void);
    let reply_25: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_WM_WINDOW_TYPE_NOTIFICATION_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_25.is_null() {
        errorlog(
            c"ERROR: Could not get atom _NET_WM_WINDOW_TYPE_NOTIFICATION\n"
                .as_ptr()
                .cast_mut(),
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_WM_WINDOW_TYPE_NOTIFICATION = (*reply_25).atom;
    free(reply_25 as *mut ::core::ffi::c_void);
    let reply_26: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_WM_DESKTOP_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_26.is_null() {
        errorlog(
            c"ERROR: Could not get atom _NET_WM_DESKTOP\n"
                .as_ptr()
                .cast_mut(),
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_WM_DESKTOP = (*reply_26).atom;
    free(reply_26 as *mut ::core::ffi::c_void);
    let reply_27: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_WM_STRUT_PARTIAL_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_27.is_null() {
        errorlog(
            c"ERROR: Could not get atom _NET_WM_STRUT_PARTIAL\n"
                .as_ptr()
                .cast_mut(),
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_WM_STRUT_PARTIAL = (*reply_27).atom;
    free(reply_27 as *mut ::core::ffi::c_void);
    let reply_28: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_CLIENT_LIST_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_28.is_null() {
        errorlog(
            c"ERROR: Could not get atom _NET_CLIENT_LIST\n"
                .as_ptr()
                .cast_mut(),
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_CLIENT_LIST = (*reply_28).atom;
    free(reply_28 as *mut ::core::ffi::c_void);
    let reply_29: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_CLIENT_LIST_STACKING_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_29.is_null() {
        errorlog(
            c"ERROR: Could not get atom _NET_CLIENT_LIST_STACKING\n"
                .as_ptr()
                .cast_mut(),
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_CLIENT_LIST_STACKING = (*reply_29).atom;
    free(reply_29 as *mut ::core::ffi::c_void);
    let reply_30: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_CURRENT_DESKTOP_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_30.is_null() {
        errorlog(
            c"ERROR: Could not get atom _NET_CURRENT_DESKTOP\n"
                .as_ptr()
                .cast_mut(),
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_CURRENT_DESKTOP = (*reply_30).atom;
    free(reply_30 as *mut ::core::ffi::c_void);
    let reply_31: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_NUMBER_OF_DESKTOPS_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_31.is_null() {
        errorlog(
            c"ERROR: Could not get atom _NET_NUMBER_OF_DESKTOPS\n"
                .as_ptr()
                .cast_mut(),
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_NUMBER_OF_DESKTOPS = (*reply_31).atom;
    free(reply_31 as *mut ::core::ffi::c_void);
    let reply_32: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_DESKTOP_NAMES_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_32.is_null() {
        errorlog(
            c"ERROR: Could not get atom _NET_DESKTOP_NAMES\n"
                .as_ptr()
                .cast_mut(),
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_DESKTOP_NAMES = (*reply_32).atom;
    free(reply_32 as *mut ::core::ffi::c_void);
    let reply_33: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_DESKTOP_VIEWPORT_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_33.is_null() {
        errorlog(
            c"ERROR: Could not get atom _NET_DESKTOP_VIEWPORT\n"
                .as_ptr()
                .cast_mut(),
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_DESKTOP_VIEWPORT = (*reply_33).atom;
    free(reply_33 as *mut ::core::ffi::c_void);
    let reply_34: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_ACTIVE_WINDOW_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_34.is_null() {
        errorlog(
            c"ERROR: Could not get atom _NET_ACTIVE_WINDOW\n"
                .as_ptr()
                .cast_mut(),
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_ACTIVE_WINDOW = (*reply_34).atom;
    free(reply_34 as *mut ::core::ffi::c_void);
    let reply_35: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_CLOSE_WINDOW_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_35.is_null() {
        errorlog(
            c"ERROR: Could not get atom _NET_CLOSE_WINDOW\n"
                .as_ptr()
                .cast_mut(),
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_CLOSE_WINDOW = (*reply_35).atom;
    free(reply_35 as *mut ::core::ffi::c_void);
    let reply_36: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_MOVERESIZE_WINDOW_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_36.is_null() {
        errorlog(
            c"ERROR: Could not get atom _NET_MOVERESIZE_WINDOW\n"
                .as_ptr()
                .cast_mut(),
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_MOVERESIZE_WINDOW = (*reply_36).atom;
    free(reply_36 as *mut ::core::ffi::c_void);
    let reply_37: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_WM_USER_TIME_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_37.is_null() {
        errorlog(
            c"ERROR: Could not get atom _NET_WM_USER_TIME\n"
                .as_ptr()
                .cast_mut(),
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_WM_USER_TIME = (*reply_37).atom;
    free(reply_37 as *mut ::core::ffi::c_void);
    let reply_38: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_STARTUP_ID_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_38.is_null() {
        errorlog(
            c"ERROR: Could not get atom _NET_STARTUP_ID\n"
                .as_ptr()
                .cast_mut(),
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_STARTUP_ID = (*reply_38).atom;
    free(reply_38 as *mut ::core::ffi::c_void);
    let reply_39: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_WORKAREA_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_39.is_null() {
        errorlog(
            c"ERROR: Could not get atom _NET_WORKAREA\n"
                .as_ptr()
                .cast_mut(),
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_WORKAREA = (*reply_39).atom;
    free(reply_39 as *mut ::core::ffi::c_void);
    let reply_40: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_WM_ICON_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_40.is_null() {
        errorlog(
            c"ERROR: Could not get atom _NET_WM_ICON\n"
                .as_ptr()
                .cast_mut(),
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_WM_ICON = (*reply_40).atom;
    free(reply_40 as *mut ::core::ffi::c_void);
    let reply_41: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        WM_PROTOCOLS_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_41.is_null() {
        errorlog(
            c"ERROR: Could not get atom WM_PROTOCOLS\n"
                .as_ptr()
                .cast_mut(),
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A_WM_PROTOCOLS = (*reply_41).atom;
    free(reply_41 as *mut ::core::ffi::c_void);
    let reply_42: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        WM_DELETE_WINDOW_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_42.is_null() {
        errorlog(
            c"ERROR: Could not get atom WM_DELETE_WINDOW\n"
                .as_ptr()
                .cast_mut(),
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A_WM_DELETE_WINDOW = (*reply_42).atom;
    free(reply_42 as *mut ::core::ffi::c_void);
    let reply_43: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        UTF8_STRING_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_43.is_null() {
        errorlog(
            c"ERROR: Could not get atom UTF8_STRING\n"
                .as_ptr()
                .cast_mut(),
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A_UTF8_STRING = (*reply_43).atom;
    free(reply_43 as *mut ::core::ffi::c_void);
    let reply_44: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        WM_STATE_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_44.is_null() {
        errorlog(c"ERROR: Could not get atom WM_STATE\n".as_ptr().cast_mut());
        exit(-(1 as ::core::ffi::c_int));
    }
    A_WM_STATE = (*reply_44).atom;
    free(reply_44 as *mut ::core::ffi::c_void);
    let reply_45: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        WM_CLIENT_LEADER_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_45.is_null() {
        errorlog(
            c"ERROR: Could not get atom WM_CLIENT_LEADER\n"
                .as_ptr()
                .cast_mut(),
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A_WM_CLIENT_LEADER = (*reply_45).atom;
    free(reply_45 as *mut ::core::ffi::c_void);
    let reply_46: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        WM_TAKE_FOCUS_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_46.is_null() {
        errorlog(
            c"ERROR: Could not get atom WM_TAKE_FOCUS\n"
                .as_ptr()
                .cast_mut(),
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A_WM_TAKE_FOCUS = (*reply_46).atom;
    free(reply_46 as *mut ::core::ffi::c_void);
    let reply_47: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        WM_WINDOW_ROLE_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_47.is_null() {
        errorlog(
            c"ERROR: Could not get atom WM_WINDOW_ROLE\n"
                .as_ptr()
                .cast_mut(),
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A_WM_WINDOW_ROLE = (*reply_47).atom;
    free(reply_47 as *mut ::core::ffi::c_void);
    let reply_48: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        I3_SOCKET_PATH_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_48.is_null() {
        errorlog(
            c"ERROR: Could not get atom I3_SOCKET_PATH\n"
                .as_ptr()
                .cast_mut(),
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A_I3_SOCKET_PATH = (*reply_48).atom;
    free(reply_48 as *mut ::core::ffi::c_void);
    let reply_49: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        I3_CONFIG_PATH_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_49.is_null() {
        errorlog(
            c"ERROR: Could not get atom I3_CONFIG_PATH\n"
                .as_ptr()
                .cast_mut(),
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A_I3_CONFIG_PATH = (*reply_49).atom;
    free(reply_49 as *mut ::core::ffi::c_void);
    let reply_50: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        I3_SYNC_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_50.is_null() {
        errorlog(c"ERROR: Could not get atom I3_SYNC\n".as_ptr().cast_mut());
        exit(-(1 as ::core::ffi::c_int));
    }
    A_I3_SYNC = (*reply_50).atom;
    free(reply_50 as *mut ::core::ffi::c_void);
    let reply_51: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        I3_SHMLOG_PATH_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_51.is_null() {
        errorlog(
            c"ERROR: Could not get atom I3_SHMLOG_PATH\n"
                .as_ptr()
                .cast_mut(),
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A_I3_SHMLOG_PATH = (*reply_51).atom;
    free(reply_51 as *mut ::core::ffi::c_void);
    let reply_52: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        I3_PID_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_52.is_null() {
        errorlog(c"ERROR: Could not get atom I3_PID\n".as_ptr().cast_mut());
        exit(-(1 as ::core::ffi::c_int));
    }
    A_I3_PID = (*reply_52).atom;
    free(reply_52 as *mut ::core::ffi::c_void);
    let reply_53: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        I3_LOG_STREAM_SOCKET_PATH_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_53.is_null() {
        errorlog(
            c"ERROR: Could not get atom I3_LOG_STREAM_SOCKET_PATH\n"
                .as_ptr()
                .cast_mut(),
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A_I3_LOG_STREAM_SOCKET_PATH = (*reply_53).atom;
    free(reply_53 as *mut ::core::ffi::c_void);
    let reply_54: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        I3_FLOATING_WINDOW_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_54.is_null() {
        errorlog(
            c"ERROR: Could not get atom I3_FLOATING_WINDOW\n"
                .as_ptr()
                .cast_mut(),
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A_I3_FLOATING_WINDOW = (*reply_54).atom;
    free(reply_54 as *mut ::core::ffi::c_void);
    let reply_55: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_REQUEST_FRAME_EXTENTS_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_55.is_null() {
        errorlog(
            c"ERROR: Could not get atom _NET_REQUEST_FRAME_EXTENTS\n"
                .as_ptr()
                .cast_mut(),
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_REQUEST_FRAME_EXTENTS = (*reply_55).atom;
    free(reply_55 as *mut ::core::ffi::c_void);
    let reply_56: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_FRAME_EXTENTS_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_56.is_null() {
        errorlog(
            c"ERROR: Could not get atom _NET_FRAME_EXTENTS\n"
                .as_ptr()
                .cast_mut(),
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_FRAME_EXTENTS = (*reply_56).atom;
    free(reply_56 as *mut ::core::ffi::c_void);
    let reply_57: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _MOTIF_WM_HINTS_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_57.is_null() {
        errorlog(
            c"ERROR: Could not get atom _MOTIF_WM_HINTS\n"
                .as_ptr()
                .cast_mut(),
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__MOTIF_WM_HINTS = (*reply_57).atom;
    free(reply_57 as *mut ::core::ffi::c_void);
    let reply_58: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        WM_CHANGE_STATE_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_58.is_null() {
        errorlog(
            c"ERROR: Could not get atom WM_CHANGE_STATE\n"
                .as_ptr()
                .cast_mut(),
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A_WM_CHANGE_STATE = (*reply_58).atom;
    free(reply_58 as *mut ::core::ffi::c_void);
    let reply_59: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        MANAGER_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_59.is_null() {
        errorlog(c"ERROR: Could not get atom MANAGER\n".as_ptr().cast_mut());
        exit(-(1 as ::core::ffi::c_int));
    }
    A_MANAGER = (*reply_59).atom;
    free(reply_59 as *mut ::core::ffi::c_void);
    load_configuration(override_configpath, C_LOAD);
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
        xcb_atom_name_by_screen(c"WM".as_ptr(), conn_screen as uint8_t);
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
        xcb_intern_atom_unchecked(conn, 0 as uint8_t, strlen(atom_name) as uint16_t, atom_name),
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
        -(1 as ::core::ffi::c_int) as int16_t,
        -(1 as ::core::ffi::c_int) as int16_t,
        1 as uint16_t,
        1 as uint16_t,
        0 as uint16_t,
        XCB_WINDOW_CLASS_INPUT_OUTPUT as ::core::ffi::c_int as uint16_t,
        (*root_screen).root_visual,
        0 as uint32_t,
        ::core::ptr::null::<::core::ffi::c_void>(),
    );
    xcb_change_property(
        conn,
        XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as uint8_t,
        wm_sn_selection_owner,
        XCB_ATOM_WM_CLASS as ::core::ffi::c_int as xcb_atom_t,
        XCB_ATOM_STRING as ::core::ffi::c_int as xcb_atom_t,
        8 as uint8_t,
        strlen(c"i3-WM_Sn".as_ptr())
            .wrapping_add(1 as size_t)
            .wrapping_mul(2 as size_t) as uint32_t,
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
            usleep(usleep_time as __useconds_t);
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
    free(selection_reply as *mut ::core::ffi::c_void);
    let mut event_0: C2RustUnnamed_49 = C2RustUnnamed_49 {
        message: xcb_client_message_event_t {
            response_type: 0 as uint8_t,
            format: 0,
            sequence: 0,
            window: 0,
            type_0: 0,
            data: xcb_client_message_data_t { data8: [0; 20] },
        },
    };
    event_0.message.response_type = XCB_CLIENT_MESSAGE as uint8_t;
    event_0.message.window = (*root_screen).root;
    event_0.message.format = 32 as uint8_t;
    event_0.message.type_0 = A_MANAGER;
    event_0.message.data.data32[0 as ::core::ffi::c_int as usize] = last_timestamp as uint32_t;
    event_0.message.data.data32[1 as ::core::ffi::c_int as usize] = wm_sn as uint32_t;
    event_0.message.data.data32[2 as ::core::ffi::c_int as usize] =
        wm_sn_selection_owner as uint32_t;
    xcb_send_event(
        conn,
        0 as uint8_t,
        (*root_screen).root,
        XCB_EVENT_MASK_STRUCTURE_NOTIFY as ::core::ffi::c_int as uint32_t,
        &raw mut event_0.storage as *mut ::core::ffi::c_char,
    );
    let mut cookie: xcb_void_cookie_t = xcb_void_cookie_t { sequence: 0 };
    let mut fresh8: [uint32_t; 1] = [(XCB_EVENT_MASK_SUBSTRUCTURE_REDIRECT as ::core::ffi::c_int
        | XCB_EVENT_MASK_BUTTON_PRESS as ::core::ffi::c_int
        | XCB_EVENT_MASK_STRUCTURE_NOTIFY as ::core::ffi::c_int
        | XCB_EVENT_MASK_POINTER_MOTION as ::core::ffi::c_int
        | XCB_EVENT_MASK_PROPERTY_CHANGE as ::core::ffi::c_int
        | XCB_EVENT_MASK_FOCUS_CHANGE as ::core::ffi::c_int
        | XCB_EVENT_MASK_ENTER_WINDOW as ::core::ffi::c_int)
        as uint32_t];
    cookie = xcb_change_window_attributes_checked(
        conn,
        root,
        XCB_CW_EVENT_MASK as ::core::ffi::c_int as uint32_t,
        &raw mut fresh8 as *mut uint32_t as *const ::core::ffi::c_void,
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
    xcursor_set_root_cursor(XCURSOR_CURSOR_POINTER as ::core::ffi::c_int);
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
            XCB_XKB_MAJOR_VERSION as uint16_t,
            XCB_XKB_MINOR_VERSION as uint16_t,
        );
        xcb_xkb_select_events(
            conn,
            XCB_XKB_ID_USE_CORE_KBD as ::core::ffi::c_int as xcb_xkb_device_spec_t,
            (XCB_XKB_EVENT_TYPE_STATE_NOTIFY as ::core::ffi::c_int
                | XCB_XKB_EVENT_TYPE_MAP_NOTIFY as ::core::ffi::c_int
                | XCB_XKB_EVENT_TYPE_NEW_KEYBOARD_NOTIFY as ::core::ffi::c_int)
                as uint16_t,
            0 as uint16_t,
            (XCB_XKB_EVENT_TYPE_STATE_NOTIFY as ::core::ffi::c_int
                | XCB_XKB_EVENT_TYPE_MAP_NOTIFY as ::core::ffi::c_int
                | XCB_XKB_EVENT_TYPE_NEW_KEYBOARD_NOTIFY as ::core::ffi::c_int)
                as uint16_t,
            0xff as uint16_t,
            0xff as uint16_t,
            ::core::ptr::null::<::core::ffi::c_void>(),
        );
        let mask: uint32_t = (XCB_XKB_PER_CLIENT_FLAG_GRABS_USE_XKB_STATE as ::core::ffi::c_int
            | XCB_XKB_PER_CLIENT_FLAG_LOOKUP_STATE_WHEN_GRABBED as ::core::ffi::c_int
            | XCB_XKB_PER_CLIENT_FLAG_DETECTABLE_AUTO_REPEAT as ::core::ffi::c_int)
            as uint32_t;
        let mut pcf_reply: *mut xcb_xkb_per_client_flags_reply_t =
            ::core::ptr::null_mut::<xcb_xkb_per_client_flags_reply_t>();
        pcf_reply = xcb_xkb_per_client_flags_reply(
            conn,
            xcb_xkb_per_client_flags(
                conn,
                XCB_XKB_ID_USE_CORE_KBD as ::core::ffi::c_int as xcb_xkb_device_spec_t,
                mask,
                mask,
                0 as uint32_t,
                0 as uint32_t,
                0 as uint32_t,
            ),
            ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
        );
        if pcf_reply.is_null()
            || (*pcf_reply).value
                & XCB_XKB_PER_CLIENT_FLAG_GRABS_USE_XKB_STATE as ::core::ffi::c_int as uint32_t
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
                & XCB_XKB_PER_CLIENT_FLAG_LOOKUP_STATE_WHEN_GRABBED as ::core::ffi::c_int
                    as uint32_t
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
                & XCB_XKB_PER_CLIENT_FLAG_DETECTABLE_AUTO_REPEAT as ::core::ffi::c_int as uint32_t
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
    xcb_numlock_mask =
        aio_get_mod_mask_for(XCB_NUM_LOCK as uint32_t, keysyms) as ::core::ffi::c_uint;
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
            let dir: *const ::core::ffi::c_char = dirname(layout_path);
            rmdir(dir);
        }
    }
    if needs_tree_init {
        tree_init(greply);
    }
    free(greply as *mut ::core::ffi::c_void);
    if fake_outputs.is_null() && !config.fake_outputs.is_null() {
        fake_outputs = config.fake_outputs;
    }
    if !fake_outputs.is_null() {
        fake_outputs_init(fake_outputs);
        free(fake_outputs as *mut ::core::ffi::c_void);
        fake_outputs = ::core::ptr::null_mut::<::core::ffi::c_char>();
        config.fake_outputs = ::core::ptr::null_mut::<::core::ffi::c_char>();
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
    free(layout_path as *mut ::core::ffi::c_void);
    layout_path = ::core::ptr::null_mut::<::core::ffi::c_char>();
    scratchpad_fix_resolution();
    let mut output_0: *mut Output = ::core::ptr::null_mut::<Output>();
    let pointer_reply: *mut xcb_query_pointer_reply_t = xcb_query_pointer_reply(
        conn,
        pointercookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
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
    free(pointer_reply as *mut ::core::ffi::c_void);
    tree_render();
    let mut ipc_io: *mut ev_io =
        scalloc(1 as size_t, ::core::mem::size_of::<ev_io>() as size_t) as *mut ev_io;
    let ref mut fresh9 = (*(ipc_io as *mut ::core::ffi::c_void as *mut ev_watcher)).pending;
    *fresh9 = 0 as ::core::ffi::c_int;
    (*(ipc_io as *mut ::core::ffi::c_void as *mut ev_watcher)).active = *fresh9;
    (*(ipc_io as *mut ::core::ffi::c_void as *mut ev_watcher)).priority = 0 as ::core::ffi::c_int;
    (*ipc_io).cb = Some(
        ipc_new_client as unsafe extern "C" fn(*mut ev_loop, *mut ev_io, ::core::ffi::c_int) -> (),
    )
        as Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_io, ::core::ffi::c_int) -> ()>;
    memmove(
        &raw mut (*(ipc_io as *mut ev_watcher)).cb as *mut ::core::ffi::c_void,
        &raw mut (*ipc_io).cb as *const ::core::ffi::c_void,
        ::core::mem::size_of::<
            Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_io, ::core::ffi::c_int) -> ()>,
        >() as size_t,
    );
    (*ipc_io).fd = ipc_socket;
    (*ipc_io).events = EV_READ as ::core::ffi::c_int | EV__IOFDSET as ::core::ffi::c_int;
    ev_io_start(main_loop, ipc_io);
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
        log_io = scalloc(1 as size_t, ::core::mem::size_of::<ev_io>() as size_t) as *mut ev_io;
        let ref mut fresh10 = (*(log_io as *mut ::core::ffi::c_void as *mut ev_watcher)).pending;
        *fresh10 = 0 as ::core::ffi::c_int;
        (*(log_io as *mut ::core::ffi::c_void as *mut ev_watcher)).active = *fresh10;
        (*(log_io as *mut ::core::ffi::c_void as *mut ev_watcher)).priority =
            0 as ::core::ffi::c_int;
        (*log_io).cb = Some(
            log_new_client
                as unsafe extern "C" fn(*mut ev_loop, *mut ev_io, ::core::ffi::c_int) -> (),
        )
            as Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_io, ::core::ffi::c_int) -> ()>;
        memmove(
            &raw mut (*(log_io as *mut ev_watcher)).cb as *mut ::core::ffi::c_void,
            &raw mut (*log_io).cb as *const ::core::ffi::c_void,
            ::core::mem::size_of::<
                Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_io, ::core::ffi::c_int) -> ()>,
            >() as size_t,
        );
        (*log_io).fd = log_socket;
        (*log_io).events = EV_READ as ::core::ffi::c_int | EV__IOFDSET as ::core::ffi::c_int;
        ev_io_start(main_loop, log_io);
    }
    let mut socket_ipc_io: *mut ev_io = ::core::ptr::null_mut::<ev_io>();
    listen_fds = sd_listen_fds(0 as ::core::ffi::c_int);
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
            socket_ipc_io =
                scalloc(1 as size_t, ::core::mem::size_of::<ev_io>() as size_t) as *mut ev_io;
            let ref mut fresh11 =
                (*(socket_ipc_io as *mut ::core::ffi::c_void as *mut ev_watcher)).pending;
            *fresh11 = 0 as ::core::ffi::c_int;
            (*(socket_ipc_io as *mut ::core::ffi::c_void as *mut ev_watcher)).active = *fresh11;
            (*(socket_ipc_io as *mut ::core::ffi::c_void as *mut ev_watcher)).priority =
                0 as ::core::ffi::c_int;
            (*socket_ipc_io).cb = Some(
                ipc_new_client
                    as unsafe extern "C" fn(*mut ev_loop, *mut ev_io, ::core::ffi::c_int) -> (),
            )
                as Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_io, ::core::ffi::c_int) -> ()>;
            memmove(
                &raw mut (*(socket_ipc_io as *mut ev_watcher)).cb as *mut ::core::ffi::c_void,
                &raw mut (*socket_ipc_io).cb as *const ::core::ffi::c_void,
                ::core::mem::size_of::<
                    Option<
                        unsafe extern "C" fn(*mut ev_loop, *mut ev_io, ::core::ffi::c_int) -> (),
                    >,
                >() as size_t,
            );
            (*socket_ipc_io).fd = fd;
            (*socket_ipc_io).events =
                EV_READ as ::core::ffi::c_int | EV__IOFDSET as ::core::ffi::c_int;
            ev_io_start(main_loop, socket_ipc_io);
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
    let mut xcb_watcher: *mut ev_io =
        scalloc(1 as size_t, ::core::mem::size_of::<ev_io>() as size_t) as *mut ev_io;
    xcb_prepare =
        scalloc(1 as size_t, ::core::mem::size_of::<ev_prepare>() as size_t) as *mut ev_prepare;
    let ref mut fresh12 = (*(xcb_watcher as *mut ::core::ffi::c_void as *mut ev_watcher)).pending;
    *fresh12 = 0 as ::core::ffi::c_int;
    (*(xcb_watcher as *mut ::core::ffi::c_void as *mut ev_watcher)).active = *fresh12;
    (*(xcb_watcher as *mut ::core::ffi::c_void as *mut ev_watcher)).priority =
        0 as ::core::ffi::c_int;
    (*xcb_watcher).cb = Some(
        xcb_got_event as unsafe extern "C" fn(*mut ev_loop, *mut ev_io, ::core::ffi::c_int) -> (),
    )
        as Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_io, ::core::ffi::c_int) -> ()>;
    memmove(
        &raw mut (*(xcb_watcher as *mut ev_watcher)).cb as *mut ::core::ffi::c_void,
        &raw mut (*xcb_watcher).cb as *const ::core::ffi::c_void,
        ::core::mem::size_of::<
            Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_io, ::core::ffi::c_int) -> ()>,
        >() as size_t,
    );
    (*xcb_watcher).fd = xcb_get_file_descriptor(conn);
    (*xcb_watcher).events = EV_READ as ::core::ffi::c_int | EV__IOFDSET as ::core::ffi::c_int;
    ev_io_start(main_loop, xcb_watcher);
    let ref mut fresh13 = (*(xcb_prepare as *mut ::core::ffi::c_void as *mut ev_watcher)).pending;
    *fresh13 = 0 as ::core::ffi::c_int;
    (*(xcb_prepare as *mut ::core::ffi::c_void as *mut ev_watcher)).active = *fresh13;
    (*(xcb_prepare as *mut ::core::ffi::c_void as *mut ev_watcher)).priority =
        0 as ::core::ffi::c_int;
    (*xcb_prepare).cb = Some(
        xcb_prepare_cb
            as unsafe extern "C" fn(*mut ev_loop, *mut ev_prepare, ::core::ffi::c_int) -> (),
    )
        as Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_prepare, ::core::ffi::c_int) -> ()>;
    memmove(
        &raw mut (*(xcb_prepare as *mut ev_watcher)).cb as *mut ::core::ffi::c_void,
        &raw mut (*xcb_prepare).cb as *const ::core::ffi::c_void,
        ::core::mem::size_of::<
            Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_prepare, ::core::ffi::c_int) -> ()>,
        >() as size_t,
    );
    ev_prepare_start(main_loop, xcb_prepare);
    xcb_flush(conn);
    xcb_grab_server(conn);
    xcb_aux_sync(conn);
    let mut event_1: *mut xcb_generic_event_t = ::core::ptr::null_mut::<xcb_generic_event_t>();
    loop {
        event_1 = xcb_poll_for_event(conn);
        if event_1.is_null() {
            break;
        }
        if (*event_1).response_type as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            free(event_1 as *mut ::core::ffi::c_void);
        } else {
            let type_0: ::core::ffi::c_int =
                (*event_1).response_type as ::core::ffi::c_int & 0x7f as ::core::ffi::c_int;
            if type_0 == XCB_MAP_REQUEST {
                handle_event(type_0, event_1);
            }
            free(event_1 as *mut ::core::ffi::c_void);
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
        let mut command: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        sasprintf(
            &raw mut command,
            c"%s %s --bar_id=%s --socket=\"%s\"".as_ptr(),
            if !(*barconfig).i3bar_command.is_null() {
                (*barconfig).i3bar_command as *const ::core::ffi::c_char
            } else {
                c"exec i3bar".as_ptr()
            },
            if (*barconfig).verbose as ::core::ffi::c_int != 0 {
                c"-V".as_ptr()
            } else {
                c"".as_ptr()
            },
            (*barconfig).id,
            current_socketpath,
        );
        verboselog(c"Starting bar process: %s\n".as_ptr().cast_mut(), command);
        start_application(command, true_0 != 0, XCB_NONE as xcb_window_t);
        free(command as *mut ::core::ffi::c_void);
        barconfig = (*barconfig).configs.tqe_next as *mut Barconfig;
    }
    atexit(i3_exit);
    raise(SIGCHLD);
    sd_notify(1 as ::core::ffi::c_int, c"READY=1".as_ptr());
    ev_loop(main_loop, 0 as ::core::ffi::c_int);
    free(ipc_io as *mut ::core::ffi::c_void);
    ipc_io = ::core::ptr::null_mut::<ev_io>();
    free(socket_ipc_io as *mut ::core::ffi::c_void);
    socket_ipc_io = ::core::ptr::null_mut::<ev_io>();
    free(log_io as *mut ::core::ffi::c_void);
    log_io = ::core::ptr::null_mut::<ev_io>();
    free(xcb_watcher as *mut ::core::ffi::c_void);
    xcb_watcher = ::core::ptr::null_mut::<ev_io>();
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
use crate::sd_daemon::sd_listen_fds;
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
pub use i3_common::string::I3String as _i3String;

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
