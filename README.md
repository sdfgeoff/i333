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
3. Prepare regression dependencies once: `python3 scripts/prepare-tests.py`.
   It keeps the pinned C test reference, Perl/Python modules and cached tools in
   `.work/`; see [one-time prerequisites](docs/development.md).
4. Run **`cargo test`**. This builds all seven binaries and runs unit tests,
   native compatibility oracles, status-header and IPC/CLI comparisons, private-display checks and
   all 4,205 upstream assertions. Any failure fails the Cargo command; detailed
   logs remain in `.work/`. Normal test runs need no network or setup commands.
5. Run `cargo build --workspace --locked --release` for optimized binaries in
   `target/release/`. The slower fresh-source build audit is optional:
   `cargo test --features i3-test-support/clean-source-check`.

Building does not install or launch a window manager. Tests launch private X
servers and isolated window managers; they never replace your active desktop. Configuration, helpers,
manuals and session resources remain available for packaging. See
[PORTING.md](PORTING.md) for compatibility and isolated regression testing, and
[recorded validation](docs/validation.json) for test evidence.

Shared helpers now own text, paths and IPC messages through Rust APIs. JSON uses
`serde_json`; maintained GTK and XKB bindings handle native interoperability. The
window tree still retains unsafe pointers, intrusive lists and legacy allocation
contracts. The port currently supports Linux x86_64 with glibc. This project
derives from [i3](https://i3wm.org/); original copyright and license notices remain in the Rust sources and [LICENSE](LICENSE).
