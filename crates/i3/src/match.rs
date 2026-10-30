use crate::con::arena::{containers, resolve};
use crate::types::FloatingState;
use crate::types::UrgencyMatch;
use crate::types::WindowDock;
use crate::types::WindowMode;
/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * A "match" is a data structure which acts like a mask or expression to match
 * certain windows or not. For example, when using commands, you can specify a
 * command like this: [title="*Firefox*"] kill. The title member of the match
 * data structure will then be filled and i3 will check each window using
 * match_matches_window() to find the windows affected by this command.
 *
 */
// Mechanical file-by-file Rust translation of src/match.c.
// Originally translated with C2Rust 0.22.1; subsequently maintained in Rust.

pub const M_DOCK_BOTTOM: ::core::ffi::c_int = 3;
pub const M_DOCK_TOP: ::core::ffi::c_int = 2;
pub const M_DOCK_ANY: ::core::ffi::c_int = 1;
pub const M_NODOCK: ::core::ffi::c_int = 0;
pub const M_DONTCHECK: ::core::ffi::c_int = -1;

pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

fn empty_match() -> Match {
    Match {
        error: std::ptr::null_mut(),
        title: std::ptr::null_mut(),
        application: std::ptr::null_mut(),
        class: std::ptr::null_mut(),
        instance: std::ptr::null_mut(),
        mark: std::ptr::null_mut(),
        window_role: std::ptr::null_mut(),
        workspace: std::ptr::null_mut(),
        machine: std::ptr::null_mut(),
        window_type: u32::MAX,
        urgent: UrgencyMatch::DontCheck,
        dock: M_NODOCK,
        id: 0,
        window_mode: WindowMode::Any,
        con_id: std::ptr::null_mut(),
        match_all_windows: false,
        insert_where: 0,
        matches: crate::types::MatchMatches {
            tqe_next: std::ptr::null_mut(),
            tqe_prev: std::ptr::null_mut(),
        },
        restart_mode: false,
    }
}

/// # Safety
/// Destination is aligned/writable Match storage. Previously owned expressions
/// and errors must already have been released before reinitialization.
pub unsafe fn match_init(destination: *mut Match) {
    unsafe {
        destination.write(empty_match());
    }
}

fn is_empty(value: &Match) -> bool {
    value.title.is_null()
        && value.mark.is_null()
        && value.application.is_null()
        && value.class.is_null()
        && value.instance.is_null()
        && value.window_role.is_null()
        && value.workspace.is_null()
        && value.machine.is_null()
        && value.urgent == UrgencyMatch::DontCheck
        && value.id == 0
        && value.window_type == u32::MAX
        && value.con_id.is_null()
        && value.dock == M_NODOCK
        && value.window_mode == WindowMode::Any
        && !value.match_all_windows
}

/// # Safety
/// The pointer borrows a live, initialized Match for this call.
pub unsafe fn match_is_empty(value: *mut Match) -> bool {
    is_empty(unsafe { &*value })
}

fn compile_expression(pattern: &CStr) -> Option<Box<Regex>> {
    match Regex::compile(pattern) {
        Ok(expression) => Some(Box::new(expression)),
        Err(error) => {
            emit(
                Level::Error,
                format!(
                    "ERROR: PCRE regular expression compilation failed at {}: {}\n",
                    error.offset, error.message
                )
                .as_bytes(),
            );
            None
        }
    }
}

/// # Safety
/// A non-null pointer borrows a live compiled expression.
unsafe fn copy_expression(pointer: *mut Regex) -> *mut Regex {
    unsafe { pointer.as_ref() }
        .and_then(|expression| compile_expression(expression.pattern()))
        .map_or(std::ptr::null_mut(), Box::into_raw)
}

/// # Safety
/// Source is a live Match whose strings/expressions remain borrowed during the
/// copy. Destination is writable, nonoverlapping, and has no unreleased owners.
/// Queue links retain the legacy shallow-copy convention; owned fields do not.
pub unsafe fn match_copy(destination: *mut Match, source: *mut Match) {
    if destination == source {
        return;
    }
    let mut copy = unsafe { *source };
    copy.title = unsafe { copy_expression(copy.title) };
    copy.mark = unsafe { copy_expression(copy.mark) };
    copy.application = unsafe { copy_expression(copy.application) };
    copy.class = unsafe { copy_expression(copy.class) };
    copy.instance = unsafe { copy_expression(copy.instance) };
    copy.window_role = unsafe { copy_expression(copy.window_role) };
    copy.workspace = unsafe { copy_expression(copy.workspace) };
    copy.machine = unsafe { copy_expression(copy.machine) };
    // Errors have the same malloc/free contract as before, but copied invalid
    // criteria must not share an allocation that both owners later free.
    if !copy.error.is_null() {
        copy.error = unsafe { sstrdup(copy.error) };
    }
    unsafe {
        destination.write(copy);
    }
}

