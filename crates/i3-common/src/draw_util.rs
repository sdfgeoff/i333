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

pub type i3String = _i3String;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct color_t {
    pub red: ::core::ffi::c_double,
    pub green: ::core::ffi::c_double,
    pub blue: ::core::ffi::c_double,
    pub alpha: ::core::ffi::c_double,
    pub colorpixel: u32,
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
    pub depth: u8,
    pub gc: xcb_gcontext_t,
}
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
fn surface_initialized(surface: &surface_t) -> bool {
    if surface.id == 0 {
        crate::logging::emit(
            crate::logging::Level::Error,
            format!(
                "[libi3] ERROR: Surface {:p} is not initialized, skipping drawing.\n",
                surface
            )
            .as_bytes(),
        );
        return false;
    }
    true
}
unsafe fn get_gc(
    conn: *mut xcb_connection_t,
    depth: u8,
    drawable: xcb_drawable_t,
    should_free: *mut bool,
) -> xcb_gcontext_t {
    static mut gc_cache: [C2RustUnnamed; 2] = [
        C2RustUnnamed {
            depth: 0 as u8,
            gc: 0,
        },
        C2RustUnnamed { depth: 0, gc: 0 },
    ];
    let mut index: usize = 0 as usize;
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
        0 as u32,
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

pub unsafe fn get_visual_depth(visual_id: xcb_visualid_t) -> u16 {
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
                return (*depth_iter.data).depth as u16;
            }
            xcb_visualtype_next(&raw mut visual_iter);
        }
        xcb_depth_next(&raw mut depth_iter);
    }
    0 as u16
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
        get_visual_depth((*visual).visual_id) as u8,
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
    if strlen(color) < 6 as usize
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
// These helpers work with real borrowed cairo-rs objects. The legacy surface
// record keeps ownership; no reference is retained beyond the caller's operation.
fn set_source_color(context: &cairo::Context, color: color_t) {
    context.set_source_rgba(color.red, color.green, color.blue, color.alpha);
}

fn flush_twice(surface: &cairo::Surface) {
    // Retain the historical flush ordering, including the redundant second call.
    surface.flush();
    surface.flush();
}

/// # Safety
/// text must point to a live I3String; surface must point to a readable record.
/// An initialized surface must contain live non-null Cairo objects and a valid
/// drawable/GC, and the current font/display must remain valid during drawing.
#[deny(unsafe_op_in_unsafe_fn)]
pub unsafe fn draw_util_text(
    text: *mut i3String,
    surface: *mut surface_t,
    fg_color: color_t,
    bg_color: color_t,
    x: i32,
    y: i32,
    max_width: i32,
) {
    // SAFETY: the caller keeps this record readable for the whole operation.
    let surface = unsafe { &*surface };
    if !surface_initialized(surface) {
        return;
    }
    // SAFETY: an initialized record holds a live non-null surface. This wrapper
    // borrows its reference, including error surfaces, and never outlives it.
    let target = unsafe { cairo::Surface::from_raw_borrow(surface.surface) };
    flush_twice(&target);
    // SAFETY: text, font, drawable and GC validity are the caller's contract.
    unsafe {
        set_font_colors(surface.gc, fg_color, bg_color);
        draw_text(
            text,
            surface.id,
            surface.gc,
            surface.surface,
            x,
            y,
            max_width,
        );
    }
    target.mark_dirty();
}

/// # Safety
/// surface must point to a readable record with live non-null Cairo objects when
/// initialized. image must be a live non-null Cairo surface for the entire call.
#[deny(unsafe_op_in_unsafe_fn)]
pub unsafe fn draw_util_image(
    image: *mut cairo_surface_t,
    surface: *mut surface_t,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
) {
    // SAFETY: the caller guarantees the record and its initialized native objects.
    let surface = unsafe { &*surface };
    if !surface_initialized(surface) {
        return;
    }
    // SAFETY: both pointers remain live, including native error objects. Borrowed
    // wrappers neither take ownership nor reject the native sticky error status.
    let context = unsafe { cairo::Context::from_raw_borrow(surface.cr) };
    let image_surface = unsafe { cairo::Surface::from_raw_borrow(image) };
    let _ = context.save();
    context.translate(x.into(), y.into());
    // SAFETY: image is live. Keep the native getter behavior for non-image/error
    // surfaces instead of introducing a typed ImageSurface conversion failure.
    let (src_width, src_height) = unsafe {
        (
            cairo_image_surface_get_width(image),
            cairo_image_surface_get_height(image),
        )
    };
    let horizontal = f64::from(width) / f64::from(src_width);
    let vertical = f64::from(height) / f64::from(src_height);
    let scale = if horizontal < vertical {
        horizontal
    } else {
        vertical
    };
    context.scale(scale, scale);
    let _ = context.set_source_surface(&*image_surface, 0.0, 0.0);
    let _ = context.paint();
    let _ = context.restore();
}

