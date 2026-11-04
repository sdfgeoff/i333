//! Standard Rust logging shares the program's event-thread byte sink. Native
//! byte diagnostics remain available without UTF-8 conversion through `emit`.
mod compat;
pub use compat::{pointer, string_bytes};

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
    *SINK
        .write()
        .unwrap_or_else(std::sync::PoisonError::into_inner) = Some(RegisteredSink {
        callback: sink,
        thread: std::thread::current().id(),
    });
}

/// Error diagnostics remain available during startup before a sink is installed.
pub fn emit(level: Level, message: &[u8]) {
    let sink = *SINK
        .read()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    if let Some(sink) = sink.filter(|sink| sink.thread == std::thread::current().id()) {
        (sink.callback)(level, message);
    } else {
        stderr_sink(level, message);
    }
}

/// Explicit fallback for small tools without a custom logging backend.
pub fn stderr_sink(level: Level, message: &[u8]) {
    if level == Level::Error {
        let _ = std::io::stderr().lock().write_all(message);
    }
}

unsafe fn format_message(format: *const libc::c_char, arguments: core::ffi::VaList) -> Vec<u8> {
    let mut buffer = std::ptr::null_mut();
    let length = unsafe { crate::native_stdio::vasprintf(&mut buffer, format, arguments) };
    if length < 0 {
        return b"[libi3] diagnostic formatting failed\n".to_vec();
    }
    let message =
        unsafe { std::slice::from_raw_parts(buffer.cast::<u8>(), length as usize) }.to_vec();
    unsafe { libc::free(buffer.cast()) };
    message
}

/// # Safety
/// `format` is a valid C format string and all arguments match its conversions.
pub unsafe extern "C" fn errorlog(format: *const libc::c_char, mut arguments: ...) {
    let message = unsafe { format_message(format, crate::i333_va_list!(arguments)) };
    emit(Level::Error, &message);
}

/// # Safety
/// `format` is a valid C format string and all arguments match its conversions.
pub unsafe extern "C" fn verboselog(format: *const libc::c_char, mut arguments: ...) {
    let message = unsafe { format_message(format, crate::i333_va_list!(arguments)) };
    emit(Level::Verbose, &message);
}

/// # Safety
/// `format` is a valid C format string and all arguments match its conversions.
pub unsafe extern "C" fn debuglog(format: *const libc::c_char, mut arguments: ...) {
    let message = unsafe { format_message(format, crate::i333_va_list!(arguments)) };
    emit(Level::Debug, &message);
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
    fn native_formatting_dispatches_bytes_and_levels_to_rust_sink() {
        set_sink(record);
        unsafe {
            errorlog(c"logging-test:%s %d %%\n".as_ptr(), c"text".as_ptr(), 42i32);
            verboselog(c"logging-test:%.3s".as_ptr(), c"abcdef".as_ptr());
            debuglog(c"logging-test:%c%c".as_ptr(), 0i32, 255i32);
        }
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
}
