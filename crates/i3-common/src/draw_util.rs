#![deny(unsafe_op_in_unsafe_fn)]
mod rendering;
pub use rendering::{
    GraphicsContext, NativeRenderingContext, OwnedPixmap, OwnedWindow, RenderingContext,
};

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

use crate::string::I3String;
use xcb::{x, Xid, XidNew};
#[derive(Copy, Clone)]
#[repr(C)]
pub struct color_t {
    pub red: ::core::ffi::c_double,
    pub green: ::core::ffi::c_double,
    pub blue: ::core::ffi::c_double,
    pub alpha: ::core::ffi::c_double,
    pub colorpixel: u32,
}
/// One owned native Cairo surface reference, including Cairo error surfaces.
///
/// Cairo's fallible `Surface::from_raw_full` rejects error objects. The native
/// drawing contract keeps those objects and their sticky status, so adoption
/// uses a reference-preserving conversion instead. No reference can be copied
/// implicitly; shared Cairo borrows remain tied to this owner.
#[derive(Debug)]
pub struct CairoSurface(cairo::Surface);

#[deny(unsafe_op_in_unsafe_fn)]
impl CairoSurface {
    /// Adopt one complete native reference without rejecting its error status.
    ///
    /// # Safety
    /// `pointer` must be non-null and transfer one live Cairo reference. The
    /// caller must neither destroy that reference nor use it after this owner.
    pub unsafe fn from_raw_owned(pointer: *mut cairo_sys::cairo_surface_t) -> Self {
        assert!(
            !pointer.is_null(),
            "a Cairo surface reference must be non-null"
        );
        // SAFETY: the caller transfers a valid reference. from_raw_none keeps
        // error objects; releasing the original leaves exactly one owned ref.
        let surface = unsafe { cairo::Surface::from_raw_none(pointer) };
        unsafe { cairo_sys::cairo_surface_destroy(pointer) };
        Self(surface)
    }

    pub fn from_surface(surface: cairo::Surface) -> Self {
        Self(surface)
    }

    pub fn as_raw(&self) -> *mut cairo_sys::cairo_surface_t {
        self.0.to_raw_none()
    }

    pub fn as_surface(&self) -> &cairo::Surface {
        &self.0
    }
}

/// An owned pair of Cairo drawing references for a borrowed server drawable.
/// Drawable IDs survive `clear` because reconfiguration reuses existing windows
/// and pixmaps. Clearing releases only the Cairo references and uncached GC.
/// The server drawable remains the creator's responsibility.
pub struct surface_t {
    pub id: u32,
    pub gc: u32,
    pub width: i32,
    pub height: i32,
    resources: Option<SurfaceResources>,
    pixmap: Option<OwnedPixmap>,
}

