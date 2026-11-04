use i3_common::draw_util::draw_util_clear_surface;
use i3_common::draw_util::draw_util_hex_to_color;
use i3_common::draw_util::draw_util_text;
use i3_common::draw_util::{OwnedWindow, RenderingContext};
use i3_common::font::load_font;
use i3_common::font::predict_text_width;
use i3_common::mkdirp::{create_dir_all, report_create_error};
use std::ffi::{CStr, CString};
use std::os::unix::ffi::{OsStrExt, OsStringExt};
use std::rc::Rc;
use xcb::{x, Xid, XidNew};
use xcb_interop::events::key_press_state;
use xcb_interop::keysyms::KeySymbols;
use xcb_interop::transport::keyboard_grab_status;
/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * i3-config-wizard: Program to convert configs using keycodes to configs using
 *                   keysyms.
 *
 */
// Mechanical file-by-file Rust translation of i3-config-wizard/main.c.
// Originally translated with C2Rust 0.22.1; subsequently maintained in Rust.
use i3_common::dpi::{init_dpi, logical_px};

pub use i3_common::font::Font;

pub use i3_common::draw_util::color_t;
pub use i3_common::draw_util::surface_t;

#[derive(Copy, Clone)]
struct GrammarToken {
    name: &'static [u8],
    identifier: Option<&'static [u8]>,
    next_state: ParserState,
    call_identifier: u16,
}

pub const DEFAULT_DIR_MODE: libc::mode_t =
    libc::S_IRWXU | libc::S_IRGRP | libc::S_IXGRP | libc::S_IROTH | libc::S_IXOTH;

pub const XK_Return: i32 = 0xff0d_i32;
pub const XK_Escape: i32 = 0xff1b_i32;
pub const XK_Up: i32 = 0xff52_i32;
pub const XK_Down: i32 = 0xff54_i32;
pub const XK_KP_Enter: i32 = 0xff8d_i32;

pub static mut visual_type: *mut cairo_sys::xcb_visualtype_t =
    std::ptr::null_mut::<cairo_sys::xcb_visualtype_t>();

static mut xcb_numlock_mask: u32 = 0;

pub static mut conn: *mut xcb_connection_t = std::ptr::null_mut::<xcb_connection_t>();

pub static mut root_screen: Option<i3_common::display::DisplayScreen> = None;
static mut font: Font = Font::empty();
static mut bold_font: Font = Font::empty();
static mut char_width: i32 = 0;
static mut win: u32 = 0;
static mut surface: surface_t = surface_t::empty();

