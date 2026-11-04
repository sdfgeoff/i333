use i3_common::draw_util::draw_util_clear_surface;
use i3_common::draw_util::draw_util_hex_to_color;
use i3_common::draw_util::draw_util_text;
use i3_common::draw_util::{OwnedWindow, RenderingContext};
use i3_common::font::load_font;
use i3_common::font::predict_text_width;
use i3_common::mkdirp::{create_dir_all, report_create_error};
use std::ffi::{CStr, CString};
use std::os::unix::ffi::OsStrExt;
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

pub type cmdp_token = token;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct token {
    pub name: *mut ::core::ffi::c_char,
    pub identifier: *mut ::core::ffi::c_char,
    pub next_state: ParserState,
    pub extra: WizardTokenData,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union WizardTokenData {
    pub call_identifier: u16,
}

pub type cmdp_token_ptr = tokenptr;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tokenptr {
    pub array: *mut cmdp_token,
    pub n: ::core::ffi::c_int,
}

pub const DEFAULT_DIR_MODE: libc::mode_t =
    libc::S_IRWXU | libc::S_IRGRP | libc::S_IXGRP | libc::S_IROTH | libc::S_IXOTH;

pub const XK_Return: ::core::ffi::c_int = 0xff0d as ::core::ffi::c_int;
pub const XK_Escape: ::core::ffi::c_int = 0xff1b as ::core::ffi::c_int;
pub const XK_Up: ::core::ffi::c_int = 0xff52 as ::core::ffi::c_int;
pub const XK_Down: ::core::ffi::c_int = 0xff54 as ::core::ffi::c_int;
pub const XK_KP_Enter: ::core::ffi::c_int = 0xff8d as ::core::ffi::c_int;

pub static mut visual_type: *mut cairo_sys::xcb_visualtype_t =
    std::ptr::null_mut::<cairo_sys::xcb_visualtype_t>();

static mut xcb_numlock_mask: u32 = 0;

pub static mut conn: *mut xcb_connection_t = std::ptr::null_mut::<xcb_connection_t>();

pub static mut root_screen: Option<i3_common::display::DisplayScreen> = None;
static mut font: Font = Font::empty();
static mut bold_font: Font = Font::empty();
static mut char_width: ::core::ffi::c_int = 0;
static mut win: u32 = 0;
static mut surface: surface_t = surface_t::empty();

pub static mut root: u32 = 0;
static mut xkb_keymap: *mut xkb_keymap = std::ptr::null_mut::<xkb_keymap>();
static mut xkb_base_event: u8 = 0;
static mut xkb_base_error: u8 = 0;
static mut tokens_FORCE_DISPLAY_URGENCY_HINT_MS: [cmdp_token; 2] = [
    token {
        name: c"'ms".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::ForceDisplayUrgencyHintMs,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_FLOATING_MAXIMUM_SIZE_HEIGHT: [cmdp_token; 1] = [token {
    name: c"number".as_ptr().cast_mut(),
    identifier: c"height".as_ptr().cast_mut(),
    next_state: ParserState::Call,
    extra: WizardTokenData {
        call_identifier: 1 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_FLOATING_MINIMUM_SIZE_HEIGHT: [cmdp_token; 1] = [token {
    name: c"number".as_ptr().cast_mut(),
    identifier: c"height".as_ptr().cast_mut(),
    next_state: ParserState::Call,
    extra: WizardTokenData {
        call_identifier: 2 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_BAR_STRIP_WORKSPACE_NUMBERS: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"value".as_ptr().cast_mut(),
    next_state: ParserState::Call,
    extra: WizardTokenData {
        call_identifier: 3 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_FLOATING_MAXIMUM_SIZE_WIDTH: [cmdp_token; 1] = [token {
    name: c"number".as_ptr().cast_mut(),
    identifier: c"width".as_ptr().cast_mut(),
    next_state: ParserState::FloatingMaximumSizeX,
    extra: WizardTokenData {
        call_identifier: 0 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_FLOATING_MINIMUM_SIZE_WIDTH: [cmdp_token; 1] = [token {
    name: c"number".as_ptr().cast_mut(),
    identifier: c"width".as_ptr().cast_mut(),
    next_state: ParserState::FloatingMinimumSizeX,
    extra: WizardTokenData {
        call_identifier: 0 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_BAR_BINDING_MODE_INDICATOR: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"value".as_ptr().cast_mut(),
    next_state: ParserState::Call,
    extra: WizardTokenData {
        call_identifier: 4 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_BAR_WORKSPACE_MIN_WIDTH_PX: [cmdp_token; 2] = [
    token {
        name: c"'px".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::BarWorkspaceMinWidthPx,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 5 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_FOCUS_ON_WINDOW_ACTIVATION: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"mode".as_ptr().cast_mut(),
    next_state: ParserState::Call,
    extra: WizardTokenData {
        call_identifier: 6 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_FORCE_DISPLAY_URGENCY_HINT: [cmdp_token; 1] = [token {
    name: c"number".as_ptr().cast_mut(),
    identifier: c"duration_ms".as_ptr().cast_mut(),
    next_state: ParserState::ForceDisplayUrgencyHintMs,
    extra: WizardTokenData {
        call_identifier: 0 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_TILING_DRAG_SWAP_MODIFIER: [cmdp_token; 10] = [
    token {
        name: c"'Mod1".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::TilingDragSwapModifier,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Mod2".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::TilingDragSwapModifier,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Mod3".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::TilingDragSwapModifier,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Mod4".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::TilingDragSwapModifier,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Mod5".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::TilingDragSwapModifier,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Shift".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::TilingDragSwapModifier,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Control".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::TilingDragSwapModifier,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Ctrl".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::TilingDragSwapModifier,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'+".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::TilingDragSwapModifier,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 7 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_BAR_STRIP_WORKSPACE_NAME: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"value".as_ptr().cast_mut(),
    next_state: ParserState::Call,
    extra: WizardTokenData {
        call_identifier: 8 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_CRITERION_FROM_STR_START: [cmdp_token; 3] = [
    token {
        name: c"'\"".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::CriterionFromStr,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'auto".as_ptr().cast_mut(),
        identifier: c"kind".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 9 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'user".as_ptr().cast_mut(),
        identifier: c"kind".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 10 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_DEFAULT_BORDER_PIXELS_PX: [cmdp_token; 2] = [
    token {
        name: c"'px".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::DefaultBorderPixelsPx,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 11 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_WORKSPACE_BACK_AND_FORTH: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"value".as_ptr().cast_mut(),
    next_state: ParserState::Call,
    extra: WizardTokenData {
        call_identifier: 12 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_ASSIGN_WORKSPACE_NUMBER: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"number".as_ptr().cast_mut(),
    next_state: ParserState::Call,
    extra: WizardTokenData {
        call_identifier: 13 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_BAR_WORKSPACE_MIN_WIDTH: [cmdp_token; 1] = [token {
    name: c"number".as_ptr().cast_mut(),
    identifier: c"width".as_ptr().cast_mut(),
    next_state: ParserState::BarWorkspaceMinWidthPx,
    extra: WizardTokenData {
        call_identifier: 0 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_FLOATING_MAXIMUM_SIZE_X: [cmdp_token; 1] = [token {
    name: c"'x".as_ptr().cast_mut(),
    identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
    next_state: ParserState::FloatingMaximumSizeHeight,
    extra: WizardTokenData {
        call_identifier: 0 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_FLOATING_MINIMUM_SIZE_X: [cmdp_token; 1] = [token {
    name: c"'x".as_ptr().cast_mut(),
    identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
    next_state: ParserState::FloatingMinimumSizeHeight,
    extra: WizardTokenData {
        call_identifier: 0 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_POPUP_DURING_FULLSCREEN: [cmdp_token; 4] = [
    token {
        name: c"'ignore".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 14 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'leave_fullscreen".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 15 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'all".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 16 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'smart".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 17 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_BAR_COLORS_IGNORE_LINE: [cmdp_token; 1] = [token {
    name: c"line".as_ptr().cast_mut(),
    identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
    next_state: ParserState::BarColors,
    extra: WizardTokenData {
        call_identifier: 0 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_CRITERION_FROM_STR_END: [cmdp_token; 1] = [token {
    name: c"'\"".as_ptr().cast_mut(),
    identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
    next_state: ParserState::Call,
    extra: WizardTokenData {
        call_identifier: 18 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_BAR_COLORS_BACKGROUND: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"background".as_ptr().cast_mut(),
    next_state: ParserState::BarColorsText,
    extra: WizardTokenData {
        call_identifier: 0 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_BAR_WORKSPACE_BUTTONS: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"value".as_ptr().cast_mut(),
    next_state: ParserState::Call,
    extra: WizardTokenData {
        call_identifier: 19 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_BAR_WORKSPACE_COMMAND: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"command".as_ptr().cast_mut(),
    next_state: ParserState::Call,
    extra: WizardTokenData {
        call_identifier: 20 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_DEFAULT_BORDER_PIXELS: [cmdp_token; 2] = [
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 21 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"number".as_ptr().cast_mut(),
        identifier: c"width".as_ptr().cast_mut(),
        next_state: ParserState::DefaultBorderPixelsPx,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_WORKSPACE_OUTPUT_WORD: [cmdp_token; 2] = [
    token {
        name: c"word".as_ptr().cast_mut(),
        identifier: c"output".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 22 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::Initial,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_BAR_SEPARATOR_SYMBOL: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"separator".as_ptr().cast_mut(),
    next_state: ParserState::Call,
    extra: WizardTokenData {
        call_identifier: 23 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_FORCE_FOCUS_WRAPPING: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"value".as_ptr().cast_mut(),
    next_state: ParserState::Call,
    extra: WizardTokenData {
        call_identifier: 24 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_BAR_BINDSYM_COMMAND: [cmdp_token; 2] = [
    token {
        name: c"'--release".as_ptr().cast_mut(),
        identifier: c"release".as_ptr().cast_mut(),
        next_state: ParserState::BarBindsymCommand,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"string".as_ptr().cast_mut(),
        identifier: c"command".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 25 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_BAR_TRAY_PADDING_PX: [cmdp_token; 2] = [
    token {
        name: c"'px".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::BarTrayPaddingPx,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 26 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_DEFAULT_ORIENTATION: [cmdp_token; 3] = [
    token {
        name: c"'horizontal".as_ptr().cast_mut(),
        identifier: c"orientation".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 27 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'vertical".as_ptr().cast_mut(),
        identifier: c"orientation".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 28 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'auto".as_ptr().cast_mut(),
        identifier: c"orientation".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 29 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_FOCUS_FOLLOWS_MOUSE: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"value".as_ptr().cast_mut(),
    next_state: ParserState::Call,
    extra: WizardTokenData {
        call_identifier: 30 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_BAR_PADDING_BOTTOM: [cmdp_token; 3] = [
    token {
        name: c"'px".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::BarPaddingBottom,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"number".as_ptr().cast_mut(),
        identifier: c"left".as_ptr().cast_mut(),
        next_state: ParserState::BarPaddingLeft,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 31 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_BAR_STATUS_COMMAND: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"command".as_ptr().cast_mut(),
    next_state: ParserState::Call,
    extra: WizardTokenData {
        call_identifier: 32 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_BAR_WHEEL_DOWN_CMD: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"command".as_ptr().cast_mut(),
    next_state: ParserState::Call,
    extra: WizardTokenData {
        call_identifier: 33 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_COLOR_CHILD_BORDER: [cmdp_token; 2] = [
    token {
        name: c"word".as_ptr().cast_mut(),
        identifier: c"child_border".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 34 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 35 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_CRITERION_FROM_STR: [cmdp_token; 2] = [
    token {
        name: c"'auto".as_ptr().cast_mut(),
        identifier: c"kind".as_ptr().cast_mut(),
        next_state: ParserState::CriterionFromStrEnd,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'user".as_ptr().cast_mut(),
        identifier: c"kind".as_ptr().cast_mut(),
        next_state: ParserState::CriterionFromStrEnd,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_FOR_WINDOW_COMMAND: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"command".as_ptr().cast_mut(),
    next_state: ParserState::Call,
    extra: WizardTokenData {
        call_identifier: 36 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_BAR_COLORS_BORDER: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"border".as_ptr().cast_mut(),
    next_state: ParserState::BarColorsBackground,
    extra: WizardTokenData {
        call_identifier: 0 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_BAR_COLORS_SINGLE: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"color".as_ptr().cast_mut(),
    next_state: ParserState::Call,
    extra: WizardTokenData {
        call_identifier: 37 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_BAR_PADDING_RIGHT: [cmdp_token; 3] = [
    token {
        name: c"'px".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::BarPaddingRight,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"number".as_ptr().cast_mut(),
        identifier: c"bottom".as_ptr().cast_mut(),
        next_state: ParserState::BarPaddingBottom,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 38 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_FLOATING_MODIFIER: [cmdp_token; 10] = [
    token {
        name: c"'Mod1".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::FloatingModifier,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Mod2".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::FloatingModifier,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Mod3".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::FloatingModifier,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Mod4".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::FloatingModifier,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Mod5".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::FloatingModifier,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Shift".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::FloatingModifier,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Control".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::FloatingModifier,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Ctrl".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::FloatingModifier,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'+".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::FloatingModifier,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 39 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_HIDE_EDGE_BORDERS: [cmdp_token; 12] = [
    token {
        name: c"'none".as_ptr().cast_mut(),
        identifier: c"hide_borders".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 40 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'vertical".as_ptr().cast_mut(),
        identifier: c"hide_borders".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 41 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'horizontal".as_ptr().cast_mut(),
        identifier: c"hide_borders".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 42 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'both".as_ptr().cast_mut(),
        identifier: c"hide_borders".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 43 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'smart_no_gaps".as_ptr().cast_mut(),
        identifier: c"hide_borders".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 44 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'smart".as_ptr().cast_mut(),
        identifier: c"hide_borders".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 45 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'1".as_ptr().cast_mut(),
        identifier: c"hide_borders".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 46 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'yes".as_ptr().cast_mut(),
        identifier: c"hide_borders".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 47 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'true".as_ptr().cast_mut(),
        identifier: c"hide_borders".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 48 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'on".as_ptr().cast_mut(),
        identifier: c"hide_borders".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 49 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'enable".as_ptr().cast_mut(),
        identifier: c"hide_borders".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 50 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'active".as_ptr().cast_mut(),
        identifier: c"hide_borders".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 51 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_WORKSPACE_COMMAND: [cmdp_token; 2] = [
    token {
        name: c"'output".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::WorkspaceOutputWord,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'gaps".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::Gaps,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_ASSIGN_WORKSPACE: [cmdp_token; 5] = [
    token {
        name: c"'→".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::AssignWorkspace,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'output".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::AssignOutput,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'workspace".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::AssignWorkspace,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'number".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::AssignWorkspaceNumber,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"string".as_ptr().cast_mut(),
        identifier: c"workspace".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 52 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_BAR_COLORS_BRACE: [cmdp_token; 2] = [
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::BarColorsBrace,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'{".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::BarColors,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_BAR_HIDDEN_STATE: [cmdp_token; 2] = [
    token {
        name: c"'hide".as_ptr().cast_mut(),
        identifier: c"hidden_state".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 53 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'show".as_ptr().cast_mut(),
        identifier: c"hidden_state".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 54 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_BAR_PADDING_LEFT: [cmdp_token; 2] = [
    token {
        name: c"'px".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::BarPaddingLeft,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 55 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_BAR_TRAY_PADDING: [cmdp_token; 1] = [token {
    name: c"number".as_ptr().cast_mut(),
    identifier: c"padding_px".as_ptr().cast_mut(),
    next_state: ParserState::BarTrayPaddingPx,
    extra: WizardTokenData {
        call_identifier: 0 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_BAR_WHEEL_UP_CMD: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"command".as_ptr().cast_mut(),
    next_state: ParserState::Call,
    extra: WizardTokenData {
        call_identifier: 56 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_COLOR_BACKGROUND: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"background".as_ptr().cast_mut(),
    next_state: ParserState::ColorText,
    extra: WizardTokenData {
        call_identifier: 0 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_IPC_KILL_TIMEOUT: [cmdp_token; 1] = [token {
    name: c"number".as_ptr().cast_mut(),
    identifier: c"timeout".as_ptr().cast_mut(),
    next_state: ParserState::Call,
    extra: WizardTokenData {
        call_identifier: 57 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_MODE_BINDCOMMAND: [cmdp_token; 5] = [
    token {
        name: c"'--release".as_ptr().cast_mut(),
        identifier: c"release".as_ptr().cast_mut(),
        next_state: ParserState::ModeBindcommand,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'--border".as_ptr().cast_mut(),
        identifier: c"border".as_ptr().cast_mut(),
        next_state: ParserState::ModeBindcommand,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'--whole-window".as_ptr().cast_mut(),
        identifier: c"whole_window".as_ptr().cast_mut(),
        next_state: ParserState::ModeBindcommand,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'--exclude-titlebar".as_ptr().cast_mut(),
        identifier: c"exclude_titlebar".as_ptr().cast_mut(),
        next_state: ParserState::ModeBindcommand,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"string".as_ptr().cast_mut(),
        identifier: c"command".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 58 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_MODE_IGNORE_LINE: [cmdp_token; 1] = [token {
    name: c"line".as_ptr().cast_mut(),
    identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
    next_state: ParserState::Mode,
    extra: WizardTokenData {
        call_identifier: 0 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_TILING_DRAG_MODE: [cmdp_token; 3] = [
    token {
        name: c"'modifier".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: ParserState::TilingDragMode,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'titlebar".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: ParserState::TilingDragMode,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 59 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_WORKSPACE_LAYOUT: [cmdp_token; 4] = [
    token {
        name: c"'default".as_ptr().cast_mut(),
        identifier: c"layout".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 60 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'stacking".as_ptr().cast_mut(),
        identifier: c"layout".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 61 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'stacked".as_ptr().cast_mut(),
        identifier: c"layout".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 62 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'tabbed".as_ptr().cast_mut(),
        identifier: c"layout".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 63 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_BAR_BAR_COMMAND: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"command".as_ptr().cast_mut(),
    next_state: ParserState::Call,
    extra: WizardTokenData {
        call_identifier: 64 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_BAR_COLORS_TEXT: [cmdp_token; 2] = [
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 65 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"word".as_ptr().cast_mut(),
        identifier: c"text".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 66 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_BAR_IGNORE_LINE: [cmdp_token; 1] = [token {
    name: c"line".as_ptr().cast_mut(),
    identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
    next_state: ParserState::Bar,
    extra: WizardTokenData {
        call_identifier: 0 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_BAR_PADDING_TOP: [cmdp_token; 3] = [
    token {
        name: c"'px".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::BarPaddingTop,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"number".as_ptr().cast_mut(),
        identifier: c"right_or_right_and_left".as_ptr().cast_mut(),
        next_state: ParserState::BarPaddingRight,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 67 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_BAR_SOCKET_PATH: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"path".as_ptr().cast_mut(),
    next_state: ParserState::Call,
    extra: WizardTokenData {
        call_identifier: 68 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_BAR_TRAY_OUTPUT: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"output".as_ptr().cast_mut(),
    next_state: ParserState::Call,
    extra: WizardTokenData {
        call_identifier: 69 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_COLOR_INDICATOR: [cmdp_token; 2] = [
    token {
        name: c"word".as_ptr().cast_mut(),
        identifier: c"indicator".as_ptr().cast_mut(),
        next_state: ParserState::ColorChildBorder,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 70 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_DISABLE_RANDR15: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"value".as_ptr().cast_mut(),
    next_state: ParserState::Call,
    extra: WizardTokenData {
        call_identifier: 71 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_GAPS_WITH_SCOPE: [cmdp_token; 1] = [token {
    name: c"number".as_ptr().cast_mut(),
    identifier: c"value".as_ptr().cast_mut(),
    next_state: ParserState::GapsEnd,
    extra: WizardTokenData {
        call_identifier: 0 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_CRITERION_FROM: [cmdp_token; 1] = [token {
    name: c"'=".as_ptr().cast_mut(),
    identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
    next_state: ParserState::CriterionFromStrStart,
    extra: WizardTokenData {
        call_identifier: 0 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_DEFAULT_BORDER: [cmdp_token; 4] = [
    token {
        name: c"'normal".as_ptr().cast_mut(),
        identifier: c"border".as_ptr().cast_mut(),
        next_state: ParserState::DefaultBorderPixels,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'pixel".as_ptr().cast_mut(),
        identifier: c"border".as_ptr().cast_mut(),
        next_state: ParserState::DefaultBorderPixels,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'1pixel".as_ptr().cast_mut(),
        identifier: c"border".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 72 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'none".as_ptr().cast_mut(),
        identifier: c"border".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 73 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_FOCUS_WRAPPING: [cmdp_token; 14] = [
    token {
        name: c"'1".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 74 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'yes".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 75 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'true".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 76 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'on".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 77 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'enable".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 78 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'active".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 79 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'0".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 80 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'no".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 81 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'false".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 82 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'off".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 83 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'disable".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 84 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'inactive".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 85 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'force".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 86 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'workspace".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 87 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_FORCE_XINERAMA: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"value".as_ptr().cast_mut(),
    next_state: ParserState::Call,
    extra: WizardTokenData {
        call_identifier: 88 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_ASSIGN_OUTPUT: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"output".as_ptr().cast_mut(),
    next_state: ParserState::Call,
    extra: WizardTokenData {
        call_identifier: 89 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_CRITERION_STR: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"cvalue".as_ptr().cast_mut(),
    next_state: ParserState::Call,
    extra: WizardTokenData {
        call_identifier: 90 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_MOUSE_WARPING: [cmdp_token; 2] = [
    token {
        name: c"'none".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 91 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'output".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 92 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_RESTART_STATE: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"path".as_ptr().cast_mut(),
    next_state: ParserState::Call,
    extra: WizardTokenData {
        call_identifier: 93 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_SMART_BORDERS: [cmdp_token; 7] = [
    token {
        name: c"'1".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 94 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'yes".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 95 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'true".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 96 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'on".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 97 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'enable".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 98 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'active".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 99 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'no_gaps".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 100 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_BAR_MODIFIER: [cmdp_token; 12] = [
    token {
        name: c"'off".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 101 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'none".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 102 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Mod1".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::BarModifier,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Mod2".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::BarModifier,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Mod3".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::BarModifier,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Mod4".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::BarModifier,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Mod5".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::BarModifier,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Shift".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::BarModifier,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Control".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::BarModifier,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Ctrl".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::BarModifier,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'+".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::BarModifier,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 103 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_BAR_POSITION: [cmdp_token; 2] = [
    token {
        name: c"'top".as_ptr().cast_mut(),
        identifier: c"position".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 104 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'bottom".as_ptr().cast_mut(),
        identifier: c"position".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 105 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_COLOR_BORDER: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"border".as_ptr().cast_mut(),
    next_state: ParserState::ColorBackground,
    extra: WizardTokenData {
        call_identifier: 0 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_COLOR_SINGLE: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"color".as_ptr().cast_mut(),
    next_state: ParserState::Call,
    extra: WizardTokenData {
        call_identifier: 106 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_FAKE_OUTPUTS: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"outputs".as_ptr().cast_mut(),
    next_state: ParserState::Call,
    extra: WizardTokenData {
        call_identifier: 107 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_MODE_BINDING: [cmdp_token; 20] = [
    token {
        name: c"'--release".as_ptr().cast_mut(),
        identifier: c"release".as_ptr().cast_mut(),
        next_state: ParserState::ModeBinding,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'--border".as_ptr().cast_mut(),
        identifier: c"border".as_ptr().cast_mut(),
        next_state: ParserState::ModeBinding,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'--whole-window".as_ptr().cast_mut(),
        identifier: c"whole_window".as_ptr().cast_mut(),
        next_state: ParserState::ModeBinding,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'--exclude-titlebar".as_ptr().cast_mut(),
        identifier: c"exclude_titlebar".as_ptr().cast_mut(),
        next_state: ParserState::ModeBinding,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Mod1".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::ModeBinding,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Mod2".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::ModeBinding,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Mod3".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::ModeBinding,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Mod4".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::ModeBinding,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Mod5".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::ModeBinding,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Shift".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::ModeBinding,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Control".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::ModeBinding,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Ctrl".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::ModeBinding,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Mode_switch".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::ModeBinding,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Group1".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::ModeBinding,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Group2".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::ModeBinding,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Group3".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::ModeBinding,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Group4".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::ModeBinding,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'$mod".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::ModeBinding,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'+".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::ModeBinding,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"word".as_ptr().cast_mut(),
        identifier: c"key".as_ptr().cast_mut(),
        next_state: ParserState::ModeBindcommand,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_NO_FOCUS_END: [cmdp_token; 1] = [token {
    name: c"end".as_ptr().cast_mut(),
    identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
    next_state: ParserState::Call,
    extra: WizardTokenData {
        call_identifier: 108 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_BAR_BINDSYM: [cmdp_token; 2] = [
    token {
        name: c"'--release".as_ptr().cast_mut(),
        identifier: c"release".as_ptr().cast_mut(),
        next_state: ParserState::BarBindsym,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"word".as_ptr().cast_mut(),
        identifier: c"button".as_ptr().cast_mut(),
        next_state: ParserState::BarBindsymCommand,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_BAR_PADDING: [cmdp_token; 1] = [token {
    name: c"number".as_ptr().cast_mut(),
    identifier: c"top_or_all".as_ptr().cast_mut(),
    next_state: ParserState::BarPaddingTop,
    extra: WizardTokenData {
        call_identifier: 0 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_BAR_VERBOSE: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"value".as_ptr().cast_mut(),
    next_state: ParserState::Call,
    extra: WizardTokenData {
        call_identifier: 109 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_BINDCOMMAND: [cmdp_token; 5] = [
    token {
        name: c"'--release".as_ptr().cast_mut(),
        identifier: c"release".as_ptr().cast_mut(),
        next_state: ParserState::Bindcommand,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'--border".as_ptr().cast_mut(),
        identifier: c"border".as_ptr().cast_mut(),
        next_state: ParserState::Bindcommand,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'--whole-window".as_ptr().cast_mut(),
        identifier: c"whole_window".as_ptr().cast_mut(),
        next_state: ParserState::Bindcommand,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'--exclude-titlebar".as_ptr().cast_mut(),
        identifier: c"exclude_titlebar".as_ptr().cast_mut(),
        next_state: ParserState::Bindcommand,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"string".as_ptr().cast_mut(),
        identifier: c"command".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 110 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_IGNORE_LINE: [cmdp_token; 1] = [token {
    name: c"line".as_ptr().cast_mut(),
    identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
    next_state: ParserState::Initial,
    extra: WizardTokenData {
        call_identifier: 0 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_TILING_DRAG: [cmdp_token; 9] = [
    token {
        name: c"'0".as_ptr().cast_mut(),
        identifier: c"off".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 111 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'no".as_ptr().cast_mut(),
        identifier: c"off".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 112 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'false".as_ptr().cast_mut(),
        identifier: c"off".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 113 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'off".as_ptr().cast_mut(),
        identifier: c"off".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 114 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'disable".as_ptr().cast_mut(),
        identifier: c"off".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 115 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'inactive".as_ptr().cast_mut(),
        identifier: c"off".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 116 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'swap_modifier".as_ptr().cast_mut(),
        identifier: c"swap_modifier".as_ptr().cast_mut(),
        next_state: ParserState::TilingDragSwapModifier,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'modifier".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: ParserState::TilingDragMode,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'titlebar".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: ParserState::TilingDragMode,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_TITLE_ALIGN: [cmdp_token; 3] = [
    token {
        name: c"'left".as_ptr().cast_mut(),
        identifier: c"alignment".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 117 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'center".as_ptr().cast_mut(),
        identifier: c"alignment".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 118 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'right".as_ptr().cast_mut(),
        identifier: c"alignment".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 119 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_BAR_COLORS: [cmdp_token; 15] = [
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::BarColors,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'#".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::BarColorsIgnoreLine,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'set".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::BarColorsIgnoreLine,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'background".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: ParserState::BarColorsSingle,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'statusline".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: ParserState::BarColorsSingle,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'separator".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: ParserState::BarColorsSingle,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'focused_background".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: ParserState::BarColorsSingle,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'focused_statusline".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: ParserState::BarColorsSingle,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'focused_separator".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: ParserState::BarColorsSingle,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'focused_workspace".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: ParserState::BarColorsBorder,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'active_workspace".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: ParserState::BarColorsBorder,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'inactive_workspace".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: ParserState::BarColorsBorder,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'urgent_workspace".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: ParserState::BarColorsBorder,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'binding_mode".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: ParserState::BarColorsBorder,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'}".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::Bar,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_BAR_HEIGHT: [cmdp_token; 1] = [token {
    name: c"number".as_ptr().cast_mut(),
    identifier: c"value".as_ptr().cast_mut(),
    next_state: ParserState::Call,
    extra: WizardTokenData {
        call_identifier: 120 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_BAR_OUTPUT: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"output".as_ptr().cast_mut(),
    next_state: ParserState::Call,
    extra: WizardTokenData {
        call_identifier: 121 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_COLOR_TEXT: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"text".as_ptr().cast_mut(),
    next_state: ParserState::ColorIndicator,
    extra: WizardTokenData {
        call_identifier: 0 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_FOR_WINDOW: [cmdp_token; 1] = [token {
    name: c"'[".as_ptr().cast_mut(),
    identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
    next_state: ParserState::Call,
    extra: WizardTokenData {
        call_identifier: 122 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_IPC_SOCKET: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"path".as_ptr().cast_mut(),
    next_state: ParserState::Call,
    extra: WizardTokenData {
        call_identifier: 123 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_SHOW_MARKS: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"value".as_ptr().cast_mut(),
    next_state: ParserState::Call,
    extra: WizardTokenData {
        call_identifier: 124 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_SMART_GAPS: [cmdp_token; 13] = [
    token {
        name: c"'1".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 125 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'yes".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 126 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'true".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 127 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'on".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 128 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'enable".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 129 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'active".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 130 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'0".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 131 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'no".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 132 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'false".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 133 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'off".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 134 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'disable".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 135 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'inactive".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 136 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'inverse_outer".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 137 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_CRITERION: [cmdp_token; 1] = [token {
    name: c"'=".as_ptr().cast_mut(),
    identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
    next_state: ParserState::CriterionStr,
    extra: WizardTokenData {
        call_identifier: 0 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_MODEBRACE: [cmdp_token; 2] = [
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::Modebrace,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'{".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::Mode,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_WORKSPACE: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"workspace".as_ptr().cast_mut(),
    next_state: ParserState::WorkspaceCommand,
    extra: WizardTokenData {
        call_identifier: 0 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_BARBRACE: [cmdp_token; 2] = [
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::Barbrace,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'{".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 138 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_BAR_FONT: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"font".as_ptr().cast_mut(),
    next_state: ParserState::Call,
    extra: WizardTokenData {
        call_identifier: 139 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_BAR_MODE: [cmdp_token; 3] = [
    token {
        name: c"'dock".as_ptr().cast_mut(),
        identifier: c"mode".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 140 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'hide".as_ptr().cast_mut(),
        identifier: c"mode".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 141 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'invisible".as_ptr().cast_mut(),
        identifier: c"mode".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 142 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_CRITERIA: [cmdp_token; 17] = [
    token {
        name: c"'class".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: ParserState::Criterion,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'instance".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: ParserState::Criterion,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'window_role".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: ParserState::Criterion,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'con_id".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: ParserState::Criterion,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'id".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: ParserState::Criterion,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'window_type".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: ParserState::Criterion,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'con_mark".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: ParserState::Criterion,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'title".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: ParserState::Criterion,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'urgent".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: ParserState::Criterion,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'workspace".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: ParserState::Criterion,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'machine".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: ParserState::Criterion,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'floating_from".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: ParserState::CriterionFrom,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'tiling_from".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: ParserState::CriterionFrom,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'tiling".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 143 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'floating".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 144 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'all".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 145 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"']".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 146 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_GAPS_END: [cmdp_token; 2] = [
    token {
        name: c"'px".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::GapsEnd,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 147 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_MODENAME: [cmdp_token; 2] = [
    token {
        name: c"'--pango_markup".as_ptr().cast_mut(),
        identifier: c"pango_markup".as_ptr().cast_mut(),
        next_state: ParserState::Modename,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"word".as_ptr().cast_mut(),
        identifier: c"modename".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 148 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_NO_FOCUS: [cmdp_token; 1] = [token {
    name: c"'[".as_ptr().cast_mut(),
    identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
    next_state: ParserState::Call,
    extra: WizardTokenData {
        call_identifier: 149 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_BINDING: [cmdp_token; 20] = [
    token {
        name: c"'--release".as_ptr().cast_mut(),
        identifier: c"release".as_ptr().cast_mut(),
        next_state: ParserState::Binding,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'--border".as_ptr().cast_mut(),
        identifier: c"border".as_ptr().cast_mut(),
        next_state: ParserState::Binding,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'--whole-window".as_ptr().cast_mut(),
        identifier: c"whole_window".as_ptr().cast_mut(),
        next_state: ParserState::Binding,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'--exclude-titlebar".as_ptr().cast_mut(),
        identifier: c"exclude_titlebar".as_ptr().cast_mut(),
        next_state: ParserState::Binding,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Mod1".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::Binding,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Mod2".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::Binding,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Mod3".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::Binding,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Mod4".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::Binding,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Mod5".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::Binding,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Shift".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::Binding,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Control".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::Binding,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Ctrl".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::Binding,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Mode_switch".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::Binding,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Group1".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::Binding,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Group2".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::Binding,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Group3".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::Binding,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Group4".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::Binding,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'$mod".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::Binding,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'+".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::Binding,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"word".as_ptr().cast_mut(),
        identifier: c"key".as_ptr().cast_mut(),
        next_state: ParserState::Bindcommand,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_INCLUDE: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"pattern".as_ptr().cast_mut(),
    next_state: ParserState::Call,
    extra: WizardTokenData {
        call_identifier: 150 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_INITIAL: [cmdp_token; 60] = [
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::Initial,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"error".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::Initial,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'#".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::IgnoreLine,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'set ".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::IgnoreLine,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'set\t".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::IgnoreLine,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'set_from_resource".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::IgnoreLine,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'include".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::Include,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'bindsym".as_ptr().cast_mut(),
        identifier: c"bindtype".as_ptr().cast_mut(),
        next_state: ParserState::Binding,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'bindcode".as_ptr().cast_mut(),
        identifier: c"bindtype".as_ptr().cast_mut(),
        next_state: ParserState::Binding,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'bind".as_ptr().cast_mut(),
        identifier: c"bindtype".as_ptr().cast_mut(),
        next_state: ParserState::Binding,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'bar".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::Barbrace,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'font".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::Font,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'mode".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::Modename,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'gaps".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::Gaps,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'smart_borders".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::SmartBorders,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'smart_gaps".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::SmartGaps,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'floating_minimum_size".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::FloatingMinimumSizeWidth,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'floating_maximum_size".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::FloatingMaximumSizeWidth,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'floating_modifier".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::FloatingModifier,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'default_orientation".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::DefaultOrientation,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'workspace_layout".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::WorkspaceLayout,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'default_border".as_ptr().cast_mut(),
        identifier: c"windowtype".as_ptr().cast_mut(),
        next_state: ParserState::DefaultBorder,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'new_window".as_ptr().cast_mut(),
        identifier: c"windowtype".as_ptr().cast_mut(),
        next_state: ParserState::DefaultBorder,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'default_floating_border".as_ptr().cast_mut(),
        identifier: c"windowtype".as_ptr().cast_mut(),
        next_state: ParserState::DefaultBorder,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'new_float".as_ptr().cast_mut(),
        identifier: c"windowtype".as_ptr().cast_mut(),
        next_state: ParserState::DefaultBorder,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'hide_edge_borders".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::HideEdgeBorders,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'for_window".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::ForWindow,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'assign".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::Assign,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'no_focus".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::NoFocus,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'focus_follows_mouse".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::FocusFollowsMouse,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'mouse_warping".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::MouseWarping,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'focus_wrapping".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::FocusWrapping,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'force_focus_wrapping".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::ForceFocusWrapping,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'force_xinerama".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::ForceXinerama,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'force-xinerama".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::ForceXinerama,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'disable_randr15".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::DisableRandr15,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'disable-randr15".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::DisableRandr15,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'workspace_auto_back_and_forth".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::WorkspaceBackAndForth,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'fake_outputs".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::FakeOutputs,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'fake-outputs".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::FakeOutputs,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'force_display_urgency_hint".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::ForceDisplayUrgencyHint,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'focus_on_window_activation".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::FocusOnWindowActivation,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'title_align".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::TitleAlign,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'show_marks".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::ShowMarks,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'workspace".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::Workspace,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'ipc_socket".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::IpcSocket,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'ipc-socket".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::IpcSocket,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'ipc_kill_timeout".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::IpcKillTimeout,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'restart_state".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::RestartState,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'popup_during_fullscreen".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::PopupDuringFullscreen,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'tiling_drag".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::TilingDrag,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'exec_always".as_ptr().cast_mut(),
        identifier: c"exectype".as_ptr().cast_mut(),
        next_state: ParserState::Exec,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'exec".as_ptr().cast_mut(),
        identifier: c"exectype".as_ptr().cast_mut(),
        next_state: ParserState::Exec,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'client.background".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: ParserState::ColorSingle,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'client.focused_inactive".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: ParserState::ColorBorder,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'client.focused_tab_title".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: ParserState::ColorBorder,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'client.focused".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: ParserState::ColorBorder,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'client.unfocused".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: ParserState::ColorBorder,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'client.urgent".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: ParserState::ColorBorder,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'client.placeholder".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: ParserState::ColorBorder,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_ASSIGN: [cmdp_token; 1] = [token {
    name: c"'[".as_ptr().cast_mut(),
    identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
    next_state: ParserState::Call,
    extra: WizardTokenData {
        call_identifier: 151 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_BAR_ID: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"bar_id".as_ptr().cast_mut(),
    next_state: ParserState::Call,
    extra: WizardTokenData {
        call_identifier: 152 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_EXEC: [cmdp_token; 2] = [
    token {
        name: c"'--no-startup-id".as_ptr().cast_mut(),
        identifier: c"no_startup_id".as_ptr().cast_mut(),
        next_state: ParserState::Exec,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"string".as_ptr().cast_mut(),
        identifier: c"command".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 153 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_FONT: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"font".as_ptr().cast_mut(),
    next_state: ParserState::Call,
    extra: WizardTokenData {
        call_identifier: 154 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_GAPS: [cmdp_token; 8] = [
    token {
        name: c"'inner".as_ptr().cast_mut(),
        identifier: c"scope".as_ptr().cast_mut(),
        next_state: ParserState::GapsWithScope,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'outer".as_ptr().cast_mut(),
        identifier: c"scope".as_ptr().cast_mut(),
        next_state: ParserState::GapsWithScope,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'horizontal".as_ptr().cast_mut(),
        identifier: c"scope".as_ptr().cast_mut(),
        next_state: ParserState::GapsWithScope,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'vertical".as_ptr().cast_mut(),
        identifier: c"scope".as_ptr().cast_mut(),
        next_state: ParserState::GapsWithScope,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'top".as_ptr().cast_mut(),
        identifier: c"scope".as_ptr().cast_mut(),
        next_state: ParserState::GapsWithScope,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'right".as_ptr().cast_mut(),
        identifier: c"scope".as_ptr().cast_mut(),
        next_state: ParserState::GapsWithScope,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'bottom".as_ptr().cast_mut(),
        identifier: c"scope".as_ptr().cast_mut(),
        next_state: ParserState::GapsWithScope,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'left".as_ptr().cast_mut(),
        identifier: c"scope".as_ptr().cast_mut(),
        next_state: ParserState::GapsWithScope,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_MODE: [cmdp_token; 8] = [
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::Mode,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"error".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::Mode,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'#".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::ModeIgnoreLine,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'set".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::ModeIgnoreLine,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'bindsym".as_ptr().cast_mut(),
        identifier: c"bindtype".as_ptr().cast_mut(),
        next_state: ParserState::ModeBinding,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'bindcode".as_ptr().cast_mut(),
        identifier: c"bindtype".as_ptr().cast_mut(),
        next_state: ParserState::ModeBinding,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'bind".as_ptr().cast_mut(),
        identifier: c"bindtype".as_ptr().cast_mut(),
        next_state: ParserState::ModeBinding,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'}".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::Initial,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_BAR: [cmdp_token; 31] = [
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::Bar,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"error".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::Bar,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'#".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::BarIgnoreLine,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'set".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::BarIgnoreLine,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'i3bar_command".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::BarBarCommand,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'status_command".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::BarStatusCommand,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'workspace_command".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::BarWorkspaceCommand,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'socket_path".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::BarSocketPath,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'mode".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::BarMode,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'hidden_state".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::BarHiddenState,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'id".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::BarId,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'modifier".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::BarModifier,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'wheel_up_cmd".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::BarWheelUpCmd,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'wheel_down_cmd".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::BarWheelDownCmd,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'bindsym".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::BarBindsym,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'position".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::BarPosition,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'output".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::BarOutput,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'tray_output".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::BarTrayOutput,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'tray_padding".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::BarTrayPadding,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'font".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::BarFont,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'separator_symbol".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::BarSeparatorSymbol,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'binding_mode_indicator".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::BarBindingModeIndicator,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'workspace_buttons".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::BarWorkspaceButtons,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'workspace_min_width".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::BarWorkspaceMinWidth,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'strip_workspace_numbers".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::BarStripWorkspaceNumbers,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'strip_workspace_name".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::BarStripWorkspaceName,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'verbose".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::BarVerbose,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'height".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::BarHeight,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'padding".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::BarPadding,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'colors".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::BarColorsBrace,
        extra: WizardTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'}".as_ptr().cast_mut(),
        identifier: std::ptr::null_mut::<::core::ffi::c_char>(),
        next_state: ParserState::Call,
        extra: WizardTokenData {
            call_identifier: 155 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens: [cmdp_token_ptr; 117] = [
    tokenptr {
        array: &raw const tokens_FORCE_DISPLAY_URGENCY_HINT_MS as *mut cmdp_token,
        n: 2 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_FLOATING_MAXIMUM_SIZE_HEIGHT as *mut cmdp_token,
        n: 1 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_FLOATING_MINIMUM_SIZE_HEIGHT as *mut cmdp_token,
        n: 1 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_BAR_STRIP_WORKSPACE_NUMBERS as *mut cmdp_token,
        n: 1 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_FLOATING_MAXIMUM_SIZE_WIDTH as *mut cmdp_token,
        n: 1 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_FLOATING_MINIMUM_SIZE_WIDTH as *mut cmdp_token,
        n: 1 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_BAR_BINDING_MODE_INDICATOR as *mut cmdp_token,
        n: 1 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_BAR_WORKSPACE_MIN_WIDTH_PX as *mut cmdp_token,
        n: 2 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_FOCUS_ON_WINDOW_ACTIVATION as *mut cmdp_token,
        n: 1 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_FORCE_DISPLAY_URGENCY_HINT as *mut cmdp_token,
        n: 1 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_TILING_DRAG_SWAP_MODIFIER as *mut cmdp_token,
        n: 10 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_BAR_STRIP_WORKSPACE_NAME as *mut cmdp_token,
        n: 1 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_CRITERION_FROM_STR_START as *mut cmdp_token,
        n: 3 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_DEFAULT_BORDER_PIXELS_PX as *mut cmdp_token,
        n: 2 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_WORKSPACE_BACK_AND_FORTH as *mut cmdp_token,
        n: 1 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_ASSIGN_WORKSPACE_NUMBER as *mut cmdp_token,
        n: 1 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_BAR_WORKSPACE_MIN_WIDTH as *mut cmdp_token,
        n: 1 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_FLOATING_MAXIMUM_SIZE_X as *mut cmdp_token,
        n: 1 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_FLOATING_MINIMUM_SIZE_X as *mut cmdp_token,
        n: 1 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_POPUP_DURING_FULLSCREEN as *mut cmdp_token,
        n: 4 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_BAR_COLORS_IGNORE_LINE as *mut cmdp_token,
        n: 1 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_CRITERION_FROM_STR_END as *mut cmdp_token,
        n: 1 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_BAR_COLORS_BACKGROUND as *mut cmdp_token,
        n: 1 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_BAR_WORKSPACE_BUTTONS as *mut cmdp_token,
        n: 1 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_BAR_WORKSPACE_COMMAND as *mut cmdp_token,
        n: 1 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_DEFAULT_BORDER_PIXELS as *mut cmdp_token,
        n: 2 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_WORKSPACE_OUTPUT_WORD as *mut cmdp_token,
        n: 2 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_BAR_SEPARATOR_SYMBOL as *mut cmdp_token,
        n: 1 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_FORCE_FOCUS_WRAPPING as *mut cmdp_token,
        n: 1 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_BAR_BINDSYM_COMMAND as *mut cmdp_token,
        n: 2 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_BAR_TRAY_PADDING_PX as *mut cmdp_token,
        n: 2 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_DEFAULT_ORIENTATION as *mut cmdp_token,
        n: 3 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_FOCUS_FOLLOWS_MOUSE as *mut cmdp_token,
        n: 1 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_BAR_PADDING_BOTTOM as *mut cmdp_token,
        n: 3 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_BAR_STATUS_COMMAND as *mut cmdp_token,
        n: 1 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_BAR_WHEEL_DOWN_CMD as *mut cmdp_token,
        n: 1 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_COLOR_CHILD_BORDER as *mut cmdp_token,
        n: 2 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_CRITERION_FROM_STR as *mut cmdp_token,
        n: 2 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_FOR_WINDOW_COMMAND as *mut cmdp_token,
        n: 1 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_BAR_COLORS_BORDER as *mut cmdp_token,
        n: 1 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_BAR_COLORS_SINGLE as *mut cmdp_token,
        n: 1 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_BAR_PADDING_RIGHT as *mut cmdp_token,
        n: 3 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_FLOATING_MODIFIER as *mut cmdp_token,
        n: 10 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_HIDE_EDGE_BORDERS as *mut cmdp_token,
        n: 12 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_WORKSPACE_COMMAND as *mut cmdp_token,
        n: 2 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_ASSIGN_WORKSPACE as *mut cmdp_token,
        n: 5 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_BAR_COLORS_BRACE as *mut cmdp_token,
        n: 2 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_BAR_HIDDEN_STATE as *mut cmdp_token,
        n: 2 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_BAR_PADDING_LEFT as *mut cmdp_token,
        n: 2 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_BAR_TRAY_PADDING as *mut cmdp_token,
        n: 1 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_BAR_WHEEL_UP_CMD as *mut cmdp_token,
        n: 1 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_COLOR_BACKGROUND as *mut cmdp_token,
        n: 1 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_IPC_KILL_TIMEOUT as *mut cmdp_token,
        n: 1 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_MODE_BINDCOMMAND as *mut cmdp_token,
        n: 5 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_MODE_IGNORE_LINE as *mut cmdp_token,
        n: 1 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_TILING_DRAG_MODE as *mut cmdp_token,
        n: 3 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_WORKSPACE_LAYOUT as *mut cmdp_token,
        n: 4 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_BAR_BAR_COMMAND as *mut cmdp_token,
        n: 1 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_BAR_COLORS_TEXT as *mut cmdp_token,
        n: 2 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_BAR_IGNORE_LINE as *mut cmdp_token,
        n: 1 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_BAR_PADDING_TOP as *mut cmdp_token,
        n: 3 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_BAR_SOCKET_PATH as *mut cmdp_token,
        n: 1 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_BAR_TRAY_OUTPUT as *mut cmdp_token,
        n: 1 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_COLOR_INDICATOR as *mut cmdp_token,
        n: 2 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_DISABLE_RANDR15 as *mut cmdp_token,
        n: 1 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_GAPS_WITH_SCOPE as *mut cmdp_token,
        n: 1 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_CRITERION_FROM as *mut cmdp_token,
        n: 1 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_DEFAULT_BORDER as *mut cmdp_token,
        n: 4 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_FOCUS_WRAPPING as *mut cmdp_token,
        n: 14 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_FORCE_XINERAMA as *mut cmdp_token,
        n: 1 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_ASSIGN_OUTPUT as *mut cmdp_token,
        n: 1 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_CRITERION_STR as *mut cmdp_token,
        n: 1 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_MOUSE_WARPING as *mut cmdp_token,
        n: 2 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_RESTART_STATE as *mut cmdp_token,
        n: 1 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_SMART_BORDERS as *mut cmdp_token,
        n: 7 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_BAR_MODIFIER as *mut cmdp_token,
        n: 12 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_BAR_POSITION as *mut cmdp_token,
        n: 2 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_COLOR_BORDER as *mut cmdp_token,
        n: 1 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_COLOR_SINGLE as *mut cmdp_token,
        n: 1 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_FAKE_OUTPUTS as *mut cmdp_token,
        n: 1 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_MODE_BINDING as *mut cmdp_token,
        n: 20 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_NO_FOCUS_END as *mut cmdp_token,
        n: 1 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_BAR_BINDSYM as *mut cmdp_token,
        n: 2 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_BAR_PADDING as *mut cmdp_token,
        n: 1 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_BAR_VERBOSE as *mut cmdp_token,
        n: 1 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_BINDCOMMAND as *mut cmdp_token,
        n: 5 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_IGNORE_LINE as *mut cmdp_token,
        n: 1 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_TILING_DRAG as *mut cmdp_token,
        n: 9 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_TITLE_ALIGN as *mut cmdp_token,
        n: 3 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_BAR_COLORS as *mut cmdp_token,
        n: 15 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_BAR_HEIGHT as *mut cmdp_token,
        n: 1 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_BAR_OUTPUT as *mut cmdp_token,
        n: 1 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_COLOR_TEXT as *mut cmdp_token,
        n: 1 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_FOR_WINDOW as *mut cmdp_token,
        n: 1 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_IPC_SOCKET as *mut cmdp_token,
        n: 1 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_SHOW_MARKS as *mut cmdp_token,
        n: 1 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_SMART_GAPS as *mut cmdp_token,
        n: 13 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_CRITERION as *mut cmdp_token,
        n: 1 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_MODEBRACE as *mut cmdp_token,
        n: 2 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_WORKSPACE as *mut cmdp_token,
        n: 1 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_BARBRACE as *mut cmdp_token,
        n: 2 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_BAR_FONT as *mut cmdp_token,
        n: 1 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_BAR_MODE as *mut cmdp_token,
        n: 3 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_CRITERIA as *mut cmdp_token,
        n: 17 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_GAPS_END as *mut cmdp_token,
        n: 2 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_MODENAME as *mut cmdp_token,
        n: 2 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_NO_FOCUS as *mut cmdp_token,
        n: 1 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_BINDING as *mut cmdp_token,
        n: 20 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_INCLUDE as *mut cmdp_token,
        n: 1 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_INITIAL as *mut cmdp_token,
        n: 60 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_ASSIGN as *mut cmdp_token,
        n: 1 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_BAR_ID as *mut cmdp_token,
        n: 1 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_EXEC as *mut cmdp_token,
        n: 2 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_FONT as *mut cmdp_token,
        n: 1 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_GAPS as *mut cmdp_token,
        n: 8 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_MODE as *mut cmdp_token,
        n: 8 as ::core::ffi::c_int,
    },
    tokenptr {
        array: &raw const tokens_BAR as *mut cmdp_token,
        n: 31 as ::core::ffi::c_int,
    },
];
#[derive(Clone)]
struct ParserToken {
    name: Vec<u8>,
    identifier: Option<Vec<u8>>,
    next: ParserState,
}

// Generated tables are immutable in operation, despite their legacy raw-pointer
// representation. Only this adapter touches their pointers; parser state/captures
// and input positions are local owned Rust values.
unsafe fn parser_tokens(state: ParserState) -> Vec<ParserToken> {
    assert!((state as usize) < 117);
    let entry = (&raw const tokens)
        .cast::<tokenptr>()
        .add(state as usize)
        .read();
    std::slice::from_raw_parts(entry.array, entry.n as usize)
        .iter()
        .map(|token| ParserToken {
            name: CStr::from_ptr(token.name).to_bytes().to_vec(),
            identifier: (!token.identifier.is_null())
                .then(|| CStr::from_ptr(token.identifier).to_bytes().to_vec()),
            next: token.next_state,
        })
        .collect()
}

#[derive(Debug)]
enum CapturedValue {
    String(Vec<u8>),
    Number(libc::c_long),
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
fn decimal_prefix(input: &[u8]) -> Option<(libc::c_long, usize)> {
    let string = CString::new(input).ok()?;
    let mut end = std::ptr::null_mut();
    // SAFETY: the temporary C string is live, and end is a writable pointer.
    let number = unsafe {
        *libc::__errno_location() = 0;
        let number = libc::strtol(string.as_ptr(), &mut end, 10);
        let errno = *libc::__errno_location();
        if errno == libc::ERANGE && (number == libc::c_long::MIN || number == libc::c_long::MAX)
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
    input: &CStr,
    mut table: impl FnMut(ParserState) -> Vec<ParserToken>,
) -> Option<BindingFields> {
    let input = input.to_bytes();
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

fn format_binding(binding: &BindingFields, keysym: &CStr) -> CString {
    let mut bytes = b"bindsym ".to_vec();
    if let Some(modifiers) = &binding.modifiers {
        bytes.extend(
            modifiers
                .iter()
                .map(|&byte| if byte == b',' { b'+' } else { byte }),
        );
        bytes.push(b'+');
    }
    bytes.extend_from_slice(keysym.to_bytes());
    bytes.push(b' ');
    if let Some(release) = &binding.release {
        bytes.extend(release);
    }
    bytes.extend(&binding.command);
    bytes.push(b'\n');
    CString::new(bytes).expect("binding fields originated from C strings")
}

/// Native key symbols borrow the initialized connection's setup/key table.
/// Requires sole event-thread access to those resources for this query.
fn keysym_used_on_other_key(keysyms: &KeySymbols, sym: libc::c_ulong, except_keycode: u8) -> bool {
    let setup = keysyms.connection().get_setup();
    let (minimum, maximum) = (setup.min_keycode(), setup.max_keycode());
    // Preserve the native zero-start sentinel and level order.
    minimum != 0
        && (minimum..=maximum)
            .filter(|&code| code != except_keycode)
            .any(|code| (0..4).any(|level| keysyms.keysym(code, level) as libc::c_ulong == sym))
}
// The parser and formatting own their data. Only keysym lookup and malloc ABI
// handoff require the initialized event-thread XKB/key-symbol native resources.
unsafe fn rewrite_binding(input: &CStr, keysyms: &KeySymbols) -> Option<CString> {
    let binding = parse_binding(input, |state| parser_tokens(state))?;
    let modifiers = binding.modifiers.as_deref();
    let key = CString::new(binding.key.clone()).expect("key has no NUL");
    let keycode = atoi(key.as_ptr());
    let mut level: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if modifiers.is_some_and(|bytes| bytes.windows(5).any(|part| part == b"Shift")) {
        level = 1 as ::core::ffi::c_int;
        let mut syms: *const xkb_keysym_t = ::core::ptr::null::<xkb_keysym_t>();
        let num: ::core::ffi::c_int = xkb_keymap_key_get_syms_by_level(
            xkb_keymap,
            keycode as xkb_keycode_t,
            0 as xkb_layout_index_t,
            0 as xkb_level_index_t,
            &raw mut syms,
        );
        if num == 0 as ::core::ffi::c_int {
            errx(
                1 as ::core::ffi::c_int,
                c"xkb_keymap_key_get_syms_by_level returned no symbols for keycode %d".as_ptr(),
                keycode,
            );
        }
        if !keysym_used_on_other_key(
            keysyms,
            *syms.offset(0_isize) as libc::c_ulong,
            keycode as u8,
        ) {
            level = 0 as ::core::ffi::c_int;
        }
    }
    let mut syms_0: *const xkb_keysym_t = ::core::ptr::null::<xkb_keysym_t>();
    let num_0: ::core::ffi::c_int = xkb_keymap_key_get_syms_by_level(
        xkb_keymap,
        keycode as xkb_keycode_t,
        0 as xkb_layout_index_t,
        level as xkb_level_index_t,
        &raw mut syms_0,
    );
    if num_0 == 0 as ::core::ffi::c_int {
        errx(
            1 as ::core::ffi::c_int,
            c"xkb_keymap_key_get_syms_by_level returned no symbols for keycode %d".as_ptr(),
            keycode,
        );
    }
    if num_0 > 1 as ::core::ffi::c_int {
        printf(
                c"xkb_keymap_key_get_syms_by_level (keycode = %d) returned %d symbolsinstead of 1, using only the first one.\n".as_ptr(),
                keycode,
                num_0,
            );
    }
    let mut str: [::core::ffi::c_char; 4096] = [0; 4096];
    if xkb_keysym_get_name(
        *syms_0.offset(0_isize),
        &raw mut str as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as usize,
    ) == -(1 as ::core::ffi::c_int)
    {
        errx(
            libc::EXIT_FAILURE,
            c"xkb_keysym_get_name(%u) failed".as_ptr(),
            *syms_0.offset(0_isize),
        );
    }
    Some(format_binding(&binding, CStr::from_ptr(str.as_ptr())))
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
    config_path: CString,
    socket: Option<CString>,
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
    name: &CStr,
) -> x::Atom {
    let Ok(reply) = connection.wait_for_reply(cookie) else {
        errx(
            libc::EXIT_FAILURE,
            c"Could not get atom %s".as_ptr(),
            name.as_ptr(),
        );
    };
    reply.atom()
}

struct WizardAtoms {
    name: x::Atom,
    utf8: x::Atom,
}

unsafe fn txt(
    text_font: &Font,
    col: ::core::ffi::c_int,
    row: ::core::ffi::c_int,
    text: *mut ::core::ffi::c_char,
    fg: color_t,
    bg: color_t,
) {
    let x: ::core::ffi::c_int =
        (col - 1 as ::core::ffi::c_int) * char_width + logical_px(4 as ::core::ffi::c_int);
    let y: ::core::ffi::c_int =
        (row - 1 as ::core::ffi::c_int) * font.height + logical_px(4 as ::core::ffi::c_int);
    let string = i3_common::string::I3String::from_bytes(CStr::from_ptr(text).to_bytes());
    draw_util_text(
        text_font,
        &string,
        &raw mut surface,
        fg,
        bg,
        x,
        y,
        logical_px(300 as ::core::ffi::c_int) - x - logical_px(4 as ::core::ffi::c_int),
    );
}
unsafe fn handle_expose(ui: &WizardUi, connection: &xcb::Connection) -> ::core::ffi::c_int {
    let black: color_t = draw_util_hex_to_color(root_screen.as_ref(), "#000000") as color_t;
    let white: color_t = draw_util_hex_to_color(root_screen.as_ref(), "#FFFFFF") as color_t;
    let green: color_t = draw_util_hex_to_color(root_screen.as_ref(), "#00FF00") as color_t;
    let red: color_t = draw_util_hex_to_color(root_screen.as_ref(), "#FF0000") as color_t;
    draw_util_clear_surface(&raw mut surface, black);

    if ui.step == WizardStep::Welcome {
        txt(
            &font,
            2 as ::core::ffi::c_int,
            2 as ::core::ffi::c_int,
            c"You have not configured i3 yet.".as_ptr().cast_mut(),
            white,
            black,
        );
        txt(
            &font,
            2 as ::core::ffi::c_int,
            3 as ::core::ffi::c_int,
            c"Do you want me to generate a config at"
                .as_ptr()
                .cast_mut(),
            white,
            black,
        );
        let mut message = ui.config_path.to_bytes().to_vec();
        message.push(b'?');
        let msg = CString::new(message).expect("path contains no NUL");
        txt(
            &font,
            2 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            msg.as_ptr().cast_mut(),
            white,
            black,
        );

        txt(
            &font,
            13 as ::core::ffi::c_int,
            6 as ::core::ffi::c_int,
            c"Yes, generate the config".as_ptr().cast_mut(),
            white,
            black,
        );
        txt(
            &font,
            13 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int,
            c"No, I will use the defaults".as_ptr().cast_mut(),
            white,
            black,
        );
        txt(
            &font,
            4 as ::core::ffi::c_int,
            6 as ::core::ffi::c_int,
            c"<Enter>".as_ptr().cast_mut(),
            green,
            black,
        );
        txt(
            &font,
            5 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int,
            c"<ESC>".as_ptr().cast_mut(),
            red,
            black,
        );
    }
    if ui.step == WizardStep::Generate {
        txt(
            &font,
            2 as ::core::ffi::c_int,
            2 as ::core::ffi::c_int,
            c"Please choose either:".as_ptr().cast_mut(),
            white,
            black,
        );
        txt(
            &font,
            13 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            c"Win as default modifier".as_ptr().cast_mut(),
            white,
            black,
        );
        txt(
            &font,
            13 as ::core::ffi::c_int,
            5 as ::core::ffi::c_int,
            c"Alt as default modifier".as_ptr().cast_mut(),
            white,
            black,
        );
        txt(
            &font,
            2 as ::core::ffi::c_int,
            7 as ::core::ffi::c_int,
            c"Afterwards, press".as_ptr().cast_mut(),
            white,
            black,
        );
        txt(
            &font,
            13 as ::core::ffi::c_int,
            9 as ::core::ffi::c_int,
            c"to write the config".as_ptr().cast_mut(),
            white,
            black,
        );
        txt(
            &font,
            13 as ::core::ffi::c_int,
            10 as ::core::ffi::c_int,
            c"to abort".as_ptr().cast_mut(),
            white,
            black,
        );
        if ui.modifier == WizardModifier::Win {
            txt(
                &font,
                5 as ::core::ffi::c_int,
                5 as ::core::ffi::c_int,
                c"<Alt>".as_ptr().cast_mut(),
                white,
                black,
            );
        } else {
            txt(
                &font,
                5 as ::core::ffi::c_int,
                4 as ::core::ffi::c_int,
                c"<Win>".as_ptr().cast_mut(),
                white,
                black,
            );
        }

        if ui.modifier == WizardModifier::Win {
            txt(
                &bold_font,
                2 as ::core::ffi::c_int,
                4 as ::core::ffi::c_int,
                c"-> <Win>".as_ptr().cast_mut(),
                white,
                black,
            );
        } else {
            txt(
                &bold_font,
                2 as ::core::ffi::c_int,
                5 as ::core::ffi::c_int,
                c"-> <Alt>".as_ptr().cast_mut(),
                white,
                black,
            );
        }

        txt(
            &font,
            4 as ::core::ffi::c_int,
            9 as ::core::ffi::c_int,
            c"<Enter>".as_ptr().cast_mut(),
            green,
            black,
        );
        txt(
            &font,
            5 as ::core::ffi::c_int,
            10 as ::core::ffi::c_int,
            c"<ESC>".as_ptr().cast_mut(),
            red,
            black,
        );
    }
    let _ = connection.flush();
    1 as ::core::ffi::c_int
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
    state_filtered = (state_filtered as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as u16;
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
const CONFIG_TEMPLATE: &CStr = c"/usr/local/etc/i3/config.keycodes";
const CONFIG_HEADER: &[u8] = b"# This file has been auto-generated by i3-config-wizard(1).\n# It will not be overwritten, so edit it as you like.\n#\n# Should you change your keyboard layout some time, delete\n# this file and re-run i3-config-wizard(1).\n#\n";

struct TemplateLines {
    head: bool,
}
impl TemplateLines {
    fn rewrite(
        &mut self,
        line: &[u8],
        modifier: WizardModifier,
        mut binding: impl FnMut(&CStr) -> Option<CString>,
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
            let input = CString::new(rest).expect("first-NUL projection contains no NUL");
            output.extend_from_slice(binding(&input).ok_or(())?.to_bytes());
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
    printf(c"creating \"%s\"...\n".as_ptr(), ui.config_path.as_ptr());
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
    let input = File::open(std::ffi::OsStr::from_bytes(CONFIG_TEMPLATE.to_bytes())).unwrap_or_else(
        |error| {
            *__errno_location() = error.raw_os_error().unwrap_or(libc::EIO);
            err(
                1,
                c"Could not open input file \"%s\"".as_ptr(),
                CONFIG_TEMPLATE.as_ptr(),
            );
        },
    );
    let mut output = File::create(std::ffi::OsStr::from_bytes(ui.config_path.to_bytes()))
        .unwrap_or_else(|error| {
            *__errno_location() = error.raw_os_error().unwrap_or(libc::EIO);
            err(
                1,
                c"Could not open output config file \"%s\"".as_ptr(),
                ui.config_path.as_ptr(),
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
    let path = ui
        .socket
        .as_deref()
        .map(|path| std::path::Path::new(std::ffi::OsStr::from_bytes(path.to_bytes())));
    let mut socket = connect_or_exit(path);
    let _ = send_message_retry_would_block(&mut socket, 0, b"reload");
}
#[derive(Clone, Copy)]
enum ModifierKey {
    Alt,
    Win,
}
#[derive(Default)]
struct WizardSettings {
    socket: Option<std::ffi::CString>,
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
unsafe fn run_window(settings: WizardSettings) -> ::core::ffi::c_int {
    let mut xdg_config_home: *mut ::core::ffi::c_char = std::ptr::null_mut();
    let pattern = c"pango:monospace 8".as_ptr();
    let patternbold = c"pango:monospace bold 8".as_ptr();
    let headless_run = settings.modifier.is_some();
    let selected_modifier = match settings.modifier {
        Some(ModifierKey::Alt) => WizardModifier::Alt,
        _ => WizardModifier::Win,
    };
    let path: *mut ::core::ffi::c_char =
        get_config_path(::core::ptr::null::<::core::ffi::c_char>(), false);
    if !path.is_null() {
        printf(
            c"The config file \"%s\" already exists. Exiting.\n".as_ptr(),
            path,
        );
        free(path as *mut ::core::ffi::c_void);
        return 0 as ::core::ffi::c_int;
    }
    xdg_config_home = getenv(c"XDG_CONFIG_HOME".as_ptr());
    if xdg_config_home.is_null() {
        xdg_config_home = c"~/.config".as_ptr().cast_mut();
    }
    let base = resolve_cstring(CStr::from_ptr(xdg_config_home));
    let mut directory = base.to_bytes().to_vec();
    directory.extend_from_slice(b"/i3");
    let config_dir = CString::new(directory).expect("base path has no NUL");
    let mut path = config_dir.to_bytes().to_vec();
    path.extend_from_slice(b"/config");
    let config_path = CString::new(path).expect("base path has no NUL");
    let mut stbuf: stat = ::core::mem::zeroed();
    if stat(config_dir.as_ptr(), &raw mut stbuf) != 0 {
        let directory = std::path::Path::new(std::ffi::OsStr::from_bytes(config_dir.to_bytes()));
        if let Err(error) = create_dir_all(directory, DEFAULT_DIR_MODE) {
            let error = report_create_error(directory, error);
            *__errno_location() = error.raw_os_error().unwrap_or(libc::EIO);
            err(
                libc::EXIT_FAILURE,
                c"mkdirp(%s) failed".as_ptr(),
                config_dir.as_ptr(),
            );
        }
    }
    let mut fd: ::core::ffi::c_int = 0;
    fd = open(
        config_path.as_ptr(),
        libc::O_CREAT | libc::O_RDWR,
        0o644 as ::core::ffi::c_int,
    );
    if fd == -(1 as ::core::ffi::c_int) {
        printf(
            c"Cannot open file \"%s\" for writing: %s. Exiting.\n".as_ptr(),
            config_path.as_ptr(),
            strerror(*__errno_location()),
        );
        return 0 as ::core::ffi::c_int;
    }
    close(fd);
    unlink(config_path.as_ptr());
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
    ) != 1 as ::core::ffi::c_int
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
        return 0 as ::core::ffi::c_int;
    }
    let intern = |name: &CStr| {
        connection.send_request(&x::InternAtom {
            only_if_exists: false,
            name: &name.to_bytes()[..name.to_bytes().len() as u16 as usize],
        })
    };
    let name_cookie = intern(c"_NET_WM_NAME");
    let window_type_cookie = intern(c"_NET_WM_WINDOW_TYPE");
    let dialog_cookie = intern(c"_NET_WM_WINDOW_TYPE_DIALOG");
    let atom_cookie = intern(c"ATOM");
    let cardinal_cookie = intern(c"CARDINAL");
    let utf8_cookie = intern(c"UTF8_STRING");
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
    font = load_font(
        &font_context,
        CStr::from_ptr(pattern).to_bytes().to_vec(),
        true,
    );
    bold_font = load_font(
        &font_context,
        CStr::from_ptr(patternbold).to_bytes().to_vec(),
        true,
    );
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
    let name = atom_reply(&connection, name_cookie, c"_NET_WM_NAME");
    let window_type = atom_reply(&connection, window_type_cookie, c"_NET_WM_WINDOW_TYPE");
    let dialog = atom_reply(&connection, dialog_cookie, c"_NET_WM_WINDOW_TYPE_DIALOG");
    let atom = atom_reply(&connection, atom_cookie, c"ATOM");
    let _cardinal = atom_reply(&connection, cardinal_cookie, c"CARDINAL");
    let utf8 = atom_reply(&connection, utf8_cookie, c"UTF8_STRING");
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
        logical_px(300 as ::core::ffi::c_int),
        15 as ::core::ffi::c_int * font.height + logical_px(4 as ::core::ffi::c_int),
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
                b's' => {
                    settings.socket = Some(
                        std::ffi::CString::new(value.unwrap().as_bytes())
                            .expect("OS arguments have no NUL bytes"),
                    )
                }
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
use i3_common::resolve_tilde::resolve_cstring;

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

    fn parse(input: &CStr) -> Option<BindingFields> {
        // Generated grammar is immutable and this local parser never touches X.
        parse_binding(input, |state| unsafe { parser_tokens(state) })
    }

    #[test]
    fn binding_parser_keeps_modifiers_commands_release_and_raw_bytes() {
        let binding = parse(c"bindcode $mod+Shift+36 exec terminal\n").unwrap();
        assert_eq!(binding.modifiers.as_deref(), Some(&b"$mod,Shift"[..]));
        assert_eq!(binding.key, b"36");
        assert_eq!(binding.command, b"exec terminal");
        assert_eq!(
            format_binding(&binding, c"Return").to_bytes(),
            b"bindsym $mod+Shift+Return exec terminal\n"
        );
        let binding = parse(c"BiNdCoDe --release 36 exec raw\xff\n").unwrap();
        assert_eq!(binding.release.as_deref(), Some(&b"--release"[..]));
        // Keep the historical no-space concatenation of release and command.
        assert_eq!(
            format_binding(&binding, c"Return").to_bytes(),
            b"bindsym Return --releaseexec raw\xff\n"
        );
        let quoted = parse(c"bindcode \"36\" \"exec \\\"quoted\\\"\"\n").unwrap();
        assert_eq!(quoted.key, b"36");
        assert_eq!(quoted.command, b"exec \"quoted\"");
    }

    #[test]
    fn malformed_lines_are_bounded_and_captures_do_not_leak_between_lines() {
        for line in [
            c"",
            c"bindcode",
            c"bindcode 36",
            c"bindcode 36 \"\"",
            c"not_a_binding 36 exec x",
        ] {
            assert!(parse(line).is_none(), "{line:?}");
        }
        let first = parse(c"bindcode Mod4+36 exec x").unwrap();
        let second = parse(c"bindcode 37 exec y").unwrap();
        assert!(first.modifiers.is_some());
        assert!(second.modifiers.is_none());
        let mut input = b"bindcode ".to_vec();
        for _ in 0..100 {
            input.extend_from_slice(b"Shift+");
        }
        input.extend_from_slice(b"36 exec done");
        let parsed = parse(&CString::new(input).unwrap()).unwrap();
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
                |input| Some(format_binding(&parse(input).unwrap(), c"Return")),
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
