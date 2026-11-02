//! Owned native X resource databases with byte-preserving values.
use crate::NativeString;
use std::{
    ffi::{CStr, CString},
    ptr::NonNull,
    rc::Rc,
};

/// Opaque libxcb-xrm database for transitional native integrations.
#[repr(C)]
pub struct NativeDatabase {
    _private: [u8; 0],
}

pub struct Database {
    pointer: NonNull<NativeDatabase>,
    _connection: Option<Rc<xcb::Connection>>,
}
impl Database {
    pub fn from_connection(connection: Rc<xcb::Connection>) -> Option<Self> {
        // SAFETY: discovery synchronously borrows a retained live connection.
        let pointer =
            NonNull::new(unsafe { ffi::xcb_xrm_database_from_default(connection.get_raw_conn()) })?;
        Some(Self {
            pointer,
            _connection: Some(connection),
        })
    }
    pub fn from_string(resources: &CStr) -> Option<Self> {
        // SAFETY: the native parser copies these terminated bytes into its database.
        let pointer =
            NonNull::new(unsafe { ffi::xcb_xrm_database_from_string(resources.as_ptr()) })?;
        Some(Self {
            pointer,
            _connection: None,
        })
    }
    pub fn resource(&self, name: &CStr, class: Option<&CStr>) -> Option<CString> {
        let mut result = std::ptr::null_mut();
        // SAFETY: the owned database and terminated name/class outlive this
        // synchronous query. The utility transfers a malloc-owned native string.
        unsafe {
            ffi::xcb_xrm_resource_get_string(
                self.pointer.as_ptr(),
                name.as_ptr(),
                class.map_or(std::ptr::null(), CStr::as_ptr),
                &mut result,
            );
        }
        let result = unsafe { NativeString::from_raw(result) }?;
        Some(result.as_c_str().to_owned())
    }
    pub fn as_raw(&self) -> *mut NativeDatabase {
        self.pointer.as_ptr()
    }
}
impl Drop for Database {
    fn drop(&mut self) {
        // SAFETY: this is the sole native database owner.
        unsafe { ffi::xcb_xrm_database_free(self.pointer.as_ptr()) }
    }
}

mod ffi {
    use super::NativeDatabase;
    use std::ffi::{c_char, c_int};
    use xcb::ffi::xcb_connection_t;
    unsafe extern "C" {
        pub fn xcb_xrm_database_from_default(
            connection: *mut xcb_connection_t,
        ) -> *mut NativeDatabase;
        pub fn xcb_xrm_database_from_string(resources: *const c_char) -> *mut NativeDatabase;
        pub fn xcb_xrm_database_free(database: *mut NativeDatabase);
        pub fn xcb_xrm_resource_get_string(
            database: *mut NativeDatabase,
            name: *const c_char,
            class: *const c_char,
            result: *mut *mut c_char,
        ) -> c_int;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn database_and_values_own_their_native_input_bytes() {
        let resources = CString::new(b"Xft.dpi: 144\nI333.label: \xfflabel\n".to_vec()).unwrap();
        let database = Database::from_string(&resources).unwrap();
        drop(resources);
        let dpi = database.resource(c"Xft.dpi", None).unwrap();
        let label = database.resource(c"I333.label", None).unwrap();
        assert!(database.resource(c"I333.missing", None).is_none());
        drop(database);
        assert_eq!(dpi, c"144");
        assert_eq!(label.to_bytes(), b"\xfflabel");
    }
}
