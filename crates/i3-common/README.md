# i3-common

Internal shared helpers for the window manager, bar and command-line tools.
Cargo compiles this library once for the workspace. Native library discovery
runs in its build script through `i3-build`.

The current helpers retain their native ABI and allocation contracts. Drawing
code uses display state supplied by its executable; logging callbacks are also
provided by the executable. This is an internal library rather than a stable
public API.

Run `cargo test -p i3-common` for the IPC and helper contract tests. Tests use
anonymous sockets and temporary files without connecting to X.
