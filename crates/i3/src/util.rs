// Copyright © 2009 Michael Stapelberg and contributors (see LICENSE).
//! Value utilities and explicit boundaries into i3's live tree and native APIs.
use crate::application::{main_loop, start_argv};
use crate::bindings::command_error_nagbar_pid;
use crate::config::config;
use crate::config_parser::config_error_nagbar_pid;
use crate::ipc::ipc_shutdown;
use crate::log::{debuglog, errorlog, get_debug_logging, verboselog};
use crate::manage::restore_geometry;
use crate::tree::croot;
pub use crate::types::Rect;
use crate::types::{Direction, Layout, Orientation, Position, ShutdownReason};
use i3_common::get_process_filename::get_process_filename;
use i3_common::mkdirp::{create_dir_all, report_create_error};
use i3_common::native_stdio::{__ctype_b_loc, warn};
use i3_common::resolve_tilde::resolve_cstring;
use libc::{
    __errno_location, _exit, execvp, fork, free, kill, perror, setlocale, strcasecmp, strcmp,
    strerror, strtol, strtoll, waitpid,
};
use libev_sys::{ev_child, ev_child_start, ev_child_stop, ev_loop};
use std::ffi::{CStr, CString};
use std::io::{Read, Write};
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::OpenOptionsExt;
use std::ptr::NonNull;
const DEFAULT_DIR_MODE: libc::mode_t = 0o755;
const LONG_MIN: libc::c_long = libc::c_long::MIN;
const LONG_MAX: libc::c_long = libc::c_long::MAX;
const true_0: i32 = 1;
const false_0: i32 = 0;

/// Adopt the native restart-state filename until its contents have been copied.
struct MallocBuffer(NonNull<libc::c_char>);
impl MallocBuffer {
    /// ptr must transfer a uniquely owned malloc buffer containing a NUL.
    unsafe fn take(ptr: *mut libc::c_char) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }
    fn as_cstr(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.0.as_ptr()) }
    }
}
impl Drop for MallocBuffer {
    fn drop(&mut self) {
        unsafe {
            free(self.0.as_ptr().cast());
        }
    }
}

pub fn min(a: ::core::ffi::c_int, b: ::core::ffi::c_int) -> ::core::ffi::c_int {
    a.min(b)
}

pub fn max(a: ::core::ffi::c_int, b: ::core::ffi::c_int) -> ::core::ffi::c_int {
    a.max(b)
}

pub fn rect_contains(rect: Rect, x: u32, y: u32) -> bool {
    x >= rect.x
        && x <= rect.x.wrapping_add(rect.width)
        && y >= rect.y
        && y <= rect.y.wrapping_add(rect.height)
}

pub fn rect_add(a: Rect, b: Rect) -> Rect {
    Rect {
        x: a.x.wrapping_add(b.x),
        y: a.y.wrapping_add(b.y),
        width: a.width.wrapping_add(b.width),
        height: a.height.wrapping_add(b.height),
    }
}

pub fn rect_sub(a: Rect, b: Rect) -> Rect {
    Rect {
        x: a.x.wrapping_sub(b.x),
        y: a.y.wrapping_sub(b.y),
        width: a.width.wrapping_sub(b.width),
        height: a.height.wrapping_sub(b.height),
    }
}

pub fn rect_sanitize_dimensions(mut rect: Rect) -> Rect {
    rect.width = if rect.width as i32 <= 0 as i32 {
        1 as u32
    } else {
        rect.width
    };
    rect.height = if rect.height as i32 <= 0 as i32 {
        1 as u32
    } else {
        rect.height
    };
    rect
}

pub fn rect_equals(a: Rect, b: Rect) -> bool {
    a.x == b.x && a.y == b.y && a.width == b.width && a.height == b.height
}

/// Classify workspace bytes using the current native locale.
pub fn name_is_digits(name: &CStr) -> bool {
    // Snapshot locale-sensitive classification, then use a bounded CStr walk.
    // Do not substitute Unicode/ASCII classification for the libc locale table.
    let digits = unsafe { *__ctype_b_loc() };
    name.to_bytes().iter().all(|byte| unsafe {
        *digits.add(*byte as usize) & i3_common::native_stdio::CTYPE_DIGIT as u16 != 0
    })
}

/// Parse a layout name using the current native locale. `default` remains
/// deliberately case sensitive, while named layouts follow libc strcasecmp.
pub fn layout_from_name(name: &CStr) -> Option<Layout> {
    if name == c"default" {
        return Some(Layout::Default);
    }
    [
        (c"stacked", Layout::Stacked),
        (c"stacking", Layout::Stacked),
        (c"tabbed", Layout::Tabbed),
        (c"splitv", Layout::SplitV),
        (c"splith", Layout::SplitH),
    ]
    .into_iter()
    .find_map(|(candidate, layout)| {
        // SAFETY: both CStr references remain live through native comparison.
        (unsafe { strcasecmp(name.as_ptr(), candidate.as_ptr()) } == 0).then_some(layout)
    })
}

