use crate::bindings::check_for_duplicate_bindings;
use crate::config_directives::cfg_assign;
use crate::config_directives::cfg_assign_output;
use crate::config_directives::cfg_bar_binding_mode_indicator;
use crate::config_directives::cfg_bar_bindsym;
use crate::config_directives::cfg_bar_color;
use crate::config_directives::cfg_bar_color_single;
use crate::config_directives::cfg_bar_finish;
use crate::config_directives::cfg_bar_font;
use crate::config_directives::cfg_bar_height;
use crate::config_directives::cfg_bar_hidden_state;
use crate::config_directives::cfg_bar_i3bar_command;
use crate::config_directives::cfg_bar_id;
use crate::config_directives::cfg_bar_mode;
use crate::config_directives::cfg_bar_modifier;
use crate::config_directives::cfg_bar_output;
use crate::config_directives::cfg_bar_padding_four;
use crate::config_directives::cfg_bar_padding_one;
use crate::config_directives::cfg_bar_padding_three;
use crate::config_directives::cfg_bar_padding_two;
use crate::config_directives::cfg_bar_position;
use crate::config_directives::cfg_bar_separator_symbol;
use crate::config_directives::cfg_bar_socket_path;
use crate::config_directives::cfg_bar_start;
use crate::config_directives::cfg_bar_status_command;
use crate::config_directives::cfg_bar_strip_workspace_name;
use crate::config_directives::cfg_bar_strip_workspace_numbers;
use crate::config_directives::cfg_bar_tray_output;
use crate::config_directives::cfg_bar_tray_padding;
use crate::config_directives::cfg_bar_verbose;
use crate::config_directives::cfg_bar_wheel_down_cmd;
use crate::config_directives::cfg_bar_wheel_up_cmd;
use crate::config_directives::cfg_bar_workspace_buttons;
use crate::config_directives::cfg_bar_workspace_command;
use crate::config_directives::cfg_bar_workspace_min_width;
use crate::config_directives::cfg_binding;
use crate::config_directives::cfg_color;
use crate::config_directives::cfg_color_single;
use crate::config_directives::cfg_criteria_add;
use crate::config_directives::cfg_criteria_init;
use crate::config_directives::cfg_criteria_pop_state;
use crate::config_directives::cfg_default_border;
use crate::config_directives::cfg_default_orientation;
use crate::config_directives::cfg_disable_randr15;
use crate::config_directives::cfg_enter_mode;
use crate::config_directives::cfg_exec;
use crate::config_directives::cfg_fake_outputs;
use crate::config_directives::cfg_floating_maximum_size;
use crate::config_directives::cfg_floating_minimum_size;
use crate::config_directives::cfg_floating_modifier;
use crate::config_directives::cfg_focus_follows_mouse;
use crate::config_directives::cfg_focus_on_window_activation;
use crate::config_directives::cfg_focus_wrapping;
use crate::config_directives::cfg_font;
use crate::config_directives::cfg_for_window;
use crate::config_directives::cfg_force_display_urgency_hint;
use crate::config_directives::cfg_force_focus_wrapping;
use crate::config_directives::cfg_force_xinerama;
use crate::config_directives::cfg_gaps;
use crate::config_directives::cfg_hide_edge_borders;
use crate::config_directives::cfg_include;
use crate::config_directives::cfg_ipc_kill_timeout;
use crate::config_directives::cfg_ipc_socket;
use crate::config_directives::cfg_mode_binding;
use crate::config_directives::cfg_mouse_warping;
use crate::config_directives::cfg_no_focus;
use crate::config_directives::cfg_popup_during_fullscreen;
use crate::config_directives::cfg_restart_state;
use crate::config_directives::cfg_show_marks;
use crate::config_directives::cfg_smart_borders;
use crate::config_directives::cfg_smart_gaps;
use crate::config_directives::cfg_tiling_drag;
use crate::config_directives::cfg_tiling_drag_swap_modifier;
use crate::config_directives::cfg_title_align;
use crate::config_directives::cfg_workspace;
use crate::config_directives::cfg_workspace_back_and_forth;
use crate::config_directives::cfg_workspace_layout;
use crate::types::ParseFileResult;
use std::os::unix::ffi::{OsStrExt, OsStringExt};
/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * config_parser.c: handwritten parser to parse configuration directives.
 *
 * See also src/commands_parser.c for rationale on why we use a custom parser.
 *
 * This parser works VERY MUCH like src/commands_parser.c, so read that first.
 * The differences are:
 *
 * 1. config_parser supports the 'number' token type (in addition to 'word' and
 *    'string'). Numbers are referred to using &num (like $str).
 *
 * 2. Criteria are not executed immediately, they are just stored.
 *
 * 3. config_parser recognizes \n and \r as 'end' token, while commands_parser
 *    ignores them.
 *
 * 4. config_parser skips the current line on invalid inputs and follows the
 *    nearest <error> token.
 *
 */
// Mechanical file-by-file Rust translation of src/config_parser.c.
// Originally translated with C2Rust 0.22.1; subsequently maintained in Rust.

pub use crate::config::IncludedFile;

pub use crate::bindings::context;
pub use crate::config::Variable;

pub use crate::commands::stack;
pub use crate::config::parser_ctx;
pub use crate::config::variables_head;
pub use crate::config_directives::ConfigResultIR;

pub type cmdp_token = token;
#[derive(Clone, Copy)]
pub struct token {
    pub name: &'static [u8],
    pub identifier: Option<&'static [u8]>,
    pub next_state: ConfigParserState,
    pub extra: ConfigTokenData,
}
pub use crate::commands_parser::ParserTokenData as ConfigTokenData;

pub const LONG_MAX: i64 = __LONG_MAX__;
pub const LONG_MIN: i64 = -__LONG_MAX__ - 1_i64;
pub const O_RDONLY: i32 = 0_i32;

