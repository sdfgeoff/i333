fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    pkg_config::Config::new()
        .probe("libstartup-notification-1.0")
        .expect("startup-notification development library is required");
}
