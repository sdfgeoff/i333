#![cfg_attr(test, feature(c_variadic))]
#![allow(
    dead_code,
    unused_imports,
    unused_mut,
    unused_variables,
    unused_assignments,
    unused_parens,
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
mod domains;
mod drag;
mod ewmh;
mod fake_outputs;
mod flags;
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
    // The registered sink runs only on the event thread owning the log state.
    unsafe { log::write_log_bytes(level, message) };
}

fn common_display_context() -> i3_common::display::DisplayContext {
    // The program retains screen/visual owners through graphics teardown.
    unsafe { application::display_context() }
}
