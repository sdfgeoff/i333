// Copyright © 2009 Michael Stapelberg and contributors (see LICENSE).
//! Value utilities and explicit boundaries into i3's live tree and native APIs.
use crate::application::{main_loop, start_argv};
use crate::bindings::command_error_nagbar_pid;
use crate::config::config;
use crate::config_parser::config_error_nagbar_pid;
use crate::ipc::{dump_node, ipc_shutdown};
use crate::log::{debuglog, errorlog, get_debug_logging, verboselog};
use crate::manage::restore_geometry;
use crate::tree::croot;
pub use crate::types::Rect;
use crate::types::{Direction, Layout, Orientation, Position, ShutdownReason};
use i3_common::font::font_is_pango;
use i3_common::get_process_filename::get_process_filename;
use i3_common::mkdirp::mkdirp;
use i3_common::native_stdio::{__ctype_b_loc, warn};
use i3_common::resolve_tilde::resolve_tilde;
use i3_common::safewrappers::smalloc;
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
const INT32_MAX: i32 = i32::MAX;
const LONG_MIN: libc::c_long = libc::c_long::MIN;
const LONG_MAX: libc::c_long = libc::c_long::MAX;
const true_0: i32 = 1;
const false_0: i32 = 0;

/// Final buffers exported to still-native malloc/free callers. Rust temporary
/// strings/vectors never cross that allocation boundary.
struct MallocBuffer(NonNull<libc::c_char>);
impl MallocBuffer {
    fn copy(bytes: &[u8]) -> Self {
        let size = bytes
            .len()
            .checked_add(1)
            .expect("native buffer size overflow");
        let ptr = unsafe { smalloc(size) }.cast::<libc::c_char>();
        unsafe {
            std::ptr::copy_nonoverlapping(bytes.as_ptr(), ptr.cast(), bytes.len());
            ptr.add(bytes.len()).write(0);
        }
        Self(NonNull::new(ptr).expect("smalloc returns nonnull"))
    }
    /// ptr must transfer a uniquely owned malloc buffer containing a NUL.
    unsafe fn take(ptr: *mut libc::c_char) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }
    fn as_cstr(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.0.as_ptr()) }
    }
    fn into_raw(self) -> *mut libc::c_char {
        let ptr = self.0.as_ptr();
        std::mem::forget(self);
        ptr
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

/// name must be a live C string; libc locale changes must be serialized.
pub unsafe fn name_is_digits(name: *const ::core::ffi::c_char) -> bool {
    // Snapshot locale-sensitive classification, then use a bounded CStr walk.
    // Do not substitute Unicode/ASCII classification for the libc locale table.
    let digits = unsafe { *__ctype_b_loc() };
    unsafe { CStr::from_ptr(name) }
        .to_bytes()
        .iter()
        .all(|byte| unsafe {
            *digits.add(*byte as usize) & i3_common::native_stdio::CTYPE_DIGIT as u16 != 0
        })
}

/// Input is a live C string, out is writable; locale changes must be serialized.
pub unsafe fn layout_from_name(layout_str: *const ::core::ffi::c_char, out: *mut Layout) -> bool {
    let name = CStr::from_ptr(layout_str);
    let layout = if name == c"default" {
        Some(Layout::Default)
    } else {
        [
            (c"stacked", Layout::Stacked),
            (c"stacking", Layout::Stacked),
            (c"tabbed", Layout::Tabbed),
            (c"splitv", Layout::SplitV),
            (c"splith", Layout::SplitH),
        ]
        .into_iter()
        .find_map(|(candidate, layout)| {
            (strcasecmp(name.as_ptr(), candidate.as_ptr()) == 0).then_some(layout)
        })
    };
    if let Some(layout) = layout {
        *out = layout;
        true
    } else {
        false
    }
}

/// Input is a live C string. Retains libc prefix/whitespace/errno semantics.
pub unsafe fn ws_name_to_number(name: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    let mut endptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    *__errno_location() = 0 as ::core::ffi::c_int;
    let mut parsed_num: ::core::ffi::c_longlong =
        strtoll(name, &raw mut endptr, 10 as ::core::ffi::c_int);
    if *__errno_location() != 0 as ::core::ffi::c_int
        || parsed_num > INT32_MAX as ::core::ffi::c_longlong
        || parsed_num < 0 as ::core::ffi::c_longlong
        || endptr == name as *mut ::core::ffi::c_char
    {
        parsed_num = -(1 as ::core::ffi::c_int) as ::core::ffi::c_longlong;
    }
    parsed_num as ::core::ffi::c_int
}

/// destination must be live and writable for this event-thread update.
pub unsafe fn update_if_necessary(destination: *mut u32, new_value: u32) -> bool {
    let old_value: u32 = *destination;
    *destination = new_value;
    *destination != old_value
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
/// Live event-thread tree and serialized process-locale contract. dump_node only
/// borrows the generator for its call; Box keeps its native adapter address stable.
unsafe fn restart_layout_payload() -> Vec<u8> {
    let mut generator = Box::new(i3_json::Generator::new());
    {
        let _locale = NumericLocale::c();
        dump_node(
            (&mut *generator as *mut i3_json::Generator).cast(),
            croot,
            true,
        );
    }
    generator.bytes().to_vec()
}
unsafe fn store_restart_layout() -> *mut libc::c_char {
    let payload = restart_layout_payload();
    let filename = if config.restart_state_path.is_null() {
        get_process_filename(c"restart-state".as_ptr())
    } else {
        resolve_tilde(config.restart_state_path)
    };
    let Some(filename) = MallocBuffer::take(filename) else {
        return std::ptr::null_mut();
    };
    let base = parent_directory(filename.as_cstr());
    debuglog(
        c"%s:%s:%d - Creating \"%s\" for storing the restart layout\n"
            .as_ptr()
            .cast_mut(),
        c"../src/util.c".as_ptr(),
        c"store_restart_layout".as_ptr(),
        253 as libc::c_int,
        base.as_ptr(),
    );
    if mkdirp(base.as_ptr(), DEFAULT_DIR_MODE) != 0 {
        errorlog(c"ERROR: Could not create \"%s\" for storing the restart layout, layout will be lost.\n".as_ptr().cast_mut(),base.as_ptr());
    }
    let path = std::path::Path::new(std::ffi::OsStr::from_bytes(filename.as_cstr().to_bytes()));
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
            return std::ptr::null_mut();
        }
    };
    if let Err(error) = file.write_all(&payload) {
        errorlog(
            c"ERROR: Could not write restart layout to \"%s\", layout will be lost: %s\n"
                .as_ptr()
                .cast_mut(),
            filename.as_cstr().as_ptr(),
            strerror(error.raw_os_error().unwrap_or(libc::EIO)),
        );
        return std::ptr::null_mut();
    }
    drop(file);
    if !payload.is_empty() {
        debuglog(
            c"%s:%s:%d - layout: %.*s\n".as_ptr().cast_mut(),
            c"../src/util.c".as_ptr(),
            c"store_restart_layout".as_ptr(),
            276 as libc::c_int,
            payload.len() as libc::c_int,
            payload.as_ptr(),
        );
    }
    filename.into_raw()
}

