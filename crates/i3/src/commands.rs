/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * commands.c: all command functions (see commands_parser.c)
 *
 */
// Mechanical file-by-file Rust translation of src/commands.c.
// Generated with C2Rust 0.22.1; C ABI and original ownership retained.
extern "C" {
    pub type _PangoFontDescription;
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
    fn unlink(__name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn verboselog(fmt: *mut ::core::ffi::c_char, ...);
    fn errorlog(fmt: *mut ::core::ffi::c_char, ...);
    fn debuglog(fmt: *mut ::core::ffi::c_char, ...);
    fn smalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn scalloc(num: size_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn sstrdup(str: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn sasprintf(
        strp: *mut *mut ::core::ffi::c_char,
        fmt: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn i3string_free(str: *mut i3String);
    fn i3string_as_utf8(str: *mut i3String) -> *const ::core::ffi::c_char;
    fn logical_px(logical: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn resolve_tilde(path: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn boolstr(str: *const ::core::ffi::c_char) -> bool;
    fn rect_equals(a: Rect, b: Rect) -> bool;
    fn layout_from_name(layout_str: *const ::core::ffi::c_char, out: *mut layout_t) -> bool;
    fn ws_name_to_number(name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn i3_restart(forget_layout: bool);
    fn kill_nagbar(nagbar_pid: pid_t, wait_for_it: bool);
    fn parse_long(
        str: *const ::core::ffi::c_char,
        out: *mut ::core::ffi::c_long,
        base: ::core::ffi::c_int,
    ) -> bool;
    fn slurp(path: *const ::core::ffi::c_char, buf: *mut *mut ::core::ffi::c_char) -> ssize_t;
    fn orientation_from_direction(direction: direction_t) -> orientation_t;
    fn direction_from_orientation_position(
        orientation: orientation_t,
        position: position_t,
    ) -> direction_t;
    fn yajl_gen_integer(hand: yajl_gen, number: ::core::ffi::c_longlong) -> yajl_gen_status;
    fn yajl_gen_string(
        hand: yajl_gen,
        str: *const ::core::ffi::c_uchar,
        len: size_t,
    ) -> yajl_gen_status;
    fn yajl_gen_bool(hand: yajl_gen, boolean: ::core::ffi::c_int) -> yajl_gen_status;
    fn yajl_gen_map_open(hand: yajl_gen) -> yajl_gen_status;
    fn yajl_gen_map_close(hand: yajl_gen) -> yajl_gen_status;
    static mut croot: *mut Con;
    static mut focused: *mut Con;
    static mut all_cons: all_cons_head;
    fn tree_open_con(con: *mut Con, window: *mut i3Window) -> *mut Con;
    fn tree_split(con: *mut Con, orientation: orientation_t);
    fn level_up() -> bool;
    fn level_down() -> bool;
    fn tree_next(con: *mut Con, direction: direction_t);
    fn get_tree_next_sibling(con: *mut Con, direction: position_t) -> *mut Con;
    static mut config: Config;
    static mut barconfigs: barconfig_head;
    fn load_configuration(
        override_configfile: *const ::core::ffi::c_char,
        load_type: config_load_t,
    ) -> bool;
    fn ipc_shutdown(reason: shutdown_reason_t, exempt_fd: ::core::ffi::c_int);
    fn ipc_send_workspace_event(
        change: *const ::core::ffi::c_char,
        current: *mut Con,
        old: *mut Con,
    );
    fn ipc_send_barconfig_update_event(barconfig: *mut Barconfig);
    static mut shmlog_size: ::core::ffi::c_int;
    static mut current_log_stream_socket_path: *mut ::core::ffi::c_char;
    fn init_logging();
    fn get_debug_logging() -> bool;
    fn set_debug_logging(_debug_logging: bool);
    fn purge_zerobyte_logfile();
    static mut outputs: outputs_head;
    static mut previous_workspace_name: *mut ::core::ffi::c_char;
    fn get_existing_workspace_by_name(name: *const ::core::ffi::c_char) -> *mut Con;
    fn get_existing_workspace_by_num(num: ::core::ffi::c_int) -> *mut Con;
    fn get_assigned_output(
        name: *const ::core::ffi::c_char,
        parsed_num: ::core::ffi::c_long,
    ) -> *mut Con;
    fn workspace_get(num: *const ::core::ffi::c_char) -> *mut Con;
    fn workspace_is_visible(ws: *mut Con) -> bool;
    fn workspace_show(ws: *mut Con);
    fn workspace_show_by_name(num: *const ::core::ffi::c_char);
    fn workspace_next() -> *mut Con;
    fn workspace_prev() -> *mut Con;
    fn workspace_next_on_output() -> *mut Con;
    fn workspace_prev_on_output() -> *mut Con;
    fn workspace_back_and_forth();
    fn workspace_back_and_forth_get() -> *mut Con;
    fn workspace_move_to_output(ws: *mut Con, output: *mut Output);
    fn update_shmlog_atom();
    fn x_set_i3_atoms();
    fn floating_enable(con: *mut Con, automatic: bool) -> bool;
    fn floating_disable(con: *mut Con);
    fn toggle_floating_mode(con: *mut Con, automatic: bool);
    fn floating_maybe_reassign_ws(con: *mut Con) -> bool;
    fn floating_center(con: *mut Con, rect: Rect);
    fn floating_move_to_pointer(con: *mut Con);
    fn floating_check_size(floating_con: *mut Con, prefer_height: bool);
    fn floating_reposition(con: *mut Con, newrect: Rect) -> bool;
    fn floating_resize(floating_con: *mut Con, x: uint32_t, y: uint32_t);
    fn con_focus(con: *mut Con);
    fn con_activate(con: *mut Con);
    fn con_activate_unblock(con: *mut Con);
    fn con_close(con: *mut Con, kill_window: kill_window_t);
    fn con_has_children(con: *mut Con) -> bool;
    fn con_accepts_window(con: *mut Con) -> bool;
    fn con_get_output(con: *mut Con) -> *mut Con;
    fn con_get_workspace(con: *mut Con) -> *mut Con;
    fn con_get_fullscreen_con(con: *mut Con, fullscreen_mode: fullscreen_mode_t) -> *mut Con;
    fn con_is_internal(con: *mut Con) -> bool;
    fn con_is_floating(con: *mut Con) -> bool;
    fn con_is_docked(con: *mut Con) -> bool;
    fn con_inside_floating(con: *mut Con) -> *mut Con;
    fn con_by_window_id(window: xcb_window_t) -> *mut Con;
    fn con_by_con_id(target: ::core::ffi::c_long) -> *mut Con;
    fn con_by_mark(mark: *const ::core::ffi::c_char) -> *mut Con;
    fn con_mark_toggle(con: *mut Con, mark: *const ::core::ffi::c_char, mode: mark_mode_t);
    fn con_mark(con: *mut Con, mark: *const ::core::ffi::c_char, mode: mark_mode_t);
    fn con_unmark(con: *mut Con, name: *const ::core::ffi::c_char);
    fn con_num_children(con: *mut Con) -> ::core::ffi::c_int;
    fn con_attach(con: *mut Con, parent: *mut Con, ignore_focus: bool);
    fn con_detach(con: *mut Con);
    fn con_toggle_fullscreen(con: *mut Con, fullscreen_mode: ::core::ffi::c_int);
    fn con_enable_fullscreen(con: *mut Con, fullscreen_mode: fullscreen_mode_t);
    fn con_disable_fullscreen(con: *mut Con);
    fn con_move_to_workspace(
        con: *mut Con,
        workspace: *mut Con,
        fix_coordinates: bool,
        dont_warp: bool,
        ignore_focus: bool,
    );
    fn con_move_to_output(con: *mut Con, output: *mut Output, fix_coordinates: bool);
    fn con_move_to_mark(con: *mut Con, mark: *const ::core::ffi::c_char) -> bool;
    fn con_orientation(con: *mut Con) -> orientation_t;
    fn con_descend_focused(con: *mut Con) -> *mut Con;
    fn con_set_border_style(
        con: *mut Con,
        border_style: border_style_t,
        border_width: ::core::ffi::c_int,
    );
    fn con_set_layout(con: *mut Con, layout: layout_t);
    fn con_toggle_layout(con: *mut Con, toggle_mode: *const ::core::ffi::c_char);
    fn con_fullscreen_permits_focusing(con: *mut Con) -> bool;
    fn con_parse_title_format(con: *mut Con) -> *mut i3String;
    fn con_swap(first: *mut Con, second: *mut Con) -> bool;
    fn con_rect_size_in_orientation(con: *mut Con) -> uint32_t;
    fn json_determine_content(buf: *const ::core::ffi::c_char, len: size_t) -> json_content_t;
    fn json_validate(buf: *const ::core::ffi::c_char, len: size_t) -> bool;
    fn tree_append_json(
        con: *mut Con,
        buf: *const ::core::ffi::c_char,
        len: size_t,
        errormsg: *mut *mut ::core::ffi::c_char,
    );
    fn render_con(con: *mut Con);
    fn match_init(match_0: *mut Match);
    fn match_is_empty(match_0: *mut Match) -> bool;
    fn match_matches_window(match_0: *mut Match, window: *mut i3Window) -> bool;
    fn match_free(match_0: *mut Match);
    fn match_parse_property(
        match_0: *mut Match,
        ctype: *const ::core::ffi::c_char,
        cvalue: *const ::core::ffi::c_char,
    );
    fn resize_find_tiling_participants(
        current: *mut *mut Con,
        other: *mut *mut Con,
        direction: direction_t,
        both_sides: bool,
    ) -> bool;
    fn resize_neighboring_cons(
        first: *mut Con,
        second: *mut Con,
        px: ::core::ffi::c_int,
        ppt: ::core::ffi::c_int,
    ) -> bool;
    fn percent_for_1px(con: *mut Con) -> ::core::ffi::c_double;
    fn tree_move(con: *mut Con, direction: direction_t);
    fn output_get_content(output: *mut Con) -> *mut Con;
    fn get_output_from_string(
        current_output: *mut Output,
        output_str: *const ::core::ffi::c_char,
    ) -> *mut Output;
    fn output_primary_name(output: *mut Output) -> *mut ::core::ffi::c_char;
    fn get_output_for_con(con: *mut Con) -> *mut Output;
    fn output_push_sticky_windows(old_focus: *mut Con);
    fn ewmh_update_desktop_properties();
    fn ewmh_update_wm_desktop();
    fn ewmh_update_visible_name(window: xcb_window_t, name: *const ::core::ffi::c_char);
    fn ewmh_update_sticky(window: xcb_window_t, sticky: bool);
    fn run_assignments(window: *mut i3Window);
    fn regex_matches(regex: *const regex, input: *const ::core::ffi::c_char) -> bool;
    fn start_application(
        command: *const ::core::ffi::c_char,
        no_startup_id: bool,
        window_id: xcb_window_t,
    );
    fn startup_sequence_rename_workspace(
        old_name: *const ::core::ffi::c_char,
        new_name: *const ::core::ffi::c_char,
    );
    fn scratchpad_move(con: *mut Con);
    fn scratchpad_show(con: *mut Con) -> bool;
    static mut command_error_nagbar_pid: pid_t;
    fn switch_mode(new_mode: *const ::core::ffi::c_char);
    static mut config_error_nagbar_pid: pid_t;
    fn restore_open_placeholder_windows(con: *mut Con);
    static default_shmlog_size: ::core::ffi::c_int;
    fn fcntl(__fd: ::core::ffi::c_int, __cmd: ::core::ffi::c_int, ...) -> ::core::ffi::c_int;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn strcasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strncasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn atoi(__nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn setenv(
        __name: *const ::core::ffi::c_char,
        __value: *const ::core::ffi::c_char,
        __replace: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __pid_t = ::core::ffi::c_int;
pub type __time_t = ::core::ffi::c_long;
pub type __suseconds_t = ::core::ffi::c_long;
pub type pid_t = __pid_t;
pub type ssize_t = isize;
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
pub type uintptr_t = usize;
pub type xcb_window_t = uint32_t;
pub type xcb_font_t = uint32_t;
pub type xcb_gcontext_t = uint32_t;
pub type xcb_colormap_t = uint32_t;
pub type xcb_atom_t = uint32_t;
pub type xcb_drawable_t = uint32_t;
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
pub type mark_mode_t = ::core::ffi::c_uint;
pub const MM_ADD: mark_mode_t = 1;
pub const MM_REPLACE: mark_mode_t = 0;
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
pub struct all_cons_head {
    pub tqh_first: *mut Con,
    pub tqh_last: *mut *mut Con,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Barconfig {
    pub id: *mut ::core::ffi::c_char,
    pub num_outputs: ::core::ffi::c_int,
    pub outputs: *mut *mut ::core::ffi::c_char,
    pub tray_outputs: tray_outputs_head,
    pub tray_padding: ::core::ffi::c_int,
    pub socket_path: *mut ::core::ffi::c_char,
    pub mode: C2RustUnnamed_28,
    pub hidden_state: C2RustUnnamed_27,
    pub modifier: uint32_t,
    pub bar_bindings: bar_bindings_head,
    pub position: C2RustUnnamed_25,
    pub i3bar_command: *mut ::core::ffi::c_char,
    pub status_command: *mut ::core::ffi::c_char,
    pub workspace_command: *mut ::core::ffi::c_char,
    pub font: *mut ::core::ffi::c_char,
    pub separator_symbol: *mut ::core::ffi::c_char,
    pub hide_workspace_buttons: bool,
    pub workspace_min_width: ::core::ffi::c_int,
    pub strip_workspace_numbers: bool,
    pub strip_workspace_name: bool,
    pub hide_binding_mode_indicator: bool,
    pub verbose: bool,
    pub bar_height: uint32_t,
    pub padding: Rect,
    pub colors: bar_colors,
    pub configs: C2RustUnnamed_24,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_24 {
    pub tqe_next: *mut Barconfig,
    pub tqe_prev: *mut *mut Barconfig,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bar_colors {
    pub background: *mut ::core::ffi::c_char,
    pub statusline: *mut ::core::ffi::c_char,
    pub separator: *mut ::core::ffi::c_char,
    pub focused_background: *mut ::core::ffi::c_char,
    pub focused_statusline: *mut ::core::ffi::c_char,
    pub focused_separator: *mut ::core::ffi::c_char,
    pub focused_workspace_border: *mut ::core::ffi::c_char,
    pub focused_workspace_bg: *mut ::core::ffi::c_char,
    pub focused_workspace_text: *mut ::core::ffi::c_char,
    pub active_workspace_border: *mut ::core::ffi::c_char,
    pub active_workspace_bg: *mut ::core::ffi::c_char,
    pub active_workspace_text: *mut ::core::ffi::c_char,
    pub inactive_workspace_border: *mut ::core::ffi::c_char,
    pub inactive_workspace_bg: *mut ::core::ffi::c_char,
    pub inactive_workspace_text: *mut ::core::ffi::c_char,
    pub urgent_workspace_border: *mut ::core::ffi::c_char,
    pub urgent_workspace_bg: *mut ::core::ffi::c_char,
    pub urgent_workspace_text: *mut ::core::ffi::c_char,
    pub binding_mode_border: *mut ::core::ffi::c_char,
    pub binding_mode_bg: *mut ::core::ffi::c_char,
    pub binding_mode_text: *mut ::core::ffi::c_char,
}
pub type C2RustUnnamed_25 = ::core::ffi::c_uint;
pub const P_TOP: C2RustUnnamed_25 = 1;
pub const P_BOTTOM: C2RustUnnamed_25 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bar_bindings_head {
    pub tqh_first: *mut Barbinding,
    pub tqh_last: *mut *mut Barbinding,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Barbinding {
    pub input_code: ::core::ffi::c_int,
    pub command: *mut ::core::ffi::c_char,
    pub release: bool,
    pub bindings: C2RustUnnamed_26,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_26 {
    pub tqe_next: *mut Barbinding,
    pub tqe_prev: *mut *mut Barbinding,
}
pub type C2RustUnnamed_27 = ::core::ffi::c_uint;
pub const S_SHOW: C2RustUnnamed_27 = 1;
pub const S_HIDE: C2RustUnnamed_27 = 0;
pub type C2RustUnnamed_28 = ::core::ffi::c_uint;
pub const M_INVISIBLE: C2RustUnnamed_28 = 2;
pub const M_HIDE: C2RustUnnamed_28 = 1;
pub const M_DOCK: C2RustUnnamed_28 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tray_outputs_head {
    pub tqh_first: *mut tray_output_t,
    pub tqh_last: *mut *mut tray_output_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tray_output_t {
    pub output: *mut ::core::ffi::c_char,
    pub tray_outputs: C2RustUnnamed_29,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_29 {
    pub tqe_next: *mut tray_output_t,
    pub tqe_prev: *mut *mut tray_output_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct barconfig_head {
    pub tqh_first: *mut Barconfig,
    pub tqh_last: *mut *mut Barconfig,
}
pub type config_load_t = ::core::ffi::c_uint;
pub const C_RELOAD: config_load_t = 2;
pub const C_LOAD: config_load_t = 1;
pub const C_VALIDATE: config_load_t = 0;
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
    pub clients: C2RustUnnamed_30,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_30 {
    pub tqe_next: *mut ipc_client,
    pub tqe_prev: *mut *mut ipc_client,
}
pub type shutdown_reason_t = ::core::ffi::c_uint;
pub const SHUTDOWN_REASON_EXIT: shutdown_reason_t = 1;
pub const SHUTDOWN_REASON_RESTART: shutdown_reason_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct outputs_head {
    pub tqh_first: *mut xoutput,
    pub tqh_last: *mut *mut xoutput,
}
pub type json_content_t = ::core::ffi::c_uint;
pub const JSON_CONTENT_WORKSPACE: json_content_t = 2;
pub const JSON_CONTENT_CON: json_content_t = 1;
pub const JSON_CONTENT_UNKNOWN: json_content_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack_entry {
    pub identifier: *const ::core::ffi::c_char,
    pub type_0: C2RustUnnamed_32,
    pub val: C2RustUnnamed_31,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_31 {
    pub str_0: *mut ::core::ffi::c_char,
    pub num: ::core::ffi::c_long,
}
pub type C2RustUnnamed_32 = ::core::ffi::c_uint;
pub const STACK_LONG: C2RustUnnamed_32 = 1;
pub const STACK_STR: C2RustUnnamed_32 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack {
    pub stack: [stack_entry; 10],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct owindow {
    pub con: *mut Con,
    pub owindows: C2RustUnnamed_33,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_33 {
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
pub struct user_output_names_head {
    pub tqh_first: *mut user_output_name,
    pub tqh_last: *mut *mut user_output_name,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct user_output_name {
    pub name: *mut ::core::ffi::c_char,
    pub user_output_names: C2RustUnnamed_34,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_34 {
    pub tqe_next: *mut user_output_name,
    pub tqe_prev: *mut *mut user_output_name,
}
pub type gap_accessor = Option<unsafe extern "C" fn(*mut gaps_t) -> *mut ::core::ffi::c_int>;
static mut names: user_output_names_head = user_output_names_head {
    tqh_first: ::core::ptr::null::<user_output_name>() as *mut user_output_name,
    tqh_last: ::core::ptr::null::<*mut user_output_name>() as *mut *mut user_output_name,
};
static mut names_global: user_output_names_head = user_output_names_head {
    tqh_first: ::core::ptr::null::<user_output_name>() as *mut user_output_name,
    tqh_last: ::core::ptr::null::<*mut user_output_name>() as *mut *mut user_output_name,
};
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const XCB_NONE: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const F_GETFD: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const F_SETFD: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const FD_CLOEXEC: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
unsafe extern "C" fn maybe_back_and_forth(
    mut cmd_output: *mut CommandResultIR,
    mut name: *const ::core::ffi::c_char,
) -> bool {
    let mut ws: *const Con = con_get_workspace(focused);
    if strcmp((*ws).name, name) != 0 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    debuglog(
        b"%s:%s:%d - This workspace is already focused.\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/commands.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"maybe_back_and_forth\0" as *const u8 as *const ::core::ffi::c_char,
        91 as ::core::ffi::c_int,
    );
    if config.workspace_auto_back_and_forth {
        workspace_back_and_forth();
        (*cmd_output).needs_tree_render = true_0 != 0;
    }
    return true_0 != 0;
}
unsafe extern "C" fn maybe_auto_back_and_forth_workspace(mut workspace: *mut Con) -> *mut Con {
    if !config.workspace_auto_back_and_forth {
        return workspace;
    }
    let mut current: *const Con = con_get_workspace(focused);
    if current == workspace as *const Con {
        let mut baf: *mut Con = workspace_back_and_forth_get();
        if !baf.is_null() {
            debuglog(
                b"%s:%s:%d - Substituting workspace with back_and_forth, as it is focused.\n\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"../src/commands.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"maybe_auto_back_and_forth_workspace\0" as *const u8 as *const ::core::ffi::c_char,
                112 as ::core::ffi::c_int,
            );
            return baf;
        }
    }
    return workspace;
}
#[no_mangle]
pub unsafe extern "C" fn cmd_criteria_init(
    mut current_match: *mut Match,
    mut cmd_output: *mut CommandResultIR,
) {
    debuglog(
        b"%s:%s:%d - Initializing criteria, current_match = %p\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/commands.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"cmd_criteria_init\0" as *const u8 as *const ::core::ffi::c_char,
        133 as ::core::ffi::c_int,
        current_match,
    );
    match_free(current_match);
    match_init(current_match);
}
#[no_mangle]
pub unsafe extern "C" fn cmd_criteria_match_windows(
    mut current_match: *mut Match,
    mut cmd_output: *mut CommandResultIR,
) {
    debuglog(
        b"%s:%s:%d - match specification finished, matching...\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/commands.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"cmd_criteria_match_windows\0" as *const u8 as *const ::core::ffi::c_char,
        144 as ::core::ffi::c_int,
    );
    while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
        let mut ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
        if !(*ow).owindows.tqe_next.is_null() {
            (*(*ow).owindows.tqe_next).owindows.tqe_prev = (*ow).owindows.tqe_prev;
        } else {
            (*(*cmd_output).ctx).owindows.tqh_last = (*ow).owindows.tqe_prev;
        }
        *(*ow).owindows.tqe_prev = (*ow).owindows.tqe_next;
        free(ow as *mut ::core::ffi::c_void);
    }
    (*(*cmd_output).ctx).owindows.tqh_first = ::core::ptr::null_mut::<owindow>();
    (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*(*cmd_output).ctx).owindows.tqh_first;
    let mut con: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut current_block_41: u64;
    con = all_cons.tqh_first as *mut Con;
    while !con.is_null() {
        debuglog(
            b"%s:%s:%d - checking if con %p / %s matches\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/commands.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"cmd_criteria_match_windows\0" as *const u8 as *const ::core::ffi::c_char,
            157 as ::core::ffi::c_int,
            con,
            (*con).name,
        );
        let mut accept_match: bool = false_0 != 0;
        if !(*current_match).con_id.is_null() {
            accept_match = true_0 != 0;
            if (*current_match).con_id == con {
                debuglog(
                    b"%s:%s:%d - con_id matched.\n\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    b"../src/commands.c\0" as *const u8 as *const ::core::ffi::c_char,
                    b"cmd_criteria_match_windows\0" as *const u8 as *const ::core::ffi::c_char,
                    167 as ::core::ffi::c_int,
                );
                current_block_41 = 15089075282327824602;
            } else {
                debuglog(
                    b"%s:%s:%d - con_id does not match.\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    b"../src/commands.c\0" as *const u8 as *const ::core::ffi::c_char,
                    b"cmd_criteria_match_windows\0" as *const u8 as *const ::core::ffi::c_char,
                    169 as ::core::ffi::c_int,
                );
                current_block_41 = 3512920355445576850;
            }
        } else {
            current_block_41 = 15089075282327824602;
        }
        match current_block_41 {
            15089075282327824602 => {
                if !(*current_match).mark.is_null() && !(*con).marks_head.tqh_first.is_null() {
                    accept_match = true_0 != 0;
                    let mut matched_by_mark: bool = false_0 != 0;
                    let mut mark: *mut mark_t = ::core::ptr::null_mut::<mark_t>();
                    mark = (*con).marks_head.tqh_first as *mut mark_t;
                    while !mark.is_null() {
                        if !regex_matches((*current_match).mark, (*mark).name) {
                            mark = (*mark).marks.tqe_next as *mut mark_t;
                        } else {
                            debuglog(
                                b"%s:%s:%d - match by mark\n\0" as *const u8
                                    as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                                b"../src/commands.c\0" as *const u8 as *const ::core::ffi::c_char,
                                b"cmd_criteria_match_windows\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                184 as ::core::ffi::c_int,
                            );
                            matched_by_mark = true_0 != 0;
                            break;
                        }
                    }
                    if !matched_by_mark {
                        debuglog(
                            b"%s:%s:%d - mark does not match.\n\0" as *const u8
                                as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b"../src/commands.c\0" as *const u8 as *const ::core::ffi::c_char,
                            b"cmd_criteria_match_windows\0" as *const u8
                                as *const ::core::ffi::c_char,
                            190 as ::core::ffi::c_int,
                        );
                        current_block_41 = 3512920355445576850;
                    } else {
                        current_block_41 = 5494826135382683477;
                    }
                } else {
                    current_block_41 = 5494826135382683477;
                }
                match current_block_41 {
                    3512920355445576850 => {}
                    _ => {
                        if !(*con).window.is_null() {
                            if match_matches_window(current_match, (*con).window as *mut i3Window) {
                                debuglog(
                                    b"%s:%s:%d - matches window!\n\0" as *const u8
                                        as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    b"../src/commands.c\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    b"cmd_criteria_match_windows\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    197 as ::core::ffi::c_int,
                                );
                                accept_match = true_0 != 0;
                                current_block_41 = 8845338526596852646;
                            } else {
                                debuglog(
                                    b"%s:%s:%d - doesn't match\n\0" as *const u8
                                        as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    b"../src/commands.c\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    b"cmd_criteria_match_windows\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    200 as ::core::ffi::c_int,
                                );
                                current_block_41 = 3512920355445576850;
                            }
                        } else {
                            current_block_41 = 8845338526596852646;
                        }
                        match current_block_41 {
                            3512920355445576850 => {}
                            _ => {
                                if accept_match {
                                    debuglog(
                                        b"%s:%s:%d - matching: %p / %s\n\0" as *const u8
                                            as *const ::core::ffi::c_char
                                            as *mut ::core::ffi::c_char,
                                        b"../src/commands.c\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                        b"cmd_criteria_match_windows\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                        206 as ::core::ffi::c_int,
                                        con,
                                        (*con).name,
                                    );
                                    let mut ow_0: *mut owindow =
                                        smalloc(::core::mem::size_of::<owindow>() as size_t)
                                            as *mut owindow;
                                    (*ow_0).con = con;
                                    (*ow_0).owindows.tqe_next = ::core::ptr::null_mut::<owindow>();
                                    (*ow_0).owindows.tqe_prev =
                                        (*(*cmd_output).ctx).owindows.tqh_last;
                                    *(*(*cmd_output).ctx).owindows.tqh_last = ow_0 as *mut owindow;
                                    (*(*cmd_output).ctx).owindows.tqh_last =
                                        &raw mut (*ow_0).owindows.tqe_next;
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        }
        con = (*con).all_cons.tqe_next as *mut Con;
    }
}
#[no_mangle]
pub unsafe extern "C" fn cmd_criteria_add(
    mut current_match: *mut Match,
    mut cmd_output: *mut CommandResultIR,
    mut ctype: *const ::core::ffi::c_char,
    mut cvalue: *const ::core::ffi::c_char,
) {
    match_parse_property(current_match, ctype, cvalue);
}
unsafe extern "C" fn move_matches_to_workspace(
    mut owindows: *const owindows_head,
    mut ws: *mut Con,
) {
    let mut current: *mut owindow = ::core::ptr::null_mut::<owindow>();
    current = (*owindows).tqh_first as *mut owindow;
    while !current.is_null() {
        debuglog(
            b"%s:%s:%d - matching: %p / %s\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b"../src/commands.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"move_matches_to_workspace\0" as *const u8 as *const ::core::ffi::c_char,
            226 as ::core::ffi::c_int,
            (*current).con,
            (*(*current).con).name,
        );
        con_move_to_workspace((*current).con, ws, true_0 != 0, false_0 != 0, false_0 != 0);
        current = (*current).owindows.tqe_next as *mut owindow;
    }
}
#[no_mangle]
pub unsafe extern "C" fn cmd_move_con_to_workspace(
    mut current_match: *mut Match,
    mut cmd_output: *mut CommandResultIR,
    mut which: *const ::core::ffi::c_char,
) {
    debuglog(
        b"%s:%s:%d - which=%s\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/commands.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"cmd_move_con_to_workspace\0" as *const u8 as *const ::core::ffi::c_char,
        264 as ::core::ffi::c_int,
        which,
    );
    if !(*current_match).error.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                b"Invalid match: %s\0" as *const u8 as *const ::core::ffi::c_char,
                (*current_match).error,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"success\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"error\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message as *mut ::core::ffi::c_uchar,
                    strlen(message),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message as *mut ::core::ffi::c_void);
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let mut ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
            if !(*ow).owindows.tqe_next.is_null() {
                (*(*ow).owindows.tqe_next).owindows.tqe_prev = (*ow).owindows.tqe_prev;
            } else {
                (*(*cmd_output).ctx).owindows.tqh_last = (*ow).owindows.tqe_prev;
            }
            *(*ow).owindows.tqe_prev = (*ow).owindows.tqe_next;
            free(ow as *mut ::core::ffi::c_void);
        }
        let mut ow_0: *mut owindow =
            smalloc(::core::mem::size_of::<owindow>() as size_t) as *mut owindow;
        (*ow_0).con = focused;
        (*(*cmd_output).ctx).owindows.tqh_first = ::core::ptr::null_mut::<owindow>();
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*(*cmd_output).ctx).owindows.tqh_first;
        (*ow_0).owindows.tqe_next = ::core::ptr::null_mut::<owindow>();
        (*ow_0).owindows.tqe_prev = (*(*cmd_output).ctx).owindows.tqh_last;
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0 as *mut owindow;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    if (*(*cmd_output).ctx).owindows.tqh_first.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            let mut message_0: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message_0,
                b"Nothing to move: specified criteria don't match any window\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"success\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"error\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message_0 as *mut ::core::ffi::c_uchar,
                    strlen(message_0),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message_0 as *mut ::core::ffi::c_void);
        }
        return;
    } else {
        let mut found: bool = false_0 != 0;
        let mut current: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
        while !current.is_null() {
            let mut next: *mut owindow = (*current).owindows.tqe_next as *mut owindow;
            if (*(*current).con).type_0 as ::core::ffi::c_uint
                == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
                && !con_has_children((*current).con)
            {
                if !(*current).owindows.tqe_next.is_null() {
                    (*(*current).owindows.tqe_next).owindows.tqe_prev =
                        (*current).owindows.tqe_prev;
                } else {
                    (*(*cmd_output).ctx).owindows.tqh_last = (*current).owindows.tqe_prev;
                }
                *(*current).owindows.tqe_prev = (*current).owindows.tqe_next;
            } else {
                found = true_0 != 0;
            }
            current = next;
        }
        if !found {
            if !(*cmd_output).json_gen.is_null() {
                let mut message_1: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                sasprintf(
                    &raw mut message_1,
                    b"Nothing to move: workspace empty\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_map_open((*cmd_output).json_gen);
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_string(
                        (*cmd_output).json_gen,
                        b"success\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_uchar,
                        strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                    );
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_bool((*cmd_output).json_gen, false_0);
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_string(
                        (*cmd_output).json_gen,
                        b"error\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_uchar,
                        strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
                    );
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_string(
                        (*cmd_output).json_gen,
                        message_1 as *mut ::core::ffi::c_uchar,
                        strlen(message_1),
                    );
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_map_close((*cmd_output).json_gen);
                } else {
                };
                free(message_1 as *mut ::core::ffi::c_void);
            }
            return;
        }
    }
    let mut ws: *mut Con = ::core::ptr::null_mut::<Con>();
    if strcmp(which, b"next\0" as *const u8 as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        ws = workspace_next();
    } else if strcmp(which, b"prev\0" as *const u8 as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        ws = workspace_prev();
    } else if strcmp(
        which,
        b"next_on_output\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        ws = workspace_next_on_output();
    } else if strcmp(
        which,
        b"prev_on_output\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        ws = workspace_prev_on_output();
    } else if strcmp(
        which,
        b"current\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        ws = con_get_workspace(focused);
    } else {
        if !(*cmd_output).json_gen.is_null() {
            let mut message_2: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message_2,
                b"BUG: called with which=%s\0" as *const u8 as *const ::core::ffi::c_char,
                which,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"success\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"error\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message_2 as *mut ::core::ffi::c_uchar,
                    strlen(message_2),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message_2 as *mut ::core::ffi::c_void);
        }
        return;
    }
    move_matches_to_workspace(&raw mut (*(*cmd_output).ctx).owindows, ws);
    (*cmd_output).needs_tree_render = true_0 != 0;
    if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                b"success\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_uchar,
                strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn cmd_move_con_to_workspace_back_and_forth(
    mut current_match: *mut Match,
    mut cmd_output: *mut CommandResultIR,
) {
    let mut ws: *mut Con = workspace_back_and_forth_get();
    if ws.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                b"No workspace was previously active.\0" as *const u8 as *const ::core::ffi::c_char,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"success\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"error\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message as *mut ::core::ffi::c_uchar,
                    strlen(message),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message as *mut ::core::ffi::c_void);
        }
        return;
    }
    if !(*current_match).error.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            let mut message_0: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message_0,
                b"Invalid match: %s\0" as *const u8 as *const ::core::ffi::c_char,
                (*current_match).error,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"success\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"error\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message_0 as *mut ::core::ffi::c_uchar,
                    strlen(message_0),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message_0 as *mut ::core::ffi::c_void);
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let mut ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
            if !(*ow).owindows.tqe_next.is_null() {
                (*(*ow).owindows.tqe_next).owindows.tqe_prev = (*ow).owindows.tqe_prev;
            } else {
                (*(*cmd_output).ctx).owindows.tqh_last = (*ow).owindows.tqe_prev;
            }
            *(*ow).owindows.tqe_prev = (*ow).owindows.tqe_next;
            free(ow as *mut ::core::ffi::c_void);
        }
        let mut ow_0: *mut owindow =
            smalloc(::core::mem::size_of::<owindow>() as size_t) as *mut owindow;
        (*ow_0).con = focused;
        (*(*cmd_output).ctx).owindows.tqh_first = ::core::ptr::null_mut::<owindow>();
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*(*cmd_output).ctx).owindows.tqh_first;
        (*ow_0).owindows.tqe_next = ::core::ptr::null_mut::<owindow>();
        (*ow_0).owindows.tqe_prev = (*(*cmd_output).ctx).owindows.tqh_last;
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0 as *mut owindow;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    move_matches_to_workspace(&raw mut (*(*cmd_output).ctx).owindows, ws);
    (*cmd_output).needs_tree_render = true_0 != 0;
    if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                b"success\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_uchar,
                strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn cmd_move_con_to_workspace_name(
    mut current_match: *mut Match,
    mut cmd_output: *mut CommandResultIR,
    mut name: *const ::core::ffi::c_char,
    mut no_auto_back_and_forth: *const ::core::ffi::c_char,
) {
    if strncasecmp(
        name,
        b"__\0" as *const u8 as *const ::core::ffi::c_char,
        strlen(b"__\0" as *const u8 as *const ::core::ffi::c_char),
    ) == 0 as ::core::ffi::c_int
    {
        if !(*cmd_output).json_gen.is_null() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                b"You cannot move containers to i3-internal workspaces (\"%s\").\0" as *const u8
                    as *const ::core::ffi::c_char,
                name,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"success\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"error\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message as *mut ::core::ffi::c_uchar,
                    strlen(message),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message as *mut ::core::ffi::c_void);
        }
        return;
    }
    if !(*current_match).error.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            let mut message_0: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message_0,
                b"Invalid match: %s\0" as *const u8 as *const ::core::ffi::c_char,
                (*current_match).error,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"success\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"error\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message_0 as *mut ::core::ffi::c_uchar,
                    strlen(message_0),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message_0 as *mut ::core::ffi::c_void);
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let mut ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
            if !(*ow).owindows.tqe_next.is_null() {
                (*(*ow).owindows.tqe_next).owindows.tqe_prev = (*ow).owindows.tqe_prev;
            } else {
                (*(*cmd_output).ctx).owindows.tqh_last = (*ow).owindows.tqe_prev;
            }
            *(*ow).owindows.tqe_prev = (*ow).owindows.tqe_next;
            free(ow as *mut ::core::ffi::c_void);
        }
        let mut ow_0: *mut owindow =
            smalloc(::core::mem::size_of::<owindow>() as size_t) as *mut owindow;
        (*ow_0).con = focused;
        (*(*cmd_output).ctx).owindows.tqh_first = ::core::ptr::null_mut::<owindow>();
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*(*cmd_output).ctx).owindows.tqh_first;
        (*ow_0).owindows.tqe_next = ::core::ptr::null_mut::<owindow>();
        (*ow_0).owindows.tqe_prev = (*(*cmd_output).ctx).owindows.tqh_last;
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0 as *mut owindow;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    if (*(*cmd_output).ctx).owindows.tqh_first.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            let mut message_1: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message_1,
                b"Nothing to move: specified criteria don't match any window\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"success\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"error\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message_1 as *mut ::core::ffi::c_uchar,
                    strlen(message_1),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message_1 as *mut ::core::ffi::c_void);
        }
        return;
    } else {
        let mut found: bool = false_0 != 0;
        let mut current: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
        while !current.is_null() {
            let mut next: *mut owindow = (*current).owindows.tqe_next as *mut owindow;
            if (*(*current).con).type_0 as ::core::ffi::c_uint
                == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
                && !con_has_children((*current).con)
            {
                if !(*current).owindows.tqe_next.is_null() {
                    (*(*current).owindows.tqe_next).owindows.tqe_prev =
                        (*current).owindows.tqe_prev;
                } else {
                    (*(*cmd_output).ctx).owindows.tqh_last = (*current).owindows.tqe_prev;
                }
                *(*current).owindows.tqe_prev = (*current).owindows.tqe_next;
            } else {
                found = true_0 != 0;
            }
            current = next;
        }
        if !found {
            if !(*cmd_output).json_gen.is_null() {
                let mut message_2: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                sasprintf(
                    &raw mut message_2,
                    b"Nothing to move: workspace empty\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_map_open((*cmd_output).json_gen);
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_string(
                        (*cmd_output).json_gen,
                        b"success\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_uchar,
                        strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                    );
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_bool((*cmd_output).json_gen, false_0);
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_string(
                        (*cmd_output).json_gen,
                        b"error\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_uchar,
                        strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
                    );
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_string(
                        (*cmd_output).json_gen,
                        message_2 as *mut ::core::ffi::c_uchar,
                        strlen(message_2),
                    );
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_map_close((*cmd_output).json_gen);
                } else {
                };
                free(message_2 as *mut ::core::ffi::c_void);
            }
            return;
        }
    }
    verboselog(
        b"should move window to workspace %s\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        name,
    );
    let mut ws: *mut Con = workspace_get(name);
    if no_auto_back_and_forth.is_null() {
        ws = maybe_auto_back_and_forth_workspace(ws);
    }
    move_matches_to_workspace(&raw mut (*(*cmd_output).ctx).owindows, ws);
    (*cmd_output).needs_tree_render = true_0 != 0;
    if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                b"success\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_uchar,
                strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn cmd_move_con_to_workspace_number(
    mut current_match: *mut Match,
    mut cmd_output: *mut CommandResultIR,
    mut which: *const ::core::ffi::c_char,
    mut no_auto_back_and_forth: *const ::core::ffi::c_char,
) {
    if !(*current_match).error.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                b"Invalid match: %s\0" as *const u8 as *const ::core::ffi::c_char,
                (*current_match).error,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"success\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"error\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message as *mut ::core::ffi::c_uchar,
                    strlen(message),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message as *mut ::core::ffi::c_void);
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let mut ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
            if !(*ow).owindows.tqe_next.is_null() {
                (*(*ow).owindows.tqe_next).owindows.tqe_prev = (*ow).owindows.tqe_prev;
            } else {
                (*(*cmd_output).ctx).owindows.tqh_last = (*ow).owindows.tqe_prev;
            }
            *(*ow).owindows.tqe_prev = (*ow).owindows.tqe_next;
            free(ow as *mut ::core::ffi::c_void);
        }
        let mut ow_0: *mut owindow =
            smalloc(::core::mem::size_of::<owindow>() as size_t) as *mut owindow;
        (*ow_0).con = focused;
        (*(*cmd_output).ctx).owindows.tqh_first = ::core::ptr::null_mut::<owindow>();
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*(*cmd_output).ctx).owindows.tqh_first;
        (*ow_0).owindows.tqe_next = ::core::ptr::null_mut::<owindow>();
        (*ow_0).owindows.tqe_prev = (*(*cmd_output).ctx).owindows.tqh_last;
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0 as *mut owindow;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    if (*(*cmd_output).ctx).owindows.tqh_first.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            let mut message_0: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message_0,
                b"Nothing to move: specified criteria don't match any window\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"success\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"error\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message_0 as *mut ::core::ffi::c_uchar,
                    strlen(message_0),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message_0 as *mut ::core::ffi::c_void);
        }
        return;
    } else {
        let mut found: bool = false_0 != 0;
        let mut current: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
        while !current.is_null() {
            let mut next: *mut owindow = (*current).owindows.tqe_next as *mut owindow;
            if (*(*current).con).type_0 as ::core::ffi::c_uint
                == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
                && !con_has_children((*current).con)
            {
                if !(*current).owindows.tqe_next.is_null() {
                    (*(*current).owindows.tqe_next).owindows.tqe_prev =
                        (*current).owindows.tqe_prev;
                } else {
                    (*(*cmd_output).ctx).owindows.tqh_last = (*current).owindows.tqe_prev;
                }
                *(*current).owindows.tqe_prev = (*current).owindows.tqe_next;
            } else {
                found = true_0 != 0;
            }
            current = next;
        }
        if !found {
            if !(*cmd_output).json_gen.is_null() {
                let mut message_1: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                sasprintf(
                    &raw mut message_1,
                    b"Nothing to move: workspace empty\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_map_open((*cmd_output).json_gen);
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_string(
                        (*cmd_output).json_gen,
                        b"success\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_uchar,
                        strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                    );
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_bool((*cmd_output).json_gen, false_0);
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_string(
                        (*cmd_output).json_gen,
                        b"error\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_uchar,
                        strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
                    );
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_string(
                        (*cmd_output).json_gen,
                        message_1 as *mut ::core::ffi::c_uchar,
                        strlen(message_1),
                    );
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_map_close((*cmd_output).json_gen);
                } else {
                };
                free(message_1 as *mut ::core::ffi::c_void);
            }
            return;
        }
    }
    verboselog(
        b"should move window to workspace %s\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        which,
    );
    let parsed_num: ::core::ffi::c_long = ws_name_to_number(which) as ::core::ffi::c_long;
    if parsed_num == -(1 as ::core::ffi::c_int) as ::core::ffi::c_long {
        verboselog(
            b"Could not parse initial part of \"%s\" as a number.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            which,
        );
        if !(*cmd_output).json_gen.is_null() {
            let mut message_2: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message_2,
                b"Could not parse number \"%s\"\0" as *const u8 as *const ::core::ffi::c_char,
                which,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"success\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"error\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message_2 as *mut ::core::ffi::c_uchar,
                    strlen(message_2),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message_2 as *mut ::core::ffi::c_void);
        }
        return;
    }
    let mut ws: *mut Con = get_existing_workspace_by_num(parsed_num as ::core::ffi::c_int);
    if ws.is_null() {
        ws = workspace_get(which);
    }
    if no_auto_back_and_forth.is_null() {
        ws = maybe_auto_back_and_forth_workspace(ws);
    }
    move_matches_to_workspace(&raw mut (*(*cmd_output).ctx).owindows, ws);
    (*cmd_output).needs_tree_render = true_0 != 0;
    if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                b"success\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_uchar,
                strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
    }
}
unsafe extern "C" fn parse_direction(mut str: *const ::core::ffi::c_char) -> direction_t {
    if strcmp(str, b"left\0" as *const u8 as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int
    {
        return D_LEFT;
    }
    if strcmp(str, b"right\0" as *const u8 as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int
    {
        return D_RIGHT;
    }
    if strcmp(str, b"up\0" as *const u8 as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int {
        return D_UP;
    }
    if strcmp(str, b"down\0" as *const u8 as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int
    {
        return D_DOWN;
    }
    errorlog(
        b"ERROR: Invalid direction. This is a parser bug.\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    if 0 as ::core::ffi::c_int != 0 {
    } else {
        __assert_fail(
            b"false\0" as *const u8 as *const ::core::ffi::c_char,
            b"../src/commands.c\0" as *const u8 as *const ::core::ffi::c_char,
            389 as ::core::ffi::c_uint,
            b"direction_t parse_direction(const char *)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    };
    panic!("Reached end of non-void function without returning");
}
unsafe extern "C" fn cmd_resize_floating(
    mut current_match: *mut Match,
    mut cmd_output: *mut CommandResultIR,
    mut direction_str: *const ::core::ffi::c_char,
    mut floating_con: *mut Con,
    mut px: ::core::ffi::c_int,
) {
    let old_rect: Rect = (*floating_con).rect as Rect;
    let mut focused_con: *const Con = con_descend_focused(floating_con);
    let mut direction: direction_t = D_LEFT;
    if strcmp(
        direction_str,
        b"height\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        direction = D_DOWN;
    } else if strcmp(
        direction_str,
        b"width\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        direction = D_RIGHT;
    } else {
        direction = parse_direction(direction_str);
    }
    let orientation: orientation_t = orientation_from_direction(direction) as orientation_t;
    let mut window: *const i3Window = (*focused_con).window;
    if !window.is_null() {
        if orientation as ::core::ffi::c_uint == VERT as ::core::ffi::c_int as ::core::ffi::c_uint {
            if px < 0 as ::core::ffi::c_int {
                px = if -px < (*window).height_increment {
                    -(*window).height_increment
                } else {
                    px
                };
            } else {
                px = if px < (*window).height_increment {
                    (*window).height_increment
                } else {
                    px
                };
            }
        } else if px < 0 as ::core::ffi::c_int {
            px = if -px < (*window).width_increment {
                -(*window).width_increment
            } else {
                px
            };
        } else {
            px = if px < (*window).width_increment {
                (*window).width_increment
            } else {
                px
            };
        }
    }
    if orientation as ::core::ffi::c_uint == VERT as ::core::ffi::c_int as ::core::ffi::c_uint {
        (*floating_con).rect.height = (*floating_con).rect.height.wrapping_add(px as uint32_t);
    } else {
        (*floating_con).rect.width = (*floating_con).rect.width.wrapping_add(px as uint32_t);
    }
    floating_check_size(
        floating_con,
        orientation as ::core::ffi::c_uint == VERT as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    if rect_equals(old_rect, (*floating_con).rect as Rect) {
        return;
    }
    if direction as ::core::ffi::c_uint == D_UP as ::core::ffi::c_int as ::core::ffi::c_uint {
        (*floating_con).rect.y = (*floating_con)
            .rect
            .y
            .wrapping_sub((*floating_con).rect.height.wrapping_sub(old_rect.height));
    } else if direction as ::core::ffi::c_uint
        == D_LEFT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*floating_con).rect.x = (*floating_con)
            .rect
            .x
            .wrapping_sub((*floating_con).rect.width.wrapping_sub(old_rect.width));
    }
    if (*floating_con).scratchpad_state as ::core::ffi::c_uint
        == SCRATCHPAD_FRESH as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*floating_con).scratchpad_state = SCRATCHPAD_CHANGED;
    }
}
unsafe extern "C" fn cmd_resize_tiling_direction(
    mut current_match: *mut Match,
    mut cmd_output: *mut CommandResultIR,
    mut current: *mut Con,
    mut direction: *const ::core::ffi::c_char,
    mut px: ::core::ffi::c_int,
    ppt: ::core::ffi::c_int,
) -> bool {
    let mut second: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut first: *mut Con = current;
    let search_direction: direction_t = parse_direction(direction) as direction_t;
    let res: bool = resize_find_tiling_participants(
        &raw mut first,
        &raw mut second,
        search_direction,
        false_0 != 0,
    ) as bool;
    if !res {
        if !(*cmd_output).json_gen.is_null() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                b"No second container found in this direction.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"success\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"error\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message as *mut ::core::ffi::c_uchar,
                    strlen(message),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message as *mut ::core::ffi::c_void);
        }
        return false_0 != 0;
    }
    if ppt != 0 {
        px = 0 as ::core::ffi::c_int;
    }
    return resize_neighboring_cons(first, second, px, ppt);
}
unsafe extern "C" fn cmd_resize_tiling_width_height(
    mut current_match: *mut Match,
    mut cmd_output: *mut CommandResultIR,
    mut current: *mut Con,
    mut direction: *const ::core::ffi::c_char,
    px: ::core::ffi::c_int,
    mut ppt: ::core::ffi::c_double,
) -> bool {
    verboselog(
        b"width/height resize\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    );
    let mut dummy: *mut Con = ::core::ptr::null_mut::<Con>();
    let search_direction: direction_t = (if strcmp(
        direction,
        b"width\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        D_LEFT as ::core::ffi::c_int
    } else {
        D_DOWN as ::core::ffi::c_int
    }) as direction_t;
    let search_result: bool = resize_find_tiling_participants(
        &raw mut current,
        &raw mut dummy,
        search_direction,
        true_0 != 0,
    ) as bool;
    if search_result as ::core::ffi::c_int == false_0 {
        if !(*cmd_output).json_gen.is_null() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                b"Failed to find appropriate tiling containers for resize operation\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"success\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"error\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message as *mut ::core::ffi::c_uchar,
                    strlen(message),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message as *mut ::core::ffi::c_void);
        }
        return false_0 != 0;
    }
    let children: ::core::ffi::c_int =
        con_num_children((*current).parent as *mut Con) as ::core::ffi::c_int;
    verboselog(
        b"ins. %d children\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        children,
    );
    let percentage: ::core::ffi::c_double = 1.0f64 / children as ::core::ffi::c_double;
    verboselog(
        b"default percentage = %f\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        percentage,
    );
    let mut child: *mut Con = ::core::ptr::null_mut::<Con>();
    child = (*(*current).parent).nodes_head.tqh_first as *mut Con;
    while !child.is_null() {
        verboselog(
            b"child->percent = %f (child %p)\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            (*child).percent,
            child,
        );
        if (*child).percent == 0.0f64 {
            (*child).percent = percentage;
        }
        child = (*child).nodes.tqe_next as *mut Con;
    }
    let mut new_current_percent: ::core::ffi::c_double = 0.;
    if ppt != 0.0f64 {
        new_current_percent = (*current).percent + ppt;
    } else {
        ppt = px as ::core::ffi::c_double
            / con_rect_size_in_orientation((*current).parent as *mut Con) as ::core::ffi::c_double;
        new_current_percent = (*current).percent + ppt;
    }
    let subtract_percent: ::core::ffi::c_double =
        ppt / (children - 1 as ::core::ffi::c_int) as ::core::ffi::c_double;
    if ppt < 0.0f64 && new_current_percent < percent_for_1px(current) {
        if !(*cmd_output).json_gen.is_null() {
            let mut message_0: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message_0,
                b"Not resizing, container would end with less than 1px\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"success\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"error\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message_0 as *mut ::core::ffi::c_uchar,
                    strlen(message_0),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message_0 as *mut ::core::ffi::c_void);
        }
        return false_0 != 0;
    }
    verboselog(
        b"new_current_percent = %f\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        new_current_percent,
    );
    verboselog(
        b"subtract_percent = %f\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        subtract_percent,
    );
    if subtract_percent >= 0.0f64 {
        child = (*(*current).parent).nodes_head.tqh_first as *mut Con;
        while !child.is_null() {
            if !(child == current) {
                if (*child).percent - subtract_percent < percent_for_1px(child) {
                    if !(*cmd_output).json_gen.is_null() {
                        let mut message_1: *mut ::core::ffi::c_char =
                            ::core::ptr::null_mut::<::core::ffi::c_char>();
                        sasprintf(
                            &raw mut message_1,
                            b"Not resizing, already at minimum size (child %p would end up with a size of %.f\0"
                                as *const u8 as *const ::core::ffi::c_char,
                            child,
                            (*child).percent - subtract_percent,
                        );
                        if !(*cmd_output).json_gen.is_null() {
                            yajl_gen_map_open((*cmd_output).json_gen);
                        } else {
                        };
                        if !(*cmd_output).json_gen.is_null() {
                            yajl_gen_string(
                                (*cmd_output).json_gen,
                                b"success\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_uchar,
                                strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                            );
                        } else {
                        };
                        if !(*cmd_output).json_gen.is_null() {
                            yajl_gen_bool((*cmd_output).json_gen, false_0);
                        } else {
                        };
                        if !(*cmd_output).json_gen.is_null() {
                            yajl_gen_string(
                                (*cmd_output).json_gen,
                                b"error\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_uchar,
                                strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
                            );
                        } else {
                        };
                        if !(*cmd_output).json_gen.is_null() {
                            yajl_gen_string(
                                (*cmd_output).json_gen,
                                message_1 as *mut ::core::ffi::c_uchar,
                                strlen(message_1),
                            );
                        } else {
                        };
                        if !(*cmd_output).json_gen.is_null() {
                            yajl_gen_map_close((*cmd_output).json_gen);
                        } else {
                        };
                        free(message_1 as *mut ::core::ffi::c_void);
                    }
                    return false_0 != 0;
                }
            }
            child = (*child).nodes.tqe_next as *mut Con;
        }
    }
    (*current).percent = new_current_percent;
    verboselog(
        b"current->percent after = %f\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        (*current).percent,
    );
    child = (*(*current).parent).nodes_head.tqh_first as *mut Con;
    while !child.is_null() {
        if !(child == current) {
            (*child).percent -= subtract_percent;
            verboselog(
                b"child->percent after (%p) = %f\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                child,
                (*child).percent,
            );
        }
        child = (*child).nodes.tqe_next as *mut Con;
    }
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn cmd_resize(
    mut current_match: *mut Match,
    mut cmd_output: *mut CommandResultIR,
    mut way: *const ::core::ffi::c_char,
    mut direction: *const ::core::ffi::c_char,
    mut resize_px: ::core::ffi::c_long,
    mut resize_ppt: ::core::ffi::c_long,
) {
    debuglog(
        b"%s:%s:%d - resizing in way %s, direction %s, px %ld or ppt %ld\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/commands.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"cmd_resize\0" as *const u8 as *const ::core::ffi::c_char,
        545 as ::core::ffi::c_int,
        way,
        direction,
        resize_px,
        resize_ppt,
    );
    if strcmp(way, b"shrink\0" as *const u8 as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        resize_px *= -(1 as ::core::ffi::c_int) as ::core::ffi::c_long;
        resize_ppt *= -(1 as ::core::ffi::c_int) as ::core::ffi::c_long;
    }
    if !(*current_match).error.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                b"Invalid match: %s\0" as *const u8 as *const ::core::ffi::c_char,
                (*current_match).error,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"success\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"error\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message as *mut ::core::ffi::c_uchar,
                    strlen(message),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message as *mut ::core::ffi::c_void);
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let mut ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
            if !(*ow).owindows.tqe_next.is_null() {
                (*(*ow).owindows.tqe_next).owindows.tqe_prev = (*ow).owindows.tqe_prev;
            } else {
                (*(*cmd_output).ctx).owindows.tqh_last = (*ow).owindows.tqe_prev;
            }
            *(*ow).owindows.tqe_prev = (*ow).owindows.tqe_next;
            free(ow as *mut ::core::ffi::c_void);
        }
        let mut ow_0: *mut owindow =
            smalloc(::core::mem::size_of::<owindow>() as size_t) as *mut owindow;
        (*ow_0).con = focused;
        (*(*cmd_output).ctx).owindows.tqh_first = ::core::ptr::null_mut::<owindow>();
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*(*cmd_output).ctx).owindows.tqh_first;
        (*ow_0).owindows.tqe_next = ::core::ptr::null_mut::<owindow>();
        (*ow_0).owindows.tqe_prev = (*(*cmd_output).ctx).owindows.tqh_last;
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0 as *mut owindow;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    let mut current: *mut owindow = ::core::ptr::null_mut::<owindow>();
    current = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
    while !current.is_null() {
        if !(*(*current).con).window.is_null()
            && (*(*(*current).con).window).dock as ::core::ffi::c_uint != 0
        {
            debuglog(
                b"%s:%s:%d - This is a dock window. Not resizing (con = %p)\n)\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"../src/commands.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"cmd_resize\0" as *const u8 as *const ::core::ffi::c_char,
                557 as ::core::ffi::c_int,
                (*current).con,
            );
        } else {
            let mut floating_con: *mut Con = ::core::ptr::null_mut::<Con>();
            floating_con = con_inside_floating((*current).con);
            if !floating_con.is_null() {
                cmd_resize_floating(
                    current_match,
                    cmd_output,
                    direction,
                    floating_con,
                    resize_px as ::core::ffi::c_int,
                );
            } else if strcmp(
                direction,
                b"width\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
                || strcmp(
                    direction,
                    b"height\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
            {
                let ppt: ::core::ffi::c_double = resize_ppt as ::core::ffi::c_double / 100.0f64;
                if !cmd_resize_tiling_width_height(
                    current_match,
                    cmd_output,
                    (*current).con,
                    direction,
                    resize_px as ::core::ffi::c_int,
                    ppt,
                ) {
                    if !(*cmd_output).json_gen.is_null() {
                        let mut message_0: *mut ::core::ffi::c_char =
                            ::core::ptr::null_mut::<::core::ffi::c_char>();
                        sasprintf(
                            &raw mut message_0,
                            b"Cannot resize.\0" as *const u8 as *const ::core::ffi::c_char,
                        );
                        if !(*cmd_output).json_gen.is_null() {
                            yajl_gen_map_open((*cmd_output).json_gen);
                        } else {
                        };
                        if !(*cmd_output).json_gen.is_null() {
                            yajl_gen_string(
                                (*cmd_output).json_gen,
                                b"success\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_uchar,
                                strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                            );
                        } else {
                        };
                        if !(*cmd_output).json_gen.is_null() {
                            yajl_gen_bool((*cmd_output).json_gen, false_0);
                        } else {
                        };
                        if !(*cmd_output).json_gen.is_null() {
                            yajl_gen_string(
                                (*cmd_output).json_gen,
                                b"error\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_uchar,
                                strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
                            );
                        } else {
                        };
                        if !(*cmd_output).json_gen.is_null() {
                            yajl_gen_string(
                                (*cmd_output).json_gen,
                                message_0 as *mut ::core::ffi::c_uchar,
                                strlen(message_0),
                            );
                        } else {
                        };
                        if !(*cmd_output).json_gen.is_null() {
                            yajl_gen_map_close((*cmd_output).json_gen);
                        } else {
                        };
                        free(message_0 as *mut ::core::ffi::c_void);
                    }
                    return;
                }
            } else if !cmd_resize_tiling_direction(
                current_match,
                cmd_output,
                (*current).con,
                direction,
                resize_px as ::core::ffi::c_int,
                resize_ppt as ::core::ffi::c_int,
            ) {
                if !(*cmd_output).json_gen.is_null() {
                    let mut message_1: *mut ::core::ffi::c_char =
                        ::core::ptr::null_mut::<::core::ffi::c_char>();
                    sasprintf(
                        &raw mut message_1,
                        b"Cannot resize.\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    if !(*cmd_output).json_gen.is_null() {
                        yajl_gen_map_open((*cmd_output).json_gen);
                    } else {
                    };
                    if !(*cmd_output).json_gen.is_null() {
                        yajl_gen_string(
                            (*cmd_output).json_gen,
                            b"success\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_uchar,
                            strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                        );
                    } else {
                    };
                    if !(*cmd_output).json_gen.is_null() {
                        yajl_gen_bool((*cmd_output).json_gen, false_0);
                    } else {
                    };
                    if !(*cmd_output).json_gen.is_null() {
                        yajl_gen_string(
                            (*cmd_output).json_gen,
                            b"error\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_uchar,
                            strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
                        );
                    } else {
                    };
                    if !(*cmd_output).json_gen.is_null() {
                        yajl_gen_string(
                            (*cmd_output).json_gen,
                            message_1 as *mut ::core::ffi::c_uchar,
                            strlen(message_1),
                        );
                    } else {
                    };
                    if !(*cmd_output).json_gen.is_null() {
                        yajl_gen_map_close((*cmd_output).json_gen);
                    } else {
                    };
                    free(message_1 as *mut ::core::ffi::c_void);
                }
                return;
            }
        }
        current = (*current).owindows.tqe_next as *mut owindow;
    }
    (*cmd_output).needs_tree_render = true_0 != 0;
    if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                b"success\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_uchar,
                strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
    }
}
unsafe extern "C" fn resize_set_tiling(
    mut current_match: *mut Match,
    mut cmd_output: *mut CommandResultIR,
    mut target: *mut Con,
    resize_orientation: orientation_t,
    is_ppt: bool,
    target_size: ::core::ffi::c_long,
) -> bool {
    let mut search_direction: direction_t = D_LEFT;
    let mut mode: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if resize_orientation as ::core::ffi::c_uint
        == HORIZ as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        search_direction = D_LEFT;
        mode = b"width\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
    } else {
        search_direction = D_DOWN;
        mode = b"height\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
    }
    let mut dummy: *mut Con = ::core::ptr::null_mut::<Con>();
    resize_find_tiling_participants(
        &raw mut target,
        &raw mut dummy,
        search_direction,
        true_0 != 0,
    );
    let mut ppt: ::core::ffi::c_double = 0.0f64;
    let mut px: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if is_ppt {
        ppt = target_size as ::core::ffi::c_double / 100.0f64 - (*target).percent;
    } else {
        px = (target_size
            - (if resize_orientation as ::core::ffi::c_uint
                == HORIZ as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                (*target).rect.width
            } else {
                (*target).rect.height
            }) as ::core::ffi::c_long) as ::core::ffi::c_int;
    }
    return cmd_resize_tiling_width_height(current_match, cmd_output, target, mode, px, ppt);
}
#[no_mangle]
pub unsafe extern "C" fn cmd_resize_set(
    mut current_match: *mut Match,
    mut cmd_output: *mut CommandResultIR,
    mut cwidth: ::core::ffi::c_long,
    mut mode_width: *const ::core::ffi::c_char,
    mut cheight: ::core::ffi::c_long,
    mut mode_height: *const ::core::ffi::c_char,
) {
    debuglog(
        b"%s:%s:%d - resizing to %ld %s x %ld %s\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/commands.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"cmd_resize_set\0" as *const u8 as *const ::core::ffi::c_char,
        624 as ::core::ffi::c_int,
        cwidth,
        mode_width,
        cheight,
        mode_height,
    );
    if cwidth < 0 as ::core::ffi::c_long || cheight < 0 as ::core::ffi::c_long {
        if !(*cmd_output).json_gen.is_null() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                b"Dimensions cannot be negative.\0" as *const u8 as *const ::core::ffi::c_char,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"success\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"error\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message as *mut ::core::ffi::c_uchar,
                    strlen(message),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message as *mut ::core::ffi::c_void);
        }
        return;
    }
    if !(*current_match).error.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            let mut message_0: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message_0,
                b"Invalid match: %s\0" as *const u8 as *const ::core::ffi::c_char,
                (*current_match).error,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"success\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"error\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message_0 as *mut ::core::ffi::c_uchar,
                    strlen(message_0),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message_0 as *mut ::core::ffi::c_void);
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let mut ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
            if !(*ow).owindows.tqe_next.is_null() {
                (*(*ow).owindows.tqe_next).owindows.tqe_prev = (*ow).owindows.tqe_prev;
            } else {
                (*(*cmd_output).ctx).owindows.tqh_last = (*ow).owindows.tqe_prev;
            }
            *(*ow).owindows.tqe_prev = (*ow).owindows.tqe_next;
            free(ow as *mut ::core::ffi::c_void);
        }
        let mut ow_0: *mut owindow =
            smalloc(::core::mem::size_of::<owindow>() as size_t) as *mut owindow;
        (*ow_0).con = focused;
        (*(*cmd_output).ctx).owindows.tqh_first = ::core::ptr::null_mut::<owindow>();
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*(*cmd_output).ctx).owindows.tqh_first;
        (*ow_0).owindows.tqe_next = ::core::ptr::null_mut::<owindow>();
        (*ow_0).owindows.tqe_prev = (*(*cmd_output).ctx).owindows.tqh_last;
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0 as *mut owindow;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    let mut current: *mut owindow = ::core::ptr::null_mut::<owindow>();
    let mut success: bool = true_0 != 0;
    current = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
    while !current.is_null() {
        let mut floating_con: *mut Con = ::core::ptr::null_mut::<Con>();
        floating_con = con_inside_floating((*current).con);
        if !floating_con.is_null() {
            let mut output: *const Con = con_get_output(floating_con);
            if cwidth == 0 as ::core::ffi::c_long {
                cwidth = (*floating_con).rect.width as ::core::ffi::c_long;
            } else if !mode_width.is_null()
                && strcmp(
                    mode_width,
                    b"ppt\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
            {
                cwidth = ((*output).rect.width as ::core::ffi::c_double
                    * (cwidth as ::core::ffi::c_double / 100.0f64))
                    as ::core::ffi::c_long;
            }
            if cheight == 0 as ::core::ffi::c_long {
                cheight = (*floating_con).rect.height as ::core::ffi::c_long;
            } else if !mode_height.is_null()
                && strcmp(
                    mode_height,
                    b"ppt\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
            {
                cheight = ((*output).rect.height as ::core::ffi::c_double
                    * (cheight as ::core::ffi::c_double / 100.0f64))
                    as ::core::ffi::c_long;
            }
            floating_resize(floating_con, cwidth as uint32_t, cheight as uint32_t);
        } else if !(*(*current).con).window.is_null()
            && (*(*(*current).con).window).dock as ::core::ffi::c_uint != 0
        {
            debuglog(
                b"%s:%s:%d - This is a dock window. Not resizing (con = %p)\n)\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"../src/commands.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"cmd_resize_set\0" as *const u8 as *const ::core::ffi::c_char,
                651 as ::core::ffi::c_int,
                (*current).con,
            );
        } else {
            if cwidth > 0 as ::core::ffi::c_long {
                let is_ppt: bool = !mode_width.is_null()
                    && strcmp(
                        mode_width,
                        b"ppt\0" as *const u8 as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int;
                success = (success as ::core::ffi::c_int
                    & resize_set_tiling(
                        current_match,
                        cmd_output,
                        (*current).con,
                        HORIZ,
                        is_ppt,
                        cwidth,
                    ) as ::core::ffi::c_int)
                    != 0;
            }
            if cheight > 0 as ::core::ffi::c_long {
                let is_ppt_0: bool = !mode_height.is_null()
                    && strcmp(
                        mode_height,
                        b"ppt\0" as *const u8 as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int;
                success = (success as ::core::ffi::c_int
                    & resize_set_tiling(
                        current_match,
                        cmd_output,
                        (*current).con,
                        VERT,
                        is_ppt_0,
                        cheight,
                    ) as ::core::ffi::c_int)
                    != 0;
            }
        }
        current = (*current).owindows.tqe_next as *mut owindow;
    }
    (*cmd_output).needs_tree_render = true_0 != 0;
    if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                b"success\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_uchar,
                strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, success as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
    }
}
unsafe extern "C" fn border_width_from_style(
    border_style: border_style_t,
    border_width: ::core::ffi::c_long,
    mut con: *mut Con,
) -> ::core::ffi::c_int {
    if border_style as ::core::ffi::c_uint == BS_NONE as ::core::ffi::c_int as ::core::ffi::c_uint {
        return 0 as ::core::ffi::c_int;
    }
    if border_width >= 0 as ::core::ffi::c_long {
        return logical_px(border_width as ::core::ffi::c_int);
    }
    let is_floating: bool = !con_inside_floating(con).is_null();
    if is_floating as ::core::ffi::c_int != 0
        && border_style as ::core::ffi::c_uint
            == config.default_floating_border as ::core::ffi::c_uint
    {
        return config.default_floating_border_width;
    } else if !is_floating
        && border_style as ::core::ffi::c_uint == config.default_border as ::core::ffi::c_uint
    {
        return config.default_border_width;
    } else {
        return logical_px(
            if border_style as ::core::ffi::c_uint
                == BS_NORMAL as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                2 as ::core::ffi::c_int
            } else {
                1 as ::core::ffi::c_int
            },
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn cmd_border(
    mut current_match: *mut Match,
    mut cmd_output: *mut CommandResultIR,
    mut border_style_str: *const ::core::ffi::c_char,
    border_width: ::core::ffi::c_long,
) {
    debuglog(
        b"%s:%s:%d - border style should be changed to %s with border width %ld\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/commands.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"cmd_border\0" as *const u8 as *const ::core::ffi::c_char,
        697 as ::core::ffi::c_int,
        border_style_str,
        border_width,
    );
    let mut current: *mut owindow = ::core::ptr::null_mut::<owindow>();
    if !(*current_match).error.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                b"Invalid match: %s\0" as *const u8 as *const ::core::ffi::c_char,
                (*current_match).error,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"success\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"error\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message as *mut ::core::ffi::c_uchar,
                    strlen(message),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message as *mut ::core::ffi::c_void);
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let mut ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
            if !(*ow).owindows.tqe_next.is_null() {
                (*(*ow).owindows.tqe_next).owindows.tqe_prev = (*ow).owindows.tqe_prev;
            } else {
                (*(*cmd_output).ctx).owindows.tqh_last = (*ow).owindows.tqe_prev;
            }
            *(*ow).owindows.tqe_prev = (*ow).owindows.tqe_next;
            free(ow as *mut ::core::ffi::c_void);
        }
        let mut ow_0: *mut owindow =
            smalloc(::core::mem::size_of::<owindow>() as size_t) as *mut owindow;
        (*ow_0).con = focused;
        (*(*cmd_output).ctx).owindows.tqh_first = ::core::ptr::null_mut::<owindow>();
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*(*cmd_output).ctx).owindows.tqh_first;
        (*ow_0).owindows.tqe_next = ::core::ptr::null_mut::<owindow>();
        (*ow_0).owindows.tqe_prev = (*(*cmd_output).ctx).owindows.tqh_last;
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0 as *mut owindow;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    current = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
    while !current.is_null() {
        debuglog(
            b"%s:%s:%d - matching: %p / %s\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b"../src/commands.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"cmd_border\0" as *const u8 as *const ::core::ffi::c_char,
            703 as ::core::ffi::c_int,
            (*current).con,
            (*(*current).con).name,
        );
        let mut border_style: border_style_t = BS_NONE;
        if strcmp(
            border_style_str,
            b"toggle\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            border_style = ((*(*current).con).border_style as ::core::ffi::c_uint)
                .wrapping_add(1 as ::core::ffi::c_uint)
                .wrapping_rem(3 as ::core::ffi::c_uint)
                as border_style_t;
        } else if strcmp(
            border_style_str,
            b"normal\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            border_style = BS_NORMAL;
        } else if strcmp(
            border_style_str,
            b"pixel\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            border_style = BS_PIXEL;
        } else if strcmp(
            border_style_str,
            b"none\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            border_style = BS_NONE;
        } else {
            if !(*cmd_output).json_gen.is_null() {
                let mut message_0: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                sasprintf(
                    &raw mut message_0,
                    b"BUG: called with border_style=%s\0" as *const u8
                        as *const ::core::ffi::c_char,
                    border_style_str,
                );
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_map_open((*cmd_output).json_gen);
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_string(
                        (*cmd_output).json_gen,
                        b"success\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_uchar,
                        strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                    );
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_bool((*cmd_output).json_gen, false_0);
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_string(
                        (*cmd_output).json_gen,
                        b"error\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_uchar,
                        strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
                    );
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_string(
                        (*cmd_output).json_gen,
                        message_0 as *mut ::core::ffi::c_uchar,
                        strlen(message_0),
                    );
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_map_close((*cmd_output).json_gen);
                } else {
                };
                free(message_0 as *mut ::core::ffi::c_void);
            }
            return;
        }
        (*(*current).con).max_user_border_style = border_style;
        let con_border_width: ::core::ffi::c_int =
            border_width_from_style(border_style, border_width, (*current).con)
                as ::core::ffi::c_int;
        con_set_border_style((*current).con, border_style, con_border_width);
        current = (*current).owindows.tqe_next as *mut owindow;
    }
    (*cmd_output).needs_tree_render = true_0 != 0;
    if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                b"success\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_uchar,
                strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn cmd_nop(
    mut current_match: *mut Match,
    mut cmd_output: *mut CommandResultIR,
    mut comment: *const ::core::ffi::c_char,
) {
    verboselog(
        b"-------------------------------------------------\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    verboselog(
        b"  NOP: %.4000s\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        comment,
    );
    verboselog(
        b"-------------------------------------------------\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                b"success\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_uchar,
                strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn cmd_append_layout(
    mut current_match: *mut Match,
    mut cmd_output: *mut CommandResultIR,
    mut cpath: *const ::core::ffi::c_char,
) {
    let mut content: json_content_t = JSON_CONTENT_UNKNOWN;
    let mut parent: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut errormsg: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    verboselog(
        b"Appending layout \"%s\"\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        cpath,
    );
    let mut path: *mut ::core::ffi::c_char = resolve_tilde(cpath);
    let mut buf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut len: ssize_t = 0;
    len = slurp(path, &raw mut buf);
    if len < 0 as ssize_t {
        if !(*cmd_output).json_gen.is_null() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                b"Could not slurp \"%s\".\0" as *const u8 as *const ::core::ffi::c_char,
                path,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"success\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"error\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message as *mut ::core::ffi::c_uchar,
                    strlen(message),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message as *mut ::core::ffi::c_void);
        }
    } else if !json_validate(buf, len as size_t) {
        errorlog(
            b"ERROR: Could not parse \"%s\" as JSON, not loading.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            path,
        );
        if !(*cmd_output).json_gen.is_null() {
            let mut message_0: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message_0,
                b"Could not parse \"%s\" as JSON.\0" as *const u8 as *const ::core::ffi::c_char,
                path,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"success\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"error\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message_0 as *mut ::core::ffi::c_uchar,
                    strlen(message_0),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message_0 as *mut ::core::ffi::c_void);
        }
    } else {
        content = json_determine_content(buf, len as size_t) as json_content_t;
        verboselog(
            b"JSON content = %d\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            content as ::core::ffi::c_uint,
        );
        if content as ::core::ffi::c_uint
            == JSON_CONTENT_UNKNOWN as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            errorlog(
                b"ERROR: Could not determine the contents of \"%s\", not loading.\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                path,
            );
            if !(*cmd_output).json_gen.is_null() {
                let mut message_1: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                sasprintf(
                    &raw mut message_1,
                    b"Could not determine the contents of \"%s\".\0" as *const u8
                        as *const ::core::ffi::c_char,
                    path,
                );
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_map_open((*cmd_output).json_gen);
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_string(
                        (*cmd_output).json_gen,
                        b"success\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_uchar,
                        strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                    );
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_bool((*cmd_output).json_gen, false_0);
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_string(
                        (*cmd_output).json_gen,
                        b"error\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_uchar,
                        strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
                    );
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_string(
                        (*cmd_output).json_gen,
                        message_1 as *mut ::core::ffi::c_uchar,
                        strlen(message_1),
                    );
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_map_close((*cmd_output).json_gen);
                } else {
                };
                free(message_1 as *mut ::core::ffi::c_void);
            }
        } else {
            parent = focused;
            if content as ::core::ffi::c_uint
                == JSON_CONTENT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                parent = output_get_content(con_get_output(parent));
            } else {
                while (*parent).type_0 as ::core::ffi::c_uint
                    != CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
                    && !con_accepts_window(parent)
                {
                    parent = (*parent).parent as *mut Con;
                }
            }
            debuglog(
                b"%s:%s:%d - Appending to parent=%p instead of focused=%p\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"../src/commands.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"cmd_append_layout\0" as *const u8 as *const ::core::ffi::c_char,
                784 as ::core::ffi::c_int,
                parent,
                focused,
            );
            errormsg = ::core::ptr::null_mut::<::core::ffi::c_char>();
            tree_append_json(parent, buf, len as size_t, &raw mut errormsg);
            if !errormsg.is_null() {
                if !(*cmd_output).json_gen.is_null() {
                    let mut message_2: *mut ::core::ffi::c_char =
                        ::core::ptr::null_mut::<::core::ffi::c_char>();
                    sasprintf(
                        &raw mut message_2,
                        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                        errormsg,
                    );
                    if !(*cmd_output).json_gen.is_null() {
                        yajl_gen_map_open((*cmd_output).json_gen);
                    } else {
                    };
                    if !(*cmd_output).json_gen.is_null() {
                        yajl_gen_string(
                            (*cmd_output).json_gen,
                            b"success\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_uchar,
                            strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                        );
                    } else {
                    };
                    if !(*cmd_output).json_gen.is_null() {
                        yajl_gen_bool((*cmd_output).json_gen, false_0);
                    } else {
                    };
                    if !(*cmd_output).json_gen.is_null() {
                        yajl_gen_string(
                            (*cmd_output).json_gen,
                            b"error\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_uchar,
                            strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
                        );
                    } else {
                    };
                    if !(*cmd_output).json_gen.is_null() {
                        yajl_gen_string(
                            (*cmd_output).json_gen,
                            message_2 as *mut ::core::ffi::c_uchar,
                            strlen(message_2),
                        );
                    } else {
                    };
                    if !(*cmd_output).json_gen.is_null() {
                        yajl_gen_map_close((*cmd_output).json_gen);
                    } else {
                    };
                    free(message_2 as *mut ::core::ffi::c_void);
                }
                free(errormsg as *mut ::core::ffi::c_void);
            } else if !(*cmd_output).json_gen.is_null() {
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_map_open((*cmd_output).json_gen);
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_string(
                        (*cmd_output).json_gen,
                        b"success\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_uchar,
                        strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                    );
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_map_close((*cmd_output).json_gen);
                } else {
                };
            }
            render_con(croot);
            restore_open_placeholder_windows(parent);
            if content as ::core::ffi::c_uint
                == JSON_CONTENT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                ipc_send_workspace_event(
                    b"restored\0" as *const u8 as *const ::core::ffi::c_char,
                    parent,
                    ::core::ptr::null_mut::<Con>(),
                );
            }
            (*cmd_output).needs_tree_render = true_0 != 0;
        }
    }
    free(path as *mut ::core::ffi::c_void);
    free(buf as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn disable_global_fullscreen() {
    let mut fs: *mut Con = con_get_fullscreen_con(croot, CF_GLOBAL);
    if !fs.is_null() {
        con_disable_fullscreen(fs);
    }
}
#[no_mangle]
pub unsafe extern "C" fn cmd_workspace(
    mut current_match: *mut Match,
    mut cmd_output: *mut CommandResultIR,
    mut which: *const ::core::ffi::c_char,
) {
    let mut ws: *mut Con = ::core::ptr::null_mut::<Con>();
    debuglog(
        b"%s:%s:%d - which=%s\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/commands.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"cmd_workspace\0" as *const u8 as *const ::core::ffi::c_char,
        832 as ::core::ffi::c_int,
        which,
    );
    disable_global_fullscreen();
    if strcmp(which, b"next\0" as *const u8 as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        ws = workspace_next();
    } else if strcmp(which, b"prev\0" as *const u8 as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        ws = workspace_prev();
    } else if strcmp(
        which,
        b"next_on_output\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        ws = workspace_next_on_output();
    } else if strcmp(
        which,
        b"prev_on_output\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        ws = workspace_prev_on_output();
    } else {
        if !(*cmd_output).json_gen.is_null() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                b"BUG: called with which=%s\0" as *const u8 as *const ::core::ffi::c_char,
                which,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"success\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"error\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message as *mut ::core::ffi::c_uchar,
                    strlen(message),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message as *mut ::core::ffi::c_void);
        }
        return;
    }
    workspace_show(ws);
    (*cmd_output).needs_tree_render = true_0 != 0;
    if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                b"success\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_uchar,
                strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn cmd_workspace_number(
    mut current_match: *mut Match,
    mut cmd_output: *mut CommandResultIR,
    mut which: *const ::core::ffi::c_char,
    mut no_auto_back_and_forth_str: *const ::core::ffi::c_char,
) {
    let no_auto_back_and_forth: bool = !no_auto_back_and_forth_str.is_null();
    disable_global_fullscreen();
    let parsed_num: ::core::ffi::c_long = ws_name_to_number(which) as ::core::ffi::c_long;
    if parsed_num == -(1 as ::core::ffi::c_int) as ::core::ffi::c_long {
        if !(*cmd_output).json_gen.is_null() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                b"Could not parse initial part of \"%s\" as a number.\0" as *const u8
                    as *const ::core::ffi::c_char,
                which,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"success\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"error\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message as *mut ::core::ffi::c_uchar,
                    strlen(message),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message as *mut ::core::ffi::c_void);
        }
        return;
    }
    let mut workspace: *mut Con = get_existing_workspace_by_num(parsed_num as ::core::ffi::c_int);
    if workspace.is_null() {
        verboselog(
            b"There is no workspace with number %ld, creating a new one.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            parsed_num,
        );
        if !(*cmd_output).json_gen.is_null() {
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"success\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
        }
        workspace_show_by_name(which);
        (*cmd_output).needs_tree_render = true_0 != 0;
        return;
    }
    if !no_auto_back_and_forth
        && maybe_back_and_forth(cmd_output, (*workspace).name) as ::core::ffi::c_int != 0
    {
        if !(*cmd_output).json_gen.is_null() {
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"success\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
        }
        return;
    }
    workspace_show(workspace);
    (*cmd_output).needs_tree_render = true_0 != 0;
    if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                b"success\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_uchar,
                strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn cmd_workspace_back_and_forth(
    mut current_match: *mut Match,
    mut cmd_output: *mut CommandResultIR,
) {
    disable_global_fullscreen();
    workspace_back_and_forth();
    (*cmd_output).needs_tree_render = true_0 != 0;
    if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                b"success\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_uchar,
                strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn cmd_workspace_name(
    mut current_match: *mut Match,
    mut cmd_output: *mut CommandResultIR,
    mut name: *const ::core::ffi::c_char,
    mut no_auto_back_and_forth_str: *const ::core::ffi::c_char,
) {
    let no_auto_back_and_forth: bool = !no_auto_back_and_forth_str.is_null();
    if strncasecmp(
        name,
        b"__\0" as *const u8 as *const ::core::ffi::c_char,
        strlen(b"__\0" as *const u8 as *const ::core::ffi::c_char),
    ) == 0 as ::core::ffi::c_int
    {
        if !(*cmd_output).json_gen.is_null() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                b"You cannot switch to the i3-internal workspaces (\"%s\").\0" as *const u8
                    as *const ::core::ffi::c_char,
                name,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"success\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"error\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message as *mut ::core::ffi::c_uchar,
                    strlen(message),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message as *mut ::core::ffi::c_void);
        }
        return;
    }
    disable_global_fullscreen();
    debuglog(
        b"%s:%s:%d - should switch to workspace %s\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/commands.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"cmd_workspace_name\0" as *const u8 as *const ::core::ffi::c_char,
        918 as ::core::ffi::c_int,
        name,
    );
    if !no_auto_back_and_forth && maybe_back_and_forth(cmd_output, name) as ::core::ffi::c_int != 0
    {
        if !(*cmd_output).json_gen.is_null() {
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"success\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
        }
        return;
    }
    workspace_show_by_name(name);
    (*cmd_output).needs_tree_render = true_0 != 0;
    if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                b"success\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_uchar,
                strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn cmd_mark(
    mut current_match: *mut Match,
    mut cmd_output: *mut CommandResultIR,
    mut mark: *const ::core::ffi::c_char,
    mut mode: *const ::core::ffi::c_char,
    mut toggle: *const ::core::ffi::c_char,
) {
    if !(*current_match).error.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                b"Invalid match: %s\0" as *const u8 as *const ::core::ffi::c_char,
                (*current_match).error,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"success\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"error\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message as *mut ::core::ffi::c_uchar,
                    strlen(message),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message as *mut ::core::ffi::c_void);
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let mut ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
            if !(*ow).owindows.tqe_next.is_null() {
                (*(*ow).owindows.tqe_next).owindows.tqe_prev = (*ow).owindows.tqe_prev;
            } else {
                (*(*cmd_output).ctx).owindows.tqh_last = (*ow).owindows.tqe_prev;
            }
            *(*ow).owindows.tqe_prev = (*ow).owindows.tqe_next;
            free(ow as *mut ::core::ffi::c_void);
        }
        let mut ow_0: *mut owindow =
            smalloc(::core::mem::size_of::<owindow>() as size_t) as *mut owindow;
        (*ow_0).con = focused;
        (*(*cmd_output).ctx).owindows.tqh_first = ::core::ptr::null_mut::<owindow>();
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*(*cmd_output).ctx).owindows.tqh_first;
        (*ow_0).owindows.tqe_next = ::core::ptr::null_mut::<owindow>();
        (*ow_0).owindows.tqe_prev = (*(*cmd_output).ctx).owindows.tqh_last;
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0 as *mut owindow;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    let mut current: *const owindow = (*(*cmd_output).ctx).owindows.tqh_first;
    if current.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            let mut message_0: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message_0,
                b"Given criteria don't match a window\0" as *const u8 as *const ::core::ffi::c_char,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"success\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"error\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message_0 as *mut ::core::ffi::c_uchar,
                    strlen(message_0),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message_0 as *mut ::core::ffi::c_void);
        }
        return;
    }
    if current
        != *(*((*(*cmd_output).ctx).owindows.tqh_last as *mut owindows_head)).tqh_last
            as *const owindow
    {
        if !(*cmd_output).json_gen.is_null() {
            let mut message_1: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message_1,
                b"A mark must not be put onto more than one window\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"success\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"error\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message_1 as *mut ::core::ffi::c_uchar,
                    strlen(message_1),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message_1 as *mut ::core::ffi::c_void);
        }
        return;
    }
    debuglog(
        b"%s:%s:%d - matching: %p / %s\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/commands.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"cmd_mark\0" as *const u8 as *const ::core::ffi::c_char,
        949 as ::core::ffi::c_int,
        (*current).con,
        (*(*current).con).name,
    );
    let mark_mode: mark_mode_t = (if mode.is_null()
        || strcmp(
            mode,
            b"--replace\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
    {
        MM_REPLACE as ::core::ffi::c_int
    } else {
        MM_ADD as ::core::ffi::c_int
    }) as mark_mode_t;
    if !toggle.is_null() {
        con_mark_toggle((*current).con, mark, mark_mode);
    } else {
        con_mark((*current).con, mark, mark_mode);
    }
    (*cmd_output).needs_tree_render = true_0 != 0;
    if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                b"success\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_uchar,
                strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn cmd_unmark(
    mut current_match: *mut Match,
    mut cmd_output: *mut CommandResultIR,
    mut mark: *const ::core::ffi::c_char,
) {
    if match_is_empty(current_match) {
        con_unmark(::core::ptr::null_mut::<Con>(), mark);
    } else {
        let mut current: *mut owindow = ::core::ptr::null_mut::<owindow>();
        current = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
        while !current.is_null() {
            con_unmark((*current).con, mark);
            current = (*current).owindows.tqe_next as *mut owindow;
        }
    }
    (*cmd_output).needs_tree_render = true_0 != 0;
    if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                b"success\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_uchar,
                strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn cmd_mode(
    mut current_match: *mut Match,
    mut cmd_output: *mut CommandResultIR,
    mut mode: *const ::core::ffi::c_char,
) {
    debuglog(
        b"%s:%s:%d - mode=%s\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/commands.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"cmd_mode\0" as *const u8 as *const ::core::ffi::c_char,
        987 as ::core::ffi::c_int,
        mode,
    );
    switch_mode(mode);
    if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                b"success\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_uchar,
                strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
    }
}
unsafe extern "C" fn user_output_names_add(
    mut list: *mut user_output_names_head,
    mut name: *const ::core::ffi::c_char,
) {
    let get_non_primary: bool = strcasecmp(
        b"nonprimary\0" as *const u8 as *const ::core::ffi::c_char,
        name,
    ) == 0 as ::core::ffi::c_int;
    if get_non_primary as ::core::ffi::c_int != 0
        || strcmp(name, b"next\0" as *const u8 as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
    {
        let mut output: *mut Output = ::core::ptr::null_mut::<Output>();
        output = outputs.tqh_first as *mut Output;
        while !output.is_null() {
            if !(get_non_primary as ::core::ffi::c_int != 0
                && (*output).primary as ::core::ffi::c_int != 0)
            {
                let mut co: *mut user_output_name = scalloc(
                    ::core::mem::size_of::<user_output_name>() as size_t,
                    1 as size_t,
                ) as *mut user_output_name;
                (*co).name = sstrdup(output_primary_name(output));
                (*co).user_output_names.tqe_next = ::core::ptr::null_mut::<user_output_name>();
                (*co).user_output_names.tqe_prev = (*list).tqh_last;
                *(*list).tqh_last = co as *mut user_output_name;
                (*list).tqh_last = &raw mut (*co).user_output_names.tqe_next;
            }
            output = (*output).outputs.tqe_next as *mut Output;
        }
        return;
    }
    let mut co_0: *mut user_output_name = scalloc(
        ::core::mem::size_of::<user_output_name>() as size_t,
        1 as size_t,
    ) as *mut user_output_name;
    (*co_0).name = sstrdup(name);
    (*co_0).user_output_names.tqe_next = ::core::ptr::null_mut::<user_output_name>();
    (*co_0).user_output_names.tqe_prev = (*list).tqh_last;
    *(*list).tqh_last = co_0 as *mut user_output_name;
    (*list).tqh_last = &raw mut (*co_0).user_output_names.tqe_next;
}
unsafe extern "C" fn user_output_names_find_next(
    mut names_0: *const user_output_names_head,
    mut current_output: *mut Output,
) -> *mut Output {
    let mut target_output: *mut Output = ::core::ptr::null_mut::<Output>();
    let mut uo: *mut user_output_name = ::core::ptr::null_mut::<user_output_name>();
    uo = (*names_0).tqh_first as *mut user_output_name;
    while !uo.is_null() {
        if target_output.is_null() {
            target_output = get_output_from_string(current_output, (*uo).name);
        }
        if strcasecmp(output_primary_name(current_output), (*uo).name) == 0 as ::core::ffi::c_int {
            loop {
                uo = (*uo).user_output_names.tqe_next as *mut user_output_name;
                if uo.is_null() {
                    break;
                }
                let mut out: *mut Output = get_output_from_string(current_output, (*uo).name);
                if !out.is_null() {
                    return out;
                }
            }
            break;
        } else {
            uo = (*uo).user_output_names.tqe_next as *mut user_output_name;
        }
    }
    return target_output;
}
unsafe extern "C" fn user_output_names_free(mut names_0: *mut user_output_names_head) {
    while !(*names_0).tqh_first.is_null() {
        let mut uo: *mut user_output_name = (*names_0).tqh_first as *mut user_output_name;
        free((*uo).name as *mut ::core::ffi::c_void);
        if !(*uo).user_output_names.tqe_next.is_null() {
            (*(*uo).user_output_names.tqe_next)
                .user_output_names
                .tqe_prev = (*uo).user_output_names.tqe_prev;
        } else {
            (*names_0).tqh_last = (*uo).user_output_names.tqe_prev;
        }
        *(*uo).user_output_names.tqe_prev = (*uo).user_output_names.tqe_next;
        free(uo as *mut ::core::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn cmd_move_con_to_output(
    mut current_match: *mut Match,
    mut cmd_output: *mut CommandResultIR,
    mut name: *const ::core::ffi::c_char,
    move_workspace: bool,
) {
    if !name.is_null() {
        user_output_names_add(&raw mut names, name);
        return;
    }
    if !(*current_match).error.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                b"Invalid match: %s\0" as *const u8 as *const ::core::ffi::c_char,
                (*current_match).error,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"success\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"error\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message as *mut ::core::ffi::c_uchar,
                    strlen(message),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message as *mut ::core::ffi::c_void);
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let mut ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
            if !(*ow).owindows.tqe_next.is_null() {
                (*(*ow).owindows.tqe_next).owindows.tqe_prev = (*ow).owindows.tqe_prev;
            } else {
                (*(*cmd_output).ctx).owindows.tqh_last = (*ow).owindows.tqe_prev;
            }
            *(*ow).owindows.tqe_prev = (*ow).owindows.tqe_next;
            free(ow as *mut ::core::ffi::c_void);
        }
        let mut ow_0: *mut owindow =
            smalloc(::core::mem::size_of::<owindow>() as size_t) as *mut owindow;
        (*ow_0).con = focused;
        (*(*cmd_output).ctx).owindows.tqh_first = ::core::ptr::null_mut::<owindow>();
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*(*cmd_output).ctx).owindows.tqh_first;
        (*ow_0).owindows.tqe_next = ::core::ptr::null_mut::<owindow>();
        (*ow_0).owindows.tqe_prev = (*(*cmd_output).ctx).owindows.tqh_last;
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0 as *mut owindow;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    if names.tqh_first.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            let mut message_0: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message_0,
                b"At least one output must be specified\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"success\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"error\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message_0 as *mut ::core::ffi::c_uchar,
                    strlen(message_0),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message_0 as *mut ::core::ffi::c_void);
        }
        return;
    }
    let mut success: bool = false_0 != 0;
    let mut current: *mut owindow = ::core::ptr::null_mut::<owindow>();
    current = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
    while !current.is_null() {
        let mut ws: *mut Con = con_get_workspace((*current).con);
        if !con_is_internal(ws) {
            let mut current_output: *mut Output = get_output_for_con(ws);
            let mut target_output: *mut Output =
                user_output_names_find_next(&raw mut names, current_output);
            if !target_output.is_null() {
                if move_workspace {
                    workspace_move_to_output(ws, target_output);
                } else {
                    con_move_to_output((*current).con, target_output, true_0 != 0);
                }
                success = true_0 != 0;
            }
        }
        current = (*current).owindows.tqe_next as *mut owindow;
    }
    user_output_names_free(&raw mut names);
    (*cmd_output).needs_tree_render = success;
    if success {
        if !(*cmd_output).json_gen.is_null() {
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"success\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
        }
    } else if !(*cmd_output).json_gen.is_null() {
        let mut message_1: *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<::core::ffi::c_char>();
        sasprintf(
            &raw mut message_1,
            b"No output matched\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                b"success\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_uchar,
                strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, false_0);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                b"error\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
                strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                message_1 as *mut ::core::ffi::c_uchar,
                strlen(message_1),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
        free(message_1 as *mut ::core::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn cmd_move_con_to_mark(
    mut current_match: *mut Match,
    mut cmd_output: *mut CommandResultIR,
    mut mark: *const ::core::ffi::c_char,
) {
    debuglog(
        b"%s:%s:%d - moving window to mark \"%s\"\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/commands.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"cmd_move_con_to_mark\0" as *const u8 as *const ::core::ffi::c_char,
        1123 as ::core::ffi::c_int,
        mark,
    );
    if !(*current_match).error.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                b"Invalid match: %s\0" as *const u8 as *const ::core::ffi::c_char,
                (*current_match).error,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"success\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"error\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message as *mut ::core::ffi::c_uchar,
                    strlen(message),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message as *mut ::core::ffi::c_void);
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let mut ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
            if !(*ow).owindows.tqe_next.is_null() {
                (*(*ow).owindows.tqe_next).owindows.tqe_prev = (*ow).owindows.tqe_prev;
            } else {
                (*(*cmd_output).ctx).owindows.tqh_last = (*ow).owindows.tqe_prev;
            }
            *(*ow).owindows.tqe_prev = (*ow).owindows.tqe_next;
            free(ow as *mut ::core::ffi::c_void);
        }
        let mut ow_0: *mut owindow =
            smalloc(::core::mem::size_of::<owindow>() as size_t) as *mut owindow;
        (*ow_0).con = focused;
        (*(*cmd_output).ctx).owindows.tqh_first = ::core::ptr::null_mut::<owindow>();
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*(*cmd_output).ctx).owindows.tqh_first;
        (*ow_0).owindows.tqe_next = ::core::ptr::null_mut::<owindow>();
        (*ow_0).owindows.tqe_prev = (*(*cmd_output).ctx).owindows.tqh_last;
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0 as *mut owindow;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    let mut result: bool = true_0 != 0;
    let mut current: *mut owindow = ::core::ptr::null_mut::<owindow>();
    current = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
    while !current.is_null() {
        debuglog(
            b"%s:%s:%d - moving matched window %p / %s to mark \"%s\"\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/commands.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"cmd_move_con_to_mark\0" as *const u8 as *const ::core::ffi::c_char,
            1130 as ::core::ffi::c_int,
            (*current).con,
            (*(*current).con).name,
            mark,
        );
        result = (result as ::core::ffi::c_int
            & con_move_to_mark((*current).con, mark) as ::core::ffi::c_int)
            != 0;
        current = (*current).owindows.tqe_next as *mut owindow;
    }
    (*cmd_output).needs_tree_render = true_0 != 0;
    if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                b"success\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_uchar,
                strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, result as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn cmd_floating(
    mut current_match: *mut Match,
    mut cmd_output: *mut CommandResultIR,
    mut floating_mode: *const ::core::ffi::c_char,
) {
    if floating_mode != ::core::ptr::null_mut::<::core::ffi::c_void>() as *const ::core::ffi::c_char
    {
    } else {
        __assert_fail(
            b"floating_mode != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            b"../src/commands.c\0" as *const u8 as *const ::core::ffi::c_char,
            1143 as ::core::ffi::c_uint,
            b"void cmd_floating(Match *, struct CommandResultIR *, const char *)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    };
    debuglog(
        b"%s:%s:%d - floating_mode=%s\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/commands.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"cmd_floating\0" as *const u8 as *const ::core::ffi::c_char,
        1144 as ::core::ffi::c_int,
        floating_mode,
    );
    if !(*current_match).error.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                b"Invalid match: %s\0" as *const u8 as *const ::core::ffi::c_char,
                (*current_match).error,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"success\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"error\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message as *mut ::core::ffi::c_uchar,
                    strlen(message),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message as *mut ::core::ffi::c_void);
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let mut ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
            if !(*ow).owindows.tqe_next.is_null() {
                (*(*ow).owindows.tqe_next).owindows.tqe_prev = (*ow).owindows.tqe_prev;
            } else {
                (*(*cmd_output).ctx).owindows.tqh_last = (*ow).owindows.tqe_prev;
            }
            *(*ow).owindows.tqe_prev = (*ow).owindows.tqe_next;
            free(ow as *mut ::core::ffi::c_void);
        }
        let mut ow_0: *mut owindow =
            smalloc(::core::mem::size_of::<owindow>() as size_t) as *mut owindow;
        (*ow_0).con = focused;
        (*(*cmd_output).ctx).owindows.tqh_first = ::core::ptr::null_mut::<owindow>();
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*(*cmd_output).ctx).owindows.tqh_first;
        (*ow_0).owindows.tqe_next = ::core::ptr::null_mut::<owindow>();
        (*ow_0).owindows.tqe_prev = (*(*cmd_output).ctx).owindows.tqh_last;
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0 as *mut owindow;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    let mut current: *mut owindow = ::core::ptr::null_mut::<owindow>();
    current = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
    while !current.is_null() {
        debuglog(
            b"%s:%s:%d - matching: %p / %s\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b"../src/commands.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"cmd_floating\0" as *const u8 as *const ::core::ffi::c_char,
            1150 as ::core::ffi::c_int,
            (*current).con,
            (*(*current).con).name,
        );
        if strcmp(
            floating_mode,
            b"toggle\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            debuglog(
                b"%s:%s:%d - should toggle mode\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"../src/commands.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"cmd_floating\0" as *const u8 as *const ::core::ffi::c_char,
                1152 as ::core::ffi::c_int,
            );
            toggle_floating_mode((*current).con, false_0 != 0);
        } else {
            debuglog(
                b"%s:%s:%d - should switch mode to %s\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"../src/commands.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"cmd_floating\0" as *const u8 as *const ::core::ffi::c_char,
                1155 as ::core::ffi::c_int,
                floating_mode,
            );
            if strcmp(
                floating_mode,
                b"enable\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                floating_enable((*current).con, false_0 != 0);
            } else {
                floating_disable((*current).con);
            }
        }
        run_assignments((*(*current).con).window as *mut i3Window);
        current = (*current).owindows.tqe_next as *mut owindow;
    }
    (*cmd_output).needs_tree_render = true_0 != 0;
    if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                b"success\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_uchar,
                strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn cmd_split(
    mut current_match: *mut Match,
    mut cmd_output: *mut CommandResultIR,
    mut direction: *const ::core::ffi::c_char,
) {
    if !(*current_match).error.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                b"Invalid match: %s\0" as *const u8 as *const ::core::ffi::c_char,
                (*current_match).error,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"success\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"error\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message as *mut ::core::ffi::c_uchar,
                    strlen(message),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message as *mut ::core::ffi::c_void);
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let mut ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
            if !(*ow).owindows.tqe_next.is_null() {
                (*(*ow).owindows.tqe_next).owindows.tqe_prev = (*ow).owindows.tqe_prev;
            } else {
                (*(*cmd_output).ctx).owindows.tqh_last = (*ow).owindows.tqe_prev;
            }
            *(*ow).owindows.tqe_prev = (*ow).owindows.tqe_next;
            free(ow as *mut ::core::ffi::c_void);
        }
        let mut ow_0: *mut owindow =
            smalloc(::core::mem::size_of::<owindow>() as size_t) as *mut owindow;
        (*ow_0).con = focused;
        (*(*cmd_output).ctx).owindows.tqh_first = ::core::ptr::null_mut::<owindow>();
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*(*cmd_output).ctx).owindows.tqh_first;
        (*ow_0).owindows.tqe_next = ::core::ptr::null_mut::<owindow>();
        (*ow_0).owindows.tqe_prev = (*(*cmd_output).ctx).owindows.tqh_last;
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0 as *mut owindow;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    let mut current: *mut owindow = ::core::ptr::null_mut::<owindow>();
    verboselog(
        b"splitting in direction %c\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        *direction.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
    );
    current = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
    while !current.is_null() {
        if con_is_docked((*current).con) {
            errorlog(
                b"ERROR: Cannot split a docked container, skipping.\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        } else {
            debuglog(
                b"%s:%s:%d - matching: %p / %s\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"../src/commands.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"cmd_split\0" as *const u8 as *const ::core::ffi::c_char,
                1185 as ::core::ffi::c_int,
                (*current).con,
                (*(*current).con).name,
            );
            if *direction.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 't' as i32
            {
                let mut current_layout: layout_t = L_DEFAULT;
                if (*(*current).con).type_0 as ::core::ffi::c_uint
                    == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    current_layout = (*(*current).con).layout;
                } else {
                    current_layout = (*(*(*current).con).parent).layout;
                }
                if current_layout as ::core::ffi::c_uint
                    == L_SPLITH as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    tree_split((*current).con, VERT);
                } else {
                    tree_split((*current).con, HORIZ);
                }
            } else {
                tree_split(
                    (*current).con,
                    (if *direction.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == 'v' as i32
                    {
                        VERT as ::core::ffi::c_int
                    } else {
                        HORIZ as ::core::ffi::c_int
                    }) as orientation_t,
                );
            }
        }
        current = (*current).owindows.tqe_next as *mut owindow;
    }
    (*cmd_output).needs_tree_render = true_0 != 0;
    if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                b"success\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_uchar,
                strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn cmd_kill(
    mut current_match: *mut Match,
    mut cmd_output: *mut CommandResultIR,
    mut kill_mode_str: *const ::core::ffi::c_char,
) {
    if kill_mode_str.is_null() {
        kill_mode_str = b"window\0" as *const u8 as *const ::core::ffi::c_char;
    }
    debuglog(
        b"%s:%s:%d - kill_mode=%s\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/commands.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"cmd_kill\0" as *const u8 as *const ::core::ffi::c_char,
        1218 as ::core::ffi::c_int,
        kill_mode_str,
    );
    let mut kill_mode: ::core::ffi::c_int = 0;
    if strcmp(
        kill_mode_str,
        b"window\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        kill_mode = KILL_WINDOW as ::core::ffi::c_int;
    } else if strcmp(
        kill_mode_str,
        b"client\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        kill_mode = KILL_CLIENT as ::core::ffi::c_int;
    } else {
        if !(*cmd_output).json_gen.is_null() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                b"BUG: called with kill_mode=%s\0" as *const u8 as *const ::core::ffi::c_char,
                kill_mode_str,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"success\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"error\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message as *mut ::core::ffi::c_uchar,
                    strlen(message),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message as *mut ::core::ffi::c_void);
        }
        return;
    }
    if !(*current_match).error.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            let mut message_0: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message_0,
                b"Invalid match: %s\0" as *const u8 as *const ::core::ffi::c_char,
                (*current_match).error,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"success\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"error\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message_0 as *mut ::core::ffi::c_uchar,
                    strlen(message_0),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message_0 as *mut ::core::ffi::c_void);
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let mut ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
            if !(*ow).owindows.tqe_next.is_null() {
                (*(*ow).owindows.tqe_next).owindows.tqe_prev = (*ow).owindows.tqe_prev;
            } else {
                (*(*cmd_output).ctx).owindows.tqh_last = (*ow).owindows.tqe_prev;
            }
            *(*ow).owindows.tqe_prev = (*ow).owindows.tqe_next;
            free(ow as *mut ::core::ffi::c_void);
        }
        let mut ow_0: *mut owindow =
            smalloc(::core::mem::size_of::<owindow>() as size_t) as *mut owindow;
        (*ow_0).con = focused;
        (*(*cmd_output).ctx).owindows.tqh_first = ::core::ptr::null_mut::<owindow>();
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*(*cmd_output).ctx).owindows.tqh_first;
        (*ow_0).owindows.tqe_next = ::core::ptr::null_mut::<owindow>();
        (*ow_0).owindows.tqe_prev = (*(*cmd_output).ctx).owindows.tqh_last;
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0 as *mut owindow;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    let mut current: *mut owindow = ::core::ptr::null_mut::<owindow>();
    current = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
    while !current.is_null() {
        con_close((*current).con, kill_mode as kill_window_t);
        current = (*current).owindows.tqe_next as *mut owindow;
    }
    (*cmd_output).needs_tree_render = true_0 != 0;
    if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                b"success\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_uchar,
                strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn cmd_exec(
    mut current_match: *mut Match,
    mut cmd_output: *mut CommandResultIR,
    mut nosn: *const ::core::ffi::c_char,
    mut command: *const ::core::ffi::c_char,
) {
    let no_startup_id: bool = !nosn.is_null();
    if !(*current_match).error.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                b"Invalid match: %s\0" as *const u8 as *const ::core::ffi::c_char,
                (*current_match).error,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"success\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"error\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message as *mut ::core::ffi::c_uchar,
                    strlen(message),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message as *mut ::core::ffi::c_void);
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let mut ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
            if !(*ow).owindows.tqe_next.is_null() {
                (*(*ow).owindows.tqe_next).owindows.tqe_prev = (*ow).owindows.tqe_prev;
            } else {
                (*(*cmd_output).ctx).owindows.tqh_last = (*ow).owindows.tqe_prev;
            }
            *(*ow).owindows.tqe_prev = (*ow).owindows.tqe_next;
            free(ow as *mut ::core::ffi::c_void);
        }
        let mut ow_0: *mut owindow =
            smalloc(::core::mem::size_of::<owindow>() as size_t) as *mut owindow;
        (*ow_0).con = focused;
        (*(*cmd_output).ctx).owindows.tqh_first = ::core::ptr::null_mut::<owindow>();
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*(*cmd_output).ctx).owindows.tqh_first;
        (*ow_0).owindows.tqe_next = ::core::ptr::null_mut::<owindow>();
        (*ow_0).owindows.tqe_prev = (*(*cmd_output).ctx).owindows.tqh_last;
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0 as *mut owindow;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut current: *mut owindow = ::core::ptr::null_mut::<owindow>();
    current = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
    while !current.is_null() {
        count += 1;
        current = (*current).owindows.tqe_next as *mut owindow;
    }
    if count > 1 as ::core::ffi::c_int {
        verboselog(
            b"WARNING: Your criteria for the exec command match %d containers, so the command will execute this many times.\n\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            count,
        );
    }
    current = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
    while !current.is_null() {
        debuglog(
            b"%s:%s:%d - should execute %s, no_startup_id = %d\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/commands.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"cmd_exec\0" as *const u8 as *const ::core::ffi::c_char,
            1264 as ::core::ffi::c_int,
            command,
            no_startup_id as ::core::ffi::c_int,
        );
        start_application(
            command,
            no_startup_id,
            (if (*(*current).con).window.is_null() {
                XCB_NONE
            } else {
                (*(*(*current).con).window).id as ::core::ffi::c_long
            }) as xcb_window_t,
        );
        current = (*current).owindows.tqe_next as *mut owindow;
    }
    if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                b"success\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_uchar,
                strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn cmd_focus_direction(
    mut current_match: *mut Match,
    mut cmd_output: *mut CommandResultIR,
    mut direction_str: *const ::core::ffi::c_char,
) {
    if !(*current_match).error.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                b"Invalid match: %s\0" as *const u8 as *const ::core::ffi::c_char,
                (*current_match).error,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"success\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"error\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message as *mut ::core::ffi::c_uchar,
                    strlen(message),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message as *mut ::core::ffi::c_void);
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let mut ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
            if !(*ow).owindows.tqe_next.is_null() {
                (*(*ow).owindows.tqe_next).owindows.tqe_prev = (*ow).owindows.tqe_prev;
            } else {
                (*(*cmd_output).ctx).owindows.tqh_last = (*ow).owindows.tqe_prev;
            }
            *(*ow).owindows.tqe_prev = (*ow).owindows.tqe_next;
            free(ow as *mut ::core::ffi::c_void);
        }
        let mut ow_0: *mut owindow =
            smalloc(::core::mem::size_of::<owindow>() as size_t) as *mut owindow;
        (*ow_0).con = focused;
        (*(*cmd_output).ctx).owindows.tqh_first = ::core::ptr::null_mut::<owindow>();
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*(*cmd_output).ctx).owindows.tqh_first;
        (*ow_0).owindows.tqe_next = ::core::ptr::null_mut::<owindow>();
        (*ow_0).owindows.tqe_prev = (*(*cmd_output).ctx).owindows.tqh_last;
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0 as *mut owindow;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut current: *mut owindow = ::core::ptr::null_mut::<owindow>();
    current = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
    while !current.is_null() {
        count += 1;
        current = (*current).owindows.tqe_next as *mut owindow;
    }
    if count > 1 as ::core::ffi::c_int {
        verboselog(
            b"WARNING: Your criteria for the focus command matches %d containers, while only exactly one container can be focused at a time.\n\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            count,
        );
    }
    let mut direction: direction_t = D_LEFT;
    let mut position: position_t = BEFORE;
    let mut auto_direction: bool = true_0 != 0;
    if strcmp(
        direction_str,
        b"prev\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        position = BEFORE;
    } else if strcmp(
        direction_str,
        b"next\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        position = AFTER;
    } else {
        auto_direction = false_0 != 0;
        direction = parse_direction(direction_str);
    }
    let mut current_0: *mut owindow = ::core::ptr::null_mut::<owindow>();
    current_0 = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
    while !current_0.is_null() {
        let mut ws: *mut Con = con_get_workspace((*current_0).con);
        if !(ws.is_null() || con_is_internal(ws) as ::core::ffi::c_int != 0) {
            if auto_direction {
                let o: orientation_t =
                    con_orientation((*(*current_0).con).parent as *mut Con) as orientation_t;
                direction = direction_from_orientation_position(o, position);
            }
            tree_next((*current_0).con, direction);
        }
        current_0 = (*current_0).owindows.tqe_next as *mut owindow;
    }
    (*cmd_output).needs_tree_render = true_0 != 0;
    if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                b"success\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_uchar,
                strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn cmd_focus_sibling(
    mut current_match: *mut Match,
    mut cmd_output: *mut CommandResultIR,
    mut direction_str: *const ::core::ffi::c_char,
) {
    if !(*current_match).error.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                b"Invalid match: %s\0" as *const u8 as *const ::core::ffi::c_char,
                (*current_match).error,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"success\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"error\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message as *mut ::core::ffi::c_uchar,
                    strlen(message),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message as *mut ::core::ffi::c_void);
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let mut ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
            if !(*ow).owindows.tqe_next.is_null() {
                (*(*ow).owindows.tqe_next).owindows.tqe_prev = (*ow).owindows.tqe_prev;
            } else {
                (*(*cmd_output).ctx).owindows.tqh_last = (*ow).owindows.tqe_prev;
            }
            *(*ow).owindows.tqe_prev = (*ow).owindows.tqe_next;
            free(ow as *mut ::core::ffi::c_void);
        }
        let mut ow_0: *mut owindow =
            smalloc(::core::mem::size_of::<owindow>() as size_t) as *mut owindow;
        (*ow_0).con = focused;
        (*(*cmd_output).ctx).owindows.tqh_first = ::core::ptr::null_mut::<owindow>();
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*(*cmd_output).ctx).owindows.tqh_first;
        (*ow_0).owindows.tqe_next = ::core::ptr::null_mut::<owindow>();
        (*ow_0).owindows.tqe_prev = (*(*cmd_output).ctx).owindows.tqh_last;
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0 as *mut owindow;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut current: *mut owindow = ::core::ptr::null_mut::<owindow>();
    current = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
    while !current.is_null() {
        count += 1;
        current = (*current).owindows.tqe_next as *mut owindow;
    }
    if count > 1 as ::core::ffi::c_int {
        verboselog(
            b"WARNING: Your criteria for the focus command matches %d containers, while only exactly one container can be focused at a time.\n\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            count,
        );
    }
    let direction: position_t = (if strncasecmp(
        direction_str,
        b"prev\0" as *const u8 as *const ::core::ffi::c_char,
        strlen(b"prev\0" as *const u8 as *const ::core::ffi::c_char),
    ) == 0 as ::core::ffi::c_int
    {
        BEFORE as ::core::ffi::c_int
    } else {
        AFTER as ::core::ffi::c_int
    }) as position_t;
    let mut current_0: *mut owindow = ::core::ptr::null_mut::<owindow>();
    current_0 = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
    while !current_0.is_null() {
        let mut ws: *mut Con = con_get_workspace((*current_0).con);
        if !(ws.is_null() || con_is_internal(ws) as ::core::ffi::c_int != 0) {
            let mut next: *mut Con = get_tree_next_sibling((*current_0).con, direction);
            if !next.is_null() {
                if (*next).type_0 as ::core::ffi::c_uint
                    == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    workspace_show(next);
                    con_focus(next);
                } else {
                    con_activate(next);
                }
            }
        }
        current_0 = (*current_0).owindows.tqe_next as *mut owindow;
    }
    (*cmd_output).needs_tree_render = true_0 != 0;
    if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                b"success\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_uchar,
                strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn cmd_focus_window_mode(
    mut current_match: *mut Match,
    mut cmd_output: *mut CommandResultIR,
    mut window_mode: *const ::core::ffi::c_char,
) {
    debuglog(
        b"%s:%s:%d - window_mode = %s\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/commands.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"cmd_focus_window_mode\0" as *const u8 as *const ::core::ffi::c_char,
        1366 as ::core::ffi::c_int,
        window_mode,
    );
    let mut to_floating: bool = false_0 != 0;
    if strcmp(
        window_mode,
        b"mode_toggle\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        to_floating = con_inside_floating(focused).is_null();
    } else if strcmp(
        window_mode,
        b"floating\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        to_floating = true_0 != 0;
    } else if strcmp(
        window_mode,
        b"tiling\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        to_floating = false_0 != 0;
    }
    let mut ws: *const Con = con_get_workspace(focused);
    let mut current: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut success: bool = false_0 != 0;
    current = (*ws).focus_head.tqh_first as *mut Con;
    while !current.is_null() {
        if to_floating as ::core::ffi::c_int != 0
            && (*current).type_0 as ::core::ffi::c_uint
                != CT_FLOATING_CON as ::core::ffi::c_int as ::core::ffi::c_uint
            || !to_floating
                && (*current).type_0 as ::core::ffi::c_uint
                    == CT_FLOATING_CON as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            current = (*current).focused.tqe_next as *mut Con;
        } else {
            con_activate_unblock(con_descend_focused(current));
            success = true_0 != 0;
            break;
        }
    }
    if success {
        (*cmd_output).needs_tree_render = true_0 != 0;
        if !(*cmd_output).json_gen.is_null() {
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"success\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
        }
    } else if !(*cmd_output).json_gen.is_null() {
        let mut message: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        sasprintf(
            &raw mut message,
            b"Failed to find a %s container in workspace.\0" as *const u8
                as *const ::core::ffi::c_char,
            if to_floating as ::core::ffi::c_int != 0 {
                b"floating\0" as *const u8 as *const ::core::ffi::c_char
            } else {
                b"tiling\0" as *const u8 as *const ::core::ffi::c_char
            },
        );
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                b"success\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_uchar,
                strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, false_0);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                b"error\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
                strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                message as *mut ::core::ffi::c_uchar,
                strlen(message),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
        free(message as *mut ::core::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn cmd_focus_level(
    mut current_match: *mut Match,
    mut cmd_output: *mut CommandResultIR,
    mut level: *const ::core::ffi::c_char,
) {
    debuglog(
        b"%s:%s:%d - level = %s\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/commands.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"cmd_focus_level\0" as *const u8 as *const ::core::ffi::c_char,
        1404 as ::core::ffi::c_int,
        level,
    );
    let mut success: bool = false_0 != 0;
    if strcmp(
        level,
        b"parent\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        if !focused.is_null() && !(*focused).parent.is_null() {
            if con_fullscreen_permits_focusing((*focused).parent as *mut Con) {
                success = level_up();
            } else {
                errorlog(
                    b"ERROR: 'focus parent': Currently in fullscreen, not going up\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            }
        }
    } else {
        success = level_down();
    }
    (*cmd_output).needs_tree_render = success;
    if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                b"success\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_uchar,
                strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, success as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn cmd_focus(
    mut current_match: *mut Match,
    mut cmd_output: *mut CommandResultIR,
    focus_workspace: bool,
) {
    debuglog(
        b"%s:%s:%d - current_match = %p\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/commands.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"cmd_focus\0" as *const u8 as *const ::core::ffi::c_char,
        1434 as ::core::ffi::c_int,
        current_match,
    );
    if match_is_empty(current_match) {
        errorlog(
            b"ERROR: You have to specify which window/container should be focused.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        errorlog(
            b"ERROR: Example: [class=\"urxvt\" title=\"irssi\"] focus\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        if !(*cmd_output).json_gen.is_null() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                b"You have to specify which window/container should be focused\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"success\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"error\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message as *mut ::core::ffi::c_uchar,
                    strlen(message),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message as *mut ::core::ffi::c_void);
        }
        return;
    } else if (*(*cmd_output).ctx).owindows.tqh_first.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            let mut message_0: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message_0,
                b"No window matches given criteria\0" as *const u8 as *const ::core::ffi::c_char,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"success\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"error\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message_0 as *mut ::core::ffi::c_uchar,
                    strlen(message_0),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message_0 as *mut ::core::ffi::c_void);
        }
        return;
    }
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut current: *mut owindow = ::core::ptr::null_mut::<owindow>();
    current = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
    while !current.is_null() {
        count += 1;
        current = (*current).owindows.tqe_next as *mut owindow;
    }
    if count > 1 as ::core::ffi::c_int {
        verboselog(
            b"WARNING: Your criteria for the focus command matches %d containers, while only exactly one container can be focused at a time.\n\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            count,
        );
    }
    let mut __i3_scratch: *const Con =
        workspace_get(b"__i3_scratch\0" as *const u8 as *const ::core::ffi::c_char);
    let mut current_0: *mut owindow = ::core::ptr::null_mut::<owindow>();
    current_0 = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
    while !current_0.is_null() {
        let mut ws: *mut Con = con_get_workspace((*current_0).con);
        if !ws.is_null() {
            if ws == __i3_scratch as *mut Con && !focus_workspace {
                scratchpad_show((*current_0).con);
                break;
            } else if focus_workspace {
                verboselog(
                    b"focusing workspace %p / %s - %p / %s\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    (*current_0).con,
                    (*(*current_0).con).name,
                    ws,
                    (*ws).name,
                );
                workspace_show(maybe_auto_back_and_forth_workspace(ws));
            } else {
                verboselog(
                    b"focusing %p / %s\n\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    (*current_0).con,
                    (*(*current_0).con).name,
                );
                con_activate_unblock((*current_0).con);
            }
        }
        current_0 = (*current_0).owindows.tqe_next as *mut owindow;
    }
    (*cmd_output).needs_tree_render = true_0 != 0;
    if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                b"success\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_uchar,
                strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn cmd_fullscreen(
    mut current_match: *mut Match,
    mut cmd_output: *mut CommandResultIR,
    mut action: *const ::core::ffi::c_char,
    mut fullscreen_mode: *const ::core::ffi::c_char,
) {
    let mode: fullscreen_mode_t = (if strcmp(
        fullscreen_mode,
        b"global\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        CF_GLOBAL as ::core::ffi::c_int
    } else {
        CF_OUTPUT as ::core::ffi::c_int
    }) as fullscreen_mode_t;
    debuglog(
        b"%s:%s:%d - %s fullscreen, mode = %s\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/commands.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"cmd_fullscreen\0" as *const u8 as *const ::core::ffi::c_char,
        1490 as ::core::ffi::c_int,
        action,
        fullscreen_mode,
    );
    let mut current: *mut owindow = ::core::ptr::null_mut::<owindow>();
    if !(*current_match).error.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                b"Invalid match: %s\0" as *const u8 as *const ::core::ffi::c_char,
                (*current_match).error,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"success\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"error\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message as *mut ::core::ffi::c_uchar,
                    strlen(message),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message as *mut ::core::ffi::c_void);
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let mut ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
            if !(*ow).owindows.tqe_next.is_null() {
                (*(*ow).owindows.tqe_next).owindows.tqe_prev = (*ow).owindows.tqe_prev;
            } else {
                (*(*cmd_output).ctx).owindows.tqh_last = (*ow).owindows.tqe_prev;
            }
            *(*ow).owindows.tqe_prev = (*ow).owindows.tqe_next;
            free(ow as *mut ::core::ffi::c_void);
        }
        let mut ow_0: *mut owindow =
            smalloc(::core::mem::size_of::<owindow>() as size_t) as *mut owindow;
        (*ow_0).con = focused;
        (*(*cmd_output).ctx).owindows.tqh_first = ::core::ptr::null_mut::<owindow>();
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*(*cmd_output).ctx).owindows.tqh_first;
        (*ow_0).owindows.tqe_next = ::core::ptr::null_mut::<owindow>();
        (*ow_0).owindows.tqe_prev = (*(*cmd_output).ctx).owindows.tqh_last;
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0 as *mut owindow;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    current = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
    while !current.is_null() {
        debuglog(
            b"%s:%s:%d - matching: %p / %s\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b"../src/commands.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"cmd_fullscreen\0" as *const u8 as *const ::core::ffi::c_char,
            1496 as ::core::ffi::c_int,
            (*current).con,
            (*(*current).con).name,
        );
        if strcmp(
            action,
            b"toggle\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            con_toggle_fullscreen((*current).con, mode as ::core::ffi::c_int);
        } else if strcmp(
            action,
            b"enable\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            con_enable_fullscreen((*current).con, mode);
        } else if strcmp(
            action,
            b"disable\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            con_disable_fullscreen((*current).con);
        }
        current = (*current).owindows.tqe_next as *mut owindow;
    }
    (*cmd_output).needs_tree_render = true_0 != 0;
    if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                b"success\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_uchar,
                strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn cmd_sticky(
    mut current_match: *mut Match,
    mut cmd_output: *mut CommandResultIR,
    mut action: *const ::core::ffi::c_char,
) {
    debuglog(
        b"%s:%s:%d - %s sticky on window\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/commands.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"cmd_sticky\0" as *const u8 as *const ::core::ffi::c_char,
        1516 as ::core::ffi::c_int,
        action,
    );
    if !(*current_match).error.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                b"Invalid match: %s\0" as *const u8 as *const ::core::ffi::c_char,
                (*current_match).error,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"success\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"error\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message as *mut ::core::ffi::c_uchar,
                    strlen(message),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message as *mut ::core::ffi::c_void);
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let mut ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
            if !(*ow).owindows.tqe_next.is_null() {
                (*(*ow).owindows.tqe_next).owindows.tqe_prev = (*ow).owindows.tqe_prev;
            } else {
                (*(*cmd_output).ctx).owindows.tqh_last = (*ow).owindows.tqe_prev;
            }
            *(*ow).owindows.tqe_prev = (*ow).owindows.tqe_next;
            free(ow as *mut ::core::ffi::c_void);
        }
        let mut ow_0: *mut owindow =
            smalloc(::core::mem::size_of::<owindow>() as size_t) as *mut owindow;
        (*ow_0).con = focused;
        (*(*cmd_output).ctx).owindows.tqh_first = ::core::ptr::null_mut::<owindow>();
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*(*cmd_output).ctx).owindows.tqh_first;
        (*ow_0).owindows.tqe_next = ::core::ptr::null_mut::<owindow>();
        (*ow_0).owindows.tqe_prev = (*(*cmd_output).ctx).owindows.tqh_last;
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0 as *mut owindow;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    let mut current: *mut owindow = ::core::ptr::null_mut::<owindow>();
    current = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
    while !current.is_null() {
        if (*(*current).con).window.is_null() {
            errorlog(
                b"ERROR: only containers holding a window can be made sticky, skipping con = %p\n\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                (*current).con,
            );
        } else {
            debuglog(
                b"%s:%s:%d - setting sticky for container = %p / %s\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"../src/commands.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"cmd_sticky\0" as *const u8 as *const ::core::ffi::c_char,
                1525 as ::core::ffi::c_int,
                (*current).con,
                (*(*current).con).name,
            );
            let mut sticky: bool = false_0 != 0;
            if strcmp(
                action,
                b"enable\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                sticky = true_0 != 0;
            } else if strcmp(
                action,
                b"disable\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                sticky = false_0 != 0;
            } else if strcmp(
                action,
                b"toggle\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                sticky = !(*(*current).con).sticky;
            }
            (*(*current).con).sticky = sticky;
            ewmh_update_sticky((*(*(*current).con).window).id, sticky);
        }
        current = (*current).owindows.tqe_next as *mut owindow;
    }
    output_push_sticky_windows(focused);
    ewmh_update_wm_desktop();
    (*cmd_output).needs_tree_render = true_0 != 0;
    if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                b"success\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_uchar,
                strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn cmd_move_direction(
    mut current_match: *mut Match,
    mut cmd_output: *mut CommandResultIR,
    mut direction_str: *const ::core::ffi::c_char,
    amount: ::core::ffi::c_long,
    mut mode: *const ::core::ffi::c_char,
) {
    let mut current: *mut owindow = ::core::ptr::null_mut::<owindow>();
    if !(*current_match).error.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                b"Invalid match: %s\0" as *const u8 as *const ::core::ffi::c_char,
                (*current_match).error,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"success\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"error\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message as *mut ::core::ffi::c_uchar,
                    strlen(message),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message as *mut ::core::ffi::c_void);
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let mut ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
            if !(*ow).owindows.tqe_next.is_null() {
                (*(*ow).owindows.tqe_next).owindows.tqe_prev = (*ow).owindows.tqe_prev;
            } else {
                (*(*cmd_output).ctx).owindows.tqh_last = (*ow).owindows.tqe_prev;
            }
            *(*ow).owindows.tqe_prev = (*ow).owindows.tqe_next;
            free(ow as *mut ::core::ffi::c_void);
        }
        let mut ow_0: *mut owindow =
            smalloc(::core::mem::size_of::<owindow>() as size_t) as *mut owindow;
        (*ow_0).con = focused;
        (*(*cmd_output).ctx).owindows.tqh_first = ::core::ptr::null_mut::<owindow>();
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*(*cmd_output).ctx).owindows.tqh_first;
        (*ow_0).owindows.tqe_next = ::core::ptr::null_mut::<owindow>();
        (*ow_0).owindows.tqe_prev = (*(*cmd_output).ctx).owindows.tqh_last;
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0 as *mut owindow;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    let direction: direction_t = parse_direction(direction_str) as direction_t;
    let is_ppt: bool = !mode.is_null()
        && strcmp(mode, b"ppt\0" as *const u8 as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int;
    debuglog(
        b"%s:%s:%d - moving in direction %s, %ld %s\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/commands.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"cmd_move_direction\0" as *const u8 as *const ::core::ffi::c_char,
        1562 as ::core::ffi::c_int,
        direction_str,
        amount,
        mode,
    );
    current = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
    while !current.is_null() {
        if con_is_floating((*current).con) {
            debuglog(
                b"%s:%s:%d - floating move with %ld %s\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"../src/commands.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"cmd_move_direction\0" as *const u8 as *const ::core::ffi::c_char,
                1565 as ::core::ffi::c_int,
                amount,
                mode,
            );
            let mut newrect: Rect = (*(*(*current).con).parent).rect as Rect;
            let mut output: *const Con = con_get_output((*current).con);
            match direction as ::core::ffi::c_uint {
                0 => {
                    newrect.x = (newrect.x as ::core::ffi::c_double
                        - if is_ppt as ::core::ffi::c_int != 0 {
                            (*output).rect.width as ::core::ffi::c_double
                                * (amount as ::core::ffi::c_double / 100.0f64)
                        } else {
                            amount as ::core::ffi::c_double
                        }) as uint32_t;
                }
                1 => {
                    newrect.x = (newrect.x as ::core::ffi::c_double
                        + if is_ppt as ::core::ffi::c_int != 0 {
                            (*output).rect.width as ::core::ffi::c_double
                                * (amount as ::core::ffi::c_double / 100.0f64)
                        } else {
                            amount as ::core::ffi::c_double
                        }) as uint32_t;
                }
                2 => {
                    newrect.y = (newrect.y as ::core::ffi::c_double
                        - if is_ppt as ::core::ffi::c_int != 0 {
                            (*output).rect.height as ::core::ffi::c_double
                                * (amount as ::core::ffi::c_double / 100.0f64)
                        } else {
                            amount as ::core::ffi::c_double
                        }) as uint32_t;
                }
                3 => {
                    newrect.y = (newrect.y as ::core::ffi::c_double
                        + if is_ppt as ::core::ffi::c_int != 0 {
                            (*output).rect.height as ::core::ffi::c_double
                                * (amount as ::core::ffi::c_double / 100.0f64)
                        } else {
                            amount as ::core::ffi::c_double
                        }) as uint32_t;
                }
                _ => {}
            }
            (*cmd_output).needs_tree_render =
                floating_reposition((*(*current).con).parent as *mut Con, newrect);
        } else {
            tree_move((*current).con, direction);
            (*cmd_output).needs_tree_render = true_0 != 0;
        }
        current = (*current).owindows.tqe_next as *mut owindow;
    }
    if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                b"success\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_uchar,
                strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn cmd_layout(
    mut current_match: *mut Match,
    mut cmd_output: *mut CommandResultIR,
    mut layout_str: *const ::core::ffi::c_char,
) {
    if !(*current_match).error.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                b"Invalid match: %s\0" as *const u8 as *const ::core::ffi::c_char,
                (*current_match).error,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"success\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"error\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message as *mut ::core::ffi::c_uchar,
                    strlen(message),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message as *mut ::core::ffi::c_void);
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let mut ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
            if !(*ow).owindows.tqe_next.is_null() {
                (*(*ow).owindows.tqe_next).owindows.tqe_prev = (*ow).owindows.tqe_prev;
            } else {
                (*(*cmd_output).ctx).owindows.tqh_last = (*ow).owindows.tqe_prev;
            }
            *(*ow).owindows.tqe_prev = (*ow).owindows.tqe_next;
            free(ow as *mut ::core::ffi::c_void);
        }
        let mut ow_0: *mut owindow =
            smalloc(::core::mem::size_of::<owindow>() as size_t) as *mut owindow;
        (*ow_0).con = focused;
        (*(*cmd_output).ctx).owindows.tqh_first = ::core::ptr::null_mut::<owindow>();
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*(*cmd_output).ctx).owindows.tqh_first;
        (*ow_0).owindows.tqe_next = ::core::ptr::null_mut::<owindow>();
        (*ow_0).owindows.tqe_prev = (*(*cmd_output).ctx).owindows.tqh_last;
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0 as *mut owindow;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    let mut layout: layout_t = L_DEFAULT;
    if !layout_from_name(layout_str, &raw mut layout) {
        if !(*cmd_output).json_gen.is_null() {
            let mut message_0: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message_0,
                b"Unknown layout \"%s\", this is a mismatch between code and parser spec.\0"
                    as *const u8 as *const ::core::ffi::c_char,
                layout_str,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"success\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"error\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message_0 as *mut ::core::ffi::c_uchar,
                    strlen(message_0),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message_0 as *mut ::core::ffi::c_void);
        }
        return;
    }
    debuglog(
        b"%s:%s:%d - changing layout to %s (%d)\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/commands.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"cmd_layout\0" as *const u8 as *const ::core::ffi::c_char,
        1608 as ::core::ffi::c_int,
        layout_str,
        layout as ::core::ffi::c_uint,
    );
    let mut current: *mut owindow = ::core::ptr::null_mut::<owindow>();
    current = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
    while !current.is_null() {
        if con_is_docked((*current).con) {
            errorlog(
                b"ERROR: cannot change layout of a docked container, skipping it.\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        } else {
            debuglog(
                b"%s:%s:%d - matching: %p / %s\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"../src/commands.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"cmd_layout\0" as *const u8 as *const ::core::ffi::c_char,
                1617 as ::core::ffi::c_int,
                (*current).con,
                (*(*current).con).name,
            );
            con_set_layout((*current).con, layout);
        }
        current = (*current).owindows.tqe_next as *mut owindow;
    }
    (*cmd_output).needs_tree_render = true_0 != 0;
    if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                b"success\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_uchar,
                strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn cmd_layout_toggle(
    mut current_match: *mut Match,
    mut cmd_output: *mut CommandResultIR,
    mut toggle_mode: *const ::core::ffi::c_char,
) {
    if toggle_mode.is_null() {
        toggle_mode = b"default\0" as *const u8 as *const ::core::ffi::c_char;
    }
    debuglog(
        b"%s:%s:%d - toggling layout (mode = %s)\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/commands.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"cmd_layout_toggle\0" as *const u8 as *const ::core::ffi::c_char,
        1635 as ::core::ffi::c_int,
        toggle_mode,
    );
    if match_is_empty(current_match) {
        con_toggle_layout(focused, toggle_mode);
    } else {
        let mut current: *mut owindow = ::core::ptr::null_mut::<owindow>();
        current = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
        while !current.is_null() {
            debuglog(
                b"%s:%s:%d - matching: %p / %s\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"../src/commands.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"cmd_layout_toggle\0" as *const u8 as *const ::core::ffi::c_char,
                1643 as ::core::ffi::c_int,
                (*current).con,
                (*(*current).con).name,
            );
            con_toggle_layout((*current).con, toggle_mode);
            current = (*current).owindows.tqe_next as *mut owindow;
        }
    }
    (*cmd_output).needs_tree_render = true_0 != 0;
    if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                b"success\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_uchar,
                strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn cmd_exit(
    mut current_match: *mut Match,
    mut cmd_output: *mut CommandResultIR,
) {
    verboselog(
        b"Exiting due to user command.\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    );
    exit(EXIT_SUCCESS);
}
#[no_mangle]
pub unsafe extern "C" fn cmd_reload(
    mut current_match: *mut Match,
    mut cmd_output: *mut CommandResultIR,
) {
    verboselog(
        b"reloading\n\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    kill_nagbar(config_error_nagbar_pid, false_0 != 0);
    kill_nagbar(command_error_nagbar_pid, false_0 != 0);
    command_error_nagbar_pid = -(1 as ::core::ffi::c_int) as pid_t;
    config_error_nagbar_pid = command_error_nagbar_pid;
    load_configuration(::core::ptr::null::<::core::ffi::c_char>(), C_RELOAD);
    x_set_i3_atoms();
    ipc_send_workspace_event(
        b"reload\0" as *const u8 as *const ::core::ffi::c_char,
        ::core::ptr::null_mut::<Con>(),
        ::core::ptr::null_mut::<Con>(),
    );
    let mut current: *mut Barconfig = ::core::ptr::null_mut::<Barconfig>();
    current = barconfigs.tqh_first as *mut Barconfig;
    while !current.is_null() {
        ipc_send_barconfig_update_event(current);
        current = (*current).configs.tqe_next as *mut Barconfig;
    }
    if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                b"success\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_uchar,
                strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn cmd_restart(
    mut current_match: *mut Match,
    mut cmd_output: *mut CommandResultIR,
) {
    verboselog(
        b"restarting i3\n\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    let mut exempt_fd: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    if !(*cmd_output).client.is_null() {
        exempt_fd = (*(*cmd_output).client).fd;
        verboselog(
            b"Carrying file descriptor %d across restart\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            exempt_fd,
        );
        let mut flags: ::core::ffi::c_int = 0;
        flags = fcntl(exempt_fd, F_GETFD);
        if flags < 0 as ::core::ffi::c_int
            || fcntl(exempt_fd, F_SETFD, flags & !FD_CLOEXEC) < 0 as ::core::ffi::c_int
        {
            errorlog(
                b"ERROR: Could not disable FD_CLOEXEC on fd %d\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                exempt_fd,
            );
        }
        let mut fdstr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        sasprintf(
            &raw mut fdstr,
            b"%d\0" as *const u8 as *const ::core::ffi::c_char,
            exempt_fd,
        );
        setenv(
            b"_I3_RESTART_FD\0" as *const u8 as *const ::core::ffi::c_char,
            fdstr,
            1 as ::core::ffi::c_int,
        );
    }
    ipc_shutdown(SHUTDOWN_REASON_RESTART, exempt_fd);
    unlink(config.ipc_socket_path);
    if !current_log_stream_socket_path.is_null() {
        unlink(current_log_stream_socket_path);
    }
    purge_zerobyte_logfile();
    i3_restart(false_0 != 0);
    if 0 as ::core::ffi::c_int != 0 {
    } else {
        __assert_fail(
            b"false\0" as *const u8 as *const ::core::ffi::c_char,
            b"../src/commands.c\0" as *const u8 as *const ::core::ffi::c_char,
            1723 as ::core::ffi::c_uint,
            b"void cmd_restart(Match *, struct CommandResultIR *)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn cmd_open(
    mut current_match: *mut Match,
    mut cmd_output: *mut CommandResultIR,
) {
    verboselog(
        b"opening new container\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    );
    let mut con: *mut Con = tree_open_con(
        ::core::ptr::null_mut::<Con>(),
        ::core::ptr::null_mut::<i3Window>(),
    );
    (*con).layout = L_SPLITH;
    con_activate(con);
    if !(*cmd_output).json_gen.is_null() {
        yajl_gen_map_open((*cmd_output).json_gen);
    } else {
    };
    if !(*cmd_output).json_gen.is_null() {
        yajl_gen_string(
            (*cmd_output).json_gen,
            b"success\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
            strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
        );
    } else {
    };
    if !(*cmd_output).json_gen.is_null() {
        yajl_gen_bool((*cmd_output).json_gen, true_0);
    } else {
    };
    if !(*cmd_output).json_gen.is_null() {
        yajl_gen_string(
            (*cmd_output).json_gen,
            b"id\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
            strlen(b"id\0" as *const u8 as *const ::core::ffi::c_char),
        );
    } else {
    };
    if !(*cmd_output).json_gen.is_null() {
        yajl_gen_integer(
            (*cmd_output).json_gen,
            con as uintptr_t as ::core::ffi::c_longlong,
        );
    } else {
    };
    if !(*cmd_output).json_gen.is_null() {
        yajl_gen_map_close((*cmd_output).json_gen);
    } else {
    };
    (*cmd_output).needs_tree_render = true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn cmd_focus_output(
    mut current_match: *mut Match,
    mut cmd_output: *mut CommandResultIR,
    mut name: *const ::core::ffi::c_char,
) {
    if !name.is_null() {
        user_output_names_add(&raw mut names_global, name);
        return;
    }
    if names_global.tqh_first.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                b"At least one output must be specified\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"success\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"error\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message as *mut ::core::ffi::c_uchar,
                    strlen(message),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message as *mut ::core::ffi::c_void);
        }
        return;
    }
    if !(*current_match).error.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            let mut message_0: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message_0,
                b"Invalid match: %s\0" as *const u8 as *const ::core::ffi::c_char,
                (*current_match).error,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"success\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"error\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message_0 as *mut ::core::ffi::c_uchar,
                    strlen(message_0),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message_0 as *mut ::core::ffi::c_void);
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let mut ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
            if !(*ow).owindows.tqe_next.is_null() {
                (*(*ow).owindows.tqe_next).owindows.tqe_prev = (*ow).owindows.tqe_prev;
            } else {
                (*(*cmd_output).ctx).owindows.tqh_last = (*ow).owindows.tqe_prev;
            }
            *(*ow).owindows.tqe_prev = (*ow).owindows.tqe_next;
            free(ow as *mut ::core::ffi::c_void);
        }
        let mut ow_0: *mut owindow =
            smalloc(::core::mem::size_of::<owindow>() as size_t) as *mut owindow;
        (*ow_0).con = focused;
        (*(*cmd_output).ctx).owindows.tqh_first = ::core::ptr::null_mut::<owindow>();
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*(*cmd_output).ctx).owindows.tqh_first;
        (*ow_0).owindows.tqe_next = ::core::ptr::null_mut::<owindow>();
        (*ow_0).owindows.tqe_prev = (*(*cmd_output).ctx).owindows.tqh_last;
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0 as *mut owindow;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    if (*(*cmd_output).ctx).owindows.tqh_first.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"success\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
        }
        return;
    }
    let mut current: *mut owindow = ::core::ptr::null_mut::<owindow>();
    let mut con: *mut Con = focused;
    current = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
    while !current.is_null() {
        if !con_is_internal(con_get_workspace((*current).con)) {
            con = (*current).con;
            break;
        } else {
            current = (*current).owindows.tqe_next as *mut owindow;
        }
    }
    let mut current_output: *mut Output = get_output_for_con(con);
    let mut target_output: *const Output =
        user_output_names_find_next(&raw mut names_global, current_output);
    user_output_names_free(&raw mut names_global);
    let mut success: bool = false_0 != 0;
    if !target_output.is_null() {
        success = true_0 != 0;
        let mut ws: *mut Con = ::core::ptr::null_mut::<Con>();
        let mut child: *mut Con = -(1 as ::core::ffi::c_int) as *mut Con;
        while child == -(1 as ::core::ffi::c_int) as *mut Con && {
            child = ::core::ptr::null_mut::<Con>();
            true_0 != 0
        } {
            child = (*output_get_content((*target_output).con))
                .nodes_head
                .tqh_first as *mut Con;
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
            if !(*cmd_output).json_gen.is_null() {
                let mut message_1: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                sasprintf(
                    &raw mut message_1,
                    b"BUG: No workspace found on output.\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_map_open((*cmd_output).json_gen);
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_string(
                        (*cmd_output).json_gen,
                        b"success\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_uchar,
                        strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                    );
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_bool((*cmd_output).json_gen, false_0);
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_string(
                        (*cmd_output).json_gen,
                        b"error\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_uchar,
                        strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
                    );
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_string(
                        (*cmd_output).json_gen,
                        message_1 as *mut ::core::ffi::c_uchar,
                        strlen(message_1),
                    );
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_map_close((*cmd_output).json_gen);
                } else {
                };
                free(message_1 as *mut ::core::ffi::c_void);
            }
            return;
        }
        workspace_show(ws);
    }
    (*cmd_output).needs_tree_render = success;
    if success {
        if !(*cmd_output).json_gen.is_null() {
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"success\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
        }
    } else if !(*cmd_output).json_gen.is_null() {
        let mut message_2: *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<::core::ffi::c_char>();
        sasprintf(
            &raw mut message_2,
            b"No output matched\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                b"success\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_uchar,
                strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, false_0);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                b"error\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
                strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                message_2 as *mut ::core::ffi::c_uchar,
                strlen(message_2),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
        free(message_2 as *mut ::core::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn cmd_move_window_to_position(
    mut current_match: *mut Match,
    mut cmd_output: *mut CommandResultIR,
    x: ::core::ffi::c_long,
    mut mode_x: *const ::core::ffi::c_char,
    y: ::core::ffi::c_long,
    mut mode_y: *const ::core::ffi::c_char,
) {
    let mut has_error: bool = false_0 != 0;
    let mut current: *mut owindow = ::core::ptr::null_mut::<owindow>();
    if !(*current_match).error.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                b"Invalid match: %s\0" as *const u8 as *const ::core::ffi::c_char,
                (*current_match).error,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"success\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"error\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message as *mut ::core::ffi::c_uchar,
                    strlen(message),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message as *mut ::core::ffi::c_void);
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let mut ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
            if !(*ow).owindows.tqe_next.is_null() {
                (*(*ow).owindows.tqe_next).owindows.tqe_prev = (*ow).owindows.tqe_prev;
            } else {
                (*(*cmd_output).ctx).owindows.tqh_last = (*ow).owindows.tqe_prev;
            }
            *(*ow).owindows.tqe_prev = (*ow).owindows.tqe_next;
            free(ow as *mut ::core::ffi::c_void);
        }
        let mut ow_0: *mut owindow =
            smalloc(::core::mem::size_of::<owindow>() as size_t) as *mut owindow;
        (*ow_0).con = focused;
        (*(*cmd_output).ctx).owindows.tqh_first = ::core::ptr::null_mut::<owindow>();
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*(*cmd_output).ctx).owindows.tqh_first;
        (*ow_0).owindows.tqe_next = ::core::ptr::null_mut::<owindow>();
        (*ow_0).owindows.tqe_prev = (*(*cmd_output).ctx).owindows.tqh_last;
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0 as *mut owindow;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    current = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
    while !current.is_null() {
        if !con_is_floating((*current).con) {
            errorlog(
                b"ERROR: Cannot change position. The window/container is not floating\n\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
            if !has_error {
                if !(*cmd_output).json_gen.is_null() {
                    let mut message_0: *mut ::core::ffi::c_char =
                        ::core::ptr::null_mut::<::core::ffi::c_char>();
                    sasprintf(
                        &raw mut message_0,
                        b"Cannot change position of a window/container because it is not floating.\0"
                            as *const u8 as *const ::core::ffi::c_char,
                    );
                    if !(*cmd_output).json_gen.is_null() {
                        yajl_gen_map_open((*cmd_output).json_gen);
                    } else {
                    };
                    if !(*cmd_output).json_gen.is_null() {
                        yajl_gen_string(
                            (*cmd_output).json_gen,
                            b"success\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_uchar,
                            strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                        );
                    } else {
                    };
                    if !(*cmd_output).json_gen.is_null() {
                        yajl_gen_bool((*cmd_output).json_gen, false_0);
                    } else {
                    };
                    if !(*cmd_output).json_gen.is_null() {
                        yajl_gen_string(
                            (*cmd_output).json_gen,
                            b"error\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_uchar,
                            strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
                        );
                    } else {
                    };
                    if !(*cmd_output).json_gen.is_null() {
                        yajl_gen_string(
                            (*cmd_output).json_gen,
                            message_0 as *mut ::core::ffi::c_uchar,
                            strlen(message_0),
                        );
                    } else {
                    };
                    if !(*cmd_output).json_gen.is_null() {
                        yajl_gen_map_close((*cmd_output).json_gen);
                    } else {
                    };
                    free(message_0 as *mut ::core::ffi::c_void);
                }
                has_error = true_0 != 0;
            }
        } else {
            let mut newrect: Rect = (*(*(*current).con).parent).rect as Rect;
            let mut output: *const Con = con_get_output((*current).con);
            newrect.x = (if !mode_x.is_null()
                && strcmp(mode_x, b"ppt\0" as *const u8 as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
            {
                (*output).rect.width as ::core::ffi::c_double
                    * (x as ::core::ffi::c_double / 100.0f64)
            } else {
                x as ::core::ffi::c_double
            }) as uint32_t;
            newrect.y = (if !mode_y.is_null()
                && strcmp(mode_y, b"ppt\0" as *const u8 as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
            {
                (*output).rect.height as ::core::ffi::c_double
                    * (y as ::core::ffi::c_double / 100.0f64)
            } else {
                y as ::core::ffi::c_double
            }) as uint32_t;
            debuglog(
                b"%s:%s:%d - moving to position %d %s %d %s\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"../src/commands.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"cmd_move_window_to_position\0" as *const u8 as *const ::core::ffi::c_char,
                1835 as ::core::ffi::c_int,
                newrect.x,
                mode_x,
                newrect.y,
                mode_y,
            );
            if !floating_reposition((*(*current).con).parent as *mut Con, newrect) {
                if !(*cmd_output).json_gen.is_null() {
                    let mut message_1: *mut ::core::ffi::c_char =
                        ::core::ptr::null_mut::<::core::ffi::c_char>();
                    sasprintf(
                        &raw mut message_1,
                        b"Cannot move window/container out of bounds.\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                    if !(*cmd_output).json_gen.is_null() {
                        yajl_gen_map_open((*cmd_output).json_gen);
                    } else {
                    };
                    if !(*cmd_output).json_gen.is_null() {
                        yajl_gen_string(
                            (*cmd_output).json_gen,
                            b"success\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_uchar,
                            strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                        );
                    } else {
                    };
                    if !(*cmd_output).json_gen.is_null() {
                        yajl_gen_bool((*cmd_output).json_gen, false_0);
                    } else {
                    };
                    if !(*cmd_output).json_gen.is_null() {
                        yajl_gen_string(
                            (*cmd_output).json_gen,
                            b"error\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_uchar,
                            strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
                        );
                    } else {
                    };
                    if !(*cmd_output).json_gen.is_null() {
                        yajl_gen_string(
                            (*cmd_output).json_gen,
                            message_1 as *mut ::core::ffi::c_uchar,
                            strlen(message_1),
                        );
                    } else {
                    };
                    if !(*cmd_output).json_gen.is_null() {
                        yajl_gen_map_close((*cmd_output).json_gen);
                    } else {
                    };
                    free(message_1 as *mut ::core::ffi::c_void);
                }
                has_error = true_0 != 0;
            }
        }
        current = (*current).owindows.tqe_next as *mut owindow;
    }
    if !has_error {
        if !(*cmd_output).json_gen.is_null() {
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"success\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn cmd_move_window_to_center(
    mut current_match: *mut Match,
    mut cmd_output: *mut CommandResultIR,
    mut method: *const ::core::ffi::c_char,
) {
    let mut has_error: bool = false_0 != 0;
    if !(*current_match).error.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                b"Invalid match: %s\0" as *const u8 as *const ::core::ffi::c_char,
                (*current_match).error,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"success\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"error\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message as *mut ::core::ffi::c_uchar,
                    strlen(message),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message as *mut ::core::ffi::c_void);
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let mut ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
            if !(*ow).owindows.tqe_next.is_null() {
                (*(*ow).owindows.tqe_next).owindows.tqe_prev = (*ow).owindows.tqe_prev;
            } else {
                (*(*cmd_output).ctx).owindows.tqh_last = (*ow).owindows.tqe_prev;
            }
            *(*ow).owindows.tqe_prev = (*ow).owindows.tqe_next;
            free(ow as *mut ::core::ffi::c_void);
        }
        let mut ow_0: *mut owindow =
            smalloc(::core::mem::size_of::<owindow>() as size_t) as *mut owindow;
        (*ow_0).con = focused;
        (*(*cmd_output).ctx).owindows.tqh_first = ::core::ptr::null_mut::<owindow>();
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*(*cmd_output).ctx).owindows.tqh_first;
        (*ow_0).owindows.tqe_next = ::core::ptr::null_mut::<owindow>();
        (*ow_0).owindows.tqe_prev = (*(*cmd_output).ctx).owindows.tqh_last;
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0 as *mut owindow;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    let mut current: *mut owindow = ::core::ptr::null_mut::<owindow>();
    current = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
    while !current.is_null() {
        let mut floating_con: *mut Con = con_inside_floating((*current).con);
        if floating_con.is_null() {
            errorlog(
                b"ERROR: con %p / %s is not floating, cannot move it to the center.\n\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                (*current).con,
                (*(*current).con).name,
            );
            if !has_error {
                if !(*cmd_output).json_gen.is_null() {
                    let mut message_0: *mut ::core::ffi::c_char =
                        ::core::ptr::null_mut::<::core::ffi::c_char>();
                    sasprintf(
                        &raw mut message_0,
                        b"Cannot change position of a window/container because it is not floating.\0"
                            as *const u8 as *const ::core::ffi::c_char,
                    );
                    if !(*cmd_output).json_gen.is_null() {
                        yajl_gen_map_open((*cmd_output).json_gen);
                    } else {
                    };
                    if !(*cmd_output).json_gen.is_null() {
                        yajl_gen_string(
                            (*cmd_output).json_gen,
                            b"success\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_uchar,
                            strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                        );
                    } else {
                    };
                    if !(*cmd_output).json_gen.is_null() {
                        yajl_gen_bool((*cmd_output).json_gen, false_0);
                    } else {
                    };
                    if !(*cmd_output).json_gen.is_null() {
                        yajl_gen_string(
                            (*cmd_output).json_gen,
                            b"error\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_uchar,
                            strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
                        );
                    } else {
                    };
                    if !(*cmd_output).json_gen.is_null() {
                        yajl_gen_string(
                            (*cmd_output).json_gen,
                            message_0 as *mut ::core::ffi::c_uchar,
                            strlen(message_0),
                        );
                    } else {
                    };
                    if !(*cmd_output).json_gen.is_null() {
                        yajl_gen_map_close((*cmd_output).json_gen);
                    } else {
                    };
                    free(message_0 as *mut ::core::ffi::c_void);
                }
                has_error = true_0 != 0;
            }
        } else {
            if strcmp(
                method,
                b"absolute\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                debuglog(
                    b"%s:%s:%d - moving to absolute center\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    b"../src/commands.c\0" as *const u8 as *const ::core::ffi::c_char,
                    b"cmd_move_window_to_center\0" as *const u8 as *const ::core::ffi::c_char,
                    1872 as ::core::ffi::c_int,
                );
                floating_center(floating_con, (*croot).rect as Rect);
                floating_maybe_reassign_ws(floating_con);
                (*cmd_output).needs_tree_render = true_0 != 0;
            }
            if strcmp(
                method,
                b"position\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                debuglog(
                    b"%s:%s:%d - moving to center\n\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    b"../src/commands.c\0" as *const u8 as *const ::core::ffi::c_char,
                    b"cmd_move_window_to_center\0" as *const u8 as *const ::core::ffi::c_char,
                    1880 as ::core::ffi::c_int,
                );
                floating_center(
                    floating_con,
                    (*con_get_workspace(floating_con)).rect as Rect,
                );
                (*cmd_output).needs_tree_render = true_0 != 0;
            }
        }
        current = (*current).owindows.tqe_next as *mut owindow;
    }
    if !has_error {
        if !(*cmd_output).json_gen.is_null() {
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"success\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn cmd_move_window_to_mouse(
    mut current_match: *mut Match,
    mut cmd_output: *mut CommandResultIR,
) {
    if !(*current_match).error.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                b"Invalid match: %s\0" as *const u8 as *const ::core::ffi::c_char,
                (*current_match).error,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"success\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"error\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message as *mut ::core::ffi::c_uchar,
                    strlen(message),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message as *mut ::core::ffi::c_void);
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let mut ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
            if !(*ow).owindows.tqe_next.is_null() {
                (*(*ow).owindows.tqe_next).owindows.tqe_prev = (*ow).owindows.tqe_prev;
            } else {
                (*(*cmd_output).ctx).owindows.tqh_last = (*ow).owindows.tqe_prev;
            }
            *(*ow).owindows.tqe_prev = (*ow).owindows.tqe_next;
            free(ow as *mut ::core::ffi::c_void);
        }
        let mut ow_0: *mut owindow =
            smalloc(::core::mem::size_of::<owindow>() as size_t) as *mut owindow;
        (*ow_0).con = focused;
        (*(*cmd_output).ctx).owindows.tqh_first = ::core::ptr::null_mut::<owindow>();
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*(*cmd_output).ctx).owindows.tqh_first;
        (*ow_0).owindows.tqe_next = ::core::ptr::null_mut::<owindow>();
        (*ow_0).owindows.tqe_prev = (*(*cmd_output).ctx).owindows.tqh_last;
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0 as *mut owindow;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    let mut current: *mut owindow = ::core::ptr::null_mut::<owindow>();
    current = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
    while !current.is_null() {
        let mut floating_con: *mut Con = con_inside_floating((*current).con);
        if floating_con.is_null() {
            debuglog(
                b"%s:%s:%d - con %p / %s is not floating, cannot move it to the mouse position.\n\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"../src/commands.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"cmd_move_window_to_mouse\0" as *const u8 as *const ::core::ffi::c_char,
                1905 as ::core::ffi::c_int,
                (*current).con,
                (*(*current).con).name,
            );
        } else {
            debuglog(
                b"%s:%s:%d - moving floating container %p / %s to cursor position\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"../src/commands.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"cmd_move_window_to_mouse\0" as *const u8 as *const ::core::ffi::c_char,
                1909 as ::core::ffi::c_int,
                floating_con,
                (*floating_con).name,
            );
            floating_move_to_pointer(floating_con);
        }
        current = (*current).owindows.tqe_next as *mut owindow;
    }
    (*cmd_output).needs_tree_render = true_0 != 0;
    if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                b"success\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_uchar,
                strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn cmd_move_scratchpad(
    mut current_match: *mut Match,
    mut cmd_output: *mut CommandResultIR,
) {
    debuglog(
        b"%s:%s:%d - should move window to scratchpad\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/commands.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"cmd_move_scratchpad\0" as *const u8 as *const ::core::ffi::c_char,
        1922 as ::core::ffi::c_int,
    );
    let mut current: *mut owindow = ::core::ptr::null_mut::<owindow>();
    if !(*current_match).error.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                b"Invalid match: %s\0" as *const u8 as *const ::core::ffi::c_char,
                (*current_match).error,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"success\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"error\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message as *mut ::core::ffi::c_uchar,
                    strlen(message),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message as *mut ::core::ffi::c_void);
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let mut ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
            if !(*ow).owindows.tqe_next.is_null() {
                (*(*ow).owindows.tqe_next).owindows.tqe_prev = (*ow).owindows.tqe_prev;
            } else {
                (*(*cmd_output).ctx).owindows.tqh_last = (*ow).owindows.tqe_prev;
            }
            *(*ow).owindows.tqe_prev = (*ow).owindows.tqe_next;
            free(ow as *mut ::core::ffi::c_void);
        }
        let mut ow_0: *mut owindow =
            smalloc(::core::mem::size_of::<owindow>() as size_t) as *mut owindow;
        (*ow_0).con = focused;
        (*(*cmd_output).ctx).owindows.tqh_first = ::core::ptr::null_mut::<owindow>();
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*(*cmd_output).ctx).owindows.tqh_first;
        (*ow_0).owindows.tqe_next = ::core::ptr::null_mut::<owindow>();
        (*ow_0).owindows.tqe_prev = (*(*cmd_output).ctx).owindows.tqh_last;
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0 as *mut owindow;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    current = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
    while !current.is_null() {
        debuglog(
            b"%s:%s:%d - matching: %p / %s\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b"../src/commands.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"cmd_move_scratchpad\0" as *const u8 as *const ::core::ffi::c_char,
            1928 as ::core::ffi::c_int,
            (*current).con,
            (*(*current).con).name,
        );
        scratchpad_move((*current).con);
        current = (*current).owindows.tqe_next as *mut owindow;
    }
    (*cmd_output).needs_tree_render = true_0 != 0;
    if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                b"success\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_uchar,
                strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn cmd_scratchpad_show(
    mut current_match: *mut Match,
    mut cmd_output: *mut CommandResultIR,
) {
    debuglog(
        b"%s:%s:%d - should show scratchpad window\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/commands.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"cmd_scratchpad_show\0" as *const u8 as *const ::core::ffi::c_char,
        1942 as ::core::ffi::c_int,
    );
    let mut result: bool = false_0 != 0;
    if match_is_empty(current_match) {
        result = scratchpad_show(::core::ptr::null_mut::<Con>());
    } else {
        let mut current: *mut owindow = ::core::ptr::null_mut::<owindow>();
        current = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
        while !current.is_null() {
            debuglog(
                b"%s:%s:%d - matching: %p / %s\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"../src/commands.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"cmd_scratchpad_show\0" as *const u8 as *const ::core::ffi::c_char,
                1950 as ::core::ffi::c_int,
                (*current).con,
                (*(*current).con).name,
            );
            result = (result as ::core::ffi::c_int
                | scratchpad_show((*current).con) as ::core::ffi::c_int)
                != 0;
            current = (*current).owindows.tqe_next as *mut owindow;
        }
    }
    (*cmd_output).needs_tree_render = true_0 != 0;
    if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                b"success\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_uchar,
                strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, result as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn cmd_swap(
    mut current_match: *mut Match,
    mut cmd_output: *mut CommandResultIR,
    mut mode: *const ::core::ffi::c_char,
    mut arg: *const ::core::ffi::c_char,
) {
    if !(*current_match).error.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                b"Invalid match: %s\0" as *const u8 as *const ::core::ffi::c_char,
                (*current_match).error,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"success\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"error\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message as *mut ::core::ffi::c_uchar,
                    strlen(message),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message as *mut ::core::ffi::c_void);
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let mut ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
            if !(*ow).owindows.tqe_next.is_null() {
                (*(*ow).owindows.tqe_next).owindows.tqe_prev = (*ow).owindows.tqe_prev;
            } else {
                (*(*cmd_output).ctx).owindows.tqh_last = (*ow).owindows.tqe_prev;
            }
            *(*ow).owindows.tqe_prev = (*ow).owindows.tqe_next;
            free(ow as *mut ::core::ffi::c_void);
        }
        let mut ow_0: *mut owindow =
            smalloc(::core::mem::size_of::<owindow>() as size_t) as *mut owindow;
        (*ow_0).con = focused;
        (*(*cmd_output).ctx).owindows.tqh_first = ::core::ptr::null_mut::<owindow>();
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*(*cmd_output).ctx).owindows.tqh_first;
        (*ow_0).owindows.tqe_next = ::core::ptr::null_mut::<owindow>();
        (*ow_0).owindows.tqe_prev = (*(*cmd_output).ctx).owindows.tqh_last;
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0 as *mut owindow;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    let mut match_0: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
    if match_0.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            let mut message_0: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message_0,
                b"No match found for swapping.\0" as *const u8 as *const ::core::ffi::c_char,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"success\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"error\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message_0 as *mut ::core::ffi::c_uchar,
                    strlen(message_0),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message_0 as *mut ::core::ffi::c_void);
        }
        return;
    }
    if (*match_0).con.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            let mut message_1: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message_1,
                b"Match %p has no container.\0" as *const u8 as *const ::core::ffi::c_char,
                match_0,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"success\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"error\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message_1 as *mut ::core::ffi::c_uchar,
                    strlen(message_1),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message_1 as *mut ::core::ffi::c_void);
        }
        return;
    }
    let mut con: *mut Con = ::core::ptr::null_mut::<Con>();
    if strcmp(mode, b"id\0" as *const u8 as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int {
        let mut target: ::core::ffi::c_long = 0;
        if !parse_long(arg, &raw mut target, 0 as ::core::ffi::c_int) {
            if !(*cmd_output).json_gen.is_null() {
                let mut message_2: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                sasprintf(
                    &raw mut message_2,
                    b"Failed to parse %s into a window id.\0" as *const u8
                        as *const ::core::ffi::c_char,
                    arg,
                );
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_map_open((*cmd_output).json_gen);
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_string(
                        (*cmd_output).json_gen,
                        b"success\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_uchar,
                        strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                    );
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_bool((*cmd_output).json_gen, false_0);
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_string(
                        (*cmd_output).json_gen,
                        b"error\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_uchar,
                        strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
                    );
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_string(
                        (*cmd_output).json_gen,
                        message_2 as *mut ::core::ffi::c_uchar,
                        strlen(message_2),
                    );
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_map_close((*cmd_output).json_gen);
                } else {
                };
                free(message_2 as *mut ::core::ffi::c_void);
            }
            return;
        }
        con = con_by_window_id(target as xcb_window_t);
    } else if strcmp(mode, b"con_id\0" as *const u8 as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        let mut target_0: ::core::ffi::c_long = 0;
        if !parse_long(arg, &raw mut target_0, 0 as ::core::ffi::c_int) {
            if !(*cmd_output).json_gen.is_null() {
                let mut message_3: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                sasprintf(
                    &raw mut message_3,
                    b"Failed to parse %s into a container id.\0" as *const u8
                        as *const ::core::ffi::c_char,
                    arg,
                );
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_map_open((*cmd_output).json_gen);
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_string(
                        (*cmd_output).json_gen,
                        b"success\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_uchar,
                        strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                    );
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_bool((*cmd_output).json_gen, false_0);
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_string(
                        (*cmd_output).json_gen,
                        b"error\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_uchar,
                        strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
                    );
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_string(
                        (*cmd_output).json_gen,
                        message_3 as *mut ::core::ffi::c_uchar,
                        strlen(message_3),
                    );
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_map_close((*cmd_output).json_gen);
                } else {
                };
                free(message_3 as *mut ::core::ffi::c_void);
            }
            return;
        }
        con = con_by_con_id(target_0);
    } else if strcmp(mode, b"mark\0" as *const u8 as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        con = con_by_mark(arg);
    } else {
        if !(*cmd_output).json_gen.is_null() {
            let mut message_4: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message_4,
                b"Unhandled swap mode \"%s\". This is a bug.\0" as *const u8
                    as *const ::core::ffi::c_char,
                mode,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"success\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"error\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message_4 as *mut ::core::ffi::c_uchar,
                    strlen(message_4),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message_4 as *mut ::core::ffi::c_void);
        }
        return;
    }
    if con.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            let mut message_5: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message_5,
                b"Could not find container for %s = %s\0" as *const u8
                    as *const ::core::ffi::c_char,
                mode,
                arg,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"success\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"error\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message_5 as *mut ::core::ffi::c_uchar,
                    strlen(message_5),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message_5 as *mut ::core::ffi::c_void);
        }
        return;
    }
    if match_0 != *(*((*(*cmd_output).ctx).owindows.tqh_last as *mut owindows_head)).tqh_last {
        verboselog(
            b"More than one container matched the swap command, only using the first one.\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    debuglog(
        b"%s:%s:%d - Swapping %p with %p.\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/commands.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"cmd_swap\0" as *const u8 as *const ::core::ffi::c_char,
        2010 as ::core::ffi::c_int,
        (*match_0).con,
        con,
    );
    let result: bool = con_swap((*match_0).con, con) as bool;
    (*cmd_output).needs_tree_render = true_0 != 0;
    if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                b"success\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_uchar,
                strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, result as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn cmd_title_format(
    mut current_match: *mut Match,
    mut cmd_output: *mut CommandResultIR,
    mut format: *const ::core::ffi::c_char,
) {
    debuglog(
        b"%s:%s:%d - setting title_format to \"%s\"\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/commands.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"cmd_title_format\0" as *const u8 as *const ::core::ffi::c_char,
        2023 as ::core::ffi::c_int,
        format,
    );
    if !(*current_match).error.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                b"Invalid match: %s\0" as *const u8 as *const ::core::ffi::c_char,
                (*current_match).error,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"success\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"error\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message as *mut ::core::ffi::c_uchar,
                    strlen(message),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message as *mut ::core::ffi::c_void);
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let mut ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
            if !(*ow).owindows.tqe_next.is_null() {
                (*(*ow).owindows.tqe_next).owindows.tqe_prev = (*ow).owindows.tqe_prev;
            } else {
                (*(*cmd_output).ctx).owindows.tqh_last = (*ow).owindows.tqe_prev;
            }
            *(*ow).owindows.tqe_prev = (*ow).owindows.tqe_next;
            free(ow as *mut ::core::ffi::c_void);
        }
        let mut ow_0: *mut owindow =
            smalloc(::core::mem::size_of::<owindow>() as size_t) as *mut owindow;
        (*ow_0).con = focused;
        (*(*cmd_output).ctx).owindows.tqh_first = ::core::ptr::null_mut::<owindow>();
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*(*cmd_output).ctx).owindows.tqh_first;
        (*ow_0).owindows.tqe_next = ::core::ptr::null_mut::<owindow>();
        (*ow_0).owindows.tqe_prev = (*(*cmd_output).ctx).owindows.tqh_last;
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0 as *mut owindow;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    let mut current: *mut owindow = ::core::ptr::null_mut::<owindow>();
    current = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
    while !current.is_null() {
        debuglog(
            b"%s:%s:%d - setting title_format for %p / %s\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/commands.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"cmd_title_format\0" as *const u8 as *const ::core::ffi::c_char,
            2028 as ::core::ffi::c_int,
            (*current).con,
            (*(*current).con).name,
        );
        free((*(*current).con).title_format as *mut ::core::ffi::c_void);
        (*(*current).con).title_format = ::core::ptr::null_mut::<::core::ffi::c_char>();
        if strcasecmp(
            format,
            b"%title\0" as *const u8 as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
        {
            (*(*current).con).title_format = sstrdup(format);
            if !(*(*current).con).window.is_null() {
                let mut formatted_title: *mut i3String = con_parse_title_format((*current).con);
                ewmh_update_visible_name(
                    (*(*(*current).con).window).id,
                    i3string_as_utf8(formatted_title),
                );
                i3string_free(formatted_title);
                formatted_title = ::core::ptr::null_mut::<i3String>();
            }
        } else if !(*(*current).con).window.is_null() {
            ewmh_update_visible_name(
                (*(*(*current).con).window).id,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
        if !(*(*current).con).window.is_null() {
            (*(*(*current).con).window).name_x_changed = true_0 != 0;
        } else {
            free((*(*current).con).deco_render_params as *mut ::core::ffi::c_void);
            (*(*current).con).deco_render_params = ::core::ptr::null_mut::<deco_render_params>();
        }
        current = (*current).owindows.tqe_next as *mut owindow;
    }
    (*cmd_output).needs_tree_render = true_0 != 0;
    if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                b"success\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_uchar,
                strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn cmd_title_window_icon(
    mut current_match: *mut Match,
    mut cmd_output: *mut CommandResultIR,
    mut enable: *const ::core::ffi::c_char,
    mut padding: ::core::ffi::c_int,
) {
    let mut is_toggle: bool = false_0 != 0;
    if !enable.is_null() {
        if strcmp(
            enable,
            b"toggle\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            is_toggle = true_0 != 0;
        } else if !boolstr(enable) {
            padding = -(1 as ::core::ffi::c_int);
        }
    }
    debuglog(
        b"%s:%s:%d - setting window_icon=%d\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/commands.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"cmd_title_window_icon\0" as *const u8 as *const ::core::ffi::c_char,
        2074 as ::core::ffi::c_int,
        padding,
    );
    if !(*current_match).error.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                b"Invalid match: %s\0" as *const u8 as *const ::core::ffi::c_char,
                (*current_match).error,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"success\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"error\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message as *mut ::core::ffi::c_uchar,
                    strlen(message),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message as *mut ::core::ffi::c_void);
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let mut ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
            if !(*ow).owindows.tqe_next.is_null() {
                (*(*ow).owindows.tqe_next).owindows.tqe_prev = (*ow).owindows.tqe_prev;
            } else {
                (*(*cmd_output).ctx).owindows.tqh_last = (*ow).owindows.tqe_prev;
            }
            *(*ow).owindows.tqe_prev = (*ow).owindows.tqe_next;
            free(ow as *mut ::core::ffi::c_void);
        }
        let mut ow_0: *mut owindow =
            smalloc(::core::mem::size_of::<owindow>() as size_t) as *mut owindow;
        (*ow_0).con = focused;
        (*(*cmd_output).ctx).owindows.tqh_first = ::core::ptr::null_mut::<owindow>();
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*(*cmd_output).ctx).owindows.tqh_first;
        (*ow_0).owindows.tqe_next = ::core::ptr::null_mut::<owindow>();
        (*ow_0).owindows.tqe_prev = (*(*cmd_output).ctx).owindows.tqh_last;
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0 as *mut owindow;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    let mut current: *mut owindow = ::core::ptr::null_mut::<owindow>();
    current = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
    while !current.is_null() {
        if is_toggle {
            let current_padding: ::core::ffi::c_int = (*(*current).con).window_icon_padding;
            if padding > 0 as ::core::ffi::c_int {
                if current_padding < 0 as ::core::ffi::c_int {
                    (*(*current).con).window_icon_padding = padding;
                } else {
                    (*(*current).con).window_icon_padding = -(padding + 1 as ::core::ffi::c_int);
                }
            } else {
                (*(*current).con).window_icon_padding =
                    -(current_padding + 1 as ::core::ffi::c_int);
            }
        } else {
            (*(*current).con).window_icon_padding = padding;
        }
        debuglog(
            b"%s:%s:%d - Set window_icon for %p / %s to %d\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/commands.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"cmd_title_window_icon\0" as *const u8 as *const ::core::ffi::c_char,
            2095 as ::core::ffi::c_int,
            (*current).con,
            (*(*current).con).name,
            (*(*current).con).window_icon_padding,
        );
        if !(*(*current).con).window.is_null() {
            (*(*(*current).con).window).name_x_changed = true_0 != 0;
        } else {
            free((*(*current).con).deco_render_params as *mut ::core::ffi::c_void);
            (*(*current).con).deco_render_params = ::core::ptr::null_mut::<deco_render_params>();
        }
        current = (*current).owindows.tqe_next as *mut owindow;
    }
    (*cmd_output).needs_tree_render = true_0 != 0;
    if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                b"success\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_uchar,
                strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn cmd_rename_workspace(
    mut current_match: *mut Match,
    mut cmd_output: *mut CommandResultIR,
    mut old_name: *const ::core::ffi::c_char,
    mut new_name: *const ::core::ffi::c_char,
) {
    if strncasecmp(
        new_name,
        b"__\0" as *const u8 as *const ::core::ffi::c_char,
        strlen(b"__\0" as *const u8 as *const ::core::ffi::c_char),
    ) == 0 as ::core::ffi::c_int
    {
        if !(*cmd_output).json_gen.is_null() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                b"Cannot rename workspace to \"%s\": names starting with __ are i3-internal.\0"
                    as *const u8 as *const ::core::ffi::c_char,
                new_name,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"success\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"error\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message as *mut ::core::ffi::c_uchar,
                    strlen(message),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message as *mut ::core::ffi::c_void);
        }
        return;
    }
    if !old_name.is_null() {
        verboselog(
            b"Renaming workspace \"%s\" to \"%s\"\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            old_name,
            new_name,
        );
    } else {
        verboselog(
            b"Renaming current workspace to \"%s\"\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            new_name,
        );
    }
    let mut workspace: *mut Con = ::core::ptr::null_mut::<Con>();
    if !old_name.is_null() {
        workspace = get_existing_workspace_by_name(old_name);
    } else {
        workspace = con_get_workspace(focused);
        old_name = (*workspace).name;
    }
    if workspace.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            let mut message_0: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message_0,
                b"Old workspace \"%s\" not found\0" as *const u8 as *const ::core::ffi::c_char,
                old_name,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"success\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"error\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message_0 as *mut ::core::ffi::c_uchar,
                    strlen(message_0),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message_0 as *mut ::core::ffi::c_void);
        }
        return;
    }
    let mut check_dest: *const Con = get_existing_workspace_by_name(new_name);
    if !check_dest.is_null() && check_dest != workspace as *const Con {
        if !(*cmd_output).json_gen.is_null() {
            let mut message_1: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message_1,
                b"New workspace \"%s\" already exists\0" as *const u8 as *const ::core::ffi::c_char,
                new_name,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"success\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"error\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message_1 as *mut ::core::ffi::c_uchar,
                    strlen(message_1),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message_1 as *mut ::core::ffi::c_void);
        }
        return;
    }
    let mut old_name_copy: *mut ::core::ffi::c_char = sstrdup(old_name);
    free((*workspace).name as *mut ::core::ffi::c_void);
    (*workspace).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*workspace).name = sstrdup(new_name);
    (*workspace).num = ws_name_to_number(new_name);
    verboselog(
        b"num = %d\n\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        (*workspace).num,
    );
    let mut previously_focused: *mut Con = focused;
    let mut previously_focused_content: *const Con = if (*focused).type_0 as ::core::ffi::c_uint
        == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*focused).parent
    } else {
        ::core::ptr::null_mut::<Con>()
    };
    let mut parent: *mut Con = (*workspace).parent as *mut Con;
    con_detach(workspace);
    con_attach(workspace, parent, false_0 != 0);
    ipc_send_workspace_event(
        b"rename\0" as *const u8 as *const ::core::ffi::c_char,
        workspace,
        ::core::ptr::null_mut::<Con>(),
    );
    let mut assigned: *mut Con =
        get_assigned_output((*workspace).name, (*workspace).num as ::core::ffi::c_long);
    if !assigned.is_null() {
        workspace_move_to_output(workspace, get_output_for_con(assigned));
    }
    let mut can_restore_focus: bool = !previously_focused.is_null();
    if !previously_focused_content.is_null() {
        let mut ws: *const Con = ::core::ptr::null::<Con>();
        let mut child: *mut Con = -(1 as ::core::ffi::c_int) as *mut Con;
        while child == -(1 as ::core::ffi::c_int) as *mut Con && {
            child = ::core::ptr::null_mut::<Con>();
            true_0 != 0
        } {
            child = (*previously_focused_content).nodes_head.tqh_first as *mut Con;
            while !child.is_null() {
                if !(child == previously_focused) {
                    child = (*child).nodes.tqe_next as *mut Con;
                } else {
                    ws = child;
                    break;
                }
            }
        }
        can_restore_focus = (can_restore_focus as ::core::ffi::c_int
            & (ws != NULL as *const Con) as ::core::ffi::c_int)
            != 0;
    }
    if can_restore_focus {
        workspace_show(con_get_workspace(previously_focused));
        con_focus(previously_focused);
    }
    if !previous_workspace_name.is_null() && strcmp(previous_workspace_name, old_name_copy) == 0 {
        free(previous_workspace_name as *mut ::core::ffi::c_void);
        previous_workspace_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
        previous_workspace_name = sstrdup(new_name);
    }
    (*cmd_output).needs_tree_render = true_0 != 0;
    if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                b"success\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_uchar,
                strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
    }
    ewmh_update_desktop_properties();
    startup_sequence_rename_workspace(old_name_copy, new_name);
    free(old_name_copy as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn cmd_bar_mode(
    mut current_match: *mut Match,
    mut cmd_output: *mut CommandResultIR,
    mut bar_mode: *const ::core::ffi::c_char,
    mut bar_id: *const ::core::ffi::c_char,
) {
    let mut mode: ::core::ffi::c_int = M_DOCK as ::core::ffi::c_int;
    let mut toggle: bool = false_0 != 0;
    if strcmp(
        bar_mode,
        b"dock\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        mode = M_DOCK as ::core::ffi::c_int;
    } else if strcmp(
        bar_mode,
        b"hide\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        mode = M_HIDE as ::core::ffi::c_int;
    } else if strcmp(
        bar_mode,
        b"invisible\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        mode = M_INVISIBLE as ::core::ffi::c_int;
    } else if strcmp(
        bar_mode,
        b"toggle\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        toggle = true_0 != 0;
    } else {
        errorlog(
            b"ERROR: Unknown bar mode \"%s\", this is a mismatch between code and parser spec.\n\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            bar_mode,
        );
        if 0 as ::core::ffi::c_int != 0 {
        } else {
            __assert_fail(
                b"false\0" as *const u8 as *const ::core::ffi::c_char,
                b"../src/commands.c\0" as *const u8 as *const ::core::ffi::c_char,
                2220 as ::core::ffi::c_uint,
                b"void cmd_bar_mode(Match *, struct CommandResultIR *, const char *, const char *)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        };
    }
    if barconfigs.tqh_first.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                b"No bars found\n\0" as *const u8 as *const ::core::ffi::c_char,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"success\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"error\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message as *mut ::core::ffi::c_uchar,
                    strlen(message),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message as *mut ::core::ffi::c_void);
        }
        return;
    }
    let mut current: *mut Barconfig = ::core::ptr::null_mut::<Barconfig>();
    current = barconfigs.tqh_first as *mut Barconfig;
    while !current.is_null() {
        if !(!bar_id.is_null() && strcmp((*current).id, bar_id) != 0 as ::core::ffi::c_int) {
            if toggle {
                mode = ((*current).mode as ::core::ffi::c_uint)
                    .wrapping_add(1 as ::core::ffi::c_uint)
                    .wrapping_rem(2 as ::core::ffi::c_uint)
                    as ::core::ffi::c_int;
            }
            debuglog(
                b"%s:%s:%d - Changing bar mode of bar_id '%s' from '%d' to '%s (%d)'\n\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"../src/commands.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"cmd_bar_mode\0" as *const u8 as *const ::core::ffi::c_char,
                2239 as ::core::ffi::c_int,
                (*current).id,
                (*current).mode as ::core::ffi::c_uint,
                bar_mode,
                mode,
            );
            if (*current).mode as ::core::ffi::c_int != mode {
                (*current).mode = mode as C2RustUnnamed_28;
                ipc_send_barconfig_update_event(current);
            }
            if !bar_id.is_null() {
                if !(*cmd_output).json_gen.is_null() {
                    if !(*cmd_output).json_gen.is_null() {
                        yajl_gen_map_open((*cmd_output).json_gen);
                    } else {
                    };
                    if !(*cmd_output).json_gen.is_null() {
                        yajl_gen_string(
                            (*cmd_output).json_gen,
                            b"success\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_uchar,
                            strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                        );
                    } else {
                    };
                    if !(*cmd_output).json_gen.is_null() {
                        yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
                    } else {
                    };
                    if !(*cmd_output).json_gen.is_null() {
                        yajl_gen_map_close((*cmd_output).json_gen);
                    } else {
                    };
                }
                return;
            }
        }
        current = (*current).configs.tqe_next as *mut Barconfig;
    }
    if !bar_id.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            let mut message_0: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message_0,
                b"Changing bar mode of bar_id %s failed, bar_id not found.\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                bar_id,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"success\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"error\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message_0 as *mut ::core::ffi::c_uchar,
                    strlen(message_0),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message_0 as *mut ::core::ffi::c_void);
        }
    } else if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                b"success\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_uchar,
                strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn cmd_bar_hidden_state(
    mut current_match: *mut Match,
    mut cmd_output: *mut CommandResultIR,
    mut bar_hidden_state: *const ::core::ffi::c_char,
    mut bar_id: *const ::core::ffi::c_char,
) {
    let mut hidden_state: ::core::ffi::c_int = S_SHOW as ::core::ffi::c_int;
    let mut toggle: bool = false_0 != 0;
    if strcmp(
        bar_hidden_state,
        b"hide\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        hidden_state = S_HIDE as ::core::ffi::c_int;
    } else if strcmp(
        bar_hidden_state,
        b"show\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        hidden_state = S_SHOW as ::core::ffi::c_int;
    } else if strcmp(
        bar_hidden_state,
        b"toggle\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        toggle = true_0 != 0;
    } else {
        errorlog(
            b"ERROR: Unknown bar state \"%s\", this is a mismatch between code and parser spec.\n\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            bar_hidden_state,
        );
        if 0 as ::core::ffi::c_int != 0 {
        } else {
            __assert_fail(
                b"false\0" as *const u8 as *const ::core::ffi::c_char,
                b"../src/commands.c\0" as *const u8 as *const ::core::ffi::c_char,
                2277 as ::core::ffi::c_uint,
                b"void cmd_bar_hidden_state(Match *, struct CommandResultIR *, const char *, const char *)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        };
    }
    if barconfigs.tqh_first.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                b"No bars found\n\0" as *const u8 as *const ::core::ffi::c_char,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"success\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"error\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message as *mut ::core::ffi::c_uchar,
                    strlen(message),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message as *mut ::core::ffi::c_void);
        }
        return;
    }
    let mut current: *mut Barconfig = ::core::ptr::null_mut::<Barconfig>();
    current = barconfigs.tqh_first as *mut Barconfig;
    while !current.is_null() {
        if !(!bar_id.is_null() && strcmp((*current).id, bar_id) != 0 as ::core::ffi::c_int) {
            if toggle {
                hidden_state = ((*current).hidden_state as ::core::ffi::c_uint)
                    .wrapping_add(1 as ::core::ffi::c_uint)
                    .wrapping_rem(2 as ::core::ffi::c_uint)
                    as ::core::ffi::c_int;
            }
            debuglog(
                b"%s:%s:%d - Changing bar hidden_state of bar_id '%s' from '%d' to '%s (%d)'\n\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"../src/commands.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"cmd_bar_hidden_state\0" as *const u8 as *const ::core::ffi::c_char,
                2296 as ::core::ffi::c_int,
                (*current).id,
                (*current).hidden_state as ::core::ffi::c_uint,
                bar_hidden_state,
                hidden_state,
            );
            if (*current).hidden_state as ::core::ffi::c_int != hidden_state {
                (*current).hidden_state = hidden_state as C2RustUnnamed_27;
                ipc_send_barconfig_update_event(current);
            }
            if !bar_id.is_null() {
                if !(*cmd_output).json_gen.is_null() {
                    if !(*cmd_output).json_gen.is_null() {
                        yajl_gen_map_open((*cmd_output).json_gen);
                    } else {
                    };
                    if !(*cmd_output).json_gen.is_null() {
                        yajl_gen_string(
                            (*cmd_output).json_gen,
                            b"success\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_uchar,
                            strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                        );
                    } else {
                    };
                    if !(*cmd_output).json_gen.is_null() {
                        yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
                    } else {
                    };
                    if !(*cmd_output).json_gen.is_null() {
                        yajl_gen_map_close((*cmd_output).json_gen);
                    } else {
                    };
                }
                return;
            }
        }
        current = (*current).configs.tqe_next as *mut Barconfig;
    }
    if !bar_id.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            let mut message_0: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message_0,
                b"Changing bar hidden_state of bar_id %s failed, bar_id not found.\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                bar_id,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"success\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    b"error\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message_0 as *mut ::core::ffi::c_uchar,
                    strlen(message_0),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message_0 as *mut ::core::ffi::c_void);
        }
    } else if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                b"success\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_uchar,
                strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn cmd_shmlog(
    mut current_match: *mut Match,
    mut cmd_output: *mut CommandResultIR,
    mut argument: *const ::core::ffi::c_char,
) {
    if strcmp(
        argument,
        b"toggle\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        shmlog_size = if shmlog_size != 0 {
            -shmlog_size
        } else {
            default_shmlog_size
        };
    } else if strcmp(argument, b"on\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        shmlog_size = default_shmlog_size;
    } else if strcmp(
        argument,
        b"off\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        shmlog_size = 0 as ::core::ffi::c_int;
    } else {
        let mut new_size: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
        if !parse_long(argument, &raw mut new_size, 0 as ::core::ffi::c_int) {
            if !(*cmd_output).json_gen.is_null() {
                let mut message: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                sasprintf(
                    &raw mut message,
                    b"Failed to parse %s into a shmlog size.\0" as *const u8
                        as *const ::core::ffi::c_char,
                    argument,
                );
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_map_open((*cmd_output).json_gen);
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_string(
                        (*cmd_output).json_gen,
                        b"success\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_uchar,
                        strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                    );
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_bool((*cmd_output).json_gen, false_0);
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_string(
                        (*cmd_output).json_gen,
                        b"error\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_uchar,
                        strlen(b"error\0" as *const u8 as *const ::core::ffi::c_char),
                    );
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_string(
                        (*cmd_output).json_gen,
                        message as *mut ::core::ffi::c_uchar,
                        strlen(message),
                    );
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_map_close((*cmd_output).json_gen);
                } else {
                };
                free(message as *mut ::core::ffi::c_void);
            }
            return;
        }
        if shmlog_size > 0 as ::core::ffi::c_int {
            shmlog_size = 0 as ::core::ffi::c_int;
            verboselog(
                b"Restarting shm logging...\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
            init_logging();
        }
        shmlog_size = new_size as ::core::ffi::c_int;
    }
    verboselog(
        b"%s shm logging\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        if shmlog_size > 0 as ::core::ffi::c_int {
            b"Enabling\0" as *const u8 as *const ::core::ffi::c_char
        } else {
            b"Disabling\0" as *const u8 as *const ::core::ffi::c_char
        },
    );
    init_logging();
    update_shmlog_atom();
    if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                b"success\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_uchar,
                strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn cmd_debuglog(
    mut current_match: *mut Match,
    mut cmd_output: *mut CommandResultIR,
    mut argument: *const ::core::ffi::c_char,
) {
    let logging: bool = get_debug_logging() as bool;
    if strcmp(
        argument,
        b"toggle\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        verboselog(
            b"%s debug logging\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            if logging as ::core::ffi::c_int != 0 {
                b"Disabling\0" as *const u8 as *const ::core::ffi::c_char
            } else {
                b"Enabling\0" as *const u8 as *const ::core::ffi::c_char
            },
        );
        set_debug_logging(!logging);
    } else if strcmp(argument, b"on\0" as *const u8 as *const ::core::ffi::c_char) == 0 && !logging
    {
        verboselog(
            b"Enabling debug logging\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        set_debug_logging(true_0 != 0);
    } else if strcmp(
        argument,
        b"off\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
        && logging as ::core::ffi::c_int != 0
    {
        verboselog(
            b"Disabling debug logging\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        set_debug_logging(false_0 != 0);
    }
    if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                b"success\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_uchar,
                strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
    }
}
unsafe extern "C" fn gaps_inner(mut gaps: *mut gaps_t) -> *mut ::core::ffi::c_int {
    return &raw mut (*gaps).inner;
}
unsafe extern "C" fn gaps_top(mut gaps: *mut gaps_t) -> *mut ::core::ffi::c_int {
    return &raw mut (*gaps).top;
}
unsafe extern "C" fn gaps_left(mut gaps: *mut gaps_t) -> *mut ::core::ffi::c_int {
    return &raw mut (*gaps).left;
}
unsafe extern "C" fn gaps_bottom(mut gaps: *mut gaps_t) -> *mut ::core::ffi::c_int {
    return &raw mut (*gaps).bottom;
}
unsafe extern "C" fn gaps_right(mut gaps: *mut gaps_t) -> *mut ::core::ffi::c_int {
    return &raw mut (*gaps).right;
}
unsafe extern "C" fn gaps_update(
    get: gap_accessor,
    mut scope: *const ::core::ffi::c_char,
    mut mode: *const ::core::ffi::c_char,
    pixels: ::core::ffi::c_int,
) -> bool {
    debuglog(
        b"%s:%s:%d - gaps_update(scope=%s, mode=%s, pixels=%d)\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/commands.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"gaps_update\0" as *const u8 as *const ::core::ffi::c_char,
        2394 as ::core::ffi::c_int,
        scope,
        mode,
        pixels,
    );
    let mut workspace: *mut Con = con_get_workspace(focused);
    let global_gap_size: ::core::ffi::c_int =
        *get.expect("non-null function pointer")(&raw mut config.gaps);
    let mut current_value: ::core::ffi::c_int = global_gap_size;
    if strcmp(
        scope,
        b"current\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        current_value += *get.expect("non-null function pointer")(&raw mut (*workspace).gaps);
    }
    debuglog(
        b"%s:%s:%d - global_gap_size=%d, current_value=%d\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/commands.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"gaps_update\0" as *const u8 as *const ::core::ffi::c_char,
        2402 as ::core::ffi::c_int,
        global_gap_size,
        current_value,
    );
    let mut reset: bool = false_0 != 0;
    if strcmp(mode, b"plus\0" as *const u8 as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int
    {
        current_value += pixels;
    } else if strcmp(mode, b"minus\0" as *const u8 as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        current_value -= pixels;
    } else if strcmp(mode, b"set\0" as *const u8 as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        current_value = pixels;
        reset = true_0 != 0;
    } else if strcmp(mode, b"toggle\0" as *const u8 as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        current_value = (current_value == 0) as ::core::ffi::c_int * pixels;
        reset = true_0 != 0;
    } else {
        errorlog(
            b"ERROR: Invalid mode %s when changing gaps\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            mode,
        );
        return false_0 != 0;
    }
    let mut min_value: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let is_outer: bool = get.expect("non-null function pointer")(&raw mut config.gaps)
        != gaps_inner(&raw mut config.gaps);
    if is_outer {
        if strcmp(scope, b"all\0" as *const u8 as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            min_value = -config.gaps.inner;
        } else {
            min_value = -config.gaps.inner - (*workspace).gaps.inner;
        }
    }
    if current_value < min_value {
        current_value = min_value;
    }
    if strcmp(scope, b"all\0" as *const u8 as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int
    {
        let mut output: *mut Con = ::core::ptr::null_mut::<Con>();
        output = (*croot).nodes_head.tqh_first as *mut Con;
        while !output.is_null() {
            let mut cur_ws: *mut Con = ::core::ptr::null_mut::<Con>();
            let mut content: *const Con = output_get_content(output);
            cur_ws = (*content).nodes_head.tqh_first as *mut Con;
            while !cur_ws.is_null() {
                let mut gaps_value: *mut ::core::ffi::c_int =
                    get.expect("non-null function pointer")(&raw mut (*cur_ws).gaps);
                debuglog(
                    b"%s:%s:%d - current gaps_value = %d\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    b"../src/commands.c\0" as *const u8 as *const ::core::ffi::c_char,
                    b"gaps_update\0" as *const u8 as *const ::core::ffi::c_char,
                    2443 as ::core::ffi::c_int,
                    *gaps_value,
                );
                if reset {
                    *gaps_value = 0 as ::core::ffi::c_int;
                } else {
                    let mut max_compensate: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    if is_outer {
                        max_compensate = config.gaps.inner;
                    }
                    if *gaps_value + current_value + max_compensate < 0 as ::core::ffi::c_int {
                        *gaps_value = -current_value - max_compensate;
                    }
                }
                debuglog(
                    b"%s:%s:%d - gaps_value after fix = %d\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    b"../src/commands.c\0" as *const u8 as *const ::core::ffi::c_char,
                    b"gaps_update\0" as *const u8 as *const ::core::ffi::c_char,
                    2459 as ::core::ffi::c_int,
                    *gaps_value,
                );
                cur_ws = (*cur_ws).nodes.tqe_next as *mut Con;
            }
            output = (*output).nodes.tqe_next as *mut Con;
        }
        *get.expect("non-null function pointer")(&raw mut config.gaps) = current_value;
        debuglog(
            b"%s:%s:%d - global gaps value after fix = %d\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/commands.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"gaps_update\0" as *const u8 as *const ::core::ffi::c_char,
            2464 as ::core::ffi::c_int,
            *get.expect("non-null function pointer")(&raw mut config.gaps),
        );
    } else {
        let mut gaps_value_0: *mut ::core::ffi::c_int =
            get.expect("non-null function pointer")(&raw mut (*workspace).gaps);
        *gaps_value_0 = current_value - global_gap_size;
    }
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn cmd_gaps(
    mut current_match: *mut Match,
    mut cmd_output: *mut CommandResultIR,
    mut type_0: *const ::core::ffi::c_char,
    mut scope: *const ::core::ffi::c_char,
    mut mode: *const ::core::ffi::c_char,
    mut value: *const ::core::ffi::c_char,
) {
    let mut current_block: u64;
    let pixels: ::core::ffi::c_int = logical_px(atoi(value)) as ::core::ffi::c_int;
    if strcmp(
        type_0,
        b"inner\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        if !gaps_update(
            Some(gaps_inner as unsafe extern "C" fn(*mut gaps_t) -> *mut ::core::ffi::c_int),
            scope,
            mode,
            pixels,
        ) {
            current_block = 1063758793336646565;
        } else if !gaps_update(
            Some(gaps_top as unsafe extern "C" fn(*mut gaps_t) -> *mut ::core::ffi::c_int),
            b"all\0" as *const u8 as *const ::core::ffi::c_char,
            b"plus\0" as *const u8 as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
        ) || !gaps_update(
            Some(gaps_bottom as unsafe extern "C" fn(*mut gaps_t) -> *mut ::core::ffi::c_int),
            b"all\0" as *const u8 as *const ::core::ffi::c_char,
            b"plus\0" as *const u8 as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
        ) || !gaps_update(
            Some(gaps_right as unsafe extern "C" fn(*mut gaps_t) -> *mut ::core::ffi::c_int),
            b"all\0" as *const u8 as *const ::core::ffi::c_char,
            b"plus\0" as *const u8 as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
        ) || !gaps_update(
            Some(gaps_left as unsafe extern "C" fn(*mut gaps_t) -> *mut ::core::ffi::c_int),
            b"all\0" as *const u8 as *const ::core::ffi::c_char,
            b"plus\0" as *const u8 as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
        ) {
            current_block = 1063758793336646565;
        } else {
            current_block = 11057878835866523405;
        }
    } else if strcmp(
        type_0,
        b"outer\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        if !gaps_update(
            Some(gaps_top as unsafe extern "C" fn(*mut gaps_t) -> *mut ::core::ffi::c_int),
            scope,
            mode,
            pixels,
        ) || !gaps_update(
            Some(gaps_bottom as unsafe extern "C" fn(*mut gaps_t) -> *mut ::core::ffi::c_int),
            scope,
            mode,
            pixels,
        ) || !gaps_update(
            Some(gaps_right as unsafe extern "C" fn(*mut gaps_t) -> *mut ::core::ffi::c_int),
            scope,
            mode,
            pixels,
        ) || !gaps_update(
            Some(gaps_left as unsafe extern "C" fn(*mut gaps_t) -> *mut ::core::ffi::c_int),
            scope,
            mode,
            pixels,
        ) {
            current_block = 1063758793336646565;
        } else {
            current_block = 11057878835866523405;
        }
    } else if strcmp(
        type_0,
        b"vertical\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        if !gaps_update(
            Some(gaps_top as unsafe extern "C" fn(*mut gaps_t) -> *mut ::core::ffi::c_int),
            scope,
            mode,
            pixels,
        ) || !gaps_update(
            Some(gaps_bottom as unsafe extern "C" fn(*mut gaps_t) -> *mut ::core::ffi::c_int),
            scope,
            mode,
            pixels,
        ) {
            current_block = 1063758793336646565;
        } else {
            current_block = 11057878835866523405;
        }
    } else if strcmp(
        type_0,
        b"horizontal\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        if !gaps_update(
            Some(gaps_right as unsafe extern "C" fn(*mut gaps_t) -> *mut ::core::ffi::c_int),
            scope,
            mode,
            pixels,
        ) || !gaps_update(
            Some(gaps_left as unsafe extern "C" fn(*mut gaps_t) -> *mut ::core::ffi::c_int),
            scope,
            mode,
            pixels,
        ) {
            current_block = 1063758793336646565;
        } else {
            current_block = 11057878835866523405;
        }
    } else if strcmp(type_0, b"top\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        if !gaps_update(
            Some(gaps_top as unsafe extern "C" fn(*mut gaps_t) -> *mut ::core::ffi::c_int),
            scope,
            mode,
            pixels,
        ) {
            current_block = 1063758793336646565;
        } else {
            current_block = 11057878835866523405;
        }
    } else if strcmp(
        type_0,
        b"bottom\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        if !gaps_update(
            Some(gaps_bottom as unsafe extern "C" fn(*mut gaps_t) -> *mut ::core::ffi::c_int),
            scope,
            mode,
            pixels,
        ) {
            current_block = 1063758793336646565;
        } else {
            current_block = 11057878835866523405;
        }
    } else if strcmp(
        type_0,
        b"right\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        if !gaps_update(
            Some(gaps_right as unsafe extern "C" fn(*mut gaps_t) -> *mut ::core::ffi::c_int),
            scope,
            mode,
            pixels,
        ) {
            current_block = 1063758793336646565;
        } else {
            current_block = 11057878835866523405;
        }
    } else if strcmp(type_0, b"left\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        if !gaps_update(
            Some(gaps_left as unsafe extern "C" fn(*mut gaps_t) -> *mut ::core::ffi::c_int),
            scope,
            mode,
            pixels,
        ) {
            current_block = 1063758793336646565;
        } else {
            current_block = 11057878835866523405;
        }
    } else {
        errorlog(
            b"ERROR: Invalid type %s when changing gaps\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            type_0,
        );
        current_block = 1063758793336646565;
    }
    match current_block {
        1063758793336646565 => {
            if !(*cmd_output).json_gen.is_null() {
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_map_open((*cmd_output).json_gen);
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_string(
                        (*cmd_output).json_gen,
                        b"success\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_uchar,
                        strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                    );
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_bool((*cmd_output).json_gen, 0 as ::core::ffi::c_int);
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_map_close((*cmd_output).json_gen);
                } else {
                };
            }
            return;
        }
        _ => {
            (*cmd_output).needs_tree_render = true_0 != 0;
            if !(*cmd_output).json_gen.is_null() {
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_map_open((*cmd_output).json_gen);
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_string(
                        (*cmd_output).json_gen,
                        b"success\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_uchar,
                        strlen(b"success\0" as *const u8 as *const ::core::ffi::c_char),
                    );
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_map_close((*cmd_output).json_gen);
                } else {
                };
            }
            return;
        }
    };
}
pub const EXIT_SUCCESS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" fn run_static_initializers() {
    names = user_output_names_head {
        tqh_first: ::core::ptr::null::<user_output_name>() as *mut user_output_name,
        tqh_last: &raw const names.tqh_first as *mut *mut user_output_name,
    };
    names_global = user_output_names_head {
        tqh_first: ::core::ptr::null::<user_output_name>() as *mut user_output_name,
        tqh_last: &raw const names_global.tqh_first as *mut *mut user_output_name,
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