/// Parse a nonnegative workspace prefix, retaining native decimal/locale rules.
/// The native errno reset and range result remain observable to callers.
pub fn ws_name_to_number(name: &CStr) -> Option<i32> {
    let mut end = std::ptr::null_mut();
    // SAFETY: the borrowed C string remains live, end is writable, and errno is
    // thread local. Preserve the original decimal prefix and errno behavior.
    let (value, errno) = unsafe {
        *__errno_location() = 0;
        let value = strtoll(name.as_ptr(), &mut end, 10);
        (value, *__errno_location())
    };
    (errno == 0 && value <= i64::from(i32::MAX) && value >= 0 && end != name.as_ptr().cast_mut())
        .then_some(value as i32)
}

/// Store the current value and report whether it changed.
pub fn update_if_necessary(destination: &mut u32, new_value: u32) -> bool {
    std::mem::replace(destination, new_value) != new_value
}

// POSIX dirname handles bare names, roots and trailing slashes differently from
// Path::parent. Keep its byte semantics on an owned writable buffer.
fn parent_directory(path: &CStr) -> CString {
    let mut bytes = path.to_bytes_with_nul().to_vec();
    unsafe { CStr::from_ptr(libc::dirname(bytes.as_mut_ptr().cast())) }.to_owned()
}
fn utility_path(directory: &CStr, name: &CStr) -> CString {
    let mut bytes = directory.to_bytes().to_vec();
    bytes.push(b'/');
    bytes.extend_from_slice(name.to_bytes());
    CString::new(bytes).expect("CStr components contain no NUL")
}

/// Prepared only in the parent: no Rust allocation/filesystem operation is
/// introduced into the fork child's exec path. A readlink failure is deferred
/// until both earlier exec attempts fail, preserving diagnostic order.
struct PreparedUtility {
    name: CString,
    startup_sibling: CString,
    proc_sibling: Result<CString, i32>,
}
impl PreparedUtility {
    fn new(name: &CStr, startup: &CStr) -> Self {
        let proc_sibling = std::fs::read_link("/proc/self/exe")
            .map_err(|error| error.raw_os_error().unwrap_or(libc::EIO))
            .map(|path| {
                let executable =
                    CString::new(path.as_os_str().as_bytes()).expect("OS path contains no NUL");
                utility_path(&parent_directory(&executable), name)
            });
        Self {
            name: name.to_owned(),
            startup_sibling: utility_path(&parent_directory(startup), name),
            proc_sibling,
        }
    }
}
/// argv is writable and NUL-terminated, with live C strings. prepared remains
/// live in the fork child through _exit/exec. Only native exec/failure operations
/// happen here; preparation and allocation were completed before the fork.
unsafe fn exec_i3_utility(prepared: &PreparedUtility, argv: *mut *mut libc::c_char) -> ! {
    argv.write(prepared.name.as_ptr().cast_mut());
    execvp(prepared.name.as_ptr(), argv.cast());
    argv.write(prepared.startup_sibling.as_ptr().cast_mut());
    execvp(prepared.startup_sibling.as_ptr(), argv.cast());
    match &prepared.proc_sibling {
        Ok(sibling) => {
            argv.write(sibling.as_ptr().cast_mut());
            execvp(sibling.as_ptr(), argv.cast());
        }
        Err(errno) => {
            *__errno_location() = *errno;
            warn(c"could not read /proc/self/exe".as_ptr());
            _exit(libc::EXIT_FAILURE);
        }
    }
    warn(c"Could not start %s".as_ptr(), prepared.name.as_ptr());
    _exit(2);
}

fn replace_argument(
    arguments: &mut Vec<CString>,
    option: &CStr,
    value: Option<&CStr>,
    alias: Option<&CStr>,
) {
    let mut original = std::mem::take(arguments).into_iter();
    while let Some(argument) = original.next() {
        if argument.as_c_str() == option || alias == Some(argument.as_c_str()) {
            if value.is_some() {
                let _ = original.next();
            }
        } else {
            arguments.push(argument);
        }
    }
    arguments.push(option.to_owned());
    if let Some(value) = value {
        arguments.push(value.to_owned());
    }
}
/// Clone a native argument array before any tree/shutdown callbacks. The caller
/// supplies a live terminated array and terminated strings through this read.
unsafe fn copy_arguments(mut argv: *const *mut libc::c_char) -> Vec<CString> {
    let mut arguments = Vec::new();
    while !unsafe { *argv }.is_null() {
        arguments.push(unsafe { CStr::from_ptr(*argv) }.to_owned());
        argv = unsafe { argv.add(1) };
    }
    arguments
}

