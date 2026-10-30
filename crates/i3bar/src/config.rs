/* i3bar - Parses the configuration received from i3. See LICENSE. */
//! Parsing produces owned, ordered updates. Applying those updates to the
//! legacy event-thread configuration is an explicit unsafe boundary.
pub use crate::application::{
    binding_t, bindings_head, config_t, tray_output_t, tray_outputs_head,
};
use crate::application::{BarDisplayMode, BarPosition, HiddenState, Rect};
use crate::application::{C2RustUnnamed_0, C2RustUnnamed_1};
use i3_common::safewrappers::srealloc;
use i3_common::string::I3String;
use i3_json::{Event, EventHandler, ParseError, ParseErrorKind, Parser};
use libc::free;
use std::cell::RefCell;
use std::ffi::CString;
use std::ptr;
use std::rc::Rc;
use std::sync::Mutex;
use xcb_sys::xcb_color_strings_t;

/// Intrusive nodes are owned by exactly one configuration list. Only the
/// explicit event-thread append/clear functions transfer or recover Boxes.
pub struct Binding {
    pub input_code: i32,
    pub command: Option<CString>,
    pub release: bool,
    pub bindings: C2RustUnnamed_0,
}
impl Binding {
    fn new(input_code: i32) -> Self {
        Self {
            input_code,
            command: None,
            release: false,
            bindings: C2RustUnnamed_0 {
                tqe_next: ptr::null_mut(),
                tqe_prev: ptr::null_mut(),
            },
        }
    }
    pub fn command_ptr(&self) -> *const libc::c_char {
        self.command
            .as_deref()
            .map_or(ptr::null(), std::ffi::CStr::as_ptr)
    }
}
pub struct TrayOutput {
    pub output: CString,
    pub tray_outputs: C2RustUnnamed_1,
}
impl TrayOutput {
    fn new(output: CString) -> Self {
        Self {
            output,
            tray_outputs: C2RustUnnamed_1 {
                tqe_next: ptr::null_mut(),
                tqe_prev: ptr::null_mut(),
            },
        }
    }
    pub fn output_ptr(&self) -> *const libc::c_char {
        self.output.as_ptr()
    }
}

