/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * commands_parser.c: hand-written parser to parse commands (commands are what
 * you bind on keys and what you can send to i3 using the IPC interface, like
 * 'move left' or 'workspace 4').
 *
 * We use a hand-written parser instead of lex/yacc because our commands are
 * easy for humans, not for computers. Thus, it’s quite hard to specify a
 * context-free grammar for the commands. A PEG grammar would be easier, but
 * there’s downsides to every PEG parser generator I have come across so far.
 *
 * This parser is basically a state machine which looks for literals or strings
 * and can push either on a stack. After identifying a literal or string, it
 * will either transition to the current state, to a different state, or call a
 * function (like cmd_move()).
 *
 * Special care has been taken that error messages are useful and the code is
 * well testable (when compiled with -DTEST_PARSER it will output to stdout
 * instead of actually calling any function).
 *
 */
// Mechanical file-by-file Rust translation of src/commands_parser.c.
// Generated with C2Rust 0.22.1; C ABI and original ownership retained.
extern "C" {
    pub type _cairo;
    pub type _cairo_surface;
    pub type _i3String;
    pub type pcre2_real_code_8;
    pub type ev_loop;
    pub type yajl_gen_t;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn __assert_single_arg(_: bool) -> bool;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn errorlog(fmt: *mut ::core::ffi::c_char, ...);
    fn debuglog(fmt: *mut ::core::ffi::c_char, ...);
    fn smalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn scalloc(num: size_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn sasprintf(
        strp: *mut *mut ::core::ffi::c_char,
        fmt: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn yajl_gen_string(
        hand: yajl_gen,
        str: *const ::core::ffi::c_uchar,
        len: size_t,
    ) -> yajl_gen_status;
    fn yajl_gen_bool(hand: yajl_gen, boolean: ::core::ffi::c_int) -> yajl_gen_status;
    fn yajl_gen_map_open(hand: yajl_gen) -> yajl_gen_status;
    fn yajl_gen_map_close(hand: yajl_gen) -> yajl_gen_status;
    fn yajl_gen_array_open(hand: yajl_gen) -> yajl_gen_status;
    fn yajl_gen_array_close(hand: yajl_gen) -> yajl_gen_status;
    fn parser_push_string(
        stack: *mut stack,
        identifier: *const ::core::ffi::c_char,
        str: *const ::core::ffi::c_char,
    );
    fn parser_push_long(
        stack: *mut stack,
        identifier: *const ::core::ffi::c_char,
        num: ::core::ffi::c_long,
    );
    fn parser_get_string(
        stack: *const stack,
        identifier: *const ::core::ffi::c_char,
    ) -> *const ::core::ffi::c_char;
    fn parser_get_long(
        stack: *const stack,
        identifier: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_long;
    fn parser_clear_stack(stack: *mut stack);
    fn cmd_criteria_init(current_match: *mut Match, cmd_output: *mut CommandResultIR);
    fn cmd_criteria_match_windows(current_match: *mut Match, cmd_output: *mut CommandResultIR);
    fn cmd_criteria_add(
        current_match: *mut Match,
        cmd_output: *mut CommandResultIR,
        ctype: *const ::core::ffi::c_char,
        cvalue: *const ::core::ffi::c_char,
    );
    fn cmd_move_con_to_workspace(
        current_match: *mut Match,
        cmd_output: *mut CommandResultIR,
        which: *const ::core::ffi::c_char,
    );
    fn cmd_move_con_to_workspace_back_and_forth(
        current_match: *mut Match,
        cmd_output: *mut CommandResultIR,
    );
    fn cmd_move_con_to_workspace_name(
        current_match: *mut Match,
        cmd_output: *mut CommandResultIR,
        name: *const ::core::ffi::c_char,
        no_auto_back_and_forth: *const ::core::ffi::c_char,
    );
    fn cmd_move_con_to_workspace_number(
        current_match: *mut Match,
        cmd_output: *mut CommandResultIR,
        which: *const ::core::ffi::c_char,
        no_auto_back_and_forth: *const ::core::ffi::c_char,
    );
    fn cmd_resize_set(
        current_match: *mut Match,
        cmd_output: *mut CommandResultIR,
        cwidth: ::core::ffi::c_long,
        mode_width: *const ::core::ffi::c_char,
        cheight: ::core::ffi::c_long,
        mode_height: *const ::core::ffi::c_char,
    );
    fn cmd_resize(
        current_match: *mut Match,
        cmd_output: *mut CommandResultIR,
        way: *const ::core::ffi::c_char,
        direction: *const ::core::ffi::c_char,
        resize_px: ::core::ffi::c_long,
        resize_ppt: ::core::ffi::c_long,
    );
    fn cmd_border(
        current_match: *mut Match,
        cmd_output: *mut CommandResultIR,
        border_style_str: *const ::core::ffi::c_char,
        border_width: ::core::ffi::c_long,
    );
    fn cmd_nop(
        current_match: *mut Match,
        cmd_output: *mut CommandResultIR,
        comment: *const ::core::ffi::c_char,
    );
    fn cmd_append_layout(
        current_match: *mut Match,
        cmd_output: *mut CommandResultIR,
        cpath: *const ::core::ffi::c_char,
    );
    fn cmd_workspace(
        current_match: *mut Match,
        cmd_output: *mut CommandResultIR,
        which: *const ::core::ffi::c_char,
    );
    fn cmd_workspace_number(
        current_match: *mut Match,
        cmd_output: *mut CommandResultIR,
        which: *const ::core::ffi::c_char,
        no_auto_back_and_forth_str: *const ::core::ffi::c_char,
    );
    fn cmd_workspace_back_and_forth(current_match: *mut Match, cmd_output: *mut CommandResultIR);
    fn cmd_workspace_name(
        current_match: *mut Match,
        cmd_output: *mut CommandResultIR,
        name: *const ::core::ffi::c_char,
        no_auto_back_and_forth_str: *const ::core::ffi::c_char,
    );
    fn cmd_mark(
        current_match: *mut Match,
        cmd_output: *mut CommandResultIR,
        mark: *const ::core::ffi::c_char,
        mode: *const ::core::ffi::c_char,
        toggle: *const ::core::ffi::c_char,
    );
    fn cmd_unmark(
        current_match: *mut Match,
        cmd_output: *mut CommandResultIR,
        mark: *const ::core::ffi::c_char,
    );
    fn cmd_mode(
        current_match: *mut Match,
        cmd_output: *mut CommandResultIR,
        mode: *const ::core::ffi::c_char,
    );
    fn cmd_move_con_to_output(
        current_match: *mut Match,
        cmd_output: *mut CommandResultIR,
        name: *const ::core::ffi::c_char,
        move_workspace: bool,
    );
    fn cmd_move_con_to_mark(
        current_match: *mut Match,
        cmd_output: *mut CommandResultIR,
        mark: *const ::core::ffi::c_char,
    );
    fn cmd_floating(
        current_match: *mut Match,
        cmd_output: *mut CommandResultIR,
        floating_mode: *const ::core::ffi::c_char,
    );
    fn cmd_split(
        current_match: *mut Match,
        cmd_output: *mut CommandResultIR,
        direction: *const ::core::ffi::c_char,
    );
    fn cmd_kill(
        current_match: *mut Match,
        cmd_output: *mut CommandResultIR,
        kill_mode_str: *const ::core::ffi::c_char,
    );
    fn cmd_exec(
        current_match: *mut Match,
        cmd_output: *mut CommandResultIR,
        nosn: *const ::core::ffi::c_char,
        command: *const ::core::ffi::c_char,
    );
    fn cmd_focus_direction(
        current_match: *mut Match,
        cmd_output: *mut CommandResultIR,
        direction_str: *const ::core::ffi::c_char,
    );
    fn cmd_focus_sibling(
        current_match: *mut Match,
        cmd_output: *mut CommandResultIR,
        direction_str: *const ::core::ffi::c_char,
    );
    fn cmd_focus_window_mode(
        current_match: *mut Match,
        cmd_output: *mut CommandResultIR,
        window_mode: *const ::core::ffi::c_char,
    );
    fn cmd_focus_level(
        current_match: *mut Match,
        cmd_output: *mut CommandResultIR,
        level: *const ::core::ffi::c_char,
    );
    fn cmd_focus(
        current_match: *mut Match,
        cmd_output: *mut CommandResultIR,
        focus_workspace: bool,
    );
    fn cmd_fullscreen(
        current_match: *mut Match,
        cmd_output: *mut CommandResultIR,
        action: *const ::core::ffi::c_char,
        fullscreen_mode: *const ::core::ffi::c_char,
    );
    fn cmd_sticky(
        current_match: *mut Match,
        cmd_output: *mut CommandResultIR,
        action: *const ::core::ffi::c_char,
    );
    fn cmd_move_direction(
        current_match: *mut Match,
        cmd_output: *mut CommandResultIR,
        direction_str: *const ::core::ffi::c_char,
        amount: ::core::ffi::c_long,
        mode: *const ::core::ffi::c_char,
    );
    fn cmd_layout(
        current_match: *mut Match,
        cmd_output: *mut CommandResultIR,
        layout_str: *const ::core::ffi::c_char,
    );
    fn cmd_layout_toggle(
        current_match: *mut Match,
        cmd_output: *mut CommandResultIR,
        toggle_mode: *const ::core::ffi::c_char,
    );
    fn cmd_exit(current_match: *mut Match, cmd_output: *mut CommandResultIR);
    fn cmd_reload(current_match: *mut Match, cmd_output: *mut CommandResultIR);
    fn cmd_restart(current_match: *mut Match, cmd_output: *mut CommandResultIR);
    fn cmd_open(current_match: *mut Match, cmd_output: *mut CommandResultIR);
    fn cmd_focus_output(
        current_match: *mut Match,
        cmd_output: *mut CommandResultIR,
        name: *const ::core::ffi::c_char,
    );
    fn cmd_move_window_to_position(
        current_match: *mut Match,
        cmd_output: *mut CommandResultIR,
        x: ::core::ffi::c_long,
        mode_x: *const ::core::ffi::c_char,
        y: ::core::ffi::c_long,
        mode_y: *const ::core::ffi::c_char,
    );
    fn cmd_move_window_to_center(
        current_match: *mut Match,
        cmd_output: *mut CommandResultIR,
        method: *const ::core::ffi::c_char,
    );
    fn cmd_move_window_to_mouse(current_match: *mut Match, cmd_output: *mut CommandResultIR);
    fn cmd_move_scratchpad(current_match: *mut Match, cmd_output: *mut CommandResultIR);
    fn cmd_scratchpad_show(current_match: *mut Match, cmd_output: *mut CommandResultIR);
    fn cmd_swap(
        current_match: *mut Match,
        cmd_output: *mut CommandResultIR,
        mode: *const ::core::ffi::c_char,
        arg: *const ::core::ffi::c_char,
    );
    fn cmd_title_format(
        current_match: *mut Match,
        cmd_output: *mut CommandResultIR,
        format: *const ::core::ffi::c_char,
    );
    fn cmd_rename_workspace(
        current_match: *mut Match,
        cmd_output: *mut CommandResultIR,
        old_name: *const ::core::ffi::c_char,
        new_name: *const ::core::ffi::c_char,
    );
    fn cmd_bar_mode(
        current_match: *mut Match,
        cmd_output: *mut CommandResultIR,
        bar_mode: *const ::core::ffi::c_char,
        bar_id: *const ::core::ffi::c_char,
    );
    fn cmd_bar_hidden_state(
        current_match: *mut Match,
        cmd_output: *mut CommandResultIR,
        bar_hidden_state: *const ::core::ffi::c_char,
        bar_id: *const ::core::ffi::c_char,
    );
    fn cmd_shmlog(
        current_match: *mut Match,
        cmd_output: *mut CommandResultIR,
        argument: *const ::core::ffi::c_char,
    );
    fn cmd_debuglog(
        current_match: *mut Match,
        cmd_output: *mut CommandResultIR,
        argument: *const ::core::ffi::c_char,
    );
    fn cmd_gaps(
        current_match: *mut Match,
        cmd_output: *mut CommandResultIR,
        type_0: *const ::core::ffi::c_char,
        scope: *const ::core::ffi::c_char,
        mode: *const ::core::ffi::c_char,
        value: *const ::core::ffi::c_char,
    );
    fn cmd_title_window_icon(
        current_match: *mut Match,
        cmd_output: *mut CommandResultIR,
        enable: *const ::core::ffi::c_char,
        padding: ::core::ffi::c_int,
    );
    fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn strncasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ev_watcher_list {
    pub active: ::core::ffi::c_int,
    pub pending: ::core::ffi::c_int,
    pub priority: ::core::ffi::c_int,
    pub data: *mut ::core::ffi::c_void,
    pub cb:
        Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_watcher_list, ::core::ffi::c_int) -> ()>,
    pub next: *mut ev_watcher_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ev_io {
    pub active: ::core::ffi::c_int,
    pub pending: ::core::ffi::c_int,
    pub priority: ::core::ffi::c_int,
    pub data: *mut ::core::ffi::c_void,
    pub cb: Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_io, ::core::ffi::c_int) -> ()>,
    pub next: *mut ev_watcher_list,
    pub fd: ::core::ffi::c_int,
    pub events: ::core::ffi::c_int,
}
pub type yajl_gen_status = ::core::ffi::c_uint;
pub const yajl_gen_invalid_string: yajl_gen_status = 7;
pub const yajl_gen_no_buf: yajl_gen_status = 6;
pub const yajl_gen_invalid_number: yajl_gen_status = 5;
pub const yajl_gen_generation_complete: yajl_gen_status = 4;
pub const yajl_gen_in_error_state: yajl_gen_status = 3;
pub const yajl_max_depth_exceeded: yajl_gen_status = 2;
pub const yajl_gen_keys_must_be_strings: yajl_gen_status = 1;
pub const yajl_gen_status_ok: yajl_gen_status = 0;
pub type yajl_gen = *mut yajl_gen_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ipc_client {
    pub fd: ::core::ffi::c_int,
    pub num_events: ::core::ffi::c_int,
    pub events: *mut *mut ::core::ffi::c_char,
    pub first_tick_sent: bool,
    pub read_callback: *mut ev_io,
    pub write_callback: *mut ev_io,
    pub timeout: *mut ev_timer,
    pub buffer: *mut uint8_t,
    pub buffer_size: size_t,
    pub clients: C2RustUnnamed_16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_16 {
    pub tqe_next: *mut ipc_client,
    pub tqe_prev: *mut *mut ipc_client,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack_entry {
    pub identifier: *const ::core::ffi::c_char,
    pub type_0: C2RustUnnamed_18,
    pub val: C2RustUnnamed_17,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_17 {
    pub str_0: *mut ::core::ffi::c_char,
    pub num: ::core::ffi::c_long,
}
pub type C2RustUnnamed_18 = ::core::ffi::c_uint;
pub const STACK_LONG: C2RustUnnamed_18 = 1;
pub const STACK_STR: C2RustUnnamed_18 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack {
    pub stack: [stack_entry; 10],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct owindow {
    pub con: *mut Con,
    pub owindows: C2RustUnnamed_19,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_19 {
    pub tqe_next: *mut owindow,
    pub tqe_prev: *mut *mut owindow,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CommandResultIR {
    pub ctx: *mut cmd_parser_ctx,
    pub json_gen: yajl_gen,
    pub client: *mut ipc_client,
    pub next_state: ::core::ffi::c_int,
    pub needs_tree_render: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmd_parser_ctx {
    pub state: ::core::ffi::c_int,
    pub current_match: Match,
    pub stack: stack,
    pub owindows: owindows_head,
    pub subcommand_output: CommandResultIR,
    pub command_output: CommandResultIR,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct owindows_head {
    pub tqh_first: *mut owindow,
    pub tqh_last: *mut *mut owindow,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CommandResult {
    pub parse_error: bool,
    pub error_message: *mut ::core::ffi::c_char,
    pub needs_tree_render: bool,
}
pub type cmdp_token_ptr = tokenptr;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tokenptr {
    pub array: *mut cmdp_token,
    pub n: ::core::ffi::c_int,
}
pub type cmdp_token = token;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct token {
    pub name: *mut ::core::ffi::c_char,
    pub identifier: *mut ::core::ffi::c_char,
    pub next_state: cmdp_state,
    pub extra: C2RustUnnamed_20,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_20 {
    pub call_identifier: uint16_t,
}
pub type cmdp_state = ::core::ffi::c_uint;
pub const __CALL: cmdp_state = 77;
pub const NOP: cmdp_state = 76;
pub const BAR: cmdp_state = 75;
pub const SWAP: cmdp_state = 74;
pub const MOVE: cmdp_state = 73;
pub const MODE: cmdp_state = 72;
pub const MARK: cmdp_state = 71;
pub const KILL: cmdp_state = 70;
pub const GAPS: cmdp_state = 69;
pub const EXEC: cmdp_state = 68;
pub const SPLIT: cmdp_state = 67;
pub const FOCUS: cmdp_state = 66;
pub const UNMARK: cmdp_state = 65;
pub const STICKY: cmdp_state = 64;
pub const SHMLOG: cmdp_state = 63;
pub const RESIZE: cmdp_state = 62;
pub const RENAME: cmdp_state = 61;
pub const LAYOUT: cmdp_state = 60;
pub const BORDER: cmdp_state = 59;
pub const INITIAL: cmdp_state = 58;
pub const GAPS_END: cmdp_state = 57;
pub const FLOATING: cmdp_state = 56;
pub const DEBUGLOG: cmdp_state = 55;
pub const CRITERIA: cmdp_state = 54;
pub const BAR_MODE: cmdp_state = 53;
pub const WORKSPACE: cmdp_state = 52;
pub const RESIZE_PX: cmdp_state = 51;
pub const CRITERION: cmdp_state = 50;
pub const SCRATCHPAD: cmdp_state = 49;
pub const RESIZE_SET: cmdp_state = 48;
pub const FULLSCREEN: cmdp_state = 47;
pub const FOCUS_AUTO: cmdp_state = 46;
pub const BAR_MODE_ID: cmdp_state = 45;
pub const TITLE_FORMAT: cmdp_state = 44;
pub const RESIZE_WIDTH: cmdp_state = 43;
pub const MOVE_TO_MARK: cmdp_state = 42;
pub const FOCUS_OUTPUT: cmdp_state = 41;
pub const BORDER_WIDTH: cmdp_state = 40;
pub const SWAP_ARGUMENT: cmdp_state = 39;
pub const RESIZE_TILING: cmdp_state = 38;
pub const RESIZE_HEIGHT: cmdp_state = 37;
pub const LAYOUT_TOGGLE: cmdp_state = 36;
pub const CRITERION_STR: cmdp_state = 35;
pub const APPEND_LAYOUT: cmdp_state = 34;
pub const MOVE_WORKSPACE: cmdp_state = 33;
pub const MOVE_TO_OUTPUT: cmdp_state = 32;
pub const MOVE_DIRECTION: cmdp_state = 31;
pub const GAPS_WITH_TYPE: cmdp_state = 30;
pub const GAPS_WITH_MODE: cmdp_state = 29;
pub const CRITERION_FROM: cmdp_state = 28;
pub const GAPS_WITH_SCOPE: cmdp_state = 27;
pub const FULLSCREEN_MODE: cmdp_state = 26;
pub const WORKSPACE_NUMBER: cmdp_state = 25;
pub const RESIZE_TILING_OR: cmdp_state = 24;
pub const RESIZE_DIRECTION: cmdp_state = 23;
pub const RENAME_WORKSPACE: cmdp_state = 22;
pub const MOVE_TO_POSITION: cmdp_state = 21;
pub const BAR_HIDDEN_STATE: cmdp_state = 20;
pub const TITLE_WINDOW_ICON: cmdp_state = 19;
pub const FULLSCREEN_COMPAT: cmdp_state = 18;
pub const MOVE_TO_POSITION_Y: cmdp_state = 17;
pub const MOVE_TO_POSITION_X: cmdp_state = 16;
pub const CRITERION_FROM_STR: cmdp_state = 15;
pub const RESIZE_TILING_FINAL: cmdp_state = 14;
pub const RENAME_WORKSPACE_TO: cmdp_state = 13;
pub const BAR_HIDDEN_STATE_ID: cmdp_state = 12;
pub const MOVE_WORKSPACE_NUMBER: cmdp_state = 11;
pub const MOVE_DIRECTION_NUMBER: cmdp_state = 10;
pub const CRITERION_FROM_STR_END: cmdp_state = 9;
pub const RESIZE_HEIGHT_GET_NUMBER: cmdp_state = 8;
pub const MOVE_WORKSPACE_TO_OUTPUT: cmdp_state = 7;
pub const CRITERION_FROM_STR_START: cmdp_state = 6;
pub const TITLE_WINDOW_ICON_PADDING: cmdp_state = 5;
pub const MOVE_TO_ABSOLUTE_POSITION: cmdp_state = 4;
pub const RENAME_WORKSPACE_LIKELY_TO: cmdp_state = 3;
pub const RENAME_WORKSPACE_TO_NEW_NAME: cmdp_state = 2;
pub const MOVE_WORKSPACE_TO_OUTPUT_WORD: cmdp_state = 1;
pub const RENAME_WORKSPACE_LIKELY_TO_NEW_NAME: cmdp_state = 0;
pub const __ASSERT_FUNCTION: [::core::ffi::c_char; 82] = unsafe {
    ::core::mem::transmute::<[u8; 82], [::core::ffi::c_char; 82]>(
        *b"void GENERATED_call(Match *, struct stack *, const int, struct CommandResultIR *)\0",
    )
};
pub const LONG_MAX: ::core::ffi::c_long = __LONG_MAX__;
pub const LONG_MIN: ::core::ffi::c_long = -__LONG_MAX__ - 1 as ::core::ffi::c_long;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
static mut tokens_RENAME_WORKSPACE_LIKELY_TO_NEW_NAME: [cmdp_token; 2] = [
    token {
        name: b"string\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"new_name\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"end\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 1 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_MOVE_WORKSPACE_TO_OUTPUT_WORD: [cmdp_token; 2] = [
    token {
        name: b"word\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"output\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 2 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"end\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 3 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_RENAME_WORKSPACE_TO_NEW_NAME: [cmdp_token; 1] = [token {
    name: b"string\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"new_name\0" as *const u8 as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_20 {
        call_identifier: 4 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_RENAME_WORKSPACE_LIKELY_TO: [cmdp_token; 2] = [
    token {
        name: b"'to \0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: RENAME_WORKSPACE_LIKELY_TO_NEW_NAME,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"word\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"new_name\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 5 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_MOVE_TO_ABSOLUTE_POSITION: [cmdp_token; 1] = [token {
    name: b"'position\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: MOVE_TO_POSITION,
    extra: C2RustUnnamed_20 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_TITLE_WINDOW_ICON_PADDING: [cmdp_token; 3] = [
    token {
        name: b"end\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 6 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'px\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 7 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"number\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"padding\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: TITLE_WINDOW_ICON_PADDING,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_CRITERION_FROM_STR_START: [cmdp_token; 3] = [
    token {
        name: b"'\"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CRITERION_FROM_STR,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'auto\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"kind\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 8 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'user\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"kind\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 9 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_MOVE_WORKSPACE_TO_OUTPUT: [cmdp_token; 1] = [token {
    name: b"'output\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: MOVE_WORKSPACE_TO_OUTPUT_WORD,
    extra: C2RustUnnamed_20 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_RESIZE_HEIGHT_GET_NUMBER: [cmdp_token; 1] = [token {
    name: b"number\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"height\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: RESIZE_HEIGHT,
    extra: C2RustUnnamed_20 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_CRITERION_FROM_STR_END: [cmdp_token; 1] = [token {
    name: b"'\"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_20 {
        call_identifier: 10 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_MOVE_DIRECTION_NUMBER: [cmdp_token; 3] = [
    token {
        name: b"'px\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"mode\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 11 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'ppt\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"mode\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 12 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"end\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 13 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_MOVE_WORKSPACE_NUMBER: [cmdp_token; 1] = [token {
    name: b"string\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"number\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_20 {
        call_identifier: 14 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_HIDDEN_STATE_ID: [cmdp_token; 2] = [
    token {
        name: b"word\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"bar_id\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BAR_HIDDEN_STATE_ID,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"end\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 15 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_RENAME_WORKSPACE_TO: [cmdp_token; 1] = [token {
    name: b"'to\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: RENAME_WORKSPACE_TO_NEW_NAME,
    extra: C2RustUnnamed_20 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_RESIZE_TILING_FINAL: [cmdp_token; 2] = [
    token {
        name: b"'ppt\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 16 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"end\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 17 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_CRITERION_FROM_STR: [cmdp_token; 2] = [
    token {
        name: b"'auto\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"kind\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: CRITERION_FROM_STR_END,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'user\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"kind\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: CRITERION_FROM_STR_END,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_MOVE_TO_POSITION_X: [cmdp_token; 3] = [
    token {
        name: b"'px\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"mode_x\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: MOVE_TO_POSITION_X,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'ppt\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"mode_x\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: MOVE_TO_POSITION_X,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"number\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"coord_y\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: MOVE_TO_POSITION_Y,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_MOVE_TO_POSITION_Y: [cmdp_token; 3] = [
    token {
        name: b"'px\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"mode_y\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 18 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'ppt\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"mode_y\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 19 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"end\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 20 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_FULLSCREEN_COMPAT: [cmdp_token; 2] = [
    token {
        name: b"'global\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"mode\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 21 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"end\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 22 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_TITLE_WINDOW_ICON: [cmdp_token; 14] = [
    token {
        name: b"'padding\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: TITLE_WINDOW_ICON_PADDING,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'toggle\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"enable\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: TITLE_WINDOW_ICON_PADDING,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'1\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"enable\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 23 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'yes\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"enable\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 24 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'true\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"enable\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 25 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'on\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"enable\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 26 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'enable\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"enable\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 27 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'active\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"enable\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 28 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'0\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"enable\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 29 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'no\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"enable\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 30 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'false\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"enable\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 31 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'off\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"enable\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 32 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'disable\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"enable\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 33 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'inactive\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"enable\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 34 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_BAR_HIDDEN_STATE: [cmdp_token; 3] = [
    token {
        name: b"'hide\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"bar_value\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BAR_HIDDEN_STATE_ID,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'show\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"bar_value\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BAR_HIDDEN_STATE_ID,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'toggle\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"bar_value\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BAR_HIDDEN_STATE_ID,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_MOVE_TO_POSITION: [cmdp_token; 5] = [
    token {
        name: b"'center\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 35 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'mouse\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 36 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'cursor\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 37 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'pointer\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 38 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"number\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"coord_x\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: MOVE_TO_POSITION_X,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_RENAME_WORKSPACE: [cmdp_token; 2] = [
    token {
        name: b"'to\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: RENAME_WORKSPACE_LIKELY_TO,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"word\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"old_name\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: RENAME_WORKSPACE_TO,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_RESIZE_DIRECTION: [cmdp_token; 6] = [
    token {
        name: b"'up\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"direction\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: RESIZE_PX,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'down\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"direction\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: RESIZE_PX,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'left\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"direction\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: RESIZE_PX,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'right\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"direction\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: RESIZE_PX,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'width\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"direction\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: RESIZE_PX,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'height\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"direction\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: RESIZE_PX,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_RESIZE_TILING_OR: [cmdp_token; 1] = [token {
    name: b"number\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"resize_ppt\0" as *const u8 as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    next_state: RESIZE_TILING_FINAL,
    extra: C2RustUnnamed_20 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_WORKSPACE_NUMBER: [cmdp_token; 1] = [token {
    name: b"string\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"workspace\0" as *const u8 as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_20 {
        call_identifier: 39 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_FULLSCREEN_MODE: [cmdp_token; 2] = [
    token {
        name: b"'global\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"mode\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 40 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"end\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 41 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_GAPS_WITH_SCOPE: [cmdp_token; 4] = [
    token {
        name: b"'plus\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"mode\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: GAPS_WITH_MODE,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'minus\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"mode\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: GAPS_WITH_MODE,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'set\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"mode\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: GAPS_WITH_MODE,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'toggle\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"mode\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: GAPS_WITH_MODE,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_CRITERION_FROM: [cmdp_token; 1] = [token {
    name: b"'=\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: CRITERION_FROM_STR_START,
    extra: C2RustUnnamed_20 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_GAPS_WITH_MODE: [cmdp_token; 1] = [token {
    name: b"word\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"value\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: GAPS_END,
    extra: C2RustUnnamed_20 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_GAPS_WITH_TYPE: [cmdp_token; 2] = [
    token {
        name: b"'current\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"scope\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: GAPS_WITH_SCOPE,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'all\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"scope\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: GAPS_WITH_SCOPE,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_MOVE_DIRECTION: [cmdp_token; 2] = [
    token {
        name: b"number\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"amount\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: MOVE_DIRECTION_NUMBER,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"end\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 42 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_MOVE_TO_OUTPUT: [cmdp_token; 2] = [
    token {
        name: b"word\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"output\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 43 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"end\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 44 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_MOVE_WORKSPACE: [cmdp_token; 9] = [
    token {
        name: b"'to \0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: MOVE_WORKSPACE_TO_OUTPUT,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'next_on_output\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"workspace\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 45 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'prev_on_output\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"workspace\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 46 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'next\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"workspace\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 47 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'prev\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"workspace\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 48 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'current\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"workspace\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 49 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'back_and_forth\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 50 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'number\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: MOVE_WORKSPACE_NUMBER,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"string\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"workspace\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 51 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_APPEND_LAYOUT: [cmdp_token; 1] = [token {
    name: b"string\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"path\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_20 {
        call_identifier: 52 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_CRITERION_STR: [cmdp_token; 1] = [token {
    name: b"word\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"cvalue\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_20 {
        call_identifier: 53 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_LAYOUT_TOGGLE: [cmdp_token; 2] = [
    token {
        name: b"end\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 54 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"string\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"toggle_mode\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 55 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_RESIZE_HEIGHT: [cmdp_token; 3] = [
    token {
        name: b"'px\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"mode_height\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: RESIZE_HEIGHT,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'ppt\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"mode_height\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: RESIZE_HEIGHT,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"end\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 56 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_RESIZE_TILING: [cmdp_token; 3] = [
    token {
        name: b"'px\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: RESIZE_TILING,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'or\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: RESIZE_TILING_OR,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"end\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 57 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_SWAP_ARGUMENT: [cmdp_token; 1] = [token {
    name: b"string\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"arg\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_20 {
        call_identifier: 58 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BORDER_WIDTH: [cmdp_token; 2] = [
    token {
        name: b"end\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 59 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"number\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"border_width\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 60 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_FOCUS_OUTPUT: [cmdp_token; 2] = [
    token {
        name: b"word\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"output\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 61 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"end\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 62 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_MOVE_TO_MARK: [cmdp_token; 1] = [token {
    name: b"string\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"mark\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_20 {
        call_identifier: 63 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_RESIZE_WIDTH: [cmdp_token; 5] = [
    token {
        name: b"'px\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"mode_width\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: RESIZE_WIDTH,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'ppt\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"mode_width\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: RESIZE_WIDTH,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"end\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 64 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'height\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: RESIZE_HEIGHT_GET_NUMBER,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"number\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"height\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: RESIZE_HEIGHT,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_TITLE_FORMAT: [cmdp_token; 1] = [token {
    name: b"string\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"format\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_20 {
        call_identifier: 65 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_MODE_ID: [cmdp_token; 2] = [
    token {
        name: b"word\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"bar_id\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BAR_MODE_ID,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"end\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 66 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_FOCUS_AUTO: [cmdp_token; 2] = [
    token {
        name: b"'sibling\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 67 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"end\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 68 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_FULLSCREEN: [cmdp_token; 4] = [
    token {
        name: b"'disable\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"action\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 69 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'enable\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"action\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: FULLSCREEN_MODE,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'toggle\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"action\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: FULLSCREEN_MODE,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"action\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: FULLSCREEN_COMPAT,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_RESIZE_SET: [cmdp_token; 3] = [
    token {
        name: b"'height\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: RESIZE_HEIGHT_GET_NUMBER,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'width\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: RESIZE_SET,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"number\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"width\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: RESIZE_WIDTH,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_SCRATCHPAD: [cmdp_token; 1] = [token {
    name: b"'show\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_20 {
        call_identifier: 70 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_CRITERION: [cmdp_token; 1] = [token {
    name: b"'=\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: CRITERION_STR,
    extra: C2RustUnnamed_20 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_RESIZE_PX: [cmdp_token; 2] = [
    token {
        name: b"number\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"resize_px\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: RESIZE_TILING,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"end\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 71 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_WORKSPACE: [cmdp_token; 8] = [
    token {
        name: b"'--no-auto-back-and-forth\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"no_auto_back_and_forth\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: WORKSPACE,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'next_on_output\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"direction\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 72 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'prev_on_output\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"direction\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 73 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'next\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"direction\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 74 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'prev\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"direction\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 75 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'back_and_forth\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 76 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'number\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: WORKSPACE_NUMBER,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"string\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"workspace\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 77 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_BAR_MODE: [cmdp_token; 4] = [
    token {
        name: b"'dock\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"bar_value\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BAR_MODE_ID,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'hide\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"bar_value\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BAR_MODE_ID,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'invisible\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"bar_value\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BAR_MODE_ID,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'toggle\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"bar_value\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BAR_MODE_ID,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_CRITERIA: [cmdp_token; 17] = [
    token {
        name: b"'class\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"ctype\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: CRITERION,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'instance\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"ctype\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: CRITERION,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'window_role\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"ctype\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: CRITERION,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'con_id\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"ctype\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: CRITERION,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'id\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"ctype\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: CRITERION,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'window_type\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"ctype\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: CRITERION,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'con_mark\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"ctype\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: CRITERION,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'title\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"ctype\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: CRITERION,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'urgent\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"ctype\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: CRITERION,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'workspace\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"ctype\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: CRITERION,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'machine\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"ctype\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: CRITERION,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'floating_from\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"ctype\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: CRITERION_FROM,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'tiling_from\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"ctype\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: CRITERION_FROM,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'tiling\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"ctype\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 78 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'floating\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"ctype\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 79 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'all\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"ctype\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 80 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"']\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 81 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_DEBUGLOG: [cmdp_token; 3] = [
    token {
        name: b"'toggle\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"argument\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 82 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'on\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"argument\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 83 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'off\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"argument\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 84 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_FLOATING: [cmdp_token; 3] = [
    token {
        name: b"'enable\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"floating\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 85 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'disable\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"floating\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 86 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'toggle\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"floating\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 87 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_GAPS_END: [cmdp_token; 2] = [
    token {
        name: b"'px\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: GAPS_END,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"end\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 88 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_INITIAL: [cmdp_token; 32] = [
    token {
        name: b"end\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: INITIAL,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'[\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 89 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'move\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: MOVE,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'exec\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: EXEC,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'exit\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 90 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'restart\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 91 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'reload\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 92 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'shmlog\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: SHMLOG,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'debuglog\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: DEBUGLOG,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'border\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BORDER,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'layout\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: LAYOUT,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'append_layout\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: APPEND_LAYOUT,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'workspace\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: WORKSPACE,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'focus\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: FOCUS,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'kill\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: KILL,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'open\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 93 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'fullscreen\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: FULLSCREEN,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'sticky\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: STICKY,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'split\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: SPLIT,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'floating\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: FLOATING,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'mark\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: MARK,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'unmark\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: UNMARK,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'resize\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: RESIZE,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'rename\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: RENAME,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'nop\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: NOP,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'scratchpad\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: SCRATCHPAD,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'swap\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: SWAP,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'title_format\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: TITLE_FORMAT,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'title_window_icon\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: TITLE_WINDOW_ICON,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'mode\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: MODE,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'bar\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'gaps\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: GAPS,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_BORDER: [cmdp_token; 5] = [
    token {
        name: b"'normal\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"border_style\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BORDER_WIDTH,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'pixel\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"border_style\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BORDER_WIDTH,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'toggle\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"border_style\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BORDER_WIDTH,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'none\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"border_style\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 94 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'1pixel\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 95 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_LAYOUT: [cmdp_token; 7] = [
    token {
        name: b"'default\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"layout_mode\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 96 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'stacked\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"layout_mode\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 97 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'stacking\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"layout_mode\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 98 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'tabbed\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"layout_mode\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 99 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'splitv\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"layout_mode\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 100 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'splith\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"layout_mode\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 101 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'toggle\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: LAYOUT_TOGGLE,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_RENAME: [cmdp_token; 1] = [token {
    name: b"'workspace\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: RENAME_WORKSPACE,
    extra: C2RustUnnamed_20 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_RESIZE: [cmdp_token; 3] = [
    token {
        name: b"'grow\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"way\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        next_state: RESIZE_DIRECTION,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'shrink\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"way\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        next_state: RESIZE_DIRECTION,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'set\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"set\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        next_state: RESIZE_SET,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_SHMLOG: [cmdp_token; 1] = [token {
    name: b"string\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"argument\0" as *const u8 as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_20 {
        call_identifier: 102 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_STICKY: [cmdp_token; 3] = [
    token {
        name: b"'enable\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"action\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 103 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'disable\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"action\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 104 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'toggle\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"action\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 105 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_UNMARK: [cmdp_token; 2] = [
    token {
        name: b"end\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 106 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"string\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"mark\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 107 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_FOCUS: [cmdp_token; 14] = [
    token {
        name: b"'left\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"direction\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 108 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'right\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"direction\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 109 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'up\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"direction\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 110 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'down\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"direction\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 111 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'prev\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"direction\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: FOCUS_AUTO,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'next\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"direction\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: FOCUS_AUTO,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'output\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: FOCUS_OUTPUT,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'tiling\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"window_mode\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 112 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'floating\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"window_mode\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 113 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'mode_toggle\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"window_mode\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 114 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'parent\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"level\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 115 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'child\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"level\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 116 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'workspace\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"workspace\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 117 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"end\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 118 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_SPLIT: [cmdp_token; 6] = [
    token {
        name: b"'horizontal\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"direction\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 119 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'vertical\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"direction\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 120 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'toggle\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"direction\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 121 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'v\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"direction\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 122 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'h\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"direction\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 123 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'t\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"direction\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 124 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_EXEC: [cmdp_token; 2] = [
    token {
        name: b"'--no-startup-id\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"nosn\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: EXEC,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"string\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"command\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 125 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_GAPS: [cmdp_token; 8] = [
    token {
        name: b"'inner\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"type\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: GAPS_WITH_TYPE,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'outer\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"type\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: GAPS_WITH_TYPE,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'horizontal\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"type\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: GAPS_WITH_TYPE,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'vertical\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"type\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: GAPS_WITH_TYPE,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'top\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"type\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: GAPS_WITH_TYPE,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'right\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"type\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: GAPS_WITH_TYPE,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'bottom\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"type\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: GAPS_WITH_TYPE,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'left\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"type\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: GAPS_WITH_TYPE,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_KILL: [cmdp_token; 3] = [
    token {
        name: b"'window\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"kill_mode\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 126 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'client\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"kill_mode\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 127 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"end\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 128 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_MARK: [cmdp_token; 4] = [
    token {
        name: b"'--add\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"mode\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: MARK,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'--replace\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"mode\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: MARK,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'--toggle\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"toggle\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: MARK,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"string\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"mark\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 129 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_MODE: [cmdp_token; 1] = [token {
    name: b"string\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"mode\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_20 {
        call_identifier: 130 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_MOVE: [cmdp_token; 14] = [
    token {
        name: b"'window\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: MOVE,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'container\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: MOVE,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'to\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: MOVE,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'--no-auto-back-and-forth\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"no_auto_back_and_forth\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: MOVE,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'workspace\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: MOVE_WORKSPACE,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'output\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: MOVE_TO_OUTPUT,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'mark\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: MOVE_TO_MARK,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'scratchpad\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 131 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'left\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"direction\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: MOVE_DIRECTION,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'right\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"direction\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: MOVE_DIRECTION,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'up\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"direction\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: MOVE_DIRECTION,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'down\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"direction\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: MOVE_DIRECTION,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'position\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"method\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: MOVE_TO_POSITION,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'absolute\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"method\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: MOVE_TO_ABSOLUTE_POSITION,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_SWAP: [cmdp_token; 5] = [
    token {
        name: b"'container\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: SWAP,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'with\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: SWAP,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'id\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"mode\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: SWAP_ARGUMENT,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'con_id\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"mode\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: SWAP_ARGUMENT,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'mark\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"mode\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: SWAP_ARGUMENT,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_BAR: [cmdp_token; 2] = [
    token {
        name: b"'hidden_state\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_HIDDEN_STATE,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'mode\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_MODE,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_NOP: [cmdp_token; 2] = [
    token {
        name: b"string\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"comment\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 132 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"end\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_20 {
            call_identifier: 133 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens: [cmdp_token_ptr; 77] = unsafe {
    [
        tokenptr {
            array: &raw const tokens_RENAME_WORKSPACE_LIKELY_TO_NEW_NAME as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_MOVE_WORKSPACE_TO_OUTPUT_WORD as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_RENAME_WORKSPACE_TO_NEW_NAME as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_RENAME_WORKSPACE_LIKELY_TO as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_MOVE_TO_ABSOLUTE_POSITION as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_TITLE_WINDOW_ICON_PADDING as *mut cmdp_token,
            n: 3 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_CRITERION_FROM_STR_START as *mut cmdp_token,
            n: 3 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_MOVE_WORKSPACE_TO_OUTPUT as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_RESIZE_HEIGHT_GET_NUMBER as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_CRITERION_FROM_STR_END as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_MOVE_DIRECTION_NUMBER as *mut cmdp_token,
            n: 3 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_MOVE_WORKSPACE_NUMBER as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_HIDDEN_STATE_ID as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_RENAME_WORKSPACE_TO as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_RESIZE_TILING_FINAL as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_CRITERION_FROM_STR as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_MOVE_TO_POSITION_X as *mut cmdp_token,
            n: 3 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_MOVE_TO_POSITION_Y as *mut cmdp_token,
            n: 3 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_FULLSCREEN_COMPAT as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_TITLE_WINDOW_ICON as *mut cmdp_token,
            n: 14 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_HIDDEN_STATE as *mut cmdp_token,
            n: 3 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_MOVE_TO_POSITION as *mut cmdp_token,
            n: 5 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_RENAME_WORKSPACE as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_RESIZE_DIRECTION as *mut cmdp_token,
            n: 6 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_RESIZE_TILING_OR as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_WORKSPACE_NUMBER as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_FULLSCREEN_MODE as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_GAPS_WITH_SCOPE as *mut cmdp_token,
            n: 4 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_CRITERION_FROM as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_GAPS_WITH_MODE as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_GAPS_WITH_TYPE as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_MOVE_DIRECTION as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_MOVE_TO_OUTPUT as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_MOVE_WORKSPACE as *mut cmdp_token,
            n: 9 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_APPEND_LAYOUT as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_CRITERION_STR as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_LAYOUT_TOGGLE as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_RESIZE_HEIGHT as *mut cmdp_token,
            n: 3 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_RESIZE_TILING as *mut cmdp_token,
            n: 3 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_SWAP_ARGUMENT as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BORDER_WIDTH as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_FOCUS_OUTPUT as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_MOVE_TO_MARK as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_RESIZE_WIDTH as *mut cmdp_token,
            n: 5 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_TITLE_FORMAT as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_MODE_ID as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_FOCUS_AUTO as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_FULLSCREEN as *mut cmdp_token,
            n: 4 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_RESIZE_SET as *mut cmdp_token,
            n: 3 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_SCRATCHPAD as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_CRITERION as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_RESIZE_PX as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_WORKSPACE as *mut cmdp_token,
            n: 8 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_MODE as *mut cmdp_token,
            n: 4 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_CRITERIA as *mut cmdp_token,
            n: 17 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_DEBUGLOG as *mut cmdp_token,
            n: 3 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_FLOATING as *mut cmdp_token,
            n: 3 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_GAPS_END as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_INITIAL as *mut cmdp_token,
            n: 32 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BORDER as *mut cmdp_token,
            n: 5 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_LAYOUT as *mut cmdp_token,
            n: 7 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_RENAME as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_RESIZE as *mut cmdp_token,
            n: 3 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_SHMLOG as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_STICKY as *mut cmdp_token,
            n: 3 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_UNMARK as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_FOCUS as *mut cmdp_token,
            n: 14 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_SPLIT as *mut cmdp_token,
            n: 6 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_EXEC as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_GAPS as *mut cmdp_token,
            n: 8 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_KILL as *mut cmdp_token,
            n: 3 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_MARK as *mut cmdp_token,
            n: 4 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_MODE as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_MOVE as *mut cmdp_token,
            n: 14 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_SWAP as *mut cmdp_token,
            n: 5 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_NOP as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
    ]
};
unsafe extern "C" fn GENERATED_call(
    mut current_match: *mut Match,
    mut stack: *mut stack,
    call_identifier: ::core::ffi::c_int,
    mut result: *mut CommandResultIR,
) {
    match call_identifier {
        0 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_rename_workspace(
                current_match,
                result,
                b"to\0" as *const u8 as *const ::core::ffi::c_char,
                parser_get_string(
                    stack,
                    b"new_name\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        1 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_rename_workspace(
                current_match,
                result,
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"to\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        2 => {
            (*result).next_state = MOVE_WORKSPACE_TO_OUTPUT_WORD as ::core::ffi::c_int;
            cmd_move_con_to_output(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"output\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                1 as ::core::ffi::c_int != 0,
            );
        }
        3 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_move_con_to_output(
                current_match,
                result,
                ::core::ptr::null::<::core::ffi::c_char>(),
                1 as ::core::ffi::c_int != 0,
            );
        }
        4 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_rename_workspace(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"old_name\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                parser_get_string(
                    stack,
                    b"new_name\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        5 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_rename_workspace(
                current_match,
                result,
                ::core::ptr::null::<::core::ffi::c_char>(),
                parser_get_string(
                    stack,
                    b"new_name\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        6 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_title_window_icon(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"enable\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                parser_get_long(
                    stack,
                    b"padding\0" as *const u8 as *const ::core::ffi::c_char,
                ) as ::core::ffi::c_int,
            );
        }
        7 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_title_window_icon(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"enable\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                parser_get_long(
                    stack,
                    b"padding\0" as *const u8 as *const ::core::ffi::c_char,
                ) as ::core::ffi::c_int,
            );
        }
        8 => {
            (*result).next_state = CRITERIA as ::core::ffi::c_int;
            cmd_criteria_add(
                current_match,
                result,
                parser_get_string(stack, b"ctype\0" as *const u8 as *const ::core::ffi::c_char),
                parser_get_string(stack, b"kind\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        9 => {
            (*result).next_state = CRITERIA as ::core::ffi::c_int;
            cmd_criteria_add(
                current_match,
                result,
                parser_get_string(stack, b"ctype\0" as *const u8 as *const ::core::ffi::c_char),
                parser_get_string(stack, b"kind\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        10 => {
            (*result).next_state = CRITERIA as ::core::ffi::c_int;
            cmd_criteria_add(
                current_match,
                result,
                parser_get_string(stack, b"ctype\0" as *const u8 as *const ::core::ffi::c_char),
                parser_get_string(stack, b"kind\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        11 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_move_direction(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"direction\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                parser_get_long(
                    stack,
                    b"amount\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                parser_get_string(stack, b"mode\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        12 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_move_direction(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"direction\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                parser_get_long(
                    stack,
                    b"amount\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                parser_get_string(stack, b"mode\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        13 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_move_direction(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"direction\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                parser_get_long(
                    stack,
                    b"amount\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                b"px\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        14 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_move_con_to_workspace_number(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"number\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                parser_get_string(
                    stack,
                    b"no_auto_back_and_forth\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        15 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_bar_hidden_state(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"bar_value\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                parser_get_string(
                    stack,
                    b"bar_id\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        16 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_resize(
                current_match,
                result,
                parser_get_string(stack, b"way\0" as *const u8 as *const ::core::ffi::c_char),
                parser_get_string(
                    stack,
                    b"direction\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                parser_get_long(
                    stack,
                    b"resize_px\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                parser_get_long(
                    stack,
                    b"resize_ppt\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        17 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_resize(
                current_match,
                result,
                parser_get_string(stack, b"way\0" as *const u8 as *const ::core::ffi::c_char),
                parser_get_string(
                    stack,
                    b"direction\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                parser_get_long(
                    stack,
                    b"resize_px\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                parser_get_long(
                    stack,
                    b"resize_ppt\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        18 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_move_window_to_position(
                current_match,
                result,
                parser_get_long(
                    stack,
                    b"coord_x\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                parser_get_string(
                    stack,
                    b"mode_x\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                parser_get_long(
                    stack,
                    b"coord_y\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                parser_get_string(
                    stack,
                    b"mode_y\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        19 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_move_window_to_position(
                current_match,
                result,
                parser_get_long(
                    stack,
                    b"coord_x\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                parser_get_string(
                    stack,
                    b"mode_x\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                parser_get_long(
                    stack,
                    b"coord_y\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                parser_get_string(
                    stack,
                    b"mode_y\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        20 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_move_window_to_position(
                current_match,
                result,
                parser_get_long(
                    stack,
                    b"coord_x\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                parser_get_string(
                    stack,
                    b"mode_x\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                parser_get_long(
                    stack,
                    b"coord_y\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
        21 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_fullscreen(
                current_match,
                result,
                b"toggle\0" as *const u8 as *const ::core::ffi::c_char,
                parser_get_string(stack, b"mode\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        22 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_fullscreen(
                current_match,
                result,
                b"toggle\0" as *const u8 as *const ::core::ffi::c_char,
                b"output\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        23 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_title_window_icon(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"enable\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                0 as ::core::ffi::c_int,
            );
        }
        24 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_title_window_icon(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"enable\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                0 as ::core::ffi::c_int,
            );
        }
        25 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_title_window_icon(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"enable\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                0 as ::core::ffi::c_int,
            );
        }
        26 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_title_window_icon(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"enable\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                0 as ::core::ffi::c_int,
            );
        }
        27 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_title_window_icon(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"enable\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                0 as ::core::ffi::c_int,
            );
        }
        28 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_title_window_icon(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"enable\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                0 as ::core::ffi::c_int,
            );
        }
        29 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_title_window_icon(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"enable\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                0 as ::core::ffi::c_int,
            );
        }
        30 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_title_window_icon(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"enable\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                0 as ::core::ffi::c_int,
            );
        }
        31 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_title_window_icon(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"enable\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                0 as ::core::ffi::c_int,
            );
        }
        32 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_title_window_icon(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"enable\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                0 as ::core::ffi::c_int,
            );
        }
        33 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_title_window_icon(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"enable\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                0 as ::core::ffi::c_int,
            );
        }
        34 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_title_window_icon(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"enable\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                0 as ::core::ffi::c_int,
            );
        }
        35 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_move_window_to_center(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"method\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        36 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_move_window_to_mouse(current_match, result);
        }
        37 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_move_window_to_mouse(current_match, result);
        }
        38 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_move_window_to_mouse(current_match, result);
        }
        39 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_workspace_number(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"workspace\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                parser_get_string(
                    stack,
                    b"no_auto_back_and_forth\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        40 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_fullscreen(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"action\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                parser_get_string(stack, b"mode\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        41 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_fullscreen(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"action\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                b"output\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        42 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_move_direction(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"direction\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                10 as ::core::ffi::c_long,
                b"px\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        43 => {
            (*result).next_state = MOVE_TO_OUTPUT as ::core::ffi::c_int;
            cmd_move_con_to_output(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"output\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                0 as ::core::ffi::c_int != 0,
            );
        }
        44 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_move_con_to_output(
                current_match,
                result,
                ::core::ptr::null::<::core::ffi::c_char>(),
                0 as ::core::ffi::c_int != 0,
            );
        }
        45 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_move_con_to_workspace(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"workspace\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        46 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_move_con_to_workspace(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"workspace\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        47 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_move_con_to_workspace(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"workspace\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        48 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_move_con_to_workspace(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"workspace\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        49 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_move_con_to_workspace(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"workspace\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        50 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_move_con_to_workspace_back_and_forth(current_match, result);
        }
        51 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_move_con_to_workspace_name(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"workspace\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                parser_get_string(
                    stack,
                    b"no_auto_back_and_forth\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        52 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_append_layout(
                current_match,
                result,
                parser_get_string(stack, b"path\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        53 => {
            (*result).next_state = CRITERIA as ::core::ffi::c_int;
            cmd_criteria_add(
                current_match,
                result,
                parser_get_string(stack, b"ctype\0" as *const u8 as *const ::core::ffi::c_char),
                parser_get_string(
                    stack,
                    b"cvalue\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        54 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_layout_toggle(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"toggle_mode\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        55 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_layout_toggle(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"toggle_mode\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        56 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_resize_set(
                current_match,
                result,
                parser_get_long(stack, b"width\0" as *const u8 as *const ::core::ffi::c_char),
                parser_get_string(
                    stack,
                    b"mode_width\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                parser_get_long(
                    stack,
                    b"height\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                parser_get_string(
                    stack,
                    b"mode_height\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        57 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_resize(
                current_match,
                result,
                parser_get_string(stack, b"way\0" as *const u8 as *const ::core::ffi::c_char),
                parser_get_string(
                    stack,
                    b"direction\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                parser_get_long(
                    stack,
                    b"resize_px\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                0 as ::core::ffi::c_long,
            );
        }
        58 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_swap(
                current_match,
                result,
                parser_get_string(stack, b"mode\0" as *const u8 as *const ::core::ffi::c_char),
                parser_get_string(stack, b"arg\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        59 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_border(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"border_style\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                -(1 as ::core::ffi::c_int) as ::core::ffi::c_long,
            );
        }
        60 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_border(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"border_style\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                parser_get_long(
                    stack,
                    b"border_width\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        61 => {
            (*result).next_state = FOCUS_OUTPUT as ::core::ffi::c_int;
            cmd_focus_output(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"output\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        62 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_focus_output(
                current_match,
                result,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
        63 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_move_con_to_mark(
                current_match,
                result,
                parser_get_string(stack, b"mark\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        64 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_resize_set(
                current_match,
                result,
                parser_get_long(stack, b"width\0" as *const u8 as *const ::core::ffi::c_char),
                parser_get_string(
                    stack,
                    b"mode_width\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                0 as ::core::ffi::c_long,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
        65 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_title_format(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"format\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        66 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_bar_mode(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"bar_value\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                parser_get_string(
                    stack,
                    b"bar_id\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        67 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_focus_sibling(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"direction\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        68 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_focus_direction(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"direction\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        69 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_fullscreen(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"action\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                b"output\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        70 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_scratchpad_show(current_match, result);
        }
        71 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_resize(
                current_match,
                result,
                parser_get_string(stack, b"way\0" as *const u8 as *const ::core::ffi::c_char),
                parser_get_string(
                    stack,
                    b"direction\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                10 as ::core::ffi::c_long,
                10 as ::core::ffi::c_long,
            );
        }
        72 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_workspace(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"direction\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        73 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_workspace(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"direction\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        74 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_workspace(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"direction\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        75 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_workspace(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"direction\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        76 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_workspace_back_and_forth(current_match, result);
        }
        77 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_workspace_name(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"workspace\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                parser_get_string(
                    stack,
                    b"no_auto_back_and_forth\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        78 => {
            (*result).next_state = CRITERIA as ::core::ffi::c_int;
            cmd_criteria_add(
                current_match,
                result,
                parser_get_string(stack, b"ctype\0" as *const u8 as *const ::core::ffi::c_char),
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
        79 => {
            (*result).next_state = CRITERIA as ::core::ffi::c_int;
            cmd_criteria_add(
                current_match,
                result,
                parser_get_string(stack, b"ctype\0" as *const u8 as *const ::core::ffi::c_char),
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
        80 => {
            (*result).next_state = CRITERIA as ::core::ffi::c_int;
            cmd_criteria_add(
                current_match,
                result,
                parser_get_string(stack, b"ctype\0" as *const u8 as *const ::core::ffi::c_char),
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
        81 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_criteria_match_windows(current_match, result);
        }
        82 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_debuglog(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"argument\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        83 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_debuglog(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"argument\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        84 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_debuglog(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"argument\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        85 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_floating(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"floating\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        86 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_floating(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"floating\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        87 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_floating(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"floating\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        88 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_gaps(
                current_match,
                result,
                parser_get_string(stack, b"type\0" as *const u8 as *const ::core::ffi::c_char),
                parser_get_string(stack, b"scope\0" as *const u8 as *const ::core::ffi::c_char),
                parser_get_string(stack, b"mode\0" as *const u8 as *const ::core::ffi::c_char),
                parser_get_string(stack, b"value\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        89 => {
            (*result).next_state = CRITERIA as ::core::ffi::c_int;
            cmd_criteria_init(current_match, result);
        }
        90 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_exit(current_match, result);
        }
        91 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_restart(current_match, result);
        }
        92 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_reload(current_match, result);
        }
        93 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_open(current_match, result);
        }
        94 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_border(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"border_style\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                0 as ::core::ffi::c_long,
            );
        }
        95 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_border(
                current_match,
                result,
                b"pixel\0" as *const u8 as *const ::core::ffi::c_char,
                1 as ::core::ffi::c_long,
            );
        }
        96 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_layout(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"layout_mode\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        97 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_layout(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"layout_mode\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        98 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_layout(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"layout_mode\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        99 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_layout(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"layout_mode\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        100 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_layout(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"layout_mode\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        101 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_layout(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"layout_mode\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        102 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_shmlog(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"argument\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        103 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_sticky(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"action\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        104 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_sticky(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"action\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        105 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_sticky(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"action\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        106 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_unmark(
                current_match,
                result,
                parser_get_string(stack, b"mark\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        107 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_unmark(
                current_match,
                result,
                parser_get_string(stack, b"mark\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        108 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_focus_direction(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"direction\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        109 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_focus_direction(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"direction\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        110 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_focus_direction(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"direction\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        111 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_focus_direction(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"direction\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        112 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_focus_window_mode(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"window_mode\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        113 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_focus_window_mode(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"window_mode\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        114 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_focus_window_mode(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"window_mode\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        115 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_focus_level(
                current_match,
                result,
                parser_get_string(stack, b"level\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        116 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_focus_level(
                current_match,
                result,
                parser_get_string(stack, b"level\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        117 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_focus(current_match, result, 1 as ::core::ffi::c_int != 0);
        }
        118 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_focus(current_match, result, 0 as ::core::ffi::c_int != 0);
        }
        119 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_split(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"direction\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        120 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_split(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"direction\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        121 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_split(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"direction\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        122 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_split(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"direction\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        123 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_split(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"direction\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        124 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_split(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"direction\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        125 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_exec(
                current_match,
                result,
                parser_get_string(stack, b"nosn\0" as *const u8 as *const ::core::ffi::c_char),
                parser_get_string(
                    stack,
                    b"command\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        126 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_kill(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"kill_mode\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        127 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_kill(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"kill_mode\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        128 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_kill(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"kill_mode\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        129 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_mark(
                current_match,
                result,
                parser_get_string(stack, b"mark\0" as *const u8 as *const ::core::ffi::c_char),
                parser_get_string(stack, b"mode\0" as *const u8 as *const ::core::ffi::c_char),
                parser_get_string(
                    stack,
                    b"toggle\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        130 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_mode(
                current_match,
                result,
                parser_get_string(stack, b"mode\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        131 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_move_scratchpad(current_match, result);
        }
        132 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_nop(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"comment\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        133 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cmd_nop(
                current_match,
                result,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
        _ => {
            printf(
                b"BUG in the parser. state = %d\n\0" as *const u8 as *const ::core::ffi::c_char,
                call_identifier,
            );
            if 0 as ::core::ffi::c_int != 0 {
            } else {
                __assert_fail(
                    b"false\0" as *const u8 as *const ::core::ffi::c_char,
                    b"./GENERATED_command_call.h\0" as *const u8 as *const ::core::ffi::c_char,
                    1078 as ::core::ffi::c_uint,
                    __ASSERT_FUNCTION.as_ptr(),
                );
            };
        }
    };
}
unsafe extern "C" fn next_state(mut token: *const cmdp_token, mut cmd_ctx: *mut cmd_parser_ctx) {
    if (*token).next_state as ::core::ffi::c_uint
        == __CALL as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*cmd_ctx).subcommand_output.ctx = cmd_ctx as *mut cmd_parser_ctx;
        (*cmd_ctx).subcommand_output.json_gen = (*cmd_ctx).command_output.json_gen;
        (*cmd_ctx).subcommand_output.client = (*cmd_ctx).command_output.client;
        (*cmd_ctx).subcommand_output.needs_tree_render = false_0 != 0;
        GENERATED_call(
            &raw mut (*cmd_ctx).current_match,
            &raw mut (*cmd_ctx).stack,
            (*token).extra.call_identifier as ::core::ffi::c_int,
            &raw mut (*cmd_ctx).subcommand_output,
        );
        (*cmd_ctx).state = (*cmd_ctx).subcommand_output.next_state;
        if (*cmd_ctx).subcommand_output.needs_tree_render {
            (*cmd_ctx).command_output.needs_tree_render = true_0 != 0;
        }
        parser_clear_stack(&raw mut (*cmd_ctx).stack);
        return;
    }
    (*cmd_ctx).state = (*token).next_state as ::core::ffi::c_int;
    if (*cmd_ctx).state == INITIAL as ::core::ffi::c_int {
        parser_clear_stack(&raw mut (*cmd_ctx).stack);
    }
}
#[no_mangle]
pub unsafe extern "C" fn parse_string(
    mut walk: *mut *const ::core::ffi::c_char,
    as_word: bool,
) -> *mut ::core::ffi::c_char {
    let mut beginning: *const ::core::ffi::c_char = *walk;
    if **walk as ::core::ffi::c_int == '"' as i32 {
        beginning = beginning.offset(1);
        *walk = (*walk).offset(1);
        while **walk as ::core::ffi::c_int != '\0' as i32
            && **walk as ::core::ffi::c_int != '"' as i32
        {
            if **walk as ::core::ffi::c_int == '\\' as i32
                && *(*walk).offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    != '\0' as i32
            {
                *walk = (*walk).offset(1);
            }
            *walk = (*walk).offset(1);
        }
    } else if !as_word {
        while **walk as ::core::ffi::c_int != ';' as i32
            && **walk as ::core::ffi::c_int != ',' as i32
            && **walk as ::core::ffi::c_int != '\0' as i32
            && **walk as ::core::ffi::c_int != '\r' as i32
            && **walk as ::core::ffi::c_int != '\n' as i32
        {
            *walk = (*walk).offset(1);
        }
    } else {
        while **walk as ::core::ffi::c_int != ' ' as i32
            && **walk as ::core::ffi::c_int != '\t' as i32
            && **walk as ::core::ffi::c_int != ']' as i32
            && **walk as ::core::ffi::c_int != ',' as i32
            && **walk as ::core::ffi::c_int != ';' as i32
            && **walk as ::core::ffi::c_int != '\r' as i32
            && **walk as ::core::ffi::c_int != '\n' as i32
            && **walk as ::core::ffi::c_int != '\0' as i32
        {
            *walk = (*walk).offset(1);
        }
    }
    if *walk == beginning {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    let mut str: *mut ::core::ffi::c_char = scalloc(
        ((*walk).offset_from(beginning) as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
            as size_t,
        1 as size_t,
    ) as *mut ::core::ffi::c_char;
    let mut inpos: ::core::ffi::c_int = 0;
    let mut outpos: ::core::ffi::c_int = 0;
    inpos = 0 as ::core::ffi::c_int;
    outpos = 0 as ::core::ffi::c_int;
    while (inpos as ::core::ffi::c_long) < (*walk).offset_from(beginning) as ::core::ffi::c_long {
        if *beginning.offset(inpos as isize) as ::core::ffi::c_int == '\\' as i32
            && (*beginning.offset((inpos + 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                == '"' as i32
                || *beginning.offset((inpos + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    == '\\' as i32)
        {
            inpos += 1;
        }
        *str.offset(outpos as isize) = *beginning.offset(inpos as isize);
        inpos += 1;
        outpos += 1;
    }
    return str;
}
#[no_mangle]
pub unsafe extern "C" fn parse_command(
    mut input: *const ::core::ffi::c_char,
    mut gen: yajl_gen,
    mut client: *mut ipc_client,
) -> *mut CommandResult {
    debuglog(
        b"%s:%s:%d - COMMAND: *%.4000s*\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/commands_parser.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"parse_command\0" as *const u8 as *const ::core::ffi::c_char,
        159 as ::core::ffi::c_int,
        input,
    );
    let mut cmd_ctx: cmd_parser_ctx = cmd_parser_ctx {
        state: 0 as ::core::ffi::c_int,
        current_match: Match {
            error: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            title: ::core::ptr::null_mut::<regex>(),
            application: ::core::ptr::null_mut::<regex>(),
            class: ::core::ptr::null_mut::<regex>(),
            instance: ::core::ptr::null_mut::<regex>(),
            mark: ::core::ptr::null_mut::<regex>(),
            window_role: ::core::ptr::null_mut::<regex>(),
            workspace: ::core::ptr::null_mut::<regex>(),
            machine: ::core::ptr::null_mut::<regex>(),
            window_type: 0,
            urgent: U_LATEST,
            dock: M_NODOCK,
            id: 0,
            window_mode: WM_ANY,
            con_id: ::core::ptr::null_mut::<Con>(),
            match_all_windows: false,
            insert_where: M_HERE,
            matches: C2RustUnnamed_5 {
                tqe_next: ::core::ptr::null_mut::<Match>(),
                tqe_prev: ::core::ptr::null_mut::<*mut Match>(),
            },
            restart_mode: false,
        },
        stack: stack {
            stack: [stack_entry {
                identifier: ::core::ptr::null::<::core::ffi::c_char>(),
                type_0: STACK_STR,
                val: C2RustUnnamed_17 {
                    str_0: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                },
            }; 10],
        },
        owindows: owindows_head {
            tqh_first: ::core::ptr::null_mut::<owindow>(),
            tqh_last: ::core::ptr::null_mut::<*mut owindow>(),
        },
        subcommand_output: CommandResultIR {
            ctx: ::core::ptr::null_mut::<cmd_parser_ctx>(),
            json_gen: ::core::ptr::null_mut::<yajl_gen_t>(),
            client: ::core::ptr::null_mut::<ipc_client>(),
            next_state: 0,
            needs_tree_render: false,
        },
        command_output: CommandResultIR {
            ctx: ::core::ptr::null_mut::<cmd_parser_ctx>(),
            json_gen: ::core::ptr::null_mut::<yajl_gen_t>(),
            client: ::core::ptr::null_mut::<ipc_client>(),
            next_state: 0,
            needs_tree_render: false,
        },
    };
    cmd_ctx.state = INITIAL as ::core::ffi::c_int;
    let mut result: *mut CommandResult = scalloc(
        1 as size_t,
        ::core::mem::size_of::<CommandResult>() as size_t,
    ) as *mut CommandResult;
    cmd_ctx.command_output.ctx = &raw mut cmd_ctx as *mut cmd_parser_ctx;
    cmd_ctx.command_output.client = client;
    cmd_ctx.command_output.json_gen = gen;
    if !cmd_ctx.command_output.json_gen.is_null() {
        yajl_gen_array_open(cmd_ctx.command_output.json_gen);
    } else {
    };
    cmd_ctx.command_output.needs_tree_render = false_0 != 0;
    let mut walk: *const ::core::ffi::c_char = input;
    let len: size_t = strlen(input) as size_t;
    let mut c: ::core::ffi::c_int = 0;
    let mut token: *const cmdp_token = ::core::ptr::null::<cmdp_token>();
    let mut token_handled: bool = false;
    cmd_criteria_init(
        &raw mut cmd_ctx.current_match,
        &raw mut cmd_ctx.subcommand_output,
    );
    while walk.offset_from(input) as ::core::ffi::c_long as size_t <= len {
        while (*walk as ::core::ffi::c_int == ' ' as i32
            || *walk as ::core::ffi::c_int == '\t' as i32
            || *walk as ::core::ffi::c_int == '\r' as i32
            || *walk as ::core::ffi::c_int == '\n' as i32)
            && *walk as ::core::ffi::c_int != '\0' as i32
        {
            walk = walk.offset(1);
        }
        let mut ptr: *const cmdp_token_ptr = (&raw mut tokens as *mut cmdp_token_ptr)
            .offset(cmd_ctx.state as isize)
            as *mut cmdp_token_ptr;
        token_handled = false_0 != 0;
        c = 0 as ::core::ffi::c_int;
        while c < (*ptr).n {
            token = (*ptr).array.offset(c as isize) as *mut cmdp_token;
            if *(*token).name.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '\'' as i32
            {
                if strncasecmp(
                    walk,
                    (*token).name.offset(1 as ::core::ffi::c_int as isize),
                    strlen((*token).name).wrapping_sub(1 as size_t),
                ) == 0 as ::core::ffi::c_int
                {
                    if !(*token).identifier.is_null() {
                        parser_push_string(
                            &raw mut cmd_ctx.stack,
                            (*token).identifier,
                            (*token).name.offset(1 as ::core::ffi::c_int as isize),
                        );
                    }
                    walk = walk.offset(strlen((*token).name).wrapping_sub(1 as size_t) as isize);
                    next_state(token, &raw mut cmd_ctx);
                    token_handled = true_0 != 0;
                    break;
                }
            } else if strcmp(
                (*token).name,
                b"number\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                let mut end: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                *__errno_location() = 0 as ::core::ffi::c_int;
                let num: ::core::ffi::c_long =
                    strtol(walk, &raw mut end, 10 as ::core::ffi::c_int) as ::core::ffi::c_long;
                if !(*__errno_location() == ERANGE && (num == LONG_MIN || num == LONG_MAX)
                    || *__errno_location() != 0 as ::core::ffi::c_int
                        && num == 0 as ::core::ffi::c_long)
                {
                    if !(end == walk as *mut ::core::ffi::c_char) {
                        if !(*token).identifier.is_null() {
                            parser_push_long(&raw mut cmd_ctx.stack, (*token).identifier, num);
                        }
                        walk = end;
                        next_state(token, &raw mut cmd_ctx);
                        token_handled = true_0 != 0;
                        break;
                    }
                }
            } else {
                if strcmp(
                    (*token).name,
                    b"string\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                    || strcmp(
                        (*token).name,
                        b"word\0" as *const u8 as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                {
                    let mut str: *mut ::core::ffi::c_char = parse_string(
                        &raw mut walk,
                        *(*token).name.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            != 's' as i32,
                    );
                    if !str.is_null() {
                        if !(*token).identifier.is_null() {
                            parser_push_string(&raw mut cmd_ctx.stack, (*token).identifier, str);
                        }
                        free(str as *mut ::core::ffi::c_void);
                        if *walk as ::core::ffi::c_int == '"' as i32 {
                            walk = walk.offset(1);
                        }
                        next_state(token, &raw mut cmd_ctx);
                        token_handled = true_0 != 0;
                        break;
                    }
                }
                if strcmp(
                    (*token).name,
                    b"end\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    if *walk as ::core::ffi::c_int == '\0' as i32
                        || *walk as ::core::ffi::c_int == ',' as i32
                        || *walk as ::core::ffi::c_int == ';' as i32
                    {
                        next_state(token, &raw mut cmd_ctx);
                        token_handled = true_0 != 0;
                        if *walk as ::core::ffi::c_int == '\0' as i32
                            || *walk as ::core::ffi::c_int == ';' as i32
                        {
                            cmd_criteria_init(
                                &raw mut cmd_ctx.current_match,
                                &raw mut cmd_ctx.subcommand_output,
                            );
                        }
                        walk = walk.offset(1);
                        break;
                    }
                }
            }
            c += 1;
        }
        if token_handled {
            continue;
        }
        let mut tokenlen: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        c = 0 as ::core::ffi::c_int;
        while c < (*ptr).n {
            tokenlen = (tokenlen as size_t).wrapping_add(
                strlen((*(*ptr).array.offset(c as isize)).name)
                    .wrapping_add(strlen(b"'', \0" as *const u8 as *const ::core::ffi::c_char)),
            ) as ::core::ffi::c_int as ::core::ffi::c_int;
            c += 1;
        }
        let mut errormessage: *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut possible_tokens: *mut ::core::ffi::c_char =
            smalloc((tokenlen + 1 as ::core::ffi::c_int) as size_t) as *mut ::core::ffi::c_char;
        let mut tokenwalk: *mut ::core::ffi::c_char = possible_tokens;
        c = 0 as ::core::ffi::c_int;
        while c < (*ptr).n {
            token = (*ptr).array.offset(c as isize) as *mut cmdp_token;
            if *(*token).name.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '\'' as i32
            {
                let fresh0 = tokenwalk;
                tokenwalk = tokenwalk.offset(1);
                *fresh0 = '\'' as i32 as ::core::ffi::c_char;
                strcpy(
                    tokenwalk,
                    (*token).name.offset(1 as ::core::ffi::c_int as isize),
                );
                tokenwalk = tokenwalk.offset(strlen(
                    (*token).name.offset(1 as ::core::ffi::c_int as isize),
                ) as isize);
                let fresh1 = tokenwalk;
                tokenwalk = tokenwalk.offset(1);
                *fresh1 = '\'' as i32 as ::core::ffi::c_char;
            } else {
                let fresh2 = tokenwalk;
                tokenwalk = tokenwalk.offset(1);
                *fresh2 = '<' as i32 as ::core::ffi::c_char;
                strcpy(tokenwalk, (*token).name);
                tokenwalk = tokenwalk.offset(strlen((*token).name) as isize);
                let fresh3 = tokenwalk;
                tokenwalk = tokenwalk.offset(1);
                *fresh3 = '>' as i32 as ::core::ffi::c_char;
            }
            if c < (*ptr).n - 1 as ::core::ffi::c_int {
                let fresh4 = tokenwalk;
                tokenwalk = tokenwalk.offset(1);
                *fresh4 = ',' as i32 as ::core::ffi::c_char;
                let fresh5 = tokenwalk;
                tokenwalk = tokenwalk.offset(1);
                *fresh5 = ' ' as i32 as ::core::ffi::c_char;
            }
            c += 1;
        }
        *tokenwalk = '\0' as i32 as ::core::ffi::c_char;
        sasprintf(
            &raw mut errormessage,
            b"Expected one of these tokens: %s\0" as *const u8 as *const ::core::ffi::c_char,
            possible_tokens,
        );
        free(possible_tokens as *mut ::core::ffi::c_void);
        let mut position: *mut ::core::ffi::c_char =
            smalloc(len.wrapping_add(1 as size_t)) as *mut ::core::ffi::c_char;
        let mut copywalk: *const ::core::ffi::c_char = input;
        while *copywalk as ::core::ffi::c_int != '\0' as i32 {
            *position.offset(copywalk.offset_from(input) as ::core::ffi::c_long as isize) =
                (if copywalk >= walk {
                    '^' as i32
                } else {
                    ' ' as i32
                }) as ::core::ffi::c_char;
            copywalk = copywalk.offset(1);
        }
        *position.offset(len as isize) = '\0' as i32 as ::core::ffi::c_char;
        errorlog(
            b"ERROR: %s\n\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            errormessage,
        );
        errorlog(
            b"ERROR: Your command: %s\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            input,
        );
        errorlog(
            b"ERROR:               %s\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            position,
        );
        (*result).parse_error = true_0 != 0;
        (*result).error_message = errormessage;
        if !cmd_ctx.command_output.json_gen.is_null() {
            yajl_gen_map_open(cmd_ctx.command_output.json_gen);
        } else {
        };
        if !cmd_ctx.command_output.json_gen.is_null() {
            yajl_gen_string(
                cmd_ctx.command_output.json_gen,
                b"success\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_uchar,
                strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
            );
        } else {
        };
        if !cmd_ctx.command_output.json_gen.is_null() {
            yajl_gen_bool(cmd_ctx.command_output.json_gen, false_0);
        } else {
        };
        if !cmd_ctx.command_output.json_gen.is_null() {
            yajl_gen_string(
                cmd_ctx.command_output.json_gen,
                b"parse_error\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_uchar,
                strlen(b"parse_error\0" as *const u8 as *const ::core::ffi::c_char),
            );
        } else {
        };
        if !cmd_ctx.command_output.json_gen.is_null() {
            yajl_gen_bool(cmd_ctx.command_output.json_gen, true_0);
        } else {
        };
        if !cmd_ctx.command_output.json_gen.is_null() {
            yajl_gen_string(
                cmd_ctx.command_output.json_gen,
                b"error\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
                strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
            );
        } else {
        };
        if !cmd_ctx.command_output.json_gen.is_null() {
            yajl_gen_string(
                cmd_ctx.command_output.json_gen,
                errormessage as *mut ::core::ffi::c_uchar,
                strlen(errormessage),
            );
        } else {
        };
        if !cmd_ctx.command_output.json_gen.is_null() {
            yajl_gen_string(
                cmd_ctx.command_output.json_gen,
                b"input\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
                strlen(b"input\0" as *const u8 as *const ::core::ffi::c_char),
            );
        } else {
        };
        if !cmd_ctx.command_output.json_gen.is_null() {
            yajl_gen_string(
                cmd_ctx.command_output.json_gen,
                input as *mut ::core::ffi::c_uchar,
                strlen(input),
            );
        } else {
        };
        if !cmd_ctx.command_output.json_gen.is_null() {
            yajl_gen_string(
                cmd_ctx.command_output.json_gen,
                b"errorposition\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_uchar,
                strlen(b"errorposition\0" as *const u8 as *const ::core::ffi::c_char),
            );
        } else {
        };
        if !cmd_ctx.command_output.json_gen.is_null() {
            yajl_gen_string(
                cmd_ctx.command_output.json_gen,
                position as *mut ::core::ffi::c_uchar,
                strlen(position),
            );
        } else {
        };
        if !cmd_ctx.command_output.json_gen.is_null() {
            yajl_gen_map_close(cmd_ctx.command_output.json_gen);
        } else {
        };
        free(position as *mut ::core::ffi::c_void);
        parser_clear_stack(&raw mut cmd_ctx.stack);
        break;
    }
    if !cmd_ctx.command_output.json_gen.is_null() {
        yajl_gen_array_close(cmd_ctx.command_output.json_gen);
    } else {
    };
    (*result).needs_tree_render = cmd_ctx.command_output.needs_tree_render;
    while !cmd_ctx.owindows.tqh_first.is_null() {
        let mut ow: *mut owindow = cmd_ctx.owindows.tqh_first;
        if !(*ow).owindows.tqe_next.is_null() {
            (*(*ow).owindows.tqe_next).owindows.tqe_prev = (*ow).owindows.tqe_prev;
        } else {
            cmd_ctx.owindows.tqh_last = (*ow).owindows.tqe_prev;
        }
        *(*ow).owindows.tqe_prev = (*ow).owindows.tqe_next;
        free(ow as *mut ::core::ffi::c_void);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn command_result_free(mut result: *mut CommandResult) {
    if result.is_null() {
        return;
    }
    free((*result).error_message as *mut ::core::ffi::c_void);
    (*result).error_message = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free(result as *mut ::core::ffi::c_void);
    result = ::core::ptr::null_mut::<CommandResult>();
}
pub const ERANGE: ::core::ffi::c_int = 34 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
