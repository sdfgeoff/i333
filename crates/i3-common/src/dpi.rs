/*
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE).
 */
//! Native X resource discovery with an independently readable scalar DPI cache.
#![deny(unsafe_op_in_unsafe_fn)]

use crate::logging::{emit, Level};
use std::ffi::CStr;
use std::ops::Deref;
use std::ptr::{self, NonNull};
use std::sync::atomic::{AtomicIsize, Ordering};
use xcb_sys::{
    xcb_connection_t, xcb_xrm_database_free, xcb_xrm_database_from_default, xcb_xrm_database_t,
    xcb_xrm_resource_get_string,
};

// Linux c_long and isize have the same width on the supported ILP32/LP64 ABIs.
static DPI: AtomicIsize = AtomicIsize::new(0);

unsafe extern "C" {
    fn lround(value: f64) -> libc::c_long;
}

struct Database(NonNull<xcb_xrm_database_t>);
impl Database {
    /// # Safety
    /// connection must be a live XCB connection for native X resource discovery.
    unsafe fn from_display(connection: *mut xcb_connection_t) -> Option<Self> {
        // SAFETY: the caller supplies the native connection for this synchronous call.
        NonNull::new(unsafe { xcb_xrm_database_from_default(connection) }).map(Self)
    }

    fn dpi_resource(&self) -> Option<Resource> {
        let mut resource = ptr::null_mut();
        // SAFETY: self owns the live database; output storage is writable and
        // Xrm transfers a malloc-owned, NUL-terminated string when available.
        unsafe {
            xcb_xrm_resource_get_string(
                self.0.as_ptr(),
                c"Xft.dpi".as_ptr(),
                ptr::null(),
                &mut resource,
            );
        }
        NonNull::new(resource).map(Resource)
    }
}
impl Drop for Database {
    fn drop(&mut self) {
        // SAFETY: this wrapper uniquely owns the database returned by Xrm.
        unsafe { xcb_xrm_database_free(self.0.as_ptr()) };
    }
}

struct Resource(NonNull<libc::c_char>);
impl Deref for Resource {
    type Target = CStr;
    fn deref(&self) -> &CStr {
        // SAFETY: Xrm supplies a terminated string owned until this wrapper drops.
        unsafe { CStr::from_ptr(self.0.as_ptr()) }
    }
}
impl Drop for Resource {
    fn drop(&mut self) {
        // SAFETY: Xrm resource strings are transferred malloc allocations.
        unsafe { libc::free(self.0.as_ptr().cast()) };
    }
}

fn parse_resource(resource: &CStr) -> Option<libc::c_long> {
    let mut end = ptr::null_mut();
    // SAFETY: resource is a readable C string and end is writable pointer storage.
    let value = unsafe { libc::strtod(resource.as_ptr(), &mut end) };
    // strtod's end pointer always identifies a byte of the input (including its
    // terminator); checking it preserves native locale and trailing-text behavior.
    // NaN intentionally remains accepted, exactly as in the original predicate.
    if value == f64::INFINITY
        || value < 0.0
        || end == resource.as_ptr().cast_mut()
        || unsafe { *end } != 0
    {
        return None;
    }
    // SAFETY: retain native lround's handling of ties, range errors and NaN,
    // including the platform result when the floating-point invalid flag is set.
    Some(unsafe { lround(value) })
}

fn store(value: libc::c_long) {
    DPI.store(value as isize, Ordering::Relaxed);
}

/// Initialize DPI from Xft.dpi, falling back to physical screen geometry only
/// when the current cached value is zero. Missing resources retain a prior value.
///
/// # Safety
/// The display provider must return live native connection/screen records on the
/// display event thread. The screen must be readable whenever fallback is needed.
/// Display discovery and its callbacks must not race destruction of those records.
pub unsafe fn init_dpi() {
    let crate::display::DisplayContext {
        connection,
        root_screen,
        ..
    } = crate::display::context();
    if !connection.is_null() {
        // SAFETY: caller guarantees the display connection remains live.
        match unsafe { Database::from_display(connection) } {
            None => emit(
                Level::Error,
                b"[libi3] ERROR: Failed to open the resource database.\n",
            ),
            Some(database) => match database.dpi_resource() {
                None => emit(
                    Level::Debug,
                    b"../libi3/dpi.c:init_dpi:42 - Resource Xft.dpi not specified, skipping.\n",
                ),
                Some(resource) => match parse_resource(&resource) {
                    None => {
                        // Keep the original byte-preserving diagnostic for locale
                        // strings, without assuming that an X resource is UTF-8.
                        let mut message = b"[libi3] ERROR: Xft.dpi = ".to_vec();
                        message.extend_from_slice(resource.to_bytes());
                        message
                            .extend_from_slice(b" is an invalid number and couldn't be parsed.\n");
                        emit(Level::Error, &message);
                        store(0);
                    }
                    Some(value) => {
                        store(value);
                        emit(
                            Level::Debug,
                            format!("../libi3/dpi.c:init_dpi:55 - Found Xft.dpi = {value}.\n")
                                .as_bytes(),
                        );
                    }
                },
            },
        }
    }
    if get_dpi_value() == 0 {
        emit(
            Level::Debug,
            b"../libi3/dpi.c:init_dpi:65 - Using fallback for calculating DPI.\n",
        );
        // SAFETY: caller supplies a live readable screen when fallback is needed.
        let screen = unsafe { &*root_screen };
        // Preserve the previous float-to-integer cast, including a zero physical
        // height; no substitute monitor dimensions or fallback policy is introduced.
        let value = (f64::from(screen.height_in_pixels) * 25.4
            / f64::from(screen.height_in_millimeters)) as libc::c_long;
        store(value);
        emit(
            Level::Debug,
            format!("../libi3/dpi.c:init_dpi:67 - Using dpi = {value}\n").as_bytes(),
        );
    }
}