pub static mut config: config_t = empty_config();
pub(crate) const fn empty_config() -> config_t {
    config_t {
        modifier: 0,
        bindings: bindings_head {
            tqh_first: ::core::ptr::null::<binding_t>() as *mut binding_t,
            tqh_last: ::core::ptr::null::<*mut binding_t>() as *mut *mut binding_t,
        },
        position: crate::application::BarPosition::None,
        verbose: false,
        bar_height: 0,
        padding: Rect {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        transparency: false,
        colors: xcb_color_strings_t {
            bar_fg: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            bar_bg: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            sep_fg: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            focus_bar_fg: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            focus_bar_bg: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            focus_sep_fg: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            active_ws_fg: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            active_ws_bg: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            active_ws_border: ::core::ptr::null::<::core::ffi::c_char>()
                as *mut ::core::ffi::c_char,
            inactive_ws_fg: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            inactive_ws_bg: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            inactive_ws_border: ::core::ptr::null::<::core::ffi::c_char>()
                as *mut ::core::ffi::c_char,
            focus_ws_bg: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            focus_ws_fg: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            focus_ws_border: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            urgent_ws_bg: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            urgent_ws_fg: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            urgent_ws_border: ::core::ptr::null::<::core::ffi::c_char>()
                as *mut ::core::ffi::c_char,
            binding_mode_bg: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            binding_mode_fg: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            binding_mode_border: ::core::ptr::null::<::core::ffi::c_char>()
                as *mut ::core::ffi::c_char,
        },
        disable_binding_mode_indicator: false,
        disable_ws: false,
        ws_min_width: 0,
        strip_ws_numbers: false,
        strip_ws_name: false,
        bar_id: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        command: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        workspace_command: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        fontname: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        separator_symbol: None,
        tray_outputs: tray_outputs_head {
            tqh_first: ::core::ptr::null::<tray_output_t>() as *mut tray_output_t,
            tqh_last: ::core::ptr::null::<*mut tray_output_t>() as *mut *mut tray_output_t,
        },
        tray_padding: 0,
        num_outputs: 0,
        outputs: ::core::ptr::null::<*mut ::core::ffi::c_char>() as *mut *mut ::core::ffi::c_char,
        hide_on_modifier: crate::application::BarDisplayMode::Dock,
        hidden_state: crate::application::HiddenState::Hide,
    }
}

#[derive(Debug, Clone, Copy)]
enum StringField {
    Command,
    WorkspaceCommand,
    Font,
    BarFg,
    BarBg,
    SepFg,
    FocusBarFg,
    FocusBarBg,
    FocusSepFg,
    FocusWsBorder,
    FocusWsBg,
    FocusWsFg,
    ActiveWsBorder,
    ActiveWsBg,
    ActiveWsFg,
    InactiveWsBorder,
    InactiveWsBg,
    InactiveWsFg,
    UrgentWsBorder,
    UrgentWsBg,
    UrgentWsFg,
    BindingModeBorder,
    BindingModeBg,
    BindingModeFg,
}
#[derive(Debug, Clone, Copy)]
enum BooleanField {
    BindingIndicator,
    WorkspaceButtons,
    StripNumbers,
    StripName,
    Verbose,
}
#[derive(Debug, Clone, Copy)]
enum IntegerField {
    PaddingX,
    PaddingY,
    PaddingWidth,
    PaddingHeight,
    BarHeight,
    TrayPadding,
    Modifier,
    WorkspaceMinWidth,
}
enum Update {
    String(StringField, CString),
    Boolean(BooleanField, bool),
    Integer(IntegerField, i64),
    Mode(BarDisplayMode),
    HiddenState(HiddenState),
    Position(BarPosition),
    Separator(I3String),
    Output(CString),
    Binding(Box<Binding>),
    Tray(Box<TrayOutput>),
    Log {
        label: &'static str,
        value: Vec<u8>,
        length: usize,
    },
}
#[derive(Clone, Default)]
struct ReaderState {
    key: Vec<u8>,
    bindings: bool,
    tray_outputs: bool,
    padding: bool,
}
// This state intentionally survives accepted partial replies, as the original
// callback keys/flags did. It is separate from the event-thread configuration.
static READER_STATE: Mutex<ReaderState> = Mutex::new(ReaderState {
    key: Vec::new(),
    bindings: false,
    tray_outputs: false,
    padding: false,
});
struct ConfigPatch {
    updates: Vec<Update>,
    state: ReaderState,
}
struct ConfigReader {
    patch: ConfigPatch,
    last_binding: Option<usize>,
    error: Option<&'static str>,
}
struct SharedReader(Rc<RefCell<ConfigReader>>);
fn prefix(bytes: &[u8]) -> &[u8] {
    bytes.split(|byte| *byte == 0).next().unwrap()
}
fn owned_string(bytes: &[u8]) -> CString {
    CString::new(prefix(bytes)).expect("NUL-free prefix")
}
fn modifier(value: &[u8]) -> u32 {
    match value {
        b"none" => 0,
        b"shift" => 1,
        b"ctrl" => 4,
        b"Mod1" => 8,
        b"Mod2" => 16,
        b"Mod3" => 32,
        b"Mod5" => 128,
        _ => 64,
    }
}
fn string_field(key: &[u8]) -> Option<StringField> {
    Some(match key {
        b"status_command" => StringField::Command,
        b"workspace_command" => StringField::WorkspaceCommand,
        b"font" => StringField::Font,
        b"statusline" => StringField::BarFg,
        b"background" => StringField::BarBg,
        b"separator" => StringField::SepFg,
        b"focused_statusline" => StringField::FocusBarFg,
        b"focused_background" => StringField::FocusBarBg,
        b"focused_separator" => StringField::FocusSepFg,
        b"focused_workspace_border" => StringField::FocusWsBorder,
        b"focused_workspace_bg" => StringField::FocusWsBg,
        b"focused_workspace_text" => StringField::FocusWsFg,
        b"active_workspace_border" => StringField::ActiveWsBorder,
        b"active_workspace_bg" => StringField::ActiveWsBg,
        b"active_workspace_text" => StringField::ActiveWsFg,
        b"inactive_workspace_border" => StringField::InactiveWsBorder,
        b"inactive_workspace_bg" => StringField::InactiveWsBg,
        b"inactive_workspace_text" => StringField::InactiveWsFg,
        b"urgent_workspace_border" => StringField::UrgentWsBorder,
        b"urgent_workspace_bg" => StringField::UrgentWsBg,
        b"urgent_workspace_text" => StringField::UrgentWsFg,
        b"binding_mode_border" => StringField::BindingModeBorder,
        b"binding_mode_bg" => StringField::BindingModeBg,
        b"binding_mode_text" => StringField::BindingModeFg,
        _ => return None,
    })
}
impl ConfigReader {
    fn fail(&mut self, message: &'static str) -> bool {
        self.error = Some(message);
        false
    }
    fn log(&mut self, label: &'static str, value: &[u8]) {
        self.patch.updates.push(Update::Log {
            label,
            value: prefix(value).to_vec(),
            length: value.len(),
        });
    }
    fn binding_mut(&mut self) -> Option<&mut Binding> {
        match self.patch.updates.get_mut(self.last_binding?)? {
            Update::Binding(binding) => Some(binding),
            _ => unreachable!(),
        }
    }
    fn add_binding(&mut self, binding: Binding) {
        self.last_binding = Some(self.patch.updates.len());
        self.patch.updates.push(Update::Binding(Box::new(binding)));
    }
    fn string(&mut self, value: &[u8]) -> bool {
        let key = self.patch.state.key.clone();
        if key == b"id" || key == b"socket_path" {
            return true;
        }
        if self.patch.state.bindings {
            if key != b"command" {
                return self.fail("Unknown key while parsing bar bindings");
            }
            let Some(binding) = self.binding_mut() else {
                return self.fail("There is no binding to put the current command onto");
            };
            if binding.command.is_some() {
                return self.fail("The binding already has a command");
            }
            binding.command = Some(owned_string(value));
            return true;
        }
        if self.patch.state.tray_outputs {
            self.log("Adding tray_output", value);
            self.patch
                .updates
                .push(Update::Tray(Box::new(TrayOutput::new(owned_string(value)))));
            return true;
        }
        let update = match key.as_slice() {
            b"mode" => {
                self.log("mode", value);
                Update::Mode(match value {
                    b"dock" => BarDisplayMode::Dock,
                    b"hide" => BarDisplayMode::Hide,
                    _ => BarDisplayMode::Invisible,
                })
            }
            b"hidden_state" => {
                self.log("hidden_state", value);
                Update::HiddenState(if value == b"hide" {
                    HiddenState::Hide
                } else {
                    HiddenState::Show
                })
            }
            b"modifier" => {
                self.log("modifier", value);
                Update::Integer(IntegerField::Modifier, modifier(value).into())
            }
            b"position" => {
                self.log("position", value);
                Update::Position(if value == b"top" {
                    BarPosition::Top
                } else {
                    BarPosition::Bottom
                })
            }
            b"separator_symbol" => {
                self.log("separator_symbol", value);
                Update::Separator(I3String::from_bytes_with_legacy_length(value))
            }
            b"outputs" => {
                self.log("+output", value);
                Update::Output(owned_string(value))
            }
            b"tray_output" => {
                self.log("Found deprecated key tray_output", value);
                Update::Tray(Box::new(TrayOutput::new(owned_string(value))))
            }
            b"wheel_up_cmd" | b"wheel_down_cmd" => {
                self.log(
                    if key == b"wheel_up_cmd" {
                        "wheel_up_cmd"
                    } else {
                        "wheel_down_cmd"
                    },
                    value,
                );
                let mut binding = Binding::new(if key == b"wheel_up_cmd" { 4 } else { 5 });
                binding.command = Some(owned_string(value));
                self.add_binding(binding);
                return true;
            }
            _ => {
                let Some(field) = string_field(&key) else {
                    return self.fail("Unexpected config string");
                };
                // Field names are static; diagnostics preserve update order.
                self.log(string_label(field), value);
                Update::String(field, owned_string(value))
            }
        };
        self.patch.updates.push(update);
        true
    }
    fn boolean(&mut self, value: bool) -> bool {
        let key = self.patch.state.key.clone();
        if self.patch.state.bindings && key == b"release" {
            let Some(binding) = self.binding_mut() else {
                return self.fail("There is no binding to put release onto");
            };
            binding.release = value;
            return true;
        }
        let field = match key.as_slice() {
            b"binding_mode_indicator" => BooleanField::BindingIndicator,
            b"workspace_buttons" => BooleanField::WorkspaceButtons,
            b"strip_workspace_numbers" => BooleanField::StripNumbers,
            b"strip_workspace_name" => BooleanField::StripName,
            b"verbose" => BooleanField::Verbose,
            _ => return self.fail("Unknown config boolean"),
        };
        let label = match field {
            BooleanField::BindingIndicator => "binding_mode_indicator",
            BooleanField::WorkspaceButtons => "workspace_buttons",
            BooleanField::StripNumbers => "strip_workspace_numbers",
            BooleanField::StripName => "strip_workspace_name",
            BooleanField::Verbose => "verbose",
        };
        if !matches!(field, BooleanField::Verbose) {
            self.log(label, if value { b"1" } else { b"0" });
        }
        self.patch.updates.push(Update::Boolean(field, value));
        true
    }
    fn integer(&mut self, value: i64) -> bool {
        let key = self.patch.state.key.clone();
        if self.patch.state.bindings {
            if key != b"input_code" {
                return self.fail("Unknown integer key while parsing bar bindings");
            }
            self.add_binding(Binding::new(value as i32));
            return true;
        }
        let field = match key.as_slice() {
            b"x" if self.patch.state.padding => IntegerField::PaddingX,
            b"y" if self.patch.state.padding => IntegerField::PaddingY,
            b"width" if self.patch.state.padding => IntegerField::PaddingWidth,
            b"height" if self.patch.state.padding => IntegerField::PaddingHeight,
            b"bar_height" => IntegerField::BarHeight,
            b"tray_padding" => IntegerField::TrayPadding,
            b"modifier" => IntegerField::Modifier,
            b"workspace_min_width" => IntegerField::WorkspaceMinWidth,
            _ => return self.fail("Unknown config integer"),
        };
        let label = match field {
            IntegerField::PaddingX => "padding.x",
            IntegerField::PaddingY => "padding.y",
            IntegerField::PaddingWidth => "padding.width",
            IntegerField::PaddingHeight => "padding.height",
            IntegerField::BarHeight => "bar_height",
            IntegerField::TrayPadding => "tray_padding",
            IntegerField::Modifier => "modifier_integer",
            IntegerField::WorkspaceMinWidth => "workspace_min_width",
        };
        self.log(label, value.to_string().as_bytes());
        self.patch.updates.push(Update::Integer(field, value));
        true
    }
}
fn string_label(field: StringField) -> &'static str {
    match field {
        StringField::Command => "status_command",
        StringField::WorkspaceCommand => "workspace_command",
        StringField::Font => "font",
        StringField::BarFg => "statusline",
        StringField::BarBg => "background",
        StringField::SepFg => "separator",
        StringField::FocusBarFg => "focused_statusline",
        StringField::FocusBarBg => "focused_background",
        StringField::FocusSepFg => "focused_separator",
        StringField::FocusWsBorder => "focused_workspace_border",
        StringField::FocusWsBg => "focused_workspace_bg",
        StringField::FocusWsFg => "focused_workspace_text",
        StringField::ActiveWsBorder => "active_workspace_border",
        StringField::ActiveWsBg => "active_workspace_bg",
        StringField::ActiveWsFg => "active_workspace_text",
        StringField::InactiveWsBorder => "inactive_workspace_border",
        StringField::InactiveWsBg => "inactive_workspace_bg",
        StringField::InactiveWsFg => "inactive_workspace_text",
        StringField::UrgentWsBorder => "urgent_workspace_border",
        StringField::UrgentWsBg => "urgent_workspace_bg",
        StringField::UrgentWsFg => "urgent_workspace_text",
        StringField::BindingModeBorder => "binding_mode_border",
        StringField::BindingModeBg => "binding_mode_bg",
        StringField::BindingModeFg => "binding_mode_text",
    }
}
// Keep historical verbose messages, including the original decoded length
// in mode/state messages, without passing borrowed JSON buffers to printf.
fn write_log(output: &mut impl std::io::Write, label: &str, value: &[u8], length: usize) {
    let (line, before, after, include_length) = match label {
        "Adding tray_output" => (105, "Adding tray_output = ", " to the list.\n", false),
        "mode" => (113, "mode = ", "\n", true),
        "hidden_state" => (121, "hidden_state = ", "\n", true),
        "modifier" => (128, "modifier = ", "\n", false),
        "wheel_up_cmd" => (166, "wheel_up_cmd = ", "\n", false),
        "wheel_down_cmd" => (177, "wheel_down_cmd = ", "\n", false),
        "position" => (186, "position = ", "\n", false),
        "status_command" => (192, "status_command = ", "\n", false),
        "workspace_command" => (198, "workspace_command = ", "\n", false),
        "font" => (204, "font = ", "\n", false),
        "separator_symbol" => (211, "separator = ", "\n", false),
        "+output" => (218, "+output ", "\n", false),
        "Found deprecated key tray_output" => {
            (229, "Found deprecated key tray_output ", ".\n", false)
        }
        "statusline" => (245, "statusline = bar_fg = ", "\n", false),
        "background" => (246, "background = bar_bg = ", "\n", false),
        "separator" => (247, "separator = sep_fg = ", "\n", false),
        "focused_statusline" => (248, "focused_statusline = focus_bar_fg = ", "\n", false),
        "focused_background" => (249, "focused_background = focus_bar_bg = ", "\n", false),
        "focused_separator" => (250, "focused_separator = focus_sep_fg = ", "\n", false),
        "focused_workspace_border" => (
            251,
            "focused_workspace_border = focus_ws_border = ",
            "\n",
            false,
        ),
        "focused_workspace_bg" => (252, "focused_workspace_bg = focus_ws_bg = ", "\n", false),
        "focused_workspace_text" => (253, "focused_workspace_text = focus_ws_fg = ", "\n", false),
        "active_workspace_border" => (
            254,
            "active_workspace_border = active_ws_border = ",
            "\n",
            false,
        ),
        "active_workspace_bg" => (255, "active_workspace_bg = active_ws_bg = ", "\n", false),
        "active_workspace_text" => (256, "active_workspace_text = active_ws_fg = ", "\n", false),
        "inactive_workspace_border" => (
            257,
            "inactive_workspace_border = inactive_ws_border = ",
            "\n",
            false,
        ),
        "inactive_workspace_bg" => (
            258,
            "inactive_workspace_bg = inactive_ws_bg = ",
            "\n",
            false,
        ),
        "inactive_workspace_text" => (
            259,
            "inactive_workspace_text = inactive_ws_fg = ",
            "\n",
            false,
        ),
        "urgent_workspace_border" => (
            260,
            "urgent_workspace_border = urgent_ws_border = ",
            "\n",
            false,
        ),
        "urgent_workspace_bg" => (261, "urgent_workspace_bg = urgent_ws_bg = ", "\n", false),
        "urgent_workspace_text" => (262, "urgent_workspace_text = urgent_ws_fg = ", "\n", false),
        "binding_mode_border" => (
            263,
            "binding_mode_border = binding_mode_border = ",
            "\n",
            false,
        ),
        "binding_mode_bg" => (264, "binding_mode_bg = binding_mode_bg = ", "\n", false),
        "binding_mode_text" => (265, "binding_mode_text = binding_mode_fg = ", "\n", false),
        "binding_mode_indicator" => (293, "binding_mode_indicator = ", "\n", false),
        "workspace_buttons" => (299, "workspace_buttons = ", "\n", false),
        "strip_workspace_numbers" => (305, "strip_workspace_numbers = ", "\n", false),
        "strip_workspace_name" => (311, "strip_workspace_name = ", "\n", false),
        "padding.x" => (347, "padding.x = ", "\n", false),
        "padding.y" => (352, "padding.y = ", "\n", false),
        "padding.width" => (357, "padding.width = ", "\n", false),
        "padding.height" => (362, "padding.height = ", "\n", false),
        "bar_height" => (369, "bar_height = ", "\n", false),
        "tray_padding" => (375, "tray_padding = ", "\n", false),
        "modifier_integer" => (381, "modifier = ", "\n", false),
        "workspace_min_width" => (387, "workspace_min_width = ", "\n", false),
        _ => unreachable!("typed diagnostic label"),
    };
    let _ = write!(output, "[../i3bar/src/config.c:{line}] {before}");
    let _ = output.write_all(value);
    if include_length {
        let _ = write!(output, ", len = {}", length as i32);
    }
    let _ = output.write_all(after.as_bytes());
}
impl EventHandler for SharedReader {
    fn on_event(&mut self, event: Event<'_>) -> bool {
        let mut reader = self.0.borrow_mut();
        match event {
            Event::MapKey(key) => {
                let state = &mut reader.patch.state;
                state.key = prefix(key).to_vec();
                match state.key.as_slice() {
                    b"bindings" => state.bindings = true,
                    b"tray_outputs" => state.tray_outputs = true,
                    b"padding" => state.padding = true,
                    _ => {}
                }
            }
            Event::EndArray => {
                let state = &mut reader.patch.state;
                state.bindings = false;
                state.tray_outputs = false;
                state.padding = false;
            }
            Event::Null if reader.patch.state.key == b"id" => {
                return reader.fail("No such bar config. Configure a bar block in your i3 config")
            }
            Event::String(value) => return reader.string(value),
            Event::Boolean(value) => return reader.boolean(value),
            Event::Number(value) if !value.contains(['.', 'e', 'E']) => {
                let Ok(value) = value.parse::<i64>() else {
                    return reader.fail("Config integer exceeds signed callback range");
                };
                if value == i64::MIN {
                    return reader.fail("Config integer exceeds signed callback range");
                }
                return reader.integer(value);
            }
            _ => {}
        }
        true
    }
}
fn read_config(json: &[u8], state: ReaderState) -> Result<ConfigPatch, ParseError> {
    let reader = Rc::new(RefCell::new(ConfigReader {
        patch: ConfigPatch {
            updates: Vec::new(),
            state,
        },
        last_binding: None,
        error: None,
    }));
    let mut parser = Parser::new(SharedReader(Rc::clone(&reader)));
    parser.options_mut().allow_legacy_whitespace = true;
    let result = parser.parse(json);
    drop(parser);
    let reader = Rc::try_unwrap(reader)
        .ok()
        .expect("parser released reader")
        .into_inner();
    if let Some(message) = reader.error {
        return Err(ParseError {
            kind: ParseErrorKind::Canceled,
            offset: 0,
            message: message.into(),
        });
    }
    result?;
    Ok(reader.patch)
}

