/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 */
// Mechanical file-by-file Rust translation of libi3/font.c.
// Originally translated with C2Rust 0.22.1; subsequently maintained in Rust.

use crate::string::I3String;
use crate::ucs2_conversion::Ucs2Char;
use xcb::{x, Xid, XidNew};
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Font {
    pub type_0: FontType,
    pub height: i32,
    pub pattern: *mut libc::c_char,
    pub specific: FontBackendData,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union FontBackendData {
    pub xcb: XcbFontData,
    pub pango_desc: *mut PangoFontDescription,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XcbFontData {
    pub id: xcb_font_t,
    /// Sole Box owner; the enclosing legacy Font POD must not be freed twice.
    pub info: *mut x::QueryFontReply,
}

pub type i3Font = Font;

static mut SAVED_FONT: *const i3Font = ptr::null();
static mut ROOT_VISUAL_TYPE: *mut cairo_sys::xcb_visualtype_t = ptr::null_mut();
static mut PANGO_FONT_RED: f64 = 0.;
static mut PANGO_FONT_GREEN: f64 = 0.;
static mut PANGO_FONT_BLUE: f64 = 0.;
static mut PANGO_FONT_ALPHA: f64 = 0.;
use glib::translate::{from_glib_borrow, from_glib_full, IntoGlibPtr, ToGlibPtr};
use std::{ffi::CStr, mem::size_of, ptr, ptr::NonNull};

fn create_layout_with_dpi(context: &cairo::Context, dpi: f64) -> pango::Layout {
    let pango_context = pangocairo::functions::create_context(context);
    pangocairo::functions::context_set_resolution(&pango_context, dpi);
    pango::Layout::new(&pango_context)
}

/// Set the original explicit byte count without decoding, repairing, or rejecting
/// embedded NULs. The maintained &str setters cannot express all legacy inputs.
///
/// # Safety
/// text must satisfy Pango's native text/markup setter contract for len as i32:
/// readable bytes for a nonnegative count, or NUL-terminated when it is negative.
#[deny(unsafe_op_in_unsafe_fn)]
unsafe fn set_layout_bytes(
    layout: &pango::Layout,
    text: *const libc::c_char,
    len: usize,
    markup: bool,
) {
    // SAFETY: caller supplies the original native byte buffer and length. Pango
    // copies/consumes the buffer synchronously; it never escapes this call.
    unsafe {
        if markup {
            pango_layout_set_markup(layout.to_glib_none().0, text, len as i32);
        } else {
            pango_layout_set_text(layout.to_glib_none().0, text, len as i32);
        }
    }
}

/// # Safety
/// conn/root/visual must describe a live XCB display and a valid root visual.
#[deny(unsafe_op_in_unsafe_fn)]
unsafe fn font_surface(
    conn: *mut xcb_connection_t,
    root: xcb_drawable_t,
    visual: *mut cairo_sys::xcb_visualtype_t,
) -> cairo::Surface {
    // SAFETY: native Cairo returns a non-null surface, including error objects.
    let pointer = unsafe { cairo_xcb_surface_create(conn.cast(), root, visual.cast(), 1, 1) };
    // SAFETY: take a reference without status-checking, preserving error objects.
    // from_raw_full would reject those objects, changing the legacy error path.
    let surface = unsafe { cairo::Surface::from_raw_none(pointer) };
    // SAFETY: the maintained owner now holds a reference; release the original.
    unsafe { cairo_surface_destroy(pointer) };
    surface
}

/// # Safety
/// surface must be live and non-null. The resulting context owns its native ref.
#[deny(unsafe_op_in_unsafe_fn)]
unsafe fn font_context(surface: *mut cairo_surface_t) -> cairo::Context {
    // SAFETY: cairo_create returns a non-null context even for native errors;
    // from_raw_full owns that reference without rejecting its sticky error.
    unsafe { cairo::Context::from_raw_full(cairo_create(surface)) }
}

/// # Safety
/// A non-null stored description must remain live while the returned borrow is used.
#[deny(unsafe_op_in_unsafe_fn)]
unsafe fn borrowed_description(
    font: &Font,
) -> Option<glib::translate::Borrowed<pango::FontDescription>> {
    // SAFETY: the active union member is selected by the caller's Pango font.
    NonNull::new(unsafe { font.specific.pango_desc }).map(|pointer| {
        // SAFETY: the caller keeps this non-null description live for the borrow.
        unsafe { from_glib_borrow(pointer.as_ptr()) }
    })
}

/// # Safety
/// font is writable; desc is a valid native C string. Display/font globals must
/// be used only on the event thread and remain live throughout this operation.
#[deny(unsafe_op_in_unsafe_fn)]
unsafe fn load_pango_font(font: *mut i3Font, desc: *const libc::c_char) -> bool {
    let crate::display::DisplayContext {
        connection: conn,
        screen,
        ..
    } = crate::display::context();
    // SAFETY: desc is a native C string; preserve its bytes rather than assuming UTF-8.
    let pointer = unsafe { pango_font_description_from_string(desc) };
    // Preserve the legacy field value on the native failure path.
    unsafe { (*font).specific.pango_desc = pointer };
    if pointer.is_null() {
        // SAFETY: format and desc are live NUL-terminated native strings.
        unsafe {
            errorlog(
                c"[libi3] ERROR: Could not open font %s with Pango, fallback to X font.\n"
                    .as_ptr()
                    .cast_mut(),
                desc,
            )
        };
        return false;
    }
    // SAFETY: native construction transfers a non-null description to this owner.
    let description: pango::FontDescription = unsafe { from_glib_full(pointer) };
    // SAFETY: description and the borrowed family C string remain live for logging.
    unsafe {
        verboselog(
            c"[libi3] ../libi3/font.c Using Pango font %s, size %d\n"
                .as_ptr()
                .cast_mut(),
            pango_font_description_get_family(pointer),
            description.size() / PANGO_SCALE,
        )
    };
    // SAFETY: caller guarantees the display setup, event-thread globals and font.
    let (surface, dpi) = unsafe {
        ROOT_VISUAL_TYPE =
            crate::display::visual(screen.expect("initialized display screen").root_visual)
                .expect("root Cairo visual")
                .as_ptr();
        (
            font_surface(
                conn.cast(),
                screen.expect("initialized display screen").root,
                ROOT_VISUAL_TYPE,
            ),
            get_dpi_value() as f64,
        )
    };
    // SAFETY: the local owning surface remains live while its context is used.
    let context = unsafe { font_context(surface.to_raw_none()) };
    let layout = create_layout_with_dpi(&context, dpi);
    layout.set_font_description(Some(&description));
    // SAFETY: transfer the description owner into the unchanged legacy Font POD.
    // free_font will reclaim it; no Rust-owned fields are installed in this record.
    unsafe {
        (*font).height = layout.pixel_size().1;
        (*font).specific.pango_desc = description.into_glib_ptr();
        (*font).type_0 = FontType::Pango;
    }
    true
}

/// # Safety
/// text satisfies set_layout_bytes' native buffer contract. surface and the
/// saved Pango Font/description must stay live on the event thread until return.
#[deny(unsafe_op_in_unsafe_fn)]
unsafe fn draw_text_pango(
    text: *const libc::c_char,
    text_len: usize,
    _drawable: xcb_drawable_t,
    surface: *mut cairo_surface_t,
    x: i32,
    y: i32,
    max_width: i32,
    pango_markup: bool,
) {
    // SAFETY: caller keeps the native surface, saved font and globals valid.
    let (context, font, dpi, color) = unsafe {
        (
            font_context(surface),
            &*SAVED_FONT,
            get_dpi_value() as f64,
            (
                PANGO_FONT_RED,
                PANGO_FONT_GREEN,
                PANGO_FONT_BLUE,
                PANGO_FONT_ALPHA,
            ),
        )
    };
    let layout = create_layout_with_dpi(&context, dpi);
    // SAFETY: borrow the live description stored in the saved Font, without copying
    // or taking ownership. The local borrowed wrapper cannot outlive this call.
    let description = unsafe { borrowed_description(font) };
    layout.set_font_description(description.as_deref());
    layout.set_width(max_width * PANGO_SCALE);
    layout.set_wrap(pango::WrapMode::Char);
    layout.set_ellipsize(pango::EllipsizeMode::End);
    layout.set_single_paragraph_mode(true);
    // SAFETY: preserve the caller's original native buffer and explicit count.
    unsafe { set_layout_bytes(&layout, text, text_len, pango_markup) };
    draw_pango_layout(&context, &layout, color, x, y, font.height);
}

fn draw_pango_layout(
    context: &cairo::Context,
    layout: &pango::Layout,
    color: (f64, f64, f64, f64),
    x: i32,
    y: i32,
    font_height: i32,
) {
    context.set_operator(cairo::Operator::Source);
    context.set_source_rgba(color.0, color.1, color.2, color.3);
    pangocairo::functions::update_layout(context, layout);
    let yoffset = (layout.pixel_size().1 - font_height) / 2;
    context.move_to(x.into(), (y - yoffset).into());
    pangocairo::functions::show_layout(context, layout);
}

/// # Safety
/// text satisfies set_layout_bytes' native buffer contract. The saved Pango font,
/// description, display and global visual must stay live on the event thread.
#[deny(unsafe_op_in_unsafe_fn)]
unsafe fn predict_text_width_pango(
    text: *const libc::c_char,
    text_len: usize,
    pango_markup: bool,
) -> i32 {
    let crate::display::DisplayContext {
        connection: conn,
        screen,
        ..
    } = crate::display::context();
    // SAFETY: caller guarantees the display, global visual, DPI and saved font.
    let (surface, dpi, font) = unsafe {
        (
            font_surface(
                conn.cast(),
                screen.expect("initialized display screen").root,
                ROOT_VISUAL_TYPE,
            ),
            get_dpi_value() as f64,
            &*SAVED_FONT,
        )
    };
    // SAFETY: surface stays owned throughout the context's use.
    let context = unsafe { font_context(surface.to_raw_none()) };
    let layout = create_layout_with_dpi(&context, dpi);
    // SAFETY: the saved font's description is borrowed only during this call.
    let description = unsafe { borrowed_description(font) };
    layout.set_font_description(description.as_deref());
    // SAFETY: preserve the original explicit native byte buffer/count.
    unsafe { set_layout_bytes(&layout, text, text_len, pango_markup) };
    measure_pango_layout(&context, &layout)
}

fn measure_pango_layout(context: &cairo::Context, layout: &pango::Layout) -> i32 {
    pangocairo::functions::update_layout(context, layout);
    layout.pixel_size().0
}

fn pango_pattern(pattern: &CStr) -> Option<&CStr> {
    let bytes = pattern.to_bytes_with_nul();
    let description = bytes
        .strip_prefix(b"pango:")
        .or_else(|| bytes.strip_prefix(b"xft:"))?;
    let description = CStr::from_bytes_with_nul(description).ok()?;
    (!description.to_bytes().is_empty()).then_some(description)
}

/// Load a legacy POD font whose native allocations are released by free_font.
///
/// # Safety
/// pattern must be NUL-terminated and readable. Display globals and any previous
/// saved Font must remain valid on the event thread; that previous Font's resources
/// are released here. The returned POD must not acquire multiple owning copies.
pub unsafe fn load_font(pattern: *const libc::c_char, fallback: bool) -> i3Font {
    let crate::display::DisplayContext {
        connection: conn, ..
    } = crate::display::context();

    free_font();
    let mut font: i3Font = Font {
        type_0: FontType::None,
        height: 0,
        pattern: ptr::null_mut(),
        specific: FontBackendData {
            xcb: XcbFontData {
                id: 0,
                info: ptr::null_mut(),
            },
        },
    };
    if conn.is_null() {
        return font;
    }
    // SAFETY: the caller provides a native NUL-terminated pattern. Prefix parsing
    // uses borrowed bytes without decoding or allocating a different font name.
    let mut pattern = CStr::from_ptr(pattern);
    if let Some(description) = pango_pattern(pattern) {
        if load_pango_font(&raw mut font, description.as_ptr()) {
            font.pattern = sstrdup(pattern.as_ptr());
            return font;
        }
    }
    let connection = crate::display::connection()
        .or_else(|| crate::xcb_connection::borrowed(conn.cast()).map(std::rc::Rc::new));
    let Some(connection) = connection else {
        errx(
            libc::EXIT_FAILURE,
            c"Could not load font \"%s\"".as_ptr(),
            pattern.as_ptr(),
        );
    };
    let id: x::Font = connection.generate_id();
    font.specific.xcb.id = id.resource_id();
    let (font_cookie, mut info_cookie) = open_core_font(&connection, id, pattern);
    let mut error = connection.check_request(font_cookie).err();
    if fallback && error.is_some() {
        errorlog(
            c"[libi3] ERROR: Could not open font %s (X error %d). Trying fallback to 'fixed'.\n"
                .as_ptr()
                .cast_mut(),
            pattern.as_ptr(),
            i32::from(error.as_ref().map_or(0, protocol_error_code)),
        );
        pattern = c"fixed";
        let (font_cookie, cookie) = open_core_font(&connection, id, pattern);
        info_cookie = cookie;
        error = connection.check_request(font_cookie).err();
        if error.is_some() {
            errorlog(
                c"[libi3] ERROR: Could not open fallback font 'fixed', trying with '-misc-*'.\n"
                    .as_ptr()
                    .cast_mut(),
            );
            pattern = c"-misc-*";
            let (font_cookie, cookie) = open_core_font(&connection, id, pattern);
            info_cookie = cookie;
            error = connection.check_request(font_cookie).err();
            if error.is_some() {
                errx(
                    libc::EXIT_FAILURE,
                    c"Could open neither requested font nor fallbacks (fixed or -misc-*): X11 error %d".as_ptr(),
                    i32::from(error.as_ref().map_or(0, protocol_error_code)),
                );
            }
        }
    }
    drop(error);
    font.pattern = sstrdup(pattern.as_ptr());
    verboselog(
        c"[libi3] ../libi3/font.c Using X font %s\n"
            .as_ptr()
            .cast_mut(),
        pattern.as_ptr(),
    );
    let Ok(info) = connection.wait_for_reply(info_cookie) else {
        errx(
            libc::EXIT_FAILURE,
            c"Could not load font \"%s\"".as_ptr(),
            pattern.as_ptr(),
        );
    };
    if info.length() < 7 {
        errx(
            libc::EXIT_FAILURE,
            c"Could not load font \"%s\"".as_ptr(),
            pattern.as_ptr(),
        );
    }
    font.height = i32::from(info.font_ascent()) + i32::from(info.font_descent());
    font.specific.xcb.info = Box::into_raw(Box::new(info));
    font.type_0 = FontType::Xcb;
    font
}

/// Select a borrowed font for subsequent drawing and eventual free_font.
///
/// # Safety
/// font must remain readable and unmoved until replaced/freed, with all resources
/// matching its active type. Calls using this global are restricted to the event
/// thread. POD copies must not independently free the same native allocations.
pub unsafe fn set_font(font: *mut i3Font) {
    SAVED_FONT = font;
}

/// Release resources of the currently selected borrowed legacy font.
///
/// # Safety
/// The saved font, its unique native allocations, and display connection must
/// remain valid on the event thread. Do not use the freed POD until reinitialized.
pub unsafe fn free_font() {
    let crate::display::DisplayContext {
        connection: conn, ..
    } = crate::display::context();

    if SAVED_FONT.is_null() {
        return;
    }
    free((*SAVED_FONT).pattern as *mut libc::c_void);
    match (*SAVED_FONT).type_0 {
        FontType::Xcb => {
            if let Some(connection) = crate::display::connection()
                .or_else(|| crate::xcb_connection::borrowed(conn.cast()).map(std::rc::Rc::new))
            {
                connection.send_request(&x::CloseFont {
                    font: x::Font::new((*SAVED_FONT).specific.xcb.id),
                });
            }
            if let Some(info) = NonNull::new((*SAVED_FONT).specific.xcb.info) {
                drop(Box::from_raw(info.as_ptr()));
            }
        }
        FontType::Pango => {
            // Reclaim the description reference transferred into the raw POD.
            // A null description is Pango's default, not an owning allocation.
            if let Some(pointer) = NonNull::new((*SAVED_FONT).specific.pango_desc) {
                drop(from_glib_full::<_, pango::FontDescription>(
                    pointer.as_ptr(),
                ));
            }
        }
        FontType::None => {}
    }
    SAVED_FONT = ptr::null();
}

/// # Safety
/// A live font must be selected on the event thread. For XCB fonts, gc must be a
/// valid graphics context on the registered display connection.
pub unsafe fn set_font_colors(gc: xcb_gcontext_t, foreground: color_t, background: color_t) {
    let crate::display::DisplayContext {
        connection: conn, ..
    } = crate::display::context();

    if SAVED_FONT.is_null() {
        __assert_fail(
            c"savedFont != NULL".as_ptr(),
            c"../libi3/font.c".as_ptr(),
            296 as u32,
            c"void set_font_colors(xcb_gcontext_t, color_t, color_t)".as_ptr(),
        );
    };
    match (*SAVED_FONT).type_0 {
        FontType::Xcb => {
            if let Some(connection) = crate::display::connection()
                .or_else(|| crate::xcb_connection::borrowed(conn.cast()).map(std::rc::Rc::new))
            {
                connection.send_request(&x::ChangeGc {
                    gc: x::Gcontext::new(gc),
                    value_list: &[
                        x::Gc::Foreground(foreground.colorpixel),
                        x::Gc::Background(background.colorpixel),
                        x::Gc::Font(x::Font::new((*SAVED_FONT).specific.xcb.id)),
                    ],
                });
            }
        }
        FontType::Pango => {
            PANGO_FONT_RED = foreground.red;
            PANGO_FONT_GREEN = foreground.green;
            PANGO_FONT_BLUE = foreground.blue;
            PANGO_FONT_ALPHA = foreground.alpha;
        }
        FontType::None => {}
    };
}

/// # Safety
/// A readable font must remain selected on the event thread during this call.
pub unsafe fn font_is_pango() -> bool {
    (*SAVED_FONT).type_0 == FontType::Pango
}
/// # Safety
/// The saved XCB font/info, display, drawable and GC must stay live on the event thread.
#[deny(unsafe_op_in_unsafe_fn)]
unsafe fn draw_text_xcb(
    glyphs: &[Ucs2Char],
    drawable: xcb_drawable_t,
    gc: xcb_gcontext_t,
    mut x: i32,
    y: i32,
) {
    let raw = crate::display::context().connection;
    // SAFETY: the saved font and display remain live under the caller's contract.
    let connection = crate::display::connection()
        .or_else(|| unsafe { crate::xcb_connection::borrowed(raw.cast()) }.map(std::rc::Rc::new));
    let Some(connection) = connection else { return };
    let pos_y = y + i32::from(unsafe { (*(*SAVED_FONT).specific.xcb.info).font_ascent() });
    let mut chunks = glyphs
        .chunks(255)
        .chain(glyphs.is_empty().then_some(&[][..]))
        .peekable();
    let mut wire = [x::Char2b { byte1: 0, byte2: 0 }; 255];
    while let Some(chunk) = chunks.next() {
        for (slot, glyph) in wire.iter_mut().zip(chunk) {
            *slot = x::Char2b {
                byte1: glyph.byte1,
                byte2: glyph.byte2,
            };
        }
        connection.send_request(&x::ImageText16 {
            drawable: x::Drawable::Unknown(drawable),
            gc: unsafe { x::Gcontext::new(gc) },
            x: x as i16,
            y: pos_y as i16,
            string: &wire[..chunk.len()],
        });
        if chunks.peek().is_some() {
            x += unsafe { predict_text_width_xcb(chunk) };
        }
    }
}

/// # Safety
/// The selected Font and display resources must
/// remain live on the event thread. The requested backend's drawable/GC/Cairo
/// surface must be valid during native drawing (including non-null error surfaces).
pub unsafe fn draw_text(
    text: &I3String,
    drawable: xcb_drawable_t,
    gc: xcb_gcontext_t,
    surface: *mut cairo_surface_t,
    x: i32,
    y: i32,
    max_width: i32,
) {
    if SAVED_FONT.is_null() {
        __assert_fail(
            c"savedFont != NULL".as_ptr(),
            c"../libi3/font.c".as_ptr(),
            369 as u32,
            c"void draw_text(i3String *, xcb_drawable_t, xcb_gcontext_t, cairo_surface_t *, int, int, int)".as_ptr(),
        );
    };
    match (*SAVED_FONT).type_0 {
        FontType::None => {}
        FontType::Xcb => {
            draw_text_xcb(text.rendering_glyphs(), drawable, gc, x, y);
        }
        FontType::Pango => {
            draw_text_pango(
                text.cstr_or_report()
                    .map_or(std::ptr::null(), std::ffi::CStr::as_ptr),
                text.rendering_byte_len(),
                drawable,
                surface,
                x,
                y,
                max_width,
                text.is_markup(),
            );
        }
    }
}
fn wire_glyphs(glyphs: &[Ucs2Char]) -> Vec<x::Char2b> {
    glyphs
        .iter()
        .map(|glyph| x::Char2b {
            byte1: glyph.byte1,
            byte2: glyph.byte2,
        })
        .collect()
}

fn protocol_error_code(error: &xcb::ProtocolError) -> u8 {
    // SAFETY: the maintained error owns its native header. This byte preserves
    // numeric diagnostics without calling any closed-enum accessor.
    unsafe { *error.as_raw().cast::<u8>().add(1) }
}

fn open_core_font(
    connection: &xcb::Connection,
    font: x::Font,
    pattern: &CStr,
) -> (xcb::VoidCookieChecked, x::QueryFontCookie) {
    // Native OpenFont truncated the byte count to u16 before serialization.
    let name = &pattern.to_bytes()[..pattern.to_bytes().len() as u16 as usize];
    let open = connection.send_request_checked(&x::OpenFont { fid: font, name });
    let query = connection.send_request(&x::QueryFont {
        font: x::Fontable::Font(font),
    });
    (open, query)
}

/// # Safety
/// Saved font/reply and display remain live on the event thread.
#[deny(unsafe_op_in_unsafe_fn)]
unsafe fn xcb_query_text_width(text: &[Ucs2Char]) -> i32 {
    static FIRST_INVOCATION: std::sync::atomic::AtomicBool =
        std::sync::atomic::AtomicBool::new(true);
    if FIRST_INVOCATION.swap(false, std::sync::atomic::Ordering::Relaxed) {
        unsafe { fprintf(stderr, c"Using slow code path for text extents\n".as_ptr()) };
    }
    let raw = crate::display::context().connection;
    let connection = crate::display::connection()
        .or_else(|| unsafe { crate::xcb_connection::borrowed(raw.cast()) }.map(std::rc::Rc::new));
    let result = connection.map(|connection| {
        let wire = wire_glyphs(&text[..text.len() as u32 as usize]);
        let cookie = connection.send_request(&x::QueryTextExtents {
            font: x::Fontable::Font(unsafe { x::Font::new((*SAVED_FONT).specific.xcb.id) }),
            string: &wire,
        });
        connection.wait_for_reply(cookie)
    });
    if let Some(Ok(reply)) = result.as_ref() {
        return reply.overall_width();
    }
    let code = match result {
        Some(Err(xcb::Error::Protocol(error))) => protocol_error_code(&error),
        _ => 0,
    };
    unsafe {
        fprintf(
            stderr,
            c"Could not get text extents (X error code %d)\n".as_ptr(),
            i32::from(code),
        );
        ((*(*SAVED_FONT).specific.xcb.info)
            .max_bounds()
            .character_width as usize)
            .wrapping_mul(text.len()) as i32
    }
}

// QueryFont's length counts four-byte units after the first 32 wire bytes, not
// after its full 60-byte fixed header. Properties precede the glyph metrics.
fn font_table_length(info: &x::QueryFontReply) -> Option<usize> {
    let total = usize::try_from(info.length())
        .ok()?
        .checked_mul(4)?
        .checked_add(32)?;
    if total > isize::MAX as usize {
        return None;
    }
    let payload = total.checked_sub(60)?;
    // The maintained variable-payload getters trust counts. Validate both
    // fixed header counts against the declared allocation before borrowing.
    let raw = unsafe { xcb::Reply::as_raw(info) };
    let properties =
        usize::from(unsafe { raw.add(46).cast::<u16>().read_unaligned() }).checked_mul(8)?;
    let count = usize::try_from(unsafe { raw.add(56).cast::<u32>().read_unaligned() }).ok()?;
    let available = payload.checked_sub(properties)?;
    (count <= available / size_of::<x::Charinfo>()).then_some(count)
}

fn metric_width(info: &x::QueryFontReply, table: &[x::Charinfo], glyphs: &[Ucs2Char]) -> i32 {
    let mut width = 0;
    for glyph in glyphs {
        let (row, column) = (glyph.byte1, u16::from(glyph.byte2));
        if row < info.min_byte1()
            || row > info.max_byte1()
            || column < info.min_char_or_byte2()
            || column > info.max_char_or_byte2()
        {
            continue;
        }
        let columns = usize::from(info.max_char_or_byte2() - info.min_char_or_byte2()) + 1;
        let index = usize::from(row - info.min_byte1()) * columns
            + usize::from(column - info.min_char_or_byte2());
        if let Some(metric) = table.get(index) {
            if metric.character_width != 0
                || metric.right_side_bearing != 0
                || metric.left_side_bearing != 0
                || metric.ascent != 0
                || metric.descent != 0
            {
                width += i32::from(metric.character_width);
            }
        }
    }
    width
}

/// # Safety
/// The selected XCB font and its owned query reply must remain live.
#[deny(unsafe_op_in_unsafe_fn)]
unsafe fn predict_text_width_xcb(glyphs: &[Ucs2Char]) -> i32 {
    if glyphs.is_empty() {
        return 0;
    }
    // SAFETY: caller retains the selected font and its uniquely owned Box.
    let info = unsafe { &*(*SAVED_FONT).specific.xcb.info };
    let Some(count) = font_table_length(info) else {
        // Preserve the former null-table slow path for invalid/empty tables.
        return unsafe { xcb_query_text_width(glyphs) };
    };
    if count == 0 {
        return unsafe { xcb_query_text_width(glyphs) };
    }
    metric_width(info, info.char_infos(), glyphs)
}

/// # Safety
/// The selected font (including its complete native query reply/table)
/// must remain valid on the event thread for the duration of the measurement.
pub unsafe fn predict_text_width(text: &I3String) -> i32 {
    if SAVED_FONT.is_null() {
        __assert_fail(
            c"savedFont != NULL".as_ptr(),
            c"../libi3/font.c".as_ptr(),
            467 as u32,
            c"int predict_text_width(i3String *)".as_ptr(),
        );
    };
    match (*SAVED_FONT).type_0 {
        FontType::None => 0,
        FontType::Xcb => predict_text_width_xcb(text.rendering_glyphs()),
        FontType::Pango => predict_text_width_pango(
            text.cstr_or_report()
                .map_or(std::ptr::null(), std::ffi::CStr::as_ptr),
            text.rendering_byte_len(),
            text.is_markup(),
        ),
    }
}

// Internal helpers use Rust linkage rather than foreign symbol lookup.
use crate::dpi::get_dpi_value;
use crate::safewrappers::sstrdup;

// Canonical maintained native types remain part of the legacy POD interface.
use cairo_sys::{cairo_create, cairo_surface_destroy, cairo_xcb_surface_create};
pub use cairo_sys::{cairo_surface_t, cairo_t};
pub use pango_sys::PangoFontDescription;
use pango_sys::{
    pango_font_description_from_string, pango_font_description_get_family, pango_layout_set_markup,
    pango_layout_set_text, PANGO_SCALE,
};

use xcb_sys::{xcb_connection_t, xcb_drawable_t, xcb_font_t, xcb_gcontext_t};

use libc::{fprintf, free};

use crate::native_stdio::{__assert_fail, errx, stderr};

use crate::logging::{errorlog, verboselog};

// Shared model types retain field layout while using one Rust identity.
pub use crate::draw_util::color_t;

// Internal calls and globals use canonical Rust identities.

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum FontType {
    None = 0,
    Xcb = 1,
    Pango = 2,
}

#[cfg(test)]
mod tests {
    use super::*;
    use cairo::{Context, Format, ImageSurface};

    // Reproduce the pre-port native DPI/layout setup independently of the
    // production helper, with maintained owners only for test cleanup.
    fn reference_layout(context: &Context, dpi: f64) -> pango::Layout {
        // SAFETY: context is live; native constructors transfer GObject refs.
        unsafe {
            let native = pango_cairo_sys::pango_cairo_create_context(context.to_raw_none());
            pango_cairo_sys::pango_cairo_context_set_resolution(native, dpi);
            let layout = pango_sys::pango_layout_new(native);
            gobject_sys::g_object_unref(native.cast());
            from_glib_full(layout)
        }
    }

    fn bytes(layout: &pango::Layout) -> Vec<u8> {
        // SAFETY: Pango returns a borrowed, NUL-terminated layout string kept
        // alive by layout; copy its bytes without imposing Rust UTF-8 assumptions.
        unsafe { CStr::from_ptr(pango_sys::pango_layout_get_text(layout.to_glib_none().0)) }
            .to_bytes()
            .to_vec()
    }

    fn render(
        image: &ImageSurface,
        context: &Context,
        layout: &pango::Layout,
        reference: bool,
    ) -> Vec<u8> {
        if reference {
            // SAFETY: every native object is owned by this fixture. These are
            // the original rendering calls, independent of the production helper.
            unsafe {
                cairo_sys::cairo_set_operator(context.to_raw_none(), cairo_sys::OPERATOR_SOURCE);
                cairo_sys::cairo_set_source_rgba(context.to_raw_none(), 1.0, 1.0, 1.0, 1.0);
                pango_cairo_sys::pango_cairo_update_layout(
                    context.to_raw_none(),
                    layout.to_glib_none().0,
                );
                let mut height = 0;
                pango_sys::pango_layout_get_pixel_size(
                    layout.to_glib_none().0,
                    ptr::null_mut(),
                    &mut height,
                );
                cairo_sys::cairo_move_to(
                    context.to_raw_none(),
                    2.0,
                    f64::from(15 - (height - 8) / 2),
                );
                pango_cairo_sys::pango_cairo_show_layout(
                    context.to_raw_none(),
                    layout.to_glib_none().0,
                );
            }
        } else {
            draw_pango_layout(context, layout, (1.0, 1.0, 1.0, 1.0), 2, 15, 8);
        }
        let mut pixels = Vec::new();
        image
            .with_data(|data| pixels.extend_from_slice(data))
            .unwrap();
        pixels
    }

    #[test]
    fn pango_owned_layouts_match_native_measurement_and_rendering_for_legacy_bytes() {
        let inputs: &[(&[u8], usize, bool)] = &[
            (b"plain text ignored\0", 10, false),
            (b"A\0tail\0", 6, false),
            ("é €\0".as_bytes(), 6, false),
            ("é\0".as_bytes(), 1, false), // explicit length truncates UTF-8
            (b"<b>A &amp; B</b>\0", 16, true),
            (b"<b>A</b>\0ignored\0", 16, true),
        ];
        for dpi in [0.0, 96.0, 144.0] {
            for &(input, length, markup) in inputs {
                let image = ImageSurface::create(Format::ARgb32, 160, 50).unwrap();
                let context = Context::new(&image).unwrap();
                let expected_image = ImageSurface::create(Format::ARgb32, 160, 50).unwrap();
                let expected_context = Context::new(&expected_image).unwrap();
                let layout = create_layout_with_dpi(&context, dpi);
                let expected = reference_layout(&expected_context, dpi);
                let description = pango::FontDescription::from_string("monospace 10");
                layout.set_font_description(Some(&description));
                // SAFETY: native reference uses the same live description and buffers.
                unsafe {
                    pango_sys::pango_layout_set_font_description(
                        expected.to_glib_none().0,
                        description.to_glib_none().0,
                    );
                    set_layout_bytes(&layout, input.as_ptr().cast(), length, markup);
                    if markup {
                        pango_layout_set_markup(
                            expected.to_glib_none().0,
                            input.as_ptr().cast(),
                            length as i32,
                        );
                    } else {
                        pango_layout_set_text(
                            expected.to_glib_none().0,
                            input.as_ptr().cast(),
                            length as i32,
                        );
                    }
                }
                // Both native and maintained layout setters copy descriptions;
                // neither layout can depend on the temporary owner's lifetime.
                drop(description);
                assert_eq!(bytes(&layout), bytes(&expected));
                assert_eq!(layout.pixel_size(), expected.pixel_size());
                let width = measure_pango_layout(&context, &layout);
                let mut expected_width = 0;
                // SAFETY: measure the independent live reference through native calls.
                unsafe {
                    pango_cairo_sys::pango_cairo_update_layout(
                        expected_context.to_raw_none(),
                        expected.to_glib_none().0,
                    );
                    pango_sys::pango_layout_get_pixel_size(
                        expected.to_glib_none().0,
                        &mut expected_width,
                        ptr::null_mut(),
                    );
                }
                assert_eq!(width, expected_width);
                let actual = render(&image, &context, &layout, false);
                assert_eq!(
                    actual,
                    render(&expected_image, &expected_context, &expected, true)
                );
                assert!(actual.iter().any(|&byte| byte != 0));
            }
        }
    }

    #[test]
    fn stored_description_borrow_and_transfer_preserve_raw_font_ownership() {
        assert!(!std::mem::needs_drop::<Font>());
        let image = ImageSurface::create(Format::ARgb32, 1, 1).unwrap();
        let context = Context::new(&image).unwrap();
        let layout = create_layout_with_dpi(&context, 96.0);
        let description = pango::FontDescription::from_string("Sans Bold 12");
        let pointer = description.into_glib_ptr();
        let mut font = Font {
            type_0: FontType::Pango,
            height: 0,
            pattern: ptr::null_mut(),
            specific: FontBackendData {
                pango_desc: pointer,
            },
        };
        {
            // SAFETY: the raw font still uniquely owns pointer; borrow ends here.
            let borrowed = unsafe { borrowed_description(&font) };
            assert_eq!(borrowed.as_ref().unwrap().size(), 12 * PANGO_SCALE);
            layout.set_font_description(borrowed.as_deref());
        }
        // SAFETY: reclaim exactly the reference previously transferred into POD.
        drop(unsafe { from_glib_full::<_, pango::FontDescription>(pointer) });
        font.specific.pango_desc = ptr::null_mut();
        assert_eq!(layout.font_description().unwrap().size(), 12 * PANGO_SCALE);
        // SAFETY: null descriptions select Pango defaults rather than constructing
        // an invalid non-null gtk-rs borrowed wrapper.
        let default = unsafe { borrowed_description(&font) };
        assert!(default.is_none());
        layout.set_font_description(default.as_deref());
        assert!(layout.font_description().is_none());
    }

    #[test]
    fn temporary_context_preserves_native_error_objects_without_status_panic() {
        // SAFETY: Cairo returns a non-null error surface/context for invalid sizes.
        unsafe {
            let surface = cairo_sys::cairo_image_surface_create(cairo_sys::FORMAT_A_RGB32, -1, -1);
            let context = font_context(surface);
            assert_eq!(context.status(), Err(cairo::Error::InvalidSize));
            drop(context);
            cairo_surface_destroy(surface);
        }
    }

    #[test]
    fn pango_prefixes_preserve_native_bytes_and_require_nonempty_descriptions() {
        assert_eq!(pango_pattern(c"pango:Sans 10"), Some(c"Sans 10"));
        assert_eq!(pango_pattern(c"xft:monospace"), Some(c"monospace"));
        assert_eq!(pango_pattern(c"pango:"), None);
        assert_eq!(pango_pattern(c"xft:"), None);
        assert_eq!(pango_pattern(c"fixed"), None);
        let invalid_utf8 = CStr::from_bytes_with_nul(b"pango:\xff\0").unwrap();
        assert_eq!(pango_pattern(invalid_utf8).unwrap().to_bytes(), b"\xff");
    }

    fn font_info() -> [u8; 60] {
        let mut header = [0; 60];
        header[0] = 1;
        header
    }

    fn set_u16(header: &mut [u8; 60], offset: usize, value: u16) {
        header[offset..offset + 2].copy_from_slice(&value.to_ne_bytes());
    }
    fn set_u32(header: &mut [u8; 60], offset: usize, value: u32) {
        header[offset..offset + 4].copy_from_slice(&value.to_ne_bytes());
    }
    fn owned_font_info(header: &[u8; 60]) -> x::QueryFontReply {
        let words = u32::from_ne_bytes(header[4..8].try_into().unwrap()) as usize;
        // Allocate the complete advertised reply, retaining a full fixed header
        // even when the deliberately malformed length claims it is shorter.
        unsafe {
            let raw = libc::calloc(1, (32 + words * 4).max(60)).cast::<u8>();
            assert!(!raw.is_null());
            std::ptr::copy_nonoverlapping(header.as_ptr(), raw, header.len());
            xcb::Reply::from_raw(raw)
        }
    }

    fn metric(width: i16) -> x::Charinfo {
        x::Charinfo {
            left_side_bearing: 0,
            right_side_bearing: 0,
            character_width: width,
            ascent: 0,
            descent: 0,
            attributes: 0,
        }
    }

    #[test]
    fn font_table_bounds_include_fixed_header_and_properties_before_glyphs() {
        let mut info = font_info();
        assert_eq!(font_table_length(&owned_font_info(&info)), None);
        set_u32(&mut info, 4, 7);
        assert_eq!(font_table_length(&owned_font_info(&info)), Some(0));
        set_u32(&mut info, 56, 1);
        assert_eq!(font_table_length(&owned_font_info(&info)), None);
        set_u32(&mut info, 4, 10); // 60-byte header plus one 12-byte glyph
        assert_eq!(font_table_length(&owned_font_info(&info)), Some(1));
        set_u16(&mut info, 46, 1); // property consumes eight bytes
        assert_eq!(font_table_length(&owned_font_info(&info)), None);
        set_u32(&mut info, 4, 12);
        assert_eq!(font_table_length(&owned_font_info(&info)), Some(1));
        set_u32(&mut info, 56, u32::MAX);
        assert_eq!(font_table_length(&owned_font_info(&info)), None);
    }

    #[test]
    fn xcb_metric_rows_missing_glyphs_and_signed_advances_use_checked_indices() {
        let mut header = font_info();
        header[49] = 1;
        header[50] = 2;
        set_u16(&mut header, 40, 10);
        set_u16(&mut header, 42, 11);
        let info = owned_font_info(&header);
        let table = [metric(-3), metric(4), metric(0), metric(9)];
        let glyphs = [
            Ucs2Char {
                byte1: 1,
                byte2: 10,
            },
            Ucs2Char {
                byte1: 1,
                byte2: 11,
            },
            Ucs2Char {
                byte1: 2,
                byte2: 11,
            },
            Ucs2Char {
                byte1: 0,
                byte2: 10,
            },
            Ucs2Char {
                byte1: 1,
                byte2: 12,
            },
        ];
        assert_eq!(metric_width(&info, &table, &glyphs), 10);
        assert_eq!(metric_width(&info, &table[..3], &glyphs), 1);
        assert_eq!(metric_width(&info, &[], &glyphs), 0);
        assert_eq!(metric_width(&info, &table, &[]), 0);
    }
}
