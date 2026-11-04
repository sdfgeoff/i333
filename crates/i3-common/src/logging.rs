//! Standard Rust logging shares the program's event-thread byte sink. Native
//! byte diagnostics remain available without UTF-8 conversion through `emit`.
#![cfg_attr(not(test), deny(unsafe_code))]

mod compat;
pub use crate::native_stdio::{fixed_decimal, FixedPrecision};
pub use compat::{pointer, string_bytes};

use std::cell::Cell;
use std::io::Write;
use std::sync::{Once, RwLock};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Level {
    Error,
    Verbose,
    Debug,
}

pub type Sink = fn(Level, &[u8]);
#[derive(Clone, Copy)]
struct RegisteredSink {
    callback: Sink,
    thread: std::thread::ThreadId,
}
static SINK: RwLock<Option<RegisteredSink>> = RwLock::new(None);

// A const, Copy-only slot has no lazy value allocation or TLS destructor. It is
// touched during normal registration before signal handlers can be installed.
thread_local! {
    static EMERGENCY_SINK: Cell<Option<Sink>> = const { Cell::new(None) };
}

/// Switch this already-prepared thread to its known emergency backend. No
/// shared sink lock or Rust ThreadId lookup occurs here or on subsequent emits.
/// The override lasts until exit/exec; unrelated threads retain normal routing.
pub fn enter_emergency_logging(sink: Sink) {
    EMERGENCY_SINK.set(Some(sink));
}

struct Logger;
static LOGGER: Logger = Logger;
static INITIALIZE: Once = Once::new();

impl ::log::Log for Logger {
    fn enabled(&self, _: &::log::Metadata<'_>) -> bool {
        true
    }
    fn log(&self, record: &::log::Record<'_>) {
        let level = match record.level() {
            ::log::Level::Error | ::log::Level::Warn => Level::Error,
            ::log::Level::Info => Level::Verbose,
            ::log::Level::Debug | ::log::Level::Trace => Level::Debug,
        };
        emit(level, record.args().to_string().as_bytes());
    }
    fn flush(&self) {}
}

/// Install once, before application startup. Filtering stays in the program
/// backend because even suppressed stdout messages may belong in shared memory.
pub fn initialize() {
    INITIALIZE.call_once(|| {
        ::log::set_logger(&LOGGER).expect("i333 owns the process logger");
        ::log::set_max_level(::log::LevelFilter::Trace);
    });
}

/// Install the program's sink on its event thread before calling common helpers.
/// Diagnostics from other threads use stderr instead of touching program globals.
pub fn set_sink(sink: Sink) {
    initialize();
    // Force a real access so dynamically linked TLS storage, if any, is ready
    // before entry from an asynchronous signal handler.
    EMERGENCY_SINK.with(|cell| {
        std::hint::black_box(cell);
    });
    *SINK
        .write()
        .unwrap_or_else(std::sync::PoisonError::into_inner) = Some(RegisteredSink {
        callback: sink,
        thread: std::thread::current().id(),
    });
}

/// Error diagnostics remain available during startup before a sink is installed.
pub fn emit(level: Level, message: &[u8]) {
    // Copy the override before dispatch: reentry never retains a Cell borrow.
    if let Some(sink) = EMERGENCY_SINK.get() {
        sink(level, message);
        return;
    }
    let sink = *SINK
        .read()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    if let Some(sink) = sink.filter(|sink| sink.thread == std::thread::current().id()) {
        (sink.callback)(level, message);
    } else {
        stderr_sink(level, message);
    }
}

/// Join explicit byte fragments for exceptional non-UTF-8 diagnostics. Normal
/// text records use standard log macros; this path performs no formatting.
/// All fragments are copied before dispatching a potentially reentrant sink.
pub fn emit_parts(level: Level, parts: &[&[u8]]) {
    emit(level, &parts.concat());
}

