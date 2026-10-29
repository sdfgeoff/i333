// Copyright © 2009 Michael Stapelberg and contributors (see LICENSE).
//! Read the shared-memory ring and, optionally, i3's live log stream.

use i3_common::ipc_connect::{connect_path, socket_path};
use i3_common::ipc_recv_message::receive_message;
use i3_common::ipc_send_message::send_message;
use std::env;
use std::ffi::{CString, OsString};
use std::fs::File;
use std::io::{self, Read, Write};
use std::os::fd::FromRawFd;
use std::os::unix::ffi::OsStringExt;
use std::os::unix::fs::FileExt;
use std::os::unix::net::UnixStream;
use std::path::PathBuf;
use x11rb::connection::Connection;
use x11rb::protocol::xproto::{AtomEnum, ConnectionExt};
use x11rb::rust_connection::RustConnection;

const VERSION: &str = env!("I3_VERSION");
const HEADER_SIZE: usize = 16;

#[derive(Default, Debug)]
struct Options {
    verbose: bool,
    follow: bool,
}
enum Action {
    Run(Options),
    Help,
    Version,
}

const LOG_OPTIONS: &[i3_common::cli::OptionSpec] = &[
    i3_common::cli::OptionSpec::flag(b'v', "version"),
    i3_common::cli::OptionSpec::flag(b'V', "verbose"),
    i3_common::cli::OptionSpec::flag(b'f', "follow"),
    i3_common::cli::OptionSpec::flag(b'h', "help"),
    i3_common::cli::OptionSpec::value(b's', "socket").short_only(),
];

/// dump-log historically diagnoses unknown flags, then continues processing.
fn parse(arguments: &[OsString], posix: bool) -> (Action, Vec<String>) {
    use i3_common::cli::{Item, Parser};
    let mut options = Options::default();
    let mut diagnostics = Vec::new();
    for item in Parser::new(arguments.iter().cloned(), LOG_OPTIONS, posix) {
        match item {
            Item::Error(message) => diagnostics.push(message),
            Item::Operand(_) => {}
            Item::Option { short, .. } => match short {
                b'v' => return (Action::Version, diagnostics),
                b'h' => return (Action::Help, diagnostics),
                b'V' => options.verbose = true,
                b'f' => options.follow = true,
                b's' => {} // legacy accepted-but-unused short argument
                _ => unreachable!(),
            },
        }
    }
    (Action::Run(options), diagnostics)
}

fn os_message(error: &io::Error) -> String {
    let message = error.to_string();
    message
        .split(" (os error ")
        .next()
        .unwrap_or(&message)
        .to_owned()
}
fn io_context(context: &str, error: io::Error) -> String {
    format!("{context}: {}", os_message(&error))
}

struct RootProperties {
    connection: RustConnection,
    root: u32,
}
impl RootProperties {
    fn connect() -> Result<Self, ()> {
        let (connection, screen) = x11rb::connect(None).map_err(|_| ())?;
        let root = connection.setup().roots.get(screen).ok_or(())?.root;
        Ok(Self { connection, root })
    }
    fn get(&self, name: &[u8]) -> Option<Vec<u8>> {
        let atom = self
            .connection
            .intern_atom(false, name)
            .ok()?
            .reply()
            .ok()?
            .atom;
        let reply = self
            .connection
            .get_property(false, self.root, atom, AtomEnum::ANY, 0, u32::MAX)
            .ok()?
            .reply()
            .ok()?;
        if reply.value.is_empty() {
            return None;
        }
        Some(
            reply
                .value
                .split(|byte| *byte == 0)
                .next()
                .unwrap_or_default()
                .to_vec(),
        )
    }
}

