use i3_common::draw_util::draw_util_clear_surface;
use i3_common::draw_util::draw_util_hex_to_color;
use i3_common::draw_util::draw_util_surface_free;
use i3_common::draw_util::draw_util_surface_init;
use i3_common::draw_util::draw_util_text;
use i3_common::font::load_font;
use i3_common::font::predict_text_width;
use i3_common::font::set_font;
use i3_common::mkdirp::mkdirp;
use std::ffi::{CStr, CString};
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
use i3_common::i333_va_list;

pub use i3_common::font::C2RustUnnamed;
pub use i3_common::font::C2RustUnnamed_0;
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
    pub extra: C2RustUnnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
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

pub static mut A__NET_WM_WINDOW_TYPE: xcb_atom_t = 0;

pub static mut A_UTF8_STRING: xcb_atom_t = 0;

pub static mut A_ATOM: xcb_atom_t = 0;

pub static mut A_CARDINAL: xcb_atom_t = 0;

pub static mut A__NET_WM_NAME: xcb_atom_t = 0;

pub static mut A__NET_WM_WINDOW_TYPE_DIALOG: xcb_atom_t = 0;

pub static mut visual_type: *mut xcb_visualtype_t =
    ::core::ptr::null::<xcb_visualtype_t>() as *mut xcb_visualtype_t;

static mut xcb_numlock_mask: u32 = 0;

pub static mut conn: *mut xcb_connection_t =
    ::core::ptr::null::<xcb_connection_t>() as *mut xcb_connection_t;
static mut keysyms: *mut xcb_key_symbols_t =
    ::core::ptr::null::<xcb_key_symbols_t>() as *mut xcb_key_symbols_t;

pub static mut root_screen: *mut xcb_screen_t =
    ::core::ptr::null::<xcb_screen_t>() as *mut xcb_screen_t;
static mut font: Font = Font {
    type_0: i3_common::font::FontType::None,
    height: 0,
    pattern: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    specific: C2RustUnnamed {
        xcb: C2RustUnnamed_0 {
            id: 0,
            info: ::core::ptr::null::<xcb_query_font_reply_t>() as *mut xcb_query_font_reply_t,
            table: ::core::ptr::null::<xcb_charinfo_t>() as *mut xcb_charinfo_t,
        },
    },
};
static mut bold_font: Font = Font {
    type_0: i3_common::font::FontType::None,
    height: 0,
    pattern: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    specific: C2RustUnnamed {
        xcb: C2RustUnnamed_0 {
            id: 0,
            info: ::core::ptr::null::<xcb_query_font_reply_t>() as *mut xcb_query_font_reply_t,
            table: ::core::ptr::null::<xcb_charinfo_t>() as *mut xcb_charinfo_t,
        },
    },
};
static mut char_width: ::core::ffi::c_int = 0;
static mut win: xcb_window_t = 0;
static mut surface: surface_t = surface_t {
    id: 0,
    gc: 0,
    owns_gc: false,
    width: 0,
    height: 0,
    surface: ::core::ptr::null::<cairo_surface_t>() as *mut cairo_surface_t,
    cr: ::core::ptr::null::<cairo_t>() as *mut cairo_t,
};
static mut symbols: *mut xcb_key_symbols_t =
    ::core::ptr::null::<xcb_key_symbols_t>() as *mut xcb_key_symbols_t;

