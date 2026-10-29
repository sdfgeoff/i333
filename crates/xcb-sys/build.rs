fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    for library in [
        "xcb",
        "xcb-xkb",
        "xcb-xinerama",
        "xcb-randr",
        "xcb-shape",
        "xcb-util",
        "xcb-cursor",
        "xcb-keysyms",
        "xcb-icccm",
        "xcb-xrm",
    ] {
        pkg_config::Config::new()
            .probe(library)
            .unwrap_or_else(|error| panic!("{library}: {error}"));
    }
}
