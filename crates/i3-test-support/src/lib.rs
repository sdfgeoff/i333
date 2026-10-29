//! Shared, development-only regression process support.
//!
//! Child output stays in Cargo's test capture on success. A failure prints the
//! command, its captured output and a persistent workspace log before failing
//! the Rust test, so the Cargo exit status is the complete pass/fail result.

use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Mutex, MutexGuard};
use std::time::{SystemTime, UNIX_EPOCH};

static SUITE_LOCK: Mutex<()> = Mutex::new(());
static LOG_SEQUENCE: AtomicUsize = AtomicUsize::new(0);

pub fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("test-support crate belongs to a readable workspace")
}

/// Serialize the X-server suites even when the Rust harness has many threads.
pub fn suite_lock() -> MutexGuard<'static, ()> {
    SUITE_LOCK
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

pub fn product_directory(anchor_binary: &str) -> PathBuf {
    let binary = Path::new(anchor_binary);
    assert!(binary.is_file(), "Cargo did not build {}", binary.display());
    let directory = binary
        .parent()
        .expect("Cargo binary has a parent directory");
    for name in [
        "i3",
        "i3bar",
        "i3-msg",
        "i3-dump-log",
        "i3-input",
        "i3-nagbar",
        "i3-config-wizard",
    ] {
        assert!(directory.join(name).is_file(),
            "Missing Cargo-built product {}. Run cargo test from the workspace root; package-only tests can use --bin to select unit tests.", directory.join(name).display());
    }
    directory.to_owned()
}

fn prepare_environment(command: &mut Command, root: &Path) {
    for name in [
        "DISPLAY",
        "I3SOCK",
        "WAYLAND_DISPLAY",
        "XAUTHORITY",
        "DBUS_SESSION_BUS_ADDRESS",
        "NOTIFY_SOCKET",
        "LISTEN_PID",
        "LISTEN_FDS",
        "LISTEN_FDNAMES",
    ] {
        command.env_remove(name);
    }
    let scratch = root.join(".work/tmp");
    fs::create_dir_all(&scratch).expect("create workspace test scratch directory");
    command.env("TMPDIR", scratch).current_dir(root);
}

pub fn run_command(label: &str, command: &mut Command) {
    let root = workspace_root();
    prepare_environment(command, &root);
    let description = format!("{command:?}");
    let output = command
        .output()
        .unwrap_or_else(|error| panic!("Could not start {label}: {description}\n{error}"));
    let log_directory = root.join(".work/cargo-tests");
    fs::create_dir_all(&log_directory).expect("create regression log directory");
    let label: String = label
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() {
                character
            } else {
                '-'
            }
        })
        .collect();
    let log_path = log_directory.join(format!(
        "{label}-{}-{}-{}.log",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos(),
        std::process::id(),
        LOG_SEQUENCE.fetch_add(1, Ordering::Relaxed)
    ));
    let mut captured = output.stdout;
    if !captured.ends_with(b"\n") {
        captured.push(b'\n');
    }
    captured.extend_from_slice(&output.stderr);
    fs::write(&log_path, &captured).expect("write captured regression log");
    assert!(
        output.status.success(),
        "{label} failed ({status}).\nCommand: {description}\nLog: {log}\n\n{captured}",
        status = output.status,
        log = log_path.display(),
        captured = String::from_utf8_lossy(&captured)
    );
    print!("{}", String::from_utf8_lossy(&captured));
}

pub fn run_python(script: &str, arguments: &[OsString]) {
    let root = workspace_root();
    let interpreter =
        std::env::var_os("I333_TEST_PYTHON").unwrap_or_else(|| OsString::from("python3"));
    let mut command = Command::new(interpreter);
    command
        .arg(root.join("scripts").join(script))
        .args(arguments);
    run_command(script, &mut command);
}
