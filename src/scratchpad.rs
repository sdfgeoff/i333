/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * scratchpad.c: Moving windows to the scratchpad and making them visible again.
 *
 */
// Mechanical file-by-file Rust translation of src/scratchpad.c.
// Generated with C2Rust 0.22.1; C ABI and original ownership retained.
extern "C" {
    pub type _cairo;
    pub type _cairo_surface;
    pub type _i3String;
    pub type pcre2_real_code_8;
    pub type ev_loop;
    fn verboselog(fmt: *mut ::core::ffi::c_char, ...);
    fn debuglog(fmt: *mut ::core::ffi::c_char, ...);
    fn rect_equals(a: Rect, b: Rect) -> bool;
    static mut croot: *mut Con;
    static mut focused: *mut Con;
    static mut all_cons: all_cons_head;
    fn workspace_get(num: *const ::core::ffi::c_char) -> *mut Con;
    fn workspace_show(ws: *mut Con);
    fn floating_enable(con: *mut Con, automatic: bool) -> bool;
    fn floating_center(con: *mut Con, rect: Rect);
    fn floating_check_size(floating_con: *mut Con, prefer_height: bool);
    fn floating_fix_coordinates(con: *mut Con, old_rect: *mut Rect, new_rect: *mut Rect);
    fn con_activate(con: *mut Con);
    fn con_get_output(con: *mut Con) -> *mut Con;
    fn con_get_workspace(con: *mut Con) -> *mut Con;
    fn con_is_internal(con: *mut Con) -> bool;
    fn con_inside_floating(con: *mut Con) -> *mut Con;
    fn con_toggle_fullscreen(con: *mut Con, fullscreen_mode: ::core::ffi::c_int);
    fn con_disable_fullscreen(con: *mut Con);
    fn con_move_to_workspace(
        con: *mut Con,
        workspace: *mut Con,
        fix_coordinates: bool,
        dont_warp: bool,
        ignore_focus: bool,
    );
    fn con_descend_focused(con: *mut Con) -> *mut Con;
    fn con_descend_tiling_focused(con: *mut Con) -> *mut Con;
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct all_cons_head {
    pub tqh_first: *mut Con,
    pub tqh_last: *mut *mut Con,
}
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[no_mangle]
pub unsafe extern "C" fn scratchpad_move(mut con: *mut Con) {
    if (*con).type_0 as ::core::ffi::c_uint
        == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        verboselog(
            b"'move scratchpad' used on a workspace \"%s\". Calling it recursively on all windows on this workspace.\n\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            (*con).name,
        );
        let mut current: *mut Con = (*con).focus_head.tqh_first as *mut Con;
        while !current.is_null() {
            let mut next: *mut Con = (*current).focused.tqe_next as *mut Con;
            scratchpad_move(current);
            current = next;
        }
        return;
    }
    debuglog(
        b"%s:%s:%d - should move con %p to __i3_scratch\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/scratchpad.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"scratchpad_move\0" as *const u8 as *const ::core::ffi::c_char,
        32 as ::core::ffi::c_int,
        con,
    );
    let mut __i3_scratch: *mut Con =
        workspace_get(b"__i3_scratch\0" as *const u8 as *const ::core::ffi::c_char);
    if con_get_workspace(con) == __i3_scratch {
        debuglog(
            b"%s:%s:%d - This window is already on __i3_scratch.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/scratchpad.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"scratchpad_move\0" as *const u8 as *const ::core::ffi::c_char,
            36 as ::core::ffi::c_int,
        );
        return;
    }
    con_disable_fullscreen(con);
    let mut maybe_floating_con: *mut Con = con_inside_floating(con);
    if maybe_floating_con.is_null() {
        floating_enable(con, false_0 != 0);
        con = (*con).parent as *mut Con;
    } else {
        con = maybe_floating_con;
    }
    con_move_to_workspace(con, __i3_scratch, true_0 != 0, true_0 != 0, false_0 != 0);
    if (*con).scratchpad_state as ::core::ffi::c_uint
        == SCRATCHPAD_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        debuglog(
            b"%s:%s:%d - This window was never used as a scratchpad before.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/scratchpad.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"scratchpad_move\0" as *const u8 as *const ::core::ffi::c_char,
            64 as ::core::ffi::c_int,
        );
        if con == maybe_floating_con {
            debuglog(
                b"%s:%s:%d - It was in floating mode before, set scratchpad state to changed.\n\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"../src/scratchpad.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"scratchpad_move\0" as *const u8 as *const ::core::ffi::c_char,
                66 as ::core::ffi::c_int,
            );
            (*con).scratchpad_state = SCRATCHPAD_CHANGED;
        } else {
            debuglog(
                b"%s:%s:%d - It was in tiling mode before, set scratchpad state to fresh.\n\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"../src/scratchpad.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"scratchpad_move\0" as *const u8 as *const ::core::ffi::c_char,
                69 as ::core::ffi::c_int,
            );
            (*con).scratchpad_state = SCRATCHPAD_FRESH;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn scratchpad_show(mut con: *mut Con) -> bool {
    debuglog(
        b"%s:%s:%d - should show scratchpad window %p\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/scratchpad.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"scratchpad_show\0" as *const u8 as *const ::core::ffi::c_char,
        85 as ::core::ffi::c_int,
        con,
    );
    let mut __i3_scratch: *mut Con =
        workspace_get(b"__i3_scratch\0" as *const u8 as *const ::core::ffi::c_char);
    let mut floating: *mut Con = ::core::ptr::null_mut::<Con>();
    if con.is_null()
        && {
            floating = con_inside_floating(focused);
            !floating.is_null()
        }
        && (*floating).scratchpad_state as ::core::ffi::c_uint
            != SCRATCHPAD_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        debuglog(
            b"%s:%s:%d - Focused window is a scratchpad window, hiding it.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/scratchpad.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"scratchpad_show\0" as *const u8 as *const ::core::ffi::c_char,
            95 as ::core::ffi::c_int,
        );
        scratchpad_move(focused);
        return true_0 != 0;
    }
    let mut fs: *mut Con = focused;
    while !fs.is_null()
        && (*fs).fullscreen_mode as ::core::ffi::c_uint
            == CF_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        fs = (*fs).parent as *mut Con;
    }
    if !fs.is_null()
        && (*fs).type_0 as ::core::ffi::c_uint
            != CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        con_toggle_fullscreen(fs, CF_OUTPUT as ::core::ffi::c_int);
    }
    let mut walk_con: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut focused_ws: *mut Con = con_get_workspace(focused);
    walk_con = (*focused_ws).floating_head.tqh_first as *mut Con;
    while !walk_con.is_null() {
        if con.is_null()
            && {
                floating = con_inside_floating(walk_con);
                !floating.is_null()
            }
            && (*floating).scratchpad_state as ::core::ffi::c_uint
                != SCRATCHPAD_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
            && floating != con_inside_floating(focused)
        {
            debuglog(
                b"%s:%s:%d - Found an unfocused scratchpad window on this workspace\n\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"../src/scratchpad.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"scratchpad_show\0" as *const u8 as *const ::core::ffi::c_char,
                119 as ::core::ffi::c_int,
            );
            debuglog(
                b"%s:%s:%d - Focusing it: %p\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"../src/scratchpad.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"scratchpad_show\0" as *const u8 as *const ::core::ffi::c_char,
                120 as ::core::ffi::c_int,
                walk_con,
            );
            con_activate(con_descend_tiling_focused(walk_con));
            return true_0 != 0;
        }
        walk_con = (*walk_con).floating_windows.tqe_next as *mut Con;
    }
    focused_ws = con_get_workspace(focused);
    walk_con = all_cons.tqh_first as *mut Con;
    while !walk_con.is_null() {
        let mut walk_ws: *mut Con = con_get_workspace(walk_con);
        if con.is_null()
            && !walk_ws.is_null()
            && !con_is_internal(walk_ws)
            && focused_ws != walk_ws
            && {
                floating = con_inside_floating(walk_con);
                !floating.is_null()
            }
            && (*floating).scratchpad_state as ::core::ffi::c_uint
                != SCRATCHPAD_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            debuglog(
                b"%s:%s:%d - Found a visible scratchpad window on another workspace,\n\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"../src/scratchpad.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"scratchpad_show\0" as *const u8 as *const ::core::ffi::c_char,
                139 as ::core::ffi::c_int,
            );
            debuglog(
                b"%s:%s:%d - moving it to this workspace: con = %p\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"../src/scratchpad.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"scratchpad_show\0" as *const u8 as *const ::core::ffi::c_char,
                140 as ::core::ffi::c_int,
                walk_con,
            );
            con_move_to_workspace(
                floating,
                focused_ws,
                true_0 != 0,
                false_0 != 0,
                false_0 != 0,
            );
            con_activate(con_descend_focused(walk_con));
            return true_0 != 0;
        }
        walk_con = (*walk_con).all_cons.tqe_next as *mut Con;
    }
    if !con.is_null()
        && (*(*con).parent).scratchpad_state as ::core::ffi::c_uint
            == SCRATCHPAD_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        debuglog(
            b"%s:%s:%d - Window is not in the scratchpad, doing nothing.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/scratchpad.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"scratchpad_show\0" as *const u8 as *const ::core::ffi::c_char,
            150 as ::core::ffi::c_int,
        );
        return false_0 != 0;
    }
    let mut active: *mut Con = con_get_workspace(focused);
    let mut current: *mut Con = con_get_workspace(con);
    if !con.is_null()
        && {
            floating = con_inside_floating(con);
            !floating.is_null()
        }
        && (*floating).scratchpad_state as ::core::ffi::c_uint
            != SCRATCHPAD_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
        && current != __i3_scratch
    {
        if current == active {
            debuglog(
                b"%s:%s:%d - Window is a scratchpad window, hiding it.\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"../src/scratchpad.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"scratchpad_show\0" as *const u8 as *const ::core::ffi::c_char,
                165 as ::core::ffi::c_int,
            );
            scratchpad_move(con);
            return true_0 != 0;
        }
    }
    if con.is_null() {
        con = (*__i3_scratch).floating_head.tqh_first as *mut Con;
        if con.is_null() {
            verboselog(
                b"You don't have any scratchpad windows yet.\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
            verboselog(
                b"Use 'move scratchpad' to move a window to the scratchpad.\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
            return false_0 != 0;
        }
    } else {
        con = con_inside_floating(con);
    }
    con_move_to_workspace(con, active, true_0 != 0, false_0 != 0, false_0 != 0);
    if (*con).scratchpad_state as ::core::ffi::c_uint
        == SCRATCHPAD_FRESH as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        debuglog(
            b"%s:%s:%d - Adjusting size of this window.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/scratchpad.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"scratchpad_show\0" as *const u8 as *const ::core::ffi::c_char,
            193 as ::core::ffi::c_int,
        );
        let mut output: *mut Con = con_get_output(con);
        (*con).rect.width = ((*output).rect.width as ::core::ffi::c_double * 0.5f64) as uint32_t;
        (*con).rect.height = ((*output).rect.height as ::core::ffi::c_double * 0.75f64) as uint32_t;
        floating_check_size(con, false_0 != 0);
        floating_center(con, (*con_get_workspace(con)).rect as Rect);
    }
    if current != active {
        workspace_show(active);
    }
    con_activate(con_descend_focused(con));
    return true_0 != 0;
}
unsafe extern "C" fn _gcd(m: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int {
    if n == 0 as ::core::ffi::c_int {
        return m;
    }
    return _gcd(n, m % n);
}
unsafe extern "C" fn _lcm(m: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let o: ::core::ffi::c_int = _gcd(m, n) as ::core::ffi::c_int;
    return m * n / o;
}
#[no_mangle]
pub unsafe extern "C" fn scratchpad_fix_resolution() {
    let mut __i3_scratch: *mut Con =
        workspace_get(b"__i3_scratch\0" as *const u8 as *const ::core::ffi::c_char);
    let mut __i3_output: *mut Con = con_get_output(__i3_scratch);
    debuglog(
        b"%s:%s:%d - Current resolution: (%d, %d) %d x %d\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/scratchpad.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"scratchpad_fix_resolution\0" as *const u8 as *const ::core::ffi::c_char,
        253 as ::core::ffi::c_int,
        (*__i3_output).rect.x,
        (*__i3_output).rect.y,
        (*__i3_output).rect.width,
        (*__i3_output).rect.height,
    );
    let mut output: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut new_width: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut new_height: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    output = (*croot).nodes_head.tqh_first as *mut Con;
    while !output.is_null() {
        if !(output == __i3_output) {
            debuglog(
                b"%s:%s:%d - output %s's resolution: (%d, %d) %d x %d\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"../src/scratchpad.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"scratchpad_fix_resolution\0" as *const u8 as *const ::core::ffi::c_char,
                263 as ::core::ffi::c_int,
                (*output).name,
                (*output).rect.x,
                (*output).rect.y,
                (*output).rect.width,
                (*output).rect.height,
            );
            if new_width == -(1 as ::core::ffi::c_int) {
                new_width = (*output).rect.width as ::core::ffi::c_int;
                new_height = (*output).rect.height as ::core::ffi::c_int;
            } else {
                new_width = _lcm(new_width, (*output).rect.width as ::core::ffi::c_int);
                new_height = _lcm(new_height, (*output).rect.height as ::core::ffi::c_int);
            }
        }
        output = (*output).nodes.tqe_next as *mut Con;
    }
    let mut old_rect: Rect = (*__i3_output).rect as Rect;
    debuglog(
        b"%s:%s:%d - new width = %d, new height = %d\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/scratchpad.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"scratchpad_fix_resolution\0" as *const u8 as *const ::core::ffi::c_char,
        276 as ::core::ffi::c_int,
        new_width,
        new_height,
    );
    (*__i3_output).rect.width = new_width as uint32_t;
    (*__i3_output).rect.height = new_height as uint32_t;
    let mut new_rect: Rect = (*__i3_output).rect as Rect;
    if rect_equals(new_rect, old_rect) {
        debuglog(
            b"%s:%s:%d - Scratchpad size unchanged.\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b"../src/scratchpad.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"scratchpad_fix_resolution\0" as *const u8 as *const ::core::ffi::c_char,
            283 as ::core::ffi::c_int,
        );
        return;
    }
    debuglog(
        b"%s:%s:%d - Fixing coordinates of scratchpad windows\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/scratchpad.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"scratchpad_fix_resolution\0" as *const u8 as *const ::core::ffi::c_char,
        287 as ::core::ffi::c_int,
    );
    let mut con: *mut Con = ::core::ptr::null_mut::<Con>();
    con = (*__i3_scratch).floating_head.tqh_first as *mut Con;
    while !con.is_null() {
        floating_fix_coordinates(con, &raw mut old_rect, &raw mut new_rect);
        con = (*con).floating_windows.tqe_next as *mut Con;
    }
}