pub static mut root: u32 = 0;
static mut xkb_keymap: *mut xkb_keymap = std::ptr::null_mut::<xkb_keymap>();
static mut xkb_base_event: u8 = 0;
static mut xkb_base_error: u8 = 0;
static TOKENS_FORCE_DISPLAY_URGENCY_HINT_MS: [GrammarToken; 2] = [
    GrammarToken {
        name: b"'ms",
        identifier: None,
        next_state: ParserState::ForceDisplayUrgencyHintMs,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"end",
        identifier: None,
        next_state: ParserState::Call,
        call_identifier: 0,
    },
];
static TOKENS_FLOATING_MAXIMUM_SIZE_HEIGHT: [GrammarToken; 1] = [GrammarToken {
    name: b"number",
    identifier: Some(b"height"),
    next_state: ParserState::Call,
    call_identifier: 1,
}];
static TOKENS_FLOATING_MINIMUM_SIZE_HEIGHT: [GrammarToken; 1] = [GrammarToken {
    name: b"number",
    identifier: Some(b"height"),
    next_state: ParserState::Call,
    call_identifier: 2,
}];
static TOKENS_BAR_STRIP_WORKSPACE_NUMBERS: [GrammarToken; 1] = [GrammarToken {
    name: b"word",
    identifier: Some(b"value"),
    next_state: ParserState::Call,
    call_identifier: 3,
}];
static TOKENS_FLOATING_MAXIMUM_SIZE_WIDTH: [GrammarToken; 1] = [GrammarToken {
    name: b"number",
    identifier: Some(b"width"),
    next_state: ParserState::FloatingMaximumSizeX,
    call_identifier: 0,
}];
static TOKENS_FLOATING_MINIMUM_SIZE_WIDTH: [GrammarToken; 1] = [GrammarToken {
    name: b"number",
    identifier: Some(b"width"),
    next_state: ParserState::FloatingMinimumSizeX,
    call_identifier: 0,
}];
static TOKENS_BAR_BINDING_MODE_INDICATOR: [GrammarToken; 1] = [GrammarToken {
    name: b"word",
    identifier: Some(b"value"),
    next_state: ParserState::Call,
    call_identifier: 4,
}];
static TOKENS_BAR_WORKSPACE_MIN_WIDTH_PX: [GrammarToken; 2] = [
    GrammarToken {
        name: b"'px",
        identifier: None,
        next_state: ParserState::BarWorkspaceMinWidthPx,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"end",
        identifier: None,
        next_state: ParserState::Call,
        call_identifier: 5,
    },
];
static TOKENS_FOCUS_ON_WINDOW_ACTIVATION: [GrammarToken; 1] = [GrammarToken {
    name: b"word",
    identifier: Some(b"mode"),
    next_state: ParserState::Call,
    call_identifier: 6,
}];
static TOKENS_FORCE_DISPLAY_URGENCY_HINT: [GrammarToken; 1] = [GrammarToken {
    name: b"number",
    identifier: Some(b"duration_ms"),
    next_state: ParserState::ForceDisplayUrgencyHintMs,
    call_identifier: 0,
}];
static TOKENS_TILING_DRAG_SWAP_MODIFIER: [GrammarToken; 10] = [
    GrammarToken {
        name: b"'Mod1",
        identifier: Some(b"modifiers"),
        next_state: ParserState::TilingDragSwapModifier,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'Mod2",
        identifier: Some(b"modifiers"),
        next_state: ParserState::TilingDragSwapModifier,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'Mod3",
        identifier: Some(b"modifiers"),
        next_state: ParserState::TilingDragSwapModifier,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'Mod4",
        identifier: Some(b"modifiers"),
        next_state: ParserState::TilingDragSwapModifier,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'Mod5",
        identifier: Some(b"modifiers"),
        next_state: ParserState::TilingDragSwapModifier,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'Shift",
        identifier: Some(b"modifiers"),
        next_state: ParserState::TilingDragSwapModifier,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'Control",
        identifier: Some(b"modifiers"),
        next_state: ParserState::TilingDragSwapModifier,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'Ctrl",
        identifier: Some(b"modifiers"),
        next_state: ParserState::TilingDragSwapModifier,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'+",
        identifier: None,
        next_state: ParserState::TilingDragSwapModifier,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"end",
        identifier: None,
        next_state: ParserState::Call,
        call_identifier: 7,
    },
];
static TOKENS_BAR_STRIP_WORKSPACE_NAME: [GrammarToken; 1] = [GrammarToken {
    name: b"word",
    identifier: Some(b"value"),
    next_state: ParserState::Call,
    call_identifier: 8,
}];
static TOKENS_CRITERION_FROM_STR_START: [GrammarToken; 3] = [
    GrammarToken {
        name: b"'\"",
        identifier: None,
        next_state: ParserState::CriterionFromStr,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'auto",
        identifier: Some(b"kind"),
        next_state: ParserState::Call,
        call_identifier: 9,
    },
    GrammarToken {
        name: b"'user",
        identifier: Some(b"kind"),
        next_state: ParserState::Call,
        call_identifier: 10,
    },
];
static TOKENS_DEFAULT_BORDER_PIXELS_PX: [GrammarToken; 2] = [
    GrammarToken {
        name: b"'px",
        identifier: None,
        next_state: ParserState::DefaultBorderPixelsPx,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"end",
        identifier: None,
        next_state: ParserState::Call,
        call_identifier: 11,
    },
];
static TOKENS_WORKSPACE_BACK_AND_FORTH: [GrammarToken; 1] = [GrammarToken {
    name: b"word",
    identifier: Some(b"value"),
    next_state: ParserState::Call,
    call_identifier: 12,
}];
static TOKENS_ASSIGN_WORKSPACE_NUMBER: [GrammarToken; 1] = [GrammarToken {
    name: b"string",
    identifier: Some(b"number"),
    next_state: ParserState::Call,
    call_identifier: 13,
}];
static TOKENS_BAR_WORKSPACE_MIN_WIDTH: [GrammarToken; 1] = [GrammarToken {
    name: b"number",
    identifier: Some(b"width"),
    next_state: ParserState::BarWorkspaceMinWidthPx,
    call_identifier: 0,
}];
static TOKENS_FLOATING_MAXIMUM_SIZE_X: [GrammarToken; 1] = [GrammarToken {
    name: b"'x",
    identifier: None,
    next_state: ParserState::FloatingMaximumSizeHeight,
    call_identifier: 0,
}];
static TOKENS_FLOATING_MINIMUM_SIZE_X: [GrammarToken; 1] = [GrammarToken {
    name: b"'x",
    identifier: None,
    next_state: ParserState::FloatingMinimumSizeHeight,
    call_identifier: 0,
}];
static TOKENS_POPUP_DURING_FULLSCREEN: [GrammarToken; 4] = [
    GrammarToken {
        name: b"'ignore",
        identifier: Some(b"value"),
        next_state: ParserState::Call,
        call_identifier: 14,
    },
    GrammarToken {
        name: b"'leave_fullscreen",
        identifier: Some(b"value"),
        next_state: ParserState::Call,
        call_identifier: 15,
    },
    GrammarToken {
        name: b"'all",
        identifier: Some(b"value"),
        next_state: ParserState::Call,
        call_identifier: 16,
    },
    GrammarToken {
        name: b"'smart",
        identifier: Some(b"value"),
        next_state: ParserState::Call,
        call_identifier: 17,
    },
];
static TOKENS_BAR_COLORS_IGNORE_LINE: [GrammarToken; 1] = [GrammarToken {
    name: b"line",
    identifier: None,
    next_state: ParserState::BarColors,
    call_identifier: 0,
}];
static TOKENS_CRITERION_FROM_STR_END: [GrammarToken; 1] = [GrammarToken {
    name: b"'\"",
    identifier: None,
    next_state: ParserState::Call,
    call_identifier: 18,
}];
static TOKENS_BAR_COLORS_BACKGROUND: [GrammarToken; 1] = [GrammarToken {
    name: b"word",
    identifier: Some(b"background"),
    next_state: ParserState::BarColorsText,
    call_identifier: 0,
}];
static TOKENS_BAR_WORKSPACE_BUTTONS: [GrammarToken; 1] = [GrammarToken {
    name: b"word",
    identifier: Some(b"value"),
    next_state: ParserState::Call,
    call_identifier: 19,
}];
static TOKENS_BAR_WORKSPACE_COMMAND: [GrammarToken; 1] = [GrammarToken {
    name: b"string",
    identifier: Some(b"command"),
    next_state: ParserState::Call,
    call_identifier: 20,
}];
static TOKENS_DEFAULT_BORDER_PIXELS: [GrammarToken; 2] = [
    GrammarToken {
        name: b"end",
        identifier: None,
        next_state: ParserState::Call,
        call_identifier: 21,
    },
    GrammarToken {
        name: b"number",
        identifier: Some(b"width"),
        next_state: ParserState::DefaultBorderPixelsPx,
        call_identifier: 0,
    },
];
static TOKENS_WORKSPACE_OUTPUT_WORD: [GrammarToken; 2] = [
    GrammarToken {
        name: b"word",
        identifier: Some(b"output"),
        next_state: ParserState::Call,
        call_identifier: 22,
    },
    GrammarToken {
        name: b"end",
        identifier: None,
        next_state: ParserState::Initial,
        call_identifier: 0,
    },
];
static TOKENS_BAR_SEPARATOR_SYMBOL: [GrammarToken; 1] = [GrammarToken {
    name: b"string",
    identifier: Some(b"separator"),
    next_state: ParserState::Call,
    call_identifier: 23,
}];
static TOKENS_FORCE_FOCUS_WRAPPING: [GrammarToken; 1] = [GrammarToken {
    name: b"word",
    identifier: Some(b"value"),
    next_state: ParserState::Call,
    call_identifier: 24,
}];
static TOKENS_BAR_BINDSYM_COMMAND: [GrammarToken; 2] = [
    GrammarToken {
        name: b"'--release",
        identifier: Some(b"release"),
        next_state: ParserState::BarBindsymCommand,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"string",
        identifier: Some(b"command"),
        next_state: ParserState::Call,
        call_identifier: 25,
    },
];
static TOKENS_BAR_TRAY_PADDING_PX: [GrammarToken; 2] = [
    GrammarToken {
        name: b"'px",
        identifier: None,
        next_state: ParserState::BarTrayPaddingPx,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"end",
        identifier: None,
        next_state: ParserState::Call,
        call_identifier: 26,
    },
];
static TOKENS_DEFAULT_ORIENTATION: [GrammarToken; 3] = [
    GrammarToken {
        name: b"'horizontal",
        identifier: Some(b"orientation"),
        next_state: ParserState::Call,
        call_identifier: 27,
    },
    GrammarToken {
        name: b"'vertical",
        identifier: Some(b"orientation"),
        next_state: ParserState::Call,
        call_identifier: 28,
    },
    GrammarToken {
        name: b"'auto",
        identifier: Some(b"orientation"),
        next_state: ParserState::Call,
        call_identifier: 29,
    },
];
static TOKENS_FOCUS_FOLLOWS_MOUSE: [GrammarToken; 1] = [GrammarToken {
    name: b"word",
    identifier: Some(b"value"),
    next_state: ParserState::Call,
    call_identifier: 30,
}];
static TOKENS_BAR_PADDING_BOTTOM: [GrammarToken; 3] = [
    GrammarToken {
        name: b"'px",
        identifier: None,
        next_state: ParserState::BarPaddingBottom,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"number",
        identifier: Some(b"left"),
        next_state: ParserState::BarPaddingLeft,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"end",
        identifier: None,
        next_state: ParserState::Call,
        call_identifier: 31,
    },
];
static TOKENS_BAR_STATUS_COMMAND: [GrammarToken; 1] = [GrammarToken {
    name: b"string",
    identifier: Some(b"command"),
    next_state: ParserState::Call,
    call_identifier: 32,
}];
static TOKENS_BAR_WHEEL_DOWN_CMD: [GrammarToken; 1] = [GrammarToken {
    name: b"string",
    identifier: Some(b"command"),
    next_state: ParserState::Call,
    call_identifier: 33,
}];
static TOKENS_COLOR_CHILD_BORDER: [GrammarToken; 2] = [
    GrammarToken {
        name: b"word",
        identifier: Some(b"child_border"),
        next_state: ParserState::Call,
        call_identifier: 34,
    },
    GrammarToken {
        name: b"end",
        identifier: None,
        next_state: ParserState::Call,
        call_identifier: 35,
    },
];
static TOKENS_CRITERION_FROM_STR: [GrammarToken; 2] = [
    GrammarToken {
        name: b"'auto",
        identifier: Some(b"kind"),
        next_state: ParserState::CriterionFromStrEnd,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'user",
        identifier: Some(b"kind"),
        next_state: ParserState::CriterionFromStrEnd,
        call_identifier: 0,
    },
];
static TOKENS_FOR_WINDOW_COMMAND: [GrammarToken; 1] = [GrammarToken {
    name: b"string",
    identifier: Some(b"command"),
    next_state: ParserState::Call,
    call_identifier: 36,
}];
static TOKENS_BAR_COLORS_BORDER: [GrammarToken; 1] = [GrammarToken {
    name: b"word",
    identifier: Some(b"border"),
    next_state: ParserState::BarColorsBackground,
    call_identifier: 0,
}];
static TOKENS_BAR_COLORS_SINGLE: [GrammarToken; 1] = [GrammarToken {
    name: b"word",
    identifier: Some(b"color"),
    next_state: ParserState::Call,
    call_identifier: 37,
}];
static TOKENS_BAR_PADDING_RIGHT: [GrammarToken; 3] = [
    GrammarToken {
        name: b"'px",
        identifier: None,
        next_state: ParserState::BarPaddingRight,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"number",
        identifier: Some(b"bottom"),
        next_state: ParserState::BarPaddingBottom,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"end",
        identifier: None,
        next_state: ParserState::Call,
        call_identifier: 38,
    },
];
static TOKENS_FLOATING_MODIFIER: [GrammarToken; 10] = [
    GrammarToken {
        name: b"'Mod1",
        identifier: Some(b"modifiers"),
        next_state: ParserState::FloatingModifier,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'Mod2",
        identifier: Some(b"modifiers"),
        next_state: ParserState::FloatingModifier,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'Mod3",
        identifier: Some(b"modifiers"),
        next_state: ParserState::FloatingModifier,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'Mod4",
        identifier: Some(b"modifiers"),
        next_state: ParserState::FloatingModifier,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'Mod5",
        identifier: Some(b"modifiers"),
        next_state: ParserState::FloatingModifier,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'Shift",
        identifier: Some(b"modifiers"),
        next_state: ParserState::FloatingModifier,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'Control",
        identifier: Some(b"modifiers"),
        next_state: ParserState::FloatingModifier,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'Ctrl",
        identifier: Some(b"modifiers"),
        next_state: ParserState::FloatingModifier,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'+",
        identifier: None,
        next_state: ParserState::FloatingModifier,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"end",
        identifier: None,
        next_state: ParserState::Call,
        call_identifier: 39,
    },
];
static TOKENS_HIDE_EDGE_BORDERS: [GrammarToken; 12] = [
    GrammarToken {
        name: b"'none",
        identifier: Some(b"hide_borders"),
        next_state: ParserState::Call,
        call_identifier: 40,
    },
    GrammarToken {
        name: b"'vertical",
        identifier: Some(b"hide_borders"),
        next_state: ParserState::Call,
        call_identifier: 41,
    },
    GrammarToken {
        name: b"'horizontal",
        identifier: Some(b"hide_borders"),
        next_state: ParserState::Call,
        call_identifier: 42,
    },
    GrammarToken {
        name: b"'both",
        identifier: Some(b"hide_borders"),
        next_state: ParserState::Call,
        call_identifier: 43,
    },
    GrammarToken {
        name: b"'smart_no_gaps",
        identifier: Some(b"hide_borders"),
        next_state: ParserState::Call,
        call_identifier: 44,
    },
    GrammarToken {
        name: b"'smart",
        identifier: Some(b"hide_borders"),
        next_state: ParserState::Call,
        call_identifier: 45,
    },
    GrammarToken {
        name: b"'1",
        identifier: Some(b"hide_borders"),
        next_state: ParserState::Call,
        call_identifier: 46,
    },
    GrammarToken {
        name: b"'yes",
        identifier: Some(b"hide_borders"),
        next_state: ParserState::Call,
        call_identifier: 47,
    },
    GrammarToken {
        name: b"'true",
        identifier: Some(b"hide_borders"),
        next_state: ParserState::Call,
        call_identifier: 48,
    },
    GrammarToken {
        name: b"'on",
        identifier: Some(b"hide_borders"),
        next_state: ParserState::Call,
        call_identifier: 49,
    },
    GrammarToken {
        name: b"'enable",
        identifier: Some(b"hide_borders"),
        next_state: ParserState::Call,
        call_identifier: 50,
    },
    GrammarToken {
        name: b"'active",
        identifier: Some(b"hide_borders"),
        next_state: ParserState::Call,
        call_identifier: 51,
    },
];
static TOKENS_WORKSPACE_COMMAND: [GrammarToken; 2] = [
    GrammarToken {
        name: b"'output",
        identifier: None,
        next_state: ParserState::WorkspaceOutputWord,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'gaps",
        identifier: None,
        next_state: ParserState::Gaps,
        call_identifier: 0,
    },
];
static TOKENS_ASSIGN_WORKSPACE: [GrammarToken; 5] = [
    GrammarToken {
        name: b"'\xe2\x86\x92",
        identifier: None,
        next_state: ParserState::AssignWorkspace,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'output",
        identifier: None,
        next_state: ParserState::AssignOutput,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'workspace",
        identifier: None,
        next_state: ParserState::AssignWorkspace,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'number",
        identifier: None,
        next_state: ParserState::AssignWorkspaceNumber,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"string",
        identifier: Some(b"workspace"),
        next_state: ParserState::Call,
        call_identifier: 52,
    },
];
static TOKENS_BAR_COLORS_BRACE: [GrammarToken; 2] = [
    GrammarToken {
        name: b"end",
        identifier: None,
        next_state: ParserState::BarColorsBrace,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'{",
        identifier: None,
        next_state: ParserState::BarColors,
        call_identifier: 0,
    },
];
static TOKENS_BAR_HIDDEN_STATE: [GrammarToken; 2] = [
    GrammarToken {
        name: b"'hide",
        identifier: Some(b"hidden_state"),
        next_state: ParserState::Call,
        call_identifier: 53,
    },
    GrammarToken {
        name: b"'show",
        identifier: Some(b"hidden_state"),
        next_state: ParserState::Call,
        call_identifier: 54,
    },
];
static TOKENS_BAR_PADDING_LEFT: [GrammarToken; 2] = [
    GrammarToken {
        name: b"'px",
        identifier: None,
        next_state: ParserState::BarPaddingLeft,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"end",
        identifier: None,
        next_state: ParserState::Call,
        call_identifier: 55,
    },
];
static TOKENS_BAR_TRAY_PADDING: [GrammarToken; 1] = [GrammarToken {
    name: b"number",
    identifier: Some(b"padding_px"),
    next_state: ParserState::BarTrayPaddingPx,
    call_identifier: 0,
}];
static TOKENS_BAR_WHEEL_UP_CMD: [GrammarToken; 1] = [GrammarToken {
    name: b"string",
    identifier: Some(b"command"),
    next_state: ParserState::Call,
    call_identifier: 56,
}];
static TOKENS_COLOR_BACKGROUND: [GrammarToken; 1] = [GrammarToken {
    name: b"word",
    identifier: Some(b"background"),
    next_state: ParserState::ColorText,
    call_identifier: 0,
}];
static TOKENS_IPC_KILL_TIMEOUT: [GrammarToken; 1] = [GrammarToken {
    name: b"number",
    identifier: Some(b"timeout"),
    next_state: ParserState::Call,
    call_identifier: 57,
}];
static TOKENS_MODE_BINDCOMMAND: [GrammarToken; 5] = [
    GrammarToken {
        name: b"'--release",
        identifier: Some(b"release"),
        next_state: ParserState::ModeBindcommand,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'--border",
        identifier: Some(b"border"),
        next_state: ParserState::ModeBindcommand,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'--whole-window",
        identifier: Some(b"whole_window"),
        next_state: ParserState::ModeBindcommand,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'--exclude-titlebar",
        identifier: Some(b"exclude_titlebar"),
        next_state: ParserState::ModeBindcommand,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"string",
        identifier: Some(b"command"),
        next_state: ParserState::Call,
        call_identifier: 58,
    },
];
static TOKENS_MODE_IGNORE_LINE: [GrammarToken; 1] = [GrammarToken {
    name: b"line",
    identifier: None,
    next_state: ParserState::Mode,
    call_identifier: 0,
}];
static TOKENS_TILING_DRAG_MODE: [GrammarToken; 3] = [
    GrammarToken {
        name: b"'modifier",
        identifier: Some(b"value"),
        next_state: ParserState::TilingDragMode,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'titlebar",
        identifier: Some(b"value"),
        next_state: ParserState::TilingDragMode,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"end",
        identifier: None,
        next_state: ParserState::Call,
        call_identifier: 59,
    },
];
static TOKENS_WORKSPACE_LAYOUT: [GrammarToken; 4] = [
    GrammarToken {
        name: b"'default",
        identifier: Some(b"layout"),
        next_state: ParserState::Call,
        call_identifier: 60,
    },
    GrammarToken {
        name: b"'stacking",
        identifier: Some(b"layout"),
        next_state: ParserState::Call,
        call_identifier: 61,
    },
    GrammarToken {
        name: b"'stacked",
        identifier: Some(b"layout"),
        next_state: ParserState::Call,
        call_identifier: 62,
    },
    GrammarToken {
        name: b"'tabbed",
        identifier: Some(b"layout"),
        next_state: ParserState::Call,
        call_identifier: 63,
    },
];
static TOKENS_BAR_BAR_COMMAND: [GrammarToken; 1] = [GrammarToken {
    name: b"string",
    identifier: Some(b"command"),
    next_state: ParserState::Call,
    call_identifier: 64,
}];
static TOKENS_BAR_COLORS_TEXT: [GrammarToken; 2] = [
    GrammarToken {
        name: b"end",
        identifier: None,
        next_state: ParserState::Call,
        call_identifier: 65,
    },
    GrammarToken {
        name: b"word",
        identifier: Some(b"text"),
        next_state: ParserState::Call,
        call_identifier: 66,
    },
];
static TOKENS_BAR_IGNORE_LINE: [GrammarToken; 1] = [GrammarToken {
    name: b"line",
    identifier: None,
    next_state: ParserState::Bar,
    call_identifier: 0,
}];
static TOKENS_BAR_PADDING_TOP: [GrammarToken; 3] = [
    GrammarToken {
        name: b"'px",
        identifier: None,
        next_state: ParserState::BarPaddingTop,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"number",
        identifier: Some(b"right_or_right_and_left"),
        next_state: ParserState::BarPaddingRight,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"end",
        identifier: None,
        next_state: ParserState::Call,
        call_identifier: 67,
    },
];
static TOKENS_BAR_SOCKET_PATH: [GrammarToken; 1] = [GrammarToken {
    name: b"string",
    identifier: Some(b"path"),
    next_state: ParserState::Call,
    call_identifier: 68,
}];
static TOKENS_BAR_TRAY_OUTPUT: [GrammarToken; 1] = [GrammarToken {
    name: b"word",
    identifier: Some(b"output"),
    next_state: ParserState::Call,
    call_identifier: 69,
}];
static TOKENS_COLOR_INDICATOR: [GrammarToken; 2] = [
    GrammarToken {
        name: b"word",
        identifier: Some(b"indicator"),
        next_state: ParserState::ColorChildBorder,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"end",
        identifier: None,
        next_state: ParserState::Call,
        call_identifier: 70,
    },
];
static TOKENS_DISABLE_RANDR15: [GrammarToken; 1] = [GrammarToken {
    name: b"word",
    identifier: Some(b"value"),
    next_state: ParserState::Call,
    call_identifier: 71,
}];
static TOKENS_GAPS_WITH_SCOPE: [GrammarToken; 1] = [GrammarToken {
    name: b"number",
    identifier: Some(b"value"),
    next_state: ParserState::GapsEnd,
    call_identifier: 0,
}];
static TOKENS_CRITERION_FROM: [GrammarToken; 1] = [GrammarToken {
    name: b"'=",
    identifier: None,
    next_state: ParserState::CriterionFromStrStart,
    call_identifier: 0,
}];
static TOKENS_DEFAULT_BORDER: [GrammarToken; 4] = [
    GrammarToken {
        name: b"'normal",
        identifier: Some(b"border"),
        next_state: ParserState::DefaultBorderPixels,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'pixel",
        identifier: Some(b"border"),
        next_state: ParserState::DefaultBorderPixels,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'1pixel",
        identifier: Some(b"border"),
        next_state: ParserState::Call,
        call_identifier: 72,
    },
    GrammarToken {
        name: b"'none",
        identifier: Some(b"border"),
        next_state: ParserState::Call,
        call_identifier: 73,
    },
];
static TOKENS_FOCUS_WRAPPING: [GrammarToken; 14] = [
    GrammarToken {
        name: b"'1",
        identifier: Some(b"value"),
        next_state: ParserState::Call,
        call_identifier: 74,
    },
    GrammarToken {
        name: b"'yes",
        identifier: Some(b"value"),
        next_state: ParserState::Call,
        call_identifier: 75,
    },
    GrammarToken {
        name: b"'true",
        identifier: Some(b"value"),
        next_state: ParserState::Call,
        call_identifier: 76,
    },
    GrammarToken {
        name: b"'on",
        identifier: Some(b"value"),
        next_state: ParserState::Call,
        call_identifier: 77,
    },
    GrammarToken {
        name: b"'enable",
        identifier: Some(b"value"),
        next_state: ParserState::Call,
        call_identifier: 78,
    },
    GrammarToken {
        name: b"'active",
        identifier: Some(b"value"),
        next_state: ParserState::Call,
        call_identifier: 79,
    },
    GrammarToken {
        name: b"'0",
        identifier: Some(b"value"),
        next_state: ParserState::Call,
        call_identifier: 80,
    },
    GrammarToken {
        name: b"'no",
        identifier: Some(b"value"),
        next_state: ParserState::Call,
        call_identifier: 81,
    },
    GrammarToken {
        name: b"'false",
        identifier: Some(b"value"),
        next_state: ParserState::Call,
        call_identifier: 82,
    },
    GrammarToken {
        name: b"'off",
        identifier: Some(b"value"),
        next_state: ParserState::Call,
        call_identifier: 83,
    },
    GrammarToken {
        name: b"'disable",
        identifier: Some(b"value"),
        next_state: ParserState::Call,
        call_identifier: 84,
    },
    GrammarToken {
        name: b"'inactive",
        identifier: Some(b"value"),
        next_state: ParserState::Call,
        call_identifier: 85,
    },
    GrammarToken {
        name: b"'force",
        identifier: Some(b"value"),
        next_state: ParserState::Call,
        call_identifier: 86,
    },
    GrammarToken {
        name: b"'workspace",
        identifier: Some(b"value"),
        next_state: ParserState::Call,
        call_identifier: 87,
    },
];
static TOKENS_FORCE_XINERAMA: [GrammarToken; 1] = [GrammarToken {
    name: b"word",
    identifier: Some(b"value"),
    next_state: ParserState::Call,
    call_identifier: 88,
}];
static TOKENS_ASSIGN_OUTPUT: [GrammarToken; 1] = [GrammarToken {
    name: b"string",
    identifier: Some(b"output"),
    next_state: ParserState::Call,
    call_identifier: 89,
}];
static TOKENS_CRITERION_STR: [GrammarToken; 1] = [GrammarToken {
    name: b"word",
    identifier: Some(b"cvalue"),
    next_state: ParserState::Call,
    call_identifier: 90,
}];
static TOKENS_MOUSE_WARPING: [GrammarToken; 2] = [
    GrammarToken {
        name: b"'none",
        identifier: Some(b"value"),
        next_state: ParserState::Call,
        call_identifier: 91,
    },
    GrammarToken {
        name: b"'output",
        identifier: Some(b"value"),
        next_state: ParserState::Call,
        call_identifier: 92,
    },
];
static TOKENS_RESTART_STATE: [GrammarToken; 1] = [GrammarToken {
    name: b"string",
    identifier: Some(b"path"),
    next_state: ParserState::Call,
    call_identifier: 93,
}];
static TOKENS_SMART_BORDERS: [GrammarToken; 7] = [
    GrammarToken {
        name: b"'1",
        identifier: Some(b"enabled"),
        next_state: ParserState::Call,
        call_identifier: 94,
    },
    GrammarToken {
        name: b"'yes",
        identifier: Some(b"enabled"),
        next_state: ParserState::Call,
        call_identifier: 95,
    },
    GrammarToken {
        name: b"'true",
        identifier: Some(b"enabled"),
        next_state: ParserState::Call,
        call_identifier: 96,
    },
    GrammarToken {
        name: b"'on",
        identifier: Some(b"enabled"),
        next_state: ParserState::Call,
        call_identifier: 97,
    },
    GrammarToken {
        name: b"'enable",
        identifier: Some(b"enabled"),
        next_state: ParserState::Call,
        call_identifier: 98,
    },
    GrammarToken {
        name: b"'active",
        identifier: Some(b"enabled"),
        next_state: ParserState::Call,
        call_identifier: 99,
    },
    GrammarToken {
        name: b"'no_gaps",
        identifier: Some(b"enabled"),
        next_state: ParserState::Call,
        call_identifier: 100,
    },
];
static TOKENS_BAR_MODIFIER: [GrammarToken; 12] = [
    GrammarToken {
        name: b"'off",
        identifier: None,
        next_state: ParserState::Call,
        call_identifier: 101,
    },
    GrammarToken {
        name: b"'none",
        identifier: None,
        next_state: ParserState::Call,
        call_identifier: 102,
    },
    GrammarToken {
        name: b"'Mod1",
        identifier: Some(b"modifiers"),
        next_state: ParserState::BarModifier,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'Mod2",
        identifier: Some(b"modifiers"),
        next_state: ParserState::BarModifier,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'Mod3",
        identifier: Some(b"modifiers"),
        next_state: ParserState::BarModifier,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'Mod4",
        identifier: Some(b"modifiers"),
        next_state: ParserState::BarModifier,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'Mod5",
        identifier: Some(b"modifiers"),
        next_state: ParserState::BarModifier,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'Shift",
        identifier: Some(b"modifiers"),
        next_state: ParserState::BarModifier,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'Control",
        identifier: Some(b"modifiers"),
        next_state: ParserState::BarModifier,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'Ctrl",
        identifier: Some(b"modifiers"),
        next_state: ParserState::BarModifier,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'+",
        identifier: None,
        next_state: ParserState::BarModifier,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"end",
        identifier: None,
        next_state: ParserState::Call,
        call_identifier: 103,
    },
];
static TOKENS_BAR_POSITION: [GrammarToken; 2] = [
    GrammarToken {
        name: b"'top",
        identifier: Some(b"position"),
        next_state: ParserState::Call,
        call_identifier: 104,
    },
    GrammarToken {
        name: b"'bottom",
        identifier: Some(b"position"),
        next_state: ParserState::Call,
        call_identifier: 105,
    },
];
static TOKENS_COLOR_BORDER: [GrammarToken; 1] = [GrammarToken {
    name: b"word",
    identifier: Some(b"border"),
    next_state: ParserState::ColorBackground,
    call_identifier: 0,
}];
static TOKENS_COLOR_SINGLE: [GrammarToken; 1] = [GrammarToken {
    name: b"word",
    identifier: Some(b"color"),
    next_state: ParserState::Call,
    call_identifier: 106,
}];
static TOKENS_FAKE_OUTPUTS: [GrammarToken; 1] = [GrammarToken {
    name: b"string",
    identifier: Some(b"outputs"),
    next_state: ParserState::Call,
    call_identifier: 107,
}];
static TOKENS_MODE_BINDING: [GrammarToken; 20] = [
    GrammarToken {
        name: b"'--release",
        identifier: Some(b"release"),
        next_state: ParserState::ModeBinding,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'--border",
        identifier: Some(b"border"),
        next_state: ParserState::ModeBinding,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'--whole-window",
        identifier: Some(b"whole_window"),
        next_state: ParserState::ModeBinding,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'--exclude-titlebar",
        identifier: Some(b"exclude_titlebar"),
        next_state: ParserState::ModeBinding,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'Mod1",
        identifier: Some(b"modifiers"),
        next_state: ParserState::ModeBinding,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'Mod2",
        identifier: Some(b"modifiers"),
        next_state: ParserState::ModeBinding,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'Mod3",
        identifier: Some(b"modifiers"),
        next_state: ParserState::ModeBinding,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'Mod4",
        identifier: Some(b"modifiers"),
        next_state: ParserState::ModeBinding,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'Mod5",
        identifier: Some(b"modifiers"),
        next_state: ParserState::ModeBinding,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'Shift",
        identifier: Some(b"modifiers"),
        next_state: ParserState::ModeBinding,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'Control",
        identifier: Some(b"modifiers"),
        next_state: ParserState::ModeBinding,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'Ctrl",
        identifier: Some(b"modifiers"),
        next_state: ParserState::ModeBinding,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'Mode_switch",
        identifier: Some(b"modifiers"),
        next_state: ParserState::ModeBinding,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'Group1",
        identifier: Some(b"modifiers"),
        next_state: ParserState::ModeBinding,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'Group2",
        identifier: Some(b"modifiers"),
        next_state: ParserState::ModeBinding,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'Group3",
        identifier: Some(b"modifiers"),
        next_state: ParserState::ModeBinding,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'Group4",
        identifier: Some(b"modifiers"),
        next_state: ParserState::ModeBinding,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'$mod",
        identifier: Some(b"modifiers"),
        next_state: ParserState::ModeBinding,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'+",
        identifier: None,
        next_state: ParserState::ModeBinding,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"word",
        identifier: Some(b"key"),
        next_state: ParserState::ModeBindcommand,
        call_identifier: 0,
    },
];
static TOKENS_NO_FOCUS_END: [GrammarToken; 1] = [GrammarToken {
    name: b"end",
    identifier: None,
    next_state: ParserState::Call,
    call_identifier: 108,
}];
static TOKENS_BAR_BINDSYM: [GrammarToken; 2] = [
    GrammarToken {
        name: b"'--release",
        identifier: Some(b"release"),
        next_state: ParserState::BarBindsym,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"word",
        identifier: Some(b"button"),
        next_state: ParserState::BarBindsymCommand,
        call_identifier: 0,
    },
];
static TOKENS_BAR_PADDING: [GrammarToken; 1] = [GrammarToken {
    name: b"number",
    identifier: Some(b"top_or_all"),
    next_state: ParserState::BarPaddingTop,
    call_identifier: 0,
}];
static TOKENS_BAR_VERBOSE: [GrammarToken; 1] = [GrammarToken {
    name: b"word",
    identifier: Some(b"value"),
    next_state: ParserState::Call,
    call_identifier: 109,
}];
static TOKENS_BINDCOMMAND: [GrammarToken; 5] = [
    GrammarToken {
        name: b"'--release",
        identifier: Some(b"release"),
        next_state: ParserState::Bindcommand,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'--border",
        identifier: Some(b"border"),
        next_state: ParserState::Bindcommand,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'--whole-window",
        identifier: Some(b"whole_window"),
        next_state: ParserState::Bindcommand,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'--exclude-titlebar",
        identifier: Some(b"exclude_titlebar"),
        next_state: ParserState::Bindcommand,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"string",
        identifier: Some(b"command"),
        next_state: ParserState::Call,
        call_identifier: 110,
    },
];
static TOKENS_IGNORE_LINE: [GrammarToken; 1] = [GrammarToken {
    name: b"line",
    identifier: None,
    next_state: ParserState::Initial,
    call_identifier: 0,
}];
static TOKENS_TILING_DRAG: [GrammarToken; 9] = [
    GrammarToken {
        name: b"'0",
        identifier: Some(b"off"),
        next_state: ParserState::Call,
        call_identifier: 111,
    },
    GrammarToken {
        name: b"'no",
        identifier: Some(b"off"),
        next_state: ParserState::Call,
        call_identifier: 112,
    },
    GrammarToken {
        name: b"'false",
        identifier: Some(b"off"),
        next_state: ParserState::Call,
        call_identifier: 113,
    },
    GrammarToken {
        name: b"'off",
        identifier: Some(b"off"),
        next_state: ParserState::Call,
        call_identifier: 114,
    },
    GrammarToken {
        name: b"'disable",
        identifier: Some(b"off"),
        next_state: ParserState::Call,
        call_identifier: 115,
    },
    GrammarToken {
        name: b"'inactive",
        identifier: Some(b"off"),
        next_state: ParserState::Call,
        call_identifier: 116,
    },
    GrammarToken {
        name: b"'swap_modifier",
        identifier: Some(b"swap_modifier"),
        next_state: ParserState::TilingDragSwapModifier,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'modifier",
        identifier: Some(b"value"),
        next_state: ParserState::TilingDragMode,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'titlebar",
        identifier: Some(b"value"),
        next_state: ParserState::TilingDragMode,
        call_identifier: 0,
    },
];
static TOKENS_TITLE_ALIGN: [GrammarToken; 3] = [
    GrammarToken {
        name: b"'left",
        identifier: Some(b"alignment"),
        next_state: ParserState::Call,
        call_identifier: 117,
    },
    GrammarToken {
        name: b"'center",
        identifier: Some(b"alignment"),
        next_state: ParserState::Call,
        call_identifier: 118,
    },
    GrammarToken {
        name: b"'right",
        identifier: Some(b"alignment"),
        next_state: ParserState::Call,
        call_identifier: 119,
    },
];
static TOKENS_BAR_COLORS: [GrammarToken; 15] = [
    GrammarToken {
        name: b"end",
        identifier: None,
        next_state: ParserState::BarColors,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'#",
        identifier: None,
        next_state: ParserState::BarColorsIgnoreLine,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'set",
        identifier: None,
        next_state: ParserState::BarColorsIgnoreLine,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'background",
        identifier: Some(b"colorclass"),
        next_state: ParserState::BarColorsSingle,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'statusline",
        identifier: Some(b"colorclass"),
        next_state: ParserState::BarColorsSingle,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'separator",
        identifier: Some(b"colorclass"),
        next_state: ParserState::BarColorsSingle,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'focused_background",
        identifier: Some(b"colorclass"),
        next_state: ParserState::BarColorsSingle,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'focused_statusline",
        identifier: Some(b"colorclass"),
        next_state: ParserState::BarColorsSingle,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'focused_separator",
        identifier: Some(b"colorclass"),
        next_state: ParserState::BarColorsSingle,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'focused_workspace",
        identifier: Some(b"colorclass"),
        next_state: ParserState::BarColorsBorder,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'active_workspace",
        identifier: Some(b"colorclass"),
        next_state: ParserState::BarColorsBorder,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'inactive_workspace",
        identifier: Some(b"colorclass"),
        next_state: ParserState::BarColorsBorder,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'urgent_workspace",
        identifier: Some(b"colorclass"),
        next_state: ParserState::BarColorsBorder,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'binding_mode",
        identifier: Some(b"colorclass"),
        next_state: ParserState::BarColorsBorder,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'}",
        identifier: None,
        next_state: ParserState::Bar,
        call_identifier: 0,
    },
];
static TOKENS_BAR_HEIGHT: [GrammarToken; 1] = [GrammarToken {
    name: b"number",
    identifier: Some(b"value"),
    next_state: ParserState::Call,
    call_identifier: 120,
}];
static TOKENS_BAR_OUTPUT: [GrammarToken; 1] = [GrammarToken {
    name: b"word",
    identifier: Some(b"output"),
    next_state: ParserState::Call,
    call_identifier: 121,
}];
static TOKENS_COLOR_TEXT: [GrammarToken; 1] = [GrammarToken {
    name: b"word",
    identifier: Some(b"text"),
    next_state: ParserState::ColorIndicator,
    call_identifier: 0,
}];
static TOKENS_FOR_WINDOW: [GrammarToken; 1] = [GrammarToken {
    name: b"'[",
    identifier: None,
    next_state: ParserState::Call,
    call_identifier: 122,
}];
static TOKENS_IPC_SOCKET: [GrammarToken; 1] = [GrammarToken {
    name: b"string",
    identifier: Some(b"path"),
    next_state: ParserState::Call,
    call_identifier: 123,
}];
static TOKENS_SHOW_MARKS: [GrammarToken; 1] = [GrammarToken {
    name: b"word",
    identifier: Some(b"value"),
    next_state: ParserState::Call,
    call_identifier: 124,
}];
static TOKENS_SMART_GAPS: [GrammarToken; 13] = [
    GrammarToken {
        name: b"'1",
        identifier: Some(b"enabled"),
        next_state: ParserState::Call,
        call_identifier: 125,
    },
    GrammarToken {
        name: b"'yes",
        identifier: Some(b"enabled"),
        next_state: ParserState::Call,
        call_identifier: 126,
    },
    GrammarToken {
        name: b"'true",
        identifier: Some(b"enabled"),
        next_state: ParserState::Call,
        call_identifier: 127,
    },
    GrammarToken {
        name: b"'on",
        identifier: Some(b"enabled"),
        next_state: ParserState::Call,
        call_identifier: 128,
    },
    GrammarToken {
        name: b"'enable",
        identifier: Some(b"enabled"),
        next_state: ParserState::Call,
        call_identifier: 129,
    },
    GrammarToken {
        name: b"'active",
        identifier: Some(b"enabled"),
        next_state: ParserState::Call,
        call_identifier: 130,
    },
    GrammarToken {
        name: b"'0",
        identifier: Some(b"enabled"),
        next_state: ParserState::Call,
        call_identifier: 131,
    },
    GrammarToken {
        name: b"'no",
        identifier: Some(b"enabled"),
        next_state: ParserState::Call,
        call_identifier: 132,
    },
    GrammarToken {
        name: b"'false",
        identifier: Some(b"enabled"),
        next_state: ParserState::Call,
        call_identifier: 133,
    },
    GrammarToken {
        name: b"'off",
        identifier: Some(b"enabled"),
        next_state: ParserState::Call,
        call_identifier: 134,
    },
    GrammarToken {
        name: b"'disable",
        identifier: Some(b"enabled"),
        next_state: ParserState::Call,
        call_identifier: 135,
    },
    GrammarToken {
        name: b"'inactive",
        identifier: Some(b"enabled"),
        next_state: ParserState::Call,
        call_identifier: 136,
    },
    GrammarToken {
        name: b"'inverse_outer",
        identifier: Some(b"enabled"),
        next_state: ParserState::Call,
        call_identifier: 137,
    },
];
static TOKENS_CRITERION: [GrammarToken; 1] = [GrammarToken {
    name: b"'=",
    identifier: None,
    next_state: ParserState::CriterionStr,
    call_identifier: 0,
}];
static TOKENS_MODEBRACE: [GrammarToken; 2] = [
    GrammarToken {
        name: b"end",
        identifier: None,
        next_state: ParserState::Modebrace,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'{",
        identifier: None,
        next_state: ParserState::Mode,
        call_identifier: 0,
    },
];
static TOKENS_WORKSPACE: [GrammarToken; 1] = [GrammarToken {
    name: b"word",
    identifier: Some(b"workspace"),
    next_state: ParserState::WorkspaceCommand,
    call_identifier: 0,
}];
static TOKENS_BARBRACE: [GrammarToken; 2] = [
    GrammarToken {
        name: b"end",
        identifier: None,
        next_state: ParserState::Barbrace,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'{",
        identifier: None,
        next_state: ParserState::Call,
        call_identifier: 138,
    },
];
static TOKENS_BAR_FONT: [GrammarToken; 1] = [GrammarToken {
    name: b"string",
    identifier: Some(b"font"),
    next_state: ParserState::Call,
    call_identifier: 139,
}];
static TOKENS_BAR_MODE: [GrammarToken; 3] = [
    GrammarToken {
        name: b"'dock",
        identifier: Some(b"mode"),
        next_state: ParserState::Call,
        call_identifier: 140,
    },
    GrammarToken {
        name: b"'hide",
        identifier: Some(b"mode"),
        next_state: ParserState::Call,
        call_identifier: 141,
    },
    GrammarToken {
        name: b"'invisible",
        identifier: Some(b"mode"),
        next_state: ParserState::Call,
        call_identifier: 142,
    },
];
static TOKENS_CRITERIA: [GrammarToken; 17] = [
    GrammarToken {
        name: b"'class",
        identifier: Some(b"ctype"),
        next_state: ParserState::Criterion,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'instance",
        identifier: Some(b"ctype"),
        next_state: ParserState::Criterion,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'window_role",
        identifier: Some(b"ctype"),
        next_state: ParserState::Criterion,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'con_id",
        identifier: Some(b"ctype"),
        next_state: ParserState::Criterion,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'id",
        identifier: Some(b"ctype"),
        next_state: ParserState::Criterion,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'window_type",
        identifier: Some(b"ctype"),
        next_state: ParserState::Criterion,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'con_mark",
        identifier: Some(b"ctype"),
        next_state: ParserState::Criterion,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'title",
        identifier: Some(b"ctype"),
        next_state: ParserState::Criterion,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'urgent",
        identifier: Some(b"ctype"),
        next_state: ParserState::Criterion,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'workspace",
        identifier: Some(b"ctype"),
        next_state: ParserState::Criterion,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'machine",
        identifier: Some(b"ctype"),
        next_state: ParserState::Criterion,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'floating_from",
        identifier: Some(b"ctype"),
        next_state: ParserState::CriterionFrom,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'tiling_from",
        identifier: Some(b"ctype"),
        next_state: ParserState::CriterionFrom,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'tiling",
        identifier: Some(b"ctype"),
        next_state: ParserState::Call,
        call_identifier: 143,
    },
    GrammarToken {
        name: b"'floating",
        identifier: Some(b"ctype"),
        next_state: ParserState::Call,
        call_identifier: 144,
    },
    GrammarToken {
        name: b"'all",
        identifier: Some(b"ctype"),
        next_state: ParserState::Call,
        call_identifier: 145,
    },
    GrammarToken {
        name: b"']",
        identifier: None,
        next_state: ParserState::Call,
        call_identifier: 146,
    },
];
static TOKENS_GAPS_END: [GrammarToken; 2] = [
    GrammarToken {
        name: b"'px",
        identifier: None,
        next_state: ParserState::GapsEnd,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"end",
        identifier: None,
        next_state: ParserState::Call,
        call_identifier: 147,
    },
];
static TOKENS_MODENAME: [GrammarToken; 2] = [
    GrammarToken {
        name: b"'--pango_markup",
        identifier: Some(b"pango_markup"),
        next_state: ParserState::Modename,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"word",
        identifier: Some(b"modename"),
        next_state: ParserState::Call,
        call_identifier: 148,
    },
];
static TOKENS_NO_FOCUS: [GrammarToken; 1] = [GrammarToken {
    name: b"'[",
    identifier: None,
    next_state: ParserState::Call,
    call_identifier: 149,
}];
static TOKENS_BINDING: [GrammarToken; 20] = [
    GrammarToken {
        name: b"'--release",
        identifier: Some(b"release"),
        next_state: ParserState::Binding,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'--border",
        identifier: Some(b"border"),
        next_state: ParserState::Binding,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'--whole-window",
        identifier: Some(b"whole_window"),
        next_state: ParserState::Binding,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'--exclude-titlebar",
        identifier: Some(b"exclude_titlebar"),
        next_state: ParserState::Binding,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'Mod1",
        identifier: Some(b"modifiers"),
        next_state: ParserState::Binding,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'Mod2",
        identifier: Some(b"modifiers"),
        next_state: ParserState::Binding,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'Mod3",
        identifier: Some(b"modifiers"),
        next_state: ParserState::Binding,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'Mod4",
        identifier: Some(b"modifiers"),
        next_state: ParserState::Binding,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'Mod5",
        identifier: Some(b"modifiers"),
        next_state: ParserState::Binding,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'Shift",
        identifier: Some(b"modifiers"),
        next_state: ParserState::Binding,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'Control",
        identifier: Some(b"modifiers"),
        next_state: ParserState::Binding,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'Ctrl",
        identifier: Some(b"modifiers"),
        next_state: ParserState::Binding,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'Mode_switch",
        identifier: Some(b"modifiers"),
        next_state: ParserState::Binding,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'Group1",
        identifier: Some(b"modifiers"),
        next_state: ParserState::Binding,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'Group2",
        identifier: Some(b"modifiers"),
        next_state: ParserState::Binding,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'Group3",
        identifier: Some(b"modifiers"),
        next_state: ParserState::Binding,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'Group4",
        identifier: Some(b"modifiers"),
        next_state: ParserState::Binding,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'$mod",
        identifier: Some(b"modifiers"),
        next_state: ParserState::Binding,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'+",
        identifier: None,
        next_state: ParserState::Binding,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"word",
        identifier: Some(b"key"),
        next_state: ParserState::Bindcommand,
        call_identifier: 0,
    },
];
static TOKENS_INCLUDE: [GrammarToken; 1] = [GrammarToken {
    name: b"string",
    identifier: Some(b"pattern"),
    next_state: ParserState::Call,
    call_identifier: 150,
}];
static TOKENS_INITIAL: [GrammarToken; 60] = [
    GrammarToken {
        name: b"end",
        identifier: None,
        next_state: ParserState::Initial,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"error",
        identifier: None,
        next_state: ParserState::Initial,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'#",
        identifier: None,
        next_state: ParserState::IgnoreLine,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'set ",
        identifier: None,
        next_state: ParserState::IgnoreLine,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'set\t",
        identifier: None,
        next_state: ParserState::IgnoreLine,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'set_from_resource",
        identifier: None,
        next_state: ParserState::IgnoreLine,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'include",
        identifier: None,
        next_state: ParserState::Include,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'bindsym",
        identifier: Some(b"bindtype"),
        next_state: ParserState::Binding,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'bindcode",
        identifier: Some(b"bindtype"),
        next_state: ParserState::Binding,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'bind",
        identifier: Some(b"bindtype"),
        next_state: ParserState::Binding,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'bar",
        identifier: None,
        next_state: ParserState::Barbrace,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'font",
        identifier: None,
        next_state: ParserState::Font,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'mode",
        identifier: None,
        next_state: ParserState::Modename,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'gaps",
        identifier: None,
        next_state: ParserState::Gaps,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'smart_borders",
        identifier: None,
        next_state: ParserState::SmartBorders,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'smart_gaps",
        identifier: None,
        next_state: ParserState::SmartGaps,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'floating_minimum_size",
        identifier: None,
        next_state: ParserState::FloatingMinimumSizeWidth,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'floating_maximum_size",
        identifier: None,
        next_state: ParserState::FloatingMaximumSizeWidth,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'floating_modifier",
        identifier: None,
        next_state: ParserState::FloatingModifier,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'default_orientation",
        identifier: None,
        next_state: ParserState::DefaultOrientation,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'workspace_layout",
        identifier: None,
        next_state: ParserState::WorkspaceLayout,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'default_border",
        identifier: Some(b"windowtype"),
        next_state: ParserState::DefaultBorder,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'new_window",
        identifier: Some(b"windowtype"),
        next_state: ParserState::DefaultBorder,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'default_floating_border",
        identifier: Some(b"windowtype"),
        next_state: ParserState::DefaultBorder,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'new_float",
        identifier: Some(b"windowtype"),
        next_state: ParserState::DefaultBorder,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'hide_edge_borders",
        identifier: None,
        next_state: ParserState::HideEdgeBorders,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'for_window",
        identifier: None,
        next_state: ParserState::ForWindow,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'assign",
        identifier: None,
        next_state: ParserState::Assign,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'no_focus",
        identifier: None,
        next_state: ParserState::NoFocus,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'focus_follows_mouse",
        identifier: None,
        next_state: ParserState::FocusFollowsMouse,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'mouse_warping",
        identifier: None,
        next_state: ParserState::MouseWarping,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'focus_wrapping",
        identifier: None,
        next_state: ParserState::FocusWrapping,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'force_focus_wrapping",
        identifier: None,
        next_state: ParserState::ForceFocusWrapping,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'force_xinerama",
        identifier: None,
        next_state: ParserState::ForceXinerama,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'force-xinerama",
        identifier: None,
        next_state: ParserState::ForceXinerama,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'disable_randr15",
        identifier: None,
        next_state: ParserState::DisableRandr15,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'disable-randr15",
        identifier: None,
        next_state: ParserState::DisableRandr15,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'workspace_auto_back_and_forth",
        identifier: None,
        next_state: ParserState::WorkspaceBackAndForth,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'fake_outputs",
        identifier: None,
        next_state: ParserState::FakeOutputs,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'fake-outputs",
        identifier: None,
        next_state: ParserState::FakeOutputs,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'force_display_urgency_hint",
        identifier: None,
        next_state: ParserState::ForceDisplayUrgencyHint,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'focus_on_window_activation",
        identifier: None,
        next_state: ParserState::FocusOnWindowActivation,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'title_align",
        identifier: None,
        next_state: ParserState::TitleAlign,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'show_marks",
        identifier: None,
        next_state: ParserState::ShowMarks,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'workspace",
        identifier: None,
        next_state: ParserState::Workspace,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'ipc_socket",
        identifier: None,
        next_state: ParserState::IpcSocket,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'ipc-socket",
        identifier: None,
        next_state: ParserState::IpcSocket,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'ipc_kill_timeout",
        identifier: None,
        next_state: ParserState::IpcKillTimeout,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'restart_state",
        identifier: None,
        next_state: ParserState::RestartState,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'popup_during_fullscreen",
        identifier: None,
        next_state: ParserState::PopupDuringFullscreen,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'tiling_drag",
        identifier: None,
        next_state: ParserState::TilingDrag,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'exec_always",
        identifier: Some(b"exectype"),
        next_state: ParserState::Exec,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'exec",
        identifier: Some(b"exectype"),
        next_state: ParserState::Exec,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'client.background",
        identifier: Some(b"colorclass"),
        next_state: ParserState::ColorSingle,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'client.focused_inactive",
        identifier: Some(b"colorclass"),
        next_state: ParserState::ColorBorder,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'client.focused_tab_title",
        identifier: Some(b"colorclass"),
        next_state: ParserState::ColorBorder,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'client.focused",
        identifier: Some(b"colorclass"),
        next_state: ParserState::ColorBorder,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'client.unfocused",
        identifier: Some(b"colorclass"),
        next_state: ParserState::ColorBorder,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'client.urgent",
        identifier: Some(b"colorclass"),
        next_state: ParserState::ColorBorder,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'client.placeholder",
        identifier: Some(b"colorclass"),
        next_state: ParserState::ColorBorder,
        call_identifier: 0,
    },
];
static TOKENS_ASSIGN: [GrammarToken; 1] = [GrammarToken {
    name: b"'[",
    identifier: None,
    next_state: ParserState::Call,
    call_identifier: 151,
}];
static TOKENS_BAR_ID: [GrammarToken; 1] = [GrammarToken {
    name: b"word",
    identifier: Some(b"bar_id"),
    next_state: ParserState::Call,
    call_identifier: 152,
}];
static TOKENS_EXEC: [GrammarToken; 2] = [
    GrammarToken {
        name: b"'--no-startup-id",
        identifier: Some(b"no_startup_id"),
        next_state: ParserState::Exec,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"string",
        identifier: Some(b"command"),
        next_state: ParserState::Call,
        call_identifier: 153,
    },
];
static TOKENS_FONT: [GrammarToken; 1] = [GrammarToken {
    name: b"string",
    identifier: Some(b"font"),
    next_state: ParserState::Call,
    call_identifier: 154,
}];
static TOKENS_GAPS: [GrammarToken; 8] = [
    GrammarToken {
        name: b"'inner",
        identifier: Some(b"scope"),
        next_state: ParserState::GapsWithScope,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'outer",
        identifier: Some(b"scope"),
        next_state: ParserState::GapsWithScope,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'horizontal",
        identifier: Some(b"scope"),
        next_state: ParserState::GapsWithScope,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'vertical",
        identifier: Some(b"scope"),
        next_state: ParserState::GapsWithScope,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'top",
        identifier: Some(b"scope"),
        next_state: ParserState::GapsWithScope,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'right",
        identifier: Some(b"scope"),
        next_state: ParserState::GapsWithScope,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'bottom",
        identifier: Some(b"scope"),
        next_state: ParserState::GapsWithScope,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'left",
        identifier: Some(b"scope"),
        next_state: ParserState::GapsWithScope,
        call_identifier: 0,
    },
];
static TOKENS_MODE: [GrammarToken; 8] = [
    GrammarToken {
        name: b"end",
        identifier: None,
        next_state: ParserState::Mode,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"error",
        identifier: None,
        next_state: ParserState::Mode,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'#",
        identifier: None,
        next_state: ParserState::ModeIgnoreLine,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'set",
        identifier: None,
        next_state: ParserState::ModeIgnoreLine,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'bindsym",
        identifier: Some(b"bindtype"),
        next_state: ParserState::ModeBinding,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'bindcode",
        identifier: Some(b"bindtype"),
        next_state: ParserState::ModeBinding,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'bind",
        identifier: Some(b"bindtype"),
        next_state: ParserState::ModeBinding,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'}",
        identifier: None,
        next_state: ParserState::Initial,
        call_identifier: 0,
    },
];
static TOKENS_BAR: [GrammarToken; 31] = [
    GrammarToken {
        name: b"end",
        identifier: None,
        next_state: ParserState::Bar,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"error",
        identifier: None,
        next_state: ParserState::Bar,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'#",
        identifier: None,
        next_state: ParserState::BarIgnoreLine,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'set",
        identifier: None,
        next_state: ParserState::BarIgnoreLine,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'i3bar_command",
        identifier: None,
        next_state: ParserState::BarBarCommand,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'status_command",
        identifier: None,
        next_state: ParserState::BarStatusCommand,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'workspace_command",
        identifier: None,
        next_state: ParserState::BarWorkspaceCommand,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'socket_path",
        identifier: None,
        next_state: ParserState::BarSocketPath,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'mode",
        identifier: None,
        next_state: ParserState::BarMode,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'hidden_state",
        identifier: None,
        next_state: ParserState::BarHiddenState,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'id",
        identifier: None,
        next_state: ParserState::BarId,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'modifier",
        identifier: None,
        next_state: ParserState::BarModifier,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'wheel_up_cmd",
        identifier: None,
        next_state: ParserState::BarWheelUpCmd,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'wheel_down_cmd",
        identifier: None,
        next_state: ParserState::BarWheelDownCmd,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'bindsym",
        identifier: None,
        next_state: ParserState::BarBindsym,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'position",
        identifier: None,
        next_state: ParserState::BarPosition,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'output",
        identifier: None,
        next_state: ParserState::BarOutput,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'tray_output",
        identifier: None,
        next_state: ParserState::BarTrayOutput,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'tray_padding",
        identifier: None,
        next_state: ParserState::BarTrayPadding,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'font",
        identifier: None,
        next_state: ParserState::BarFont,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'separator_symbol",
        identifier: None,
        next_state: ParserState::BarSeparatorSymbol,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'binding_mode_indicator",
        identifier: None,
        next_state: ParserState::BarBindingModeIndicator,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'workspace_buttons",
        identifier: None,
        next_state: ParserState::BarWorkspaceButtons,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'workspace_min_width",
        identifier: None,
        next_state: ParserState::BarWorkspaceMinWidth,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'strip_workspace_numbers",
        identifier: None,
        next_state: ParserState::BarStripWorkspaceNumbers,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'strip_workspace_name",
        identifier: None,
        next_state: ParserState::BarStripWorkspaceName,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'verbose",
        identifier: None,
        next_state: ParserState::BarVerbose,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'height",
        identifier: None,
        next_state: ParserState::BarHeight,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'padding",
        identifier: None,
        next_state: ParserState::BarPadding,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'colors",
        identifier: None,
        next_state: ParserState::BarColorsBrace,
        call_identifier: 0,
    },
    GrammarToken {
        name: b"'}",
        identifier: None,
        next_state: ParserState::Call,
        call_identifier: 155,
    },
];
static TOKENS: [&[GrammarToken]; 117] = [
    &TOKENS_FORCE_DISPLAY_URGENCY_HINT_MS,
    &TOKENS_FLOATING_MAXIMUM_SIZE_HEIGHT,
    &TOKENS_FLOATING_MINIMUM_SIZE_HEIGHT,
    &TOKENS_BAR_STRIP_WORKSPACE_NUMBERS,
    &TOKENS_FLOATING_MAXIMUM_SIZE_WIDTH,
    &TOKENS_FLOATING_MINIMUM_SIZE_WIDTH,
    &TOKENS_BAR_BINDING_MODE_INDICATOR,
    &TOKENS_BAR_WORKSPACE_MIN_WIDTH_PX,
    &TOKENS_FOCUS_ON_WINDOW_ACTIVATION,
    &TOKENS_FORCE_DISPLAY_URGENCY_HINT,
    &TOKENS_TILING_DRAG_SWAP_MODIFIER,
    &TOKENS_BAR_STRIP_WORKSPACE_NAME,
    &TOKENS_CRITERION_FROM_STR_START,
    &TOKENS_DEFAULT_BORDER_PIXELS_PX,
    &TOKENS_WORKSPACE_BACK_AND_FORTH,
    &TOKENS_ASSIGN_WORKSPACE_NUMBER,
    &TOKENS_BAR_WORKSPACE_MIN_WIDTH,
    &TOKENS_FLOATING_MAXIMUM_SIZE_X,
    &TOKENS_FLOATING_MINIMUM_SIZE_X,
    &TOKENS_POPUP_DURING_FULLSCREEN,
    &TOKENS_BAR_COLORS_IGNORE_LINE,
    &TOKENS_CRITERION_FROM_STR_END,
    &TOKENS_BAR_COLORS_BACKGROUND,
    &TOKENS_BAR_WORKSPACE_BUTTONS,
    &TOKENS_BAR_WORKSPACE_COMMAND,
    &TOKENS_DEFAULT_BORDER_PIXELS,
    &TOKENS_WORKSPACE_OUTPUT_WORD,
    &TOKENS_BAR_SEPARATOR_SYMBOL,
    &TOKENS_FORCE_FOCUS_WRAPPING,
    &TOKENS_BAR_BINDSYM_COMMAND,
    &TOKENS_BAR_TRAY_PADDING_PX,
    &TOKENS_DEFAULT_ORIENTATION,
    &TOKENS_FOCUS_FOLLOWS_MOUSE,
    &TOKENS_BAR_PADDING_BOTTOM,
    &TOKENS_BAR_STATUS_COMMAND,
    &TOKENS_BAR_WHEEL_DOWN_CMD,
    &TOKENS_COLOR_CHILD_BORDER,
    &TOKENS_CRITERION_FROM_STR,
    &TOKENS_FOR_WINDOW_COMMAND,
    &TOKENS_BAR_COLORS_BORDER,
    &TOKENS_BAR_COLORS_SINGLE,
    &TOKENS_BAR_PADDING_RIGHT,
    &TOKENS_FLOATING_MODIFIER,
    &TOKENS_HIDE_EDGE_BORDERS,
    &TOKENS_WORKSPACE_COMMAND,
    &TOKENS_ASSIGN_WORKSPACE,
    &TOKENS_BAR_COLORS_BRACE,
    &TOKENS_BAR_HIDDEN_STATE,
    &TOKENS_BAR_PADDING_LEFT,
    &TOKENS_BAR_TRAY_PADDING,
    &TOKENS_BAR_WHEEL_UP_CMD,
    &TOKENS_COLOR_BACKGROUND,
    &TOKENS_IPC_KILL_TIMEOUT,
    &TOKENS_MODE_BINDCOMMAND,
    &TOKENS_MODE_IGNORE_LINE,
    &TOKENS_TILING_DRAG_MODE,
    &TOKENS_WORKSPACE_LAYOUT,
    &TOKENS_BAR_BAR_COMMAND,
    &TOKENS_BAR_COLORS_TEXT,
    &TOKENS_BAR_IGNORE_LINE,
    &TOKENS_BAR_PADDING_TOP,
    &TOKENS_BAR_SOCKET_PATH,
    &TOKENS_BAR_TRAY_OUTPUT,
    &TOKENS_COLOR_INDICATOR,
    &TOKENS_DISABLE_RANDR15,
    &TOKENS_GAPS_WITH_SCOPE,
    &TOKENS_CRITERION_FROM,
    &TOKENS_DEFAULT_BORDER,
    &TOKENS_FOCUS_WRAPPING,
    &TOKENS_FORCE_XINERAMA,
    &TOKENS_ASSIGN_OUTPUT,
    &TOKENS_CRITERION_STR,
    &TOKENS_MOUSE_WARPING,
    &TOKENS_RESTART_STATE,
    &TOKENS_SMART_BORDERS,
    &TOKENS_BAR_MODIFIER,
    &TOKENS_BAR_POSITION,
    &TOKENS_COLOR_BORDER,
    &TOKENS_COLOR_SINGLE,
    &TOKENS_FAKE_OUTPUTS,
    &TOKENS_MODE_BINDING,
    &TOKENS_NO_FOCUS_END,
    &TOKENS_BAR_BINDSYM,
    &TOKENS_BAR_PADDING,
    &TOKENS_BAR_VERBOSE,
    &TOKENS_BINDCOMMAND,
    &TOKENS_IGNORE_LINE,
    &TOKENS_TILING_DRAG,
    &TOKENS_TITLE_ALIGN,
    &TOKENS_BAR_COLORS,
    &TOKENS_BAR_HEIGHT,
    &TOKENS_BAR_OUTPUT,
    &TOKENS_COLOR_TEXT,
    &TOKENS_FOR_WINDOW,
    &TOKENS_IPC_SOCKET,
    &TOKENS_SHOW_MARKS,
    &TOKENS_SMART_GAPS,
    &TOKENS_CRITERION,
    &TOKENS_MODEBRACE,
    &TOKENS_WORKSPACE,
    &TOKENS_BARBRACE,
    &TOKENS_BAR_FONT,
    &TOKENS_BAR_MODE,
    &TOKENS_CRITERIA,
    &TOKENS_GAPS_END,
    &TOKENS_MODENAME,
    &TOKENS_NO_FOCUS,
    &TOKENS_BINDING,
    &TOKENS_INCLUDE,
    &TOKENS_INITIAL,
    &TOKENS_ASSIGN,
    &TOKENS_BAR_ID,
    &TOKENS_EXEC,
    &TOKENS_FONT,
    &TOKENS_GAPS,
    &TOKENS_MODE,
    &TOKENS_BAR,
];
#[derive(Clone)]
struct ParserToken {
    name: Vec<u8>,
    identifier: Option<Vec<u8>>,
    next: ParserState,
}

