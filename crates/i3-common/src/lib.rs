#![feature(c_variadic, extern_types)]
#![allow(
    dead_code,
    unused_imports,
    unused_mut,
    unused_variables,
    unused_assignments,
    unused_parens,
    unused_unsafe,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    static_mut_refs,
    clashing_extern_declarations,
    stable_features,
    ambiguous_wide_pointer_comparisons
)]
#[macro_use]
extern crate c2rust_bitfields;

pub mod boolstr;
pub mod create_socket;
pub mod dpi;
pub mod draw_util;
pub mod fake_configure_notify;
pub mod font;
pub mod format_placeholders;
pub mod g_utf8_make_valid;
pub mod get_colorpixel;
pub mod get_config_path;
pub mod get_exe_path;
pub mod get_mod_mask;
pub mod get_process_filename;
pub mod get_visualtype;
pub mod ipc_connect;
pub mod ipc_recv_message;
pub mod ipc_send_message;
pub mod is_background_set;
pub mod is_debug_build;
pub mod mkdirp;
pub mod nonblock;
pub mod path_exists;
pub mod resolve_tilde;
pub mod root_atom_contents;
pub mod safewrappers;
pub mod screenshot_wallpaper;
pub mod string;
pub mod ucs2_conversion;
mod variadic_compat;

#[cfg(test)]
mod tests;