pub static mut root: xcb_window_t = 0;
static mut xkb_keymap: *mut xkb_keymap = ::core::ptr::null::<xkb_keymap>() as *mut xkb_keymap;
static mut xkb_base_event: u8 = 0;
static mut xkb_base_error: u8 = 0;
static mut tokens_FORCE_DISPLAY_URGENCY_HINT_MS: [cmdp_token; 2] = [
    token {
        name: c"'ms".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::ForceDisplayUrgencyHintMs,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_FLOATING_MAXIMUM_SIZE_HEIGHT: [cmdp_token; 1] = [token {
    name: c"number".as_ptr().cast_mut(),
    identifier: c"height".as_ptr().cast_mut(),
    next_state: ParserState::Call,
    extra: C2RustUnnamed_5 {
        call_identifier: 1 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_FLOATING_MINIMUM_SIZE_HEIGHT: [cmdp_token; 1] = [token {
    name: c"number".as_ptr().cast_mut(),
    identifier: c"height".as_ptr().cast_mut(),
    next_state: ParserState::Call,
    extra: C2RustUnnamed_5 {
        call_identifier: 2 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_BAR_STRIP_WORKSPACE_NUMBERS: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"value".as_ptr().cast_mut(),
    next_state: ParserState::Call,
    extra: C2RustUnnamed_5 {
        call_identifier: 3 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_FLOATING_MAXIMUM_SIZE_WIDTH: [cmdp_token; 1] = [token {
    name: c"number".as_ptr().cast_mut(),
    identifier: c"width".as_ptr().cast_mut(),
    next_state: ParserState::FloatingMaximumSizeX,
    extra: C2RustUnnamed_5 {
        call_identifier: 0 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_FLOATING_MINIMUM_SIZE_WIDTH: [cmdp_token; 1] = [token {
    name: c"number".as_ptr().cast_mut(),
    identifier: c"width".as_ptr().cast_mut(),
    next_state: ParserState::FloatingMinimumSizeX,
    extra: C2RustUnnamed_5 {
        call_identifier: 0 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_BAR_BINDING_MODE_INDICATOR: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"value".as_ptr().cast_mut(),
    next_state: ParserState::Call,
    extra: C2RustUnnamed_5 {
        call_identifier: 4 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_BAR_WORKSPACE_MIN_WIDTH_PX: [cmdp_token; 2] = [
    token {
        name: c"'px".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::BarWorkspaceMinWidthPx,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 5 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_FOCUS_ON_WINDOW_ACTIVATION: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"mode".as_ptr().cast_mut(),
    next_state: ParserState::Call,
    extra: C2RustUnnamed_5 {
        call_identifier: 6 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_FORCE_DISPLAY_URGENCY_HINT: [cmdp_token; 1] = [token {
    name: c"number".as_ptr().cast_mut(),
    identifier: c"duration_ms".as_ptr().cast_mut(),
    next_state: ParserState::ForceDisplayUrgencyHintMs,
    extra: C2RustUnnamed_5 {
        call_identifier: 0 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_TILING_DRAG_SWAP_MODIFIER: [cmdp_token; 10] = [
    token {
        name: c"'Mod1".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::TilingDragSwapModifier,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Mod2".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::TilingDragSwapModifier,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Mod3".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::TilingDragSwapModifier,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Mod4".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::TilingDragSwapModifier,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Mod5".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::TilingDragSwapModifier,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Shift".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::TilingDragSwapModifier,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Control".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::TilingDragSwapModifier,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Ctrl".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::TilingDragSwapModifier,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'+".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::TilingDragSwapModifier,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 7 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_BAR_STRIP_WORKSPACE_NAME: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"value".as_ptr().cast_mut(),
    next_state: ParserState::Call,
    extra: C2RustUnnamed_5 {
        call_identifier: 8 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_CRITERION_FROM_STR_START: [cmdp_token; 3] = [
    token {
        name: c"'\"".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::CriterionFromStr,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'auto".as_ptr().cast_mut(),
        identifier: c"kind".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 9 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'user".as_ptr().cast_mut(),
        identifier: c"kind".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 10 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_DEFAULT_BORDER_PIXELS_PX: [cmdp_token; 2] = [
    token {
        name: c"'px".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::DefaultBorderPixelsPx,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 11 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_WORKSPACE_BACK_AND_FORTH: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"value".as_ptr().cast_mut(),
    next_state: ParserState::Call,
    extra: C2RustUnnamed_5 {
        call_identifier: 12 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_ASSIGN_WORKSPACE_NUMBER: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"number".as_ptr().cast_mut(),
    next_state: ParserState::Call,
    extra: C2RustUnnamed_5 {
        call_identifier: 13 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_BAR_WORKSPACE_MIN_WIDTH: [cmdp_token; 1] = [token {
    name: c"number".as_ptr().cast_mut(),
    identifier: c"width".as_ptr().cast_mut(),
    next_state: ParserState::BarWorkspaceMinWidthPx,
    extra: C2RustUnnamed_5 {
        call_identifier: 0 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_FLOATING_MAXIMUM_SIZE_X: [cmdp_token; 1] = [token {
    name: c"'x".as_ptr().cast_mut(),
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: ParserState::FloatingMaximumSizeHeight,
    extra: C2RustUnnamed_5 {
        call_identifier: 0 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_FLOATING_MINIMUM_SIZE_X: [cmdp_token; 1] = [token {
    name: c"'x".as_ptr().cast_mut(),
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: ParserState::FloatingMinimumSizeHeight,
    extra: C2RustUnnamed_5 {
        call_identifier: 0 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_POPUP_DURING_FULLSCREEN: [cmdp_token; 4] = [
    token {
        name: c"'ignore".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 14 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'leave_fullscreen".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 15 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'all".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 16 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'smart".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 17 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_BAR_COLORS_IGNORE_LINE: [cmdp_token; 1] = [token {
    name: c"line".as_ptr().cast_mut(),
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: ParserState::BarColors,
    extra: C2RustUnnamed_5 {
        call_identifier: 0 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_CRITERION_FROM_STR_END: [cmdp_token; 1] = [token {
    name: c"'\"".as_ptr().cast_mut(),
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: ParserState::Call,
    extra: C2RustUnnamed_5 {
        call_identifier: 18 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_BAR_COLORS_BACKGROUND: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"background".as_ptr().cast_mut(),
    next_state: ParserState::BarColorsText,
    extra: C2RustUnnamed_5 {
        call_identifier: 0 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_BAR_WORKSPACE_BUTTONS: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"value".as_ptr().cast_mut(),
    next_state: ParserState::Call,
    extra: C2RustUnnamed_5 {
        call_identifier: 19 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_BAR_WORKSPACE_COMMAND: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"command".as_ptr().cast_mut(),
    next_state: ParserState::Call,
    extra: C2RustUnnamed_5 {
        call_identifier: 20 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_DEFAULT_BORDER_PIXELS: [cmdp_token; 2] = [
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 21 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"number".as_ptr().cast_mut(),
        identifier: c"width".as_ptr().cast_mut(),
        next_state: ParserState::DefaultBorderPixelsPx,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_WORKSPACE_OUTPUT_WORD: [cmdp_token; 2] = [
    token {
        name: c"word".as_ptr().cast_mut(),
        identifier: c"output".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 22 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::Initial,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_BAR_SEPARATOR_SYMBOL: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"separator".as_ptr().cast_mut(),
    next_state: ParserState::Call,
    extra: C2RustUnnamed_5 {
        call_identifier: 23 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_FORCE_FOCUS_WRAPPING: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"value".as_ptr().cast_mut(),
    next_state: ParserState::Call,
    extra: C2RustUnnamed_5 {
        call_identifier: 24 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_BAR_BINDSYM_COMMAND: [cmdp_token; 2] = [
    token {
        name: c"'--release".as_ptr().cast_mut(),
        identifier: c"release".as_ptr().cast_mut(),
        next_state: ParserState::BarBindsymCommand,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"string".as_ptr().cast_mut(),
        identifier: c"command".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 25 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_BAR_TRAY_PADDING_PX: [cmdp_token; 2] = [
    token {
        name: c"'px".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::BarTrayPaddingPx,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 26 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_DEFAULT_ORIENTATION: [cmdp_token; 3] = [
    token {
        name: c"'horizontal".as_ptr().cast_mut(),
        identifier: c"orientation".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 27 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'vertical".as_ptr().cast_mut(),
        identifier: c"orientation".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 28 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'auto".as_ptr().cast_mut(),
        identifier: c"orientation".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 29 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_FOCUS_FOLLOWS_MOUSE: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"value".as_ptr().cast_mut(),
    next_state: ParserState::Call,
    extra: C2RustUnnamed_5 {
        call_identifier: 30 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_BAR_PADDING_BOTTOM: [cmdp_token; 3] = [
    token {
        name: c"'px".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::BarPaddingBottom,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"number".as_ptr().cast_mut(),
        identifier: c"left".as_ptr().cast_mut(),
        next_state: ParserState::BarPaddingLeft,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 31 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_BAR_STATUS_COMMAND: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"command".as_ptr().cast_mut(),
    next_state: ParserState::Call,
    extra: C2RustUnnamed_5 {
        call_identifier: 32 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_BAR_WHEEL_DOWN_CMD: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"command".as_ptr().cast_mut(),
    next_state: ParserState::Call,
    extra: C2RustUnnamed_5 {
        call_identifier: 33 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_COLOR_CHILD_BORDER: [cmdp_token; 2] = [
    token {
        name: c"word".as_ptr().cast_mut(),
        identifier: c"child_border".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 34 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 35 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_CRITERION_FROM_STR: [cmdp_token; 2] = [
    token {
        name: c"'auto".as_ptr().cast_mut(),
        identifier: c"kind".as_ptr().cast_mut(),
        next_state: ParserState::CriterionFromStrEnd,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'user".as_ptr().cast_mut(),
        identifier: c"kind".as_ptr().cast_mut(),
        next_state: ParserState::CriterionFromStrEnd,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_FOR_WINDOW_COMMAND: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"command".as_ptr().cast_mut(),
    next_state: ParserState::Call,
    extra: C2RustUnnamed_5 {
        call_identifier: 36 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_BAR_COLORS_BORDER: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"border".as_ptr().cast_mut(),
    next_state: ParserState::BarColorsBackground,
    extra: C2RustUnnamed_5 {
        call_identifier: 0 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_BAR_COLORS_SINGLE: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"color".as_ptr().cast_mut(),
    next_state: ParserState::Call,
    extra: C2RustUnnamed_5 {
        call_identifier: 37 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_BAR_PADDING_RIGHT: [cmdp_token; 3] = [
    token {
        name: c"'px".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::BarPaddingRight,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"number".as_ptr().cast_mut(),
        identifier: c"bottom".as_ptr().cast_mut(),
        next_state: ParserState::BarPaddingBottom,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 38 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_FLOATING_MODIFIER: [cmdp_token; 10] = [
    token {
        name: c"'Mod1".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::FloatingModifier,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Mod2".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::FloatingModifier,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Mod3".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::FloatingModifier,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Mod4".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::FloatingModifier,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Mod5".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::FloatingModifier,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Shift".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::FloatingModifier,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Control".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::FloatingModifier,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Ctrl".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::FloatingModifier,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'+".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::FloatingModifier,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 39 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_HIDE_EDGE_BORDERS: [cmdp_token; 12] = [
    token {
        name: c"'none".as_ptr().cast_mut(),
        identifier: c"hide_borders".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 40 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'vertical".as_ptr().cast_mut(),
        identifier: c"hide_borders".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 41 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'horizontal".as_ptr().cast_mut(),
        identifier: c"hide_borders".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 42 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'both".as_ptr().cast_mut(),
        identifier: c"hide_borders".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 43 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'smart_no_gaps".as_ptr().cast_mut(),
        identifier: c"hide_borders".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 44 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'smart".as_ptr().cast_mut(),
        identifier: c"hide_borders".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 45 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'1".as_ptr().cast_mut(),
        identifier: c"hide_borders".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 46 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'yes".as_ptr().cast_mut(),
        identifier: c"hide_borders".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 47 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'true".as_ptr().cast_mut(),
        identifier: c"hide_borders".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 48 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'on".as_ptr().cast_mut(),
        identifier: c"hide_borders".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 49 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'enable".as_ptr().cast_mut(),
        identifier: c"hide_borders".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 50 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'active".as_ptr().cast_mut(),
        identifier: c"hide_borders".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 51 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_WORKSPACE_COMMAND: [cmdp_token; 2] = [
    token {
        name: c"'output".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::WorkspaceOutputWord,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'gaps".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::Gaps,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_ASSIGN_WORKSPACE: [cmdp_token; 5] = [
    token {
        name: c"'→".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::AssignWorkspace,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'output".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::AssignOutput,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'workspace".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::AssignWorkspace,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'number".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::AssignWorkspaceNumber,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"string".as_ptr().cast_mut(),
        identifier: c"workspace".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 52 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_BAR_COLORS_BRACE: [cmdp_token; 2] = [
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::BarColorsBrace,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'{".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::BarColors,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_BAR_HIDDEN_STATE: [cmdp_token; 2] = [
    token {
        name: c"'hide".as_ptr().cast_mut(),
        identifier: c"hidden_state".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 53 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'show".as_ptr().cast_mut(),
        identifier: c"hidden_state".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 54 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_BAR_PADDING_LEFT: [cmdp_token; 2] = [
    token {
        name: c"'px".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::BarPaddingLeft,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 55 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_BAR_TRAY_PADDING: [cmdp_token; 1] = [token {
    name: c"number".as_ptr().cast_mut(),
    identifier: c"padding_px".as_ptr().cast_mut(),
    next_state: ParserState::BarTrayPaddingPx,
    extra: C2RustUnnamed_5 {
        call_identifier: 0 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_BAR_WHEEL_UP_CMD: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"command".as_ptr().cast_mut(),
    next_state: ParserState::Call,
    extra: C2RustUnnamed_5 {
        call_identifier: 56 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_COLOR_BACKGROUND: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"background".as_ptr().cast_mut(),
    next_state: ParserState::ColorText,
    extra: C2RustUnnamed_5 {
        call_identifier: 0 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_IPC_KILL_TIMEOUT: [cmdp_token; 1] = [token {
    name: c"number".as_ptr().cast_mut(),
    identifier: c"timeout".as_ptr().cast_mut(),
    next_state: ParserState::Call,
    extra: C2RustUnnamed_5 {
        call_identifier: 57 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_MODE_BINDCOMMAND: [cmdp_token; 5] = [
    token {
        name: c"'--release".as_ptr().cast_mut(),
        identifier: c"release".as_ptr().cast_mut(),
        next_state: ParserState::ModeBindcommand,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'--border".as_ptr().cast_mut(),
        identifier: c"border".as_ptr().cast_mut(),
        next_state: ParserState::ModeBindcommand,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'--whole-window".as_ptr().cast_mut(),
        identifier: c"whole_window".as_ptr().cast_mut(),
        next_state: ParserState::ModeBindcommand,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'--exclude-titlebar".as_ptr().cast_mut(),
        identifier: c"exclude_titlebar".as_ptr().cast_mut(),
        next_state: ParserState::ModeBindcommand,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"string".as_ptr().cast_mut(),
        identifier: c"command".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 58 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_MODE_IGNORE_LINE: [cmdp_token; 1] = [token {
    name: c"line".as_ptr().cast_mut(),
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: ParserState::Mode,
    extra: C2RustUnnamed_5 {
        call_identifier: 0 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_TILING_DRAG_MODE: [cmdp_token; 3] = [
    token {
        name: c"'modifier".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: ParserState::TilingDragMode,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'titlebar".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: ParserState::TilingDragMode,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 59 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_WORKSPACE_LAYOUT: [cmdp_token; 4] = [
    token {
        name: c"'default".as_ptr().cast_mut(),
        identifier: c"layout".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 60 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'stacking".as_ptr().cast_mut(),
        identifier: c"layout".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 61 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'stacked".as_ptr().cast_mut(),
        identifier: c"layout".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 62 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'tabbed".as_ptr().cast_mut(),
        identifier: c"layout".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 63 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_BAR_BAR_COMMAND: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"command".as_ptr().cast_mut(),
    next_state: ParserState::Call,
    extra: C2RustUnnamed_5 {
        call_identifier: 64 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_BAR_COLORS_TEXT: [cmdp_token; 2] = [
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 65 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"word".as_ptr().cast_mut(),
        identifier: c"text".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 66 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_BAR_IGNORE_LINE: [cmdp_token; 1] = [token {
    name: c"line".as_ptr().cast_mut(),
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: ParserState::Bar,
    extra: C2RustUnnamed_5 {
        call_identifier: 0 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_BAR_PADDING_TOP: [cmdp_token; 3] = [
    token {
        name: c"'px".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::BarPaddingTop,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"number".as_ptr().cast_mut(),
        identifier: c"right_or_right_and_left".as_ptr().cast_mut(),
        next_state: ParserState::BarPaddingRight,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 67 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_BAR_SOCKET_PATH: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"path".as_ptr().cast_mut(),
    next_state: ParserState::Call,
    extra: C2RustUnnamed_5 {
        call_identifier: 68 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_BAR_TRAY_OUTPUT: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"output".as_ptr().cast_mut(),
    next_state: ParserState::Call,
    extra: C2RustUnnamed_5 {
        call_identifier: 69 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_COLOR_INDICATOR: [cmdp_token; 2] = [
    token {
        name: c"word".as_ptr().cast_mut(),
        identifier: c"indicator".as_ptr().cast_mut(),
        next_state: ParserState::ColorChildBorder,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 70 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_DISABLE_RANDR15: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"value".as_ptr().cast_mut(),
    next_state: ParserState::Call,
    extra: C2RustUnnamed_5 {
        call_identifier: 71 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_GAPS_WITH_SCOPE: [cmdp_token; 1] = [token {
    name: c"number".as_ptr().cast_mut(),
    identifier: c"value".as_ptr().cast_mut(),
    next_state: ParserState::GapsEnd,
    extra: C2RustUnnamed_5 {
        call_identifier: 0 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_CRITERION_FROM: [cmdp_token; 1] = [token {
    name: c"'=".as_ptr().cast_mut(),
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: ParserState::CriterionFromStrStart,
    extra: C2RustUnnamed_5 {
        call_identifier: 0 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_DEFAULT_BORDER: [cmdp_token; 4] = [
    token {
        name: c"'normal".as_ptr().cast_mut(),
        identifier: c"border".as_ptr().cast_mut(),
        next_state: ParserState::DefaultBorderPixels,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'pixel".as_ptr().cast_mut(),
        identifier: c"border".as_ptr().cast_mut(),
        next_state: ParserState::DefaultBorderPixels,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'1pixel".as_ptr().cast_mut(),
        identifier: c"border".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 72 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'none".as_ptr().cast_mut(),
        identifier: c"border".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 73 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_FOCUS_WRAPPING: [cmdp_token; 14] = [
    token {
        name: c"'1".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 74 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'yes".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 75 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'true".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 76 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'on".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 77 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'enable".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 78 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'active".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 79 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'0".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 80 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'no".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 81 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'false".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 82 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'off".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 83 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'disable".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 84 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'inactive".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 85 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'force".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 86 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'workspace".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 87 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_FORCE_XINERAMA: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"value".as_ptr().cast_mut(),
    next_state: ParserState::Call,
    extra: C2RustUnnamed_5 {
        call_identifier: 88 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_ASSIGN_OUTPUT: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"output".as_ptr().cast_mut(),
    next_state: ParserState::Call,
    extra: C2RustUnnamed_5 {
        call_identifier: 89 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_CRITERION_STR: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"cvalue".as_ptr().cast_mut(),
    next_state: ParserState::Call,
    extra: C2RustUnnamed_5 {
        call_identifier: 90 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_MOUSE_WARPING: [cmdp_token; 2] = [
    token {
        name: c"'none".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 91 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'output".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 92 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_RESTART_STATE: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"path".as_ptr().cast_mut(),
    next_state: ParserState::Call,
    extra: C2RustUnnamed_5 {
        call_identifier: 93 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_SMART_BORDERS: [cmdp_token; 7] = [
    token {
        name: c"'1".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 94 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'yes".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 95 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'true".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 96 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'on".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 97 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'enable".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 98 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'active".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 99 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'no_gaps".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 100 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_BAR_MODIFIER: [cmdp_token; 12] = [
    token {
        name: c"'off".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 101 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'none".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 102 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Mod1".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::BarModifier,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Mod2".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::BarModifier,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Mod3".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::BarModifier,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Mod4".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::BarModifier,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Mod5".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::BarModifier,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Shift".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::BarModifier,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Control".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::BarModifier,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Ctrl".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::BarModifier,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'+".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::BarModifier,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 103 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_BAR_POSITION: [cmdp_token; 2] = [
    token {
        name: c"'top".as_ptr().cast_mut(),
        identifier: c"position".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 104 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'bottom".as_ptr().cast_mut(),
        identifier: c"position".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 105 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_COLOR_BORDER: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"border".as_ptr().cast_mut(),
    next_state: ParserState::ColorBackground,
    extra: C2RustUnnamed_5 {
        call_identifier: 0 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_COLOR_SINGLE: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"color".as_ptr().cast_mut(),
    next_state: ParserState::Call,
    extra: C2RustUnnamed_5 {
        call_identifier: 106 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_FAKE_OUTPUTS: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"outputs".as_ptr().cast_mut(),
    next_state: ParserState::Call,
    extra: C2RustUnnamed_5 {
        call_identifier: 107 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_MODE_BINDING: [cmdp_token; 20] = [
    token {
        name: c"'--release".as_ptr().cast_mut(),
        identifier: c"release".as_ptr().cast_mut(),
        next_state: ParserState::ModeBinding,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'--border".as_ptr().cast_mut(),
        identifier: c"border".as_ptr().cast_mut(),
        next_state: ParserState::ModeBinding,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'--whole-window".as_ptr().cast_mut(),
        identifier: c"whole_window".as_ptr().cast_mut(),
        next_state: ParserState::ModeBinding,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'--exclude-titlebar".as_ptr().cast_mut(),
        identifier: c"exclude_titlebar".as_ptr().cast_mut(),
        next_state: ParserState::ModeBinding,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Mod1".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::ModeBinding,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Mod2".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::ModeBinding,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Mod3".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::ModeBinding,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Mod4".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::ModeBinding,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Mod5".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::ModeBinding,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Shift".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::ModeBinding,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Control".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::ModeBinding,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Ctrl".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::ModeBinding,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Mode_switch".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::ModeBinding,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Group1".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::ModeBinding,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Group2".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::ModeBinding,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Group3".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::ModeBinding,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Group4".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::ModeBinding,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'$mod".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::ModeBinding,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'+".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::ModeBinding,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"word".as_ptr().cast_mut(),
        identifier: c"key".as_ptr().cast_mut(),
        next_state: ParserState::ModeBindcommand,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_NO_FOCUS_END: [cmdp_token; 1] = [token {
    name: c"end".as_ptr().cast_mut(),
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: ParserState::Call,
    extra: C2RustUnnamed_5 {
        call_identifier: 108 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_BAR_BINDSYM: [cmdp_token; 2] = [
    token {
        name: c"'--release".as_ptr().cast_mut(),
        identifier: c"release".as_ptr().cast_mut(),
        next_state: ParserState::BarBindsym,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"word".as_ptr().cast_mut(),
        identifier: c"button".as_ptr().cast_mut(),
        next_state: ParserState::BarBindsymCommand,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_BAR_PADDING: [cmdp_token; 1] = [token {
    name: c"number".as_ptr().cast_mut(),
    identifier: c"top_or_all".as_ptr().cast_mut(),
    next_state: ParserState::BarPaddingTop,
    extra: C2RustUnnamed_5 {
        call_identifier: 0 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_BAR_VERBOSE: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"value".as_ptr().cast_mut(),
    next_state: ParserState::Call,
    extra: C2RustUnnamed_5 {
        call_identifier: 109 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_BINDCOMMAND: [cmdp_token; 5] = [
    token {
        name: c"'--release".as_ptr().cast_mut(),
        identifier: c"release".as_ptr().cast_mut(),
        next_state: ParserState::Bindcommand,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'--border".as_ptr().cast_mut(),
        identifier: c"border".as_ptr().cast_mut(),
        next_state: ParserState::Bindcommand,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'--whole-window".as_ptr().cast_mut(),
        identifier: c"whole_window".as_ptr().cast_mut(),
        next_state: ParserState::Bindcommand,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'--exclude-titlebar".as_ptr().cast_mut(),
        identifier: c"exclude_titlebar".as_ptr().cast_mut(),
        next_state: ParserState::Bindcommand,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"string".as_ptr().cast_mut(),
        identifier: c"command".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 110 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_IGNORE_LINE: [cmdp_token; 1] = [token {
    name: c"line".as_ptr().cast_mut(),
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: ParserState::Initial,
    extra: C2RustUnnamed_5 {
        call_identifier: 0 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_TILING_DRAG: [cmdp_token; 9] = [
    token {
        name: c"'0".as_ptr().cast_mut(),
        identifier: c"off".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 111 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'no".as_ptr().cast_mut(),
        identifier: c"off".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 112 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'false".as_ptr().cast_mut(),
        identifier: c"off".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 113 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'off".as_ptr().cast_mut(),
        identifier: c"off".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 114 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'disable".as_ptr().cast_mut(),
        identifier: c"off".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 115 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'inactive".as_ptr().cast_mut(),
        identifier: c"off".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 116 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'swap_modifier".as_ptr().cast_mut(),
        identifier: c"swap_modifier".as_ptr().cast_mut(),
        next_state: ParserState::TilingDragSwapModifier,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'modifier".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: ParserState::TilingDragMode,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'titlebar".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: ParserState::TilingDragMode,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_TITLE_ALIGN: [cmdp_token; 3] = [
    token {
        name: c"'left".as_ptr().cast_mut(),
        identifier: c"alignment".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 117 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'center".as_ptr().cast_mut(),
        identifier: c"alignment".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 118 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'right".as_ptr().cast_mut(),
        identifier: c"alignment".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 119 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_BAR_COLORS: [cmdp_token; 15] = [
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::BarColors,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'#".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::BarColorsIgnoreLine,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'set".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::BarColorsIgnoreLine,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'background".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: ParserState::BarColorsSingle,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'statusline".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: ParserState::BarColorsSingle,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'separator".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: ParserState::BarColorsSingle,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'focused_background".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: ParserState::BarColorsSingle,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'focused_statusline".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: ParserState::BarColorsSingle,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'focused_separator".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: ParserState::BarColorsSingle,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'focused_workspace".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: ParserState::BarColorsBorder,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'active_workspace".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: ParserState::BarColorsBorder,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'inactive_workspace".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: ParserState::BarColorsBorder,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'urgent_workspace".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: ParserState::BarColorsBorder,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'binding_mode".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: ParserState::BarColorsBorder,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'}".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::Bar,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_BAR_HEIGHT: [cmdp_token; 1] = [token {
    name: c"number".as_ptr().cast_mut(),
    identifier: c"value".as_ptr().cast_mut(),
    next_state: ParserState::Call,
    extra: C2RustUnnamed_5 {
        call_identifier: 120 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_BAR_OUTPUT: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"output".as_ptr().cast_mut(),
    next_state: ParserState::Call,
    extra: C2RustUnnamed_5 {
        call_identifier: 121 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_COLOR_TEXT: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"text".as_ptr().cast_mut(),
    next_state: ParserState::ColorIndicator,
    extra: C2RustUnnamed_5 {
        call_identifier: 0 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_FOR_WINDOW: [cmdp_token; 1] = [token {
    name: c"'[".as_ptr().cast_mut(),
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: ParserState::Call,
    extra: C2RustUnnamed_5 {
        call_identifier: 122 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_IPC_SOCKET: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"path".as_ptr().cast_mut(),
    next_state: ParserState::Call,
    extra: C2RustUnnamed_5 {
        call_identifier: 123 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_SHOW_MARKS: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"value".as_ptr().cast_mut(),
    next_state: ParserState::Call,
    extra: C2RustUnnamed_5 {
        call_identifier: 124 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_SMART_GAPS: [cmdp_token; 13] = [
    token {
        name: c"'1".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 125 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'yes".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 126 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'true".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 127 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'on".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 128 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'enable".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 129 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'active".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 130 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'0".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 131 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'no".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 132 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'false".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 133 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'off".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 134 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'disable".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 135 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'inactive".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 136 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'inverse_outer".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 137 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_CRITERION: [cmdp_token; 1] = [token {
    name: c"'=".as_ptr().cast_mut(),
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: ParserState::CriterionStr,
    extra: C2RustUnnamed_5 {
        call_identifier: 0 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_MODEBRACE: [cmdp_token; 2] = [
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::Modebrace,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'{".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::Mode,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_WORKSPACE: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"workspace".as_ptr().cast_mut(),
    next_state: ParserState::WorkspaceCommand,
    extra: C2RustUnnamed_5 {
        call_identifier: 0 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_BARBRACE: [cmdp_token; 2] = [
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::Barbrace,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'{".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 138 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_BAR_FONT: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"font".as_ptr().cast_mut(),
    next_state: ParserState::Call,
    extra: C2RustUnnamed_5 {
        call_identifier: 139 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_BAR_MODE: [cmdp_token; 3] = [
    token {
        name: c"'dock".as_ptr().cast_mut(),
        identifier: c"mode".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 140 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'hide".as_ptr().cast_mut(),
        identifier: c"mode".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 141 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'invisible".as_ptr().cast_mut(),
        identifier: c"mode".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 142 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_CRITERIA: [cmdp_token; 17] = [
    token {
        name: c"'class".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: ParserState::Criterion,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'instance".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: ParserState::Criterion,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'window_role".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: ParserState::Criterion,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'con_id".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: ParserState::Criterion,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'id".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: ParserState::Criterion,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'window_type".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: ParserState::Criterion,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'con_mark".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: ParserState::Criterion,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'title".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: ParserState::Criterion,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'urgent".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: ParserState::Criterion,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'workspace".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: ParserState::Criterion,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'machine".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: ParserState::Criterion,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'floating_from".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: ParserState::CriterionFrom,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'tiling_from".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: ParserState::CriterionFrom,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'tiling".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 143 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'floating".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 144 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'all".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 145 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"']".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 146 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_GAPS_END: [cmdp_token; 2] = [
    token {
        name: c"'px".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::GapsEnd,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 147 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_MODENAME: [cmdp_token; 2] = [
    token {
        name: c"'--pango_markup".as_ptr().cast_mut(),
        identifier: c"pango_markup".as_ptr().cast_mut(),
        next_state: ParserState::Modename,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"word".as_ptr().cast_mut(),
        identifier: c"modename".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 148 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_NO_FOCUS: [cmdp_token; 1] = [token {
    name: c"'[".as_ptr().cast_mut(),
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: ParserState::Call,
    extra: C2RustUnnamed_5 {
        call_identifier: 149 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_BINDING: [cmdp_token; 20] = [
    token {
        name: c"'--release".as_ptr().cast_mut(),
        identifier: c"release".as_ptr().cast_mut(),
        next_state: ParserState::Binding,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'--border".as_ptr().cast_mut(),
        identifier: c"border".as_ptr().cast_mut(),
        next_state: ParserState::Binding,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'--whole-window".as_ptr().cast_mut(),
        identifier: c"whole_window".as_ptr().cast_mut(),
        next_state: ParserState::Binding,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'--exclude-titlebar".as_ptr().cast_mut(),
        identifier: c"exclude_titlebar".as_ptr().cast_mut(),
        next_state: ParserState::Binding,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Mod1".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::Binding,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Mod2".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::Binding,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Mod3".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::Binding,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Mod4".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::Binding,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Mod5".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::Binding,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Shift".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::Binding,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Control".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::Binding,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Ctrl".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::Binding,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Mode_switch".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::Binding,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Group1".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::Binding,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Group2".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::Binding,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Group3".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::Binding,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Group4".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::Binding,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'$mod".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ParserState::Binding,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'+".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::Binding,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"word".as_ptr().cast_mut(),
        identifier: c"key".as_ptr().cast_mut(),
        next_state: ParserState::Bindcommand,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_INCLUDE: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"pattern".as_ptr().cast_mut(),
    next_state: ParserState::Call,
    extra: C2RustUnnamed_5 {
        call_identifier: 150 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_INITIAL: [cmdp_token; 60] = [
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::Initial,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"error".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::Initial,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'#".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::IgnoreLine,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'set ".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::IgnoreLine,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'set\t".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::IgnoreLine,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'set_from_resource".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::IgnoreLine,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'include".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::Include,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'bindsym".as_ptr().cast_mut(),
        identifier: c"bindtype".as_ptr().cast_mut(),
        next_state: ParserState::Binding,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'bindcode".as_ptr().cast_mut(),
        identifier: c"bindtype".as_ptr().cast_mut(),
        next_state: ParserState::Binding,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'bind".as_ptr().cast_mut(),
        identifier: c"bindtype".as_ptr().cast_mut(),
        next_state: ParserState::Binding,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'bar".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::Barbrace,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'font".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::Font,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'mode".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::Modename,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'gaps".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::Gaps,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'smart_borders".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::SmartBorders,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'smart_gaps".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::SmartGaps,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'floating_minimum_size".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::FloatingMinimumSizeWidth,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'floating_maximum_size".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::FloatingMaximumSizeWidth,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'floating_modifier".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::FloatingModifier,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'default_orientation".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::DefaultOrientation,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'workspace_layout".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::WorkspaceLayout,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'default_border".as_ptr().cast_mut(),
        identifier: c"windowtype".as_ptr().cast_mut(),
        next_state: ParserState::DefaultBorder,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'new_window".as_ptr().cast_mut(),
        identifier: c"windowtype".as_ptr().cast_mut(),
        next_state: ParserState::DefaultBorder,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'default_floating_border".as_ptr().cast_mut(),
        identifier: c"windowtype".as_ptr().cast_mut(),
        next_state: ParserState::DefaultBorder,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'new_float".as_ptr().cast_mut(),
        identifier: c"windowtype".as_ptr().cast_mut(),
        next_state: ParserState::DefaultBorder,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'hide_edge_borders".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::HideEdgeBorders,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'for_window".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::ForWindow,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'assign".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::Assign,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'no_focus".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::NoFocus,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'focus_follows_mouse".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::FocusFollowsMouse,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'mouse_warping".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::MouseWarping,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'focus_wrapping".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::FocusWrapping,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'force_focus_wrapping".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::ForceFocusWrapping,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'force_xinerama".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::ForceXinerama,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'force-xinerama".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::ForceXinerama,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'disable_randr15".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::DisableRandr15,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'disable-randr15".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::DisableRandr15,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'workspace_auto_back_and_forth".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::WorkspaceBackAndForth,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'fake_outputs".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::FakeOutputs,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'fake-outputs".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::FakeOutputs,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'force_display_urgency_hint".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::ForceDisplayUrgencyHint,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'focus_on_window_activation".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::FocusOnWindowActivation,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'title_align".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::TitleAlign,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'show_marks".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::ShowMarks,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'workspace".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::Workspace,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'ipc_socket".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::IpcSocket,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'ipc-socket".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::IpcSocket,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'ipc_kill_timeout".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::IpcKillTimeout,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'restart_state".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::RestartState,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'popup_during_fullscreen".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::PopupDuringFullscreen,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'tiling_drag".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::TilingDrag,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'exec_always".as_ptr().cast_mut(),
        identifier: c"exectype".as_ptr().cast_mut(),
        next_state: ParserState::Exec,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'exec".as_ptr().cast_mut(),
        identifier: c"exectype".as_ptr().cast_mut(),
        next_state: ParserState::Exec,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'client.background".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: ParserState::ColorSingle,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'client.focused_inactive".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: ParserState::ColorBorder,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'client.focused_tab_title".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: ParserState::ColorBorder,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'client.focused".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: ParserState::ColorBorder,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'client.unfocused".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: ParserState::ColorBorder,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'client.urgent".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: ParserState::ColorBorder,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'client.placeholder".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: ParserState::ColorBorder,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_ASSIGN: [cmdp_token; 1] = [token {
    name: c"'[".as_ptr().cast_mut(),
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: ParserState::Call,
    extra: C2RustUnnamed_5 {
        call_identifier: 151 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_BAR_ID: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"bar_id".as_ptr().cast_mut(),
    next_state: ParserState::Call,
    extra: C2RustUnnamed_5 {
        call_identifier: 152 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_EXEC: [cmdp_token; 2] = [
    token {
        name: c"'--no-startup-id".as_ptr().cast_mut(),
        identifier: c"no_startup_id".as_ptr().cast_mut(),
        next_state: ParserState::Exec,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"string".as_ptr().cast_mut(),
        identifier: c"command".as_ptr().cast_mut(),
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 153 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_FONT: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"font".as_ptr().cast_mut(),
    next_state: ParserState::Call,
    extra: C2RustUnnamed_5 {
        call_identifier: 154 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_GAPS: [cmdp_token; 8] = [
    token {
        name: c"'inner".as_ptr().cast_mut(),
        identifier: c"scope".as_ptr().cast_mut(),
        next_state: ParserState::GapsWithScope,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'outer".as_ptr().cast_mut(),
        identifier: c"scope".as_ptr().cast_mut(),
        next_state: ParserState::GapsWithScope,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'horizontal".as_ptr().cast_mut(),
        identifier: c"scope".as_ptr().cast_mut(),
        next_state: ParserState::GapsWithScope,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'vertical".as_ptr().cast_mut(),
        identifier: c"scope".as_ptr().cast_mut(),
        next_state: ParserState::GapsWithScope,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'top".as_ptr().cast_mut(),
        identifier: c"scope".as_ptr().cast_mut(),
        next_state: ParserState::GapsWithScope,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'right".as_ptr().cast_mut(),
        identifier: c"scope".as_ptr().cast_mut(),
        next_state: ParserState::GapsWithScope,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'bottom".as_ptr().cast_mut(),
        identifier: c"scope".as_ptr().cast_mut(),
        next_state: ParserState::GapsWithScope,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'left".as_ptr().cast_mut(),
        identifier: c"scope".as_ptr().cast_mut(),
        next_state: ParserState::GapsWithScope,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_MODE: [cmdp_token; 8] = [
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::Mode,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"error".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::Mode,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'#".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::ModeIgnoreLine,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'set".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::ModeIgnoreLine,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'bindsym".as_ptr().cast_mut(),
        identifier: c"bindtype".as_ptr().cast_mut(),
        next_state: ParserState::ModeBinding,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'bindcode".as_ptr().cast_mut(),
        identifier: c"bindtype".as_ptr().cast_mut(),
        next_state: ParserState::ModeBinding,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'bind".as_ptr().cast_mut(),
        identifier: c"bindtype".as_ptr().cast_mut(),
        next_state: ParserState::ModeBinding,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'}".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::Initial,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_BAR: [cmdp_token; 31] = [
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::Bar,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"error".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::Bar,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'#".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::BarIgnoreLine,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'set".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::BarIgnoreLine,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'i3bar_command".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::BarBarCommand,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'status_command".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::BarStatusCommand,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'workspace_command".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::BarWorkspaceCommand,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'socket_path".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::BarSocketPath,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'mode".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::BarMode,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'hidden_state".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::BarHiddenState,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'id".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::BarId,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'modifier".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::BarModifier,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'wheel_up_cmd".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::BarWheelUpCmd,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'wheel_down_cmd".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::BarWheelDownCmd,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'bindsym".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::BarBindsym,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'position".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::BarPosition,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'output".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::BarOutput,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'tray_output".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::BarTrayOutput,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'tray_padding".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::BarTrayPadding,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'font".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::BarFont,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'separator_symbol".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::BarSeparatorSymbol,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'binding_mode_indicator".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::BarBindingModeIndicator,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'workspace_buttons".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::BarWorkspaceButtons,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'workspace_min_width".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::BarWorkspaceMinWidth,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'strip_workspace_numbers".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::BarStripWorkspaceNumbers,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'strip_workspace_name".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::BarStripWorkspaceName,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'verbose".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::BarVerbose,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'height".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::BarHeight,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'padding".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::BarPadding,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'colors".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::BarColorsBrace,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'}".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ParserState::Call,
        extra: C2RustUnnamed_5 {
            call_identifier: 155 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens: [cmdp_token_ptr; 117] = unsafe {
    [
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
    ]
};
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
unsafe fn keysym_used_on_other_key(sym: libc::c_ulong, except_keycode: xcb_keycode_t) -> bool {
    let (minimum, maximum) = {
        let setup = &*xcb_get_setup(conn);
        (setup.min_keycode, setup.max_keycode)
    };
    // Preserve the old zero-start sentinel while eliminating u8 wraparound
    // control flow for the full 1..=255 native keycode range.
    minimum != 0
        && (minimum..=maximum)
            .filter(|&code| code != except_keycode)
            .any(|code| {
                (0..4).any(|level| {
                    xcb_key_symbols_get_keysym(keysyms, code, level) as libc::c_ulong == sym
                })
            })
}
// The parser and formatting own their data. Only keysym lookup and malloc ABI
// handoff require the initialized event-thread XKB/key-symbol native resources.
unsafe fn rewrite_binding(input: &CStr) -> Option<CString> {
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
            *syms.offset(0 as ::core::ffi::c_int as isize) as libc::c_ulong,
            keycode as xcb_keycode_t,
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
        *syms_0.offset(0 as ::core::ffi::c_int as isize),
        &raw mut str as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as usize,
    ) == -(1 as ::core::ffi::c_int)
    {
        errx(
            libc::EXIT_FAILURE,
            c"xkb_keysym_get_name(%u) failed".as_ptr(),
            *syms_0.offset(0 as ::core::ffi::c_int as isize),
        );
    }
    Some(format_binding(&binding, CStr::from_ptr(str.as_ptr())))
}

pub unsafe extern "C" fn verboselog(fmt: *mut ::core::ffi::c_char, args: ...) {
    let mut args_0;
    args_0 = args.clone();
    vfprintf(stdout, fmt, i333_va_list!(args_0));
}

pub unsafe extern "C" fn errorlog(fmt: *mut ::core::ffi::c_char, args: ...) {
    let mut args_0;
    args_0 = args.clone();
    vfprintf(stderr, fmt, i333_va_list!(args_0));
}

pub unsafe extern "C" fn debuglog(fmt: *mut ::core::ffi::c_char, args: ...) {}
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

struct Native<T>(std::ptr::NonNull<T>);
impl<T> Native<T> {
    /// The pointer transfers a unique complete malloc-backed XCB allocation.
    unsafe fn take(pointer: *mut T) -> Option<Self> {
        std::ptr::NonNull::new(pointer).map(Self)
    }
    fn raw(&self) -> *mut T {
        self.0.as_ptr()
    }
}
impl<T> Drop for Native<T> {
    fn drop(&mut self) {
        unsafe {
            free(self.raw().cast());
        }
    }
}

// Session ownership for the native renderer globals. Fonts remain at static
// stable addresses because the shared backend borrows the selected Font POD.
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
    connection: std::ptr::NonNull<xcb_connection_t>,
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
                draw_util_surface_free(self.connection.as_ptr(), &raw mut surface);
            }
            if self.fonts {
                set_font(&raw mut bold_font);
                i3_common::font::free_font();
                set_font(&raw mut font);
                i3_common::font::free_font();
            }
            if !xkb_keymap.is_null() {
                xkb_keymap_unref(xkb_keymap);
                xkb_keymap = std::ptr::null_mut();
            }
            if !symbols.is_null() {
                xcb_sys::xcb_key_symbols_free(symbols);
                symbols = std::ptr::null_mut();
            }
            if !keysyms.is_null() {
                xcb_sys::xcb_key_symbols_free(keysyms);
                keysyms = std::ptr::null_mut();
            }
            if win != 0 {
                xcb_sys::xcb_destroy_window(self.connection.as_ptr(), win);
                win = 0;
            }
            root_screen = std::ptr::null_mut();
            visual_type = std::ptr::null_mut();
            root = 0;
            conn = std::ptr::null_mut();
            xcb_sys::xcb_disconnect(self.connection.as_ptr());
        }
    }
}
unsafe fn take_cstring(pointer: *mut libc::c_char) -> CString {
    let owned = CStr::from_ptr(pointer).to_owned();
    free(pointer.cast());
    owned
}
unsafe fn atom_reply(cookie: xcb_intern_atom_cookie_t, name: &CStr) -> xcb_atom_t {
    let Some(reply) = Native::take(xcb_intern_atom_reply(conn, cookie, std::ptr::null_mut()))
    else {
        errx(
            libc::EXIT_FAILURE,
            c"Could not get atom %s".as_ptr(),
            name.as_ptr(),
        );
    };
    (*reply.raw()).atom
}

unsafe fn txt(
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
    let string = i3_common::string::I3String::from_cstr(CStr::from_ptr(text));
    draw_util_text(
        &string,
        &raw mut surface,
        fg,
        bg,
        x,
        y,
        logical_px(300 as ::core::ffi::c_int) - x - logical_px(4 as ::core::ffi::c_int),
    );
}
unsafe fn handle_expose(ui: &WizardUi) -> ::core::ffi::c_int {
    let black: color_t = draw_util_hex_to_color(c"#000000".as_ptr()) as color_t;
    let white: color_t = draw_util_hex_to_color(c"#FFFFFF".as_ptr()) as color_t;
    let green: color_t = draw_util_hex_to_color(c"#00FF00".as_ptr()) as color_t;
    let red: color_t = draw_util_hex_to_color(c"#FF0000".as_ptr()) as color_t;
    draw_util_clear_surface(&raw mut surface, black);
    set_font(&raw mut font);
    if ui.step == WizardStep::Welcome {
        txt(
            2 as ::core::ffi::c_int,
            2 as ::core::ffi::c_int,
            c"You have not configured i3 yet.".as_ptr().cast_mut(),
            white,
            black,
        );
        txt(
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
            2 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            msg.as_ptr().cast_mut(),
            white,
            black,
        );

        txt(
            13 as ::core::ffi::c_int,
            6 as ::core::ffi::c_int,
            c"Yes, generate the config".as_ptr().cast_mut(),
            white,
            black,
        );
        txt(
            13 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int,
            c"No, I will use the defaults".as_ptr().cast_mut(),
            white,
            black,
        );
        txt(
            4 as ::core::ffi::c_int,
            6 as ::core::ffi::c_int,
            c"<Enter>".as_ptr().cast_mut(),
            green,
            black,
        );
        txt(
            5 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int,
            c"<ESC>".as_ptr().cast_mut(),
            red,
            black,
        );
    }
    if ui.step == WizardStep::Generate {
        txt(
            2 as ::core::ffi::c_int,
            2 as ::core::ffi::c_int,
            c"Please choose either:".as_ptr().cast_mut(),
            white,
            black,
        );
        txt(
            13 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            c"Win as default modifier".as_ptr().cast_mut(),
            white,
            black,
        );
        txt(
            13 as ::core::ffi::c_int,
            5 as ::core::ffi::c_int,
            c"Alt as default modifier".as_ptr().cast_mut(),
            white,
            black,
        );
        txt(
            2 as ::core::ffi::c_int,
            7 as ::core::ffi::c_int,
            c"Afterwards, press".as_ptr().cast_mut(),
            white,
            black,
        );
        txt(
            13 as ::core::ffi::c_int,
            9 as ::core::ffi::c_int,
            c"to write the config".as_ptr().cast_mut(),
            white,
            black,
        );
        txt(
            13 as ::core::ffi::c_int,
            10 as ::core::ffi::c_int,
            c"to abort".as_ptr().cast_mut(),
            white,
            black,
        );
        if ui.modifier == WizardModifier::Win {
            txt(
                5 as ::core::ffi::c_int,
                5 as ::core::ffi::c_int,
                c"<Alt>".as_ptr().cast_mut(),
                white,
                black,
            );
        } else {
            txt(
                5 as ::core::ffi::c_int,
                4 as ::core::ffi::c_int,
                c"<Win>".as_ptr().cast_mut(),
                white,
                black,
            );
        }
        set_font(&raw mut bold_font);
        if ui.modifier == WizardModifier::Win {
            txt(
                2 as ::core::ffi::c_int,
                4 as ::core::ffi::c_int,
                c"-> <Win>".as_ptr().cast_mut(),
                white,
                black,
            );
        } else {
            txt(
                2 as ::core::ffi::c_int,
                5 as ::core::ffi::c_int,
                c"-> <Alt>".as_ptr().cast_mut(),
                white,
                black,
            );
        }
        set_font(&raw mut font);
        txt(
            4 as ::core::ffi::c_int,
            9 as ::core::ffi::c_int,
            c"<Enter>".as_ptr().cast_mut(),
            green,
            black,
        );
        txt(
            5 as ::core::ffi::c_int,
            10 as ::core::ffi::c_int,
            c"<ESC>".as_ptr().cast_mut(),
            red,
            black,
        );
    }
    xcb_flush(conn);
    1 as ::core::ffi::c_int
}
unsafe fn handle_key_press(
    ui: &mut WizardUi,
    conn_0: *mut xcb_connection_t,
    event: *mut xcb_key_press_event_t,
) -> bool {
    printf(
        c"Keypress %d, state raw = %d\n".as_ptr(),
        (*event).detail as ::core::ffi::c_int,
        (*event).state as ::core::ffi::c_int,
    );
    let mut state_filtered: u16 = ((*event).state as u32
        & !(xcb_numlock_mask | XCB_MOD_MASK_LOCK as ::core::ffi::c_int as u32))
        as u16;
    state_filtered = (state_filtered as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as u16;
    let sym: xcb_keysym_t =
        xcb_key_press_lookup_keysym(symbols, event, state_filtered as ::core::ffi::c_int);
    printf(c"sym = %c (%d)\n".as_ptr(), sym, sym);
    if sym == XK_Return as xcb_keysym_t || sym == XK_KP_Enter as xcb_keysym_t {
        if ui.step == WizardStep::Welcome {
            ui.step = WizardStep::Generate;
            xcb_change_property(
                conn_0,
                XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as u8,
                win,
                A__NET_WM_NAME,
                A_UTF8_STRING,
                8 as u8,
                strlen(c"i3: generate config".as_ptr()) as u32,
                c"i3: generate config".as_ptr() as *const ::core::ffi::c_void,
            );
            xcb_flush(conn_0);
        } else {
            finish(ui);
            return true;
        }
    }
    if sym == XK_Up as xcb_keysym_t || sym == XK_Down as xcb_keysym_t {
        ui.modifier = if ui.modifier == WizardModifier::Alt {
            WizardModifier::Win
        } else {
            WizardModifier::Alt
        };
        handle_expose(ui);
    }
    if sym == XK_Escape as xcb_keysym_t {
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
            if code == (*event).detail {
                ui.modifier = selected;
                printf(c"This is %s!\n".as_ptr(), label.as_ptr());
            }
        }
    }
    handle_expose(ui);
    false
}
unsafe fn handle_button_press(ui: &mut WizardUi, event: &xcb_button_press_event_t) {
    if ui.step != WizardStep::Generate {
        return;
    }
    if let Some(modifier) = clicked_modifier(
        event.event_x,
        event.event_y,
        char_width,
        font.height,
        logical_px(4),
    ) {
        ui.modifier = modifier;
        handle_expose(ui);
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
// display/XKB state and retained legacy IPC adapter remain native boundaries.
unsafe fn finish(ui: &WizardUi) {
    use std::fs::File;
    use std::io::{BufRead, BufReader, Write};
    use std::os::fd::{AsRawFd, FromRawFd, OwnedFd};
    use std::os::unix::ffi::OsStrExt;
    printf(c"creating \"%s\"...\n".as_ptr(), ui.config_path.as_ptr());
    let Some(context) = std::ptr::NonNull::new(xkb_context_new(XKB_CONTEXT_NO_FLAGS)) else {
        errx(1, c"could not create xkbcommon context".as_ptr());
    };
    let context = XkbContext(context);
    let device_id = xkb_x11_get_core_keyboard_device_id(conn);
    xkb_keymap = xkb_x11_keymap_new_from_device(
        context.0.as_ptr(),
        conn,
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
        match template.rewrite(&line, ui.modifier, |input| rewrite_binding(input)) {
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
    // ipc_connect returns a new uniquely owned socket (or exits). The borrowed
    // descriptor stays open until the original reload adapter has sent its frame.
    let descriptor = OwnedFd::from_raw_fd(ipc_connect(
        ui.socket
            .as_ref()
            .map_or(std::ptr::null(), |path| path.as_ptr()),
    ));
    ipc_send_message(descriptor.as_raw_fd(), 6, 0, c"reload".as_ptr().cast());
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
    let base = take_cstring(resolve_tilde(xdg_config_home));
    let mut directory = base.to_bytes().to_vec();
    directory.extend_from_slice(b"/i3");
    let config_dir = CString::new(directory).expect("base path has no NUL");
    let mut path = config_dir.to_bytes().to_vec();
    path.extend_from_slice(b"/config");
    let config_path = CString::new(path).expect("base path has no NUL");
    let mut stbuf: stat = ::core::mem::zeroed();
    if stat(config_dir.as_ptr(), &raw mut stbuf) != 0 {
        if mkdirp(config_dir.as_ptr(), DEFAULT_DIR_MODE) != 0 {
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
    let mut screen: ::core::ffi::c_int = 0;
    conn = xcb_connect(::core::ptr::null::<::core::ffi::c_char>(), &raw mut screen);
    if conn.is_null() || xcb_connection_has_error(conn) != 0 {
        errx(1 as ::core::ffi::c_int, c"Cannot open display".as_ptr());
    }
    let mut resources = WizardResources {
        connection: std::ptr::NonNull::new(conn).unwrap(),
        fonts: false,
        surface: false,
    };
    if xkb_x11_setup_xkb_extension(
        conn,
        XKB_X11_MIN_MAJOR_XKB_VERSION as u16,
        XKB_X11_MIN_MINOR_XKB_VERSION as u16,
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
    keysyms = xcb_key_symbols_alloc(conn);
    let mut modmap_cookie: xcb_get_modifier_mapping_cookie_t =
        xcb_get_modifier_mapping_cookie_t { sequence: 0 };
    modmap_cookie = xcb_get_modifier_mapping(conn);
    symbols = xcb_key_symbols_alloc(conn);
    if symbols.is_null() || keysyms.is_null() {
        errx(1, c"Could not allocate key symbols".as_ptr());
    }
    if headless_run {
        finish(&ui);
        return 0 as ::core::ffi::c_int;
    }
    let mut _NET_WM_NAME_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as u8,
        strlen(c"_NET_WM_NAME".as_ptr()) as u16,
        c"_NET_WM_NAME".as_ptr(),
    );
    let mut _NET_WM_WINDOW_TYPE_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as u8,
        strlen(c"_NET_WM_WINDOW_TYPE".as_ptr()) as u16,
        c"_NET_WM_WINDOW_TYPE".as_ptr(),
    );
    let mut _NET_WM_WINDOW_TYPE_DIALOG_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as u8,
        strlen(c"_NET_WM_WINDOW_TYPE_DIALOG".as_ptr()) as u16,
        c"_NET_WM_WINDOW_TYPE_DIALOG".as_ptr(),
    );
    let ATOM_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as u8,
        strlen(c"ATOM".as_ptr()) as u16,
        c"ATOM".as_ptr(),
    );
    let CARDINAL_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as u8,
        strlen(c"CARDINAL".as_ptr()) as u16,
        c"CARDINAL".as_ptr(),
    );
    let UTF8_STRING_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as u8,
        strlen(c"UTF8_STRING".as_ptr()) as u16,
        c"UTF8_STRING".as_ptr(),
    );
    let sndisplay: *mut SnDisplay = sn_xcb_display_new(conn, None, None);
    let sncontext: *mut SnLauncheeContext =
        sn_launchee_context_new_from_environment(sndisplay, screen);
    let startup = StartupContext(sncontext);
    sn_display_unref(sndisplay);
    root_screen = xcb_aux_get_screen(conn, screen);
    if root_screen.is_null() {
        errx(1, c"Cannot find display screen".as_ptr());
    }
    root = (*root_screen).root;
    let Some(modmap) = Native::take(xcb_get_modifier_mapping_reply(
        conn,
        modmap_cookie,
        std::ptr::null_mut(),
    )) else {
        errx(
            libc::EXIT_FAILURE,
            c"Could not get modifier mapping".as_ptr(),
        );
    };
    let count = usize::from((*modmap.raw()).keycodes_per_modifier);
    let available = ((*modmap.raw()).length as usize)
        .checked_mul(4)
        .unwrap_or(0);
    if count * 8 > available {
        errx(
            libc::EXIT_FAILURE,
            c"Could not get modifier mapping".as_ptr(),
        );
    }
    let codes =
        std::slice::from_raw_parts(xcb_get_modifier_mapping_keycodes(modmap.raw()), count * 8);
    ui.modifier_codes = [
        codes[count * 3..count * 4].to_vec(),
        codes[count * 6..count * 7].to_vec(),
    ];
    xcb_numlock_mask = get_mod_mask_for(XCB_NUM_LOCK as u32, symbols, modmap.raw());
    init_dpi();
    font = load_font(pattern, true);
    bold_font = load_font(patternbold, true);
    resources.fonts = true;
    set_font(&raw mut font);
    let letter = i3_common::string::I3String::new("a");
    char_width = predict_text_width(&letter);
    win = xcb_generate_id(conn) as xcb_window_t;
    let mut fresh1: [u32; 2] = [
        0 as ::core::ffi::c_int as u32,
        (XCB_EVENT_MASK_EXPOSURE as ::core::ffi::c_int
            | XCB_EVENT_MASK_BUTTON_PRESS as ::core::ffi::c_int) as u32,
    ];
    xcb_create_window(
        conn,
        XCB_COPY_FROM_PARENT as u8,
        win,
        root,
        logical_px(490 as ::core::ffi::c_int) as i16,
        logical_px(297 as ::core::ffi::c_int) as i16,
        logical_px(300 as ::core::ffi::c_int) as u16,
        (15 as ::core::ffi::c_int * font.height + logical_px(4 as ::core::ffi::c_int)) as u16,
        0 as u16,
        XCB_WINDOW_CLASS_INPUT_OUTPUT as ::core::ffi::c_int as u16,
        XCB_WINDOW_CLASS_COPY_FROM_PARENT as ::core::ffi::c_int as xcb_visualid_t,
        (XCB_CW_BACK_PIXEL as ::core::ffi::c_int | XCB_CW_EVENT_MASK as ::core::ffi::c_int) as u32,
        &raw mut fresh1 as *mut u32 as *const ::core::ffi::c_void,
    );
    if !sncontext.is_null() {
        sn_launchee_context_setup_window(sncontext, win as libc::c_ulong);
    }
    xcb_map_window(conn, win);
    A__NET_WM_NAME = atom_reply(_NET_WM_NAME_cookie, c"_NET_WM_NAME");
    A__NET_WM_WINDOW_TYPE = atom_reply(_NET_WM_WINDOW_TYPE_cookie, c"_NET_WM_WINDOW_TYPE");
    A__NET_WM_WINDOW_TYPE_DIALOG = atom_reply(
        _NET_WM_WINDOW_TYPE_DIALOG_cookie,
        c"_NET_WM_WINDOW_TYPE_DIALOG",
    );
    A_ATOM = atom_reply(ATOM_cookie, c"ATOM");
    A_CARDINAL = atom_reply(CARDINAL_cookie, c"CARDINAL");
    A_UTF8_STRING = atom_reply(UTF8_STRING_cookie, c"UTF8_STRING");
    xcb_change_property(
        conn,
        XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as u8,
        win,
        A__NET_WM_WINDOW_TYPE,
        A_ATOM,
        32 as u8,
        1 as u32,
        &raw mut A__NET_WM_WINDOW_TYPE_DIALOG as *mut ::core::ffi::c_uchar
            as *const ::core::ffi::c_void,
    );
    xcb_change_property(
        conn,
        XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as u8,
        win,
        A__NET_WM_NAME,
        A_UTF8_STRING,
        8 as u8,
        strlen(c"i3: first configuration".as_ptr()) as u32,
        c"i3: first configuration".as_ptr() as *const ::core::ffi::c_void,
    );
    draw_util_surface_init(
        conn,
        &raw mut surface,
        win as xcb_drawable_t,
        get_visualtype(root_screen),
        logical_px(300 as ::core::ffi::c_int),
        15 as ::core::ffi::c_int * font.height + logical_px(4 as ::core::ffi::c_int),
    );
    xcb_flush(conn);
    resources.surface = true;
    let mut status = None;
    for _ in 0..500 {
        let reply = Native::take(xcb_grab_keyboard_reply(
            conn,
            xcb_grab_keyboard(
                conn,
                0,
                win,
                XCB_CURRENT_TIME as xcb_timestamp_t,
                XCB_GRAB_MODE_ASYNC as u8,
                XCB_GRAB_MODE_ASYNC as u8,
            ),
            std::ptr::null_mut(),
        ));
        status = reply.as_ref().map(|reply| (*reply.raw()).status);
        usleep(1000);
        if status == Some(XCB_GRAB_STATUS_SUCCESS as u8) {
            break;
        }
    }
    if status != Some(XCB_GRAB_STATUS_SUCCESS as u8) {
        fprintf(
            stderr,
            c"Could not grab keyboard, status = %d\n".as_ptr(),
            status.map_or(-1, i32::from),
        );
        return -1;
    }
    if !sncontext.is_null() {
        sn_launchee_context_complete(sncontext);
    }
    drop(startup);
    xcb_flush(conn);
    loop {
        let Some(event_owner) = Native::take(xcb_wait_for_event(conn)) else {
            break;
        };
        let event = event_owner.raw();
        if (*event).response_type as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            fprintf(
                stderr,
                c"X11 Error received! sequence %x\n".as_ptr(),
                (*event).sequence as ::core::ffi::c_int,
            );
        } else {
            let type_0: ::core::ffi::c_int =
                (*event).response_type as ::core::ffi::c_int & 0x7f as ::core::ffi::c_int;
            match type_0 {
                XCB_KEY_PRESS => {
                    if handle_key_press(&mut ui, conn, event.cast()) {
                        return 0;
                    }
                }
                XCB_BUTTON_PRESS => {
                    handle_button_press(&mut ui, &*event.cast::<xcb_button_press_event_t>());
                }
                XCB_EXPOSE => {
                    if (*(event as *mut xcb_expose_event_t)).count as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    {
                        handle_expose(&ui);
                    }
                }
                _ => {}
            }
        }
    }

    0 as ::core::ffi::c_int
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

use xcb_sys::{
    xcb_atom_t, xcb_aux_get_screen, xcb_button_press_event_t, xcb_change_property, xcb_charinfo_t,
    xcb_connect, xcb_connection_has_error, xcb_connection_t, xcb_create_window, xcb_drawable_t,
    xcb_expose_event_t, xcb_flush, xcb_generate_id, xcb_generic_error_t, xcb_generic_event_t,
    xcb_get_modifier_mapping, xcb_get_modifier_mapping_cookie_t, xcb_get_modifier_mapping_keycodes,
    xcb_get_modifier_mapping_reply, xcb_get_modifier_mapping_reply_t, xcb_get_setup,
    xcb_grab_keyboard, xcb_grab_keyboard_cookie_t, xcb_grab_keyboard_reply,
    xcb_grab_keyboard_reply_t, xcb_intern_atom, xcb_intern_atom_cookie_t, xcb_intern_atom_reply,
    xcb_intern_atom_reply_t, xcb_key_press_event_t, xcb_key_press_lookup_keysym,
    xcb_key_symbols_alloc, xcb_key_symbols_get_keysym, xcb_key_symbols_t, xcb_keycode_t,
    xcb_keysym_t, xcb_map_window, xcb_query_font_reply_t, xcb_screen_t, xcb_timestamp_t,
    xcb_visualid_t, xcb_visualtype_t, xcb_wait_for_event, xcb_window_t, XCB_BUTTON_PRESS,
    XCB_COPY_FROM_PARENT, XCB_CURRENT_TIME, XCB_CW_BACK_PIXEL, XCB_CW_EVENT_MASK,
    XCB_EVENT_MASK_BUTTON_PRESS, XCB_EVENT_MASK_EXPOSURE, XCB_EXPOSE, XCB_GRAB_MODE_ASYNC,
    XCB_GRAB_STATUS_SUCCESS, XCB_KEY_PRESS, XCB_MOD_MASK_LOCK, XCB_NONE, XCB_NUM_LOCK,
    XCB_PROP_MODE_REPLACE, XCB_WINDOW_CLASS_COPY_FROM_PARENT, XCB_WINDOW_CLASS_INPUT_OUTPUT,
};

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

use i3_common::native_stdio::{err, errx, stderr, stdout, vfprintf};

// Internal calls and globals use canonical Rust identities.
use i3_common::get_config_path::get_config_path;
use i3_common::get_mod_mask::get_mod_mask_for;
use i3_common::get_visualtype::get_visualtype;
use i3_common::ipc_connect::ipc_connect;
use i3_common::ipc_send_message::ipc_send_message;
use i3_common::resolve_tilde::resolve_tilde;

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
