/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 */
// Mechanical file-by-file Rust translation of src/sighandler.c.
// Generated with C2Rust 0.22.1; C ABI and original ownership retained.
extern "C" {
    pub type xcb_connection_t;
    pub type _XCBKeySymbols;
    pub type _PangoFontDescription;
    pub type _cairo;
    pub type _cairo_surface;
    pub type _i3String;
    pub type pcre2_real_code_8;
    pub type ev_loop;
    fn err(__status: ::core::ffi::c_int, __format: *const ::core::ffi::c_char, ...) -> !;
    fn xcb_destroy_window(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t;
    fn xcb_grab_pointer(
        c: *mut xcb_connection_t,
        owner_events: uint8_t,
        grab_window: xcb_window_t,
        event_mask: uint16_t,
        pointer_mode: uint8_t,
        keyboard_mode: uint8_t,
        confine_to: xcb_window_t,
        cursor: xcb_cursor_t,
        time: xcb_timestamp_t,
    ) -> xcb_grab_pointer_cookie_t;
    fn xcb_grab_keyboard(
        c: *mut xcb_connection_t,
        owner_events: uint8_t,
        grab_window: xcb_window_t,
        time: xcb_timestamp_t,
        pointer_mode: uint8_t,
        keyboard_mode: uint8_t,
    ) -> xcb_grab_keyboard_cookie_t;
    fn xcb_create_colormap(
        c: *mut xcb_connection_t,
        alloc: uint8_t,
        mid: xcb_colormap_t,
        window: xcb_window_t,
        visual: xcb_visualid_t,
    ) -> xcb_void_cookie_t;
    fn xcb_free_colormap(c: *mut xcb_connection_t, cmap: xcb_colormap_t) -> xcb_void_cookie_t;
    fn xcb_flush(c: *mut xcb_connection_t) -> ::core::ffi::c_int;
    fn xcb_wait_for_event(c: *mut xcb_connection_t) -> *mut xcb_generic_event_t;
    fn xcb_generate_id(c: *mut xcb_connection_t) -> uint32_t;
    fn xcb_key_press_lookup_keysym(
        syms: *mut xcb_key_symbols_t,
        event: *mut xcb_key_press_event_t,
        col: ::core::ffi::c_int,
    ) -> xcb_keysym_t;
    fn sigemptyset(__set: *mut sigset_t) -> ::core::ffi::c_int;
    fn sigaction(
        __sig: ::core::ffi::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> ::core::ffi::c_int;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn pipe(__pipedes: *mut ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn dup2(__fd: ::core::ffi::c_int, __fd2: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn execvp(
        __file: *const ::core::ffi::c_char,
        __argv: *const *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn getpid() -> __pid_t;
    fn fork() -> __pid_t;
    static mut conn: *mut xcb_connection_t;
    static mut root_screen: *mut xcb_screen_t;
    fn errorlog(fmt: *mut ::core::ffi::c_char, ...);
    fn debuglog(fmt: *mut ::core::ffi::c_char, ...);
    fn scalloc(num: size_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn sasprintf(
        strp: *mut *mut ::core::ffi::c_char,
        fmt: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn i3string_from_utf8(from_utf8: *const ::core::ffi::c_char) -> *mut i3String;
    fn predict_text_width(text: *mut i3String) -> ::core::ffi::c_int;
    fn logical_px(logical: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn draw_util_surface_init(
        conn_0: *mut xcb_connection_t,
        surface: *mut surface_t,
        drawable: xcb_drawable_t,
        visual: *mut xcb_visualtype_t,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
    );
    fn draw_util_surface_free(conn_0: *mut xcb_connection_t, surface: *mut surface_t);
    fn draw_util_hex_to_color(color: *const ::core::ffi::c_char) -> color_t;
    fn draw_util_text(
        text: *mut i3String,
        surface: *mut surface_t,
        fg_color: color_t,
        bg_color: color_t,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
        max_width: ::core::ffi::c_int,
    );
    fn draw_util_rectangle(
        surface: *mut surface_t,
        color: color_t,
        x: ::core::ffi::c_double,
        y: ::core::ffi::c_double,
        w: ::core::ffi::c_double,
        h: ::core::ffi::c_double,
    );
    fn draw_util_clear_surface(surface: *mut surface_t, color: color_t);
    fn path_exists(path: *const ::core::ffi::c_char) -> bool;
    fn i3_restart(forget_layout: bool);
    static mut keysyms: *mut xcb_key_symbols_t;
    static mut start_argv: *mut *mut ::core::ffi::c_char;
    static mut root_depth: uint8_t;
    static mut root: xcb_window_t;
    static mut config: Config;
    static mut xcb_numlock_mask: ::core::ffi::c_uint;
    fn create_window(
        conn_0: *mut xcb_connection_t,
        r: Rect,
        depth: uint16_t,
        visual: xcb_visualid_t,
        window_class: uint16_t,
        cursor: xcursor_cursor_t,
        map: bool,
        mask: uint32_t,
        values: *mut uint32_t,
    ) -> xcb_window_t;
    fn get_visualtype_by_id(visual_id: xcb_visualid_t) -> *mut xcb_visualtype_t;
    fn get_visualid_by_depth(depth: uint16_t) -> xcb_visualid_t;
    static mut outputs: outputs_head;
    fn waitpid(
        __pid: __pid_t,
        __stat_loc: *mut ::core::ffi::c_int,
        __options: ::core::ffi::c_int,
    ) -> __pid_t;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn getenv(__name: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __uid_t = ::core::ffi::c_uint;
pub type __pid_t = ::core::ffi::c_int;
pub type __clock_t = ::core::ffi::c_long;
pub type __time_t = ::core::ffi::c_long;
pub type __suseconds_t = ::core::ffi::c_long;
pub type pid_t = __pid_t;
pub type time_t = __time_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [::core::ffi::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: ::core::ffi::c_int,
    pub sival_ptr: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_generic_event_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub pad: [uint32_t; 7],
    pub full_sequence: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_void_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
pub type xcb_window_t = uint32_t;
pub type xcb_cursor_t = uint32_t;
pub type xcb_font_t = uint32_t;
pub type xcb_gcontext_t = uint32_t;
pub type xcb_colormap_t = uint32_t;
pub type xcb_atom_t = uint32_t;
pub type xcb_drawable_t = uint32_t;
pub type xcb_visualid_t = uint32_t;
pub type xcb_timestamp_t = uint32_t;
pub type xcb_keysym_t = uint32_t;
pub type xcb_keycode_t = uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_visualtype_t {
    pub visual_id: xcb_visualid_t,
    pub _class: uint8_t,
    pub bits_per_rgb_value: uint8_t,
    pub colormap_entries: uint16_t,
    pub red_mask: uint32_t,
    pub green_mask: uint32_t,
    pub blue_mask: uint32_t,
    pub pad0: [uint8_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_screen_t {
    pub root: xcb_window_t,
    pub default_colormap: xcb_colormap_t,
    pub white_pixel: uint32_t,
    pub black_pixel: uint32_t,
    pub current_input_masks: uint32_t,
    pub width_in_pixels: uint16_t,
    pub height_in_pixels: uint16_t,
    pub width_in_millimeters: uint16_t,
    pub height_in_millimeters: uint16_t,
    pub min_installed_maps: uint16_t,
    pub max_installed_maps: uint16_t,
    pub root_visual: xcb_visualid_t,
    pub backing_stores: uint8_t,
    pub save_unders: uint8_t,
    pub root_depth: uint8_t,
    pub allowed_depths_len: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_key_press_event_t {
    pub response_type: uint8_t,
    pub detail: xcb_keycode_t,
    pub sequence: uint16_t,
    pub time: xcb_timestamp_t,
    pub root: xcb_window_t,
    pub event: xcb_window_t,
    pub child: xcb_window_t,
    pub root_x: int16_t,
    pub root_y: int16_t,
    pub event_x: int16_t,
    pub event_y: int16_t,
    pub state: uint16_t,
    pub same_screen: uint8_t,
    pub pad0: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_expose_event_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub window: xcb_window_t,
    pub x: uint16_t,
    pub y: uint16_t,
    pub width: uint16_t,
    pub height: uint16_t,
    pub count: uint16_t,
    pub pad1: [uint8_t; 2],
}
pub type xcb_window_class_t = ::core::ffi::c_uint;
pub const XCB_WINDOW_CLASS_INPUT_ONLY: xcb_window_class_t = 2;
pub const XCB_WINDOW_CLASS_INPUT_OUTPUT: xcb_window_class_t = 1;
pub const XCB_WINDOW_CLASS_COPY_FROM_PARENT: xcb_window_class_t = 0;
pub type xcb_cw_t = ::core::ffi::c_uint;
pub const XCB_CW_CURSOR: xcb_cw_t = 16384;
pub const XCB_CW_COLORMAP: xcb_cw_t = 8192;
pub const XCB_CW_DONT_PROPAGATE: xcb_cw_t = 4096;
pub const XCB_CW_EVENT_MASK: xcb_cw_t = 2048;
pub const XCB_CW_SAVE_UNDER: xcb_cw_t = 1024;
pub const XCB_CW_OVERRIDE_REDIRECT: xcb_cw_t = 512;
pub const XCB_CW_BACKING_PIXEL: xcb_cw_t = 256;
pub const XCB_CW_BACKING_PLANES: xcb_cw_t = 128;
pub const XCB_CW_BACKING_STORE: xcb_cw_t = 64;
pub const XCB_CW_WIN_GRAVITY: xcb_cw_t = 32;
pub const XCB_CW_BIT_GRAVITY: xcb_cw_t = 16;
pub const XCB_CW_BORDER_PIXEL: xcb_cw_t = 8;
pub const XCB_CW_BORDER_PIXMAP: xcb_cw_t = 4;
pub const XCB_CW_BACK_PIXEL: xcb_cw_t = 2;
pub const XCB_CW_BACK_PIXMAP: xcb_cw_t = 1;
pub type xcb_grab_mode_t = ::core::ffi::c_uint;
pub const XCB_GRAB_MODE_ASYNC: xcb_grab_mode_t = 1;
pub const XCB_GRAB_MODE_SYNC: xcb_grab_mode_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_grab_pointer_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_grab_keyboard_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_charinfo_t {
    pub left_side_bearing: int16_t,
    pub right_side_bearing: int16_t,
    pub character_width: int16_t,
    pub ascent: int16_t,
    pub descent: int16_t,
    pub attributes: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_query_font_reply_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub min_bounds: xcb_charinfo_t,
    pub pad1: [uint8_t; 4],
    pub max_bounds: xcb_charinfo_t,
    pub pad2: [uint8_t; 4],
    pub min_char_or_byte2: uint16_t,
    pub max_char_or_byte2: uint16_t,
    pub default_char: uint16_t,
    pub properties_len: uint16_t,
    pub draw_direction: uint8_t,
    pub min_byte1: uint8_t,
    pub max_byte1: uint8_t,
    pub all_chars_exist: uint8_t,
    pub font_ascent: int16_t,
    pub font_descent: int16_t,
    pub char_infos_len: uint32_t,
}
pub type xcb_colormap_alloc_t = ::core::ffi::c_uint;
pub const XCB_COLORMAP_ALLOC_ALL: xcb_colormap_alloc_t = 1;
pub const XCB_COLORMAP_ALLOC_NONE: xcb_colormap_alloc_t = 0;
pub type xcb_key_symbols_t = _XCBKeySymbols;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct siginfo_t {
    pub si_signo: ::core::ffi::c_int,
    pub si_errno: ::core::ffi::c_int,
    pub si_code: ::core::ffi::c_int,
    pub __pad0: ::core::ffi::c_int,
    pub _sifields: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub _pad: [::core::ffi::c_int; 28],
    pub _kill: C2RustUnnamed_8,
    pub _timer: C2RustUnnamed_7,
    pub _rt: C2RustUnnamed_6,
    pub _sigchld: C2RustUnnamed_5,
    pub _sigfault: C2RustUnnamed_2,
    pub _sigpoll: C2RustUnnamed_1,
    pub _sigsys: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub _call_addr: *mut ::core::ffi::c_void,
    pub _syscall: ::core::ffi::c_int,
    pub _arch: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub si_band: ::core::ffi::c_long,
    pub si_fd: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub si_addr: *mut ::core::ffi::c_void,
    pub si_addr_lsb: ::core::ffi::c_short,
    pub _bounds: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub _addr_bnd: C2RustUnnamed_4,
    pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub _lower: *mut ::core::ffi::c_void,
    pub _upper: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: ::core::ffi::c_int,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub si_tid: ::core::ffi::c_int,
    pub si_overrun: ::core::ffi::c_int,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
pub type __sighandler_t = Option<unsafe extern "C" fn(::core::ffi::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: C2RustUnnamed_9,
    pub sa_mask: __sigset_t,
    pub sa_flags: ::core::ffi::c_int,
    pub sa_restorer: Option<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction: Option<
        unsafe extern "C" fn(::core::ffi::c_int, *mut siginfo_t, *mut ::core::ffi::c_void) -> (),
    >,
}
pub type PangoFontDescription = _PangoFontDescription;
pub type cairo_t = _cairo;
pub type cairo_surface_t = _cairo_surface;
pub type i3String = _i3String;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Font {
    pub type_0: C2RustUnnamed_12,
    pub height: ::core::ffi::c_int,
    pub pattern: *mut ::core::ffi::c_char,
    pub specific: C2RustUnnamed_10,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
    pub xcb: C2RustUnnamed_11,
    pub pango_desc: *mut PangoFontDescription,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub id: xcb_font_t,
    pub info: *mut xcb_query_font_reply_t,
    pub table: *mut xcb_charinfo_t,
}
pub type C2RustUnnamed_12 = ::core::ffi::c_uint;
pub const FONT_TYPE_PANGO: C2RustUnnamed_12 = 2;
pub const FONT_TYPE_XCB: C2RustUnnamed_12 = 1;
pub const FONT_TYPE_NONE: C2RustUnnamed_12 = 0;
pub type i3Font = Font;
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
    pub outputs: C2RustUnnamed_13,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
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
    pub type_0: C2RustUnnamed_30,
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
    pub floating: C2RustUnnamed_19,
    pub nodes: C2RustUnnamed_18,
    pub focused: C2RustUnnamed_17,
    pub all_cons: C2RustUnnamed_16,
    pub floating_windows: C2RustUnnamed_15,
    pub on_remove_child: Option<unsafe extern "C" fn(*mut Con) -> ()>,
    pub scratchpad_state: C2RustUnnamed_14,
    pub old_id: ::core::ffi::c_int,
    pub depth: uint16_t,
    pub colormap: xcb_colormap_t,
}
pub type C2RustUnnamed_14 = ::core::ffi::c_uint;
pub const SCRATCHPAD_CHANGED: C2RustUnnamed_14 = 2;
pub const SCRATCHPAD_FRESH: C2RustUnnamed_14 = 1;
pub const SCRATCHPAD_NONE: C2RustUnnamed_14 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_15 {
    pub tqe_next: *mut Con,
    pub tqe_prev: *mut *mut Con,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_16 {
    pub tqe_next: *mut Con,
    pub tqe_prev: *mut *mut Con,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_17 {
    pub tqe_next: *mut Con,
    pub tqe_prev: *mut *mut Con,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_18 {
    pub tqe_next: *mut Con,
    pub tqe_prev: *mut *mut Con,
}
pub type C2RustUnnamed_19 = ::core::ffi::c_uint;
pub const FLOATING_USER_ON: C2RustUnnamed_19 = 3;
pub const FLOATING_AUTO_ON: C2RustUnnamed_19 = 2;
pub const FLOATING_USER_OFF: C2RustUnnamed_19 = 1;
pub const FLOATING_AUTO_OFF: C2RustUnnamed_19 = 0;
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
    pub urgent: C2RustUnnamed_24,
    pub dock: C2RustUnnamed_23,
    pub id: xcb_window_t,
    pub window_mode: C2RustUnnamed_22,
    pub con_id: *mut Con,
    pub match_all_windows: bool,
    pub insert_where: C2RustUnnamed_21,
    pub matches: C2RustUnnamed_20,
    pub restart_mode: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_20 {
    pub tqe_next: *mut Match,
    pub tqe_prev: *mut *mut Match,
}
pub type C2RustUnnamed_21 = ::core::ffi::c_uint;
pub const M_BELOW: C2RustUnnamed_21 = 2;
pub const M_ASSIGN_WS: C2RustUnnamed_21 = 1;
pub const M_HERE: C2RustUnnamed_21 = 0;
pub type C2RustUnnamed_22 = ::core::ffi::c_uint;
pub const WM_FLOATING: C2RustUnnamed_22 = 6;
pub const WM_FLOATING_USER: C2RustUnnamed_22 = 5;
pub const WM_FLOATING_AUTO: C2RustUnnamed_22 = 4;
pub const WM_TILING: C2RustUnnamed_22 = 3;
pub const WM_TILING_USER: C2RustUnnamed_22 = 2;
pub const WM_TILING_AUTO: C2RustUnnamed_22 = 1;
pub const WM_ANY: C2RustUnnamed_22 = 0;
pub type C2RustUnnamed_23 = ::core::ffi::c_int;
pub const M_DOCK_BOTTOM: C2RustUnnamed_23 = 3;
pub const M_DOCK_TOP: C2RustUnnamed_23 = 2;
pub const M_DOCK_ANY: C2RustUnnamed_23 = 1;
pub const M_NODOCK: C2RustUnnamed_23 = 0;
pub const M_DONTCHECK: C2RustUnnamed_23 = -1;
pub type C2RustUnnamed_24 = ::core::ffi::c_int;
pub const U_OLDEST: C2RustUnnamed_24 = 1;
pub const U_LATEST: C2RustUnnamed_24 = 0;
pub const U_DONTCHECK: C2RustUnnamed_24 = -1;
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
    pub dock: C2RustUnnamed_25,
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
pub type C2RustUnnamed_25 = ::core::ffi::c_uint;
pub const W_DOCK_BOTTOM: C2RustUnnamed_25 = 2;
pub const W_DOCK_TOP: C2RustUnnamed_25 = 1;
pub const W_NODOCK: C2RustUnnamed_25 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Assignment {
    pub type_0: C2RustUnnamed_28,
    pub match_0: Match,
    pub dest: C2RustUnnamed_27,
    pub assignments: C2RustUnnamed_26,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_26 {
    pub tqe_next: *mut Assignment,
    pub tqe_prev: *mut *mut Assignment,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_27 {
    pub command: *mut ::core::ffi::c_char,
    pub workspace: *mut ::core::ffi::c_char,
    pub output: *mut ::core::ffi::c_char,
}
pub type C2RustUnnamed_28 = ::core::ffi::c_uint;
pub const A_TO_OUTPUT: C2RustUnnamed_28 = 16;
pub const A_TO_WORKSPACE_NUMBER: C2RustUnnamed_28 = 8;
pub const A_NO_FOCUS: C2RustUnnamed_28 = 4;
pub const A_TO_WORKSPACE: C2RustUnnamed_28 = 2;
pub const A_COMMAND: C2RustUnnamed_28 = 1;
pub const A_ANY: C2RustUnnamed_28 = 0;
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
    pub marks: C2RustUnnamed_29,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_29 {
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
pub type C2RustUnnamed_30 = ::core::ffi::c_uint;
pub const CT_DOCKAREA: C2RustUnnamed_30 = 5;
pub const CT_WORKSPACE: C2RustUnnamed_30 = 4;
pub const CT_FLOATING_CON: C2RustUnnamed_30 = 3;
pub const CT_CON: C2RustUnnamed_30 = 2;
pub const CT_OUTPUT: C2RustUnnamed_30 = 1;
pub const CT_ROOT: C2RustUnnamed_30 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct names_head {
    pub slh_first: *mut output_name,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct output_name {
    pub name: *mut ::core::ffi::c_char,
    pub names: C2RustUnnamed_31,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_31 {
    pub sle_next: *mut output_name,
}
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
pub type focus_wrapping_t = ::core::ffi::c_uint;
pub const FOCUS_WRAPPING_WORKSPACE: focus_wrapping_t = 3;
pub const FOCUS_WRAPPING_FORCE: focus_wrapping_t = 2;
pub const FOCUS_WRAPPING_ON: focus_wrapping_t = 1;
pub const FOCUS_WRAPPING_OFF: focus_wrapping_t = 0;
pub type tiling_drag_t = ::core::ffi::c_uint;
pub const TILING_DRAG_MODIFIER_OR_TITLEBAR: tiling_drag_t = 3;
pub const TILING_DRAG_TITLEBAR: tiling_drag_t = 2;
pub const TILING_DRAG_MODIFIER: tiling_drag_t = 1;
pub const TILING_DRAG_OFF: tiling_drag_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Config {
    pub terminal: *const ::core::ffi::c_char,
    pub font: i3Font,
    pub ipc_socket_path: *mut ::core::ffi::c_char,
    pub restart_state_path: *mut ::core::ffi::c_char,
    pub default_layout: layout_t,
    pub container_stack_limit: ::core::ffi::c_int,
    pub container_stack_limit_value: ::core::ffi::c_int,
    pub default_border_width: ::core::ffi::c_int,
    pub default_floating_border_width: ::core::ffi::c_int,
    pub default_orientation: ::core::ffi::c_int,
    pub disable_focus_follows_mouse: bool,
    pub mouse_warping: warping_t,
    pub hide_edge_borders: hide_edge_borders_mode_t,
    pub disable_workspace_bar: bool,
    pub focus_wrapping: focus_wrapping_t,
    pub force_xinerama: bool,
    pub disable_randr15: bool,
    pub fake_outputs: *mut ::core::ffi::c_char,
    pub workspace_auto_back_and_forth: bool,
    pub workspace_urgency_timer: ::core::ffi::c_float,
    pub focus_on_window_activation: C2RustUnnamed_34,
    pub show_marks: bool,
    pub title_align: C2RustUnnamed_33,
    pub default_border: border_style_t,
    pub default_floating_border: border_style_t,
    pub floating_modifier: uint32_t,
    pub swap_modifier: uint32_t,
    pub floating_maximum_width: int32_t,
    pub floating_maximum_height: int32_t,
    pub floating_minimum_width: int32_t,
    pub floating_minimum_height: int32_t,
    pub client: config_client,
    pub bar: config_bar,
    pub popup_during_fullscreen: C2RustUnnamed_32,
    pub number_barconfigs: ::core::ffi::c_int,
    pub tiling_drag: tiling_drag_t,
    pub gaps: gaps_t,
    pub smart_gaps: smart_gaps_t,
}
pub type C2RustUnnamed_32 = ::core::ffi::c_uint;
pub const PDF_ALL: C2RustUnnamed_32 = 3;
pub const PDF_IGNORE: C2RustUnnamed_32 = 2;
pub const PDF_LEAVE_FULLSCREEN: C2RustUnnamed_32 = 1;
pub const PDF_SMART: C2RustUnnamed_32 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct config_bar {
    pub focused: Colortriple,
    pub unfocused: Colortriple,
    pub urgent: Colortriple,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct config_client {
    pub background: color_t,
    pub focused: Colortriple,
    pub focused_inactive: Colortriple,
    pub focused_tab_title: Colortriple,
    pub unfocused: Colortriple,
    pub urgent: Colortriple,
    pub placeholder: Colortriple,
    pub got_focused_tab_title: bool,
}
pub type C2RustUnnamed_33 = ::core::ffi::c_uint;
pub const ALIGN_RIGHT: C2RustUnnamed_33 = 2;
pub const ALIGN_CENTER: C2RustUnnamed_33 = 1;
pub const ALIGN_LEFT: C2RustUnnamed_33 = 0;
pub type C2RustUnnamed_34 = ::core::ffi::c_uint;
pub const FOWA_NONE: C2RustUnnamed_34 = 3;
pub const FOWA_FOCUS: C2RustUnnamed_34 = 2;
pub const FOWA_URGENT: C2RustUnnamed_34 = 1;
pub const FOWA_SMART: C2RustUnnamed_34 = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dialog_t {
    pub id: xcb_window_t,
    pub colormap: xcb_colormap_t,
    pub dims: Rect,
    pub surface: surface_t,
    pub dialogs: C2RustUnnamed_35,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_35 {
    pub tqe_next: *mut dialog_t,
    pub tqe_prev: *mut *mut dialog_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dialogs_head {
    pub tqh_first: *mut dialog_t,
    pub tqh_last: *mut *mut dialog_t,
}
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const XCB_KEY_PRESS: ::core::ffi::c_int = 2;
pub const XCB_EXPOSE: ::core::ffi::c_int = 12;
pub const XCB_NONE: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const XCB_CURRENT_TIME: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const SIG_DFL: __sighandler_t = None;
pub const SIGILL: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SIGABRT: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const SIGFPE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const SIGSEGV: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const SIGQUIT: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const SA_SIGINFO: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SA_NODEFER: ::core::ffi::c_int = 0x40000000 as ::core::ffi::c_int;
pub const SA_RESETHAND: ::core::ffi::c_uint = 0x80000000 as ::core::ffi::c_uint;
pub const STDIN_FILENO: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const STDOUT_FILENO: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const STDERR_FILENO: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
static mut dialogs: dialogs_head = dialogs_head {
    tqh_first: ::core::ptr::null::<dialog_t>() as *mut dialog_t,
    tqh_last: ::core::ptr::null::<*mut dialog_t>() as *mut *mut dialog_t,
};
static mut raised_signal: ::core::ffi::c_int = 0;
static mut backtrace_done: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut message_intro: *mut i3String = ::core::ptr::null::<i3String>() as *mut i3String;
static mut message_intro2: *mut i3String = ::core::ptr::null::<i3String>() as *mut i3String;
static mut message_option_backtrace: *mut i3String =
    ::core::ptr::null::<i3String>() as *mut i3String;
static mut message_option_restart: *mut i3String = ::core::ptr::null::<i3String>() as *mut i3String;
static mut message_option_forget: *mut i3String = ::core::ptr::null::<i3String>() as *mut i3String;
static mut dialog_width: ::core::ffi::c_int = 0;
static mut dialog_height: ::core::ffi::c_int = 0;
static mut border_width: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
static mut margin: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
unsafe extern "C" fn sighandler_backtrace() -> ::core::ffi::c_int {
    let mut tmpdir: *mut ::core::ffi::c_char =
        getenv(b"TMPDIR\0" as *const u8 as *const ::core::ffi::c_char);
    if tmpdir.is_null() {
        tmpdir = b"/tmp\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
    }
    let mut pid_parent: pid_t = getpid() as pid_t;
    let mut filename: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut suffix: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    loop {
        free(filename as *mut ::core::ffi::c_void);
        filename = ::core::ptr::null_mut::<::core::ffi::c_char>();
        sasprintf(
            &raw mut filename,
            b"%s/i3-backtrace.%d.%d.txt\0" as *const u8 as *const ::core::ffi::c_char,
            tmpdir,
            pid_parent,
            suffix,
        );
        suffix += 1;
        if !path_exists(filename) {
            break;
        }
    }
    let mut pid_gdb: pid_t = fork() as pid_t;
    if pid_gdb < 0 as ::core::ffi::c_int {
        debuglog(
            b"%s:%s:%d - Failed to fork for GDB\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b"../src/sighandler.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"sighandler_backtrace\0" as *const u8 as *const ::core::ffi::c_char,
            69 as ::core::ffi::c_int,
        );
        return -(1 as ::core::ffi::c_int);
    } else if pid_gdb == 0 as ::core::ffi::c_int {
        let mut stdin_pipe: [::core::ffi::c_int; 2] = [0; 2];
        let mut stdout_pipe: [::core::ffi::c_int; 2] = [0; 2];
        if pipe(&raw mut stdin_pipe as *mut ::core::ffi::c_int) == -(1 as ::core::ffi::c_int) {
            errorlog(
                b"ERROR: Failed to init stdin_pipe\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
            return -(1 as ::core::ffi::c_int);
        }
        if pipe(&raw mut stdout_pipe as *mut ::core::ffi::c_int) == -(1 as ::core::ffi::c_int) {
            errorlog(
                b"ERROR: Failed to init stdout_pipe\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
            return -(1 as ::core::ffi::c_int);
        }
        close(STDIN_FILENO);
        close(STDOUT_FILENO);
        close(STDERR_FILENO);
        dup2(stdin_pipe[0 as ::core::ffi::c_int as usize], STDIN_FILENO);
        dup2(stdout_pipe[1 as ::core::ffi::c_int as usize], STDOUT_FILENO);
        let mut pid_s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut gdb_log_cmd: *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<::core::ffi::c_char>();
        sasprintf(
            &raw mut pid_s,
            b"%d\0" as *const u8 as *const ::core::ffi::c_char,
            pid_parent,
        );
        sasprintf(
            &raw mut gdb_log_cmd,
            b"set logging file %s\0" as *const u8 as *const ::core::ffi::c_char,
            filename,
        );
        let mut args: [*mut ::core::ffi::c_char; 15] = [
            b"gdb\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            *start_argv.offset(0 as ::core::ffi::c_int as isize),
            b"-p\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            pid_s,
            b"-batch\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"-nx\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"-ex\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            gdb_log_cmd,
            b"-ex\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"set logging on\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b"-ex\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"bt full\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"-ex\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"quit\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
        ];
        execvp(
            args[0 as ::core::ffi::c_int as usize],
            &raw mut args as *mut *mut ::core::ffi::c_char as *const *mut ::core::ffi::c_char,
        );
        err(
            EXIT_FAILURE,
            b"execvp(gdb)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let mut status: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    waitpid(pid_gdb as __pid_t, &raw mut status, 0 as ::core::ffi::c_int);
    if !(status & 0x7f as ::core::ffi::c_int == 0 as ::core::ffi::c_int)
        || (status & 0xff00 as ::core::ffi::c_int) >> 8 as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int
    {
        debuglog(
            b"%s:%s:%d - GDB did not run properly\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b"../src/sighandler.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"sighandler_backtrace\0" as *const u8 as *const ::core::ffi::c_char,
            123 as ::core::ffi::c_int,
        );
        return -(1 as ::core::ffi::c_int);
    } else if !path_exists(filename) {
        debuglog(
            b"%s:%s:%d - GDB executed successfully, but no backtrace was generated\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/sighandler.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"sighandler_backtrace\0" as *const u8 as *const ::core::ffi::c_char,
            126 as ::core::ffi::c_int,
        );
        return -(1 as ::core::ffi::c_int);
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn sighandler_setup() {
    border_width = logical_px(border_width);
    margin = logical_px(margin);
    let mut num_lines: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
    message_intro = i3string_from_utf8(
        b"i3 has just crashed. Please report a bug for this.\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    message_intro2 = i3string_from_utf8(
        b"To debug this problem, you can either attach gdb or choose from the following options:\0"
            as *const u8 as *const ::core::ffi::c_char,
    );
    message_option_backtrace = i3string_from_utf8(
        b"- 'b' to save a backtrace (requires gdb)\0" as *const u8 as *const ::core::ffi::c_char,
    );
    message_option_restart = i3string_from_utf8(
        b"- 'r' to restart i3 in-place\0" as *const u8 as *const ::core::ffi::c_char,
    );
    message_option_forget = i3string_from_utf8(
        b"- 'f' to forget the previous layout and restart i3\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    let mut width_longest_message: ::core::ffi::c_int = predict_text_width(message_intro2);
    dialog_width = width_longest_message
        + 2 as ::core::ffi::c_int * border_width
        + 2 as ::core::ffi::c_int * margin;
    dialog_height = num_lines * config.font.height
        + 2 as ::core::ffi::c_int * border_width
        + 2 as ::core::ffi::c_int * margin;
}
unsafe extern "C" fn sighandler_create_dialogs() {
    let mut output: *mut Output = ::core::ptr::null_mut::<Output>();
    output = outputs.tqh_first as *mut Output;
    while !output.is_null() {
        if (*output).active {
            let mut dialog: *mut dialog_t =
                scalloc(1 as size_t, ::core::mem::size_of::<dialog_t>() as size_t) as *mut dialog_t;
            (*dialog).dialogs.tqe_next = ::core::ptr::null_mut::<dialog_t>();
            (*dialog).dialogs.tqe_prev = dialogs.tqh_last;
            *dialogs.tqh_last = dialog as *mut dialog_t;
            dialogs.tqh_last = &raw mut (*dialog).dialogs.tqe_next;
            let mut visual: xcb_visualid_t = get_visualid_by_depth(root_depth as uint16_t);
            (*dialog).colormap = xcb_generate_id(conn) as xcb_colormap_t;
            xcb_create_colormap(
                conn,
                XCB_COLORMAP_ALLOC_NONE as ::core::ffi::c_int as uint8_t,
                (*dialog).colormap,
                root,
                visual,
            );
            let mut mask: uint32_t = 0 as uint32_t;
            let mut values: [uint32_t; 4] = [0; 4];
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            mask |= XCB_CW_BACK_PIXEL as ::core::ffi::c_int as uint32_t;
            let fresh0 = i;
            i = i + 1;
            values[fresh0 as usize] = (*root_screen).black_pixel;
            mask |= XCB_CW_BORDER_PIXEL as ::core::ffi::c_int as uint32_t;
            let fresh1 = i;
            i = i + 1;
            values[fresh1 as usize] = (*root_screen).black_pixel;
            mask |= XCB_CW_OVERRIDE_REDIRECT as ::core::ffi::c_int as uint32_t;
            let fresh2 = i;
            i = i + 1;
            values[fresh2 as usize] = 1 as uint32_t;
            mask |= XCB_CW_COLORMAP as ::core::ffi::c_int as uint32_t;
            let fresh3 = i;
            i = i + 1;
            values[fresh3 as usize] = (*dialog).colormap as uint32_t;
            (*dialog).dims.x = (*output)
                .rect
                .x
                .wrapping_add((*output).rect.width.wrapping_div(2 as uint32_t));
            (*dialog).dims.y = (*output)
                .rect
                .y
                .wrapping_add((*output).rect.height.wrapping_div(2 as uint32_t));
            (*dialog).dims.width = dialog_width as uint32_t;
            (*dialog).dims.height = dialog_height as uint32_t;
            (*dialog).dims.x = (*dialog)
                .dims
                .x
                .wrapping_sub((*dialog).dims.width.wrapping_div(2 as uint32_t));
            (*dialog).dims.y = (*dialog)
                .dims
                .y
                .wrapping_sub((*dialog).dims.height.wrapping_div(2 as uint32_t));
            (*dialog).id = create_window(
                conn,
                (*dialog).dims,
                root_depth as uint16_t,
                visual,
                XCB_WINDOW_CLASS_INPUT_OUTPUT as ::core::ffi::c_int as uint16_t,
                XCURSOR_CURSOR_POINTER,
                true_0 != 0,
                mask,
                &raw mut values as *mut uint32_t,
            );
            draw_util_surface_init(
                conn,
                &raw mut (*dialog).surface,
                (*dialog).id as xcb_drawable_t,
                get_visualtype_by_id(visual),
                (*dialog).dims.width as ::core::ffi::c_int,
                (*dialog).dims.height as ::core::ffi::c_int,
            );
            xcb_grab_keyboard(
                conn,
                false_0 as uint8_t,
                (*dialog).id,
                XCB_CURRENT_TIME as xcb_timestamp_t,
                XCB_GRAB_MODE_ASYNC as ::core::ffi::c_int as uint8_t,
                XCB_GRAB_MODE_ASYNC as ::core::ffi::c_int as uint8_t,
            );
            xcb_grab_pointer(
                conn,
                false_0 as uint8_t,
                (*dialog).id,
                XCB_NONE as uint16_t,
                XCB_GRAB_MODE_ASYNC as ::core::ffi::c_int as uint8_t,
                XCB_GRAB_MODE_ASYNC as ::core::ffi::c_int as uint8_t,
                (*dialog).id,
                XCB_NONE as xcb_cursor_t,
                XCB_CURRENT_TIME as xcb_timestamp_t,
            );
        }
        output = (*output).outputs.tqe_next as *mut Output;
    }
    sighandler_handle_expose();
    xcb_flush(conn);
}
unsafe extern "C" fn sighandler_destroy_dialogs() {
    while !dialogs.tqh_first.is_null() {
        let mut dialog: *mut dialog_t = dialogs.tqh_first as *mut dialog_t;
        xcb_free_colormap(conn, (*dialog).colormap);
        draw_util_surface_free(conn, &raw mut (*dialog).surface);
        xcb_destroy_window(conn, (*dialog).id);
        if !(*dialog).dialogs.tqe_next.is_null() {
            (*(*dialog).dialogs.tqe_next).dialogs.tqe_prev = (*dialog).dialogs.tqe_prev;
        } else {
            dialogs.tqh_last = (*dialog).dialogs.tqe_prev;
        }
        *(*dialog).dialogs.tqe_prev = (*dialog).dialogs.tqe_next;
        free(dialog as *mut ::core::ffi::c_void);
    }
    xcb_flush(conn);
}
unsafe extern "C" fn sighandler_handle_expose() {
    let mut current: *mut dialog_t = ::core::ptr::null_mut::<dialog_t>();
    current = dialogs.tqh_first as *mut dialog_t;
    while !current.is_null() {
        sighandler_draw_dialog(current);
        current = (*current).dialogs.tqe_next as *mut dialog_t;
    }
    xcb_flush(conn);
}
unsafe extern "C" fn sighandler_draw_dialog(mut dialog: *mut dialog_t) {
    let black: color_t =
        draw_util_hex_to_color(b"#000000\0" as *const u8 as *const ::core::ffi::c_char) as color_t;
    let white: color_t =
        draw_util_hex_to_color(b"#FFFFFF\0" as *const u8 as *const ::core::ffi::c_char) as color_t;
    let red: color_t =
        draw_util_hex_to_color(b"#FF0000\0" as *const u8 as *const ::core::ffi::c_char) as color_t;
    draw_util_clear_surface(&raw mut (*dialog).surface, red);
    draw_util_rectangle(
        &raw mut (*dialog).surface,
        black,
        border_width as ::core::ffi::c_double,
        border_width as ::core::ffi::c_double,
        (*dialog)
            .dims
            .width
            .wrapping_sub((2 as ::core::ffi::c_int * border_width) as uint32_t)
            as ::core::ffi::c_double,
        (*dialog)
            .dims
            .height
            .wrapping_sub((2 as ::core::ffi::c_int * border_width) as uint32_t)
            as ::core::ffi::c_double,
    );
    let mut y: ::core::ffi::c_int = border_width + margin;
    let x: ::core::ffi::c_int = border_width + margin;
    let max_width: ::core::ffi::c_int = (*dialog)
        .dims
        .width
        .wrapping_sub((2 as ::core::ffi::c_int * x) as uint32_t)
        as ::core::ffi::c_int;
    draw_util_text(
        message_intro,
        &raw mut (*dialog).surface,
        white,
        black,
        x,
        y,
        max_width,
    );
    y += config.font.height;
    draw_util_text(
        message_intro2,
        &raw mut (*dialog).surface,
        white,
        black,
        x,
        y,
        max_width,
    );
    y += config.font.height;
    let mut bt_color: *mut ::core::ffi::c_char =
        b"#FFFFFF\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
    if backtrace_done < 0 as ::core::ffi::c_int {
        bt_color =
            b"#AA0000\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
    } else if backtrace_done > 0 as ::core::ffi::c_int {
        bt_color =
            b"#00AA00\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
    }
    draw_util_text(
        message_option_backtrace,
        &raw mut (*dialog).surface,
        draw_util_hex_to_color(bt_color),
        black,
        x,
        y,
        max_width,
    );
    y += config.font.height;
    draw_util_text(
        message_option_restart,
        &raw mut (*dialog).surface,
        white,
        black,
        x,
        y,
        max_width,
    );
    y += config.font.height;
    draw_util_text(
        message_option_forget,
        &raw mut (*dialog).surface,
        white,
        black,
        x,
        y,
        max_width,
    );
    y += config.font.height;
}
unsafe extern "C" fn sighandler_handle_key_press(mut event: *mut xcb_key_press_event_t) {
    let mut state: uint16_t = (*event).state;
    state = (state as ::core::ffi::c_uint & !xcb_numlock_mask) as uint16_t;
    let mut sym: xcb_keysym_t =
        xcb_key_press_lookup_keysym(keysyms, event, state as ::core::ffi::c_int);
    if sym == 'b' as i32 as xcb_keysym_t {
        debuglog(
            b"%s:%s:%d - User issued core-dump command.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/sighandler.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"sighandler_handle_key_press\0" as *const u8 as *const ::core::ffi::c_char,
            280 as ::core::ffi::c_int,
        );
        backtrace_done = sighandler_backtrace();
        sighandler_handle_expose();
    } else if sym == 'r' as i32 as xcb_keysym_t {
        sighandler_destroy_dialogs();
        i3_restart(false_0 != 0);
    } else if sym == 'f' as i32 as xcb_keysym_t {
        sighandler_destroy_dialogs();
        i3_restart(true_0 != 0);
    }
}
unsafe extern "C" fn handle_signal(
    mut sig: ::core::ffi::c_int,
    mut info: *mut siginfo_t,
    mut data: *mut ::core::ffi::c_void,
) {
    debuglog(
        b"%s:%s:%d - i3 crashed. SIG: %d\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/sighandler.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"handle_signal\0" as *const u8 as *const ::core::ffi::c_char,
        296 as ::core::ffi::c_int,
        sig,
    );
    let mut action: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_9 { sa_handler: None },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    action.__sigaction_handler.sa_handler = SIG_DFL;
    action.sa_flags = 0 as ::core::ffi::c_int;
    sigemptyset(&raw mut action.sa_mask);
    sigaction(sig, &raw mut action, ::core::ptr::null_mut::<sigaction>());
    raised_signal = sig;
    sighandler_setup();
    sighandler_create_dialogs();
    let mut event: *mut xcb_generic_event_t = ::core::ptr::null_mut::<xcb_generic_event_t>();
    loop {
        event = xcb_wait_for_event(conn);
        if event.is_null() {
            break;
        }
        let mut type_0: ::core::ffi::c_int =
            (*event).response_type as ::core::ffi::c_int & 0x7f as ::core::ffi::c_int;
        match type_0 {
            XCB_KEY_PRESS => {
                sighandler_handle_key_press(event as *mut xcb_key_press_event_t);
            }
            XCB_EXPOSE => {
                if (*(event as *mut xcb_expose_event_t)).count as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    sighandler_handle_expose();
                }
            }
            _ => {}
        }
        free(event as *mut ::core::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn setup_signal_handler() {
    let mut action: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_9 { sa_handler: None },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    action.__sigaction_handler.sa_sigaction = Some(
        handle_signal
            as unsafe extern "C" fn(
                ::core::ffi::c_int,
                *mut siginfo_t,
                *mut ::core::ffi::c_void,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                ::core::ffi::c_int,
                *mut siginfo_t,
                *mut ::core::ffi::c_void,
            ) -> (),
        >;
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
            b"ERROR: Could not setup signal handler.\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
    }
}
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
unsafe extern "C" fn run_static_initializers() {
    dialogs = dialogs_head {
        tqh_first: ::core::ptr::null_mut::<dialog_t>(),
        tqh_last: &raw mut dialogs.tqh_first,
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
