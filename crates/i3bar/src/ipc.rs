/* i3bar - Communicating with i3. See LICENSE. */
//! Frames own their bytes; Rust handler dispatch is separate from the native
//! libev/descriptor boundary. Tree/configuration effects stay on the event thread.
use crate::child::{clear_statusline, statusline_head};
use crate::config::config;
use crate::outputs::outputs;
use crate::xcb::kick_tray_clients;
use i3_common::ipc_recv_message::{receive_message, Message, ReceiveError};
use i3_common::ipc_send_message::send_message;
use libc::{exit, fprintf, printf};
use libev_sys::{ev_io, ev_io_start, ev_io_stop, ev_loop, EV_READ};
use std::ffi::CStr;
use std::io::{self, Read, Write};
use std::os::fd::{AsRawFd, BorrowedFd, FromRawFd, OwnedFd};
use std::ptr;

const I3_IPC_MESSAGE_TYPE_GET_WORKSPACES: u32 = 1;
const I3_IPC_MESSAGE_TYPE_SUBSCRIBE: u32 = 2;
const I3_IPC_MESSAGE_TYPE_GET_OUTPUTS: u32 = 3;
const I3_IPC_MESSAGE_TYPE_GET_BAR_CONFIG: u32 = 6;
const EVENT_BIT: u32 = 1 << 31;
type Handler = unsafe fn(&[u8]);

