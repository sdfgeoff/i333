//! Contracts for helpers that need neither an X server nor program globals.

use std::ffi::{CStr, CString};
use std::io::{Read, Write};
use std::net::Shutdown;
use std::os::fd::AsRawFd;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::symlink;
use std::os::unix::net::UnixStream;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

// Programs supply these callbacks in production. Decoder tests assert returned
// errors rather than diagnostic formatting. Do not replace err(), exit(), or X
// functions: an unexpected fatal error must still fail the test process.
#[no_mangle]
unsafe extern "C" fn errorlog(_format: *const libc::c_char, _arguments: ...) {}
#[no_mangle]
unsafe extern "C" fn debuglog(_format: *const libc::c_char, _arguments: ...) {}

struct Received {
    status: libc::c_int,
    kind: u32,
    length: u32,
    data: *mut u8,
}

impl Received {
    fn read(stream: &UnixStream) -> Self {
        let mut received = Self {
            status: 99,
            kind: 0x1122_3344,
            length: 0x5566_7788,
            data: std::ptr::null_mut(),
        };
        // SAFETY: Each output points to live writable storage. The file
        // descriptor is borrowed from a live UnixStream throughout this call.
        received.status = unsafe {
            crate::ipc_recv_message::ipc_recv_message(
                stream.as_raw_fd(),
                &mut received.kind,
                &mut received.length,
                &mut received.data,
            )
        };
        received
    }

    fn body(&self) -> &[u8] {
        assert_eq!(self.status, 0, "only a complete payload is initialized");
        if self.length == 0 {
            return &[];
        }
        assert!(!self.data.is_null());
        // SAFETY: Success means all length bytes were initialized by read().
        // The buffer remains allocated until this Received is dropped.
        unsafe { std::slice::from_raw_parts(self.data, self.length as usize) }
    }
}

impl Drop for Received {
    fn drop(&mut self) {
        // SAFETY: The IPC helper returns malloc-owned storage, including on
        // payload read errors. free(null) also covers failures before allocation.
        unsafe { libc::free(self.data.cast()) }
    }
}

fn socket_pair() -> (UnixStream, UnixStream) {
    let (a, b) = UnixStream::pair().unwrap();
    for stream in [&a, &b] {
        stream
            .set_read_timeout(Some(Duration::from_secs(2)))
            .unwrap();
        stream
            .set_write_timeout(Some(Duration::from_secs(2)))
            .unwrap();
    }
    (a, b)
}

// The public protocol is six magic bytes, native-endian u32 length and type,
// then exactly length bytes. Construct fixtures independently of the sender.
fn frame(kind: u32, payload: &[u8]) -> Vec<u8> {
    let mut wire = b"i3-ipc".to_vec();
    wire.extend_from_slice(&(payload.len() as u32).to_ne_bytes());
    wire.extend_from_slice(&kind.to_ne_bytes());
    wire.extend_from_slice(payload);
    wire
}

#[test]
fn enabled_words_are_case_insensitive_without_trimming() {
    for word in ["1", "yes", "true", "on", "enable", "active"] {
        for variant in [word.to_owned(), word.to_uppercase()] {
            let value = CString::new(variant).unwrap();
            // SAFETY: CString supplies a valid NUL-terminated borrowed string.
            assert!(unsafe { crate::boolstr::boolstr(value.as_ptr()) }, "{word}");
        }
    }
    for word in [
        "", "0", "no", "false", "disabled", " yes", "true ", "y", "2",
    ] {
        let value = CString::new(word).unwrap();
        // SAFETY: CString remains alive through the call.
        assert!(
            !unsafe { crate::boolstr::boolstr(value.as_ptr()) },
            "{word:?}"
        );
    }
}

#[test]
fn sender_emits_native_endian_header_and_binary_payload() {
    let (sender, mut reader) = socket_pair();
    let body = b"\0binary\xff\0";
    let kind = 0x8123_4567;
    // SAFETY: body is readable for its supplied length; sender is a live socket.
    let result = unsafe {
        crate::ipc_send_message::ipc_send_message(
            sender.as_raw_fd(),
            body.len() as u32,
            kind,
            body.as_ptr(),
        )
    };
    assert_eq!(result, 0);
    sender.shutdown(Shutdown::Write).unwrap();
    let mut actual = Vec::new();
    reader.read_to_end(&mut actual).unwrap();
    assert_eq!(actual, frame(kind, body));
}

#[test]
fn zero_length_message_has_a_header_and_no_payload() {
    let (sender, mut reader) = socket_pair();
    // SAFETY: A zero-length payload never dereferences its null pointer.
    assert_eq!(
        unsafe {
            crate::ipc_send_message::ipc_send_message(sender.as_raw_fd(), 0, 12, std::ptr::null())
        },
        0
    );
    sender.shutdown(Shutdown::Write).unwrap();
    let mut wire = Vec::new();
    reader.read_to_end(&mut wire).unwrap();
    assert_eq!(wire, frame(12, &[]));
}