// Immutable Rust metadata has no foreign layout or lifetime contract. Captures
// still own their token bytes, retaining the existing parser state algorithm.
fn parser_tokens(state: ParserState) -> Vec<ParserToken> {
    TOKENS[state as usize]
        .iter()
        .map(|token| ParserToken {
            name: token.name.to_vec(),
            identifier: token.identifier.map(<[u8]>::to_vec),
            next: token.next_state,
        })
        .collect()
}

#[derive(Debug)]
enum CapturedValue {
    String(Vec<u8>),
    Number(i64),
}
#[derive(Default)]
struct Captures(Vec<(Vec<u8>, CapturedValue)>);
impl Captures {
    fn string(&mut self, identifier: &[u8], value: Vec<u8>) -> Option<()> {
        if let Some((_, previous)) = self.0.iter_mut().find(|(key, _)| key == identifier) {
            let CapturedValue::String(previous) = previous else {
                return None;
            };
            previous.push(b',');
            previous.extend(value);
        } else {
            self.0
                .push((identifier.to_vec(), CapturedValue::String(value)));
        }
        Some(())
    }
    fn get(&self, identifier: &[u8]) -> Option<&[u8]> {
        self.0.iter().find_map(|(key, value)| match value {
            CapturedValue::String(bytes) if key == identifier => Some(bytes.as_slice()),
            _ => None,
        })
    }
}
#[derive(Debug, PartialEq, Eq)]
struct BindingFields {
    modifiers: Option<Vec<u8>>,
    key: Vec<u8>,
    release: Option<Vec<u8>>,
    command: Vec<u8>,
}
impl BindingFields {
    fn from_captures(captures: Captures) -> Option<Self> {
        Some(Self {
            modifiers: captures.get(b"modifiers").map(Vec::from),
            key: captures.get(b"key")?.to_vec(),
            release: captures.get(b"release").map(Vec::from),
            command: captures.get(b"command")?.to_vec(),
        })
    }
}
fn case_prefix(input: &[u8], word: &[u8]) -> bool {
    input.len() >= word.len()
        && input.iter().zip(word).all(|(&a, &b)| {
            // SAFETY: unsigned-byte inputs satisfy tolower's scalar precondition.
            // Keep locale-sensitive strncasecmp behavior, including raw byte input.
            unsafe { libc::tolower(i32::from(a)) == libc::tolower(i32::from(b)) }
        })
}
fn decimal_prefix(input: &[u8]) -> Option<(i64, usize)> {
    let string = CString::new(input).ok()?;
    let mut end = std::ptr::null_mut();
    // SAFETY: the temporary C string is live, and end is a writable pointer.
    let number = unsafe {
        *libc::__errno_location() = 0;
        let number = libc::strtol(string.as_ptr(), &mut end, 10);
        let errno = *libc::__errno_location();
        if errno == libc::ERANGE && (number == i64::MIN || number == i64::MAX)
            || errno != 0 && number == 0
        {
            return None;
        }
        number
    };
    // SAFETY: strtol's end pointer belongs to this same C-string allocation.
    let consumed = unsafe { end.offset_from(string.as_ptr()) } as usize;
    (consumed != 0).then_some((number, consumed))
}
fn scan_string(input: &[u8], whole_line: bool) -> Option<(Vec<u8>, usize)> {
    let quoted = input.first() == Some(&b'"');
    let beginning = usize::from(quoted);
    let mut end = beginning;
    while let Some(&byte) = input.get(end) {
        let stop = if quoted {
            byte == b'"' && input[end - 1] != b'\\'
        } else if whole_line {
            matches!(byte, b'\r' | b'\n')
        } else {
            matches!(byte, b' ' | b'\t' | b']' | b',' | b';' | b'\r' | b'\n')
        };
        if stop {
            break;
        }
        end += 1;
    }
    if end == beginning {
        return None;
    }
    let mut value = Vec::new();
    let mut at = beginning;
    while at < end {
        if input[at] == b'\\' && input.get(at + 1) == Some(&b'"') {
            at += 1;
        }
        value.push(input[at]);
        at += 1;
    }
    let consumed = end + usize::from(input.get(end) == Some(&b'"'));
    Some((value, consumed))
}
fn parse_binding(
    input: &[u8],
    mut table: impl FnMut(ParserState) -> Vec<ParserToken>,
) -> Option<BindingFields> {
    let input = input.split(|byte| *byte == 0).next().unwrap_or_default();
    let mut offset = 0;
    let mut state = ParserState::Initial;
    let mut captures = Captures::default();
    while offset <= input.len() {
        while matches!(input.get(offset), Some(b' ' | b'\t')) {
            offset += 1;
        }
        let rest = &input[offset..];
        let mut matched = false;
        for token in table(state) {
            let mut captured = None;
            let consumed = if token.name.first() == Some(&b'\'') {
                let literal = &token.name[1..];
                if !case_prefix(rest, literal) {
                    continue;
                }
                captured = Some(CapturedValue::String(literal.to_vec()));
                literal.len()
            } else if token.name == b"number" {
                let Some((number, count)) = decimal_prefix(rest) else {
                    continue;
                };
                captured = Some(CapturedValue::Number(number));
                count
            } else if token.name == b"string" || token.name == b"word" {
                let Some((value, count)) = scan_string(rest, token.name == b"string") else {
                    continue;
                };
                captured = Some(CapturedValue::String(value));
                count
            } else if token.name == b"end" && matches!(rest.first(), None | Some(b'\n' | b'\r')) {
                1
            } else {
                continue;
            };
            if let (Some(identifier), Some(value)) = (token.identifier, captured) {
                match value {
                    CapturedValue::String(value) => captures.string(&identifier, value)?,
                    CapturedValue::Number(number) => {
                        captures.0.push((identifier, CapturedValue::Number(number)))
                    }
                }
            }
            if token.next == ParserState::Call {
                return BindingFields::from_captures(captures);
            }
            state = token.next;
            offset += consumed;
            matched = true;
            break;
        }
        // Invalid input used to stall forever while walking raw pointers. A
        // failed rewrite is now bounded and cannot contaminate the next line.
        if !matched {
            return None;
        }
    }
    None
}