pub static mut config_error_nagbar_pid: libc::pid_t = -(1 as libc::pid_t);
static tokens_FORCE_DISPLAY_URGENCY_HINT_MS: [cmdp_token; 2] = [
    token {
        name: b"'ms",
        identifier: None,
        next_state: ConfigParserState::ForceDisplayUrgencyHintMs,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"end",
        identifier: None,
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
];
static tokens_FLOATING_MAXIMUM_SIZE_HEIGHT: [cmdp_token; 1] = [token {
    name: b"number",
    identifier: Some(b"height"),
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 1_i32 as u16,
    },
}];
static tokens_FLOATING_MINIMUM_SIZE_HEIGHT: [cmdp_token; 1] = [token {
    name: b"number",
    identifier: Some(b"height"),
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 2_i32 as u16,
    },
}];
static tokens_BAR_STRIP_WORKSPACE_NUMBERS: [cmdp_token; 1] = [token {
    name: b"word",
    identifier: Some(b"value"),
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 3_i32 as u16,
    },
}];
static tokens_FLOATING_MAXIMUM_SIZE_WIDTH: [cmdp_token; 1] = [token {
    name: b"number",
    identifier: Some(b"width"),
    next_state: ConfigParserState::FloatingMaximumSizeX,
    extra: ConfigTokenData {
        call_identifier: 0_i32 as u16,
    },
}];
static tokens_FLOATING_MINIMUM_SIZE_WIDTH: [cmdp_token; 1] = [token {
    name: b"number",
    identifier: Some(b"width"),
    next_state: ConfigParserState::FloatingMinimumSizeX,
    extra: ConfigTokenData {
        call_identifier: 0_i32 as u16,
    },
}];
static tokens_BAR_BINDING_MODE_INDICATOR: [cmdp_token; 1] = [token {
    name: b"word",
    identifier: Some(b"value"),
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 4_i32 as u16,
    },
}];
static tokens_BAR_WORKSPACE_MIN_WIDTH_PX: [cmdp_token; 2] = [
    token {
        name: b"'px",
        identifier: None,
        next_state: ConfigParserState::BarWorkspaceMinWidthPx,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"end",
        identifier: None,
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 5_i32 as u16,
        },
    },
];
static tokens_FOCUS_ON_WINDOW_ACTIVATION: [cmdp_token; 1] = [token {
    name: b"word",
    identifier: Some(b"mode"),
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 6_i32 as u16,
    },
}];
static tokens_FORCE_DISPLAY_URGENCY_HINT: [cmdp_token; 1] = [token {
    name: b"number",
    identifier: Some(b"duration_ms"),
    next_state: ConfigParserState::ForceDisplayUrgencyHintMs,
    extra: ConfigTokenData {
        call_identifier: 0_i32 as u16,
    },
}];
static tokens_TILING_DRAG_SWAP_MODIFIER: [cmdp_token; 10] = [
    token {
        name: b"'Mod1",
        identifier: Some(b"modifiers"),
        next_state: ConfigParserState::TilingDragSwapModifier,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'Mod2",
        identifier: Some(b"modifiers"),
        next_state: ConfigParserState::TilingDragSwapModifier,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'Mod3",
        identifier: Some(b"modifiers"),
        next_state: ConfigParserState::TilingDragSwapModifier,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'Mod4",
        identifier: Some(b"modifiers"),
        next_state: ConfigParserState::TilingDragSwapModifier,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'Mod5",
        identifier: Some(b"modifiers"),
        next_state: ConfigParserState::TilingDragSwapModifier,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'Shift",
        identifier: Some(b"modifiers"),
        next_state: ConfigParserState::TilingDragSwapModifier,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'Control",
        identifier: Some(b"modifiers"),
        next_state: ConfigParserState::TilingDragSwapModifier,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'Ctrl",
        identifier: Some(b"modifiers"),
        next_state: ConfigParserState::TilingDragSwapModifier,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'+",
        identifier: None,
        next_state: ConfigParserState::TilingDragSwapModifier,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"end",
        identifier: None,
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 7_i32 as u16,
        },
    },
];
static tokens_BAR_STRIP_WORKSPACE_NAME: [cmdp_token; 1] = [token {
    name: b"word",
    identifier: Some(b"value"),
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 8_i32 as u16,
    },
}];
static tokens_CRITERION_FROM_STR_START: [cmdp_token; 3] = [
    token {
        name: b"'\"",
        identifier: None,
        next_state: ConfigParserState::CriterionFromStr,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'auto",
        identifier: Some(b"kind"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 9_i32 as u16,
        },
    },
    token {
        name: b"'user",
        identifier: Some(b"kind"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 10_i32 as u16,
        },
    },
];
static tokens_DEFAULT_BORDER_PIXELS_PX: [cmdp_token; 2] = [
    token {
        name: b"'px",
        identifier: None,
        next_state: ConfigParserState::DefaultBorderPixelsPx,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"end",
        identifier: None,
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 11_i32 as u16,
        },
    },
];
static tokens_WORKSPACE_BACK_AND_FORTH: [cmdp_token; 1] = [token {
    name: b"word",
    identifier: Some(b"value"),
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 12_i32 as u16,
    },
}];
static tokens_ASSIGN_WORKSPACE_NUMBER: [cmdp_token; 1] = [token {
    name: b"string",
    identifier: Some(b"number"),
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 13_i32 as u16,
    },
}];
static tokens_BAR_WORKSPACE_MIN_WIDTH: [cmdp_token; 1] = [token {
    name: b"number",
    identifier: Some(b"width"),
    next_state: ConfigParserState::BarWorkspaceMinWidthPx,
    extra: ConfigTokenData {
        call_identifier: 0_i32 as u16,
    },
}];
static tokens_FLOATING_MAXIMUM_SIZE_X: [cmdp_token; 1] = [token {
    name: b"'x",
    identifier: None,
    next_state: ConfigParserState::FloatingMaximumSizeHeight,
    extra: ConfigTokenData {
        call_identifier: 0_i32 as u16,
    },
}];
static tokens_FLOATING_MINIMUM_SIZE_X: [cmdp_token; 1] = [token {
    name: b"'x",
    identifier: None,
    next_state: ConfigParserState::FloatingMinimumSizeHeight,
    extra: ConfigTokenData {
        call_identifier: 0_i32 as u16,
    },
}];
static tokens_POPUP_DURING_FULLSCREEN: [cmdp_token; 4] = [
    token {
        name: b"'ignore",
        identifier: Some(b"value"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 14_i32 as u16,
        },
    },
    token {
        name: b"'leave_fullscreen",
        identifier: Some(b"value"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 15_i32 as u16,
        },
    },
    token {
        name: b"'all",
        identifier: Some(b"value"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 16_i32 as u16,
        },
    },
    token {
        name: b"'smart",
        identifier: Some(b"value"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 17_i32 as u16,
        },
    },
];
static tokens_BAR_COLORS_IGNORE_LINE: [cmdp_token; 1] = [token {
    name: b"line",
    identifier: None,
    next_state: ConfigParserState::BarColors,
    extra: ConfigTokenData {
        call_identifier: 0_i32 as u16,
    },
}];
static tokens_CRITERION_FROM_STR_END: [cmdp_token; 1] = [token {
    name: b"'\"",
    identifier: None,
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 18_i32 as u16,
    },
}];
static tokens_BAR_COLORS_BACKGROUND: [cmdp_token; 1] = [token {
    name: b"word",
    identifier: Some(b"background"),
    next_state: ConfigParserState::BarColorsText,
    extra: ConfigTokenData {
        call_identifier: 0_i32 as u16,
    },
}];
static tokens_BAR_WORKSPACE_BUTTONS: [cmdp_token; 1] = [token {
    name: b"word",
    identifier: Some(b"value"),
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 19_i32 as u16,
    },
}];
static tokens_BAR_WORKSPACE_COMMAND: [cmdp_token; 1] = [token {
    name: b"string",
    identifier: Some(b"command"),
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 20_i32 as u16,
    },
}];
static tokens_DEFAULT_BORDER_PIXELS: [cmdp_token; 2] = [
    token {
        name: b"end",
        identifier: None,
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 21_i32 as u16,
        },
    },
    token {
        name: b"number",
        identifier: Some(b"width"),
        next_state: ConfigParserState::DefaultBorderPixelsPx,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
];
static tokens_WORKSPACE_OUTPUT_WORD: [cmdp_token; 2] = [
    token {
        name: b"word",
        identifier: Some(b"output"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 22_i32 as u16,
        },
    },
    token {
        name: b"end",
        identifier: None,
        next_state: ConfigParserState::Initial,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
];
static tokens_BAR_SEPARATOR_SYMBOL: [cmdp_token; 1] = [token {
    name: b"string",
    identifier: Some(b"separator"),
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 23_i32 as u16,
    },
}];
static tokens_FORCE_FOCUS_WRAPPING: [cmdp_token; 1] = [token {
    name: b"word",
    identifier: Some(b"value"),
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 24_i32 as u16,
    },
}];
static tokens_BAR_BINDSYM_COMMAND: [cmdp_token; 2] = [
    token {
        name: b"'--release",
        identifier: Some(b"release"),
        next_state: ConfigParserState::BarBindsymCommand,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"string",
        identifier: Some(b"command"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 25_i32 as u16,
        },
    },
];
static tokens_BAR_TRAY_PADDING_PX: [cmdp_token; 2] = [
    token {
        name: b"'px",
        identifier: None,
        next_state: ConfigParserState::BarTrayPaddingPx,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"end",
        identifier: None,
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 26_i32 as u16,
        },
    },
];
static tokens_DEFAULT_ORIENTATION: [cmdp_token; 3] = [
    token {
        name: b"'horizontal",
        identifier: Some(b"orientation"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 27_i32 as u16,
        },
    },
    token {
        name: b"'vertical",
        identifier: Some(b"orientation"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 28_i32 as u16,
        },
    },
    token {
        name: b"'auto",
        identifier: Some(b"orientation"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 29_i32 as u16,
        },
    },
];
static tokens_FOCUS_FOLLOWS_MOUSE: [cmdp_token; 1] = [token {
    name: b"word",
    identifier: Some(b"value"),
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 30_i32 as u16,
    },
}];
static tokens_BAR_PADDING_BOTTOM: [cmdp_token; 3] = [
    token {
        name: b"'px",
        identifier: None,
        next_state: ConfigParserState::BarPaddingBottom,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"number",
        identifier: Some(b"left"),
        next_state: ConfigParserState::BarPaddingLeft,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"end",
        identifier: None,
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 31_i32 as u16,
        },
    },
];
static tokens_BAR_STATUS_COMMAND: [cmdp_token; 1] = [token {
    name: b"string",
    identifier: Some(b"command"),
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 32_i32 as u16,
    },
}];
static tokens_BAR_WHEEL_DOWN_CMD: [cmdp_token; 1] = [token {
    name: b"string",
    identifier: Some(b"command"),
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 33_i32 as u16,
    },
}];
static tokens_COLOR_CHILD_BORDER: [cmdp_token; 2] = [
    token {
        name: b"word",
        identifier: Some(b"child_border"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 34_i32 as u16,
        },
    },
    token {
        name: b"end",
        identifier: None,
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 35_i32 as u16,
        },
    },
];
static tokens_CRITERION_FROM_STR: [cmdp_token; 2] = [
    token {
        name: b"'auto",
        identifier: Some(b"kind"),
        next_state: ConfigParserState::CriterionFromStrEnd,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'user",
        identifier: Some(b"kind"),
        next_state: ConfigParserState::CriterionFromStrEnd,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
];
static tokens_FOR_WINDOW_COMMAND: [cmdp_token; 1] = [token {
    name: b"string",
    identifier: Some(b"command"),
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 36_i32 as u16,
    },
}];
static tokens_BAR_COLORS_BORDER: [cmdp_token; 1] = [token {
    name: b"word",
    identifier: Some(b"border"),
    next_state: ConfigParserState::BarColorsBackground,
    extra: ConfigTokenData {
        call_identifier: 0_i32 as u16,
    },
}];
static tokens_BAR_COLORS_SINGLE: [cmdp_token; 1] = [token {
    name: b"word",
    identifier: Some(b"color"),
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 37_i32 as u16,
    },
}];
static tokens_BAR_PADDING_RIGHT: [cmdp_token; 3] = [
    token {
        name: b"'px",
        identifier: None,
        next_state: ConfigParserState::BarPaddingRight,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"number",
        identifier: Some(b"bottom"),
        next_state: ConfigParserState::BarPaddingBottom,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"end",
        identifier: None,
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 38_i32 as u16,
        },
    },
];
static tokens_FLOATING_MODIFIER: [cmdp_token; 10] = [
    token {
        name: b"'Mod1",
        identifier: Some(b"modifiers"),
        next_state: ConfigParserState::FloatingModifier,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'Mod2",
        identifier: Some(b"modifiers"),
        next_state: ConfigParserState::FloatingModifier,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'Mod3",
        identifier: Some(b"modifiers"),
        next_state: ConfigParserState::FloatingModifier,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'Mod4",
        identifier: Some(b"modifiers"),
        next_state: ConfigParserState::FloatingModifier,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'Mod5",
        identifier: Some(b"modifiers"),
        next_state: ConfigParserState::FloatingModifier,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'Shift",
        identifier: Some(b"modifiers"),
        next_state: ConfigParserState::FloatingModifier,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'Control",
        identifier: Some(b"modifiers"),
        next_state: ConfigParserState::FloatingModifier,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'Ctrl",
        identifier: Some(b"modifiers"),
        next_state: ConfigParserState::FloatingModifier,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'+",
        identifier: None,
        next_state: ConfigParserState::FloatingModifier,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"end",
        identifier: None,
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 39_i32 as u16,
        },
    },
];
static tokens_HIDE_EDGE_BORDERS: [cmdp_token; 12] = [
    token {
        name: b"'none",
        identifier: Some(b"hide_borders"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 40_i32 as u16,
        },
    },
    token {
        name: b"'vertical",
        identifier: Some(b"hide_borders"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 41_i32 as u16,
        },
    },
    token {
        name: b"'horizontal",
        identifier: Some(b"hide_borders"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 42_i32 as u16,
        },
    },
    token {
        name: b"'both",
        identifier: Some(b"hide_borders"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 43_i32 as u16,
        },
    },
    token {
        name: b"'smart_no_gaps",
        identifier: Some(b"hide_borders"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 44_i32 as u16,
        },
    },
    token {
        name: b"'smart",
        identifier: Some(b"hide_borders"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 45_i32 as u16,
        },
    },
    token {
        name: b"'1",
        identifier: Some(b"hide_borders"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 46_i32 as u16,
        },
    },
    token {
        name: b"'yes",
        identifier: Some(b"hide_borders"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 47_i32 as u16,
        },
    },
    token {
        name: b"'true",
        identifier: Some(b"hide_borders"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 48_i32 as u16,
        },
    },
    token {
        name: b"'on",
        identifier: Some(b"hide_borders"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 49_i32 as u16,
        },
    },
    token {
        name: b"'enable",
        identifier: Some(b"hide_borders"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 50_i32 as u16,
        },
    },
    token {
        name: b"'active",
        identifier: Some(b"hide_borders"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 51_i32 as u16,
        },
    },
];
static tokens_WORKSPACE_COMMAND: [cmdp_token; 2] = [
    token {
        name: b"'output",
        identifier: None,
        next_state: ConfigParserState::WorkspaceOutputWord,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'gaps",
        identifier: None,
        next_state: ConfigParserState::Gaps,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
];
static tokens_ASSIGN_WORKSPACE: [cmdp_token; 5] = [
    token {
        name: "'→".as_bytes(),
        identifier: None,
        next_state: ConfigParserState::AssignWorkspace,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'output",
        identifier: None,
        next_state: ConfigParserState::AssignOutput,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'workspace",
        identifier: None,
        next_state: ConfigParserState::AssignWorkspace,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'number",
        identifier: None,
        next_state: ConfigParserState::AssignWorkspaceNumber,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"string",
        identifier: Some(b"workspace"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 52_i32 as u16,
        },
    },
];
static tokens_BAR_COLORS_BRACE: [cmdp_token; 2] = [
    token {
        name: b"end",
        identifier: None,
        next_state: ConfigParserState::BarColorsBrace,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'{",
        identifier: None,
        next_state: ConfigParserState::BarColors,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
];
static tokens_BAR_HIDDEN_STATE: [cmdp_token; 2] = [
    token {
        name: b"'hide",
        identifier: Some(b"hidden_state"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 53_i32 as u16,
        },
    },
    token {
        name: b"'show",
        identifier: Some(b"hidden_state"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 54_i32 as u16,
        },
    },
];
static tokens_BAR_PADDING_LEFT: [cmdp_token; 2] = [
    token {
        name: b"'px",
        identifier: None,
        next_state: ConfigParserState::BarPaddingLeft,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"end",
        identifier: None,
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 55_i32 as u16,
        },
    },
];
static tokens_BAR_TRAY_PADDING: [cmdp_token; 1] = [token {
    name: b"number",
    identifier: Some(b"padding_px"),
    next_state: ConfigParserState::BarTrayPaddingPx,
    extra: ConfigTokenData {
        call_identifier: 0_i32 as u16,
    },
}];
static tokens_BAR_WHEEL_UP_CMD: [cmdp_token; 1] = [token {
    name: b"string",
    identifier: Some(b"command"),
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 56_i32 as u16,
    },
}];
static tokens_COLOR_BACKGROUND: [cmdp_token; 1] = [token {
    name: b"word",
    identifier: Some(b"background"),
    next_state: ConfigParserState::ColorText,
    extra: ConfigTokenData {
        call_identifier: 0_i32 as u16,
    },
}];
static tokens_IPC_KILL_TIMEOUT: [cmdp_token; 1] = [token {
    name: b"number",
    identifier: Some(b"timeout"),
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 57_i32 as u16,
    },
}];
static tokens_MODE_BINDCOMMAND: [cmdp_token; 5] = [
    token {
        name: b"'--release",
        identifier: Some(b"release"),
        next_state: ConfigParserState::ModeBindcommand,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'--border",
        identifier: Some(b"border"),
        next_state: ConfigParserState::ModeBindcommand,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'--whole-window",
        identifier: Some(b"whole_window"),
        next_state: ConfigParserState::ModeBindcommand,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'--exclude-titlebar",
        identifier: Some(b"exclude_titlebar"),
        next_state: ConfigParserState::ModeBindcommand,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"string",
        identifier: Some(b"command"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 58_i32 as u16,
        },
    },
];
static tokens_MODE_IGNORE_LINE: [cmdp_token; 1] = [token {
    name: b"line",
    identifier: None,
    next_state: ConfigParserState::Mode,
    extra: ConfigTokenData {
        call_identifier: 0_i32 as u16,
    },
}];
static tokens_TILING_DRAG_MODE: [cmdp_token; 3] = [
    token {
        name: b"'modifier",
        identifier: Some(b"value"),
        next_state: ConfigParserState::TilingDragMode,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'titlebar",
        identifier: Some(b"value"),
        next_state: ConfigParserState::TilingDragMode,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"end",
        identifier: None,
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 59_i32 as u16,
        },
    },
];
static tokens_WORKSPACE_LAYOUT: [cmdp_token; 4] = [
    token {
        name: b"'default",
        identifier: Some(b"layout"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 60_i32 as u16,
        },
    },
    token {
        name: b"'stacking",
        identifier: Some(b"layout"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 61_i32 as u16,
        },
    },
    token {
        name: b"'stacked",
        identifier: Some(b"layout"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 62_i32 as u16,
        },
    },
    token {
        name: b"'tabbed",
        identifier: Some(b"layout"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 63_i32 as u16,
        },
    },
];
static tokens_BAR_BAR_COMMAND: [cmdp_token; 1] = [token {
    name: b"string",
    identifier: Some(b"command"),
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 64_i32 as u16,
    },
}];
static tokens_BAR_COLORS_TEXT: [cmdp_token; 2] = [
    token {
        name: b"end",
        identifier: None,
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 65_i32 as u16,
        },
    },
    token {
        name: b"word",
        identifier: Some(b"text"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 66_i32 as u16,
        },
    },
];
static tokens_BAR_IGNORE_LINE: [cmdp_token; 1] = [token {
    name: b"line",
    identifier: None,
    next_state: ConfigParserState::Bar,
    extra: ConfigTokenData {
        call_identifier: 0_i32 as u16,
    },
}];
static tokens_BAR_PADDING_TOP: [cmdp_token; 3] = [
    token {
        name: b"'px",
        identifier: None,
        next_state: ConfigParserState::BarPaddingTop,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"number",
        identifier: Some(b"right_or_right_and_left"),
        next_state: ConfigParserState::BarPaddingRight,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"end",
        identifier: None,
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 67_i32 as u16,
        },
    },
];
static tokens_BAR_SOCKET_PATH: [cmdp_token; 1] = [token {
    name: b"string",
    identifier: Some(b"path"),
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 68_i32 as u16,
    },
}];
static tokens_BAR_TRAY_OUTPUT: [cmdp_token; 1] = [token {
    name: b"word",
    identifier: Some(b"output"),
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 69_i32 as u16,
    },
}];
static tokens_COLOR_INDICATOR: [cmdp_token; 2] = [
    token {
        name: b"word",
        identifier: Some(b"indicator"),
        next_state: ConfigParserState::ColorChildBorder,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"end",
        identifier: None,
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 70_i32 as u16,
        },
    },
];
static tokens_DISABLE_RANDR15: [cmdp_token; 1] = [token {
    name: b"word",
    identifier: Some(b"value"),
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 71_i32 as u16,
    },
}];
static tokens_GAPS_WITH_SCOPE: [cmdp_token; 1] = [token {
    name: b"number",
    identifier: Some(b"value"),
    next_state: ConfigParserState::GapsEnd,
    extra: ConfigTokenData {
        call_identifier: 0_i32 as u16,
    },
}];
static tokens_CRITERION_FROM: [cmdp_token; 1] = [token {
    name: b"'=",
    identifier: None,
    next_state: ConfigParserState::CriterionFromStrStart,
    extra: ConfigTokenData {
        call_identifier: 0_i32 as u16,
    },
}];
static tokens_DEFAULT_BORDER: [cmdp_token; 4] = [
    token {
        name: b"'normal",
        identifier: Some(b"border"),
        next_state: ConfigParserState::DefaultBorderPixels,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'pixel",
        identifier: Some(b"border"),
        next_state: ConfigParserState::DefaultBorderPixels,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'1pixel",
        identifier: Some(b"border"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 72_i32 as u16,
        },
    },
    token {
        name: b"'none",
        identifier: Some(b"border"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 73_i32 as u16,
        },
    },
];
static tokens_FOCUS_WRAPPING: [cmdp_token; 14] = [
    token {
        name: b"'1",
        identifier: Some(b"value"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 74_i32 as u16,
        },
    },
    token {
        name: b"'yes",
        identifier: Some(b"value"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 75_i32 as u16,
        },
    },
    token {
        name: b"'true",
        identifier: Some(b"value"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 76_i32 as u16,
        },
    },
    token {
        name: b"'on",
        identifier: Some(b"value"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 77_i32 as u16,
        },
    },
    token {
        name: b"'enable",
        identifier: Some(b"value"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 78_i32 as u16,
        },
    },
    token {
        name: b"'active",
        identifier: Some(b"value"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 79_i32 as u16,
        },
    },
    token {
        name: b"'0",
        identifier: Some(b"value"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 80_i32 as u16,
        },
    },
    token {
        name: b"'no",
        identifier: Some(b"value"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 81_i32 as u16,
        },
    },
    token {
        name: b"'false",
        identifier: Some(b"value"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 82_i32 as u16,
        },
    },
    token {
        name: b"'off",
        identifier: Some(b"value"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 83_i32 as u16,
        },
    },
    token {
        name: b"'disable",
        identifier: Some(b"value"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 84_i32 as u16,
        },
    },
    token {
        name: b"'inactive",
        identifier: Some(b"value"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 85_i32 as u16,
        },
    },
    token {
        name: b"'force",
        identifier: Some(b"value"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 86_i32 as u16,
        },
    },
    token {
        name: b"'workspace",
        identifier: Some(b"value"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 87_i32 as u16,
        },
    },
];
static tokens_FORCE_XINERAMA: [cmdp_token; 1] = [token {
    name: b"word",
    identifier: Some(b"value"),
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 88_i32 as u16,
    },
}];
static tokens_ASSIGN_OUTPUT: [cmdp_token; 1] = [token {
    name: b"string",
    identifier: Some(b"output"),
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 89_i32 as u16,
    },
}];
static tokens_CRITERION_STR: [cmdp_token; 1] = [token {
    name: b"word",
    identifier: Some(b"cvalue"),
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 90_i32 as u16,
    },
}];
static tokens_MOUSE_WARPING: [cmdp_token; 2] = [
    token {
        name: b"'none",
        identifier: Some(b"value"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 91_i32 as u16,
        },
    },
    token {
        name: b"'output",
        identifier: Some(b"value"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 92_i32 as u16,
        },
    },
];
static tokens_RESTART_STATE: [cmdp_token; 1] = [token {
    name: b"string",
    identifier: Some(b"path"),
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 93_i32 as u16,
    },
}];
static tokens_SMART_BORDERS: [cmdp_token; 7] = [
    token {
        name: b"'1",
        identifier: Some(b"enabled"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 94_i32 as u16,
        },
    },
    token {
        name: b"'yes",
        identifier: Some(b"enabled"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 95_i32 as u16,
        },
    },
    token {
        name: b"'true",
        identifier: Some(b"enabled"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 96_i32 as u16,
        },
    },
    token {
        name: b"'on",
        identifier: Some(b"enabled"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 97_i32 as u16,
        },
    },
    token {
        name: b"'enable",
        identifier: Some(b"enabled"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 98_i32 as u16,
        },
    },
    token {
        name: b"'active",
        identifier: Some(b"enabled"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 99_i32 as u16,
        },
    },
    token {
        name: b"'no_gaps",
        identifier: Some(b"enabled"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 100_i32 as u16,
        },
    },
];
static tokens_BAR_MODIFIER: [cmdp_token; 12] = [
    token {
        name: b"'off",
        identifier: None,
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 101_i32 as u16,
        },
    },
    token {
        name: b"'none",
        identifier: None,
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 102_i32 as u16,
        },
    },
    token {
        name: b"'Mod1",
        identifier: Some(b"modifiers"),
        next_state: ConfigParserState::BarModifier,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'Mod2",
        identifier: Some(b"modifiers"),
        next_state: ConfigParserState::BarModifier,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'Mod3",
        identifier: Some(b"modifiers"),
        next_state: ConfigParserState::BarModifier,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'Mod4",
        identifier: Some(b"modifiers"),
        next_state: ConfigParserState::BarModifier,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'Mod5",
        identifier: Some(b"modifiers"),
        next_state: ConfigParserState::BarModifier,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'Shift",
        identifier: Some(b"modifiers"),
        next_state: ConfigParserState::BarModifier,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'Control",
        identifier: Some(b"modifiers"),
        next_state: ConfigParserState::BarModifier,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'Ctrl",
        identifier: Some(b"modifiers"),
        next_state: ConfigParserState::BarModifier,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'+",
        identifier: None,
        next_state: ConfigParserState::BarModifier,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"end",
        identifier: None,
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 103_i32 as u16,
        },
    },
];
static tokens_BAR_POSITION: [cmdp_token; 2] = [
    token {
        name: b"'top",
        identifier: Some(b"position"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 104_i32 as u16,
        },
    },
    token {
        name: b"'bottom",
        identifier: Some(b"position"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 105_i32 as u16,
        },
    },
];
static tokens_COLOR_BORDER: [cmdp_token; 1] = [token {
    name: b"word",
    identifier: Some(b"border"),
    next_state: ConfigParserState::ColorBackground,
    extra: ConfigTokenData {
        call_identifier: 0_i32 as u16,
    },
}];
static tokens_COLOR_SINGLE: [cmdp_token; 1] = [token {
    name: b"word",
    identifier: Some(b"color"),
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 106_i32 as u16,
    },
}];
static tokens_FAKE_OUTPUTS: [cmdp_token; 1] = [token {
    name: b"string",
    identifier: Some(b"outputs"),
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 107_i32 as u16,
    },
}];
static tokens_MODE_BINDING: [cmdp_token; 20] = [
    token {
        name: b"'--release",
        identifier: Some(b"release"),
        next_state: ConfigParserState::ModeBinding,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'--border",
        identifier: Some(b"border"),
        next_state: ConfigParserState::ModeBinding,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'--whole-window",
        identifier: Some(b"whole_window"),
        next_state: ConfigParserState::ModeBinding,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'--exclude-titlebar",
        identifier: Some(b"exclude_titlebar"),
        next_state: ConfigParserState::ModeBinding,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'Mod1",
        identifier: Some(b"modifiers"),
        next_state: ConfigParserState::ModeBinding,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'Mod2",
        identifier: Some(b"modifiers"),
        next_state: ConfigParserState::ModeBinding,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'Mod3",
        identifier: Some(b"modifiers"),
        next_state: ConfigParserState::ModeBinding,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'Mod4",
        identifier: Some(b"modifiers"),
        next_state: ConfigParserState::ModeBinding,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'Mod5",
        identifier: Some(b"modifiers"),
        next_state: ConfigParserState::ModeBinding,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'Shift",
        identifier: Some(b"modifiers"),
        next_state: ConfigParserState::ModeBinding,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'Control",
        identifier: Some(b"modifiers"),
        next_state: ConfigParserState::ModeBinding,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'Ctrl",
        identifier: Some(b"modifiers"),
        next_state: ConfigParserState::ModeBinding,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'Mode_switch",
        identifier: Some(b"modifiers"),
        next_state: ConfigParserState::ModeBinding,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'Group1",
        identifier: Some(b"modifiers"),
        next_state: ConfigParserState::ModeBinding,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'Group2",
        identifier: Some(b"modifiers"),
        next_state: ConfigParserState::ModeBinding,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'Group3",
        identifier: Some(b"modifiers"),
        next_state: ConfigParserState::ModeBinding,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'Group4",
        identifier: Some(b"modifiers"),
        next_state: ConfigParserState::ModeBinding,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'$mod",
        identifier: Some(b"modifiers"),
        next_state: ConfigParserState::ModeBinding,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'+",
        identifier: None,
        next_state: ConfigParserState::ModeBinding,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"word",
        identifier: Some(b"key"),
        next_state: ConfigParserState::ModeBindcommand,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
];
static tokens_NO_FOCUS_END: [cmdp_token; 1] = [token {
    name: b"end",
    identifier: None,
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 108_i32 as u16,
    },
}];
static tokens_BAR_BINDSYM: [cmdp_token; 2] = [
    token {
        name: b"'--release",
        identifier: Some(b"release"),
        next_state: ConfigParserState::BarBindsym,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"word",
        identifier: Some(b"button"),
        next_state: ConfigParserState::BarBindsymCommand,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
];
static tokens_BAR_PADDING: [cmdp_token; 1] = [token {
    name: b"number",
    identifier: Some(b"top_or_all"),
    next_state: ConfigParserState::BarPaddingTop,
    extra: ConfigTokenData {
        call_identifier: 0_i32 as u16,
    },
}];
static tokens_BAR_VERBOSE: [cmdp_token; 1] = [token {
    name: b"word",
    identifier: Some(b"value"),
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 109_i32 as u16,
    },
}];
static tokens_BINDCOMMAND: [cmdp_token; 5] = [
    token {
        name: b"'--release",
        identifier: Some(b"release"),
        next_state: ConfigParserState::Bindcommand,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'--border",
        identifier: Some(b"border"),
        next_state: ConfigParserState::Bindcommand,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'--whole-window",
        identifier: Some(b"whole_window"),
        next_state: ConfigParserState::Bindcommand,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'--exclude-titlebar",
        identifier: Some(b"exclude_titlebar"),
        next_state: ConfigParserState::Bindcommand,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"string",
        identifier: Some(b"command"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 110_i32 as u16,
        },
    },
];
static tokens_IGNORE_LINE: [cmdp_token; 1] = [token {
    name: b"line",
    identifier: None,
    next_state: ConfigParserState::Initial,
    extra: ConfigTokenData {
        call_identifier: 0_i32 as u16,
    },
}];
static tokens_TILING_DRAG: [cmdp_token; 9] = [
    token {
        name: b"'0",
        identifier: Some(b"off"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 111_i32 as u16,
        },
    },
    token {
        name: b"'no",
        identifier: Some(b"off"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 112_i32 as u16,
        },
    },
    token {
        name: b"'false",
        identifier: Some(b"off"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 113_i32 as u16,
        },
    },
    token {
        name: b"'off",
        identifier: Some(b"off"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 114_i32 as u16,
        },
    },
    token {
        name: b"'disable",
        identifier: Some(b"off"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 115_i32 as u16,
        },
    },
    token {
        name: b"'inactive",
        identifier: Some(b"off"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 116_i32 as u16,
        },
    },
    token {
        name: b"'swap_modifier",
        identifier: Some(b"swap_modifier"),
        next_state: ConfigParserState::TilingDragSwapModifier,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'modifier",
        identifier: Some(b"value"),
        next_state: ConfigParserState::TilingDragMode,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'titlebar",
        identifier: Some(b"value"),
        next_state: ConfigParserState::TilingDragMode,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
];
static tokens_TITLE_ALIGN: [cmdp_token; 3] = [
    token {
        name: b"'left",
        identifier: Some(b"alignment"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 117_i32 as u16,
        },
    },
    token {
        name: b"'center",
        identifier: Some(b"alignment"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 118_i32 as u16,
        },
    },
    token {
        name: b"'right",
        identifier: Some(b"alignment"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 119_i32 as u16,
        },
    },
];
static tokens_BAR_COLORS: [cmdp_token; 15] = [
    token {
        name: b"end",
        identifier: None,
        next_state: ConfigParserState::BarColors,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'#",
        identifier: None,
        next_state: ConfigParserState::BarColorsIgnoreLine,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'set",
        identifier: None,
        next_state: ConfigParserState::BarColorsIgnoreLine,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'background",
        identifier: Some(b"colorclass"),
        next_state: ConfigParserState::BarColorsSingle,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'statusline",
        identifier: Some(b"colorclass"),
        next_state: ConfigParserState::BarColorsSingle,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'separator",
        identifier: Some(b"colorclass"),
        next_state: ConfigParserState::BarColorsSingle,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'focused_background",
        identifier: Some(b"colorclass"),
        next_state: ConfigParserState::BarColorsSingle,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'focused_statusline",
        identifier: Some(b"colorclass"),
        next_state: ConfigParserState::BarColorsSingle,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'focused_separator",
        identifier: Some(b"colorclass"),
        next_state: ConfigParserState::BarColorsSingle,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'focused_workspace",
        identifier: Some(b"colorclass"),
        next_state: ConfigParserState::BarColorsBorder,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'active_workspace",
        identifier: Some(b"colorclass"),
        next_state: ConfigParserState::BarColorsBorder,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'inactive_workspace",
        identifier: Some(b"colorclass"),
        next_state: ConfigParserState::BarColorsBorder,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'urgent_workspace",
        identifier: Some(b"colorclass"),
        next_state: ConfigParserState::BarColorsBorder,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'binding_mode",
        identifier: Some(b"colorclass"),
        next_state: ConfigParserState::BarColorsBorder,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'}",
        identifier: None,
        next_state: ConfigParserState::Bar,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
];
static tokens_BAR_HEIGHT: [cmdp_token; 1] = [token {
    name: b"number",
    identifier: Some(b"value"),
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 120_i32 as u16,
    },
}];
static tokens_BAR_OUTPUT: [cmdp_token; 1] = [token {
    name: b"word",
    identifier: Some(b"output"),
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 121_i32 as u16,
    },
}];
static tokens_COLOR_TEXT: [cmdp_token; 1] = [token {
    name: b"word",
    identifier: Some(b"text"),
    next_state: ConfigParserState::ColorIndicator,
    extra: ConfigTokenData {
        call_identifier: 0_i32 as u16,
    },
}];
static tokens_FOR_WINDOW: [cmdp_token; 1] = [token {
    name: b"'[",
    identifier: None,
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 122_i32 as u16,
    },
}];
static tokens_IPC_SOCKET: [cmdp_token; 1] = [token {
    name: b"string",
    identifier: Some(b"path"),
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 123_i32 as u16,
    },
}];
static tokens_SHOW_MARKS: [cmdp_token; 1] = [token {
    name: b"word",
    identifier: Some(b"value"),
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 124_i32 as u16,
    },
}];
static tokens_SMART_GAPS: [cmdp_token; 13] = [
    token {
        name: b"'1",
        identifier: Some(b"enabled"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 125_i32 as u16,
        },
    },
    token {
        name: b"'yes",
        identifier: Some(b"enabled"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 126_i32 as u16,
        },
    },
    token {
        name: b"'true",
        identifier: Some(b"enabled"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 127_i32 as u16,
        },
    },
    token {
        name: b"'on",
        identifier: Some(b"enabled"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 128_i32 as u16,
        },
    },
    token {
        name: b"'enable",
        identifier: Some(b"enabled"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 129_i32 as u16,
        },
    },
    token {
        name: b"'active",
        identifier: Some(b"enabled"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 130_i32 as u16,
        },
    },
    token {
        name: b"'0",
        identifier: Some(b"enabled"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 131_i32 as u16,
        },
    },
    token {
        name: b"'no",
        identifier: Some(b"enabled"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 132_i32 as u16,
        },
    },
    token {
        name: b"'false",
        identifier: Some(b"enabled"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 133_i32 as u16,
        },
    },
    token {
        name: b"'off",
        identifier: Some(b"enabled"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 134_i32 as u16,
        },
    },
    token {
        name: b"'disable",
        identifier: Some(b"enabled"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 135_i32 as u16,
        },
    },
    token {
        name: b"'inactive",
        identifier: Some(b"enabled"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 136_i32 as u16,
        },
    },
    token {
        name: b"'inverse_outer",
        identifier: Some(b"enabled"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 137_i32 as u16,
        },
    },
];
static tokens_CRITERION: [cmdp_token; 1] = [token {
    name: b"'=",
    identifier: None,
    next_state: ConfigParserState::CriterionStr,
    extra: ConfigTokenData {
        call_identifier: 0_i32 as u16,
    },
}];
static tokens_MODEBRACE: [cmdp_token; 2] = [
    token {
        name: b"end",
        identifier: None,
        next_state: ConfigParserState::Modebrace,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'{",
        identifier: None,
        next_state: ConfigParserState::Mode,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
];
static tokens_WORKSPACE: [cmdp_token; 1] = [token {
    name: b"word",
    identifier: Some(b"workspace"),
    next_state: ConfigParserState::WorkspaceCommand,
    extra: ConfigTokenData {
        call_identifier: 0_i32 as u16,
    },
}];
static tokens_BARBRACE: [cmdp_token; 2] = [
    token {
        name: b"end",
        identifier: None,
        next_state: ConfigParserState::Barbrace,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'{",
        identifier: None,
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 138_i32 as u16,
        },
    },
];
static tokens_BAR_FONT: [cmdp_token; 1] = [token {
    name: b"string",
    identifier: Some(b"font"),
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 139_i32 as u16,
    },
}];
static tokens_BAR_MODE: [cmdp_token; 3] = [
    token {
        name: b"'dock",
        identifier: Some(b"mode"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 140_i32 as u16,
        },
    },
    token {
        name: b"'hide",
        identifier: Some(b"mode"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 141_i32 as u16,
        },
    },
    token {
        name: b"'invisible",
        identifier: Some(b"mode"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 142_i32 as u16,
        },
    },
];
static tokens_CRITERIA: [cmdp_token; 17] = [
    token {
        name: b"'class",
        identifier: Some(b"ctype"),
        next_state: ConfigParserState::Criterion,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'instance",
        identifier: Some(b"ctype"),
        next_state: ConfigParserState::Criterion,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'window_role",
        identifier: Some(b"ctype"),
        next_state: ConfigParserState::Criterion,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'con_id",
        identifier: Some(b"ctype"),
        next_state: ConfigParserState::Criterion,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'id",
        identifier: Some(b"ctype"),
        next_state: ConfigParserState::Criterion,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'window_type",
        identifier: Some(b"ctype"),
        next_state: ConfigParserState::Criterion,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'con_mark",
        identifier: Some(b"ctype"),
        next_state: ConfigParserState::Criterion,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'title",
        identifier: Some(b"ctype"),
        next_state: ConfigParserState::Criterion,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'urgent",
        identifier: Some(b"ctype"),
        next_state: ConfigParserState::Criterion,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'workspace",
        identifier: Some(b"ctype"),
        next_state: ConfigParserState::Criterion,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'machine",
        identifier: Some(b"ctype"),
        next_state: ConfigParserState::Criterion,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'floating_from",
        identifier: Some(b"ctype"),
        next_state: ConfigParserState::CriterionFrom,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'tiling_from",
        identifier: Some(b"ctype"),
        next_state: ConfigParserState::CriterionFrom,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'tiling",
        identifier: Some(b"ctype"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 143_i32 as u16,
        },
    },
    token {
        name: b"'floating",
        identifier: Some(b"ctype"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 144_i32 as u16,
        },
    },
    token {
        name: b"'all",
        identifier: Some(b"ctype"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 145_i32 as u16,
        },
    },
    token {
        name: b"']",
        identifier: None,
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 146_i32 as u16,
        },
    },
];
static tokens_GAPS_END: [cmdp_token; 2] = [
    token {
        name: b"'px",
        identifier: None,
        next_state: ConfigParserState::GapsEnd,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"end",
        identifier: None,
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 147_i32 as u16,
        },
    },
];
static tokens_MODENAME: [cmdp_token; 2] = [
    token {
        name: b"'--pango_markup",
        identifier: Some(b"pango_markup"),
        next_state: ConfigParserState::Modename,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"word",
        identifier: Some(b"modename"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 148_i32 as u16,
        },
    },
];
static tokens_NO_FOCUS: [cmdp_token; 1] = [token {
    name: b"'[",
    identifier: None,
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 149_i32 as u16,
    },
}];
static tokens_BINDING: [cmdp_token; 20] = [
    token {
        name: b"'--release",
        identifier: Some(b"release"),
        next_state: ConfigParserState::Binding,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'--border",
        identifier: Some(b"border"),
        next_state: ConfigParserState::Binding,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'--whole-window",
        identifier: Some(b"whole_window"),
        next_state: ConfigParserState::Binding,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'--exclude-titlebar",
        identifier: Some(b"exclude_titlebar"),
        next_state: ConfigParserState::Binding,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'Mod1",
        identifier: Some(b"modifiers"),
        next_state: ConfigParserState::Binding,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'Mod2",
        identifier: Some(b"modifiers"),
        next_state: ConfigParserState::Binding,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'Mod3",
        identifier: Some(b"modifiers"),
        next_state: ConfigParserState::Binding,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'Mod4",
        identifier: Some(b"modifiers"),
        next_state: ConfigParserState::Binding,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'Mod5",
        identifier: Some(b"modifiers"),
        next_state: ConfigParserState::Binding,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'Shift",
        identifier: Some(b"modifiers"),
        next_state: ConfigParserState::Binding,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'Control",
        identifier: Some(b"modifiers"),
        next_state: ConfigParserState::Binding,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'Ctrl",
        identifier: Some(b"modifiers"),
        next_state: ConfigParserState::Binding,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'Mode_switch",
        identifier: Some(b"modifiers"),
        next_state: ConfigParserState::Binding,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'Group1",
        identifier: Some(b"modifiers"),
        next_state: ConfigParserState::Binding,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'Group2",
        identifier: Some(b"modifiers"),
        next_state: ConfigParserState::Binding,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'Group3",
        identifier: Some(b"modifiers"),
        next_state: ConfigParserState::Binding,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'Group4",
        identifier: Some(b"modifiers"),
        next_state: ConfigParserState::Binding,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'$mod",
        identifier: Some(b"modifiers"),
        next_state: ConfigParserState::Binding,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'+",
        identifier: None,
        next_state: ConfigParserState::Binding,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"word",
        identifier: Some(b"key"),
        next_state: ConfigParserState::Bindcommand,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
];
static tokens_INCLUDE: [cmdp_token; 1] = [token {
    name: b"string",
    identifier: Some(b"pattern"),
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 150_i32 as u16,
    },
}];
static tokens_INITIAL: [cmdp_token; 60] = [
    token {
        name: b"end",
        identifier: None,
        next_state: ConfigParserState::Initial,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"error",
        identifier: None,
        next_state: ConfigParserState::Initial,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'#",
        identifier: None,
        next_state: ConfigParserState::IgnoreLine,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'set ",
        identifier: None,
        next_state: ConfigParserState::IgnoreLine,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'set\t",
        identifier: None,
        next_state: ConfigParserState::IgnoreLine,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'set_from_resource",
        identifier: None,
        next_state: ConfigParserState::IgnoreLine,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'include",
        identifier: None,
        next_state: ConfigParserState::Include,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'bindsym",
        identifier: Some(b"bindtype"),
        next_state: ConfigParserState::Binding,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'bindcode",
        identifier: Some(b"bindtype"),
        next_state: ConfigParserState::Binding,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'bind",
        identifier: Some(b"bindtype"),
        next_state: ConfigParserState::Binding,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'bar",
        identifier: None,
        next_state: ConfigParserState::Barbrace,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'font",
        identifier: None,
        next_state: ConfigParserState::Font,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'mode",
        identifier: None,
        next_state: ConfigParserState::Modename,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'gaps",
        identifier: None,
        next_state: ConfigParserState::Gaps,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'smart_borders",
        identifier: None,
        next_state: ConfigParserState::SmartBorders,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'smart_gaps",
        identifier: None,
        next_state: ConfigParserState::SmartGaps,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'floating_minimum_size",
        identifier: None,
        next_state: ConfigParserState::FloatingMinimumSizeWidth,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'floating_maximum_size",
        identifier: None,
        next_state: ConfigParserState::FloatingMaximumSizeWidth,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'floating_modifier",
        identifier: None,
        next_state: ConfigParserState::FloatingModifier,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'default_orientation",
        identifier: None,
        next_state: ConfigParserState::DefaultOrientation,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'workspace_layout",
        identifier: None,
        next_state: ConfigParserState::WorkspaceLayout,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'default_border",
        identifier: Some(b"windowtype"),
        next_state: ConfigParserState::DefaultBorder,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'new_window",
        identifier: Some(b"windowtype"),
        next_state: ConfigParserState::DefaultBorder,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'default_floating_border",
        identifier: Some(b"windowtype"),
        next_state: ConfigParserState::DefaultBorder,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'new_float",
        identifier: Some(b"windowtype"),
        next_state: ConfigParserState::DefaultBorder,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'hide_edge_borders",
        identifier: None,
        next_state: ConfigParserState::HideEdgeBorders,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'for_window",
        identifier: None,
        next_state: ConfigParserState::ForWindow,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'assign",
        identifier: None,
        next_state: ConfigParserState::Assign,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'no_focus",
        identifier: None,
        next_state: ConfigParserState::NoFocus,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'focus_follows_mouse",
        identifier: None,
        next_state: ConfigParserState::FocusFollowsMouse,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'mouse_warping",
        identifier: None,
        next_state: ConfigParserState::MouseWarping,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'focus_wrapping",
        identifier: None,
        next_state: ConfigParserState::FocusWrapping,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'force_focus_wrapping",
        identifier: None,
        next_state: ConfigParserState::ForceFocusWrapping,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'force_xinerama",
        identifier: None,
        next_state: ConfigParserState::ForceXinerama,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'force-xinerama",
        identifier: None,
        next_state: ConfigParserState::ForceXinerama,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'disable_randr15",
        identifier: None,
        next_state: ConfigParserState::DisableRandr15,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'disable-randr15",
        identifier: None,
        next_state: ConfigParserState::DisableRandr15,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'workspace_auto_back_and_forth",
        identifier: None,
        next_state: ConfigParserState::WorkspaceBackAndForth,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'fake_outputs",
        identifier: None,
        next_state: ConfigParserState::FakeOutputs,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'fake-outputs",
        identifier: None,
        next_state: ConfigParserState::FakeOutputs,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'force_display_urgency_hint",
        identifier: None,
        next_state: ConfigParserState::ForceDisplayUrgencyHint,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'focus_on_window_activation",
        identifier: None,
        next_state: ConfigParserState::FocusOnWindowActivation,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'title_align",
        identifier: None,
        next_state: ConfigParserState::TitleAlign,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'show_marks",
        identifier: None,
        next_state: ConfigParserState::ShowMarks,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'workspace",
        identifier: None,
        next_state: ConfigParserState::Workspace,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'ipc_socket",
        identifier: None,
        next_state: ConfigParserState::IpcSocket,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'ipc-socket",
        identifier: None,
        next_state: ConfigParserState::IpcSocket,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'ipc_kill_timeout",
        identifier: None,
        next_state: ConfigParserState::IpcKillTimeout,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'restart_state",
        identifier: None,
        next_state: ConfigParserState::RestartState,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'popup_during_fullscreen",
        identifier: None,
        next_state: ConfigParserState::PopupDuringFullscreen,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'tiling_drag",
        identifier: None,
        next_state: ConfigParserState::TilingDrag,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'exec_always",
        identifier: Some(b"exectype"),
        next_state: ConfigParserState::Exec,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'exec",
        identifier: Some(b"exectype"),
        next_state: ConfigParserState::Exec,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'client.background",
        identifier: Some(b"colorclass"),
        next_state: ConfigParserState::ColorSingle,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'client.focused_inactive",
        identifier: Some(b"colorclass"),
        next_state: ConfigParserState::ColorBorder,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'client.focused_tab_title",
        identifier: Some(b"colorclass"),
        next_state: ConfigParserState::ColorBorder,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'client.focused",
        identifier: Some(b"colorclass"),
        next_state: ConfigParserState::ColorBorder,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'client.unfocused",
        identifier: Some(b"colorclass"),
        next_state: ConfigParserState::ColorBorder,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'client.urgent",
        identifier: Some(b"colorclass"),
        next_state: ConfigParserState::ColorBorder,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'client.placeholder",
        identifier: Some(b"colorclass"),
        next_state: ConfigParserState::ColorBorder,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
];
static tokens_ASSIGN: [cmdp_token; 1] = [token {
    name: b"'[",
    identifier: None,
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 151_i32 as u16,
    },
}];
static tokens_BAR_ID: [cmdp_token; 1] = [token {
    name: b"word",
    identifier: Some(b"bar_id"),
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 152_i32 as u16,
    },
}];
static tokens_EXEC: [cmdp_token; 2] = [
    token {
        name: b"'--no-startup-id",
        identifier: Some(b"no_startup_id"),
        next_state: ConfigParserState::Exec,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"string",
        identifier: Some(b"command"),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 153_i32 as u16,
        },
    },
];
static tokens_FONT: [cmdp_token; 1] = [token {
    name: b"string",
    identifier: Some(b"font"),
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 154_i32 as u16,
    },
}];
static tokens_GAPS: [cmdp_token; 8] = [
    token {
        name: b"'inner",
        identifier: Some(b"scope"),
        next_state: ConfigParserState::GapsWithScope,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'outer",
        identifier: Some(b"scope"),
        next_state: ConfigParserState::GapsWithScope,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'horizontal",
        identifier: Some(b"scope"),
        next_state: ConfigParserState::GapsWithScope,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'vertical",
        identifier: Some(b"scope"),
        next_state: ConfigParserState::GapsWithScope,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'top",
        identifier: Some(b"scope"),
        next_state: ConfigParserState::GapsWithScope,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'right",
        identifier: Some(b"scope"),
        next_state: ConfigParserState::GapsWithScope,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'bottom",
        identifier: Some(b"scope"),
        next_state: ConfigParserState::GapsWithScope,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'left",
        identifier: Some(b"scope"),
        next_state: ConfigParserState::GapsWithScope,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
];
static tokens_MODE: [cmdp_token; 8] = [
    token {
        name: b"end",
        identifier: None,
        next_state: ConfigParserState::Mode,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"error",
        identifier: None,
        next_state: ConfigParserState::Mode,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'#",
        identifier: None,
        next_state: ConfigParserState::ModeIgnoreLine,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'set",
        identifier: None,
        next_state: ConfigParserState::ModeIgnoreLine,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'bindsym",
        identifier: Some(b"bindtype"),
        next_state: ConfigParserState::ModeBinding,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'bindcode",
        identifier: Some(b"bindtype"),
        next_state: ConfigParserState::ModeBinding,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'bind",
        identifier: Some(b"bindtype"),
        next_state: ConfigParserState::ModeBinding,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'}",
        identifier: None,
        next_state: ConfigParserState::Initial,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
];
static tokens_BAR: [cmdp_token; 31] = [
    token {
        name: b"end",
        identifier: None,
        next_state: ConfigParserState::Bar,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"error",
        identifier: None,
        next_state: ConfigParserState::Bar,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'#",
        identifier: None,
        next_state: ConfigParserState::BarIgnoreLine,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'set",
        identifier: None,
        next_state: ConfigParserState::BarIgnoreLine,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'i3bar_command",
        identifier: None,
        next_state: ConfigParserState::BarBarCommand,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'status_command",
        identifier: None,
        next_state: ConfigParserState::BarStatusCommand,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'workspace_command",
        identifier: None,
        next_state: ConfigParserState::BarWorkspaceCommand,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'socket_path",
        identifier: None,
        next_state: ConfigParserState::BarSocketPath,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'mode",
        identifier: None,
        next_state: ConfigParserState::BarMode,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'hidden_state",
        identifier: None,
        next_state: ConfigParserState::BarHiddenState,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'id",
        identifier: None,
        next_state: ConfigParserState::BarId,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'modifier",
        identifier: None,
        next_state: ConfigParserState::BarModifier,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'wheel_up_cmd",
        identifier: None,
        next_state: ConfigParserState::BarWheelUpCmd,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'wheel_down_cmd",
        identifier: None,
        next_state: ConfigParserState::BarWheelDownCmd,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'bindsym",
        identifier: None,
        next_state: ConfigParserState::BarBindsym,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'position",
        identifier: None,
        next_state: ConfigParserState::BarPosition,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'output",
        identifier: None,
        next_state: ConfigParserState::BarOutput,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'tray_output",
        identifier: None,
        next_state: ConfigParserState::BarTrayOutput,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'tray_padding",
        identifier: None,
        next_state: ConfigParserState::BarTrayPadding,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'font",
        identifier: None,
        next_state: ConfigParserState::BarFont,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'separator_symbol",
        identifier: None,
        next_state: ConfigParserState::BarSeparatorSymbol,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'binding_mode_indicator",
        identifier: None,
        next_state: ConfigParserState::BarBindingModeIndicator,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'workspace_buttons",
        identifier: None,
        next_state: ConfigParserState::BarWorkspaceButtons,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'workspace_min_width",
        identifier: None,
        next_state: ConfigParserState::BarWorkspaceMinWidth,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'strip_workspace_numbers",
        identifier: None,
        next_state: ConfigParserState::BarStripWorkspaceNumbers,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'strip_workspace_name",
        identifier: None,
        next_state: ConfigParserState::BarStripWorkspaceName,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'verbose",
        identifier: None,
        next_state: ConfigParserState::BarVerbose,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'height",
        identifier: None,
        next_state: ConfigParserState::BarHeight,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'padding",
        identifier: None,
        next_state: ConfigParserState::BarPadding,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'colors",
        identifier: None,
        next_state: ConfigParserState::BarColorsBrace,
        extra: ConfigTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'}",
        identifier: None,
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 155_i32 as u16,
        },
    },
];
static tokens: [&[cmdp_token]; 117] = [
    &tokens_FORCE_DISPLAY_URGENCY_HINT_MS,
    &tokens_FLOATING_MAXIMUM_SIZE_HEIGHT,
    &tokens_FLOATING_MINIMUM_SIZE_HEIGHT,
    &tokens_BAR_STRIP_WORKSPACE_NUMBERS,
    &tokens_FLOATING_MAXIMUM_SIZE_WIDTH,
    &tokens_FLOATING_MINIMUM_SIZE_WIDTH,
    &tokens_BAR_BINDING_MODE_INDICATOR,
    &tokens_BAR_WORKSPACE_MIN_WIDTH_PX,
    &tokens_FOCUS_ON_WINDOW_ACTIVATION,
    &tokens_FORCE_DISPLAY_URGENCY_HINT,
    &tokens_TILING_DRAG_SWAP_MODIFIER,
    &tokens_BAR_STRIP_WORKSPACE_NAME,
    &tokens_CRITERION_FROM_STR_START,
    &tokens_DEFAULT_BORDER_PIXELS_PX,
    &tokens_WORKSPACE_BACK_AND_FORTH,
    &tokens_ASSIGN_WORKSPACE_NUMBER,
    &tokens_BAR_WORKSPACE_MIN_WIDTH,
    &tokens_FLOATING_MAXIMUM_SIZE_X,
    &tokens_FLOATING_MINIMUM_SIZE_X,
    &tokens_POPUP_DURING_FULLSCREEN,
    &tokens_BAR_COLORS_IGNORE_LINE,
    &tokens_CRITERION_FROM_STR_END,
    &tokens_BAR_COLORS_BACKGROUND,
    &tokens_BAR_WORKSPACE_BUTTONS,
    &tokens_BAR_WORKSPACE_COMMAND,
    &tokens_DEFAULT_BORDER_PIXELS,
    &tokens_WORKSPACE_OUTPUT_WORD,
    &tokens_BAR_SEPARATOR_SYMBOL,
    &tokens_FORCE_FOCUS_WRAPPING,
    &tokens_BAR_BINDSYM_COMMAND,
    &tokens_BAR_TRAY_PADDING_PX,
    &tokens_DEFAULT_ORIENTATION,
    &tokens_FOCUS_FOLLOWS_MOUSE,
    &tokens_BAR_PADDING_BOTTOM,
    &tokens_BAR_STATUS_COMMAND,
    &tokens_BAR_WHEEL_DOWN_CMD,
    &tokens_COLOR_CHILD_BORDER,
    &tokens_CRITERION_FROM_STR,
    &tokens_FOR_WINDOW_COMMAND,
    &tokens_BAR_COLORS_BORDER,
    &tokens_BAR_COLORS_SINGLE,
    &tokens_BAR_PADDING_RIGHT,
    &tokens_FLOATING_MODIFIER,
    &tokens_HIDE_EDGE_BORDERS,
    &tokens_WORKSPACE_COMMAND,
    &tokens_ASSIGN_WORKSPACE,
    &tokens_BAR_COLORS_BRACE,
    &tokens_BAR_HIDDEN_STATE,
    &tokens_BAR_PADDING_LEFT,
    &tokens_BAR_TRAY_PADDING,
    &tokens_BAR_WHEEL_UP_CMD,
    &tokens_COLOR_BACKGROUND,
    &tokens_IPC_KILL_TIMEOUT,
    &tokens_MODE_BINDCOMMAND,
    &tokens_MODE_IGNORE_LINE,
    &tokens_TILING_DRAG_MODE,
    &tokens_WORKSPACE_LAYOUT,
    &tokens_BAR_BAR_COMMAND,
    &tokens_BAR_COLORS_TEXT,
    &tokens_BAR_IGNORE_LINE,
    &tokens_BAR_PADDING_TOP,
    &tokens_BAR_SOCKET_PATH,
    &tokens_BAR_TRAY_OUTPUT,
    &tokens_COLOR_INDICATOR,
    &tokens_DISABLE_RANDR15,
    &tokens_GAPS_WITH_SCOPE,
    &tokens_CRITERION_FROM,
    &tokens_DEFAULT_BORDER,
    &tokens_FOCUS_WRAPPING,
    &tokens_FORCE_XINERAMA,
    &tokens_ASSIGN_OUTPUT,
    &tokens_CRITERION_STR,
    &tokens_MOUSE_WARPING,
    &tokens_RESTART_STATE,
    &tokens_SMART_BORDERS,
    &tokens_BAR_MODIFIER,
    &tokens_BAR_POSITION,
    &tokens_COLOR_BORDER,
    &tokens_COLOR_SINGLE,
    &tokens_FAKE_OUTPUTS,
    &tokens_MODE_BINDING,
    &tokens_NO_FOCUS_END,
    &tokens_BAR_BINDSYM,
    &tokens_BAR_PADDING,
    &tokens_BAR_VERBOSE,
    &tokens_BINDCOMMAND,
    &tokens_IGNORE_LINE,
    &tokens_TILING_DRAG,
    &tokens_TITLE_ALIGN,
    &tokens_BAR_COLORS,
    &tokens_BAR_HEIGHT,
    &tokens_BAR_OUTPUT,
    &tokens_COLOR_TEXT,
    &tokens_FOR_WINDOW,
    &tokens_IPC_SOCKET,
    &tokens_SHOW_MARKS,
    &tokens_SMART_GAPS,
    &tokens_CRITERION,
    &tokens_MODEBRACE,
    &tokens_WORKSPACE,
    &tokens_BARBRACE,
    &tokens_BAR_FONT,
    &tokens_BAR_MODE,
    &tokens_CRITERIA,
    &tokens_GAPS_END,
    &tokens_MODENAME,
    &tokens_NO_FOCUS,
    &tokens_BINDING,
    &tokens_INCLUDE,
    &tokens_INITIAL,
    &tokens_ASSIGN,
    &tokens_BAR_ID,
    &tokens_EXEC,
    &tokens_FONT,
    &tokens_GAPS,
    &tokens_MODE,
    &tokens_BAR,
];
unsafe fn GENERATED_call(
    current_match: *mut Match,
    stack: *mut stack,
    call_identifier: i32,
    result: *mut ConfigResultIR,
) {
    // Independent captures remain live even when a directive reloads parser/model state.
    let captures = (&*stack).clone();
    match call_identifier {
        0 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_force_display_urgency_hint(
                current_match,
                result,
                captures.get_long(b"duration_ms"),
            );
        }
        1 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_floating_maximum_size(
                current_match,
                result,
                captures.get_long(b"width"),
                captures.get_long(b"height"),
            );
        }
        2 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_floating_minimum_size(
                current_match,
                result,
                captures.get_long(b"width"),
                captures.get_long(b"height"),
            );
        }
        3 => {
            (*result).next_state = ConfigParserState::Bar;
            cfg_bar_strip_workspace_numbers(
                current_match,
                result,
                i3_common::boolstr::is_enabled(
                    captures
                        .get_string(b"value")
                        .expect("boolean directive has a captured value"),
                ),
            );
        }
        4 => {
            (*result).next_state = ConfigParserState::Bar;
            cfg_bar_binding_mode_indicator(
                current_match,
                result,
                i3_common::boolstr::is_enabled(
                    captures
                        .get_string(b"value")
                        .expect("boolean directive has a captured value"),
                ),
            );
        }
        5 => {
            (*result).next_state = ConfigParserState::Bar;
            cfg_bar_workspace_min_width(current_match, result, captures.get_long(b"width"));
        }
        6 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_focus_on_window_activation(
                current_match,
                result,
                captures
                    .get_string(b"mode")
                    .expect("required directive capture"),
            );
        }
        7 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_tiling_drag_swap_modifier(current_match, result, captures.get_string(b"modifiers"));
        }
        8 => {
            (*result).next_state = ConfigParserState::Bar;
            cfg_bar_strip_workspace_name(
                current_match,
                result,
                i3_common::boolstr::is_enabled(
                    captures
                        .get_string(b"value")
                        .expect("boolean directive has a captured value"),
                ),
            );
        }
        9 => {
            (*result).next_state = ConfigParserState::Criteria;
            cfg_criteria_add(
                current_match,
                result,
                captures
                    .get_string(b"ctype")
                    .expect("required directive capture"),
                captures.get_string(b"kind"),
            );
        }
        10 => {
            (*result).next_state = ConfigParserState::Criteria;
            cfg_criteria_add(
                current_match,
                result,
                captures
                    .get_string(b"ctype")
                    .expect("required directive capture"),
                captures.get_string(b"kind"),
            );
        }
        11 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_default_border(
                current_match,
                result,
                captures
                    .get_string(b"windowtype")
                    .expect("required directive capture"),
                captures
                    .get_string(b"border")
                    .expect("required directive capture"),
                captures.get_long(b"width"),
            );
        }
        12 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_workspace_back_and_forth(
                current_match,
                result,
                i3_common::boolstr::is_enabled(
                    captures
                        .get_string(b"value")
                        .expect("boolean directive has a captured value"),
                ),
            );
        }
        13 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_assign(
                current_match,
                result,
                captures
                    .get_string(b"number")
                    .expect("required directive capture"),
                true,
            );
        }
        14 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_popup_during_fullscreen(
                current_match,
                result,
                captures
                    .get_string(b"value")
                    .expect("required directive capture"),
            );
        }
        15 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_popup_during_fullscreen(
                current_match,
                result,
                captures
                    .get_string(b"value")
                    .expect("required directive capture"),
            );
        }
        16 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_popup_during_fullscreen(
                current_match,
                result,
                captures
                    .get_string(b"value")
                    .expect("required directive capture"),
            );
        }
        17 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_popup_during_fullscreen(
                current_match,
                result,
                captures
                    .get_string(b"value")
                    .expect("required directive capture"),
            );
        }
        18 => {
            (*result).next_state = ConfigParserState::Criteria;
            cfg_criteria_add(
                current_match,
                result,
                captures
                    .get_string(b"ctype")
                    .expect("required directive capture"),
                captures.get_string(b"kind"),
            );
        }
        19 => {
            (*result).next_state = ConfigParserState::Bar;
            cfg_bar_workspace_buttons(
                current_match,
                result,
                i3_common::boolstr::is_enabled(
                    captures
                        .get_string(b"value")
                        .expect("boolean directive has a captured value"),
                ),
            );
        }
        20 => {
            (*result).next_state = ConfigParserState::Bar;
            {
                let bar_arg_command = captures
                    .get_string(b"command")
                    .expect("required directive capture")
                    .to_vec();
                cfg_bar_workspace_command(current_match, result, &bar_arg_command);
            }
        }
        21 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_default_border(
                current_match,
                result,
                captures
                    .get_string(b"windowtype")
                    .expect("required directive capture"),
                captures
                    .get_string(b"border")
                    .expect("required directive capture"),
                2_i64,
            );
        }
        22 => {
            (*result).next_state = ConfigParserState::WorkspaceOutputWord;
            let workspace_pointer = captures.get_string(b"workspace");
            let output_pointer = captures.get_string(b"output");
            let workspace = workspace_pointer.map(<[u8]>::to_vec);
            let output = output_pointer.map(<[u8]>::to_vec);
            cfg_workspace(
                current_match,
                result,
                workspace.as_deref(),
                output.as_deref(),
            );
        }
        23 => {
            (*result).next_state = ConfigParserState::Bar;
            {
                let bar_arg_separator = captures
                    .get_string(b"separator")
                    .expect("required directive capture")
                    .to_vec();
                cfg_bar_separator_symbol(current_match, result, &bar_arg_separator);
            }
        }
        24 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_force_focus_wrapping(
                current_match,
                result,
                i3_common::boolstr::is_enabled(
                    captures
                        .get_string(b"value")
                        .expect("boolean directive has a captured value"),
                ),
            );
        }
        25 => {
            (*result).next_state = ConfigParserState::Bar;
            {
                let bar_arg_button = captures
                    .get_string(b"button")
                    .expect("required directive capture")
                    .to_vec();
                let bar_arg_command = captures
                    .get_string(b"command")
                    .expect("required directive capture")
                    .to_vec();
                cfg_bar_bindsym(
                    current_match,
                    result,
                    &bar_arg_button,
                    captures.get_string(b"release").is_some(),
                    &bar_arg_command,
                );
            }
        }
        26 => {
            (*result).next_state = ConfigParserState::Bar;
            cfg_bar_tray_padding(current_match, result, captures.get_long(b"padding_px"));
        }
        27 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_default_orientation(
                current_match,
                result,
                captures
                    .get_string(b"orientation")
                    .expect("required directive capture"),
            );
        }
        28 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_default_orientation(
                current_match,
                result,
                captures
                    .get_string(b"orientation")
                    .expect("required directive capture"),
            );
        }
        29 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_default_orientation(
                current_match,
                result,
                captures
                    .get_string(b"orientation")
                    .expect("required directive capture"),
            );
        }
        30 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_focus_follows_mouse(
                current_match,
                result,
                i3_common::boolstr::is_enabled(
                    captures
                        .get_string(b"value")
                        .expect("boolean directive has a captured value"),
                ),
            );
        }
        31 => {
            (*result).next_state = ConfigParserState::Bar;
            cfg_bar_padding_three(
                current_match,
                result,
                captures.get_long(b"top_or_all"),
                captures.get_long(b"right_or_right_and_left"),
                captures.get_long(b"bottom"),
            );
        }
        32 => {
            (*result).next_state = ConfigParserState::Bar;
            {
                let bar_arg_command = captures
                    .get_string(b"command")
                    .expect("required directive capture")
                    .to_vec();
                cfg_bar_status_command(current_match, result, &bar_arg_command);
            }
        }
        33 => {
            (*result).next_state = ConfigParserState::Bar;
            {
                let bar_arg_command = captures
                    .get_string(b"command")
                    .expect("required directive capture")
                    .to_vec();
                cfg_bar_wheel_down_cmd(current_match, result, &bar_arg_command);
            }
        }
        34 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_color(
                current_match,
                result,
                captures
                    .get_string(b"colorclass")
                    .expect("required directive capture"),
                captures
                    .get_string(b"border")
                    .expect("required directive capture"),
                captures
                    .get_string(b"background")
                    .expect("required directive capture"),
                captures
                    .get_string(b"text")
                    .expect("required directive capture"),
                captures.get_string(b"indicator"),
                captures.get_string(b"child_border"),
            );
        }
        35 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_color(
                current_match,
                result,
                captures
                    .get_string(b"colorclass")
                    .expect("required directive capture"),
                captures
                    .get_string(b"border")
                    .expect("required directive capture"),
                captures
                    .get_string(b"background")
                    .expect("required directive capture"),
                captures
                    .get_string(b"text")
                    .expect("required directive capture"),
                captures.get_string(b"indicator"),
                None,
            );
        }
        36 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_for_window(
                current_match,
                result,
                captures
                    .get_string(b"command")
                    .expect("required directive capture"),
            );
        }
        37 => {
            (*result).next_state = ConfigParserState::BarColors;
            {
                let bar_arg_colorclass = captures
                    .get_string(b"colorclass")
                    .expect("required directive capture")
                    .to_vec();
                let bar_arg_color = captures
                    .get_string(b"color")
                    .expect("required directive capture")
                    .to_vec();
                cfg_bar_color_single(current_match, result, &bar_arg_colorclass, &bar_arg_color);
            }
        }
        38 => {
            (*result).next_state = ConfigParserState::Bar;
            cfg_bar_padding_two(
                current_match,
                result,
                captures.get_long(b"top_or_all"),
                captures.get_long(b"right_or_right_and_left"),
            );
        }
        39 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_floating_modifier(current_match, result, captures.get_string(b"modifiers"));
        }
        40 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_hide_edge_borders(
                current_match,
                result,
                captures
                    .get_string(b"hide_borders")
                    .expect("boolean directive has a captured value"),
            );
        }
        41 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_hide_edge_borders(
                current_match,
                result,
                captures
                    .get_string(b"hide_borders")
                    .expect("boolean directive has a captured value"),
            );
        }
        42 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_hide_edge_borders(
                current_match,
                result,
                captures
                    .get_string(b"hide_borders")
                    .expect("boolean directive has a captured value"),
            );
        }
        43 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_hide_edge_borders(
                current_match,
                result,
                captures
                    .get_string(b"hide_borders")
                    .expect("boolean directive has a captured value"),
            );
        }
        44 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_hide_edge_borders(
                current_match,
                result,
                captures
                    .get_string(b"hide_borders")
                    .expect("boolean directive has a captured value"),
            );
        }
        45 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_hide_edge_borders(
                current_match,
                result,
                captures
                    .get_string(b"hide_borders")
                    .expect("boolean directive has a captured value"),
            );
        }
        46 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_hide_edge_borders(
                current_match,
                result,
                captures
                    .get_string(b"hide_borders")
                    .expect("boolean directive has a captured value"),
            );
        }
        47 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_hide_edge_borders(
                current_match,
                result,
                captures
                    .get_string(b"hide_borders")
                    .expect("boolean directive has a captured value"),
            );
        }
        48 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_hide_edge_borders(
                current_match,
                result,
                captures
                    .get_string(b"hide_borders")
                    .expect("boolean directive has a captured value"),
            );
        }
        49 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_hide_edge_borders(
                current_match,
                result,
                captures
                    .get_string(b"hide_borders")
                    .expect("boolean directive has a captured value"),
            );
        }
        50 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_hide_edge_borders(
                current_match,
                result,
                captures
                    .get_string(b"hide_borders")
                    .expect("boolean directive has a captured value"),
            );
        }
        51 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_hide_edge_borders(
                current_match,
                result,
                captures
                    .get_string(b"hide_borders")
                    .expect("boolean directive has a captured value"),
            );
        }
        52 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_assign(
                current_match,
                result,
                captures
                    .get_string(b"workspace")
                    .expect("required directive capture"),
                false,
            );
        }
        53 => {
            (*result).next_state = ConfigParserState::Bar;
            {
                let bar_arg_hidden_state = captures
                    .get_string(b"hidden_state")
                    .expect("required directive capture")
                    .to_vec();
                cfg_bar_hidden_state(current_match, result, &bar_arg_hidden_state);
            }
        }
        54 => {
            (*result).next_state = ConfigParserState::Bar;
            {
                let bar_arg_hidden_state = captures
                    .get_string(b"hidden_state")
                    .expect("required directive capture")
                    .to_vec();
                cfg_bar_hidden_state(current_match, result, &bar_arg_hidden_state);
            }
        }
        55 => {
            (*result).next_state = ConfigParserState::Bar;
            cfg_bar_padding_four(
                current_match,
                result,
                captures.get_long(b"top_or_all"),
                captures.get_long(b"right_or_right_and_left"),
                captures.get_long(b"bottom"),
                captures.get_long(b"left"),
            );
        }
        56 => {
            (*result).next_state = ConfigParserState::Bar;
            {
                let bar_arg_command = captures
                    .get_string(b"command")
                    .expect("required directive capture")
                    .to_vec();
                cfg_bar_wheel_up_cmd(current_match, result, &bar_arg_command);
            }
        }
        57 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_ipc_kill_timeout(current_match, result, captures.get_long(b"timeout"));
        }
        58 => {
            (*result).next_state = ConfigParserState::Mode;
            let bindtype_pointer = captures.get_string(b"bindtype");
            let bindtype_owner = bindtype_pointer
                .expect("required directive capture")
                .to_vec();
            let modifiers_pointer = captures.get_string(b"modifiers");
            let modifiers_owner = modifiers_pointer.map(<[u8]>::to_vec);
            let key_pointer = captures.get_string(b"key");
            let key_owner = key_pointer.expect("required directive capture").to_vec();
            let release_pointer = captures.get_string(b"release");
            let release_owner = release_pointer.map(<[u8]>::to_vec);
            let border_pointer = captures.get_string(b"border");
            let border_owner = border_pointer.map(<[u8]>::to_vec);
            let whole_window_pointer = captures.get_string(b"whole_window");
            let whole_window_owner = whole_window_pointer.map(<[u8]>::to_vec);
            let exclude_titlebar_pointer = captures.get_string(b"exclude_titlebar");
            let exclude_titlebar_owner = exclude_titlebar_pointer.map(<[u8]>::to_vec);
            let command_pointer = captures.get_string(b"command");
            let command_owner = command_pointer
                .expect("required directive capture")
                .to_vec();
            cfg_mode_binding(
                current_match,
                result,
                &bindtype_owner,
                modifiers_owner.as_deref(),
                &key_owner,
                release_owner.as_deref(),
                border_owner.as_deref(),
                whole_window_owner.as_deref(),
                exclude_titlebar_owner.as_deref(),
                &command_owner,
            );
        }
        59 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_tiling_drag(
                current_match,
                result,
                captures
                    .get_string(b"value")
                    .expect("required directive capture"),
            );
        }
        60 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_workspace_layout(
                current_match,
                result,
                captures
                    .get_string(b"layout")
                    .expect("required directive capture"),
            );
        }
        61 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_workspace_layout(
                current_match,
                result,
                captures
                    .get_string(b"layout")
                    .expect("required directive capture"),
            );
        }
        62 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_workspace_layout(
                current_match,
                result,
                captures
                    .get_string(b"layout")
                    .expect("required directive capture"),
            );
        }
        63 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_workspace_layout(
                current_match,
                result,
                captures
                    .get_string(b"layout")
                    .expect("required directive capture"),
            );
        }
        64 => {
            (*result).next_state = ConfigParserState::Bar;
            {
                let bar_arg_i3bar_command = captures
                    .get_string(b"command")
                    .expect("required directive capture")
                    .to_vec();
                cfg_bar_i3bar_command(current_match, result, &bar_arg_i3bar_command);
            }
        }
        65 => {
            (*result).next_state = ConfigParserState::BarColors;
            {
                let bar_arg_colorclass = captures
                    .get_string(b"colorclass")
                    .expect("required directive capture")
                    .to_vec();
                let bar_arg_border = captures
                    .get_string(b"border")
                    .expect("required directive capture")
                    .to_vec();
                let bar_arg_background = captures
                    .get_string(b"background")
                    .expect("required directive capture")
                    .to_vec();
                let bar_arg_text: Option<Vec<u8>> = None;
                cfg_bar_color(
                    current_match,
                    result,
                    &bar_arg_colorclass,
                    &bar_arg_border,
                    &bar_arg_background,
                    bar_arg_text.as_deref(),
                );
            }
        }
        66 => {
            (*result).next_state = ConfigParserState::BarColors;
            {
                let bar_arg_colorclass = captures
                    .get_string(b"colorclass")
                    .expect("required directive capture")
                    .to_vec();
                let bar_arg_border = captures
                    .get_string(b"border")
                    .expect("required directive capture")
                    .to_vec();
                let bar_arg_background = captures
                    .get_string(b"background")
                    .expect("required directive capture")
                    .to_vec();
                let bar_arg_text = captures.get_string(b"text").map(<[u8]>::to_vec);
                cfg_bar_color(
                    current_match,
                    result,
                    &bar_arg_colorclass,
                    &bar_arg_border,
                    &bar_arg_background,
                    bar_arg_text.as_deref(),
                );
            }
        }
        67 => {
            (*result).next_state = ConfigParserState::Bar;
            cfg_bar_padding_one(current_match, result, captures.get_long(b"top_or_all"));
        }
        68 => {
            (*result).next_state = ConfigParserState::Bar;
            {
                let bar_arg_socket_path = captures
                    .get_string(b"path")
                    .expect("required directive capture")
                    .to_vec();
                let bar_arg_socket_path =
                    std::path::PathBuf::from(std::ffi::OsString::from_vec(bar_arg_socket_path));
                cfg_bar_socket_path(current_match, result, &bar_arg_socket_path);
            }
        }
        69 => {
            (*result).next_state = ConfigParserState::Bar;
            {
                let bar_arg_output = captures
                    .get_string(b"output")
                    .expect("required directive capture")
                    .to_vec();
                cfg_bar_tray_output(current_match, result, &bar_arg_output);
            }
        }
        70 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_color(
                current_match,
                result,
                captures
                    .get_string(b"colorclass")
                    .expect("required directive capture"),
                captures
                    .get_string(b"border")
                    .expect("required directive capture"),
                captures
                    .get_string(b"background")
                    .expect("required directive capture"),
                captures
                    .get_string(b"text")
                    .expect("required directive capture"),
                None,
                None,
            );
        }
        71 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_disable_randr15(
                current_match,
                result,
                i3_common::boolstr::is_enabled(
                    captures
                        .get_string(b"value")
                        .expect("boolean directive has a captured value"),
                ),
            );
        }
        72 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_default_border(
                current_match,
                result,
                captures
                    .get_string(b"windowtype")
                    .expect("required directive capture"),
                captures
                    .get_string(b"border")
                    .expect("required directive capture"),
                -1_i32 as i64,
            );
        }
        73 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_default_border(
                current_match,
                result,
                captures
                    .get_string(b"windowtype")
                    .expect("required directive capture"),
                captures
                    .get_string(b"border")
                    .expect("required directive capture"),
                -1_i32 as i64,
            );
        }
        74 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_focus_wrapping(
                current_match,
                result,
                captures
                    .get_string(b"value")
                    .expect("boolean directive has a captured value"),
            );
        }
        75 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_focus_wrapping(
                current_match,
                result,
                captures
                    .get_string(b"value")
                    .expect("boolean directive has a captured value"),
            );
        }
        76 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_focus_wrapping(
                current_match,
                result,
                captures
                    .get_string(b"value")
                    .expect("boolean directive has a captured value"),
            );
        }
        77 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_focus_wrapping(
                current_match,
                result,
                captures
                    .get_string(b"value")
                    .expect("boolean directive has a captured value"),
            );
        }
        78 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_focus_wrapping(
                current_match,
                result,
                captures
                    .get_string(b"value")
                    .expect("boolean directive has a captured value"),
            );
        }
        79 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_focus_wrapping(
                current_match,
                result,
                captures
                    .get_string(b"value")
                    .expect("boolean directive has a captured value"),
            );
        }
        80 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_focus_wrapping(
                current_match,
                result,
                captures
                    .get_string(b"value")
                    .expect("boolean directive has a captured value"),
            );
        }
        81 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_focus_wrapping(
                current_match,
                result,
                captures
                    .get_string(b"value")
                    .expect("boolean directive has a captured value"),
            );
        }
        82 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_focus_wrapping(
                current_match,
                result,
                captures
                    .get_string(b"value")
                    .expect("boolean directive has a captured value"),
            );
        }
        83 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_focus_wrapping(
                current_match,
                result,
                captures
                    .get_string(b"value")
                    .expect("boolean directive has a captured value"),
            );
        }
        84 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_focus_wrapping(
                current_match,
                result,
                captures
                    .get_string(b"value")
                    .expect("boolean directive has a captured value"),
            );
        }
        85 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_focus_wrapping(
                current_match,
                result,
                captures
                    .get_string(b"value")
                    .expect("boolean directive has a captured value"),
            );
        }
        86 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_focus_wrapping(
                current_match,
                result,
                captures
                    .get_string(b"value")
                    .expect("boolean directive has a captured value"),
            );
        }
        87 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_focus_wrapping(
                current_match,
                result,
                captures
                    .get_string(b"value")
                    .expect("boolean directive has a captured value"),
            );
        }
        88 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_force_xinerama(
                current_match,
                result,
                i3_common::boolstr::is_enabled(
                    captures
                        .get_string(b"value")
                        .expect("boolean directive has a captured value"),
                ),
            );
        }
        89 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_assign_output(
                current_match,
                result,
                captures
                    .get_string(b"output")
                    .expect("required directive capture"),
            );
        }
        90 => {
            (*result).next_state = ConfigParserState::Criteria;
            cfg_criteria_add(
                current_match,
                result,
                captures
                    .get_string(b"ctype")
                    .expect("required directive capture"),
                captures.get_string(b"cvalue"),
            );
        }
        91 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_mouse_warping(
                current_match,
                result,
                captures
                    .get_string(b"value")
                    .expect("required directive capture"),
            );
        }
        92 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_mouse_warping(
                current_match,
                result,
                captures
                    .get_string(b"value")
                    .expect("required directive capture"),
            );
        }
        93 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_restart_state(
                current_match,
                result,
                std::path::Path::new(std::ffi::OsStr::from_bytes(
                    captures
                        .get_string(b"path")
                        .expect("required directive capture"),
                )),
            );
        }
        94 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_smart_borders(
                current_match,
                result,
                captures
                    .get_string(b"enabled")
                    .expect("boolean directive has a captured value"),
            );
        }
        95 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_smart_borders(
                current_match,
                result,
                captures
                    .get_string(b"enabled")
                    .expect("boolean directive has a captured value"),
            );
        }
        96 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_smart_borders(
                current_match,
                result,
                captures
                    .get_string(b"enabled")
                    .expect("boolean directive has a captured value"),
            );
        }
        97 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_smart_borders(
                current_match,
                result,
                captures
                    .get_string(b"enabled")
                    .expect("boolean directive has a captured value"),
            );
        }
        98 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_smart_borders(
                current_match,
                result,
                captures
                    .get_string(b"enabled")
                    .expect("boolean directive has a captured value"),
            );
        }
        99 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_smart_borders(
                current_match,
                result,
                captures
                    .get_string(b"enabled")
                    .expect("boolean directive has a captured value"),
            );
        }
        100 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_smart_borders(
                current_match,
                result,
                captures
                    .get_string(b"enabled")
                    .expect("boolean directive has a captured value"),
            );
        }
        101 => {
            (*result).next_state = ConfigParserState::Bar;
            {
                let bar_arg_modifiers: Option<Vec<u8>> = None;
                cfg_bar_modifier(current_match, result, bar_arg_modifiers.as_deref());
            }
        }
        102 => {
            (*result).next_state = ConfigParserState::Bar;
            {
                let bar_arg_modifiers: Option<Vec<u8>> = None;
                cfg_bar_modifier(current_match, result, bar_arg_modifiers.as_deref());
            }
        }
        103 => {
            (*result).next_state = ConfigParserState::Bar;
            {
                let bar_arg_modifiers = captures.get_string(b"modifiers").map(<[u8]>::to_vec);
                cfg_bar_modifier(current_match, result, bar_arg_modifiers.as_deref());
            }
        }
        104 => {
            (*result).next_state = ConfigParserState::Bar;
            {
                let bar_arg_position = captures
                    .get_string(b"position")
                    .expect("required directive capture")
                    .to_vec();
                cfg_bar_position(current_match, result, &bar_arg_position);
            }
        }
        105 => {
            (*result).next_state = ConfigParserState::Bar;
            {
                let bar_arg_position = captures
                    .get_string(b"position")
                    .expect("required directive capture")
                    .to_vec();
                cfg_bar_position(current_match, result, &bar_arg_position);
            }
        }
        106 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_color_single(
                current_match,
                result,
                captures
                    .get_string(b"colorclass")
                    .expect("required directive capture"),
                captures
                    .get_string(b"color")
                    .expect("required directive capture"),
            );
        }
        107 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_fake_outputs(
                current_match,
                result,
                captures
                    .get_string(b"outputs")
                    .expect("required directive capture"),
            );
        }
        108 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_no_focus(current_match, result);
        }
        109 => {
            (*result).next_state = ConfigParserState::Bar;
            cfg_bar_verbose(
                current_match,
                result,
                i3_common::boolstr::is_enabled(
                    captures
                        .get_string(b"value")
                        .expect("boolean directive has a captured value"),
                ),
            );
        }
        110 => {
            (*result).next_state = ConfigParserState::Initial;
            let bindtype_pointer = captures.get_string(b"bindtype");
            let bindtype_owner = bindtype_pointer
                .expect("required directive capture")
                .to_vec();
            let modifiers_pointer = captures.get_string(b"modifiers");
            let modifiers_owner = modifiers_pointer.map(<[u8]>::to_vec);
            let key_pointer = captures.get_string(b"key");
            let key_owner = key_pointer.expect("required directive capture").to_vec();
            let release_pointer = captures.get_string(b"release");
            let release_owner = release_pointer.map(<[u8]>::to_vec);
            let border_pointer = captures.get_string(b"border");
            let border_owner = border_pointer.map(<[u8]>::to_vec);
            let whole_window_pointer = captures.get_string(b"whole_window");
            let whole_window_owner = whole_window_pointer.map(<[u8]>::to_vec);
            let exclude_titlebar_pointer = captures.get_string(b"exclude_titlebar");
            let exclude_titlebar_owner = exclude_titlebar_pointer.map(<[u8]>::to_vec);
            let command_pointer = captures.get_string(b"command");
            let command_owner = command_pointer
                .expect("required directive capture")
                .to_vec();
            cfg_binding(
                current_match,
                result,
                &bindtype_owner,
                modifiers_owner.as_deref(),
                &key_owner,
                release_owner.as_deref(),
                border_owner.as_deref(),
                whole_window_owner.as_deref(),
                exclude_titlebar_owner.as_deref(),
                &command_owner,
            );
        }
        111 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_tiling_drag(
                current_match,
                result,
                captures
                    .get_string(b"off")
                    .expect("required directive capture"),
            );
        }
        112 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_tiling_drag(
                current_match,
                result,
                captures
                    .get_string(b"off")
                    .expect("required directive capture"),
            );
        }
        113 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_tiling_drag(
                current_match,
                result,
                captures
                    .get_string(b"off")
                    .expect("required directive capture"),
            );
        }
        114 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_tiling_drag(
                current_match,
                result,
                captures
                    .get_string(b"off")
                    .expect("required directive capture"),
            );
        }
        115 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_tiling_drag(
                current_match,
                result,
                captures
                    .get_string(b"off")
                    .expect("required directive capture"),
            );
        }
        116 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_tiling_drag(
                current_match,
                result,
                captures
                    .get_string(b"off")
                    .expect("required directive capture"),
            );
        }
        117 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_title_align(
                current_match,
                result,
                captures
                    .get_string(b"alignment")
                    .expect("required directive capture"),
            );
        }
        118 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_title_align(
                current_match,
                result,
                captures
                    .get_string(b"alignment")
                    .expect("required directive capture"),
            );
        }
        119 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_title_align(
                current_match,
                result,
                captures
                    .get_string(b"alignment")
                    .expect("required directive capture"),
            );
        }
        120 => {
            (*result).next_state = ConfigParserState::Bar;
            cfg_bar_height(current_match, result, captures.get_long(b"value"));
        }
        121 => {
            (*result).next_state = ConfigParserState::Bar;
            {
                let bar_arg_output = captures
                    .get_string(b"output")
                    .expect("required directive capture")
                    .to_vec();
                cfg_bar_output(current_match, result, &bar_arg_output);
            }
        }
        122 => {
            (*result).next_state = ConfigParserState::Criteria;
            cfg_criteria_init(current_match, result, ConfigParserState::ForWindowCommand);
        }
        123 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_ipc_socket(
                current_match,
                result,
                std::path::Path::new(std::ffi::OsStr::from_bytes(
                    captures
                        .get_string(b"path")
                        .expect("required directive capture"),
                )),
            );
        }
        124 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_show_marks(
                current_match,
                result,
                i3_common::boolstr::is_enabled(
                    captures
                        .get_string(b"value")
                        .expect("boolean directive has a captured value"),
                ),
            );
        }
        125 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_smart_gaps(
                current_match,
                result,
                captures
                    .get_string(b"enabled")
                    .expect("boolean directive has a captured value"),
            );
        }
        126 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_smart_gaps(
                current_match,
                result,
                captures
                    .get_string(b"enabled")
                    .expect("boolean directive has a captured value"),
            );
        }
        127 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_smart_gaps(
                current_match,
                result,
                captures
                    .get_string(b"enabled")
                    .expect("boolean directive has a captured value"),
            );
        }
        128 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_smart_gaps(
                current_match,
                result,
                captures
                    .get_string(b"enabled")
                    .expect("boolean directive has a captured value"),
            );
        }
        129 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_smart_gaps(
                current_match,
                result,
                captures
                    .get_string(b"enabled")
                    .expect("boolean directive has a captured value"),
            );
        }
        130 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_smart_gaps(
                current_match,
                result,
                captures
                    .get_string(b"enabled")
                    .expect("boolean directive has a captured value"),
            );
        }
        131 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_smart_gaps(
                current_match,
                result,
                captures
                    .get_string(b"enabled")
                    .expect("boolean directive has a captured value"),
            );
        }
        132 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_smart_gaps(
                current_match,
                result,
                captures
                    .get_string(b"enabled")
                    .expect("boolean directive has a captured value"),
            );
        }
        133 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_smart_gaps(
                current_match,
                result,
                captures
                    .get_string(b"enabled")
                    .expect("boolean directive has a captured value"),
            );
        }
        134 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_smart_gaps(
                current_match,
                result,
                captures
                    .get_string(b"enabled")
                    .expect("boolean directive has a captured value"),
            );
        }
        135 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_smart_gaps(
                current_match,
                result,
                captures
                    .get_string(b"enabled")
                    .expect("boolean directive has a captured value"),
            );
        }
        136 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_smart_gaps(
                current_match,
                result,
                captures
                    .get_string(b"enabled")
                    .expect("boolean directive has a captured value"),
            );
        }
        137 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_smart_gaps(
                current_match,
                result,
                captures
                    .get_string(b"enabled")
                    .expect("boolean directive has a captured value"),
            );
        }
        138 => {
            (*result).next_state = ConfigParserState::Bar;
            cfg_bar_start(current_match, result);
        }
        139 => {
            (*result).next_state = ConfigParserState::Bar;
            {
                let bar_arg_font = captures
                    .get_string(b"font")
                    .expect("required directive capture")
                    .to_vec();
                cfg_bar_font(current_match, result, &bar_arg_font);
            }
        }
        140 => {
            (*result).next_state = ConfigParserState::Bar;
            {
                let bar_arg_mode = captures
                    .get_string(b"mode")
                    .expect("required directive capture")
                    .to_vec();
                cfg_bar_mode(current_match, result, &bar_arg_mode);
            }
        }
        141 => {
            (*result).next_state = ConfigParserState::Bar;
            {
                let bar_arg_mode = captures
                    .get_string(b"mode")
                    .expect("required directive capture")
                    .to_vec();
                cfg_bar_mode(current_match, result, &bar_arg_mode);
            }
        }
        142 => {
            (*result).next_state = ConfigParserState::Bar;
            {
                let bar_arg_mode = captures
                    .get_string(b"mode")
                    .expect("required directive capture")
                    .to_vec();
                cfg_bar_mode(current_match, result, &bar_arg_mode);
            }
        }
        143 => {
            (*result).next_state = ConfigParserState::Criteria;
            cfg_criteria_add(
                current_match,
                result,
                captures
                    .get_string(b"ctype")
                    .expect("required directive capture"),
                None,
            );
        }
        144 => {
            (*result).next_state = ConfigParserState::Criteria;
            cfg_criteria_add(
                current_match,
                result,
                captures
                    .get_string(b"ctype")
                    .expect("required directive capture"),
                None,
            );
        }
        145 => {
            (*result).next_state = ConfigParserState::Criteria;
            cfg_criteria_add(
                current_match,
                result,
                captures
                    .get_string(b"ctype")
                    .expect("required directive capture"),
                None,
            );
        }
        146 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_criteria_pop_state(current_match, result);
        }
        147 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_gaps(
                current_match,
                result,
                captures.get_string(b"workspace"),
                captures
                    .get_string(b"scope")
                    .expect("required directive capture"),
                captures.get_long(b"value"),
            );
        }
        148 => {
            (*result).next_state = ConfigParserState::Modebrace;
            let modename = captures
                .get_string(b"modename")
                .expect("required directive capture")
                .to_vec();
            cfg_enter_mode(
                current_match,
                result,
                captures.get_string(b"pango_markup").is_some(),
                &modename,
            );
        }
        149 => {
            (*result).next_state = ConfigParserState::Criteria;
            cfg_criteria_init(current_match, result, ConfigParserState::NoFocusEnd);
        }
        150 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_include(
                current_match,
                result,
                captures
                    .get_string(b"pattern")
                    .expect("required directive capture"),
            );
        }
        151 => {
            (*result).next_state = ConfigParserState::Criteria;
            cfg_criteria_init(current_match, result, ConfigParserState::AssignWorkspace);
        }
        152 => {
            (*result).next_state = ConfigParserState::Bar;
            {
                let bar_arg_bar_id = captures
                    .get_string(b"bar_id")
                    .expect("required directive capture")
                    .to_vec();
                cfg_bar_id(current_match, result, &bar_arg_bar_id);
            }
        }
        153 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_exec(
                current_match,
                result,
                captures
                    .get_string(b"exectype")
                    .expect("required directive capture"),
                captures.get_string(b"no_startup_id").is_some(),
                captures
                    .get_string(b"command")
                    .expect("required directive capture"),
            );
        }
        154 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_font(
                current_match,
                result,
                captures
                    .get_string(b"font")
                    .expect("required directive capture"),
            );
        }
        155 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_bar_finish(current_match, result);
        }
        _ => {
            printf(c"BUG in the parser. state = %d\n".as_ptr(), call_identifier);
            __assert_fail(
                c"false".as_ptr(),
                c"./GENERATED_config_call.h".as_ptr(),
                1265 as ::core::ffi::c_uint,
                c"void GENERATED_call(Match *, struct stack *, const int, struct ConfigResultIR *)"
                    .as_ptr(),
            );
        }
    };
}
// Grammar metadata is immutable Rust bytes. Bounds-check the state before lookup.
fn state_tokens(state: ConfigParserState) -> &'static [cmdp_token] {
    tokens
        .get(state as usize)
        .copied()
        .expect("grammar state has a token table")
}

