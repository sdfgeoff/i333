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
   | `i3-test-support` | Development-only Cargo regression orchestration and captured logs |
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
2. **Build and tests.** Nightly Rust is selected by `rust-toolchain.toml`.
   `cargo build --workspace --locked` builds all seven binaries in `target/debug/`;
   add `--release` for `target/release/`. Building installs nothing and opens no
   display. After the one-time regression setup below, **`cargo test`** is the
   complete test command: workspace unit tests, native JSON/regex/variadic
   compatibility checks, 602 status-header comparisons, 16 IPC groups, 30 CLI cases, 52 headless state
   comparisons, 17 input cases and all 4,205 retained upstream assertions.
   Cargo supplies the actual executable paths, so custom target directories and
   test profiles use the products from that invocation. Runtime checks use only
   private sockets and X servers. Successful child output is captured; failures
   fail Cargo and identify retained logs under `.work/cargo-tests/` or `.work/tmp/`.
   One standard nightly run is sufficient; older-toolchain compatibility is a
   separate explicit check. If the shell sets `RUSTUP_TOOLCHAIN`, that overrides
   the repository file. Use `env -u RUSTUP_TOOLCHAIN cargo test` to select this
   repository's nightly without changing global Rust configuration.
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
   Rust products neither build nor link YAJL. Complete test preparation needs
   its development package for the pinned external C reference; the default
   native JSON oracle uses only its shared library. These are test prerequisites,
   not a Rust product dependency.
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
6. **One-time regression setup.** Run this once after installing native build
   dependencies, and again if the prepared test tools are removed:

   ```sh
   python3 scripts/prepare-tests.py
   cargo test
   ```

   Setup is explicit: it may download dependencies and compile the external C
   reference. It installs nothing globally, opens no display and keeps its cache
   and environment manifest in `.work/test-deps/`. Routine Cargo tests never run
   package managers, download tools or invoke Cargo recursively. Missing
   prerequisites fail with this setup command rather than silently skipping tests.

   The extra test prerequisites are Python 3, Perl, a C compiler, `pkg-config`,
   Ninja, Xvfb (including `xvfb-run`), Xephyr, `xauth`, `xkbcomp`, YAJL headers,
   and XCB XTEST development headers. Python needs venv/pip support or `uv`.
   Perl XS module builds also need Perl and Expat development headers. On Ubuntu,
   the additional packages include `python3-venv`, `perl`, `libperl-dev`,
   `libexpat1-dev`, `ninja-build`, `xvfb`, `xserver-xephyr`, `xauth`,
   `x11-xkb-utils`, `libxcb-xtest0-dev`, `libxcb-composite0-dev`, `xcb-proto`
   and `libyajl-dev`. The Perl XCB fixture needs the composite development
   library and xcb-proto XML metadata even though Rust products do not. On Arch,
   setup can extract pinned Xvfb/Xephyr packages into `.work` when those executables are
   absent; their matching native runtime libraries must already be available.

   Setup reuses valid existing tools, installs `python-xlib==0.33` in a local
   interpreter when needed, and prepares the required Perl modules locally. It
   extracts upstream revision `be04c853ee7fa01a82a0c81f8217565395e2627c` from Git
   history or a cached download and builds its seven reference executables and
   three parser/RandR helpers once. C sources, Meson and headers stay outside the
   product tree in `.work`. Unchanged Inline::C fixtures share a persistent cache;
   new or changed fixture source compiles locally on first use. C/Rust comparisons
   normalize only human-readable build stamps, not numeric IPC version fields or
   behavior. The cached reference therefore needs no rebuild after each commit.

   `python3 scripts/prepare-tests.py --offline` verifies/reuses prepared tools;
   `--rebuild-reference` explicitly rebuilds the pinned C fixture. The resulting
   `.work/test-deps/environment.json` records paths. `I333_TEST_ENVIRONMENT`
   selects another manifest, and `I333_TEST_REFERENCE_BUILD`, `I333_TEST_PERL_LIB`,
   `I333_TEST_XVFB`, `I333_TEST_XEPHYR` and `I333_TEST_PYTHON` override individual
   paths. Direct Python runners retain their explicit executable/path options.
   Test artifacts default to `.work/tmp`; `TMPDIR` overrides that location.

   The slower structural audit is maintenance-only:
   `cargo test --features i3-test-support/clean-source-check`, or
   `python3 scripts/test-cargo-build.py`. It builds a fresh Rust-only copy with
   C compilation and legacy tools blocked, compiles all test targets with
   `cargo test --no-run`, and checks seven version outputs. It never recursively
   executes the regression suite. Ordinary `cargo test` already runs all actual
   tests. Runtime helpers/assets remain packaging resources; Git history retains
   the C migration baseline. Never send commands or input to the active desktop.
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

   Cargo runs the JSON/PCRE/variadic compatibility drivers using already-built
   Rust probe executables. Their Python code uses ctypes only for independent
   native oracles and opens no display. Missing required oracle libraries fail
   the Cargo run; no nested Cargo builds occur. The direct Python driver options
   and `--report PATH` remain useful when investigating counterexamples. It covers byte boundaries,
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

   The regex driver compares the owned wrapper with the installed PCRE2 library.
   Neither oracle becomes a dependency of the Rust executables.