/// Explicit fallback for small tools without a custom logging backend.
pub fn stderr_sink(level: Level, message: &[u8]) {
    if level == Level::Error {
        let _ = std::io::stderr().lock().write_all(message);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;
    static MESSAGES: Mutex<Vec<(Level, Vec<u8>)>> = Mutex::new(Vec::new());
    fn record(level: Level, message: &[u8]) {
        MESSAGES.lock().unwrap().push((level, message.to_vec()));
    }

    #[test]
    fn standard_macros_and_native_bytes_dispatch_to_the_rust_sink() {
        set_sink(record);
        ::log::error!("logging-test:{} {} %\n", "text", 42);
        ::log::info!("logging-test:{}", "abc");
        emit_parts(Level::Debug, &[b"logging-test:", b"\0\xff"]);
        ::log::error!("logging-test:rust {} {:04x}\n", 42, 15);
        ::log::warn!("logging-test:warning");
        ::log::info!("logging-test:info");
        ::log::debug!("logging-test:debug");
        ::log::trace!("logging-test:trace");
        let messages = MESSAGES.lock().unwrap();
        assert!(messages.contains(&(Level::Error, b"logging-test:rust 42 000f\n".to_vec())));
        assert!(messages.contains(&(Level::Error, b"logging-test:warning".to_vec())));
        assert!(messages.contains(&(Level::Verbose, b"logging-test:info".to_vec())));
        assert!(messages.contains(&(Level::Debug, b"logging-test:debug".to_vec())));
        assert!(messages.contains(&(Level::Debug, b"logging-test:trace".to_vec())));
        assert!(messages.contains(&(Level::Error, b"logging-test:text 42 %\n".to_vec())));
        assert!(messages.contains(&(Level::Verbose, b"logging-test:abc".to_vec())));
        assert!(messages.contains(&(Level::Debug, b"logging-test:\0\xff".to_vec())));
    }
    #[test]
    fn emergency_records_bypass_a_held_sink_lock_and_keep_thread_fallback() {
        if std::env::var_os("I333_EMERGENCY_LOG_PROBE").is_none() {
            let mut child = std::process::Command::new(std::env::current_exe().unwrap())
                .args(["--exact", "logging::tests::emergency_records_bypass_a_held_sink_lock_and_keep_thread_fallback", "--nocapture"])
                .env("I333_EMERGENCY_LOG_PROBE", "1")
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .spawn().unwrap();
            let start = std::time::Instant::now();
            while child.try_wait().unwrap().is_none() {
                if start.elapsed() > std::time::Duration::from_secs(5) {
                    child.kill().unwrap();
                    let output = child.wait_with_output().unwrap();
                    panic!(
                        "emergency logging blocked on the held sink lock: {}",
                        String::from_utf8_lossy(&output.stderr)
                    );
                }
                std::thread::sleep(std::time::Duration::from_millis(10));
            }
            let output = child.wait_with_output().unwrap();
            assert!(
                output.status.success(),
                "{}\n{}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
            assert!(output
                .stderr
                .windows(b"emergency-foreign-thread\n".len())
                .any(|bytes| bytes == b"emergency-foreign-thread\n"));
            return;
        }
        static EMERGENCY_MESSAGES: Mutex<Vec<(Level, Vec<u8>)>> = Mutex::new(Vec::new());
        fn emergency(level: Level, bytes: &[u8]) {
            EMERGENCY_MESSAGES
                .lock()
                .unwrap()
                .push((level, bytes.to_vec()));
        }
        fn ordinary(_: Level, _: &[u8]) {
            panic!("the normal owner sink must be bypassed");
        }
        set_sink(ordinary); // Also primes the same const TLS key before entry.
        let locked = SINK.write().unwrap();
        enter_emergency_logging(emergency);
        fn nested_restart_record() {
            ::log::info!("nested restart record {}\n", 42);
            emit_parts(Level::Error, &[b"binary emergency: ", b"\xff\0"]);
        }
        nested_restart_record();
        drop(locked);
        std::thread::spawn(|| {
            ::log::error!("emergency-foreign-thread\n");
        })
        .join()
        .unwrap();
        assert_eq!(
            *EMERGENCY_MESSAGES.lock().unwrap(),
            [
                (Level::Verbose, b"nested restart record 42\n".to_vec()),
                (Level::Error, b"binary emergency: \xff\0".to_vec()),
            ]
        );
    }
}
