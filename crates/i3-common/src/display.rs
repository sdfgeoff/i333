//! Borrowed display state supplied explicitly by each graphical program.
//!
//! The provider retains ownership of the connection and screen resources. Unsafe
//! rendering callers must keep those resources alive for the whole operation.
use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::RwLock;
use xcb_sys::{xcb_connection_t, xcb_screen_t, xcb_visualtype_t};

thread_local! {
    static CONNECTION: RefCell<Option<Rc<xcb::Connection>>> = const { RefCell::new(None) };
}

/// Keeps the event thread's display registered until native/graphics teardown.
///
/// The registry and caller share one maintained connection owner through `Rc`.
/// Drop this guard after releasing native resources, before the caller's final
/// connection owner. Clones obtained during callbacks remain valid independently.
#[must_use = "the display connection is registered only while this guard lives"]
pub struct DisplayConnection {
    // The registration belongs to its originating thread and cannot be sent.
    _thread: PhantomData<Rc<()>>,
}

/// Register the sole maintained connection owner for the current event thread.
///
/// Each graphical program registers once after connecting, before common display
/// helpers run. Nested registrations indicate a lifecycle bug and are rejected.
pub fn install_connection(connection: Rc<xcb::Connection>) -> DisplayConnection {
    CONNECTION.with(|registered| {
        let mut registered = registered.borrow_mut();
        assert!(
            registered.is_none(),
            "a display connection is already registered"
        );
        *registered = Some(connection);
    });
    DisplayConnection {
        _thread: PhantomData,
    }
}

/// Clone the current thread's connection without retaining a registry borrow.
///
/// Callbacks can reenter display helpers or unregister the display while a
/// request uses this clone; the same single native connection stays alive.
pub fn connection() -> Option<Rc<xcb::Connection>> {
    CONNECTION.with(|registered| registered.borrow().clone())
}

impl Drop for DisplayConnection {
    fn drop(&mut self) {
        // Drop the Rc after releasing the registry borrow, even if this is its
        // final owner. Reentrant teardown must never see a borrowed RefCell.
        let owner = CONNECTION.with(|registered| registered.borrow_mut().take());
        drop(owner);
    }
}

#[derive(Clone, Copy, Debug)]
pub struct DisplayContext {
    pub connection: *mut xcb_connection_t,
    pub root_screen: *mut xcb_screen_t,
    pub visual_type: *mut xcb_visualtype_t,
}

impl Default for DisplayContext {
    fn default() -> Self {
        Self {
            connection: std::ptr::null_mut(),
            root_screen: std::ptr::null_mut(),
            visual_type: std::ptr::null_mut(),
        }
    }
}

pub type Provider = fn() -> DisplayContext;
#[derive(Clone, Copy)]
struct RegisteredProvider {
    callback: Provider,
    thread: std::thread::ThreadId,
}
static PROVIDER: RwLock<Option<RegisteredProvider>> = RwLock::new(None);

/// Install on the display event thread before calling display-dependent helpers.
/// Other threads receive a detached context rather than reading program globals.
pub fn set_provider(provider: Provider) {
    *PROVIDER
        .write()
        .unwrap_or_else(std::sync::PoisonError::into_inner) = Some(RegisteredProvider {
        callback: provider,
        thread: std::thread::current().id(),
    });
}

/// Snapshot the current state without holding the registry lock during callbacks.
pub fn context() -> DisplayContext {
    let provider = *PROVIDER
        .read()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    match provider {
        Some(provider) if provider.thread == std::thread::current().id() => (provider.callback)(),
        _ => DisplayContext::default(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn detached() -> DisplayContext {
        DisplayContext {
            connection: std::ptr::NonNull::<xcb_connection_t>::dangling().as_ptr(),
            ..DisplayContext::default()
        }
    }

    #[test]
    fn provider_registration_returns_typed_borrowed_state() {
        assert!(DisplayContext::default().connection.is_null());
        assert!(DisplayContext::default().root_screen.is_null());
        assert!(DisplayContext::default().visual_type.is_null());
        set_provider(detached);
        assert_eq!(context().connection, detached().connection);
        assert!(context().root_screen.is_null());
        assert!(std::thread::spawn(|| context().connection.is_null())
            .join()
            .unwrap());
        set_provider(DisplayContext::default);
        assert!(context().connection.is_null());
    }
}
