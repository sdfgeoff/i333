//! Owned font resources and explicit, borrowed rendering operations.
#![deny(unsafe_op_in_unsafe_fn)]

pub use crate::draw_util::color_t;
use crate::native_stdio::{errx, stderr};
use crate::{
    display::{CairoVisual, DisplayScreen},
    dpi::get_dpi_value,
    string::I3String,
    ucs2_conversion::Ucs2Char,
};
use cairo_sys::{cairo_create, cairo_surface_destroy, cairo_xcb_surface_create};
pub use cairo_sys::{cairo_surface_t, cairo_t};
use glib::translate::{from_glib_full, ToGlibPtr};
use libc::fprintf;
use pango_sys::{
    pango_font_description_from_string, pango_font_description_get_family, pango_layout_set_markup,
    pango_layout_set_text, PANGO_SCALE,
};
use std::{
    ffi::{CStr, CString},
    mem::size_of,
    ptr,
    rc::Rc,
};
use xcb::ffi::xcb_connection_t;
use xcb::{x, Xid, XidNew};

/// Display resources used to open and measure fonts, retained independently of
/// ambient providers. DPI remains read at measurement time, as in native i3.
#[derive(Clone)]
pub struct FontContext {
    screen: DisplayScreen,
    visual: Option<Rc<CairoVisual>>,
}
impl FontContext {
    pub fn for_screen(screen: &DisplayScreen) -> Self {
        Self {
            screen: screen.clone(),
            visual: screen.visual(screen.info().root_visual).map(Rc::new),
        }
    }
    fn connection(&self) -> &xcb::Connection {
        self.screen.connection()
    }
}

/// Owns every backend resource. Fonts must be moved, never byte-copied or zeroed.
pub struct Font {
    pub height: i32,
    pattern: Option<CString>,
    backend: FontBackend,
}
enum FontBackend {
    None,
    Pango(PangoFont),
    Xcb(CoreFont),
}
struct PangoFont {
    description: pango::FontDescription,
    context: FontContext,
}
struct CoreFont {
    context: FontContext,
    id: x::Font,
    info: x::QueryFontReply,
}
impl Drop for CoreFont {
    fn drop(&mut self) {
        // The retained single connection owner stays live through CloseFont.
        self.context
            .connection()
            .send_request(&x::CloseFont { font: self.id });
    }
}
impl Default for Font {
    fn default() -> Self {
        Self::empty()
    }
}
impl Font {
    pub const fn empty() -> Self {
        Self {
            height: 0,
            pattern: None,
            backend: FontBackend::None,
        }
    }
    pub fn is_loaded(&self) -> bool {
        !matches!(self.backend, FontBackend::None)
    }
    pub fn is_pango(&self) -> bool {
        matches!(self.backend, FontBackend::Pango(_))
    }
    pub fn pattern(&self) -> Option<&CStr> {
        self.pattern.as_deref()
    }
    /// Borrowed core-font details for protocol diagnostics and native comparison.
    pub fn xcb_details(&self) -> Option<(x::Font, &x::QueryFontReply)> {
        match &self.backend {
            FontBackend::Xcb(font) => Some((font.id, &font.info)),
            _ => None,
        }
    }
    /// Replace a server metric snapshot while retaining allocation ownership.
    /// Returns the previous snapshot; the font XID/connection stay unchanged.
    pub fn replace_xcb_metrics(&mut self, info: x::QueryFontReply) -> Option<x::QueryFontReply> {
        match &mut self.backend {
            FontBackend::Xcb(font) => Some(std::mem::replace(&mut font.info, info)),
            _ => None,
        }
    }
}

pub fn font_is_pango(font: &Font) -> bool {
    font.is_pango()
}

/// Drop the previous owner before opening its replacement, preserving request
/// ordering. Copy the bytes first in case a native caller aliases the old pattern.
pub fn replace_font(font: &mut Font, context: &FontContext, pattern: &CStr, fallback: bool) {
    let pattern = pattern.to_owned();
    *font = Font::empty();
    *font = load_font(context, &pattern, fallback);
}

fn create_layout_with_dpi(context: &cairo::Context, dpi: f64) -> pango::Layout {
    let pango_context = pangocairo::functions::create_context(context);
    pangocairo::functions::context_set_resolution(&pango_context, dpi);
    pango::Layout::new(&pango_context)
}