fn rectangle(
    context: &cairo::Context,
    target: &cairo::Surface,
    color: color_t,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
) {
    // Native drawing historically ignores results and retains Cairo's sticky
    // error status. Do not turn those errors into panics or early returns.
    let _ = context.save();
    context.set_operator(cairo::Operator::Source);
    set_source_color(context, color);
    context.rectangle(x, y, width, height);
    let _ = context.fill();
    flush_twice(target);
    let _ = context.restore();
}

/// # Safety
/// surface must point to a readable record. An initialized record must hold live
/// non-null Cairo context/surface pointers throughout this call.
#[deny(unsafe_op_in_unsafe_fn)]
pub unsafe fn draw_util_rectangle(
    surface: *mut surface_t,
    color: color_t,
    x: f64,
    y: f64,
    w: f64,
    h: f64,
) {
    // SAFETY: the caller keeps the record readable, and its native objects live.
    let surface = unsafe { &*surface };
    if !surface_initialized(surface) {
        return;
    }
    // SAFETY: both initialized object pointers remain live until these borrowed
    // wrappers leave scope. Their original reference counts are unchanged.
    let context = unsafe { cairo::Context::from_raw_borrow(surface.cr) };
    let target = unsafe { cairo::Surface::from_raw_borrow(surface.surface) };
    rectangle(&context, &target, color, x, y, w, h);
}

fn clear(context: &cairo::Context, target: &cairo::Surface, color: color_t) {
    let _ = context.save();
    context.set_operator(cairo::Operator::Source);
    set_source_color(context, color);
    let _ = context.paint();
    flush_twice(target);
    let _ = context.restore();
}

/// # Safety
/// surface must point to a readable record. An initialized record must hold live
/// non-null Cairo context/surface pointers throughout this call.
#[deny(unsafe_op_in_unsafe_fn)]
pub unsafe fn draw_util_clear_surface(surface: *mut surface_t, color: color_t) {
    // SAFETY: the caller keeps the record readable and its native objects live.
    let surface = unsafe { &*surface };
    if !surface_initialized(surface) {
        return;
    }
    // SAFETY: wrappers borrow live native objects without taking their references.
    let context = unsafe { cairo::Context::from_raw_borrow(surface.cr) };
    let target = unsafe { cairo::Surface::from_raw_borrow(surface.surface) };
    clear(&context, &target, color);
}

fn copy_surface(
    source: &cairo::Surface,
    target: &cairo::Surface,
    context: &cairo::Context,
    src_x: f64,
    src_y: f64,
    dest_x: f64,
    dest_y: f64,
    width: f64,
    height: f64,
) {
    let _ = context.save();
    context.set_operator(cairo::Operator::Source);
    let _ = context.set_source_surface(source, dest_x - src_x, dest_y - src_y);
    context.rectangle(dest_x, dest_y, width, height);
    let _ = context.fill();
    flush_twice(source);
    flush_twice(target);
    let _ = context.restore();
}

/// # Safety
/// src/dest must point to readable records. Initialized records must contain
/// live non-null Cairo objects throughout the call. Aliasing src/dest is allowed.
#[deny(unsafe_op_in_unsafe_fn)]
pub unsafe fn draw_util_copy_surface(
    src: *mut surface_t,
    dest: *mut surface_t,
    src_x: f64,
    src_y: f64,
    dest_x: f64,
    dest_y: f64,
    width: f64,
    height: f64,
) {
    // SAFETY: only shared record borrows are made, allowing legacy self copies.
    let (src, dest) = unsafe { (&*src, &*dest) };
    if !surface_initialized(src) || !surface_initialized(dest) {
        return;
    }
    // SAFETY: each pointer is live for these local, non-owning Cairo borrows.
    let context = unsafe { cairo::Context::from_raw_borrow(dest.cr) };
    let source = unsafe { cairo::Surface::from_raw_borrow(src.surface) };
    let target = unsafe { cairo::Surface::from_raw_borrow(dest.surface) };
    copy_surface(
        &source, &target, &context, src_x, src_y, dest_x, dest_y, width, height,
    );
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
    xcb_visualtype_iterator_t, xcb_visualtype_next, xcb_visualtype_t, xcb_void_cookie_t,
};