/// Live event-thread graph/process boundary. Never call this against a desktop
/// being used for testing: it shuts down IPC, restores geometry and execs i3.
pub unsafe fn i3_restart(forget_layout: bool) {
    let mut arguments = copy_arguments(start_argv);
    let restart_filename = if forget_layout {
        None
    } else {
        MallocBuffer::take(store_restart_layout())
    };
    kill_nagbar(config_error_nagbar_pid, true);
    kill_nagbar(command_error_nagbar_pid, true);
    restore_geometry();
    ipc_shutdown(ShutdownReason::Restart, -1);
    verboselog(
        c"restarting \"%s\"...\n".as_ptr().cast_mut(),
        arguments[0].as_ptr(),
    );
    replace_argument(&mut arguments, c"-a", None, None);
    if get_debug_logging() {
        replace_argument(&mut arguments, c"-d", Some(c"all"), None);
    }
    if let Some(filename) = restart_filename.as_ref() {
        replace_argument(
            &mut arguments,
            c"--restart",
            Some(filename.as_cstr()),
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

/// input is null or a uniquely owned malloc-compatible C string. The return
/// value transfers that ownership to the caller, including the non-Pango path.
pub unsafe fn pango_escape_markup(input: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    if !font_is_pango() {
        return input;
    }
    if input.is_null() {
        return input;
    }
    // The renderer receives owned, nul-terminated text. Repair invalid UTF-8
    // through the shared text model, escape in Rust, and retain malloc ownership
    // for the existing caller while it migrates to owned strings.
    let text = i3_common::string::I3String::from_cstr(std::ffi::CStr::from_ptr(input));
    let escaped_text =
        i3_common::string::escape_markup(text.as_str().expect("UTF-8 source is initialized"));
    let escaped =
        i3_common::safewrappers::sstrndup(escaped_text.as_ptr().cast(), escaped_text.len());
    free(input.cast());
    escaped
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
        errorlog(
            c"ERROR: i3-nagbar (%d) did not exit normally. This is not an error if the config was reloaded while a nagbar was active.\n".as_ptr().cast_mut(),
            (*watcher).pid,
        );
    } else if exitcode != 0 as ::core::ffi::c_int {
        errorlog(
            c"ERROR: i3-nagbar (%d) process exited with status %d\n"
                .as_ptr()
                .cast_mut(),
            (*watcher).pid,
            exitcode,
        );
    } else {
        debuglog(
            c"%s:%s:%d - i3-nagbar (%d) process exited with status %d\n"
                .as_ptr()
                .cast_mut(),
            c"../src/util.c".as_ptr(),
            c"nagbar_exited".as_ptr(),
            348 as ::core::ffi::c_int,
            (*watcher).pid,
            exitcode,
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
        debuglog(
            c"%s:%s:%d - i3-nagbar already running (PID %d), not starting again.\n"
                .as_ptr()
                .cast_mut(),
            c"../src/util.c".as_ptr(),
            c"start_nagbar".as_ptr(),
            368 as ::core::ffi::c_int,
            *nagbar_pid,
        );
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
    debuglog(
        c"%s:%s:%d - Starting i3-nagbar with PID %d\n"
            .as_ptr()
            .cast_mut(),
        c"../src/util.c".as_ptr(),
        c"start_nagbar".as_ptr(),
        383 as ::core::ffi::c_int,
        *nagbar_pid,
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

/// Input is a terminated C string, out is writable. Intentionally keeps
/// strtol's locale/base rules, empty-string acceptance and LONG_MIN/MAX rejection.
pub unsafe fn parse_long(
    str: *const ::core::ffi::c_char,
    out: *mut ::core::ffi::c_long,
    base: ::core::ffi::c_int,
) -> bool {
    let mut end: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let result: ::core::ffi::c_long = strtol(str, &raw mut end, base);
    if result == LONG_MIN
        || result == LONG_MAX
        || result < 0 as ::core::ffi::c_long
        || !end.is_null() && *end as ::core::ffi::c_int != '\0' as i32
    {
        *out = result;
        return false_0 != 0;
    }
    *out = result;
    true_0 != 0
}

#[derive(Debug)]
enum SlurpFailure {
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
/// path is a terminated C string and buf is writable. Success transfers a native
/// malloc-owned, NUL-terminated allocation (its payload may contain NULs). As in
/// the original API, open/stat failure leaves buf untouched; short reads clear it.
pub unsafe fn slurp(path: *const libc::c_char, buf: *mut *mut libc::c_char) -> isize {
    let name = CStr::from_ptr(path);
    let path_view = std::path::Path::new(std::ffi::OsStr::from_bytes(name.to_bytes()));
    match slurp_file(path_view) {
        Ok(bytes) => {
            *buf = MallocBuffer::copy(&bytes).into_raw();
            bytes.len() as isize
        }
        Err(SlurpFailure::Open(error)) => {
            errorlog(
                c"ERROR: Cannot open file \"%s\": %s\n".as_ptr().cast_mut(),
                path,
                strerror(error.raw_os_error().unwrap_or(libc::EIO)),
            );
            -1
        }
        Err(SlurpFailure::Metadata(error)) => {
            errorlog(
                c"ERROR: Cannot fstat() \"%s\": %s\n".as_ptr().cast_mut(),
                path,
                strerror(error.raw_os_error().unwrap_or(libc::EIO)),
            );
            -1
        }
        Err(SlurpFailure::Short { got, want }) => {
            errorlog(
                c"ERROR: File \"%s\" could not be read entirely: got %zd, want %li\n"
                    .as_ptr()
                    .cast_mut(),
                path,
                got,
                want as libc::c_long,
            );
            *buf = std::ptr::null_mut();
            -1
        }
    }
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

pub fn direction_to_string(direction: Direction) -> *const ::core::ffi::c_char {
    match direction {
        Direction::Left => c"left".as_ptr(),
        Direction::Right => c"right".as_ptr(),
        Direction::Up => c"up".as_ptr(),
        Direction::Down => c"down".as_ptr(),
    }
}

pub fn position_to_string(position: Position) -> *const ::core::ffi::c_char {
    match position {
        Position::Before => c"before".as_ptr(),
        Position::After => c"after".as_ptr(),
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
                assert_eq!(ws_name_to_number(name.as_ptr()), expected);
            }
            for (name, expected, value) in [
                (c"", true, 0),
                (c"  +42", true, 42),
                (c"42 ", false, 42),
                (c"-1", false, -1),
                (c"9223372036854775807", false, libc::c_long::MAX),
                (c"0x10", true, 16),
            ] {
                let mut output = -9;
                assert_eq!(parse_long(name.as_ptr(), &mut output, 0), expected);
                assert_eq!(output, value);
            }
            assert!(name_is_digits(c"".as_ptr()));
            assert!(name_is_digits(c"123".as_ptr()));
            for byte in 1..=255u8 {
                let value = CString::new([byte]).unwrap();
                assert_eq!(
                    name_is_digits(value.as_ptr()),
                    libc::isdigit(byte as i32) != 0
                );
            }
            let mut layout = Layout::Output;
            assert!(!layout_from_name(c"DEFAULT".as_ptr(), &mut layout));
            assert_eq!(layout, Layout::Output);
            assert!(layout_from_name(c"StAcKiNg".as_ptr(), &mut layout));
            assert_eq!(layout, Layout::Stacked);
        }
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
        let native = MallocBuffer::copy(b"a\0b");
        unsafe {
            assert_eq!(
                std::slice::from_raw_parts(native.0.as_ptr().cast::<u8>(), 4),
                b"a\0b\0"
            );
        }
    }

    #[test]
    fn file_snapshot_uses_owned_file_and_native_buffer_boundary() {
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
        assert_eq!(slurp_file(&path).unwrap(), b"layout\0bytes");
        let path = CString::new(path.as_os_str().as_bytes()).unwrap();
        let mut buffer = std::ptr::null_mut();
        unsafe {
            assert_eq!(slurp(path.as_ptr(), &mut buffer), 12);
            let buffer = MallocBuffer::take(buffer).unwrap();
            assert_eq!(
                std::slice::from_raw_parts(buffer.0.as_ptr().cast::<u8>(), 13),
                b"layout\0bytes\0"
            );
        }
    }
}