// Only DetachedCommand::take can create this malloc owner. The POD global
// config retains its raw-pointer ABI; old snapshots stay live through restart.
struct DetachedCommand(*mut libc::c_char);
impl DetachedCommand {
    /// slot exclusively owns a malloc string or null and is detached here.
    unsafe fn take(slot: *mut *mut libc::c_char) -> Self {
        Self(ptr::replace(slot, ptr::null_mut()))
    }
    fn as_c_str(&self) -> Option<&CStr> {
        // SAFETY: take preserves sole ownership of this live terminated string.
        unsafe { optional_c_str(self.0) }
    }
}
impl Drop for DetachedCommand {
    fn drop(&mut self) {
        unsafe { libc::free(self.0.cast()) }
    }
}
unsafe fn optional_c_str<'a>(value: *const libc::c_char) -> Option<&'a CStr> {
    if value.is_null() {
        None
    } else {
        Some(CStr::from_ptr(value))
    }
}
fn child_needs_restart(alive: bool, previous: Option<&CStr>, current: Option<&CStr>) -> bool {
    !alive || previous != current
}
fn prefix(bytes: &[u8]) -> &[u8] {
    bytes.split(|byte| *byte == 0).next().unwrap()
}
fn matches_bar_id(event: &[u8], id: &[u8]) -> bool {
    let mut expected = b"\"id\":\"".to_vec();
    expected.extend_from_slice(id);
    expected.push(b'"');
    prefix(event)
        .windows(expected.len())
        .any(|bytes| bytes == expected)
}
/// The event thread owns initialized native stdout. Use the same FILE buffer
/// as the surrounding printf diagnostics so redirected logs retain their order.
unsafe fn log_payload(line: u16, before: &[u8], payload: &[u8], after: &[u8]) {
    printf(c"[../i3bar/src/ipc.c:%d] ".as_ptr(), i32::from(line));
    for bytes in [before, prefix(payload), after] {
        libc::fwrite(
            bytes.as_ptr().cast(),
            1,
            bytes.len(),
            i3_common::native_stdio::stdout,
        );
    }
}
unsafe fn i3_provides_workspaces() -> bool {
    !config.disable_ws && config.workspace_command.is_null()
}
unsafe fn got_command_reply(_reply: &[u8]) {}
unsafe fn got_workspace_reply(reply: &[u8]) {
    if config.verbose {
        printf(
            c"[%s:%d] Got workspace data!\n".as_ptr(),
            c"../i3bar/src/ipc.c".as_ptr(),
            53i32,
        );
    }
    parse_workspaces_json(reply.as_ptr(), reply.len());
    draw_bars(false);
}
unsafe fn got_subscribe_reply(reply: &[u8]) {
    if config.verbose {
        log_payload(64, b"Got subscribe reply: ", reply, b"\n");
    }
}
unsafe fn got_output_reply(reply: &[u8]) {
    if config.verbose {
        printf(
            c"[%s:%d] Clearing old output configuration...\n".as_ptr(),
            c"../i3bar/src/ipc.c".as_ptr(),
            73i32,
        );
    }
    free_outputs();
    if config.verbose {
        printf(
            c"[%s:%d] Parsing outputs JSON...\n".as_ptr(),
            c"../i3bar/src/ipc.c".as_ptr(),
            76i32,
        );
    }
    parse_outputs_json(reply.as_ptr(), reply.len());
    if config.verbose {
        printf(
            c"[%s:%d] Reconfiguring windows...\n".as_ptr(),
            c"../i3bar/src/ipc.c".as_ptr(),
            78i32,
        );
    }
    reconfig_windows(false);
    let mut o_walk = (*outputs).slh_first;
    while !o_walk.is_null() {
        kick_tray_clients(o_walk);
        o_walk = (*o_walk).slist.sle_next;
    }
    if i3_provides_workspaces() {
        i3_send_msg(I3_IPC_MESSAGE_TYPE_GET_WORKSPACES, ptr::null());
    } else if !config.workspace_command.is_null() {
        repeat_last_ws_json();
    }
    draw_bars(false);
}
unsafe fn got_bar_config(reply: &[u8]) {
    if config.bar_id.is_null() {
        if config.verbose {
            log_payload(110, b"Received bar list \"", reply, b"\"\n");
        }
        parse_get_first_i3bar_config(reply.as_ptr(), reply.len());
        if config.bar_id.is_null() {
            fprintf(
                stderr,
                c"[%s:%d] ERROR: No bar configuration found, please configure a bar block in your i3 config file.\n".as_ptr(),
                c"../i3bar/src/ipc.c".as_ptr(),
                114i32,
            );
            exit(libc::EXIT_FAILURE);
        }
        verboselog(
            c"[libi3] ../i3bar/src/ipc.c Using first bar config: %s. Use --bar_id to manually select a different bar configuration.\n".as_ptr().cast_mut(),
            config.bar_id,
        );
        i3_send_msg(I3_IPC_MESSAGE_TYPE_GET_BAR_CONFIG, config.bar_id);
        return;
    }
    if config.verbose {
        log_payload(123, b"Received bar config \"", reply, b"\"\n");
    }
    i3_send_msg(I3_IPC_MESSAGE_TYPE_GET_OUTPUTS, ptr::null());
    free_colors(&raw mut config.colors);
    parse_config_json(reply.as_ptr(), reply.len());
    subscribe_events();
    if i3_provides_workspaces() {
        i3_send_msg(I3_IPC_MESSAGE_TYPE_GET_WORKSPACES, ptr::null());
    }
    init_xcb_late(config.fontname);
    init_colors(&raw mut config.colors);
    start_child(config.command);
    start_ws_child(config.workspace_command);
}

