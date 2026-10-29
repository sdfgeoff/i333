// Copyright © 2009 Michael Stapelberg and contributors (see LICENSE).
//! i3's IPC command-line client. Arguments and wire payloads retain their bytes.

use i3_common::ipc_connect::{connect_path, socket_path};
use i3_common::ipc_recv_message::{receive_message, Message, ReceiveError};
use i3_common::ipc_send_message::send_message;
use serde_json::Value;
use std::env;
use std::ffi::OsString;
use std::io::{self, Write};
use std::os::unix::ffi::{OsStrExt, OsStringExt};
use std::path::PathBuf;

const VERSION: &str = env!("I3_VERSION");
const EVENT_MASK: u32 = 1 << 31;
const TYPES: &[(&[u8], u32)] = &[
    (b"command", 0),
    (b"run_command", 0),
    (b"get_workspaces", 1),
    (b"get_outputs", 3),
    (b"get_tree", 4),
    (b"get_marks", 5),
    (b"get_bar_config", 6),
    (b"get_binding_modes", 8),
    (b"get_binding_state", 12),
    (b"get_version", 7),
    (b"get_config", 9),
    (b"send_tick", 10),
    (b"subscribe", 2),
];
const KNOWN_TYPES: &str = "Known types: run_command, get_workspaces, get_outputs, get_tree, get_marks, get_bar_config, get_binding_modes, get_binding_state, get_version, get_config, send_tick, subscribe\n";
const LONG_OPTIONS: &[i3_common::cli::OptionSpec] = &[
    i3_common::cli::OptionSpec::value(b's', "socket"),
    i3_common::cli::OptionSpec::value(b't', "type"),
    i3_common::cli::OptionSpec::flag(b'v', "version"),
    i3_common::cli::OptionSpec::flag(b'q', "quiet"),
    i3_common::cli::OptionSpec::flag(b'm', "monitor"),
    i3_common::cli::OptionSpec::flag(b'h', "help"),
    i3_common::cli::OptionSpec::flag(b'r', "raw"),
];

#[derive(Default, Debug)]
struct Options {
    socket: Option<PathBuf>,
    message_type: u32,
    payload: Vec<u8>,
    quiet: bool,
    monitor: bool,
    raw: bool,
}

enum Action {
    Run(Options),
    Help,
    Version,
}
enum ParseError {
    Usage(String),
    UnknownType,
}
enum Control {
    Continue,
    Help,
    Version,
}

fn apply(options: &mut Options, flag: u8, value: Option<Vec<u8>>) -> Result<Control, ParseError> {
    match flag {
        b's' => options.socket = Some(PathBuf::from(OsString::from_vec(value.unwrap()))),
        b't' => {
            let value = value.unwrap();
            options.message_type = TYPES
                .iter()
                .find(|(name, _)| value.eq_ignore_ascii_case(name))
                .map(|(_, kind)| *kind)
                .ok_or(ParseError::UnknownType)?;
        }
        b'q' => options.quiet = true,
        b'm' => options.monitor = true,
        b'r' => options.raw = true,
        b'h' => return Ok(Control::Help),
        b'v' => return Ok(Control::Version),
        _ => unreachable!("parser only dispatches known flags"),
    }
    Ok(Control::Continue)
}

/// Scan byte-preserving arguments with the same shared GNU option rules used
/// by the WM, bar and graphical tools.
fn parse(arguments: &[OsString], posix: bool) -> Result<Action, ParseError> {
    use i3_common::cli::{Item, Parser};
    let mut options = Options::default();
    let mut operands = Vec::new();
    for item in Parser::new(arguments.iter().cloned(), LONG_OPTIONS, posix) {
        match item {
            Item::Operand(value) => operands.push(value),
            Item::Error(message) => return Err(ParseError::Usage(message)),
            Item::Option { short, value } => {
                match apply(
                    &mut options,
                    short,
                    value.map(|value| value.as_bytes().to_vec()),
                )? {
                    Control::Help => return Ok(Action::Help),
                    Control::Version => return Ok(Action::Version),
                    Control::Continue => {}
                }
            }
        }
    }
    options.payload = operands
        .iter()
        .map(|value| value.as_bytes())
        .collect::<Vec<_>>()
        .join(&b' ');
    Ok(Action::Run(options))
}

// Keep the original printf output convention, including embedded NUL handling.
fn text(bytes: &[u8]) -> &[u8] {
    bytes.split(|byte| *byte == 0).next().unwrap_or_default()
}
fn line(output: &mut impl Write, bytes: &[u8]) -> io::Result<()> {
    output.write_all(text(bytes))?;
    output.write_all(b"\n")
}
fn os_message(error: &io::Error) -> String {
    let message = error.to_string();
    message
        .split(" (os error ")
        .next()
        .unwrap_or(&message)
        .to_owned()
}
fn diagnostic(
    error: &mut impl Write,
    program: &str,
    message: impl std::fmt::Display,
) -> io::Result<()> {
    writeln!(error, "{program}: {message}")
}
fn receive(
    stream: &mut std::os::unix::net::UnixStream,
    errors: &mut impl Write,
    program: &str,
) -> Result<Message, ()> {
    receive_message(stream).map_err(|error| match error {
        ReceiveError::Eof => {}
        ReceiveError::Io(error) => {
            let _ = diagnostic(
                errors,
                program,
                format!("IPC: read(): {}", os_message(&error)),
            );
        }
        error => {
            let _ = errors.write_all(b"[libi3] ERROR: ");
            let _ = line(errors, &error.diagnostic_bytes());
        }
    })
}

