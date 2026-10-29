// Copyright © 2009 Michael Stapelberg and contributors (see LICENSE).
//! Startup notification contexts and workspace routing. Graph/native callbacks
//! remain event-thread-only; no borrowed graph or sequence list spans a callback.
use crate::application::{
    conn, conn_screen, last_timestamp, listen_fds, main_loop, original_rlimit_core, sndisplay,
    A__NET_STARTUP_ID,
};
use crate::con::{con_by_window_id, con_get_workspace};
use crate::ipc::current_socketpath;
use crate::log::{debuglog, verboselog};
use crate::tree::focused;
use crate::types::Window;
use crate::xcursor::{xcursor_set_root_cursor, Cursor};
use libev_sys::{ev_loop, ev_timer, ev_timer_start, ev_timer_stop};
use startup_notification_sys::*;
use std::ffi::{CStr, CString};
use std::ptr::NonNull;
use xcb_sys::{xcb_get_property_reply_t, xcb_window_t};

struct LauncherRef(NonNull<SnLauncherContext>);
impl LauncherRef {
    /// display must be live on its native event thread.
    unsafe fn new() -> Self {
        Self(
            NonNull::new(unsafe { sn_launcher_context_new(sndisplay, conn_screen) })
                .expect("startup launcher context allocation"),
        )
    }
    fn raw(&self) -> *mut SnLauncherContext {
        self.0.as_ptr()
    }
}
impl Clone for LauncherRef {
    fn clone(&self) -> Self {
        unsafe {
            sn_launcher_context_ref(self.raw());
        }
        Self(self.0)
    }
}
impl Drop for LauncherRef {
    fn drop(&mut self) {
        unsafe {
            sn_launcher_context_unref(self.raw());
        }
    }
}

pub struct Startup_Sequence {
    id: CString,
    workspace: CString,
    context: LauncherRef,
    delete_at: libc::time_t,
}
// Box keeps published sequence/workspace addresses stable until explicit delete.
static mut SEQUENCES: Vec<NonNull<Startup_Sequence>> = Vec::new();
unsafe fn find_sequence(id: &CStr) -> *mut Startup_Sequence {
    (*(&raw const SEQUENCES))
        .iter()
        .copied()
        .find(|sequence| unsafe { &(*sequence.as_ptr()).id } == id)
        .map_or(std::ptr::null_mut(), NonNull::as_ptr)
}