unsafe fn next_state(token: &cmdp_token, ctx: *mut parser_ctx) {
    let mut _next_state: ConfigParserState = token.next_state;
    if token.next_state == ConfigParserState::Call {
        let mut subcommand_output: ConfigResultIR = ConfigResultIR {
            ctx,
            next_state: ConfigParserState::ForceDisplayUrgencyHintMs,
            has_errors: false,
        };
        GENERATED_call(
            &raw mut (*ctx).current_match,
            (*ctx).stack,
            token.extra.call_identifier as i32,
            &raw mut subcommand_output,
        );
        if subcommand_output.has_errors {
            (*ctx).has_errors = true;
        }
        _next_state = subcommand_output.next_state;
        (&mut *(*ctx).stack).clear();
    }
    (*ctx).state = _next_state;
    if (*ctx).state == ConfigParserState::Initial {
        (&mut *(*ctx).stack).clear();
    }
    if let Some(index) = (*ctx)
        .statelist
        .iter()
        .position(|state| *state == _next_state)
    {
        (*ctx).statelist.truncate(index + 1);
    } else {
        (*ctx).statelist.push(_next_state);
    }
    (*ctx).statelist_idx = (*ctx).statelist.len() as i32;
}
fn line_start(bytes: &[u8], position: usize) -> usize {
    let end = position.saturating_add(1).min(bytes.len());
    bytes[..end]
        .iter()
        .rposition(|&byte| byte == b'\n' || byte == b'\r')
        .map_or(0, |index| index + 1)
}

