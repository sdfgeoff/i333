/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3bar - an xcb-based status- and ws-bar for i3
 * © 2010 Axel Wagner and contributors (see also: LICENSE)
 *
 * workspaces.c: Maintaining the workspace lists
 *
 */
// Mechanical file-by-file Rust translation of i3bar/src/workspaces.c.
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
    fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn scalloc(num: size_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn sstrndup(str: *const ::core::ffi::c_char, size: size_t) -> *mut ::core::ffi::c_char;
    fn sasprintf(
        strp: *mut *mut ::core::ffi::c_char,
        fmt: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn i3string_from_markup(from_markup: *const ::core::ffi::c_char) -> *mut i3String;
    fn i3string_from_markup_with_length(
        from_markup: *const ::core::ffi::c_char,
        num_bytes: size_t,
    ) -> *mut i3String;
    fn i3string_free(str: *mut i3String);
    fn i3string_as_utf8(str: *mut i3String) -> *const ::core::ffi::c_char;
    fn i3string_get_num_glyphs(str: *mut i3String) -> size_t;
    fn predict_text_width(text: *mut i3String) -> ::core::ffi::c_int;
    fn kill_ws_child();
    fn set_workspace_button_error(message: *const ::core::ffi::c_char);
    static mut outputs: *mut outputs_head;
    fn get_output_by_name(name: *mut ::core::ffi::c_char) -> *mut i3_output;
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
    fn yajl_get_error(
        hand: yajl_handle,
        verbose: ::core::ffi::c_int,
        jsonText: *const ::core::ffi::c_uchar,
        jsonTextLength: size_t,
    ) -> *mut ::core::ffi::c_uchar;
    fn yajl_free_error(hand: yajl_handle, str: *mut ::core::ffi::c_uchar);
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strspn(
        __s: *const ::core::ffi::c_char,
        __accept: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_ulong;
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
pub struct workspaces_json_params {
    pub workspaces: *mut ws_head,
    pub workspaces_walk: *mut i3_ws,
    pub cur_key: *mut ::core::ffi::c_char,
    pub need_output: bool,
    pub parsing_rect: bool,
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
unsafe extern "C" fn workspaces_boolean_cb(
    mut params_: *mut ::core::ffi::c_void,
    val: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut params: *mut workspaces_json_params = params_ as *mut workspaces_json_params;
    if strcmp(
        (*params).cur_key,
        b"visible\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        (*(*params).workspaces_walk).visible = val != 0;
        free((*params).cur_key as *mut ::core::ffi::c_void);
        (*params).cur_key = ::core::ptr::null_mut::<::core::ffi::c_char>();
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(
        (*params).cur_key,
        b"focused\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        (*(*params).workspaces_walk).focused = val != 0;
        free((*params).cur_key as *mut ::core::ffi::c_void);
        (*params).cur_key = ::core::ptr::null_mut::<::core::ffi::c_char>();
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(
        (*params).cur_key,
        b"urgent\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        (*(*params).workspaces_walk).urgent = val != 0;
        free((*params).cur_key as *mut ::core::ffi::c_void);
        (*params).cur_key = ::core::ptr::null_mut::<::core::ffi::c_char>();
        return 1 as ::core::ffi::c_int;
    }
    free((*params).cur_key as *mut ::core::ffi::c_void);
    (*params).cur_key = ::core::ptr::null_mut::<::core::ffi::c_char>();
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn workspaces_integer_cb(
    mut params_: *mut ::core::ffi::c_void,
    val: ::core::ffi::c_longlong,
) -> ::core::ffi::c_int {
    let mut params: *mut workspaces_json_params = params_ as *mut workspaces_json_params;
    if strcmp(
        (*params).cur_key,
        b"id\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        (*(*params).workspaces_walk).id = val as uintptr_t;
        free((*params).cur_key as *mut ::core::ffi::c_void);
        (*params).cur_key = ::core::ptr::null_mut::<::core::ffi::c_char>();
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(
        (*params).cur_key,
        b"num\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        (*(*params).workspaces_walk).num = val as ::core::ffi::c_int;
        free((*params).cur_key as *mut ::core::ffi::c_void);
        (*params).cur_key = ::core::ptr::null_mut::<::core::ffi::c_char>();
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(
        (*params).cur_key,
        b"x\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        free((*params).cur_key as *mut ::core::ffi::c_void);
        (*params).cur_key = ::core::ptr::null_mut::<::core::ffi::c_char>();
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(
        (*params).cur_key,
        b"y\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        free((*params).cur_key as *mut ::core::ffi::c_void);
        (*params).cur_key = ::core::ptr::null_mut::<::core::ffi::c_char>();
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(
        (*params).cur_key,
        b"width\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        free((*params).cur_key as *mut ::core::ffi::c_void);
        (*params).cur_key = ::core::ptr::null_mut::<::core::ffi::c_char>();
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(
        (*params).cur_key,
        b"height\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        free((*params).cur_key as *mut ::core::ffi::c_void);
        (*params).cur_key = ::core::ptr::null_mut::<::core::ffi::c_char>();
        return 1 as ::core::ffi::c_int;
    }
    free((*params).cur_key as *mut ::core::ffi::c_void);
    (*params).cur_key = ::core::ptr::null_mut::<::core::ffi::c_char>();
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn workspaces_string_cb(
    mut params_: *mut ::core::ffi::c_void,
    mut val: *const ::core::ffi::c_uchar,
    len: size_t,
) -> ::core::ffi::c_int {
    let mut params: *mut workspaces_json_params = params_ as *mut workspaces_json_params;
    if strcmp(
        (*params).cur_key,
        b"name\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        let mut ws_name: *const ::core::ffi::c_char = val as *const ::core::ffi::c_char;
        (*(*params).workspaces_walk).canonical_name = sstrndup(ws_name, len);
        if (config.strip_ws_numbers as ::core::ffi::c_int != 0
            || config.strip_ws_name as ::core::ffi::c_int != 0)
            && (*(*params).workspaces_walk).num >= 0 as ::core::ffi::c_int
        {
            static mut ws_num: [::core::ffi::c_char; 32] = [0; 32];
            snprintf(
                &raw mut ws_num as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 32]>() as size_t,
                b"%d\0" as *const u8 as *const ::core::ffi::c_char,
                (*(*params).workspaces_walk).num,
            );
            let mut offset: size_t =
                strspn(ws_name, &raw mut ws_num as *mut ::core::ffi::c_char) as size_t;
            if offset != 0 && *ws_name.offset(offset as isize) as ::core::ffi::c_int == ':' as i32 {
                offset = offset.wrapping_add(1 as size_t);
            }
            if config.strip_ws_numbers {
                (*(*params).workspaces_walk).name = if offset < len {
                    i3string_from_markup_with_length(
                        ws_name.offset(offset as isize),
                        len.wrapping_sub(offset),
                    )
                } else {
                    i3string_from_markup(&raw mut ws_num as *mut ::core::ffi::c_char)
                };
            } else {
                (*(*params).workspaces_walk).name =
                    i3string_from_markup(&raw mut ws_num as *mut ::core::ffi::c_char);
            }
        } else {
            (*(*params).workspaces_walk).name = i3string_from_markup_with_length(ws_name, len);
        }
        (*(*params).workspaces_walk).name_width =
            predict_text_width((*(*params).workspaces_walk).name);
        if config.verbose {
            printf(
                b"[%s:%d] Got workspace canonical: %s, name: '%s', name_width: %d, glyphs: %zu\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
                b"../i3bar/src/workspaces.c\0" as *const u8 as *const ::core::ffi::c_char,
                146 as ::core::ffi::c_int,
                (*(*params).workspaces_walk).canonical_name,
                i3string_as_utf8((*(*params).workspaces_walk).name),
                (*(*params).workspaces_walk).name_width,
                i3string_get_num_glyphs((*(*params).workspaces_walk).name),
            );
        }
        free((*params).cur_key as *mut ::core::ffi::c_void);
        (*params).cur_key = ::core::ptr::null_mut::<::core::ffi::c_char>();
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(
        (*params).cur_key,
        b"output\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        let mut output_name: *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<::core::ffi::c_char>();
        sasprintf(
            &raw mut output_name,
            b"%.*s\0" as *const u8 as *const ::core::ffi::c_char,
            len as ::core::ffi::c_int,
            val,
        );
        let mut target: *mut i3_output = get_output_by_name(output_name);
        let mut ws: *mut i3_ws = (*params).workspaces_walk;
        if !target.is_null() {
            (*ws).output = target as *mut i3_output;
            (*ws).tailq.tqe_next = ::core::ptr::null_mut::<i3_ws>();
            (*ws).tailq.tqe_prev = (*(*(*ws).output).workspaces).tqh_last;
            *(*(*(*ws).output).workspaces).tqh_last = ws as *mut i3_ws;
            (*(*(*ws).output).workspaces).tqh_last = &raw mut (*ws).tailq.tqe_next;
        }
        (*params).need_output = false_0 != 0;
        free(output_name as *mut ::core::ffi::c_void);
        output_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
        free((*params).cur_key as *mut ::core::ffi::c_void);
        (*params).cur_key = ::core::ptr::null_mut::<::core::ffi::c_char>();
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn workspaces_start_map_cb(
    mut params_: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut params: *mut workspaces_json_params = params_ as *mut workspaces_json_params;
    if (*params).cur_key.is_null() {
        let mut new_workspace: *mut i3_ws =
            scalloc(1 as size_t, ::core::mem::size_of::<i3_ws>() as size_t) as *mut i3_ws;
        (*new_workspace).num = -(1 as ::core::ffi::c_int);
        (*params).workspaces_walk = new_workspace;
        (*params).need_output = true_0 != 0;
        (*params).parsing_rect = false_0 != 0;
    } else {
        (*params).parsing_rect = true_0 != 0;
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn workspaces_end_map_cb(
    mut params_: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut params: *mut workspaces_json_params = params_ as *mut workspaces_json_params;
    if (*params).parsing_rect {
        (*params).parsing_rect = false_0 != 0;
        return 1 as ::core::ffi::c_int;
    }
    let mut ws: *mut i3_ws = (*params).workspaces_walk;
    if ws.is_null() || !(*ws).output.is_null() {
        return 1 as ::core::ffi::c_int;
    }
    if !(*params).need_output {
        i3string_free((*ws).name);
        (*ws).name = ::core::ptr::null_mut::<i3String>();
        free((*ws).canonical_name as *mut ::core::ffi::c_void);
        (*ws).canonical_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
        free((*params).workspaces_walk as *mut ::core::ffi::c_void);
        (*params).workspaces_walk = ::core::ptr::null_mut::<i3_ws>();
        return 1 as ::core::ffi::c_int;
    }
    if (*ws).name.is_null() || (*outputs).slh_first.is_null() {
        i3string_free((*ws).name);
        (*ws).name = ::core::ptr::null_mut::<i3String>();
        free((*ws).canonical_name as *mut ::core::ffi::c_void);
        (*ws).canonical_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
        free((*params).workspaces_walk as *mut ::core::ffi::c_void);
        (*params).workspaces_walk = ::core::ptr::null_mut::<i3_ws>();
        return 1 as ::core::ffi::c_int;
    }
    (*ws).output = get_output_by_name(
        b"primary\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ) as *mut i3_output;
    if (*ws).output.is_null() {
        (*ws).output = (*outputs).slh_first;
    }
    (*ws).tailq.tqe_next = ::core::ptr::null_mut::<i3_ws>();
    (*ws).tailq.tqe_prev = (*(*(*ws).output).workspaces).tqh_last;
    *(*(*(*ws).output).workspaces).tqh_last = ws as *mut i3_ws;
    (*(*(*ws).output).workspaces).tqh_last = &raw mut (*ws).tailq.tqe_next;
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn workspaces_map_key_cb(
    mut params_: *mut ::core::ffi::c_void,
    mut keyVal: *const ::core::ffi::c_uchar,
    keyLen: size_t,
) -> ::core::ffi::c_int {
    let mut params: *mut workspaces_json_params = params_ as *mut workspaces_json_params;
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
static mut workspaces_callbacks: yajl_callbacks = unsafe {
    yajl_callbacks {
        yajl_null: None,
        yajl_boolean: Some(
            workspaces_boolean_cb
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        yajl_integer: Some(
            workspaces_integer_cb
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_longlong,
                ) -> ::core::ffi::c_int,
        ),
        yajl_double: None,
        yajl_number: None,
        yajl_string: Some(
            workspaces_string_cb
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_uchar,
                    size_t,
                ) -> ::core::ffi::c_int,
        ),
        yajl_start_map: Some(
            workspaces_start_map_cb
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
        ),
        yajl_map_key: Some(
            workspaces_map_key_cb
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_uchar,
                    size_t,
                ) -> ::core::ffi::c_int,
        ),
        yajl_end_map: Some(
            workspaces_end_map_cb
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
        ),
        yajl_start_array: None,
        yajl_end_array: None,
    }
};
#[no_mangle]
pub unsafe extern "C" fn parse_workspaces_json(
    mut json: *const ::core::ffi::c_uchar,
    size: size_t,
) {
    free_workspaces();
    let mut params: workspaces_json_params = workspaces_json_params {
        workspaces: ::core::ptr::null_mut::<ws_head>(),
        workspaces_walk: ::core::ptr::null_mut::<i3_ws>(),
        cur_key: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        need_output: false,
        parsing_rect: false,
    };
    let handle: yajl_handle = yajl_alloc(
        &raw mut workspaces_callbacks,
        ::core::ptr::null_mut::<yajl_alloc_funcs>(),
        &raw mut params as *mut ::core::ffi::c_void,
    ) as yajl_handle;
    let state: yajl_status = yajl_parse(handle, json, size) as yajl_status;
    match state as ::core::ffi::c_uint {
        1 | 2 => {
            let mut err: *mut ::core::ffi::c_uchar =
                yajl_get_error(handle, 1 as ::core::ffi::c_int, json, size);
            fprintf(
                stderr,
                b"[%s:%d] ERROR: Could not parse workspaces reply, error:\n%s\njson:---%s---\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
                b"../i3bar/src/workspaces.c\0" as *const u8 as *const ::core::ffi::c_char,
                275 as ::core::ffi::c_int,
                err as *mut ::core::ffi::c_char,
                json as *mut ::core::ffi::c_char,
            );
            yajl_free_error(handle, err);
            if !config.workspace_command.is_null() {
                kill_ws_child();
                set_workspace_button_error(
                    b"Could not parse workspace_command's JSON\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            } else {
                exit(EXIT_FAILURE);
            }
        }
        0 | _ => {}
    }
    yajl_free(handle);
    free(params.cur_key as *mut ::core::ffi::c_void);
    params.cur_key = ::core::ptr::null_mut::<::core::ffi::c_char>();
}
#[no_mangle]
pub unsafe extern "C" fn free_workspaces() {
    if outputs.is_null() {
        return;
    }
    let mut outputs_walk: *mut i3_output = ::core::ptr::null_mut::<i3_output>();
    outputs_walk = (*outputs).slh_first as *mut i3_output;
    while !outputs_walk.is_null() {
        if !(*outputs_walk).workspaces.is_null()
            && !(*(*outputs_walk).workspaces).tqh_first.is_null()
        {
            let mut ws_walk: *mut i3_ws = ::core::ptr::null_mut::<i3_ws>();
            ws_walk = (*(*outputs_walk).workspaces).tqh_first as *mut i3_ws;
            while !ws_walk.is_null() {
                i3string_free((*ws_walk).name);
                (*ws_walk).name = ::core::ptr::null_mut::<i3String>();
                free((*ws_walk).canonical_name as *mut ::core::ffi::c_void);
                (*ws_walk).canonical_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
                ws_walk = (*ws_walk).tailq.tqe_next as *mut i3_ws;
            }
            let mut walk: *mut i3_ws = (*(*outputs_walk).workspaces).tqh_first as *mut i3_ws;
            while !(*(*outputs_walk).workspaces).tqh_first.is_null() {
                if !(*(*(*outputs_walk).workspaces).tqh_first)
                    .tailq
                    .tqe_next
                    .is_null()
                {
                    (*(*(*(*outputs_walk).workspaces).tqh_first).tailq.tqe_next)
                        .tailq
                        .tqe_prev = (*(*(*outputs_walk).workspaces).tqh_first).tailq.tqe_prev;
                } else {
                    (*(*outputs_walk).workspaces).tqh_last =
                        (*(*(*outputs_walk).workspaces).tqh_first).tailq.tqe_prev;
                }
                *(*(*(*outputs_walk).workspaces).tqh_first).tailq.tqe_prev =
                    (*(*(*outputs_walk).workspaces).tqh_first).tailq.tqe_next;
                free(walk as *mut ::core::ffi::c_void);
                walk = ::core::ptr::null_mut::<i3_ws>();
                walk = (*(*outputs_walk).workspaces).tqh_first as *mut i3_ws;
            }
        }
        outputs_walk = (*outputs_walk).slist.sle_next as *mut i3_output;
    }
}
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
