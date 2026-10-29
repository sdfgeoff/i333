/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 */
// Mechanical file-by-file Rust translation of libi3/font.c.
// Originally translated with C2Rust 0.22.1; subsequently maintained in Rust.
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

pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;

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

pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
static mut savedFont: *const i3Font = ::core::ptr::null::<i3Font>();
static mut root_visual_type: *mut xcb_visualtype_t =
    ::core::ptr::null::<xcb_visualtype_t>() as *mut xcb_visualtype_t;
static mut pango_font_red: ::core::ffi::c_double = 0.;
static mut pango_font_green: ::core::ffi::c_double = 0.;
static mut pango_font_blue: ::core::ffi::c_double = 0.;
static mut pango_font_alpha: ::core::ffi::c_double = 0.;
unsafe fn create_layout_with_dpi(cr: *mut cairo_t) -> *mut PangoLayout {
    let mut layout: *mut PangoLayout = ::core::ptr::null_mut::<PangoLayout>();
    let mut context: *mut PangoContext = ::core::ptr::null_mut::<PangoContext>();
    context = pango_cairo_create_context(cr);
    pango_cairo_context_set_resolution(context, get_dpi_value() as ::core::ffi::c_double);
    layout = pango_layout_new(context);
    g_object_unref(context.cast());
    layout
}
unsafe fn load_pango_font(font: *mut i3Font, desc: *const ::core::ffi::c_char) -> bool {
    let crate::display::DisplayContext {
        connection: conn,
        root_screen,
        ..
    } = crate::display::context();

    (*font).specific.pango_desc = pango_font_description_from_string(desc);
    if (*font).specific.pango_desc.is_null() {
        errorlog(
            c"[libi3] ERROR: Could not open font %s with Pango, fallback to X font.\n"
                .as_ptr()
                .cast_mut(),
            desc,
        );
        return false_0 != 0;
    }
    verboselog(
        c"[libi3] ../libi3/font.c Using Pango font %s, size %d\n"
            .as_ptr()
            .cast_mut(),
        pango_font_description_get_family((*font).specific.pango_desc),
        pango_font_description_get_size((*font).specific.pango_desc) as ::core::ffi::c_int
            / PANGO_SCALE,
    );
    root_visual_type = get_visualtype(root_screen);
    let surface: *mut cairo_surface_t = cairo_xcb_surface_create(
        (conn).cast(),
        (*root_screen).root as xcb_drawable_t,
        (root_visual_type).cast(),
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
    );
    let cr: *mut cairo_t = cairo_create(surface);
    let layout: *mut PangoLayout = create_layout_with_dpi(cr);
    pango_layout_set_font_description(layout, (*font).specific.pango_desc);
    let mut height: ::core::ffi::c_int = 0;
    pango_layout_get_pixel_size(
        layout,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        &raw mut height,
    );
    (*font).height = height as ::core::ffi::c_int;
    g_object_unref(layout.cast());
    cairo_destroy(cr);
    cairo_surface_destroy(surface);
    (*font).type_0 = FONT_TYPE_PANGO;
    true_0 != 0
}
unsafe fn draw_text_pango(
    text: *const ::core::ffi::c_char,
    text_len: size_t,
    drawable: xcb_drawable_t,
    surface: *mut cairo_surface_t,
    x: ::core::ffi::c_int,
    y: ::core::ffi::c_int,
    max_width: ::core::ffi::c_int,
    pango_markup: bool,
) {
    let cr: *mut cairo_t = cairo_create(surface);
    let layout: *mut PangoLayout = create_layout_with_dpi(cr);
    let mut height: ::core::ffi::c_int = 0;
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
    cairo_set_operator(cr, OPERATOR_SOURCE);
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
    let yoffset: ::core::ffi::c_int =
        (height as ::core::ffi::c_int - (*savedFont).height) / 2 as ::core::ffi::c_int;
    cairo_move_to(
        cr,
        x as ::core::ffi::c_double,
        (y - yoffset) as ::core::ffi::c_double,
    );
    pango_cairo_show_layout(cr, layout);
    g_object_unref(layout.cast());
    cairo_destroy(cr);
}
unsafe fn predict_text_width_pango(
    text: *const ::core::ffi::c_char,
    text_len: size_t,
    pango_markup: bool,
) -> ::core::ffi::c_int {
    let crate::display::DisplayContext {
        connection: conn,
        root_screen,
        ..
    } = crate::display::context();

    let surface: *mut cairo_surface_t = cairo_xcb_surface_create(
        (conn).cast(),
        (*root_screen).root as xcb_drawable_t,
        (root_visual_type).cast(),
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
    );
    let cr: *mut cairo_t = cairo_create(surface);
    let layout: *mut PangoLayout = create_layout_with_dpi(cr);
    let mut width: ::core::ffi::c_int = 0;
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
    g_object_unref(layout.cast());
    cairo_destroy(cr);
    cairo_surface_destroy(surface);
    width as ::core::ffi::c_int
}