struct NumericLocale(CString);
impl NumericLocale {
    /// Global locale mutation requires external serialization with native
    /// locale users, exactly as the existing restart dump requires.
    unsafe fn c() -> Option<Self> {
        let locale = unsafe { setlocale(libc::LC_NUMERIC, std::ptr::null()) };
        if locale.is_null() {
            return None;
        }
        let previous = unsafe { CStr::from_ptr(locale) }.to_owned();
        unsafe {
            setlocale(libc::LC_NUMERIC, c"C".as_ptr());
        }
        Some(Self(previous))
    }
}
impl Drop for NumericLocale {
    fn drop(&mut self) {
        unsafe {
            setlocale(libc::LC_NUMERIC, self.0.as_ptr());
        }
    }
}
/// Serialize the live event-thread tree while holding the process-locale guard.
unsafe fn restart_layout_payload() -> Vec<u8> {
    let _locale = NumericLocale::c();
    crate::ipc::node_payload(croot, true)
}
unsafe fn store_restart_layout() -> Option<CString> {
    let payload = restart_layout_payload();
    let filename = if let Some(path) = config.restart_state_path.as_deref() {
        resolve_cstring(path)
    } else {
        let filename = MallocBuffer::take(get_process_filename(c"restart-state".as_ptr()))?;
        filename.as_cstr().to_owned()
    };
    let base = parent_directory(filename.as_c_str());
    {
        let diagnostic_5_owner = base.clone();
        let diagnostic_5_bytes = diagnostic_5_owner.as_bytes();
        match (std::str::from_utf8(&diagnostic_5_bytes),) { (Ok(diagnostic_5_text),) => ::log::debug!("../src/util.c:store_restart_layout:253 - Creating \"{}\" for storing the restart layout\n", diagnostic_5_text), _ => emit_parts(Level::Debug, &[b"../src/util.c:store_restart_layout:253 - Creating \"", &diagnostic_5_bytes, b"\" for storing the restart layout\n"]), }
    };
    if let Err(error) = create_dir_all(
        std::ffi::OsStr::from_bytes(base.to_bytes()),
        DEFAULT_DIR_MODE,
    ) {
        let error = report_create_error(std::ffi::OsStr::from_bytes(base.to_bytes()), error);
        *__errno_location() = error.raw_os_error().unwrap_or(libc::EIO);
        {
            let diagnostic_2_owner = base.clone();
            let diagnostic_2_bytes = diagnostic_2_owner.as_bytes();
            match (std::str::from_utf8(&diagnostic_2_bytes),) { (Ok(diagnostic_2_text),) => ::log::error!("ERROR: Could not create \"{}\" for storing the restart layout, layout will be lost.\n", diagnostic_2_text), _ => emit_parts(Level::Error, &[b"ERROR: Could not create \"", &diagnostic_2_bytes, b"\" for storing the restart layout, layout will be lost.\n"]), }
        };
    }
    let path = std::path::Path::new(std::ffi::OsStr::from_bytes(filename.as_c_str().to_bytes()));
    let mut file = match std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .mode(0o600)
        .open(path)
    {
        Ok(file) => file,
        Err(error) => {
            *__errno_location() = error.raw_os_error().unwrap_or(libc::EIO);
            perror(c"open()".as_ptr());
            return None;
        }
    };
    if let Err(error) = file.write_all(&payload) {
        {
            let diagnostic_2_owner = Some(filename.clone());
            let diagnostic_2_bytes = string_bytes(diagnostic_2_owner.as_deref(), None);
            let diagnostic_3_owner = unsafe {
                (strerror(error.raw_os_error().unwrap_or(libc::EIO)))
                    .cast::<libc::c_char>()
                    .as_ref()
                    .map(|value| std::ffi::CStr::from_ptr(value).to_owned())
            };
            let diagnostic_3_bytes = string_bytes(diagnostic_3_owner.as_deref(), None);
            match (
                std::str::from_utf8(&diagnostic_2_bytes),
                std::str::from_utf8(&diagnostic_3_bytes),
            ) {
                (Ok(diagnostic_2_text), Ok(diagnostic_3_text)) => ::log::error!(
                    "ERROR: Could not write restart layout to \"{}\", layout will be lost: {}\n",
                    diagnostic_2_text,
                    diagnostic_3_text
                ),
                _ => emit_parts(
                    Level::Error,
                    &[
                        b"ERROR: Could not write restart layout to \"",
                        &diagnostic_2_bytes,
                        b"\", layout will be lost: ",
                        &diagnostic_3_bytes,
                        b"\n",
                    ],
                ),
            }
        };
        return None;
    }
    drop(file);
    if !payload.is_empty() {
        // The JSON payload is length-delimited, not a C string. Preserve the
        // old int precision cast and stop at the first NUL within that bound.
        let precision = payload.len() as libc::c_int;
        let bytes = &payload[..usize::try_from(precision)
            .unwrap_or(payload.len())
            .min(payload.len())];
        let end = bytes
            .iter()
            .position(|byte| *byte == 0)
            .unwrap_or(bytes.len());
        let bytes = &bytes[..end];
        match std::str::from_utf8(bytes) {
            Ok(layout) => ::log::debug!(
                "../src/util.c:store_restart_layout:276 - layout: {}\n",
                layout
            ),
            Err(_) => emit_parts(
                Level::Debug,
                &[
                    b"../src/util.c:store_restart_layout:276 - layout: ",
                    bytes,
                    b"\n",
                ],
            ),
        };
    }
    Some(filename)
}

