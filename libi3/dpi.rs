/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 */
// Mechanical file-by-file Rust translation of libi3/dpi.c.
// Generated with C2Rust 0.22.1; C ABI and original ownership retained.
extern "C" {
    pub type xcb_connection_t;
    pub type xcb_xrm_database_t;
    static mut conn: *mut xcb_connection_t;
    static mut root_screen: *mut xcb_screen_t;
    fn errorlog(fmt: *mut ::core::ffi::c_char, ...);
    fn debuglog(fmt: *mut ::core::ffi::c_char, ...);
    fn ceil(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn lround(__x: ::core::ffi::c_double) -> ::core::ffi::c_long;
    fn xcb_xrm_database_from_default(conn_0: *mut xcb_connection_t) -> *mut xcb_xrm_database_t;
    fn xcb_xrm_database_free(database: *mut xcb_xrm_database_t);
    fn xcb_xrm_resource_get_string(
        database: *mut xcb_xrm_database_t,
        res_name: *const ::core::ffi::c_char,
        res_class: *const ::core::ffi::c_char,
        out: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strtod(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_double;
    fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type xcb_window_t = uint32_t;
pub type xcb_colormap_t = uint32_t;
pub type xcb_visualid_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_screen_t {
    pub root: xcb_window_t,
    pub default_colormap: xcb_colormap_t,
    pub white_pixel: uint32_t,
    pub black_pixel: uint32_t,
    pub current_input_masks: uint32_t,
    pub width_in_pixels: uint16_t,
    pub height_in_pixels: uint16_t,
    pub width_in_millimeters: uint16_t,
    pub height_in_millimeters: uint16_t,
    pub min_installed_maps: uint16_t,
    pub max_installed_maps: uint16_t,
    pub root_visual: xcb_visualid_t,
    pub backing_stores: uint8_t,
    pub save_unders: uint8_t,
    pub root_depth: uint8_t,
    pub allowed_depths_len: uint8_t,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
static mut dpi: ::core::ffi::c_long = 0;
unsafe extern "C" fn init_dpi_fallback() -> ::core::ffi::c_long {
    return ((*root_screen).height_in_pixels as ::core::ffi::c_double * 25.4f64
        / (*root_screen).height_in_millimeters as ::core::ffi::c_double)
        as ::core::ffi::c_long;
}
#[no_mangle]
pub unsafe extern "C" fn init_dpi() {
    let mut endptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut in_dpi: ::core::ffi::c_double = 0.;
    let mut database: *mut xcb_xrm_database_t = ::core::ptr::null_mut::<xcb_xrm_database_t>();
    let mut resource: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if !conn.is_null() {
        database = xcb_xrm_database_from_default(conn);
        if database.is_null() {
            errorlog(
                b"[libi3] ERROR: Failed to open the resource database.\n\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
        } else {
            xcb_xrm_resource_get_string(
                database,
                b"Xft.dpi\0" as *const u8 as *const ::core::ffi::c_char,
                ::core::ptr::null::<::core::ffi::c_char>(),
                &raw mut resource,
            );
            if resource.is_null() {
                debuglog(
                    b"%s:%s:%d - Resource Xft.dpi not specified, skipping.\n\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    b"../libi3/dpi.c\0" as *const u8 as *const ::core::ffi::c_char,
                    b"init_dpi\0" as *const u8 as *const ::core::ffi::c_char,
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
                        b"[libi3] ERROR: Xft.dpi = %s is an invalid number and couldn't be parsed.\n\0"
                            as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        resource,
                    );
                    dpi = 0 as ::core::ffi::c_long;
                } else {
                    dpi = lround(in_dpi);
                    debuglog(
                        b"%s:%s:%d - Found Xft.dpi = %ld.\n\0" as *const u8
                            as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        b"../libi3/dpi.c\0" as *const u8 as *const ::core::ffi::c_char,
                        b"init_dpi\0" as *const u8 as *const ::core::ffi::c_char,
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
            b"%s:%s:%d - Using fallback for calculating DPI.\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"../libi3/dpi.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"init_dpi\0" as *const u8 as *const ::core::ffi::c_char,
            65 as ::core::ffi::c_int,
        );
        dpi = init_dpi_fallback();
        debuglog(
            b"%s:%s:%d - Using dpi = %ld\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b"../libi3/dpi.c\0" as *const u8 as *const ::core::ffi::c_char,
            b"init_dpi\0" as *const u8 as *const ::core::ffi::c_char,
            67 as ::core::ffi::c_int,
            dpi,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn get_dpi_value() -> ::core::ffi::c_long {
    return dpi;
}
#[no_mangle]
pub unsafe extern "C" fn logical_px(logical: ::core::ffi::c_int) -> ::core::ffi::c_int {
    if root_screen.is_null() {
        return logical;
    }
    if dpi as ::core::ffi::c_double / 96.0f64 < 1.25f64 {
        return logical;
    }
    return ceil(dpi as ::core::ffi::c_double / 96.0f64 * logical as ::core::ffi::c_double)
        as ::core::ffi::c_int;
}
