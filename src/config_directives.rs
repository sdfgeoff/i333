/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * config_directives.c: all config storing functions (see config_parser.c)
 *
 */
// Mechanical file-by-file Rust translation of src/config_directives.c.
// Generated with C2Rust 0.22.1; C ABI and original ownership retained.
extern "C" {
    pub type _PangoFontDescription;
    pub type _cairo;
    pub type _cairo_surface;
    pub type _i3String;
    pub type pcre2_real_code_8;
    pub type ev_loop;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn __assert_single_arg(_: bool) -> bool;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn verboselog(fmt: *mut ::core::ffi::c_char, ...);
    fn errorlog(fmt: *mut ::core::ffi::c_char, ...);
    fn debuglog(fmt: *mut ::core::ffi::c_char, ...);
    fn smalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn scalloc(num: size_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn srealloc(ptr: *mut ::core::ffi::c_void, size: size_t) -> *mut ::core::ffi::c_void;
    fn sstrdup(str: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn sasprintf(
        strp: *mut *mut ::core::ffi::c_char,
        fmt: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn load_font(pattern: *const ::core::ffi::c_char, fallback: bool) -> i3Font;
    fn set_font(font: *mut i3Font);
    fn logical_px(logical: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn draw_util_hex_to_color(color: *const ::core::ffi::c_char) -> color_t;
    fn boolstr(str: *const ::core::ffi::c_char) -> bool;
    fn ws_name_to_number(name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    static mut autostarts: autostarts_head;
    static mut autostarts_always: autostarts_always_head;
    static mut ws_assignments: ws_assignments_head;
    static mut assignments: assignments_head;
    static mut config: Config;
    static mut modes: modes_head;
    static mut barconfigs: barconfig_head;
    static mut included_files: includedfiles_head;
    fn ipc_set_kill_timeout(new: ev_tstamp);
    fn match_init(match_0: *mut Match);
    fn match_is_empty(match_0: *mut Match) -> bool;
    fn match_copy(dest: *mut Match, src: *mut Match);
    fn match_free(match_0: *mut Match);
    fn match_parse_property(
        match_0: *mut Match,
        ctype: *const ::core::ffi::c_char,
        cvalue: *const ::core::ffi::c_char,
    );
    static mut DEFAULT_BINDING_MODE: *const ::core::ffi::c_char;
    fn configure_binding(
        bindtype: *const ::core::ffi::c_char,
        modifiers: *const ::core::ffi::c_char,
        input_code: *const ::core::ffi::c_char,
        release: *const ::core::ffi::c_char,
        border: *const ::core::ffi::c_char,
        whole_window: *const ::core::ffi::c_char,
        exclude_titlebar: *const ::core::ffi::c_char,
        command: *const ::core::ffi::c_char,
        mode: *const ::core::ffi::c_char,
        pango_markup: bool,
    ) -> *mut Binding;
    fn parse_file(
        ctx: *mut parser_ctx,
        f: *const ::core::ffi::c_char,
        included_file: *mut IncludedFile,
    ) -> parse_file_result_t;
    fn wordexp(
        __words: *const ::core::ffi::c_char,
        __pwordexp: *mut wordexp_t,
        __flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn wordfree(__wordexp: *mut wordexp_t);
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    fn strcasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strncasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn atoi(__nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn realpath(
        __name: *const ::core::ffi::c_char,
        __resolved: *mut ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __time_t = ::core::ffi::c_long;
pub type __suseconds_t = ::core::ffi::c_long;
pub type time_t = __time_t;
pub type int16_t = __int16_t;
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
pub type xcb_window_t = uint32_t;
pub type xcb_font_t = uint32_t;
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
pub type PangoFontDescription = _PangoFontDescription;
pub type cairo_t = _cairo;
pub type cairo_surface_t = _cairo_surface;
pub type i3String = _i3String;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Font {
    pub type_0: C2RustUnnamed_1,
    pub height: ::core::ffi::c_int,
    pub pattern: *mut ::core::ffi::c_char,
    pub specific: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub xcb: C2RustUnnamed_0,
    pub pango_desc: *mut PangoFontDescription,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub id: xcb_font_t,
    pub info: *mut xcb_query_font_reply_t,
    pub table: *mut xcb_charinfo_t,
}
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const FONT_TYPE_PANGO: C2RustUnnamed_1 = 2;
pub const FONT_TYPE_XCB: C2RustUnnamed_1 = 1;
pub const FONT_TYPE_NONE: C2RustUnnamed_1 = 0;
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
pub type pcre2_code_8 = pcre2_real_code_8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Binding {
    pub input_type: input_type_t,
    pub release: C2RustUnnamed_4,
    pub border: bool,
    pub whole_window: bool,
    pub exclude_titlebar: bool,
    pub keycode: uint32_t,
    pub event_state_mask: i3_event_state_mask_t,
    pub symbol: *mut ::core::ffi::c_char,
    pub keycodes_head: keycodes_head,
    pub command: *mut ::core::ffi::c_char,
    pub bindings: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
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
    pub keycodes: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub tqe_next: *mut Binding_Keycode,
    pub tqe_prev: *mut *mut Binding_Keycode,
}
pub type i3_event_state_mask_t = uint32_t;
pub type C2RustUnnamed_4 = ::core::ffi::c_uint;
pub const B_UPON_KEYRELEASE_IGNORE_MODS: C2RustUnnamed_4 = 2;
pub const B_UPON_KEYRELEASE: C2RustUnnamed_4 = 1;
pub const B_UPON_KEYPRESS: C2RustUnnamed_4 = 0;
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
pub struct Con {
    pub mapped: bool,
    pub urgent: bool,
    pub ignore_unmap: uint8_t,
    pub frame: surface_t,
    pub frame_buffer: surface_t,
    pub pixmap_recreated: bool,
    pub type_0: C2RustUnnamed_21,
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
    pub floating: C2RustUnnamed_10,
    pub nodes: C2RustUnnamed_9,
    pub focused: C2RustUnnamed_8,
    pub all_cons: C2RustUnnamed_7,
    pub floating_windows: C2RustUnnamed_6,
    pub on_remove_child: Option<unsafe extern "C" fn(*mut Con) -> ()>,
    pub scratchpad_state: C2RustUnnamed_5,
    pub old_id: ::core::ffi::c_int,
    pub depth: uint16_t,
    pub colormap: xcb_colormap_t,
}
pub type C2RustUnnamed_5 = ::core::ffi::c_uint;
pub const SCRATCHPAD_CHANGED: C2RustUnnamed_5 = 2;
pub const SCRATCHPAD_FRESH: C2RustUnnamed_5 = 1;
pub const SCRATCHPAD_NONE: C2RustUnnamed_5 = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub tqe_next: *mut Con,
    pub tqe_prev: *mut *mut Con,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub tqe_next: *mut Con,
    pub tqe_prev: *mut *mut Con,
}
pub type C2RustUnnamed_10 = ::core::ffi::c_uint;
pub const FLOATING_USER_ON: C2RustUnnamed_10 = 3;
pub const FLOATING_AUTO_ON: C2RustUnnamed_10 = 2;
pub const FLOATING_USER_OFF: C2RustUnnamed_10 = 1;
pub const FLOATING_AUTO_OFF: C2RustUnnamed_10 = 0;
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
    pub urgent: C2RustUnnamed_15,
    pub dock: C2RustUnnamed_14,
    pub id: xcb_window_t,
    pub window_mode: C2RustUnnamed_13,
    pub con_id: *mut Con,
    pub match_all_windows: bool,
    pub insert_where: C2RustUnnamed_12,
    pub matches: C2RustUnnamed_11,
    pub restart_mode: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub tqe_next: *mut Match,
    pub tqe_prev: *mut *mut Match,
}
pub type C2RustUnnamed_12 = ::core::ffi::c_uint;
pub const M_BELOW: C2RustUnnamed_12 = 2;
pub const M_ASSIGN_WS: C2RustUnnamed_12 = 1;
pub const M_HERE: C2RustUnnamed_12 = 0;
pub type C2RustUnnamed_13 = ::core::ffi::c_uint;
pub const WM_FLOATING: C2RustUnnamed_13 = 6;
pub const WM_FLOATING_USER: C2RustUnnamed_13 = 5;
pub const WM_FLOATING_AUTO: C2RustUnnamed_13 = 4;
pub const WM_TILING: C2RustUnnamed_13 = 3;
pub const WM_TILING_USER: C2RustUnnamed_13 = 2;
pub const WM_TILING_AUTO: C2RustUnnamed_13 = 1;
pub const WM_ANY: C2RustUnnamed_13 = 0;
pub type C2RustUnnamed_14 = ::core::ffi::c_int;
pub const M_DOCK_BOTTOM: C2RustUnnamed_14 = 3;
pub const M_DOCK_TOP: C2RustUnnamed_14 = 2;
pub const M_DOCK_ANY: C2RustUnnamed_14 = 1;
pub const M_NODOCK: C2RustUnnamed_14 = 0;
pub const M_DONTCHECK: C2RustUnnamed_14 = -1;
pub type C2RustUnnamed_15 = ::core::ffi::c_int;
pub const U_OLDEST: C2RustUnnamed_15 = 1;
pub const U_LATEST: C2RustUnnamed_15 = 0;
pub const U_DONTCHECK: C2RustUnnamed_15 = -1;
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
    pub dock: C2RustUnnamed_16,
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
pub type C2RustUnnamed_16 = ::core::ffi::c_uint;
pub const W_DOCK_BOTTOM: C2RustUnnamed_16 = 2;
pub const W_DOCK_TOP: C2RustUnnamed_16 = 1;
pub const W_NODOCK: C2RustUnnamed_16 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Assignment {
    pub type_0: C2RustUnnamed_19,
    pub match_0: Match,
    pub dest: C2RustUnnamed_18,
    pub assignments: C2RustUnnamed_17,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_17 {
    pub tqe_next: *mut Assignment,
    pub tqe_prev: *mut *mut Assignment,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_18 {
    pub command: *mut ::core::ffi::c_char,
    pub workspace: *mut ::core::ffi::c_char,
    pub output: *mut ::core::ffi::c_char,
}
pub type C2RustUnnamed_19 = ::core::ffi::c_uint;
pub const A_TO_OUTPUT: C2RustUnnamed_19 = 16;
pub const A_TO_WORKSPACE_NUMBER: C2RustUnnamed_19 = 8;
pub const A_NO_FOCUS: C2RustUnnamed_19 = 4;
pub const A_TO_WORKSPACE: C2RustUnnamed_19 = 2;
pub const A_COMMAND: C2RustUnnamed_19 = 1;
pub const A_ANY: C2RustUnnamed_19 = 0;
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
    pub marks: C2RustUnnamed_20,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_20 {
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
pub type C2RustUnnamed_21 = ::core::ffi::c_uint;
pub const CT_DOCKAREA: C2RustUnnamed_21 = 5;
pub const CT_WORKSPACE: C2RustUnnamed_21 = 4;
pub const CT_FLOATING_CON: C2RustUnnamed_21 = 3;
pub const CT_CON: C2RustUnnamed_21 = 2;
pub const CT_OUTPUT: C2RustUnnamed_21 = 1;
pub const CT_ROOT: C2RustUnnamed_21 = 0;
pub type C2RustUnnamed_22 = ::core::ffi::c_uint;
pub const VERT: C2RustUnnamed_22 = 2;
pub const HORIZ: C2RustUnnamed_22 = 1;
pub const NO_ORIENTATION: C2RustUnnamed_22 = 0;
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
pub type C2RustUnnamed_23 = ::core::ffi::c_uint;
pub const I3_XKB_GROUP_MASK_4: C2RustUnnamed_23 = 8;
pub const I3_XKB_GROUP_MASK_3: C2RustUnnamed_23 = 4;
pub const I3_XKB_GROUP_MASK_2: C2RustUnnamed_23 = 2;
pub const I3_XKB_GROUP_MASK_1: C2RustUnnamed_23 = 1;
pub const I3_XKB_GROUP_MASK_ANY: C2RustUnnamed_23 = 0;
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
    pub ws_assignments: C2RustUnnamed_24,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_24 {
    pub tqe_next: *mut Workspace_Assignment,
    pub tqe_prev: *mut *mut Workspace_Assignment,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Autostart {
    pub command: *mut ::core::ffi::c_char,
    pub no_startup_id: bool,
    pub autostarts: C2RustUnnamed_26,
    pub autostarts_always: C2RustUnnamed_25,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_25 {
    pub tqe_next: *mut Autostart,
    pub tqe_prev: *mut *mut Autostart,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_26 {
    pub tqe_next: *mut Autostart,
    pub tqe_prev: *mut *mut Autostart,
}
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
pub struct IncludedFile {
    pub path: *mut ::core::ffi::c_char,
    pub raw_contents: *mut ::core::ffi::c_char,
    pub variable_replaced_contents: *mut ::core::ffi::c_char,
    pub files: C2RustUnnamed_27,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_27 {
    pub tqe_next: *mut IncludedFile,
    pub tqe_prev: *mut *mut IncludedFile,
}
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
    pub focus_on_window_activation: C2RustUnnamed_30,
    pub show_marks: bool,
    pub title_align: C2RustUnnamed_29,
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
    pub popup_during_fullscreen: C2RustUnnamed_28,
    pub number_barconfigs: ::core::ffi::c_int,
    pub tiling_drag: tiling_drag_t,
    pub gaps: gaps_t,
    pub smart_gaps: smart_gaps_t,
}
pub type C2RustUnnamed_28 = ::core::ffi::c_uint;
pub const PDF_ALL: C2RustUnnamed_28 = 3;
pub const PDF_IGNORE: C2RustUnnamed_28 = 2;
pub const PDF_LEAVE_FULLSCREEN: C2RustUnnamed_28 = 1;
pub const PDF_SMART: C2RustUnnamed_28 = 0;
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
pub type C2RustUnnamed_29 = ::core::ffi::c_uint;
pub const ALIGN_RIGHT: C2RustUnnamed_29 = 2;
pub const ALIGN_CENTER: C2RustUnnamed_29 = 1;
pub const ALIGN_LEFT: C2RustUnnamed_29 = 0;
pub type C2RustUnnamed_30 = ::core::ffi::c_uint;
pub const FOWA_NONE: C2RustUnnamed_30 = 3;
pub const FOWA_FOCUS: C2RustUnnamed_30 = 2;
pub const FOWA_URGENT: C2RustUnnamed_30 = 1;
pub const FOWA_SMART: C2RustUnnamed_30 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Barconfig {
    pub id: *mut ::core::ffi::c_char,
    pub num_outputs: ::core::ffi::c_int,
    pub outputs: *mut *mut ::core::ffi::c_char,
    pub tray_outputs: tray_outputs_head,
    pub tray_padding: ::core::ffi::c_int,
    pub socket_path: *mut ::core::ffi::c_char,
    pub mode: C2RustUnnamed_35,
    pub hidden_state: C2RustUnnamed_34,
    pub modifier: uint32_t,
    pub bar_bindings: bar_bindings_head,
    pub position: C2RustUnnamed_32,
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
    pub configs: C2RustUnnamed_31,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_31 {
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
pub type C2RustUnnamed_32 = ::core::ffi::c_uint;
pub const P_TOP: C2RustUnnamed_32 = 1;
pub const P_BOTTOM: C2RustUnnamed_32 = 0;
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
    pub bindings: C2RustUnnamed_33,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_33 {
    pub tqe_next: *mut Barbinding,
    pub tqe_prev: *mut *mut Barbinding,
}
pub type C2RustUnnamed_34 = ::core::ffi::c_uint;
pub const S_SHOW: C2RustUnnamed_34 = 1;
pub const S_HIDE: C2RustUnnamed_34 = 0;
pub type C2RustUnnamed_35 = ::core::ffi::c_uint;
pub const M_INVISIBLE: C2RustUnnamed_35 = 2;
pub const M_HIDE: C2RustUnnamed_35 = 1;
pub const M_DOCK: C2RustUnnamed_35 = 0;
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
    pub tray_outputs: C2RustUnnamed_36,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_36 {
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
    pub modes: C2RustUnnamed_37,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_37 {
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
#[repr(C)]
pub struct Variable {
    pub key: *mut ::core::ffi::c_char,
    pub value: *mut ::core::ffi::c_char,
    pub next_match: *mut ::core::ffi::c_char,
    pub variables: C2RustUnnamed_38,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_38 {
    pub sle_next: *mut Variable,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack_entry {
    pub identifier: *const ::core::ffi::c_char,
    pub type_0: C2RustUnnamed_40,
    pub val: C2RustUnnamed_39,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_39 {
    pub str_0: *mut ::core::ffi::c_char,
    pub num: ::core::ffi::c_long,
}
pub type C2RustUnnamed_40 = ::core::ffi::c_uint;
pub const STACK_LONG: C2RustUnnamed_40 = 1;
pub const STACK_STR: C2RustUnnamed_40 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack {
    pub stack: [stack_entry; 10],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct variables_head {
    pub slh_first: *mut Variable,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct parser_ctx {
    pub use_nagbar: bool,
    pub state: ::core::ffi::c_int,
    pub current_match: Match,
    pub statelist: [::core::ffi::c_int; 10],
    pub statelist_idx: ::core::ffi::c_int,
    pub stack: *mut stack,
    pub variables: variables_head,
    pub has_errors: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ConfigResultIR {
    pub ctx: *mut parser_ctx,
    pub next_state: ::core::ffi::c_int,
    pub has_errors: bool,
}
pub type parse_file_result_t = ::core::ffi::c_int;
pub const PARSE_FILE_CONFIG_ERRORS: parse_file_result_t = 1;
pub const PARSE_FILE_SUCCESS: parse_file_result_t = 0;
pub const PARSE_FILE_FAILED: parse_file_result_t = -1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wordexp_t {
    pub we_wordc: size_t,
    pub we_wordv: *mut *mut ::core::ffi::c_char,
    pub we_offs: size_t,
}
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const XCB_NONE: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[no_mangle]
pub unsafe extern "C" fn cfg_include(
    mut current_match: *mut Match,
    mut result: *mut ConfigResultIR,
    mut pattern: *const ::core::ffi::c_char,
) {
    debuglog(
        b"%s:%s:%d - include %s\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/config_directives.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"cfg_include\0" as *const u8 as *const ::core::ffi::c_char,
        19 as ::core::ffi::c_int,
        pattern,
    );
    let mut p: wordexp_t = wordexp_t {
        we_wordc: 0,
        we_wordv: ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        we_offs: 0,
    };
    let ret: ::core::ffi::c_int =
        wordexp(pattern, &raw mut p, 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    if ret != 0 as ::core::ffi::c_int {
        errorlog(
            b"ERROR: wordexp(%s): error %d\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            pattern,
            ret,
        );
        (*result).has_errors = true_0 != 0;
        return;
    }
    let mut w: *mut *mut ::core::ffi::c_char = p.we_wordv;
    let mut i: size_t = 0 as size_t;
    while i < p.we_wordc {
        let mut resolved_path: [::core::ffi::c_char; 4096] = [
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
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
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
        if realpath(
            *w.offset(i as isize),
            &raw mut resolved_path as *mut ::core::ffi::c_char,
        )
        .is_null()
        {
            verboselog(
                b"Skipping %s: %s\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                *w.offset(i as isize),
                strerror(*__errno_location()),
            );
        } else {
            let mut skip: bool = false_0 != 0;
            let mut file: *mut IncludedFile = ::core::ptr::null_mut::<IncludedFile>();
            file = included_files.tqh_first as *mut IncludedFile;
            while !file.is_null() {
                if strcmp(
                    (*file).path,
                    &raw mut resolved_path as *mut ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    skip = true_0 != 0;
                    break;
                } else {
                    file = (*file).files.tqe_next as *mut IncludedFile;
                }
            }
            if skip {
                verboselog(
                    b"Skipping file %s (already included)\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    &raw mut resolved_path as *mut ::core::ffi::c_char,
                );
            } else {
                verboselog(
                    b"Including config file %s\n\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    &raw mut resolved_path as *mut ::core::ffi::c_char,
                );
                file = scalloc(
                    1 as size_t,
                    ::core::mem::size_of::<IncludedFile>() as size_t,
                ) as *mut IncludedFile;
                (*file).path = sstrdup(&raw mut resolved_path as *mut ::core::ffi::c_char);
                (*file).files.tqe_next = ::core::ptr::null_mut::<IncludedFile>();
                (*file).files.tqe_prev = included_files.tqh_last;
                *included_files.tqh_last = file as *mut IncludedFile;
                included_files.tqh_last = &raw mut (*file).files.tqe_next;
                let mut stack: stack = stack {
                    stack: [
                        stack_entry {
                            identifier: ::core::ptr::null::<::core::ffi::c_char>(),
                            type_0: STACK_STR,
                            val: C2RustUnnamed_39 {
                                str_0: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                            },
                        },
                        stack_entry {
                            identifier: ::core::ptr::null::<::core::ffi::c_char>(),
                            type_0: STACK_STR,
                            val: C2RustUnnamed_39 {
                                str_0: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                            },
                        },
                        stack_entry {
                            identifier: ::core::ptr::null::<::core::ffi::c_char>(),
                            type_0: STACK_STR,
                            val: C2RustUnnamed_39 {
                                str_0: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                            },
                        },
                        stack_entry {
                            identifier: ::core::ptr::null::<::core::ffi::c_char>(),
                            type_0: STACK_STR,
                            val: C2RustUnnamed_39 {
                                str_0: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                            },
                        },
                        stack_entry {
                            identifier: ::core::ptr::null::<::core::ffi::c_char>(),
                            type_0: STACK_STR,
                            val: C2RustUnnamed_39 {
                                str_0: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                            },
                        },
                        stack_entry {
                            identifier: ::core::ptr::null::<::core::ffi::c_char>(),
                            type_0: STACK_STR,
                            val: C2RustUnnamed_39 {
                                str_0: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                            },
                        },
                        stack_entry {
                            identifier: ::core::ptr::null::<::core::ffi::c_char>(),
                            type_0: STACK_STR,
                            val: C2RustUnnamed_39 {
                                str_0: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                            },
                        },
                        stack_entry {
                            identifier: ::core::ptr::null::<::core::ffi::c_char>(),
                            type_0: STACK_STR,
                            val: C2RustUnnamed_39 {
                                str_0: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                            },
                        },
                        stack_entry {
                            identifier: ::core::ptr::null::<::core::ffi::c_char>(),
                            type_0: STACK_STR,
                            val: C2RustUnnamed_39 {
                                str_0: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                            },
                        },
                        stack_entry {
                            identifier: ::core::ptr::null::<::core::ffi::c_char>(),
                            type_0: STACK_STR,
                            val: C2RustUnnamed_39 {
                                str_0: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                            },
                        },
                    ],
                };
                let mut ctx: parser_ctx = parser_ctx {
                    use_nagbar: (*(*result).ctx).use_nagbar,
                    state: 0,
                    current_match: Match {
                        error: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                        title: ::core::ptr::null_mut::<regex>(),
                        application: ::core::ptr::null_mut::<regex>(),
                        class: ::core::ptr::null_mut::<regex>(),
                        instance: ::core::ptr::null_mut::<regex>(),
                        mark: ::core::ptr::null_mut::<regex>(),
                        window_role: ::core::ptr::null_mut::<regex>(),
                        workspace: ::core::ptr::null_mut::<regex>(),
                        machine: ::core::ptr::null_mut::<regex>(),
                        window_type: 0,
                        urgent: U_LATEST,
                        dock: M_NODOCK,
                        id: 0,
                        window_mode: WM_ANY,
                        con_id: ::core::ptr::null_mut::<Con>(),
                        match_all_windows: false,
                        insert_where: M_HERE,
                        matches: C2RustUnnamed_11 {
                            tqe_next: ::core::ptr::null_mut::<Match>(),
                            tqe_prev: ::core::ptr::null_mut::<*mut Match>(),
                        },
                        restart_mode: false,
                    },
                    statelist: [0; 10],
                    statelist_idx: 0,
                    stack: &raw mut stack,
                    variables: (*(*result).ctx).variables,
                    has_errors: false,
                };
                let mut current_block_46: u64;
                match parse_file(
                    &raw mut ctx,
                    &raw mut resolved_path as *mut ::core::ffi::c_char,
                    file,
                ) as ::core::ffi::c_int
                {
                    0 => {
                        current_block_46 = 10150597327160359210;
                    }
                    -1 => {
                        errorlog(
                            b"ERROR: including config file %s: %s\n\0" as *const u8
                                as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            &raw mut resolved_path as *mut ::core::ffi::c_char,
                            strerror(*__errno_location()),
                        );
                        current_block_46 = 1383695296304875401;
                    }
                    1 => {
                        current_block_46 = 1383695296304875401;
                    }
                    _ => {
                        if 0 as ::core::ffi::c_int != 0 {
                        } else {
                            __assert_fail(
                                b"false\0" as *const u8 as *const ::core::ffi::c_char,
                                b"../src/config_directives.c\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                80 as ::core::ffi::c_uint,
                                b"void cfg_include(Match *, struct ConfigResultIR *, const char *)\0"
                                    as *const u8 as *const ::core::ffi::c_char,
                            );
                        };
                        current_block_46 = 10150597327160359210;
                    }
                }
                match current_block_46 {
                    1383695296304875401 => {
                        (*result).has_errors = true_0 != 0;
                        if !(*file).files.tqe_next.is_null() {
                            (*(*file).files.tqe_next).files.tqe_prev = (*file).files.tqe_prev;
                        } else {
                            included_files.tqh_last = (*file).files.tqe_prev;
                        }
                        *(*file).files.tqe_prev = (*file).files.tqe_next;
                        free((*file).path as *mut ::core::ffi::c_void);
                        (*file).path = ::core::ptr::null_mut::<::core::ffi::c_char>();
                        free((*file).raw_contents as *mut ::core::ffi::c_void);
                        (*file).raw_contents = ::core::ptr::null_mut::<::core::ffi::c_char>();
                        free((*file).variable_replaced_contents as *mut ::core::ffi::c_void);
                        (*file).variable_replaced_contents =
                            ::core::ptr::null_mut::<::core::ffi::c_char>();
                        free(file as *mut ::core::ffi::c_void);
                        file = ::core::ptr::null_mut::<IncludedFile>();
                    }
                    _ => {}
                }
                (*(*result).ctx).variables = ctx.variables;
            }
        }
        i = i.wrapping_add(1);
    }
    wordfree(&raw mut p);
}
static mut criteria_next_state: ::core::ffi::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn cfg_criteria_init(
    mut current_match: *mut Match,
    mut result: *mut ConfigResultIR,
    _state: ::core::ffi::c_int,
) {
    criteria_next_state = _state;
    debuglog(
        b"%s:%s:%d - Initializing criteria, current_match = %p, state = %d\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/config_directives.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"cfg_criteria_init\0" as *const u8 as *const ::core::ffi::c_char,
        101 as ::core::ffi::c_int,
        current_match,
        _state,
    );
    match_free(current_match);
    match_init(current_match);
}
#[no_mangle]
pub unsafe extern "C" fn cfg_criteria_pop_state(
    mut current_match: *mut Match,
    mut result: *mut ConfigResultIR,
) {
    (*result).next_state = criteria_next_state;
}
#[no_mangle]
pub unsafe extern "C" fn cfg_criteria_add(
    mut current_match: *mut Match,
    mut result: *mut ConfigResultIR,
    mut ctype: *const ::core::ffi::c_char,
    mut cvalue: *const ::core::ffi::c_char,
) {
    match_parse_property(current_match, ctype, cvalue);
}
#[no_mangle]
pub unsafe extern "C" fn event_state_from_str(
    mut str: *const ::core::ffi::c_char,
) -> i3_event_state_mask_t {
    let mut result: i3_event_state_mask_t = 0 as i3_event_state_mask_t;
    if str.is_null() {
        return result;
    }
    if !strstr(str, b"Mod1\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
        result |= XCB_KEY_BUT_MASK_MOD_1 as ::core::ffi::c_int as i3_event_state_mask_t;
    }
    if !strstr(str, b"Mod2\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
        result |= XCB_KEY_BUT_MASK_MOD_2 as ::core::ffi::c_int as i3_event_state_mask_t;
    }
    if !strstr(str, b"Mod3\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
        result |= XCB_KEY_BUT_MASK_MOD_3 as ::core::ffi::c_int as i3_event_state_mask_t;
    }
    if !strstr(str, b"Mod4\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
        result |= XCB_KEY_BUT_MASK_MOD_4 as ::core::ffi::c_int as i3_event_state_mask_t;
    }
    if !strstr(str, b"Mod5\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
        result |= XCB_KEY_BUT_MASK_MOD_5 as ::core::ffi::c_int as i3_event_state_mask_t;
    }
    if !strstr(str, b"Control\0" as *const u8 as *const ::core::ffi::c_char).is_null()
        || !strstr(str, b"Ctrl\0" as *const u8 as *const ::core::ffi::c_char).is_null()
    {
        result |= XCB_KEY_BUT_MASK_CONTROL as ::core::ffi::c_int as i3_event_state_mask_t;
    }
    if !strstr(str, b"Shift\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
        result |= XCB_KEY_BUT_MASK_SHIFT as ::core::ffi::c_int as i3_event_state_mask_t;
    }
    if !strstr(str, b"Group1\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
        result |= ((I3_XKB_GROUP_MASK_1 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int)
            as i3_event_state_mask_t;
    }
    if !strstr(str, b"Group2\0" as *const u8 as *const ::core::ffi::c_char).is_null()
        || !strstr(
            str,
            b"Mode_switch\0" as *const u8 as *const ::core::ffi::c_char,
        )
        .is_null()
    {
        result |= ((I3_XKB_GROUP_MASK_2 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int)
            as i3_event_state_mask_t;
    }
    if !strstr(str, b"Group3\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
        result |= ((I3_XKB_GROUP_MASK_3 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int)
            as i3_event_state_mask_t;
    }
    if !strstr(str, b"Group4\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
        result |= ((I3_XKB_GROUP_MASK_4 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int)
            as i3_event_state_mask_t;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn cfg_font(
    mut current_match: *mut Match,
    mut result: *mut ConfigResultIR,
    mut font: *const ::core::ffi::c_char,
) {
    config.font = load_font(font, true_0 != 0);
    set_font(&raw mut config.font);
}
#[no_mangle]
pub unsafe extern "C" fn cfg_binding(
    mut current_match: *mut Match,
    mut result: *mut ConfigResultIR,
    mut bindtype: *const ::core::ffi::c_char,
    mut modifiers: *const ::core::ffi::c_char,
    mut key: *const ::core::ffi::c_char,
    mut release: *const ::core::ffi::c_char,
    mut border: *const ::core::ffi::c_char,
    mut whole_window: *const ::core::ffi::c_char,
    mut exclude_titlebar: *const ::core::ffi::c_char,
    mut command: *const ::core::ffi::c_char,
) {
    configure_binding(
        bindtype,
        modifiers,
        key,
        release,
        border,
        whole_window,
        exclude_titlebar,
        command,
        DEFAULT_BINDING_MODE,
        false_0 != 0,
    );
}
static mut current_mode: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
static mut current_mode_pango_markup: bool = false;
#[no_mangle]
pub unsafe extern "C" fn cfg_mode_binding(
    mut current_match: *mut Match,
    mut result: *mut ConfigResultIR,
    mut bindtype: *const ::core::ffi::c_char,
    mut modifiers: *const ::core::ffi::c_char,
    mut key: *const ::core::ffi::c_char,
    mut release: *const ::core::ffi::c_char,
    mut border: *const ::core::ffi::c_char,
    mut whole_window: *const ::core::ffi::c_char,
    mut exclude_titlebar: *const ::core::ffi::c_char,
    mut command: *const ::core::ffi::c_char,
) {
    if current_mode.is_null() {
        return;
    }
    configure_binding(
        bindtype,
        modifiers,
        key,
        release,
        border,
        whole_window,
        exclude_titlebar,
        command,
        current_mode,
        current_mode_pango_markup,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cfg_enter_mode(
    mut current_match: *mut Match,
    mut result: *mut ConfigResultIR,
    mut pango_markup: *const ::core::ffi::c_char,
    mut modename: *const ::core::ffi::c_char,
) {
    if strcmp(modename, DEFAULT_BINDING_MODE) == 0 as ::core::ffi::c_int {
        errorlog(
            b"ERROR: You cannot use the name %s for your mode\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            DEFAULT_BINDING_MODE,
        );
        return;
    }
    let mut mode: *mut Mode = ::core::ptr::null_mut::<Mode>();
    mode = modes.slh_first as *mut Mode;
    while !mode.is_null() {
        if strcmp((*mode).name, modename) == 0 as ::core::ffi::c_int {
            errorlog(
                b"ERROR: The binding mode with name \"%s\" is defined at least twice.\n\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                modename,
            );
        }
        mode = (*mode).modes.sle_next;
    }
    debuglog(
        b"%s:%s:%d - \t now in mode %s\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/config_directives.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"cfg_enter_mode\0" as *const u8 as *const ::core::ffi::c_char,
        211 as ::core::ffi::c_int,
        modename,
    );
    free(current_mode as *mut ::core::ffi::c_void);
    current_mode = ::core::ptr::null_mut::<::core::ffi::c_char>();
    current_mode = sstrdup(modename);
    current_mode_pango_markup = !pango_markup.is_null();
}
#[no_mangle]
pub unsafe extern "C" fn cfg_exec(
    mut current_match: *mut Match,
    mut result: *mut ConfigResultIR,
    mut exectype: *const ::core::ffi::c_char,
    mut no_startup_id: *const ::core::ffi::c_char,
    mut command: *const ::core::ffi::c_char,
) {
    let mut new: *mut Autostart =
        smalloc(::core::mem::size_of::<Autostart>() as size_t) as *mut Autostart;
    (*new).command = sstrdup(command);
    (*new).no_startup_id = !no_startup_id.is_null();
    if strcmp(
        exectype,
        b"exec\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        (*new).autostarts.tqe_next = ::core::ptr::null_mut::<Autostart>();
        (*new).autostarts.tqe_prev = autostarts.tqh_last;
        *autostarts.tqh_last = new;
        autostarts.tqh_last = &raw mut (*new).autostarts.tqe_next;
    } else {
        (*new).autostarts_always.tqe_next = ::core::ptr::null_mut::<Autostart>();
        (*new).autostarts_always.tqe_prev = autostarts_always.tqh_last;
        *autostarts_always.tqh_last = new;
        autostarts_always.tqh_last = &raw mut (*new).autostarts_always.tqe_next;
    };
}
#[no_mangle]
pub unsafe extern "C" fn cfg_for_window(
    mut current_match: *mut Match,
    mut result: *mut ConfigResultIR,
    mut command: *const ::core::ffi::c_char,
) {
    if !(*current_match).error.is_null() {
        errorlog(
            b"ERROR: match has error: %s\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            (*current_match).error,
        );
        return;
    }
    if match_is_empty(current_match) {
        errorlog(
            b"ERROR: Match is empty, ignoring this for_window statement\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        return;
    }
    debuglog(
        b"%s:%s:%d - \t should execute command %s for the criteria mentioned above\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/config_directives.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"cfg_for_window\0" as *const u8 as *const ::core::ffi::c_char,
        237 as ::core::ffi::c_int,
        command,
    );
    let mut assignment: *mut Assignment =
        scalloc(1 as size_t, ::core::mem::size_of::<Assignment>() as size_t) as *mut Assignment;
    (*assignment).type_0 = A_COMMAND;
    match_copy(&raw mut (*assignment).match_0, current_match);
    (*assignment).dest.command = sstrdup(command);
    (*assignment).assignments.tqe_next = ::core::ptr::null_mut::<Assignment>();
    (*assignment).assignments.tqe_prev = assignments.tqh_last;
    *assignments.tqh_last = assignment as *mut Assignment;
    assignments.tqh_last = &raw mut (*assignment).assignments.tqe_next;
}
unsafe extern "C" fn apply_gaps(
    mut gaps: *mut gaps_t,
    mask: gaps_mask_t,
    value: ::core::ffi::c_int,
) {
    if gaps.is_null() {
        return;
    }
    if mask as ::core::ffi::c_uint & GAPS_INNER as ::core::ffi::c_int as ::core::ffi::c_uint != 0 {
        (*gaps).inner = value;
    }
    if mask as ::core::ffi::c_uint & GAPS_TOP as ::core::ffi::c_int as ::core::ffi::c_uint != 0 {
        (*gaps).top = value;
    }
    if mask as ::core::ffi::c_uint & GAPS_RIGHT as ::core::ffi::c_int as ::core::ffi::c_uint != 0 {
        (*gaps).right = value;
    }
    if mask as ::core::ffi::c_uint & GAPS_BOTTOM as ::core::ffi::c_int as ::core::ffi::c_uint != 0 {
        (*gaps).bottom = value;
    }
    if mask as ::core::ffi::c_uint & GAPS_LEFT as ::core::ffi::c_int as ::core::ffi::c_uint != 0 {
        (*gaps).left = value;
    }
}
unsafe extern "C" fn create_gaps_assignment(
    mut workspace: *const ::core::ffi::c_char,
    mask: gaps_mask_t,
    pixels: ::core::ffi::c_int,
) {
    if mask as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint {
        return;
    }
    debuglog(
        b"%s:%s:%d - Setting gaps for workspace %s\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/config_directives.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"create_gaps_assignment\0" as *const u8 as *const ::core::ffi::c_char,
        271 as ::core::ffi::c_int,
        workspace,
    );
    let mut found: bool = false_0 != 0;
    let mut assignment: *mut Workspace_Assignment = ::core::ptr::null_mut::<Workspace_Assignment>();
    assignment = ws_assignments.tqh_first;
    while !assignment.is_null() {
        if strcasecmp((*assignment).name, workspace) == 0 as ::core::ffi::c_int {
            found = true_0 != 0;
            break;
        } else {
            assignment = (*assignment).ws_assignments.tqe_next;
        }
    }
    if !found {
        assignment = scalloc(
            1 as size_t,
            ::core::mem::size_of::<Workspace_Assignment>() as size_t,
        ) as *mut Workspace_Assignment;
        (*assignment).name = sstrdup(workspace);
        (*assignment).output = ::core::ptr::null_mut::<::core::ffi::c_char>();
        (*assignment).ws_assignments.tqe_next = ::core::ptr::null_mut::<Workspace_Assignment>();
        (*assignment).ws_assignments.tqe_prev = ws_assignments.tqh_last;
        *ws_assignments.tqh_last = assignment;
        ws_assignments.tqh_last = &raw mut (*assignment).ws_assignments.tqe_next;
    }
    (*assignment).gaps_mask = ::core::mem::transmute::<::core::ffi::c_uint, gaps_mask_t>(
        (*assignment).gaps_mask as ::core::ffi::c_uint | mask as ::core::ffi::c_uint,
    );
    apply_gaps(&raw mut (*assignment).gaps, mask, pixels);
}
unsafe extern "C" fn gaps_scope_to_mask(mut scope: *const ::core::ffi::c_char) -> gaps_mask_t {
    if strcmp(scope, b"inner\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        return GAPS_INNER;
    }
    if strcmp(scope, b"outer\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        return GAPS_OUTER;
    }
    if strcmp(
        scope,
        b"vertical\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        return GAPS_VERTICAL;
    }
    if strcmp(
        scope,
        b"horizontal\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        return GAPS_HORIZONTAL;
    }
    if strcmp(scope, b"top\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        return GAPS_TOP;
    }
    if strcmp(scope, b"right\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        return GAPS_RIGHT;
    }
    if strcmp(
        scope,
        b"bottom\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        return GAPS_BOTTOM;
    }
    if strcmp(scope, b"left\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        return GAPS_LEFT;
    }
    errorlog(
        b"ERROR: Invalid command, cannot process scope %s\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        scope,
    );
    return 0 as gaps_mask_t;
}
#[no_mangle]
pub unsafe extern "C" fn cfg_gaps(
    mut current_match: *mut Match,
    mut result: *mut ConfigResultIR,
    mut workspace: *const ::core::ffi::c_char,
    mut scope: *const ::core::ffi::c_char,
    value: ::core::ffi::c_long,
) {
    let pixels: ::core::ffi::c_int = logical_px(value as ::core::ffi::c_int) as ::core::ffi::c_int;
    let mask: gaps_mask_t = gaps_scope_to_mask(scope) as gaps_mask_t;
    if workspace.is_null() {
        apply_gaps(&raw mut config.gaps, mask, pixels);
    } else {
        create_gaps_assignment(workspace, mask, pixels);
    };
}
#[no_mangle]
pub unsafe extern "C" fn cfg_smart_borders(
    mut current_match: *mut Match,
    mut result: *mut ConfigResultIR,
    mut enable: *const ::core::ffi::c_char,
) {
    if strcmp(
        enable,
        b"no_gaps\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        config.hide_edge_borders = HEBM_SMART_NO_GAPS;
    } else if boolstr(enable) {
        if config.hide_edge_borders as ::core::ffi::c_uint
            == HEBM_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            config.hide_edge_borders = HEBM_SMART;
        } else {
            errorlog(
                b"ERROR: Both hide_edge_borders and smart_borders was used. Ignoring smart_borders as it is deprecated.\n\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn cfg_smart_gaps(
    mut current_match: *mut Match,
    mut result: *mut ConfigResultIR,
    mut enable: *const ::core::ffi::c_char,
) {
    if strcmp(
        enable,
        b"inverse_outer\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        config.smart_gaps = SMART_GAPS_INVERSE_OUTER;
    } else {
        config.smart_gaps = (if boolstr(enable) as ::core::ffi::c_int != 0 {
            SMART_GAPS_ON as ::core::ffi::c_int
        } else {
            SMART_GAPS_OFF as ::core::ffi::c_int
        }) as smart_gaps_t;
    };
}
#[no_mangle]
pub unsafe extern "C" fn cfg_floating_minimum_size(
    mut current_match: *mut Match,
    mut result: *mut ConfigResultIR,
    width: ::core::ffi::c_long,
    height: ::core::ffi::c_long,
) {
    config.floating_minimum_width = width as int32_t;
    config.floating_minimum_height = height as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn cfg_floating_maximum_size(
    mut current_match: *mut Match,
    mut result: *mut ConfigResultIR,
    width: ::core::ffi::c_long,
    height: ::core::ffi::c_long,
) {
    config.floating_maximum_width = width as int32_t;
    config.floating_maximum_height = height as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn cfg_floating_modifier(
    mut current_match: *mut Match,
    mut result: *mut ConfigResultIR,
    mut modifiers: *const ::core::ffi::c_char,
) {
    config.floating_modifier = event_state_from_str(modifiers) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn cfg_tiling_drag_swap_modifier(
    mut current_match: *mut Match,
    mut result: *mut ConfigResultIR,
    mut modifiers: *const ::core::ffi::c_char,
) {
    config.swap_modifier = event_state_from_str(modifiers) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn cfg_default_orientation(
    mut current_match: *mut Match,
    mut result: *mut ConfigResultIR,
    mut orientation: *const ::core::ffi::c_char,
) {
    if strcmp(
        orientation,
        b"horizontal\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        config.default_orientation = HORIZ as ::core::ffi::c_int;
    } else if strcmp(
        orientation,
        b"vertical\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        config.default_orientation = VERT as ::core::ffi::c_int;
    } else {
        config.default_orientation = NO_ORIENTATION as ::core::ffi::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn cfg_workspace_layout(
    mut current_match: *mut Match,
    mut result: *mut ConfigResultIR,
    mut layout: *const ::core::ffi::c_char,
) {
    if strcmp(
        layout,
        b"default\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        config.default_layout = L_DEFAULT;
    } else if strcmp(
        layout,
        b"stacking\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
        || strcmp(
            layout,
            b"stacked\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
    {
        config.default_layout = L_STACKED;
    } else {
        config.default_layout = L_TABBED;
    };
}
#[no_mangle]
pub unsafe extern "C" fn cfg_default_border(
    mut current_match: *mut Match,
    mut result: *mut ConfigResultIR,
    mut windowtype: *const ::core::ffi::c_char,
    mut border: *const ::core::ffi::c_char,
    width: ::core::ffi::c_long,
) {
    let mut border_style: ::core::ffi::c_int = 0;
    let mut border_width: ::core::ffi::c_int = 0;
    if strcmp(
        border,
        b"1pixel\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        border_style = BS_PIXEL as ::core::ffi::c_int;
        border_width = 1 as ::core::ffi::c_int;
    } else if strcmp(border, b"none\0" as *const u8 as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        border_style = BS_NONE as ::core::ffi::c_int;
        border_width = 0 as ::core::ffi::c_int;
    } else if strcmp(
        border,
        b"pixel\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        border_style = BS_PIXEL as ::core::ffi::c_int;
        border_width = width as ::core::ffi::c_int;
    } else {
        border_style = BS_NORMAL as ::core::ffi::c_int;
        border_width = width as ::core::ffi::c_int;
    }
    if strcmp(
        windowtype,
        b"default_border\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
        || strcmp(
            windowtype,
            b"new_window\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
    {
        debuglog(
            b"%s:%s:%d - default tiled border style = %d and border width = %d (%d physical px)\n\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/config_directives.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"cfg_default_border\0" as *const u8 as *const ::core::ffi::c_char,
            416 as ::core::ffi::c_int,
            border_style,
            border_width,
            logical_px(border_width),
        );
        config.default_border = border_style as border_style_t;
        config.default_border_width = logical_px(border_width);
    } else {
        debuglog(
            b"%s:%s:%d - default floating border style = %d and border width = %d (%d physical px)\n\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/config_directives.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"cfg_default_border\0" as *const u8 as *const ::core::ffi::c_char,
            421 as ::core::ffi::c_int,
            border_style,
            border_width,
            logical_px(border_width),
        );
        config.default_floating_border = border_style as border_style_t;
        config.default_floating_border_width = logical_px(border_width);
    };
}
#[no_mangle]
pub unsafe extern "C" fn cfg_hide_edge_borders(
    mut current_match: *mut Match,
    mut result: *mut ConfigResultIR,
    mut borders: *const ::core::ffi::c_char,
) {
    if strcmp(
        borders,
        b"smart_no_gaps\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        config.hide_edge_borders = HEBM_SMART_NO_GAPS;
    } else if strcmp(
        borders,
        b"smart\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        config.hide_edge_borders = HEBM_SMART;
    } else if strcmp(
        borders,
        b"vertical\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        config.hide_edge_borders = HEBM_VERTICAL;
    } else if strcmp(
        borders,
        b"horizontal\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        config.hide_edge_borders = HEBM_HORIZONTAL;
    } else if strcmp(
        borders,
        b"both\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        config.hide_edge_borders = HEBM_BOTH;
    } else if strcmp(
        borders,
        b"none\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        config.hide_edge_borders = HEBM_NONE;
    } else if boolstr(borders) {
        config.hide_edge_borders = HEBM_VERTICAL;
    } else {
        config.hide_edge_borders = HEBM_NONE;
    };
}
#[no_mangle]
pub unsafe extern "C" fn cfg_focus_follows_mouse(
    mut current_match: *mut Match,
    mut result: *mut ConfigResultIR,
    mut value: *const ::core::ffi::c_char,
) {
    config.disable_focus_follows_mouse = !boolstr(value);
}
#[no_mangle]
pub unsafe extern "C" fn cfg_mouse_warping(
    mut current_match: *mut Match,
    mut result: *mut ConfigResultIR,
    mut value: *const ::core::ffi::c_char,
) {
    if strcmp(value, b"none\0" as *const u8 as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        config.mouse_warping = POINTER_WARPING_NONE;
    } else if strcmp(
        value,
        b"output\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        config.mouse_warping = POINTER_WARPING_OUTPUT;
    }
}
#[no_mangle]
pub unsafe extern "C" fn cfg_force_xinerama(
    mut current_match: *mut Match,
    mut result: *mut ConfigResultIR,
    mut value: *const ::core::ffi::c_char,
) {
    config.force_xinerama = boolstr(value);
}
#[no_mangle]
pub unsafe extern "C" fn cfg_disable_randr15(
    mut current_match: *mut Match,
    mut result: *mut ConfigResultIR,
    mut value: *const ::core::ffi::c_char,
) {
    config.disable_randr15 = boolstr(value);
}
#[no_mangle]
pub unsafe extern "C" fn cfg_focus_wrapping(
    mut current_match: *mut Match,
    mut result: *mut ConfigResultIR,
    mut value: *const ::core::ffi::c_char,
) {
    if strcmp(value, b"force\0" as *const u8 as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        config.focus_wrapping = FOCUS_WRAPPING_FORCE;
    } else if strcmp(
        value,
        b"workspace\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        config.focus_wrapping = FOCUS_WRAPPING_WORKSPACE;
    } else if boolstr(value) {
        config.focus_wrapping = FOCUS_WRAPPING_ON;
    } else {
        config.focus_wrapping = FOCUS_WRAPPING_OFF;
    };
}
#[no_mangle]
pub unsafe extern "C" fn cfg_force_focus_wrapping(
    mut current_match: *mut Match,
    mut result: *mut ConfigResultIR,
    mut value: *const ::core::ffi::c_char,
) {
    if boolstr(value) {
        config.focus_wrapping = FOCUS_WRAPPING_FORCE;
    } else if config.focus_wrapping as ::core::ffi::c_uint
        == FOCUS_WRAPPING_FORCE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        config.focus_wrapping = FOCUS_WRAPPING_ON;
    }
}
#[no_mangle]
pub unsafe extern "C" fn cfg_workspace_back_and_forth(
    mut current_match: *mut Match,
    mut result: *mut ConfigResultIR,
    mut value: *const ::core::ffi::c_char,
) {
    config.workspace_auto_back_and_forth = boolstr(value);
}
#[no_mangle]
pub unsafe extern "C" fn cfg_fake_outputs(
    mut current_match: *mut Match,
    mut result: *mut ConfigResultIR,
    mut outputs: *const ::core::ffi::c_char,
) {
    free(config.fake_outputs as *mut ::core::ffi::c_void);
    config.fake_outputs = sstrdup(outputs);
}
#[no_mangle]
pub unsafe extern "C" fn cfg_force_display_urgency_hint(
    mut current_match: *mut Match,
    mut result: *mut ConfigResultIR,
    duration_ms: ::core::ffi::c_long,
) {
    config.workspace_urgency_timer =
        (duration_ms as ::core::ffi::c_double / 1000.0f64) as ::core::ffi::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn cfg_focus_on_window_activation(
    mut current_match: *mut Match,
    mut result: *mut ConfigResultIR,
    mut mode: *const ::core::ffi::c_char,
) {
    if strcmp(mode, b"smart\0" as *const u8 as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        config.focus_on_window_activation = FOWA_SMART;
    } else if strcmp(mode, b"urgent\0" as *const u8 as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        config.focus_on_window_activation = FOWA_URGENT;
    } else if strcmp(mode, b"focus\0" as *const u8 as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        config.focus_on_window_activation = FOWA_FOCUS;
    } else if strcmp(mode, b"none\0" as *const u8 as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        config.focus_on_window_activation = FOWA_NONE;
    } else {
        errorlog(
            b"ERROR: Unknown focus_on_window_activation mode \"%s\", ignoring it.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            mode,
        );
        return;
    }
    debuglog(
        b"%s:%s:%d - Set new focus_on_window_activation mode = %i.\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/config_directives.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"cfg_focus_on_window_activation\0" as *const u8 as *const ::core::ffi::c_char,
        519 as ::core::ffi::c_int,
        config.focus_on_window_activation as ::core::ffi::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cfg_title_align(
    mut current_match: *mut Match,
    mut result: *mut ConfigResultIR,
    mut alignment: *const ::core::ffi::c_char,
) {
    if strcmp(
        alignment,
        b"left\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        config.title_align = ALIGN_LEFT;
    } else if strcmp(
        alignment,
        b"center\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        config.title_align = ALIGN_CENTER;
    } else if strcmp(
        alignment,
        b"right\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        config.title_align = ALIGN_RIGHT;
    } else {
        if 0 as ::core::ffi::c_int != 0 {
        } else {
            __assert_fail(
                b"false\0" as *const u8 as *const ::core::ffi::c_char,
                b"../src/config_directives.c\0" as *const u8 as *const ::core::ffi::c_char,
                530 as ::core::ffi::c_uint,
                b"void cfg_title_align(Match *, struct ConfigResultIR *, const char *)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        };
    };
}
#[no_mangle]
pub unsafe extern "C" fn cfg_show_marks(
    mut current_match: *mut Match,
    mut result: *mut ConfigResultIR,
    mut value: *const ::core::ffi::c_char,
) {
    config.show_marks = boolstr(value);
}
static mut current_workspace: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]
pub unsafe extern "C" fn cfg_workspace(
    mut current_match: *mut Match,
    mut result: *mut ConfigResultIR,
    mut workspace: *const ::core::ffi::c_char,
    mut output: *const ::core::ffi::c_char,
) {
    let mut assignment: *mut Workspace_Assignment = ::core::ptr::null_mut::<Workspace_Assignment>();
    if !workspace.is_null() {
        free(current_workspace as *mut ::core::ffi::c_void);
        current_workspace = ::core::ptr::null_mut::<::core::ffi::c_char>();
        assignment = ws_assignments.tqh_first;
        while !assignment.is_null() {
            if strcasecmp((*assignment).name, workspace) == 0 as ::core::ffi::c_int {
                if !(*assignment).output.is_null() {
                    errorlog(
                        b"ERROR: You have a duplicate workspace assignment for workspace \"%s\"\n\0"
                            as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        workspace,
                    );
                    return;
                }
            }
            assignment = (*assignment).ws_assignments.tqe_next;
        }
        current_workspace = sstrdup(workspace);
    } else {
        if current_workspace.is_null() {
            debuglog(
                b"%s:%s:%d - Both workspace and current_workspace are NULL, assuming we had an error before\n\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"../src/config_directives.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"cfg_workspace\0" as *const u8 as *const ::core::ffi::c_char,
                563 as ::core::ffi::c_int,
            );
            return;
        }
        workspace = current_workspace;
    }
    debuglog(
        b"%s:%s:%d - Assigning workspace \"%s\" to output \"%s\"\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/config_directives.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"cfg_workspace\0" as *const u8 as *const ::core::ffi::c_char,
        569 as ::core::ffi::c_int,
        workspace,
        output,
    );
    assignment = scalloc(
        1 as size_t,
        ::core::mem::size_of::<Workspace_Assignment>() as size_t,
    ) as *mut Workspace_Assignment;
    (*assignment).name = sstrdup(workspace);
    (*assignment).output = sstrdup(output);
    (*assignment).ws_assignments.tqe_next = ::core::ptr::null_mut::<Workspace_Assignment>();
    (*assignment).ws_assignments.tqe_prev = ws_assignments.tqh_last;
    *ws_assignments.tqh_last = assignment;
    ws_assignments.tqh_last = &raw mut (*assignment).ws_assignments.tqe_next;
}
#[no_mangle]
pub unsafe extern "C" fn cfg_ipc_socket(
    mut current_match: *mut Match,
    mut result: *mut ConfigResultIR,
    mut path: *const ::core::ffi::c_char,
) {
    free(config.ipc_socket_path as *mut ::core::ffi::c_void);
    config.ipc_socket_path = sstrdup(path);
}
#[no_mangle]
pub unsafe extern "C" fn cfg_restart_state(
    mut current_match: *mut Match,
    mut result: *mut ConfigResultIR,
    mut path: *const ::core::ffi::c_char,
) {
    free(config.restart_state_path as *mut ::core::ffi::c_void);
    config.restart_state_path = sstrdup(path);
}
#[no_mangle]
pub unsafe extern "C" fn cfg_popup_during_fullscreen(
    mut current_match: *mut Match,
    mut result: *mut ConfigResultIR,
    mut value: *const ::core::ffi::c_char,
) {
    if strcmp(
        value,
        b"ignore\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        config.popup_during_fullscreen = PDF_IGNORE;
    } else if strcmp(
        value,
        b"leave_fullscreen\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        config.popup_during_fullscreen = PDF_LEAVE_FULLSCREEN;
    } else if strcmp(value, b"all\0" as *const u8 as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        config.popup_during_fullscreen = PDF_ALL;
    } else {
        config.popup_during_fullscreen = PDF_SMART;
    };
}
#[no_mangle]
pub unsafe extern "C" fn cfg_color_single(
    mut current_match: *mut Match,
    mut result: *mut ConfigResultIR,
    mut colorclass: *const ::core::ffi::c_char,
    mut color: *const ::core::ffi::c_char,
) {
    config.client.background = draw_util_hex_to_color(color);
}
#[no_mangle]
pub unsafe extern "C" fn cfg_color(
    mut current_match: *mut Match,
    mut result: *mut ConfigResultIR,
    mut colorclass: *const ::core::ffi::c_char,
    mut border: *const ::core::ffi::c_char,
    mut background: *const ::core::ffi::c_char,
    mut text: *const ::core::ffi::c_char,
    mut indicator: *const ::core::ffi::c_char,
    mut child_border: *const ::core::ffi::c_char,
) {
    if strcmp(
        colorclass,
        b"client.focused_inactive\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        if strcmp(
            b"focused_tab_title\0" as *const u8 as *const ::core::ffi::c_char,
            b"focused_inactive\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            config.client.got_focused_tab_title = true_0 != 0;
            if !indicator.is_null() || !child_border.is_null() {
                errorlog(
                    b"ERROR: indicator and child_border colors have no effect for client.focused_tab_title\n\0"
                        as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            }
        }
        config.client.focused_inactive.border = draw_util_hex_to_color(border);
        config.client.focused_inactive.background = draw_util_hex_to_color(background);
        config.client.focused_inactive.text = draw_util_hex_to_color(text);
        if !indicator.is_null() {
            config.client.focused_inactive.indicator = draw_util_hex_to_color(indicator);
        }
        if !child_border.is_null() {
            config.client.focused_inactive.child_border = draw_util_hex_to_color(child_border);
        } else {
            config.client.focused_inactive.child_border = config.client.focused_inactive.background;
        }
        return;
    }
    if strcmp(
        colorclass,
        b"client.focused_tab_title\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        if strcmp(
            b"focused_tab_title\0" as *const u8 as *const ::core::ffi::c_char,
            b"focused_tab_title\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            config.client.got_focused_tab_title = true_0 != 0;
            if !indicator.is_null() || !child_border.is_null() {
                errorlog(
                    b"ERROR: indicator and child_border colors have no effect for client.focused_tab_title\n\0"
                        as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            }
        }
        config.client.focused_tab_title.border = draw_util_hex_to_color(border);
        config.client.focused_tab_title.background = draw_util_hex_to_color(background);
        config.client.focused_tab_title.text = draw_util_hex_to_color(text);
        if !indicator.is_null() {
            config.client.focused_tab_title.indicator = draw_util_hex_to_color(indicator);
        }
        if !child_border.is_null() {
            config.client.focused_tab_title.child_border = draw_util_hex_to_color(child_border);
        } else {
            config.client.focused_tab_title.child_border =
                config.client.focused_tab_title.background;
        }
        return;
    }
    if strcmp(
        colorclass,
        b"client.focused\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        if strcmp(
            b"focused_tab_title\0" as *const u8 as *const ::core::ffi::c_char,
            b"focused\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            config.client.got_focused_tab_title = true_0 != 0;
            if !indicator.is_null() || !child_border.is_null() {
                errorlog(
                    b"ERROR: indicator and child_border colors have no effect for client.focused_tab_title\n\0"
                        as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            }
        }
        config.client.focused.border = draw_util_hex_to_color(border);
        config.client.focused.background = draw_util_hex_to_color(background);
        config.client.focused.text = draw_util_hex_to_color(text);
        if !indicator.is_null() {
            config.client.focused.indicator = draw_util_hex_to_color(indicator);
        }
        if !child_border.is_null() {
            config.client.focused.child_border = draw_util_hex_to_color(child_border);
        } else {
            config.client.focused.child_border = config.client.focused.background;
        }
        return;
    }
    if strcmp(
        colorclass,
        b"client.unfocused\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        if strcmp(
            b"focused_tab_title\0" as *const u8 as *const ::core::ffi::c_char,
            b"unfocused\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            config.client.got_focused_tab_title = true_0 != 0;
            if !indicator.is_null() || !child_border.is_null() {
                errorlog(
                    b"ERROR: indicator and child_border colors have no effect for client.focused_tab_title\n\0"
                        as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            }
        }
        config.client.unfocused.border = draw_util_hex_to_color(border);
        config.client.unfocused.background = draw_util_hex_to_color(background);
        config.client.unfocused.text = draw_util_hex_to_color(text);
        if !indicator.is_null() {
            config.client.unfocused.indicator = draw_util_hex_to_color(indicator);
        }
        if !child_border.is_null() {
            config.client.unfocused.child_border = draw_util_hex_to_color(child_border);
        } else {
            config.client.unfocused.child_border = config.client.unfocused.background;
        }
        return;
    }
    if strcmp(
        colorclass,
        b"client.urgent\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        if strcmp(
            b"focused_tab_title\0" as *const u8 as *const ::core::ffi::c_char,
            b"urgent\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            config.client.got_focused_tab_title = true_0 != 0;
            if !indicator.is_null() || !child_border.is_null() {
                errorlog(
                    b"ERROR: indicator and child_border colors have no effect for client.focused_tab_title\n\0"
                        as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            }
        }
        config.client.urgent.border = draw_util_hex_to_color(border);
        config.client.urgent.background = draw_util_hex_to_color(background);
        config.client.urgent.text = draw_util_hex_to_color(text);
        if !indicator.is_null() {
            config.client.urgent.indicator = draw_util_hex_to_color(indicator);
        }
        if !child_border.is_null() {
            config.client.urgent.child_border = draw_util_hex_to_color(child_border);
        } else {
            config.client.urgent.child_border = config.client.urgent.background;
        }
        return;
    }
    if strcmp(
        colorclass,
        b"client.placeholder\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        if strcmp(
            b"focused_tab_title\0" as *const u8 as *const ::core::ffi::c_char,
            b"placeholder\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            config.client.got_focused_tab_title = true_0 != 0;
            if !indicator.is_null() || !child_border.is_null() {
                errorlog(
                    b"ERROR: indicator and child_border colors have no effect for client.focused_tab_title\n\0"
                        as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            }
        }
        config.client.placeholder.border = draw_util_hex_to_color(border);
        config.client.placeholder.background = draw_util_hex_to_color(background);
        config.client.placeholder.text = draw_util_hex_to_color(text);
        if !indicator.is_null() {
            config.client.placeholder.indicator = draw_util_hex_to_color(indicator);
        }
        if !child_border.is_null() {
            config.client.placeholder.child_border = draw_util_hex_to_color(child_border);
        } else {
            config.client.placeholder.child_border = config.client.placeholder.background;
        }
        return;
    }
}
#[no_mangle]
pub unsafe extern "C" fn cfg_assign_output(
    mut current_match: *mut Match,
    mut result: *mut ConfigResultIR,
    mut output: *const ::core::ffi::c_char,
) {
    if !(*current_match).error.is_null() {
        errorlog(
            b"ERROR: match has error: %s\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            (*current_match).error,
        );
        return;
    }
    if match_is_empty(current_match) {
        errorlog(
            b"ERROR: Match is empty, ignoring this assignment\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        return;
    }
    if (*current_match).window_mode as ::core::ffi::c_uint
        != WM_ANY as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        errorlog(
            b"ERROR: Assignments using window mode (floating/tiling) is not supported\n\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        return;
    }
    debuglog(
        b"%s:%s:%d - New assignment, using above criteria, to output \"%s\".\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/config_directives.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"cfg_assign_output\0" as *const u8 as *const ::core::ffi::c_char,
        654 as ::core::ffi::c_int,
        output,
    );
    let mut assignment: *mut Assignment =
        scalloc(1 as size_t, ::core::mem::size_of::<Assignment>() as size_t) as *mut Assignment;
    match_copy(&raw mut (*assignment).match_0, current_match);
    (*assignment).type_0 = A_TO_OUTPUT;
    (*assignment).dest.output = sstrdup(output);
    (*assignment).assignments.tqe_next = ::core::ptr::null_mut::<Assignment>();
    (*assignment).assignments.tqe_prev = assignments.tqh_last;
    *assignments.tqh_last = assignment as *mut Assignment;
    assignments.tqh_last = &raw mut (*assignment).assignments.tqe_next;
}
#[no_mangle]
pub unsafe extern "C" fn cfg_assign(
    mut current_match: *mut Match,
    mut result: *mut ConfigResultIR,
    mut workspace: *const ::core::ffi::c_char,
    is_number: bool,
) {
    if !(*current_match).error.is_null() {
        errorlog(
            b"ERROR: match has error: %s\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            (*current_match).error,
        );
        return;
    }
    if match_is_empty(current_match) {
        errorlog(
            b"ERROR: Match is empty, ignoring this assignment\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        return;
    }
    if (*current_match).window_mode as ::core::ffi::c_uint
        != WM_ANY as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        errorlog(
            b"ERROR: Assignments using window mode (floating/tiling) is not supported\n\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        return;
    }
    if is_number as ::core::ffi::c_int != 0
        && ws_name_to_number(workspace) == -(1 as ::core::ffi::c_int)
    {
        errorlog(
            b"ERROR: Could not parse initial part of \"%s\" as a number.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            workspace,
        );
        return;
    }
    debuglog(
        b"%s:%s:%d - New assignment, using above criteria, to workspace \"%s\".\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/config_directives.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"cfg_assign\0" as *const u8 as *const ::core::ffi::c_char,
        682 as ::core::ffi::c_int,
        workspace,
    );
    let mut assignment: *mut Assignment =
        scalloc(1 as size_t, ::core::mem::size_of::<Assignment>() as size_t) as *mut Assignment;
    match_copy(&raw mut (*assignment).match_0, current_match);
    (*assignment).type_0 = (if is_number as ::core::ffi::c_int != 0 {
        A_TO_WORKSPACE_NUMBER as ::core::ffi::c_int
    } else {
        A_TO_WORKSPACE as ::core::ffi::c_int
    }) as C2RustUnnamed_19;
    (*assignment).dest.workspace = sstrdup(workspace);
    (*assignment).assignments.tqe_next = ::core::ptr::null_mut::<Assignment>();
    (*assignment).assignments.tqe_prev = assignments.tqh_last;
    *assignments.tqh_last = assignment as *mut Assignment;
    assignments.tqh_last = &raw mut (*assignment).assignments.tqe_next;
}
#[no_mangle]
pub unsafe extern "C" fn cfg_no_focus(
    mut current_match: *mut Match,
    mut result: *mut ConfigResultIR,
) {
    if !(*current_match).error.is_null() {
        errorlog(
            b"ERROR: match has error: %s\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            (*current_match).error,
        );
        return;
    }
    if match_is_empty(current_match) {
        errorlog(
            b"ERROR: Match is empty, ignoring this assignment\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        return;
    }
    debuglog(
        b"%s:%s:%d - New assignment, using above criteria, to ignore focus on manage.\n\0"
            as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/config_directives.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"cfg_no_focus\0" as *const u8 as *const ::core::ffi::c_char,
        700 as ::core::ffi::c_int,
    );
    let mut assignment: *mut Assignment =
        scalloc(1 as size_t, ::core::mem::size_of::<Assignment>() as size_t) as *mut Assignment;
    match_copy(&raw mut (*assignment).match_0, current_match);
    (*assignment).type_0 = A_NO_FOCUS;
    (*assignment).assignments.tqe_next = ::core::ptr::null_mut::<Assignment>();
    (*assignment).assignments.tqe_prev = assignments.tqh_last;
    *assignments.tqh_last = assignment as *mut Assignment;
    assignments.tqh_last = &raw mut (*assignment).assignments.tqe_next;
}
#[no_mangle]
pub unsafe extern "C" fn cfg_ipc_kill_timeout(
    mut current_match: *mut Match,
    mut result: *mut ConfigResultIR,
    timeout_ms: ::core::ffi::c_long,
) {
    ipc_set_kill_timeout(timeout_ms as ev_tstamp / 1000.0f64);
}
#[no_mangle]
pub unsafe extern "C" fn cfg_tiling_drag(
    mut current_match: *mut Match,
    mut result: *mut ConfigResultIR,
    mut value: *const ::core::ffi::c_char,
) {
    if strcmp(
        value,
        b"modifier\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        config.tiling_drag = TILING_DRAG_MODIFIER;
    } else if strcmp(
        value,
        b"titlebar\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        config.tiling_drag = TILING_DRAG_TITLEBAR;
    } else if strcmp(
        value,
        b"modifier,titlebar\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
        || strcmp(
            value,
            b"titlebar,modifier\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
    {
        config.tiling_drag = TILING_DRAG_MODIFIER_OR_TITLEBAR;
    } else {
        config.tiling_drag = TILING_DRAG_OFF;
    };
}
static mut current_bar: *mut Barconfig = ::core::ptr::null::<Barconfig>() as *mut Barconfig;
#[no_mangle]
pub unsafe extern "C" fn cfg_bar_font(
    mut current_match: *mut Match,
    mut result: *mut ConfigResultIR,
    mut font: *const ::core::ffi::c_char,
) {
    free((*current_bar).font as *mut ::core::ffi::c_void);
    (*current_bar).font = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*current_bar).font = sstrdup(font);
}
#[no_mangle]
pub unsafe extern "C" fn cfg_bar_separator_symbol(
    mut current_match: *mut Match,
    mut result: *mut ConfigResultIR,
    mut separator: *const ::core::ffi::c_char,
) {
    free((*current_bar).separator_symbol as *mut ::core::ffi::c_void);
    (*current_bar).separator_symbol = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*current_bar).separator_symbol = sstrdup(separator);
}
#[no_mangle]
pub unsafe extern "C" fn cfg_bar_mode(
    mut current_match: *mut Match,
    mut result: *mut ConfigResultIR,
    mut mode: *const ::core::ffi::c_char,
) {
    (*current_bar).mode = (if strcmp(mode, b"dock\0" as *const u8 as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        M_DOCK as ::core::ffi::c_int
    } else if strcmp(mode, b"hide\0" as *const u8 as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        M_HIDE as ::core::ffi::c_int
    } else {
        M_INVISIBLE as ::core::ffi::c_int
    }) as C2RustUnnamed_35;
}
#[no_mangle]
pub unsafe extern "C" fn cfg_bar_hidden_state(
    mut current_match: *mut Match,
    mut result: *mut ConfigResultIR,
    mut hidden_state: *const ::core::ffi::c_char,
) {
    (*current_bar).hidden_state = (if strcmp(
        hidden_state,
        b"hide\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        S_HIDE as ::core::ffi::c_int
    } else {
        S_SHOW as ::core::ffi::c_int
    }) as C2RustUnnamed_34;
}
#[no_mangle]
pub unsafe extern "C" fn cfg_bar_id(
    mut current_match: *mut Match,
    mut result: *mut ConfigResultIR,
    mut bar_id: *const ::core::ffi::c_char,
) {
    (*current_bar).id = sstrdup(bar_id);
}
#[no_mangle]
pub unsafe extern "C" fn cfg_bar_output(
    mut current_match: *mut Match,
    mut result: *mut ConfigResultIR,
    mut output: *const ::core::ffi::c_char,
) {
    let new_outputs: ::core::ffi::c_int = (*current_bar).num_outputs + 1 as ::core::ffi::c_int;
    (*current_bar).outputs = srealloc(
        (*current_bar).outputs as *mut ::core::ffi::c_void,
        (::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t)
            .wrapping_mul(new_outputs as size_t),
    ) as *mut *mut ::core::ffi::c_char;
    let ref mut fresh0 = *(*current_bar)
        .outputs
        .offset((*current_bar).num_outputs as isize);
    *fresh0 = sstrdup(output);
    (*current_bar).num_outputs = new_outputs;
}
#[no_mangle]
pub unsafe extern "C" fn cfg_bar_verbose(
    mut current_match: *mut Match,
    mut result: *mut ConfigResultIR,
    mut verbose: *const ::core::ffi::c_char,
) {
    (*current_bar).verbose = boolstr(verbose);
}
#[no_mangle]
pub unsafe extern "C" fn cfg_bar_height(
    mut current_match: *mut Match,
    mut result: *mut ConfigResultIR,
    height: ::core::ffi::c_long,
) {
    (*current_bar).bar_height = height as uint32_t;
}
unsafe extern "C" fn dlog_padding() {
    debuglog(
        b"%s:%s:%d - padding now: x=%d, y=%d, w=%d, h=%d\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/config_directives.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"dlog_padding\0" as *const u8 as *const ::core::ffi::c_char,
        773 as ::core::ffi::c_int,
        (*current_bar).padding.x,
        (*current_bar).padding.y,
        (*current_bar).padding.width,
        (*current_bar).padding.height,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cfg_bar_padding_one(
    mut current_match: *mut Match,
    mut result: *mut ConfigResultIR,
    all: ::core::ffi::c_long,
) {
    (*current_bar).padding.x = all as uint32_t;
    (*current_bar).padding.y = all as uint32_t;
    (*current_bar).padding.width = all as uint32_t;
    (*current_bar).padding.height = all as uint32_t;
    dlog_padding();
}
#[no_mangle]
pub unsafe extern "C" fn cfg_bar_padding_two(
    mut current_match: *mut Match,
    mut result: *mut ConfigResultIR,
    top_and_bottom: ::core::ffi::c_long,
    right_and_left: ::core::ffi::c_long,
) {
    (*current_bar).padding.x = right_and_left as uint32_t;
    (*current_bar).padding.y = top_and_bottom as uint32_t;
    (*current_bar).padding.width = right_and_left as uint32_t;
    (*current_bar).padding.height = top_and_bottom as uint32_t;
    dlog_padding();
}
#[no_mangle]
pub unsafe extern "C" fn cfg_bar_padding_three(
    mut current_match: *mut Match,
    mut result: *mut ConfigResultIR,
    top: ::core::ffi::c_long,
    right_and_left: ::core::ffi::c_long,
    bottom: ::core::ffi::c_long,
) {
    (*current_bar).padding.x = right_and_left as uint32_t;
    (*current_bar).padding.y = top as uint32_t;
    (*current_bar).padding.width = right_and_left as uint32_t;
    (*current_bar).padding.height = bottom as uint32_t;
    dlog_padding();
}
#[no_mangle]
pub unsafe extern "C" fn cfg_bar_padding_four(
    mut current_match: *mut Match,
    mut result: *mut ConfigResultIR,
    top: ::core::ffi::c_long,
    right: ::core::ffi::c_long,
    bottom: ::core::ffi::c_long,
    left: ::core::ffi::c_long,
) {
    (*current_bar).padding.x = left as uint32_t;
    (*current_bar).padding.y = top as uint32_t;
    (*current_bar).padding.width = right as uint32_t;
    (*current_bar).padding.height = bottom as uint32_t;
    dlog_padding();
}
#[no_mangle]
pub unsafe extern "C" fn cfg_bar_modifier(
    mut current_match: *mut Match,
    mut result: *mut ConfigResultIR,
    mut modifiers: *const ::core::ffi::c_char,
) {
    (*current_bar).modifier = (if !modifiers.is_null() {
        event_state_from_str(modifiers) as ::core::ffi::c_long
    } else {
        XCB_NONE
    }) as uint32_t;
}
unsafe extern "C" fn bar_configure_binding(
    mut button: *const ::core::ffi::c_char,
    mut release: *const ::core::ffi::c_char,
    mut command: *const ::core::ffi::c_char,
) {
    if strncasecmp(
        button,
        b"button\0" as *const u8 as *const ::core::ffi::c_char,
        strlen(b"button\0" as *const u8 as *const ::core::ffi::c_char),
    ) != 0 as ::core::ffi::c_int
    {
        errorlog(
            b"ERROR: Bindings for a bar can only be mouse bindings, not \"%s\", ignoring.\n\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            button,
        );
        return;
    }
    let input_code: ::core::ffi::c_int = atoi(
        button.offset(strlen(b"button\0" as *const u8 as *const ::core::ffi::c_char) as isize),
    ) as ::core::ffi::c_int;
    if input_code < 1 as ::core::ffi::c_int {
        errorlog(
            b"ERROR: Button \"%s\" does not seem to be in format 'buttonX'.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            button,
        );
        return;
    }
    let release_bool: bool = !release.is_null();
    let mut current: *mut Barbinding = ::core::ptr::null_mut::<Barbinding>();
    current = (*current_bar).bar_bindings.tqh_first as *mut Barbinding;
    while !current.is_null() {
        if (*current).input_code == input_code
            && (*current).release as ::core::ffi::c_int == release_bool as ::core::ffi::c_int
        {
            errorlog(
                b"ERROR: command for button %s was already specified, ignoring.\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                button,
            );
            return;
        }
        current = (*current).bindings.tqe_next;
    }
    let mut new_binding: *mut Barbinding =
        scalloc(1 as size_t, ::core::mem::size_of::<Barbinding>() as size_t) as *mut Barbinding;
    (*new_binding).release = release_bool;
    (*new_binding).input_code = input_code;
    (*new_binding).command = sstrdup(command);
    (*new_binding).bindings.tqe_next = ::core::ptr::null_mut::<Barbinding>();
    (*new_binding).bindings.tqe_prev = (*current_bar).bar_bindings.tqh_last;
    *(*current_bar).bar_bindings.tqh_last = new_binding;
    (*current_bar).bar_bindings.tqh_last = &raw mut (*new_binding).bindings.tqe_next;
}
#[no_mangle]
pub unsafe extern "C" fn cfg_bar_wheel_up_cmd(
    mut current_match: *mut Match,
    mut result: *mut ConfigResultIR,
    mut command: *const ::core::ffi::c_char,
) {
    errorlog(
        b"ERROR: 'wheel_up_cmd' is deprecated. Please use 'bindsym button4 %s' instead.\n\0"
            as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        command,
    );
    bar_configure_binding(
        b"button4\0" as *const u8 as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
        command,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cfg_bar_wheel_down_cmd(
    mut current_match: *mut Match,
    mut result: *mut ConfigResultIR,
    mut command: *const ::core::ffi::c_char,
) {
    errorlog(
        b"ERROR: 'wheel_down_cmd' is deprecated. Please use 'bindsym button5 %s' instead.\n\0"
            as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        command,
    );
    bar_configure_binding(
        b"button5\0" as *const u8 as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
        command,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cfg_bar_bindsym(
    mut current_match: *mut Match,
    mut result: *mut ConfigResultIR,
    mut button: *const ::core::ffi::c_char,
    mut release: *const ::core::ffi::c_char,
    mut command: *const ::core::ffi::c_char,
) {
    bar_configure_binding(button, release, command);
}
#[no_mangle]
pub unsafe extern "C" fn cfg_bar_position(
    mut current_match: *mut Match,
    mut result: *mut ConfigResultIR,
    mut position: *const ::core::ffi::c_char,
) {
    (*current_bar).position = (if strcmp(
        position,
        b"top\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        P_TOP as ::core::ffi::c_int
    } else {
        P_BOTTOM as ::core::ffi::c_int
    }) as C2RustUnnamed_32;
}
#[no_mangle]
pub unsafe extern "C" fn cfg_bar_i3bar_command(
    mut current_match: *mut Match,
    mut result: *mut ConfigResultIR,
    mut i3bar_command: *const ::core::ffi::c_char,
) {
    free((*current_bar).i3bar_command as *mut ::core::ffi::c_void);
    (*current_bar).i3bar_command = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*current_bar).i3bar_command = sstrdup(i3bar_command);
}
#[no_mangle]
pub unsafe extern "C" fn cfg_bar_color(
    mut current_match: *mut Match,
    mut result: *mut ConfigResultIR,
    mut colorclass: *const ::core::ffi::c_char,
    mut border: *const ::core::ffi::c_char,
    mut background: *const ::core::ffi::c_char,
    mut text: *const ::core::ffi::c_char,
) {
    if strcmp(
        colorclass,
        b"focused_workspace\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        if !text.is_null() {
            (*current_bar).colors.focused_workspace_border = sstrdup(border);
            (*current_bar).colors.focused_workspace_bg = sstrdup(background);
            (*current_bar).colors.focused_workspace_text = sstrdup(text);
        } else {
            (*current_bar).colors.focused_workspace_bg = sstrdup(background);
            (*current_bar).colors.focused_workspace_text = sstrdup(border);
        }
    }
    if strcmp(
        colorclass,
        b"active_workspace\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        if !text.is_null() {
            (*current_bar).colors.active_workspace_border = sstrdup(border);
            (*current_bar).colors.active_workspace_bg = sstrdup(background);
            (*current_bar).colors.active_workspace_text = sstrdup(text);
        } else {
            (*current_bar).colors.active_workspace_bg = sstrdup(background);
            (*current_bar).colors.active_workspace_text = sstrdup(border);
        }
    }
    if strcmp(
        colorclass,
        b"inactive_workspace\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        if !text.is_null() {
            (*current_bar).colors.inactive_workspace_border = sstrdup(border);
            (*current_bar).colors.inactive_workspace_bg = sstrdup(background);
            (*current_bar).colors.inactive_workspace_text = sstrdup(text);
        } else {
            (*current_bar).colors.inactive_workspace_bg = sstrdup(background);
            (*current_bar).colors.inactive_workspace_text = sstrdup(border);
        }
    }
    if strcmp(
        colorclass,
        b"urgent_workspace\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        if !text.is_null() {
            (*current_bar).colors.urgent_workspace_border = sstrdup(border);
            (*current_bar).colors.urgent_workspace_bg = sstrdup(background);
            (*current_bar).colors.urgent_workspace_text = sstrdup(text);
        } else {
            (*current_bar).colors.urgent_workspace_bg = sstrdup(background);
            (*current_bar).colors.urgent_workspace_text = sstrdup(border);
        }
    }
    if strcmp(
        colorclass,
        b"binding_mode\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        if !text.is_null() {
            (*current_bar).colors.binding_mode_border = sstrdup(border);
            (*current_bar).colors.binding_mode_bg = sstrdup(background);
            (*current_bar).colors.binding_mode_text = sstrdup(text);
        } else {
            (*current_bar).colors.binding_mode_bg = sstrdup(background);
            (*current_bar).colors.binding_mode_text = sstrdup(border);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn cfg_bar_socket_path(
    mut current_match: *mut Match,
    mut result: *mut ConfigResultIR,
    mut socket_path: *const ::core::ffi::c_char,
) {
    free((*current_bar).socket_path as *mut ::core::ffi::c_void);
    (*current_bar).socket_path = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*current_bar).socket_path = sstrdup(socket_path);
}
#[no_mangle]
pub unsafe extern "C" fn cfg_bar_tray_output(
    mut current_match: *mut Match,
    mut result: *mut ConfigResultIR,
    mut output: *const ::core::ffi::c_char,
) {
    let mut tray_output: *mut tray_output_t = scalloc(
        1 as size_t,
        ::core::mem::size_of::<tray_output_t>() as size_t,
    ) as *mut tray_output_t;
    (*tray_output).output = sstrdup(output);
    (*tray_output).tray_outputs.tqe_next = ::core::ptr::null_mut::<tray_output_t>();
    (*tray_output).tray_outputs.tqe_prev = (*current_bar).tray_outputs.tqh_last;
    *(*current_bar).tray_outputs.tqh_last = tray_output;
    (*current_bar).tray_outputs.tqh_last = &raw mut (*tray_output).tray_outputs.tqe_next;
}
#[no_mangle]
pub unsafe extern "C" fn cfg_bar_tray_padding(
    mut current_match: *mut Match,
    mut result: *mut ConfigResultIR,
    padding_px: ::core::ffi::c_long,
) {
    (*current_bar).tray_padding = padding_px as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn cfg_bar_color_single(
    mut current_match: *mut Match,
    mut result: *mut ConfigResultIR,
    mut colorclass: *const ::core::ffi::c_char,
    mut color: *const ::core::ffi::c_char,
) {
    if strcmp(
        colorclass,
        b"background\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        (*current_bar).colors.background = sstrdup(color);
    } else if strcmp(
        colorclass,
        b"separator\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        (*current_bar).colors.separator = sstrdup(color);
    } else if strcmp(
        colorclass,
        b"statusline\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        (*current_bar).colors.statusline = sstrdup(color);
    } else if strcmp(
        colorclass,
        b"focused_background\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        (*current_bar).colors.focused_background = sstrdup(color);
    } else if strcmp(
        colorclass,
        b"focused_separator\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        (*current_bar).colors.focused_separator = sstrdup(color);
    } else {
        (*current_bar).colors.focused_statusline = sstrdup(color);
    };
}
#[no_mangle]
pub unsafe extern "C" fn cfg_bar_status_command(
    mut current_match: *mut Match,
    mut result: *mut ConfigResultIR,
    mut command: *const ::core::ffi::c_char,
) {
    free((*current_bar).status_command as *mut ::core::ffi::c_void);
    (*current_bar).status_command = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*current_bar).status_command = sstrdup(command);
}
#[no_mangle]
pub unsafe extern "C" fn cfg_bar_workspace_command(
    mut current_match: *mut Match,
    mut result: *mut ConfigResultIR,
    mut command: *const ::core::ffi::c_char,
) {
    free((*current_bar).workspace_command as *mut ::core::ffi::c_void);
    (*current_bar).workspace_command = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*current_bar).workspace_command = sstrdup(command);
}
#[no_mangle]
pub unsafe extern "C" fn cfg_bar_binding_mode_indicator(
    mut current_match: *mut Match,
    mut result: *mut ConfigResultIR,
    mut value: *const ::core::ffi::c_char,
) {
    (*current_bar).hide_binding_mode_indicator = !boolstr(value);
}
#[no_mangle]
pub unsafe extern "C" fn cfg_bar_workspace_buttons(
    mut current_match: *mut Match,
    mut result: *mut ConfigResultIR,
    mut value: *const ::core::ffi::c_char,
) {
    (*current_bar).hide_workspace_buttons = !boolstr(value);
}
#[no_mangle]
pub unsafe extern "C" fn cfg_bar_workspace_min_width(
    mut current_match: *mut Match,
    mut result: *mut ConfigResultIR,
    width: ::core::ffi::c_long,
) {
    (*current_bar).workspace_min_width = width as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn cfg_bar_strip_workspace_numbers(
    mut current_match: *mut Match,
    mut result: *mut ConfigResultIR,
    mut value: *const ::core::ffi::c_char,
) {
    (*current_bar).strip_workspace_numbers = boolstr(value);
}
#[no_mangle]
pub unsafe extern "C" fn cfg_bar_strip_workspace_name(
    mut current_match: *mut Match,
    mut result: *mut ConfigResultIR,
    mut value: *const ::core::ffi::c_char,
) {
    (*current_bar).strip_workspace_name = boolstr(value);
}
#[no_mangle]
pub unsafe extern "C" fn cfg_bar_start(
    mut current_match: *mut Match,
    mut result: *mut ConfigResultIR,
) {
    current_bar =
        scalloc(1 as size_t, ::core::mem::size_of::<Barconfig>() as size_t) as *mut Barconfig;
    (*current_bar).bar_bindings.tqh_first = ::core::ptr::null_mut::<Barbinding>();
    (*current_bar).bar_bindings.tqh_last =
        &raw mut (*current_bar).bar_bindings.tqh_first as *mut *mut Barbinding;
    (*current_bar).tray_outputs.tqh_first = ::core::ptr::null_mut::<tray_output_t>();
    (*current_bar).tray_outputs.tqh_last =
        &raw mut (*current_bar).tray_outputs.tqh_first as *mut *mut tray_output_t;
    (*current_bar).tray_padding = 2 as ::core::ffi::c_int;
    (*current_bar).modifier = XCB_KEY_BUT_MASK_MOD_4 as ::core::ffi::c_int as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn cfg_bar_finish(
    mut current_match: *mut Match,
    mut result: *mut ConfigResultIR,
) {
    debuglog(
        b"%s:%s:%d - \t new bar configuration finished, saving.\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/config_directives.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"cfg_bar_finish\0" as *const u8 as *const ::core::ffi::c_char,
        959 as ::core::ffi::c_int,
    );
    if (*current_bar).id.is_null() {
        sasprintf(
            &raw mut (*current_bar).id,
            b"bar-%d\0" as *const u8 as *const ::core::ffi::c_char,
            config.number_barconfigs,
        );
    }
    config.number_barconfigs += 1;
    (*current_bar).configs.tqe_next = ::core::ptr::null_mut::<Barconfig>();
    (*current_bar).configs.tqe_prev = barconfigs.tqh_last;
    *barconfigs.tqh_last = current_bar as *mut Barconfig;
    barconfigs.tqh_last = &raw mut (*current_bar).configs.tqe_next;
    current_bar = ::core::ptr::null_mut::<Barconfig>();
}
