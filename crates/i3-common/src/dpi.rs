/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 */
// Mechanical file-by-file Rust translation of libi3/dpi.c.
// Originally translated with C2Rust 0.22.1; subsequently maintained in Rust.
extern "C" {

    fn ceil(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn lround(__x: ::core::ffi::c_double) -> ::core::ffi::c_long;

}

pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
static mut dpi: ::core::ffi::c_long = 0;
unsafe fn init_dpi_fallback() -> ::core::ffi::c_long {
    let crate::display::DisplayContext { root_screen, .. } = crate::display::context();

    ((*root_screen).height_in_pixels as ::core::ffi::c_double * 25.4f64
        / (*root_screen).height_in_millimeters as ::core::ffi::c_double) as ::core::ffi::c_long
}

pub unsafe fn init_dpi() {
    let crate::display::DisplayContext {
        connection: conn, ..
    } = crate::display::context();

    let mut endptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut in_dpi: ::core::ffi::c_double = 0.;
    let mut database: *mut xcb_xrm_database_t = ::core::ptr::null_mut::<xcb_xrm_database_t>();
    let mut resource: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if !conn.is_null() {
        database = xcb_xrm_database_from_default(conn);
        if database.is_null() {
            errorlog(
                c"[libi3] ERROR: Failed to open the resource database.\n"
                    .as_ptr()
                    .cast_mut(),
            );
        } else {
            xcb_xrm_resource_get_string(
                database,
                c"Xft.dpi".as_ptr(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                &raw mut resource,
            );
            if resource.is_null() {
                debuglog(
                    c"%s:%s:%d - Resource Xft.dpi not specified, skipping.\n"
                        .as_ptr()
                        .cast_mut(),
                    c"../libi3/dpi.c".as_ptr(),
                    c"init_dpi".as_ptr(),
                    42 as ::core::ffi::c_int,
                );
            } else {
                endptr = ::core::ptr::null_mut::<::core::ffi::c_char>();
                in_dpi = strtod(resource, &raw mut endptr);
                if in_dpi == ::core::f64::INFINITY
                    || in_dpi < 0 as ::core::ffi::c_int as ::core::ffi::c_double
                    || *endptr as ::core::ffi::c_int != '\0' as i32
                    || endptr == resource
                {
                    errorlog(
                        c"[libi3] ERROR: Xft.dpi = %s is an invalid number and couldn't be parsed.\n".as_ptr().cast_mut(),
                        resource,
                    );
                    dpi = 0 as ::core::ffi::c_long;
                } else {
                    dpi = lround(in_dpi);
                    debuglog(
                        c"%s:%s:%d - Found Xft.dpi = %ld.\n".as_ptr().cast_mut(),
                        c"../libi3/dpi.c".as_ptr(),
                        c"init_dpi".as_ptr(),
                        55 as ::core::ffi::c_int,
                        dpi,
                    );
                }
            }
        }
    }
    free(resource as *mut ::core::ffi::c_void);
    if !database.is_null() {
        xcb_xrm_database_free(database);
    }
    if dpi == 0 as ::core::ffi::c_long {
        debuglog(
            c"%s:%s:%d - Using fallback for calculating DPI.\n"
                .as_ptr()
                .cast_mut(),
            c"../libi3/dpi.c".as_ptr(),
            c"init_dpi".as_ptr(),
            65 as ::core::ffi::c_int,
        );
        dpi = init_dpi_fallback();
        debuglog(
            c"%s:%s:%d - Using dpi = %ld\n".as_ptr().cast_mut(),
            c"../libi3/dpi.c".as_ptr(),
            c"init_dpi".as_ptr(),
            67 as ::core::ffi::c_int,
            dpi,
        );
    }
}

pub unsafe fn get_dpi_value() -> ::core::ffi::c_long {
    dpi
}

pub unsafe fn logical_px(logical: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let crate::display::DisplayContext { root_screen, .. } = crate::display::context();

    if root_screen.is_null() {
        return logical;
    }
    if dpi as ::core::ffi::c_double / 96.0f64 < 1.25f64 {
        return logical;
    }
    ceil(dpi as ::core::ffi::c_double / 96.0f64 * logical as ::core::ffi::c_double)
        as ::core::ffi::c_int
}

use xcb_sys::{
    xcb_xrm_database_free, xcb_xrm_database_from_default, xcb_xrm_database_t,
    xcb_xrm_resource_get_string,
};

pub use libc::{free, strtod};

use crate::logging::{debuglog, errorlog};
