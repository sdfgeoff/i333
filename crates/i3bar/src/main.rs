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
    application::main();
}