/// Live event-thread graph/process boundary. Never call this against a desktop
/// being used for testing: it shuts down IPC, restores geometry and execs i3.
pub unsafe fn i3_restart(forget_layout: bool) {
    let mut arguments = copy_arguments(start_argv);
    let restart_filename = if forget_layout {
        None
    } else {
        store_restart_layout()
    };
    kill_nagbar(config_error_nagbar_pid, true);
    kill_nagbar(command_error_nagbar_pid, true);
    restore_geometry();
    ipc_shutdown(ShutdownReason::Restart, -1);
    {
        let diagnostic_2_owner = unsafe {
            (arguments[0].as_ptr())
                .cast::<libc::c_char>()
                .as_ref()
                .map(|value| std::ffi::CStr::from_ptr(value).to_owned())
        };
        let diagnostic_2_bytes = string_bytes(diagnostic_2_owner.as_deref(), None);
        match (std::str::from_utf8(&diagnostic_2_bytes),) {
            (Ok(diagnostic_2_text),) => ::log::info!("restarting \"{}\"...\n", diagnostic_2_text),
            _ => emit_parts(
                Level::Verbose,
                &[b"restarting \"", &diagnostic_2_bytes, b"\"...\n"],
            ),
        }
    };
    replace_argument(&mut arguments, c"-a", None, None);
    if get_debug_logging() {
        replace_argument(&mut arguments, c"-d", Some(c"all"), None);
    }
    if let Some(filename) = restart_filename.as_ref() {
        replace_argument(
            &mut arguments,
            c"--restart",
            Some(filename.as_c_str()),
            Some(c"-r"),
        );
    }
    let mut pointers: Vec<_> = arguments.iter().map(|argument| argument.as_ptr()).collect();
    pointers.push(std::ptr::null());
    // The startup owner keeps start_argv unchanged. These temporary argv buffers
    // transfer to exec only; on exec failure they are reclaimed without leaving
    // globals pointing into freed storage or leaking intermediate pointer arrays.
    execvp(pointers[0], pointers.as_ptr());
}

unsafe extern "C" fn nagbar_exited(
    loop_0: *mut ev_loop,
    watcher: *mut ev_child,
    revents: ::core::ffi::c_int,
) {
    // Only start_nagbar installs this callback. Stop before taking ownership:
    // libev must no longer retain the stable allocation when it is dropped.
    ev_child_stop(loop_0, watcher);
    let watcher = Box::from_raw(watcher);
    let exitcode = libc::WEXITSTATUS(watcher.rstatus);
    if !libc::WIFEXITED(watcher.rstatus) {
        ::log::error!("ERROR: i3-nagbar ({}) did not exit normally. This is not an error if the config was reloaded while a nagbar was active.\n", ((*watcher).pid) as i32);
    } else if exitcode != 0 as ::core::ffi::c_int {
        ::log::error!(
            "ERROR: i3-nagbar ({}) process exited with status {}\n",
            ((*watcher).pid) as i32,
            (exitcode) as i32
        );
    } else {
        ::log::debug!(
            "../src/util.c:nagbar_exited:348 - i3-nagbar ({}) process exited with status {}\n",
            ((*watcher).pid) as i32,
            (exitcode) as i32
        );
    }
    let nagbar_pid: *mut libc::pid_t = (*watcher).data as *mut libc::pid_t;
    if *nagbar_pid == (*watcher).pid {
        *nagbar_pid = -(1 as ::core::ffi::c_int) as libc::pid_t;
    }
}