fn format_binding(binding: &BindingFields, keysym: &[u8]) -> Vec<u8> {
    let mut bytes = b"bindsym ".to_vec();
    if let Some(modifiers) = &binding.modifiers {
        bytes.extend(
            modifiers
                .iter()
                .map(|&byte| if byte == b',' { b'+' } else { byte }),
        );
        bytes.push(b'+');
    }
    bytes.extend_from_slice(keysym.split(|byte| *byte == 0).next().unwrap_or_default());
    bytes.push(b' ');
    if let Some(release) = &binding.release {
        bytes.extend(release);
    }
    bytes.extend(&binding.command);
    bytes.push(b'\n');
    bytes
}

/// Native key symbols borrow the initialized connection's setup/key table.
/// Requires sole event-thread access to those resources for this query.
fn keysym_used_on_other_key(keysyms: &KeySymbols, sym: u64, except_keycode: u8) -> bool {
    let setup = keysyms.connection().get_setup();
    let (minimum, maximum) = (setup.min_keycode(), setup.max_keycode());
    // Preserve the native zero-start sentinel and level order.
    minimum != 0
        && (minimum..=maximum)
            .filter(|&code| code != except_keycode)
            .any(|code| (0..4).any(|level| keysyms.keysym(code, level) as u64 == sym))
}
// Parsing and formatting use owned Rust bytes. Only keysym lookup borrows the
// initialized event-thread XKB/key-symbol native resources.
unsafe fn rewrite_binding(input: &[u8], keysyms: &KeySymbols) -> Option<Vec<u8>> {
    let binding = parse_binding(input, parser_tokens)?;
    let modifiers = binding.modifiers.as_deref();
    let key = CString::new(binding.key.clone()).expect("key has no NUL");
    let keycode = atoi(key.as_ptr());
    let mut level: i32 = 0_i32;
    if modifiers.is_some_and(|bytes| bytes.windows(5).any(|part| part == b"Shift")) {
        level = 1_i32;
        let mut syms: *const xkb_keysym_t = ::core::ptr::null::<xkb_keysym_t>();
        let num: i32 = xkb_keymap_key_get_syms_by_level(
            xkb_keymap,
            keycode as xkb_keycode_t,
            0 as xkb_layout_index_t,
            0 as xkb_level_index_t,
            &raw mut syms,
        );
        if num == 0_i32 {
            errx(
                1 as ::core::ffi::c_int,
                c"xkb_keymap_key_get_syms_by_level returned no symbols for keycode %d".as_ptr(),
                keycode,
            );
        }
        if !keysym_used_on_other_key(keysyms, *syms.offset(0_isize) as u64, keycode as u8) {
            level = 0_i32;
        }
    }
    let mut syms_0: *const xkb_keysym_t = ::core::ptr::null::<xkb_keysym_t>();
    let num_0: i32 = xkb_keymap_key_get_syms_by_level(
        xkb_keymap,
        keycode as xkb_keycode_t,
        0 as xkb_layout_index_t,
        level as xkb_level_index_t,
        &raw mut syms_0,
    );
    if num_0 == 0_i32 {
        errx(
            1 as ::core::ffi::c_int,
            c"xkb_keymap_key_get_syms_by_level returned no symbols for keycode %d".as_ptr(),
            keycode,
        );
    }
    if num_0 > 1_i32 {
        printf(
                c"xkb_keymap_key_get_syms_by_level (keycode = %d) returned %d symbolsinstead of 1, using only the first one.\n".as_ptr(),
                keycode,
                num_0,
            );
    }
    let mut keysym_name: [::core::ffi::c_char; 4096] = [0; 4096];
    if xkb_keysym_get_name(
        *syms_0.offset(0_isize),
        &raw mut keysym_name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as usize,
    ) == -1_i32
    {
        errx(
            libc::EXIT_FAILURE,
            c"xkb_keysym_get_name(%u) failed".as_ptr(),
            *syms_0.offset(0_isize),
        );
    }
    Some(format_binding(
        &binding,
        CStr::from_ptr(keysym_name.as_ptr()).to_bytes(),
    ))
}

