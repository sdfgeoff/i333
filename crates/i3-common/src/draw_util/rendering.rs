//! Connection-scoped rendering state. No process-global GC cache is used here.
#![deny(unsafe_op_in_unsafe_fn)]

use super::{GcLease, OwnedGraphicsContext};
use crate::display::{CairoVisual, DisplayScreen};
use std::cell::RefCell;
use std::rc::Rc;
use xcb::{x, Xid, XidNew};

struct CacheEntry {
    depth: u8,
    owner: OwnedGraphicsContext,
}

/// Retains the setup/connection until all surfaces and GC leases are released.
/// Each connection has its own two-entry depth cache; overflow GCs are owned by
/// the requesting surface/lease. Cache borrows never cross native requests.
pub struct RenderingContext {
    // Drop cached server resources before the retained connection/setup owner.
    cache: RefCell<[Option<CacheEntry>; 2]>,
    screen: DisplayScreen,
    default_visual: u32,
}

impl RenderingContext {
    pub fn new(screen: DisplayScreen) -> Rc<Self> {
        let visual = screen.info().root_visual;
        Self::with_default_visual(screen, visual)
    }

    pub fn with_default_visual(screen: DisplayScreen, default_visual: u32) -> Rc<Self> {
        Rc::new(Self {
            cache: RefCell::new([None, None]),
            screen,
            default_visual,
        })
    }
    pub fn default_visual(&self) -> u32 {
        self.default_visual
    }
    pub fn screen(&self) -> &DisplayScreen {
        &self.screen
    }

    pub fn visual(&self, id: u32) -> Option<Rc<CairoVisual>> {
        self.screen.visual(id).map(Rc::new)
    }

    pub(super) fn visual_depth(&self, id: u32) -> u8 {
        self.screen
            .screen()
            .allowed_depths()
            .find_map(|depth| {
                depth
                    .visuals()
                    .iter()
                    .any(|visual| visual.visual_id() == id)
                    .then(|| depth.depth())
            })
            .unwrap_or(0)
    }

    /// Lease a cached or independently owned GC while retaining its connection.
    pub fn graphics_context(self: &Rc<Self>, drawable: u32, depth: u8) -> GraphicsContext {
        let (id, lease) = self.acquire_gc(drawable, depth);
        GraphicsContext {
            lease,
            id,
            _context: self.clone(),
        }
    }

    pub(super) fn acquire_gc(&self, drawable: u32, depth: u8) -> (u32, GcLease) {
        // Retain the native depth-zero cache sentinel behavior.
        if depth == 0 {
            return (0, GcLease::Borrowed);
        }
        let empty = {
            let cache = self.cache.borrow();
            if let Some(entry) = cache.iter().flatten().find(|entry| entry.depth == depth) {
                return (entry.owner.id(), GcLease::Borrowed);
            }
            cache.iter().position(Option::is_none)
        };
        let connection = self.screen.connection();
        let gc: x::Gcontext = connection.generate_id();
        let cookie = connection.send_request_checked(&x::CreateGc {
            cid: gc,
            drawable: x::Drawable::Unknown(drawable),
            value_list: &[],
        });
        if let Err(error) = connection.check_request(cookie) {
            // SAFETY: the maintained error owns its complete native header.
            let code = unsafe { (*error.as_raw()).error_code };
            ::log::error!(
                "[libi3] ERROR: Could not create graphical context. Error code: {}. Please report this bug.\n",
                code,
            );
            return (gc.resource_id(), GcLease::Borrowed);
        }
        // SAFETY: this unique successful GC is released before self.screen's
        // retained connection, or by an outward lease retaining this context.
        let owner = unsafe {
            OwnedGraphicsContext::from_raw_owned(connection.get_raw_conn(), gc.resource_id())
        };
        if let Some(index) = empty {
            let mut cache = self.cache.borrow_mut();
            if cache[index].is_none() {
                cache[index] = Some(CacheEntry { depth, owner });
                return (gc.resource_id(), GcLease::Borrowed);
            }
        }
        (gc.resource_id(), GcLease::Owned(owner))
    }
}

/// A GC lease retains its connection/cache without assuming ownership of cached
/// IDs. An uncached ID is freed before the last context reference is released.
pub struct GraphicsContext {
    lease: GcLease,
    id: u32,
    _context: Rc<RenderingContext>,
}
impl GraphicsContext {
    pub fn id(&self) -> u32 {
        self.id
    }
}

/// One leaf server window, retaining its connection until explicit close/drop.
/// Parent windows holding foreign application/tray children remain borrowed and
/// follow their existing reparent/save-set teardown policy instead.
pub struct OwnedWindow {
    window: Option<x::Window>,
    connection: Rc<xcb::Connection>,
}
impl OwnedWindow {
    /// # Safety
    /// The caller transfers unique destruction responsibility for this leaf XID.
    /// It must never parent foreign windows requiring save-set reparenting.
    pub unsafe fn from_created(connection: Rc<xcb::Connection>, window: x::Window) -> Self {
        Self {
            window: Some(window),
            connection,
        }
    }
    pub fn window(&self) -> Option<x::Window> {
        self.window
    }
    pub fn close(&mut self) {
        if let Some(window) = self.window.take() {
            self.connection.send_request(&x::DestroyWindow { window });
        }
    }
}
impl Drop for OwnedWindow {
    fn drop(&mut self) {
        self.close();
    }
}

