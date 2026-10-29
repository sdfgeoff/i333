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
#[macro_use]
extern crate c2rust_bitfields;

// Link the shared native-ABI helpers once through the workspace library.
use i3_common as _;
mod application;
mod assignments;
mod bindings;
mod click;
mod commands;
mod commands_parser;
mod con;
mod config;
mod config_directives;
mod config_parser;
mod display_version;
mod drag;
mod ewmh;
mod fake_outputs;
mod floating;
mod gaps;
mod handlers;
mod ipc;
mod key_press;
mod load_layout;
mod log;
mod manage;
mod r#match;
mod r#move;
mod output;
mod parser_util;
mod randr;
mod regex;
mod render;
mod resize;
mod restore_layout;
mod scratchpad;
mod sd_daemon;
mod sighandler;
mod startup;
mod sync;
mod tiling_drag;
mod tree;
mod types;
mod util;
mod version;
mod window;
mod workspace;
mod x;
mod xcb;
mod xcursor;
mod xinerama;

fn main() {
    i3_common::display::set_provider(common_display_context);
    i3_common::logging::set_sink(common_log_sink);
    application::main();
}

// Private adapters installed for the event-thread application. The legacy
// logger/tree globals are not made thread-safe by the shared callback interface.
fn common_log_sink(level: i3_common::logging::Level, message: &[u8]) {
    use i3_common::logging::Level;
    let length = message.len().min(i32::MAX as usize) as i32;
    // A constant format prevents diagnostic bytes from becoming conversions.
    unsafe {
        match level {
            Level::Error => log::errorlog(
                c"%.*s".as_ptr().cast_mut(),
                length,
                message.as_ptr().cast::<libc::c_char>(),
            ),
            Level::Verbose => log::verboselog(
                c"%.*s".as_ptr().cast_mut(),
                length,
                message.as_ptr().cast::<libc::c_char>(),
            ),
            Level::Debug => log::debuglog(
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
            connection: application::conn,
            root_screen: application::root_screen,
            visual_type: application::visual_type,
        }
    }
}
