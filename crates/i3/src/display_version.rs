/*
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see LICENSE).
 *
 * display_version.c: displays the running i3 version for i3 --moreversion.
 */
//! Version diagnostics own their IPC reply, paths, and parser state. Only the
//! locale-sensitive timestamp formatter needs a small native boundary.

use i3_common::ipc_connect::{connect_path, root_property, socket_path};
use i3_common::ipc_recv_message::{receive_message, ReceiveError};
use i3_common::ipc_send_message::send_message;
use i3_common::logging::{emit, Level};
use i3_json::{Event, EventHandler, ParseError, Parser};
use std::cell::RefCell;
use std::ffi::{OsStr, OsString};
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Read, Write};
use std::mem::MaybeUninit;
use std::os::unix::ffi::{OsStrExt, OsStringExt};
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};

const GET_VERSION: u32 = 7;

#[derive(Debug, Default, PartialEq, Eq)]
struct VersionReply {
    human_readable: Option<Vec<u8>>,
    config_file: Option<Vec<u8>>,
    included_files: Vec<Vec<u8>>,
}

#[derive(Default)]
enum VersionKey {
    HumanReadable,
    ConfigFile,
    IncludedFiles,
    #[default]
    Other,
}

#[derive(Default)]
struct VersionReader {
    reply: VersionReply,
    key: VersionKey,
}
struct SharedReader(Rc<RefCell<VersionReader>>);

fn c_string_prefix(bytes: &[u8]) -> &[u8] {
    bytes.split(|byte| *byte == 0).next().unwrap()
}

impl EventHandler for SharedReader {
    fn on_event(&mut self, event: Event<'_>) -> bool {
        let mut reader = self.0.borrow_mut();
        match event {
            Event::MapKey(key) => {
                reader.key = match key {
                    b"human_readable" => VersionKey::HumanReadable,
                    b"loaded_config_file_name" => VersionKey::ConfigFile,
                    b"included_config_file_names" => VersionKey::IncludedFiles,
                    _ => VersionKey::Other,
                };
            }
            Event::String(value) => {
                // The previous printf("%.*s") callbacks stopped at a decoded
                // NUL. Keep that observable behavior, without C allocations.
                let value = c_string_prefix(value).to_vec();
                match reader.key {
                    VersionKey::HumanReadable => reader.reply.human_readable = Some(value),
                    VersionKey::ConfigFile => reader.reply.config_file = Some(value),
                    VersionKey::IncludedFiles => reader.reply.included_files.push(value),
                    VersionKey::Other => {}
                }
            }
            _ => {}
        }
        true
    }
}

impl VersionReply {
    fn parse(bytes: &[u8]) -> Result<Self, ParseError> {
        let reader = Rc::new(RefCell::new(VersionReader::default()));
        {
            let mut parser = Parser::new(SharedReader(Rc::clone(&reader)));
            parser.options_mut().allow_legacy_whitespace = true;
            // Upstream called yajl_parse without complete_parse. Retain its
            // accepted partial-root prefixes and streaming callback ordering.
            parser.parse(bytes)?;
        }
        Ok(Rc::try_unwrap(reader)
            .unwrap_or_else(|_| unreachable!("the parser dropped its only reader clone"))
            .into_inner()
            .reply)
    }
}

#[derive(Debug)]
enum RequestError {
    Write(io::Error),
    Read(ReceiveError),
    WrongType(u32),
    Json(Vec<u8>),
}

fn request_version(stream: &mut (impl Read + Write)) -> Result<VersionReply, RequestError> {
    send_message(stream, GET_VERSION, b"").map_err(RequestError::Write)?;
    let message = receive_message(stream).map_err(RequestError::Read)?;
    if message.message_type != GET_VERSION {
        return Err(RequestError::WrongType(message.message_type));
    }
    VersionReply::parse(&message.payload).map_err(|_| RequestError::Json(message.payload))
}

#[derive(Debug)]
enum Failure {
    Silent,
    Diagnostic(Vec<u8>),
    System {
        operation: Vec<u8>,
        error: io::Error,
    },
}