// Borrowed criteria/window values never own or mutate the legacy graph. PCRE
// sees the original C-string bytes, not a lossy UTF-8 conversion.
use crate::regex::Regex;
use i3_common::logging::{emit, Level};
use i3_common::string::I3String;
use std::ffi::{c_char, CStr};

#[derive(Default, Clone, Copy)]
struct TextProperties<'a> {
    class: Option<&'a CStr>,
    instance: Option<&'a CStr>,
    title: Option<&'a I3String>,
    role: Option<&'a CStr>,
    machine: Option<&'a CStr>,
}

struct WindowProperties<'a> {
    text: TextProperties<'a>,
    id: u32,
    window_type: u32,
    dock: WindowDock,
    urgent: (i64, i64),
}

struct Criteria<'a> {
    class: Option<&'a Regex>,
    instance: Option<&'a Regex>,
    title: Option<&'a Regex>,
    role: Option<&'a Regex>,
    machine: Option<&'a Regex>,
    workspace: Option<&'a Regex>,
    mark: Option<&'a Regex>,
    id: u32,
    window_type: u32,
    urgent: UrgencyMatch,
    dock: i32,
    mode: WindowMode,
}

fn focused_pattern(expression: &Regex) -> bool {
    expression.pattern().to_bytes() == b"__focused__"
}

fn regex_matches_bytes(expression: &Regex, bytes: &[u8]) -> bool {
    let mut message = Vec::new();
    match expression.is_match_bytes(bytes) {
        Ok(matched) => {
            message.extend_from_slice(b"Regular expression \"");
            message.extend_from_slice(expression.pattern().to_bytes());
            message.extend_from_slice(if matched {
                b"\" matches \""
            } else {
                b"\" does not match \""
            });
            message.extend_from_slice(bytes);
            message.extend_from_slice(b"\"\n");
            emit(Level::Verbose, &message);
            matched
        }
        Err(error) => {
            message.extend_from_slice(
                format!(
                    "ERROR: PCRE error {} while trying to use regular expression \"",
                    error.code()
                )
                .as_bytes(),
            );
            message.extend_from_slice(expression.pattern().to_bytes());
            message.extend_from_slice(b"\" on input \"");
            message.extend_from_slice(bytes);
            message.extend_from_slice(b"\", see pcreapi(3)\n");
            emit(Level::Error, &message);
            false
        }
    }
}

fn text_matches(
    expression: &Regex,
    value: Option<&CStr>,
    focused: Option<&CStr>,
    label: &[u8],
) -> bool {
    let bytes = value.map_or(&b""[..], CStr::to_bytes);
    let mut message = label.to_vec();
    if focused_pattern(expression) && focused.is_some_and(|focused| focused.to_bytes() == bytes) {
        message.extend_from_slice(if label == b"workspace" {
            b" matches focused workspace\n"
        } else {
            b" matches focused window\n"
        });
    } else if regex_matches_bytes(expression, bytes) {
        message.extend_from_slice(b" matches (");
        message.extend_from_slice(bytes);
        message.extend_from_slice(b")\n");
    } else {
        return false;
    }
    emit(Level::Verbose, &message);
    true
}

fn marks_match(expression: &Regex, names: &[&CStr]) -> bool {
    let matched = names
        .iter()
        .any(|name| regex_matches_bytes(expression, name.to_bytes()));
    emit(
        Level::Verbose,
        if matched {
            b"mark matches\n"
        } else {
            b"mark does not match\n"
        },
    );
    matched
}

#[derive(Default)]
struct UrgencyRange {
    latest: Option<(i64, i64)>,
    oldest_urgent: Option<(i64, i64)>,
}
impl UrgencyRange {
    fn include(&mut self, value: (i64, i64)) {
        self.latest = Some(self.latest.map_or(value, |latest| latest.max(value)));
        if value.0 != 0 {
            self.oldest_urgent = Some(self.oldest_urgent.map_or(value, |oldest| oldest.min(value)));
        }
    }
    fn matches(&self, mode: UrgencyMatch, target: (i64, i64)) -> bool {
        match mode {
            UrgencyMatch::DontCheck => true,
            UrgencyMatch::Latest => {
                target.0 != 0 && self.latest.is_none_or(|value| value <= target)
            }
            UrgencyMatch::Oldest => {
                target.0 != 0 && self.oldest_urgent.is_none_or(|value| value >= target)
            }
        }
    }
}

