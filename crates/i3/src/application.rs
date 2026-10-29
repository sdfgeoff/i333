/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * main.c: Initialization, main loop
 *
 */
// Mechanical file-by-file Rust translation of src/main.c.
// Generated with C2Rust 0.22.1; C ABI and original ownership retained.
use ::c2rust_bitfields;
use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type xcb_connection_t;
    pub type xcb_extension_t;
    pub type _XCBKeySymbols;
    pub type _PangoFontDescription;
    pub type _cairo;
    pub type _cairo_surface;
    pub type _i3String;
    pub type SnDisplay;
    pub type pcre2_real_code_8;
    pub type ev_loop;
    pub type sockaddr_x25;
    pub type sockaddr_ns;
    pub type sockaddr_iso;
    pub type sockaddr_ipx;
    pub type sockaddr_inarp;
    pub type sockaddr_in6;
    pub type sockaddr_in;
    pub type sockaddr_eon;
    pub type sockaddr_dl;
    pub type sockaddr_ax25;
    pub type sockaddr_at;
    fn err(__status: ::core::ffi::c_int, __format: *const ::core::ffi::c_char, ...) -> !;
    fn errx(__status: ::core::ffi::c_int, __format: *const ::core::ffi::c_char, ...) -> !;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn setbuf(__stream: *mut FILE, __buf: *mut ::core::ffi::c_char);
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn fileno(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn time(__timer: *mut time_t) -> time_t;
    fn xcb_create_window(
        c: *mut xcb_connection_t,
        depth: uint8_t,
        wid: xcb_window_t,
        parent: xcb_window_t,
        x: int16_t,
        y: int16_t,
        width: uint16_t,
        height: uint16_t,
        border_width: uint16_t,
        _class: uint16_t,
        visual: xcb_visualid_t,
        value_mask: uint32_t,
        value_list: *const ::core::ffi::c_void,
    ) -> xcb_void_cookie_t;
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
    fn xcb_get_geometry(
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
    ) -> xcb_get_geometry_cookie_t;
    fn xcb_get_geometry_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_get_geometry_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_get_geometry_reply_t;
    fn xcb_intern_atom(
        c: *mut xcb_connection_t,
        only_if_exists: uint8_t,
        name_len: uint16_t,
        name: *const ::core::ffi::c_char,
    ) -> xcb_intern_atom_cookie_t;
    fn xcb_intern_atom_unchecked(
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
    fn xcb_set_selection_owner(
        c: *mut xcb_connection_t,
        owner: xcb_window_t,
        selection: xcb_atom_t,
        time_0: xcb_timestamp_t,
    ) -> xcb_void_cookie_t;
    fn xcb_get_selection_owner(
        c: *mut xcb_connection_t,
        selection: xcb_atom_t,
    ) -> xcb_get_selection_owner_cookie_t;
    fn xcb_get_selection_owner_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_get_selection_owner_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_get_selection_owner_reply_t;
    fn xcb_send_event(
        c: *mut xcb_connection_t,
        propagate: uint8_t,
        destination: xcb_window_t,
        event_mask: uint32_t,
        event: *const ::core::ffi::c_char,
    ) -> xcb_void_cookie_t;
    fn xcb_grab_server(c: *mut xcb_connection_t) -> xcb_void_cookie_t;
    fn xcb_ungrab_server(c: *mut xcb_connection_t) -> xcb_void_cookie_t;
    fn xcb_query_pointer(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_query_pointer_cookie_t;
    fn xcb_query_pointer_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_query_pointer_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_query_pointer_reply_t;
    fn xcb_create_colormap_checked(
        c: *mut xcb_connection_t,
        alloc: uint8_t,
        mid: xcb_colormap_t,
        window: xcb_window_t,
        visual: xcb_visualid_t,
    ) -> xcb_void_cookie_t;
    fn xcb_flush(c: *mut xcb_connection_t) -> ::core::ffi::c_int;
    fn xcb_prefetch_maximum_request_length(c: *mut xcb_connection_t);
    fn xcb_wait_for_event(c: *mut xcb_connection_t) -> *mut xcb_generic_event_t;
    fn xcb_poll_for_event(c: *mut xcb_connection_t) -> *mut xcb_generic_event_t;
    fn xcb_request_check(
        c: *mut xcb_connection_t,
        cookie: xcb_void_cookie_t,
    ) -> *mut xcb_generic_error_t;
    fn xcb_get_extension_data(
        c: *mut xcb_connection_t,
        ext: *mut xcb_extension_t,
    ) -> *const xcb_query_extension_reply_t;
    fn xcb_prefetch_extension_data(c: *mut xcb_connection_t, ext: *mut xcb_extension_t);
    fn xcb_get_file_descriptor(c: *mut xcb_connection_t) -> ::core::ffi::c_int;
    fn xcb_connection_has_error(c: *mut xcb_connection_t) -> ::core::ffi::c_int;
    fn xcb_disconnect(c: *mut xcb_connection_t);
    fn xcb_connect(
        displayname: *const ::core::ffi::c_char,
        screenp: *mut ::core::ffi::c_int,
    ) -> *mut xcb_connection_t;
    fn xcb_generate_id(c: *mut xcb_connection_t) -> uint32_t;
    fn xcb_key_symbols_alloc(c: *mut xcb_connection_t) -> *mut xcb_key_symbols_t;
    static mut xcb_xkb_id: xcb_extension_t;
    fn xcb_xkb_use_extension(
        c: *mut xcb_connection_t,
        wantedMajor: uint16_t,
        wantedMinor: uint16_t,
    ) -> xcb_xkb_use_extension_cookie_t;
    fn xcb_xkb_select_events(
        c: *mut xcb_connection_t,
        deviceSpec: xcb_xkb_device_spec_t,
        affectWhich: uint16_t,
        clear: uint16_t,
        selectAll: uint16_t,
        affectMap: uint16_t,
        map: uint16_t,
        details: *const ::core::ffi::c_void,
    ) -> xcb_void_cookie_t;
    fn xcb_xkb_per_client_flags(
        c: *mut xcb_connection_t,
        deviceSpec: xcb_xkb_device_spec_t,
        change: uint32_t,
        value: uint32_t,
        ctrlsToChange: uint32_t,
        autoCtrls: uint32_t,
        autoCtrlsValues: uint32_t,
    ) -> xcb_xkb_per_client_flags_cookie_t;
    fn xcb_xkb_per_client_flags_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_xkb_per_client_flags_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xkb_per_client_flags_reply_t;
    fn xcb_aux_get_depth_of_visual(screen: *mut xcb_screen_t, id: xcb_visualid_t) -> uint8_t;
    fn xcb_aux_get_screen(
        c: *mut xcb_connection_t,
        screen: ::core::ffi::c_int,
    ) -> *mut xcb_screen_t;
    fn xcb_aux_find_visual_by_attrs(
        screen: *mut xcb_screen_t,
        class_: int8_t,
        depth: int8_t,
    ) -> *mut xcb_visualtype_t;
    fn xcb_aux_sync(c: *mut xcb_connection_t);
    fn signal(__sig: ::core::ffi::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn raise(__sig: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sigemptyset(__set: *mut sigset_t) -> ::core::ffi::c_int;
    fn sigaction(
        __sig: ::core::ffi::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> ::core::ffi::c_int;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn read(__fd: ::core::ffi::c_int, __buf: *mut ::core::ffi::c_void, __nbytes: size_t)
        -> ssize_t;
    fn usleep(__useconds: __useconds_t) -> ::core::ffi::c_int;
    fn getcwd(__buf: *mut ::core::ffi::c_char, __size: size_t) -> *mut ::core::ffi::c_char;
    fn isatty(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn unlink(__name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn rmdir(__path: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    static mut optarg: *mut ::core::ffi::c_char;
    static mut optind: ::core::ffi::c_int;
    fn verboselog(fmt: *mut ::core::ffi::c_char, ...);
    fn errorlog(fmt: *mut ::core::ffi::c_char, ...);
    fn debuglog(fmt: *mut ::core::ffi::c_char, ...);
    fn root_atom_contents(
        atomname: *const ::core::ffi::c_char,
        provided_conn: *mut xcb_connection_t,
        screen: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn scalloc(num: size_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn sstrdup(str: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn sasprintf(
        strp: *mut *mut ::core::ffi::c_char,
        fmt: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn ipc_send_message(
        sockfd: ::core::ffi::c_int,
        message_size: uint32_t,
        message_type: uint32_t,
        payload: *const uint8_t,
    ) -> ::core::ffi::c_int;
    fn ipc_recv_message(
        sockfd: ::core::ffi::c_int,
        message_type: *mut uint32_t,
        reply_length: *mut uint32_t,
        reply: *mut *mut uint8_t,
    ) -> ::core::ffi::c_int;
    fn aio_get_mod_mask_for(keysym: uint32_t, symbols: *mut xcb_key_symbols_t) -> uint32_t;
    fn get_visualtype(screen: *mut xcb_screen_t) -> *mut xcb_visualtype_t;
    fn is_debug_build() -> bool;
    fn get_process_filename(prefix: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn init_dpi();
    fn logical_px(logical: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn create_socket(
        filename: *const ::core::ffi::c_char,
        out_socketpath: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn set_screenshot_as_wallpaper(conn_0: *mut xcb_connection_t, screen: *mut xcb_screen_t);
    fn is_background_set(conn_0: *mut xcb_connection_t, screen: *mut xcb_screen_t) -> bool;
    fn sn_xcb_display_new(
        xconnection: *mut xcb_connection_t,
        push_trap_func: SnXcbDisplayErrorTrapPush,
        pop_trap_func: SnXcbDisplayErrorTrapPop,
    ) -> *mut SnDisplay;
    static mut xcb_randr_id: xcb_extension_t;
    fn kill_nagbar(nagbar_pid: pid_t, wait_for_it: bool);
    fn parse_long(
        str: *const ::core::ffi::c_char,
        out: *mut ::core::ffi::c_long,
        base: ::core::ffi::c_int,
    ) -> bool;
    fn ev_default_loop(flags: ::core::ffi::c_uint) -> *mut ev_loop;
    fn ev_loop_destroy(loop_0: *mut ev_loop);
    fn ev_run(loop_0: *mut ev_loop, flags: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn ev_unref(loop_0: *mut ev_loop);
    fn ev_feed_event(
        loop_0: *mut ev_loop,
        w: *mut ::core::ffi::c_void,
        revents: ::core::ffi::c_int,
    );
    fn ev_io_start(loop_0: *mut ev_loop, w: *mut ev_io);
    fn ev_signal_start(loop_0: *mut ev_loop, w: *mut ev_signal);
    fn ev_prepare_start(loop_0: *mut ev_loop, w: *mut ev_prepare);
    fn ev_prepare_stop(loop_0: *mut ev_loop, w: *mut ev_prepare);
    static mut croot: *mut Con;
    fn tree_init(geometry: *mut xcb_get_geometry_reply_t);
    fn tree_render();
    fn tree_restore(
        path: *const ::core::ffi::c_char,
        geometry: *mut xcb_get_geometry_reply_t,
    ) -> bool;
    fn getrlimit(__resource: __rlimit_resource_t, __rlimits: *mut rlimit) -> ::core::ffi::c_int;
    fn setrlimit(__resource: __rlimit_resource_t, __rlimits: *const rlimit) -> ::core::ffi::c_int;
    static mut xcb_shape_id: xcb_extension_t;
    fn xcb_shape_query_version(c: *mut xcb_connection_t) -> xcb_shape_query_version_cookie_t;
    fn xcb_shape_query_version_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_shape_query_version_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_shape_query_version_reply_t;
    static mut i3_version: *const ::core::ffi::c_char;
    static mut config: Config;
    static mut barconfigs: barconfig_head;
    fn load_configuration(
        override_configfile: *const ::core::ffi::c_char,
        load_type: config_load_t,
    ) -> bool;
    static mut current_socketpath: *mut ::core::ffi::c_char;
    fn ipc_new_client(loop_0: *mut ev_loop, w: *mut ev_io, revents: ::core::ffi::c_int);
    fn ipc_new_client_on_fd(loop_0: *mut ev_loop, fd: ::core::ffi::c_int) -> *mut ipc_client;
    fn ipc_shutdown(reason: shutdown_reason_t, exempt_fd: ::core::ffi::c_int);
    fn ipc_confirm_restart(client: *mut ipc_client);
    static mut shmlogname: *mut ::core::ffi::c_char;
    static mut shmlog_size: ::core::ffi::c_int;
    static mut current_log_stream_socket_path: *mut ::core::ffi::c_char;
    fn init_logging();
    fn set_debug_logging(_debug_logging: bool);
    fn set_verbosity(_verbose: bool);
    fn log_new_client(loop_0: *mut ev_loop, w: *mut ev_io, revents: ::core::ffi::c_int);
    fn xcursor_load_cursors();
    fn xcursor_set_root_cursor(cursor_id: ::core::ffi::c_int);
    static mut xcb_numlock_mask: ::core::ffi::c_uint;
    fn manage_existing_windows(root_0: xcb_window_t);
    static mut outputs: outputs_head;
    fn randr_init(event_base: *mut ::core::ffi::c_int, disable_randr15: bool);
    fn output_init_con(output: *mut Output);
    fn randr_disable_output(output: *mut Output);
    fn get_first_output() -> *mut Output;
    fn get_output_containing(x: ::core::ffi::c_uint, y: ::core::ffi::c_uint) -> *mut Output;
    fn x_set_i3_atoms();
    static mut randr_base: ::core::ffi::c_int;
    static mut xkb_base: ::core::ffi::c_int;
    static mut shape_base: ::core::ffi::c_int;
    fn event_is_ignored(
        sequence: ignore_event_sequence_t,
        response_type: ::core::ffi::c_int,
    ) -> bool;
    fn handle_event(type_0: ::core::ffi::c_int, event: *mut xcb_generic_event_t);
    fn property_handlers_init();
    fn xinerama_init();
    fn con_activate(con: *mut Con);
    fn con_descend_focused(con: *mut Con) -> *mut Con;
    fn setup_signal_handler();
    fn output_get_content(output: *mut Con) -> *mut Con;
    fn output_primary_name(output: *mut Output) -> *mut ::core::ffi::c_char;
    fn ewmh_update_desktop_properties();
    fn ewmh_setup_hints();
    fn ewmh_update_workarea();
    fn start_application(
        command: *const ::core::ffi::c_char,
        no_startup_id: bool,
        window_id: xcb_window_t,
    );
    fn scratchpad_fix_resolution();
    static mut command_error_nagbar_pid: pid_t;
    fn grab_all_keys(conn_0: *mut xcb_connection_t);
    fn translate_keysyms();
    fn load_keymap() -> bool;
    static mut config_error_nagbar_pid: pid_t;
    fn fake_outputs_init(output_spec: *const ::core::ffi::c_char);
    fn display_running_version();
    fn restore_connect();
    fn fcntl(__fd: ::core::ffi::c_int, __cmd: ::core::ffi::c_int, ...) -> ::core::ffi::c_int;
    fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    fn getopt_long(
        ___argc: ::core::ffi::c_int,
        ___argv: *const *mut ::core::ffi::c_char,
        __shortopts: *const ::core::ffi::c_char,
        __longopts: *const option,
        __longind: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn dirname(__path: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn setlocale(
        __category: ::core::ffi::c_int,
        __locale: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn shm_unlink(__name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn socket(
        __domain: ::core::ffi::c_int,
        __type: ::core::ffi::c_int,
        __protocol: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn connect(
        __fd: ::core::ffi::c_int,
        __addr: __CONST_SOCKADDR_ARG,
        __len: socklen_t,
    ) -> ::core::ffi::c_int;
    fn xcb_atom_name_by_screen(
        base: *const ::core::ffi::c_char,
        screen: uint8_t,
    ) -> *mut ::core::ffi::c_char;
    static mut xcb_xinerama_id: xcb_extension_t;
    static mut xcb_big_requests_id: xcb_extension_t;
    fn sd_listen_fds(unset_environment: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sd_notify(
        unset_environment: ::core::ffi::c_int,
        state: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn memmove(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strncpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn atoi(__nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn srand(__seed: ::core::ffi::c_uint);
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn atexit(__func: Option<unsafe extern "C" fn() -> ()>) -> ::core::ffi::c_int;
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn getenv(__name: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn unsetenv(__name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
}
pub type size_t = usize;
pub type __int8_t = i8;
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type __uid_t = ::core::ffi::c_uint;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __pid_t = ::core::ffi::c_int;
pub type __clock_t = ::core::ffi::c_long;
pub type __rlim64_t = ::core::ffi::c_ulong;
pub type __time_t = ::core::ffi::c_long;
pub type __useconds_t = ::core::ffi::c_uint;
pub type __suseconds_t = ::core::ffi::c_long;
pub type __socklen_t = ::core::ffi::c_uint;
pub type pid_t = __pid_t;
pub type ssize_t = isize;
pub type time_t = __time_t;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [::core::ffi::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
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
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: ::core::ffi::c_int,
    pub sival_ptr: *mut ::core::ffi::c_void,
}
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
pub type xcb_visualid_t = uint32_t;
pub type xcb_timestamp_t = uint32_t;
pub type xcb_keycode_t = uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_visualtype_t {
    pub visual_id: xcb_visualid_t,
    pub _class: uint8_t,
    pub bits_per_rgb_value: uint8_t,
    pub colormap_entries: uint16_t,
    pub red_mask: uint32_t,
    pub green_mask: uint32_t,
    pub blue_mask: uint32_t,
    pub pad0: [uint8_t; 4],
}
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
pub type xcb_window_class_t = ::core::ffi::c_uint;
pub const XCB_WINDOW_CLASS_INPUT_ONLY: xcb_window_class_t = 2;
pub const XCB_WINDOW_CLASS_INPUT_OUTPUT: xcb_window_class_t = 1;
pub const XCB_WINDOW_CLASS_COPY_FROM_PARENT: xcb_window_class_t = 0;
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
pub type xcb_prop_mode_t = ::core::ffi::c_uint;
pub const XCB_PROP_MODE_APPEND: xcb_prop_mode_t = 2;
pub const XCB_PROP_MODE_PREPEND: xcb_prop_mode_t = 1;
pub const XCB_PROP_MODE_REPLACE: xcb_prop_mode_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_get_selection_owner_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_get_selection_owner_reply_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub owner: xcb_window_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_query_pointer_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_query_pointer_reply_t {
    pub response_type: uint8_t,
    pub same_screen: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub root: xcb_window_t,
    pub child: xcb_window_t,
    pub root_x: int16_t,
    pub root_y: int16_t,
    pub win_x: int16_t,
    pub win_y: int16_t,
    pub mask: uint16_t,
    pub pad0: [uint8_t; 2],
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
pub type xcb_colormap_alloc_t = ::core::ffi::c_uint;
pub const XCB_COLORMAP_ALLOC_ALL: xcb_colormap_alloc_t = 1;
pub const XCB_COLORMAP_ALLOC_NONE: xcb_colormap_alloc_t = 0;
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
pub type xcb_key_symbols_t = _XCBKeySymbols;
pub type xcb_xkb_event_type_t = ::core::ffi::c_uint;
pub const XCB_XKB_EVENT_TYPE_EXTENSION_DEVICE_NOTIFY: xcb_xkb_event_type_t = 2048;
pub const XCB_XKB_EVENT_TYPE_ACCESS_X_NOTIFY: xcb_xkb_event_type_t = 1024;
pub const XCB_XKB_EVENT_TYPE_ACTION_MESSAGE: xcb_xkb_event_type_t = 512;
pub const XCB_XKB_EVENT_TYPE_BELL_NOTIFY: xcb_xkb_event_type_t = 256;
pub const XCB_XKB_EVENT_TYPE_COMPAT_MAP_NOTIFY: xcb_xkb_event_type_t = 128;
pub const XCB_XKB_EVENT_TYPE_NAMES_NOTIFY: xcb_xkb_event_type_t = 64;
pub const XCB_XKB_EVENT_TYPE_INDICATOR_MAP_NOTIFY: xcb_xkb_event_type_t = 32;
pub const XCB_XKB_EVENT_TYPE_INDICATOR_STATE_NOTIFY: xcb_xkb_event_type_t = 16;
pub const XCB_XKB_EVENT_TYPE_CONTROLS_NOTIFY: xcb_xkb_event_type_t = 8;
pub const XCB_XKB_EVENT_TYPE_STATE_NOTIFY: xcb_xkb_event_type_t = 4;
pub const XCB_XKB_EVENT_TYPE_MAP_NOTIFY: xcb_xkb_event_type_t = 2;
pub const XCB_XKB_EVENT_TYPE_NEW_KEYBOARD_NOTIFY: xcb_xkb_event_type_t = 1;
pub type xcb_xkb_device_spec_t = uint16_t;
pub type xcb_xkb_id_t = ::core::ffi::c_uint;
pub const XCB_XKB_ID_XI_NONE: xcb_xkb_id_t = 65280;
pub const XCB_XKB_ID_ALL_XI_ID: xcb_xkb_id_t = 1536;
pub const XCB_XKB_ID_ALL_XI_CLASS: xcb_xkb_id_t = 1280;
pub const XCB_XKB_ID_DFLT_XI_ID: xcb_xkb_id_t = 1024;
pub const XCB_XKB_ID_DFLT_XI_CLASS: xcb_xkb_id_t = 768;
pub const XCB_XKB_ID_USE_CORE_PTR: xcb_xkb_id_t = 512;
pub const XCB_XKB_ID_USE_CORE_KBD: xcb_xkb_id_t = 256;
pub type xcb_xkb_per_client_flag_t = ::core::ffi::c_uint;
pub const XCB_XKB_PER_CLIENT_FLAG_SEND_EVENT_USES_XKB_STATE: xcb_xkb_per_client_flag_t = 16;
pub const XCB_XKB_PER_CLIENT_FLAG_LOOKUP_STATE_WHEN_GRABBED: xcb_xkb_per_client_flag_t = 8;
pub const XCB_XKB_PER_CLIENT_FLAG_AUTO_RESET_CONTROLS: xcb_xkb_per_client_flag_t = 4;
pub const XCB_XKB_PER_CLIENT_FLAG_GRABS_USE_XKB_STATE: xcb_xkb_per_client_flag_t = 2;
pub const XCB_XKB_PER_CLIENT_FLAG_DETECTABLE_AUTO_REPEAT: xcb_xkb_per_client_flag_t = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xkb_use_extension_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xkb_per_client_flags_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xkb_per_client_flags_reply_t {
    pub response_type: uint8_t,
    pub deviceID: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub supported: uint32_t,
    pub value: uint32_t,
    pub autoCtrls: uint32_t,
    pub autoCtrlsValues: uint32_t,
    pub pad0: [uint8_t; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct siginfo_t {
    pub si_signo: ::core::ffi::c_int,
    pub si_errno: ::core::ffi::c_int,
    pub si_code: ::core::ffi::c_int,
    pub __pad0: ::core::ffi::c_int,
    pub _sifields: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub _pad: [::core::ffi::c_int; 28],
    pub _kill: C2RustUnnamed_8,
    pub _timer: C2RustUnnamed_7,
    pub _rt: C2RustUnnamed_6,
    pub _sigchld: C2RustUnnamed_5,
    pub _sigfault: C2RustUnnamed_2,
    pub _sigpoll: C2RustUnnamed_1,
    pub _sigsys: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub _call_addr: *mut ::core::ffi::c_void,
    pub _syscall: ::core::ffi::c_int,
    pub _arch: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub si_band: ::core::ffi::c_long,
    pub si_fd: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub si_addr: *mut ::core::ffi::c_void,
    pub si_addr_lsb: ::core::ffi::c_short,
    pub _bounds: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub _addr_bnd: C2RustUnnamed_4,
    pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub _lower: *mut ::core::ffi::c_void,
    pub _upper: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: ::core::ffi::c_int,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub si_tid: ::core::ffi::c_int,
    pub si_overrun: ::core::ffi::c_int,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
pub type __sighandler_t = Option<unsafe extern "C" fn(::core::ffi::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: C2RustUnnamed_9,
    pub sa_mask: __sigset_t,
    pub sa_flags: ::core::ffi::c_int,
    pub sa_restorer: Option<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction: Option<
        unsafe extern "C" fn(::core::ffi::c_int, *mut siginfo_t, *mut ::core::ffi::c_void) -> (),
    >,
}
pub type socklen_t = __socklen_t;
pub type PangoFontDescription = _PangoFontDescription;
pub type cairo_t = _cairo;
pub type cairo_surface_t = _cairo_surface;
pub type i3String = _i3String;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Font {
    pub type_0: C2RustUnnamed_12,
    pub height: ::core::ffi::c_int,
    pub pattern: *mut ::core::ffi::c_char,
    pub specific: C2RustUnnamed_10,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
    pub xcb: C2RustUnnamed_11,
    pub pango_desc: *mut PangoFontDescription,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub id: xcb_font_t,
    pub info: *mut xcb_query_font_reply_t,
    pub table: *mut xcb_charinfo_t,
}
pub type C2RustUnnamed_12 = ::core::ffi::c_uint;
pub const FONT_TYPE_PANGO: C2RustUnnamed_12 = 2;
pub const FONT_TYPE_XCB: C2RustUnnamed_12 = 1;
pub const FONT_TYPE_NONE: C2RustUnnamed_12 = 0;
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
pub type SnXcbDisplayErrorTrapPush =
    Option<unsafe extern "C" fn(*mut SnDisplay, *mut xcb_connection_t) -> ()>;
pub type SnXcbDisplayErrorTrapPop =
    Option<unsafe extern "C" fn(*mut SnDisplay, *mut xcb_connection_t) -> ()>;
pub type xcb_randr_output_t = uint32_t;
pub type pcre2_code_8 = pcre2_real_code_8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Binding {
    pub input_type: input_type_t,
    pub release: C2RustUnnamed_15,
    pub border: bool,
    pub whole_window: bool,
    pub exclude_titlebar: bool,
    pub keycode: uint32_t,
    pub event_state_mask: i3_event_state_mask_t,
    pub symbol: *mut ::core::ffi::c_char,
    pub keycodes_head: keycodes_head,
    pub command: *mut ::core::ffi::c_char,
    pub bindings: C2RustUnnamed_13,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
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
    pub keycodes: C2RustUnnamed_14,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub tqe_next: *mut Binding_Keycode,
    pub tqe_prev: *mut *mut Binding_Keycode,
}
pub type i3_event_state_mask_t = uint32_t;
pub type C2RustUnnamed_15 = ::core::ffi::c_uint;
pub const B_UPON_KEYRELEASE_IGNORE_MODS: C2RustUnnamed_15 = 2;
pub const B_UPON_KEYRELEASE: C2RustUnnamed_15 = 1;
pub const B_UPON_KEYPRESS: C2RustUnnamed_15 = 0;
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
pub struct xoutput {
    pub id: xcb_randr_output_t,
    pub active: bool,
    pub changed: bool,
    pub to_be_disabled: bool,
    pub primary: bool,
    pub names_head: names_head,
    pub con: *mut Con,
    pub rect: Rect,
    pub outputs: C2RustUnnamed_16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_16 {
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
    pub type_0: C2RustUnnamed_33,
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
    pub floating: C2RustUnnamed_22,
    pub nodes: C2RustUnnamed_21,
    pub focused: C2RustUnnamed_20,
    pub all_cons: C2RustUnnamed_19,
    pub floating_windows: C2RustUnnamed_18,
    pub on_remove_child: Option<unsafe extern "C" fn(*mut Con) -> ()>,
    pub scratchpad_state: C2RustUnnamed_17,
    pub old_id: ::core::ffi::c_int,
    pub depth: uint16_t,
    pub colormap: xcb_colormap_t,
}
pub type C2RustUnnamed_17 = ::core::ffi::c_uint;
pub const SCRATCHPAD_CHANGED: C2RustUnnamed_17 = 2;
pub const SCRATCHPAD_FRESH: C2RustUnnamed_17 = 1;
pub const SCRATCHPAD_NONE: C2RustUnnamed_17 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_18 {
    pub tqe_next: *mut Con,
    pub tqe_prev: *mut *mut Con,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_19 {
    pub tqe_next: *mut Con,
    pub tqe_prev: *mut *mut Con,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_20 {
    pub tqe_next: *mut Con,
    pub tqe_prev: *mut *mut Con,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_21 {
    pub tqe_next: *mut Con,
    pub tqe_prev: *mut *mut Con,
}
pub type C2RustUnnamed_22 = ::core::ffi::c_uint;
pub const FLOATING_USER_ON: C2RustUnnamed_22 = 3;
pub const FLOATING_AUTO_ON: C2RustUnnamed_22 = 2;
pub const FLOATING_USER_OFF: C2RustUnnamed_22 = 1;
pub const FLOATING_AUTO_OFF: C2RustUnnamed_22 = 0;
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
    pub urgent: C2RustUnnamed_27,
    pub dock: C2RustUnnamed_26,
    pub id: xcb_window_t,
    pub window_mode: C2RustUnnamed_25,
    pub con_id: *mut Con,
    pub match_all_windows: bool,
    pub insert_where: C2RustUnnamed_24,
    pub matches: C2RustUnnamed_23,
    pub restart_mode: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_23 {
    pub tqe_next: *mut Match,
    pub tqe_prev: *mut *mut Match,
}
pub type C2RustUnnamed_24 = ::core::ffi::c_uint;
pub const M_BELOW: C2RustUnnamed_24 = 2;
pub const M_ASSIGN_WS: C2RustUnnamed_24 = 1;
pub const M_HERE: C2RustUnnamed_24 = 0;
pub type C2RustUnnamed_25 = ::core::ffi::c_uint;
pub const WM_FLOATING: C2RustUnnamed_25 = 6;
pub const WM_FLOATING_USER: C2RustUnnamed_25 = 5;
pub const WM_FLOATING_AUTO: C2RustUnnamed_25 = 4;
pub const WM_TILING: C2RustUnnamed_25 = 3;
pub const WM_TILING_USER: C2RustUnnamed_25 = 2;
pub const WM_TILING_AUTO: C2RustUnnamed_25 = 1;
pub const WM_ANY: C2RustUnnamed_25 = 0;
pub type C2RustUnnamed_26 = ::core::ffi::c_int;
pub const M_DOCK_BOTTOM: C2RustUnnamed_26 = 3;
pub const M_DOCK_TOP: C2RustUnnamed_26 = 2;
pub const M_DOCK_ANY: C2RustUnnamed_26 = 1;
pub const M_NODOCK: C2RustUnnamed_26 = 0;
pub const M_DONTCHECK: C2RustUnnamed_26 = -1;
pub type C2RustUnnamed_27 = ::core::ffi::c_int;
pub const U_OLDEST: C2RustUnnamed_27 = 1;
pub const U_LATEST: C2RustUnnamed_27 = 0;
pub const U_DONTCHECK: C2RustUnnamed_27 = -1;
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
    pub dock: C2RustUnnamed_28,
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
pub type C2RustUnnamed_28 = ::core::ffi::c_uint;
pub const W_DOCK_BOTTOM: C2RustUnnamed_28 = 2;
pub const W_DOCK_TOP: C2RustUnnamed_28 = 1;
pub const W_NODOCK: C2RustUnnamed_28 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Assignment {
    pub type_0: C2RustUnnamed_31,
    pub match_0: Match,
    pub dest: C2RustUnnamed_30,
    pub assignments: C2RustUnnamed_29,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_29 {
    pub tqe_next: *mut Assignment,
    pub tqe_prev: *mut *mut Assignment,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_30 {
    pub command: *mut ::core::ffi::c_char,
    pub workspace: *mut ::core::ffi::c_char,
    pub output: *mut ::core::ffi::c_char,
}
pub type C2RustUnnamed_31 = ::core::ffi::c_uint;
pub const A_TO_OUTPUT: C2RustUnnamed_31 = 16;
pub const A_TO_WORKSPACE_NUMBER: C2RustUnnamed_31 = 8;
pub const A_NO_FOCUS: C2RustUnnamed_31 = 4;
pub const A_TO_WORKSPACE: C2RustUnnamed_31 = 2;
pub const A_COMMAND: C2RustUnnamed_31 = 1;
pub const A_ANY: C2RustUnnamed_31 = 0;
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
    pub marks: C2RustUnnamed_32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_32 {
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
pub type C2RustUnnamed_33 = ::core::ffi::c_uint;
pub const CT_DOCKAREA: C2RustUnnamed_33 = 5;
pub const CT_WORKSPACE: C2RustUnnamed_33 = 4;
pub const CT_FLOATING_CON: C2RustUnnamed_33 = 3;
pub const CT_CON: C2RustUnnamed_33 = 2;
pub const CT_OUTPUT: C2RustUnnamed_33 = 1;
pub const CT_ROOT: C2RustUnnamed_33 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct names_head {
    pub slh_first: *mut output_name,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct output_name {
    pub name: *mut ::core::ffi::c_char,
    pub names: C2RustUnnamed_34,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_34 {
    pub sle_next: *mut output_name,
}
pub type Output = xoutput;
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
    pub ws_assignments: C2RustUnnamed_35,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_35 {
    pub tqe_next: *mut Workspace_Assignment,
    pub tqe_prev: *mut *mut Workspace_Assignment,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ignore_event_sequence_t {
    pub value: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Autostart {
    pub command: *mut ::core::ffi::c_char,
    pub no_startup_id: bool,
    pub autostarts: C2RustUnnamed_37,
    pub autostarts_always: C2RustUnnamed_36,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_36 {
    pub tqe_next: *mut Autostart,
    pub tqe_prev: *mut *mut Autostart,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_37 {
    pub tqe_next: *mut Autostart,
    pub tqe_prev: *mut *mut Autostart,
}
pub type C2RustUnnamed_38 = ::core::ffi::c_int;
pub const EV_ERROR: C2RustUnnamed_38 = -2147483648;
pub const EV_CUSTOM: C2RustUnnamed_38 = 16777216;
pub const EV_ASYNC: C2RustUnnamed_38 = 524288;
pub const EV_CLEANUP: C2RustUnnamed_38 = 262144;
pub const EV_FORK: C2RustUnnamed_38 = 131072;
pub const EV_EMBED: C2RustUnnamed_38 = 65536;
pub const EV_CHECK: C2RustUnnamed_38 = 32768;
pub const EV_PREPARE: C2RustUnnamed_38 = 16384;
pub const EV_IDLE: C2RustUnnamed_38 = 8192;
pub const EV_STAT: C2RustUnnamed_38 = 4096;
pub const EV_CHILD: C2RustUnnamed_38 = 2048;
pub const EV_SIGNAL: C2RustUnnamed_38 = 1024;
pub const EV_PERIODIC: C2RustUnnamed_38 = 512;
pub const EV_TIMEOUT: C2RustUnnamed_38 = 256;
pub const EV_TIMER: C2RustUnnamed_38 = 256;
pub const EV_IO: C2RustUnnamed_38 = 1;
pub const EV__IOFDSET: C2RustUnnamed_38 = 128;
pub const EV_WRITE: C2RustUnnamed_38 = 2;
pub const EV_READ: C2RustUnnamed_38 = 1;
pub const EV_NONE: C2RustUnnamed_38 = 0;
pub const EV_UNDEF: C2RustUnnamed_38 = -1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ev_watcher {
    pub active: ::core::ffi::c_int,
    pub pending: ::core::ffi::c_int,
    pub priority: ::core::ffi::c_int,
    pub data: *mut ::core::ffi::c_void,
    pub cb: Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_watcher, ::core::ffi::c_int) -> ()>,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ev_signal {
    pub active: ::core::ffi::c_int,
    pub pending: ::core::ffi::c_int,
    pub priority: ::core::ffi::c_int,
    pub data: *mut ::core::ffi::c_void,
    pub cb: Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_signal, ::core::ffi::c_int) -> ()>,
    pub next: *mut ev_watcher_list,
    pub signum: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ev_prepare {
    pub active: ::core::ffi::c_int,
    pub pending: ::core::ffi::c_int,
    pub priority: ::core::ffi::c_int,
    pub data: *mut ::core::ffi::c_void,
    pub cb: Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_prepare, ::core::ffi::c_int) -> ()>,
}
pub type __rlimit_resource = ::core::ffi::c_uint;
pub const __RLIM_NLIMITS: __rlimit_resource = 16;
pub const __RLIMIT_NLIMITS: __rlimit_resource = 16;
pub const __RLIMIT_RTTIME: __rlimit_resource = 15;
pub const __RLIMIT_RTPRIO: __rlimit_resource = 14;
pub const __RLIMIT_NICE: __rlimit_resource = 13;
pub const __RLIMIT_MSGQUEUE: __rlimit_resource = 12;
pub const __RLIMIT_SIGPENDING: __rlimit_resource = 11;
pub const __RLIMIT_LOCKS: __rlimit_resource = 10;
pub const __RLIMIT_MEMLOCK: __rlimit_resource = 8;
pub const __RLIMIT_NPROC: __rlimit_resource = 6;
pub const RLIMIT_AS: __rlimit_resource = 9;
pub const __RLIMIT_OFILE: __rlimit_resource = 7;
pub const RLIMIT_NOFILE: __rlimit_resource = 7;
pub const __RLIMIT_RSS: __rlimit_resource = 5;
pub const RLIMIT_CORE: __rlimit_resource = 4;
pub const RLIMIT_STACK: __rlimit_resource = 3;
pub const RLIMIT_DATA: __rlimit_resource = 2;
pub const RLIMIT_FSIZE: __rlimit_resource = 1;
pub const RLIMIT_CPU: __rlimit_resource = 0;
pub type rlim_t = __rlim64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rlimit {
    pub rlim_cur: rlim_t,
    pub rlim_max: rlim_t,
}
pub type __rlimit_resource_t = __rlimit_resource;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_shape_query_version_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_shape_query_version_reply_t {
    pub response_type: uint8_t,
    pub pad0: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub major_version: uint16_t,
    pub minor_version: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bindings_head {
    pub tqh_first: *mut Binding,
    pub tqh_last: *mut *mut Binding,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct autostarts_head {
    pub tqh_first: *mut Autostart,
    pub tqh_last: *mut *mut Autostart,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct autostarts_always_head {
    pub tqh_first: *mut Autostart,
    pub tqh_last: *mut *mut Autostart,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ws_assignments_head {
    pub tqh_first: *mut Workspace_Assignment,
    pub tqh_last: *mut *mut Workspace_Assignment,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct assignments_head {
    pub tqh_first: *mut Assignment,
    pub tqh_last: *mut *mut Assignment,
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
    pub focus_on_window_activation: C2RustUnnamed_41,
    pub show_marks: bool,
    pub title_align: C2RustUnnamed_40,
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
    pub popup_during_fullscreen: C2RustUnnamed_39,
    pub number_barconfigs: ::core::ffi::c_int,
    pub tiling_drag: tiling_drag_t,
    pub gaps: gaps_t,
    pub smart_gaps: smart_gaps_t,
}
pub type C2RustUnnamed_39 = ::core::ffi::c_uint;
pub const PDF_ALL: C2RustUnnamed_39 = 3;
pub const PDF_IGNORE: C2RustUnnamed_39 = 2;
pub const PDF_LEAVE_FULLSCREEN: C2RustUnnamed_39 = 1;
pub const PDF_SMART: C2RustUnnamed_39 = 0;
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
pub type C2RustUnnamed_40 = ::core::ffi::c_uint;
pub const ALIGN_RIGHT: C2RustUnnamed_40 = 2;
pub const ALIGN_CENTER: C2RustUnnamed_40 = 1;
pub const ALIGN_LEFT: C2RustUnnamed_40 = 0;
pub type C2RustUnnamed_41 = ::core::ffi::c_uint;
pub const FOWA_NONE: C2RustUnnamed_41 = 3;
pub const FOWA_FOCUS: C2RustUnnamed_41 = 2;
pub const FOWA_URGENT: C2RustUnnamed_41 = 1;
pub const FOWA_SMART: C2RustUnnamed_41 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Barconfig {
    pub id: *mut ::core::ffi::c_char,
    pub num_outputs: ::core::ffi::c_int,
    pub outputs: *mut *mut ::core::ffi::c_char,
    pub tray_outputs: tray_outputs_head,
    pub tray_padding: ::core::ffi::c_int,
    pub socket_path: *mut ::core::ffi::c_char,
    pub mode: C2RustUnnamed_46,
    pub hidden_state: C2RustUnnamed_45,
    pub modifier: uint32_t,
    pub bar_bindings: bar_bindings_head,
    pub position: C2RustUnnamed_43,
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
    pub configs: C2RustUnnamed_42,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_42 {
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
pub type C2RustUnnamed_43 = ::core::ffi::c_uint;
pub const P_TOP: C2RustUnnamed_43 = 1;
pub const P_BOTTOM: C2RustUnnamed_43 = 0;
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
    pub bindings: C2RustUnnamed_44,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_44 {
    pub tqe_next: *mut Barbinding,
    pub tqe_prev: *mut *mut Barbinding,
}
pub type C2RustUnnamed_45 = ::core::ffi::c_uint;
pub const S_SHOW: C2RustUnnamed_45 = 1;
pub const S_HIDE: C2RustUnnamed_45 = 0;
pub type C2RustUnnamed_46 = ::core::ffi::c_uint;
pub const M_INVISIBLE: C2RustUnnamed_46 = 2;
pub const M_HIDE: C2RustUnnamed_46 = 1;
pub const M_DOCK: C2RustUnnamed_46 = 0;
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
    pub tray_outputs: C2RustUnnamed_47,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_47 {
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
    pub clients: C2RustUnnamed_48,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_48 {
    pub tqe_next: *mut ipc_client,
    pub tqe_prev: *mut *mut ipc_client,
}
pub type shutdown_reason_t = ::core::ffi::c_uint;
pub const SHUTDOWN_REASON_EXIT: shutdown_reason_t = 1;
pub const SHUTDOWN_REASON_RESTART: shutdown_reason_t = 0;
pub type xcursor_cursor_t = ::core::ffi::c_uint;
pub const XCURSOR_CURSOR_MAX: xcursor_cursor_t = 9;
pub const XCURSOR_CURSOR_MOVE: xcursor_cursor_t = 8;
pub const XCURSOR_CURSOR_WATCH: xcursor_cursor_t = 7;
pub const XCURSOR_CURSOR_BOTTOM_RIGHT_CORNER: xcursor_cursor_t = 6;
pub const XCURSOR_CURSOR_BOTTOM_LEFT_CORNER: xcursor_cursor_t = 5;
pub const XCURSOR_CURSOR_TOP_RIGHT_CORNER: xcursor_cursor_t = 4;
pub const XCURSOR_CURSOR_TOP_LEFT_CORNER: xcursor_cursor_t = 3;
pub const XCURSOR_CURSOR_RESIZE_VERTICAL: xcursor_cursor_t = 2;
pub const XCURSOR_CURSOR_RESIZE_HORIZONTAL: xcursor_cursor_t = 1;
pub const XCURSOR_CURSOR_POINTER: xcursor_cursor_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct outputs_head {
    pub tqh_first: *mut xoutput,
    pub tqh_last: *mut *mut xoutput,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const ::core::ffi::c_char,
    pub has_arg: ::core::ffi::c_int,
    pub flag: *mut ::core::ffi::c_int,
    pub val: ::core::ffi::c_int,
}
pub type __socket_type = ::core::ffi::c_uint;
pub const SOCK_NONBLOCK: __socket_type = 2048;
pub const SOCK_CLOEXEC: __socket_type = 524288;
pub const SOCK_PACKET: __socket_type = 10;
pub const SOCK_DCCP: __socket_type = 6;
pub const SOCK_SEQPACKET: __socket_type = 5;
pub const SOCK_RDM: __socket_type = 4;
pub const SOCK_RAW: __socket_type = 3;
pub const SOCK_DGRAM: __socket_type = 2;
pub const SOCK_STREAM: __socket_type = 1;
pub type sa_family_t = ::core::ffi::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [::core::ffi::c_char; 14],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_un {
    pub sun_family: sa_family_t,
    pub sun_path: [::core::ffi::c_char; 108],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __CONST_SOCKADDR_ARG {
    pub __sockaddr__: *const sockaddr,
    pub __sockaddr_at__: *const sockaddr_at,
    pub __sockaddr_ax25__: *const sockaddr_ax25,
    pub __sockaddr_dl__: *const sockaddr_dl,
    pub __sockaddr_eon__: *const sockaddr_eon,
    pub __sockaddr_in__: *const sockaddr_in,
    pub __sockaddr_in6__: *const sockaddr_in6,
    pub __sockaddr_inarp__: *const sockaddr_inarp,
    pub __sockaddr_ipx__: *const sockaddr_ipx,
    pub __sockaddr_iso__: *const sockaddr_iso,
    pub __sockaddr_ns__: *const sockaddr_ns,
    pub __sockaddr_un__: *const sockaddr_un,
    pub __sockaddr_x25__: *const sockaddr_x25,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_49 {
    pub message: xcb_client_message_event_t,
    pub storage: [::core::ffi::c_char; 32],
}
pub const I3_VERSION: &[u8] = concat!(env!("I3_VERSION"), "\0").as_bytes();
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const XCB_MAP_REQUEST: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const XCB_PROPERTY_NOTIFY: ::core::ffi::c_int = 28 as ::core::ffi::c_int;
pub const XCB_CLIENT_MESSAGE: ::core::ffi::c_int = 33 as ::core::ffi::c_int;
pub const XCB_NONE: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const XCB_CURRENT_TIME: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const XCB_XKB_MAJOR_VERSION: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const XCB_XKB_MINOR_VERSION: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SIGILL: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SIGABRT: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const SIGFPE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const SIGSEGV: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const SIGQUIT: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const SIGPIPE: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const SIGCHLD: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const SA_SIGINFO: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SA_NODEFER: ::core::ffi::c_int = 0x40000000 as ::core::ffi::c_int;
pub const SA_RESETHAND: ::core::ffi::c_uint = 0x80000000 as ::core::ffi::c_uint;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[inline]
unsafe extern "C" fn ev_loop(mut loop_0: *mut ev_loop, mut flags: ::core::ffi::c_int) {
    ev_run(loop_0, flags);
}
pub const RLIM_INFINITY: ::core::ffi::c_ulonglong = 0xffffffffffffffff as ::core::ffi::c_ulonglong;
pub const I3_IPC_MESSAGE_TYPE_RUN_COMMAND: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const I3_IPC_REPLY_TYPE_COMMAND: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const XCB_NUM_LOCK: ::core::ffi::c_int = 0xff7f as ::core::ffi::c_int;
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const F_GETFD: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const F_SETFD: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const FD_CLOEXEC: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const no_argument: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const required_argument: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const __LC_ALL: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const LC_ALL: ::core::ffi::c_int = __LC_ALL;
pub const PF_LOCAL: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const AF_LOCAL: ::core::ffi::c_int = PF_LOCAL;
pub const SD_LISTEN_FDS_START: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
#[no_mangle]
pub static mut original_rlimit_core: rlimit = rlimit {
    rlim_cur: 0,
    rlim_max: 0,
};
#[no_mangle]
pub static mut listen_fds: ::core::ffi::c_int = 0;
static mut xcb_prepare: *mut ev_prepare = ::core::ptr::null::<ev_prepare>() as *mut ev_prepare;
#[no_mangle]
pub static mut start_argv: *mut *mut ::core::ffi::c_char =
    ::core::ptr::null::<*mut ::core::ffi::c_char>() as *mut *mut ::core::ffi::c_char;
#[no_mangle]
pub static mut conn: *mut xcb_connection_t =
    ::core::ptr::null::<xcb_connection_t>() as *mut xcb_connection_t;
#[no_mangle]
pub static mut conn_screen: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sndisplay: *mut SnDisplay = ::core::ptr::null::<SnDisplay>() as *mut SnDisplay;
#[no_mangle]
pub static mut last_timestamp: xcb_timestamp_t = XCB_CURRENT_TIME as xcb_timestamp_t;
#[no_mangle]
pub static mut root_screen: *mut xcb_screen_t =
    ::core::ptr::null::<xcb_screen_t>() as *mut xcb_screen_t;
#[no_mangle]
pub static mut root: xcb_window_t = 0;
#[no_mangle]
pub static mut wm_sn_selection_owner: xcb_window_t = 0;
#[no_mangle]
pub static mut wm_sn: xcb_atom_t = 0;
#[no_mangle]
pub static mut root_depth: uint8_t = 0;
#[no_mangle]
pub static mut visual_type: *mut xcb_visualtype_t =
    ::core::ptr::null::<xcb_visualtype_t>() as *mut xcb_visualtype_t;
#[no_mangle]
pub static mut colormap: xcb_colormap_t = 0;
#[no_mangle]
pub static mut main_loop: *mut ev_loop = ::core::ptr::null::<ev_loop>() as *mut ev_loop;
#[no_mangle]
pub static mut keysyms: *mut xcb_key_symbols_t =
    ::core::ptr::null::<xcb_key_symbols_t>() as *mut xcb_key_symbols_t;
#[no_mangle]
pub static mut default_shmlog_size: ::core::ffi::c_int =
    25 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int;
#[no_mangle]
pub static mut bindings: *mut bindings_head =
    ::core::ptr::null::<bindings_head>() as *mut bindings_head;
#[no_mangle]
pub static mut current_binding_mode: *const ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>();
#[no_mangle]
pub static mut autostarts: autostarts_head = autostarts_head {
    tqh_first: ::core::ptr::null::<Autostart>() as *mut Autostart,
    tqh_last: ::core::ptr::null::<*mut Autostart>() as *mut *mut Autostart,
};
#[no_mangle]
pub static mut autostarts_always: autostarts_always_head = autostarts_always_head {
    tqh_first: ::core::ptr::null::<Autostart>() as *mut Autostart,
    tqh_last: ::core::ptr::null::<*mut Autostart>() as *mut *mut Autostart,
};
#[no_mangle]
pub static mut assignments: assignments_head = assignments_head {
    tqh_first: ::core::ptr::null::<Assignment>() as *mut Assignment,
    tqh_last: ::core::ptr::null::<*mut Assignment>() as *mut *mut Assignment,
};
#[no_mangle]
pub static mut ws_assignments: ws_assignments_head = ws_assignments_head {
    tqh_first: ::core::ptr::null::<Workspace_Assignment>() as *mut Workspace_Assignment,
    tqh_last: ::core::ptr::null::<*mut Workspace_Assignment>() as *mut *mut Workspace_Assignment,
};
#[no_mangle]
pub static mut xkb_supported: bool = true_0 != 0;
#[no_mangle]
pub static mut shape_supported: bool = true_0 != 0;
#[no_mangle]
pub static mut force_xinerama: bool = false_0 != 0;
#[no_mangle]
pub static mut A__NET_WM_STATE_MAXIMIZED_HORZ: xcb_atom_t = 0;
#[no_mangle]
pub static mut A__NET_WM_STATE_FOCUSED: xcb_atom_t = 0;
#[no_mangle]
pub static mut A__NET_WM_STATE_MAXIMIZED_VERT: xcb_atom_t = 0;
#[no_mangle]
pub static mut A__NET_WM_STATE: xcb_atom_t = 0;
#[no_mangle]
pub static mut A__NET_WM_WINDOW_TYPE: xcb_atom_t = 0;
#[no_mangle]
pub static mut A__NET_WM_WINDOW_TYPE_NORMAL: xcb_atom_t = 0;
#[no_mangle]
pub static mut A__NET_WM_WINDOW_TYPE_DOCK: xcb_atom_t = 0;
#[no_mangle]
pub static mut A__NET_WM_WINDOW_TYPE_DIALOG: xcb_atom_t = 0;
#[no_mangle]
pub static mut A__NET_WM_WINDOW_TYPE_UTILITY: xcb_atom_t = 0;
#[no_mangle]
pub static mut A__NET_WM_WINDOW_TYPE_TOOLBAR: xcb_atom_t = 0;
#[no_mangle]
pub static mut A__NET_WM_WINDOW_TYPE_SPLASH: xcb_atom_t = 0;
#[no_mangle]
pub static mut A__NET_WM_WINDOW_TYPE_MENU: xcb_atom_t = 0;
#[no_mangle]
pub static mut A__NET_WM_WINDOW_TYPE_DROPDOWN_MENU: xcb_atom_t = 0;
#[no_mangle]
pub static mut A__NET_WM_WINDOW_TYPE_POPUP_MENU: xcb_atom_t = 0;
#[no_mangle]
pub static mut A__NET_WM_WINDOW_TYPE_TOOLTIP: xcb_atom_t = 0;
#[no_mangle]
pub static mut A__NET_WM_WINDOW_TYPE_NOTIFICATION: xcb_atom_t = 0;
#[no_mangle]
pub static mut A__NET_WM_STATE_HIDDEN: xcb_atom_t = 0;
#[no_mangle]
pub static mut A__NET_WM_STRUT_PARTIAL: xcb_atom_t = 0;
#[no_mangle]
pub static mut A__NET_CLIENT_LIST: xcb_atom_t = 0;
#[no_mangle]
pub static mut A__NET_CLIENT_LIST_STACKING: xcb_atom_t = 0;
#[no_mangle]
pub static mut A__NET_CURRENT_DESKTOP: xcb_atom_t = 0;
#[no_mangle]
pub static mut A__NET_NUMBER_OF_DESKTOPS: xcb_atom_t = 0;
#[no_mangle]
pub static mut A__NET_DESKTOP_NAMES: xcb_atom_t = 0;
#[no_mangle]
pub static mut A__NET_DESKTOP_VIEWPORT: xcb_atom_t = 0;
#[no_mangle]
pub static mut A__NET_ACTIVE_WINDOW: xcb_atom_t = 0;
#[no_mangle]
pub static mut A__NET_CLOSE_WINDOW: xcb_atom_t = 0;
#[no_mangle]
pub static mut A__NET_MOVERESIZE_WINDOW: xcb_atom_t = 0;
#[no_mangle]
pub static mut A__NET_WM_STATE_STICKY: xcb_atom_t = 0;
#[no_mangle]
pub static mut A__NET_SUPPORTED: xcb_atom_t = 0;
#[no_mangle]
pub static mut A__NET_SUPPORTING_WM_CHECK: xcb_atom_t = 0;
#[no_mangle]
pub static mut A__NET_WM_NAME: xcb_atom_t = 0;
#[no_mangle]
pub static mut A__NET_WM_VISIBLE_NAME: xcb_atom_t = 0;
#[no_mangle]
pub static mut A__NET_WM_STATE_FULLSCREEN: xcb_atom_t = 0;
#[no_mangle]
pub static mut A__NET_WM_MOVERESIZE: xcb_atom_t = 0;
#[no_mangle]
pub static mut A__NET_WM_STATE_DEMANDS_ATTENTION: xcb_atom_t = 0;
#[no_mangle]
pub static mut A__NET_WM_STATE_MODAL: xcb_atom_t = 0;
#[no_mangle]
pub static mut A__NET_WM_DESKTOP: xcb_atom_t = 0;
#[no_mangle]
pub static mut A_WM_TAKE_FOCUS: xcb_atom_t = 0;
#[no_mangle]
pub static mut A_I3_CONFIG_PATH: xcb_atom_t = 0;
#[no_mangle]
pub static mut A__NET_WM_USER_TIME: xcb_atom_t = 0;
#[no_mangle]
pub static mut A__NET_STARTUP_ID: xcb_atom_t = 0;
#[no_mangle]
pub static mut A__NET_WORKAREA: xcb_atom_t = 0;
#[no_mangle]
pub static mut A__NET_WM_ICON: xcb_atom_t = 0;
#[no_mangle]
pub static mut A_WM_PROTOCOLS: xcb_atom_t = 0;
#[no_mangle]
pub static mut A_WM_DELETE_WINDOW: xcb_atom_t = 0;
#[no_mangle]
pub static mut A_UTF8_STRING: xcb_atom_t = 0;
#[no_mangle]
pub static mut A_WM_STATE: xcb_atom_t = 0;
#[no_mangle]
pub static mut A_WM_CLIENT_LEADER: xcb_atom_t = 0;
#[no_mangle]
pub static mut A_WM_WINDOW_ROLE: xcb_atom_t = 0;
#[no_mangle]
pub static mut A_I3_SOCKET_PATH: xcb_atom_t = 0;
#[no_mangle]
pub static mut A_I3_SYNC: xcb_atom_t = 0;
#[no_mangle]
pub static mut A__NET_FRAME_EXTENTS: xcb_atom_t = 0;
#[no_mangle]
pub static mut A_I3_FLOATING_WINDOW: xcb_atom_t = 0;
#[no_mangle]
pub static mut A_I3_LOG_STREAM_SOCKET_PATH: xcb_atom_t = 0;
#[no_mangle]
pub static mut A_I3_SHMLOG_PATH: xcb_atom_t = 0;
#[no_mangle]
pub static mut A_I3_PID: xcb_atom_t = 0;
#[no_mangle]
pub static mut A__NET_REQUEST_FRAME_EXTENTS: xcb_atom_t = 0;
#[no_mangle]
pub static mut A__MOTIF_WM_HINTS: xcb_atom_t = 0;
#[no_mangle]
pub static mut A_WM_CHANGE_STATE: xcb_atom_t = 0;
#[no_mangle]
pub static mut A_MANAGER: xcb_atom_t = 0;
unsafe extern "C" fn xcb_got_event(
    mut loop_0: *mut ev_loop,
    mut w: *mut ev_io,
    mut revents: ::core::ffi::c_int,
) {
}
unsafe extern "C" fn xcb_prepare_cb(
    mut loop_0: *mut ev_loop,
    mut w: *mut ev_prepare,
    mut revents: ::core::ffi::c_int,
) {
    let mut event: *mut xcb_generic_event_t = ::core::ptr::null_mut::<xcb_generic_event_t>();
    loop {
        event = xcb_poll_for_event(conn);
        if event.is_null() {
            break;
        }
        if (*event).response_type as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            if event_is_ignored(
                ignore_event_sequence_t {
                    value: (*event).full_sequence,
                },
                0 as ::core::ffi::c_int,
            ) {
                debuglog(
                    b"%s:%s:%d - Expected X11 Error received for sequence %x\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    b"../src/main.c\0" as *const u8 as *const ::core::ffi::c_char,
                    b"xcb_prepare_cb\0" as *const u8 as *const ::core::ffi::c_char,
                    137 as ::core::ffi::c_int,
                    (*event).sequence as ::core::ffi::c_int,
                );
            } else {
                let mut error: *mut xcb_generic_error_t = event as *mut xcb_generic_error_t;
                debuglog(
                    b"%s:%s:%d - X11 Error received (probably harmless)! sequence 0x%x, error_code = %d\n\0"
                        as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    b"../src/main.c\0" as *const u8 as *const ::core::ffi::c_char,
                    b"xcb_prepare_cb\0" as *const u8 as *const ::core::ffi::c_char,
                    141 as ::core::ffi::c_int,
                    (*error).sequence as ::core::ffi::c_int,
                    (*error).error_code as ::core::ffi::c_int,
                );
            }
            free(event as *mut ::core::ffi::c_void);
        } else {
            let type_0: ::core::ffi::c_int =
                (*event).response_type as ::core::ffi::c_int & 0x7f as ::core::ffi::c_int;
            handle_event(type_0, event);
            free(event as *mut ::core::ffi::c_void);
        }
    }
    xcb_flush(conn);
}
#[no_mangle]
pub unsafe extern "C" fn main_set_x11_cb(mut enable: bool) {
    debuglog(
        b"%s:%s:%d - Setting main X11 callback to enabled=%d\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/main.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"main_set_x11_cb\0" as *const u8 as *const ::core::ffi::c_char,
        165 as ::core::ffi::c_int,
        enable as ::core::ffi::c_int,
    );
    if enable {
        ev_prepare_start(main_loop, xcb_prepare);
        ev_feed_event(
            main_loop,
            xcb_prepare as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
        );
    } else {
        ev_prepare_stop(main_loop, xcb_prepare);
    };
}
unsafe extern "C" fn i3_exit() {
    if *shmlogname as ::core::ffi::c_int != '\0' as i32 {
        fprintf(
            stderr,
            b"Closing SHM log \"%s\"\n\0" as *const u8 as *const ::core::ffi::c_char,
            shmlogname,
        );
        fflush(stderr);
        shm_unlink(shmlogname);
    }
    ipc_shutdown(SHUTDOWN_REASON_EXIT, -(1 as ::core::ffi::c_int));
    unlink(config.ipc_socket_path);
    if !current_log_stream_socket_path.is_null() {
        unlink(current_log_stream_socket_path);
    }
    xcb_disconnect(conn);
    kill_nagbar(config_error_nagbar_pid, false_0 != 0);
    kill_nagbar(command_error_nagbar_pid, false_0 != 0);
    ev_loop_destroy(main_loop);
}
unsafe extern "C" fn handle_core_signal(
    mut sig: ::core::ffi::c_int,
    mut info: *mut siginfo_t,
    mut data: *mut ::core::ffi::c_void,
) {
    if *shmlogname as ::core::ffi::c_int != '\0' as i32 {
        shm_unlink(shmlogname);
    }
    raise(sig);
}
unsafe extern "C" fn handle_term_signal(
    mut loop_0: *mut ev_loop,
    mut signal_0: *mut ev_signal,
    mut revents: ::core::ffi::c_int,
) {
    exit(128 as ::core::ffi::c_int + (*signal_0).signum);
}
unsafe extern "C" fn setup_term_handlers() {
    static mut signal_watchers: [ev_signal; 6] = [ev_signal {
        active: 0,
        pending: 0,
        priority: 0,
        data: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
        cb: None,
        next: ::core::ptr::null::<ev_watcher_list>() as *mut ev_watcher_list,
        signum: 0,
    }; 6];
    let num_watchers: size_t = (::core::mem::size_of::<[ev_signal; 6]>() as size_t)
        .wrapping_div(::core::mem::size_of::<ev_signal>() as size_t);
    let ref mut fresh0 = (*((&raw mut signal_watchers as *mut ev_signal)
        .offset(0 as ::core::ffi::c_int as isize) as *mut ev_signal
        as *mut ::core::ffi::c_void as *mut ev_watcher))
        .pending;
    *fresh0 = 0 as ::core::ffi::c_int;
    (*((&raw mut signal_watchers as *mut ev_signal).offset(0 as ::core::ffi::c_int as isize)
        as *mut ev_signal as *mut ::core::ffi::c_void as *mut ev_watcher))
        .active = *fresh0;
    (*((&raw mut signal_watchers as *mut ev_signal).offset(0 as ::core::ffi::c_int as isize)
        as *mut ev_signal as *mut ::core::ffi::c_void as *mut ev_watcher))
        .priority = 0 as ::core::ffi::c_int;
    signal_watchers[0 as ::core::ffi::c_int as usize].cb = Some(
        handle_term_signal
            as unsafe extern "C" fn(*mut ev_loop, *mut ev_signal, ::core::ffi::c_int) -> (),
    )
        as Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_signal, ::core::ffi::c_int) -> ()>;
    memmove(
        &raw mut (*((&raw mut signal_watchers as *mut ev_signal)
            .offset(0 as ::core::ffi::c_int as isize) as *mut ev_signal
            as *mut ev_watcher))
            .cb as *mut ::core::ffi::c_void,
        &raw mut (*(&raw mut signal_watchers as *mut ev_signal)
            .offset(0 as ::core::ffi::c_int as isize))
        .cb as *const ::core::ffi::c_void,
        ::core::mem::size_of::<
            Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_signal, ::core::ffi::c_int) -> ()>,
        >() as size_t,
    );
    signal_watchers[0 as ::core::ffi::c_int as usize].signum = 1 as ::core::ffi::c_int;
    let ref mut fresh1 = (*((&raw mut signal_watchers as *mut ev_signal)
        .offset(1 as ::core::ffi::c_int as isize) as *mut ev_signal
        as *mut ::core::ffi::c_void as *mut ev_watcher))
        .pending;
    *fresh1 = 0 as ::core::ffi::c_int;
    (*((&raw mut signal_watchers as *mut ev_signal).offset(1 as ::core::ffi::c_int as isize)
        as *mut ev_signal as *mut ::core::ffi::c_void as *mut ev_watcher))
        .active = *fresh1;
    (*((&raw mut signal_watchers as *mut ev_signal).offset(1 as ::core::ffi::c_int as isize)
        as *mut ev_signal as *mut ::core::ffi::c_void as *mut ev_watcher))
        .priority = 0 as ::core::ffi::c_int;
    signal_watchers[1 as ::core::ffi::c_int as usize].cb = Some(
        handle_term_signal
            as unsafe extern "C" fn(*mut ev_loop, *mut ev_signal, ::core::ffi::c_int) -> (),
    )
        as Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_signal, ::core::ffi::c_int) -> ()>;
    memmove(
        &raw mut (*((&raw mut signal_watchers as *mut ev_signal)
            .offset(1 as ::core::ffi::c_int as isize) as *mut ev_signal
            as *mut ev_watcher))
            .cb as *mut ::core::ffi::c_void,
        &raw mut (*(&raw mut signal_watchers as *mut ev_signal)
            .offset(1 as ::core::ffi::c_int as isize))
        .cb as *const ::core::ffi::c_void,
        ::core::mem::size_of::<
            Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_signal, ::core::ffi::c_int) -> ()>,
        >() as size_t,
    );
    signal_watchers[1 as ::core::ffi::c_int as usize].signum = 2 as ::core::ffi::c_int;
    let ref mut fresh2 = (*((&raw mut signal_watchers as *mut ev_signal)
        .offset(2 as ::core::ffi::c_int as isize) as *mut ev_signal
        as *mut ::core::ffi::c_void as *mut ev_watcher))
        .pending;
    *fresh2 = 0 as ::core::ffi::c_int;
    (*((&raw mut signal_watchers as *mut ev_signal).offset(2 as ::core::ffi::c_int as isize)
        as *mut ev_signal as *mut ::core::ffi::c_void as *mut ev_watcher))
        .active = *fresh2;
    (*((&raw mut signal_watchers as *mut ev_signal).offset(2 as ::core::ffi::c_int as isize)
        as *mut ev_signal as *mut ::core::ffi::c_void as *mut ev_watcher))
        .priority = 0 as ::core::ffi::c_int;
    signal_watchers[2 as ::core::ffi::c_int as usize].cb = Some(
        handle_term_signal
            as unsafe extern "C" fn(*mut ev_loop, *mut ev_signal, ::core::ffi::c_int) -> (),
    )
        as Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_signal, ::core::ffi::c_int) -> ()>;
    memmove(
        &raw mut (*((&raw mut signal_watchers as *mut ev_signal)
            .offset(2 as ::core::ffi::c_int as isize) as *mut ev_signal
            as *mut ev_watcher))
            .cb as *mut ::core::ffi::c_void,
        &raw mut (*(&raw mut signal_watchers as *mut ev_signal)
            .offset(2 as ::core::ffi::c_int as isize))
        .cb as *const ::core::ffi::c_void,
        ::core::mem::size_of::<
            Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_signal, ::core::ffi::c_int) -> ()>,
        >() as size_t,
    );
    signal_watchers[2 as ::core::ffi::c_int as usize].signum = 14 as ::core::ffi::c_int;
    let ref mut fresh3 = (*((&raw mut signal_watchers as *mut ev_signal)
        .offset(3 as ::core::ffi::c_int as isize) as *mut ev_signal
        as *mut ::core::ffi::c_void as *mut ev_watcher))
        .pending;
    *fresh3 = 0 as ::core::ffi::c_int;
    (*((&raw mut signal_watchers as *mut ev_signal).offset(3 as ::core::ffi::c_int as isize)
        as *mut ev_signal as *mut ::core::ffi::c_void as *mut ev_watcher))
        .active = *fresh3;
    (*((&raw mut signal_watchers as *mut ev_signal).offset(3 as ::core::ffi::c_int as isize)
        as *mut ev_signal as *mut ::core::ffi::c_void as *mut ev_watcher))
        .priority = 0 as ::core::ffi::c_int;
    signal_watchers[3 as ::core::ffi::c_int as usize].cb = Some(
        handle_term_signal
            as unsafe extern "C" fn(*mut ev_loop, *mut ev_signal, ::core::ffi::c_int) -> (),
    )
        as Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_signal, ::core::ffi::c_int) -> ()>;
    memmove(
        &raw mut (*((&raw mut signal_watchers as *mut ev_signal)
            .offset(3 as ::core::ffi::c_int as isize) as *mut ev_signal
            as *mut ev_watcher))
            .cb as *mut ::core::ffi::c_void,
        &raw mut (*(&raw mut signal_watchers as *mut ev_signal)
            .offset(3 as ::core::ffi::c_int as isize))
        .cb as *const ::core::ffi::c_void,
        ::core::mem::size_of::<
            Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_signal, ::core::ffi::c_int) -> ()>,
        >() as size_t,
    );
    signal_watchers[3 as ::core::ffi::c_int as usize].signum = 15 as ::core::ffi::c_int;
    let ref mut fresh4 = (*((&raw mut signal_watchers as *mut ev_signal)
        .offset(4 as ::core::ffi::c_int as isize) as *mut ev_signal
        as *mut ::core::ffi::c_void as *mut ev_watcher))
        .pending;
    *fresh4 = 0 as ::core::ffi::c_int;
    (*((&raw mut signal_watchers as *mut ev_signal).offset(4 as ::core::ffi::c_int as isize)
        as *mut ev_signal as *mut ::core::ffi::c_void as *mut ev_watcher))
        .active = *fresh4;
    (*((&raw mut signal_watchers as *mut ev_signal).offset(4 as ::core::ffi::c_int as isize)
        as *mut ev_signal as *mut ::core::ffi::c_void as *mut ev_watcher))
        .priority = 0 as ::core::ffi::c_int;
    signal_watchers[4 as ::core::ffi::c_int as usize].cb = Some(
        handle_term_signal
            as unsafe extern "C" fn(*mut ev_loop, *mut ev_signal, ::core::ffi::c_int) -> (),
    )
        as Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_signal, ::core::ffi::c_int) -> ()>;
    memmove(
        &raw mut (*((&raw mut signal_watchers as *mut ev_signal)
            .offset(4 as ::core::ffi::c_int as isize) as *mut ev_signal
            as *mut ev_watcher))
            .cb as *mut ::core::ffi::c_void,
        &raw mut (*(&raw mut signal_watchers as *mut ev_signal)
            .offset(4 as ::core::ffi::c_int as isize))
        .cb as *const ::core::ffi::c_void,
        ::core::mem::size_of::<
            Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_signal, ::core::ffi::c_int) -> ()>,
        >() as size_t,
    );
    signal_watchers[4 as ::core::ffi::c_int as usize].signum = 10 as ::core::ffi::c_int;
    let ref mut fresh5 = (*((&raw mut signal_watchers as *mut ev_signal)
        .offset(5 as ::core::ffi::c_int as isize) as *mut ev_signal
        as *mut ::core::ffi::c_void as *mut ev_watcher))
        .pending;
    *fresh5 = 0 as ::core::ffi::c_int;
    (*((&raw mut signal_watchers as *mut ev_signal).offset(5 as ::core::ffi::c_int as isize)
        as *mut ev_signal as *mut ::core::ffi::c_void as *mut ev_watcher))
        .active = *fresh5;
    (*((&raw mut signal_watchers as *mut ev_signal).offset(5 as ::core::ffi::c_int as isize)
        as *mut ev_signal as *mut ::core::ffi::c_void as *mut ev_watcher))
        .priority = 0 as ::core::ffi::c_int;
    signal_watchers[5 as ::core::ffi::c_int as usize].cb = Some(
        handle_term_signal
            as unsafe extern "C" fn(*mut ev_loop, *mut ev_signal, ::core::ffi::c_int) -> (),
    )
        as Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_signal, ::core::ffi::c_int) -> ()>;
    memmove(
        &raw mut (*((&raw mut signal_watchers as *mut ev_signal)
            .offset(5 as ::core::ffi::c_int as isize) as *mut ev_signal
            as *mut ev_watcher))
            .cb as *mut ::core::ffi::c_void,
        &raw mut (*(&raw mut signal_watchers as *mut ev_signal)
            .offset(5 as ::core::ffi::c_int as isize))
        .cb as *const ::core::ffi::c_void,
        ::core::mem::size_of::<
            Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_signal, ::core::ffi::c_int) -> ()>,
        >() as size_t,
    );
    signal_watchers[5 as ::core::ffi::c_int as usize].signum = 12 as ::core::ffi::c_int;
    let mut i: size_t = 0 as size_t;
    while i < num_watchers {
        ev_signal_start(
            main_loop,
            (&raw mut signal_watchers as *mut ev_signal).offset(i as isize) as *mut ev_signal,
        );
        ev_unref(main_loop);
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn parse_restart_fd() -> ::core::ffi::c_int {
    let mut restart_fd: *const ::core::ffi::c_char =
        getenv(b"_I3_RESTART_FD\0" as *const u8 as *const ::core::ffi::c_char);
    if restart_fd.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    let mut fd: ::core::ffi::c_long = -(1 as ::core::ffi::c_int) as ::core::ffi::c_long;
    if !parse_long(restart_fd, &raw mut fd, 10 as ::core::ffi::c_int) {
        errorlog(
            b"ERROR: Malformed _I3_RESTART_FD \"%s\"\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            restart_fd,
        );
        return -(1 as ::core::ffi::c_int);
    }
    return fd as ::core::ffi::c_int;
}
unsafe fn main_0(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    #[used]
    static mut _i3_version: *const ::core::ffi::c_char =
        I3_VERSION.as_ptr().cast::<::core::ffi::c_char>();
    let mut override_configpath: *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut autostart: bool = true_0 != 0;
    let mut layout_path: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut delete_layout_path: bool = false_0 != 0;
    let mut disable_randr15: bool = false_0 != 0;
    let mut fake_outputs: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut disable_signalhandler: bool = false_0 != 0;
    let mut only_check_config: bool = false_0 != 0;
    let mut replace_wm: bool = false_0 != 0;
    static mut long_options: [option; 23] = [
        option {
            name: b"no-autostart\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 'a' as i32,
        },
        option {
            name: b"config\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 'c' as i32,
        },
        option {
            name: b"version\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 'v' as i32,
        },
        option {
            name: b"moreversion\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 'm' as i32,
        },
        option {
            name: b"more-version\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 'm' as i32,
        },
        option {
            name: b"more_version\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 'm' as i32,
        },
        option {
            name: b"help\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 'h' as i32,
        },
        option {
            name: b"layout\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 'L' as i32,
        },
        option {
            name: b"restart\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 0 as ::core::ffi::c_int,
        },
        option {
            name: b"force-xinerama\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 0 as ::core::ffi::c_int,
        },
        option {
            name: b"force_xinerama\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 0 as ::core::ffi::c_int,
        },
        option {
            name: b"disable-randr15\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 0 as ::core::ffi::c_int,
        },
        option {
            name: b"disable_randr15\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 0 as ::core::ffi::c_int,
        },
        option {
            name: b"disable-signalhandler\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 0 as ::core::ffi::c_int,
        },
        option {
            name: b"shmlog-size\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 0 as ::core::ffi::c_int,
        },
        option {
            name: b"shmlog_size\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 0 as ::core::ffi::c_int,
        },
        option {
            name: b"get-socketpath\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 0 as ::core::ffi::c_int,
        },
        option {
            name: b"get_socketpath\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 0 as ::core::ffi::c_int,
        },
        option {
            name: b"fake_outputs\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 0 as ::core::ffi::c_int,
        },
        option {
            name: b"fake-outputs\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 0 as ::core::ffi::c_int,
        },
        option {
            name: b"force-old-config-parser-v4.4-only\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 0 as ::core::ffi::c_int,
        },
        option {
            name: b"replace\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 'r' as i32,
        },
        option {
            name: ::core::ptr::null::<::core::ffi::c_char>(),
            has_arg: 0 as ::core::ffi::c_int,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 0 as ::core::ffi::c_int,
        },
    ];
    let mut option_index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut opt: ::core::ffi::c_int = 0;
    setlocale(LC_ALL, b"\0" as *const u8 as *const ::core::ffi::c_char);
    getrlimit(RLIMIT_CORE, &raw mut original_rlimit_core);
    if isatty(fileno(stdout)) == 0 {
        setbuf(stdout, ::core::ptr::null_mut::<::core::ffi::c_char>());
    }
    srand(time(::core::ptr::null_mut::<time_t>()) as ::core::ffi::c_uint);
    init_logging();
    shmlog_size = if is_debug_build() as ::core::ffi::c_int != 0
        || !strstr(
            *argv.offset(0 as ::core::ffi::c_int as isize),
            b"i3-with-shmlog\0" as *const u8 as *const ::core::ffi::c_char,
        )
        .is_null()
    {
        default_shmlog_size
    } else {
        0 as ::core::ffi::c_int
    };
    start_argv = argv as *mut *mut ::core::ffi::c_char;
    loop {
        opt = getopt_long(
            argc,
            argv as *const *mut ::core::ffi::c_char,
            b"c:CvmaL:hld:Vr\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut long_options as *mut option,
            &raw mut option_index,
        );
        if !(opt != -(1 as ::core::ffi::c_int)) {
            break;
        }
        let mut current_block_75: u64;
        match opt {
            97 => {
                verboselog(
                    b"Autostart disabled using -a\n\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
                autostart = false_0 != 0;
                current_block_75 = 919954187481050311;
            }
            76 => {
                free(layout_path as *mut ::core::ffi::c_void);
                layout_path = ::core::ptr::null_mut::<::core::ffi::c_char>();
                layout_path = sstrdup(optarg);
                delete_layout_path = false_0 != 0;
                current_block_75 = 919954187481050311;
            }
            99 => {
                free(override_configpath as *mut ::core::ffi::c_void);
                override_configpath = ::core::ptr::null_mut::<::core::ffi::c_char>();
                override_configpath = sstrdup(optarg);
                current_block_75 = 919954187481050311;
            }
            67 => {
                verboselog(
                    b"Checking configuration file only (-C)\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
                only_check_config = true_0 != 0;
                current_block_75 = 919954187481050311;
            }
            118 => {
                printf(
                    b"i3 version %s \xC2\xA9 2009 Michael Stapelberg and contributors\n\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    i3_version,
                );
                exit(EXIT_SUCCESS);
            }
            109 => {
                printf(
                    b"Binary i3 version:  %s \xC2\xA9 2009 Michael Stapelberg and contributors\n\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    i3_version,
                );
                display_running_version();
                exit(EXIT_SUCCESS);
            }
            86 => {
                set_verbosity(true_0 != 0);
                current_block_75 = 919954187481050311;
            }
            100 => {
                verboselog(
                    b"Enabling debug logging\n\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
                set_debug_logging(true_0 != 0);
                current_block_75 = 919954187481050311;
            }
            108 => {
                current_block_75 = 919954187481050311;
            }
            114 => {
                replace_wm = true_0 != 0;
                current_block_75 = 919954187481050311;
            }
            0 => {
                if strcmp(
                    long_options[option_index as usize].name,
                    b"force-xinerama\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                    || strcmp(
                        long_options[option_index as usize].name,
                        b"force_xinerama\0" as *const u8 as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                {
                    force_xinerama = true_0 != 0;
                    errorlog(
                        b"ERROR: Using Xinerama instead of RandR. This option should be avoided at all cost because it does not refresh the list of screens, so you cannot configure displays at runtime. Please check if your driver really does not support RandR and disable this option as soon as you can.\n\0"
                            as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    );
                    current_block_75 = 919954187481050311;
                } else if strcmp(
                    long_options[option_index as usize].name,
                    b"disable-randr15\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                    || strcmp(
                        long_options[option_index as usize].name,
                        b"disable_randr15\0" as *const u8 as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                {
                    disable_randr15 = true_0 != 0;
                    current_block_75 = 919954187481050311;
                } else if strcmp(
                    long_options[option_index as usize].name,
                    b"disable-signalhandler\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    disable_signalhandler = true_0 != 0;
                    current_block_75 = 919954187481050311;
                } else if strcmp(
                    long_options[option_index as usize].name,
                    b"get-socketpath\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                    || strcmp(
                        long_options[option_index as usize].name,
                        b"get_socketpath\0" as *const u8 as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                {
                    let mut socket_path: *mut ::core::ffi::c_char = root_atom_contents(
                        b"I3_SOCKET_PATH\0" as *const u8 as *const ::core::ffi::c_char,
                        ::core::ptr::null_mut::<xcb_connection_t>(),
                        0 as ::core::ffi::c_int,
                    );
                    if !socket_path.is_null() {
                        printf(
                            b"%s\n\0" as *const u8 as *const ::core::ffi::c_char,
                            socket_path,
                        );
                        free(socket_path as *mut ::core::ffi::c_void);
                        exit(EXIT_SUCCESS);
                    }
                    exit(EXIT_FAILURE);
                } else if strcmp(
                    long_options[option_index as usize].name,
                    b"shmlog-size\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                    || strcmp(
                        long_options[option_index as usize].name,
                        b"shmlog_size\0" as *const u8 as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                {
                    shmlog_size = atoi(optarg);
                    init_logging();
                    verboselog(
                        b"Limiting SHM log size to %d bytes\n\0" as *const u8
                            as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        shmlog_size,
                    );
                    current_block_75 = 919954187481050311;
                } else if strcmp(
                    long_options[option_index as usize].name,
                    b"restart\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    free(layout_path as *mut ::core::ffi::c_void);
                    layout_path = ::core::ptr::null_mut::<::core::ffi::c_char>();
                    layout_path = sstrdup(optarg);
                    delete_layout_path = true_0 != 0;
                    current_block_75 = 919954187481050311;
                } else if strcmp(
                    long_options[option_index as usize].name,
                    b"fake-outputs\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                    || strcmp(
                        long_options[option_index as usize].name,
                        b"fake_outputs\0" as *const u8 as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                {
                    verboselog(
                        b"Initializing fake outputs: %s\n\0" as *const u8
                            as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        optarg,
                    );
                    fake_outputs = sstrdup(optarg);
                    current_block_75 = 919954187481050311;
                } else if strcmp(
                    long_options[option_index as usize].name,
                    b"force-old-config-parser-v4.4-only\0" as *const u8
                        as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    errorlog(
                        b"ERROR: You are passing --force-old-config-parser-v4.4-only, but that flag was removed by now.\n\0"
                            as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    );
                    current_block_75 = 919954187481050311;
                } else {
                    current_block_75 = 12300424817507855837;
                }
            }
            _ => {
                current_block_75 = 12300424817507855837;
            }
        }
        match current_block_75 {
            919954187481050311 => {}
            _ => {
                fprintf(
                    stderr,
                    b"Usage: %s [-c configfile] [-d all] [-a] [-v] [-V] [-C]\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    *argv.offset(0 as ::core::ffi::c_int as isize),
                );
                fprintf(stderr, b"\n\0" as *const u8 as *const ::core::ffi::c_char);
                fprintf(
                    stderr,
                    b"\t-a          disable autostart ('exec' lines in config)\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                fprintf(
                    stderr,
                    b"\t-c <file>   use the provided configfile instead\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                fprintf(
                    stderr,
                    b"\t-C          validate configuration file and exit\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                fprintf(
                    stderr,
                    b"\t-d all      enable debug output\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                fprintf(
                    stderr,
                    b"\t-L <file>   path to the serialized layout during restarts\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                fprintf(
                    stderr,
                    b"\t-v          display version and exit\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                fprintf(
                    stderr,
                    b"\t-V          enable verbose mode\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                fprintf(stderr, b"\n\0" as *const u8 as *const ::core::ffi::c_char);
                fprintf(
                    stderr,
                    b"\t--force-xinerama\n\tUse Xinerama instead of RandR.\n\tThis option should only be used if you are stuck with the\n\told nVidia closed source driver (older than 302.17), which does\n\tnot support RandR.\n\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
                fprintf(stderr, b"\n\0" as *const u8 as *const ::core::ffi::c_char);
                fprintf(
                    stderr,
                    b"\t--get-socketpath\n\tRetrieve the i3 IPC socket path from X11, print it, then exit.\n\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
                fprintf(stderr, b"\n\0" as *const u8 as *const ::core::ffi::c_char);
                fprintf(
                    stderr,
                    b"\t--shmlog-size <limit>\n\tLimits the size of the i3 SHM log to <limit> bytes. Setting this\n\tto 0 disables SHM logging entirely.\n\tThe default is %d bytes.\n\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    shmlog_size,
                );
                fprintf(stderr, b"\n\0" as *const u8 as *const ::core::ffi::c_char);
                fprintf(
                    stderr,
                    b"\t--replace\n\tReplace an existing window manager.\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                fprintf(stderr, b"\n\0" as *const u8 as *const ::core::ffi::c_char);
                fprintf(
                    stderr,
                    b"If you pass plain text arguments, i3 will interpret them as a command\nto send to a currently running i3 (like i3-msg). This allows you to\nuse nice and logical commands, such as:\n\n\ti3 border none\n\ti3 floating toggle\n\ti3 kill window\n\n\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
                exit(if opt == 'h' as i32 {
                    EXIT_SUCCESS
                } else {
                    EXIT_FAILURE
                });
            }
        }
    }
    if only_check_config {
        exit(
            if load_configuration(override_configpath, C_VALIDATE) as ::core::ffi::c_int != 0 {
                EXIT_SUCCESS
            } else {
                EXIT_FAILURE
            },
        );
    }
    if optind < argc {
        set_verbosity(true_0 != 0);
        verboselog(
            b"Additional arguments passed. Sending them as a command to i3.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        let mut payload: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        while optind < argc {
            if payload.is_null() {
                payload = sstrdup(*argv.offset(optind as isize));
            } else {
                let mut both: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                sasprintf(
                    &raw mut both,
                    b"%s %s\0" as *const u8 as *const ::core::ffi::c_char,
                    payload,
                    *argv.offset(optind as isize),
                );
                free(payload as *mut ::core::ffi::c_void);
                payload = both;
            }
            optind += 1;
        }
        debuglog(
            b"%s:%s:%d - Command is: %s (%zd bytes)\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b"../src/main.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"main\0" as *const u8 as *const ::core::ffi::c_char,
            500 as ::core::ffi::c_int,
            payload,
            strlen(payload),
        );
        let mut socket_path_0: *mut ::core::ffi::c_char = root_atom_contents(
            b"I3_SOCKET_PATH\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<xcb_connection_t>(),
            0 as ::core::ffi::c_int,
        );
        if socket_path_0.is_null() {
            errorlog(
                b"ERROR: Could not get i3 IPC socket path\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
            return 1 as ::core::ffi::c_int;
        }
        let mut sockfd: ::core::ffi::c_int = socket(
            AF_LOCAL,
            SOCK_STREAM as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        );
        if sockfd == -(1 as ::core::ffi::c_int) {
            err(
                EXIT_FAILURE,
                b"Could not create socket\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        let mut addr: sockaddr_un = sockaddr_un {
            sun_family: 0 as sa_family_t,
            sun_path: [0; 108],
        };
        addr.sun_family = AF_LOCAL as sa_family_t;
        strncpy(
            &raw mut addr.sun_path as *mut ::core::ffi::c_char,
            socket_path_0,
            (::core::mem::size_of::<[::core::ffi::c_char; 108]>() as size_t)
                .wrapping_sub(1 as size_t),
        );
        free(socket_path_0 as *mut ::core::ffi::c_void);
        socket_path_0 = ::core::ptr::null_mut::<::core::ffi::c_char>();
        if connect(
            sockfd,
            __CONST_SOCKADDR_ARG {
                __sockaddr__: &raw mut addr as *const sockaddr,
            },
            ::core::mem::size_of::<sockaddr_un>() as socklen_t,
        ) < 0 as ::core::ffi::c_int
        {
            err(
                EXIT_FAILURE,
                b"Could not connect to i3\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        if ipc_send_message(
            sockfd,
            strlen(payload) as uint32_t,
            I3_IPC_MESSAGE_TYPE_RUN_COMMAND as uint32_t,
            payload as *mut uint8_t,
        ) == -(1 as ::core::ffi::c_int)
        {
            err(
                EXIT_FAILURE,
                b"IPC: write()\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        free(payload as *mut ::core::ffi::c_void);
        payload = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut reply_length: uint32_t = 0;
        let mut reply_type: uint32_t = 0;
        let mut reply: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut ret: ::core::ffi::c_int = 0;
        ret = ipc_recv_message(
            sockfd,
            &raw mut reply_type,
            &raw mut reply_length,
            &raw mut reply,
        );
        if ret != 0 as ::core::ffi::c_int {
            if ret == -(1 as ::core::ffi::c_int) {
                err(
                    EXIT_FAILURE,
                    b"IPC: read()\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            return 1 as ::core::ffi::c_int;
        }
        if reply_type != I3_IPC_REPLY_TYPE_COMMAND as uint32_t {
            errx(
                EXIT_FAILURE,
                b"IPC: received reply of type %d but expected %d (COMMAND)\0" as *const u8
                    as *const ::core::ffi::c_char,
                reply_type,
                I3_IPC_REPLY_TYPE_COMMAND,
            );
        }
        printf(
            b"%.*s\n\0" as *const u8 as *const ::core::ffi::c_char,
            reply_length,
            reply as *mut ::core::ffi::c_char,
        );
        free(reply as *mut ::core::ffi::c_void);
        reply = ::core::ptr::null_mut::<uint8_t>();
        return 0 as ::core::ffi::c_int;
    }
    init_logging();
    if is_debug_build() {
        let mut limit: rlimit = rlimit {
            rlim_cur: RLIM_INFINITY as rlim_t,
            rlim_max: RLIM_INFINITY as rlim_t,
        };
        setrlimit(RLIMIT_CORE, &raw mut limit);
        verboselog(
            b"CORE DUMPS: You are running a development version of i3, so coredumps were automatically enabled (ulimit -c unlimited).\n\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        let mut cwd: *mut ::core::ffi::c_char =
            getcwd(::core::ptr::null_mut::<::core::ffi::c_char>(), 0 as size_t);
        if !cwd.is_null() {
            verboselog(
                b"CORE DUMPS: Your current working directory is \"%s\".\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                cwd,
            );
            free(cwd as *mut ::core::ffi::c_void);
        }
        let buffer_size: size_t = 1024 as size_t;
        let mut buffer: *mut ::core::ffi::c_char = scalloc(
            buffer_size,
            ::core::mem::size_of::<::core::ffi::c_char>() as size_t,
        ) as *mut ::core::ffi::c_char;
        let mut patternfd: ::core::ffi::c_int = 0;
        patternfd = open(
            b"/proc/sys/kernel/core_pattern\0" as *const u8 as *const ::core::ffi::c_char,
            O_RDONLY,
        );
        if patternfd >= 0 as ::core::ffi::c_int {
            if read(
                patternfd,
                buffer as *mut ::core::ffi::c_void,
                buffer_size.wrapping_sub(1 as size_t),
            ) > 0 as ssize_t
            {
                verboselog(
                    b"CORE DUMPS: Your core_pattern is: %s\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    buffer,
                );
            }
            close(patternfd);
        }
        free(buffer as *mut ::core::ffi::c_void);
    }
    verboselog(
        b"i3 %s starting\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        _i3_version,
    );
    conn = xcb_connect(
        ::core::ptr::null::<::core::ffi::c_char>(),
        &raw mut conn_screen,
    );
    if xcb_connection_has_error(conn) != 0 {
        errx(
            EXIT_FAILURE,
            b"Cannot open display\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    sndisplay = sn_xcb_display_new(conn, None, None);
    main_loop = ev_default_loop(0 as ::core::ffi::c_uint);
    if main_loop.is_null() {
        errx(
            EXIT_FAILURE,
            b"Could not initialize libev. Bad LIBEV_FLAGS?\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    root_screen = xcb_aux_get_screen(conn, conn_screen);
    root = (*root_screen).root;
    xcb_prefetch_extension_data(conn, &raw mut xcb_xkb_id);
    xcb_prefetch_extension_data(conn, &raw mut xcb_shape_id);
    xcb_prefetch_extension_data(conn, &raw mut xcb_big_requests_id);
    if force_xinerama {
        xcb_prefetch_extension_data(conn, &raw mut xcb_xinerama_id);
    } else {
        xcb_prefetch_extension_data(conn, &raw mut xcb_randr_id);
    }
    let mut fresh6: [uint32_t; 1] =
        [XCB_EVENT_MASK_PROPERTY_CHANGE as ::core::ffi::c_int as uint32_t];
    xcb_change_window_attributes(
        conn,
        root,
        XCB_CW_EVENT_MASK as ::core::ffi::c_int as uint32_t,
        &raw mut fresh6 as *mut uint32_t as *const ::core::ffi::c_void,
    );
    xcb_change_property(
        conn,
        XCB_PROP_MODE_APPEND as ::core::ffi::c_int as uint8_t,
        root,
        XCB_ATOM_SUPERSCRIPT_X as ::core::ffi::c_int as xcb_atom_t,
        XCB_ATOM_CARDINAL as ::core::ffi::c_int as xcb_atom_t,
        32 as uint8_t,
        0 as uint32_t,
        b"\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
    );
    let mut _NET_SUPPORTED_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(b"_NET_SUPPORTED\0" as *const u8 as *const ::core::ffi::c_char) as uint16_t,
        b"_NET_SUPPORTED\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut _NET_SUPPORTING_WM_CHECK_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(b"_NET_SUPPORTING_WM_CHECK\0" as *const u8 as *const ::core::ffi::c_char)
            as uint16_t,
        b"_NET_SUPPORTING_WM_CHECK\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut _NET_WM_NAME_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(b"_NET_WM_NAME\0" as *const u8 as *const ::core::ffi::c_char) as uint16_t,
        b"_NET_WM_NAME\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut _NET_WM_VISIBLE_NAME_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(b"_NET_WM_VISIBLE_NAME\0" as *const u8 as *const ::core::ffi::c_char) as uint16_t,
        b"_NET_WM_VISIBLE_NAME\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut _NET_WM_MOVERESIZE_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(b"_NET_WM_MOVERESIZE\0" as *const u8 as *const ::core::ffi::c_char) as uint16_t,
        b"_NET_WM_MOVERESIZE\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut _NET_WM_STATE_STICKY_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(b"_NET_WM_STATE_STICKY\0" as *const u8 as *const ::core::ffi::c_char) as uint16_t,
        b"_NET_WM_STATE_STICKY\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut _NET_WM_STATE_FULLSCREEN_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(b"_NET_WM_STATE_FULLSCREEN\0" as *const u8 as *const ::core::ffi::c_char)
            as uint16_t,
        b"_NET_WM_STATE_FULLSCREEN\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut _NET_WM_STATE_DEMANDS_ATTENTION_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(b"_NET_WM_STATE_DEMANDS_ATTENTION\0" as *const u8 as *const ::core::ffi::c_char)
            as uint16_t,
        b"_NET_WM_STATE_DEMANDS_ATTENTION\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut _NET_WM_STATE_MODAL_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(b"_NET_WM_STATE_MODAL\0" as *const u8 as *const ::core::ffi::c_char) as uint16_t,
        b"_NET_WM_STATE_MODAL\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut _NET_WM_STATE_HIDDEN_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(b"_NET_WM_STATE_HIDDEN\0" as *const u8 as *const ::core::ffi::c_char) as uint16_t,
        b"_NET_WM_STATE_HIDDEN\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut _NET_WM_STATE_FOCUSED_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(b"_NET_WM_STATE_FOCUSED\0" as *const u8 as *const ::core::ffi::c_char) as uint16_t,
        b"_NET_WM_STATE_FOCUSED\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut _NET_WM_STATE_MAXIMIZED_VERT_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(b"_NET_WM_STATE_MAXIMIZED_VERT\0" as *const u8 as *const ::core::ffi::c_char)
            as uint16_t,
        b"_NET_WM_STATE_MAXIMIZED_VERT\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut _NET_WM_STATE_MAXIMIZED_HORZ_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(b"_NET_WM_STATE_MAXIMIZED_HORZ\0" as *const u8 as *const ::core::ffi::c_char)
            as uint16_t,
        b"_NET_WM_STATE_MAXIMIZED_HORZ\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut _NET_WM_STATE_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(b"_NET_WM_STATE\0" as *const u8 as *const ::core::ffi::c_char) as uint16_t,
        b"_NET_WM_STATE\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut _NET_WM_WINDOW_TYPE_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(b"_NET_WM_WINDOW_TYPE\0" as *const u8 as *const ::core::ffi::c_char) as uint16_t,
        b"_NET_WM_WINDOW_TYPE\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut _NET_WM_WINDOW_TYPE_NORMAL_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(b"_NET_WM_WINDOW_TYPE_NORMAL\0" as *const u8 as *const ::core::ffi::c_char)
            as uint16_t,
        b"_NET_WM_WINDOW_TYPE_NORMAL\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut _NET_WM_WINDOW_TYPE_DOCK_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(b"_NET_WM_WINDOW_TYPE_DOCK\0" as *const u8 as *const ::core::ffi::c_char)
            as uint16_t,
        b"_NET_WM_WINDOW_TYPE_DOCK\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut _NET_WM_WINDOW_TYPE_DIALOG_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(b"_NET_WM_WINDOW_TYPE_DIALOG\0" as *const u8 as *const ::core::ffi::c_char)
            as uint16_t,
        b"_NET_WM_WINDOW_TYPE_DIALOG\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut _NET_WM_WINDOW_TYPE_UTILITY_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(b"_NET_WM_WINDOW_TYPE_UTILITY\0" as *const u8 as *const ::core::ffi::c_char)
            as uint16_t,
        b"_NET_WM_WINDOW_TYPE_UTILITY\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut _NET_WM_WINDOW_TYPE_TOOLBAR_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(b"_NET_WM_WINDOW_TYPE_TOOLBAR\0" as *const u8 as *const ::core::ffi::c_char)
            as uint16_t,
        b"_NET_WM_WINDOW_TYPE_TOOLBAR\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut _NET_WM_WINDOW_TYPE_SPLASH_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(b"_NET_WM_WINDOW_TYPE_SPLASH\0" as *const u8 as *const ::core::ffi::c_char)
            as uint16_t,
        b"_NET_WM_WINDOW_TYPE_SPLASH\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut _NET_WM_WINDOW_TYPE_MENU_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(b"_NET_WM_WINDOW_TYPE_MENU\0" as *const u8 as *const ::core::ffi::c_char)
            as uint16_t,
        b"_NET_WM_WINDOW_TYPE_MENU\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut _NET_WM_WINDOW_TYPE_DROPDOWN_MENU_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(b"_NET_WM_WINDOW_TYPE_DROPDOWN_MENU\0" as *const u8 as *const ::core::ffi::c_char)
            as uint16_t,
        b"_NET_WM_WINDOW_TYPE_DROPDOWN_MENU\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut _NET_WM_WINDOW_TYPE_POPUP_MENU_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(b"_NET_WM_WINDOW_TYPE_POPUP_MENU\0" as *const u8 as *const ::core::ffi::c_char)
            as uint16_t,
        b"_NET_WM_WINDOW_TYPE_POPUP_MENU\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut _NET_WM_WINDOW_TYPE_TOOLTIP_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(b"_NET_WM_WINDOW_TYPE_TOOLTIP\0" as *const u8 as *const ::core::ffi::c_char)
            as uint16_t,
        b"_NET_WM_WINDOW_TYPE_TOOLTIP\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut _NET_WM_WINDOW_TYPE_NOTIFICATION_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(b"_NET_WM_WINDOW_TYPE_NOTIFICATION\0" as *const u8 as *const ::core::ffi::c_char)
            as uint16_t,
        b"_NET_WM_WINDOW_TYPE_NOTIFICATION\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut _NET_WM_DESKTOP_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(b"_NET_WM_DESKTOP\0" as *const u8 as *const ::core::ffi::c_char) as uint16_t,
        b"_NET_WM_DESKTOP\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut _NET_WM_STRUT_PARTIAL_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(b"_NET_WM_STRUT_PARTIAL\0" as *const u8 as *const ::core::ffi::c_char) as uint16_t,
        b"_NET_WM_STRUT_PARTIAL\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut _NET_CLIENT_LIST_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(b"_NET_CLIENT_LIST\0" as *const u8 as *const ::core::ffi::c_char) as uint16_t,
        b"_NET_CLIENT_LIST\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut _NET_CLIENT_LIST_STACKING_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(b"_NET_CLIENT_LIST_STACKING\0" as *const u8 as *const ::core::ffi::c_char)
            as uint16_t,
        b"_NET_CLIENT_LIST_STACKING\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut _NET_CURRENT_DESKTOP_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(b"_NET_CURRENT_DESKTOP\0" as *const u8 as *const ::core::ffi::c_char) as uint16_t,
        b"_NET_CURRENT_DESKTOP\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut _NET_NUMBER_OF_DESKTOPS_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(b"_NET_NUMBER_OF_DESKTOPS\0" as *const u8 as *const ::core::ffi::c_char) as uint16_t,
        b"_NET_NUMBER_OF_DESKTOPS\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut _NET_DESKTOP_NAMES_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(b"_NET_DESKTOP_NAMES\0" as *const u8 as *const ::core::ffi::c_char) as uint16_t,
        b"_NET_DESKTOP_NAMES\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut _NET_DESKTOP_VIEWPORT_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(b"_NET_DESKTOP_VIEWPORT\0" as *const u8 as *const ::core::ffi::c_char) as uint16_t,
        b"_NET_DESKTOP_VIEWPORT\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut _NET_ACTIVE_WINDOW_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(b"_NET_ACTIVE_WINDOW\0" as *const u8 as *const ::core::ffi::c_char) as uint16_t,
        b"_NET_ACTIVE_WINDOW\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut _NET_CLOSE_WINDOW_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(b"_NET_CLOSE_WINDOW\0" as *const u8 as *const ::core::ffi::c_char) as uint16_t,
        b"_NET_CLOSE_WINDOW\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut _NET_MOVERESIZE_WINDOW_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(b"_NET_MOVERESIZE_WINDOW\0" as *const u8 as *const ::core::ffi::c_char) as uint16_t,
        b"_NET_MOVERESIZE_WINDOW\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut _NET_WM_USER_TIME_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(b"_NET_WM_USER_TIME\0" as *const u8 as *const ::core::ffi::c_char) as uint16_t,
        b"_NET_WM_USER_TIME\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut _NET_STARTUP_ID_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(b"_NET_STARTUP_ID\0" as *const u8 as *const ::core::ffi::c_char) as uint16_t,
        b"_NET_STARTUP_ID\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut _NET_WORKAREA_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(b"_NET_WORKAREA\0" as *const u8 as *const ::core::ffi::c_char) as uint16_t,
        b"_NET_WORKAREA\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut _NET_WM_ICON_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(b"_NET_WM_ICON\0" as *const u8 as *const ::core::ffi::c_char) as uint16_t,
        b"_NET_WM_ICON\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut WM_PROTOCOLS_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(b"WM_PROTOCOLS\0" as *const u8 as *const ::core::ffi::c_char) as uint16_t,
        b"WM_PROTOCOLS\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut WM_DELETE_WINDOW_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(b"WM_DELETE_WINDOW\0" as *const u8 as *const ::core::ffi::c_char) as uint16_t,
        b"WM_DELETE_WINDOW\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut UTF8_STRING_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(b"UTF8_STRING\0" as *const u8 as *const ::core::ffi::c_char) as uint16_t,
        b"UTF8_STRING\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut WM_STATE_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(b"WM_STATE\0" as *const u8 as *const ::core::ffi::c_char) as uint16_t,
        b"WM_STATE\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut WM_CLIENT_LEADER_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(b"WM_CLIENT_LEADER\0" as *const u8 as *const ::core::ffi::c_char) as uint16_t,
        b"WM_CLIENT_LEADER\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut WM_TAKE_FOCUS_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(b"WM_TAKE_FOCUS\0" as *const u8 as *const ::core::ffi::c_char) as uint16_t,
        b"WM_TAKE_FOCUS\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut WM_WINDOW_ROLE_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(b"WM_WINDOW_ROLE\0" as *const u8 as *const ::core::ffi::c_char) as uint16_t,
        b"WM_WINDOW_ROLE\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut I3_SOCKET_PATH_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(b"I3_SOCKET_PATH\0" as *const u8 as *const ::core::ffi::c_char) as uint16_t,
        b"I3_SOCKET_PATH\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut I3_CONFIG_PATH_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(b"I3_CONFIG_PATH\0" as *const u8 as *const ::core::ffi::c_char) as uint16_t,
        b"I3_CONFIG_PATH\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut I3_SYNC_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(b"I3_SYNC\0" as *const u8 as *const ::core::ffi::c_char) as uint16_t,
        b"I3_SYNC\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut I3_SHMLOG_PATH_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(b"I3_SHMLOG_PATH\0" as *const u8 as *const ::core::ffi::c_char) as uint16_t,
        b"I3_SHMLOG_PATH\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut I3_PID_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(b"I3_PID\0" as *const u8 as *const ::core::ffi::c_char) as uint16_t,
        b"I3_PID\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut I3_LOG_STREAM_SOCKET_PATH_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(b"I3_LOG_STREAM_SOCKET_PATH\0" as *const u8 as *const ::core::ffi::c_char)
            as uint16_t,
        b"I3_LOG_STREAM_SOCKET_PATH\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut I3_FLOATING_WINDOW_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(b"I3_FLOATING_WINDOW\0" as *const u8 as *const ::core::ffi::c_char) as uint16_t,
        b"I3_FLOATING_WINDOW\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut _NET_REQUEST_FRAME_EXTENTS_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(b"_NET_REQUEST_FRAME_EXTENTS\0" as *const u8 as *const ::core::ffi::c_char)
            as uint16_t,
        b"_NET_REQUEST_FRAME_EXTENTS\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut _NET_FRAME_EXTENTS_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(b"_NET_FRAME_EXTENTS\0" as *const u8 as *const ::core::ffi::c_char) as uint16_t,
        b"_NET_FRAME_EXTENTS\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut _MOTIF_WM_HINTS_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(b"_MOTIF_WM_HINTS\0" as *const u8 as *const ::core::ffi::c_char) as uint16_t,
        b"_MOTIF_WM_HINTS\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut WM_CHANGE_STATE_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(b"WM_CHANGE_STATE\0" as *const u8 as *const ::core::ffi::c_char) as uint16_t,
        b"WM_CHANGE_STATE\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut MANAGER_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(b"MANAGER\0" as *const u8 as *const ::core::ffi::c_char) as uint16_t,
        b"MANAGER\0" as *const u8 as *const ::core::ffi::c_char,
    );
    root_depth = (*root_screen).root_depth;
    colormap = (*root_screen).default_colormap;
    visual_type = xcb_aux_find_visual_by_attrs(
        root_screen,
        -(1 as ::core::ffi::c_int) as int8_t,
        32 as int8_t,
    );
    if !visual_type.is_null() {
        root_depth = xcb_aux_get_depth_of_visual(root_screen, (*visual_type).visual_id);
        colormap = xcb_generate_id(conn) as xcb_colormap_t;
        let mut cm_cookie: xcb_void_cookie_t = xcb_create_colormap_checked(
            conn,
            XCB_COLORMAP_ALLOC_NONE as ::core::ffi::c_int as uint8_t,
            colormap,
            root,
            (*visual_type).visual_id,
        );
        let mut error: *mut xcb_generic_error_t = xcb_request_check(conn, cm_cookie);
        if !error.is_null() {
            errorlog(
                b"ERROR: Could not create colormap. Error code: %d\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                (*error).error_code as ::core::ffi::c_int,
            );
            exit(EXIT_FAILURE);
        }
    } else {
        visual_type = get_visualtype(root_screen);
    }
    xcb_prefetch_maximum_request_length(conn);
    init_dpi();
    debuglog(
        b"%s:%s:%d - root_depth = %d, visual_id = 0x%08x.\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/main.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"main\0" as *const u8 as *const ::core::ffi::c_char,
        648 as ::core::ffi::c_int,
        root_depth as ::core::ffi::c_int,
        (*visual_type).visual_id,
    );
    debuglog(
        b"%s:%s:%d - root_screen->height_in_pixels = %d, root_screen->height_in_millimeters = %d\n\0"
            as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/main.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"main\0" as *const u8 as *const ::core::ffi::c_char,
        650 as ::core::ffi::c_int,
        (*root_screen).height_in_pixels as ::core::ffi::c_int,
        (*root_screen).height_in_millimeters as ::core::ffi::c_int,
    );
    debuglog(
        b"%s:%s:%d - One logical pixel corresponds to %d physical pixels on this display.\n\0"
            as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/main.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"main\0" as *const u8 as *const ::core::ffi::c_char,
        651 as ::core::ffi::c_int,
        logical_px(1 as ::core::ffi::c_int),
    );
    let mut gcookie: xcb_get_geometry_cookie_t = xcb_get_geometry(conn, root as xcb_drawable_t);
    let mut pointercookie: xcb_query_pointer_cookie_t = xcb_query_pointer(conn, root);
    xcb_flush(conn);
    let mut event: *mut xcb_generic_event_t = ::core::ptr::null_mut::<xcb_generic_event_t>();
    debuglog(
        b"%s:%s:%d - waiting for PropertyNotify event\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/main.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"main\0" as *const u8 as *const ::core::ffi::c_char,
        660 as ::core::ffi::c_int,
    );
    loop {
        event = xcb_wait_for_event(conn);
        if event.is_null() {
            break;
        }
        if (*event).response_type as ::core::ffi::c_int == XCB_PROPERTY_NOTIFY {
            last_timestamp = (*(event as *mut xcb_property_notify_event_t)).time;
            free(event as *mut ::core::ffi::c_void);
            break;
        } else {
            free(event as *mut ::core::ffi::c_void);
        }
    }
    debuglog(
        b"%s:%s:%d - got timestamp %d\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        b"../src/main.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"main\0" as *const u8 as *const ::core::ffi::c_char,
        669 as ::core::ffi::c_int,
        last_timestamp,
    );
    let mut reply_0: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_SUPPORTED_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_0.is_null() {
        errorlog(
            b"ERROR: Could not get atom _NET_SUPPORTED\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_SUPPORTED = (*reply_0).atom;
    free(reply_0 as *mut ::core::ffi::c_void);
    let mut reply_1: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_SUPPORTING_WM_CHECK_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_1.is_null() {
        errorlog(
            b"ERROR: Could not get atom _NET_SUPPORTING_WM_CHECK\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_SUPPORTING_WM_CHECK = (*reply_1).atom;
    free(reply_1 as *mut ::core::ffi::c_void);
    let mut reply_2: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_WM_NAME_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_2.is_null() {
        errorlog(
            b"ERROR: Could not get atom _NET_WM_NAME\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_WM_NAME = (*reply_2).atom;
    free(reply_2 as *mut ::core::ffi::c_void);
    let mut reply_3: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_WM_VISIBLE_NAME_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_3.is_null() {
        errorlog(
            b"ERROR: Could not get atom _NET_WM_VISIBLE_NAME\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_WM_VISIBLE_NAME = (*reply_3).atom;
    free(reply_3 as *mut ::core::ffi::c_void);
    let mut reply_4: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_WM_MOVERESIZE_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_4.is_null() {
        errorlog(
            b"ERROR: Could not get atom _NET_WM_MOVERESIZE\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_WM_MOVERESIZE = (*reply_4).atom;
    free(reply_4 as *mut ::core::ffi::c_void);
    let mut reply_5: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_WM_STATE_STICKY_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_5.is_null() {
        errorlog(
            b"ERROR: Could not get atom _NET_WM_STATE_STICKY\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_WM_STATE_STICKY = (*reply_5).atom;
    free(reply_5 as *mut ::core::ffi::c_void);
    let mut reply_6: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_WM_STATE_FULLSCREEN_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_6.is_null() {
        errorlog(
            b"ERROR: Could not get atom _NET_WM_STATE_FULLSCREEN\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_WM_STATE_FULLSCREEN = (*reply_6).atom;
    free(reply_6 as *mut ::core::ffi::c_void);
    let mut reply_7: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_WM_STATE_DEMANDS_ATTENTION_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_7.is_null() {
        errorlog(
            b"ERROR: Could not get atom _NET_WM_STATE_DEMANDS_ATTENTION\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_WM_STATE_DEMANDS_ATTENTION = (*reply_7).atom;
    free(reply_7 as *mut ::core::ffi::c_void);
    let mut reply_8: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_WM_STATE_MODAL_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_8.is_null() {
        errorlog(
            b"ERROR: Could not get atom _NET_WM_STATE_MODAL\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_WM_STATE_MODAL = (*reply_8).atom;
    free(reply_8 as *mut ::core::ffi::c_void);
    let mut reply_9: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_WM_STATE_HIDDEN_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_9.is_null() {
        errorlog(
            b"ERROR: Could not get atom _NET_WM_STATE_HIDDEN\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_WM_STATE_HIDDEN = (*reply_9).atom;
    free(reply_9 as *mut ::core::ffi::c_void);
    let mut reply_10: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_WM_STATE_FOCUSED_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_10.is_null() {
        errorlog(
            b"ERROR: Could not get atom _NET_WM_STATE_FOCUSED\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_WM_STATE_FOCUSED = (*reply_10).atom;
    free(reply_10 as *mut ::core::ffi::c_void);
    let mut reply_11: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_WM_STATE_MAXIMIZED_VERT_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_11.is_null() {
        errorlog(
            b"ERROR: Could not get atom _NET_WM_STATE_MAXIMIZED_VERT\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_WM_STATE_MAXIMIZED_VERT = (*reply_11).atom;
    free(reply_11 as *mut ::core::ffi::c_void);
    let mut reply_12: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_WM_STATE_MAXIMIZED_HORZ_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_12.is_null() {
        errorlog(
            b"ERROR: Could not get atom _NET_WM_STATE_MAXIMIZED_HORZ\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_WM_STATE_MAXIMIZED_HORZ = (*reply_12).atom;
    free(reply_12 as *mut ::core::ffi::c_void);
    let mut reply_13: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_WM_STATE_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_13.is_null() {
        errorlog(
            b"ERROR: Could not get atom _NET_WM_STATE\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_WM_STATE = (*reply_13).atom;
    free(reply_13 as *mut ::core::ffi::c_void);
    let mut reply_14: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_WM_WINDOW_TYPE_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_14.is_null() {
        errorlog(
            b"ERROR: Could not get atom _NET_WM_WINDOW_TYPE\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_WM_WINDOW_TYPE = (*reply_14).atom;
    free(reply_14 as *mut ::core::ffi::c_void);
    let mut reply_15: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_WM_WINDOW_TYPE_NORMAL_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_15.is_null() {
        errorlog(
            b"ERROR: Could not get atom _NET_WM_WINDOW_TYPE_NORMAL\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_WM_WINDOW_TYPE_NORMAL = (*reply_15).atom;
    free(reply_15 as *mut ::core::ffi::c_void);
    let mut reply_16: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_WM_WINDOW_TYPE_DOCK_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_16.is_null() {
        errorlog(
            b"ERROR: Could not get atom _NET_WM_WINDOW_TYPE_DOCK\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_WM_WINDOW_TYPE_DOCK = (*reply_16).atom;
    free(reply_16 as *mut ::core::ffi::c_void);
    let mut reply_17: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_WM_WINDOW_TYPE_DIALOG_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_17.is_null() {
        errorlog(
            b"ERROR: Could not get atom _NET_WM_WINDOW_TYPE_DIALOG\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_WM_WINDOW_TYPE_DIALOG = (*reply_17).atom;
    free(reply_17 as *mut ::core::ffi::c_void);
    let mut reply_18: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_WM_WINDOW_TYPE_UTILITY_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_18.is_null() {
        errorlog(
            b"ERROR: Could not get atom _NET_WM_WINDOW_TYPE_UTILITY\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_WM_WINDOW_TYPE_UTILITY = (*reply_18).atom;
    free(reply_18 as *mut ::core::ffi::c_void);
    let mut reply_19: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_WM_WINDOW_TYPE_TOOLBAR_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_19.is_null() {
        errorlog(
            b"ERROR: Could not get atom _NET_WM_WINDOW_TYPE_TOOLBAR\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_WM_WINDOW_TYPE_TOOLBAR = (*reply_19).atom;
    free(reply_19 as *mut ::core::ffi::c_void);
    let mut reply_20: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_WM_WINDOW_TYPE_SPLASH_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_20.is_null() {
        errorlog(
            b"ERROR: Could not get atom _NET_WM_WINDOW_TYPE_SPLASH\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_WM_WINDOW_TYPE_SPLASH = (*reply_20).atom;
    free(reply_20 as *mut ::core::ffi::c_void);
    let mut reply_21: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_WM_WINDOW_TYPE_MENU_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_21.is_null() {
        errorlog(
            b"ERROR: Could not get atom _NET_WM_WINDOW_TYPE_MENU\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_WM_WINDOW_TYPE_MENU = (*reply_21).atom;
    free(reply_21 as *mut ::core::ffi::c_void);
    let mut reply_22: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_WM_WINDOW_TYPE_DROPDOWN_MENU_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_22.is_null() {
        errorlog(
            b"ERROR: Could not get atom _NET_WM_WINDOW_TYPE_DROPDOWN_MENU\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_WM_WINDOW_TYPE_DROPDOWN_MENU = (*reply_22).atom;
    free(reply_22 as *mut ::core::ffi::c_void);
    let mut reply_23: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_WM_WINDOW_TYPE_POPUP_MENU_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_23.is_null() {
        errorlog(
            b"ERROR: Could not get atom _NET_WM_WINDOW_TYPE_POPUP_MENU\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_WM_WINDOW_TYPE_POPUP_MENU = (*reply_23).atom;
    free(reply_23 as *mut ::core::ffi::c_void);
    let mut reply_24: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_WM_WINDOW_TYPE_TOOLTIP_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_24.is_null() {
        errorlog(
            b"ERROR: Could not get atom _NET_WM_WINDOW_TYPE_TOOLTIP\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_WM_WINDOW_TYPE_TOOLTIP = (*reply_24).atom;
    free(reply_24 as *mut ::core::ffi::c_void);
    let mut reply_25: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_WM_WINDOW_TYPE_NOTIFICATION_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_25.is_null() {
        errorlog(
            b"ERROR: Could not get atom _NET_WM_WINDOW_TYPE_NOTIFICATION\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_WM_WINDOW_TYPE_NOTIFICATION = (*reply_25).atom;
    free(reply_25 as *mut ::core::ffi::c_void);
    let mut reply_26: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_WM_DESKTOP_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_26.is_null() {
        errorlog(
            b"ERROR: Could not get atom _NET_WM_DESKTOP\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_WM_DESKTOP = (*reply_26).atom;
    free(reply_26 as *mut ::core::ffi::c_void);
    let mut reply_27: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_WM_STRUT_PARTIAL_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_27.is_null() {
        errorlog(
            b"ERROR: Could not get atom _NET_WM_STRUT_PARTIAL\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_WM_STRUT_PARTIAL = (*reply_27).atom;
    free(reply_27 as *mut ::core::ffi::c_void);
    let mut reply_28: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_CLIENT_LIST_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_28.is_null() {
        errorlog(
            b"ERROR: Could not get atom _NET_CLIENT_LIST\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_CLIENT_LIST = (*reply_28).atom;
    free(reply_28 as *mut ::core::ffi::c_void);
    let mut reply_29: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_CLIENT_LIST_STACKING_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_29.is_null() {
        errorlog(
            b"ERROR: Could not get atom _NET_CLIENT_LIST_STACKING\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_CLIENT_LIST_STACKING = (*reply_29).atom;
    free(reply_29 as *mut ::core::ffi::c_void);
    let mut reply_30: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_CURRENT_DESKTOP_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_30.is_null() {
        errorlog(
            b"ERROR: Could not get atom _NET_CURRENT_DESKTOP\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_CURRENT_DESKTOP = (*reply_30).atom;
    free(reply_30 as *mut ::core::ffi::c_void);
    let mut reply_31: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_NUMBER_OF_DESKTOPS_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_31.is_null() {
        errorlog(
            b"ERROR: Could not get atom _NET_NUMBER_OF_DESKTOPS\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_NUMBER_OF_DESKTOPS = (*reply_31).atom;
    free(reply_31 as *mut ::core::ffi::c_void);
    let mut reply_32: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_DESKTOP_NAMES_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_32.is_null() {
        errorlog(
            b"ERROR: Could not get atom _NET_DESKTOP_NAMES\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_DESKTOP_NAMES = (*reply_32).atom;
    free(reply_32 as *mut ::core::ffi::c_void);
    let mut reply_33: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_DESKTOP_VIEWPORT_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_33.is_null() {
        errorlog(
            b"ERROR: Could not get atom _NET_DESKTOP_VIEWPORT\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_DESKTOP_VIEWPORT = (*reply_33).atom;
    free(reply_33 as *mut ::core::ffi::c_void);
    let mut reply_34: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_ACTIVE_WINDOW_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_34.is_null() {
        errorlog(
            b"ERROR: Could not get atom _NET_ACTIVE_WINDOW\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_ACTIVE_WINDOW = (*reply_34).atom;
    free(reply_34 as *mut ::core::ffi::c_void);
    let mut reply_35: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_CLOSE_WINDOW_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_35.is_null() {
        errorlog(
            b"ERROR: Could not get atom _NET_CLOSE_WINDOW\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_CLOSE_WINDOW = (*reply_35).atom;
    free(reply_35 as *mut ::core::ffi::c_void);
    let mut reply_36: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_MOVERESIZE_WINDOW_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_36.is_null() {
        errorlog(
            b"ERROR: Could not get atom _NET_MOVERESIZE_WINDOW\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_MOVERESIZE_WINDOW = (*reply_36).atom;
    free(reply_36 as *mut ::core::ffi::c_void);
    let mut reply_37: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_WM_USER_TIME_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_37.is_null() {
        errorlog(
            b"ERROR: Could not get atom _NET_WM_USER_TIME\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_WM_USER_TIME = (*reply_37).atom;
    free(reply_37 as *mut ::core::ffi::c_void);
    let mut reply_38: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_STARTUP_ID_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_38.is_null() {
        errorlog(
            b"ERROR: Could not get atom _NET_STARTUP_ID\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_STARTUP_ID = (*reply_38).atom;
    free(reply_38 as *mut ::core::ffi::c_void);
    let mut reply_39: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_WORKAREA_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_39.is_null() {
        errorlog(
            b"ERROR: Could not get atom _NET_WORKAREA\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_WORKAREA = (*reply_39).atom;
    free(reply_39 as *mut ::core::ffi::c_void);
    let mut reply_40: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_WM_ICON_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_40.is_null() {
        errorlog(
            b"ERROR: Could not get atom _NET_WM_ICON\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_WM_ICON = (*reply_40).atom;
    free(reply_40 as *mut ::core::ffi::c_void);
    let mut reply_41: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        WM_PROTOCOLS_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_41.is_null() {
        errorlog(
            b"ERROR: Could not get atom WM_PROTOCOLS\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A_WM_PROTOCOLS = (*reply_41).atom;
    free(reply_41 as *mut ::core::ffi::c_void);
    let mut reply_42: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        WM_DELETE_WINDOW_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_42.is_null() {
        errorlog(
            b"ERROR: Could not get atom WM_DELETE_WINDOW\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A_WM_DELETE_WINDOW = (*reply_42).atom;
    free(reply_42 as *mut ::core::ffi::c_void);
    let mut reply_43: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        UTF8_STRING_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_43.is_null() {
        errorlog(
            b"ERROR: Could not get atom UTF8_STRING\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A_UTF8_STRING = (*reply_43).atom;
    free(reply_43 as *mut ::core::ffi::c_void);
    let mut reply_44: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        WM_STATE_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_44.is_null() {
        errorlog(
            b"ERROR: Could not get atom WM_STATE\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A_WM_STATE = (*reply_44).atom;
    free(reply_44 as *mut ::core::ffi::c_void);
    let mut reply_45: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        WM_CLIENT_LEADER_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_45.is_null() {
        errorlog(
            b"ERROR: Could not get atom WM_CLIENT_LEADER\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A_WM_CLIENT_LEADER = (*reply_45).atom;
    free(reply_45 as *mut ::core::ffi::c_void);
    let mut reply_46: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        WM_TAKE_FOCUS_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_46.is_null() {
        errorlog(
            b"ERROR: Could not get atom WM_TAKE_FOCUS\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A_WM_TAKE_FOCUS = (*reply_46).atom;
    free(reply_46 as *mut ::core::ffi::c_void);
    let mut reply_47: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        WM_WINDOW_ROLE_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_47.is_null() {
        errorlog(
            b"ERROR: Could not get atom WM_WINDOW_ROLE\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A_WM_WINDOW_ROLE = (*reply_47).atom;
    free(reply_47 as *mut ::core::ffi::c_void);
    let mut reply_48: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        I3_SOCKET_PATH_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_48.is_null() {
        errorlog(
            b"ERROR: Could not get atom I3_SOCKET_PATH\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A_I3_SOCKET_PATH = (*reply_48).atom;
    free(reply_48 as *mut ::core::ffi::c_void);
    let mut reply_49: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        I3_CONFIG_PATH_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_49.is_null() {
        errorlog(
            b"ERROR: Could not get atom I3_CONFIG_PATH\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A_I3_CONFIG_PATH = (*reply_49).atom;
    free(reply_49 as *mut ::core::ffi::c_void);
    let mut reply_50: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        I3_SYNC_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_50.is_null() {
        errorlog(
            b"ERROR: Could not get atom I3_SYNC\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A_I3_SYNC = (*reply_50).atom;
    free(reply_50 as *mut ::core::ffi::c_void);
    let mut reply_51: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        I3_SHMLOG_PATH_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_51.is_null() {
        errorlog(
            b"ERROR: Could not get atom I3_SHMLOG_PATH\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A_I3_SHMLOG_PATH = (*reply_51).atom;
    free(reply_51 as *mut ::core::ffi::c_void);
    let mut reply_52: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        I3_PID_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_52.is_null() {
        errorlog(
            b"ERROR: Could not get atom I3_PID\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A_I3_PID = (*reply_52).atom;
    free(reply_52 as *mut ::core::ffi::c_void);
    let mut reply_53: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        I3_LOG_STREAM_SOCKET_PATH_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_53.is_null() {
        errorlog(
            b"ERROR: Could not get atom I3_LOG_STREAM_SOCKET_PATH\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A_I3_LOG_STREAM_SOCKET_PATH = (*reply_53).atom;
    free(reply_53 as *mut ::core::ffi::c_void);
    let mut reply_54: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        I3_FLOATING_WINDOW_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_54.is_null() {
        errorlog(
            b"ERROR: Could not get atom I3_FLOATING_WINDOW\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A_I3_FLOATING_WINDOW = (*reply_54).atom;
    free(reply_54 as *mut ::core::ffi::c_void);
    let mut reply_55: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_REQUEST_FRAME_EXTENTS_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_55.is_null() {
        errorlog(
            b"ERROR: Could not get atom _NET_REQUEST_FRAME_EXTENTS\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_REQUEST_FRAME_EXTENTS = (*reply_55).atom;
    free(reply_55 as *mut ::core::ffi::c_void);
    let mut reply_56: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_FRAME_EXTENTS_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_56.is_null() {
        errorlog(
            b"ERROR: Could not get atom _NET_FRAME_EXTENTS\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__NET_FRAME_EXTENTS = (*reply_56).atom;
    free(reply_56 as *mut ::core::ffi::c_void);
    let mut reply_57: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _MOTIF_WM_HINTS_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_57.is_null() {
        errorlog(
            b"ERROR: Could not get atom _MOTIF_WM_HINTS\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A__MOTIF_WM_HINTS = (*reply_57).atom;
    free(reply_57 as *mut ::core::ffi::c_void);
    let mut reply_58: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        WM_CHANGE_STATE_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_58.is_null() {
        errorlog(
            b"ERROR: Could not get atom WM_CHANGE_STATE\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A_WM_CHANGE_STATE = (*reply_58).atom;
    free(reply_58 as *mut ::core::ffi::c_void);
    let mut reply_59: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        MANAGER_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_59.is_null() {
        errorlog(
            b"ERROR: Could not get atom MANAGER\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    A_MANAGER = (*reply_59).atom;
    free(reply_59 as *mut ::core::ffi::c_void);
    load_configuration(override_configpath, C_LOAD);
    if config.ipc_socket_path.is_null() {
        config.ipc_socket_path = getenv(b"I3SOCK\0" as *const u8 as *const ::core::ffi::c_char);
        if config.ipc_socket_path.is_null() {
            config.ipc_socket_path =
                get_process_filename(b"ipc-socket\0" as *const u8 as *const ::core::ffi::c_char);
        } else {
            config.ipc_socket_path = sstrdup(config.ipc_socket_path);
        }
    }
    let mut ipc_socket: ::core::ffi::c_int =
        create_socket(config.ipc_socket_path, &raw mut current_socketpath);
    if ipc_socket == -(1 as ::core::ffi::c_int) {
        errx(
            EXIT_FAILURE,
            b"Could not create the IPC socket: %s\0" as *const u8 as *const ::core::ffi::c_char,
            config.ipc_socket_path,
        );
    }
    if config.force_xinerama {
        force_xinerama = true_0 != 0;
    }
    let mut atom_name: *mut ::core::ffi::c_char = xcb_atom_name_by_screen(
        b"WM\0" as *const u8 as *const ::core::ffi::c_char,
        conn_screen as uint8_t,
    );
    wm_sn_selection_owner = xcb_generate_id(conn) as xcb_window_t;
    if atom_name.is_null() {
        errorlog(
            b"ERROR: xcb_atom_name_by_screen(\"WM\", %d) failed, exiting\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            conn_screen,
        );
        return 1 as ::core::ffi::c_int;
    }
    let mut atom_reply: *mut xcb_intern_atom_reply_t =
        ::core::ptr::null_mut::<xcb_intern_atom_reply_t>();
    atom_reply = xcb_intern_atom_reply(
        conn,
        xcb_intern_atom_unchecked(conn, 0 as uint8_t, strlen(atom_name) as uint16_t, atom_name),
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    free(atom_name as *mut ::core::ffi::c_void);
    if atom_reply.is_null() {
        errorlog(
            b"ERROR: Failed to intern the WM_Sn atom, exiting\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        return 1 as ::core::ffi::c_int;
    }
    wm_sn = (*atom_reply).atom;
    free(atom_reply as *mut ::core::ffi::c_void);
    let mut selection_reply: *mut xcb_get_selection_owner_reply_t = xcb_get_selection_owner_reply(
        conn,
        xcb_get_selection_owner(conn, wm_sn),
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if !selection_reply.is_null()
        && (*selection_reply).owner as ::core::ffi::c_long != XCB_NONE
        && !replace_wm
    {
        errorlog(
            b"ERROR: Another window manager is already running (WM_Sn is owned)\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        return 1 as ::core::ffi::c_int;
    }
    xcb_create_window(
        conn,
        (*root_screen).root_depth,
        wm_sn_selection_owner,
        (*root_screen).root,
        -(1 as ::core::ffi::c_int) as int16_t,
        -(1 as ::core::ffi::c_int) as int16_t,
        1 as uint16_t,
        1 as uint16_t,
        0 as uint16_t,
        XCB_WINDOW_CLASS_INPUT_OUTPUT as ::core::ffi::c_int as uint16_t,
        (*root_screen).root_visual,
        0 as uint32_t,
        ::core::ptr::null::<::core::ffi::c_void>(),
    );
    xcb_change_property(
        conn,
        XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as uint8_t,
        wm_sn_selection_owner,
        XCB_ATOM_WM_CLASS as ::core::ffi::c_int as xcb_atom_t,
        XCB_ATOM_STRING as ::core::ffi::c_int as xcb_atom_t,
        8 as uint8_t,
        strlen(b"i3-WM_Sn\0" as *const u8 as *const ::core::ffi::c_char)
            .wrapping_add(1 as size_t)
            .wrapping_mul(2 as size_t) as uint32_t,
        b"i3-WM_Sn\0i3-WM_Sn\0\0" as *const u8 as *const ::core::ffi::c_char
            as *const ::core::ffi::c_void,
    );
    xcb_set_selection_owner(conn, wm_sn_selection_owner, wm_sn, last_timestamp);
    if !selection_reply.is_null() && (*selection_reply).owner as ::core::ffi::c_long != XCB_NONE {
        let mut check_rounds: ::core::ffi::c_int = 150 as ::core::ffi::c_int;
        let mut geom_reply: *mut xcb_get_geometry_reply_t =
            ::core::ptr::null_mut::<xcb_get_geometry_reply_t>();
        debuglog(
            b"%s:%s:%d - waiting for old WM_Sn selection owner to exit\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/main.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"main\0" as *const u8 as *const ::core::ffi::c_char,
            768 as ::core::ffi::c_int,
        );
        loop {
            let usleep_time: ::core::ffi::c_uint =
                100000 as ::core::ffi::c_int as ::core::ffi::c_uint;
            free(geom_reply as *mut ::core::ffi::c_void);
            usleep(usleep_time as __useconds_t);
            let fresh7 = check_rounds;
            check_rounds = check_rounds - 1;
            if fresh7 == 0 as ::core::ffi::c_int {
                errorlog(
                    b"ERROR: The old window manager is not exiting\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
                return 1 as ::core::ffi::c_int;
            }
            geom_reply = xcb_get_geometry_reply(
                conn,
                xcb_get_geometry(conn, (*selection_reply).owner as xcb_drawable_t),
                ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
            );
            if geom_reply.is_null() {
                break;
            }
        }
    }
    free(selection_reply as *mut ::core::ffi::c_void);
    let mut event_0: C2RustUnnamed_49 = C2RustUnnamed_49 {
        message: xcb_client_message_event_t {
            response_type: 0 as uint8_t,
            format: 0,
            sequence: 0,
            window: 0,
            type_0: 0,
            data: xcb_client_message_data_t { data8: [0; 20] },
        },
    };
    event_0.message.response_type = XCB_CLIENT_MESSAGE as uint8_t;
    event_0.message.window = (*root_screen).root;
    event_0.message.format = 32 as uint8_t;
    event_0.message.type_0 = A_MANAGER;
    event_0.message.data.data32[0 as ::core::ffi::c_int as usize] = last_timestamp as uint32_t;
    event_0.message.data.data32[1 as ::core::ffi::c_int as usize] = wm_sn as uint32_t;
    event_0.message.data.data32[2 as ::core::ffi::c_int as usize] =
        wm_sn_selection_owner as uint32_t;
    xcb_send_event(
        conn,
        0 as uint8_t,
        (*root_screen).root,
        XCB_EVENT_MASK_STRUCTURE_NOTIFY as ::core::ffi::c_int as uint32_t,
        &raw mut event_0.storage as *mut ::core::ffi::c_char,
    );
    let mut cookie: xcb_void_cookie_t = xcb_void_cookie_t { sequence: 0 };
    let mut fresh8: [uint32_t; 1] = [(XCB_EVENT_MASK_SUBSTRUCTURE_REDIRECT as ::core::ffi::c_int
        | XCB_EVENT_MASK_BUTTON_PRESS as ::core::ffi::c_int
        | XCB_EVENT_MASK_STRUCTURE_NOTIFY as ::core::ffi::c_int
        | XCB_EVENT_MASK_POINTER_MOTION as ::core::ffi::c_int
        | XCB_EVENT_MASK_PROPERTY_CHANGE as ::core::ffi::c_int
        | XCB_EVENT_MASK_FOCUS_CHANGE as ::core::ffi::c_int
        | XCB_EVENT_MASK_ENTER_WINDOW as ::core::ffi::c_int)
        as uint32_t];
    cookie = xcb_change_window_attributes_checked(
        conn,
        root,
        XCB_CW_EVENT_MASK as ::core::ffi::c_int as uint32_t,
        &raw mut fresh8 as *mut uint32_t as *const ::core::ffi::c_void,
    );
    let mut error_0: *mut xcb_generic_error_t = xcb_request_check(conn, cookie);
    if !error_0.is_null() {
        errorlog(
            b"ERROR: Another window manager seems to be running (X error %d)\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            (*error_0).error_code as ::core::ffi::c_int,
        );
        return 1 as ::core::ffi::c_int;
    }
    let mut greply: *mut xcb_get_geometry_reply_t = xcb_get_geometry_reply(
        conn,
        gcookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if greply.is_null() {
        errorlog(
            b"ERROR: Could not get geometry of the root window, exiting\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        return 1 as ::core::ffi::c_int;
    }
    debuglog(
        b"%s:%s:%d - root geometry reply: (%d, %d) %d x %d\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"../src/main.c\0" as *const u8 as *const ::core::ffi::c_char,
        b"main\0" as *const u8 as *const ::core::ffi::c_char,
        819 as ::core::ffi::c_int,
        (*greply).x as ::core::ffi::c_int,
        (*greply).y as ::core::ffi::c_int,
        (*greply).width as ::core::ffi::c_int,
        (*greply).height as ::core::ffi::c_int,
    );
    xcursor_load_cursors();
    xcursor_set_root_cursor(XCURSOR_CURSOR_POINTER as ::core::ffi::c_int);
    let mut extreply: *const xcb_query_extension_reply_t =
        ::core::ptr::null::<xcb_query_extension_reply_t>();
    extreply =
        xcb_get_extension_data(conn, &raw mut xcb_xkb_id) as *const xcb_query_extension_reply_t;
    xkb_supported = (*extreply).present != 0;
    if (*extreply).present == 0 {
        debuglog(
            b"%s:%s:%d - xkb is not present on this server\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/main.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"main\0" as *const u8 as *const ::core::ffi::c_char,
            831 as ::core::ffi::c_int,
        );
    } else {
        debuglog(
            b"%s:%s:%d - initializing xcb-xkb\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b"../src/main.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"main\0" as *const u8 as *const ::core::ffi::c_char,
            833 as ::core::ffi::c_int,
        );
        xcb_xkb_use_extension(
            conn,
            XCB_XKB_MAJOR_VERSION as uint16_t,
            XCB_XKB_MINOR_VERSION as uint16_t,
        );
        xcb_xkb_select_events(
            conn,
            XCB_XKB_ID_USE_CORE_KBD as ::core::ffi::c_int as xcb_xkb_device_spec_t,
            (XCB_XKB_EVENT_TYPE_STATE_NOTIFY as ::core::ffi::c_int
                | XCB_XKB_EVENT_TYPE_MAP_NOTIFY as ::core::ffi::c_int
                | XCB_XKB_EVENT_TYPE_NEW_KEYBOARD_NOTIFY as ::core::ffi::c_int)
                as uint16_t,
            0 as uint16_t,
            (XCB_XKB_EVENT_TYPE_STATE_NOTIFY as ::core::ffi::c_int
                | XCB_XKB_EVENT_TYPE_MAP_NOTIFY as ::core::ffi::c_int
                | XCB_XKB_EVENT_TYPE_NEW_KEYBOARD_NOTIFY as ::core::ffi::c_int)
                as uint16_t,
            0xff as uint16_t,
            0xff as uint16_t,
            ::core::ptr::null::<::core::ffi::c_void>(),
        );
        let mask: uint32_t = (XCB_XKB_PER_CLIENT_FLAG_GRABS_USE_XKB_STATE as ::core::ffi::c_int
            | XCB_XKB_PER_CLIENT_FLAG_LOOKUP_STATE_WHEN_GRABBED as ::core::ffi::c_int
            | XCB_XKB_PER_CLIENT_FLAG_DETECTABLE_AUTO_REPEAT as ::core::ffi::c_int)
            as uint32_t;
        let mut pcf_reply: *mut xcb_xkb_per_client_flags_reply_t =
            ::core::ptr::null_mut::<xcb_xkb_per_client_flags_reply_t>();
        pcf_reply = xcb_xkb_per_client_flags_reply(
            conn,
            xcb_xkb_per_client_flags(
                conn,
                XCB_XKB_ID_USE_CORE_KBD as ::core::ffi::c_int as xcb_xkb_device_spec_t,
                mask,
                mask,
                0 as uint32_t,
                0 as uint32_t,
                0 as uint32_t,
            ),
            ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
        );
        if pcf_reply.is_null()
            || (*pcf_reply).value
                & XCB_XKB_PER_CLIENT_FLAG_GRABS_USE_XKB_STATE as ::core::ffi::c_int as uint32_t
                == 0
        {
            errorlog(
                b"ERROR: Could not set XCB_XKB_PER_CLIENT_FLAG_GRABS_USE_XKB_STATE\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        }
        if pcf_reply.is_null()
            || (*pcf_reply).value
                & XCB_XKB_PER_CLIENT_FLAG_LOOKUP_STATE_WHEN_GRABBED as ::core::ffi::c_int
                    as uint32_t
                == 0
        {
            errorlog(
                b"ERROR: Could not set XCB_XKB_PER_CLIENT_FLAG_LOOKUP_STATE_WHEN_GRABBED\n\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
        }
        if pcf_reply.is_null()
            || (*pcf_reply).value
                & XCB_XKB_PER_CLIENT_FLAG_DETECTABLE_AUTO_REPEAT as ::core::ffi::c_int as uint32_t
                == 0
        {
            errorlog(
                b"ERROR: Could not set XCB_XKB_PER_CLIENT_FLAG_DETECTABLE_AUTO_REPEAT\n\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
        }
        free(pcf_reply as *mut ::core::ffi::c_void);
        xkb_base = (*extreply).first_event as ::core::ffi::c_int;
    }
    extreply =
        xcb_get_extension_data(conn, &raw mut xcb_shape_id) as *const xcb_query_extension_reply_t;
    if (*extreply).present != 0 {
        shape_base = (*extreply).first_event as ::core::ffi::c_int;
        let mut cookie_0: xcb_shape_query_version_cookie_t = xcb_shape_query_version(conn);
        let mut version: *mut xcb_shape_query_version_reply_t = xcb_shape_query_version_reply(
            conn,
            cookie_0,
            ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
        );
        shape_supported = !version.is_null()
            && (*version).minor_version as ::core::ffi::c_int >= 1 as ::core::ffi::c_int;
        free(version as *mut ::core::ffi::c_void);
    } else {
        shape_supported = false_0 != 0;
    }
    if !shape_supported {
        debuglog(
            b"%s:%s:%d - shape 1.1 is not present on this server\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/main.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"main\0" as *const u8 as *const ::core::ffi::c_char,
            903 as ::core::ffi::c_int,
        );
    }
    restore_connect();
    property_handlers_init();
    ewmh_setup_hints();
    keysyms = xcb_key_symbols_alloc(conn);
    xcb_numlock_mask =
        aio_get_mod_mask_for(XCB_NUM_LOCK as uint32_t, keysyms) as ::core::ffi::c_uint;
    if !load_keymap() {
        errx(
            EXIT_FAILURE,
            b"Could not load keymap\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    translate_keysyms();
    grab_all_keys(conn);
    let mut needs_tree_init: bool = true_0 != 0;
    if !layout_path.is_null() {
        verboselog(
            b"Trying to restore the layout from \"%s\".\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            layout_path,
        );
        needs_tree_init = !tree_restore(layout_path, greply);
        if delete_layout_path {
            unlink(layout_path);
            let mut dir: *const ::core::ffi::c_char = dirname(layout_path);
            rmdir(dir);
        }
    }
    if needs_tree_init {
        tree_init(greply);
    }
    free(greply as *mut ::core::ffi::c_void);
    if fake_outputs.is_null() && !config.fake_outputs.is_null() {
        fake_outputs = config.fake_outputs;
    }
    if !fake_outputs.is_null() {
        fake_outputs_init(fake_outputs);
        free(fake_outputs as *mut ::core::ffi::c_void);
        fake_outputs = ::core::ptr::null_mut::<::core::ffi::c_char>();
        config.fake_outputs = ::core::ptr::null_mut::<::core::ffi::c_char>();
    } else if force_xinerama {
        xinerama_init();
    } else {
        debuglog(
            b"%s:%s:%d - Checking for XRandR...\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b"../src/main.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"main\0" as *const u8 as *const ::core::ffi::c_char,
            956 as ::core::ffi::c_int,
        );
        randr_init(
            &raw mut randr_base,
            disable_randr15 as ::core::ffi::c_int != 0
                || config.disable_randr15 as ::core::ffi::c_int != 0,
        );
    }
    if !layout_path.is_null() && randr_base > -(1 as ::core::ffi::c_int) {
        let mut con: *mut Con = ::core::ptr::null_mut::<Con>();
        con = (*croot).nodes_head.tqh_first as *mut Con;
        while !con.is_null() {
            let mut output: *mut Output = ::core::ptr::null_mut::<Output>();
            output = outputs.tqh_first as *mut Output;
            while !output.is_null() {
                if !((*output).active as ::core::ffi::c_int != 0
                    || strcmp((*con).name, output_primary_name(output)) != 0 as ::core::ffi::c_int)
                {
                    if (*output).con.is_null() {
                        output_init_con(output);
                        (*output).changed = false_0 != 0;
                    }
                    (*output).to_be_disabled = true_0 != 0;
                    randr_disable_output(output);
                }
                output = (*output).outputs.tqe_next as *mut Output;
            }
            con = (*con).nodes.tqe_next as *mut Con;
        }
    }
    free(layout_path as *mut ::core::ffi::c_void);
    layout_path = ::core::ptr::null_mut::<::core::ffi::c_char>();
    scratchpad_fix_resolution();
    let mut output_0: *mut Output = ::core::ptr::null_mut::<Output>();
    let mut pointer_reply: *mut xcb_query_pointer_reply_t = xcb_query_pointer_reply(
        conn,
        pointercookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if pointer_reply.is_null() {
        errorlog(
            b"ERROR: Could not query pointer position, using first screen\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    } else {
        debuglog(
            b"%s:%s:%d - Pointer at %d, %d\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b"../src/main.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"main\0" as *const u8 as *const ::core::ffi::c_char,
            995 as ::core::ffi::c_int,
            (*pointer_reply).root_x as ::core::ffi::c_int,
            (*pointer_reply).root_y as ::core::ffi::c_int,
        );
        output_0 = get_output_containing(
            (*pointer_reply).root_x as ::core::ffi::c_uint,
            (*pointer_reply).root_y as ::core::ffi::c_uint,
        );
        if output_0.is_null() {
            errorlog(
                b"ERROR: ERROR: No screen at (%d, %d), starting on the first screen\n\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                (*pointer_reply).root_x as ::core::ffi::c_int,
                (*pointer_reply).root_y as ::core::ffi::c_int,
            );
        }
    }
    if output_0.is_null() {
        output_0 = get_first_output();
    }
    con_activate(con_descend_focused(output_get_content((*output_0).con)));
    free(pointer_reply as *mut ::core::ffi::c_void);
    tree_render();
    let mut ipc_io: *mut ev_io =
        scalloc(1 as size_t, ::core::mem::size_of::<ev_io>() as size_t) as *mut ev_io;
    let ref mut fresh9 = (*(ipc_io as *mut ::core::ffi::c_void as *mut ev_watcher)).pending;
    *fresh9 = 0 as ::core::ffi::c_int;
    (*(ipc_io as *mut ::core::ffi::c_void as *mut ev_watcher)).active = *fresh9;
    (*(ipc_io as *mut ::core::ffi::c_void as *mut ev_watcher)).priority = 0 as ::core::ffi::c_int;
    (*ipc_io).cb = Some(
        ipc_new_client as unsafe extern "C" fn(*mut ev_loop, *mut ev_io, ::core::ffi::c_int) -> (),
    )
        as Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_io, ::core::ffi::c_int) -> ()>;
    memmove(
        &raw mut (*(ipc_io as *mut ev_watcher)).cb as *mut ::core::ffi::c_void,
        &raw mut (*ipc_io).cb as *const ::core::ffi::c_void,
        ::core::mem::size_of::<
            Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_io, ::core::ffi::c_int) -> ()>,
        >() as size_t,
    );
    (*ipc_io).fd = ipc_socket;
    (*ipc_io).events = EV_READ as ::core::ffi::c_int | EV__IOFDSET as ::core::ffi::c_int;
    ev_io_start(main_loop, ipc_io);
    let mut log_stream_socket_path: *mut ::core::ffi::c_char =
        get_process_filename(b"log-stream-socket\0" as *const u8 as *const ::core::ffi::c_char);
    let mut log_socket: ::core::ffi::c_int = create_socket(
        log_stream_socket_path,
        &raw mut current_log_stream_socket_path,
    );
    free(log_stream_socket_path as *mut ::core::ffi::c_void);
    let mut log_io: *mut ev_io = ::core::ptr::null_mut::<ev_io>();
    if log_socket == -(1 as ::core::ffi::c_int) {
        errorlog(
            b"ERROR: Could not create the log socket, i3-dump-log -f will not work\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    } else {
        log_io = scalloc(1 as size_t, ::core::mem::size_of::<ev_io>() as size_t) as *mut ev_io;
        let ref mut fresh10 = (*(log_io as *mut ::core::ffi::c_void as *mut ev_watcher)).pending;
        *fresh10 = 0 as ::core::ffi::c_int;
        (*(log_io as *mut ::core::ffi::c_void as *mut ev_watcher)).active = *fresh10;
        (*(log_io as *mut ::core::ffi::c_void as *mut ev_watcher)).priority =
            0 as ::core::ffi::c_int;
        (*log_io).cb = Some(
            log_new_client
                as unsafe extern "C" fn(*mut ev_loop, *mut ev_io, ::core::ffi::c_int) -> (),
        )
            as Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_io, ::core::ffi::c_int) -> ()>;
        memmove(
            &raw mut (*(log_io as *mut ev_watcher)).cb as *mut ::core::ffi::c_void,
            &raw mut (*log_io).cb as *const ::core::ffi::c_void,
            ::core::mem::size_of::<
                Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_io, ::core::ffi::c_int) -> ()>,
            >() as size_t,
        );
        (*log_io).fd = log_socket;
        (*log_io).events = EV_READ as ::core::ffi::c_int | EV__IOFDSET as ::core::ffi::c_int;
        ev_io_start(main_loop, log_io);
    }
    let mut socket_ipc_io: *mut ev_io = ::core::ptr::null_mut::<ev_io>();
    listen_fds = sd_listen_fds(0 as ::core::ffi::c_int);
    if listen_fds < 0 as ::core::ffi::c_int {
        errorlog(
            b"ERROR: socket activation: Error in sd_listen_fds\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    } else if listen_fds == 0 as ::core::ffi::c_int {
        debuglog(
            b"%s:%s:%d - socket activation: no sockets passed\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../src/main.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"main\0" as *const u8 as *const ::core::ffi::c_char,
            1036 as ::core::ffi::c_int,
        );
    } else {
        let mut flags: ::core::ffi::c_int = 0;
        let mut fd: ::core::ffi::c_int = SD_LISTEN_FDS_START;
        while fd < SD_LISTEN_FDS_START + listen_fds {
            debuglog(
                b"%s:%s:%d - socket activation: also listening on fd %d\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                b"../src/main.c\0" as *const u8 as *const ::core::ffi::c_char,
                b"main\0" as *const u8 as *const ::core::ffi::c_char,
                1042 as ::core::ffi::c_int,
                fd,
            );
            flags = fcntl(fd, F_GETFD);
            if flags < 0 as ::core::ffi::c_int
                || fcntl(fd, F_SETFD, flags & !FD_CLOEXEC) < 0 as ::core::ffi::c_int
            {
                errorlog(
                    b"ERROR: Could not disable FD_CLOEXEC on fd %d\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    fd,
                );
            }
            socket_ipc_io =
                scalloc(1 as size_t, ::core::mem::size_of::<ev_io>() as size_t) as *mut ev_io;
            let ref mut fresh11 =
                (*(socket_ipc_io as *mut ::core::ffi::c_void as *mut ev_watcher)).pending;
            *fresh11 = 0 as ::core::ffi::c_int;
            (*(socket_ipc_io as *mut ::core::ffi::c_void as *mut ev_watcher)).active = *fresh11;
            (*(socket_ipc_io as *mut ::core::ffi::c_void as *mut ev_watcher)).priority =
                0 as ::core::ffi::c_int;
            (*socket_ipc_io).cb = Some(
                ipc_new_client
                    as unsafe extern "C" fn(*mut ev_loop, *mut ev_io, ::core::ffi::c_int) -> (),
            )
                as Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_io, ::core::ffi::c_int) -> ()>;
            memmove(
                &raw mut (*(socket_ipc_io as *mut ev_watcher)).cb as *mut ::core::ffi::c_void,
                &raw mut (*socket_ipc_io).cb as *const ::core::ffi::c_void,
                ::core::mem::size_of::<
                    Option<
                        unsafe extern "C" fn(*mut ev_loop, *mut ev_io, ::core::ffi::c_int) -> (),
                    >,
                >() as size_t,
            );
            (*socket_ipc_io).fd = fd;
            (*socket_ipc_io).events =
                EV_READ as ::core::ffi::c_int | EV__IOFDSET as ::core::ffi::c_int;
            ev_io_start(main_loop, socket_ipc_io);
            fd += 1;
        }
    }
    let restart_fd: ::core::ffi::c_int = parse_restart_fd() as ::core::ffi::c_int;
    if restart_fd != -(1 as ::core::ffi::c_int) {
        debuglog(
            b"%s:%s:%d - serving restart fd %d\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b"../src/main.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"main\0" as *const u8 as *const ::core::ffi::c_char,
            1061 as ::core::ffi::c_int,
            restart_fd,
        );
        let mut client: *mut ipc_client = ipc_new_client_on_fd(main_loop, restart_fd);
        ipc_confirm_restart(client);
        unsetenv(b"_I3_RESTART_FD\0" as *const u8 as *const ::core::ffi::c_char);
    }
    x_set_i3_atoms();
    ewmh_update_workarea();
    ewmh_update_desktop_properties();
    let mut xcb_watcher: *mut ev_io =
        scalloc(1 as size_t, ::core::mem::size_of::<ev_io>() as size_t) as *mut ev_io;
    xcb_prepare =
        scalloc(1 as size_t, ::core::mem::size_of::<ev_prepare>() as size_t) as *mut ev_prepare;
    let ref mut fresh12 = (*(xcb_watcher as *mut ::core::ffi::c_void as *mut ev_watcher)).pending;
    *fresh12 = 0 as ::core::ffi::c_int;
    (*(xcb_watcher as *mut ::core::ffi::c_void as *mut ev_watcher)).active = *fresh12;
    (*(xcb_watcher as *mut ::core::ffi::c_void as *mut ev_watcher)).priority =
        0 as ::core::ffi::c_int;
    (*xcb_watcher).cb = Some(
        xcb_got_event as unsafe extern "C" fn(*mut ev_loop, *mut ev_io, ::core::ffi::c_int) -> (),
    )
        as Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_io, ::core::ffi::c_int) -> ()>;
    memmove(
        &raw mut (*(xcb_watcher as *mut ev_watcher)).cb as *mut ::core::ffi::c_void,
        &raw mut (*xcb_watcher).cb as *const ::core::ffi::c_void,
        ::core::mem::size_of::<
            Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_io, ::core::ffi::c_int) -> ()>,
        >() as size_t,
    );
    (*xcb_watcher).fd = xcb_get_file_descriptor(conn);
    (*xcb_watcher).events = EV_READ as ::core::ffi::c_int | EV__IOFDSET as ::core::ffi::c_int;
    ev_io_start(main_loop, xcb_watcher);
    let ref mut fresh13 = (*(xcb_prepare as *mut ::core::ffi::c_void as *mut ev_watcher)).pending;
    *fresh13 = 0 as ::core::ffi::c_int;
    (*(xcb_prepare as *mut ::core::ffi::c_void as *mut ev_watcher)).active = *fresh13;
    (*(xcb_prepare as *mut ::core::ffi::c_void as *mut ev_watcher)).priority =
        0 as ::core::ffi::c_int;
    (*xcb_prepare).cb = Some(
        xcb_prepare_cb
            as unsafe extern "C" fn(*mut ev_loop, *mut ev_prepare, ::core::ffi::c_int) -> (),
    )
        as Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_prepare, ::core::ffi::c_int) -> ()>;
    memmove(
        &raw mut (*(xcb_prepare as *mut ev_watcher)).cb as *mut ::core::ffi::c_void,
        &raw mut (*xcb_prepare).cb as *const ::core::ffi::c_void,
        ::core::mem::size_of::<
            Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_prepare, ::core::ffi::c_int) -> ()>,
        >() as size_t,
    );
    ev_prepare_start(main_loop, xcb_prepare);
    xcb_flush(conn);
    xcb_grab_server(conn);
    xcb_aux_sync(conn);
    let mut event_1: *mut xcb_generic_event_t = ::core::ptr::null_mut::<xcb_generic_event_t>();
    loop {
        event_1 = xcb_poll_for_event(conn);
        if event_1.is_null() {
            break;
        }
        if (*event_1).response_type as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            free(event_1 as *mut ::core::ffi::c_void);
        } else {
            let type_0: ::core::ffi::c_int =
                (*event_1).response_type as ::core::ffi::c_int & 0x7f as ::core::ffi::c_int;
            if type_0 == XCB_MAP_REQUEST {
                handle_event(type_0, event_1);
            }
            free(event_1 as *mut ::core::ffi::c_void);
        }
    }
    manage_existing_windows(root);
    xcb_ungrab_server(conn);
    if autostart {
        verboselog(
            b"This is not an in-place restart, checking if a wallpaper is set.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        let mut root_0: *mut xcb_screen_t = xcb_aux_get_screen(conn, conn_screen);
        if is_background_set(conn, root_0) {
            verboselog(
                b"A wallpaper is set, so no screenshot is necessary.\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        } else {
            verboselog(
                b"No wallpaper set, copying root window contents to a pixmap\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
            set_screenshot_as_wallpaper(conn, root_0);
        }
    }
    if !disable_signalhandler {
        setup_signal_handler();
    } else {
        let mut action: sigaction = sigaction {
            __sigaction_handler: C2RustUnnamed_9 { sa_handler: None },
            sa_mask: __sigset_t { __val: [0; 16] },
            sa_flags: 0,
            sa_restorer: None,
        };
        action.__sigaction_handler.sa_sigaction = Some(
            handle_core_signal
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut siginfo_t,
                    *mut ::core::ffi::c_void,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut siginfo_t,
                    *mut ::core::ffi::c_void,
                ) -> (),
            >;
        action.sa_flags = (SA_NODEFER as ::core::ffi::c_uint
            | SA_RESETHAND
            | SA_SIGINFO as ::core::ffi::c_uint) as ::core::ffi::c_int;
        sigemptyset(&raw mut action.sa_mask);
        if sigaction(
            SIGQUIT,
            &raw mut action,
            ::core::ptr::null_mut::<sigaction>(),
        ) == -(1 as ::core::ffi::c_int)
            || sigaction(
                SIGILL,
                &raw mut action,
                ::core::ptr::null_mut::<sigaction>(),
            ) == -(1 as ::core::ffi::c_int)
            || sigaction(
                SIGABRT,
                &raw mut action,
                ::core::ptr::null_mut::<sigaction>(),
            ) == -(1 as ::core::ffi::c_int)
            || sigaction(
                SIGFPE,
                &raw mut action,
                ::core::ptr::null_mut::<sigaction>(),
            ) == -(1 as ::core::ffi::c_int)
            || sigaction(
                SIGSEGV,
                &raw mut action,
                ::core::ptr::null_mut::<sigaction>(),
            ) == -(1 as ::core::ffi::c_int)
        {
            errorlog(
                b"ERROR: Could not setup signal handler.\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        }
    }
    setup_term_handlers();
    signal(
        SIGPIPE,
        ::core::mem::transmute::<::libc::intptr_t, __sighandler_t>(
            1 as ::core::ffi::c_int as ::libc::intptr_t,
        ),
    );
    if autostart {
        while !autostarts.tqh_first.is_null() {
            let mut exec: *mut Autostart = autostarts.tqh_first;
            verboselog(
                b"auto-starting %s\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                (*exec).command,
            );
            start_application(
                (*exec).command,
                (*exec).no_startup_id,
                XCB_NONE as xcb_window_t,
            );
            free((*exec).command as *mut ::core::ffi::c_void);
            (*exec).command = ::core::ptr::null_mut::<::core::ffi::c_char>();
            if !(*exec).autostarts.tqe_next.is_null() {
                (*(*exec).autostarts.tqe_next).autostarts.tqe_prev = (*exec).autostarts.tqe_prev;
            } else {
                autostarts.tqh_last = (*exec).autostarts.tqe_prev;
            }
            *(*exec).autostarts.tqe_prev = (*exec).autostarts.tqe_next;
            free(exec as *mut ::core::ffi::c_void);
            exec = ::core::ptr::null_mut::<Autostart>();
        }
    }
    while !autostarts_always.tqh_first.is_null() {
        let mut exec_always: *mut Autostart = autostarts_always.tqh_first;
        verboselog(
            b"auto-starting (always!) %s\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            (*exec_always).command,
        );
        start_application(
            (*exec_always).command,
            (*exec_always).no_startup_id,
            XCB_NONE as xcb_window_t,
        );
        free((*exec_always).command as *mut ::core::ffi::c_void);
        (*exec_always).command = ::core::ptr::null_mut::<::core::ffi::c_char>();
        if !(*exec_always).autostarts_always.tqe_next.is_null() {
            (*(*exec_always).autostarts_always.tqe_next)
                .autostarts_always
                .tqe_prev = (*exec_always).autostarts_always.tqe_prev;
        } else {
            autostarts_always.tqh_last = (*exec_always).autostarts_always.tqe_prev;
        }
        *(*exec_always).autostarts_always.tqe_prev = (*exec_always).autostarts_always.tqe_next;
        free(exec_always as *mut ::core::ffi::c_void);
        exec_always = ::core::ptr::null_mut::<Autostart>();
    }
    let mut barconfig: *mut Barconfig = ::core::ptr::null_mut::<Barconfig>();
    barconfig = barconfigs.tqh_first as *mut Barconfig;
    while !barconfig.is_null() {
        let mut command: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        sasprintf(
            &raw mut command,
            b"%s %s --bar_id=%s --socket=\"%s\"\0" as *const u8 as *const ::core::ffi::c_char,
            if !(*barconfig).i3bar_command.is_null() {
                (*barconfig).i3bar_command as *const ::core::ffi::c_char
            } else {
                b"exec i3bar\0" as *const u8 as *const ::core::ffi::c_char
            },
            if (*barconfig).verbose as ::core::ffi::c_int != 0 {
                b"-V\0" as *const u8 as *const ::core::ffi::c_char
            } else {
                b"\0" as *const u8 as *const ::core::ffi::c_char
            },
            (*barconfig).id,
            current_socketpath,
        );
        verboselog(
            b"Starting bar process: %s\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            command,
        );
        start_application(command, true_0 != 0, XCB_NONE as xcb_window_t);
        free(command as *mut ::core::ffi::c_void);
        barconfig = (*barconfig).configs.tqe_next as *mut Barconfig;
    }
    atexit(Some(i3_exit as unsafe extern "C" fn() -> ()));
    raise(SIGCHLD);
    sd_notify(
        1 as ::core::ffi::c_int,
        b"READY=1\0" as *const u8 as *const ::core::ffi::c_char,
    );
    ev_loop(main_loop, 0 as ::core::ffi::c_int);
    free(ipc_io as *mut ::core::ffi::c_void);
    ipc_io = ::core::ptr::null_mut::<ev_io>();
    free(socket_ipc_io as *mut ::core::ffi::c_void);
    socket_ipc_io = ::core::ptr::null_mut::<ev_io>();
    free(log_io as *mut ::core::ffi::c_void);
    log_io = ::core::ptr::null_mut::<ev_io>();
    free(xcb_watcher as *mut ::core::ffi::c_void);
    xcb_watcher = ::core::ptr::null_mut::<ev_io>();
    return 0;
}
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const EXIT_SUCCESS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub fn main() {
    let mut args_strings: Vec<Vec<u8>> = ::std::env::args()
        .map(|arg| {
            ::std::ffi::CString::new(arg)
                .expect("Failed to convert argument into CString.")
                .into_bytes_with_nul()
        })
        .collect();
    let mut args_ptrs: Vec<*mut ::core::ffi::c_char> = args_strings
        .iter_mut()
        .map(|arg| arg.as_mut_ptr() as *mut ::core::ffi::c_char)
        .chain(::core::iter::once(::core::ptr::null_mut()))
        .collect();
    unsafe {
        ::std::process::exit(main_0(
            (args_ptrs.len() - 1) as ::core::ffi::c_int,
            args_ptrs.as_mut_ptr() as *mut *mut ::core::ffi::c_char,
        ) as i32)
    }
}
unsafe extern "C" fn run_static_initializers() {
    autostarts = autostarts_head {
        tqh_first: ::core::ptr::null_mut::<Autostart>(),
        tqh_last: &raw mut autostarts.tqh_first,
    };
    autostarts_always = autostarts_always_head {
        tqh_first: ::core::ptr::null_mut::<Autostart>(),
        tqh_last: &raw mut autostarts_always.tqh_first,
    };
    ws_assignments = ws_assignments_head {
        tqh_first: ::core::ptr::null_mut::<Workspace_Assignment>(),
        tqh_last: &raw mut ws_assignments.tqh_first,
    };
    assignments = assignments_head {
        tqh_first: ::core::ptr::null_mut::<Assignment>(),
        tqh_last: &raw mut assignments.tqh_first,
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
