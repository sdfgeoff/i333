/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * For more information on RandR, please see the X.org RandR specification at
 * https://cgit.freedesktop.org/xorg/proto/randrproto/tree/randrproto.txt
 * (take your time to read it completely, it answers all questions).
 *
 */
// Mechanical file-by-file Rust translation of src/randr.c.
// Generated with C2Rust 0.22.1; C ABI and original ownership retained.
extern "C" {
    pub type xcb_connection_t;
    pub type xcb_extension_t;
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
    fn errx(__status: ::core::ffi::c_int, __format: *const ::core::ffi::c_char, ...) -> !;
    fn xcb_get_atom_name(c: *mut xcb_connection_t, atom: xcb_atom_t) -> xcb_get_atom_name_cookie_t;
    fn xcb_get_atom_name_name(R: *const xcb_get_atom_name_reply_t) -> *mut ::core::ffi::c_char;
    fn xcb_get_atom_name_name_length(R: *const xcb_get_atom_name_reply_t) -> ::core::ffi::c_int;
    fn xcb_get_atom_name_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_get_atom_name_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_get_atom_name_reply_t;
    fn xcb_flush(c: *mut xcb_connection_t) -> ::core::ffi::c_int;
    fn xcb_get_extension_data(
        c: *mut xcb_connection_t,
        ext: *mut xcb_extension_t,
    ) -> *const xcb_query_extension_reply_t;
    static mut conn: *mut xcb_connection_t;
    static mut root_screen: *mut xcb_screen_t;
    fn verboselog(fmt: *mut ::core::ffi::c_char, ...);
    fn errorlog(fmt: *mut ::core::ffi::c_char, ...);
    fn debuglog(fmt: *mut ::core::ffi::c_char, ...);
    fn scalloc(num: size_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn sstrdup(str: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn sasprintf(
        strp: *mut *mut ::core::ffi::c_char,
        fmt: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    static mut xcb_randr_id: xcb_extension_t;
    fn xcb_randr_query_version(
        c: *mut xcb_connection_t,
        major_version: uint32_t,
        minor_version: uint32_t,
    ) -> xcb_randr_query_version_cookie_t;
    fn xcb_randr_query_version_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_randr_query_version_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_randr_query_version_reply_t;
    fn xcb_randr_select_input(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        enable: uint16_t,
    ) -> xcb_void_cookie_t;
    fn xcb_randr_get_output_info(
        c: *mut xcb_connection_t,
        output: xcb_randr_output_t,
        config_timestamp: xcb_timestamp_t,
    ) -> xcb_randr_get_output_info_cookie_t;
    fn xcb_randr_get_output_info_name(R: *const xcb_randr_get_output_info_reply_t) -> *mut uint8_t;
    fn xcb_randr_get_output_info_name_length(
        R: *const xcb_randr_get_output_info_reply_t,
    ) -> ::core::ffi::c_int;
    fn xcb_randr_get_output_info_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_randr_get_output_info_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_randr_get_output_info_reply_t;
    fn xcb_randr_get_crtc_info(
        c: *mut xcb_connection_t,
        crtc: xcb_randr_crtc_t,
        config_timestamp: xcb_timestamp_t,
    ) -> xcb_randr_get_crtc_info_cookie_t;
    fn xcb_randr_get_crtc_info_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_randr_get_crtc_info_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_randr_get_crtc_info_reply_t;
    fn xcb_randr_get_screen_resources_current(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_randr_get_screen_resources_current_cookie_t;
    fn xcb_randr_get_screen_resources_current_outputs(
        R: *const xcb_randr_get_screen_resources_current_reply_t,
    ) -> *mut xcb_randr_output_t;
    fn xcb_randr_get_screen_resources_current_outputs_length(
        R: *const xcb_randr_get_screen_resources_current_reply_t,
    ) -> ::core::ffi::c_int;
    fn xcb_randr_get_screen_resources_current_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_randr_get_screen_resources_current_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_randr_get_screen_resources_current_reply_t;
    fn xcb_randr_get_output_primary(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_randr_get_output_primary_cookie_t;
    fn xcb_randr_get_output_primary_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_randr_get_output_primary_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_randr_get_output_primary_reply_t;
    fn xcb_randr_monitor_info_outputs(
        R: *const xcb_randr_monitor_info_t,
    ) -> *mut xcb_randr_output_t;
    fn xcb_randr_monitor_info_outputs_length(
        R: *const xcb_randr_monitor_info_t,
    ) -> ::core::ffi::c_int;
    fn xcb_randr_monitor_info_next(i: *mut xcb_randr_monitor_info_iterator_t);
    fn xcb_randr_get_monitors(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        get_active: uint8_t,
    ) -> xcb_randr_get_monitors_cookie_t;
    fn xcb_randr_get_monitors_monitors_length(
        R: *const xcb_randr_get_monitors_reply_t,
    ) -> ::core::ffi::c_int;
    fn xcb_randr_get_monitors_monitors_iterator(
        R: *const xcb_randr_get_monitors_reply_t,
    ) -> xcb_randr_monitor_info_iterator_t;
    fn xcb_randr_get_monitors_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_randr_get_monitors_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_randr_get_monitors_reply_t;
    fn min(a: ::core::ffi::c_int, b: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn max(a: ::core::ffi::c_int, b: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn update_if_necessary(destination: *mut uint32_t, new_value: uint32_t) -> bool;
    static mut croot: *mut Con;
    static mut focused: *mut Con;
    static mut all_cons: all_cons_head;
    fn tree_render();
    fn tree_close_internal(
        con: *mut Con,
        kill_window: kill_window_t,
        dont_kill_parent: bool,
    ) -> bool;
    static mut ws_assignments: ws_assignments_head;
    static mut root: xcb_window_t;
    static mut config: Config;
    fn get_assigned_output(
        name: *const ::core::ffi::c_char,
        parsed_num: ::core::ffi::c_long,
    ) -> *mut Con;
    fn output_triggers_assignment(
        output: *mut Output,
        assignment: *mut Workspace_Assignment,
    ) -> bool;
    fn create_workspace_on_output(output: *mut Output, content: *mut Con) -> *mut Con;
    fn workspace_show(ws: *mut Con);
    fn workspace_show_by_name(num: *const ::core::ffi::c_char);
    fn workspace_move_to_output(ws: *mut Con, output: *mut Output);
    fn x_set_name(con: *mut Con, name: *const ::core::ffi::c_char);
    fn floating_fix_coordinates(con: *mut Con, old_rect: *mut Rect, new_rect: *mut Rect);
    fn con_new(parent: *mut Con, window: *mut i3Window) -> *mut Con;
    fn con_focus(con: *mut Con);
    fn con_get_workspace(con: *mut Con) -> *mut Con;
    fn con_is_internal(con: *mut Con) -> bool;
    fn con_for_window(
        con: *mut Con,
        window: *mut i3Window,
        store_match: *mut *mut Match,
    ) -> *mut Con;
    fn con_num_children(con: *mut Con) -> ::core::ffi::c_int;
    fn con_attach(con: *mut Con, parent: *mut Con, ignore_focus: bool);
    fn con_detach(con: *mut Con);
    fn con_fix_percent(con: *mut Con);
    fn match_init(match_0: *mut Match);
    fn output_get_content(output: *mut Con) -> *mut Con;
    fn output_primary_name(output: *mut Output) -> *mut ::core::ffi::c_char;
    fn get_output_for_con(con: *mut Con) -> *mut Output;
    fn ewmh_update_desktop_properties();
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
pub struct xcb_get_atom_name_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_get_atom_name_reply_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub name_len: uint16_t,
    pub pad1: [uint8_t; 22],
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_query_extension_reply_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub present: uint8_t,
    pub major_opcode: uint8_t,
    pub first_event: uint8_t,
    pub first_error: uint8_t,
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
pub type xcb_randr_mode_t = uint32_t;
pub type xcb_randr_crtc_t = uint32_t;
pub type xcb_randr_output_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_randr_query_version_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_randr_query_version_reply_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub major_version: uint32_t,
    pub minor_version: uint32_t,
    pub pad1: [uint8_t; 16],
}
pub type xcb_randr_notify_mask_t = ::core::ffi::c_uint;
pub const XCB_RANDR_NOTIFY_MASK_LEASE: xcb_randr_notify_mask_t = 128;
pub const XCB_RANDR_NOTIFY_MASK_RESOURCE_CHANGE: xcb_randr_notify_mask_t = 64;
pub const XCB_RANDR_NOTIFY_MASK_PROVIDER_PROPERTY: xcb_randr_notify_mask_t = 32;
pub const XCB_RANDR_NOTIFY_MASK_PROVIDER_CHANGE: xcb_randr_notify_mask_t = 16;
pub const XCB_RANDR_NOTIFY_MASK_OUTPUT_PROPERTY: xcb_randr_notify_mask_t = 8;
pub const XCB_RANDR_NOTIFY_MASK_OUTPUT_CHANGE: xcb_randr_notify_mask_t = 4;
pub const XCB_RANDR_NOTIFY_MASK_CRTC_CHANGE: xcb_randr_notify_mask_t = 2;
pub const XCB_RANDR_NOTIFY_MASK_SCREEN_CHANGE: xcb_randr_notify_mask_t = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_randr_get_output_info_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_randr_get_output_info_reply_t {
    pub response_type: uint8_t,
    pub status: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub timestamp: xcb_timestamp_t,
    pub crtc: xcb_randr_crtc_t,
    pub mm_width: uint32_t,
    pub mm_height: uint32_t,
    pub connection: uint8_t,
    pub subpixel_order: uint8_t,
    pub num_crtcs: uint16_t,
    pub num_modes: uint16_t,
    pub num_preferred: uint16_t,
    pub num_clones: uint16_t,
    pub name_len: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_randr_get_crtc_info_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_randr_get_crtc_info_reply_t {
    pub response_type: uint8_t,
    pub status: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub timestamp: xcb_timestamp_t,
    pub x: int16_t,
    pub y: int16_t,
    pub width: uint16_t,
    pub height: uint16_t,
    pub mode: xcb_randr_mode_t,
    pub rotation: uint16_t,
    pub rotations: uint16_t,
    pub num_outputs: uint16_t,
    pub num_possible_outputs: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_randr_get_screen_resources_current_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_randr_get_screen_resources_current_reply_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub timestamp: xcb_timestamp_t,
    pub config_timestamp: xcb_timestamp_t,
    pub num_crtcs: uint16_t,
    pub num_outputs: uint16_t,
    pub num_modes: uint16_t,
    pub names_len: uint16_t,
    pub pad1: [uint8_t; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_randr_get_output_primary_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_randr_get_output_primary_reply_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub output: xcb_randr_output_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_randr_monitor_info_t {
    pub name: xcb_atom_t,
    pub primary: uint8_t,
    pub automatic: uint8_t,
    pub nOutput: uint16_t,
    pub x: int16_t,
    pub y: int16_t,
    pub width: uint16_t,
    pub height: uint16_t,
    pub width_in_millimeters: uint32_t,
    pub height_in_millimeters: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_randr_monitor_info_iterator_t {
    pub data: *mut xcb_randr_monitor_info_t,
    pub rem: ::core::ffi::c_int,
    pub index: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_randr_get_monitors_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_randr_get_monitors_reply_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub timestamp: xcb_timestamp_t,
    pub nMonitors: uint32_t,
    pub nOutputs: uint32_t,
    pub pad1: [uint8_t; 12],
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
pub type C2RustUnnamed_21 = ::core::ffi::c_uint;
pub const VERT: C2RustUnnamed_21 = 2;
pub const HORIZ: C2RustUnnamed_21 = 1;
pub const NO_ORIENTATION: C2RustUnnamed_21 = 0;
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
pub type gaps_mask_t = ::core::ffi::c_uint;
pub const GAPS_OUTER: gaps_mask_t = 30;
pub const GAPS_HORIZONTAL: gaps_mask_t = 20;
pub const GAPS_VERTICAL: gaps_mask_t = 10;
pub const GAPS_LEFT: gaps_mask_t = 16;
pub const GAPS_BOTTOM: gaps_mask_t = 8;
pub const GAPS_RIGHT: gaps_mask_t = 4;
pub const GAPS_TOP: gaps_mask_t = 2;
pub const GAPS_INNER: gaps_mask_t = 1;
pub type focus_wrapping_t = ::core::ffi::c_uint;
pub const FOCUS_WRAPPING_WORKSPACE: focus_wrapping_t = 3;
pub const FOCUS_WRAPPING_FORCE: focus_wrapping_t = 2;
pub const FOCUS_WRAPPING_ON: focus_wrapping_t = 1;
pub const FOCUS_WRAPPING_OFF: focus_wrapping_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Workspace_Assignment {
    pub name: *mut ::core::ffi::c_char,
    pub output: *mut ::core::ffi::c_char,
    pub gaps: gaps_t,
    pub gaps_mask: gaps_mask_t,
    pub ws_assignments: C2RustUnnamed_22,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_22 {
    pub tqe_next: *mut Workspace_Assignment,
    pub tqe_prev: *mut *mut Workspace_Assignment,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct all_cons_head {
    pub tqh_first: *mut Con,
    pub tqh_last: *mut *mut Con,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ws_assignments_head {
    pub tqh_first: *mut Workspace_Assignment,
    pub tqh_last: *mut *mut Workspace_Assignment,
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
    pub focus_on_window_activation: C2RustUnnamed_25,
    pub show_marks: bool,
    pub title_align: C2RustUnnamed_24,
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
    pub popup_during_fullscreen: C2RustUnnamed_23,
    pub number_barconfigs: ::core::ffi::c_int,
    pub tiling_drag: tiling_drag_t,
    pub gaps: gaps_t,
    pub smart_gaps: smart_gaps_t,
}
pub type C2RustUnnamed_23 = ::core::ffi::c_uint;
pub const PDF_ALL: C2RustUnnamed_23 = 3;
pub const PDF_IGNORE: C2RustUnnamed_23 = 2;
pub const PDF_LEAVE_FULLSCREEN: C2RustUnnamed_23 = 1;
pub const PDF_SMART: C2RustUnnamed_23 = 0;
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
pub type C2RustUnnamed_24 = ::core::ffi::c_uint;
pub const ALIGN_RIGHT: C2RustUnnamed_24 = 2;
pub const ALIGN_CENTER: C2RustUnnamed_24 = 1;
pub const ALIGN_LEFT: C2RustUnnamed_24 = 0;
pub type C2RustUnnamed_25 = ::core::ffi::c_uint;
pub const FOWA_NONE: C2RustUnnamed_25 = 3;
pub const FOWA_FOCUS: C2RustUnnamed_25 = 2;
pub const FOWA_URGENT: C2RustUnnamed_25 = 1;
pub const FOWA_SMART: C2RustUnnamed_25 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct outputs_head {
    pub tqh_first: *mut xoutput,
    pub tqh_last: *mut *mut xoutput,
}
pub type output_close_far_t = ::core::ffi::c_uint;
pub const FARTHEST_OUTPUT: output_close_far_t = 1;
pub const CLOSEST_OUTPUT: output_close_far_t = 0;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const XCB_NONE: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const XCB_RANDR_MAJOR_VERSION: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const XCB_RANDR_MINOR_VERSION: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
#[no_mangle]
pub static mut primary: *mut xcb_randr_get_output_primary_reply_t =
    ::core::ptr::null::<xcb_randr_get_output_primary_reply_t>()
        as *mut xcb_randr_get_output_primary_reply_t;
#[no_mangle]
pub static mut outputs: outputs_head = outputs_head {
    tqh_first: ::core::ptr::null::<xoutput>() as *mut xoutput,
    tqh_last: ::core::ptr::null::<*mut xoutput>() as *mut *mut xoutput,
};
static mut root_output: *mut Output = ::core::ptr::null::<Output>() as *mut Output;
static mut has_randr_1_5: bool = false_0 != 0;
unsafe extern "C" fn get_output_by_id(mut id: xcb_randr_output_t) -> *mut Output {
    let mut output: *mut Output = ::core::ptr::null_mut::<Output>();
    output = outputs.tqh_first as *mut Output;
    while !output.is_null() {
        if (*output).id == id {
            return output;
        }
        output = (*output).outputs.tqe_next as *mut Output;
    }
    return ::core::ptr::null_mut::<Output>();
}
#[no_mangle]
pub unsafe extern "C" fn get_output_by_name(
    mut name: *const ::core::ffi::c_char,
    require_active: bool,
) -> *mut Output {
    let get_primary: bool = strcasecmp(
        b"primary\0" as *const u8 as *const ::core::ffi::c_char,
        name,
    ) == 0 as ::core::ffi::c_int;
    let get_non_primary: bool = strcasecmp(
        b"nonprimary\0" as *const u8 as *const ::core::ffi::c_char,
        name,
    ) == 0 as ::core::ffi::c_int;
    let mut output: *mut Output = ::core::ptr::null_mut::<Output>();
    output = outputs.tqh_first as *mut Output;
    while !output.is_null() {
        if !(require_active as ::core::ffi::c_int != 0 && !(*output).active) {
            if (*output).primary as ::core::ffi::c_int != 0
                && get_primary as ::core::ffi::c_int != 0
            {
                return output;
            }
            if !(*output).primary && get_non_primary as ::core::ffi::c_int != 0 {
                return output;
            }
            let mut output_name: *mut output_name = ::core::ptr::null_mut::<output_name>();
            output_name = (*output).names_head.slh_first;
            while !output_name.is_null() {
                if strcasecmp((*output_name).name, name) == 0 as ::core::ffi::c_int {
                    return output;
                }
                output_name = (*output_name).names.sle_next;
            }
        }
        output = (*output).outputs.tqe_next as *mut Output;
    }
    return ::core::ptr::null_mut::<Output>();
}
#[no_mangle]
pub unsafe extern "C" fn get_first_output() -> *mut Output {
    let mut output: *mut Output = ::core::ptr::null_mut::<Output>();
    let mut result: *mut Output = ::core::ptr::null_mut::<Output>();
    output = outputs.tqh_first as *mut Output;
    while !output.is_null() {
        if (*output).active {
            if (*output).primary {
                return output;
            }
            if result.is_null() {
                result = output;
            }
        }
        output = (*output).outputs.tqe_next as *mut Output;
    }
    if !result.is_null() {
        return result;
    }
    errx(
        EXIT_FAILURE,
        b"No usable outputs available.\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
}
unsafe extern "C" fn any_randr_output_active() -> bool {
    let mut output: *mut Output = ::core::ptr::null_mut::<Output>();
    output = outputs.tqh_first as *mut Output;
    while !output.is_null() {
        if output != root_output
            && !(*output).to_be_disabled
            && (*output).active as ::core::ffi::c_int != 0
        {
            return true_0 != 0;
        }
        output = (*output).outputs.tqe_next as *mut Output;
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn get_output_containing(
    mut x: ::core::ffi::c_uint,
    mut y: ::core::ffi::c_uint,
) -> *mut Output {
    let mut output: *mut Output = ::core::ptr::null_mut::<Output>();
    output = outputs.tqh_first as *mut Output;
    while !output.is_null() {
        if (*output).active {
            debuglog(
                b"%s:%s:%d - comparing x=%d y=%d with x=%d and y=%d width %d height %d\n\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"../src/randr.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"get_output_containing\0" as *const u8 as *const ::core::ffi::c_char,
                129 as ::core::ffi::c_int,
                x,
                y,
                (*output).rect.x,
                (*output).rect.y,
                (*output).rect.width,
                (*output).rect.height,
            );
            if x as uint32_t >= (*output).rect.x
                && (x as uint32_t) < (*output).rect.x.wrapping_add((*output).rect.width)
                && y as uint32_t >= (*output).rect.y
                && (y as uint32_t) < (*output).rect.y.wrapping_add((*output).rect.height)
            {
                return output;
            }
        }
        output = (*output).outputs.tqe_next as *mut Output;
    }
    return ::core::ptr::null_mut::<Output>();
}
#[no_mangle]
pub unsafe extern "C" fn get_output_from_rect(mut rect: Rect) -> *mut Output {
    let mut mid_x: ::core::ffi::c_uint = (rect.x as ::core::ffi::c_uint)
        .wrapping_add((rect.width as ::core::ffi::c_uint).wrapping_div(2 as ::core::ffi::c_uint));
    let mut mid_y: ::core::ffi::c_uint = (rect.y as ::core::ffi::c_uint)
        .wrapping_add((rect.height as ::core::ffi::c_uint).wrapping_div(2 as ::core::ffi::c_uint));
    let mut output: *mut Output = get_output_containing(mid_x, mid_y);
    return if !output.is_null() {
        output
    } else {
        output_containing_rect(rect)
    };
}
#[no_mangle]
pub unsafe extern "C" fn get_output_with_dimensions(mut rect: Rect) -> *mut Output {
    let mut output: *mut Output = ::core::ptr::null_mut::<Output>();
    output = outputs.tqh_first as *mut Output;
    while !output.is_null() {
        if (*output).active {
            debuglog(
                b"%s:%s:%d - comparing x=%d y=%d %dx%d with x=%d and y=%d %dx%d\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"../src/randr.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"get_output_with_dimensions\0" as *const u8 as *const ::core::ffi::c_char,
                166 as ::core::ffi::c_int,
                rect.x,
                rect.y,
                rect.width,
                rect.height,
                (*output).rect.x,
                (*output).rect.y,
                (*output).rect.width,
                (*output).rect.height,
            );
            if rect.x == (*output).rect.x
                && rect.width == (*output).rect.width
                && rect.y == (*output).rect.y
                && rect.height == (*output).rect.height
            {
                return output;
            }
        }
        output = (*output).outputs.tqe_next as *mut Output;
    }
    return ::core::ptr::null_mut::<Output>();
}
#[no_mangle]
pub unsafe extern "C" fn output_containing_rect(mut rect: Rect) -> *mut Output {
    let mut output: *mut Output = ::core::ptr::null_mut::<Output>();
    let lx: ::core::ffi::c_int = rect.x as ::core::ffi::c_int;
    let uy: ::core::ffi::c_int = rect.y as ::core::ffi::c_int;
    let rx: ::core::ffi::c_int = rect.x.wrapping_add(rect.width) as ::core::ffi::c_int;
    let by: ::core::ffi::c_int = rect.y.wrapping_add(rect.height) as ::core::ffi::c_int;
    let mut result: *mut Output = ::core::ptr::null_mut::<Output>();
    output = outputs.tqh_first as *mut Output;
    while !output.is_null() {
        if (*output).active {
            let lx_o: ::core::ffi::c_int = (*output).rect.x as ::core::ffi::c_int;
            let uy_o: ::core::ffi::c_int = (*output).rect.y as ::core::ffi::c_int;
            let rx_o: ::core::ffi::c_int =
                (*output).rect.x.wrapping_add((*output).rect.width) as ::core::ffi::c_int;
            let by_o: ::core::ffi::c_int =
                (*output).rect.y.wrapping_add((*output).rect.height) as ::core::ffi::c_int;
            debuglog(
                b"%s:%s:%d - comparing x=%d y=%d with x=%d and y=%d width %d height %d\n\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"../src/randr.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"output_containing_rect\0" as *const u8 as *const ::core::ffi::c_char,
                199 as ::core::ffi::c_int,
                rect.x,
                rect.y,
                (*output).rect.x,
                (*output).rect.y,
                (*output).rect.width,
                (*output).rect.height,
            );
            let left: ::core::ffi::c_int = max(lx, lx_o) as ::core::ffi::c_int;
            let right: ::core::ffi::c_int = min(rx, rx_o) as ::core::ffi::c_int;
            let bottom: ::core::ffi::c_int = min(by, by_o) as ::core::ffi::c_int;
            let top: ::core::ffi::c_int = max(uy, uy_o) as ::core::ffi::c_int;
            if left < right && bottom > top {
                let max_area: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
                let area: ::core::ffi::c_long =
                    ((right - left) * (bottom - top)) as ::core::ffi::c_long;
                if area > max_area {
                    result = output;
                }
            }
        }
        output = (*output).outputs.tqe_next as *mut Output;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn get_output_next_wrap(
    mut direction: direction_t,
    mut current: *mut Output,
) -> *mut Output {
    let mut best: *mut Output = get_output_next(direction, current, CLOSEST_OUTPUT);
    if best.is_null() {
        let mut opposite: direction_t = D_LEFT;
        if direction as ::core::ffi::c_uint == D_RIGHT as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            opposite = D_LEFT;
        } else if direction as ::core::ffi::c_uint
            == D_LEFT as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            opposite = D_RIGHT;
        } else if direction as ::core::ffi::c_uint
            == D_DOWN as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            opposite = D_UP;
        } else {
            opposite = D_DOWN;
        }
        best = get_output_next(opposite, current, FARTHEST_OUTPUT);
    }
    if best.is_null() {
        best = current;
    }
    debuglog(
        b"%s:%s:%d - current = %s, best = %s\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/randr.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"get_output_next_wrap\0" as *const u8 as *const ::core::ffi::c_char,
        244 as ::core::ffi::c_int,
        output_primary_name(current),
        output_primary_name(best),
    );
    return best;
}
#[no_mangle]
pub unsafe extern "C" fn get_output_next(
    direction: direction_t,
    mut current: *mut Output,
    mut close_far: output_close_far_t,
) -> *mut Output {
    let mut cur: *const Rect = &raw mut (*current).rect;
    let mut output: *mut Output = ::core::ptr::null_mut::<Output>();
    let mut best: *mut Output = ::core::ptr::null_mut::<Output>();
    let mut current_block_3: u64;
    output = outputs.tqh_first as *mut Output;
    while !output.is_null() {
        if (*output).active {
            let mut other: *const Rect = &raw mut (*output).rect;
            if direction as ::core::ffi::c_uint
                == D_RIGHT as ::core::ffi::c_int as ::core::ffi::c_uint
                && (*other).x > (*cur).x
                || direction as ::core::ffi::c_uint
                    == D_LEFT as ::core::ffi::c_int as ::core::ffi::c_uint
                    && (*other).x < (*cur).x
            {
                if (*other).y.wrapping_add((*other).height) <= (*cur).y
                    || (*cur).y.wrapping_add((*cur).height) <= (*other).y
                {
                    current_block_3 = 16668937799742929182;
                } else {
                    current_block_3 = 11650488183268122163;
                }
            } else if direction as ::core::ffi::c_uint
                == D_DOWN as ::core::ffi::c_int as ::core::ffi::c_uint
                && (*other).y > (*cur).y
                || direction as ::core::ffi::c_uint
                    == D_UP as ::core::ffi::c_int as ::core::ffi::c_uint
                    && (*other).y < (*cur).y
            {
                if (*other).x.wrapping_add((*other).width) <= (*cur).x
                    || (*cur).x.wrapping_add((*cur).width) <= (*other).x
                {
                    current_block_3 = 16668937799742929182;
                } else {
                    current_block_3 = 11650488183268122163;
                }
            } else {
                current_block_3 = 16668937799742929182;
            }
            match current_block_3 {
                16668937799742929182 => {}
                _ => {
                    if best.is_null() {
                        best = output;
                    } else if close_far as ::core::ffi::c_uint
                        == CLOSEST_OUTPUT as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        if direction as ::core::ffi::c_uint
                            == D_RIGHT as ::core::ffi::c_int as ::core::ffi::c_uint
                            && (*other).x < (*best).rect.x
                            || direction as ::core::ffi::c_uint
                                == D_LEFT as ::core::ffi::c_int as ::core::ffi::c_uint
                                && (*other).x > (*best).rect.x
                            || direction as ::core::ffi::c_uint
                                == D_DOWN as ::core::ffi::c_int as ::core::ffi::c_uint
                                && (*other).y < (*best).rect.y
                            || direction as ::core::ffi::c_uint
                                == D_UP as ::core::ffi::c_int as ::core::ffi::c_uint
                                && (*other).y > (*best).rect.y
                        {
                            best = output;
                        }
                    } else if direction as ::core::ffi::c_uint
                        == D_RIGHT as ::core::ffi::c_int as ::core::ffi::c_uint
                        && (*other).x > (*best).rect.x
                        || direction as ::core::ffi::c_uint
                            == D_LEFT as ::core::ffi::c_int as ::core::ffi::c_uint
                            && (*other).x < (*best).rect.x
                        || direction as ::core::ffi::c_uint
                            == D_DOWN as ::core::ffi::c_int as ::core::ffi::c_uint
                            && (*other).y > (*best).rect.y
                        || direction as ::core::ffi::c_uint
                            == D_UP as ::core::ffi::c_int as ::core::ffi::c_uint
                            && (*other).y < (*best).rect.y
                    {
                        best = output;
                    }
                }
            }
        }
        output = (*output).outputs.tqe_next as *mut Output;
    }
    debuglog(
        b"%s:%s:%d - current = %s, best = %s\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/randr.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"get_output_next\0" as *const u8 as *const ::core::ffi::c_char,
        317 as ::core::ffi::c_int,
        output_primary_name(current),
        if !best.is_null() {
            output_primary_name(best) as *const ::core::ffi::c_char
        } else {
            b"NULL\0" as *const u8 as *const ::core::ffi::c_char
        },
    );
    return best;
}
#[no_mangle]
pub unsafe extern "C" fn create_root_output(mut conn_0: *mut xcb_connection_t) -> *mut Output {
    let mut s: *mut Output =
        scalloc(1 as size_t, ::core::mem::size_of::<Output>() as size_t) as *mut Output;
    (*s).active = false_0 != 0;
    (*s).rect.x = 0 as uint32_t;
    (*s).rect.y = 0 as uint32_t;
    (*s).rect.width = (*root_screen).width_in_pixels as uint32_t;
    (*s).rect.height = (*root_screen).height_in_pixels as uint32_t;
    let mut output_name: *mut output_name =
        scalloc(1 as size_t, ::core::mem::size_of::<output_name>() as size_t) as *mut output_name;
    (*output_name).name =
        b"xroot-0\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
    (*s).names_head.slh_first = ::core::ptr::null_mut::<output_name>();
    (*output_name).names.sle_next = (*s).names_head.slh_first;
    (*s).names_head.slh_first = output_name;
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn output_init_con(mut output: *mut Output) {
    let mut con: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut current: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut reused: bool = false_0 != 0;
    debuglog(
        b"%s:%s:%d - init_con for output %s\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/randr.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"output_init_con\0" as *const u8 as *const ::core::ffi::c_char,
        351 as ::core::ffi::c_int,
        output_primary_name(output),
    );
    current = (*croot).nodes_head.tqh_first as *mut Con;
    while !current.is_null() {
        if strcmp((*current).name, output_primary_name(output)) != 0 as ::core::ffi::c_int {
            current = (*current).nodes.tqe_next as *mut Con;
        } else {
            con = current;
            reused = true_0 != 0;
            debuglog(
                b"%s:%s:%d - Using existing con %p / %s\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"../src/randr.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"output_init_con\0" as *const u8 as *const ::core::ffi::c_char,
                362 as ::core::ffi::c_int,
                con,
                (*con).name,
            );
            break;
        }
    }
    if con.is_null() {
        con = con_new(croot, ::core::ptr::null_mut::<i3Window>());
        free((*con).name as *mut ::core::ffi::c_void);
        (*con).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
        (*con).name = sstrdup(output_primary_name(output));
        (*con).type_0 = CT_OUTPUT;
        (*con).layout = L_OUTPUT;
        con_fix_percent(croot);
    }
    (*con).rect = (*output).rect as Rect;
    (*output).con = con;
    let mut name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    sasprintf(
        &raw mut name,
        b"[i3 con] output %s\0" as *const u8 as *const ::core::ffi::c_char,
        (*con).name,
    );
    x_set_name(con, name);
    free(name as *mut ::core::ffi::c_void);
    name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if reused {
        debuglog(
            b"%s:%s:%d - Not adding workspace, this was a reused con\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/randr.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"output_init_con\0" as *const u8 as *const ::core::ffi::c_char,
            383 as ::core::ffi::c_int,
        );
        return;
    }
    debuglog(
        b"%s:%s:%d - Changing layout, adding top/bottom dockarea\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/randr.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"output_init_con\0" as *const u8 as *const ::core::ffi::c_char,
        387 as ::core::ffi::c_int,
    );
    let mut topdock: *mut Con = con_new(
        ::core::ptr::null_mut::<Con>(),
        ::core::ptr::null_mut::<i3Window>(),
    );
    (*topdock).type_0 = CT_DOCKAREA;
    (*topdock).layout = L_DOCKAREA;
    let mut match_0: *mut Match =
        scalloc(1 as size_t, ::core::mem::size_of::<Match>() as size_t) as *mut Match;
    match_init(match_0);
    (*match_0).dock = M_DOCK_TOP;
    (*match_0).insert_where = M_BELOW;
    (*match_0).matches.tqe_next = ::core::ptr::null_mut::<Match>();
    (*match_0).matches.tqe_prev = (*topdock).swallow_head.tqh_last;
    *(*topdock).swallow_head.tqh_last = match_0 as *mut Match;
    (*topdock).swallow_head.tqh_last = &raw mut (*match_0).matches.tqe_next;
    free((*topdock).name as *mut ::core::ffi::c_void);
    (*topdock).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*topdock).name = sstrdup(b"topdock\0" as *const u8 as *const ::core::ffi::c_char);
    sasprintf(
        &raw mut name,
        b"[i3 con] top dockarea %s\0" as *const u8 as *const ::core::ffi::c_char,
        (*con).name,
    );
    x_set_name(topdock, name);
    free(name as *mut ::core::ffi::c_void);
    name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    debuglog(
        b"%s:%s:%d - attaching\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/randr.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"output_init_con\0" as *const u8 as *const ::core::ffi::c_char,
        404 as ::core::ffi::c_int,
    );
    con_attach(topdock, con, false_0 != 0);
    debuglog(
        b"%s:%s:%d - adding main content container\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/randr.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"output_init_con\0" as *const u8 as *const ::core::ffi::c_char,
        409 as ::core::ffi::c_int,
    );
    let mut content: *mut Con = con_new(
        ::core::ptr::null_mut::<Con>(),
        ::core::ptr::null_mut::<i3Window>(),
    );
    (*content).type_0 = CT_CON;
    (*content).layout = L_SPLITH;
    free((*content).name as *mut ::core::ffi::c_void);
    (*content).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*content).name = sstrdup(b"content\0" as *const u8 as *const ::core::ffi::c_char);
    sasprintf(
        &raw mut name,
        b"[i3 con] content %s\0" as *const u8 as *const ::core::ffi::c_char,
        (*con).name,
    );
    x_set_name(content, name);
    free(name as *mut ::core::ffi::c_void);
    name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    con_attach(content, con, false_0 != 0);
    let mut bottomdock: *mut Con = con_new(
        ::core::ptr::null_mut::<Con>(),
        ::core::ptr::null_mut::<i3Window>(),
    );
    (*bottomdock).type_0 = CT_DOCKAREA;
    (*bottomdock).layout = L_DOCKAREA;
    match_0 = scalloc(1 as size_t, ::core::mem::size_of::<Match>() as size_t) as *mut Match;
    match_init(match_0);
    (*match_0).dock = M_DOCK_BOTTOM;
    (*match_0).insert_where = M_BELOW;
    (*match_0).matches.tqe_next = ::core::ptr::null_mut::<Match>();
    (*match_0).matches.tqe_prev = (*bottomdock).swallow_head.tqh_last;
    *(*bottomdock).swallow_head.tqh_last = match_0 as *mut Match;
    (*bottomdock).swallow_head.tqh_last = &raw mut (*match_0).matches.tqe_next;
    free((*bottomdock).name as *mut ::core::ffi::c_void);
    (*bottomdock).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*bottomdock).name = sstrdup(b"bottomdock\0" as *const u8 as *const ::core::ffi::c_char);
    sasprintf(
        &raw mut name,
        b"[i3 con] bottom dockarea %s\0" as *const u8 as *const ::core::ffi::c_char,
        (*con).name,
    );
    x_set_name(bottomdock, name);
    free(name as *mut ::core::ffi::c_void);
    name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    debuglog(
        b"%s:%s:%d - attaching\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/randr.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"output_init_con\0" as *const u8 as *const ::core::ffi::c_char,
        438 as ::core::ffi::c_int,
    );
    con_attach(bottomdock, con, false_0 != 0);
    if !(*content).focused.tqe_next.is_null() {
        (*(*content).focused.tqe_next).focused.tqe_prev = (*content).focused.tqe_prev;
    } else {
        (*con).focus_head.tqh_last = (*content).focused.tqe_prev;
    }
    *(*content).focused.tqe_prev = (*content).focused.tqe_next;
    (*content).focused.tqe_next = (*con).focus_head.tqh_first;
    if !(*content).focused.tqe_next.is_null() {
        (*(*con).focus_head.tqh_first).focused.tqe_prev = &raw mut (*content).focused.tqe_next;
    } else {
        (*con).focus_head.tqh_last = &raw mut (*content).focused.tqe_next;
    }
    (*con).focus_head.tqh_first = content as *mut Con;
    (*content).focused.tqe_prev = &raw mut (*con).focus_head.tqh_first;
}
#[no_mangle]
pub unsafe extern "C" fn init_ws_for_output(mut output: *mut Output) {
    let mut assignment: *mut Workspace_Assignment = ::core::ptr::null_mut::<Workspace_Assignment>();
    let mut current_block: u64;
    let mut content: *mut Con = output_get_content((*output).con);
    let mut previous_focus: *mut Con = con_get_workspace(focused);
    let mut workspace: *mut Con = ::core::ptr::null_mut::<Con>();
    workspace = all_cons.tqh_first as *mut Con;
    while !workspace.is_null() {
        if !((*workspace).type_0 as ::core::ffi::c_uint
            != CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
            || con_is_internal(workspace) as ::core::ffi::c_int != 0)
        {
            let mut workspace_out: *mut Con =
                get_assigned_output((*workspace).name, (*workspace).num as ::core::ffi::c_long);
            if !((*output).con != workspace_out) {
                debuglog(
                    b"%s:%s:%d - Moving workspace \"%s\" from output \"%s\" to \"%s\" due to assignment\n\0"
                        as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    b"../src/randr.c\0" as *const u8 as *const ::core::ffi::c_char,
                    b"init_ws_for_output\0" as *const u8 as *const ::core::ffi::c_char,
                    479 as ::core::ffi::c_int,
                    (*workspace).name,
                    output_primary_name(get_output_for_con(workspace)),
                    output_primary_name(output),
                );
                (*content).rect = (*(*output).con).rect;
                workspace_move_to_output(workspace, output);
            }
        }
        workspace = (*workspace).all_cons.tqe_next as *mut Con;
    }
    focused = content;
    if !(*content).nodes_head.tqh_first.is_null() {
        let mut visible: *mut Con = ::core::ptr::null_mut::<Con>();
        let mut child: *mut Con = -(1 as ::core::ffi::c_int) as *mut Con;
        while child == -(1 as ::core::ffi::c_int) as *mut Con && {
            child = ::core::ptr::null_mut::<Con>();
            true_0 != 0
        } {
            child = (*content).nodes_head.tqh_first as *mut Con;
            while !child.is_null() {
                if !((*child).fullscreen_mode as ::core::ffi::c_uint
                    == CF_OUTPUT as ::core::ffi::c_int as ::core::ffi::c_uint)
                {
                    child = (*child).nodes.tqe_next as *mut Con;
                } else {
                    visible = child;
                    break;
                }
            }
        }
        if visible.is_null() {
            visible = (*content).nodes_head.tqh_first as *mut Con;
            workspace_show(visible);
        }
    } else {
        assignment = ::core::ptr::null_mut::<Workspace_Assignment>();
        assignment = ws_assignments.tqh_first;
        loop {
            if assignment.is_null() {
                current_block = 11057878835866523405;
                break;
            }
            if !output_triggers_assignment(output, assignment) {
                assignment = (*assignment).ws_assignments.tqe_next;
            } else {
                verboselog(
                    b"Initializing first assigned workspace \"%s\" for output \"%s\"\n\0"
                        as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    (*assignment).name,
                    (*assignment).output,
                );
                workspace_show_by_name((*assignment).name);
                current_block = 4825863742504697761;
                break;
            }
        }
        match current_block {
            4825863742504697761 => {}
            _ => {
                debuglog(
                    b"%s:%s:%d - Now adding a workspace\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    b"../src/randr.c\0" as *const u8 as *const ::core::ffi::c_char,
                    b"init_ws_for_output\0" as *const u8 as *const ::core::ffi::c_char,
                    518 as ::core::ffi::c_int,
                );
                workspace_show(create_workspace_on_output(output, content));
            }
        }
    }
    if !previous_focus.is_null() {
        workspace_show(previous_focus);
    }
}
unsafe extern "C" fn output_change_mode(
    mut conn_0: *mut xcb_connection_t,
    mut output: *mut Output,
) {
    debuglog(
        b"%s:%s:%d - Output mode changed, updating rect\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/randr.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"output_change_mode\0" as *const u8 as *const ::core::ffi::c_char,
        539 as ::core::ffi::c_int,
    );
    if (*output).con != ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut Con {
    } else {
        __assert_fail(
            b"output->con != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            b"../src/randr.c\0" as *const u8 as *const ::core::ffi::c_char,
            540 as ::core::ffi::c_uint,
            b"void output_change_mode(xcb_connection_t *, Output *)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    };
    (*(*output).con).rect = (*output).rect as Rect;
    let mut workspace: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut child: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut content: *const Con = output_get_content((*output).con);
    workspace = (*content).nodes_head.tqh_first as *mut Con;
    while !workspace.is_null() {
        child = (*workspace).floating_head.tqh_first as *mut Con;
        while !child.is_null() {
            floating_fix_coordinates(
                child,
                &raw mut (*workspace).rect,
                &raw mut (*(*output).con).rect,
            );
            child = (*child).floating_windows.tqe_next as *mut Con;
        }
        workspace = (*workspace).nodes.tqe_next as *mut Con;
    }
    if config.default_orientation == NO_ORIENTATION as ::core::ffi::c_int {
        workspace = (*content).nodes_head.tqh_first as *mut Con;
        while !workspace.is_null() {
            if !(con_num_children(workspace) > 1 as ::core::ffi::c_int) {
                (*workspace).layout = (if (*output).rect.height > (*output).rect.width {
                    L_SPLITV as ::core::ffi::c_int
                } else {
                    L_SPLITH as ::core::ffi::c_int
                }) as layout_t;
                debuglog(
                    b"%s:%s:%d - Setting workspace [%d,%s]'s layout to %d.\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    b"../src/randr.c\0" as *const u8 as *const ::core::ffi::c_char,
                    b"output_change_mode\0" as *const u8 as *const ::core::ffi::c_char,
                    568 as ::core::ffi::c_int,
                    (*workspace).num,
                    (*workspace).name,
                    (*workspace).layout as ::core::ffi::c_uint,
                );
                child = (*workspace).nodes_head.tqh_first as *mut Con;
                if !child.is_null() {
                    if (*child).layout as ::core::ffi::c_uint
                        == L_SPLITV as ::core::ffi::c_int as ::core::ffi::c_uint
                        || (*child).layout as ::core::ffi::c_uint
                            == L_SPLITH as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        (*child).layout = (*workspace).layout;
                    }
                    debuglog(
                        b"%s:%s:%d - Setting child [%d,%s]'s layout to %d.\n\0" as *const u8
                            as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        b"../src/randr.c\0" as *const u8 as *const ::core::ffi::c_char,
                        b"output_change_mode\0" as *const u8 as *const ::core::ffi::c_char,
                        573 as ::core::ffi::c_int,
                        (*child).num,
                        (*child).name,
                        (*child).layout as ::core::ffi::c_uint,
                    );
                }
            }
            workspace = (*workspace).nodes.tqe_next as *mut Con;
        }
    }
}
unsafe extern "C" fn randr_query_outputs_15() -> bool {
    if !has_randr_1_5 {
        return false_0 != 0;
    }
    debuglog(
        b"%s:%s:%d - Querying outputs using RandR 1.5\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/randr.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"randr_query_outputs_15\0" as *const u8 as *const ::core::ffi::c_char,
        593 as ::core::ffi::c_int,
    );
    let mut err: *mut xcb_generic_error_t = ::core::ptr::null_mut::<xcb_generic_error_t>();
    let mut monitors: *mut xcb_randr_get_monitors_reply_t = xcb_randr_get_monitors_reply(
        conn,
        xcb_randr_get_monitors(conn, root, true_0 as uint8_t),
        &raw mut err,
    );
    if !err.is_null() {
        errorlog(
            b"ERROR: Could not get RandR monitors: X11 error code %d\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            (*err).error_code as ::core::ffi::c_int,
        );
        free(err as *mut ::core::ffi::c_void);
        return false_0 != 0;
    }
    let mut output: *mut Output = ::core::ptr::null_mut::<Output>();
    output = outputs.tqh_first as *mut Output;
    while !output.is_null() {
        if output != root_output {
            (*output).to_be_disabled = true_0 != 0;
        }
        output = (*output).outputs.tqe_next as *mut Output;
    }
    debuglog(
        b"%s:%s:%d - %d RandR monitors found (timestamp %d)\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/randr.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"randr_query_outputs_15\0" as *const u8 as *const ::core::ffi::c_char,
        616 as ::core::ffi::c_int,
        xcb_randr_get_monitors_monitors_length(monitors),
        (*monitors).timestamp,
    );
    let mut iter: xcb_randr_monitor_info_iterator_t = xcb_randr_monitor_info_iterator_t {
        data: ::core::ptr::null_mut::<xcb_randr_monitor_info_t>(),
        rem: 0,
        index: 0,
    };
    iter = xcb_randr_get_monitors_monitors_iterator(monitors);
    while iter.rem != 0 {
        let mut monitor_info: *const xcb_randr_monitor_info_t = iter.data;
        let mut atom_reply: *mut xcb_get_atom_name_reply_t = xcb_get_atom_name_reply(
            conn,
            xcb_get_atom_name(conn, (*monitor_info).name),
            &raw mut err,
        );
        if !err.is_null() {
            errorlog(
                b"ERROR: Could not get RandR monitor name: X11 error code %d\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                (*err).error_code as ::core::ffi::c_int,
            );
            free(err as *mut ::core::ffi::c_void);
        } else {
            let mut name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut name,
                b"%.*s\0" as *const u8 as *const ::core::ffi::c_char,
                xcb_get_atom_name_name_length(atom_reply),
                xcb_get_atom_name_name(atom_reply),
            );
            free(atom_reply as *mut ::core::ffi::c_void);
            let mut new: *mut Output = get_output_by_name(name, false_0 != 0);
            if new.is_null() {
                new =
                    scalloc(1 as size_t, ::core::mem::size_of::<Output>() as size_t) as *mut Output;
                (*new).names_head.slh_first = ::core::ptr::null_mut::<output_name>();
                let mut randr_outputs: *mut xcb_randr_output_t =
                    xcb_randr_monitor_info_outputs(monitor_info);
                let mut randr_output_len: ::core::ffi::c_int =
                    xcb_randr_monitor_info_outputs_length(monitor_info);
                let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while i < randr_output_len {
                    let mut randr_output: xcb_randr_output_t = *randr_outputs.offset(i as isize);
                    let mut info: *mut xcb_randr_get_output_info_reply_t =
                        xcb_randr_get_output_info_reply(
                            conn,
                            xcb_randr_get_output_info(conn, randr_output, (*monitors).timestamp),
                            ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
                        );
                    if !info.is_null() && (*info).crtc as ::core::ffi::c_long != XCB_NONE {
                        let mut oname: *mut ::core::ffi::c_char =
                            ::core::ptr::null_mut::<::core::ffi::c_char>();
                        sasprintf(
                            &raw mut oname,
                            b"%.*s\0" as *const u8 as *const ::core::ffi::c_char,
                            xcb_randr_get_output_info_name_length(info),
                            xcb_randr_get_output_info_name(info),
                        );
                        if strcmp(name, oname) != 0 as ::core::ffi::c_int {
                            let mut output_name: *mut output_name = scalloc(
                                1 as size_t,
                                ::core::mem::size_of::<output_name>() as size_t,
                            )
                                as *mut output_name;
                            (*output_name).name = sstrdup(oname);
                            (*output_name).names.sle_next = (*new).names_head.slh_first;
                            (*new).names_head.slh_first = output_name;
                        }
                        free(oname as *mut ::core::ffi::c_void);
                    }
                    free(info as *mut ::core::ffi::c_void);
                    info = ::core::ptr::null_mut::<xcb_randr_get_output_info_reply_t>();
                    i += 1;
                }
                let mut output_name_0: *mut output_name =
                    scalloc(1 as size_t, ::core::mem::size_of::<output_name>() as size_t)
                        as *mut output_name;
                (*output_name_0).name = sstrdup(name);
                (*output_name_0).names.sle_next = (*new).names_head.slh_first;
                (*new).names_head.slh_first = output_name_0;
                if (*monitor_info).primary != 0 {
                    (*new).outputs.tqe_next = outputs.tqh_first;
                    if !(*new).outputs.tqe_next.is_null() {
                        (*outputs.tqh_first).outputs.tqe_prev = &raw mut (*new).outputs.tqe_next;
                    } else {
                        outputs.tqh_last = &raw mut (*new).outputs.tqe_next;
                    }
                    outputs.tqh_first = new as *mut xoutput;
                    (*new).outputs.tqe_prev = &raw mut outputs.tqh_first;
                } else {
                    (*new).outputs.tqe_next = ::core::ptr::null_mut::<xoutput>();
                    (*new).outputs.tqe_prev = outputs.tqh_last;
                    *outputs.tqh_last = new as *mut xoutput;
                    outputs.tqh_last = &raw mut (*new).outputs.tqe_next;
                }
            }
            (*new).active = true_0 != 0;
            (*new).to_be_disabled = false_0 != 0;
            (*new).primary = (*monitor_info).primary != 0;
            let update_x: bool =
                update_if_necessary(&raw mut (*new).rect.x, (*monitor_info).x as uint32_t) as bool;
            let update_y: bool =
                update_if_necessary(&raw mut (*new).rect.y, (*monitor_info).y as uint32_t) as bool;
            let update_w: bool = update_if_necessary(
                &raw mut (*new).rect.width,
                (*monitor_info).width as uint32_t,
            ) as bool;
            let update_h: bool = update_if_necessary(
                &raw mut (*new).rect.height,
                (*monitor_info).height as uint32_t,
            ) as bool;
            (*new).changed = update_x as ::core::ffi::c_int != 0
                || update_y as ::core::ffi::c_int != 0
                || update_w as ::core::ffi::c_int != 0
                || update_h as ::core::ffi::c_int != 0;
            debuglog(
                b"%s:%s:%d - name %s, x %d, y %d, width %d px, height %d px, width %d mm, height %d mm, primary %d, automatic %d\n\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"../src/randr.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"randr_query_outputs_15\0" as *const u8 as *const ::core::ffi::c_char,
                699 as ::core::ffi::c_int,
                name,
                (*monitor_info).x as ::core::ffi::c_int,
                (*monitor_info).y as ::core::ffi::c_int,
                (*monitor_info).width as ::core::ffi::c_int,
                (*monitor_info).height as ::core::ffi::c_int,
                (*monitor_info).width_in_millimeters,
                (*monitor_info).height_in_millimeters,
                (*monitor_info).primary as ::core::ffi::c_int,
                (*monitor_info).automatic as ::core::ffi::c_int,
            );
            free(name as *mut ::core::ffi::c_void);
        }
        xcb_randr_monitor_info_next(&raw mut iter);
    }
    free(monitors as *mut ::core::ffi::c_void);
    return true_0 != 0;
}
unsafe extern "C" fn handle_output(
    mut conn_0: *mut xcb_connection_t,
    mut id: xcb_randr_output_t,
    mut output: *mut xcb_randr_get_output_info_reply_t,
    mut cts: xcb_timestamp_t,
    mut res: *mut xcb_randr_get_screen_resources_current_reply_t,
) {
    let mut crtc: *mut xcb_randr_get_crtc_info_reply_t =
        ::core::ptr::null_mut::<xcb_randr_get_crtc_info_reply_t>();
    let mut new: *mut Output = get_output_by_id(id);
    let mut existing: bool = !new.is_null();
    if !existing {
        new = scalloc(1 as size_t, ::core::mem::size_of::<Output>() as size_t) as *mut Output;
        (*new).names_head.slh_first = ::core::ptr::null_mut::<output_name>();
    }
    (*new).id = id;
    (*new).primary = !primary.is_null() && (*primary).output == id;
    while !(*new).names_head.slh_first.is_null() {
        free((*(*new).names_head.slh_first).name as *mut ::core::ffi::c_void);
        (*(*new).names_head.slh_first).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut old_head: *mut output_name = (*new).names_head.slh_first;
        (*new).names_head.slh_first = (*(*new).names_head.slh_first).names.sle_next;
        free(old_head as *mut ::core::ffi::c_void);
        old_head = ::core::ptr::null_mut::<output_name>();
    }
    let mut output_name: *mut output_name =
        scalloc(1 as size_t, ::core::mem::size_of::<output_name>() as size_t) as *mut output_name;
    sasprintf(
        &raw mut (*output_name).name,
        b"%.*s\0" as *const u8 as *const ::core::ffi::c_char,
        xcb_randr_get_output_info_name_length(output),
        xcb_randr_get_output_info_name(output),
    );
    (*output_name).names.sle_next = (*new).names_head.slh_first;
    (*new).names_head.slh_first = output_name;
    debuglog(
        b"%s:%s:%d - found output with name %s\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/randr.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"handle_output\0" as *const u8 as *const ::core::ffi::c_char,
        742 as ::core::ffi::c_int,
        output_primary_name(new),
    );
    if (*output).crtc as ::core::ffi::c_long == XCB_NONE {
        if !existing {
            if (*new).primary {
                (*new).outputs.tqe_next = outputs.tqh_first;
                if !(*new).outputs.tqe_next.is_null() {
                    (*outputs.tqh_first).outputs.tqe_prev = &raw mut (*new).outputs.tqe_next;
                } else {
                    outputs.tqh_last = &raw mut (*new).outputs.tqe_next;
                }
                outputs.tqh_first = new as *mut xoutput;
                (*new).outputs.tqe_prev = &raw mut outputs.tqh_first;
            } else {
                (*new).outputs.tqe_next = ::core::ptr::null_mut::<xoutput>();
                (*new).outputs.tqe_prev = outputs.tqh_last;
                *outputs.tqh_last = new as *mut xoutput;
                outputs.tqh_last = &raw mut (*new).outputs.tqe_next;
            }
        } else if (*new).active {
            (*new).to_be_disabled = true_0 != 0;
        }
        return;
    }
    let icookie: xcb_randr_get_crtc_info_cookie_t =
        xcb_randr_get_crtc_info(conn_0, (*output).crtc, cts) as xcb_randr_get_crtc_info_cookie_t;
    crtc = xcb_randr_get_crtc_info_reply(
        conn_0,
        icookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if crtc.is_null() {
        debuglog(
            b"%s:%s:%d - Skipping output %s: could not get CRTC (%p)\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/randr.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"handle_output\0" as *const u8 as *const ::core::ffi::c_char,
            763 as ::core::ffi::c_int,
            output_primary_name(new),
            crtc,
        );
        return;
    }
    let update_x: bool = update_if_necessary(&raw mut (*new).rect.x, (*crtc).x as uint32_t) as bool;
    let update_y: bool = update_if_necessary(&raw mut (*new).rect.y, (*crtc).y as uint32_t) as bool;
    let update_w: bool =
        update_if_necessary(&raw mut (*new).rect.width, (*crtc).width as uint32_t) as bool;
    let update_h: bool =
        update_if_necessary(&raw mut (*new).rect.height, (*crtc).height as uint32_t) as bool;
    let updated: bool = update_x as ::core::ffi::c_int != 0
        || update_y as ::core::ffi::c_int != 0
        || update_w as ::core::ffi::c_int != 0
        || update_h as ::core::ffi::c_int != 0;
    free(crtc as *mut ::core::ffi::c_void);
    (*new).active = (*new).rect.width != 0 as uint32_t && (*new).rect.height != 0 as uint32_t;
    if !(*new).active {
        debuglog(
            b"%s:%s:%d - width/height 0/0, disabling output\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/randr.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"handle_output\0" as *const u8 as *const ::core::ffi::c_char,
            775 as ::core::ffi::c_int,
        );
        return;
    }
    debuglog(
        b"%s:%s:%d - mode: %dx%d+%d+%d\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/randr.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"handle_output\0" as *const u8 as *const ::core::ffi::c_char,
        780 as ::core::ffi::c_int,
        (*new).rect.width,
        (*new).rect.height,
        (*new).rect.x,
        (*new).rect.y,
    );
    if !updated || !existing {
        if !existing {
            if (*new).primary {
                (*new).outputs.tqe_next = outputs.tqh_first;
                if !(*new).outputs.tqe_next.is_null() {
                    (*outputs.tqh_first).outputs.tqe_prev = &raw mut (*new).outputs.tqe_next;
                } else {
                    outputs.tqh_last = &raw mut (*new).outputs.tqe_next;
                }
                outputs.tqh_first = new as *mut xoutput;
                (*new).outputs.tqe_prev = &raw mut outputs.tqh_first;
            } else {
                (*new).outputs.tqe_next = ::core::ptr::null_mut::<xoutput>();
                (*new).outputs.tqe_prev = outputs.tqh_last;
                *outputs.tqh_last = new as *mut xoutput;
                outputs.tqh_last = &raw mut (*new).outputs.tqe_next;
            }
        }
        return;
    }
    (*new).changed = true_0 != 0;
}
unsafe extern "C" fn randr_query_outputs_14() {
    debuglog(
        b"%s:%s:%d - Querying outputs using RandR \xE2\x89\xA4 1.4\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/randr.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"randr_query_outputs_14\0" as *const u8 as *const ::core::ffi::c_char,
        804 as ::core::ffi::c_int,
    );
    let rcookie: xcb_randr_get_screen_resources_current_cookie_t =
        xcb_randr_get_screen_resources_current(conn, root)
            as xcb_randr_get_screen_resources_current_cookie_t;
    let pcookie: xcb_randr_get_output_primary_cookie_t =
        xcb_randr_get_output_primary(conn, root) as xcb_randr_get_output_primary_cookie_t;
    primary = xcb_randr_get_output_primary_reply(
        conn,
        pcookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if primary.is_null() {
        errorlog(
            b"ERROR: Could not get RandR primary output\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    } else {
        debuglog(
            b"%s:%s:%d - primary output is %08x\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b"../src/randr.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"randr_query_outputs_14\0" as *const u8 as *const ::core::ffi::c_char,
            813 as ::core::ffi::c_int,
            (*primary).output,
        );
    }
    let mut res: *mut xcb_randr_get_screen_resources_current_reply_t =
        xcb_randr_get_screen_resources_current_reply(
            conn,
            rcookie,
            ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
        );
    if res.is_null() {
        errorlog(
            b"ERROR: Could not query screen resources.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        return;
    }
    let cts: xcb_timestamp_t = (*res).config_timestamp;
    let len: ::core::ffi::c_int =
        xcb_randr_get_screen_resources_current_outputs_length(res) as ::core::ffi::c_int;
    let mut randr_outputs: *mut xcb_randr_output_t =
        xcb_randr_get_screen_resources_current_outputs(res);
    let vla = len as usize;
    let mut ocookie: Vec<xcb_randr_get_output_info_cookie_t> =
        ::std::vec::from_elem(xcb_randr_get_output_info_cookie_t { sequence: 0 }, vla);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < len {
        *ocookie.as_mut_ptr().offset(i as isize) =
            xcb_randr_get_output_info(conn, *randr_outputs.offset(i as isize), cts);
        i += 1;
    }
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < len {
        let mut output: *mut xcb_randr_get_output_info_reply_t =
            ::core::ptr::null_mut::<xcb_randr_get_output_info_reply_t>();
        output = xcb_randr_get_output_info_reply(
            conn,
            *ocookie.as_mut_ptr().offset(i_0 as isize),
            ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
        );
        if !output.is_null() {
            handle_output(conn, *randr_outputs.offset(i_0 as isize), output, cts, res);
            free(output as *mut ::core::ffi::c_void);
        }
        i_0 += 1;
    }
    free(res as *mut ::core::ffi::c_void);
    res = ::core::ptr::null_mut::<xcb_randr_get_screen_resources_current_reply_t>();
}
unsafe extern "C" fn move_content(mut con: *mut Con) {
    let mut first: *mut Con = (*get_first_output()).con;
    let mut first_content: *mut Con = output_get_content(first);
    let mut next: *mut Con = focused;
    let mut old_content: *const Con = output_get_content(con);
    while !(*old_content).nodes_head.tqh_first.is_null() {
        let mut current: *mut Con = (*old_content).nodes_head.tqh_first as *mut Con;
        if current != next && (*current).focus_head.tqh_first.is_null() {
            debuglog(
                b"%s:%s:%d - Getting rid of current = %p / %s (empty, unfocused)\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"../src/randr.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"move_content\0" as *const u8 as *const ::core::ffi::c_char,
                875 as ::core::ffi::c_int,
                current,
                (*current).name,
            );
            tree_close_internal(current, DONT_KILL_WINDOW, false_0 != 0);
        } else {
            debuglog(
                b"%s:%s:%d - Detaching current = %p / %s\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"../src/randr.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"move_content\0" as *const u8 as *const ::core::ffi::c_char,
                879 as ::core::ffi::c_int,
                current,
                (*current).name,
            );
            con_detach(current);
            debuglog(
                b"%s:%s:%d - Re-attaching current = %p / %s\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"../src/randr.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"move_content\0" as *const u8 as *const ::core::ffi::c_char,
                881 as ::core::ffi::c_int,
                current,
                (*current).name,
            );
            con_attach(current, first_content, false_0 != 0);
            debuglog(
                b"%s:%s:%d - Fixing the coordinates of floating containers\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"../src/randr.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"move_content\0" as *const u8 as *const ::core::ffi::c_char,
                883 as ::core::ffi::c_int,
            );
            let mut floating_con: *mut Con = ::core::ptr::null_mut::<Con>();
            floating_con = (*current).floating_head.tqh_first as *mut Con;
            while !floating_con.is_null() {
                floating_fix_coordinates(
                    floating_con,
                    &raw mut (*con).rect,
                    &raw mut (*first).rect,
                );
                floating_con = (*floating_con).floating_windows.tqe_next as *mut Con;
            }
        }
    }
    if !next.is_null() {
        debuglog(
            b"%s:%s:%d - now focusing next = %p\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b"../src/randr.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"move_content\0" as *const u8 as *const ::core::ffi::c_char,
            892 as ::core::ffi::c_int,
            next,
        );
        con_focus(next);
        workspace_show(con_get_workspace(next));
    }
    let mut child: *mut Con = ::core::ptr::null_mut::<Con>();
    child = (*con).nodes_head.tqh_first as *mut Con;
    while !child.is_null() {
        if !((*child).type_0 as ::core::ffi::c_uint
            != CT_DOCKAREA as ::core::ffi::c_int as ::core::ffi::c_uint)
        {
            debuglog(
                b"%s:%s:%d - Handling dock con %p\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"../src/randr.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"move_content\0" as *const u8 as *const ::core::ffi::c_char,
                903 as ::core::ffi::c_int,
                child,
            );
            while !(*child).nodes_head.tqh_first.is_null() {
                let mut match_0: *mut Match = ::core::ptr::null_mut::<Match>();
                let mut dock: *mut Con = (*child).nodes_head.tqh_first as *mut Con;
                let mut nc: *mut Con =
                    con_for_window(first, (*dock).window as *mut i3Window, &raw mut match_0);
                debuglog(
                    b"%s:%s:%d - Moving dock client %p to nc %p\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    b"../src/randr.c\0" as *const u8 as *const ::core::ffi::c_char,
                    b"move_content\0" as *const u8 as *const ::core::ffi::c_char,
                    908 as ::core::ffi::c_int,
                    dock,
                    nc,
                );
                con_detach(dock);
                debuglog(
                    b"%s:%s:%d - Re-attaching\n\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    b"../src/randr.c\0" as *const u8 as *const ::core::ffi::c_char,
                    b"move_content\0" as *const u8 as *const ::core::ffi::c_char,
                    910 as ::core::ffi::c_int,
                );
                con_attach(dock, nc, false_0 != 0);
                debuglog(
                    b"%s:%s:%d - Done\n\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    b"../src/randr.c\0" as *const u8 as *const ::core::ffi::c_char,
                    b"move_content\0" as *const u8 as *const ::core::ffi::c_char,
                    912 as ::core::ffi::c_int,
                );
            }
        }
        child = (*child).nodes.tqe_next as *mut Con;
    }
    debuglog(
        b"%s:%s:%d - Destroying disappearing con %p\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/randr.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"move_content\0" as *const u8 as *const ::core::ffi::c_char,
        916 as ::core::ffi::c_int,
        con,
    );
    tree_close_internal(con, DONT_KILL_WINDOW, true_0 != 0);
}
#[no_mangle]
pub unsafe extern "C" fn randr_query_outputs() {
    if !randr_query_outputs_15() {
        randr_query_outputs_14();
    }
    if any_randr_output_active() {
        debuglog(
            b"%s:%s:%d - Active RandR output found. Disabling root output.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/randr.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"randr_query_outputs\0" as *const u8 as *const ::core::ffi::c_char,
            933 as ::core::ffi::c_int,
        );
        if !root_output.is_null() && (*root_output).active as ::core::ffi::c_int != 0 {
            (*root_output).to_be_disabled = true_0 != 0;
        }
    } else {
        debuglog(
            b"%s:%s:%d - No active RandR output found. Enabling root output.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/randr.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"randr_query_outputs\0" as *const u8 as *const ::core::ffi::c_char,
            938 as ::core::ffi::c_int,
        );
        (*root_output).active = true_0 != 0;
    }
    let mut output: *mut Output = ::core::ptr::null_mut::<Output>();
    output = outputs.tqh_first as *mut Output;
    while !output.is_null() {
        if !(!(*output).active || (*output).to_be_disabled as ::core::ffi::c_int != 0) {
            debuglog(
                b"%s:%s:%d - output %p / %s, position (%d, %d), checking for clones\n\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"../src/randr.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"randr_query_outputs\0" as *const u8 as *const ::core::ffi::c_char,
                950 as ::core::ffi::c_int,
                output,
                output_primary_name(output),
                (*output).rect.x,
                (*output).rect.y,
            );
            let mut other: *mut Output = output;
            while !other.is_null() {
                if !(other == output
                    || !(*other).active
                    || (*other).to_be_disabled as ::core::ffi::c_int != 0)
                {
                    if !((*other).rect.x != (*output).rect.x || (*other).rect.y != (*output).rect.y)
                    {
                        debuglog(
                            b"%s:%s:%d - output %p has the same position, its mode = %d x %d\n\0"
                                as *const u8
                                as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b"../src/randr.c\0" as *const u8 as *const ::core::ffi::c_char,
                            b"randr_query_outputs\0" as *const u8 as *const ::core::ffi::c_char,
                            965 as ::core::ffi::c_int,
                            other,
                            (*other).rect.width,
                            (*other).rect.height,
                        );
                        let mut width: uint32_t = min(
                            (*other).rect.width as ::core::ffi::c_int,
                            (*output).rect.width as ::core::ffi::c_int,
                        ) as uint32_t;
                        let mut height: uint32_t = min(
                            (*other).rect.height as ::core::ffi::c_int,
                            (*output).rect.height as ::core::ffi::c_int,
                        ) as uint32_t;
                        let update_w: bool =
                            update_if_necessary(&raw mut (*output).rect.width, width) as bool;
                        let update_h: bool =
                            update_if_necessary(&raw mut (*output).rect.height, height) as bool;
                        if update_w as ::core::ffi::c_int != 0
                            || update_h as ::core::ffi::c_int != 0
                        {
                            (*output).changed = true_0 != 0;
                        }
                        update_if_necessary(&raw mut (*other).rect.width, width);
                        update_if_necessary(&raw mut (*other).rect.height, height);
                        debuglog(
                            b"%s:%s:%d - disabling output %p (%s)\n\0" as *const u8
                                as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b"../src/randr.c\0" as *const u8 as *const ::core::ffi::c_char,
                            b"randr_query_outputs\0" as *const u8 as *const ::core::ffi::c_char,
                            978 as ::core::ffi::c_int,
                            other,
                            output_primary_name(other),
                        );
                        (*other).to_be_disabled = true_0 != 0;
                        debuglog(
                            b"%s:%s:%d - new output mode %d x %d, other mode %d x %d\n\0"
                                as *const u8
                                as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b"../src/randr.c\0" as *const u8 as *const ::core::ffi::c_char,
                            b"randr_query_outputs\0" as *const u8 as *const ::core::ffi::c_char,
                            983 as ::core::ffi::c_int,
                            (*output).rect.width,
                            (*output).rect.height,
                            (*other).rect.width,
                            (*other).rect.height,
                        );
                    }
                }
                other = (*other).outputs.tqe_next as *mut Output;
            }
        }
        output = (*output).outputs.tqe_next as *mut Output;
    }
    output = outputs.tqh_first as *mut Output;
    while !output.is_null() {
        if (*output).active as ::core::ffi::c_int != 0 && (*output).con.is_null() {
            debuglog(
                b"%s:%s:%d - Need to initialize a Con for output %s\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"../src/randr.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"randr_query_outputs\0" as *const u8 as *const ::core::ffi::c_char,
                993 as ::core::ffi::c_int,
                output_primary_name(output),
            );
            output_init_con(output);
            (*output).changed = false_0 != 0;
        }
        output = (*output).outputs.tqe_next as *mut Output;
    }
    let mut con: *mut Con = (*croot).nodes_head.tqh_first as *mut Con;
    while !con.is_null() {
        let mut next: *mut Con = (*con).nodes.tqe_next as *mut Con;
        if !con_is_internal(con) && get_output_by_name((*con).name, true_0 != 0).is_null() {
            debuglog(
                b"%s:%s:%d - No output %s found, moving its old content to first output\n\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"../src/randr.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"randr_query_outputs\0" as *const u8 as *const ::core::ffi::c_char,
                1007 as ::core::ffi::c_int,
                (*con).name,
            );
            move_content(con);
        }
        con = next;
    }
    output = outputs.tqh_first as *mut Output;
    while !output.is_null() {
        if (*output).to_be_disabled {
            randr_disable_output(output);
        }
        if (*output).changed {
            output_change_mode(conn, output);
            (*output).changed = false_0 != 0;
        }
        output = (*output).outputs.tqe_next as *mut Output;
    }
    output = outputs.tqh_first as *mut Output;
    while !output.is_null() {
        if (*output).active {
            let mut content: *mut Con = output_get_content((*output).con);
            if (*content).nodes_head.tqh_first.is_null() {
                debuglog(
                    b"%s:%s:%d - Should add ws for output %s\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    b"../src/randr.c\0" as *const u8 as *const ::core::ffi::c_char,
                    b"randr_query_outputs\0" as *const u8 as *const ::core::ffi::c_char,
                    1035 as ::core::ffi::c_int,
                    output_primary_name(output),
                );
                init_ws_for_output(output);
            }
        }
        output = (*output).outputs.tqe_next as *mut Output;
    }
    output = outputs.tqh_first as *mut Output;
    while !output.is_null() {
        if !(!(*output).primary || (*output).con.is_null()) {
            debuglog(
                b"%s:%s:%d - Focusing primary output %s\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"../src/randr.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"randr_query_outputs\0" as *const u8 as *const ::core::ffi::c_char,
                1045 as ::core::ffi::c_int,
                output_primary_name(output),
            );
            let mut content_0: *mut Con = output_get_content((*output).con);
            let mut ws: *mut Con = (*content_0).focus_head.tqh_first as *mut Con;
            workspace_show(ws);
        }
        output = (*output).outputs.tqe_next as *mut Output;
    }
    ewmh_update_desktop_properties();
    tree_render();
    free(primary as *mut ::core::ffi::c_void);
    primary = ::core::ptr::null_mut::<xcb_randr_get_output_primary_reply_t>();
}
#[no_mangle]
pub unsafe extern "C" fn randr_disable_output(mut output: *mut Output) {
    if (*output).to_be_disabled as ::core::ffi::c_int != 0 {
    } else {
        __assert_fail(
            b"output->to_be_disabled\0" as *const u8 as *const ::core::ffi::c_char,
            b"../src/randr.c\0" as *const u8 as *const ::core::ffi::c_char,
            1063 as ::core::ffi::c_uint,
            b"void randr_disable_output(Output *)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
    (*output).active = false_0 != 0;
    debuglog(
        b"%s:%s:%d - Output %s disabled, re-assigning workspaces/docks\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/randr.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"randr_disable_output\0" as *const u8 as *const ::core::ffi::c_char,
        1066 as ::core::ffi::c_int,
        output_primary_name(output),
    );
    if !(*output).con.is_null() {
        let mut con: *mut Con = (*output).con;
        (*output).con = ::core::ptr::null_mut::<Con>();
        move_content(con);
    }
    (*output).to_be_disabled = false_0 != 0;
    (*output).changed = false_0 != 0;
}
unsafe extern "C" fn fallback_to_root_output() {
    (*root_output).active = true_0 != 0;
    output_init_con(root_output);
    init_ws_for_output(root_output);
}
#[no_mangle]
pub unsafe extern "C" fn randr_init(
    mut event_base: *mut ::core::ffi::c_int,
    disable_randr15: bool,
) {
    root_output = create_root_output(conn);
    (*root_output).outputs.tqe_next = ::core::ptr::null_mut::<xoutput>();
    (*root_output).outputs.tqe_prev = outputs.tqh_last;
    *outputs.tqh_last = root_output as *mut xoutput;
    outputs.tqh_last = &raw mut (*root_output).outputs.tqe_next;
    let mut extension_reply: *const xcb_query_extension_reply_t =
        xcb_get_extension_data(conn, &raw mut xcb_randr_id) as *const xcb_query_extension_reply_t;
    if (*extension_reply).present == 0 {
        debuglog(
            b"%s:%s:%d - RandR is not present, activating root output.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/randr.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"randr_init\0" as *const u8 as *const ::core::ffi::c_char,
            1096 as ::core::ffi::c_int,
        );
        fallback_to_root_output();
        return;
    }
    let mut err: *mut xcb_generic_error_t = ::core::ptr::null_mut::<xcb_generic_error_t>();
    let mut randr_version: *mut xcb_randr_query_version_reply_t = xcb_randr_query_version_reply(
        conn,
        xcb_randr_query_version(
            conn,
            XCB_RANDR_MAJOR_VERSION as uint32_t,
            XCB_RANDR_MINOR_VERSION as uint32_t,
        ),
        &raw mut err,
    );
    if !err.is_null() {
        errorlog(
            b"ERROR: Could not query RandR version: X11 error code %d\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            (*err).error_code as ::core::ffi::c_int,
        );
        free(err as *mut ::core::ffi::c_void);
        fallback_to_root_output();
        return;
    }
    has_randr_1_5 = (*randr_version).major_version >= 1 as uint32_t
        && (*randr_version).minor_version >= 5 as uint32_t
        && !disable_randr15;
    free(randr_version as *mut ::core::ffi::c_void);
    randr_query_outputs();
    if !event_base.is_null() {
        *event_base = (*extension_reply).first_event as ::core::ffi::c_int;
    }
    xcb_randr_select_input(
        conn,
        root,
        (XCB_RANDR_NOTIFY_MASK_SCREEN_CHANGE as ::core::ffi::c_int
            | XCB_RANDR_NOTIFY_MASK_OUTPUT_CHANGE as ::core::ffi::c_int
            | XCB_RANDR_NOTIFY_MASK_CRTC_CHANGE as ::core::ffi::c_int
            | XCB_RANDR_NOTIFY_MASK_OUTPUT_PROPERTY as ::core::ffi::c_int) as uint16_t,
    );
    xcb_flush(conn);
}
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
unsafe extern "C" fn run_static_initializers() {
    outputs = outputs_head {
        tqh_first: ::core::ptr::null_mut::<xoutput>(),
        tqh_last: &raw mut outputs.tqh_first,
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