unsafe fn start_of_line(
    walk: *const libc::c_char,
    beginning: *const libc::c_char,
) -> *const libc::c_char {
    let bytes = std::ffi::CStr::from_ptr(beginning).to_bytes();
    let position = walk.offset_from(beginning) as usize;
    beginning.add(line_start(bytes, position))
}

unsafe fn previous_line(
    start: *const libc::c_char,
    beginning: *const libc::c_char,
) -> *const libc::c_char {
    let position = start.offset_from(beginning) as usize;
    start_of_line(beginning.add(position.saturating_sub(2)), beginning)
}

fn single_line(bytes: &[u8]) -> Vec<u8> {
    bytes
        .split(|byte| *byte == b'\n' || *byte == 0)
        .next()
        .unwrap()
        .to_vec()
}
unsafe fn parse_config(ctx: *mut parser_ctx, input: &[u8], context: *mut context) {
    let mut scanner_input = input.split(|byte| *byte == 0).next().unwrap().to_vec();
    scanner_input.push(0);
    let input = scanner_input.as_ptr().cast::<libc::c_char>();
    let mut dumpwalk: *const ::core::ffi::c_char = input;
    let mut linecnt: i32 = 1_i32;
    while *dumpwalk as i32 != '\0' as i32 {
        let next_nl: *const ::core::ffi::c_char = strchr(dumpwalk, '\n' as i32);
        if !next_nl.is_null() {
            {
                // Snapshot native bytes before the logger can dispatch a callback.
                let log_value_0 = (linecnt);
                let log_precision_1 = (next_nl.offset_from(dumpwalk) as i64 as i32) as i32;
                let log_pointer_2 = dumpwalk;
                let log_value_1 = i3_common::logging::string_bytes(
                    (if log_pointer_2.is_null() {
                        None
                    } else {
                        Some(std::ffi::CStr::from_ptr(log_pointer_2))
                    })
                    .map(std::ffi::CStr::to_bytes),
                    usize::try_from(log_precision_1).ok(),
                )
                .to_vec();
                if let (Ok(log_value_1_text),) = (std::str::from_utf8(&log_value_1),) {
                    ::log::debug!(
                        "../src/config_parser.c:parse_config:151 - CONFIG(line {:3}): {}\n",
                        log_value_0,
                        log_value_1_text
                    );
                } else {
                    let mut message = Vec::new();
                    message.extend_from_slice(
                        format!(
                            "../src/config_parser.c:parse_config:151 - CONFIG(line {:3}): ",
                            log_value_0
                        )
                        .as_bytes(),
                    );
                    message.extend_from_slice(&log_value_1);
                    message.extend_from_slice("\n".as_bytes());
                    i3_common::logging::emit(i3_common::logging::Level::Debug, &message);
                }
            };
            dumpwalk = next_nl.offset(1_isize);
            linecnt += 1;
        } else {
            {
                // Snapshot native bytes before the logger can dispatch a callback.
                let log_value_0 = (linecnt);
                let log_pointer_2 = dumpwalk;
                let log_value_1 = i3_common::logging::string_bytes(
                    (if log_pointer_2.is_null() {
                        None
                    } else {
                        Some(std::ffi::CStr::from_ptr(log_pointer_2))
                    })
                    .map(std::ffi::CStr::to_bytes),
                    None,
                )
                .to_vec();
                if let (Ok(log_value_1_text),) = (std::str::from_utf8(&log_value_1),) {
                    ::log::debug!(
                        "../src/config_parser.c:parse_config:154 - CONFIG(line {:3}): {}\n",
                        log_value_0,
                        log_value_1_text
                    );
                } else {
                    let mut message = Vec::new();
                    message.extend_from_slice(
                        format!(
                            "../src/config_parser.c:parse_config:154 - CONFIG(line {:3}): ",
                            log_value_0
                        )
                        .as_bytes(),
                    );
                    message.extend_from_slice(&log_value_1);
                    message.extend_from_slice("\n".as_bytes());
                    i3_common::logging::emit(i3_common::logging::Level::Debug, &message);
                }
            };
            break;
        }
    }
    (*ctx).state = ConfigParserState::Initial;
    (*ctx).statelist.clear();
    (*ctx).statelist.push(ConfigParserState::Initial);
    (*ctx).statelist_idx = 1;
    let mut walk: *const ::core::ffi::c_char = input;
    let len: usize = strlen(input) as usize;
    let mut c: i32 = 0;
    let mut token_handled: bool = false;
    linecnt = 1_i32;
    let mut subcommand_output: ConfigResultIR = ConfigResultIR {
        ctx,
        next_state: ConfigParserState::ForceDisplayUrgencyHintMs,
        has_errors: false,
    };
    cfg_criteria_init(
        &raw mut (*ctx).current_match,
        &raw mut subcommand_output,
        ConfigParserState::Initial,
    );
    while walk.offset_from(input) as i64 as usize <= len {
        while (*walk as i32 == ' ' as i32 || *walk as i32 == '\t' as i32)
            && *walk as i32 != '\0' as i32
        {
            walk = walk.offset(1);
        }
        let grammar = state_tokens((*ctx).state);
        token_handled = false;
        c = 0_i32;
        while c < grammar.len() as i32 {
            let token = &grammar[c as usize];
            if let Some(keyword) = token.name.strip_prefix(b"'") {
                let remaining = std::ffi::CStr::from_ptr(walk).to_bytes();
                if remaining.len() >= keyword.len()
                    && crate::util::text_eq_ignore_case(&remaining[..keyword.len()], keyword)
                {
                    if let Some(identifier) = token.identifier {
                        (&mut *(*ctx).stack).push_string(identifier, keyword);
                    }
                    walk = walk.add(keyword.len());
                    next_state(token, ctx);
                    token_handled = true;
                    break;
                }
            } else if token.name == b"number" {
                let mut end: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                *__errno_location() = 0_i32;
                let num: i64 = strtol(walk, &raw mut end, 10 as ::core::ffi::c_int) as i64;
                if !(*__errno_location() == ERANGE && (num == LONG_MIN || num == LONG_MAX)
                    || *__errno_location() != 0_i32 && num == 0_i64)
                    && !std::ptr::eq(end, walk)
                {
                    if let Some(identifier) = token.identifier {
                        (&mut *(*ctx).stack).push_long(identifier, num);
                    }
                    walk = end;
                    next_state(token, ctx);
                    token_handled = true;
                    break;
                }
            } else {
                if token.name == b"string" || token.name == b"word" {
                    let remaining = std::ffi::CStr::from_ptr(walk);
                    let capture = crate::parser_util::lex_string(
                        remaining.to_bytes(),
                        token.name.first() != Some(&b's'),
                        crate::parser_util::StringGrammar::Config,
                    );
                    walk = walk.add(capture.consumed);
                    if let Some(bytes) = capture.bytes {
                        if let Some(identifier) = token.identifier {
                            (&mut *(*ctx).stack).push_string(identifier, &bytes);
                        }
                        if *walk == b'"' as libc::c_char {
                            walk = walk.add(1);
                        }
                        next_state(token, ctx);
                        token_handled = true;
                        break;
                    }
                }
                if token.name == b"line" {
                    while *walk as i32 != '\0' as i32
                        && *walk as i32 != '\n' as i32
                        && *walk as i32 != '\r' as i32
                    {
                        walk = walk.offset(1);
                    }
                    next_state(token, ctx);
                    token_handled = true;
                    linecnt += 1;
                    walk = walk.offset(1);
                    break;
                } else if token.name == b"end"
                    && (*walk as i32 == '\0' as i32
                        || *walk as i32 == '\n' as i32
                        || *walk as i32 == '\r' as i32)
                {
                    next_state(token, ctx);
                    token_handled = true;
                    cfg_criteria_init(
                        &raw mut (*ctx).current_match,
                        &raw mut subcommand_output,
                        ConfigParserState::Initial,
                    );
                    linecnt += 1;
                    walk = walk.offset(1);
                    break;
                }
            }
            c += 1;
        }
        if !token_handled {
            let names: Vec<_> = grammar.iter().map(|token| token.name).collect();
            let mut message = b"Expected one of these tokens: ".to_vec();
            message.extend_from_slice(&crate::parser_util::expected_tokens(&names, true));
            let errormessage = message;
            let mut error_line = start_of_line(walk, input);
            let line_bytes = std::ffi::CStr::from_ptr(error_line).to_bytes();
            let line_len = line_bytes
                .iter()
                .position(|&byte| byte == b'\n' || byte == b'\r')
                .unwrap_or(line_bytes.len());
            let offset = (walk.offset_from(error_line) as usize).min(line_len);
            let caret: Vec<_> = line_bytes[..line_len]
                .iter()
                .enumerate()
                .map(|(index, &byte)| {
                    if index >= offset {
                        b'^'
                    } else if byte == b'\t' {
                        b'\t'
                    } else {
                        b' '
                    }
                })
                .collect();
            let position = caret;
            {
                // Snapshot native bytes before the logger can dispatch a callback.
                let log_value_0 = errormessage.clone();
                if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
                    ::log::error!("ERROR: CONFIG: {}\n", log_value_0_text);
                } else {
                    let mut message = Vec::new();
                    message.extend_from_slice("ERROR: CONFIG: ".as_bytes());
                    message.extend_from_slice(&log_value_0);
                    message.extend_from_slice("\n".as_bytes());
                    i3_common::logging::emit(i3_common::logging::Level::Error, &message);
                }
            };
            {
                // Snapshot native bytes before the logger can dispatch a callback.
                let filename = (*context).filename.clone();
                let log_value_0 = i3_common::logging::string_bytes(
                    filename.as_deref().map(|path| path.as_os_str().as_bytes()),
                    None,
                )
                .to_vec();
                if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
                    ::log::error!("ERROR: CONFIG: (in file {})\n", log_value_0_text);
                } else {
                    let mut message = Vec::new();
                    message.extend_from_slice("ERROR: CONFIG: (in file ".as_bytes());
                    message.extend_from_slice(&log_value_0);
                    message.extend_from_slice(")\n".as_bytes());
                    i3_common::logging::emit(i3_common::logging::Level::Error, &message);
                }
            };
            let mut error_copy = single_line(std::ffi::CStr::from_ptr(error_line).to_bytes());
            if linecnt > 1_i32 {
                let context_p1_start: *const ::core::ffi::c_char = previous_line(error_line, input);
                let context_p1_line =
                    single_line(std::ffi::CStr::from_ptr(context_p1_start).to_bytes());
                if linecnt > 2_i32 {
                    let context_p2_start: *const ::core::ffi::c_char =
                        previous_line(context_p1_start, input);
                    let context_p2_line =
                        single_line(std::ffi::CStr::from_ptr(context_p2_start).to_bytes());
                    {
                        // Snapshot native bytes before the logger can dispatch a callback.
                        let log_value_0 = (linecnt - 2_i32);
                        let log_value_1 = context_p2_line.clone();
                        if let (Ok(log_value_1_text),) = (std::str::from_utf8(&log_value_1),) {
                            ::log::error!(
                                "ERROR: CONFIG: Line {:3}: {}\n",
                                log_value_0,
                                log_value_1_text
                            );
                        } else {
                            let mut message = Vec::new();
                            message.extend_from_slice(
                                format!("ERROR: CONFIG: Line {:3}: ", log_value_0).as_bytes(),
                            );
                            message.extend_from_slice(&log_value_1);
                            message.extend_from_slice("\n".as_bytes());
                            i3_common::logging::emit(i3_common::logging::Level::Error, &message);
                        }
                    };
                }
                {
                    // Snapshot native bytes before the logger can dispatch a callback.
                    let log_value_0 = (linecnt - 1_i32);
                    let log_value_1 = context_p1_line.clone();
                    if let (Ok(log_value_1_text),) = (std::str::from_utf8(&log_value_1),) {
                        ::log::error!(
                            "ERROR: CONFIG: Line {:3}: {}\n",
                            log_value_0,
                            log_value_1_text
                        );
                    } else {
                        let mut message = Vec::new();
                        message.extend_from_slice(
                            format!("ERROR: CONFIG: Line {:3}: ", log_value_0).as_bytes(),
                        );
                        message.extend_from_slice(&log_value_1);
                        message.extend_from_slice("\n".as_bytes());
                        i3_common::logging::emit(i3_common::logging::Level::Error, &message);
                    }
                };
            }
            {
                // Snapshot native bytes before the logger can dispatch a callback.
                let log_value_0 = (linecnt);
                let log_value_1 = error_copy.clone();
                if let (Ok(log_value_1_text),) = (std::str::from_utf8(&log_value_1),) {
                    ::log::error!(
                        "ERROR: CONFIG: Line {:3}: {}\n",
                        log_value_0,
                        log_value_1_text
                    );
                } else {
                    let mut message = Vec::new();
                    message.extend_from_slice(
                        format!("ERROR: CONFIG: Line {:3}: ", log_value_0).as_bytes(),
                    );
                    message.extend_from_slice(&log_value_1);
                    message.extend_from_slice("\n".as_bytes());
                    i3_common::logging::emit(i3_common::logging::Level::Error, &message);
                }
            };
            {
                // Snapshot native bytes before the logger can dispatch a callback.
                let log_value_0 = position.clone();
                if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
                    ::log::error!("ERROR: CONFIG:           {}\n", log_value_0_text);
                } else {
                    let mut message = Vec::new();
                    message.extend_from_slice("ERROR: CONFIG:           ".as_bytes());
                    message.extend_from_slice(&log_value_0);
                    message.extend_from_slice("\n".as_bytes());
                    i3_common::logging::emit(i3_common::logging::Level::Error, &message);
                }
            };
            let mut i_0: i32 = 0_i32;
            while i_0 < 2_i32 {
                let error_line_end: *const ::core::ffi::c_char = strchr(error_line, '\n' as i32);
                if !error_line_end.is_null()
                    && *error_line_end.offset(1_isize) as i32 != '\0' as i32
                {
                    error_line = error_line_end.offset(1_isize);
                    error_copy = single_line(std::ffi::CStr::from_ptr(error_line).to_bytes());
                    {
                        // Snapshot native bytes before the logger can dispatch a callback.
                        let log_value_0 = (linecnt + i_0 + 1_i32);
                        let log_value_1 = error_copy.clone();
                        if let (Ok(log_value_1_text),) = (std::str::from_utf8(&log_value_1),) {
                            ::log::error!(
                                "ERROR: CONFIG: Line {:3}: {}\n",
                                log_value_0,
                                log_value_1_text
                            );
                        } else {
                            let mut message = Vec::new();
                            message.extend_from_slice(
                                format!("ERROR: CONFIG: Line {:3}: ", log_value_0).as_bytes(),
                            );
                            message.extend_from_slice(&log_value_1);
                            message.extend_from_slice("\n".as_bytes());
                            i3_common::logging::emit(i3_common::logging::Level::Error, &message);
                        }
                    };
                }
                i_0 += 1;
            }
            (*context).has_errors = true;
            while walk.offset_from(input) as i64 as usize <= len && *walk as i32 != '\n' as i32 {
                walk = walk.offset(1);
            }
            (&mut *(*ctx).stack).clear();
            let mut error_token_found: bool = false;
            let mut i_1: i32 = (*ctx).statelist_idx - 1_i32;
            while i_1 >= 0_i32 && !error_token_found {
                let recovery = state_tokens((&(*ctx).statelist)[i_1 as usize]);
                let mut j: i32 = 0_i32;
                while j < recovery.len() as i32 {
                    if recovery[j as usize].name != b"error" {
                        j += 1;
                    } else {
                        next_state(&recovery[j as usize], ctx);
                        error_token_found = true;
                        break;
                    }
                }
                i_1 -= 1;
            }
            if error_token_found {
            } else {
                __assert_fail(
                    c"error_token_found".as_ptr(),
                    c"../src/config_parser.c".as_ptr(),
                    437 as ::core::ffi::c_uint,
                    c"void parse_config(struct parser_ctx *, const char *, struct context *)"
                        .as_ptr(),
                );
            };
        }
    }
}