pub unsafe fn load_font(mut pattern: *const ::core::ffi::c_char, fallback: bool) -> i3Font {
    let crate::display::DisplayContext {
        connection: conn, ..
    } = crate::display::context();

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
    if strlen(pattern) > strlen(c"pango:".as_ptr())
        && strncmp(pattern, c"pango:".as_ptr(), strlen(c"pango:".as_ptr())) == 0
    {
        let font_pattern: *const ::core::ffi::c_char =
            pattern.offset(strlen(c"pango:".as_ptr()) as isize);
        if load_pango_font(&raw mut font, font_pattern) {
            font.pattern = sstrdup(pattern);
            return font;
        }
    } else if strlen(pattern) > strlen(c"xft:".as_ptr())
        && strncmp(pattern, c"xft:".as_ptr(), strlen(c"xft:".as_ptr())) == 0
    {
        let font_pattern_0: *const ::core::ffi::c_char =
            pattern.offset(strlen(c"xft:".as_ptr()) as isize);
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
            c"[libi3] ERROR: Could not open font %s (X error %d). Trying fallback to 'fixed'.\n"
                .as_ptr()
                .cast_mut(),
            pattern,
            (*error).error_code as ::core::ffi::c_int,
        );
        pattern = c"fixed".as_ptr();
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
                c"[libi3] ERROR: Could not open fallback font 'fixed', trying with '-misc-*'.\n"
                    .as_ptr()
                    .cast_mut(),
            );
            pattern = c"-misc-*".as_ptr();
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
                    c"Could open neither requested font nor fallbacks (fixed or -misc-*): X11 error %d".as_ptr(),
                    (*error).error_code as ::core::ffi::c_int,
                );
            }
        }
    }
    free(error as *mut ::core::ffi::c_void);
    font.pattern = sstrdup(pattern);
    verboselog(
        c"[libi3] ../libi3/font.c Using X font %s\n"
            .as_ptr()
            .cast_mut(),
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
            c"Could not load font \"%s\"".as_ptr(),
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
    font
}

pub unsafe fn set_font(font: *mut i3Font) {
    savedFont = font;
}