pub use libc::{free, strlen, strtol};

use crate::logging::{errorlog, verboselog};

// Internal calls and globals use canonical Rust identities.
use crate::font::set_font_colors;

pub use crate::string::I3String as _i3String;

#[cfg(test)]
mod drawing_tests {
    use super::*;
    use cairo::{Context, Format, ImageSurface, Operator};

    fn color(red: f64, green: f64, blue: f64, alpha: f64) -> color_t {
        color_t {
            red,
            green,
            blue,
            alpha,
            colorpixel: 0,
        }
    }

    // This record only borrows the test's owning Cairo objects; it is never
    // passed to the legacy ownership-transfer/free functions.
    fn record(image: &ImageSurface, context: &Context) -> surface_t {
        surface_t {
            id: 1,
            gc: 0,
            owns_gc: false,
            width: image.width(),
            height: image.height(),
            surface: image.to_raw_none(),
            cr: context.to_raw_none(),
        }
    }

    fn pixel(image: &ImageSurface, x: usize, y: usize) -> u32 {
        let offset = y * image.stride() as usize + x * 4;
        let mut pixel = 0;
        image
            .with_data(|bytes| {
                pixel = u32::from_ne_bytes(bytes[offset..offset + 4].try_into().unwrap())
            })
            .unwrap();
        pixel
    }

    fn references(image: &ImageSurface, context: &Context) -> (u32, u32) {
        // SAFETY: test-owned image is live for this native reference-count query.
        (
            unsafe { cairo_sys::cairo_surface_get_reference_count(image.to_raw_none()) },
            context.reference_count(),
        )
    }

    #[test]
    fn rectangle_and_clear_preserve_clip_source_state_and_references() {
        let image = ImageSurface::create(Format::ARgb32, 4, 4).unwrap();
        let context = Context::new(&image).unwrap();
        let mut raw = record(&image, &context);
        context.rectangle(1.0, 1.0, 2.0, 2.0);
        context.clip();
        context.set_source_rgb(0.0, 1.0, 0.0);
        let before = references(&image, &context);
        let matrix = context.matrix();
        // SAFETY: raw borrows the live test-owned image/context.
        unsafe {
            draw_util_clear_surface(&mut raw, color(1.0, 0.0, 0.0, 1.0));
            draw_util_rectangle(&mut raw, color(1.0, 0.0, 0.0, 0.5), 1.0, 1.0, 1.0, 1.0);
        }
        assert_eq!(pixel(&image, 0, 0), 0);
        assert_eq!(pixel(&image, 1, 1), 0x80800000);
        assert_eq!(pixel(&image, 2, 2), 0xFFFF0000);
        assert_eq!(context.operator(), Operator::Over);
        assert_eq!(context.matrix(), matrix);
        assert_eq!(references(&image, &context), before);
        context.paint().unwrap();
        assert_eq!(pixel(&image, 1, 1), 0xFF00FF00);
        assert_eq!(pixel(&image, 0, 0), 0);
    }

