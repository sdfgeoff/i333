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

fn receive(
    stream: &UnixStream,
) -> Result<crate::ipc_recv_message::Message, crate::ipc_recv_message::ReceiveError> {
    crate::ipc_recv_message::receive_message_retry_payload(&mut &*stream)
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
            assert!(crate::boolstr::is_enabled(value.to_bytes()), "{word}");
        }
    }
    for word in [
        "", "0", "no", "false", "disabled", " yes", "true ", "y", "2",
    ] {
        let value = CString::new(word).unwrap();
        assert!(!crate::boolstr::is_enabled(value.to_bytes()), "{word:?}");
    }
}

#[test]
fn sender_emits_native_endian_header_and_binary_payload() {
    let (mut sender, mut reader) = socket_pair();
    let body = b"\0binary\xff\0";
    let kind = 0x8123_4567;
    crate::ipc_send_message::send_message_retry_would_block(&mut sender, kind, body).unwrap();
    sender.shutdown(Shutdown::Write).unwrap();
    let mut actual = Vec::new();
    reader.read_to_end(&mut actual).unwrap();
    assert_eq!(actual, frame(kind, body));
}

#[test]
fn zero_length_message_has_a_header_and_no_payload() {
    let (mut sender, mut reader) = socket_pair();
    crate::ipc_send_message::send_message_retry_would_block(&mut sender, 12, &[]).unwrap();
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
    let received = receive(&reader).unwrap();
    producer.join().unwrap();
    assert_eq!(received.message_type, 0x8000_0007);
    assert_eq!(received.payload, expected);
}

#[test]
fn receiver_keeps_back_to_back_frames_separate() {
    let (reader, mut writer) = socket_pair();
    let mut wire = frame(4, b"first");
    wire.extend_from_slice(&frame(u32::MAX, b"\0\xffnext"));
    writer.write_all(&wire).unwrap();
    let first = receive(&reader).unwrap();
    let second = receive(&reader).unwrap();
    assert_eq!(
        (first.message_type, first.payload.as_slice()),
        (4, b"first".as_slice())
    );
    // The framing layer must preserve unknown/future message and event types.
    assert_eq!(
        (second.message_type, second.payload.as_slice()),
        (u32::MAX, b"\0\xffnext".as_slice())
    );
}

#[test]
fn receiver_accepts_empty_payload() {
    let (reader, mut writer) = socket_pair();
    writer.write_all(&frame(7, &[])).unwrap();
    let received = receive(&reader).unwrap();
    assert_eq!(received.message_type, 7);
    assert_eq!(received.payload, b"");
}

#[test]
fn clean_eof_differs_from_truncated_header() {
    use crate::ipc_recv_message::ReceiveError;
    for prefix in [b"".as_slice(), b"i3-ip".as_slice()] {
        let (reader, mut writer) = socket_pair();
        writer.write_all(prefix).unwrap();
        writer.shutdown(Shutdown::Write).unwrap();
        match receive(&reader).unwrap_err() {
            ReceiveError::Eof => assert!(prefix.is_empty()),
            ReceiveError::TruncatedHeader { received } => assert_eq!(received, prefix.len()),
            error => panic!("wrong error: {error}"),
        }
    }
}

#[test]
fn invalid_magic_is_rejected_before_payload_allocation() {
    let (reader, mut writer) = socket_pair();
    let mut wire = frame(0, b"ignored");
    wire[..6].copy_from_slice(b"badipc");
    writer.write_all(&wire).unwrap();
    assert!(
        matches!(receive(&reader), Err(crate::ipc_recv_message::ReceiveError::InvalidMagic(magic)) if magic == *b"badipc")
    );
}

#[test]
fn truncated_payload_reports_progress_without_exporting_storage() {
    let (reader, mut writer) = socket_pair();
    let wire = frame(5, b"four");
    writer.write_all(&wire[..wire.len() - 1]).unwrap();
    writer.shutdown(Shutdown::Write).unwrap();
    assert!(matches!(
        receive(&reader),
        Err(crate::ipc_recv_message::ReceiveError::TruncatedPayload {
            received: 3,
            expected: 4
        })
    ));
}

