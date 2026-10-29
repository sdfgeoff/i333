# Building and packaging

1. **Workspace layout.** The root `Cargo.toml` centralizes dependency versions,
   package metadata and build profiles. The members live under `crates/`:

   | Member | Responsibility |
   | --- | --- |
   | `i3` | Window manager executable |
   | `i3-common` | Owned text, paths, IPC, CLI, logging, display context and rendering helpers |
   | `i3bar` | Status bar executable |
   | `i3-tools` | `i3-msg`, `i3-dump-log`, `i3-input`, `i3-nagbar`, `i3-config-wizard` |
   | `i3-build` | Shared version, target and compiler configuration |
   | `i3-json` | Rust streaming JSON parser/generator using `serde_json` |
   | `libev-sys` | Canonical native event-loop handles and watcher layouts |
   | `xcb-sys` | Shared XCB protocol and extension boundary |
   | `startup-notification-sys` | Native startup-notification handles and callbacks |

   Each crate uses ordinary module declarations with source inside its own
   directory. Product crates depend on the common `rlib`; they no longer compile
   independent copies of its modules. `i3-build::configure()` supplies each
   member's version, target check and variadic compatibility cfg. Native link
   discovery belongs to each native boundary crate and the maintained binding
   dependencies; Cargo propagates their native linking metadata. The common
   build script adds only the platform `m` and `rt` libraries.
2. **Build and unit tests.** Nightly Rust is selected by `rust-toolchain.toml`.
   Run `cargo build --workspace --locked`,
   `cargo build --workspace --locked --release`, and
   `cargo test --workspace --locked`. All seven binary names remain unchanged
   in `target/debug/` or `target/release/`. A native linker and the libraries below
   are required. Builds and unit tests do not install files or launch X.
3. **Native dependencies.** Workspace and maintained binding build scripts
   discover these installed libraries through pkg-config:

   ```text
   libstartup-notification-1.0
   xcb xcb-xkb xcb-xinerama xcb-randr xcb-shape
   xcb-util xcb-cursor xcb-keysyms xcb-icccm xcb-xrm
   xkbcommon xkbcommon-x11
   libpcre2-8
   cairo pangocairo glib-2.0 gobject-2.0
   ```

   libev is linked as `ev`; it need not provide a `.pc` file. Pango, HarfBuzz and
   other native dependencies arrive transitively. libc, `m` and `rt` are also
   linked. Install the corresponding development packages. The Ubuntu package
   list is maintained in [the Cargo CI workflow](../.github/workflows/main.yml).
   `pkg-config` discovers libraries; it generates no bindings or project code.
   YAJL is not a build or runtime dependency. Its optional test oracle needs only
   the host YAJL shared library, without headers or generated bindings.
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
   external reference paths and use private sockets/headless displays. Test
   scripts keep scratch files and logs under the ignored `.work/` directory;
   set `TMPDIR` to override the default `.work/tmp` scratch location.
   `python3 scripts/test-cargo-build.py` separately verifies a clean workspace
   build without C sources or existing project build outputs. Never send
   commands or input to the developer's active i3/display during testing.
7. **Platform scope.** Build support currently accepts
   `x86_64-unknown-linux-gnu`. Structs and native calls reflect that ABI. Version
   strings come from `I3_VERSION`, workspace Git metadata when available, or the
   calling package's version fallback. ABI, installation-prefix and architecture
   changes require deliberate updates and regression checks. Recorded results
   and tested artifact hashes are in [validation.json](validation.json).

