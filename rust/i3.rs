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
#[path = "../src/main.rs"]
pub mod program_main;
#[path = "../src/assignments.rs"]
pub mod source_assignments;
#[path = "../src/bindings.rs"]
pub mod source_bindings;
#[path = "../src/click.rs"]
pub mod source_click;
#[path = "../src/commands.rs"]
pub mod source_commands;
#[path = "../src/commands_parser.rs"]
pub mod source_commands_parser;
#[path = "../src/con.rs"]
pub mod source_con;
#[path = "../src/config.rs"]
pub mod source_config;
#[path = "../src/config_directives.rs"]
pub mod source_config_directives;
#[path = "../src/config_parser.rs"]
pub mod source_config_parser;
#[path = "../src/display_version.rs"]
pub mod source_display_version;
#[path = "../src/drag.rs"]
pub mod source_drag;
#[path = "../src/ewmh.rs"]
pub mod source_ewmh;
#[path = "../src/fake_outputs.rs"]
pub mod source_fake_outputs;
#[path = "../src/floating.rs"]
pub mod source_floating;
#[path = "../src/gaps.rs"]
pub mod source_gaps;
#[path = "../src/handlers.rs"]
pub mod source_handlers;
#[path = "../src/ipc.rs"]
pub mod source_ipc;
#[path = "../src/key_press.rs"]
pub mod source_key_press;
#[path = "../src/load_layout.rs"]
pub mod source_load_layout;
#[path = "../src/log.rs"]
pub mod source_log;
#[path = "../src/manage.rs"]
pub mod source_manage;
#[path = "../src/match.rs"]
pub mod source_match;
#[path = "../src/move.rs"]
pub mod source_move;
#[path = "../src/output.rs"]
pub mod source_output;
#[path = "../src/parser_util.rs"]
pub mod source_parser_util;
#[path = "../src/randr.rs"]
pub mod source_randr;
#[path = "../src/regex.rs"]
pub mod source_regex;
#[path = "../src/render.rs"]
pub mod source_render;
#[path = "../src/resize.rs"]
pub mod source_resize;
#[path = "../src/restore_layout.rs"]
pub mod source_restore_layout;
#[path = "../src/scratchpad.rs"]
pub mod source_scratchpad;
#[path = "../src/sd-daemon.rs"]
pub mod source_sd_daemon;
#[path = "../src/sighandler.rs"]
pub mod source_sighandler;
#[path = "../src/startup.rs"]
pub mod source_startup;
#[path = "../src/sync.rs"]
pub mod source_sync;
#[path = "../src/tiling_drag.rs"]
pub mod source_tiling_drag;
#[path = "../src/tree.rs"]
pub mod source_tree;
#[path = "../src/util.rs"]
pub mod source_util;
#[path = "../src/version.rs"]
pub mod source_version;
#[path = "../src/window.rs"]
pub mod source_window;
#[path = "../src/workspace.rs"]
pub mod source_workspace;
#[path = "../src/x.rs"]
pub mod source_x;
#[path = "../src/xcb.rs"]
pub mod source_xcb;
#[path = "../src/xcursor.rs"]
pub mod source_xcursor;
#[path = "../src/xinerama.rs"]
pub mod source_xinerama;

fn main() {
    program_main::main();
}