struct StartupTimer {
    watcher: ev_timer,
    context: LauncherRef,
}
unsafe extern "C" fn startup_timeout(loop_: *mut ev_loop, watcher: *mut ev_timer, _: libc::c_int) {
    ev_timer_stop(loop_, watcher);
    let timer = Box::from_raw((*watcher).data.cast::<StartupTimer>());
    let id = CStr::from_ptr(sn_launcher_context_get_startup_id(timer.context.raw())).to_owned();
    debuglog(
        c"%s:%s:%d - Timeout for startup sequence %s\n"
            .as_ptr()
            .cast_mut(),
        c"../src/startup.c".as_ptr(),
        c"startup_timeout".as_ptr(),
        36 as libc::c_int,
        id.as_ptr(),
    );
    if !find_sequence(&id).is_null() {
        // Keep the timer's reference alive through synchronous completion events.
        sn_launcher_context_complete(timer.context.raw());
    } else {
        debuglog(
            c"%s:%s:%d - Sequence already deleted, nevermind.\n"
                .as_ptr()
                .cast_mut(),
            c"../src/startup.c".as_ptr(),
            c"startup_timeout".as_ptr(),
            52 as libc::c_int,
        );
    }
}
fn expired(delete_at: libc::time_t, now: libc::time_t) -> bool {
    delete_at > 0 && now > delete_at
}
unsafe fn prune_startup_sequences() -> usize {
    let now = libc::time(std::ptr::null_mut());
    let pending: Vec<_> = (*(&raw const SEQUENCES))
        .iter()
        .map(|sequence| {
            let ptr = sequence.as_ptr();
            ((*ptr).id.clone(), (*ptr).delete_at)
        })
        .collect();
    let active = pending
        .iter()
        .filter(|(_, deadline)| *deadline == 0)
        .count();
    for (id, deadline) in pending {
        if deadline != 0 && now > deadline {
            let sequence = find_sequence(&id);
            if !sequence.is_null() {
                startup_sequence_delete(sequence);
            }
        }
    }
    active
}
/// sequence is a live pointer from this module. Unlink before releasing its SN
/// reference so reentrant completion callbacks never see a disappearing owner.
pub unsafe fn startup_sequence_delete(sequence: *mut Startup_Sequence) {
    assert!(!sequence.is_null());
    let index = (*(&raw const SEQUENCES))
        .iter()
        .position(|item| item.as_ptr() == sequence);
    if let Some(index) = index {
        let sequence = Box::from_raw((*(&raw mut SEQUENCES)).remove(index).as_ptr());
        debuglog(
            c"%s:%s:%d - Deleting startup sequence %s, delete_at = %lld, current_time = %lld\n"
                .as_ptr()
                .cast_mut(),
            c"../src/startup.c".as_ptr(),
            c"startup_sequence_delete".as_ptr(),
            108 as libc::c_int,
            sequence.id.as_ptr(),
            sequence.delete_at as libc::c_longlong,
            libc::time(std::ptr::null_mut()) as libc::c_longlong,
        );
        drop(sequence);
    }
}
fn first_word(command: &CStr) -> CString {
    let bytes = command.to_bytes();
    CString::new(
        &bytes[..bytes
            .iter()
            .position(|byte| *byte == b' ')
            .unwrap_or(bytes.len())],
    )
    .unwrap()
}
fn window_id_string(window_id: u32) -> CString {
    CString::new((window_id as i32).to_string()).unwrap()
}
struct PreparedCommand {
    command: CString,
    socket: CString,
    window: Option<CString>,
    first_word: CString,
}
impl PreparedCommand {
    fn new(command: &CStr, socket: &CStr, window: u32) -> Self {
        Self {
            command: command.to_owned(),
            socket: socket.to_owned(),
            window: (window != 0).then(|| window_id_string(window)),
            first_word: first_word(command),
        }
    }
}
/// command is live C text; globals/graph/SN display are serialized on the event
/// thread. All Rust-owned preparation occurs before fork. The child uses only
/// the existing native process/environment/SN setup and exec failure boundary.
pub unsafe fn start_application(
    command: *const libc::c_char,
    no_startup_id: bool,
    window_id: xcb_window_t,
) {
    let prepared = PreparedCommand::new(
        CStr::from_ptr(command),
        CStr::from_ptr(current_socketpath),
        window_id,
    );
    let mut context = None;
    if !no_startup_id {
        let launcher = LauncherRef::new();
        sn_launcher_context_set_name(launcher.raw(), c"i3".as_ptr());
        sn_launcher_context_set_description(launcher.raw(), c"exec command in i3".as_ptr());
        sn_launcher_context_initiate(
            launcher.raw(),
            c"i3".as_ptr(),
            prepared.first_word.as_ptr(),
            last_timestamp as libc::c_ulong,
        );
        let id = CStr::from_ptr(sn_launcher_context_get_startup_id(launcher.raw())).to_owned();
        verboselog(c"startup id = %s\n".as_ptr().cast_mut(), id.as_ptr());
        let workspace = CStr::from_ptr((*con_get_workspace(focused)).name).to_owned();
        let sequence = Box::new(Startup_Sequence {
            id,
            workspace,
            context: launcher.clone(),
            delete_at: 0,
        });
        let timer = Box::into_raw(Box::new(StartupTimer {
            watcher: ev_timer::new(60.0, 0.0, startup_timeout),
            context: launcher.clone(),
        }));
        (*timer).watcher.data = timer.cast();
        ev_timer_start(main_loop, &raw mut (*timer).watcher);
        (*(&raw mut SEQUENCES)).push(NonNull::new(Box::into_raw(sequence)).unwrap());
        context = Some(launcher);
    }
    verboselog(
        c"executing: %s\n".as_ptr().cast_mut(),
        prepared.command.as_ptr(),
    );
    // Native descriptor range and context pointer are captured before the fork.
    let fd_end = 3i32.saturating_add(listen_fds);
    let launcher = context
        .as_ref()
        .map_or(std::ptr::null_mut(), LauncherRef::raw);
    if libc::fork() == 0 {
        libc::setsid();
        libc::setrlimit(libc::RLIMIT_CORE, &raw const original_rlimit_core);
        let mut fd = 3;
        while fd < fd_end {
            libc::close(fd);
            fd += 1;
        }
        libc::unsetenv(c"LISTEN_PID".as_ptr());
        libc::unsetenv(c"LISTEN_FDS".as_ptr());
        libc::signal(libc::SIGPIPE, libc::SIG_DFL);
        if !launcher.is_null() {
            sn_launcher_context_setup_child_process(launcher);
        }
        libc::setenv(c"I3SOCK".as_ptr(), prepared.socket.as_ptr(), 1);
        if let Some(window) = prepared.window.as_ref() {
            libc::setenv(c"I3_WINDOW_ID".as_ptr(), window.as_ptr(), 1);
        }
        libc::execl(
            c"/bin/sh".as_ptr(),
            c"/bin/sh".as_ptr(),
            c"-c".as_ptr(),
            prepared.command.as_ptr(),
            std::ptr::null::<libc::c_char>(),
        );
        i3_common::native_stdio::err(libc::EXIT_FAILURE, c"execl return".as_ptr());
    }
    if !no_startup_id {
        xcursor_set_root_cursor(Cursor::Watch);
    }
}

