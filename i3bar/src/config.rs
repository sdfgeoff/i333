/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3bar - an xcb-based status- and ws-bar for i3
 * © 2010 Axel Wagner and contributors (see also: LICENSE)
 *
 * config.c: Parses the configuration (received from i3).
 *
 */
// Mechanical file-by-file Rust translation of i3bar/src/config.c.
// Generated with C2Rust 0.22.1; C ABI and original ownership retained.
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type _i3String;
    pub type yajl_handle_t;
    static mut stderr: *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn scalloc(num: size_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn srealloc(ptr: *mut ::core::ffi::c_void, size: size_t) -> *mut ::core::ffi::c_void;
    fn sasprintf(
        strp: *mut *mut ::core::ffi::c_char,
        fmt: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn i3string_from_utf8_with_length(
        from_utf8: *const ::core::ffi::c_char,
        num_bytes: ssize_t,
    ) -> *mut i3String;
    fn i3string_free(str: *mut i3String);
    fn yajl_alloc(
        callbacks: *const yajl_callbacks,
        afs: *mut yajl_alloc_funcs,
        ctx: *mut ::core::ffi::c_void,
    ) -> yajl_handle;
    fn yajl_free(handle: yajl_handle);
    fn yajl_parse(
        hand: yajl_handle,
        jsonText: *const ::core::ffi::c_uchar,
        jsonTextLength: size_t,
    ) -> yajl_status;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn exit(__status: ::core::ffi::c_int) -> !;
}
pub type size_t = usize;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type ssize_t = isize;
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
pub type uint32_t = __uint32_t;
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
pub type i3String = _i3String;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_color_strings_t {
    pub bar_fg: *mut ::core::ffi::c_char,
    pub bar_bg: *mut ::core::ffi::c_char,
    pub sep_fg: *mut ::core::ffi::c_char,
    pub focus_bar_fg: *mut ::core::ffi::c_char,
    pub focus_bar_bg: *mut ::core::ffi::c_char,
    pub focus_sep_fg: *mut ::core::ffi::c_char,
    pub active_ws_fg: *mut ::core::ffi::c_char,
    pub active_ws_bg: *mut ::core::ffi::c_char,
    pub active_ws_border: *mut ::core::ffi::c_char,
    pub inactive_ws_fg: *mut ::core::ffi::c_char,
    pub inactive_ws_bg: *mut ::core::ffi::c_char,
    pub inactive_ws_border: *mut ::core::ffi::c_char,
    pub focus_ws_bg: *mut ::core::ffi::c_char,
    pub focus_ws_fg: *mut ::core::ffi::c_char,
    pub focus_ws_border: *mut ::core::ffi::c_char,
    pub urgent_ws_bg: *mut ::core::ffi::c_char,
    pub urgent_ws_fg: *mut ::core::ffi::c_char,
    pub urgent_ws_border: *mut ::core::ffi::c_char,
    pub binding_mode_bg: *mut ::core::ffi::c_char,
    pub binding_mode_fg: *mut ::core::ffi::c_char,
    pub binding_mode_border: *mut ::core::ffi::c_char,
}
pub type position_t = ::core::ffi::c_uint;
pub const POS_BOT: position_t = 2;
pub const POS_TOP: position_t = 1;
pub const POS_NONE: position_t = 0;
pub type bar_display_mode_t = ::core::ffi::c_uint;
pub const M_INVISIBLE: bar_display_mode_t = 2;
pub const M_HIDE: bar_display_mode_t = 1;
pub const M_DOCK: bar_display_mode_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct binding_t {
    pub input_code: ::core::ffi::c_int,
    pub command: *mut ::core::ffi::c_char,
    pub release: bool,
    pub bindings: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub tqe_next: *mut binding_t,
    pub tqe_prev: *mut *mut binding_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tray_output_t {
    pub output: *mut ::core::ffi::c_char,
    pub tray_outputs: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub tqe_next: *mut tray_output_t,
    pub tqe_prev: *mut *mut tray_output_t,
}
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
pub struct config_t {
    pub modifier: uint32_t,
    pub bindings: bindings_head,
    pub position: position_t,
    pub verbose: bool,
    pub bar_height: uint32_t,
    pub padding: Rect,
    pub transparency: bool,
    pub colors: xcb_color_strings_t,
    pub disable_binding_mode_indicator: bool,
    pub disable_ws: bool,
    pub ws_min_width: ::core::ffi::c_int,
    pub strip_ws_numbers: bool,
    pub strip_ws_name: bool,
    pub bar_id: *mut ::core::ffi::c_char,
    pub command: *mut ::core::ffi::c_char,
    pub workspace_command: *mut ::core::ffi::c_char,
    pub fontname: *mut ::core::ffi::c_char,
    pub separator_symbol: *mut i3String,
    pub tray_outputs: tray_outputs_head,
    pub tray_padding: ::core::ffi::c_int,
    pub num_outputs: ::core::ffi::c_int,
    pub outputs: *mut *mut ::core::ffi::c_char,
    pub hide_on_modifier: bar_display_mode_t,
    pub hidden_state: C2RustUnnamed_1,
}
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const S_SHOW: C2RustUnnamed_1 = 1;
pub const S_HIDE: C2RustUnnamed_1 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tray_outputs_head {
    pub tqh_first: *mut tray_output_t,
    pub tqh_last: *mut *mut tray_output_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bindings_head {
    pub tqh_first: *mut binding_t,
    pub tqh_last: *mut *mut binding_t,
}
pub type yajl_handle = *mut yajl_handle_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yajl_alloc_funcs {
    pub malloc: yajl_malloc_func,
    pub realloc: yajl_realloc_func,
    pub free: yajl_free_func,
    pub ctx: *mut ::core::ffi::c_void,
}
pub type yajl_free_func =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> ()>;
pub type yajl_realloc_func = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
        size_t,
    ) -> *mut ::core::ffi::c_void,
>;
pub type yajl_malloc_func =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> *mut ::core::ffi::c_void>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yajl_callbacks {
    pub yajl_null: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
    pub yajl_boolean: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub yajl_integer: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_longlong,
        ) -> ::core::ffi::c_int,
    >,
    pub yajl_double: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, ::core::ffi::c_double) -> ::core::ffi::c_int,
    >,
    pub yajl_number: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_char,
            size_t,
        ) -> ::core::ffi::c_int,
    >,
    pub yajl_string: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_uchar,
            size_t,
        ) -> ::core::ffi::c_int,
    >,
    pub yajl_start_map:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
    pub yajl_map_key: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_uchar,
            size_t,
        ) -> ::core::ffi::c_int,
    >,
    pub yajl_end_map: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
    pub yajl_start_array:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
    pub yajl_end_array:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
}
pub const yajl_status_error: yajl_status = 2;
pub const yajl_status_client_canceled: yajl_status = 1;
pub const yajl_status_ok: yajl_status = 0;
pub type yajl_status = ::core::ffi::c_uint;
pub const XCB_NONE: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[no_mangle]
pub static mut config: config_t = config_t {
    modifier: 0,
    bindings: bindings_head {
        tqh_first: ::core::ptr::null::<binding_t>() as *mut binding_t,
        tqh_last: ::core::ptr::null::<*mut binding_t>() as *mut *mut binding_t,
    },
    position: POS_NONE,
    verbose: false,
    bar_height: 0,
    padding: Rect {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    },
    transparency: false,
    colors: xcb_color_strings_t {
        bar_fg: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        bar_bg: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        sep_fg: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        focus_bar_fg: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        focus_bar_bg: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        focus_sep_fg: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        active_ws_fg: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        active_ws_bg: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        active_ws_border: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        inactive_ws_fg: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        inactive_ws_bg: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        inactive_ws_border: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        focus_ws_bg: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        focus_ws_fg: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        focus_ws_border: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        urgent_ws_bg: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        urgent_ws_fg: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        urgent_ws_border: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        binding_mode_bg: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        binding_mode_fg: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        binding_mode_border: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    },
    disable_binding_mode_indicator: false,
    disable_ws: false,
    ws_min_width: 0,
    strip_ws_numbers: false,
    strip_ws_name: false,
    bar_id: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    command: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    workspace_command: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    fontname: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    separator_symbol: ::core::ptr::null::<i3String>() as *mut i3String,
    tray_outputs: tray_outputs_head {
        tqh_first: ::core::ptr::null::<tray_output_t>() as *mut tray_output_t,
        tqh_last: ::core::ptr::null::<*mut tray_output_t>() as *mut *mut tray_output_t,
    },
    tray_padding: 0,
    num_outputs: 0,
    outputs: ::core::ptr::null::<*mut ::core::ffi::c_char>() as *mut *mut ::core::ffi::c_char,
    hide_on_modifier: M_DOCK,
    hidden_state: S_HIDE,
};
static mut cur_key: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
static mut parsing_bindings: bool = false;
static mut parsing_tray_outputs: bool = false;
static mut parsing_padding: bool = false;
unsafe extern "C" fn config_map_key_cb(
    mut params_: *mut ::core::ffi::c_void,
    mut keyVal: *const ::core::ffi::c_uchar,
    mut keyLen: size_t,
) -> ::core::ffi::c_int {
    free(cur_key as *mut ::core::ffi::c_void);
    cur_key = ::core::ptr::null_mut::<::core::ffi::c_char>();
    sasprintf(
        &raw mut cur_key,
        b"%.*s\0" as *const u8 as *const ::core::ffi::c_char,
        keyLen as ::core::ffi::c_int,
        keyVal,
    );
    if strcmp(
        cur_key,
        b"bindings\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        parsing_bindings = true_0 != 0;
    }
    if strcmp(
        cur_key,
        b"tray_outputs\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        parsing_tray_outputs = true_0 != 0;
    }
    if strcmp(
        cur_key,
        b"padding\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        parsing_padding = true_0 != 0;
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn config_end_array_cb(
    mut params_: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    parsing_bindings = false_0 != 0;
    parsing_tray_outputs = false_0 != 0;
    parsing_padding = false_0 != 0;
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn config_null_cb(mut params_: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    if strcmp(cur_key, b"id\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        fprintf(
            stderr,
            b"[%s:%d] ERROR: No such bar config. Use 'i3-msg -t get_bar_config' to get the available configs.\n\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"../i3bar/src/config.c\0" as *const u8 as *const ::core::ffi::c_char,
            63 as ::core::ffi::c_int,
        );
        fprintf(
            stderr,
            b"[%s:%d] ERROR: Are you starting i3bar by hand? You should not:\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"../i3bar/src/config.c\0" as *const u8 as *const ::core::ffi::c_char,
            64 as ::core::ffi::c_int,
        );
        fprintf(
            stderr,
            b"[%s:%d] ERROR: Configure a 'bar' block in your i3 config and i3 will launch i3bar automatically.\n\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"../i3bar/src/config.c\0" as *const u8 as *const ::core::ffi::c_char,
            65 as ::core::ffi::c_int,
        );
        exit(EXIT_FAILURE);
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn config_string_cb(
    mut params_: *mut ::core::ffi::c_void,
    mut val: *const ::core::ffi::c_uchar,
    mut _len: size_t,
) -> ::core::ffi::c_int {
    let mut len: ::core::ffi::c_int = _len as ::core::ffi::c_int;
    if strcmp(cur_key, b"id\0" as *const u8 as *const ::core::ffi::c_char) == 0
        || strcmp(
            cur_key,
            b"socket_path\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0
    {
        return 1 as ::core::ffi::c_int;
    }
    if parsing_bindings {
        if strcmp(
            cur_key,
            b"command\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            let mut binding: *mut binding_t =
                *(*(config.bindings.tqh_last as *mut bindings_head)).tqh_last;
            if binding.is_null() {
                fprintf(
                    stderr,
                    b"[%s:%d] ERROR: There is no binding to put the current command onto. This is a bug in i3.\n\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    b"../i3bar/src/config.c\0" as *const u8
                        as *const ::core::ffi::c_char,
                    87 as ::core::ffi::c_int,
                );
                return 0 as ::core::ffi::c_int;
            }
            if !(*binding).command.is_null() {
                fprintf(
                    stderr,
                    b"[%s:%d] ERROR: The binding for input_code = %d already has a command. This is a bug in i3.\n\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    b"../i3bar/src/config.c\0" as *const u8
                        as *const ::core::ffi::c_char,
                    92 as ::core::ffi::c_int,
                    (*binding).input_code,
                );
                return 0 as ::core::ffi::c_int;
            }
            sasprintf(
                &raw mut (*binding).command,
                b"%.*s\0" as *const u8 as *const ::core::ffi::c_char,
                len,
                val,
            );
            return 1 as ::core::ffi::c_int;
        }
        fprintf(
            stderr,
            b"[%s:%d] ERROR: Unknown key \"%s\" while parsing bar bindings.\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"../i3bar/src/config.c\0" as *const u8 as *const ::core::ffi::c_char,
            100 as ::core::ffi::c_int,
            cur_key,
        );
        return 0 as ::core::ffi::c_int;
    }
    if parsing_tray_outputs {
        if config.verbose {
            printf(
                b"[%s:%d] Adding tray_output = %.*s to the list.\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"../i3bar/src/config.c\0" as *const u8 as *const ::core::ffi::c_char,
                105 as ::core::ffi::c_int,
                len,
                val,
            );
        }
        let mut tray_output: *mut tray_output_t = scalloc(
            1 as size_t,
            ::core::mem::size_of::<tray_output_t>() as size_t,
        ) as *mut tray_output_t;
        sasprintf(
            &raw mut (*tray_output).output,
            b"%.*s\0" as *const u8 as *const ::core::ffi::c_char,
            len,
            val,
        );
        (*tray_output).tray_outputs.tqe_next = ::core::ptr::null_mut::<tray_output_t>();
        (*tray_output).tray_outputs.tqe_prev = config.tray_outputs.tqh_last;
        *config.tray_outputs.tqh_last = tray_output as *mut tray_output_t;
        config.tray_outputs.tqh_last = &raw mut (*tray_output).tray_outputs.tqe_next;
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(
        cur_key,
        b"mode\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        if config.verbose {
            printf(
                b"[%s:%d] mode = %.*s, len = %d\n\0" as *const u8 as *const ::core::ffi::c_char,
                b"../i3bar/src/config.c\0" as *const u8 as *const ::core::ffi::c_char,
                113 as ::core::ffi::c_int,
                len,
                val,
                len,
            );
        }
        config.hide_on_modifier = (if len as size_t
            == strlen(b"dock\0" as *const u8 as *const ::core::ffi::c_char)
            && strncmp(
                val as *const ::core::ffi::c_char,
                b"dock\0" as *const u8 as *const ::core::ffi::c_char,
                strlen(b"dock\0" as *const u8 as *const ::core::ffi::c_char),
            ) == 0
        {
            M_DOCK as ::core::ffi::c_int
        } else if len as size_t == strlen(b"hide\0" as *const u8 as *const ::core::ffi::c_char)
            && strncmp(
                val as *const ::core::ffi::c_char,
                b"hide\0" as *const u8 as *const ::core::ffi::c_char,
                strlen(b"hide\0" as *const u8 as *const ::core::ffi::c_char),
            ) == 0
        {
            M_HIDE as ::core::ffi::c_int
        } else {
            M_INVISIBLE as ::core::ffi::c_int
        }) as bar_display_mode_t;
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(
        cur_key,
        b"hidden_state\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        if config.verbose {
            printf(
                b"[%s:%d] hidden_state = %.*s, len = %d\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"../i3bar/src/config.c\0" as *const u8 as *const ::core::ffi::c_char,
                121 as ::core::ffi::c_int,
                len,
                val,
                len,
            );
        }
        config.hidden_state = (if len as size_t
            == strlen(b"hide\0" as *const u8 as *const ::core::ffi::c_char)
            && strncmp(
                val as *const ::core::ffi::c_char,
                b"hide\0" as *const u8 as *const ::core::ffi::c_char,
                strlen(b"hide\0" as *const u8 as *const ::core::ffi::c_char),
            ) == 0
        {
            S_HIDE as ::core::ffi::c_int
        } else {
            S_SHOW as ::core::ffi::c_int
        }) as C2RustUnnamed_1;
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(
        cur_key,
        b"modifier\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        if config.verbose {
            printf(
                b"[%s:%d] modifier = %.*s\n\0" as *const u8 as *const ::core::ffi::c_char,
                b"../i3bar/src/config.c\0" as *const u8 as *const ::core::ffi::c_char,
                128 as ::core::ffi::c_int,
                len,
                val,
            );
        }
        if len as size_t == strlen(b"none\0" as *const u8 as *const ::core::ffi::c_char)
            && strncmp(
                val as *const ::core::ffi::c_char,
                b"none\0" as *const u8 as *const ::core::ffi::c_char,
                strlen(b"none\0" as *const u8 as *const ::core::ffi::c_char),
            ) == 0
        {
            config.modifier = XCB_NONE as uint32_t;
            return 1 as ::core::ffi::c_int;
        }
        if len as size_t == strlen(b"shift\0" as *const u8 as *const ::core::ffi::c_char)
            && strncmp(
                val as *const ::core::ffi::c_char,
                b"shift\0" as *const u8 as *const ::core::ffi::c_char,
                strlen(b"shift\0" as *const u8 as *const ::core::ffi::c_char),
            ) == 0
        {
            config.modifier = XCB_MOD_MASK_SHIFT as ::core::ffi::c_int as uint32_t;
            return 1 as ::core::ffi::c_int;
        }
        if len as size_t == strlen(b"ctrl\0" as *const u8 as *const ::core::ffi::c_char)
            && strncmp(
                val as *const ::core::ffi::c_char,
                b"ctrl\0" as *const u8 as *const ::core::ffi::c_char,
                strlen(b"ctrl\0" as *const u8 as *const ::core::ffi::c_char),
            ) == 0
        {
            config.modifier = XCB_MOD_MASK_CONTROL as ::core::ffi::c_int as uint32_t;
            return 1 as ::core::ffi::c_int;
        }
        if len as size_t
            == strlen(b"Mod\0" as *const u8 as *const ::core::ffi::c_char).wrapping_add(1 as size_t)
            && strncmp(
                val as *const ::core::ffi::c_char,
                b"Mod\0" as *const u8 as *const ::core::ffi::c_char,
                strlen(b"Mod\0" as *const u8 as *const ::core::ffi::c_char),
            ) == 0
        {
            match *val.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int {
                49 => {
                    config.modifier = XCB_MOD_MASK_1 as ::core::ffi::c_int as uint32_t;
                    return 1 as ::core::ffi::c_int;
                }
                50 => {
                    config.modifier = XCB_MOD_MASK_2 as ::core::ffi::c_int as uint32_t;
                    return 1 as ::core::ffi::c_int;
                }
                51 => {
                    config.modifier = XCB_MOD_MASK_3 as ::core::ffi::c_int as uint32_t;
                    return 1 as ::core::ffi::c_int;
                }
                53 => {
                    config.modifier = XCB_MOD_MASK_5 as ::core::ffi::c_int as uint32_t;
                    return 1 as ::core::ffi::c_int;
                }
                _ => {}
            }
        }
        config.modifier = XCB_MOD_MASK_4 as ::core::ffi::c_int as uint32_t;
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(
        cur_key,
        b"wheel_up_cmd\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        if config.verbose {
            printf(
                b"[%s:%d] wheel_up_cmd = %.*s\n\0" as *const u8 as *const ::core::ffi::c_char,
                b"../i3bar/src/config.c\0" as *const u8 as *const ::core::ffi::c_char,
                166 as ::core::ffi::c_int,
                len,
                val,
            );
        }
        let mut binding_0: *mut binding_t =
            scalloc(1 as size_t, ::core::mem::size_of::<binding_t>() as size_t) as *mut binding_t;
        (*binding_0).input_code = 4 as ::core::ffi::c_int;
        sasprintf(
            &raw mut (*binding_0).command,
            b"%.*s\0" as *const u8 as *const ::core::ffi::c_char,
            len,
            val,
        );
        (*binding_0).bindings.tqe_next = ::core::ptr::null_mut::<binding_t>();
        (*binding_0).bindings.tqe_prev = config.bindings.tqh_last;
        *config.bindings.tqh_last = binding_0 as *mut binding_t;
        config.bindings.tqh_last = &raw mut (*binding_0).bindings.tqe_next;
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(
        cur_key,
        b"wheel_down_cmd\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        if config.verbose {
            printf(
                b"[%s:%d] wheel_down_cmd = %.*s\n\0" as *const u8 as *const ::core::ffi::c_char,
                b"../i3bar/src/config.c\0" as *const u8 as *const ::core::ffi::c_char,
                177 as ::core::ffi::c_int,
                len,
                val,
            );
        }
        let mut binding_1: *mut binding_t =
            scalloc(1 as size_t, ::core::mem::size_of::<binding_t>() as size_t) as *mut binding_t;
        (*binding_1).input_code = 5 as ::core::ffi::c_int;
        sasprintf(
            &raw mut (*binding_1).command,
            b"%.*s\0" as *const u8 as *const ::core::ffi::c_char,
            len,
            val,
        );
        (*binding_1).bindings.tqe_next = ::core::ptr::null_mut::<binding_t>();
        (*binding_1).bindings.tqe_prev = config.bindings.tqh_last;
        *config.bindings.tqh_last = binding_1 as *mut binding_t;
        config.bindings.tqh_last = &raw mut (*binding_1).bindings.tqe_next;
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(
        cur_key,
        b"position\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        if config.verbose {
            printf(
                b"[%s:%d] position = %.*s\n\0" as *const u8 as *const ::core::ffi::c_char,
                b"../i3bar/src/config.c\0" as *const u8 as *const ::core::ffi::c_char,
                186 as ::core::ffi::c_int,
                len,
                val,
            );
        }
        config.position = (if len as size_t
            == strlen(b"top\0" as *const u8 as *const ::core::ffi::c_char)
            && strncmp(
                val as *const ::core::ffi::c_char,
                b"top\0" as *const u8 as *const ::core::ffi::c_char,
                strlen(b"top\0" as *const u8 as *const ::core::ffi::c_char),
            ) == 0
        {
            POS_TOP as ::core::ffi::c_int
        } else {
            POS_BOT as ::core::ffi::c_int
        }) as position_t;
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(
        cur_key,
        b"status_command\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        if config.verbose {
            printf(
                b"[%s:%d] status_command = %.*s\n\0" as *const u8 as *const ::core::ffi::c_char,
                b"../i3bar/src/config.c\0" as *const u8 as *const ::core::ffi::c_char,
                192 as ::core::ffi::c_int,
                len,
                val,
            );
        }
        sasprintf(
            &raw mut config.command,
            b"%.*s\0" as *const u8 as *const ::core::ffi::c_char,
            len,
            val,
        );
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(
        cur_key,
        b"workspace_command\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        if config.verbose {
            printf(
                b"[%s:%d] workspace_command = %.*s\n\0" as *const u8 as *const ::core::ffi::c_char,
                b"../i3bar/src/config.c\0" as *const u8 as *const ::core::ffi::c_char,
                198 as ::core::ffi::c_int,
                len,
                val,
            );
        }
        sasprintf(
            &raw mut config.workspace_command,
            b"%.*s\0" as *const u8 as *const ::core::ffi::c_char,
            len,
            val,
        );
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(
        cur_key,
        b"font\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        if config.verbose {
            printf(
                b"[%s:%d] font = %.*s\n\0" as *const u8 as *const ::core::ffi::c_char,
                b"../i3bar/src/config.c\0" as *const u8 as *const ::core::ffi::c_char,
                204 as ::core::ffi::c_int,
                len,
                val,
            );
        }
        free(config.fontname as *mut ::core::ffi::c_void);
        config.fontname = ::core::ptr::null_mut::<::core::ffi::c_char>();
        sasprintf(
            &raw mut config.fontname,
            b"%.*s\0" as *const u8 as *const ::core::ffi::c_char,
            len,
            val,
        );
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(
        cur_key,
        b"separator_symbol\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        if config.verbose {
            printf(
                b"[%s:%d] separator = %.*s\n\0" as *const u8 as *const ::core::ffi::c_char,
                b"../i3bar/src/config.c\0" as *const u8 as *const ::core::ffi::c_char,
                211 as ::core::ffi::c_int,
                len,
                val,
            );
        }
        i3string_free(config.separator_symbol);
        config.separator_symbol = ::core::ptr::null_mut::<i3String>();
        config.separator_symbol =
            i3string_from_utf8_with_length(val as *const ::core::ffi::c_char, len as ssize_t);
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(
        cur_key,
        b"outputs\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        if config.verbose {
            printf(
                b"[%s:%d] +output %.*s\n\0" as *const u8 as *const ::core::ffi::c_char,
                b"../i3bar/src/config.c\0" as *const u8 as *const ::core::ffi::c_char,
                218 as ::core::ffi::c_int,
                len,
                val,
            );
        }
        let mut new_num_outputs: ::core::ffi::c_int = config.num_outputs + 1 as ::core::ffi::c_int;
        config.outputs = srealloc(
            config.outputs as *mut ::core::ffi::c_void,
            (::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t)
                .wrapping_mul(new_num_outputs as size_t),
        ) as *mut *mut ::core::ffi::c_char;
        sasprintf(
            config.outputs.offset(config.num_outputs as isize) as *mut *mut ::core::ffi::c_char,
            b"%.*s\0" as *const u8 as *const ::core::ffi::c_char,
            len,
            val,
        );
        config.num_outputs = new_num_outputs;
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(
        cur_key,
        b"tray_output\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        if config.verbose {
            printf(
                b"[%s:%d] Found deprecated key tray_output %.*s.\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"../i3bar/src/config.c\0" as *const u8 as *const ::core::ffi::c_char,
                229 as ::core::ffi::c_int,
                len,
                val,
            );
        }
        let mut tray_output_0: *mut tray_output_t = scalloc(
            1 as size_t,
            ::core::mem::size_of::<tray_output_t>() as size_t,
        ) as *mut tray_output_t;
        sasprintf(
            &raw mut (*tray_output_0).output,
            b"%.*s\0" as *const u8 as *const ::core::ffi::c_char,
            len,
            val,
        );
        (*tray_output_0).tray_outputs.tqe_next = ::core::ptr::null_mut::<tray_output_t>();
        (*tray_output_0).tray_outputs.tqe_prev = config.tray_outputs.tqh_last;
        *config.tray_outputs.tqh_last = tray_output_0 as *mut tray_output_t;
        config.tray_outputs.tqh_last = &raw mut (*tray_output_0).tray_outputs.tqe_next;
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(
        cur_key,
        b"statusline\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        if config.verbose {
            printf(
                b"[%s:%d] statusline = bar_fg = %.*s\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"../i3bar/src/config.c\0" as *const u8 as *const ::core::ffi::c_char,
                245 as ::core::ffi::c_int,
                len,
                val,
            );
        }
        sasprintf(
            &raw mut config.colors.bar_fg,
            b"%.*s\0" as *const u8 as *const ::core::ffi::c_char,
            len,
            val,
        );
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(
        cur_key,
        b"background\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        if config.verbose {
            printf(
                b"[%s:%d] background = bar_bg = %.*s\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"../i3bar/src/config.c\0" as *const u8 as *const ::core::ffi::c_char,
                246 as ::core::ffi::c_int,
                len,
                val,
            );
        }
        sasprintf(
            &raw mut config.colors.bar_bg,
            b"%.*s\0" as *const u8 as *const ::core::ffi::c_char,
            len,
            val,
        );
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(
        cur_key,
        b"separator\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        if config.verbose {
            printf(
                b"[%s:%d] separator = sep_fg = %.*s\n\0" as *const u8 as *const ::core::ffi::c_char,
                b"../i3bar/src/config.c\0" as *const u8 as *const ::core::ffi::c_char,
                247 as ::core::ffi::c_int,
                len,
                val,
            );
        }
        sasprintf(
            &raw mut config.colors.sep_fg,
            b"%.*s\0" as *const u8 as *const ::core::ffi::c_char,
            len,
            val,
        );
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(
        cur_key,
        b"focused_statusline\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        if config.verbose {
            printf(
                b"[%s:%d] focused_statusline = focus_bar_fg = %.*s\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"../i3bar/src/config.c\0" as *const u8 as *const ::core::ffi::c_char,
                248 as ::core::ffi::c_int,
                len,
                val,
            );
        }
        sasprintf(
            &raw mut config.colors.focus_bar_fg,
            b"%.*s\0" as *const u8 as *const ::core::ffi::c_char,
            len,
            val,
        );
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(
        cur_key,
        b"focused_background\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        if config.verbose {
            printf(
                b"[%s:%d] focused_background = focus_bar_bg = %.*s\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"../i3bar/src/config.c\0" as *const u8 as *const ::core::ffi::c_char,
                249 as ::core::ffi::c_int,
                len,
                val,
            );
        }
        sasprintf(
            &raw mut config.colors.focus_bar_bg,
            b"%.*s\0" as *const u8 as *const ::core::ffi::c_char,
            len,
            val,
        );
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(
        cur_key,
        b"focused_separator\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        if config.verbose {
            printf(
                b"[%s:%d] focused_separator = focus_sep_fg = %.*s\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"../i3bar/src/config.c\0" as *const u8 as *const ::core::ffi::c_char,
                250 as ::core::ffi::c_int,
                len,
                val,
            );
        }
        sasprintf(
            &raw mut config.colors.focus_sep_fg,
            b"%.*s\0" as *const u8 as *const ::core::ffi::c_char,
            len,
            val,
        );
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(
        cur_key,
        b"focused_workspace_border\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        if config.verbose {
            printf(
                b"[%s:%d] focused_workspace_border = focus_ws_border = %.*s\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"../i3bar/src/config.c\0" as *const u8 as *const ::core::ffi::c_char,
                251 as ::core::ffi::c_int,
                len,
                val,
            );
        }
        sasprintf(
            &raw mut config.colors.focus_ws_border,
            b"%.*s\0" as *const u8 as *const ::core::ffi::c_char,
            len,
            val,
        );
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(
        cur_key,
        b"focused_workspace_bg\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        if config.verbose {
            printf(
                b"[%s:%d] focused_workspace_bg = focus_ws_bg = %.*s\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"../i3bar/src/config.c\0" as *const u8 as *const ::core::ffi::c_char,
                252 as ::core::ffi::c_int,
                len,
                val,
            );
        }
        sasprintf(
            &raw mut config.colors.focus_ws_bg,
            b"%.*s\0" as *const u8 as *const ::core::ffi::c_char,
            len,
            val,
        );
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(
        cur_key,
        b"focused_workspace_text\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        if config.verbose {
            printf(
                b"[%s:%d] focused_workspace_text = focus_ws_fg = %.*s\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"../i3bar/src/config.c\0" as *const u8 as *const ::core::ffi::c_char,
                253 as ::core::ffi::c_int,
                len,
                val,
            );
        }
        sasprintf(
            &raw mut config.colors.focus_ws_fg,
            b"%.*s\0" as *const u8 as *const ::core::ffi::c_char,
            len,
            val,
        );
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(
        cur_key,
        b"active_workspace_border\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        if config.verbose {
            printf(
                b"[%s:%d] active_workspace_border = active_ws_border = %.*s\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"../i3bar/src/config.c\0" as *const u8 as *const ::core::ffi::c_char,
                254 as ::core::ffi::c_int,
                len,
                val,
            );
        }
        sasprintf(
            &raw mut config.colors.active_ws_border,
            b"%.*s\0" as *const u8 as *const ::core::ffi::c_char,
            len,
            val,
        );
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(
        cur_key,
        b"active_workspace_bg\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        if config.verbose {
            printf(
                b"[%s:%d] active_workspace_bg = active_ws_bg = %.*s\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"../i3bar/src/config.c\0" as *const u8 as *const ::core::ffi::c_char,
                255 as ::core::ffi::c_int,
                len,
                val,
            );
        }
        sasprintf(
            &raw mut config.colors.active_ws_bg,
            b"%.*s\0" as *const u8 as *const ::core::ffi::c_char,
            len,
            val,
        );
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(
        cur_key,
        b"active_workspace_text\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        if config.verbose {
            printf(
                b"[%s:%d] active_workspace_text = active_ws_fg = %.*s\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"../i3bar/src/config.c\0" as *const u8 as *const ::core::ffi::c_char,
                256 as ::core::ffi::c_int,
                len,
                val,
            );
        }
        sasprintf(
            &raw mut config.colors.active_ws_fg,
            b"%.*s\0" as *const u8 as *const ::core::ffi::c_char,
            len,
            val,
        );
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(
        cur_key,
        b"inactive_workspace_border\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        if config.verbose {
            printf(
                b"[%s:%d] inactive_workspace_border = inactive_ws_border = %.*s\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"../i3bar/src/config.c\0" as *const u8 as *const ::core::ffi::c_char,
                257 as ::core::ffi::c_int,
                len,
                val,
            );
        }
        sasprintf(
            &raw mut config.colors.inactive_ws_border,
            b"%.*s\0" as *const u8 as *const ::core::ffi::c_char,
            len,
            val,
        );
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(
        cur_key,
        b"inactive_workspace_bg\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        if config.verbose {
            printf(
                b"[%s:%d] inactive_workspace_bg = inactive_ws_bg = %.*s\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"../i3bar/src/config.c\0" as *const u8 as *const ::core::ffi::c_char,
                258 as ::core::ffi::c_int,
                len,
                val,
            );
        }
        sasprintf(
            &raw mut config.colors.inactive_ws_bg,
            b"%.*s\0" as *const u8 as *const ::core::ffi::c_char,
            len,
            val,
        );
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(
        cur_key,
        b"inactive_workspace_text\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        if config.verbose {
            printf(
                b"[%s:%d] inactive_workspace_text = inactive_ws_fg = %.*s\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"../i3bar/src/config.c\0" as *const u8 as *const ::core::ffi::c_char,
                259 as ::core::ffi::c_int,
                len,
                val,
            );
        }
        sasprintf(
            &raw mut config.colors.inactive_ws_fg,
            b"%.*s\0" as *const u8 as *const ::core::ffi::c_char,
            len,
            val,
        );
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(
        cur_key,
        b"urgent_workspace_border\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        if config.verbose {
            printf(
                b"[%s:%d] urgent_workspace_border = urgent_ws_border = %.*s\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"../i3bar/src/config.c\0" as *const u8 as *const ::core::ffi::c_char,
                260 as ::core::ffi::c_int,
                len,
                val,
            );
        }
        sasprintf(
            &raw mut config.colors.urgent_ws_border,
            b"%.*s\0" as *const u8 as *const ::core::ffi::c_char,
            len,
            val,
        );
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(
        cur_key,
        b"urgent_workspace_bg\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        if config.verbose {
            printf(
                b"[%s:%d] urgent_workspace_bg = urgent_ws_bg = %.*s\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"../i3bar/src/config.c\0" as *const u8 as *const ::core::ffi::c_char,
                261 as ::core::ffi::c_int,
                len,
                val,
            );
        }
        sasprintf(
            &raw mut config.colors.urgent_ws_bg,
            b"%.*s\0" as *const u8 as *const ::core::ffi::c_char,
            len,
            val,
        );
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(
        cur_key,
        b"urgent_workspace_text\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        if config.verbose {
            printf(
                b"[%s:%d] urgent_workspace_text = urgent_ws_fg = %.*s\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"../i3bar/src/config.c\0" as *const u8 as *const ::core::ffi::c_char,
                262 as ::core::ffi::c_int,
                len,
                val,
            );
        }
        sasprintf(
            &raw mut config.colors.urgent_ws_fg,
            b"%.*s\0" as *const u8 as *const ::core::ffi::c_char,
            len,
            val,
        );
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(
        cur_key,
        b"binding_mode_border\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        if config.verbose {
            printf(
                b"[%s:%d] binding_mode_border = binding_mode_border = %.*s\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"../i3bar/src/config.c\0" as *const u8 as *const ::core::ffi::c_char,
                263 as ::core::ffi::c_int,
                len,
                val,
            );
        }
        sasprintf(
            &raw mut config.colors.binding_mode_border,
            b"%.*s\0" as *const u8 as *const ::core::ffi::c_char,
            len,
            val,
        );
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(
        cur_key,
        b"binding_mode_bg\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        if config.verbose {
            printf(
                b"[%s:%d] binding_mode_bg = binding_mode_bg = %.*s\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"../i3bar/src/config.c\0" as *const u8 as *const ::core::ffi::c_char,
                264 as ::core::ffi::c_int,
                len,
                val,
            );
        }
        sasprintf(
            &raw mut config.colors.binding_mode_bg,
            b"%.*s\0" as *const u8 as *const ::core::ffi::c_char,
            len,
            val,
        );
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(
        cur_key,
        b"binding_mode_text\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        if config.verbose {
            printf(
                b"[%s:%d] binding_mode_text = binding_mode_fg = %.*s\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"../i3bar/src/config.c\0" as *const u8 as *const ::core::ffi::c_char,
                265 as ::core::ffi::c_int,
                len,
                val,
            );
        }
        sasprintf(
            &raw mut config.colors.binding_mode_fg,
            b"%.*s\0" as *const u8 as *const ::core::ffi::c_char,
            len,
            val,
        );
        return 1 as ::core::ffi::c_int;
    }
    printf(
        b"got unexpected string %.*s for cur_key = %s\n\0" as *const u8
            as *const ::core::ffi::c_char,
        len,
        val,
        cur_key,
    );
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn config_boolean_cb(
    mut params_: *mut ::core::ffi::c_void,
    mut val: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if parsing_bindings {
        if strcmp(
            cur_key,
            b"release\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            let mut binding: *mut binding_t =
                *(*(config.bindings.tqh_last as *mut bindings_head)).tqh_last;
            if binding.is_null() {
                fprintf(
                    stderr,
                    b"[%s:%d] ERROR: There is no binding to put the current command onto. This is a bug in i3.\n\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    b"../i3bar/src/config.c\0" as *const u8
                        as *const ::core::ffi::c_char,
                    281 as ::core::ffi::c_int,
                );
                return 0 as ::core::ffi::c_int;
            }
            (*binding).release = val != 0;
            return 1 as ::core::ffi::c_int;
        }
        fprintf(
            stderr,
            b"[%s:%d] ERROR: Unknown key \"%s\" while parsing bar bindings.\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"../i3bar/src/config.c\0" as *const u8 as *const ::core::ffi::c_char,
            289 as ::core::ffi::c_int,
            cur_key,
        );
    }
    if strcmp(
        cur_key,
        b"binding_mode_indicator\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        if config.verbose {
            printf(
                b"[%s:%d] binding_mode_indicator = %d\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"../i3bar/src/config.c\0" as *const u8 as *const ::core::ffi::c_char,
                293 as ::core::ffi::c_int,
                val,
            );
        }
        config.disable_binding_mode_indicator = val == 0;
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(
        cur_key,
        b"workspace_buttons\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        if config.verbose {
            printf(
                b"[%s:%d] workspace_buttons = %d\n\0" as *const u8 as *const ::core::ffi::c_char,
                b"../i3bar/src/config.c\0" as *const u8 as *const ::core::ffi::c_char,
                299 as ::core::ffi::c_int,
                val,
            );
        }
        config.disable_ws = val == 0;
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(
        cur_key,
        b"strip_workspace_numbers\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        if config.verbose {
            printf(
                b"[%s:%d] strip_workspace_numbers = %d\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"../i3bar/src/config.c\0" as *const u8 as *const ::core::ffi::c_char,
                305 as ::core::ffi::c_int,
                val,
            );
        }
        config.strip_ws_numbers = val != 0;
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(
        cur_key,
        b"strip_workspace_name\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        if config.verbose {
            printf(
                b"[%s:%d] strip_workspace_name = %d\n\0" as *const u8 as *const ::core::ffi::c_char,
                b"../i3bar/src/config.c\0" as *const u8 as *const ::core::ffi::c_char,
                311 as ::core::ffi::c_int,
                val,
            );
        }
        config.strip_ws_name = val != 0;
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(
        cur_key,
        b"verbose\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        if !config.verbose {
            if config.verbose {
                printf(
                    b"[%s:%d] verbose = %d\n\0" as *const u8 as *const ::core::ffi::c_char,
                    b"../i3bar/src/config.c\0" as *const u8 as *const ::core::ffi::c_char,
                    318 as ::core::ffi::c_int,
                    val,
                );
            }
            config.verbose = val != 0;
        }
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn config_integer_cb(
    mut params_: *mut ::core::ffi::c_void,
    mut val: ::core::ffi::c_longlong,
) -> ::core::ffi::c_int {
    if parsing_bindings {
        if strcmp(
            cur_key,
            b"input_code\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            let mut binding: *mut binding_t =
                scalloc(1 as size_t, ::core::mem::size_of::<binding_t>() as size_t)
                    as *mut binding_t;
            (*binding).input_code = val as ::core::ffi::c_int;
            (*binding).bindings.tqe_next = ::core::ptr::null_mut::<binding_t>();
            (*binding).bindings.tqe_prev = config.bindings.tqh_last;
            *config.bindings.tqh_last = binding as *mut binding_t;
            config.bindings.tqh_last = &raw mut (*binding).bindings.tqe_next;
            return 1 as ::core::ffi::c_int;
        }
        fprintf(
            stderr,
            b"[%s:%d] ERROR: Unknown key \"%s\" while parsing bar bindings.\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"../i3bar/src/config.c\0" as *const u8 as *const ::core::ffi::c_char,
            341 as ::core::ffi::c_int,
            cur_key,
        );
        return 0 as ::core::ffi::c_int;
    }
    if parsing_padding {
        if strcmp(cur_key, b"x\0" as *const u8 as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            if config.verbose {
                printf(
                    b"[%s:%d] padding.x = %lld\n\0" as *const u8 as *const ::core::ffi::c_char,
                    b"../i3bar/src/config.c\0" as *const u8 as *const ::core::ffi::c_char,
                    347 as ::core::ffi::c_int,
                    val,
                );
            }
            config.padding.x = val as uint32_t;
            return 1 as ::core::ffi::c_int;
        }
        if strcmp(cur_key, b"y\0" as *const u8 as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            if config.verbose {
                printf(
                    b"[%s:%d] padding.y = %lld\n\0" as *const u8 as *const ::core::ffi::c_char,
                    b"../i3bar/src/config.c\0" as *const u8 as *const ::core::ffi::c_char,
                    352 as ::core::ffi::c_int,
                    val,
                );
            }
            config.padding.y = val as uint32_t;
            return 1 as ::core::ffi::c_int;
        }
        if strcmp(
            cur_key,
            b"width\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            if config.verbose {
                printf(
                    b"[%s:%d] padding.width = %lld\n\0" as *const u8 as *const ::core::ffi::c_char,
                    b"../i3bar/src/config.c\0" as *const u8 as *const ::core::ffi::c_char,
                    357 as ::core::ffi::c_int,
                    val,
                );
            }
            config.padding.width = val as uint32_t;
            return 1 as ::core::ffi::c_int;
        }
        if strcmp(
            cur_key,
            b"height\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            if config.verbose {
                printf(
                    b"[%s:%d] padding.height = %lld\n\0" as *const u8 as *const ::core::ffi::c_char,
                    b"../i3bar/src/config.c\0" as *const u8 as *const ::core::ffi::c_char,
                    362 as ::core::ffi::c_int,
                    val,
                );
            }
            config.padding.height = val as uint32_t;
            return 1 as ::core::ffi::c_int;
        }
    }
    if strcmp(
        cur_key,
        b"bar_height\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        if config.verbose {
            printf(
                b"[%s:%d] bar_height = %lld\n\0" as *const u8 as *const ::core::ffi::c_char,
                b"../i3bar/src/config.c\0" as *const u8 as *const ::core::ffi::c_char,
                369 as ::core::ffi::c_int,
                val,
            );
        }
        config.bar_height = val as uint32_t;
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(
        cur_key,
        b"tray_padding\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        if config.verbose {
            printf(
                b"[%s:%d] tray_padding = %lld\n\0" as *const u8 as *const ::core::ffi::c_char,
                b"../i3bar/src/config.c\0" as *const u8 as *const ::core::ffi::c_char,
                375 as ::core::ffi::c_int,
                val,
            );
        }
        config.tray_padding = val as ::core::ffi::c_int;
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(
        cur_key,
        b"modifier\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        if config.verbose {
            printf(
                b"[%s:%d] modifier = %lld\n\0" as *const u8 as *const ::core::ffi::c_char,
                b"../i3bar/src/config.c\0" as *const u8 as *const ::core::ffi::c_char,
                381 as ::core::ffi::c_int,
                val,
            );
        }
        config.modifier = val as uint32_t;
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(
        cur_key,
        b"workspace_min_width\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        if config.verbose {
            printf(
                b"[%s:%d] workspace_min_width = %lld\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"../i3bar/src/config.c\0" as *const u8 as *const ::core::ffi::c_char,
                387 as ::core::ffi::c_int,
                val,
            );
        }
        config.ws_min_width = val as ::core::ffi::c_int;
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
static mut outputs_callbacks: yajl_callbacks = unsafe {
    yajl_callbacks {
        yajl_null: Some(
            config_null_cb as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
        ),
        yajl_boolean: Some(
            config_boolean_cb
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        yajl_integer: Some(
            config_integer_cb
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_longlong,
                ) -> ::core::ffi::c_int,
        ),
        yajl_double: None,
        yajl_number: None,
        yajl_string: Some(
            config_string_cb
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_uchar,
                    size_t,
                ) -> ::core::ffi::c_int,
        ),
        yajl_start_map: None,
        yajl_map_key: Some(
            config_map_key_cb
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_uchar,
                    size_t,
                ) -> ::core::ffi::c_int,
        ),
        yajl_end_map: None,
        yajl_start_array: None,
        yajl_end_array: Some(
            config_end_array_cb
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
        ),
    }
};
#[no_mangle]
pub unsafe extern "C" fn parse_config_json(
    mut json: *const ::core::ffi::c_uchar,
    mut size: size_t,
) {
    config.bindings.tqh_first = ::core::ptr::null_mut::<binding_t>();
    config.bindings.tqh_last = &raw mut config.bindings.tqh_first;
    config.tray_outputs.tqh_first = ::core::ptr::null_mut::<tray_output_t>();
    config.tray_outputs.tqh_last = &raw mut config.tray_outputs.tqh_first;
    let mut handle: yajl_handle = yajl_alloc(
        &raw mut outputs_callbacks,
        ::core::ptr::null_mut::<yajl_alloc_funcs>(),
        NULL,
    );
    let mut state: yajl_status = yajl_parse(handle, json, size);
    match state as ::core::ffi::c_uint {
        1 | 2 => {
            fprintf(
                stderr,
                b"[%s:%d] ERROR: Could not parse config reply!\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"../i3bar/src/config.c\0" as *const u8 as *const ::core::ffi::c_char,
                422 as ::core::ffi::c_int,
            );
            exit(EXIT_FAILURE);
        }
        0 | _ => {}
    }
    if config.disable_ws as ::core::ffi::c_int != 0 && !config.workspace_command.is_null() {
        fprintf(
            stderr,
            b"[%s:%d] ERROR: You have specified 'workspace_buttons no'. Your 'workspace_command %s' will be ignored.\n\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"../i3bar/src/config.c\0" as *const u8 as *const ::core::ffi::c_char,
            428 as ::core::ffi::c_int,
            config.workspace_command,
        );
        free(config.workspace_command as *mut ::core::ffi::c_void);
        config.workspace_command = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    yajl_free(handle);
}
unsafe extern "C" fn i3bar_config_string_cb(
    mut params_: *mut ::core::ffi::c_void,
    mut val: *const ::core::ffi::c_uchar,
    mut _len: size_t,
) -> ::core::ffi::c_int {
    sasprintf(
        &raw mut config.bar_id,
        b"%.*s\0" as *const u8 as *const ::core::ffi::c_char,
        _len as ::core::ffi::c_int,
        val,
    );
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn parse_get_first_i3bar_config(
    mut json: *const ::core::ffi::c_uchar,
    mut size: size_t,
) {
    let mut configs_callbacks: yajl_callbacks = yajl_callbacks {
        yajl_null: None,
        yajl_boolean: None,
        yajl_integer: None,
        yajl_double: None,
        yajl_number: None,
        yajl_string: Some(
            i3bar_config_string_cb
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_uchar,
                    size_t,
                ) -> ::core::ffi::c_int,
        ),
        yajl_start_map: None,
        yajl_map_key: None,
        yajl_end_map: None,
        yajl_start_array: None,
        yajl_end_array: None,
    };
    let mut handle: yajl_handle = yajl_alloc(
        &raw mut configs_callbacks,
        ::core::ptr::null_mut::<yajl_alloc_funcs>(),
        NULL,
    );
    yajl_parse(handle, json, size);
    yajl_free(handle);
}
#[no_mangle]
pub unsafe extern "C" fn free_colors(mut colors: *mut xcb_color_strings_t) {
    free((*colors).bar_fg as *mut ::core::ffi::c_void);
    (*colors).bar_fg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free((*colors).bar_bg as *mut ::core::ffi::c_void);
    (*colors).bar_bg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free((*colors).sep_fg as *mut ::core::ffi::c_void);
    (*colors).sep_fg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free((*colors).focus_bar_fg as *mut ::core::ffi::c_void);
    (*colors).focus_bar_fg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free((*colors).focus_bar_bg as *mut ::core::ffi::c_void);
    (*colors).focus_bar_bg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free((*colors).focus_sep_fg as *mut ::core::ffi::c_void);
    (*colors).focus_sep_fg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free((*colors).active_ws_fg as *mut ::core::ffi::c_void);
    (*colors).active_ws_fg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free((*colors).active_ws_bg as *mut ::core::ffi::c_void);
    (*colors).active_ws_bg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free((*colors).active_ws_border as *mut ::core::ffi::c_void);
    (*colors).active_ws_border = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free((*colors).inactive_ws_fg as *mut ::core::ffi::c_void);
    (*colors).inactive_ws_fg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free((*colors).inactive_ws_bg as *mut ::core::ffi::c_void);
    (*colors).inactive_ws_bg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free((*colors).inactive_ws_border as *mut ::core::ffi::c_void);
    (*colors).inactive_ws_border = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free((*colors).urgent_ws_fg as *mut ::core::ffi::c_void);
    (*colors).urgent_ws_fg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free((*colors).urgent_ws_bg as *mut ::core::ffi::c_void);
    (*colors).urgent_ws_bg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free((*colors).urgent_ws_border as *mut ::core::ffi::c_void);
    (*colors).urgent_ws_border = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free((*colors).focus_ws_fg as *mut ::core::ffi::c_void);
    (*colors).focus_ws_fg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free((*colors).focus_ws_bg as *mut ::core::ffi::c_void);
    (*colors).focus_ws_bg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free((*colors).focus_ws_border as *mut ::core::ffi::c_void);
    (*colors).focus_ws_border = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free((*colors).binding_mode_fg as *mut ::core::ffi::c_void);
    (*colors).binding_mode_fg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free((*colors).binding_mode_bg as *mut ::core::ffi::c_void);
    (*colors).binding_mode_bg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free((*colors).binding_mode_border as *mut ::core::ffi::c_void);
    (*colors).binding_mode_border = ::core::ptr::null_mut::<::core::ffi::c_char>();
}
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
