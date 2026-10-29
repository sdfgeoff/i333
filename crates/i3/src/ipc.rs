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
// Generated with C2Rust 0.22.1; C ABI and original ownership retained.
extern "C" {
    pub type _cairo;
    pub type _cairo_surface;
    pub type _i3String;
    pub type pcre2_real_code_8;
    pub type ev_loop;
    pub type yajl_gen_t;
    pub type yajl_handle_t;
    pub type sockaddr_x25;
    pub type sockaddr_ns;
    pub type sockaddr_iso;
    pub type sockaddr_ipx;
    pub type sockaddr_inarp;
    pub type sockaddr_in6;
    pub type sockaddr_in;
    pub type sockaddr_eon;
    pub type sockaddr_dl;
    pub type sockaddr_ax25;
    pub type sockaddr_at;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn __assert_single_arg(_: bool) -> bool;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn perror(__s: *const ::core::ffi::c_char);
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn read(__fd: ::core::ffi::c_int, __buf: *mut ::core::ffi::c_void, __nbytes: size_t)
        -> ssize_t;
    fn verboselog(fmt: *mut ::core::ffi::c_char, ...);
    fn errorlog(fmt: *mut ::core::ffi::c_char, ...);
    fn debuglog(fmt: *mut ::core::ffi::c_char, ...);
    fn scalloc(num: size_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn srealloc(ptr: *mut ::core::ffi::c_void, size: size_t) -> *mut ::core::ffi::c_void;
    fn sstrdup(str: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn sstrndup(str: *const ::core::ffi::c_char, size: size_t) -> *mut ::core::ffi::c_char;
    fn sasprintf(
        strp: *mut *mut ::core::ffi::c_char,
        fmt: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn writeall_nonblock(
        fd: ::core::ffi::c_int,
        buf: *const ::core::ffi::c_void,
        count: size_t,
    ) -> ssize_t;
    fn i3string_as_utf8(str: *mut i3String) -> *const ::core::ffi::c_char;
    fn ipc_recv_message(
        sockfd: ::core::ffi::c_int,
        message_type: *mut uint32_t,
        reply_length: *mut uint32_t,
        reply: *mut *mut uint8_t,
    ) -> ::core::ffi::c_int;
    fn set_nonblock(sockfd: ::core::ffi::c_int);
    fn ev_io_start(loop_0: *mut ev_loop, w: *mut ev_io);
    fn ev_io_stop(loop_0: *mut ev_loop, w: *mut ev_io);
    fn ev_timer_start(loop_0: *mut ev_loop, w: *mut ev_timer);
    fn ev_timer_stop(loop_0: *mut ev_loop, w: *mut ev_timer);
    fn yajl_gen_alloc(allocFuncs: *const yajl_alloc_funcs) -> yajl_gen;
    fn yajl_gen_free(handle: yajl_gen);
    fn yajl_gen_integer(hand: yajl_gen, number: ::core::ffi::c_longlong) -> yajl_gen_status;
    fn yajl_gen_double(hand: yajl_gen, number: ::core::ffi::c_double) -> yajl_gen_status;
    fn yajl_gen_string(
        hand: yajl_gen,
        str: *const ::core::ffi::c_uchar,
        len: size_t,
    ) -> yajl_gen_status;
    fn yajl_gen_null(hand: yajl_gen) -> yajl_gen_status;
    fn yajl_gen_bool(hand: yajl_gen, boolean: ::core::ffi::c_int) -> yajl_gen_status;
    fn yajl_gen_map_open(hand: yajl_gen) -> yajl_gen_status;
    fn yajl_gen_map_close(hand: yajl_gen) -> yajl_gen_status;
    fn yajl_gen_array_open(hand: yajl_gen) -> yajl_gen_status;
    fn yajl_gen_array_close(hand: yajl_gen) -> yajl_gen_status;
    fn yajl_gen_get_buf(
        hand: yajl_gen,
        buf: *mut *const ::core::ffi::c_uchar,
        len: *mut size_t,
    ) -> yajl_gen_status;
    fn yajl_alloc(
        callbacks: *const yajl_callbacks,
        afs: *mut yajl_alloc_funcs,
        ctx: *mut ::core::ffi::c_void,
    ) -> yajl_handle;
    fn yajl_free(handle: yajl_handle);
    fn yajl_parse(
        hand: yajl_handle,
        jsonText: *const ::core::ffi::c_uchar,
        jsonTextLength: size_t,
    ) -> yajl_status;
    fn yajl_get_error(
        hand: yajl_handle,
        verbose: ::core::ffi::c_int,
        jsonText: *const ::core::ffi::c_uchar,
        jsonTextLength: size_t,
    ) -> *mut ::core::ffi::c_uchar;
    fn yajl_free_error(hand: yajl_handle, str: *mut ::core::ffi::c_uchar);
    static mut croot: *mut Con;
    static mut focused: *mut Con;
    static mut all_cons: all_cons_head;
    fn tree_render();
    static mut i3_version: *const ::core::ffi::c_char;
    static mut current_binding_mode: *const ::core::ffi::c_char;
    static mut main_loop: *mut ev_loop;
    static mut current_configpath: *mut ::core::ffi::c_char;
    static mut modes: modes_head;
    static mut barconfigs: barconfig_head;
    static mut included_files: includedfiles_head;
    static mut A__NET_WM_WINDOW_TYPE_NOTIFICATION: xcb_atom_t;
    static mut A__NET_WM_WINDOW_TYPE_TOOLTIP: xcb_atom_t;
    static mut A__NET_WM_WINDOW_TYPE_POPUP_MENU: xcb_atom_t;
    static mut A__NET_WM_WINDOW_TYPE_DROPDOWN_MENU: xcb_atom_t;
    static mut A__NET_WM_WINDOW_TYPE_MENU: xcb_atom_t;
    static mut A__NET_WM_WINDOW_TYPE_SPLASH: xcb_atom_t;
    static mut A__NET_WM_WINDOW_TYPE_TOOLBAR: xcb_atom_t;
    static mut A__NET_WM_WINDOW_TYPE_UTILITY: xcb_atom_t;
    static mut A__NET_WM_WINDOW_TYPE_DIALOG: xcb_atom_t;
    static mut A__NET_WM_WINDOW_TYPE_DOCK: xcb_atom_t;
    static mut A__NET_WM_WINDOW_TYPE_NORMAL: xcb_atom_t;
    static mut outputs: outputs_head;
    fn get_output_by_name(name: *const ::core::ffi::c_char, require_active: bool) -> *mut Output;
    static mut previous_workspace_name: *mut ::core::ffi::c_char;
    fn workspace_is_visible(ws: *mut Con) -> bool;
    fn con_is_split(con: *mut Con) -> bool;
    fn con_get_output(con: *mut Con) -> *mut Con;
    fn con_get_workspace(con: *mut Con) -> *mut Con;
    fn con_get_fullscreen_con(con: *mut Con, fullscreen_mode: fullscreen_mode_t) -> *mut Con;
    fn con_is_internal(con: *mut Con) -> bool;
    fn con_orientation(con: *mut Con) -> orientation_t;
    fn con_draw_decoration_into_frame(con: *mut Con) -> bool;
    fn output_get_content(output: *mut Con) -> *mut Con;
    fn output_primary_name(output: *mut Output) -> *mut ::core::ffi::c_char;
    fn parse_command(
        input: *const ::core::ffi::c_char,
        gen: yajl_gen,
        client: *mut ipc_client,
    ) -> *mut CommandResult;
    fn command_result_free(result: *mut CommandResult);
    fn sync_respond(window: xcb_window_t, rnd: uint32_t);
    fn fcntl(__fd: ::core::ffi::c_int, __cmd: ::core::ffi::c_int, ...) -> ::core::ffi::c_int;
    fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    fn setlocale(
        __category: ::core::ffi::c_int,
        __locale: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn getsockopt(
        __fd: ::core::ffi::c_int,
        __level: ::core::ffi::c_int,
        __optname: ::core::ffi::c_int,
        __optval: *mut ::core::ffi::c_void,
        __optlen: *mut socklen_t,
    ) -> ::core::ffi::c_int;
    fn accept(
        __fd: ::core::ffi::c_int,
        __addr: __SOCKADDR_ARG,
        __addr_len: *mut socklen_t,
    ) -> ::core::ffi::c_int;
    fn shutdown(__fd: ::core::ffi::c_int, __how: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memmove(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn strcasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn free(__ptr: *mut ::core::ffi::c_void);
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uintptr_t = usize;
pub type xcb_window_t = uint32_t;
pub type xcb_gcontext_t = uint32_t;
pub type xcb_colormap_t = uint32_t;
pub type xcb_atom_t = uint32_t;
pub type xcb_drawable_t = uint32_t;
pub type xcb_keycode_t = uint8_t;
pub type xcb_key_but_mask_t = ::core::ffi::c_uint;
pub const XCB_KEY_BUT_MASK_BUTTON_5: xcb_key_but_mask_t = 4096;
pub const XCB_KEY_BUT_MASK_BUTTON_4: xcb_key_but_mask_t = 2048;
pub const XCB_KEY_BUT_MASK_BUTTON_3: xcb_key_but_mask_t = 1024;
pub const XCB_KEY_BUT_MASK_BUTTON_2: xcb_key_but_mask_t = 512;
pub const XCB_KEY_BUT_MASK_BUTTON_1: xcb_key_but_mask_t = 256;
pub const XCB_KEY_BUT_MASK_MOD_5: xcb_key_but_mask_t = 128;
pub const XCB_KEY_BUT_MASK_MOD_4: xcb_key_but_mask_t = 64;
pub const XCB_KEY_BUT_MASK_MOD_3: xcb_key_but_mask_t = 32;
pub const XCB_KEY_BUT_MASK_MOD_2: xcb_key_but_mask_t = 16;
pub const XCB_KEY_BUT_MASK_MOD_1: xcb_key_but_mask_t = 8;
pub const XCB_KEY_BUT_MASK_CONTROL: xcb_key_but_mask_t = 4;
pub const XCB_KEY_BUT_MASK_LOCK: xcb_key_but_mask_t = 2;
pub const XCB_KEY_BUT_MASK_SHIFT: xcb_key_but_mask_t = 1;
pub type xcb_window_enum_t = ::core::ffi::c_uint;
pub const XCB_WINDOW_NONE: xcb_window_enum_t = 0;
pub type socklen_t = __socklen_t;
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
pub type xcb_randr_output_t = uint32_t;
pub type pcre2_code_8 = pcre2_real_code_8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Binding {
    pub input_type: input_type_t,
    pub release: C2RustUnnamed_1,
    pub border: bool,
    pub whole_window: bool,
    pub exclude_titlebar: bool,
    pub keycode: uint32_t,
    pub event_state_mask: i3_event_state_mask_t,
    pub symbol: *mut ::core::ffi::c_char,
    pub keycodes_head: keycodes_head,
    pub command: *mut ::core::ffi::c_char,
    pub bindings: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
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
    pub keycodes: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub tqe_next: *mut Binding_Keycode,
    pub tqe_prev: *mut *mut Binding_Keycode,
}
pub type i3_event_state_mask_t = uint32_t;
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const B_UPON_KEYRELEASE_IGNORE_MODS: C2RustUnnamed_1 = 2;
pub const B_UPON_KEYRELEASE: C2RustUnnamed_1 = 1;
pub const B_UPON_KEYPRESS: C2RustUnnamed_1 = 0;
pub type input_type_t = ::core::ffi::c_uint;
pub const B_MOUSE: input_type_t = 1;
pub const B_KEYBOARD: input_type_t = 0;
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
pub struct xoutput {
    pub id: xcb_randr_output_t,
    pub active: bool,
    pub changed: bool,
    pub to_be_disabled: bool,
    pub primary: bool,
    pub names_head: names_head,
    pub con: *mut Con,
    pub rect: Rect,
    pub outputs: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub tqe_next: *mut xoutput,
    pub tqe_prev: *mut *mut xoutput,
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
    pub type_0: C2RustUnnamed_19,
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
    pub floating: C2RustUnnamed_8,
    pub nodes: C2RustUnnamed_7,
    pub focused: C2RustUnnamed_6,
    pub all_cons: C2RustUnnamed_5,
    pub floating_windows: C2RustUnnamed_4,
    pub on_remove_child: Option<unsafe extern "C" fn(*mut Con) -> ()>,
    pub scratchpad_state: C2RustUnnamed_3,
    pub old_id: ::core::ffi::c_int,
    pub depth: uint16_t,
    pub colormap: xcb_colormap_t,
}
pub type C2RustUnnamed_3 = ::core::ffi::c_uint;
pub const SCRATCHPAD_CHANGED: C2RustUnnamed_3 = 2;
pub const SCRATCHPAD_FRESH: C2RustUnnamed_3 = 1;
pub const SCRATCHPAD_NONE: C2RustUnnamed_3 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub tqe_next: *mut Con,
    pub tqe_prev: *mut *mut Con,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub tqe_next: *mut Con,
    pub tqe_prev: *mut *mut Con,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub tqe_next: *mut Con,
    pub tqe_prev: *mut *mut Con,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub tqe_next: *mut Con,
    pub tqe_prev: *mut *mut Con,
}
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
    pub urgent: C2RustUnnamed_13,
    pub dock: C2RustUnnamed_12,
    pub id: xcb_window_t,
    pub window_mode: C2RustUnnamed_11,
    pub con_id: *mut Con,
    pub match_all_windows: bool,
    pub insert_where: C2RustUnnamed_10,
    pub matches: C2RustUnnamed_9,
    pub restart_mode: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub tqe_next: *mut Match,
    pub tqe_prev: *mut *mut Match,
}
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
    pub dock: C2RustUnnamed_14,
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
pub type C2RustUnnamed_14 = ::core::ffi::c_uint;
pub const W_DOCK_BOTTOM: C2RustUnnamed_14 = 2;
pub const W_DOCK_TOP: C2RustUnnamed_14 = 1;
pub const W_NODOCK: C2RustUnnamed_14 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Assignment {
    pub type_0: C2RustUnnamed_17,
    pub match_0: Match,
    pub dest: C2RustUnnamed_16,
    pub assignments: C2RustUnnamed_15,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_15 {
    pub tqe_next: *mut Assignment,
    pub tqe_prev: *mut *mut Assignment,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_16 {
    pub command: *mut ::core::ffi::c_char,
    pub workspace: *mut ::core::ffi::c_char,
    pub output: *mut ::core::ffi::c_char,
}
pub type C2RustUnnamed_17 = ::core::ffi::c_uint;
pub const A_TO_OUTPUT: C2RustUnnamed_17 = 16;
pub const A_TO_WORKSPACE_NUMBER: C2RustUnnamed_17 = 8;
pub const A_NO_FOCUS: C2RustUnnamed_17 = 4;
pub const A_TO_WORKSPACE: C2RustUnnamed_17 = 2;
pub const A_COMMAND: C2RustUnnamed_17 = 1;
pub const A_ANY: C2RustUnnamed_17 = 0;
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
    pub marks: C2RustUnnamed_18,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_18 {
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
pub type C2RustUnnamed_19 = ::core::ffi::c_uint;
pub const CT_DOCKAREA: C2RustUnnamed_19 = 5;
pub const CT_WORKSPACE: C2RustUnnamed_19 = 4;
pub const CT_FLOATING_CON: C2RustUnnamed_19 = 3;
pub const CT_CON: C2RustUnnamed_19 = 2;
pub const CT_OUTPUT: C2RustUnnamed_19 = 1;
pub const CT_ROOT: C2RustUnnamed_19 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct names_head {
    pub slh_first: *mut output_name,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct output_name {
    pub name: *mut ::core::ffi::c_char,
    pub names: C2RustUnnamed_20,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_20 {
    pub sle_next: *mut output_name,
}
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
pub type C2RustUnnamed_22 = ::core::ffi::c_int;
pub const EV_ERROR: C2RustUnnamed_22 = -2147483648;
pub const EV_CUSTOM: C2RustUnnamed_22 = 16777216;
pub const EV_ASYNC: C2RustUnnamed_22 = 524288;
pub const EV_CLEANUP: C2RustUnnamed_22 = 262144;
pub const EV_FORK: C2RustUnnamed_22 = 131072;
pub const EV_EMBED: C2RustUnnamed_22 = 65536;
pub const EV_CHECK: C2RustUnnamed_22 = 32768;
pub const EV_PREPARE: C2RustUnnamed_22 = 16384;
pub const EV_IDLE: C2RustUnnamed_22 = 8192;
pub const EV_STAT: C2RustUnnamed_22 = 4096;
pub const EV_CHILD: C2RustUnnamed_22 = 2048;
pub const EV_SIGNAL: C2RustUnnamed_22 = 1024;
pub const EV_PERIODIC: C2RustUnnamed_22 = 512;
pub const EV_TIMEOUT: C2RustUnnamed_22 = 256;
pub const EV_TIMER: C2RustUnnamed_22 = 256;
pub const EV_IO: C2RustUnnamed_22 = 1;
pub const EV__IOFDSET: C2RustUnnamed_22 = 128;
pub const EV_WRITE: C2RustUnnamed_22 = 2;
pub const EV_READ: C2RustUnnamed_22 = 1;
pub const EV_NONE: C2RustUnnamed_22 = 0;
pub const EV_UNDEF: C2RustUnnamed_22 = -1;
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
pub struct ev_watcher_list {
    pub active: ::core::ffi::c_int,
    pub pending: ::core::ffi::c_int,
    pub priority: ::core::ffi::c_int,
    pub data: *mut ::core::ffi::c_void,
    pub cb:
        Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_watcher_list, ::core::ffi::c_int) -> ()>,
    pub next: *mut ev_watcher_list,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ev_io {
    pub active: ::core::ffi::c_int,
    pub pending: ::core::ffi::c_int,
    pub priority: ::core::ffi::c_int,
    pub data: *mut ::core::ffi::c_void,
    pub cb: Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_io, ::core::ffi::c_int) -> ()>,
    pub next: *mut ev_watcher_list,
    pub fd: ::core::ffi::c_int,
    pub events: ::core::ffi::c_int,
}
pub type yajl_malloc_func =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> *mut ::core::ffi::c_void>;
pub type yajl_free_func =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> ()>;
pub type yajl_realloc_func = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
        size_t,
    ) -> *mut ::core::ffi::c_void,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yajl_alloc_funcs {
    pub malloc: yajl_malloc_func,
    pub realloc: yajl_realloc_func,
    pub free: yajl_free_func,
    pub ctx: *mut ::core::ffi::c_void,
}
pub type yajl_gen_status = ::core::ffi::c_uint;
pub const yajl_gen_invalid_string: yajl_gen_status = 7;
pub const yajl_gen_no_buf: yajl_gen_status = 6;
pub const yajl_gen_invalid_number: yajl_gen_status = 5;
pub const yajl_gen_generation_complete: yajl_gen_status = 4;
pub const yajl_gen_in_error_state: yajl_gen_status = 3;
pub const yajl_max_depth_exceeded: yajl_gen_status = 2;
pub const yajl_gen_keys_must_be_strings: yajl_gen_status = 1;
pub const yajl_gen_status_ok: yajl_gen_status = 0;
pub type yajl_gen = *mut yajl_gen_t;
pub type yajl_status = ::core::ffi::c_uint;
pub const yajl_status_error: yajl_status = 2;
pub const yajl_status_client_canceled: yajl_status = 1;
pub const yajl_status_ok: yajl_status = 0;
pub type yajl_handle = *mut yajl_handle_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yajl_callbacks {
    pub yajl_null: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
    pub yajl_boolean: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub yajl_integer: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_longlong,
        ) -> ::core::ffi::c_int,
    >,
    pub yajl_double: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, ::core::ffi::c_double) -> ::core::ffi::c_int,
    >,
    pub yajl_number: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_char,
            size_t,
        ) -> ::core::ffi::c_int,
    >,
    pub yajl_string: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_uchar,
            size_t,
        ) -> ::core::ffi::c_int,
    >,
    pub yajl_start_map:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
    pub yajl_map_key: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_uchar,
            size_t,
        ) -> ::core::ffi::c_int,
    >,
    pub yajl_end_map: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
    pub yajl_start_array:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
    pub yajl_end_array:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct all_cons_head {
    pub tqh_first: *mut Con,
    pub tqh_last: *mut *mut Con,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bindings_head {
    pub tqh_first: *mut Binding,
    pub tqh_last: *mut *mut Binding,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IncludedFile {
    pub path: *mut ::core::ffi::c_char,
    pub raw_contents: *mut ::core::ffi::c_char,
    pub variable_replaced_contents: *mut ::core::ffi::c_char,
    pub files: C2RustUnnamed_23,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_23 {
    pub tqe_next: *mut IncludedFile,
    pub tqe_prev: *mut *mut IncludedFile,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Barconfig {
    pub id: *mut ::core::ffi::c_char,
    pub num_outputs: ::core::ffi::c_int,
    pub outputs: *mut *mut ::core::ffi::c_char,
    pub tray_outputs: tray_outputs_head,
    pub tray_padding: ::core::ffi::c_int,
    pub socket_path: *mut ::core::ffi::c_char,
    pub mode: C2RustUnnamed_28,
    pub hidden_state: C2RustUnnamed_27,
    pub modifier: uint32_t,
    pub bar_bindings: bar_bindings_head,
    pub position: C2RustUnnamed_25,
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
    pub configs: C2RustUnnamed_24,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_24 {
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
pub type C2RustUnnamed_25 = ::core::ffi::c_uint;
pub const P_TOP: C2RustUnnamed_25 = 1;
pub const P_BOTTOM: C2RustUnnamed_25 = 0;
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
    pub bindings: C2RustUnnamed_26,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_26 {
    pub tqe_next: *mut Barbinding,
    pub tqe_prev: *mut *mut Barbinding,
}
pub type C2RustUnnamed_27 = ::core::ffi::c_uint;
pub const S_SHOW: C2RustUnnamed_27 = 1;
pub const S_HIDE: C2RustUnnamed_27 = 0;
pub type C2RustUnnamed_28 = ::core::ffi::c_uint;
pub const M_INVISIBLE: C2RustUnnamed_28 = 2;
pub const M_HIDE: C2RustUnnamed_28 = 1;
pub const M_DOCK: C2RustUnnamed_28 = 0;
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
    pub tray_outputs: C2RustUnnamed_29,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_29 {
    pub tqe_next: *mut tray_output_t,
    pub tqe_prev: *mut *mut tray_output_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct modes_head {
    pub slh_first: *mut Mode,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Mode {
    pub name: *mut ::core::ffi::c_char,
    pub pango_markup: bool,
    pub bindings: *mut bindings_head,
    pub modes: C2RustUnnamed_30,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_30 {
    pub sle_next: *mut Mode,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct barconfig_head {
    pub tqh_first: *mut Barconfig,
    pub tqh_last: *mut *mut Barconfig,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct includedfiles_head {
    pub tqh_first: *mut IncludedFile,
    pub tqh_last: *mut *mut IncludedFile,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct i3_ipc_header {
    pub magic: [::core::ffi::c_char; 6],
    pub size: uint32_t,
    pub type_0: uint32_t,
}
pub type i3_ipc_header_t = i3_ipc_header;
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
    pub clients: C2RustUnnamed_31,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_31 {
    pub tqe_next: *mut ipc_client,
    pub tqe_prev: *mut *mut ipc_client,
}
pub type handler_t = Option<
    unsafe extern "C" fn(
        *mut ipc_client,
        *mut uint8_t,
        ::core::ffi::c_int,
        uint32_t,
        uint32_t,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ipc_client_head {
    pub tqh_first: *mut ipc_client,
    pub tqh_last: *mut *mut ipc_client,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ucred {
    pub pid: pid_t,
    pub uid: uid_t,
    pub gid: gid_t,
}
pub type ylength = size_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sync_state {
    pub last_key: *mut ::core::ffi::c_char,
    pub rnd: uint32_t,
    pub window: xcb_window_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct outputs_head {
    pub tqh_first: *mut xoutput,
    pub tqh_last: *mut *mut xoutput,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CommandResult {
    pub parse_error: bool,
    pub error_message: *mut ::core::ffi::c_char,
    pub needs_tree_render: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_un {
    pub sun_family: sa_family_t,
    pub sun_path: [::core::ffi::c_char; 108],
}
pub type sa_family_t = ::core::ffi::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [::core::ffi::c_char; 14],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __SOCKADDR_ARG {
    pub __sockaddr__: *mut sockaddr,
    pub __sockaddr_at__: *mut sockaddr_at,
    pub __sockaddr_ax25__: *mut sockaddr_ax25,
    pub __sockaddr_dl__: *mut sockaddr_dl,
    pub __sockaddr_eon__: *mut sockaddr_eon,
    pub __sockaddr_in__: *mut sockaddr_in,
    pub __sockaddr_in6__: *mut sockaddr_in6,
    pub __sockaddr_inarp__: *mut sockaddr_inarp,
    pub __sockaddr_ipx__: *mut sockaddr_ipx,
    pub __sockaddr_iso__: *mut sockaddr_iso,
    pub __sockaddr_ns__: *mut sockaddr_ns,
    pub __sockaddr_un__: *mut sockaddr_un,
    pub __sockaddr_x25__: *mut sockaddr_x25,
}
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
pub const XCB_NONE: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
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
#[no_mangle]
pub static mut current_socketpath: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]
pub static mut all_clients: ipc_client_head = ipc_client_head {
    tqh_first: ::core::ptr::null::<ipc_client>() as *mut ipc_client,
    tqh_last: ::core::ptr::null::<*mut ipc_client>() as *mut *mut ipc_client,
};
static mut kill_timeout: ev_tstamp = 10.0f64;
#[no_mangle]
pub unsafe extern "C" fn ipc_set_kill_timeout(mut new: ev_tstamp) {
    kill_timeout = new;
}
unsafe extern "C" fn ipc_push_pending(mut client: *mut ipc_client) {
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
        let mut timeout: *mut ev_timer =
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
unsafe extern "C" fn ipc_send_client_message(
    mut client: *mut ipc_client,
    mut size: size_t,
    message_type: uint32_t,
    mut payload: *const uint8_t,
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
unsafe extern "C" fn free_ipc_client(
    mut client: *mut ipc_client,
    mut exempt_fd: ::core::ffi::c_int,
) {
    if (*client).fd != exempt_fd {
        debuglog(
            b"%s:%s:%d - Disconnecting client on fd %d\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/ipc.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"free_ipc_client\0" as *const u8 as *const ::core::ffi::c_char,
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
#[no_mangle]
pub unsafe extern "C" fn ipc_send_event(
    mut event: *const ::core::ffi::c_char,
    mut message_type: uint32_t,
    mut payload: *const ::core::ffi::c_char,
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
unsafe extern "C" fn ipc_send_shutdown_event(mut reason: shutdown_reason_t) {
    let mut gen: yajl_gen = yajl_gen_alloc(::core::ptr::null::<yajl_alloc_funcs>());
    yajl_gen_map_open(gen);
    yajl_gen_string(
        gen,
        b"change\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
        strlen(b"change\0" as *const u8 as *const ::core::ffi::c_char),
    );
    if reason as ::core::ffi::c_uint
        == SHUTDOWN_REASON_RESTART as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        yajl_gen_string(
            gen,
            b"restart\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
            strlen(b"restart\0" as *const u8 as *const ::core::ffi::c_char),
        );
    } else if reason as ::core::ffi::c_uint
        == SHUTDOWN_REASON_EXIT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        yajl_gen_string(
            gen,
            b"exit\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
            strlen(b"exit\0" as *const u8 as *const ::core::ffi::c_char),
        );
    }
    yajl_gen_map_close(gen);
    let mut payload: *const ::core::ffi::c_uchar = ::core::ptr::null::<::core::ffi::c_uchar>();
    let mut length: ylength = 0;
    yajl_gen_get_buf(gen, &raw mut payload, &raw mut length);
    ipc_send_event(
        b"shutdown\0" as *const u8 as *const ::core::ffi::c_char,
        I3_IPC_EVENT_SHUTDOWN as uint32_t,
        payload as *const ::core::ffi::c_char,
    );
    yajl_gen_free(gen);
}
#[no_mangle]
pub unsafe extern "C" fn ipc_shutdown(
    mut reason: shutdown_reason_t,
    mut exempt_fd: ::core::ffi::c_int,
) {
    ipc_send_shutdown_event(reason);
    while !all_clients.tqh_first.is_null() {
        let mut current: *mut ipc_client = all_clients.tqh_first as *mut ipc_client;
        if (*current).fd != exempt_fd {
            shutdown((*current).fd, SHUT_RDWR as ::core::ffi::c_int);
        }
        free_ipc_client(current, exempt_fd);
    }
}
unsafe extern "C" fn handle_run_command(
    mut client: *mut ipc_client,
    mut message: *mut uint8_t,
    mut size: ::core::ffi::c_int,
    mut message_size: uint32_t,
    mut message_type: uint32_t,
) {
    let mut command: *mut ::core::ffi::c_char = sstrndup(
        message as *const ::core::ffi::c_char,
        message_size as size_t,
    );
    verboselog(
        b"IPC: received: *%.4000s*\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        command,
    );
    let mut gen: yajl_gen = yajl_gen_alloc(::core::ptr::null::<yajl_alloc_funcs>());
    let mut result: *mut CommandResult = parse_command(command, gen, client);
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
unsafe extern "C" fn dump_rect(
    mut gen: yajl_gen,
    mut name: *const ::core::ffi::c_char,
    mut r: Rect,
) {
    yajl_gen_string(gen, name as *mut ::core::ffi::c_uchar, strlen(name));
    yajl_gen_map_open(gen);
    yajl_gen_string(
        gen,
        b"x\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
        strlen(b"x\0" as *const u8 as *const ::core::ffi::c_char),
    );
    yajl_gen_integer(gen, r.x as int32_t as ::core::ffi::c_longlong);
    yajl_gen_string(
        gen,
        b"y\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
        strlen(b"y\0" as *const u8 as *const ::core::ffi::c_char),
    );
    yajl_gen_integer(gen, r.y as int32_t as ::core::ffi::c_longlong);
    yajl_gen_string(
        gen,
        b"width\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
        strlen(b"width\0" as *const u8 as *const ::core::ffi::c_char),
    );
    yajl_gen_integer(gen, r.width as ::core::ffi::c_longlong);
    yajl_gen_string(
        gen,
        b"height\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
        strlen(b"height\0" as *const u8 as *const ::core::ffi::c_char),
    );
    yajl_gen_integer(gen, r.height as ::core::ffi::c_longlong);
    yajl_gen_map_close(gen);
}
unsafe extern "C" fn dump_gaps(
    mut gen: yajl_gen,
    mut name: *const ::core::ffi::c_char,
    mut gaps: gaps_t,
) {
    yajl_gen_string(gen, name as *mut ::core::ffi::c_uchar, strlen(name));
    yajl_gen_map_open(gen);
    yajl_gen_string(
        gen,
        b"inner\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
        strlen(b"inner\0" as *const u8 as *const ::core::ffi::c_char),
    );
    yajl_gen_integer(gen, gaps.inner as ::core::ffi::c_longlong);
    yajl_gen_string(
        gen,
        b"outer\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
        strlen(b"outer\0" as *const u8 as *const ::core::ffi::c_char),
    );
    yajl_gen_integer(gen, gaps.top as ::core::ffi::c_longlong);
    yajl_gen_string(
        gen,
        b"top\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
        strlen(b"top\0" as *const u8 as *const ::core::ffi::c_char),
    );
    yajl_gen_integer(gen, gaps.top as ::core::ffi::c_longlong);
    yajl_gen_string(
        gen,
        b"right\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
        strlen(b"right\0" as *const u8 as *const ::core::ffi::c_char),
    );
    yajl_gen_integer(gen, gaps.right as ::core::ffi::c_longlong);
    yajl_gen_string(
        gen,
        b"bottom\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
        strlen(b"bottom\0" as *const u8 as *const ::core::ffi::c_char),
    );
    yajl_gen_integer(gen, gaps.bottom as ::core::ffi::c_longlong);
    yajl_gen_string(
        gen,
        b"left\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
        strlen(b"left\0" as *const u8 as *const ::core::ffi::c_char),
    );
    yajl_gen_integer(gen, gaps.left as ::core::ffi::c_longlong);
    yajl_gen_map_close(gen);
}
unsafe extern "C" fn dump_event_state_mask(mut gen: yajl_gen, mut bind: *mut Binding) {
    yajl_gen_array_open(gen);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < 20 as ::core::ffi::c_int {
        if (*bind).event_state_mask & ((1 as ::core::ffi::c_int) << i) as i3_event_state_mask_t != 0
        {
            match (1 as ::core::ffi::c_int) << i {
                1 => {
                    yajl_gen_string(
                        gen,
                        b"shift\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_uchar,
                        strlen(b"shift\0" as *const u8 as *const ::core::ffi::c_char),
                    );
                }
                2 => {
                    yajl_gen_string(
                        gen,
                        b"lock\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_uchar,
                        strlen(b"lock\0" as *const u8 as *const ::core::ffi::c_char),
                    );
                }
                4 => {
                    yajl_gen_string(
                        gen,
                        b"ctrl\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_uchar,
                        strlen(b"ctrl\0" as *const u8 as *const ::core::ffi::c_char),
                    );
                }
                8 => {
                    yajl_gen_string(
                        gen,
                        b"Mod1\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_uchar,
                        strlen(b"Mod1\0" as *const u8 as *const ::core::ffi::c_char),
                    );
                }
                16 => {
                    yajl_gen_string(
                        gen,
                        b"Mod2\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_uchar,
                        strlen(b"Mod2\0" as *const u8 as *const ::core::ffi::c_char),
                    );
                }
                32 => {
                    yajl_gen_string(
                        gen,
                        b"Mod3\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_uchar,
                        strlen(b"Mod3\0" as *const u8 as *const ::core::ffi::c_char),
                    );
                }
                64 => {
                    yajl_gen_string(
                        gen,
                        b"Mod4\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_uchar,
                        strlen(b"Mod4\0" as *const u8 as *const ::core::ffi::c_char),
                    );
                }
                128 => {
                    yajl_gen_string(
                        gen,
                        b"Mod5\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_uchar,
                        strlen(b"Mod5\0" as *const u8 as *const ::core::ffi::c_char),
                    );
                }
                256 => {
                    yajl_gen_string(
                        gen,
                        b"Button1\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_uchar,
                        strlen(b"Button1\0" as *const u8 as *const ::core::ffi::c_char),
                    );
                }
                512 => {
                    yajl_gen_string(
                        gen,
                        b"Button2\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_uchar,
                        strlen(b"Button2\0" as *const u8 as *const ::core::ffi::c_char),
                    );
                }
                1024 => {
                    yajl_gen_string(
                        gen,
                        b"Button3\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_uchar,
                        strlen(b"Button3\0" as *const u8 as *const ::core::ffi::c_char),
                    );
                }
                2048 => {
                    yajl_gen_string(
                        gen,
                        b"Button4\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_uchar,
                        strlen(b"Button4\0" as *const u8 as *const ::core::ffi::c_char),
                    );
                }
                4096 => {
                    yajl_gen_string(
                        gen,
                        b"Button5\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_uchar,
                        strlen(b"Button5\0" as *const u8 as *const ::core::ffi::c_char),
                    );
                }
                65536 => {
                    yajl_gen_string(
                        gen,
                        b"Group1\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_uchar,
                        strlen(b"Group1\0" as *const u8 as *const ::core::ffi::c_char),
                    );
                }
                131072 => {
                    yajl_gen_string(
                        gen,
                        b"Group2\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_uchar,
                        strlen(b"Group2\0" as *const u8 as *const ::core::ffi::c_char),
                    );
                }
                262144 => {
                    yajl_gen_string(
                        gen,
                        b"Group3\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_uchar,
                        strlen(b"Group3\0" as *const u8 as *const ::core::ffi::c_char),
                    );
                }
                524288 => {
                    yajl_gen_string(
                        gen,
                        b"Group4\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_uchar,
                        strlen(b"Group4\0" as *const u8 as *const ::core::ffi::c_char),
                    );
                }
                _ => {}
            }
        }
        i += 1;
    }
    yajl_gen_array_close(gen);
}
unsafe extern "C" fn dump_binding(mut gen: yajl_gen, mut bind: *mut Binding) {
    yajl_gen_map_open(gen);
    yajl_gen_string(
        gen,
        b"input_code\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
        strlen(b"input_code\0" as *const u8 as *const ::core::ffi::c_char),
    );
    yajl_gen_integer(gen, (*bind).keycode as ::core::ffi::c_longlong);
    yajl_gen_string(
        gen,
        b"input_type\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
        strlen(b"input_type\0" as *const u8 as *const ::core::ffi::c_char),
    );
    yajl_gen_string(
        gen,
        (if (*bind).input_type as ::core::ffi::c_uint
            == B_KEYBOARD as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            b"keyboard\0" as *const u8 as *const ::core::ffi::c_char
        } else {
            b"mouse\0" as *const u8 as *const ::core::ffi::c_char
        }) as *mut ::core::ffi::c_uchar,
        strlen(
            if (*bind).input_type as ::core::ffi::c_uint
                == B_KEYBOARD as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                b"keyboard\0" as *const u8 as *const ::core::ffi::c_char
            } else {
                b"mouse\0" as *const u8 as *const ::core::ffi::c_char
            },
        ),
    );
    yajl_gen_string(
        gen,
        b"symbol\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
        strlen(b"symbol\0" as *const u8 as *const ::core::ffi::c_char),
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
        b"command\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
        strlen(b"command\0" as *const u8 as *const ::core::ffi::c_char),
    );
    yajl_gen_string(
        gen,
        (*bind).command as *mut ::core::ffi::c_uchar,
        strlen((*bind).command),
    );
    yajl_gen_string(
        gen,
        b"mods\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
        strlen(b"mods\0" as *const u8 as *const ::core::ffi::c_char),
    );
    dump_event_state_mask(gen, bind);
    yajl_gen_string(
        gen,
        b"event_state_mask\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_uchar,
        strlen(b"event_state_mask\0" as *const u8 as *const ::core::ffi::c_char),
    );
    dump_event_state_mask(gen, bind);
    yajl_gen_map_close(gen);
}
#[no_mangle]
pub unsafe extern "C" fn dump_node(
    mut gen: yajl_gen,
    mut con: *mut Con,
    mut inplace_restart: bool,
) {
    yajl_gen_map_open(gen);
    yajl_gen_string(
        gen,
        b"id\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
        strlen(b"id\0" as *const u8 as *const ::core::ffi::c_char),
    );
    yajl_gen_integer(gen, con as uintptr_t as ::core::ffi::c_longlong);
    yajl_gen_string(
        gen,
        b"type\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
        strlen(b"type\0" as *const u8 as *const ::core::ffi::c_char),
    );
    match (*con).type_0 as ::core::ffi::c_uint {
        0 => {
            yajl_gen_string(
                gen,
                b"root\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
                strlen(b"root\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        1 => {
            yajl_gen_string(
                gen,
                b"output\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
                strlen(b"output\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        2 => {
            yajl_gen_string(
                gen,
                b"con\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
                strlen(b"con\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        3 => {
            yajl_gen_string(
                gen,
                b"floating_con\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_uchar,
                strlen(b"floating_con\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        4 => {
            yajl_gen_string(
                gen,
                b"workspace\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_uchar,
                strlen(b"workspace\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        5 => {
            yajl_gen_string(
                gen,
                b"dockarea\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_uchar,
                strlen(b"dockarea\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        _ => {}
    }
    yajl_gen_string(
        gen,
        b"orientation\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
        strlen(b"orientation\0" as *const u8 as *const ::core::ffi::c_char),
    );
    if !con_is_split(con) {
        yajl_gen_string(
            gen,
            b"none\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
            strlen(b"none\0" as *const u8 as *const ::core::ffi::c_char),
        );
    } else if con_orientation(con) as ::core::ffi::c_uint
        == HORIZ as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        yajl_gen_string(
            gen,
            b"horizontal\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
            strlen(b"horizontal\0" as *const u8 as *const ::core::ffi::c_char),
        );
    } else {
        yajl_gen_string(
            gen,
            b"vertical\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
            strlen(b"vertical\0" as *const u8 as *const ::core::ffi::c_char),
        );
    }
    yajl_gen_string(
        gen,
        b"scratchpad_state\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_uchar,
        strlen(b"scratchpad_state\0" as *const u8 as *const ::core::ffi::c_char),
    );
    match (*con).scratchpad_state as ::core::ffi::c_uint {
        0 => {
            yajl_gen_string(
                gen,
                b"none\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
                strlen(b"none\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        1 => {
            yajl_gen_string(
                gen,
                b"fresh\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
                strlen(b"fresh\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        2 => {
            yajl_gen_string(
                gen,
                b"changed\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_uchar,
                strlen(b"changed\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        _ => {}
    }
    yajl_gen_string(
        gen,
        b"percent\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
        strlen(b"percent\0" as *const u8 as *const ::core::ffi::c_char),
    );
    if (*con).percent == 0.0f64 {
        yajl_gen_null(gen);
    } else {
        yajl_gen_double(gen, (*con).percent);
    }
    yajl_gen_string(
        gen,
        b"urgent\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
        strlen(b"urgent\0" as *const u8 as *const ::core::ffi::c_char),
    );
    yajl_gen_bool(gen, (*con).urgent as ::core::ffi::c_int);
    yajl_gen_string(
        gen,
        b"marks\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
        strlen(b"marks\0" as *const u8 as *const ::core::ffi::c_char),
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
        b"focused\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
        strlen(b"focused\0" as *const u8 as *const ::core::ffi::c_char),
    );
    yajl_gen_bool(gen, (con == focused) as ::core::ffi::c_int);
    if (*con).type_0 as ::core::ffi::c_uint != CT_ROOT as ::core::ffi::c_int as ::core::ffi::c_uint
        && (*con).type_0 as ::core::ffi::c_uint
            != CT_OUTPUT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        yajl_gen_string(
            gen,
            b"output\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
            strlen(b"output\0" as *const u8 as *const ::core::ffi::c_char),
        );
        yajl_gen_string(
            gen,
            (*con_get_output(con)).name as *mut ::core::ffi::c_uchar,
            strlen((*con_get_output(con)).name),
        );
    }
    yajl_gen_string(
        gen,
        b"layout\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
        strlen(b"layout\0" as *const u8 as *const ::core::ffi::c_char),
    );
    match (*con).layout as ::core::ffi::c_uint {
        0 => {
            debuglog(
                b"%s:%s:%d - About to dump layout=default, this is a bug in the code.\n\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"../src/ipc.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"dump_node\0" as *const u8 as *const ::core::ffi::c_char,
                442 as ::core::ffi::c_int,
            );
            if 0 as ::core::ffi::c_int != 0 {
            } else {
                __assert_fail(
                    b"false\0" as *const u8 as *const ::core::ffi::c_char,
                    b"../src/ipc.c\0" as *const u8 as *const ::core::ffi::c_char,
                    443 as ::core::ffi::c_uint,
                    b"void dump_node(yajl_gen, Con *, _Bool)\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            };
        }
        5 => {
            yajl_gen_string(
                gen,
                b"splitv\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
                strlen(b"splitv\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        6 => {
            yajl_gen_string(
                gen,
                b"splith\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
                strlen(b"splith\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        1 => {
            yajl_gen_string(
                gen,
                b"stacked\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_uchar,
                strlen(b"stacked\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        2 => {
            yajl_gen_string(
                gen,
                b"tabbed\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
                strlen(b"tabbed\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        3 => {
            yajl_gen_string(
                gen,
                b"dockarea\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_uchar,
                strlen(b"dockarea\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        4 => {
            yajl_gen_string(
                gen,
                b"output\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
                strlen(b"output\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        _ => {}
    }
    yajl_gen_string(
        gen,
        b"workspace_layout\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_uchar,
        strlen(b"workspace_layout\0" as *const u8 as *const ::core::ffi::c_char),
    );
    match (*con).workspace_layout as ::core::ffi::c_uint {
        0 => {
            yajl_gen_string(
                gen,
                b"default\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_uchar,
                strlen(b"default\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        1 => {
            yajl_gen_string(
                gen,
                b"stacked\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_uchar,
                strlen(b"stacked\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        2 => {
            yajl_gen_string(
                gen,
                b"tabbed\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
                strlen(b"tabbed\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        _ => {
            debuglog(
                b"%s:%s:%d - About to dump workspace_layout=%d (none of default/stacked/tabbed), this is a bug.\n\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"../src/ipc.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"dump_node\0" as *const u8 as *const ::core::ffi::c_char,
                477 as ::core::ffi::c_int,
                (*con).workspace_layout as ::core::ffi::c_uint,
            );
            if 0 as ::core::ffi::c_int != 0 {
            } else {
                __assert_fail(
                    b"false\0" as *const u8 as *const ::core::ffi::c_char,
                    b"../src/ipc.c\0" as *const u8 as *const ::core::ffi::c_char,
                    478 as ::core::ffi::c_uint,
                    b"void dump_node(yajl_gen, Con *, _Bool)\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            };
        }
    }
    yajl_gen_string(
        gen,
        b"last_split_layout\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_uchar,
        strlen(b"last_split_layout\0" as *const u8 as *const ::core::ffi::c_char),
    );
    match (*con).layout as ::core::ffi::c_uint {
        5 => {
            yajl_gen_string(
                gen,
                b"splitv\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
                strlen(b"splitv\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        _ => {
            yajl_gen_string(
                gen,
                b"splith\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
                strlen(b"splith\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
    }
    yajl_gen_string(
        gen,
        b"border\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
        strlen(b"border\0" as *const u8 as *const ::core::ffi::c_char),
    );
    match (*con).border_style as ::core::ffi::c_uint {
        2 => {
            yajl_gen_string(
                gen,
                b"normal\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
                strlen(b"normal\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        0 => {
            yajl_gen_string(
                gen,
                b"none\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
                strlen(b"none\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        1 => {
            yajl_gen_string(
                gen,
                b"pixel\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
                strlen(b"pixel\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        _ => {}
    }
    yajl_gen_string(
        gen,
        b"current_border_width\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_uchar,
        strlen(b"current_border_width\0" as *const u8 as *const ::core::ffi::c_char),
    );
    yajl_gen_integer(gen, (*con).current_border_width as ::core::ffi::c_longlong);
    dump_rect(
        gen,
        b"rect\0" as *const u8 as *const ::core::ffi::c_char,
        (*con).rect as Rect,
    );
    if con_draw_decoration_into_frame(con) {
        let mut simulated_deco_rect: Rect = (*con).deco_rect as Rect;
        simulated_deco_rect.x = (*con).rect.x.wrapping_sub((*(*con).parent).rect.x);
        simulated_deco_rect.y = (*con).rect.y.wrapping_sub((*(*con).parent).rect.y);
        dump_rect(
            gen,
            b"deco_rect\0" as *const u8 as *const ::core::ffi::c_char,
            simulated_deco_rect,
        );
        dump_rect(
            gen,
            b"actual_deco_rect\0" as *const u8 as *const ::core::ffi::c_char,
            (*con).deco_rect as Rect,
        );
    } else {
        dump_rect(
            gen,
            b"deco_rect\0" as *const u8 as *const ::core::ffi::c_char,
            (*con).deco_rect as Rect,
        );
    }
    dump_rect(
        gen,
        b"window_rect\0" as *const u8 as *const ::core::ffi::c_char,
        (*con).window_rect as Rect,
    );
    dump_rect(
        gen,
        b"geometry\0" as *const u8 as *const ::core::ffi::c_char,
        (*con).geometry as Rect,
    );
    yajl_gen_string(
        gen,
        b"name\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
        strlen(b"name\0" as *const u8 as *const ::core::ffi::c_char),
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
            b"title_format\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_uchar,
            strlen(b"title_format\0" as *const u8 as *const ::core::ffi::c_char),
        );
        yajl_gen_string(
            gen,
            (*con).title_format as *mut ::core::ffi::c_uchar,
            strlen((*con).title_format),
        );
    }
    yajl_gen_string(
        gen,
        b"window_icon_padding\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_uchar,
        strlen(b"window_icon_padding\0" as *const u8 as *const ::core::ffi::c_char),
    );
    yajl_gen_integer(gen, (*con).window_icon_padding as ::core::ffi::c_longlong);
    if (*con).type_0 as ::core::ffi::c_uint
        == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        yajl_gen_string(
            gen,
            b"num\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
            strlen(b"num\0" as *const u8 as *const ::core::ffi::c_char),
        );
        yajl_gen_integer(gen, (*con).num as ::core::ffi::c_longlong);
        dump_gaps(
            gen,
            b"gaps\0" as *const u8 as *const ::core::ffi::c_char,
            (*con).gaps,
        );
    }
    yajl_gen_string(
        gen,
        b"window\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
        strlen(b"window\0" as *const u8 as *const ::core::ffi::c_char),
    );
    if !(*con).window.is_null() {
        yajl_gen_integer(gen, (*(*con).window).id as ::core::ffi::c_longlong);
    } else {
        yajl_gen_null(gen);
    }
    yajl_gen_string(
        gen,
        b"window_type\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
        strlen(b"window_type\0" as *const u8 as *const ::core::ffi::c_char),
    );
    if !(*con).window.is_null() {
        if (*(*con).window).window_type == A__NET_WM_WINDOW_TYPE_NORMAL {
            yajl_gen_string(
                gen,
                b"normal\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
                strlen(b"normal\0" as *const u8 as *const ::core::ffi::c_char),
            );
        } else if (*(*con).window).window_type == A__NET_WM_WINDOW_TYPE_DOCK {
            yajl_gen_string(
                gen,
                b"dock\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
                strlen(b"dock\0" as *const u8 as *const ::core::ffi::c_char),
            );
        } else if (*(*con).window).window_type == A__NET_WM_WINDOW_TYPE_DIALOG {
            yajl_gen_string(
                gen,
                b"dialog\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
                strlen(b"dialog\0" as *const u8 as *const ::core::ffi::c_char),
            );
        } else if (*(*con).window).window_type == A__NET_WM_WINDOW_TYPE_UTILITY {
            yajl_gen_string(
                gen,
                b"utility\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_uchar,
                strlen(b"utility\0" as *const u8 as *const ::core::ffi::c_char),
            );
        } else if (*(*con).window).window_type == A__NET_WM_WINDOW_TYPE_TOOLBAR {
            yajl_gen_string(
                gen,
                b"toolbar\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_uchar,
                strlen(b"toolbar\0" as *const u8 as *const ::core::ffi::c_char),
            );
        } else if (*(*con).window).window_type == A__NET_WM_WINDOW_TYPE_SPLASH {
            yajl_gen_string(
                gen,
                b"splash\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
                strlen(b"splash\0" as *const u8 as *const ::core::ffi::c_char),
            );
        } else if (*(*con).window).window_type == A__NET_WM_WINDOW_TYPE_MENU {
            yajl_gen_string(
                gen,
                b"menu\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
                strlen(b"menu\0" as *const u8 as *const ::core::ffi::c_char),
            );
        } else if (*(*con).window).window_type == A__NET_WM_WINDOW_TYPE_DROPDOWN_MENU {
            yajl_gen_string(
                gen,
                b"dropdown_menu\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_uchar,
                strlen(b"dropdown_menu\0" as *const u8 as *const ::core::ffi::c_char),
            );
        } else if (*(*con).window).window_type == A__NET_WM_WINDOW_TYPE_POPUP_MENU {
            yajl_gen_string(
                gen,
                b"popup_menu\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_uchar,
                strlen(b"popup_menu\0" as *const u8 as *const ::core::ffi::c_char),
            );
        } else if (*(*con).window).window_type == A__NET_WM_WINDOW_TYPE_TOOLTIP {
            yajl_gen_string(
                gen,
                b"tooltip\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_uchar,
                strlen(b"tooltip\0" as *const u8 as *const ::core::ffi::c_char),
            );
        } else if (*(*con).window).window_type == A__NET_WM_WINDOW_TYPE_NOTIFICATION {
            yajl_gen_string(
                gen,
                b"notification\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_uchar,
                strlen(b"notification\0" as *const u8 as *const ::core::ffi::c_char),
            );
        } else {
            yajl_gen_string(
                gen,
                b"unknown\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_uchar,
                strlen(b"unknown\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
    } else {
        yajl_gen_null(gen);
    }
    if !(*con).window.is_null() && !inplace_restart {
        yajl_gen_string(
            gen,
            b"window_properties\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_uchar,
            strlen(b"window_properties\0" as *const u8 as *const ::core::ffi::c_char),
        );
        yajl_gen_map_open(gen);
        if !(*(*con).window).class_class.is_null() {
            yajl_gen_string(
                gen,
                b"class\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
                strlen(b"class\0" as *const u8 as *const ::core::ffi::c_char),
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
                b"instance\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_uchar,
                strlen(b"instance\0" as *const u8 as *const ::core::ffi::c_char),
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
                b"window_role\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_uchar,
                strlen(b"window_role\0" as *const u8 as *const ::core::ffi::c_char),
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
                b"machine\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_uchar,
                strlen(b"machine\0" as *const u8 as *const ::core::ffi::c_char),
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
                b"title\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
                strlen(b"title\0" as *const u8 as *const ::core::ffi::c_char),
            );
            yajl_gen_string(
                gen,
                i3string_as_utf8((*(*con).window).name) as *mut ::core::ffi::c_uchar,
                strlen(i3string_as_utf8((*(*con).window).name)),
            );
        }
        yajl_gen_string(
            gen,
            b"transient_for\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_uchar,
            strlen(b"transient_for\0" as *const u8 as *const ::core::ffi::c_char),
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
        b"nodes\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
        strlen(b"nodes\0" as *const u8 as *const ::core::ffi::c_char),
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
        b"floating_nodes\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
        strlen(b"floating_nodes\0" as *const u8 as *const ::core::ffi::c_char),
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
        b"focus\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
        strlen(b"focus\0" as *const u8 as *const ::core::ffi::c_char),
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
        b"fullscreen_mode\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_uchar,
        strlen(b"fullscreen_mode\0" as *const u8 as *const ::core::ffi::c_char),
    );
    yajl_gen_integer(gen, (*con).fullscreen_mode as ::core::ffi::c_longlong);
    yajl_gen_string(
        gen,
        b"sticky\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
        strlen(b"sticky\0" as *const u8 as *const ::core::ffi::c_char),
    );
    yajl_gen_bool(gen, (*con).sticky as ::core::ffi::c_int);
    yajl_gen_string(
        gen,
        b"floating\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
        strlen(b"floating\0" as *const u8 as *const ::core::ffi::c_char),
    );
    match (*con).floating as ::core::ffi::c_uint {
        0 => {
            yajl_gen_string(
                gen,
                b"auto_off\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_uchar,
                strlen(b"auto_off\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        2 => {
            yajl_gen_string(
                gen,
                b"auto_on\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_uchar,
                strlen(b"auto_on\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        1 => {
            yajl_gen_string(
                gen,
                b"user_off\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_uchar,
                strlen(b"user_off\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        3 => {
            yajl_gen_string(
                gen,
                b"user_on\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_uchar,
                strlen(b"user_on\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        _ => {}
    }
    yajl_gen_string(
        gen,
        b"swallows\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
        strlen(b"swallows\0" as *const u8 as *const ::core::ffi::c_char),
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
                    b"dock\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"dock\0" as *const u8 as *const ::core::ffi::c_char),
                );
                yajl_gen_integer(gen, (*match_0).dock as ::core::ffi::c_longlong);
                yajl_gen_string(
                    gen,
                    b"insert_where\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"insert_where\0" as *const u8 as *const ::core::ffi::c_char),
                );
                yajl_gen_integer(gen, (*match_0).insert_where as ::core::ffi::c_longlong);
            }
            if !(*match_0).class.is_null() {
                yajl_gen_string(
                    gen,
                    b"class\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"class\0" as *const u8 as *const ::core::ffi::c_char),
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
                    b"instance\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"instance\0" as *const u8 as *const ::core::ffi::c_char),
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
                    b"window_role\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"window_role\0" as *const u8 as *const ::core::ffi::c_char),
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
                    b"title\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"title\0" as *const u8 as *const ::core::ffi::c_char),
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
                    b"machine\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"machine\0" as *const u8 as *const ::core::ffi::c_char),
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
                b"id\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
                strlen(b"id\0" as *const u8 as *const ::core::ffi::c_char),
            );
            yajl_gen_integer(gen, (*(*con).window).id as ::core::ffi::c_longlong);
            yajl_gen_string(
                gen,
                b"restart_mode\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_uchar,
                strlen(b"restart_mode\0" as *const u8 as *const ::core::ffi::c_char),
            );
            yajl_gen_bool(gen, true_0);
            yajl_gen_map_close(gen);
        }
    }
    yajl_gen_array_close(gen);
    if inplace_restart as ::core::ffi::c_int != 0 && !(*con).window.is_null() {
        yajl_gen_string(
            gen,
            b"depth\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
            strlen(b"depth\0" as *const u8 as *const ::core::ffi::c_char),
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
            b"previous_workspace_name\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_uchar,
            strlen(b"previous_workspace_name\0" as *const u8 as *const ::core::ffi::c_char),
        );
        yajl_gen_string(
            gen,
            previous_workspace_name as *mut ::core::ffi::c_uchar,
            strlen(previous_workspace_name),
        );
    }
    yajl_gen_map_close(gen);
}
unsafe extern "C" fn dump_bar_bindings(mut gen: yajl_gen, mut config: *mut Barconfig) {
    if (*config).bar_bindings.tqh_first.is_null() {
        return;
    }
    yajl_gen_string(
        gen,
        b"bindings\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
        strlen(b"bindings\0" as *const u8 as *const ::core::ffi::c_char),
    );
    yajl_gen_array_open(gen);
    let mut current: *mut Barbinding = ::core::ptr::null_mut::<Barbinding>();
    current = (*config).bar_bindings.tqh_first as *mut Barbinding;
    while !current.is_null() {
        yajl_gen_map_open(gen);
        yajl_gen_string(
            gen,
            b"input_code\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
            strlen(b"input_code\0" as *const u8 as *const ::core::ffi::c_char),
        );
        yajl_gen_integer(gen, (*current).input_code as ::core::ffi::c_longlong);
        yajl_gen_string(
            gen,
            b"command\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
            strlen(b"command\0" as *const u8 as *const ::core::ffi::c_char),
        );
        yajl_gen_string(
            gen,
            (*current).command as *mut ::core::ffi::c_uchar,
            strlen((*current).command),
        );
        yajl_gen_string(
            gen,
            b"release\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
            strlen(b"release\0" as *const u8 as *const ::core::ffi::c_char),
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
unsafe extern "C" fn canonicalize_output_name(
    mut name: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    if strcasecmp(
        name,
        b"primary\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
        || strcasecmp(
            name,
            b"nonprimary\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
    {
        return name;
    }
    let mut output: *mut Output = get_output_by_name(name, false_0 != 0);
    return if !output.is_null() {
        output_primary_name(output)
    } else {
        name
    };
}
unsafe extern "C" fn dump_bar_config(mut gen: yajl_gen, mut config: *mut Barconfig) {
    yajl_gen_map_open(gen);
    yajl_gen_string(
        gen,
        b"id\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
        strlen(b"id\0" as *const u8 as *const ::core::ffi::c_char),
    );
    yajl_gen_string(
        gen,
        (*config).id as *mut ::core::ffi::c_uchar,
        strlen((*config).id),
    );
    if (*config).num_outputs > 0 as ::core::ffi::c_int {
        yajl_gen_string(
            gen,
            b"outputs\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
            strlen(b"outputs\0" as *const u8 as *const ::core::ffi::c_char),
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
            b"tray_outputs\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_uchar,
            strlen(b"tray_outputs\0" as *const u8 as *const ::core::ffi::c_char),
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
        b"tray_padding\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
        strlen(b"tray_padding\0" as *const u8 as *const ::core::ffi::c_char),
    );
    yajl_gen_integer(gen, (*config).tray_padding as ::core::ffi::c_longlong);
    if !(*config).socket_path.is_null() {
        yajl_gen_string(
            gen,
            b"socket_path\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_uchar,
            strlen(b"socket_path\0" as *const u8 as *const ::core::ffi::c_char),
        );
        yajl_gen_string(
            gen,
            (*config).socket_path as *mut ::core::ffi::c_uchar,
            strlen((*config).socket_path),
        );
    }
    yajl_gen_string(
        gen,
        b"mode\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
        strlen(b"mode\0" as *const u8 as *const ::core::ffi::c_char),
    );
    match (*config).mode as ::core::ffi::c_uint {
        1 => {
            yajl_gen_string(
                gen,
                b"hide\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
                strlen(b"hide\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        2 => {
            yajl_gen_string(
                gen,
                b"invisible\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_uchar,
                strlen(b"invisible\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        0 | _ => {
            yajl_gen_string(
                gen,
                b"dock\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
                strlen(b"dock\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
    }
    yajl_gen_string(
        gen,
        b"hidden_state\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
        strlen(b"hidden_state\0" as *const u8 as *const ::core::ffi::c_char),
    );
    match (*config).hidden_state as ::core::ffi::c_uint {
        1 => {
            yajl_gen_string(
                gen,
                b"show\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
                strlen(b"show\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        0 | _ => {
            yajl_gen_string(
                gen,
                b"hide\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
                strlen(b"hide\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
    }
    yajl_gen_string(
        gen,
        b"modifier\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
        strlen(b"modifier\0" as *const u8 as *const ::core::ffi::c_char),
    );
    yajl_gen_integer(gen, (*config).modifier as ::core::ffi::c_longlong);
    dump_bar_bindings(gen, config);
    yajl_gen_string(
        gen,
        b"position\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
        strlen(b"position\0" as *const u8 as *const ::core::ffi::c_char),
    );
    if (*config).position as ::core::ffi::c_uint
        == P_BOTTOM as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        yajl_gen_string(
            gen,
            b"bottom\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
            strlen(b"bottom\0" as *const u8 as *const ::core::ffi::c_char),
        );
    } else {
        yajl_gen_string(
            gen,
            b"top\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
            strlen(b"top\0" as *const u8 as *const ::core::ffi::c_char),
        );
    }
    if !(*config).status_command.is_null() {
        yajl_gen_string(
            gen,
            b"status_command\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_uchar,
            strlen(b"status_command\0" as *const u8 as *const ::core::ffi::c_char),
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
            b"workspace_command\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_uchar,
            strlen(b"workspace_command\0" as *const u8 as *const ::core::ffi::c_char),
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
            b"font\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
            strlen(b"font\0" as *const u8 as *const ::core::ffi::c_char),
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
            b"bar_height\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
            strlen(b"bar_height\0" as *const u8 as *const ::core::ffi::c_char),
        );
        yajl_gen_integer(gen, (*config).bar_height as ::core::ffi::c_longlong);
    }
    dump_rect(
        gen,
        b"padding\0" as *const u8 as *const ::core::ffi::c_char,
        (*config).padding as Rect,
    );
    if !(*config).separator_symbol.is_null() {
        yajl_gen_string(
            gen,
            b"separator_symbol\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_uchar,
            strlen(b"separator_symbol\0" as *const u8 as *const ::core::ffi::c_char),
        );
        yajl_gen_string(
            gen,
            (*config).separator_symbol as *mut ::core::ffi::c_uchar,
            strlen((*config).separator_symbol),
        );
    }
    yajl_gen_string(
        gen,
        b"workspace_buttons\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_uchar,
        strlen(b"workspace_buttons\0" as *const u8 as *const ::core::ffi::c_char),
    );
    yajl_gen_bool(gen, !(*config).hide_workspace_buttons as ::core::ffi::c_int);
    yajl_gen_string(
        gen,
        b"workspace_min_width\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_uchar,
        strlen(b"workspace_min_width\0" as *const u8 as *const ::core::ffi::c_char),
    );
    yajl_gen_integer(
        gen,
        (*config).workspace_min_width as ::core::ffi::c_longlong,
    );
    yajl_gen_string(
        gen,
        b"strip_workspace_numbers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_uchar,
        strlen(b"strip_workspace_numbers\0" as *const u8 as *const ::core::ffi::c_char),
    );
    yajl_gen_bool(gen, (*config).strip_workspace_numbers as ::core::ffi::c_int);
    yajl_gen_string(
        gen,
        b"strip_workspace_name\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_uchar,
        strlen(b"strip_workspace_name\0" as *const u8 as *const ::core::ffi::c_char),
    );
    yajl_gen_bool(gen, (*config).strip_workspace_name as ::core::ffi::c_int);
    yajl_gen_string(
        gen,
        b"binding_mode_indicator\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_uchar,
        strlen(b"binding_mode_indicator\0" as *const u8 as *const ::core::ffi::c_char),
    );
    yajl_gen_bool(
        gen,
        !(*config).hide_binding_mode_indicator as ::core::ffi::c_int,
    );
    yajl_gen_string(
        gen,
        b"verbose\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
        strlen(b"verbose\0" as *const u8 as *const ::core::ffi::c_char),
    );
    yajl_gen_bool(gen, (*config).verbose as ::core::ffi::c_int);
    yajl_gen_string(
        gen,
        b"colors\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
        strlen(b"colors\0" as *const u8 as *const ::core::ffi::c_char),
    );
    yajl_gen_map_open(gen);
    if !(*config).colors.background.is_null() {
        yajl_gen_string(
            gen,
            b"background\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
            strlen(b"background\0" as *const u8 as *const ::core::ffi::c_char),
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
            b"statusline\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
            strlen(b"statusline\0" as *const u8 as *const ::core::ffi::c_char),
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
            b"separator\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
            strlen(b"separator\0" as *const u8 as *const ::core::ffi::c_char),
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
            b"focused_background\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_uchar,
            strlen(b"focused_background\0" as *const u8 as *const ::core::ffi::c_char),
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
            b"focused_statusline\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_uchar,
            strlen(b"focused_statusline\0" as *const u8 as *const ::core::ffi::c_char),
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
            b"focused_separator\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_uchar,
            strlen(b"focused_separator\0" as *const u8 as *const ::core::ffi::c_char),
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
            b"focused_workspace_border\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_uchar,
            strlen(b"focused_workspace_border\0" as *const u8 as *const ::core::ffi::c_char),
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
            b"focused_workspace_bg\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_uchar,
            strlen(b"focused_workspace_bg\0" as *const u8 as *const ::core::ffi::c_char),
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
            b"focused_workspace_text\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_uchar,
            strlen(b"focused_workspace_text\0" as *const u8 as *const ::core::ffi::c_char),
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
            b"active_workspace_border\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_uchar,
            strlen(b"active_workspace_border\0" as *const u8 as *const ::core::ffi::c_char),
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
            b"active_workspace_bg\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_uchar,
            strlen(b"active_workspace_bg\0" as *const u8 as *const ::core::ffi::c_char),
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
            b"active_workspace_text\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_uchar,
            strlen(b"active_workspace_text\0" as *const u8 as *const ::core::ffi::c_char),
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
            b"inactive_workspace_border\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_uchar,
            strlen(b"inactive_workspace_border\0" as *const u8 as *const ::core::ffi::c_char),
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
            b"inactive_workspace_bg\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_uchar,
            strlen(b"inactive_workspace_bg\0" as *const u8 as *const ::core::ffi::c_char),
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
            b"inactive_workspace_text\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_uchar,
            strlen(b"inactive_workspace_text\0" as *const u8 as *const ::core::ffi::c_char),
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
            b"urgent_workspace_border\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_uchar,
            strlen(b"urgent_workspace_border\0" as *const u8 as *const ::core::ffi::c_char),
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
            b"urgent_workspace_bg\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_uchar,
            strlen(b"urgent_workspace_bg\0" as *const u8 as *const ::core::ffi::c_char),
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
            b"urgent_workspace_text\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_uchar,
            strlen(b"urgent_workspace_text\0" as *const u8 as *const ::core::ffi::c_char),
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
            b"binding_mode_border\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_uchar,
            strlen(b"binding_mode_border\0" as *const u8 as *const ::core::ffi::c_char),
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
            b"binding_mode_bg\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_uchar,
            strlen(b"binding_mode_bg\0" as *const u8 as *const ::core::ffi::c_char),
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
            b"binding_mode_text\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_uchar,
            strlen(b"binding_mode_text\0" as *const u8 as *const ::core::ffi::c_char),
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
unsafe extern "C" fn handle_tree(
    mut client: *mut ipc_client,
    mut message: *mut uint8_t,
    mut size: ::core::ffi::c_int,
    mut message_size: uint32_t,
    mut message_type: uint32_t,
) {
    let mut prev_locale: *mut ::core::ffi::c_char = sstrdup(setlocale(
        LC_NUMERIC,
        ::core::ptr::null::<::core::ffi::c_char>(),
    ));
    setlocale(
        LC_NUMERIC,
        b"C\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut gen: yajl_gen = yajl_gen_alloc(::core::ptr::null::<yajl_alloc_funcs>());
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
unsafe extern "C" fn handle_get_workspaces(
    mut client: *mut ipc_client,
    mut message: *mut uint8_t,
    mut size: ::core::ffi::c_int,
    mut message_size: uint32_t,
    mut message_type: uint32_t,
) {
    let mut gen: yajl_gen = yajl_gen_alloc(::core::ptr::null::<yajl_alloc_funcs>());
    yajl_gen_array_open(gen);
    let mut focused_ws: *mut Con = con_get_workspace(focused);
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
                        b"ws->type == CT_WORKSPACE\0" as *const u8
                            as *const ::core::ffi::c_char,
                        b"../src/ipc.c\0" as *const u8 as *const ::core::ffi::c_char,
                        945 as ::core::ffi::c_uint,
                        b"void handle_get_workspaces(ipc_client *, uint8_t *, int, uint32_t, uint32_t)\0"
                            as *const u8 as *const ::core::ffi::c_char,
                    );
                };
                yajl_gen_map_open(gen);
                yajl_gen_string(
                    gen,
                    b"id\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
                    strlen(b"id\0" as *const u8 as *const ::core::ffi::c_char),
                );
                yajl_gen_integer(gen, ws as uintptr_t as ::core::ffi::c_longlong);
                yajl_gen_string(
                    gen,
                    b"num\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"num\0" as *const u8 as *const ::core::ffi::c_char),
                );
                yajl_gen_integer(gen, (*ws).num as ::core::ffi::c_longlong);
                yajl_gen_string(
                    gen,
                    b"name\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"name\0" as *const u8 as *const ::core::ffi::c_char),
                );
                yajl_gen_string(
                    gen,
                    (*ws).name as *mut ::core::ffi::c_uchar,
                    strlen((*ws).name),
                );
                yajl_gen_string(
                    gen,
                    b"visible\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"visible\0" as *const u8 as *const ::core::ffi::c_char),
                );
                yajl_gen_bool(gen, workspace_is_visible(ws) as ::core::ffi::c_int);
                yajl_gen_string(
                    gen,
                    b"focused\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"focused\0" as *const u8 as *const ::core::ffi::c_char),
                );
                yajl_gen_bool(gen, (ws == focused_ws) as ::core::ffi::c_int);
                yajl_gen_string(
                    gen,
                    b"rect\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"rect\0" as *const u8 as *const ::core::ffi::c_char),
                );
                yajl_gen_map_open(gen);
                yajl_gen_string(
                    gen,
                    b"x\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
                    strlen(b"x\0" as *const u8 as *const ::core::ffi::c_char),
                );
                yajl_gen_integer(gen, (*ws).rect.x as ::core::ffi::c_longlong);
                yajl_gen_string(
                    gen,
                    b"y\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
                    strlen(b"y\0" as *const u8 as *const ::core::ffi::c_char),
                );
                yajl_gen_integer(gen, (*ws).rect.y as ::core::ffi::c_longlong);
                yajl_gen_string(
                    gen,
                    b"width\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"width\0" as *const u8 as *const ::core::ffi::c_char),
                );
                yajl_gen_integer(gen, (*ws).rect.width as ::core::ffi::c_longlong);
                yajl_gen_string(
                    gen,
                    b"height\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"height\0" as *const u8 as *const ::core::ffi::c_char),
                );
                yajl_gen_integer(gen, (*ws).rect.height as ::core::ffi::c_longlong);
                yajl_gen_map_close(gen);
                yajl_gen_string(
                    gen,
                    b"output\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"output\0" as *const u8 as *const ::core::ffi::c_char),
                );
                yajl_gen_string(
                    gen,
                    (*output).name as *mut ::core::ffi::c_uchar,
                    strlen((*output).name),
                );
                yajl_gen_string(
                    gen,
                    b"urgent\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"urgent\0" as *const u8 as *const ::core::ffi::c_char),
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
unsafe extern "C" fn handle_get_outputs(
    mut client: *mut ipc_client,
    mut message: *mut uint8_t,
    mut size: ::core::ffi::c_int,
    mut message_size: uint32_t,
    mut message_type: uint32_t,
) {
    let mut gen: yajl_gen = yajl_gen_alloc(::core::ptr::null::<yajl_alloc_funcs>());
    yajl_gen_array_open(gen);
    let mut output: *mut Output = ::core::ptr::null_mut::<Output>();
    output = outputs.tqh_first as *mut Output;
    while !output.is_null() {
        yajl_gen_map_open(gen);
        yajl_gen_string(
            gen,
            b"name\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
            strlen(b"name\0" as *const u8 as *const ::core::ffi::c_char),
        );
        yajl_gen_string(
            gen,
            output_primary_name(output) as *mut ::core::ffi::c_uchar,
            strlen(output_primary_name(output)),
        );
        yajl_gen_string(
            gen,
            b"active\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
            strlen(b"active\0" as *const u8 as *const ::core::ffi::c_char),
        );
        yajl_gen_bool(gen, (*output).active as ::core::ffi::c_int);
        yajl_gen_string(
            gen,
            b"primary\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
            strlen(b"primary\0" as *const u8 as *const ::core::ffi::c_char),
        );
        yajl_gen_bool(gen, (*output).primary as ::core::ffi::c_int);
        yajl_gen_string(
            gen,
            b"rect\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
            strlen(b"rect\0" as *const u8 as *const ::core::ffi::c_char),
        );
        yajl_gen_map_open(gen);
        yajl_gen_string(
            gen,
            b"x\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
            strlen(b"x\0" as *const u8 as *const ::core::ffi::c_char),
        );
        yajl_gen_integer(gen, (*output).rect.x as ::core::ffi::c_longlong);
        yajl_gen_string(
            gen,
            b"y\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
            strlen(b"y\0" as *const u8 as *const ::core::ffi::c_char),
        );
        yajl_gen_integer(gen, (*output).rect.y as ::core::ffi::c_longlong);
        yajl_gen_string(
            gen,
            b"width\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
            strlen(b"width\0" as *const u8 as *const ::core::ffi::c_char),
        );
        yajl_gen_integer(gen, (*output).rect.width as ::core::ffi::c_longlong);
        yajl_gen_string(
            gen,
            b"height\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
            strlen(b"height\0" as *const u8 as *const ::core::ffi::c_char),
        );
        yajl_gen_integer(gen, (*output).rect.height as ::core::ffi::c_longlong);
        yajl_gen_map_close(gen);
        yajl_gen_string(
            gen,
            b"current_workspace\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_uchar,
            strlen(b"current_workspace\0" as *const u8 as *const ::core::ffi::c_char),
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
unsafe extern "C" fn handle_get_marks(
    mut client: *mut ipc_client,
    mut message: *mut uint8_t,
    mut size: ::core::ffi::c_int,
    mut message_size: uint32_t,
    mut message_type: uint32_t,
) {
    let mut gen: yajl_gen = yajl_gen_alloc(::core::ptr::null::<yajl_alloc_funcs>());
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
unsafe extern "C" fn handle_get_version(
    mut client: *mut ipc_client,
    mut message: *mut uint8_t,
    mut size: ::core::ffi::c_int,
    mut message_size: uint32_t,
    mut message_type: uint32_t,
) {
    let mut gen: yajl_gen = yajl_gen_alloc(::core::ptr::null::<yajl_alloc_funcs>());
    yajl_gen_map_open(gen);
    yajl_gen_string(
        gen,
        b"major\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
        strlen(b"major\0" as *const u8 as *const ::core::ffi::c_char),
    );
    yajl_gen_integer(gen, MAJOR_VERSION as ::core::ffi::c_longlong);
    yajl_gen_string(
        gen,
        b"minor\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
        strlen(b"minor\0" as *const u8 as *const ::core::ffi::c_char),
    );
    yajl_gen_integer(gen, MINOR_VERSION as ::core::ffi::c_longlong);
    yajl_gen_string(
        gen,
        b"patch\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
        strlen(b"patch\0" as *const u8 as *const ::core::ffi::c_char),
    );
    yajl_gen_integer(gen, PATCH_VERSION as ::core::ffi::c_longlong);
    yajl_gen_string(
        gen,
        b"human_readable\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
        strlen(b"human_readable\0" as *const u8 as *const ::core::ffi::c_char),
    );
    yajl_gen_string(
        gen,
        i3_version as *mut ::core::ffi::c_uchar,
        strlen(i3_version),
    );
    yajl_gen_string(
        gen,
        b"loaded_config_file_name\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_uchar,
        strlen(b"loaded_config_file_name\0" as *const u8 as *const ::core::ffi::c_char),
    );
    yajl_gen_string(
        gen,
        current_configpath as *mut ::core::ffi::c_uchar,
        strlen(current_configpath),
    );
    yajl_gen_string(
        gen,
        b"included_config_file_names\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_uchar,
        strlen(b"included_config_file_names\0" as *const u8 as *const ::core::ffi::c_char),
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
unsafe extern "C" fn handle_get_bar_config(
    mut client: *mut ipc_client,
    mut message: *mut uint8_t,
    mut size: ::core::ffi::c_int,
    mut message_size: uint32_t,
    mut message_type: uint32_t,
) {
    let mut gen: yajl_gen = yajl_gen_alloc(::core::ptr::null::<yajl_alloc_funcs>());
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
    sasprintf(
        &raw mut bar_id,
        b"%.*s\0" as *const u8 as *const ::core::ffi::c_char,
        message_size,
        message,
    );
    verboselog(
        b"IPC: looking for config for bar ID \"%s\"\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
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
            b"id\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
            strlen(b"id\0" as *const u8 as *const ::core::ffi::c_char),
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
unsafe extern "C" fn handle_get_binding_modes(
    mut client: *mut ipc_client,
    mut message: *mut uint8_t,
    mut size: ::core::ffi::c_int,
    mut message_size: uint32_t,
    mut message_type: uint32_t,
) {
    let mut gen: yajl_gen = yajl_gen_alloc(::core::ptr::null::<yajl_alloc_funcs>());
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
unsafe extern "C" fn add_subscription(
    mut extra: *mut ::core::ffi::c_void,
    mut s: *const ::core::ffi::c_uchar,
    mut len: ylength,
) -> ::core::ffi::c_int {
    let mut client: *mut ipc_client = extra as *mut ipc_client;
    debuglog(
        b"%s:%s:%d - should add subscription to extra %p, sub %.*s\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/ipc.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"add_subscription\0" as *const u8 as *const ::core::ffi::c_char,
        1214 as ::core::ffi::c_int,
        client,
        len as ::core::ffi::c_int,
        s,
    );
    let mut event: ::core::ffi::c_int = (*client).num_events;
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
        b"%s:%s:%d - client is now subscribed to:\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/ipc.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"add_subscription\0" as *const u8 as *const ::core::ffi::c_char,
        1224 as ::core::ffi::c_int,
    );
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*client).num_events {
        debuglog(
            b"%s:%s:%d - event %s\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b"../src/ipc.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"add_subscription\0" as *const u8 as *const ::core::ffi::c_char,
            1226 as ::core::ffi::c_int,
            *(*client).events.offset(i as isize),
        );
        i += 1;
    }
    debuglog(
        b"%s:%s:%d - (done)\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/ipc.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"add_subscription\0" as *const u8 as *const ::core::ffi::c_char,
        1228 as ::core::ffi::c_int,
    );
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn handle_subscribe(
    mut client: *mut ipc_client,
    mut message: *mut uint8_t,
    mut size: ::core::ffi::c_int,
    mut message_size: uint32_t,
    mut message_type: uint32_t,
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
                    as unsafe extern "C" fn(
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
        let mut err: *mut ::core::ffi::c_uchar =
            yajl_get_error(p, true_0, message, message_size as size_t);
        errorlog(
            b"ERROR: YAJL parse error: %s\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            err,
        );
        yajl_free_error(p, err);
        let mut reply: *const ::core::ffi::c_char =
            b"{\"success\":false}\0" as *const u8 as *const ::core::ffi::c_char;
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
    let mut reply_0: *const ::core::ffi::c_char =
        b"{\"success\":true}\0" as *const u8 as *const ::core::ffi::c_char;
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
        if strcmp(
            *(*client).events.offset(i as isize),
            b"tick\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
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
    let mut payload: *const ::core::ffi::c_char =
        b"{\"first\":true,\"payload\":\"\"}\0" as *const u8 as *const ::core::ffi::c_char;
    ipc_send_client_message(
        client,
        strlen(payload),
        I3_IPC_EVENT_TICK as uint32_t,
        payload as *const uint8_t,
    );
}
unsafe extern "C" fn handle_get_config(
    mut client: *mut ipc_client,
    mut message: *mut uint8_t,
    mut size: ::core::ffi::c_int,
    mut message_size: uint32_t,
    mut message_type: uint32_t,
) {
    let mut gen: yajl_gen = yajl_gen_alloc(::core::ptr::null::<yajl_alloc_funcs>());
    yajl_gen_map_open(gen);
    yajl_gen_string(
        gen,
        b"config\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
        strlen(b"config\0" as *const u8 as *const ::core::ffi::c_char),
    );
    let mut file: *mut IncludedFile = included_files.tqh_first as *mut IncludedFile;
    yajl_gen_string(
        gen,
        (*file).raw_contents as *mut ::core::ffi::c_uchar,
        strlen((*file).raw_contents),
    );
    yajl_gen_string(
        gen,
        b"included_configs\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_uchar,
        strlen(b"included_configs\0" as *const u8 as *const ::core::ffi::c_char),
    );
    yajl_gen_array_open(gen);
    file = included_files.tqh_first as *mut IncludedFile;
    while !file.is_null() {
        yajl_gen_map_open(gen);
        yajl_gen_string(
            gen,
            b"path\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
            strlen(b"path\0" as *const u8 as *const ::core::ffi::c_char),
        );
        yajl_gen_string(
            gen,
            (*file).path as *mut ::core::ffi::c_uchar,
            strlen((*file).path),
        );
        yajl_gen_string(
            gen,
            b"raw_contents\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_uchar,
            strlen(b"raw_contents\0" as *const u8 as *const ::core::ffi::c_char),
        );
        yajl_gen_string(
            gen,
            (*file).raw_contents as *mut ::core::ffi::c_uchar,
            strlen((*file).raw_contents),
        );
        yajl_gen_string(
            gen,
            b"variable_replaced_contents\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_uchar,
            strlen(b"variable_replaced_contents\0" as *const u8 as *const ::core::ffi::c_char),
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
unsafe extern "C" fn handle_send_tick(
    mut client: *mut ipc_client,
    mut message: *mut uint8_t,
    mut size: ::core::ffi::c_int,
    mut message_size: uint32_t,
    mut message_type: uint32_t,
) {
    let mut gen: yajl_gen = yajl_gen_alloc(::core::ptr::null::<yajl_alloc_funcs>());
    yajl_gen_map_open(gen);
    yajl_gen_string(
        gen,
        b"first\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
        strlen(b"first\0" as *const u8 as *const ::core::ffi::c_char),
    );
    yajl_gen_bool(gen, false_0);
    yajl_gen_string(
        gen,
        b"payload\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
        strlen(b"payload\0" as *const u8 as *const ::core::ffi::c_char),
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
        b"tick\0" as *const u8 as *const ::core::ffi::c_char,
        I3_IPC_EVENT_TICK as uint32_t,
        payload as *const ::core::ffi::c_char,
    );
    yajl_gen_free(gen);
    let mut reply: *const ::core::ffi::c_char =
        b"{\"success\":true}\0" as *const u8 as *const ::core::ffi::c_char;
    ipc_send_client_message(
        client,
        strlen(reply),
        I3_IPC_REPLY_TYPE_TICK as uint32_t,
        reply as *const uint8_t,
    );
    debuglog(
        b"%s:%s:%d - Sent tick event\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/ipc.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"handle_send_tick\0" as *const u8 as *const ::core::ffi::c_char,
        1342 as ::core::ffi::c_int,
    );
}
unsafe extern "C" fn _sync_json_key(
    mut extra: *mut ::core::ffi::c_void,
    mut val: *const ::core::ffi::c_uchar,
    mut len: size_t,
) -> ::core::ffi::c_int {
    let mut state: *mut sync_state = extra as *mut sync_state;
    free((*state).last_key as *mut ::core::ffi::c_void);
    (*state).last_key = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*state).last_key =
        scalloc(len.wrapping_add(1 as size_t), 1 as size_t) as *mut ::core::ffi::c_char;
    memcpy(
        (*state).last_key as *mut ::core::ffi::c_void,
        val as *const ::core::ffi::c_void,
        len,
    );
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn _sync_json_int(
    mut extra: *mut ::core::ffi::c_void,
    mut val: ::core::ffi::c_longlong,
) -> ::core::ffi::c_int {
    let mut state: *mut sync_state = extra as *mut sync_state;
    if strcasecmp(
        (*state).last_key,
        b"rnd\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        (*state).rnd = val as uint32_t;
    } else if strcasecmp(
        (*state).last_key,
        b"window\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        (*state).window = val as xcb_window_t;
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn handle_sync(
    mut client: *mut ipc_client,
    mut message: *mut uint8_t,
    mut size: ::core::ffi::c_int,
    mut message_size: uint32_t,
    mut message_type: uint32_t,
) {
    static mut callbacks: yajl_callbacks = unsafe {
        yajl_callbacks {
            yajl_null: None,
            yajl_boolean: None,
            yajl_integer: Some(
                _sync_json_int
                    as unsafe extern "C" fn(
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
                    as unsafe extern "C" fn(
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
    let mut p: yajl_handle = yajl_alloc(
        &raw mut callbacks,
        ::core::ptr::null_mut::<yajl_alloc_funcs>(),
        &raw mut state as *mut ::core::ffi::c_void,
    );
    let mut stat: yajl_status = yajl_parse(p, message, message_size as size_t);
    free(state.last_key as *mut ::core::ffi::c_void);
    state.last_key = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if stat as ::core::ffi::c_uint != yajl_status_ok as ::core::ffi::c_int as ::core::ffi::c_uint {
        let mut err: *mut ::core::ffi::c_uchar =
            yajl_get_error(p, true_0, message, message_size as size_t);
        errorlog(
            b"ERROR: YAJL parse error: %s\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            err,
        );
        yajl_free_error(p, err);
        let mut reply: *const ::core::ffi::c_char =
            b"{\"success\":false}\0" as *const u8 as *const ::core::ffi::c_char;
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
        b"%s:%s:%d - received IPC sync request (rnd = %d, window = 0x%08x)\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/ipc.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"handle_sync\0" as *const u8 as *const ::core::ffi::c_char,
        1392 as ::core::ffi::c_int,
        state.rnd,
        state.window,
    );
    sync_respond(state.window, state.rnd);
    let mut reply_0: *const ::core::ffi::c_char =
        b"{\"success\":true}\0" as *const u8 as *const ::core::ffi::c_char;
    ipc_send_client_message(
        client,
        strlen(reply_0),
        I3_IPC_REPLY_TYPE_SYNC as uint32_t,
        reply_0 as *const uint8_t,
    );
}
unsafe extern "C" fn handle_get_binding_state(
    mut client: *mut ipc_client,
    mut message: *mut uint8_t,
    mut size: ::core::ffi::c_int,
    mut message_size: uint32_t,
    mut message_type: uint32_t,
) {
    let mut gen: yajl_gen = yajl_gen_alloc(::core::ptr::null::<yajl_alloc_funcs>());
    yajl_gen_map_open(gen);
    yajl_gen_string(
        gen,
        b"name\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
        strlen(b"name\0" as *const u8 as *const ::core::ffi::c_char),
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
#[no_mangle]
pub static mut handlers: [handler_t; 13] = unsafe {
    [
        Some(
            handle_run_command
                as unsafe extern "C" fn(
                    *mut ipc_client,
                    *mut uint8_t,
                    ::core::ffi::c_int,
                    uint32_t,
                    uint32_t,
                ) -> (),
        ),
        Some(
            handle_get_workspaces
                as unsafe extern "C" fn(
                    *mut ipc_client,
                    *mut uint8_t,
                    ::core::ffi::c_int,
                    uint32_t,
                    uint32_t,
                ) -> (),
        ),
        Some(
            handle_subscribe
                as unsafe extern "C" fn(
                    *mut ipc_client,
                    *mut uint8_t,
                    ::core::ffi::c_int,
                    uint32_t,
                    uint32_t,
                ) -> (),
        ),
        Some(
            handle_get_outputs
                as unsafe extern "C" fn(
                    *mut ipc_client,
                    *mut uint8_t,
                    ::core::ffi::c_int,
                    uint32_t,
                    uint32_t,
                ) -> (),
        ),
        Some(
            handle_tree
                as unsafe extern "C" fn(
                    *mut ipc_client,
                    *mut uint8_t,
                    ::core::ffi::c_int,
                    uint32_t,
                    uint32_t,
                ) -> (),
        ),
        Some(
            handle_get_marks
                as unsafe extern "C" fn(
                    *mut ipc_client,
                    *mut uint8_t,
                    ::core::ffi::c_int,
                    uint32_t,
                    uint32_t,
                ) -> (),
        ),
        Some(
            handle_get_bar_config
                as unsafe extern "C" fn(
                    *mut ipc_client,
                    *mut uint8_t,
                    ::core::ffi::c_int,
                    uint32_t,
                    uint32_t,
                ) -> (),
        ),
        Some(
            handle_get_version
                as unsafe extern "C" fn(
                    *mut ipc_client,
                    *mut uint8_t,
                    ::core::ffi::c_int,
                    uint32_t,
                    uint32_t,
                ) -> (),
        ),
        Some(
            handle_get_binding_modes
                as unsafe extern "C" fn(
                    *mut ipc_client,
                    *mut uint8_t,
                    ::core::ffi::c_int,
                    uint32_t,
                    uint32_t,
                ) -> (),
        ),
        Some(
            handle_get_config
                as unsafe extern "C" fn(
                    *mut ipc_client,
                    *mut uint8_t,
                    ::core::ffi::c_int,
                    uint32_t,
                    uint32_t,
                ) -> (),
        ),
        Some(
            handle_send_tick
                as unsafe extern "C" fn(
                    *mut ipc_client,
                    *mut uint8_t,
                    ::core::ffi::c_int,
                    uint32_t,
                    uint32_t,
                ) -> (),
        ),
        Some(
            handle_sync
                as unsafe extern "C" fn(
                    *mut ipc_client,
                    *mut uint8_t,
                    ::core::ffi::c_int,
                    uint32_t,
                    uint32_t,
                ) -> (),
        ),
        Some(
            handle_get_binding_state
                as unsafe extern "C" fn(
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
    mut loop_0: *mut ev_loop,
    mut w: *mut ev_io,
    mut revents: ::core::ffi::c_int,
) {
    let mut message_type: uint32_t = 0;
    let mut message_length: uint32_t = 0;
    let mut message: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut client: *mut ipc_client = (*w).data as *mut ipc_client;
    if (*client).fd == (*w).fd {
    } else {
        __assert_fail(
            b"client->fd == w->fd\0" as *const u8 as *const ::core::ffi::c_char,
            b"../src/ipc.c\0" as *const u8 as *const ::core::ffi::c_char,
            1449 as ::core::ffi::c_uint,
            b"void ipc_receive_message(struct ev_loop *, ev_io *, int)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    };
    let mut ret: ::core::ffi::c_int = ipc_recv_message(
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
            b"%s:%s:%d - Unhandled message type: %d\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b"../src/ipc.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"ipc_receive_message\0" as *const u8 as *const ::core::ffi::c_char,
            1468 as ::core::ffi::c_int,
            message_type,
        );
    } else {
        let mut h: handler_t = handlers[message_type as usize];
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
    mut loop_0: *mut ev_loop,
    mut w: *mut ev_timer,
    mut revents: ::core::ffi::c_int,
) {
    let mut exepath: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut fd: ::core::ffi::c_int = 0;
    let mut buf: [::core::ffi::c_char; 512] = [0; 512];
    let mut n: ssize_t = 0;
    let mut client: *mut ipc_client = (*w).data as *mut ipc_client;
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
        sasprintf(
            &raw mut exepath,
            b"/proc/%d/cmdline\0" as *const u8 as *const ::core::ffi::c_char,
            peercred.pid,
        );
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
                        b"ERROR: client %p with pid %d and cmdline '%s' on fd %d timed out, killing\n\0"
                            as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
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
            b"ERROR: client %p on fd %d timed out, killing\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            client,
            (*client).fd,
        );
    }
    free_ipc_client(client, -(1 as ::core::ffi::c_int));
}
unsafe extern "C" fn ipc_socket_writeable_cb(
    mut loop_0: *mut ev_loop,
    mut w: *mut ev_io,
    mut revents: ::core::ffi::c_int,
) {
    debuglog(
        b"%s:%s:%d - fd %d writeable\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/ipc.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"ipc_socket_writeable_cb\0" as *const u8 as *const ::core::ffi::c_char,
        1524 as ::core::ffi::c_int,
        (*w).fd,
    );
    let mut client: *mut ipc_client = (*w).data as *mut ipc_client;
    if (*client).timeout != ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut ev_timer {
    } else {
        __assert_fail(
            b"client->timeout != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            b"../src/ipc.c\0" as *const u8 as *const ::core::ffi::c_char,
            1529 as ::core::ffi::c_uint,
            b"void ipc_socket_writeable_cb(struct ev_loop *, ev_io *, int)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    };
    ipc_push_pending(client);
}
#[no_mangle]
pub unsafe extern "C" fn ipc_new_client(
    mut loop_0: *mut ev_loop,
    mut w: *mut ev_io,
    mut revents: ::core::ffi::c_int,
) {
    let mut peer: sockaddr_un = sockaddr_un {
        sun_family: 0,
        sun_path: [0; 108],
    };
    let mut len: socklen_t = ::core::mem::size_of::<sockaddr_un>() as socklen_t;
    let mut fd: ::core::ffi::c_int = 0;
    fd = accept(
        (*w).fd,
        __SOCKADDR_ARG {
            __sockaddr__: &raw mut peer as *mut sockaddr,
        },
        &raw mut len,
    );
    if fd < 0 as ::core::ffi::c_int {
        if *__errno_location() != EINTR {
            perror(b"accept()\0" as *const u8 as *const ::core::ffi::c_char);
        }
        return;
    }
    fcntl(fd, F_SETFD, FD_CLOEXEC);
    ipc_new_client_on_fd(loop_0, fd);
}
#[no_mangle]
pub unsafe extern "C" fn ipc_new_client_on_fd(
    mut loop_0: *mut ev_loop,
    mut fd: ::core::ffi::c_int,
) -> *mut ipc_client {
    set_nonblock(fd);
    let mut client: *mut ipc_client =
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
        b"%s:%s:%d - IPC: new client connected on fd %d\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/ipc.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"ipc_new_client_on_fd\0" as *const u8 as *const ::core::ffi::c_char,
        1580 as ::core::ffi::c_int,
        fd,
    );
    (*client).clients.tqe_next = ::core::ptr::null_mut::<ipc_client>();
    (*client).clients.tqe_prev = all_clients.tqh_last;
    *all_clients.tqh_last = client as *mut ipc_client;
    all_clients.tqh_last = &raw mut (*client).clients.tqe_next;
    return client;
}
#[no_mangle]
pub unsafe extern "C" fn ipc_marshal_workspace_event(
    mut change: *const ::core::ffi::c_char,
    mut current: *mut Con,
    mut old: *mut Con,
) -> yajl_gen {
    let mut prev_locale: *mut ::core::ffi::c_char = sstrdup(setlocale(
        LC_NUMERIC,
        ::core::ptr::null::<::core::ffi::c_char>(),
    ));
    setlocale(
        LC_NUMERIC,
        b"C\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut gen: yajl_gen = yajl_gen_alloc(::core::ptr::null::<yajl_alloc_funcs>());
    yajl_gen_map_open(gen);
    yajl_gen_string(
        gen,
        b"change\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
        strlen(b"change\0" as *const u8 as *const ::core::ffi::c_char),
    );
    yajl_gen_string(gen, change as *mut ::core::ffi::c_uchar, strlen(change));
    yajl_gen_string(
        gen,
        b"current\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
        strlen(b"current\0" as *const u8 as *const ::core::ffi::c_char),
    );
    if current.is_null() {
        yajl_gen_null(gen);
    } else {
        dump_node(gen, current, false_0 != 0);
    }
    yajl_gen_string(
        gen,
        b"old\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
        strlen(b"old\0" as *const u8 as *const ::core::ffi::c_char),
    );
    if old.is_null() {
        yajl_gen_null(gen);
    } else {
        dump_node(gen, old, false_0 != 0);
    }
    yajl_gen_map_close(gen);
    setlocale(LC_NUMERIC, prev_locale);
    free(prev_locale as *mut ::core::ffi::c_void);
    return gen;
}
#[no_mangle]
pub unsafe extern "C" fn ipc_send_workspace_event(
    mut change: *const ::core::ffi::c_char,
    mut current: *mut Con,
    mut old: *mut Con,
) {
    let mut gen: yajl_gen = ipc_marshal_workspace_event(change, current, old);
    let mut payload: *const ::core::ffi::c_uchar = ::core::ptr::null::<::core::ffi::c_uchar>();
    let mut length: ylength = 0;
    yajl_gen_get_buf(gen, &raw mut payload, &raw mut length);
    ipc_send_event(
        b"workspace\0" as *const u8 as *const ::core::ffi::c_char,
        I3_IPC_EVENT_WORKSPACE as uint32_t,
        payload as *const ::core::ffi::c_char,
    );
    yajl_gen_free(gen);
}
#[no_mangle]
pub unsafe extern "C" fn ipc_send_window_event(
    mut property: *const ::core::ffi::c_char,
    mut con: *mut Con,
) {
    debuglog(
        b"%s:%s:%d - Issue IPC window %s event (con = %p, window = 0x%08x)\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/ipc.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"ipc_send_window_event\0" as *const u8 as *const ::core::ffi::c_char,
        1644 as ::core::ffi::c_int,
        property,
        con,
        if !(*con).window.is_null() {
            (*(*con).window).id
        } else {
            XCB_WINDOW_NONE as ::core::ffi::c_int as xcb_window_t
        },
    );
    let mut prev_locale: *mut ::core::ffi::c_char = sstrdup(setlocale(
        LC_NUMERIC,
        ::core::ptr::null::<::core::ffi::c_char>(),
    ));
    setlocale(
        LC_NUMERIC,
        b"C\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut gen: yajl_gen = yajl_gen_alloc(::core::ptr::null::<yajl_alloc_funcs>());
    yajl_gen_map_open(gen);
    yajl_gen_string(
        gen,
        b"change\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
        strlen(b"change\0" as *const u8 as *const ::core::ffi::c_char),
    );
    yajl_gen_string(gen, property as *mut ::core::ffi::c_uchar, strlen(property));
    yajl_gen_string(
        gen,
        b"container\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
        strlen(b"container\0" as *const u8 as *const ::core::ffi::c_char),
    );
    dump_node(gen, con, false_0 != 0);
    yajl_gen_map_close(gen);
    let mut payload: *const ::core::ffi::c_uchar = ::core::ptr::null::<::core::ffi::c_uchar>();
    let mut length: ylength = 0;
    yajl_gen_get_buf(gen, &raw mut payload, &raw mut length);
    ipc_send_event(
        b"window\0" as *const u8 as *const ::core::ffi::c_char,
        I3_IPC_EVENT_WINDOW as uint32_t,
        payload as *const ::core::ffi::c_char,
    );
    yajl_gen_free(gen);
    setlocale(LC_NUMERIC, prev_locale);
    free(prev_locale as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn ipc_send_barconfig_update_event(mut barconfig: *mut Barconfig) {
    debuglog(
        b"%s:%s:%d - Issue barconfig_update event for id = %s\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/ipc.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"ipc_send_barconfig_update_event\0" as *const u8 as *const ::core::ffi::c_char,
        1674 as ::core::ffi::c_int,
        (*barconfig).id,
    );
    let mut prev_locale: *mut ::core::ffi::c_char = sstrdup(setlocale(
        LC_NUMERIC,
        ::core::ptr::null::<::core::ffi::c_char>(),
    ));
    setlocale(
        LC_NUMERIC,
        b"C\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut gen: yajl_gen = yajl_gen_alloc(::core::ptr::null::<yajl_alloc_funcs>());
    dump_bar_config(gen, barconfig);
    let mut payload: *const ::core::ffi::c_uchar = ::core::ptr::null::<::core::ffi::c_uchar>();
    let mut length: ylength = 0;
    yajl_gen_get_buf(gen, &raw mut payload, &raw mut length);
    ipc_send_event(
        b"barconfig_update\0" as *const u8 as *const ::core::ffi::c_char,
        I3_IPC_EVENT_BARCONFIG_UPDATE as uint32_t,
        payload as *const ::core::ffi::c_char,
    );
    yajl_gen_free(gen);
    setlocale(LC_NUMERIC, prev_locale);
    free(prev_locale as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn ipc_send_binding_event(
    mut event_type: *const ::core::ffi::c_char,
    mut bind: *mut Binding,
    mut modename: *const ::core::ffi::c_char,
) {
    debuglog(
        b"%s:%s:%d - Issue IPC binding %s event (sym = %s, code = %d)\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/ipc.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"ipc_send_binding_event\0" as *const u8 as *const ::core::ffi::c_char,
        1695 as ::core::ffi::c_int,
        event_type,
        (*bind).symbol,
        (*bind).keycode,
    );
    let mut prev_locale: *mut ::core::ffi::c_char = sstrdup(setlocale(
        LC_NUMERIC,
        ::core::ptr::null::<::core::ffi::c_char>(),
    ));
    setlocale(
        LC_NUMERIC,
        b"C\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut gen: yajl_gen = yajl_gen_alloc(::core::ptr::null::<yajl_alloc_funcs>());
    yajl_gen_map_open(gen);
    yajl_gen_string(
        gen,
        b"change\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
        strlen(b"change\0" as *const u8 as *const ::core::ffi::c_char),
    );
    yajl_gen_string(
        gen,
        event_type as *mut ::core::ffi::c_uchar,
        strlen(event_type),
    );
    yajl_gen_string(
        gen,
        b"mode\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
        strlen(b"mode\0" as *const u8 as *const ::core::ffi::c_char),
    );
    if modename.is_null() {
        yajl_gen_string(
            gen,
            b"default\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
            strlen(b"default\0" as *const u8 as *const ::core::ffi::c_char),
        );
    } else {
        yajl_gen_string(gen, modename as *mut ::core::ffi::c_uchar, strlen(modename));
    }
    yajl_gen_string(
        gen,
        b"binding\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
        strlen(b"binding\0" as *const u8 as *const ::core::ffi::c_char),
    );
    dump_binding(gen, bind);
    yajl_gen_map_close(gen);
    let mut payload: *const ::core::ffi::c_uchar = ::core::ptr::null::<::core::ffi::c_uchar>();
    let mut length: ylength = 0;
    yajl_gen_get_buf(gen, &raw mut payload, &raw mut length);
    ipc_send_event(
        b"binding\0" as *const u8 as *const ::core::ffi::c_char,
        I3_IPC_EVENT_BINDING as uint32_t,
        payload as *const ::core::ffi::c_char,
    );
    yajl_gen_free(gen);
    setlocale(LC_NUMERIC, prev_locale);
    free(prev_locale as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn ipc_confirm_restart(mut client: *mut ipc_client) {
    debuglog(
        b"%s:%s:%d - ipc_confirm_restart(fd %d)\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/ipc.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"ipc_confirm_restart\0" as *const u8 as *const ::core::ffi::c_char,
        1734 as ::core::ffi::c_int,
        (*client).fd,
    );
    static mut reply: *const ::core::ffi::c_char =
        b"[{\"success\":true}]\0" as *const u8 as *const ::core::ffi::c_char;
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
