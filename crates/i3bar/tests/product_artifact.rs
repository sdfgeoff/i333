//! Request the regular binary alongside this package's unit-test binary.
#[test]
fn cargo_builds_regular_bar_product() {
    assert!(std::path::Path::new(env!("CARGO_BIN_EXE_i3bar")).is_file());
}