fn system_failure(operation: impl Into<Vec<u8>>, error: io::Error) -> Failure {
    Failure::System {
        operation: operation.into(),
        error,
    }
}

fn request_failure(error: RequestError) -> Failure {
    match error {
        RequestError::Write(error) => system_failure(b"IPC: write()".to_vec(), error),
        RequestError::Read(ReceiveError::Io(error)) => {
            system_failure(b"IPC: read()".to_vec(), error)
        }
        RequestError::Read(ReceiveError::Eof) => Failure::Silent,
        RequestError::Read(error) => {
            let mut message = b"[libi3] ERROR: ".to_vec();
            message.extend_from_slice(&error.diagnostic_bytes());
            message.push(b'\n');
            emit(Level::Error, &message);
            Failure::Silent
        }
        RequestError::WrongType(actual) => Failure::Diagnostic(
            format!(
                "Got reply type {}, but expected {GET_VERSION} (GET_VERSION)",
                actual as i32
            )
            .into_bytes(),
        ),
        RequestError::Json(payload) => {
            let mut message = b"Could not parse my own reply. That's weird. reply is ".to_vec();
            message.extend_from_slice(c_string_prefix(&payload));
            Failure::Diagnostic(message)
        }
    }
}

fn operation_path(operation: &str, path: &Path) -> Vec<u8> {
    let mut result = operation.as_bytes().to_vec();
    result.push(b'(');
    result.extend_from_slice(path.as_os_str().as_bytes());
    result.push(b')');
    result
}

fn proc_path(pid: &[u8], file: &[u8]) -> PathBuf {
    let mut bytes = b"/proc/".to_vec();
    bytes.extend_from_slice(pid);
    bytes.push(b'/');
    bytes.extend_from_slice(file);
    PathBuf::from(OsString::from_vec(bytes))
}

fn read_command_name(input: &mut impl BufRead) -> io::Result<Vec<u8>> {
    let mut bytes = Vec::new();
    input.read_until(0, &mut bytes)?;
    if bytes.last() == Some(&0) {
        bytes.pop();
    }
    Ok(bytes)
}

fn locale_timestamp(seconds: libc::time_t) -> Vec<u8> {
    let mut local = MaybeUninit::<libc::tm>::uninit();
    let mut bytes = [0u8; 64];
    // Both buffers are correctly sized and exclusively borrowed. localtime_r
    // fills the tm before strftime reads it; a failed/oversized conversion does
    // not expose uninitialized data or read beyond the bounded output buffer.
    let length = unsafe {
        if libc::localtime_r(&seconds, local.as_mut_ptr()).is_null() {
            return Vec::new();
        }
        libc::strftime(
            bytes.as_mut_ptr().cast(),
            bytes.len(),
            c"%c".as_ptr(),
            local.as_ptr(),
        )
    };
    bytes[..length].to_vec()
}

fn unix_seconds(now: SystemTime) -> i128 {
    match now.duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_secs().into(),
        Err(error) => {
            let duration = error.duration();
            -i128::from(duration.as_secs()) - i128::from(duration.subsec_nanos() != 0)
        }
    }
}

fn write_config_path(output: &mut impl Write, bytes: &[u8], role: &str) -> io::Result<()> {
    output.write_all(b"  ")?;
    output.write_all(bytes)?;
    write!(output, " ({role})")?;
    match fs::metadata(Path::new(OsStr::from_bytes(bytes))) {
        Ok(metadata) => {
            output.write_all(b" (last modified: ")?;
            output.write_all(&locale_timestamp(metadata.mtime()))?;
            let seconds_ago =
                (unix_seconds(SystemTime::now()) - i128::from(metadata.mtime())) as f64;
            writeln!(output, ", {seconds_ago:.0} seconds ago)")
        }
        Err(_) => {
            output.write_all(b"\n")?;
            let mut message = b"ERROR: Cannot stat config file \"".to_vec();
            message.extend_from_slice(bytes);
            message.extend_from_slice(b"\"\n");
            emit(Level::Error, &message);
            Ok(())
        }
    }
}