/// Program-local byte sink: verbose records use stdout, errors stderr, and
/// debug remains suppressed exactly as in the original tools/bar logger.
pub fn write_log_bytes(level: i3_common::logging::Level, message: &[u8]) {
    use i3_common::logging::Level;
    unsafe {
        let stream = match level {
            Level::Error => stderr,
            Level::Verbose => stdout,
            Level::Debug => return,
        };
        libc::fwrite(message.as_ptr().cast(), message.len(), 1, stream);
    }
}

struct WizardUi {
    step: WizardStep,
    modifier: WizardModifier,
    config_path: std::path::PathBuf,
    socket: Option<std::path::PathBuf>,
    modifier_codes: [Vec<u8>; 2],
}
fn clicked_modifier(
    x: i16,
    y: i16,
    glyph_width: i32,
    font_height: i32,
    margin: i32,
) -> Option<WizardModifier> {
    let (x, y, w, h, margin) = (
        i64::from(x),
        i64::from(y),
        i64::from(glyph_width),
        i64::from(font_height),
        i64::from(margin),
    );
    if x < 4 * w + margin || x > 9 * w + margin {
        return None;
    }
    // Shared boundary belongs to Alt: the original two independent ifs chose
    // Win first and then Alt when the rows touched.
    if y >= 4 * h + margin && y <= 5 * h + margin {
        Some(WizardModifier::Alt)
    } else if y >= 3 * h + margin && y <= 4 * h + margin {
        Some(WizardModifier::Win)
    } else {
        None
    }
}

