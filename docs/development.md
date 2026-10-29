# Building and packaging

1. **Workspace layout.** The root `Cargo.toml` centralizes dependency versions,
   package metadata and build profiles. The five members live under `crates/`:

   | Member | Responsibility |
   | --- | --- |
   | `i3` | Window manager executable |
   | `i3-common` | Shared Rust library for native-ABI helpers |
   | `i3bar` | Status bar executable |
   | `i3-tools` | `i3-msg`, `i3-dump-log`, `i3-input`, `i3-nagbar`, `i3-config-wizard` |
   | `i3-build` | Shared build configuration and native dependency discovery |

   Each crate uses ordinary module declarations with source inside its own
   directory. Product crates depend on the common `rlib`; they no longer compile
   independent copies of its modules. `i3-build::configure()` supplies each
   member's version, target check and variadic compatibility cfg. Native link
   discovery runs only in `crates/i3-common/build.rs`; its library metadata
   propagates native linking requirements to the executables.
2. **Build and unit tests.** Nightly Rust is selected by `rust-toolchain.toml`.
   Run `cargo build --workspace --locked`,
   `cargo build --workspace --locked --release`, and
   `cargo test --workspace --locked`. All seven binary names remain unchanged
   in `target/debug/` or `target/release/`. A native linker and the libraries below
   are required. Builds and unit tests do not install files or launch X.
3. **Native dependencies.** `crates/i3-build/src/lib.rs` discovers these
   pkg-config modules when called by the common crate's build script:

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
4. **Parser tables.** Token tables, enums and dispatch routines are compiled
   into `crates/i3/src/commands_parser.rs`,
   `crates/i3/src/config_parser.rs` and
   `crates/i3-tools/src/bin/i3-config-wizard/application.rs`. `GENERATED_*.h`
   strings identify diagnostic source locations; they are not includes or file
   reads. No parser generator, C headers, configured `build/` directory, Meson,
   Ninja, Perl or C2Rust is needed to build the Rust products. Parser behavior
   changes must update these Rust tables directly or regenerate from a
   separately retrieved baseline.
5. **Runtime resources.** `etc/config` and `etc/config.keycodes` remain available.
   The original configured system prefix is still `/usr/local/etc`: the wizard
   reads `/usr/local/etc/i3/config.keycodes`, and system configuration discovery
   uses that prefix. User configuration and explicit config paths retain their
   behavior. Cargo installs none of these files. Shell/Perl helpers such as
   `i3-sensible-terminal`, `i3-dmenu-desktop` and `i3-save-tree`, session
   descriptors in `share/`, manuals and assets remain distribution resources.
   Packaging must arrange configuration, helper search paths and session
   registration. Perl may be needed by a runtime helper independently of Cargo.
6. **Source history and regression tests.** C sources, headers and the old build
   system have been removed. Rust modules retain source-path provenance and
   copyright/license comments. [The porting notes](../PORTING.md) identify the
   exact upstream baseline; Git history or an external upstream clone supplies
   source references and C regression fixtures. Regression scripts accept
   external reference paths and use private sockets/headless displays.
   `python3 scripts/test-cargo-build.py` separately verifies a clean workspace
   build without C sources or existing project build outputs. Never send
   commands or input to the developer's active i3/display during testing.
7. **Platform scope.** Build support currently accepts
   `x86_64-unknown-linux-gnu`. Structs and native calls reflect that ABI. Version
   strings come from `I3_VERSION`, workspace Git metadata when available, or the
   calling package's version fallback. ABI, installation-prefix and architecture
   changes require deliberate updates and regression checks. Recorded results
   and tested artifact hashes are in [validation.json](validation.json).
