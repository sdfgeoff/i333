/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3bar - an xcb-based status- and ws-bar for i3
 * © 2010 Axel Wagner and contributors (see also: LICENSE)
 *
 * outputs.c: Maintaining the outputs list
 *
 */
// Mechanical file-by-file Rust translation of i3bar/src/outputs.c.
// Generated with C2Rust 0.22.1; C ABI and original ownership retained.
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type _cairo;
    pub type _cairo_surface;
    pub type _i3String;
    pub type yajl_handle_t;
    static mut stderr: *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn smalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn sasprintf(
        strp: *mut *mut ::core::ffi::c_char,
        fmt: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn free_workspaces();
    fn destroy_window(output: *mut i3_output);
    static mut config: config_t;
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
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strcasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn exit(__status: ::core::ffi::c_int) -> !;
}
pub type size_t = usize;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
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
pub type uintptr_t = usize;
pub type xcb_window_t = uint32_t;
pub type xcb_gcontext_t = uint32_t;
pub type xcb_drawable_t = uint32_t;
pub type cairo_t = _cairo;
pub type cairo_surface_t = _cairo_surface;
pub type i3String = _i3String;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rect_t {
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub w: ::core::ffi::c_int,
    pub h: ::core::ffi::c_int,
}
pub type rect = rect_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct i3_output {
    pub name: *mut ::core::ffi::c_char,
    pub active: bool,
    pub primary: bool,
    pub visible: bool,
    pub ws: ::core::ffi::c_int,
    pub rect: rect,
    pub buffer: surface_t,
    pub statusline_buffer: surface_t,
    pub statusline_width: ::core::ffi::c_int,
    pub bar: surface_t,
    pub workspaces: *mut ws_head,
    pub trayclients: *mut tc_head,
    pub slist: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub sle_next: *mut i3_output,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tc_head {
    pub tqh_first: *mut trayclient,
    pub tqh_last: *mut *mut trayclient,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct trayclient {
    pub win: xcb_window_t,
    pub mapped: bool,
    pub xe_version: ::core::ffi::c_int,
    pub class_class: *mut ::core::ffi::c_char,
    pub class_instance: *mut ::core::ffi::c_char,
    pub tailq: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub tqe_next: *mut trayclient,
    pub tqe_prev: *mut *mut trayclient,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ws_head {
    pub tqh_first: *mut i3_ws,
    pub tqh_last: *mut *mut i3_ws,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct i3_ws {
    pub id: uintptr_t,
    pub num: ::core::ffi::c_int,
    pub canonical_name: *mut ::core::ffi::c_char,
    pub name: *mut i3String,
    pub name_width: ::core::ffi::c_int,
    pub visible: bool,
    pub focused: bool,
    pub urgent: bool,
    pub output: *mut i3_output,
    pub tailq: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub tqe_next: *mut i3_ws,
    pub tqe_prev: *mut *mut i3_ws,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct outputs_head {
    pub slh_first: *mut i3_output,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct outputs_json_params {
    pub outputs_walk: *mut i3_output,
    pub cur_key: *mut ::core::ffi::c_char,
    pub in_rect: bool,
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
    pub hidden_state: C2RustUnnamed_2,
}
pub type C2RustUnnamed_2 = ::core::ffi::c_uint;
pub const S_SHOW: C2RustUnnamed_2 = 1;
pub const S_HIDE: C2RustUnnamed_2 = 0;
pub type bar_display_mode_t = ::core::ffi::c_uint;
pub const M_INVISIBLE: bar_display_mode_t = 2;
pub const M_HIDE: bar_display_mode_t = 1;
pub const M_DOCK: bar_display_mode_t = 0;
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
    pub tray_outputs: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub tqe_next: *mut tray_output_t,
    pub tqe_prev: *mut *mut tray_output_t,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Rect {
    pub x: uint32_t,
    pub y: uint32_t,
    pub width: uint32_t,
    pub height: uint32_t,
}
pub type position_t = ::core::ffi::c_uint;
pub const POS_BOT: position_t = 2;
pub const POS_TOP: position_t = 1;
pub const POS_NONE: position_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bindings_head {
    pub tqh_first: *mut binding_t,
    pub tqh_last: *mut *mut binding_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct binding_t {
    pub input_code: ::core::ffi::c_int,
    pub command: *mut ::core::ffi::c_char,
    pub release: bool,
    pub bindings: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub tqe_next: *mut binding_t,
    pub tqe_prev: *mut *mut binding_t,
}
pub const yajl_status_error: yajl_status = 2;
pub const yajl_status_client_canceled: yajl_status = 1;
pub const yajl_status_ok: yajl_status = 0;
pub type yajl_status = ::core::ffi::c_uint;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
unsafe extern "C" fn outputs_null_cb(mut params_: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    let mut params: *mut outputs_json_params = params_ as *mut outputs_json_params;
    free((*params).cur_key as *mut ::core::ffi::c_void);
    (*params).cur_key = ::core::ptr::null_mut::<::core::ffi::c_char>();
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn outputs_boolean_cb(
    mut params_: *mut ::core::ffi::c_void,
    mut val: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut params: *mut outputs_json_params = params_ as *mut outputs_json_params;
    if strcmp(
        (*params).cur_key,
        b"active\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        (*(*params).outputs_walk).active = val != 0;
        free((*params).cur_key as *mut ::core::ffi::c_void);
        (*params).cur_key = ::core::ptr::null_mut::<::core::ffi::c_char>();
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(
        (*params).cur_key,
        b"primary\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        (*(*params).outputs_walk).primary = val != 0;
        free((*params).cur_key as *mut ::core::ffi::c_void);
        (*params).cur_key = ::core::ptr::null_mut::<::core::ffi::c_char>();
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn outputs_integer_cb(
    mut params_: *mut ::core::ffi::c_void,
    mut val: ::core::ffi::c_longlong,
) -> ::core::ffi::c_int {
    let mut params: *mut outputs_json_params = params_ as *mut outputs_json_params;
    if strcmp(
        (*params).cur_key,
        b"current_workspace\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        (*(*params).outputs_walk).ws = val as ::core::ffi::c_int;
        free((*params).cur_key as *mut ::core::ffi::c_void);
        (*params).cur_key = ::core::ptr::null_mut::<::core::ffi::c_char>();
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(
        (*params).cur_key,
        b"x\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        (*(*params).outputs_walk).rect.x = val as ::core::ffi::c_int;
        free((*params).cur_key as *mut ::core::ffi::c_void);
        (*params).cur_key = ::core::ptr::null_mut::<::core::ffi::c_char>();
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(
        (*params).cur_key,
        b"y\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        (*(*params).outputs_walk).rect.y = val as ::core::ffi::c_int;
        free((*params).cur_key as *mut ::core::ffi::c_void);
        (*params).cur_key = ::core::ptr::null_mut::<::core::ffi::c_char>();
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(
        (*params).cur_key,
        b"width\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        (*(*params).outputs_walk).rect.w = val as ::core::ffi::c_int;
        free((*params).cur_key as *mut ::core::ffi::c_void);
        (*params).cur_key = ::core::ptr::null_mut::<::core::ffi::c_char>();
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(
        (*params).cur_key,
        b"height\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        (*(*params).outputs_walk).rect.h = val as ::core::ffi::c_int;
        free((*params).cur_key as *mut ::core::ffi::c_void);
        (*params).cur_key = ::core::ptr::null_mut::<::core::ffi::c_char>();
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn outputs_string_cb(
    mut params_: *mut ::core::ffi::c_void,
    mut val: *const ::core::ffi::c_uchar,
    mut len: size_t,
) -> ::core::ffi::c_int {
    let mut params: *mut outputs_json_params = params_ as *mut outputs_json_params;
    if strcmp(
        (*params).cur_key,
        b"current_workspace\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        let mut copy: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        sasprintf(
            &raw mut copy,
            b"%.*s\0" as *const u8 as *const ::core::ffi::c_char,
            len as ::core::ffi::c_int,
            val,
        );
        let mut end: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        *__errno_location() = 0 as ::core::ffi::c_int;
        let mut parsed_num: ::core::ffi::c_long =
            strtol(copy, &raw mut end, 10 as ::core::ffi::c_int);
        if *__errno_location() == 0 as ::core::ffi::c_int
            && (!end.is_null() && *end as ::core::ffi::c_int == '\0' as i32)
        {
            (*(*params).outputs_walk).ws = parsed_num as ::core::ffi::c_int;
        }
        free(copy as *mut ::core::ffi::c_void);
        copy = ::core::ptr::null_mut::<::core::ffi::c_char>();
        free((*params).cur_key as *mut ::core::ffi::c_void);
        (*params).cur_key = ::core::ptr::null_mut::<::core::ffi::c_char>();
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(
        (*params).cur_key,
        b"name\0" as *const u8 as *const ::core::ffi::c_char,
    ) != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    sasprintf(
        &raw mut (*(*params).outputs_walk).name,
        b"%.*s\0" as *const u8 as *const ::core::ffi::c_char,
        len as ::core::ffi::c_int,
        val,
    );
    free((*params).cur_key as *mut ::core::ffi::c_void);
    (*params).cur_key = ::core::ptr::null_mut::<::core::ffi::c_char>();
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn outputs_start_map_cb(
    mut params_: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut params: *mut outputs_json_params = params_ as *mut outputs_json_params;
    let mut new_output: *mut i3_output = ::core::ptr::null_mut::<i3_output>();
    if (*params).cur_key.is_null() {
        new_output = smalloc(::core::mem::size_of::<i3_output>() as size_t) as *mut i3_output;
        (*new_output).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
        (*new_output).active = false_0 != 0;
        (*new_output).primary = false_0 != 0;
        (*new_output).visible = false_0 != 0;
        (*new_output).ws = 0 as ::core::ffi::c_int;
        (*new_output).statusline_width = 0 as ::core::ffi::c_int;
        memset(
            &raw mut (*new_output).rect as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<rect>() as size_t,
        );
        memset(
            &raw mut (*new_output).bar as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<surface_t>() as size_t,
        );
        memset(
            &raw mut (*new_output).buffer as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<surface_t>() as size_t,
        );
        memset(
            &raw mut (*new_output).statusline_buffer as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<surface_t>() as size_t,
        );
        (*new_output).workspaces =
            smalloc(::core::mem::size_of::<ws_head>() as size_t) as *mut ws_head;
        (*(*new_output).workspaces).tqh_first = ::core::ptr::null_mut::<i3_ws>();
        (*(*new_output).workspaces).tqh_last = &raw mut (*(*new_output).workspaces).tqh_first;
        (*new_output).trayclients =
            smalloc(::core::mem::size_of::<tc_head>() as size_t) as *mut tc_head;
        (*(*new_output).trayclients).tqh_first = ::core::ptr::null_mut::<trayclient>();
        (*(*new_output).trayclients).tqh_last = &raw mut (*(*new_output).trayclients).tqh_first;
        (*params).outputs_walk = new_output;
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(
        (*params).cur_key,
        b"rect\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        (*params).in_rect = true_0 != 0;
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn clear_output(mut output: *mut i3_output) {
    free((*output).name as *mut ::core::ffi::c_void);
    (*output).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free((*output).workspaces as *mut ::core::ffi::c_void);
    (*output).workspaces = ::core::ptr::null_mut::<ws_head>();
    free((*output).trayclients as *mut ::core::ffi::c_void);
    (*output).trayclients = ::core::ptr::null_mut::<tc_head>();
}
unsafe extern "C" fn outputs_end_map_cb(
    mut params_: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut params: *mut outputs_json_params = params_ as *mut outputs_json_params;
    if (*params).in_rect {
        (*params).in_rect = false_0 != 0;
        return 1 as ::core::ffi::c_int;
    }
    if config.num_outputs > 0 as ::core::ffi::c_int {
        let is_primary: bool = (*(*params).outputs_walk).primary;
        let mut handle_output: bool = false_0 != 0;
        let mut c: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while c < config.num_outputs {
            if strcasecmp(
                (*(*params).outputs_walk).name,
                *config.outputs.offset(c as isize),
            ) == 0 as ::core::ffi::c_int
                || strcasecmp(
                    *config.outputs.offset(c as isize),
                    b"primary\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                    && is_primary as ::core::ffi::c_int != 0
                || strcasecmp(
                    *config.outputs.offset(c as isize),
                    b"nonprimary\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                    && !is_primary
            {
                handle_output = true_0 != 0;
                break;
            } else {
                c += 1;
            }
        }
        if !handle_output {
            if config.verbose {
                printf(
                    b"[%s:%d] Ignoring output \"%s\", not configured to handle it.\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    b"../i3bar/src/outputs.c\0" as *const u8 as *const ::core::ffi::c_char,
                    205 as ::core::ffi::c_int,
                    (*(*params).outputs_walk).name,
                );
            }
            clear_output((*params).outputs_walk);
            free((*params).outputs_walk as *mut ::core::ffi::c_void);
            (*params).outputs_walk = ::core::ptr::null_mut::<i3_output>();
            free((*params).cur_key as *mut ::core::ffi::c_void);
            (*params).cur_key = ::core::ptr::null_mut::<::core::ffi::c_char>();
            return 1 as ::core::ffi::c_int;
        }
    }
    let mut target: *mut i3_output = get_output_by_name((*(*params).outputs_walk).name);
    if target.is_null() {
        (*(*params).outputs_walk).slist.sle_next = (*outputs).slh_first;
        (*outputs).slh_first = (*params).outputs_walk as *mut i3_output;
    } else {
        (*target).active = (*(*params).outputs_walk).active;
        (*target).primary = (*(*params).outputs_walk).primary;
        (*target).ws = (*(*params).outputs_walk).ws;
        (*target).rect = (*(*params).outputs_walk).rect;
        clear_output((*params).outputs_walk);
        free((*params).outputs_walk as *mut ::core::ffi::c_void);
        (*params).outputs_walk = ::core::ptr::null_mut::<i3_output>();
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn outputs_map_key_cb(
    mut params_: *mut ::core::ffi::c_void,
    mut keyVal: *const ::core::ffi::c_uchar,
    mut keyLen: size_t,
) -> ::core::ffi::c_int {
    let mut params: *mut outputs_json_params = params_ as *mut outputs_json_params;
    free((*params).cur_key as *mut ::core::ffi::c_void);
    (*params).cur_key = ::core::ptr::null_mut::<::core::ffi::c_char>();
    sasprintf(
        &raw mut (*params).cur_key,
        b"%.*s\0" as *const u8 as *const ::core::ffi::c_char,
        keyLen as ::core::ffi::c_int,
        keyVal,
    );
    return 1 as ::core::ffi::c_int;
}
static mut outputs_callbacks: yajl_callbacks = unsafe {
    yajl_callbacks {
        yajl_null: Some(
            outputs_null_cb as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
        ),
        yajl_boolean: Some(
            outputs_boolean_cb
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        yajl_integer: Some(
            outputs_integer_cb
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_longlong,
                ) -> ::core::ffi::c_int,
        ),
        yajl_double: None,
        yajl_number: None,
        yajl_string: Some(
            outputs_string_cb
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_uchar,
                    size_t,
                ) -> ::core::ffi::c_int,
        ),
        yajl_start_map: Some(
            outputs_start_map_cb
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
        ),
        yajl_map_key: Some(
            outputs_map_key_cb
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_uchar,
                    size_t,
                ) -> ::core::ffi::c_int,
        ),
        yajl_end_map: Some(
            outputs_end_map_cb
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
        ),
        yajl_start_array: None,
        yajl_end_array: None,
    }
};
#[no_mangle]
pub static mut outputs: *mut outputs_head =
    ::core::ptr::null::<outputs_head>() as *mut outputs_head;
#[no_mangle]
pub unsafe extern "C" fn init_outputs() {
    outputs = smalloc(::core::mem::size_of::<outputs_head>() as size_t) as *mut outputs_head;
    (*outputs).slh_first = ::core::ptr::null_mut::<i3_output>();
}
#[no_mangle]
pub unsafe extern "C" fn parse_outputs_json(
    mut json: *const ::core::ffi::c_uchar,
    mut size: size_t,
) {
    let mut params: outputs_json_params = outputs_json_params {
        outputs_walk: ::core::ptr::null_mut::<i3_output>(),
        cur_key: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        in_rect: false,
    };
    params.outputs_walk = ::core::ptr::null_mut::<i3_output>();
    params.cur_key = ::core::ptr::null_mut::<::core::ffi::c_char>();
    params.in_rect = false_0 != 0;
    let mut handle: yajl_handle = yajl_alloc(
        &raw mut outputs_callbacks,
        ::core::ptr::null_mut::<yajl_alloc_funcs>(),
        &raw mut params as *mut ::core::ffi::c_void,
    );
    let mut state: yajl_status = yajl_parse(handle, json, size);
    match state as ::core::ffi::c_uint {
        1 | 2 => {
            fprintf(
                stderr,
                b"[%s:%d] ERROR: Could not parse outputs reply!\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"../i3bar/src/outputs.c\0" as *const u8 as *const ::core::ffi::c_char,
                282 as ::core::ffi::c_int,
            );
            exit(EXIT_FAILURE);
        }
        0 | _ => {}
    }
    yajl_free(handle);
    free(params.cur_key as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn free_outputs() {
    free_workspaces();
    let mut outputs_walk: *mut i3_output = ::core::ptr::null_mut::<i3_output>();
    if outputs.is_null() {
        return;
    }
    outputs_walk = (*outputs).slh_first as *mut i3_output;
    while !outputs_walk.is_null() {
        destroy_window(outputs_walk);
        if !(*outputs_walk).trayclients.is_null()
            && !(*(*outputs_walk).trayclients).tqh_first.is_null()
        {
            let mut walk: *mut trayclient =
                (*(*outputs_walk).trayclients).tqh_first as *mut trayclient;
            while !(*(*outputs_walk).trayclients).tqh_first.is_null() {
                if !(*(*(*outputs_walk).trayclients).tqh_first)
                    .tailq
                    .tqe_next
                    .is_null()
                {
                    (*(*(*(*outputs_walk).trayclients).tqh_first).tailq.tqe_next)
                        .tailq
                        .tqe_prev = (*(*(*outputs_walk).trayclients).tqh_first).tailq.tqe_prev;
                } else {
                    (*(*outputs_walk).trayclients).tqh_last =
                        (*(*(*outputs_walk).trayclients).tqh_first).tailq.tqe_prev;
                }
                *(*(*(*outputs_walk).trayclients).tqh_first).tailq.tqe_prev =
                    (*(*(*outputs_walk).trayclients).tqh_first).tailq.tqe_next;
                free(walk as *mut ::core::ffi::c_void);
                walk = ::core::ptr::null_mut::<trayclient>();
                walk = (*(*outputs_walk).trayclients).tqh_first as *mut trayclient;
            }
        }
        clear_output(outputs_walk);
        outputs_walk = (*outputs_walk).slist.sle_next as *mut i3_output;
    }
    let mut walk_0: *mut i3_output = (*outputs).slh_first as *mut i3_output;
    while !(*outputs).slh_first.is_null() {
        (*outputs).slh_first = (*(*outputs).slh_first).slist.sle_next;
        free(walk_0 as *mut ::core::ffi::c_void);
        walk_0 = ::core::ptr::null_mut::<i3_output>();
        walk_0 = (*outputs).slh_first as *mut i3_output;
    }
}
#[no_mangle]
pub unsafe extern "C" fn get_output_by_name(mut name: *mut ::core::ffi::c_char) -> *mut i3_output {
    if name.is_null() {
        return ::core::ptr::null_mut::<i3_output>();
    }
    let is_primary: bool = strcasecmp(
        name,
        b"primary\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0;
    let mut walk: *mut i3_output = ::core::ptr::null_mut::<i3_output>();
    walk = (*outputs).slh_first as *mut i3_output;
    while !walk.is_null() {
        if is_primary as ::core::ffi::c_int != 0 && (*walk).primary as ::core::ffi::c_int != 0
            || strcmp((*walk).name, name) == 0
        {
            break;
        }
        walk = (*walk).slist.sle_next as *mut i3_output;
    }
    return walk;
}
#[no_mangle]
pub unsafe extern "C" fn output_has_focus(mut output: *mut i3_output) -> bool {
    let mut ws_walk: *mut i3_ws = ::core::ptr::null_mut::<i3_ws>();
    ws_walk = (*(*output).workspaces).tqh_first as *mut i3_ws;
    while !ws_walk.is_null() {
        if (*ws_walk).focused {
            return true_0 != 0;
        }
        ws_walk = (*ws_walk).tailq.tqe_next as *mut i3_ws;
    }
    return false_0 != 0;
}
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