impl Criteria<'_> {
    fn needs_focused_window(&self) -> bool {
        [
            self.class,
            self.instance,
            self.title,
            self.role,
            self.machine,
        ]
        .into_iter()
        .flatten()
        .any(focused_pattern)
    }

    fn fields_match(&self, window: &WindowProperties<'_>, focused: TextProperties<'_>) -> bool {
        let mut message = format!("Checking window 0x{:08x} (class ", window.id).into_bytes();
        message.extend_from_slice(window.text.class.map_or(&b"(null)"[..], CStr::to_bytes));
        message.extend_from_slice(b")\n");
        emit(Level::Verbose, &message);
        if let Some(expression) = self.class {
            if !text_matches(
                expression,
                window.text.class,
                focused.class,
                b"window class",
            ) {
                return false;
            }
        }
        if let Some(expression) = self.instance {
            if !text_matches(
                expression,
                window.text.instance,
                focused.instance,
                b"window instance",
            ) {
                return false;
            }
        }
        if self.id != 0 {
            if window.id != self.id {
                emit(Level::Verbose, b"window id does not match\n");
                return false;
            }
            emit(
                Level::Verbose,
                format!("match made by window id ({})\n", window.id as i32).as_bytes(),
            );
        }
        if let Some(expression) = self.title {
            let title = match window.text.title.map(I3String::as_cstr).transpose() {
                Ok(title) => title,
                // An unrepresentable title cannot be passed as a null C string
                // to PCRE. Refuse it rather than invoking the old undefined read.
                Err(_) => return false,
            };
            let focused_title = focused.title.and_then(|title| title.as_cstr().ok());
            if !text_matches(expression, title, focused_title, b"window title") {
                return false;
            }
        }
        if let Some(expression) = self.role {
            if !text_matches(
                expression,
                window.text.role,
                focused.role,
                b"window window_role",
            ) {
                return false;
            }
        }
        if self.window_type != u32::MAX {
            if window.window_type != self.window_type {
                return false;
            }
            emit(
                Level::Verbose,
                format!("window_type matches ({})\n", self.window_type as i32).as_bytes(),
            );
        }
        if let Some(expression) = self.machine {
            if !text_matches(
                expression,
                window.text.machine,
                focused.machine,
                b"window machine",
            ) {
                return false;
            }
        }
        true
    }

    fn dock_matches(&self, dock: WindowDock) -> bool {
        if self.dock == M_DONTCHECK {
            return true;
        }
        let matches = matches!(
            (self.dock, dock),
            (M_NODOCK, WindowDock::None)
                | (M_DOCK_TOP, WindowDock::Top)
                | (M_DOCK_BOTTOM, WindowDock::Bottom)
                | (M_DOCK_ANY, WindowDock::Top | WindowDock::Bottom)
        );
        emit(
            Level::Verbose,
            if matches {
                b"dock status matches\n"
            } else {
                b"dock status does not match\n"
            },
        );
        matches
    }

    fn mode_matches(&self, floating: FloatingState, inside_floating: bool) -> bool {
        let matches = match self.mode {
            WindowMode::Any => true,
            WindowMode::TilingAuto => floating == FloatingState::AutoOff,
            WindowMode::TilingUser => floating == FloatingState::UserOff,
            WindowMode::Tiling => !inside_floating,
            WindowMode::FloatingAuto => floating == FloatingState::AutoOn,
            WindowMode::FloatingUser => floating == FloatingState::UserOn,
            WindowMode::Floating => inside_floating,
        };
        if matches {
            emit(Level::Verbose, b"window_mode matches\n");
        }
        matches
    }
}

/// All referenced expressions must remain allocated and immutable for 'a.
unsafe fn criteria_view<'a>(value: &'a Match) -> Criteria<'a> {
    unsafe {
        Criteria {
            class: value.class.as_ref(),
            instance: value.instance.as_ref(),
            title: value.title.as_ref(),
            role: value.window_role.as_ref(),
            machine: value.machine.as_ref(),
            workspace: value.workspace.as_ref(),
            mark: value.mark.as_ref(),
            id: value.id,
            window_type: value.window_type,
            urgent: value.urgent,
            dock: value.dock,
            mode: value.window_mode,
        }
    }
}