unsafe fn got_workspace_event(_event: &[u8]) {
    if config.verbose {
        printf(
            c"[%s:%d] Got workspace event!\n".as_ptr(),
            c"../i3bar/src/ipc.c".as_ptr(),
            170i32,
        );
    }
    i3_send_msg(I3_IPC_MESSAGE_TYPE_GET_WORKSPACES, ptr::null());
}
unsafe fn got_output_event(_event: &[u8]) {
    if config.verbose {
        printf(
            c"[%s:%d] Got output event!\n".as_ptr(),
            c"../i3bar/src/ipc.c".as_ptr(),
            179i32,
        );
    }
    i3_send_msg(I3_IPC_MESSAGE_TYPE_GET_OUTPUTS, ptr::null());
}
unsafe fn got_mode_event(event: &[u8]) {
    if config.verbose {
        printf(
            c"[%s:%d] Got mode event!\n".as_ptr(),
            c"../i3bar/src/ipc.c".as_ptr(),
            188i32,
        );
    }
    let parsed = match parse_mode_json(event) {
        Ok(parsed) => parsed,
        Err(_) => {
            fprintf(
                stderr,
                c"[%s:%d] ERROR: Could not parse mode event!\n".as_ptr(),
                c"../i3bar/src/mode.c".as_ptr(),
                116,
            );
            exit(libc::EXIT_FAILURE);
        }
    };
    let current = parsed.into_mode(
        |name| {
            // Font measurement reads a live, owned string. Its legacy pointer
            // API only changes interior text caches; ownership stays here.
            i3_common::font::predict_text_width(
                (name as *const i3_common::string::I3String).cast_mut(),
            )
        },
        config.verbose,
    );
    crate::xcb::set_current_mode(current);
    draw_bars(false);
}
unsafe fn got_bar_config_update(event: &[u8]) {
    // Keep the original compact substring match and first-NUL boundary; the
    // matching decision owns no global graph references across subsequent calls.
    if config.bar_id.is_null() || !matches_bar_id(event, CStr::from_ptr(config.bar_id).to_bytes()) {
        return;
    }
    i3_send_msg(I3_IPC_MESSAGE_TYPE_GET_OUTPUTS, ptr::null());
    free_colors(&raw mut config.colors);
    if config.verbose {
        log_payload(225, b"Received bar config update \"", event, b"\"\n");
    }
    let old_command = DetachedCommand::take(&raw mut config.command);
    let old_workspace_command = DetachedCommand::take(&raw mut config.workspace_command);
    let old_mode: crate::application::BarDisplayMode = config.hide_on_modifier;
    parse_config_json(event.as_ptr(), event.len());
    if old_mode != config.hide_on_modifier {
        reconfig_windows(true);
    }
    init_xcb_late(config.fontname);
    init_colors(&raw mut config.colors);
    if child_needs_restart(
        status_child_is_alive(),
        old_command.as_c_str(),
        optional_c_str(config.command),
    ) {
        kill_child();
        clear_statusline(&raw mut statusline_head, true);
        start_child(config.command);
    }
    drop(old_command);
    if child_needs_restart(
        ws_child_is_alive(),
        old_workspace_command.as_c_str(),
        optional_c_str(config.workspace_command),
    ) {
        free_workspaces();
        kill_ws_child();
        start_ws_child(config.workspace_command);
    }
    drop(old_workspace_command);
    draw_bars(false);
}

const REPLY_HANDLERS: [Option<Handler>; 12] = [
    Some(got_command_reply),
    Some(got_workspace_reply),
    Some(got_subscribe_reply),
    Some(got_output_reply),
    None,
    None,
    Some(got_bar_config),
    None,
    None,
    None,
    None,
    None,
];
const EVENT_HANDLERS: [Option<Handler>; 5] = [
    Some(got_workspace_event),
    Some(got_output_event),
    Some(got_mode_event),
    None,
    Some(got_bar_config_update),
];
fn route(message_type: u32) -> Result<Option<Handler>, u32> {
    if message_type & EVENT_BIT != 0 {
        let index = (message_type ^ EVENT_BIT) as usize;
        EVENT_HANDLERS
            .get(index)
            .copied()
            .flatten()
            .map(Some)
            .ok_or(message_type)
    } else {
        REPLY_HANDLERS
            .get(message_type as usize)
            .copied()
            .ok_or(message_type)
    }
}
fn subscriptions(provides_workspaces: bool) -> &'static CStr {
    if provides_workspaces {
        c"[ \"workspace\", \"output\", \"mode\", \"barconfig_update\" ]"
    } else {
        c"[ \"output\", \"mode\", \"barconfig_update\" ]"
    }
}

