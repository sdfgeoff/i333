/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * ewmh.c: Get/set certain EWMH properties easily.
 *
 */
// Mechanical file-by-file Rust translation of src/ewmh.c.
// Generated with C2Rust 0.22.1; C ABI and original ownership retained.
extern "C" {
    pub type xcb_connection_t;
    pub type _cairo;
    pub type _cairo_surface;
    pub type _i3String;
    pub type pcre2_real_code_8;
    pub type ev_loop;
    fn xcb_create_window(
        c: *mut xcb_connection_t,
        depth: uint8_t,
        wid: xcb_window_t,
        parent: xcb_window_t,
        x: int16_t,
        y: int16_t,
        width: uint16_t,
        height: uint16_t,
        border_width: uint16_t,
        _class: uint16_t,
        visual: xcb_visualid_t,
        value_mask: uint32_t,
        value_list: *const ::core::ffi::c_void,
    ) -> xcb_void_cookie_t;
    fn xcb_map_window(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t;
    fn xcb_configure_window(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        value_mask: uint16_t,
        value_list: *const ::core::ffi::c_void,
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
    fn xcb_delete_property(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        property: xcb_atom_t,
    ) -> xcb_void_cookie_t;
    fn xcb_generate_id(c: *mut xcb_connection_t) -> uint32_t;
    static mut conn: *mut xcb_connection_t;
    fn errorlog(fmt: *mut ::core::ffi::c_char, ...);
    fn debuglog(fmt: *mut ::core::ffi::c_char, ...);
    static mut croot: *mut Con;
    static mut focused: *mut Con;
    static mut root: xcb_window_t;
    static mut A__NET_WM_WINDOW_TYPE_DIALOG: xcb_atom_t;
    static mut A__NET_WM_STATE_STICKY: xcb_atom_t;
    static mut A__NET_WM_STATE_FULLSCREEN: xcb_atom_t;
    static mut A__NET_WM_STATE_DEMANDS_ATTENTION: xcb_atom_t;
    static mut A__NET_WM_STATE_MODAL: xcb_atom_t;
    static mut A__NET_WM_STATE_HIDDEN: xcb_atom_t;
    static mut A__NET_WM_STATE_FOCUSED: xcb_atom_t;
    static mut A__NET_WM_STATE_MAXIMIZED_VERT: xcb_atom_t;
    static mut A__NET_WM_STATE_MAXIMIZED_HORZ: xcb_atom_t;
    static mut A__NET_WM_STATE: xcb_atom_t;
    static mut A__NET_WM_WINDOW_TYPE: xcb_atom_t;
    static mut A__NET_WM_WINDOW_TYPE_NORMAL: xcb_atom_t;
    static mut A__NET_WM_WINDOW_TYPE_DOCK: xcb_atom_t;
    static mut A__NET_SUPPORTED: xcb_atom_t;
    static mut A__NET_WM_WINDOW_TYPE_UTILITY: xcb_atom_t;
    static mut A__NET_WM_WINDOW_TYPE_TOOLBAR: xcb_atom_t;
    static mut A__NET_WM_WINDOW_TYPE_SPLASH: xcb_atom_t;
    static mut A__NET_WM_WINDOW_TYPE_MENU: xcb_atom_t;
    static mut A__NET_WM_WINDOW_TYPE_DROPDOWN_MENU: xcb_atom_t;
    static mut A__NET_WM_WINDOW_TYPE_POPUP_MENU: xcb_atom_t;
    static mut A__NET_WM_WINDOW_TYPE_TOOLTIP: xcb_atom_t;
    static mut A__NET_WM_WINDOW_TYPE_NOTIFICATION: xcb_atom_t;
    static mut A__NET_WM_DESKTOP: xcb_atom_t;
    static mut A__NET_WM_STRUT_PARTIAL: xcb_atom_t;
    static mut A__NET_CLIENT_LIST: xcb_atom_t;
    static mut A__NET_CLIENT_LIST_STACKING: xcb_atom_t;
    static mut A__NET_CURRENT_DESKTOP: xcb_atom_t;
    static mut A__NET_NUMBER_OF_DESKTOPS: xcb_atom_t;
    static mut A__NET_DESKTOP_NAMES: xcb_atom_t;
    static mut A__NET_DESKTOP_VIEWPORT: xcb_atom_t;
    static mut A__NET_ACTIVE_WINDOW: xcb_atom_t;
    static mut A__NET_CLOSE_WINDOW: xcb_atom_t;
    static mut A__NET_MOVERESIZE_WINDOW: xcb_atom_t;
    static mut A__NET_SUPPORTING_WM_CHECK: xcb_atom_t;
    static mut A__NET_WM_NAME: xcb_atom_t;
    static mut A__NET_WM_VISIBLE_NAME: xcb_atom_t;
    static mut A__NET_WM_MOVERESIZE: xcb_atom_t;
    static mut A__NET_WORKAREA: xcb_atom_t;
    static mut A_UTF8_STRING: xcb_atom_t;
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
    fn con_has_managed_window(con: *mut Con) -> bool;
    fn con_is_sticky(con: *mut Con) -> bool;
    fn con_get_workspace(con: *mut Con) -> *mut Con;
    fn con_is_internal(con: *mut Con) -> bool;
    fn con_is_floating(con: *mut Con) -> bool;
    fn output_get_content(output: *mut Con) -> *mut Con;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __time_t = ::core::ffi::c_long;
pub type __suseconds_t = ::core::ffi::c_long;
pub type time_t = __time_t;
pub type int16_t = __int16_t;
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
pub type xcb_gcontext_t = uint32_t;
pub type xcb_colormap_t = uint32_t;
pub type xcb_atom_t = uint32_t;
pub type xcb_drawable_t = uint32_t;
pub type xcb_visualid_t = uint32_t;
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
pub type xcb_stack_mode_t = ::core::ffi::c_uint;
pub const XCB_STACK_MODE_OPPOSITE: xcb_stack_mode_t = 4;
pub const XCB_STACK_MODE_BOTTOM_IF: xcb_stack_mode_t = 3;
pub const XCB_STACK_MODE_TOP_IF: xcb_stack_mode_t = 2;
pub const XCB_STACK_MODE_BELOW: xcb_stack_mode_t = 1;
pub const XCB_STACK_MODE_ABOVE: xcb_stack_mode_t = 0;
pub type xcb_prop_mode_t = ::core::ffi::c_uint;
pub const XCB_PROP_MODE_APPEND: xcb_prop_mode_t = 2;
pub const XCB_PROP_MODE_PREPEND: xcb_prop_mode_t = 1;
pub const XCB_PROP_MODE_REPLACE: xcb_prop_mode_t = 0;
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
pub const XCB_COPY_FROM_PARENT: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NET_WM_DESKTOP_NONE: ::core::ffi::c_uint = 0xfffffff0 as ::core::ffi::c_uint;
pub const NET_WM_DESKTOP_ALL: ::core::ffi::c_uint = 0xffffffff as ::core::ffi::c_uint;
#[no_mangle]
pub static mut ewmh_window: xcb_window_t = 0;
#[no_mangle]
pub unsafe extern "C" fn ewmh_update_current_desktop() {
    static mut old_idx: uint32_t = NET_WM_DESKTOP_NONE as uint32_t;
    let idx: uint32_t = ewmh_get_workspace_index(focused) as uint32_t;
    if idx == old_idx || idx == NET_WM_DESKTOP_NONE as uint32_t {
        return;
    }
    old_idx = idx;
    xcb_change_property(
        conn,
        XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as uint8_t,
        root,
        A__NET_CURRENT_DESKTOP,
        XCB_ATOM_CARDINAL as ::core::ffi::c_int as xcb_atom_t,
        32 as uint8_t,
        1 as uint32_t,
        &raw const idx as *const ::core::ffi::c_void,
    );
}
unsafe extern "C" fn ewmh_update_number_of_desktops() {
    let mut output: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut ws: *mut Con = ::core::ptr::null_mut::<Con>();
    static mut old_idx: uint32_t = 0 as uint32_t;
    let mut idx: uint32_t = 0 as uint32_t;
    output = (*croot).nodes_head.tqh_first as *mut Con;
    while !output.is_null() {
        ws = (*output_get_content(output)).nodes_head.tqh_first as *mut Con;
        while !ws.is_null() {
            if !con_is_internal(ws) {
                idx = idx.wrapping_add(1);
            }
            ws = (*ws).nodes.tqe_next as *mut Con;
        }
        output = (*output).nodes.tqe_next as *mut Con;
    }
    if idx == old_idx {
        return;
    }
    old_idx = idx;
    xcb_change_property(
        conn,
        XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as uint8_t,
        root,
        A__NET_NUMBER_OF_DESKTOPS,
        XCB_ATOM_CARDINAL as ::core::ffi::c_int as xcb_atom_t,
        32 as uint8_t,
        1 as uint32_t,
        &raw mut idx as *const ::core::ffi::c_void,
    );
}
unsafe extern "C" fn ewmh_update_desktop_names() {
    let mut output: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut ws: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut msg_length: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    output = (*croot).nodes_head.tqh_first as *mut Con;
    while !output.is_null() {
        ws = (*output_get_content(output)).nodes_head.tqh_first as *mut Con;
        while !ws.is_null() {
            if !con_is_internal(ws) {
                msg_length = (msg_length as size_t)
                    .wrapping_add(strlen((*ws).name).wrapping_add(1 as size_t))
                    as ::core::ffi::c_int as ::core::ffi::c_int;
            }
            ws = (*ws).nodes.tqe_next as *mut Con;
        }
        output = (*output).nodes.tqe_next as *mut Con;
    }
    let vla = msg_length as usize;
    let mut desktop_names: Vec<::core::ffi::c_char> = ::std::vec::from_elem(0, vla);
    let mut current_position: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    output = (*croot).nodes_head.tqh_first as *mut Con;
    while !output.is_null() {
        ws = (*output_get_content(output)).nodes_head.tqh_first as *mut Con;
        while !ws.is_null() {
            if !con_is_internal(ws) {
                let mut i: size_t = 0 as size_t;
                while i < strlen((*ws).name).wrapping_add(1 as size_t) {
                    let fresh0 = current_position;
                    current_position = current_position + 1;
                    *desktop_names.as_mut_ptr().offset(fresh0 as isize) =
                        *(*ws).name.offset(i as isize);
                    i = i.wrapping_add(1);
                }
            }
            ws = (*ws).nodes.tqe_next as *mut Con;
        }
        output = (*output).nodes.tqe_next as *mut Con;
    }
    xcb_change_property(
        conn,
        XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as uint8_t,
        root,
        A__NET_DESKTOP_NAMES,
        A_UTF8_STRING,
        8 as uint8_t,
        msg_length as uint32_t,
        desktop_names.as_mut_ptr() as *const ::core::ffi::c_void,
    );
}
unsafe extern "C" fn ewmh_update_desktop_viewport() {
    let mut output: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut ws: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut num_desktops: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    output = (*croot).nodes_head.tqh_first as *mut Con;
    while !output.is_null() {
        ws = (*output_get_content(output)).nodes_head.tqh_first as *mut Con;
        while !ws.is_null() {
            if !con_is_internal(ws) {
                num_desktops += 1;
            }
            ws = (*ws).nodes.tqe_next as *mut Con;
        }
        output = (*output).nodes.tqe_next as *mut Con;
    }
    let vla = (num_desktops * 2 as ::core::ffi::c_int) as usize;
    let mut viewports: Vec<uint32_t> = ::std::vec::from_elem(0, vla);
    let mut current_position: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    output = (*croot).nodes_head.tqh_first as *mut Con;
    while !output.is_null() {
        ws = (*output_get_content(output)).nodes_head.tqh_first as *mut Con;
        while !ws.is_null() {
            if !con_is_internal(ws) {
                let fresh1 = current_position;
                current_position = current_position + 1;
                *viewports.as_mut_ptr().offset(fresh1 as isize) = (*output).rect.x;
                let fresh2 = current_position;
                current_position = current_position + 1;
                *viewports.as_mut_ptr().offset(fresh2 as isize) = (*output).rect.y;
            }
            ws = (*ws).nodes.tqe_next as *mut Con;
        }
        output = (*output).nodes.tqe_next as *mut Con;
    }
    xcb_change_property(
        conn,
        XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as uint8_t,
        root,
        A__NET_DESKTOP_VIEWPORT,
        XCB_ATOM_CARDINAL as ::core::ffi::c_int as xcb_atom_t,
        32 as uint8_t,
        current_position as uint32_t,
        viewports.as_mut_ptr() as *const ::core::ffi::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ewmh_update_desktop_properties() {
    ewmh_update_number_of_desktops();
    ewmh_update_desktop_viewport();
    ewmh_update_current_desktop();
    ewmh_update_desktop_names();
    ewmh_update_wm_desktop();
}
unsafe extern "C" fn ewmh_update_wm_desktop_recursively(mut con: *mut Con, desktop: uint32_t) {
    let mut child: *mut Con = ::core::ptr::null_mut::<Con>();
    child = (*con).nodes_head.tqh_first as *mut Con;
    while !child.is_null() {
        ewmh_update_wm_desktop_recursively(child, desktop);
        child = (*child).nodes.tqe_next as *mut Con;
    }
    if (*con).type_0 as ::core::ffi::c_uint
        == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        child = (*con).floating_head.tqh_first as *mut Con;
        while !child.is_null() {
            ewmh_update_wm_desktop_recursively(child, desktop);
            child = (*child).floating_windows.tqe_next as *mut Con;
        }
    }
    if !con_has_managed_window(con) {
        return;
    }
    let mut wm_desktop: uint32_t = desktop;
    if con_is_sticky(con) as ::core::ffi::c_int != 0
        && con_is_floating(con) as ::core::ffi::c_int != 0
    {
        wm_desktop = NET_WM_DESKTOP_ALL as uint32_t;
    }
    let mut ws: *mut Con = con_get_workspace(con);
    if !ws.is_null() && con_is_internal(ws) as ::core::ffi::c_int != 0 {
        wm_desktop = NET_WM_DESKTOP_ALL as uint32_t;
    }
    if (*(*con).window).wm_desktop == wm_desktop {
        return;
    }
    (*(*con).window).wm_desktop = wm_desktop;
    let window: xcb_window_t = (*(*con).window).id;
    if wm_desktop != NET_WM_DESKTOP_NONE as uint32_t {
        debuglog(
            b"%s:%s:%d - Setting _NET_WM_DESKTOP = %d for window 0x%08x.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/ewmh.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"ewmh_update_wm_desktop_recursively\0" as *const u8 as *const ::core::ffi::c_char,
            171 as ::core::ffi::c_int,
            wm_desktop,
            window,
        );
        xcb_change_property(
            conn,
            XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as uint8_t,
            window,
            A__NET_WM_DESKTOP,
            XCB_ATOM_CARDINAL as ::core::ffi::c_int as xcb_atom_t,
            32 as uint8_t,
            1 as uint32_t,
            &raw mut wm_desktop as *const ::core::ffi::c_void,
        );
    } else {
        errorlog(
            b"ERROR: Failed to determine the proper EWMH desktop index for window 0x%08x, deleting _NET_WM_DESKTOP.\n\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            window,
        );
        xcb_delete_property(conn, window, A__NET_WM_DESKTOP);
    };
}
#[no_mangle]
pub unsafe extern "C" fn ewmh_update_wm_desktop() {
    let mut desktop: uint32_t = 0 as uint32_t;
    let mut output: *mut Con = ::core::ptr::null_mut::<Con>();
    output = (*croot).nodes_head.tqh_first as *mut Con;
    while !output.is_null() {
        let mut workspace: *mut Con = ::core::ptr::null_mut::<Con>();
        workspace = (*output_get_content(output)).nodes_head.tqh_first as *mut Con;
        while !workspace.is_null() {
            ewmh_update_wm_desktop_recursively(workspace, desktop);
            if !con_is_internal(workspace) {
                desktop = desktop.wrapping_add(1);
            }
            workspace = (*workspace).nodes.tqe_next as *mut Con;
        }
        output = (*output).nodes.tqe_next as *mut Con;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ewmh_update_active_window(mut window: xcb_window_t) {
    xcb_change_property(
        conn,
        XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as uint8_t,
        root,
        A__NET_ACTIVE_WINDOW,
        XCB_ATOM_WINDOW as ::core::ffi::c_int as xcb_atom_t,
        32 as uint8_t,
        1 as uint32_t,
        &raw mut window as *const ::core::ffi::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ewmh_update_visible_name(
    mut window: xcb_window_t,
    mut name: *const ::core::ffi::c_char,
) {
    if !name.is_null() {
        xcb_change_property(
            conn,
            XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as uint8_t,
            window,
            A__NET_WM_VISIBLE_NAME,
            A_UTF8_STRING,
            8 as uint8_t,
            strlen(name) as uint32_t,
            name as *const ::core::ffi::c_void,
        );
    } else {
        xcb_delete_property(conn, window, A__NET_WM_VISIBLE_NAME);
    };
}
#[no_mangle]
pub unsafe extern "C" fn ewmh_update_workarea() {
    xcb_delete_property(conn, root, A__NET_WORKAREA);
}
#[no_mangle]
pub unsafe extern "C" fn ewmh_update_client_list(
    mut list: *mut xcb_window_t,
    mut num_windows: ::core::ffi::c_int,
) {
    xcb_change_property(
        conn,
        XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as uint8_t,
        root,
        A__NET_CLIENT_LIST,
        XCB_ATOM_WINDOW as ::core::ffi::c_int as xcb_atom_t,
        32 as uint8_t,
        num_windows as uint32_t,
        list as *const ::core::ffi::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ewmh_update_client_list_stacking(
    mut stack: *mut xcb_window_t,
    mut num_windows: ::core::ffi::c_int,
) {
    xcb_change_property(
        conn,
        XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as uint8_t,
        root,
        A__NET_CLIENT_LIST_STACKING,
        XCB_ATOM_WINDOW as ::core::ffi::c_int as xcb_atom_t,
        32 as uint8_t,
        num_windows as uint32_t,
        stack as *const ::core::ffi::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ewmh_update_sticky(mut window: xcb_window_t, mut sticky: bool) {
    if sticky {
        debuglog(
            b"%s:%s:%d - Setting _NET_WM_STATE_STICKY for window = %08x.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/ewmh.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"ewmh_update_sticky\0" as *const u8 as *const ::core::ffi::c_char,
            283 as ::core::ffi::c_int,
            window,
        );
        xcb_add_property_atom(conn, window, A__NET_WM_STATE, A__NET_WM_STATE_STICKY);
    } else {
        debuglog(
            b"%s:%s:%d - Removing _NET_WM_STATE_STICKY for window = %08x.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/ewmh.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"ewmh_update_sticky\0" as *const u8 as *const ::core::ffi::c_char,
            286 as ::core::ffi::c_int,
            window,
        );
        xcb_remove_property_atom(conn, window, A__NET_WM_STATE, A__NET_WM_STATE_STICKY);
    };
}
#[no_mangle]
pub unsafe extern "C" fn ewmh_update_focused(mut window: xcb_window_t, mut is_focused: bool) {
    if is_focused {
        debuglog(
            b"%s:%s:%d - Setting _NET_WM_STATE_FOCUSED for window = %08x.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/ewmh.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"ewmh_update_focused\0" as *const u8 as *const ::core::ffi::c_char,
            297 as ::core::ffi::c_int,
            window,
        );
        xcb_add_property_atom(conn, window, A__NET_WM_STATE, A__NET_WM_STATE_FOCUSED);
    } else {
        debuglog(
            b"%s:%s:%d - Removing _NET_WM_STATE_FOCUSED for window = %08x.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/ewmh.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"ewmh_update_focused\0" as *const u8 as *const ::core::ffi::c_char,
            300 as ::core::ffi::c_int,
            window,
        );
        xcb_remove_property_atom(conn, window, A__NET_WM_STATE, A__NET_WM_STATE_FOCUSED);
    };
}
#[no_mangle]
pub unsafe extern "C" fn ewmh_setup_hints() {
    let mut supported_atoms: [xcb_atom_t; 37] = [
        A__NET_SUPPORTED,
        A__NET_SUPPORTING_WM_CHECK,
        A__NET_WM_NAME,
        A__NET_WM_VISIBLE_NAME,
        A__NET_WM_MOVERESIZE,
        A__NET_WM_STATE_STICKY,
        A__NET_WM_STATE_FULLSCREEN,
        A__NET_WM_STATE_DEMANDS_ATTENTION,
        A__NET_WM_STATE_MODAL,
        A__NET_WM_STATE_HIDDEN,
        A__NET_WM_STATE_FOCUSED,
        A__NET_WM_STATE_MAXIMIZED_VERT,
        A__NET_WM_STATE_MAXIMIZED_HORZ,
        A__NET_WM_STATE,
        A__NET_WM_WINDOW_TYPE,
        A__NET_WM_WINDOW_TYPE_NORMAL,
        A__NET_WM_WINDOW_TYPE_DOCK,
        A__NET_WM_WINDOW_TYPE_DIALOG,
        A__NET_WM_WINDOW_TYPE_UTILITY,
        A__NET_WM_WINDOW_TYPE_TOOLBAR,
        A__NET_WM_WINDOW_TYPE_SPLASH,
        A__NET_WM_WINDOW_TYPE_MENU,
        A__NET_WM_WINDOW_TYPE_DROPDOWN_MENU,
        A__NET_WM_WINDOW_TYPE_POPUP_MENU,
        A__NET_WM_WINDOW_TYPE_TOOLTIP,
        A__NET_WM_WINDOW_TYPE_NOTIFICATION,
        A__NET_WM_DESKTOP,
        A__NET_WM_STRUT_PARTIAL,
        A__NET_CLIENT_LIST,
        A__NET_CLIENT_LIST_STACKING,
        A__NET_CURRENT_DESKTOP,
        A__NET_NUMBER_OF_DESKTOPS,
        A__NET_DESKTOP_NAMES,
        A__NET_DESKTOP_VIEWPORT,
        A__NET_ACTIVE_WINDOW,
        A__NET_CLOSE_WINDOW,
        A__NET_MOVERESIZE_WINDOW,
    ];
    ewmh_window = xcb_generate_id(conn) as xcb_window_t;
    let mut fresh3: [uint32_t; 1] = [1 as ::core::ffi::c_int as uint32_t];
    xcb_create_window(
        conn,
        XCB_COPY_FROM_PARENT as uint8_t,
        ewmh_window,
        root,
        -(1 as ::core::ffi::c_int) as int16_t,
        -(1 as ::core::ffi::c_int) as int16_t,
        1 as uint16_t,
        1 as uint16_t,
        0 as uint16_t,
        XCB_WINDOW_CLASS_INPUT_ONLY as ::core::ffi::c_int as uint16_t,
        XCB_COPY_FROM_PARENT as xcb_visualid_t,
        XCB_CW_OVERRIDE_REDIRECT as ::core::ffi::c_int as uint32_t,
        &raw mut fresh3 as *mut uint32_t as *const ::core::ffi::c_void,
    );
    xcb_change_property(
        conn,
        XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as uint8_t,
        ewmh_window,
        A__NET_SUPPORTING_WM_CHECK,
        XCB_ATOM_WINDOW as ::core::ffi::c_int as xcb_atom_t,
        32 as uint8_t,
        1 as uint32_t,
        &raw mut ewmh_window as *const ::core::ffi::c_void,
    );
    xcb_change_property(
        conn,
        XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as uint8_t,
        ewmh_window,
        A__NET_WM_NAME,
        A_UTF8_STRING,
        8 as uint8_t,
        strlen(b"i3\0" as *const u8 as *const ::core::ffi::c_char) as uint32_t,
        b"i3\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
    );
    xcb_change_property(
        conn,
        XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as uint8_t,
        root,
        A__NET_SUPPORTING_WM_CHECK,
        XCB_ATOM_WINDOW as ::core::ffi::c_int as xcb_atom_t,
        32 as uint8_t,
        1 as uint32_t,
        &raw mut ewmh_window as *const ::core::ffi::c_void,
    );
    xcb_change_property(
        conn,
        XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as uint8_t,
        root,
        A__NET_WM_NAME,
        A_UTF8_STRING,
        8 as uint8_t,
        strlen(b"i3\0" as *const u8 as *const ::core::ffi::c_char) as uint32_t,
        b"i3\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
    );
    xcb_change_property(
        conn,
        XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as uint8_t,
        root,
        A__NET_SUPPORTED,
        XCB_ATOM_ATOM as ::core::ffi::c_int as xcb_atom_t,
        32 as uint8_t,
        (::core::mem::size_of::<[xcb_atom_t; 37]>() as usize)
            .wrapping_div(::core::mem::size_of::<xcb_atom_t>() as usize) as uint32_t,
        &raw mut supported_atoms as *mut xcb_atom_t as *const ::core::ffi::c_void,
    );
    xcb_map_window(conn, ewmh_window);
    let mut fresh4: [uint32_t; 1] = [XCB_STACK_MODE_BELOW as ::core::ffi::c_int as uint32_t];
    xcb_configure_window(
        conn,
        ewmh_window,
        XCB_CONFIG_WINDOW_STACK_MODE as ::core::ffi::c_int as uint16_t,
        &raw mut fresh4 as *mut uint32_t as *const ::core::ffi::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ewmh_get_workspace_by_index(mut idx: uint32_t) -> *mut Con {
    if idx == NET_WM_DESKTOP_NONE as uint32_t {
        return ::core::ptr::null_mut::<Con>();
    }
    let mut current_index: uint32_t = 0 as uint32_t;
    let mut output: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut ws: *mut Con = ::core::ptr::null_mut::<Con>();
    output = (*croot).nodes_head.tqh_first as *mut Con;
    while !output.is_null() {
        ws = (*output_get_content(output)).nodes_head.tqh_first as *mut Con;
        while !ws.is_null() {
            if !con_is_internal(ws) {
                if current_index == idx {
                    return ws;
                }
                current_index = current_index.wrapping_add(1);
            }
            ws = (*ws).nodes.tqe_next as *mut Con;
        }
        output = (*output).nodes.tqe_next as *mut Con;
    }
    return ::core::ptr::null_mut::<Con>();
}
#[no_mangle]
pub unsafe extern "C" fn ewmh_get_workspace_index(mut con: *mut Con) -> uint32_t {
    let mut index: uint32_t = 0 as uint32_t;
    let mut target_workspace: *mut Con = con_get_workspace(con);
    let mut output: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut ws: *mut Con = ::core::ptr::null_mut::<Con>();
    output = (*croot).nodes_head.tqh_first as *mut Con;
    while !output.is_null() {
        ws = (*output_get_content(output)).nodes_head.tqh_first as *mut Con;
        while !ws.is_null() {
            if !con_is_internal(ws) {
                if ws == target_workspace {
                    return index;
                }
                index = index.wrapping_add(1);
            }
            ws = (*ws).nodes.tqe_next as *mut Con;
        }
        output = (*output).nodes.tqe_next as *mut Con;
    }
    return NET_WM_DESKTOP_NONE as uint32_t;
}
