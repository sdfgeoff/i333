//! Shared diagnostics use Rust callbacks, independent of executable symbols.
//!
//! Translated helpers still call the isolated printf adapters below. Their C
//! ABI is required only by Rust's variadic syntax; no native library or dynamic
//! symbol lookup is involved. New helpers can call `emit` with borrowed bytes.
use std::io::Write;
use std::sync::RwLock;

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

/// Install the program's sink on its event thread before calling common helpers.
/// Diagnostics from other threads use stderr instead of touching program globals.
pub fn set_sink(sink: Sink) {
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
        let messages = MESSAGES.lock().unwrap();
        assert!(messages.contains(&(Level::Error, b"logging-test:text 42 %\n".to_vec())));
        assert!(messages.contains(&(Level::Verbose, b"logging-test:abc".to_vec())));
        assert!(messages.contains(&(Level::Debug, b"logging-test:\0\xff".to_vec())));
    }
}