/// Event-thread only; nagbar_pid must outlive its registered child watcher.
/// argv is a writable, terminated native argument array valid through exec.
pub unsafe fn start_nagbar(nagbar_pid: *mut libc::pid_t, argv: *mut *mut ::core::ffi::c_char) {
    if *nagbar_pid != -(1 as ::core::ffi::c_int) {
        ::log::debug!("../src/util.c:start_nagbar:368 - i3-nagbar already running (PID {}), not starting again.\n", (*nagbar_pid) as i32);
        return;
    }
    let prepared = PreparedUtility::new(c"i3-nagbar", CStr::from_ptr(*start_argv));
    *nagbar_pid = fork() as libc::pid_t;
    if *nagbar_pid == -(1 as ::core::ffi::c_int) {
        warn(c"Could not fork()".as_ptr());
        return;
    }
    if *nagbar_pid == 0 as ::core::ffi::c_int {
        exec_i3_utility(&prepared, argv);
    }
    ::log::debug!(
        "../src/util.c:start_nagbar:383 - Starting i3-nagbar with PID {}\n",
        (*nagbar_pid) as i32
    );
    let mut child = Box::new(ev_child::new(*nagbar_pid, false, nagbar_exited));
    child.data = nagbar_pid.cast();
    let child = Box::into_raw(child);
    ev_child_start(main_loop, child);
}

/// The caller owns this child PID and any concurrent wait/reap coordination.
pub unsafe fn kill_nagbar(nagbar_pid: libc::pid_t, wait_for_it: bool) {
    if nagbar_pid == -(1 as ::core::ffi::c_int) {
        return;
    }
    if kill(nagbar_pid as libc::pid_t, libc::SIGTERM) == -(1 as ::core::ffi::c_int) {
        warn(c"kill(configerror_nagbar) failed".as_ptr());
    }
    if !wait_for_it {
        return;
    }
    waitpid(
        nagbar_pid as libc::pid_t,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        0 as ::core::ffi::c_int,
    );
}

/// A rejected native integer still carries strtol's parsed value. Some native
/// binding paths log the rejection and continue using its numeric projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseLongError {
    pub value: libc::c_long,
}

/// Retain strtol's locale/base rules, empty-string acceptance, errno behavior
/// and LONG_MIN/MAX rejection, without an output pointer or borrowed raw input.
pub fn parse_long(input: &CStr, base: libc::c_int) -> Result<libc::c_long, ParseLongError> {
    let mut end = std::ptr::null_mut();
    // SAFETY: input is a live native string; end receives a pointer into it (or
    // null for a base rejected by libc). Do not clear/test errno: native i3 did
    // neither here, unlike workspace-prefix parsing above.
    let value = unsafe { strtol(input.as_ptr(), &mut end, base) };
    let trailing = !end.is_null() && unsafe { *end != 0 };
    if value == LONG_MIN || value == LONG_MAX || value < 0 || trailing {
        Err(ParseLongError { value })
    } else {
        Ok(value)
    }
}

