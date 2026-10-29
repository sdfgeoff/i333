# i3 Rust port

A file-by-file Rust port of i3, preserving its original interfaces and
architecture. All seven product executables build from Rust. Native X11 and
rendering libraries remain dependencies; C sources, headers and the Meson build
have been removed from this tree.

1. Install nightly Rust, a native linker, `pkg-config`, and the native development
   libraries listed in [repository details](rust/REPOSITORY.md).
2. Run `cargo build --locked`. Binaries appear in `target/debug/`: `i3`, `i3bar`,
   `i3-msg`, `i3-input`, `i3-nagbar`, `i3-config-wizard` and `i3-dump-log`.
3. Run `cargo build --locked --release` for optimized binaries in `target/release/`.
4. Run `python3 scripts/test-cargo-build.py` to verify a fresh build with C
   compilation and legacy build tools blocked. It checks version output without
   connecting to a desktop session.

Building does not install or launch a window manager. Configuration, helpers,
manuals and session resources remain available for packaging. See
[PORTING.md](PORTING.md) for compatibility and isolated regression testing, and
[recorded validation](rust/VALIDATION.json) for test evidence.

The port currently supports Linux x86_64 with glibc and retains unsafe C ownership
patterns. This project derives from [i3](https://i3wm.org/); original copyright
and license notices remain in the Rust sources and [LICENSE](LICENSE).