// A scoped descriptor borrow, never stored in a frame or retained by a handler.
struct Descriptor<'a>(BorrowedFd<'a>);
impl Read for Descriptor<'_> {
    fn read(&mut self, bytes: &mut [u8]) -> io::Result<usize> {
        // SAFETY: The descriptor remains live for this borrow and bytes provides
        // writable storage. This call deliberately does not retry EINTR/EAGAIN.
        let count =
            unsafe { libc::read(self.0.as_raw_fd(), bytes.as_mut_ptr().cast(), bytes.len()) };
        if count < 0 {
            Err(io::Error::last_os_error())
        } else {
            Ok(count as usize)
        }
    }
}
impl Write for Descriptor<'_> {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        loop {
            // SAFETY: bytes is readable and the scoped descriptor is live.
            let count =
                unsafe { libc::write(self.0.as_raw_fd(), bytes.as_ptr().cast(), bytes.len()) };
            if count >= 0 {
                return Ok(count as usize);
            }
            let error = io::Error::last_os_error();
            // swrite retried EAGAIN and EINTR; Write::write_all retries EINTR.
            if error.kind() != io::ErrorKind::WouldBlock {
                return Err(error);
            }
        }
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

struct BarReader<'a, R: ?Sized> {
    reader: &'a mut R,
    received: usize,
    error: Option<io::Error>,
}
impl<R: Read + ?Sized> Read for BarReader<'_, R> {
    fn read(&mut self, bytes: &mut [u8]) -> io::Result<usize> {
        match self.reader.read(bytes) {
            Ok(count) => {
                self.received += count;
                Ok(count)
            }
            Err(error) => {
                // The shared decoder retries payload EINTR. Bar historically
                // treats it as fatal, so suppress that retry and restore the
                // original error after decoding, including its errno/message.
                self.error = Some(error);
                Err(io::Error::from(io::ErrorKind::Other))
            }
        }
    }
}
#[derive(Debug)]
struct FrameError {
    error: ReceiveError,
    payload: bool,
}
fn read_frame(reader: &mut (impl Read + ?Sized)) -> Result<Message, FrameError> {
    let mut reader = BarReader {
        reader,
        received: 0,
        error: None,
    };
    receive_message(&mut reader).map_err(|error| FrameError {
        error: match (error, reader.error.take()) {
            (ReceiveError::Io(_), Some(error)) => ReceiveError::Io(error),
            (error, _) => error,
        },
        payload: reader.received >= 14,
    })
}

struct Connection {
    watcher: ev_io,
    fd: Option<OwnedFd>,
}
static mut CONNECTION: *mut Connection = ptr::null_mut();

