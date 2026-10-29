/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * render.c: Renders (determines position/sizes) the layout tree, updating the
 *           various rects. Needs to be pushed to X11 (see x.c) to be visible.
 *
 */
// Mechanical file-by-file Rust translation of src/render.c.
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
    fn debuglog(fmt: *mut ::core::ffi::c_char, ...);
    fn smalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn rect_add(a: Rect, b: Rect) -> Rect;
    fn rect_sanitize_dimensions(rect: Rect) -> Rect;
    static mut config: Config;
    fn x_raise_con(con: *const Con);
    fn calculate_effective_gaps(con: *mut Con) -> gaps_t;
    fn gaps_should_inset_con(con: *mut Con, children: ::core::ffi::c_int) -> bool;
    fn gaps_has_adjacent_container(con: *mut Con, direction: direction_t) -> bool;
    fn con_is_leaf(con: *mut Con) -> bool;
    fn con_get_fullscreen_con(con: *mut Con, fullscreen_mode: fullscreen_mode_t) -> *mut Con;
    fn con_get_fullscreen_covering_ws(ws: *mut Con) -> *mut Con;
    fn con_is_internal(con: *mut Con) -> bool;
    fn con_has_parent(con: *mut Con, parent: *mut Con) -> bool;
    fn con_find_transient_for_window(start: *mut Con, target: xcb_window_t) -> bool;
    fn con_num_children(con: *mut Con) -> ::core::ffi::c_int;
    fn con_descend_focused(con: *mut Con) -> *mut Con;
    fn con_border_style_rect(con: *mut Con) -> Rect;
    fn con_rect_size_in_orientation(con: *mut Con) -> uint32_t;
    fn output_get_content(output: *mut Con) -> *mut Con;
    fn floor(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn lround(__x: ::core::ffi::c_double) -> ::core::ffi::c_long;
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
    pub type_0: C2RustUnnamed_18,
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
    pub floating: C2RustUnnamed_7,
    pub nodes: C2RustUnnamed_6,
    pub focused: C2RustUnnamed_5,
    pub all_cons: C2RustUnnamed_4,
    pub floating_windows: C2RustUnnamed_3,
    pub on_remove_child: Option<unsafe extern "C" fn(*mut Con) -> ()>,
    pub scratchpad_state: C2RustUnnamed_2,
    pub old_id: ::core::ffi::c_int,
    pub depth: uint16_t,
    pub colormap: xcb_colormap_t,
}
pub type C2RustUnnamed_2 = ::core::ffi::c_uint;
pub const SCRATCHPAD_CHANGED: C2RustUnnamed_2 = 2;
pub const SCRATCHPAD_FRESH: C2RustUnnamed_2 = 1;
pub const SCRATCHPAD_NONE: C2RustUnnamed_2 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub tqe_next: *mut Con,
    pub tqe_prev: *mut *mut Con,
}
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
pub type C2RustUnnamed_7 = ::core::ffi::c_uint;
pub const FLOATING_USER_ON: C2RustUnnamed_7 = 3;
pub const FLOATING_AUTO_ON: C2RustUnnamed_7 = 2;
pub const FLOATING_USER_OFF: C2RustUnnamed_7 = 1;
pub const FLOATING_AUTO_OFF: C2RustUnnamed_7 = 0;
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
    pub urgent: C2RustUnnamed_12,
    pub dock: C2RustUnnamed_11,
    pub id: xcb_window_t,
    pub window_mode: C2RustUnnamed_10,
    pub con_id: *mut Con,
    pub match_all_windows: bool,
    pub insert_where: C2RustUnnamed_9,
    pub matches: C2RustUnnamed_8,
    pub restart_mode: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub tqe_next: *mut Match,
    pub tqe_prev: *mut *mut Match,
}
pub type C2RustUnnamed_9 = ::core::ffi::c_uint;
pub const M_BELOW: C2RustUnnamed_9 = 2;
pub const M_ASSIGN_WS: C2RustUnnamed_9 = 1;
pub const M_HERE: C2RustUnnamed_9 = 0;
pub type C2RustUnnamed_10 = ::core::ffi::c_uint;
pub const WM_FLOATING: C2RustUnnamed_10 = 6;
pub const WM_FLOATING_USER: C2RustUnnamed_10 = 5;
pub const WM_FLOATING_AUTO: C2RustUnnamed_10 = 4;
pub const WM_TILING: C2RustUnnamed_10 = 3;
pub const WM_TILING_USER: C2RustUnnamed_10 = 2;
pub const WM_TILING_AUTO: C2RustUnnamed_10 = 1;
pub const WM_ANY: C2RustUnnamed_10 = 0;
pub type C2RustUnnamed_11 = ::core::ffi::c_int;
pub const M_DOCK_BOTTOM: C2RustUnnamed_11 = 3;
pub const M_DOCK_TOP: C2RustUnnamed_11 = 2;
pub const M_DOCK_ANY: C2RustUnnamed_11 = 1;
pub const M_NODOCK: C2RustUnnamed_11 = 0;
pub const M_DONTCHECK: C2RustUnnamed_11 = -1;
pub type C2RustUnnamed_12 = ::core::ffi::c_int;
pub const U_OLDEST: C2RustUnnamed_12 = 1;
pub const U_LATEST: C2RustUnnamed_12 = 0;
pub const U_DONTCHECK: C2RustUnnamed_12 = -1;
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
    pub dock: C2RustUnnamed_13,
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
pub type C2RustUnnamed_13 = ::core::ffi::c_uint;
pub const W_DOCK_BOTTOM: C2RustUnnamed_13 = 2;
pub const W_DOCK_TOP: C2RustUnnamed_13 = 1;
pub const W_NODOCK: C2RustUnnamed_13 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Assignment {
    pub type_0: C2RustUnnamed_16,
    pub match_0: Match,
    pub dest: C2RustUnnamed_15,
    pub assignments: C2RustUnnamed_14,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub tqe_next: *mut Assignment,
    pub tqe_prev: *mut *mut Assignment,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_15 {
    pub command: *mut ::core::ffi::c_char,
    pub workspace: *mut ::core::ffi::c_char,
    pub output: *mut ::core::ffi::c_char,
}
pub type C2RustUnnamed_16 = ::core::ffi::c_uint;
pub const A_TO_OUTPUT: C2RustUnnamed_16 = 16;
pub const A_TO_WORKSPACE_NUMBER: C2RustUnnamed_16 = 8;
pub const A_NO_FOCUS: C2RustUnnamed_16 = 4;
pub const A_TO_WORKSPACE: C2RustUnnamed_16 = 2;
pub const A_COMMAND: C2RustUnnamed_16 = 1;
pub const A_ANY: C2RustUnnamed_16 = 0;
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
    pub marks: C2RustUnnamed_17,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_17 {
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
pub type C2RustUnnamed_18 = ::core::ffi::c_uint;
pub const CT_DOCKAREA: C2RustUnnamed_18 = 5;
pub const CT_WORKSPACE: C2RustUnnamed_18 = 4;
pub const CT_FLOATING_CON: C2RustUnnamed_18 = 3;
pub const CT_CON: C2RustUnnamed_18 = 2;
pub const CT_OUTPUT: C2RustUnnamed_18 = 1;
pub const CT_ROOT: C2RustUnnamed_18 = 0;
pub type direction_t = ::core::ffi::c_uint;
pub const D_DOWN: direction_t = 3;
pub const D_UP: direction_t = 2;
pub const D_RIGHT: direction_t = 1;
pub const D_LEFT: direction_t = 0;
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
    pub focus_on_window_activation: C2RustUnnamed_21,
    pub show_marks: bool,
    pub title_align: C2RustUnnamed_20,
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
    pub popup_during_fullscreen: C2RustUnnamed_19,
    pub number_barconfigs: ::core::ffi::c_int,
    pub tiling_drag: tiling_drag_t,
    pub gaps: gaps_t,
    pub smart_gaps: smart_gaps_t,
}
pub type C2RustUnnamed_19 = ::core::ffi::c_uint;
pub const PDF_ALL: C2RustUnnamed_19 = 3;
pub const PDF_IGNORE: C2RustUnnamed_19 = 2;
pub const PDF_LEAVE_FULLSCREEN: C2RustUnnamed_19 = 1;
pub const PDF_SMART: C2RustUnnamed_19 = 0;
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
pub type C2RustUnnamed_20 = ::core::ffi::c_uint;
pub const ALIGN_RIGHT: C2RustUnnamed_20 = 2;
pub const ALIGN_CENTER: C2RustUnnamed_20 = 1;
pub const ALIGN_LEFT: C2RustUnnamed_20 = 0;
pub type C2RustUnnamed_21 = ::core::ffi::c_uint;
pub const FOWA_NONE: C2RustUnnamed_21 = 3;
pub const FOWA_FOCUS: C2RustUnnamed_21 = 2;
pub const FOWA_URGENT: C2RustUnnamed_21 = 1;
pub const FOWA_SMART: C2RustUnnamed_21 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct render_params {
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub deco_height: ::core::ffi::c_int,
    pub rect: Rect,
    pub children: ::core::ffi::c_int,
    pub sizes: *mut ::core::ffi::c_int,
}
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[no_mangle]
pub unsafe extern "C" fn render_deco_height() -> ::core::ffi::c_int {
    let mut deco_height: ::core::ffi::c_int = config.font.height + 4 as ::core::ffi::c_int;
    if config.font.height & 0x1 as ::core::ffi::c_int != 0 {
        deco_height += 1;
    }
    return deco_height;
}
#[no_mangle]
pub unsafe extern "C" fn render_con(mut con: *mut Con) {
    let mut params: render_params = render_params {
        x: (*con).rect.x as ::core::ffi::c_int,
        y: (*con).rect.y as ::core::ffi::c_int,
        deco_height: 0,
        rect: (*con).rect as Rect,
        children: con_num_children(con),
        sizes: ::core::ptr::null_mut::<::core::ffi::c_int>(),
    };
    debuglog(
        b"%s:%s:%d - Rendering node %p / %s / layout %d / children %d\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/render.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"render_con\0" as *const u8 as *const ::core::ffi::c_char,
        51 as ::core::ffi::c_int,
        con,
        (*con).name,
        (*con).layout as ::core::ffi::c_uint,
        params.children,
    );
    if (*con).type_0 as ::core::ffi::c_uint
        == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut gaps: gaps_t = calculate_effective_gaps(con);
        let mut inset: Rect = Rect {
            x: gaps.left as uint32_t,
            y: gaps.top as uint32_t,
            width: -(gaps.left + gaps.right) as uint32_t,
            height: -(gaps.top + gaps.bottom) as uint32_t,
        };
        (*con).rect = rect_add((*con).rect as Rect, inset) as Rect;
        params.rect = rect_add(params.rect, inset);
        params.x += gaps.left;
        params.y += gaps.top;
    }
    if gaps_should_inset_con(con, params.children) {
        let mut gaps_0: gaps_t = calculate_effective_gaps(con);
        let mut inset_0: Rect = Rect {
            x: (if gaps_has_adjacent_container(con, D_LEFT) as ::core::ffi::c_int != 0 {
                gaps_0.inner / 2 as ::core::ffi::c_int
            } else {
                gaps_0.inner
            }) as uint32_t,
            y: (if gaps_has_adjacent_container(con, D_UP) as ::core::ffi::c_int != 0 {
                gaps_0.inner / 2 as ::core::ffi::c_int
            } else {
                gaps_0.inner
            }) as uint32_t,
            width: (if gaps_has_adjacent_container(con, D_RIGHT) as ::core::ffi::c_int != 0 {
                -(gaps_0.inner / 2 as ::core::ffi::c_int)
            } else {
                -gaps_0.inner
            }) as uint32_t,
            height: (if gaps_has_adjacent_container(con, D_DOWN) as ::core::ffi::c_int != 0 {
                -(gaps_0.inner / 2 as ::core::ffi::c_int)
            } else {
                -gaps_0.inner
            }) as uint32_t,
        };
        inset_0.width = inset_0.width.wrapping_sub(inset_0.x);
        inset_0.height = inset_0.height.wrapping_sub(inset_0.y);
        if (*con).fullscreen_mode as ::core::ffi::c_uint
            == CF_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            params.rect = rect_add(params.rect, inset_0);
            (*con).rect = rect_add((*con).rect as Rect, inset_0) as Rect;
        }
        inset_0.height = 0 as uint32_t;
        params.x = (*con).rect.x as ::core::ffi::c_int;
        params.y = (*con).rect.y as ::core::ffi::c_int;
    }
    (*con).mapped = true_0 != 0;
    if !(*con).window.is_null() {
        let mut inset_1: Rect = Rect {
            x: 0 as uint32_t,
            y: 0 as uint32_t,
            width: (*con).rect.width,
            height: (*con).rect.height,
        };
        if (*con).fullscreen_mode as ::core::ffi::c_uint
            == CF_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            debuglog(
                b"%s:%s:%d - deco_rect.height = %d\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"../src/render.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"render_con\0" as *const u8 as *const ::core::ffi::c_char,
                101 as ::core::ffi::c_int,
                (*con).deco_rect.height,
            );
            let mut bsr: Rect = con_border_style_rect(con);
            debuglog(
                b"%s:%s:%d - bsr at %dx%d with size %dx%d\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"../src/render.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"render_con\0" as *const u8 as *const ::core::ffi::c_char,
                104 as ::core::ffi::c_int,
                bsr.x,
                bsr.y,
                bsr.width,
                bsr.height,
            );
            inset_1 = rect_add(inset_1, bsr);
        }
        inset_1.width = inset_1
            .width
            .wrapping_sub((2 as ::core::ffi::c_int * (*con).border_width) as uint32_t);
        inset_1.height = inset_1
            .height
            .wrapping_sub((2 as ::core::ffi::c_int * (*con).border_width) as uint32_t);
        inset_1 = rect_sanitize_dimensions(inset_1);
        (*con).window_rect = inset_1 as Rect;
        debuglog(
            b"%s:%s:%d - child will be at %dx%d with size %dx%d\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/render.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"render_con\0" as *const u8 as *const ::core::ffi::c_char,
            123 as ::core::ffi::c_int,
            inset_1.x,
            inset_1.y,
            inset_1.width,
            inset_1.height,
        );
    }
    let mut fullscreen: *mut Con = ::core::ptr::null_mut::<Con>();
    if (*con).type_0 as ::core::ffi::c_uint
        != CT_OUTPUT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        fullscreen = con_get_fullscreen_con(
            con,
            (if (*con).type_0 as ::core::ffi::c_uint
                == CT_ROOT as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                CF_GLOBAL as ::core::ffi::c_int
            } else {
                CF_OUTPUT as ::core::ffi::c_int
            }) as fullscreen_mode_t,
        );
    }
    if !fullscreen.is_null() {
        (*fullscreen).rect = params.rect as Rect;
        x_raise_con(fullscreen);
        render_con(fullscreen);
        if (*con).type_0 as ::core::ffi::c_uint
            != CT_ROOT as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return;
        }
    }
    params.deco_height = render_deco_height();
    params.sizes = precalculate_sizes(con, &raw mut params);
    if (*con).layout as ::core::ffi::c_uint == L_OUTPUT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if !con_is_internal(con) {
            render_output(con);
        }
    } else if (*con).type_0 as ::core::ffi::c_uint
        == CT_ROOT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        render_root(con, fullscreen);
    } else {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut child: *mut Con = ::core::ptr::null_mut::<Con>();
        child = (*con).nodes_head.tqh_first as *mut Con;
        while !child.is_null() {
            if params.children > 0 as ::core::ffi::c_int {
            } else {
                __assert_fail(
                    b"params.children > 0\0" as *const u8 as *const ::core::ffi::c_char,
                    b"../src/render.c\0" as *const u8 as *const ::core::ffi::c_char,
                    164 as ::core::ffi::c_uint,
                    b"void render_con(Con *)\0" as *const u8 as *const ::core::ffi::c_char,
                );
            };
            if (*con).layout as ::core::ffi::c_uint
                == L_SPLITH as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*con).layout as ::core::ffi::c_uint
                    == L_SPLITV as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                render_con_split(con, child, &raw mut params, i);
            } else if (*con).layout as ::core::ffi::c_uint
                == L_STACKED as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                render_con_stacked(con, child, &raw mut params, i);
            } else if (*con).layout as ::core::ffi::c_uint
                == L_TABBED as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                render_con_tabbed(con, child, &raw mut params, i);
            } else if (*con).layout as ::core::ffi::c_uint
                == L_DOCKAREA as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                render_con_dockarea(con, child, &raw mut params);
            }
            (*child).rect = rect_sanitize_dimensions((*child).rect as Rect) as Rect;
            debuglog(
                b"%s:%s:%d - child at (%d, %d) with (%d x %d)\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"../src/render.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"render_con\0" as *const u8 as *const ::core::ffi::c_char,
                179 as ::core::ffi::c_int,
                (*child).rect.x,
                (*child).rect.y,
                (*child).rect.width,
                (*child).rect.height,
            );
            x_raise_con(child);
            render_con(child);
            if (*con).layout as ::core::ffi::c_uint
                == L_SPLITH as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*con).layout as ::core::ffi::c_uint
                    == L_SPLITV as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                if con_is_leaf(child) {
                    if (*child).border_style as ::core::ffi::c_uint
                        == BS_NORMAL as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        (*child).deco_rect.width = (*child).rect.width;
                    }
                }
            }
            i += 1;
            child = (*child).nodes.tqe_next as *mut Con;
        }
        if (*con).layout as ::core::ffi::c_uint
            == L_STACKED as ::core::ffi::c_int as ::core::ffi::c_uint
            || (*con).layout as ::core::ffi::c_uint
                == L_TABBED as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            child = *(*((*con).focus_head.tqh_last as *mut focus_head)).tqh_last as *mut Con;
            while !child.is_null() {
                x_raise_con(child);
                child = *(*((*child).focused.tqe_prev as *mut focus_head)).tqh_last as *mut Con;
            }
            child = (*con).focus_head.tqh_first as *mut Con;
            if !child.is_null() {
                render_con(child);
            }
            if params.children != 1 as ::core::ffi::c_int {
                x_raise_con(con);
            }
        }
    }
    free(params.sizes as *mut ::core::ffi::c_void);
    params.sizes = ::core::ptr::null_mut::<::core::ffi::c_int>();
}
unsafe extern "C" fn precalculate_sizes(
    mut con: *mut Con,
    mut p: *mut render_params,
) -> *mut ::core::ffi::c_int {
    if (*con).layout as ::core::ffi::c_uint != L_SPLITH as ::core::ffi::c_int as ::core::ffi::c_uint
        && (*con).layout as ::core::ffi::c_uint
            != L_SPLITV as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*p).children <= 0 as ::core::ffi::c_int
    {
        return ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
    let mut sizes: *mut ::core::ffi::c_int = smalloc(
        ((*p).children as size_t)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
    ) as *mut ::core::ffi::c_int;
    if !(*con).nodes_head.tqh_first.is_null() as ::core::ffi::c_int != 0 {
    } else {
        __assert_fail(
            b"!TAILQ_EMPTY(&con->nodes_head)\0" as *const u8 as *const ::core::ffi::c_char,
            b"../src/render.c\0" as *const u8 as *const ::core::ffi::c_char,
            230 as ::core::ffi::c_uint,
            b"int *precalculate_sizes(Con *, render_params *)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    };
    let mut child: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut assigned: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let total: ::core::ffi::c_int = con_rect_size_in_orientation(con) as ::core::ffi::c_int;
    child = (*con).nodes_head.tqh_first as *mut Con;
    while !child.is_null() {
        let percentage: ::core::ffi::c_double = if (*child).percent > 0.0f64 {
            (*child).percent
        } else {
            1.0f64 / (*p).children as ::core::ffi::c_double
        };
        let fresh0 = i;
        i = i + 1;
        let ref mut fresh1 = *sizes.offset(fresh0 as isize);
        *fresh1 = lround(percentage * total as ::core::ffi::c_double) as ::core::ffi::c_int;
        assigned += *fresh1;
        child = (*child).nodes.tqe_next as *mut Con;
    }
    if assigned == total
        || assigned > total && assigned - total <= (*p).children * 2 as ::core::ffi::c_int
        || assigned < total && total - assigned <= (*p).children * 2 as ::core::ffi::c_int
    {
    } else {
        __assert_fail(
            b"assigned == total || (assigned > total && assigned - total <= p->children * 2) || (assigned < total && total - assigned <= p->children * 2)\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"../src/render.c\0" as *const u8 as *const ::core::ffi::c_char,
            241 as ::core::ffi::c_uint,
            b"int *precalculate_sizes(Con *, render_params *)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    };
    let mut signal: ::core::ffi::c_int = if assigned < total {
        1 as ::core::ffi::c_int
    } else {
        -(1 as ::core::ffi::c_int)
    };
    while assigned != total {
        i = 0 as ::core::ffi::c_int;
        while i < (*p).children && assigned != total {
            *sizes.offset(i as isize) += signal;
            assigned += signal;
            i += 1;
        }
    }
    return sizes;
}
unsafe extern "C" fn fullscreen_blocks_floating_render(
    mut fullscreen: *mut Con,
    mut floating: *mut Con,
) -> bool {
    if fullscreen.is_null() {
        return false_0 != 0;
    }
    match config.popup_during_fullscreen as ::core::ffi::c_uint {
        1 | 2 => return true_0 != 0,
        0 => {
            return (*fullscreen).window.is_null()
                || !con_find_transient_for_window(
                    con_descend_focused(floating),
                    (*(*fullscreen).window).id,
                );
        }
        3 => return con_has_parent(fullscreen, floating),
        _ => {}
    }
    return false_0 != 0;
}
unsafe extern "C" fn render_root(mut con: *mut Con, mut fullscreen: *mut Con) {
    let mut output: *mut Con = ::core::ptr::null_mut::<Con>();
    if fullscreen.is_null() {
        output = (*con).nodes_head.tqh_first as *mut Con;
        while !output.is_null() {
            render_con(output);
            output = (*output).nodes.tqe_next as *mut Con;
        }
    }
    debuglog(
        b"%s:%s:%d - Rendering floating windows:\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/render.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"render_root\0" as *const u8 as *const ::core::ffi::c_char,
        285 as ::core::ffi::c_int,
    );
    output = (*con).nodes_head.tqh_first as *mut Con;
    while !output.is_null() {
        if !con_is_internal(output) {
            let mut content: *mut Con = output_get_content(output);
            if content.is_null() || (*content).focus_head.tqh_first.is_null() {
                debuglog(
                    b"%s:%s:%d - Skipping this output because it is currently being destroyed.\n\0"
                        as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    b"../src/render.c\0" as *const u8 as *const ::core::ffi::c_char,
                    b"render_root\0" as *const u8 as *const ::core::ffi::c_char,
                    293 as ::core::ffi::c_int,
                );
            } else {
                let mut workspace: *mut Con = (*content).focus_head.tqh_first as *mut Con;
                let mut fullscreen_0: *mut Con = con_get_fullscreen_covering_ws(workspace);
                let mut child: *mut Con = ::core::ptr::null_mut::<Con>();
                child = (*workspace).floating_head.tqh_first as *mut Con;
                while !child.is_null() {
                    if !fullscreen_blocks_floating_render(fullscreen_0, child) {
                        debuglog(
                            b"%s:%s:%d - floating child at (%d,%d) with %d x %d\n\0" as *const u8
                                as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b"../src/render.c\0" as *const u8 as *const ::core::ffi::c_char,
                            b"render_root\0" as *const u8 as *const ::core::ffi::c_char,
                            304 as ::core::ffi::c_int,
                            (*child).rect.x,
                            (*child).rect.y,
                            (*child).rect.width,
                            (*child).rect.height,
                        );
                        x_raise_con(child);
                        render_con(child);
                    }
                    child = (*child).floating_windows.tqe_next as *mut Con;
                }
            }
        }
        output = (*output).nodes.tqe_next as *mut Con;
    }
}
unsafe extern "C" fn render_output(mut con: *mut Con) {
    let mut child: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut dockchild: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut x: ::core::ffi::c_int = (*con).rect.x as ::core::ffi::c_int;
    let mut y: ::core::ffi::c_int = (*con).rect.y as ::core::ffi::c_int;
    let mut height: ::core::ffi::c_int = (*con).rect.height as ::core::ffi::c_int;
    let mut content: *mut Con = ::core::ptr::null_mut::<Con>();
    child = (*con).nodes_head.tqh_first as *mut Con;
    while !child.is_null() {
        if (*child).type_0 as ::core::ffi::c_uint
            == CT_CON as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            if !content.is_null() {
                debuglog(
                    b"%s:%s:%d - More than one CT_CON on output container\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    b"../src/render.c\0" as *const u8 as *const ::core::ffi::c_char,
                    b"render_output\0" as *const u8 as *const ::core::ffi::c_char,
                    329 as ::core::ffi::c_int,
                );
                if 0 as ::core::ffi::c_int != 0 {
                } else {
                    __assert_fail(
                        b"false\0" as *const u8 as *const ::core::ffi::c_char,
                        b"../src/render.c\0" as *const u8 as *const ::core::ffi::c_char,
                        330 as ::core::ffi::c_uint,
                        b"void render_output(Con *)\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                };
            }
            content = child;
        } else if (*child).type_0 as ::core::ffi::c_uint
            != CT_DOCKAREA as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            debuglog(
                b"%s:%s:%d - Child %p of type %d is inside the OUTPUT con\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"../src/render.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"render_output\0" as *const u8 as *const ::core::ffi::c_char,
                334 as ::core::ffi::c_int,
                child,
                (*child).type_0 as ::core::ffi::c_uint,
            );
            if 0 as ::core::ffi::c_int != 0 {
            } else {
                __assert_fail(
                    b"false\0" as *const u8 as *const ::core::ffi::c_char,
                    b"../src/render.c\0" as *const u8 as *const ::core::ffi::c_char,
                    335 as ::core::ffi::c_uint,
                    b"void render_output(Con *)\0" as *const u8 as *const ::core::ffi::c_char,
                );
            };
        }
        child = (*child).nodes.tqe_next as *mut Con;
    }
    if content.is_null() {
        debuglog(
            b"%s:%s:%d - Skipping this output because it is currently being destroyed.\n\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/render.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"render_output\0" as *const u8 as *const ::core::ffi::c_char,
            340 as ::core::ffi::c_int,
        );
        return;
    }
    let mut ws: *mut Con = con_get_fullscreen_con(content, CF_OUTPUT);
    if ws.is_null() {
        debuglog(
            b"%s:%s:%d - Skipping this output because it is currently being destroyed.\n\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/render.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"render_output\0" as *const u8 as *const ::core::ffi::c_char,
            349 as ::core::ffi::c_int,
        );
        return;
    }
    let mut fullscreen: *mut Con = con_get_fullscreen_con(ws, CF_OUTPUT);
    if !fullscreen.is_null() {
        (*fullscreen).rect = (*con).rect;
        x_raise_con(fullscreen);
        render_con(fullscreen);
        return;
    }
    child = (*con).nodes_head.tqh_first as *mut Con;
    while !child.is_null() {
        if !((*child).type_0 as ::core::ffi::c_uint
            != CT_DOCKAREA as ::core::ffi::c_int as ::core::ffi::c_uint)
        {
            (*child).rect.height = 0 as uint32_t;
            dockchild = (*child).nodes_head.tqh_first as *mut Con;
            while !dockchild.is_null() {
                (*child).rect.height = (*child)
                    .rect
                    .height
                    .wrapping_add((*dockchild).geometry.height);
                dockchild = (*dockchild).nodes.tqe_next as *mut Con;
            }
            height = (height as uint32_t).wrapping_sub((*child).rect.height) as ::core::ffi::c_int
                as ::core::ffi::c_int;
        }
        child = (*child).nodes.tqe_next as *mut Con;
    }
    child = (*con).nodes_head.tqh_first as *mut Con;
    while !child.is_null() {
        if (*child).type_0 as ::core::ffi::c_uint
            == CT_CON as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            (*child).rect.x = x as uint32_t;
            (*child).rect.y = y as uint32_t;
            (*child).rect.width = (*con).rect.width;
            (*child).rect.height = height as uint32_t;
        }
        (*child).rect.x = x as uint32_t;
        (*child).rect.y = y as uint32_t;
        (*child).rect.width = (*con).rect.width;
        (*child).deco_rect.x = 0 as uint32_t;
        (*child).deco_rect.y = 0 as uint32_t;
        (*child).deco_rect.width = 0 as uint32_t;
        (*child).deco_rect.height = 0 as uint32_t;
        y = (y as uint32_t).wrapping_add((*child).rect.height) as ::core::ffi::c_int
            as ::core::ffi::c_int;
        debuglog(
            b"%s:%s:%d - child at (%d, %d) with (%d x %d)\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/render.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"render_output\0" as *const u8 as *const ::core::ffi::c_char,
            396 as ::core::ffi::c_int,
            (*child).rect.x,
            (*child).rect.y,
            (*child).rect.width,
            (*child).rect.height,
        );
        x_raise_con(child);
        render_con(child);
        child = (*child).nodes.tqe_next as *mut Con;
    }
}
unsafe extern "C" fn render_con_split(
    mut con: *mut Con,
    mut child: *mut Con,
    mut p: *mut render_params,
    mut i: ::core::ffi::c_int,
) {
    if (*con).layout as ::core::ffi::c_uint == L_SPLITH as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*con).layout as ::core::ffi::c_uint
            == L_SPLITV as ::core::ffi::c_int as ::core::ffi::c_uint
    {
    } else {
        __assert_fail(
            b"con->layout == L_SPLITH || con->layout == L_SPLITV\0" as *const u8
                as *const ::core::ffi::c_char,
            b"../src/render.c\0" as *const u8 as *const ::core::ffi::c_char,
            403 as ::core::ffi::c_uint,
            b"void render_con_split(Con *, Con *, render_params *, int)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    };
    if (*con).layout as ::core::ffi::c_uint == L_SPLITH as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*child).rect.x = (*p).x as uint32_t;
        (*child).rect.y = (*p).y as uint32_t;
        (*child).rect.width = *(*p).sizes.offset(i as isize) as uint32_t;
        (*child).rect.height = (*p).rect.height;
        (*p).x = ((*p).x as uint32_t).wrapping_add((*child).rect.width) as ::core::ffi::c_int
            as ::core::ffi::c_int;
    } else {
        (*child).rect.x = (*p).x as uint32_t;
        (*child).rect.y = (*p).y as uint32_t;
        (*child).rect.width = (*p).rect.width;
        (*child).rect.height = *(*p).sizes.offset(i as isize) as uint32_t;
        (*p).y = ((*p).y as uint32_t).wrapping_add((*child).rect.height) as ::core::ffi::c_int
            as ::core::ffi::c_int;
    }
    if con_is_leaf(child) {
        if (*child).border_style as ::core::ffi::c_uint
            == BS_NORMAL as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            (*child).deco_rect.x = 0 as uint32_t;
            (*child).deco_rect.y = 0 as uint32_t;
            (*child).deco_rect.width = (*child).rect.width;
            (*child).deco_rect.height = (*p).deco_height as uint32_t;
        } else {
            (*child).deco_rect.x = 0 as uint32_t;
            (*child).deco_rect.y = 0 as uint32_t;
            (*child).deco_rect.width = 0 as uint32_t;
            (*child).deco_rect.height = 0 as uint32_t;
        }
    }
}
unsafe extern "C" fn render_con_stacked(
    mut con: *mut Con,
    mut child: *mut Con,
    mut p: *mut render_params,
    mut i: ::core::ffi::c_int,
) {
    if (*con).layout as ::core::ffi::c_uint
        == L_STACKED as ::core::ffi::c_int as ::core::ffi::c_uint
    {
    } else {
        __assert_fail(
            b"con->layout == L_STACKED\0" as *const u8 as *const ::core::ffi::c_char,
            b"../src/render.c\0" as *const u8 as *const ::core::ffi::c_char,
            438 as ::core::ffi::c_uint,
            b"void render_con_stacked(Con *, Con *, render_params *, int)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    };
    (*child).rect.x = (*p).x as uint32_t;
    (*child).rect.y = (*p).y as uint32_t;
    (*child).rect.width = (*p).rect.width;
    (*child).rect.height = (*p).rect.height;
    (*child).deco_rect.x = ((*p).x as uint32_t).wrapping_sub((*con).rect.x);
    (*child).deco_rect.y = ((*p).y as uint32_t)
        .wrapping_sub((*con).rect.y)
        .wrapping_add((i * (*p).deco_height) as uint32_t);
    (*child).deco_rect.width = (*child).rect.width;
    (*child).deco_rect.height = (*p).deco_height as uint32_t;
    if (*p).children > 1 as ::core::ffi::c_int
        || (*child).border_style as ::core::ffi::c_uint
            != BS_PIXEL as ::core::ffi::c_int as ::core::ffi::c_uint
            && (*child).border_style as ::core::ffi::c_uint
                != BS_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*child).rect.y = (*child)
            .rect
            .y
            .wrapping_add(((*p).deco_height * (*p).children) as uint32_t);
        (*child).rect.height = (*child)
            .rect
            .height
            .wrapping_sub(((*p).deco_height * (*p).children) as uint32_t);
    }
}
unsafe extern "C" fn render_con_tabbed(
    mut con: *mut Con,
    mut child: *mut Con,
    mut p: *mut render_params,
    mut i: ::core::ffi::c_int,
) {
    if (*con).layout as ::core::ffi::c_uint == L_TABBED as ::core::ffi::c_int as ::core::ffi::c_uint
    {
    } else {
        __assert_fail(
            b"con->layout == L_TABBED\0" as *const u8 as *const ::core::ffi::c_char,
            b"../src/render.c\0" as *const u8 as *const ::core::ffi::c_char,
            457 as ::core::ffi::c_uint,
            b"void render_con_tabbed(Con *, Con *, render_params *, int)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    };
    (*child).rect.x = (*p).x as uint32_t;
    (*child).rect.y = (*p).y as uint32_t;
    (*child).rect.width = (*p).rect.width;
    (*child).rect.height = (*p).rect.height;
    (*child).deco_rect.width = floor(
        ((*child).rect.width as ::core::ffi::c_float / (*p).children as ::core::ffi::c_float)
            as ::core::ffi::c_double,
    ) as uint32_t;
    (*child).deco_rect.x = ((*p).x as uint32_t)
        .wrapping_sub((*con).rect.x)
        .wrapping_add((i as uint32_t).wrapping_mul((*child).deco_rect.width));
    (*child).deco_rect.y = ((*p).y as uint32_t).wrapping_sub((*con).rect.y);
    if i == (*p).children - 1 as ::core::ffi::c_int {
        (*child).deco_rect.width = (*child).rect.width.wrapping_sub((*child).deco_rect.x);
    }
    if (*p).children > 1 as ::core::ffi::c_int
        || (*child).border_style as ::core::ffi::c_uint
            != BS_PIXEL as ::core::ffi::c_int as ::core::ffi::c_uint
            && (*child).border_style as ::core::ffi::c_uint
                != BS_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*child).rect.y = (*child).rect.y.wrapping_add((*p).deco_height as uint32_t);
        (*child).rect.height = (*child)
            .rect
            .height
            .wrapping_sub((*p).deco_height as uint32_t);
        (*child).deco_rect.height = (*p).deco_height as uint32_t;
    } else {
        (*child).deco_rect.height = (if (*child).border_style as ::core::ffi::c_uint
            == BS_PIXEL as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t;
    };
}
unsafe extern "C" fn render_con_dockarea(
    mut con: *mut Con,
    mut child: *mut Con,
    mut p: *mut render_params,
) {
    if (*con).layout as ::core::ffi::c_uint
        == L_DOCKAREA as ::core::ffi::c_int as ::core::ffi::c_uint
    {
    } else {
        __assert_fail(
            b"con->layout == L_DOCKAREA\0" as *const u8 as *const ::core::ffi::c_char,
            b"../src/render.c\0" as *const u8 as *const ::core::ffi::c_char,
            484 as ::core::ffi::c_uint,
            b"void render_con_dockarea(Con *, Con *, render_params *)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    };
    (*child).rect.x = (*p).x as uint32_t;
    (*child).rect.y = (*p).y as uint32_t;
    (*child).rect.width = (*p).rect.width;
    (*child).rect.height = (*child).geometry.height;
    (*child).deco_rect.x = 0 as uint32_t;
    (*child).deco_rect.y = 0 as uint32_t;
    (*child).deco_rect.width = 0 as uint32_t;
    (*child).deco_rect.height = 0 as uint32_t;
    (*p).y = ((*p).y as uint32_t).wrapping_add((*child).rect.height) as ::core::ffi::c_int
        as ::core::ffi::c_int;
}