// Session ownership keeps fonts, graphics and leaf-window teardown explicit.
// Rendering borrows each owning font directly; no selected-font POD is stored.
struct StartupContext(*mut SnLauncheeContext);
impl Drop for StartupContext {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe {
                sn_launchee_context_unref(self.0);
            }
        }
    }
}
struct WizardResources {
    _registration: i3_common::display::DisplayConnection,
    connection: Rc<xcb::Connection>,
    rendering: Option<Rc<RenderingContext>>,
    window_owner: Option<OwnedWindow>,
    keysyms: Option<KeySymbols>,
    symbols: Option<KeySymbols>,
    fonts: bool,
    surface: bool,
}
impl Drop for WizardResources {
    fn drop(&mut self) {
        // SAFETY: the event loop/borrowed events have ended on their owning
        // thread. Release render/font/key resources before clearing provider
        // borrows and closing this session's uniquely owned connection.
        unsafe {
            if self.surface {
                surface.clear();
            }
            if self.fonts {
                bold_font = Font::empty();
                font = Font::empty();
            }
            if !xkb_keymap.is_null() {
                xkb_keymap_unref(xkb_keymap);
                xkb_keymap = std::ptr::null_mut();
            }
            drop(self.symbols.take());
            drop(self.keysyms.take());
            if let Some(window) = self.window_owner.as_mut() {
                window.close();
            }
            drop(self.window_owner.take());
            win = 0;
            // Surface and key/font users are gone before cached GCs.
            drop(self.rendering.take());
            root_screen = None;
            visual_type = std::ptr::null_mut();
            root = 0;
            conn = std::ptr::null_mut();
            // Registry and owning connection drop after native teardown.
        }
    }
}
unsafe fn atom_reply(
    connection: &xcb::Connection,
    cookie: x::InternAtomCookie,
    name: &[u8],
) -> x::Atom {
    let Ok(reply) = connection.wait_for_reply(cookie) else {
        let native_name = CString::new(name.split(|&byte| byte == 0).next().unwrap_or_default())
            .expect("first-NUL atom name");
        errx(
            libc::EXIT_FAILURE,
            c"Could not get atom %s".as_ptr(),
            native_name.as_ptr(),
        );
    };
    reply.atom()
}

struct WizardAtoms {
    name: x::Atom,
    utf8: x::Atom,
}

unsafe fn txt(text_font: &Font, col: i32, row: i32, text: &[u8], fg: color_t, bg: color_t) {
    let x: i32 = (col - 1_i32) * char_width + logical_px(4_i32);
    let y: i32 = (row - 1_i32) * font.height + logical_px(4_i32);
    let string = i3_common::string::I3String::from_bytes(
        &text[..text
            .iter()
            .position(|&byte| byte == 0)
            .unwrap_or(text.len())],
    );
    draw_util_text(
        text_font,
        &string,
        &raw mut surface,
        fg,
        bg,
        x,
        y,
        logical_px(300_i32) - x - logical_px(4_i32),
    );
}
unsafe fn handle_expose(ui: &WizardUi, connection: &xcb::Connection) -> i32 {
    let black: color_t = draw_util_hex_to_color(root_screen.as_ref(), "#000000") as color_t;
    let white: color_t = draw_util_hex_to_color(root_screen.as_ref(), "#FFFFFF") as color_t;
    let green: color_t = draw_util_hex_to_color(root_screen.as_ref(), "#00FF00") as color_t;
    let red: color_t = draw_util_hex_to_color(root_screen.as_ref(), "#FF0000") as color_t;
    draw_util_clear_surface(&raw mut surface, black);

    if ui.step == WizardStep::Welcome {
        txt(
            &font,
            2_i32,
            2_i32,
            b"You have not configured i3 yet.",
            white,
            black,
        );
        txt(
            &font,
            2_i32,
            3_i32,
            b"Do you want me to generate a config at",
            white,
            black,
        );
        let mut message = ui.config_path.as_os_str().as_bytes().to_vec();
        message.push(b'?');
        txt(&font, 2_i32, 4_i32, &message, white, black);

        txt(
            &font,
            13_i32,
            6_i32,
            b"Yes, generate the config",
            white,
            black,
        );
        txt(
            &font,
            13_i32,
            8_i32,
            b"No, I will use the defaults",
            white,
            black,
        );
        txt(&font, 4_i32, 6_i32, b"<Enter>", green, black);
        txt(&font, 5_i32, 8_i32, b"<ESC>", red, black);
    }
    if ui.step == WizardStep::Generate {
        txt(&font, 2_i32, 2_i32, b"Please choose either:", white, black);
        txt(
            &font,
            13_i32,
            4_i32,
            b"Win as default modifier",
            white,
            black,
        );
        txt(
            &font,
            13_i32,
            5_i32,
            b"Alt as default modifier",
            white,
            black,
        );
        txt(&font, 2_i32, 7_i32, b"Afterwards, press", white, black);
        txt(&font, 13_i32, 9_i32, b"to write the config", white, black);
        txt(&font, 13_i32, 10_i32, b"to abort", white, black);
        if ui.modifier == WizardModifier::Win {
            txt(&font, 5_i32, 5_i32, b"<Alt>", white, black);
        } else {
            txt(&font, 5_i32, 4_i32, b"<Win>", white, black);
        }

        if ui.modifier == WizardModifier::Win {
            txt(&bold_font, 2_i32, 4_i32, b"-> <Win>", white, black);
        } else {
            txt(&bold_font, 2_i32, 5_i32, b"-> <Alt>", white, black);
        }

        txt(&font, 4_i32, 9_i32, b"<Enter>", green, black);
        txt(&font, 5_i32, 10_i32, b"<ESC>", red, black);
    }
    let _ = connection.flush();
    1_i32
}
unsafe fn handle_key_press(
    ui: &mut WizardUi,
    connection: &xcb::Connection,
    symbols: &KeySymbols,
    keysyms: &KeySymbols,
    atoms: &WizardAtoms,
    event: &x::KeyPressEvent,
) -> bool {
    printf(
        c"Keypress %d, state raw = %d\n".as_ptr(),
        event.detail() as ::core::ffi::c_int,
        key_press_state(event) as ::core::ffi::c_int,
    );
    let mut state_filtered: u16 =
        (key_press_state(event) as u32 & !(xcb_numlock_mask | x::ModMask::LOCK.bits())) as u16;
    state_filtered = (state_filtered as i32 & 0xff_i32) as u16;
    let sym: u32 = symbols.keysym(event.detail(), i32::from(state_filtered));
    printf(c"sym = %c (%d)\n".as_ptr(), sym, sym);
    if sym == XK_Return as u32 || sym == XK_KP_Enter as u32 {
        if ui.step == WizardStep::Welcome {
            ui.step = WizardStep::Generate;
            connection.send_request(&x::ChangeProperty {
                mode: x::PropMode::Replace,
                window: x::Window::new(win),
                property: atoms.name,
                r#type: atoms.utf8,
                data: b"i3: generate config",
            });
            let _ = connection.flush();
        } else {
            finish(ui, keysyms);
            return true;
        }
    }
    if sym == XK_Up as u32 || sym == XK_Down as u32 {
        ui.modifier = if ui.modifier == WizardModifier::Alt {
            WizardModifier::Win
        } else {
            WizardModifier::Alt
        };
        handle_expose(ui, connection);
    }
    if sym == XK_Escape as u32 {
        return true;
    }
    for (index, label, selected) in [
        (0usize, c"Mod1", WizardModifier::Alt),
        (1, c"Mod4", WizardModifier::Win),
    ] {
        for &code in &ui.modifier_codes[index] {
            if code == 0 {
                continue;
            }
            printf(
                c"Modifier keycode for %s: 0x%02x\n".as_ptr(),
                label.as_ptr(),
                i32::from(code),
            );
            if code == event.detail() {
                ui.modifier = selected;
                printf(c"This is %s!\n".as_ptr(), label.as_ptr());
            }
        }
    }
    handle_expose(ui, connection);
    false
}
unsafe fn handle_button_press(
    ui: &mut WizardUi,
    connection: &xcb::Connection,
    event: &x::ButtonPressEvent,
) {
    if ui.step != WizardStep::Generate {
        return;
    }
    if let Some(modifier) = clicked_modifier(
        event.event_x(),
        event.event_y(),
        char_width,
        font.height,
        logical_px(4),
    ) {
        ui.modifier = modifier;
        handle_expose(ui, connection);
    }
}
const CONFIG_TEMPLATE: &str = "/usr/local/etc/i3/config.keycodes";
const CONFIG_HEADER: &[u8] = b"# This file has been auto-generated by i3-config-wizard(1).\n# It will not be overwritten, so edit it as you like.\n#\n# Should you change your keyboard layout some time, delete\n# this file and re-run i3-config-wizard(1).\n#\n";

struct TemplateLines {
    head: bool,
}
impl TemplateLines {
    fn rewrite(
        &mut self,
        line: &[u8],
        modifier: WizardModifier,
        mut binding: impl FnMut(&[u8]) -> Option<Vec<u8>>,
    ) -> Result<Option<Vec<u8>>, ()> {
        let line = line.split(|byte| *byte == 0).next().unwrap_or_default();
        if self.head && line.starts_with(b"# WARNING") {
            return Ok(None);
        }
        self.head = false;
        let indentation = line
            .iter()
            .take_while(|&&byte| {
                // SAFETY: isspace accepts every unsigned-char value.
                unsafe { libc::isspace(i32::from(byte)) != 0 }
            })
            .count();
        let (indent, rest) = line.split_at(indentation);
        let mut output = indent.to_vec();
        if rest.starts_with(b"set $mod ") {
            output.extend_from_slice(if modifier == WizardModifier::Alt {
                b"set $mod Mod1\n"
            } else {
                b"set $mod Mod4\n"
            });
        } else if rest.starts_with(b"bindcode") {
            output.extend_from_slice(&binding(rest).ok_or(())?);
        } else {
            output.extend_from_slice(rest);
        }
        Ok(Some(output))
    }
}
struct XkbContext(std::ptr::NonNull<xkb_context>);
impl Drop for XkbContext {
    fn drop(&mut self) {
        unsafe {
            xkb_context_unref(self.0.as_ptr());
        }
    }
}