pub unsafe extern "C" fn startup_monitor_event(event: *mut SnMonitorEvent, _: *mut libc::c_void) {
    let native = sn_monitor_event_get_startup_sequence(event);
    let id = CStr::from_ptr(sn_startup_sequence_get_id(native)).to_owned();
    let sequence = find_sequence(&id);
    if sequence.is_null() {
        debuglog(c"%s:%s:%d - Got event for startup sequence that we did not initiate (ID = %s). Ignoring.\n".as_ptr().cast_mut(),
            c"../src/startup.c".as_ptr(),c"startup_monitor_event".as_ptr(),237 as libc::c_int,id.as_ptr());
        return;
    }
    if sn_monitor_event_get_type(event) as u32 == 1 {
        debuglog(
            c"%s:%s:%d - startup sequence %s completed\n"
                .as_ptr()
                .cast_mut(),
            c"../src/startup.c".as_ptr(),
            c"startup_monitor_event".as_ptr(),
            243 as libc::c_int,
            id.as_ptr(),
        );
        (*sequence).delete_at = libc::time(std::ptr::null_mut()).wrapping_add(30);
        if prune_startup_sequences() == 0 {
            xcursor_set_root_cursor(Cursor::Pointer);
        }
    }
}
pub unsafe fn startup_sequence_rename_workspace(
    old_name: *const libc::c_char,
    new_name: *const libc::c_char,
) {
    let old_name = CStr::from_ptr(old_name).to_owned();
    let new_name = CStr::from_ptr(new_name).to_owned();
    for sequence in (*(&raw const SEQUENCES)).iter().copied() {
        let ptr = sequence.as_ptr();
        if (*ptr).workspace == old_name {
            (*ptr).workspace = new_name.clone();
        }
    }
}