pub unsafe fn start_config_error_nagbar(configpath: Option<&std::path::Path>, has_errors: bool) {
    let configpath = configpath
        .map_or(b"(null)".as_slice(), |path| path.as_os_str().as_bytes())
        .to_vec();
    let error_path = errorfilename.clone();
    let mut edit_bytes = b"i3-sensible-editor \"".to_vec();
    edit_bytes.extend_from_slice(&configpath);
    edit_bytes.extend_from_slice(b"\" && i3-msg reload\n");
    let mut pager_bytes = b"i3-sensible-pager \"".to_vec();
    pager_bytes.extend_from_slice(
        error_path
            .as_deref()
            .map_or(b"(null)".as_slice(), |path| path.as_os_str().as_bytes()),
    );
    pager_bytes.extend_from_slice(b"\"\n");
    // These terminated owners are the actual child process argv allocation.
    let editaction = std::ffi::CString::new(edit_bytes).unwrap();
    let pageraction = std::ffi::CString::new(pager_bytes).unwrap();
    let font_pattern = std::ffi::CString::new(config.font.pattern().unwrap_or(b"fixed"))
        .expect("native font argument has no NUL");
    let mut argv: [*mut ::core::ffi::c_char; 14] = [
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
        c"-f".as_ptr().cast_mut(),
        font_pattern.as_ptr().cast_mut(),
        c"-t".as_ptr().cast_mut(),
        (if has_errors {
            c"error".as_ptr()
        } else {
            c"warning".as_ptr()
        }) as *mut ::core::ffi::c_char,
        c"-m".as_ptr().cast_mut(),
        (if has_errors {
            c"You have an error in your i3 config file!".as_ptr()
        } else {
            c"Your config is outdated. Please fix the warnings to make sure everything works."
                .as_ptr()
        }) as *mut ::core::ffi::c_char,
        c"-b".as_ptr().cast_mut(),
        c"edit config".as_ptr().cast_mut(),
        editaction.as_ptr().cast_mut(),
        (if error_path.is_some() {
            c"-b".as_ptr()
        } else {
            ::core::ptr::null::<::core::ffi::c_char>()
        }) as *mut ::core::ffi::c_char,
        (if has_errors {
            c"show errors".as_ptr()
        } else {
            c"show warnings".as_ptr()
        }) as *mut ::core::ffi::c_char,
        pageraction.as_ptr().cast_mut(),
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
    ];
    start_nagbar(
        &raw mut config_error_nagbar_pid,
        &raw mut argv as *mut *mut ::core::ffi::c_char,
    );
}
fn store_variable(variables: &mut variables_head, key: &[u8], value: &[u8]) {
    let key = key.split(|byte| *byte == 0).next().unwrap();
    let value = value.split(|byte| *byte == 0).next().unwrap();
    if let Some(variable) = variables.iter_mut().find(|variable| variable.key == key) {
        variable.value = value.to_owned();
        return;
    }
    // Longest names win; a new equal-length key precedes existing entries.
    let position = variables
        .iter()
        .position(|variable| key.len() >= variable.key.len())
        .unwrap_or(variables.len());
    variables.insert(
        position,
        Variable {
            key: key.to_owned(),
            value: value.to_owned(),
        },
    );
}