#[test]
fn write_failure_preserves_os_error() {
    struct Broken;
    impl Write for Broken {
        fn write(&mut self, _: &[u8]) -> std::io::Result<usize> {
            Err(std::io::Error::from_raw_os_error(libc::EBADF))
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    let error =
        crate::ipc_send_message::send_message_retry_would_block(&mut Broken, 0, b"x").unwrap_err();
    assert_eq!(error.raw_os_error(), Some(libc::EBADF));
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
        assert_eq!(crate::path_exists::exists(path), expected);
    }
}

#[test]
fn placeholders_expand_repeated_names_and_preserve_unknown_tokens() {
    let result = crate::format_placeholders::expand_cstr(
        c"%title:%x:%title:%unknown",
        &[(c"%title", c"hello"), (c"%x", c"")],
    );
    assert_eq!(result.to_bytes(), b"hello::hello:%unknown");
    assert_eq!(
        crate::format_placeholders::expand_cstr(c"%unknown", &[]).as_c_str(),
        c"%unknown"
    );
}

#[test]
fn safe_boolean_api_handles_strings_and_arbitrary_bytes() {
    assert!(crate::boolstr::is_enabled("TrUe"));
    assert!(crate::boolstr::is_enabled(b"ACTIVE"));
    assert!(!crate::boolstr::is_enabled(" on"));
    assert!(!crate::boolstr::is_enabled(b"true\xff"));
}

#[test]
fn safe_sender_handles_interrupted_and_short_writes() {
    struct ShortWriter {
        first: bool,
        bytes: Vec<u8>,
    }
    impl Write for ShortWriter {
        fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
            if self.first {
                self.first = false;
                return Err(std::io::ErrorKind::Interrupted.into());
            }
            let count = bytes.len().min(2);
            self.bytes.extend_from_slice(&bytes[..count]);
            Ok(count)
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    let mut writer = ShortWriter {
        first: true,
        bytes: Vec::new(),
    };
    crate::ipc_send_message::send_message(&mut writer, 0x8000_0003, b"\0\xffwindow").unwrap();
    assert_eq!(writer.bytes, frame(0x8000_0003, b"\0\xffwindow"));
}

#[test]
fn safe_sender_propagates_write_failures() {
    struct Broken;
    impl Write for Broken {
        fn write(&mut self, _: &[u8]) -> std::io::Result<usize> {
            Err(std::io::Error::from_raw_os_error(libc::EPIPE))
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    let error = crate::ipc_send_message::send_message(&mut Broken, 0, b"command").unwrap_err();
    assert_eq!(error.raw_os_error(), Some(libc::EPIPE));
}

#[test]
fn owned_message_decoder_consumes_one_frame_at_a_time() {
    use crate::ipc_recv_message::receive_message;
    let mut bytes = frame(7, b"\0\xffversion");
    bytes.extend_from_slice(&frame(u32::MAX, &[]));
    let mut reader = std::io::Cursor::new(bytes);
    let first = receive_message(&mut reader).unwrap();
    assert_eq!(first.message_type, 7);
    assert_eq!(first.payload, b"\0\xffversion");
    assert_eq!(reader.position(), (14 + first.payload.len()) as u64);
    let second = receive_message(&mut reader).unwrap();
    assert_eq!(second.message_type, u32::MAX);
    assert!(second.payload.is_empty());
}

#[test]
fn safe_decoder_exposes_distinct_eof_and_protocol_errors() {
    use crate::ipc_recv_message::{receive_message, ReceiveError};
    assert!(matches!(
        receive_message(&mut b"".as_slice()),
        Err(ReceiveError::Eof)
    ));
    assert!(matches!(
        receive_message(&mut b"i3-ip".as_slice()),
        Err(ReceiveError::TruncatedHeader { received: 5 })
    ));
    let mut truncated = frame(0, b"four");
    truncated.pop();
    assert!(matches!(
        receive_message(&mut truncated.as_slice()),
        Err(ReceiveError::TruncatedPayload {
            received: 3,
            expected: 4
        })
    ));
    let mut bad_magic = frame(0, b"");
    bad_magic[..6].copy_from_slice(b"b\xff\0abc");
    let error = receive_message(&mut bad_magic.as_slice()).unwrap_err();
    assert!(matches!(
        error,
        ReceiveError::InvalidMagic([b'b', 255, 0, b'a', b'b', b'c'])
    ));
    assert_eq!(
        error.diagnostic_bytes(),
        b"IPC: invalid magic in header, got \"b\xff\", want \"i3-ipc\""
    );
}

#[test]
fn safe_decoder_preserves_os_errors_and_retries_interrupted_payload_reads() {
    use crate::ipc_recv_message::{receive_message, ReceiveError};
    struct Failure;
    impl Read for Failure {
        fn read(&mut self, _: &mut [u8]) -> std::io::Result<usize> {
            Err(std::io::Error::from_raw_os_error(libc::EACCES))
        }
    }
    match receive_message(&mut Failure).unwrap_err() {
        ReceiveError::Io(error) => assert_eq!(error.raw_os_error(), Some(libc::EACCES)),
        error => panic!("wrong error: {error}"),
    }
    struct InterruptedPayload {
        cursor: std::io::Cursor<Vec<u8>>,
        interrupted: bool,
    }
    impl Read for InterruptedPayload {
        fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
            if self.cursor.position() == 14 && !self.interrupted {
                self.interrupted = true;
                return Err(std::io::ErrorKind::Interrupted.into());
            }
            self.cursor.read(buffer)
        }
    }
    let mut reader = InterruptedPayload {
        cursor: std::io::Cursor::new(frame(4, b"tree")),
        interrupted: false,
    };
    assert_eq!(receive_message(&mut reader).unwrap().payload, b"tree");
    assert!(reader.interrupted);
}

#[test]
fn safe_nonblocking_api_is_reversible_and_preserves_descriptor_flags() {
    let (mut stream, _peer) = socket_pair();
    // SAFETY: F_GETFD only queries integer descriptor flags on a live socket.
    let before = unsafe { libc::fcntl(stream.as_raw_fd(), libc::F_GETFD) };
    crate::nonblock::set_nonblocking(&stream, true).unwrap();
    crate::nonblock::set_nonblocking(&stream, true).unwrap();
    let error = stream.read(&mut [0]).unwrap_err();
    assert_eq!(error.kind(), std::io::ErrorKind::WouldBlock);
    // SAFETY: The socket is still alive and the query does not borrow memory.
    assert_eq!(
        unsafe { libc::fcntl(stream.as_raw_fd(), libc::F_GETFD) },
        before
    );
    crate::nonblock::set_nonblocking(&stream, false).unwrap();
    // SAFETY: F_GETFL only queries integer flags on the live socket.
    assert_eq!(
        unsafe { libc::fcntl(stream.as_raw_fd(), libc::F_GETFL) } & libc::O_NONBLOCK,
        0
    );
}

#[test]
fn safe_placeholder_apis_preserve_unicode_binary_bytes_and_matching_order() {
    use crate::format_placeholders::{expand, expand_cstr};
    assert_eq!(
        expand("%title/%title/%unknown", &[("%title", "λ"), ("%ti", "bad")]),
        "λ/λ/%unknown"
    );
    assert_eq!(
        expand("%title", &[("%ti", "prefix"), ("%title", "whole")]),
        "prefixtle"
    );
    assert_eq!(
        expand("a%title", &[("", "bad"), ("a", "bad"), ("%title", "")]),
        "a"
    );
    let binary = CString::new(b"\xffvalue".to_vec()).unwrap();
    let format = CString::new(b"before:%x:\xfe".to_vec()).unwrap();
    let expanded = expand_cstr(&format, &[(c"%x", binary.as_c_str())]);
    assert_eq!(expanded.to_bytes(), b"before:\xffvalue:\xfe");
}

#[test]
fn executable_path_reports_current_program_even_with_unrelated_argv0() {
    let path = crate::get_exe_path::executable_path("not-an-installed-i333-executable").unwrap();
    assert!(path.is_absolute());
    assert!(std::fs::metadata(path).unwrap().is_file());
    let invalid = std::ffi::OsStr::from_bytes(b"invalid\0name");
    assert_eq!(
        crate::get_exe_path::executable_path(invalid)
            .unwrap_err()
            .kind(),
        std::io::ErrorKind::InvalidInput
    );
}

#[test]
fn safe_socket_connection_owns_its_descriptor_and_explicit_path_wins() {
    use crate::ipc_connect::{connect, connect_path, socket_path};
    use crate::ipc_recv_message::receive_message;
    use crate::ipc_send_message::send_message;
    let temp = TempDir::new();
    let path = temp.0.join("ipc");
    let listener = std::os::unix::net::UnixListener::bind(&path).unwrap();
    assert_eq!(socket_path(Some(&path)).unwrap(), path);
    for explicit in [true, false] {
        let mut client = if explicit {
            connect_path(&path)
        } else {
            connect(Some(&path))
        }
        .unwrap();
        let (mut server, _) = listener.accept().unwrap();
        // SAFETY: F_GETFD queries a descriptor owned by a live UnixStream.
        assert_ne!(
            unsafe { libc::fcntl(client.as_raw_fd(), libc::F_GETFD) } & libc::FD_CLOEXEC,
            0
        );
        send_message(&mut client, 10, b"tick").unwrap();
        let message = receive_message(&mut server).unwrap();
        assert_eq!(message.message_type, 10);
        assert_eq!(message.payload, b"tick");
    }
}

#[test]
fn ucs2_is_big_endian_and_never_combines_surrogates() {
    use crate::ucs2_conversion::{
        decode_ucs2, encode_ucs2, ConversionIssue, EncodingPolicy, Ucs2Char,
    };
    let encoded = encode_ucs2("Aλ\u{fffe}\u{ffff}".as_bytes(), EncodingPolicy::Ignore);
    assert_eq!(encoded.issue, None);
    assert_eq!(
        encoded.glyphs,
        [0x0041, 0x03bb, 0xfffe, 0xffff].map(Ucs2Char::from)
    );
    assert_eq!(
        encoded.glyphs[1],
        Ucs2Char {
            byte1: 3,
            byte2: 0xbb
        }
    );
    assert_eq!(decode_ucs2(&encoded.glyphs).unwrap(), "Aλ\u{fffe}\u{ffff}");
    assert_eq!(
        decode_ucs2(&[0xd83d.into(), 0xde00.into()]),
        Err(ConversionIssue::InvalidSequence)
    );
    assert_eq!(
        decode_ucs2(&[0x0041.into(), 0xdfff.into()]),
        Err(ConversionIssue::InvalidSequence)
    );
    assert_eq!(
        decode_ucs2(&[0x0041.into(), 0.into(), 0x0042.into()]).unwrap(),
        "A\0B"
    );
}

#[test]
fn ucs2_ignore_and_strict_policies_preserve_their_distinct_partial_results() {
    use crate::ucs2_conversion::{encode_ucs2, ConversionIssue, EncodingPolicy, Ucs2Char};
    let input = b"A\xf0\x9f\x98\x80\xffB";
    let ignored = encode_ucs2(input, EncodingPolicy::Ignore);
    assert_eq!(ignored.glyphs, [0x0041, 0x0042].map(Ucs2Char::from));
    assert_eq!(ignored.issue, Some(ConversionIssue::InvalidSequence));
    let strict = encode_ucs2(input, EncodingPolicy::Strict);
    assert_eq!(strict.glyphs, [0x0041].map(Ucs2Char::from));
    assert_eq!(strict.issue, Some(ConversionIssue::InvalidSequence));
    let incomplete = encode_ucs2(b"A\xffB\xe2\x82", EncodingPolicy::Ignore);
    assert_eq!(incomplete.glyphs, [0x0041, 0x0042].map(Ucs2Char::from));
    assert_eq!(incomplete.issue, Some(ConversionIssue::IncompleteSequence));
}

#[test]
fn legacy_ucs2_buffers_retain_malloc_ownership_and_empty_null_result() {
    use crate::ucs2_conversion::{convert_ucs2_to_utf8, convert_utf8_to_ucs2};
    let mut length = 999;
    let mut input = c"Aλ".to_bytes_with_nul().to_vec();
    // SAFETY: Input is writable and NUL-terminated; length is a live output.
    unsafe {
        let glyphs = convert_utf8_to_ucs2(input.as_mut_ptr().cast(), &mut length);
        assert_eq!(length, 2);
        assert_eq!(
            std::slice::from_raw_parts(glyphs, length),
            &[0x0041.into(), 0x03bb.into()]
        );
        let output = convert_ucs2_to_utf8(glyphs, length);
        let decoded = CStr::from_ptr(output).to_bytes().to_vec();
        libc::free(glyphs.cast());
        libc::free(output.cast());
        assert_eq!(decoded, "Aλ".as_bytes());
        assert!(convert_utf8_to_ucs2(c"".as_ptr().cast_mut(), &mut length).is_null());
        assert_eq!(length, 0);
        let mut nul_glyphs = [0x0041.into(), 0.into(), 0x0042.into()];
        let output = convert_ucs2_to_utf8(nul_glyphs.as_mut_ptr(), 3);
        assert_eq!(
            std::slice::from_raw_parts(output.cast::<u8>(), 4),
            b"A\0B\0"
        );
        libc::free(output.cast());
    }
}

#[test]
fn owned_text_replaces_each_invalid_byte_and_embedded_nul() {
    use crate::string::I3String;
    let text = I3String::from_bytes(b"A\xe2\x82\0B\xff");
    assert_eq!(text.as_str().unwrap(), "A��\u{fffd}B�");
    assert_eq!(text.byte_len().unwrap(), text.as_str().unwrap().len());
    let pointer = text.as_cstr().unwrap().as_ptr();
    assert_eq!(text.glyph_count(), 6);
    text.set_markup(true);
    assert!(text.is_markup());
    assert_eq!(text.as_cstr().unwrap().as_ptr(), pointer);
    let astral = I3String::new("A😀B");
    assert_eq!(astral.as_str().unwrap(), "A😀B");
    assert_eq!(astral.as_ucs2(), &[0x0041.into(), 0x0042.into()]);
}

#[test]
fn markup_escaping_matches_entities_and_xml_control_rules() {
    let text = crate::string::I3String::new("<>&\"'\u{1}\t\n\r\u{7f}\u{80}\u{85}\u{86}\u{9f}");
    text.set_markup(true);
    let escaped = text.escaped_markup().unwrap();
    assert_eq!(
        escaped.as_str().unwrap(),
        "&lt;&gt;&amp;&quot;&apos;&#x1;\t\n\r&#x7f;&#x80;\u{85}&#x86;&#x9f;"
    );
    assert!(!escaped.is_markup());
    assert!(text.is_markup());
}

#[test]
fn owned_text_from_ucs2_retains_glyphs_when_c_view_stops_at_nul() {
    let text = crate::string::I3String::from_ucs2(&[0x0041.into(), 0.into(), 0x0042.into()]);
    assert_eq!(text.as_str().unwrap(), "A");
    assert_eq!(text.glyph_count(), 3);
    assert_eq!(text.as_ucs2(), &[0x0041.into(), 0.into(), 0x0042.into()]);
    let invalid = crate::string::I3String::from_ucs2(&[0xd800.into()]);
    assert!(invalid.as_str().is_err());
}

#[test]
fn legacy_text_keeps_explicit_length_quirk_and_independent_copies() {
    use crate::string::*;
    struct Owned(*mut I3String);
    impl Drop for Owned {
        fn drop(&mut self) {
            // SAFETY: Each Owned holds exactly one box from the legacy constructor.
            unsafe { i3string_free(self.0) }
        }
    }
    let input = [b'A', 255];
    // SAFETY: The constructor receives the complete readable byte span; all
    // following calls borrow that live box, and Owned releases each result once.
    unsafe {
        let original = Owned(i3string_from_utf8_with_length(
            input.as_ptr().cast(),
            input.len() as isize,
        ));
        assert_eq!(
            CStr::from_ptr(i3string_as_utf8(original.0)).to_bytes(),
            "A�".as_bytes()
        );
        assert_eq!(i3string_get_num_bytes(original.0), 2);
        i3string_set_markup(original.0, true);
        let copy = Owned(i3string_copy(original.0));
        assert_eq!(i3string_get_num_bytes(copy.0), 4);
        assert!(i3string_is_markup(copy.0));
        assert_ne!(i3string_as_utf8(copy.0), i3string_as_utf8(original.0));
        i3string_set_markup(original.0, false);
        assert!(i3string_is_markup(copy.0));
    }
}

#[test]
fn recursive_directories_preserve_existing_permissions_and_reject_files() {
    use std::os::unix::fs::PermissionsExt;
    let temp = TempDir::new();
    let parent = temp.0.join("existing");
    std::fs::create_dir(&parent).unwrap();
    std::fs::set_permissions(&parent, std::fs::Permissions::from_mode(0o750)).unwrap();
    let nested = parent.join("new/deep");
    crate::mkdirp::create_dir_all(&nested, 0o700).unwrap();
    assert!(nested.is_dir());
    assert_eq!(
        std::fs::metadata(&parent).unwrap().permissions().mode() & 0o777,
        0o750
    );
    assert_eq!(
        std::fs::metadata(&nested).unwrap().permissions().mode() & 0o077,
        0
    );
    let file = temp.0.join("file");
    std::fs::write(&file, b"ordinary").unwrap();
    assert!(crate::mkdirp::create_dir_all(file, 0o700).is_err());
    assert!(crate::mkdirp::create_dir_all("", 0o700).is_err());
}

#[test]
fn recursive_directory_failure_diagnostics_retain_native_bytes_and_errno() {
    const CHILD_PATH: &str = "I333_MKDIR_DIAGNOSTIC_CHILD";
    if let Some(path) = std::env::var_os(CHILD_PATH) {
        let path = PathBuf::from(path);
        let error = crate::mkdirp::create_dir_all(&path, 0o700).unwrap_err();
        assert_eq!(error.raw_os_error(), Some(libc::EEXIST));
        let error = crate::mkdirp::report_create_error(&path, error);
        assert_eq!(error.raw_os_error(), Some(libc::EEXIST));
        let nested = path.join("nested");
        let error = crate::mkdirp::create_dir_all(&nested, 0o700).unwrap_err();
        let error = crate::mkdirp::report_create_error(&nested, error);
        assert_eq!(error.raw_os_error(), Some(libc::ENOTDIR));
        // Reproduce the disappearance between failed mkdir and follow-up stat.
        std::fs::remove_file(&path).unwrap();
        let error = crate::mkdirp::report_create_error(
            &path,
            std::io::Error::from_raw_os_error(libc::EEXIST),
        );
        assert_eq!(error.raw_os_error(), Some(libc::ENOENT));
        return;
    }
    let temp = TempDir::new();
    let file = temp.0.join(std::ffi::OsStr::from_bytes(b"file-\xff"));
    std::fs::write(&file, b"ordinary file").unwrap();
    let child = std::process::Command::new(std::env::current_exe().unwrap())
        .args([
            "--exact",
            "tests::recursive_directory_failure_diagnostics_retain_native_bytes_and_errno",
            "--nocapture",
        ])
        .env(CHILD_PATH, &file)
        .output()
        .unwrap();
    assert!(
        child.status.success(),
        "{}",
        String::from_utf8_lossy(&child.stderr)
    );
    let mut expected = Vec::new();
    for (operation, path, code) in [
        ("mkdir", file.clone(), libc::ENOTDIR),
        ("mkdir", file.join("nested"), libc::ENOTDIR),
        ("stat", file, libc::ENOENT),
    ] {
        expected.extend_from_slice(b"[libi3] ERROR: ");
        expected.extend_from_slice(operation.as_bytes());
        expected.push(b'(');
        expected.extend_from_slice(path.as_os_str().as_bytes());
        expected.extend_from_slice(b") failed: ");
        // SAFETY: strerror returns a live native locale description.
        expected.extend_from_slice(unsafe { CStr::from_ptr(libc::strerror(code)) }.to_bytes());
        expected.push(b'\n');
    }
    assert_eq!(child.stderr, expected);
}

#[test]
fn runtime_paths_are_explicit_owned_and_private() {
    use std::os::unix::fs::PermissionsExt;
    let temp = TempDir::new();
    let runtime = crate::get_process_filename::RuntimePaths::new(Some(&temp.0)).unwrap();
    assert_eq!(runtime.directory(), temp.0.join("i3"));
    assert_eq!(
        std::fs::metadata(runtime.directory())
            .unwrap()
            .permissions()
            .mode()
            & 0o077,
        0
    );
    let expected = runtime
        .directory()
        .join(format!("ipc-socket.{}", std::process::id()));
    assert_eq!(runtime.filename("ipc-socket"), expected);
    assert_eq!(runtime.clone().filename("ipc-socket"), expected);
}

#[test]
fn configuration_search_preserves_user_then_system_order() {
    use crate::get_config_path::{find_config_path, ConfigSearch};
    let temp = TempDir::new();
    let search = ConfigSearch {
        config_home: temp.0.join("home"),
        legacy_config: temp.0.join("legacy/config"),
        system_dirs: vec![temp.0.join("first-system"), temp.0.join("second-system")],
        system_config: temp.0.join("fallback"),
    };
    for directory in [
        &search.config_home,
        &search.system_dirs[0],
        &search.system_dirs[1],
    ] {
        std::fs::create_dir_all(directory.join("i3")).unwrap();
    }
    std::fs::create_dir_all(search.legacy_config.parent().unwrap()).unwrap();
    let candidates = [
        search.config_home.join("i3/config"),
        search.legacy_config.clone(),
        search.system_dirs[0].join("i3/config"),
        search.system_dirs[1].join("i3/config"),
        search.system_config.clone(),
    ];
    for candidate in &candidates {
        std::fs::write(candidate, b"config").unwrap();
    }
    for (index, candidate) in candidates.iter().enumerate() {
        assert_eq!(search.find(true).unwrap().as_ref(), Some(candidate));
        if index >= 2 {
            assert_eq!(search.find(false).unwrap(), None);
        }
        std::fs::remove_file(candidate).unwrap();
    }
    assert_eq!(search.find(true).unwrap(), None);
    let missing_override = temp.0.join("missing-override");
    assert_eq!(
        find_config_path(Some(&missing_override), false).unwrap(),
        Some(missing_override)
    );
}

#[test]
fn legacy_configuration_override_owns_its_remembered_path() {
    let expected = b"./unresolved-override-config";
    // SAFETY: CString remains live for the first call, then is intentionally
    // dropped. The second call must use the adapter's owned remembered path.
    unsafe {
        let first = {
            let temporary = CString::new(expected.as_slice()).unwrap();
            crate::get_config_path::get_config_path(temporary.as_ptr(), false)
        };
        let actual = CStr::from_ptr(first).to_bytes().to_vec();
        libc::free(first.cast());
        assert_eq!(actual, expected);
        let second = crate::get_config_path::get_config_path(std::ptr::null(), false);
        let actual = CStr::from_ptr(second).to_bytes().to_vec();
        libc::free(second.cast());
        assert_eq!(actual, expected);
    }
}

#[test]
fn tilde_resolution_preserves_absolute_paths_and_limits_globs_to_first_component() {
    use crate::resolve_tilde::resolve;
    let absolute = std::env::temp_dir().join("i333-no-match-*/tail");
    assert_eq!(resolve(&absolute).unwrap(), absolute);
    let literal = PathBuf::from("literal//untouched?[tail");
    assert_eq!(resolve(&literal).unwrap(), literal);
    assert_eq!(
        resolve("~i333-definitely-no-such-user/tail").unwrap(),
        PathBuf::from("~i333-definitely-no-such-user/tail")
    );
    let invalid = std::ffi::OsStr::from_bytes(b"invalid\0path");
    assert_eq!(
        resolve(invalid).unwrap_err().kind(),
        std::io::ErrorKind::InvalidInput
    );
    // Cargo sets the test working directory to the package root. This tests an
    // actual first-component glob without changing cwd or writing user files.
    assert!(PathBuf::from("Cargo.toml").is_file());
    assert_eq!(
        resolve("Cargo.tom?/tail*").unwrap(),
        PathBuf::from("Cargo.toml/tail*")
    );
}

#[test]
fn owned_listener_replaces_stale_sockets_but_rejects_active_listeners() {
    const CHILD: &str = "I333_STALE_LISTENER_CHILD";
    if std::env::var_os(CHILD).is_none() {
        // Parallel tests spawn processes. A fork can briefly inherit the live
        // listener until exec, making its parent's dropped socket still active.
        // Create this fixture only after exec, with no sibling tests in its process.
        let mut child = std::process::Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "tests::owned_listener_replaces_stale_sockets_but_rejects_active_listeners",
                "--color=never",
            ])
            .env(CHILD, "1")
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .unwrap();
        let start = std::time::Instant::now();
        while child.try_wait().unwrap().is_none() {
            if start.elapsed() > Duration::from_secs(5) {
                let _ = child.kill();
                let output = child.wait_with_output().unwrap();
                panic!(
                    "isolated listener regression timed out: {}",
                    String::from_utf8_lossy(&output.stderr)
                );
            }
            thread::sleep(Duration::from_millis(10));
        }
        let output = child.wait_with_output().unwrap();
        assert!(
            output.status.success(),
            "{}\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(
            String::from_utf8_lossy(&output.stdout).contains("1 passed; 0 failed"),
            "the exact-filter child must execute the listener regression"
        );
        return;
    }
    use crate::create_socket::bind;
    let temp = TempDir::new();
    let path = temp.0.join("nested/ipc");
    let bound = bind(&path).unwrap();
    assert_eq!(bound.path, path);
    assert_eq!(
        bound.listener.accept().unwrap_err().kind(),
        std::io::ErrorKind::WouldBlock
    );
    assert_eq!(bind(&path).unwrap_err().raw_os_error(), Some(libc::EEXIST));
    drop(bound);
    assert!(path.exists());
    let replacement = bind(&path).unwrap();
    let _client = UnixStream::connect(&path).unwrap();
    let _accepted = replacement.listener.accept().unwrap();
}

#[test]
fn compatibility_receiver_retries_only_payload_would_block_without_losing_progress() {
    use crate::ipc_recv_message::{receive_message, receive_message_retry_payload, ReceiveError};
    struct Paused {
        bytes: std::io::Cursor<Vec<u8>>,
        failures: std::collections::VecDeque<(u64, std::io::ErrorKind)>,
    }
    impl Read for Paused {
        fn read(&mut self, output: &mut [u8]) -> std::io::Result<usize> {
            if self
                .failures
                .front()
                .is_some_and(|(position, _)| *position == self.bytes.position())
            {
                return Err(self.failures.pop_front().unwrap().1.into());
            }
            let count = output.len().min(2);
            self.bytes.read(&mut output[..count])
        }
    }
    let make = |failures: Vec<_>| Paused {
        bytes: std::io::Cursor::new(frame(0xffff_ffff, b"a\0b\xff")),
        failures: failures.into(),
    };
    // A partially consumed header still returns OS errors immediately.
    for kind in [
        std::io::ErrorKind::Interrupted,
        std::io::ErrorKind::WouldBlock,
    ] {
        let mut reader = make(vec![(2, kind)]);
        assert!(
            matches!(receive_message_retry_payload(&mut reader), Err(ReceiveError::Io(error)) if error.kind() == kind)
        );
        assert_eq!(reader.bytes.position(), 2);
    }
    let mut reader = make(vec![
        (14, std::io::ErrorKind::WouldBlock),
        (16, std::io::ErrorKind::Interrupted),
        (16, std::io::ErrorKind::WouldBlock),
    ]);
    let message = receive_message_retry_payload(&mut reader).unwrap();
    assert_eq!(message.message_type, u32::MAX);
    assert_eq!(message.payload, b"a\0b\xff");
    // The modern default retains its nonblocking error semantics.
    let mut reader = make(vec![(16, std::io::ErrorKind::WouldBlock)]);
    assert!(
        matches!(receive_message(&mut reader), Err(ReceiveError::Io(error)) if error.kind() == std::io::ErrorKind::WouldBlock)
    );
    assert_eq!(reader.bytes.position(), 16);
}

#[test]
fn compatibility_sender_retries_would_block_without_duplicating_partial_bytes() {
    use std::collections::VecDeque;
    struct Writer {
        steps: VecDeque<Result<usize, std::io::ErrorKind>>,
        bytes: Vec<u8>,
    }
    impl Write for Writer {
        fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
            let count = self
                .steps
                .pop_front()
                .unwrap_or(Ok(bytes.len()))
                .map_err(std::io::Error::from)?;
            let count = count.min(bytes.len());
            self.bytes.extend_from_slice(&bytes[..count]);
            Ok(count)
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    let mut writer = Writer {
        steps: [
            Err(std::io::ErrorKind::WouldBlock),
            Ok(2),
            Err(std::io::ErrorKind::Interrupted),
            Err(std::io::ErrorKind::WouldBlock),
            Ok(12),
            Ok(1),
            Err(std::io::ErrorKind::WouldBlock),
        ]
        .into(),
        bytes: Vec::new(),
    };
    crate::ipc_send_message::send_message_retry_would_block(&mut writer, u32::MAX, b"\0\xffbody")
        .unwrap();
    assert_eq!(writer.bytes, frame(u32::MAX, b"\0\xffbody"));
    let mut blocked = Writer {
        steps: [Err(std::io::ErrorKind::WouldBlock)].into(),
        bytes: Vec::new(),
    };
    assert_eq!(
        crate::ipc_send_message::send_message(&mut blocked, 0, b"x")
            .unwrap_err()
            .kind(),
        std::io::ErrorKind::WouldBlock
    );
    assert!(blocked.bytes.is_empty());
}