    #[test]
    fn copy_uses_source_offset_and_clips_destination_without_retaining_source() {
        let source = ImageSurface::create(Format::ARgb32, 4, 4).unwrap();
        let source_context = Context::new(&source).unwrap();
        let mut src = record(&source, &source_context);
        let target = ImageSurface::create(Format::ARgb32, 5, 4).unwrap();
        let target_context = Context::new(&target).unwrap();
        let mut dest = record(&target, &target_context);
        let before = (
            references(&source, &source_context),
            references(&target, &target_context),
        );
        // SAFETY: both records borrow live owning Cairo objects, including the
        // aliasing self-copy path below; no Rust-exclusive native borrow is made.
        unsafe {
            draw_util_rectangle(&mut src, color(1.0, 0.0, 0.0, 1.0), 1.0, 1.0, 2.0, 2.0);
            draw_util_copy_surface(&mut src, &mut dest, 1.0, 1.0, 2.0, 0.0, 2.0, 2.0);
        }
        assert_eq!(pixel(&target, 2, 0), 0xFFFF0000);
        assert_eq!(pixel(&target, 3, 1), 0xFFFF0000);
        assert_eq!(pixel(&target, 1, 0), 0);
        assert_eq!(pixel(&target, 4, 0), 0);
        assert_eq!(target_context.operator(), Operator::Over);
        assert_eq!(
            (
                references(&source, &source_context),
                references(&target, &target_context)
            ),
            before
        );
        // SAFETY: shared record/native object borrows permit aliasing src/dest.
        unsafe { draw_util_copy_surface(&mut dest, &mut dest, 2.0, 0.0, 0.0, 2.0, 1.0, 1.0) };
        assert_eq!(pixel(&target, 0, 2), 0xFFFF0000);
    }

    #[test]
    fn image_draw_preserves_aspect_ratio_transform_and_borrowed_ownership() {
        let source = ImageSurface::create(Format::ARgb32, 4, 2).unwrap();
        let source_context = Context::new(&source).unwrap();
        source_context.set_source_rgb(0.0, 0.0, 1.0);
        source_context.paint().unwrap();
        let target = ImageSurface::create(Format::ARgb32, 10, 10).unwrap();
        let context = Context::new(&target).unwrap();
        let mut dest = record(&target, &context);
        let before = (
            references(&source, &source_context),
            references(&target, &context),
        );
        let matrix = context.matrix();
        // SAFETY: source and destination objects remain owned by this test.
        unsafe { draw_util_image(source.to_raw_none(), &mut dest, 1, 2, 8, 8) };
        assert_eq!(pixel(&target, 4, 3), 0xFF0000FF);
        assert_eq!(pixel(&target, 4, 8), 0);
        assert_eq!(context.matrix(), matrix);
        assert_eq!(
            (
                references(&source, &source_context),
                references(&target, &context)
            ),
            before
        );
    }

    #[test]
    fn native_error_objects_remain_sticky_and_uninitialized_records_are_skipped() {
        // SAFETY: Cairo returns a non-null error surface for invalid dimensions.
        // from_raw_none keeps that object's error status rather than rejecting it.
        let pointer =
            unsafe { cairo_sys::cairo_image_surface_create(cairo_sys::FORMAT_A_RGB32, -1, -1) };
        let surface = unsafe { cairo::Surface::from_raw_none(pointer) };
        // SAFETY: release the original native reference; the wrapper owns its ref.
        unsafe { cairo_sys::cairo_surface_destroy(pointer) };
        // SAFETY: native cairo_create always supplies a non-null context, even
        // when the target has an error. This owning wrapper preserves that error.
        let context =
            unsafe { Context::from_raw_full(cairo_sys::cairo_create(surface.to_raw_none())) };
        let mut raw = surface_t {
            id: 1,
            gc: 0,
            owns_gc: false,
            width: -1,
            height: -1,
            surface: surface.to_raw_none(),
            cr: context.to_raw_none(),
        };
        let before = context.status();
        assert!(before.is_err());
        // SAFETY: non-null native error objects remain live and support drawing
        // no-ops. These calls must not convert their sticky error into a panic.
        unsafe {
            draw_util_clear_surface(&mut raw, color(1.0, 0.0, 0.0, 1.0));
            draw_util_rectangle(&mut raw, color(1.0, 0.0, 0.0, 1.0), 0.0, 0.0, 1.0, 1.0);
            draw_util_copy_surface(&mut raw, &mut raw, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0);
            draw_util_image(surface.to_raw_none(), &mut raw, 0, 0, 1, 1);
        }
        assert_eq!(context.status(), before);
        raw.id = 0;
        raw.surface = std::ptr::null_mut();
        raw.cr = std::ptr::null_mut();
        // SAFETY: the zero id guard permits uninitialized null Cairo fields and
        // skips drawing before attempting any native wrapper construction.
        unsafe { draw_util_clear_surface(&mut raw, color(0.0, 0.0, 0.0, 0.0)) };
    }
}
