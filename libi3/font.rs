/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 */
// Mechanical file-by-file Rust translation of libi3/font.c.
// Generated with C2Rust 0.22.1; C ABI and original ownership retained.
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type xcb_connection_t;
    pub type _PangoContext;
    pub type _PangoFontDescription;
    pub type _PangoLayout;
    pub type _cairo;
    pub type _cairo_surface;
    pub type _i3String;
    static mut stderr: *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn xcb_open_font_checked(
        c: *mut xcb_connection_t,
        fid: xcb_font_t,
        name_len: uint16_t,
        name: *const ::core::ffi::c_char,
    ) -> xcb_void_cookie_t;
    fn xcb_close_font(c: *mut xcb_connection_t, font: xcb_font_t) -> xcb_void_cookie_t;
    fn xcb_query_font(c: *mut xcb_connection_t, font: xcb_fontable_t) -> xcb_query_font_cookie_t;
    fn xcb_query_font_char_infos(R: *const xcb_query_font_reply_t) -> *mut xcb_charinfo_t;
    fn xcb_query_font_char_infos_length(R: *const xcb_query_font_reply_t) -> ::core::ffi::c_int;
    fn xcb_query_font_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_query_font_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_query_font_reply_t;
    fn xcb_query_text_extents(
        c: *mut xcb_connection_t,
        font: xcb_fontable_t,
        string_len: uint32_t,
        string: *const xcb_char2b_t,
    ) -> xcb_query_text_extents_cookie_t;
    fn xcb_query_text_extents_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_query_text_extents_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_query_text_extents_reply_t;
    fn xcb_change_gc(
        c: *mut xcb_connection_t,
        gc: xcb_gcontext_t,
        value_mask: uint32_t,
        value_list: *const ::core::ffi::c_void,
    ) -> xcb_void_cookie_t;
    fn xcb_image_text_16(
        c: *mut xcb_connection_t,
        string_len: uint8_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        x: int16_t,
        y: int16_t,
        string: *const xcb_char2b_t,
    ) -> xcb_void_cookie_t;
    fn xcb_request_check(
        c: *mut xcb_connection_t,
        cookie: xcb_void_cookie_t,
    ) -> *mut xcb_generic_error_t;
    fn xcb_generate_id(c: *mut xcb_connection_t) -> uint32_t;
    fn g_object_unref(object: gpointer);
    fn pango_font_description_free(desc: *mut PangoFontDescription);
    fn pango_font_description_get_family(
        desc: *const PangoFontDescription,
    ) -> *const ::core::ffi::c_char;
    fn pango_font_description_get_size(desc: *const PangoFontDescription) -> gint;
    fn pango_font_description_from_string(
        str: *const ::core::ffi::c_char,
    ) -> *mut PangoFontDescription;
    fn pango_layout_new(context: *mut PangoContext) -> *mut PangoLayout;
    fn pango_layout_set_text(
        layout: *mut PangoLayout,
        text: *const ::core::ffi::c_char,
        length: ::core::ffi::c_int,
    );
    fn pango_layout_set_markup(
        layout: *mut PangoLayout,
        markup: *const ::core::ffi::c_char,
        length: ::core::ffi::c_int,
    );
    fn pango_layout_set_font_description(
        layout: *mut PangoLayout,
        desc: *const PangoFontDescription,
    );
    fn pango_layout_set_width(layout: *mut PangoLayout, width: ::core::ffi::c_int);
    fn pango_layout_set_wrap(layout: *mut PangoLayout, wrap: PangoWrapMode);
    fn pango_layout_set_single_paragraph_mode(layout: *mut PangoLayout, setting: gboolean);
    fn pango_layout_set_ellipsize(layout: *mut PangoLayout, ellipsize: PangoEllipsizeMode);
    fn pango_layout_get_pixel_size(
        layout: *mut PangoLayout,
        width: *mut ::core::ffi::c_int,
        height: *mut ::core::ffi::c_int,
    );
    fn cairo_create(target: *mut cairo_surface_t) -> *mut cairo_t;
    fn cairo_destroy(cr: *mut cairo_t);
    fn cairo_set_operator(cr: *mut cairo_t, op: cairo_operator_t);
    fn cairo_set_source_rgba(
        cr: *mut cairo_t,
        red: ::core::ffi::c_double,
        green: ::core::ffi::c_double,
        blue: ::core::ffi::c_double,
        alpha: ::core::ffi::c_double,
    );
    fn cairo_move_to(cr: *mut cairo_t, x: ::core::ffi::c_double, y: ::core::ffi::c_double);
    fn cairo_surface_destroy(surface: *mut cairo_surface_t);
    fn cairo_xcb_surface_create(
        connection: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        visual: *mut xcb_visualtype_t,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
    ) -> *mut cairo_surface_t;
    static mut conn: *mut xcb_connection_t;
    static mut root_screen: *mut xcb_screen_t;
    fn verboselog(fmt: *mut ::core::ffi::c_char, ...);
    fn errorlog(fmt: *mut ::core::ffi::c_char, ...);
    fn sstrdup(str: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn i3string_as_utf8(str: *mut i3String) -> *const ::core::ffi::c_char;
    fn i3string_as_ucs2(str: *mut i3String) -> *const xcb_char2b_t;
    fn i3string_get_num_bytes(str: *mut i3String) -> size_t;
    fn i3string_is_markup(str: *mut i3String) -> bool;
    fn i3string_get_num_glyphs(str: *mut i3String) -> size_t;
    fn get_visualtype(screen: *mut xcb_screen_t) -> *mut xcb_visualtype_t;
    fn get_dpi_value() -> ::core::ffi::c_long;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn __assert_single_arg(_: bool) -> bool;
    fn errx(__status: ::core::ffi::c_int, __format: *const ::core::ffi::c_char, ...) -> !;
    fn pango_cairo_context_set_resolution(context: *mut PangoContext, dpi: ::core::ffi::c_double);
    fn pango_cairo_create_context(cr: *mut cairo_t) -> *mut PangoContext;
    fn pango_cairo_update_layout(cr: *mut cairo_t, layout: *mut PangoLayout);
    fn pango_cairo_show_layout(cr: *mut cairo_t, layout: *mut PangoLayout);
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_char2b_t {
    pub byte1: uint8_t,
    pub byte2: uint8_t,
}
pub type xcb_window_t = uint32_t;
pub type xcb_font_t = uint32_t;
pub type xcb_gcontext_t = uint32_t;
pub type xcb_colormap_t = uint32_t;
pub type xcb_drawable_t = uint32_t;
pub type xcb_fontable_t = uint32_t;
pub type xcb_visualid_t = uint32_t;
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
pub struct xcb_query_font_cookie_t {
    pub sequence: ::core::ffi::c_uint,
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
pub struct xcb_query_text_extents_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_query_text_extents_reply_t {
    pub response_type: uint8_t,
    pub draw_direction: uint8_t,
    pub sequence: uint16_t,
    pub length: uint32_t,
    pub font_ascent: int16_t,
    pub font_descent: int16_t,
    pub overall_ascent: int16_t,
    pub overall_descent: int16_t,
    pub overall_width: int32_t,
    pub overall_left: int32_t,
    pub overall_right: int32_t,
}
pub type xcb_gc_t = ::core::ffi::c_uint;
pub const XCB_GC_ARC_MODE: xcb_gc_t = 4194304;
pub const XCB_GC_DASH_LIST: xcb_gc_t = 2097152;
pub const XCB_GC_DASH_OFFSET: xcb_gc_t = 1048576;
pub const XCB_GC_CLIP_MASK: xcb_gc_t = 524288;
pub const XCB_GC_CLIP_ORIGIN_Y: xcb_gc_t = 262144;
pub const XCB_GC_CLIP_ORIGIN_X: xcb_gc_t = 131072;
pub const XCB_GC_GRAPHICS_EXPOSURES: xcb_gc_t = 65536;
pub const XCB_GC_SUBWINDOW_MODE: xcb_gc_t = 32768;
pub const XCB_GC_FONT: xcb_gc_t = 16384;
pub const XCB_GC_TILE_STIPPLE_ORIGIN_Y: xcb_gc_t = 8192;
pub const XCB_GC_TILE_STIPPLE_ORIGIN_X: xcb_gc_t = 4096;
pub const XCB_GC_STIPPLE: xcb_gc_t = 2048;
pub const XCB_GC_TILE: xcb_gc_t = 1024;
pub const XCB_GC_FILL_RULE: xcb_gc_t = 512;
pub const XCB_GC_FILL_STYLE: xcb_gc_t = 256;
pub const XCB_GC_JOIN_STYLE: xcb_gc_t = 128;
pub const XCB_GC_CAP_STYLE: xcb_gc_t = 64;
pub const XCB_GC_LINE_STYLE: xcb_gc_t = 32;
pub const XCB_GC_LINE_WIDTH: xcb_gc_t = 16;
pub const XCB_GC_BACKGROUND: xcb_gc_t = 8;
pub const XCB_GC_FOREGROUND: xcb_gc_t = 4;
pub const XCB_GC_PLANE_MASK: xcb_gc_t = 2;
pub const XCB_GC_FUNCTION: xcb_gc_t = 1;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type gpointer = *mut ::core::ffi::c_void;
pub type PangoContext = _PangoContext;
pub type PangoFontDescription = _PangoFontDescription;
pub type PangoLayout = _PangoLayout;
pub type PangoWrapMode = ::core::ffi::c_uint;
pub const PANGO_WRAP_NONE: PangoWrapMode = 3;
pub const PANGO_WRAP_WORD_CHAR: PangoWrapMode = 2;
pub const PANGO_WRAP_CHAR: PangoWrapMode = 1;
pub const PANGO_WRAP_WORD: PangoWrapMode = 0;
pub type PangoEllipsizeMode = ::core::ffi::c_uint;
pub const PANGO_ELLIPSIZE_END: PangoEllipsizeMode = 3;
pub const PANGO_ELLIPSIZE_MIDDLE: PangoEllipsizeMode = 2;
pub const PANGO_ELLIPSIZE_START: PangoEllipsizeMode = 1;
pub const PANGO_ELLIPSIZE_NONE: PangoEllipsizeMode = 0;
pub type cairo_t = _cairo;
pub type cairo_surface_t = _cairo_surface;
pub type _cairo_operator = ::core::ffi::c_uint;
pub const CAIRO_OPERATOR_HSL_LUMINOSITY: _cairo_operator = 28;
pub const CAIRO_OPERATOR_HSL_COLOR: _cairo_operator = 27;
pub const CAIRO_OPERATOR_HSL_SATURATION: _cairo_operator = 26;
pub const CAIRO_OPERATOR_HSL_HUE: _cairo_operator = 25;
pub const CAIRO_OPERATOR_EXCLUSION: _cairo_operator = 24;
pub const CAIRO_OPERATOR_DIFFERENCE: _cairo_operator = 23;
pub const CAIRO_OPERATOR_SOFT_LIGHT: _cairo_operator = 22;
pub const CAIRO_OPERATOR_HARD_LIGHT: _cairo_operator = 21;
pub const CAIRO_OPERATOR_COLOR_BURN: _cairo_operator = 20;
pub const CAIRO_OPERATOR_COLOR_DODGE: _cairo_operator = 19;
pub const CAIRO_OPERATOR_LIGHTEN: _cairo_operator = 18;
pub const CAIRO_OPERATOR_DARKEN: _cairo_operator = 17;
pub const CAIRO_OPERATOR_OVERLAY: _cairo_operator = 16;
pub const CAIRO_OPERATOR_SCREEN: _cairo_operator = 15;
pub const CAIRO_OPERATOR_MULTIPLY: _cairo_operator = 14;
pub const CAIRO_OPERATOR_SATURATE: _cairo_operator = 13;
pub const CAIRO_OPERATOR_ADD: _cairo_operator = 12;
pub const CAIRO_OPERATOR_XOR: _cairo_operator = 11;
pub const CAIRO_OPERATOR_DEST_ATOP: _cairo_operator = 10;
pub const CAIRO_OPERATOR_DEST_OUT: _cairo_operator = 9;
pub const CAIRO_OPERATOR_DEST_IN: _cairo_operator = 8;
pub const CAIRO_OPERATOR_DEST_OVER: _cairo_operator = 7;
pub const CAIRO_OPERATOR_DEST: _cairo_operator = 6;
pub const CAIRO_OPERATOR_ATOP: _cairo_operator = 5;
pub const CAIRO_OPERATOR_OUT: _cairo_operator = 4;
pub const CAIRO_OPERATOR_IN: _cairo_operator = 3;
pub const CAIRO_OPERATOR_OVER: _cairo_operator = 2;
pub const CAIRO_OPERATOR_SOURCE: _cairo_operator = 1;
pub const CAIRO_OPERATOR_CLEAR: _cairo_operator = 0;
pub type cairo_operator_t = _cairo_operator;
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
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const PANGO_SCALE: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
static mut savedFont: *const i3Font = ::core::ptr::null::<i3Font>();
static mut root_visual_type: *mut xcb_visualtype_t =
    ::core::ptr::null::<xcb_visualtype_t>() as *mut xcb_visualtype_t;
static mut pango_font_red: ::core::ffi::c_double = 0.;
static mut pango_font_green: ::core::ffi::c_double = 0.;
static mut pango_font_blue: ::core::ffi::c_double = 0.;
static mut pango_font_alpha: ::core::ffi::c_double = 0.;
unsafe extern "C" fn create_layout_with_dpi(mut cr: *mut cairo_t) -> *mut PangoLayout {
    let mut layout: *mut PangoLayout = ::core::ptr::null_mut::<PangoLayout>();
    let mut context: *mut PangoContext = ::core::ptr::null_mut::<PangoContext>();
    context = pango_cairo_create_context(cr);
    pango_cairo_context_set_resolution(context, get_dpi_value() as ::core::ffi::c_double);
    layout = pango_layout_new(context);
    g_object_unref(context as gpointer);
    return layout;
}
unsafe extern "C" fn load_pango_font(
    mut font: *mut i3Font,
    mut desc: *const ::core::ffi::c_char,
) -> bool {
    (*font).specific.pango_desc = pango_font_description_from_string(desc);
    if (*font).specific.pango_desc.is_null() {
        errorlog(
            b"[libi3] ERROR: Could not open font %s with Pango, fallback to X font.\n\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            desc,
        );
        return false_0 != 0;
    }
    verboselog(
        b"[libi3] ../libi3/font.c Using Pango font %s, size %d\n\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        pango_font_description_get_family((*font).specific.pango_desc),
        pango_font_description_get_size((*font).specific.pango_desc) as ::core::ffi::c_int
            / PANGO_SCALE,
    );
    root_visual_type = get_visualtype(root_screen);
    let mut surface: *mut cairo_surface_t = cairo_xcb_surface_create(
        conn,
        (*root_screen).root as xcb_drawable_t,
        root_visual_type,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
    );
    let mut cr: *mut cairo_t = cairo_create(surface);
    let mut layout: *mut PangoLayout = create_layout_with_dpi(cr);
    pango_layout_set_font_description(layout, (*font).specific.pango_desc);
    let mut height: gint = 0;
    pango_layout_get_pixel_size(
        layout,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        &raw mut height,
    );
    (*font).height = height as ::core::ffi::c_int;
    g_object_unref(layout as gpointer);
    cairo_destroy(cr);
    cairo_surface_destroy(surface);
    (*font).type_0 = FONT_TYPE_PANGO;
    return true_0 != 0;
}
unsafe extern "C" fn draw_text_pango(
    mut text: *const ::core::ffi::c_char,
    mut text_len: size_t,
    mut drawable: xcb_drawable_t,
    mut surface: *mut cairo_surface_t,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
    mut max_width: ::core::ffi::c_int,
    mut pango_markup: bool,
) {
    let mut cr: *mut cairo_t = cairo_create(surface);
    let mut layout: *mut PangoLayout = create_layout_with_dpi(cr);
    let mut height: gint = 0;
    pango_layout_set_font_description(layout, (*savedFont).specific.pango_desc);
    pango_layout_set_width(layout, max_width * PANGO_SCALE);
    pango_layout_set_wrap(layout, PANGO_WRAP_CHAR);
    pango_layout_set_ellipsize(layout, PANGO_ELLIPSIZE_END);
    pango_layout_set_single_paragraph_mode(layout, true_0);
    if pango_markup {
        pango_layout_set_markup(layout, text, text_len as ::core::ffi::c_int);
    } else {
        pango_layout_set_text(layout, text, text_len as ::core::ffi::c_int);
    }
    cairo_set_operator(cr, CAIRO_OPERATOR_SOURCE);
    cairo_set_source_rgba(
        cr,
        pango_font_red,
        pango_font_green,
        pango_font_blue,
        pango_font_alpha,
    );
    pango_cairo_update_layout(cr, layout);
    pango_layout_get_pixel_size(
        layout,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        &raw mut height,
    );
    let mut yoffset: ::core::ffi::c_int =
        (height as ::core::ffi::c_int - (*savedFont).height) / 2 as ::core::ffi::c_int;
    cairo_move_to(
        cr,
        x as ::core::ffi::c_double,
        (y - yoffset) as ::core::ffi::c_double,
    );
    pango_cairo_show_layout(cr, layout);
    g_object_unref(layout as gpointer);
    cairo_destroy(cr);
}
unsafe extern "C" fn predict_text_width_pango(
    mut text: *const ::core::ffi::c_char,
    mut text_len: size_t,
    mut pango_markup: bool,
) -> ::core::ffi::c_int {
    let mut surface: *mut cairo_surface_t = cairo_xcb_surface_create(
        conn,
        (*root_screen).root as xcb_drawable_t,
        root_visual_type,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
    );
    let mut cr: *mut cairo_t = cairo_create(surface);
    let mut layout: *mut PangoLayout = create_layout_with_dpi(cr);
    let mut width: gint = 0;
    pango_layout_set_font_description(layout, (*savedFont).specific.pango_desc);
    if pango_markup {
        pango_layout_set_markup(layout, text, text_len as ::core::ffi::c_int);
    } else {
        pango_layout_set_text(layout, text, text_len as ::core::ffi::c_int);
    }
    pango_cairo_update_layout(cr, layout);
    pango_layout_get_pixel_size(
        layout,
        &raw mut width,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    );
    g_object_unref(layout as gpointer);
    cairo_destroy(cr);
    cairo_surface_destroy(surface);
    return width as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn load_font(
    mut pattern: *const ::core::ffi::c_char,
    fallback: bool,
) -> i3Font {
    free_font();
    let mut font: i3Font = Font {
        type_0: FONT_TYPE_NONE,
        height: 0,
        pattern: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        specific: C2RustUnnamed {
            xcb: C2RustUnnamed_0 {
                id: 0,
                info: ::core::ptr::null_mut::<xcb_query_font_reply_t>(),
                table: ::core::ptr::null_mut::<xcb_charinfo_t>(),
            },
        },
    };
    font.type_0 = FONT_TYPE_NONE;
    font.pattern = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if conn.is_null() {
        return font;
    }
    if strlen(pattern) > strlen(b"pango:\0" as *const u8 as *const ::core::ffi::c_char)
        && strncmp(
            pattern,
            b"pango:\0" as *const u8 as *const ::core::ffi::c_char,
            strlen(b"pango:\0" as *const u8 as *const ::core::ffi::c_char),
        ) == 0
    {
        let mut font_pattern: *const ::core::ffi::c_char =
            pattern.offset(strlen(b"pango:\0" as *const u8 as *const ::core::ffi::c_char) as isize);
        if load_pango_font(&raw mut font, font_pattern) {
            font.pattern = sstrdup(pattern);
            return font;
        }
    } else if strlen(pattern) > strlen(b"xft:\0" as *const u8 as *const ::core::ffi::c_char)
        && strncmp(
            pattern,
            b"xft:\0" as *const u8 as *const ::core::ffi::c_char,
            strlen(b"xft:\0" as *const u8 as *const ::core::ffi::c_char),
        ) == 0
    {
        let mut font_pattern_0: *const ::core::ffi::c_char =
            pattern.offset(strlen(b"xft:\0" as *const u8 as *const ::core::ffi::c_char) as isize);
        if load_pango_font(&raw mut font, font_pattern_0) {
            font.pattern = sstrdup(pattern);
            return font;
        }
    }
    font.specific.xcb.id = xcb_generate_id(conn) as xcb_font_t;
    let mut font_cookie: xcb_void_cookie_t = xcb_open_font_checked(
        conn,
        font.specific.xcb.id,
        strlen(pattern) as uint16_t,
        pattern,
    );
    let mut info_cookie: xcb_query_font_cookie_t =
        xcb_query_font(conn, font.specific.xcb.id as xcb_fontable_t);
    let mut error: *mut xcb_generic_error_t = ::core::ptr::null_mut::<xcb_generic_error_t>();
    error = xcb_request_check(conn, font_cookie);
    if fallback as ::core::ffi::c_int != 0 && !error.is_null() {
        errorlog(
            b"[libi3] ERROR: Could not open font %s (X error %d). Trying fallback to 'fixed'.\n\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            pattern,
            (*error).error_code as ::core::ffi::c_int,
        );
        pattern = b"fixed\0" as *const u8 as *const ::core::ffi::c_char;
        font_cookie = xcb_open_font_checked(
            conn,
            font.specific.xcb.id,
            strlen(pattern) as uint16_t,
            pattern,
        );
        info_cookie = xcb_query_font(conn, font.specific.xcb.id as xcb_fontable_t);
        free(error as *mut ::core::ffi::c_void);
        error = xcb_request_check(conn, font_cookie);
        if !error.is_null() {
            errorlog(
                b"[libi3] ERROR: Could not open fallback font 'fixed', trying with '-misc-*'.\n\0"
                    as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
            pattern = b"-misc-*\0" as *const u8 as *const ::core::ffi::c_char;
            font_cookie = xcb_open_font_checked(
                conn,
                font.specific.xcb.id,
                strlen(pattern) as uint16_t,
                pattern,
            );
            info_cookie = xcb_query_font(conn, font.specific.xcb.id as xcb_fontable_t);
            free(error as *mut ::core::ffi::c_void);
            error = xcb_request_check(conn, font_cookie);
            if !error.is_null() {
                errx(
                    EXIT_FAILURE,
                    b"Could open neither requested font nor fallbacks (fixed or -misc-*): X11 error %d\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    (*error).error_code as ::core::ffi::c_int,
                );
            }
        }
    }
    free(error as *mut ::core::ffi::c_void);
    font.pattern = sstrdup(pattern);
    verboselog(
        b"[libi3] ../libi3/font.c Using X font %s\n\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        pattern,
    );
    font.specific.xcb.info = xcb_query_font_reply(
        conn,
        info_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if font.specific.xcb.info.is_null() {
        errx(
            EXIT_FAILURE,
            b"Could not load font \"%s\"\0" as *const u8 as *const ::core::ffi::c_char,
            pattern,
        );
    }
    if xcb_query_font_char_infos_length(font.specific.xcb.info) == 0 as ::core::ffi::c_int {
        font.specific.xcb.table = ::core::ptr::null_mut::<xcb_charinfo_t>();
    } else {
        font.specific.xcb.table = xcb_query_font_char_infos(font.specific.xcb.info);
    }
    font.height = (*font.specific.xcb.info).font_ascent as ::core::ffi::c_int
        + (*font.specific.xcb.info).font_descent as ::core::ffi::c_int;
    font.type_0 = FONT_TYPE_XCB;
    return font;
}
#[no_mangle]
pub unsafe extern "C" fn set_font(mut font: *mut i3Font) {
    savedFont = font;
}
#[no_mangle]
pub unsafe extern "C" fn free_font() {
    if savedFont.is_null() {
        return;
    }
    free((*savedFont).pattern as *mut ::core::ffi::c_void);
    match (*savedFont).type_0 as ::core::ffi::c_uint {
        1 => {
            xcb_close_font(conn, (*savedFont).specific.xcb.id);
            free((*savedFont).specific.xcb.info as *mut ::core::ffi::c_void);
        }
        2 => {
            pango_font_description_free((*savedFont).specific.pango_desc);
        }
        0 | _ => {}
    }
    savedFont = ::core::ptr::null::<i3Font>();
}
#[no_mangle]
pub unsafe extern "C" fn set_font_colors(
    mut gc: xcb_gcontext_t,
    mut foreground: color_t,
    mut background: color_t,
) {
    if savedFont != ::core::ptr::null_mut::<::core::ffi::c_void>() as *const i3Font {
    } else {
        __assert_fail(
            b"savedFont != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            b"../libi3/font.c\0" as *const u8 as *const ::core::ffi::c_char,
            296 as ::core::ffi::c_uint,
            b"void set_font_colors(xcb_gcontext_t, color_t, color_t)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    };
    match (*savedFont).type_0 as ::core::ffi::c_uint {
        1 => {
            let mut mask: uint32_t = (XCB_GC_FOREGROUND as ::core::ffi::c_int
                | XCB_GC_BACKGROUND as ::core::ffi::c_int
                | XCB_GC_FONT as ::core::ffi::c_int)
                as uint32_t;
            let mut values: [uint32_t; 3] = [
                foreground.colorpixel,
                background.colorpixel,
                (*savedFont).specific.xcb.id,
            ];
            xcb_change_gc(
                conn,
                gc,
                mask,
                &raw mut values as *mut uint32_t as *const ::core::ffi::c_void,
            );
        }
        2 => {
            pango_font_red = foreground.red;
            pango_font_green = foreground.green;
            pango_font_blue = foreground.blue;
            pango_font_alpha = foreground.alpha;
        }
        0 | _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn font_is_pango() -> bool {
    return (*savedFont).type_0 as ::core::ffi::c_uint
        == FONT_TYPE_PANGO as ::core::ffi::c_int as ::core::ffi::c_uint;
}
unsafe extern "C" fn draw_text_xcb(
    mut text: *const xcb_char2b_t,
    mut text_len: size_t,
    mut drawable: xcb_drawable_t,
    mut gc: xcb_gcontext_t,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
) {
    let mut pos_y: ::core::ffi::c_int =
        y + (*(*savedFont).specific.xcb.info).font_ascent as ::core::ffi::c_int;
    let mut offset: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    loop {
        let mut chunk_size: ::core::ffi::c_int = (if text_len > 255 as size_t {
            255 as size_t
        } else {
            text_len
        }) as ::core::ffi::c_int;
        let mut chunk: *const xcb_char2b_t = text.offset(offset as isize);
        xcb_image_text_16(
            conn,
            chunk_size as uint8_t,
            drawable,
            gc,
            x as int16_t,
            pos_y as int16_t,
            chunk,
        );
        offset += chunk_size;
        text_len = text_len.wrapping_sub(chunk_size as size_t);
        if text_len == 0 as size_t {
            break;
        }
        x += predict_text_width_xcb(chunk, chunk_size as size_t);
    }
}
#[no_mangle]
pub unsafe extern "C" fn draw_text(
    mut text: *mut i3String,
    mut drawable: xcb_drawable_t,
    mut gc: xcb_gcontext_t,
    mut surface: *mut cairo_surface_t,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
    mut max_width: ::core::ffi::c_int,
) {
    if savedFont != ::core::ptr::null_mut::<::core::ffi::c_void>() as *const i3Font {
    } else {
        __assert_fail(
            b"savedFont != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            b"../libi3/font.c\0" as *const u8 as *const ::core::ffi::c_char,
            369 as ::core::ffi::c_uint,
            b"void draw_text(i3String *, xcb_drawable_t, xcb_gcontext_t, cairo_surface_t *, int, int, int)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
    };
    match (*savedFont).type_0 as ::core::ffi::c_uint {
        0 => return,
        1 => {
            draw_text_xcb(
                i3string_as_ucs2(text),
                i3string_get_num_glyphs(text),
                drawable,
                gc,
                x,
                y,
            );
        }
        2 => {
            draw_text_pango(
                i3string_as_utf8(text),
                i3string_get_num_bytes(text),
                drawable,
                surface,
                x,
                y,
                max_width,
                i3string_is_markup(text),
            );
            return;
        }
        _ => {}
    };
}
unsafe extern "C" fn xcb_query_text_width(
    mut text: *const xcb_char2b_t,
    mut text_len: size_t,
) -> ::core::ffi::c_int {
    static mut first_invocation: bool = true_0 != 0;
    if first_invocation {
        fprintf(
            stderr,
            b"Using slow code path for text extents\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
        first_invocation = false_0 != 0;
    }
    let mut error: *mut xcb_generic_error_t = ::core::ptr::null_mut::<xcb_generic_error_t>();
    let mut cookie: xcb_query_text_extents_cookie_t = xcb_query_text_extents(
        conn,
        (*savedFont).specific.xcb.id as xcb_fontable_t,
        text_len as uint32_t,
        text as *mut xcb_char2b_t,
    );
    let mut reply: *mut xcb_query_text_extents_reply_t =
        xcb_query_text_extents_reply(conn, cookie, &raw mut error);
    if reply.is_null() {
        fprintf(
            stderr,
            b"Could not get text extents (X error code %d)\n\0" as *const u8
                as *const ::core::ffi::c_char,
            (*error).error_code as ::core::ffi::c_int,
        );
        free(error as *mut ::core::ffi::c_void);
        return ((*(*savedFont).specific.xcb.info).max_bounds.character_width as size_t)
            .wrapping_mul(text_len) as ::core::ffi::c_int;
    }
    let mut width: ::core::ffi::c_int = (*reply).overall_width as ::core::ffi::c_int;
    free(reply as *mut ::core::ffi::c_void);
    return width;
}
unsafe extern "C" fn predict_text_width_xcb(
    mut input: *const xcb_char2b_t,
    mut text_len: size_t,
) -> ::core::ffi::c_int {
    if text_len == 0 as size_t {
        return 0 as ::core::ffi::c_int;
    }
    let mut width: ::core::ffi::c_int = 0;
    if (*savedFont).specific.xcb.table.is_null() {
        width = xcb_query_text_width(input, text_len);
    } else {
        let mut font_info: *mut xcb_query_font_reply_t = (*savedFont).specific.xcb.info;
        let mut font_table: *mut xcb_charinfo_t = (*savedFont).specific.xcb.table;
        width = 0 as ::core::ffi::c_int;
        let mut i: size_t = 0 as size_t;
        while i < text_len {
            let mut info: *mut xcb_charinfo_t = ::core::ptr::null_mut::<xcb_charinfo_t>();
            let mut row: ::core::ffi::c_int =
                (*input.offset(i as isize)).byte1 as ::core::ffi::c_int;
            let mut col: ::core::ffi::c_int =
                (*input.offset(i as isize)).byte2 as ::core::ffi::c_int;
            if !(row < (*font_info).min_byte1 as ::core::ffi::c_int
                || row > (*font_info).max_byte1 as ::core::ffi::c_int
                || col < (*font_info).min_char_or_byte2 as ::core::ffi::c_int
                || col > (*font_info).max_char_or_byte2 as ::core::ffi::c_int)
            {
                info = font_table.offset(
                    ((row - (*font_info).min_byte1 as ::core::ffi::c_int)
                        * ((*font_info).max_char_or_byte2 as ::core::ffi::c_int
                            - (*font_info).min_char_or_byte2 as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int)
                        + (col - (*font_info).min_char_or_byte2 as ::core::ffi::c_int))
                        as isize,
                ) as *mut xcb_charinfo_t;
                if (*info).character_width as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                    || (*info).right_side_bearing as ::core::ffi::c_int
                        | (*info).left_side_bearing as ::core::ffi::c_int
                        | (*info).ascent as ::core::ffi::c_int
                        | (*info).descent as ::core::ffi::c_int
                        != 0 as ::core::ffi::c_int
                {
                    width += (*info).character_width as ::core::ffi::c_int;
                }
            }
            i = i.wrapping_add(1);
        }
    }
    return width;
}
#[no_mangle]
pub unsafe extern "C" fn predict_text_width(mut text: *mut i3String) -> ::core::ffi::c_int {
    if savedFont != ::core::ptr::null_mut::<::core::ffi::c_void>() as *const i3Font {
    } else {
        __assert_fail(
            b"savedFont != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            b"../libi3/font.c\0" as *const u8 as *const ::core::ffi::c_char,
            467 as ::core::ffi::c_uint,
            b"int predict_text_width(i3String *)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
    match (*savedFont).type_0 as ::core::ffi::c_uint {
        0 => return 0 as ::core::ffi::c_int,
        1 => {
            return predict_text_width_xcb(i3string_as_ucs2(text), i3string_get_num_glyphs(text));
        }
        2 => {
            return predict_text_width_pango(
                i3string_as_utf8(text),
                i3string_get_num_bytes(text),
                i3string_is_markup(text),
            );
        }
        _ => {}
    }
    if 0 as ::core::ffi::c_int != 0 {
    } else {
        __assert_fail(
            b"false\0" as *const u8 as *const ::core::ffi::c_char,
            b"../libi3/font.c\0" as *const u8 as *const ::core::ffi::c_char,
            480 as ::core::ffi::c_uint,
            b"int predict_text_width(i3String *)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
    panic!("Reached end of non-void function without returning");
}
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
