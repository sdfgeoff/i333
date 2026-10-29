/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * Faking outputs is useful in pathological situations (like network X servers
 * which don’t support multi-monitor in a useful way) and for our testsuite.
 *
 */
// Mechanical file-by-file Rust translation of src/fake_outputs.c.
// Generated with C2Rust 0.22.1; C ABI and original ownership retained.
extern "C" {
    pub type _cairo;
    pub type _cairo_surface;
    pub type _i3String;
    pub type pcre2_real_code_8;
    pub type ev_loop;
    fn sscanf(
        __s: *const ::core::ffi::c_char,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn errorlog(fmt: *mut ::core::ffi::c_char, ...);
    fn debuglog(fmt: *mut ::core::ffi::c_char, ...);
    fn scalloc(num: size_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn sasprintf(
        strp: *mut *mut ::core::ffi::c_char,
        fmt: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn min(a: ::core::ffi::c_int, b: ::core::ffi::c_int) -> ::core::ffi::c_int;
    static mut outputs: outputs_head;
    fn output_init_con(output: *mut Output);
    fn init_ws_for_output(output: *mut Output);
    fn output_primary_name(output: *mut Output) -> *mut ::core::ffi::c_char;
    fn exit(__status: ::core::ffi::c_int) -> !;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct outputs_head {
    pub tqh_first: *mut xoutput,
    pub tqh_last: *mut *mut xoutput,
}
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
static mut num_screens: ::core::ffi::c_int = 0;
unsafe extern "C" fn get_screen_at(
    mut x: ::core::ffi::c_uint,
    mut y: ::core::ffi::c_uint,
) -> *mut Output {
    let mut output: *mut Output = ::core::ptr::null_mut::<Output>();
    output = outputs.tqh_first as *mut Output;
    while !output.is_null() {
        if (*output).rect.x == x as uint32_t && (*output).rect.y == y as uint32_t {
            return output;
        }
        output = (*output).outputs.tqe_next as *mut Output;
    }
    return ::core::ptr::null_mut::<Output>();
}
#[no_mangle]
pub unsafe extern "C" fn fake_outputs_init(mut output_spec: *const ::core::ffi::c_char) {
    let mut walk: *const ::core::ffi::c_char = output_spec;
    let mut x: ::core::ffi::c_uint = 0;
    let mut y: ::core::ffi::c_uint = 0;
    let mut width: ::core::ffi::c_uint = 0;
    let mut height: ::core::ffi::c_uint = 0;
    let mut chars_consumed: ::core::ffi::c_int = 0;
    while sscanf(
        walk,
        b"%ux%u+%u+%u%n\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut width,
        &raw mut height,
        &raw mut x,
        &raw mut y,
        &raw mut chars_consumed,
    ) == 4 as ::core::ffi::c_int
    {
        walk = walk.offset(chars_consumed as isize);
        let mut primary: bool = false_0 != 0;
        if *walk as ::core::ffi::c_int == 'P' as i32 {
            primary = true_0 != 0;
            walk = walk.offset(1);
        }
        if *walk as ::core::ffi::c_int == ',' as i32 {
            walk = walk.offset(1);
        }
        debuglog(
            b"%s:%s:%d - Parsed output as width = %u, height = %u at (%u, %u)%s\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/fake_outputs.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"fake_outputs_init\0" as *const u8 as *const ::core::ffi::c_char,
            53 as ::core::ffi::c_int,
            width,
            height,
            x,
            y,
            if primary as ::core::ffi::c_int != 0 {
                b" (primary)\0" as *const u8 as *const ::core::ffi::c_char
            } else {
                b"\0" as *const u8 as *const ::core::ffi::c_char
            },
        );
        let mut new_output: *mut Output = get_screen_at(x, y);
        if !new_output.is_null() {
            debuglog(
                b"%s:%s:%d - Re-used old output %p\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"../src/fake_outputs.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"fake_outputs_init\0" as *const u8 as *const ::core::ffi::c_char,
                57 as ::core::ffi::c_int,
                new_output,
            );
            (*new_output).rect.width = min(
                (*new_output).rect.width as ::core::ffi::c_int,
                width as ::core::ffi::c_int,
            ) as uint32_t;
            (*new_output).rect.height = min(
                (*new_output).rect.height as ::core::ffi::c_int,
                height as ::core::ffi::c_int,
            ) as uint32_t;
        } else {
            let mut output_name: *mut output_name =
                scalloc(1 as size_t, ::core::mem::size_of::<output_name>() as size_t)
                    as *mut output_name;
            new_output =
                scalloc(1 as size_t, ::core::mem::size_of::<Output>() as size_t) as *mut Output;
            sasprintf(
                &raw mut (*output_name).name,
                b"fake-%d\0" as *const u8 as *const ::core::ffi::c_char,
                num_screens,
            );
            (*new_output).names_head.slh_first = ::core::ptr::null_mut::<output_name>();
            (*output_name).names.sle_next = (*new_output).names_head.slh_first;
            (*new_output).names_head.slh_first = output_name;
            debuglog(
                b"%s:%s:%d - Created new fake output %s (%p)\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"../src/fake_outputs.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"fake_outputs_init\0" as *const u8 as *const ::core::ffi::c_char,
                68 as ::core::ffi::c_int,
                output_primary_name(new_output),
                new_output,
            );
            (*new_output).active = true_0 != 0;
            (*new_output).rect.x = x as uint32_t;
            (*new_output).rect.y = y as uint32_t;
            (*new_output).rect.width = width as uint32_t;
            (*new_output).rect.height = height as uint32_t;
            if (*new_output).rect.x == 0 as uint32_t && (*new_output).rect.y == 0 as uint32_t {
                (*new_output).outputs.tqe_next = outputs.tqh_first;
                if !(*new_output).outputs.tqe_next.is_null() {
                    (*outputs.tqh_first).outputs.tqe_prev = &raw mut (*new_output).outputs.tqe_next;
                } else {
                    outputs.tqh_last = &raw mut (*new_output).outputs.tqe_next;
                }
                outputs.tqh_first = new_output as *mut xoutput;
                (*new_output).outputs.tqe_prev = &raw mut outputs.tqh_first;
            } else {
                (*new_output).outputs.tqe_next = ::core::ptr::null_mut::<xoutput>();
                (*new_output).outputs.tqe_prev = outputs.tqh_last;
                *outputs.tqh_last = new_output as *mut xoutput;
                outputs.tqh_last = &raw mut (*new_output).outputs.tqe_next;
            }
            output_init_con(new_output);
            init_ws_for_output(new_output);
            num_screens += 1;
        }
        (*new_output).primary = primary;
    }
    if num_screens == 0 as ::core::ffi::c_int {
        errorlog(
            b"ERROR: No screens found. Please fix your setup. i3 will exit now.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        exit(EXIT_FAILURE);
    }
}
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