/// Native Pango byte setter, retaining explicit byte counts and invalid UTF-8.
/// # Safety
/// text is readable for len as i32, or NUL-terminated when that count is negative.
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
/// The connection, root and visual owner must stay live through the returned surface.
unsafe fn font_surface(
    conn: *mut xcb_connection_t,
    root: u32,
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
/// surface is a live non-null Cairo surface, including a native error object.
unsafe fn font_context(surface: *mut cairo_surface_t) -> cairo::Context {
    // SAFETY: cairo_create returns a non-null context even for native errors;
    // from_raw_full owns that reference without rejecting its sticky error.
    unsafe { cairo::Context::from_raw_full(cairo_create(surface)) }
}

fn measurement_context(font: &PangoFont) -> cairo::Context {
    // SAFETY: the font retains its live display and aligned native visual owner.
    let surface = unsafe {
        font_surface(
            font.context.connection().get_raw_conn(),
            font.context.screen.info().root,
            font.context
                .visual
                .as_ref()
                .expect("root Cairo visual")
                .as_ptr(),
        )
    };
    // Cairo's context retains its own reference to the local owned surface.
    unsafe { font_context(surface.to_raw_none()) }
}

fn load_pango_font(context: &FontContext, desc: &CStr) -> Option<(i32, PangoFont)> {
    // Keep native non-UTF8 description bytes; the &str factory cannot express them.
    let pointer = unsafe { pango_font_description_from_string(desc.as_ptr()) };
    if pointer.is_null() {
        {
            let log_text_bytes = (desc.to_bytes()).to_vec();
            match std::str::from_utf8(&log_text_bytes) {
                Ok(log_text) => ::log::error!(
                    "[libi3] ERROR: Could not open font {} with Pango, fallback to X font.\n",
                    log_text
                ),
                Err(_) => {
                    let mut message = b"[libi3] ERROR: Could not open font ".to_vec();
                    message.extend_from_slice(&log_text_bytes);
                    message.extend_from_slice(b" with Pango, fallback to X font.\n");
                    crate::logging::emit(crate::logging::Level::Error, &message);
                }
            }
        };
        return None;
    }
    let description: pango::FontDescription = unsafe { from_glib_full(pointer) };
    {
        let log_text_bytes = ({
            let family = unsafe { pango_font_description_get_family(pointer) };
            let family = if family.is_null() {
                None
            } else {
                Some(unsafe { CStr::from_ptr(family) })
            };
            crate::logging::string_bytes(family, None)
        })
        .to_vec();
        let log_number = description.size() / PANGO_SCALE;
        match std::str::from_utf8(&log_text_bytes) {
            Ok(log_text) => ::log::info!(
                "[libi3] ../libi3/font.c Using Pango font {}, size {}\n",
                log_text,
                log_number
            ),
            Err(_) => {
                let mut message = b"[libi3] ../libi3/font.c Using Pango font ".to_vec();
                message.extend_from_slice(&log_text_bytes);
                message.extend_from_slice(format!(", size {}\n", log_number).as_bytes());
                crate::logging::emit(crate::logging::Level::Verbose, &message);
            }
        }
    };
    let font = PangoFont {
        description,
        context: context.clone(),
    };
    let context = measurement_context(&font);
    let layout = create_layout_with_dpi(&context, get_dpi_value() as f64);
    layout.set_font_description(Some(&font.description));
    Some((layout.pixel_size().1, font))
}

/// # Safety
/// The supplied Cairo surface and native text byte span remain live during drawing.
unsafe fn draw_text_pango(
    font: &Font,
    backend: &PangoFont,
    text: *const libc::c_char,
    text_len: usize,
    surface: *mut cairo_surface_t,
    foreground: color_t,
    x: i32,
    y: i32,
    max_width: i32,
    markup: bool,
) {
    let context = unsafe { font_context(surface) };
    let layout = create_layout_with_dpi(&context, get_dpi_value() as f64);
    layout.set_font_description(Some(&backend.description));
    layout.set_width(max_width * PANGO_SCALE);
    layout.set_wrap(pango::WrapMode::Char);
    layout.set_ellipsize(pango::EllipsizeMode::End);
    layout.set_single_paragraph_mode(true);
    unsafe { set_layout_bytes(&layout, text, text_len, markup) };
    draw_pango_layout(
        &context,
        &layout,
        (
            foreground.red,
            foreground.green,
            foreground.blue,
            foreground.alpha,
        ),
        x,
        y,
        font.height,
    );
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
/// text satisfies the explicit native Pango byte-span contract.
unsafe fn predict_text_width_pango(
    font: &PangoFont,
    text: *const libc::c_char,
    text_len: usize,
    markup: bool,
) -> i32 {
    let context = measurement_context(font);
    let layout = create_layout_with_dpi(&context, get_dpi_value() as f64);
    layout.set_font_description(Some(&font.description));
    unsafe { set_layout_bytes(&layout, text, text_len, markup) };
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

/// Open an owned font on the explicit retained display. Pattern and fallback
/// bytes remain native C strings; no provider or registry is consulted.
pub fn load_font(context: &FontContext, pattern: &CStr, fallback: bool) -> Font {
    let connection = context.connection();
    if let Some(description) = pango_pattern(pattern) {
        if let Some((height, backend)) = load_pango_font(context, description) {
            return Font {
                height,
                pattern: Some(pattern.to_owned()),
                backend: FontBackend::Pango(backend),
            };
        }
    }
    let mut pattern = pattern;
    let id: x::Font = connection.generate_id();
    let (font_cookie, mut info_cookie) = open_core_font(&connection, id, pattern);
    let mut error = connection.check_request(font_cookie).err();
    if fallback && error.is_some() {
        {
            let log_text_bytes = (pattern.to_bytes()).to_vec();
            let log_number = i32::from(error.as_ref().map_or(0, protocol_error_code));
            match std::str::from_utf8(&log_text_bytes) {
                Ok(log_text) => ::log::error!(
                    concat!(
                        "[libi3] ERROR: Could not open font {} (X error {}). ",
                        "Trying fallback to 'fixed'.\n"
                    ),
                    log_text,
                    log_number
                ),
                Err(_) => {
                    let mut message = b"[libi3] ERROR: Could not open font ".to_vec();
                    message.extend_from_slice(&log_text_bytes);
                    message.extend_from_slice(
                        format!(" (X error {}). Trying fallback to 'fixed'.\n", log_number)
                            .as_bytes(),
                    );
                    crate::logging::emit(crate::logging::Level::Error, &message);
                }
            }
        };
        pattern = c"fixed";
        let (font_cookie, cookie) = open_core_font(&connection, id, pattern);
        info_cookie = cookie;
        error = connection.check_request(font_cookie).err();
        if error.is_some() {
            ::log::error!(
                "[libi3] ERROR: Could not open fallback font 'fixed', trying with '-misc-*'.\n"
            );
            pattern = c"-misc-*";
            let (font_cookie, cookie) = open_core_font(&connection, id, pattern);
            info_cookie = cookie;
            error = connection.check_request(font_cookie).err();
            if error.is_some() {
                unsafe {
                    errx(
                    libc::EXIT_FAILURE,
                    c"Could open neither requested font nor fallbacks (fixed or -misc-*): X11 error %d".as_ptr(),
                    i32::from(error.as_ref().map_or(0, protocol_error_code)),
                )
                };
            }
        }
    }
    drop(error);
    {
        let log_text_bytes = (pattern.to_bytes()).to_vec();
        match std::str::from_utf8(&log_text_bytes) {
            Ok(log_text) => ::log::info!("[libi3] ../libi3/font.c Using X font {}\n", log_text),
            Err(_) => {
                let mut message = b"[libi3] ../libi3/font.c Using X font ".to_vec();
                message.extend_from_slice(&log_text_bytes);
                message.extend_from_slice(b"\n");
                crate::logging::emit(crate::logging::Level::Verbose, &message);
            }
        }
    };
    let Ok(info) = connection.wait_for_reply(info_cookie) else {
        unsafe {
            errx(
                libc::EXIT_FAILURE,
                c"Could not load font \"%s\"".as_ptr(),
                pattern.as_ptr(),
            )
        };
    };
    if info.length() < 7 {
        unsafe {
            errx(
                libc::EXIT_FAILURE,
                c"Could not load font \"%s\"".as_ptr(),
                pattern.as_ptr(),
            )
        };
    }
    let height = i32::from(info.font_ascent()) + i32::from(info.font_descent());
    Font {
        height,
        pattern: Some(pattern.to_owned()),
        backend: FontBackend::Xcb(CoreFont {
            context: context.clone(),
            id,
            info,
        }),
    }
}

fn draw_text_xcb(font: &CoreFont, glyphs: &[Ucs2Char], drawable: u32, gc: u32, mut x: i32, y: i32) {
    let connection = font.context.connection();
    let pos_y = y + i32::from(font.info.font_ascent());
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
            gc: x::Gcontext::new(gc),
            x: x as i16,
            y: pos_y as i16,
            string: &wire[..chunk.len()],
        });
        if chunks.peek().is_some() {
            x += predict_text_width_xcb(font, chunk);
        }
    }
}

