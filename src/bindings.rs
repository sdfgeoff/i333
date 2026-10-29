/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * bindings.c: Functions for configuring, finding and, running bindings.
 */
// Mechanical file-by-file Rust translation of src/bindings.c.
// Generated with C2Rust 0.22.1; C ABI and original ownership retained.
extern "C" {
    pub type xcb_connection_t;
    pub type _PangoFontDescription;
    pub type _cairo;
    pub type _cairo_surface;
    pub type _i3String;
    pub type pcre2_real_code_8;
    pub type ev_loop;
    pub type yajl_gen_t;
    pub type xkb_state;
    pub type xkb_keymap;
    pub type xkb_context;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn __assert_single_arg(_: bool) -> bool;
    fn xcb_intern_atom(
        c: *mut xcb_connection_t,
        only_if_exists: uint8_t,
        name_len: uint16_t,
        name: *const ::core::ffi::c_char,
    ) -> xcb_intern_atom_cookie_t;
    fn xcb_intern_atom_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_intern_atom_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_intern_atom_reply_t;
    fn xcb_get_property_unchecked(
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
    fn xcb_ungrab_button(
        c: *mut xcb_connection_t,
        button: uint8_t,
        grab_window: xcb_window_t,
        modifiers: uint16_t,
    ) -> xcb_void_cookie_t;
    fn xcb_grab_key(
        c: *mut xcb_connection_t,
        owner_events: uint8_t,
        grab_window: xcb_window_t,
        modifiers: uint16_t,
        key: xcb_keycode_t,
        pointer_mode: uint8_t,
        keyboard_mode: uint8_t,
    ) -> xcb_void_cookie_t;
    fn xcb_grab_server(c: *mut xcb_connection_t) -> xcb_void_cookie_t;
    fn xcb_ungrab_server(c: *mut xcb_connection_t) -> xcb_void_cookie_t;
    static mut conn: *mut xcb_connection_t;
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
    fn start_nagbar(nagbar_pid: *mut pid_t, argv: *mut *mut ::core::ffi::c_char);
    fn parse_long(
        str: *const ::core::ffi::c_char,
        out: *mut ::core::ffi::c_long,
        base: ::core::ffi::c_int,
    ) -> bool;
    static mut all_cons: all_cons_head;
    fn tree_render();
    static mut xkb_current_group: ::core::ffi::c_int;
    static mut bindings: *mut bindings_head;
    static mut current_binding_mode: *const ::core::ffi::c_char;
    static mut xkb_supported: bool;
    static mut root: xcb_window_t;
    static mut current_configpath: *mut ::core::ffi::c_char;
    static mut config: Config;
    static mut modes: modes_head;
    fn ungrab_all_keys(conn_0: *mut xcb_connection_t);
    fn ipc_send_event(
        event: *const ::core::ffi::c_char,
        message_type: uint32_t,
        payload: *const ::core::ffi::c_char,
    );
    fn ipc_send_binding_event(
        event_type: *const ::core::ffi::c_char,
        bind: *mut Binding,
        modename: *const ::core::ffi::c_char,
    );
    static mut errorfilename: *mut ::core::ffi::c_char;
    static mut xcb_numlock_mask: ::core::ffi::c_uint;
    fn xcb_grab_buttons(
        conn_0: *mut xcb_connection_t,
        window: xcb_window_t,
        buttons: *mut ::core::ffi::c_int,
    );
    fn parse_command(
        input: *const ::core::ffi::c_char,
        gen: yajl_gen,
        client: *mut ipc_client,
    ) -> *mut CommandResult;
    fn start_config_error_nagbar(configpath: *const ::core::ffi::c_char, has_errors: bool);
    fn event_state_from_str(str: *const ::core::ffi::c_char) -> i3_event_state_mask_t;
    fn ceil(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn xkb_keysym_from_name(
        name: *const ::core::ffi::c_char,
        flags: xkb_keysym_flags,
    ) -> xkb_keysym_t;
    fn xkb_context_new(flags: xkb_context_flags) -> *mut xkb_context;
    fn xkb_keymap_new_from_names(
        context: *mut xkb_context,
        names: *const xkb_rule_names,
        flags: xkb_keymap_compile_flags,
    ) -> *mut xkb_keymap;
    fn xkb_keymap_unref(keymap: *mut xkb_keymap);
    fn xkb_keymap_key_for_each(
        keymap: *mut xkb_keymap,
        iter: xkb_keymap_key_iter_t,
        data: *mut ::core::ffi::c_void,
    );
    fn xkb_state_new(keymap: *mut xkb_keymap) -> *mut xkb_state;
    fn xkb_state_unref(state: *mut xkb_state);
    fn xkb_state_update_mask(
        state: *mut xkb_state,
        depressed_mods: xkb_mod_mask_t,
        latched_mods: xkb_mod_mask_t,
        locked_mods: xkb_mod_mask_t,
        depressed_layout: xkb_layout_index_t,
        latched_layout: xkb_layout_index_t,
        locked_layout: xkb_layout_index_t,
    ) -> xkb_state_component;
    fn xkb_state_key_get_one_sym(state: *mut xkb_state, key: xkb_keycode_t) -> xkb_keysym_t;
    fn xkb_state_key_get_layout(state: *mut xkb_state, key: xkb_keycode_t) -> xkb_layout_index_t;
    fn xkb_state_key_get_level(
        state: *mut xkb_state,
        key: xkb_keycode_t,
        layout: xkb_layout_index_t,
    ) -> xkb_level_index_t;
    fn xkb_x11_get_core_keyboard_device_id(connection: *mut xcb_connection_t) -> int32_t;
    fn xkb_x11_keymap_new_from_device(
        context: *mut xkb_context,
        connection: *mut xcb_connection_t,
        device_id: int32_t,
        flags: xkb_keymap_compile_flags,
    ) -> *mut xkb_keymap;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn strnlen(__string: *const ::core::ffi::c_char, __maxlen: size_t) -> size_t;
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
    fn qsort(
        __base: *mut ::core::ffi::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
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
pub type time_t = __time_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type __compar_fn_t = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_void,
        *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
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
pub type xcb_mod_mask_t = ::core::ffi::c_uint;
pub const XCB_MOD_MASK_ANY: xcb_mod_mask_t = 32768;
pub const XCB_MOD_MASK_5: xcb_mod_mask_t = 128;
pub const XCB_MOD_MASK_4: xcb_mod_mask_t = 64;
pub const XCB_MOD_MASK_3: xcb_mod_mask_t = 32;
pub const XCB_MOD_MASK_2: xcb_mod_mask_t = 16;
pub const XCB_MOD_MASK_1: xcb_mod_mask_t = 8;
pub const XCB_MOD_MASK_CONTROL: xcb_mod_mask_t = 4;
pub const XCB_MOD_MASK_LOCK: xcb_mod_mask_t = 2;
pub const XCB_MOD_MASK_SHIFT: xcb_mod_mask_t = 1;
pub type xcb_key_but_mask_t = ::core::ffi::c_uint;
pub const XCB_KEY_BUT_MASK_BUTTON_5: xcb_key_but_mask_t = 4096;
pub const XCB_KEY_BUT_MASK_BUTTON_4: xcb_key_but_mask_t = 2048;
pub const XCB_KEY_BUT_MASK_BUTTON_3: xcb_key_but_mask_t = 1024;
pub const XCB_KEY_BUT_MASK_BUTTON_2: xcb_key_but_mask_t = 512;
pub const XCB_KEY_BUT_MASK_BUTTON_1: xcb_key_but_mask_t = 256;
pub const XCB_KEY_BUT_MASK_MOD_5: xcb_key_but_mask_t = 128;
pub const XCB_KEY_BUT_MASK_MOD_4: xcb_key_but_mask_t = 64;
pub const XCB_KEY_BUT_MASK_MOD_3: xcb_key_but_mask_t = 32;
pub const XCB_KEY_BUT_MASK_MOD_2: xcb_key_but_mask_t = 16;
pub const XCB_KEY_BUT_MASK_MOD_1: xcb_key_but_mask_t = 8;
pub const XCB_KEY_BUT_MASK_CONTROL: xcb_key_but_mask_t = 4;
pub const XCB_KEY_BUT_MASK_LOCK: xcb_key_but_mask_t = 2;
pub const XCB_KEY_BUT_MASK_SHIFT: xcb_key_but_mask_t = 1;
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
pub type xcb_button_mask_t = ::core::ffi::c_uint;
pub const XCB_BUTTON_MASK_ANY: xcb_button_mask_t = 32768;
pub const XCB_BUTTON_MASK_5: xcb_button_mask_t = 4096;
pub const XCB_BUTTON_MASK_4: xcb_button_mask_t = 2048;
pub const XCB_BUTTON_MASK_3: xcb_button_mask_t = 1024;
pub const XCB_BUTTON_MASK_2: xcb_button_mask_t = 512;
pub const XCB_BUTTON_MASK_1: xcb_button_mask_t = 256;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_intern_atom_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_intern_atom_reply_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub atom: xcb_atom_t,
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
pub type xcb_grab_mode_t = ::core::ffi::c_uint;
pub const XCB_GRAB_MODE_ASYNC: xcb_grab_mode_t = 1;
pub const XCB_GRAB_MODE_SYNC: xcb_grab_mode_t = 0;
pub type xcb_button_index_t = ::core::ffi::c_uint;
pub const XCB_BUTTON_INDEX_5: xcb_button_index_t = 5;
pub const XCB_BUTTON_INDEX_4: xcb_button_index_t = 4;
pub const XCB_BUTTON_INDEX_3: xcb_button_index_t = 3;
pub const XCB_BUTTON_INDEX_2: xcb_button_index_t = 2;
pub const XCB_BUTTON_INDEX_1: xcb_button_index_t = 1;
pub const XCB_BUTTON_INDEX_ANY: xcb_button_index_t = 0;
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
pub type xcb_xkb_group_t = ::core::ffi::c_uint;
pub const XCB_XKB_GROUP_4: xcb_xkb_group_t = 3;
pub const XCB_XKB_GROUP_3: xcb_xkb_group_t = 2;
pub const XCB_XKB_GROUP_2: xcb_xkb_group_t = 1;
pub const XCB_XKB_GROUP_1: xcb_xkb_group_t = 0;
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
pub type pcre2_code_8 = pcre2_real_code_8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Binding {
    pub input_type: input_type_t,
    pub release: C2RustUnnamed_4,
    pub border: bool,
    pub whole_window: bool,
    pub exclude_titlebar: bool,
    pub keycode: uint32_t,
    pub event_state_mask: i3_event_state_mask_t,
    pub symbol: *mut ::core::ffi::c_char,
    pub keycodes_head: keycodes_head,
    pub command: *mut ::core::ffi::c_char,
    pub bindings: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub tqe_next: *mut Binding,
    pub tqe_prev: *mut *mut Binding,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct keycodes_head {
    pub tqh_first: *mut Binding_Keycode,
    pub tqh_last: *mut *mut Binding_Keycode,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Binding_Keycode {
    pub keycode: xcb_keycode_t,
    pub modifiers: i3_event_state_mask_t,
    pub keycodes: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub tqe_next: *mut Binding_Keycode,
    pub tqe_prev: *mut *mut Binding_Keycode,
}
pub type i3_event_state_mask_t = uint32_t;
pub type C2RustUnnamed_4 = ::core::ffi::c_uint;
pub const B_UPON_KEYRELEASE_IGNORE_MODS: C2RustUnnamed_4 = 2;
pub const B_UPON_KEYRELEASE: C2RustUnnamed_4 = 1;
pub const B_UPON_KEYPRESS: C2RustUnnamed_4 = 0;
pub type input_type_t = ::core::ffi::c_uint;
pub const B_MOUSE: input_type_t = 1;
pub const B_KEYBOARD: input_type_t = 0;
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
    pub type_0: C2RustUnnamed_21,
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
    pub floating: C2RustUnnamed_10,
    pub nodes: C2RustUnnamed_9,
    pub focused: C2RustUnnamed_8,
    pub all_cons: C2RustUnnamed_7,
    pub floating_windows: C2RustUnnamed_6,
    pub on_remove_child: Option<unsafe extern "C" fn(*mut Con) -> ()>,
    pub scratchpad_state: C2RustUnnamed_5,
    pub old_id: ::core::ffi::c_int,
    pub depth: uint16_t,
    pub colormap: xcb_colormap_t,
}
pub type C2RustUnnamed_5 = ::core::ffi::c_uint;
pub const SCRATCHPAD_CHANGED: C2RustUnnamed_5 = 2;
pub const SCRATCHPAD_FRESH: C2RustUnnamed_5 = 1;
pub const SCRATCHPAD_NONE: C2RustUnnamed_5 = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub tqe_next: *mut Con,
    pub tqe_prev: *mut *mut Con,
}
pub type C2RustUnnamed_10 = ::core::ffi::c_uint;
pub const FLOATING_USER_ON: C2RustUnnamed_10 = 3;
pub const FLOATING_AUTO_ON: C2RustUnnamed_10 = 2;
pub const FLOATING_USER_OFF: C2RustUnnamed_10 = 1;
pub const FLOATING_AUTO_OFF: C2RustUnnamed_10 = 0;
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
    pub urgent: C2RustUnnamed_15,
    pub dock: C2RustUnnamed_14,
    pub id: xcb_window_t,
    pub window_mode: C2RustUnnamed_13,
    pub con_id: *mut Con,
    pub match_all_windows: bool,
    pub insert_where: C2RustUnnamed_12,
    pub matches: C2RustUnnamed_11,
    pub restart_mode: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub tqe_next: *mut Match,
    pub tqe_prev: *mut *mut Match,
}
pub type C2RustUnnamed_12 = ::core::ffi::c_uint;
pub const M_BELOW: C2RustUnnamed_12 = 2;
pub const M_ASSIGN_WS: C2RustUnnamed_12 = 1;
pub const M_HERE: C2RustUnnamed_12 = 0;
pub type C2RustUnnamed_13 = ::core::ffi::c_uint;
pub const WM_FLOATING: C2RustUnnamed_13 = 6;
pub const WM_FLOATING_USER: C2RustUnnamed_13 = 5;
pub const WM_FLOATING_AUTO: C2RustUnnamed_13 = 4;
pub const WM_TILING: C2RustUnnamed_13 = 3;
pub const WM_TILING_USER: C2RustUnnamed_13 = 2;
pub const WM_TILING_AUTO: C2RustUnnamed_13 = 1;
pub const WM_ANY: C2RustUnnamed_13 = 0;
pub type C2RustUnnamed_14 = ::core::ffi::c_int;
pub const M_DOCK_BOTTOM: C2RustUnnamed_14 = 3;
pub const M_DOCK_TOP: C2RustUnnamed_14 = 2;
pub const M_DOCK_ANY: C2RustUnnamed_14 = 1;
pub const M_NODOCK: C2RustUnnamed_14 = 0;
pub const M_DONTCHECK: C2RustUnnamed_14 = -1;
pub type C2RustUnnamed_15 = ::core::ffi::c_int;
pub const U_OLDEST: C2RustUnnamed_15 = 1;
pub const U_LATEST: C2RustUnnamed_15 = 0;
pub const U_DONTCHECK: C2RustUnnamed_15 = -1;
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
    pub dock: C2RustUnnamed_16,
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
pub type C2RustUnnamed_16 = ::core::ffi::c_uint;
pub const W_DOCK_BOTTOM: C2RustUnnamed_16 = 2;
pub const W_DOCK_TOP: C2RustUnnamed_16 = 1;
pub const W_NODOCK: C2RustUnnamed_16 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Assignment {
    pub type_0: C2RustUnnamed_19,
    pub match_0: Match,
    pub dest: C2RustUnnamed_18,
    pub assignments: C2RustUnnamed_17,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_17 {
    pub tqe_next: *mut Assignment,
    pub tqe_prev: *mut *mut Assignment,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_18 {
    pub command: *mut ::core::ffi::c_char,
    pub workspace: *mut ::core::ffi::c_char,
    pub output: *mut ::core::ffi::c_char,
}
pub type C2RustUnnamed_19 = ::core::ffi::c_uint;
pub const A_TO_OUTPUT: C2RustUnnamed_19 = 16;
pub const A_TO_WORKSPACE_NUMBER: C2RustUnnamed_19 = 8;
pub const A_NO_FOCUS: C2RustUnnamed_19 = 4;
pub const A_TO_WORKSPACE: C2RustUnnamed_19 = 2;
pub const A_COMMAND: C2RustUnnamed_19 = 1;
pub const A_ANY: C2RustUnnamed_19 = 0;
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
    pub marks: C2RustUnnamed_20,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_20 {
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
pub type C2RustUnnamed_21 = ::core::ffi::c_uint;
pub const CT_DOCKAREA: C2RustUnnamed_21 = 5;
pub const CT_WORKSPACE: C2RustUnnamed_21 = 4;
pub const CT_FLOATING_CON: C2RustUnnamed_21 = 3;
pub const CT_CON: C2RustUnnamed_21 = 2;
pub const CT_OUTPUT: C2RustUnnamed_21 = 1;
pub const CT_ROOT: C2RustUnnamed_21 = 0;
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
pub type C2RustUnnamed_22 = ::core::ffi::c_uint;
pub const I3_XKB_GROUP_MASK_4: C2RustUnnamed_22 = 8;
pub const I3_XKB_GROUP_MASK_3: C2RustUnnamed_22 = 4;
pub const I3_XKB_GROUP_MASK_2: C2RustUnnamed_22 = 2;
pub const I3_XKB_GROUP_MASK_1: C2RustUnnamed_22 = 1;
pub const I3_XKB_GROUP_MASK_ANY: C2RustUnnamed_22 = 0;
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
pub type yajl_gen = *mut yajl_gen_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct all_cons_head {
    pub tqh_first: *mut Con,
    pub tqh_last: *mut *mut Con,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bindings_head {
    pub tqh_first: *mut Binding,
    pub tqh_last: *mut *mut Binding,
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
pub struct modes_head {
    pub slh_first: *mut Mode,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Mode {
    pub name: *mut ::core::ffi::c_char,
    pub pango_markup: bool,
    pub bindings: *mut bindings_head,
    pub modes: C2RustUnnamed_26,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_26 {
    pub sle_next: *mut Mode,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct context {
    pub has_errors: bool,
    pub has_warnings: bool,
    pub line_number: ::core::ffi::c_int,
    pub line_copy: *mut ::core::ffi::c_char,
    pub filename: *const ::core::ffi::c_char,
    pub compact_error: *mut ::core::ffi::c_char,
    pub first_column: ::core::ffi::c_int,
    pub last_column: ::core::ffi::c_int,
}
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
    pub clients: C2RustUnnamed_27,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_27 {
    pub tqe_next: *mut ipc_client,
    pub tqe_prev: *mut *mut ipc_client,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CommandResult {
    pub parse_error: bool,
    pub error_message: *mut ::core::ffi::c_char,
    pub needs_tree_render: bool,
}
pub type xkb_keysym_t = uint32_t;
pub type xkb_keysym_flags = ::core::ffi::c_uint;
pub const XKB_KEYSYM_CASE_INSENSITIVE: xkb_keysym_flags = 1;
pub const XKB_KEYSYM_NO_FLAGS: xkb_keysym_flags = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct resolve {
    pub bind: *mut Binding,
    pub keysym: xkb_keysym_t,
    pub xkb_state: *mut xkb_state,
    pub xkb_state_no_shift: *mut xkb_state,
    pub xkb_state_numlock: *mut xkb_state,
    pub xkb_state_numlock_no_shift: *mut xkb_state,
}
pub type xkb_keycode_t = uint32_t;
pub type xkb_level_index_t = uint32_t;
pub type xkb_layout_index_t = uint32_t;
pub type xkb_keymap_key_iter_t =
    Option<unsafe extern "C" fn(*mut xkb_keymap, xkb_keycode_t, *mut ::core::ffi::c_void) -> ()>;
pub type xkb_state_component = ::core::ffi::c_uint;
pub const XKB_STATE_LEDS: xkb_state_component = 256;
pub const XKB_STATE_LAYOUT_EFFECTIVE: xkb_state_component = 128;
pub const XKB_STATE_LAYOUT_LOCKED: xkb_state_component = 64;
pub const XKB_STATE_LAYOUT_LATCHED: xkb_state_component = 32;
pub const XKB_STATE_LAYOUT_DEPRESSED: xkb_state_component = 16;
pub const XKB_STATE_MODS_EFFECTIVE: xkb_state_component = 8;
pub const XKB_STATE_MODS_LOCKED: xkb_state_component = 4;
pub const XKB_STATE_MODS_LATCHED: xkb_state_component = 2;
pub const XKB_STATE_MODS_DEPRESSED: xkb_state_component = 1;
pub type xkb_mod_mask_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xkb_rule_names {
    pub rules: *const ::core::ffi::c_char,
    pub model: *const ::core::ffi::c_char,
    pub layout: *const ::core::ffi::c_char,
    pub variant: *const ::core::ffi::c_char,
    pub options: *const ::core::ffi::c_char,
}
pub type xkb_keymap_compile_flags = ::core::ffi::c_uint;
pub const XKB_KEYMAP_COMPILE_NO_FLAGS: xkb_keymap_compile_flags = 0;
pub type xkb_context_flags = ::core::ffi::c_uint;
pub const XKB_CONTEXT_NO_SECURE_GETENV: xkb_context_flags = 4;
pub const XKB_CONTEXT_NO_ENVIRONMENT_NAMES: xkb_context_flags = 2;
pub const XKB_CONTEXT_NO_DEFAULT_INCLUDES: xkb_context_flags = 1;
pub const XKB_CONTEXT_NO_FLAGS: xkb_context_flags = 0;
pub const __ASSERT_FUNCTION: [::core::ffi::c_char; 45] = unsafe {
    ::core::mem::transmute::<[u8; 45], [::core::ffi::c_char; 45]>(
        *b"void reorder_bindings_of_mode(struct Mode *)\0",
    )
};
pub const XCB_KEY_RELEASE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const XCB_BUTTON_PRESS: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const XCB_BUTTON_RELEASE: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const I3_IPC_EVENT_MASK: ::core::ffi::c_ulong =
    (1 as ::core::ffi::c_ulong) << 31 as ::core::ffi::c_int;
pub const I3_IPC_EVENT_MODE: ::core::ffi::c_ulong = I3_IPC_EVENT_MASK | 2 as ::core::ffi::c_ulong;
pub const XKB_LAYOUT_INVALID: ::core::ffi::c_uint = 0xffffffff as ::core::ffi::c_uint;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const XKB_KEY_NoSymbol: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const XKB_KEY_KP_Space: ::core::ffi::c_int = 0xff80 as ::core::ffi::c_int;
pub const XKB_KEY_KP_Equal: ::core::ffi::c_int = 0xffbd as ::core::ffi::c_int;
static mut xkb_context: *mut xkb_context = ::core::ptr::null::<xkb_context>() as *mut xkb_context;
static mut xkb_keymap: *mut xkb_keymap = ::core::ptr::null::<xkb_keymap>() as *mut xkb_keymap;
#[no_mangle]
pub static mut command_error_nagbar_pid: pid_t = -(1 as pid_t);
#[no_mangle]
pub static mut DEFAULT_BINDING_MODE: *const ::core::ffi::c_char =
    b"default\0" as *const u8 as *const ::core::ffi::c_char;
unsafe extern "C" fn mode_from_name(
    mut name: *const ::core::ffi::c_char,
    mut pango_markup: bool,
) -> *mut Mode {
    let mut mode: *mut Mode = ::core::ptr::null_mut::<Mode>();
    mode = modes.slh_first as *mut Mode;
    while !mode.is_null() {
        if strcmp((*mode).name, name) == 0 as ::core::ffi::c_int {
            return mode;
        }
        mode = (*mode).modes.sle_next;
    }
    mode = scalloc(1 as size_t, ::core::mem::size_of::<Mode>() as size_t) as *mut Mode;
    (*mode).name = sstrdup(name);
    (*mode).pango_markup = pango_markup;
    (*mode).bindings = scalloc(
        1 as size_t,
        ::core::mem::size_of::<bindings_head>() as size_t,
    ) as *mut bindings_head;
    (*(*mode).bindings).tqh_first = ::core::ptr::null_mut::<Binding>();
    (*(*mode).bindings).tqh_last = &raw mut (*(*mode).bindings).tqh_first;
    (*mode).modes.sle_next = modes.slh_first as *mut Mode;
    modes.slh_first = mode as *mut Mode;
    return mode;
}
#[no_mangle]
pub unsafe extern "C" fn configure_binding(
    mut bindtype: *const ::core::ffi::c_char,
    mut modifiers: *const ::core::ffi::c_char,
    mut input_code: *const ::core::ffi::c_char,
    mut release: *const ::core::ffi::c_char,
    mut border: *const ::core::ffi::c_char,
    mut whole_window: *const ::core::ffi::c_char,
    mut exclude_titlebar: *const ::core::ffi::c_char,
    mut command: *const ::core::ffi::c_char,
    mut modename: *const ::core::ffi::c_char,
    mut pango_markup: bool,
) -> *mut Binding {
    let mut new_binding: *mut Binding =
        scalloc(1 as size_t, ::core::mem::size_of::<Binding>() as size_t) as *mut Binding;
    debuglog(
        b"%s:%s:%d - Binding %p bindtype %s, modifiers %s, input code %s, release %s\n\0"
            as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/bindings.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"configure_binding\0" as *const u8 as *const ::core::ffi::c_char,
        64 as ::core::ffi::c_int,
        new_binding,
        bindtype,
        modifiers,
        input_code,
        release,
    );
    (*new_binding).release = (if !release.is_null() {
        B_UPON_KEYRELEASE as ::core::ffi::c_int
    } else {
        B_UPON_KEYPRESS as ::core::ffi::c_int
    }) as C2RustUnnamed_4;
    (*new_binding).border = !border.is_null();
    (*new_binding).whole_window = !whole_window.is_null();
    (*new_binding).exclude_titlebar = !exclude_titlebar.is_null();
    if strcmp(
        bindtype,
        b"bindsym\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        (*new_binding).input_type = (if strncasecmp(
            input_code,
            b"button\0" as *const u8 as *const ::core::ffi::c_char,
            (::core::mem::size_of::<[::core::ffi::c_char; 7]>() as size_t)
                .wrapping_sub(1 as size_t),
        ) == 0 as ::core::ffi::c_int
        {
            B_MOUSE as ::core::ffi::c_int
        } else {
            B_KEYBOARD as ::core::ffi::c_int
        }) as input_type_t;
        (*new_binding).symbol = sstrdup(input_code);
    } else {
        let mut keycode: ::core::ffi::c_long = 0;
        if !parse_long(input_code, &raw mut keycode, 10 as ::core::ffi::c_int) {
            errorlog(
                b"ERROR: Could not parse \"%s\" as an input code, ignoring this binding.\n\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                input_code,
            );
            free(new_binding as *mut ::core::ffi::c_void);
            new_binding = ::core::ptr::null_mut::<Binding>();
            return ::core::ptr::null_mut::<Binding>();
        }
        (*new_binding).keycode = keycode as uint32_t;
        (*new_binding).input_type = B_KEYBOARD;
    }
    (*new_binding).command = sstrdup(command);
    (*new_binding).event_state_mask = event_state_from_str(modifiers);
    let mut group_bits_set: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if (*new_binding).event_state_mask >> 16 as ::core::ffi::c_int
        & I3_XKB_GROUP_MASK_1 as ::core::ffi::c_int as i3_event_state_mask_t
        != 0
    {
        group_bits_set += 1;
    }
    if (*new_binding).event_state_mask >> 16 as ::core::ffi::c_int
        & I3_XKB_GROUP_MASK_2 as ::core::ffi::c_int as i3_event_state_mask_t
        != 0
    {
        group_bits_set += 1;
    }
    if (*new_binding).event_state_mask >> 16 as ::core::ffi::c_int
        & I3_XKB_GROUP_MASK_3 as ::core::ffi::c_int as i3_event_state_mask_t
        != 0
    {
        group_bits_set += 1;
    }
    if (*new_binding).event_state_mask >> 16 as ::core::ffi::c_int
        & I3_XKB_GROUP_MASK_4 as ::core::ffi::c_int as i3_event_state_mask_t
        != 0
    {
        group_bits_set += 1;
    }
    if group_bits_set > 1 as ::core::ffi::c_int {
        errorlog(
            b"ERROR: Keybinding has more than one Group specified, but your X server is always in precisely one group. The keybinding can never trigger.\n\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    let mut mode: *const Mode = mode_from_name(modename, pango_markup);
    (*new_binding).bindings.tqe_next = ::core::ptr::null_mut::<Binding>();
    (*new_binding).bindings.tqe_prev = (*(*mode).bindings).tqh_last;
    *(*(*mode).bindings).tqh_last = new_binding as *mut Binding;
    (*(*mode).bindings).tqh_last = &raw mut (*new_binding).bindings.tqe_next;
    (*new_binding).keycodes_head.tqh_first = ::core::ptr::null_mut::<Binding_Keycode>();
    (*new_binding).keycodes_head.tqh_last = &raw mut (*new_binding).keycodes_head.tqh_first;
    return new_binding;
}
unsafe extern "C" fn binding_in_current_group(mut bind: *const Binding) -> bool {
    if (*bind).event_state_mask >> 16 as ::core::ffi::c_int
        == I3_XKB_GROUP_MASK_ANY as ::core::ffi::c_int as i3_event_state_mask_t
    {
        return true_0 != 0;
    }
    match xkb_current_group {
        0 => {
            return (*bind).event_state_mask >> 16 as ::core::ffi::c_int
                & I3_XKB_GROUP_MASK_1 as ::core::ffi::c_int as i3_event_state_mask_t
                != 0;
        }
        1 => {
            return (*bind).event_state_mask >> 16 as ::core::ffi::c_int
                & I3_XKB_GROUP_MASK_2 as ::core::ffi::c_int as i3_event_state_mask_t
                != 0;
        }
        2 => {
            return (*bind).event_state_mask >> 16 as ::core::ffi::c_int
                & I3_XKB_GROUP_MASK_3 as ::core::ffi::c_int as i3_event_state_mask_t
                != 0;
        }
        3 => {
            return (*bind).event_state_mask >> 16 as ::core::ffi::c_int
                & I3_XKB_GROUP_MASK_4 as ::core::ffi::c_int as i3_event_state_mask_t
                != 0;
        }
        _ => {
            errorlog(
                b"ERROR: BUG: xkb_current_group (= %d) outside of [XCB_XKB_GROUP_1..XCB_XKB_GROUP_4]\n\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                xkb_current_group,
            );
            return false_0 != 0;
        }
    };
}
unsafe extern "C" fn grab_keycode_for_binding(
    mut conn_0: *mut xcb_connection_t,
    mut bind: *mut Binding,
    mut keycode: uint32_t,
) {
    let mods: ::core::ffi::c_int =
        ((*bind).event_state_mask & 0xffff as i3_event_state_mask_t) as ::core::ffi::c_int;
    debuglog(
        b"%s:%s:%d - Binding %p Grabbing keycode %d with event state mask 0x%x (mods 0x%x)\n\0"
            as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/bindings.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"grab_keycode_for_binding\0" as *const u8 as *const ::core::ffi::c_char,
        141 as ::core::ffi::c_int,
        bind,
        keycode,
        (*bind).event_state_mask,
        mods,
    );
    xcb_grab_key(
        conn_0,
        0 as uint8_t,
        root,
        mods as uint16_t,
        keycode as xcb_keycode_t,
        XCB_GRAB_MODE_ASYNC as ::core::ffi::c_int as uint8_t,
        XCB_GRAB_MODE_ASYNC as ::core::ffi::c_int as uint8_t,
    );
    xcb_grab_key(
        conn_0,
        0 as uint8_t,
        root,
        (mods as ::core::ffi::c_uint | xcb_numlock_mask) as uint16_t,
        keycode as xcb_keycode_t,
        XCB_GRAB_MODE_ASYNC as ::core::ffi::c_int as uint8_t,
        XCB_GRAB_MODE_ASYNC as ::core::ffi::c_int as uint8_t,
    );
    xcb_grab_key(
        conn_0,
        0 as uint8_t,
        root,
        (mods | XCB_MOD_MASK_LOCK as ::core::ffi::c_int) as uint16_t,
        keycode as xcb_keycode_t,
        XCB_GRAB_MODE_ASYNC as ::core::ffi::c_int as uint8_t,
        XCB_GRAB_MODE_ASYNC as ::core::ffi::c_int as uint8_t,
    );
    xcb_grab_key(
        conn_0,
        0 as uint8_t,
        root,
        (mods as ::core::ffi::c_uint
            | xcb_numlock_mask
            | XCB_MOD_MASK_LOCK as ::core::ffi::c_int as ::core::ffi::c_uint) as uint16_t,
        keycode as xcb_keycode_t,
        XCB_GRAB_MODE_ASYNC as ::core::ffi::c_int as uint8_t,
        XCB_GRAB_MODE_ASYNC as ::core::ffi::c_int as uint8_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn grab_all_keys(mut conn_0: *mut xcb_connection_t) {
    let mut bind: *mut Binding = ::core::ptr::null_mut::<Binding>();
    bind = (*bindings).tqh_first as *mut Binding;
    while !bind.is_null() {
        if !((*bind).input_type as ::core::ffi::c_uint
            != B_KEYBOARD as ::core::ffi::c_int as ::core::ffi::c_uint)
        {
            if binding_in_current_group(bind) {
                if (*bind).keycode > 0 as uint32_t {
                    grab_keycode_for_binding(conn_0, bind, (*bind).keycode);
                } else {
                    let mut binding_keycode: *mut Binding_Keycode =
                        ::core::ptr::null_mut::<Binding_Keycode>();
                    binding_keycode = (*bind).keycodes_head.tqh_first;
                    while !binding_keycode.is_null() {
                        let keycode: ::core::ffi::c_int =
                            (*binding_keycode).keycode as ::core::ffi::c_int;
                        let mods: ::core::ffi::c_int = ((*binding_keycode).modifiers
                            & 0xffff as i3_event_state_mask_t)
                            as ::core::ffi::c_int;
                        debuglog(
                            b"%s:%s:%d - Binding %p Grabbing keycode %d with mods %d\n\0"
                                as *const u8
                                as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b"../src/bindings.c\0" as *const u8 as *const ::core::ffi::c_char,
                            b"grab_all_keys\0" as *const u8 as *const ::core::ffi::c_char,
                            176 as ::core::ffi::c_int,
                            bind,
                            keycode,
                            mods,
                        );
                        xcb_grab_key(
                            conn_0,
                            0 as uint8_t,
                            root,
                            mods as uint16_t,
                            keycode as xcb_keycode_t,
                            XCB_GRAB_MODE_ASYNC as ::core::ffi::c_int as uint8_t,
                            XCB_GRAB_MODE_ASYNC as ::core::ffi::c_int as uint8_t,
                        );
                        binding_keycode = (*binding_keycode).keycodes.tqe_next;
                    }
                }
            }
        }
        bind = (*bind).bindings.tqe_next as *mut Binding;
    }
}
#[no_mangle]
pub unsafe extern "C" fn regrab_all_buttons(mut conn_0: *mut xcb_connection_t) {
    let mut buttons: *mut ::core::ffi::c_int = bindings_get_buttons_to_grab();
    xcb_grab_server(conn_0);
    let mut con: *mut Con = ::core::ptr::null_mut::<Con>();
    con = all_cons.tqh_first as *mut Con;
    while !con.is_null() {
        if !(*con).window.is_null() {
            xcb_ungrab_button(
                conn_0,
                XCB_BUTTON_INDEX_ANY as ::core::ffi::c_int as uint8_t,
                (*(*con).window).id,
                XCB_BUTTON_MASK_ANY as ::core::ffi::c_int as uint16_t,
            );
            xcb_grab_buttons(conn_0, (*(*con).window).id, buttons);
        }
        con = (*con).all_cons.tqe_next as *mut Con;
    }
    free(buttons as *mut ::core::ffi::c_void);
    buttons = ::core::ptr::null_mut::<::core::ffi::c_int>();
    xcb_ungrab_server(conn_0);
}
unsafe extern "C" fn get_binding(
    mut state_filtered: i3_event_state_mask_t,
    mut is_release: bool,
    mut input_code: uint16_t,
    mut input_type: input_type_t,
) -> *mut Binding {
    let mut bind: *mut Binding = ::core::ptr::null_mut::<Binding>();
    let mut result: *mut Binding = ::core::ptr::null_mut::<Binding>();
    if !is_release {
        bind = (*bindings).tqh_first as *mut Binding;
        while !bind.is_null() {
            if !((*bind).input_type as ::core::ffi::c_uint != input_type as ::core::ffi::c_uint) {
                if (*bind).release as ::core::ffi::c_uint
                    == B_UPON_KEYRELEASE_IGNORE_MODS as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    (*bind).release = B_UPON_KEYRELEASE;
                }
            }
            bind = (*bind).bindings.tqe_next as *mut Binding;
        }
    }
    let xkb_group_state: uint32_t = state_filtered as uint32_t & 0xffff0000 as uint32_t;
    let modifiers_state: uint32_t = state_filtered as uint32_t & 0xffff as uint32_t;
    let mut current_block_25: u64;
    bind = (*bindings).tqh_first as *mut Binding;
    while !bind.is_null() {
        if !((*bind).input_type as ::core::ffi::c_uint != input_type as ::core::ffi::c_uint) {
            let xkb_group_mask: uint32_t =
                (*bind).event_state_mask as uint32_t & 0xffff0000 as uint32_t;
            let groups_match: bool = xkb_group_state & xkb_group_mask == xkb_group_mask;
            if !groups_match {
                debuglog(
                    b"%s:%s:%d - skipping binding %p because XKB groups do not match\n\0"
                        as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    b"../src/bindings.c\0" as *const u8 as *const ::core::ffi::c_char,
                    b"get_binding\0" as *const u8 as *const ::core::ffi::c_char,
                    237 as ::core::ffi::c_int,
                    bind,
                );
            } else {
                let mut found_keycode: bool = false_0 != 0;
                if input_type as ::core::ffi::c_uint
                    == B_KEYBOARD as ::core::ffi::c_int as ::core::ffi::c_uint
                    && !(*bind).symbol.is_null()
                {
                    let input_keycode: xcb_keycode_t = input_code as xcb_keycode_t;
                    let mut binding_keycode: *mut Binding_Keycode =
                        ::core::ptr::null_mut::<Binding_Keycode>();
                    binding_keycode = (*bind).keycodes_head.tqh_first;
                    while !binding_keycode.is_null() {
                        let modifiers_mask: uint32_t =
                            (*binding_keycode).modifiers as uint32_t & 0xffff as uint32_t;
                        let mods_match: bool = modifiers_mask == modifiers_state;
                        debuglog(
                            b"%s:%s:%d - binding_keycode->modifiers = %d, modifiers_mask = %d, modifiers_state = %d, mods_match = %s\n\0"
                                as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b"../src/bindings.c\0" as *const u8
                                as *const ::core::ffi::c_char,
                            b"get_binding\0" as *const u8 as *const ::core::ffi::c_char,
                            252 as ::core::ffi::c_int,
                            (*binding_keycode).modifiers,
                            modifiers_mask,
                            modifiers_state,
                            if mods_match as ::core::ffi::c_int != 0 {
                                b"yes\0" as *const u8 as *const ::core::ffi::c_char
                            } else {
                                b"no\0" as *const u8 as *const ::core::ffi::c_char
                            },
                        );
                        if (*binding_keycode).keycode as ::core::ffi::c_int
                            == input_keycode as ::core::ffi::c_int
                            && (mods_match as ::core::ffi::c_int != 0
                                || (*bind).release as ::core::ffi::c_uint
                                    == B_UPON_KEYRELEASE_IGNORE_MODS as ::core::ffi::c_int
                                        as ::core::ffi::c_uint
                                    && is_release as ::core::ffi::c_int != 0)
                        {
                            found_keycode = true_0 != 0;
                            break;
                        } else {
                            binding_keycode = (*binding_keycode).keycodes.tqe_next;
                        }
                    }
                    current_block_25 = 14136749492126903395;
                } else if (*bind).keycode != input_code as uint32_t {
                    current_block_25 = 2868539653012386629;
                } else {
                    let mut binding_keycode_0: *mut Binding_Keycode =
                        ::core::ptr::null_mut::<Binding_Keycode>();
                    binding_keycode_0 = (*bind).keycodes_head.tqh_first;
                    while !binding_keycode_0.is_null() {
                        let modifiers_mask_0: uint32_t =
                            (*binding_keycode_0).modifiers as uint32_t & 0xffff as uint32_t;
                        let mods_match_0: bool = modifiers_mask_0 == modifiers_state;
                        debuglog(
                            b"%s:%s:%d - binding_keycode->modifiers = %d, modifiers_mask = %d, modifiers_state = %d, mods_match = %s\n\0"
                                as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b"../src/bindings.c\0" as *const u8
                                as *const ::core::ffi::c_char,
                            b"get_binding\0" as *const u8 as *const ::core::ffi::c_char,
                            270 as ::core::ffi::c_int,
                            (*binding_keycode_0).modifiers,
                            modifiers_mask_0,
                            modifiers_state,
                            if mods_match_0 as ::core::ffi::c_int != 0 {
                                b"yes\0" as *const u8 as *const ::core::ffi::c_char
                            } else {
                                b"no\0" as *const u8 as *const ::core::ffi::c_char
                            },
                        );
                        if mods_match_0 as ::core::ffi::c_int != 0
                            || (*bind).release as ::core::ffi::c_uint
                                == B_UPON_KEYRELEASE_IGNORE_MODS as ::core::ffi::c_int
                                    as ::core::ffi::c_uint
                                && is_release as ::core::ffi::c_int != 0
                        {
                            found_keycode = true_0 != 0;
                            break;
                        } else {
                            binding_keycode_0 = (*binding_keycode_0).keycodes.tqe_next;
                        }
                    }
                    current_block_25 = 14136749492126903395;
                }
                match current_block_25 {
                    2868539653012386629 => {}
                    _ => {
                        if found_keycode {
                            if (*bind).release as ::core::ffi::c_uint
                                == B_UPON_KEYRELEASE as ::core::ffi::c_int as ::core::ffi::c_uint
                                && !is_release
                            {
                                (*bind).release = B_UPON_KEYRELEASE_IGNORE_MODS;
                                debuglog(
                                    b"%s:%s:%d - marked bind %p as B_UPON_KEYRELEASE_IGNORE_MODS\n\0"
                                        as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    b"../src/bindings.c\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    b"get_binding\0" as *const u8 as *const ::core::ffi::c_char,
                                    287 as ::core::ffi::c_int,
                                    bind,
                                );
                                if !result.is_null() {
                                    break;
                                }
                            } else if !((*bind).release as ::core::ffi::c_uint
                                == B_UPON_KEYPRESS as ::core::ffi::c_int as ::core::ffi::c_uint
                                && is_release as ::core::ffi::c_int != 0)
                            {
                                if is_release {
                                    return bind;
                                } else if result.is_null() {
                                    result = bind;
                                }
                            }
                        }
                    }
                }
            }
        }
        bind = (*bind).bindings.tqe_next as *mut Binding;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn get_binding_from_xcb_event(
    mut event: *mut xcb_generic_event_t,
) -> *mut Binding {
    let is_release: bool = (*event).response_type as ::core::ffi::c_int == XCB_KEY_RELEASE
        || (*event).response_type as ::core::ffi::c_int == XCB_BUTTON_RELEASE;
    let input_type: input_type_t = (if (*event).response_type as ::core::ffi::c_int
        == XCB_BUTTON_RELEASE
        || (*event).response_type as ::core::ffi::c_int == XCB_BUTTON_PRESS
    {
        B_MOUSE as ::core::ffi::c_int
    } else {
        B_KEYBOARD as ::core::ffi::c_int
    }) as input_type_t;
    let event_state: uint16_t = (*(event as *mut xcb_key_press_event_t)).state;
    let event_detail: uint16_t = (*(event as *mut xcb_key_press_event_t)).detail as uint16_t;
    let mut state_filtered: i3_event_state_mask_t = (event_state as ::core::ffi::c_int
        & !(XCB_MOD_MASK_LOCK as ::core::ffi::c_int))
        as i3_event_state_mask_t;
    debuglog(
        b"%s:%s:%d - (removed capslock, state = 0x%x)\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/bindings.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"get_binding_from_xcb_event\0" as *const u8 as *const ::core::ffi::c_char,
        329 as ::core::ffi::c_int,
        state_filtered,
    );
    match (event_state as ::core::ffi::c_int & 0x6000 as ::core::ffi::c_int)
        >> 13 as ::core::ffi::c_int
    {
        0 => {
            state_filtered |= ((I3_XKB_GROUP_MASK_1 as ::core::ffi::c_int)
                << 16 as ::core::ffi::c_int) as i3_event_state_mask_t;
        }
        1 => {
            state_filtered |= ((I3_XKB_GROUP_MASK_2 as ::core::ffi::c_int)
                << 16 as ::core::ffi::c_int) as i3_event_state_mask_t;
        }
        2 => {
            state_filtered |= ((I3_XKB_GROUP_MASK_3 as ::core::ffi::c_int)
                << 16 as ::core::ffi::c_int) as i3_event_state_mask_t;
        }
        3 => {
            state_filtered |= ((I3_XKB_GROUP_MASK_4 as ::core::ffi::c_int)
                << 16 as ::core::ffi::c_int) as i3_event_state_mask_t;
        }
        _ => {}
    }
    state_filtered &= !(0x6000 as ::core::ffi::c_int) as i3_event_state_mask_t;
    debuglog(
        b"%s:%s:%d - (transformed keyboard group, state = 0x%x)\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/bindings.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"get_binding_from_xcb_event\0" as *const u8 as *const ::core::ffi::c_char,
        354 as ::core::ffi::c_int,
        state_filtered,
    );
    return get_binding(state_filtered, is_release, event_detail, input_type);
}
unsafe extern "C" fn add_keycode_if_matches(
    mut keymap: *mut xkb_keymap,
    mut key: xkb_keycode_t,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut resolving: *const resolve = data as *const resolve;
    let mut numlock_state: *mut xkb_state = (*resolving).xkb_state_numlock;
    let mut sym: xkb_keysym_t = xkb_state_key_get_one_sym((*resolving).xkb_state, key);
    if sym != (*resolving).keysym {
        let layout: xkb_layout_index_t =
            xkb_state_key_get_layout((*resolving).xkb_state, key) as xkb_layout_index_t;
        if layout == XKB_LAYOUT_INVALID as xkb_layout_index_t {
            return;
        }
        if xkb_state_key_get_level((*resolving).xkb_state, key, layout) > 1 as xkb_level_index_t {
            return;
        }
        if sym >= XKB_KEY_KP_Space as xkb_keysym_t && sym <= XKB_KEY_KP_Equal as xkb_keysym_t {
            return;
        }
        numlock_state = (*resolving).xkb_state_numlock_no_shift;
        sym = xkb_state_key_get_one_sym((*resolving).xkb_state_no_shift, key);
        if sym != (*resolving).keysym {
            return;
        }
    }
    let mut bind: *mut Binding = (*resolving).bind;
    let mut binding_keycode: *mut Binding_Keycode =
        smalloc(::core::mem::size_of::<Binding_Keycode>() as size_t) as *mut Binding_Keycode;
    (*binding_keycode).modifiers = (*bind).event_state_mask;
    (*binding_keycode).keycode = key as xcb_keycode_t;
    (*binding_keycode).keycodes.tqe_next = ::core::ptr::null_mut::<Binding_Keycode>();
    (*binding_keycode).keycodes.tqe_prev = (*bind).keycodes_head.tqh_last;
    *(*bind).keycodes_head.tqh_last = binding_keycode;
    (*bind).keycodes_head.tqh_last = &raw mut (*binding_keycode).keycodes.tqe_next;
    let mut binding_keycode_0: *mut Binding_Keycode =
        smalloc(::core::mem::size_of::<Binding_Keycode>() as size_t) as *mut Binding_Keycode;
    (*binding_keycode_0).modifiers =
        (*bind).event_state_mask | XCB_MOD_MASK_LOCK as ::core::ffi::c_int as i3_event_state_mask_t;
    (*binding_keycode_0).keycode = key as xcb_keycode_t;
    (*binding_keycode_0).keycodes.tqe_next = ::core::ptr::null_mut::<Binding_Keycode>();
    (*binding_keycode_0).keycodes.tqe_prev = (*bind).keycodes_head.tqh_last;
    *(*bind).keycodes_head.tqh_last = binding_keycode_0;
    (*bind).keycodes_head.tqh_last = &raw mut (*binding_keycode_0).keycodes.tqe_next;
    if (*bind).event_state_mask & xcb_numlock_mask as i3_event_state_mask_t
        != xcb_numlock_mask as i3_event_state_mask_t
    {
        let mut sym_numlock: xkb_keysym_t = xkb_state_key_get_one_sym(numlock_state, key);
        if sym_numlock == (*resolving).keysym {
            let mut binding_keycode_1: *mut Binding_Keycode =
                smalloc(::core::mem::size_of::<Binding_Keycode>() as size_t)
                    as *mut Binding_Keycode;
            (*binding_keycode_1).modifiers =
                (*bind).event_state_mask | xcb_numlock_mask as i3_event_state_mask_t;
            (*binding_keycode_1).keycode = key as xcb_keycode_t;
            (*binding_keycode_1).keycodes.tqe_next = ::core::ptr::null_mut::<Binding_Keycode>();
            (*binding_keycode_1).keycodes.tqe_prev = (*bind).keycodes_head.tqh_last;
            *(*bind).keycodes_head.tqh_last = binding_keycode_1;
            (*bind).keycodes_head.tqh_last = &raw mut (*binding_keycode_1).keycodes.tqe_next;
            let mut binding_keycode_2: *mut Binding_Keycode =
                smalloc(::core::mem::size_of::<Binding_Keycode>() as size_t)
                    as *mut Binding_Keycode;
            (*binding_keycode_2).modifiers = (*bind).event_state_mask
                | xcb_numlock_mask as i3_event_state_mask_t
                | XCB_MOD_MASK_LOCK as ::core::ffi::c_int as i3_event_state_mask_t;
            (*binding_keycode_2).keycode = key as xcb_keycode_t;
            (*binding_keycode_2).keycodes.tqe_next = ::core::ptr::null_mut::<Binding_Keycode>();
            (*binding_keycode_2).keycodes.tqe_prev = (*bind).keycodes_head.tqh_last;
            *(*bind).keycodes_head.tqh_last = binding_keycode_2;
            (*bind).keycodes_head.tqh_last = &raw mut (*binding_keycode_2).keycodes.tqe_next;
        } else {
            debuglog(
                b"%s:%s:%d - Skipping automatic numlock fallback, key %d resolves to 0x%x with numlock\n\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"../src/bindings.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"add_keycode_if_matches\0" as *const u8 as *const ::core::ffi::c_char,
                440 as ::core::ffi::c_int,
                key,
                sym_numlock,
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn translate_keysyms() {
    let mut bind: *mut Binding = ::core::ptr::null_mut::<Binding>();
    let mut dummy_state: *mut xkb_state = ::core::ptr::null_mut::<xkb_state>();
    let mut dummy_state_no_shift: *mut xkb_state = ::core::ptr::null_mut::<xkb_state>();
    let mut dummy_state_numlock: *mut xkb_state = ::core::ptr::null_mut::<xkb_state>();
    let mut dummy_state_numlock_no_shift: *mut xkb_state = ::core::ptr::null_mut::<xkb_state>();
    let mut has_errors: bool = false_0 != 0;
    dummy_state = xkb_state_new(xkb_keymap);
    if dummy_state.is_null()
        || {
            dummy_state_no_shift = xkb_state_new(xkb_keymap);
            dummy_state_no_shift.is_null()
        }
        || {
            dummy_state_numlock = xkb_state_new(xkb_keymap);
            dummy_state_numlock.is_null()
        }
        || {
            dummy_state_numlock_no_shift = xkb_state_new(xkb_keymap);
            dummy_state_numlock_no_shift.is_null()
        }
    {
        errorlog(
            b"ERROR: Could not create XKB state, cannot translate keysyms.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    } else {
        bind = ::core::ptr::null_mut::<Binding>();
        bind = (*bindings).tqh_first as *mut Binding;
        while !bind.is_null() {
            if (*bind).input_type as ::core::ffi::c_uint
                == B_MOUSE as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                let mut button: ::core::ffi::c_long = 0;
                if !parse_long(
                    (*bind).symbol.offset(
                        (::core::mem::size_of::<[::core::ffi::c_char; 7]>() as usize)
                            .wrapping_sub(1 as usize) as isize,
                    ),
                    &raw mut button,
                    10 as ::core::ffi::c_int,
                ) {
                    errorlog(
                        b"ERROR: Could not translate string to button: \"%s\"\n\0" as *const u8
                            as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        (*bind).symbol,
                    );
                }
                let mut key: xcb_keycode_t = button as xcb_keycode_t;
                (*bind).keycode = key as uint32_t;
                debuglog(
                    b"%s:%s:%d - Binding Mouse button, Keycode = %d\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    b"../src/bindings.c\0" as *const u8 as *const ::core::ffi::c_char,
                    b"translate_keysyms\0" as *const u8 as *const ::core::ffi::c_char,
                    474 as ::core::ffi::c_int,
                    key as ::core::ffi::c_int,
                );
            }
            let mut group: xkb_layout_index_t =
                XCB_XKB_GROUP_1 as ::core::ffi::c_int as xkb_layout_index_t;
            if (*bind).event_state_mask >> 16 as ::core::ffi::c_int
                & I3_XKB_GROUP_MASK_2 as ::core::ffi::c_int as i3_event_state_mask_t
                != 0
            {
                group = XCB_XKB_GROUP_2 as ::core::ffi::c_int as xkb_layout_index_t;
            } else if (*bind).event_state_mask >> 16 as ::core::ffi::c_int
                & I3_XKB_GROUP_MASK_3 as ::core::ffi::c_int as i3_event_state_mask_t
                != 0
            {
                group = XCB_XKB_GROUP_3 as ::core::ffi::c_int as xkb_layout_index_t;
            } else if (*bind).event_state_mask >> 16 as ::core::ffi::c_int
                & I3_XKB_GROUP_MASK_4 as ::core::ffi::c_int as i3_event_state_mask_t
                != 0
            {
                group = XCB_XKB_GROUP_4 as ::core::ffi::c_int as xkb_layout_index_t;
            }
            debuglog(
                b"%s:%s:%d - Binding %p group = %d, event_state_mask = %d, &2 = %s, &3 = %s, &4 = %s\n\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"../src/bindings.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"translate_keysyms\0" as *const u8 as *const ::core::ffi::c_char,
                492 as ::core::ffi::c_int,
                bind,
                group,
                (*bind).event_state_mask,
                if (*bind).event_state_mask
                    & I3_XKB_GROUP_MASK_2 as ::core::ffi::c_int as i3_event_state_mask_t
                    != 0
                {
                    b"yes\0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"no\0" as *const u8 as *const ::core::ffi::c_char
                },
                if (*bind).event_state_mask
                    & I3_XKB_GROUP_MASK_3 as ::core::ffi::c_int as i3_event_state_mask_t
                    != 0
                {
                    b"yes\0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"no\0" as *const u8 as *const ::core::ffi::c_char
                },
                if (*bind).event_state_mask
                    & I3_XKB_GROUP_MASK_4 as ::core::ffi::c_int as i3_event_state_mask_t
                    != 0
                {
                    b"yes\0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"no\0" as *const u8 as *const ::core::ffi::c_char
                },
            );
            xkb_state_update_mask(
                dummy_state,
                (*bind).event_state_mask as xkb_mod_mask_t & 0x1fff as xkb_mod_mask_t,
                0 as xkb_mod_mask_t,
                0 as xkb_mod_mask_t,
                0 as xkb_layout_index_t,
                0 as xkb_layout_index_t,
                group,
            );
            xkb_state_update_mask(
                dummy_state_no_shift,
                (*bind).event_state_mask as xkb_mod_mask_t & 0x1fff as xkb_mod_mask_t
                    ^ XCB_KEY_BUT_MASK_SHIFT as ::core::ffi::c_int as xkb_mod_mask_t,
                0 as xkb_mod_mask_t,
                0 as xkb_mod_mask_t,
                0 as xkb_layout_index_t,
                0 as xkb_layout_index_t,
                group,
            );
            xkb_state_update_mask(
                dummy_state_numlock,
                (*bind).event_state_mask as xkb_mod_mask_t & 0x1fff as xkb_mod_mask_t
                    | xcb_numlock_mask as xkb_mod_mask_t,
                0 as xkb_mod_mask_t,
                0 as xkb_mod_mask_t,
                0 as xkb_layout_index_t,
                0 as xkb_layout_index_t,
                group,
            );
            xkb_state_update_mask(
                dummy_state_numlock_no_shift,
                ((*bind).event_state_mask as xkb_mod_mask_t & 0x1fff as xkb_mod_mask_t
                    | xcb_numlock_mask as xkb_mod_mask_t)
                    ^ XCB_KEY_BUT_MASK_SHIFT as ::core::ffi::c_int as xkb_mod_mask_t,
                0 as xkb_mod_mask_t,
                0 as xkb_mod_mask_t,
                0 as xkb_layout_index_t,
                0 as xkb_layout_index_t,
                group,
            );
            if (*bind).keycode > 0 as uint32_t {
                while !(*bind).keycodes_head.tqh_first.is_null() {
                    let mut first: *mut Binding_Keycode = (*bind).keycodes_head.tqh_first;
                    if !(*first).keycodes.tqe_next.is_null() {
                        (*(*first).keycodes.tqe_next).keycodes.tqe_prev =
                            (*first).keycodes.tqe_prev;
                    } else {
                        (*bind).keycodes_head.tqh_last = (*first).keycodes.tqe_prev;
                    }
                    *(*first).keycodes.tqe_prev = (*first).keycodes.tqe_next;
                    free(first as *mut ::core::ffi::c_void);
                    first = ::core::ptr::null_mut::<Binding_Keycode>();
                }
                let mut binding_keycode: *mut Binding_Keycode =
                    smalloc(::core::mem::size_of::<Binding_Keycode>() as size_t)
                        as *mut Binding_Keycode;
                (*binding_keycode).modifiers = (*bind).event_state_mask;
                (*binding_keycode).keycode = (*bind).keycode as xcb_keycode_t;
                (*binding_keycode).keycodes.tqe_next = ::core::ptr::null_mut::<Binding_Keycode>();
                (*binding_keycode).keycodes.tqe_prev = (*bind).keycodes_head.tqh_last;
                *(*bind).keycodes_head.tqh_last = binding_keycode;
                (*bind).keycodes_head.tqh_last = &raw mut (*binding_keycode).keycodes.tqe_next;
                let mut binding_keycode_0: *mut Binding_Keycode =
                    smalloc(::core::mem::size_of::<Binding_Keycode>() as size_t)
                        as *mut Binding_Keycode;
                (*binding_keycode_0).modifiers = (*bind).event_state_mask
                    | XCB_MOD_MASK_LOCK as ::core::ffi::c_int as i3_event_state_mask_t;
                (*binding_keycode_0).keycode = (*bind).keycode as xcb_keycode_t;
                (*binding_keycode_0).keycodes.tqe_next = ::core::ptr::null_mut::<Binding_Keycode>();
                (*binding_keycode_0).keycodes.tqe_prev = (*bind).keycodes_head.tqh_last;
                *(*bind).keycodes_head.tqh_last = binding_keycode_0;
                (*bind).keycodes_head.tqh_last = &raw mut (*binding_keycode_0).keycodes.tqe_next;
                if (*bind).event_state_mask & xcb_numlock_mask as i3_event_state_mask_t
                    != xcb_numlock_mask as i3_event_state_mask_t
                {
                    let mut sym: xkb_keysym_t =
                        xkb_state_key_get_one_sym(dummy_state, (*bind).keycode as xkb_keycode_t);
                    let mut sym_numlock: xkb_keysym_t = xkb_state_key_get_one_sym(
                        dummy_state_numlock,
                        (*bind).keycode as xkb_keycode_t,
                    );
                    if sym == sym_numlock {
                        let mut binding_keycode_1: *mut Binding_Keycode =
                            smalloc(::core::mem::size_of::<Binding_Keycode>() as size_t)
                                as *mut Binding_Keycode;
                        (*binding_keycode_1).modifiers =
                            (*bind).event_state_mask | xcb_numlock_mask as i3_event_state_mask_t;
                        (*binding_keycode_1).keycode = (*bind).keycode as xcb_keycode_t;
                        (*binding_keycode_1).keycodes.tqe_next =
                            ::core::ptr::null_mut::<Binding_Keycode>();
                        (*binding_keycode_1).keycodes.tqe_prev = (*bind).keycodes_head.tqh_last;
                        *(*bind).keycodes_head.tqh_last = binding_keycode_1;
                        (*bind).keycodes_head.tqh_last =
                            &raw mut (*binding_keycode_1).keycodes.tqe_next;
                        let mut binding_keycode_2: *mut Binding_Keycode =
                            smalloc(::core::mem::size_of::<Binding_Keycode>() as size_t)
                                as *mut Binding_Keycode;
                        (*binding_keycode_2).modifiers = (*bind).event_state_mask
                            | xcb_numlock_mask as i3_event_state_mask_t
                            | XCB_MOD_MASK_LOCK as ::core::ffi::c_int as i3_event_state_mask_t;
                        (*binding_keycode_2).keycode = (*bind).keycode as xcb_keycode_t;
                        (*binding_keycode_2).keycodes.tqe_next =
                            ::core::ptr::null_mut::<Binding_Keycode>();
                        (*binding_keycode_2).keycodes.tqe_prev = (*bind).keycodes_head.tqh_last;
                        *(*bind).keycodes_head.tqh_last = binding_keycode_2;
                        (*bind).keycodes_head.tqh_last =
                            &raw mut (*binding_keycode_2).keycodes.tqe_next;
                    } else {
                        debuglog(
                            b"%s:%s:%d - Skipping automatic numlock fallback, key %d resolves to 0x%x with numlock\n\0"
                                as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b"../src/bindings.c\0" as *const u8
                                as *const ::core::ffi::c_char,
                            b"translate_keysyms\0" as *const u8
                                as *const ::core::ffi::c_char,
                            560 as ::core::ffi::c_int,
                            (*bind).keycode,
                            sym_numlock,
                        );
                    }
                }
            } else {
                let keysym: xkb_keysym_t =
                    xkb_keysym_from_name((*bind).symbol, XKB_KEYSYM_NO_FLAGS) as xkb_keysym_t;
                if keysym == XKB_KEY_NoSymbol as xkb_keysym_t {
                    errorlog(
                        b"ERROR: Could not translate string to key symbol: \"%s\"\n\0" as *const u8
                            as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        (*bind).symbol,
                    );
                } else {
                    let mut resolving: resolve = resolve {
                        bind: bind,
                        keysym: keysym,
                        xkb_state: dummy_state,
                        xkb_state_no_shift: dummy_state_no_shift,
                        xkb_state_numlock: dummy_state_numlock,
                        xkb_state_numlock_no_shift: dummy_state_numlock_no_shift,
                    };
                    while !(*bind).keycodes_head.tqh_first.is_null() {
                        let mut first_0: *mut Binding_Keycode = (*bind).keycodes_head.tqh_first;
                        if !(*first_0).keycodes.tqe_next.is_null() {
                            (*(*first_0).keycodes.tqe_next).keycodes.tqe_prev =
                                (*first_0).keycodes.tqe_prev;
                        } else {
                            (*bind).keycodes_head.tqh_last = (*first_0).keycodes.tqe_prev;
                        }
                        *(*first_0).keycodes.tqe_prev = (*first_0).keycodes.tqe_next;
                        free(first_0 as *mut ::core::ffi::c_void);
                        first_0 = ::core::ptr::null_mut::<Binding_Keycode>();
                    }
                    xkb_keymap_key_for_each(
                        xkb_keymap,
                        Some(
                            add_keycode_if_matches
                                as unsafe extern "C" fn(
                                    *mut xkb_keymap,
                                    xkb_keycode_t,
                                    *mut ::core::ffi::c_void,
                                ) -> (),
                        ),
                        &raw mut resolving as *mut ::core::ffi::c_void,
                    );
                    let mut keycodes: *mut ::core::ffi::c_char =
                        sstrdup(b"\0" as *const u8 as *const ::core::ffi::c_char);
                    let mut num_keycodes: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    let mut binding_keycode_3: *mut Binding_Keycode =
                        ::core::ptr::null_mut::<Binding_Keycode>();
                    binding_keycode_3 = (*bind).keycodes_head.tqh_first;
                    while !binding_keycode_3.is_null() {
                        let mut tmp: *mut ::core::ffi::c_char =
                            ::core::ptr::null_mut::<::core::ffi::c_char>();
                        sasprintf(
                            &raw mut tmp,
                            b"%s %d\0" as *const u8 as *const ::core::ffi::c_char,
                            keycodes,
                            (*binding_keycode_3).keycode as ::core::ffi::c_int,
                        );
                        free(keycodes as *mut ::core::ffi::c_void);
                        keycodes = tmp;
                        num_keycodes += 1;
                        let mut check: *mut Binding = ::core::ptr::null_mut::<Binding>();
                        check = (*bindings).tqh_first as *mut Binding;
                        while !check.is_null() {
                            if !(check == bind) {
                                if (*check).symbol.is_null() {
                                    if !((*check).keycode
                                        != (*binding_keycode_3).keycode as uint32_t
                                        || (*check).event_state_mask
                                            != (*binding_keycode_3).modifiers
                                        || (*check).release as ::core::ffi::c_uint
                                            != (*bind).release as ::core::ffi::c_uint)
                                    {
                                        has_errors = true_0 != 0;
                                        errorlog(
                                            b"ERROR: Duplicate keybinding in config file:\n  keysym = %s, keycode = %d, state_mask = 0x%x\n\0"
                                                as *const u8 as *const ::core::ffi::c_char
                                                as *mut ::core::ffi::c_char,
                                            (*bind).symbol,
                                            (*check).keycode,
                                            (*bind).event_state_mask,
                                        );
                                    }
                                }
                            }
                            check = (*check).bindings.tqe_next as *mut Binding;
                        }
                        binding_keycode_3 = (*binding_keycode_3).keycodes.tqe_next;
                    }
                    debuglog(
                        b"%s:%s:%d - state=0x%x, cfg=\"%s\", sym=0x%x \xE2\x86\x92 keycodes%s (%d)\n\0"
                            as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        b"../src/bindings.c\0" as *const u8
                            as *const ::core::ffi::c_char,
                        b"translate_keysyms\0" as *const u8
                            as *const ::core::ffi::c_char,
                        618 as ::core::ffi::c_int,
                        (*bind).event_state_mask,
                        (*bind).symbol,
                        keysym,
                        keycodes,
                        num_keycodes,
                    );
                    free(keycodes as *mut ::core::ffi::c_void);
                }
            }
            bind = (*bind).bindings.tqe_next as *mut Binding;
        }
    }
    xkb_state_unref(dummy_state);
    xkb_state_unref(dummy_state_no_shift);
    xkb_state_unref(dummy_state_numlock);
    xkb_state_unref(dummy_state_numlock_no_shift);
    if has_errors {
        start_config_error_nagbar(current_configpath, true_0 != 0);
    }
}
#[no_mangle]
pub unsafe extern "C" fn switch_mode(mut new_mode: *const ::core::ffi::c_char) {
    let mut mode: *mut Mode = ::core::ptr::null_mut::<Mode>();
    debuglog(
        b"%s:%s:%d - Switching to mode %s\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/bindings.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"switch_mode\0" as *const u8 as *const ::core::ffi::c_char,
        642 as ::core::ffi::c_int,
        new_mode,
    );
    mode = modes.slh_first as *mut Mode;
    while !mode.is_null() {
        if strcmp((*mode).name, new_mode) != 0 as ::core::ffi::c_int {
            mode = (*mode).modes.sle_next;
        } else {
            ungrab_all_keys(conn);
            bindings = (*mode).bindings as *mut bindings_head;
            current_binding_mode = (*mode).name;
            translate_keysyms();
            grab_all_keys(conn);
            regrab_all_buttons(conn);
            let mut bind: *mut Binding = ::core::ptr::null_mut::<Binding>();
            bind = (*bindings).tqh_first as *mut Binding;
            while !bind.is_null() {
                if (*bind).release as ::core::ffi::c_uint
                    == B_UPON_KEYRELEASE_IGNORE_MODS as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    (*bind).release = B_UPON_KEYRELEASE;
                }
                bind = (*bind).bindings.tqe_next as *mut Binding;
            }
            let mut event_msg: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut event_msg,
                b"{\"change\":\"%s\", \"pango_markup\":%s}\0" as *const u8
                    as *const ::core::ffi::c_char,
                (*mode).name,
                if (*mode).pango_markup as ::core::ffi::c_int != 0 {
                    b"true\0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"false\0" as *const u8 as *const ::core::ffi::c_char
                },
            );
            ipc_send_event(
                b"mode\0" as *const u8 as *const ::core::ffi::c_char,
                I3_IPC_EVENT_MODE as uint32_t,
                event_msg,
            );
            free(event_msg as *mut ::core::ffi::c_void);
            event_msg = ::core::ptr::null_mut::<::core::ffi::c_char>();
            return;
        }
    }
    errorlog(
        b"ERROR: Mode not found\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    );
}
unsafe extern "C" fn reorder_binding_cmp(
    mut a: *const ::core::ffi::c_void,
    mut b: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut first: *mut Binding = *(a as *mut *mut Binding);
    let mut second: *mut Binding = *(b as *mut *mut Binding);
    if (*first).event_state_mask < (*second).event_state_mask {
        return 1 as ::core::ffi::c_int;
    }
    if (*first).event_state_mask == (*second).event_state_mask {
        return 0 as ::core::ffi::c_int;
    }
    return -(1 as ::core::ffi::c_int);
}
unsafe extern "C" fn reorder_bindings_of_mode(mut mode: *mut Mode) {
    let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut current: *mut Binding = ::core::ptr::null_mut::<Binding>();
    current = (*(*mode).bindings).tqh_first as *mut Binding;
    while !current.is_null() {
        n += 1;
        current = (*current).bindings.tqe_next as *mut Binding;
    }
    let mut tmp: *mut *mut Binding = scalloc(
        n as size_t,
        ::core::mem::size_of::<*mut Binding>() as size_t,
    ) as *mut *mut Binding;
    n = 0 as ::core::ffi::c_int;
    current = (*(*mode).bindings).tqh_first as *mut Binding;
    while !current.is_null() {
        let fresh5 = n;
        n = n + 1;
        let ref mut fresh6 = *tmp.offset(fresh5 as isize);
        *fresh6 = current;
        current = (*current).bindings.tqe_next as *mut Binding;
    }
    qsort(
        tmp as *mut ::core::ffi::c_void,
        n as size_t,
        ::core::mem::size_of::<*mut Binding>() as size_t,
        Some(
            reorder_binding_cmp
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    );
    let mut reordered: *mut bindings_head = scalloc(
        1 as size_t,
        ::core::mem::size_of::<bindings_head>() as size_t,
    ) as *mut bindings_head;
    (*reordered).tqh_first = ::core::ptr::null_mut::<Binding>();
    (*reordered).tqh_last = &raw mut (*reordered).tqh_first;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < n {
        current = *tmp.offset(i as isize);
        if !(*current).bindings.tqe_next.is_null() {
            (*(*current).bindings.tqe_next).bindings.tqe_prev = (*current).bindings.tqe_prev;
        } else {
            (*(*mode).bindings).tqh_last = (*current).bindings.tqe_prev;
        }
        *(*current).bindings.tqe_prev = (*current).bindings.tqe_next;
        (*current).bindings.tqe_next = ::core::ptr::null_mut::<Binding>();
        (*current).bindings.tqe_prev = (*reordered).tqh_last;
        *(*reordered).tqh_last = current as *mut Binding;
        (*reordered).tqh_last = &raw mut (*current).bindings.tqe_next;
        i += 1;
    }
    free(tmp as *mut ::core::ffi::c_void);
    if (*(*mode).bindings).tqh_first
        == ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut Binding
    {
    } else {
        __assert_fail(
            b"TAILQ_EMPTY(mode->bindings)\0" as *const u8 as *const ::core::ffi::c_char,
            b"../src/bindings.c\0" as *const u8 as *const ::core::ffi::c_char,
            713 as ::core::ffi::c_uint,
            __ASSERT_FUNCTION.as_ptr(),
        );
    };
    free((*mode).bindings as *mut ::core::ffi::c_void);
    (*mode).bindings = reordered;
}
#[no_mangle]
pub unsafe extern "C" fn reorder_bindings() {
    let mut mode: *mut Mode = ::core::ptr::null_mut::<Mode>();
    mode = modes.slh_first as *mut Mode;
    while !mode.is_null() {
        let current_mode: bool = (*mode).bindings == bindings;
        reorder_bindings_of_mode(mode);
        if current_mode {
            bindings = (*mode).bindings as *mut bindings_head;
        }
        mode = (*mode).modes.sle_next;
    }
}
unsafe extern "C" fn binding_same_key(mut a: *mut Binding, mut b: *mut Binding) -> bool {
    if (*a).input_type as ::core::ffi::c_uint != (*b).input_type as ::core::ffi::c_uint {
        return false_0 != 0;
    }
    if (*a).symbol.is_null() && !(*b).symbol.is_null()
        || !(*a).symbol.is_null() && (*b).symbol.is_null()
    {
        return false_0 != 0;
    }
    if !(*a).symbol.is_null() && strcasecmp((*a).symbol, (*b).symbol) != 0 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    if (*a).keycode != (*b).keycode
        || (*a).event_state_mask != (*b).event_state_mask
        || (*a).release as ::core::ffi::c_uint != (*b).release as ::core::ffi::c_uint
    {
        return false_0 != 0;
    }
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn check_for_duplicate_bindings(mut context: *mut context) {
    let mut bind: *mut Binding = ::core::ptr::null_mut::<Binding>();
    let mut current: *mut Binding = ::core::ptr::null_mut::<Binding>();
    current = (*bindings).tqh_first as *mut Binding;
    while !current.is_null() {
        bind = (*bindings).tqh_first as *mut Binding;
        while !bind.is_null() {
            if bind == current {
                break;
            }
            if binding_same_key(bind, current) {
                (*context).has_errors = true_0 != 0;
                if (*current).keycode != 0 as uint32_t {
                    errorlog(
                        b"ERROR: Duplicate keybinding in config file:\n  state mask 0x%x with keycode %d, command \"%s\"\n\0"
                            as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        (*current).event_state_mask,
                        (*current).keycode,
                        (*current).command,
                    );
                } else {
                    errorlog(
                        b"ERROR: Duplicate keybinding in config file:\n  state mask 0x%x with keysym %s, command \"%s\"\n\0"
                            as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        (*current).event_state_mask,
                        (*current).symbol,
                        (*current).command,
                    );
                }
            }
            bind = (*bind).bindings.tqe_next as *mut Binding;
        }
        current = (*current).bindings.tqe_next as *mut Binding;
    }
}
unsafe extern "C" fn binding_copy(mut bind: *mut Binding) -> *mut Binding {
    let mut ret: *mut Binding =
        smalloc(::core::mem::size_of::<Binding>() as size_t) as *mut Binding;
    *ret = *bind;
    if !(*bind).symbol.is_null() {
        (*ret).symbol = sstrdup((*bind).symbol);
    }
    if !(*bind).command.is_null() {
        (*ret).command = sstrdup((*bind).command);
    }
    (*ret).keycodes_head.tqh_first = ::core::ptr::null_mut::<Binding_Keycode>();
    (*ret).keycodes_head.tqh_last = &raw mut (*ret).keycodes_head.tqh_first;
    let mut binding_keycode: *mut Binding_Keycode = ::core::ptr::null_mut::<Binding_Keycode>();
    binding_keycode = (*bind).keycodes_head.tqh_first;
    while !binding_keycode.is_null() {
        let mut ret_binding_keycode: *mut Binding_Keycode =
            smalloc(::core::mem::size_of::<Binding_Keycode>() as size_t) as *mut Binding_Keycode;
        *ret_binding_keycode = *binding_keycode;
        (*ret_binding_keycode).keycodes.tqe_next = ::core::ptr::null_mut::<Binding_Keycode>();
        (*ret_binding_keycode).keycodes.tqe_prev = (*ret).keycodes_head.tqh_last;
        *(*ret).keycodes_head.tqh_last = ret_binding_keycode;
        (*ret).keycodes_head.tqh_last = &raw mut (*ret_binding_keycode).keycodes.tqe_next;
        binding_keycode = (*binding_keycode).keycodes.tqe_next;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn binding_free(mut bind: *mut Binding) {
    if bind.is_null() {
        return;
    }
    while !(*bind).keycodes_head.tqh_first.is_null() {
        let mut first: *mut Binding_Keycode = (*bind).keycodes_head.tqh_first;
        if !(*first).keycodes.tqe_next.is_null() {
            (*(*first).keycodes.tqe_next).keycodes.tqe_prev = (*first).keycodes.tqe_prev;
        } else {
            (*bind).keycodes_head.tqh_last = (*first).keycodes.tqe_prev;
        }
        *(*first).keycodes.tqe_prev = (*first).keycodes.tqe_next;
        free(first as *mut ::core::ffi::c_void);
        first = ::core::ptr::null_mut::<Binding_Keycode>();
    }
    free((*bind).symbol as *mut ::core::ffi::c_void);
    (*bind).symbol = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free((*bind).command as *mut ::core::ffi::c_void);
    (*bind).command = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free(bind as *mut ::core::ffi::c_void);
    bind = ::core::ptr::null_mut::<Binding>();
}
#[no_mangle]
pub unsafe extern "C" fn run_binding(
    mut bind: *mut Binding,
    mut con: *mut Con,
) -> *mut CommandResult {
    let mut command: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if con.is_null() {
        command = sstrdup((*bind).command);
    } else {
        sasprintf(
            &raw mut command,
            b"[con_id=\"%p\"] %s\0" as *const u8 as *const ::core::ffi::c_char,
            con,
            (*bind).command,
        );
    }
    let mut bind_cp: *mut Binding = binding_copy(bind);
    let mut modename: *mut ::core::ffi::c_char = sstrdup(current_binding_mode);
    let mut result: *mut CommandResult = parse_command(
        command,
        ::core::ptr::null_mut::<yajl_gen_t>(),
        ::core::ptr::null_mut::<ipc_client>(),
    );
    free(command as *mut ::core::ffi::c_void);
    if (*result).needs_tree_render {
        tree_render();
    }
    if (*result).parse_error {
        let mut pageraction: *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<::core::ffi::c_char>();
        sasprintf(
            &raw mut pageraction,
            b"i3-sensible-pager \"%s\"\n\0" as *const u8 as *const ::core::ffi::c_char,
            errorfilename,
        );
        let mut argv: [*mut ::core::ffi::c_char; 11] = [
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            b"-f\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            config.font.pattern,
            b"-t\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"error\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"-m\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"The configured command for this shortcut could not be run successfully.\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"-b\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"show errors\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            pageraction,
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
        ];
        start_nagbar(
            &raw mut command_error_nagbar_pid,
            &raw mut argv as *mut *mut ::core::ffi::c_char,
        );
        free(pageraction as *mut ::core::ffi::c_void);
    }
    ipc_send_binding_event(
        b"run\0" as *const u8 as *const ::core::ffi::c_char,
        bind_cp,
        modename,
    );
    free(modename as *mut ::core::ffi::c_void);
    modename = ::core::ptr::null_mut::<::core::ffi::c_char>();
    binding_free(bind_cp);
    return result;
}
unsafe extern "C" fn fill_rmlvo_from_root(
    mut xkb_names: *mut xkb_rule_names,
) -> ::core::ffi::c_int {
    let mut content_max_words: size_t = 256 as size_t;
    let mut atom_reply: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        xcb_intern_atom(
            conn,
            0 as uint8_t,
            strlen(b"_XKB_RULES_NAMES\0" as *const u8 as *const ::core::ffi::c_char) as uint16_t,
            b"_XKB_RULES_NAMES\0" as *const u8 as *const ::core::ffi::c_char,
        ),
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if atom_reply.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    let mut prop_cookie: xcb_get_property_cookie_t = xcb_get_property_unchecked(
        conn,
        false_0 as uint8_t,
        root,
        (*atom_reply).atom,
        XCB_GET_PROPERTY_TYPE_ANY as ::core::ffi::c_int as xcb_atom_t,
        0 as uint32_t,
        content_max_words as uint32_t,
    );
    let mut prop_reply: *mut xcb_get_property_reply_t = xcb_get_property_reply(
        conn,
        prop_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if prop_reply.is_null() {
        free(atom_reply as *mut ::core::ffi::c_void);
        return -(1 as ::core::ffi::c_int);
    }
    if xcb_get_property_value_length(prop_reply) > 0 as ::core::ffi::c_int
        && (*prop_reply).bytes_after > 0 as uint32_t
    {
        content_max_words = (content_max_words as ::core::ffi::c_double
            + ceil((*prop_reply).bytes_after as ::core::ffi::c_double / 4.0f64))
            as size_t;
        free(prop_reply as *mut ::core::ffi::c_void);
        prop_cookie = xcb_get_property_unchecked(
            conn,
            false_0 as uint8_t,
            root,
            (*atom_reply).atom,
            XCB_GET_PROPERTY_TYPE_ANY as ::core::ffi::c_int as xcb_atom_t,
            0 as uint32_t,
            content_max_words as uint32_t,
        );
        prop_reply = xcb_get_property_reply(
            conn,
            prop_cookie,
            ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
        );
        if prop_reply.is_null() {
            free(atom_reply as *mut ::core::ffi::c_void);
            return -(1 as ::core::ffi::c_int);
        }
    }
    if xcb_get_property_value_length(prop_reply) == 0 as ::core::ffi::c_int {
        free(atom_reply as *mut ::core::ffi::c_void);
        free(prop_reply as *mut ::core::ffi::c_void);
        return -(1 as ::core::ffi::c_int);
    }
    let mut walk: *const ::core::ffi::c_char =
        xcb_get_property_value(prop_reply) as *const ::core::ffi::c_char;
    let mut remaining: ::core::ffi::c_int = xcb_get_property_value_length(prop_reply);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < 5 as ::core::ffi::c_int && remaining > 0 as ::core::ffi::c_int {
        let len: ::core::ffi::c_int = strnlen(walk, remaining as size_t) as ::core::ffi::c_int;
        match i {
            0 => {
                sasprintf(
                    &raw mut (*xkb_names).rules as *mut *mut ::core::ffi::c_char,
                    b"%.*s\0" as *const u8 as *const ::core::ffi::c_char,
                    len,
                    walk,
                );
            }
            1 => {
                sasprintf(
                    &raw mut (*xkb_names).model as *mut *mut ::core::ffi::c_char,
                    b"%.*s\0" as *const u8 as *const ::core::ffi::c_char,
                    len,
                    walk,
                );
            }
            2 => {
                sasprintf(
                    &raw mut (*xkb_names).layout as *mut *mut ::core::ffi::c_char,
                    b"%.*s\0" as *const u8 as *const ::core::ffi::c_char,
                    len,
                    walk,
                );
            }
            3 => {
                sasprintf(
                    &raw mut (*xkb_names).variant as *mut *mut ::core::ffi::c_char,
                    b"%.*s\0" as *const u8 as *const ::core::ffi::c_char,
                    len,
                    walk,
                );
            }
            4 => {
                sasprintf(
                    &raw mut (*xkb_names).options as *mut *mut ::core::ffi::c_char,
                    b"%.*s\0" as *const u8 as *const ::core::ffi::c_char,
                    len,
                    walk,
                );
            }
            _ => {}
        }
        debuglog(
            b"%s:%s:%d - component %d of _XKB_RULES_NAMES is \"%.*s\"\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/bindings.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"fill_rmlvo_from_root\0" as *const u8 as *const ::core::ffi::c_char,
            966 as ::core::ffi::c_int,
            i,
            len,
            walk,
        );
        walk = walk.offset((len + 1 as ::core::ffi::c_int) as isize);
        remaining -= len + 1 as ::core::ffi::c_int;
        i += 1;
    }
    free(atom_reply as *mut ::core::ffi::c_void);
    free(prop_reply as *mut ::core::ffi::c_void);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn load_keymap() -> bool {
    if xkb_context.is_null() {
        xkb_context = xkb_context_new(XKB_CONTEXT_NO_FLAGS);
        if xkb_context.is_null() {
            errorlog(
                b"ERROR: Could not create xkbcommon context\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
            return false_0 != 0;
        }
    }
    let mut new_keymap: *mut xkb_keymap = ::core::ptr::null_mut::<xkb_keymap>();
    let mut device_id: int32_t = 0;
    if xkb_supported as ::core::ffi::c_int != 0 && {
        device_id = xkb_x11_get_core_keyboard_device_id(conn);
        device_id > -(1 as int32_t)
    } {
        new_keymap = xkb_x11_keymap_new_from_device(
            xkb_context,
            conn,
            device_id,
            XKB_KEYMAP_COMPILE_NO_FLAGS,
        );
        if new_keymap.is_null() {
            errorlog(
                b"ERROR: xkb_x11_keymap_new_from_device failed\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
            return false_0 != 0;
        }
    } else {
        verboselog(
            b"No XKB / core keyboard device? Assembling keymap from local RMLVO.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        let mut names: xkb_rule_names = xkb_rule_names {
            rules: ::core::ptr::null::<::core::ffi::c_char>(),
            model: ::core::ptr::null::<::core::ffi::c_char>(),
            layout: ::core::ptr::null::<::core::ffi::c_char>(),
            variant: ::core::ptr::null::<::core::ffi::c_char>(),
            options: ::core::ptr::null::<::core::ffi::c_char>(),
        };
        if fill_rmlvo_from_root(&raw mut names) == -(1 as ::core::ffi::c_int) {
            errorlog(
                b"ERROR: Could not get _XKB_RULES_NAMES atom from root window, falling back to defaults.\n\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
        }
        new_keymap =
            xkb_keymap_new_from_names(xkb_context, &raw mut names, XKB_KEYMAP_COMPILE_NO_FLAGS);
        free(names.rules as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void);
        free(names.model as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void);
        free(names.layout as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void);
        free(names.variant as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void);
        free(names.options as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void);
        if new_keymap.is_null() {
            errorlog(
                b"ERROR: xkb_keymap_new_from_names failed\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
            return false_0 != 0;
        }
    }
    xkb_keymap_unref(xkb_keymap);
    xkb_keymap = new_keymap;
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn bindings_get_buttons_to_grab() -> *mut ::core::ffi::c_int {
    let num_max: ::core::ffi::c_int = 25 as ::core::ffi::c_int;
    let vla = num_max as usize;
    let mut buffer: Vec<::core::ffi::c_int> = ::std::vec::from_elem(0, vla);
    let mut num: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let fresh0 = num;
    num = num + 1;
    *buffer.as_mut_ptr().offset(fresh0 as isize) = 1 as ::core::ffi::c_int;
    let fresh1 = num;
    num = num + 1;
    *buffer.as_mut_ptr().offset(fresh1 as isize) = 2 as ::core::ffi::c_int;
    let fresh2 = num;
    num = num + 1;
    *buffer.as_mut_ptr().offset(fresh2 as isize) = 3 as ::core::ffi::c_int;
    let mut bind: *mut Binding = ::core::ptr::null_mut::<Binding>();
    bind = (*bindings).tqh_first as *mut Binding;
    while !bind.is_null() {
        if num + 1 as ::core::ffi::c_int == num_max {
            break;
        }
        if !((*bind).input_type as ::core::ffi::c_uint
            != B_MOUSE as ::core::ffi::c_int as ::core::ffi::c_uint
            || !(*bind).whole_window)
        {
            let mut button: ::core::ffi::c_long = 0;
            if !parse_long(
                (*bind).symbol.offset(
                    (::core::mem::size_of::<[::core::ffi::c_char; 7]>() as usize)
                        .wrapping_sub(1 as usize) as isize,
                ),
                &raw mut button,
                10 as ::core::ffi::c_int,
            ) {
                errorlog(
                    b"ERROR: Could not parse button number, skipping this binding. Please report this bug in i3.\n\0"
                        as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            } else {
                let mut exists: bool = false_0 != 0;
                let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while i < num {
                    if *buffer.as_mut_ptr().offset(i as isize) as ::core::ffi::c_long == button {
                        exists = true_0 != 0;
                        break;
                    } else {
                        i += 1;
                    }
                }
                if !exists {
                    let fresh3 = num;
                    num = num + 1;
                    *buffer.as_mut_ptr().offset(fresh3 as isize) = button as ::core::ffi::c_int;
                }
            }
        }
        bind = (*bind).bindings.tqe_next as *mut Binding;
    }
    let fresh4 = num;
    num = num + 1;
    *buffer.as_mut_ptr().offset(fresh4 as isize) = 0 as ::core::ffi::c_int;
    let mut buttons: *mut ::core::ffi::c_int = scalloc(
        num as size_t,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
    ) as *mut ::core::ffi::c_int;
    memcpy(
        buttons as *mut ::core::ffi::c_void,
        buffer.as_mut_ptr() as *const ::core::ffi::c_void,
        (num as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
    );
    return buttons;
}
