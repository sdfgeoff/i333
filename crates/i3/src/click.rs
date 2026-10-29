/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * click.c: Button press (mouse click) events.
 *
 */
// Mechanical file-by-file Rust translation of src/click.c.
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
    fn xcb_allow_events(
        c: *mut xcb_connection_t,
        mode: uint8_t,
        time: xcb_timestamp_t,
    ) -> xcb_void_cookie_t;
    fn xcb_flush(c: *mut xcb_connection_t) -> ::core::ffi::c_int;
    static mut conn: *mut xcb_connection_t;
    fn errorlog(fmt: *mut ::core::ffi::c_char, ...);
    fn debuglog(fmt: *mut ::core::ffi::c_char, ...);
    fn rect_contains(rect: Rect, x: uint32_t, y: uint32_t) -> bool;
    static mut croot: *mut Con;
    static mut focused: *mut Con;
    fn tree_render();
    fn get_tree_next_sibling(con: *mut Con, direction: position_t) -> *mut Con;
    static mut last_timestamp: xcb_timestamp_t;
    static mut root: xcb_window_t;
    fn has_drop_targets() -> bool;
    fn tiling_drag(con: *mut Con, event: *mut xcb_button_press_event_t, use_threshold: bool);
    static mut config: Config;
    fn workspace_show(ws: *mut Con);
    fn floating_drag_window(
        con: *mut Con,
        event: *const xcb_button_press_event_t,
        use_threshold: bool,
    );
    fn floating_resize_window(
        con: *mut Con,
        proportional: bool,
        event: *const xcb_button_press_event_t,
    );
    fn con_activate(con: *mut Con);
    fn con_get_output(con: *mut Con) -> *mut Con;
    fn con_get_workspace(con: *mut Con) -> *mut Con;
    fn con_get_fullscreen_covering_ws(ws: *mut Con) -> *mut Con;
    fn con_is_internal(con: *mut Con) -> bool;
    fn con_inside_floating(con: *mut Con) -> *mut Con;
    fn con_has_parent(con: *mut Con, parent: *mut Con) -> bool;
    fn con_by_window_id(window: xcb_window_t) -> *mut Con;
    fn con_by_frame_id(frame: xcb_window_t) -> *mut Con;
    fn con_descend_focused(con: *mut Con) -> *mut Con;
    fn con_border_style_rect(con: *mut Con) -> Rect;
    fn resize_find_tiling_participants(
        current: *mut *mut Con,
        other: *mut *mut Con,
        direction: direction_t,
        both_sides: bool,
    ) -> bool;
    fn resize_graphical_handler(
        first: *mut Con,
        second: *mut Con,
        orientation: orientation_t,
        event: *const xcb_button_press_event_t,
        use_threshold: bool,
    );
    fn output_get_content(output: *mut Con) -> *mut Con;
    fn command_result_free(result: *mut CommandResult);
    fn get_binding_from_xcb_event(event: *mut xcb_generic_event_t) -> *mut Binding;
    fn run_binding(bind: *mut Binding, con: *mut Con) -> *mut CommandResult;
}
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
pub type xcb_font_t = uint32_t;
pub type xcb_gcontext_t = uint32_t;
pub type xcb_colormap_t = uint32_t;
pub type xcb_atom_t = uint32_t;
pub type xcb_drawable_t = uint32_t;
pub type xcb_timestamp_t = uint32_t;
pub type xcb_keycode_t = uint8_t;
pub type xcb_button_t = uint8_t;
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
pub struct xcb_button_press_event_t {
    pub response_type: uint8_t,
    pub detail: xcb_button_t,
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
pub type xcb_button_index_t = ::core::ffi::c_uint;
pub const XCB_BUTTON_INDEX_5: xcb_button_index_t = 5;
pub const XCB_BUTTON_INDEX_4: xcb_button_index_t = 4;
pub const XCB_BUTTON_INDEX_3: xcb_button_index_t = 3;
pub const XCB_BUTTON_INDEX_2: xcb_button_index_t = 2;
pub const XCB_BUTTON_INDEX_1: xcb_button_index_t = 1;
pub const XCB_BUTTON_INDEX_ANY: xcb_button_index_t = 0;
pub type xcb_allow_t = ::core::ffi::c_uint;
pub const XCB_ALLOW_SYNC_BOTH: xcb_allow_t = 7;
pub const XCB_ALLOW_ASYNC_BOTH: xcb_allow_t = 6;
pub const XCB_ALLOW_REPLAY_KEYBOARD: xcb_allow_t = 5;
pub const XCB_ALLOW_SYNC_KEYBOARD: xcb_allow_t = 4;
pub const XCB_ALLOW_ASYNC_KEYBOARD: xcb_allow_t = 3;
pub const XCB_ALLOW_REPLAY_POINTER: xcb_allow_t = 2;
pub const XCB_ALLOW_SYNC_POINTER: xcb_allow_t = 1;
pub const XCB_ALLOW_ASYNC_POINTER: xcb_allow_t = 0;
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
pub type click_destination_t = ::core::ffi::c_uint;
pub const CLICK_INSIDE: click_destination_t = 2;
pub const CLICK_DECORATION: click_destination_t = 1;
pub const CLICK_BORDER: click_destination_t = 0;
pub type border_t = ::core::ffi::c_uint;
pub const BORDER_BOTTOM: border_t = 8;
pub const BORDER_TOP: border_t = 4;
pub const BORDER_RIGHT: border_t = 2;
pub const BORDER_LEFT: border_t = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CommandResult {
    pub parse_error: bool,
    pub error_message: *mut ::core::ffi::c_char,
    pub needs_tree_render: bool,
}
pub const __ASSERT_FUNCTION: [::core::ffi::c_char; 83] = unsafe {
    ::core::mem::transmute::<[u8; 83], [::core::ffi::c_char; 83]>(
        *b"_Bool tiling_resize_for_border(Con *, border_t, xcb_button_press_event_t *, _Bool)\0",
    )
};
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const XCB_BUTTON_PRESS: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const XCB_BUTTON_RELEASE: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const XCB_NONE: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const XCB_BUTTON_SCROLL_LEFT: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const XCB_BUTTON_SCROLL_RIGHT: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
unsafe extern "C" fn tiling_resize_for_border(
    mut con: *mut Con,
    mut border: border_t,
    mut event: *mut xcb_button_press_event_t,
    mut use_threshold: bool,
) -> bool {
    debuglog(
        b"%s:%s:%d - border = %d, con = %p\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/click.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"tiling_resize_for_border\0" as *const u8 as *const ::core::ffi::c_char,
        25 as ::core::ffi::c_int,
        border as ::core::ffi::c_uint,
        con,
    );
    let mut second: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut first: *mut Con = con;
    let mut search_direction: direction_t = D_LEFT;
    match border as ::core::ffi::c_uint {
        1 => {
            search_direction = D_LEFT;
        }
        2 => {
            search_direction = D_RIGHT;
        }
        4 => {
            search_direction = D_UP;
        }
        8 => {
            search_direction = D_DOWN;
        }
        _ => {
            errorlog(
                b"ERROR: BUG: invalid border value %d\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                border as ::core::ffi::c_uint,
            );
            return false_0 != 0;
        }
    }
    let mut res: bool = resize_find_tiling_participants(
        &raw mut first,
        &raw mut second,
        search_direction,
        false_0 != 0,
    );
    if !res {
        debuglog(
            b"%s:%s:%d - No second container in this direction found.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/click.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"tiling_resize_for_border\0" as *const u8 as *const ::core::ffi::c_char,
            49 as ::core::ffi::c_int,
        );
        return false_0 != 0;
    }
    if (*first).fullscreen_mode as ::core::ffi::c_uint
        != (*second).fullscreen_mode as ::core::ffi::c_uint
    {
        debuglog(
            b"%s:%s:%d - Avoiding resize between containers with different fullscreen modes, %d != %d\n\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/click.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"tiling_resize_for_border\0" as *const u8 as *const ::core::ffi::c_char,
            53 as ::core::ffi::c_int,
            (*first).fullscreen_mode as ::core::ffi::c_uint,
            (*second).fullscreen_mode as ::core::ffi::c_uint,
        );
        return false_0 != 0;
    }
    if first != second {
    } else {
        __assert_fail(
            b"first != second\0" as *const u8 as *const ::core::ffi::c_char,
            b"../src/click.c\0" as *const u8 as *const ::core::ffi::c_char,
            57 as ::core::ffi::c_uint,
            __ASSERT_FUNCTION.as_ptr(),
        );
    };
    if (*first).parent == (*second).parent {
    } else {
        __assert_fail(
            b"first->parent == second->parent\0" as *const u8 as *const ::core::ffi::c_char,
            b"../src/click.c\0" as *const u8 as *const ::core::ffi::c_char,
            58 as ::core::ffi::c_uint,
            __ASSERT_FUNCTION.as_ptr(),
        );
    };
    if search_direction as ::core::ffi::c_uint == D_UP as ::core::ffi::c_int as ::core::ffi::c_uint
        || search_direction as ::core::ffi::c_uint
            == D_LEFT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut tmp: *mut Con = first;
        first = second;
        second = tmp;
    }
    let orientation: orientation_t = (if border as ::core::ffi::c_uint
        == BORDER_LEFT as ::core::ffi::c_int as ::core::ffi::c_uint
        || border as ::core::ffi::c_uint
            == BORDER_RIGHT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        HORIZ as ::core::ffi::c_int
    } else {
        VERT as ::core::ffi::c_int
    }) as orientation_t;
    resize_graphical_handler(first, second, orientation, event, use_threshold);
    debuglog(
        b"%s:%s:%d - After resize handler, rendering\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/click.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"tiling_resize_for_border\0" as *const u8 as *const ::core::ffi::c_char,
        71 as ::core::ffi::c_int,
    );
    tree_render();
    return true_0 != 0;
}
unsafe extern "C" fn floating_mod_on_tiled_client(
    mut con: *mut Con,
    mut event: *mut xcb_button_press_event_t,
) -> bool {
    let mut to_right: ::core::ffi::c_int =
        (*con).rect.width.wrapping_sub((*event).event_x as uint32_t) as ::core::ffi::c_int;
    let mut to_left: ::core::ffi::c_int = (*event).event_x as ::core::ffi::c_int;
    let mut to_top: ::core::ffi::c_int = (*event).event_y as ::core::ffi::c_int;
    let mut to_bottom: ::core::ffi::c_int = (*con)
        .rect
        .height
        .wrapping_sub((*event).event_y as uint32_t)
        as ::core::ffi::c_int;
    debuglog(
        b"%s:%s:%d - click was %d px to the right, %d px to the left, %d px to top, %d px to bottom\n\0"
            as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/click.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"floating_mod_on_tiled_client\0" as *const u8 as *const ::core::ffi::c_char,
        94 as ::core::ffi::c_int,
        to_right,
        to_left,
        to_top,
        to_bottom,
    );
    if to_right < to_left && to_right < to_top && to_right < to_bottom {
        return tiling_resize_for_border(con, BORDER_RIGHT, event, false_0 != 0);
    }
    if to_left < to_right && to_left < to_top && to_left < to_bottom {
        return tiling_resize_for_border(con, BORDER_LEFT, event, false_0 != 0);
    }
    if to_top < to_right && to_top < to_left && to_top < to_bottom {
        return tiling_resize_for_border(con, BORDER_TOP, event, false_0 != 0);
    }
    if to_bottom < to_right && to_bottom < to_left && to_bottom < to_top {
        return tiling_resize_for_border(con, BORDER_BOTTOM, event, false_0 != 0);
    }
    return false_0 != 0;
}
unsafe extern "C" fn tiling_resize(
    mut con: *mut Con,
    mut event: *mut xcb_button_press_event_t,
    dest: click_destination_t,
    mut use_threshold: bool,
) -> bool {
    let mut bsr: Rect = con_border_style_rect(con);
    debuglog(
        b"%s:%s:%d - BORDER x = %d, y = %d for con %p, window 0x%08x\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/click.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"tiling_resize\0" as *const u8 as *const ::core::ffi::c_char,
        131 as ::core::ffi::c_int,
        (*event).event_x as ::core::ffi::c_int,
        (*event).event_y as ::core::ffi::c_int,
        con,
        (*event).event,
    );
    debuglog(
        b"%s:%s:%d - checks for right >= %d\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/click.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"tiling_resize\0" as *const u8 as *const ::core::ffi::c_char,
        132 as ::core::ffi::c_int,
        (*con).window_rect.x.wrapping_add((*con).window_rect.width),
    );
    if dest as ::core::ffi::c_uint == CLICK_DECORATION as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return tiling_resize_for_border(con, BORDER_TOP, event, use_threshold);
    } else if dest as ::core::ffi::c_uint
        == CLICK_BORDER as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if (*event).event_y as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
            && (*event).event_y as int32_t <= bsr.y as int32_t
            && (*event).event_x as int32_t >= bsr.x as int32_t
            && (*event).event_x as int32_t <= (*con).rect.width.wrapping_add(bsr.width) as int32_t
        {
            return tiling_resize_for_border(con, BORDER_TOP, event, false_0 != 0);
        }
    }
    if (*event).event_x as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
        && (*event).event_x as int32_t <= bsr.x as int32_t
        && (*event).event_y as int32_t >= bsr.y as int32_t
        && (*event).event_y as int32_t <= (*con).rect.height.wrapping_add(bsr.height) as int32_t
    {
        return tiling_resize_for_border(con, BORDER_LEFT, event, false_0 != 0);
    }
    if (*event).event_x as int32_t
        >= (*con).window_rect.x.wrapping_add((*con).window_rect.width) as int32_t
        && (*event).event_y as int32_t >= bsr.y as int32_t
        && (*event).event_y as int32_t <= (*con).rect.height.wrapping_add(bsr.height) as int32_t
    {
        return tiling_resize_for_border(con, BORDER_RIGHT, event, false_0 != 0);
    }
    if (*event).event_y as int32_t
        >= (*con).window_rect.y.wrapping_add((*con).window_rect.height) as int32_t
    {
        return tiling_resize_for_border(con, BORDER_BOTTOM, event, false_0 != 0);
    }
    return false_0 != 0;
}
unsafe extern "C" fn allow_replay_pointer(mut time: xcb_timestamp_t) {
    xcb_allow_events(
        conn,
        XCB_ALLOW_REPLAY_POINTER as ::core::ffi::c_int as uint8_t,
        time,
    );
    xcb_flush(conn);
    tree_render();
}
unsafe extern "C" fn route_click(
    mut con: *mut Con,
    mut event: *mut xcb_button_press_event_t,
    dest: click_destination_t,
) {
    let mod_0: uint32_t = config.floating_modifier & 0xffff as uint32_t;
    let mod_pressed: bool = mod_0 != 0 as uint32_t && (*event).state as uint32_t & mod_0 == mod_0;
    debuglog(
        b"%s:%s:%d - --> click properties: mod = %d, destination = %d\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/click.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"route_click\0" as *const u8 as *const ::core::ffi::c_char,
        173 as ::core::ffi::c_int,
        mod_pressed as ::core::ffi::c_int,
        dest as ::core::ffi::c_uint,
    );
    debuglog(
        b"%s:%s:%d - --> OUTCOME = %p\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/click.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"route_click\0" as *const u8 as *const ::core::ffi::c_char,
        174 as ::core::ffi::c_int,
        con,
    );
    debuglog(
        b"%s:%s:%d - type = %d, name = %s\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/click.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"route_click\0" as *const u8 as *const ::core::ffi::c_char,
        175 as ::core::ffi::c_int,
        (*con).type_0 as ::core::ffi::c_uint,
        (*con).name,
    );
    if (*(*con).parent).type_0 as ::core::ffi::c_uint
        == CT_DOCKAREA as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        allow_replay_pointer((*event).time);
        return;
    }
    let mut bind: *mut Binding = get_binding_from_xcb_event(event as *mut xcb_generic_event_t);
    if !bind.is_null()
        && (dest as ::core::ffi::c_uint
            == CLICK_DECORATION as ::core::ffi::c_int as ::core::ffi::c_uint
            && !(*bind).exclude_titlebar
            || dest as ::core::ffi::c_uint
                == CLICK_INSIDE as ::core::ffi::c_int as ::core::ffi::c_uint
                && (*bind).whole_window as ::core::ffi::c_int != 0
            || dest as ::core::ffi::c_uint
                == CLICK_BORDER as ::core::ffi::c_int as ::core::ffi::c_uint
                && (*bind).border as ::core::ffi::c_int != 0)
    {
        let mut result: *mut CommandResult = run_binding(bind, con);
        xcb_allow_events(
            conn,
            XCB_ALLOW_ASYNC_POINTER as ::core::ffi::c_int as uint8_t,
            (*event).time,
        );
        xcb_flush(conn);
        command_result_free(result);
        return;
    }
    if (*event).response_type as ::core::ffi::c_int == XCB_BUTTON_RELEASE {
        allow_replay_pointer((*event).time);
        return;
    }
    let mut ws: *mut Con = con_get_workspace(con);
    let mut focused_workspace: *mut Con = con_get_workspace(focused);
    if ws.is_null() {
        ws = (*output_get_content(con_get_output(con)))
            .focus_head
            .tqh_first as *mut Con;
        if ws.is_null() {
            allow_replay_pointer((*event).time);
            return;
        }
    }
    let mut floatingcon: *mut Con = con_inside_floating(con);
    let proportional: bool = (*event).state as ::core::ffi::c_int
        & XCB_KEY_BUT_MASK_SHIFT as ::core::ffi::c_int
        == XCB_KEY_BUT_MASK_SHIFT as ::core::ffi::c_int;
    let in_stacked: bool = (*(*con).parent).layout as ::core::ffi::c_uint
        == L_STACKED as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*(*con).parent).layout as ::core::ffi::c_uint
            == L_TABBED as ::core::ffi::c_int as ::core::ffi::c_uint;
    let was_focused: bool = focused == con;
    let is_left_click: bool =
        (*event).detail as ::core::ffi::c_int == XCB_BUTTON_INDEX_1 as ::core::ffi::c_int;
    let is_right_click: bool =
        (*event).detail as ::core::ffi::c_int == XCB_BUTTON_INDEX_3 as ::core::ffi::c_int;
    let is_left_or_right_click: bool =
        is_left_click as ::core::ffi::c_int != 0 || is_right_click as ::core::ffi::c_int != 0;
    let is_scroll: bool = (*event).detail as ::core::ffi::c_int
        == XCB_BUTTON_INDEX_4 as ::core::ffi::c_int
        || (*event).detail as ::core::ffi::c_int == XCB_BUTTON_INDEX_5 as ::core::ffi::c_int
        || (*event).detail as ::core::ffi::c_int == XCB_BUTTON_SCROLL_LEFT
        || (*event).detail as ::core::ffi::c_int == XCB_BUTTON_SCROLL_RIGHT;
    if in_stacked as ::core::ffi::c_int != 0
        && dest as ::core::ffi::c_uint
            == CLICK_DECORATION as ::core::ffi::c_int as ::core::ffi::c_uint
        && is_scroll as ::core::ffi::c_int != 0
    {
        debuglog(
            b"%s:%s:%d - Scrolling on a window decoration\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/click.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"route_click\0" as *const u8 as *const ::core::ffi::c_char,
            234 as ::core::ffi::c_int,
        );
        workspace_show(ws);
        let mut current: *mut Con = (*(*con).parent).focus_head.tqh_first as *mut Con;
        let direction: position_t = (if (*event).detail as ::core::ffi::c_int
            == XCB_BUTTON_INDEX_4 as ::core::ffi::c_int
            || (*event).detail as ::core::ffi::c_int == XCB_BUTTON_SCROLL_LEFT
        {
            BEFORE as ::core::ffi::c_int
        } else {
            AFTER as ::core::ffi::c_int
        }) as position_t;
        let mut next: *mut Con = get_tree_next_sibling(current, direction);
        con_activate(con_descend_focused(if !next.is_null() {
            next
        } else {
            current
        }));
        allow_replay_pointer((*event).time);
        return;
    }
    if mod_pressed as ::core::ffi::c_int != 0
        && is_left_click as ::core::ffi::c_int != 0
        && floatingcon.is_null()
        && (config.tiling_drag as ::core::ffi::c_uint
            == TILING_DRAG_MODIFIER as ::core::ffi::c_int as ::core::ffi::c_uint
            || config.tiling_drag as ::core::ffi::c_uint
                == TILING_DRAG_MODIFIER_OR_TITLEBAR as ::core::ffi::c_int as ::core::ffi::c_uint)
        && has_drop_targets() as ::core::ffi::c_int != 0
    {
        let use_threshold: bool = !mod_pressed;
        tiling_drag(con, event, use_threshold);
        allow_replay_pointer((*event).time);
        return;
    }
    let mut con_to_focus: *mut Con = con;
    if in_stacked as ::core::ffi::c_int != 0
        && dest as ::core::ffi::c_uint
            == CLICK_DECORATION as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if was_focused as ::core::ffi::c_int != 0 || !con_has_parent(focused, con) {
            while !(*con_to_focus).focus_head.tqh_first.is_null() {
                con_to_focus = (*con_to_focus).focus_head.tqh_first as *mut Con;
            }
        }
    }
    if ws != focused_workspace {
        workspace_show(ws);
    }
    con_activate(con_to_focus);
    let mut fs: *mut Con = con_get_fullscreen_covering_ws(ws);
    if !floatingcon.is_null() && fs != con {
        if mod_pressed as ::core::ffi::c_int != 0 && is_left_click as ::core::ffi::c_int != 0 {
            floating_drag_window(floatingcon, event, false_0 != 0);
            return;
        }
        if mod_pressed as ::core::ffi::c_int != 0 && is_right_click as ::core::ffi::c_int != 0 {
            debuglog(
                b"%s:%s:%d - floating resize due to floatingmodifier\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"../src/click.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"route_click\0" as *const u8 as *const ::core::ffi::c_char,
                293 as ::core::ffi::c_int,
            );
            floating_resize_window(floatingcon, proportional, event);
            return;
        }
        if (dest as ::core::ffi::c_uint
            == CLICK_BORDER as ::core::ffi::c_int as ::core::ffi::c_uint
            || dest as ::core::ffi::c_uint
                == CLICK_DECORATION as ::core::ffi::c_int as ::core::ffi::c_uint)
            && is_left_or_right_click as ::core::ffi::c_int != 0
        {
            debuglog(
                b"%s:%s:%d - tiling resize with fallback\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"../src/click.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"route_click\0" as *const u8 as *const ::core::ffi::c_char,
                301 as ::core::ffi::c_int,
            );
            if tiling_resize(
                con,
                event,
                dest,
                dest as ::core::ffi::c_uint
                    == CLICK_DECORATION as ::core::ffi::c_int as ::core::ffi::c_uint
                    && !was_focused,
            ) {
                allow_replay_pointer((*event).time);
                return;
            }
        }
        if dest as ::core::ffi::c_uint
            == CLICK_DECORATION as ::core::ffi::c_int as ::core::ffi::c_uint
            && is_right_click as ::core::ffi::c_int != 0
        {
            debuglog(
                b"%s:%s:%d - floating resize due to decoration right click\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"../src/click.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"route_click\0" as *const u8 as *const ::core::ffi::c_char,
                309 as ::core::ffi::c_int,
            );
            floating_resize_window(floatingcon, proportional, event);
            return;
        }
        if dest as ::core::ffi::c_uint == CLICK_BORDER as ::core::ffi::c_int as ::core::ffi::c_uint
            && is_left_or_right_click as ::core::ffi::c_int != 0
        {
            debuglog(
                b"%s:%s:%d - floating resize due to border click\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"../src/click.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"route_click\0" as *const u8 as *const ::core::ffi::c_char,
                315 as ::core::ffi::c_int,
            );
            floating_resize_window(floatingcon, proportional, event);
            return;
        }
        if dest as ::core::ffi::c_uint
            == CLICK_DECORATION as ::core::ffi::c_int as ::core::ffi::c_uint
            && is_left_click as ::core::ffi::c_int != 0
        {
            floating_drag_window(floatingcon, event, !was_focused);
            return;
        }
        allow_replay_pointer((*event).time);
        return;
    }
    if is_left_click as ::core::ffi::c_int != 0
        && (config.tiling_drag as ::core::ffi::c_uint
            == TILING_DRAG_TITLEBAR as ::core::ffi::c_int as ::core::ffi::c_uint
            && dest as ::core::ffi::c_uint
                == CLICK_DECORATION as ::core::ffi::c_int as ::core::ffi::c_uint
            || config.tiling_drag as ::core::ffi::c_uint
                == TILING_DRAG_MODIFIER_OR_TITLEBAR as ::core::ffi::c_int as ::core::ffi::c_uint
                && (mod_pressed as ::core::ffi::c_int != 0
                    || dest as ::core::ffi::c_uint
                        == CLICK_DECORATION as ::core::ffi::c_int as ::core::ffi::c_uint))
        && has_drop_targets() as ::core::ffi::c_int != 0
    {
        allow_replay_pointer((*event).time);
        let use_threshold_0: bool = !mod_pressed;
        tiling_drag(con, event, use_threshold_0);
        return;
    }
    if dest as ::core::ffi::c_uint == CLICK_INSIDE as ::core::ffi::c_int as ::core::ffi::c_uint
        && mod_pressed as ::core::ffi::c_int != 0
        && is_right_click as ::core::ffi::c_int != 0
    {
        if floating_mod_on_tiled_client(con, event) {
            return;
        }
        xcb_allow_events(
            conn,
            XCB_ALLOW_ASYNC_POINTER as ::core::ffi::c_int as uint8_t,
            (*event).time,
        );
        xcb_flush(conn);
        return;
    }
    if (dest as ::core::ffi::c_uint == CLICK_BORDER as ::core::ffi::c_int as ::core::ffi::c_uint
        || dest as ::core::ffi::c_uint
            == CLICK_DECORATION as ::core::ffi::c_int as ::core::ffi::c_uint)
        && is_left_or_right_click as ::core::ffi::c_int != 0
    {
        debuglog(
            b"%s:%s:%d - Trying to resize (tiling)\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b"../src/click.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"route_click\0" as *const u8 as *const ::core::ffi::c_char,
            357 as ::core::ffi::c_int,
        );
        tiling_resize(
            con,
            event,
            dest,
            dest as ::core::ffi::c_uint
                == CLICK_DECORATION as ::core::ffi::c_int as ::core::ffi::c_uint
                && !was_focused,
        );
    }
    allow_replay_pointer((*event).time);
}
#[no_mangle]
pub unsafe extern "C" fn handle_button_press(mut event: *mut xcb_button_press_event_t) {
    let mut con: *mut Con = ::core::ptr::null_mut::<Con>();
    debuglog(
        b"%s:%s:%d - Button %d (state %d) %s on window 0x%08x (child 0x%08x) at (%d, %d) (root %d, %d)\n\0"
            as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/click.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"handle_button_press\0" as *const u8 as *const ::core::ffi::c_char,
        377 as ::core::ffi::c_int,
        (*event).detail as ::core::ffi::c_int,
        (*event).state as ::core::ffi::c_int,
        if (*event).response_type as ::core::ffi::c_int == XCB_BUTTON_PRESS {
            b"press\0" as *const u8 as *const ::core::ffi::c_char
        } else {
            b"release\0" as *const u8 as *const ::core::ffi::c_char
        },
        (*event).event,
        (*event).child,
        (*event).event_x as ::core::ffi::c_int,
        (*event).event_y as ::core::ffi::c_int,
        (*event).root_x as ::core::ffi::c_int,
        (*event).root_y as ::core::ffi::c_int,
    );
    last_timestamp = (*event).time;
    con = con_by_window_id((*event).event);
    if !con.is_null() {
        route_click(con, event, CLICK_INSIDE);
        return;
    }
    con = con_by_frame_id((*event).event);
    if con.is_null() {
        if (*event).event == root {
            let mut bind: *mut Binding =
                get_binding_from_xcb_event(event as *mut xcb_generic_event_t);
            if !bind.is_null() && (*bind).whole_window as ::core::ffi::c_int != 0 {
                let mut result: *mut CommandResult =
                    run_binding(bind, ::core::ptr::null_mut::<Con>());
                command_result_free(result);
            }
        }
        if (*event).event == root
            && (*event).response_type as ::core::ffi::c_int == XCB_BUTTON_PRESS
        {
            let mut output: *mut Con = ::core::ptr::null_mut::<Con>();
            output = (*croot).nodes_head.tqh_first as *mut Con;
            while !output.is_null() {
                if con_is_internal(output) as ::core::ffi::c_int != 0
                    || !rect_contains(
                        (*output).rect as Rect,
                        (*event).event_x as uint32_t,
                        (*event).event_y as uint32_t,
                    )
                {
                    output = (*output).nodes.tqe_next as *mut Con;
                } else {
                    let mut ws: *mut Con =
                        (*output_get_content(output)).focus_head.tqh_first as *mut Con;
                    if ws != con_get_workspace(focused) {
                        workspace_show(ws);
                        tree_render();
                    }
                    return;
                }
            }
            return;
        }
        errorlog(
            b"ERROR: Clicked into unknown window?!\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        xcb_allow_events(
            conn,
            XCB_ALLOW_REPLAY_POINTER as ::core::ffi::c_int as uint8_t,
            (*event).time,
        );
        xcb_flush(conn);
        return;
    }
    if !(*con).window.is_null() {
        if rect_contains(
            (*con).deco_rect as Rect,
            (*event).event_x as uint32_t,
            (*event).event_y as uint32_t,
        ) {
            route_click(con, event, CLICK_DECORATION);
            return;
        }
    } else {
        let mut child: *mut Con = ::core::ptr::null_mut::<Con>();
        child = *(*((*con).nodes_head.tqh_last as *mut nodes_head)).tqh_last as *mut Con;
        while !child.is_null() {
            if !rect_contains(
                (*child).deco_rect as Rect,
                (*event).event_x as uint32_t,
                (*event).event_y as uint32_t,
            ) {
                child = *(*((*child).nodes.tqe_prev as *mut nodes_head)).tqh_last as *mut Con;
            } else {
                route_click(child, event, CLICK_DECORATION);
                return;
            }
        }
    }
    if (*event).child as ::core::ffi::c_long != XCB_NONE {
        debuglog(
            b"%s:%s:%d - event->child not XCB_NONE, so this is an event which originated from a click into the application, but the application did not handle it.\n\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/click.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"handle_button_press\0" as *const u8 as *const ::core::ffi::c_char,
            443 as ::core::ffi::c_int,
        );
        route_click(con, event, CLICK_INSIDE);
        return;
    }
    route_click(con, event, CLICK_BORDER);
}
