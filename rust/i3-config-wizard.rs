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
#[path = "../i3-config-wizard/main.rs"]
pub mod program_main;

fn main() {
    program_main::main();
}
