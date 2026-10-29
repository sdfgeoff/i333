# i3 Rust port

A file-by-file Rust port of i3, preserving its original interfaces and
behavior. A Cargo workspace shares Rust helpers, JSON streaming and native library
boundaries. All seven product executables build from Rust. Native X11 and
rendering libraries remain dependencies; C sources, headers and the Meson build
have been removed from this tree.

1. Install nightly Rust, a native linker, `pkg-config`, and the native development
   libraries listed in [repository details](docs/development.md).
2. Run `cargo build --workspace --locked`. Binaries appear in `target/debug/`: `i3`, `i3bar`,
   `i3-msg`, `i3-input`, `i3-nagbar`, `i3-config-wizard` and `i3-dump-log`.
3. Run `cargo build --workspace --locked --release` for optimized binaries in `target/release/`.
4. Run `cargo test --workspace --locked` for workspace unit tests.
5. Run `python3 scripts/test-cargo-build.py` to verify a fresh build with C
   compilation and legacy build tools blocked. It checks version output without
   connecting to a desktop session.

Building does not install or launch a window manager. Configuration, helpers,
manuals and session resources remain available for packaging. See
[PORTING.md](PORTING.md) for compatibility and isolated regression testing, and
[recorded validation](docs/validation.json) for test evidence.

Shared helpers now own text, paths and IPC messages through Rust APIs. JSON uses
`serde_json`; maintained GTK and XKB bindings handle native interoperability. The
window tree still retains unsafe pointers, intrusive lists and legacy allocation
contracts. The port currently supports Linux x86_64 with glibc. This project
derives from [i3](https://i3wm.org/); original copyright and license notices remain in the Rust sources and [LICENSE](LICENSE).
