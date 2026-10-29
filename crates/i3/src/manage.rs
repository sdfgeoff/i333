/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * manage.c: Initially managing new windows (or existing ones on restart).
 *
 */
// Mechanical file-by-file Rust translation of src/manage.c.
// Generated with C2Rust 0.22.1; C ABI and original ownership retained.
extern "C" {
    pub type xcb_connection_t;
    pub type _PangoFontDescription;
    pub type _cairo;
    pub type _cairo_surface;
    pub type _i3String;
    pub type pcre2_real_code_8;
    pub type ev_loop;
    fn time(__timer: *mut time_t) -> time_t;
    fn xcb_change_window_attributes_checked(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        value_mask: uint32_t,
        value_list: *const ::core::ffi::c_void,
    ) -> xcb_void_cookie_t;
    fn xcb_change_window_attributes(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        value_mask: uint32_t,
        value_list: *const ::core::ffi::c_void,
    ) -> xcb_void_cookie_t;
    fn xcb_get_window_attributes(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_get_window_attributes_cookie_t;
    fn xcb_get_window_attributes_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_get_window_attributes_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_get_window_attributes_reply_t;
    fn xcb_destroy_window(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t;
    fn xcb_change_save_set(
        c: *mut xcb_connection_t,
        mode: uint8_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t;
    fn xcb_reparent_window_checked(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        parent: xcb_window_t,
        x: int16_t,
        y: int16_t,
    ) -> xcb_void_cookie_t;
    fn xcb_reparent_window(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        parent: xcb_window_t,
        x: int16_t,
        y: int16_t,
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
    fn xcb_query_tree(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_query_tree_cookie_t;
    fn xcb_query_tree_children(R: *const xcb_query_tree_reply_t) -> *mut xcb_window_t;
    fn xcb_query_tree_children_length(R: *const xcb_query_tree_reply_t) -> ::core::ffi::c_int;
    fn xcb_query_tree_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_query_tree_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_query_tree_reply_t;
    fn xcb_get_property(
        c: *mut xcb_connection_t,
        _delete: uint8_t,
        window: xcb_window_t,
        property: xcb_atom_t,
        type_0: xcb_atom_t,
        long_offset: uint32_t,
        long_length: uint32_t,
    ) -> xcb_get_property_cookie_t;
    fn xcb_get_property_value(R: *const xcb_get_property_reply_t) -> *mut ::core::ffi::c_void;
    fn xcb_get_property_value_length(R: *const xcb_get_property_reply_t) -> ::core::ffi::c_int;
    fn xcb_get_property_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_get_property_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_get_property_reply_t;
    fn xcb_flush(c: *mut xcb_connection_t) -> ::core::ffi::c_int;
    fn xcb_request_check(
        c: *mut xcb_connection_t,
        cookie: xcb_void_cookie_t,
    ) -> *mut xcb_generic_error_t;
    fn xcb_discard_reply(c: *mut xcb_connection_t, sequence: ::core::ffi::c_uint);
    fn xcb_icccm_get_wm_normal_hints(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_get_property_cookie_t;
    fn xcb_icccm_get_wm_hints(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_get_property_cookie_t;
    fn xcb_aux_sync(c: *mut xcb_connection_t);
    static mut conn: *mut xcb_connection_t;
    fn verboselog(fmt: *mut ::core::ffi::c_char, ...);
    fn debuglog(fmt: *mut ::core::ffi::c_char, ...);
    fn smalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn scalloc(num: size_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn sasprintf(
        strp: *mut *mut ::core::ffi::c_char,
        fmt: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn get_visual_depth(visual_id: xcb_visualid_t) -> uint16_t;
    fn ws_name_to_number(name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    static mut croot: *mut Con;
    static mut focused: *mut Con;
    static mut all_cons: all_cons_head;
    fn tree_open_con(con: *mut Con, window: *mut i3Window) -> *mut Con;
    fn tree_render();
    fn xcb_shape_query_extents(
        c: *mut xcb_connection_t,
        destination_window: xcb_window_t,
    ) -> xcb_shape_query_extents_cookie_t;
    fn xcb_shape_query_extents_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_shape_query_extents_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_shape_query_extents_reply_t;
    fn xcb_shape_select_input(
        c: *mut xcb_connection_t,
        destination_window: xcb_window_t,
        enable: uint8_t,
    ) -> xcb_void_cookie_t;
    static mut shape_supported: bool;
    static mut root: xcb_window_t;
    static mut config: Config;
    fn ipc_send_window_event(property: *const ::core::ffi::c_char, con: *mut Con);
    static mut A__NET_WM_NAME: xcb_atom_t;
    static mut A__NET_WM_STATE_FULLSCREEN: xcb_atom_t;
    static mut A__NET_WM_STATE_MODAL: xcb_atom_t;
    static mut A__NET_WM_STATE: xcb_atom_t;
    static mut A__NET_WM_WINDOW_TYPE: xcb_atom_t;
    static mut A__NET_WM_WINDOW_TYPE_DOCK: xcb_atom_t;
    static mut A__NET_WM_WINDOW_TYPE_DIALOG: xcb_atom_t;
    static mut A__NET_WM_WINDOW_TYPE_UTILITY: xcb_atom_t;
    static mut A__NET_WM_WINDOW_TYPE_TOOLBAR: xcb_atom_t;
    static mut A__NET_WM_WINDOW_TYPE_SPLASH: xcb_atom_t;
    static mut A__NET_WM_WINDOW_TYPE_NOTIFICATION: xcb_atom_t;
    static mut A__NET_WM_DESKTOP: xcb_atom_t;
    static mut A__NET_WM_STRUT_PARTIAL: xcb_atom_t;
    static mut A__NET_WM_STATE_STICKY: xcb_atom_t;
    static mut A__NET_WM_USER_TIME: xcb_atom_t;
    static mut A__NET_STARTUP_ID: xcb_atom_t;
    static mut A__NET_WM_ICON: xcb_atom_t;
    static mut A_WM_CLIENT_LEADER: xcb_atom_t;
    static mut A_WM_TAKE_FOCUS: xcb_atom_t;
    static mut A_WM_WINDOW_ROLE: xcb_atom_t;
    static mut A__MOTIF_WM_HINTS: xcb_atom_t;
    fn xcb_set_window_rect(conn_0: *mut xcb_connection_t, window: xcb_window_t, r: Rect);
    fn xcb_get_preferred_window_type(reply: *mut xcb_get_property_reply_t) -> xcb_atom_t;
    fn xcb_reply_contains_atom(prop: *mut xcb_get_property_reply_t, atom: xcb_atom_t) -> bool;
    fn xcb_grab_buttons(
        conn_0: *mut xcb_connection_t,
        window: xcb_window_t,
        buttons: *mut ::core::ffi::c_int,
    );
    fn get_output_containing(x: ::core::ffi::c_uint, y: ::core::ffi::c_uint) -> *mut Output;
    fn get_output_with_dimensions(rect: Rect) -> *mut Output;
    fn get_existing_workspace_by_num(num: ::core::ffi::c_int) -> *mut Con;
    fn workspace_get(num: *const ::core::ffi::c_char) -> *mut Con;
    fn workspace_is_visible(ws: *mut Con) -> bool;
    fn x_reparent_child(con: *mut Con, old: *mut Con);
    fn x_reinit(con: *mut Con);
    fn x_con_reframe(con: *mut Con);
    fn window_supports_protocol(window: xcb_window_t, atom: xcb_atom_t) -> bool;
    fn x_set_name(con: *mut Con, name: *const ::core::ffi::c_char);
    fn floating_enable(con: *mut Con, automatic: bool) -> bool;
    fn con_activate(con: *mut Con);
    fn con_accepts_window(con: *mut Con) -> bool;
    fn con_get_output(con: *mut Con) -> *mut Con;
    fn con_get_workspace(con: *mut Con) -> *mut Con;
    fn con_get_fullscreen_covering_ws(ws: *mut Con) -> *mut Con;
    fn con_is_internal(con: *mut Con) -> bool;
    fn con_by_window_id(window: xcb_window_t) -> *mut Con;
    fn con_find_transient_for_window(start: *mut Con, target: xcb_window_t) -> bool;
    fn con_for_window(
        con: *mut Con,
        window: *mut i3Window,
        store_match: *mut *mut Match,
    ) -> *mut Con;
    fn con_num_windows(con: *mut Con) -> ::core::ffi::c_int;
    fn con_toggle_fullscreen(con: *mut Con, fullscreen_mode: ::core::ffi::c_int);
    fn con_disable_fullscreen(con: *mut Con);
    fn con_move_to_output(con: *mut Con, output: *mut Output, fix_coordinates: bool);
    fn con_move_to_output_name(
        con: *mut Con,
        name: *const ::core::ffi::c_char,
        fix_coordinates: bool,
    ) -> bool;
    fn con_descend_tiling_focused(con: *mut Con) -> *mut Con;
    fn con_set_border_style(
        con: *mut Con,
        border_style: border_style_t,
        border_width: ::core::ffi::c_int,
    );
    fn con_set_urgency(con: *mut Con, urgent: bool);
    fn con_merge_into(old: *mut Con, new: *mut Con);
    fn render_con(con: *mut Con);
    fn window_free(win: *mut i3Window);
    fn window_update_class(win: *mut i3Window, prop: *mut xcb_get_property_reply_t);
    fn window_update_name(win: *mut i3Window, prop: *mut xcb_get_property_reply_t);
    fn window_update_name_legacy(win: *mut i3Window, prop: *mut xcb_get_property_reply_t);
    fn window_update_leader(win: *mut i3Window, prop: *mut xcb_get_property_reply_t);
    fn window_update_transient_for(win: *mut i3Window, prop: *mut xcb_get_property_reply_t);
    fn window_update_strut_partial(win: *mut i3Window, prop: *mut xcb_get_property_reply_t);
    fn window_update_role(win: *mut i3Window, prop: *mut xcb_get_property_reply_t);
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
    fn match_free(match_0: *mut Match);
    fn output_primary_name(output: *mut Output) -> *mut ::core::ffi::c_char;
    fn output_push_sticky_windows(old_focus: *mut Con);
    fn ewmh_update_wm_desktop();
    fn ewmh_get_workspace_by_index(idx: uint32_t) -> *mut Con;
    fn run_assignments(window: *mut i3Window);
    fn assignment_for(window: *mut i3Window, type_0: ::core::ffi::c_int) -> *mut Assignment;
    fn startup_workspace_for_window(
        cwindow: *const i3Window,
        startup_id_reply: *mut xcb_get_property_reply_t,
    ) -> *mut ::core::ffi::c_char;
    fn startup_sequence_delete_by_window(win: *const i3Window);
    fn bindings_get_buttons_to_grab() -> *mut ::core::ffi::c_int;
    fn restore_kill_placeholder(placeholder: xcb_window_t) -> bool;
    fn free(__ptr: *mut ::core::ffi::c_void);
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
pub type xcb_event_mask_t = ::core::ffi::c_uint;
pub const XCB_EVENT_MASK_OWNER_GRAB_BUTTON: xcb_event_mask_t = 16777216;
pub const XCB_EVENT_MASK_COLOR_MAP_CHANGE: xcb_event_mask_t = 8388608;
pub const XCB_EVENT_MASK_PROPERTY_CHANGE: xcb_event_mask_t = 4194304;
pub const XCB_EVENT_MASK_FOCUS_CHANGE: xcb_event_mask_t = 2097152;
pub const XCB_EVENT_MASK_SUBSTRUCTURE_REDIRECT: xcb_event_mask_t = 1048576;
pub const XCB_EVENT_MASK_SUBSTRUCTURE_NOTIFY: xcb_event_mask_t = 524288;
pub const XCB_EVENT_MASK_RESIZE_REDIRECT: xcb_event_mask_t = 262144;
pub const XCB_EVENT_MASK_STRUCTURE_NOTIFY: xcb_event_mask_t = 131072;
pub const XCB_EVENT_MASK_VISIBILITY_CHANGE: xcb_event_mask_t = 65536;
pub const XCB_EVENT_MASK_EXPOSURE: xcb_event_mask_t = 32768;
pub const XCB_EVENT_MASK_KEYMAP_STATE: xcb_event_mask_t = 16384;
pub const XCB_EVENT_MASK_BUTTON_MOTION: xcb_event_mask_t = 8192;
pub const XCB_EVENT_MASK_BUTTON_5_MOTION: xcb_event_mask_t = 4096;
pub const XCB_EVENT_MASK_BUTTON_4_MOTION: xcb_event_mask_t = 2048;
pub const XCB_EVENT_MASK_BUTTON_3_MOTION: xcb_event_mask_t = 1024;
pub const XCB_EVENT_MASK_BUTTON_2_MOTION: xcb_event_mask_t = 512;
pub const XCB_EVENT_MASK_BUTTON_1_MOTION: xcb_event_mask_t = 256;
pub const XCB_EVENT_MASK_POINTER_MOTION_HINT: xcb_event_mask_t = 128;
pub const XCB_EVENT_MASK_POINTER_MOTION: xcb_event_mask_t = 64;
pub const XCB_EVENT_MASK_LEAVE_WINDOW: xcb_event_mask_t = 32;
pub const XCB_EVENT_MASK_ENTER_WINDOW: xcb_event_mask_t = 16;
pub const XCB_EVENT_MASK_BUTTON_RELEASE: xcb_event_mask_t = 8;
pub const XCB_EVENT_MASK_BUTTON_PRESS: xcb_event_mask_t = 4;
pub const XCB_EVENT_MASK_KEY_RELEASE: xcb_event_mask_t = 2;
pub const XCB_EVENT_MASK_KEY_PRESS: xcb_event_mask_t = 1;
pub const XCB_EVENT_MASK_NO_EVENT: xcb_event_mask_t = 0;
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
pub type xcb_map_state_t = ::core::ffi::c_uint;
pub const XCB_MAP_STATE_VIEWABLE: xcb_map_state_t = 2;
pub const XCB_MAP_STATE_UNVIEWABLE: xcb_map_state_t = 1;
pub const XCB_MAP_STATE_UNMAPPED: xcb_map_state_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_get_window_attributes_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_get_window_attributes_reply_t {
    pub response_type: uint8_t,
    pub backing_store: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub visual: xcb_visualid_t,
    pub _class: uint16_t,
    pub bit_gravity: uint8_t,
    pub win_gravity: uint8_t,
    pub backing_planes: uint32_t,
    pub backing_pixel: uint32_t,
    pub save_under: uint8_t,
    pub map_is_installed: uint8_t,
    pub map_state: uint8_t,
    pub override_redirect: uint8_t,
    pub colormap: xcb_colormap_t,
    pub all_event_masks: uint32_t,
    pub your_event_mask: uint32_t,
    pub do_not_propagate_mask: uint16_t,
    pub pad0: [uint8_t; 2],
}
pub type xcb_set_mode_t = ::core::ffi::c_uint;
pub const XCB_SET_MODE_DELETE: xcb_set_mode_t = 1;
pub const XCB_SET_MODE_INSERT: xcb_set_mode_t = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_query_tree_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_query_tree_reply_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub root: xcb_window_t,
    pub parent: xcb_window_t,
    pub children_len: uint16_t,
    pub pad1: [uint8_t; 14],
}
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
pub struct all_cons_head {
    pub tqh_first: *mut Con,
    pub tqh_last: *mut *mut Con,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_shape_query_extents_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_shape_query_extents_reply_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub bounding_shaped: uint8_t,
    pub clip_shaped: uint8_t,
    pub pad1: [uint8_t; 2],
    pub bounding_shape_extents_x: int16_t,
    pub bounding_shape_extents_y: int16_t,
    pub bounding_shape_extents_width: uint16_t,
    pub bounding_shape_extents_height: uint16_t,
    pub clip_shape_extents_x: int16_t,
    pub clip_shape_extents_y: int16_t,
    pub clip_shape_extents_width: uint16_t,
    pub clip_shape_extents_height: uint16_t,
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
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const XCB_NONE: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NET_WM_DESKTOP_NONE: ::core::ffi::c_uint = 0xfffffff0 as ::core::ffi::c_uint;
pub const NET_WM_DESKTOP_ALL: ::core::ffi::c_uint = 0xffffffff as ::core::ffi::c_uint;
unsafe extern "C" fn _match_depth(mut win: *mut i3Window, mut con: *mut Con) -> xcb_window_t {
    let mut old_frame: xcb_window_t = XCB_NONE as xcb_window_t;
    if (*con).depth as ::core::ffi::c_int != (*win).depth as ::core::ffi::c_int {
        old_frame = (*con).frame.id as xcb_window_t;
        (*con).depth = (*win).depth;
        x_con_reframe(con);
    }
    return old_frame;
}
unsafe extern "C" fn _remove_matches(mut con: *mut Con) {
    while !(*con).swallow_head.tqh_first.is_null() {
        let mut first: *mut Match = (*con).swallow_head.tqh_first as *mut Match;
        if !(*first).matches.tqe_next.is_null() {
            (*(*first).matches.tqe_next).matches.tqe_prev = (*first).matches.tqe_prev;
        } else {
            (*con).swallow_head.tqh_last = (*first).matches.tqe_prev;
        }
        *(*first).matches.tqe_prev = (*first).matches.tqe_next;
        match_free(first);
        free(first as *mut ::core::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn manage_existing_windows(mut root_0: xcb_window_t) {
    let mut reply: *mut xcb_query_tree_reply_t = ::core::ptr::null_mut::<xcb_query_tree_reply_t>();
    reply = xcb_query_tree_reply(
        conn,
        xcb_query_tree(conn, root_0),
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply.is_null() {
        return;
    }
    let mut len: ::core::ffi::c_int = xcb_query_tree_children_length(reply);
    let mut cookies: *mut xcb_get_window_attributes_cookie_t = smalloc(
        (len as size_t)
            .wrapping_mul(::core::mem::size_of::<xcb_get_window_attributes_cookie_t>() as size_t),
    )
        as *mut xcb_get_window_attributes_cookie_t;
    let mut children: *mut xcb_window_t = xcb_query_tree_children(reply);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < len {
        *cookies.offset(i as isize) = xcb_get_window_attributes(conn, *children.offset(i as isize));
        i += 1;
    }
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < len {
        manage_window(
            *children.offset(i_0 as isize),
            *cookies.offset(i_0 as isize),
            true_0 != 0,
        );
        i_0 += 1;
    }
    free(reply as *mut ::core::ffi::c_void);
    free(cookies as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn restore_geometry() {
    debuglog(
        b"%s:%s:%d - Restoring geometry\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/manage.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"restore_geometry\0" as *const u8 as *const ::core::ffi::c_char,
        79 as ::core::ffi::c_int,
    );
    let mut con: *mut Con = ::core::ptr::null_mut::<Con>();
    con = all_cons.tqh_first as *mut Con;
    while !con.is_null() {
        if !(*con).window.is_null() {
            debuglog(
                b"%s:%s:%d - Re-adding X11 border of %d px\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"../src/manage.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"restore_geometry\0" as *const u8 as *const ::core::ffi::c_char,
                84 as ::core::ffi::c_int,
                (*con).border_width,
            );
            (*con).window_rect.width = (*con)
                .window_rect
                .width
                .wrapping_add((2 as ::core::ffi::c_int * (*con).border_width) as uint32_t);
            (*con).window_rect.height = (*con)
                .window_rect
                .height
                .wrapping_add((2 as ::core::ffi::c_int * (*con).border_width) as uint32_t);
            xcb_set_window_rect(conn, (*(*con).window).id, (*con).window_rect as Rect);
            debuglog(
                b"%s:%s:%d - placing window %08x at %d %d\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"../src/manage.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"restore_geometry\0" as *const u8 as *const ::core::ffi::c_char,
                88 as ::core::ffi::c_int,
                (*(*con).window).id,
                (*con).rect.x,
                (*con).rect.y,
            );
            xcb_reparent_window(
                conn,
                (*(*con).window).id,
                root,
                (*con).rect.x as int16_t,
                (*con).rect.y as int16_t,
            );
        }
        con = (*con).all_cons.tqe_next as *mut Con;
    }
    let mut fresh0: [uint32_t; 1] =
        [XCB_EVENT_MASK_SUBSTRUCTURE_REDIRECT as ::core::ffi::c_int as uint32_t];
    xcb_change_window_attributes(
        conn,
        root,
        XCB_CW_EVENT_MASK as ::core::ffi::c_int as uint32_t,
        &raw mut fresh0 as *mut uint32_t as *const ::core::ffi::c_void,
    );
    xcb_aux_sync(conn);
}
#[no_mangle]
pub unsafe extern "C" fn manage_window(
    mut window: xcb_window_t,
    mut cookie: xcb_get_window_attributes_cookie_t,
    mut needs_to_be_mapped: bool,
) {
    let mut values: [uint32_t; 1] = [0; 1];
    let mut event_mask_cookie: xcb_void_cookie_t = xcb_void_cookie_t { sequence: 0 };
    let mut cwindow: *mut i3Window = ::core::ptr::null_mut::<i3Window>();
    let mut buttons: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut urgency_hint: bool = false;
    let mut motif_border_style: border_style_t = BS_NONE;
    let mut has_mwm_hints: bool = false;
    let mut type_reply: *mut xcb_get_property_reply_t =
        ::core::ptr::null_mut::<xcb_get_property_reply_t>();
    let mut state_reply: *mut xcb_get_property_reply_t =
        ::core::ptr::null_mut::<xcb_get_property_reply_t>();
    let mut startup_id_reply: *mut xcb_get_property_reply_t =
        ::core::ptr::null_mut::<xcb_get_property_reply_t>();
    let mut startup_ws: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut wm_desktop_reply: *mut xcb_get_property_reply_t =
        ::core::ptr::null_mut::<xcb_get_property_reply_t>();
    let mut search_at: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut match_0: *mut Match = ::core::ptr::null_mut::<Match>();
    let mut nc: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut match_from_restart_mode: bool = false;
    let mut old_frame: xcb_window_t = 0;
    let mut name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut ws: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut fs: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut set_focus: bool = false;
    let mut want_floating: bool = false;
    let mut rcookie: xcb_void_cookie_t = xcb_void_cookie_t { sequence: 0 };
    debuglog(
        b"%s:%s:%d - window 0x%08x\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/manage.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"manage_window\0" as *const u8 as *const ::core::ffi::c_char,
        108 as ::core::ffi::c_int,
        window,
    );
    let mut d: xcb_drawable_t = window;
    let mut geomc: xcb_get_geometry_cookie_t = xcb_get_geometry_cookie_t { sequence: 0 };
    let mut geom: *mut xcb_get_geometry_reply_t =
        ::core::ptr::null_mut::<xcb_get_geometry_reply_t>();
    let mut attr: *mut xcb_get_window_attributes_reply_t =
        ::core::ptr::null_mut::<xcb_get_window_attributes_reply_t>();
    let mut wm_type_cookie: xcb_get_property_cookie_t = xcb_get_property_cookie_t { sequence: 0 };
    let mut strut_cookie: xcb_get_property_cookie_t = xcb_get_property_cookie_t { sequence: 0 };
    let mut state_cookie: xcb_get_property_cookie_t = xcb_get_property_cookie_t { sequence: 0 };
    let mut utf8_title_cookie: xcb_get_property_cookie_t =
        xcb_get_property_cookie_t { sequence: 0 };
    let mut title_cookie: xcb_get_property_cookie_t = xcb_get_property_cookie_t { sequence: 0 };
    let mut class_cookie: xcb_get_property_cookie_t = xcb_get_property_cookie_t { sequence: 0 };
    let mut leader_cookie: xcb_get_property_cookie_t = xcb_get_property_cookie_t { sequence: 0 };
    let mut transient_cookie: xcb_get_property_cookie_t = xcb_get_property_cookie_t { sequence: 0 };
    let mut role_cookie: xcb_get_property_cookie_t = xcb_get_property_cookie_t { sequence: 0 };
    let mut startup_id_cookie: xcb_get_property_cookie_t =
        xcb_get_property_cookie_t { sequence: 0 };
    let mut wm_hints_cookie: xcb_get_property_cookie_t = xcb_get_property_cookie_t { sequence: 0 };
    let mut wm_normal_hints_cookie: xcb_get_property_cookie_t =
        xcb_get_property_cookie_t { sequence: 0 };
    let mut motif_wm_hints_cookie: xcb_get_property_cookie_t =
        xcb_get_property_cookie_t { sequence: 0 };
    let mut wm_user_time_cookie: xcb_get_property_cookie_t =
        xcb_get_property_cookie_t { sequence: 0 };
    let mut wm_desktop_cookie: xcb_get_property_cookie_t =
        xcb_get_property_cookie_t { sequence: 0 };
    let mut wm_machine_cookie: xcb_get_property_cookie_t =
        xcb_get_property_cookie_t { sequence: 0 };
    let mut wm_icon_cookie: xcb_get_property_cookie_t = xcb_get_property_cookie_t { sequence: 0 };
    geomc = xcb_get_geometry(conn, d);
    attr = xcb_get_window_attributes_reply(
        conn,
        cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if attr.is_null() {
        debuglog(
            b"%s:%s:%d - Could not get attributes\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b"../src/manage.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"manage_window\0" as *const u8 as *const ::core::ffi::c_char,
            129 as ::core::ffi::c_int,
        );
        xcb_discard_reply(conn, geomc.sequence);
        return;
    }
    if needs_to_be_mapped as ::core::ffi::c_int != 0
        && (*attr).map_state as ::core::ffi::c_int != XCB_MAP_STATE_VIEWABLE as ::core::ffi::c_int
    {
        xcb_discard_reply(conn, geomc.sequence);
    } else if (*attr).override_redirect != 0 {
        xcb_discard_reply(conn, geomc.sequence);
    } else if !con_by_window_id(window).is_null() {
        debuglog(
            b"%s:%s:%d - already managed (by con %p)\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b"../src/manage.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"manage_window\0" as *const u8 as *const ::core::ffi::c_char,
            147 as ::core::ffi::c_int,
            con_by_window_id(window),
        );
        xcb_discard_reply(conn, geomc.sequence);
    } else {
        geom = xcb_get_geometry_reply(
            conn,
            geomc,
            ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
        );
        if geom.is_null() {
            debuglog(
                b"%s:%s:%d - could not get geometry\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"../src/manage.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"manage_window\0" as *const u8 as *const ::core::ffi::c_char,
                154 as ::core::ffi::c_int,
            );
        } else {
            values = [0; 1];
            values[0 as ::core::ffi::c_int as usize] = (XCB_EVENT_MASK_PROPERTY_CHANGE
                as ::core::ffi::c_int
                | XCB_EVENT_MASK_STRUCTURE_NOTIFY as ::core::ffi::c_int)
                as uint32_t;
            event_mask_cookie = xcb_change_window_attributes_checked(
                conn,
                window,
                XCB_CW_EVENT_MASK as ::core::ffi::c_int as uint32_t,
                &raw mut values as *mut uint32_t as *const ::core::ffi::c_void,
            );
            if !xcb_request_check(conn, event_mask_cookie).is_null() {
                verboselog(
                    b"Could not change event mask, the window probably already disappeared.\n\0"
                        as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            } else {
                wm_type_cookie = xcb_get_property(
                    conn,
                    false_0 as uint8_t,
                    window,
                    A__NET_WM_WINDOW_TYPE,
                    XCB_GET_PROPERTY_TYPE_ANY as ::core::ffi::c_int as xcb_atom_t,
                    0 as uint32_t,
                    4294967295 as uint32_t,
                );
                strut_cookie = xcb_get_property(
                    conn,
                    false_0 as uint8_t,
                    window,
                    A__NET_WM_STRUT_PARTIAL,
                    XCB_GET_PROPERTY_TYPE_ANY as ::core::ffi::c_int as xcb_atom_t,
                    0 as uint32_t,
                    4294967295 as uint32_t,
                );
                state_cookie = xcb_get_property(
                    conn,
                    false_0 as uint8_t,
                    window,
                    A__NET_WM_STATE,
                    XCB_GET_PROPERTY_TYPE_ANY as ::core::ffi::c_int as xcb_atom_t,
                    0 as uint32_t,
                    4294967295 as uint32_t,
                );
                utf8_title_cookie = xcb_get_property(
                    conn,
                    false_0 as uint8_t,
                    window,
                    A__NET_WM_NAME,
                    XCB_GET_PROPERTY_TYPE_ANY as ::core::ffi::c_int as xcb_atom_t,
                    0 as uint32_t,
                    128 as uint32_t,
                );
                leader_cookie = xcb_get_property(
                    conn,
                    false_0 as uint8_t,
                    window,
                    A_WM_CLIENT_LEADER,
                    XCB_GET_PROPERTY_TYPE_ANY as ::core::ffi::c_int as xcb_atom_t,
                    0 as uint32_t,
                    4294967295 as uint32_t,
                );
                transient_cookie = xcb_get_property(
                    conn,
                    false_0 as uint8_t,
                    window,
                    XCB_ATOM_WM_TRANSIENT_FOR as ::core::ffi::c_int as xcb_atom_t,
                    XCB_GET_PROPERTY_TYPE_ANY as ::core::ffi::c_int as xcb_atom_t,
                    0 as uint32_t,
                    4294967295 as uint32_t,
                );
                title_cookie = xcb_get_property(
                    conn,
                    false_0 as uint8_t,
                    window,
                    XCB_ATOM_WM_NAME as ::core::ffi::c_int as xcb_atom_t,
                    XCB_GET_PROPERTY_TYPE_ANY as ::core::ffi::c_int as xcb_atom_t,
                    0 as uint32_t,
                    128 as uint32_t,
                );
                class_cookie = xcb_get_property(
                    conn,
                    false_0 as uint8_t,
                    window,
                    XCB_ATOM_WM_CLASS as ::core::ffi::c_int as xcb_atom_t,
                    XCB_GET_PROPERTY_TYPE_ANY as ::core::ffi::c_int as xcb_atom_t,
                    0 as uint32_t,
                    128 as uint32_t,
                );
                role_cookie = xcb_get_property(
                    conn,
                    false_0 as uint8_t,
                    window,
                    A_WM_WINDOW_ROLE,
                    XCB_GET_PROPERTY_TYPE_ANY as ::core::ffi::c_int as xcb_atom_t,
                    0 as uint32_t,
                    128 as uint32_t,
                );
                startup_id_cookie = xcb_get_property(
                    conn,
                    false_0 as uint8_t,
                    window,
                    A__NET_STARTUP_ID,
                    XCB_GET_PROPERTY_TYPE_ANY as ::core::ffi::c_int as xcb_atom_t,
                    0 as uint32_t,
                    512 as uint32_t,
                );
                wm_hints_cookie = xcb_icccm_get_wm_hints(conn, window);
                wm_normal_hints_cookie = xcb_icccm_get_wm_normal_hints(conn, window);
                motif_wm_hints_cookie = xcb_get_property(
                    conn,
                    false_0 as uint8_t,
                    window,
                    A__MOTIF_WM_HINTS,
                    XCB_GET_PROPERTY_TYPE_ANY as ::core::ffi::c_int as xcb_atom_t,
                    0 as uint32_t,
                    (5 as usize).wrapping_mul(::core::mem::size_of::<uint64_t>() as usize)
                        as uint32_t,
                );
                wm_user_time_cookie = xcb_get_property(
                    conn,
                    false_0 as uint8_t,
                    window,
                    A__NET_WM_USER_TIME,
                    XCB_GET_PROPERTY_TYPE_ANY as ::core::ffi::c_int as xcb_atom_t,
                    0 as uint32_t,
                    4294967295 as uint32_t,
                );
                wm_desktop_cookie = xcb_get_property(
                    conn,
                    false_0 as uint8_t,
                    window,
                    A__NET_WM_DESKTOP,
                    XCB_GET_PROPERTY_TYPE_ANY as ::core::ffi::c_int as xcb_atom_t,
                    0 as uint32_t,
                    4294967295 as uint32_t,
                );
                wm_machine_cookie = xcb_get_property(
                    conn,
                    false_0 as uint8_t,
                    window,
                    XCB_ATOM_WM_CLIENT_MACHINE as ::core::ffi::c_int as xcb_atom_t,
                    XCB_GET_PROPERTY_TYPE_ANY as ::core::ffi::c_int as xcb_atom_t,
                    0 as uint32_t,
                    4294967295 as uint32_t,
                );
                wm_icon_cookie = xcb_get_property(
                    conn,
                    false_0 as uint8_t,
                    window,
                    A__NET_WM_ICON,
                    XCB_GET_PROPERTY_TYPE_ANY as ::core::ffi::c_int as xcb_atom_t,
                    0 as uint32_t,
                    4294967295 as uint32_t,
                );
                cwindow = scalloc(1 as size_t, ::core::mem::size_of::<i3Window>() as size_t)
                    as *mut i3Window;
                (*cwindow).id = window;
                (*cwindow).depth = get_visual_depth((*attr).visual);
                buttons = bindings_get_buttons_to_grab();
                xcb_grab_buttons(conn, window, buttons);
                free(buttons as *mut ::core::ffi::c_void);
                buttons = ::core::ptr::null_mut::<::core::ffi::c_int>();
                window_update_class(
                    cwindow,
                    xcb_get_property_reply(
                        conn,
                        class_cookie,
                        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
                    ),
                );
                window_update_name_legacy(
                    cwindow,
                    xcb_get_property_reply(
                        conn,
                        title_cookie,
                        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
                    ),
                );
                window_update_name(
                    cwindow,
                    xcb_get_property_reply(
                        conn,
                        utf8_title_cookie,
                        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
                    ),
                );
                window_update_icon(
                    cwindow,
                    xcb_get_property_reply(
                        conn,
                        wm_icon_cookie,
                        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
                    ),
                );
                window_update_leader(
                    cwindow,
                    xcb_get_property_reply(
                        conn,
                        leader_cookie,
                        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
                    ),
                );
                window_update_transient_for(
                    cwindow,
                    xcb_get_property_reply(
                        conn,
                        transient_cookie,
                        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
                    ),
                );
                window_update_strut_partial(
                    cwindow,
                    xcb_get_property_reply(
                        conn,
                        strut_cookie,
                        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
                    ),
                );
                window_update_role(
                    cwindow,
                    xcb_get_property_reply(
                        conn,
                        role_cookie,
                        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
                    ),
                );
                urgency_hint = false;
                window_update_hints(
                    cwindow,
                    xcb_get_property_reply(
                        conn,
                        wm_hints_cookie,
                        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
                    ),
                    &raw mut urgency_hint,
                );
                motif_border_style = BS_NONE;
                has_mwm_hints = window_update_motif_hints(
                    cwindow,
                    xcb_get_property_reply(
                        conn,
                        motif_wm_hints_cookie,
                        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
                    ),
                    &raw mut motif_border_style,
                );
                window_update_normal_hints(
                    cwindow,
                    xcb_get_property_reply(
                        conn,
                        wm_normal_hints_cookie,
                        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
                    ),
                    geom,
                );
                window_update_machine(
                    cwindow,
                    xcb_get_property_reply(
                        conn,
                        wm_machine_cookie,
                        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
                    ),
                );
                type_reply = xcb_get_property_reply(
                    conn,
                    wm_type_cookie,
                    ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
                );
                state_reply = xcb_get_property_reply(
                    conn,
                    state_cookie,
                    ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
                );
                startup_id_reply = ::core::ptr::null_mut::<xcb_get_property_reply_t>();
                startup_id_reply = xcb_get_property_reply(
                    conn,
                    startup_id_cookie,
                    ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
                );
                startup_ws = startup_workspace_for_window(cwindow, startup_id_reply);
                debuglog(
                    b"%s:%s:%d - startup workspace = %s\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    b"../src/manage.c\0" as *const u8 as *const ::core::ffi::c_char,
                    b"manage_window\0" as *const u8 as *const ::core::ffi::c_char,
                    227 as ::core::ffi::c_int,
                    startup_ws,
                );
                wm_desktop_reply = ::core::ptr::null_mut::<xcb_get_property_reply_t>();
                wm_desktop_reply = xcb_get_property_reply(
                    conn,
                    wm_desktop_cookie,
                    ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
                );
                (*cwindow).wm_desktop = NET_WM_DESKTOP_NONE as uint32_t;
                if !wm_desktop_reply.is_null()
                    && xcb_get_property_value_length(wm_desktop_reply) != 0 as ::core::ffi::c_int
                {
                    let mut wm_desktops: *mut uint32_t =
                        xcb_get_property_value(wm_desktop_reply) as *mut uint32_t;
                    (*cwindow).wm_desktop = *wm_desktops.offset(0 as ::core::ffi::c_int as isize)
                        as int32_t as uint32_t;
                }
                free(wm_desktop_reply as *mut ::core::ffi::c_void);
                wm_desktop_reply = ::core::ptr::null_mut::<xcb_get_property_reply_t>();
                (*cwindow).needs_take_focus =
                    window_supports_protocol((*cwindow).id, A_WM_TAKE_FOCUS);
                (*cwindow).window_type = xcb_get_preferred_window_type(type_reply);
                search_at = croot;
                if xcb_reply_contains_atom(type_reply, A__NET_WM_WINDOW_TYPE_DOCK) {
                    verboselog(
                        b"This window is of type dock\n\0" as *const u8
                            as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    );
                    let mut output: *mut Output = get_output_containing(
                        (*geom).x as ::core::ffi::c_uint,
                        (*geom).y as ::core::ffi::c_uint,
                    );
                    if !output.is_null() {
                        debuglog(
                            b"%s:%s:%d - Starting search at output %s\n\0" as *const u8
                                as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b"../src/manage.c\0" as *const u8 as *const ::core::ffi::c_char,
                            b"manage_window\0" as *const u8 as *const ::core::ffi::c_char,
                            252 as ::core::ffi::c_int,
                            output_primary_name(output),
                        );
                        search_at = (*output).con;
                    }
                    if (*cwindow).reserved.top > 0 as uint32_t
                        && (*cwindow).reserved.bottom == 0 as uint32_t
                    {
                        debuglog(
                            b"%s:%s:%d - Top dock client\n\0" as *const u8
                                as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b"../src/manage.c\0" as *const u8 as *const ::core::ffi::c_char,
                            b"manage_window\0" as *const u8 as *const ::core::ffi::c_char,
                            258 as ::core::ffi::c_int,
                        );
                        (*cwindow).dock = W_DOCK_TOP;
                    } else if (*cwindow).reserved.top == 0 as uint32_t
                        && (*cwindow).reserved.bottom > 0 as uint32_t
                    {
                        debuglog(
                            b"%s:%s:%d - Bottom dock client\n\0" as *const u8
                                as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b"../src/manage.c\0" as *const u8 as *const ::core::ffi::c_char,
                            b"manage_window\0" as *const u8 as *const ::core::ffi::c_char,
                            261 as ::core::ffi::c_int,
                        );
                        (*cwindow).dock = W_DOCK_BOTTOM;
                    } else {
                        debuglog(
                            b"%s:%s:%d - Ignoring invalid reserved edges (_NET_WM_STRUT_PARTIAL), using position as fallback:\n\0"
                                as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b"../src/manage.c\0" as *const u8
                                as *const ::core::ffi::c_char,
                            b"manage_window\0" as *const u8
                                as *const ::core::ffi::c_char,
                            264 as ::core::ffi::c_int,
                        );
                        if ((*geom).y as ::core::ffi::c_int)
                            < (*search_at).rect.height.wrapping_div(2 as uint32_t) as int16_t
                                as ::core::ffi::c_int
                        {
                            debuglog(
                                b"%s:%s:%d - geom->y = %d < rect.height / 2 = %d, it is a top dock client\n\0"
                                    as *const u8 as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                                b"../src/manage.c\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                b"manage_window\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                267 as ::core::ffi::c_int,
                                (*geom).y as ::core::ffi::c_int,
                                (*search_at).rect.height.wrapping_div(2 as uint32_t),
                            );
                            (*cwindow).dock = W_DOCK_TOP;
                        } else {
                            debuglog(
                                b"%s:%s:%d - geom->y = %d >= rect.height / 2 = %d, it is a bottom dock client\n\0"
                                    as *const u8 as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                                b"../src/manage.c\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                b"manage_window\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                271 as ::core::ffi::c_int,
                                (*geom).y as ::core::ffi::c_int,
                                (*search_at).rect.height.wrapping_div(2 as uint32_t),
                            );
                            (*cwindow).dock = W_DOCK_BOTTOM;
                        }
                    }
                }
                debuglog(
                    b"%s:%s:%d - Initial geometry: (%d, %d, %d, %d)\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    b"../src/manage.c\0" as *const u8 as *const ::core::ffi::c_char,
                    b"manage_window\0" as *const u8 as *const ::core::ffi::c_char,
                    277 as ::core::ffi::c_int,
                    (*geom).x as ::core::ffi::c_int,
                    (*geom).y as ::core::ffi::c_int,
                    (*geom).width as ::core::ffi::c_int,
                    (*geom).height as ::core::ffi::c_int,
                );
                (*cwindow).swallowed = false_0 != 0;
                match_0 = ::core::ptr::null_mut::<Match>();
                nc = con_for_window(search_at, cwindow, &raw mut match_0);
                match_from_restart_mode =
                    !match_0.is_null() && (*match_0).restart_mode as ::core::ffi::c_int != 0;
                if nc.is_null() {
                    let mut wm_desktop_ws: *mut Con = ::core::ptr::null_mut::<Con>();
                    let mut assignment: *mut Assignment = ::core::ptr::null_mut::<Assignment>();
                    assignment = assignment_for(cwindow, A_TO_WORKSPACE as ::core::ffi::c_int);
                    if !assignment.is_null() || {
                        assignment =
                            assignment_for(cwindow, A_TO_WORKSPACE_NUMBER as ::core::ffi::c_int);
                        !assignment.is_null()
                    } {
                        debuglog(
                            b"%s:%s:%d - Assignment matches (%p)\n\0" as *const u8
                                as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b"../src/manage.c\0" as *const u8 as *const ::core::ffi::c_char,
                            b"manage_window\0" as *const u8 as *const ::core::ffi::c_char,
                            291 as ::core::ffi::c_int,
                            match_0,
                        );
                        let mut assigned_ws: *mut Con = ::core::ptr::null_mut::<Con>();
                        if (*assignment).type_0 as ::core::ffi::c_uint
                            == A_TO_WORKSPACE_NUMBER as ::core::ffi::c_int as ::core::ffi::c_uint
                        {
                            let mut parsed_num: ::core::ffi::c_long =
                                ws_name_to_number((*assignment).dest.workspace)
                                    as ::core::ffi::c_long;
                            assigned_ws =
                                get_existing_workspace_by_num(parsed_num as ::core::ffi::c_int);
                        }
                        if assigned_ws.is_null() {
                            assigned_ws = workspace_get((*assignment).dest.workspace);
                        }
                        nc = con_descend_tiling_focused(assigned_ws);
                        debuglog(
                            b"%s:%s:%d - focused on ws %s: %p / %s\n\0" as *const u8
                                as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b"../src/manage.c\0" as *const u8 as *const ::core::ffi::c_char,
                            b"manage_window\0" as *const u8 as *const ::core::ffi::c_char,
                            306 as ::core::ffi::c_int,
                            (*assigned_ws).name,
                            nc,
                            (*nc).name,
                        );
                        if (*nc).type_0 as ::core::ffi::c_uint
                            == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
                        {
                            nc = tree_open_con(nc, cwindow);
                        } else {
                            nc = tree_open_con((*nc).parent as *mut Con, cwindow);
                        }
                        if !workspace_is_visible(assigned_ws) {
                            urgency_hint = true_0 != 0;
                        }
                    } else if (*cwindow).wm_desktop != NET_WM_DESKTOP_NONE as uint32_t
                        && (*cwindow).wm_desktop != NET_WM_DESKTOP_ALL as uint32_t
                        && {
                            wm_desktop_ws = ewmh_get_workspace_by_index((*cwindow).wm_desktop);
                            !wm_desktop_ws.is_null()
                        }
                    {
                        debuglog(
                            b"%s:%s:%d - Using workspace %p / %s because _NET_WM_DESKTOP = %d.\n\0"
                                as *const u8
                                as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b"../src/manage.c\0" as *const u8 as *const ::core::ffi::c_char,
                            b"manage_window\0" as *const u8 as *const ::core::ffi::c_char,
                            325 as ::core::ffi::c_int,
                            wm_desktop_ws,
                            (*wm_desktop_ws).name,
                            (*cwindow).wm_desktop,
                        );
                        nc = con_descend_tiling_focused(wm_desktop_ws);
                        if (*nc).type_0 as ::core::ffi::c_uint
                            == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
                        {
                            nc = tree_open_con(nc, cwindow);
                        } else {
                            nc = tree_open_con((*nc).parent as *mut Con, cwindow);
                        }
                    } else if !startup_ws.is_null() {
                        debuglog(
                            b"%s:%s:%d - Using workspace on which this application was started (%s)\n\0"
                                as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b"../src/manage.c\0" as *const u8
                                as *const ::core::ffi::c_char,
                            b"manage_window\0" as *const u8
                                as *const ::core::ffi::c_char,
                            335 as ::core::ffi::c_int,
                            startup_ws,
                        );
                        nc = con_descend_tiling_focused(workspace_get(startup_ws));
                        debuglog(
                            b"%s:%s:%d - focused on ws %s: %p / %s\n\0" as *const u8
                                as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b"../src/manage.c\0" as *const u8 as *const ::core::ffi::c_char,
                            b"manage_window\0" as *const u8 as *const ::core::ffi::c_char,
                            337 as ::core::ffi::c_int,
                            startup_ws,
                            nc,
                            (*nc).name,
                        );
                        if (*nc).type_0 as ::core::ffi::c_uint
                            == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
                        {
                            nc = tree_open_con(nc, cwindow);
                        } else {
                            nc = tree_open_con((*nc).parent as *mut Con, cwindow);
                        }
                    } else if (*focused).type_0 as ::core::ffi::c_uint
                        == CT_CON as ::core::ffi::c_int as ::core::ffi::c_uint
                        && con_accepts_window(focused) as ::core::ffi::c_int != 0
                    {
                        verboselog(
                            b"using current container, focused = %p, focused->name = %s\n\0"
                                as *const u8
                                as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            focused,
                            (*focused).name,
                        );
                        nc = focused;
                    } else {
                        nc = tree_open_con(::core::ptr::null_mut::<Con>(), cwindow);
                    }
                    assignment = assignment_for(cwindow, A_TO_OUTPUT as ::core::ffi::c_int);
                    if !assignment.is_null() {
                        con_move_to_output_name(nc, (*assignment).dest.output, true_0 != 0);
                    }
                } else {
                    if !match_0.is_null()
                        && (*match_0).insert_where as ::core::ffi::c_uint
                            == M_BELOW as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        nc = tree_open_con(nc, cwindow);
                    }
                    if !match_0.is_null()
                        && (*match_0).insert_where as ::core::ffi::c_uint
                            != M_BELOW as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        debuglog(
                            b"%s:%s:%d - Removing match %p from container %p\n\0" as *const u8
                                as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b"../src/manage.c\0" as *const u8 as *const ::core::ffi::c_char,
                            b"manage_window\0" as *const u8 as *const ::core::ffi::c_char,
                            369 as ::core::ffi::c_int,
                            match_0,
                            nc,
                        );
                        if !(*match_0).matches.tqe_next.is_null() {
                            (*(*match_0).matches.tqe_next).matches.tqe_prev =
                                (*match_0).matches.tqe_prev;
                        } else {
                            (*nc).swallow_head.tqh_last = (*match_0).matches.tqe_prev;
                        }
                        *(*match_0).matches.tqe_prev = (*match_0).matches.tqe_next;
                        match_free(match_0);
                        free(match_0 as *mut ::core::ffi::c_void);
                        match_0 = ::core::ptr::null_mut::<Match>();
                    }
                    (*cwindow).swallowed = true_0 != 0;
                }
                debuglog(
                    b"%s:%s:%d - new container = %p\n\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    b"../src/manage.c\0" as *const u8 as *const ::core::ffi::c_char,
                    b"manage_window\0" as *const u8 as *const ::core::ffi::c_char,
                    378 as ::core::ffi::c_int,
                    nc,
                );
                if !(*nc).window.is_null() && (*nc).window != cwindow {
                    if !restore_kill_placeholder((*(*nc).window).id) {
                        debuglog(
                            b"%s:%s:%d - Uh?! Container without a placeholder, but with a window, has swallowed this to-be-managed window?!\n\0"
                                as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b"../src/manage.c\0" as *const u8
                                as *const ::core::ffi::c_char,
                            b"manage_window\0" as *const u8
                                as *const ::core::ffi::c_char,
                            381 as ::core::ffi::c_int,
                        );
                    } else {
                        _remove_matches(nc);
                    }
                }
                old_frame = XCB_NONE as xcb_window_t;
                if (*nc).window != cwindow && !(*nc).window.is_null() {
                    window_free((*nc).window as *mut i3Window);
                    old_frame = _match_depth(cwindow, nc);
                }
                (*nc).window = cwindow as *mut Window;
                x_reinit(nc);
                (*nc).border_width = (*geom).border_width as ::core::ffi::c_int;
                name = ::core::ptr::null_mut::<::core::ffi::c_char>();
                sasprintf(
                    &raw mut name,
                    b"[i3 con] container around %p\0" as *const u8 as *const ::core::ffi::c_char,
                    cwindow,
                );
                x_set_name(nc, name);
                free(name as *mut ::core::ffi::c_void);
                ws = con_get_workspace(nc);
                fs = con_get_fullscreen_covering_ws(ws);
                if xcb_reply_contains_atom(state_reply, A__NET_WM_STATE_FULLSCREEN) {
                    if fs != nc {
                        let mut output_0: *mut Output = get_output_with_dimensions(Rect {
                            x: (*geom).x as uint32_t,
                            y: (*geom).y as uint32_t,
                            width: (*geom).width as uint32_t,
                            height: (*geom).height as uint32_t,
                        });
                        if !output_0.is_null() {
                            con_move_to_output(nc, output_0, false_0 != 0);
                        }
                        con_toggle_fullscreen(nc, CF_OUTPUT as ::core::ffi::c_int);
                    }
                    fs = ::core::ptr::null_mut::<Con>();
                }
                set_focus = false_0 != 0;
                if fs.is_null() {
                    debuglog(
                        b"%s:%s:%d - Not in fullscreen mode, focusing\n\0" as *const u8
                            as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        b"../src/manage.c\0" as *const u8 as *const ::core::ffi::c_char,
                        b"manage_window\0" as *const u8 as *const ::core::ffi::c_char,
                        426 as ::core::ffi::c_int,
                    );
                    if (*cwindow).dock as u64 == 0 {
                        let mut current_output: *mut Con = con_get_output(focused);
                        let mut target_output: *mut Con = con_get_output(ws);
                        if workspace_is_visible(ws) as ::core::ffi::c_int != 0
                            && current_output == target_output
                        {
                            if !match_from_restart_mode {
                                set_focus = true_0 != 0;
                            } else {
                                debuglog(
                                    b"%s:%s:%d - not focusing, matched with restart_mode == true\n\0"
                                        as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    b"../src/manage.c\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    b"manage_window\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    438 as ::core::ffi::c_int,
                                );
                            }
                        } else {
                            debuglog(
                                b"%s:%s:%d - workspace not visible, not focusing\n\0" as *const u8
                                    as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                                b"../src/manage.c\0" as *const u8 as *const ::core::ffi::c_char,
                                b"manage_window\0" as *const u8 as *const ::core::ffi::c_char,
                                441 as ::core::ffi::c_int,
                            );
                        }
                    } else {
                        debuglog(
                            b"%s:%s:%d - dock, not focusing\n\0" as *const u8
                                as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b"../src/manage.c\0" as *const u8 as *const ::core::ffi::c_char,
                            b"manage_window\0" as *const u8 as *const ::core::ffi::c_char,
                            444 as ::core::ffi::c_int,
                        );
                    }
                } else {
                    debuglog(
                        b"%s:%s:%d - fs = %p, ws = %p, not focusing\n\0" as *const u8
                            as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        b"../src/manage.c\0" as *const u8 as *const ::core::ffi::c_char,
                        b"manage_window\0" as *const u8 as *const ::core::ffi::c_char,
                        447 as ::core::ffi::c_int,
                        fs,
                        ws,
                    );
                    let mut first: *mut Con = (*(*nc).parent).focus_head.tqh_first as *mut Con;
                    if first != nc {
                        if !(*nc).focused.tqe_next.is_null() {
                            (*(*nc).focused.tqe_next).focused.tqe_prev = (*nc).focused.tqe_prev;
                        } else {
                            (*(*nc).parent).focus_head.tqh_last = (*nc).focused.tqe_prev;
                        }
                        *(*nc).focused.tqe_prev = (*nc).focused.tqe_next;
                        (*nc).focused.tqe_next = (*first).focused.tqe_next;
                        if !(*nc).focused.tqe_next.is_null() {
                            (*(*nc).focused.tqe_next).focused.tqe_prev =
                                &raw mut (*nc).focused.tqe_next;
                        } else {
                            (*(*nc).parent).focus_head.tqh_last = &raw mut (*nc).focused.tqe_next;
                        }
                        (*first).focused.tqe_next = nc as *mut Con;
                        (*nc).focused.tqe_prev = &raw mut (*first).focused.tqe_next;
                    }
                }
                want_floating = false_0 != 0;
                if xcb_reply_contains_atom(type_reply, A__NET_WM_WINDOW_TYPE_DIALOG)
                    as ::core::ffi::c_int
                    != 0
                    || xcb_reply_contains_atom(type_reply, A__NET_WM_WINDOW_TYPE_UTILITY)
                        as ::core::ffi::c_int
                        != 0
                    || xcb_reply_contains_atom(type_reply, A__NET_WM_WINDOW_TYPE_TOOLBAR)
                        as ::core::ffi::c_int
                        != 0
                    || xcb_reply_contains_atom(type_reply, A__NET_WM_WINDOW_TYPE_NOTIFICATION)
                        as ::core::ffi::c_int
                        != 0
                    || xcb_reply_contains_atom(type_reply, A__NET_WM_WINDOW_TYPE_SPLASH)
                        as ::core::ffi::c_int
                        != 0
                    || xcb_reply_contains_atom(state_reply, A__NET_WM_STATE_MODAL)
                        as ::core::ffi::c_int
                        != 0
                    || (*cwindow).max_width > 0 as ::core::ffi::c_int
                        && (*cwindow).max_height > 0 as ::core::ffi::c_int
                        && (*cwindow).min_height == (*cwindow).max_height
                        && (*cwindow).min_width == (*cwindow).max_width
                {
                    verboselog(
                        b"This window is a dialog window, setting floating\n\0" as *const u8
                            as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    );
                    want_floating = true_0 != 0;
                }
                if xcb_reply_contains_atom(state_reply, A__NET_WM_STATE_STICKY) {
                    (*nc).sticky = true_0 != 0;
                }
                if (*cwindow).wm_desktop == NET_WM_DESKTOP_ALL as uint32_t
                    && (ws.is_null() || !con_is_internal(ws))
                {
                    debuglog(
                        b"%s:%s:%d - This window has _NET_WM_DESKTOP = 0xFFFFFFFF. Will float it and make it sticky.\n\0"
                            as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        b"../src/manage.c\0" as *const u8 as *const ::core::ffi::c_char,
                        b"manage_window\0" as *const u8 as *const ::core::ffi::c_char,
                        484 as ::core::ffi::c_int,
                    );
                    (*nc).sticky = true_0 != 0;
                    want_floating = true_0 != 0;
                }
                free(state_reply as *mut ::core::ffi::c_void);
                state_reply = ::core::ptr::null_mut::<xcb_get_property_reply_t>();
                free(type_reply as *mut ::core::ffi::c_void);
                type_reply = ::core::ptr::null_mut::<xcb_get_property_reply_t>();
                if (*cwindow).transient_for as ::core::ffi::c_long != XCB_NONE
                    || (*cwindow).leader as ::core::ffi::c_long != XCB_NONE
                        && (*cwindow).leader != (*cwindow).id
                        && !con_by_window_id((*cwindow).leader).is_null()
                {
                    debuglog(
                        b"%s:%s:%d - This window is transient for another window, setting floating\n\0"
                            as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        b"../src/manage.c\0" as *const u8 as *const ::core::ffi::c_char,
                        b"manage_window\0" as *const u8 as *const ::core::ffi::c_char,
                        496 as ::core::ffi::c_int,
                    );
                    want_floating = true_0 != 0;
                    if config.popup_during_fullscreen as ::core::ffi::c_uint
                        == PDF_LEAVE_FULLSCREEN as ::core::ffi::c_int as ::core::ffi::c_uint
                        && !fs.is_null()
                    {
                        debuglog(
                            b"%s:%s:%d - There is a fullscreen window, leaving fullscreen mode\n\0"
                                as *const u8
                                as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b"../src/manage.c\0" as *const u8 as *const ::core::ffi::c_char,
                            b"manage_window\0" as *const u8 as *const ::core::ffi::c_char,
                            501 as ::core::ffi::c_int,
                        );
                        con_disable_fullscreen(fs);
                    } else if config.popup_during_fullscreen as ::core::ffi::c_uint
                        == PDF_SMART as ::core::ffi::c_int as ::core::ffi::c_uint
                        && !fs.is_null()
                        && !(*fs).window.is_null()
                    {
                        set_focus = con_find_transient_for_window(nc, (*(*fs).window).id);
                    }
                }
                if (*cwindow).dock as u64 != 0 {
                    want_floating = false_0 != 0;
                }
                if (*nc).geometry.width == 0 as uint32_t {
                    (*nc).geometry = Rect {
                        x: (*geom).x as uint32_t,
                        y: (*geom).y as uint32_t,
                        width: (*geom).width as uint32_t,
                        height: (*geom).height as uint32_t,
                    } as Rect;
                }
                if config.popup_during_fullscreen as ::core::ffi::c_uint
                    == PDF_ALL as ::core::ffi::c_int as ::core::ffi::c_uint
                    && want_floating as ::core::ffi::c_int != 0
                    && !fs.is_null()
                {
                    set_focus = true_0 != 0;
                }
                if want_floating {
                    debuglog(
                        b"%s:%s:%d - geometry = %d x %d\n\0" as *const u8
                            as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        b"../src/manage.c\0" as *const u8 as *const ::core::ffi::c_char,
                        b"manage_window\0" as *const u8 as *const ::core::ffi::c_char,
                        529 as ::core::ffi::c_int,
                        (*nc).geometry.width,
                        (*nc).geometry.height,
                    );
                    if floating_enable(nc, true_0 != 0) {
                        (*nc).floating = FLOATING_AUTO_ON;
                    }
                }
                if has_mwm_hints {
                    debuglog(
                        b"%s:%s:%d - MOTIF_WM_HINTS specifies decorations (border_style = %d)\n\0"
                            as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        b"../src/manage.c\0" as *const u8 as *const ::core::ffi::c_char,
                        b"manage_window\0" as *const u8 as *const ::core::ffi::c_char,
                        536 as ::core::ffi::c_int,
                        motif_border_style as ::core::ffi::c_uint,
                    );
                    if want_floating {
                        con_set_border_style(
                            nc,
                            motif_border_style,
                            config.default_floating_border_width,
                        );
                    } else {
                        con_set_border_style(nc, motif_border_style, config.default_border_width);
                    }
                }
                if (*nc).current_border_width == -(1 as ::core::ffi::c_int) {
                    (*nc).current_border_width = if want_floating as ::core::ffi::c_int != 0 {
                        config.default_floating_border_width
                    } else {
                        config.default_border_width
                    };
                }
                values[0 as ::core::ffi::c_int as usize] = XCB_NONE as uint32_t;
                xcb_change_window_attributes(
                    conn,
                    window,
                    XCB_CW_EVENT_MASK as ::core::ffi::c_int as uint32_t,
                    &raw mut values as *mut uint32_t as *const ::core::ffi::c_void,
                );
                rcookie = xcb_reparent_window_checked(
                    conn,
                    window,
                    (*nc).frame.id as xcb_window_t,
                    0 as int16_t,
                    0 as int16_t,
                );
                if !xcb_request_check(conn, rcookie).is_null() {
                    verboselog(
                        b"Could not reparent the window, aborting\n\0" as *const u8
                            as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    );
                } else {
                    values[0 as ::core::ffi::c_int as usize] = ((XCB_EVENT_MASK_PROPERTY_CHANGE
                        as ::core::ffi::c_int
                        | XCB_EVENT_MASK_STRUCTURE_NOTIFY as ::core::ffi::c_int
                        | XCB_EVENT_MASK_FOCUS_CHANGE as ::core::ffi::c_int)
                        & !(XCB_EVENT_MASK_ENTER_WINDOW as ::core::ffi::c_int))
                        as uint32_t;
                    xcb_change_window_attributes(
                        conn,
                        window,
                        XCB_CW_EVENT_MASK as ::core::ffi::c_int as uint32_t,
                        &raw mut values as *mut uint32_t as *const ::core::ffi::c_void,
                    );
                    xcb_flush(conn);
                    xcb_change_save_set(
                        conn,
                        XCB_SET_MODE_INSERT as ::core::ffi::c_int as uint8_t,
                        window,
                    );
                    if shape_supported {
                        xcb_shape_select_input(conn, window, true_0 as uint8_t);
                        let mut cookie_0: xcb_shape_query_extents_cookie_t =
                            xcb_shape_query_extents(conn, window);
                        let mut reply: *mut xcb_shape_query_extents_reply_t =
                            xcb_shape_query_extents_reply(
                                conn,
                                cookie_0,
                                ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
                            );
                        if !reply.is_null() && (*reply).bounding_shaped as ::core::ffi::c_int != 0 {
                            (*cwindow).shaped = true_0 != 0;
                        }
                        free(reply as *mut ::core::ffi::c_void);
                        reply = ::core::ptr::null_mut::<xcb_shape_query_extents_reply_t>();
                    }
                    run_assignments(cwindow);
                    ws = con_get_workspace(nc);
                    if !ws.is_null() && !workspace_is_visible(ws) {
                        (*ws).rect = (*(*ws).parent).rect;
                        render_con(ws);
                        set_focus = false_0 != 0;
                    }
                    render_con(croot);
                    (*cwindow).managed_since = time(::core::ptr::null_mut::<time_t>());
                    ipc_send_window_event(b"new\0" as *const u8 as *const ::core::ffi::c_char, nc);
                    if set_focus as ::core::ffi::c_int != 0
                        && !assignment_for(cwindow, A_NO_FOCUS as ::core::ffi::c_int).is_null()
                    {
                        if con_num_windows(ws) == 1 as ::core::ffi::c_int {
                            debuglog(
                                b"%s:%s:%d - This is the first window on this workspace, ignoring no_focus.\n\0"
                                    as *const u8 as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                                b"../src/manage.c\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                b"manage_window\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                626 as ::core::ffi::c_int,
                            );
                        } else {
                            debuglog(
                                b"%s:%s:%d - no_focus was set for con = %p, not setting focus.\n\0"
                                    as *const u8
                                    as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                                b"../src/manage.c\0" as *const u8 as *const ::core::ffi::c_char,
                                b"manage_window\0" as *const u8 as *const ::core::ffi::c_char,
                                628 as ::core::ffi::c_int,
                                nc,
                            );
                            set_focus = false_0 != 0;
                        }
                    }
                    if set_focus {
                        debuglog(
                            b"%s:%s:%d - Checking con = %p for _NET_WM_USER_TIME.\n\0" as *const u8
                                as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b"../src/manage.c\0" as *const u8 as *const ::core::ffi::c_char,
                            b"manage_window\0" as *const u8 as *const ::core::ffi::c_char,
                            634 as ::core::ffi::c_int,
                            nc,
                        );
                        let mut wm_user_time: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
                        let mut wm_user_time_reply: *mut xcb_get_property_reply_t =
                            xcb_get_property_reply(
                                conn,
                                wm_user_time_cookie,
                                ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
                            );
                        if !wm_user_time_reply.is_null()
                            && xcb_get_property_value_length(wm_user_time_reply)
                                != 0 as ::core::ffi::c_int
                            && {
                                wm_user_time =
                                    xcb_get_property_value(wm_user_time_reply) as *mut uint32_t;
                                !wm_user_time.is_null()
                            }
                            && *wm_user_time.offset(0 as ::core::ffi::c_int as isize)
                                == 0 as uint32_t
                        {
                            debuglog(
                                b"%s:%s:%d - _NET_WM_USER_TIME set to 0, not focusing con = %p.\n\0"
                                    as *const u8
                                    as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                                b"../src/manage.c\0" as *const u8 as *const ::core::ffi::c_char,
                                b"manage_window\0" as *const u8 as *const ::core::ffi::c_char,
                                641 as ::core::ffi::c_int,
                                nc,
                            );
                            set_focus = false_0 != 0;
                        }
                        free(wm_user_time_reply as *mut ::core::ffi::c_void);
                        wm_user_time_reply = ::core::ptr::null_mut::<xcb_get_property_reply_t>();
                    } else {
                        xcb_discard_reply(conn, wm_user_time_cookie.sequence);
                    }
                    if (*(*nc).window).doesnt_accept_focus as ::core::ffi::c_int != 0
                        && !(*(*nc).window).needs_take_focus
                    {
                        set_focus = false_0 != 0;
                    }
                    if set_focus as ::core::ffi::c_int != 0
                        && (*nc).mapped as ::core::ffi::c_int != 0
                    {
                        debuglog(
                            b"%s:%s:%d - Now setting focus.\n\0" as *const u8
                                as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b"../src/manage.c\0" as *const u8 as *const ::core::ffi::c_char,
                            b"manage_window\0" as *const u8 as *const ::core::ffi::c_char,
                            662 as ::core::ffi::c_int,
                        );
                        con_activate(nc);
                    }
                    tree_render();
                    if old_frame as ::core::ffi::c_long != XCB_NONE {
                        xcb_destroy_window(conn, old_frame);
                    }
                    con_set_urgency(nc, urgency_hint);
                    (*cwindow).wm_desktop = NET_WM_DESKTOP_NONE as uint32_t;
                    ewmh_update_wm_desktop();
                    output_push_sticky_windows(focused);
                }
                free(geom as *mut ::core::ffi::c_void);
            }
        }
    }
    free(attr as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn placeholder_for_con(mut con: *mut Con) -> *mut Con {
    if (*(*con).window).swallowed {
        return ::core::ptr::null_mut::<Con>();
    }
    let mut match_0: *mut Match = ::core::ptr::null_mut::<Match>();
    let mut nc: *mut Con = con_for_window(croot, (*con).window as *mut i3Window, &raw mut match_0);
    if nc.is_null() || (*nc).window.is_null() || (*nc).window == (*con).window {
        return ::core::ptr::null_mut::<Con>();
    }
    if (*(*nc).window).managed_since > (*(*con).window).managed_since {
        return ::core::ptr::null_mut::<Con>();
    }
    return nc;
}
#[no_mangle]
pub unsafe extern "C" fn remanage_window(mut con: *mut Con) -> *mut Con {
    let mut nc: *mut Con = placeholder_for_con(con);
    if nc.is_null() {
        run_assignments((*con).window as *mut i3Window);
        return con;
    }
    if !restore_kill_placeholder((*(*nc).window).id) {
        debuglog(
            b"%s:%s:%d - Uh?! Container without a placeholder, but with a window, has swallowed this managed window?!\n\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/manage.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"remanage_window\0" as *const u8 as *const ::core::ffi::c_char,
            726 as ::core::ffi::c_int,
        );
    } else {
        _remove_matches(nc);
    }
    window_free((*nc).window as *mut i3Window);
    let mut old_frame: xcb_window_t = _match_depth((*con).window as *mut i3Window, nc);
    x_reparent_child(nc, con);
    let mut moved_workpaces: bool = con_get_workspace(nc) != con_get_workspace(con);
    con_merge_into(con, nc);
    if old_frame as ::core::ffi::c_long != XCB_NONE {
        xcb_destroy_window(conn, old_frame);
    }
    run_assignments((*nc).window as *mut i3Window);
    if moved_workpaces {
        startup_sequence_delete_by_window((*nc).window);
        ewmh_update_wm_desktop();
    }
    (*(*nc).window).swallowed = true_0 != 0;
    return nc;
}