/// # Safety
/// libev supplies a live watcher on the event thread. No watcher/FD or graph
/// reference is retained while dispatching handlers that may reenter teardown.
unsafe extern "C" fn got_data(_loop: *mut ev_loop, watcher: *mut ev_io, _events: i32) {
    if config.verbose {
        printf(c"[../i3bar/src/ipc.c:275] Got data!\n".as_ptr());
    }
    let result = {
        let descriptor = BorrowedFd::borrow_raw((*watcher).fd);
        read_frame(&mut Descriptor(descriptor))
    };
    let message = match result {
        Ok(message) => message,
        Err(FrameError {
            error: ReceiveError::Eof | ReceiveError::TruncatedHeader { .. },
            ..
        }) => {
            if config.verbose {
                printf(c"[../i3bar/src/ipc.c:294] EOF received, exiting...\n".as_ptr());
            }
            clean_xcb();
            exit(libc::EXIT_SUCCESS);
        }
        Err(FrameError {
            error: ReceiveError::InvalidMagic(magic),
            ..
        }) => {
            fprintf(
                stderr,
                c"[../i3bar/src/ipc.c:309] ERROR: Wrong magic code: ".as_ptr(),
            );
            let magic = prefix(&magic);
            libc::fwrite(magic.as_ptr().cast(), 1, magic.len(), stderr);
            fprintf(stderr, c"\n Expected: i3-ipc\n".as_ptr());
            exit(libc::EXIT_FAILURE);
        }
        Err(FrameError {
            error: ReceiveError::TruncatedPayload { .. },
            ..
        }) => {
            fprintf(
                stderr,
                c"[../i3bar/src/ipc.c:332] ERROR: Nothing to read!\n".as_ptr(),
            );
            exit(libc::EXIT_FAILURE);
        }
        Err(FrameError {
            error: ReceiveError::Io(error),
            payload,
        }) => {
            let line = if payload { 328 } else { 288 };
            if let Some(errno) = error.raw_os_error() {
                fprintf(
                    stderr,
                    c"[../i3bar/src/ipc.c:%d] ERROR: read() failed: %s\n".as_ptr(),
                    line,
                    libc::strerror(errno),
                );
            } else {
                eprintln!("[../i3bar/src/ipc.c:{line}] ERROR: read() failed: {error}");
            }
            exit(libc::EXIT_FAILURE);
        }
    };
    match route(message.message_type) {
        Ok(Some(handler)) => handler(&message.payload),
        Ok(None) => {}
        Err(message_type) => {
            eprintln!("[../i3bar/src/ipc.c] ERROR: Unsupported IPC message type {message_type}");
            exit(libc::EXIT_FAILURE);
        }
    }
}

/// # Safety
/// payload is null or a live terminated string; the initialized connection is
/// exclusively used by this event thread and cannot close during the write.
pub unsafe fn i3_send_msg(message_type: u32, payload: *const libc::c_char) -> i32 {
    let payload = optional_c_str(payload).map_or(&[][..], CStr::to_bytes);
    let fd = (*CONNECTION).watcher.fd;
    let descriptor = BorrowedFd::borrow_raw(fd);
    if let Err(error) = send_message(&mut Descriptor(descriptor), message_type, payload) {
        *libc::__errno_location() = error.raw_os_error().unwrap_or(libc::EIO);
        i3_common::native_stdio::err(libc::EXIT_FAILURE, c"Failed to write %d".as_ptr(), fd);
    }
    1
}

/// # Safety
/// socket_path is a readable native path. The event thread owns the initialized
/// main_loop and calls this once before using the connection.
pub unsafe fn init_connection(socket_path: *const libc::c_char) {
    let fd = ipc_connect(socket_path);
    let connection = Box::new(Connection {
        watcher: ev_io::new(fd, EV_READ, got_data),
        fd: Some(OwnedFd::from_raw_fd(fd)),
    });
    CONNECTION = Box::into_raw(connection);
    ev_io_start(main_loop, &raw mut (*CONNECTION).watcher);
}
/// # Safety
/// The event thread exclusively owns the registered connection. It must not be
/// borrowed by a reader/writer; libev stop completes before its Box is dropped.
pub unsafe fn destroy_connection() {
    let connection = ptr::replace(&raw mut CONNECTION, ptr::null_mut());
    if connection.is_null() {
        return;
    }
    let mut connection = Box::from_raw(connection);
    // Preserve close-before-stop ordering, while retaining the watcher's stable
    // address until libev has removed it and cleared any pending callback.
    drop(connection.fd.take());
    ev_io_stop(main_loop, &raw mut connection.watcher);
}
/// # Safety
/// The event thread owns live configuration and an initialized IPC connection.
pub unsafe fn subscribe_events() {
    i3_send_msg(
        I3_IPC_MESSAGE_TYPE_SUBSCRIBE,
        subscriptions(i3_provides_workspaces()).as_ptr(),
    );
}
// Internal helpers use Rust linkage rather than foreign symbol lookup.
use crate::child::kill_child;
use crate::child::kill_ws_child;
use crate::child::repeat_last_ws_json;
use crate::child::start_child;
use crate::child::start_ws_child;
use crate::child::status_child_is_alive;
use crate::child::ws_child_is_alive;
use crate::config::parse_config_json;
use crate::config::parse_get_first_i3bar_config;
use crate::mode::parse_mode_json;
use crate::outputs::free_outputs;
use crate::outputs::parse_outputs_json;
use crate::workspaces::free_workspaces;
use crate::workspaces::parse_workspaces_json;
use crate::xcb::clean_xcb;
use crate::xcb::draw_bars;
use crate::xcb::init_xcb_late;
use crate::xcb::reconfig_windows;
use i3_common::native_stdio::stderr;