// Paths, file buffers and output bytes are owned locally. Only the initialized
// display/XKB state remains a native boundary.
unsafe fn finish(ui: &WizardUi, keysyms: &KeySymbols) {
    use std::fs::File;
    use std::io::{BufRead, BufReader, Write};
    use std::os::unix::ffi::OsStrExt;
    let native_config_path =
        CString::new(ui.config_path.as_os_str().as_bytes()).expect("OS path has no NUL");
    printf(
        c"creating \"%s\"...\n".as_ptr(),
        native_config_path.as_ptr(),
    );
    let Some(context) = std::ptr::NonNull::new(xkb_context_new(XKB_CONTEXT_NO_FLAGS)) else {
        errx(1, c"could not create xkbcommon context".as_ptr());
    };
    let context = XkbContext(context);
    let device_id = xkb_x11_get_core_keyboard_device_id(conn.cast());
    xkb_keymap = xkb_x11_keymap_new_from_device(
        context.0.as_ptr(),
        conn.cast(),
        device_id,
        XKB_KEYMAP_COMPILE_NO_FLAGS,
    );
    if xkb_keymap.is_null() {
        errx(1, c"xkb_x11_keymap_new_from_device failed".as_ptr());
    }
    let input = File::open(CONFIG_TEMPLATE).unwrap_or_else(|error| {
        let native_template = CString::new(CONFIG_TEMPLATE).expect("static OS path has no NUL");
        *__errno_location() = error.raw_os_error().unwrap_or(libc::EIO);
        err(
            1,
            c"Could not open input file \"%s\"".as_ptr(),
            native_template.as_ptr(),
        );
    });
    let mut output = File::create(&ui.config_path).unwrap_or_else(|error| {
        *__errno_location() = error.raw_os_error().unwrap_or(libc::EIO);
        err(
            1,
            c"Could not open output config file \"%s\"".as_ptr(),
            native_config_path.as_ptr(),
        );
    });
    let mut input = BufReader::new(input);
    let _ = output.write_all(CONFIG_HEADER);
    let mut template = TemplateLines { head: true };
    let mut line = Vec::new();
    loop {
        line.clear();
        match input.read_until(b'\n', &mut line) {
            Ok(0) | Err(_) => break, // getline historically stops on read failure
            Ok(_) => {}
        }
        match template.rewrite(&line, ui.modifier, |input| rewrite_binding(input, keysyms)) {
            Ok(Some(bytes)) => {
                let _ = output.write_all(&bytes);
            }
            Ok(None) => {}
            Err(()) => errx(1, c"Could not rewrite config binding".as_ptr()),
        }
    }
    let _ = output.flush();
    let _ = output.sync_all();
    drop(output);
    let mut socket = connect_or_exit(ui.socket.as_deref());
    let _ = send_message_retry_would_block(&mut socket, 0, b"reload");
}
#[derive(Clone, Copy)]
enum ModifierKey {
    Alt,
    Win,
}
#[derive(Default)]
struct WizardSettings {
    socket: Option<std::path::PathBuf>,
    modifier: Option<ModifierKey>,
}
const WIZARD_OPTIONS: &[i3_common::cli::OptionSpec] = &[
    i3_common::cli::OptionSpec::value(b's', "socket"),
    i3_common::cli::OptionSpec::flag(b'v', "version"),
    i3_common::cli::OptionSpec::value(b'm', "modifier"),
    i3_common::cli::OptionSpec::value(b'l', "limit").long_only(),
    i3_common::cli::OptionSpec::value(b'P', "prompt").long_only(),
    i3_common::cli::OptionSpec::value(b'p', "prefix").long_only(),
    i3_common::cli::OptionSpec::value(b'f', "font").long_only(),
    i3_common::cli::OptionSpec::flag(b'h', "help"),
];
unsafe fn run_window(settings: WizardSettings) -> i32 {
    let pattern = "pango:monospace 8";
    let patternbold = "pango:monospace bold 8";
    let headless_run = settings.modifier.is_some();
    let selected_modifier = match settings.modifier {
        Some(ModifierKey::Alt) => WizardModifier::Alt,
        _ => WizardModifier::Win,
    };
    if let Some(path) = get_config_path(None, false) {
        let native_path = CString::new(path.as_os_str().as_bytes()).expect("OS path has no NUL");
        printf(
            c"The config file \"%s\" already exists. Exiting.\n".as_ptr(),
            native_path.as_ptr(),
        );
        return 0_i32;
    }
    let config_home = std::env::var_os("XDG_CONFIG_HOME").unwrap_or_else(|| "~/.config".into());
    let base = resolve_or_exit(std::path::Path::new(&config_home));
    // Append exact byte suffixes rather than normalizing repeated separators.
    let mut directory = base.as_os_str().as_bytes().to_vec();
    directory.extend_from_slice(b"/i3");
    let config_dir = std::path::PathBuf::from(std::ffi::OsString::from_vec(directory));
    let mut path = config_dir.as_os_str().as_bytes().to_vec();
    path.extend_from_slice(b"/config");
    let config_path = std::path::PathBuf::from(std::ffi::OsString::from_vec(path));
    let native_config_dir =
        CString::new(config_dir.as_os_str().as_bytes()).expect("OS path has no NUL");
    let native_config_path =
        CString::new(config_path.as_os_str().as_bytes()).expect("OS path has no NUL");
    let mut stbuf: stat = ::core::mem::zeroed();
    if stat(native_config_dir.as_ptr(), &raw mut stbuf) != 0 {
        let directory = config_dir.as_path();
        if let Err(error) = create_dir_all(directory, DEFAULT_DIR_MODE) {
            let error = report_create_error(directory, error);
            *__errno_location() = error.raw_os_error().unwrap_or(libc::EIO);
            err(
                libc::EXIT_FAILURE,
                c"mkdirp(%s) failed".as_ptr(),
                native_config_dir.as_ptr(),
            );
        }
    }
    let mut fd: i32 = 0;
    fd = open(
        native_config_path.as_ptr(),
        libc::O_CREAT | libc::O_RDWR,
        0o644 as ::core::ffi::c_int,
    );
    if fd == -1_i32 {
        printf(
            c"Cannot open file \"%s\" for writing: %s. Exiting.\n".as_ptr(),
            native_config_path.as_ptr(),
            strerror(*__errno_location()),
        );
        return 0_i32;
    }
    close(fd);
    unlink(native_config_path.as_ptr());
    let mut ui = WizardUi {
        step: WizardStep::Welcome,
        modifier: selected_modifier,
        config_path,
        socket: settings.socket,
        modifier_codes: [Vec::new(), Vec::new()],
    };
    let Ok((connection, screen)) = xcb::Connection::connect(None) else {
        errx(libc::EXIT_FAILURE, c"Cannot open display".as_ptr());
    };
    let connection = Rc::new(connection);
    conn = connection.get_raw_conn().cast();
    let registration = i3_common::display::install_connection(connection.clone());
    let mut resources = WizardResources {
        _registration: registration,
        connection: connection.clone(),
        rendering: None,
        window_owner: None,
        keysyms: None,
        symbols: None,
        fonts: false,
        surface: false,
    };
    if xkb_x11_setup_xkb_extension(
        conn.cast(),
        XKB_X11_MIN_MAJOR_XKB_VERSION,
        XKB_X11_MIN_MINOR_XKB_VERSION,
        xkb_x11_setup_xkb_extension_flags::NO_FLAGS,
        ::core::ptr::null_mut::<u16>(),
        ::core::ptr::null_mut::<u16>(),
        &raw mut xkb_base_event,
        &raw mut xkb_base_error,
    ) != 1_i32
    {
        errx(
            libc::EXIT_FAILURE,
            c"Could not setup XKB extension.".as_ptr(),
        );
    }
    resources.keysyms = Some(
        KeySymbols::new(connection.clone())
            .unwrap_or_else(|| errx(1, c"Could not allocate key symbols".as_ptr())),
    );
    // The no-drop view borrows WizardResources' sole native connection owner.
    let modmap_connection = resources.connection.clone();
    let modmap_cookie = modmap_connection.send_request(&xcb::x::GetModifierMapping {});
    resources.symbols = Some(
        KeySymbols::new(connection.clone())
            .unwrap_or_else(|| errx(1, c"Could not allocate key symbols".as_ptr())),
    );
    if headless_run {
        drop(modmap_connection);
        finish(
            &ui,
            resources
                .keysyms
                .as_ref()
                .expect("rewrite symbols initialized"),
        );
        return 0_i32;
    }
    let intern = |name: &[u8]| {
        let name = name.split(|&byte| byte == 0).next().unwrap_or_default();
        connection.send_request(&x::InternAtom {
            only_if_exists: false,
            name: &name[..name.len() as u16 as usize],
        })
    };
    let name_cookie = intern(b"_NET_WM_NAME");
    let window_type_cookie = intern(b"_NET_WM_WINDOW_TYPE");
    let dialog_cookie = intern(b"_NET_WM_WINDOW_TYPE_DIALOG");
    let atom_cookie = intern(b"ATOM");
    let cardinal_cookie = intern(b"CARDINAL");
    let utf8_cookie = intern(b"UTF8_STRING");
    let sndisplay: *mut SnDisplay = sn_xcb_display_new(conn.cast(), None, None);
    let sncontext: *mut SnLauncheeContext =
        sn_launchee_context_new_from_environment(sndisplay, screen);
    let startup = StartupContext(sncontext);
    sn_display_unref(sndisplay);
    root_screen = i3_common::display::DisplayScreen::new(resources.connection.clone(), screen);
    if root_screen.is_none() {
        errx(1, c"Cannot find display screen".as_ptr());
    }
    let screen_info = root_screen
        .as_ref()
        .expect("initialized display screen")
        .info();
    root = screen_info.root;
    resources.rendering = Some(RenderingContext::new(
        root_screen
            .as_ref()
            .expect("initialized display screen")
            .clone(),
    ));
    visual_type = i3_common::display::visual(screen_info.root_visual)
        .expect("root Cairo visual")
        .as_ptr();
    let Ok(modmap) = modmap_connection.wait_for_reply(modmap_cookie) else {
        errx(
            libc::EXIT_FAILURE,
            c"Could not get modifier mapping".as_ptr(),
        );
    };
    let count = usize::from(modmap.keycodes_per_modifier());
    let Some(codes) = i3_common::get_mod_mask::modifier_keycodes(&modmap) else {
        errx(
            libc::EXIT_FAILURE,
            c"Could not get modifier mapping".as_ptr(),
        );
    };
    ui.modifier_codes = [
        codes[count * 3..count * 4].to_vec(),
        codes[count * 6..count * 7].to_vec(),
    ];
    xcb_numlock_mask = i3_common::get_mod_mask::modifier_mask_for(
        xkeysym::key::Num_Lock,
        resources
            .symbols
            .as_ref()
            .expect("input symbols initialized"),
        &modmap,
    );
    drop(modmap_connection);
    init_dpi(root_screen.as_ref().expect("initialized display screen"));
    let font_context = i3_common::font::FontContext::for_screen(
        root_screen.as_ref().expect("initialized display screen"),
    );
    font = load_font(&font_context, pattern, true);
    bold_font = load_font(&font_context, patternbold, true);
    resources.fonts = true;

    let letter = i3_common::string::I3String::new("a");
    char_width = predict_text_width(&font, &letter);
    let window: x::Window = connection.generate_id();
    win = window.resource_id();
    connection.send_request(&x::CreateWindow {
        depth: 0,
        wid: window,
        parent: x::Window::new(root),
        x: logical_px(490) as i16,
        y: logical_px(297) as i16,
        width: logical_px(300) as u16,
        height: (15 * font.height + logical_px(4)) as u16,
        border_width: 0,
        class: x::WindowClass::InputOutput,
        visual: 0,
        value_list: &[
            x::Cw::BackPixel(0),
            x::Cw::EventMask(x::EventMask::EXPOSURE | x::EventMask::BUTTON_PRESS),
        ],
    });
    // Unique leaf XID; preserve the unchecked create/error route and the exact
    // former explicit close position in the session destructor.
    resources.window_owner = Some(OwnedWindow::from_created(connection.clone(), window));
    if !sncontext.is_null() {
        sn_launchee_context_setup_window(sncontext, win as libc::c_ulong);
    }
    connection.send_request(&x::MapWindow { window });
    let name = atom_reply(&connection, name_cookie, b"_NET_WM_NAME");
    let window_type = atom_reply(&connection, window_type_cookie, b"_NET_WM_WINDOW_TYPE");
    let dialog = atom_reply(&connection, dialog_cookie, b"_NET_WM_WINDOW_TYPE_DIALOG");
    let atom = atom_reply(&connection, atom_cookie, b"ATOM");
    let _cardinal = atom_reply(&connection, cardinal_cookie, b"CARDINAL");
    let utf8 = atom_reply(&connection, utf8_cookie, b"UTF8_STRING");
    let atoms = WizardAtoms { name, utf8 };
    connection.send_request(&x::ChangeProperty {
        mode: x::PropMode::Replace,
        window,
        property: window_type,
        r#type: atom,
        data: &[dialog.resource_id()],
    });
    connection.send_request(&x::ChangeProperty {
        mode: x::PropMode::Replace,
        window,
        property: name,
        r#type: utf8,
        data: b"i3: first configuration",
    });
    let rendering = resources
        .rendering
        .as_ref()
        .expect("session rendering context");
    let visual = rendering
        .visual(rendering.screen().info().root_visual)
        .expect("root visual belongs to the selected screen");
    surface = surface_t::for_drawable(
        rendering.clone(),
        win,
        visual,
        logical_px(300_i32),
        15_i32 * font.height + logical_px(4_i32),
    );
    let _ = connection.flush();
    resources.surface = true;
    let mut status = None;
    for _ in 0..500 {
        let reply = connection.wait_for_reply(connection.send_request(&x::GrabKeyboard {
            owner_events: false,
            grab_window: window,
            time: x::CURRENT_TIME,
            pointer_mode: x::GrabMode::Async,
            keyboard_mode: x::GrabMode::Async,
        }));
        status = reply.as_ref().ok().map(keyboard_grab_status);
        usleep(1000);
        if status == Some(Ok(x::GrabStatus::Success)) {
            break;
        }
    }
    if status != Some(Ok(x::GrabStatus::Success)) {
        fprintf(
            stderr,
            c"Could not grab keyboard, status = %d\n".as_ptr(),
            status.map_or(-1, |status| {
                status.map_or_else(i32::from, |status| status as i32)
            }),
        );
        return -1;
    }
    if !sncontext.is_null() {
        sn_launchee_context_complete(sncontext);
    }
    drop(startup);
    let _ = connection.flush();
    let symbols = resources
        .symbols
        .as_ref()
        .expect("input symbols initialized");
    let keysyms = resources
        .keysyms
        .as_ref()
        .expect("rewrite symbols initialized");
    loop {
        let event = match connection.wait_for_event() {
            Ok(event) => event,
            Err(xcb::Error::Protocol(error)) => {
                fprintf(
                    stderr,
                    c"X11 Error received! sequence %x\n".as_ptr(),
                    (*error.as_raw()).sequence as i32,
                );
                continue;
            }
            Err(xcb::Error::Connection(_)) => break,
        };
        match event {
            xcb::Event::X(x::Event::KeyPress(event)) => {
                if handle_key_press(&mut ui, &connection, symbols, keysyms, &atoms, &event) {
                    return 0;
                }
            }
            xcb::Event::X(x::Event::ButtonPress(event)) => {
                handle_button_press(&mut ui, &connection, &event);
            }
            xcb::Event::X(x::Event::Expose(event)) if event.count() == 0 => {
                handle_expose(&ui, &connection);
            }
            _ => {}
        }
    }
    0
}

