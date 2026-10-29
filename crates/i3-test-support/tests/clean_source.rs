#[test]
fn fresh_source_build_without_c_compilation_or_generators() {
    i3_test_support::run_python("test-cargo-build.py", &[]);
}