/// Draw using this explicit font and colors; no selected-font/color registry is
/// accessed, including during synchronous emergency rendering.
/// # Safety
/// The supplied Cairo surface is live/non-null for Pango; drawable/GC identify
/// resources on this font's retained display for the core backend.
pub unsafe fn draw_text(
    font: &Font,
    text: &I3String,
    drawable: u32,
    gc: u32,
    surface: *mut cairo_surface_t,
    foreground: color_t,
    background: color_t,
    x: i32,
    y: i32,
    max_width: i32,
) {
    match &font.backend {
        FontBackend::None => {}
        FontBackend::Xcb(backend) => {
            backend.context.connection().send_request(&x::ChangeGc {
                gc: x::Gcontext::new(gc),
                value_list: &[
                    x::Gc::Foreground(foreground.colorpixel),
                    x::Gc::Background(background.colorpixel),
                    x::Gc::Font(backend.id),
                ],
            });
            draw_text_xcb(backend, text.rendering_glyphs(), drawable, gc, x, y);
        }
        FontBackend::Pango(backend) => unsafe {
            draw_text_pango(
                font,
                backend,
                text.cstr_or_report().map_or(ptr::null(), CStr::as_ptr),
                text.rendering_byte_len(),
                surface,
                foreground,
                x,
                y,
                max_width,
                text.is_markup(),
            )
        },
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

fn xcb_query_text_width(font: &CoreFont, text: &[Ucs2Char]) -> i32 {
    static FIRST_INVOCATION: std::sync::atomic::AtomicBool =
        std::sync::atomic::AtomicBool::new(true);
    if FIRST_INVOCATION.swap(false, std::sync::atomic::Ordering::Relaxed) {
        unsafe { fprintf(stderr, c"Using slow code path for text extents\n".as_ptr()) };
    }
    let wire = wire_glyphs(&text[..text.len() as u32 as usize]);
    let cookie = font
        .context
        .connection()
        .send_request(&x::QueryTextExtents {
            font: x::Fontable::Font(font.id),
            string: &wire,
        });
    let result = font.context.connection().wait_for_reply(cookie);
    if let Ok(reply) = result.as_ref() {
        return reply.overall_width();
    }
    let code = match result {
        Err(xcb::Error::Protocol(error)) => protocol_error_code(&error),
        _ => 0,
    };
    unsafe {
        fprintf(
            stderr,
            c"Could not get text extents (X error code %d)\n".as_ptr(),
            i32::from(code),
        );
        (font.info.max_bounds().character_width as usize).wrapping_mul(text.len()) as i32
    }
}

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

fn predict_text_width_xcb(font: &CoreFont, glyphs: &[Ucs2Char]) -> i32 {
    if glyphs.is_empty() {
        return 0;
    }
    // The borrowed backend retains the complete owned metric reply.
    let info = &font.info;
    let Some(count) = font_table_length(info) else {
        // Preserve the former null-table slow path for invalid/empty tables.
        return xcb_query_text_width(font, glyphs);
    };
    if count == 0 {
        return xcb_query_text_width(font, glyphs);
    }
    metric_width(info, info.char_infos(), glyphs)
}

/// Measure with a borrowed owning font. Native replies and display/visual owners
/// remain live through the borrow; no mutable selected-font global is needed.
pub fn predict_text_width(font: &Font, text: &I3String) -> i32 {
    match &font.backend {
        FontBackend::None => 0,
        FontBackend::Xcb(backend) => predict_text_width_xcb(backend, text.rendering_glyphs()),
        FontBackend::Pango(backend) => unsafe {
            predict_text_width_pango(
                backend,
                text.cstr_or_report().map_or(ptr::null(), CStr::as_ptr),
                text.rendering_byte_len(),
                text.is_markup(),
            )
        },
    }
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
    fn description_owner_and_layout_hold_independent_native_references() {
        assert!(std::mem::needs_drop::<Font>());
        let image = ImageSurface::create(Format::ARgb32, 1, 1).unwrap();
        let context = Context::new(&image).unwrap();
        let layout = create_layout_with_dpi(&context, 96.0);
        let description = pango::FontDescription::from_string("Sans Bold 12");
        layout.set_font_description(Some(&description));
        drop(description);
        assert_eq!(layout.font_description().unwrap().size(), 12 * PANGO_SCALE);
        layout.set_font_description(None);
        assert!(layout.font_description().is_none());
        assert!(!Font::empty().is_loaded());
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
