/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * floating.c: Floating windows.
 *
 */
// Mechanical file-by-file Rust translation of src/floating.c.
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
    fn xcb_delete_property(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        property: xcb_atom_t,
    ) -> xcb_void_cookie_t;
    fn xcb_query_pointer(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_query_pointer_cookie_t;
    fn xcb_query_pointer_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_query_pointer_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_query_pointer_reply_t;
    fn xcb_flush(c: *mut xcb_connection_t) -> ::core::ffi::c_int;
    static mut conn: *mut xcb_connection_t;
    static mut root_screen: *mut xcb_screen_t;
    fn verboselog(fmt: *mut ::core::ffi::c_char, ...);
    fn errorlog(fmt: *mut ::core::ffi::c_char, ...);
    fn debuglog(fmt: *mut ::core::ffi::c_char, ...);
    fn sasprintf(
        strp: *mut *mut ::core::ffi::c_char,
        fmt: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn min(a: ::core::ffi::c_int, b: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn max(a: ::core::ffi::c_int, b: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn rect_equals(a: Rect, b: Rect) -> bool;
    static mut croot: *mut Con;
    static mut focused: *mut Con;
    fn tree_render();
    fn tree_close_internal(
        con: *mut Con,
        kill_window: kill_window_t,
        dont_kill_parent: bool,
    ) -> bool;
    static mut root: xcb_window_t;
    static mut config: Config;
    fn ipc_send_window_event(property: *const ::core::ffi::c_char, con: *mut Con);
    static mut A_I3_FLOATING_WINDOW: xcb_atom_t;
    static mut outputs: outputs_head;
    fn get_output_containing(x: ::core::ffi::c_uint, y: ::core::ffi::c_uint) -> *mut Output;
    fn get_output_from_rect(rect: Rect) -> *mut Output;
    fn output_containing_rect(rect: Rect) -> *mut Output;
    fn workspace_show(ws: *mut Con);
    fn x_push_node(con: *mut Con);
    fn x_push_changes(con: *mut Con);
    fn x_set_name(con: *mut Con, name: *const ::core::ffi::c_char);
    fn x_set_warp_to(rect: *mut Rect);
    fn drag_pointer(
        con: *mut Con,
        event: *const xcb_button_press_event_t,
        confine_to: xcb_window_t,
        cursor: ::core::ffi::c_int,
        use_threshold: bool,
        callback: callback_t,
        extra: *const ::core::ffi::c_void,
    ) -> drag_result_t;
    fn con_new(parent: *mut Con, window: *mut i3Window) -> *mut Con;
    fn con_activate(con: *mut Con);
    fn con_is_leaf(con: *mut Con) -> bool;
    fn con_get_output(con: *mut Con) -> *mut Con;
    fn con_get_workspace(con: *mut Con) -> *mut Con;
    fn con_is_internal(con: *mut Con) -> bool;
    fn con_is_floating(con: *mut Con) -> bool;
    fn con_is_docked(con: *mut Con) -> bool;
    fn con_inside_focused(con: *mut Con) -> bool;
    fn con_by_window_id(window: xcb_window_t) -> *mut Con;
    fn con_exists(con: *mut Con) -> bool;
    fn con_num_children(con: *mut Con) -> ::core::ffi::c_int;
    fn con_attach(con: *mut Con, parent: *mut Con, ignore_focus: bool);
    fn con_detach(con: *mut Con);
    fn con_fix_percent(con: *mut Con);
    fn con_move_to_workspace(
        con: *mut Con,
        workspace: *mut Con,
        fix_coordinates: bool,
        dont_warp: bool,
        ignore_focus: bool,
    );
    fn con_descend_focused(con: *mut Con) -> *mut Con;
    fn con_descend_tiling_focused(con: *mut Con) -> *mut Con;
    fn con_border_style_rect(con: *mut Con) -> Rect;
    fn render_con(con: *mut Con);
    fn insert_con_into(con: *mut Con, target: *mut Con, position: position_t);
    fn output_get_content(output: *mut Con) -> *mut Con;
    fn round(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn free(__ptr: *mut ::core::ffi::c_void);
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
pub type xcb_visualid_t = uint32_t;
pub type xcb_timestamp_t = uint32_t;
pub type xcb_button_t = uint8_t;
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
pub type xcb_atom_enum_t = ::core::ffi::c_uint;
pub const XCB_ATOM_WM_TRANSIENT_FOR: xcb_atom_enum_t = 68;
pub const XCB_ATOM_WM_CLASS: xcb_atom_enum_t = 67;
pub const XCB_ATOM_CAP_HEIGHT: xcb_atom_enum_t = 66;
pub const XCB_ATOM_FULL_NAME: xcb_atom_enum_t = 65;
pub const XCB_ATOM_FAMILY_NAME: xcb_atom_enum_t = 64;
pub const XCB_ATOM_FONT_NAME: xcb_atom_enum_t = 63;
pub const XCB_ATOM_NOTICE: xcb_atom_enum_t = 62;
pub const XCB_ATOM_COPYRIGHT: xcb_atom_enum_t = 61;
pub const XCB_ATOM_RESOLUTION: xcb_atom_enum_t = 60;
pub const XCB_ATOM_POINT_SIZE: xcb_atom_enum_t = 59;
pub const XCB_ATOM_WEIGHT: xcb_atom_enum_t = 58;
pub const XCB_ATOM_QUAD_WIDTH: xcb_atom_enum_t = 57;
pub const XCB_ATOM_X_HEIGHT: xcb_atom_enum_t = 56;
pub const XCB_ATOM_ITALIC_ANGLE: xcb_atom_enum_t = 55;
pub const XCB_ATOM_STRIKEOUT_DESCENT: xcb_atom_enum_t = 54;
pub const XCB_ATOM_STRIKEOUT_ASCENT: xcb_atom_enum_t = 53;
pub const XCB_ATOM_UNDERLINE_THICKNESS: xcb_atom_enum_t = 52;
pub const XCB_ATOM_UNDERLINE_POSITION: xcb_atom_enum_t = 51;
pub const XCB_ATOM_SUBSCRIPT_Y: xcb_atom_enum_t = 50;
pub const XCB_ATOM_SUBSCRIPT_X: xcb_atom_enum_t = 49;
pub const XCB_ATOM_SUPERSCRIPT_Y: xcb_atom_enum_t = 48;
pub const XCB_ATOM_SUPERSCRIPT_X: xcb_atom_enum_t = 47;
pub const XCB_ATOM_END_SPACE: xcb_atom_enum_t = 46;
pub const XCB_ATOM_MAX_SPACE: xcb_atom_enum_t = 45;
pub const XCB_ATOM_NORM_SPACE: xcb_atom_enum_t = 44;
pub const XCB_ATOM_MIN_SPACE: xcb_atom_enum_t = 43;
pub const XCB_ATOM_WM_ZOOM_HINTS: xcb_atom_enum_t = 42;
pub const XCB_ATOM_WM_SIZE_HINTS: xcb_atom_enum_t = 41;
pub const XCB_ATOM_WM_NORMAL_HINTS: xcb_atom_enum_t = 40;
pub const XCB_ATOM_WM_NAME: xcb_atom_enum_t = 39;
pub const XCB_ATOM_WM_ICON_SIZE: xcb_atom_enum_t = 38;
pub const XCB_ATOM_WM_ICON_NAME: xcb_atom_enum_t = 37;
pub const XCB_ATOM_WM_CLIENT_MACHINE: xcb_atom_enum_t = 36;
pub const XCB_ATOM_WM_HINTS: xcb_atom_enum_t = 35;
pub const XCB_ATOM_WM_COMMAND: xcb_atom_enum_t = 34;
pub const XCB_ATOM_WINDOW: xcb_atom_enum_t = 33;
pub const XCB_ATOM_VISUALID: xcb_atom_enum_t = 32;
pub const XCB_ATOM_STRING: xcb_atom_enum_t = 31;
pub const XCB_ATOM_RGB_RED_MAP: xcb_atom_enum_t = 30;
pub const XCB_ATOM_RGB_GREEN_MAP: xcb_atom_enum_t = 29;
pub const XCB_ATOM_RGB_GRAY_MAP: xcb_atom_enum_t = 28;
pub const XCB_ATOM_RGB_DEFAULT_MAP: xcb_atom_enum_t = 27;
pub const XCB_ATOM_RGB_BLUE_MAP: xcb_atom_enum_t = 26;
pub const XCB_ATOM_RGB_BEST_MAP: xcb_atom_enum_t = 25;
pub const XCB_ATOM_RGB_COLOR_MAP: xcb_atom_enum_t = 24;
pub const XCB_ATOM_RESOURCE_MANAGER: xcb_atom_enum_t = 23;
pub const XCB_ATOM_RECTANGLE: xcb_atom_enum_t = 22;
pub const XCB_ATOM_POINT: xcb_atom_enum_t = 21;
pub const XCB_ATOM_PIXMAP: xcb_atom_enum_t = 20;
pub const XCB_ATOM_INTEGER: xcb_atom_enum_t = 19;
pub const XCB_ATOM_FONT: xcb_atom_enum_t = 18;
pub const XCB_ATOM_DRAWABLE: xcb_atom_enum_t = 17;
pub const XCB_ATOM_CUT_BUFFER7: xcb_atom_enum_t = 16;
pub const XCB_ATOM_CUT_BUFFER6: xcb_atom_enum_t = 15;
pub const XCB_ATOM_CUT_BUFFER5: xcb_atom_enum_t = 14;
pub const XCB_ATOM_CUT_BUFFER4: xcb_atom_enum_t = 13;
pub const XCB_ATOM_CUT_BUFFER3: xcb_atom_enum_t = 12;
pub const XCB_ATOM_CUT_BUFFER2: xcb_atom_enum_t = 11;
pub const XCB_ATOM_CUT_BUFFER1: xcb_atom_enum_t = 10;
pub const XCB_ATOM_CUT_BUFFER0: xcb_atom_enum_t = 9;
pub const XCB_ATOM_CURSOR: xcb_atom_enum_t = 8;
pub const XCB_ATOM_COLORMAP: xcb_atom_enum_t = 7;
pub const XCB_ATOM_CARDINAL: xcb_atom_enum_t = 6;
pub const XCB_ATOM_BITMAP: xcb_atom_enum_t = 5;
pub const XCB_ATOM_ATOM: xcb_atom_enum_t = 4;
pub const XCB_ATOM_ARC: xcb_atom_enum_t = 3;
pub const XCB_ATOM_SECONDARY: xcb_atom_enum_t = 2;
pub const XCB_ATOM_PRIMARY: xcb_atom_enum_t = 1;
pub const XCB_ATOM_ANY: xcb_atom_enum_t = 0;
pub const XCB_ATOM_NONE: xcb_atom_enum_t = 0;
pub type xcb_prop_mode_t = ::core::ffi::c_uint;
pub const XCB_PROP_MODE_APPEND: xcb_prop_mode_t = 2;
pub const XCB_PROP_MODE_PREPEND: xcb_prop_mode_t = 1;
pub const XCB_PROP_MODE_REPLACE: xcb_prop_mode_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_query_pointer_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_query_pointer_reply_t {
    pub response_type: uint8_t,
    pub same_screen: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub root: xcb_window_t,
    pub child: xcb_window_t,
    pub root_x: int16_t,
    pub root_y: int16_t,
    pub win_x: int16_t,
    pub win_y: int16_t,
    pub mask: uint16_t,
    pub pad0: [uint8_t; 2],
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
pub type border_t = ::core::ffi::c_uint;
pub const BORDER_BOTTOM: border_t = 8;
pub const BORDER_TOP: border_t = 4;
pub const BORDER_RIGHT: border_t = 2;
pub const BORDER_LEFT: border_t = 1;
pub const DRAG_REVERT: drag_result_t = 2;
pub type drag_result_t = ::core::ffi::c_uint;
pub const DRAG_ABORT: drag_result_t = 3;
pub const DRAG_SUCCESS: drag_result_t = 1;
pub const DRAGGING: drag_result_t = 0;
pub type callback_t = Option<
    unsafe extern "C" fn(
        *mut Con,
        *mut Rect,
        uint32_t,
        uint32_t,
        *const xcb_button_press_event_t,
        *const ::core::ffi::c_void,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct resize_window_callback_params {
    pub corner: border_t,
    pub proportional: bool,
}
pub const __ASSERT_FUNCTION: [::core::ffi::c_char; 37] = unsafe {
    ::core::mem::transmute::<[u8; 37], [::core::ffi::c_char; 37]>(
        *b"void floating_move_to_pointer(Con *)\0",
    )
};
pub const XCB_NONE: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
unsafe extern "C" fn total_outputs_dimensions() -> Rect {
    if outputs.tqh_first.is_null() {
        return Rect {
            x: 0 as uint32_t,
            y: 0 as uint32_t,
            width: (*root_screen).width_in_pixels as uint32_t,
            height: (*root_screen).height_in_pixels as uint32_t,
        };
    }
    let mut output: *mut Output = ::core::ptr::null_mut::<Output>();
    let mut outputs_dimensions: Rect = Rect {
        x: 0 as uint32_t,
        y: 0 as uint32_t,
        width: 0 as uint32_t,
        height: 0 as uint32_t,
    };
    output = outputs.tqh_first as *mut Output;
    while !output.is_null() {
        outputs_dimensions.height = outputs_dimensions
            .height
            .wrapping_add((*output).rect.height);
        outputs_dimensions.width = outputs_dimensions.width.wrapping_add((*output).rect.width);
        output = (*output).outputs.tqe_next as *mut Output;
    }
    return outputs_dimensions;
}
unsafe extern "C" fn floating_set_hint_atom(mut con: *mut Con, mut floating: bool) {
    if !con_is_leaf(con) {
        let mut child: *mut Con = ::core::ptr::null_mut::<Con>();
        child = (*con).nodes_head.tqh_first as *mut Con;
        while !child.is_null() {
            floating_set_hint_atom(child, floating);
            child = (*child).nodes.tqe_next as *mut Con;
        }
    }
    if (*con).window.is_null() {
        return;
    }
    if floating {
        let mut val: uint32_t = 1 as uint32_t;
        xcb_change_property(
            conn,
            XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as uint8_t,
            (*(*con).window).id,
            A_I3_FLOATING_WINDOW,
            XCB_ATOM_CARDINAL as ::core::ffi::c_int as xcb_atom_t,
            32 as uint8_t,
            1 as uint32_t,
            &raw mut val as *const ::core::ffi::c_void,
        );
    } else {
        xcb_delete_property(conn, (*(*con).window).id, A_I3_FLOATING_WINDOW);
    }
    xcb_flush(conn);
}
#[no_mangle]
pub unsafe extern "C" fn floating_check_size(mut floating_con: *mut Con, mut prefer_height: bool) {
    let floating_sane_min_height: ::core::ffi::c_int = 50 as ::core::ffi::c_int;
    let floating_sane_min_width: ::core::ffi::c_int = 75 as ::core::ffi::c_int;
    let mut focused_con: *mut Con = con_descend_focused(floating_con);
    debuglog(
        b"%s:%s:%d - deco_rect.height = %d\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/floating.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"floating_check_size\0" as *const u8 as *const ::core::ffi::c_char,
        82 as ::core::ffi::c_int,
        (*focused_con).deco_rect.height,
    );
    let mut border_rect: Rect = con_border_style_rect(focused_con);
    border_rect.width = border_rect.width.wrapping_neg();
    border_rect.height = border_rect.height.wrapping_neg();
    border_rect.width = border_rect
        .width
        .wrapping_add((2 as ::core::ffi::c_int * (*focused_con).border_width) as uint32_t);
    border_rect.height = border_rect
        .height
        .wrapping_add((2 as ::core::ffi::c_int * (*focused_con).border_width) as uint32_t);
    debuglog(
        b"%s:%s:%d - floating_check_size, want min width %d, min height %d, border extra: w=%d, h=%d\n\0"
            as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/floating.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"floating_check_size\0" as *const u8 as *const ::core::ffi::c_char,
        97 as ::core::ffi::c_int,
        floating_sane_min_width,
        floating_sane_min_height,
        border_rect.width,
        border_rect.height,
    );
    let mut window: *mut i3Window = (*focused_con).window as *mut i3Window;
    if !window.is_null() {
        let mut min_width: ::core::ffi::c_int = if (*window).min_width != 0 {
            (*window).min_width
        } else {
            (*window).base_width
        };
        let mut min_height: ::core::ffi::c_int = if (*window).min_height != 0 {
            (*window).min_height
        } else {
            (*window).base_height
        };
        let mut base_width: ::core::ffi::c_int = if (*window).base_width != 0 {
            (*window).base_width
        } else {
            (*window).min_width
        };
        let mut base_height: ::core::ffi::c_int = if (*window).base_height != 0 {
            (*window).base_height
        } else {
            (*window).min_height
        };
        if min_width != 0 {
            (*floating_con).rect.width = (*floating_con).rect.width.wrapping_sub(border_rect.width);
            (*floating_con).rect.width =
                max((*floating_con).rect.width as ::core::ffi::c_int, min_width) as uint32_t;
            (*floating_con).rect.width = (*floating_con).rect.width.wrapping_add(border_rect.width);
        }
        if min_height != 0 {
            (*floating_con).rect.height =
                (*floating_con).rect.height.wrapping_sub(border_rect.height);
            (*floating_con).rect.height = max(
                (*floating_con).rect.height as ::core::ffi::c_int,
                min_height,
            ) as uint32_t;
            (*floating_con).rect.height =
                (*floating_con).rect.height.wrapping_add(border_rect.height);
        }
        if (*window).max_width != 0 {
            (*floating_con).rect.width = (*floating_con).rect.width.wrapping_sub(border_rect.width);
            (*floating_con).rect.width = min(
                (*floating_con).rect.width as ::core::ffi::c_int,
                (*window).max_width,
            ) as uint32_t;
            (*floating_con).rect.width = (*floating_con).rect.width.wrapping_add(border_rect.width);
        }
        if (*window).max_height != 0 {
            (*floating_con).rect.height =
                (*floating_con).rect.height.wrapping_sub(border_rect.height);
            (*floating_con).rect.height = min(
                (*floating_con).rect.height as ::core::ffi::c_int,
                (*window).max_height,
            ) as uint32_t;
            (*floating_con).rect.height =
                (*floating_con).rect.height.wrapping_add(border_rect.height);
        }
        let min_ar: ::core::ffi::c_double = (*window).min_aspect_ratio;
        let max_ar: ::core::ffi::c_double = (*window).max_aspect_ratio;
        if (*floating_con).fullscreen_mode as ::core::ffi::c_uint
            == CF_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
            && (min_ar > 0 as ::core::ffi::c_int as ::core::ffi::c_double
                || max_ar > 0 as ::core::ffi::c_int as ::core::ffi::c_double)
        {
            let mut width: ::core::ffi::c_double = (*floating_con)
                .rect
                .width
                .wrapping_sub((*window).base_width as uint32_t)
                .wrapping_sub(border_rect.width)
                as ::core::ffi::c_double;
            let mut height: ::core::ffi::c_double = (*floating_con)
                .rect
                .height
                .wrapping_sub((*window).base_height as uint32_t)
                .wrapping_sub(border_rect.height)
                as ::core::ffi::c_double;
            let ar: ::core::ffi::c_double = width / height;
            let mut new_ar: ::core::ffi::c_double =
                -(1 as ::core::ffi::c_int) as ::core::ffi::c_double;
            if min_ar > 0 as ::core::ffi::c_int as ::core::ffi::c_double && ar < min_ar {
                new_ar = min_ar;
            } else if max_ar > 0 as ::core::ffi::c_int as ::core::ffi::c_double && ar > max_ar {
                new_ar = max_ar;
            }
            if new_ar > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                if prefer_height {
                    width = round(height * new_ar);
                    height = round(width / new_ar);
                } else {
                    height = round(width / new_ar);
                    width = round(height * new_ar);
                }
                (*floating_con).rect.width = (width
                    + (*window).base_width as ::core::ffi::c_double
                    + border_rect.width as ::core::ffi::c_double)
                    as uint32_t;
                (*floating_con).rect.height = (height
                    + (*window).base_height as ::core::ffi::c_double
                    + border_rect.height as ::core::ffi::c_double)
                    as uint32_t;
            }
        }
        if (*window).height_increment != 0
            && (*floating_con).rect.height
                >= (base_height as uint32_t).wrapping_add(border_rect.height)
        {
            (*floating_con).rect.height = (*floating_con)
                .rect
                .height
                .wrapping_sub((base_height as uint32_t).wrapping_add(border_rect.height));
            (*floating_con).rect.height = (*floating_con).rect.height.wrapping_sub(
                (*floating_con)
                    .rect
                    .height
                    .wrapping_rem((*window).height_increment as uint32_t),
            );
            (*floating_con).rect.height = (*floating_con)
                .rect
                .height
                .wrapping_add((base_height as uint32_t).wrapping_add(border_rect.height));
        }
        if (*window).width_increment != 0
            && (*floating_con).rect.width
                >= (base_width as uint32_t).wrapping_add(border_rect.width)
        {
            (*floating_con).rect.width = (*floating_con)
                .rect
                .width
                .wrapping_sub((base_width as uint32_t).wrapping_add(border_rect.width));
            (*floating_con).rect.width = (*floating_con).rect.width.wrapping_sub(
                (*floating_con)
                    .rect
                    .width
                    .wrapping_rem((*window).width_increment as uint32_t),
            );
            (*floating_con).rect.width = (*floating_con)
                .rect
                .width
                .wrapping_add((base_width as uint32_t).wrapping_add(border_rect.width));
        }
    }
    if config.floating_minimum_height != -(1 as int32_t) {
        (*floating_con).rect.height = (*floating_con).rect.height.wrapping_sub(border_rect.height);
        if config.floating_minimum_height == 0 as int32_t {
            (*floating_con).rect.height = max(
                (*floating_con).rect.height as ::core::ffi::c_int,
                floating_sane_min_height,
            ) as uint32_t;
        } else {
            (*floating_con).rect.height = max(
                (*floating_con).rect.height as ::core::ffi::c_int,
                config.floating_minimum_height as ::core::ffi::c_int,
            ) as uint32_t;
        }
        (*floating_con).rect.height = (*floating_con).rect.height.wrapping_add(border_rect.height);
    }
    if config.floating_minimum_width != -(1 as int32_t) {
        (*floating_con).rect.width = (*floating_con).rect.width.wrapping_sub(border_rect.width);
        if config.floating_minimum_width == 0 as int32_t {
            (*floating_con).rect.width = max(
                (*floating_con).rect.width as ::core::ffi::c_int,
                floating_sane_min_width,
            ) as uint32_t;
        } else {
            (*floating_con).rect.width = max(
                (*floating_con).rect.width as ::core::ffi::c_int,
                config.floating_minimum_width as ::core::ffi::c_int,
            ) as uint32_t;
        }
        (*floating_con).rect.width = (*floating_con).rect.width.wrapping_add(border_rect.width);
    }
    let mut floating_sane_max_dimensions: Rect = total_outputs_dimensions();
    if config.floating_maximum_height != -(1 as int32_t) {
        (*floating_con).rect.height = (*floating_con).rect.height.wrapping_sub(border_rect.height);
        if config.floating_maximum_height == 0 as int32_t {
            (*floating_con).rect.height = min(
                (*floating_con).rect.height as ::core::ffi::c_int,
                floating_sane_max_dimensions.height as ::core::ffi::c_int,
            ) as uint32_t;
        } else {
            (*floating_con).rect.height = min(
                (*floating_con).rect.height as ::core::ffi::c_int,
                config.floating_maximum_height as ::core::ffi::c_int,
            ) as uint32_t;
        }
        (*floating_con).rect.height = (*floating_con).rect.height.wrapping_add(border_rect.height);
    }
    if config.floating_maximum_width != -(1 as int32_t) {
        (*floating_con).rect.width = (*floating_con).rect.width.wrapping_sub(border_rect.width);
        if config.floating_maximum_width == 0 as int32_t {
            (*floating_con).rect.width = min(
                (*floating_con).rect.width as ::core::ffi::c_int,
                floating_sane_max_dimensions.width as ::core::ffi::c_int,
            ) as uint32_t;
        } else {
            (*floating_con).rect.width = min(
                (*floating_con).rect.width as ::core::ffi::c_int,
                config.floating_maximum_width as ::core::ffi::c_int,
            ) as uint32_t;
        }
        (*floating_con).rect.width = (*floating_con).rect.width.wrapping_add(border_rect.width);
    }
}
#[no_mangle]
pub unsafe extern "C" fn floating_enable(mut con: *mut Con, mut automatic: bool) -> bool {
    let mut set_focus: bool = con == focused;
    if con_is_docked(con) {
        verboselog(
            b"Container is a dock window, not enabling floating mode.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        return false_0 != 0;
    }
    if con_is_floating(con) {
        verboselog(
            b"Container is already in floating mode, not doing anything.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        return false_0 != 0;
    }
    if (*con).type_0 as ::core::ffi::c_uint
        == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        verboselog(
            b"Container is a workspace, not enabling floating mode.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        return false_0 != 0;
    }
    let mut focus_head_placeholder: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut focus_before_parent: bool = true_0 != 0;
    if !set_focus {
        let mut ancestor: *mut Con = con;
        while (*(*ancestor).parent).type_0 as ::core::ffi::c_uint
            != CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            focus_before_parent = (focus_before_parent as ::core::ffi::c_int
                & ((*(*ancestor).parent).focus_head.tqh_first == ancestor) as ::core::ffi::c_int)
                != 0;
            ancestor = (*ancestor).parent as *mut Con;
        }
        if focus_before_parent {
            focus_head_placeholder =
                *(*((*ancestor).focused.tqe_prev as *mut focus_head)).tqh_last as *mut Con;
        } else {
            focus_head_placeholder = (*ancestor).focused.tqe_next as *mut Con;
        }
    }
    con_detach(con);
    con_fix_percent((*con).parent as *mut Con);
    let mut nc: *mut Con = con_new(
        ::core::ptr::null_mut::<Con>(),
        ::core::ptr::null_mut::<i3Window>(),
    );
    let mut ws: *mut Con = con_get_workspace(con);
    (*nc).parent = ws as *mut Con;
    (*nc).type_0 = CT_FLOATING_CON;
    (*nc).layout = L_SPLITH;
    (*nc).floating_windows.tqe_next = (*ws).floating_head.tqh_first;
    if !(*nc).floating_windows.tqe_next.is_null() {
        (*(*ws).floating_head.tqh_first).floating_windows.tqe_prev =
            &raw mut (*nc).floating_windows.tqe_next;
    } else {
        (*ws).floating_head.tqh_last = &raw mut (*nc).floating_windows.tqe_next;
    }
    (*ws).floating_head.tqh_first = nc as *mut Con;
    (*nc).floating_windows.tqe_prev = &raw mut (*ws).floating_head.tqh_first;
    let mut fh: *mut focus_head = &raw mut (*ws).focus_head;
    if focus_before_parent {
        if !focus_head_placeholder.is_null() {
            (*nc).focused.tqe_next = (*focus_head_placeholder).focused.tqe_next;
            if !(*nc).focused.tqe_next.is_null() {
                (*(*nc).focused.tqe_next).focused.tqe_prev = &raw mut (*nc).focused.tqe_next;
            } else {
                (*fh).tqh_last = &raw mut (*nc).focused.tqe_next;
            }
            (*focus_head_placeholder).focused.tqe_next = nc as *mut Con;
            (*nc).focused.tqe_prev = &raw mut (*focus_head_placeholder).focused.tqe_next;
        } else {
            (*nc).focused.tqe_next = (*fh).tqh_first;
            if !(*nc).focused.tqe_next.is_null() {
                (*(*fh).tqh_first).focused.tqe_prev = &raw mut (*nc).focused.tqe_next;
            } else {
                (*fh).tqh_last = &raw mut (*nc).focused.tqe_next;
            }
            (*fh).tqh_first = nc as *mut Con;
            (*nc).focused.tqe_prev = &raw mut (*fh).tqh_first;
        }
    } else if !focus_head_placeholder.is_null() {
        (*nc).focused.tqe_prev = (*focus_head_placeholder).focused.tqe_prev;
        (*nc).focused.tqe_next = focus_head_placeholder as *mut Con;
        *(*focus_head_placeholder).focused.tqe_prev = nc as *mut Con;
        (*focus_head_placeholder).focused.tqe_prev = &raw mut (*nc).focused.tqe_next;
    } else {
        (*nc).focused.tqe_next = ::core::ptr::null_mut::<Con>();
        (*nc).focused.tqe_prev = (*fh).tqh_last;
        *(*fh).tqh_last = nc as *mut Con;
        (*fh).tqh_last = &raw mut (*nc).focused.tqe_next;
    }
    if ((*(*con).parent).type_0 as ::core::ffi::c_uint
        == CT_CON as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*(*con).parent).type_0 as ::core::ffi::c_uint
            == CT_FLOATING_CON as ::core::ffi::c_int as ::core::ffi::c_uint)
        && con_num_children((*con).parent as *mut Con) == 0 as ::core::ffi::c_int
    {
        debuglog(
            b"%s:%s:%d - Old container empty after setting this child to floating, closing\n\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/floating.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"floating_enable\0" as *const u8 as *const ::core::ffi::c_char,
            316 as ::core::ffi::c_int,
        );
        let mut parent: *mut Con = (*con).parent as *mut Con;
        (*con).parent = ::core::ptr::null_mut::<Con>();
        tree_close_internal(parent, DONT_KILL_WINDOW, false_0 != 0);
    }
    let mut name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    sasprintf(
        &raw mut name,
        b"[i3 con] floatingcon around %p\0" as *const u8 as *const ::core::ffi::c_char,
        con,
    );
    x_set_name(nc, name);
    free(name as *mut ::core::ffi::c_void);
    debuglog(
        b"%s:%s:%d - Original rect: (%d, %d) with %d x %d\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/floating.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"floating_enable\0" as *const u8 as *const ::core::ffi::c_char,
        328 as ::core::ffi::c_int,
        (*con).rect.x,
        (*con).rect.y,
        (*con).rect.width,
        (*con).rect.height,
    );
    debuglog(
        b"%s:%s:%d - Geometry = (%d, %d) with %d x %d\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/floating.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"floating_enable\0" as *const u8 as *const ::core::ffi::c_char,
        329 as ::core::ffi::c_int,
        (*con).geometry.x,
        (*con).geometry.y,
        (*con).geometry.width,
        (*con).geometry.height,
    );
    (*nc).rect = (*con).geometry;
    if rect_equals(
        (*nc).rect as Rect,
        Rect {
            x: 0 as uint32_t,
            y: 0 as uint32_t,
            width: 0 as uint32_t,
            height: 0 as uint32_t,
        },
    ) {
        debuglog(
            b"%s:%s:%d - Geometry not set, combining children\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/floating.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"floating_enable\0" as *const u8 as *const ::core::ffi::c_char,
            334 as ::core::ffi::c_int,
        );
        let mut child: *mut Con = ::core::ptr::null_mut::<Con>();
        child = (*con).nodes_head.tqh_first as *mut Con;
        while !child.is_null() {
            debuglog(
                b"%s:%s:%d - child geometry: %d x %d\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"../src/floating.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"floating_enable\0" as *const u8 as *const ::core::ffi::c_char,
                337 as ::core::ffi::c_int,
                (*child).geometry.width,
                (*child).geometry.height,
            );
            (*nc).rect.width = (*nc).rect.width.wrapping_add((*child).geometry.width);
            (*nc).rect.height = max(
                (*nc).rect.height as ::core::ffi::c_int,
                (*child).geometry.height as ::core::ffi::c_int,
            ) as uint32_t;
            child = (*child).nodes.tqe_next as *mut Con;
        }
    }
    (*con).nodes.tqe_next = ::core::ptr::null_mut::<Con>();
    (*con).nodes.tqe_prev = (*nc).nodes_head.tqh_last;
    *(*nc).nodes_head.tqh_last = con as *mut Con;
    (*nc).nodes_head.tqh_last = &raw mut (*con).nodes.tqe_next;
    (*con).focused.tqe_next = ::core::ptr::null_mut::<Con>();
    (*con).focused.tqe_prev = (*nc).focus_head.tqh_last;
    *(*nc).focus_head.tqh_last = con as *mut Con;
    (*nc).focus_head.tqh_last = &raw mut (*con).focused.tqe_next;
    (*con).parent = nc as *mut Con;
    (*con).percent = 1.0f64;
    (*con).floating = FLOATING_USER_ON;
    if automatic {
        (*con).max_user_border_style = config.default_floating_border;
        (*con).border_style = (*con).max_user_border_style;
    }
    let mut bsr: Rect = con_border_style_rect(con);
    (*nc).rect.height = (*nc).rect.height.wrapping_sub(bsr.height);
    (*nc).rect.width = (*nc).rect.width.wrapping_sub(bsr.width);
    (*nc).rect.height = (*nc)
        .rect
        .height
        .wrapping_add(((*con).border_width * 2 as ::core::ffi::c_int) as uint32_t);
    (*nc).rect.width = (*nc)
        .rect
        .width
        .wrapping_add(((*con).border_width * 2 as ::core::ffi::c_int) as uint32_t);
    floating_check_size(nc, false_0 != 0);
    if (*nc).rect.x == 0 as uint32_t && (*nc).rect.y == 0 as uint32_t {
        let mut leader: *mut Con = ::core::ptr::null_mut::<Con>();
        if !(*con).window.is_null()
            && (*(*con).window).leader as ::core::ffi::c_long != XCB_NONE
            && (*(*con).window).id != (*(*con).window).leader
            && {
                leader = con_by_window_id((*(*con).window).leader);
                !leader.is_null()
            }
        {
            debuglog(
                b"%s:%s:%d - Centering above leader\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"../src/floating.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"floating_enable\0" as *const u8 as *const ::core::ffi::c_char,
                377 as ::core::ffi::c_int,
            );
            floating_center(nc, (*leader).rect as Rect);
        } else {
            floating_center(nc, (*ws).rect as Rect);
        }
    }
    let mut current_output: *mut Output = get_output_from_rect((*nc).rect as Rect);
    let mut correct_output: *mut Con = con_get_output(ws);
    if current_output.is_null() || (*current_output).con != correct_output {
        debuglog(
            b"%s:%s:%d - This floating window is on the wrong output, fixing coordinates (currently (%d, %d))\n\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/floating.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"floating_enable\0" as *const u8 as *const ::core::ffi::c_char,
            391 as ::core::ffi::c_int,
            (*nc).rect.x,
            (*nc).rect.y,
        );
        if !current_output.is_null() {
            floating_fix_coordinates(
                nc,
                &raw mut (*(*current_output).con).rect,
                &raw mut (*correct_output).rect,
            );
            current_output = get_output_from_rect((*nc).rect as Rect);
        }
        if current_output.is_null() || (*current_output).con != correct_output {
            floating_center(nc, (*ws).rect as Rect);
        }
    }
    debuglog(
        b"%s:%s:%d - Floating rect: (%d, %d) with %d x %d\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/floating.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"floating_enable\0" as *const u8 as *const ::core::ffi::c_char,
        405 as ::core::ffi::c_int,
        (*nc).rect.x,
        (*nc).rect.y,
        (*nc).rect.width,
        (*nc).rect.height,
    );
    render_con(nc);
    if set_focus {
        con_activate(con);
    }
    floating_set_hint_atom(nc, true_0 != 0);
    ipc_send_window_event(
        b"floating\0" as *const u8 as *const ::core::ffi::c_char,
        con,
    );
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn floating_disable(mut con: *mut Con) {
    if !con_is_floating(con) {
        verboselog(
            b"Container isn't floating, not doing anything.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        return;
    }
    let mut ws: *mut Con = con_get_workspace(con);
    if con_is_internal(ws) {
        verboselog(
            b"Can't disable floating for container in internal workspace.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        return;
    }
    let mut tiling_focused: *mut Con = con_descend_tiling_focused(ws);
    if (*tiling_focused).type_0 as ::core::ffi::c_uint
        == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut parent: *mut Con = (*con).parent as *mut Con;
        con_detach(con);
        (*con).parent = ::core::ptr::null_mut::<Con>();
        tree_close_internal(parent, DONT_KILL_WINDOW, true_0 != 0);
        con_attach(con, tiling_focused, false_0 != 0);
        (*con).percent = 0.0f64;
        con_fix_percent((*con).parent as *mut Con);
    } else {
        insert_con_into(con, tiling_focused, AFTER);
    }
    (*con).floating = FLOATING_USER_OFF;
    floating_set_hint_atom(con, false_0 != 0);
    ipc_send_window_event(
        b"floating\0" as *const u8 as *const ::core::ffi::c_char,
        con,
    );
}
#[no_mangle]
pub unsafe extern "C" fn toggle_floating_mode(mut con: *mut Con, mut automatic: bool) {
    if (*con).type_0 as ::core::ffi::c_uint
        == CT_FLOATING_CON as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        errorlog(
            b"ERROR: Cannot toggle floating mode on con = %p because it is of type CT_FLOATING_CON.\n\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            con,
        );
        return;
    }
    if con_is_floating(con) {
        verboselog(
            b"already floating, re-setting to tiling\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        floating_disable(con);
        return;
    }
    floating_enable(con, automatic);
}
#[no_mangle]
pub unsafe extern "C" fn floating_raise_con(mut con: *mut Con) {
    debuglog(
        b"%s:%s:%d - Raising floating con %p / %s\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/floating.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"floating_raise_con\0" as *const u8 as *const ::core::ffi::c_char,
        479 as ::core::ffi::c_int,
        con,
        (*con).name,
    );
    if !(*con).floating_windows.tqe_next.is_null() {
        (*(*con).floating_windows.tqe_next)
            .floating_windows
            .tqe_prev = (*con).floating_windows.tqe_prev;
    } else {
        (*(*con).parent).floating_head.tqh_last = (*con).floating_windows.tqe_prev;
    }
    *(*con).floating_windows.tqe_prev = (*con).floating_windows.tqe_next;
    (*con).floating_windows.tqe_next = ::core::ptr::null_mut::<Con>();
    (*con).floating_windows.tqe_prev = (*(*con).parent).floating_head.tqh_last;
    *(*(*con).parent).floating_head.tqh_last = con as *mut Con;
    (*(*con).parent).floating_head.tqh_last = &raw mut (*con).floating_windows.tqe_next;
}
#[no_mangle]
pub unsafe extern "C" fn floating_maybe_reassign_ws(mut con: *mut Con) -> bool {
    if con_is_internal(con_get_workspace(con)) {
        debuglog(
            b"%s:%s:%d - Con in an internal workspace\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/floating.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"floating_maybe_reassign_ws\0" as *const u8 as *const ::core::ffi::c_char,
            491 as ::core::ffi::c_int,
        );
        return false_0 != 0;
    }
    let mut output: *mut Output = get_output_from_rect((*con).rect as Rect);
    if output.is_null() {
        errorlog(
            b"ERROR: No output found at destination coordinates?\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        return false_0 != 0;
    }
    if con_get_output(con) == (*output).con {
        debuglog(
            b"%s:%s:%d - still the same ws\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b"../src/floating.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"floating_maybe_reassign_ws\0" as *const u8 as *const ::core::ffi::c_char,
            503 as ::core::ffi::c_int,
        );
        return false_0 != 0;
    }
    debuglog(
        b"%s:%s:%d - Need to re-assign!\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/floating.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"floating_maybe_reassign_ws\0" as *const u8 as *const ::core::ffi::c_char,
        507 as ::core::ffi::c_int,
    );
    let mut content: *mut Con = output_get_content((*output).con);
    let mut ws: *mut Con = (*content).focus_head.tqh_first as *mut Con;
    debuglog(
        b"%s:%s:%d - Moving con %p / %s to workspace %p / %s\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/floating.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"floating_maybe_reassign_ws\0" as *const u8 as *const ::core::ffi::c_char,
        511 as ::core::ffi::c_int,
        con,
        (*con).name,
        ws,
        (*ws).name,
    );
    let mut needs_focus: *mut Con = con_descend_focused(con);
    if !con_inside_focused(needs_focus) {
        needs_focus = ::core::ptr::null_mut::<Con>();
    }
    con_move_to_workspace(con, ws, false_0 != 0, true_0 != 0, false_0 != 0);
    if !needs_focus.is_null() {
        workspace_show(ws);
        con_activate(needs_focus);
    }
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn floating_center(mut con: *mut Con, mut rect: Rect) {
    (*con).rect.x = rect
        .x
        .wrapping_add(rect.width.wrapping_div(2 as uint32_t))
        .wrapping_sub((*con).rect.width.wrapping_div(2 as uint32_t));
    (*con).rect.y = rect
        .y
        .wrapping_add(rect.height.wrapping_div(2 as uint32_t))
        .wrapping_sub((*con).rect.height.wrapping_div(2 as uint32_t));
}
#[no_mangle]
pub unsafe extern "C" fn floating_move_to_pointer(mut con: *mut Con) {
    if (*con).type_0 as ::core::ffi::c_uint
        == CT_FLOATING_CON as ::core::ffi::c_int as ::core::ffi::c_uint
    {
    } else {
        __assert_fail(
            b"con->type == CT_FLOATING_CON\0" as *const u8 as *const ::core::ffi::c_char,
            b"../src/floating.c\0" as *const u8 as *const ::core::ffi::c_char,
            538 as ::core::ffi::c_uint,
            __ASSERT_FUNCTION.as_ptr(),
        );
    };
    let mut reply: *mut xcb_query_pointer_reply_t = xcb_query_pointer_reply(
        conn,
        xcb_query_pointer(conn, root),
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply.is_null() {
        errorlog(
            b"ERROR: could not query pointer position, not moving this container\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        return;
    }
    let mut output: *mut Output = get_output_containing(
        (*reply).root_x as ::core::ffi::c_uint,
        (*reply).root_y as ::core::ffi::c_uint,
    );
    if output.is_null() {
        errorlog(
            b"ERROR: The pointer is not on any output, cannot move the container here.\n\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        return;
    }
    let mut x: int32_t = ((*reply).root_x as uint32_t)
        .wrapping_sub((*con).rect.width.wrapping_div(2 as uint32_t))
        as int32_t;
    let mut y: int32_t = ((*reply).root_y as uint32_t)
        .wrapping_sub((*con).rect.height.wrapping_div(2 as uint32_t))
        as int32_t;
    free(reply as *mut ::core::ffi::c_void);
    reply = ::core::ptr::null_mut::<xcb_query_pointer_reply_t>();
    x = if x > (*output).rect.x as int32_t {
        x
    } else {
        (*output).rect.x as int32_t
    };
    y = if y > (*output).rect.y as int32_t {
        y
    } else {
        (*output).rect.y as int32_t
    };
    if (x as uint32_t).wrapping_add((*con).rect.width)
        > (*output).rect.x.wrapping_add((*output).rect.width)
    {
        x = (*output)
            .rect
            .x
            .wrapping_add((*output).rect.width)
            .wrapping_sub((*con).rect.width) as int32_t;
    }
    if (y as uint32_t).wrapping_add((*con).rect.height)
        > (*output).rect.y.wrapping_add((*output).rect.height)
    {
        y = (*output)
            .rect
            .y
            .wrapping_add((*output).rect.height)
            .wrapping_sub((*con).rect.height) as int32_t;
    }
    floating_reposition(
        con,
        Rect {
            x: x as uint32_t,
            y: y as uint32_t,
            width: (*con).rect.width,
            height: (*con).rect.height,
        },
    );
}
unsafe extern "C" fn drag_window_callback(
    mut con: *mut Con,
    mut old_rect: *mut Rect,
    mut new_x: uint32_t,
    mut new_y: uint32_t,
    mut event: *const xcb_button_press_event_t,
    mut extra: *const ::core::ffi::c_void,
) {
    (*con).rect.x = (*old_rect)
        .x
        .wrapping_add(new_x.wrapping_sub((*event).root_x as uint32_t));
    (*con).rect.y = (*old_rect)
        .y
        .wrapping_add(new_y.wrapping_sub((*event).root_y as uint32_t));
    render_con(con);
    x_push_node(con);
    xcb_flush(conn);
    if !floating_maybe_reassign_ws(con) {
        return;
    }
    x_set_warp_to(::core::ptr::null_mut::<Rect>());
    tree_render();
}
#[no_mangle]
pub unsafe extern "C" fn floating_drag_window(
    mut con: *mut Con,
    mut event: *const xcb_button_press_event_t,
    mut use_threshold: bool,
) {
    debuglog(
        b"%s:%s:%d - floating_drag_window\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/floating.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"floating_drag_window\0" as *const u8 as *const ::core::ffi::c_char,
        595 as ::core::ffi::c_int,
    );
    tree_render();
    let mut initial_rect: Rect = (*con).rect as Rect;
    let mut drag_result: drag_result_t = drag_pointer(
        con,
        event,
        XCB_NONE as xcb_window_t,
        XCURSOR_CURSOR_MOVE as ::core::ffi::c_int,
        use_threshold,
        Some(
            drag_window_callback
                as unsafe extern "C" fn(
                    *mut Con,
                    *mut Rect,
                    uint32_t,
                    uint32_t,
                    *const xcb_button_press_event_t,
                    *const ::core::ffi::c_void,
                ) -> (),
        ),
        ::core::ptr::null::<::core::ffi::c_void>(),
    );
    if !con_exists(con) {
        debuglog(
            b"%s:%s:%d - The container has been closed in the meantime.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/floating.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"floating_drag_window\0" as *const u8 as *const ::core::ffi::c_char,
            608 as ::core::ffi::c_int,
        );
        return;
    }
    if drag_result as ::core::ffi::c_uint
        == DRAG_REVERT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        floating_reposition(con, initial_rect);
        return;
    }
    if (*con).scratchpad_state as ::core::ffi::c_uint
        == SCRATCHPAD_FRESH as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*con).scratchpad_state = SCRATCHPAD_CHANGED;
    }
    tree_render();
}
unsafe extern "C" fn resize_window_callback(
    mut con: *mut Con,
    mut old_rect: *mut Rect,
    mut new_x: uint32_t,
    mut new_y: uint32_t,
    mut event: *const xcb_button_press_event_t,
    mut extra: *const ::core::ffi::c_void,
) {
    let mut params: *const resize_window_callback_params =
        extra as *const resize_window_callback_params;
    let mut corner: border_t = (*params).corner;
    let mut dest_x: int32_t = (*con).rect.x as int32_t;
    let mut dest_y: int32_t = (*con).rect.y as int32_t;
    let mut dest_width: uint32_t = 0;
    let mut dest_height: uint32_t = 0;
    let mut ratio: ::core::ffi::c_double =
        (*old_rect).width as ::core::ffi::c_double / (*old_rect).height as ::core::ffi::c_double;
    if corner as ::core::ffi::c_uint & BORDER_LEFT as ::core::ffi::c_int as ::core::ffi::c_uint != 0
    {
        dest_width = (*old_rect)
            .width
            .wrapping_sub(new_x.wrapping_sub((*event).root_x as uint32_t));
    } else {
        dest_width = (*old_rect)
            .width
            .wrapping_add(new_x.wrapping_sub((*event).root_x as uint32_t));
    }
    if corner as ::core::ffi::c_uint & BORDER_TOP as ::core::ffi::c_int as ::core::ffi::c_uint != 0
    {
        dest_height = (*old_rect)
            .height
            .wrapping_sub(new_y.wrapping_sub((*event).root_y as uint32_t));
    } else {
        dest_height = (*old_rect)
            .height
            .wrapping_add(new_y.wrapping_sub((*event).root_y as uint32_t));
    }
    if (*params).proportional {
        dest_width = max(
            dest_width as ::core::ffi::c_int,
            (dest_height as ::core::ffi::c_double * ratio) as ::core::ffi::c_int,
        ) as uint32_t;
        dest_height = max(
            dest_height as ::core::ffi::c_int,
            (dest_width as ::core::ffi::c_double / ratio) as ::core::ffi::c_int,
        ) as uint32_t;
    }
    (*con).rect = Rect {
        x: dest_x as uint32_t,
        y: dest_y as uint32_t,
        width: dest_width,
        height: dest_height,
    } as Rect;
    floating_check_size(con, false_0 != 0);
    if corner as ::core::ffi::c_uint & BORDER_LEFT as ::core::ffi::c_int as ::core::ffi::c_uint != 0
    {
        dest_x = (*old_rect)
            .x
            .wrapping_add((*old_rect).width.wrapping_sub((*con).rect.width))
            as int32_t;
    }
    if corner as ::core::ffi::c_uint & BORDER_TOP as ::core::ffi::c_int as ::core::ffi::c_uint != 0
    {
        dest_y = (*old_rect)
            .y
            .wrapping_add((*old_rect).height.wrapping_sub((*con).rect.height))
            as int32_t;
    }
    (*con).rect.x = dest_x as uint32_t;
    (*con).rect.y = dest_y as uint32_t;
    render_con(con);
    x_push_changes(croot);
}
#[no_mangle]
pub unsafe extern "C" fn floating_resize_window(
    mut con: *mut Con,
    proportional: bool,
    mut event: *const xcb_button_press_event_t,
) {
    debuglog(
        b"%s:%s:%d - floating_resize_window\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/floating.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"floating_resize_window\0" as *const u8 as *const ::core::ffi::c_char,
        699 as ::core::ffi::c_int,
    );
    tree_render();
    let mut corner: border_t = 0 as border_t;
    if (*event).event_x as ::core::ffi::c_int
        <= (*con).rect.width.wrapping_div(2 as uint32_t) as int16_t as ::core::ffi::c_int
    {
        corner = ::core::mem::transmute::<::core::ffi::c_uint, border_t>(
            corner as ::core::ffi::c_uint
                | BORDER_LEFT as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
    } else {
        corner = ::core::mem::transmute::<::core::ffi::c_uint, border_t>(
            corner as ::core::ffi::c_uint
                | BORDER_RIGHT as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
    }
    let mut cursor: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if (*event).event_y as ::core::ffi::c_int
        <= (*con).rect.height.wrapping_div(2 as uint32_t) as int16_t as ::core::ffi::c_int
    {
        corner = ::core::mem::transmute::<::core::ffi::c_uint, border_t>(
            corner as ::core::ffi::c_uint | BORDER_TOP as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
        cursor = if corner as ::core::ffi::c_uint
            & BORDER_LEFT as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
        {
            XCURSOR_CURSOR_TOP_LEFT_CORNER as ::core::ffi::c_int
        } else {
            XCURSOR_CURSOR_TOP_RIGHT_CORNER as ::core::ffi::c_int
        };
    } else {
        corner = ::core::mem::transmute::<::core::ffi::c_uint, border_t>(
            corner as ::core::ffi::c_uint
                | BORDER_BOTTOM as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
        cursor = if corner as ::core::ffi::c_uint
            & BORDER_LEFT as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
        {
            XCURSOR_CURSOR_BOTTOM_LEFT_CORNER as ::core::ffi::c_int
        } else {
            XCURSOR_CURSOR_BOTTOM_RIGHT_CORNER as ::core::ffi::c_int
        };
    }
    let mut params: resize_window_callback_params = resize_window_callback_params {
        corner: corner,
        proportional: proportional,
    };
    let mut initial_rect: Rect = (*con).rect as Rect;
    let mut drag_result: drag_result_t = drag_pointer(
        con,
        event,
        XCB_NONE as xcb_window_t,
        cursor,
        false_0 != 0,
        Some(
            resize_window_callback
                as unsafe extern "C" fn(
                    *mut Con,
                    *mut Rect,
                    uint32_t,
                    uint32_t,
                    *const xcb_button_press_event_t,
                    *const ::core::ffi::c_void,
                ) -> (),
        ),
        &raw mut params as *const ::core::ffi::c_void,
    );
    if !con_exists(con) {
        debuglog(
            b"%s:%s:%d - The container has been closed in the meantime.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/floating.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"floating_resize_window\0" as *const u8 as *const ::core::ffi::c_char,
            732 as ::core::ffi::c_int,
        );
        return;
    }
    if drag_result as ::core::ffi::c_uint
        == DRAG_REVERT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        floating_reposition(con, initial_rect);
    }
    if (*con).scratchpad_state as ::core::ffi::c_uint
        == SCRATCHPAD_FRESH as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*con).scratchpad_state = SCRATCHPAD_CHANGED;
    }
}
#[no_mangle]
pub unsafe extern "C" fn floating_reposition(mut con: *mut Con, mut newrect: Rect) -> bool {
    if output_containing_rect(newrect).is_null() {
        errorlog(
            b"ERROR: No output found at destination coordinates. Not repositioning.\n\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        return false_0 != 0;
    }
    (*con).rect = newrect as Rect;
    floating_maybe_reassign_ws(con);
    if (*con).scratchpad_state as ::core::ffi::c_uint
        == SCRATCHPAD_FRESH as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*con).scratchpad_state = SCRATCHPAD_CHANGED;
    }
    tree_render();
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn floating_resize(
    mut floating_con: *mut Con,
    mut x: uint32_t,
    mut y: uint32_t,
) {
    debuglog(
        b"%s:%s:%d - floating resize to %dx%d px\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/floating.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"floating_resize\0" as *const u8 as *const ::core::ffi::c_char,
        783 as ::core::ffi::c_int,
        x,
        y,
    );
    let mut rect: *mut Rect = &raw mut (*floating_con).rect;
    let mut focused_con: *mut Con = con_descend_focused(floating_con);
    if (*focused_con).window.is_null() {
        debuglog(
            b"%s:%s:%d - No window is focused. Not resizing.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/floating.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"floating_resize\0" as *const u8 as *const ::core::ffi::c_char,
            787 as ::core::ffi::c_int,
        );
        return;
    }
    let mut wi: ::core::ffi::c_int = (*(*focused_con).window).width_increment;
    let mut hi: ::core::ffi::c_int = (*(*focused_con).window).height_increment;
    let mut prefer_height: bool = (*rect).width == x;
    (*rect).width = x;
    (*rect).height = y;
    if wi != 0 {
        (*rect).width = (*rect).width.wrapping_add(
            ((wi - 1 as ::core::ffi::c_int) as uint32_t)
                .wrapping_sub((*rect).width)
                .wrapping_rem(wi as uint32_t),
        );
    }
    if hi != 0 {
        (*rect).height = (*rect).height.wrapping_add(
            ((hi - 1 as ::core::ffi::c_int) as uint32_t)
                .wrapping_sub((*rect).height)
                .wrapping_rem(hi as uint32_t),
        );
    }
    floating_check_size(floating_con, prefer_height);
    if (*floating_con).scratchpad_state as ::core::ffi::c_uint
        == SCRATCHPAD_FRESH as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*floating_con).scratchpad_state = SCRATCHPAD_CHANGED;
    }
}
#[no_mangle]
pub unsafe extern "C" fn floating_fix_coordinates(
    mut con: *mut Con,
    mut old_rect: *mut Rect,
    mut new_rect: *mut Rect,
) {
    debuglog(
        b"%s:%s:%d - Fixing coordinates of floating window %p (rect (%d, %d), %d x %d)\n\0"
            as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/floating.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"floating_fix_coordinates\0" as *const u8 as *const ::core::ffi::c_char,
        817 as ::core::ffi::c_int,
        con,
        (*con).rect.x,
        (*con).rect.y,
        (*con).rect.width,
        (*con).rect.height,
    );
    debuglog(
        b"%s:%s:%d - old_rect = (%d, %d), %d x %d\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/floating.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"floating_fix_coordinates\0" as *const u8 as *const ::core::ffi::c_char,
        819 as ::core::ffi::c_int,
        (*old_rect).x,
        (*old_rect).y,
        (*old_rect).width,
        (*old_rect).height,
    );
    debuglog(
        b"%s:%s:%d - new_rect = (%d, %d), %d x %d\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/floating.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"floating_fix_coordinates\0" as *const u8 as *const ::core::ffi::c_char,
        821 as ::core::ffi::c_int,
        (*new_rect).x,
        (*new_rect).y,
        (*new_rect).width,
        (*new_rect).height,
    );
    let mut rel_x: int32_t = (*con)
        .rect
        .x
        .wrapping_sub((*old_rect).x)
        .wrapping_add((*con).rect.width.wrapping_div(2 as uint32_t) as int32_t as uint32_t)
        as int32_t;
    let mut rel_y: int32_t = (*con)
        .rect
        .y
        .wrapping_sub((*old_rect).y)
        .wrapping_add((*con).rect.height.wrapping_div(2 as uint32_t) as int32_t as uint32_t)
        as int32_t;
    debuglog(
        b"%s:%s:%d - rel_x = %d, rel_y = %d, fraction_x = %f, fraction_y = %f, output->w = %d, output->h = %d\n\0"
            as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/floating.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"floating_fix_coordinates\0" as *const u8 as *const ::core::ffi::c_char,
        830 as ::core::ffi::c_int,
        rel_x,
        rel_y,
        rel_x as ::core::ffi::c_double / (*old_rect).width as ::core::ffi::c_double,
        rel_y as ::core::ffi::c_double / (*old_rect).height as ::core::ffi::c_double,
        (*old_rect).width,
        (*old_rect).height,
    );
    (*con).rect.x = ((*new_rect).x as int32_t as ::core::ffi::c_double
        + (rel_x * (*new_rect).width as int32_t) as ::core::ffi::c_double
            / (*old_rect).width as int32_t as ::core::ffi::c_double
        - (*con).rect.width.wrapping_div(2 as uint32_t) as int32_t as ::core::ffi::c_double)
        as uint32_t;
    (*con).rect.y = ((*new_rect).y as int32_t as ::core::ffi::c_double
        + (rel_y * (*new_rect).height as int32_t) as ::core::ffi::c_double
            / (*old_rect).height as int32_t as ::core::ffi::c_double
        - (*con).rect.height.wrapping_div(2 as uint32_t) as int32_t as ::core::ffi::c_double)
        as uint32_t;
    debuglog(
        b"%s:%s:%d - Resulting coordinates: x = %d, y = %d\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/floating.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"floating_fix_coordinates\0" as *const u8 as *const ::core::ffi::c_char,
        834 as ::core::ffi::c_int,
        (*con).rect.x,
        (*con).rect.y,
    );
}
