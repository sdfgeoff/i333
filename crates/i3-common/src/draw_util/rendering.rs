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
}

impl RenderingContext {
    pub fn new(screen: DisplayScreen) -> Rc<Self> {
        Rc::new(Self {
            cache: RefCell::new([None, None]),
            screen,
        })
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
            unsafe {
                crate::logging::errorlog(
                    c"[libi3] ERROR: Could not create graphical context. Error code: %d. Please report this bug.\n".as_ptr().cast_mut(),
                    i32::from(code),
                );
            }
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
