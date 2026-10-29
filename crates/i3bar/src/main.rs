#![feature(c_variadic)]
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
// Link the shared native-ABI helpers once through the workspace library.
use i3_common as _;
mod application;
mod child;
mod config;
mod ipc;
mod mode;
mod outputs;
mod parse_json_header;
mod workspaces;
mod xcb;

fn main() {
    i3_common::display::set_provider(common_display_context);
    i3_common::logging::set_sink(common_log_sink);
    application::main();
}

fn common_log_sink(level: i3_common::logging::Level, message: &[u8]) {
    use i3_common::logging::Level;
    let length = message.len().min(i32::MAX as usize) as i32;
    // A constant format prevents diagnostic bytes from becoming conversions.
    unsafe {
        match level {
            Level::Error => application::errorlog(
                c"%.*s".as_ptr().cast_mut(),
                length,
                message.as_ptr().cast::<libc::c_char>(),
            ),
            Level::Verbose => application::verboselog(
                c"%.*s".as_ptr().cast_mut(),
                length,
                message.as_ptr().cast::<libc::c_char>(),
            ),
            Level::Debug => application::debuglog(
                c"%.*s".as_ptr().cast_mut(),
                length,
                message.as_ptr().cast::<libc::c_char>(),
            ),
        }
    }
}

fn common_display_context() -> i3_common::display::DisplayContext {
    // These pointers remain owned by the program and are read on its event thread.
    unsafe {
        i3_common::display::DisplayContext {
            connection: xcb::conn,
            root_screen: xcb::root_screen,
            visual_type: xcb::visual_type,
        }
    }
}
