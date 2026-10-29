// Cargo entry point only; source modules retain their original layout.
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
include!("libi3-modules.rs");
#[path = "../i3bar/src/main.rs"]
pub mod program_main;
#[path = "../i3bar/src/child.rs"]
pub mod source_child;
#[path = "../i3bar/src/config.rs"]
pub mod source_config;
#[path = "../i3bar/src/ipc.rs"]
pub mod source_ipc;
#[path = "../i3bar/src/mode.rs"]
pub mod source_mode;
#[path = "../i3bar/src/outputs.rs"]
pub mod source_outputs;
#[path = "../i3bar/src/parse_json_header.rs"]
pub mod source_parse_json_header;
#[path = "../i3bar/src/workspaces.rs"]
pub mod source_workspaces;
#[path = "../i3bar/src/xcb.rs"]
pub mod source_xcb;

fn main() {
    program_main::main();
}
