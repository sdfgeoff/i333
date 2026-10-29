# i3 Rust port

1. **Compatibility first.** The port preserves upstream function names, control
   flow, C ABI, struct layouts, globals, callbacks and allocation ownership.
   XCB, libev, Cairo, Pango, YAJL and PCRE2 remain native backends. Architecture
   and ownership cleanup are separate work.
2. **Provenance.** The baseline is upstream
   [`be04c853ee7fa01a82a0c81f8217565395e2627c`](https://github.com/i3/i3/tree/be04c853ee7fa01a82a0c81f8217565395e2627c).
   All 90 product C files were translated, with 89 translation units configured
   on the original host; `strndup` was an inactive portability fallback. Those
   counts describe the migration baseline, not current source adjacency. C
   sources, headers, Meson and translation scaffolding are no longer in this
   tree. Retrieve the baseline from Git history or a separate upstream checkout
   for source comparisons. Original license notices are retained in Rust.
3. **Method.** [Bun's adjacent-file workflow](https://github.com/oven-sh/bun/blob/3157cb14b5970b69532a47800504a28ef5963e22/docs/PORTING.md)
   guided the translation, with compilation and validation following separately.
   Most files were translated mechanically using C2Rust 0.22.1 and LLVM 21;
   unsafe pointers deliberately retain the original ownership contracts.
4. **Build.** Use `cargo build --workspace --locked` or `cargo build --workspace --locked --release`.
   Nightly Rust, a native linker, `pkg-config` and the development libraries in
   [repository details](docs/development.md) are required. All parser tables are
   checked into Rust. Cargo does not invoke Meson, Perl, C2Rust or a C compiler
   to compile project sources. A C compiler driver may still invoke the native
   linker. The workspace separates the window manager (`i3`), shared native-ABI
   helper library (`i3-common`), bar (`i3bar`), five utility binaries
   (`i3-tools`) and centralized build configuration (`i3-build`). Standard module
   declarations keep source inside each crate; native link discovery runs once
   through the shared library build script.
5. **Test isolation.** Never replace, restart, signal, or send commands to the
   user's running i3. Never move the pointer or inject input on their display.
   Runtime tests require a separate headless X server, private IPC socket,
   private configuration and scoped environment. Building and CI must not
   install the binaries into a desktop session.
6. **Regression reference.** `cargo test --workspace --locked` runs workspace
   unit tests. `scripts/test-cargo-build.py` verifies a fresh Rust
   source copy with C compilation and old generators blocked. Differential
   IPC and headless runners exercise Rust directly; `--reference` optionally
   supplies an external upstream executable for differential checks. The full
   upstream suite requires `--reference-build` with three external precompiled
   test helpers and private X servers. Consult each runner's `--help`. External
   C test fixtures and Perl harness dependencies are regression tools, not Rust
   build dependencies.
7. **Recorded validation.** The complete upstream suite passed for both C and
   Rust: 285 files and 4,205 assertions. Helper, IPC and headless comparisons are
   recorded in [docs/validation.json](docs/validation.json), with tested binary
   hashes and profiles. Historical validation does not prove every behavior or
   validate a new build automatically.
8. **Limits.** Current ABI definitions target Linux x86_64 with glibc, expanded
   from the original host headers. Other targets and compile-time variants
   require deliberate updates and separate validation. Preserve malloc/free
   ownership, widths, native-endian IPC, nulls and error codes across FFI.
9. **Small safety changes.** `mkdirp` accepts an empty path without reading a
   preceding byte; `i3-dump-log` omits an unused out-of-bounds NUL write after a
   full read. Normal outputs and return values match the reference cases.