/// One server pixmap. Closing is idempotent and precedes connection release.
pub struct OwnedPixmap {
    pixmap: Option<x::Pixmap>,
    connection: Rc<xcb::Connection>,
}
impl OwnedPixmap {
    /// # Safety
    /// The caller transfers unique release responsibility for this created XID.
    pub unsafe fn from_created(connection: Rc<xcb::Connection>, pixmap: x::Pixmap) -> Self {
        Self {
            pixmap: Some(pixmap),
            connection,
        }
    }
    pub fn pixmap(&self) -> Option<x::Pixmap> {
        self.pixmap
    }
    pub fn close(&mut self) {
        if let Some(pixmap) = self.pixmap.take() {
            self.connection.send_request(&x::FreePixmap { pixmap });
        }
    }
}
impl Drop for OwnedPixmap {
    fn drop(&mut self) {
        self.close();
    }
}

/// Explicit emergency graphics state. The caller retains its native connection
/// through all derived surfaces, which must be cleared before this cache drops.
/// Its cache is exclusive native state, separate from normal RefCell/Rc caches.
pub struct NativeRenderingContext {
    cache: [Option<CacheEntry>; 2],
    connection: *mut xcb::ffi::xcb_connection_t,
    visual: CairoVisual,
    depth: u8,
}
impl NativeRenderingContext {
    /// # Safety
    /// The connection must remain live until this context and all derived
    /// graphics are destroyed. Access is exclusive on the native signal edge.
    pub unsafe fn new(
        connection: *mut xcb::ffi::xcb_connection_t,
        visual: CairoVisual,
        depth: u8,
    ) -> Self {
        Self {
            cache: [None, None],
            connection,
            visual,
            depth,
        }
    }
    /// Populate the emergency depth cache before installing the signal handler.
    pub fn prepare(&mut self, drawable: u32) {
        // The physical root can differ from the ARGB crash-window depth. Make
        // a scratch drawable of the exact depth before the signal path begins.
        let Some(connection) = (unsafe { crate::xcb_connection::borrowed(self.connection) }) else {
            return;
        };
        let pixmap: x::Pixmap = connection.generate_id();
        connection.send_request(&x::CreatePixmap {
            depth: self.depth,
            pid: pixmap,
            drawable: x::Drawable::Unknown(drawable),
            width: 1,
            height: 1,
        });
        let (_, lease) = self.acquire_gc(pixmap.resource_id());
        drop(lease);
        connection.send_request(&x::FreePixmap { pixmap });
    }
    pub fn visual(&self) -> &CairoVisual {
        &self.visual
    }
    pub(super) fn connection(&self) -> *mut xcb::ffi::xcb_connection_t {
        self.connection
    }
    pub(super) fn prepared_gc(&self) -> u32 {
        self.cache
            .iter()
            .flatten()
            .find(|entry| entry.depth == self.depth)
            .map_or(0, |entry| entry.owner.id())
    }
    pub(super) fn acquire_gc(&mut self, drawable: u32) -> (u32, GcLease) {
        if self.depth == 0 {
            return (0, GcLease::Borrowed);
        }
        if let Some(entry) = self
            .cache
            .iter()
            .flatten()
            .find(|entry| entry.depth == self.depth)
        {
            return (entry.owner.id(), GcLease::Borrowed);
        }
        // SAFETY: native context adoption retains the same healthy connection;
        // the no-drop view neither clones nor accesses the event-thread registry.
        let Some(connection) = (unsafe { crate::xcb_connection::borrowed(self.connection) }) else {
            return (0, GcLease::Borrowed);
        };
        let gc: x::Gcontext = connection.generate_id();
        let cookie = connection.send_request_checked(&x::CreateGc {
            cid: gc,
            drawable: x::Drawable::Unknown(drawable),
            value_list: &[],
        });
        if let Err(error) = connection.check_request(cookie) {
            let code = unsafe { (*error.as_raw()).error_code };
            ::log::error!(
                "[libi3] ERROR: Could not create graphical context. Error code: {}. Please report this bug.\n",
                code,
            );
            return (gc.resource_id(), GcLease::Borrowed);
        }
        let owner =
            unsafe { OwnedGraphicsContext::from_raw_owned(self.connection, gc.resource_id()) };
        if let Some(slot) = self.cache.iter_mut().find(|slot| slot.is_none()) {
            *slot = Some(CacheEntry {
                depth: self.depth,
                owner,
            });
            (gc.resource_id(), GcLease::Borrowed)
        } else {
            (gc.resource_id(), GcLease::Owned(owner))
        }
    }
}