pub fn get_dpi_value() -> libc::c_long {
    DPI.load(Ordering::Relaxed) as libc::c_long
}

fn scaled_pixels(logical: i32, dpi: libc::c_long) -> i32 {
    let scale = dpi as f64 / 96.0;
    if scale < 1.25 {
        logical
    } else {
        (scale * f64::from(logical)).ceil() as i32
    }
}

/// Scale logical pixels when a display is attached. No native pointer is
/// dereferenced here; detached callers retain the original unscaled dimensions.
pub fn logical_px(logical: i32) -> i32 {
    if crate::display::context().root_screen.is_null() {
        logical
    } else {
        scaled_pixels(logical, get_dpi_value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn native_parse(input: &CStr) -> Option<libc::c_long> {
        // SAFETY: independent copy of the original predicate and native rounding;
        // input and output pointer storage remain live throughout conversion.
        unsafe {
            let mut end = ptr::null_mut();
            let value = libc::strtod(input.as_ptr(), &mut end);
            if value == f64::INFINITY
                || value < 0.0
                || *end != 0
                || end == input.as_ptr().cast_mut()
            {
                None
            } else {
                Some(lround(value))
            }
        }
    }

    #[test]
    fn dpi_resource_parsing_retains_native_rounding_locale_and_exception_rules() {
        for input in [
            c"",
            c" ",
            c"96",
            c"96.49",
            c"96.5",
            c"+144",
            c"-0",
            c"-0.5",
            c"0x1.8p6",
            c" 120",
            c"120 ",
            c"120dpi",
            c"inf",
            c"+infinity",
            c"-inf",
            c"nan",
            c"nan(payload)",
            c"1e999",
            c"1e-999",
            c"9223372036854775808",
            c"96,5",
        ] {
            assert_eq!(parse_resource(input), native_parse(input), "{input:?}");
        }
        assert_eq!(parse_resource(c"96.5"), Some(97));
        assert_eq!(parse_resource(c"120 "), None);
        assert_eq!(parse_resource(c"+inf"), None);
    }

    #[test]
    fn missing_display_retains_an_initialized_dpi_without_reading_a_screen() {
        let previous = get_dpi_value();
        store(144);
        // The provider registry gives threads without a registered display a
        // detached context. A missing resource must retain a nonzero cache,
        // avoiding fallback through that context's null screen pointer.
        assert!(crate::display::context().connection.is_null());
        // SAFETY: no connection is queried and no screen is needed because the
        // already initialized scalar is retained on this missing-resource path.
        unsafe { init_dpi() };
        let retained = get_dpi_value();
        store(previous);
        assert_eq!(retained, 144);
    }

    #[test]
    fn pixel_scaling_retains_threshold_ceiling_and_integer_casts() {
        unsafe extern "C" {
            fn ceil(value: f64) -> f64;
        }
        for dpi in [
            0,
            96,
            119,
            120,
            121,
            144,
            192,
            libc::c_long::MIN,
            libc::c_long::MAX,
        ] {
            for logical in [i32::MIN, -3, -1, 0, 1, 3, i32::MAX] {
                let expected = if (dpi as f64 / 96.0) < 1.25 {
                    logical
                }
                // SAFETY: native ceil accepts every floating-point input.
                else {
                    (unsafe { ceil(dpi as f64 / 96.0 * f64::from(logical)) }) as i32
                };
                assert_eq!(scaled_pixels(logical, dpi), expected);
            }
        }
        assert_eq!(scaled_pixels(3, 119), 3);
        assert_eq!(scaled_pixels(3, 120), 4);
        assert_eq!(scaled_pixels(-3, 120), -3);
    }
}