/// Every non-null field points to immutable text alive for the borrowed model.
unsafe fn text_properties<'a>(window: &'a Window) -> TextProperties<'a> {
    unsafe {
        TextProperties {
            class: borrowed_text(window.class_class),
            instance: borrowed_text(window.class_instance),
            title: window.name.as_ref(),
            role: borrowed_text(window.role),
            machine: borrowed_text(window.machine),
        }
    }
}

/// The non-null pointer is a live C string with the returned borrow's lifetime.
unsafe fn borrowed_text<'a>(pointer: *const c_char) -> Option<&'a CStr> {
    if pointer.is_null() {
        None
    } else {
        Some(unsafe { CStr::from_ptr(pointer) })
    }
}

/// The event-thread container list and its window models remain live and are
/// not mutated during this traversal. Only value timestamps escape the boundary.
unsafe fn urgency_snapshot() -> UrgencyRange {
    let mut range = UrgencyRange::default();
    for id in containers() {
        let Some(con) = resolve(id) else { continue };
        let container = unsafe { &*con };
        if let Some(window) = unsafe { container.window.as_ref() } {
            range.include((window.urgent.tv_sec, window.urgent.tv_usec));
        }
    }
    range
}

/// # Safety
/// Match/window models, their regexes and text, and the event-thread container
/// graph must be live and immutable for this call. No borrowed view escapes.
/// Like upstream, this window-specific predicate does not add container-id,
/// application, or `all` filtering; outer command selection handles its own gates.
pub unsafe fn match_matches_window(raw_match: *mut Match, raw_window: *mut Window) -> bool {
    let criteria = unsafe { criteria_view(&*raw_match) };
    let window = unsafe { &*raw_window };
    let properties = WindowProperties {
        text: unsafe { text_properties(window) },
        id: window.id,
        window_type: window.window_type,
        dock: window.dock,
        urgent: (window.urgent.tv_sec, window.urgent.tv_usec),
    };
    let focused_text = if criteria.needs_focused_window() {
        unsafe {
            global_focused
                .as_ref()
                .and_then(|container| container.window.as_ref())
                .map(|window| text_properties(window))
                .unwrap_or_default()
        }
    } else {
        TextProperties::default()
    };
    if !criteria.fields_match(&properties, focused_text) {
        return false;
    }
    if criteria.urgent != UrgencyMatch::DontCheck {
        if !unsafe { urgency_snapshot() }.matches(criteria.urgent, properties.urgent) {
            return false;
        }
        emit(
            Level::Verbose,
            if criteria.urgent == UrgencyMatch::Latest {
                b"urgent matches latest\n"
            } else {
                b"urgent matches oldest\n"
            },
        );
    }
    let mut con = std::ptr::null_mut::<Con>();
    if let Some(expression) = criteria.workspace {
        con = unsafe { con_by_window_id(window.id) };
        if con.is_null() {
            return false;
        }
        let workspace = unsafe { con_get_workspace(con).as_ref() };
        let Some(workspace) = workspace else {
            return false;
        };
        let name = unsafe { borrowed_text(workspace.name) };
        let focused_name = if focused_pattern(expression) {
            unsafe {
                con_get_workspace(global_focused)
                    .as_ref()
                    .and_then(|workspace| borrowed_text(workspace.name))
            }
        } else {
            None
        };
        if !text_matches(expression, name, focused_name, b"workspace") {
            return false;
        }
    }
    if !criteria.dock_matches(properties.dock) {
        return false;
    }
    if let Some(expression) = criteria.mark {
        if con.is_null() {
            con = unsafe { con_by_window_id(window.id) };
        }
        let Some(container) = (unsafe { con.as_ref() }) else {
            return false;
        };
        let mut names = Vec::new();
        let mut mark = container.marks_head.tqh_first;
        while let Some(value) = unsafe { mark.as_ref() } {
            if let Some(name) = unsafe { borrowed_text(value.name) } {
                names.push(name);
            }
            mark = value.marks.tqe_next;
        }
        let matched = marks_match(expression, &names);
        if !matched {
            return false;
        }
    }
    if criteria.mode != WindowMode::Any {
        if con.is_null() {
            con = unsafe { con_by_window_id(window.id) };
        }
        let Some(container) = (unsafe { con.as_ref() }) else {
            return false;
        };
        let inside = matches!(criteria.mode, WindowMode::Tiling | WindowMode::Floating)
            && !unsafe { con_inside_floating(con) }.is_null();
        if !criteria.mode_matches(container.floating, inside) {
            return false;
        }
    }
    true
}

