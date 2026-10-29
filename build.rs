use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    assert_eq!(
        env::var("TARGET").as_deref(),
        Ok("x86_64-unknown-linux-gnu"),
        "The translated native ABI currently supports x86_64-unknown-linux-gnu only"
    );
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=I3_VERSION");
    println!("cargo:rustc-check-cfg=cfg(i333_new_va_list)");
    // Rust 1.99 stabilized C variadics with a different VaList API. Retain
    // compatibility with the older compiler used for the original translation.
    let rustc = env::var_os("RUSTC").expect("Cargo supplies RUSTC");
    let compiler = Command::new(rustc)
        .arg("--version")
        .output()
        .expect("query rustc version");
    let compiler = String::from_utf8_lossy(&compiler.stdout);
    let minor = compiler
        .split_whitespace()
        .nth(1)
        .and_then(|version| version.split('.').nth(1))
        .and_then(|minor| minor.parse::<u32>().ok())
        .expect("rustc version number");
    if minor >= 99 {
        println!("cargo:rustc-cfg=i333_new_va_list");
    }
    // Git supplies development version information when available. Building a
    // source archive needs neither Git nor the original Meson config.h.
    for path in [".git/HEAD", ".git/packed-refs"] {
        if Path::new(path).is_file() {
            println!("cargo:rerun-if-changed={path}");
        }
    }
    if let Ok(head) = std::fs::read_to_string(".git/HEAD") {
        if let Some(reference) = head.trim().strip_prefix("ref: ") {
            let path = format!(".git/{reference}");
            if Path::new(&path).is_file() {
                println!("cargo:rerun-if-changed={path}");
            }
        }
    }
    let version = env::var("I3_VERSION")
        .ok()
        .or_else(|| {
            if !Path::new(".git").exists() {
                return None;
            }
            let output = Command::new("git")
                .args(["describe", "--always", "--dirty=+"])
                .output()
                .ok()?;
            output
                .status
                .success()
                .then(|| String::from_utf8_lossy(&output.stdout).trim().to_owned())
        })
        .unwrap_or_else(|| format!("{}-rust", env!("CARGO_PKG_VERSION")));
    assert!(
        !version.contains(['\n', '\r', '\0']),
        "I3_VERSION must be a single line without NUL"
    );
    println!("cargo:rustc-env=I3_VERSION={version}");

    // Preserve the existing native backends. pkg-config only locates libraries;
    // it does not generate bindings or compile any C source or headers.
    for package in [
        "libstartup-notification-1.0",
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
        "xkbcommon",
        "xkbcommon-x11",
        "yajl",
        "libpcre2-8",
        "cairo",
        "pangocairo",
        "glib-2.0",
        "gobject-2.0",
    ] {
        pkg_config::Config::new().probe(package).unwrap_or_else(|error| {
            panic!("Native dependency {package} is unavailable. Install its development package and pkg-config.\n{error}")
        });
    }
    // libev has no .pc file on some distributions, including this test host.
    for library in ["ev", "m", "rt"] {
        println!("cargo:rustc-link-lib={library}");
    }
}
