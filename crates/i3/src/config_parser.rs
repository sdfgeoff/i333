/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * config_parser.c: handwritten parser to parse configuration directives.
 *
 * See also src/commands_parser.c for rationale on why we use a custom parser.
 *
 * This parser works VERY MUCH like src/commands_parser.c, so read that first.
 * The differences are:
 *
 * 1. config_parser supports the 'number' token type (in addition to 'word' and
 *    'string'). Numbers are referred to using &num (like $str).
 *
 * 2. Criteria are not executed immediately, they are just stored.
 *
 * 3. config_parser recognizes \n and \r as 'end' token, while commands_parser
 *    ignores them.
 *
 * 4. config_parser skips the current line on invalid inputs and follows the
 *    nearest <error> token.
 *
 */
// Mechanical file-by-file Rust translation of src/config_parser.c.
// Generated with C2Rust 0.22.1; C ABI and original ownership retained.
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type xcb_connection_t;
    pub type _PangoFontDescription;
    pub type _cairo;
    pub type _cairo_surface;
    pub type _i3String;
    pub type pcre2_real_code_8;
    pub type ev_loop;
    pub type xcb_xrm_database_t;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn __assert_single_arg(_: bool) -> bool;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fdopen(__fd: ::core::ffi::c_int, __modes: *const ::core::ffi::c_char) -> *mut FILE;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn sscanf(
        __s: *const ::core::ffi::c_char,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn fgets(
        __s: *mut ::core::ffi::c_char,
        __n: ::core::ffi::c_int,
        __stream: *mut FILE,
    ) -> *mut ::core::ffi::c_char;
    fn fread(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    fn rewind(__stream: *mut FILE);
    fn feof(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fstat(__fd: ::core::ffi::c_int, __buf: *mut stat) -> ::core::ffi::c_int;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn chdir(__path: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn getcwd(__buf: *mut ::core::ffi::c_char, __size: size_t) -> *mut ::core::ffi::c_char;
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
    static mut i3_version: *const ::core::ffi::c_char;
    static mut config: Config;
    static mut errorfilename: *mut ::core::ffi::c_char;
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
    fn check_for_duplicate_bindings(context: *mut context);
    fn cfg_criteria_init(
        current_match: *mut Match,
        result: *mut ConfigResultIR,
        _state: ::core::ffi::c_int,
    );
    fn cfg_criteria_add(
        current_match: *mut Match,
        result: *mut ConfigResultIR,
        ctype: *const ::core::ffi::c_char,
        cvalue: *const ::core::ffi::c_char,
    );
    fn cfg_criteria_pop_state(current_match: *mut Match, result: *mut ConfigResultIR);
    fn cfg_include(
        current_match: *mut Match,
        result: *mut ConfigResultIR,
        pattern: *const ::core::ffi::c_char,
    );
    fn cfg_font(
        current_match: *mut Match,
        result: *mut ConfigResultIR,
        font: *const ::core::ffi::c_char,
    );
    fn cfg_exec(
        current_match: *mut Match,
        result: *mut ConfigResultIR,
        exectype: *const ::core::ffi::c_char,
        no_startup_id: *const ::core::ffi::c_char,
        command: *const ::core::ffi::c_char,
    );
    fn cfg_for_window(
        current_match: *mut Match,
        result: *mut ConfigResultIR,
        command: *const ::core::ffi::c_char,
    );
    fn cfg_gaps(
        current_match: *mut Match,
        result: *mut ConfigResultIR,
        workspace: *const ::core::ffi::c_char,
        scope: *const ::core::ffi::c_char,
        value: ::core::ffi::c_long,
    );
    fn cfg_smart_borders(
        current_match: *mut Match,
        result: *mut ConfigResultIR,
        enable: *const ::core::ffi::c_char,
    );
    fn cfg_smart_gaps(
        current_match: *mut Match,
        result: *mut ConfigResultIR,
        enable: *const ::core::ffi::c_char,
    );
    fn cfg_floating_minimum_size(
        current_match: *mut Match,
        result: *mut ConfigResultIR,
        width: ::core::ffi::c_long,
        height: ::core::ffi::c_long,
    );
    fn cfg_floating_maximum_size(
        current_match: *mut Match,
        result: *mut ConfigResultIR,
        width: ::core::ffi::c_long,
        height: ::core::ffi::c_long,
    );
    fn cfg_default_orientation(
        current_match: *mut Match,
        result: *mut ConfigResultIR,
        orientation: *const ::core::ffi::c_char,
    );
    fn cfg_workspace_layout(
        current_match: *mut Match,
        result: *mut ConfigResultIR,
        layout: *const ::core::ffi::c_char,
    );
    fn cfg_workspace_back_and_forth(
        current_match: *mut Match,
        result: *mut ConfigResultIR,
        value: *const ::core::ffi::c_char,
    );
    fn cfg_focus_follows_mouse(
        current_match: *mut Match,
        result: *mut ConfigResultIR,
        value: *const ::core::ffi::c_char,
    );
    fn cfg_mouse_warping(
        current_match: *mut Match,
        result: *mut ConfigResultIR,
        value: *const ::core::ffi::c_char,
    );
    fn cfg_focus_wrapping(
        current_match: *mut Match,
        result: *mut ConfigResultIR,
        value: *const ::core::ffi::c_char,
    );
    fn cfg_force_focus_wrapping(
        current_match: *mut Match,
        result: *mut ConfigResultIR,
        value: *const ::core::ffi::c_char,
    );
    fn cfg_force_xinerama(
        current_match: *mut Match,
        result: *mut ConfigResultIR,
        value: *const ::core::ffi::c_char,
    );
    fn cfg_disable_randr15(
        current_match: *mut Match,
        result: *mut ConfigResultIR,
        value: *const ::core::ffi::c_char,
    );
    fn cfg_fake_outputs(
        current_match: *mut Match,
        result: *mut ConfigResultIR,
        outputs: *const ::core::ffi::c_char,
    );
    fn cfg_force_display_urgency_hint(
        current_match: *mut Match,
        result: *mut ConfigResultIR,
        duration_ms: ::core::ffi::c_long,
    );
    fn cfg_focus_on_window_activation(
        current_match: *mut Match,
        result: *mut ConfigResultIR,
        mode: *const ::core::ffi::c_char,
    );
    fn cfg_title_align(
        current_match: *mut Match,
        result: *mut ConfigResultIR,
        alignment: *const ::core::ffi::c_char,
    );
    fn cfg_show_marks(
        current_match: *mut Match,
        result: *mut ConfigResultIR,
        value: *const ::core::ffi::c_char,
    );
    fn cfg_hide_edge_borders(
        current_match: *mut Match,
        result: *mut ConfigResultIR,
        borders: *const ::core::ffi::c_char,
    );
    fn cfg_assign_output(
        current_match: *mut Match,
        result: *mut ConfigResultIR,
        output: *const ::core::ffi::c_char,
    );
    fn cfg_assign(
        current_match: *mut Match,
        result: *mut ConfigResultIR,
        workspace: *const ::core::ffi::c_char,
        is_number: bool,
    );
    fn cfg_no_focus(current_match: *mut Match, result: *mut ConfigResultIR);
    fn cfg_ipc_socket(
        current_match: *mut Match,
        result: *mut ConfigResultIR,
        path: *const ::core::ffi::c_char,
    );
    fn cfg_ipc_kill_timeout(
        current_match: *mut Match,
        result: *mut ConfigResultIR,
        timeout_ms: ::core::ffi::c_long,
    );
    fn cfg_tiling_drag(
        current_match: *mut Match,
        result: *mut ConfigResultIR,
        value: *const ::core::ffi::c_char,
    );
    fn cfg_tiling_drag_swap_modifier(
        current_match: *mut Match,
        result: *mut ConfigResultIR,
        modifiers: *const ::core::ffi::c_char,
    );
    fn cfg_restart_state(
        current_match: *mut Match,
        result: *mut ConfigResultIR,
        path: *const ::core::ffi::c_char,
    );
    fn cfg_popup_during_fullscreen(
        current_match: *mut Match,
        result: *mut ConfigResultIR,
        value: *const ::core::ffi::c_char,
    );
    fn cfg_color(
        current_match: *mut Match,
        result: *mut ConfigResultIR,
        colorclass: *const ::core::ffi::c_char,
        border: *const ::core::ffi::c_char,
        background: *const ::core::ffi::c_char,
        text: *const ::core::ffi::c_char,
        indicator: *const ::core::ffi::c_char,
        child_border: *const ::core::ffi::c_char,
    );
    fn cfg_color_single(
        current_match: *mut Match,
        result: *mut ConfigResultIR,
        colorclass: *const ::core::ffi::c_char,
        color: *const ::core::ffi::c_char,
    );
    fn cfg_floating_modifier(
        current_match: *mut Match,
        result: *mut ConfigResultIR,
        modifiers: *const ::core::ffi::c_char,
    );
    fn cfg_default_border(
        current_match: *mut Match,
        result: *mut ConfigResultIR,
        windowtype: *const ::core::ffi::c_char,
        border: *const ::core::ffi::c_char,
        width: ::core::ffi::c_long,
    );
    fn cfg_workspace(
        current_match: *mut Match,
        result: *mut ConfigResultIR,
        workspace: *const ::core::ffi::c_char,
        output: *const ::core::ffi::c_char,
    );
    fn cfg_binding(
        current_match: *mut Match,
        result: *mut ConfigResultIR,
        bindtype: *const ::core::ffi::c_char,
        modifiers: *const ::core::ffi::c_char,
        key: *const ::core::ffi::c_char,
        release: *const ::core::ffi::c_char,
        border: *const ::core::ffi::c_char,
        whole_window: *const ::core::ffi::c_char,
        exclude_titlebar: *const ::core::ffi::c_char,
        command: *const ::core::ffi::c_char,
    );
    fn cfg_enter_mode(
        current_match: *mut Match,
        result: *mut ConfigResultIR,
        pango_markup: *const ::core::ffi::c_char,
        modename: *const ::core::ffi::c_char,
    );
    fn cfg_mode_binding(
        current_match: *mut Match,
        result: *mut ConfigResultIR,
        bindtype: *const ::core::ffi::c_char,
        modifiers: *const ::core::ffi::c_char,
        key: *const ::core::ffi::c_char,
        release: *const ::core::ffi::c_char,
        border: *const ::core::ffi::c_char,
        whole_window: *const ::core::ffi::c_char,
        exclude_titlebar: *const ::core::ffi::c_char,
        command: *const ::core::ffi::c_char,
    );
    fn cfg_bar_font(
        current_match: *mut Match,
        result: *mut ConfigResultIR,
        font: *const ::core::ffi::c_char,
    );
    fn cfg_bar_separator_symbol(
        current_match: *mut Match,
        result: *mut ConfigResultIR,
        separator: *const ::core::ffi::c_char,
    );
    fn cfg_bar_mode(
        current_match: *mut Match,
        result: *mut ConfigResultIR,
        mode: *const ::core::ffi::c_char,
    );
    fn cfg_bar_hidden_state(
        current_match: *mut Match,
        result: *mut ConfigResultIR,
        hidden_state: *const ::core::ffi::c_char,
    );
    fn cfg_bar_id(
        current_match: *mut Match,
        result: *mut ConfigResultIR,
        bar_id: *const ::core::ffi::c_char,
    );
    fn cfg_bar_output(
        current_match: *mut Match,
        result: *mut ConfigResultIR,
        output: *const ::core::ffi::c_char,
    );
    fn cfg_bar_verbose(
        current_match: *mut Match,
        result: *mut ConfigResultIR,
        verbose: *const ::core::ffi::c_char,
    );
    fn cfg_bar_height(
        current_match: *mut Match,
        result: *mut ConfigResultIR,
        height: ::core::ffi::c_long,
    );
    fn cfg_bar_padding_one(
        current_match: *mut Match,
        result: *mut ConfigResultIR,
        all: ::core::ffi::c_long,
    );
    fn cfg_bar_padding_two(
        current_match: *mut Match,
        result: *mut ConfigResultIR,
        top_and_bottom: ::core::ffi::c_long,
        right_and_left: ::core::ffi::c_long,
    );
    fn cfg_bar_padding_three(
        current_match: *mut Match,
        result: *mut ConfigResultIR,
        top: ::core::ffi::c_long,
        right_and_left: ::core::ffi::c_long,
        bottom: ::core::ffi::c_long,
    );
    fn cfg_bar_padding_four(
        current_match: *mut Match,
        result: *mut ConfigResultIR,
        top: ::core::ffi::c_long,
        right: ::core::ffi::c_long,
        bottom: ::core::ffi::c_long,
        left: ::core::ffi::c_long,
    );
    fn cfg_bar_modifier(
        current_match: *mut Match,
        result: *mut ConfigResultIR,
        modifiers: *const ::core::ffi::c_char,
    );
    fn cfg_bar_wheel_up_cmd(
        current_match: *mut Match,
        result: *mut ConfigResultIR,
        command: *const ::core::ffi::c_char,
    );
    fn cfg_bar_wheel_down_cmd(
        current_match: *mut Match,
        result: *mut ConfigResultIR,
        command: *const ::core::ffi::c_char,
    );
    fn cfg_bar_bindsym(
        current_match: *mut Match,
        result: *mut ConfigResultIR,
        button: *const ::core::ffi::c_char,
        release: *const ::core::ffi::c_char,
        command: *const ::core::ffi::c_char,
    );
    fn cfg_bar_position(
        current_match: *mut Match,
        result: *mut ConfigResultIR,
        position: *const ::core::ffi::c_char,
    );
    fn cfg_bar_i3bar_command(
        current_match: *mut Match,
        result: *mut ConfigResultIR,
        i3bar_command: *const ::core::ffi::c_char,
    );
    fn cfg_bar_color(
        current_match: *mut Match,
        result: *mut ConfigResultIR,
        colorclass: *const ::core::ffi::c_char,
        border: *const ::core::ffi::c_char,
        background: *const ::core::ffi::c_char,
        text: *const ::core::ffi::c_char,
    );
    fn cfg_bar_socket_path(
        current_match: *mut Match,
        result: *mut ConfigResultIR,
        socket_path: *const ::core::ffi::c_char,
    );
    fn cfg_bar_tray_output(
        current_match: *mut Match,
        result: *mut ConfigResultIR,
        output: *const ::core::ffi::c_char,
    );
    fn cfg_bar_tray_padding(
        current_match: *mut Match,
        result: *mut ConfigResultIR,
        padding_px: ::core::ffi::c_long,
    );
    fn cfg_bar_color_single(
        current_match: *mut Match,
        result: *mut ConfigResultIR,
        colorclass: *const ::core::ffi::c_char,
        color: *const ::core::ffi::c_char,
    );
    fn cfg_bar_status_command(
        current_match: *mut Match,
        result: *mut ConfigResultIR,
        command: *const ::core::ffi::c_char,
    );
    fn cfg_bar_workspace_command(
        current_match: *mut Match,
        result: *mut ConfigResultIR,
        command: *const ::core::ffi::c_char,
    );
    fn cfg_bar_binding_mode_indicator(
        current_match: *mut Match,
        result: *mut ConfigResultIR,
        value: *const ::core::ffi::c_char,
    );
    fn cfg_bar_workspace_buttons(
        current_match: *mut Match,
        result: *mut ConfigResultIR,
        value: *const ::core::ffi::c_char,
    );
    fn cfg_bar_workspace_min_width(
        current_match: *mut Match,
        result: *mut ConfigResultIR,
        width: ::core::ffi::c_long,
    );
    fn cfg_bar_strip_workspace_numbers(
        current_match: *mut Match,
        result: *mut ConfigResultIR,
        value: *const ::core::ffi::c_char,
    );
    fn cfg_bar_strip_workspace_name(
        current_match: *mut Match,
        result: *mut ConfigResultIR,
        value: *const ::core::ffi::c_char,
    );
    fn cfg_bar_start(current_match: *mut Match, result: *mut ConfigResultIR);
    fn cfg_bar_finish(current_match: *mut Match, result: *mut ConfigResultIR);
    fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    fn dirname(__path: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn xcb_xrm_database_from_default(conn_0: *mut xcb_connection_t) -> *mut xcb_xrm_database_t;
    fn xcb_xrm_database_from_string(str: *const ::core::ffi::c_char) -> *mut xcb_xrm_database_t;
    fn xcb_xrm_database_free(database_0: *mut xcb_xrm_database_t);
    fn xcb_xrm_resource_get_string(
        database_0: *mut xcb_xrm_database_t,
        res_name: *const ::core::ffi::c_char,
        res_class: *const ::core::ffi::c_char,
        out: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strncpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strcasestr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    fn strcasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
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
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type __dev_t = ::core::ffi::c_ulong;
pub type __uid_t = ::core::ffi::c_uint;
pub type __gid_t = ::core::ffi::c_uint;
pub type __ino_t = ::core::ffi::c_ulong;
pub type __mode_t = ::core::ffi::c_uint;
pub type __nlink_t = ::core::ffi::c_ulong;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __pid_t = ::core::ffi::c_int;
pub type __time_t = ::core::ffi::c_long;
pub type __suseconds_t = ::core::ffi::c_long;
pub type __blksize_t = ::core::ffi::c_long;
pub type __blkcnt_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: ::core::ffi::c_int,
    pub _IO_read_ptr: *mut ::core::ffi::c_char,
    pub _IO_read_end: *mut ::core::ffi::c_char,
    pub _IO_read_base: *mut ::core::ffi::c_char,
    pub _IO_write_base: *mut ::core::ffi::c_char,
    pub _IO_write_ptr: *mut ::core::ffi::c_char,
    pub _IO_write_end: *mut ::core::ffi::c_char,
    pub _IO_buf_base: *mut ::core::ffi::c_char,
    pub _IO_buf_end: *mut ::core::ffi::c_char,
    pub _IO_save_base: *mut ::core::ffi::c_char,
    pub _IO_backup_base: *mut ::core::ffi::c_char,
    pub _IO_save_end: *mut ::core::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::core::ffi::c_int,
    #[bitfield(name = "_flags2", ty = "::core::ffi::c_int", bits = "0..=23")]
    pub _flags2: [u8; 3],
    pub _short_backupbuf: [::core::ffi::c_char; 1],
    pub _old_offset: __off_t,
    pub _cur_column: ::core::ffi::c_ushort,
    pub _vtable_offset: ::core::ffi::c_schar,
    pub _shortbuf: [::core::ffi::c_char; 1],
    pub _lock: *mut ::core::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::core::ffi::c_void,
    pub _prevchain: *mut *mut _IO_FILE,
    pub _mode: ::core::ffi::c_int,
    pub _unused3: ::core::ffi::c_int,
    pub _total_written: __uint64_t,
    pub _unused2: [::core::ffi::c_char; 8],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: ::core::ffi::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
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
    pub type_0: C2RustUnnamed_18,
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
    pub floating: C2RustUnnamed_7,
    pub nodes: C2RustUnnamed_6,
    pub focused: C2RustUnnamed_5,
    pub all_cons: C2RustUnnamed_4,
    pub floating_windows: C2RustUnnamed_3,
    pub on_remove_child: Option<unsafe extern "C" fn(*mut Con) -> ()>,
    pub scratchpad_state: C2RustUnnamed_2,
    pub old_id: ::core::ffi::c_int,
    pub depth: uint16_t,
    pub colormap: xcb_colormap_t,
}
pub type C2RustUnnamed_2 = ::core::ffi::c_uint;
pub const SCRATCHPAD_CHANGED: C2RustUnnamed_2 = 2;
pub const SCRATCHPAD_FRESH: C2RustUnnamed_2 = 1;
pub const SCRATCHPAD_NONE: C2RustUnnamed_2 = 0;
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
pub type C2RustUnnamed_7 = ::core::ffi::c_uint;
pub const FLOATING_USER_ON: C2RustUnnamed_7 = 3;
pub const FLOATING_AUTO_ON: C2RustUnnamed_7 = 2;
pub const FLOATING_USER_OFF: C2RustUnnamed_7 = 1;
pub const FLOATING_AUTO_OFF: C2RustUnnamed_7 = 0;
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
    pub urgent: C2RustUnnamed_12,
    pub dock: C2RustUnnamed_11,
    pub id: xcb_window_t,
    pub window_mode: C2RustUnnamed_10,
    pub con_id: *mut Con,
    pub match_all_windows: bool,
    pub insert_where: C2RustUnnamed_9,
    pub matches: C2RustUnnamed_8,
    pub restart_mode: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub tqe_next: *mut Match,
    pub tqe_prev: *mut *mut Match,
}
pub type C2RustUnnamed_9 = ::core::ffi::c_uint;
pub const M_BELOW: C2RustUnnamed_9 = 2;
pub const M_ASSIGN_WS: C2RustUnnamed_9 = 1;
pub const M_HERE: C2RustUnnamed_9 = 0;
pub type C2RustUnnamed_10 = ::core::ffi::c_uint;
pub const WM_FLOATING: C2RustUnnamed_10 = 6;
pub const WM_FLOATING_USER: C2RustUnnamed_10 = 5;
pub const WM_FLOATING_AUTO: C2RustUnnamed_10 = 4;
pub const WM_TILING: C2RustUnnamed_10 = 3;
pub const WM_TILING_USER: C2RustUnnamed_10 = 2;
pub const WM_TILING_AUTO: C2RustUnnamed_10 = 1;
pub const WM_ANY: C2RustUnnamed_10 = 0;
pub type C2RustUnnamed_11 = ::core::ffi::c_int;
pub const M_DOCK_BOTTOM: C2RustUnnamed_11 = 3;
pub const M_DOCK_TOP: C2RustUnnamed_11 = 2;
pub const M_DOCK_ANY: C2RustUnnamed_11 = 1;
pub const M_NODOCK: C2RustUnnamed_11 = 0;
pub const M_DONTCHECK: C2RustUnnamed_11 = -1;
pub type C2RustUnnamed_12 = ::core::ffi::c_int;
pub const U_OLDEST: C2RustUnnamed_12 = 1;
pub const U_LATEST: C2RustUnnamed_12 = 0;
pub const U_DONTCHECK: C2RustUnnamed_12 = -1;
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
    pub dock: C2RustUnnamed_13,
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
pub type C2RustUnnamed_13 = ::core::ffi::c_uint;
pub const W_DOCK_BOTTOM: C2RustUnnamed_13 = 2;
pub const W_DOCK_TOP: C2RustUnnamed_13 = 1;
pub const W_NODOCK: C2RustUnnamed_13 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Assignment {
    pub type_0: C2RustUnnamed_16,
    pub match_0: Match,
    pub dest: C2RustUnnamed_15,
    pub assignments: C2RustUnnamed_14,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub tqe_next: *mut Assignment,
    pub tqe_prev: *mut *mut Assignment,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_15 {
    pub command: *mut ::core::ffi::c_char,
    pub workspace: *mut ::core::ffi::c_char,
    pub output: *mut ::core::ffi::c_char,
}
pub type C2RustUnnamed_16 = ::core::ffi::c_uint;
pub const A_TO_OUTPUT: C2RustUnnamed_16 = 16;
pub const A_TO_WORKSPACE_NUMBER: C2RustUnnamed_16 = 8;
pub const A_NO_FOCUS: C2RustUnnamed_16 = 4;
pub const A_TO_WORKSPACE: C2RustUnnamed_16 = 2;
pub const A_COMMAND: C2RustUnnamed_16 = 1;
pub const A_ANY: C2RustUnnamed_16 = 0;
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
    pub marks: C2RustUnnamed_17,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_17 {
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
pub type C2RustUnnamed_18 = ::core::ffi::c_uint;
pub const CT_DOCKAREA: C2RustUnnamed_18 = 5;
pub const CT_WORKSPACE: C2RustUnnamed_18 = 4;
pub const CT_FLOATING_CON: C2RustUnnamed_18 = 3;
pub const CT_CON: C2RustUnnamed_18 = 2;
pub const CT_OUTPUT: C2RustUnnamed_18 = 1;
pub const CT_ROOT: C2RustUnnamed_18 = 0;
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
pub type tiling_drag_t = ::core::ffi::c_uint;
pub const TILING_DRAG_MODIFIER_OR_TITLEBAR: tiling_drag_t = 3;
pub const TILING_DRAG_TITLEBAR: tiling_drag_t = 2;
pub const TILING_DRAG_MODIFIER: tiling_drag_t = 1;
pub const TILING_DRAG_OFF: tiling_drag_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IncludedFile {
    pub path: *mut ::core::ffi::c_char,
    pub raw_contents: *mut ::core::ffi::c_char,
    pub variable_replaced_contents: *mut ::core::ffi::c_char,
    pub files: C2RustUnnamed_19,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_19 {
    pub tqe_next: *mut IncludedFile,
    pub tqe_prev: *mut *mut IncludedFile,
}
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
    pub focus_on_window_activation: C2RustUnnamed_22,
    pub show_marks: bool,
    pub title_align: C2RustUnnamed_21,
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
    pub popup_during_fullscreen: C2RustUnnamed_20,
    pub number_barconfigs: ::core::ffi::c_int,
    pub tiling_drag: tiling_drag_t,
    pub gaps: gaps_t,
    pub smart_gaps: smart_gaps_t,
}
pub type C2RustUnnamed_20 = ::core::ffi::c_uint;
pub const PDF_ALL: C2RustUnnamed_20 = 3;
pub const PDF_IGNORE: C2RustUnnamed_20 = 2;
pub const PDF_LEAVE_FULLSCREEN: C2RustUnnamed_20 = 1;
pub const PDF_SMART: C2RustUnnamed_20 = 0;
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
pub type C2RustUnnamed_21 = ::core::ffi::c_uint;
pub const ALIGN_RIGHT: C2RustUnnamed_21 = 2;
pub const ALIGN_CENTER: C2RustUnnamed_21 = 1;
pub const ALIGN_LEFT: C2RustUnnamed_21 = 0;
pub type C2RustUnnamed_22 = ::core::ffi::c_uint;
pub const FOWA_NONE: C2RustUnnamed_22 = 3;
pub const FOWA_FOCUS: C2RustUnnamed_22 = 2;
pub const FOWA_URGENT: C2RustUnnamed_22 = 1;
pub const FOWA_SMART: C2RustUnnamed_22 = 0;
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
pub struct Variable {
    pub key: *mut ::core::ffi::c_char,
    pub value: *mut ::core::ffi::c_char,
    pub next_match: *mut ::core::ffi::c_char,
    pub variables: C2RustUnnamed_23,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_23 {
    pub sle_next: *mut Variable,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack_entry {
    pub identifier: *const ::core::ffi::c_char,
    pub type_0: C2RustUnnamed_25,
    pub val: C2RustUnnamed_24,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_24 {
    pub str_0: *mut ::core::ffi::c_char,
    pub num: ::core::ffi::c_long,
}
pub type C2RustUnnamed_25 = ::core::ffi::c_uint;
pub const STACK_LONG: C2RustUnnamed_25 = 1;
pub const STACK_STR: C2RustUnnamed_25 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack {
    pub stack: [stack_entry; 10],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct variables_head {
    pub slh_first: *mut Variable,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct parser_ctx {
    pub use_nagbar: bool,
    pub state: ::core::ffi::c_int,
    pub current_match: Match,
    pub statelist: [::core::ffi::c_int; 10],
    pub statelist_idx: ::core::ffi::c_int,
    pub stack: *mut stack,
    pub variables: variables_head,
    pub has_errors: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ConfigResultIR {
    pub ctx: *mut parser_ctx,
    pub next_state: ::core::ffi::c_int,
    pub has_errors: bool,
}
pub type parse_file_result_t = ::core::ffi::c_int;
pub const PARSE_FILE_CONFIG_ERRORS: parse_file_result_t = 1;
pub const PARSE_FILE_SUCCESS: parse_file_result_t = 0;
pub const PARSE_FILE_FAILED: parse_file_result_t = -1;
pub type cmdp_token = token;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct token {
    pub name: *mut ::core::ffi::c_char,
    pub identifier: *mut ::core::ffi::c_char,
    pub next_state: cmdp_state,
    pub extra: C2RustUnnamed_26,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_26 {
    pub call_identifier: uint16_t,
}
pub type cmdp_state = ::core::ffi::c_uint;
pub const __CALL: cmdp_state = 117;
pub const BAR: cmdp_state = 116;
pub const MODE: cmdp_state = 115;
pub const GAPS: cmdp_state = 114;
pub const FONT: cmdp_state = 113;
pub const EXEC: cmdp_state = 112;
pub const BAR_ID: cmdp_state = 111;
pub const ASSIGN: cmdp_state = 110;
pub const INITIAL: cmdp_state = 109;
pub const INCLUDE: cmdp_state = 108;
pub const BINDING: cmdp_state = 107;
pub const NO_FOCUS: cmdp_state = 106;
pub const MODENAME: cmdp_state = 105;
pub const GAPS_END: cmdp_state = 104;
pub const CRITERIA: cmdp_state = 103;
pub const BAR_MODE: cmdp_state = 102;
pub const BAR_FONT: cmdp_state = 101;
pub const BARBRACE: cmdp_state = 100;
pub const WORKSPACE: cmdp_state = 99;
pub const MODEBRACE: cmdp_state = 98;
pub const CRITERION: cmdp_state = 97;
pub const SMART_GAPS: cmdp_state = 96;
pub const SHOW_MARKS: cmdp_state = 95;
pub const IPC_SOCKET: cmdp_state = 94;
pub const FOR_WINDOW: cmdp_state = 93;
pub const COLOR_TEXT: cmdp_state = 92;
pub const BAR_OUTPUT: cmdp_state = 91;
pub const BAR_HEIGHT: cmdp_state = 90;
pub const BAR_COLORS: cmdp_state = 89;
pub const TITLE_ALIGN: cmdp_state = 88;
pub const TILING_DRAG: cmdp_state = 87;
pub const IGNORE_LINE: cmdp_state = 86;
pub const BINDCOMMAND: cmdp_state = 85;
pub const BAR_VERBOSE: cmdp_state = 84;
pub const BAR_PADDING: cmdp_state = 83;
pub const BAR_BINDSYM: cmdp_state = 82;
pub const NO_FOCUS_END: cmdp_state = 81;
pub const MODE_BINDING: cmdp_state = 80;
pub const FAKE_OUTPUTS: cmdp_state = 79;
pub const COLOR_SINGLE: cmdp_state = 78;
pub const COLOR_BORDER: cmdp_state = 77;
pub const BAR_POSITION: cmdp_state = 76;
pub const BAR_MODIFIER: cmdp_state = 75;
pub const SMART_BORDERS: cmdp_state = 74;
pub const RESTART_STATE: cmdp_state = 73;
pub const MOUSE_WARPING: cmdp_state = 72;
pub const CRITERION_STR: cmdp_state = 71;
pub const ASSIGN_OUTPUT: cmdp_state = 70;
pub const FORCE_XINERAMA: cmdp_state = 69;
pub const FOCUS_WRAPPING: cmdp_state = 68;
pub const DEFAULT_BORDER: cmdp_state = 67;
pub const CRITERION_FROM: cmdp_state = 66;
pub const GAPS_WITH_SCOPE: cmdp_state = 65;
pub const DISABLE_RANDR15: cmdp_state = 64;
pub const COLOR_INDICATOR: cmdp_state = 63;
pub const BAR_TRAY_OUTPUT: cmdp_state = 62;
pub const BAR_SOCKET_PATH: cmdp_state = 61;
pub const BAR_PADDING_TOP: cmdp_state = 60;
pub const BAR_IGNORE_LINE: cmdp_state = 59;
pub const BAR_COLORS_TEXT: cmdp_state = 58;
pub const BAR_BAR_COMMAND: cmdp_state = 57;
pub const WORKSPACE_LAYOUT: cmdp_state = 56;
pub const TILING_DRAG_MODE: cmdp_state = 55;
pub const MODE_IGNORE_LINE: cmdp_state = 54;
pub const MODE_BINDCOMMAND: cmdp_state = 53;
pub const IPC_KILL_TIMEOUT: cmdp_state = 52;
pub const COLOR_BACKGROUND: cmdp_state = 51;
pub const BAR_WHEEL_UP_CMD: cmdp_state = 50;
pub const BAR_TRAY_PADDING: cmdp_state = 49;
pub const BAR_PADDING_LEFT: cmdp_state = 48;
pub const BAR_HIDDEN_STATE: cmdp_state = 47;
pub const BAR_COLORS_BRACE: cmdp_state = 46;
pub const ASSIGN_WORKSPACE: cmdp_state = 45;
pub const WORKSPACE_COMMAND: cmdp_state = 44;
pub const HIDE_EDGE_BORDERS: cmdp_state = 43;
pub const FLOATING_MODIFIER: cmdp_state = 42;
pub const BAR_PADDING_RIGHT: cmdp_state = 41;
pub const BAR_COLORS_SINGLE: cmdp_state = 40;
pub const BAR_COLORS_BORDER: cmdp_state = 39;
pub const FOR_WINDOW_COMMAND: cmdp_state = 38;
pub const CRITERION_FROM_STR: cmdp_state = 37;
pub const COLOR_CHILD_BORDER: cmdp_state = 36;
pub const BAR_WHEEL_DOWN_CMD: cmdp_state = 35;
pub const BAR_STATUS_COMMAND: cmdp_state = 34;
pub const BAR_PADDING_BOTTOM: cmdp_state = 33;
pub const FOCUS_FOLLOWS_MOUSE: cmdp_state = 32;
pub const DEFAULT_ORIENTATION: cmdp_state = 31;
pub const BAR_TRAY_PADDING_PX: cmdp_state = 30;
pub const BAR_BINDSYM_COMMAND: cmdp_state = 29;
pub const FORCE_FOCUS_WRAPPING: cmdp_state = 28;
pub const BAR_SEPARATOR_SYMBOL: cmdp_state = 27;
pub const WORKSPACE_OUTPUT_WORD: cmdp_state = 26;
pub const DEFAULT_BORDER_PIXELS: cmdp_state = 25;
pub const BAR_WORKSPACE_COMMAND: cmdp_state = 24;
pub const BAR_WORKSPACE_BUTTONS: cmdp_state = 23;
pub const BAR_COLORS_BACKGROUND: cmdp_state = 22;
pub const CRITERION_FROM_STR_END: cmdp_state = 21;
pub const BAR_COLORS_IGNORE_LINE: cmdp_state = 20;
pub const POPUP_DURING_FULLSCREEN: cmdp_state = 19;
pub const FLOATING_MINIMUM_SIZE_X: cmdp_state = 18;
pub const FLOATING_MAXIMUM_SIZE_X: cmdp_state = 17;
pub const BAR_WORKSPACE_MIN_WIDTH: cmdp_state = 16;
pub const ASSIGN_WORKSPACE_NUMBER: cmdp_state = 15;
pub const WORKSPACE_BACK_AND_FORTH: cmdp_state = 14;
pub const DEFAULT_BORDER_PIXELS_PX: cmdp_state = 13;
pub const CRITERION_FROM_STR_START: cmdp_state = 12;
pub const BAR_STRIP_WORKSPACE_NAME: cmdp_state = 11;
pub const TILING_DRAG_SWAP_MODIFIER: cmdp_state = 10;
pub const FORCE_DISPLAY_URGENCY_HINT: cmdp_state = 9;
pub const FOCUS_ON_WINDOW_ACTIVATION: cmdp_state = 8;
pub const BAR_WORKSPACE_MIN_WIDTH_PX: cmdp_state = 7;
pub const BAR_BINDING_MODE_INDICATOR: cmdp_state = 6;
pub const FLOATING_MINIMUM_SIZE_WIDTH: cmdp_state = 5;
pub const FLOATING_MAXIMUM_SIZE_WIDTH: cmdp_state = 4;
pub const BAR_STRIP_WORKSPACE_NUMBERS: cmdp_state = 3;
pub const FLOATING_MINIMUM_SIZE_HEIGHT: cmdp_state = 2;
pub const FLOATING_MAXIMUM_SIZE_HEIGHT: cmdp_state = 1;
pub const FORCE_DISPLAY_URGENCY_HINT_MS: cmdp_state = 0;
pub type cmdp_token_ptr = tokenptr;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tokenptr {
    pub array: *mut cmdp_token,
    pub n: ::core::ffi::c_int,
}
pub const LONG_MAX: ::core::ffi::c_long = __LONG_MAX__;
pub const LONG_MIN: ::core::ffi::c_long = -__LONG_MAX__ - 1 as ::core::ffi::c_long;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut database: *mut xcb_xrm_database_t =
    ::core::ptr::null::<xcb_xrm_database_t>() as *mut xcb_xrm_database_t;
#[no_mangle]
pub static mut config_error_nagbar_pid: pid_t = -(1 as pid_t);
static mut tokens_FORCE_DISPLAY_URGENCY_HINT_MS: [cmdp_token; 2] = [
    token {
        name: b"'ms\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: FORCE_DISPLAY_URGENCY_HINT_MS,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"end\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_FLOATING_MAXIMUM_SIZE_HEIGHT: [cmdp_token; 1] = [token {
    name: b"number\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"height\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 1 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_FLOATING_MINIMUM_SIZE_HEIGHT: [cmdp_token; 1] = [token {
    name: b"number\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"height\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 2 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_STRIP_WORKSPACE_NUMBERS: [cmdp_token; 1] = [token {
    name: b"word\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"value\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 3 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_FLOATING_MAXIMUM_SIZE_WIDTH: [cmdp_token; 1] = [token {
    name: b"number\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"width\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: FLOATING_MAXIMUM_SIZE_X,
    extra: C2RustUnnamed_26 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_FLOATING_MINIMUM_SIZE_WIDTH: [cmdp_token; 1] = [token {
    name: b"number\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"width\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: FLOATING_MINIMUM_SIZE_X,
    extra: C2RustUnnamed_26 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_BINDING_MODE_INDICATOR: [cmdp_token; 1] = [token {
    name: b"word\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"value\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 4 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_WORKSPACE_MIN_WIDTH_PX: [cmdp_token; 2] = [
    token {
        name: b"'px\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_WORKSPACE_MIN_WIDTH_PX,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"end\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 5 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_FOCUS_ON_WINDOW_ACTIVATION: [cmdp_token; 1] = [token {
    name: b"word\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"mode\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 6 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_FORCE_DISPLAY_URGENCY_HINT: [cmdp_token; 1] = [token {
    name: b"number\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"duration_ms\0" as *const u8 as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    next_state: FORCE_DISPLAY_URGENCY_HINT_MS,
    extra: C2RustUnnamed_26 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_TILING_DRAG_SWAP_MODIFIER: [cmdp_token; 10] = [
    token {
        name: b"'Mod1\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: TILING_DRAG_SWAP_MODIFIER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Mod2\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: TILING_DRAG_SWAP_MODIFIER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Mod3\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: TILING_DRAG_SWAP_MODIFIER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Mod4\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: TILING_DRAG_SWAP_MODIFIER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Mod5\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: TILING_DRAG_SWAP_MODIFIER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Shift\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: TILING_DRAG_SWAP_MODIFIER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Control\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: TILING_DRAG_SWAP_MODIFIER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Ctrl\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: TILING_DRAG_SWAP_MODIFIER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'+\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: TILING_DRAG_SWAP_MODIFIER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"end\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 7 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_BAR_STRIP_WORKSPACE_NAME: [cmdp_token; 1] = [token {
    name: b"word\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"value\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 8 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_CRITERION_FROM_STR_START: [cmdp_token; 3] = [
    token {
        name: b"'\"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CRITERION_FROM_STR,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'auto\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"kind\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 9 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'user\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"kind\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 10 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_DEFAULT_BORDER_PIXELS_PX: [cmdp_token; 2] = [
    token {
        name: b"'px\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: DEFAULT_BORDER_PIXELS_PX,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"end\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 11 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_WORKSPACE_BACK_AND_FORTH: [cmdp_token; 1] = [token {
    name: b"word\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"value\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 12 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_ASSIGN_WORKSPACE_NUMBER: [cmdp_token; 1] = [token {
    name: b"string\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"number\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 13 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_WORKSPACE_MIN_WIDTH: [cmdp_token; 1] = [token {
    name: b"number\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"width\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: BAR_WORKSPACE_MIN_WIDTH_PX,
    extra: C2RustUnnamed_26 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_FLOATING_MAXIMUM_SIZE_X: [cmdp_token; 1] = [token {
    name: b"'x\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: FLOATING_MAXIMUM_SIZE_HEIGHT,
    extra: C2RustUnnamed_26 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_FLOATING_MINIMUM_SIZE_X: [cmdp_token; 1] = [token {
    name: b"'x\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: FLOATING_MINIMUM_SIZE_HEIGHT,
    extra: C2RustUnnamed_26 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_POPUP_DURING_FULLSCREEN: [cmdp_token; 4] = [
    token {
        name: b"'ignore\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"value\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 14 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'leave_fullscreen\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"value\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 15 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'all\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"value\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 16 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'smart\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"value\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 17 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_BAR_COLORS_IGNORE_LINE: [cmdp_token; 1] = [token {
    name: b"line\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: BAR_COLORS,
    extra: C2RustUnnamed_26 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_CRITERION_FROM_STR_END: [cmdp_token; 1] = [token {
    name: b"'\"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 18 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_COLORS_BACKGROUND: [cmdp_token; 1] = [token {
    name: b"word\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"background\0" as *const u8 as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    next_state: BAR_COLORS_TEXT,
    extra: C2RustUnnamed_26 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_WORKSPACE_BUTTONS: [cmdp_token; 1] = [token {
    name: b"word\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"value\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 19 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_WORKSPACE_COMMAND: [cmdp_token; 1] = [token {
    name: b"string\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"command\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 20 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_DEFAULT_BORDER_PIXELS: [cmdp_token; 2] = [
    token {
        name: b"end\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 21 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"number\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"width\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: DEFAULT_BORDER_PIXELS_PX,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_WORKSPACE_OUTPUT_WORD: [cmdp_token; 2] = [
    token {
        name: b"word\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"output\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 22 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"end\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: INITIAL,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_BAR_SEPARATOR_SYMBOL: [cmdp_token; 1] = [token {
    name: b"string\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"separator\0" as *const u8 as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 23 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_FORCE_FOCUS_WRAPPING: [cmdp_token; 1] = [token {
    name: b"word\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"value\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 24 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_BINDSYM_COMMAND: [cmdp_token; 2] = [
    token {
        name: b"'--release\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"release\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BAR_BINDSYM_COMMAND,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"string\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"command\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 25 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_BAR_TRAY_PADDING_PX: [cmdp_token; 2] = [
    token {
        name: b"'px\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_TRAY_PADDING_PX,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"end\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 26 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_DEFAULT_ORIENTATION: [cmdp_token; 3] = [
    token {
        name: b"'horizontal\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"orientation\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 27 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'vertical\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"orientation\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 28 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'auto\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"orientation\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 29 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_FOCUS_FOLLOWS_MOUSE: [cmdp_token; 1] = [token {
    name: b"word\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"value\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 30 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_PADDING_BOTTOM: [cmdp_token; 3] = [
    token {
        name: b"'px\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_PADDING_BOTTOM,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"number\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"left\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BAR_PADDING_LEFT,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"end\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 31 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_BAR_STATUS_COMMAND: [cmdp_token; 1] = [token {
    name: b"string\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"command\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 32 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_WHEEL_DOWN_CMD: [cmdp_token; 1] = [token {
    name: b"string\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"command\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 33 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_COLOR_CHILD_BORDER: [cmdp_token; 2] = [
    token {
        name: b"word\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"child_border\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 34 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"end\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 35 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_CRITERION_FROM_STR: [cmdp_token; 2] = [
    token {
        name: b"'auto\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"kind\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: CRITERION_FROM_STR_END,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'user\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"kind\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: CRITERION_FROM_STR_END,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_FOR_WINDOW_COMMAND: [cmdp_token; 1] = [token {
    name: b"string\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"command\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 36 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_COLORS_BORDER: [cmdp_token; 1] = [token {
    name: b"word\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"border\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: BAR_COLORS_BACKGROUND,
    extra: C2RustUnnamed_26 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_COLORS_SINGLE: [cmdp_token; 1] = [token {
    name: b"word\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"color\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 37 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_PADDING_RIGHT: [cmdp_token; 3] = [
    token {
        name: b"'px\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_PADDING_RIGHT,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"number\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"bottom\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BAR_PADDING_BOTTOM,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"end\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 38 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_FLOATING_MODIFIER: [cmdp_token; 10] = [
    token {
        name: b"'Mod1\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: FLOATING_MODIFIER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Mod2\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: FLOATING_MODIFIER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Mod3\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: FLOATING_MODIFIER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Mod4\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: FLOATING_MODIFIER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Mod5\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: FLOATING_MODIFIER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Shift\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: FLOATING_MODIFIER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Control\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: FLOATING_MODIFIER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Ctrl\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: FLOATING_MODIFIER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'+\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: FLOATING_MODIFIER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"end\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 39 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_HIDE_EDGE_BORDERS: [cmdp_token; 12] = [
    token {
        name: b"'none\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"hide_borders\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 40 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'vertical\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"hide_borders\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 41 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'horizontal\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"hide_borders\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 42 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'both\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"hide_borders\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 43 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'smart_no_gaps\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"hide_borders\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 44 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'smart\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"hide_borders\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 45 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'1\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"hide_borders\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 46 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'yes\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"hide_borders\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 47 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'true\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"hide_borders\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 48 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'on\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"hide_borders\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 49 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'enable\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"hide_borders\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 50 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'active\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"hide_borders\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 51 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_WORKSPACE_COMMAND: [cmdp_token; 2] = [
    token {
        name: b"'output\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: WORKSPACE_OUTPUT_WORD,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'gaps\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: GAPS,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_ASSIGN_WORKSPACE: [cmdp_token; 5] = [
    token {
        name: b"'\xE2\x86\x92\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ASSIGN_WORKSPACE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'output\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ASSIGN_OUTPUT,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'workspace\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ASSIGN_WORKSPACE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'number\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ASSIGN_WORKSPACE_NUMBER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"string\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"workspace\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 52 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_BAR_COLORS_BRACE: [cmdp_token; 2] = [
    token {
        name: b"end\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_COLORS_BRACE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'{\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_COLORS,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_BAR_HIDDEN_STATE: [cmdp_token; 2] = [
    token {
        name: b"'hide\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"hidden_state\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 53 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'show\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"hidden_state\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 54 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_BAR_PADDING_LEFT: [cmdp_token; 2] = [
    token {
        name: b"'px\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_PADDING_LEFT,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"end\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 55 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_BAR_TRAY_PADDING: [cmdp_token; 1] = [token {
    name: b"number\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"padding_px\0" as *const u8 as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    next_state: BAR_TRAY_PADDING_PX,
    extra: C2RustUnnamed_26 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_WHEEL_UP_CMD: [cmdp_token; 1] = [token {
    name: b"string\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"command\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 56 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_COLOR_BACKGROUND: [cmdp_token; 1] = [token {
    name: b"word\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"background\0" as *const u8 as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    next_state: COLOR_TEXT,
    extra: C2RustUnnamed_26 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_IPC_KILL_TIMEOUT: [cmdp_token; 1] = [token {
    name: b"number\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"timeout\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 57 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_MODE_BINDCOMMAND: [cmdp_token; 5] = [
    token {
        name: b"'--release\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"release\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: MODE_BINDCOMMAND,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'--border\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"border\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: MODE_BINDCOMMAND,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'--whole-window\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"whole_window\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: MODE_BINDCOMMAND,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'--exclude-titlebar\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"exclude_titlebar\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: MODE_BINDCOMMAND,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"string\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"command\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 58 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_MODE_IGNORE_LINE: [cmdp_token; 1] = [token {
    name: b"line\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: MODE,
    extra: C2RustUnnamed_26 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_TILING_DRAG_MODE: [cmdp_token; 3] = [
    token {
        name: b"'modifier\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"value\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: TILING_DRAG_MODE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'titlebar\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"value\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: TILING_DRAG_MODE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"end\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 59 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_WORKSPACE_LAYOUT: [cmdp_token; 4] = [
    token {
        name: b"'default\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"layout\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 60 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'stacking\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"layout\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 61 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'stacked\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"layout\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 62 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'tabbed\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"layout\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 63 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_BAR_BAR_COMMAND: [cmdp_token; 1] = [token {
    name: b"string\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"command\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 64 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_COLORS_TEXT: [cmdp_token; 2] = [
    token {
        name: b"end\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 65 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"word\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"text\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 66 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_BAR_IGNORE_LINE: [cmdp_token; 1] = [token {
    name: b"line\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: BAR,
    extra: C2RustUnnamed_26 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_PADDING_TOP: [cmdp_token; 3] = [
    token {
        name: b"'px\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_PADDING_TOP,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"number\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"right_or_right_and_left\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BAR_PADDING_RIGHT,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"end\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 67 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_BAR_SOCKET_PATH: [cmdp_token; 1] = [token {
    name: b"string\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"path\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 68 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_TRAY_OUTPUT: [cmdp_token; 1] = [token {
    name: b"word\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"output\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 69 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_COLOR_INDICATOR: [cmdp_token; 2] = [
    token {
        name: b"word\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"indicator\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: COLOR_CHILD_BORDER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"end\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 70 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_DISABLE_RANDR15: [cmdp_token; 1] = [token {
    name: b"word\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"value\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 71 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_GAPS_WITH_SCOPE: [cmdp_token; 1] = [token {
    name: b"number\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"value\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: GAPS_END,
    extra: C2RustUnnamed_26 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_CRITERION_FROM: [cmdp_token; 1] = [token {
    name: b"'=\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: CRITERION_FROM_STR_START,
    extra: C2RustUnnamed_26 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_DEFAULT_BORDER: [cmdp_token; 4] = [
    token {
        name: b"'normal\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"border\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: DEFAULT_BORDER_PIXELS,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'pixel\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"border\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: DEFAULT_BORDER_PIXELS,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'1pixel\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"border\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 72 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'none\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"border\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 73 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_FOCUS_WRAPPING: [cmdp_token; 14] = [
    token {
        name: b"'1\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"value\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 74 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'yes\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"value\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 75 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'true\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"value\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 76 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'on\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"value\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 77 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'enable\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"value\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 78 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'active\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"value\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 79 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'0\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"value\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 80 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'no\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"value\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 81 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'false\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"value\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 82 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'off\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"value\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 83 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'disable\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"value\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 84 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'inactive\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"value\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 85 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'force\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"value\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 86 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'workspace\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"value\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 87 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_FORCE_XINERAMA: [cmdp_token; 1] = [token {
    name: b"word\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"value\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 88 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_ASSIGN_OUTPUT: [cmdp_token; 1] = [token {
    name: b"string\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"output\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 89 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_CRITERION_STR: [cmdp_token; 1] = [token {
    name: b"word\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"cvalue\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 90 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_MOUSE_WARPING: [cmdp_token; 2] = [
    token {
        name: b"'none\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"value\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 91 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'output\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"value\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 92 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_RESTART_STATE: [cmdp_token; 1] = [token {
    name: b"string\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"path\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 93 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_SMART_BORDERS: [cmdp_token; 7] = [
    token {
        name: b"'1\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"enabled\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 94 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'yes\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"enabled\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 95 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'true\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"enabled\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 96 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'on\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"enabled\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 97 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'enable\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"enabled\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 98 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'active\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"enabled\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 99 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'no_gaps\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"enabled\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 100 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_BAR_MODIFIER: [cmdp_token; 12] = [
    token {
        name: b"'off\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 101 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'none\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 102 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Mod1\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BAR_MODIFIER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Mod2\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BAR_MODIFIER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Mod3\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BAR_MODIFIER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Mod4\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BAR_MODIFIER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Mod5\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BAR_MODIFIER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Shift\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BAR_MODIFIER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Control\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BAR_MODIFIER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Ctrl\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BAR_MODIFIER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'+\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_MODIFIER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"end\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 103 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_BAR_POSITION: [cmdp_token; 2] = [
    token {
        name: b"'top\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"position\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 104 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'bottom\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"position\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 105 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_COLOR_BORDER: [cmdp_token; 1] = [token {
    name: b"word\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"border\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: COLOR_BACKGROUND,
    extra: C2RustUnnamed_26 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_COLOR_SINGLE: [cmdp_token; 1] = [token {
    name: b"word\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"color\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 106 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_FAKE_OUTPUTS: [cmdp_token; 1] = [token {
    name: b"string\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"outputs\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 107 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_MODE_BINDING: [cmdp_token; 20] = [
    token {
        name: b"'--release\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"release\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'--border\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"border\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'--whole-window\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"whole_window\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'--exclude-titlebar\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"exclude_titlebar\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Mod1\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Mod2\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Mod3\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Mod4\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Mod5\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Shift\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Control\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Ctrl\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Mode_switch\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Group1\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Group2\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Group3\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Group4\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'$mod\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'+\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"word\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"key\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        next_state: MODE_BINDCOMMAND,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_NO_FOCUS_END: [cmdp_token; 1] = [token {
    name: b"end\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 108 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_BINDSYM: [cmdp_token; 2] = [
    token {
        name: b"'--release\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"release\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BAR_BINDSYM,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"word\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"button\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BAR_BINDSYM_COMMAND,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_BAR_PADDING: [cmdp_token; 1] = [token {
    name: b"number\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"top_or_all\0" as *const u8 as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    next_state: BAR_PADDING_TOP,
    extra: C2RustUnnamed_26 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_VERBOSE: [cmdp_token; 1] = [token {
    name: b"word\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"value\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 109 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BINDCOMMAND: [cmdp_token; 5] = [
    token {
        name: b"'--release\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"release\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BINDCOMMAND,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'--border\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"border\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BINDCOMMAND,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'--whole-window\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"whole_window\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BINDCOMMAND,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'--exclude-titlebar\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"exclude_titlebar\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BINDCOMMAND,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"string\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"command\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 110 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_IGNORE_LINE: [cmdp_token; 1] = [token {
    name: b"line\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: INITIAL,
    extra: C2RustUnnamed_26 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_TILING_DRAG: [cmdp_token; 9] = [
    token {
        name: b"'0\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"off\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 111 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'no\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"off\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 112 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'false\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"off\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 113 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'off\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"off\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 114 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'disable\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"off\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 115 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'inactive\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"off\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 116 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'swap_modifier\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"swap_modifier\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: TILING_DRAG_SWAP_MODIFIER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'modifier\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"value\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: TILING_DRAG_MODE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'titlebar\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"value\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: TILING_DRAG_MODE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_TITLE_ALIGN: [cmdp_token; 3] = [
    token {
        name: b"'left\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"alignment\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 117 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'center\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"alignment\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 118 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'right\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"alignment\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 119 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_BAR_COLORS: [cmdp_token; 15] = [
    token {
        name: b"end\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_COLORS,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'#\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_COLORS_IGNORE_LINE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'set\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_COLORS_IGNORE_LINE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'background\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"colorclass\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BAR_COLORS_SINGLE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'statusline\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"colorclass\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BAR_COLORS_SINGLE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'separator\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"colorclass\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BAR_COLORS_SINGLE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'focused_background\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"colorclass\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BAR_COLORS_SINGLE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'focused_statusline\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"colorclass\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BAR_COLORS_SINGLE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'focused_separator\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"colorclass\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BAR_COLORS_SINGLE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'focused_workspace\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"colorclass\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BAR_COLORS_BORDER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'active_workspace\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"colorclass\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BAR_COLORS_BORDER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'inactive_workspace\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"colorclass\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BAR_COLORS_BORDER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'urgent_workspace\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"colorclass\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BAR_COLORS_BORDER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'binding_mode\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"colorclass\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BAR_COLORS_BORDER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'}\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_BAR_HEIGHT: [cmdp_token; 1] = [token {
    name: b"number\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"value\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 120 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_OUTPUT: [cmdp_token; 1] = [token {
    name: b"word\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"output\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 121 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_COLOR_TEXT: [cmdp_token; 1] = [token {
    name: b"word\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"text\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: COLOR_INDICATOR,
    extra: C2RustUnnamed_26 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_FOR_WINDOW: [cmdp_token; 1] = [token {
    name: b"'[\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 122 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_IPC_SOCKET: [cmdp_token; 1] = [token {
    name: b"string\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"path\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 123 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_SHOW_MARKS: [cmdp_token; 1] = [token {
    name: b"word\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"value\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 124 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_SMART_GAPS: [cmdp_token; 13] = [
    token {
        name: b"'1\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"enabled\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 125 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'yes\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"enabled\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 126 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'true\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"enabled\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 127 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'on\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"enabled\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 128 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'enable\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"enabled\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 129 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'active\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"enabled\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 130 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'0\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"enabled\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 131 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'no\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"enabled\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 132 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'false\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"enabled\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 133 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'off\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"enabled\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 134 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'disable\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"enabled\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 135 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'inactive\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"enabled\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 136 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'inverse_outer\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"enabled\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 137 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_CRITERION: [cmdp_token; 1] = [token {
    name: b"'=\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: CRITERION_STR,
    extra: C2RustUnnamed_26 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_MODEBRACE: [cmdp_token; 2] = [
    token {
        name: b"end\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: MODEBRACE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'{\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: MODE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_WORKSPACE: [cmdp_token; 1] = [token {
    name: b"word\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"workspace\0" as *const u8 as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    next_state: WORKSPACE_COMMAND,
    extra: C2RustUnnamed_26 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BARBRACE: [cmdp_token; 2] = [
    token {
        name: b"end\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BARBRACE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'{\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 138 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_BAR_FONT: [cmdp_token; 1] = [token {
    name: b"string\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"font\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 139 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_MODE: [cmdp_token; 3] = [
    token {
        name: b"'dock\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"mode\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 140 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'hide\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"mode\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 141 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'invisible\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"mode\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 142 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_CRITERIA: [cmdp_token; 17] = [
    token {
        name: b"'class\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"ctype\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: CRITERION,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'instance\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"ctype\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: CRITERION,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'window_role\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"ctype\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: CRITERION,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'con_id\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"ctype\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: CRITERION,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'id\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"ctype\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: CRITERION,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'window_type\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"ctype\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: CRITERION,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'con_mark\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"ctype\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: CRITERION,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'title\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"ctype\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: CRITERION,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'urgent\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"ctype\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: CRITERION,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'workspace\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"ctype\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: CRITERION,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'machine\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"ctype\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: CRITERION,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'floating_from\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"ctype\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: CRITERION_FROM,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'tiling_from\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"ctype\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: CRITERION_FROM,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'tiling\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"ctype\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 143 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'floating\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"ctype\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 144 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'all\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"ctype\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 145 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"']\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 146 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_GAPS_END: [cmdp_token; 2] = [
    token {
        name: b"'px\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: GAPS_END,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"end\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 147 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_MODENAME: [cmdp_token; 2] = [
    token {
        name: b"'--pango_markup\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"pango_markup\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: MODENAME,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"word\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modename\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 148 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_NO_FOCUS: [cmdp_token; 1] = [token {
    name: b"'[\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 149 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BINDING: [cmdp_token; 20] = [
    token {
        name: b"'--release\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"release\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'--border\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"border\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'--whole-window\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"whole_window\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'--exclude-titlebar\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"exclude_titlebar\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Mod1\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Mod2\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Mod3\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Mod4\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Mod5\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Shift\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Control\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Ctrl\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Mode_switch\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Group1\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Group2\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Group3\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'Group4\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'$mod\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"modifiers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'+\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"word\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"key\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        next_state: BINDCOMMAND,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_INCLUDE: [cmdp_token; 1] = [token {
    name: b"string\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"pattern\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 150 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_INITIAL: [cmdp_token; 60] = [
    token {
        name: b"end\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: INITIAL,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"error\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: INITIAL,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'#\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: IGNORE_LINE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'set \0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: IGNORE_LINE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'set\t\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: IGNORE_LINE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'set_from_resource\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: IGNORE_LINE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'include\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: INCLUDE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'bindsym\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"bindtype\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'bindcode\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"bindtype\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'bind\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"bindtype\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'bar\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BARBRACE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'font\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: FONT,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'mode\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: MODENAME,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'gaps\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: GAPS,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'smart_borders\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: SMART_BORDERS,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'smart_gaps\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: SMART_GAPS,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'floating_minimum_size\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: FLOATING_MINIMUM_SIZE_WIDTH,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'floating_maximum_size\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: FLOATING_MAXIMUM_SIZE_WIDTH,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'floating_modifier\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: FLOATING_MODIFIER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'default_orientation\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: DEFAULT_ORIENTATION,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'workspace_layout\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: WORKSPACE_LAYOUT,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'default_border\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"windowtype\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: DEFAULT_BORDER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'new_window\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"windowtype\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: DEFAULT_BORDER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'default_floating_border\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"windowtype\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: DEFAULT_BORDER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'new_float\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"windowtype\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: DEFAULT_BORDER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'hide_edge_borders\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: HIDE_EDGE_BORDERS,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'for_window\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: FOR_WINDOW,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'assign\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ASSIGN,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'no_focus\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: NO_FOCUS,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'focus_follows_mouse\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: FOCUS_FOLLOWS_MOUSE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'mouse_warping\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: MOUSE_WARPING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'focus_wrapping\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: FOCUS_WRAPPING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'force_focus_wrapping\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: FORCE_FOCUS_WRAPPING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'force_xinerama\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: FORCE_XINERAMA,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'force-xinerama\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: FORCE_XINERAMA,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'disable_randr15\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: DISABLE_RANDR15,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'disable-randr15\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: DISABLE_RANDR15,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'workspace_auto_back_and_forth\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: WORKSPACE_BACK_AND_FORTH,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'fake_outputs\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: FAKE_OUTPUTS,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'fake-outputs\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: FAKE_OUTPUTS,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'force_display_urgency_hint\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: FORCE_DISPLAY_URGENCY_HINT,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'focus_on_window_activation\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: FOCUS_ON_WINDOW_ACTIVATION,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'title_align\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: TITLE_ALIGN,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'show_marks\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: SHOW_MARKS,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'workspace\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: WORKSPACE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'ipc_socket\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: IPC_SOCKET,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'ipc-socket\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: IPC_SOCKET,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'ipc_kill_timeout\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: IPC_KILL_TIMEOUT,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'restart_state\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: RESTART_STATE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'popup_during_fullscreen\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: POPUP_DURING_FULLSCREEN,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'tiling_drag\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: TILING_DRAG,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'exec_always\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"exectype\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: EXEC,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'exec\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"exectype\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: EXEC,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'client.background\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"colorclass\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: COLOR_SINGLE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'client.focused_inactive\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"colorclass\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: COLOR_BORDER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'client.focused_tab_title\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"colorclass\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: COLOR_BORDER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'client.focused\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"colorclass\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: COLOR_BORDER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'client.unfocused\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"colorclass\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: COLOR_BORDER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'client.urgent\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"colorclass\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: COLOR_BORDER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'client.placeholder\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"colorclass\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: COLOR_BORDER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_ASSIGN: [cmdp_token; 1] = [token {
    name: b"'[\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 151 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_ID: [cmdp_token; 1] = [token {
    name: b"word\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"bar_id\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 152 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_EXEC: [cmdp_token; 2] = [
    token {
        name: b"'--no-startup-id\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"no_startup_id\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: EXEC,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"string\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"command\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 153 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_FONT: [cmdp_token; 1] = [token {
    name: b"string\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    identifier: b"font\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 154 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_GAPS: [cmdp_token; 8] = [
    token {
        name: b"'inner\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"scope\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: GAPS_WITH_SCOPE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'outer\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"scope\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: GAPS_WITH_SCOPE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'horizontal\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: b"scope\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: GAPS_WITH_SCOPE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'vertical\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"scope\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: GAPS_WITH_SCOPE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'top\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"scope\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: GAPS_WITH_SCOPE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'right\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"scope\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: GAPS_WITH_SCOPE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'bottom\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"scope\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: GAPS_WITH_SCOPE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'left\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"scope\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: GAPS_WITH_SCOPE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_MODE: [cmdp_token; 8] = [
    token {
        name: b"end\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: MODE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"error\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: MODE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'#\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: MODE_IGNORE_LINE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'set\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: MODE_IGNORE_LINE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'bindsym\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"bindtype\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'bindcode\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"bindtype\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'bind\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: b"bindtype\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'}\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: INITIAL,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_BAR: [cmdp_token; 31] = [
    token {
        name: b"end\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"error\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'#\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_IGNORE_LINE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'set\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_IGNORE_LINE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'i3bar_command\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_BAR_COMMAND,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'status_command\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_STATUS_COMMAND,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'workspace_command\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_WORKSPACE_COMMAND,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'socket_path\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_SOCKET_PATH,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'mode\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_MODE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'hidden_state\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_HIDDEN_STATE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'id\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_ID,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'modifier\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_MODIFIER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'wheel_up_cmd\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_WHEEL_UP_CMD,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'wheel_down_cmd\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_WHEEL_DOWN_CMD,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'bindsym\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_BINDSYM,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'position\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_POSITION,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'output\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_OUTPUT,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'tray_output\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_TRAY_OUTPUT,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'tray_padding\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_TRAY_PADDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'font\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_FONT,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'separator_symbol\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_SEPARATOR_SYMBOL,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'binding_mode_indicator\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_BINDING_MODE_INDICATOR,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'workspace_buttons\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_WORKSPACE_BUTTONS,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'workspace_min_width\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_WORKSPACE_MIN_WIDTH,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'strip_workspace_numbers\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_STRIP_WORKSPACE_NUMBERS,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'strip_workspace_name\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_STRIP_WORKSPACE_NAME,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'verbose\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_VERBOSE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'height\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_HEIGHT,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'padding\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_PADDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'colors\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_COLORS_BRACE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: b"'}\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 155 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens: [cmdp_token_ptr; 117] = unsafe {
    [
        tokenptr {
            array: &raw const tokens_FORCE_DISPLAY_URGENCY_HINT_MS as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_FLOATING_MAXIMUM_SIZE_HEIGHT as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_FLOATING_MINIMUM_SIZE_HEIGHT as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_STRIP_WORKSPACE_NUMBERS as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_FLOATING_MAXIMUM_SIZE_WIDTH as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_FLOATING_MINIMUM_SIZE_WIDTH as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_BINDING_MODE_INDICATOR as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_WORKSPACE_MIN_WIDTH_PX as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_FOCUS_ON_WINDOW_ACTIVATION as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_FORCE_DISPLAY_URGENCY_HINT as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_TILING_DRAG_SWAP_MODIFIER as *mut cmdp_token,
            n: 10 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_STRIP_WORKSPACE_NAME as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_CRITERION_FROM_STR_START as *mut cmdp_token,
            n: 3 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_DEFAULT_BORDER_PIXELS_PX as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_WORKSPACE_BACK_AND_FORTH as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_ASSIGN_WORKSPACE_NUMBER as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_WORKSPACE_MIN_WIDTH as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_FLOATING_MAXIMUM_SIZE_X as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_FLOATING_MINIMUM_SIZE_X as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_POPUP_DURING_FULLSCREEN as *mut cmdp_token,
            n: 4 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_COLORS_IGNORE_LINE as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_CRITERION_FROM_STR_END as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_COLORS_BACKGROUND as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_WORKSPACE_BUTTONS as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_WORKSPACE_COMMAND as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_DEFAULT_BORDER_PIXELS as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_WORKSPACE_OUTPUT_WORD as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_SEPARATOR_SYMBOL as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_FORCE_FOCUS_WRAPPING as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_BINDSYM_COMMAND as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_TRAY_PADDING_PX as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_DEFAULT_ORIENTATION as *mut cmdp_token,
            n: 3 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_FOCUS_FOLLOWS_MOUSE as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_PADDING_BOTTOM as *mut cmdp_token,
            n: 3 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_STATUS_COMMAND as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_WHEEL_DOWN_CMD as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_COLOR_CHILD_BORDER as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_CRITERION_FROM_STR as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_FOR_WINDOW_COMMAND as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_COLORS_BORDER as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_COLORS_SINGLE as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_PADDING_RIGHT as *mut cmdp_token,
            n: 3 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_FLOATING_MODIFIER as *mut cmdp_token,
            n: 10 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_HIDE_EDGE_BORDERS as *mut cmdp_token,
            n: 12 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_WORKSPACE_COMMAND as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_ASSIGN_WORKSPACE as *mut cmdp_token,
            n: 5 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_COLORS_BRACE as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_HIDDEN_STATE as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_PADDING_LEFT as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_TRAY_PADDING as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_WHEEL_UP_CMD as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_COLOR_BACKGROUND as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_IPC_KILL_TIMEOUT as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_MODE_BINDCOMMAND as *mut cmdp_token,
            n: 5 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_MODE_IGNORE_LINE as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_TILING_DRAG_MODE as *mut cmdp_token,
            n: 3 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_WORKSPACE_LAYOUT as *mut cmdp_token,
            n: 4 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_BAR_COMMAND as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_COLORS_TEXT as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_IGNORE_LINE as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_PADDING_TOP as *mut cmdp_token,
            n: 3 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_SOCKET_PATH as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_TRAY_OUTPUT as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_COLOR_INDICATOR as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_DISABLE_RANDR15 as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_GAPS_WITH_SCOPE as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_CRITERION_FROM as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_DEFAULT_BORDER as *mut cmdp_token,
            n: 4 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_FOCUS_WRAPPING as *mut cmdp_token,
            n: 14 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_FORCE_XINERAMA as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_ASSIGN_OUTPUT as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_CRITERION_STR as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_MOUSE_WARPING as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_RESTART_STATE as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_SMART_BORDERS as *mut cmdp_token,
            n: 7 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_MODIFIER as *mut cmdp_token,
            n: 12 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_POSITION as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_COLOR_BORDER as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_COLOR_SINGLE as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_FAKE_OUTPUTS as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_MODE_BINDING as *mut cmdp_token,
            n: 20 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_NO_FOCUS_END as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_BINDSYM as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_PADDING as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_VERBOSE as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BINDCOMMAND as *mut cmdp_token,
            n: 5 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_IGNORE_LINE as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_TILING_DRAG as *mut cmdp_token,
            n: 9 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_TITLE_ALIGN as *mut cmdp_token,
            n: 3 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_COLORS as *mut cmdp_token,
            n: 15 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_HEIGHT as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_OUTPUT as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_COLOR_TEXT as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_FOR_WINDOW as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_IPC_SOCKET as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_SHOW_MARKS as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_SMART_GAPS as *mut cmdp_token,
            n: 13 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_CRITERION as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_MODEBRACE as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_WORKSPACE as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BARBRACE as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_FONT as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_MODE as *mut cmdp_token,
            n: 3 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_CRITERIA as *mut cmdp_token,
            n: 17 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_GAPS_END as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_MODENAME as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_NO_FOCUS as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BINDING as *mut cmdp_token,
            n: 20 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_INCLUDE as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_INITIAL as *mut cmdp_token,
            n: 60 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_ASSIGN as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_ID as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_EXEC as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_FONT as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_GAPS as *mut cmdp_token,
            n: 8 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_MODE as *mut cmdp_token,
            n: 8 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR as *mut cmdp_token,
            n: 31 as ::core::ffi::c_int,
        },
    ]
};
unsafe extern "C" fn GENERATED_call(
    mut current_match: *mut Match,
    mut stack: *mut stack,
    call_identifier: ::core::ffi::c_int,
    mut result: *mut ConfigResultIR,
) {
    match call_identifier {
        0 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_force_display_urgency_hint(
                current_match,
                result,
                parser_get_long(
                    stack,
                    b"duration_ms\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        1 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_floating_maximum_size(
                current_match,
                result,
                parser_get_long(stack, b"width\0" as *const u8 as *const ::core::ffi::c_char),
                parser_get_long(
                    stack,
                    b"height\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        2 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_floating_minimum_size(
                current_match,
                result,
                parser_get_long(stack, b"width\0" as *const u8 as *const ::core::ffi::c_char),
                parser_get_long(
                    stack,
                    b"height\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        3 => {
            (*result).next_state = BAR as ::core::ffi::c_int;
            cfg_bar_strip_workspace_numbers(
                current_match,
                result,
                parser_get_string(stack, b"value\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        4 => {
            (*result).next_state = BAR as ::core::ffi::c_int;
            cfg_bar_binding_mode_indicator(
                current_match,
                result,
                parser_get_string(stack, b"value\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        5 => {
            (*result).next_state = BAR as ::core::ffi::c_int;
            cfg_bar_workspace_min_width(
                current_match,
                result,
                parser_get_long(stack, b"width\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        6 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_focus_on_window_activation(
                current_match,
                result,
                parser_get_string(stack, b"mode\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        7 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_tiling_drag_swap_modifier(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"modifiers\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        8 => {
            (*result).next_state = BAR as ::core::ffi::c_int;
            cfg_bar_strip_workspace_name(
                current_match,
                result,
                parser_get_string(stack, b"value\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        9 => {
            (*result).next_state = CRITERIA as ::core::ffi::c_int;
            cfg_criteria_add(
                current_match,
                result,
                parser_get_string(stack, b"ctype\0" as *const u8 as *const ::core::ffi::c_char),
                parser_get_string(stack, b"kind\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        10 => {
            (*result).next_state = CRITERIA as ::core::ffi::c_int;
            cfg_criteria_add(
                current_match,
                result,
                parser_get_string(stack, b"ctype\0" as *const u8 as *const ::core::ffi::c_char),
                parser_get_string(stack, b"kind\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        11 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_default_border(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"windowtype\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                parser_get_string(
                    stack,
                    b"border\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                parser_get_long(stack, b"width\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        12 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_workspace_back_and_forth(
                current_match,
                result,
                parser_get_string(stack, b"value\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        13 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_assign(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"number\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                1 as ::core::ffi::c_int != 0,
            );
        }
        14 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_popup_during_fullscreen(
                current_match,
                result,
                parser_get_string(stack, b"value\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        15 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_popup_during_fullscreen(
                current_match,
                result,
                parser_get_string(stack, b"value\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        16 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_popup_during_fullscreen(
                current_match,
                result,
                parser_get_string(stack, b"value\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        17 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_popup_during_fullscreen(
                current_match,
                result,
                parser_get_string(stack, b"value\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        18 => {
            (*result).next_state = CRITERIA as ::core::ffi::c_int;
            cfg_criteria_add(
                current_match,
                result,
                parser_get_string(stack, b"ctype\0" as *const u8 as *const ::core::ffi::c_char),
                parser_get_string(stack, b"kind\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        19 => {
            (*result).next_state = BAR as ::core::ffi::c_int;
            cfg_bar_workspace_buttons(
                current_match,
                result,
                parser_get_string(stack, b"value\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        20 => {
            (*result).next_state = BAR as ::core::ffi::c_int;
            cfg_bar_workspace_command(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"command\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        21 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_default_border(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"windowtype\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                parser_get_string(
                    stack,
                    b"border\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                2 as ::core::ffi::c_long,
            );
        }
        22 => {
            (*result).next_state = WORKSPACE_OUTPUT_WORD as ::core::ffi::c_int;
            cfg_workspace(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"workspace\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                parser_get_string(
                    stack,
                    b"output\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        23 => {
            (*result).next_state = BAR as ::core::ffi::c_int;
            cfg_bar_separator_symbol(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"separator\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        24 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_force_focus_wrapping(
                current_match,
                result,
                parser_get_string(stack, b"value\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        25 => {
            (*result).next_state = BAR as ::core::ffi::c_int;
            cfg_bar_bindsym(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"button\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                parser_get_string(
                    stack,
                    b"release\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                parser_get_string(
                    stack,
                    b"command\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        26 => {
            (*result).next_state = BAR as ::core::ffi::c_int;
            cfg_bar_tray_padding(
                current_match,
                result,
                parser_get_long(
                    stack,
                    b"padding_px\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        27 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_default_orientation(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"orientation\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        28 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_default_orientation(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"orientation\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        29 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_default_orientation(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"orientation\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        30 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_focus_follows_mouse(
                current_match,
                result,
                parser_get_string(stack, b"value\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        31 => {
            (*result).next_state = BAR as ::core::ffi::c_int;
            cfg_bar_padding_three(
                current_match,
                result,
                parser_get_long(
                    stack,
                    b"top_or_all\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                parser_get_long(
                    stack,
                    b"right_or_right_and_left\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                parser_get_long(
                    stack,
                    b"bottom\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        32 => {
            (*result).next_state = BAR as ::core::ffi::c_int;
            cfg_bar_status_command(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"command\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        33 => {
            (*result).next_state = BAR as ::core::ffi::c_int;
            cfg_bar_wheel_down_cmd(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"command\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        34 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_color(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"colorclass\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                parser_get_string(
                    stack,
                    b"border\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                parser_get_string(
                    stack,
                    b"background\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                parser_get_string(stack, b"text\0" as *const u8 as *const ::core::ffi::c_char),
                parser_get_string(
                    stack,
                    b"indicator\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                parser_get_string(
                    stack,
                    b"child_border\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        35 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_color(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"colorclass\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                parser_get_string(
                    stack,
                    b"border\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                parser_get_string(
                    stack,
                    b"background\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                parser_get_string(stack, b"text\0" as *const u8 as *const ::core::ffi::c_char),
                parser_get_string(
                    stack,
                    b"indicator\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
        36 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_for_window(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"command\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        37 => {
            (*result).next_state = BAR_COLORS as ::core::ffi::c_int;
            cfg_bar_color_single(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"colorclass\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                parser_get_string(stack, b"color\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        38 => {
            (*result).next_state = BAR as ::core::ffi::c_int;
            cfg_bar_padding_two(
                current_match,
                result,
                parser_get_long(
                    stack,
                    b"top_or_all\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                parser_get_long(
                    stack,
                    b"right_or_right_and_left\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        39 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_floating_modifier(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"modifiers\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        40 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_hide_edge_borders(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"hide_borders\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        41 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_hide_edge_borders(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"hide_borders\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        42 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_hide_edge_borders(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"hide_borders\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        43 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_hide_edge_borders(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"hide_borders\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        44 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_hide_edge_borders(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"hide_borders\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        45 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_hide_edge_borders(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"hide_borders\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        46 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_hide_edge_borders(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"hide_borders\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        47 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_hide_edge_borders(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"hide_borders\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        48 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_hide_edge_borders(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"hide_borders\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        49 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_hide_edge_borders(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"hide_borders\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        50 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_hide_edge_borders(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"hide_borders\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        51 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_hide_edge_borders(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"hide_borders\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        52 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_assign(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"workspace\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                0 as ::core::ffi::c_int != 0,
            );
        }
        53 => {
            (*result).next_state = BAR as ::core::ffi::c_int;
            cfg_bar_hidden_state(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"hidden_state\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        54 => {
            (*result).next_state = BAR as ::core::ffi::c_int;
            cfg_bar_hidden_state(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"hidden_state\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        55 => {
            (*result).next_state = BAR as ::core::ffi::c_int;
            cfg_bar_padding_four(
                current_match,
                result,
                parser_get_long(
                    stack,
                    b"top_or_all\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                parser_get_long(
                    stack,
                    b"right_or_right_and_left\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                parser_get_long(
                    stack,
                    b"bottom\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                parser_get_long(stack, b"left\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        56 => {
            (*result).next_state = BAR as ::core::ffi::c_int;
            cfg_bar_wheel_up_cmd(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"command\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        57 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_ipc_kill_timeout(
                current_match,
                result,
                parser_get_long(
                    stack,
                    b"timeout\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        58 => {
            (*result).next_state = MODE as ::core::ffi::c_int;
            cfg_mode_binding(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"bindtype\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                parser_get_string(
                    stack,
                    b"modifiers\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                parser_get_string(stack, b"key\0" as *const u8 as *const ::core::ffi::c_char),
                parser_get_string(
                    stack,
                    b"release\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                parser_get_string(
                    stack,
                    b"border\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                parser_get_string(
                    stack,
                    b"whole_window\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                parser_get_string(
                    stack,
                    b"exclude_titlebar\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                parser_get_string(
                    stack,
                    b"command\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        59 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_tiling_drag(
                current_match,
                result,
                parser_get_string(stack, b"value\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        60 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_workspace_layout(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"layout\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        61 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_workspace_layout(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"layout\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        62 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_workspace_layout(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"layout\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        63 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_workspace_layout(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"layout\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        64 => {
            (*result).next_state = BAR as ::core::ffi::c_int;
            cfg_bar_i3bar_command(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"command\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        65 => {
            (*result).next_state = BAR_COLORS as ::core::ffi::c_int;
            cfg_bar_color(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"colorclass\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                parser_get_string(
                    stack,
                    b"border\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                parser_get_string(
                    stack,
                    b"background\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
        66 => {
            (*result).next_state = BAR_COLORS as ::core::ffi::c_int;
            cfg_bar_color(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"colorclass\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                parser_get_string(
                    stack,
                    b"border\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                parser_get_string(
                    stack,
                    b"background\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                parser_get_string(stack, b"text\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        67 => {
            (*result).next_state = BAR as ::core::ffi::c_int;
            cfg_bar_padding_one(
                current_match,
                result,
                parser_get_long(
                    stack,
                    b"top_or_all\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        68 => {
            (*result).next_state = BAR as ::core::ffi::c_int;
            cfg_bar_socket_path(
                current_match,
                result,
                parser_get_string(stack, b"path\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        69 => {
            (*result).next_state = BAR as ::core::ffi::c_int;
            cfg_bar_tray_output(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"output\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        70 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_color(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"colorclass\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                parser_get_string(
                    stack,
                    b"border\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                parser_get_string(
                    stack,
                    b"background\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                parser_get_string(stack, b"text\0" as *const u8 as *const ::core::ffi::c_char),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
        71 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_disable_randr15(
                current_match,
                result,
                parser_get_string(stack, b"value\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        72 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_default_border(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"windowtype\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                parser_get_string(
                    stack,
                    b"border\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                -(1 as ::core::ffi::c_int) as ::core::ffi::c_long,
            );
        }
        73 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_default_border(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"windowtype\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                parser_get_string(
                    stack,
                    b"border\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                -(1 as ::core::ffi::c_int) as ::core::ffi::c_long,
            );
        }
        74 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_focus_wrapping(
                current_match,
                result,
                parser_get_string(stack, b"value\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        75 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_focus_wrapping(
                current_match,
                result,
                parser_get_string(stack, b"value\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        76 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_focus_wrapping(
                current_match,
                result,
                parser_get_string(stack, b"value\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        77 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_focus_wrapping(
                current_match,
                result,
                parser_get_string(stack, b"value\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        78 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_focus_wrapping(
                current_match,
                result,
                parser_get_string(stack, b"value\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        79 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_focus_wrapping(
                current_match,
                result,
                parser_get_string(stack, b"value\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        80 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_focus_wrapping(
                current_match,
                result,
                parser_get_string(stack, b"value\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        81 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_focus_wrapping(
                current_match,
                result,
                parser_get_string(stack, b"value\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        82 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_focus_wrapping(
                current_match,
                result,
                parser_get_string(stack, b"value\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        83 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_focus_wrapping(
                current_match,
                result,
                parser_get_string(stack, b"value\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        84 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_focus_wrapping(
                current_match,
                result,
                parser_get_string(stack, b"value\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        85 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_focus_wrapping(
                current_match,
                result,
                parser_get_string(stack, b"value\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        86 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_focus_wrapping(
                current_match,
                result,
                parser_get_string(stack, b"value\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        87 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_focus_wrapping(
                current_match,
                result,
                parser_get_string(stack, b"value\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        88 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_force_xinerama(
                current_match,
                result,
                parser_get_string(stack, b"value\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        89 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_assign_output(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"output\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        90 => {
            (*result).next_state = CRITERIA as ::core::ffi::c_int;
            cfg_criteria_add(
                current_match,
                result,
                parser_get_string(stack, b"ctype\0" as *const u8 as *const ::core::ffi::c_char),
                parser_get_string(
                    stack,
                    b"cvalue\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        91 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_mouse_warping(
                current_match,
                result,
                parser_get_string(stack, b"value\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        92 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_mouse_warping(
                current_match,
                result,
                parser_get_string(stack, b"value\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        93 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_restart_state(
                current_match,
                result,
                parser_get_string(stack, b"path\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        94 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_smart_borders(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"enabled\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        95 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_smart_borders(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"enabled\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        96 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_smart_borders(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"enabled\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        97 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_smart_borders(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"enabled\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        98 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_smart_borders(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"enabled\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        99 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_smart_borders(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"enabled\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        100 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_smart_borders(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"enabled\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        101 => {
            (*result).next_state = BAR as ::core::ffi::c_int;
            cfg_bar_modifier(
                current_match,
                result,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
        102 => {
            (*result).next_state = BAR as ::core::ffi::c_int;
            cfg_bar_modifier(
                current_match,
                result,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
        103 => {
            (*result).next_state = BAR as ::core::ffi::c_int;
            cfg_bar_modifier(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"modifiers\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        104 => {
            (*result).next_state = BAR as ::core::ffi::c_int;
            cfg_bar_position(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"position\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        105 => {
            (*result).next_state = BAR as ::core::ffi::c_int;
            cfg_bar_position(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"position\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        106 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_color_single(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"colorclass\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                parser_get_string(stack, b"color\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        107 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_fake_outputs(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"outputs\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        108 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_no_focus(current_match, result);
        }
        109 => {
            (*result).next_state = BAR as ::core::ffi::c_int;
            cfg_bar_verbose(
                current_match,
                result,
                parser_get_string(stack, b"value\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        110 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_binding(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"bindtype\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                parser_get_string(
                    stack,
                    b"modifiers\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                parser_get_string(stack, b"key\0" as *const u8 as *const ::core::ffi::c_char),
                parser_get_string(
                    stack,
                    b"release\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                parser_get_string(
                    stack,
                    b"border\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                parser_get_string(
                    stack,
                    b"whole_window\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                parser_get_string(
                    stack,
                    b"exclude_titlebar\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                parser_get_string(
                    stack,
                    b"command\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        111 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_tiling_drag(
                current_match,
                result,
                parser_get_string(stack, b"off\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        112 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_tiling_drag(
                current_match,
                result,
                parser_get_string(stack, b"off\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        113 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_tiling_drag(
                current_match,
                result,
                parser_get_string(stack, b"off\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        114 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_tiling_drag(
                current_match,
                result,
                parser_get_string(stack, b"off\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        115 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_tiling_drag(
                current_match,
                result,
                parser_get_string(stack, b"off\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        116 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_tiling_drag(
                current_match,
                result,
                parser_get_string(stack, b"off\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        117 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_title_align(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"alignment\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        118 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_title_align(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"alignment\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        119 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_title_align(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"alignment\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        120 => {
            (*result).next_state = BAR as ::core::ffi::c_int;
            cfg_bar_height(
                current_match,
                result,
                parser_get_long(stack, b"value\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        121 => {
            (*result).next_state = BAR as ::core::ffi::c_int;
            cfg_bar_output(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"output\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        122 => {
            (*result).next_state = CRITERIA as ::core::ffi::c_int;
            cfg_criteria_init(current_match, result, 38 as ::core::ffi::c_int);
        }
        123 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_ipc_socket(
                current_match,
                result,
                parser_get_string(stack, b"path\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        124 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_show_marks(
                current_match,
                result,
                parser_get_string(stack, b"value\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        125 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_smart_gaps(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"enabled\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        126 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_smart_gaps(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"enabled\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        127 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_smart_gaps(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"enabled\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        128 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_smart_gaps(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"enabled\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        129 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_smart_gaps(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"enabled\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        130 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_smart_gaps(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"enabled\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        131 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_smart_gaps(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"enabled\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        132 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_smart_gaps(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"enabled\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        133 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_smart_gaps(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"enabled\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        134 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_smart_gaps(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"enabled\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        135 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_smart_gaps(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"enabled\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        136 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_smart_gaps(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"enabled\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        137 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_smart_gaps(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"enabled\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        138 => {
            (*result).next_state = BAR as ::core::ffi::c_int;
            cfg_bar_start(current_match, result);
        }
        139 => {
            (*result).next_state = BAR as ::core::ffi::c_int;
            cfg_bar_font(
                current_match,
                result,
                parser_get_string(stack, b"font\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        140 => {
            (*result).next_state = BAR as ::core::ffi::c_int;
            cfg_bar_mode(
                current_match,
                result,
                parser_get_string(stack, b"mode\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        141 => {
            (*result).next_state = BAR as ::core::ffi::c_int;
            cfg_bar_mode(
                current_match,
                result,
                parser_get_string(stack, b"mode\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        142 => {
            (*result).next_state = BAR as ::core::ffi::c_int;
            cfg_bar_mode(
                current_match,
                result,
                parser_get_string(stack, b"mode\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        143 => {
            (*result).next_state = CRITERIA as ::core::ffi::c_int;
            cfg_criteria_add(
                current_match,
                result,
                parser_get_string(stack, b"ctype\0" as *const u8 as *const ::core::ffi::c_char),
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
        144 => {
            (*result).next_state = CRITERIA as ::core::ffi::c_int;
            cfg_criteria_add(
                current_match,
                result,
                parser_get_string(stack, b"ctype\0" as *const u8 as *const ::core::ffi::c_char),
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
        145 => {
            (*result).next_state = CRITERIA as ::core::ffi::c_int;
            cfg_criteria_add(
                current_match,
                result,
                parser_get_string(stack, b"ctype\0" as *const u8 as *const ::core::ffi::c_char),
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
        146 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_criteria_pop_state(current_match, result);
        }
        147 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_gaps(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"workspace\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                parser_get_string(stack, b"scope\0" as *const u8 as *const ::core::ffi::c_char),
                parser_get_long(stack, b"value\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        148 => {
            (*result).next_state = MODEBRACE as ::core::ffi::c_int;
            cfg_enter_mode(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"pango_markup\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                parser_get_string(
                    stack,
                    b"modename\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        149 => {
            (*result).next_state = CRITERIA as ::core::ffi::c_int;
            cfg_criteria_init(current_match, result, 81 as ::core::ffi::c_int);
        }
        150 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_include(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"pattern\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        151 => {
            (*result).next_state = CRITERIA as ::core::ffi::c_int;
            cfg_criteria_init(current_match, result, 45 as ::core::ffi::c_int);
        }
        152 => {
            (*result).next_state = BAR as ::core::ffi::c_int;
            cfg_bar_id(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"bar_id\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        153 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_exec(
                current_match,
                result,
                parser_get_string(
                    stack,
                    b"exectype\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                parser_get_string(
                    stack,
                    b"no_startup_id\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                parser_get_string(
                    stack,
                    b"command\0" as *const u8 as *const ::core::ffi::c_char,
                ),
            );
        }
        154 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_font(
                current_match,
                result,
                parser_get_string(stack, b"font\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        155 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_bar_finish(current_match, result);
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
                    b"./GENERATED_config_call.h\0" as *const u8
                        as *const ::core::ffi::c_char,
                    1265 as ::core::ffi::c_uint,
                    b"void GENERATED_call(Match *, struct stack *, const int, struct ConfigResultIR *)\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            };
        }
    };
}
unsafe extern "C" fn next_state(mut token: *const cmdp_token, mut ctx: *mut parser_ctx) {
    let mut _next_state: cmdp_state = (*token).next_state;
    if (*token).next_state as ::core::ffi::c_uint
        == __CALL as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut subcommand_output: ConfigResultIR = ConfigResultIR {
            ctx: ctx,
            next_state: 0,
            has_errors: false,
        };
        GENERATED_call(
            &raw mut (*ctx).current_match,
            (*ctx).stack,
            (*token).extra.call_identifier as ::core::ffi::c_int,
            &raw mut subcommand_output,
        );
        if subcommand_output.has_errors {
            (*ctx).has_errors = true_0 != 0;
        }
        _next_state = subcommand_output.next_state as cmdp_state;
        parser_clear_stack((*ctx).stack);
    }
    (*ctx).state = _next_state as ::core::ffi::c_int;
    if (*ctx).state == INITIAL as ::core::ffi::c_int {
        parser_clear_stack((*ctx).stack);
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*ctx).statelist_idx {
        if (*ctx).statelist[i as usize] as cmdp_state as ::core::ffi::c_uint
            != _next_state as ::core::ffi::c_uint
        {
            i += 1;
        } else {
            (*ctx).statelist_idx = i + 1 as ::core::ffi::c_int;
            return;
        }
    }
    let fresh7 = (*ctx).statelist_idx;
    (*ctx).statelist_idx = (*ctx).statelist_idx + 1;
    (*ctx).statelist[fresh7 as usize] = _next_state as ::core::ffi::c_int;
}
unsafe extern "C" fn start_of_line(
    mut walk: *const ::core::ffi::c_char,
    mut beginning: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    while walk >= beginning
        && *walk as ::core::ffi::c_int != '\n' as i32
        && *walk as ::core::ffi::c_int != '\r' as i32
    {
        walk = walk.offset(-1);
    }
    return walk.offset(1 as ::core::ffi::c_int as isize);
}
unsafe extern "C" fn single_line(
    mut start: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut result: *mut ::core::ffi::c_char = sstrdup(start);
    let mut end: *mut ::core::ffi::c_char = strchr(result, '\n' as i32);
    if !end.is_null() {
        *end = '\0' as i32 as ::core::ffi::c_char;
    }
    return result;
}
unsafe extern "C" fn parse_config(
    mut ctx: *mut parser_ctx,
    mut input: *const ::core::ffi::c_char,
    mut context: *mut context,
) {
    let mut dumpwalk: *const ::core::ffi::c_char = input;
    let mut linecnt: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while *dumpwalk as ::core::ffi::c_int != '\0' as i32 {
        let mut next_nl: *const ::core::ffi::c_char = strchr(dumpwalk, '\n' as i32);
        if !next_nl.is_null() {
            debuglog(
                b"%s:%s:%d - CONFIG(line %3d): %.*s\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"../src/config_parser.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"parse_config\0" as *const u8 as *const ::core::ffi::c_char,
                151 as ::core::ffi::c_int,
                linecnt,
                next_nl.offset_from(dumpwalk) as ::core::ffi::c_long as ::core::ffi::c_int,
                dumpwalk,
            );
            dumpwalk = next_nl.offset(1 as ::core::ffi::c_int as isize);
            linecnt += 1;
        } else {
            debuglog(
                b"%s:%s:%d - CONFIG(line %3d): %s\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"../src/config_parser.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"parse_config\0" as *const u8 as *const ::core::ffi::c_char,
                154 as ::core::ffi::c_int,
                linecnt,
                dumpwalk,
            );
            break;
        }
    }
    (*ctx).state = INITIAL as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < 10 as ::core::ffi::c_int {
        (*ctx).statelist[i as usize] = INITIAL as ::core::ffi::c_int;
        i += 1;
    }
    (*ctx).statelist_idx = 1 as ::core::ffi::c_int;
    let mut walk: *const ::core::ffi::c_char = input;
    let len: size_t = strlen(input) as size_t;
    let mut c: ::core::ffi::c_int = 0;
    let mut token: *const cmdp_token = ::core::ptr::null::<cmdp_token>();
    let mut token_handled: bool = false;
    linecnt = 1 as ::core::ffi::c_int;
    let mut subcommand_output: ConfigResultIR = ConfigResultIR {
        ctx: ctx,
        next_state: 0,
        has_errors: false,
    };
    cfg_criteria_init(
        &raw mut (*ctx).current_match,
        &raw mut subcommand_output,
        INITIAL as ::core::ffi::c_int,
    );
    while walk.offset_from(input) as ::core::ffi::c_long as size_t <= len {
        while (*walk as ::core::ffi::c_int == ' ' as i32
            || *walk as ::core::ffi::c_int == '\t' as i32)
            && *walk as ::core::ffi::c_int != '\0' as i32
        {
            walk = walk.offset(1);
        }
        let mut ptr: *const cmdp_token_ptr = (&raw mut tokens as *mut cmdp_token_ptr)
            .offset((*ctx).state as isize)
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
                            (*ctx).stack,
                            (*token).identifier,
                            (*token).name.offset(1 as ::core::ffi::c_int as isize),
                        );
                    }
                    walk = walk.offset(strlen((*token).name).wrapping_sub(1 as size_t) as isize);
                    next_state(token, ctx);
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
                            parser_push_long((*ctx).stack, (*token).identifier, num);
                        }
                        walk = end;
                        next_state(token, ctx);
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
                    let mut beginning: *const ::core::ffi::c_char = walk;
                    if *walk as ::core::ffi::c_int == '"' as i32 {
                        beginning = beginning.offset(1);
                        walk = walk.offset(1);
                        while *walk as ::core::ffi::c_int != '\0' as i32
                            && (*walk as ::core::ffi::c_int != '"' as i32
                                || *walk.offset(-(1 as ::core::ffi::c_int as isize))
                                    as ::core::ffi::c_int
                                    == '\\' as i32)
                        {
                            walk = walk.offset(1);
                        }
                    } else if *(*token).name.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        == 's' as i32
                    {
                        while *walk as ::core::ffi::c_int != '\0' as i32
                            && *walk as ::core::ffi::c_int != '\r' as i32
                            && *walk as ::core::ffi::c_int != '\n' as i32
                        {
                            walk = walk.offset(1);
                        }
                    } else {
                        while *walk as ::core::ffi::c_int != ' ' as i32
                            && *walk as ::core::ffi::c_int != '\t' as i32
                            && *walk as ::core::ffi::c_int != ']' as i32
                            && *walk as ::core::ffi::c_int != ',' as i32
                            && *walk as ::core::ffi::c_int != ';' as i32
                            && *walk as ::core::ffi::c_int != '\r' as i32
                            && *walk as ::core::ffi::c_int != '\n' as i32
                            && *walk as ::core::ffi::c_int != '\0' as i32
                        {
                            walk = walk.offset(1);
                        }
                    }
                    if walk != beginning {
                        let mut str: *mut ::core::ffi::c_char = scalloc(
                            (walk.offset_from(beginning) as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long) as size_t,
                            1 as size_t,
                        )
                            as *mut ::core::ffi::c_char;
                        let mut inpos: ::core::ffi::c_int = 0;
                        let mut outpos: ::core::ffi::c_int = 0;
                        inpos = 0 as ::core::ffi::c_int;
                        outpos = 0 as ::core::ffi::c_int;
                        while (inpos as ::core::ffi::c_long)
                            < walk.offset_from(beginning) as ::core::ffi::c_long
                        {
                            if *beginning.offset(inpos as isize) as ::core::ffi::c_int
                                == '\\' as i32
                                && *beginning.offset((inpos + 1 as ::core::ffi::c_int) as isize)
                                    as ::core::ffi::c_int
                                    == '"' as i32
                            {
                                inpos += 1;
                            }
                            *str.offset(outpos as isize) = *beginning.offset(inpos as isize);
                            inpos += 1;
                            outpos += 1;
                        }
                        if !(*token).identifier.is_null() {
                            parser_push_string((*ctx).stack, (*token).identifier, str);
                        }
                        free(str as *mut ::core::ffi::c_void);
                        if *walk as ::core::ffi::c_int == '"' as i32 {
                            walk = walk.offset(1);
                        }
                        next_state(token, ctx);
                        token_handled = true_0 != 0;
                        break;
                    }
                }
                if strcmp(
                    (*token).name,
                    b"line\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    while *walk as ::core::ffi::c_int != '\0' as i32
                        && *walk as ::core::ffi::c_int != '\n' as i32
                        && *walk as ::core::ffi::c_int != '\r' as i32
                    {
                        walk = walk.offset(1);
                    }
                    next_state(token, ctx);
                    token_handled = true_0 != 0;
                    linecnt += 1;
                    walk = walk.offset(1);
                    break;
                } else if strcmp(
                    (*token).name,
                    b"end\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    if *walk as ::core::ffi::c_int == '\0' as i32
                        || *walk as ::core::ffi::c_int == '\n' as i32
                        || *walk as ::core::ffi::c_int == '\r' as i32
                    {
                        next_state(token, ctx);
                        token_handled = true_0 != 0;
                        cfg_criteria_init(
                            &raw mut (*ctx).current_match,
                            &raw mut subcommand_output,
                            INITIAL as ::core::ffi::c_int,
                        );
                        linecnt += 1;
                        walk = walk.offset(1);
                        break;
                    }
                }
            }
            c += 1;
        }
        if !token_handled {
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
            let mut current_block_96: u64;
            c = 0 as ::core::ffi::c_int;
            while c < (*ptr).n {
                token = (*ptr).array.offset(c as isize) as *mut cmdp_token;
                if *(*token).name.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '\'' as i32
                {
                    let fresh1 = tokenwalk;
                    tokenwalk = tokenwalk.offset(1);
                    *fresh1 = '\'' as i32 as ::core::ffi::c_char;
                    strcpy(
                        tokenwalk,
                        (*token).name.offset(1 as ::core::ffi::c_int as isize),
                    );
                    tokenwalk = tokenwalk.offset(strlen(
                        (*token).name.offset(1 as ::core::ffi::c_int as isize),
                    ) as isize);
                    let fresh2 = tokenwalk;
                    tokenwalk = tokenwalk.offset(1);
                    *fresh2 = '\'' as i32 as ::core::ffi::c_char;
                    current_block_96 = 796174441944384681;
                } else if strcmp(
                    (*token).name,
                    b"error\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    current_block_96 = 14541395414537699361;
                } else {
                    let fresh3 = tokenwalk;
                    tokenwalk = tokenwalk.offset(1);
                    *fresh3 = '<' as i32 as ::core::ffi::c_char;
                    strcpy(tokenwalk, (*token).name);
                    tokenwalk = tokenwalk.offset(strlen((*token).name) as isize);
                    let fresh4 = tokenwalk;
                    tokenwalk = tokenwalk.offset(1);
                    *fresh4 = '>' as i32 as ::core::ffi::c_char;
                    current_block_96 = 796174441944384681;
                }
                match current_block_96 {
                    796174441944384681 => {
                        if c < (*ptr).n - 1 as ::core::ffi::c_int {
                            let fresh5 = tokenwalk;
                            tokenwalk = tokenwalk.offset(1);
                            *fresh5 = ',' as i32 as ::core::ffi::c_char;
                            let fresh6 = tokenwalk;
                            tokenwalk = tokenwalk.offset(1);
                            *fresh6 = ' ' as i32 as ::core::ffi::c_char;
                        }
                    }
                    _ => {}
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
            let mut error_line: *const ::core::ffi::c_char = start_of_line(walk, input);
            let mut position: *mut ::core::ffi::c_char =
                scalloc(strlen(error_line).wrapping_add(1 as size_t), 1 as size_t)
                    as *mut ::core::ffi::c_char;
            let mut copywalk: *const ::core::ffi::c_char =
                ::core::ptr::null::<::core::ffi::c_char>();
            copywalk = error_line;
            while *copywalk as ::core::ffi::c_int != '\n' as i32
                && *copywalk as ::core::ffi::c_int != '\r' as i32
                && *copywalk as ::core::ffi::c_int != '\0' as i32
            {
                *position
                    .offset(copywalk.offset_from(error_line) as ::core::ffi::c_long as isize) =
                    (if copywalk >= walk {
                        '^' as i32
                    } else if *copywalk as ::core::ffi::c_int == '\t' as i32 {
                        '\t' as i32
                    } else {
                        ' ' as i32
                    }) as ::core::ffi::c_char;
                copywalk = copywalk.offset(1);
            }
            *position.offset(copywalk.offset_from(error_line) as ::core::ffi::c_long as isize) =
                '\0' as i32 as ::core::ffi::c_char;
            errorlog(
                b"ERROR: CONFIG: %s\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                errormessage,
            );
            errorlog(
                b"ERROR: CONFIG: (in file %s)\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                (*context).filename,
            );
            let mut error_copy: *mut ::core::ffi::c_char = single_line(error_line);
            if linecnt > 1 as ::core::ffi::c_int {
                let mut context_p1_start: *const ::core::ffi::c_char = start_of_line(
                    error_line.offset(-(2 as ::core::ffi::c_int as isize)),
                    input,
                );
                let mut context_p1_line: *mut ::core::ffi::c_char = single_line(context_p1_start);
                if linecnt > 2 as ::core::ffi::c_int {
                    let mut context_p2_start: *const ::core::ffi::c_char = start_of_line(
                        context_p1_start.offset(-(2 as ::core::ffi::c_int as isize)),
                        input,
                    );
                    let mut context_p2_line: *mut ::core::ffi::c_char =
                        single_line(context_p2_start);
                    errorlog(
                        b"ERROR: CONFIG: Line %3d: %s\n\0" as *const u8
                            as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        linecnt - 2 as ::core::ffi::c_int,
                        context_p2_line,
                    );
                    free(context_p2_line as *mut ::core::ffi::c_void);
                }
                errorlog(
                    b"ERROR: CONFIG: Line %3d: %s\n\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    linecnt - 1 as ::core::ffi::c_int,
                    context_p1_line,
                );
                free(context_p1_line as *mut ::core::ffi::c_void);
            }
            errorlog(
                b"ERROR: CONFIG: Line %3d: %s\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                linecnt,
                error_copy,
            );
            errorlog(
                b"ERROR: CONFIG:           %s\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                position,
            );
            free(error_copy as *mut ::core::ffi::c_void);
            let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_0 < 2 as ::core::ffi::c_int {
                let mut error_line_end: *const ::core::ffi::c_char =
                    strchr(error_line, '\n' as i32);
                if !error_line_end.is_null()
                    && *error_line_end.offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        != '\0' as i32
                {
                    error_line = error_line_end.offset(1 as ::core::ffi::c_int as isize);
                    error_copy = single_line(error_line);
                    errorlog(
                        b"ERROR: CONFIG: Line %3d: %s\n\0" as *const u8
                            as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        linecnt + i_0 + 1 as ::core::ffi::c_int,
                        error_copy,
                    );
                    free(error_copy as *mut ::core::ffi::c_void);
                }
                i_0 += 1;
            }
            (*context).has_errors = true_0 != 0;
            while walk.offset_from(input) as ::core::ffi::c_long as size_t <= len
                && *walk as ::core::ffi::c_int != '\n' as i32
            {
                walk = walk.offset(1);
            }
            free(position as *mut ::core::ffi::c_void);
            free(errormessage as *mut ::core::ffi::c_void);
            parser_clear_stack((*ctx).stack);
            let mut error_token_found: bool = false_0 != 0;
            let mut i_1: ::core::ffi::c_int = (*ctx).statelist_idx - 1 as ::core::ffi::c_int;
            while i_1 >= 0 as ::core::ffi::c_int && !error_token_found {
                let mut errptr: *const cmdp_token_ptr =
                    (&raw mut tokens as *mut cmdp_token_ptr).offset(
                        *(&raw mut (*ctx).statelist as *mut ::core::ffi::c_int).offset(i_1 as isize)
                            as isize,
                    ) as *mut cmdp_token_ptr;
                let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while j < (*errptr).n {
                    if strcmp(
                        (*(*errptr).array.offset(j as isize)).name,
                        b"error\0" as *const u8 as *const ::core::ffi::c_char,
                    ) != 0 as ::core::ffi::c_int
                    {
                        j += 1;
                    } else {
                        next_state((*errptr).array.offset(j as isize) as *mut cmdp_token, ctx);
                        error_token_found = true_0 != 0;
                        break;
                    }
                }
                i_1 -= 1;
            }
            if error_token_found as ::core::ffi::c_int != 0 {
            } else {
                __assert_fail(
                    b"error_token_found\0" as *const u8 as *const ::core::ffi::c_char,
                    b"../src/config_parser.c\0" as *const u8 as *const ::core::ffi::c_char,
                    437 as ::core::ffi::c_uint,
                    b"void parse_config(struct parser_ctx *, const char *, struct context *)\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            };
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn start_config_error_nagbar(
    mut configpath: *const ::core::ffi::c_char,
    has_errors: bool,
) {
    let mut editaction: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut pageraction: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    sasprintf(
        &raw mut editaction,
        b"i3-sensible-editor \"%s\" && i3-msg reload\n\0" as *const u8
            as *const ::core::ffi::c_char,
        configpath,
    );
    sasprintf(
        &raw mut pageraction,
        b"i3-sensible-pager \"%s\"\n\0" as *const u8 as *const ::core::ffi::c_char,
        errorfilename,
    );
    let mut argv: [*mut ::core::ffi::c_char; 14] = [
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
        b"-f\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        (if !config.font.pattern.is_null() {
            config.font.pattern as *const ::core::ffi::c_char
        } else {
            b"fixed\0" as *const u8 as *const ::core::ffi::c_char
        }) as *mut ::core::ffi::c_char,
        b"-t\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        (if has_errors as ::core::ffi::c_int != 0 {
            b"error\0" as *const u8 as *const ::core::ffi::c_char
        } else {
            b"warning\0" as *const u8 as *const ::core::ffi::c_char
        }) as *mut ::core::ffi::c_char,
        b"-m\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        (if has_errors as ::core::ffi::c_int != 0 {
            b"You have an error in your i3 config file!\0" as *const u8
                as *const ::core::ffi::c_char
        } else {
            b"Your config is outdated. Please fix the warnings to make sure everything works.\0"
                as *const u8 as *const ::core::ffi::c_char
        }) as *mut ::core::ffi::c_char,
        b"-b\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"edit config\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        editaction,
        (if !errorfilename.is_null() {
            b"-b\0" as *const u8 as *const ::core::ffi::c_char
        } else {
            ::core::ptr::null::<::core::ffi::c_char>()
        }) as *mut ::core::ffi::c_char,
        (if has_errors as ::core::ffi::c_int != 0 {
            b"show errors\0" as *const u8 as *const ::core::ffi::c_char
        } else {
            b"show warnings\0" as *const u8 as *const ::core::ffi::c_char
        }) as *mut ::core::ffi::c_char,
        pageraction,
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
    ];
    start_nagbar(
        &raw mut config_error_nagbar_pid,
        &raw mut argv as *mut *mut ::core::ffi::c_char,
    );
    free(editaction as *mut ::core::ffi::c_void);
    free(pageraction as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn upsert_variable(
    mut variables: *mut variables_head,
    mut key: *mut ::core::ffi::c_char,
    mut value: *mut ::core::ffi::c_char,
) {
    let mut current: *mut Variable = ::core::ptr::null_mut::<Variable>();
    current = (*variables).slh_first;
    while !current.is_null() {
        if strcmp((*current).key, key) != 0 as ::core::ffi::c_int {
            current = (*current).variables.sle_next;
        } else {
            debuglog(
                b"%s:%s:%d - Updated variable: %s = %s -> %s\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"../src/config_parser.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"upsert_variable\0" as *const u8 as *const ::core::ffi::c_char,
                543 as ::core::ffi::c_int,
                key,
                (*current).value,
                value,
            );
            free((*current).value as *mut ::core::ffi::c_void);
            (*current).value = ::core::ptr::null_mut::<::core::ffi::c_char>();
            (*current).value = sstrdup(value);
            return;
        }
    }
    debuglog(
        b"%s:%s:%d - Defined new variable: %s = %s\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/config_parser.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"upsert_variable\0" as *const u8 as *const ::core::ffi::c_char,
        549 as ::core::ffi::c_int,
        key,
        value,
    );
    let mut new: *mut Variable =
        scalloc(1 as size_t, ::core::mem::size_of::<Variable>() as size_t) as *mut Variable;
    let mut test: *mut Variable = ::core::ptr::null_mut::<Variable>();
    let mut loc: *mut Variable = ::core::ptr::null_mut::<Variable>();
    (*new).key = sstrdup(key);
    (*new).value = sstrdup(value);
    test = (*variables).slh_first;
    while !test.is_null() {
        if strlen((*new).key) >= strlen((*test).key) {
            break;
        }
        loc = test;
        test = (*test).variables.sle_next;
    }
    if loc.is_null() {
        (*new).variables.sle_next = (*variables).slh_first;
        (*variables).slh_first = new;
    } else {
        (*new).variables.sle_next = (*loc).variables.sle_next;
        (*loc).variables.sle_next = new;
    };
}
unsafe extern "C" fn get_resource(
    mut name: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    if conn.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if database.is_null() {
        database = xcb_xrm_database_from_default(conn);
        if database.is_null() {
            errorlog(
                b"ERROR: Failed to open the resource database.\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
            database =
                xcb_xrm_database_from_string(b"\0" as *const u8 as *const ::core::ffi::c_char);
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
    }
    let mut resource: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    xcb_xrm_resource_get_string(
        database,
        name,
        ::core::ptr::null::<::core::ffi::c_char>(),
        &raw mut resource,
    );
    return resource;
}
#[no_mangle]
pub unsafe extern "C" fn free_variables(mut ctx: *mut parser_ctx) {
    while !(*ctx).variables.slh_first.is_null() {
        let mut current: *mut Variable = (*ctx).variables.slh_first;
        free((*current).key as *mut ::core::ffi::c_void);
        (*current).key = ::core::ptr::null_mut::<::core::ffi::c_char>();
        free((*current).value as *mut ::core::ffi::c_void);
        (*current).value = ::core::ptr::null_mut::<::core::ffi::c_char>();
        (*ctx).variables.slh_first = (*(*ctx).variables.slh_first).variables.sle_next;
        free(current as *mut ::core::ffi::c_void);
        current = ::core::ptr::null_mut::<Variable>();
    }
}
unsafe extern "C" fn try_chdir(mut path: *const ::core::ffi::c_char) -> bool {
    let mut result: bool = true_0 != 0;
    let mut dir: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut dirbuf: *mut ::core::ffi::c_char = sstrdup(path);
    dir = dirname(dirbuf);
    if !dir.is_null() {
        verboselog(
            b"Changing working directory to config file directory %s\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            dir,
        );
        if chdir(dir) == -(1 as ::core::ffi::c_int) {
            errorlog(
                b"ERROR: chdir(%s) failed: %s\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                dir,
                strerror(*__errno_location()),
            );
            result = false_0 != 0;
        }
    }
    free(dirbuf as *mut ::core::ffi::c_void);
    dirbuf = ::core::ptr::null_mut::<::core::ffi::c_char>();
    return result;
}
unsafe extern "C" fn parse_file_inner(
    mut ctx: *mut parser_ctx,
    mut f: *const ::core::ffi::c_char,
    mut included_file: *mut IncludedFile,
) -> parse_file_result_t {
    if !try_chdir(f) {
        return PARSE_FILE_FAILED;
    }
    let mut fd: ::core::ffi::c_int = 0;
    fd = open(f, O_RDONLY);
    if fd == -(1 as ::core::ffi::c_int) {
        return PARSE_FILE_FAILED;
    }
    let mut stbuf: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    if fstat(fd, &raw mut stbuf) == -(1 as ::core::ffi::c_int) {
        close(fd);
        return PARSE_FILE_FAILED;
    }
    let mut fstr: *mut FILE = ::core::ptr::null_mut::<FILE>();
    fstr = fdopen(fd, b"r\0" as *const u8 as *const ::core::ffi::c_char);
    if fstr.is_null() {
        return PARSE_FILE_FAILED;
    }
    (*included_file).raw_contents =
        scalloc((stbuf.st_size + 1 as __off_t) as size_t, 1 as size_t) as *mut ::core::ffi::c_char;
    if fread(
        (*included_file).raw_contents as *mut ::core::ffi::c_void,
        1 as size_t,
        stbuf.st_size as size_t,
        fstr,
    ) as ssize_t
        != stbuf.st_size as ssize_t
    {
        fclose(fstr);
        return PARSE_FILE_FAILED;
    }
    rewind(fstr);
    let mut buffer: [::core::ffi::c_char; 4096] = [0; 4096];
    let mut key: [::core::ffi::c_char; 512] = [0; 512];
    let mut value: [::core::ffi::c_char; 4096] = [0; 4096];
    let mut continuation: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut invalid_sets: bool = false_0 != 0;
    let mut buf: *mut ::core::ffi::c_char =
        scalloc((stbuf.st_size + 1 as __off_t) as size_t, 1 as size_t) as *mut ::core::ffi::c_char;
    while feof(fstr) == 0 {
        if continuation.is_null() {
            continuation = &raw mut buffer as *mut ::core::ffi::c_char;
        }
        if fgets(
            continuation,
            (::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as usize).wrapping_sub(
                continuation.offset_from(&raw mut buffer as *mut ::core::ffi::c_char)
                    as ::core::ffi::c_long as usize,
            ) as ::core::ffi::c_int,
            fstr,
        )
        .is_null()
        {
            if feof(fstr) != 0 {
                break;
            }
            fclose(fstr);
            return PARSE_FILE_FAILED;
        } else {
            if buffer[strlen(&raw mut buffer as *mut ::core::ffi::c_char).wrapping_sub(1 as size_t)
                as usize] as ::core::ffi::c_int
                != '\n' as i32
                && feof(fstr) == 0
            {
                errorlog(
                    b"ERROR: Your line continuation is too long, it exceeds %zd bytes\n\0"
                        as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 4096]>(),
                );
            }
            value[0 as ::core::ffi::c_int as usize] = '\0' as i32 as ::core::ffi::c_char;
            let skip_line: bool = sscanf(
                &raw mut buffer as *mut ::core::ffi::c_char,
                b"%511s %4095[^\n]\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut key as *mut ::core::ffi::c_char,
                &raw mut value as *mut ::core::ffi::c_char,
            ) < 1 as ::core::ffi::c_int
                || strlen(&raw mut key as *mut ::core::ffi::c_char) < 3 as size_t;
            let comment: bool =
                key[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int == '#' as i32;
            value[4095 as ::core::ffi::c_int as usize] = '\n' as i32 as ::core::ffi::c_char;
            continuation = strstr(
                &raw mut buffer as *mut ::core::ffi::c_char,
                b"\\\n\0" as *const u8 as *const ::core::ffi::c_char,
            );
            if !continuation.is_null() {
                if !comment {
                    continue;
                }
                debuglog(
                    b"%s:%s:%d - line continuation in comment is ignored: \"%.*s\"\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    b"../src/config_parser.c\0" as *const u8 as *const ::core::ffi::c_char,
                    b"parse_file_inner\0" as *const u8 as *const ::core::ffi::c_char,
                    682 as ::core::ffi::c_int,
                    strlen(&raw mut buffer as *mut ::core::ffi::c_char) as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int,
                    &raw mut buffer as *mut ::core::ffi::c_char,
                );
                continuation = ::core::ptr::null_mut::<::core::ffi::c_char>();
            }
            strcpy(
                buf.offset(strlen(buf) as isize),
                &raw mut buffer as *mut ::core::ffi::c_char,
            );
            if skip_line as ::core::ffi::c_int != 0 || comment as ::core::ffi::c_int != 0 {
                continue;
            }
            if strcasecmp(
                &raw mut key as *mut ::core::ffi::c_char,
                b"set\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
                && *(&raw mut value as *mut ::core::ffi::c_char) as ::core::ffi::c_int
                    != '\0' as i32
            {
                let mut v_key: [::core::ffi::c_char; 512] = [0; 512];
                let mut v_value: [::core::ffi::c_char; 4096] = [
                    '\0' as i32 as ::core::ffi::c_char,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                ];
                if sscanf(
                    &raw mut value as *mut ::core::ffi::c_char,
                    b"%511s %4095[^\n]\0" as *const u8 as *const ::core::ffi::c_char,
                    &raw mut v_key as *mut ::core::ffi::c_char,
                    &raw mut v_value as *mut ::core::ffi::c_char,
                ) < 1 as ::core::ffi::c_int
                {
                    errorlog(
                        b"ERROR: Failed to parse variable specification '%s', skipping it.\n\0"
                            as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        &raw mut value as *mut ::core::ffi::c_char,
                    );
                    invalid_sets = true_0 != 0;
                } else if v_key[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    != '$' as i32
                {
                    errorlog(
                        b"ERROR: Malformed variable assignment, name has to start with $\n\0"
                            as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    );
                    invalid_sets = true_0 != 0;
                } else {
                    upsert_variable(
                        &raw mut (*ctx).variables,
                        &raw mut v_key as *mut ::core::ffi::c_char,
                        &raw mut v_value as *mut ::core::ffi::c_char,
                    );
                }
            } else {
                if !(strcasecmp(
                    &raw mut key as *mut ::core::ffi::c_char,
                    b"set_from_resource\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int)
                {
                    continue;
                }
                let mut res_name: [::core::ffi::c_char; 512] = [
                    '\0' as i32 as ::core::ffi::c_char,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                ];
                let mut v_key_0: [::core::ffi::c_char; 512] = [0; 512];
                let mut fallback: [::core::ffi::c_char; 4096] = [
                    '\0' as i32 as ::core::ffi::c_char,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                ];
                fallback[0 as ::core::ffi::c_int as usize] = '\0' as i32 as ::core::ffi::c_char;
                if sscanf(
                    &raw mut value as *mut ::core::ffi::c_char,
                    b"%511s %511s %4095[^\n]\0" as *const u8 as *const ::core::ffi::c_char,
                    &raw mut v_key_0 as *mut ::core::ffi::c_char,
                    &raw mut res_name as *mut ::core::ffi::c_char,
                    &raw mut fallback as *mut ::core::ffi::c_char,
                ) < 1 as ::core::ffi::c_int
                {
                    errorlog(
                        b"ERROR: Failed to parse resource specification '%s', skipping it.\n\0"
                            as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        &raw mut value as *mut ::core::ffi::c_char,
                    );
                    invalid_sets = true_0 != 0;
                } else if v_key_0[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    != '$' as i32
                {
                    errorlog(
                        b"ERROR: Malformed variable assignment, name has to start with $\n\0"
                            as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    );
                    invalid_sets = true_0 != 0;
                } else {
                    let mut res_value: *mut ::core::ffi::c_char =
                        get_resource(&raw mut res_name as *mut ::core::ffi::c_char);
                    if res_value.is_null() {
                        debuglog(
                            b"%s:%s:%d - Could not get resource '%s', using fallback '%s'.\n\0"
                                as *const u8
                                as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            b"../src/config_parser.c\0" as *const u8 as *const ::core::ffi::c_char,
                            b"parse_file_inner\0" as *const u8 as *const ::core::ffi::c_char,
                            738 as ::core::ffi::c_int,
                            &raw mut res_name as *mut ::core::ffi::c_char,
                            &raw mut fallback as *mut ::core::ffi::c_char,
                        );
                        res_value = sstrdup(&raw mut fallback as *mut ::core::ffi::c_char);
                    }
                    upsert_variable(
                        &raw mut (*ctx).variables,
                        &raw mut v_key_0 as *mut ::core::ffi::c_char,
                        res_value,
                    );
                    free(res_value as *mut ::core::ffi::c_void);
                    res_value = ::core::ptr::null_mut::<::core::ffi::c_char>();
                }
            }
        }
    }
    fclose(fstr);
    if !database.is_null() {
        xcb_xrm_database_free(database);
        database = ::core::ptr::null_mut::<xcb_xrm_database_t>();
    }
    let mut current: *mut Variable = ::core::ptr::null_mut::<Variable>();
    let mut extra_bytes: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut bufcopy: *mut ::core::ffi::c_char = sstrdup(buf);
    current = (*ctx).variables.slh_first;
    while !current.is_null() {
        let extra: ::core::ffi::c_int =
            strlen((*current).value).wrapping_sub(strlen((*current).key)) as ::core::ffi::c_int;
        let mut next: *mut ::core::ffi::c_char = bufcopy;
        while next < bufcopy.offset(stbuf.st_size as isize) && {
            next = strcasestr(next, (*current).key);
            !next.is_null()
        } {
            let mut end: *const ::core::ffi::c_char = next.offset(strlen((*current).key) as isize);
            while next < end as *mut ::core::ffi::c_char {
                let fresh0 = next;
                next = next.offset(1);
                *fresh0 = ' ' as i32 as ::core::ffi::c_char;
            }
            extra_bytes += extra;
        }
        current = (*current).variables.sle_next;
    }
    free(bufcopy as *mut ::core::ffi::c_void);
    bufcopy = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut walk: *const ::core::ffi::c_char = buf;
    let mut new: *mut ::core::ffi::c_char = scalloc(
        (stbuf.st_size + extra_bytes as __off_t + 1 as __off_t) as size_t,
        1 as size_t,
    ) as *mut ::core::ffi::c_char;
    let mut destwalk: *mut ::core::ffi::c_char = new;
    while walk < buf.offset(stbuf.st_size as isize) as *const ::core::ffi::c_char {
        current = (*ctx).variables.slh_first;
        while !current.is_null() {
            (*current).next_match = strcasestr(walk, (*current).key);
            current = (*current).variables.sle_next;
        }
        let mut nearest: *const Variable = ::core::ptr::null::<Variable>();
        let mut distance: ::core::ffi::c_int = stbuf.st_size as ::core::ffi::c_int;
        current = (*ctx).variables.slh_first;
        while !current.is_null() {
            if !(*current).next_match.is_null() {
                if ((*current).next_match.offset_from(walk) as ::core::ffi::c_long)
                    < distance as ::core::ffi::c_long
                {
                    distance = (*current).next_match.offset_from(walk) as ::core::ffi::c_long
                        as ::core::ffi::c_int;
                    nearest = current;
                }
            }
            current = (*current).variables.sle_next;
        }
        if nearest.is_null() {
            strncpy(
                destwalk,
                walk,
                buf.offset(stbuf.st_size as isize).offset_from(walk) as ::core::ffi::c_long
                    as size_t,
            );
            destwalk = destwalk.offset(buf.offset(stbuf.st_size as isize).offset_from(walk)
                as ::core::ffi::c_long as isize);
            *destwalk = '\0' as i32 as ::core::ffi::c_char;
            break;
        } else {
            strncpy(destwalk, walk, distance as size_t);
            strcpy(destwalk.offset(distance as isize), (*nearest).value);
            walk = walk.offset((distance as size_t).wrapping_add(strlen((*nearest).key)) as isize);
            destwalk = destwalk
                .offset((distance as size_t).wrapping_add(strlen((*nearest).value)) as isize);
        }
    }
    (*included_file).variable_replaced_contents = sstrdup(new);
    let mut context: *mut context =
        scalloc(1 as size_t, ::core::mem::size_of::<context>() as size_t) as *mut context;
    (*context).filename = f;
    parse_config(ctx, new, context);
    if (*ctx).has_errors {
        (*context).has_errors = true_0 != 0;
    }
    check_for_duplicate_bindings(context);
    if (*ctx).use_nagbar as ::core::ffi::c_int != 0
        && ((*context).has_errors as ::core::ffi::c_int != 0
            || (*context).has_warnings as ::core::ffi::c_int != 0
            || invalid_sets as ::core::ffi::c_int != 0)
    {
        errorlog(
            b"ERROR: FYI: You are using i3 version %s\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            i3_version,
        );
        start_config_error_nagbar(
            f,
            (*context).has_errors as ::core::ffi::c_int != 0
                || invalid_sets as ::core::ffi::c_int != 0,
        );
    }
    let has_errors: bool = (*context).has_errors;
    free((*context).line_copy as *mut ::core::ffi::c_void);
    (*context).line_copy = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free(context as *mut ::core::ffi::c_void);
    free(new as *mut ::core::ffi::c_void);
    free(buf as *mut ::core::ffi::c_void);
    if has_errors {
        return PARSE_FILE_CONFIG_ERRORS;
    }
    return PARSE_FILE_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn parse_file(
    mut ctx: *mut parser_ctx,
    mut f: *const ::core::ffi::c_char,
    mut included_file: *mut IncludedFile,
) -> parse_file_result_t {
    let mut old_dir: *mut ::core::ffi::c_char =
        getcwd(::core::ptr::null_mut::<::core::ffi::c_char>(), 0 as size_t);
    let mut result: parse_file_result_t = parse_file_inner(ctx, f, included_file);
    if chdir(old_dir) == -(1 as ::core::ffi::c_int) {
        errorlog(
            b"ERROR: chdir(%s) failed: %s\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            old_dir,
            strerror(*__errno_location()),
        );
        result = PARSE_FILE_FAILED;
    }
    free(old_dir as *mut ::core::ffi::c_void);
    return result;
}
pub const ERANGE: ::core::ffi::c_int = 34 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
