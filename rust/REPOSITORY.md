# Building and packaging

1. **Cargo inputs.** `Cargo.toml`, `Cargo.lock`, `build.rs`, `rust/*.rs` entry
   points and the translated modules build all seven executables. Nightly Rust
   is selected by `rust-toolchain.toml`; C variadics and opaque foreign types
   retain the original ABI. A native linker and the libraries below remain
   required. `cargo build --locked` writes `target/debug/`; adding `--release`
   writes `target/release/`. The build does not install files or launch X.
2. **Native dependencies.** `build.rs` locates these pkg-config modules:

   ```text
   libstartup-notification-1.0
   xcb xcb-xkb xcb-xinerama xcb-randr xcb-shape
   xcb-util xcb-cursor xcb-keysyms xcb-icccm xcb-xrm
   xkbcommon xkbcommon-x11
   yajl libpcre2-8
   cairo pangocairo glib-2.0 gobject-2.0
   ```

   libev is linked as `ev`; it need not provide a `.pc` file. Pango, HarfBuzz and
   other native dependencies arrive transitively. libc, `m` and `rt` are also
   linked. Install the corresponding development packages. The Ubuntu package
   list is maintained in [the Cargo CI workflow](../.github/workflows/main.yml).
   `pkg-config` discovers libraries; it generates no bindings or project code.
3. **Parser tables.** Token tables, enums and dispatch routines are already
   compiled into `src/commands_parser.rs`, `src/config_parser.rs` and the
   configuration wizard's `main.rs`. `GENERATED_*.h` strings identify diagnostic
   source locations; they are not includes or file reads. No parser generator,
   C headers, configured `build/` directory, Meson, Ninja, Perl or C2Rust is
   needed to build the Rust products. Parser behavior changes must update these
   Rust tables directly or regenerate from a separately retrieved baseline.
4. **Runtime resources.** `etc/config` and `etc/config.keycodes` remain available.
   The original configured system prefix is still `/usr/local/etc`: the wizard
   reads `/usr/local/etc/i3/config.keycodes`, and system configuration discovery
   uses that prefix. User configuration and explicit config paths retain their
   behavior. Cargo installs none of these files. Shell/Perl helpers such as
   `i3-sensible-terminal`, `i3-dmenu-desktop` and `i3-save-tree`, session
   descriptors in `share/`, manuals and assets remain distribution resources.
   Packaging must arrange configuration, helper search paths and session
   registration. Perl may be needed by a runtime helper independently of Cargo.
5. **Source history and tests.** C sources, headers and the old build system have
   been removed. Rust files retain their corresponding source-path provenance
   and copyright/license comments. [The porting notes](../PORTING.md) identify
   the exact upstream baseline; Git history or an external upstream clone
   supplies source references and C regression fixtures. Regression scripts
   accept external reference paths and use private sockets/headless displays.
   `python3 scripts/test-cargo-build.py` separately verifies a clean Rust build
   without consulting C sources or existing project build outputs.
6. **Platform scope.** `build.rs` currently accepts
   `x86_64-unknown-linux-gnu`. Structs and native calls reflect that ABI. Version
   strings come from `I3_VERSION`, Git metadata when available, or the Cargo
   package version fallback. ABI, installation-prefix and architecture changes
   require deliberate updates and regression checks.