// CString allocations cannot be handed to libc::free. The remaining legacy
// config string slots deliberately receive native strdup allocations instead.
unsafe fn store_native(slot: *mut *mut libc::c_char, value: &CString) {
    *slot = libc::strdup(value.as_ptr());
    if (*slot).is_null() {
        std::alloc::handle_alloc_error(std::alloc::Layout::new::<u8>());
    }
}

/// # Safety
/// config's intrusive heads exclusively own distinct Box nodes created here;
/// no native caller or iterator borrows these nodes during replacement.
unsafe fn clear_config_lists(target: *mut config_t) {
    let mut binding = (*target).bindings.tqh_first;
    (*target).bindings.tqh_first = ptr::null_mut();
    (*target).bindings.tqh_last = &raw mut (*target).bindings.tqh_first;
    while !binding.is_null() {
        let next = (*binding).bindings.tqe_next;
        drop(Box::from_raw(binding));
        binding = next;
    }
    let mut output = (*target).tray_outputs.tqh_first;
    (*target).tray_outputs.tqh_first = ptr::null_mut();
    (*target).tray_outputs.tqh_last = &raw mut (*target).tray_outputs.tqh_first;
    while !output.is_null() {
        let next = (*output).tray_outputs.tqe_next;
        drop(Box::from_raw(output));
        output = next;
    }
}
unsafe fn append_binding(target: *mut config_t, mut binding: Box<Binding>) {
    binding.bindings.tqe_prev = (*target).bindings.tqh_last;
    let binding = Box::into_raw(binding);
    *(*target).bindings.tqh_last = binding;
    (*target).bindings.tqh_last = &raw mut (*binding).bindings.tqe_next;
}
unsafe fn append_tray(target: *mut config_t, mut output: Box<TrayOutput>) {
    output.tray_outputs.tqe_prev = (*target).tray_outputs.tqh_last;
    let output = Box::into_raw(output);
    *(*target).tray_outputs.tqh_last = output;
    (*target).tray_outputs.tqh_last = &raw mut (*output).tray_outputs.tqe_next;
}
/// # Safety
/// The caller exclusively owns the event-thread config graph. Native font and
/// command consumers must not retain slots replaced by these updates. IPC's
/// detached old command snapshots are never freed by this function.
unsafe fn apply_config(target: *mut config_t, updates: Vec<Update>) {
    clear_config_lists(target);
    for update in updates {
        match update {
            Update::String(field, value) => {
                let slot = match field {
                    StringField::Command => &raw mut (*target).command,
                    StringField::WorkspaceCommand => &raw mut (*target).workspace_command,
                    StringField::Font => {
                        free((*target).fontname.cast());
                        &raw mut (*target).fontname
                    }
                    StringField::BarFg => &raw mut (*target).colors.bar_fg,
                    StringField::BarBg => &raw mut (*target).colors.bar_bg,
                    StringField::SepFg => &raw mut (*target).colors.sep_fg,
                    StringField::FocusBarFg => &raw mut (*target).colors.focus_bar_fg,
                    StringField::FocusBarBg => &raw mut (*target).colors.focus_bar_bg,
                    StringField::FocusSepFg => &raw mut (*target).colors.focus_sep_fg,
                    StringField::FocusWsBorder => &raw mut (*target).colors.focus_ws_border,
                    StringField::FocusWsBg => &raw mut (*target).colors.focus_ws_bg,
                    StringField::FocusWsFg => &raw mut (*target).colors.focus_ws_fg,
                    StringField::ActiveWsBorder => &raw mut (*target).colors.active_ws_border,
                    StringField::ActiveWsBg => &raw mut (*target).colors.active_ws_bg,
                    StringField::ActiveWsFg => &raw mut (*target).colors.active_ws_fg,
                    StringField::InactiveWsBorder => &raw mut (*target).colors.inactive_ws_border,
                    StringField::InactiveWsBg => &raw mut (*target).colors.inactive_ws_bg,
                    StringField::InactiveWsFg => &raw mut (*target).colors.inactive_ws_fg,
                    StringField::UrgentWsBorder => &raw mut (*target).colors.urgent_ws_border,
                    StringField::UrgentWsBg => &raw mut (*target).colors.urgent_ws_bg,
                    StringField::UrgentWsFg => &raw mut (*target).colors.urgent_ws_fg,
                    StringField::BindingModeBorder => &raw mut (*target).colors.binding_mode_border,
                    StringField::BindingModeBg => &raw mut (*target).colors.binding_mode_bg,
                    StringField::BindingModeFg => &raw mut (*target).colors.binding_mode_fg,
                };
                store_native(slot, &value);
            }
            Update::Boolean(field, value) => match field {
                BooleanField::BindingIndicator => (*target).disable_binding_mode_indicator = !value,
                BooleanField::WorkspaceButtons => (*target).disable_ws = !value,
                BooleanField::StripNumbers => (*target).strip_ws_numbers = value,
                BooleanField::StripName => (*target).strip_ws_name = value,
                BooleanField::Verbose => (*target).verbose |= value,
            },
            Update::Integer(field, value) => match field {
                IntegerField::PaddingX => (*target).padding.x = value as u32,
                IntegerField::PaddingY => (*target).padding.y = value as u32,
                IntegerField::PaddingWidth => (*target).padding.width = value as u32,
                IntegerField::PaddingHeight => (*target).padding.height = value as u32,
                IntegerField::BarHeight => (*target).bar_height = value as u32,
                IntegerField::TrayPadding => (*target).tray_padding = value as i32,
                IntegerField::Modifier => (*target).modifier = value as u32,
                IntegerField::WorkspaceMinWidth => (*target).ws_min_width = value as i32,
            },
            Update::Mode(value) => (*target).hide_on_modifier = value,
            Update::HiddenState(value) => (*target).hidden_state = value,
            Update::Position(value) => (*target).position = value,
            Update::Separator(value) => {
                (*target).separator_symbol = Some(value);
            }
            Update::Binding(binding) => append_binding(target, binding),
            Update::Tray(output) => append_tray(target, output),
            Update::Output(value) => {
                let count = (*target)
                    .num_outputs
                    .checked_add(1)
                    .expect("too many configured outputs");
                let bytes = (count as usize)
                    .checked_mul(std::mem::size_of::<*mut libc::c_char>())
                    .expect("output array exceeds address space");
                (*target).outputs = srealloc((*target).outputs.cast(), bytes).cast();
                store_native(
                    (*target).outputs.add((*target).num_outputs as usize),
                    &value,
                );
                (*target).num_outputs = count;
            }
            Update::Log {
                label,
                value,
                length,
            } if (*target).verbose => {
                write_log(&mut std::io::stdout().lock(), label, &value, length);
            }
            _ => {}
        }
    }
    if (*target).disable_ws && !(*target).workspace_command.is_null() {
        libc::fprintf(i3_common::native_stdio::stderr,c"[../i3bar/src/config.c:428] ERROR: You have specified 'workspace_buttons no'. Your 'workspace_command %s' will be ignored.\n".as_ptr(),(*target).workspace_command);
        free((*target).workspace_command.cast());
        (*target).workspace_command = ptr::null_mut();
    }
}
/// # Safety
/// json points to size readable bytes. The caller owns the native event-thread
/// configuration and no reentrant callback may modify it while applying updates.
pub unsafe fn parse_config_json(json: *const u8, size: usize) {
    let json = if size == 0 {
        &[]
    } else {
        std::slice::from_raw_parts(json, size)
    };
    let state = READER_STATE
        .lock()
        .expect("config reader state poisoned")
        .clone();
    match read_config(json, state) {
        Ok(patch) => {
            apply_config(&raw mut config, patch.updates);
            *READER_STATE.lock().expect("config reader state poisoned") = patch.state;
        }
        Err(error) => {
            eprintln!("[../i3bar/src/config.c:422] ERROR: Could not parse config reply: {error}");
            libc::exit(libc::EXIT_FAILURE);
        }
    }
}
struct FirstConfig(Rc<RefCell<Option<CString>>>);
impl EventHandler for FirstConfig {
    fn on_event(&mut self, event: Event<'_>) -> bool {
        if let Event::String(value) = event {
            *self.0.borrow_mut() = Some(owned_string(value));
            return false;
        }
        true
    }
}
fn read_first_config(json: &[u8]) -> Option<CString> {
    let first = Rc::new(RefCell::new(None));
    let mut parser = Parser::new(FirstConfig(Rc::clone(&first)));
    parser.options_mut().allow_legacy_whitespace = true;
    // Client cancellation on the first string deliberately ignores the rest,
    // including malformed trailing data, as the original callback did.
    let _ = parser.parse(json);
    drop(parser);
    Rc::try_unwrap(first)
        .ok()
        .expect("parser released first config")
        .into_inner()
}
/// # Safety
/// json points to size readable bytes; the event thread owns config.bar_id.
pub unsafe fn parse_get_first_i3bar_config(json: *const u8, size: usize) {
    let json = if size == 0 {
        &[]
    } else {
        std::slice::from_raw_parts(json, size)
    };
    if let Some(first) = read_first_config(json) {
        store_native(&raw mut config.bar_id, &first);
    }
}

