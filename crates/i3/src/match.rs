/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * A "match" is a data structure which acts like a mask or expression to match
 * certain windows or not. For example, when using commands, you can specify a
 * command like this: [title="*Firefox*"] kill. The title member of the match
 * data structure will then be filled and i3 will check each window using
 * match_matches_window() to find the windows affected by this command.
 *
 */
// Mechanical file-by-file Rust translation of src/match.c.
// Generated with C2Rust 0.22.1; C ABI and original ownership retained.
extern "C" {
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
    fn verboselog(fmt: *mut ::core::ffi::c_char, ...);
    fn errorlog(fmt: *mut ::core::ffi::c_char, ...);
    fn debuglog(fmt: *mut ::core::ffi::c_char, ...);
    fn sstrdup(str: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn i3string_as_utf8(str: *mut i3String) -> *const ::core::ffi::c_char;
    fn parse_long(
        str: *const ::core::ffi::c_char,
        out: *mut ::core::ffi::c_long,
        base: ::core::ffi::c_int,
    ) -> bool;
    static mut focused: *mut Con;
    static mut all_cons: all_cons_head;
    static mut A__NET_WM_WINDOW_TYPE_NORMAL: xcb_atom_t;
    static mut A__NET_WM_WINDOW_TYPE_DIALOG: xcb_atom_t;
    static mut A__NET_WM_WINDOW_TYPE_UTILITY: xcb_atom_t;
    static mut A__NET_WM_WINDOW_TYPE_TOOLBAR: xcb_atom_t;
    static mut A__NET_WM_WINDOW_TYPE_SPLASH: xcb_atom_t;
    static mut A__NET_WM_WINDOW_TYPE_MENU: xcb_atom_t;
    static mut A__NET_WM_WINDOW_TYPE_DROPDOWN_MENU: xcb_atom_t;
    static mut A__NET_WM_WINDOW_TYPE_POPUP_MENU: xcb_atom_t;
    static mut A__NET_WM_WINDOW_TYPE_TOOLTIP: xcb_atom_t;
    static mut A__NET_WM_WINDOW_TYPE_NOTIFICATION: xcb_atom_t;
    fn con_get_workspace(con: *mut Con) -> *mut Con;
    fn con_inside_floating(con: *mut Con) -> *mut Con;
    fn con_by_window_id(window: xcb_window_t) -> *mut Con;
    fn regex_new(pattern: *const ::core::ffi::c_char) -> *mut regex;
    fn regex_free(regex: *mut regex);
    fn regex_matches(regex: *const regex, input: *const ::core::ffi::c_char) -> bool;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct all_cons_head {
    pub tqh_first: *mut Con,
    pub tqh_last: *mut *mut Con,
}
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const UINT32_MAX: ::core::ffi::c_uint = 4294967295 as ::core::ffi::c_uint;
pub const XCB_NONE: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[no_mangle]
pub unsafe extern "C" fn match_init(mut match_0: *mut Match) {
    memset(
        match_0 as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<Match>() as size_t,
    );
    (*match_0).urgent = U_DONTCHECK;
    (*match_0).window_mode = WM_ANY;
    (*match_0).window_type = UINT32_MAX as xcb_atom_t;
}
#[no_mangle]
pub unsafe extern "C" fn match_is_empty(mut match_0: *mut Match) -> bool {
    return (*match_0).title.is_null()
        && (*match_0).mark.is_null()
        && (*match_0).application.is_null()
        && (*match_0).class.is_null()
        && (*match_0).instance.is_null()
        && (*match_0).window_role.is_null()
        && (*match_0).workspace.is_null()
        && (*match_0).machine.is_null()
        && (*match_0).urgent as ::core::ffi::c_int == U_DONTCHECK as ::core::ffi::c_int
        && (*match_0).id as ::core::ffi::c_long == XCB_NONE
        && (*match_0).window_type == UINT32_MAX as xcb_atom_t
        && (*match_0).con_id.is_null()
        && (*match_0).dock as ::core::ffi::c_int == M_NODOCK as ::core::ffi::c_int
        && (*match_0).window_mode as ::core::ffi::c_uint
            == WM_ANY as ::core::ffi::c_int as ::core::ffi::c_uint
        && (*match_0).match_all_windows as ::core::ffi::c_int == false_0;
}
#[no_mangle]
pub unsafe extern "C" fn match_copy(mut dest: *mut Match, mut src: *mut Match) {
    memcpy(
        dest as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<Match>() as size_t,
    );
    if !(*src).title.is_null() {
        (*dest).title = regex_new((*(*src).title).pattern);
    }
    if !(*src).mark.is_null() {
        (*dest).mark = regex_new((*(*src).mark).pattern);
    }
    if !(*src).application.is_null() {
        (*dest).application = regex_new((*(*src).application).pattern);
    }
    if !(*src).class.is_null() {
        (*dest).class = regex_new((*(*src).class).pattern);
    }
    if !(*src).instance.is_null() {
        (*dest).instance = regex_new((*(*src).instance).pattern);
    }
    if !(*src).window_role.is_null() {
        (*dest).window_role = regex_new((*(*src).window_role).pattern);
    }
    if !(*src).workspace.is_null() {
        (*dest).workspace = regex_new((*(*src).workspace).pattern);
    }
    if !(*src).machine.is_null() {
        (*dest).machine = regex_new((*(*src).machine).pattern);
    }
}
#[no_mangle]
pub unsafe extern "C" fn match_matches_window(
    mut match_0: *mut Match,
    mut window: *mut i3Window,
) -> bool {
    verboselog(
        b"Checking window 0x%08x (class %s)\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        (*window).id,
        (*window).class_class,
    );
    if !(*match_0).class.is_null() {
        let mut window_field_str: *const ::core::ffi::c_char = if (*window).class_class.is_null() {
            b"\0" as *const u8 as *const ::core::ffi::c_char
        } else {
            (*window).class_class as *const ::core::ffi::c_char
        };
        if strcmp(
            (*(*match_0).class).pattern,
            b"__focused__\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            && !focused.is_null()
            && !(*focused).window.is_null()
            && !(*(*focused).window).class_class.is_null()
            && strcmp(window_field_str, (*(*focused).window).class_class) == 0 as ::core::ffi::c_int
        {
            verboselog(
                b"window class matches focused window\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        } else if regex_matches((*match_0).class, window_field_str) {
            verboselog(
                b"window class matches (%s)\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                window_field_str,
            );
        } else {
            return false_0 != 0;
        }
    }
    if !(*match_0).instance.is_null() {
        let mut window_field_str_0: *const ::core::ffi::c_char =
            if (*window).class_instance.is_null() {
                b"\0" as *const u8 as *const ::core::ffi::c_char
            } else {
                (*window).class_instance as *const ::core::ffi::c_char
            };
        if strcmp(
            (*(*match_0).instance).pattern,
            b"__focused__\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            && !focused.is_null()
            && !(*focused).window.is_null()
            && !(*(*focused).window).class_instance.is_null()
            && strcmp(window_field_str_0, (*(*focused).window).class_instance)
                == 0 as ::core::ffi::c_int
        {
            verboselog(
                b"window instance matches focused window\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        } else if regex_matches((*match_0).instance, window_field_str_0) {
            verboselog(
                b"window instance matches (%s)\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                window_field_str_0,
            );
        } else {
            return false_0 != 0;
        }
    }
    if (*match_0).id as ::core::ffi::c_long != XCB_NONE {
        if (*window).id == (*match_0).id {
            verboselog(
                b"match made by window id (%d)\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                (*window).id,
            );
        } else {
            verboselog(
                b"window id does not match\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
            return false_0 != 0;
        }
    }
    if !(*match_0).title.is_null() {
        let mut window_field_str_1: *const ::core::ffi::c_char = if (*window).name.is_null() {
            b"\0" as *const u8 as *const ::core::ffi::c_char
        } else {
            i3string_as_utf8((*window).name)
        };
        if strcmp(
            (*(*match_0).title).pattern,
            b"__focused__\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            && !focused.is_null()
            && !(*focused).window.is_null()
            && !(*(*focused).window).name.is_null()
            && strcmp(
                window_field_str_1,
                i3string_as_utf8((*(*focused).window).name),
            ) == 0 as ::core::ffi::c_int
        {
            verboselog(
                b"window title matches focused window\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        } else if regex_matches((*match_0).title, window_field_str_1) {
            verboselog(
                b"window title matches (%s)\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                window_field_str_1,
            );
        } else {
            return false_0 != 0;
        }
    }
    if !(*match_0).window_role.is_null() {
        let mut window_field_str_2: *const ::core::ffi::c_char = if (*window).role.is_null() {
            b"\0" as *const u8 as *const ::core::ffi::c_char
        } else {
            (*window).role as *const ::core::ffi::c_char
        };
        if strcmp(
            (*(*match_0).window_role).pattern,
            b"__focused__\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            && !focused.is_null()
            && !(*focused).window.is_null()
            && !(*(*focused).window).role.is_null()
            && strcmp(window_field_str_2, (*(*focused).window).role) == 0 as ::core::ffi::c_int
        {
            verboselog(
                b"window window_role matches focused window\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        } else if regex_matches((*match_0).window_role, window_field_str_2) {
            verboselog(
                b"window window_role matches (%s)\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                window_field_str_2,
            );
        } else {
            return false_0 != 0;
        }
    }
    if (*match_0).window_type != UINT32_MAX as xcb_atom_t {
        if (*window).window_type == (*match_0).window_type {
            verboselog(
                b"window_type matches (%i)\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                (*match_0).window_type,
            );
        } else {
            return false_0 != 0;
        }
    }
    if !(*match_0).machine.is_null() {
        let mut window_field_str_3: *const ::core::ffi::c_char = if (*window).machine.is_null() {
            b"\0" as *const u8 as *const ::core::ffi::c_char
        } else {
            (*window).machine as *const ::core::ffi::c_char
        };
        if strcmp(
            (*(*match_0).machine).pattern,
            b"__focused__\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            && !focused.is_null()
            && !(*focused).window.is_null()
            && !(*(*focused).window).machine.is_null()
            && strcmp(window_field_str_3, (*(*focused).window).machine) == 0 as ::core::ffi::c_int
        {
            verboselog(
                b"window machine matches focused window\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        } else if regex_matches((*match_0).machine, window_field_str_3) {
            verboselog(
                b"window machine matches (%s)\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                window_field_str_3,
            );
        } else {
            return false_0 != 0;
        }
    }
    let mut con: *mut Con = ::core::ptr::null_mut::<Con>();
    if (*match_0).urgent as ::core::ffi::c_int == U_LATEST as ::core::ffi::c_int {
        if (*window).urgent.tv_sec == 0 as __time_t {
            return false_0 != 0;
        }
        con = all_cons.tqh_first as *mut Con;
        while !con.is_null() {
            if !(*con).window.is_null()
                && (if (*(*con).window).urgent.tv_sec == (*window).urgent.tv_sec {
                    ((*(*con).window).urgent.tv_usec > (*window).urgent.tv_usec)
                        as ::core::ffi::c_int
                } else {
                    ((*(*con).window).urgent.tv_sec > (*window).urgent.tv_sec) as ::core::ffi::c_int
                }) != 0
            {
                return false_0 != 0;
            }
            con = (*con).all_cons.tqe_next as *mut Con;
        }
        verboselog(
            b"urgent matches latest\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
    }
    if (*match_0).urgent as ::core::ffi::c_int == U_OLDEST as ::core::ffi::c_int {
        if (*window).urgent.tv_sec == 0 as __time_t {
            return false_0 != 0;
        }
        con = all_cons.tqh_first as *mut Con;
        while !con.is_null() {
            if !(*con).window.is_null()
                && (*(*con).window).urgent.tv_sec != 0 as __time_t
                && (if (*(*con).window).urgent.tv_sec == (*window).urgent.tv_sec {
                    ((*(*con).window).urgent.tv_usec < (*window).urgent.tv_usec)
                        as ::core::ffi::c_int
                } else {
                    ((*(*con).window).urgent.tv_sec < (*window).urgent.tv_sec) as ::core::ffi::c_int
                }) != 0
            {
                return false_0 != 0;
            }
            con = (*con).all_cons.tqe_next as *mut Con;
        }
        verboselog(
            b"urgent matches oldest\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
    }
    if !(*match_0).workspace.is_null() {
        con = con_by_window_id((*window).id);
        if con.is_null() {
            return false_0 != 0;
        }
        let mut ws: *mut Con = con_get_workspace(con);
        if ws.is_null() {
            return false_0 != 0;
        }
        if strcmp(
            (*(*match_0).workspace).pattern,
            b"__focused__\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            && strcmp((*ws).name, (*con_get_workspace(focused)).name) == 0 as ::core::ffi::c_int
        {
            verboselog(
                b"workspace matches focused workspace\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        } else if regex_matches((*match_0).workspace, (*ws).name) {
            verboselog(
                b"workspace matches (%s)\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                (*ws).name,
            );
        } else {
            return false_0 != 0;
        }
    }
    if (*match_0).dock as ::core::ffi::c_int != M_DONTCHECK as ::core::ffi::c_int {
        if (*window).dock as ::core::ffi::c_uint
            == W_DOCK_TOP as ::core::ffi::c_int as ::core::ffi::c_uint
            && (*match_0).dock as ::core::ffi::c_int == M_DOCK_TOP as ::core::ffi::c_int
            || (*window).dock as ::core::ffi::c_uint
                == W_DOCK_BOTTOM as ::core::ffi::c_int as ::core::ffi::c_uint
                && (*match_0).dock as ::core::ffi::c_int == M_DOCK_BOTTOM as ::core::ffi::c_int
            || ((*window).dock as ::core::ffi::c_uint
                == W_DOCK_TOP as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*window).dock as ::core::ffi::c_uint
                    == W_DOCK_BOTTOM as ::core::ffi::c_int as ::core::ffi::c_uint)
                && (*match_0).dock as ::core::ffi::c_int == M_DOCK_ANY as ::core::ffi::c_int
            || (*window).dock as ::core::ffi::c_uint
                == W_NODOCK as ::core::ffi::c_int as ::core::ffi::c_uint
                && (*match_0).dock as ::core::ffi::c_int == M_NODOCK as ::core::ffi::c_int
        {
            verboselog(
                b"dock status matches\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
        } else {
            verboselog(
                b"dock status does not match\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
            return false_0 != 0;
        }
    }
    if !(*match_0).mark.is_null() {
        con = con_by_window_id((*window).id);
        if con.is_null() {
            return false_0 != 0;
        }
        let mut matched: bool = false_0 != 0;
        let mut mark: *mut mark_t = ::core::ptr::null_mut::<mark_t>();
        mark = (*con).marks_head.tqh_first as *mut mark_t;
        while !mark.is_null() {
            if regex_matches((*match_0).mark, (*mark).name) {
                matched = true_0 != 0;
                break;
            } else {
                mark = (*mark).marks.tqe_next as *mut mark_t;
            }
        }
        if matched {
            verboselog(
                b"mark matches\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
        } else {
            verboselog(
                b"mark does not match\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
            return false_0 != 0;
        }
    }
    if (*match_0).window_mode as ::core::ffi::c_uint
        != WM_ANY as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        con = con_by_window_id((*window).id);
        if con.is_null() {
            return false_0 != 0;
        }
        match (*match_0).window_mode as ::core::ffi::c_uint {
            1 => {
                if (*con).floating as ::core::ffi::c_uint
                    != FLOATING_AUTO_OFF as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    return false_0 != 0;
                }
            }
            2 => {
                if (*con).floating as ::core::ffi::c_uint
                    != FLOATING_USER_OFF as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    return false_0 != 0;
                }
            }
            3 => {
                if !con_inside_floating(con).is_null() {
                    return false_0 != 0;
                }
            }
            4 => {
                if (*con).floating as ::core::ffi::c_uint
                    != FLOATING_AUTO_ON as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    return false_0 != 0;
                }
            }
            5 => {
                if (*con).floating as ::core::ffi::c_uint
                    != FLOATING_USER_ON as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    return false_0 != 0;
                }
            }
            6 => {
                if con_inside_floating(con).is_null() {
                    return false_0 != 0;
                }
            }
            0 => {
                if 0 as ::core::ffi::c_int != 0 {
                } else {
                    __assert_fail(
                        b"false\0" as *const u8 as *const ::core::ffi::c_char,
                        b"../src/match.c\0" as *const u8 as *const ::core::ffi::c_char,
                        262 as ::core::ffi::c_uint,
                        b"_Bool match_matches_window(Match *, i3Window *)\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                };
            }
            _ => {}
        }
        verboselog(
            b"window_mode matches\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
    }
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn match_free(mut match_0: *mut Match) {
    free((*match_0).error as *mut ::core::ffi::c_void);
    (*match_0).error = ::core::ptr::null_mut::<::core::ffi::c_char>();
    regex_free((*match_0).title);
    regex_free((*match_0).application);
    regex_free((*match_0).class);
    regex_free((*match_0).instance);
    regex_free((*match_0).mark);
    regex_free((*match_0).window_role);
    regex_free((*match_0).workspace);
    regex_free((*match_0).machine);
}
#[no_mangle]
pub unsafe extern "C" fn match_parse_property(
    mut match_0: *mut Match,
    mut ctype: *const ::core::ffi::c_char,
    mut cvalue: *const ::core::ffi::c_char,
) {
    if match_0 != ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut Match {
    } else {
        __assert_fail(
            b"match != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            b"../src/match.c\0" as *const u8 as *const ::core::ffi::c_char,
            296 as ::core::ffi::c_uint,
            b"void match_parse_property(Match *, const char *, const char *)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    };
    debuglog(
        b"%s:%s:%d - ctype=*%s*, cvalue=*%s*\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/match.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"match_parse_property\0" as *const u8 as *const ::core::ffi::c_char,
        297 as ::core::ffi::c_int,
        ctype,
        cvalue,
    );
    if strcmp(ctype, b"class\0" as *const u8 as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        regex_free((*match_0).class);
        (*match_0).class = regex_new(cvalue);
        return;
    }
    if strcmp(
        ctype,
        b"instance\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        regex_free((*match_0).instance);
        (*match_0).instance = regex_new(cvalue);
        return;
    }
    if strcmp(
        ctype,
        b"window_role\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        regex_free((*match_0).window_role);
        (*match_0).window_role = regex_new(cvalue);
        return;
    }
    if strcmp(
        ctype,
        b"con_id\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        if strcmp(
            cvalue,
            b"__focused__\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*match_0).con_id = focused;
            return;
        }
        let mut parsed: ::core::ffi::c_long = 0;
        if !parse_long(cvalue, &raw mut parsed, 0 as ::core::ffi::c_int) {
            errorlog(
                b"ERROR: Could not parse con id \"%s\"\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                cvalue,
            );
            (*match_0).error =
                sstrdup(b"invalid con_id\0" as *const u8 as *const ::core::ffi::c_char);
        } else {
            (*match_0).con_id = parsed as *mut Con;
            debuglog(
                b"%s:%s:%d - id as int = %p\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"../src/match.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"match_parse_property\0" as *const u8 as *const ::core::ffi::c_char,
                329 as ::core::ffi::c_int,
                (*match_0).con_id,
            );
        }
        return;
    }
    if strcmp(ctype, b"id\0" as *const u8 as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int
    {
        let mut parsed_0: ::core::ffi::c_long = 0;
        if !parse_long(cvalue, &raw mut parsed_0, 0 as ::core::ffi::c_int) {
            errorlog(
                b"ERROR: Could not parse window id \"%s\"\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                cvalue,
            );
            (*match_0).error = sstrdup(b"invalid id\0" as *const u8 as *const ::core::ffi::c_char);
        } else {
            (*match_0).id = parsed_0 as xcb_window_t;
            debuglog(
                b"%s:%s:%d - window id as int = %d\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"../src/match.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"match_parse_property\0" as *const u8 as *const ::core::ffi::c_char,
                341 as ::core::ffi::c_int,
                (*match_0).id,
            );
        }
        return;
    }
    if strcmp(
        ctype,
        b"window_type\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        if strcasecmp(
            cvalue,
            b"normal\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*match_0).window_type = A__NET_WM_WINDOW_TYPE_NORMAL;
        } else if strcasecmp(
            cvalue,
            b"dialog\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*match_0).window_type = A__NET_WM_WINDOW_TYPE_DIALOG;
        } else if strcasecmp(
            cvalue,
            b"utility\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*match_0).window_type = A__NET_WM_WINDOW_TYPE_UTILITY;
        } else if strcasecmp(
            cvalue,
            b"toolbar\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*match_0).window_type = A__NET_WM_WINDOW_TYPE_TOOLBAR;
        } else if strcasecmp(
            cvalue,
            b"splash\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*match_0).window_type = A__NET_WM_WINDOW_TYPE_SPLASH;
        } else if strcasecmp(cvalue, b"menu\0" as *const u8 as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            (*match_0).window_type = A__NET_WM_WINDOW_TYPE_MENU;
        } else if strcasecmp(
            cvalue,
            b"dropdown_menu\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*match_0).window_type = A__NET_WM_WINDOW_TYPE_DROPDOWN_MENU;
        } else if strcasecmp(
            cvalue,
            b"popup_menu\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*match_0).window_type = A__NET_WM_WINDOW_TYPE_POPUP_MENU;
        } else if strcasecmp(
            cvalue,
            b"tooltip\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*match_0).window_type = A__NET_WM_WINDOW_TYPE_TOOLTIP;
        } else if strcasecmp(
            cvalue,
            b"notification\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*match_0).window_type = A__NET_WM_WINDOW_TYPE_NOTIFICATION;
        } else {
            errorlog(
                b"ERROR: unknown window_type value \"%s\"\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                cvalue,
            );
            (*match_0).error =
                sstrdup(b"unknown window_type value\0" as *const u8 as *const ::core::ffi::c_char);
        }
        return;
    }
    if strcmp(
        ctype,
        b"con_mark\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        regex_free((*match_0).mark);
        (*match_0).mark = regex_new(cvalue);
        return;
    }
    if strcmp(ctype, b"title\0" as *const u8 as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        regex_free((*match_0).title);
        (*match_0).title = regex_new(cvalue);
        return;
    }
    if strcmp(
        ctype,
        b"urgent\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        if strcasecmp(
            cvalue,
            b"latest\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            || strcasecmp(
                cvalue,
                b"newest\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            || strcasecmp(
                cvalue,
                b"recent\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            || strcasecmp(cvalue, b"last\0" as *const u8 as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
        {
            (*match_0).urgent = U_LATEST;
        } else if strcasecmp(
            cvalue,
            b"oldest\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            || strcasecmp(
                cvalue,
                b"first\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            (*match_0).urgent = U_OLDEST;
        }
        return;
    }
    if strcmp(
        ctype,
        b"workspace\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        regex_free((*match_0).workspace);
        (*match_0).workspace = regex_new(cvalue);
        return;
    }
    if strcmp(
        ctype,
        b"machine\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        regex_free((*match_0).machine);
        (*match_0).machine = regex_new(cvalue);
        return;
    }
    if strcmp(
        ctype,
        b"tiling\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        (*match_0).window_mode = WM_TILING;
        return;
    }
    if strcmp(
        ctype,
        b"tiling_from\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
        && !cvalue.is_null()
        && strcmp(cvalue, b"auto\0" as *const u8 as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
    {
        (*match_0).window_mode = WM_TILING_AUTO;
        return;
    }
    if strcmp(
        ctype,
        b"tiling_from\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
        && !cvalue.is_null()
        && strcmp(cvalue, b"user\0" as *const u8 as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
    {
        (*match_0).window_mode = WM_TILING_USER;
        return;
    }
    if strcmp(
        ctype,
        b"floating\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        (*match_0).window_mode = WM_FLOATING;
        return;
    }
    if strcmp(
        ctype,
        b"floating_from\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
        && !cvalue.is_null()
        && strcmp(cvalue, b"auto\0" as *const u8 as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
    {
        (*match_0).window_mode = WM_FLOATING_AUTO;
        return;
    }
    if strcmp(
        ctype,
        b"floating_from\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
        && !cvalue.is_null()
        && strcmp(cvalue, b"user\0" as *const u8 as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
    {
        (*match_0).window_mode = WM_FLOATING_USER;
        return;
    }
    if strcmp(ctype, b"all\0" as *const u8 as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int
    {
        (*match_0).match_all_windows = true_0 != 0;
        return;
    }
    errorlog(
        b"ERROR: Unknown criterion: %s\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        ctype,
    );
}