/// Own the temporary logging change so every normal error path restores it.
struct EnabledLog {
    stream: UnixStream,
    active: bool,
}
impl EnabledLog {
    fn enable() -> Result<Self, String> {
        let path = socket_path(None)
            .map_err(|error| io_context("Could not determine i3 socket path", error))?;
        let stream = connect_path(&path).map_err(|error| {
            io_context(
                &format!("Could not connect to i3 on socket {}", path.display()),
                error,
            )
        })?;
        let mut guard = Self {
            stream,
            active: false,
        };
        guard.command(b"debuglog on; shmlog 5242880")?;
        guard.active = true;
        Ok(guard)
    }
    fn command(&mut self, command: &[u8]) -> Result<(), String> {
        send_message(&mut self.stream, 0, command)
            .map_err(|error| io_context("IPC send", error))?;
        receive_message(&mut self.stream).map_err(|error| format!("IPC recv: {error}"))?;
        Ok(())
    }
    fn finish(&mut self) -> Result<(), String> {
        if !self.active {
            return Ok(());
        }
        self.active = false;
        self.command(b"debuglog off; shmlog off")
    }
}
impl Drop for EnabledLog {
    fn drop(&mut self) {
        let _ = self.finish();
    }
}

fn open_shared_memory(name: &[u8]) -> io::Result<File> {
    let name = CString::new(name)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "SHM name contains NUL"))?;
    // SAFETY: POSIX borrows the terminated name. The returned descriptor is new
    // and transfers immediately to File, which closes it on every return path.
    let descriptor = unsafe { libc::shm_open(name.as_ptr(), libc::O_RDWR | libc::O_CLOEXEC, 0) };
    if descriptor < 0 {
        return Err(io::Error::last_os_error());
    }
    // SAFETY: descriptor is valid and uniquely owned after successful shm_open.
    Ok(unsafe { File::from_raw_fd(descriptor) })
}

