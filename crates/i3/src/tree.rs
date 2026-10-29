/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * tree.c: Everything that primarily modifies the layout tree data structure.
 *
 */
// Mechanical file-by-file Rust translation of src/tree.c.
// Generated with C2Rust 0.22.1; C ABI and original ownership retained.
extern "C" {
    pub type xcb_connection_t;
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
    fn xcb_change_window_attributes(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        value_mask: uint32_t,
        value_list: *const ::core::ffi::c_void,
    ) -> xcb_void_cookie_t;
    fn xcb_change_save_set(
        c: *mut xcb_connection_t,
        mode: uint8_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t;
    fn xcb_reparent_window(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        parent: xcb_window_t,
        x: int16_t,
        y: int16_t,
    ) -> xcb_void_cookie_t;
    fn xcb_unmap_window(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t;
    fn xcb_change_property(
        c: *mut xcb_connection_t,
        mode: uint8_t,
        window: xcb_window_t,
        property: xcb_atom_t,
        type_0: xcb_atom_t,
        format: uint8_t,
        data_len: uint32_t,
        data: *const ::core::ffi::c_void,
    ) -> xcb_void_cookie_t;
    static mut conn: *mut xcb_connection_t;
    fn verboselog(fmt: *mut ::core::ffi::c_char, ...);
    fn errorlog(fmt: *mut ::core::ffi::c_char, ...);
    fn debuglog(fmt: *mut ::core::ffi::c_char, ...);
    fn sstrdup(str: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn resolve_tilde(path: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn path_exists(path: *const ::core::ffi::c_char) -> bool;
    fn slurp(path: *const ::core::ffi::c_char, buf: *mut *mut ::core::ffi::c_char) -> ssize_t;
    fn orientation_from_direction(direction: direction_t) -> orientation_t;
    fn position_from_direction(direction: direction_t) -> position_t;
    fn ev_timer_stop(loop_0: *mut ev_loop, w: *mut ev_timer);
    fn xcb_shape_select_input(
        c: *mut xcb_connection_t,
        destination_window: xcb_window_t,
        enable: uint8_t,
    ) -> xcb_void_cookie_t;
    static mut shape_supported: bool;
    static mut root: xcb_window_t;
    static mut main_loop: *mut ev_loop;
    static mut config: Config;
    fn ipc_send_window_event(property: *const ::core::ffi::c_char, con: *mut Con);
    static mut A_WM_STATE: xcb_atom_t;
    fn get_output_containing(x: ::core::ffi::c_uint, y: ::core::ffi::c_uint) -> *mut Output;
    fn get_output_next(
        direction: direction_t,
        current: *mut Output,
        close_far: output_close_far_t,
    ) -> *mut Output;
    fn workspace_is_visible(ws: *mut Con) -> bool;
    fn workspace_show(ws: *mut Con);
    fn workspace_update_urgent_flag(ws: *mut Con);
    fn workspace_encapsulate(ws: *mut Con) -> *mut Con;
    fn x_con_kill(con: *mut Con);
    fn x_window_kill(window: xcb_window_t, kill_window: kill_window_t);
    fn x_push_changes(con: *mut Con);
    fn x_set_name(con: *mut Con, name: *const ::core::ffi::c_char);
    fn x_set_warp_to(rect: *mut Rect);
    fn add_ignore_event(sequence: ignore_event_sequence_t, response_type: ::core::ffi::c_int);
    fn con_new(parent: *mut Con, window: *mut i3Window) -> *mut Con;
    fn con_free(con: *mut Con);
    fn con_activate(con: *mut Con);
    fn con_is_split(con: *mut Con) -> bool;
    fn con_get_workspace(con: *mut Con) -> *mut Con;
    fn con_get_fullscreen_con(con: *mut Con, fullscreen_mode: fullscreen_mode_t) -> *mut Con;
    fn con_is_floating(con: *mut Con) -> bool;
    fn con_num_children(con: *mut Con) -> ::core::ffi::c_int;
    fn con_attach(con: *mut Con, parent: *mut Con, ignore_focus: bool);
    fn con_detach(con: *mut Con);
    fn con_fix_percent(con: *mut Con);
    fn con_orientation(con: *mut Con) -> orientation_t;
    fn con_next_focused(con: *mut Con) -> *mut Con;
    fn con_descend_focused(con: *mut Con) -> *mut Con;
    fn con_descend_tiling_focused(con: *mut Con) -> *mut Con;
    fn con_fullscreen_permits_focusing(con: *mut Con) -> bool;
    fn con_update_parents_urgency(con: *mut Con);
    fn con_set_urgency(con: *mut Con, urgent: bool);
    fn con_force_split_parents_redraw(con: *mut Con);
    fn tree_append_json(
        con: *mut Con,
        buf: *const ::core::ffi::c_char,
        len: size_t,
        errormsg: *mut *mut ::core::ffi::c_char,
    );
    fn render_con(con: *mut Con);
    fn window_free(win: *mut i3Window);
    fn output_get_content(output: *mut Con) -> *mut Con;
    fn output_primary_name(output: *mut Output) -> *mut ::core::ffi::c_char;
    fn ewmh_update_desktop_properties();
    fn restore_open_placeholder_windows(con: *mut Con);
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __time_t = ::core::ffi::c_long;
pub type __suseconds_t = ::core::ffi::c_long;
pub type ssize_t = isize;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_void_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
pub type xcb_window_t = uint32_t;
pub type xcb_font_t = uint32_t;
pub type xcb_gcontext_t = uint32_t;
pub type xcb_colormap_t = uint32_t;
pub type xcb_atom_t = uint32_t;
pub type xcb_drawable_t = uint32_t;
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
pub type xcb_set_mode_t = ::core::ffi::c_uint;
pub const XCB_SET_MODE_DELETE: xcb_set_mode_t = 1;
pub const XCB_SET_MODE_INSERT: xcb_set_mode_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_get_geometry_reply_t {
    pub response_type: uint8_t,
    pub depth: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub root: xcb_window_t,
    pub x: int16_t,
    pub y: int16_t,
    pub width: uint16_t,
    pub height: uint16_t,
    pub border_width: uint16_t,
    pub pad0: [uint8_t; 2],
}
pub type xcb_prop_mode_t = ::core::ffi::c_uint;
pub const XCB_PROP_MODE_APPEND: xcb_prop_mode_t = 2;
pub const XCB_PROP_MODE_PREPEND: xcb_prop_mode_t = 1;
pub const XCB_PROP_MODE_REPLACE: xcb_prop_mode_t = 0;
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
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const XCB_ICCCM_WM_STATE_ICONIC: C2RustUnnamed = 3;
pub const XCB_ICCCM_WM_STATE_NORMAL: C2RustUnnamed = 1;
pub const XCB_ICCCM_WM_STATE_WITHDRAWN: C2RustUnnamed = 0;
pub type PangoFontDescription = _PangoFontDescription;
pub type cairo_t = _cairo;
pub type cairo_surface_t = _cairo_surface;
pub type i3String = _i3String;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Font {
    pub type_0: C2RustUnnamed_2,
    pub height: ::core::ffi::c_int,
    pub pattern: *mut ::core::ffi::c_char,
    pub specific: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub xcb: C2RustUnnamed_1,
    pub pango_desc: *mut PangoFontDescription,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub id: xcb_font_t,
    pub info: *mut xcb_query_font_reply_t,
    pub table: *mut xcb_charinfo_t,
}
pub type C2RustUnnamed_2 = ::core::ffi::c_uint;
pub const FONT_TYPE_PANGO: C2RustUnnamed_2 = 2;
pub const FONT_TYPE_XCB: C2RustUnnamed_2 = 1;
pub const FONT_TYPE_NONE: C2RustUnnamed_2 = 0;
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
    pub outputs: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
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
    pub type_0: C2RustUnnamed_20,
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
    pub floating: C2RustUnnamed_9,
    pub nodes: C2RustUnnamed_8,
    pub focused: C2RustUnnamed_7,
    pub all_cons: C2RustUnnamed_6,
    pub floating_windows: C2RustUnnamed_5,
    pub on_remove_child: Option<unsafe extern "C" fn(*mut Con) -> ()>,
    pub scratchpad_state: C2RustUnnamed_4,
    pub old_id: ::core::ffi::c_int,
    pub depth: uint16_t,
    pub colormap: xcb_colormap_t,
}
pub type C2RustUnnamed_4 = ::core::ffi::c_uint;
pub const SCRATCHPAD_CHANGED: C2RustUnnamed_4 = 2;
pub const SCRATCHPAD_FRESH: C2RustUnnamed_4 = 1;
pub const SCRATCHPAD_NONE: C2RustUnnamed_4 = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub tqe_next: *mut Con,
    pub tqe_prev: *mut *mut Con,
}
pub type C2RustUnnamed_9 = ::core::ffi::c_uint;
pub const FLOATING_USER_ON: C2RustUnnamed_9 = 3;
pub const FLOATING_AUTO_ON: C2RustUnnamed_9 = 2;
pub const FLOATING_USER_OFF: C2RustUnnamed_9 = 1;
pub const FLOATING_AUTO_OFF: C2RustUnnamed_9 = 0;
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
    pub urgent: C2RustUnnamed_14,
    pub dock: C2RustUnnamed_13,
    pub id: xcb_window_t,
    pub window_mode: C2RustUnnamed_12,
    pub con_id: *mut Con,
    pub match_all_windows: bool,
    pub insert_where: C2RustUnnamed_11,
    pub matches: C2RustUnnamed_10,
    pub restart_mode: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub tqe_next: *mut Match,
    pub tqe_prev: *mut *mut Match,
}
pub type C2RustUnnamed_11 = ::core::ffi::c_uint;
pub const M_BELOW: C2RustUnnamed_11 = 2;
pub const M_ASSIGN_WS: C2RustUnnamed_11 = 1;
pub const M_HERE: C2RustUnnamed_11 = 0;
pub type C2RustUnnamed_12 = ::core::ffi::c_uint;
pub const WM_FLOATING: C2RustUnnamed_12 = 6;
pub const WM_FLOATING_USER: C2RustUnnamed_12 = 5;
pub const WM_FLOATING_AUTO: C2RustUnnamed_12 = 4;
pub const WM_TILING: C2RustUnnamed_12 = 3;
pub const WM_TILING_USER: C2RustUnnamed_12 = 2;
pub const WM_TILING_AUTO: C2RustUnnamed_12 = 1;
pub const WM_ANY: C2RustUnnamed_12 = 0;
pub type C2RustUnnamed_13 = ::core::ffi::c_int;
pub const M_DOCK_BOTTOM: C2RustUnnamed_13 = 3;
pub const M_DOCK_TOP: C2RustUnnamed_13 = 2;
pub const M_DOCK_ANY: C2RustUnnamed_13 = 1;
pub const M_NODOCK: C2RustUnnamed_13 = 0;
pub const M_DONTCHECK: C2RustUnnamed_13 = -1;
pub type C2RustUnnamed_14 = ::core::ffi::c_int;
pub const U_OLDEST: C2RustUnnamed_14 = 1;
pub const U_LATEST: C2RustUnnamed_14 = 0;
pub const U_DONTCHECK: C2RustUnnamed_14 = -1;
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
    pub dock: C2RustUnnamed_15,
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
pub type C2RustUnnamed_15 = ::core::ffi::c_uint;
pub const W_DOCK_BOTTOM: C2RustUnnamed_15 = 2;
pub const W_DOCK_TOP: C2RustUnnamed_15 = 1;
pub const W_NODOCK: C2RustUnnamed_15 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Assignment {
    pub type_0: C2RustUnnamed_18,
    pub match_0: Match,
    pub dest: C2RustUnnamed_17,
    pub assignments: C2RustUnnamed_16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_16 {
    pub tqe_next: *mut Assignment,
    pub tqe_prev: *mut *mut Assignment,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_17 {
    pub command: *mut ::core::ffi::c_char,
    pub workspace: *mut ::core::ffi::c_char,
    pub output: *mut ::core::ffi::c_char,
}
pub type C2RustUnnamed_18 = ::core::ffi::c_uint;
pub const A_TO_OUTPUT: C2RustUnnamed_18 = 16;
pub const A_TO_WORKSPACE_NUMBER: C2RustUnnamed_18 = 8;
pub const A_NO_FOCUS: C2RustUnnamed_18 = 4;
pub const A_TO_WORKSPACE: C2RustUnnamed_18 = 2;
pub const A_COMMAND: C2RustUnnamed_18 = 1;
pub const A_ANY: C2RustUnnamed_18 = 0;
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
    pub marks: C2RustUnnamed_19,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_19 {
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
pub type C2RustUnnamed_20 = ::core::ffi::c_uint;
pub const CT_DOCKAREA: C2RustUnnamed_20 = 5;
pub const CT_WORKSPACE: C2RustUnnamed_20 = 4;
pub const CT_FLOATING_CON: C2RustUnnamed_20 = 3;
pub const CT_CON: C2RustUnnamed_20 = 2;
pub const CT_OUTPUT: C2RustUnnamed_20 = 1;
pub const CT_ROOT: C2RustUnnamed_20 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct names_head {
    pub slh_first: *mut output_name,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct output_name {
    pub name: *mut ::core::ffi::c_char,
    pub names: C2RustUnnamed_21,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_21 {
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
pub type position_t = ::core::ffi::c_uint;
pub const AFTER: position_t = 1;
pub const BEFORE: position_t = 0;
pub type kill_window_t = ::core::ffi::c_uint;
pub const KILL_CLIENT: kill_window_t = 2;
pub const KILL_WINDOW: kill_window_t = 1;
pub const DONT_KILL_WINDOW: kill_window_t = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ignore_event_sequence_t {
    pub value: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct all_cons_head {
    pub tqh_first: *mut Con,
    pub tqh_last: *mut *mut Con,
}
pub type output_close_far_t = ::core::ffi::c_uint;
pub const FARTHEST_OUTPUT: output_close_far_t = 1;
pub const CLOSEST_OUTPUT: output_close_far_t = 0;
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
    pub focus_on_window_activation: C2RustUnnamed_24,
    pub show_marks: bool,
    pub title_align: C2RustUnnamed_23,
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
    pub popup_during_fullscreen: C2RustUnnamed_22,
    pub number_barconfigs: ::core::ffi::c_int,
    pub tiling_drag: tiling_drag_t,
    pub gaps: gaps_t,
    pub smart_gaps: smart_gaps_t,
}
pub type tiling_drag_t = ::core::ffi::c_uint;
pub const TILING_DRAG_MODIFIER_OR_TITLEBAR: tiling_drag_t = 3;
pub const TILING_DRAG_TITLEBAR: tiling_drag_t = 2;
pub const TILING_DRAG_MODIFIER: tiling_drag_t = 1;
pub const TILING_DRAG_OFF: tiling_drag_t = 0;
pub type C2RustUnnamed_22 = ::core::ffi::c_uint;
pub const PDF_ALL: C2RustUnnamed_22 = 3;
pub const PDF_IGNORE: C2RustUnnamed_22 = 2;
pub const PDF_LEAVE_FULLSCREEN: C2RustUnnamed_22 = 1;
pub const PDF_SMART: C2RustUnnamed_22 = 0;
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
pub type C2RustUnnamed_23 = ::core::ffi::c_uint;
pub const ALIGN_RIGHT: C2RustUnnamed_23 = 2;
pub const ALIGN_CENTER: C2RustUnnamed_23 = 1;
pub const ALIGN_LEFT: C2RustUnnamed_23 = 0;
pub type C2RustUnnamed_24 = ::core::ffi::c_uint;
pub const FOWA_NONE: C2RustUnnamed_24 = 3;
pub const FOWA_FOCUS: C2RustUnnamed_24 = 2;
pub const FOWA_URGENT: C2RustUnnamed_24 = 1;
pub const FOWA_SMART: C2RustUnnamed_24 = 0;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const XCB_NONE: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[no_mangle]
pub static mut croot: *mut Con = ::core::ptr::null::<Con>() as *mut Con;
#[no_mangle]
pub static mut focused: *mut Con = ::core::ptr::null::<Con>() as *mut Con;
#[no_mangle]
pub static mut all_cons: all_cons_head = all_cons_head {
    tqh_first: ::core::ptr::null::<Con>() as *mut Con,
    tqh_last: ::core::ptr::null::<*mut Con>() as *mut *mut Con,
};
unsafe extern "C" fn _create___i3() -> *mut Con {
    let mut __i3: *mut Con = con_new(croot, ::core::ptr::null_mut::<i3Window>());
    free((*__i3).name as *mut ::core::ffi::c_void);
    (*__i3).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*__i3).name = sstrdup(b"__i3\0" as *const u8 as *const ::core::ffi::c_char);
    (*__i3).type_0 = CT_OUTPUT;
    (*__i3).layout = L_OUTPUT;
    con_fix_percent(croot);
    x_set_name(
        __i3,
        b"[i3 con] pseudo-output __i3\0" as *const u8 as *const ::core::ffi::c_char,
    );
    (*__i3).rect.width = 1280 as uint32_t;
    (*__i3).rect.height = 1024 as uint32_t;
    debuglog(
        b"%s:%s:%d - adding main content container\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/tree.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"_create___i3\0" as *const u8 as *const ::core::ffi::c_char,
        39 as ::core::ffi::c_int,
    );
    let mut content: *mut Con = con_new(
        ::core::ptr::null_mut::<Con>(),
        ::core::ptr::null_mut::<i3Window>(),
    );
    (*content).type_0 = CT_CON;
    free((*content).name as *mut ::core::ffi::c_void);
    (*content).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*content).name = sstrdup(b"content\0" as *const u8 as *const ::core::ffi::c_char);
    (*content).layout = L_SPLITH;
    x_set_name(
        content,
        b"[i3 con] content __i3\0" as *const u8 as *const ::core::ffi::c_char,
    );
    con_attach(content, __i3, false_0 != 0);
    let mut ws: *mut Con = con_new(
        ::core::ptr::null_mut::<Con>(),
        ::core::ptr::null_mut::<i3Window>(),
    );
    (*ws).type_0 = CT_WORKSPACE;
    (*ws).num = -(1 as ::core::ffi::c_int);
    (*ws).name = sstrdup(b"__i3_scratch\0" as *const u8 as *const ::core::ffi::c_char);
    (*ws).layout = L_SPLITH;
    con_attach(ws, content, false_0 != 0);
    x_set_name(
        ws,
        b"[i3 con] workspace __i3_scratch\0" as *const u8 as *const ::core::ffi::c_char,
    );
    (*ws).fullscreen_mode = CF_OUTPUT;
    return __i3;
}
#[no_mangle]
pub unsafe extern "C" fn tree_restore(
    mut path: *const ::core::ffi::c_char,
    mut geometry: *mut xcb_get_geometry_reply_t,
) -> bool {
    let mut len: ssize_t = 0;
    let mut out: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut ws: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut result: bool = false_0 != 0;
    let mut globbed: *mut ::core::ffi::c_char = resolve_tilde(path);
    let mut buf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if !path_exists(globbed) {
        verboselog(
            b"%s does not exist, not restoring tree\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            globbed,
        );
    } else {
        len = 0;
        len = slurp(globbed, &raw mut buf);
        if !(len < 0 as ssize_t) {
            croot = con_new(
                ::core::ptr::null_mut::<Con>(),
                ::core::ptr::null_mut::<i3Window>(),
            );
            (*croot).rect = Rect {
                x: (*geometry).x as uint32_t,
                y: (*geometry).y as uint32_t,
                width: (*geometry).width as uint32_t,
                height: (*geometry).height as uint32_t,
            } as Rect;
            focused = croot;
            tree_append_json(
                focused,
                buf,
                len as size_t,
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            );
            debuglog(
                b"%s:%s:%d - appended tree, using new root\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"../src/tree.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"tree_restore\0" as *const u8 as *const ::core::ffi::c_char,
                93 as ::core::ffi::c_int,
            );
            croot = (*croot).nodes_head.tqh_first as *mut Con;
            if !croot.is_null() {
                debuglog(
                    b"%s:%s:%d - new root = %p\n\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    b"../src/tree.c\0" as *const u8 as *const ::core::ffi::c_char,
                    b"tree_restore\0" as *const u8 as *const ::core::ffi::c_char,
                    99 as ::core::ffi::c_int,
                    croot,
                );
                out = (*croot).nodes_head.tqh_first as *mut Con;
                debuglog(
                    b"%s:%s:%d - out = %p\n\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    b"../src/tree.c\0" as *const u8 as *const ::core::ffi::c_char,
                    b"tree_restore\0" as *const u8 as *const ::core::ffi::c_char,
                    101 as ::core::ffi::c_int,
                    out,
                );
                ws = (*out).nodes_head.tqh_first as *mut Con;
                debuglog(
                    b"%s:%s:%d - ws = %p\n\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    b"../src/tree.c\0" as *const u8 as *const ::core::ffi::c_char,
                    b"tree_restore\0" as *const u8 as *const ::core::ffi::c_char,
                    103 as ::core::ffi::c_int,
                    ws,
                );
                if strcmp(
                    (*out).name,
                    b"__i3\0" as *const u8 as *const ::core::ffi::c_char,
                ) != 0 as ::core::ffi::c_int
                {
                    debuglog(
                        b"%s:%s:%d - Adding pseudo-output __i3 during inplace restart\n\0"
                            as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        b"../src/tree.c\0" as *const u8 as *const ::core::ffi::c_char,
                        b"tree_restore\0" as *const u8 as *const ::core::ffi::c_char,
                        108 as ::core::ffi::c_int,
                    );
                    let mut __i3: *mut Con = _create___i3();
                    if !(*__i3).nodes.tqe_next.is_null() {
                        (*(*__i3).nodes.tqe_next).nodes.tqe_prev = (*__i3).nodes.tqe_prev;
                    } else {
                        (*croot).nodes_head.tqh_last = (*__i3).nodes.tqe_prev;
                    }
                    *(*__i3).nodes.tqe_prev = (*__i3).nodes.tqe_next;
                    (*__i3).nodes.tqe_next = (*croot).nodes_head.tqh_first;
                    if !(*__i3).nodes.tqe_next.is_null() {
                        (*(*croot).nodes_head.tqh_first).nodes.tqe_prev =
                            &raw mut (*__i3).nodes.tqe_next;
                    } else {
                        (*croot).nodes_head.tqh_last = &raw mut (*__i3).nodes.tqe_next;
                    }
                    (*croot).nodes_head.tqh_first = __i3 as *mut Con;
                    (*__i3).nodes.tqe_prev = &raw mut (*croot).nodes_head.tqh_first;
                }
                restore_open_placeholder_windows(croot);
                result = true_0 != 0;
            }
        }
    }
    free(globbed as *mut ::core::ffi::c_void);
    free(buf as *mut ::core::ffi::c_void);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn tree_init(mut geometry: *mut xcb_get_geometry_reply_t) {
    croot = con_new(
        ::core::ptr::null_mut::<Con>(),
        ::core::ptr::null_mut::<i3Window>(),
    );
    free((*croot).name as *mut ::core::ffi::c_void);
    (*croot).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*croot).name =
        b"root\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
    (*croot).type_0 = CT_ROOT;
    (*croot).layout = L_SPLITH;
    (*croot).rect = Rect {
        x: (*geometry).x as uint32_t,
        y: (*geometry).y as uint32_t,
        width: (*geometry).width as uint32_t,
        height: (*geometry).height as uint32_t,
    } as Rect;
    _create___i3();
}
#[no_mangle]
pub unsafe extern "C" fn tree_open_con(mut con: *mut Con, mut window: *mut i3Window) -> *mut Con {
    if con.is_null() {
        con = (*focused).parent as *mut Con;
        if (*(*con).parent).type_0 as ::core::ffi::c_uint
            == CT_OUTPUT as ::core::ffi::c_int as ::core::ffi::c_uint
            && (*con).type_0 as ::core::ffi::c_uint
                != CT_DOCKAREA as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            con = focused;
        }
        if (*con).type_0 as ::core::ffi::c_uint
            == CT_FLOATING_CON as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            con = con_descend_tiling_focused((*con).parent as *mut Con);
            if (*con).type_0 as ::core::ffi::c_uint
                != CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                con = (*con).parent as *mut Con;
            }
        }
        debuglog(
            b"%s:%s:%d - con = %p\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b"../src/tree.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"tree_open_con\0" as *const u8 as *const ::core::ffi::c_char,
            168 as ::core::ffi::c_int,
            con,
        );
    }
    if con != ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut Con {
    } else {
        __assert_fail(
            b"con != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            b"../src/tree.c\0" as *const u8 as *const ::core::ffi::c_char,
            171 as ::core::ffi::c_uint,
            b"Con *tree_open_con(Con *, i3Window *)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
    let mut new: *mut Con = con_new(con, window);
    (*new).layout = L_SPLITH;
    con_fix_percent(con);
    return new;
}
#[no_mangle]
pub unsafe extern "C" fn tree_close_internal(
    mut con: *mut Con,
    mut kill_window: kill_window_t,
    mut dont_kill_parent: bool,
) -> bool {
    let mut parent: *mut Con = (*con).parent as *mut Con;
    if (*con).urgent {
        con_set_urgency(con, false_0 != 0);
        con_update_parents_urgency(con);
        workspace_update_urgent_flag(con_get_workspace(con));
    }
    debuglog(
        b"%s:%s:%d - closing %p, kill_window = %d\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/tree.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"tree_close_internal\0" as *const u8 as *const ::core::ffi::c_char,
        202 as ::core::ffi::c_int,
        con,
        kill_window as ::core::ffi::c_uint,
    );
    let mut abort_kill: bool = false_0 != 0;
    let mut child: *mut Con = (*con).nodes_head.tqh_first as *mut Con;
    while !child.is_null() {
        let mut next_child: *mut Con = (*child).nodes.tqe_next as *mut Con;
        debuglog(
            b"%s:%s:%d - killing child=%p\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b"../src/tree.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"tree_close_internal\0" as *const u8 as *const ::core::ffi::c_char,
            208 as ::core::ffi::c_int,
            child,
        );
        if !tree_close_internal(child, kill_window, true_0 != 0) {
            abort_kill = true_0 != 0;
        }
        child = next_child;
    }
    if abort_kill {
        debuglog(
            b"%s:%s:%d - One of the children could not be killed immediately (WM_DELETE sent), aborting.\n\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/tree.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"tree_close_internal\0" as *const u8 as *const ::core::ffi::c_char,
            216 as ::core::ffi::c_int,
        );
        return false_0 != 0;
    }
    if !(*con).window.is_null() {
        if kill_window as ::core::ffi::c_uint
            != DONT_KILL_WINDOW as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            x_window_kill((*(*con).window).id, kill_window);
            return false_0 != 0;
        }
        let mut fresh0: [uint32_t; 1] = [XCB_NONE as uint32_t];
        xcb_change_window_attributes(
            conn,
            (*(*con).window).id,
            XCB_CW_EVENT_MASK as ::core::ffi::c_int as uint32_t,
            &raw mut fresh0 as *mut uint32_t as *const ::core::ffi::c_void,
        );
        xcb_unmap_window(conn, (*(*con).window).id);
        let mut cookie: xcb_void_cookie_t = xcb_reparent_window(
            conn,
            (*(*con).window).id,
            root,
            (*con).rect.x as int16_t,
            (*con).rect.y as int16_t,
        );
        add_ignore_event(
            ignore_event_sequence_t {
                value: cookie.sequence as uint32_t,
            },
            0 as ::core::ffi::c_int,
        );
        let mut data: [::core::ffi::c_long; 2] = [
            XCB_ICCCM_WM_STATE_WITHDRAWN as ::core::ffi::c_int as ::core::ffi::c_long,
            XCB_NONE,
        ];
        cookie = xcb_change_property(
            conn,
            XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as uint8_t,
            (*(*con).window).id,
            A_WM_STATE,
            A_WM_STATE,
            32 as uint8_t,
            2 as uint32_t,
            &raw mut data as *mut ::core::ffi::c_long as *const ::core::ffi::c_void,
        );
        xcb_change_save_set(
            conn,
            XCB_SET_MODE_DELETE as ::core::ffi::c_int as uint8_t,
            (*(*con).window).id,
        );
        if shape_supported {
            xcb_shape_select_input(conn, (*(*con).window).id, false_0 as uint8_t);
        }
        add_ignore_event(
            ignore_event_sequence_t {
                value: cookie.sequence as uint32_t,
            },
            0 as ::core::ffi::c_int,
        );
        ipc_send_window_event(b"close\0" as *const u8 as *const ::core::ffi::c_char, con);
        window_free((*con).window as *mut i3Window);
        (*con).window = ::core::ptr::null_mut::<Window>();
    }
    let mut ws: *mut Con = con_get_workspace(con);
    let mut next: *mut Con = if con == focused {
        con_next_focused(con)
    } else {
        ::core::ptr::null_mut::<Con>()
    };
    debuglog(
        b"%s:%s:%d - next = %p, focused = %p\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/tree.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"tree_close_internal\0" as *const u8 as *const ::core::ffi::c_char,
        267 as ::core::ffi::c_int,
        next,
        focused,
    );
    con_detach(con);
    if !(*con).urgency_timer.is_null() {
        debuglog(
            b"%s:%s:%d - Removing urgency timer of con %p\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/tree.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"tree_close_internal\0" as *const u8 as *const ::core::ffi::c_char,
            274 as ::core::ffi::c_int,
            con,
        );
        workspace_update_urgent_flag(ws);
        ev_timer_stop(main_loop, (*con).urgency_timer as *mut ev_timer);
        free((*con).urgency_timer as *mut ::core::ffi::c_void);
        (*con).urgency_timer = ::core::ptr::null_mut::<ev_timer>();
    }
    if (*con).type_0 as ::core::ffi::c_uint
        != CT_FLOATING_CON as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        con_fix_percent(parent);
    }
    if !dont_kill_parent {
        tree_render();
    }
    x_con_kill(con);
    if ws == con {
        debuglog(
            b"%s:%s:%d - Closing workspace container %s, updating EWMH atoms\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/tree.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"tree_close_internal\0" as *const u8 as *const ::core::ffi::c_char,
            302 as ::core::ffi::c_int,
            (*ws).name,
        );
        ewmh_update_desktop_properties();
    }
    con_free(con);
    if !next.is_null() {
        con_activate(next);
    } else {
        debuglog(
            b"%s:%s:%d - not changing focus, the container was not focused before\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/tree.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"tree_close_internal\0" as *const u8 as *const ::core::ffi::c_char,
            311 as ::core::ffi::c_int,
        );
    }
    if !dont_kill_parent {
        (*parent)
            .on_remove_child
            .expect("non-null function pointer")(parent);
    }
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn tree_split(mut con: *mut Con, mut orientation: orientation_t) {
    if con_is_floating(con) {
        debuglog(
            b"%s:%s:%d - Floating containers can't be split.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/tree.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"tree_split\0" as *const u8 as *const ::core::ffi::c_char,
            329 as ::core::ffi::c_int,
        );
        return;
    }
    if (*con).type_0 as ::core::ffi::c_uint
        == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if con_num_children(con) < 2 as ::core::ffi::c_int {
            if con_num_children(con) == 0 as ::core::ffi::c_int {
                debuglog(
                    b"%s:%s:%d - Changing workspace_layout to L_DEFAULT\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    b"../src/tree.c\0" as *const u8 as *const ::core::ffi::c_char,
                    b"tree_split\0" as *const u8 as *const ::core::ffi::c_char,
                    336 as ::core::ffi::c_int,
                );
                (*con).workspace_layout = L_DEFAULT;
            }
            debuglog(
                b"%s:%s:%d - Changing orientation of workspace\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"../src/tree.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"tree_split\0" as *const u8 as *const ::core::ffi::c_char,
                339 as ::core::ffi::c_int,
            );
            (*con).layout = (if orientation as ::core::ffi::c_uint
                == HORIZ as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                L_SPLITH as ::core::ffi::c_int
            } else {
                L_SPLITV as ::core::ffi::c_int
            }) as layout_t;
            return;
        } else {
            con = workspace_encapsulate(con);
        }
    }
    let mut parent: *mut Con = (*con).parent as *mut Con;
    con_force_split_parents_redraw(con);
    if con_num_children(parent) == 1 as ::core::ffi::c_int
        && ((*parent).layout as ::core::ffi::c_uint
            == L_SPLITH as ::core::ffi::c_int as ::core::ffi::c_uint
            || (*parent).layout as ::core::ffi::c_uint
                == L_SPLITV as ::core::ffi::c_int as ::core::ffi::c_uint)
    {
        (*parent).layout = (if orientation as ::core::ffi::c_uint
            == HORIZ as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            L_SPLITH as ::core::ffi::c_int
        } else {
            L_SPLITV as ::core::ffi::c_int
        }) as layout_t;
        debuglog(
            b"%s:%s:%d - Just changing orientation of existing container\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/tree.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"tree_split\0" as *const u8 as *const ::core::ffi::c_char,
            361 as ::core::ffi::c_int,
        );
        return;
    }
    debuglog(
        b"%s:%s:%d - Splitting in orientation %d\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/tree.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"tree_split\0" as *const u8 as *const ::core::ffi::c_char,
        365 as ::core::ffi::c_int,
        orientation as ::core::ffi::c_uint,
    );
    let mut new: *mut Con = con_new(
        ::core::ptr::null_mut::<Con>(),
        ::core::ptr::null_mut::<i3Window>(),
    );
    (*new).nodes.tqe_next = (*con).nodes.tqe_next;
    if !(*new).nodes.tqe_next.is_null() {
        (*(*new).nodes.tqe_next).nodes.tqe_prev = &raw mut (*new).nodes.tqe_next;
    } else {
        (*parent).nodes_head.tqh_last = &raw mut (*new).nodes.tqe_next;
    }
    (*new).nodes.tqe_prev = (*con).nodes.tqe_prev;
    *(*new).nodes.tqe_prev = new as *mut Con;
    (*new).focused.tqe_next = (*con).focused.tqe_next;
    if !(*new).focused.tqe_next.is_null() {
        (*(*new).focused.tqe_next).focused.tqe_prev = &raw mut (*new).focused.tqe_next;
    } else {
        (*parent).focus_head.tqh_last = &raw mut (*new).focused.tqe_next;
    }
    (*new).focused.tqe_prev = (*con).focused.tqe_prev;
    *(*new).focused.tqe_prev = new as *mut Con;
    (*new).parent = parent as *mut Con;
    (*new).layout = (if orientation as ::core::ffi::c_uint
        == HORIZ as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        L_SPLITH as ::core::ffi::c_int
    } else {
        L_SPLITV as ::core::ffi::c_int
    }) as layout_t;
    (*new).percent = (*con).percent;
    (*con).percent = 0.0f64;
    con_attach(con, new, false_0 != 0);
}
#[no_mangle]
pub unsafe extern "C" fn level_up() -> bool {
    if (*(*focused).parent).type_0 as ::core::ffi::c_uint
        == CT_FLOATING_CON as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        con_activate((*(*focused).parent).parent as *mut Con);
        return true_0 != 0;
    }
    if (*(*focused).parent).type_0 as ::core::ffi::c_uint
        != CT_CON as ::core::ffi::c_int as ::core::ffi::c_uint
        && (*(*focused).parent).type_0 as ::core::ffi::c_uint
            != CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*focused).type_0 as ::core::ffi::c_uint
            == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        errorlog(
            b"ERROR: 'focus parent': Focus is already on the workspace, cannot go higher than that.\n\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        return false_0 != 0;
    }
    con_activate((*focused).parent as *mut Con);
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn level_down() -> bool {
    let mut next: *mut Con = (*focused).focus_head.tqh_first as *mut Con;
    if next.is_null() {
        debuglog(
            b"%s:%s:%d - cannot go down\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b"../src/tree.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"level_down\0" as *const u8 as *const ::core::ffi::c_char,
            413 as ::core::ffi::c_int,
        );
        return false_0 != 0;
    } else if (*next).type_0 as ::core::ffi::c_uint
        == CT_FLOATING_CON as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut child: *mut Con = (*next).focus_head.tqh_first as *mut Con;
        if child.is_null() {
            debuglog(
                b"%s:%s:%d - cannot go down\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"../src/tree.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"level_down\0" as *const u8 as *const ::core::ffi::c_char,
                420 as ::core::ffi::c_int,
            );
            return false_0 != 0;
        } else {
            next = (*next).focus_head.tqh_first as *mut Con;
        }
    }
    con_activate(next);
    return true_0 != 0;
}
unsafe extern "C" fn mark_unmapped(mut con: *mut Con) {
    let mut current: *mut Con = ::core::ptr::null_mut::<Con>();
    (*con).mapped = false_0 != 0;
    current = (*con).nodes_head.tqh_first as *mut Con;
    while !current.is_null() {
        mark_unmapped(current);
        current = (*current).nodes.tqe_next as *mut Con;
    }
    if (*con).type_0 as ::core::ffi::c_uint
        == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        current = (*con).floating_head.tqh_first as *mut Con;
        while !current.is_null() {
            mark_unmapped(current);
            current = (*current).floating_windows.tqe_next as *mut Con;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn tree_render() {
    if croot.is_null() {
        return;
    }
    debuglog(
        b"%s:%s:%d - -- BEGIN RENDERING --\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/tree.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"tree_render\0" as *const u8 as *const ::core::ffi::c_char,
        457 as ::core::ffi::c_int,
    );
    mark_unmapped(croot);
    (*croot).mapped = true_0 != 0;
    render_con(croot);
    x_push_changes(croot);
    debuglog(
        b"%s:%s:%d - -- END RENDERING --\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/tree.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"tree_render\0" as *const u8 as *const ::core::ffi::c_char,
        466 as ::core::ffi::c_int,
    );
}
unsafe extern "C" fn get_tree_next_workspace(
    mut con: *mut Con,
    mut direction: direction_t,
) -> *mut Con {
    if !con_get_fullscreen_con(con, CF_GLOBAL).is_null() {
        debuglog(
            b"%s:%s:%d - Cannot change workspace while in global fullscreen mode.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/tree.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"get_tree_next_workspace\0" as *const u8 as *const ::core::ffi::c_char,
            471 as ::core::ffi::c_int,
        );
        return ::core::ptr::null_mut::<Con>();
    }
    let x: uint32_t = (*con)
        .rect
        .x
        .wrapping_add((*con).rect.width.wrapping_div(2 as uint32_t));
    let y: uint32_t = (*con)
        .rect
        .y
        .wrapping_add((*con).rect.height.wrapping_div(2 as uint32_t));
    let mut current_output: *mut Output =
        get_output_containing(x as ::core::ffi::c_uint, y as ::core::ffi::c_uint);
    if current_output.is_null() {
        return ::core::ptr::null_mut::<Con>();
    }
    debuglog(
        b"%s:%s:%d - Current output is %s\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/tree.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"get_tree_next_workspace\0" as *const u8 as *const ::core::ffi::c_char,
        483 as ::core::ffi::c_int,
        output_primary_name(current_output),
    );
    let mut next_output: *mut Output = get_output_next(direction, current_output, CLOSEST_OUTPUT);
    if next_output.is_null() {
        return ::core::ptr::null_mut::<Con>();
    }
    debuglog(
        b"%s:%s:%d - Next output is %s\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/tree.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"get_tree_next_workspace\0" as *const u8 as *const ::core::ffi::c_char,
        489 as ::core::ffi::c_int,
        output_primary_name(next_output),
    );
    let mut workspace: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut child: *mut Con = -(1 as ::core::ffi::c_int) as *mut Con;
    while child == -(1 as ::core::ffi::c_int) as *mut Con && {
        child = ::core::ptr::null_mut::<Con>();
        true_0 != 0
    } {
        child = (*output_get_content((*next_output).con))
            .nodes_head
            .tqh_first as *mut Con;
        while !child.is_null() {
            if !workspace_is_visible(child) {
                child = (*child).nodes.tqe_next as *mut Con;
            } else {
                workspace = child;
                break;
            }
        }
    }
    return workspace;
}
unsafe extern "C" fn get_tree_next(mut con: *mut Con, mut direction: direction_t) -> *mut Con {
    let previous: bool = position_from_direction(direction) as ::core::ffi::c_uint
        == BEFORE as ::core::ffi::c_int as ::core::ffi::c_uint;
    let orientation: orientation_t = orientation_from_direction(direction) as orientation_t;
    let mut first_wrap: *mut Con = ::core::ptr::null_mut::<Con>();
    if !((*con).type_0 as ::core::ffi::c_uint
        == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint)
    {
        while (*con).type_0 as ::core::ffi::c_uint
            != CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            if (*con).fullscreen_mode as ::core::ffi::c_uint
                == CF_OUTPUT as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                con = con_get_workspace(con);
                break;
            } else {
                if (*con).fullscreen_mode as ::core::ffi::c_uint
                    == CF_GLOBAL as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    return first_wrap;
                }
                let parent: *mut Con = (*con).parent as *mut Con;
                if (*con).type_0 as ::core::ffi::c_uint
                    == CT_FLOATING_CON as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    if orientation as ::core::ffi::c_uint
                        != HORIZ as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        return ::core::ptr::null_mut::<Con>();
                    }
                    let mut next: *mut Con = if previous as ::core::ffi::c_int != 0 {
                        *(*((*con).floating_windows.tqe_prev as *mut floating_head)).tqh_last
                    } else {
                        (*con).floating_windows.tqe_next as *mut Con
                    };
                    if next.is_null() {
                        next = (if previous as ::core::ffi::c_int != 0 {
                            *(*((*parent).floating_head.tqh_last as *mut floating_head)).tqh_last
                        } else {
                            (*parent).floating_head.tqh_first
                        }) as *mut Con;
                    }
                    if !next.is_null() {
                    } else {
                        __assert_fail(
                            b"next\0" as *const u8 as *const ::core::ffi::c_char,
                            b"../src/tree.c\0" as *const u8 as *const ::core::ffi::c_char,
                            543 as ::core::ffi::c_uint,
                            b"Con *get_tree_next(Con *, direction_t)\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                    };
                    return next;
                }
                if con_num_children(parent) > 1 as ::core::ffi::c_int
                    && con_orientation(parent) as ::core::ffi::c_uint
                        == orientation as ::core::ffi::c_uint
                {
                    let next_0: *mut Con = if previous as ::core::ffi::c_int != 0 {
                        *(*((*con).nodes.tqe_prev as *mut nodes_head)).tqh_last
                    } else {
                        (*con).nodes.tqe_next as *mut Con
                    };
                    if !next_0.is_null()
                        && con_fullscreen_permits_focusing(next_0) as ::core::ffi::c_int != 0
                    {
                        return next_0;
                    }
                    let wrap: *mut Con = if previous as ::core::ffi::c_int != 0 {
                        *(*((*parent).nodes_head.tqh_last as *mut nodes_head)).tqh_last
                    } else {
                        (*parent).nodes_head.tqh_first as *mut Con
                    };
                    match config.focus_wrapping as ::core::ffi::c_uint {
                        3 | 1 => {
                            if first_wrap.is_null()
                                && con_fullscreen_permits_focusing(wrap) as ::core::ffi::c_int != 0
                            {
                                first_wrap = wrap;
                            }
                        }
                        2 => {
                            if !next_0.is_null() {
                                return ::core::ptr::null_mut::<Con>();
                            }
                            return if con_fullscreen_permits_focusing(wrap) as ::core::ffi::c_int
                                != 0
                            {
                                wrap
                            } else {
                                ::core::ptr::null_mut::<Con>()
                            };
                        }
                        0 | _ => {}
                    }
                }
                con = parent;
            }
        }
        if (*con).type_0 as ::core::ffi::c_uint
            == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
        {
        } else {
            __assert_fail(
                b"con->type == CT_WORKSPACE\0" as *const u8 as *const ::core::ffi::c_char,
                b"../src/tree.c\0" as *const u8 as *const ::core::ffi::c_char,
                579 as ::core::ffi::c_uint,
                b"Con *get_tree_next(Con *, direction_t)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        };
        if config.focus_wrapping as ::core::ffi::c_uint
            == FOCUS_WRAPPING_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return first_wrap;
        }
    }
    let mut workspace: *mut Con = get_tree_next_workspace(con, direction);
    return if !workspace.is_null() {
        workspace
    } else {
        first_wrap
    };
}
#[no_mangle]
pub unsafe extern "C" fn tree_next(mut con: *mut Con, mut direction: direction_t) {
    let mut next: *mut Con = get_tree_next(con, direction);
    if next.is_null() {
        return;
    }
    if (*next).type_0 as ::core::ffi::c_uint
        == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut focus: *mut Con = con_descend_focused(next);
        if (*focus).fullscreen_mode as ::core::ffi::c_uint
            == CF_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            let mut focus_tiling: *mut Con = con_descend_tiling_focused(next);
            if focus_tiling != next {
                focus = focus_tiling;
            }
        }
        workspace_show(next);
        con_activate(focus);
        x_set_warp_to(&raw mut (*focus).rect);
        return;
    } else if (*next).type_0 as ::core::ffi::c_uint
        == CT_FLOATING_CON as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut parent: *mut Con = (*next).parent as *mut Con;
        while *(*((*parent).floating_head.tqh_last as *mut floating_head)).tqh_last != next {
            let mut last: *mut Con =
                *(*((*parent).floating_head.tqh_last as *mut floating_head)).tqh_last;
            if !(*last).floating_windows.tqe_next.is_null() {
                (*(*last).floating_windows.tqe_next)
                    .floating_windows
                    .tqe_prev = (*last).floating_windows.tqe_prev;
            } else {
                (*parent).floating_head.tqh_last = (*last).floating_windows.tqe_prev;
            }
            *(*last).floating_windows.tqe_prev = (*last).floating_windows.tqe_next;
            (*last).floating_windows.tqe_next = (*parent).floating_head.tqh_first;
            if !(*last).floating_windows.tqe_next.is_null() {
                (*(*parent).floating_head.tqh_first)
                    .floating_windows
                    .tqe_prev = &raw mut (*last).floating_windows.tqe_next;
            } else {
                (*parent).floating_head.tqh_last = &raw mut (*last).floating_windows.tqe_next;
            }
            (*parent).floating_head.tqh_first = last as *mut Con;
            (*last).floating_windows.tqe_prev = &raw mut (*parent).floating_head.tqh_first;
        }
    }
    workspace_show(con_get_workspace(next));
    con_activate(con_descend_focused(next));
}
#[no_mangle]
pub unsafe extern "C" fn get_tree_next_sibling(
    mut con: *mut Con,
    mut direction: position_t,
) -> *mut Con {
    let mut to_focus: *mut Con = if direction as ::core::ffi::c_uint
        == BEFORE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        *(*((*con).nodes.tqe_prev as *mut nodes_head)).tqh_last
    } else {
        (*con).nodes.tqe_next as *mut Con
    };
    if !to_focus.is_null() && con_fullscreen_permits_focusing(to_focus) as ::core::ffi::c_int != 0 {
        return to_focus;
    }
    return ::core::ptr::null_mut::<Con>();
}
#[no_mangle]
pub unsafe extern "C" fn tree_flatten(mut con: *mut Con) {
    let mut focus_next: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut current: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut child: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut parent: *mut Con = (*con).parent as *mut Con;
    debuglog(
        b"%s:%s:%d - Checking if I can flatten con = %p / %s\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/tree.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"tree_flatten\0" as *const u8 as *const ::core::ffi::c_char,
        659 as ::core::ffi::c_int,
        con,
        (*con).name,
    );
    if !((*con).type_0 as ::core::ffi::c_uint
        != CT_CON as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*parent).layout as ::core::ffi::c_uint
            == L_OUTPUT as ::core::ffi::c_int as ::core::ffi::c_uint
        || !(*con).window.is_null())
    {
        child = (*con).nodes_head.tqh_first as *mut Con;
        if !(child.is_null() || !(*child).nodes.tqe_next.is_null()) {
            debuglog(
                b"%s:%s:%d - child = %p, con = %p, parent = %p\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"../src/tree.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"tree_flatten\0" as *const u8 as *const ::core::ffi::c_char,
                674 as ::core::ffi::c_int,
                child,
                con,
                parent,
            );
            if !(!con_is_split(con)
                || !con_is_split(child)
                || (*con).layout as ::core::ffi::c_uint
                    != L_SPLITH as ::core::ffi::c_int as ::core::ffi::c_uint
                    && (*con).layout as ::core::ffi::c_uint
                        != L_SPLITV as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*child).layout as ::core::ffi::c_uint
                    != L_SPLITH as ::core::ffi::c_int as ::core::ffi::c_uint
                    && (*child).layout as ::core::ffi::c_uint
                        != L_SPLITV as ::core::ffi::c_int as ::core::ffi::c_uint
                || con_orientation(con) as ::core::ffi::c_uint
                    == con_orientation(child) as ::core::ffi::c_uint
                || con_orientation(child) as ::core::ffi::c_uint
                    != con_orientation(parent) as ::core::ffi::c_uint)
            {
                debuglog(
                    b"%s:%s:%d - Alright, I have to flatten this situation now. Stay calm.\n\0"
                        as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    b"../src/tree.c\0" as *const u8 as *const ::core::ffi::c_char,
                    b"tree_flatten\0" as *const u8 as *const ::core::ffi::c_char,
                    687 as ::core::ffi::c_int,
                );
                focus_next = (*child).focus_head.tqh_first as *mut Con;
                debuglog(
                    b"%s:%s:%d - detaching...\n\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    b"../src/tree.c\0" as *const u8 as *const ::core::ffi::c_char,
                    b"tree_flatten\0" as *const u8 as *const ::core::ffi::c_char,
                    691 as ::core::ffi::c_int,
                );
                while !(*child).nodes_head.tqh_first.is_null() {
                    current = (*child).nodes_head.tqh_first as *mut Con;
                    debuglog(
                        b"%s:%s:%d - detaching current=%p / %s\n\0" as *const u8
                            as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        b"../src/tree.c\0" as *const u8 as *const ::core::ffi::c_char,
                        b"tree_flatten\0" as *const u8 as *const ::core::ffi::c_char,
                        695 as ::core::ffi::c_int,
                        current,
                        (*current).name,
                    );
                    con_detach(current);
                    debuglog(
                        b"%s:%s:%d - re-attaching\n\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        b"../src/tree.c\0" as *const u8 as *const ::core::ffi::c_char,
                        b"tree_flatten\0" as *const u8 as *const ::core::ffi::c_char,
                        697 as ::core::ffi::c_int,
                    );
                    (*current).parent = parent as *mut Con;
                    (*current).nodes.tqe_prev = (*con).nodes.tqe_prev;
                    (*current).nodes.tqe_next = con as *mut Con;
                    *(*con).nodes.tqe_prev = current as *mut Con;
                    (*con).nodes.tqe_prev = &raw mut (*current).nodes.tqe_next;
                    debuglog(
                        b"%s:%s:%d - attaching to focus list\n\0" as *const u8
                            as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        b"../src/tree.c\0" as *const u8 as *const ::core::ffi::c_char,
                        b"tree_flatten\0" as *const u8 as *const ::core::ffi::c_char,
                        704 as ::core::ffi::c_int,
                    );
                    (*current).focused.tqe_next = ::core::ptr::null_mut::<Con>();
                    (*current).focused.tqe_prev = (*parent).focus_head.tqh_last;
                    *(*parent).focus_head.tqh_last = current as *mut Con;
                    (*parent).focus_head.tqh_last = &raw mut (*current).focused.tqe_next;
                    (*current).percent = (*con).percent;
                }
                debuglog(
                    b"%s:%s:%d - re-attached all\n\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    b"../src/tree.c\0" as *const u8 as *const ::core::ffi::c_char,
                    b"tree_flatten\0" as *const u8 as *const ::core::ffi::c_char,
                    708 as ::core::ffi::c_int,
                );
                if !focus_next.is_null() && (*parent).focus_head.tqh_first == con {
                    debuglog(
                        b"%s:%s:%d - restoring focus to focus_next=%p\n\0" as *const u8
                            as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        b"../src/tree.c\0" as *const u8 as *const ::core::ffi::c_char,
                        b"tree_flatten\0" as *const u8 as *const ::core::ffi::c_char,
                        713 as ::core::ffi::c_int,
                        focus_next,
                    );
                    if !(*focus_next).focused.tqe_next.is_null() {
                        (*(*focus_next).focused.tqe_next).focused.tqe_prev =
                            (*focus_next).focused.tqe_prev;
                    } else {
                        (*parent).focus_head.tqh_last = (*focus_next).focused.tqe_prev;
                    }
                    *(*focus_next).focused.tqe_prev = (*focus_next).focused.tqe_next;
                    (*focus_next).focused.tqe_next = (*parent).focus_head.tqh_first;
                    if !(*focus_next).focused.tqe_next.is_null() {
                        (*(*parent).focus_head.tqh_first).focused.tqe_prev =
                            &raw mut (*focus_next).focused.tqe_next;
                    } else {
                        (*parent).focus_head.tqh_last = &raw mut (*focus_next).focused.tqe_next;
                    }
                    (*parent).focus_head.tqh_first = focus_next as *mut Con;
                    (*focus_next).focused.tqe_prev = &raw mut (*parent).focus_head.tqh_first;
                    debuglog(
                        b"%s:%s:%d - restored focus.\n\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        b"../src/tree.c\0" as *const u8 as *const ::core::ffi::c_char,
                        b"tree_flatten\0" as *const u8 as *const ::core::ffi::c_char,
                        716 as ::core::ffi::c_int,
                    );
                }
                debuglog(
                    b"%s:%s:%d - closing redundant cons\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    b"../src/tree.c\0" as *const u8 as *const ::core::ffi::c_char,
                    b"tree_flatten\0" as *const u8 as *const ::core::ffi::c_char,
                    720 as ::core::ffi::c_int,
                );
                tree_close_internal(con, DONT_KILL_WINDOW, true_0 != 0);
                return;
            }
        }
    }
    current = (*con).nodes_head.tqh_first as *mut Con;
    while !current.is_null() {
        let mut next: *mut Con = (*current).nodes.tqe_next as *mut Con;
        tree_flatten(current);
        current = next;
    }
    current = (*con).floating_head.tqh_first as *mut Con;
    while !current.is_null() {
        let mut next_0: *mut Con = (*current).floating_windows.tqe_next as *mut Con;
        tree_flatten(current);
        current = next_0;
    }
}
unsafe extern "C" fn run_static_initializers() {
    all_cons = all_cons_head {
        tqh_first: ::core::ptr::null_mut::<Con>(),
        tqh_last: &raw mut all_cons.tqh_first,
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