pub fn main() {
    use i3_common::cli::{Item, Parser};
    use std::os::unix::ffi::OsStrExt;
    let mut arguments = std::env::args_os();
    let program = arguments
        .next()
        .unwrap_or_else(|| "i3-config-wizard".into());
    let mut settings = WizardSettings::default();
    for item in Parser::new(
        arguments,
        WIZARD_OPTIONS,
        std::env::var_os("POSIXLY_CORRECT").is_some(),
    ) {
        match item {
            Item::Error(message) => eprintln!("{}: {message}", program.to_string_lossy()),
            Item::Operand(_) => {}
            Item::Option { short, value } => match short {
                b'v' => {
                    println!("i3-config-wizard {}", env!("I3_VERSION"));
                    return;
                }
                b'h' => {
                    println!("i3-config-wizard {}\ni3-config-wizard [-s <socket>] [-m win|alt] [-v] [-h]", env!("I3_VERSION"));
                    return;
                }
                b's' => settings.socket = Some(std::path::PathBuf::from(value.unwrap())),
                b'm' => {
                    let value = value.unwrap();
                    settings.modifier = Some(match value.as_bytes() {
                        b"alt" => ModifierKey::Alt,
                        b"win" => ModifierKey::Win,
                        _ => {
                            eprintln!(
                                "{}: Invalid modifier key {}: Success",
                                std::path::Path::new(&program)
                                    .file_name()
                                    .unwrap_or(&program)
                                    .to_string_lossy(),
                                value.to_string_lossy()
                            );
                            std::process::exit(1);
                        }
                    });
                }
                // These long options were accepted but unused upstream.
                b'l' | b'P' | b'p' | b'f' => {}
                _ => unreachable!(),
            },
        }
    }
    // SAFETY: settings retains the socket string throughout the native event
    // loop and the headless configuration path; callbacks borrow it only there.
    std::process::exit(unsafe { run_window(settings) });
}

// Canonical Cairo backend ABI; renderer logic remains local.
pub use cairo_sys::{cairo_surface_t, cairo_t};

// Canonical Pango backend ABI; font policy remains local.

use xcb::ffi::xcb_connection_t;

use xkbcommon::xkb::ffi::*;
use xkbcommon::xkb::x11::ffi::*;

use startup_notification_sys::{
    sn_display_unref, sn_launchee_context_complete, sn_launchee_context_new_from_environment,
    sn_launchee_context_setup_window, sn_launchee_context_unref, sn_xcb_display_new, SnDisplay,
    SnLauncheeContext,
};

use libc::{
    __errno_location, atoi, close, fprintf, free, getenv, open, printf, stat, strerror, strlen,
    unlink, usleep,
};

use i3_common::native_stdio::{err, errx, stderr, stdout};

// Internal calls and globals use canonical Rust identities.
use i3_common::get_config_path::get_config_path;
use i3_common::ipc_connect::connect_or_exit;
use i3_common::ipc_send_message::send_message_retry_would_block;
use i3_common::resolve_tilde::resolve_or_exit;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum WizardStep {
    Welcome = 0,
    Generate = 1,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum WizardModifier {
    Alt = 0,
    Win = 1,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum ParserState {
    Call = 117,
    Bar = 116,
    Mode = 115,
    Gaps = 114,
    Font = 113,
    Exec = 112,
    BarId = 111,
    Assign = 110,
    Initial = 109,
    Include = 108,
    Binding = 107,
    NoFocus = 106,
    Modename = 105,
    GapsEnd = 104,
    Criteria = 103,
    BarMode = 102,
    BarFont = 101,
    Barbrace = 100,
    Workspace = 99,
    Modebrace = 98,
    Criterion = 97,
    SmartGaps = 96,
    ShowMarks = 95,
    IpcSocket = 94,
    ForWindow = 93,
    ColorText = 92,
    BarOutput = 91,
    BarHeight = 90,
    BarColors = 89,
    TitleAlign = 88,
    TilingDrag = 87,
    IgnoreLine = 86,
    Bindcommand = 85,
    BarVerbose = 84,
    BarPadding = 83,
    BarBindsym = 82,
    NoFocusEnd = 81,
    ModeBinding = 80,
    FakeOutputs = 79,
    ColorSingle = 78,
    ColorBorder = 77,
    BarPosition = 76,
    BarModifier = 75,
    SmartBorders = 74,
    RestartState = 73,
    MouseWarping = 72,
    CriterionStr = 71,
    AssignOutput = 70,
    ForceXinerama = 69,
    FocusWrapping = 68,
    DefaultBorder = 67,
    CriterionFrom = 66,
    GapsWithScope = 65,
    DisableRandr15 = 64,
    ColorIndicator = 63,
    BarTrayOutput = 62,
    BarSocketPath = 61,
    BarPaddingTop = 60,
    BarIgnoreLine = 59,
    BarColorsText = 58,
    BarBarCommand = 57,
    WorkspaceLayout = 56,
    TilingDragMode = 55,
    ModeIgnoreLine = 54,
    ModeBindcommand = 53,
    IpcKillTimeout = 52,
    ColorBackground = 51,
    BarWheelUpCmd = 50,
    BarTrayPadding = 49,
    BarPaddingLeft = 48,
    BarHiddenState = 47,
    BarColorsBrace = 46,
    AssignWorkspace = 45,
    WorkspaceCommand = 44,
    HideEdgeBorders = 43,
    FloatingModifier = 42,
    BarPaddingRight = 41,
    BarColorsSingle = 40,
    BarColorsBorder = 39,
    ForWindowCommand = 38,
    CriterionFromStr = 37,
    ColorChildBorder = 36,
    BarWheelDownCmd = 35,
    BarStatusCommand = 34,
    BarPaddingBottom = 33,
    FocusFollowsMouse = 32,
    DefaultOrientation = 31,
    BarTrayPaddingPx = 30,
    BarBindsymCommand = 29,
    ForceFocusWrapping = 28,
    BarSeparatorSymbol = 27,
    WorkspaceOutputWord = 26,
    DefaultBorderPixels = 25,
    BarWorkspaceCommand = 24,
    BarWorkspaceButtons = 23,
    BarColorsBackground = 22,
    CriterionFromStrEnd = 21,
    BarColorsIgnoreLine = 20,
    PopupDuringFullscreen = 19,
    FloatingMinimumSizeX = 18,
    FloatingMaximumSizeX = 17,
    BarWorkspaceMinWidth = 16,
    AssignWorkspaceNumber = 15,
    WorkspaceBackAndForth = 14,
    DefaultBorderPixelsPx = 13,
    CriterionFromStrStart = 12,
    BarStripWorkspaceName = 11,
    TilingDragSwapModifier = 10,
    ForceDisplayUrgencyHint = 9,
    FocusOnWindowActivation = 8,
    BarWorkspaceMinWidthPx = 7,
    BarBindingModeIndicator = 6,
    FloatingMinimumSizeWidth = 5,
    FloatingMaximumSizeWidth = 4,
    BarStripWorkspaceNumbers = 3,
    FloatingMinimumSizeHeight = 2,
    FloatingMaximumSizeHeight = 1,
    ForceDisplayUrgencyHintMs = 0,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn immutable_grammar_keeps_every_original_token_state_and_call_id() {
        fn add(hash: &mut u64, bytes: &[u8]) {
            for &byte in bytes {
                *hash = (*hash ^ u64::from(byte)).wrapping_mul(0x100000001b3);
            }
        }
        let mut hash = 0xcbf29ce484222325u64;
        for (state, row) in TOKENS.iter().enumerate() {
            add(&mut hash, &(state as u32).to_le_bytes());
            add(&mut hash, &(row.len() as u32).to_le_bytes());
            for token in *row {
                add(&mut hash, &(token.name.len() as u32).to_le_bytes());
                add(&mut hash, token.name);
                add(&mut hash, &[u8::from(token.identifier.is_some())]);
                if let Some(identifier) = token.identifier {
                    add(&mut hash, &(identifier.len() as u32).to_le_bytes());
                    add(&mut hash, identifier);
                }
                add(&mut hash, &(token.next_state as u32).to_le_bytes());
                add(&mut hash, &token.call_identifier.to_le_bytes());
            }
        }
        // Independently recorded from the prior generated C-pointer tables.
        assert_eq!(hash, 0x451f06603a0965b3u64);
    }

    fn parse(input: &[u8]) -> Option<BindingFields> {
        // Generated grammar is immutable and this local parser never touches X.
        parse_binding(input, parser_tokens)
    }

    #[test]
    fn binding_parser_keeps_modifiers_commands_release_and_raw_bytes() {
        let binding = parse(b"bindcode $mod+Shift+36 exec terminal\n").unwrap();
        assert_eq!(binding.modifiers.as_deref(), Some(&b"$mod,Shift"[..]));
        assert_eq!(binding.key, b"36");
        assert_eq!(binding.command, b"exec terminal");
        assert_eq!(
            format_binding(&binding, b"Return"),
            b"bindsym $mod+Shift+Return exec terminal\n"
        );
        let binding = parse(b"BiNdCoDe --release 36 exec raw\xff\n").unwrap();
        assert_eq!(binding.release.as_deref(), Some(&b"--release"[..]));
        // Keep the historical no-space concatenation of release and command.
        assert_eq!(
            format_binding(&binding, b"Return"),
            b"bindsym Return --releaseexec raw\xff\n"
        );
        let quoted = parse(b"bindcode \"36\" \"exec \\\"quoted\\\"\"\n").unwrap();
        assert_eq!(quoted.key, b"36");
        assert_eq!(quoted.command, b"exec \"quoted\"");
        let with_tail = parse(b"bindcode 36 exec byte\xff\0ignored").unwrap();
        assert_eq!(with_tail.command, b"exec byte\xff");
        assert_eq!(
            format_binding(&with_tail, b"Return\0ignored"),
            b"bindsym Return exec byte\xff\n"
        );
    }

    #[test]
    fn malformed_lines_are_bounded_and_captures_do_not_leak_between_lines() {
        for line in [
            b"".as_slice(),
            b"bindcode",
            b"bindcode 36",
            b"bindcode 36 \"\"",
            b"not_a_binding 36 exec x",
        ] {
            assert!(parse(line).is_none(), "{line:?}");
        }
        let first = parse(b"bindcode Mod4+36 exec x").unwrap();
        let second = parse(b"bindcode 37 exec y").unwrap();
        assert!(first.modifiers.is_some());
        assert!(second.modifiers.is_none());
        let mut input = b"bindcode ".to_vec();
        for _ in 0..100 {
            input.extend_from_slice(b"Shift+");
        }
        input.extend_from_slice(b"36 exec done");
        let parsed = parse(&input).unwrap();
        assert_eq!(
            parsed
                .modifiers
                .unwrap()
                .split(|byte| *byte == b',')
                .count(),
            100
        );
    }

    #[test]
    fn token_scanning_matches_legacy_quoting_whitespace_and_number_prefixes() {
        assert_eq!(
            scan_string(b"\"a\\\"b\" trailing", false),
            Some((b"a\"b".to_vec(), 6))
        );
        assert_eq!(
            scan_string(b"unclosed", true),
            Some((b"unclosed".to_vec(), 8))
        );
        assert_eq!(
            scan_string(b"word,next", false),
            Some((b"word".to_vec(), 4))
        );
        assert_eq!(
            scan_string(b"exec one; exec two\rnext", true),
            Some((b"exec one; exec two".to_vec(), 18))
        );
        assert_eq!(decimal_prefix(b" -123suffix"), Some((-123, 5)));
        assert!(decimal_prefix(b"+").is_none());
        assert!(decimal_prefix(b"999999999999999999999999999999999").is_none());
    }

    #[test]
    fn template_output_preserves_warning_header_indent_nuls_and_modifier_choice() {
        let mut lines = TemplateLines { head: true };
        assert_eq!(
            lines
                .rewrite(b"# WARNING skip\n", WizardModifier::Win, |_| unreachable!())
                .unwrap(),
            None
        );
        assert_eq!(
            lines
                .rewrite(b"\n", WizardModifier::Win, |_| unreachable!())
                .unwrap(),
            Some(b"\n".to_vec())
        );
        assert_eq!(
            lines
                .rewrite(
                    b"# WARNING retained\n",
                    WizardModifier::Win,
                    |_| unreachable!()
                )
                .unwrap(),
            Some(b"# WARNING retained\n".to_vec())
        );
        assert_eq!(
            lines
                .rewrite(
                    b" \tset $mod Mod4\n",
                    WizardModifier::Alt,
                    |_| unreachable!()
                )
                .unwrap(),
            Some(b" \tset $mod Mod1\n".to_vec())
        );
        assert_eq!(
            lines
                .rewrite(
                    b"raw\xff\0ignored\n",
                    WizardModifier::Alt,
                    |_| unreachable!()
                )
                .unwrap(),
            Some(b"raw\xff".to_vec())
        );
        let output = lines
            .rewrite(
                b"\tbindcode $mod+36 exec x\n",
                WizardModifier::Win,
                |input| Some(format_binding(&parse(input).unwrap(), b"Return")),
            )
            .unwrap();
        assert_eq!(output, Some(b"\tbindsym $mod+Return exec x\n".to_vec()));
        assert!(lines
            .rewrite(b"bindcode invalid", WizardModifier::Win, |_| None)
            .is_err());
    }

    #[test]
    fn modifier_hit_test_keeps_inclusive_edges_and_alt_wins_shared_row() {
        assert_eq!(
            clicked_modifier(44, 34, 10, 10, 4),
            Some(WizardModifier::Win)
        );
        assert_eq!(
            clicked_modifier(94, 43, 10, 10, 4),
            Some(WizardModifier::Win)
        );
        assert_eq!(
            clicked_modifier(44, 44, 10, 10, 4),
            Some(WizardModifier::Alt)
        );
        assert_eq!(
            clicked_modifier(94, 54, 10, 10, 4),
            Some(WizardModifier::Alt)
        );
        assert_eq!(clicked_modifier(43, 40, 10, 10, 4), None);
        assert_eq!(clicked_modifier(95, 40, 10, 10, 4), None);
        assert_eq!(clicked_modifier(44, 55, 10, 10, 4), None);
        assert_eq!(
            clicked_modifier(i16::MIN, i16::MAX, i32::MAX, i32::MAX, i32::MIN),
            None
        );
    }
}