fn io_output(error: io::Error) -> Failure {
    system_failure(b"write()".to_vec(), error)
}

fn write_running_version(output: &mut impl Write, pid: &[u8]) -> Result<(), Failure> {
    output
        .write_all(b"(Getting version from running i3, press ctrl-c to abort\xe2\x80\xa6)")
        .map_err(io_output)?;
    output.flush().map_err(io_output)?;
    let path = socket_path(None).map_err(|error| {
        system_failure(
            b"Could not determine i3 socket path".to_vec(),
            io::Error::from_raw_os_error(error.raw_os_error().unwrap_or(libc::ENOENT)),
        )
    })?;
    // Legacy sockaddr_un callers retained only107 path bytes. Preserve that
    // connection selection while keeping the stream and its descriptor owned.
    let bytes = path.as_os_str().as_bytes();
    let socket = Path::new(OsStr::from_bytes(&bytes[..bytes.len().min(107)]));
    let mut stream = connect_path(socket).map_err(|error| {
        let mut operation = b"Could not connect to i3 on socket ".to_vec();
        operation.extend_from_slice(bytes);
        system_failure(operation, error)
    })?;
    let version = request_version(&mut stream).map_err(request_failure)?;
    write_report(output, pid, version)
}

fn write_report(output: &mut impl Write, pid: &[u8], version: VersionReply) -> Result<(), Failure> {
    output
        .write_all(b"\r\x1b[KRunning i3 version: ")
        .map_err(io_output)?;
    output
        .write_all(version.human_readable.as_deref().unwrap_or(b"(null)"))
        .map_err(io_output)?;
    output.write_all(b" (pid ").map_err(io_output)?;
    output.write_all(pid).map_err(io_output)?;
    output.write_all(b")\n").map_err(io_output)?;
    if let Some(config_file) = version.config_file {
        output
            .write_all(b"Loaded i3 config:\n")
            .map_err(io_output)?;
        write_config_path(output, &config_file, "main").map_err(io_output)?;
        for included in version.included_files {
            write_config_path(output, &included, "included").map_err(io_output)?;
        }
    }
    let own_exe = proc_path(std::process::id().to_string().as_bytes(), b"exe");
    let own_binary = fs::read_link(&own_exe)
        .map_err(|error| system_failure(operation_path("readlink", &own_exe), error))?;
    output
        .write_all(b"\nThe i3 binary you just called: ")
        .map_err(io_output)?;
    output
        .write_all(own_binary.as_os_str().as_bytes())
        .map_err(io_output)?;
    output.write_all(b"\n").map_err(io_output)?;
    let running_exe = proc_path(pid, b"exe");
    let running_binary = fs::read_link(&running_exe)
        .map_err(|error| system_failure(operation_path("readlink", &running_exe), error))?;
    if running_binary
        .as_os_str()
        .as_bytes()
        .windows(b"(deleted)".len())
        .any(|window| window == b"(deleted)")
    {
        output
            .write_all(b"RUNNING BINARY DIFFERENT FROM BINARY ON DISK!\n")
            .map_err(io_output)?;
    }
    let cmdline = proc_path(pid, b"cmdline");
    let file = File::open(&cmdline)
        .map_err(|error| system_failure(operation_path("open", &cmdline), error))?;
    // Read the complete first argument, not sizeof(pointer) bytes over a stale
    // executable-path buffer. This intentionally fixes custom-argv diagnostics.
    let command = read_command_name(&mut BufReader::new(file))
        .map_err(|error| system_failure(operation_path("read", &cmdline), error))?;
    output
        .write_all(b"The i3 binary you are running: ")
        .map_err(io_output)?;
    output.write_all(&command).map_err(io_output)?;
    output.write_all(b"\n").map_err(io_output)
}