impl Default for surface_t {
    fn default() -> Self {
        Self::empty()
    }
}
impl surface_t {
    pub const fn empty() -> Self {
        Self {
            id: 0,
            gc: 0,
            width: 0,
            height: 0,
            resources: None,
            pixmap: None,
        }
    }
    pub fn attach_pixmap(&mut self, owner: OwnedPixmap) {
        assert!(
            self.pixmap.is_none(),
            "release the old pixmap owner before replacement"
        );
        assert_eq!(
            owner.pixmap().map(|pixmap| pixmap.resource_id()),
            Some(self.id)
        );
        self.pixmap = Some(owner);
    }
    pub fn release_pixmap(&mut self) -> bool {
        let owner = self.pixmap.take();
        let owned = owner.is_some();
        drop(owner);
        owned
    }
    pub fn has_resources(&self) -> bool {
        self.resources.is_some()
    }
    pub fn clear(&mut self) {
        // Drop the detached owner after taking it, making repeated clear safe.
        drop(self.resources.take());
    }
    /// Adopt complete native drawing references, including Cairo error objects.
    /// # Safety
    /// Both non-null pointers transfer one live reference; all native display
    /// and visual lifetimes must cover this record and any derived native refs.
    #[deny(unsafe_op_in_unsafe_fn)]
    pub unsafe fn from_raw_cairo(
        id: u32,
        width: i32,
        height: i32,
        surface: *mut cairo_surface_t,
        context: *mut cairo_t,
    ) -> Self {
        let surface = unsafe { CairoSurface::from_raw_owned(surface) };
        let context = unsafe { cairo::Context::from_raw_full(context) };
        Self::from_cairo(id, width, height, surface, context)
    }
    pub fn context_status(&self) -> cairo_status_t {
        self.resources.as_ref().map_or(STATUS_SUCCESS, |resources| {
            // SAFETY: the native context reference is retained by this record.
            unsafe { cairo_status(resources.context.to_raw_none()) }
        })
    }
    /// Create owned graphics for a borrowed drawable, retaining the connection
    /// and aligned visual through Cairo teardown and any uncached GC release.
    pub fn for_drawable(
        context: std::rc::Rc<RenderingContext>,
        drawable: u32,
        visual: std::rc::Rc<crate::display::CairoVisual>,
        width: i32,
        height: i32,
    ) -> Self {
        let depth = context.visual_depth(visual.visual_id());
        let (gc, lease) = context.acquire_gc(drawable, depth);
        // SAFETY: both connection and aligned visual owners remain retained in
        // this record after the native references and GC fields are destroyed.
        let target = unsafe {
            CairoSurface::from_raw_owned(cairo_xcb_surface_create(
                context.screen().connection().get_raw_conn().cast(),
                drawable,
                visual.as_ptr(),
                width,
                height,
            ))
        };
        let cr = unsafe { cairo::Context::from_raw_full(cairo_create(target.as_raw())) };
        Self {
            id: drawable,
            gc,
            width,
            height,
            resources: Some(SurfaceResources {
                gc: lease,
                surface: target,
                context: cr,
                retained: Some((context, visual)),
            }),
            pixmap: None,
        }
    }
    pub fn from_cairo(
        id: u32,
        width: i32,
        height: i32,
        surface: CairoSurface,
        context: cairo::Context,
    ) -> Self {
        Self {
            id,
            gc: 0,
            width,
            height,
            resources: Some(SurfaceResources {
                gc: GcLease::Borrowed,
                surface,
                context,
                retained: None,
            }),
            pixmap: None,
        }
    }
}
impl Drop for surface_t {
    fn drop(&mut self) {
        self.clear();
        self.release_pixmap();
    }
}

// Cached GCs are borrowed, while a third-depth GC belongs to this surface.
// Unsafe native initialization requires the connection to remain live through
// clear/drop, including independent placeholder and emergency connections.
enum GcLease {
    Borrowed,
    Owned(OwnedGraphicsContext),
}

