/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * move.c: Moving containers into some direction.
 *
 */
// Mechanical file-by-file Rust translation of src/move.c.
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
    fn orientation_from_direction(direction: direction_t) -> orientation_t;
    static mut croot: *mut Con;
    static mut focused: *mut Con;
    fn tree_flatten(child: *mut Con);
    fn ipc_send_window_event(property: *const ::core::ffi::c_char, con: *mut Con);
    fn get_output_next(
        direction: direction_t,
        current: *mut Output,
        close_far: output_close_far_t,
    ) -> *mut Output;
    fn workspace_is_visible(ws: *mut Con) -> bool;
    fn workspace_show(ws: *mut Con);
    fn ws_force_orientation(ws: *mut Con, orientation: orientation_t);
    fn workspace_attach_to(ws: *mut Con) -> *mut Con;
    fn floating_disable(con: *mut Con);
    fn con_focus(con: *mut Con);
    fn con_is_leaf(con: *mut Con) -> bool;
    fn con_get_workspace(con: *mut Con) -> *mut Con;
    fn con_parent_with_orientation(con: *mut Con, orientation: orientation_t) -> *mut Con;
    fn con_is_floating(con: *mut Con) -> bool;
    fn con_inside_floating(con: *mut Con) -> *mut Con;
    fn con_num_children(con: *mut Con) -> ::core::ffi::c_int;
    fn con_attach(con: *mut Con, parent: *mut Con, ignore_focus: bool);
    fn con_detach(con: *mut Con);
    fn con_fix_percent(con: *mut Con);
    fn con_orientation(con: *mut Con) -> orientation_t;
    fn con_descend_direction(con: *mut Con, direction: direction_t) -> *mut Con;
    fn con_fullscreen_permits_focusing(con: *mut Con) -> bool;
    fn con_force_split_parents_redraw(con: *mut Con);
    fn output_get_content(output: *mut Con) -> *mut Con;
    fn get_output_for_con(con: *mut Con) -> *mut Output;
    fn ewmh_update_wm_desktop();
    fn run_assignments(window: *mut i3Window);
    fn free(__ptr: *mut ::core::ffi::c_void);
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
    pub outputs: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
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
    pub type_0: C2RustUnnamed_16,
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
    pub floating: C2RustUnnamed_5,
    pub nodes: C2RustUnnamed_4,
    pub focused: C2RustUnnamed_3,
    pub all_cons: C2RustUnnamed_2,
    pub floating_windows: C2RustUnnamed_1,
    pub on_remove_child: Option<unsafe extern "C" fn(*mut Con) -> ()>,
    pub scratchpad_state: C2RustUnnamed_0,
    pub old_id: ::core::ffi::c_int,
    pub depth: uint16_t,
    pub colormap: xcb_colormap_t,
}
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const SCRATCHPAD_CHANGED: C2RustUnnamed_0 = 2;
pub const SCRATCHPAD_FRESH: C2RustUnnamed_0 = 1;
pub const SCRATCHPAD_NONE: C2RustUnnamed_0 = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub tqe_next: *mut Con,
    pub tqe_prev: *mut *mut Con,
}
pub type C2RustUnnamed_5 = ::core::ffi::c_uint;
pub const FLOATING_USER_ON: C2RustUnnamed_5 = 3;
pub const FLOATING_AUTO_ON: C2RustUnnamed_5 = 2;
pub const FLOATING_USER_OFF: C2RustUnnamed_5 = 1;
pub const FLOATING_AUTO_OFF: C2RustUnnamed_5 = 0;
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
    pub urgent: C2RustUnnamed_10,
    pub dock: C2RustUnnamed_9,
    pub id: xcb_window_t,
    pub window_mode: C2RustUnnamed_8,
    pub con_id: *mut Con,
    pub match_all_windows: bool,
    pub insert_where: C2RustUnnamed_7,
    pub matches: C2RustUnnamed_6,
    pub restart_mode: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub tqe_next: *mut Match,
    pub tqe_prev: *mut *mut Match,
}
pub type C2RustUnnamed_7 = ::core::ffi::c_uint;
pub const M_BELOW: C2RustUnnamed_7 = 2;
pub const M_ASSIGN_WS: C2RustUnnamed_7 = 1;
pub const M_HERE: C2RustUnnamed_7 = 0;
pub type C2RustUnnamed_8 = ::core::ffi::c_uint;
pub const WM_FLOATING: C2RustUnnamed_8 = 6;
pub const WM_FLOATING_USER: C2RustUnnamed_8 = 5;
pub const WM_FLOATING_AUTO: C2RustUnnamed_8 = 4;
pub const WM_TILING: C2RustUnnamed_8 = 3;
pub const WM_TILING_USER: C2RustUnnamed_8 = 2;
pub const WM_TILING_AUTO: C2RustUnnamed_8 = 1;
pub const WM_ANY: C2RustUnnamed_8 = 0;
pub type C2RustUnnamed_9 = ::core::ffi::c_int;
pub const M_DOCK_BOTTOM: C2RustUnnamed_9 = 3;
pub const M_DOCK_TOP: C2RustUnnamed_9 = 2;
pub const M_DOCK_ANY: C2RustUnnamed_9 = 1;
pub const M_NODOCK: C2RustUnnamed_9 = 0;
pub const M_DONTCHECK: C2RustUnnamed_9 = -1;
pub type C2RustUnnamed_10 = ::core::ffi::c_int;
pub const U_OLDEST: C2RustUnnamed_10 = 1;
pub const U_LATEST: C2RustUnnamed_10 = 0;
pub const U_DONTCHECK: C2RustUnnamed_10 = -1;
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
    pub dock: C2RustUnnamed_11,
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
pub type C2RustUnnamed_11 = ::core::ffi::c_uint;
pub const W_DOCK_BOTTOM: C2RustUnnamed_11 = 2;
pub const W_DOCK_TOP: C2RustUnnamed_11 = 1;
pub const W_NODOCK: C2RustUnnamed_11 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Assignment {
    pub type_0: C2RustUnnamed_14,
    pub match_0: Match,
    pub dest: C2RustUnnamed_13,
    pub assignments: C2RustUnnamed_12,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub tqe_next: *mut Assignment,
    pub tqe_prev: *mut *mut Assignment,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_13 {
    pub command: *mut ::core::ffi::c_char,
    pub workspace: *mut ::core::ffi::c_char,
    pub output: *mut ::core::ffi::c_char,
}
pub type C2RustUnnamed_14 = ::core::ffi::c_uint;
pub const A_TO_OUTPUT: C2RustUnnamed_14 = 16;
pub const A_TO_WORKSPACE_NUMBER: C2RustUnnamed_14 = 8;
pub const A_NO_FOCUS: C2RustUnnamed_14 = 4;
pub const A_TO_WORKSPACE: C2RustUnnamed_14 = 2;
pub const A_COMMAND: C2RustUnnamed_14 = 1;
pub const A_ANY: C2RustUnnamed_14 = 0;
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
    pub marks: C2RustUnnamed_15,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_15 {
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
pub type C2RustUnnamed_16 = ::core::ffi::c_uint;
pub const CT_DOCKAREA: C2RustUnnamed_16 = 5;
pub const CT_WORKSPACE: C2RustUnnamed_16 = 4;
pub const CT_FLOATING_CON: C2RustUnnamed_16 = 3;
pub const CT_CON: C2RustUnnamed_16 = 2;
pub const CT_OUTPUT: C2RustUnnamed_16 = 1;
pub const CT_ROOT: C2RustUnnamed_16 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct names_head {
    pub slh_first: *mut output_name,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct output_name {
    pub name: *mut ::core::ffi::c_char,
    pub names: C2RustUnnamed_17,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_17 {
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
pub type output_close_far_t = ::core::ffi::c_uint;
pub const FARTHEST_OUTPUT: output_close_far_t = 1;
pub const CLOSEST_OUTPUT: output_close_far_t = 0;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
unsafe extern "C" fn lowest_common_ancestor(mut a: *mut Con, mut b: *mut Con) -> *mut Con {
    let mut parent_a: *mut Con = a;
    while !parent_a.is_null() {
        let mut parent_b: *mut Con = b;
        while !parent_b.is_null() {
            if parent_a == parent_b {
                return parent_a;
            }
            parent_b = (*parent_b).parent as *mut Con;
        }
        parent_a = (*parent_a).parent as *mut Con;
    }
    if 0 as ::core::ffi::c_int != 0 {
    } else {
        __assert_fail(
            b"false\0" as *const u8 as *const ::core::ffi::c_char,
            b"../src/move.c\0" as *const u8 as *const ::core::ffi::c_char,
            28 as ::core::ffi::c_uint,
            b"Con *lowest_common_ancestor(Con *, Con *)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    };
    panic!("Reached end of non-void function without returning");
}
unsafe extern "C" fn child_containing_con_recursively(
    mut ancestor: *mut Con,
    mut con: *mut Con,
) -> *mut Con {
    let mut child: *mut Con = con;
    while !child.is_null() && (*child).parent != ancestor {
        child = (*child).parent as *mut Con;
        if !(*child).parent.is_null() {
        } else {
            __assert_fail(
                b"child->parent\0" as *const u8 as *const ::core::ffi::c_char,
                b"../src/move.c\0" as *const u8 as *const ::core::ffi::c_char,
                39 as ::core::ffi::c_uint,
                b"Con *child_containing_con_recursively(Con *, Con *)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        };
    }
    return child;
}
unsafe extern "C" fn is_focused_descendant(mut con: *mut Con, mut ancestor: *mut Con) -> bool {
    let mut current: *mut Con = con;
    while current != ancestor {
        if (*(*current).parent).focus_head.tqh_first != current {
            return false_0 != 0;
        }
        current = (*current).parent as *mut Con;
        if !(*current).parent.is_null() {
        } else {
            __assert_fail(
                b"current->parent\0" as *const u8 as *const ::core::ffi::c_char,
                b"../src/move.c\0" as *const u8 as *const ::core::ffi::c_char,
                55 as ::core::ffi::c_uint,
                b"_Bool is_focused_descendant(Con *, Con *)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        };
    }
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn insert_con_into(
    mut con: *mut Con,
    mut target: *mut Con,
    mut position: position_t,
) {
    let mut parent: *mut Con = (*target).parent as *mut Con;
    let mut old_parent: *mut Con = (*con).parent as *mut Con;
    let mut lca: *mut Con = lowest_common_ancestor(con, parent);
    if lca == con {
        errorlog(
            b"ERROR: Container is being inserted into one of its descendants.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        return;
    }
    let mut con_ancestor: *mut Con = child_containing_con_recursively(lca, con);
    let mut target_ancestor: *mut Con = child_containing_con_recursively(lca, target);
    let mut moves_focus_from_ancestor: bool = is_focused_descendant(con, con_ancestor);
    let mut focus_before: bool = false;
    if con_ancestor == target_ancestor {
        focus_before = moves_focus_from_ancestor;
    } else {
        let mut current: *mut Con = ::core::ptr::null_mut::<Con>();
        current = (*lca).focus_head.tqh_first as *mut Con;
        while !current.is_null() {
            if current == con_ancestor || current == target_ancestor {
                break;
            }
            current = (*current).focused.tqe_next as *mut Con;
        }
        focus_before = current == con_ancestor;
    }
    if moves_focus_from_ancestor as ::core::ffi::c_int != 0
        && focus_before as ::core::ffi::c_int != 0
    {
        let mut place: *mut Con =
            *(*((*con_ancestor).focused.tqe_prev as *mut focus_head)).tqh_last;
        if !(*target_ancestor).focused.tqe_next.is_null() {
            (*(*target_ancestor).focused.tqe_next).focused.tqe_prev =
                (*target_ancestor).focused.tqe_prev;
        } else {
            (*lca).focus_head.tqh_last = (*target_ancestor).focused.tqe_prev;
        }
        *(*target_ancestor).focused.tqe_prev = (*target_ancestor).focused.tqe_next;
        if !place.is_null() {
            (*target_ancestor).focused.tqe_next = (*place).focused.tqe_next;
            if !(*target_ancestor).focused.tqe_next.is_null() {
                (*(*target_ancestor).focused.tqe_next).focused.tqe_prev =
                    &raw mut (*target_ancestor).focused.tqe_next;
            } else {
                (*lca).focus_head.tqh_last = &raw mut (*target_ancestor).focused.tqe_next;
            }
            (*place).focused.tqe_next = target_ancestor as *mut Con;
            (*target_ancestor).focused.tqe_prev = &raw mut (*place).focused.tqe_next;
        } else {
            (*target_ancestor).focused.tqe_next = (*lca).focus_head.tqh_first;
            if !(*target_ancestor).focused.tqe_next.is_null() {
                (*(*lca).focus_head.tqh_first).focused.tqe_prev =
                    &raw mut (*target_ancestor).focused.tqe_next;
            } else {
                (*lca).focus_head.tqh_last = &raw mut (*target_ancestor).focused.tqe_next;
            }
            (*lca).focus_head.tqh_first = target_ancestor as *mut Con;
            (*target_ancestor).focused.tqe_prev = &raw mut (*lca).focus_head.tqh_first;
        }
    }
    con_detach(con);
    con_fix_percent((*con).parent as *mut Con);
    if (*parent).type_0 as ::core::ffi::c_uint
        == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut split: *mut Con = workspace_attach_to(parent);
        if split != parent {
            debuglog(
                b"%s:%s:%d - Got a new split con, using that one instead\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"../src/move.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"insert_con_into\0" as *const u8 as *const ::core::ffi::c_char,
                126 as ::core::ffi::c_int,
            );
            (*con).parent = split as *mut Con;
            con_attach(con, split, false_0 != 0);
            debuglog(
                b"%s:%s:%d - attached\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"../src/move.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"insert_con_into\0" as *const u8 as *const ::core::ffi::c_char,
                129 as ::core::ffi::c_int,
            );
            (*con).percent = 0.0f64;
            con_fix_percent(split);
            con = split;
            debuglog(
                b"%s:%s:%d - ok, continuing with con %p instead\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"../src/move.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"insert_con_into\0" as *const u8 as *const ::core::ffi::c_char,
                133 as ::core::ffi::c_int,
                con,
            );
            con_detach(con);
        }
    }
    (*con).parent = parent as *mut Con;
    if parent == lca {
        if focus_before {
            (*con).focused.tqe_prev = (*target).focused.tqe_prev;
            (*con).focused.tqe_next = target as *mut Con;
            *(*target).focused.tqe_prev = con as *mut Con;
            (*target).focused.tqe_prev = &raw mut (*con).focused.tqe_next;
        } else {
            (*con).focused.tqe_next = (*target).focused.tqe_next;
            if !(*con).focused.tqe_next.is_null() {
                (*(*con).focused.tqe_next).focused.tqe_prev = &raw mut (*con).focused.tqe_next;
            } else {
                (*parent).focus_head.tqh_last = &raw mut (*con).focused.tqe_next;
            }
            (*target).focused.tqe_next = con as *mut Con;
            (*con).focused.tqe_prev = &raw mut (*target).focused.tqe_next;
        }
    } else if focus_before {
        (*con).focused.tqe_next = (*parent).focus_head.tqh_first;
        if !(*con).focused.tqe_next.is_null() {
            (*(*parent).focus_head.tqh_first).focused.tqe_prev = &raw mut (*con).focused.tqe_next;
        } else {
            (*parent).focus_head.tqh_last = &raw mut (*con).focused.tqe_next;
        }
        (*parent).focus_head.tqh_first = con as *mut Con;
        (*con).focused.tqe_prev = &raw mut (*parent).focus_head.tqh_first;
    } else {
        (*con).focused.tqe_next = ::core::ptr::null_mut::<Con>();
        (*con).focused.tqe_prev = (*parent).focus_head.tqh_last;
        *(*parent).focus_head.tqh_last = con as *mut Con;
        (*parent).focus_head.tqh_last = &raw mut (*con).focused.tqe_next;
    }
    if position as ::core::ffi::c_uint == BEFORE as ::core::ffi::c_int as ::core::ffi::c_uint {
        (*con).nodes.tqe_prev = (*target).nodes.tqe_prev;
        (*con).nodes.tqe_next = target as *mut Con;
        *(*target).nodes.tqe_prev = con as *mut Con;
        (*target).nodes.tqe_prev = &raw mut (*con).nodes.tqe_next;
    } else if position as ::core::ffi::c_uint == AFTER as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*con).nodes.tqe_next = (*target).nodes.tqe_next;
        if !(*con).nodes.tqe_next.is_null() {
            (*(*con).nodes.tqe_next).nodes.tqe_prev = &raw mut (*con).nodes.tqe_next;
        } else {
            (*parent).nodes_head.tqh_last = &raw mut (*con).nodes.tqe_next;
        }
        (*target).nodes.tqe_next = con as *mut Con;
        (*con).nodes.tqe_prev = &raw mut (*target).nodes.tqe_next;
    }
    (*con).percent = 0.0f64;
    con_fix_percent(parent);
    (*old_parent)
        .on_remove_child
        .expect("non-null function pointer")(old_parent);
}
unsafe extern "C" fn attach_to_workspace(
    mut con: *mut Con,
    mut ws: *mut Con,
    mut direction: direction_t,
) {
    con_detach(con);
    let mut old_parent: *mut Con = (*con).parent as *mut Con;
    (*con).parent = ws as *mut Con;
    if direction as ::core::ffi::c_uint == D_RIGHT as ::core::ffi::c_int as ::core::ffi::c_uint
        || direction as ::core::ffi::c_uint == D_DOWN as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*con).nodes.tqe_next = (*ws).nodes_head.tqh_first;
        if !(*con).nodes.tqe_next.is_null() {
            (*(*ws).nodes_head.tqh_first).nodes.tqe_prev = &raw mut (*con).nodes.tqe_next;
        } else {
            (*ws).nodes_head.tqh_last = &raw mut (*con).nodes.tqe_next;
        }
        (*ws).nodes_head.tqh_first = con as *mut Con;
        (*con).nodes.tqe_prev = &raw mut (*ws).nodes_head.tqh_first;
    } else {
        (*con).nodes.tqe_next = ::core::ptr::null_mut::<Con>();
        (*con).nodes.tqe_prev = (*ws).nodes_head.tqh_last;
        *(*ws).nodes_head.tqh_last = con as *mut Con;
        (*ws).nodes_head.tqh_last = &raw mut (*con).nodes.tqe_next;
    }
    (*con).focused.tqe_next = ::core::ptr::null_mut::<Con>();
    (*con).focused.tqe_prev = (*ws).focus_head.tqh_last;
    *(*ws).focus_head.tqh_last = con as *mut Con;
    (*ws).focus_head.tqh_last = &raw mut (*con).focused.tqe_next;
    (*con).percent = 0.0f64;
    con_fix_percent(ws);
    con_fix_percent(old_parent);
    (*old_parent)
        .on_remove_child
        .expect("non-null function pointer")(old_parent);
}
unsafe extern "C" fn move_to_output_directed(mut con: *mut Con, mut direction: direction_t) {
    let mut current_output: *mut Output = get_output_for_con(con);
    let mut output: *mut Output = get_output_next(direction, current_output, CLOSEST_OUTPUT);
    if output.is_null() {
        debuglog(
            b"%s:%s:%d - No output in this direction found. Not moving.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/move.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"move_to_output_directed\0" as *const u8 as *const ::core::ffi::c_char,
            211 as ::core::ffi::c_int,
        );
        return;
    }
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
    if ws.is_null() {
        debuglog(
            b"%s:%s:%d - No workspace on output in this direction found. Not moving.\n\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/move.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"move_to_output_directed\0" as *const u8 as *const ::core::ffi::c_char,
            219 as ::core::ffi::c_int,
        );
        return;
    }
    let mut old_ws: *mut Con = con_get_workspace(con);
    let moves_focus: bool = focused == con;
    attach_to_workspace(con, ws, direction);
    if moves_focus {
        con_focus(con);
        focused = old_ws;
        workspace_show(ws);
        con_focus(con);
    }
    free((*con).deco_render_params as *mut ::core::ffi::c_void);
    (*con).deco_render_params = ::core::ptr::null_mut::<deco_render_params>();
    ipc_send_window_event(b"move\0" as *const u8 as *const ::core::ffi::c_char, con);
    tree_flatten(croot);
    ewmh_update_wm_desktop();
}
#[no_mangle]
pub unsafe extern "C" fn tree_move(mut con: *mut Con, mut direction: direction_t) {
    let mut above: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut next: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut current_block: u64;
    let mut position: position_t = BEFORE;
    let mut target: *mut Con = ::core::ptr::null_mut::<Con>();
    debuglog(
        b"%s:%s:%d - Moving in direction %d\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/move.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"tree_move\0" as *const u8 as *const ::core::ffi::c_char,
        263 as ::core::ffi::c_int,
        direction as ::core::ffi::c_uint,
    );
    if (*con).type_0 as ::core::ffi::c_uint
        == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        debuglog(
            b"%s:%s:%d - Not moving workspace\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b"../src/move.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"tree_move\0" as *const u8 as *const ::core::ffi::c_char,
            268 as ::core::ffi::c_int,
        );
        return;
    }
    if (*con).fullscreen_mode as ::core::ffi::c_uint
        == CF_GLOBAL as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        debuglog(
            b"%s:%s:%d - Not moving fullscreen global container\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/move.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"tree_move\0" as *const u8 as *const ::core::ffi::c_char,
            273 as ::core::ffi::c_int,
        );
        return;
    }
    if (*con).fullscreen_mode as ::core::ffi::c_uint
        == CF_OUTPUT as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*(*con).parent).type_0 as ::core::ffi::c_uint
            == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
            && con_num_children((*con).parent as *mut Con) == 1 as ::core::ffi::c_int
    {
        move_to_output_directed(con, direction);
        return;
    }
    let mut o: orientation_t = orientation_from_direction(direction);
    let mut same_orientation: *mut Con = con_parent_with_orientation(con, o);
    loop {
        if same_orientation.is_null() {
            if con_is_floating(con) {
                floating_disable(con);
                run_assignments((*con).window as *mut i3Window);
                return;
            }
            if !con_inside_floating(con).is_null() {
                debuglog(
                    b"%s:%s:%d - Inside floating, moving to workspace\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    b"../src/move.c\0" as *const u8 as *const ::core::ffi::c_char,
                    b"tree_move\0" as *const u8 as *const ::core::ffi::c_char,
                    301 as ::core::ffi::c_int,
                );
                attach_to_workspace(con, con_get_workspace(con), direction);
                current_block = 11393196477283502759;
                break;
            } else {
                debuglog(
                    b"%s:%s:%d - Force-changing orientation\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    b"../src/move.c\0" as *const u8 as *const ::core::ffi::c_char,
                    b"tree_move\0" as *const u8 as *const ::core::ffi::c_char,
                    305 as ::core::ffi::c_int,
                );
                ws_force_orientation(con_get_workspace(con), o);
                same_orientation = con_parent_with_orientation(con, o);
            }
        }
        if same_orientation == (*con).parent {
            let mut swap: *mut Con = if direction as ::core::ffi::c_uint
                == D_LEFT as ::core::ffi::c_int as ::core::ffi::c_uint
                || direction as ::core::ffi::c_uint
                    == D_UP as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                *(*((*con).nodes.tqe_prev as *mut nodes_head)).tqh_last
            } else {
                (*con).nodes.tqe_next as *mut Con
            };
            if !swap.is_null() {
                if !con_is_leaf(swap) {
                    debuglog(
                        b"%s:%s:%d - Moving into our bordering branch\n\0" as *const u8
                            as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        b"../src/move.c\0" as *const u8 as *const ::core::ffi::c_char,
                        b"tree_move\0" as *const u8 as *const ::core::ffi::c_char,
                        317 as ::core::ffi::c_int,
                    );
                    target = con_descend_direction(swap, direction);
                    position = (if con_orientation((*target).parent as *mut Con)
                        as ::core::ffi::c_uint
                        != o as ::core::ffi::c_uint
                        || direction as ::core::ffi::c_uint
                            == D_UP as ::core::ffi::c_int as ::core::ffi::c_uint
                        || direction as ::core::ffi::c_uint
                            == D_LEFT as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        AFTER as ::core::ffi::c_int
                    } else {
                        BEFORE as ::core::ffi::c_int
                    }) as position_t;
                    insert_con_into(con, target, position);
                    current_block = 11393196477283502759;
                    break;
                } else {
                    debuglog(
                        b"%s:%s:%d - Swapping with sibling.\n\0" as *const u8
                            as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        b"../src/move.c\0" as *const u8 as *const ::core::ffi::c_char,
                        b"tree_move\0" as *const u8 as *const ::core::ffi::c_char,
                        328 as ::core::ffi::c_int,
                    );
                    if direction as ::core::ffi::c_uint
                        == D_LEFT as ::core::ffi::c_int as ::core::ffi::c_uint
                        || direction as ::core::ffi::c_uint
                            == D_UP as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        *(*swap).nodes.tqe_prev = con as *mut Con;
                        (*con).nodes.tqe_prev = (*swap).nodes.tqe_prev;
                        (*swap).nodes.tqe_prev = &raw mut (*con).nodes.tqe_next;
                        (*swap).nodes.tqe_next = (*con).nodes.tqe_next;
                        if !(*con).nodes.tqe_next.is_null() {
                            (*(*con).nodes.tqe_next).nodes.tqe_prev =
                                &raw mut (*swap).nodes.tqe_next;
                        }
                        (*con).nodes.tqe_next = swap as *mut Con;
                        if (*(*swap).parent).nodes_head.tqh_last == &raw mut (*con).nodes.tqe_next {
                            (*(*swap).parent).nodes_head.tqh_last = &raw mut (*swap).nodes.tqe_next;
                        }
                    } else {
                        *(*con).nodes.tqe_prev = swap as *mut Con;
                        (*swap).nodes.tqe_prev = (*con).nodes.tqe_prev;
                        (*con).nodes.tqe_prev = &raw mut (*swap).nodes.tqe_next;
                        (*con).nodes.tqe_next = (*swap).nodes.tqe_next;
                        if !(*swap).nodes.tqe_next.is_null() {
                            (*(*swap).nodes.tqe_next).nodes.tqe_prev =
                                &raw mut (*con).nodes.tqe_next;
                        }
                        (*swap).nodes.tqe_next = con as *mut Con;
                        if (*(*swap).parent).nodes_head.tqh_last == &raw mut (*swap).nodes.tqe_next
                        {
                            (*(*swap).parent).nodes_head.tqh_last = &raw mut (*con).nodes.tqe_next;
                        }
                    }
                    con_force_split_parents_redraw(con);
                    ipc_send_window_event(
                        b"move\0" as *const u8 as *const ::core::ffi::c_char,
                        con,
                    );
                    return;
                }
            } else {
                if (*con).parent == con_get_workspace(con) {
                    move_to_output_directed(con, direction);
                    return;
                }
                same_orientation = con_parent_with_orientation((*con).parent as *mut Con, o);
            }
        }
        if !same_orientation.is_null() {
            current_block = 11777552016271000781;
            break;
        }
    }
    match current_block {
        11777552016271000781 => {
            above = con;
            while (*above).parent != same_orientation {
                above = (*above).parent as *mut Con;
            }
            if !con_fullscreen_permits_focusing((*above).parent as *mut Con) {
                verboselog(
                    b"Cannot move out of fullscreen container\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
                return;
            }
            debuglog(
                b"%s:%s:%d - above = %p\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"../src/move.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"tree_move\0" as *const u8 as *const ::core::ffi::c_char,
                369 as ::core::ffi::c_int,
                above,
            );
            next = if direction as ::core::ffi::c_uint
                == D_UP as ::core::ffi::c_int as ::core::ffi::c_uint
                || direction as ::core::ffi::c_uint
                    == D_LEFT as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                *(*((*above).nodes.tqe_prev as *mut nodes_head)).tqh_last
            } else {
                (*above).nodes.tqe_next as *mut Con
            };
            if !next.is_null() && !con_is_leaf(next) {
                debuglog(
                    b"%s:%s:%d - Moving into the bordering branch of our adjacent container\n\0"
                        as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    b"../src/move.c\0" as *const u8 as *const ::core::ffi::c_char,
                    b"tree_move\0" as *const u8 as *const ::core::ffi::c_char,
                    374 as ::core::ffi::c_int,
                );
                target = con_descend_direction(next, direction);
                position = (if con_orientation((*target).parent as *mut Con) as ::core::ffi::c_uint
                    != o as ::core::ffi::c_uint
                    || direction as ::core::ffi::c_uint
                        == D_UP as ::core::ffi::c_int as ::core::ffi::c_uint
                    || direction as ::core::ffi::c_uint
                        == D_LEFT as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    AFTER as ::core::ffi::c_int
                } else {
                    BEFORE as ::core::ffi::c_int
                }) as position_t;
                insert_con_into(con, target, position);
            } else if next.is_null()
                && (*(*(*con).parent).parent).type_0 as ::core::ffi::c_uint
                    == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
                && (*(*con).parent).layout as ::core::ffi::c_uint
                    != L_DEFAULT as ::core::ffi::c_int as ::core::ffi::c_uint
                && con_num_children((*con).parent as *mut Con) == 1 as ::core::ffi::c_int
            {
                debuglog(
                    b"%s:%s:%d - Grandparent is workspace\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    b"../src/move.c\0" as *const u8 as *const ::core::ffi::c_char,
                    b"tree_move\0" as *const u8 as *const ::core::ffi::c_char,
                    389 as ::core::ffi::c_int,
                );
                move_to_output_directed(con, direction);
                return;
            } else {
                debuglog(
                    b"%s:%s:%d - Moving into container above\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    b"../src/move.c\0" as *const u8 as *const ::core::ffi::c_char,
                    b"tree_move\0" as *const u8 as *const ::core::ffi::c_char,
                    393 as ::core::ffi::c_int,
                );
                position = (if direction as ::core::ffi::c_uint
                    == D_UP as ::core::ffi::c_int as ::core::ffi::c_uint
                    || direction as ::core::ffi::c_uint
                        == D_LEFT as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    BEFORE as ::core::ffi::c_int
                } else {
                    AFTER as ::core::ffi::c_int
                }) as position_t;
                insert_con_into(con, above, position);
            }
        }
        _ => {}
    }
    free((*con).deco_render_params as *mut ::core::ffi::c_void);
    (*con).deco_render_params = ::core::ptr::null_mut::<deco_render_params>();
    ipc_send_window_event(b"move\0" as *const u8 as *const ::core::ffi::c_char, con);
    tree_flatten(croot);
    ewmh_update_wm_desktop();
}
