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
// Generated with C2Rust 0.22.1; C ABI and original ownership retained.
extern "C" {
    pub type xcb_connection_t;
    pub type _cairo;
    pub type _cairo_surface;
    pub type _i3String;
    pub type SnDisplay;
    pub type SnLauncherContext;
    pub type pcre2_real_code_8;
    pub type ev_loop;
    pub type SnMonitorEvent;
    pub type SnStartupSequence;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn __assert_single_arg(_: bool) -> bool;
    fn err(__status: ::core::ffi::c_int, __format: *const ::core::ffi::c_char, ...) -> !;
    fn time(__timer: *mut time_t) -> time_t;
    fn xcb_get_property(
        c: *mut xcb_connection_t,
        _delete: uint8_t,
        window: xcb_window_t,
        property: xcb_atom_t,
        type_0: xcb_atom_t,
        long_offset: uint32_t,
        long_length: uint32_t,
    ) -> xcb_get_property_cookie_t;
    fn xcb_get_property_value(R: *const xcb_get_property_reply_t) -> *mut ::core::ffi::c_void;
    fn xcb_get_property_value_length(R: *const xcb_get_property_reply_t) -> ::core::ffi::c_int;
    fn xcb_get_property_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_get_property_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_get_property_reply_t;
    fn signal(__sig: ::core::ffi::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn execl(
        __path: *const ::core::ffi::c_char,
        __arg: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn setsid() -> __pid_t;
    fn fork() -> __pid_t;
    static mut conn: *mut xcb_connection_t;
    fn verboselog(fmt: *mut ::core::ffi::c_char, ...);
    fn debuglog(fmt: *mut ::core::ffi::c_char, ...);
    fn scalloc(num: size_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn sstrdup(str: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn sasprintf(
        strp: *mut *mut ::core::ffi::c_char,
        fmt: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn sn_launcher_context_new(
        display: *mut SnDisplay,
        screen: ::core::ffi::c_int,
    ) -> *mut SnLauncherContext;
    fn sn_launcher_context_ref(context: *mut SnLauncherContext);
    fn sn_launcher_context_unref(context: *mut SnLauncherContext);
    fn sn_launcher_context_initiate(
        context: *mut SnLauncherContext,
        launcher_name: *const ::core::ffi::c_char,
        launchee_name: *const ::core::ffi::c_char,
        timestamp: Time,
    );
    fn sn_launcher_context_complete(context: *mut SnLauncherContext);
    fn sn_launcher_context_get_startup_id(
        context: *mut SnLauncherContext,
    ) -> *const ::core::ffi::c_char;
    fn sn_launcher_context_setup_child_process(context: *mut SnLauncherContext);
    fn sn_launcher_context_set_name(
        context: *mut SnLauncherContext,
        name: *const ::core::ffi::c_char,
    );
    fn sn_launcher_context_set_description(
        context: *mut SnLauncherContext,
        description: *const ::core::ffi::c_char,
    );
    fn ev_timer_start(loop_0: *mut ev_loop, w: *mut ev_timer);
    static mut focused: *mut Con;
    fn setrlimit(__resource: __rlimit_resource_t, __rlimits: *const rlimit) -> ::core::ffi::c_int;
    static mut original_rlimit_core: rlimit;
    static mut listen_fds: ::core::ffi::c_int;
    static mut conn_screen: ::core::ffi::c_int;
    static mut last_timestamp: xcb_timestamp_t;
    static mut sndisplay: *mut SnDisplay;
    static mut main_loop: *mut ev_loop;
    static mut current_socketpath: *mut ::core::ffi::c_char;
    fn xcursor_set_root_cursor(cursor_id: ::core::ffi::c_int);
    static mut A__NET_STARTUP_ID: xcb_atom_t;
    fn con_get_workspace(con: *mut Con) -> *mut Con;
    fn con_by_window_id(window: xcb_window_t) -> *mut Con;
    fn sn_monitor_event_get_type(event: *mut SnMonitorEvent) -> SnMonitorEventType;
    fn sn_monitor_event_get_startup_sequence(event: *mut SnMonitorEvent) -> *mut SnStartupSequence;
    fn sn_startup_sequence_get_id(sequence: *mut SnStartupSequence) -> *const ::core::ffi::c_char;
    fn memmove(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn setenv(
        __name: *const ::core::ffi::c_char,
        __value: *const ::core::ffi::c_char,
        __replace: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn unsetenv(__name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __pid_t = ::core::ffi::c_int;
pub type __rlim64_t = ::core::ffi::c_ulong;
pub type __time_t = ::core::ffi::c_long;
pub type __suseconds_t = ::core::ffi::c_long;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_generic_error_t {
    pub response_type: uint8_t,
    pub error_code: uint8_t,
    pub sequence: uint16_t,
    pub resource_id: uint32_t,
    pub minor_code: uint16_t,
    pub major_code: uint8_t,
    pub pad0: uint8_t,
    pub pad: [uint32_t; 5],
    pub full_sequence: uint32_t,
}
pub type xcb_window_t = uint32_t;
pub type xcb_gcontext_t = uint32_t;
pub type xcb_colormap_t = uint32_t;
pub type xcb_atom_t = uint32_t;
pub type xcb_drawable_t = uint32_t;
pub type xcb_timestamp_t = uint32_t;
pub type xcb_get_property_type_t = ::core::ffi::c_uint;
pub const XCB_GET_PROPERTY_TYPE_ANY: xcb_get_property_type_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_get_property_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_get_property_reply_t {
    pub response_type: uint8_t,
    pub format: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub type_0: xcb_atom_t,
    pub bytes_after: uint32_t,
    pub value_len: uint32_t,
    pub pad0: [uint8_t; 12],
}
pub type __sighandler_t = Option<unsafe extern "C" fn(::core::ffi::c_int) -> ()>;
pub type cairo_t = _cairo;
pub type cairo_surface_t = _cairo_surface;
pub type i3String = _i3String;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct color_t {
    pub red: ::core::ffi::c_double,
    pub green: ::core::ffi::c_double,
    pub blue: ::core::ffi::c_double,
    pub alpha: ::core::ffi::c_double,
    pub colorpixel: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct surface_t {
    pub id: xcb_drawable_t,
    pub gc: xcb_gcontext_t,
    pub owns_gc: bool,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub surface: *mut cairo_surface_t,
    pub cr: *mut cairo_t,
}
pub type Time = ::core::ffi::c_ulong;
pub type pcre2_code_8 = pcre2_real_code_8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Rect {
    pub x: uint32_t,
    pub y: uint32_t,
    pub width: uint32_t,
    pub height: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Con {
    pub mapped: bool,
    pub urgent: bool,
    pub ignore_unmap: uint8_t,
    pub frame: surface_t,
    pub frame_buffer: surface_t,
    pub pixmap_recreated: bool,
    pub type_0: C2RustUnnamed_15,
    pub num: ::core::ffi::c_int,
    pub gaps: gaps_t,
    pub parent: *mut Con,
    pub rect: Rect,
    pub window_rect: Rect,
    pub deco_rect: Rect,
    pub geometry: Rect,
    pub name: *mut ::core::ffi::c_char,
    pub title_format: *mut ::core::ffi::c_char,
    pub window_icon_padding: ::core::ffi::c_int,
    pub sticky_group: *mut ::core::ffi::c_char,
    pub marks_head: marks_head,
    pub mark_changed: bool,
    pub percent: ::core::ffi::c_double,
    pub border_width: ::core::ffi::c_int,
    pub current_border_width: ::core::ffi::c_int,
    pub window: *mut Window,
    pub urgency_timer: *mut ev_timer,
    pub deco_render_params: *mut deco_render_params,
    pub floating_head: floating_head,
    pub nodes_head: nodes_head,
    pub focus_head: focus_head,
    pub swallow_head: swallow_head,
    pub fullscreen_mode: fullscreen_mode_t,
    pub sticky: bool,
    pub layout: layout_t,
    pub last_split_layout: layout_t,
    pub workspace_layout: layout_t,
    pub border_style: border_style_t,
    pub max_user_border_style: border_style_t,
    pub floating: C2RustUnnamed_4,
    pub nodes: C2RustUnnamed_3,
    pub focused: C2RustUnnamed_2,
    pub all_cons: C2RustUnnamed_1,
    pub floating_windows: C2RustUnnamed_0,
    pub on_remove_child: Option<unsafe extern "C" fn(*mut Con) -> ()>,
    pub scratchpad_state: C2RustUnnamed,
    pub old_id: ::core::ffi::c_int,
    pub depth: uint16_t,
    pub colormap: xcb_colormap_t,
}
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const SCRATCHPAD_CHANGED: C2RustUnnamed = 2;
pub const SCRATCHPAD_FRESH: C2RustUnnamed = 1;
pub const SCRATCHPAD_NONE: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub tqe_next: *mut Con,
    pub tqe_prev: *mut *mut Con,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub tqe_next: *mut Con,
    pub tqe_prev: *mut *mut Con,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub tqe_next: *mut Con,
    pub tqe_prev: *mut *mut Con,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub tqe_next: *mut Con,
    pub tqe_prev: *mut *mut Con,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct swallow_head {
    pub tqh_first: *mut Match,
    pub tqh_last: *mut *mut Match,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Match {
    pub error: *mut ::core::ffi::c_char,
    pub title: *mut regex,
    pub application: *mut regex,
    pub class: *mut regex,
    pub instance: *mut regex,
    pub mark: *mut regex,
    pub window_role: *mut regex,
    pub workspace: *mut regex,
    pub machine: *mut regex,
    pub window_type: xcb_atom_t,
    pub urgent: C2RustUnnamed_9,
    pub dock: C2RustUnnamed_8,
    pub id: xcb_window_t,
    pub window_mode: C2RustUnnamed_7,
    pub con_id: *mut Con,
    pub match_all_windows: bool,
    pub insert_where: C2RustUnnamed_6,
    pub matches: C2RustUnnamed_5,
    pub restart_mode: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub tqe_next: *mut Match,
    pub tqe_prev: *mut *mut Match,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct regex {
    pub pattern: *mut ::core::ffi::c_char,
    pub regex: *mut pcre2_code_8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct focus_head {
    pub tqh_first: *mut Con,
    pub tqh_last: *mut *mut Con,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nodes_head {
    pub tqh_first: *mut Con,
    pub tqh_last: *mut *mut Con,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct floating_head {
    pub tqh_first: *mut Con,
    pub tqh_last: *mut *mut Con,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct deco_render_params {
    pub color: *mut Colortriple,
    pub border_style: ::core::ffi::c_int,
    pub con_rect: width_height,
    pub con_window_rect: width_height,
    pub con_deco_rect: Rect,
    pub background: color_t,
    pub parent_layout: layout_t,
    pub con_is_leaf: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct width_height {
    pub w: uint32_t,
    pub h: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Colortriple {
    pub border: color_t,
    pub background: color_t,
    pub text: color_t,
    pub indicator: color_t,
    pub child_border: color_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ev_timer {
    pub active: ::core::ffi::c_int,
    pub pending: ::core::ffi::c_int,
    pub priority: ::core::ffi::c_int,
    pub data: *mut ::core::ffi::c_void,
    pub cb: Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_timer, ::core::ffi::c_int) -> ()>,
    pub at: ev_tstamp,
    pub repeat: ev_tstamp,
}
pub type ev_tstamp = ::core::ffi::c_double;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Window {
    pub id: xcb_window_t,
    pub leader: xcb_window_t,
    pub transient_for: xcb_window_t,
    pub nr_assignments: uint32_t,
    pub ran_assignments: *mut *mut Assignment,
    pub class_class: *mut ::core::ffi::c_char,
    pub class_instance: *mut ::core::ffi::c_char,
    pub name: *mut i3String,
    pub role: *mut ::core::ffi::c_char,
    pub machine: *mut ::core::ffi::c_char,
    pub name_x_changed: bool,
    pub uses_net_wm_name: bool,
    pub needs_take_focus: bool,
    pub doesnt_accept_focus: bool,
    pub window_type: xcb_atom_t,
    pub wm_desktop: uint32_t,
    pub dock: C2RustUnnamed_10,
    pub urgent: timeval,
    pub reserved: reservedpx,
    pub depth: uint16_t,
    pub base_width: ::core::ffi::c_int,
    pub base_height: ::core::ffi::c_int,
    pub width_increment: ::core::ffi::c_int,
    pub height_increment: ::core::ffi::c_int,
    pub min_width: ::core::ffi::c_int,
    pub min_height: ::core::ffi::c_int,
    pub max_width: ::core::ffi::c_int,
    pub max_height: ::core::ffi::c_int,
    pub min_aspect_ratio: ::core::ffi::c_double,
    pub max_aspect_ratio: ::core::ffi::c_double,
    pub icon: *mut cairo_surface_t,
    pub shaped: bool,
    pub input_shaped: bool,
    pub managed_since: time_t,
    pub swallowed: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct reservedpx {
    pub left: uint32_t,
    pub right: uint32_t,
    pub top: uint32_t,
    pub bottom: uint32_t,
}
pub type C2RustUnnamed_10 = ::core::ffi::c_uint;
pub const W_DOCK_BOTTOM: C2RustUnnamed_10 = 2;
pub const W_DOCK_TOP: C2RustUnnamed_10 = 1;
pub const W_NODOCK: C2RustUnnamed_10 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Assignment {
    pub type_0: C2RustUnnamed_13,
    pub match_0: Match,
    pub dest: C2RustUnnamed_12,
    pub assignments: C2RustUnnamed_11,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub tqe_next: *mut Assignment,
    pub tqe_prev: *mut *mut Assignment,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_12 {
    pub command: *mut ::core::ffi::c_char,
    pub workspace: *mut ::core::ffi::c_char,
    pub output: *mut ::core::ffi::c_char,
}
pub type C2RustUnnamed_13 = ::core::ffi::c_uint;
pub const A_TO_OUTPUT: C2RustUnnamed_13 = 16;
pub const A_TO_WORKSPACE_NUMBER: C2RustUnnamed_13 = 8;
pub const A_NO_FOCUS: C2RustUnnamed_13 = 4;
pub const A_TO_WORKSPACE: C2RustUnnamed_13 = 2;
pub const A_COMMAND: C2RustUnnamed_13 = 1;
pub const A_ANY: C2RustUnnamed_13 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct marks_head {
    pub tqh_first: *mut mark_t,
    pub tqh_last: *mut *mut mark_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mark_t {
    pub name: *mut ::core::ffi::c_char,
    pub marks: C2RustUnnamed_14,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub tqe_next: *mut mark_t,
    pub tqe_prev: *mut *mut mark_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gaps_t {
    pub inner: ::core::ffi::c_int,
    pub top: ::core::ffi::c_int,
    pub right: ::core::ffi::c_int,
    pub bottom: ::core::ffi::c_int,
    pub left: ::core::ffi::c_int,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ev_watcher {
    pub active: ::core::ffi::c_int,
    pub pending: ::core::ffi::c_int,
    pub priority: ::core::ffi::c_int,
    pub data: *mut ::core::ffi::c_void,
    pub cb: Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_watcher, ::core::ffi::c_int) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ev_watcher_time {
    pub active: ::core::ffi::c_int,
    pub pending: ::core::ffi::c_int,
    pub priority: ::core::ffi::c_int,
    pub data: *mut ::core::ffi::c_void,
    pub cb:
        Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_watcher_time, ::core::ffi::c_int) -> ()>,
    pub at: ev_tstamp,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rlimit {
    pub rlim_cur: rlim_t,
    pub rlim_max: rlim_t,
}
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
pub type SnMonitorEventType = ::core::ffi::c_uint;
pub const SN_MONITOR_EVENT_CANCELED: SnMonitorEventType = 3;
pub const SN_MONITOR_EVENT_CHANGED: SnMonitorEventType = 2;
pub const SN_MONITOR_EVENT_COMPLETED: SnMonitorEventType = 1;
pub const SN_MONITOR_EVENT_INITIATED: SnMonitorEventType = 0;
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
pub const XCB_NONE: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const SIG_DFL: __sighandler_t = None;
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
    mut loop_0: *mut ev_loop,
    mut w: *mut ev_timer,
    mut revents: ::core::ffi::c_int,
) {
    let mut id: *const ::core::ffi::c_char =
        sn_launcher_context_get_startup_id((*w).data as *mut SnLauncherContext);
    debuglog(
        b"%s:%s:%d - Timeout for startup sequence %s\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/startup.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"startup_timeout\0" as *const u8 as *const ::core::ffi::c_char,
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
            b"%s:%s:%d - Sequence already deleted, nevermind.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/startup.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"startup_timeout\0" as *const u8 as *const ::core::ffi::c_char,
            52 as ::core::ffi::c_int,
        );
        free(w as *mut ::core::ffi::c_void);
        return;
    }
    sn_launcher_context_complete((*w).data as *mut SnLauncherContext);
    free(w as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn _prune_startup_sequences() -> ::core::ffi::c_int {
    let current_time: time_t = time(::core::ptr::null_mut::<time_t>()) as time_t;
    let mut active_sequences: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut next: *mut Startup_Sequence = startup_sequences.tqh_first;
    while !next.is_null() {
        let mut current: *mut Startup_Sequence = next;
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
    return active_sequences;
}
#[no_mangle]
pub unsafe extern "C" fn startup_sequence_delete(mut sequence: *mut Startup_Sequence) {
    if sequence != ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut Startup_Sequence {
    } else {
        __assert_fail(
            b"sequence != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            b"../src/startup.c\0" as *const u8 as *const ::core::ffi::c_char,
            106 as ::core::ffi::c_uint,
            __ASSERT_FUNCTION.as_ptr(),
        );
    };
    debuglog(
        b"%s:%s:%d - Deleting startup sequence %s, delete_at = %lld, current_time = %lld\n\0"
            as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/startup.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"startup_sequence_delete\0" as *const u8 as *const ::core::ffi::c_char,
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
#[no_mangle]
pub unsafe extern "C" fn start_application(
    mut command: *const ::core::ffi::c_char,
    mut no_startup_id: bool,
    mut window_id: xcb_window_t,
) {
    let mut context: *mut SnLauncherContext = ::core::ptr::null_mut::<SnLauncherContext>();
    if !no_startup_id {
        context = sn_launcher_context_new(sndisplay, conn_screen);
        sn_launcher_context_set_name(context, b"i3\0" as *const u8 as *const ::core::ffi::c_char);
        sn_launcher_context_set_description(
            context,
            b"exec command in i3\0" as *const u8 as *const ::core::ffi::c_char,
        );
        let mut first_word: *mut ::core::ffi::c_char = sstrdup(command);
        let mut space: *mut ::core::ffi::c_char = strchr(first_word, ' ' as i32);
        if !space.is_null() {
            *space = '\0' as i32 as ::core::ffi::c_char;
        }
        sn_launcher_context_initiate(
            context,
            b"i3\0" as *const u8 as *const ::core::ffi::c_char,
            first_word,
            last_timestamp as Time,
        );
        free(first_word as *mut ::core::ffi::c_void);
        let mut timeout: *mut ev_timer =
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
            b"startup id = %s\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            sn_launcher_context_get_startup_id(context),
        );
        let mut ws: *mut Con = con_get_workspace(focused);
        let mut sequence: *mut Startup_Sequence = scalloc(
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
    verboselog(
        b"executing: %s\n\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        command,
    );
    if fork() == 0 as ::core::ffi::c_int {
        setsid();
        setrlimit(RLIMIT_CORE, &raw mut original_rlimit_core);
        let mut fd: ::core::ffi::c_int = SD_LISTEN_FDS_START;
        while fd < SD_LISTEN_FDS_START + listen_fds {
            close(fd);
            fd += 1;
        }
        unsetenv(b"LISTEN_PID\0" as *const u8 as *const ::core::ffi::c_char);
        unsetenv(b"LISTEN_FDS\0" as *const u8 as *const ::core::ffi::c_char);
        signal(SIGPIPE, SIG_DFL);
        if !no_startup_id {
            sn_launcher_context_setup_child_process(context);
        }
        setenv(
            b"I3SOCK\0" as *const u8 as *const ::core::ffi::c_char,
            current_socketpath,
            1 as ::core::ffi::c_int,
        );
        if window_id as ::core::ffi::c_long != XCB_NONE {
            let mut window_id_str: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut window_id_str,
                b"%d\0" as *const u8 as *const ::core::ffi::c_char,
                window_id,
            );
            setenv(
                b"I3_WINDOW_ID\0" as *const u8 as *const ::core::ffi::c_char,
                window_id_str,
                1 as ::core::ffi::c_int,
            );
            free(window_id_str as *mut ::core::ffi::c_void);
        }
        execl(
            _PATH_BSHELL.as_ptr(),
            _PATH_BSHELL.as_ptr(),
            b"-c\0" as *const u8 as *const ::core::ffi::c_char,
            command,
            NULL,
        );
        err(
            EXIT_FAILURE,
            b"execl return\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if !no_startup_id {
        xcursor_set_root_cursor(XCURSOR_CURSOR_WATCH as ::core::ffi::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn startup_monitor_event(
    mut event: *mut SnMonitorEvent,
    mut userdata: *mut ::core::ffi::c_void,
) {
    let mut sn_startup_sequence: *mut SnStartupSequence =
        sn_monitor_event_get_startup_sequence(event);
    let mut id: *const ::core::ffi::c_char = sn_startup_sequence_get_id(sn_startup_sequence);
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
            b"%s:%s:%d - Got event for startup sequence that we did not initiate (ID = %s). Ignoring.\n\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/startup.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"startup_monitor_event\0" as *const u8 as *const ::core::ffi::c_char,
            237 as ::core::ffi::c_int,
            id,
        );
        return;
    }
    match sn_monitor_event_get_type(event) as ::core::ffi::c_uint {
        1 => {
            debuglog(
                b"%s:%s:%d - startup sequence %s completed\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"../src/startup.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"startup_monitor_event\0" as *const u8 as *const ::core::ffi::c_char,
                243 as ::core::ffi::c_int,
                sn_startup_sequence_get_id(sn_startup_sequence),
            );
            let mut current_time: time_t = time(::core::ptr::null_mut::<time_t>());
            (*sequence).delete_at = current_time + 30 as time_t;
            debuglog(
                b"%s:%s:%d - Will delete startup sequence %s at timestamp %lld\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"../src/startup.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"startup_monitor_event\0" as *const u8 as *const ::core::ffi::c_char,
                249 as ::core::ffi::c_int,
                (*sequence).id,
                (*sequence).delete_at as ::core::ffi::c_longlong,
            );
            if _prune_startup_sequences() == 0 as ::core::ffi::c_int {
                debuglog(
                    b"%s:%s:%d - No more startup sequences running, changing root window cursor to default pointer.\n\0"
                        as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    b"../src/startup.c\0" as *const u8 as *const ::core::ffi::c_char,
                    b"startup_monitor_event\0" as *const u8
                        as *const ::core::ffi::c_char,
                    252 as ::core::ffi::c_int,
                );
                xcursor_set_root_cursor(XCURSOR_CURSOR_POINTER as ::core::ffi::c_int);
            }
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn startup_sequence_rename_workspace(
    mut old_name: *const ::core::ffi::c_char,
    mut new_name: *const ::core::ffi::c_char,
) {
    let mut current: *mut Startup_Sequence = ::core::ptr::null_mut::<Startup_Sequence>();
    current = startup_sequences.tqh_first;
    while !current.is_null() {
        if !(strcmp((*current).workspace, old_name) != 0 as ::core::ffi::c_int) {
            debuglog(
                b"%s:%s:%d - Renaming workspace \"%s\" to \"%s\" in startup sequence %s.\n\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"../src/startup.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"startup_sequence_rename_workspace\0" as *const u8 as *const ::core::ffi::c_char,
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
#[no_mangle]
pub unsafe extern "C" fn startup_sequence_get(
    mut cwindow: *const i3Window,
    mut startup_id_reply: *mut xcb_get_property_reply_t,
    mut ignore_mapped_leader: bool,
) -> *mut Startup_Sequence {
    if startup_id_reply.is_null()
        || xcb_get_property_value_length(startup_id_reply) == 0 as ::core::ffi::c_int
    {
        free(startup_id_reply as *mut ::core::ffi::c_void);
        startup_id_reply = ::core::ptr::null_mut::<xcb_get_property_reply_t>();
        debuglog(
            b"%s:%s:%d - No _NET_STARTUP_ID set on window 0x%08x\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/startup.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"startup_sequence_get\0" as *const u8 as *const ::core::ffi::c_char,
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
                b"%s:%s:%d - Ignoring leader window 0x%08x\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"../src/startup.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"startup_sequence_get\0" as *const u8 as *const ::core::ffi::c_char,
                303 as ::core::ffi::c_int,
                (*cwindow).leader,
            );
            return ::core::ptr::null_mut::<Startup_Sequence>();
        }
        debuglog(
            b"%s:%s:%d - Checking leader window 0x%08x\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/startup.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"startup_sequence_get\0" as *const u8 as *const ::core::ffi::c_char,
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
                b"%s:%s:%d - No _NET_STARTUP_ID set on the leader either\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"../src/startup.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"startup_sequence_get\0" as *const u8 as *const ::core::ffi::c_char,
                316 as ::core::ffi::c_int,
            );
            return ::core::ptr::null_mut::<Startup_Sequence>();
        }
    }
    let mut startup_id: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    sasprintf(
        &raw mut startup_id,
        b"%.*s\0" as *const u8 as *const ::core::ffi::c_char,
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
            b"%s:%s:%d - WARNING: This sequence (ID %s) was not found\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/startup.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"startup_sequence_get\0" as *const u8 as *const ::core::ffi::c_char,
            335 as ::core::ffi::c_int,
            startup_id,
        );
        free(startup_id as *mut ::core::ffi::c_void);
        free(startup_id_reply as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<Startup_Sequence>();
    }
    free(startup_id as *mut ::core::ffi::c_void);
    free(startup_id_reply as *mut ::core::ffi::c_void);
    return sequence;
}
#[no_mangle]
pub unsafe extern "C" fn startup_workspace_for_window(
    mut cwindow: *const i3Window,
    mut startup_id_reply: *mut xcb_get_property_reply_t,
) -> *mut ::core::ffi::c_char {
    let mut sequence: *mut Startup_Sequence =
        startup_sequence_get(cwindow, startup_id_reply, false_0 != 0);
    if sequence.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    let mut current_time: time_t = time(::core::ptr::null_mut::<time_t>());
    if (*sequence).delete_at > 0 as time_t && current_time > (*sequence).delete_at {
        debuglog(
            b"%s:%s:%d - Deleting expired startup sequence %s\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/startup.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"startup_workspace_for_window\0" as *const u8 as *const ::core::ffi::c_char,
            365 as ::core::ffi::c_int,
            (*sequence).id,
        );
        startup_sequence_delete(sequence);
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    return (*sequence).workspace;
}
#[no_mangle]
pub unsafe extern "C" fn startup_sequence_delete_by_window(mut win: *const i3Window) {
    let cookie: xcb_get_property_cookie_t = xcb_get_property(
        conn,
        false_0 as uint8_t,
        (*win).id,
        A__NET_STARTUP_ID,
        XCB_GET_PROPERTY_TYPE_ANY as ::core::ffi::c_int as xcb_atom_t,
        0 as uint32_t,
        512 as uint32_t,
    ) as xcb_get_property_cookie_t;
    let mut startup_id_reply: *mut xcb_get_property_reply_t = xcb_get_property_reply(
        conn,
        cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    let mut sequence: *mut Startup_Sequence =
        startup_sequence_get(win, startup_id_reply, true_0 != 0);
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