fn command_status(payload: &[u8], errors: &mut impl Write) -> Result<i32, ()> {
    let value: Value = serde_json::from_slice(payload).map_err(|_| ())?;
    let replies: Vec<&Value> = match &value {
        Value::Array(items) => items.iter().collect(),
        Value::Object(_) => vec![&value],
        _ => vec![],
    };
    let mut status = 0;
    for reply in replies {
        if reply
            .get("success")
            .and_then(Value::as_bool)
            .unwrap_or(false)
        {
            continue;
        }
        if let Some(input) = reply.get("input").and_then(Value::as_str) {
            errors.write_all(b"ERROR: Your command: ").map_err(|_| ())?;
            line(errors, input.as_bytes()).map_err(|_| ())?;
            errors.write_all(b"ERROR:               ").map_err(|_| ())?;
            line(
                errors,
                reply
                    .get("errorposition")
                    .and_then(Value::as_str)
                    .unwrap_or("(null)")
                    .as_bytes(),
            )
            .map_err(|_| ())?;
        }
        errors.write_all(b"ERROR: ").map_err(|_| ())?;
        line(
            errors,
            reply
                .get("error")
                .and_then(Value::as_str)
                .unwrap_or("(null)")
                .as_bytes(),
        )
        .map_err(|_| ())?;
        status = 2;
    }
    Ok(status)
}

fn execute(
    options: Options,
    output: &mut impl Write,
    errors: &mut impl Write,
    program: &str,
) -> io::Result<i32> {
    if options.monitor && options.message_type != 2 {
        writeln!(
            errors,
            "The monitor option -m is used with -t SUBSCRIBE exclusively."
        )?;
        return Ok(1);
    }
    let path = match socket_path(options.socket.as_deref()) {
        Ok(path) => path,
        Err(error) => {
            diagnostic(
                errors,
                program,
                format!("Could not determine i3 socket path: {}", os_message(&error)),
            )?;
            return Ok(1);
        }
    };
    let mut stream = match connect_path(&path) {
        Ok(stream) => stream,
        Err(error) => {
            diagnostic(
                errors,
                program,
                format!(
                    "Could not connect to i3 on socket {}: {}",
                    path.display(),
                    os_message(&error)
                ),
            )?;
            return Ok(1);
        }
    };
    if let Err(error) = send_message(&mut stream, options.message_type, &options.payload) {
        diagnostic(
            errors,
            program,
            format!("IPC: write(): {}", os_message(&error)),
        )?;
        return Ok(1);
    }
    let Ok(reply) = receive(&mut stream, errors, program) else {
        return Ok(1);
    };
    if reply.message_type != options.message_type {
        diagnostic(
            errors,
            program,
            format!(
                "IPC: Received reply of type {} but expected {}",
                reply.message_type, options.message_type
            ),
        )?;
        return Ok(1);
    }
    match reply.message_type {
        0 => {
            let status = if options.raw {
                0
            } else {
                match command_status(&reply.payload, errors) {
                    Ok(status) => status,
                    Err(()) => {
                        diagnostic(errors, program, "IPC: Could not parse JSON reply.")?;
                        return Ok(1);
                    }
                }
            };
            if !options.quiet || options.raw {
                line(output, &reply.payload)?;
            }
            Ok(status)
        }
        9 if !options.raw => match serde_json::from_slice::<Value>(&reply.payload) {
            Ok(value) => {
                if let Some(config) = value.get("config").and_then(Value::as_str) {
                    output.write_all(text(config.as_bytes()))?;
                }
                Ok(0)
            }
            Err(_) => {
                diagnostic(errors, program, "IPC: Could not parse JSON reply.")?;
                Ok(1)
            }
        },
        9 => {
            line(output, &reply.payload)?;
            Ok(0)
        }
        2 => loop {
            let Ok(event) = receive(&mut stream, errors, program) else {
                return Ok(1);
            };
            if event.message_type & EVENT_MASK == 0 {
                diagnostic(
                    errors,
                    program,
                    format!(
                        "IPC: Received reply of type {} but expected an event",
                        event.message_type
                    ),
                )?;
                return Ok(1);
            }
            if !options.quiet {
                line(output, &event.payload)?;
                output.flush()?;
            }
            if !options.monitor {
                return Ok(0);
            }
        },
        _ => {
            if !options.quiet {
                line(output, &reply.payload)?;
            }
            Ok(0)
        }
    }
}

