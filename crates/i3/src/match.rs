#![deny(unsafe_op_in_unsafe_fn)]

use crate::con::arena::{containers, resolve};
use crate::domains::{DockMatch, MatchInsertWhere};
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

fn empty_match() -> Match {
    Match::empty()
}

fn is_empty(value: &Match) -> bool {
    value.title.is_none()
        && value.mark.is_none()
        && value.application.is_none()
        && value.class.is_none()
        && value.instance.is_none()
        && value.window_role.is_none()
        && value.workspace.is_none()
        && value.machine.is_none()
        && value.urgent == UrgencyMatch::DontCheck
        && value.id == 0
        && value.window_type == u32::MAX
        && value.con_id.is_null()
        && value.dock == DockMatch::NoDock
        && value.window_mode == WindowMode::Any
        && !value.match_all_windows
}

/// # Safety
/// The pointer borrows a live, initialized Match for this call.
pub unsafe fn match_is_empty(value: *mut Match) -> bool {
    is_empty(unsafe { &*value })
}

fn compile_expression(pattern: &CStr) -> Option<Regex> {
    match Regex::compile(pattern) {
        Ok(expression) => Some(expression),
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

// Borrowed criteria/window values never own or mutate the legacy graph. PCRE
// sees the original C-string bytes, not a lossy UTF-8 conversion.
use crate::regex::Regex;
use i3_common::logging::{emit, emit_parts, Level};
use i3_common::string::I3String;
use std::ffi::{c_char, CStr};

#[derive(Default, Clone, Copy)]
struct TextProperties<'a> {
    class: Option<&'a [u8]>,
    instance: Option<&'a [u8]>,
    title: Option<&'a I3String>,
    role: Option<&'a [u8]>,
    machine: Option<&'a [u8]>,
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
    dock: DockMatch,
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
    value: Option<&[u8]>,
    focused: Option<&[u8]>,
    label: &[u8],
) -> bool {
    let bytes = value.unwrap_or(b"");
    let mut message = label.to_vec();
    if focused_pattern(expression) && focused.is_some_and(|focused| focused == bytes) {
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
        message.extend_from_slice(window.text.class.unwrap_or(b"(null)"));
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
            let title = match window.text.title.map(I3String::as_bytes).transpose() {
                Ok(title) => title,
                // An unrepresentable title cannot be passed as a null C string
                // to PCRE. Refuse it rather than invoking the old undefined read.
                Err(_) => return false,
            };
            let focused_title = focused.title.and_then(|title| title.as_bytes().ok());
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
        if self.dock == DockMatch::DontCheck {
            return true;
        }
        let matches = matches!(
            (self.dock, dock),
            (DockMatch::NoDock, WindowDock::None)
                | (DockMatch::Top, WindowDock::Top)
                | (DockMatch::Bottom, WindowDock::Bottom)
                | (DockMatch::Any, WindowDock::Top | WindowDock::Bottom)
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
fn criteria_view(value: &Match) -> Criteria<'_> {
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

/// Text views borrow their Window's owners and cannot outlive the model.
fn text_properties(window: &Window) -> TextProperties<'_> {
    TextProperties {
        class: window.class_class.as_deref(),
        instance: window.class_instance.as_deref(),
        title: window.name.as_ref(),
        role: window.role.as_deref(),
        machine: window.machine.as_deref(),
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
        text: text_properties(window),
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
        let workspace = unsafe { con_get_workspace(con) };
        if workspace.is_null() {
            return false;
        }
        let name = unsafe { (*workspace).name.clone() };
        let focused_name = if focused_pattern(expression) {
            unsafe {
                con_get_workspace(global_focused)
                    .as_ref()
                    .and_then(|workspace| workspace.name.clone())
            }
        } else {
            None
        };
        if !text_matches(
            expression,
            name.as_deref(),
            focused_name.as_deref(),
            b"workspace",
        ) {
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
/// Parser compatibility boundary over an exclusively borrowed owned Match.
/// # Safety
/// Match is exclusively borrowed; ctype is a live C string. cvalue is null only
/// for flag criteria, otherwise a live C string. Global atom/focus state belongs
/// to the calling event thread, and replaced expressions are uniquely owned.
pub unsafe fn match_parse_property(
    match_0: *mut Match,
    ctype: *const ::core::ffi::c_char,
    cvalue: *const ::core::ffi::c_char,
) {
    // SAFETY: this compatibility edge borrows the caller's live C arguments and
    // exclusive Match/event-thread globals for the complete parse. Owned fields
    // are replaced normally; no allocation ownership crosses the raw boundary.
    unsafe {
        if !match_0.is_null() {
        } else {
            __assert_fail(
                c"match != NULL".as_ptr(),
                c"../src/match.c".as_ptr(),
                296 as ::core::ffi::c_uint,
                c"void match_parse_property(Match *, const char *, const char *)".as_ptr(),
            );
        };
        {
            let property = CStr::from_ptr(ctype).to_owned();
            let value = cvalue
                .as_ref()
                .map(|value| CStr::from_ptr(value).to_owned());
            let value_bytes = i3_common::logging::string_bytes(
                (value.as_deref()).map(std::ffi::CStr::to_bytes),
                None,
            );
            match (property.to_str(), std::str::from_utf8(value_bytes)) {
                (Ok(property), Ok(value)) => ::log::debug!(
                    "../src/match.c:match_parse_property:297 - ctype=*{}*, cvalue=*{}*\n",
                    property,
                    value
                ),
                _ => emit_parts(
                    Level::Debug,
                    &[
                        b"../src/match.c:match_parse_property:297 - ctype=*",
                        property.to_bytes(),
                        b"*, cvalue=*",
                        value_bytes,
                        b"*\n",
                    ],
                ),
            }
        };
        let property = CStr::from_ptr(ctype).to_bytes();
        if property == b"class" {
            (*match_0).class = compile_expression(CStr::from_ptr(cvalue));
            return;
        }
        if property == b"instance" {
            (*match_0).instance = compile_expression(CStr::from_ptr(cvalue));
            return;
        }
        if property == b"window_role" {
            (*match_0).window_role = compile_expression(CStr::from_ptr(cvalue));
            return;
        }
        if property == b"con_id" {
            if strcmp(cvalue, c"__focused__".as_ptr()) == 0_i32 {
                (*match_0).con_id = global_focused;
                return;
            }
            match parse_long(CStr::from_ptr(cvalue).to_bytes(), 0) {
                Err(_) => {
                    {
                        let value = CStr::from_ptr(cvalue).to_owned();
                        match value.to_str() {
                            Ok(value) => {
                                ::log::error!("ERROR: Could not parse con id \"{}\"\n", value)
                            }
                            Err(_) => emit_parts(
                                Level::Error,
                                &[
                                    b"ERROR: Could not parse con id \"",
                                    value.to_bytes(),
                                    b"\"\n",
                                ],
                            ),
                        }
                    };
                    (*match_0).error = Some(b"invalid con_id".to_vec());
                }
                Ok(parsed) => {
                    (*match_0).con_id = parsed as *mut Con;
                    ::log::debug!(
                        "../src/match.c:match_parse_property:329 - id as int = {}\n",
                        i3_common::logging::pointer((*match_0).con_id)
                    );
                }
            }
            return;
        }
        if property == b"id" {
            match parse_long(CStr::from_ptr(cvalue).to_bytes(), 0) {
                Err(_) => {
                    {
                        let value = CStr::from_ptr(cvalue).to_owned();
                        match value.to_str() {
                            Ok(value) => {
                                ::log::error!("ERROR: Could not parse window id \"{}\"\n", value)
                            }
                            Err(_) => emit_parts(
                                Level::Error,
                                &[
                                    b"ERROR: Could not parse window id \"",
                                    value.to_bytes(),
                                    b"\"\n",
                                ],
                            ),
                        }
                    };
                    (*match_0).error = Some(b"invalid id".to_vec());
                }
                Ok(parsed_0) => {
                    (*match_0).id = parsed_0 as u32;
                    ::log::debug!(
                        "../src/match.c:match_parse_property:341 - window id as int = {}\n",
                        ((*match_0).id) as i32
                    );
                }
            }
            return;
        }
        if property == b"window_type" {
            if strcasecmp(cvalue, c"normal".as_ptr()) == 0_i32 {
                (*match_0).window_type = A__NET_WM_WINDOW_TYPE_NORMAL;
            } else if strcasecmp(cvalue, c"dialog".as_ptr()) == 0_i32 {
                (*match_0).window_type = A__NET_WM_WINDOW_TYPE_DIALOG;
            } else if strcasecmp(cvalue, c"utility".as_ptr()) == 0_i32 {
                (*match_0).window_type = A__NET_WM_WINDOW_TYPE_UTILITY;
            } else if strcasecmp(cvalue, c"toolbar".as_ptr()) == 0_i32 {
                (*match_0).window_type = A__NET_WM_WINDOW_TYPE_TOOLBAR;
            } else if strcasecmp(cvalue, c"splash".as_ptr()) == 0_i32 {
                (*match_0).window_type = A__NET_WM_WINDOW_TYPE_SPLASH;
            } else if strcasecmp(cvalue, c"menu".as_ptr()) == 0_i32 {
                (*match_0).window_type = A__NET_WM_WINDOW_TYPE_MENU;
            } else if strcasecmp(cvalue, c"dropdown_menu".as_ptr()) == 0_i32 {
                (*match_0).window_type = A__NET_WM_WINDOW_TYPE_DROPDOWN_MENU;
            } else if strcasecmp(cvalue, c"popup_menu".as_ptr()) == 0_i32 {
                (*match_0).window_type = A__NET_WM_WINDOW_TYPE_POPUP_MENU;
            } else if strcasecmp(cvalue, c"tooltip".as_ptr()) == 0_i32 {
                (*match_0).window_type = A__NET_WM_WINDOW_TYPE_TOOLTIP;
            } else if strcasecmp(cvalue, c"notification".as_ptr()) == 0_i32 {
                (*match_0).window_type = A__NET_WM_WINDOW_TYPE_NOTIFICATION;
            } else {
                {
                    let value = CStr::from_ptr(cvalue).to_owned();
                    match value.to_str() {
                        Ok(value) => {
                            ::log::error!("ERROR: unknown window_type value \"{}\"\n", value)
                        }
                        Err(_) => emit_parts(
                            Level::Error,
                            &[
                                b"ERROR: unknown window_type value \"",
                                value.to_bytes(),
                                b"\"\n",
                            ],
                        ),
                    }
                };
                (*match_0).error = Some(b"unknown window_type value".to_vec());
            }
            return;
        }
        if property == b"con_mark" {
            (*match_0).mark = compile_expression(CStr::from_ptr(cvalue));
            return;
        }
        if property == b"title" {
            (*match_0).title = compile_expression(CStr::from_ptr(cvalue));
            return;
        }
        if property == b"urgent" {
            if strcasecmp(cvalue, c"latest".as_ptr()) == 0_i32
                || strcasecmp(cvalue, c"newest".as_ptr()) == 0_i32
                || strcasecmp(cvalue, c"recent".as_ptr()) == 0_i32
                || strcasecmp(cvalue, c"last".as_ptr()) == 0_i32
            {
                (*match_0).urgent = UrgencyMatch::Latest;
            } else if strcasecmp(cvalue, c"oldest".as_ptr()) == 0_i32
                || strcasecmp(cvalue, c"first".as_ptr()) == 0_i32
            {
                (*match_0).urgent = UrgencyMatch::Oldest;
            }
            return;
        }
        if property == b"workspace" {
            (*match_0).workspace = compile_expression(CStr::from_ptr(cvalue));
            return;
        }
        if property == b"machine" {
            (*match_0).machine = compile_expression(CStr::from_ptr(cvalue));
            return;
        }
        if property == b"tiling" {
            (*match_0).window_mode = WindowMode::Tiling;
            return;
        }
        if property == b"tiling_from"
            && !cvalue.is_null()
            && strcmp(cvalue, c"auto".as_ptr()) == 0_i32
        {
            (*match_0).window_mode = WindowMode::TilingAuto;
            return;
        }
        if property == b"tiling_from"
            && !cvalue.is_null()
            && strcmp(cvalue, c"user".as_ptr()) == 0_i32
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
            && strcmp(cvalue, c"auto".as_ptr()) == 0_i32
        {
            (*match_0).window_mode = WindowMode::FloatingAuto;
            return;
        }
        if property == b"floating_from"
            && !cvalue.is_null()
            && strcmp(cvalue, c"user".as_ptr()) == 0_i32
        {
            (*match_0).window_mode = WindowMode::FloatingUser;
            return;
        }
        if property == b"all" {
            (*match_0).match_all_windows = true;
            return;
        }
        {
            let value = CStr::from_ptr(ctype).to_owned();
            match value.to_str() {
                Ok(value) => ::log::error!("ERROR: Unknown criterion: {}\n", value),
                Err(_) => emit_parts(
                    Level::Error,
                    &[b"ERROR: Unknown criterion: ", value.to_bytes(), b"\n"],
                ),
            }
        };
    }
}

// Internal helpers use Rust linkage rather than foreign symbol lookup.
use crate::util::parse_long;

// Canonical Cairo backend ABI; renderer logic remains local.

use libc::{strcasecmp, strcmp};

use i3_common::native_stdio::__assert_fail;

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
            dock: DockMatch::NoDock,
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
        assert!(text_matches(&pattern, None, Some(b""), b"window class"));
        assert!(!text_matches(&pattern, None, None, b"window class"));
        assert!(text_matches(
            &pattern,
            Some(b"__focused__"),
            None,
            b"window class"
        ));
        assert!(text_matches(
            &pattern,
            Some(b"__focused__"),
            Some(b"different"),
            b"workspace"
        ));
        assert!(!text_matches(
            &pattern,
            Some(b"different"),
            Some(b"__focused__"),
            b"window class"
        ));
        let empty = Regex::compile_str("^$").unwrap();
        assert!(text_matches(&empty, None, None, b"window class"));
        assert!(text_matches(&empty, Some(b""), None, b"window class"));
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
        properties.text.class = Some(b"\xff");
        assert!(!criterion.fields_match(&properties, TextProperties::default()));
        properties.text.class = Some("\u{fffd}".as_bytes());
        assert!(criterion.fields_match(&properties, TextProperties::default()));
        let lookbehind = Regex::compile_str(r"(?<=prefix-)\w+$").unwrap();
        criterion.class = Some(&lookbehind);
        properties.text.class = Some("prefix-café".as_bytes());
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
            criterion.dock = DockMatch::from_raw(raw);
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
    fn copied_match_owns_payload_and_resets_intrusive_membership() {
        let mut original = empty_match();
        original.class = Some(Regex::compile_str(r"(foo)\1").unwrap());
        original.error = Some(b"error".to_vec());
        original.matches.tqe_next = std::ptr::NonNull::<Match>::dangling().as_ptr();
        let copied = original.clone();
        assert_ne!(
            copied.class.as_ref().unwrap().pattern().as_ptr(),
            original.class.as_ref().unwrap().pattern().as_ptr()
        );
        assert_ne!(
            copied.error.as_ref().unwrap().as_ptr(),
            original.error.as_ref().unwrap().as_ptr()
        );
        assert!(copied.matches.tqe_next.is_null() && copied.matches.tqe_prev.is_null());
        drop(original);
        assert!(copied
            .class
            .as_ref()
            .unwrap()
            .is_match_str("foofoo")
            .unwrap());
        assert_eq!(copied.error.as_deref(), Some(b"error".as_slice()));
        assert!(is_empty(&empty_match()));
    }
    #[test]
    fn flag_criteria_keep_null_values_in_parser_diagnostics() {
        if std::env::var_os("I333_NULL_CRITERION_PROBE").is_none() {
            let output = std::process::Command::new(std::env::current_exe().unwrap())
                .args([
                    "--exact",
                    "r#match::tests::flag_criteria_keep_null_values_in_parser_diagnostics",
                    "--nocapture",
                ])
                .env("I333_NULL_CRITERION_PROBE", "1")
                .env_remove("DISPLAY")
                .env_remove("I3SOCK")
                .output()
                .unwrap();
            assert!(
                output.status.success(),
                "{}\n{}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
            return;
        }
        static MESSAGES: std::sync::Mutex<Vec<Vec<u8>>> = std::sync::Mutex::new(Vec::new());
        fn capture(level: Level, message: &[u8]) {
            assert_eq!(level, Level::Debug);
            MESSAGES.lock().unwrap().push(message.to_vec());
        }
        i3_common::logging::set_sink(capture);
        for (property, expected) in [
            (c"tiling", WindowMode::Tiling),
            (c"floating", WindowMode::Floating),
        ] {
            let mut parsed = empty_match();
            unsafe {
                match_parse_property(&mut parsed, property.as_ptr(), std::ptr::null());
            }
            assert_eq!(parsed.window_mode, expected);
        }
        let mut parsed = empty_match();
        unsafe {
            match_parse_property(&mut parsed, c"all".as_ptr(), std::ptr::null());
        }
        assert!(parsed.match_all_windows);
        assert_eq!(
            *MESSAGES.lock().unwrap(),
            [
                b"../src/match.c:match_parse_property:297 - ctype=*tiling*, cvalue=*(null)*\n"
                    .to_vec(),
                b"../src/match.c:match_parse_property:297 - ctype=*floating*, cvalue=*(null)*\n"
                    .to_vec(),
                b"../src/match.c:match_parse_property:297 - ctype=*all*, cvalue=*(null)*\n"
                    .to_vec(),
            ]
        );
    }
}