/// One uncached server GC. Cached GCs are borrowed and never use this owner.
/// Native/independent connection lifetimes are explicit at the adoption boundary.
pub struct OwnedGraphicsContext {
    connection: *mut xcb_connection_t,
    id: u32,
}
impl OwnedGraphicsContext {
    /// # Safety
    /// `id` transfers unique ownership of a live GC; `connection` must remain
    /// live until this owner is dropped. Never adopt a borrowed cache entry.
    pub unsafe fn from_raw_owned(connection: *mut xcb_connection_t, id: u32) -> Self {
        Self { connection, id }
    }
    pub fn id(&self) -> u32 {
        self.id
    }
}
impl Drop for OwnedGraphicsContext {
    fn drop(&mut self) {
        // SAFETY: adoption requires the caller's display owner to outlive this
        // resource. The native borrowed view never clones a registry/Rc owner.
        if let Some(connection) = unsafe { crate::xcb_connection::borrowed(self.connection) } {
            connection.send_request(&x::FreeGc {
                gc: x::Gcontext::new(self.id),
            });
        }
    }
}
struct SurfaceResources {
    // Field destruction preserves native order: GC, surface, then context.
    gc: GcLease,
    surface: CairoSurface,
    context: cairo::Context,
    // Retained only on the normal owned path, after all native drawing refs.
    retained: Option<(
        std::rc::Rc<RenderingContext>,
        std::rc::Rc<crate::display::CairoVisual>,
    )>,
}
impl Drop for SurfaceResources {
    fn drop(&mut self) {
        // SAFETY: this owned Cairo context remains live through field teardown;
        // preserve numeric native error diagnostics without rejecting the object.
        unsafe {
            let status = cairo_status(self.context.to_raw_none());
            if status != STATUS_SUCCESS {
                let description = std::ffi::CStr::from_ptr(cairo_status_to_string(status))
                    .to_str()
                    .expect("Cairo status descriptions are ASCII")
                    .to_owned();
                ::log::info!("[libi3] ../libi3/draw_util.c Found cairo context in an error status while freeing, error {} is {}", status as i32, description);
            }
        }
    }
}
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
fn surface_initialized(surface: &surface_t) -> bool {
    if surface.id == 0 || surface.resources.is_none() {
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
pub unsafe fn get_visual_depth(visual_id: u32) -> u16 {
    let crate::display::DisplayContext {
        connection, screen, ..
    } = crate::display::context();
    let Some(screen_info) = screen else {
        return 0;
    };
    let Some(connection) = (unsafe { crate::xcb_connection::borrowed(connection) }) else {
        return 0;
    };
    // Select the exact registered screen instead of assuming setup screen zero.
    let Some(screen) = connection
        .get_setup()
        .roots()
        .find(|screen| screen.root().resource_id() == screen_info.root)
    else {
        return 0;
    };
    screen
        .allowed_depths()
        .find_map(|depth| {
            depth
                .visuals()
                .iter()
                .any(|visual| visual.visual_id() == visual_id)
                .then(|| u16::from(depth.depth()))
        })
        .unwrap_or(0)
}

/// Initialize owned native graphics for a borrowed server drawable.
///
/// # Safety
/// The record must be exclusively accessible Rust storage. The explicit context
/// and selected visual are retained through graphics teardown. Emergency native
/// callers use draw_util_surface_init_native instead of this Rc-backed path.
pub unsafe fn draw_util_surface_init(
    renderer: &std::rc::Rc<RenderingContext>,
    surface: *mut surface_t,
    drawable: u32,
    visual: Option<&crate::display::CairoVisual>,
    width: i32,
    height: i32,
) {
    let surface = unsafe { &mut *surface };
    surface.clear();
    let visual_id = visual.map_or(
        renderer.default_visual(),
        crate::display::CairoVisual::visual_id,
    );
    let visual = renderer
        .visual(visual_id)
        .expect("render visual belongs to the retained setup");
    let mut next = surface_t::for_drawable(renderer.clone(), drawable, visual, width, height);
    // Preserve an attached pixmap owner through Cairo-only reload/reinit.
    surface.id = next.id;
    surface.gc = next.gc;
    surface.width = width;
    surface.height = height;
    surface.resources = next.resources.take();
}

/// # Safety
/// Native display/visual owners must cover all derived surfaces. This path is
/// exclusive emergency state and never accesses a registry, Rc or RefCell.
pub unsafe fn draw_util_surface_init_native(
    renderer: &mut NativeRenderingContext,
    surface: *mut surface_t,
    drawable: u32,
    width: i32,
    height: i32,
) {
    let surface = unsafe { &mut *surface };
    surface.clear();
    let gc = renderer.prepared_gc();
    let target = unsafe {
        CairoSurface::from_raw_owned(cairo_xcb_surface_create(
            renderer.connection().cast(),
            drawable,
            renderer.visual().as_ptr(),
            width,
            height,
        ))
    };
    let context = unsafe { cairo::Context::from_raw_full(cairo_create(target.as_raw())) };
    surface.id = drawable;
    surface.gc = gc;
    surface.width = width;
    surface.height = height;
    surface.resources = Some(SurfaceResources {
        gc: GcLease::Borrowed,
        surface: target,
        context,
        retained: None,
    });
}

/// # Safety
/// `surface` is exclusively accessible live Rust storage. Its display owners
/// still satisfy the initialization lifetime contract. Repeated clears are safe.
pub unsafe fn draw_util_surface_free(_conn: *mut xcb_connection_t, surface: *mut surface_t) {
    unsafe { (&mut *surface).clear() };
}

/// # Safety
/// `surface` is exclusively accessible live Rust storage.
pub unsafe fn draw_util_surface_set_size(surface: *mut surface_t, width: i32, height: i32) {
    let surface = unsafe { &mut *surface };
    surface.width = width;
    surface.height = height;
    if let Some(resources) = &surface.resources {
        // SAFETY: the owner keeps even error surfaces live for the native call.
        unsafe { cairo_xcb_surface_set_size(resources.surface.as_raw(), width, height) };
    }
}

pub unsafe fn draw_util_hex_to_color(color: *const ::core::ffi::c_char) -> color_t {
    // SAFETY: The caller supplies a live NUL-terminated native color string, including its terminator.
    unsafe {
        if strlen(color) < 6_usize
            || *color.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '#' as i32
        {
            {
                let log_text_bytes = (std::ffi::CStr::from_ptr(color).to_bytes()).to_vec();
                match std::str::from_utf8(&log_text_bytes) {
                    Ok(log_text) => {
                        ::log::error!("[libi3] ERROR: Could not parse color: {}\n", log_text)
                    }
                    Err(_) => {
                        let mut message = b"[libi3] ERROR: Could not parse color: ".to_vec();
                        message.extend_from_slice(&log_text_bytes);
                        message.extend_from_slice(b"\n");
                        crate::logging::emit(crate::logging::Level::Error, &message);
                    }
                }
            };
            return draw_util_hex_to_color(c"#A9A9A9".as_ptr());
        }
        let mut alpha: [::core::ffi::c_char; 2] = [0; 2];
        if strlen(color) == strlen(c"#rrggbbaa".as_ptr()) {
            alpha[0 as ::core::ffi::c_int as usize] =
                *color.offset(7 as ::core::ffi::c_int as isize);
            alpha[1 as ::core::ffi::c_int as usize] =
                *color.offset(8 as ::core::ffi::c_int as isize);
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
                    .offset(0 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_char,
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                16 as ::core::ffi::c_int,
            ) as ::core::ffi::c_double
                / 255.0f64,
            green: strtol(
                &raw mut *(&raw mut groups as *mut [::core::ffi::c_char; 3])
                    .offset(1 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_char,
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                16 as ::core::ffi::c_int,
            ) as ::core::ffi::c_double
                / 255.0f64,
            blue: strtol(
                &raw mut *(&raw mut groups as *mut [::core::ffi::c_char; 3])
                    .offset(2 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_char,
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                16 as ::core::ffi::c_int,
            ) as ::core::ffi::c_double
                / 255.0f64,
            alpha: strtol(
                &raw mut *(&raw mut groups as *mut [::core::ffi::c_char; 3])
                    .offset(3 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_char,
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                16 as ::core::ffi::c_int,
            ) as ::core::ffi::c_double
                / 255.0f64,
            colorpixel: get_colorpixel(color),
        }
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
/// surface must point to a readable record.
/// An initialized surface must contain live non-null Cairo objects and a valid
/// drawable/GC, and the current font/display must remain valid during drawing.
#[deny(unsafe_op_in_unsafe_fn)]
pub unsafe fn draw_util_text(
    font: &crate::font::Font,
    text: &I3String,
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
    let target = surface.resources.as_ref().unwrap().surface.as_surface();
    flush_twice(target);
    // SAFETY: text, font, drawable and GC validity are the caller's contract.
    unsafe {
        draw_text(
            font,
            text,
            surface.id,
            surface.gc,
            surface.resources.as_ref().unwrap().surface.as_raw(),
            fg_color,
            bg_color,
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
    let context = &surface.resources.as_ref().unwrap().context;
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
    let context = &surface.resources.as_ref().unwrap().context;
    let target = surface.resources.as_ref().unwrap().surface.as_surface();
    rectangle(context, target, color, x, y, w, h);
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
    let context = &surface.resources.as_ref().unwrap().context;
    let target = surface.resources.as_ref().unwrap().surface.as_surface();
    clear(context, target, color);
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
    let context = &dest.resources.as_ref().unwrap().context;
    let source = src.resources.as_ref().unwrap().surface.as_surface();
    let target = dest.resources.as_ref().unwrap().surface.as_surface();
    copy_surface(
        source, target, context, src_x, src_y, dest_x, dest_y, width, height,
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

use xcb::ffi::xcb_connection_t;

pub use libc::{strlen, strtol};

// Internal calls and globals use canonical Rust identities.

#[cfg(test)]
mod drawing_tests {
    use super::*;
    use cairo::{Context, Format, ImageSurface, Operator};

    #[test]
    fn surface_clear_and_drop_release_each_reference_once_and_keep_drawable_identity() {
        let image = ImageSurface::create(Format::ARgb32, 1, 1).unwrap();
        let context = Context::new(&image).unwrap();
        let before = unsafe {
            (
                cairo_sys::cairo_get_reference_count(context.to_raw_none()),
                cairo_sys::cairo_surface_get_reference_count(image.to_raw_none()),
            )
        };
        let mut owned = record(&image, &context);
        owned.id = 73;
        owned.gc = 19;
        assert!(owned.has_resources());
        owned.clear();
        assert!(!owned.has_resources());
        assert_eq!(
            (owned.id, owned.gc, owned.width, owned.height),
            (73, 19, 1, 1)
        );
        owned.clear();
        drop(owned);
        unsafe {
            assert_eq!(
                cairo_sys::cairo_get_reference_count(context.to_raw_none()),
                before.0
            );
            assert_eq!(
                cairo_sys::cairo_surface_get_reference_count(image.to_raw_none()),
                before.1
            );
        }
    }

    #[test]
    fn owned_native_surface_releases_exactly_one_reference_and_keeps_error_objects() {
        unsafe {
            let raw = cairo_sys::cairo_image_surface_create(cairo_sys::FORMAT_A_RGB32, 1, 1);
            let observer = cairo::Surface::from_raw_none(raw);
            let owned = CairoSurface::from_raw_owned(raw);
            assert_eq!(cairo_sys::cairo_surface_get_reference_count(raw), 2);
            drop(owned);
            assert_eq!(
                cairo_sys::cairo_surface_get_reference_count(observer.to_raw_none()),
                1
            );
            let raw = cairo_sys::cairo_image_surface_create(cairo_sys::FORMAT_A_RGB32, -1, -1);
            let status = cairo_sys::cairo_surface_status(raw);
            assert_ne!(status, cairo_sys::STATUS_SUCCESS);
            let owned = CairoSurface::from_raw_owned(raw);
            assert_eq!(cairo_sys::cairo_surface_status(owned.as_raw()), status);
            assert!(owned.as_surface().status().is_err());
        }
    }

    fn color(red: f64, green: f64, blue: f64, alpha: f64) -> color_t {
        color_t {
            red,
            green,
            blue,
            alpha,
            colorpixel: 0,
        }
    }

    fn record(image: &ImageSurface, context: &Context) -> surface_t {
        surface_t::from_cairo(
            1,
            image.width(),
            image.height(),
            CairoSurface::from_surface(image.as_ref().clone()),
            context.clone(),
        )
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
        let mut raw = surface_t::from_cairo(
            1,
            -1,
            -1,
            CairoSurface::from_surface(surface.clone()),
            context.clone(),
        );
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
        raw.clear();
        // SAFETY: the zero id guard permits uninitialized null Cairo fields and
        // skips drawing before attempting any native wrapper construction.
        unsafe { draw_util_clear_surface(&mut raw, color(0.0, 0.0, 0.0, 0.0)) };
    }
}