/// # Safety
/// The Match is exclusively borrowed and each non-null expression/error is its
/// unique live owner. Match storage/queue links are not freed by this function.
pub unsafe fn match_free(raw: *mut Match) {
    let value = unsafe { &mut *raw };
    let error = std::mem::replace(&mut value.error, std::ptr::null_mut());
    unsafe {
        free(error.cast());
    }
    for slot in [
        &mut value.title,
        &mut value.application,
        &mut value.class,
        &mut value.instance,
        &mut value.mark,
        &mut value.window_role,
        &mut value.workspace,
        &mut value.machine,
    ] {
        let expression = std::mem::replace(slot, std::ptr::null_mut());
        unsafe {
            regex_free(expression);
        }
    }
}

/// Legacy model mutation boundary. Regex/error allocations remain compatible
/// with the other creators and their explicit match_free calls.
/// # Safety
/// Match is exclusively borrowed; ctype is a live C string. cvalue is null only
/// for flag criteria, otherwise a live C string. Global atom/focus state belongs
/// to the calling event thread, and replaced expressions are uniquely owned.
pub unsafe fn match_parse_property(
    match_0: *mut Match,
    ctype: *const ::core::ffi::c_char,
    cvalue: *const ::core::ffi::c_char,
) {
    if match_0 != ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut Match {
    } else {
        __assert_fail(
            c"match != NULL".as_ptr(),
            c"../src/match.c".as_ptr(),
            296 as ::core::ffi::c_uint,
            c"void match_parse_property(Match *, const char *, const char *)".as_ptr(),
        );
    };
    debuglog(
        c"%s:%s:%d - ctype=*%s*, cvalue=*%s*\n".as_ptr().cast_mut(),
        c"../src/match.c".as_ptr(),
        c"match_parse_property".as_ptr(),
        297 as ::core::ffi::c_int,
        ctype,
        cvalue,
    );
    let property = unsafe { CStr::from_ptr(ctype) }.to_bytes();
    if property == b"class" {
        regex_free((*match_0).class);
        (*match_0).class = regex_new(cvalue);
        return;
    }
    if property == b"instance" {
        regex_free((*match_0).instance);
        (*match_0).instance = regex_new(cvalue);
        return;
    }
    if property == b"window_role" {
        regex_free((*match_0).window_role);
        (*match_0).window_role = regex_new(cvalue);
        return;
    }
    if property == b"con_id" {
        if strcmp(cvalue, c"__focused__".as_ptr()) == 0 as ::core::ffi::c_int {
            (*match_0).con_id = global_focused;
            return;
        }
        let mut parsed: ::core::ffi::c_long = 0;
        if !parse_long(cvalue, &raw mut parsed, 0 as ::core::ffi::c_int) {
            errorlog(
                c"ERROR: Could not parse con id \"%s\"\n"
                    .as_ptr()
                    .cast_mut(),
                cvalue,
            );
            (*match_0).error = sstrdup(c"invalid con_id".as_ptr());
        } else {
            (*match_0).con_id = parsed as *mut Con;
            debuglog(
                c"%s:%s:%d - id as int = %p\n".as_ptr().cast_mut(),
                c"../src/match.c".as_ptr(),
                c"match_parse_property".as_ptr(),
                329 as ::core::ffi::c_int,
                (*match_0).con_id,
            );
        }
        return;
    }
    if property == b"id" {
        let mut parsed_0: ::core::ffi::c_long = 0;
        if !parse_long(cvalue, &raw mut parsed_0, 0 as ::core::ffi::c_int) {
            errorlog(
                c"ERROR: Could not parse window id \"%s\"\n"
                    .as_ptr()
                    .cast_mut(),
                cvalue,
            );
            (*match_0).error = sstrdup(c"invalid id".as_ptr());
        } else {
            (*match_0).id = parsed_0 as xcb_window_t;
            debuglog(
                c"%s:%s:%d - window id as int = %d\n".as_ptr().cast_mut(),
                c"../src/match.c".as_ptr(),
                c"match_parse_property".as_ptr(),
                341 as ::core::ffi::c_int,
                (*match_0).id,
            );
        }
        return;
    }
    if property == b"window_type" {
        if strcasecmp(cvalue, c"normal".as_ptr()) == 0 as ::core::ffi::c_int {
            (*match_0).window_type = A__NET_WM_WINDOW_TYPE_NORMAL;
        } else if strcasecmp(cvalue, c"dialog".as_ptr()) == 0 as ::core::ffi::c_int {
            (*match_0).window_type = A__NET_WM_WINDOW_TYPE_DIALOG;
        } else if strcasecmp(cvalue, c"utility".as_ptr()) == 0 as ::core::ffi::c_int {
            (*match_0).window_type = A__NET_WM_WINDOW_TYPE_UTILITY;
        } else if strcasecmp(cvalue, c"toolbar".as_ptr()) == 0 as ::core::ffi::c_int {
            (*match_0).window_type = A__NET_WM_WINDOW_TYPE_TOOLBAR;
        } else if strcasecmp(cvalue, c"splash".as_ptr()) == 0 as ::core::ffi::c_int {
            (*match_0).window_type = A__NET_WM_WINDOW_TYPE_SPLASH;
        } else if strcasecmp(cvalue, c"menu".as_ptr()) == 0 as ::core::ffi::c_int {
            (*match_0).window_type = A__NET_WM_WINDOW_TYPE_MENU;
        } else if strcasecmp(cvalue, c"dropdown_menu".as_ptr()) == 0 as ::core::ffi::c_int {
            (*match_0).window_type = A__NET_WM_WINDOW_TYPE_DROPDOWN_MENU;
        } else if strcasecmp(cvalue, c"popup_menu".as_ptr()) == 0 as ::core::ffi::c_int {
            (*match_0).window_type = A__NET_WM_WINDOW_TYPE_POPUP_MENU;
        } else if strcasecmp(cvalue, c"tooltip".as_ptr()) == 0 as ::core::ffi::c_int {
            (*match_0).window_type = A__NET_WM_WINDOW_TYPE_TOOLTIP;
        } else if strcasecmp(cvalue, c"notification".as_ptr()) == 0 as ::core::ffi::c_int {
            (*match_0).window_type = A__NET_WM_WINDOW_TYPE_NOTIFICATION;
        } else {
            errorlog(
                c"ERROR: unknown window_type value \"%s\"\n"
                    .as_ptr()
                    .cast_mut(),
                cvalue,
            );
            (*match_0).error = sstrdup(c"unknown window_type value".as_ptr());
        }
        return;
    }
    if property == b"con_mark" {
        regex_free((*match_0).mark);
        (*match_0).mark = regex_new(cvalue);
        return;
    }
    if property == b"title" {
        regex_free((*match_0).title);
        (*match_0).title = regex_new(cvalue);
        return;
    }
    if property == b"urgent" {
        if strcasecmp(cvalue, c"latest".as_ptr()) == 0 as ::core::ffi::c_int
            || strcasecmp(cvalue, c"newest".as_ptr()) == 0 as ::core::ffi::c_int
            || strcasecmp(cvalue, c"recent".as_ptr()) == 0 as ::core::ffi::c_int
            || strcasecmp(cvalue, c"last".as_ptr()) == 0 as ::core::ffi::c_int
        {
            (*match_0).urgent = UrgencyMatch::Latest;
        } else if strcasecmp(cvalue, c"oldest".as_ptr()) == 0 as ::core::ffi::c_int
            || strcasecmp(cvalue, c"first".as_ptr()) == 0 as ::core::ffi::c_int
        {
            (*match_0).urgent = UrgencyMatch::Oldest;
        }
        return;
    }
    if property == b"workspace" {
        regex_free((*match_0).workspace);
        (*match_0).workspace = regex_new(cvalue);
        return;
    }
    if property == b"machine" {
        regex_free((*match_0).machine);
        (*match_0).machine = regex_new(cvalue);
        return;
    }
    if property == b"tiling" {
        (*match_0).window_mode = WindowMode::Tiling;
        return;
    }
    if property == b"tiling_from"
        && !cvalue.is_null()
        && strcmp(cvalue, c"auto".as_ptr()) == 0 as ::core::ffi::c_int
    {
        (*match_0).window_mode = WindowMode::TilingAuto;
        return;
    }
    if property == b"tiling_from"
        && !cvalue.is_null()
        && strcmp(cvalue, c"user".as_ptr()) == 0 as ::core::ffi::c_int
    {
        (*match_0).window_mode = WindowMode::TilingUser;
        return;
    }
    if property == b"floating" {
        (*match_0).window_mode = WindowMode::Floating;
        return;
    }
    if property == b"floating_from"
        && !cvalue.is_null()
        && strcmp(cvalue, c"auto".as_ptr()) == 0 as ::core::ffi::c_int
    {
        (*match_0).window_mode = WindowMode::FloatingAuto;
        return;
    }
    if property == b"floating_from"
        && !cvalue.is_null()
        && strcmp(cvalue, c"user".as_ptr()) == 0 as ::core::ffi::c_int
    {
        (*match_0).window_mode = WindowMode::FloatingUser;
        return;
    }
    if property == b"all" {
        (*match_0).match_all_windows = true_0 != 0;
        return;
    }
    errorlog(c"ERROR: Unknown criterion: %s\n".as_ptr().cast_mut(), ctype);
}

