//! Cursor theme loading with a retained maintained display owner.
use std::{ffi::CStr, ptr::NonNull, rc::Rc};
use xcb::{x, XidNew};

/// Opaque libxcb-cursor allocation for transitional native integrations.
#[repr(C)]
pub struct NativeCursorContext {
    _private: [u8; 0],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CursorError {
    MissingScreen(i32),
    Native(i32),
}
impl std::fmt::Display for CursorError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingScreen(screen) => write!(formatter, "X screen {screen} does not exist"),
            Self::Native(code) => write!(formatter, "native cursor initialization failed ({code})"),
        }
    }
}
impl std::error::Error for CursorError {}

pub struct CursorContext {
    pointer: NonNull<NativeCursorContext>,
    // The context retains native screen/setup pointers into this connection.
    _connection: Rc<xcb::Connection>,
}
impl CursorContext {
    pub fn new(connection: Rc<xcb::Connection>, screen: i32) -> Result<Self, CursorError> {
        let index = usize::try_from(screen).map_err(|_| CursorError::MissingScreen(screen))?;
        if connection.get_setup().roots().nth(index).is_none() {
            return Err(CursorError::MissingScreen(screen));
        }
        // SAFETY: the validated native screen is borrowed from this retained
        // connection. Never cast maintained alignment-one wire screen/visual views.
        let native_screen = unsafe { ffi::xcb_aux_get_screen(connection.get_raw_conn(), screen) };
        if native_screen.is_null() {
            return Err(CursorError::MissingScreen(screen));
        }
        let mut pointer = std::ptr::null_mut();
        let status = unsafe {
            ffi::xcb_cursor_context_new(connection.get_raw_conn(), native_screen, &mut pointer)
        };
        if status < 0 {
            return Err(CursorError::Native(status));
        }
        let pointer = NonNull::new(pointer).ok_or(CursorError::Native(status))?;
        Ok(Self {
            pointer,
            _connection: connection,
        })
    }
    /// Load a server cursor. The caller releases its ID with `xcb::x::FreeCursor`.
    pub fn load(&self, name: &CStr) -> x::Cursor {
        // SAFETY: the context/display remain live and the native name is terminated.
        unsafe {
            x::Cursor::new(ffi::xcb_cursor_load_cursor(
                self.pointer.as_ptr(),
                name.as_ptr(),
            ))
        }
    }
    pub fn as_raw(&self) -> *mut NativeCursorContext {
        self.pointer.as_ptr()
    }
}
impl Drop for CursorContext {
    fn drop(&mut self) {
        // SAFETY: the context is uniquely owned and its display still retained.
        unsafe { ffi::xcb_cursor_context_free(self.pointer.as_ptr()) }
    }
}

mod ffi {
    use super::NativeCursorContext;
    use std::ffi::{c_char, c_int, c_void};
    use xcb::ffi::xcb_connection_t;
    unsafe extern "C" {
        // Native screen is private to the foreign utility; no protocol setup
        // schema is copied into this crate to interpret it.
        pub fn xcb_aux_get_screen(connection: *mut xcb_connection_t, screen: c_int) -> *mut c_void;
        pub fn xcb_cursor_context_new(
            connection: *mut xcb_connection_t,
            screen: *mut c_void,
            context: *mut *mut NativeCursorContext,
        ) -> c_int;
        pub fn xcb_cursor_context_free(context: *mut NativeCursorContext);
        pub fn xcb_cursor_load_cursor(
            context: *mut NativeCursorContext,
            name: *const c_char,
        ) -> u32;
    }
}