use crate::application::verboselog;

// Internal calls and globals use canonical Rust identities.
use crate::application::main_loop;
use crate::config::free_colors;
use crate::xcb::init_colors;
use i3_common::ipc_connect::ipc_connect;

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use std::os::fd::AsFd;
    use std::os::unix::net::UnixStream;
    use std::time::Duration;

    fn frame(message_type: u32, payload: &[u8]) -> Vec<u8> {
        let mut bytes = Vec::new();
        send_message(&mut bytes, message_type, payload).unwrap();
        bytes
    }

    #[test]
    fn private_socket_frames_keep_binary_payloads_native_order_and_message_boundaries() {
        let (mut writer, reader) = UnixStream::pair().unwrap();
        reader
            .set_read_timeout(Some(Duration::from_secs(2)))
            .unwrap();
        let first = frame(1, b"one\0\xff");
        assert_eq!(&first[..6], b"i3-ipc");
        assert_eq!(&first[6..10], &5u32.to_ne_bytes());
        assert_eq!(&first[10..14], &1u32.to_ne_bytes());
        let second = frame(EVENT_BIT | 2, b"{}");
        for chunk in first.chunks(3).chain(second.chunks(2)) {
            writer.write_all(chunk).unwrap();
        }
        writer.shutdown(std::net::Shutdown::Write).unwrap();
        struct Fragments<R>(R);
        impl<R: Read> Read for Fragments<R> {
            fn read(&mut self, bytes: &mut [u8]) -> io::Result<usize> {
                let length = bytes.len().min(2);
                self.0.read(&mut bytes[..length])
            }
        }
        let mut reader = Fragments(Descriptor(reader.as_fd()));
        assert_eq!(
            read_frame(&mut reader).unwrap(),
            Message {
                message_type: 1,
                payload: b"one\0\xff".to_vec()
            }
        );
        assert_eq!(
            read_frame(&mut reader).unwrap(),
            Message {
                message_type: EVENT_BIT | 2,
                payload: b"{}".to_vec()
            }
        );
        assert!(matches!(
            read_frame(&mut reader).unwrap_err().error,
            ReceiveError::Eof
        ));
    }

    #[test]
    fn eof_and_invalid_magic_remain_distinct_in_each_receive_phase() {
        let data = frame(2, b"abc");
        for length in 0..14 {
            let failure = read_frame(&mut Cursor::new(&data[..length])).unwrap_err();
            assert!(!failure.payload);
            if length == 0 {
                assert!(matches!(failure.error, ReceiveError::Eof));
            } else {
                assert!(
                    matches!(failure.error,ReceiveError::TruncatedHeader {received} if received == length)
                );
            }
        }
        for length in 14..data.len() {
            let failure = read_frame(&mut Cursor::new(&data[..length])).unwrap_err();
            assert!(failure.payload);
            assert!(
                matches!(failure.error,ReceiveError::TruncatedPayload {received, expected:3} if received == length-14)
            );
        }
        let mut invalid = frame(0, b"");
        invalid[..6].copy_from_slice(b"bad\0\xff!");
        let failure = read_frame(&mut Cursor::new(invalid)).unwrap_err();
        assert!(
            matches!(failure.error,ReceiveError::InvalidMagic(magic) if magic == *b"bad\0\xff!")
        );
    }

    #[test]
    fn bar_read_errors_are_not_retried_in_header_or_payload() {
        struct Interrupted {
            bytes: Cursor<Vec<u8>>,
            fail_after: usize,
            failed: bool,
            errno: i32,
        }
        impl Read for Interrupted {
            fn read(&mut self, bytes: &mut [u8]) -> io::Result<usize> {
                assert!(!self.failed, "bar must not retry read errors");
                let remaining = self.fail_after - self.bytes.position() as usize;
                if remaining == 0 {
                    self.failed = true;
                    return Err(io::Error::from_raw_os_error(self.errno));
                }
                let length = remaining.min(bytes.len());
                self.bytes.read(&mut bytes[..length])
            }
        }
        for fail_after in [3, 16] {
            for errno in [libc::EINTR, libc::EAGAIN, libc::EBADF] {
                let mut reader = Interrupted {
                    bytes: Cursor::new(frame(0, b"payload")),
                    fail_after,
                    failed: false,
                    errno,
                };
                let failure = read_frame(&mut reader).unwrap_err();
                assert_eq!(failure.payload, fail_after >= 14);
                assert!(
                    matches!(failure.error,ReceiveError::Io(error) if error.raw_os_error()==Some(errno))
                );
            }
        }
    }

    #[test]
    fn routing_checks_event_bit_bounds_and_ignored_reply_slots() {
        let reply = route(1).unwrap().unwrap();
        let event = route(EVENT_BIT | 1).unwrap().unwrap();
        assert!(std::ptr::fn_addr_eq(reply, got_workspace_reply as Handler));
        assert!(std::ptr::fn_addr_eq(event, got_output_event as Handler));
        assert!(route(4).unwrap().is_none());
        assert!(route(11).unwrap().is_none());
        assert!(route(12).is_err());
        assert!(route(EVENT_BIT | 3).is_err());
        assert!(route(EVENT_BIT | 5).is_err());
        assert!(route(u32::MAX).is_err());
    }

    #[test]
    fn compact_bar_id_matching_and_subscriptions_keep_legacy_bytes() {
        assert!(matches_bar_id(br#"{"id":"bar-0","mode":"dock"}"#, b"bar-0"));
        assert!(matches_bar_id(br#"{"id":"\xff"}"#, br#"\xff"#));
        assert!(!matches_bar_id(br#"{"id": "bar-0"}"#, b"bar-0"));
        assert!(!matches_bar_id(br#"{"id":"bar-01"}"#, b"bar-0"));
        assert!(!matches_bar_id(b"ignored\0{\"id\":\"bar-0\"}", b"bar-0"));
        assert_eq!(prefix(b"raw\xff\0trailing"), b"raw\xff");
        assert_eq!(
            subscriptions(true).to_bytes(),
            br#"[ "workspace", "output", "mode", "barconfig_update" ]"#
        );
        assert_eq!(
            subscriptions(false).to_bytes(),
            br#"[ "output", "mode", "barconfig_update" ]"#
        );
    }

    #[test]
    fn detached_commands_survive_comparison_and_preserve_null_empty_difference() {
        assert!(!child_needs_restart(true, None, None));
        assert!(child_needs_restart(false, Some(c"same"), Some(c"same")));
        assert!(!child_needs_restart(true, Some(c"same"), Some(c"same")));
        assert!(child_needs_restart(true, None, Some(c"")));
        assert!(child_needs_restart(true, Some(c""), None));
        assert!(child_needs_restart(true, Some(c"old"), Some(c"new")));
        unsafe {
            let mut slot = libc::strdup(c"old\xff".as_ptr());
            assert!(!slot.is_null());
            let command = DetachedCommand::take(&raw mut slot);
            assert!(slot.is_null());
            assert_eq!(command.as_c_str(), Some(c"old\xff"));
            assert!(!child_needs_restart(
                true,
                command.as_c_str(),
                Some(c"old\xff")
            ));
            drop(command);
        }
    }
}
