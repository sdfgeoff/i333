/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * tiling_drag.c: Reposition tiled windows by dragging.
 *
 */
// Mechanical file-by-file Rust translation of src/tiling_drag.c.
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
    fn xcb_destroy_window(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t;
    fn xcb_map_window(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t;
    fn xcb_configure_window(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        value_mask: uint16_t,
        value_list: *const ::core::ffi::c_void,
    ) -> xcb_void_cookie_t;
    fn xcb_circulate_window(
        c: *mut xcb_connection_t,
        direction: uint8_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t;
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
    fn xcb_flush(c: *mut xcb_connection_t) -> ::core::ffi::c_int;
    static mut conn: *mut xcb_connection_t;
    fn errorlog(fmt: *mut ::core::ffi::c_char, ...);
    fn debuglog(fmt: *mut ::core::ffi::c_char, ...);
    fn logical_px(logical: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn min(a: ::core::ffi::c_int, b: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn max(a: ::core::ffi::c_int, b: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn rect_contains(rect: Rect, x: uint32_t, y: uint32_t) -> bool;
    fn orientation_from_direction(direction: direction_t) -> orientation_t;
    fn position_from_direction(direction: direction_t) -> position_t;
    fn direction_to_string(direction: direction_t) -> *const ::core::ffi::c_char;
    fn position_to_string(position: position_t) -> *const ::core::ffi::c_char;
    static mut croot: *mut Con;
    static mut focused: *mut Con;
    static mut all_cons: all_cons_head;
    fn tree_split(con: *mut Con, orientation: orientation_t);
    fn tree_render();
    static mut config: Config;
    fn ipc_send_window_event(property: *const ::core::ffi::c_char, con: *mut Con);
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
    fn get_output_containing(x: ::core::ffi::c_uint, y: ::core::ffi::c_uint) -> *mut Output;
    fn workspace_is_visible(ws: *mut Con) -> bool;
    fn workspace_show(ws: *mut Con);
    fn x_mask_event_mask(mask: uint32_t);
    fn drag_pointer(
        con: *mut Con,
        event: *const xcb_button_press_event_t,
        confine_to: xcb_window_t,
        cursor: ::core::ffi::c_int,
        use_threshold: bool,
        callback: callback_t,
        extra: *const ::core::ffi::c_void,
    ) -> drag_result_t;
    fn con_focus(con: *mut Con);
    fn con_activate(con: *mut Con);
    fn con_has_managed_window(con: *mut Con) -> bool;
    fn con_is_hidden(con: *mut Con) -> bool;
    fn con_get_workspace(con: *mut Con) -> *mut Con;
    fn con_get_fullscreen_covering_ws(ws: *mut Con) -> *mut Con;
    fn con_is_internal(con: *mut Con) -> bool;
    fn con_is_floating(con: *mut Con) -> bool;
    fn con_exists(con: *mut Con) -> bool;
    fn con_num_children(con: *mut Con) -> ::core::ffi::c_int;
    fn con_enable_fullscreen(con: *mut Con, fullscreen_mode: fullscreen_mode_t);
    fn con_disable_fullscreen(con: *mut Con);
    fn con_move_to_target(con: *mut Con, target: *mut Con) -> bool;
    fn con_orientation(con: *mut Con) -> orientation_t;
    fn con_descend_tiling_focused(con: *mut Con) -> *mut Con;
    fn con_descend_direction(con: *mut Con, direction: direction_t) -> *mut Con;
    fn con_swap(first: *mut Con, second: *mut Con) -> bool;
    fn render_deco_height() -> ::core::ffi::c_int;
    fn tree_move(con: *mut Con, direction: direction_t);
    fn insert_con_into(con: *mut Con, target: *mut Con, position: position_t);
    fn output_get_content(output: *mut Con) -> *mut Con;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
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
pub type xcb_event_mask_t = ::core::ffi::c_uint;
pub const XCB_EVENT_MASK_OWNER_GRAB_BUTTON: xcb_event_mask_t = 16777216;
pub const XCB_EVENT_MASK_COLOR_MAP_CHANGE: xcb_event_mask_t = 8388608;
pub const XCB_EVENT_MASK_PROPERTY_CHANGE: xcb_event_mask_t = 4194304;
pub const XCB_EVENT_MASK_FOCUS_CHANGE: xcb_event_mask_t = 2097152;
pub const XCB_EVENT_MASK_SUBSTRUCTURE_REDIRECT: xcb_event_mask_t = 1048576;
pub const XCB_EVENT_MASK_SUBSTRUCTURE_NOTIFY: xcb_event_mask_t = 524288;
pub const XCB_EVENT_MASK_RESIZE_REDIRECT: xcb_event_mask_t = 262144;
pub const XCB_EVENT_MASK_STRUCTURE_NOTIFY: xcb_event_mask_t = 131072;
pub const XCB_EVENT_MASK_VISIBILITY_CHANGE: xcb_event_mask_t = 65536;
pub const XCB_EVENT_MASK_EXPOSURE: xcb_event_mask_t = 32768;
pub const XCB_EVENT_MASK_KEYMAP_STATE: xcb_event_mask_t = 16384;
pub const XCB_EVENT_MASK_BUTTON_MOTION: xcb_event_mask_t = 8192;
pub const XCB_EVENT_MASK_BUTTON_5_MOTION: xcb_event_mask_t = 4096;
pub const XCB_EVENT_MASK_BUTTON_4_MOTION: xcb_event_mask_t = 2048;
pub const XCB_EVENT_MASK_BUTTON_3_MOTION: xcb_event_mask_t = 1024;
pub const XCB_EVENT_MASK_BUTTON_2_MOTION: xcb_event_mask_t = 512;
pub const XCB_EVENT_MASK_BUTTON_1_MOTION: xcb_event_mask_t = 256;
pub const XCB_EVENT_MASK_POINTER_MOTION_HINT: xcb_event_mask_t = 128;
pub const XCB_EVENT_MASK_POINTER_MOTION: xcb_event_mask_t = 64;
pub const XCB_EVENT_MASK_LEAVE_WINDOW: xcb_event_mask_t = 32;
pub const XCB_EVENT_MASK_ENTER_WINDOW: xcb_event_mask_t = 16;
pub const XCB_EVENT_MASK_BUTTON_RELEASE: xcb_event_mask_t = 8;
pub const XCB_EVENT_MASK_BUTTON_PRESS: xcb_event_mask_t = 4;
pub const XCB_EVENT_MASK_KEY_RELEASE: xcb_event_mask_t = 2;
pub const XCB_EVENT_MASK_KEY_PRESS: xcb_event_mask_t = 1;
pub const XCB_EVENT_MASK_NO_EVENT: xcb_event_mask_t = 0;
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
pub type xcb_config_window_t = ::core::ffi::c_uint;
pub const XCB_CONFIG_WINDOW_STACK_MODE: xcb_config_window_t = 64;
pub const XCB_CONFIG_WINDOW_SIBLING: xcb_config_window_t = 32;
pub const XCB_CONFIG_WINDOW_BORDER_WIDTH: xcb_config_window_t = 16;
pub const XCB_CONFIG_WINDOW_HEIGHT: xcb_config_window_t = 8;
pub const XCB_CONFIG_WINDOW_WIDTH: xcb_config_window_t = 4;
pub const XCB_CONFIG_WINDOW_Y: xcb_config_window_t = 2;
pub const XCB_CONFIG_WINDOW_X: xcb_config_window_t = 1;
pub type xcb_circulate_t = ::core::ffi::c_uint;
pub const XCB_CIRCULATE_LOWER_HIGHEST: xcb_circulate_t = 1;
pub const XCB_CIRCULATE_RAISE_LOWEST: xcb_circulate_t = 0;
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
pub const DT_PARENT: drop_type_t = 2;
pub const DT_SIBLING: drop_type_t = 0;
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
pub const DT_CENTER: drop_type_t = 1;
pub type drop_type_t = ::core::ffi::c_uint;
pub const DRAG_REVERT: drag_result_t = 2;
pub type drag_result_t = ::core::ffi::c_uint;
pub const DRAG_ABORT: drag_result_t = 3;
pub const DRAG_SUCCESS: drag_result_t = 1;
pub const DRAGGING: drag_result_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct callback_params {
    pub indicator: *mut xcb_window_t,
    pub target: *mut *mut Con,
    pub direction: *mut direction_t,
    pub drop_type: *mut drop_type_t,
}
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
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const XCB_NONE: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const XCB_COPY_FROM_PARENT: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
unsafe extern "C" fn is_tiling_drop_target(mut con: *mut Con) -> bool {
    if !con_has_managed_window(con)
        || con_is_floating(con) as ::core::ffi::c_int != 0
        || con_is_hidden(con) as ::core::ffi::c_int != 0
    {
        return false_0 != 0;
    }
    let mut ws: *mut Con = con_get_workspace(con);
    if con_is_internal(ws) {
        return false_0 != 0;
    }
    if !workspace_is_visible(ws) {
        return false_0 != 0;
    }
    let mut fs: *mut Con = con_get_fullscreen_covering_ws(ws);
    if !fs.is_null() && fs != con {
        return false_0 != 0;
    }
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn has_drop_targets() -> bool {
    let mut drop_targets: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut con: *mut Con = ::core::ptr::null_mut::<Con>();
    con = all_cons.tqh_first as *mut Con;
    while !con.is_null() {
        if is_tiling_drop_target(con) {
            drop_targets += 1;
        }
        con = (*con).all_cons.tqe_next as *mut Con;
    }
    let mut output: *mut Con = ::core::ptr::null_mut::<Con>();
    output = (*croot).focus_head.tqh_first as *mut Con;
    while !output.is_null() {
        if !con_is_internal(output) {
            let mut visible_ws: *mut Con = ::core::ptr::null_mut::<Con>();
            let mut child: *mut Con = -(1 as ::core::ffi::c_int) as *mut Con;
            while child == -(1 as ::core::ffi::c_int) as *mut Con && {
                child = ::core::ptr::null_mut::<Con>();
                true_0 != 0
            } {
                child = (*output_get_content(output)).nodes_head.tqh_first as *mut Con;
                while !child.is_null() {
                    if !workspace_is_visible(child) {
                        child = (*child).nodes.tqe_next as *mut Con;
                    } else {
                        visible_ws = child;
                        break;
                    }
                }
            }
            if !visible_ws.is_null() && con_num_children(visible_ws) == 0 as ::core::ffi::c_int {
                drop_targets += 1;
            }
        }
        output = (*output).focused.tqe_next as *mut Con;
    }
    return drop_targets > 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn find_drop_target(mut x: uint32_t, mut y: uint32_t) -> *mut Con {
    let mut con: *mut Con = ::core::ptr::null_mut::<Con>();
    con = all_cons.tqh_first as *mut Con;
    while !con.is_null() {
        let mut rect: Rect = (*con).rect as Rect;
        if !rect_contains(rect, x, y) || !is_tiling_drop_target(con) {
            con = (*con).all_cons.tqe_next as *mut Con;
        } else {
            let mut ws: *mut Con = con_get_workspace(con);
            let mut fs: *mut Con = con_get_fullscreen_covering_ws(ws);
            return if !fs.is_null() { fs } else { con };
        }
    }
    let mut output: *mut Output =
        get_output_containing(x as ::core::ffi::c_uint, y as ::core::ffi::c_uint);
    if output.is_null() {
        return ::core::ptr::null_mut::<Con>();
    }
    let mut content: *mut Con = output_get_content((*output).con);
    return con_descend_tiling_focused(content);
}
unsafe extern "C" fn adjust_rect(
    mut rect: Rect,
    mut direction: direction_t,
    mut threshold: uint32_t,
) -> Rect {
    match direction as ::core::ffi::c_uint {
        0 => {
            rect.width = threshold;
        }
        2 => {
            rect.height = threshold;
        }
        1 => {
            rect.x = rect.x.wrapping_add(rect.width.wrapping_sub(threshold));
            rect.width = threshold;
        }
        3 => {
            rect.y = rect.y.wrapping_add(rect.height.wrapping_sub(threshold));
            rect.height = threshold;
        }
        _ => {}
    }
    return rect;
}
unsafe extern "C" fn con_on_side_of_parent(mut con: *mut Con, mut direction: direction_t) -> bool {
    let orientation: orientation_t = orientation_from_direction(direction) as orientation_t;
    let mut reverse_direction: direction_t = D_LEFT;
    match direction as ::core::ffi::c_uint {
        0 => {
            reverse_direction = D_RIGHT;
        }
        1 => {
            reverse_direction = D_LEFT;
        }
        2 => {
            reverse_direction = D_DOWN;
        }
        3 => {
            reverse_direction = D_UP;
        }
        _ => {}
    }
    return con_orientation((*con).parent as *mut Con) as ::core::ffi::c_uint
        != orientation as ::core::ffi::c_uint
        || (*(*con).parent).layout as ::core::ffi::c_uint
            == L_STACKED as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*(*con).parent).layout as ::core::ffi::c_uint
            == L_TABBED as ::core::ffi::c_int as ::core::ffi::c_uint
        || con_descend_direction((*con).parent as *mut Con, reverse_direction) == con;
}
unsafe extern "C" fn drag_callback(
    mut con: *mut Con,
    mut old_rect: *mut Rect,
    mut new_x: uint32_t,
    mut new_y: uint32_t,
    mut event: *const xcb_button_press_event_t,
    mut extra: *const ::core::ffi::c_void,
) {
    let mut min_rect_dimension: uint32_t = 0;
    let mut sibling_indicator_size: uint32_t = 0;
    let mut parent_indicator_size: uint32_t = 0;
    let mut d_left: uint32_t = 0;
    let mut d_top: uint32_t = 0;
    let mut d_right: uint32_t = 0;
    let mut d_bottom: uint32_t = 0;
    let mut d_min: uint32_t = 0;
    let mut target_parent: bool = false;
    let mut target_sibling: bool = false;
    let sibling_indicator_percent_of_rect: ::core::ffi::c_double = 0.3f64;
    let parent_indicator_max_size: ::core::ffi::c_double =
        (render_deco_height() + logical_px(5 as ::core::ffi::c_int)) as ::core::ffi::c_double;
    let mut target: *mut Con = find_drop_target(new_x, new_y);
    if target.is_null() {
        return;
    }
    let mut rect: Rect = (*target).rect as Rect;
    let mut direction: direction_t = D_LEFT;
    let mut drop_type: drop_type_t = DT_CENTER;
    let mut draw_window: bool = true_0 != 0;
    let mut params: *const callback_params = extra as *const callback_params;
    if !((*target).type_0 as ::core::ffi::c_uint
        == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint)
    {
        min_rect_dimension = min(
            rect.width as ::core::ffi::c_int,
            rect.height as ::core::ffi::c_int,
        ) as uint32_t;
        sibling_indicator_size = max(
            logical_px(2 as ::core::ffi::c_int),
            (sibling_indicator_percent_of_rect * min_rect_dimension as ::core::ffi::c_double)
                as uint32_t as ::core::ffi::c_int,
        ) as uint32_t;
        parent_indicator_size = min(
            parent_indicator_max_size as ::core::ffi::c_int,
            sibling_indicator_size.wrapping_sub(1 as uint32_t) as ::core::ffi::c_int,
        ) as uint32_t;
        d_left = new_x.wrapping_sub(rect.x);
        d_top = new_y.wrapping_sub(rect.y);
        d_right = rect.x.wrapping_add(rect.width).wrapping_sub(new_x);
        d_bottom = rect.y.wrapping_add(rect.height).wrapping_sub(new_y);
        d_min = min(
            min(d_left as ::core::ffi::c_int, d_right as ::core::ffi::c_int),
            min(d_top as ::core::ffi::c_int, d_bottom as ::core::ffi::c_int),
        ) as uint32_t;
        if d_left == d_min {
            direction = D_LEFT;
        } else if d_top == d_min {
            direction = D_UP;
        } else if d_right == d_min {
            direction = D_RIGHT;
        } else if d_bottom == d_min {
            direction = D_DOWN;
        } else {
            errorlog(
                b"ERROR: min() is broken\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
            if 0 as ::core::ffi::c_int != 0 {
            } else {
                __assert_fail(
                    b"false\0" as *const u8 as *const ::core::ffi::c_char,
                    b"../src/tiling_drag.c\0" as *const u8 as *const ::core::ffi::c_char,
                    212 as ::core::ffi::c_uint,
                    b"void drag_callback(Con *, Rect *, uint32_t, uint32_t, const xcb_button_press_event_t *, const void *)\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            };
        }
        target_parent = d_min < parent_indicator_size
            && con_on_side_of_parent(target, direction) as ::core::ffi::c_int != 0;
        target_sibling = d_min < sibling_indicator_size;
        drop_type = (if target_parent as ::core::ffi::c_int != 0 {
            DT_PARENT as ::core::ffi::c_int
        } else if target_sibling as ::core::ffi::c_int != 0 {
            DT_SIBLING as ::core::ffi::c_int
        } else {
            DT_CENTER as ::core::ffi::c_int
        }) as drop_type_t;
        if drop_type as ::core::ffi::c_uint
            != DT_PARENT as ::core::ffi::c_int as ::core::ffi::c_uint
            && target == con
        {
            draw_window = false_0 != 0;
            xcb_destroy_window(conn, *(*params).indicator);
            *(*params).indicator = 0 as xcb_window_t;
        } else {
            match drop_type as ::core::ffi::c_uint {
                2 => {
                    while (*(*target).parent).type_0 as ::core::ffi::c_uint
                        != CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
                        && con_on_side_of_parent((*target).parent as *mut Con, direction)
                            as ::core::ffi::c_int
                            != 0
                    {
                        target = (*target).parent as *mut Con;
                    }
                    rect = adjust_rect(
                        (*(*target).parent).rect as Rect,
                        direction,
                        parent_indicator_size,
                    );
                }
                1 => {
                    rect = (*target).rect as Rect;
                    rect.x = rect.x.wrapping_add(sibling_indicator_size);
                    rect.y = rect.y.wrapping_add(sibling_indicator_size);
                    rect.width = rect
                        .width
                        .wrapping_sub(sibling_indicator_size.wrapping_mul(2 as uint32_t));
                    rect.height = rect
                        .height
                        .wrapping_sub(sibling_indicator_size.wrapping_mul(2 as uint32_t));
                }
                0 => {
                    rect = adjust_rect((*target).rect as Rect, direction, sibling_indicator_size);
                }
                _ => {}
            }
        }
    }
    if draw_window {
        if *(*params).indicator == 0 as xcb_window_t {
            *(*params).indicator = create_drop_indicator(rect);
        } else {
            let values: [uint32_t; 4] = [rect.x, rect.y, rect.width, rect.height];
            let mask: uint32_t = (XCB_CONFIG_WINDOW_X as ::core::ffi::c_int
                | XCB_CONFIG_WINDOW_Y as ::core::ffi::c_int
                | XCB_CONFIG_WINDOW_WIDTH as ::core::ffi::c_int
                | XCB_CONFIG_WINDOW_HEIGHT as ::core::ffi::c_int)
                as uint32_t;
            xcb_configure_window(
                conn,
                *(*params).indicator,
                mask as uint16_t,
                &raw const values as *const uint32_t as *const ::core::ffi::c_void,
            );
        }
    }
    x_mask_event_mask(!(XCB_EVENT_MASK_ENTER_WINDOW as ::core::ffi::c_int) as uint32_t);
    xcb_flush(conn);
    *(*params).target = target;
    *(*params).direction = direction;
    *(*params).drop_type = drop_type;
}
unsafe extern "C" fn create_drop_indicator(mut rect: Rect) -> xcb_window_t {
    let mut mask: uint32_t = 0 as uint32_t;
    let mut values: [uint32_t; 2] = [0; 2];
    mask = XCB_CW_BACK_PIXEL as ::core::ffi::c_int as uint32_t;
    values[0 as ::core::ffi::c_int as usize] = config.client.focused.indicator.colorpixel;
    mask |= XCB_CW_OVERRIDE_REDIRECT as ::core::ffi::c_int as uint32_t;
    values[1 as ::core::ffi::c_int as usize] = 1 as uint32_t;
    let mut indicator: xcb_window_t = create_window(
        conn,
        rect,
        XCB_COPY_FROM_PARENT as uint16_t,
        XCB_COPY_FROM_PARENT as xcb_visualid_t,
        XCB_WINDOW_CLASS_INPUT_OUTPUT as ::core::ffi::c_int as uint16_t,
        XCURSOR_CURSOR_MOVE,
        false_0 != 0,
        mask,
        &raw mut values as *mut uint32_t,
    );
    xcb_change_property(
        conn,
        XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as uint8_t,
        indicator,
        XCB_ATOM_WM_CLASS as ::core::ffi::c_int as xcb_atom_t,
        XCB_ATOM_STRING as ::core::ffi::c_int as xcb_atom_t,
        8 as uint8_t,
        strlen(b"i3-drag\0" as *const u8 as *const ::core::ffi::c_char)
            .wrapping_add(1 as size_t)
            .wrapping_mul(2 as size_t) as uint32_t,
        b"i3-drag\0i3-drag\0\0" as *const u8 as *const ::core::ffi::c_char
            as *const ::core::ffi::c_void,
    );
    xcb_map_window(conn, indicator);
    xcb_circulate_window(
        conn,
        XCB_CIRCULATE_RAISE_LOWEST as ::core::ffi::c_int as uint8_t,
        indicator,
    );
    return indicator;
}
#[no_mangle]
pub unsafe extern "C" fn tiling_drag(
    mut con: *mut Con,
    mut event: *mut xcb_button_press_event_t,
    mut use_threshold: bool,
) {
    debuglog(
        b"%s:%s:%d - Start dragging tiled container: con = %p\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/tiling_drag.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"tiling_drag\0" as *const u8 as *const ::core::ffi::c_char,
        305 as ::core::ffi::c_int,
        con,
    );
    let mut set_focus: bool = con == focused;
    let mut set_fs: bool = (*con).fullscreen_mode as ::core::ffi::c_uint
        != CF_NONE as ::core::ffi::c_int as ::core::ffi::c_uint;
    x_mask_event_mask(!(XCB_EVENT_MASK_ENTER_WINDOW as ::core::ffi::c_int) as uint32_t);
    xcb_flush(conn);
    let mut target: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut direction: direction_t = D_LEFT;
    let mut drop_type: drop_type_t = DT_SIBLING;
    let mut indicator: xcb_window_t = 0 as xcb_window_t;
    let params: callback_params = callback_params {
        indicator: &raw mut indicator,
        target: &raw mut target,
        direction: &raw mut direction,
        drop_type: &raw mut drop_type,
    };
    let mut drag_result: drag_result_t = drag_pointer(
        con,
        event,
        XCB_NONE as xcb_window_t,
        XCURSOR_CURSOR_MOVE as ::core::ffi::c_int,
        use_threshold,
        Some(
            drag_callback
                as unsafe extern "C" fn(
                    *mut Con,
                    *mut Rect,
                    uint32_t,
                    uint32_t,
                    *const xcb_button_press_event_t,
                    *const ::core::ffi::c_void,
                ) -> (),
        ),
        &raw const params as *const ::core::ffi::c_void,
    );
    xcb_destroy_window(conn, indicator);
    if drag_result as ::core::ffi::c_uint
        == DRAG_REVERT as ::core::ffi::c_int as ::core::ffi::c_uint
        || target.is_null()
        || target == con
            && drop_type as ::core::ffi::c_uint
                != DT_PARENT as ::core::ffi::c_int as ::core::ffi::c_uint
        || !con_exists(target)
    {
        debuglog(
            b"%s:%s:%d - drop aborted\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b"../src/tiling_drag.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"tiling_drag\0" as *const u8 as *const ::core::ffi::c_char,
            329 as ::core::ffi::c_int,
        );
        return;
    }
    let orientation: orientation_t = orientation_from_direction(direction) as orientation_t;
    let position: position_t = position_from_direction(direction) as position_t;
    let layout: layout_t = (if orientation as ::core::ffi::c_uint
        == VERT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        L_SPLITV as ::core::ffi::c_int
    } else {
        L_SPLITH as ::core::ffi::c_int
    }) as layout_t;
    con_disable_fullscreen(con);
    let mut mod_0: uint32_t = 0;
    let mut swap_pressed: bool = false;
    match drop_type as ::core::ffi::c_uint {
        1 => {
            debuglog(
                b"%s:%s:%d - drop to center of %p\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"../src/tiling_drag.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"tiling_drag\0" as *const u8 as *const ::core::ffi::c_char,
                340 as ::core::ffi::c_int,
                target,
            );
            mod_0 = config.swap_modifier & 0xffff as uint32_t;
            swap_pressed = mod_0 != 0 as uint32_t && (*event).state as uint32_t & mod_0 == mod_0;
            if swap_pressed {
                if !con_swap(con, target) {
                    return;
                }
            } else {
                con_move_to_target(con, target);
            }
        }
        0 => {
            debuglog(
                b"%s:%s:%d - drop %s %p\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"../src/tiling_drag.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"tiling_drag\0" as *const u8 as *const ::core::ffi::c_char,
                352 as ::core::ffi::c_int,
                position_to_string(position),
                target,
            );
            if con_orientation((*target).parent as *mut Con) as ::core::ffi::c_uint
                != orientation as ::core::ffi::c_uint
            {
                if (*con).parent == (*target).parent
                    && con_num_children((*target).parent as *mut Con) == 2 as ::core::ffi::c_int
                {
                    (*(*target).parent).layout = layout;
                } else {
                    tree_split(target, orientation);
                }
            }
            insert_con_into(con, target, position);
            ipc_send_window_event(b"move\0" as *const u8 as *const ::core::ffi::c_char, con);
        }
        2 => {
            let parent_tabbed_or_stacked: bool = (*(*target).parent).layout as ::core::ffi::c_uint
                == L_TABBED as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*(*target).parent).layout as ::core::ffi::c_uint
                    == L_STACKED as ::core::ffi::c_int as ::core::ffi::c_uint;
            debuglog(
                b"%s:%s:%d - drop %s (%s) of %s%p\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"../src/tiling_drag.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"tiling_drag\0" as *const u8 as *const ::core::ffi::c_char,
                375 as ::core::ffi::c_int,
                direction_to_string(direction),
                position_to_string(position),
                if parent_tabbed_or_stacked as ::core::ffi::c_int != 0 {
                    b"tabbed/stacked \0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"\0" as *const u8 as *const ::core::ffi::c_char
                },
                target,
            );
            if parent_tabbed_or_stacked {
                if position as ::core::ffi::c_uint
                    == BEFORE as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    target = (*(*target).parent).nodes_head.tqh_first as *mut Con;
                } else {
                    target = *(*((*(*target).parent).nodes_head.tqh_last as *mut nodes_head))
                        .tqh_last as *mut Con;
                }
            }
            if con != target {
                insert_con_into(con, target, position);
            }
            let mut old_focus: *mut Con = focused;
            tree_move(con, direction);
            if focused != old_focus && con_exists(old_focus) as ::core::ffi::c_int != 0 {
                con_activate(old_focus);
            }
        }
        _ => {}
    }
    target = ::core::ptr::null_mut::<Con>();
    if set_focus as ::core::ffi::c_int != 0 || set_fs as ::core::ffi::c_int != 0 {
        let mut fs: *mut Con = con_get_fullscreen_covering_ws(con_get_workspace(con));
        if fs == con {
            errorlog(
                b"ERROR: dragged container somehow got fullscreen again.\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
            if 0 as ::core::ffi::c_int != 0 {
            } else {
                __assert_fail(
                    b"false\0" as *const u8 as *const ::core::ffi::c_char,
                    b"../src/tiling_drag.c\0" as *const u8 as *const ::core::ffi::c_char,
                    406 as ::core::ffi::c_uint,
                    b"void tiling_drag(Con *, xcb_button_press_event_t *, _Bool)\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            };
        } else if !fs.is_null()
            && set_focus as ::core::ffi::c_int != 0
            && set_fs as ::core::ffi::c_int != 0
        {
            con_disable_fullscreen(fs);
        } else if !fs.is_null() {
            set_focus = false_0 != 0;
            set_fs = set_focus;
        } else if !set_focus {
            set_focus = set_fs as ::core::ffi::c_int != 0
                && con_get_workspace(focused) == con_get_workspace(con);
        }
    }
    if set_fs {
        con_enable_fullscreen(con, CF_OUTPUT);
    }
    if set_focus {
        workspace_show(con_get_workspace(con));
        con_focus(con);
    }
    tree_render();
}