/// # Safety
/// variables is a live event-thread parser owner, exclusively accessible for
/// this call including diagnostics. Input bytes are copied before callbacks.
unsafe fn upsert_variable(variables: *mut variables_head, key: &[u8], value: &[u8]) {
    let key_text = key.split(|byte| *byte == 0).next().unwrap().to_vec();
    let value_text = value.split(|byte| *byte == 0).next().unwrap().to_vec();
    // Snapshot the old value before diagnostics, so no vector borrow survives a
    // logger callback. Preserve the original debug text and ordering.
    let previous = (&*variables)
        .iter()
        .find(|entry| entry.key == key_text)
        .map(|entry| entry.value.clone());
    if let Some(previous) = previous {
        {
            // Snapshot native bytes before the logger can dispatch a callback.
            let log_value_0 = key_text.clone();
            let log_value_1 = previous;
            let log_value_2 = value_text.clone();
            if let (Ok(log_value_0_text), Ok(log_value_1_text), Ok(log_value_2_text)) = (
                std::str::from_utf8(&log_value_0),
                std::str::from_utf8(&log_value_1),
                std::str::from_utf8(&log_value_2),
            ) {
                ::log::debug!("../src/config_parser.c:upsert_variable:543 - Updated variable: {} = {} -> {}\n", log_value_0_text, log_value_1_text, log_value_2_text);
            } else {
                let mut message = Vec::new();
                message.extend_from_slice(
                    "../src/config_parser.c:upsert_variable:543 - Updated variable: ".as_bytes(),
                );
                message.extend_from_slice(&log_value_0);
                message.extend_from_slice(" = ".as_bytes());
                message.extend_from_slice(&log_value_1);
                message.extend_from_slice(" -> ".as_bytes());
                message.extend_from_slice(&log_value_2);
                message.extend_from_slice("\n".as_bytes());
                i3_common::logging::emit(i3_common::logging::Level::Debug, &message);
            }
        };
    } else {
        {
            // Snapshot native bytes before the logger can dispatch a callback.
            let log_value_0 = key_text.clone();
            let log_value_1 = value_text.clone();
            if let (Ok(log_value_0_text), Ok(log_value_1_text)) = (
                std::str::from_utf8(&log_value_0),
                std::str::from_utf8(&log_value_1),
            ) {
                ::log::debug!(
                    "../src/config_parser.c:upsert_variable:549 - Defined new variable: {} = {}\n",
                    log_value_0_text,
                    log_value_1_text
                );
            } else {
                let mut message = Vec::new();
                message.extend_from_slice(
                    "../src/config_parser.c:upsert_variable:549 - Defined new variable: "
                        .as_bytes(),
                );
                message.extend_from_slice(&log_value_0);
                message.extend_from_slice(" = ".as_bytes());
                message.extend_from_slice(&log_value_1);
                message.extend_from_slice("\n".as_bytes());
                i3_common::logging::emit(i3_common::logging::Level::Debug, &message);
            }
        };
    }
    store_variable(&mut *variables, &key_text, &value_text);
}

