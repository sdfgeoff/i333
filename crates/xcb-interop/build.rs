fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    for library in ["xcb-cursor", "xcb-keysyms", "xcb-xrm", "xcb-util"] {
        pkg_config::Config::new()
            .probe(library)
            .unwrap_or_else(|error| panic!("{library}: {error}"));
    }
}
