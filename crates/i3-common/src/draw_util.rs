use crate::font::draw_text;
/*
 * vim:ts=4:sw=4:expandtab
 *
 * © 2015 Ingo Bürk and contributors (see also: LICENSE)
 *
 * draw.c: Utility for drawing.
 *
 */
// Mechanical file-by-file Rust translation of libi3/draw_util.c.
// Originally translated with C2Rust 0.22.1; subsequently maintained in Rust.
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;

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

pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
unsafe fn surface_initialized(surface: *mut surface_t) -> bool {
    if (*surface).id as ::core::ffi::c_long == XCB_NONE {
        errorlog(
            c"[libi3] ERROR: Surface %p is not initialized, skipping drawing.\n"
                .as_ptr()
                .cast_mut(),
            surface,
        );
        return false_0 != 0;
    }
    true_0 != 0
}
unsafe fn get_gc(
    conn: *mut xcb_connection_t,
    depth: uint8_t,
    drawable: xcb_drawable_t,
    should_free: *mut bool,
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
    let gc: xcb_gcontext_t = xcb_generate_id(conn) as xcb_gcontext_t;
    let gc_cookie: xcb_void_cookie_t = xcb_create_gc_checked(
        conn,
        gc,
        drawable,
        0 as uint32_t,
        ::core::ptr::null::<::core::ffi::c_void>(),
    );
    let error: *mut xcb_generic_error_t = xcb_request_check(conn, gc_cookie);
    if !error.is_null() {
        errorlog(
            c"[libi3] ERROR: Could not create graphical context. Error code: %d. Please report this bug.\n".as_ptr().cast_mut(),
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
    gc
}

pub unsafe fn get_visual_depth(visual_id: xcb_visualid_t) -> uint16_t {
    let crate::display::DisplayContext { root_screen, .. } = crate::display::context();

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
    0 as uint16_t
}

pub unsafe fn draw_util_surface_init(
    conn: *mut xcb_connection_t,
    surface: *mut surface_t,
    drawable: xcb_drawable_t,
    mut visual: *mut xcb_visualtype_t,
    width: ::core::ffi::c_int,
    height: ::core::ffi::c_int,
) {
    let crate::display::DisplayContext { visual_type, .. } = crate::display::context();

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
    (*surface).surface =
        cairo_xcb_surface_create((conn).cast(), (*surface).id, (visual).cast(), width, height);
    (*surface).cr = cairo_create((*surface).surface);
}

pub unsafe fn draw_util_surface_free(conn: *mut xcb_connection_t, surface: *mut surface_t) {
    let mut status: cairo_status_t = STATUS_SUCCESS;
    if !(*surface).cr.is_null() {
        status = cairo_status((*surface).cr);
    }
    if status != STATUS_SUCCESS {
        verboselog(
            c"[libi3] ../libi3/draw_util.c Found cairo context in an error status while freeing, error %d is %s".as_ptr().cast_mut(),
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

pub unsafe fn draw_util_surface_set_size(
    surface: *mut surface_t,
    width: ::core::ffi::c_int,
    height: ::core::ffi::c_int,
) {
    (*surface).width = width;
    (*surface).height = height;
    cairo_xcb_surface_set_size((*surface).surface, width, height);
}

pub unsafe fn draw_util_hex_to_color(color: *const ::core::ffi::c_char) -> color_t {
    if strlen(color) < 6 as size_t
        || *color.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '#' as i32
    {
        errorlog(
            c"[libi3] ERROR: Could not parse color: %s\n"
                .as_ptr()
                .cast_mut(),
            color,
        );
        return draw_util_hex_to_color(c"#A9A9A9".as_ptr());
    }
    let mut alpha: [::core::ffi::c_char; 2] = [0; 2];
    if strlen(color) == strlen(c"#rrggbbaa".as_ptr()) {
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
    color_t {
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
    }
}
unsafe fn draw_util_set_source_color(surface: *mut surface_t, color: color_t) {
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

pub unsafe fn draw_util_text(
    text: *mut i3String,
    surface: *mut surface_t,
    fg_color: color_t,
    bg_color: color_t,
    x: ::core::ffi::c_int,
    y: ::core::ffi::c_int,
    max_width: ::core::ffi::c_int,
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

pub unsafe fn draw_util_image(
    image: *mut cairo_surface_t,
    surface: *mut surface_t,
    x: ::core::ffi::c_int,
    y: ::core::ffi::c_int,
    width: ::core::ffi::c_int,
    height: ::core::ffi::c_int,
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
    let scale: ::core::ffi::c_double = if (width as ::core::ffi::c_double
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

pub unsafe fn draw_util_rectangle(
    surface: *mut surface_t,
    color: color_t,
    x: ::core::ffi::c_double,
    y: ::core::ffi::c_double,
    w: ::core::ffi::c_double,
    h: ::core::ffi::c_double,
) {
    if !surface_initialized(surface) {
        return;
    }
    cairo_save((*surface).cr);
    cairo_set_operator((*surface).cr, OPERATOR_SOURCE);
    draw_util_set_source_color(surface, color);
    cairo_rectangle((*surface).cr, x, y, w, h);
    cairo_fill((*surface).cr);
    cairo_surface_flush((*surface).surface);
    cairo_surface_flush((*surface).surface);
    cairo_restore((*surface).cr);
}

pub unsafe fn draw_util_clear_surface(surface: *mut surface_t, color: color_t) {
    if !surface_initialized(surface) {
        return;
    }
    cairo_save((*surface).cr);
    cairo_set_operator((*surface).cr, OPERATOR_SOURCE);
    draw_util_set_source_color(surface, color);
    cairo_paint((*surface).cr);
    cairo_surface_flush((*surface).surface);
    cairo_surface_flush((*surface).surface);
    cairo_restore((*surface).cr);
}

pub unsafe fn draw_util_copy_surface(
    src: *mut surface_t,
    dest: *mut surface_t,
    src_x: ::core::ffi::c_double,
    src_y: ::core::ffi::c_double,
    dest_x: ::core::ffi::c_double,
    dest_y: ::core::ffi::c_double,
    width: ::core::ffi::c_double,
    height: ::core::ffi::c_double,
) {
    if !surface_initialized(src) || !surface_initialized(dest) {
        return;
    }
    cairo_save((*dest).cr);
    cairo_set_operator((*dest).cr, OPERATOR_SOURCE);
    cairo_set_source_surface((*dest).cr, (*src).surface, dest_x - src_x, dest_y - src_y);
    cairo_rectangle((*dest).cr, dest_x, dest_y, width, height);
    cairo_fill((*dest).cr);
    cairo_surface_flush((*src).surface);
    cairo_surface_flush((*src).surface);
    cairo_surface_flush((*dest).surface);
    cairo_surface_flush((*dest).surface);
    cairo_restore((*dest).cr);
}

// Internal helpers use Rust linkage rather than foreign symbol lookup.
use crate::get_colorpixel::get_colorpixel;

// Canonical Cairo backend ABI; renderer logic remains local.
pub use cairo_sys::{
    cairo_create, cairo_destroy, cairo_fill, cairo_image_surface_get_height,
    cairo_image_surface_get_width, cairo_paint, cairo_rectangle, cairo_restore, cairo_save,
    cairo_scale, cairo_set_operator, cairo_set_source_rgba, cairo_set_source_surface, cairo_status,
    cairo_status_t, cairo_status_to_string, cairo_surface_destroy, cairo_surface_flush,
    cairo_surface_mark_dirty, cairo_surface_t, cairo_t, cairo_translate, cairo_xcb_surface_create,
    cairo_xcb_surface_set_size, OPERATOR_SOURCE, STATUS_SUCCESS,
};

use xcb_sys::{
    xcb_connection_t, xcb_create_gc_checked, xcb_depth_iterator_t, xcb_depth_next, xcb_depth_t,
    xcb_depth_visuals_iterator, xcb_drawable_t, xcb_free_gc, xcb_gcontext_t, xcb_generate_id,
    xcb_generic_error_t, xcb_request_check, xcb_screen_allowed_depths_iterator, xcb_visualid_t,
    xcb_visualtype_iterator_t, xcb_visualtype_next, xcb_visualtype_t, xcb_void_cookie_t, XCB_NONE,
};

pub use libc::{free, strlen, strtol};

use crate::logging::{errorlog, verboselog};

// Internal calls and globals use canonical Rust identities.
use crate::font::set_font_colors;

pub use crate::string::I3String as _i3String;