#[derive(Debug)]
pub(crate) enum SlurpFailure {
    Open(std::io::Error),
    Metadata(std::io::Error),
    Short { got: usize, want: usize },
}
fn read_sized(reader: &mut impl Read, size: usize) -> Result<Vec<u8>, usize> {
    let mut bytes = vec![0; size];
    let mut count = 0;
    while count < size {
        match reader.read(&mut bytes[count..]) {
            Ok(0) => break,
            Ok(n) => count += n,
            Err(error) if error.kind() == std::io::ErrorKind::Interrupted => continue,
            Err(_) => break,
        }
    }
    if count == size {
        Ok(bytes)
    } else {
        Err(count)
    }
}
fn slurp_file(path: &std::path::Path) -> Result<Vec<u8>, SlurpFailure> {
    let mut file = std::fs::File::open(path).map_err(SlurpFailure::Open)?;
    let metadata = file.metadata().map_err(SlurpFailure::Metadata)?;
    // Keep the old stat-sized snapshot (including empty procfs files), rather
    // than reading a growing file indefinitely. Reject unrepresentable lengths.
    let size = usize::try_from(metadata.len())
        .ok()
        .filter(|size| *size < isize::MAX as usize)
        .ok_or_else(|| SlurpFailure::Metadata(std::io::Error::from_raw_os_error(libc::EFBIG)))?;
    read_sized(&mut file, size).map_err(|got| SlurpFailure::Short { got, want: size })
}
/// Read the stat-sized snapshot, preserving binary bytes and logging the native
/// open/stat/short-read diagnostics. The caller owns the returned payload.
pub(crate) fn slurp(path: &CStr) -> Result<Vec<u8>, SlurpFailure> {
    let path_view = std::path::Path::new(std::ffi::OsStr::from_bytes(path.to_bytes()));
    let error = match slurp_file(path_view) {
        Ok(bytes) => return Ok(bytes),
        Err(error) => error,
    };
    // Snapshot diagnostic bytes before any potentially reentrant logger callback.
    match &error {
        SlurpFailure::Open(error) => {
            let diagnostic_2_owner = path.to_owned();
            let diagnostic_2_bytes = diagnostic_2_owner.as_bytes();
            let diagnostic_3_owner = unsafe {
                (strerror(error.raw_os_error().unwrap_or(libc::EIO)))
                    .cast::<libc::c_char>()
                    .as_ref()
                    .map(|value| std::ffi::CStr::from_ptr(value).to_owned())
            };
            let diagnostic_3_bytes = string_bytes(diagnostic_3_owner.as_deref(), None);
            match (
                std::str::from_utf8(&diagnostic_2_bytes),
                std::str::from_utf8(&diagnostic_3_bytes),
            ) {
                (Ok(diagnostic_2_text), Ok(diagnostic_3_text)) => ::log::error!(
                    "ERROR: Cannot open file \"{}\": {}\n",
                    diagnostic_2_text,
                    diagnostic_3_text
                ),
                _ => emit_parts(
                    Level::Error,
                    &[
                        b"ERROR: Cannot open file \"",
                        &diagnostic_2_bytes,
                        b"\": ",
                        &diagnostic_3_bytes,
                        b"\n",
                    ],
                ),
            }
        }
        SlurpFailure::Metadata(error) => {
            let diagnostic_2_owner = path.to_owned();
            let diagnostic_2_bytes = diagnostic_2_owner.as_bytes();
            let diagnostic_3_owner = unsafe {
                (strerror(error.raw_os_error().unwrap_or(libc::EIO)))
                    .cast::<libc::c_char>()
                    .as_ref()
                    .map(|value| std::ffi::CStr::from_ptr(value).to_owned())
            };
            let diagnostic_3_bytes = string_bytes(diagnostic_3_owner.as_deref(), None);
            match (
                std::str::from_utf8(&diagnostic_2_bytes),
                std::str::from_utf8(&diagnostic_3_bytes),
            ) {
                (Ok(diagnostic_2_text), Ok(diagnostic_3_text)) => ::log::error!(
                    "ERROR: Cannot fstat() \"{}\": {}\n",
                    diagnostic_2_text,
                    diagnostic_3_text
                ),
                _ => emit_parts(
                    Level::Error,
                    &[
                        b"ERROR: Cannot fstat() \"",
                        &diagnostic_2_bytes,
                        b"\": ",
                        &diagnostic_3_bytes,
                        b"\n",
                    ],
                ),
            }
        }
        SlurpFailure::Short { got, want } => {
            let diagnostic_2_owner = path.to_owned();
            let diagnostic_2_bytes = diagnostic_2_owner.as_bytes();
            let diagnostic_3 = (*got) as isize;
            let diagnostic_4 = (*want as libc::c_long) as libc::c_long;
            match (std::str::from_utf8(&diagnostic_2_bytes),) {
                (Ok(diagnostic_2_text),) => ::log::error!(
                    "ERROR: File \"{}\" could not be read entirely: got {}, want {}\n",
                    diagnostic_2_text,
                    diagnostic_3,
                    diagnostic_4
                ),
                _ => emit_parts(
                    Level::Error,
                    &[
                        b"ERROR: File \"",
                        &diagnostic_2_bytes,
                        b"\" could not be read entirely: got ",
                        format!("{}", diagnostic_3).as_bytes(),
                        b", want ",
                        format!("{}", diagnostic_4).as_bytes(),
                        b"\n",
                    ],
                ),
            }
        }
    }
    Err(error)
}

pub fn orientation_from_direction(direction: Direction) -> Orientation {
    match direction {
        Direction::Left | Direction::Right => Orientation::Horizontal,
        Direction::Up | Direction::Down => Orientation::Vertical,
    }
}

pub fn position_from_direction(direction: Direction) -> Position {
    match direction {
        Direction::Left | Direction::Up => Position::Before,
        Direction::Right | Direction::Down => Position::After,
    }
}

pub fn direction_from_orientation_position(
    orientation: Orientation,
    position: Position,
) -> Direction {
    match (orientation, position) {
        (Orientation::Horizontal, Position::Before) => Direction::Left,
        (Orientation::Horizontal, Position::After) => Direction::Right,
        (Orientation::None | Orientation::Vertical, Position::Before) => Direction::Up,
        (Orientation::None | Orientation::Vertical, Position::After) => Direction::Down,
    }
}

pub fn direction_name(direction: Direction) -> &'static str {
    match direction {
        Direction::Left => "left",
        Direction::Right => "right",
        Direction::Up => "up",
        Direction::Down => "down",
    }
}