fn write_failure(output: &mut impl Write, failure: Failure) {
    let message = match failure {
        Failure::Silent => return,
        Failure::Diagnostic(message) => message,
        Failure::System {
            mut operation,
            error,
        } => {
            operation.extend_from_slice(b": ");
            let description = error.to_string();
            let description = error
                .raw_os_error()
                .map(|code| format!(" (os error {code})"))
                .and_then(|suffix| description.strip_suffix(&suffix))
                .unwrap_or(&description);
            operation.extend_from_slice(description.as_bytes());
            operation
        }
    };
    let program = std::env::args_os().next().unwrap_or_else(|| "i3".into());
    let name = program
        .as_bytes()
        .rsplit(|byte| *byte == b'/')
        .next()
        .unwrap();
    let _ = output.write_all(name);
    let _ = output.write_all(b": ");
    let _ = output.write_all(&message);
    let _ = output.write_all(b"\n");
}

pub fn display_running_version() -> libc::c_int {
    if std::env::var_os("DISPLAY").is_none() {
        let _ = std::io::stderr().lock().write_all(b"\nYour DISPLAY environment variable is not set.\nAre you running i3 via SSH or on a virtual console?\nTry DISPLAY=:0 i3 --moreversion\n");
        return libc::EXIT_FAILURE;
    }
    let mut output = std::io::stdout().lock();
    let Some(pid) = root_property(c"I3_PID") else {
        let _ = output.write_all(b"\nRunning version: < 4.2-200\n");
        return libc::EXIT_SUCCESS;
    };
    match write_running_version(&mut output, &pid) {
        Ok(()) => libc::EXIT_SUCCESS,
        Err(error) => {
            write_failure(&mut std::io::stderr().lock(), error);
            libc::EXIT_FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn owned_version_reply_retains_streaming_keys_nul_and_partial_root_behavior() {
        let reply = VersionReply::parse(br#"{"human_readable":"old","human_readable":"v\u0000tail","loaded_config_file_name":["first","last"],"included_config_file_names":["one",["two\u0000tail"]]}"#).unwrap();
        assert_eq!(
            reply,
            VersionReply {
                human_readable: Some(b"v".to_vec()),
                config_file: Some(b"last".to_vec()),
                included_files: vec![b"one".to_vec(), b"two".to_vec()],
            }
        );
        assert_eq!(
            VersionReply::parse(b"\x0b{\x0c\"human_readable\":\"partial\"")
                .unwrap()
                .human_readable,
            Some(b"partial".to_vec())
        );
        assert_eq!(
            VersionReply::parse(br#"{"human_readable\u0000tail":"ignored"}"#).unwrap(),
            VersionReply::default()
        );
        assert_eq!(VersionReply::parse(b"{}").unwrap(), VersionReply::default());
        assert!(VersionReply::parse(b"{bad").is_err());
    }

    struct MemoryIpc {
        response: Cursor<Vec<u8>>,
        request: Vec<u8>,
    }
    impl Read for MemoryIpc {
        fn read(&mut self, bytes: &mut [u8]) -> io::Result<usize> {
            self.response.read(bytes)
        }
    }
    impl Write for MemoryIpc {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            self.request.extend_from_slice(bytes);
            Ok(bytes.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    fn ipc(kind: u32, payload: &[u8]) -> MemoryIpc {
        let mut bytes = Vec::new();
        send_message(&mut bytes, kind, payload).unwrap();
        MemoryIpc {
            response: Cursor::new(bytes),
            request: Vec::new(),
        }
    }

    #[test]
    fn owned_ipc_request_checks_reply_type_and_payload_without_raw_fd_transfer() {
        let mut stream = ipc(
            GET_VERSION,
            br#"{"human_readable":"version","included_config_file_names":["a"]}"#,
        );
        let reply = request_version(&mut stream).unwrap();
        assert_eq!(reply.human_readable, Some(b"version".to_vec()));
        assert_eq!(reply.included_files, [b"a".to_vec()]);
        let request = receive_message(&mut Cursor::new(stream.request)).unwrap();
        assert_eq!(request.message_type, GET_VERSION);
        assert!(request.payload.is_empty());
        assert!(matches!(
            request_version(&mut ipc(99, b"{}")),
            Err(RequestError::WrongType(99))
        ));
        assert!(
            matches!(request_failure(RequestError::WrongType(u32::MAX)), Failure::Diagnostic(bytes)
            if bytes == b"Got reply type -1, but expected 7 (GET_VERSION)")
        );
        assert!(
            matches!(request_version(&mut ipc(GET_VERSION, b"{bad")), Err(RequestError::Json(bytes)) if bytes == b"{bad")
        );
        let mut closed = MemoryIpc {
            response: Cursor::new(Vec::new()),
            request: Vec::new(),
        };
        assert!(matches!(
            request_version(&mut closed),
            Err(RequestError::Read(ReceiveError::Eof))
        ));
    }

    #[test]
    fn command_name_reads_complete_first_argument_without_stale_bytes() {
        let mut command = vec![b'x'; 4096];
        command.extend_from_slice(b"\xff\0second\0");
        let result = read_command_name(&mut Cursor::new(command)).unwrap();
        assert_eq!(result.len(), 4097);
        assert_eq!(result.last(), Some(&255));
        assert_eq!(
            read_command_name(&mut Cursor::new(b"short\0other")).unwrap(),
            b"short"
        );
        assert_eq!(
            read_command_name(&mut Cursor::new(b"unterminated")).unwrap(),
            b"unterminated"
        );
        assert!(read_command_name(&mut Cursor::new(b"")).unwrap().is_empty());
        assert!(read_command_name(&mut Cursor::new(b"\0other"))
            .unwrap()
            .is_empty());
    }

    #[test]
    fn report_owns_included_paths_and_reads_the_actual_process_argument() {
        let directory = i3_test_support::workspace_root()
            .join(".work/tmp")
            .join(format!(
                "moreversion-{}-{}",
                std::process::id(),
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_nanos()
            ));
        fs::create_dir_all(&directory).unwrap();
        let main = directory.join("main");
        let included = directory.join("included");
        fs::write(&main, b"main").unwrap();
        fs::write(&included, b"included").unwrap();
        let mut output = Vec::new();
        write_report(
            &mut output,
            std::process::id().to_string().as_bytes(),
            VersionReply {
                human_readable: Some(b"test-version".to_vec()),
                config_file: Some(main.as_os_str().as_bytes().to_vec()),
                included_files: vec![included.as_os_str().as_bytes().to_vec()],
            },
        )
        .unwrap();
        fs::remove_dir_all(directory).unwrap();
        assert!(output.starts_with(b"\r\x1b[KRunning i3 version: test-version (pid "));
        assert!(output
            .windows(b" (main) (last modified: ".len())
            .any(|window| window == b" (main) (last modified: "));
        assert!(output
            .windows(b" (included) (last modified: ".len())
            .any(|window| window == b" (included) (last modified: "));
        let command = std::env::args_os().next().unwrap();
        let mut expected = b"The i3 binary you are running: ".to_vec();
        expected.extend_from_slice(command.as_bytes());
        expected.push(b'\n');
        assert!(output.ends_with(&expected));
    }

    #[test]
    fn proc_paths_and_diagnostics_retain_bytes_and_native_error_spelling() {
        assert_eq!(
            proc_path(b"12\xff", b"exe").as_os_str().as_bytes(),
            b"/proc/12\xff/exe"
        );
        assert_eq!(
            unix_seconds(UNIX_EPOCH - std::time::Duration::from_millis(100)),
            -1
        );
        let mut output = Vec::new();
        write_failure(
            &mut output,
            system_failure(
                b"open(\xff)".to_vec(),
                io::Error::from_raw_os_error(libc::ENOENT),
            ),
        );
        assert!(output
            .windows(b"open(\xff): ".len())
            .any(|window| window == b"open(\xff): "));
        assert!(!output
            .windows(b"(os error".len())
            .any(|window| window == b"(os error"));
        assert_eq!(output.last(), Some(&b'\n'));
    }
}
