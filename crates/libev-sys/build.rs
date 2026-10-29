fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    assert_eq!(
        std::env::var("TARGET").as_deref(),
        Ok("x86_64-unknown-linux-gnu"),
        "These libev watcher layouts currently target x86_64-unknown-linux-gnu"
    );
    if pkg_config::Config::new().probe("libev").is_err() {
        // Several distributions ship libev without a pkg-config file.
        println!("cargo:rustc-link-lib=ev");
    }
}