#[derive(Default)]
struct ResourceDatabase(Option<xcb_interop::xrm::Database>);
impl ResourceDatabase {
    /// The core connection is read only on its event thread. A configuration
    /// check without a display keeps using the variable's configured fallback.
    unsafe fn get(&mut self, name: &[u8]) -> Option<Vec<u8>> {
        if conn.is_null() {
            return None;
        }
        if self.0.is_none() {
            self.0 = i3_common::display::connection()
                .and_then(xcb_interop::xrm::Database::from_connection);
            if self.0.is_none() {
                ::log::error!("ERROR: Failed to open the resource database.\n");
                self.0 = xcb_interop::xrm::Database::from_string(c"");
                return None;
            }
        }
        let native_name =
            std::ffi::CString::new(name.split(|byte| *byte == 0).next().unwrap()).unwrap();
        self.0
            .as_ref()?
            .resource(&native_name, None)
            .map(std::ffi::CString::into_bytes)
    }
}

struct NativeString(*mut libc::c_char);
impl Drop for NativeString {
    fn drop(&mut self) {
        unsafe { free(self.0.cast()) };
    }
}
struct ConfigFile(*mut FILE);
impl Drop for ConfigFile {
    fn drop(&mut self) {
        unsafe { fclose(self.0) };
    }
}

fn expand_variables(input: &[u8], variables: &[(&[u8], &[u8])]) -> Vec<u8> {
    let bytes = input.split(|byte| *byte == 0).next().unwrap();
    // libc's locale-sensitive search remains the backend. Only its temporary
    // native arguments need termination; the parser's model owns Rust bytes.
    let native_input = std::ffi::CString::new(bytes).unwrap();
    let native_variables: Vec<_> = variables
        .iter()
        .map(|&(key, value)| {
            let key = key.split(|byte| *byte == 0).next().unwrap();
            let value = value.split(|byte| *byte == 0).next().unwrap();
            (std::ffi::CString::new(key).unwrap(), value)
        })
        .collect();
    let mut output = Vec::with_capacity(bytes.len());
    let mut offset = 0;
    while offset < bytes.len() {
        // Offset is within the live native input allocation, including its NUL.
        let remainder = unsafe { native_input.as_ptr().add(offset) };
        let nearest = native_variables
            .iter()
            .filter_map(|(key, value)| {
                if key.as_bytes().is_empty() {
                    return None;
                }
                let found = unsafe { libc::strcasestr(remainder, key.as_ptr()) };
                if found.is_null() {
                    None
                } else {
                    let distance = unsafe { found.offset_from(remainder) } as usize;
                    Some((distance, key.as_bytes().len(), *value))
                }
            })
            .min_by_key(|&(distance, _, _)| distance);
        let Some((distance, key_length, value)) = nearest else {
            output.extend_from_slice(&bytes[offset..]);
            break;
        };
        output.extend_from_slice(&bytes[offset..offset + distance]);
        output.extend_from_slice(value);
        offset += distance + key_length;
    }
    output
}

/// # Safety
/// ctx is a live exclusively borrowed parser context, normally constructed in Rust.
pub unsafe fn free_variables(ctx: *mut parser_ctx) {
    (*ctx).variables.clear();
}

unsafe fn try_chdir(path: &std::path::Path) -> bool {
    let mut result: bool = true;
    let mut dir: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut native_path = std::ffi::CString::new(path.as_os_str().as_bytes())
        .expect("resolved configuration path has no NUL")
        .into_bytes_with_nul();
    dir = dirname(native_path.as_mut_ptr().cast());
    if !dir.is_null() {
        {
            // Snapshot native bytes before the logger can dispatch a callback.
            let log_pointer_1 = dir;
            let log_value_0 = i3_common::logging::string_bytes(
                (if log_pointer_1.is_null() {
                    None
                } else {
                    Some(std::ffi::CStr::from_ptr(log_pointer_1))
                })
                .map(std::ffi::CStr::to_bytes),
                None,
            )
            .to_vec();
            if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
                ::log::info!(
                    "Changing working directory to config file directory {}\n",
                    log_value_0_text
                );
            } else {
                let mut message = Vec::new();
                message.extend_from_slice(
                    "Changing working directory to config file directory ".as_bytes(),
                );
                message.extend_from_slice(&log_value_0);
                message.extend_from_slice("\n".as_bytes());
                i3_common::logging::emit(i3_common::logging::Level::Verbose, &message);
            }
        };
        if chdir(dir) == -1_i32 {
            {
                // Snapshot native bytes before the logger can dispatch a callback.
                let log_pointer_1 = dir;
                let log_value_0 = i3_common::logging::string_bytes(
                    (if log_pointer_1.is_null() {
                        None
                    } else {
                        Some(std::ffi::CStr::from_ptr(log_pointer_1))
                    })
                    .map(std::ffi::CStr::to_bytes),
                    None,
                )
                .to_vec();
                let log_pointer_2 = strerror(*__errno_location());
                let log_value_1 = i3_common::logging::string_bytes(
                    (if log_pointer_2.is_null() {
                        None
                    } else {
                        Some(std::ffi::CStr::from_ptr(log_pointer_2))
                    })
                    .map(std::ffi::CStr::to_bytes),
                    None,
                )
                .to_vec();
                if let (Ok(log_value_0_text), Ok(log_value_1_text)) = (
                    std::str::from_utf8(&log_value_0),
                    std::str::from_utf8(&log_value_1),
                ) {
                    ::log::error!(
                        "ERROR: chdir({}) failed: {}\n",
                        log_value_0_text,
                        log_value_1_text
                    );
                } else {
                    let mut message = Vec::new();
                    message.extend_from_slice("ERROR: chdir(".as_bytes());
                    message.extend_from_slice(&log_value_0);
                    message.extend_from_slice(") failed: ".as_bytes());
                    message.extend_from_slice(&log_value_1);
                    message.extend_from_slice("\n".as_bytes());
                    i3_common::logging::emit(i3_common::logging::Level::Error, &message);
                }
            };
            result = false;
        }
    }
    result
}
/// The caller keeps the file node and FILE live; only native fread touches the
/// temporary buffer pointer. Assign before reading so failed reads retain their
/// initialized prefix and zero-filled tail, matching the original allocation.
#[deny(unsafe_op_in_unsafe_fn)]
unsafe fn read_config_contents(file: *mut IncludedFile, stream: *mut FILE, size: usize) -> bool {
    let Some(allocation) = size
        .checked_add(1)
        .filter(|&size| size <= isize::MAX as usize)
    else {
        return false;
    };
    let mut contents = vec![0; allocation];
    let target = contents.as_mut_ptr();
    // SAFETY: file remains a live Box for this call, and fread only writes the
    // announced size into its newly initialized, size+1-byte owned allocation.
    unsafe {
        (*file).raw_contents = Some(contents);
        fread(target.cast(), 1, size, stream) == size
    }
}

const CONFIG_READ_CAPACITY: usize = 4096;

fn config_prefix(bytes: &[u8]) -> &[u8] {
    bytes.split(|byte| *byte == 0).next().unwrap()
}

struct ConfigFields {
    count: i32,
    key: Vec<u8>,
    resource: Vec<u8>,
    value: Vec<u8>,
}

// Keep the two native scanf formats: locale whitespace, scanset semantics and
// their field widths are part of configuration compatibility.
fn config_fields(bytes: &[u8], resource: bool) -> ConfigFields {
    let native = std::ffi::CString::new(config_prefix(bytes)).unwrap();
    let mut key = vec![0u8; 512];
    let mut name = vec![0u8; 512];
    let mut value = vec![0u8; CONFIG_READ_CAPACITY];
    let count = unsafe {
        if resource {
            sscanf(
                native.as_ptr(),
                c"%511s %511s %4095[^\n]".as_ptr(),
                key.as_mut_ptr().cast::<libc::c_char>(),
                name.as_mut_ptr().cast::<libc::c_char>(),
                value.as_mut_ptr().cast::<libc::c_char>(),
            )
        } else {
            sscanf(
                native.as_ptr(),
                c"%511s %4095[^\n]".as_ptr(),
                key.as_mut_ptr().cast::<libc::c_char>(),
                value.as_mut_ptr().cast::<libc::c_char>(),
            )
        }
    };
    key.truncate(config_prefix(&key).len());
    name.truncate(config_prefix(&name).len());
    value.truncate(config_prefix(&value).len());
    ConfigFields {
        count,
        key,
        resource: name,
        value,
    }
}

#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
struct ConfigFragment {
    bytes: Vec<u8>,
    key: Vec<u8>,
    value: Vec<u8>,
    skip: bool,
    comment: bool,
    continued: bool,
    ignored_comment_continuation: bool,
    #[cfg(test)]
    overlong: bool,
}

struct ConfigPreprocessor {
    buffer: Vec<u8>,
    continuation: Option<usize>,
    previous_key: Vec<u8>,
}
impl ConfigPreprocessor {
    fn new() -> Self {
        Self {
            buffer: vec![0; CONFIG_READ_CAPACITY],
            continuation: None,
            previous_key: Vec::new(),
        }
    }

    /// # Safety
    /// `stream` is a live readable FILE, exclusively used for this preprocessing
    /// pass. Only fgets writes into the remaining initialized native byte span.
    #[deny(unsafe_op_in_unsafe_fn)]
    unsafe fn read(&mut self, stream: *mut FILE) -> Result<Option<ConfigFragment>, ()> {
        if unsafe { feof(stream) } != 0 {
            return Ok(None);
        }
        let start = self.continuation.unwrap_or(0);
        let target = &mut self.buffer[start..];
        if unsafe { fgets(target.as_mut_ptr().cast(), target.len() as i32, stream) }.is_null() {
            return if unsafe { feof(stream) } != 0 {
                Ok(None)
            } else {
                Err(())
            };
        }
        let bytes = config_prefix(&self.buffer);
        let overlong = bytes.last() != Some(&b'\n') && unsafe { feof(stream) } == 0;
        if overlong {
            ::log::error!(
                "ERROR: Your line continuation is too long, it exceeds {} bytes\n",
                CONFIG_READ_CAPACITY as isize
            );
        }
        let fields = config_fields(bytes, false);
        // sscanf leaves its first output unchanged on a failed conversion.
        if fields.count >= 1 {
            self.previous_key = fields.key;
        }
        let skip = fields.count < 1 || self.previous_key.len() < 3;
        let comment = self.previous_key.first() == Some(&b'#');
        let continuation = bytes.windows(2).position(|pair| pair == b"\\\n");
        let continued = continuation.is_some() && !comment;
        self.continuation = if continued { continuation } else { None };
        Ok(Some(ConfigFragment {
            bytes: bytes.to_vec(),
            key: self.previous_key.clone(),
            value: fields.value,
            skip,
            comment,
            continued,
            ignored_comment_continuation: continuation.is_some() && comment,
            #[cfg(test)]
            overlong,
        }))
    }
}

