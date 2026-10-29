/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 */
// Mechanical file-by-file Rust translation of libi3/get_config_path.c.
// Generated with C2Rust 0.22.1; C ABI and original ownership retained.
extern "C" {
    fn sstrdup(str: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn sasprintf(
        strp: *mut *mut ::core::ffi::c_char,
        fmt: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn resolve_tilde(path: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn path_exists(path: *const ::core::ffi::c_char) -> bool;
    fn strtok(
        __s: *mut ::core::ffi::c_char,
        __delim: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn getenv(__name: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[no_mangle]
pub unsafe extern "C" fn get_config_path(
    mut override_configpath: *const ::core::ffi::c_char,
    mut use_system_paths: bool,
) -> *mut ::core::ffi::c_char {
    let mut xdg_config_home: *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut xdg_config_dirs: *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut config_path: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    static mut saved_configpath: *const ::core::ffi::c_char =
        ::core::ptr::null::<::core::ffi::c_char>();
    if !override_configpath.is_null() {
        saved_configpath = override_configpath;
        return sstrdup(saved_configpath);
    }
    if !saved_configpath.is_null() {
        return sstrdup(saved_configpath);
    }
    xdg_config_home = getenv(b"XDG_CONFIG_HOME\0" as *const u8 as *const ::core::ffi::c_char);
    if xdg_config_home.is_null() {
        xdg_config_home =
            b"~/.config\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
    }
    xdg_config_home = resolve_tilde(xdg_config_home);
    sasprintf(
        &raw mut config_path,
        b"%s/i3/config\0" as *const u8 as *const ::core::ffi::c_char,
        xdg_config_home,
    );
    free(xdg_config_home as *mut ::core::ffi::c_void);
    if path_exists(config_path) {
        return config_path;
    }
    free(config_path as *mut ::core::ffi::c_void);
    config_path = resolve_tilde(b"~/.i3/config\0" as *const u8 as *const ::core::ffi::c_char);
    if path_exists(config_path) {
        return config_path;
    }
    free(config_path as *mut ::core::ffi::c_void);
    if !use_system_paths {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    xdg_config_dirs = getenv(b"XDG_CONFIG_DIRS\0" as *const u8 as *const ::core::ffi::c_char);
    if xdg_config_dirs.is_null() {
        xdg_config_dirs = b"/usr/local/etc/xdg\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char;
    }
    let mut buf: *mut ::core::ffi::c_char = sstrdup(xdg_config_dirs);
    let mut tok: *mut ::core::ffi::c_char =
        strtok(buf, b":\0" as *const u8 as *const ::core::ffi::c_char);
    while !tok.is_null() {
        tok = resolve_tilde(tok);
        sasprintf(
            &raw mut config_path,
            b"%s/i3/config\0" as *const u8 as *const ::core::ffi::c_char,
            tok,
        );
        free(tok as *mut ::core::ffi::c_void);
        if path_exists(config_path) {
            free(buf as *mut ::core::ffi::c_void);
            return config_path;
        }
        free(config_path as *mut ::core::ffi::c_void);
        tok = strtok(
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            b":\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    free(buf as *mut ::core::ffi::c_void);
    config_path = b"/usr/local/etc/i3/config\0" as *const u8 as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char;
    if path_exists(config_path) {
        return sstrdup(config_path);
    }
    return ::core::ptr::null_mut::<::core::ffi::c_char>();
}