pub fn run() -> i32 {
    let mut arguments = env::args_os();
    let invocation = arguments.next().unwrap_or_else(|| OsString::from("i3-msg"));
    let program = std::path::Path::new(&invocation)
        .file_name()
        .unwrap_or(invocation.as_os_str())
        .to_string_lossy();
    let mut output = io::stdout().lock();
    let mut errors = io::stderr().lock();
    let status = match parse(
        &arguments.collect::<Vec<_>>(),
        env::var_os("POSIXLY_CORRECT").is_some(),
    ) {
        Ok(Action::Version) => writeln!(output, "i3-msg {VERSION}").map(|_| 0),
        Ok(Action::Help) => writeln!(
            output,
            "i3-msg {VERSION}\ni3-msg [-s <socket>] [-t <type>] [-m] <message>"
        )
        .map(|_| 0),
        Ok(Action::Run(options)) => execute(options, &mut output, &mut errors, &program),
        Err(ParseError::UnknownType) => {
            write!(output, "Unknown message type\n{KNOWN_TYPES}").map(|_| 1)
        }
        Err(ParseError::Usage(message)) => {
            writeln!(errors, "{}: {message}", invocation.to_string_lossy()).map(|_| 1)
        }
    };
    if let Err(error) = output.flush() {
        let _ = diagnostic(&mut errors, &program, error);
        return 1;
    }
    match status {
        Ok(status) => status,
        Err(error) => {
            let _ = diagnostic(&mut errors, &program, error);
            1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn args(values: &[&[u8]]) -> Vec<OsString> {
        values
            .iter()
            .map(|value| OsString::from_vec(value.to_vec()))
            .collect()
    }
    fn options(values: &[&[u8]], posix: bool) -> Options {
        match parse(&args(values), posix) {
            Ok(Action::Run(options)) => options,
            _ => panic!("expected runnable options"),
        }
    }
    #[test]
    fn payload_retains_non_utf8_and_empty_operands() {
        let parsed = options(&[b"--", b"exec", b"\xff", b""], false);
        assert_eq!(parsed.payload, b"exec \xff ");
    }
    #[test]
    fn getopt_permutation_and_attached_values_are_preserved() {
        let parsed = options(
            &[
                b"mark",
                b"-qr",
                b"example",
                b"--t=GET_TREE",
                b"-s/tmp/test.sock",
            ],
            false,
        );
        assert_eq!(parsed.payload, b"mark example");
        assert!(parsed.quiet && parsed.raw);
        assert_eq!(parsed.message_type, 4);
        assert_eq!(parsed.socket, Some(PathBuf::from("/tmp/test.sock")));
    }
    #[test]
    fn posix_stops_at_first_operand() {
        assert_eq!(options(&[b"nop", b"-qr"], true).payload, b"nop -qr");
    }
    #[test]
    fn socket_argument_retains_its_bytes() {
        let parsed = options(&[b"--socket", b"/tmp/\xff.sock"], false);
        assert_eq!(
            parsed.socket.unwrap().as_os_str().as_bytes(),
            b"/tmp/\xff.sock"
        );
    }
    #[test]
    fn long_prefixes_and_end_marker_match_getopt() {
        let parsed = options(&[b"--q", b"--", b"-t", b"get_tree"], false);
        assert!(parsed.quiet);
        assert_eq!(parsed.payload, b"-t get_tree");
        match parse(&args(&[b"--soc"]), false) {
            Err(ParseError::Usage(message)) => {
                assert_eq!(message, "option '--socket' requires an argument")
            }
            _ => panic!("expected missing socket argument"),
        }
        assert!(matches!(
            parse(&args(&[b"--quiet=value"]), false),
            Err(ParseError::Usage(_))
        ));
        assert!(matches!(
            parse(&args(&[b"--type", b"unknown"]), false),
            Err(ParseError::UnknownType)
        ));
    }
    #[test]
    fn command_diagnostics_preserve_order_and_status() {
        let mut errors = Vec::new();
        let status = command_status(br#"[{"success":true},{"success":false,"error":"bad","input":"bad","errorposition":"^^^"},{"success":true}]"#, &mut errors).unwrap();
        assert_eq!(status, 2);
        assert_eq!(
            errors,
            b"ERROR: Your command: bad\nERROR:               ^^^\nERROR: bad\n"
        );
    }
    #[test]
    fn malformed_json_is_rejected_and_missing_optional_fields_are_safe() {
        let mut errors = Vec::new();
        assert!(command_status(b"not-json", &mut errors).is_err());
        assert_eq!(
            command_status(br#"[{"success":false,"error":"failed"}]"#, &mut errors),
            Ok(2)
        );
        assert_eq!(errors, b"ERROR: failed\n");
    }
    #[test]
    fn output_keeps_printf_nul_and_newline_conventions() {
        let mut output = Vec::new();
        line(&mut output, b"hello\0ignored").unwrap();
        assert_eq!(output, b"hello\n");
    }
}