use crate::regex::{regex_free, regex_new};

// Internal helpers use Rust linkage rather than foreign symbol lookup.
use crate::util::parse_long;
use i3_common::safewrappers::sstrdup;

// Canonical Cairo backend ABI; renderer logic remains local.

use xcb_sys::xcb_window_t;

use libc::{free, strcasecmp, strcmp};

use i3_common::native_stdio::__assert_fail;

use crate::log::{debuglog, errorlog};

// Shared model types retain field layout while using one Rust identity.
pub use crate::types::Con;
pub use crate::types::Match;
pub use crate::types::Window;

// Internal calls and globals use canonical Rust identities.
use crate::application::A__NET_WM_WINDOW_TYPE_DIALOG;
use crate::application::A__NET_WM_WINDOW_TYPE_DROPDOWN_MENU;
use crate::application::A__NET_WM_WINDOW_TYPE_MENU;
use crate::application::A__NET_WM_WINDOW_TYPE_NORMAL;
use crate::application::A__NET_WM_WINDOW_TYPE_NOTIFICATION;
use crate::application::A__NET_WM_WINDOW_TYPE_POPUP_MENU;
use crate::application::A__NET_WM_WINDOW_TYPE_SPLASH;
use crate::application::A__NET_WM_WINDOW_TYPE_TOOLBAR;
use crate::application::A__NET_WM_WINDOW_TYPE_TOOLTIP;
use crate::application::A__NET_WM_WINDOW_TYPE_UTILITY;
use crate::con::con_by_window_id;
use crate::con::con_get_workspace;
use crate::con::con_inside_floating;
use crate::tree::focused as global_focused;