unsafe fn parse_file_inner(
    ctx: *mut parser_ctx,
    filename: &std::path::Path,
    included_file: *mut IncludedFile,
) -> ParseFileResult {
    if !try_chdir(filename) {
        return ParseFileResult::Failed;
    }
    let native_filename = std::ffi::CString::new(filename.as_os_str().as_bytes())
        .expect("resolved configuration path has no NUL");
    let fd = open(native_filename.as_ptr(), O_RDONLY);
    if fd == -1_i32 {
        return ParseFileResult::Failed;
    }
    let mut stbuf: stat = ::core::mem::zeroed();
    if fstat(fd, &raw mut stbuf) == -1_i32 {
        close(fd);
        return ParseFileResult::Failed;
    }
    let fstr = fdopen(fd, c"r".as_ptr());
    if fstr.is_null() {
        let saved_errno = *__errno_location();
        close(fd);
        *__errno_location() = saved_errno;
        return ParseFileResult::Failed;
    }
    let stream = ConfigFile(fstr);
    let mut resources = ResourceDatabase::default();
    let Ok(file_size) = usize::try_from(stbuf.st_size) else {
        return ParseFileResult::Failed;
    };
    if !read_config_contents(included_file, fstr, file_size) {
        return ParseFileResult::Failed;
    }
    rewind(fstr);
    let mut preprocessor = ConfigPreprocessor::new();
    let mut invalid_sets = false;
    let mut buffer_contents = Vec::with_capacity(file_size);
    loop {
        let fragment = match preprocessor.read(fstr) {
            Ok(Some(fragment)) => fragment,
            Ok(None) => break,
            Err(()) => return ParseFileResult::Failed,
        };
        if fragment.continued {
            continue;
        }
        if fragment.ignored_comment_continuation {
            let bytes = &fragment.bytes[..fragment.bytes.len().saturating_sub(1)];
            match std::str::from_utf8(bytes) {
                Ok(text) => ::log::debug!("../src/config_parser.c:parse_file_inner:682 - line continuation in comment is ignored: \"{}\"\n", text),
                Err(_) => i3_common::logging::emit_parts(i3_common::logging::Level::Debug,
                    &[b"../src/config_parser.c:parse_file_inner:682 - line continuation in comment is ignored: \"", bytes, b"\"\n"]),
            }
        }
        buffer_contents.extend_from_slice(&fragment.bytes);
        if fragment.skip || fragment.comment {
            continue;
        }
        let key = fragment.key;
        let value = fragment.value;
        if crate::util::text_eq_ignore_case(&key, b"set") && !value.is_empty() {
            let fields = config_fields(&value, false);
            let v_key = fields.key;
            let v_value = fields.value;
            if fields.count < 1 {
                {
                    // Snapshot native bytes before the logger can dispatch a callback.
                    let log_value_0 = value.clone();
                    if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
                        ::log::error!(
                            "ERROR: Failed to parse variable specification '{}', skipping it.\n",
                            log_value_0_text
                        );
                    } else {
                        let mut message = Vec::new();
                        message.extend_from_slice(
                            "ERROR: Failed to parse variable specification '".as_bytes(),
                        );
                        message.extend_from_slice(&log_value_0);
                        message.extend_from_slice("', skipping it.\n".as_bytes());
                        i3_common::logging::emit(i3_common::logging::Level::Error, &message);
                    }
                };
                invalid_sets = true;
            } else if v_key.first() != Some(&b'$') {
                ::log::error!("ERROR: Malformed variable assignment, name has to start with $\n");
                invalid_sets = true;
            } else {
                upsert_variable(&raw mut (*ctx).variables, &v_key, &v_value);
            }
        } else {
            if !crate::util::text_eq_ignore_case(&key, b"set_from_resource") {
                continue;
            }
            let fields = config_fields(&value, true);
            let v_key_0 = fields.key;
            let res_name = fields.resource;
            let fallback = fields.value;
            if fields.count < 1 {
                {
                    // Snapshot native bytes before the logger can dispatch a callback.
                    let log_value_0 = value.clone();
                    if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
                        ::log::error!(
                            "ERROR: Failed to parse resource specification '{}', skipping it.\n",
                            log_value_0_text
                        );
                    } else {
                        let mut message = Vec::new();
                        message.extend_from_slice(
                            "ERROR: Failed to parse resource specification '".as_bytes(),
                        );
                        message.extend_from_slice(&log_value_0);
                        message.extend_from_slice("', skipping it.\n".as_bytes());
                        i3_common::logging::emit(i3_common::logging::Level::Error, &message);
                    }
                };
                invalid_sets = true;
            } else if v_key_0.first() != Some(&b'$') {
                ::log::error!("ERROR: Malformed variable assignment, name has to start with $\n");
                invalid_sets = true;
            } else {
                let resource_name = res_name.as_slice();
                let res_value = if let Some(resource) = resources.get(resource_name) {
                    resource
                } else {
                    {
                        // Snapshot native bytes before the logger can dispatch a callback.
                        let log_value_0 = res_name.clone();
                        let log_value_1 = fallback.clone();
                        if let (Ok(log_value_0_text), Ok(log_value_1_text)) = (
                            std::str::from_utf8(&log_value_0),
                            std::str::from_utf8(&log_value_1),
                        ) {
                            ::log::debug!("../src/config_parser.c:parse_file_inner:738 - Could not get resource '{}', using fallback '{}'.\n", log_value_0_text, log_value_1_text);
                        } else {
                            let mut message = Vec::new();
                            message.extend_from_slice("../src/config_parser.c:parse_file_inner:738 - Could not get resource '".as_bytes());
                            message.extend_from_slice(&log_value_0);
                            message.extend_from_slice("', using fallback '".as_bytes());
                            message.extend_from_slice(&log_value_1);
                            message.extend_from_slice("'.\n".as_bytes());
                            i3_common::logging::emit(i3_common::logging::Level::Debug, &message);
                        }
                    };
                    fallback.clone()
                };
                upsert_variable(&raw mut (*ctx).variables, &v_key_0, &res_value);
            }
        }
    }
    drop(stream);
    drop(resources);
    let input = buffer_contents;
    let variables: Vec<_> = (*ctx)
        .variables
        .iter()
        .map(|variable| (variable.key.as_slice(), variable.value.as_slice()))
        .collect();
    let expanded = expand_variables(&input, &variables);
    drop(variables);
    (*included_file).variable_replaced_contents = Some(expanded.clone());

    let mut local_context = context {
        filename: Some(filename.to_path_buf()),
        ..Default::default()
    };
    let context = &raw mut local_context;
    parse_config(ctx, &expanded, context);
    if (*ctx).has_errors {
        (*context).has_errors = true;
    }
    check_for_duplicate_bindings(context);
    if (*ctx).use_nagbar && ((*context).has_errors || (*context).has_warnings || invalid_sets) {
        {
            // Snapshot native bytes before the logger can dispatch a callback.
            let log_pointer_1 = i3_version;
            let log_value_0 = i3_common::logging::string_bytes(
                (if log_pointer_1.is_null() {
                    None
                } else {
                    Some(std::ffi::CStr::from_ptr(log_pointer_1))
                })
                .map(std::ffi::CStr::to_bytes),
                None,
            )
            .to_vec();
            if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
                ::log::error!(
                    "ERROR: FYI: You are using i3 version {}\n",
                    log_value_0_text
                );
            } else {
                let mut message = Vec::new();
                message.extend_from_slice("ERROR: FYI: You are using i3 version ".as_bytes());
                message.extend_from_slice(&log_value_0);
                message.extend_from_slice("\n".as_bytes());
                i3_common::logging::emit(i3_common::logging::Level::Error, &message);
            }
        };
        start_config_error_nagbar(Some(filename), (*context).has_errors || invalid_sets);
    }
    let has_errors: bool = (*context).has_errors;

    if has_errors {
        return ParseFileResult::ConfigErrors;
    }
    ParseFileResult::Success
}

/// Own the pathname separately so nested includes/logging do not borrow
/// storage from a queued IncludedFile.
/// # Safety
/// The caller retains the live file node and exclusively borrowed parser context
/// for the whole call, including any callbacks. Queue mutation is serialized.
pub unsafe fn parse_file(
    ctx: *mut parser_ctx,
    filename: &std::path::Path,
    included_file: *mut IncludedFile,
) -> ParseFileResult {
    let filename = filename.to_path_buf();
    let old_directory = NativeString(getcwd(std::ptr::null_mut(), 0));
    let old_dir = old_directory.0;
    let mut result: ParseFileResult = parse_file_inner(ctx, &filename, included_file);
    if chdir(old_dir) == -1_i32 {
        {
            // Snapshot native bytes before the logger can dispatch a callback.
            let log_pointer_1 = old_dir;
            let log_value_0 = i3_common::logging::string_bytes(
                (if log_pointer_1.is_null() {
                    None
                } else {
                    Some(std::ffi::CStr::from_ptr(log_pointer_1))
                })
                .map(std::ffi::CStr::to_bytes),
                None,
            )
            .to_vec();
            let log_pointer_2 = strerror(*__errno_location());
            let log_value_1 = i3_common::logging::string_bytes(
                (if log_pointer_2.is_null() {
                    None
                } else {
                    Some(std::ffi::CStr::from_ptr(log_pointer_2))
                })
                .map(std::ffi::CStr::to_bytes),
                None,
            )
            .to_vec();
            if let (Ok(log_value_0_text), Ok(log_value_1_text)) = (
                std::str::from_utf8(&log_value_0),
                std::str::from_utf8(&log_value_1),
            ) {
                ::log::error!(
                    "ERROR: chdir({}) failed: {}\n",
                    log_value_0_text,
                    log_value_1_text
                );
            } else {
                let mut message = Vec::new();
                message.extend_from_slice("ERROR: chdir(".as_bytes());
                message.extend_from_slice(&log_value_0);
                message.extend_from_slice(") failed: ".as_bytes());
                message.extend_from_slice(&log_value_1);
                message.extend_from_slice("\n".as_bytes());
                i3_common::logging::emit(i3_common::logging::Level::Error, &message);
            }
        };
        result = ParseFileResult::Failed;
    }

    result
}
pub const ERANGE: i32 = 34_i32;
pub const __LONG_MAX__: i64 = 9223372036854775807_i64;

// Internal helpers use Rust linkage rather than foreign symbol lookup.
use crate::util::start_nagbar;
use i3_common::safewrappers::scalloc;
use i3_common::safewrappers::smalloc;

// Canonical Cairo backend ABI; renderer logic remains local.

// Canonical Pango backend ABI; font policy remains local.

pub use libc::{
    __errno_location, chdir, close, dirname, fclose, fdopen, feof, fgets, fread, free, fstat,
    getcwd, open, printf, rewind, sscanf, stat, strcasecmp, strcasestr, strchr, strcmp, strcpy,
    strerror, strlen, strncasecmp, strncpy, strstr, strtol, FILE,
};

use i3_common::native_stdio::__assert_fail;

// Shared model types retain field layout while using one Rust identity.
pub use crate::types::Match;
pub use i3_common::font::Font;

// Internal calls and globals use canonical Rust identities.
use crate::application::conn;
use crate::config::config;
use crate::log::errorfilename;
use crate::version::i3_version;

/// A valid state in the generated configuration parser table.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum ConfigParserState {
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
mod owned_config_tests {
    fn preprocessing_trace(bytes: &[u8], native: bool) -> Vec<super::ConfigFragment> {
        let mut input = bytes.to_vec();
        if input.is_empty() {
            input.push(0);
        }
        let raw = unsafe { libc::fmemopen(input.as_mut_ptr().cast(), bytes.len(), c"r".as_ptr()) };
        assert!(!raw.is_null());
        let stream = super::ConfigFile(raw);
        let mut result = Vec::new();
        if !native {
            let mut state = super::ConfigPreprocessor::new();
            while let Some(fragment) = unsafe { state.read(stream.0).unwrap() } {
                result.push(fragment);
            }
            return result;
        }
        // Independent oracle: the retired fgets/scanf/strstr pointer loop,
        // including persistent scratch outputs and the reduced read capacity.
        unsafe {
            let mut buffer = [0 as libc::c_char; 4096];
            let mut key = [0 as libc::c_char; 512];
            let mut value = [0 as libc::c_char; 4096];
            let mut continuation: *mut libc::c_char = std::ptr::null_mut();
            while libc::feof(stream.0) == 0 {
                if continuation.is_null() {
                    continuation = buffer.as_mut_ptr();
                }
                let remaining = 4096 - continuation.offset_from(buffer.as_ptr()) as usize;
                if libc::fgets(continuation, remaining as i32, stream.0).is_null() {
                    assert_ne!(libc::feof(stream.0), 0);
                    break;
                }
                let length = libc::strlen(buffer.as_ptr());
                let overlong = buffer[length.saturating_sub(1)] != b'\n' as libc::c_char
                    && libc::feof(stream.0) == 0;
                value[0] = 0;
                let count = libc::sscanf(
                    buffer.as_ptr(),
                    c"%511s %4095[^\n]".as_ptr(),
                    key.as_mut_ptr(),
                    value.as_mut_ptr(),
                );
                let skip = count < 1 || libc::strlen(key.as_ptr()) < 3;
                let comment = key[0] == b'#' as libc::c_char;
                value[4095] = b'\n' as libc::c_char;
                continuation = libc::strstr(buffer.as_ptr(), c"\\\n".as_ptr());
                let continued = !continuation.is_null() && !comment;
                let ignored_comment_continuation = !continuation.is_null() && comment;
                if comment {
                    continuation = std::ptr::null_mut();
                }
                result.push(super::ConfigFragment {
                    bytes: std::ffi::CStr::from_ptr(buffer.as_ptr())
                        .to_bytes()
                        .to_vec(),
                    key: std::ffi::CStr::from_ptr(key.as_ptr()).to_bytes().to_vec(),
                    value: std::ffi::CStr::from_ptr(value.as_ptr()).to_bytes().to_vec(),
                    skip,
                    comment,
                    continued,
                    ignored_comment_continuation,
                    overlong,
                });
            }
        }
        result
    }

    #[test]
    fn owned_preprocessing_matches_native_chunks_continuations_and_scan_scratch() {
        let mut corpus = vec![
            Vec::new(),
            b"set $x value".to_vec(),
            b"set $x a\\\nb\n".to_vec(),
            b"# comment\\\n\nset $x value\n".to_vec(),
            b"# comment\n\0\\\nignored\nset $x \xff\n".to_vec(),
            b"set $x before\0ignored\\\nset $y after\n".to_vec(),
            b"\x0b\x0cset\t$x\r\nset_from_resource $r\nset $missing\n".to_vec(),
            b"\\\n\\\nset $x joined\n".to_vec(),
        ];
        for length in [4093, 4094, 4095, 4096, 8190] {
            corpus.push(vec![b'x'; length]);
            let mut line = vec![b'x'; length];
            line.push(b'\n');
            corpus.push(line);
        }
        for prefix in [0, 1, 511, 4092, 4093] {
            let mut line = b"set $x ".to_vec();
            line.extend(std::iter::repeat_n(b'a', prefix));
            line.extend_from_slice(b"\\\ncontinued\n");
            corpus.push(line);
        }
        for input in corpus {
            assert_eq!(
                preprocessing_trace(&input, false),
                preprocessing_trace(&input, true),
                "input length {}: {:?}",
                input.len(),
                &input[..input.len().min(48)]
            );
        }
        let exact = preprocessing_trace(&vec![b'x'; 4095], false);
        assert!(exact[0].overlong);
        let short = preprocessing_trace(&vec![b'x'; 4094], false);
        assert!(!short[0].overlong);
    }

    #[test]
    fn preprocessing_read_failure_retains_previous_fragment_state() {
        let mut source = b"# retained\\\n".to_vec();
        let raw =
            unsafe { libc::fmemopen(source.as_mut_ptr().cast(), source.len(), c"r".as_ptr()) };
        assert!(!raw.is_null());
        let input = super::ConfigFile(raw);
        let mut state = super::ConfigPreprocessor::new();
        assert!(unsafe { state.read(input.0) }.unwrap().is_some());
        let before = (
            state.buffer.clone(),
            state.previous_key.clone(),
            state.continuation,
        );
        let mut output_bytes = [0u8; 8];
        let raw = unsafe {
            libc::fmemopen(
                output_bytes.as_mut_ptr().cast(),
                output_bytes.len(),
                c"w".as_ptr(),
            )
        };
        assert!(!raw.is_null());
        let output = super::ConfigFile(raw);
        assert!(unsafe { state.read(output.0) }.is_err());
        assert_eq!(
            std::io::Error::last_os_error().raw_os_error(),
            Some(libc::EBADF)
        );
        assert_eq!(
            (state.buffer, state.previous_key, state.continuation),
            before
        );
    }

    #[test]
    fn owned_scan_fields_keep_native_widths_whitespace_missing_values_and_prefixes() {
        let fields = super::config_fields(b"\x0b\x0cset\t$x value\r\n", false);
        assert_eq!(
            (fields.count, fields.key.as_slice(), fields.value.as_slice()),
            (2, b"set".as_slice(), b"$x value\r".as_slice())
        );
        let fields = super::config_fields(b"$x", false);
        assert_eq!(
            (fields.count, fields.key.as_slice(), fields.value.as_slice()),
            (1, b"$x".as_slice(), b"".as_slice())
        );
        let fields = super::config_fields(b"$r resource.name fallback\0ignored", true);
        assert_eq!(
            (
                fields.count,
                fields.key.as_slice(),
                fields.resource.as_slice(),
                fields.value.as_slice()
            ),
            (
                3,
                b"$r".as_slice(),
                b"resource.name".as_slice(),
                b"fallback".as_slice()
            )
        );
        let fields = super::config_fields(b"$r", true);
        assert_eq!(
            (
                fields.count,
                fields.resource.as_slice(),
                fields.value.as_slice()
            ),
            (1, b"".as_slice(), b"".as_slice())
        );
        let mut wide = vec![b'k'; 512];
        wide.extend_from_slice(b" tail");
        let fields = super::config_fields(&wide, false);
        assert_eq!(fields.key.len(), 511);
        assert_eq!(fields.value, b"k tail");
    }

    #[test]
    fn owned_file_reads_keep_partial_zero_tail_binary_suffix_and_empty_state() {
        let mut file = crate::config::IncludedFile::new(std::path::PathBuf::from(
            std::ffi::OsStr::from_bytes(b"config-\xff"),
        ));
        assert!(file.raw_contents.is_none());
        assert!(file.variable_replaced_contents.is_none());
        let mut input = [b'a', 0xff];
        let stream =
            unsafe { libc::fmemopen(input.as_mut_ptr().cast(), input.len(), c"r".as_ptr()) };
        assert!(!stream.is_null());
        let _stream = super::ConfigFile(stream);
        assert!(!unsafe { super::read_config_contents(&mut file, stream, 6) });
        assert_eq!(
            file.raw_contents.as_deref(),
            Some(b"a\xff\0\0\0\0\0".as_slice())
        );
        assert_eq!(file.raw_contents_prefix(), Some(b"a\xff".as_slice()));
        assert!(file.variable_replaced_contents.is_none());
        let mut input = [b'a', 0, b'b', 0xff];
        let stream =
            unsafe { libc::fmemopen(input.as_mut_ptr().cast(), input.len(), c"r".as_ptr()) };
        assert!(!stream.is_null());
        let _stream = super::ConfigFile(stream);
        assert!(unsafe { super::read_config_contents(&mut file, stream, 4) });
        assert_eq!(file.raw_contents.as_deref(), Some(b"a\0b\xff\0".as_slice()));
        assert_eq!(file.raw_contents_prefix(), Some(b"a".as_slice()));
        assert!(unsafe { super::read_config_contents(&mut file, stream, 0) });
        assert_eq!(file.raw_contents.as_deref(), Some(b"\0".as_slice()));
        assert_eq!(file.raw_contents_prefix(), Some(b"".as_slice()));
        file.variable_replaced_contents = Some(Vec::new());
        assert_eq!(
            file.variable_replaced_contents.as_deref(),
            Some(b"".as_slice())
        );
    }

    use super::*;
    #[test]
    fn variable_storage_keeps_longest_first_case_sensitive_updates_and_nonrecursive_expansion() {
        let mut variables = Vec::new();
        store_variable(&mut variables, b"$A", b"short");
        store_variable(&mut variables, b"$ABC", b"long");
        store_variable(&mut variables, b"$X", b"$A");
        store_variable(&mut variables, b"$a", b"case");
        store_variable(&mut variables, b"$ABC", b"updated");
        assert_eq!(variables.len(), 4);
        assert_eq!(variables[0].key, b"$ABC");
        let pairs: Vec<_> = variables
            .iter()
            .map(|entry| (entry.key.as_slice(), entry.value.as_slice()))
            .collect();
        assert_eq!(expand_variables(b"$abc $A $X", &pairs), b"updated case $A");
        assert_eq!(expand_variables(b"nothing\xff", &pairs), b"nothing\xff");
    }
    #[test]
    fn bounded_line_context_handles_first_byte_crlf_and_previous_lines() {
        assert_eq!(line_start(b"abc", 0), 0);
        assert_eq!(line_start(b"abc", usize::MAX), 0);
        assert_eq!(line_start(b"a\r\nb", 2), 3);
        assert_eq!(single_line(b"first\r\nsecond"), b"first\r");
        assert_eq!(single_line(b"\xffline\0ignored"), b"\xffline");
        assert_eq!(single_line(b""), b"");
        let input = c"a\n\nb";
        unsafe {
            assert_eq!(
                previous_line(input.as_ptr(), input.as_ptr()),
                input.as_ptr()
            );
            assert_eq!(
                previous_line(input.as_ptr().add(3), input.as_ptr()),
                input.as_ptr().add(2)
            );
        }
    }
    #[test]
    fn generated_config_tables_keep_complete_states_and_dispatch_ids() {
        let states = &tokens;
        assert_eq!(states.len(), 117);
        let mut calls = std::collections::BTreeSet::new();
        for table in states {
            assert!(!table.is_empty());
            for entry in *table {
                if entry.next_state == ConfigParserState::Call {
                    calls.insert(entry.extra.call_identifier);
                } else {
                    assert!((entry.next_state as usize) < states.len());
                }
            }
        }
        assert_eq!(
            calls.into_iter().collect::<Vec<_>>(),
            (0..156).collect::<Vec<u16>>()
        );
    }
}
