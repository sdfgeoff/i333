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
use i3_common as _;
mod application;
mod keysym2ucs;

fn main() {
    i3_common::display::set_provider(common_display_context);
    i3_common::logging::set_sink(common_log_sink);
    application::main();
}

fn common_log_sink(level: i3_common::logging::Level, message: &[u8]) {
    application::write_log_bytes(level, message);
}

fn common_display_context() -> i3_common::display::DisplayContext {
    // These pointers remain owned by the program and are read on its event thread.
    unsafe {
        i3_common::display::DisplayContext {
            connection: application::conn.cast(),
            screen: application::root_screen
                .as_ref()
                .map(|screen| screen.info()),
            visual_type: application::visual_type,
            visual_id: application::root_screen
                .as_ref()
                .map_or(0, |screen| screen.info().root_visual),
        }
    }
}
