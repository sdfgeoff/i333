/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * handlers.c: Small handlers for various events (keypresses, focus changes,
 *             …).
 *
 */
// Mechanical file-by-file Rust translation of src/handlers.c.
// Generated with C2Rust 0.22.1; C ABI and original ownership retained.
extern "C" {
    pub type xcb_connection_t;
    pub type _XCBKeySymbols;
    pub type _PangoFontDescription;
    pub type _cairo;
    pub type _cairo_surface;
    pub type _i3String;
    pub type SnDisplay;
    pub type pcre2_real_code_8;
    pub type ev_loop;
    pub type SnMonitorContext;
    pub type SnMonitorEvent;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn __assert_single_arg(_: bool) -> bool;
    fn time(__timer: *mut time_t) -> time_t;
    fn xcb_get_window_attributes_unchecked(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_get_window_attributes_cookie_t;
    fn xcb_configure_window(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        value_mask: uint16_t,
        value_list: *const ::core::ffi::c_void,
    ) -> xcb_void_cookie_t;
    fn xcb_get_geometry(
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
    ) -> xcb_get_geometry_cookie_t;
    fn xcb_get_geometry_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_get_geometry_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_get_geometry_reply_t;
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
    fn xcb_get_property(
        c: *mut xcb_connection_t,
        _delete: uint8_t,
        window: xcb_window_t,
        property: xcb_atom_t,
        type_0: xcb_atom_t,
        long_offset: uint32_t,
        long_length: uint32_t,
    ) -> xcb_get_property_cookie_t;
    fn xcb_get_property_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_get_property_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_get_property_reply_t;
    fn xcb_get_input_focus(c: *mut xcb_connection_t) -> xcb_get_input_focus_cookie_t;
    fn xcb_get_input_focus_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_get_input_focus_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_get_input_focus_reply_t;
    fn xcb_flush(c: *mut xcb_connection_t) -> ::core::ffi::c_int;
    fn xcb_key_symbols_alloc(c: *mut xcb_connection_t) -> *mut xcb_key_symbols_t;
    fn xcb_key_symbols_free(syms: *mut xcb_key_symbols_t);
    fn xcb_refresh_keyboard_mapping(
        syms: *mut xcb_key_symbols_t,
        event: *mut xcb_mapping_notify_event_t,
    ) -> ::core::ffi::c_int;
    static mut conn: *mut xcb_connection_t;
    fn verboselog(fmt: *mut ::core::ffi::c_char, ...);
    fn errorlog(fmt: *mut ::core::ffi::c_char, ...);
    fn debuglog(fmt: *mut ::core::ffi::c_char, ...);
    fn smalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn scalloc(num: size_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn sstrdup(str: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn i3string_as_utf8(str: *mut i3String) -> *const ::core::ffi::c_char;
    fn aio_get_mod_mask_for(keysym: uint32_t, symbols: *mut xcb_key_symbols_t) -> uint32_t;
    fn draw_util_copy_surface(
        src: *mut surface_t,
        dest: *mut surface_t,
        src_x: ::core::ffi::c_double,
        src_y: ::core::ffi::c_double,
        dest_x: ::core::ffi::c_double,
        dest_y: ::core::ffi::c_double,
        width: ::core::ffi::c_double,
        height: ::core::ffi::c_double,
    );
    fn sn_xcb_display_process_event(
        display: *mut SnDisplay,
        xevent: *mut xcb_generic_event_t,
    ) -> sn_bool_t;
    fn rect_contains(rect: Rect, x: uint32_t, y: uint32_t) -> bool;
    static mut croot: *mut Con;
    static mut focused: *mut Con;
    fn tree_render();
    fn tree_close_internal(
        con: *mut Con,
        kill_window: kill_window_t,
        dont_kill_parent: bool,
    ) -> bool;
    static mut conn_screen: ::core::ffi::c_int;
    static mut wm_sn: xcb_atom_t;
    static mut last_timestamp: xcb_timestamp_t;
    static mut sndisplay: *mut SnDisplay;
    static mut keysyms: *mut xcb_key_symbols_t;
    static mut shape_supported: bool;
    static mut root: xcb_window_t;
    static mut force_xinerama: bool;
    static mut config: Config;
    fn ungrab_all_keys(conn_0: *mut xcb_connection_t);
    fn ipc_send_event(
        event: *const ::core::ffi::c_char,
        message_type: uint32_t,
        payload: *const ::core::ffi::c_char,
    );
    fn ipc_send_window_event(property: *const ::core::ffi::c_char, con: *mut Con);
    static mut A__NET_WM_STATE: xcb_atom_t;
    static mut A__NET_WM_STRUT_PARTIAL: xcb_atom_t;
    static mut A__NET_WM_WINDOW_TYPE: xcb_atom_t;
    static mut A__NET_WM_STATE_DEMANDS_ATTENTION: xcb_atom_t;
    static mut A__NET_WM_STATE_STICKY: xcb_atom_t;
    static mut A__NET_WM_NAME: xcb_atom_t;
    static mut A__NET_WM_MOVERESIZE: xcb_atom_t;
    static mut A__NET_WM_STATE_FULLSCREEN: xcb_atom_t;
    static mut A__NET_WM_DESKTOP: xcb_atom_t;
    static mut A__NET_CURRENT_DESKTOP: xcb_atom_t;
    static mut A__NET_ACTIVE_WINDOW: xcb_atom_t;
    static mut A__NET_CLOSE_WINDOW: xcb_atom_t;
    static mut A__NET_MOVERESIZE_WINDOW: xcb_atom_t;
    static mut A__NET_WM_ICON: xcb_atom_t;
    static mut A_WM_STATE: xcb_atom_t;
    static mut A_WM_CLIENT_LEADER: xcb_atom_t;
    static mut A_WM_WINDOW_ROLE: xcb_atom_t;
    static mut A_I3_SYNC: xcb_atom_t;
    static mut A__NET_REQUEST_FRAME_EXTENTS: xcb_atom_t;
    static mut A__NET_FRAME_EXTENTS: xcb_atom_t;
    static mut A__MOTIF_WM_HINTS: xcb_atom_t;
    static mut A_WM_CHANGE_STATE: xcb_atom_t;
    static mut xcb_numlock_mask: ::core::ffi::c_uint;
    fn fake_absolute_configure_notify(con: *mut Con);
    fn manage_window(
        window: xcb_window_t,
        cookie: xcb_get_window_attributes_cookie_t,
        needs_to_be_mapped: bool,
    );
    fn remanage_window(con: *mut Con) -> *mut Con;
    fn randr_query_outputs();
    fn get_output_containing(x: ::core::ffi::c_uint, y: ::core::ffi::c_uint) -> *mut Output;
    fn workspace_get(num: *const ::core::ffi::c_char) -> *mut Con;
    fn workspace_is_visible(ws: *mut Con) -> bool;
    fn workspace_show(ws: *mut Con);
    static mut focused_id: xcb_window_t;
    fn x_push_changes(con: *mut Con);
    fn x_set_shape(con: *mut Con, kind: xcb_shape_sk_t, enable: bool);
    fn handle_button_press(event: *mut xcb_button_press_event_t);
    fn handle_key_press(event: *mut xcb_key_press_event_t);
    fn floating_enable(con: *mut Con, automatic: bool) -> bool;
    fn floating_drag_window(
        con: *mut Con,
        event: *const xcb_button_press_event_t,
        use_threshold: bool,
    );
    fn floating_resize_window(
        con: *mut Con,
        proportional: bool,
        event: *const xcb_button_press_event_t,
    );
    fn floating_check_size(floating_con: *mut Con, prefer_height: bool);
    fn floating_reposition(con: *mut Con, newrect: Rect) -> bool;
    fn con_focus(con: *mut Con);
    fn con_activate_unblock(con: *mut Con);
    fn con_is_leaf(con: *mut Con) -> bool;
    fn con_get_output(con: *mut Con) -> *mut Con;
    fn con_get_workspace(con: *mut Con) -> *mut Con;
    fn con_get_fullscreen_covering_ws(ws: *mut Con) -> *mut Con;
    fn con_is_internal(con: *mut Con) -> bool;
    fn con_is_floating(con: *mut Con) -> bool;
    fn con_inside_floating(con: *mut Con) -> *mut Con;
    fn con_by_window_id(window: xcb_window_t) -> *mut Con;
    fn con_by_frame_id(frame: xcb_window_t) -> *mut Con;
    fn con_for_window(
        con: *mut Con,
        window: *mut i3Window,
        store_match: *mut *mut Match,
    ) -> *mut Con;
    fn con_attach(con: *mut Con, parent: *mut Con, ignore_focus: bool);
    fn con_detach(con: *mut Con);
    fn con_toggle_fullscreen(con: *mut Con, fullscreen_mode: ::core::ffi::c_int);
    fn con_move_to_workspace(
        con: *mut Con,
        workspace: *mut Con,
        fix_coordinates: bool,
        dont_warp: bool,
        ignore_focus: bool,
    );
    fn con_descend_focused(con: *mut Con) -> *mut Con;
    fn con_border_style_rect(con: *mut Con) -> Rect;
    fn con_set_border_style(
        con: *mut Con,
        border_style: border_style_t,
        border_width: ::core::ffi::c_int,
    );
    fn con_set_urgency(con: *mut Con, urgent: bool);
    fn render_deco_height() -> ::core::ffi::c_int;
    fn window_update_class(win: *mut i3Window, prop: *mut xcb_get_property_reply_t);
    fn window_update_name(win: *mut i3Window, prop: *mut xcb_get_property_reply_t);
    fn window_update_name_legacy(win: *mut i3Window, prop: *mut xcb_get_property_reply_t);
    fn window_update_leader(win: *mut i3Window, prop: *mut xcb_get_property_reply_t);
    fn window_update_transient_for(win: *mut i3Window, prop: *mut xcb_get_property_reply_t);
    fn window_update_strut_partial(win: *mut i3Window, prop: *mut xcb_get_property_reply_t);
    fn window_update_role(win: *mut i3Window, prop: *mut xcb_get_property_reply_t);
    fn window_update_type(window: *mut i3Window, reply: *mut xcb_get_property_reply_t);
    fn window_update_normal_hints(
        win: *mut i3Window,
        reply: *mut xcb_get_property_reply_t,
        geom: *mut xcb_get_geometry_reply_t,
    ) -> bool;
    fn window_update_hints(
        win: *mut i3Window,
        prop: *mut xcb_get_property_reply_t,
        urgency_hint: *mut bool,
    );
    fn window_update_motif_hints(
        win: *mut i3Window,
        prop: *mut xcb_get_property_reply_t,
        motif_border_style: *mut border_style_t,
    ) -> bool;
    fn window_update_machine(win: *mut i3Window, prop: *mut xcb_get_property_reply_t);
    fn window_update_icon(win: *mut i3Window, prop: *mut xcb_get_property_reply_t);
    fn output_get_content(output: *mut Con) -> *mut Con;
    fn output_primary_name(output: *mut Output) -> *mut ::core::ffi::c_char;
    fn output_push_sticky_windows(old_focus: *mut Con);
    fn ewmh_update_wm_desktop();
    fn ewmh_update_sticky(window: xcb_window_t, sticky: bool);
    fn ewmh_get_workspace_by_index(idx: uint32_t) -> *mut Con;
    fn run_assignments(window: *mut i3Window);
    fn sn_monitor_context_new(
        display: *mut SnDisplay,
        screen: ::core::ffi::c_int,
        event_func: SnMonitorEventFunc,
        event_func_data: *mut ::core::ffi::c_void,
        free_data_func: SnFreeFunc,
    ) -> *mut SnMonitorContext;
    fn startup_monitor_event(event: *mut SnMonitorEvent, userdata: *mut ::core::ffi::c_void);
    fn scratchpad_show(con: *mut Con) -> bool;
    fn scratchpad_fix_resolution();
    fn grab_all_keys(conn_0: *mut xcb_connection_t);
    fn translate_keysyms();
    fn load_keymap() -> bool;
    fn sync_respond(window: xcb_window_t, rnd: uint32_t);
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn exit(__status: ::core::ffi::c_int) -> !;
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
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
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_generic_event_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub pad: [uint32_t; 7],
    pub full_sequence: uint32_t,
}
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
pub type xcb_timestamp_t = uint32_t;
pub type xcb_keycode_t = uint8_t;
pub type xcb_button_t = uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_key_press_event_t {
    pub response_type: uint8_t,
    pub detail: xcb_keycode_t,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_motion_notify_event_t {
    pub response_type: uint8_t,
    pub detail: uint8_t,
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
pub type xcb_notify_detail_t = ::core::ffi::c_uint;
pub const XCB_NOTIFY_DETAIL_NONE: xcb_notify_detail_t = 7;
pub const XCB_NOTIFY_DETAIL_POINTER_ROOT: xcb_notify_detail_t = 6;
pub const XCB_NOTIFY_DETAIL_POINTER: xcb_notify_detail_t = 5;
pub const XCB_NOTIFY_DETAIL_NONLINEAR_VIRTUAL: xcb_notify_detail_t = 4;
pub const XCB_NOTIFY_DETAIL_NONLINEAR: xcb_notify_detail_t = 3;
pub const XCB_NOTIFY_DETAIL_INFERIOR: xcb_notify_detail_t = 2;
pub const XCB_NOTIFY_DETAIL_VIRTUAL: xcb_notify_detail_t = 1;
pub const XCB_NOTIFY_DETAIL_ANCESTOR: xcb_notify_detail_t = 0;
pub type xcb_notify_mode_t = ::core::ffi::c_uint;
pub const XCB_NOTIFY_MODE_WHILE_GRABBED: xcb_notify_mode_t = 3;
pub const XCB_NOTIFY_MODE_UNGRAB: xcb_notify_mode_t = 2;
pub const XCB_NOTIFY_MODE_GRAB: xcb_notify_mode_t = 1;
pub const XCB_NOTIFY_MODE_NORMAL: xcb_notify_mode_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_enter_notify_event_t {
    pub response_type: uint8_t,
    pub detail: uint8_t,
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
    pub mode: uint8_t,
    pub same_screen_focus: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_focus_in_event_t {
    pub response_type: uint8_t,
    pub detail: uint8_t,
    pub sequence: uint16_t,
    pub event: xcb_window_t,
    pub mode: uint8_t,
    pub pad0: [uint8_t; 3],
}
pub type xcb_focus_out_event_t = xcb_focus_in_event_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_expose_event_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub window: xcb_window_t,
    pub x: uint16_t,
    pub y: uint16_t,
    pub width: uint16_t,
    pub height: uint16_t,
    pub count: uint16_t,
    pub pad1: [uint8_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_destroy_notify_event_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub event: xcb_window_t,
    pub window: xcb_window_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_unmap_notify_event_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub event: xcb_window_t,
    pub window: xcb_window_t,
    pub from_configure: uint8_t,
    pub pad1: [uint8_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_map_request_event_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub parent: xcb_window_t,
    pub window: xcb_window_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_configure_notify_event_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub event: xcb_window_t,
    pub window: xcb_window_t,
    pub above_sibling: xcb_window_t,
    pub x: int16_t,
    pub y: int16_t,
    pub width: uint16_t,
    pub height: uint16_t,
    pub border_width: uint16_t,
    pub override_redirect: uint8_t,
    pub pad1: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_configure_request_event_t {
    pub response_type: uint8_t,
    pub stack_mode: uint8_t,
    pub sequence: uint16_t,
    pub parent: xcb_window_t,
    pub window: xcb_window_t,
    pub sibling: xcb_window_t,
    pub x: int16_t,
    pub y: int16_t,
    pub width: uint16_t,
    pub height: uint16_t,
    pub border_width: uint16_t,
    pub value_mask: uint16_t,
}
pub type xcb_property_t = ::core::ffi::c_uint;
pub const XCB_PROPERTY_DELETE: xcb_property_t = 1;
pub const XCB_PROPERTY_NEW_VALUE: xcb_property_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_property_notify_event_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub window: xcb_window_t,
    pub atom: xcb_atom_t,
    pub time: xcb_timestamp_t,
    pub state: uint8_t,
    pub pad1: [uint8_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_selection_clear_event_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub time: xcb_timestamp_t,
    pub owner: xcb_window_t,
    pub selection: xcb_atom_t,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union xcb_client_message_data_t {
    pub data8: [uint8_t; 20],
    pub data16: [uint16_t; 10],
    pub data32: [uint32_t; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_client_message_event_t {
    pub response_type: uint8_t,
    pub format: uint8_t,
    pub sequence: uint16_t,
    pub window: xcb_window_t,
    pub type_0: xcb_atom_t,
    pub data: xcb_client_message_data_t,
}
pub type xcb_mapping_t = ::core::ffi::c_uint;
pub const XCB_MAPPING_POINTER: xcb_mapping_t = 2;
pub const XCB_MAPPING_KEYBOARD: xcb_mapping_t = 1;
pub const XCB_MAPPING_MODIFIER: xcb_mapping_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_mapping_notify_event_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub request: uint8_t,
    pub first_keycode: xcb_keycode_t,
    pub count: uint8_t,
    pub pad1: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_get_window_attributes_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_get_geometry_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_get_geometry_reply_t {
    pub response_type: uint8_t,
    pub depth: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub root: xcb_window_t,
    pub x: int16_t,
    pub y: int16_t,
    pub width: uint16_t,
    pub height: uint16_t,
    pub border_width: uint16_t,
    pub pad0: [uint8_t; 2],
}
pub type xcb_prop_mode_t = ::core::ffi::c_uint;
pub const XCB_PROP_MODE_APPEND: xcb_prop_mode_t = 2;
pub const XCB_PROP_MODE_PREPEND: xcb_prop_mode_t = 1;
pub const XCB_PROP_MODE_REPLACE: xcb_prop_mode_t = 0;
pub type xcb_get_property_type_t = ::core::ffi::c_uint;
pub const XCB_GET_PROPERTY_TYPE_ANY: xcb_get_property_type_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_get_property_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_get_property_reply_t {
    pub response_type: uint8_t,
    pub format: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub type_0: xcb_atom_t,
    pub bytes_after: uint32_t,
    pub value_len: uint32_t,
    pub pad0: [uint8_t; 12],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_get_input_focus_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_get_input_focus_reply_t {
    pub response_type: uint8_t,
    pub revert_to: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub focus: xcb_window_t,
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
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const XCB_ICCCM_WM_STATE_ICONIC: C2RustUnnamed = 3;
pub const XCB_ICCCM_WM_STATE_NORMAL: C2RustUnnamed = 1;
pub const XCB_ICCCM_WM_STATE_WITHDRAWN: C2RustUnnamed = 0;
pub type xcb_key_symbols_t = _XCBKeySymbols;
pub type xcb_xkb_nkn_detail_t = ::core::ffi::c_uint;
pub const XCB_XKB_NKN_DETAIL_DEVICE_ID: xcb_xkb_nkn_detail_t = 4;
pub const XCB_XKB_NKN_DETAIL_GEOMETRY: xcb_xkb_nkn_detail_t = 2;
pub const XCB_XKB_NKN_DETAIL_KEYCODES: xcb_xkb_nkn_detail_t = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xkb_new_keyboard_notify_event_t {
    pub response_type: uint8_t,
    pub xkbType: uint8_t,
    pub sequence: uint16_t,
    pub time: xcb_timestamp_t,
    pub deviceID: uint8_t,
    pub oldDeviceID: uint8_t,
    pub minKeyCode: xcb_keycode_t,
    pub maxKeyCode: xcb_keycode_t,
    pub oldMinKeyCode: xcb_keycode_t,
    pub oldMaxKeyCode: xcb_keycode_t,
    pub requestMajor: uint8_t,
    pub requestMinor: uint8_t,
    pub changed: uint16_t,
    pub pad0: [uint8_t; 14],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xkb_state_notify_event_t {
    pub response_type: uint8_t,
    pub xkbType: uint8_t,
    pub sequence: uint16_t,
    pub time: xcb_timestamp_t,
    pub deviceID: uint8_t,
    pub mods: uint8_t,
    pub baseMods: uint8_t,
    pub latchedMods: uint8_t,
    pub lockedMods: uint8_t,
    pub group: uint8_t,
    pub baseGroup: int16_t,
    pub latchedGroup: int16_t,
    pub lockedGroup: uint8_t,
    pub compatState: uint8_t,
    pub grabMods: uint8_t,
    pub compatGrabMods: uint8_t,
    pub lookupMods: uint8_t,
    pub compatLoockupMods: uint8_t,
    pub ptrBtnState: uint16_t,
    pub changed: uint16_t,
    pub keycode: xcb_keycode_t,
    pub eventType: uint8_t,
    pub requestMajor: uint8_t,
    pub requestMinor: uint8_t,
}
pub type PangoFontDescription = _PangoFontDescription;
pub type cairo_t = _cairo;
pub type cairo_surface_t = _cairo_surface;
pub type i3String = _i3String;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Font {
    pub type_0: C2RustUnnamed_2,
    pub height: ::core::ffi::c_int,
    pub pattern: *mut ::core::ffi::c_char,
    pub specific: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub xcb: C2RustUnnamed_1,
    pub pango_desc: *mut PangoFontDescription,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub id: xcb_font_t,
    pub info: *mut xcb_query_font_reply_t,
    pub table: *mut xcb_charinfo_t,
}
pub type C2RustUnnamed_2 = ::core::ffi::c_uint;
pub const FONT_TYPE_PANGO: C2RustUnnamed_2 = 2;
pub const FONT_TYPE_XCB: C2RustUnnamed_2 = 1;
pub const FONT_TYPE_NONE: C2RustUnnamed_2 = 0;
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
pub type sn_bool_t = ::core::ffi::c_int;
pub type SnFreeFunc = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
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
    pub outputs: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
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
    pub type_0: C2RustUnnamed_20,
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
    pub floating: C2RustUnnamed_9,
    pub nodes: C2RustUnnamed_8,
    pub focused: C2RustUnnamed_7,
    pub all_cons: C2RustUnnamed_6,
    pub floating_windows: C2RustUnnamed_5,
    pub on_remove_child: Option<unsafe extern "C" fn(*mut Con) -> ()>,
    pub scratchpad_state: C2RustUnnamed_4,
    pub old_id: ::core::ffi::c_int,
    pub depth: uint16_t,
    pub colormap: xcb_colormap_t,
}
pub type C2RustUnnamed_4 = ::core::ffi::c_uint;
pub const SCRATCHPAD_CHANGED: C2RustUnnamed_4 = 2;
pub const SCRATCHPAD_FRESH: C2RustUnnamed_4 = 1;
pub const SCRATCHPAD_NONE: C2RustUnnamed_4 = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub tqe_next: *mut Con,
    pub tqe_prev: *mut *mut Con,
}
pub type C2RustUnnamed_9 = ::core::ffi::c_uint;
pub const FLOATING_USER_ON: C2RustUnnamed_9 = 3;
pub const FLOATING_AUTO_ON: C2RustUnnamed_9 = 2;
pub const FLOATING_USER_OFF: C2RustUnnamed_9 = 1;
pub const FLOATING_AUTO_OFF: C2RustUnnamed_9 = 0;
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
    pub urgent: C2RustUnnamed_14,
    pub dock: C2RustUnnamed_13,
    pub id: xcb_window_t,
    pub window_mode: C2RustUnnamed_12,
    pub con_id: *mut Con,
    pub match_all_windows: bool,
    pub insert_where: C2RustUnnamed_11,
    pub matches: C2RustUnnamed_10,
    pub restart_mode: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub tqe_next: *mut Match,
    pub tqe_prev: *mut *mut Match,
}
pub type C2RustUnnamed_11 = ::core::ffi::c_uint;
pub const M_BELOW: C2RustUnnamed_11 = 2;
pub const M_ASSIGN_WS: C2RustUnnamed_11 = 1;
pub const M_HERE: C2RustUnnamed_11 = 0;
pub type C2RustUnnamed_12 = ::core::ffi::c_uint;
pub const WM_FLOATING: C2RustUnnamed_12 = 6;
pub const WM_FLOATING_USER: C2RustUnnamed_12 = 5;
pub const WM_FLOATING_AUTO: C2RustUnnamed_12 = 4;
pub const WM_TILING: C2RustUnnamed_12 = 3;
pub const WM_TILING_USER: C2RustUnnamed_12 = 2;
pub const WM_TILING_AUTO: C2RustUnnamed_12 = 1;
pub const WM_ANY: C2RustUnnamed_12 = 0;
pub type C2RustUnnamed_13 = ::core::ffi::c_int;
pub const M_DOCK_BOTTOM: C2RustUnnamed_13 = 3;
pub const M_DOCK_TOP: C2RustUnnamed_13 = 2;
pub const M_DOCK_ANY: C2RustUnnamed_13 = 1;
pub const M_NODOCK: C2RustUnnamed_13 = 0;
pub const M_DONTCHECK: C2RustUnnamed_13 = -1;
pub type C2RustUnnamed_14 = ::core::ffi::c_int;
pub const U_OLDEST: C2RustUnnamed_14 = 1;
pub const U_LATEST: C2RustUnnamed_14 = 0;
pub const U_DONTCHECK: C2RustUnnamed_14 = -1;
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
    pub dock: C2RustUnnamed_15,
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
pub type C2RustUnnamed_15 = ::core::ffi::c_uint;
pub const W_DOCK_BOTTOM: C2RustUnnamed_15 = 2;
pub const W_DOCK_TOP: C2RustUnnamed_15 = 1;
pub const W_NODOCK: C2RustUnnamed_15 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Assignment {
    pub type_0: C2RustUnnamed_18,
    pub match_0: Match,
    pub dest: C2RustUnnamed_17,
    pub assignments: C2RustUnnamed_16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_16 {
    pub tqe_next: *mut Assignment,
    pub tqe_prev: *mut *mut Assignment,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_17 {
    pub command: *mut ::core::ffi::c_char,
    pub workspace: *mut ::core::ffi::c_char,
    pub output: *mut ::core::ffi::c_char,
}
pub type C2RustUnnamed_18 = ::core::ffi::c_uint;
pub const A_TO_OUTPUT: C2RustUnnamed_18 = 16;
pub const A_TO_WORKSPACE_NUMBER: C2RustUnnamed_18 = 8;
pub const A_NO_FOCUS: C2RustUnnamed_18 = 4;
pub const A_TO_WORKSPACE: C2RustUnnamed_18 = 2;
pub const A_COMMAND: C2RustUnnamed_18 = 1;
pub const A_ANY: C2RustUnnamed_18 = 0;
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
    pub marks: C2RustUnnamed_19,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_19 {
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
pub type C2RustUnnamed_20 = ::core::ffi::c_uint;
pub const CT_DOCKAREA: C2RustUnnamed_20 = 5;
pub const CT_WORKSPACE: C2RustUnnamed_20 = 4;
pub const CT_FLOATING_CON: C2RustUnnamed_20 = 3;
pub const CT_CON: C2RustUnnamed_20 = 2;
pub const CT_OUTPUT: C2RustUnnamed_20 = 1;
pub const CT_ROOT: C2RustUnnamed_20 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct names_head {
    pub slh_first: *mut output_name,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct output_name {
    pub name: *mut ::core::ffi::c_char,
    pub names: C2RustUnnamed_21,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_21 {
    pub sle_next: *mut output_name,
}
pub type Output = xoutput;
pub type i3Window = Window;
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
pub type focus_wrapping_t = ::core::ffi::c_uint;
pub const FOCUS_WRAPPING_WORKSPACE: focus_wrapping_t = 3;
pub const FOCUS_WRAPPING_FORCE: focus_wrapping_t = 2;
pub const FOCUS_WRAPPING_ON: focus_wrapping_t = 1;
pub const FOCUS_WRAPPING_OFF: focus_wrapping_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ignore_event_sequence_t {
    pub value: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Ignore_Event {
    pub sequence: ignore_event_sequence_t,
    pub response_type: ::core::ffi::c_int,
    pub added: time_t,
    pub ignore_events: C2RustUnnamed_22,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_22 {
    pub sle_next: *mut Ignore_Event,
}
pub type xcb_shape_kind_t = uint8_t;
pub type xcb_shape_sk_t = ::core::ffi::c_uint;
pub const XCB_SHAPE_SK_INPUT: xcb_shape_sk_t = 2;
pub const XCB_SHAPE_SK_CLIP: xcb_shape_sk_t = 1;
pub const XCB_SHAPE_SK_BOUNDING: xcb_shape_sk_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_shape_notify_event_t {
    pub response_type: uint8_t,
    pub shape_kind: xcb_shape_kind_t,
    pub sequence: uint16_t,
    pub affected_window: xcb_window_t,
    pub extents_x: int16_t,
    pub extents_y: int16_t,
    pub extents_width: uint16_t,
    pub extents_height: uint16_t,
    pub server_time: xcb_timestamp_t,
    pub shaped: uint8_t,
    pub pad0: [uint8_t; 11],
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
pub struct ignore_head {
    pub slh_first: *mut Ignore_Event,
}
pub type cb_property_handler_t =
    Option<unsafe extern "C" fn(*mut Con, *mut xcb_get_property_reply_t) -> bool>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct property_handler_t {
    pub atom: xcb_atom_t,
    pub long_len: uint32_t,
    pub cb: cb_property_handler_t,
}
pub type SnMonitorEventFunc =
    Option<unsafe extern "C" fn(*mut SnMonitorEvent, *mut ::core::ffi::c_void) -> ()>;
pub const __ASSERT_FUNCTION: [::core::ffi::c_char; 69] = unsafe {
    ::core::mem::transmute::<[u8; 69], [::core::ffi::c_char; 69]>(
        *b"_Bool handle_strut_partial_change(Con *, xcb_get_property_reply_t *)\0",
    )
};
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const XCB_KEY_PRESS: ::core::ffi::c_int = 2;
pub const XCB_KEY_RELEASE: ::core::ffi::c_int = 3;
pub const XCB_BUTTON_PRESS: ::core::ffi::c_int = 4;
pub const XCB_BUTTON_RELEASE: ::core::ffi::c_int = 5;
pub const XCB_MOTION_NOTIFY: ::core::ffi::c_int = 6;
pub const XCB_ENTER_NOTIFY: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const XCB_FOCUS_IN: ::core::ffi::c_int = 9;
pub const XCB_FOCUS_OUT: ::core::ffi::c_int = 10;
pub const XCB_EXPOSE: ::core::ffi::c_int = 12;
pub const XCB_DESTROY_NOTIFY: ::core::ffi::c_int = 17;
pub const XCB_UNMAP_NOTIFY: ::core::ffi::c_int = 18;
pub const XCB_MAP_REQUEST: ::core::ffi::c_int = 20;
pub const XCB_CONFIGURE_NOTIFY: ::core::ffi::c_int = 22;
pub const XCB_CONFIGURE_REQUEST: ::core::ffi::c_int = 23;
pub const XCB_PROPERTY_NOTIFY: ::core::ffi::c_int = 28;
pub const XCB_SELECTION_CLEAR: ::core::ffi::c_int = 29;
pub const XCB_CLIENT_MESSAGE: ::core::ffi::c_int = 33;
pub const XCB_MAPPING_NOTIFY: ::core::ffi::c_int = 34;
pub const XCB_NONE: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const XCB_XKB_NEW_KEYBOARD_NOTIFY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const XCB_XKB_MAP_NOTIFY: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const XCB_XKB_STATE_NOTIFY: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const XCB_RANDR_SCREEN_CHANGE_NOTIFY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const XCB_SHAPE_NOTIFY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const I3_IPC_EVENT_MASK: ::core::ffi::c_ulong =
    (1 as ::core::ffi::c_ulong) << 31 as ::core::ffi::c_int;
pub const I3_IPC_EVENT_OUTPUT: ::core::ffi::c_ulong = I3_IPC_EVENT_MASK | 1 as ::core::ffi::c_ulong;
pub const _NET_WM_STATE_REMOVE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const _NET_WM_STATE_ADD: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const _NET_WM_STATE_TOGGLE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const XCB_NUM_LOCK: ::core::ffi::c_int = 0xff7f as ::core::ffi::c_int;
pub const NET_WM_DESKTOP_ALL: ::core::ffi::c_uint = 0xffffffff as ::core::ffi::c_uint;
#[no_mangle]
pub static mut randr_base: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
#[no_mangle]
pub static mut xkb_base: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
#[no_mangle]
pub static mut xkb_current_group: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut shape_base: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
static mut ignore_events: ignore_head = ignore_head {
    slh_first: ::core::ptr::null::<Ignore_Event>() as *mut Ignore_Event,
};
#[no_mangle]
pub unsafe extern "C" fn add_ignore_event(
    sequence: ignore_event_sequence_t,
    response_type: ::core::ffi::c_int,
) {
    let mut event: *mut Ignore_Event =
        smalloc(::core::mem::size_of::<Ignore_Event>() as size_t) as *mut Ignore_Event;
    (*event).sequence = sequence;
    (*event).response_type = response_type;
    (*event).added = time(::core::ptr::null_mut::<time_t>());
    (*event).ignore_events.sle_next = ignore_events.slh_first;
    ignore_events.slh_first = event;
}
#[no_mangle]
pub unsafe extern "C" fn event_is_ignored(
    sequence: ignore_event_sequence_t,
    response_type: ::core::ffi::c_int,
) -> bool {
    let mut event: *mut Ignore_Event = ::core::ptr::null_mut::<Ignore_Event>();
    let mut now: time_t = time(::core::ptr::null_mut::<time_t>());
    event = ignore_events.slh_first;
    while !event.is_null() {
        if now - (*event).added > 5 as time_t {
            let mut save: *mut Ignore_Event = event;
            event = (*event).ignore_events.sle_next;
            if ignore_events.slh_first == save {
                ignore_events.slh_first = (*ignore_events.slh_first).ignore_events.sle_next;
            } else {
                let mut curelm: *mut Ignore_Event = ignore_events.slh_first;
                while (*curelm).ignore_events.sle_next != save {
                    curelm = (*curelm).ignore_events.sle_next;
                }
                (*curelm).ignore_events.sle_next =
                    (*(*curelm).ignore_events.sle_next).ignore_events.sle_next;
            }
            free(save as *mut ::core::ffi::c_void);
        } else {
            event = (*event).ignore_events.sle_next;
        }
    }
    event = ignore_events.slh_first;
    while !event.is_null() {
        if !((*event).sequence.value != sequence.value) {
            if !((*event).response_type != -(1 as ::core::ffi::c_int)
                && (*event).response_type != response_type)
            {
                return true_0 != 0;
            }
        }
        event = (*event).ignore_events.sle_next;
    }
    return false_0 != 0;
}
unsafe extern "C" fn check_crossing_screen_boundary(mut x: uint32_t, mut y: uint32_t) {
    let mut output: *mut Output = ::core::ptr::null_mut::<Output>();
    if config.disable_focus_follows_mouse {
        return;
    }
    output = get_output_containing(x as ::core::ffi::c_uint, y as ::core::ffi::c_uint);
    if output.is_null() {
        errorlog(
            b"ERROR: ERROR: No such screen\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        return;
    }
    if (*output).con.is_null() {
        errorlog(
            b"ERROR: ERROR: The screen is not recognized by i3 (no container associated)\n\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        return;
    }
    let mut old_focused: *mut Con = focused;
    let mut next: *mut Con = con_descend_focused(output_get_content((*output).con));
    workspace_show(con_get_workspace(next));
    con_focus(next);
    if old_focused != focused {
        tree_render();
    }
}
unsafe extern "C" fn handle_enter_notify(
    mut event: *mut xcb_enter_notify_event_t,
    mut full_sequence: uint32_t,
) {
    let mut con: *mut Con = ::core::ptr::null_mut::<Con>();
    last_timestamp = (*event).time;
    debuglog(
        b"%s:%s:%d - enter_notify for %08x, mode = %d, detail %d, serial %d\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"handle_enter_notify\0" as *const u8 as *const ::core::ffi::c_char,
        133 as ::core::ffi::c_int,
        (*event).event,
        (*event).mode as ::core::ffi::c_int,
        (*event).detail as ::core::ffi::c_int,
        (*event).sequence as ::core::ffi::c_int,
    );
    debuglog(
        b"%s:%s:%d - coordinates %d, %d\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"handle_enter_notify\0" as *const u8 as *const ::core::ffi::c_char,
        134 as ::core::ffi::c_int,
        (*event).event_x as ::core::ffi::c_int,
        (*event).event_y as ::core::ffi::c_int,
    );
    if (*event).mode as ::core::ffi::c_int != XCB_NOTIFY_MODE_NORMAL as ::core::ffi::c_int {
        debuglog(
            b"%s:%s:%d - This was not a normal notify, ignoring\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"handle_enter_notify\0" as *const u8 as *const ::core::ffi::c_char,
            136 as ::core::ffi::c_int,
        );
        return;
    }
    if event_is_ignored(
        ignore_event_sequence_t {
            value: full_sequence,
        },
        XCB_ENTER_NOTIFY,
    ) {
        debuglog(
            b"%s:%s:%d - Event ignored\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"handle_enter_notify\0" as *const u8 as *const ::core::ffi::c_char,
            142 as ::core::ffi::c_int,
        );
        return;
    }
    let mut enter_child: bool = false_0 != 0;
    con = con_by_frame_id((*event).event);
    if con.is_null() {
        con = con_by_window_id((*event).event);
        enter_child = true_0 != 0;
    }
    if con.is_null()
        || (*(*con).parent).type_0 as ::core::ffi::c_uint
            == CT_DOCKAREA as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        debuglog(
            b"%s:%s:%d - Getting screen at %d x %d\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"handle_enter_notify\0" as *const u8 as *const ::core::ffi::c_char,
            157 as ::core::ffi::c_int,
            (*event).root_x as ::core::ffi::c_int,
            (*event).root_y as ::core::ffi::c_int,
        );
        check_crossing_screen_boundary((*event).root_x as uint32_t, (*event).root_y as uint32_t);
        return;
    }
    let mut layout: layout_t = (if enter_child as ::core::ffi::c_int != 0 {
        (*(*con).parent).layout as ::core::ffi::c_uint
    } else {
        (*con).layout as ::core::ffi::c_uint
    }) as layout_t;
    if layout as ::core::ffi::c_uint == L_DEFAULT as ::core::ffi::c_int as ::core::ffi::c_uint {
        let mut child: *mut Con = ::core::ptr::null_mut::<Con>();
        child = *(*((*con).nodes_head.tqh_last as *mut nodes_head)).tqh_last as *mut Con;
        while !child.is_null() {
            if rect_contains(
                (*child).deco_rect as Rect,
                (*event).event_x as uint32_t,
                (*event).event_y as uint32_t,
            ) {
                verboselog(
                    b"using child %p / %s instead!\n\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    child,
                    (*child).name,
                );
                con = child;
                break;
            } else {
                child = *(*((*child).nodes.tqe_prev as *mut nodes_head)).tqh_last as *mut Con;
            }
        }
    }
    if config.disable_focus_follows_mouse {
        return;
    }
    if con == focused {
        return;
    }
    let mut ws: *mut Con = con_get_workspace(con);
    if ws != con_get_workspace(focused) {
        workspace_show(ws);
    }
    focused_id = XCB_NONE as xcb_window_t;
    con_focus(con_descend_focused(con));
    tree_render();
}
unsafe extern "C" fn handle_motion_notify(mut event: *mut xcb_motion_notify_event_t) {
    last_timestamp = (*event).time;
    if (*event).child as ::core::ffi::c_long != XCB_NONE {
        return;
    }
    let mut con: *mut Con = ::core::ptr::null_mut::<Con>();
    con = con_by_frame_id((*event).event);
    if con.is_null() {
        debuglog(
            b"%s:%s:%d - MotionNotify for an unknown container, checking if it crosses screen boundaries.\n\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"handle_motion_notify\0" as *const u8 as *const ::core::ffi::c_char,
            214 as ::core::ffi::c_int,
        );
        check_crossing_screen_boundary((*event).root_x as uint32_t, (*event).root_y as uint32_t);
        return;
    }
    if config.disable_focus_follows_mouse {
        return;
    }
    if (*con).layout as ::core::ffi::c_uint
        != L_DEFAULT as ::core::ffi::c_int as ::core::ffi::c_uint
        && (*con).layout as ::core::ffi::c_uint
            != L_SPLITV as ::core::ffi::c_int as ::core::ffi::c_uint
        && (*con).layout as ::core::ffi::c_uint
            != L_SPLITH as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return;
    }
    if !(*con).window.is_null() {
        if rect_contains(
            (*con).deco_rect as Rect,
            (*event).event_x as uint32_t,
            (*event).event_y as uint32_t,
        ) {
            if (*(*con).parent).focus_head.tqh_first == con {
                return;
            }
            con_focus(con);
            x_push_changes(croot);
            return;
        }
    } else {
        let mut current: *mut Con = ::core::ptr::null_mut::<Con>();
        current = *(*((*con).nodes_head.tqh_last as *mut nodes_head)).tqh_last as *mut Con;
        while !current.is_null() {
            if !rect_contains(
                (*current).deco_rect as Rect,
                (*event).event_x as uint32_t,
                (*event).event_y as uint32_t,
            ) {
                current = *(*((*current).nodes.tqe_prev as *mut nodes_head)).tqh_last as *mut Con;
            } else {
                if (*con).focus_head.tqh_first == current {
                    return;
                }
                con_focus(current);
                x_push_changes(croot);
                return;
            }
        }
    };
}
unsafe extern "C" fn handle_mapping_notify(mut event: *mut xcb_mapping_notify_event_t) {
    if (*event).request as ::core::ffi::c_int != XCB_MAPPING_KEYBOARD as ::core::ffi::c_int
        && (*event).request as ::core::ffi::c_int != XCB_MAPPING_MODIFIER as ::core::ffi::c_int
    {
        return;
    }
    debuglog(
        b"%s:%s:%d - Received mapping_notify for keyboard or modifier mapping, re-grabbing keys\n\0"
            as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"handle_mapping_notify\0" as *const u8 as *const ::core::ffi::c_char,
        269 as ::core::ffi::c_int,
    );
    xcb_refresh_keyboard_mapping(keysyms, event);
    xcb_numlock_mask =
        aio_get_mod_mask_for(XCB_NUM_LOCK as uint32_t, keysyms) as ::core::ffi::c_uint;
    ungrab_all_keys(conn);
    translate_keysyms();
    grab_all_keys(conn);
}
unsafe extern "C" fn handle_map_request(
    mut event: *const xcb_map_request_event_t,
    mut full_sequence: uint32_t,
) {
    let mut cookie: xcb_get_window_attributes_cookie_t =
        xcb_get_window_attributes_unchecked(conn, (*event).window);
    debuglog(
        b"%s:%s:%d - window = 0x%08x, serial is %d.\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"handle_map_request\0" as *const u8 as *const ::core::ffi::c_char,
        286 as ::core::ffi::c_int,
        (*event).window,
        (*event).sequence as ::core::ffi::c_int,
    );
    add_ignore_event(
        ignore_event_sequence_t {
            value: full_sequence,
        },
        -(1 as ::core::ffi::c_int),
    );
    manage_window((*event).window, cookie, false_0 != 0);
}
unsafe extern "C" fn handle_configure_request(mut event: *mut xcb_configure_request_event_t) {
    let mut fullscreen: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut con: *mut Con = ::core::ptr::null_mut::<Con>();
    debuglog(
        b"%s:%s:%d - window 0x%08x wants to be at %dx%d with %dx%d\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"handle_configure_request\0" as *const u8 as *const ::core::ffi::c_char,
        304 as ::core::ffi::c_int,
        (*event).window,
        (*event).x as ::core::ffi::c_int,
        (*event).y as ::core::ffi::c_int,
        (*event).width as ::core::ffi::c_int,
        (*event).height as ::core::ffi::c_int,
    );
    con = con_by_window_id((*event).window);
    if con.is_null() {
        debuglog(
            b"%s:%s:%d - Configure request for unmanaged window, can do that.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"handle_configure_request\0" as *const u8 as *const ::core::ffi::c_char,
            309 as ::core::ffi::c_int,
        );
        let mut mask: uint32_t = 0 as uint32_t;
        let mut values: [uint32_t; 7] = [0; 7];
        let mut c: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if (*event).value_mask as ::core::ffi::c_int & XCB_CONFIG_WINDOW_X as ::core::ffi::c_int
            != 0
        {
            mask |= XCB_CONFIG_WINDOW_X as ::core::ffi::c_int as uint32_t;
            let fresh0 = c;
            c = c + 1;
            values[fresh0 as usize] = (*event).x as uint32_t;
        }
        if (*event).value_mask as ::core::ffi::c_int & XCB_CONFIG_WINDOW_Y as ::core::ffi::c_int
            != 0
        {
            mask |= XCB_CONFIG_WINDOW_Y as ::core::ffi::c_int as uint32_t;
            let fresh1 = c;
            c = c + 1;
            values[fresh1 as usize] = (*event).y as uint32_t;
        }
        if (*event).value_mask as ::core::ffi::c_int & XCB_CONFIG_WINDOW_WIDTH as ::core::ffi::c_int
            != 0
        {
            mask |= XCB_CONFIG_WINDOW_WIDTH as ::core::ffi::c_int as uint32_t;
            let fresh2 = c;
            c = c + 1;
            values[fresh2 as usize] = (*event).width as uint32_t;
        }
        if (*event).value_mask as ::core::ffi::c_int
            & XCB_CONFIG_WINDOW_HEIGHT as ::core::ffi::c_int
            != 0
        {
            mask |= XCB_CONFIG_WINDOW_HEIGHT as ::core::ffi::c_int as uint32_t;
            let fresh3 = c;
            c = c + 1;
            values[fresh3 as usize] = (*event).height as uint32_t;
        }
        if (*event).value_mask as ::core::ffi::c_int
            & XCB_CONFIG_WINDOW_BORDER_WIDTH as ::core::ffi::c_int
            != 0
        {
            mask |= XCB_CONFIG_WINDOW_BORDER_WIDTH as ::core::ffi::c_int as uint32_t;
            let fresh4 = c;
            c = c + 1;
            values[fresh4 as usize] = (*event).border_width as uint32_t;
        }
        if (*event).value_mask as ::core::ffi::c_int
            & XCB_CONFIG_WINDOW_SIBLING as ::core::ffi::c_int
            != 0
        {
            mask |= XCB_CONFIG_WINDOW_SIBLING as ::core::ffi::c_int as uint32_t;
            let fresh5 = c;
            c = c + 1;
            values[fresh5 as usize] = (*event).sibling as uint32_t;
        }
        if (*event).value_mask as ::core::ffi::c_int
            & XCB_CONFIG_WINDOW_STACK_MODE as ::core::ffi::c_int
            != 0
        {
            mask |= XCB_CONFIG_WINDOW_STACK_MODE as ::core::ffi::c_int as uint32_t;
            let fresh6 = c;
            c = c + 1;
            values[fresh6 as usize] = (*event).stack_mode as uint32_t;
        }
        xcb_configure_window(
            conn,
            (*event).window,
            mask as uint16_t,
            &raw mut values as *mut uint32_t as *const ::core::ffi::c_void,
        );
        xcb_flush(conn);
        return;
    }
    debuglog(
        b"%s:%s:%d - Configure request!\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"handle_configure_request\0" as *const u8 as *const ::core::ffi::c_char,
        336 as ::core::ffi::c_int,
    );
    let mut workspace: *mut Con = con_get_workspace(con);
    if !workspace.is_null()
        && strcmp(
            (*workspace).name,
            b"__i3_scratch\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
    {
        debuglog(
            b"%s:%s:%d - This is a scratchpad container, ignoring ConfigureRequest\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"handle_configure_request\0" as *const u8 as *const ::core::ffi::c_char,
            340 as ::core::ffi::c_int,
        );
    } else {
        fullscreen = con_get_fullscreen_covering_ws(workspace);
        if fullscreen != con
            && con_is_floating(con) as ::core::ffi::c_int != 0
            && con_is_leaf(con) as ::core::ffi::c_int != 0
        {
            let mut bsr: Rect = con_border_style_rect(con);
            let mut floatingcon: *mut Con = (*con).parent as *mut Con;
            let mut newrect: Rect = (*floatingcon).rect as Rect;
            if (*event).value_mask as ::core::ffi::c_int & XCB_CONFIG_WINDOW_X as ::core::ffi::c_int
                != 0
            {
                newrect.x = ((*event).x as uint32_t)
                    .wrapping_add((-(1 as ::core::ffi::c_int) as uint32_t).wrapping_mul(bsr.x));
                debuglog(
                    b"%s:%s:%d - proposed x = %d, new x is %d\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
                    b"handle_configure_request\0" as *const u8 as *const ::core::ffi::c_char,
                    354 as ::core::ffi::c_int,
                    (*event).x as ::core::ffi::c_int,
                    newrect.x,
                );
            }
            if (*event).value_mask as ::core::ffi::c_int & XCB_CONFIG_WINDOW_Y as ::core::ffi::c_int
                != 0
            {
                newrect.y = ((*event).y as uint32_t)
                    .wrapping_add((-(1 as ::core::ffi::c_int) as uint32_t).wrapping_mul(bsr.y));
                debuglog(
                    b"%s:%s:%d - proposed y = %d, new y is %d\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
                    b"handle_configure_request\0" as *const u8 as *const ::core::ffi::c_char,
                    358 as ::core::ffi::c_int,
                    (*event).y as ::core::ffi::c_int,
                    newrect.y,
                );
            }
            if (*event).value_mask as ::core::ffi::c_int
                & XCB_CONFIG_WINDOW_WIDTH as ::core::ffi::c_int
                != 0
            {
                newrect.width = ((*event).width as uint32_t)
                    .wrapping_add((-(1 as ::core::ffi::c_int) as uint32_t).wrapping_mul(bsr.width));
                newrect.width = newrect
                    .width
                    .wrapping_add(((*con).border_width * 2 as ::core::ffi::c_int) as uint32_t);
                debuglog(
                    b"%s:%s:%d - proposed width = %d, new width is %d (x11 border %d)\n\0"
                        as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
                    b"handle_configure_request\0" as *const u8 as *const ::core::ffi::c_char,
                    364 as ::core::ffi::c_int,
                    (*event).width as ::core::ffi::c_int,
                    newrect.width,
                    (*con).border_width,
                );
            }
            if (*event).value_mask as ::core::ffi::c_int
                & XCB_CONFIG_WINDOW_HEIGHT as ::core::ffi::c_int
                != 0
            {
                newrect.height = ((*event).height as uint32_t).wrapping_add(
                    (-(1 as ::core::ffi::c_int) as uint32_t).wrapping_mul(bsr.height),
                );
                newrect.height = newrect
                    .height
                    .wrapping_add(((*con).border_width * 2 as ::core::ffi::c_int) as uint32_t);
                debuglog(
                    b"%s:%s:%d - proposed height = %d, new height is %d (x11 border %d)\n\0"
                        as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
                    b"handle_configure_request\0" as *const u8 as *const ::core::ffi::c_char,
                    370 as ::core::ffi::c_int,
                    (*event).height as ::core::ffi::c_int,
                    newrect.height,
                    (*con).border_width,
                );
            }
            debuglog(
                b"%s:%s:%d - Container is a floating leaf node, will do that.\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"handle_configure_request\0" as *const u8 as *const ::core::ffi::c_char,
                373 as ::core::ffi::c_int,
            );
            floating_reposition(floatingcon, newrect);
            return;
        }
        if !(*con).parent.is_null()
            && (*(*con).parent).type_0 as ::core::ffi::c_uint
                == CT_DOCKAREA as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            debuglog(
                b"%s:%s:%d - Reconfiguring dock window (con = %p).\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"handle_configure_request\0" as *const u8 as *const ::core::ffi::c_char,
                380 as ::core::ffi::c_int,
                con,
            );
            if (*event).value_mask as ::core::ffi::c_int
                & XCB_CONFIG_WINDOW_HEIGHT as ::core::ffi::c_int
                != 0
            {
                debuglog(
                    b"%s:%s:%d - Dock client wants to change height to %d, we can do that.\n\0"
                        as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
                    b"handle_configure_request\0" as *const u8 as *const ::core::ffi::c_char,
                    382 as ::core::ffi::c_int,
                    (*event).height as ::core::ffi::c_int,
                );
                (*con).geometry.height = (*event).height as uint32_t;
                tree_render();
            }
            if (*event).value_mask as ::core::ffi::c_int & XCB_CONFIG_WINDOW_X as ::core::ffi::c_int
                != 0
                || (*event).value_mask as ::core::ffi::c_int
                    & XCB_CONFIG_WINDOW_Y as ::core::ffi::c_int
                    != 0
            {
                let mut x: int16_t = (if (*event).value_mask as ::core::ffi::c_int
                    & XCB_CONFIG_WINDOW_X as ::core::ffi::c_int
                    != 0
                {
                    (*event).x as ::core::ffi::c_int
                } else {
                    (*con).geometry.x as int16_t as ::core::ffi::c_int
                }) as int16_t;
                let mut y: int16_t = (if (*event).value_mask as ::core::ffi::c_int
                    & XCB_CONFIG_WINDOW_Y as ::core::ffi::c_int
                    != 0
                {
                    (*event).y as ::core::ffi::c_int
                } else {
                    (*con).geometry.y as int16_t as ::core::ffi::c_int
                }) as int16_t;
                let mut current_output: *mut Con = con_get_output(con);
                let mut target: *mut Output =
                    get_output_containing(x as ::core::ffi::c_uint, y as ::core::ffi::c_uint);
                if !target.is_null() && current_output != (*target).con {
                    debuglog(
                        b"%s:%s:%d - Dock client is requested to be moved to output %s, moving it there.\n\0"
                            as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        b"../src/handlers.c\0" as *const u8
                            as *const ::core::ffi::c_char,
                        b"handle_configure_request\0" as *const u8
                            as *const ::core::ffi::c_char,
                        395 as ::core::ffi::c_int,
                        output_primary_name(target),
                    );
                    let mut match_0: *mut Match = ::core::ptr::null_mut::<Match>();
                    let mut nc: *mut Con = con_for_window(
                        (*target).con,
                        (*con).window as *mut i3Window,
                        &raw mut match_0,
                    );
                    debuglog(
                        b"%s:%s:%d - Dock client will be moved to container %p.\n\0" as *const u8
                            as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
                        b"handle_configure_request\0" as *const u8 as *const ::core::ffi::c_char,
                        398 as ::core::ffi::c_int,
                        nc,
                    );
                    con_detach(con);
                    con_attach(con, nc, false_0 != 0);
                    tree_render();
                } else {
                    debuglog(
                        b"%s:%s:%d - Dock client will not be moved, we only support moving it to another output.\n\0"
                            as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        b"../src/handlers.c\0" as *const u8
                            as *const ::core::ffi::c_char,
                        b"handle_configure_request\0" as *const u8
                            as *const ::core::ffi::c_char,
                        404 as ::core::ffi::c_int,
                    );
                }
            }
        } else if (*event).value_mask as ::core::ffi::c_int
            & XCB_CONFIG_WINDOW_STACK_MODE as ::core::ffi::c_int
            != 0
        {
            debuglog(
                b"%s:%s:%d - window 0x%08x wants to be stacked %d\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"handle_configure_request\0" as *const u8 as *const ::core::ffi::c_char,
                411 as ::core::ffi::c_int,
                (*event).window,
                (*event).stack_mode as ::core::ffi::c_int,
            );
            if (*event).stack_mode as ::core::ffi::c_int
                != XCB_STACK_MODE_ABOVE as ::core::ffi::c_int
            {
                debuglog(
                    b"%s:%s:%d - stack_mode != XCB_STACK_MODE_ABOVE, ignoring ConfigureRequest\n\0"
                        as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
                    b"handle_configure_request\0" as *const u8 as *const ::core::ffi::c_char,
                    416 as ::core::ffi::c_int,
                );
            } else if !fullscreen.is_null() || !con_is_leaf(con) {
                debuglog(
                    b"%s:%s:%d - fullscreen or not a leaf, ignoring ConfigureRequest\n\0"
                        as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
                    b"handle_configure_request\0" as *const u8 as *const ::core::ffi::c_char,
                    421 as ::core::ffi::c_int,
                );
            } else if workspace.is_null() {
                debuglog(
                    b"%s:%s:%d - Window is not being managed, ignoring ConfigureRequest\n\0"
                        as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
                    b"handle_configure_request\0" as *const u8 as *const ::core::ffi::c_char,
                    426 as ::core::ffi::c_int,
                );
            } else if config.focus_on_window_activation as ::core::ffi::c_uint
                == FOWA_FOCUS as ::core::ffi::c_int as ::core::ffi::c_uint
                || config.focus_on_window_activation as ::core::ffi::c_uint
                    == FOWA_SMART as ::core::ffi::c_int as ::core::ffi::c_uint
                    && workspace_is_visible(workspace) as ::core::ffi::c_int != 0
            {
                debuglog(
                    b"%s:%s:%d - Focusing con = %p\n\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
                    b"handle_configure_request\0" as *const u8 as *const ::core::ffi::c_char,
                    431 as ::core::ffi::c_int,
                    con,
                );
                workspace_show(workspace);
                con_activate_unblock(con);
                tree_render();
            } else if config.focus_on_window_activation as ::core::ffi::c_uint
                == FOWA_URGENT as ::core::ffi::c_int as ::core::ffi::c_uint
                || config.focus_on_window_activation as ::core::ffi::c_uint
                    == FOWA_SMART as ::core::ffi::c_int as ::core::ffi::c_uint
                    && !workspace_is_visible(workspace)
            {
                debuglog(
                    b"%s:%s:%d - Marking con = %p urgent\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
                    b"handle_configure_request\0" as *const u8 as *const ::core::ffi::c_char,
                    436 as ::core::ffi::c_int,
                    con,
                );
                con_set_urgency(con, true_0 != 0);
                con = remanage_window(con);
                tree_render();
            } else {
                debuglog(
                    b"%s:%s:%d - Ignoring request for con = %p.\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
                    b"handle_configure_request\0" as *const u8 as *const ::core::ffi::c_char,
                    441 as ::core::ffi::c_int,
                    con,
                );
            }
        }
    }
    fake_absolute_configure_notify(con);
}
unsafe extern "C" fn handle_screen_change(mut e: *mut xcb_generic_event_t) {
    debuglog(
        b"%s:%s:%d - RandR screen change\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"handle_screen_change\0" as *const u8 as *const ::core::ffi::c_char,
        455 as ::core::ffi::c_int,
    );
    let mut cookie: xcb_get_geometry_cookie_t = xcb_get_geometry(conn, root as xcb_drawable_t);
    let mut reply: *mut xcb_get_geometry_reply_t = xcb_get_geometry_reply(
        conn,
        cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply.is_null() {
        errorlog(
            b"ERROR: Could not get geometry of the root window, exiting\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        exit(EXIT_FAILURE);
    }
    debuglog(
        b"%s:%s:%d - root geometry reply: (%d, %d) %d x %d\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"handle_screen_change\0" as *const u8 as *const ::core::ffi::c_char,
        465 as ::core::ffi::c_int,
        (*reply).x as ::core::ffi::c_int,
        (*reply).y as ::core::ffi::c_int,
        (*reply).width as ::core::ffi::c_int,
        (*reply).height as ::core::ffi::c_int,
    );
    (*croot).rect.width = (*reply).width as uint32_t;
    (*croot).rect.height = (*reply).height as uint32_t;
    randr_query_outputs();
    scratchpad_fix_resolution();
    ipc_send_event(
        b"output\0" as *const u8 as *const ::core::ffi::c_char,
        I3_IPC_EVENT_OUTPUT as uint32_t,
        b"{\"change\":\"unspecified\"}\0" as *const u8 as *const ::core::ffi::c_char,
    );
}
unsafe extern "C" fn handle_unmap_notify_event(
    mut event: *mut xcb_unmap_notify_event_t,
    mut full_sequence: uint32_t,
) {
    debuglog(
        b"%s:%s:%d - UnmapNotify for 0x%08x (received from 0x%08x), serial %d\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"handle_unmap_notify_event\0" as *const u8 as *const ::core::ffi::c_char,
        483 as ::core::ffi::c_int,
        (*event).window,
        (*event).event,
        (*event).sequence as ::core::ffi::c_int,
    );
    let mut cookie: xcb_get_input_focus_cookie_t = xcb_get_input_focus_cookie_t { sequence: 0 };
    let mut con: *mut Con = con_by_window_id((*event).window);
    if con.is_null() {
        con = con_by_frame_id((*event).window);
        if con.is_null() {
            verboselog(
                b"Not a managed window, ignoring UnmapNotify event\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
            return;
        }
        if (*con).ignore_unmap as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            (*con).ignore_unmap = (*con).ignore_unmap.wrapping_sub(1);
        }
        cookie = xcb_get_input_focus(conn);
        debuglog(
            b"%s:%s:%d - ignore_unmap = %d for frame of container %p\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"handle_unmap_notify_event\0" as *const u8 as *const ::core::ffi::c_char,
            500 as ::core::ffi::c_int,
            (*con).ignore_unmap as ::core::ffi::c_int,
            con,
        );
    } else {
        cookie = xcb_get_input_focus(conn);
        if (*con).ignore_unmap as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            debuglog(
                b"%s:%s:%d - ignore_unmap = %d, dec\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"handle_unmap_notify_event\0" as *const u8 as *const ::core::ffi::c_char,
                508 as ::core::ffi::c_int,
                (*con).ignore_unmap as ::core::ffi::c_int,
            );
            (*con).ignore_unmap = (*con).ignore_unmap.wrapping_sub(1);
        } else {
            xcb_delete_property(conn, (*event).window, A__NET_WM_DESKTOP);
            xcb_delete_property(conn, (*event).window, A__NET_WM_STATE);
            tree_close_internal(con, DONT_KILL_WINDOW, false_0 != 0);
            tree_render();
        }
    }
    add_ignore_event(
        ignore_event_sequence_t {
            value: full_sequence,
        },
        XCB_ENTER_NOTIFY,
    );
    free(xcb_get_input_focus_reply(
        conn,
        cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    ) as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn handle_destroy_notify_event(
    mut event: *mut xcb_destroy_notify_event_t,
    mut full_sequence: uint32_t,
) {
    debuglog(
        b"%s:%s:%d - destroy notify for 0x%08x, 0x%08x\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"handle_destroy_notify_event\0" as *const u8 as *const ::core::ffi::c_char,
        555 as ::core::ffi::c_int,
        (*event).event,
        (*event).window,
    );
    let mut unmap: xcb_unmap_notify_event_t = xcb_unmap_notify_event_t {
        response_type: 0,
        pad0: 0,
        sequence: 0,
        event: 0,
        window: 0,
        from_configure: 0,
        pad1: [0; 3],
    };
    unmap.sequence = (*event).sequence;
    unmap.event = (*event).event;
    unmap.window = (*event).window;
    handle_unmap_notify_event(&raw mut unmap, full_sequence);
}
unsafe extern "C" fn window_name_changed(
    mut window: *mut i3Window,
    mut old_name: *mut ::core::ffi::c_char,
) -> bool {
    if old_name.is_null() && (*window).name.is_null() {
        return false_0 != 0;
    }
    if (old_name == NULL as *mut ::core::ffi::c_char) as ::core::ffi::c_int
        ^ ((*window).name == NULL as *mut i3String) as ::core::ffi::c_int
        != 0
    {
        return true_0 != 0;
    }
    return strcmp(old_name, i3string_as_utf8((*window).name)) != 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn handle_windowname_change(
    mut con: *mut Con,
    mut prop: *mut xcb_get_property_reply_t,
) -> bool {
    let mut old_name: *mut ::core::ffi::c_char = if !(*(*con).window).name.is_null() {
        sstrdup(i3string_as_utf8((*(*con).window).name))
    } else {
        ::core::ptr::null_mut::<::core::ffi::c_char>()
    };
    window_update_name((*con).window as *mut i3Window, prop);
    con = remanage_window(con);
    x_push_changes(croot);
    if window_name_changed((*con).window as *mut i3Window, old_name) {
        ipc_send_window_event(b"title\0" as *const u8 as *const ::core::ffi::c_char, con);
    }
    free(old_name as *mut ::core::ffi::c_void);
    old_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    return true_0 != 0;
}
unsafe extern "C" fn handle_windowname_change_legacy(
    mut con: *mut Con,
    mut prop: *mut xcb_get_property_reply_t,
) -> bool {
    let mut old_name: *mut ::core::ffi::c_char = if !(*(*con).window).name.is_null() {
        sstrdup(i3string_as_utf8((*(*con).window).name))
    } else {
        ::core::ptr::null_mut::<::core::ffi::c_char>()
    };
    window_update_name_legacy((*con).window as *mut i3Window, prop);
    con = remanage_window(con);
    x_push_changes(croot);
    if window_name_changed((*con).window as *mut i3Window, old_name) {
        ipc_send_window_event(b"title\0" as *const u8 as *const ::core::ffi::c_char, con);
    }
    free(old_name as *mut ::core::ffi::c_void);
    old_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    return true_0 != 0;
}
unsafe extern "C" fn handle_windowrole_change(
    mut con: *mut Con,
    mut prop: *mut xcb_get_property_reply_t,
) -> bool {
    window_update_role((*con).window as *mut i3Window, prop);
    remanage_window(con);
    return true_0 != 0;
}
unsafe extern "C" fn handle_expose_event(mut event: *mut xcb_expose_event_t) {
    let mut parent: *mut Con = ::core::ptr::null_mut::<Con>();
    debuglog(
        b"%s:%s:%d - window = %08x\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"handle_expose_event\0" as *const u8 as *const ::core::ffi::c_char,
        642 as ::core::ffi::c_int,
        (*event).window,
    );
    parent = con_by_frame_id((*event).window);
    if parent.is_null() {
        verboselog(
            b"expose event for unknown window, ignoring\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        return;
    }
    draw_util_copy_surface(
        &raw mut (*parent).frame_buffer,
        &raw mut (*parent).frame,
        0 as ::core::ffi::c_int as ::core::ffi::c_double,
        0 as ::core::ffi::c_int as ::core::ffi::c_double,
        0 as ::core::ffi::c_int as ::core::ffi::c_double,
        0 as ::core::ffi::c_int as ::core::ffi::c_double,
        (*parent).rect.width as ::core::ffi::c_double,
        (*parent).rect.height as ::core::ffi::c_double,
    );
    xcb_flush(conn);
}
pub const _NET_WM_MOVERESIZE_SIZE_TOPLEFT: uint32_t = 0 as uint32_t;
pub const _NET_WM_MOVERESIZE_MOVE: uint32_t = 8 as uint32_t;
pub const _NET_MOVERESIZE_WINDOW_X: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const _NET_MOVERESIZE_WINDOW_Y: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int;
pub const _NET_MOVERESIZE_WINDOW_WIDTH: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int;
pub const _NET_MOVERESIZE_WINDOW_HEIGHT: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 11 as ::core::ffi::c_int;
unsafe extern "C" fn handle_net_wm_state_change(
    mut con: *mut Con,
    mut change: uint32_t,
    mut atom: uint32_t,
) {
    if atom == 0 as uint32_t {
        return;
    }
    let mut debug_change: *const ::core::ffi::c_char = if change == _NET_WM_STATE_REMOVE as uint32_t
    {
        b"remove\0" as *const u8 as *const ::core::ffi::c_char
    } else if change == _NET_WM_STATE_ADD as uint32_t {
        b"add\0" as *const u8 as *const ::core::ffi::c_char
    } else {
        b"toggle\0" as *const u8 as *const ::core::ffi::c_char
    };
    if atom == A__NET_WM_STATE_FULLSCREEN {
        debuglog(
            b"%s:%s:%d - Received a client message to %s _NET_WM_STATE_FULLSCREEN.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"handle_net_wm_state_change\0" as *const u8 as *const ::core::ffi::c_char,
            682 as ::core::ffi::c_int,
            debug_change,
        );
        if change == _NET_WM_STATE_TOGGLE as uint32_t
            || (*con).fullscreen_mode as ::core::ffi::c_uint
                != CF_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
                && change == _NET_WM_STATE_REMOVE as uint32_t
            || (*con).fullscreen_mode as ::core::ffi::c_uint
                == CF_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
                && change == _NET_WM_STATE_ADD as uint32_t
        {
            debuglog(
                b"%s:%s:%d - toggling fullscreen\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"handle_net_wm_state_change\0" as *const u8 as *const ::core::ffi::c_char,
                688 as ::core::ffi::c_int,
            );
            con_toggle_fullscreen(con, CF_OUTPUT as ::core::ffi::c_int);
        }
    } else if atom == A__NET_WM_STATE_DEMANDS_ATTENTION {
        debuglog(
            b"%s:%s:%d - Received a client message to %s _NET_WM_STATE_DEMANDS_ATTENTION.\n\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"handle_net_wm_state_change\0" as *const u8 as *const ::core::ffi::c_char,
            692 as ::core::ffi::c_int,
            debug_change,
        );
        if change == _NET_WM_STATE_ADD as uint32_t {
            con_set_urgency(con, true_0 != 0);
            con = remanage_window(con);
        } else if change == _NET_WM_STATE_REMOVE as uint32_t {
            con_set_urgency(con, false_0 != 0);
            con = remanage_window(con);
        } else if change == _NET_WM_STATE_TOGGLE as uint32_t {
            con_set_urgency(con, !(*con).urgent);
            con = remanage_window(con);
        }
    } else if atom == A__NET_WM_STATE_STICKY {
        debuglog(
            b"%s:%s:%d - Received a client message to %s _NET_WM_STATE_STICKY.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"handle_net_wm_state_change\0" as *const u8 as *const ::core::ffi::c_char,
            706 as ::core::ffi::c_int,
            debug_change,
        );
        if change == _NET_WM_STATE_ADD as uint32_t {
            (*con).sticky = true_0 != 0;
        } else if change == _NET_WM_STATE_REMOVE as uint32_t {
            (*con).sticky = false_0 != 0;
        } else if change == _NET_WM_STATE_TOGGLE as uint32_t {
            (*con).sticky = !(*con).sticky;
        }
        debuglog(
            b"%s:%s:%d - New sticky status for con = %p is %i.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"handle_net_wm_state_change\0" as *const u8 as *const ::core::ffi::c_char,
            716 as ::core::ffi::c_int,
            con,
            (*con).sticky as ::core::ffi::c_int,
        );
        ewmh_update_sticky((*(*con).window).id, (*con).sticky);
        output_push_sticky_windows(focused);
        ewmh_update_wm_desktop();
    } else {
        debuglog(
            b"%s:%s:%d - Unknown atom in ClientMessage to %s type %u\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"handle_net_wm_state_change\0" as *const u8 as *const ::core::ffi::c_char,
            721 as ::core::ffi::c_int,
            debug_change,
            atom,
        );
        return;
    }
    tree_render();
}
unsafe extern "C" fn handle_client_message(mut event: *mut xcb_client_message_event_t) {
    if sn_xcb_display_process_event(sndisplay, event as *mut xcb_generic_event_t) != 0 {
        return;
    }
    verboselog(
        b"ClientMessage for window 0x%08x\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        (*event).window,
    );
    if (*event).type_0 == A__NET_WM_STATE {
        if (*event).format as ::core::ffi::c_int != 32 as ::core::ffi::c_int {
            debuglog(
                b"%s:%s:%d - Unknown format %d in ClientMessage\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"handle_client_message\0" as *const u8 as *const ::core::ffi::c_char,
                742 as ::core::ffi::c_int,
                (*event).format as ::core::ffi::c_int,
            );
            return;
        }
        let mut con: *mut Con = con_by_window_id((*event).window);
        if con.is_null() {
            debuglog(
                b"%s:%s:%d - Could not get window for client message\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"handle_client_message\0" as *const u8 as *const ::core::ffi::c_char,
                748 as ::core::ffi::c_int,
            );
            return;
        }
        let mut i: size_t = 0 as size_t;
        while i
            < (::core::mem::size_of::<[uint32_t; 5]>() as usize)
                .wrapping_div(::core::mem::size_of::<uint32_t>() as usize)
                .wrapping_sub(1 as usize)
        {
            handle_net_wm_state_change(
                con,
                (*event).data.data32[0 as ::core::ffi::c_int as usize],
                (*event).data.data32[i.wrapping_add(1 as size_t) as usize],
            );
            i = i.wrapping_add(1);
        }
    } else if (*event).type_0 == A__NET_ACTIVE_WINDOW {
        if (*event).format as ::core::ffi::c_int != 32 as ::core::ffi::c_int {
            return;
        }
        debuglog(
            b"%s:%s:%d - _NET_ACTIVE_WINDOW: Window 0x%08x should be activated\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"handle_client_message\0" as *const u8 as *const ::core::ffi::c_char,
            760 as ::core::ffi::c_int,
            (*event).window,
        );
        let mut con_0: *mut Con = con_by_window_id((*event).window);
        if con_0.is_null() {
            debuglog(
                b"%s:%s:%d - Could not get window for client message\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"handle_client_message\0" as *const u8 as *const ::core::ffi::c_char,
                764 as ::core::ffi::c_int,
            );
            return;
        }
        let mut ws: *mut Con = con_get_workspace(con_0);
        if ws.is_null() {
            debuglog(
                b"%s:%s:%d - Window is not being managed, ignoring _NET_ACTIVE_WINDOW\n\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"handle_client_message\0" as *const u8 as *const ::core::ffi::c_char,
                770 as ::core::ffi::c_int,
            );
            return;
        }
        if con_is_internal(ws) as ::core::ffi::c_int != 0
            && ws != workspace_get(b"__i3_scratch\0" as *const u8 as *const ::core::ffi::c_char)
        {
            debuglog(
                b"%s:%s:%d - Workspace is internal but not scratchpad, ignoring _NET_ACTIVE_WINDOW\n\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"handle_client_message\0" as *const u8 as *const ::core::ffi::c_char,
                775 as ::core::ffi::c_int,
            );
            return;
        }
        if (*event).data.data32[0 as ::core::ffi::c_int as usize] == 2 as uint32_t {
            debuglog(
                b"%s:%s:%d - This request came from a pager. Focusing con = %p\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"handle_client_message\0" as *const u8 as *const ::core::ffi::c_char,
                783 as ::core::ffi::c_int,
                con_0,
            );
            if con_is_internal(ws) {
                scratchpad_show(con_0);
            } else {
                workspace_show(ws);
                focused_id = XCB_NONE as xcb_window_t;
                con_activate_unblock(con_0);
            }
        } else {
            if con_is_internal(ws) {
                debuglog(
                    b"%s:%s:%d - Ignoring request to make con = %p active because it's on an internal workspace.\n\0"
                        as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
                    b"handle_client_message\0" as *const u8
                        as *const ::core::ffi::c_char,
                    796 as ::core::ffi::c_int,
                    con_0,
                );
                return;
            }
            if config.focus_on_window_activation as ::core::ffi::c_uint
                == FOWA_FOCUS as ::core::ffi::c_int as ::core::ffi::c_uint
                || config.focus_on_window_activation as ::core::ffi::c_uint
                    == FOWA_SMART as ::core::ffi::c_int as ::core::ffi::c_uint
                    && workspace_is_visible(ws) as ::core::ffi::c_int != 0
            {
                debuglog(
                    b"%s:%s:%d - Focusing con = %p\n\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
                    b"handle_client_message\0" as *const u8 as *const ::core::ffi::c_char,
                    801 as ::core::ffi::c_int,
                    con_0,
                );
                con_activate_unblock(con_0);
            } else if config.focus_on_window_activation as ::core::ffi::c_uint
                == FOWA_URGENT as ::core::ffi::c_int as ::core::ffi::c_uint
                || config.focus_on_window_activation as ::core::ffi::c_uint
                    == FOWA_SMART as ::core::ffi::c_int as ::core::ffi::c_uint
                    && !workspace_is_visible(ws)
            {
                debuglog(
                    b"%s:%s:%d - Marking con = %p urgent\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
                    b"handle_client_message\0" as *const u8 as *const ::core::ffi::c_char,
                    804 as ::core::ffi::c_int,
                    con_0,
                );
                con_set_urgency(con_0, true_0 != 0);
                con_0 = remanage_window(con_0);
            } else {
                debuglog(
                    b"%s:%s:%d - Ignoring request for con = %p.\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
                    b"handle_client_message\0" as *const u8 as *const ::core::ffi::c_char,
                    808 as ::core::ffi::c_int,
                    con_0,
                );
            }
        }
        tree_render();
    } else if (*event).type_0 == A_I3_SYNC {
        let mut window: xcb_window_t = (*event).data.data32[0 as ::core::ffi::c_int as usize];
        let mut rnd: uint32_t = (*event).data.data32[1 as ::core::ffi::c_int as usize];
        sync_respond(window, rnd);
    } else if (*event).type_0 == A__NET_REQUEST_FRAME_EXTENTS {
        debuglog(
            b"%s:%s:%d - _NET_REQUEST_FRAME_EXTENTS for window 0x%08x\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"handle_client_message\0" as *const u8 as *const ::core::ffi::c_char,
            830 as ::core::ffi::c_int,
            (*event).window,
        );
        let mut r: Rect = Rect {
            x: config.default_border_width as uint32_t,
            y: config.default_border_width as uint32_t,
            width: render_deco_height() as uint32_t,
            height: config.default_border_width as uint32_t,
        };
        xcb_change_property(
            conn,
            XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as uint8_t,
            (*event).window,
            A__NET_FRAME_EXTENTS,
            XCB_ATOM_CARDINAL as ::core::ffi::c_int as xcb_atom_t,
            32 as uint8_t,
            4 as uint32_t,
            &raw mut r as *const ::core::ffi::c_void,
        );
        xcb_flush(conn);
    } else if (*event).type_0 == A_WM_CHANGE_STATE {
        if (*event).data.data32[0 as ::core::ffi::c_int as usize]
            == XCB_ICCCM_WM_STATE_ICONIC as ::core::ffi::c_int as uint32_t
        {
            debuglog(
                b"%s:%s:%d - Client has requested iconic state, rejecting. (window = %08x)\n\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"handle_client_message\0" as *const u8 as *const ::core::ffi::c_char,
                852 as ::core::ffi::c_int,
                (*event).window,
            );
            let mut data: [::core::ffi::c_long; 2] = [
                XCB_ICCCM_WM_STATE_NORMAL as ::core::ffi::c_int as ::core::ffi::c_long,
                XCB_NONE,
            ];
            xcb_change_property(
                conn,
                XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as uint8_t,
                (*event).window,
                A_WM_STATE,
                A_WM_STATE,
                32 as uint8_t,
                2 as uint32_t,
                &raw mut data as *mut ::core::ffi::c_long as *const ::core::ffi::c_void,
            );
        } else {
            debuglog(
                b"%s:%s:%d - Not handling WM_CHANGE_STATE request. (window = %08x, state = %d)\n\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"handle_client_message\0" as *const u8 as *const ::core::ffi::c_char,
                857 as ::core::ffi::c_int,
                (*event).window,
                (*event).data.data32[0 as ::core::ffi::c_int as usize],
            );
        }
    } else if (*event).type_0 == A__NET_CURRENT_DESKTOP {
        debuglog(
            b"%s:%s:%d - Request to change current desktop to index %d\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"handle_client_message\0" as *const u8 as *const ::core::ffi::c_char,
            865 as ::core::ffi::c_int,
            (*event).data.data32[0 as ::core::ffi::c_int as usize],
        );
        let mut ws_0: *mut Con =
            ewmh_get_workspace_by_index((*event).data.data32[0 as ::core::ffi::c_int as usize]);
        if ws_0.is_null() {
            errorlog(
                b"ERROR: Could not determine workspace for this index, ignoring request.\n\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
            return;
        }
        debuglog(
            b"%s:%s:%d - Handling request to focus workspace %s\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"handle_client_message\0" as *const u8 as *const ::core::ffi::c_char,
            872 as ::core::ffi::c_int,
            (*ws_0).name,
        );
        workspace_show(ws_0);
        tree_render();
    } else if (*event).type_0 == A__NET_WM_DESKTOP {
        let mut index: uint32_t = (*event).data.data32[0 as ::core::ffi::c_int as usize];
        debuglog(
            b"%s:%s:%d - Request to move window %d to EWMH desktop index %d\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"handle_client_message\0" as *const u8 as *const ::core::ffi::c_char,
            877 as ::core::ffi::c_int,
            (*event).window,
            index,
        );
        let mut con_1: *mut Con = con_by_window_id((*event).window);
        if con_1.is_null() {
            debuglog(
                b"%s:%s:%d - Couldn't find con for window %d, ignoring the request.\n\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"handle_client_message\0" as *const u8 as *const ::core::ffi::c_char,
                881 as ::core::ffi::c_int,
                (*event).window,
            );
            return;
        }
        if index == NET_WM_DESKTOP_ALL as uint32_t {
            debuglog(
                b"%s:%s:%d - The window was requested to be visible on all workspaces, making it sticky and floating.\n\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"handle_client_message\0" as *const u8 as *const ::core::ffi::c_char,
                888 as ::core::ffi::c_int,
            );
            if floating_enable(con_1, false_0 != 0) {
                (*con_1).floating = FLOATING_AUTO_ON;
                (*con_1).sticky = true_0 != 0;
                ewmh_update_sticky((*(*con_1).window).id, true_0 != 0);
                output_push_sticky_windows(focused);
                run_assignments((*con_1).window as *mut i3Window);
            }
        } else {
            let mut ws_1: *mut Con = ewmh_get_workspace_by_index(index);
            if ws_1.is_null() {
                errorlog(
                    b"ERROR: Could not determine workspace for this index, ignoring request.\n\0"
                        as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
                return;
            }
            con_move_to_workspace(con_1, ws_1, true_0 != 0, false_0 != 0, false_0 != 0);
        }
        tree_render();
        ewmh_update_wm_desktop();
    } else if (*event).type_0 == A__NET_CLOSE_WINDOW {
        let mut con_2: *mut Con = con_by_window_id((*event).window);
        if !con_2.is_null() {
            debuglog(
                b"%s:%s:%d - Handling _NET_CLOSE_WINDOW request (con = %p)\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"handle_client_message\0" as *const u8 as *const ::core::ffi::c_char,
                918 as ::core::ffi::c_int,
                con_2,
            );
            if (*event).data.data32[0 as ::core::ffi::c_int as usize] != 0 {
                last_timestamp =
                    (*event).data.data32[0 as ::core::ffi::c_int as usize] as xcb_timestamp_t;
            }
            tree_close_internal(con_2, KILL_WINDOW, false_0 != 0);
            tree_render();
        } else {
            debuglog(
                b"%s:%s:%d - Couldn't find con for _NET_CLOSE_WINDOW request. (window = %08x)\n\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"handle_client_message\0" as *const u8 as *const ::core::ffi::c_char,
                927 as ::core::ffi::c_int,
                (*event).window,
            );
        }
    } else if (*event).type_0 == A__NET_WM_MOVERESIZE {
        let mut con_3: *mut Con = con_by_window_id((*event).window);
        if con_3.is_null() || !con_is_floating(con_3) {
            debuglog(
                b"%s:%s:%d - Couldn't find con for _NET_WM_MOVERESIZE request, or con not floating (window = %08x)\n\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"handle_client_message\0" as *const u8 as *const ::core::ffi::c_char,
                936 as ::core::ffi::c_int,
                (*event).window,
            );
            return;
        }
        debuglog(
            b"%s:%s:%d - Handling _NET_WM_MOVERESIZE request (con = %p)\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"handle_client_message\0" as *const u8 as *const ::core::ffi::c_char,
            939 as ::core::ffi::c_int,
            con_3,
        );
        let mut direction: uint32_t = (*event).data.data32[2 as ::core::ffi::c_int as usize];
        let mut x_root: uint32_t = (*event).data.data32[0 as ::core::ffi::c_int as usize];
        let mut y_root: uint32_t = (*event).data.data32[1 as ::core::ffi::c_int as usize];
        let mut fake: xcb_button_press_event_t = xcb_button_press_event_t {
            response_type: 0,
            detail: (*event).data.data32[3 as ::core::ffi::c_int as usize] as xcb_button_t,
            sequence: 0,
            time: 0,
            root: 0,
            event: 0,
            child: 0,
            root_x: x_root as int16_t,
            root_y: y_root as int16_t,
            event_x: x_root.wrapping_sub((*con_3).rect.x) as int16_t,
            event_y: y_root.wrapping_sub((*con_3).rect.y) as int16_t,
            state: 0,
            same_screen: 0,
            pad0: 0,
        };
        match direction {
            8 => {
                floating_drag_window((*con_3).parent as *mut Con, &raw mut fake, false_0 != 0);
            }
            0 => {
                floating_resize_window((*con_3).parent as *mut Con, false_0 != 0, &raw mut fake);
            }
            _ => {
                debuglog(
                    b"%s:%s:%d - _NET_WM_MOVERESIZE direction %d not implemented\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
                    b"handle_client_message\0" as *const u8 as *const ::core::ffi::c_char,
                    958 as ::core::ffi::c_int,
                    direction,
                );
            }
        }
    } else if (*event).type_0 == A__NET_MOVERESIZE_WINDOW {
        debuglog(
            b"%s:%s:%d - Received _NET_MOVE_RESIZE_WINDOW. Handling by faking a configure request.\n\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"handle_client_message\0" as *const u8 as *const ::core::ffi::c_char,
            962 as ::core::ffi::c_int,
        );
        let mut _generated_event: *mut ::core::ffi::c_void = scalloc(32 as size_t, 1 as size_t);
        let mut generated_event: *mut xcb_configure_request_event_t =
            _generated_event as *mut xcb_configure_request_event_t;
        (*generated_event).window = (*event).window;
        (*generated_event).response_type = XCB_CONFIGURE_REQUEST as uint8_t;
        (*generated_event).value_mask = 0 as uint16_t;
        if (*event).data.data32[0 as ::core::ffi::c_int as usize]
            & _NET_MOVERESIZE_WINDOW_X as uint32_t
            != 0
        {
            (*generated_event).value_mask = ((*generated_event).value_mask as ::core::ffi::c_int
                | XCB_CONFIG_WINDOW_X as ::core::ffi::c_int)
                as uint16_t;
            (*generated_event).x =
                (*event).data.data32[1 as ::core::ffi::c_int as usize] as int16_t;
        }
        if (*event).data.data32[0 as ::core::ffi::c_int as usize]
            & _NET_MOVERESIZE_WINDOW_Y as uint32_t
            != 0
        {
            (*generated_event).value_mask = ((*generated_event).value_mask as ::core::ffi::c_int
                | XCB_CONFIG_WINDOW_Y as ::core::ffi::c_int)
                as uint16_t;
            (*generated_event).y =
                (*event).data.data32[2 as ::core::ffi::c_int as usize] as int16_t;
        }
        if (*event).data.data32[0 as ::core::ffi::c_int as usize]
            & _NET_MOVERESIZE_WINDOW_WIDTH as uint32_t
            != 0
        {
            (*generated_event).value_mask = ((*generated_event).value_mask as ::core::ffi::c_int
                | XCB_CONFIG_WINDOW_WIDTH as ::core::ffi::c_int)
                as uint16_t;
            (*generated_event).width =
                (*event).data.data32[3 as ::core::ffi::c_int as usize] as uint16_t;
        }
        if (*event).data.data32[0 as ::core::ffi::c_int as usize]
            & _NET_MOVERESIZE_WINDOW_HEIGHT as uint32_t
            != 0
        {
            (*generated_event).value_mask = ((*generated_event).value_mask as ::core::ffi::c_int
                | XCB_CONFIG_WINDOW_HEIGHT as ::core::ffi::c_int)
                as uint16_t;
            (*generated_event).height =
                (*event).data.data32[4 as ::core::ffi::c_int as usize] as uint16_t;
        }
        handle_configure_request(generated_event);
        free(generated_event as *mut ::core::ffi::c_void);
        generated_event = ::core::ptr::null_mut::<xcb_configure_request_event_t>();
    } else {
        debuglog(
            b"%s:%s:%d - Skipping client message for unhandled type %d\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"handle_client_message\0" as *const u8 as *const ::core::ffi::c_char,
            991 as ::core::ffi::c_int,
            (*event).type_0,
        );
    };
}
unsafe extern "C" fn handle_window_type(
    mut con: *mut Con,
    mut reply: *mut xcb_get_property_reply_t,
) -> bool {
    window_update_type((*con).window as *mut i3Window, reply);
    return true_0 != 0;
}
unsafe extern "C" fn handle_normal_hints(
    mut con: *mut Con,
    mut reply: *mut xcb_get_property_reply_t,
) -> bool {
    let mut changed: bool = window_update_normal_hints(
        (*con).window as *mut i3Window,
        reply,
        ::core::ptr::null_mut::<xcb_get_geometry_reply_t>(),
    );
    if changed {
        let mut floating: *mut Con = con_inside_floating(con);
        if !floating.is_null() {
            floating_check_size(con, false_0 != 0);
            tree_render();
        }
    }
    free(reply as *mut ::core::ffi::c_void);
    reply = ::core::ptr::null_mut::<xcb_get_property_reply_t>();
    return true_0 != 0;
}
unsafe extern "C" fn handle_hints(
    mut con: *mut Con,
    mut reply: *mut xcb_get_property_reply_t,
) -> bool {
    let mut urgency_hint: bool = false;
    window_update_hints((*con).window as *mut i3Window, reply, &raw mut urgency_hint);
    con_set_urgency(con, urgency_hint);
    remanage_window(con);
    tree_render();
    return true_0 != 0;
}
unsafe extern "C" fn handle_transient_for(
    mut con: *mut Con,
    mut prop: *mut xcb_get_property_reply_t,
) -> bool {
    window_update_transient_for((*con).window as *mut i3Window, prop);
    return true_0 != 0;
}
unsafe extern "C" fn handle_clientleader_change(
    mut con: *mut Con,
    mut prop: *mut xcb_get_property_reply_t,
) -> bool {
    window_update_leader((*con).window as *mut i3Window, prop);
    return true_0 != 0;
}
unsafe extern "C" fn handle_focus_in(mut event: *mut xcb_focus_in_event_t) {
    debuglog(
        b"%s:%s:%d - focus change in, for window 0x%08x\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"handle_focus_in\0" as *const u8 as *const ::core::ffi::c_char,
        1064 as ::core::ffi::c_int,
        (*event).event,
    );
    if (*event).event == root {
        debuglog(
            b"%s:%s:%d - Received focus in for root window, refocusing the focused window.\n\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"handle_focus_in\0" as *const u8 as *const ::core::ffi::c_char,
            1067 as ::core::ffi::c_int,
        );
        con_focus(focused);
        focused_id = XCB_NONE as xcb_window_t;
        x_push_changes(croot);
    }
    let mut con: *mut Con = ::core::ptr::null_mut::<Con>();
    con = con_by_window_id((*event).event);
    if con.is_null() || (*con).window.is_null() {
        return;
    }
    debuglog(
        b"%s:%s:%d - That is con %p / %s\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"handle_focus_in\0" as *const u8 as *const ::core::ffi::c_char,
        1077 as ::core::ffi::c_int,
        con,
        (*con).name,
    );
    if (*event).mode as ::core::ffi::c_int == XCB_NOTIFY_MODE_GRAB as ::core::ffi::c_int
        || (*event).mode as ::core::ffi::c_int == XCB_NOTIFY_MODE_UNGRAB as ::core::ffi::c_int
    {
        debuglog(
            b"%s:%s:%d - FocusIn event for grab/ungrab, ignoring\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"handle_focus_in\0" as *const u8 as *const ::core::ffi::c_char,
            1081 as ::core::ffi::c_int,
        );
        return;
    }
    if (*event).detail as ::core::ffi::c_int == XCB_NOTIFY_DETAIL_POINTER as ::core::ffi::c_int {
        debuglog(
            b"%s:%s:%d - notify detail is pointer, ignoring this event\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"handle_focus_in\0" as *const u8 as *const ::core::ffi::c_char,
            1086 as ::core::ffi::c_int,
        );
        return;
    }
    if focused_id == (*event).event && con_inside_floating(con).is_null() {
        debuglog(
            b"%s:%s:%d - focus matches the currently focused window, not doing anything\n\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"handle_focus_in\0" as *const u8 as *const ::core::ffi::c_char,
            1093 as ::core::ffi::c_int,
        );
        return;
    }
    if (*(*con).parent).type_0 as ::core::ffi::c_uint
        == CT_DOCKAREA as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        debuglog(
            b"%s:%s:%d - This is a dock client, not focusing.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"handle_focus_in\0" as *const u8 as *const ::core::ffi::c_char,
            1099 as ::core::ffi::c_int,
        );
        return;
    }
    debuglog(
        b"%s:%s:%d - focus is different / refocusing floating window: updating decorations\n\0"
            as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"handle_focus_in\0" as *const u8 as *const ::core::ffi::c_char,
        1103 as ::core::ffi::c_int,
    );
    con_activate_unblock(con);
    focused_id = (*event).event;
    tree_render();
}
unsafe extern "C" fn handle_focus_out(mut event: *mut xcb_focus_in_event_t) {
    let mut con: *mut Con = con_by_window_id((*event).event);
    let mut window_name: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut mode: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut detail: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if !con.is_null() {
        window_name = (*con).name;
        if window_name.is_null() {
            window_name = b"<unnamed con>\0" as *const u8 as *const ::core::ffi::c_char;
        }
    } else if (*event).event == root {
        window_name = b"<the root window>\0" as *const u8 as *const ::core::ffi::c_char;
    } else {
        window_name = b"<unknown window>\0" as *const u8 as *const ::core::ffi::c_char;
    }
    match (*event).mode as ::core::ffi::c_int {
        0 => {
            mode = b"Normal\0" as *const u8 as *const ::core::ffi::c_char;
        }
        1 => {
            mode = b"Grab\0" as *const u8 as *const ::core::ffi::c_char;
        }
        2 => {
            mode = b"Ungrab\0" as *const u8 as *const ::core::ffi::c_char;
        }
        3 => {
            mode = b"WhileGrabbed\0" as *const u8 as *const ::core::ffi::c_char;
        }
        _ => {
            mode = b"<unknown>\0" as *const u8 as *const ::core::ffi::c_char;
        }
    }
    match (*event).detail as ::core::ffi::c_int {
        0 => {
            detail = b"Ancestor\0" as *const u8 as *const ::core::ffi::c_char;
        }
        1 => {
            detail = b"Virtual\0" as *const u8 as *const ::core::ffi::c_char;
        }
        2 => {
            detail = b"Inferior\0" as *const u8 as *const ::core::ffi::c_char;
        }
        3 => {
            detail = b"Nonlinear\0" as *const u8 as *const ::core::ffi::c_char;
        }
        4 => {
            detail = b"NonlinearVirtual\0" as *const u8 as *const ::core::ffi::c_char;
        }
        5 => {
            detail = b"Pointer\0" as *const u8 as *const ::core::ffi::c_char;
        }
        6 => {
            detail = b"PointerRoot\0" as *const u8 as *const ::core::ffi::c_char;
        }
        7 => {
            detail = b"NONE\0" as *const u8 as *const ::core::ffi::c_char;
        }
        _ => {
            detail = b"unknown\0" as *const u8 as *const ::core::ffi::c_char;
        }
    }
    debuglog(
        b"%s:%s:%d - focus change out: window 0x%08x (con %p, %s) lost focus with detail=%s, mode=%s\n\0"
            as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"handle_focus_out\0" as *const u8 as *const ::core::ffi::c_char,
        1179 as ::core::ffi::c_int,
        (*event).event,
        con,
        window_name,
        detail,
        mode,
    );
}
unsafe extern "C" fn handle_configure_notify(mut event: *mut xcb_configure_notify_event_t) {
    if (*event).event != root {
        debuglog(
            b"%s:%s:%d - ConfigureNotify for non-root window 0x%08x, ignoring\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"handle_configure_notify\0" as *const u8 as *const ::core::ffi::c_char,
            1189 as ::core::ffi::c_int,
            (*event).event,
        );
        return;
    }
    debuglog(
        b"%s:%s:%d - ConfigureNotify for root window 0x%08x\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"handle_configure_notify\0" as *const u8 as *const ::core::ffi::c_char,
        1192 as ::core::ffi::c_int,
        (*event).event,
    );
    if force_xinerama {
        return;
    }
    randr_query_outputs();
    ipc_send_event(
        b"output\0" as *const u8 as *const ::core::ffi::c_char,
        I3_IPC_EVENT_OUTPUT as uint32_t,
        b"{\"change\":\"unspecified\"}\0" as *const u8 as *const ::core::ffi::c_char,
    );
}
unsafe extern "C" fn handle_selection_clear(mut event: *mut xcb_selection_clear_event_t) {
    if (*event).selection != wm_sn {
        debuglog(
            b"%s:%s:%d - SelectionClear for unknown selection %d, ignoring\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"handle_selection_clear\0" as *const u8 as *const ::core::ffi::c_char,
            1208 as ::core::ffi::c_int,
            (*event).selection,
        );
        return;
    }
    verboselog(
        b"Lost WM_Sn selection, exiting.\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    );
    exit(EXIT_SUCCESS);
}
unsafe extern "C" fn handle_class_change(
    mut con: *mut Con,
    mut prop: *mut xcb_get_property_reply_t,
) -> bool {
    window_update_class((*con).window as *mut i3Window, prop);
    con = remanage_window(con);
    return true_0 != 0;
}
unsafe extern "C" fn handle_machine_change(
    mut con: *mut Con,
    mut prop: *mut xcb_get_property_reply_t,
) -> bool {
    window_update_machine((*con).window as *mut i3Window, prop);
    con = remanage_window(con);
    return true_0 != 0;
}
unsafe extern "C" fn handle_motif_hints_change(
    mut con: *mut Con,
    mut prop: *mut xcb_get_property_reply_t,
) -> bool {
    let mut motif_border_style: border_style_t = BS_NONE;
    let mut has_mwm_hints: bool = window_update_motif_hints(
        (*con).window as *mut i3Window,
        prop,
        &raw mut motif_border_style,
    );
    if has_mwm_hints as ::core::ffi::c_int != 0
        && motif_border_style as ::core::ffi::c_uint != (*con).border_style as ::core::ffi::c_uint
    {
        debuglog(
            b"%s:%s:%d - Update border style of con %p to %d\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"handle_motif_hints_change\0" as *const u8 as *const ::core::ffi::c_char,
            1246 as ::core::ffi::c_int,
            con,
            motif_border_style as ::core::ffi::c_uint,
        );
        con_set_border_style(con, motif_border_style, (*con).current_border_width);
        x_push_changes(croot);
    }
    return true_0 != 0;
}
unsafe extern "C" fn handle_strut_partial_change(
    mut con: *mut Con,
    mut prop: *mut xcb_get_property_reply_t,
) -> bool {
    window_update_strut_partial((*con).window as *mut i3Window, prop);
    if (*con).parent.is_null()
        || (*(*con).parent).type_0 as ::core::ffi::c_uint
            != CT_DOCKAREA as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return true_0 != 0;
    }
    let mut search_at: *mut Con = croot;
    let mut output: *mut Con = con_get_output(con);
    if !output.is_null() {
        debuglog(
            b"%s:%s:%d - Starting search at output %s\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"handle_strut_partial_change\0" as *const u8 as *const ::core::ffi::c_char,
            1270 as ::core::ffi::c_int,
            (*output).name,
        );
        search_at = output;
    }
    if (*(*con).window).reserved.top > 0 as uint32_t
        && (*(*con).window).reserved.bottom == 0 as uint32_t
    {
        debuglog(
            b"%s:%s:%d - Top dock client\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"handle_strut_partial_change\0" as *const u8 as *const ::core::ffi::c_char,
            1276 as ::core::ffi::c_int,
        );
        (*(*con).window).dock = W_DOCK_TOP;
    } else if (*(*con).window).reserved.top == 0 as uint32_t
        && (*(*con).window).reserved.bottom > 0 as uint32_t
    {
        debuglog(
            b"%s:%s:%d - Bottom dock client\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"handle_strut_partial_change\0" as *const u8 as *const ::core::ffi::c_char,
            1279 as ::core::ffi::c_int,
        );
        (*(*con).window).dock = W_DOCK_BOTTOM;
    } else {
        debuglog(
            b"%s:%s:%d - Ignoring invalid reserved edges (_NET_WM_STRUT_PARTIAL), using position as fallback:\n\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"handle_strut_partial_change\0" as *const u8 as *const ::core::ffi::c_char,
            1282 as ::core::ffi::c_int,
        );
        if (*con).geometry.y < (*search_at).rect.height.wrapping_div(2 as uint32_t) {
            debuglog(
                b"%s:%s:%d - geom->y = %d < rect.height / 2 = %d, it is a top dock client\n\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"handle_strut_partial_change\0" as *const u8 as *const ::core::ffi::c_char,
                1285 as ::core::ffi::c_int,
                (*con).geometry.y,
                (*search_at).rect.height.wrapping_div(2 as uint32_t),
            );
            (*(*con).window).dock = W_DOCK_TOP;
        } else {
            debuglog(
                b"%s:%s:%d - geom->y = %d >= rect.height / 2 = %d, it is a bottom dock client\n\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"handle_strut_partial_change\0" as *const u8 as *const ::core::ffi::c_char,
                1289 as ::core::ffi::c_int,
                (*con).geometry.y,
                (*search_at).rect.height.wrapping_div(2 as uint32_t),
            );
            (*(*con).window).dock = W_DOCK_BOTTOM;
        }
    }
    let mut dockarea: *mut Con = con_for_window(
        search_at,
        (*con).window as *mut i3Window,
        ::core::ptr::null_mut::<*mut Match>(),
    );
    if dockarea != ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut Con {
    } else {
        __assert_fail(
            b"dockarea != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
            1296 as ::core::ffi::c_uint,
            __ASSERT_FUNCTION.as_ptr(),
        );
    };
    con_detach(con);
    con_attach(con, dockarea, true_0 != 0);
    tree_render();
    return true_0 != 0;
}
unsafe extern "C" fn handle_windowicon_change(
    mut con: *mut Con,
    mut prop: *mut xcb_get_property_reply_t,
) -> bool {
    window_update_icon((*con).window as *mut i3Window, prop);
    x_push_changes(croot);
    return true_0 != 0;
}
static mut property_handlers: [property_handler_t; 13] = [property_handler_t {
    atom: 0,
    long_len: 0,
    cb: None,
}; 13];
pub const NUM_HANDLERS: usize = (::core::mem::size_of::<[property_handler_t; 13]>() as usize)
    .wrapping_div(::core::mem::size_of::<property_handler_t>() as usize);
#[no_mangle]
pub unsafe extern "C" fn property_handlers_init() {
    sn_monitor_context_new(
        sndisplay,
        conn_screen,
        Some(
            startup_monitor_event
                as unsafe extern "C" fn(*mut SnMonitorEvent, *mut ::core::ffi::c_void) -> (),
        ),
        NULL,
        None,
    );
    property_handlers[0 as ::core::ffi::c_int as usize].atom = A__NET_WM_NAME;
    property_handlers[1 as ::core::ffi::c_int as usize].atom =
        XCB_ATOM_WM_HINTS as ::core::ffi::c_int as xcb_atom_t;
    property_handlers[2 as ::core::ffi::c_int as usize].atom =
        XCB_ATOM_WM_NAME as ::core::ffi::c_int as xcb_atom_t;
    property_handlers[3 as ::core::ffi::c_int as usize].atom =
        XCB_ATOM_WM_NORMAL_HINTS as ::core::ffi::c_int as xcb_atom_t;
    property_handlers[4 as ::core::ffi::c_int as usize].atom = A_WM_CLIENT_LEADER;
    property_handlers[5 as ::core::ffi::c_int as usize].atom =
        XCB_ATOM_WM_TRANSIENT_FOR as ::core::ffi::c_int as xcb_atom_t;
    property_handlers[6 as ::core::ffi::c_int as usize].atom = A_WM_WINDOW_ROLE;
    property_handlers[7 as ::core::ffi::c_int as usize].atom =
        XCB_ATOM_WM_CLASS as ::core::ffi::c_int as xcb_atom_t;
    property_handlers[8 as ::core::ffi::c_int as usize].atom = A__NET_WM_STRUT_PARTIAL;
    property_handlers[9 as ::core::ffi::c_int as usize].atom = A__NET_WM_WINDOW_TYPE;
    property_handlers[10 as ::core::ffi::c_int as usize].atom =
        XCB_ATOM_WM_CLIENT_MACHINE as ::core::ffi::c_int as xcb_atom_t;
    property_handlers[11 as ::core::ffi::c_int as usize].atom = A__MOTIF_WM_HINTS;
    property_handlers[12 as ::core::ffi::c_int as usize].atom = A__NET_WM_ICON;
}
unsafe extern "C" fn property_notify(
    mut state: uint8_t,
    mut window: xcb_window_t,
    mut atom: xcb_atom_t,
) {
    let mut handler: *mut property_handler_t = ::core::ptr::null_mut::<property_handler_t>();
    let mut propr: *mut xcb_get_property_reply_t =
        ::core::ptr::null_mut::<xcb_get_property_reply_t>();
    let mut err: *mut xcb_generic_error_t = ::core::ptr::null_mut::<xcb_generic_error_t>();
    let mut con: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut c: size_t = 0 as size_t;
    while c < NUM_HANDLERS {
        if property_handlers[c as usize].atom != atom {
            c = c.wrapping_add(1);
        } else {
            handler = (&raw mut property_handlers as *mut property_handler_t).offset(c as isize)
                as *mut property_handler_t;
            break;
        }
    }
    if handler.is_null() {
        return;
    }
    con = con_by_window_id(window);
    if con.is_null() || (*con).window.is_null() {
        debuglog(
            b"%s:%s:%d - Received property for atom %d for unknown client\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"property_notify\0" as *const u8 as *const ::core::ffi::c_char,
            1385 as ::core::ffi::c_int,
            atom,
        );
        return;
    }
    if state as ::core::ffi::c_int != XCB_PROPERTY_DELETE as ::core::ffi::c_int {
        let mut cookie: xcb_get_property_cookie_t = xcb_get_property(
            conn,
            0 as uint8_t,
            window,
            atom,
            XCB_GET_PROPERTY_TYPE_ANY as ::core::ffi::c_int as xcb_atom_t,
            0 as uint32_t,
            (*handler).long_len,
        );
        propr = xcb_get_property_reply(conn, cookie, &raw mut err);
        if !err.is_null() {
            debuglog(
                b"%s:%s:%d - got error %d when getting property of atom %d\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"property_notify\0" as *const u8 as *const ::core::ffi::c_char,
                1393 as ::core::ffi::c_int,
                (*err).error_code as ::core::ffi::c_int,
                atom,
            );
            free(err as *mut ::core::ffi::c_void);
            err = ::core::ptr::null_mut::<xcb_generic_error_t>();
            return;
        }
    }
    if !(*handler).cb.expect("non-null function pointer")(con, propr) {
        free(propr as *mut ::core::ffi::c_void);
        propr = ::core::ptr::null_mut::<xcb_get_property_reply_t>();
    }
}
#[no_mangle]
pub unsafe extern "C" fn handle_event(
    mut type_0: ::core::ffi::c_int,
    mut event: *mut xcb_generic_event_t,
) {
    if type_0 != XCB_MOTION_NOTIFY {
        debuglog(
            b"%s:%s:%d - event type %d, xkb_base %d\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"handle_event\0" as *const u8 as *const ::core::ffi::c_char,
            1412 as ::core::ffi::c_int,
            type_0,
            xkb_base,
        );
    }
    if randr_base > -(1 as ::core::ffi::c_int)
        && type_0 == randr_base + XCB_RANDR_SCREEN_CHANGE_NOTIFY
    {
        handle_screen_change(event);
        return;
    }
    if xkb_base > -(1 as ::core::ffi::c_int) && type_0 == xkb_base {
        debuglog(
            b"%s:%s:%d - xkb event, need to handle it.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"handle_event\0" as *const u8 as *const ::core::ffi::c_char,
            1422 as ::core::ffi::c_int,
        );
        let mut state: *mut xcb_xkb_state_notify_event_t =
            event as *mut xcb_xkb_state_notify_event_t;
        if (*state).xkbType as ::core::ffi::c_int == XCB_XKB_NEW_KEYBOARD_NOTIFY {
            debuglog(
                b"%s:%s:%d - xkb new keyboard notify, sequence %d, time %d\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"handle_event\0" as *const u8 as *const ::core::ffi::c_char,
                1426 as ::core::ffi::c_int,
                (*state).sequence as ::core::ffi::c_int,
                (*state).time,
            );
            xcb_key_symbols_free(keysyms);
            keysyms = xcb_key_symbols_alloc(conn);
            if (*(event as *mut xcb_xkb_new_keyboard_notify_event_t)).changed as ::core::ffi::c_int
                & XCB_XKB_NKN_DETAIL_KEYCODES as ::core::ffi::c_int
                != 0
            {
                load_keymap();
            }
            ungrab_all_keys(conn);
            translate_keysyms();
            grab_all_keys(conn);
        } else if (*state).xkbType as ::core::ffi::c_int == XCB_XKB_MAP_NOTIFY {
            if event_is_ignored(
                ignore_event_sequence_t {
                    value: (*event).full_sequence,
                },
                type_0,
            ) {
                debuglog(
                    b"%s:%s:%d - Ignoring map notify event for sequence %d.\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
                    b"handle_event\0" as *const u8 as *const ::core::ffi::c_char,
                    1437 as ::core::ffi::c_int,
                    (*state).sequence as ::core::ffi::c_int,
                );
            } else {
                debuglog(
                    b"%s:%s:%d - xkb map notify, sequence %d, time %d\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
                    b"handle_event\0" as *const u8 as *const ::core::ffi::c_char,
                    1439 as ::core::ffi::c_int,
                    (*state).sequence as ::core::ffi::c_int,
                    (*state).time,
                );
                add_ignore_event(
                    ignore_event_sequence_t {
                        value: (*event).full_sequence,
                    },
                    type_0,
                );
                xcb_key_symbols_free(keysyms);
                keysyms = xcb_key_symbols_alloc(conn);
                ungrab_all_keys(conn);
                translate_keysyms();
                grab_all_keys(conn);
                load_keymap();
            }
        } else if (*state).xkbType as ::core::ffi::c_int == XCB_XKB_STATE_NOTIFY {
            debuglog(
                b"%s:%s:%d - xkb state group = %d\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"handle_event\0" as *const u8 as *const ::core::ffi::c_char,
                1449 as ::core::ffi::c_int,
                (*state).group as ::core::ffi::c_int,
            );
            if xkb_current_group == (*state).group as ::core::ffi::c_int {
                return;
            }
            xkb_current_group = (*state).group as ::core::ffi::c_int;
            ungrab_all_keys(conn);
            grab_all_keys(conn);
        }
        return;
    }
    if shape_supported as ::core::ffi::c_int != 0 && type_0 == shape_base + XCB_SHAPE_NOTIFY {
        let mut shape: *mut xcb_shape_notify_event_t = event as *mut xcb_shape_notify_event_t;
        debuglog(
            b"%s:%s:%d - shape_notify_event for window 0x%08x, shape_kind = %d, shaped = %d\n\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/handlers.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"handle_event\0" as *const u8 as *const ::core::ffi::c_char,
            1465 as ::core::ffi::c_int,
            (*shape).affected_window,
            (*shape).shape_kind as ::core::ffi::c_int,
            (*shape).shaped as ::core::ffi::c_int,
        );
        let mut con: *mut Con = con_by_window_id((*shape).affected_window);
        if con.is_null() {
            verboselog(
                b"Not a managed window 0x%08x, ignoring shape_notify_event\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                (*shape).affected_window,
            );
            return;
        }
        if (*shape).shape_kind as ::core::ffi::c_int == XCB_SHAPE_SK_BOUNDING as ::core::ffi::c_int
            || (*shape).shape_kind as ::core::ffi::c_int == XCB_SHAPE_SK_INPUT as ::core::ffi::c_int
        {
            x_set_shape(
                con,
                (*shape).shape_kind as xcb_shape_sk_t,
                (*shape).shaped != 0,
            );
        }
        return;
    }
    match type_0 {
        XCB_KEY_PRESS | XCB_KEY_RELEASE => {
            handle_key_press(event as *mut xcb_key_press_event_t);
        }
        XCB_BUTTON_PRESS | XCB_BUTTON_RELEASE => {
            handle_button_press(event as *mut xcb_button_press_event_t);
        }
        XCB_MAP_REQUEST => {
            handle_map_request(
                event as *mut xcb_map_request_event_t,
                (*event).full_sequence,
            );
        }
        XCB_UNMAP_NOTIFY => {
            handle_unmap_notify_event(
                event as *mut xcb_unmap_notify_event_t,
                (*event).full_sequence,
            );
        }
        XCB_DESTROY_NOTIFY => {
            handle_destroy_notify_event(
                event as *mut xcb_destroy_notify_event_t,
                (*event).full_sequence,
            );
        }
        XCB_EXPOSE => {
            if (*(event as *mut xcb_expose_event_t)).count as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                handle_expose_event(event as *mut xcb_expose_event_t);
            }
        }
        XCB_MOTION_NOTIFY => {
            handle_motion_notify(event as *mut xcb_motion_notify_event_t);
        }
        XCB_ENTER_NOTIFY => {
            handle_enter_notify(
                event as *mut xcb_enter_notify_event_t,
                (*event).full_sequence,
            );
        }
        XCB_CLIENT_MESSAGE => {
            handle_client_message(event as *mut xcb_client_message_event_t);
        }
        XCB_CONFIGURE_REQUEST => {
            handle_configure_request(event as *mut xcb_configure_request_event_t);
        }
        XCB_MAPPING_NOTIFY => {
            handle_mapping_notify(event as *mut xcb_mapping_notify_event_t);
        }
        XCB_FOCUS_IN => {
            handle_focus_in(event as *mut xcb_focus_in_event_t);
        }
        XCB_FOCUS_OUT => {
            handle_focus_out(event as *mut xcb_focus_in_event_t);
        }
        XCB_PROPERTY_NOTIFY => {
            let mut e: *mut xcb_property_notify_event_t = event as *mut xcb_property_notify_event_t;
            last_timestamp = (*e).time;
            property_notify((*e).state, (*e).window, (*e).atom);
        }
        XCB_CONFIGURE_NOTIFY => {
            handle_configure_notify(event as *mut xcb_configure_notify_event_t);
        }
        XCB_SELECTION_CLEAR => {
            handle_selection_clear(event as *mut xcb_selection_clear_event_t);
        }
        _ => {}
    };
}
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const EXIT_SUCCESS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const UINT_MAX: ::core::ffi::c_uint = (__INT_MAX__ as ::core::ffi::c_uint)
    .wrapping_mul(2 as ::core::ffi::c_uint)
    .wrapping_add(1 as ::core::ffi::c_uint);
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
unsafe extern "C" fn run_static_initializers() {
    property_handlers = [
        property_handler_t {
            atom: 0 as xcb_atom_t,
            long_len: 128 as uint32_t,
            cb: Some(
                handle_windowname_change
                    as unsafe extern "C" fn(*mut Con, *mut xcb_get_property_reply_t) -> bool,
            ),
        },
        property_handler_t {
            atom: 0 as xcb_atom_t,
            long_len: UINT_MAX as uint32_t,
            cb: Some(
                handle_hints
                    as unsafe extern "C" fn(*mut Con, *mut xcb_get_property_reply_t) -> bool,
            ),
        },
        property_handler_t {
            atom: 0 as xcb_atom_t,
            long_len: 128 as uint32_t,
            cb: Some(
                handle_windowname_change_legacy
                    as unsafe extern "C" fn(*mut Con, *mut xcb_get_property_reply_t) -> bool,
            ),
        },
        property_handler_t {
            atom: 0 as xcb_atom_t,
            long_len: UINT_MAX as uint32_t,
            cb: Some(
                handle_normal_hints
                    as unsafe extern "C" fn(*mut Con, *mut xcb_get_property_reply_t) -> bool,
            ),
        },
        property_handler_t {
            atom: 0 as xcb_atom_t,
            long_len: UINT_MAX as uint32_t,
            cb: Some(
                handle_clientleader_change
                    as unsafe extern "C" fn(*mut Con, *mut xcb_get_property_reply_t) -> bool,
            ),
        },
        property_handler_t {
            atom: 0 as xcb_atom_t,
            long_len: UINT_MAX as uint32_t,
            cb: Some(
                handle_transient_for
                    as unsafe extern "C" fn(*mut Con, *mut xcb_get_property_reply_t) -> bool,
            ),
        },
        property_handler_t {
            atom: 0 as xcb_atom_t,
            long_len: 128 as uint32_t,
            cb: Some(
                handle_windowrole_change
                    as unsafe extern "C" fn(*mut Con, *mut xcb_get_property_reply_t) -> bool,
            ),
        },
        property_handler_t {
            atom: 0 as xcb_atom_t,
            long_len: 128 as uint32_t,
            cb: Some(
                handle_class_change
                    as unsafe extern "C" fn(*mut Con, *mut xcb_get_property_reply_t) -> bool,
            ),
        },
        property_handler_t {
            atom: 0 as xcb_atom_t,
            long_len: UINT_MAX as uint32_t,
            cb: Some(
                handle_strut_partial_change
                    as unsafe extern "C" fn(*mut Con, *mut xcb_get_property_reply_t) -> bool,
            ),
        },
        property_handler_t {
            atom: 0 as xcb_atom_t,
            long_len: UINT_MAX as uint32_t,
            cb: Some(
                handle_window_type
                    as unsafe extern "C" fn(*mut Con, *mut xcb_get_property_reply_t) -> bool,
            ),
        },
        property_handler_t {
            atom: 0 as xcb_atom_t,
            long_len: 128 as uint32_t,
            cb: Some(
                handle_machine_change
                    as unsafe extern "C" fn(*mut Con, *mut xcb_get_property_reply_t) -> bool,
            ),
        },
        property_handler_t {
            atom: 0 as xcb_atom_t,
            long_len: (5 as usize).wrapping_mul(::core::mem::size_of::<uint64_t>() as usize)
                as uint32_t,
            cb: Some(
                handle_motif_hints_change
                    as unsafe extern "C" fn(*mut Con, *mut xcb_get_property_reply_t) -> bool,
            ),
        },
        property_handler_t {
            atom: 0 as xcb_atom_t,
            long_len: UINT_MAX as uint32_t,
            cb: Some(
                handle_windowicon_change
                    as unsafe extern "C" fn(*mut Con, *mut xcb_get_property_reply_t) -> bool,
            ),
        },
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
