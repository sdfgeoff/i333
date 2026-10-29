//! Every runtime suite runs on private test displays and propagates its status
//! through the ordinary Cargo test result. Cargo builds the other products via
//! the artifact integration targets in i3bar and i3-tools before running tests.

fn suite(name: &str) {
    let _serial = i3_test_support::suite_lock();
    let binaries = i3_test_support::product_directory(env!("CARGO_BIN_EXE_i3"));
    i3_test_support::run_python(
        "run-regression.py",
        &[
            "--suite".into(),
            name.into(),
            "--bin-dir".into(),
            binaries.into_os_string(),
        ],
    );
}

#[test]
fn ipc_16_fixture_groups() {
    suite("ipc");
}
#[test]
fn cli_30_core_and_bar_cases() {
    suite("cli");
}
#[test]
fn headless_52_window_manager_checkpoints() {
    suite("headless");
}
#[test]
fn input_17_private_keyboard_checks() {
    suite("input");
}
#[test]
fn upstream_285_files_4205_assertions() {
    suite("upstream");
}

#[test]
fn nagbar_4_private_render_and_close_checks() {
    suite("nagbar");
}