#[cfg(test)]
mod tests {
    use super::*;

    fn criteria<'a>() -> Criteria<'a> {
        Criteria {
            class: None,
            instance: None,
            title: None,
            role: None,
            machine: None,
            workspace: None,
            mark: None,
            id: 0,
            window_type: u32::MAX,
            urgent: UrgencyMatch::DontCheck,
            dock: M_NODOCK,
            mode: WindowMode::Any,
        }
    }
    fn window() -> WindowProperties<'static> {
        WindowProperties {
            text: TextProperties::default(),
            id: 12,
            window_type: 50,
            dock: WindowDock::None,
            urgent: (0, 0),
        }
    }

    #[test]
    fn focused_sentinel_retains_literal_regex_fallback_and_missing_empty_distinction() {
        let pattern = Regex::compile_str("__focused__").unwrap();
        assert!(text_matches(&pattern, None, Some(c""), b"window class"));
        assert!(!text_matches(&pattern, None, None, b"window class"));
        assert!(text_matches(
            &pattern,
            Some(c"__focused__"),
            None,
            b"window class"
        ));
        assert!(text_matches(
            &pattern,
            Some(c"__focused__"),
            Some(c"different"),
            b"workspace"
        ));
        assert!(!text_matches(
            &pattern,
            Some(c"different"),
            Some(c"__focused__"),
            b"window class"
        ));
        let empty = Regex::compile_str("^$").unwrap();
        assert!(text_matches(&empty, None, None, b"window class"));
        assert!(text_matches(&empty, Some(c""), None, b"window class"));
        let mut criterion = criteria();
        criterion.title = Some(&pattern);
        let properties = window();
        let empty_title = I3String::new("");
        assert!(criterion.fields_match(
            &properties,
            TextProperties {
                title: Some(&empty_title),
                ..TextProperties::default()
            }
        ));
        assert!(!criterion.fields_match(&properties, TextProperties::default()));
    }

    #[test]
    fn class_matching_passes_original_invalid_bytes_to_pcre_without_utf8_repair() {
        // This case deliberately causes a native UTF error; capture it instead
        // of printing an expected ERROR line in an otherwise passing test run.
        i3_common::logging::set_sink(|_, _| {});
        let expression = Regex::compile_str(r"^.$").unwrap();
        let mut criterion = criteria();
        criterion.class = Some(&expression);
        let mut properties = window();
        properties.text.class = Some(c"\xff");
        assert!(!criterion.fields_match(&properties, TextProperties::default()));
        properties.text.class = Some(c"\u{fffd}");
        assert!(criterion.fields_match(&properties, TextProperties::default()));
        let lookbehind = Regex::compile_str(r"(?<=prefix-)\w+$").unwrap();
        criterion.class = Some(&lookbehind);
        properties.text.class = Some(c"prefix-café");
        assert!(criterion.fields_match(&properties, TextProperties::default()));
        i3_common::logging::set_sink(i3_common::logging::stderr_sink);
    }

    #[test]
    fn urgency_extrema_preserve_ties_microseconds_and_nonurgent_zero_seconds() {
        let mut range = UrgencyRange::default();
        range.include((0, 999));
        range.include((10, 1));
        assert!(range.matches(UrgencyMatch::Latest, (10, 1)));
        assert!(range.matches(UrgencyMatch::Oldest, (10, 1)));
        assert!(!range.matches(UrgencyMatch::Latest, (0, 999)));
        range.include((10, 2));
        assert!(!range.matches(UrgencyMatch::Latest, (10, 1)));
        assert!(range.matches(UrgencyMatch::Oldest, (10, 1)));
        range.include((9, 9));
        assert!(!range.matches(UrgencyMatch::Oldest, (10, 1)));
        let mut negative = UrgencyRange::default();
        negative.include((-2, 1));
        negative.include((0, 0));
        assert!(!negative.matches(UrgencyMatch::Latest, (-2, 1)));
        assert!(negative.matches(UrgencyMatch::Oldest, (-2, 1)));
    }

    #[test]
    fn open_dock_numbers_and_mode_containment_keep_their_distinct_rules() {
        let mut criterion = criteria();
        for (raw, expected) in [
            (-1, [true, true, true]),
            (0, [true, false, false]),
            (1, [false, true, true]),
            (2, [false, true, false]),
            (3, [false, false, true]),
            (4, [false, false, false]),
            (i32::MIN, [false, false, false]),
        ] {
            criterion.dock = raw;
            for (dock, expected) in [WindowDock::None, WindowDock::Top, WindowDock::Bottom]
                .into_iter()
                .zip(expected)
            {
                assert_eq!(criterion.dock_matches(dock), expected);
            }
        }
        criterion.mode = WindowMode::TilingAuto;
        assert!(criterion.mode_matches(FloatingState::AutoOff, true));
        criterion.mode = WindowMode::FloatingAuto;
        assert!(criterion.mode_matches(FloatingState::AutoOn, false));
        criterion.mode = WindowMode::Tiling;
        assert!(!criterion.mode_matches(FloatingState::AutoOff, true));
        criterion.mode = WindowMode::Floating;
        assert!(criterion.mode_matches(FloatingState::AutoOff, true));
    }

    #[test]
    fn marks_match_any_name_and_do_not_interpret_the_focused_sentinel() {
        let expression = Regex::compile_str(r"(?<=mark-)\d+").unwrap();
        assert!(!marks_match(&expression, &[]));
        assert!(marks_match(&expression, &[c"unrelated", c"mark-42"]));
        assert!(!marks_match(&expression, &[c"mark-text"]));
        let literal = Regex::compile_str("__focused__").unwrap();
        assert!(marks_match(&literal, &[c"__focused__"]));
        assert!(!marks_match(&literal, &[c"other"]));
    }

    #[test]
    fn copied_match_owns_error_and_regex_after_the_original_is_freed() {
        let mut original = empty_match();
        original.class = Box::into_raw(Box::new(Regex::compile_str(r"(foo)\1").unwrap()));
        original.error = unsafe { sstrdup(c"error".as_ptr()) };
        let mut copy = std::mem::MaybeUninit::uninit();
        unsafe {
            match_copy(copy.as_mut_ptr(), &raw mut original);
        }
        let mut copy = unsafe { copy.assume_init() };
        assert_ne!(copy.class, original.class);
        assert_ne!(copy.error, original.error);
        unsafe {
            match_free(&raw mut original);
        }
        assert!(unsafe { &*copy.class }.is_match_str("foofoo").unwrap());
        assert_eq!(unsafe { CStr::from_ptr(copy.error) }, c"error");
        unsafe {
            match_free(&raw mut copy);
            match_free(&raw mut copy);
        }
        assert!(copy.class.is_null() && copy.error.is_null());
        assert!(is_empty(&empty_match()));
    }
}