#[derive(Debug, PartialEq)]
struct Header {
    next_write: usize,
    last_wrap: usize,
    size: u32,
    wrap_count: u32,
}
impl Header {
    fn read(bytes: &[u8]) -> io::Result<Self> {
        if bytes.len() < HEADER_SIZE {
            return Err(invalid_log("SHM log header is truncated"));
        }
        let word = |at| u32::from_ne_bytes(bytes[at..at + 4].try_into().unwrap());
        let header = Self {
            next_write: word(0) as usize,
            last_wrap: word(4) as usize,
            size: word(8),
            wrap_count: word(12),
        };
        if !(HEADER_SIZE..=bytes.len()).contains(&header.next_write) {
            return Err(invalid_log("SHM log write offset is outside the segment"));
        }
        if header.wrap_count != 0 && !(HEADER_SIZE..=bytes.len()).contains(&header.last_wrap) {
            return Err(invalid_log("SHM log wrap offset is outside the segment"));
        }
        Ok(header)
    }
}
fn invalid_log(message: &'static str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}

/// The shared-memory ABI is four native-endian u32s, followed by the ring.
/// Read into owned bytes instead of aliasing memory concurrently written by i3.
fn read_snapshot(file: &File, length: usize) -> io::Result<Vec<u8>> {
    let mut bytes = Vec::new();
    bytes
        .try_reserve_exact(length)
        .map_err(|_| io::Error::new(io::ErrorKind::OutOfMemory, "SHM log is too large"))?;
    bytes.resize(length, 0);
    file.read_exact_at(&mut bytes, 0)?;
    Ok(bytes)
}
fn write_snapshot(bytes: &[u8], header: &Header, output: &mut impl Write) -> io::Result<()> {
    if header.wrap_count != 0 && header.next_write < header.last_wrap {
        let old = &bytes[header.next_write..header.last_wrap];
        // The first old line is often partial. Match dump-log's skip rule.
        let start = if old.first() == Some(&0) {
            0
        } else {
            old.iter()
                .position(|byte| *byte == b'\n')
                .map_or(old.len(), |at| at + 1)
        };
        output.write_all(&old[start..])?;
    }
    output.write_all(&bytes[HEADER_SIZE..header.next_write])
}

fn dump(
    properties: &RootProperties,
    name: &[u8],
    options: &Options,
    output: &mut impl Write,
) -> Result<(), String> {
    if name.is_empty() {
        return Err("Cannot dump log: SHM logging is disabled in i3.".to_owned());
    }
    let display_name = String::from_utf8_lossy(name);
    let file = open_shared_memory(name).map_err(|error| {
        io_context(
            &format!("Could not shm_open SHM segment for the i3 log ({display_name})"),
            error,
        )
    })?;
    let length = file
        .metadata()
        .map_err(|error| io_context(&format!("stat({display_name})"), error))?
        .len();
    let length = usize::try_from(length).map_err(|_| "SHM segment is too large".to_owned())?;
    let bytes = read_snapshot(&file, length)
        .map_err(|error| io_context("Could not read SHM segment for the i3 log", error))?;
    let header = Header::read(&bytes).map_err(|error| error.to_string())?;
    if options.verbose {
        writeln!(
            output,
            "next_write = {}, last_wrap = {}, logbuffer_size = {}, shmname = {display_name}",
            header.next_write, header.last_wrap, header.size
        )
        .map_err(|error| io_context("Failed to write 1", error))?;
    }
    write_snapshot(&bytes, &header, output)
        .map_err(|error| io_context("Failed to write 1", error))?;
    output
        .flush()
        .map_err(|error| io_context("Failed to write 1", error))?;
    if !options.follow {
        return Ok(());
    }
    let path = properties.get(b"I3_LOG_STREAM_SOCKET_PATH").ok_or_else(|| "could not determine i3 log stream socket path: possible i3-dump-log and i3 version mismatch".to_owned())?;
    let path = PathBuf::from(OsString::from_vec(path));
    let mut stream = UnixStream::connect(&path).map_err(|error| {
        io_context(
            &format!("Could not connect to i3 on socket {}", path.display()),
            error,
        )
    })?;
    let mut buffer = [0; 4096];
    loop {
        let count = match stream.read(&mut buffer) {
            Ok(count) => count,
            Err(error) if error.kind() == io::ErrorKind::Interrupted => continue,
            Err(error) => return Err(io_context("read(log-stream-socket):", error)),
        };
        if count == 0 {
            return Ok(());
        }
        output
            .write_all(&buffer[..count])
            .map_err(|error| io_context("Failed to write 1", error))?;
        output
            .flush()
            .map_err(|error| io_context("Failed to write 1", error))?;
    }
}

fn execute(
    options: Options,
    output: &mut impl Write,
    errors: &mut impl Write,
) -> Result<(), String> {
    let properties = match RootProperties::connect() {
        Ok(properties) => properties,
        Err(()) => {
            let _ = writeln!(errors, "i3-dump-log: ERROR: Cannot connect to X11.\n");
            match env::var_os("DISPLAY") {
                None => {
                    let _ = writeln!(errors, "Your DISPLAY environment variable is not set.\nAre you running i3-dump-log via SSH or on a virtual console?\nTry DISPLAY=:0 i3-dump-log");
                }
                Some(display) => {
                    let _ = writeln!(
                        errors,
                        "FYI: The DISPLAY environment variable is set to \"{}\".",
                        display.to_string_lossy()
                    );
                }
            }
            return Err(String::new());
        }
    };
    let mut enabled = None;
    let name = match properties.get(b"I3_SHMLOG_PATH") {
        Some(name) => Some(name),
        None if properties.get(b"I3_CONFIG_PATH").is_some() => {
            let _ = writeln!(errors, "i3-dump-log: i3 is running, but SHM logging is not enabled. Enabling SHM log now while i3-dump-log is running\n");
            enabled = Some(EnabledLog::enable()?);
            let name = properties.get(b"I3_SHMLOG_PATH");
            if name.is_none() && VERSION.find('(').is_some_and(|at| at <= 10) {
                let _ = writeln!(errors, "You seem to be using a release version of i3:\n  {VERSION}\n\nRelease versions do not use SHM logging by default,\ntherefore i3-dump-log does not work.\n\nPlease follow this guide instead:\nhttps://i3wm.org/docs/debugging-release-version.html");
                return Err(String::new());
            }
            name
        }
        None => None,
    };
    let name = name.ok_or_else(|| {
        "Cannot get I3_SHMLOG_PATH atom contents. Is i3 running on this display?".to_owned()
    })?;
    let result = dump(&properties, &name, &options, output);
    if let Some(mut enabled) = enabled {
        enabled.finish()?;
    }
    result
}

pub fn run() -> i32 {
    let mut arguments = env::args_os();
    let invocation = arguments
        .next()
        .unwrap_or_else(|| OsString::from("i3-dump-log"));
    let program = std::path::Path::new(&invocation)
        .file_name()
        .unwrap_or(invocation.as_os_str())
        .to_string_lossy();
    let mut output = io::stdout().lock();
    let mut errors = io::stderr().lock();
    let (action, diagnostics) = parse(
        &arguments.collect::<Vec<_>>(),
        env::var_os("POSIXLY_CORRECT").is_some(),
    );
    for message in diagnostics {
        let _ = writeln!(errors, "{}: {message}", invocation.to_string_lossy());
    }
    let result = match action {
        Action::Version => {
            writeln!(output, "i3-dump-log {VERSION}").map_err(|error| error.to_string())
        }
        Action::Help => writeln!(output, "i3-dump-log {VERSION}\ni3-dump-log [-fhVv]")
            .map_err(|error| error.to_string()),
        Action::Run(options) => execute(options, &mut output, &mut errors),
    };
    match result.and_then(|()| output.flush().map_err(|error| error.to_string())) {
        Ok(()) => 0,
        Err(message) => {
            if !message.is_empty() {
                let _ = writeln!(errors, "{program}: {message}");
            }
            1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn ring(next: u32, last: u32, wraps: u32, body: &[u8]) -> Vec<u8> {
        let size = (HEADER_SIZE + body.len()) as u32;
        [
            next.to_ne_bytes().as_slice(),
            last.to_ne_bytes().as_slice(),
            size.to_ne_bytes().as_slice(),
            wraps.to_ne_bytes().as_slice(),
            body,
        ]
        .concat()
    }
    #[test]
    fn snapshot_without_wrap_prints_only_written_bytes() {
        let bytes = ring(20, 0, 0, b"new\nignored");
        let header = Header::read(&bytes).unwrap();
        let mut output = Vec::new();
        write_snapshot(&bytes, &header, &mut output).unwrap();
        assert_eq!(output, b"new\n");
    }
    #[test]
    fn wrapped_snapshot_skips_partial_old_line_and_preserves_order() {
        let bytes = ring(20, 32, 1, b"new\npartial\nold\ntrailing");
        let header = Header::read(&bytes).unwrap();
        let mut output = Vec::new();
        write_snapshot(&bytes, &header, &mut output).unwrap();
        assert_eq!(output, b"old\nnew\n");
    }
    #[test]
    fn invalid_offsets_and_short_headers_never_index_out_of_bounds() {
        assert!(Header::read(&[0; 15]).is_err());
        assert!(Header::read(&ring(1, 0, 0, b"data")).is_err());
        assert!(Header::read(&ring(900, 0, 0, b"data")).is_err());
        assert!(Header::read(&ring(16, 900, 1, b"data")).is_err());
        let bytes = ring(20, 0, 0, b"data");
        assert!(Header::read(&bytes).is_ok());
    }
    #[test]
    fn flags_support_clusters_abbreviations_and_legacy_ignored_socket() {
        let arguments = ["-Vf", "-s", "unused", "operand"].map(OsString::from);
        let (Action::Run(options), errors) = parse(&arguments, false) else {
            panic!("expected run");
        };
        assert!(options.verbose && options.follow);
        assert!(errors.is_empty());
        let (action, errors) = parse(&[OsString::from("--v"), OsString::from("--help")], false);
        assert!(matches!(action, Action::Help));
        assert_eq!(
            errors,
            ["option '--v' is ambiguous; possibilities: '--version' '--verbose'"]
        );
    }
    #[test]
    fn verbose_only_argument_and_posix_operand_behavior() {
        let (Action::Run(options), _) =
            parse(&[OsString::from("operand"), OsString::from("-f")], true)
        else {
            panic!("expected run");
        };
        assert!(!options.follow);
        assert!(matches!(
            parse(&[OsString::from("--verbo"), OsString::from("-v")], false).0,
            Action::Version
        ));
    }
}
