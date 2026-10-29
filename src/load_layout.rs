/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * load_layout.c: Restore (parts of) the layout, for example after an inplace
 *                restart.
 *
 */
// Mechanical file-by-file Rust translation of src/load_layout.c.
// Generated with C2Rust 0.22.1; C ABI and original ownership retained.
extern "C" {
    pub type _cairo;
    pub type _cairo_surface;
    pub type _i3String;
    pub type pcre2_real_code_8;
    pub type ev_loop;
    pub type yajl_handle_t;
    fn verboselog(fmt: *mut ::core::ffi::c_char, ...);
    fn errorlog(fmt: *mut ::core::ffi::c_char, ...);
    fn debuglog(fmt: *mut ::core::ffi::c_char, ...);
    fn smalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn scalloc(num: size_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn srealloc(ptr: *mut ::core::ffi::c_void, size: size_t) -> *mut ::core::ffi::c_void;
    fn sstrdup(str: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn sstrndup(str: *const ::core::ffi::c_char, size: size_t) -> *mut ::core::ffi::c_char;
    fn sasprintf(
        strp: *mut *mut ::core::ffi::c_char,
        fmt: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn max(a: ::core::ffi::c_int, b: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn rect_equals(a: Rect, b: Rect) -> bool;
    fn ws_name_to_number(name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn yajl_alloc(
        callbacks: *const yajl_callbacks,
        afs: *mut yajl_alloc_funcs,
        ctx: *mut ::core::ffi::c_void,
    ) -> yajl_handle;
    fn yajl_config(h: yajl_handle, opt: yajl_option, ...) -> ::core::ffi::c_int;
    fn yajl_free(handle: yajl_handle);
    fn yajl_parse(
        hand: yajl_handle,
        jsonText: *const ::core::ffi::c_uchar,
        jsonTextLength: size_t,
    ) -> yajl_status;
    fn yajl_complete_parse(hand: yajl_handle) -> yajl_status;
    fn yajl_get_error(
        hand: yajl_handle,
        verbose: ::core::ffi::c_int,
        jsonText: *const ::core::ffi::c_uchar,
        jsonTextLength: size_t,
    ) -> *mut ::core::ffi::c_uchar;
    fn yajl_free_error(hand: yajl_handle, str: *mut ::core::ffi::c_uchar);
    static mut previous_workspace_name: *mut ::core::ffi::c_char;
    fn get_existing_workspace_by_name(name: *const ::core::ffi::c_char) -> *mut Con;
    fn x_con_init(con: *mut Con);
    fn floating_enable(con: *mut Con, automatic: bool) -> bool;
    fn floating_check_size(floating_con: *mut Con, prefer_height: bool);
    fn con_new_skeleton(parent: *mut Con, window: *mut i3Window) -> *mut Con;
    fn con_free(con: *mut Con);
    fn con_activate(con: *mut Con);
    fn con_is_split(con: *mut Con) -> bool;
    fn con_get_workspace(con: *mut Con) -> *mut Con;
    fn con_mark(con: *mut Con, mark: *const ::core::ffi::c_char, mode: mark_mode_t);
    fn con_attach(con: *mut Con, parent: *mut Con, ignore_focus: bool);
    fn con_fix_percent(con: *mut Con);
    fn match_init(match_0: *mut Match);
    fn match_free(match_0: *mut Match);
    fn regex_new(pattern: *const ::core::ffi::c_char) -> *mut regex;
    fn setlocale(
        __category: ::core::ffi::c_int,
        __locale: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strcasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strncasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __time_t = ::core::ffi::c_long;
pub type __suseconds_t = ::core::ffi::c_long;
pub type time_t = __time_t;
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
pub type xcb_gcontext_t = uint32_t;
pub type xcb_colormap_t = uint32_t;
pub type xcb_atom_t = uint32_t;
pub type xcb_drawable_t = uint32_t;
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
pub type i3Window = Window;
pub type mark_mode_t = ::core::ffi::c_uint;
pub const MM_ADD: mark_mode_t = 1;
pub const MM_REPLACE: mark_mode_t = 0;
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
pub type yajl_option = ::core::ffi::c_uint;
pub const yajl_allow_partial_values: yajl_option = 16;
pub const yajl_allow_multiple_values: yajl_option = 8;
pub const yajl_allow_trailing_garbage: yajl_option = 4;
pub const yajl_dont_validate_strings: yajl_option = 2;
pub const yajl_allow_comments: yajl_option = 1;
pub type json_content_t = ::core::ffi::c_uint;
pub const JSON_CONTENT_WORKSPACE: json_content_t = 2;
pub const JSON_CONTENT_CON: json_content_t = 1;
pub const JSON_CONTENT_UNKNOWN: json_content_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct focus_mapping {
    pub old_id: ::core::ffi::c_int,
    pub focus_mappings: C2RustUnnamed_16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_16 {
    pub tqe_next: *mut focus_mapping,
    pub tqe_prev: *mut *mut focus_mapping,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct focus_mappings_head {
    pub tqh_first: *mut focus_mapping,
    pub tqh_last: *mut *mut focus_mapping,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pending_marks {
    pub mark: *mut ::core::ffi::c_char,
    pub con_to_be_marked: *mut Con,
}
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const __LC_NUMERIC: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const LC_NUMERIC: ::core::ffi::c_int = __LC_NUMERIC;
static mut last_key: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
static mut incomplete: ::core::ffi::c_int = 0;
static mut json_node: *mut Con = ::core::ptr::null::<Con>() as *mut Con;
static mut to_focus: *mut Con = ::core::ptr::null::<Con>() as *mut Con;
static mut parsing_gaps: bool = false;
static mut parsing_swallows: bool = false;
static mut parsing_rect: bool = false;
static mut parsing_actual_deco_rect: bool = false;
static mut parsing_deco_rect: bool = false;
static mut parsing_window_rect: bool = false;
static mut parsing_geometry: bool = false;
static mut parsing_focus: bool = false;
static mut parsing_marks: bool = false;
#[no_mangle]
pub static mut current_swallow: *mut Match = ::core::ptr::null::<Match>() as *mut Match;
static mut swallow_is_empty: bool = false;
static mut num_marks: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut marks: *mut pending_marks =
    ::core::ptr::null::<pending_marks>() as *mut pending_marks;
static mut focus_mappings: focus_mappings_head = focus_mappings_head {
    tqh_first: ::core::ptr::null::<focus_mapping>() as *mut focus_mapping,
    tqh_last: ::core::ptr::null::<*mut focus_mapping>() as *mut *mut focus_mapping,
};
unsafe extern "C" fn json_start_map(mut ctx: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    verboselog(
        b"start of map, last_key = %s\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        last_key,
    );
    if parsing_swallows {
        verboselog(
            b"creating new swallow\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        current_swallow = smalloc(::core::mem::size_of::<Match>() as size_t) as *mut Match;
        match_init(current_swallow);
        (*current_swallow).dock = M_DONTCHECK;
        (*current_swallow).matches.tqe_next = ::core::ptr::null_mut::<Match>();
        (*current_swallow).matches.tqe_prev = (*json_node).swallow_head.tqh_last;
        *(*json_node).swallow_head.tqh_last = current_swallow as *mut Match;
        (*json_node).swallow_head.tqh_last = &raw mut (*current_swallow).matches.tqe_next;
        swallow_is_empty = true_0 != 0;
    } else if !parsing_rect
        && !parsing_actual_deco_rect
        && !parsing_deco_rect
        && !parsing_window_rect
        && !parsing_geometry
        && !parsing_gaps
    {
        if !last_key.is_null()
            && strcasecmp(
                last_key,
                b"floating_nodes\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            debuglog(
                b"%s:%s:%d - New floating_node\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"../src/load_layout.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"json_start_map\0" as *const u8 as *const ::core::ffi::c_char,
                72 as ::core::ffi::c_int,
            );
            let mut ws: *mut Con = con_get_workspace(json_node);
            json_node = con_new_skeleton(
                ::core::ptr::null_mut::<Con>(),
                ::core::ptr::null_mut::<i3Window>(),
            );
            (*json_node).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
            (*json_node).parent = ws as *mut Con;
            debuglog(
                b"%s:%s:%d - Parent is workspace = %p\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"../src/load_layout.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"json_start_map\0" as *const u8 as *const ::core::ffi::c_char,
                77 as ::core::ffi::c_int,
                ws,
            );
        } else {
            let mut parent: *mut Con = json_node;
            json_node = con_new_skeleton(
                ::core::ptr::null_mut::<Con>(),
                ::core::ptr::null_mut::<i3Window>(),
            );
            (*json_node).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
            (*json_node).parent = parent as *mut Con;
        }
        incomplete += 1;
        debuglog(
            b"%s:%s:%d - incomplete = %d\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b"../src/load_layout.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"json_start_map\0" as *const u8 as *const ::core::ffi::c_char,
            86 as ::core::ffi::c_int,
            incomplete,
        );
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn json_end_map(mut ctx: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    verboselog(
        b"end of map\n\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    if !parsing_swallows
        && !parsing_rect
        && !parsing_actual_deco_rect
        && !parsing_deco_rect
        && !parsing_window_rect
        && !parsing_geometry
        && !parsing_gaps
    {
        if (*json_node).layout as ::core::ffi::c_uint
            == L_DEFAULT as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            debuglog(
                b"%s:%s:%d - Setting layout = L_SPLITH\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"../src/load_layout.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"json_end_map\0" as *const u8 as *const ::core::ffi::c_char,
                103 as ::core::ffi::c_int,
            );
            (*json_node).layout = L_SPLITH;
        }
        if con_is_split(json_node) as ::core::ffi::c_int > 0 as ::core::ffi::c_int
            && !(*json_node).swallow_head.tqh_first.is_null()
        {
            debuglog(
                b"%s:%s:%d - sanity check: removing swallows specification from split container\n\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"../src/load_layout.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"json_end_map\0" as *const u8 as *const ::core::ffi::c_char,
                110 as ::core::ffi::c_int,
            );
            while !(*json_node).swallow_head.tqh_first.is_null() {
                let mut match_0: *mut Match = (*json_node).swallow_head.tqh_first as *mut Match;
                if !(*match_0).matches.tqe_next.is_null() {
                    (*(*match_0).matches.tqe_next).matches.tqe_prev = (*match_0).matches.tqe_prev;
                } else {
                    (*json_node).swallow_head.tqh_last = (*match_0).matches.tqe_prev;
                }
                *(*match_0).matches.tqe_prev = (*match_0).matches.tqe_next;
                match_free(match_0);
                free(match_0 as *mut ::core::ffi::c_void);
            }
        }
        if (*json_node).type_0 as ::core::ffi::c_uint
            == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            debuglog(
                b"%s:%s:%d - Attaching workspace. name = %s\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"../src/load_layout.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"json_end_map\0" as *const u8 as *const ::core::ffi::c_char,
                121 as ::core::ffi::c_int,
                (*json_node).name,
            );
            if (*json_node).name.is_null()
                || strcmp(
                    (*json_node).name,
                    b"\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
            {
                (*json_node).name =
                    sstrdup(b"unnamed\0" as *const u8 as *const ::core::ffi::c_char);
            }
            let mut base: *mut ::core::ffi::c_char = sstrdup((*json_node).name);
            let mut cnt: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
            while !get_existing_workspace_by_name((*json_node).name).is_null() {
                free((*json_node).name as *mut ::core::ffi::c_void);
                (*json_node).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
                let fresh1 = cnt;
                cnt = cnt + 1;
                sasprintf(
                    &raw mut (*json_node).name,
                    b"%s_%d\0" as *const u8 as *const ::core::ffi::c_char,
                    base,
                    fresh1,
                );
            }
            free(base as *mut ::core::ffi::c_void);
            (*json_node).num = ws_name_to_number((*json_node).name);
        }
        if (*json_node).type_0 as ::core::ffi::c_uint
            == CT_FLOATING_CON as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            debuglog(
                b"%s:%s:%d - fixing parent which currently is %p / %s\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"../src/load_layout.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"json_end_map\0" as *const u8 as *const ::core::ffi::c_char,
                148 as ::core::ffi::c_int,
                (*json_node).parent,
                (*(*json_node).parent).name,
            );
            (*json_node).parent = con_get_workspace((*json_node).parent as *mut Con) as *mut Con;
            if rect_equals(
                (*json_node).rect as Rect,
                Rect {
                    x: 0 as uint32_t,
                    y: 0 as uint32_t,
                    width: 0 as uint32_t,
                    height: 0 as uint32_t,
                },
            ) {
                debuglog(
                    b"%s:%s:%d - Geometry not set, combining children\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    b"../src/load_layout.c\0" as *const u8 as *const ::core::ffi::c_char,
                    b"json_end_map\0" as *const u8 as *const ::core::ffi::c_char,
                    155 as ::core::ffi::c_int,
                );
                let mut child: *mut Con = ::core::ptr::null_mut::<Con>();
                child = (*json_node).nodes_head.tqh_first as *mut Con;
                while !child.is_null() {
                    debuglog(
                        b"%s:%s:%d - child geometry: %d x %d\n\0" as *const u8
                            as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        b"../src/load_layout.c\0" as *const u8 as *const ::core::ffi::c_char,
                        b"json_end_map\0" as *const u8 as *const ::core::ffi::c_char,
                        158 as ::core::ffi::c_int,
                        (*child).geometry.width,
                        (*child).geometry.height,
                    );
                    (*json_node).rect.width = (*json_node)
                        .rect
                        .width
                        .wrapping_add((*child).geometry.width);
                    (*json_node).rect.height = max(
                        (*json_node).rect.height as ::core::ffi::c_int,
                        (*child).geometry.height as ::core::ffi::c_int,
                    ) as uint32_t;
                    child = (*child).nodes.tqe_next as *mut Con;
                }
            }
            floating_check_size(json_node, false_0 != 0);
        }
        if num_marks > 0 as ::core::ffi::c_int {
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i < num_marks {
                let mut con: *mut Con = (*marks.offset(i as isize)).con_to_be_marked;
                let mut mark: *mut ::core::ffi::c_char = (*marks.offset(i as isize)).mark;
                con_mark(con, mark, MM_ADD);
                free(mark as *mut ::core::ffi::c_void);
                i += 1;
            }
            free(marks as *mut ::core::ffi::c_void);
            marks = ::core::ptr::null_mut::<pending_marks>();
            num_marks = 0 as ::core::ffi::c_int;
        }
        verboselog(
            b"attaching\n\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        con_attach(json_node, (*json_node).parent as *mut Con, true_0 != 0);
        verboselog(
            b"Creating window\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        x_con_init(json_node);
        let old_floating_mode: ::core::ffi::c_int = (*json_node).floating as ::core::ffi::c_int;
        if old_floating_mode >= FLOATING_AUTO_ON as ::core::ffi::c_int
            && (*(*json_node).parent).type_0 as ::core::ffi::c_uint
                != CT_FLOATING_CON as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            verboselog(
                b"Fixing floating node without CT_FLOATING_CON parent\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
            (*json_node).floating = FLOATING_AUTO_OFF;
            floating_enable(json_node, false_0 != 0);
            (*json_node).floating = old_floating_mode as C2RustUnnamed_4;
        }
        json_node = (*json_node).parent as *mut Con;
        incomplete -= 1;
        debuglog(
            b"%s:%s:%d - incomplete = %d\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b"../src/load_layout.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"json_end_map\0" as *const u8 as *const ::core::ffi::c_char,
            198 as ::core::ffi::c_int,
            incomplete,
        );
    }
    if parsing_swallows as ::core::ffi::c_int != 0 && swallow_is_empty as ::core::ffi::c_int != 0 {
        errorlog(
            b"ERROR: Layout file is invalid: found an empty swallow definition.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    }
    parsing_gaps = false_0 != 0;
    parsing_rect = false_0 != 0;
    parsing_actual_deco_rect = false_0 != 0;
    parsing_deco_rect = false_0 != 0;
    parsing_window_rect = false_0 != 0;
    parsing_geometry = false_0 != 0;
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn json_end_array(mut ctx: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    verboselog(
        b"end of array\n\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    if !parsing_swallows && !parsing_focus && !parsing_marks {
        con_fix_percent(json_node);
    }
    if parsing_swallows {
        parsing_swallows = false_0 != 0;
    }
    if parsing_marks {
        parsing_marks = false_0 != 0;
    }
    if parsing_focus {
        let mut mapping: *mut focus_mapping = ::core::ptr::null_mut::<focus_mapping>();
        mapping = *(*(focus_mappings.tqh_last as *mut focus_mappings_head)).tqh_last;
        while !mapping.is_null() {
            verboselog(
                b"focus (reverse) %d\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                (*mapping).old_id,
            );
            let mut con: *mut Con = ::core::ptr::null_mut::<Con>();
            con = (*json_node).focus_head.tqh_first as *mut Con;
            while !con.is_null() {
                if (*con).old_id != (*mapping).old_id {
                    con = (*con).focused.tqe_next as *mut Con;
                } else {
                    verboselog(
                        b"got it! %p\n\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        con,
                    );
                    if !(*con).focused.tqe_next.is_null() {
                        (*(*con).focused.tqe_next).focused.tqe_prev = (*con).focused.tqe_prev;
                    } else {
                        (*json_node).focus_head.tqh_last = (*con).focused.tqe_prev;
                    }
                    *(*con).focused.tqe_prev = (*con).focused.tqe_next;
                    (*con).focused.tqe_next = (*json_node).focus_head.tqh_first;
                    if !(*con).focused.tqe_next.is_null() {
                        (*(*json_node).focus_head.tqh_first).focused.tqe_prev =
                            &raw mut (*con).focused.tqe_next;
                    } else {
                        (*json_node).focus_head.tqh_last = &raw mut (*con).focused.tqe_next;
                    }
                    (*json_node).focus_head.tqh_first = con as *mut Con;
                    (*con).focused.tqe_prev = &raw mut (*json_node).focus_head.tqh_first;
                    break;
                }
            }
            mapping = *(*((*mapping).focus_mappings.tqe_prev as *mut focus_mappings_head)).tqh_last;
        }
        while !focus_mappings.tqh_first.is_null() {
            mapping = focus_mappings.tqh_first;
            if !(*mapping).focus_mappings.tqe_next.is_null() {
                (*(*mapping).focus_mappings.tqe_next)
                    .focus_mappings
                    .tqe_prev = (*mapping).focus_mappings.tqe_prev;
            } else {
                focus_mappings.tqh_last = (*mapping).focus_mappings.tqe_prev;
            }
            *(*mapping).focus_mappings.tqe_prev = (*mapping).focus_mappings.tqe_next;
            free(mapping as *mut ::core::ffi::c_void);
        }
        parsing_focus = false_0 != 0;
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn json_key(
    mut ctx: *mut ::core::ffi::c_void,
    mut val: *const ::core::ffi::c_uchar,
    mut len: size_t,
) -> ::core::ffi::c_int {
    verboselog(
        b"key: %.*s\n\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        len as ::core::ffi::c_int,
        val,
    );
    free(last_key as *mut ::core::ffi::c_void);
    last_key = ::core::ptr::null_mut::<::core::ffi::c_char>();
    last_key = scalloc(len.wrapping_add(1 as size_t), 1 as size_t) as *mut ::core::ffi::c_char;
    memcpy(
        last_key as *mut ::core::ffi::c_void,
        val as *const ::core::ffi::c_void,
        len,
    );
    if strcasecmp(
        last_key,
        b"swallows\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        parsing_swallows = true_0 != 0;
    }
    if strcasecmp(
        last_key,
        b"gaps\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        parsing_gaps = true_0 != 0;
    }
    if strcasecmp(
        last_key,
        b"rect\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        parsing_rect = true_0 != 0;
    }
    if strcasecmp(
        last_key,
        b"actual_deco_rect\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        parsing_actual_deco_rect = true_0 != 0;
    }
    if strcasecmp(
        last_key,
        b"deco_rect\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        parsing_deco_rect = true_0 != 0;
    }
    if strcasecmp(
        last_key,
        b"window_rect\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        parsing_window_rect = true_0 != 0;
    }
    if strcasecmp(
        last_key,
        b"geometry\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        parsing_geometry = true_0 != 0;
    }
    if strcasecmp(
        last_key,
        b"focus\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        parsing_focus = true_0 != 0;
    }
    if strcasecmp(
        last_key,
        b"marks\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        num_marks = 0 as ::core::ffi::c_int;
        parsing_marks = true_0 != 0;
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn json_string(
    mut ctx: *mut ::core::ffi::c_void,
    mut val: *const ::core::ffi::c_uchar,
    mut len: size_t,
) -> ::core::ffi::c_int {
    verboselog(
        b"string: %.*s for key %s\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        len as ::core::ffi::c_int,
        val,
        last_key,
    );
    if parsing_swallows {
        let mut sval: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        sasprintf(
            &raw mut sval,
            b"%.*s\0" as *const u8 as *const ::core::ffi::c_char,
            len as ::core::ffi::c_int,
            val,
        );
        if strcasecmp(
            last_key,
            b"class\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*current_swallow).class = regex_new(sval);
            swallow_is_empty = false_0 != 0;
        } else if strcasecmp(
            last_key,
            b"instance\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*current_swallow).instance = regex_new(sval);
            swallow_is_empty = false_0 != 0;
        } else if strcasecmp(
            last_key,
            b"window_role\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*current_swallow).window_role = regex_new(sval);
            swallow_is_empty = false_0 != 0;
        } else if strcasecmp(
            last_key,
            b"title\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*current_swallow).title = regex_new(sval);
            swallow_is_empty = false_0 != 0;
        } else if strcasecmp(
            last_key,
            b"machine\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*current_swallow).machine = regex_new(sval);
            swallow_is_empty = false_0 != 0;
        } else {
            errorlog(
                b"ERROR: swallow key %s unknown\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                last_key,
            );
        }
        free(sval as *mut ::core::ffi::c_void);
    } else if parsing_marks {
        let mut mark: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        sasprintf(
            &raw mut mark,
            b"%.*s\0" as *const u8 as *const ::core::ffi::c_char,
            len as ::core::ffi::c_int,
            val,
        );
        num_marks += 1;
        marks = srealloc(
            marks as *mut ::core::ffi::c_void,
            (num_marks as size_t).wrapping_mul(::core::mem::size_of::<pending_marks>() as size_t),
        ) as *mut pending_marks;
        let ref mut fresh2 = (*marks.offset((num_marks - 1 as ::core::ffi::c_int) as isize)).mark;
        *fresh2 = mark;
        let ref mut fresh3 =
            (*marks.offset((num_marks - 1 as ::core::ffi::c_int) as isize)).con_to_be_marked;
        *fresh3 = json_node;
    } else if strcasecmp(
        last_key,
        b"name\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        (*json_node).name =
            scalloc(len.wrapping_add(1 as size_t), 1 as size_t) as *mut ::core::ffi::c_char;
        memcpy(
            (*json_node).name as *mut ::core::ffi::c_void,
            val as *const ::core::ffi::c_void,
            len,
        );
    } else if strcasecmp(
        last_key,
        b"title_format\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        (*json_node).title_format =
            scalloc(len.wrapping_add(1 as size_t), 1 as size_t) as *mut ::core::ffi::c_char;
        memcpy(
            (*json_node).title_format as *mut ::core::ffi::c_void,
            val as *const ::core::ffi::c_void,
            len,
        );
    } else if strcasecmp(
        last_key,
        b"sticky_group\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        (*json_node).sticky_group =
            scalloc(len.wrapping_add(1 as size_t), 1 as size_t) as *mut ::core::ffi::c_char;
        memcpy(
            (*json_node).sticky_group as *mut ::core::ffi::c_void,
            val as *const ::core::ffi::c_void,
            len,
        );
        verboselog(
            b"sticky_group of this container is %s\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            (*json_node).sticky_group,
        );
    } else if strcasecmp(
        last_key,
        b"orientation\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        let mut buf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        sasprintf(
            &raw mut buf,
            b"%.*s\0" as *const u8 as *const ::core::ffi::c_char,
            len as ::core::ffi::c_int,
            val,
        );
        if strcasecmp(buf, b"none\0" as *const u8 as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
            || strcasecmp(
                buf,
                b"horizontal\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            (*json_node).last_split_layout = L_SPLITH;
        } else if strcasecmp(
            buf,
            b"vertical\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*json_node).last_split_layout = L_SPLITV;
        } else {
            verboselog(
                b"Unhandled orientation: %s\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                buf,
            );
        }
        free(buf as *mut ::core::ffi::c_void);
    } else if strcasecmp(
        last_key,
        b"border\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        let mut buf_0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        sasprintf(
            &raw mut buf_0,
            b"%.*s\0" as *const u8 as *const ::core::ffi::c_char,
            len as ::core::ffi::c_int,
            val,
        );
        if strcasecmp(buf_0, b"none\0" as *const u8 as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            (*json_node).border_style = BS_NONE;
            (*json_node).max_user_border_style = (*json_node).border_style;
        } else if strcasecmp(
            buf_0,
            b"1pixel\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*json_node).border_style = BS_PIXEL;
            (*json_node).max_user_border_style = (*json_node).border_style;
            (*json_node).current_border_width = 1 as ::core::ffi::c_int;
        } else if strcasecmp(buf_0, b"pixel\0" as *const u8 as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            (*json_node).border_style = BS_PIXEL;
            (*json_node).max_user_border_style = (*json_node).border_style;
        } else if strcasecmp(
            buf_0,
            b"normal\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*json_node).border_style = BS_NORMAL;
            (*json_node).max_user_border_style = (*json_node).border_style;
        } else {
            verboselog(
                b"Unhandled \"border\": %s\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                buf_0,
            );
        }
        free(buf_0 as *mut ::core::ffi::c_void);
    } else if strcasecmp(
        last_key,
        b"type\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        let mut buf_1: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        sasprintf(
            &raw mut buf_1,
            b"%.*s\0" as *const u8 as *const ::core::ffi::c_char,
            len as ::core::ffi::c_int,
            val,
        );
        if strcasecmp(buf_1, b"root\0" as *const u8 as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            (*json_node).type_0 = CT_ROOT;
        } else if strcasecmp(
            buf_1,
            b"output\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*json_node).type_0 = CT_OUTPUT;
        } else if strcasecmp(buf_1, b"con\0" as *const u8 as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            (*json_node).type_0 = CT_CON;
        } else if strcasecmp(
            buf_1,
            b"floating_con\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*json_node).type_0 = CT_FLOATING_CON;
        } else if strcasecmp(
            buf_1,
            b"workspace\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*json_node).type_0 = CT_WORKSPACE;
        } else if strcasecmp(
            buf_1,
            b"dockarea\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*json_node).type_0 = CT_DOCKAREA;
        } else {
            verboselog(
                b"Unhandled \"type\": %s\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                buf_1,
            );
        }
        free(buf_1 as *mut ::core::ffi::c_void);
    } else if strcasecmp(
        last_key,
        b"layout\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        let mut buf_2: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        sasprintf(
            &raw mut buf_2,
            b"%.*s\0" as *const u8 as *const ::core::ffi::c_char,
            len as ::core::ffi::c_int,
            val,
        );
        if strcasecmp(
            buf_2,
            b"default\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*json_node).layout = (*json_node).last_split_layout;
        } else if strcasecmp(
            buf_2,
            b"stacked\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*json_node).layout = L_STACKED;
        } else if strcasecmp(
            buf_2,
            b"tabbed\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*json_node).layout = L_TABBED;
        } else if strcasecmp(
            buf_2,
            b"dockarea\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*json_node).layout = L_DOCKAREA;
        } else if strcasecmp(
            buf_2,
            b"output\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*json_node).layout = L_OUTPUT;
        } else if strcasecmp(
            buf_2,
            b"splith\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*json_node).layout = L_SPLITH;
        } else if strcasecmp(
            buf_2,
            b"splitv\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*json_node).layout = L_SPLITV;
        } else {
            verboselog(
                b"Unhandled \"layout\": %s\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                buf_2,
            );
        }
        free(buf_2 as *mut ::core::ffi::c_void);
    } else if strcasecmp(
        last_key,
        b"workspace_layout\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        let mut buf_3: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        sasprintf(
            &raw mut buf_3,
            b"%.*s\0" as *const u8 as *const ::core::ffi::c_char,
            len as ::core::ffi::c_int,
            val,
        );
        if strcasecmp(
            buf_3,
            b"default\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*json_node).workspace_layout = L_DEFAULT;
        } else if strcasecmp(
            buf_3,
            b"stacked\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*json_node).workspace_layout = L_STACKED;
        } else if strcasecmp(
            buf_3,
            b"tabbed\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*json_node).workspace_layout = L_TABBED;
        } else {
            verboselog(
                b"Unhandled \"workspace_layout\": %s\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                buf_3,
            );
        }
        free(buf_3 as *mut ::core::ffi::c_void);
    } else if strcasecmp(
        last_key,
        b"last_split_layout\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        let mut buf_4: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        sasprintf(
            &raw mut buf_4,
            b"%.*s\0" as *const u8 as *const ::core::ffi::c_char,
            len as ::core::ffi::c_int,
            val,
        );
        if strcasecmp(
            buf_4,
            b"splith\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*json_node).last_split_layout = L_SPLITH;
        } else if strcasecmp(
            buf_4,
            b"splitv\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*json_node).last_split_layout = L_SPLITV;
        } else {
            verboselog(
                b"Unhandled \"last_splitlayout\": %s\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                buf_4,
            );
        }
        free(buf_4 as *mut ::core::ffi::c_void);
    } else if strcasecmp(
        last_key,
        b"mark\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        debuglog(
            b"%s:%s:%d - Found deprecated key \"mark\".\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/load_layout.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"json_string\0" as *const u8 as *const ::core::ffi::c_char,
            443 as ::core::ffi::c_int,
        );
        let mut buf_5: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        sasprintf(
            &raw mut buf_5,
            b"%.*s\0" as *const u8 as *const ::core::ffi::c_char,
            len as ::core::ffi::c_int,
            val,
        );
        con_mark(json_node, buf_5, MM_REPLACE);
    } else if strcasecmp(
        last_key,
        b"floating\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        let mut buf_6: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        sasprintf(
            &raw mut buf_6,
            b"%.*s\0" as *const u8 as *const ::core::ffi::c_char,
            len as ::core::ffi::c_int,
            val,
        );
        if strcasecmp(
            buf_6,
            b"auto_off\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*json_node).floating = FLOATING_AUTO_OFF;
        } else if strcasecmp(
            buf_6,
            b"auto_on\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*json_node).floating = FLOATING_AUTO_ON;
        } else if strcasecmp(
            buf_6,
            b"user_off\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*json_node).floating = FLOATING_USER_OFF;
        } else if strcasecmp(
            buf_6,
            b"user_on\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*json_node).floating = FLOATING_USER_ON;
        }
        free(buf_6 as *mut ::core::ffi::c_void);
    } else if strcasecmp(
        last_key,
        b"scratchpad_state\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        let mut buf_7: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        sasprintf(
            &raw mut buf_7,
            b"%.*s\0" as *const u8 as *const ::core::ffi::c_char,
            len as ::core::ffi::c_int,
            val,
        );
        if strcasecmp(buf_7, b"none\0" as *const u8 as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            (*json_node).scratchpad_state = SCRATCHPAD_NONE;
        } else if strcasecmp(buf_7, b"fresh\0" as *const u8 as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            (*json_node).scratchpad_state = SCRATCHPAD_FRESH;
        } else if strcasecmp(
            buf_7,
            b"changed\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*json_node).scratchpad_state = SCRATCHPAD_CHANGED;
        }
        free(buf_7 as *mut ::core::ffi::c_void);
    } else if strcasecmp(
        last_key,
        b"previous_workspace_name\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        free(previous_workspace_name as *mut ::core::ffi::c_void);
        previous_workspace_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
        previous_workspace_name = sstrndup(val as *const ::core::ffi::c_char, len);
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn json_int(
    mut ctx: *mut ::core::ffi::c_void,
    mut val: ::core::ffi::c_longlong,
) -> ::core::ffi::c_int {
    verboselog(
        b"int %lld for key %s\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        val,
        last_key,
    );
    if strcasecmp(
        last_key,
        b"type\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        (*json_node).type_0 = val as C2RustUnnamed_15;
    }
    if strcasecmp(
        last_key,
        b"fullscreen_mode\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        (*json_node).fullscreen_mode = val as fullscreen_mode_t;
    }
    if strcasecmp(
        last_key,
        b"num\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        (*json_node).num = val as ::core::ffi::c_int;
    }
    if strcasecmp(
        last_key,
        b"current_border_width\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        (*json_node).current_border_width = val as ::core::ffi::c_int;
    }
    if strcasecmp(
        last_key,
        b"window_icon_padding\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        (*json_node).window_icon_padding = val as ::core::ffi::c_int;
    }
    if strcasecmp(
        last_key,
        b"depth\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        (*json_node).depth = val as uint16_t;
    }
    if !parsing_swallows
        && strcasecmp(last_key, b"id\0" as *const u8 as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
    {
        (*json_node).old_id = val as ::core::ffi::c_int;
    }
    if parsing_focus {
        let mut focus_mapping: *mut focus_mapping = scalloc(
            1 as size_t,
            ::core::mem::size_of::<focus_mapping>() as size_t,
        ) as *mut focus_mapping;
        (*focus_mapping).old_id = val as ::core::ffi::c_int;
        (*focus_mapping).focus_mappings.tqe_next = ::core::ptr::null_mut::<focus_mapping>();
        (*focus_mapping).focus_mappings.tqe_prev = focus_mappings.tqh_last;
        *focus_mappings.tqh_last = focus_mapping;
        focus_mappings.tqh_last = &raw mut (*focus_mapping).focus_mappings.tqe_next;
    }
    if parsing_rect as ::core::ffi::c_int != 0
        || parsing_window_rect as ::core::ffi::c_int != 0
        || parsing_geometry as ::core::ffi::c_int != 0
    {
        let mut r: *mut Rect = ::core::ptr::null_mut::<Rect>();
        if parsing_rect {
            r = &raw mut (*json_node).rect as *mut Rect;
        } else if parsing_window_rect {
            r = &raw mut (*json_node).window_rect as *mut Rect;
        } else {
            r = &raw mut (*json_node).geometry as *mut Rect;
        }
        if strcasecmp(last_key, b"x\0" as *const u8 as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            (*r).x = val as uint32_t;
        } else if strcasecmp(last_key, b"y\0" as *const u8 as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            (*r).y = val as uint32_t;
        } else if strcasecmp(
            last_key,
            b"width\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*r).width = val as uint32_t;
        } else if strcasecmp(
            last_key,
            b"height\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*r).height = val as uint32_t;
        } else {
            errorlog(
                b"ERROR: WARNING: unknown key %s in rect\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                last_key,
            );
        }
        debuglog(
            b"%s:%s:%d - rect now: (%d, %d, %d, %d)\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b"../src/load_layout.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"json_int\0" as *const u8 as *const ::core::ffi::c_char,
            539 as ::core::ffi::c_int,
            (*r).x,
            (*r).y,
            (*r).width,
            (*r).height,
        );
    }
    if parsing_swallows {
        if strcasecmp(last_key, b"id\0" as *const u8 as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            (*current_swallow).id = val as xcb_window_t;
            swallow_is_empty = false_0 != 0;
        }
        if strcasecmp(
            last_key,
            b"dock\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*current_swallow).dock = val as C2RustUnnamed_8;
            swallow_is_empty = false_0 != 0;
        }
        if strcasecmp(
            last_key,
            b"insert_where\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*current_swallow).insert_where = val as C2RustUnnamed_6;
            swallow_is_empty = false_0 != 0;
        }
    }
    if parsing_gaps {
        if strcasecmp(
            last_key,
            b"inner\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*json_node).gaps.inner = val as ::core::ffi::c_int;
        } else if strcasecmp(
            last_key,
            b"top\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*json_node).gaps.top = val as ::core::ffi::c_int;
        } else if strcasecmp(
            last_key,
            b"right\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*json_node).gaps.right = val as ::core::ffi::c_int;
        } else if strcasecmp(
            last_key,
            b"bottom\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*json_node).gaps.bottom = val as ::core::ffi::c_int;
        } else if strcasecmp(
            last_key,
            b"left\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*json_node).gaps.left = val as ::core::ffi::c_int;
        }
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn json_bool(
    mut ctx: *mut ::core::ffi::c_void,
    mut val: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    verboselog(
        b"bool %d for key %s\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        val,
        last_key,
    );
    if strcasecmp(
        last_key,
        b"focused\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
        && val != 0
    {
        to_focus = json_node;
    }
    if strcasecmp(
        last_key,
        b"sticky\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        (*json_node).sticky = val != 0;
    }
    if parsing_swallows {
        if strcasecmp(
            last_key,
            b"restart_mode\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*current_swallow).restart_mode = val != 0;
            swallow_is_empty = false_0 != 0;
        }
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn json_double(
    mut ctx: *mut ::core::ffi::c_void,
    mut val: ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    verboselog(
        b"double %f for key %s\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        val,
        last_key,
    );
    if strcasecmp(
        last_key,
        b"percent\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        (*json_node).percent = val;
    }
    return 1 as ::core::ffi::c_int;
}
static mut content_result: json_content_t = JSON_CONTENT_UNKNOWN;
static mut content_level: ::core::ffi::c_int = 0;
unsafe extern "C" fn json_determine_content_deeper(
    mut ctx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    content_level += 1;
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn json_determine_content_shallower(
    mut ctx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    content_level -= 1;
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn json_determine_content_string(
    mut ctx: *mut ::core::ffi::c_void,
    mut val: *const ::core::ffi::c_uchar,
    mut len: size_t,
) -> ::core::ffi::c_int {
    if strcasecmp(
        last_key,
        b"type\0" as *const u8 as *const ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int
        || content_level > 1 as ::core::ffi::c_int
    {
        return 1 as ::core::ffi::c_int;
    }
    debuglog(
        b"%s:%s:%d - string = %.*s, last_key = %s\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/load_layout.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"json_determine_content_string\0" as *const u8 as *const ::core::ffi::c_char,
        618 as ::core::ffi::c_int,
        len as ::core::ffi::c_int,
        val,
        last_key,
    );
    if strncasecmp(
        val as *const ::core::ffi::c_char,
        b"workspace\0" as *const u8 as *const ::core::ffi::c_char,
        len,
    ) == 0 as ::core::ffi::c_int
    {
        content_result = JSON_CONTENT_WORKSPACE;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn json_validate(mut buf: *const ::core::ffi::c_char, len: size_t) -> bool {
    let mut valid: bool = true_0 != 0;
    let mut hand: yajl_handle = yajl_alloc(
        ::core::ptr::null::<yajl_callbacks>(),
        ::core::ptr::null_mut::<yajl_alloc_funcs>(),
        NULL,
    );
    yajl_config(hand, yajl_allow_comments, true_0);
    yajl_config(hand, yajl_allow_multiple_values, true_0);
    let mut prev_locale: *mut ::core::ffi::c_char = sstrdup(setlocale(
        LC_NUMERIC,
        ::core::ptr::null::<::core::ffi::c_char>(),
    ));
    setlocale(
        LC_NUMERIC,
        b"C\0" as *const u8 as *const ::core::ffi::c_char,
    );
    if yajl_parse(hand, buf as *const ::core::ffi::c_uchar, len) as ::core::ffi::c_uint
        != yajl_status_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut str: *mut ::core::ffi::c_uchar = yajl_get_error(
            hand,
            1 as ::core::ffi::c_int,
            buf as *const ::core::ffi::c_uchar,
            len,
        );
        errorlog(
            b"ERROR: JSON parsing error: %s\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            str,
        );
        yajl_free_error(hand, str);
        valid = false_0 != 0;
    }
    setlocale(LC_NUMERIC, prev_locale);
    free(prev_locale as *mut ::core::ffi::c_void);
    yajl_complete_parse(hand);
    yajl_free(hand);
    return valid;
}
#[no_mangle]
pub unsafe extern "C" fn json_determine_content(
    mut buf: *const ::core::ffi::c_char,
    len: size_t,
) -> json_content_t {
    content_result = JSON_CONTENT_CON;
    content_level = 0 as ::core::ffi::c_int;
    static mut callbacks: yajl_callbacks = unsafe {
        yajl_callbacks {
            yajl_null: None,
            yajl_boolean: None,
            yajl_integer: None,
            yajl_double: None,
            yajl_number: None,
            yajl_string: Some(
                json_determine_content_string
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const ::core::ffi::c_uchar,
                        size_t,
                    ) -> ::core::ffi::c_int,
            ),
            yajl_start_map: Some(
                json_determine_content_deeper
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
            ),
            yajl_map_key: Some(
                json_key
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const ::core::ffi::c_uchar,
                        size_t,
                    ) -> ::core::ffi::c_int,
            ),
            yajl_end_map: Some(
                json_determine_content_shallower
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
            ),
            yajl_start_array: Some(
                json_determine_content_deeper
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
            ),
            yajl_end_array: Some(
                json_determine_content_shallower
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
            ),
        }
    };
    let mut hand: yajl_handle = yajl_alloc(
        &raw mut callbacks,
        ::core::ptr::null_mut::<yajl_alloc_funcs>(),
        NULL,
    );
    yajl_config(hand, yajl_allow_comments, true_0);
    yajl_config(hand, yajl_allow_multiple_values, true_0);
    let mut prev_locale: *mut ::core::ffi::c_char = sstrdup(setlocale(
        LC_NUMERIC,
        ::core::ptr::null::<::core::ffi::c_char>(),
    ));
    setlocale(
        LC_NUMERIC,
        b"C\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let stat: yajl_status =
        yajl_parse(hand, buf as *const ::core::ffi::c_uchar, len) as yajl_status;
    if stat as ::core::ffi::c_uint != yajl_status_ok as ::core::ffi::c_int as ::core::ffi::c_uint
        && stat as ::core::ffi::c_uint
            != yajl_status_client_canceled as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut str: *mut ::core::ffi::c_uchar = yajl_get_error(
            hand,
            1 as ::core::ffi::c_int,
            buf as *const ::core::ffi::c_uchar,
            len,
        );
        errorlog(
            b"ERROR: JSON parsing error: %s\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            str,
        );
        yajl_free_error(hand, str);
    }
    setlocale(LC_NUMERIC, prev_locale);
    free(prev_locale as *mut ::core::ffi::c_void);
    yajl_complete_parse(hand);
    yajl_free(hand);
    return content_result;
}
#[no_mangle]
pub unsafe extern "C" fn tree_append_json(
    mut con: *mut Con,
    mut buf: *const ::core::ffi::c_char,
    len: size_t,
    mut errormsg: *mut *mut ::core::ffi::c_char,
) {
    static mut callbacks: yajl_callbacks = unsafe {
        yajl_callbacks {
            yajl_null: None,
            yajl_boolean: Some(
                json_bool
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
            yajl_integer: Some(
                json_int
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        ::core::ffi::c_longlong,
                    ) -> ::core::ffi::c_int,
            ),
            yajl_double: Some(
                json_double
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        ::core::ffi::c_double,
                    ) -> ::core::ffi::c_int,
            ),
            yajl_number: None,
            yajl_string: Some(
                json_string
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const ::core::ffi::c_uchar,
                        size_t,
                    ) -> ::core::ffi::c_int,
            ),
            yajl_start_map: Some(
                json_start_map
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
            ),
            yajl_map_key: Some(
                json_key
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const ::core::ffi::c_uchar,
                        size_t,
                    ) -> ::core::ffi::c_int,
            ),
            yajl_end_map: Some(
                json_end_map
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
            ),
            yajl_start_array: None,
            yajl_end_array: Some(
                json_end_array
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
            ),
        }
    };
    let mut hand: yajl_handle = yajl_alloc(
        &raw mut callbacks,
        ::core::ptr::null_mut::<yajl_alloc_funcs>(),
        NULL,
    );
    yajl_config(hand, yajl_allow_comments, true_0);
    yajl_config(hand, yajl_allow_multiple_values, true_0);
    yajl_config(hand, yajl_dont_validate_strings, true_0);
    json_node = con;
    to_focus = ::core::ptr::null_mut::<Con>();
    parsing_gaps = false_0 != 0;
    incomplete = 0 as ::core::ffi::c_int;
    parsing_swallows = false_0 != 0;
    parsing_rect = false_0 != 0;
    parsing_actual_deco_rect = false_0 != 0;
    parsing_deco_rect = false_0 != 0;
    parsing_window_rect = false_0 != 0;
    parsing_geometry = false_0 != 0;
    parsing_focus = false_0 != 0;
    parsing_marks = false_0 != 0;
    let mut prev_locale: *mut ::core::ffi::c_char = sstrdup(setlocale(
        LC_NUMERIC,
        ::core::ptr::null::<::core::ffi::c_char>(),
    ));
    setlocale(
        LC_NUMERIC,
        b"C\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let stat: yajl_status =
        yajl_parse(hand, buf as *const ::core::ffi::c_uchar, len) as yajl_status;
    if stat as ::core::ffi::c_uint != yajl_status_ok as ::core::ffi::c_int as ::core::ffi::c_uint {
        let mut str: *mut ::core::ffi::c_uchar = yajl_get_error(
            hand,
            1 as ::core::ffi::c_int,
            buf as *const ::core::ffi::c_uchar,
            len,
        );
        errorlog(
            b"ERROR: JSON parsing error: %s\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            str,
        );
        if !errormsg.is_null() {
            *errormsg = sstrdup(str as *const ::core::ffi::c_char);
        }
        yajl_free_error(hand, str);
        loop {
            let fresh0 = incomplete;
            incomplete = incomplete - 1;
            if !(fresh0 > 0 as ::core::ffi::c_int) {
                break;
            }
            let mut parent: *mut Con = (*json_node).parent as *mut Con;
            debuglog(
                b"%s:%s:%d - freeing incomplete container %p\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"../src/load_layout.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"tree_append_json\0" as *const u8 as *const ::core::ffi::c_char,
                744 as ::core::ffi::c_int,
                json_node,
            );
            if json_node == to_focus {
                to_focus = ::core::ptr::null_mut::<Con>();
            }
            con_free(json_node);
            json_node = parent;
        }
    }
    con_fix_percent(con);
    setlocale(LC_NUMERIC, prev_locale);
    free(prev_locale as *mut ::core::ffi::c_void);
    yajl_complete_parse(hand);
    yajl_free(hand);
    if !to_focus.is_null() {
        con_activate(to_focus);
    }
}
unsafe extern "C" fn run_static_initializers() {
    focus_mappings = focus_mappings_head {
        tqh_first: ::core::ptr::null_mut::<focus_mapping>(),
        tqh_last: &raw mut focus_mappings.tqh_first,
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