/// # Safety
/// Every non-null color slot is a native malloc allocation exclusively owned
/// by this transport; no borrowed color pointer survives the call.
pub unsafe fn free_colors(colors: *mut xcb_color_strings_t) {
    free((*colors).bar_fg as *mut ::core::ffi::c_void);
    (*colors).bar_fg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free((*colors).bar_bg as *mut ::core::ffi::c_void);
    (*colors).bar_bg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free((*colors).sep_fg as *mut ::core::ffi::c_void);
    (*colors).sep_fg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free((*colors).focus_bar_fg as *mut ::core::ffi::c_void);
    (*colors).focus_bar_fg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free((*colors).focus_bar_bg as *mut ::core::ffi::c_void);
    (*colors).focus_bar_bg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free((*colors).focus_sep_fg as *mut ::core::ffi::c_void);
    (*colors).focus_sep_fg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free((*colors).active_ws_fg as *mut ::core::ffi::c_void);
    (*colors).active_ws_fg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free((*colors).active_ws_bg as *mut ::core::ffi::c_void);
    (*colors).active_ws_bg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free((*colors).active_ws_border as *mut ::core::ffi::c_void);
    (*colors).active_ws_border = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free((*colors).inactive_ws_fg as *mut ::core::ffi::c_void);
    (*colors).inactive_ws_fg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free((*colors).inactive_ws_bg as *mut ::core::ffi::c_void);
    (*colors).inactive_ws_bg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free((*colors).inactive_ws_border as *mut ::core::ffi::c_void);
    (*colors).inactive_ws_border = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free((*colors).urgent_ws_fg as *mut ::core::ffi::c_void);
    (*colors).urgent_ws_fg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free((*colors).urgent_ws_bg as *mut ::core::ffi::c_void);
    (*colors).urgent_ws_bg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free((*colors).urgent_ws_border as *mut ::core::ffi::c_void);
    (*colors).urgent_ws_border = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free((*colors).focus_ws_fg as *mut ::core::ffi::c_void);
    (*colors).focus_ws_fg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free((*colors).focus_ws_bg as *mut ::core::ffi::c_void);
    (*colors).focus_ws_bg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free((*colors).focus_ws_border as *mut ::core::ffi::c_void);
    (*colors).focus_ws_border = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free((*colors).binding_mode_fg as *mut ::core::ffi::c_void);
    (*colors).binding_mode_fg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free((*colors).binding_mode_bg as *mut ::core::ffi::c_void);
    (*colors).binding_mode_bg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free((*colors).binding_mode_border as *mut ::core::ffi::c_void);
    (*colors).binding_mode_border = ::core::ptr::null_mut::<::core::ffi::c_char>();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CStr;

    fn parse(json: &[u8]) -> ConfigPatch {
        read_config(json, ReaderState::default()).unwrap()
    }
    // Tests own a local POD transport; they never access the real event thread.
    unsafe fn release(target: *mut config_t) {
        clear_config_lists(target);
        free_colors(&raw mut (*target).colors);
        free((*target).command.cast());
        free((*target).workspace_command.cast());
        free((*target).fontname.cast());
        (*target).separator_symbol = None;
        for index in 0..(*target).num_outputs {
            free((*(*target).outputs.add(index as usize)).cast());
        }
        free((*target).outputs.cast());
    }

    #[test]
    fn partial_configs_keep_omitted_scalars_and_append_outputs_but_replace_lists() {
        let mut target = empty_config();
        let first = parse(br#"{"bar_height":23,"font":"old-font","outputs":["one"],"bindings":[{"input_code":4,"command":"old"}],"tray_outputs":["one"]}"#);
        let second = parse(br#"{"outputs":["two"],"strip_workspace_name":true,"padding":{"x":6 "#);
        unsafe {
            apply_config(&raw mut target, first.updates);
            apply_config(&raw mut target, second.updates);
            assert_eq!(target.bar_height, 23);
            assert_eq!(target.padding.x, 6);
            assert!(target.strip_ws_name);
            assert_eq!(CStr::from_ptr(target.fontname), c"old-font");
            assert_eq!(target.num_outputs, 2);
            assert_eq!(CStr::from_ptr(*target.outputs), c"one");
            assert_eq!(CStr::from_ptr(*target.outputs.add(1)), c"two");
            assert!(target.bindings.tqh_first.is_null());
            assert!(target.tray_outputs.tqh_first.is_null());
            clear_config_lists(&raw mut target);
            assert_eq!(target.bindings.tqh_last, &raw mut target.bindings.tqh_first);
            release(&raw mut target);
        }
    }

    #[test]
    fn binding_nodes_follow_input_code_order_across_maps_and_own_strings() {
        let input = br#"{"wheel_up_cmd":"wheel","bindings":[{"input_code":8},{"command":"next\u0000ignored","release":true},{"input_code":4294967305,"command":"last"}],"tray_outputs":["DP-1\u0000ignored","primary"]}"#.to_vec();
        let patch = parse(&input);
        drop(input);
        let mut target = empty_config();
        unsafe {
            apply_config(&raw mut target, patch.updates);
            let wheel = target.bindings.tqh_first;
            let next = (*wheel).bindings.tqe_next;
            let last = (*next).bindings.tqe_next;
            assert_eq!((*wheel).input_code, 4);
            assert_eq!((*wheel).command.as_deref(), Some(c"wheel"));
            assert_eq!((*next).input_code, 8);
            assert_eq!((*next).command.as_deref(), Some(c"next"));
            assert!((*next).release);
            assert_eq!((*last).input_code, 9);
            assert_eq!(
                (*last).bindings.tqe_prev,
                &raw mut (*next).bindings.tqe_next
            );
            assert_eq!(target.bindings.tqh_last, &raw mut (*last).bindings.tqe_next);
            let tray = target.tray_outputs.tqh_first;
            assert_eq!((*tray).output.as_c_str(), c"DP-1");
            assert_eq!(
                (*(*tray).tray_outputs.tqe_next).output.as_c_str(),
                c"primary"
            );
            release(&raw mut target);
        }
    }

    #[test]
    fn sticky_flags_keys_and_any_end_array_survive_stream_boundaries() {
        let first = parse(br#"{"bindings":[{"input_code":7},"#);
        assert!(first.state.bindings);
        let second = read_config(br#"{"input_code":9,"command":"new"}"#, first.state).unwrap();
        assert!(second.state.bindings);
        let reset = read_config(br#"{"irrelevant":[],"bar_height":12}"#, second.state).unwrap();
        assert!(!reset.state.bindings);
        // A float/null neither dispatches an integer callback nor clears key;
        // array members reuse the original key until an explicit MapKey.
        let sticky = parse(br#"{"bar_height":[2.5,null,31],"padding":{"x":2},"width":7}"#);
        let mut target = empty_config();
        unsafe {
            apply_config(&raw mut target, sticky.updates);
            assert_eq!(target.bar_height, 31);
            assert_eq!(target.padding.x, 2);
            assert_eq!(target.padding.width, 7);
            release(&raw mut target);
        }
    }

    #[test]
    fn strings_use_full_enum_lengths_but_owned_storage_uses_nul_prefixes() {
        let patch = parse(br#"{"mode":"dock\u0000x","position":"top\u0000x","modifier":"Mod1\u0000x","status_command\u0000ignored":"run\u0000ignored","separator_symbol":"a\u0000b","verbose":true,"verbose":false}"#);
        let mut target = empty_config();
        unsafe {
            apply_config(&raw mut target, patch.updates);
            assert_eq!(target.hide_on_modifier, BarDisplayMode::Invisible);
            assert_eq!(target.position, BarPosition::Bottom);
            assert_eq!(target.modifier, 64);
            assert_eq!(CStr::from_ptr(target.command), c"run");
            assert_eq!(
                target.separator_symbol.as_ref().unwrap().as_str().unwrap(),
                "a�b"
            );
            assert_eq!(target.separator_symbol.as_ref().unwrap().byte_len(), Ok(5));
            assert!(target.verbose);
            release(&raw mut target);
        }
    }

    #[test]
    fn rejected_updates_never_touch_transport_and_callback_bounds_are_preserved() {
        for json in [
            br#"{"bindings":[{"command":"early"}]}"#.as_slice(),
            br#"{"bindings":[{"input_code":1,"command":"one","command":"two"}]}"#,
            br#"{"bindings":[{"release":true}]}"#,
            br#"{"bindings":[{"other":1}]}"#,
            br#"{"unknown":"text"}"#,
            br#"{"unknown":true}"#,
            br#"{"unknown":3}"#,
            br#"{"id":null}"#,
            br#"{"bar_height":9223372036854775808}"#,
            br#"{"bar_height":-9223372036854775808}"#,
        ] {
            assert!(
                read_config(json, ReaderState::default()).is_err(),
                "{json:?}"
            );
        }
        let patch = parse(
            br#"{"bar_height":4294967297,"tray_padding":4294967295,"ignored":2.5,"ignored":null}"#,
        );
        let mut target = empty_config();
        unsafe {
            apply_config(&raw mut target, patch.updates);
            assert_eq!(target.bar_height, 1);
            assert_eq!(target.tray_padding, -1);
            release(&raw mut target);
        }
    }

    #[test]
    fn command_snapshots_remain_live_through_reload_comparisons() {
        let mut target = empty_config();
        unsafe {
            apply_config(
                &raw mut target,
                parse(br#"{"status_command":"same","workspace_command":"different"}"#).updates,
            );
            // IPC deliberately detaches these snapshots before parsing a reload.
            let old_command = target.command;
            let old_workspace_command = target.workspace_command;
            target.command = ptr::null_mut();
            target.workspace_command = ptr::null_mut();
            apply_config(
                &raw mut target,
                parse(br#"{"status_command":"same","workspace_command":"new"}"#).updates,
            );
            assert_eq!(CStr::from_ptr(old_command), CStr::from_ptr(target.command));
            assert_ne!(
                CStr::from_ptr(old_workspace_command),
                CStr::from_ptr(target.workspace_command)
            );
            free(old_command.cast());
            free(old_workspace_command.cast());
            release(&raw mut target);
        }
    }

    #[test]
    fn first_config_stops_on_first_string_even_before_invalid_trailing_bytes() {
        assert_eq!(
            read_first_config(br#"[null,3,"bar\u0000ignored",garbage"#).as_deref(),
            Some(c"bar")
        );
        assert_eq!(
            read_first_config(br#"{"ignored":"first","second":"later"}"#).as_deref(),
            Some(c"first")
        );
        assert!(read_first_config(br#"[null,3]"#).is_none());
        assert!(read_first_config(br#"["unfinished"#).is_none());
    }
}