struct PropertyReply(*mut xcb_get_property_reply_t);
impl PropertyReply {
    /// Complete malloc-owned XCB reply with its declared wire allocation.
    unsafe fn bytes(&self) -> &[u8] {
        let Some(reply) = (unsafe { self.0.as_ref() }) else {
            return &[];
        };
        let width = match reply.format {
            8 => 1,
            16 => 2,
            32 => 4,
            _ => return &[],
        };
        let Some(len) = (reply.value_len as usize).checked_mul(width) else {
            return &[];
        };
        let Some(wire) = (reply.length as usize).checked_mul(4) else {
            return &[];
        };
        if len > wire || len > isize::MAX as usize {
            return &[];
        }
        unsafe { std::slice::from_raw_parts(self.0.add(1).cast(), len) }
    }
}
impl Drop for PropertyReply {
    fn drop(&mut self) {
        unsafe {
            libc::free(self.0.cast());
        }
    }
}
fn startup_id(bytes: &[u8]) -> CString {
    CString::new(
        &bytes[..bytes
            .iter()
            .position(|byte| *byte == 0)
            .unwrap_or(bytes.len())],
    )
    .unwrap()
}
/// The reply transfers ownership; cwindow and any returned sequence are borrowed
/// from the event-thread graph until a sequence deletion or workspace rename.
pub unsafe fn startup_sequence_get(
    cwindow: *const Window,
    reply: *mut xcb_get_property_reply_t,
    ignore_mapped_leader: bool,
) -> *mut Startup_Sequence {
    let mut reply = PropertyReply(reply);
    if reply.bytes().is_empty() {
        let leader = (*cwindow).leader;
        if leader == 0 || ignore_mapped_leader && !con_by_window_id(leader).is_null() {
            return std::ptr::null_mut();
        }
        let cookie = xcb_sys::xcb_get_property(
            conn,
            0,
            leader,
            A__NET_STARTUP_ID,
            xcb_sys::XCB_GET_PROPERTY_TYPE_ANY,
            0,
            512,
        );
        reply = PropertyReply(xcb_sys::xcb_get_property_reply(
            conn,
            cookie,
            std::ptr::null_mut(),
        ));
        if reply.bytes().is_empty() {
            return std::ptr::null_mut();
        }
    }
    find_sequence(&startup_id(reply.bytes()))
}
pub unsafe fn startup_workspace_for_window(
    cwindow: *const Window,
    reply: *mut xcb_get_property_reply_t,
) -> *mut libc::c_char {
    let sequence = startup_sequence_get(cwindow, reply, false);
    if sequence.is_null() {
        return std::ptr::null_mut();
    }
    if expired((*sequence).delete_at, libc::time(std::ptr::null_mut())) {
        startup_sequence_delete(sequence);
        return std::ptr::null_mut();
    }
    (*sequence).workspace.as_ptr().cast_mut()
}
pub unsafe fn startup_sequence_delete_by_window(window: *const Window) {
    let cookie = xcb_sys::xcb_get_property(
        conn,
        0,
        (*window).id,
        A__NET_STARTUP_ID,
        xcb_sys::XCB_GET_PROPERTY_TYPE_ANY,
        0,
        512,
    );
    let reply = xcb_sys::xcb_get_property_reply(conn, cookie, std::ptr::null_mut());
    let sequence = startup_sequence_get(window, reply, true);
    if !sequence.is_null() {
        startup_sequence_delete(sequence);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn prepared_command_retains_bytes_signed_xid_and_literal_first_space() {
        let command = PreparedCommand::new(c"\xff\tapp arg", c"/private/socket", u32::MAX);
        assert_eq!(command.first_word.to_bytes(), b"\xff\tapp");
        assert_eq!(command.window.unwrap().to_bytes(), b"-1");
        assert_eq!(first_word(c" command").to_bytes(), b"");
        assert_eq!(window_id_string(0x8000_0000).to_bytes(), b"-2147483648");
        assert!(PreparedCommand::new(c"a", c"b", 0).window.is_none());
    }
    #[test]
    fn startup_ids_and_expiry_preserve_prefix_and_strict_deadline() {
        assert_eq!(startup_id(b"\xff\0ignored").to_bytes(), b"\xff");
        assert!(!expired(0, 100));
        assert!(!expired(100, 100));
        assert!(expired(100, 101));
    }
}