pub unsafe fn free_font() {
    let crate::display::DisplayContext {
        connection: conn, ..
    } = crate::display::context();

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

pub unsafe fn set_font_colors(gc: xcb_gcontext_t, foreground: color_t, background: color_t) {
    let crate::display::DisplayContext {
        connection: conn, ..
    } = crate::display::context();

    if savedFont != ::core::ptr::null_mut::<::core::ffi::c_void>() as *const i3Font {
    } else {
        __assert_fail(
            c"savedFont != NULL".as_ptr(),
            c"../libi3/font.c".as_ptr(),
            296 as ::core::ffi::c_uint,
            c"void set_font_colors(xcb_gcontext_t, color_t, color_t)".as_ptr(),
        );
    };
    match (*savedFont).type_0 as ::core::ffi::c_uint {
        1 => {
            let mask: uint32_t = (XCB_GC_FOREGROUND as ::core::ffi::c_int
                | XCB_GC_BACKGROUND as ::core::ffi::c_int
                | XCB_GC_FONT as ::core::ffi::c_int) as uint32_t;
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

pub unsafe fn font_is_pango() -> bool {
    (*savedFont).type_0 as ::core::ffi::c_uint
        == FONT_TYPE_PANGO as ::core::ffi::c_int as ::core::ffi::c_uint
}
unsafe fn draw_text_xcb(
    text: *const xcb_char2b_t,
    mut text_len: size_t,
    drawable: xcb_drawable_t,
    gc: xcb_gcontext_t,
    mut x: ::core::ffi::c_int,
    y: ::core::ffi::c_int,
) {
    let crate::display::DisplayContext {
        connection: conn, ..
    } = crate::display::context();

    let pos_y: ::core::ffi::c_int =
        y + (*(*savedFont).specific.xcb.info).font_ascent as ::core::ffi::c_int;
    let mut offset: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    loop {
        let chunk_size: ::core::ffi::c_int = (if text_len > 255 as size_t {
            255 as size_t
        } else {
            text_len
        }) as ::core::ffi::c_int;
        let chunk: *const xcb_char2b_t = text.offset(offset as isize);
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

pub unsafe fn draw_text(
    text: *mut i3String,
    drawable: xcb_drawable_t,
    gc: xcb_gcontext_t,
    surface: *mut cairo_surface_t,
    x: ::core::ffi::c_int,
    y: ::core::ffi::c_int,
    max_width: ::core::ffi::c_int,
) {
    if savedFont != ::core::ptr::null_mut::<::core::ffi::c_void>() as *const i3Font {
    } else {
        __assert_fail(
            c"savedFont != NULL".as_ptr(),
            c"../libi3/font.c".as_ptr(),
            369 as ::core::ffi::c_uint,
            c"void draw_text(i3String *, xcb_drawable_t, xcb_gcontext_t, cairo_surface_t *, int, int, int)".as_ptr(),
        );
    };
    match (*savedFont).type_0 as ::core::ffi::c_uint {
        0 => (),
        1 => {
            draw_text_xcb(
                i3string_as_ucs2(text).cast(),
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
        }
        _ => {}
    }
}
unsafe fn xcb_query_text_width(text: *const xcb_char2b_t, text_len: size_t) -> ::core::ffi::c_int {
    let crate::display::DisplayContext {
        connection: conn, ..
    } = crate::display::context();

    static mut first_invocation: bool = true_0 != 0;
    if first_invocation {
        fprintf(stderr, c"Using slow code path for text extents\n".as_ptr());
        first_invocation = false_0 != 0;
    }
    let mut error: *mut xcb_generic_error_t = ::core::ptr::null_mut::<xcb_generic_error_t>();
    let cookie: xcb_query_text_extents_cookie_t = xcb_query_text_extents(
        conn,
        (*savedFont).specific.xcb.id as xcb_fontable_t,
        text_len as uint32_t,
        text as *mut xcb_char2b_t,
    );
    let reply: *mut xcb_query_text_extents_reply_t =
        xcb_query_text_extents_reply(conn, cookie, &raw mut error);
    if reply.is_null() {
        fprintf(
            stderr,
            c"Could not get text extents (X error code %d)\n".as_ptr(),
            (*error).error_code as ::core::ffi::c_int,
        );
        free(error as *mut ::core::ffi::c_void);
        return ((*(*savedFont).specific.xcb.info).max_bounds.character_width as size_t)
            .wrapping_mul(text_len) as ::core::ffi::c_int;
    }
    let width: ::core::ffi::c_int = (*reply).overall_width as ::core::ffi::c_int;
    free(reply as *mut ::core::ffi::c_void);
    width
}
unsafe fn predict_text_width_xcb(
    input: *const xcb_char2b_t,
    text_len: size_t,
) -> ::core::ffi::c_int {
    if text_len == 0 as size_t {
        return 0 as ::core::ffi::c_int;
    }
    let mut width: ::core::ffi::c_int = 0;
    if (*savedFont).specific.xcb.table.is_null() {
        width = xcb_query_text_width(input, text_len);
    } else {
        let font_info: *mut xcb_query_font_reply_t = (*savedFont).specific.xcb.info;
        let font_table: *mut xcb_charinfo_t = (*savedFont).specific.xcb.table;
        width = 0 as ::core::ffi::c_int;
        let mut i: size_t = 0 as size_t;
        while i < text_len {
            let mut info: *mut xcb_charinfo_t = ::core::ptr::null_mut::<xcb_charinfo_t>();
            let row: ::core::ffi::c_int = (*input.offset(i as isize)).byte1 as ::core::ffi::c_int;
            let col: ::core::ffi::c_int = (*input.offset(i as isize)).byte2 as ::core::ffi::c_int;
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
    width
}

pub unsafe fn predict_text_width(text: *mut i3String) -> ::core::ffi::c_int {
    if savedFont != ::core::ptr::null_mut::<::core::ffi::c_void>() as *const i3Font {
    } else {
        __assert_fail(
            c"savedFont != NULL".as_ptr(),
            c"../libi3/font.c".as_ptr(),
            467 as ::core::ffi::c_uint,
            c"int predict_text_width(i3String *)".as_ptr(),
        );
    };
    match (*savedFont).type_0 as ::core::ffi::c_uint {
        0 => return 0 as ::core::ffi::c_int,
        1 => {
            return predict_text_width_xcb(
                i3string_as_ucs2(text).cast(),
                i3string_get_num_glyphs(text),
            );
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
            c"false".as_ptr(),
            c"../libi3/font.c".as_ptr(),
            480 as ::core::ffi::c_uint,
            c"int predict_text_width(i3String *)".as_ptr(),
        );
    };
    panic!("Reached end of non-void function without returning");
}
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

// Internal helpers use Rust linkage rather than foreign symbol lookup.
use crate::dpi::get_dpi_value;
use crate::safewrappers::sstrdup;

// Canonical Cairo backend ABI; renderer logic remains local.
pub use cairo_sys::{
    cairo_create, cairo_destroy, cairo_move_to, cairo_set_operator, cairo_set_source_rgba,
    cairo_surface_destroy, cairo_surface_t, cairo_t, cairo_xcb_surface_create, OPERATOR_SOURCE,
};

// Canonical Pango backend ABI; font policy remains local.
pub use pango_sys::*;

use gobject_sys::g_object_unref;

use pango_cairo_sys::{
    pango_cairo_context_set_resolution, pango_cairo_create_context, pango_cairo_show_layout,
    pango_cairo_update_layout,
};

use xcb_sys::{
    xcb_change_gc, xcb_char2b_t, xcb_charinfo_t, xcb_close_font, xcb_drawable_t, xcb_font_t,
    xcb_fontable_t, xcb_gcontext_t, xcb_generate_id, xcb_generic_error_t, xcb_image_text_16,
    xcb_open_font_checked, xcb_query_font, xcb_query_font_char_infos,
    xcb_query_font_char_infos_length, xcb_query_font_cookie_t, xcb_query_font_reply,
    xcb_query_font_reply_t, xcb_query_text_extents, xcb_query_text_extents_cookie_t,
    xcb_query_text_extents_reply, xcb_query_text_extents_reply_t, xcb_request_check,
    xcb_visualtype_t, xcb_void_cookie_t, XCB_GC_BACKGROUND, XCB_GC_FONT, XCB_GC_FOREGROUND,
};

pub use libc::{fprintf, free, strlen, strncmp, FILE};

use crate::native_stdio::{__assert_fail, errx, stderr};

use crate::logging::{errorlog, verboselog};

// Shared model types retain field layout while using one Rust identity.
pub use crate::draw_util::color_t;
pub use crate::string::I3String as _i3String;

// Internal calls and globals use canonical Rust identities.
use crate::get_visualtype::get_visualtype;
use crate::string::i3string_as_utf8;
use crate::string::i3string_get_num_bytes;
use crate::string::i3string_get_num_glyphs;
use crate::string::i3string_is_markup;

use crate::string::i3string_as_ucs2;
