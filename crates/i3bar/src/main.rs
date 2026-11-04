#![feature(c_variadic)]
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
    application::write_log_bytes(level, message);
}

fn common_display_context() -> i3_common::display::DisplayContext {
    // The bar event thread publishes a Copy snapshot backed by its retained
    // connection/visual guard until explicit graphics cleanup.
    unsafe { xcb::common_display_context() }
}
