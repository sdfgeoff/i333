use crate::config::config;
use crate::outputs::outputs;
use crate::parse_json_header::parse_json_header;
use i3_common::font::predict_text_width;
use i3_common::safewrappers::sasprintf;
use i3_common::string::i3string_as_utf8;
use i3_common::string::i3string_free;
use i3_common::string::i3string_from_markup_with_length;
use i3_common::string::i3string_from_utf8;
use i3_common::string::i3string_set_markup;
/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3bar - an xcb-based status- and ws-bar for i3
 * © 2010 Axel Wagner and contributors (see also: LICENSE)
 *
 * child.c: Getting input for the statusline
 *
 */
// Mechanical file-by-file Rust translation of i3bar/src/child.c.
// Originally translated with C2Rust 0.22.1; subsequently maintained in Rust.
use i3_common::i333_va_list;
pub use libev_sys::{
    ev_child, ev_child_start, ev_child_stop, ev_io, ev_io_start, ev_io_stop, ev_loop, ev_watcher,
    EV_READ, EV__IOFDSET,
};

pub use i3_common::draw_util::surface_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rect_t {
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub w: ::core::ffi::c_int,
    pub h: ::core::ffi::c_int,
}
pub type rect = rect_t;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct status_block_render_desc {
    pub width: u32,
    pub x_offset: u32,
    pub x_append: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct status_block {
    pub full_text: *mut I3String,
    pub short_text: *mut I3String,
    pub use_short: bool,
    pub render_length: u32,
    pub color: *mut ::core::ffi::c_char,
    pub background: *mut ::core::ffi::c_char,
    pub border: *mut ::core::ffi::c_char,
    pub min_width: u32,
    pub min_width_str: *mut ::core::ffi::c_char,
    pub align: crate::child::BlockAlign,
    pub urgent: bool,
    pub no_separator: bool,
    pub border_top: u32,
    pub border_right: u32,
    pub border_bottom: u32,
    pub border_left: u32,
    pub pango_markup: bool,
    pub sep_block_width: u32,
    pub full_render: status_block_render_desc,
    pub short_render: status_block_render_desc,
    pub name: *mut ::core::ffi::c_char,
    pub instance: *mut ::core::ffi::c_char,
    pub blocks: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub tqe_next: *mut status_block,
    pub tqe_prev: *mut *mut status_block,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct statusline_head {
    pub tqh_first: *mut status_block,
    pub tqh_last: *mut *mut status_block,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct i3bar_child {
    pub pid: libc::pid_t,
    pub version: u32,
    pub stopped: bool,
    pub stop_signal: ::core::ffi::c_int,
    pub cont_signal: ::core::ffi::c_int,
    pub click_events: bool,
    pub click_events_init: bool,
    pub stdin_io: *mut ev_io,
    pub child_sig: *mut ev_child,
    pub stdin_fd: ::core::ffi::c_int,
    pub pending_line: *mut ::core::ffi::c_char,
}

pub use crate::workspaces::Workspace as i3_ws;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub tqe_next: *mut i3_ws,
    pub tqe_prev: *mut *mut i3_ws,
}
pub use crate::outputs::Output as i3_output;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub sle_next: *mut i3_output,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tc_head {
    pub tqh_first: *mut trayclient,
    pub tqh_last: *mut *mut trayclient,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct trayclient {
    pub win: xcb_window_t,
    pub mapped: bool,
    pub xe_version: ::core::ffi::c_int,
    pub class_class: *mut ::core::ffi::c_char,
    pub class_instance: *mut ::core::ffi::c_char,
    pub tailq: C2RustUnnamed_6,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub tqe_next: *mut trayclient,
    pub tqe_prev: *mut *mut trayclient,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ws_head {
    pub tqh_first: *mut i3_ws,
    pub tqh_last: *mut *mut i3_ws,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct outputs_head {
    pub slh_first: *mut i3_output,
}

pub const STDIN_CHUNK_SIZE: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;

pub static mut status_child: i3bar_child = i3bar_child {
    pid: 0 as libc::pid_t,
    version: 0,
    stopped: false,
    stop_signal: 0,
    cont_signal: 0,
    click_events: false,
    click_events_init: false,
    stdin_io: ::core::ptr::null::<ev_io>() as *mut ev_io,
    child_sig: ::core::ptr::null::<ev_child>() as *mut ev_child,
    stdin_fd: 0,
    pending_line: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
};

pub static mut ws_child: i3bar_child = i3bar_child {
    pid: 0 as libc::pid_t,
    version: 0,
    stopped: false,
    stop_signal: 0,
    cont_signal: 0,
    click_events: false,
    click_events_init: false,
    stdin_io: ::core::ptr::null::<ev_io>() as *mut ev_io,
    child_sig: ::core::ptr::null::<ev_child>() as *mut ev_child,
    stdin_fd: 0,
    pending_line: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
};

pub static mut statusline_head: statusline_head = statusline_head {
    tqh_first: ::core::ptr::null::<status_block>() as *mut status_block,
    tqh_last: ::core::ptr::null::<*mut status_block>() as *mut *mut status_block,
};

thread_local! {
    static CLICK_GENERATOR: std::cell::RefCell<i3_json::Generator> = std::cell::RefCell::new(i3_json::Generator::new());
}

pub static mut child_stdin: ::core::ffi::c_int = 0;

pub unsafe fn clear_statusline(head: *mut statusline_head, free_resources: bool) {
    let mut first: *mut status_block = ::core::ptr::null_mut::<status_block>();
    while !(*head).tqh_first.is_null() {
        first = (*head).tqh_first;
        if free_resources {
            i3string_free((*first).full_text);
            (*first).full_text = ::core::ptr::null_mut::<I3String>();
            i3string_free((*first).short_text);
            (*first).short_text = ::core::ptr::null_mut::<I3String>();
            free((*first).color as *mut ::core::ffi::c_void);
            (*first).color = ::core::ptr::null_mut::<::core::ffi::c_char>();
            free((*first).name as *mut ::core::ffi::c_void);
            (*first).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
            free((*first).instance as *mut ::core::ffi::c_void);
            (*first).instance = ::core::ptr::null_mut::<::core::ffi::c_char>();
            free((*first).min_width_str as *mut ::core::ffi::c_void);
            (*first).min_width_str = ::core::ptr::null_mut::<::core::ffi::c_char>();
            free((*first).background as *mut ::core::ffi::c_void);
            (*first).background = ::core::ptr::null_mut::<::core::ffi::c_char>();
            free((*first).border as *mut ::core::ffi::c_void);
            (*first).border = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        if !(*first).blocks.tqe_next.is_null() {
            (*(*first).blocks.tqe_next).blocks.tqe_prev = (*first).blocks.tqe_prev;
        } else {
            (*head).tqh_last = (*first).blocks.tqe_prev;
        }
        *(*first).blocks.tqe_prev = (*first).blocks.tqe_next;
        free(first as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C" fn set_statusline_error(format: *const ::core::ffi::c_char, args: ...) {
    let mut err_block: *mut status_block = ::core::ptr::null_mut::<status_block>();
    let mut message_block: *mut status_block = ::core::ptr::null_mut::<status_block>();
    clear_statusline(&raw mut statusline_head, true);
    let mut message: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut args_0;
    args_0 = args.clone();
    if !(vasprintf(&raw mut message, format, i333_va_list!(args_0)) == -(1 as ::core::ffi::c_int)) {
        err_block = scalloc(1 as usize, ::core::mem::size_of::<status_block>() as usize)
            as *mut status_block;
        (*err_block).full_text = i3string_from_utf8(c"Error: ".as_ptr());
        (*err_block).name = sstrdup(c"error".as_ptr());
        (*err_block).color = sstrdup(c"#ff0000".as_ptr());
        (*err_block).no_separator = true;
        message_block = scalloc(1 as usize, ::core::mem::size_of::<status_block>() as usize)
            as *mut status_block;
        (*message_block).full_text = i3string_from_utf8(message);
        (*message_block).name = sstrdup(c"error_message".as_ptr());
        (*message_block).color = sstrdup(c"#ff0000".as_ptr());
        (*message_block).no_separator = true;
        (*err_block).blocks.tqe_next = statusline_head.tqh_first;
        if !(*err_block).blocks.tqe_next.is_null() {
            (*statusline_head.tqh_first).blocks.tqe_prev = &raw mut (*err_block).blocks.tqe_next;
        } else {
            statusline_head.tqh_last = &raw mut (*err_block).blocks.tqe_next;
        }
        statusline_head.tqh_first = err_block;
        (*err_block).blocks.tqe_prev = &raw mut statusline_head.tqh_first;
        (*message_block).blocks.tqe_next = ::core::ptr::null_mut::<status_block>();
        (*message_block).blocks.tqe_prev = statusline_head.tqh_last;
        *statusline_head.tqh_last = message_block;
        statusline_head.tqh_last = &raw mut (*message_block).blocks.tqe_next;
    }
    free(message as *mut ::core::ffi::c_void);
}
unsafe fn cleanup(c: *mut i3bar_child) {
    if (*c).pid == 0 as ::core::ffi::c_int {
        if config.verbose {
            printf(
                c"[%s:%d] %s: child pid = 0\n".as_ptr(),
                c"../i3bar/src/child.c".as_ptr(),
                158 as ::core::ffi::c_int,
                c"cleanup".as_ptr(),
            );
        }
    } else if (*c).pid == status_child.pid {
        if config.verbose {
            printf(
                c"[%s:%d] %s: status_command: pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n".as_ptr(),
                c"../i3bar/src/child.c".as_ptr(),
                158 as ::core::ffi::c_int,
                c"cleanup".as_ptr(),
                (*c).pid as ::core::ffi::c_long,
                (*c).stopped as ::core::ffi::c_int,
                (*c).stop_signal,
                (*c).cont_signal,
                (*c).click_events as ::core::ffi::c_int,
                (*c).click_events_init as ::core::ffi::c_int,
            );
        }
    } else if (*c).pid == ws_child.pid {
        if config.verbose {
            printf(
                c"[%s:%d] %s: workspace_command: pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n".as_ptr(),
                c"../i3bar/src/child.c".as_ptr(),
                158 as ::core::ffi::c_int,
                c"cleanup".as_ptr(),
                (*c).pid as ::core::ffi::c_long,
                (*c).stopped as ::core::ffi::c_int,
                (*c).stop_signal,
                (*c).cont_signal,
                (*c).click_events as ::core::ffi::c_int,
                (*c).click_events_init as ::core::ffi::c_int,
            );
        }
    } else {
        fprintf(
            stderr,
            c"[%s:%d] ERROR: %s: unknown child, this should never happen pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n".as_ptr(),
            c"../i3bar/src/child.c".as_ptr(),
            158 as ::core::ffi::c_int,
            c"cleanup".as_ptr(),
            (*c).pid as ::core::ffi::c_long,
            (*c).stopped as ::core::ffi::c_int,
            (*c).stop_signal,
            (*c).cont_signal,
            (*c).click_events as ::core::ffi::c_int,
            (*c).click_events_init as ::core::ffi::c_int,
        );
    }
    if !(*c).stdin_io.is_null() {
        ev_io_stop(main_loop as *mut ev_loop, (*c).stdin_io);
        free((*c).stdin_io as *mut ::core::ffi::c_void);
        (*c).stdin_io = ::core::ptr::null_mut::<ev_io>();
        if (*c).pid == status_child.pid {
            close(child_stdin);
            child_stdin = 0 as ::core::ffi::c_int;
        }
        close((*c).stdin_fd);
    }
    if !(*c).child_sig.is_null() {
        ev_child_stop(main_loop as *mut ev_loop, (*c).child_sig);
        free((*c).child_sig as *mut ::core::ffi::c_void);
        (*c).child_sig = ::core::ptr::null_mut::<ev_child>();
    }
    if c == &raw mut ws_child {
        WORKSPACE_LINES.with(|state| state.borrow_mut().pending = None);
    }
    free((*c).pending_line as *mut ::core::ffi::c_void);
    (*c).pending_line = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if c == &raw mut status_child {
        STATUS_PARSER.with(|parser| *parser.borrow_mut() = None);
        CLICK_GENERATOR.with(|generator| *generator.borrow_mut() = i3_json::Generator::new());
    }
    memset(
        c as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<i3bar_child>() as usize,
    );
}
#[derive(Clone)]
struct StatusValue {
    full: Option<Vec<u8>>,
    short: Option<Vec<u8>>,
    color: Option<Vec<u8>>,
    background: Option<Vec<u8>>,
    border: Option<Vec<u8>>,
    minimum_text: Option<Vec<u8>>,
    name: Option<Vec<u8>>,
    instance: Option<Vec<u8>>,
    minimum: u32,
    separator: u32,
    align: BlockAlign,
    urgent: bool,
    no_separator: bool,
    borders: [u32; 4],
    markup: bool,
}
impl StatusValue {
    fn new(separator: u32) -> Self {
        Self {
            full: None,
            short: None,
            color: None,
            background: None,
            border: None,
            minimum_text: None,
            name: None,
            instance: None,
            minimum: 0,
            separator,
            align: BlockAlign::Left,
            urgent: false,
            no_separator: false,
            borders: [1; 4],
            markup: false,
        }
    }
}
fn first_nul(bytes: &[u8]) -> &[u8] {
    bytes.split(|&byte| byte == 0).next().unwrap_or_default()
}
fn key_is(key: &[u8], expected: &[u8]) -> bool {
    key.len() == expected.len()
        && key.iter().zip(expected).all(|(&a, &b)| {
            // SAFETY: unsigned-byte values satisfy tolower's input contract.
            unsafe { libc::tolower(i32::from(a)) == libc::tolower(i32::from(b)) }
        })
}
struct StatusValues {
    key: Option<Vec<u8>>,
    block: StatusValue,
    buffer: Vec<StatusValue>,
    frames: Vec<Vec<StatusValue>>,
    has_urgent: bool,
    separator: u32,
    error: Option<String>,
}
impl i3_json::EventHandler for StatusValues {
    fn on_event(&mut self, event: i3_json::Event<'_>) -> bool {
        use i3_json::Event;
        match event {
            Event::StartArray => self.buffer.clear(),
            Event::StartMap => self.block = StatusValue::new(self.separator),
            Event::EndMap => {
                self.has_urgent |= self.block.urgent;
                self.buffer.push(self.block.clone());
            }
            Event::EndArray => self.frames.push(self.buffer.clone()),
            Event::MapKey(bytes) => self.key = Some(first_nul(bytes).to_vec()),
            Event::Null => {}
            Event::Boolean(value) => {
                let Some(key) = self.key.as_deref() else {
                    return false;
                };
                if key_is(key, b"urgent") {
                    self.block.urgent = value;
                }
                if key_is(key, b"separator") {
                    self.block.no_separator = !value;
                }
            }
            Event::String(bytes) => {
                let Some(key) = self.key.as_deref() else {
                    return false;
                };
                if key_is(key, b"full_text") {
                    self.block.full = Some(bytes.to_vec());
                } else if key_is(key, b"short_text") {
                    self.block.short = Some(bytes.to_vec());
                } else if key_is(key, b"markup") {
                    self.block.markup = key_is(bytes, b"pango");
                } else if key_is(key, b"align") {
                    self.block.align = match bytes {
                        b"center" => BlockAlign::Center,
                        b"right" => BlockAlign::Right,
                        _ => BlockAlign::Left,
                    };
                } else {
                    let field = if key_is(key, b"color") {
                        Some(&mut self.block.color)
                    } else if key_is(key, b"background") {
                        Some(&mut self.block.background)
                    } else if key_is(key, b"border") {
                        Some(&mut self.block.border)
                    } else if key_is(key, b"min_width") {
                        Some(&mut self.block.minimum_text)
                    } else if key_is(key, b"name") {
                        Some(&mut self.block.name)
                    } else if key_is(key, b"instance") {
                        Some(&mut self.block.instance)
                    } else {
                        None
                    };
                    if let Some(field) = field {
                        *field = Some(first_nul(bytes).to_vec());
                    }
                }
            }
            Event::Number(number) => {
                // Match the compatibility parser: ignored floating values and
                // rejected i64::MIN/overflow when an integer callback exists.
                if number.contains(['.', 'e', 'E']) {
                    return true;
                }
                let Ok(number) = number.parse::<i64>() else {
                    self.error = Some("parse error: integer overflow".into());
                    return false;
                };
                if number == i64::MIN {
                    self.error = Some("parse error: integer overflow".into());
                    return false;
                }
                let Some(key) = self.key.as_deref() else {
                    return false;
                };
                let value = number as u32;
                if key_is(key, b"min_width") {
                    self.block.minimum = value;
                } else if key_is(key, b"separator_block_width") {
                    self.block.separator = value;
                } else if key_is(key, b"border_top") {
                    self.block.borders[0] = value;
                } else if key_is(key, b"border_right") {
                    self.block.borders[1] = value;
                } else if key_is(key, b"border_bottom") {
                    self.block.borders[2] = value;
                } else if key_is(key, b"border_left") {
                    self.block.borders[3] = value;
                }
            }
        }
        true
    }
    fn error_message(&self) -> Option<&str> {
        self.error.as_deref()
    }
}
struct StatusHandler(std::rc::Rc<std::cell::RefCell<StatusValues>>);
impl i3_json::EventHandler for StatusHandler {
    fn on_event(&mut self, event: i3_json::Event<'_>) -> bool {
        i3_json::EventHandler::on_event(&mut *self.0.borrow_mut(), event)
    }
    fn error_message(&self) -> Option<&str> {
        None
    }
}
// Handler-owned error text is copied into ParseError via this adapter's last
// failure; integer overflow is normalized after parse below without raw context.
struct StatusParser {
    parser: i3_json::Parser,
    values: std::rc::Rc<std::cell::RefCell<StatusValues>>,
}
impl StatusParser {
    fn new(urgent: bool) -> Self {
        let values = std::rc::Rc::new(std::cell::RefCell::new(StatusValues {
            key: None,
            block: StatusValue::new(0),
            buffer: Vec::new(),
            frames: Vec::new(),
            has_urgent: urgent,
            separator: 0,
            error: None,
        }));
        let mut parser = i3_json::Parser::new(StatusHandler(values.clone()));
        parser.options_mut().allow_legacy_whitespace = true;
        Self { parser, values }
    }
    fn feed(
        &mut self,
        input: &[u8],
        separator: u32,
    ) -> (Vec<Vec<StatusValue>>, bool, Result<(), i3_json::ParseError>) {
        self.values.borrow_mut().separator = separator;
        let mut result = self.parser.parse(input);
        let mut values = self.values.borrow_mut();
        if let (Err(error), Some(message)) = (&mut result, &values.error) {
            error.message = message.clone();
        }
        (
            std::mem::take(&mut values.frames),
            values.has_urgent,
            result,
        )
    }
}
#[derive(Default)]
struct WorkspaceLines {
    pending: Option<Vec<u8>>,
    last: Option<Vec<u8>>,
}
impl WorkspaceLines {
    fn feed(&mut self, input: &[u8]) -> Vec<Vec<u8>> {
        let input = first_nul(input);
        if input.is_empty() {
            return Vec::new();
        }
        let lines: Vec<_> = input.split(|&byte| byte == b'\n').collect();
        let mut completed = Vec::new();
        for (index, line) in lines.iter().enumerate() {
            if self.pending.is_none() && line.is_empty() {
                continue;
            }
            let mut next = self.pending.take().unwrap_or_default();
            next.extend_from_slice(line);
            if index + 1 == lines.len() {
                self.pending = Some(next);
            } else {
                self.last = Some(next.clone());
                completed.push(next);
            }
        }
        completed
    }
}
thread_local! {
    static STATUS_PARSER: std::cell::RefCell<Option<StatusParser>> = const { std::cell::RefCell::new(None) };
    // Original has_urgent becomes sticky after the first urgent block and is
    // not cleared by child restart. Retain this historical state intentionally.
    static EVER_URGENT: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    static WORKSPACE_LINES: std::cell::RefCell<WorkspaceLines> = std::cell::RefCell::new(WorkspaceLines::default());
}

/// Materialize one completed owned frame into the existing renderer's malloc
/// queue. Caller owns the event-thread queue/font; no Rust state borrow overlaps
/// native allocation, font prediction, logging or rendering callbacks.
unsafe fn install_status_frame(frame: Vec<StatusValue>) {
    if config.verbose {
        printf(
            c"[%s:%d] copying statusline_buffer to statusline_head\n".as_ptr(),
            c"../i3bar/src/child.c".as_ptr(),
            374i32,
        );
    }
    clear_statusline(&raw mut statusline_head, true);
    for value in frame {
        let block = scalloc(1, std::mem::size_of::<status_block>()).cast::<status_block>();
        (*block).full_text = value.full.as_ref().map_or_else(
            || i3string_from_utf8(c"SPEC VIOLATION: full_text is NULL!".as_ptr()),
            |text| i3string_from_markup_with_length(text.as_ptr().cast(), text.len()),
        );
        (*block).short_text = value.short.as_ref().map_or(std::ptr::null_mut(), |text| {
            i3string_from_markup_with_length(text.as_ptr().cast(), text.len())
        });
        for (slot, bytes) in [
            (&raw mut (*block).color, value.color),
            (&raw mut (*block).background, value.background),
            (&raw mut (*block).border, value.border),
            (&raw mut (*block).name, value.name),
            (&raw mut (*block).instance, value.instance),
            (&raw mut (*block).min_width_str, value.minimum_text),
        ] {
            if let Some(bytes) = bytes {
                let string = std::ffi::CString::new(bytes).unwrap();
                *slot = sstrdup(string.as_ptr());
            }
        }
        (*block).min_width = value.minimum;
        (*block).sep_block_width = value.separator;
        (*block).align = value.align;
        (*block).urgent = value.urgent;
        (*block).no_separator = value.no_separator;
        (*block).border_top = value.borders[0];
        (*block).border_right = value.borders[1];
        (*block).border_bottom = value.borders[2];
        (*block).border_left = value.borders[3];
        (*block).pango_markup = value.markup;
        if !(*block).min_width_str.is_null() {
            let text = i3string_from_utf8((*block).min_width_str);
            i3string_set_markup(text, value.markup);
            (*block).min_width = predict_text_width(text) as u32;
            i3string_free(text);
        }
        i3string_set_markup((*block).full_text, value.markup);
        if !(*block).short_text.is_null() {
            i3string_set_markup((*block).short_text, value.markup);
        }
        (*block).blocks.tqe_next = std::ptr::null_mut();
        (*block).blocks.tqe_prev = statusline_head.tqh_last;
        *statusline_head.tqh_last = block;
        statusline_head.tqh_last = &raw mut (*block).blocks.tqe_next;
    }
    if config.verbose {
        printf(
            c"[%s:%d] dumping statusline:\n".as_ptr(),
            c"../i3bar/src/child.c".as_ptr(),
            378i32,
        );
        let mut current = statusline_head.tqh_first;
        while !current.is_null() {
            printf(
                c"[%s:%d] full_text = %s\n".as_ptr(),
                c"../i3bar/src/child.c".as_ptr(),
                381i32,
                i3string_as_utf8((*current).full_text),
            );
            printf(
                c"[%s:%d] short_text = %s\n".as_ptr(),
                c"../i3bar/src/child.c".as_ptr(),
                382i32,
                if (*current).short_text.is_null() {
                    std::ptr::null()
                } else {
                    i3string_as_utf8((*current).short_text)
                },
            );
            printf(
                c"[%s:%d] color = %s\n".as_ptr(),
                c"../i3bar/src/child.c".as_ptr(),
                383i32,
                (*current).color,
            );
            current = (*current).blocks.tqe_next;
        }
        printf(
            c"[%s:%d] end of dump\n".as_ptr(),
            c"../i3bar/src/child.c".as_ptr(),
            385i32,
        );
    }
}

#[derive(Debug, PartialEq, Eq)]
enum InputChunk {
    Bytes(Vec<u8>),
    Empty,
    Eof,
}
fn read_available(reader: &mut impl std::io::Read) -> std::io::Result<InputChunk> {
    let mut bytes = Vec::new();
    let mut chunk = [0u8; 1024];
    loop {
        match reader.read(&mut chunk) {
            Ok(0) => return Ok(InputChunk::Eof), // legacy discards bytes before EOF
            Ok(count) => bytes.extend_from_slice(&chunk[..count]),
            Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => break,
            Err(error) => return Err(error), // including EINTR, as before
        }
    }
    Ok(if bytes.first().is_none_or(|&byte| byte == 0) {
        InputChunk::Empty
    } else {
        InputChunk::Bytes(bytes)
    })
}
struct Descriptor(std::os::fd::RawFd);
impl std::io::Read for Descriptor {
    fn read(&mut self, bytes: &mut [u8]) -> std::io::Result<usize> {
        let count = unsafe { libc::read(self.0, bytes.as_mut_ptr().cast(), bytes.len()) };
        if count < 0 {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(count as usize)
        }
    }
}
unsafe fn get_buffer(fd: i32) -> Option<Vec<u8>> {
    match read_available(&mut Descriptor(fd)) {
        Ok(InputChunk::Bytes(bytes)) => Some(bytes),
        Ok(InputChunk::Empty) => None,
        Ok(InputChunk::Eof) => {
            fprintf(
                stderr,
                c"[%s:%d] ERROR: stdin: received EOF\n".as_ptr(),
                c"../i3bar/src/child.c".as_ptr(),
                412i32,
            );
            None
        }
        Err(error) => {
            fprintf(
                stderr,
                c"[%s:%d] ERROR: read() failed!: %s\n".as_ptr(),
                c"../i3bar/src/child.c".as_ptr(),
                407i32,
                libc::strerror(error.raw_os_error().unwrap_or(libc::EIO)),
            );
            exit(libc::EXIT_FAILURE);
        }
    }
}
fn flat_text(bytes: &[u8]) -> &[u8] {
    let bytes = if matches!(bytes.last(), Some(b'\n' | b'\r')) {
        &bytes[..bytes.len() - 1]
    } else {
        bytes
    };
    first_nul(bytes)
}
unsafe fn read_flat_input(bytes: &[u8]) {
    let first = statusline_head.tqh_first;
    i3string_free((*first).full_text);
    (*first).full_text = Box::into_raw(Box::new(i3_common::string::I3String::from_bytes(
        flat_text(bytes),
    )));
}
unsafe fn read_json_input(input: &[u8]) -> bool {
    let separator = if config.separator_symbol.is_null() {
        logical_px(9)
    } else {
        logical_px(8) + separator_symbol_width
    } as u32;
    let (frames, urgent, result) = STATUS_PARSER.with(|parser| {
        parser
            .borrow_mut()
            .as_mut()
            .expect("initialized status parser")
            .feed(input, separator)
    });
    EVER_URGENT.with(|value| value.set(urgent));
    for frame in frames {
        install_status_frame(frame);
    }
    match result {
        Ok(()) => urgent,
        Err(error) => {
            let status = if error.kind == i3_json::ParseErrorKind::Canceled {
                1u32
            } else {
                2
            };
            let diagnostic = error.to_string();
            let message = std::ffi::CString::new(diagnostic.trim_end_matches('\n'))
                .expect("JSON diagnostic has no NUL");
            fprintf(
                stderr,
                c"[i3bar] Could not parse JSON input (code = %d, message = %s): %.*s\n".as_ptr(),
                status,
                message.as_ptr(),
                input.len().min(i32::MAX as usize) as i32,
                input.as_ptr(),
            );
            set_statusline_error(c"Could not parse JSON (%s)".as_ptr(), message.as_ptr());
            draw_bars(false);
            false
        }
    }
}
unsafe fn stdin_io_cb(fd: i32) {
    let Some(buffer) = get_buffer(fd) else {
        return;
    };
    let urgent = if status_child.version > 0 {
        read_json_input(&buffer)
    } else {
        read_flat_input(&buffer);
        false
    };
    draw_bars(urgent);
}
unsafe fn stdin_io_first_line_cb(fd: i32) {
    let Some(buffer) = get_buffer(fd) else {
        return;
    };
    if config.verbose {
        printf(
            c"[%s:%d] Detecting input type based on buffer *%.*s*\n".as_ptr(),
            c"../i3bar/src/child.c".as_ptr(),
            503i32,
            buffer.len().min(i32::MAX as usize) as i32,
            buffer.as_ptr(),
        );
    }
    let header = parse_json_header(&buffer);
    status_child.version = header.version;
    status_child.stop_signal = header.stop_signal;
    status_child.cont_signal = header.cont_signal;
    if let Some(click) = header.click_events {
        status_child.click_events = click;
    }
    if status_child.version > 0 {
        if config.hide_on_modifier != crate::application::BarDisplayMode::Dock {
            stop_children();
        }
        draw_bars(read_json_input(&buffer[header.consumed..]));
    } else {
        let block = scalloc(1, std::mem::size_of::<status_block>()).cast::<status_block>();
        (*block).blocks.tqe_next = std::ptr::null_mut();
        (*block).blocks.tqe_prev = statusline_head.tqh_last;
        *statusline_head.tqh_last = block;
        statusline_head.tqh_last = &raw mut (*block).blocks.tqe_next;
        read_flat_input(&buffer);
    }
}
unsafe fn ws_stdin_io_cb(fd: i32) {
    let Some(buffer) = get_buffer(fd) else {
        return;
    };
    let frames = WORKSPACE_LINES.with(|state| state.borrow_mut().feed(&buffer));
    for frame in frames {
        parse_workspaces_json(frame.as_ptr(), frame.len());
    }
    draw_bars(false);
}
unsafe extern "C" fn common_stdin_cb(
    loop_0: *mut ev_loop,
    watcher: *mut ev_io,
    revents: ::core::ffi::c_int,
) {
    if watcher == status_child.stdin_io {
        if status_child.version == -(1 as ::core::ffi::c_int) as u32 {
            stdin_io_first_line_cb((*watcher).fd);
        } else {
            stdin_io_cb((*watcher).fd);
        }
    } else if watcher == ws_child.stdin_io {
        ws_stdin_io_cb((*watcher).fd);
    } else {
        fprintf(
            stderr,
            c"[%s:%d] ERROR: Got callback for unknown watcher fd=%d\n".as_ptr(),
            c"../i3bar/src/child.c".as_ptr(),
            594 as ::core::ffi::c_int,
            (*watcher).fd,
        );
    };
}

pub unsafe fn repeat_last_ws_json() {
    let frame = WORKSPACE_LINES.with(|state| state.borrow().last.clone());
    if let Some(frame) = frame {
        if config.verbose {
            printf(
                c"[%s:%d] Repeating last workspace JSON\n".as_ptr(),
                c"../i3bar/src/child.c".as_ptr(),
                604i32,
            );
        }
        parse_workspaces_json(frame.as_ptr(), frame.len());
    }
}
unsafe extern "C" fn set_workspace_button_error_f(format: *const ::core::ffi::c_char, args: ...) {
    let mut message: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut args_0;
    args_0 = args.clone();
    if !(vasprintf(&raw mut message, format, i333_va_list!(args_0)) == -(1 as ::core::ffi::c_int)) {
        set_workspace_button_error(message);
    }
    free(message as *mut ::core::ffi::c_void);
}

pub unsafe fn set_workspace_button_error(message: *const ::core::ffi::c_char) {
    free_workspaces();
    let mut text = b"Error: ".to_vec();
    text.extend_from_slice(std::ffi::CStr::from_ptr(message).to_bytes());
    let name = i3_common::string::I3String::from_bytes(&text);
    let mut output = (*outputs).slh_first;
    while !output.is_null() {
        let text = name.clone();
        let width = predict_text_width((&text as *const i3_common::string::I3String).cast_mut());
        crate::workspaces::append_workspace(output, Box::new(i3_ws::error(text, width)));
        output = (*output).slist.sle_next;
    }
}
unsafe extern "C" fn child_sig_cb(
    loop_0: *mut ev_loop,
    watcher: *mut ev_child,
    revents: ::core::ffi::c_int,
) {
    let exit_status: ::core::ffi::c_int =
        ((*watcher).rstatus & 0xff00 as ::core::ffi::c_int) >> 8 as ::core::ffi::c_int;
    fprintf(
        stderr,
        c"[%s:%d] ERROR: Child (pid: %d) unexpectedly exited with status %d\n".as_ptr(),
        c"../i3bar/src/child.c".as_ptr(),
        664 as ::core::ffi::c_int,
        (*watcher).pid,
        exit_status,
    );
    let mut error_function_pointer: Option<
        unsafe extern "C" fn(*const ::core::ffi::c_char, ...) -> (),
    > = None;
    let mut command_type: *const ::core::ffi::c_char = c"".as_ptr();
    let mut c: *mut i3bar_child = ::core::ptr::null_mut::<i3bar_child>();
    if (*watcher).pid == status_child.pid {
        command_type = c"status_command".as_ptr();
        error_function_pointer = Some(
            set_statusline_error as unsafe extern "C" fn(*const ::core::ffi::c_char, ...) -> (),
        )
            as Option<unsafe extern "C" fn(*const ::core::ffi::c_char, ...) -> ()>;
        c = &raw mut status_child;
    } else if (*watcher).pid == ws_child.pid {
        command_type = c"workspace_command".as_ptr();
        error_function_pointer = Some(
            set_workspace_button_error_f
                as unsafe extern "C" fn(*const ::core::ffi::c_char, ...) -> (),
        )
            as Option<unsafe extern "C" fn(*const ::core::ffi::c_char, ...) -> ()>;
        c = &raw mut ws_child;
    } else {
        fprintf(
            stderr,
            c"[%s:%d] ERROR: Unknown child pid, this should never happen\n".as_ptr(),
            c"../i3bar/src/child.c".as_ptr(),
            678 as ::core::ffi::c_int,
        );
        return;
    }
    if (*c).pid == 0 as ::core::ffi::c_int {
        if config.verbose {
            printf(
                c"[%s:%d] %s: child pid = 0\n".as_ptr(),
                c"../i3bar/src/child.c".as_ptr(),
                681 as ::core::ffi::c_int,
                c"child_sig_cb".as_ptr(),
            );
        }
    } else if (*c).pid == status_child.pid {
        if config.verbose {
            printf(
                c"[%s:%d] %s: status_command: pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n".as_ptr(),
                c"../i3bar/src/child.c".as_ptr(),
                681 as ::core::ffi::c_int,
                c"child_sig_cb".as_ptr(),
                (*c).pid as ::core::ffi::c_long,
                (*c).stopped as ::core::ffi::c_int,
                (*c).stop_signal,
                (*c).cont_signal,
                (*c).click_events as ::core::ffi::c_int,
                (*c).click_events_init as ::core::ffi::c_int,
            );
        }
    } else if (*c).pid == ws_child.pid {
        if config.verbose {
            printf(
                c"[%s:%d] %s: workspace_command: pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n".as_ptr(),
                c"../i3bar/src/child.c".as_ptr(),
                681 as ::core::ffi::c_int,
                c"child_sig_cb".as_ptr(),
                (*c).pid as ::core::ffi::c_long,
                (*c).stopped as ::core::ffi::c_int,
                (*c).stop_signal,
                (*c).cont_signal,
                (*c).click_events as ::core::ffi::c_int,
                (*c).click_events_init as ::core::ffi::c_int,
            );
        }
    } else {
        fprintf(
            stderr,
            c"[%s:%d] ERROR: %s: unknown child, this should never happen pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n".as_ptr(),
            c"../i3bar/src/child.c".as_ptr(),
            681 as ::core::ffi::c_int,
            c"child_sig_cb".as_ptr(),
            (*c).pid as ::core::ffi::c_long,
            (*c).stopped as ::core::ffi::c_int,
            (*c).stop_signal,
            (*c).cont_signal,
            (*c).click_events as ::core::ffi::c_int,
            (*c).click_events_init as ::core::ffi::c_int,
        );
    }
    if exit_status == 126 as ::core::ffi::c_int {
        error_function_pointer.expect("non-null function pointer")(
            c"%s is not executable (exit %d)".as_ptr(),
            command_type,
            exit_status,
        );
    } else if exit_status == 127 as ::core::ffi::c_int {
        error_function_pointer.expect("non-null function pointer")(
            c"%s not found or is missing a library dependency (exit %d)".as_ptr(),
            command_type,
            exit_status,
        );
    } else {
        error_function_pointer.expect("non-null function pointer")(
            c"%s process exited unexpectedly (exit %d)".as_ptr(),
            command_type,
            exit_status,
        );
    }
    cleanup(c);
    draw_bars(false);
}
unsafe fn child_write_output() {
    if !status_child.click_events {
        return;
    }
    // End the generator borrow before writes can kill/restart the native child.
    let bytes = CLICK_GENERATOR.with(|generator| {
        let mut generator = generator.borrow_mut();
        let bytes = generator.bytes().to_vec();
        generator.clear();
        bytes
    });
    let mut count = writeall(child_stdin, bytes.as_ptr().cast(), bytes.len());
    if count != -1 {
        count = writeall(child_stdin, c"\n".as_ptr().cast(), 1);
    }
    if count == -1 {
        status_child.click_events = false;
        kill_child();
        set_statusline_error(c"child_write_output failed".as_ptr());
        draw_bars(false);
    }
}
unsafe fn sfork() -> libc::pid_t {
    let pid: libc::pid_t = fork() as libc::pid_t;
    if pid == -(1 as ::core::ffi::c_int) {
        fprintf(
            stderr,
            c"[%s:%d] ERROR: Couldn't fork(): %s\n".as_ptr(),
            c"../i3bar/src/child.c".as_ptr(),
            724 as ::core::ffi::c_int,
            strerror(*__errno_location()),
        );
        exit(libc::EXIT_FAILURE);
    }
    pid
}
unsafe fn spipe(pipedes: *mut ::core::ffi::c_int) {
    if pipe(pipedes) == -(1 as ::core::ffi::c_int) {
        err(libc::EXIT_FAILURE, c"pipe(pipe_in)".as_ptr());
    }
}
unsafe fn exec_shell(command: *mut ::core::ffi::c_char) {
    execl(
        c"/bin/sh".as_ptr(),
        c"/bin/sh".as_ptr(),
        c"-c".as_ptr(),
        command,
        std::ptr::null::<libc::c_char>(),
    );
    err(libc::EXIT_FAILURE, c"execl return".as_ptr());
}
unsafe fn setup_child_cb(child: *mut i3bar_child) {
    fcntl((*child).stdin_fd, libc::F_SETFL, libc::O_NONBLOCK);
    (*child).stdin_io = smalloc(std::mem::size_of::<ev_io>()).cast();
    (*child)
        .stdin_io
        .write(ev_io::new((*child).stdin_fd, EV_READ, common_stdin_cb));
    ev_io_start(main_loop, (*child).stdin_io);
    (*child).child_sig = smalloc(std::mem::size_of::<ev_child>()).cast();
    (*child)
        .child_sig
        .write(ev_child::new((*child).pid, false, child_sig_cb));
    ev_child_start(main_loop, (*child).child_sig);
    if (*child).pid == 0 as ::core::ffi::c_int {
        if config.verbose {
            printf(
                c"[%s:%d] %s: child pid = 0\n".as_ptr(),
                c"../i3bar/src/child.c".as_ptr(),
                754 as ::core::ffi::c_int,
                c"setup_child_cb".as_ptr(),
            );
        }
    } else if (*child).pid == status_child.pid {
        if config.verbose {
            printf(
                c"[%s:%d] %s: status_command: pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n".as_ptr(),
                c"../i3bar/src/child.c".as_ptr(),
                754 as ::core::ffi::c_int,
                c"setup_child_cb".as_ptr(),
                (*child).pid as ::core::ffi::c_long,
                (*child).stopped as ::core::ffi::c_int,
                (*child).stop_signal,
                (*child).cont_signal,
                (*child).click_events as ::core::ffi::c_int,
                (*child).click_events_init as ::core::ffi::c_int,
            );
        }
    } else if (*child).pid == ws_child.pid {
        if config.verbose {
            printf(
                c"[%s:%d] %s: workspace_command: pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n".as_ptr(),
                c"../i3bar/src/child.c".as_ptr(),
                754 as ::core::ffi::c_int,
                c"setup_child_cb".as_ptr(),
                (*child).pid as ::core::ffi::c_long,
                (*child).stopped as ::core::ffi::c_int,
                (*child).stop_signal,
                (*child).cont_signal,
                (*child).click_events as ::core::ffi::c_int,
                (*child).click_events_init as ::core::ffi::c_int,
            );
        }
    } else {
        fprintf(
            stderr,
            c"[%s:%d] ERROR: %s: unknown child, this should never happen pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n".as_ptr(),
            c"../i3bar/src/child.c".as_ptr(),
            754 as ::core::ffi::c_int,
            c"setup_child_cb".as_ptr(),
            (*child).pid as ::core::ffi::c_long,
            (*child).stopped as ::core::ffi::c_int,
            (*child).stop_signal,
            (*child).cont_signal,
            (*child).click_events as ::core::ffi::c_int,
            (*child).click_events_init as ::core::ffi::c_int,
        );
    };
}

pub unsafe fn start_child(command: *mut ::core::ffi::c_char) {
    if command.is_null() {
        return;
    }
    let urgent = EVER_URGENT.with(|value| value.get());
    STATUS_PARSER.with(|parser| *parser.borrow_mut() = Some(StatusParser::new(urgent)));
    CLICK_GENERATOR.with(|generator| *generator.borrow_mut() = i3_json::Generator::new());
    let mut pipe_in: [::core::ffi::c_int; 2] = [0; 2];
    let mut pipe_out: [::core::ffi::c_int; 2] = [0; 2];
    spipe(&raw mut pipe_in as *mut ::core::ffi::c_int);
    spipe(&raw mut pipe_out as *mut ::core::ffi::c_int);
    status_child.pid = sfork();
    if status_child.pid == 0 as ::core::ffi::c_int {
        close(pipe_in[0 as ::core::ffi::c_int as usize]);
        close(pipe_out[1 as ::core::ffi::c_int as usize]);
        dup2(
            pipe_in[1 as ::core::ffi::c_int as usize],
            libc::STDOUT_FILENO,
        );
        dup2(
            pipe_out[0 as ::core::ffi::c_int as usize],
            libc::STDIN_FILENO,
        );
        setpgid(status_child.pid as libc::pid_t, 0 as libc::pid_t);
        exec_shell(command);
        return;
    }
    close(pipe_in[1 as ::core::ffi::c_int as usize]);
    close(pipe_out[0 as ::core::ffi::c_int as usize]);
    status_child.stdin_fd = pipe_in[0 as ::core::ffi::c_int as usize];
    child_stdin = pipe_out[1 as ::core::ffi::c_int as usize];
    status_child.version = -(1 as ::core::ffi::c_int) as u32;
    setup_child_cb(&raw mut status_child);
}

pub unsafe fn start_ws_child(command: *mut ::core::ffi::c_char) {
    if command.is_null() {
        return;
    }
    ws_child.stop_signal = libc::SIGSTOP;
    ws_child.cont_signal = libc::SIGCONT;
    let mut pipe_in: [::core::ffi::c_int; 2] = [0; 2];
    spipe(&raw mut pipe_in as *mut ::core::ffi::c_int);
    ws_child.pid = sfork();
    if ws_child.pid == 0 as ::core::ffi::c_int {
        close(pipe_in[0 as ::core::ffi::c_int as usize]);
        dup2(
            pipe_in[1 as ::core::ffi::c_int as usize],
            libc::STDOUT_FILENO,
        );
        setpgid(ws_child.pid as libc::pid_t, 0 as libc::pid_t);
        exec_shell(command);
    }
    close(pipe_in[1 as ::core::ffi::c_int as usize]);
    ws_child.stdin_fd = pipe_in[0 as ::core::ffi::c_int as usize];
    setup_child_cb(&raw mut ws_child);
}
unsafe fn child_click_events_initialize() {
    if status_child.pid == 0 as ::core::ffi::c_int {
        if config.verbose {
            printf(
                c"[%s:%d] %s: child pid = 0\n".as_ptr(),
                c"../i3bar/src/child.c".as_ptr(),
                847 as ::core::ffi::c_int,
                c"child_click_events_initialize".as_ptr(),
            );
        }
    } else if status_child.pid == status_child.pid {
        if config.verbose {
            printf(
                c"[%s:%d] %s: status_command: pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n".as_ptr(),
                c"../i3bar/src/child.c".as_ptr(),
                847 as ::core::ffi::c_int,
                c"child_click_events_initialize".as_ptr(),
                status_child.pid as ::core::ffi::c_long,
                status_child.stopped as ::core::ffi::c_int,
                status_child.stop_signal,
                status_child.cont_signal,
                status_child.click_events as ::core::ffi::c_int,
                status_child.click_events_init as ::core::ffi::c_int,
            );
        }
    } else if status_child.pid == ws_child.pid {
        if config.verbose {
            printf(
                c"[%s:%d] %s: workspace_command: pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n".as_ptr(),
                c"../i3bar/src/child.c".as_ptr(),
                847 as ::core::ffi::c_int,
                c"child_click_events_initialize".as_ptr(),
                status_child.pid as ::core::ffi::c_long,
                status_child.stopped as ::core::ffi::c_int,
                status_child.stop_signal,
                status_child.cont_signal,
                status_child.click_events as ::core::ffi::c_int,
                status_child.click_events_init as ::core::ffi::c_int,
            );
        }
    } else {
        fprintf(
            stderr,
            c"[%s:%d] ERROR: %s: unknown child, this should never happen pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n".as_ptr(),
            c"../i3bar/src/child.c".as_ptr(),
            847 as ::core::ffi::c_int,
            c"child_click_events_initialize".as_ptr(),
            status_child.pid as ::core::ffi::c_long,
            status_child.stopped as ::core::ffi::c_int,
            status_child.stop_signal,
            status_child.cont_signal,
            status_child.click_events as ::core::ffi::c_int,
            status_child.click_events_init as ::core::ffi::c_int,
        );
    }
    if !status_child.click_events_init {
        CLICK_GENERATOR.with(|generator| {
            let _ = generator.borrow_mut().start_array();
        });
        child_write_output();
        status_child.click_events_init = true;
    }
}

struct ClickValue<'a> {
    name: Option<&'a [u8]>,
    instance: Option<&'a [u8]>,
    button: i32,
    position: [i32; 8],
    modifiers: i32,
}
fn generator_bytes(generator: &mut i3_json::Generator, bytes: &[u8]) {
    // The legacy compatibility generator rejects invalid UTF-8 without advancing
    // its state. Preserve that behavior instead of repairing byte identifiers.
    if let Ok(string) = std::str::from_utf8(bytes) {
        let _ = generator.string(string);
    }
}
fn append_click(generator: &mut i3_json::Generator, click: ClickValue<'_>) {
    let _ = generator.start_object();
    for (key, value) in [("name", click.name), ("instance", click.instance)] {
        if let Some(value) = value {
            let _ = generator.string(key);
            generator_bytes(generator, value);
        }
    }
    let _ = generator.string("button");
    let _ = generator.integer(i64::from(click.button));
    let _ = generator.string("modifiers");
    let _ = generator.start_array();
    for (mask, name) in [
        (XCB_MOD_MASK_SHIFT, "Shift"),
        (XCB_MOD_MASK_CONTROL, "Control"),
        (XCB_MOD_MASK_1, "Mod1"),
        (XCB_MOD_MASK_2, "Mod2"),
        (XCB_MOD_MASK_3, "Mod3"),
        (XCB_MOD_MASK_4, "Mod4"),
        (XCB_MOD_MASK_5, "Mod5"),
    ] {
        if click.modifiers & mask as i32 != 0 {
            let _ = generator.string(name);
        }
    }
    let _ = generator.end_array();
    for (key, value) in [
        "x",
        "y",
        "relative_x",
        "relative_y",
        "output_x",
        "output_y",
        "width",
        "height",
    ]
    .into_iter()
    .zip(click.position)
    {
        let _ = generator.string(key);
        let _ = generator.integer(i64::from(value));
    }
    let _ = generator.end_object();
}
pub unsafe fn send_block_clicked(
    button: ::core::ffi::c_int,
    name: *const ::core::ffi::c_char,
    instance: *const ::core::ffi::c_char,
    x: ::core::ffi::c_int,
    y: ::core::ffi::c_int,
    x_rel: ::core::ffi::c_int,
    y_rel: ::core::ffi::c_int,
    out_x: ::core::ffi::c_int,
    out_y: ::core::ffi::c_int,
    width: ::core::ffi::c_int,
    height: ::core::ffi::c_int,
    mods: ::core::ffi::c_int,
) {
    if !status_child.click_events {
        return;
    }
    child_click_events_initialize();
    // Name/instance are live native strings for this callback. Generator access
    // ends before output failure can invoke kill_child/cleanup/reload.
    let name = (!name.is_null()).then(|| std::ffi::CStr::from_ptr(name).to_bytes());
    let instance = (!instance.is_null()).then(|| std::ffi::CStr::from_ptr(instance).to_bytes());
    CLICK_GENERATOR.with(|generator| {
        append_click(
            &mut generator.borrow_mut(),
            ClickValue {
                name,
                instance,
                button,
                position: [x, y, x_rel, y_rel, out_x, out_y, width, height],
                modifiers: mods,
            },
        )
    });
    child_write_output();
}
unsafe fn is_alive(c: *mut i3bar_child) -> bool {
    (*c).pid > 0 as ::core::ffi::c_int
}

pub unsafe fn status_child_is_alive() -> bool {
    is_alive(&raw mut status_child)
}

pub unsafe fn ws_child_is_alive() -> bool {
    is_alive(&raw mut ws_child)
}

pub extern "C" fn kill_children_at_exit() {
    unsafe {
        if status_child.pid == 0 as ::core::ffi::c_int {
            if config.verbose {
                printf(
                    c"[%s:%d] %s: child pid = 0\n".as_ptr(),
                    c"../i3bar/src/child.c".as_ptr(),
                    960 as ::core::ffi::c_int,
                    c"kill_children_at_exit".as_ptr(),
                );
            }
        } else if status_child.pid == status_child.pid {
            if config.verbose {
                printf(
                c"[%s:%d] %s: status_command: pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n".as_ptr(),
                c"../i3bar/src/child.c".as_ptr(),
                960 as ::core::ffi::c_int,
                c"kill_children_at_exit".as_ptr(),
                status_child.pid as ::core::ffi::c_long,
                status_child.stopped as ::core::ffi::c_int,
                status_child.stop_signal,
                status_child.cont_signal,
                status_child.click_events as ::core::ffi::c_int,
                status_child.click_events_init as ::core::ffi::c_int,
            );
            }
        } else if status_child.pid == ws_child.pid {
            if config.verbose {
                printf(
                c"[%s:%d] %s: workspace_command: pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n".as_ptr(),
                c"../i3bar/src/child.c".as_ptr(),
                960 as ::core::ffi::c_int,
                c"kill_children_at_exit".as_ptr(),
                status_child.pid as ::core::ffi::c_long,
                status_child.stopped as ::core::ffi::c_int,
                status_child.stop_signal,
                status_child.cont_signal,
                status_child.click_events as ::core::ffi::c_int,
                status_child.click_events_init as ::core::ffi::c_int,
            );
            }
        } else {
            fprintf(
            stderr,
            c"[%s:%d] ERROR: %s: unknown child, this should never happen pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n".as_ptr(),
            c"../i3bar/src/child.c".as_ptr(),
            960 as ::core::ffi::c_int,
            c"kill_children_at_exit".as_ptr(),
            status_child.pid as ::core::ffi::c_long,
            status_child.stopped as ::core::ffi::c_int,
            status_child.stop_signal,
            status_child.cont_signal,
            status_child.click_events as ::core::ffi::c_int,
            status_child.click_events_init as ::core::ffi::c_int,
        );
        }
        if ws_child.pid == 0 as ::core::ffi::c_int {
            if config.verbose {
                printf(
                    c"[%s:%d] %s: child pid = 0\n".as_ptr(),
                    c"../i3bar/src/child.c".as_ptr(),
                    960 as ::core::ffi::c_int,
                    c"kill_children_at_exit".as_ptr(),
                );
            }
        } else if ws_child.pid == status_child.pid {
            if config.verbose {
                printf(
                c"[%s:%d] %s: status_command: pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n".as_ptr(),
                c"../i3bar/src/child.c".as_ptr(),
                960 as ::core::ffi::c_int,
                c"kill_children_at_exit".as_ptr(),
                ws_child.pid as ::core::ffi::c_long,
                ws_child.stopped as ::core::ffi::c_int,
                ws_child.stop_signal,
                ws_child.cont_signal,
                ws_child.click_events as ::core::ffi::c_int,
                ws_child.click_events_init as ::core::ffi::c_int,
            );
            }
        } else if ws_child.pid == ws_child.pid {
            if config.verbose {
                printf(
                c"[%s:%d] %s: workspace_command: pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n".as_ptr(),
                c"../i3bar/src/child.c".as_ptr(),
                960 as ::core::ffi::c_int,
                c"kill_children_at_exit".as_ptr(),
                ws_child.pid as ::core::ffi::c_long,
                ws_child.stopped as ::core::ffi::c_int,
                ws_child.stop_signal,
                ws_child.cont_signal,
                ws_child.click_events as ::core::ffi::c_int,
                ws_child.click_events_init as ::core::ffi::c_int,
            );
            }
        } else {
            fprintf(
            stderr,
            c"[%s:%d] ERROR: %s: unknown child, this should never happen pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n".as_ptr(),
            c"../i3bar/src/child.c".as_ptr(),
            960 as ::core::ffi::c_int,
            c"kill_children_at_exit".as_ptr(),
            ws_child.pid as ::core::ffi::c_long,
            ws_child.stopped as ::core::ffi::c_int,
            ws_child.stop_signal,
            ws_child.cont_signal,
            ws_child.click_events as ::core::ffi::c_int,
            ws_child.click_events_init as ::core::ffi::c_int,
        );
        }
        cont_children();
        if is_alive(&raw mut status_child) {
            killpg(status_child.pid as libc::pid_t, libc::SIGTERM);
        }
        if is_alive(&raw mut ws_child) {
            killpg(ws_child.pid as libc::pid_t, libc::SIGTERM);
        }
    }
}
unsafe fn cont_child(c: *mut i3bar_child) {
    if is_alive(c) as ::core::ffi::c_int != 0
        && (*c).cont_signal > 0 as ::core::ffi::c_int
        && (*c).stopped as ::core::ffi::c_int != 0
    {
        (*c).stopped = false;
        killpg((*c).pid as libc::pid_t, (*c).cont_signal);
    }
}
unsafe fn kill_and_wait(c: *mut i3bar_child) {
    if (*c).pid == 0 as ::core::ffi::c_int {
        if config.verbose {
            printf(
                c"[%s:%d] %s: child pid = 0\n".as_ptr(),
                c"../i3bar/src/child.c".as_ptr(),
                979 as ::core::ffi::c_int,
                c"kill_and_wait".as_ptr(),
            );
        }
    } else if (*c).pid == status_child.pid {
        if config.verbose {
            printf(
                c"[%s:%d] %s: status_command: pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n".as_ptr(),
                c"../i3bar/src/child.c".as_ptr(),
                979 as ::core::ffi::c_int,
                c"kill_and_wait".as_ptr(),
                (*c).pid as ::core::ffi::c_long,
                (*c).stopped as ::core::ffi::c_int,
                (*c).stop_signal,
                (*c).cont_signal,
                (*c).click_events as ::core::ffi::c_int,
                (*c).click_events_init as ::core::ffi::c_int,
            );
        }
    } else if (*c).pid == ws_child.pid {
        if config.verbose {
            printf(
                c"[%s:%d] %s: workspace_command: pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n".as_ptr(),
                c"../i3bar/src/child.c".as_ptr(),
                979 as ::core::ffi::c_int,
                c"kill_and_wait".as_ptr(),
                (*c).pid as ::core::ffi::c_long,
                (*c).stopped as ::core::ffi::c_int,
                (*c).stop_signal,
                (*c).cont_signal,
                (*c).click_events as ::core::ffi::c_int,
                (*c).click_events_init as ::core::ffi::c_int,
            );
        }
    } else {
        fprintf(
            stderr,
            c"[%s:%d] ERROR: %s: unknown child, this should never happen pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n".as_ptr(),
            c"../i3bar/src/child.c".as_ptr(),
            979 as ::core::ffi::c_int,
            c"kill_and_wait".as_ptr(),
            (*c).pid as ::core::ffi::c_long,
            (*c).stopped as ::core::ffi::c_int,
            (*c).stop_signal,
            (*c).cont_signal,
            (*c).click_events as ::core::ffi::c_int,
            (*c).click_events_init as ::core::ffi::c_int,
        );
    }
    if !is_alive(c) {
        return;
    }
    cont_child(c);
    killpg((*c).pid as libc::pid_t, libc::SIGTERM);
    let mut status: ::core::ffi::c_int = 0;
    waitpid(
        (*c).pid as libc::pid_t,
        &raw mut status,
        0 as ::core::ffi::c_int,
    );
    cleanup(c);
}

pub unsafe fn kill_child() {
    kill_and_wait(&raw mut status_child);
}

pub unsafe fn kill_ws_child() {
    kill_and_wait(&raw mut ws_child);
}
unsafe fn stop_child(c: *mut i3bar_child) {
    if (*c).stop_signal > 0 as ::core::ffi::c_int && !(*c).stopped {
        (*c).stopped = true;
        killpg((*c).pid as libc::pid_t, (*c).stop_signal);
    }
}

pub unsafe fn stop_children() {
    if status_child.pid == 0 as ::core::ffi::c_int {
        if config.verbose {
            printf(
                c"[%s:%d] %s: child pid = 0\n".as_ptr(),
                c"../i3bar/src/child.c".as_ptr(),
                1022 as ::core::ffi::c_int,
                c"stop_children".as_ptr(),
            );
        }
    } else if status_child.pid == status_child.pid {
        if config.verbose {
            printf(
                c"[%s:%d] %s: status_command: pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n".as_ptr(),
                c"../i3bar/src/child.c".as_ptr(),
                1022 as ::core::ffi::c_int,
                c"stop_children".as_ptr(),
                status_child.pid as ::core::ffi::c_long,
                status_child.stopped as ::core::ffi::c_int,
                status_child.stop_signal,
                status_child.cont_signal,
                status_child.click_events as ::core::ffi::c_int,
                status_child.click_events_init as ::core::ffi::c_int,
            );
        }
    } else if status_child.pid == ws_child.pid {
        if config.verbose {
            printf(
                c"[%s:%d] %s: workspace_command: pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n".as_ptr(),
                c"../i3bar/src/child.c".as_ptr(),
                1022 as ::core::ffi::c_int,
                c"stop_children".as_ptr(),
                status_child.pid as ::core::ffi::c_long,
                status_child.stopped as ::core::ffi::c_int,
                status_child.stop_signal,
                status_child.cont_signal,
                status_child.click_events as ::core::ffi::c_int,
                status_child.click_events_init as ::core::ffi::c_int,
            );
        }
    } else {
        fprintf(
            stderr,
            c"[%s:%d] ERROR: %s: unknown child, this should never happen pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n".as_ptr(),
            c"../i3bar/src/child.c".as_ptr(),
            1022 as ::core::ffi::c_int,
            c"stop_children".as_ptr(),
            status_child.pid as ::core::ffi::c_long,
            status_child.stopped as ::core::ffi::c_int,
            status_child.stop_signal,
            status_child.cont_signal,
            status_child.click_events as ::core::ffi::c_int,
            status_child.click_events_init as ::core::ffi::c_int,
        );
    }
    if ws_child.pid == 0 as ::core::ffi::c_int {
        if config.verbose {
            printf(
                c"[%s:%d] %s: child pid = 0\n".as_ptr(),
                c"../i3bar/src/child.c".as_ptr(),
                1022 as ::core::ffi::c_int,
                c"stop_children".as_ptr(),
            );
        }
    } else if ws_child.pid == status_child.pid {
        if config.verbose {
            printf(
                c"[%s:%d] %s: status_command: pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n".as_ptr(),
                c"../i3bar/src/child.c".as_ptr(),
                1022 as ::core::ffi::c_int,
                c"stop_children".as_ptr(),
                ws_child.pid as ::core::ffi::c_long,
                ws_child.stopped as ::core::ffi::c_int,
                ws_child.stop_signal,
                ws_child.cont_signal,
                ws_child.click_events as ::core::ffi::c_int,
                ws_child.click_events_init as ::core::ffi::c_int,
            );
        }
    } else if ws_child.pid == ws_child.pid {
        if config.verbose {
            printf(
                c"[%s:%d] %s: workspace_command: pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n".as_ptr(),
                c"../i3bar/src/child.c".as_ptr(),
                1022 as ::core::ffi::c_int,
                c"stop_children".as_ptr(),
                ws_child.pid as ::core::ffi::c_long,
                ws_child.stopped as ::core::ffi::c_int,
                ws_child.stop_signal,
                ws_child.cont_signal,
                ws_child.click_events as ::core::ffi::c_int,
                ws_child.click_events_init as ::core::ffi::c_int,
            );
        }
    } else {
        fprintf(
            stderr,
            c"[%s:%d] ERROR: %s: unknown child, this should never happen pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n".as_ptr(),
            c"../i3bar/src/child.c".as_ptr(),
            1022 as ::core::ffi::c_int,
            c"stop_children".as_ptr(),
            ws_child.pid as ::core::ffi::c_long,
            ws_child.stopped as ::core::ffi::c_int,
            ws_child.stop_signal,
            ws_child.cont_signal,
            ws_child.click_events as ::core::ffi::c_int,
            ws_child.click_events_init as ::core::ffi::c_int,
        );
    }
    stop_child(&raw mut status_child);
    stop_child(&raw mut ws_child);
}

pub unsafe fn cont_children() {
    if status_child.pid == 0 as ::core::ffi::c_int {
        if config.verbose {
            printf(
                c"[%s:%d] %s: child pid = 0\n".as_ptr(),
                c"../i3bar/src/child.c".as_ptr(),
                1032 as ::core::ffi::c_int,
                c"cont_children".as_ptr(),
            );
        }
    } else if status_child.pid == status_child.pid {
        if config.verbose {
            printf(
                c"[%s:%d] %s: status_command: pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n".as_ptr(),
                c"../i3bar/src/child.c".as_ptr(),
                1032 as ::core::ffi::c_int,
                c"cont_children".as_ptr(),
                status_child.pid as ::core::ffi::c_long,
                status_child.stopped as ::core::ffi::c_int,
                status_child.stop_signal,
                status_child.cont_signal,
                status_child.click_events as ::core::ffi::c_int,
                status_child.click_events_init as ::core::ffi::c_int,
            );
        }
    } else if status_child.pid == ws_child.pid {
        if config.verbose {
            printf(
                c"[%s:%d] %s: workspace_command: pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n".as_ptr(),
                c"../i3bar/src/child.c".as_ptr(),
                1032 as ::core::ffi::c_int,
                c"cont_children".as_ptr(),
                status_child.pid as ::core::ffi::c_long,
                status_child.stopped as ::core::ffi::c_int,
                status_child.stop_signal,
                status_child.cont_signal,
                status_child.click_events as ::core::ffi::c_int,
                status_child.click_events_init as ::core::ffi::c_int,
            );
        }
    } else {
        fprintf(
            stderr,
            c"[%s:%d] ERROR: %s: unknown child, this should never happen pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n".as_ptr(),
            c"../i3bar/src/child.c".as_ptr(),
            1032 as ::core::ffi::c_int,
            c"cont_children".as_ptr(),
            status_child.pid as ::core::ffi::c_long,
            status_child.stopped as ::core::ffi::c_int,
            status_child.stop_signal,
            status_child.cont_signal,
            status_child.click_events as ::core::ffi::c_int,
            status_child.click_events_init as ::core::ffi::c_int,
        );
    }
    if ws_child.pid == 0 as ::core::ffi::c_int {
        if config.verbose {
            printf(
                c"[%s:%d] %s: child pid = 0\n".as_ptr(),
                c"../i3bar/src/child.c".as_ptr(),
                1032 as ::core::ffi::c_int,
                c"cont_children".as_ptr(),
            );
        }
    } else if ws_child.pid == status_child.pid {
        if config.verbose {
            printf(
                c"[%s:%d] %s: status_command: pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n".as_ptr(),
                c"../i3bar/src/child.c".as_ptr(),
                1032 as ::core::ffi::c_int,
                c"cont_children".as_ptr(),
                ws_child.pid as ::core::ffi::c_long,
                ws_child.stopped as ::core::ffi::c_int,
                ws_child.stop_signal,
                ws_child.cont_signal,
                ws_child.click_events as ::core::ffi::c_int,
                ws_child.click_events_init as ::core::ffi::c_int,
            );
        }
    } else if ws_child.pid == ws_child.pid {
        if config.verbose {
            printf(
                c"[%s:%d] %s: workspace_command: pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n".as_ptr(),
                c"../i3bar/src/child.c".as_ptr(),
                1032 as ::core::ffi::c_int,
                c"cont_children".as_ptr(),
                ws_child.pid as ::core::ffi::c_long,
                ws_child.stopped as ::core::ffi::c_int,
                ws_child.stop_signal,
                ws_child.cont_signal,
                ws_child.click_events as ::core::ffi::c_int,
                ws_child.click_events_init as ::core::ffi::c_int,
            );
        }
    } else {
        fprintf(
            stderr,
            c"[%s:%d] ERROR: %s: unknown child, this should never happen pid=%ld stopped=%d stop_signal=%d cont_signal=%d click_events=%d click_events_init=%d\n".as_ptr(),
            c"../i3bar/src/child.c".as_ptr(),
            1032 as ::core::ffi::c_int,
            c"cont_children".as_ptr(),
            ws_child.pid as ::core::ffi::c_long,
            ws_child.stopped as ::core::ffi::c_int,
            ws_child.stop_signal,
            ws_child.cont_signal,
            ws_child.click_events as ::core::ffi::c_int,
            ws_child.click_events_init as ::core::ffi::c_int,
        );
    }
    cont_child(&raw mut status_child);
    cont_child(&raw mut ws_child);
}

pub unsafe fn child_want_click_events() -> bool {
    status_child.click_events
}

unsafe extern "C" fn run_static_initializers() {
    statusline_head = statusline_head {
        tqh_first: ::core::ptr::null_mut::<status_block>(),
        tqh_last: &raw mut statusline_head.tqh_first,
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];

// Internal helpers use Rust linkage rather than foreign symbol lookup.
use crate::workspaces::free_workspaces;
use crate::workspaces::parse_workspaces_json;
use crate::xcb::draw_bars;
use i3_common::dpi::logical_px;
use i3_common::safewrappers::scalloc;
use i3_common::safewrappers::smalloc;
use i3_common::safewrappers::sstrdup;
use i3_common::safewrappers::writeall;

// Canonical Cairo backend ABI; renderer logic remains local.

use xcb_sys::{
    xcb_window_t, XCB_MOD_MASK_1, XCB_MOD_MASK_2, XCB_MOD_MASK_3, XCB_MOD_MASK_4, XCB_MOD_MASK_5,
    XCB_MOD_MASK_CONTROL, XCB_MOD_MASK_SHIFT,
};

pub use libc::{
    __errno_location, close, dup2, execl, exit, fcntl, fork, fprintf, free, killpg, memcpy,
    memmove, memset, pipe, printf, read, setpgid, strcasecmp, strerror, strlen, strncasecmp,
    strncmp, waitpid,
};

use i3_common::native_stdio::{__ctype_b_loc, err, stderr, vasprintf};

// Internal calls and globals use canonical Rust identities.
use crate::application::main_loop;
use crate::xcb::separator_symbol_width;

use i3_common::string::I3String;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum BlockAlign {
    Left = 0,
    Center = 1,
    Right = 2,
}

#[cfg(test)]
mod owned_tests {
    use super::*;
    #[test]
    fn open_status_array_handles_every_fragment_boundary_and_raw_string_distinctions() {
        let input = br#"[[{"full_text":"a\u0000b","name":"first\u0000ignored","urgent":true,"min_width":-1,"markup":"PaNgO","align":"RIGHT"}],[{"full_text":"second","full_text":"last"}]"#;
        for boundary in 0..=input.len() {
            let mut parser = StatusParser::new(false);
            let (mut frames, _, result) = parser.feed(&input[..boundary], 9);
            result.unwrap();
            let (tail, urgent, result) = parser.feed(&input[boundary..], 9);
            result.unwrap();
            frames.extend(tail);
            assert_eq!(frames.len(), 2, "boundary {boundary}");
            assert_eq!(frames[0][0].full.as_deref(), Some(&b"a\0b"[..]));
            assert_eq!(frames[0][0].name.as_deref(), Some(&b"first"[..]));
            assert_eq!(frames[0][0].minimum, u32::MAX);
            assert!(frames[0][0].markup);
            assert_eq!(frames[0][0].align, BlockAlign::Left);
            assert_eq!(frames[0][0].borders, [1; 4]);
            assert_eq!(frames[0][0].separator, 9);
            assert_eq!(frames[1][0].full.as_deref(), Some(&b"last"[..]));
            assert!(
                urgent,
                "legacy urgency remains set after later nonurgent frame"
            );
        }
    }
    #[test]
    fn valid_prefix_frames_survive_bad_tail_and_integer_overflow_remains_canceled() {
        let mut parser = StatusParser::new(false);
        let (frames, _, result) = parser.feed(br#"[[{"full_text":"valid"}],?"#, 9);
        assert_eq!(frames.len(), 1);
        assert!(result.is_err());
        let mut parser = StatusParser::new(false);
        let (_, _, result) = parser.feed(br#"[[{"min_width":-9223372036854775808}]]"#, 9);
        let error = result.unwrap_err();
        assert_eq!(error.kind, i3_json::ParseErrorKind::Canceled);
        assert_eq!(error.message, "parse error: integer overflow");
        let mut parser = StatusParser::new(true);
        let (frames, urgent, result) =
            parser.feed(br#"[[{"full_text":"ok","min_width":2.5}]]"#, 12);
        result.unwrap();
        assert!(urgent);
        assert_eq!(frames.last().unwrap()[0].minimum, 0);
    }
    #[test]
    fn workspace_line_cache_handles_fragments_empty_lines_and_nul_projection() {
        let mut state = WorkspaceLines::default();
        assert!(state.feed(b"one").is_empty());
        assert_eq!(state.feed(b"-two\n\npart"), [b"one-two".to_vec()]);
        assert_eq!(state.last.as_deref(), Some(&b"one-two"[..]));
        assert_eq!(state.feed(b"ial\nlast\0ignored\n"), [b"partial".to_vec()]);
        assert_eq!(state.pending.as_deref(), Some(&b"last"[..]));
        assert_eq!(state.feed(b"\n"), [b"last".to_vec()]);
        assert_eq!(state.last.as_deref(), Some(&b"last"[..]));
        assert!(state.feed(b"\0ignored").is_empty());
        assert_eq!(flat_text(b"raw\xff\r\n"), b"raw\xff\r");
        assert_eq!(flat_text(b"one\0two\n"), b"one");
    }
    #[test]
    fn read_drain_keeps_eof_discard_and_fatal_eintr_policy() {
        struct Reader(std::collections::VecDeque<std::io::Result<Vec<u8>>>);
        impl std::io::Read for Reader {
            fn read(&mut self, out: &mut [u8]) -> std::io::Result<usize> {
                match self.0.pop_front().unwrap() {
                    Ok(bytes) => {
                        out[..bytes.len()].copy_from_slice(&bytes);
                        Ok(bytes.len())
                    }
                    Err(error) => Err(error),
                }
            }
        }
        let blocked = || Err(std::io::Error::from(std::io::ErrorKind::WouldBlock));
        assert_eq!(
            read_available(&mut Reader(
                [Ok(b"ab".to_vec()), Ok(b"cd".to_vec()), blocked()].into()
            ))
            .unwrap(),
            InputChunk::Bytes(b"abcd".to_vec())
        );
        assert_eq!(
            read_available(&mut Reader([Ok(b"discard".to_vec()), Ok(vec![])].into())).unwrap(),
            InputChunk::Eof
        );
        assert_eq!(
            read_available(&mut Reader([Ok(b"\0ignored".to_vec()), blocked()].into())).unwrap(),
            InputChunk::Empty
        );
        let error = read_available(&mut Reader(
            [Err(std::io::Error::from_raw_os_error(libc::EINTR))].into(),
        ))
        .unwrap_err();
        assert_eq!(error.raw_os_error(), Some(libc::EINTR));
    }
    #[test]
    fn click_generator_keeps_open_array_order_and_invalid_utf8_rejection() {
        let mut generator = i3_json::Generator::new();
        generator.start_array().unwrap();
        assert_eq!(generator.bytes(), b"[");
        generator.clear();
        append_click(
            &mut generator,
            ClickValue {
                name: Some(b"test"),
                instance: None,
                button: 1,
                position: [-1, 2, 3, 4, 5, 6, 7, 8],
                modifiers: XCB_MOD_MASK_SHIFT as i32 | XCB_MOD_MASK_4 as i32,
            },
        );
        assert_eq!(generator.bytes(), br#"{"name":"test","button":1,"modifiers":["Shift","Mod4"],"x":-1,"y":2,"relative_x":3,"relative_y":4,"output_x":5,"output_y":6,"width":7,"height":8}"#);
        generator.clear();
        append_click(
            &mut generator,
            ClickValue {
                name: None,
                instance: None,
                button: 2,
                position: [0; 8],
                modifiers: 0,
            },
        );
        assert!(generator.bytes().starts_with(b",{"));
        let mut invalid = i3_json::Generator::new();
        invalid.start_array().unwrap();
        invalid.clear();
        append_click(
            &mut invalid,
            ClickValue {
                name: Some(b"\xff"),
                instance: None,
                button: 1,
                position: [0; 8],
                modifiers: 0,
            },
        );
        assert_eq!(invalid.bytes(), br#"{"name":"button","modifiers":[],"x":0,"y":0,"relative_x":0,"relative_y":0,"output_x":0,"output_y":0,"width":0,"height":0}"#);
    }
}
