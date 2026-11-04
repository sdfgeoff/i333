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
use crate::parser_util::parser_clear_stack;
use crate::parser_util::parser_get_long;
use crate::parser_util::parser_get_string;
use crate::parser_util::parser_push_long;
use crate::parser_util::parser_push_string;
use crate::types::ParseFileResult;
use i3_common::safewrappers::sasprintf;
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
#[repr(C)]
pub struct token {
    pub name: *mut ::core::ffi::c_char,
    pub identifier: *mut ::core::ffi::c_char,
    pub next_state: ConfigParserState,
    pub extra: ConfigTokenData,
}
pub use crate::commands_parser::ParserTokenData as ConfigTokenData;

pub type cmdp_token_ptr = tokenptr;
#[derive(Clone, Copy)]
#[repr(C)]
pub struct tokenptr {
    pub array: *mut cmdp_token,
    pub n: ::core::ffi::c_int,
}
pub const LONG_MAX: ::core::ffi::c_long = __LONG_MAX__;
pub const LONG_MIN: ::core::ffi::c_long = -__LONG_MAX__ - 1 as ::core::ffi::c_long;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub static mut config_error_nagbar_pid: libc::pid_t = -(1 as libc::pid_t);
static mut tokens_FORCE_DISPLAY_URGENCY_HINT_MS: [cmdp_token; 2] = [
    token {
        name: c"'ms".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::ForceDisplayUrgencyHintMs,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_FLOATING_MAXIMUM_SIZE_HEIGHT: [cmdp_token; 1] = [token {
    name: c"number".as_ptr().cast_mut(),
    identifier: c"height".as_ptr().cast_mut(),
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 1 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_FLOATING_MINIMUM_SIZE_HEIGHT: [cmdp_token; 1] = [token {
    name: c"number".as_ptr().cast_mut(),
    identifier: c"height".as_ptr().cast_mut(),
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 2 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_BAR_STRIP_WORKSPACE_NUMBERS: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"value".as_ptr().cast_mut(),
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 3 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_FLOATING_MAXIMUM_SIZE_WIDTH: [cmdp_token; 1] = [token {
    name: c"number".as_ptr().cast_mut(),
    identifier: c"width".as_ptr().cast_mut(),
    next_state: ConfigParserState::FloatingMaximumSizeX,
    extra: ConfigTokenData {
        call_identifier: 0 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_FLOATING_MINIMUM_SIZE_WIDTH: [cmdp_token; 1] = [token {
    name: c"number".as_ptr().cast_mut(),
    identifier: c"width".as_ptr().cast_mut(),
    next_state: ConfigParserState::FloatingMinimumSizeX,
    extra: ConfigTokenData {
        call_identifier: 0 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_BAR_BINDING_MODE_INDICATOR: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"value".as_ptr().cast_mut(),
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 4 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_BAR_WORKSPACE_MIN_WIDTH_PX: [cmdp_token; 2] = [
    token {
        name: c"'px".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::BarWorkspaceMinWidthPx,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 5 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_FOCUS_ON_WINDOW_ACTIVATION: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"mode".as_ptr().cast_mut(),
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 6 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_FORCE_DISPLAY_URGENCY_HINT: [cmdp_token; 1] = [token {
    name: c"number".as_ptr().cast_mut(),
    identifier: c"duration_ms".as_ptr().cast_mut(),
    next_state: ConfigParserState::ForceDisplayUrgencyHintMs,
    extra: ConfigTokenData {
        call_identifier: 0 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_TILING_DRAG_SWAP_MODIFIER: [cmdp_token; 10] = [
    token {
        name: c"'Mod1".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ConfigParserState::TilingDragSwapModifier,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Mod2".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ConfigParserState::TilingDragSwapModifier,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Mod3".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ConfigParserState::TilingDragSwapModifier,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Mod4".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ConfigParserState::TilingDragSwapModifier,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Mod5".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ConfigParserState::TilingDragSwapModifier,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Shift".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ConfigParserState::TilingDragSwapModifier,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Control".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ConfigParserState::TilingDragSwapModifier,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Ctrl".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ConfigParserState::TilingDragSwapModifier,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'+".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::TilingDragSwapModifier,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 7 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_BAR_STRIP_WORKSPACE_NAME: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"value".as_ptr().cast_mut(),
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 8 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_CRITERION_FROM_STR_START: [cmdp_token; 3] = [
    token {
        name: c"'\"".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::CriterionFromStr,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'auto".as_ptr().cast_mut(),
        identifier: c"kind".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 9 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'user".as_ptr().cast_mut(),
        identifier: c"kind".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 10 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_DEFAULT_BORDER_PIXELS_PX: [cmdp_token; 2] = [
    token {
        name: c"'px".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::DefaultBorderPixelsPx,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 11 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_WORKSPACE_BACK_AND_FORTH: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"value".as_ptr().cast_mut(),
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 12 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_ASSIGN_WORKSPACE_NUMBER: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"number".as_ptr().cast_mut(),
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 13 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_BAR_WORKSPACE_MIN_WIDTH: [cmdp_token; 1] = [token {
    name: c"number".as_ptr().cast_mut(),
    identifier: c"width".as_ptr().cast_mut(),
    next_state: ConfigParserState::BarWorkspaceMinWidthPx,
    extra: ConfigTokenData {
        call_identifier: 0 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_FLOATING_MAXIMUM_SIZE_X: [cmdp_token; 1] = [token {
    name: c"'x".as_ptr().cast_mut(),
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: ConfigParserState::FloatingMaximumSizeHeight,
    extra: ConfigTokenData {
        call_identifier: 0 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_FLOATING_MINIMUM_SIZE_X: [cmdp_token; 1] = [token {
    name: c"'x".as_ptr().cast_mut(),
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: ConfigParserState::FloatingMinimumSizeHeight,
    extra: ConfigTokenData {
        call_identifier: 0 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_POPUP_DURING_FULLSCREEN: [cmdp_token; 4] = [
    token {
        name: c"'ignore".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 14 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'leave_fullscreen".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 15 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'all".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 16 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'smart".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 17 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_BAR_COLORS_IGNORE_LINE: [cmdp_token; 1] = [token {
    name: c"line".as_ptr().cast_mut(),
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: ConfigParserState::BarColors,
    extra: ConfigTokenData {
        call_identifier: 0 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_CRITERION_FROM_STR_END: [cmdp_token; 1] = [token {
    name: c"'\"".as_ptr().cast_mut(),
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 18 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_BAR_COLORS_BACKGROUND: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"background".as_ptr().cast_mut(),
    next_state: ConfigParserState::BarColorsText,
    extra: ConfigTokenData {
        call_identifier: 0 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_BAR_WORKSPACE_BUTTONS: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"value".as_ptr().cast_mut(),
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 19 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_BAR_WORKSPACE_COMMAND: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"command".as_ptr().cast_mut(),
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 20 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_DEFAULT_BORDER_PIXELS: [cmdp_token; 2] = [
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 21 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"number".as_ptr().cast_mut(),
        identifier: c"width".as_ptr().cast_mut(),
        next_state: ConfigParserState::DefaultBorderPixelsPx,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_WORKSPACE_OUTPUT_WORD: [cmdp_token; 2] = [
    token {
        name: c"word".as_ptr().cast_mut(),
        identifier: c"output".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 22 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::Initial,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_BAR_SEPARATOR_SYMBOL: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"separator".as_ptr().cast_mut(),
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 23 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_FORCE_FOCUS_WRAPPING: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"value".as_ptr().cast_mut(),
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 24 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_BAR_BINDSYM_COMMAND: [cmdp_token; 2] = [
    token {
        name: c"'--release".as_ptr().cast_mut(),
        identifier: c"release".as_ptr().cast_mut(),
        next_state: ConfigParserState::BarBindsymCommand,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"string".as_ptr().cast_mut(),
        identifier: c"command".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 25 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_BAR_TRAY_PADDING_PX: [cmdp_token; 2] = [
    token {
        name: c"'px".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::BarTrayPaddingPx,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 26 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_DEFAULT_ORIENTATION: [cmdp_token; 3] = [
    token {
        name: c"'horizontal".as_ptr().cast_mut(),
        identifier: c"orientation".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 27 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'vertical".as_ptr().cast_mut(),
        identifier: c"orientation".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 28 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'auto".as_ptr().cast_mut(),
        identifier: c"orientation".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 29 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_FOCUS_FOLLOWS_MOUSE: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"value".as_ptr().cast_mut(),
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 30 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_BAR_PADDING_BOTTOM: [cmdp_token; 3] = [
    token {
        name: c"'px".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::BarPaddingBottom,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"number".as_ptr().cast_mut(),
        identifier: c"left".as_ptr().cast_mut(),
        next_state: ConfigParserState::BarPaddingLeft,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 31 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_BAR_STATUS_COMMAND: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"command".as_ptr().cast_mut(),
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 32 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_BAR_WHEEL_DOWN_CMD: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"command".as_ptr().cast_mut(),
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 33 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_COLOR_CHILD_BORDER: [cmdp_token; 2] = [
    token {
        name: c"word".as_ptr().cast_mut(),
        identifier: c"child_border".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 34 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 35 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_CRITERION_FROM_STR: [cmdp_token; 2] = [
    token {
        name: c"'auto".as_ptr().cast_mut(),
        identifier: c"kind".as_ptr().cast_mut(),
        next_state: ConfigParserState::CriterionFromStrEnd,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'user".as_ptr().cast_mut(),
        identifier: c"kind".as_ptr().cast_mut(),
        next_state: ConfigParserState::CriterionFromStrEnd,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_FOR_WINDOW_COMMAND: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"command".as_ptr().cast_mut(),
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 36 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_BAR_COLORS_BORDER: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"border".as_ptr().cast_mut(),
    next_state: ConfigParserState::BarColorsBackground,
    extra: ConfigTokenData {
        call_identifier: 0 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_BAR_COLORS_SINGLE: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"color".as_ptr().cast_mut(),
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 37 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_BAR_PADDING_RIGHT: [cmdp_token; 3] = [
    token {
        name: c"'px".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::BarPaddingRight,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"number".as_ptr().cast_mut(),
        identifier: c"bottom".as_ptr().cast_mut(),
        next_state: ConfigParserState::BarPaddingBottom,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 38 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_FLOATING_MODIFIER: [cmdp_token; 10] = [
    token {
        name: c"'Mod1".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ConfigParserState::FloatingModifier,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Mod2".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ConfigParserState::FloatingModifier,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Mod3".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ConfigParserState::FloatingModifier,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Mod4".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ConfigParserState::FloatingModifier,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Mod5".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ConfigParserState::FloatingModifier,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Shift".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ConfigParserState::FloatingModifier,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Control".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ConfigParserState::FloatingModifier,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Ctrl".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ConfigParserState::FloatingModifier,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'+".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::FloatingModifier,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 39 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_HIDE_EDGE_BORDERS: [cmdp_token; 12] = [
    token {
        name: c"'none".as_ptr().cast_mut(),
        identifier: c"hide_borders".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 40 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'vertical".as_ptr().cast_mut(),
        identifier: c"hide_borders".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 41 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'horizontal".as_ptr().cast_mut(),
        identifier: c"hide_borders".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 42 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'both".as_ptr().cast_mut(),
        identifier: c"hide_borders".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 43 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'smart_no_gaps".as_ptr().cast_mut(),
        identifier: c"hide_borders".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 44 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'smart".as_ptr().cast_mut(),
        identifier: c"hide_borders".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 45 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'1".as_ptr().cast_mut(),
        identifier: c"hide_borders".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 46 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'yes".as_ptr().cast_mut(),
        identifier: c"hide_borders".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 47 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'true".as_ptr().cast_mut(),
        identifier: c"hide_borders".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 48 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'on".as_ptr().cast_mut(),
        identifier: c"hide_borders".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 49 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'enable".as_ptr().cast_mut(),
        identifier: c"hide_borders".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 50 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'active".as_ptr().cast_mut(),
        identifier: c"hide_borders".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 51 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_WORKSPACE_COMMAND: [cmdp_token; 2] = [
    token {
        name: c"'output".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::WorkspaceOutputWord,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'gaps".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::Gaps,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_ASSIGN_WORKSPACE: [cmdp_token; 5] = [
    token {
        name: c"'→".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::AssignWorkspace,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'output".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::AssignOutput,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'workspace".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::AssignWorkspace,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'number".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::AssignWorkspaceNumber,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"string".as_ptr().cast_mut(),
        identifier: c"workspace".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 52 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_BAR_COLORS_BRACE: [cmdp_token; 2] = [
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::BarColorsBrace,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'{".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::BarColors,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_BAR_HIDDEN_STATE: [cmdp_token; 2] = [
    token {
        name: c"'hide".as_ptr().cast_mut(),
        identifier: c"hidden_state".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 53 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'show".as_ptr().cast_mut(),
        identifier: c"hidden_state".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 54 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_BAR_PADDING_LEFT: [cmdp_token; 2] = [
    token {
        name: c"'px".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::BarPaddingLeft,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 55 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_BAR_TRAY_PADDING: [cmdp_token; 1] = [token {
    name: c"number".as_ptr().cast_mut(),
    identifier: c"padding_px".as_ptr().cast_mut(),
    next_state: ConfigParserState::BarTrayPaddingPx,
    extra: ConfigTokenData {
        call_identifier: 0 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_BAR_WHEEL_UP_CMD: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"command".as_ptr().cast_mut(),
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 56 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_COLOR_BACKGROUND: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"background".as_ptr().cast_mut(),
    next_state: ConfigParserState::ColorText,
    extra: ConfigTokenData {
        call_identifier: 0 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_IPC_KILL_TIMEOUT: [cmdp_token; 1] = [token {
    name: c"number".as_ptr().cast_mut(),
    identifier: c"timeout".as_ptr().cast_mut(),
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 57 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_MODE_BINDCOMMAND: [cmdp_token; 5] = [
    token {
        name: c"'--release".as_ptr().cast_mut(),
        identifier: c"release".as_ptr().cast_mut(),
        next_state: ConfigParserState::ModeBindcommand,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'--border".as_ptr().cast_mut(),
        identifier: c"border".as_ptr().cast_mut(),
        next_state: ConfigParserState::ModeBindcommand,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'--whole-window".as_ptr().cast_mut(),
        identifier: c"whole_window".as_ptr().cast_mut(),
        next_state: ConfigParserState::ModeBindcommand,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'--exclude-titlebar".as_ptr().cast_mut(),
        identifier: c"exclude_titlebar".as_ptr().cast_mut(),
        next_state: ConfigParserState::ModeBindcommand,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"string".as_ptr().cast_mut(),
        identifier: c"command".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 58 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_MODE_IGNORE_LINE: [cmdp_token; 1] = [token {
    name: c"line".as_ptr().cast_mut(),
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: ConfigParserState::Mode,
    extra: ConfigTokenData {
        call_identifier: 0 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_TILING_DRAG_MODE: [cmdp_token; 3] = [
    token {
        name: c"'modifier".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: ConfigParserState::TilingDragMode,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'titlebar".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: ConfigParserState::TilingDragMode,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 59 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_WORKSPACE_LAYOUT: [cmdp_token; 4] = [
    token {
        name: c"'default".as_ptr().cast_mut(),
        identifier: c"layout".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 60 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'stacking".as_ptr().cast_mut(),
        identifier: c"layout".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 61 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'stacked".as_ptr().cast_mut(),
        identifier: c"layout".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 62 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'tabbed".as_ptr().cast_mut(),
        identifier: c"layout".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 63 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_BAR_BAR_COMMAND: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"command".as_ptr().cast_mut(),
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 64 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_BAR_COLORS_TEXT: [cmdp_token; 2] = [
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 65 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"word".as_ptr().cast_mut(),
        identifier: c"text".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 66 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_BAR_IGNORE_LINE: [cmdp_token; 1] = [token {
    name: c"line".as_ptr().cast_mut(),
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: ConfigParserState::Bar,
    extra: ConfigTokenData {
        call_identifier: 0 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_BAR_PADDING_TOP: [cmdp_token; 3] = [
    token {
        name: c"'px".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::BarPaddingTop,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"number".as_ptr().cast_mut(),
        identifier: c"right_or_right_and_left".as_ptr().cast_mut(),
        next_state: ConfigParserState::BarPaddingRight,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 67 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_BAR_SOCKET_PATH: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"path".as_ptr().cast_mut(),
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 68 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_BAR_TRAY_OUTPUT: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"output".as_ptr().cast_mut(),
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 69 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_COLOR_INDICATOR: [cmdp_token; 2] = [
    token {
        name: c"word".as_ptr().cast_mut(),
        identifier: c"indicator".as_ptr().cast_mut(),
        next_state: ConfigParserState::ColorChildBorder,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 70 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_DISABLE_RANDR15: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"value".as_ptr().cast_mut(),
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 71 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_GAPS_WITH_SCOPE: [cmdp_token; 1] = [token {
    name: c"number".as_ptr().cast_mut(),
    identifier: c"value".as_ptr().cast_mut(),
    next_state: ConfigParserState::GapsEnd,
    extra: ConfigTokenData {
        call_identifier: 0 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_CRITERION_FROM: [cmdp_token; 1] = [token {
    name: c"'=".as_ptr().cast_mut(),
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: ConfigParserState::CriterionFromStrStart,
    extra: ConfigTokenData {
        call_identifier: 0 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_DEFAULT_BORDER: [cmdp_token; 4] = [
    token {
        name: c"'normal".as_ptr().cast_mut(),
        identifier: c"border".as_ptr().cast_mut(),
        next_state: ConfigParserState::DefaultBorderPixels,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'pixel".as_ptr().cast_mut(),
        identifier: c"border".as_ptr().cast_mut(),
        next_state: ConfigParserState::DefaultBorderPixels,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'1pixel".as_ptr().cast_mut(),
        identifier: c"border".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 72 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'none".as_ptr().cast_mut(),
        identifier: c"border".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 73 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_FOCUS_WRAPPING: [cmdp_token; 14] = [
    token {
        name: c"'1".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 74 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'yes".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 75 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'true".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 76 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'on".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 77 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'enable".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 78 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'active".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 79 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'0".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 80 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'no".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 81 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'false".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 82 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'off".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 83 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'disable".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 84 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'inactive".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 85 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'force".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 86 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'workspace".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 87 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_FORCE_XINERAMA: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"value".as_ptr().cast_mut(),
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 88 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_ASSIGN_OUTPUT: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"output".as_ptr().cast_mut(),
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 89 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_CRITERION_STR: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"cvalue".as_ptr().cast_mut(),
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 90 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_MOUSE_WARPING: [cmdp_token; 2] = [
    token {
        name: c"'none".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 91 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'output".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 92 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_RESTART_STATE: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"path".as_ptr().cast_mut(),
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 93 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_SMART_BORDERS: [cmdp_token; 7] = [
    token {
        name: c"'1".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 94 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'yes".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 95 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'true".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 96 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'on".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 97 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'enable".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 98 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'active".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 99 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'no_gaps".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 100 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_BAR_MODIFIER: [cmdp_token; 12] = [
    token {
        name: c"'off".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 101 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'none".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 102 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Mod1".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ConfigParserState::BarModifier,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Mod2".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ConfigParserState::BarModifier,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Mod3".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ConfigParserState::BarModifier,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Mod4".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ConfigParserState::BarModifier,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Mod5".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ConfigParserState::BarModifier,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Shift".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ConfigParserState::BarModifier,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Control".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ConfigParserState::BarModifier,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Ctrl".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ConfigParserState::BarModifier,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'+".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::BarModifier,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 103 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_BAR_POSITION: [cmdp_token; 2] = [
    token {
        name: c"'top".as_ptr().cast_mut(),
        identifier: c"position".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 104 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'bottom".as_ptr().cast_mut(),
        identifier: c"position".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 105 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_COLOR_BORDER: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"border".as_ptr().cast_mut(),
    next_state: ConfigParserState::ColorBackground,
    extra: ConfigTokenData {
        call_identifier: 0 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_COLOR_SINGLE: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"color".as_ptr().cast_mut(),
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 106 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_FAKE_OUTPUTS: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"outputs".as_ptr().cast_mut(),
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 107 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_MODE_BINDING: [cmdp_token; 20] = [
    token {
        name: c"'--release".as_ptr().cast_mut(),
        identifier: c"release".as_ptr().cast_mut(),
        next_state: ConfigParserState::ModeBinding,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'--border".as_ptr().cast_mut(),
        identifier: c"border".as_ptr().cast_mut(),
        next_state: ConfigParserState::ModeBinding,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'--whole-window".as_ptr().cast_mut(),
        identifier: c"whole_window".as_ptr().cast_mut(),
        next_state: ConfigParserState::ModeBinding,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'--exclude-titlebar".as_ptr().cast_mut(),
        identifier: c"exclude_titlebar".as_ptr().cast_mut(),
        next_state: ConfigParserState::ModeBinding,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Mod1".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ConfigParserState::ModeBinding,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Mod2".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ConfigParserState::ModeBinding,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Mod3".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ConfigParserState::ModeBinding,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Mod4".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ConfigParserState::ModeBinding,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Mod5".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ConfigParserState::ModeBinding,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Shift".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ConfigParserState::ModeBinding,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Control".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ConfigParserState::ModeBinding,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Ctrl".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ConfigParserState::ModeBinding,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Mode_switch".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ConfigParserState::ModeBinding,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Group1".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ConfigParserState::ModeBinding,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Group2".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ConfigParserState::ModeBinding,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Group3".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ConfigParserState::ModeBinding,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Group4".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ConfigParserState::ModeBinding,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'$mod".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ConfigParserState::ModeBinding,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'+".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::ModeBinding,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"word".as_ptr().cast_mut(),
        identifier: c"key".as_ptr().cast_mut(),
        next_state: ConfigParserState::ModeBindcommand,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_NO_FOCUS_END: [cmdp_token; 1] = [token {
    name: c"end".as_ptr().cast_mut(),
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 108 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_BAR_BINDSYM: [cmdp_token; 2] = [
    token {
        name: c"'--release".as_ptr().cast_mut(),
        identifier: c"release".as_ptr().cast_mut(),
        next_state: ConfigParserState::BarBindsym,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"word".as_ptr().cast_mut(),
        identifier: c"button".as_ptr().cast_mut(),
        next_state: ConfigParserState::BarBindsymCommand,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_BAR_PADDING: [cmdp_token; 1] = [token {
    name: c"number".as_ptr().cast_mut(),
    identifier: c"top_or_all".as_ptr().cast_mut(),
    next_state: ConfigParserState::BarPaddingTop,
    extra: ConfigTokenData {
        call_identifier: 0 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_BAR_VERBOSE: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"value".as_ptr().cast_mut(),
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 109 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_BINDCOMMAND: [cmdp_token; 5] = [
    token {
        name: c"'--release".as_ptr().cast_mut(),
        identifier: c"release".as_ptr().cast_mut(),
        next_state: ConfigParserState::Bindcommand,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'--border".as_ptr().cast_mut(),
        identifier: c"border".as_ptr().cast_mut(),
        next_state: ConfigParserState::Bindcommand,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'--whole-window".as_ptr().cast_mut(),
        identifier: c"whole_window".as_ptr().cast_mut(),
        next_state: ConfigParserState::Bindcommand,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'--exclude-titlebar".as_ptr().cast_mut(),
        identifier: c"exclude_titlebar".as_ptr().cast_mut(),
        next_state: ConfigParserState::Bindcommand,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"string".as_ptr().cast_mut(),
        identifier: c"command".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 110 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_IGNORE_LINE: [cmdp_token; 1] = [token {
    name: c"line".as_ptr().cast_mut(),
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: ConfigParserState::Initial,
    extra: ConfigTokenData {
        call_identifier: 0 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_TILING_DRAG: [cmdp_token; 9] = [
    token {
        name: c"'0".as_ptr().cast_mut(),
        identifier: c"off".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 111 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'no".as_ptr().cast_mut(),
        identifier: c"off".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 112 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'false".as_ptr().cast_mut(),
        identifier: c"off".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 113 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'off".as_ptr().cast_mut(),
        identifier: c"off".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 114 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'disable".as_ptr().cast_mut(),
        identifier: c"off".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 115 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'inactive".as_ptr().cast_mut(),
        identifier: c"off".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 116 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'swap_modifier".as_ptr().cast_mut(),
        identifier: c"swap_modifier".as_ptr().cast_mut(),
        next_state: ConfigParserState::TilingDragSwapModifier,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'modifier".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: ConfigParserState::TilingDragMode,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'titlebar".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: ConfigParserState::TilingDragMode,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_TITLE_ALIGN: [cmdp_token; 3] = [
    token {
        name: c"'left".as_ptr().cast_mut(),
        identifier: c"alignment".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 117 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'center".as_ptr().cast_mut(),
        identifier: c"alignment".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 118 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'right".as_ptr().cast_mut(),
        identifier: c"alignment".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 119 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_BAR_COLORS: [cmdp_token; 15] = [
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::BarColors,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'#".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::BarColorsIgnoreLine,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'set".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::BarColorsIgnoreLine,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'background".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: ConfigParserState::BarColorsSingle,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'statusline".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: ConfigParserState::BarColorsSingle,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'separator".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: ConfigParserState::BarColorsSingle,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'focused_background".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: ConfigParserState::BarColorsSingle,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'focused_statusline".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: ConfigParserState::BarColorsSingle,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'focused_separator".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: ConfigParserState::BarColorsSingle,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'focused_workspace".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: ConfigParserState::BarColorsBorder,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'active_workspace".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: ConfigParserState::BarColorsBorder,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'inactive_workspace".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: ConfigParserState::BarColorsBorder,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'urgent_workspace".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: ConfigParserState::BarColorsBorder,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'binding_mode".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: ConfigParserState::BarColorsBorder,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'}".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::Bar,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_BAR_HEIGHT: [cmdp_token; 1] = [token {
    name: c"number".as_ptr().cast_mut(),
    identifier: c"value".as_ptr().cast_mut(),
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 120 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_BAR_OUTPUT: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"output".as_ptr().cast_mut(),
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 121 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_COLOR_TEXT: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"text".as_ptr().cast_mut(),
    next_state: ConfigParserState::ColorIndicator,
    extra: ConfigTokenData {
        call_identifier: 0 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_FOR_WINDOW: [cmdp_token; 1] = [token {
    name: c"'[".as_ptr().cast_mut(),
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 122 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_IPC_SOCKET: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"path".as_ptr().cast_mut(),
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 123 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_SHOW_MARKS: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"value".as_ptr().cast_mut(),
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 124 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_SMART_GAPS: [cmdp_token; 13] = [
    token {
        name: c"'1".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 125 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'yes".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 126 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'true".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 127 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'on".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 128 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'enable".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 129 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'active".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 130 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'0".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 131 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'no".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 132 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'false".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 133 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'off".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 134 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'disable".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 135 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'inactive".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 136 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'inverse_outer".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 137 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_CRITERION: [cmdp_token; 1] = [token {
    name: c"'=".as_ptr().cast_mut(),
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: ConfigParserState::CriterionStr,
    extra: ConfigTokenData {
        call_identifier: 0 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_MODEBRACE: [cmdp_token; 2] = [
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::Modebrace,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'{".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::Mode,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_WORKSPACE: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"workspace".as_ptr().cast_mut(),
    next_state: ConfigParserState::WorkspaceCommand,
    extra: ConfigTokenData {
        call_identifier: 0 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_BARBRACE: [cmdp_token; 2] = [
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::Barbrace,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'{".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 138 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_BAR_FONT: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"font".as_ptr().cast_mut(),
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 139 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_BAR_MODE: [cmdp_token; 3] = [
    token {
        name: c"'dock".as_ptr().cast_mut(),
        identifier: c"mode".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 140 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'hide".as_ptr().cast_mut(),
        identifier: c"mode".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 141 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'invisible".as_ptr().cast_mut(),
        identifier: c"mode".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 142 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_CRITERIA: [cmdp_token; 17] = [
    token {
        name: c"'class".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: ConfigParserState::Criterion,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'instance".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: ConfigParserState::Criterion,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'window_role".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: ConfigParserState::Criterion,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'con_id".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: ConfigParserState::Criterion,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'id".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: ConfigParserState::Criterion,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'window_type".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: ConfigParserState::Criterion,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'con_mark".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: ConfigParserState::Criterion,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'title".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: ConfigParserState::Criterion,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'urgent".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: ConfigParserState::Criterion,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'workspace".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: ConfigParserState::Criterion,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'machine".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: ConfigParserState::Criterion,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'floating_from".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: ConfigParserState::CriterionFrom,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'tiling_from".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: ConfigParserState::CriterionFrom,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'tiling".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 143 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'floating".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 144 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'all".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 145 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"']".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 146 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_GAPS_END: [cmdp_token; 2] = [
    token {
        name: c"'px".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::GapsEnd,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 147 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_MODENAME: [cmdp_token; 2] = [
    token {
        name: c"'--pango_markup".as_ptr().cast_mut(),
        identifier: c"pango_markup".as_ptr().cast_mut(),
        next_state: ConfigParserState::Modename,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"word".as_ptr().cast_mut(),
        identifier: c"modename".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 148 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_NO_FOCUS: [cmdp_token; 1] = [token {
    name: c"'[".as_ptr().cast_mut(),
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 149 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_BINDING: [cmdp_token; 20] = [
    token {
        name: c"'--release".as_ptr().cast_mut(),
        identifier: c"release".as_ptr().cast_mut(),
        next_state: ConfigParserState::Binding,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'--border".as_ptr().cast_mut(),
        identifier: c"border".as_ptr().cast_mut(),
        next_state: ConfigParserState::Binding,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'--whole-window".as_ptr().cast_mut(),
        identifier: c"whole_window".as_ptr().cast_mut(),
        next_state: ConfigParserState::Binding,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'--exclude-titlebar".as_ptr().cast_mut(),
        identifier: c"exclude_titlebar".as_ptr().cast_mut(),
        next_state: ConfigParserState::Binding,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Mod1".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ConfigParserState::Binding,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Mod2".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ConfigParserState::Binding,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Mod3".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ConfigParserState::Binding,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Mod4".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ConfigParserState::Binding,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Mod5".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ConfigParserState::Binding,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Shift".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ConfigParserState::Binding,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Control".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ConfigParserState::Binding,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Ctrl".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ConfigParserState::Binding,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Mode_switch".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ConfigParserState::Binding,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Group1".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ConfigParserState::Binding,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Group2".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ConfigParserState::Binding,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Group3".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ConfigParserState::Binding,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'Group4".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ConfigParserState::Binding,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'$mod".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: ConfigParserState::Binding,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'+".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::Binding,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"word".as_ptr().cast_mut(),
        identifier: c"key".as_ptr().cast_mut(),
        next_state: ConfigParserState::Bindcommand,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_INCLUDE: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"pattern".as_ptr().cast_mut(),
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 150 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_INITIAL: [cmdp_token; 60] = [
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::Initial,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"error".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::Initial,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'#".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::IgnoreLine,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'set ".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::IgnoreLine,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'set\t".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::IgnoreLine,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'set_from_resource".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::IgnoreLine,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'include".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::Include,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'bindsym".as_ptr().cast_mut(),
        identifier: c"bindtype".as_ptr().cast_mut(),
        next_state: ConfigParserState::Binding,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'bindcode".as_ptr().cast_mut(),
        identifier: c"bindtype".as_ptr().cast_mut(),
        next_state: ConfigParserState::Binding,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'bind".as_ptr().cast_mut(),
        identifier: c"bindtype".as_ptr().cast_mut(),
        next_state: ConfigParserState::Binding,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'bar".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::Barbrace,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'font".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::Font,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'mode".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::Modename,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'gaps".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::Gaps,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'smart_borders".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::SmartBorders,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'smart_gaps".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::SmartGaps,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'floating_minimum_size".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::FloatingMinimumSizeWidth,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'floating_maximum_size".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::FloatingMaximumSizeWidth,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'floating_modifier".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::FloatingModifier,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'default_orientation".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::DefaultOrientation,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'workspace_layout".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::WorkspaceLayout,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'default_border".as_ptr().cast_mut(),
        identifier: c"windowtype".as_ptr().cast_mut(),
        next_state: ConfigParserState::DefaultBorder,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'new_window".as_ptr().cast_mut(),
        identifier: c"windowtype".as_ptr().cast_mut(),
        next_state: ConfigParserState::DefaultBorder,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'default_floating_border".as_ptr().cast_mut(),
        identifier: c"windowtype".as_ptr().cast_mut(),
        next_state: ConfigParserState::DefaultBorder,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'new_float".as_ptr().cast_mut(),
        identifier: c"windowtype".as_ptr().cast_mut(),
        next_state: ConfigParserState::DefaultBorder,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'hide_edge_borders".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::HideEdgeBorders,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'for_window".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::ForWindow,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'assign".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::Assign,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'no_focus".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::NoFocus,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'focus_follows_mouse".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::FocusFollowsMouse,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'mouse_warping".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::MouseWarping,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'focus_wrapping".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::FocusWrapping,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'force_focus_wrapping".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::ForceFocusWrapping,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'force_xinerama".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::ForceXinerama,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'force-xinerama".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::ForceXinerama,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'disable_randr15".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::DisableRandr15,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'disable-randr15".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::DisableRandr15,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'workspace_auto_back_and_forth".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::WorkspaceBackAndForth,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'fake_outputs".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::FakeOutputs,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'fake-outputs".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::FakeOutputs,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'force_display_urgency_hint".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::ForceDisplayUrgencyHint,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'focus_on_window_activation".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::FocusOnWindowActivation,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'title_align".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::TitleAlign,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'show_marks".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::ShowMarks,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'workspace".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::Workspace,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'ipc_socket".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::IpcSocket,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'ipc-socket".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::IpcSocket,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'ipc_kill_timeout".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::IpcKillTimeout,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'restart_state".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::RestartState,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'popup_during_fullscreen".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::PopupDuringFullscreen,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'tiling_drag".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::TilingDrag,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'exec_always".as_ptr().cast_mut(),
        identifier: c"exectype".as_ptr().cast_mut(),
        next_state: ConfigParserState::Exec,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'exec".as_ptr().cast_mut(),
        identifier: c"exectype".as_ptr().cast_mut(),
        next_state: ConfigParserState::Exec,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'client.background".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: ConfigParserState::ColorSingle,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'client.focused_inactive".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: ConfigParserState::ColorBorder,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'client.focused_tab_title".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: ConfigParserState::ColorBorder,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'client.focused".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: ConfigParserState::ColorBorder,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'client.unfocused".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: ConfigParserState::ColorBorder,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'client.urgent".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: ConfigParserState::ColorBorder,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'client.placeholder".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: ConfigParserState::ColorBorder,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_ASSIGN: [cmdp_token; 1] = [token {
    name: c"'[".as_ptr().cast_mut(),
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 151 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_BAR_ID: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"bar_id".as_ptr().cast_mut(),
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 152 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_EXEC: [cmdp_token; 2] = [
    token {
        name: c"'--no-startup-id".as_ptr().cast_mut(),
        identifier: c"no_startup_id".as_ptr().cast_mut(),
        next_state: ConfigParserState::Exec,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"string".as_ptr().cast_mut(),
        identifier: c"command".as_ptr().cast_mut(),
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
            call_identifier: 153 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_FONT: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"font".as_ptr().cast_mut(),
    next_state: ConfigParserState::Call,
    extra: ConfigTokenData {
        call_identifier: 154 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_GAPS: [cmdp_token; 8] = [
    token {
        name: c"'inner".as_ptr().cast_mut(),
        identifier: c"scope".as_ptr().cast_mut(),
        next_state: ConfigParserState::GapsWithScope,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'outer".as_ptr().cast_mut(),
        identifier: c"scope".as_ptr().cast_mut(),
        next_state: ConfigParserState::GapsWithScope,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'horizontal".as_ptr().cast_mut(),
        identifier: c"scope".as_ptr().cast_mut(),
        next_state: ConfigParserState::GapsWithScope,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'vertical".as_ptr().cast_mut(),
        identifier: c"scope".as_ptr().cast_mut(),
        next_state: ConfigParserState::GapsWithScope,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'top".as_ptr().cast_mut(),
        identifier: c"scope".as_ptr().cast_mut(),
        next_state: ConfigParserState::GapsWithScope,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'right".as_ptr().cast_mut(),
        identifier: c"scope".as_ptr().cast_mut(),
        next_state: ConfigParserState::GapsWithScope,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'bottom".as_ptr().cast_mut(),
        identifier: c"scope".as_ptr().cast_mut(),
        next_state: ConfigParserState::GapsWithScope,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'left".as_ptr().cast_mut(),
        identifier: c"scope".as_ptr().cast_mut(),
        next_state: ConfigParserState::GapsWithScope,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_MODE: [cmdp_token; 8] = [
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::Mode,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"error".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::Mode,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'#".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::ModeIgnoreLine,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'set".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::ModeIgnoreLine,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'bindsym".as_ptr().cast_mut(),
        identifier: c"bindtype".as_ptr().cast_mut(),
        next_state: ConfigParserState::ModeBinding,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'bindcode".as_ptr().cast_mut(),
        identifier: c"bindtype".as_ptr().cast_mut(),
        next_state: ConfigParserState::ModeBinding,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'bind".as_ptr().cast_mut(),
        identifier: c"bindtype".as_ptr().cast_mut(),
        next_state: ConfigParserState::ModeBinding,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'}".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::Initial,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_BAR: [cmdp_token; 31] = [
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::Bar,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"error".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::Bar,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'#".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::BarIgnoreLine,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'set".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::BarIgnoreLine,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'i3bar_command".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::BarBarCommand,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'status_command".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::BarStatusCommand,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'workspace_command".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::BarWorkspaceCommand,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'socket_path".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::BarSocketPath,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'mode".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::BarMode,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'hidden_state".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::BarHiddenState,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'id".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::BarId,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'modifier".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::BarModifier,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'wheel_up_cmd".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::BarWheelUpCmd,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'wheel_down_cmd".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::BarWheelDownCmd,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'bindsym".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::BarBindsym,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'position".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::BarPosition,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'output".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::BarOutput,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'tray_output".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::BarTrayOutput,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'tray_padding".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::BarTrayPadding,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'font".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::BarFont,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'separator_symbol".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::BarSeparatorSymbol,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'binding_mode_indicator".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::BarBindingModeIndicator,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'workspace_buttons".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::BarWorkspaceButtons,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'workspace_min_width".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::BarWorkspaceMinWidth,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'strip_workspace_numbers".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::BarStripWorkspaceNumbers,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'strip_workspace_name".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::BarStripWorkspaceName,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'verbose".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::BarVerbose,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'height".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::BarHeight,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'padding".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::BarPadding,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'colors".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::BarColorsBrace,
        extra: ConfigTokenData {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'}".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ConfigParserState::Call,
        extra: ConfigTokenData {
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
unsafe fn GENERATED_call(
    current_match: *mut Match,
    stack: *mut stack,
    call_identifier: ::core::ffi::c_int,
    result: *mut ConfigResultIR,
) {
    match call_identifier {
        0 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_force_display_urgency_hint(
                current_match,
                result,
                parser_get_long(stack, c"duration_ms".as_ptr()),
            );
        }
        1 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_floating_maximum_size(
                current_match,
                result,
                parser_get_long(stack, c"width".as_ptr()),
                parser_get_long(stack, c"height".as_ptr()),
            );
        }
        2 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_floating_minimum_size(
                current_match,
                result,
                parser_get_long(stack, c"width".as_ptr()),
                parser_get_long(stack, c"height".as_ptr()),
            );
        }
        3 => {
            (*result).next_state = ConfigParserState::Bar;
            cfg_bar_strip_workspace_numbers(
                current_match,
                result,
                i3_common::boolstr::is_enabled(
                    (&*stack)
                        .get_string(c"value")
                        .expect("boolean directive has a captured value")
                        .to_bytes(),
                ),
            );
        }
        4 => {
            (*result).next_state = ConfigParserState::Bar;
            cfg_bar_binding_mode_indicator(
                current_match,
                result,
                i3_common::boolstr::is_enabled(
                    (&*stack)
                        .get_string(c"value")
                        .expect("boolean directive has a captured value")
                        .to_bytes(),
                ),
            );
        }
        5 => {
            (*result).next_state = ConfigParserState::Bar;
            cfg_bar_workspace_min_width(
                current_match,
                result,
                parser_get_long(stack, c"width".as_ptr()),
            );
        }
        6 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_focus_on_window_activation(
                current_match,
                result,
                parser_get_string(stack, c"mode".as_ptr()),
            );
        }
        7 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_tiling_drag_swap_modifier(
                current_match,
                result,
                parser_get_string(stack, c"modifiers".as_ptr()),
            );
        }
        8 => {
            (*result).next_state = ConfigParserState::Bar;
            cfg_bar_strip_workspace_name(
                current_match,
                result,
                i3_common::boolstr::is_enabled(
                    (&*stack)
                        .get_string(c"value")
                        .expect("boolean directive has a captured value")
                        .to_bytes(),
                ),
            );
        }
        9 => {
            (*result).next_state = ConfigParserState::Criteria;
            cfg_criteria_add(
                current_match,
                result,
                parser_get_string(stack, c"ctype".as_ptr()),
                parser_get_string(stack, c"kind".as_ptr()),
            );
        }
        10 => {
            (*result).next_state = ConfigParserState::Criteria;
            cfg_criteria_add(
                current_match,
                result,
                parser_get_string(stack, c"ctype".as_ptr()),
                parser_get_string(stack, c"kind".as_ptr()),
            );
        }
        11 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_default_border(
                current_match,
                result,
                parser_get_string(stack, c"windowtype".as_ptr()),
                parser_get_string(stack, c"border".as_ptr()),
                parser_get_long(stack, c"width".as_ptr()),
            );
        }
        12 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_workspace_back_and_forth(
                current_match,
                result,
                i3_common::boolstr::is_enabled(
                    (&*stack)
                        .get_string(c"value")
                        .expect("boolean directive has a captured value")
                        .to_bytes(),
                ),
            );
        }
        13 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_assign(
                current_match,
                result,
                parser_get_string(stack, c"number".as_ptr()),
                1 as ::core::ffi::c_int != 0,
            );
        }
        14 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_popup_during_fullscreen(
                current_match,
                result,
                parser_get_string(stack, c"value".as_ptr()),
            );
        }
        15 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_popup_during_fullscreen(
                current_match,
                result,
                parser_get_string(stack, c"value".as_ptr()),
            );
        }
        16 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_popup_during_fullscreen(
                current_match,
                result,
                parser_get_string(stack, c"value".as_ptr()),
            );
        }
        17 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_popup_during_fullscreen(
                current_match,
                result,
                parser_get_string(stack, c"value".as_ptr()),
            );
        }
        18 => {
            (*result).next_state = ConfigParserState::Criteria;
            cfg_criteria_add(
                current_match,
                result,
                parser_get_string(stack, c"ctype".as_ptr()),
                parser_get_string(stack, c"kind".as_ptr()),
            );
        }
        19 => {
            (*result).next_state = ConfigParserState::Bar;
            cfg_bar_workspace_buttons(
                current_match,
                result,
                i3_common::boolstr::is_enabled(
                    (&*stack)
                        .get_string(c"value")
                        .expect("boolean directive has a captured value")
                        .to_bytes(),
                ),
            );
        }
        20 => {
            (*result).next_state = ConfigParserState::Bar;
            cfg_bar_workspace_command(
                current_match,
                result,
                parser_get_string(stack, c"command".as_ptr()),
            );
        }
        21 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_default_border(
                current_match,
                result,
                parser_get_string(stack, c"windowtype".as_ptr()),
                parser_get_string(stack, c"border".as_ptr()),
                2 as ::core::ffi::c_long,
            );
        }
        22 => {
            (*result).next_state = ConfigParserState::WorkspaceOutputWord;
            cfg_workspace(
                current_match,
                result,
                parser_get_string(stack, c"workspace".as_ptr()),
                parser_get_string(stack, c"output".as_ptr()),
            );
        }
        23 => {
            (*result).next_state = ConfigParserState::Bar;
            cfg_bar_separator_symbol(
                current_match,
                result,
                parser_get_string(stack, c"separator".as_ptr()),
            );
        }
        24 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_force_focus_wrapping(
                current_match,
                result,
                i3_common::boolstr::is_enabled(
                    (&*stack)
                        .get_string(c"value")
                        .expect("boolean directive has a captured value")
                        .to_bytes(),
                ),
            );
        }
        25 => {
            (*result).next_state = ConfigParserState::Bar;
            cfg_bar_bindsym(
                current_match,
                result,
                parser_get_string(stack, c"button".as_ptr()),
                parser_get_string(stack, c"release".as_ptr()),
                parser_get_string(stack, c"command".as_ptr()),
            );
        }
        26 => {
            (*result).next_state = ConfigParserState::Bar;
            cfg_bar_tray_padding(
                current_match,
                result,
                parser_get_long(stack, c"padding_px".as_ptr()),
            );
        }
        27 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_default_orientation(
                current_match,
                result,
                parser_get_string(stack, c"orientation".as_ptr()),
            );
        }
        28 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_default_orientation(
                current_match,
                result,
                parser_get_string(stack, c"orientation".as_ptr()),
            );
        }
        29 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_default_orientation(
                current_match,
                result,
                parser_get_string(stack, c"orientation".as_ptr()),
            );
        }
        30 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_focus_follows_mouse(
                current_match,
                result,
                i3_common::boolstr::is_enabled(
                    (&*stack)
                        .get_string(c"value")
                        .expect("boolean directive has a captured value")
                        .to_bytes(),
                ),
            );
        }
        31 => {
            (*result).next_state = ConfigParserState::Bar;
            cfg_bar_padding_three(
                current_match,
                result,
                parser_get_long(stack, c"top_or_all".as_ptr()),
                parser_get_long(stack, c"right_or_right_and_left".as_ptr()),
                parser_get_long(stack, c"bottom".as_ptr()),
            );
        }
        32 => {
            (*result).next_state = ConfigParserState::Bar;
            cfg_bar_status_command(
                current_match,
                result,
                parser_get_string(stack, c"command".as_ptr()),
            );
        }
        33 => {
            (*result).next_state = ConfigParserState::Bar;
            cfg_bar_wheel_down_cmd(
                current_match,
                result,
                parser_get_string(stack, c"command".as_ptr()),
            );
        }
        34 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_color(
                current_match,
                result,
                parser_get_string(stack, c"colorclass".as_ptr()),
                parser_get_string(stack, c"border".as_ptr()),
                parser_get_string(stack, c"background".as_ptr()),
                parser_get_string(stack, c"text".as_ptr()),
                parser_get_string(stack, c"indicator".as_ptr()),
                parser_get_string(stack, c"child_border".as_ptr()),
            );
        }
        35 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_color(
                current_match,
                result,
                parser_get_string(stack, c"colorclass".as_ptr()),
                parser_get_string(stack, c"border".as_ptr()),
                parser_get_string(stack, c"background".as_ptr()),
                parser_get_string(stack, c"text".as_ptr()),
                parser_get_string(stack, c"indicator".as_ptr()),
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
        36 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_for_window(
                current_match,
                result,
                parser_get_string(stack, c"command".as_ptr()),
            );
        }
        37 => {
            (*result).next_state = ConfigParserState::BarColors;
            cfg_bar_color_single(
                current_match,
                result,
                parser_get_string(stack, c"colorclass".as_ptr()),
                parser_get_string(stack, c"color".as_ptr()),
            );
        }
        38 => {
            (*result).next_state = ConfigParserState::Bar;
            cfg_bar_padding_two(
                current_match,
                result,
                parser_get_long(stack, c"top_or_all".as_ptr()),
                parser_get_long(stack, c"right_or_right_and_left".as_ptr()),
            );
        }
        39 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_floating_modifier(
                current_match,
                result,
                parser_get_string(stack, c"modifiers".as_ptr()),
            );
        }
        40 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_hide_edge_borders(
                current_match,
                result,
                (&*stack)
                    .get_string(c"hide_borders")
                    .expect("boolean directive has a captured value"),
            );
        }
        41 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_hide_edge_borders(
                current_match,
                result,
                (&*stack)
                    .get_string(c"hide_borders")
                    .expect("boolean directive has a captured value"),
            );
        }
        42 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_hide_edge_borders(
                current_match,
                result,
                (&*stack)
                    .get_string(c"hide_borders")
                    .expect("boolean directive has a captured value"),
            );
        }
        43 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_hide_edge_borders(
                current_match,
                result,
                (&*stack)
                    .get_string(c"hide_borders")
                    .expect("boolean directive has a captured value"),
            );
        }
        44 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_hide_edge_borders(
                current_match,
                result,
                (&*stack)
                    .get_string(c"hide_borders")
                    .expect("boolean directive has a captured value"),
            );
        }
        45 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_hide_edge_borders(
                current_match,
                result,
                (&*stack)
                    .get_string(c"hide_borders")
                    .expect("boolean directive has a captured value"),
            );
        }
        46 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_hide_edge_borders(
                current_match,
                result,
                (&*stack)
                    .get_string(c"hide_borders")
                    .expect("boolean directive has a captured value"),
            );
        }
        47 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_hide_edge_borders(
                current_match,
                result,
                (&*stack)
                    .get_string(c"hide_borders")
                    .expect("boolean directive has a captured value"),
            );
        }
        48 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_hide_edge_borders(
                current_match,
                result,
                (&*stack)
                    .get_string(c"hide_borders")
                    .expect("boolean directive has a captured value"),
            );
        }
        49 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_hide_edge_borders(
                current_match,
                result,
                (&*stack)
                    .get_string(c"hide_borders")
                    .expect("boolean directive has a captured value"),
            );
        }
        50 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_hide_edge_borders(
                current_match,
                result,
                (&*stack)
                    .get_string(c"hide_borders")
                    .expect("boolean directive has a captured value"),
            );
        }
        51 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_hide_edge_borders(
                current_match,
                result,
                (&*stack)
                    .get_string(c"hide_borders")
                    .expect("boolean directive has a captured value"),
            );
        }
        52 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_assign(
                current_match,
                result,
                parser_get_string(stack, c"workspace".as_ptr()),
                0 as ::core::ffi::c_int != 0,
            );
        }
        53 => {
            (*result).next_state = ConfigParserState::Bar;
            cfg_bar_hidden_state(
                current_match,
                result,
                parser_get_string(stack, c"hidden_state".as_ptr()),
            );
        }
        54 => {
            (*result).next_state = ConfigParserState::Bar;
            cfg_bar_hidden_state(
                current_match,
                result,
                parser_get_string(stack, c"hidden_state".as_ptr()),
            );
        }
        55 => {
            (*result).next_state = ConfigParserState::Bar;
            cfg_bar_padding_four(
                current_match,
                result,
                parser_get_long(stack, c"top_or_all".as_ptr()),
                parser_get_long(stack, c"right_or_right_and_left".as_ptr()),
                parser_get_long(stack, c"bottom".as_ptr()),
                parser_get_long(stack, c"left".as_ptr()),
            );
        }
        56 => {
            (*result).next_state = ConfigParserState::Bar;
            cfg_bar_wheel_up_cmd(
                current_match,
                result,
                parser_get_string(stack, c"command".as_ptr()),
            );
        }
        57 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_ipc_kill_timeout(
                current_match,
                result,
                parser_get_long(stack, c"timeout".as_ptr()),
            );
        }
        58 => {
            (*result).next_state = ConfigParserState::Mode;
            cfg_mode_binding(
                current_match,
                result,
                parser_get_string(stack, c"bindtype".as_ptr()),
                parser_get_string(stack, c"modifiers".as_ptr()),
                parser_get_string(stack, c"key".as_ptr()),
                parser_get_string(stack, c"release".as_ptr()),
                parser_get_string(stack, c"border".as_ptr()),
                parser_get_string(stack, c"whole_window".as_ptr()),
                parser_get_string(stack, c"exclude_titlebar".as_ptr()),
                parser_get_string(stack, c"command".as_ptr()),
            );
        }
        59 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_tiling_drag(
                current_match,
                result,
                parser_get_string(stack, c"value".as_ptr()),
            );
        }
        60 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_workspace_layout(
                current_match,
                result,
                parser_get_string(stack, c"layout".as_ptr()),
            );
        }
        61 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_workspace_layout(
                current_match,
                result,
                parser_get_string(stack, c"layout".as_ptr()),
            );
        }
        62 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_workspace_layout(
                current_match,
                result,
                parser_get_string(stack, c"layout".as_ptr()),
            );
        }
        63 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_workspace_layout(
                current_match,
                result,
                parser_get_string(stack, c"layout".as_ptr()),
            );
        }
        64 => {
            (*result).next_state = ConfigParserState::Bar;
            cfg_bar_i3bar_command(
                current_match,
                result,
                parser_get_string(stack, c"command".as_ptr()),
            );
        }
        65 => {
            (*result).next_state = ConfigParserState::BarColors;
            cfg_bar_color(
                current_match,
                result,
                parser_get_string(stack, c"colorclass".as_ptr()),
                parser_get_string(stack, c"border".as_ptr()),
                parser_get_string(stack, c"background".as_ptr()),
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
        66 => {
            (*result).next_state = ConfigParserState::BarColors;
            cfg_bar_color(
                current_match,
                result,
                parser_get_string(stack, c"colorclass".as_ptr()),
                parser_get_string(stack, c"border".as_ptr()),
                parser_get_string(stack, c"background".as_ptr()),
                parser_get_string(stack, c"text".as_ptr()),
            );
        }
        67 => {
            (*result).next_state = ConfigParserState::Bar;
            cfg_bar_padding_one(
                current_match,
                result,
                parser_get_long(stack, c"top_or_all".as_ptr()),
            );
        }
        68 => {
            (*result).next_state = ConfigParserState::Bar;
            cfg_bar_socket_path(
                current_match,
                result,
                parser_get_string(stack, c"path".as_ptr()),
            );
        }
        69 => {
            (*result).next_state = ConfigParserState::Bar;
            cfg_bar_tray_output(
                current_match,
                result,
                parser_get_string(stack, c"output".as_ptr()),
            );
        }
        70 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_color(
                current_match,
                result,
                parser_get_string(stack, c"colorclass".as_ptr()),
                parser_get_string(stack, c"border".as_ptr()),
                parser_get_string(stack, c"background".as_ptr()),
                parser_get_string(stack, c"text".as_ptr()),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
        71 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_disable_randr15(
                current_match,
                result,
                i3_common::boolstr::is_enabled(
                    (&*stack)
                        .get_string(c"value")
                        .expect("boolean directive has a captured value")
                        .to_bytes(),
                ),
            );
        }
        72 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_default_border(
                current_match,
                result,
                parser_get_string(stack, c"windowtype".as_ptr()),
                parser_get_string(stack, c"border".as_ptr()),
                -(1 as ::core::ffi::c_int) as ::core::ffi::c_long,
            );
        }
        73 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_default_border(
                current_match,
                result,
                parser_get_string(stack, c"windowtype".as_ptr()),
                parser_get_string(stack, c"border".as_ptr()),
                -(1 as ::core::ffi::c_int) as ::core::ffi::c_long,
            );
        }
        74 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_focus_wrapping(
                current_match,
                result,
                (&*stack)
                    .get_string(c"value")
                    .expect("boolean directive has a captured value"),
            );
        }
        75 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_focus_wrapping(
                current_match,
                result,
                (&*stack)
                    .get_string(c"value")
                    .expect("boolean directive has a captured value"),
            );
        }
        76 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_focus_wrapping(
                current_match,
                result,
                (&*stack)
                    .get_string(c"value")
                    .expect("boolean directive has a captured value"),
            );
        }
        77 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_focus_wrapping(
                current_match,
                result,
                (&*stack)
                    .get_string(c"value")
                    .expect("boolean directive has a captured value"),
            );
        }
        78 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_focus_wrapping(
                current_match,
                result,
                (&*stack)
                    .get_string(c"value")
                    .expect("boolean directive has a captured value"),
            );
        }
        79 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_focus_wrapping(
                current_match,
                result,
                (&*stack)
                    .get_string(c"value")
                    .expect("boolean directive has a captured value"),
            );
        }
        80 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_focus_wrapping(
                current_match,
                result,
                (&*stack)
                    .get_string(c"value")
                    .expect("boolean directive has a captured value"),
            );
        }
        81 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_focus_wrapping(
                current_match,
                result,
                (&*stack)
                    .get_string(c"value")
                    .expect("boolean directive has a captured value"),
            );
        }
        82 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_focus_wrapping(
                current_match,
                result,
                (&*stack)
                    .get_string(c"value")
                    .expect("boolean directive has a captured value"),
            );
        }
        83 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_focus_wrapping(
                current_match,
                result,
                (&*stack)
                    .get_string(c"value")
                    .expect("boolean directive has a captured value"),
            );
        }
        84 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_focus_wrapping(
                current_match,
                result,
                (&*stack)
                    .get_string(c"value")
                    .expect("boolean directive has a captured value"),
            );
        }
        85 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_focus_wrapping(
                current_match,
                result,
                (&*stack)
                    .get_string(c"value")
                    .expect("boolean directive has a captured value"),
            );
        }
        86 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_focus_wrapping(
                current_match,
                result,
                (&*stack)
                    .get_string(c"value")
                    .expect("boolean directive has a captured value"),
            );
        }
        87 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_focus_wrapping(
                current_match,
                result,
                (&*stack)
                    .get_string(c"value")
                    .expect("boolean directive has a captured value"),
            );
        }
        88 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_force_xinerama(
                current_match,
                result,
                i3_common::boolstr::is_enabled(
                    (&*stack)
                        .get_string(c"value")
                        .expect("boolean directive has a captured value")
                        .to_bytes(),
                ),
            );
        }
        89 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_assign_output(
                current_match,
                result,
                parser_get_string(stack, c"output".as_ptr()),
            );
        }
        90 => {
            (*result).next_state = ConfigParserState::Criteria;
            cfg_criteria_add(
                current_match,
                result,
                parser_get_string(stack, c"ctype".as_ptr()),
                parser_get_string(stack, c"cvalue".as_ptr()),
            );
        }
        91 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_mouse_warping(
                current_match,
                result,
                parser_get_string(stack, c"value".as_ptr()),
            );
        }
        92 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_mouse_warping(
                current_match,
                result,
                parser_get_string(stack, c"value".as_ptr()),
            );
        }
        93 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_restart_state(
                current_match,
                result,
                parser_get_string(stack, c"path".as_ptr()),
            );
        }
        94 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_smart_borders(
                current_match,
                result,
                (&*stack)
                    .get_string(c"enabled")
                    .expect("boolean directive has a captured value"),
            );
        }
        95 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_smart_borders(
                current_match,
                result,
                (&*stack)
                    .get_string(c"enabled")
                    .expect("boolean directive has a captured value"),
            );
        }
        96 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_smart_borders(
                current_match,
                result,
                (&*stack)
                    .get_string(c"enabled")
                    .expect("boolean directive has a captured value"),
            );
        }
        97 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_smart_borders(
                current_match,
                result,
                (&*stack)
                    .get_string(c"enabled")
                    .expect("boolean directive has a captured value"),
            );
        }
        98 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_smart_borders(
                current_match,
                result,
                (&*stack)
                    .get_string(c"enabled")
                    .expect("boolean directive has a captured value"),
            );
        }
        99 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_smart_borders(
                current_match,
                result,
                (&*stack)
                    .get_string(c"enabled")
                    .expect("boolean directive has a captured value"),
            );
        }
        100 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_smart_borders(
                current_match,
                result,
                (&*stack)
                    .get_string(c"enabled")
                    .expect("boolean directive has a captured value"),
            );
        }
        101 => {
            (*result).next_state = ConfigParserState::Bar;
            cfg_bar_modifier(
                current_match,
                result,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
        102 => {
            (*result).next_state = ConfigParserState::Bar;
            cfg_bar_modifier(
                current_match,
                result,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
        103 => {
            (*result).next_state = ConfigParserState::Bar;
            cfg_bar_modifier(
                current_match,
                result,
                parser_get_string(stack, c"modifiers".as_ptr()),
            );
        }
        104 => {
            (*result).next_state = ConfigParserState::Bar;
            cfg_bar_position(
                current_match,
                result,
                parser_get_string(stack, c"position".as_ptr()),
            );
        }
        105 => {
            (*result).next_state = ConfigParserState::Bar;
            cfg_bar_position(
                current_match,
                result,
                parser_get_string(stack, c"position".as_ptr()),
            );
        }
        106 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_color_single(
                current_match,
                result,
                parser_get_string(stack, c"colorclass".as_ptr()),
                parser_get_string(stack, c"color".as_ptr()),
            );
        }
        107 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_fake_outputs(
                current_match,
                result,
                parser_get_string(stack, c"outputs".as_ptr()),
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
                    (&*stack)
                        .get_string(c"value")
                        .expect("boolean directive has a captured value")
                        .to_bytes(),
                ),
            );
        }
        110 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_binding(
                current_match,
                result,
                parser_get_string(stack, c"bindtype".as_ptr()),
                parser_get_string(stack, c"modifiers".as_ptr()),
                parser_get_string(stack, c"key".as_ptr()),
                parser_get_string(stack, c"release".as_ptr()),
                parser_get_string(stack, c"border".as_ptr()),
                parser_get_string(stack, c"whole_window".as_ptr()),
                parser_get_string(stack, c"exclude_titlebar".as_ptr()),
                parser_get_string(stack, c"command".as_ptr()),
            );
        }
        111 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_tiling_drag(
                current_match,
                result,
                parser_get_string(stack, c"off".as_ptr()),
            );
        }
        112 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_tiling_drag(
                current_match,
                result,
                parser_get_string(stack, c"off".as_ptr()),
            );
        }
        113 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_tiling_drag(
                current_match,
                result,
                parser_get_string(stack, c"off".as_ptr()),
            );
        }
        114 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_tiling_drag(
                current_match,
                result,
                parser_get_string(stack, c"off".as_ptr()),
            );
        }
        115 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_tiling_drag(
                current_match,
                result,
                parser_get_string(stack, c"off".as_ptr()),
            );
        }
        116 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_tiling_drag(
                current_match,
                result,
                parser_get_string(stack, c"off".as_ptr()),
            );
        }
        117 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_title_align(
                current_match,
                result,
                parser_get_string(stack, c"alignment".as_ptr()),
            );
        }
        118 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_title_align(
                current_match,
                result,
                parser_get_string(stack, c"alignment".as_ptr()),
            );
        }
        119 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_title_align(
                current_match,
                result,
                parser_get_string(stack, c"alignment".as_ptr()),
            );
        }
        120 => {
            (*result).next_state = ConfigParserState::Bar;
            cfg_bar_height(
                current_match,
                result,
                parser_get_long(stack, c"value".as_ptr()),
            );
        }
        121 => {
            (*result).next_state = ConfigParserState::Bar;
            cfg_bar_output(
                current_match,
                result,
                parser_get_string(stack, c"output".as_ptr()),
            );
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
                parser_get_string(stack, c"path".as_ptr()),
            );
        }
        124 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_show_marks(
                current_match,
                result,
                i3_common::boolstr::is_enabled(
                    (&*stack)
                        .get_string(c"value")
                        .expect("boolean directive has a captured value")
                        .to_bytes(),
                ),
            );
        }
        125 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_smart_gaps(
                current_match,
                result,
                (&*stack)
                    .get_string(c"enabled")
                    .expect("boolean directive has a captured value"),
            );
        }
        126 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_smart_gaps(
                current_match,
                result,
                (&*stack)
                    .get_string(c"enabled")
                    .expect("boolean directive has a captured value"),
            );
        }
        127 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_smart_gaps(
                current_match,
                result,
                (&*stack)
                    .get_string(c"enabled")
                    .expect("boolean directive has a captured value"),
            );
        }
        128 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_smart_gaps(
                current_match,
                result,
                (&*stack)
                    .get_string(c"enabled")
                    .expect("boolean directive has a captured value"),
            );
        }
        129 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_smart_gaps(
                current_match,
                result,
                (&*stack)
                    .get_string(c"enabled")
                    .expect("boolean directive has a captured value"),
            );
        }
        130 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_smart_gaps(
                current_match,
                result,
                (&*stack)
                    .get_string(c"enabled")
                    .expect("boolean directive has a captured value"),
            );
        }
        131 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_smart_gaps(
                current_match,
                result,
                (&*stack)
                    .get_string(c"enabled")
                    .expect("boolean directive has a captured value"),
            );
        }
        132 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_smart_gaps(
                current_match,
                result,
                (&*stack)
                    .get_string(c"enabled")
                    .expect("boolean directive has a captured value"),
            );
        }
        133 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_smart_gaps(
                current_match,
                result,
                (&*stack)
                    .get_string(c"enabled")
                    .expect("boolean directive has a captured value"),
            );
        }
        134 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_smart_gaps(
                current_match,
                result,
                (&*stack)
                    .get_string(c"enabled")
                    .expect("boolean directive has a captured value"),
            );
        }
        135 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_smart_gaps(
                current_match,
                result,
                (&*stack)
                    .get_string(c"enabled")
                    .expect("boolean directive has a captured value"),
            );
        }
        136 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_smart_gaps(
                current_match,
                result,
                (&*stack)
                    .get_string(c"enabled")
                    .expect("boolean directive has a captured value"),
            );
        }
        137 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_smart_gaps(
                current_match,
                result,
                (&*stack)
                    .get_string(c"enabled")
                    .expect("boolean directive has a captured value"),
            );
        }
        138 => {
            (*result).next_state = ConfigParserState::Bar;
            cfg_bar_start(current_match, result);
        }
        139 => {
            (*result).next_state = ConfigParserState::Bar;
            cfg_bar_font(
                current_match,
                result,
                parser_get_string(stack, c"font".as_ptr()),
            );
        }
        140 => {
            (*result).next_state = ConfigParserState::Bar;
            cfg_bar_mode(
                current_match,
                result,
                parser_get_string(stack, c"mode".as_ptr()),
            );
        }
        141 => {
            (*result).next_state = ConfigParserState::Bar;
            cfg_bar_mode(
                current_match,
                result,
                parser_get_string(stack, c"mode".as_ptr()),
            );
        }
        142 => {
            (*result).next_state = ConfigParserState::Bar;
            cfg_bar_mode(
                current_match,
                result,
                parser_get_string(stack, c"mode".as_ptr()),
            );
        }
        143 => {
            (*result).next_state = ConfigParserState::Criteria;
            cfg_criteria_add(
                current_match,
                result,
                parser_get_string(stack, c"ctype".as_ptr()),
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
        144 => {
            (*result).next_state = ConfigParserState::Criteria;
            cfg_criteria_add(
                current_match,
                result,
                parser_get_string(stack, c"ctype".as_ptr()),
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
        145 => {
            (*result).next_state = ConfigParserState::Criteria;
            cfg_criteria_add(
                current_match,
                result,
                parser_get_string(stack, c"ctype".as_ptr()),
                ::core::ptr::null::<::core::ffi::c_char>(),
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
                parser_get_string(stack, c"workspace".as_ptr()),
                parser_get_string(stack, c"scope".as_ptr()),
                parser_get_long(stack, c"value".as_ptr()),
            );
        }
        148 => {
            (*result).next_state = ConfigParserState::Modebrace;
            cfg_enter_mode(
                current_match,
                result,
                parser_get_string(stack, c"pango_markup".as_ptr()),
                parser_get_string(stack, c"modename".as_ptr()),
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
                parser_get_string(stack, c"pattern".as_ptr()),
            );
        }
        151 => {
            (*result).next_state = ConfigParserState::Criteria;
            cfg_criteria_init(current_match, result, ConfigParserState::AssignWorkspace);
        }
        152 => {
            (*result).next_state = ConfigParserState::Bar;
            cfg_bar_id(
                current_match,
                result,
                parser_get_string(stack, c"bar_id".as_ptr()),
            );
        }
        153 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_exec(
                current_match,
                result,
                parser_get_string(stack, c"exectype".as_ptr()),
                parser_get_string(stack, c"no_startup_id".as_ptr()),
                parser_get_string(stack, c"command".as_ptr()),
            );
        }
        154 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_font(
                current_match,
                result,
                parser_get_string(stack, c"font".as_ptr()),
            );
        }
        155 => {
            (*result).next_state = ConfigParserState::Initial;
            cfg_bar_finish(current_match, result);
        }
        _ => {
            printf(c"BUG in the parser. state = %d\n".as_ptr(), call_identifier);
            if 0 as ::core::ffi::c_int != 0 {
            } else {
                __assert_fail(
                    c"false".as_ptr(),
                    c"./GENERATED_config_call.h".as_ptr(),
                    1265 as ::core::ffi::c_uint,
                    c"void GENERATED_call(Match *, struct stack *, const int, struct ConfigResultIR *)".as_ptr(),
                );
            };
        }
    };
}
// These private generated arrays are initialized once and never mutated. Only
// the legacy metadata representation uses raw pointers; callers receive a
// checked immutable view, and no user-supplied pointer enters this boundary.
fn state_tokens(state: ConfigParserState) -> &'static [cmdp_token] {
    // SAFETY: schema-checked static token arrays have their declared lengths and
    // remain alive/immutable for the process. State indexing is checked first.
    unsafe {
        let states = &*(&raw const tokens);
        let entry = states
            .get(state as usize)
            .expect("grammar state has a token table");
        std::slice::from_raw_parts(entry.array, entry.n as usize)
    }
}

unsafe fn next_state(token: *const cmdp_token, ctx: *mut parser_ctx) {
    let mut _next_state: ConfigParserState = (*token).next_state;
    if (*token).next_state == ConfigParserState::Call {
        let mut subcommand_output: ConfigResultIR = ConfigResultIR {
            ctx,
            next_state: ConfigParserState::ForceDisplayUrgencyHintMs,
            has_errors: false,
        };
        GENERATED_call(
            &raw mut (*ctx).current_match,
            (*ctx).stack,
            (*token).extra.call_identifier as ::core::ffi::c_int,
            &raw mut subcommand_output,
        );
        if subcommand_output.has_errors {
            (*ctx).has_errors = true_0 != 0;
        }
        _next_state = subcommand_output.next_state;
        parser_clear_stack((*ctx).stack);
    }
    (*ctx).state = _next_state;
    if (*ctx).state == ConfigParserState::Initial {
        parser_clear_stack((*ctx).stack);
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
    (*ctx).statelist_idx = (*ctx).statelist.len() as libc::c_int;
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

unsafe fn single_line(start: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    let result: *mut ::core::ffi::c_char = sstrdup(start);
    let end: *mut ::core::ffi::c_char = strchr(result, '\n' as i32);
    if !end.is_null() {
        *end = '\0' as i32 as ::core::ffi::c_char;
    }
    result
}
unsafe fn parse_config(
    ctx: *mut parser_ctx,
    input: *const ::core::ffi::c_char,
    context: *mut context,
) {
    let mut dumpwalk: *const ::core::ffi::c_char = input;
    let mut linecnt: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while *dumpwalk as ::core::ffi::c_int != '\0' as i32 {
        let next_nl: *const ::core::ffi::c_char = strchr(dumpwalk, '\n' as i32);
        if !next_nl.is_null() {
            {
                // Snapshot native bytes before the logger can dispatch a callback.
                let log_value_0 = (linecnt) as i32;
                let log_precision_1 = (next_nl.offset_from(dumpwalk) as ::core::ffi::c_long
                    as ::core::ffi::c_int) as i32;
                let log_pointer_2 = dumpwalk;
                let log_value_1 = i3_common::logging::string_bytes(
                    if log_pointer_2.is_null() {
                        None
                    } else {
                        Some(std::ffi::CStr::from_ptr(log_pointer_2))
                    },
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
            dumpwalk = next_nl.offset(1 as ::core::ffi::c_int as isize);
            linecnt += 1;
        } else {
            {
                // Snapshot native bytes before the logger can dispatch a callback.
                let log_value_0 = (linecnt) as i32;
                let log_pointer_2 = dumpwalk;
                let log_value_1 = i3_common::logging::string_bytes(
                    if log_pointer_2.is_null() {
                        None
                    } else {
                        Some(std::ffi::CStr::from_ptr(log_pointer_2))
                    },
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
    let mut c: ::core::ffi::c_int = 0;
    let mut token: *const cmdp_token = ::core::ptr::null::<cmdp_token>();
    let mut token_handled: bool = false;
    linecnt = 1 as ::core::ffi::c_int;
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
    while walk.offset_from(input) as ::core::ffi::c_long as usize <= len {
        while (*walk as ::core::ffi::c_int == ' ' as i32
            || *walk as ::core::ffi::c_int == '\t' as i32)
            && *walk as ::core::ffi::c_int != '\0' as i32
        {
            walk = walk.offset(1);
        }
        let grammar = state_tokens((*ctx).state);
        token_handled = false_0 != 0;
        c = 0 as ::core::ffi::c_int;
        while c < grammar.len() as libc::c_int {
            token = &grammar[c as usize] as *const cmdp_token;
            if *(*token).name.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '\'' as i32
            {
                if strncasecmp(
                    walk,
                    (*token).name.offset(1 as ::core::ffi::c_int as isize),
                    strlen((*token).name).wrapping_sub(1_usize),
                ) == 0 as ::core::ffi::c_int
                {
                    if !(*token).identifier.is_null() {
                        parser_push_string(
                            (*ctx).stack,
                            (*token).identifier,
                            (*token).name.offset(1 as ::core::ffi::c_int as isize),
                        );
                    }
                    walk = walk.add(strlen((*token).name).wrapping_sub(1_usize));
                    next_state(token, ctx);
                    token_handled = true_0 != 0;
                    break;
                }
            } else if strcmp((*token).name, c"number".as_ptr()) == 0 as ::core::ffi::c_int {
                let mut end: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                *__errno_location() = 0 as ::core::ffi::c_int;
                let num: ::core::ffi::c_long =
                    strtol(walk, &raw mut end, 10 as ::core::ffi::c_int) as ::core::ffi::c_long;
                if !(*__errno_location() == ERANGE && (num == LONG_MIN || num == LONG_MAX)
                    || *__errno_location() != 0 as ::core::ffi::c_int
                        && num == 0 as ::core::ffi::c_long)
                    && !std::ptr::eq(end, walk)
                {
                    if !(*token).identifier.is_null() {
                        parser_push_long((*ctx).stack, (*token).identifier, num);
                    }
                    walk = end;
                    next_state(token, ctx);
                    token_handled = true_0 != 0;
                    break;
                }
            } else {
                if strcmp((*token).name, c"string".as_ptr()) == 0 as ::core::ffi::c_int
                    || strcmp((*token).name, c"word".as_ptr()) == 0 as ::core::ffi::c_int
                {
                    let remaining = std::ffi::CStr::from_ptr(walk);
                    let capture = crate::parser_util::lex_string(
                        remaining.to_bytes(),
                        *(*token).name != b's' as libc::c_char,
                        crate::parser_util::StringGrammar::Config,
                    );
                    walk = walk.add(capture.consumed);
                    if let Some(bytes) = capture.bytes {
                        if !(*token).identifier.is_null() {
                            let value = std::ffi::CString::new(bytes).expect("lexer excludes NUL");
                            parser_push_string((*ctx).stack, (*token).identifier, value.as_ptr());
                        }
                        if *walk == b'"' as libc::c_char {
                            walk = walk.add(1);
                        }
                        next_state(token, ctx);
                        token_handled = true;
                        break;
                    }
                }
                if strcmp((*token).name, c"line".as_ptr()) == 0 as ::core::ffi::c_int {
                    while *walk as ::core::ffi::c_int != '\0' as i32
                        && *walk as ::core::ffi::c_int != '\n' as i32
                        && *walk as ::core::ffi::c_int != '\r' as i32
                    {
                        walk = walk.offset(1);
                    }
                    next_state(token, ctx);
                    token_handled = true_0 != 0;
                    linecnt += 1;
                    walk = walk.offset(1);
                    break;
                } else if strcmp((*token).name, c"end".as_ptr()) == 0 as ::core::ffi::c_int
                    && (*walk as ::core::ffi::c_int == '\0' as i32
                        || *walk as ::core::ffi::c_int == '\n' as i32
                        || *walk as ::core::ffi::c_int == '\r' as i32)
                {
                    next_state(token, ctx);
                    token_handled = true_0 != 0;
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
            let names: Vec<_> = grammar
                .iter()
                .map(|token| std::ffi::CStr::from_ptr(token.name))
                .collect();
            let mut message = b"Expected one of these tokens: ".to_vec();
            message.extend_from_slice(&crate::parser_util::expected_tokens(&names, true));
            let message = std::ffi::CString::new(message).expect("grammar names contain no NUL");
            let errormessage = message.as_ptr();
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
            let caret = std::ffi::CString::new(caret).expect("caret bytes contain no NUL");
            let position = caret.as_ptr();
            {
                // Snapshot native bytes before the logger can dispatch a callback.
                let log_pointer_1 = errormessage;
                let log_value_0 = i3_common::logging::string_bytes(
                    if log_pointer_1.is_null() {
                        None
                    } else {
                        Some(std::ffi::CStr::from_ptr(log_pointer_1))
                    },
                    None,
                )
                .to_vec();
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
                let log_pointer_1 = (*context).filename;
                let log_value_0 = i3_common::logging::string_bytes(
                    if log_pointer_1.is_null() {
                        None
                    } else {
                        Some(std::ffi::CStr::from_ptr(log_pointer_1))
                    },
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
            let mut error_copy: *mut ::core::ffi::c_char = single_line(error_line);
            if linecnt > 1 as ::core::ffi::c_int {
                let context_p1_start: *const ::core::ffi::c_char = previous_line(error_line, input);
                let context_p1_line: *mut ::core::ffi::c_char = single_line(context_p1_start);
                if linecnt > 2 as ::core::ffi::c_int {
                    let context_p2_start: *const ::core::ffi::c_char =
                        previous_line(context_p1_start, input);
                    let context_p2_line: *mut ::core::ffi::c_char = single_line(context_p2_start);
                    {
                        // Snapshot native bytes before the logger can dispatch a callback.
                        let log_value_0 = (linecnt - 2 as ::core::ffi::c_int) as i32;
                        let log_pointer_2 = context_p2_line;
                        let log_value_1 = i3_common::logging::string_bytes(
                            if log_pointer_2.is_null() {
                                None
                            } else {
                                Some(std::ffi::CStr::from_ptr(log_pointer_2))
                            },
                            None,
                        )
                        .to_vec();
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
                    free(context_p2_line as *mut ::core::ffi::c_void);
                }
                {
                    // Snapshot native bytes before the logger can dispatch a callback.
                    let log_value_0 = (linecnt - 1 as ::core::ffi::c_int) as i32;
                    let log_pointer_2 = context_p1_line;
                    let log_value_1 = i3_common::logging::string_bytes(
                        if log_pointer_2.is_null() {
                            None
                        } else {
                            Some(std::ffi::CStr::from_ptr(log_pointer_2))
                        },
                        None,
                    )
                    .to_vec();
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
                free(context_p1_line as *mut ::core::ffi::c_void);
            }
            {
                // Snapshot native bytes before the logger can dispatch a callback.
                let log_value_0 = (linecnt) as i32;
                let log_pointer_2 = error_copy;
                let log_value_1 = i3_common::logging::string_bytes(
                    if log_pointer_2.is_null() {
                        None
                    } else {
                        Some(std::ffi::CStr::from_ptr(log_pointer_2))
                    },
                    None,
                )
                .to_vec();
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
                let log_pointer_1 = position;
                let log_value_0 = i3_common::logging::string_bytes(
                    if log_pointer_1.is_null() {
                        None
                    } else {
                        Some(std::ffi::CStr::from_ptr(log_pointer_1))
                    },
                    None,
                )
                .to_vec();
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
            free(error_copy as *mut ::core::ffi::c_void);
            let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_0 < 2 as ::core::ffi::c_int {
                let error_line_end: *const ::core::ffi::c_char = strchr(error_line, '\n' as i32);
                if !error_line_end.is_null()
                    && *error_line_end.offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        != '\0' as i32
                {
                    error_line = error_line_end.offset(1 as ::core::ffi::c_int as isize);
                    error_copy = single_line(error_line);
                    {
                        // Snapshot native bytes before the logger can dispatch a callback.
                        let log_value_0 = (linecnt + i_0 + 1 as ::core::ffi::c_int) as i32;
                        let log_pointer_2 = error_copy;
                        let log_value_1 = i3_common::logging::string_bytes(
                            if log_pointer_2.is_null() {
                                None
                            } else {
                                Some(std::ffi::CStr::from_ptr(log_pointer_2))
                            },
                            None,
                        )
                        .to_vec();
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
                    free(error_copy as *mut ::core::ffi::c_void);
                }
                i_0 += 1;
            }
            (*context).has_errors = true_0 != 0;
            while walk.offset_from(input) as ::core::ffi::c_long as usize <= len
                && *walk as ::core::ffi::c_int != '\n' as i32
            {
                walk = walk.offset(1);
            }
            parser_clear_stack((*ctx).stack);
            let mut error_token_found: bool = false_0 != 0;
            let mut i_1: ::core::ffi::c_int = (*ctx).statelist_idx - 1 as ::core::ffi::c_int;
            while i_1 >= 0 as ::core::ffi::c_int && !error_token_found {
                let recovery = state_tokens((&(*ctx).statelist)[i_1 as usize]);
                let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while j < recovery.len() as libc::c_int {
                    if strcmp(recovery[j as usize].name, c"error".as_ptr())
                        != 0 as ::core::ffi::c_int
                    {
                        j += 1;
                    } else {
                        next_state(&recovery[j as usize] as *const cmdp_token, ctx);
                        error_token_found = true_0 != 0;
                        break;
                    }
                }
                i_1 -= 1;
            }
            if error_token_found as ::core::ffi::c_int != 0 {
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

pub unsafe fn start_config_error_nagbar(configpath: *const ::core::ffi::c_char, has_errors: bool) {
    let mut editaction: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut pageraction: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    sasprintf(
        &raw mut editaction,
        c"i3-sensible-editor \"%s\" && i3-msg reload\n".as_ptr(),
        configpath,
    );
    sasprintf(
        &raw mut pageraction,
        c"i3-sensible-pager \"%s\"\n".as_ptr(),
        errorfilename,
    );
    let mut argv: [*mut ::core::ffi::c_char; 14] = [
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
        c"-f".as_ptr().cast_mut(),
        config.font.pattern().unwrap_or(c"fixed").as_ptr() as *mut ::core::ffi::c_char,
        c"-t".as_ptr().cast_mut(),
        (if has_errors as ::core::ffi::c_int != 0 {
            c"error".as_ptr()
        } else {
            c"warning".as_ptr()
        }) as *mut ::core::ffi::c_char,
        c"-m".as_ptr().cast_mut(),
        (if has_errors as ::core::ffi::c_int != 0 {
            c"You have an error in your i3 config file!".as_ptr()
        } else {
            c"Your config is outdated. Please fix the warnings to make sure everything works."
                .as_ptr()
        }) as *mut ::core::ffi::c_char,
        c"-b".as_ptr().cast_mut(),
        c"edit config".as_ptr().cast_mut(),
        editaction,
        (if !errorfilename.is_null() {
            c"-b".as_ptr()
        } else {
            ::core::ptr::null::<::core::ffi::c_char>()
        }) as *mut ::core::ffi::c_char,
        (if has_errors as ::core::ffi::c_int != 0 {
            c"show errors".as_ptr()
        } else {
            c"show warnings".as_ptr()
        }) as *mut ::core::ffi::c_char,
        pageraction,
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
    ];
    start_nagbar(
        &raw mut config_error_nagbar_pid,
        &raw mut argv as *mut *mut ::core::ffi::c_char,
    );
    free(editaction as *mut ::core::ffi::c_void);
    free(pageraction as *mut ::core::ffi::c_void);
}
fn store_variable(variables: &mut variables_head, key: &std::ffi::CStr, value: &std::ffi::CStr) {
    if let Some(variable) = variables
        .iter_mut()
        .find(|variable| variable.key.as_c_str() == key)
    {
        variable.value = value.to_owned();
        return;
    }
    // Longest names win; a new equal-length key precedes existing entries.
    let position = variables
        .iter()
        .position(|variable| key.to_bytes().len() >= variable.key.as_bytes().len())
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
/// variables is exclusively borrowed, and key/value are readable C strings.
unsafe fn upsert_variable(
    variables: *mut variables_head,
    key: *const libc::c_char,
    value: *const libc::c_char,
) {
    let key_text = std::ffi::CStr::from_ptr(key).to_owned();
    let value_text = std::ffi::CStr::from_ptr(value).to_owned();
    let key = key_text.as_ptr();
    let value = value_text.as_ptr();
    // Snapshot the old value before diagnostics, so no vector borrow survives a
    // logger callback. Preserve the original debug text and ordering.
    let previous = (&*variables)
        .iter()
        .find(|entry| entry.key.as_c_str() == key_text.as_c_str())
        .map(|entry| entry.value.clone());
    if let Some(previous) = previous {
        {
            // Snapshot native bytes before the logger can dispatch a callback.
            let log_pointer_1 = key;
            let log_value_0 = i3_common::logging::string_bytes(
                if log_pointer_1.is_null() {
                    None
                } else {
                    Some(std::ffi::CStr::from_ptr(log_pointer_1))
                },
                None,
            )
            .to_vec();
            let log_pointer_2 = previous.as_ptr();
            let log_value_1 = i3_common::logging::string_bytes(
                if log_pointer_2.is_null() {
                    None
                } else {
                    Some(std::ffi::CStr::from_ptr(log_pointer_2))
                },
                None,
            )
            .to_vec();
            let log_pointer_3 = value;
            let log_value_2 = i3_common::logging::string_bytes(
                if log_pointer_3.is_null() {
                    None
                } else {
                    Some(std::ffi::CStr::from_ptr(log_pointer_3))
                },
                None,
            )
            .to_vec();
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
            let log_pointer_1 = key;
            let log_value_0 = i3_common::logging::string_bytes(
                if log_pointer_1.is_null() {
                    None
                } else {
                    Some(std::ffi::CStr::from_ptr(log_pointer_1))
                },
                None,
            )
            .to_vec();
            let log_pointer_2 = value;
            let log_value_1 = i3_common::logging::string_bytes(
                if log_pointer_2.is_null() {
                    None
                } else {
                    Some(std::ffi::CStr::from_ptr(log_pointer_2))
                },
                None,
            )
            .to_vec();
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
    unsafe fn get(&mut self, name: &std::ffi::CStr) -> Option<std::ffi::CString> {
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
        self.0.as_ref()?.resource(name, None)
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

fn expand_variables(
    input: &std::ffi::CStr,
    variables: &[(&std::ffi::CStr, &std::ffi::CStr)],
) -> Vec<u8> {
    let bytes = input.to_bytes();
    let mut output = Vec::with_capacity(bytes.len());
    let mut offset = 0;
    while offset < bytes.len() {
        let remainder = std::ffi::CStr::from_bytes_with_nul(&input.to_bytes_with_nul()[offset..])
            .expect("input suffix remains terminated");
        let nearest = variables
            .iter()
            .filter_map(|&(key, value)| {
                if key.to_bytes().is_empty() {
                    return None;
                }
                // SAFETY: native case-folded search preserves locale behavior; both
                // inputs are C strings and any result points into remainder.
                let found = unsafe { libc::strcasestr(remainder.as_ptr(), key.as_ptr()) };
                if found.is_null() {
                    None
                } else {
                    let distance = unsafe { found.offset_from(remainder.as_ptr()) } as usize;
                    Some((distance, key, value))
                }
            })
            .min_by_key(|&(distance, _, _)| distance);
        let Some((distance, key, value)) = nearest else {
            output.extend_from_slice(&bytes[offset..]);
            break;
        };
        output.extend_from_slice(&bytes[offset..offset + distance]);
        output.extend_from_slice(value.to_bytes());
        offset += distance + key.to_bytes().len();
    }
    output
}

/// # Safety
/// ctx is a live exclusively borrowed parser context, normally constructed in Rust.
pub unsafe fn free_variables(ctx: *mut parser_ctx) {
    (*ctx).variables.clear();
}

unsafe fn try_chdir(path: *const ::core::ffi::c_char) -> bool {
    let mut result: bool = true_0 != 0;
    let mut dir: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut dirbuf: *mut ::core::ffi::c_char = sstrdup(path);
    dir = dirname(dirbuf);
    if !dir.is_null() {
        {
            // Snapshot native bytes before the logger can dispatch a callback.
            let log_pointer_1 = dir;
            let log_value_0 = i3_common::logging::string_bytes(
                if log_pointer_1.is_null() {
                    None
                } else {
                    Some(std::ffi::CStr::from_ptr(log_pointer_1))
                },
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
        if chdir(dir) == -(1 as ::core::ffi::c_int) {
            {
                // Snapshot native bytes before the logger can dispatch a callback.
                let log_pointer_1 = dir;
                let log_value_0 = i3_common::logging::string_bytes(
                    if log_pointer_1.is_null() {
                        None
                    } else {
                        Some(std::ffi::CStr::from_ptr(log_pointer_1))
                    },
                    None,
                )
                .to_vec();
                let log_pointer_2 = strerror(*__errno_location());
                let log_value_1 = i3_common::logging::string_bytes(
                    if log_pointer_2.is_null() {
                        None
                    } else {
                        Some(std::ffi::CStr::from_ptr(log_pointer_2))
                    },
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
            result = false_0 != 0;
        }
    }
    free(dirbuf as *mut ::core::ffi::c_void);
    dirbuf = ::core::ptr::null_mut::<::core::ffi::c_char>();
    result
}
unsafe fn parse_file_inner(
    ctx: *mut parser_ctx,
    f: *const ::core::ffi::c_char,
    included_file: *mut IncludedFile,
) -> ParseFileResult {
    if !try_chdir(f) {
        return ParseFileResult::Failed;
    }
    let fd = open(f, O_RDONLY);
    if fd == -(1 as ::core::ffi::c_int) {
        return ParseFileResult::Failed;
    }
    let mut stbuf: stat = ::core::mem::zeroed();
    if fstat(fd, &raw mut stbuf) == -(1 as ::core::ffi::c_int) {
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
    let Some(allocation_size) = file_size
        .checked_add(1)
        .filter(|&size| size <= isize::MAX as usize)
    else {
        return ParseFileResult::Failed;
    };
    (*included_file).raw_contents = scalloc(allocation_size, 1) as *mut ::core::ffi::c_char;
    if fread(
        (*included_file).raw_contents as *mut ::core::ffi::c_void,
        1_usize,
        stbuf.st_size as usize,
        fstr,
    ) as isize
        != stbuf.st_size as isize
    {
        return ParseFileResult::Failed;
    }
    rewind(fstr);
    let mut buffer: [::core::ffi::c_char; 4096] = [0; 4096];
    let mut key: [::core::ffi::c_char; 512] = [0; 512];
    let mut value: [::core::ffi::c_char; 4096] = [0; 4096];
    let mut continuation: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut invalid_sets: bool = false_0 != 0;
    let mut buffer_contents = Vec::with_capacity(file_size);
    while feof(fstr) == 0 {
        if continuation.is_null() {
            continuation = &raw mut buffer as *mut ::core::ffi::c_char;
        }
        if fgets(
            continuation,
            (::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as usize).wrapping_sub(
                continuation.offset_from(&raw mut buffer as *mut ::core::ffi::c_char)
                    as ::core::ffi::c_long as usize,
            ) as ::core::ffi::c_int,
            fstr,
        )
        .is_null()
        {
            if feof(fstr) != 0 {
                break;
            }
            return ParseFileResult::Failed;
        } else {
            if buffer[strlen(buffer.as_ptr()).saturating_sub(1)] as ::core::ffi::c_int
                != '\n' as i32
                && feof(fstr) == 0
            {
                {
                    let log_value_0 =
                        (::core::mem::size_of::<[::core::ffi::c_char; 4096]>()) as isize;
                    ::log::error!(
                        "ERROR: Your line continuation is too long, it exceeds {} bytes\n",
                        log_value_0
                    );
                };
            }
            value[0 as ::core::ffi::c_int as usize] = '\0' as i32 as ::core::ffi::c_char;
            let skip_line: bool = sscanf(
                &raw mut buffer as *mut ::core::ffi::c_char,
                c"%511s %4095[^\n]".as_ptr(),
                &raw mut key as *mut ::core::ffi::c_char,
                &raw mut value as *mut ::core::ffi::c_char,
            ) < 1 as ::core::ffi::c_int
                || strlen(&raw mut key as *mut ::core::ffi::c_char) < 3_usize;
            let comment: bool =
                key[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int == '#' as i32;
            value[4095 as ::core::ffi::c_int as usize] = '\n' as i32 as ::core::ffi::c_char;
            continuation = strstr(
                &raw mut buffer as *mut ::core::ffi::c_char,
                c"\\\n".as_ptr(),
            );
            if !continuation.is_null() {
                if !comment {
                    continue;
                }
                {
                    // Snapshot native bytes before the logger can dispatch a callback.
                    let log_precision_0 = (strlen(&raw mut buffer as *mut ::core::ffi::c_char)
                        as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int) as i32;
                    let log_pointer_1 = &raw mut buffer as *mut ::core::ffi::c_char;
                    let log_value_0 = i3_common::logging::string_bytes(
                        if log_pointer_1.is_null() {
                            None
                        } else {
                            Some(std::ffi::CStr::from_ptr(log_pointer_1))
                        },
                        usize::try_from(log_precision_0).ok(),
                    )
                    .to_vec();
                    if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
                        ::log::debug!("../src/config_parser.c:parse_file_inner:682 - line continuation in comment is ignored: \"{}\"\n", log_value_0_text);
                    } else {
                        let mut message = Vec::new();
                        message.extend_from_slice("../src/config_parser.c:parse_file_inner:682 - line continuation in comment is ignored: \"".as_bytes());
                        message.extend_from_slice(&log_value_0);
                        message.extend_from_slice("\"\n".as_bytes());
                        i3_common::logging::emit(i3_common::logging::Level::Debug, &message);
                    }
                };
                continuation = ::core::ptr::null_mut::<::core::ffi::c_char>();
            }
            buffer_contents.extend_from_slice(std::ffi::CStr::from_ptr(buffer.as_ptr()).to_bytes());
            if skip_line as ::core::ffi::c_int != 0 || comment as ::core::ffi::c_int != 0 {
                continue;
            }
            if strcasecmp(&raw mut key as *mut ::core::ffi::c_char, c"set".as_ptr())
                == 0 as ::core::ffi::c_int
                && *(&raw mut value as *mut ::core::ffi::c_char) as ::core::ffi::c_int
                    != '\0' as i32
            {
                let mut v_key: [::core::ffi::c_char; 512] = [0; 512];
                let mut v_value: [::core::ffi::c_char; 4096] = [0; 4096];
                if sscanf(
                    &raw mut value as *mut ::core::ffi::c_char,
                    c"%511s %4095[^\n]".as_ptr(),
                    &raw mut v_key as *mut ::core::ffi::c_char,
                    &raw mut v_value as *mut ::core::ffi::c_char,
                ) < 1 as ::core::ffi::c_int
                {
                    {
                        // Snapshot native bytes before the logger can dispatch a callback.
                        let log_pointer_1 = &raw mut value as *mut ::core::ffi::c_char;
                        let log_value_0 = i3_common::logging::string_bytes(
                            if log_pointer_1.is_null() {
                                None
                            } else {
                                Some(std::ffi::CStr::from_ptr(log_pointer_1))
                            },
                            None,
                        )
                        .to_vec();
                        if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
                            ::log::error!("ERROR: Failed to parse variable specification '{}', skipping it.\n", log_value_0_text);
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
                    invalid_sets = true_0 != 0;
                } else if v_key[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    != '$' as i32
                {
                    ::log::error!(
                        "ERROR: Malformed variable assignment, name has to start with $\n"
                    );
                    invalid_sets = true_0 != 0;
                } else {
                    upsert_variable(
                        &raw mut (*ctx).variables,
                        &raw mut v_key as *mut ::core::ffi::c_char,
                        &raw mut v_value as *mut ::core::ffi::c_char,
                    );
                }
            } else {
                if !(strcasecmp(
                    &raw mut key as *mut ::core::ffi::c_char,
                    c"set_from_resource".as_ptr(),
                ) == 0 as ::core::ffi::c_int)
                {
                    continue;
                }
                let mut res_name: [::core::ffi::c_char; 512] = [0; 512];
                let mut v_key_0: [::core::ffi::c_char; 512] = [0; 512];
                let mut fallback: [::core::ffi::c_char; 4096] = [0; 4096];
                fallback[0 as ::core::ffi::c_int as usize] = '\0' as i32 as ::core::ffi::c_char;
                if sscanf(
                    &raw mut value as *mut ::core::ffi::c_char,
                    c"%511s %511s %4095[^\n]".as_ptr(),
                    &raw mut v_key_0 as *mut ::core::ffi::c_char,
                    &raw mut res_name as *mut ::core::ffi::c_char,
                    &raw mut fallback as *mut ::core::ffi::c_char,
                ) < 1 as ::core::ffi::c_int
                {
                    {
                        // Snapshot native bytes before the logger can dispatch a callback.
                        let log_pointer_1 = &raw mut value as *mut ::core::ffi::c_char;
                        let log_value_0 = i3_common::logging::string_bytes(
                            if log_pointer_1.is_null() {
                                None
                            } else {
                                Some(std::ffi::CStr::from_ptr(log_pointer_1))
                            },
                            None,
                        )
                        .to_vec();
                        if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
                            ::log::error!("ERROR: Failed to parse resource specification '{}', skipping it.\n", log_value_0_text);
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
                    invalid_sets = true_0 != 0;
                } else if v_key_0[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    != '$' as i32
                {
                    ::log::error!(
                        "ERROR: Malformed variable assignment, name has to start with $\n"
                    );
                    invalid_sets = true_0 != 0;
                } else {
                    let resource_name = std::ffi::CStr::from_ptr(res_name.as_ptr());
                    let resource = resources.get(resource_name);
                    let mut res_value = resource
                        .as_ref()
                        .map_or(std::ptr::null(), |value| value.as_ptr());
                    if res_value.is_null() {
                        {
                            // Snapshot native bytes before the logger can dispatch a callback.
                            let log_pointer_1 = &raw mut res_name as *mut ::core::ffi::c_char;
                            let log_value_0 = i3_common::logging::string_bytes(
                                if log_pointer_1.is_null() {
                                    None
                                } else {
                                    Some(std::ffi::CStr::from_ptr(log_pointer_1))
                                },
                                None,
                            )
                            .to_vec();
                            let log_pointer_2 = &raw mut fallback as *mut ::core::ffi::c_char;
                            let log_value_1 = i3_common::logging::string_bytes(
                                if log_pointer_2.is_null() {
                                    None
                                } else {
                                    Some(std::ffi::CStr::from_ptr(log_pointer_2))
                                },
                                None,
                            )
                            .to_vec();
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
                                i3_common::logging::emit(
                                    i3_common::logging::Level::Debug,
                                    &message,
                                );
                            }
                        };
                        res_value = fallback.as_ptr();
                    }
                    upsert_variable(
                        &raw mut (*ctx).variables,
                        &raw mut v_key_0 as *mut ::core::ffi::c_char,
                        res_value,
                    );
                }
            }
        }
    }
    drop(stream);
    drop(resources);
    let input =
        std::ffi::CString::new(buffer_contents).expect("fgets C-string projections contain no NUL");
    let variables: Vec<_> = (*ctx)
        .variables
        .iter()
        .map(|variable| (variable.key.as_c_str(), variable.value.as_c_str()))
        .collect();
    let expanded = expand_variables(&input, &variables);
    drop(variables);
    let expanded = std::ffi::CString::new(expanded).expect("variable C strings contain no NUL");
    let new = expanded.as_ptr();
    (*included_file).variable_replaced_contents = sstrdup(new);
    let mut local_context: context = std::mem::zeroed();
    local_context.filename = f;
    let context = &raw mut local_context;
    parse_config(ctx, new, context);
    if (*ctx).has_errors {
        (*context).has_errors = true_0 != 0;
    }
    check_for_duplicate_bindings(context);
    if (*ctx).use_nagbar as ::core::ffi::c_int != 0
        && ((*context).has_errors as ::core::ffi::c_int != 0
            || (*context).has_warnings as ::core::ffi::c_int != 0
            || invalid_sets as ::core::ffi::c_int != 0)
    {
        {
            // Snapshot native bytes before the logger can dispatch a callback.
            let log_pointer_1 = i3_version;
            let log_value_0 = i3_common::logging::string_bytes(
                if log_pointer_1.is_null() {
                    None
                } else {
                    Some(std::ffi::CStr::from_ptr(log_pointer_1))
                },
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
        start_config_error_nagbar(
            f,
            (*context).has_errors as ::core::ffi::c_int != 0
                || invalid_sets as ::core::ffi::c_int != 0,
        );
    }
    let has_errors: bool = (*context).has_errors;
    free((*context).line_copy as *mut ::core::ffi::c_void);
    (*context).line_copy = ::core::ptr::null_mut::<::core::ffi::c_char>();

    if has_errors {
        return ParseFileResult::ConfigErrors;
    }
    ParseFileResult::Success
}

pub unsafe fn parse_file(
    ctx: *mut parser_ctx,
    f: *const ::core::ffi::c_char,
    included_file: *mut IncludedFile,
) -> ParseFileResult {
    let old_directory = NativeString(getcwd(std::ptr::null_mut(), 0));
    let old_dir = old_directory.0;
    let mut result: ParseFileResult = parse_file_inner(ctx, f, included_file);
    if chdir(old_dir) == -(1 as ::core::ffi::c_int) {
        {
            // Snapshot native bytes before the logger can dispatch a callback.
            let log_pointer_1 = old_dir;
            let log_value_0 = i3_common::logging::string_bytes(
                if log_pointer_1.is_null() {
                    None
                } else {
                    Some(std::ffi::CStr::from_ptr(log_pointer_1))
                },
                None,
            )
            .to_vec();
            let log_pointer_2 = strerror(*__errno_location());
            let log_value_1 = i3_common::logging::string_bytes(
                if log_pointer_2.is_null() {
                    None
                } else {
                    Some(std::ffi::CStr::from_ptr(log_pointer_2))
                },
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
pub const ERANGE: ::core::ffi::c_int = 34 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;

// Internal helpers use Rust linkage rather than foreign symbol lookup.
use crate::util::start_nagbar;
use i3_common::safewrappers::scalloc;
use i3_common::safewrappers::smalloc;
use i3_common::safewrappers::sstrdup;

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
    use super::*;
    #[test]
    fn variable_storage_keeps_longest_first_case_sensitive_updates_and_nonrecursive_expansion() {
        let mut variables = Vec::new();
        store_variable(&mut variables, c"$A", c"short");
        store_variable(&mut variables, c"$ABC", c"long");
        store_variable(&mut variables, c"$X", c"$A");
        store_variable(&mut variables, c"$a", c"case");
        store_variable(&mut variables, c"$ABC", c"updated");
        assert_eq!(variables.len(), 4);
        assert_eq!(variables[0].key.as_c_str(), c"$ABC");
        let pairs: Vec<_> = variables
            .iter()
            .map(|entry| (entry.key.as_c_str(), entry.value.as_c_str()))
            .collect();
        assert_eq!(expand_variables(c"$abc $A $X", &pairs), b"updated case $A");
        assert_eq!(expand_variables(c"nothing\xff", &pairs), b"nothing\xff");
    }
    #[test]
    fn bounded_line_context_handles_first_byte_crlf_and_previous_lines() {
        assert_eq!(line_start(b"abc", 0), 0);
        assert_eq!(line_start(b"abc", usize::MAX), 0);
        assert_eq!(line_start(b"a\r\nb", 2), 3);
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
        let states = unsafe { &*(&raw const tokens) };
        assert_eq!(states.len(), 117);
        let mut calls = std::collections::BTreeSet::new();
        for table in states {
            assert!(table.n > 0);
            let entries = unsafe { std::slice::from_raw_parts(table.array, table.n as usize) };
            for entry in entries {
                if entry.next_state == ConfigParserState::Call {
                    calls.insert(unsafe { entry.extra.call_identifier });
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
