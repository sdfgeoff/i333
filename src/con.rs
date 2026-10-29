/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * con.c: Functions which deal with containers directly (creating containers,
 *        searching containers, getting specific properties from containers,
 *        …).
 *
 */
// Mechanical file-by-file Rust translation of src/con.c.
// Generated with C2Rust 0.22.1; C ABI and original ownership retained.
extern "C" {
    pub type xcb_connection_t;
    pub type _PangoFontDescription;
    pub type _cairo;
    pub type _cairo_surface;
    pub type _i3String;
    pub type pcre2_real_code_8;
    pub type ev_loop;
    pub type yajl_gen_t;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn __assert_single_arg(_: bool) -> bool;
    fn free(__ptr: *mut ::core::ffi::c_void);
    static mut conn: *mut xcb_connection_t;
    fn verboselog(fmt: *mut ::core::ffi::c_char, ...);
    fn errorlog(fmt: *mut ::core::ffi::c_char, ...);
    fn debuglog(fmt: *mut ::core::ffi::c_char, ...);
    fn smalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn scalloc(num: size_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn sstrdup(str: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn sasprintf(
        strp: *mut *mut ::core::ffi::c_char,
        fmt: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn strcasecmp_nullable(
        a: *const ::core::ffi::c_char,
        b: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn i3string_from_utf8(from_utf8: *const ::core::ffi::c_char) -> *mut i3String;
    fn i3string_as_utf8(str: *mut i3String) -> *const ::core::ffi::c_char;
    fn i3string_set_markup(str: *mut i3String, pango_markup: bool);
    fn font_is_pango() -> bool;
    fn format_placeholders(
        format: *mut ::core::ffi::c_char,
        placeholders: *mut placeholder_t,
        num: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    fn max(a: ::core::ffi::c_int, b: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn rect_add(a: Rect, b: Rect) -> Rect;
    fn rect_sub(a: Rect, b: Rect) -> Rect;
    fn layout_from_name(layout_str: *const ::core::ffi::c_char, out: *mut layout_t) -> bool;
    fn pango_escape_markup(input: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn yajl_gen_free(handle: yajl_gen);
    fn yajl_gen_get_buf(
        hand: yajl_gen,
        buf: *mut *const ::core::ffi::c_uchar,
        len: *mut size_t,
    ) -> yajl_gen_status;
    static mut croot: *mut Con;
    static mut focused: *mut Con;
    static mut all_cons: all_cons_head;
    fn tree_close_internal(
        con: *mut Con,
        kill_window: kill_window_t,
        dont_kill_parent: bool,
    ) -> bool;
    fn tree_flatten(child: *mut Con);
    static mut root_depth: uint8_t;
    static mut config: Config;
    fn ipc_send_event(
        event: *const ::core::ffi::c_char,
        message_type: uint32_t,
        payload: *const ::core::ffi::c_char,
    );
    fn ipc_marshal_workspace_event(
        change: *const ::core::ffi::c_char,
        current: *mut Con,
        old: *mut Con,
    ) -> yajl_gen;
    fn ipc_send_window_event(property: *const ::core::ffi::c_char, con: *mut Con);
    static mut A__NET_WM_STATE: xcb_atom_t;
    static mut A__NET_WM_STATE_FULLSCREEN: xcb_atom_t;
    fn xcb_add_property_atom(
        conn_0: *mut xcb_connection_t,
        window: xcb_window_t,
        property: xcb_atom_t,
        atom: xcb_atom_t,
    );
    fn xcb_remove_property_atom(
        conn_0: *mut xcb_connection_t,
        window: xcb_window_t,
        property: xcb_atom_t,
        atom: xcb_atom_t,
    );
    fn workspace_get(num: *const ::core::ffi::c_char) -> *mut Con;
    fn workspace_is_visible(ws: *mut Con) -> bool;
    fn workspace_show(ws: *mut Con);
    fn workspace_update_urgent_flag(ws: *mut Con);
    fn workspace_attach_to(ws: *mut Con) -> *mut Con;
    fn workspace_encapsulate(ws: *mut Con) -> *mut Con;
    fn x_con_init(con: *mut Con);
    fn x_set_warp_to(rect: *mut Rect);
    fn floating_raise_con(con: *mut Con);
    fn floating_fix_coordinates(con: *mut Con, old_rect: *mut Rect, new_rect: *mut Rect);
    fn calculate_effective_gaps(con: *mut Con) -> gaps_t;
    fn render_deco_height() -> ::core::ffi::c_int;
    fn match_matches_window(match_0: *mut Match, window: *mut i3Window) -> bool;
    fn match_free(match_0: *mut Match);
    fn output_get_content(output: *mut Con) -> *mut Con;
    fn get_output_from_string(
        current_output: *mut Output,
        output_str: *const ::core::ffi::c_char,
    ) -> *mut Output;
    fn output_primary_name(output: *mut Output) -> *mut ::core::ffi::c_char;
    fn get_output_for_con(con: *mut Con) -> *mut Output;
    fn ewmh_update_wm_desktop();
    fn startup_sequence_delete_by_window(win: *const i3Window);
    fn scratchpad_move(con: *mut Con);
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strtok(
        __s: *mut ::core::ffi::c_char,
        __delim: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strcasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __time_t = ::core::ffi::c_long;
pub type __suseconds_t = ::core::ffi::c_long;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type xcb_window_t = uint32_t;
pub type xcb_font_t = uint32_t;
pub type xcb_gcontext_t = uint32_t;
pub type xcb_colormap_t = uint32_t;
pub type xcb_atom_t = uint32_t;
pub type xcb_drawable_t = uint32_t;
pub type xcb_window_enum_t = ::core::ffi::c_uint;
pub const XCB_WINDOW_NONE: xcb_window_enum_t = 0;
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
pub struct placeholder_t {
    pub name: *const ::core::ffi::c_char,
    pub value: *const ::core::ffi::c_char,
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
pub type i3Window = Window;
pub type direction_t = ::core::ffi::c_uint;
pub const D_DOWN: direction_t = 3;
pub const D_UP: direction_t = 2;
pub const D_RIGHT: direction_t = 1;
pub const D_LEFT: direction_t = 0;
pub type orientation_t = ::core::ffi::c_uint;
pub const VERT: orientation_t = 2;
pub const HORIZ: orientation_t = 1;
pub const NO_ORIENTATION: orientation_t = 0;
pub type kill_window_t = ::core::ffi::c_uint;
pub const KILL_CLIENT: kill_window_t = 2;
pub const KILL_WINDOW: kill_window_t = 1;
pub const DONT_KILL_WINDOW: kill_window_t = 0;
pub type adjacent_t = ::core::ffi::c_uint;
pub const ADJ_LOWER_SCREEN_EDGE: adjacent_t = 16;
pub const ADJ_UPPER_SCREEN_EDGE: adjacent_t = 4;
pub const ADJ_RIGHT_SCREEN_EDGE: adjacent_t = 2;
pub const ADJ_LEFT_SCREEN_EDGE: adjacent_t = 1;
pub const ADJ_NONE: adjacent_t = 0;
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
pub type mark_mode_t = ::core::ffi::c_uint;
pub const MM_ADD: mark_mode_t = 1;
pub const MM_REPLACE: mark_mode_t = 0;
pub type warping_t = ::core::ffi::c_uint;
pub const POINTER_WARPING_NONE: warping_t = 1;
pub const POINTER_WARPING_OUTPUT: warping_t = 0;
pub type focus_wrapping_t = ::core::ffi::c_uint;
pub const FOCUS_WRAPPING_WORKSPACE: focus_wrapping_t = 3;
pub const FOCUS_WRAPPING_FORCE: focus_wrapping_t = 2;
pub const FOCUS_WRAPPING_ON: focus_wrapping_t = 1;
pub const FOCUS_WRAPPING_OFF: focus_wrapping_t = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct all_cons_head {
    pub tqh_first: *mut Con,
    pub tqh_last: *mut *mut Con,
}
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
    pub focus_on_window_activation: C2RustUnnamed_23,
    pub show_marks: bool,
    pub title_align: C2RustUnnamed_22,
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
    pub popup_during_fullscreen: C2RustUnnamed_21,
    pub number_barconfigs: ::core::ffi::c_int,
    pub tiling_drag: tiling_drag_t,
    pub gaps: gaps_t,
    pub smart_gaps: smart_gaps_t,
}
pub type C2RustUnnamed_21 = ::core::ffi::c_uint;
pub const PDF_ALL: C2RustUnnamed_21 = 3;
pub const PDF_IGNORE: C2RustUnnamed_21 = 2;
pub const PDF_LEAVE_FULLSCREEN: C2RustUnnamed_21 = 1;
pub const PDF_SMART: C2RustUnnamed_21 = 0;
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
pub type C2RustUnnamed_22 = ::core::ffi::c_uint;
pub const ALIGN_RIGHT: C2RustUnnamed_22 = 2;
pub const ALIGN_CENTER: C2RustUnnamed_22 = 1;
pub const ALIGN_LEFT: C2RustUnnamed_22 = 0;
pub type C2RustUnnamed_23 = ::core::ffi::c_uint;
pub const FOWA_NONE: C2RustUnnamed_23 = 3;
pub const FOWA_FOCUS: C2RustUnnamed_23 = 2;
pub const FOWA_URGENT: C2RustUnnamed_23 = 1;
pub const FOWA_SMART: C2RustUnnamed_23 = 0;
pub type ylength = size_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bfs_entry {
    pub con: *mut Con,
    pub entries: C2RustUnnamed_24,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_24 {
    pub tqe_next: *mut bfs_entry,
    pub tqe_prev: *mut *mut bfs_entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bfs_head {
    pub tqh_first: *mut bfs_entry,
    pub tqh_last: *mut *mut bfs_entry,
}
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const XCB_NONE: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const I3_IPC_EVENT_MASK: ::core::ffi::c_ulong =
    (1 as ::core::ffi::c_ulong) << 31 as ::core::ffi::c_int;
pub const I3_IPC_EVENT_WORKSPACE: ::core::ffi::c_ulong =
    I3_IPC_EVENT_MASK | 0 as ::core::ffi::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn con_force_split_parents_redraw(mut con: *mut Con) {
    let mut parent: *mut Con = con;
    while !parent.is_null()
        && (*parent).type_0 as ::core::ffi::c_uint
            != CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
        && (*parent).type_0 as ::core::ffi::c_uint
            != CT_DOCKAREA as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if !con_is_leaf(parent) {
            free((*parent).deco_render_params as *mut ::core::ffi::c_void);
            (*parent).deco_render_params = ::core::ptr::null_mut::<deco_render_params>();
        }
        parent = (*parent).parent as *mut Con;
    }
}
#[no_mangle]
pub unsafe extern "C" fn con_new_skeleton(
    mut parent: *mut Con,
    mut window: *mut i3Window,
) -> *mut Con {
    let mut new: *mut Con =
        scalloc(1 as size_t, ::core::mem::size_of::<Con>() as size_t) as *mut Con;
    (*new).on_remove_child = Some(con_on_remove_child as unsafe extern "C" fn(*mut Con) -> ())
        as Option<unsafe extern "C" fn(*mut Con) -> ()>;
    (*new).all_cons.tqe_next = ::core::ptr::null_mut::<Con>();
    (*new).all_cons.tqe_prev = all_cons.tqh_last;
    *all_cons.tqh_last = new as *mut Con;
    all_cons.tqh_last = &raw mut (*new).all_cons.tqe_next;
    (*new).type_0 = CT_CON;
    (*new).window = window as *mut Window;
    (*new).max_user_border_style = config.default_border;
    (*new).border_style = (*new).max_user_border_style;
    (*new).current_border_width = -(1 as ::core::ffi::c_int);
    (*new).window_icon_padding = -(1 as ::core::ffi::c_int);
    if !window.is_null() {
        (*new).depth = (*window).depth;
    } else {
        (*new).depth = root_depth as uint16_t;
    }
    debuglog(
        b"%s:%s:%d - opening window\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"con_new_skeleton\0" as *const u8 as *const ::core::ffi::c_char,
        52 as ::core::ffi::c_int,
    );
    (*new).floating_head.tqh_first = ::core::ptr::null_mut::<Con>();
    (*new).floating_head.tqh_last = &raw mut (*new).floating_head.tqh_first;
    (*new).nodes_head.tqh_first = ::core::ptr::null_mut::<Con>();
    (*new).nodes_head.tqh_last = &raw mut (*new).nodes_head.tqh_first;
    (*new).focus_head.tqh_first = ::core::ptr::null_mut::<Con>();
    (*new).focus_head.tqh_last = &raw mut (*new).focus_head.tqh_first;
    (*new).swallow_head.tqh_first = ::core::ptr::null_mut::<Match>();
    (*new).swallow_head.tqh_last = &raw mut (*new).swallow_head.tqh_first;
    (*new).marks_head.tqh_first = ::core::ptr::null_mut::<mark_t>();
    (*new).marks_head.tqh_last = &raw mut (*new).marks_head.tqh_first;
    if !parent.is_null() {
        con_attach(new, parent, false_0 != 0);
    }
    return new;
}
#[no_mangle]
pub unsafe extern "C" fn con_new(mut parent: *mut Con, mut window: *mut i3Window) -> *mut Con {
    let mut new: *mut Con = con_new_skeleton(parent, window);
    x_con_init(new);
    return new;
}
#[no_mangle]
pub unsafe extern "C" fn con_free(mut con: *mut Con) {
    free((*con).name as *mut ::core::ffi::c_void);
    free((*con).deco_render_params as *mut ::core::ffi::c_void);
    (*con).deco_render_params = ::core::ptr::null_mut::<deco_render_params>();
    if !(*con).all_cons.tqe_next.is_null() {
        (*(*con).all_cons.tqe_next).all_cons.tqe_prev = (*con).all_cons.tqe_prev;
    } else {
        all_cons.tqh_last = (*con).all_cons.tqe_prev;
    }
    *(*con).all_cons.tqe_prev = (*con).all_cons.tqe_next;
    while !(*con).swallow_head.tqh_first.is_null() {
        let mut match_0: *mut Match = (*con).swallow_head.tqh_first as *mut Match;
        if !(*match_0).matches.tqe_next.is_null() {
            (*(*match_0).matches.tqe_next).matches.tqe_prev = (*match_0).matches.tqe_prev;
        } else {
            (*con).swallow_head.tqh_last = (*match_0).matches.tqe_prev;
        }
        *(*match_0).matches.tqe_prev = (*match_0).matches.tqe_next;
        match_free(match_0);
        free(match_0 as *mut ::core::ffi::c_void);
    }
    while !(*con).marks_head.tqh_first.is_null() {
        let mut mark: *mut mark_t = (*con).marks_head.tqh_first as *mut mark_t;
        if !(*mark).marks.tqe_next.is_null() {
            (*(*mark).marks.tqe_next).marks.tqe_prev = (*mark).marks.tqe_prev;
        } else {
            (*con).marks_head.tqh_last = (*mark).marks.tqe_prev;
        }
        *(*mark).marks.tqe_prev = (*mark).marks.tqe_next;
        free((*mark).name as *mut ::core::ffi::c_void);
        (*mark).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
        free(mark as *mut ::core::ffi::c_void);
        mark = ::core::ptr::null_mut::<mark_t>();
    }
    debuglog(
        b"%s:%s:%d - con %p freed\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"con_free\0" as *const u8 as *const ::core::ffi::c_char,
        96 as ::core::ffi::c_int,
        con,
    );
    free(con as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn _con_attach(
    mut con: *mut Con,
    mut parent: *mut Con,
    mut previous: *mut Con,
    mut ignore_focus: bool,
) {
    (*con).parent = parent as *mut Con;
    let mut loop_0: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut current: *mut Con = previous;
    let mut nodes_head: *mut nodes_head = &raw mut (*parent).nodes_head;
    let mut focus_head: *mut focus_head = &raw mut (*parent).focus_head;
    if (*con).type_0 as ::core::ffi::c_uint
        == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        debuglog(
            b"%s:%s:%d - it's a workspace. num = %d\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"_con_attach\0" as *const u8 as *const ::core::ffi::c_char,
            110 as ::core::ffi::c_int,
            (*con).num,
        );
        if (*con).num == -(1 as ::core::ffi::c_int) || (*nodes_head).tqh_first.is_null() {
            (*con).nodes.tqe_next = ::core::ptr::null_mut::<Con>();
            (*con).nodes.tqe_prev = (*nodes_head).tqh_last;
            *(*nodes_head).tqh_last = con as *mut Con;
            (*nodes_head).tqh_last = &raw mut (*con).nodes.tqe_next;
        } else {
            current = (*nodes_head).tqh_first as *mut Con;
            if (*con).num < (*current).num {
                (*con).nodes.tqe_next = (*nodes_head).tqh_first;
                if !(*con).nodes.tqe_next.is_null() {
                    (*(*nodes_head).tqh_first).nodes.tqe_prev = &raw mut (*con).nodes.tqe_next;
                } else {
                    (*nodes_head).tqh_last = &raw mut (*con).nodes.tqe_next;
                }
                (*nodes_head).tqh_first = con as *mut Con;
                (*con).nodes.tqe_prev = &raw mut (*nodes_head).tqh_first;
            } else {
                while (*current).num != -(1 as ::core::ffi::c_int) && (*con).num >= (*current).num {
                    current = (*current).nodes.tqe_next as *mut Con;
                    if !current.is_null() {
                        continue;
                    }
                    current = ::core::ptr::null_mut::<Con>();
                    break;
                }
                if !current.is_null() {
                    (*con).nodes.tqe_prev = (*current).nodes.tqe_prev;
                    (*con).nodes.tqe_next = current as *mut Con;
                    *(*current).nodes.tqe_prev = con as *mut Con;
                    (*current).nodes.tqe_prev = &raw mut (*con).nodes.tqe_next;
                } else {
                    (*con).nodes.tqe_next = ::core::ptr::null_mut::<Con>();
                    (*con).nodes.tqe_prev = (*nodes_head).tqh_last;
                    *(*nodes_head).tqh_last = con as *mut Con;
                    (*nodes_head).tqh_last = &raw mut (*con).nodes.tqe_next;
                }
            }
        }
    } else if (*parent).type_0 as ::core::ffi::c_uint
        == CT_DOCKAREA as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        current = ::core::ptr::null_mut::<Con>();
        loop_0 = (*nodes_head).tqh_first as *mut Con;
        while !loop_0.is_null() {
            let mut result: ::core::ffi::c_int = strcasecmp_nullable(
                (*(*con).window).class_class,
                (*(*loop_0).window).class_class,
            );
            if result == 0 as ::core::ffi::c_int {
                result = strcasecmp_nullable(
                    (*(*con).window).class_instance,
                    (*(*loop_0).window).class_instance,
                );
            }
            if result < 0 as ::core::ffi::c_int {
                current = loop_0;
                break;
            } else {
                loop_0 = (*loop_0).nodes.tqe_next as *mut Con;
            }
        }
        if !current.is_null() {
            (*con).nodes.tqe_prev = (*loop_0).nodes.tqe_prev;
            (*con).nodes.tqe_next = loop_0 as *mut Con;
            *(*loop_0).nodes.tqe_prev = con as *mut Con;
            (*loop_0).nodes.tqe_prev = &raw mut (*con).nodes.tqe_next;
        } else {
            (*con).nodes.tqe_next = ::core::ptr::null_mut::<Con>();
            (*con).nodes.tqe_prev = (*nodes_head).tqh_last;
            *(*nodes_head).tqh_last = con as *mut Con;
            (*nodes_head).tqh_last = &raw mut (*con).nodes.tqe_next;
        }
    } else if (*con).type_0 as ::core::ffi::c_uint
        == CT_FLOATING_CON as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        debuglog(
            b"%s:%s:%d - Inserting into floating containers\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"_con_attach\0" as *const u8 as *const ::core::ffi::c_char,
            162 as ::core::ffi::c_int,
        );
        (*con).floating_windows.tqe_next = ::core::ptr::null_mut::<Con>();
        (*con).floating_windows.tqe_prev = (*parent).floating_head.tqh_last;
        *(*parent).floating_head.tqh_last = con as *mut Con;
        (*parent).floating_head.tqh_last = &raw mut (*con).floating_windows.tqe_next;
    } else {
        if !ignore_focus {
            loop_0 = (*parent).focus_head.tqh_first as *mut Con;
            while !loop_0.is_null() {
                if (*loop_0).type_0 as ::core::ffi::c_uint
                    == CT_FLOATING_CON as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    loop_0 = (*loop_0).focused.tqe_next as *mut Con;
                } else {
                    current = loop_0;
                    break;
                }
            }
        }
        if !(*con).window.is_null()
            && (*parent).type_0 as ::core::ffi::c_uint
                == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
            && (*parent).workspace_layout as ::core::ffi::c_uint
                != L_DEFAULT as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            debuglog(
                b"%s:%s:%d - Parent is a workspace. Applying default layout...\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"_con_attach\0" as *const u8 as *const ::core::ffi::c_char,
                186 as ::core::ffi::c_int,
            );
            let mut target: *mut Con = workspace_attach_to(parent);
            nodes_head = &raw mut (*target).nodes_head as *mut nodes_head;
            focus_head = &raw mut (*target).focus_head as *mut focus_head;
            (*con).parent = target as *mut Con;
            current = ::core::ptr::null_mut::<Con>();
            debuglog(
                b"%s:%s:%d - done\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"_con_attach\0" as *const u8 as *const ::core::ffi::c_char,
                195 as ::core::ffi::c_int,
            );
        }
        if !current.is_null()
            && (*parent).type_0 as ::core::ffi::c_uint
                != CT_OUTPUT as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            debuglog(
                b"%s:%s:%d - Inserting con = %p after con %p\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"_con_attach\0" as *const u8 as *const ::core::ffi::c_char,
                201 as ::core::ffi::c_int,
                con,
                current,
            );
            (*con).nodes.tqe_next = (*current).nodes.tqe_next;
            if !(*con).nodes.tqe_next.is_null() {
                (*(*con).nodes.tqe_next).nodes.tqe_prev = &raw mut (*con).nodes.tqe_next;
            } else {
                (*nodes_head).tqh_last = &raw mut (*con).nodes.tqe_next;
            }
            (*current).nodes.tqe_next = con as *mut Con;
            (*con).nodes.tqe_prev = &raw mut (*current).nodes.tqe_next;
        } else {
            (*con).nodes.tqe_next = ::core::ptr::null_mut::<Con>();
            (*con).nodes.tqe_prev = (*nodes_head).tqh_last;
            *(*nodes_head).tqh_last = con as *mut Con;
            (*nodes_head).tqh_last = &raw mut (*con).nodes.tqe_next;
        }
    }
    (*con).focused.tqe_next = ::core::ptr::null_mut::<Con>();
    (*con).focused.tqe_prev = (*focus_head).tqh_last;
    *(*focus_head).tqh_last = con as *mut Con;
    (*focus_head).tqh_last = &raw mut (*con).focused.tqe_next;
    con_force_split_parents_redraw(con);
}
#[no_mangle]
pub unsafe extern "C" fn con_attach(
    mut con: *mut Con,
    mut parent: *mut Con,
    mut ignore_focus: bool,
) {
    _con_attach(con, parent, ::core::ptr::null_mut::<Con>(), ignore_focus);
}
#[no_mangle]
pub unsafe extern "C" fn con_detach(mut con: *mut Con) {
    con_force_split_parents_redraw(con);
    if (*con).type_0 as ::core::ffi::c_uint
        == CT_FLOATING_CON as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if !(*con).floating_windows.tqe_next.is_null() {
            (*(*con).floating_windows.tqe_next)
                .floating_windows
                .tqe_prev = (*con).floating_windows.tqe_prev;
        } else {
            (*(*con).parent).floating_head.tqh_last = (*con).floating_windows.tqe_prev;
        }
        *(*con).floating_windows.tqe_prev = (*con).floating_windows.tqe_next;
        if !(*con).focused.tqe_next.is_null() {
            (*(*con).focused.tqe_next).focused.tqe_prev = (*con).focused.tqe_prev;
        } else {
            (*(*con).parent).focus_head.tqh_last = (*con).focused.tqe_prev;
        }
        *(*con).focused.tqe_prev = (*con).focused.tqe_next;
    } else {
        if !(*con).nodes.tqe_next.is_null() {
            (*(*con).nodes.tqe_next).nodes.tqe_prev = (*con).nodes.tqe_prev;
        } else {
            (*(*con).parent).nodes_head.tqh_last = (*con).nodes.tqe_prev;
        }
        *(*con).nodes.tqe_prev = (*con).nodes.tqe_next;
        if !(*con).focused.tqe_next.is_null() {
            (*(*con).focused.tqe_next).focused.tqe_prev = (*con).focused.tqe_prev;
        } else {
            (*(*con).parent).focus_head.tqh_last = (*con).focused.tqe_prev;
        }
        *(*con).focused.tqe_prev = (*con).focused.tqe_next;
    };
}
#[no_mangle]
pub unsafe extern "C" fn con_focus(mut con: *mut Con) {
    if con != ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut Con {
    } else {
        __assert_fail(
            b"con != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
            251 as ::core::ffi::c_uint,
            b"void con_focus(Con *)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
    debuglog(
        b"%s:%s:%d - con_focus = %p\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"con_focus\0" as *const u8 as *const ::core::ffi::c_char,
        252 as ::core::ffi::c_int,
        con,
    );
    if !(*con).focused.tqe_next.is_null() {
        (*(*con).focused.tqe_next).focused.tqe_prev = (*con).focused.tqe_prev;
    } else {
        (*(*con).parent).focus_head.tqh_last = (*con).focused.tqe_prev;
    }
    *(*con).focused.tqe_prev = (*con).focused.tqe_next;
    (*con).focused.tqe_next = (*(*con).parent).focus_head.tqh_first;
    if !(*con).focused.tqe_next.is_null() {
        (*(*(*con).parent).focus_head.tqh_first).focused.tqe_prev =
            &raw mut (*con).focused.tqe_next;
    } else {
        (*(*con).parent).focus_head.tqh_last = &raw mut (*con).focused.tqe_next;
    }
    (*(*con).parent).focus_head.tqh_first = con as *mut Con;
    (*con).focused.tqe_prev = &raw mut (*(*con).parent).focus_head.tqh_first;
    if !(*(*con).parent).parent.is_null() {
        con_focus((*con).parent as *mut Con);
    }
    focused = con;
    if (*con).urgent as ::core::ffi::c_int != 0 && con_is_leaf(con) as ::core::ffi::c_int != 0 {
        con_set_urgency(con, false_0 != 0);
        con_update_parents_urgency(con);
        workspace_update_urgent_flag(con_get_workspace(con));
        ipc_send_window_event(b"urgent\0" as *const u8 as *const ::core::ffi::c_char, con);
    }
}
unsafe extern "C" fn con_raise(mut con: *mut Con) {
    let mut floating: *mut Con = con_inside_floating(con);
    if !floating.is_null() {
        floating_raise_con(floating);
    }
}
#[no_mangle]
pub unsafe extern "C" fn con_activate(mut con: *mut Con) {
    con_focus(con);
    con_raise(con);
}
#[no_mangle]
pub unsafe extern "C" fn con_activate_unblock(mut con: *mut Con) {
    let mut ws: *mut Con = con_get_workspace(con);
    let mut previous_focus: *mut Con = focused;
    let mut fullscreen_on_ws: *mut Con = con_get_fullscreen_covering_ws(ws);
    if !fullscreen_on_ws.is_null()
        && fullscreen_on_ws != con
        && !con_has_parent(con, fullscreen_on_ws)
    {
        con_disable_fullscreen(fullscreen_on_ws);
    }
    con_activate(con);
    if ws != con_get_workspace(previous_focus) {
        con_activate(previous_focus);
        workspace_show(ws);
        con_activate(con);
    }
}
#[no_mangle]
pub unsafe extern "C" fn con_close(mut con: *mut Con, mut kill_window: kill_window_t) {
    if con != ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut Con {
    } else {
        __assert_fail(
            b"con != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
            337 as ::core::ffi::c_uint,
            b"void con_close(Con *, kill_window_t)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
    debuglog(
        b"%s:%s:%d - Closing con = %p.\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"con_close\0" as *const u8 as *const ::core::ffi::c_char,
        338 as ::core::ffi::c_int,
        con,
    );
    if (*con).type_0 as ::core::ffi::c_uint
        == CT_OUTPUT as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*con).type_0 as ::core::ffi::c_uint
            == CT_ROOT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        debuglog(
            b"%s:%s:%d - con = %p is of type %d, not closing anything.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"con_close\0" as *const u8 as *const ::core::ffi::c_char,
            342 as ::core::ffi::c_int,
            con,
            (*con).type_0 as ::core::ffi::c_uint,
        );
        return;
    }
    if (*con).type_0 as ::core::ffi::c_uint
        == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        debuglog(
            b"%s:%s:%d - con = %p is a workspace, closing all children instead.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"con_close\0" as *const u8 as *const ::core::ffi::c_char,
            347 as ::core::ffi::c_int,
            con,
        );
        let mut child: *mut Con = (*con).focus_head.tqh_first as *mut Con;
        while !child.is_null() {
            let mut next_child: *mut Con = (*child).focused.tqe_next as *mut Con;
            debuglog(
                b"%s:%s:%d - killing child = %p.\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"con_close\0" as *const u8 as *const ::core::ffi::c_char,
                350 as ::core::ffi::c_int,
                child,
            );
            tree_close_internal(child, kill_window, false_0 != 0);
            child = next_child;
        }
        return;
    }
    tree_close_internal(con, kill_window, false_0 != 0);
}
#[no_mangle]
pub unsafe extern "C" fn con_is_leaf(mut con: *mut Con) -> bool {
    return (*con).nodes_head.tqh_first.is_null();
}
#[no_mangle]
pub unsafe extern "C" fn con_has_managed_window(mut con: *mut Con) -> bool {
    return !con.is_null()
        && !(*con).window.is_null()
        && (*(*con).window).id != XCB_WINDOW_NONE as ::core::ffi::c_int as xcb_window_t
        && !con_get_workspace(con).is_null();
}
#[no_mangle]
pub unsafe extern "C" fn con_has_children(mut con: *mut Con) -> bool {
    return !con_is_leaf(con) || !(*con).floating_head.tqh_first.is_null();
}
#[no_mangle]
pub unsafe extern "C" fn con_is_split(mut con: *mut Con) -> bool {
    if con_is_leaf(con) {
        return false_0 != 0;
    }
    match (*con).layout as ::core::ffi::c_uint {
        3 | 4 => return false_0 != 0,
        _ => return true_0 != 0,
    };
}
#[no_mangle]
pub unsafe extern "C" fn con_is_hidden(mut con: *mut Con) -> bool {
    let mut current: *mut Con = con;
    while !current.is_null()
        && (*current).type_0 as ::core::ffi::c_uint
            != CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut parent: *mut Con = (*current).parent as *mut Con;
        if !parent.is_null()
            && ((*parent).layout as ::core::ffi::c_uint
                == L_TABBED as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*parent).layout as ::core::ffi::c_uint
                    == L_STACKED as ::core::ffi::c_int as ::core::ffi::c_uint)
        {
            if (*parent).focus_head.tqh_first != current {
                return true_0 != 0;
            }
        }
        current = parent;
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn con_is_maximized(
    mut con: *mut Con,
    mut orientation: orientation_t,
) -> bool {
    if (*con).fullscreen_mode as ::core::ffi::c_uint
        != CF_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return true_0 != 0;
    }
    let mut layout: layout_t = L_DEFAULT;
    match orientation as ::core::ffi::c_uint {
        1 => {
            layout = L_SPLITH;
        }
        2 => {
            layout = L_SPLITV;
        }
        _ => {
            if 0 as ::core::ffi::c_int != 0 {
            } else {
                __assert_fail(
                    b"false\0" as *const u8 as *const ::core::ffi::c_char,
                    b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
                    458 as ::core::ffi::c_uint,
                    b"_Bool con_is_maximized(Con *, orientation_t)\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            };
        }
    }
    let mut current: *mut Con = con;
    loop {
        let mut parent: *mut Con = (*current).parent as *mut Con;
        if parent.is_null()
            || (*current).type_0 as ::core::ffi::c_uint
                == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return true_0 != 0;
        }
        if (*parent).layout as ::core::ffi::c_uint == layout as ::core::ffi::c_uint
            && con_num_children(parent) > 1 as ::core::ffi::c_int
        {
            return false_0 != 0;
        }
        if (*parent).type_0 as ::core::ffi::c_uint
            == CT_FLOATING_CON as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return false_0 != 0;
        }
        current = parent;
    }
}
#[no_mangle]
pub unsafe extern "C" fn con_is_sticky(mut con: *mut Con) -> bool {
    if (*con).sticky {
        return true_0 != 0;
    }
    let mut child: *mut Con = ::core::ptr::null_mut::<Con>();
    child = (*con).nodes_head.tqh_first as *mut Con;
    while !child.is_null() {
        if con_is_sticky(child) {
            return true_0 != 0;
        }
        child = (*child).nodes.tqe_next as *mut Con;
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn con_accepts_window(mut con: *mut Con) -> bool {
    if (*con).type_0 as ::core::ffi::c_uint
        == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return false_0 != 0;
    }
    if con_is_split(con) {
        debuglog(
            b"%s:%s:%d - container %p does not accept windows, it is a split container.\n\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"con_accepts_window\0" as *const u8 as *const ::core::ffi::c_char,
            518 as ::core::ffi::c_int,
            con,
        );
        return false_0 != 0;
    }
    return (*con).window.is_null();
}
#[no_mangle]
pub unsafe extern "C" fn con_get_output(mut con: *mut Con) -> *mut Con {
    let mut result: *mut Con = con;
    while !result.is_null()
        && (*result).type_0 as ::core::ffi::c_uint
            != CT_OUTPUT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        result = (*result).parent as *mut Con;
    }
    if result != ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut Con {
    } else {
        __assert_fail(
            b"result != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
            538 as ::core::ffi::c_uint,
            b"Con *con_get_output(Con *)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn con_get_workspace(mut con: *mut Con) -> *mut Con {
    let mut result: *mut Con = con;
    while !result.is_null()
        && (*result).type_0 as ::core::ffi::c_uint
            != CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        result = (*result).parent as *mut Con;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn con_parent_with_orientation(
    mut con: *mut Con,
    mut orientation: orientation_t,
) -> *mut Con {
    debuglog(
        b"%s:%s:%d - Searching for parent of Con %p with orientation %d\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"con_parent_with_orientation\0" as *const u8 as *const ::core::ffi::c_char,
        560 as ::core::ffi::c_int,
        con,
        orientation as ::core::ffi::c_uint,
    );
    let mut parent: *mut Con = (*con).parent as *mut Con;
    if (*parent).type_0 as ::core::ffi::c_uint
        == CT_FLOATING_CON as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return ::core::ptr::null_mut::<Con>();
    }
    while con_orientation(parent) as ::core::ffi::c_uint != orientation as ::core::ffi::c_uint {
        debuglog(
            b"%s:%s:%d - Need to go one level further up\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"con_parent_with_orientation\0" as *const u8 as *const ::core::ffi::c_char,
            566 as ::core::ffi::c_int,
        );
        parent = (*parent).parent as *mut Con;
        if !parent.is_null()
            && ((*parent).type_0 as ::core::ffi::c_uint
                == CT_FLOATING_CON as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*parent).type_0 as ::core::ffi::c_uint
                    == CT_OUTPUT as ::core::ffi::c_int as ::core::ffi::c_uint
                || !(*parent).parent.is_null()
                    && (*(*parent).parent).type_0 as ::core::ffi::c_uint
                        == CT_OUTPUT as ::core::ffi::c_int as ::core::ffi::c_uint)
        {
            parent = ::core::ptr::null_mut::<Con>();
        }
        if parent.is_null() {
            break;
        }
    }
    debuglog(
        b"%s:%s:%d - Result: %p\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"con_parent_with_orientation\0" as *const u8 as *const ::core::ffi::c_char,
        579 as ::core::ffi::c_int,
        parent,
    );
    return parent;
}
#[no_mangle]
pub unsafe extern "C" fn con_get_fullscreen_con(
    mut con: *mut Con,
    mut fullscreen_mode: fullscreen_mode_t,
) -> *mut Con {
    let mut bfs_head_0: bfs_head = bfs_head {
        tqh_first: ::core::ptr::null_mut::<bfs_entry>(),
        tqh_last: ::core::ptr::null_mut::<*mut bfs_entry>(),
    };
    bfs_head_0 = bfs_head {
        tqh_first: ::core::ptr::null_mut::<bfs_entry>(),
        tqh_last: &raw mut bfs_head_0.tqh_first,
    };
    let mut entry: *mut bfs_entry =
        smalloc(::core::mem::size_of::<bfs_entry>() as size_t) as *mut bfs_entry;
    (*entry).con = con;
    (*entry).entries.tqe_next = ::core::ptr::null_mut::<bfs_entry>();
    (*entry).entries.tqe_prev = bfs_head_0.tqh_last;
    *bfs_head_0.tqh_last = entry;
    bfs_head_0.tqh_last = &raw mut (*entry).entries.tqe_next;
    while !bfs_head_0.tqh_first.is_null() {
        entry = bfs_head_0.tqh_first;
        let mut current: *mut Con = (*entry).con;
        if current != con
            && (*current).fullscreen_mode as ::core::ffi::c_uint
                == fullscreen_mode as ::core::ffi::c_uint
        {
            while !bfs_head_0.tqh_first.is_null() {
                entry = bfs_head_0.tqh_first;
                if !(*entry).entries.tqe_next.is_null() {
                    (*(*entry).entries.tqe_next).entries.tqe_prev = (*entry).entries.tqe_prev;
                } else {
                    bfs_head_0.tqh_last = (*entry).entries.tqe_prev;
                }
                *(*entry).entries.tqe_prev = (*entry).entries.tqe_next;
                free(entry as *mut ::core::ffi::c_void);
            }
            return current;
        }
        if !(*entry).entries.tqe_next.is_null() {
            (*(*entry).entries.tqe_next).entries.tqe_prev = (*entry).entries.tqe_prev;
        } else {
            bfs_head_0.tqh_last = (*entry).entries.tqe_prev;
        }
        *(*entry).entries.tqe_prev = (*entry).entries.tqe_next;
        free(entry as *mut ::core::ffi::c_void);
        let mut child: *mut Con = ::core::ptr::null_mut::<Con>();
        child = (*current).nodes_head.tqh_first as *mut Con;
        while !child.is_null() {
            entry = smalloc(::core::mem::size_of::<bfs_entry>() as size_t) as *mut bfs_entry;
            (*entry).con = child;
            (*entry).entries.tqe_next = ::core::ptr::null_mut::<bfs_entry>();
            (*entry).entries.tqe_prev = bfs_head_0.tqh_last;
            *bfs_head_0.tqh_last = entry;
            bfs_head_0.tqh_last = &raw mut (*entry).entries.tqe_next;
            child = (*child).nodes.tqe_next as *mut Con;
        }
        child = (*current).floating_head.tqh_first as *mut Con;
        while !child.is_null() {
            entry = smalloc(::core::mem::size_of::<bfs_entry>() as size_t) as *mut bfs_entry;
            (*entry).con = child;
            (*entry).entries.tqe_next = ::core::ptr::null_mut::<bfs_entry>();
            (*entry).entries.tqe_prev = bfs_head_0.tqh_last;
            *bfs_head_0.tqh_last = entry;
            bfs_head_0.tqh_last = &raw mut (*entry).entries.tqe_next;
            child = (*child).floating_windows.tqe_next as *mut Con;
        }
    }
    return ::core::ptr::null_mut::<Con>();
}
#[no_mangle]
pub unsafe extern "C" fn con_get_fullscreen_covering_ws(mut ws: *mut Con) -> *mut Con {
    if ws.is_null() {
        return ::core::ptr::null_mut::<Con>();
    }
    let mut fs: *mut Con = con_get_fullscreen_con(croot, CF_GLOBAL);
    if fs.is_null() {
        return con_get_fullscreen_con(ws, CF_OUTPUT);
    }
    return fs;
}
#[no_mangle]
pub unsafe extern "C" fn con_is_internal(mut con: *mut Con) -> bool {
    return *(*con).name.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        == '_' as i32
        && *(*con).name.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '_' as i32;
}
#[no_mangle]
pub unsafe extern "C" fn con_is_floating(mut con: *mut Con) -> bool {
    if con != ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut Con {
    } else {
        __assert_fail(
            b"con != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
            669 as ::core::ffi::c_uint,
            b"_Bool con_is_floating(Con *)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
    return (*con).floating as ::core::ffi::c_uint
        >= FLOATING_AUTO_ON as ::core::ffi::c_int as ::core::ffi::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn con_is_docked(mut con: *mut Con) -> bool {
    if (*con).parent.is_null() {
        return false_0 != 0;
    }
    if (*(*con).parent).type_0 as ::core::ffi::c_uint
        == CT_DOCKAREA as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return true_0 != 0;
    }
    return con_is_docked((*con).parent as *mut Con);
}
#[no_mangle]
pub unsafe extern "C" fn con_inside_floating(mut con: *mut Con) -> *mut Con {
    if con.is_null() {
        return ::core::ptr::null_mut::<Con>();
    }
    if (*con).type_0 as ::core::ffi::c_uint
        == CT_FLOATING_CON as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return con;
    }
    if (*con).floating as ::core::ffi::c_uint
        >= FLOATING_AUTO_ON as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return (*con).parent as *mut Con;
    }
    if (*con).type_0 as ::core::ffi::c_uint
        == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*con).type_0 as ::core::ffi::c_uint
            == CT_OUTPUT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return ::core::ptr::null_mut::<Con>();
    }
    return con_inside_floating((*con).parent as *mut Con);
}
#[no_mangle]
pub unsafe extern "C" fn con_inside_focused(mut con: *mut Con) -> bool {
    if con == focused {
        return true_0 != 0;
    }
    if (*con).parent.is_null() {
        return false_0 != 0;
    }
    return con_inside_focused((*con).parent as *mut Con);
}
#[no_mangle]
pub unsafe extern "C" fn con_has_parent(mut con: *mut Con, mut parent: *mut Con) -> bool {
    let mut current: *mut Con = (*con).parent as *mut Con;
    if current.is_null() {
        return false_0 != 0;
    }
    if current == parent {
        return true_0 != 0;
    }
    return con_has_parent(current, parent);
}
#[no_mangle]
pub unsafe extern "C" fn con_by_window_id(mut window: xcb_window_t) -> *mut Con {
    let mut con: *mut Con = ::core::ptr::null_mut::<Con>();
    con = all_cons.tqh_first as *mut Con;
    while !con.is_null() {
        if !(*con).window.is_null() && (*(*con).window).id == window {
            return con;
        }
        con = (*con).all_cons.tqe_next as *mut Con;
    }
    return ::core::ptr::null_mut::<Con>();
}
#[no_mangle]
pub unsafe extern "C" fn con_by_con_id(mut target: ::core::ffi::c_long) -> *mut Con {
    let mut con: *mut Con = ::core::ptr::null_mut::<Con>();
    con = all_cons.tqh_first as *mut Con;
    while !con.is_null() {
        if con == target as *mut Con {
            return con;
        }
        con = (*con).all_cons.tqe_next as *mut Con;
    }
    return ::core::ptr::null_mut::<Con>();
}
#[no_mangle]
pub unsafe extern "C" fn con_exists(mut con: *mut Con) -> bool {
    return !con_by_con_id(con as ::core::ffi::c_long).is_null();
}
#[no_mangle]
pub unsafe extern "C" fn con_by_frame_id(mut frame: xcb_window_t) -> *mut Con {
    let mut con: *mut Con = ::core::ptr::null_mut::<Con>();
    con = all_cons.tqh_first as *mut Con;
    while !con.is_null() {
        if (*con).frame.id == frame {
            return con;
        }
        con = (*con).all_cons.tqe_next as *mut Con;
    }
    return ::core::ptr::null_mut::<Con>();
}
#[no_mangle]
pub unsafe extern "C" fn con_by_mark(mut mark: *const ::core::ffi::c_char) -> *mut Con {
    let mut con: *mut Con = ::core::ptr::null_mut::<Con>();
    con = all_cons.tqh_first as *mut Con;
    while !con.is_null() {
        if con_has_mark(con, mark) {
            return con;
        }
        con = (*con).all_cons.tqe_next as *mut Con;
    }
    return ::core::ptr::null_mut::<Con>();
}
#[no_mangle]
pub unsafe extern "C" fn con_find_transient_for_window(
    mut start: *mut Con,
    mut target: xcb_window_t,
) -> bool {
    let mut transient_con: *mut Con = start;
    let mut count: ::core::ffi::c_int = con_num_windows(croot);
    while !transient_con.is_null()
        && !(*transient_con).window.is_null()
        && (*(*transient_con).window).transient_for as ::core::ffi::c_long != XCB_NONE
    {
        debuglog(
            b"%s:%s:%d - transient_con = 0x%08x, transient_con->window->transient_for = 0x%08x, target = 0x%08x\n\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"con_find_transient_for_window\0" as *const u8
                as *const ::core::ffi::c_char,
            829 as ::core::ffi::c_int,
            (*(*transient_con).window).id,
            (*(*transient_con).window).transient_for,
            target,
        );
        if (*(*transient_con).window).transient_for == target {
            return true_0 != 0;
        }
        let mut next_transient: *mut Con =
            con_by_window_id((*(*transient_con).window).transient_for);
        if next_transient.is_null() {
            break;
        }
        if transient_con == next_transient {
            break;
        }
        transient_con = next_transient;
        let fresh0 = count;
        count = count - 1;
        if fresh0 <= 0 as ::core::ffi::c_int {
            break;
        }
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn con_has_mark(
    mut con: *mut Con,
    mut mark: *const ::core::ffi::c_char,
) -> bool {
    let mut current: *mut mark_t = ::core::ptr::null_mut::<mark_t>();
    current = (*con).marks_head.tqh_first as *mut mark_t;
    while !current.is_null() {
        if strcmp((*current).name, mark) == 0 as ::core::ffi::c_int {
            return true_0 != 0;
        }
        current = (*current).marks.tqe_next as *mut mark_t;
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn con_mark_toggle(
    mut con: *mut Con,
    mut mark: *const ::core::ffi::c_char,
    mut mode: mark_mode_t,
) {
    if con != ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut Con {
    } else {
        __assert_fail(
            b"con != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
            873 as ::core::ffi::c_uint,
            b"void con_mark_toggle(Con *, const char *, mark_mode_t)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    };
    debuglog(
        b"%s:%s:%d - Toggling mark \"%s\" on con = %p.\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"con_mark_toggle\0" as *const u8 as *const ::core::ffi::c_char,
        874 as ::core::ffi::c_int,
        mark,
        con,
    );
    if con_has_mark(con, mark) {
        con_unmark(con, mark);
    } else {
        con_mark(con, mark, mode);
    };
}
#[no_mangle]
pub unsafe extern "C" fn con_mark(
    mut con: *mut Con,
    mut mark: *const ::core::ffi::c_char,
    mut mode: mark_mode_t,
) {
    if con != ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut Con {
    } else {
        __assert_fail(
            b"con != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
            888 as ::core::ffi::c_uint,
            b"void con_mark(Con *, const char *, mark_mode_t)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    };
    debuglog(
        b"%s:%s:%d - Setting mark \"%s\" on con = %p.\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"con_mark\0" as *const u8 as *const ::core::ffi::c_char,
        889 as ::core::ffi::c_int,
        mark,
        con,
    );
    con_unmark(::core::ptr::null_mut::<Con>(), mark);
    if mode as ::core::ffi::c_uint == MM_REPLACE as ::core::ffi::c_int as ::core::ffi::c_uint {
        debuglog(
            b"%s:%s:%d - Removing all existing marks on con = %p.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"con_mark\0" as *const u8 as *const ::core::ffi::c_char,
            893 as ::core::ffi::c_int,
            con,
        );
        while !(*con).marks_head.tqh_first.is_null() {
            let mut current: *const mark_t = (*con).marks_head.tqh_first;
            con_unmark(con, (*current).name);
        }
    }
    let mut new: *mut mark_t =
        scalloc(1 as size_t, ::core::mem::size_of::<mark_t>() as size_t) as *mut mark_t;
    (*new).name = sstrdup(mark);
    (*new).marks.tqe_next = ::core::ptr::null_mut::<mark_t>();
    (*new).marks.tqe_prev = (*con).marks_head.tqh_last;
    *(*con).marks_head.tqh_last = new as *mut mark_t;
    (*con).marks_head.tqh_last = &raw mut (*new).marks.tqe_next;
    ipc_send_window_event(b"mark\0" as *const u8 as *const ::core::ffi::c_char, con);
    (*con).mark_changed = true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn con_unmark(mut con: *mut Con, mut name: *const ::core::ffi::c_char) {
    let mut current: *mut Con = ::core::ptr::null_mut::<Con>();
    if name.is_null() {
        debuglog(
            b"%s:%s:%d - Unmarking all containers.\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"con_unmark\0" as *const u8 as *const ::core::ffi::c_char,
            919 as ::core::ffi::c_int,
        );
        current = all_cons.tqh_first as *mut Con;
        while !current.is_null() {
            if !(!con.is_null() && current != con) {
                if !(*current).marks_head.tqh_first.is_null() {
                    while !(*current).marks_head.tqh_first.is_null() {
                        let mut mark: *mut mark_t = (*current).marks_head.tqh_first as *mut mark_t;
                        free((*mark).name as *mut ::core::ffi::c_void);
                        (*mark).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
                        if !(*mark).marks.tqe_next.is_null() {
                            (*(*mark).marks.tqe_next).marks.tqe_prev = (*mark).marks.tqe_prev;
                        } else {
                            (*current).marks_head.tqh_last = (*mark).marks.tqe_prev;
                        }
                        *(*mark).marks.tqe_prev = (*mark).marks.tqe_next;
                        free(mark as *mut ::core::ffi::c_void);
                        mark = ::core::ptr::null_mut::<mark_t>();
                        ipc_send_window_event(
                            b"mark\0" as *const u8 as *const ::core::ffi::c_char,
                            current,
                        );
                    }
                    (*current).mark_changed = true_0 != 0;
                }
            }
            current = (*current).all_cons.tqe_next as *mut Con;
        }
    } else {
        debuglog(
            b"%s:%s:%d - Removing mark \"%s\".\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"con_unmark\0" as *const u8 as *const ::core::ffi::c_char,
            941 as ::core::ffi::c_int,
            name,
        );
        current = if con.is_null() {
            con_by_mark(name)
        } else {
            con
        };
        if current.is_null() {
            debuglog(
                b"%s:%s:%d - No container found with this mark, so there is nothing to do.\n\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"con_unmark\0" as *const u8 as *const ::core::ffi::c_char,
                944 as ::core::ffi::c_int,
            );
            return;
        }
        debuglog(
            b"%s:%s:%d - Found mark on con = %p. Removing it now.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"con_unmark\0" as *const u8 as *const ::core::ffi::c_char,
            948 as ::core::ffi::c_int,
            current,
        );
        (*current).mark_changed = true_0 != 0;
        let mut mark_0: *mut mark_t = ::core::ptr::null_mut::<mark_t>();
        mark_0 = (*current).marks_head.tqh_first as *mut mark_t;
        while !mark_0.is_null() {
            if strcmp((*mark_0).name, name) != 0 as ::core::ffi::c_int {
                mark_0 = (*mark_0).marks.tqe_next as *mut mark_t;
            } else {
                free((*mark_0).name as *mut ::core::ffi::c_void);
                (*mark_0).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
                if !(*mark_0).marks.tqe_next.is_null() {
                    (*(*mark_0).marks.tqe_next).marks.tqe_prev = (*mark_0).marks.tqe_prev;
                } else {
                    (*current).marks_head.tqh_last = (*mark_0).marks.tqe_prev;
                }
                *(*mark_0).marks.tqe_prev = (*mark_0).marks.tqe_next;
                free(mark_0 as *mut ::core::ffi::c_void);
                mark_0 = ::core::ptr::null_mut::<mark_t>();
                ipc_send_window_event(
                    b"mark\0" as *const u8 as *const ::core::ffi::c_char,
                    current,
                );
                break;
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn con_for_window(
    mut con: *mut Con,
    mut window: *mut i3Window,
    mut store_match: *mut *mut Match,
) -> *mut Con {
    let mut child: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut match_0: *mut Match = ::core::ptr::null_mut::<Match>();
    child = (*con).nodes_head.tqh_first as *mut Con;
    while !child.is_null() {
        match_0 = (*child).swallow_head.tqh_first as *mut Match;
        while !match_0.is_null() {
            if !match_matches_window(match_0, window) {
                match_0 = (*match_0).matches.tqe_next as *mut Match;
            } else {
                if !store_match.is_null() {
                    *store_match = match_0;
                }
                return child;
            }
        }
        let mut result: *mut Con = con_for_window(child, window, store_match);
        if !result.is_null() {
            return result;
        }
        child = (*child).nodes.tqe_next as *mut Con;
    }
    child = (*con).floating_head.tqh_first as *mut Con;
    while !child.is_null() {
        match_0 = (*child).swallow_head.tqh_first as *mut Match;
        while !match_0.is_null() {
            if !match_matches_window(match_0, window) {
                match_0 = (*match_0).matches.tqe_next as *mut Match;
            } else {
                if !store_match.is_null() {
                    *store_match = match_0;
                }
                return child;
            }
        }
        let mut result_0: *mut Con = con_for_window(child, window, store_match);
        if !result_0.is_null() {
            return result_0;
        }
        child = (*child).floating_windows.tqe_next as *mut Con;
    }
    return ::core::ptr::null_mut::<Con>();
}
unsafe extern "C" fn num_focus_heads(mut con: *mut Con) -> ::core::ffi::c_int {
    let mut focus_heads: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut current: *mut Con = ::core::ptr::null_mut::<Con>();
    current = (*con).focus_head.tqh_first as *mut Con;
    while !current.is_null() {
        focus_heads += 1;
        current = (*current).focused.tqe_next as *mut Con;
    }
    return focus_heads;
}
#[no_mangle]
pub unsafe extern "C" fn get_focus_order(mut con: *mut Con) -> *mut *mut Con {
    let focus_heads: ::core::ffi::c_int = num_focus_heads(con) as ::core::ffi::c_int;
    let mut focus_order: *mut *mut Con =
        smalloc((focus_heads as size_t).wrapping_mul(::core::mem::size_of::<*mut Con>() as size_t))
            as *mut *mut Con;
    let mut current: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut idx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    current = (*con).focus_head.tqh_first as *mut Con;
    while !current.is_null() {
        if idx < focus_heads {
        } else {
            __assert_fail(
                b"idx < focus_heads\0" as *const u8 as *const ::core::ffi::c_char,
                b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
                1033 as ::core::ffi::c_uint,
                b"Con **get_focus_order(Con *)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        };
        let fresh1 = idx;
        idx = idx + 1;
        let ref mut fresh2 = *focus_order.offset(fresh1 as isize);
        *fresh2 = current;
        current = (*current).focused.tqe_next as *mut Con;
    }
    return focus_order;
}
#[no_mangle]
pub unsafe extern "C" fn set_focus_order(mut con: *mut Con, mut focus_order: *mut *mut Con) {
    let mut focus_heads: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while !(*con).focus_head.tqh_first.is_null() {
        let mut current: *mut Con = (*con).focus_head.tqh_first as *mut Con;
        if !(*current).focused.tqe_next.is_null() {
            (*(*current).focused.tqe_next).focused.tqe_prev = (*current).focused.tqe_prev;
        } else {
            (*con).focus_head.tqh_last = (*current).focused.tqe_prev;
        }
        *(*current).focused.tqe_prev = (*current).focused.tqe_next;
        focus_heads += 1;
    }
    let mut idx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while idx < focus_heads {
        if (*con).type_0 as ::core::ffi::c_uint
            != CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
            && !con_inside_floating(*focus_order.offset(idx as isize)).is_null()
        {
            focus_heads += 1;
        } else {
            let ref mut fresh3 = (**focus_order.offset(idx as isize)).focused.tqe_next;
            *fresh3 = ::core::ptr::null_mut::<Con>();
            let ref mut fresh4 = (**focus_order.offset(idx as isize)).focused.tqe_prev;
            *fresh4 = (*con).focus_head.tqh_last;
            *(*con).focus_head.tqh_last = *focus_order.offset(idx as isize) as *mut Con;
            (*con).focus_head.tqh_last =
                &raw mut (**focus_order.offset(idx as isize)).focused.tqe_next;
        }
        idx += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn con_num_children(mut con: *mut Con) -> ::core::ffi::c_int {
    let mut child: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut children: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    child = (*con).nodes_head.tqh_first as *mut Con;
    while !child.is_null() {
        children += 1;
        child = (*child).nodes.tqe_next as *mut Con;
    }
    return children;
}
#[no_mangle]
pub unsafe extern "C" fn con_num_visible_children(mut con: *mut Con) -> ::core::ffi::c_int {
    if con.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    let mut children: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut current: *mut Con = ::core::ptr::null_mut::<Con>();
    current = (*con).nodes_head.tqh_first as *mut Con;
    while !current.is_null() {
        if !con_is_hidden(current) && con_is_leaf(current) as ::core::ffi::c_int != 0 {
            children += 1;
        } else {
            children += con_num_visible_children(current);
        }
        current = (*current).nodes.tqe_next as *mut Con;
    }
    return children;
}
#[no_mangle]
pub unsafe extern "C" fn con_num_windows(mut con: *mut Con) -> ::core::ffi::c_int {
    if con.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if con_has_managed_window(con) {
        return 1 as ::core::ffi::c_int;
    }
    let mut num: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut current: *mut Con = ::core::ptr::null_mut::<Con>();
    current = (*con).nodes_head.tqh_first as *mut Con;
    while !current.is_null() {
        num += con_num_windows(current);
        current = (*current).nodes.tqe_next as *mut Con;
    }
    current = (*con).floating_head.tqh_first as *mut Con;
    while !current.is_null() {
        num += con_num_windows(current);
        current = (*current).floating_windows.tqe_next as *mut Con;
    }
    return num;
}
#[no_mangle]
pub unsafe extern "C" fn con_fix_percent(mut con: *mut Con) {
    let mut child: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut children: ::core::ffi::c_int = con_num_children(con);
    let mut total: ::core::ffi::c_double = 0.0f64;
    let mut children_with_percent: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    child = (*con).nodes_head.tqh_first as *mut Con;
    while !child.is_null() {
        if (*child).percent > 0.0f64 {
            total += (*child).percent;
            children_with_percent += 1;
        }
        child = (*child).nodes.tqe_next as *mut Con;
    }
    if children_with_percent != children {
        child = (*con).nodes_head.tqh_first as *mut Con;
        while !child.is_null() {
            if (*child).percent <= 0.0f64 {
                if children_with_percent == 0 as ::core::ffi::c_int {
                    (*child).percent = 1.0f64;
                    total += (*child).percent;
                } else {
                    (*child).percent = total / children_with_percent as ::core::ffi::c_double;
                    total += (*child).percent;
                }
            }
            child = (*child).nodes.tqe_next as *mut Con;
        }
    }
    if total == 0.0f64 {
        child = (*con).nodes_head.tqh_first as *mut Con;
        while !child.is_null() {
            (*child).percent = 1.0f64 / children as ::core::ffi::c_double;
            child = (*child).nodes.tqe_next as *mut Con;
        }
    } else if total != 1.0f64 {
        child = (*con).nodes_head.tqh_first as *mut Con;
        while !child.is_null() {
            (*child).percent /= total;
            child = (*child).nodes.tqe_next as *mut Con;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn con_toggle_fullscreen(
    mut con: *mut Con,
    mut fullscreen_mode: ::core::ffi::c_int,
) {
    if (*con).type_0 as ::core::ffi::c_uint
        == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        debuglog(
            b"%s:%s:%d - You cannot make a workspace fullscreen.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"con_toggle_fullscreen\0" as *const u8 as *const ::core::ffi::c_char,
            1190 as ::core::ffi::c_int,
        );
        return;
    }
    debuglog(
        b"%s:%s:%d - toggling fullscreen for %p / %s\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"con_toggle_fullscreen\0" as *const u8 as *const ::core::ffi::c_char,
        1194 as ::core::ffi::c_int,
        con,
        (*con).name,
    );
    if (*con).fullscreen_mode as ::core::ffi::c_uint
        == CF_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        con_enable_fullscreen(con, fullscreen_mode as fullscreen_mode_t);
    } else {
        con_disable_fullscreen(con);
    };
}
unsafe extern "C" fn con_set_fullscreen_mode(
    mut con: *mut Con,
    mut fullscreen_mode: fullscreen_mode_t,
) {
    (*con).fullscreen_mode = fullscreen_mode;
    debuglog(
        b"%s:%s:%d - mode now: %d\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"con_set_fullscreen_mode\0" as *const u8 as *const ::core::ffi::c_char,
        1212 as ::core::ffi::c_int,
        (*con).fullscreen_mode as ::core::ffi::c_uint,
    );
    ipc_send_window_event(
        b"fullscreen_mode\0" as *const u8 as *const ::core::ffi::c_char,
        con,
    );
    if (*con).window.is_null() {
        return;
    }
    if (*con).fullscreen_mode as ::core::ffi::c_uint
        != CF_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        debuglog(
            b"%s:%s:%d - Setting _NET_WM_STATE_FULLSCREEN for con = %p / window = %d.\n\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"con_set_fullscreen_mode\0" as *const u8 as *const ::core::ffi::c_char,
            1225 as ::core::ffi::c_int,
            con,
            (*(*con).window).id,
        );
        xcb_add_property_atom(
            conn,
            (*(*con).window).id,
            A__NET_WM_STATE,
            A__NET_WM_STATE_FULLSCREEN,
        );
    } else {
        debuglog(
            b"%s:%s:%d - Removing _NET_WM_STATE_FULLSCREEN for con = %p / window = %d.\n\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"con_set_fullscreen_mode\0" as *const u8 as *const ::core::ffi::c_char,
            1228 as ::core::ffi::c_int,
            con,
            (*(*con).window).id,
        );
        xcb_remove_property_atom(
            conn,
            (*(*con).window).id,
            A__NET_WM_STATE,
            A__NET_WM_STATE_FULLSCREEN,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn con_enable_fullscreen(
    mut con: *mut Con,
    mut fullscreen_mode: fullscreen_mode_t,
) {
    if (*con).type_0 as ::core::ffi::c_uint
        == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        debuglog(
            b"%s:%s:%d - You cannot make a workspace fullscreen.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"con_enable_fullscreen\0" as *const u8 as *const ::core::ffi::c_char,
            1246 as ::core::ffi::c_int,
        );
        return;
    }
    if fullscreen_mode as ::core::ffi::c_uint
        == CF_GLOBAL as ::core::ffi::c_int as ::core::ffi::c_uint
        || fullscreen_mode as ::core::ffi::c_uint
            == CF_OUTPUT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
    } else {
        __assert_fail(
            b"fullscreen_mode == CF_GLOBAL || fullscreen_mode == CF_OUTPUT\0" as *const u8
                as *const ::core::ffi::c_char,
            b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
            1250 as ::core::ffi::c_uint,
            b"void con_enable_fullscreen(Con *, fullscreen_mode_t)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    };
    if fullscreen_mode as ::core::ffi::c_uint
        == CF_GLOBAL as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        debuglog(
            b"%s:%s:%d - enabling global fullscreen for %p / %s\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"con_enable_fullscreen\0" as *const u8 as *const ::core::ffi::c_char,
            1253 as ::core::ffi::c_int,
            con,
            (*con).name,
        );
    } else {
        debuglog(
            b"%s:%s:%d - enabling fullscreen for %p / %s\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"con_enable_fullscreen\0" as *const u8 as *const ::core::ffi::c_char,
            1255 as ::core::ffi::c_int,
            con,
            (*con).name,
        );
    }
    if (*con).fullscreen_mode as ::core::ffi::c_uint == fullscreen_mode as ::core::ffi::c_uint {
        debuglog(
            b"%s:%s:%d - fullscreen already enabled for %p / %s\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"con_enable_fullscreen\0" as *const u8 as *const ::core::ffi::c_char,
            1259 as ::core::ffi::c_int,
            con,
            (*con).name,
        );
        return;
    }
    let mut con_ws: *mut Con = con_get_workspace(con);
    let mut fullscreen: *mut Con = con_get_fullscreen_con(croot, CF_GLOBAL);
    if fullscreen.is_null() {
        fullscreen = con_get_fullscreen_con(con_ws, CF_OUTPUT);
    }
    if !fullscreen.is_null() {
        con_disable_fullscreen(fullscreen);
    }
    let mut cur_ws: *mut Con = con_get_workspace(focused);
    let mut old_focused: *mut Con = focused;
    if fullscreen_mode as ::core::ffi::c_uint
        == CF_GLOBAL as ::core::ffi::c_int as ::core::ffi::c_uint
        && cur_ws != con_ws
    {
        workspace_show(con_ws);
    }
    con_activate(con);
    if fullscreen_mode as ::core::ffi::c_uint
        != CF_GLOBAL as ::core::ffi::c_int as ::core::ffi::c_uint
        && cur_ws != con_ws
    {
        con_activate(old_focused);
    }
    con_set_fullscreen_mode(con, fullscreen_mode);
}
#[no_mangle]
pub unsafe extern "C" fn con_disable_fullscreen(mut con: *mut Con) {
    if (*con).type_0 as ::core::ffi::c_uint
        == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        debuglog(
            b"%s:%s:%d - You cannot make a workspace fullscreen.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"con_disable_fullscreen\0" as *const u8 as *const ::core::ffi::c_char,
            1297 as ::core::ffi::c_int,
        );
        return;
    }
    debuglog(
        b"%s:%s:%d - disabling fullscreen for %p / %s\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"con_disable_fullscreen\0" as *const u8 as *const ::core::ffi::c_char,
        1301 as ::core::ffi::c_int,
        con,
        (*con).name,
    );
    if (*con).fullscreen_mode as ::core::ffi::c_uint
        == CF_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        debuglog(
            b"%s:%s:%d - fullscreen already disabled for %p / %s\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"con_disable_fullscreen\0" as *const u8 as *const ::core::ffi::c_char,
            1304 as ::core::ffi::c_int,
            con,
            (*con).name,
        );
        return;
    }
    con_set_fullscreen_mode(con, CF_NONE);
}
unsafe extern "C" fn _con_move_to_con(
    mut con: *mut Con,
    mut target: *mut Con,
    mut behind_focused: bool,
    mut fix_coordinates: bool,
    mut dont_warp: bool,
    mut ignore_focus: bool,
    mut fix_percentage: bool,
) -> bool {
    let mut orig_target: *mut Con = target;
    let mut target_ws: *mut Con = con_get_workspace(target);
    if !ignore_focus && !con_fullscreen_permits_focusing(target_ws) {
        verboselog(
            b"Cannot move out of a fullscreen container.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        return false_0 != 0;
    }
    if con_is_floating(con) {
        debuglog(
            b"%s:%s:%d - Container is floating, using parent instead.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"_con_move_to_con\0" as *const u8 as *const ::core::ffi::c_char,
            1322 as ::core::ffi::c_int,
        );
        con = (*con).parent as *mut Con;
    }
    let mut source_ws: *mut Con = con_get_workspace(con);
    if (*con).type_0 as ::core::ffi::c_uint
        == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        while !(*source_ws).floating_head.tqh_first.is_null() {
            let mut child: *mut Con = (*source_ws).floating_head.tqh_first as *mut Con;
            con_move_to_workspace(child, target_ws, true_0 != 0, true_0 != 0, false_0 != 0);
        }
        if con_is_leaf(con) {
            return false_0 != 0;
        }
        con = workspace_encapsulate(con);
        if con.is_null() {
            errorlog(
                b"ERROR: Workspace failed to move its contents into a container!\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
            return false_0 != 0;
        }
    }
    let mut urgent: bool = (*con).urgent;
    let mut current_ws: *mut Con = con_get_workspace(focused);
    let mut source_output: *mut Con = con_get_output(con);
    let mut dest_output: *mut Con = con_get_output(target_ws);
    let mut focus_next: *mut Con = ::core::ptr::null_mut::<Con>();
    if !ignore_focus && source_ws == current_ws && target_ws != source_ws {
        focus_next = con_descend_focused(source_ws);
        if focus_next == con || con_has_parent(focus_next, con) as ::core::ffi::c_int != 0 {
            focus_next = con_next_focused(con);
        }
    }
    if (*target).type_0 as ::core::ffi::c_uint
        != CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        debuglog(
            b"%s:%s:%d - target originally = %p / %s / type %d\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"_con_move_to_con\0" as *const u8 as *const ::core::ffi::c_char,
            1369 as ::core::ffi::c_int,
            target,
            (*target).name,
            (*target).type_0 as ::core::ffi::c_uint,
        );
        target = (*target).parent as *mut Con;
    }
    if (*target).type_0 as ::core::ffi::c_uint
        == CT_FLOATING_CON as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        debuglog(
            b"%s:%s:%d - floatingcon, going up even further\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"_con_move_to_con\0" as *const u8 as *const ::core::ffi::c_char,
            1377 as ::core::ffi::c_int,
        );
        orig_target = target;
        target = (*target).parent as *mut Con;
    }
    if (*con).type_0 as ::core::ffi::c_uint
        == CT_FLOATING_CON as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut ws: *mut Con = con_get_workspace(target);
        debuglog(
            b"%s:%s:%d - This is a floating window, using workspace %p / %s\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"_con_move_to_con\0" as *const u8 as *const ::core::ffi::c_char,
            1384 as ::core::ffi::c_int,
            ws,
            (*ws).name,
        );
        target = ws;
    }
    if source_output != dest_output {
        if fix_coordinates as ::core::ffi::c_int != 0
            && (*con).type_0 as ::core::ffi::c_uint
                == CT_FLOATING_CON as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            floating_fix_coordinates(
                con,
                &raw mut (*source_output).rect,
                &raw mut (*dest_output).rect,
            );
        } else {
            debuglog(
                b"%s:%s:%d - Not fixing coordinates, fix_coordinates flag = %d\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"_con_move_to_con\0" as *const u8 as *const ::core::ffi::c_char,
                1394 as ::core::ffi::c_int,
                fix_coordinates as ::core::ffi::c_int,
            );
        }
    }
    let mut fullscreen: *mut Con = con_get_fullscreen_con(target_ws, CF_OUTPUT);
    let con_has_fullscreen: bool = (*con).fullscreen_mode as ::core::ffi::c_uint
        != CF_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
        || !con_get_fullscreen_con(con, CF_GLOBAL).is_null()
        || !con_get_fullscreen_con(con, CF_OUTPUT).is_null();
    if con_has_fullscreen as ::core::ffi::c_int != 0 && !fullscreen.is_null() {
        con_toggle_fullscreen(fullscreen, CF_OUTPUT as ::core::ffi::c_int);
        fullscreen = ::core::ptr::null_mut::<Con>();
    }
    debuglog(
        b"%s:%s:%d - Re-attaching container to %p / %s\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"_con_move_to_con\0" as *const u8 as *const ::core::ffi::c_char,
        1414 as ::core::ffi::c_int,
        target,
        (*target).name,
    );
    let mut parent: *mut Con = (*con).parent as *mut Con;
    con_detach(con);
    _con_attach(
        con,
        target,
        if behind_focused as ::core::ffi::c_int != 0 {
            ::core::ptr::null_mut::<Con>()
        } else {
            orig_target
        },
        !behind_focused,
    );
    if fix_percentage {
        con_fix_percent(parent);
        (*con).percent = 0.0f64;
        con_fix_percent(target);
    }
    if !ignore_focus && !con_is_internal(target_ws) && fullscreen.is_null() {
        let mut old_focus_ws: *mut Con =
            (*output_get_content(dest_output)).focus_head.tqh_first as *mut Con;
        let mut old_focus: *mut Con = focused;
        con_activate(con_descend_focused(con));
        if old_focus_ws == current_ws
            && (*old_focus).type_0 as ::core::ffi::c_uint
                != CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            con_activate(old_focus);
        } else if con_get_workspace(focused) != old_focus_ws {
            con_focus(con_descend_focused(old_focus_ws));
        }
    }
    if !ignore_focus {
        workspace_show(current_ws);
        if dont_warp {
            debuglog(
                b"%s:%s:%d - x_set_warp_to(NULL) because dont_warp is set\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"_con_move_to_con\0" as *const u8 as *const ::core::ffi::c_char,
                1455 as ::core::ffi::c_int,
            );
            x_set_warp_to(::core::ptr::null_mut::<Rect>());
        }
    }
    if !focus_next.is_null() {
        con_activate(con_descend_focused(focus_next));
    }
    if !con_is_leaf(con) {
        let mut child_0: *mut Con = ::core::ptr::null_mut::<Con>();
        child_0 = (*con).nodes_head.tqh_first as *mut Con;
        while !child_0.is_null() {
            if !(*child_0).window.is_null() {
                startup_sequence_delete_by_window((*child_0).window);
            }
            child_0 = (*child_0).nodes.tqe_next as *mut Con;
        }
    }
    if !(*con).window.is_null() {
        startup_sequence_delete_by_window((*con).window);
    }
    if urgent {
        workspace_update_urgent_flag(source_ws);
        con_set_urgency(con, true_0 != 0);
    }
    free((*con).deco_render_params as *mut ::core::ffi::c_void);
    (*con).deco_render_params = ::core::ptr::null_mut::<deco_render_params>();
    (*parent)
        .on_remove_child
        .expect("non-null function pointer")(parent);
    ipc_send_window_event(b"move\0" as *const u8 as *const ::core::ffi::c_char, con);
    ewmh_update_wm_desktop();
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn con_move_to_target(mut con: *mut Con, mut target: *mut Con) -> bool {
    if con_get_workspace(target)
        == workspace_get(b"__i3_scratch\0" as *const u8 as *const ::core::ffi::c_char)
    {
        debuglog(
            b"%s:%s:%d - target container is in the scratchpad, moving container to scratchpad.\n\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"con_move_to_target\0" as *const u8 as *const ::core::ffi::c_char,
            1501 as ::core::ffi::c_int,
        );
        scratchpad_move(con);
        return true_0 != 0;
    }
    if con_is_floating(target) {
        debuglog(
            b"%s:%s:%d - target container is floating, moving container to target's workspace.\n\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"con_move_to_target\0" as *const u8 as *const ::core::ffi::c_char,
            1508 as ::core::ffi::c_int,
        );
        con_move_to_workspace(
            con,
            con_get_workspace(target),
            true_0 != 0,
            false_0 != 0,
            false_0 != 0,
        );
        return true_0 != 0;
    }
    if (*target).type_0 as ::core::ffi::c_uint
        == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
        && con_is_leaf(target) as ::core::ffi::c_int != 0
    {
        debuglog(
            b"%s:%s:%d - target container is an empty workspace, simply moving the container there.\n\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"con_move_to_target\0" as *const u8 as *const ::core::ffi::c_char,
            1514 as ::core::ffi::c_int,
        );
        con_move_to_workspace(con, target, true_0 != 0, false_0 != 0, false_0 != 0);
        return true_0 != 0;
    }
    if con_is_split(target) {
        debuglog(
            b"%s:%s:%d - target is a split container, descending to the currently focused child.\n\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"con_move_to_target\0" as *const u8 as *const ::core::ffi::c_char,
            1523 as ::core::ffi::c_int,
        );
        target = (*target).focus_head.tqh_first as *mut Con;
    }
    if con == target || con_has_parent(target, con) as ::core::ffi::c_int != 0 {
        debuglog(
            b"%s:%s:%d - cannot move the container to or inside itself, aborting.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"con_move_to_target\0" as *const u8 as *const ::core::ffi::c_char,
            1528 as ::core::ffi::c_int,
        );
        return false_0 != 0;
    }
    return _con_move_to_con(
        con,
        target,
        false_0 != 0,
        true_0 != 0,
        false_0 != 0,
        false_0 != 0,
        true_0 != 0,
    );
}
#[no_mangle]
pub unsafe extern "C" fn con_move_to_mark(
    mut con: *mut Con,
    mut mark: *const ::core::ffi::c_char,
) -> bool {
    let mut target: *mut Con = con_by_mark(mark);
    if target.is_null() {
        debuglog(
            b"%s:%s:%d - found no container with mark \"%s\"\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"con_move_to_mark\0" as *const u8 as *const ::core::ffi::c_char,
            1542 as ::core::ffi::c_int,
            mark,
        );
        return false_0 != 0;
    }
    return con_move_to_target(con, target);
}
#[no_mangle]
pub unsafe extern "C" fn con_move_to_workspace(
    mut con: *mut Con,
    mut workspace: *mut Con,
    mut fix_coordinates: bool,
    mut dont_warp: bool,
    mut ignore_focus: bool,
) {
    if (*workspace).type_0 as ::core::ffi::c_uint
        == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
    } else {
        __assert_fail(
            b"workspace->type == CT_WORKSPACE\0" as *const u8 as *const ::core::ffi::c_char,
            b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
            1570 as ::core::ffi::c_uint,
            b"void con_move_to_workspace(Con *, Con *, _Bool, _Bool, _Bool)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    };
    let mut source_ws: *mut Con = con_get_workspace(con);
    if workspace == source_ws {
        debuglog(
            b"%s:%s:%d - Not moving, already there\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"con_move_to_workspace\0" as *const u8 as *const ::core::ffi::c_char,
            1574 as ::core::ffi::c_int,
        );
        return;
    }
    let mut target: *mut Con = con_descend_focused(workspace);
    _con_move_to_con(
        con,
        target,
        true_0 != 0,
        fix_coordinates,
        dont_warp,
        ignore_focus,
        true_0 != 0,
    );
}
#[no_mangle]
pub unsafe extern "C" fn con_move_to_output(
    mut con: *mut Con,
    mut output: *mut Output,
    mut fix_coordinates: bool,
) {
    let mut ws: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut child: *mut Con = -(1 as ::core::ffi::c_int) as *mut Con;
    while child == -(1 as ::core::ffi::c_int) as *mut Con && {
        child = ::core::ptr::null_mut::<Con>();
        true_0 != 0
    } {
        child = (*output_get_content((*output).con)).nodes_head.tqh_first as *mut Con;
        while !child.is_null() {
            if !workspace_is_visible(child) {
                child = (*child).nodes.tqe_next as *mut Con;
            } else {
                ws = child;
                break;
            }
        }
    }
    if ws != ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut Con {
    } else {
        __assert_fail(
            b"ws != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
            1590 as ::core::ffi::c_uint,
            b"void con_move_to_output(Con *, Output *, _Bool)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    };
    debuglog(
        b"%s:%s:%d - Moving con %p to output %s\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"con_move_to_output\0" as *const u8 as *const ::core::ffi::c_char,
        1591 as ::core::ffi::c_int,
        con,
        output_primary_name(output),
    );
    con_move_to_workspace(con, ws, fix_coordinates, false_0 != 0, false_0 != 0);
}
#[no_mangle]
pub unsafe extern "C" fn con_move_to_output_name(
    mut con: *mut Con,
    mut name: *const ::core::ffi::c_char,
    mut fix_coordinates: bool,
) -> bool {
    let mut current_output: *mut Output = get_output_for_con(con);
    let mut output: *mut Output = get_output_from_string(current_output, name);
    if output.is_null() {
        errorlog(
            b"ERROR: Could not find output \"%s\"\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            name,
        );
        return false_0 != 0;
    }
    con_move_to_output(con, output, fix_coordinates);
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn con_orientation(mut con: *mut Con) -> orientation_t {
    let mut current_block_5: u64;
    match (*con).layout as ::core::ffi::c_uint {
        5 | 1 => return VERT,
        6 | 2 => return HORIZ,
        0 => {
            errorlog(
                b"ERROR: Someone called con_orientation() on a con with L_DEFAULT, this is a bug in the code.\n\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
            if 0 as ::core::ffi::c_int != 0 {
            } else {
                __assert_fail(
                    b"false\0" as *const u8 as *const ::core::ffi::c_char,
                    b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
                    1634 as ::core::ffi::c_uint,
                    b"orientation_t con_orientation(Con *)\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            };
            current_block_5 = 10837979787936441410;
        }
        3 | 4 => {
            current_block_5 = 10837979787936441410;
        }
        _ => {
            current_block_5 = 2473556513754201174;
        }
    }
    match current_block_5 {
        10837979787936441410 => {
            errorlog(
                b"ERROR: con_orientation() called on dockarea/output (%d) container %p\n\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                (*con).layout as ::core::ffi::c_uint,
                con,
            );
            if 0 as ::core::ffi::c_int != 0 {
            } else {
                __assert_fail(
                    b"false\0" as *const u8 as *const ::core::ffi::c_char,
                    b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
                    1639 as ::core::ffi::c_uint,
                    b"orientation_t con_orientation(Con *)\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            };
        }
        _ => {}
    }
    if 0 as ::core::ffi::c_int != 0 {
    } else {
        __assert_fail(
            b"false\0" as *const u8 as *const ::core::ffi::c_char,
            b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
            1642 as ::core::ffi::c_uint,
            b"orientation_t con_orientation(Con *)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
    panic!("Reached end of non-void function without returning");
}
#[no_mangle]
pub unsafe extern "C" fn con_next_focused(mut con: *mut Con) -> *mut Con {
    if (*(*con).parent).type_0 as ::core::ffi::c_uint
        == CT_DOCKAREA as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        debuglog(
            b"%s:%s:%d - selecting workspace for dock client\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"con_next_focused\0" as *const u8 as *const ::core::ffi::c_char,
            1654 as ::core::ffi::c_int,
        );
        return con_descend_focused(output_get_content((*(*con).parent).parent as *mut Con));
    }
    if con_is_floating(con) {
        con = (*con).parent as *mut Con;
    }
    let mut next: *mut Con = (*(*con).parent).focus_head.tqh_first as *mut Con;
    if next != con {
        debuglog(
            b"%s:%s:%d - Using first entry %p\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"con_next_focused\0" as *const u8 as *const ::core::ffi::c_char,
            1665 as ::core::ffi::c_int,
            next,
        );
    } else {
        next = (*con).focused.tqe_next as *mut Con;
        if next.is_null() {
            next = (*con).parent as *mut Con;
        }
    }
    while !(*next).focus_head.tqh_first.is_null() && (*next).focus_head.tqh_first != con {
        next = (*next).focus_head.tqh_first as *mut Con;
    }
    if (*con).type_0 as ::core::ffi::c_uint
        == CT_FLOATING_CON as ::core::ffi::c_int as ::core::ffi::c_uint
        && next != (*con).parent
    {
        next = con_descend_focused(next);
    }
    return next;
}
#[no_mangle]
pub unsafe extern "C" fn con_descend_focused(mut con: *mut Con) -> *mut Con {
    let mut next: *mut Con = con;
    while next != focused && !(*next).focus_head.tqh_first.is_null() {
        next = (*next).focus_head.tqh_first as *mut Con;
    }
    return next;
}
#[no_mangle]
pub unsafe extern "C" fn con_descend_tiling_focused(mut con: *mut Con) -> *mut Con {
    let mut next: *mut Con = con;
    let mut before: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut child: *mut Con = ::core::ptr::null_mut::<Con>();
    if next == focused {
        return next;
    }
    loop {
        before = next;
        child = (*next).focus_head.tqh_first as *mut Con;
        while !child.is_null() {
            if (*child).type_0 as ::core::ffi::c_uint
                == CT_FLOATING_CON as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                child = (*child).focused.tqe_next as *mut Con;
            } else {
                next = child;
                break;
            }
        }
        if !(before != next && next != focused) {
            break;
        }
    }
    return next;
}
#[no_mangle]
pub unsafe extern "C" fn con_descend_direction(
    mut con: *mut Con,
    mut direction: direction_t,
) -> *mut Con {
    let mut most: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut current: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut orientation: ::core::ffi::c_int = con_orientation(con) as ::core::ffi::c_int;
    debuglog(
        b"%s:%s:%d - con_descend_direction(%p, orientation %d, direction %d)\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"con_descend_direction\0" as *const u8 as *const ::core::ffi::c_char,
        1741 as ::core::ffi::c_int,
        con,
        orientation,
        direction as ::core::ffi::c_uint,
    );
    if direction as ::core::ffi::c_uint == D_LEFT as ::core::ffi::c_int as ::core::ffi::c_uint
        || direction as ::core::ffi::c_uint == D_RIGHT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if orientation == HORIZ as ::core::ffi::c_int {
            if direction as ::core::ffi::c_uint
                == D_RIGHT as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                most = (*con).nodes_head.tqh_first as *mut Con;
            } else {
                most = *(*((*con).nodes_head.tqh_last as *mut nodes_head)).tqh_last as *mut Con;
            }
        } else if orientation == VERT as ::core::ffi::c_int {
            current = (*con).focus_head.tqh_first as *mut Con;
            while !current.is_null() {
                if (*current).type_0 as ::core::ffi::c_uint
                    != CT_FLOATING_CON as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    most = current;
                    break;
                } else {
                    current = (*current).focused.tqe_next as *mut Con;
                }
            }
        } else {
            return con;
        }
    }
    if direction as ::core::ffi::c_uint == D_UP as ::core::ffi::c_int as ::core::ffi::c_uint
        || direction as ::core::ffi::c_uint == D_DOWN as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if orientation == VERT as ::core::ffi::c_int {
            if direction as ::core::ffi::c_uint == D_UP as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                most = *(*((*con).nodes_head.tqh_last as *mut nodes_head)).tqh_last as *mut Con;
            } else {
                most = (*con).nodes_head.tqh_first as *mut Con;
            }
        } else if orientation == HORIZ as ::core::ffi::c_int {
            current = (*con).focus_head.tqh_first as *mut Con;
            while !current.is_null() {
                if (*current).type_0 as ::core::ffi::c_uint
                    != CT_FLOATING_CON as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    most = current;
                    break;
                } else {
                    current = (*current).focused.tqe_next as *mut Con;
                }
            }
        } else {
            return con;
        }
    }
    if most.is_null() {
        return con;
    }
    return con_descend_direction(most, direction);
}
unsafe extern "C" fn has_outer_gaps(mut gaps: gaps_t) -> bool {
    return gaps.top > 0 as ::core::ffi::c_int
        || gaps.right > 0 as ::core::ffi::c_int
        || gaps.bottom > 0 as ::core::ffi::c_int
        || gaps.left > 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn con_draw_decoration_into_frame(mut con: *mut Con) -> bool {
    return con_is_leaf(con) as ::core::ffi::c_int != 0
        && con_border_style(con) == BS_NORMAL as ::core::ffi::c_int
        && ((*con).parent.is_null()
            || (*(*con).parent).layout as ::core::ffi::c_uint
                != L_TABBED as ::core::ffi::c_int as ::core::ffi::c_uint
                && (*(*con).parent).layout as ::core::ffi::c_uint
                    != L_STACKED as ::core::ffi::c_int as ::core::ffi::c_uint);
}
unsafe extern "C" fn con_border_style_rect_without_title(mut con: *mut Con) -> Rect {
    if config.hide_edge_borders as ::core::ffi::c_uint
        == HEBM_SMART as ::core::ffi::c_int as ::core::ffi::c_uint
        && con_num_visible_children(con_get_workspace(con)) <= 1 as ::core::ffi::c_int
        || config.hide_edge_borders as ::core::ffi::c_uint
            == HEBM_SMART_NO_GAPS as ::core::ffi::c_int as ::core::ffi::c_uint
            && con_num_visible_children(con_get_workspace(con)) <= 1 as ::core::ffi::c_int
            && !has_outer_gaps(calculate_effective_gaps(con))
    {
        if !con_is_floating(con) {
            return Rect {
                x: 0 as uint32_t,
                y: 0 as uint32_t,
                width: 0 as uint32_t,
                height: 0 as uint32_t,
            };
        }
    }
    let mut borders_to_hide: adjacent_t = ADJ_NONE;
    let mut border_width: ::core::ffi::c_int = (*con).current_border_width;
    debuglog(
        b"%s:%s:%d - The border width for con is set to: %d\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"con_border_style_rect_without_title\0" as *const u8 as *const ::core::ffi::c_char,
        1831 as ::core::ffi::c_int,
        (*con).current_border_width,
    );
    let mut result: Rect = Rect {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    if (*con).current_border_width < 0 as ::core::ffi::c_int {
        if con_is_floating(con) {
            border_width = config.default_floating_border_width;
        } else {
            border_width = config.default_border_width;
        }
    }
    debuglog(
        b"%s:%s:%d - Effective border width is set to: %d\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"con_border_style_rect_without_title\0" as *const u8 as *const ::core::ffi::c_char,
        1840 as ::core::ffi::c_int,
        border_width,
    );
    let mut border_style: ::core::ffi::c_int = con_border_style(con);
    if border_style == BS_NONE as ::core::ffi::c_int {
        return Rect {
            x: 0 as uint32_t,
            y: 0 as uint32_t,
            width: 0 as uint32_t,
            height: 0 as uint32_t,
        };
    }
    if border_style == BS_NORMAL as ::core::ffi::c_int {
        result = Rect {
            x: border_width as uint32_t,
            y: 0 as uint32_t,
            width: -(2 as ::core::ffi::c_int * border_width) as uint32_t,
            height: -border_width as uint32_t,
        };
    } else {
        result = Rect {
            x: border_width as uint32_t,
            y: border_width as uint32_t,
            width: -(2 as ::core::ffi::c_int * border_width) as uint32_t,
            height: -(2 as ::core::ffi::c_int * border_width) as uint32_t,
        };
    }
    if config.hide_edge_borders as ::core::ffi::c_uint
        == HEBM_SMART_NO_GAPS as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        borders_to_hide = (con_adjacent_borders(con) as ::core::ffi::c_uint
            & HEBM_NONE as ::core::ffi::c_int as ::core::ffi::c_uint)
            as adjacent_t;
    } else {
        borders_to_hide = (con_adjacent_borders(con) as ::core::ffi::c_uint
            & config.hide_edge_borders as ::core::ffi::c_uint)
            as adjacent_t;
    }
    if borders_to_hide as ::core::ffi::c_uint
        & ADJ_LEFT_SCREEN_EDGE as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        result.x = result.x.wrapping_sub(border_width as uint32_t);
        result.width = result.width.wrapping_add(border_width as uint32_t);
    }
    if borders_to_hide as ::core::ffi::c_uint
        & ADJ_RIGHT_SCREEN_EDGE as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        result.width = result.width.wrapping_add(border_width as uint32_t);
    }
    if borders_to_hide as ::core::ffi::c_uint
        & ADJ_UPPER_SCREEN_EDGE as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
        && border_style != BS_NORMAL as ::core::ffi::c_int
    {
        result.y = result.y.wrapping_sub(border_width as uint32_t);
        result.height = result.height.wrapping_add(border_width as uint32_t);
    }
    if borders_to_hide as ::core::ffi::c_uint
        & ADJ_LOWER_SCREEN_EDGE as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        result.height = result.height.wrapping_add(border_width as uint32_t);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn con_border_style_rect(mut con: *mut Con) -> Rect {
    let mut result: Rect = con_border_style_rect_without_title(con);
    if con_border_style(con) == BS_NORMAL as ::core::ffi::c_int
        && con_draw_decoration_into_frame(con) as ::core::ffi::c_int != 0
    {
        let deco_height: ::core::ffi::c_int = render_deco_height() as ::core::ffi::c_int;
        result.y = result.y.wrapping_add(deco_height as uint32_t);
        result.height = result.height.wrapping_sub(deco_height as uint32_t);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn con_adjacent_borders(mut con: *mut Con) -> adjacent_t {
    let mut result: adjacent_t = ADJ_NONE;
    if con_is_floating(con) {
        return result;
    }
    let mut workspace: *mut Con = con_get_workspace(con);
    if (*con).rect.x == (*workspace).rect.x {
        result = ::core::mem::transmute::<::core::ffi::c_uint, adjacent_t>(
            result as ::core::ffi::c_uint
                | ADJ_LEFT_SCREEN_EDGE as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
    }
    if (*con).rect.x.wrapping_add((*con).rect.width)
        == (*workspace).rect.x.wrapping_add((*workspace).rect.width)
    {
        result = ::core::mem::transmute::<::core::ffi::c_uint, adjacent_t>(
            result as ::core::ffi::c_uint
                | ADJ_RIGHT_SCREEN_EDGE as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
    }
    if (*con).rect.y == (*workspace).rect.y {
        result = ::core::mem::transmute::<::core::ffi::c_uint, adjacent_t>(
            result as ::core::ffi::c_uint
                | ADJ_UPPER_SCREEN_EDGE as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
    }
    if (*con).rect.y.wrapping_add((*con).rect.height)
        == (*workspace).rect.y.wrapping_add((*workspace).rect.height)
    {
        result = ::core::mem::transmute::<::core::ffi::c_uint, adjacent_t>(
            result as ::core::ffi::c_uint
                | ADJ_LOWER_SCREEN_EDGE as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn con_border_style(mut con: *mut Con) -> ::core::ffi::c_int {
    if (*con).fullscreen_mode as ::core::ffi::c_uint
        == CF_OUTPUT as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*con).fullscreen_mode as ::core::ffi::c_uint
            == CF_GLOBAL as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        debuglog(
            b"%s:%s:%d - this one is fullscreen! overriding BS_NONE\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"con_border_style\0" as *const u8 as *const ::core::ffi::c_char,
            1933 as ::core::ffi::c_int,
        );
        return BS_NONE as ::core::ffi::c_int;
    }
    if !(*con).parent.is_null() {
        if (*(*con).parent).layout as ::core::ffi::c_uint
            == L_STACKED as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return (if con_num_children((*con).parent as *mut Con) == 1 as ::core::ffi::c_int {
                (*con).border_style as ::core::ffi::c_uint
            } else {
                BS_NORMAL as ::core::ffi::c_int as ::core::ffi::c_uint
            }) as ::core::ffi::c_int;
        }
        if (*(*con).parent).layout as ::core::ffi::c_uint
            == L_TABBED as ::core::ffi::c_int as ::core::ffi::c_uint
            && (*con).border_style as ::core::ffi::c_uint
                != BS_NORMAL as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return (if con_num_children((*con).parent as *mut Con) == 1 as ::core::ffi::c_int {
                (*con).border_style as ::core::ffi::c_uint
            } else {
                BS_NORMAL as ::core::ffi::c_int as ::core::ffi::c_uint
            }) as ::core::ffi::c_int;
        }
        if (*(*con).parent).type_0 as ::core::ffi::c_uint
            == CT_DOCKAREA as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return BS_NONE as ::core::ffi::c_int;
        }
    }
    return (*con).border_style as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn con_set_border_style(
    mut con: *mut Con,
    mut border_style: border_style_t,
    mut border_width: ::core::ffi::c_int,
) {
    if border_style as ::core::ffi::c_uint > (*con).max_user_border_style as ::core::ffi::c_uint {
        border_style = (*con).max_user_border_style;
    }
    if !con_is_floating(con) {
        (*con).border_style = border_style;
        (*con).current_border_width = border_width;
        return;
    }
    let mut parent: *mut Con = (*con).parent as *mut Con;
    let mut bsr: Rect = con_border_style_rect(con);
    (*con).rect = rect_add((*con).rect as Rect, bsr) as Rect;
    (*parent).rect = rect_add((*parent).rect as Rect, bsr) as Rect;
    (*con).border_style = border_style;
    (*con).current_border_width = border_width;
    bsr = con_border_style_rect(con);
    (*con).rect = rect_sub((*con).rect as Rect, bsr) as Rect;
    (*parent).rect = rect_sub((*parent).rect as Rect, bsr) as Rect;
}
#[no_mangle]
pub unsafe extern "C" fn con_set_layout(mut con: *mut Con, mut layout: layout_t) {
    debuglog(
        b"%s:%s:%d - con_set_layout(%p, %d), con->type = %d\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"con_set_layout\0" as *const u8 as *const ::core::ffi::c_char,
        1999 as ::core::ffi::c_int,
        con,
        layout as ::core::ffi::c_uint,
        (*con).type_0 as ::core::ffi::c_uint,
    );
    if (*con).type_0 as ::core::ffi::c_uint
        != CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        con = (*con).parent as *mut Con;
    }
    if (*con).layout as ::core::ffi::c_uint == L_SPLITH as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*con).layout as ::core::ffi::c_uint
            == L_SPLITV as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*con).last_split_layout = (*con).layout;
    }
    if (*con).type_0 as ::core::ffi::c_uint
        == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if con_num_children(con) == 0 as ::core::ffi::c_int {
            let mut ws_layout: layout_t = (if layout as ::core::ffi::c_uint
                == L_STACKED as ::core::ffi::c_int as ::core::ffi::c_uint
                || layout as ::core::ffi::c_uint
                    == L_TABBED as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                layout as ::core::ffi::c_uint
            } else {
                L_DEFAULT as ::core::ffi::c_int as ::core::ffi::c_uint
            }) as layout_t;
            debuglog(
                b"%s:%s:%d - Setting workspace_layout to %d\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"con_set_layout\0" as *const u8 as *const ::core::ffi::c_char,
                2022 as ::core::ffi::c_int,
                ws_layout as ::core::ffi::c_uint,
            );
            (*con).workspace_layout = ws_layout;
            debuglog(
                b"%s:%s:%d - Setting layout to %d\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"con_set_layout\0" as *const u8 as *const ::core::ffi::c_char,
                2024 as ::core::ffi::c_int,
                layout as ::core::ffi::c_uint,
            );
            (*con).layout = layout;
        } else if layout as ::core::ffi::c_uint
            == L_STACKED as ::core::ffi::c_int as ::core::ffi::c_uint
            || layout as ::core::ffi::c_uint
                == L_TABBED as ::core::ffi::c_int as ::core::ffi::c_uint
            || layout as ::core::ffi::c_uint
                == L_SPLITV as ::core::ffi::c_int as ::core::ffi::c_uint
            || layout as ::core::ffi::c_uint
                == L_SPLITH as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            debuglog(
                b"%s:%s:%d - Creating new split container\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"con_set_layout\0" as *const u8 as *const ::core::ffi::c_char,
                2027 as ::core::ffi::c_int,
            );
            let mut new: *mut Con = con_new(
                ::core::ptr::null_mut::<Con>(),
                ::core::ptr::null_mut::<i3Window>(),
            );
            (*new).parent = con as *mut Con;
            (*new).layout = layout;
            (*new).last_split_layout = (*con).last_split_layout;
            let mut focus_order: *mut *mut Con = get_focus_order(con);
            debuglog(
                b"%s:%s:%d - Moving cons\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"con_set_layout\0" as *const u8 as *const ::core::ffi::c_char,
                2040 as ::core::ffi::c_int,
            );
            while !(*con).nodes_head.tqh_first.is_null() {
                let mut child: *mut Con = (*con).nodes_head.tqh_first as *mut Con;
                con_detach(child);
                con_attach(child, new, true_0 != 0);
            }
            set_focus_order(new, focus_order);
            free(focus_order as *mut ::core::ffi::c_void);
            debuglog(
                b"%s:%s:%d - Attaching new split to ws\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"con_set_layout\0" as *const u8 as *const ::core::ffi::c_char,
                2051 as ::core::ffi::c_int,
            );
            con_attach(new, con, false_0 != 0);
            tree_flatten(croot);
            con_force_split_parents_redraw(con);
            return;
        }
    }
    if (*con).type_0 as ::core::ffi::c_uint
        != CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
        && (*(*con).parent).type_0 as ::core::ffi::c_uint
            != CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
        && con_num_children(con) == 1 as ::core::ffi::c_int
        && con_num_children((*con).parent as *mut Con) == 1 as ::core::ffi::c_int
    {
        let mut parent: *mut Con = (*con).parent as *mut Con;
        let mut child_0: *mut Con = (*con).nodes_head.tqh_first as *mut Con;
        con_detach(child_0);
        con_attach(child_0, parent, true_0 != 0);
        (*parent).last_split_layout = (*con).last_split_layout;
        tree_close_internal(con, DONT_KILL_WINDOW, true_0 != 0);
        con = parent;
    }
    if layout as ::core::ffi::c_uint == L_DEFAULT as ::core::ffi::c_int as ::core::ffi::c_uint {
        (*con).layout = (*con).last_split_layout;
        if (*con).layout as ::core::ffi::c_uint
            == L_DEFAULT as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            (*con).layout = L_SPLITH;
        }
    } else {
        (*con).layout = layout;
    }
    con_force_split_parents_redraw(con);
}
#[no_mangle]
pub unsafe extern "C" fn con_toggle_layout(
    mut con: *mut Con,
    mut toggle_mode: *const ::core::ffi::c_char,
) {
    let mut parent: *mut Con = con;
    if (*con).type_0 as ::core::ffi::c_uint
        != CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        parent = (*con).parent as *mut Con;
    }
    debuglog(
        b"%s:%s:%d - con_toggle_layout(%p, %s), parent = %p\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"con_toggle_layout\0" as *const u8 as *const ::core::ffi::c_char,
        2117 as ::core::ffi::c_int,
        con,
        toggle_mode,
        parent,
    );
    let delim: [::core::ffi::c_char; 2] =
        ::core::mem::transmute::<[u8; 2], [::core::ffi::c_char; 2]>(*b" \0");
    if strcasecmp(
        toggle_mode,
        b"split\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
        || !strstr(toggle_mode, &raw const delim as *const ::core::ffi::c_char).is_null()
    {
        let mut new_layout: layout_t = L_DEFAULT;
        let mut current_layout_found: bool = false_0 != 0;
        let mut tm_dup: *mut ::core::ffi::c_char = sstrdup(toggle_mode);
        let mut cur_tok: *mut ::core::ffi::c_char =
            strtok(tm_dup, &raw const delim as *const ::core::ffi::c_char);
        let mut current_block_20: u64;
        let mut layout: layout_t = L_DEFAULT;
        while !cur_tok.is_null() {
            if strcasecmp(
                cur_tok,
                b"split\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                if (*parent).layout as ::core::ffi::c_uint
                    != L_SPLITH as ::core::ffi::c_int as ::core::ffi::c_uint
                    && (*parent).layout as ::core::ffi::c_uint
                        != L_SPLITV as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    layout = (*parent).last_split_layout;
                    if layout as ::core::ffi::c_uint
                        == L_DEFAULT as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        layout = L_SPLITH;
                    }
                } else {
                    layout = (if (*parent).layout as ::core::ffi::c_uint
                        == L_SPLITH as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        L_SPLITV as ::core::ffi::c_int
                    } else {
                        L_SPLITH as ::core::ffi::c_int
                    }) as layout_t;
                }
                current_block_20 = 6057473163062296781;
            } else {
                let mut success: bool = layout_from_name(cur_tok, &raw mut layout);
                if !success
                    || layout as ::core::ffi::c_uint
                        == L_DEFAULT as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    errorlog(
                        b"ERROR: The token '%s' was not recognized and has been skipped.\n\0"
                            as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        cur_tok,
                    );
                    current_block_20 = 4906268039856690917;
                } else {
                    current_block_20 = 6057473163062296781;
                }
            }
            match current_block_20 {
                6057473163062296781 => {
                    if new_layout as ::core::ffi::c_uint
                        == L_DEFAULT as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        new_layout = layout;
                    }
                    if current_layout_found {
                        new_layout = layout;
                        break;
                    } else if (*parent).layout as ::core::ffi::c_uint
                        == layout as ::core::ffi::c_uint
                    {
                        current_layout_found = true_0 != 0;
                    }
                }
                _ => {}
            }
            cur_tok = strtok(
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
                &raw const delim as *const ::core::ffi::c_char,
            );
        }
        free(tm_dup as *mut ::core::ffi::c_void);
        if new_layout as ::core::ffi::c_uint
            != L_DEFAULT as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            con_set_layout(con, new_layout);
        }
    } else if strcasecmp(
        toggle_mode,
        b"all\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
        || strcasecmp(
            toggle_mode,
            b"default\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
    {
        if (*parent).layout as ::core::ffi::c_uint
            == L_STACKED as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            con_set_layout(con, L_TABBED);
        } else if (*parent).layout as ::core::ffi::c_uint
            == L_TABBED as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            if strcasecmp(
                toggle_mode,
                b"all\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                con_set_layout(con, L_SPLITH);
            } else {
                con_set_layout(con, (*parent).last_split_layout);
            }
        } else if (*parent).layout as ::core::ffi::c_uint
            == L_SPLITH as ::core::ffi::c_int as ::core::ffi::c_uint
            || (*parent).layout as ::core::ffi::c_uint
                == L_SPLITV as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            if strcasecmp(
                toggle_mode,
                b"all\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                if (*parent).layout as ::core::ffi::c_uint
                    == L_SPLITH as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    con_set_layout(con, L_SPLITV);
                } else {
                    con_set_layout(con, L_STACKED);
                }
            } else {
                con_set_layout(con, L_STACKED);
            }
        }
    }
}
unsafe extern "C" fn con_on_remove_child(mut con: *mut Con) {
    debuglog(
        b"%s:%s:%d - on_remove_child\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"con_on_remove_child\0" as *const u8 as *const ::core::ffi::c_char,
        2206 as ::core::ffi::c_int,
    );
    if (*con).type_0 as ::core::ffi::c_uint
        == CT_OUTPUT as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*con).type_0 as ::core::ffi::c_uint
            == CT_ROOT as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*con).type_0 as ::core::ffi::c_uint
            == CT_DOCKAREA as ::core::ffi::c_int as ::core::ffi::c_uint
        || !(*con).parent.is_null()
            && (*(*con).parent).type_0 as ::core::ffi::c_uint
                == CT_OUTPUT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        debuglog(
            b"%s:%s:%d - not handling, type = %d, name = %s\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"con_on_remove_child\0" as *const u8 as *const ::core::ffi::c_char,
            2214 as ::core::ffi::c_int,
            (*con).type_0 as ::core::ffi::c_uint,
            (*con).name,
        );
        return;
    }
    if (*con).type_0 as ::core::ffi::c_uint
        == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if (*con).focus_head.tqh_first.is_null() && !workspace_is_visible(con) {
            verboselog(
                b"Closing old workspace (%p / %s), it is empty\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                con,
                (*con).name,
            );
            let mut gen: yajl_gen = ipc_marshal_workspace_event(
                b"empty\0" as *const u8 as *const ::core::ffi::c_char,
                con,
                ::core::ptr::null_mut::<Con>(),
            );
            tree_close_internal(con, DONT_KILL_WINDOW, false_0 != 0);
            let mut payload: *const ::core::ffi::c_uchar =
                ::core::ptr::null::<::core::ffi::c_uchar>();
            let mut length: ylength = 0;
            yajl_gen_get_buf(gen, &raw mut payload, &raw mut length);
            ipc_send_event(
                b"workspace\0" as *const u8 as *const ::core::ffi::c_char,
                I3_IPC_EVENT_WORKSPACE as uint32_t,
                payload as *const ::core::ffi::c_char,
            );
            yajl_gen_free(gen);
        }
        return;
    }
    con_force_split_parents_redraw(con);
    (*con).urgent = con_has_urgent_child(con);
    con_update_parents_urgency(con);
    let mut children: ::core::ffi::c_int = con_num_children(con);
    if children == 0 as ::core::ffi::c_int {
        debuglog(
            b"%s:%s:%d - Container empty, closing\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"con_on_remove_child\0" as *const u8 as *const ::core::ffi::c_char,
            2243 as ::core::ffi::c_int,
        );
        tree_close_internal(con, DONT_KILL_WINDOW, false_0 != 0);
    }
}
#[no_mangle]
pub unsafe extern "C" fn con_minimum_size(mut con: *mut Con) -> Rect {
    debuglog(
        b"%s:%s:%d - Determining minimum size for con %p\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"con_minimum_size\0" as *const u8 as *const ::core::ffi::c_char,
        2254 as ::core::ffi::c_int,
        con,
    );
    if con_is_leaf(con) {
        debuglog(
            b"%s:%s:%d - leaf node, returning 75x50\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"con_minimum_size\0" as *const u8 as *const ::core::ffi::c_char,
            2257 as ::core::ffi::c_int,
        );
        return Rect {
            x: 0 as uint32_t,
            y: 0 as uint32_t,
            width: 75 as uint32_t,
            height: 50 as uint32_t,
        };
    }
    if (*con).type_0 as ::core::ffi::c_uint
        == CT_FLOATING_CON as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        debuglog(
            b"%s:%s:%d - floating con\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"con_minimum_size\0" as *const u8 as *const ::core::ffi::c_char,
            2262 as ::core::ffi::c_int,
        );
        let mut child: *mut Con = (*con).nodes_head.tqh_first as *mut Con;
        return con_minimum_size(child);
    }
    if (*con).layout as ::core::ffi::c_uint
        == L_STACKED as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*con).layout as ::core::ffi::c_uint
            == L_TABBED as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut max_width: uint32_t = 0 as uint32_t;
        let mut max_height: uint32_t = 0 as uint32_t;
        let mut deco_height: uint32_t = 0 as uint32_t;
        let mut child_0: *mut Con = ::core::ptr::null_mut::<Con>();
        child_0 = (*con).nodes_head.tqh_first as *mut Con;
        while !child_0.is_null() {
            let mut min: Rect = con_minimum_size(child_0);
            deco_height = deco_height.wrapping_add((*child_0).deco_rect.height);
            max_width = max(
                max_width as ::core::ffi::c_int,
                min.width as ::core::ffi::c_int,
            ) as uint32_t;
            max_height = max(
                max_height as ::core::ffi::c_int,
                min.height as ::core::ffi::c_int,
            ) as uint32_t;
            child_0 = (*child_0).nodes.tqe_next as *mut Con;
        }
        debuglog(
            b"%s:%s:%d - stacked/tabbed now, returning %d x %d + deco_rect = %d\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"con_minimum_size\0" as *const u8 as *const ::core::ffi::c_char,
            2277 as ::core::ffi::c_int,
            max_width,
            max_height,
            deco_height,
        );
        return Rect {
            x: 0 as uint32_t,
            y: 0 as uint32_t,
            width: max_width,
            height: max_height.wrapping_add(deco_height),
        };
    }
    if con_is_split(con) {
        let mut width: uint32_t = 0 as uint32_t;
        let mut height: uint32_t = 0 as uint32_t;
        let mut child_1: *mut Con = ::core::ptr::null_mut::<Con>();
        child_1 = (*con).nodes_head.tqh_first as *mut Con;
        while !child_1.is_null() {
            let mut min_0: Rect = con_minimum_size(child_1);
            if (*con).layout as ::core::ffi::c_uint
                == L_SPLITH as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                width = width.wrapping_add(min_0.width);
                height = max(
                    height as ::core::ffi::c_int,
                    min_0.height as ::core::ffi::c_int,
                ) as uint32_t;
            } else {
                height = height.wrapping_add(min_0.height);
                width = max(
                    width as ::core::ffi::c_int,
                    min_0.width as ::core::ffi::c_int,
                ) as uint32_t;
            }
            child_1 = (*child_1).nodes.tqe_next as *mut Con;
        }
        debuglog(
            b"%s:%s:%d - split container, returning width = %d x height = %d\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"con_minimum_size\0" as *const u8 as *const ::core::ffi::c_char,
            2297 as ::core::ffi::c_int,
            width,
            height,
        );
        return Rect {
            x: 0 as uint32_t,
            y: 0 as uint32_t,
            width: width,
            height: height,
        };
    }
    errorlog(
        b"ERROR: Unhandled case, type = %d, layout = %d, split = %d\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        (*con).type_0 as ::core::ffi::c_uint,
        (*con).layout as ::core::ffi::c_uint,
        con_is_split(con) as ::core::ffi::c_int,
    );
    if 0 as ::core::ffi::c_int != 0 {
    } else {
        __assert_fail(
            b"false\0" as *const u8 as *const ::core::ffi::c_char,
            b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
            2303 as ::core::ffi::c_uint,
            b"Rect con_minimum_size(Con *)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
    panic!("Reached end of non-void function without returning");
}
#[no_mangle]
pub unsafe extern "C" fn con_fullscreen_permits_focusing(mut con: *mut Con) -> bool {
    if focused.is_null() {
        return true_0 != 0;
    }
    let mut fs: *mut Con = focused;
    while !fs.is_null()
        && (*fs).fullscreen_mode as ::core::ffi::c_uint
            == CF_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        fs = (*fs).parent as *mut Con;
    }
    if fs != ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut Con {
    } else {
        __assert_fail(
            b"fs != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
            2341 as ::core::ffi::c_uint,
            b"_Bool con_fullscreen_permits_focusing(Con *)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    };
    if (*fs).fullscreen_mode as ::core::ffi::c_uint
        != CF_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
    } else {
        __assert_fail(
            b"fs->fullscreen_mode != CF_NONE\0" as *const u8 as *const ::core::ffi::c_char,
            b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
            2344 as ::core::ffi::c_uint,
            b"_Bool con_fullscreen_permits_focusing(Con *)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    };
    if (*fs).type_0 as ::core::ffi::c_uint
        == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return true_0 != 0;
    }
    if con == fs {
        return true_0 != 0;
    }
    if (*fs).fullscreen_mode as ::core::ffi::c_uint
        == CF_OUTPUT as ::core::ffi::c_int as ::core::ffi::c_uint
        && con_get_workspace(con) != con_get_workspace(fs)
    {
        return true_0 != 0;
    }
    return con_has_parent(con, fs);
}
#[no_mangle]
pub unsafe extern "C" fn con_has_urgent_child(mut con: *mut Con) -> bool {
    let mut child: *mut Con = ::core::ptr::null_mut::<Con>();
    if con_is_leaf(con) {
        return (*con).urgent;
    }
    child = (*con).nodes_head.tqh_first as *mut Con;
    while !child.is_null() {
        if con_has_urgent_child(child) {
            return true_0 != 0;
        }
        child = (*child).nodes.tqe_next as *mut Con;
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn con_update_parents_urgency(mut con: *mut Con) {
    let mut parent: *mut Con = (*con).parent as *mut Con;
    if (*con).type_0 as ::core::ffi::c_uint
        == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return;
    }
    let mut new_urgency_value: bool = (*con).urgent;
    while !parent.is_null()
        && (*parent).type_0 as ::core::ffi::c_uint
            != CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
        && (*parent).type_0 as ::core::ffi::c_uint
            != CT_DOCKAREA as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if new_urgency_value {
            (*parent).urgent = true_0 != 0;
        } else if !con_has_urgent_child(parent) {
            (*parent).urgent = false_0 != 0;
        }
        parent = (*parent).parent as *mut Con;
    }
}
#[no_mangle]
pub unsafe extern "C" fn con_set_urgency(mut con: *mut Con, mut urgent: bool) {
    if urgent as ::core::ffi::c_int != 0 && focused == con {
        debuglog(
            b"%s:%s:%d - Ignoring urgency flag for current client\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"con_set_urgency\0" as *const u8 as *const ::core::ffi::c_char,
            2426 as ::core::ffi::c_int,
        );
        return;
    }
    let old_urgent: bool = (*con).urgent;
    if (*con).urgency_timer.is_null() {
        (*con).urgent = urgent;
    } else {
        debuglog(
            b"%s:%s:%d - Discarding urgency WM_HINT because timer is running\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"con_set_urgency\0" as *const u8 as *const ::core::ffi::c_char,
            2435 as ::core::ffi::c_int,
        );
    }
    if !(*con).window.is_null() {
        if (*con).urgent {
            gettimeofday(&raw mut (*(*con).window).urgent, NULL);
        } else {
            (*(*con).window).urgent.tv_sec = 0 as __time_t;
            (*(*con).window).urgent.tv_usec = 0 as __suseconds_t;
        }
    }
    con_update_parents_urgency(con);
    let mut ws: *mut Con = ::core::ptr::null_mut::<Con>();
    ws = con_get_workspace(con);
    if !ws.is_null() {
        workspace_update_urgent_flag(ws);
    }
    if (*con).urgent as ::core::ffi::c_int != old_urgent as ::core::ffi::c_int {
        verboselog(
            b"Urgency flag changed to %d\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            (*con).urgent as ::core::ffi::c_int,
        );
        ipc_send_window_event(b"urgent\0" as *const u8 as *const ::core::ffi::c_char, con);
    }
}
#[no_mangle]
pub unsafe extern "C" fn con_get_tree_representation(
    mut con: *mut Con,
) -> *mut ::core::ffi::c_char {
    if con_is_leaf(con) {
        if (*con).window.is_null() {
            return sstrdup(b"nowin\0" as *const u8 as *const ::core::ffi::c_char);
        }
        if (*(*con).window).class_instance.is_null() {
            return sstrdup(b"noinstance\0" as *const u8 as *const ::core::ffi::c_char);
        }
        return sstrdup((*(*con).window).class_instance);
    }
    let mut buf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if (*con).layout as ::core::ffi::c_uint
        == L_DEFAULT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        buf = sstrdup(b"D[\0" as *const u8 as *const ::core::ffi::c_char);
    } else if (*con).layout as ::core::ffi::c_uint
        == L_SPLITV as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        buf = sstrdup(b"V[\0" as *const u8 as *const ::core::ffi::c_char);
    } else if (*con).layout as ::core::ffi::c_uint
        == L_SPLITH as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        buf = sstrdup(b"H[\0" as *const u8 as *const ::core::ffi::c_char);
    } else if (*con).layout as ::core::ffi::c_uint
        == L_TABBED as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        buf = sstrdup(b"T[\0" as *const u8 as *const ::core::ffi::c_char);
    } else if (*con).layout as ::core::ffi::c_uint
        == L_STACKED as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        buf = sstrdup(b"S[\0" as *const u8 as *const ::core::ffi::c_char);
    } else {
        errorlog(
            b"ERROR: BUG: Code not updated to account for new layout type\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        if 0 as ::core::ffi::c_int != 0 {
        } else {
            __assert_fail(
                b"false\0" as *const u8 as *const ::core::ffi::c_char,
                b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
                2503 as ::core::ffi::c_uint,
                b"char *con_get_tree_representation(Con *)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        };
    }
    let mut child: *mut Con = ::core::ptr::null_mut::<Con>();
    child = (*con).nodes_head.tqh_first as *mut Con;
    while !child.is_null() {
        let mut child_txt: *mut ::core::ffi::c_char = con_get_tree_representation(child);
        let mut tmp_buf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        sasprintf(
            &raw mut tmp_buf,
            b"%s%s%s\0" as *const u8 as *const ::core::ffi::c_char,
            buf,
            if (*con).nodes_head.tqh_first == child {
                b"\0" as *const u8 as *const ::core::ffi::c_char
            } else {
                b" \0" as *const u8 as *const ::core::ffi::c_char
            },
            child_txt,
        );
        free(buf as *mut ::core::ffi::c_void);
        buf = tmp_buf;
        free(child_txt as *mut ::core::ffi::c_void);
        child = (*child).nodes.tqe_next as *mut Con;
    }
    let mut complete_buf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    sasprintf(
        &raw mut complete_buf,
        b"%s]\0" as *const u8 as *const ::core::ffi::c_char,
        buf,
    );
    free(buf as *mut ::core::ffi::c_void);
    return complete_buf;
}
#[no_mangle]
pub unsafe extern "C" fn con_parse_title_format(mut con: *mut Con) -> *mut i3String {
    if (*con).title_format
        != ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut ::core::ffi::c_char
    {
    } else {
        __assert_fail(
            b"con->title_format != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
            2532 as ::core::ffi::c_uint,
            b"i3String *con_parse_title_format(Con *)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
    let mut win: *mut i3Window = (*con).window as *mut i3Window;
    let pango_markup: bool = font_is_pango() as bool;
    let mut title: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut class: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut instance: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut machine: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if win.is_null() {
        title = pango_escape_markup(con_get_tree_representation(con));
        class = sstrdup(b"i3-frame\0" as *const u8 as *const ::core::ffi::c_char);
        instance = sstrdup(b"i3-frame\0" as *const u8 as *const ::core::ffi::c_char);
        machine = sstrdup(b"\0" as *const u8 as *const ::core::ffi::c_char);
    } else {
        title = pango_escape_markup(sstrdup(if (*win).name.is_null() {
            b"\0" as *const u8 as *const ::core::ffi::c_char
        } else {
            i3string_as_utf8((*win).name)
        }));
        class = pango_escape_markup(sstrdup(if (*win).class_class.is_null() {
            b"\0" as *const u8 as *const ::core::ffi::c_char
        } else {
            (*win).class_class as *const ::core::ffi::c_char
        }));
        instance = pango_escape_markup(sstrdup(if (*win).class_instance.is_null() {
            b"\0" as *const u8 as *const ::core::ffi::c_char
        } else {
            (*win).class_instance as *const ::core::ffi::c_char
        }));
        machine = pango_escape_markup(sstrdup(if (*win).machine.is_null() {
            b"\0" as *const u8 as *const ::core::ffi::c_char
        } else {
            (*win).machine as *const ::core::ffi::c_char
        }));
    }
    let mut placeholders: [placeholder_t; 4] = [
        placeholder_t {
            name: b"%title\0" as *const u8 as *const ::core::ffi::c_char,
            value: title,
        },
        placeholder_t {
            name: b"%class\0" as *const u8 as *const ::core::ffi::c_char,
            value: class,
        },
        placeholder_t {
            name: b"%instance\0" as *const u8 as *const ::core::ffi::c_char,
            value: instance,
        },
        placeholder_t {
            name: b"%machine\0" as *const u8 as *const ::core::ffi::c_char,
            value: machine,
        },
    ];
    let num: size_t = (::core::mem::size_of::<[placeholder_t; 4]>() as size_t)
        .wrapping_div(::core::mem::size_of::<placeholder_t>() as size_t);
    let mut formatted_str: *mut ::core::ffi::c_char = format_placeholders(
        (*con).title_format,
        (&raw mut placeholders as *mut placeholder_t).offset(0 as ::core::ffi::c_int as isize)
            as *mut placeholder_t,
        num as ::core::ffi::c_int,
    );
    let mut formatted: *mut i3String = i3string_from_utf8(formatted_str);
    i3string_set_markup(formatted, pango_markup);
    free(formatted_str as *mut ::core::ffi::c_void);
    free(title as *mut ::core::ffi::c_void);
    free(class as *mut ::core::ffi::c_void);
    free(instance as *mut ::core::ffi::c_void);
    return formatted;
}
#[no_mangle]
pub unsafe extern "C" fn con_swap(mut first: *mut Con, mut second: *mut Con) -> bool {
    if first != ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut Con {
    } else {
        __assert_fail(
            b"first != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
            2581 as ::core::ffi::c_uint,
            b"_Bool con_swap(Con *, Con *)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
    if second != ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut Con {
    } else {
        __assert_fail(
            b"second != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
            2582 as ::core::ffi::c_uint,
            b"_Bool con_swap(Con *, Con *)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
    debuglog(
        b"%s:%s:%d - Swapping containers %p / %p\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"con_swap\0" as *const u8 as *const ::core::ffi::c_char,
        2583 as ::core::ffi::c_int,
        first,
        second,
    );
    if (*first).type_0 as ::core::ffi::c_uint != CT_CON as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        errorlog(
            b"ERROR: Only regular containers can be swapped, but found con = %p with type = %d.\n\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            first,
            (*first).type_0 as ::core::ffi::c_uint,
        );
        return false_0 != 0;
    }
    if (*second).type_0 as ::core::ffi::c_uint
        != CT_CON as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        errorlog(
            b"ERROR: Only regular containers can be swapped, but found con = %p with type = %d.\n\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            second,
            (*second).type_0 as ::core::ffi::c_uint,
        );
        return false_0 != 0;
    }
    if first == second {
        debuglog(
            b"%s:%s:%d - Swapping container %p with itself, nothing to do.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/con.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"con_swap\0" as *const u8 as *const ::core::ffi::c_char,
            2596 as ::core::ffi::c_int,
            first,
        );
        return false_0 != 0;
    }
    if con_has_parent(first, second) as ::core::ffi::c_int != 0
        || con_has_parent(second, first) as ::core::ffi::c_int != 0
    {
        errorlog(
            b"ERROR: Cannot swap containers %p and %p because they are in a parent-child relationship.\n\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            first,
            second,
        );
        return false_0 != 0;
    }
    let mut ws1: *mut Con = con_get_workspace(first);
    let mut ws2: *mut Con = con_get_workspace(second);
    let mut restore_focus: *mut Con = ::core::ptr::null_mut::<Con>();
    if ws1 == ws2 && ws1 == con_get_workspace(focused) {
        restore_focus = focused;
    } else if first == focused || con_has_parent(focused, first) as ::core::ffi::c_int != 0 {
        restore_focus = second;
    } else if second == focused || con_has_parent(focused, second) as ::core::ffi::c_int != 0 {
        restore_focus = first;
    }
    let mut head1: *mut nodes_head = &raw mut (*(*first).parent).nodes_head;
    let mut head2: *mut nodes_head = &raw mut (*(*second).parent).nodes_head;
    let mut first_prev: *mut Con = *(*((*first).nodes.tqe_prev as *mut nodes_head)).tqh_last;
    let mut second_prev: *mut Con = *(*((*second).nodes.tqe_prev as *mut nodes_head)).tqh_last;
    if second_prev == first {
        *(*first).nodes.tqe_prev = second as *mut Con;
        (*second).nodes.tqe_prev = (*first).nodes.tqe_prev;
        (*first).nodes.tqe_prev = &raw mut (*second).nodes.tqe_next;
        (*first).nodes.tqe_next = (*second).nodes.tqe_next;
        if !(*second).nodes.tqe_next.is_null() {
            (*(*second).nodes.tqe_next).nodes.tqe_prev = &raw mut (*first).nodes.tqe_next;
        }
        (*second).nodes.tqe_next = first as *mut Con;
        if (*head1).tqh_last == &raw mut (*second).nodes.tqe_next {
            (*head1).tqh_last = &raw mut (*first).nodes.tqe_next;
        }
    } else if first_prev == second {
        *(*second).nodes.tqe_prev = first as *mut Con;
        (*first).nodes.tqe_prev = (*second).nodes.tqe_prev;
        (*second).nodes.tqe_prev = &raw mut (*first).nodes.tqe_next;
        (*second).nodes.tqe_next = (*first).nodes.tqe_next;
        if !(*first).nodes.tqe_next.is_null() {
            (*(*first).nodes.tqe_next).nodes.tqe_prev = &raw mut (*second).nodes.tqe_next;
        }
        (*first).nodes.tqe_next = second as *mut Con;
        if (*head1).tqh_last == &raw mut (*first).nodes.tqe_next {
            (*head1).tqh_last = &raw mut (*second).nodes.tqe_next;
        }
    } else {
        if !(*first).nodes.tqe_next.is_null() {
            (*(*first).nodes.tqe_next).nodes.tqe_prev = (*first).nodes.tqe_prev;
        } else {
            (*head1).tqh_last = (*first).nodes.tqe_prev;
        }
        *(*first).nodes.tqe_prev = (*first).nodes.tqe_next;
        if !(*second).nodes.tqe_next.is_null() {
            (*(*second).nodes.tqe_next).nodes.tqe_prev = (*second).nodes.tqe_prev;
        } else {
            (*head2).tqh_last = (*second).nodes.tqe_prev;
        }
        *(*second).nodes.tqe_prev = (*second).nodes.tqe_next;
        if second_prev.is_null() {
            (*first).nodes.tqe_next = (*head2).tqh_first;
            if !(*first).nodes.tqe_next.is_null() {
                (*(*head2).tqh_first).nodes.tqe_prev = &raw mut (*first).nodes.tqe_next;
            } else {
                (*head2).tqh_last = &raw mut (*first).nodes.tqe_next;
            }
            (*head2).tqh_first = first as *mut Con;
            (*first).nodes.tqe_prev = &raw mut (*head2).tqh_first;
        } else {
            (*first).nodes.tqe_next = (*second_prev).nodes.tqe_next;
            if !(*first).nodes.tqe_next.is_null() {
                (*(*first).nodes.tqe_next).nodes.tqe_prev = &raw mut (*first).nodes.tqe_next;
            } else {
                (*head2).tqh_last = &raw mut (*first).nodes.tqe_next;
            }
            (*second_prev).nodes.tqe_next = first as *mut Con;
            (*first).nodes.tqe_prev = &raw mut (*second_prev).nodes.tqe_next;
        }
        if first_prev.is_null() {
            (*second).nodes.tqe_next = (*head1).tqh_first;
            if !(*second).nodes.tqe_next.is_null() {
                (*(*head1).tqh_first).nodes.tqe_prev = &raw mut (*second).nodes.tqe_next;
            } else {
                (*head1).tqh_last = &raw mut (*second).nodes.tqe_next;
            }
            (*head1).tqh_first = second as *mut Con;
            (*second).nodes.tqe_prev = &raw mut (*head1).tqh_first;
        } else {
            (*second).nodes.tqe_next = (*first_prev).nodes.tqe_next;
            if !(*second).nodes.tqe_next.is_null() {
                (*(*second).nodes.tqe_next).nodes.tqe_prev = &raw mut (*second).nodes.tqe_next;
            } else {
                (*head1).tqh_last = &raw mut (*second).nodes.tqe_next;
            }
            (*first_prev).nodes.tqe_next = second as *mut Con;
            (*second).nodes.tqe_prev = &raw mut (*first_prev).nodes.tqe_next;
        }
    }
    let mut head1_0: *mut focus_head = &raw mut (*(*first).parent).focus_head;
    let mut head2_0: *mut focus_head = &raw mut (*(*second).parent).focus_head;
    let mut first_prev_0: *mut Con = *(*((*first).focused.tqe_prev as *mut focus_head)).tqh_last;
    let mut second_prev_0: *mut Con = *(*((*second).focused.tqe_prev as *mut focus_head)).tqh_last;
    if second_prev_0 == first {
        *(*first).focused.tqe_prev = second as *mut Con;
        (*second).focused.tqe_prev = (*first).focused.tqe_prev;
        (*first).focused.tqe_prev = &raw mut (*second).focused.tqe_next;
        (*first).focused.tqe_next = (*second).focused.tqe_next;
        if !(*second).focused.tqe_next.is_null() {
            (*(*second).focused.tqe_next).focused.tqe_prev = &raw mut (*first).focused.tqe_next;
        }
        (*second).focused.tqe_next = first as *mut Con;
        if (*head1_0).tqh_last == &raw mut (*second).focused.tqe_next {
            (*head1_0).tqh_last = &raw mut (*first).focused.tqe_next;
        }
    } else if first_prev_0 == second {
        *(*second).focused.tqe_prev = first as *mut Con;
        (*first).focused.tqe_prev = (*second).focused.tqe_prev;
        (*second).focused.tqe_prev = &raw mut (*first).focused.tqe_next;
        (*second).focused.tqe_next = (*first).focused.tqe_next;
        if !(*first).focused.tqe_next.is_null() {
            (*(*first).focused.tqe_next).focused.tqe_prev = &raw mut (*second).focused.tqe_next;
        }
        (*first).focused.tqe_next = second as *mut Con;
        if (*head1_0).tqh_last == &raw mut (*first).focused.tqe_next {
            (*head1_0).tqh_last = &raw mut (*second).focused.tqe_next;
        }
    } else {
        if !(*first).focused.tqe_next.is_null() {
            (*(*first).focused.tqe_next).focused.tqe_prev = (*first).focused.tqe_prev;
        } else {
            (*head1_0).tqh_last = (*first).focused.tqe_prev;
        }
        *(*first).focused.tqe_prev = (*first).focused.tqe_next;
        if !(*second).focused.tqe_next.is_null() {
            (*(*second).focused.tqe_next).focused.tqe_prev = (*second).focused.tqe_prev;
        } else {
            (*head2_0).tqh_last = (*second).focused.tqe_prev;
        }
        *(*second).focused.tqe_prev = (*second).focused.tqe_next;
        if second_prev_0.is_null() {
            (*first).focused.tqe_next = (*head2_0).tqh_first;
            if !(*first).focused.tqe_next.is_null() {
                (*(*head2_0).tqh_first).focused.tqe_prev = &raw mut (*first).focused.tqe_next;
            } else {
                (*head2_0).tqh_last = &raw mut (*first).focused.tqe_next;
            }
            (*head2_0).tqh_first = first as *mut Con;
            (*first).focused.tqe_prev = &raw mut (*head2_0).tqh_first;
        } else {
            (*first).focused.tqe_next = (*second_prev_0).focused.tqe_next;
            if !(*first).focused.tqe_next.is_null() {
                (*(*first).focused.tqe_next).focused.tqe_prev = &raw mut (*first).focused.tqe_next;
            } else {
                (*head2_0).tqh_last = &raw mut (*first).focused.tqe_next;
            }
            (*second_prev_0).focused.tqe_next = first as *mut Con;
            (*first).focused.tqe_prev = &raw mut (*second_prev_0).focused.tqe_next;
        }
        if first_prev_0.is_null() {
            (*second).focused.tqe_next = (*head1_0).tqh_first;
            if !(*second).focused.tqe_next.is_null() {
                (*(*head1_0).tqh_first).focused.tqe_prev = &raw mut (*second).focused.tqe_next;
            } else {
                (*head1_0).tqh_last = &raw mut (*second).focused.tqe_next;
            }
            (*head1_0).tqh_first = second as *mut Con;
            (*second).focused.tqe_prev = &raw mut (*head1_0).tqh_first;
        } else {
            (*second).focused.tqe_next = (*first_prev_0).focused.tqe_next;
            if !(*second).focused.tqe_next.is_null() {
                (*(*second).focused.tqe_next).focused.tqe_prev =
                    &raw mut (*second).focused.tqe_next;
            } else {
                (*head1_0).tqh_last = &raw mut (*second).focused.tqe_next;
            }
            (*first_prev_0).focused.tqe_next = second as *mut Con;
            (*second).focused.tqe_prev = &raw mut (*first_prev_0).focused.tqe_next;
        }
    }
    let mut tmp_SWAP: *mut Con = (*first).parent as *mut Con;
    (*first).parent = (*second).parent;
    (*second).parent = tmp_SWAP as *mut Con;
    let mut tmp_SWAP_0: ::core::ffi::c_int = (*first).floating as ::core::ffi::c_int;
    (*first).floating = (*second).floating;
    (*second).floating = tmp_SWAP_0 as C2RustUnnamed_8;
    let mut tmp_SWAP_1: Rect = (*first).rect as Rect;
    (*first).rect = (*second).rect;
    (*second).rect = tmp_SWAP_1 as Rect;
    let mut tmp_SWAP_2: Rect = (*first).window_rect as Rect;
    (*first).window_rect = (*second).window_rect;
    (*second).window_rect = tmp_SWAP_2 as Rect;
    let mut tmp_SWAP_3: ::core::ffi::c_double = (*first).percent;
    (*first).percent = (*second).percent;
    (*second).percent = tmp_SWAP_3;
    if !restore_focus.is_null() {
        con_focus(restore_focus);
    }
    con_set_urgency(first, (*first).urgent);
    con_set_urgency(second, (*second).urgent);
    let mut second_fullscreen_mode: fullscreen_mode_t = (*second).fullscreen_mode;
    if (*first).fullscreen_mode as ::core::ffi::c_uint
        == CF_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        con_disable_fullscreen(second);
    } else {
        con_enable_fullscreen(second, (*first).fullscreen_mode);
    }
    if second_fullscreen_mode as ::core::ffi::c_uint
        == CF_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        con_disable_fullscreen(first);
    } else {
        con_enable_fullscreen(first, second_fullscreen_mode);
    }
    con_fix_percent((*first).parent as *mut Con);
    con_fix_percent((*second).parent as *mut Con);
    free((*first).deco_render_params as *mut ::core::ffi::c_void);
    (*first).deco_render_params = ::core::ptr::null_mut::<deco_render_params>();
    free((*second).deco_render_params as *mut ::core::ffi::c_void);
    (*second).deco_render_params = ::core::ptr::null_mut::<deco_render_params>();
    con_force_split_parents_redraw(first);
    con_force_split_parents_redraw(second);
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn con_rect_size_in_orientation(mut con: *mut Con) -> uint32_t {
    return if con_orientation(con) as ::core::ffi::c_uint
        == HORIZ as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*con).rect.width
    } else {
        (*con).rect.height
    };
}
#[no_mangle]
pub unsafe extern "C" fn con_merge_into(mut old: *mut Con, mut new: *mut Con) {
    (*new).window = (*old).window;
    (*old).window = ::core::ptr::null_mut::<Window>();
    if !(*old).title_format.is_null() {
        free((*new).title_format as *mut ::core::ffi::c_void);
        (*new).title_format = ::core::ptr::null_mut::<::core::ffi::c_char>();
        (*new).title_format = (*old).title_format;
        (*old).title_format = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if !(*old).sticky_group.is_null() {
        free((*new).sticky_group as *mut ::core::ffi::c_void);
        (*new).sticky_group = ::core::ptr::null_mut::<::core::ffi::c_char>();
        (*new).sticky_group = (*old).sticky_group;
        (*old).sticky_group = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    (*new).sticky = (*old).sticky;
    con_set_urgency(new, (*old).urgent);
    let mut mark: *mut mark_t = ::core::ptr::null_mut::<mark_t>();
    mark = (*old).marks_head.tqh_first as *mut mark_t;
    while !mark.is_null() {
        (*mark).marks.tqe_next = ::core::ptr::null_mut::<mark_t>();
        (*mark).marks.tqe_prev = (*new).marks_head.tqh_last;
        *(*new).marks_head.tqh_last = mark as *mut mark_t;
        (*new).marks_head.tqh_last = &raw mut (*mark).marks.tqe_next;
        ipc_send_window_event(b"mark\0" as *const u8 as *const ::core::ffi::c_char, new);
        mark = (*mark).marks.tqe_next as *mut mark_t;
    }
    (*new).mark_changed = !(*old).marks_head.tqh_first.is_null();
    (*old).marks_head.tqh_first = ::core::ptr::null_mut::<mark_t>();
    (*old).marks_head.tqh_last = &raw mut (*old).marks_head.tqh_first;
    tree_close_internal(old, DONT_KILL_WINDOW, false_0 != 0);
}
#[no_mangle]
pub unsafe extern "C" fn con_inside_stacked_or_tabbed(mut con: *mut Con) -> bool {
    if (*con).parent.is_null() {
        return false_0 != 0;
    }
    if (*(*con).parent).layout as ::core::ffi::c_uint
        == L_STACKED as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*(*con).parent).layout as ::core::ffi::c_uint
            == L_TABBED as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return true_0 != 0;
    }
    return con_inside_stacked_or_tabbed((*con).parent as *mut Con);
}
