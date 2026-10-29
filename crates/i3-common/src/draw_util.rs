/*
 * vim:ts=4:sw=4:expandtab
 *
 * © 2015 Ingo Bürk and contributors (see also: LICENSE)
 *
 * draw.c: Utility for drawing.
 *
 */
// Mechanical file-by-file Rust translation of libi3/draw_util.c.
// Generated with C2Rust 0.22.1; C ABI and original ownership retained.
extern "C" {
    pub type xcb_connection_t;
    pub type _cairo;
    pub type _cairo_surface;
    pub type _i3String;
    fn xcb_visualtype_next(i: *mut xcb_visualtype_iterator_t);
    fn xcb_depth_visuals_iterator(R: *const xcb_depth_t) -> xcb_visualtype_iterator_t;
    fn xcb_depth_next(i: *mut xcb_depth_iterator_t);
    fn xcb_screen_allowed_depths_iterator(R: *const xcb_screen_t) -> xcb_depth_iterator_t;
    fn xcb_create_gc_checked(
        c: *mut xcb_connection_t,
        cid: xcb_gcontext_t,
        drawable: xcb_drawable_t,
        value_mask: uint32_t,
        value_list: *const ::core::ffi::c_void,
    ) -> xcb_void_cookie_t;
    fn xcb_free_gc(c: *mut xcb_connection_t, gc: xcb_gcontext_t) -> xcb_void_cookie_t;
    fn xcb_request_check(
        c: *mut xcb_connection_t,
        cookie: xcb_void_cookie_t,
    ) -> *mut xcb_generic_error_t;
    fn xcb_generate_id(c: *mut xcb_connection_t) -> uint32_t;
    fn cairo_create(target: *mut cairo_surface_t) -> *mut cairo_t;
    fn cairo_destroy(cr: *mut cairo_t);
    fn cairo_save(cr: *mut cairo_t);
    fn cairo_restore(cr: *mut cairo_t);
    fn cairo_set_operator(cr: *mut cairo_t, op: cairo_operator_t);
    fn cairo_set_source_rgba(
        cr: *mut cairo_t,
        red: ::core::ffi::c_double,
        green: ::core::ffi::c_double,
        blue: ::core::ffi::c_double,
        alpha: ::core::ffi::c_double,
    );
    fn cairo_set_source_surface(
        cr: *mut cairo_t,
        surface: *mut cairo_surface_t,
        x: ::core::ffi::c_double,
        y: ::core::ffi::c_double,
    );
    fn cairo_translate(cr: *mut cairo_t, tx: ::core::ffi::c_double, ty: ::core::ffi::c_double);
    fn cairo_scale(cr: *mut cairo_t, sx: ::core::ffi::c_double, sy: ::core::ffi::c_double);
    fn cairo_rectangle(
        cr: *mut cairo_t,
        x: ::core::ffi::c_double,
        y: ::core::ffi::c_double,
        width: ::core::ffi::c_double,
        height: ::core::ffi::c_double,
    );
    fn cairo_paint(cr: *mut cairo_t);
    fn cairo_fill(cr: *mut cairo_t);
    fn cairo_status(cr: *mut cairo_t) -> cairo_status_t;
    fn cairo_status_to_string(status: cairo_status_t) -> *const ::core::ffi::c_char;
    fn cairo_surface_destroy(surface: *mut cairo_surface_t);
    fn cairo_surface_flush(surface: *mut cairo_surface_t);
    fn cairo_surface_mark_dirty(surface: *mut cairo_surface_t);
    fn cairo_image_surface_get_width(surface: *mut cairo_surface_t) -> ::core::ffi::c_int;
    fn cairo_image_surface_get_height(surface: *mut cairo_surface_t) -> ::core::ffi::c_int;
    fn cairo_xcb_surface_create(
        connection: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        visual: *mut xcb_visualtype_t,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
    ) -> *mut cairo_surface_t;
    fn cairo_xcb_surface_set_size(
        surface: *mut cairo_surface_t,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
    );
    static mut root_screen: *mut xcb_screen_t;
    fn verboselog(fmt: *mut ::core::ffi::c_char, ...);
    fn errorlog(fmt: *mut ::core::ffi::c_char, ...);
    fn get_colorpixel(hex: *const ::core::ffi::c_char) -> uint32_t;
    fn set_font_colors(gc: xcb_gcontext_t, foreground: color_t, background: color_t);
    fn draw_text(
        text: *mut i3String,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        surface: *mut cairo_surface_t,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
        max_width: ::core::ffi::c_int,
    );
    static mut visual_type: *mut xcb_visualtype_t;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
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
pub type xcb_gcontext_t = uint32_t;
pub type xcb_colormap_t = uint32_t;
pub type xcb_drawable_t = uint32_t;
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
pub struct xcb_visualtype_iterator_t {
    pub data: *mut xcb_visualtype_t,
    pub rem: ::core::ffi::c_int,
    pub index: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_depth_t {
    pub depth: uint8_t,
    pub pad0: uint8_t,
    pub visuals_len: uint16_t,
    pub pad1: [uint8_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_depth_iterator_t {
    pub data: *mut xcb_depth_t,
    pub rem: ::core::ffi::c_int,
    pub index: ::core::ffi::c_int,
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
pub type cairo_t = _cairo;
pub type cairo_surface_t = _cairo_surface;
pub type _cairo_status = ::core::ffi::c_uint;
pub const CAIRO_STATUS_LAST_STATUS: _cairo_status = 45;
pub const CAIRO_STATUS_SVG_FONT_ERROR: _cairo_status = 44;
pub const CAIRO_STATUS_DWRITE_ERROR: _cairo_status = 43;
pub const CAIRO_STATUS_TAG_ERROR: _cairo_status = 42;
pub const CAIRO_STATUS_WIN32_GDI_ERROR: _cairo_status = 41;
pub const CAIRO_STATUS_FREETYPE_ERROR: _cairo_status = 40;
pub const CAIRO_STATUS_PNG_ERROR: _cairo_status = 39;
pub const CAIRO_STATUS_JBIG2_GLOBAL_MISSING: _cairo_status = 38;
pub const CAIRO_STATUS_DEVICE_FINISHED: _cairo_status = 37;
pub const CAIRO_STATUS_INVALID_MESH_CONSTRUCTION: _cairo_status = 36;
pub const CAIRO_STATUS_DEVICE_ERROR: _cairo_status = 35;
pub const CAIRO_STATUS_DEVICE_TYPE_MISMATCH: _cairo_status = 34;
pub const CAIRO_STATUS_USER_FONT_NOT_IMPLEMENTED: _cairo_status = 33;
pub const CAIRO_STATUS_INVALID_SIZE: _cairo_status = 32;
pub const CAIRO_STATUS_INVALID_WEIGHT: _cairo_status = 31;
pub const CAIRO_STATUS_INVALID_SLANT: _cairo_status = 30;
pub const CAIRO_STATUS_INVALID_CLUSTERS: _cairo_status = 29;
pub const CAIRO_STATUS_NEGATIVE_COUNT: _cairo_status = 28;
pub const CAIRO_STATUS_USER_FONT_ERROR: _cairo_status = 27;
pub const CAIRO_STATUS_USER_FONT_IMMUTABLE: _cairo_status = 26;
pub const CAIRO_STATUS_FONT_TYPE_MISMATCH: _cairo_status = 25;
pub const CAIRO_STATUS_INVALID_STRIDE: _cairo_status = 24;
pub const CAIRO_STATUS_TEMP_FILE_ERROR: _cairo_status = 23;
pub const CAIRO_STATUS_CLIP_NOT_REPRESENTABLE: _cairo_status = 22;
pub const CAIRO_STATUS_INVALID_INDEX: _cairo_status = 21;
pub const CAIRO_STATUS_INVALID_DSC_COMMENT: _cairo_status = 20;
pub const CAIRO_STATUS_INVALID_DASH: _cairo_status = 19;
pub const CAIRO_STATUS_FILE_NOT_FOUND: _cairo_status = 18;
pub const CAIRO_STATUS_INVALID_VISUAL: _cairo_status = 17;
pub const CAIRO_STATUS_INVALID_FORMAT: _cairo_status = 16;
pub const CAIRO_STATUS_INVALID_CONTENT: _cairo_status = 15;
pub const CAIRO_STATUS_PATTERN_TYPE_MISMATCH: _cairo_status = 14;
pub const CAIRO_STATUS_SURFACE_TYPE_MISMATCH: _cairo_status = 13;
pub const CAIRO_STATUS_SURFACE_FINISHED: _cairo_status = 12;
pub const CAIRO_STATUS_WRITE_ERROR: _cairo_status = 11;
pub const CAIRO_STATUS_READ_ERROR: _cairo_status = 10;
pub const CAIRO_STATUS_INVALID_PATH_DATA: _cairo_status = 9;
pub const CAIRO_STATUS_INVALID_STRING: _cairo_status = 8;
pub const CAIRO_STATUS_NULL_POINTER: _cairo_status = 7;
pub const CAIRO_STATUS_INVALID_STATUS: _cairo_status = 6;
pub const CAIRO_STATUS_INVALID_MATRIX: _cairo_status = 5;
pub const CAIRO_STATUS_NO_CURRENT_POINT: _cairo_status = 4;
pub const CAIRO_STATUS_INVALID_POP_GROUP: _cairo_status = 3;
pub const CAIRO_STATUS_INVALID_RESTORE: _cairo_status = 2;
pub const CAIRO_STATUS_NO_MEMORY: _cairo_status = 1;
pub const CAIRO_STATUS_SUCCESS: _cairo_status = 0;
pub type cairo_status_t = _cairo_status;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub depth: uint8_t,
    pub gc: xcb_gcontext_t,
}
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const XCB_NONE: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
unsafe extern "C" fn surface_initialized(mut surface: *mut surface_t) -> bool {
    if (*surface).id as ::core::ffi::c_long == XCB_NONE {
        errorlog(
            b"[libi3] ERROR: Surface %p is not initialized, skipping drawing.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            surface,
        );
        return false_0 != 0;
    }
    return true_0 != 0;
}
unsafe extern "C" fn get_gc(
    mut conn: *mut xcb_connection_t,
    mut depth: uint8_t,
    mut drawable: xcb_drawable_t,
    mut should_free: *mut bool,
) -> xcb_gcontext_t {
    static mut gc_cache: [C2RustUnnamed; 2] = [
        C2RustUnnamed {
            depth: 0 as uint8_t,
            gc: 0,
        },
        C2RustUnnamed { depth: 0, gc: 0 },
    ];
    let mut index: size_t = 0 as size_t;
    let mut cache: bool = false_0 != 0;
    *should_free = false_0 != 0;
    while index
        < (::core::mem::size_of::<[C2RustUnnamed; 2]>() as usize)
            .wrapping_div(::core::mem::size_of::<C2RustUnnamed>() as usize)
    {
        if gc_cache[index as usize].depth as ::core::ffi::c_int == depth as ::core::ffi::c_int {
            return gc_cache[index as usize].gc;
        }
        if gc_cache[index as usize].depth as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            cache = true_0 != 0;
            break;
        } else {
            index = index.wrapping_add(1);
        }
    }
    let mut gc: xcb_gcontext_t = xcb_generate_id(conn) as xcb_gcontext_t;
    let mut gc_cookie: xcb_void_cookie_t = xcb_create_gc_checked(
        conn,
        gc,
        drawable,
        0 as uint32_t,
        ::core::ptr::null::<::core::ffi::c_void>(),
    );
    let mut error: *mut xcb_generic_error_t = xcb_request_check(conn, gc_cookie);
    if !error.is_null() {
        errorlog(
            b"[libi3] ERROR: Could not create graphical context. Error code: %d. Please report this bug.\n\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            (*error).error_code as ::core::ffi::c_int,
        );
        free(error as *mut ::core::ffi::c_void);
        return gc;
    }
    if cache {
        gc_cache[index as usize].depth = depth;
        gc_cache[index as usize].gc = gc;
    } else {
        *should_free = true_0 != 0;
    }
    return gc;
}
#[no_mangle]
pub unsafe extern "C" fn get_visual_depth(visual_id: xcb_visualid_t) -> uint16_t {
    let mut depth_iter: xcb_depth_iterator_t = xcb_depth_iterator_t {
        data: ::core::ptr::null_mut::<xcb_depth_t>(),
        rem: 0,
        index: 0,
    };
    depth_iter = xcb_screen_allowed_depths_iterator(root_screen);
    while depth_iter.rem != 0 {
        let mut visual_iter: xcb_visualtype_iterator_t = xcb_visualtype_iterator_t {
            data: ::core::ptr::null_mut::<xcb_visualtype_t>(),
            rem: 0,
            index: 0,
        };
        visual_iter = xcb_depth_visuals_iterator(depth_iter.data);
        while visual_iter.rem != 0 {
            if visual_id == (*visual_iter.data).visual_id {
                return (*depth_iter.data).depth as uint16_t;
            }
            xcb_visualtype_next(&raw mut visual_iter);
        }
        xcb_depth_next(&raw mut depth_iter);
    }
    return 0 as uint16_t;
}
#[no_mangle]
pub unsafe extern "C" fn draw_util_surface_init(
    mut conn: *mut xcb_connection_t,
    mut surface: *mut surface_t,
    mut drawable: xcb_drawable_t,
    mut visual: *mut xcb_visualtype_t,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
) {
    (*surface).id = drawable;
    (*surface).width = width;
    (*surface).height = height;
    if visual.is_null() {
        visual = visual_type;
    }
    (*surface).gc = get_gc(
        conn,
        get_visual_depth((*visual).visual_id) as uint8_t,
        drawable,
        &raw mut (*surface).owns_gc,
    );
    (*surface).surface = cairo_xcb_surface_create(conn, (*surface).id, visual, width, height);
    (*surface).cr = cairo_create((*surface).surface);
}
#[no_mangle]
pub unsafe extern "C" fn draw_util_surface_free(
    mut conn: *mut xcb_connection_t,
    mut surface: *mut surface_t,
) {
    let mut status: cairo_status_t = CAIRO_STATUS_SUCCESS;
    if !(*surface).cr.is_null() {
        status = cairo_status((*surface).cr);
    }
    if status as ::core::ffi::c_uint
        != CAIRO_STATUS_SUCCESS as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        verboselog(
            b"[libi3] ../libi3/draw_util.c Found cairo context in an error status while freeing, error %d is %s\0"
                as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            status as ::core::ffi::c_uint,
            cairo_status_to_string(status),
        );
    }
    if (*surface).owns_gc {
        xcb_free_gc(conn, (*surface).gc);
    }
    cairo_surface_destroy((*surface).surface);
    cairo_destroy((*surface).cr);
    (*surface).surface = ::core::ptr::null_mut::<cairo_surface_t>();
    (*surface).cr = ::core::ptr::null_mut::<cairo_t>();
}
#[no_mangle]
pub unsafe extern "C" fn draw_util_surface_set_size(
    mut surface: *mut surface_t,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
) {
    (*surface).width = width;
    (*surface).height = height;
    cairo_xcb_surface_set_size((*surface).surface, width, height);
}
#[no_mangle]
pub unsafe extern "C" fn draw_util_hex_to_color(mut color: *const ::core::ffi::c_char) -> color_t {
    if strlen(color) < 6 as size_t
        || *color.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '#' as i32
    {
        errorlog(
            b"[libi3] ERROR: Could not parse color: %s\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            color,
        );
        return draw_util_hex_to_color(b"#A9A9A9\0" as *const u8 as *const ::core::ffi::c_char);
    }
    let mut alpha: [::core::ffi::c_char; 2] = [0; 2];
    if strlen(color) == strlen(b"#rrggbbaa\0" as *const u8 as *const ::core::ffi::c_char) {
        alpha[0 as ::core::ffi::c_int as usize] = *color.offset(7 as ::core::ffi::c_int as isize);
        alpha[1 as ::core::ffi::c_int as usize] = *color.offset(8 as ::core::ffi::c_int as isize);
    } else {
        alpha[1 as ::core::ffi::c_int as usize] = 'F' as i32 as ::core::ffi::c_char;
        alpha[0 as ::core::ffi::c_int as usize] = alpha[1 as ::core::ffi::c_int as usize];
    }
    let mut groups: [[::core::ffi::c_char; 3]; 4] = [
        [
            *color.offset(1 as ::core::ffi::c_int as isize),
            *color.offset(2 as ::core::ffi::c_int as isize),
            '\0' as i32 as ::core::ffi::c_char,
        ],
        [
            *color.offset(3 as ::core::ffi::c_int as isize),
            *color.offset(4 as ::core::ffi::c_int as isize),
            '\0' as i32 as ::core::ffi::c_char,
        ],
        [
            *color.offset(5 as ::core::ffi::c_int as isize),
            *color.offset(6 as ::core::ffi::c_int as isize),
            '\0' as i32 as ::core::ffi::c_char,
        ],
        [
            alpha[0 as ::core::ffi::c_int as usize],
            alpha[1 as ::core::ffi::c_int as usize],
            '\0' as i32 as ::core::ffi::c_char,
        ],
    ];
    return color_t {
        red: strtol(
            &raw mut *(&raw mut groups as *mut [::core::ffi::c_char; 3])
                .offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            16 as ::core::ffi::c_int,
        ) as ::core::ffi::c_double
            / 255.0f64,
        green: strtol(
            &raw mut *(&raw mut groups as *mut [::core::ffi::c_char; 3])
                .offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            16 as ::core::ffi::c_int,
        ) as ::core::ffi::c_double
            / 255.0f64,
        blue: strtol(
            &raw mut *(&raw mut groups as *mut [::core::ffi::c_char; 3])
                .offset(2 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            16 as ::core::ffi::c_int,
        ) as ::core::ffi::c_double
            / 255.0f64,
        alpha: strtol(
            &raw mut *(&raw mut groups as *mut [::core::ffi::c_char; 3])
                .offset(3 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            16 as ::core::ffi::c_int,
        ) as ::core::ffi::c_double
            / 255.0f64,
        colorpixel: get_colorpixel(color),
    };
}
unsafe extern "C" fn draw_util_set_source_color(mut surface: *mut surface_t, mut color: color_t) {
    if !surface_initialized(surface) {
        return;
    }
    cairo_set_source_rgba(
        (*surface).cr,
        color.red,
        color.green,
        color.blue,
        color.alpha,
    );
}
#[no_mangle]
pub unsafe extern "C" fn draw_util_text(
    mut text: *mut i3String,
    mut surface: *mut surface_t,
    mut fg_color: color_t,
    mut bg_color: color_t,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
    mut max_width: ::core::ffi::c_int,
) {
    if !surface_initialized(surface) {
        return;
    }
    cairo_surface_flush((*surface).surface);
    cairo_surface_flush((*surface).surface);
    set_font_colors((*surface).gc, fg_color, bg_color);
    draw_text(
        text,
        (*surface).id,
        (*surface).gc,
        (*surface).surface,
        x,
        y,
        max_width,
    );
    cairo_surface_mark_dirty((*surface).surface);
}
#[no_mangle]
pub unsafe extern "C" fn draw_util_image(
    mut image: *mut cairo_surface_t,
    mut surface: *mut surface_t,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
) {
    if !surface_initialized(surface) {
        return;
    }
    cairo_save((*surface).cr);
    cairo_translate(
        (*surface).cr,
        x as ::core::ffi::c_double,
        y as ::core::ffi::c_double,
    );
    let src_width: ::core::ffi::c_int = cairo_image_surface_get_width(image) as ::core::ffi::c_int;
    let src_height: ::core::ffi::c_int =
        cairo_image_surface_get_height(image) as ::core::ffi::c_int;
    let mut scale: ::core::ffi::c_double = if (width as ::core::ffi::c_double
        / src_width as ::core::ffi::c_double)
        < height as ::core::ffi::c_double / src_height as ::core::ffi::c_double
    {
        width as ::core::ffi::c_double / src_width as ::core::ffi::c_double
    } else {
        height as ::core::ffi::c_double / src_height as ::core::ffi::c_double
    };
    cairo_scale((*surface).cr, scale, scale);
    cairo_set_source_surface(
        (*surface).cr,
        image,
        0 as ::core::ffi::c_int as ::core::ffi::c_double,
        0 as ::core::ffi::c_int as ::core::ffi::c_double,
    );
    cairo_paint((*surface).cr);
    cairo_restore((*surface).cr);
}
#[no_mangle]
pub unsafe extern "C" fn draw_util_rectangle(
    mut surface: *mut surface_t,
    mut color: color_t,
    mut x: ::core::ffi::c_double,
    mut y: ::core::ffi::c_double,
    mut w: ::core::ffi::c_double,
    mut h: ::core::ffi::c_double,
) {
    if !surface_initialized(surface) {
        return;
    }
    cairo_save((*surface).cr);
    cairo_set_operator((*surface).cr, CAIRO_OPERATOR_SOURCE);
    draw_util_set_source_color(surface, color);
    cairo_rectangle((*surface).cr, x, y, w, h);
    cairo_fill((*surface).cr);
    cairo_surface_flush((*surface).surface);
    cairo_surface_flush((*surface).surface);
    cairo_restore((*surface).cr);
}
#[no_mangle]
pub unsafe extern "C" fn draw_util_clear_surface(mut surface: *mut surface_t, mut color: color_t) {
    if !surface_initialized(surface) {
        return;
    }
    cairo_save((*surface).cr);
    cairo_set_operator((*surface).cr, CAIRO_OPERATOR_SOURCE);
    draw_util_set_source_color(surface, color);
    cairo_paint((*surface).cr);
    cairo_surface_flush((*surface).surface);
    cairo_surface_flush((*surface).surface);
    cairo_restore((*surface).cr);
}
#[no_mangle]
pub unsafe extern "C" fn draw_util_copy_surface(
    mut src: *mut surface_t,
    mut dest: *mut surface_t,
    mut src_x: ::core::ffi::c_double,
    mut src_y: ::core::ffi::c_double,
    mut dest_x: ::core::ffi::c_double,
    mut dest_y: ::core::ffi::c_double,
    mut width: ::core::ffi::c_double,
    mut height: ::core::ffi::c_double,
) {
    if !surface_initialized(src) || !surface_initialized(dest) {
        return;
    }
    cairo_save((*dest).cr);
    cairo_set_operator((*dest).cr, CAIRO_OPERATOR_SOURCE);
    cairo_set_source_surface((*dest).cr, (*src).surface, dest_x - src_x, dest_y - src_y);
    cairo_rectangle((*dest).cr, dest_x, dest_y, width, height);
    cairo_fill((*dest).cr);
    cairo_surface_flush((*src).surface);
    cairo_surface_flush((*src).surface);
    cairo_surface_flush((*dest).surface);
    cairo_surface_flush((*dest).surface);
    cairo_restore((*dest).cr);
}