8. **Rust and native boundaries.** File names continue to follow upstream. Owned
   text, legacy glyph buffers, paths, process filenames, configuration lookup, CLI
   scanning and IPC buffers use Rust APIs; translated callers retain small
   compatibility adapters. Core model definitions are shared within the core
   crate. The window tree, intrusive lists and some rendering allocation paths
   still use unsafe pointers and preserve their existing ownership contracts.
   This remains an incremental port rather than a completed ownership redesign.

   Maintained `cairo-sys-rs`, `pango-sys`, `pangocairo-sys` and `gobject-sys`
   provide GTK rendering bindings. Maintained `xkbcommon` supplies its raw and
   X11 interfaces. The maintained `xcb` crate supplies public generic events,
   errors and basic geometry types; `as-raw-xcb-connection` supplies the common
   connection handle. Most raw extension and utility entry points are private
   in the maintained XCB crate, so `xcb-sys` centralizes the remaining native
   declarations without changing connection ownership or generating C headers.
   Native libev watcher layouts and startup-notification callbacks have their
   own boundary crates. `libc` owns C runtime types and calls; the small
   `i3_common::native_stdio` module contains APIs missing from its public API,
   including compiler-specific variadic forwarding and `wordexp`.

   Core regular expressions use the maintained `pcre2` crate with UTF/UCP
   behavior. PCRE2 remains native because Rust's regular expression engines do
   not provide its complete pattern syntax and matching behavior. The project
   links system PCRE2 when its development package is installed. Without it,
   `pcre2-sys` can compile its bundled C fallback; the clean-build check blocks
   that fallback. Install the native development package for normal builds.

   Generated scalar typedef chains such as `__uint32_t` → `uint32_t` now use
   `u32` directly; lengths use `usize`, and platform ABI types and constants come
   from `libc`. Closed application choices use named Rust enums throughout their
   storage, comparisons and matches, including window layouts, rendering modes,
   parser states, status-header keys and utility screens. The audit covers 42
   distinct closed application families, represented by 47 enum definitions where
   separate components retain their own types. Existing discriminants and
   required representations remain stable. Enum fields in allocations still
   initialized with zero retain a valid zero variant; this preserves the legacy
   allocation contract without inventing invalid Rust values.

   Keep bit masks and native protocol integers as flags or raw binding types.
   Open-ended JSON numbers and unknown protocol values must retain their full
   ranges; validate them at the boundary before constructing a closed Rust enum.
   Native enum-like constants are not automatically application enums. Array
   lengths and count sentinels remain integer constants outside their index enum.

   Internal Rust functions use Rust linkage. Logging sinks and display context
   providers replace executable symbol lookup; providers run on their registered
   event thread. Native callbacks, loader initialization hooks and language-
   required variadic adapters retain the C ABI.
9. **JSON compatibility checks.** `i3-json` uses owned streaming state and
   `serde_json` scalar encoding/decoding. It emits events before a never-closed
   i3bar outer array finishes. Existing core/bar files still use pointer callback
   adapters in `i3_json::compat`; these are Rust functions without YAJL linking.
   New code can use `Parser` and `Generator` directly.

   Run `python3 scripts/test-json-parity.py --offline` for an optional independent
   ctypes comparison with installed YAJL. `--require-oracle` makes a missing
   library fail; `--report PATH` writes detailed counterexamples. The probe is a
   temporary Rust package and opens no display. It covers byte boundaries,
   streaming updates, flags, cancellation, overflow and generation. Separate
   report categories identify invalid-input diagnostic progress, deliberate
   rejection of malformed Unicode accepted by YAJL, and equivalent floating
   point encodings. These do not imply exact malformed-input diagnostics or
   byte-for-byte floating point formatting. Raw number events preserve arbitrarily
   large number text. Legacy typed callbacks retain YAJL's signed-magnitude
   integer limit, including its rejection of `i64::MIN`; this does not limit the
   safe parser's raw number events. Default Rust parsing also rejects
   non-JSON vertical-tab/form-feed whitespace; the compatibility adapter enables
   YAJL's legacy whitespace behavior.

   `python3 scripts/test-regex-parity.py --offline` similarly compares the owned
   regex wrapper with the installed PCRE2 library. Neither oracle becomes a
   dependency of the Rust executables.