#[test]
fn receiver_reassembles_fragments_across_header_and_payload() {
    let (reader, mut writer) = socket_pair();
    let body = b"[\"tick\"]\0\xff";
    let expected = body.to_vec();
    let wire = frame(0x8000_0007, body);
    let producer = thread::spawn(move || {
        // One-byte writes with pauses force the reader to tolerate boundaries
        // inside the magic, both integers, and the binary payload.
        for byte in wire {
            writer.write_all(&[byte]).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });
    let received = Received::read(&reader);
    producer.join().unwrap();
    assert_eq!(received.status, 0);
    assert_eq!(received.kind, 0x8000_0007);
    assert_eq!(received.body(), expected);
}

#[test]
fn receiver_keeps_back_to_back_frames_separate() {
    let (reader, mut writer) = socket_pair();
    let mut wire = frame(4, b"first");
    wire.extend_from_slice(&frame(u32::MAX, b"\0\xffnext"));
    writer.write_all(&wire).unwrap();
    let first = Received::read(&reader);
    let second = Received::read(&reader);
    assert_eq!((first.kind, first.body()), (4, b"first".as_slice()));
    // The framing layer must preserve unknown/future message and event types.
    assert_eq!(
        (second.kind, second.body()),
        (u32::MAX, b"\0\xffnext".as_slice())
    );
}

#[test]
fn receiver_accepts_empty_payload() {
    let (reader, mut writer) = socket_pair();
    writer.write_all(&frame(7, &[])).unwrap();
    let received = Received::read(&reader);
    assert_eq!(received.kind, 7);
    assert_eq!(received.body(), b"");
}

#[test]
fn clean_eof_differs_from_truncated_header() {
    for (prefix, expected_status) in [(b"".as_slice(), -2), (b"i3-ip".as_slice(), -3)] {
        let (reader, mut writer) = socket_pair();
        writer.write_all(prefix).unwrap();
        writer.shutdown(Shutdown::Write).unwrap();
        let received = Received::read(&reader);
        assert_eq!(received.status, expected_status);
        assert_eq!(received.kind, 0x1122_3344);
        assert_eq!(received.length, 0x5566_7788);
        assert!(received.data.is_null());
    }
}

#[test]
fn invalid_magic_is_rejected_before_payload_allocation() {
    let (reader, mut writer) = socket_pair();
    let mut wire = frame(0, b"ignored");
    wire[..6].copy_from_slice(b"badipc");
    writer.write_all(&wire).unwrap();
    let received = Received::read(&reader);
    assert_eq!(received.status, -3);
    assert_eq!(received.kind, 0x1122_3344);
    assert_eq!(received.length, 0x5566_7788);
    assert!(received.data.is_null());
}

#[test]
fn truncated_payload_is_reported_and_returned_storage_can_be_freed() {
    let (reader, mut writer) = socket_pair();
    let wire = frame(5, b"four");
    writer.write_all(&wire[..wire.len() - 1]).unwrap();
    writer.shutdown(Shutdown::Write).unwrap();
    let received = Received::read(&reader);
    assert_eq!(received.status, -3);
    assert_eq!((received.kind, received.length), (5, 4));
    assert!(!received.data.is_null());
    // Drop frees the allocated buffer without reading its uninitialized suffix.
}

#[test]
fn write_failure_preserves_os_error() {
    // SAFETY: -1 is an invalid fd; the kernel rejects it without touching body.
    let status = unsafe { crate::ipc_send_message::ipc_send_message(-1, 1, 0, b"x".as_ptr()) };
    assert_eq!(status, -1);
    assert_eq!(
        std::io::Error::last_os_error().raw_os_error(),
        Some(libc::EBADF)
    );
}

struct TempDir(PathBuf);
impl TempDir {
    fn new() -> Self {
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "i333-common-{}-{stamp}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir(&path).unwrap();
        Self(path)
    }
}
impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

#[test]
fn path_exists_uses_stat_semantics_for_files_directories_and_symlinks() {
    let temp = TempDir::new();
    let file = temp.0.join("file");
    let link = temp.0.join("link");
    let absent = temp.0.join("absent");
    let broken = temp.0.join("broken");
    std::fs::write(&file, b"data").unwrap();
    symlink(&file, &link).unwrap();
    symlink(&absent, &broken).unwrap();
    for (path, expected) in [
        (&temp.0, true),
        (&file, true),
        (&link, true),
        (&absent, false),
        (&broken, false),
    ] {
        let path = CString::new(path.as_os_str().as_bytes()).unwrap();
        // SAFETY: path is a live C string; stat does not retain its pointer.
        assert_eq!(
            unsafe { crate::path_exists::path_exists(path.as_ptr()) },
            expected
        );
    }
}

#[test]
fn placeholders_expand_repeated_names_and_preserve_unknown_tokens() {
    let mut format = c"%title:%x:%title:%unknown".to_bytes_with_nul().to_vec();
    let mut placeholders = [
        crate::format_placeholders::placeholder_t {
            name: c"%title".as_ptr(),
            value: c"hello".as_ptr(),
        },
        crate::format_placeholders::placeholder_t {
            name: c"%x".as_ptr(),
            value: c"".as_ptr(),
        },
    ];
    // SAFETY: Format storage is writable; both placeholder names are nonempty
    // '%' tokens. All name/value C strings outlive the call.
    let result = unsafe {
        crate::format_placeholders::format_placeholders(
            format.as_mut_ptr().cast(),
            placeholders.as_mut_ptr(),
            placeholders.len() as i32,
        )
    };
    assert!(!result.is_null());
    // SAFETY: Success returns a NUL-terminated malloc allocation; copy its bytes
    // before releasing it so assertions do not retain references after free.
    let actual = unsafe {
        let bytes = CStr::from_ptr(result).to_bytes().to_vec();
        libc::free(result.cast());
        bytes
    };
    assert_eq!(actual, b"hello::hello:%unknown");
    // SAFETY: Null format is an explicitly supported early-return case.
    assert!(unsafe {
        crate::format_placeholders::format_placeholders(
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            0,
        )
    }
    .is_null());
}
