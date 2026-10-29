//! Integration targets make Cargo build every utility, including binaries whose
//! Rust unit harness is disabled, before the workspace regression tests start.
#[test]
fn cargo_builds_all_five_regular_utility_products() {
    for product in [
        env!("CARGO_BIN_EXE_i3-msg"),
        env!("CARGO_BIN_EXE_i3-dump-log"),
        env!("CARGO_BIN_EXE_i3-input"),
        env!("CARGO_BIN_EXE_i3-nagbar"),
        env!("CARGO_BIN_EXE_i3-config-wizard"),
    ] {
        assert!(std::path::Path::new(product).is_file(), "Missing {product}");
    }
}