pub fn position_name(position: Position) -> &'static str {
    match position {
        Position::Before => "before",
        Position::After => "after",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(values: &[&str]) -> Vec<CString> {
        values
            .iter()
            .map(|value| CString::new(*value).unwrap())
            .collect()
    }
    fn bytes(values: &[CString]) -> Vec<&[u8]> {
        values.iter().map(|value| value.to_bytes()).collect()
    }

    #[test]
    fn restart_options_keep_original_order_remove_aliases_and_deduplicate() {
        let mut arguments = args(&[
            "i3",
            "-r",
            "old",
            "--restart",
            "older",
            "-a",
            "--other",
            "x",
        ]);
        replace_argument(&mut arguments, c"--restart", Some(c"new"), Some(c"-r"));
        replace_argument(&mut arguments, c"-a", None, None);
        assert_eq!(
            bytes(&arguments),
            [
                b"i3".as_slice(),
                b"--other",
                b"x",
                b"--restart",
                b"new",
                b"-a"
            ]
        );
        let mut arguments = args(&["i3", "-d", "all", "--debug=all", "-d"]);
        replace_argument(&mut arguments, c"-d", Some(c"new"), None);
        assert_eq!(
            bytes(&arguments),
            [b"i3".as_slice(), b"--debug=all", b"-d", b"new"]
        );
        let mut arguments = vec![CString::new(b"i3\xff".as_slice()).unwrap()];
        replace_argument(&mut arguments, c"-a", None, None);
        assert_eq!(arguments[0].to_bytes(), b"i3\xff");
        let pointers = [
            arguments[0].as_ptr().cast_mut(),
            arguments[1].as_ptr().cast_mut(),
            std::ptr::null_mut(),
        ];
        let copied = unsafe { copy_arguments(pointers.as_ptr()) };
        drop(arguments);
        assert_eq!(bytes(&copied), [b"i3\xff".as_slice(), b"-a"]);
    }

    #[test]
    fn utility_paths_preserve_posix_dirname_and_non_utf8_bytes() {
        for (path, expected) in [
            (c"i3", c"."),
            (c"/", c"/"),
            (c"//", c"//"),
            (c"///", c"/"),
            (c"foo/", c"."),
            (c"foo//bar//", c"foo"),
            (c"/x\xff/i3", c"/x\xff"),
        ] {
            assert_eq!(parent_directory(path).as_c_str(), expected);
        }
        assert_eq!(
            utility_path(c"/x\xff", c"i3\xfe").to_bytes(),
            b"/x\xff/i3\xfe"
        );
        assert_eq!(utility_path(c".", c"/absolute").to_bytes(), b".//absolute");
    }

    #[test]
    fn rectangle_value_helpers_retain_wrapping_and_signed_dimension_rules() {
        let a = Rect {
            x: u32::MAX,
            y: 2,
            width: 0x8000_0000,
            height: 0,
        };
        let b = Rect {
            x: 1,
            y: 3,
            width: 1,
            height: 2,
        };
        let added = rect_add(a, b);
        assert_eq!(
            (added.x, added.y, added.width, added.height),
            (0, 5, 0x8000_0001, 2)
        );
        assert!(rect_equals(rect_sub(added, b), a));
        let sane = rect_sanitize_dimensions(a);
        assert_eq!((sane.width, sane.height), (1, 1));
        let bounds = Rect {
            x: 10,
            y: 20,
            width: 3,
            height: 4,
        };
        assert!(rect_contains(bounds, 13, 24));
        assert!(!rect_contains(bounds, 14, 24));
        assert!(!rect_contains(a, u32::MAX, 2)); // wrapped right edge stays wrapped
    }

    #[test]
    fn native_numeric_helpers_retain_prefix_empty_extrema_and_locale_rules() {
        unsafe {
            for (name, expected) in [
                (c"", -1),
                (c" +3: work", 3),
                (c"-0", 0),
                (c"2147483647 tail", i32::MAX),
                (c"2147483648", -1),
                (c"-1", -1),
                (c"999999999999999999999999", -1),
            ] {
                assert_eq!(ws_name_to_number(name).unwrap_or(-1), expected);
            }
            for (name, expected, value) in [
                (c"", true, 0),
                (c"  +42", true, 42),
                (c"42 ", false, 42),
                (c"-1", false, -1),
                (c"9223372036854775807", false, libc::c_long::MAX),
                (c"0x10", true, 16),
            ] {
                let parsed = parse_long(name, 0);
                assert_eq!(parsed.is_ok(), expected);
                assert_eq!(parsed.unwrap_or_else(|error| error.value), value);
            }
            assert!(name_is_digits(c""));
            assert!(name_is_digits(c"123"));
            for byte in 1..=255u8 {
                let value = CString::new([byte]).unwrap();
                assert_eq!(name_is_digits(&value), libc::isdigit(byte as i32) != 0);
            }
            assert_eq!(layout_from_name(c"DEFAULT"), None);
            assert_eq!(layout_from_name(c"default"), Some(Layout::Default));
            assert_eq!(layout_from_name(c"StAcKiNg"), Some(Layout::Stacked));
        }
    }

    #[test]
    fn numeric_result_keeps_native_base_errno_and_rejected_output() {
        // libc supplies the independent conversion/errno oracle. The expected
        // acceptance records i3's additional trailing/extrema/negative policy.
        let maximum = CString::new(libc::c_long::MAX.to_string()).unwrap();
        let minimum = CString::new(libc::c_long::MIN.to_string()).unwrap();
        for (input, base, valid) in [
            (c"", 10, true),
            (c" ", 10, false),
            (c"+", 10, false),
            (c"010", 0, true),
            (c"010", 10, true),
            (c"0x10", 0, true),
            (c"0x10", 16, true),
            (c"101", 2, true),
            (c"z", 36, true),
            (c"12tail", 10, false),
            (c"-0", 10, true),
            (c"-12tail", 10, false),
            (maximum.as_c_str(), 10, false),
            (minimum.as_c_str(), 10, false),
            (c"999999999999999999999999999999999", 10, false),
            (c"-999999999999999999999999999999999", 10, false),
            // Invalid native bases leave end null and return zero: i3's helper
            // historically accepts that value without interpreting EINVAL.
            (c"12", 1, true),
            (c"12", 37, true),
        ] {
            for initial_errno in [0, libc::E2BIG] {
                let (native, native_errno) = unsafe {
                    *__errno_location() = initial_errno;
                    let native = libc::strtol(input.as_ptr(), std::ptr::null_mut(), base);
                    (native, *__errno_location())
                };
                unsafe { *__errno_location() = initial_errno };
                let parsed = parse_long(input, base);
                assert_eq!(unsafe { *__errno_location() }, native_errno);
                assert_eq!(parsed.is_ok(), valid, "{input:?}, base {base}");
                assert_eq!(parsed.unwrap_or_else(|error| error.value), native);
            }
        }
        // Workspace numbers deliberately reset errno, allow trailing names,
        // and reject overflow outside their signed-32-bit model field.
        unsafe { *__errno_location() = libc::E2BIG };
        assert_eq!(ws_name_to_number(c" +42:name"), Some(42));
        assert_eq!(unsafe { *__errno_location() }, 0);
        assert_eq!(ws_name_to_number(c"2147483648:name"), None);
        assert_eq!(
            ws_name_to_number(c"999999999999999999999999999999999"),
            None
        );
        assert_eq!(unsafe { *__errno_location() }, libc::ERANGE);
    }

    #[test]
    fn sized_reads_retry_interrupts_preserve_embedded_nuls_and_reject_short_data() {
        struct Chunks {
            calls: usize,
            input: std::io::Cursor<Vec<u8>>,
        }
        impl Read for Chunks {
            fn read(&mut self, output: &mut [u8]) -> std::io::Result<usize> {
                self.calls += 1;
                if self.calls == 1 {
                    return Err(std::io::ErrorKind::Interrupted.into());
                }
                let count = output.len().min(2);
                self.input.read(&mut output[..count])
            }
        }
        let mut input = Chunks {
            calls: 0,
            input: std::io::Cursor::new(b"a\0bcdEXTRA".to_vec()),
        };
        assert_eq!(read_sized(&mut input, 5).unwrap(), b"a\0bcd");
        assert_eq!(read_sized(&mut input, 6).unwrap_err(), 5);
        assert_eq!(read_sized(&mut input, 0).unwrap(), b"");
    }

    #[test]
    fn file_snapshot_preserves_binary_bytes_and_stat_size() {
        let directory = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../.work/tmp");
        std::fs::create_dir_all(&directory).unwrap();
        let path = directory.join(format!(
            "util-slurp-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        struct Remove(std::path::PathBuf);
        impl Drop for Remove {
            fn drop(&mut self) {
                let _ = std::fs::remove_file(&self.0);
            }
        }
        let _remove = Remove(path.clone());
        std::fs::write(&path, b"layout\0bytes").unwrap();
        let name = CString::new(path.as_os_str().as_bytes()).unwrap();
        assert_eq!(slurp(&name).unwrap(), b"layout\0bytes");
        assert!(matches!(
            slurp_file(&path.with_extension("missing")),
            Err(SlurpFailure::Open(_))
        ));
        // procfs reports size zero even though an ordinary read returns data.
        let proc_path = std::path::Path::new("/proc/self/status");
        assert!(!std::fs::read(proc_path).unwrap().is_empty());
        assert_eq!(slurp(c"/proc/self/status").unwrap(), b"");
    }
}

use i3_common::logging::{emit_parts, string_bytes, Level};
